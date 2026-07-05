# 检索增强 (RAG)

**是什么**:检索外部知识拼进上下文再生成,缓解幻觉、接入私有/实时数据。链路:切分 → 向量化 → 检索 → 重排 → 生成。

## 必读
1. RAG 原始论文 — [2005.11401](https://arxiv.org/abs/2005.11401)
2. DPR(稠密检索)— [2004.04906](https://arxiv.org/abs/2004.04906)
3. FiD(Fusion-in-Decoder)— [2007.01282](https://arxiv.org/abs/2007.01282)
4. Self-RAG(自省检索)— [2310.11511](https://arxiv.org/abs/2310.11511)
5. 综述:[RAG for LLMs](https://arxiv.org/abs/2312.10997)

## 核心仓库
- [llama_index](https://github.com/run-llama/llama_index) · [langchain](https://github.com/langchain-ai/langchain)
- 向量库:[faiss](https://github.com/facebookresearch/faiss) · [milvus](https://github.com/milvus-io/milvus) · [chroma](https://github.com/chroma-core/chroma)
- 嵌入/重排:[sentence-transformers](https://github.com/UKPLab/sentence-transformers) · [FlagEmbedding (BGE)](https://github.com/FlagOpen/FlagEmbedding)

## 追踪
[Eugene Yan](https://eugeneyan.com) · [HF Blog](https://huggingface.co/blog)
