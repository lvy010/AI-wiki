# 系统与部署 (Systems / Inference / Edge)

**是什么**:让模型跑得快、省、稳。覆盖高效注意力、量化、KV-cache、推理引擎、编译、边缘端部署。

## 必读
1. FlashAttention 1/2 — [2205.14135](https://arxiv.org/abs/2205.14135) · [2307.08691](https://arxiv.org/abs/2307.08691)
2. PagedAttention / vLLM — [2309.06180](https://arxiv.org/abs/2309.06180)
3. GPTQ(训练后量化)— [2210.17323](https://arxiv.org/abs/2210.17323)
4. AWQ(激活感知量化)— [2306.00978](https://arxiv.org/abs/2306.00978)
5. LLM.int8() — [2208.07339](https://arxiv.org/abs/2208.07339)
6. Speculative Decoding(投机采样)— [2211.17192](https://arxiv.org/abs/2211.17192)

## 核心仓库
- 服务/吞吐:[vllm](https://github.com/vllm-project/vllm) · [TensorRT-LLM](https://github.com/NVIDIA/TensorRT-LLM) · [sglang](https://github.com/sgl-project/sglang)
- 端侧/CPU:[llama.cpp](https://github.com/ggml-org/llama.cpp) · [ggml](https://github.com/ggml-org/ggml) · ⭐ [ncnn](https://github.com/Tencent/ncnn) · [MNN](https://github.com/alibaba/MNN) · [MLC-LLM](https://github.com/mlc-ai/mlc-llm)
- 编译:[Apache TVM](https://github.com/apache/tvm) · [torch.compile](https://pytorch.org/docs/stable/torch.compiler.html)
- 量化工具:[bitsandbytes](https://github.com/bitsandbytes-foundation/bitsandbytes) · [AutoGPTQ](https://github.com/AutoGPTQ/AutoGPTQ) · [llama.cpp GGUF]

## 实战参考
- [Penguin-VL-ncnn](https://github.com/lvy010/penguin-vl-ncnn) — VLM 从 PyTorch 到 ncnn 的全链路移植(pnnx 导出、KV-cache 契约、2D-RoPE、数值对齐、多平台 CMake)。

## 追踪
[Tri Dao](https://x.com/tri_dao) · [nihui / ncnn](https://github.com/Tencent/ncnn) · [HF Blog](https://huggingface.co/blog)
