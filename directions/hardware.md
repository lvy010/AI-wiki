# 硬件

硬件层关注“模型为什么慢、为什么贵、为什么爆显存”。AI 高手至少要能读懂瓶颈,并做基础优化。

## 要学什么

- GPU 基础:SM、warp、thread/block、显存层级、带宽、算力。
- CUDA:kernel、memory coalescing、shared memory、occupancy。
- 深度学习性能:矩阵乘、attention、激活显存、KV Cache。
- 精度与量化:FP32/FP16/BF16/INT8/INT4、GPTQ/AWQ/GGUF。
- 并行:数据并行、张量并行、流水线并行、ZeRO。
- 推理加速:batching、continuous batching、PagedAttention、speculative decoding。

## 起点资料

- ⭐ [CUDA C++ Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/) — 官方 CUDA 文档。
- ⭐ [GPU MODE](https://github.com/gpu-mode/lectures) — GPU 编程课程与资料。
- [MLSYS Book](https://mlsysbook.ai) — ML 系统与硬件约束。
- [FlashAttention](https://arxiv.org/abs/2205.14135) — 理解 IO-aware 算法。

## 核心仓库

- [llama.cpp](https://github.com/ggml-org/llama.cpp) — CPU/端侧推理与 GGUF。
- [vllm](https://github.com/vllm-project/vllm) — 高吞吐 LLM 推理。
- [flash-attention](https://github.com/Dao-AILab/flash-attention) — 高效注意力内核。
- [triton](https://github.com/triton-lang/triton) — Python 写 GPU kernel。
- [ncnn](https://github.com/Tencent/ncnn) — 移动/边缘推理。

## 可做项目

- 对比同一模型在 FP16、INT8、INT4 下的速度、显存、质量。
- 用 vLLM 部署一个模型,测试并发、吞吐、首 token 延迟。
- 写一个简单 CUDA/Triton 矩阵乘 kernel,和 PyTorch 做性能对比。

## 检查点

- 能估算模型参数量、显存占用、KV Cache 成本。
- 能看懂 nvidia-smi、torch profiler、吞吐/延迟指标。
- 能解释 FlashAttention 为什么比朴素 Attention 更省显存。