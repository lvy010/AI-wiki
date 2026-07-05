# 大语言模型 (LLM)

**是什么**:基于 Transformer 的大规模自回归模型。链路:预训练 → 指令微调(SFT)→ 对齐(RLHF/DPO)→ 推理/服务。

## 必读(按顺序)
1. Attention Is All You Need — [1706.03762](https://arxiv.org/abs/1706.03762)
2. GPT-3 Few-Shot — [2005.14165](https://arxiv.org/abs/2005.14165)
3. Scaling Laws / Chinchilla — [2001.08361](https://arxiv.org/abs/2001.08361) · [2203.15556](https://arxiv.org/abs/2203.15556)
4. InstructGPT(RLHF)— [2203.02155](https://arxiv.org/abs/2203.02155)
5. LoRA / QLoRA(高效微调)— [2106.09685](https://arxiv.org/abs/2106.09685) · [2305.14314](https://arxiv.org/abs/2305.14314)
6. DPO(简化对齐)— [2305.18290](https://arxiv.org/abs/2305.18290)

## 核心仓库
- 从零学:[nanoGPT](https://github.com/karpathy/nanoGPT) · [LLMs-from-scratch](https://github.com/rasbt/LLMs-from-scratch)
- 训练/微调:[transformers](https://github.com/huggingface/transformers) · [peft](https://github.com/huggingface/peft) · [unsloth](https://github.com/unslothai/unsloth) · [trl](https://github.com/huggingface/trl)
- 路线图:[mlabonne/llm-course](https://github.com/mlabonne/llm-course)

## 追踪
[HF Papers](https://huggingface.co/papers) · [Interconnects](https://www.interconnects.ai) · [Ahead of AI](https://magazine.sebastianraschka.com)
