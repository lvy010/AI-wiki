//! Minimal Byte-Pair Encoding (BPE) tokenizer.
//!
//! Implements the core algorithm used in GPT / LLaMA tokenizers:
//!   1. Vocabulary starts as individual characters.
//!   2. Training: repeatedly merge the most-frequent adjacent pair.
//!   3. Encoding: apply learned merges (highest-priority first).
//!   4. Decoding: look up token IDs → strings and concatenate.
//!
//! Reference: Sennrich et al. 2016 — <https://arxiv.org/abs/1508.07909>

use std::collections::HashMap;

// ── Types ────────────────────────────────────────────────────────────────────

/// A single merge rule produced during training.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Merge(pub String, pub String);

/// Trained tokenizer: vocab + ordered merge rules.
pub struct BpeTokenizer {
    vocab: HashMap<String, u32>,  // token str → id
    id_to_token: Vec<String>,     // id → token str (reverse vocab)
    merges: Vec<Merge>,           // index = priority (lower = applied first)
}

// ── Training ─────────────────────────────────────────────────────────────────

/// Train a BPE tokenizer on `corpus` performing up to `num_merges` steps.
pub fn train(corpus: &str, num_merges: usize) -> BpeTokenizer {
    // Represent each word as a sequence of single-character tokens.
    let mut words: Vec<Vec<String>> = corpus
        .split_whitespace()
        .map(|w| w.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut merges: Vec<Merge> = Vec::new();

    for _ in 0..num_merges {
        // Count adjacent-pair frequencies across all words.
        let mut freq: HashMap<(&str, &str), usize> = HashMap::new();
        for word in &words {
            for pair in word.windows(2) {
                *freq.entry((&pair[0], &pair[1])).or_insert(0) += 1;
            }
        }

        // Pick the most-frequent pair (ties broken by natural ordering).
        let best = match freq.into_iter().max_by_key(|&(_, c)| c) {
            Some(((a, b), _)) => (a.to_string(), b.to_string()),
            None => break,
        };

        // Apply the merge everywhere in the corpus.
        let merged = format!("{}{}", best.0, best.1);
        for word in &mut words {
            let mut i = 0;
            while i + 1 < word.len() {
                if word[i] == best.0 && word[i + 1] == best.1 {
                    word[i] = merged.clone();
                    word.remove(i + 1);
                } else {
                    i += 1;
                }
            }
        }
        merges.push(Merge(best.0, best.1));
    }

    // Build vocab from all tokens that appear in the merged corpus.
    let mut token_set: Vec<String> = Vec::new();
    for word in &words {
        for token in word {
            if !token_set.contains(token) {
                token_set.push(token.clone());
            }
        }
    }
    token_set.sort(); // deterministic ordering

    let vocab: HashMap<String, u32> = token_set
        .iter()
        .enumerate()
        .map(|(i, t)| (t.clone(), i as u32))
        .collect();

    BpeTokenizer { vocab, id_to_token: token_set, merges }
}

// ── BpeTokenizer impl ────────────────────────────────────────────────────────

impl BpeTokenizer {
    /// Encode whitespace-separated text into token IDs.
    /// Unknown tokens map to `u32::MAX`.
    pub fn encode(&self, text: &str) -> Vec<u32> {
        let mut ids = Vec::new();
        for word in text.split_whitespace() {
            let mut tokens: Vec<String> =
                word.chars().map(|c| c.to_string()).collect();

            // Apply merges in priority order; restart after each successful merge.
            'outer: loop {
                for Merge(a, b) in &self.merges {
                    for i in 0..tokens.len().saturating_sub(1) {
                        if tokens[i] == *a && tokens[i + 1] == *b {
                            tokens[i] = format!("{}{}", a, b);
                            tokens.remove(i + 1);
                            continue 'outer;
                        }
                    }
                }
                break;
            }

            for t in &tokens {
                ids.push(*self.vocab.get(t).unwrap_or(&u32::MAX));
            }
        }
        ids
    }

    /// Decode token IDs back to a string (spaces stripped, tokens joined).
    pub fn decode(&self, ids: &[u32]) -> String {
        ids.iter()
            .filter_map(|&id| self.id_to_token.get(id as usize))
            .cloned()
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    pub fn num_merges(&self) -> usize {
        self.merges.len()
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Classic BPE example corpus from the original paper.
    const CORPUS: &str =
        "low low low low low lower lower newest newest newest newest newest newest \
         widest widest widest";

    #[test]
    fn train_produces_merges() {
        let tok = train(CORPUS, 10);
        assert!(tok.num_merges() > 0);
    }

    #[test]
    fn frequent_pair_merged_first() {
        // "lo" or "low" must appear early since "low*" dominates the corpus.
        let tok = train(CORPUS, 5);
        assert!(tok.merges.iter().any(|Merge(a, b)| {
            format!("{}{}", a, b) == "lo" || format!("{}{}", a, b) == "low"
        }));
    }

    #[test]
    fn encode_returns_valid_ids() {
        let tok = train(CORPUS, 10);
        let ids = tok.encode("low lower");
        assert!(!ids.is_empty());
        // All IDs must be in-vocab (not MAX) for known words.
        assert!(ids.iter().all(|&id| (id as usize) < tok.vocab_size()));
    }

    #[test]
    fn decode_inverts_encode() {
        let tok = train(CORPUS, 10);
        let text = "low";
        let ids = tok.encode(text);
        let decoded = tok.decode(&ids);
        assert_eq!(decoded, text);
    }

    #[test]
    fn unknown_token_returns_max() {
        let tok = train(CORPUS, 5);
        let ids = tok.encode("zzz");
        assert!(ids.contains(&u32::MAX));
    }

    #[test]
    fn more_merges_shorter_encoding() {
        let tok0 = train(CORPUS, 0);
        let tok10 = train(CORPUS, 10);
        // More merges → fewer tokens needed to encode the same text.
        let ids0 = tok0.encode("low lower");
        let ids10 = tok10.encode("low lower");
        assert!(ids10.len() <= ids0.len());
    }
}
