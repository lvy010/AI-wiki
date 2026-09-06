# AI-wiki

- Interstellar: **数据、算法、模型、硬件、架构**

序言: 这个时代壁垒是什么? 全局思维，或者说设计架构的能力，模型架构 系统架构 芯片架构等等。未来更考验知识的广度，就是全栈思维，从算法到系统到硬件，因为某一个东西的具体深入大模型都有很强的能力，就好比有一句话“这个问题或许不知道怎么解，但是我知道应该找什么来解”。所以多接触多学习更广的知识内容。

skill: [agent-insight](./agent-insight)

- [Course](#course)
  - [LLM](#llm)
  - [Agent](#agent)
- [Blog](#blog)
  - [FlashAttention](#flashattention)
  - [Harness Engineering](#harness-engineering)
  - [Quantization](#quantization)
  - [Speculative Decoding Blog](#speculative-decoding-blog)
  - [CUDA](#cuda)
- [Book](#book)
- [Paper](#paper)
  - [Base Model](#base-model)
  - [Fine-tuning](#fine-tuning)
  - [Attention Optimization](#attention-optimization)
  - [Speculative Decoding](#speculative-decoding)
  - [KV Cache & Inference Storage](#kv-cache--inference-storage)
  - [Prefill-Decode Disaggregation](#prefill-decode-disaggregation)
  - [LLM Application & Prompt Engineering](#llm-application--prompt-engineering)
  - [MoE Mixture of Experts](#moe-mixture-of-experts)
  - [Scheduling & Batching](#scheduling--batching)
  - [Training Optimization & Scaling](#training-optimization--scaling)

计划9月构建完本仓库[roadmap](./roadmap.md), 好的资料/笔记欢迎提交 pr

## 一、Course

### LLM

- [CS224n](https://web.stanford.edu/class/cs224n/)
- [CS336: Language Modeling from Scratch (Stanford)](https://stanford-cs336.github.io/) Work in progress：[Lab](./cs336-lab/README.md)
- [CSCI 1390, Spring 2025: Systems for Machine Learning](https://cs.brown.edu/courses/csci1390/)

### Agent

- [从零开始理解 Agent](https://bbs.huaweicloud.com/blogs/476265)
- [Learn Claude Code](#)

## 2、Paper

### 底座

[PaLM](https://arxiv.org/abs/2204.02311)，[OPT](https://arxiv.org/abs/2205.01068)，[BLOOM](https://arxiv.org/abs/2211.05100)，[LLaMA](https://arxiv.org/abs/2302.13971)

### 微调

* 对齐微调: [InstructGPT (RLHF)](https://arxiv.org/abs/2203.02155)，[Constitutional AI](https://arxiv.org/abs/2212.08073)，[Self-Instruct](https://arxiv.org/abs/2212.10560)，[Direct Preference Optimization (DPO)](https://arxiv.org/abs/2305.18290)，[ORPO](https://arxiv.org/abs/2403.07691)，[GRPO](https://arxiv.org/abs/2402.03300)
* 轻量化微调: [LoRA](https://arxiv.org/abs/2106.09685)，[QLoRA](https://arxiv.org/abs/2305.14314)

### Attention 优化

- [FlashAttention](https://arxiv.org/abs/2205.14135)
- [FlashAttention-2](https://arxiv.org/abs/2307.08691)
- [RoPE (Rotary Position Embeddings)](https://arxiv.org/abs/2104.09864)
- [ALiBi](https://arxiv.org/abs/2108.12409)
- [Multi-Query Attention (MQA)](https://arxiv.org/abs/1911.02150)
- [Grouped-Query Attention (GQA)](https://arxiv.org/abs/2305.13245)

### 推测解码

- [Speculative Decoding](https://arxiv.org/abs/2401.07851)
- [Medusa: Simple LLM Inference Acceleration Framework with Multiple Decoding Heads](https://arxiv.org/abs/2401.10774)
- [Fast Inference from Transformers via Speculative Decoding](https://arxiv.org/abs/2211.17192)
- [Break the Sequential Dependency of LLM Inference Using Lookahead Decoding](https://arxiv.org/abs/2402.02057)
- [Accelerating Large Language Model Decoding with Speculative Sampling](https://arxiv.org/abs/2302.01318)

### KV Cache & 推理存储

- [PagedAttention (vLLM)](https://arxiv.org/abs/2309.06180)
- [Efficient Memory Management for Large Language Model Serving with PagedAttention](https://arxiv.org/abs/2309.06180)
- [KV Cache Compression & Optimization](https://arxiv.org/abs/2407.18003)

### PD 分离

- [Mooncake: A KVCache-centric Disaggregated Architecture for LLM Serving](https://arxiv.org/abs/2407.00079)
- [Splitwise: Efficient Generative LLM Inference Using Phase Splitting](https://arxiv.org/abs/2311.18677)
- [DualPath: Breaking the Storage Bandwidth Bottleneck in Agentic LLM Inference](https://arxiv.org/abs/2602.21548)
- [DistServe: Disaggregating Prefill and Decoding for Goodput-optimized Large Language Model Serving](https://arxiv.org/abs/2401.09670)
- [MemServe: Context Caching for Disaggregated LLM Serving with Elastic Memory Pool](https://arxiv.org/abs/2406.17565)
- [TetriInfer: Inference without Interference: Disaggregate LLM Inference for Mixed Downstream Workloads](https://arxiv.org/abs/2401.11181)

### LLM 应用与提示工程

- [Retrieval-Augmented Generation (RAG)](https://arxiv.org/abs/2005.11401)
- [METIS: Fast Quality-Aware RAG Systems with Configuration Adaptation](#)
- [CacheBlend: Fast Large Language Model Serving for RAG with Cached Knowledge Fusion](https://arxiv.org/abs/2405.16444)
- [Parrot: Efficient Serving of LLM-based Applications with Semantic Variable](https://arxiv.org/abs/2405.19888)
- [Towards End-to-End Optimization of LLM-based Applications with Ayo](#)
- [Chain-of-Thought Prompting](https://arxiv.org/abs/2201.11903)
- [Tree of Thoughts](https://arxiv.org/abs/2305.10601)
- [ReAct](https://arxiv.org/abs/2210.03629)

### MoE 混合专家

- [Mixture of Experts (Switch Transformer)](https://arxiv.org/abs/2101.03961)
- [DeepSeekMoE](https://arxiv.org/abs/2401.06066)

### 调度与批处理

- [DeepSpeed-FastGen: High-throughput Text Generation for LLMs](https://arxiv.org/abs/2401.08671)
- [SARATHI: Efficient LLM Inference by Piggybacking Decodes with Chunked Prefills](https://arxiv.org/abs/2308.16369)
- [Taming Throughput-Latency Tradeoff in LLM Inference with Sarathi-Serve](https://arxiv.org/abs/2403.02310)

### 训练优化与缩放

- [Test-Time Scaling](https://arxiv.org/abs/2408.03314)
- [Muon Optimizer](https://kellerjordan.github.io/posts/muon/)

## 3、Blog

### FlashAttention

- [ELI5: FlashAttention](https://gordicaleksa.medium.com/eli5-flash-attention-5c44017022ad)
- [FlashAttention from First Principles](#)
- [Flash Attention 2.0 with Tri Dao (author)!](https://www.latent.space/p/flashattention)
- [图解大模型计算加速系列：FlashAttention V1，从硬件到计算逻辑](https://zhuanlan.zhihu.com/p/669926191)
- [FlashAttention — Visually and Exhaustively Explained](https://aiadvances.org/flashattention-visually-and-exhaustively-explained-d6124670f7fb)
- [Designing Hardware-Aware Algorithms: FlashAttention](https://www.digitalocean.com/community/tutorials/flashattention)
- [FlashAttention: Fast and Memory-Efficient Exact Attention With IO-Awareness](https://arxiv.org/abs/2205.14135)

### Harness Engineering

设计环境、规则、测试反馈系统，让 AI Agent 自动生成并改进代码

- [Minions: Stripe’s one-shot, end-to-end coding agents—Part 2](#)
- [Effective harnesses for long-running agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
- [Minions: Stripe’s one-shot, end-to-end coding agents](#)
- [Harness engineering: leveraging Codex in an agent-first world](https://openai.com/index/harness-engineering/)
- [Vibe Coding AReaL：零手打代码开发分布式 RL 训练框架](https://zhuanlan.zhihu.com/p/2003269671630165191)

### Triton

- [Deep Dive into Triton Internals (Part 3)](https://www.kapilsharma.dev/posts/deep-dive-into-triton-internals-3/)
- [Deep Dive into Triton Internals (Part 1)](https://www.kapilsharma.dev/posts/deep-dive-into-triton-internals/)
- [Deep Dive into Triton Internals (Part 2)](https://www.kapilsharma.dev/posts/deep-dive-into-triton-internals-2/)

### vLLM

- [vLLM源码解析](https://zhuanlan.zhihu.com/p/691045737)
- [Inside vLLM: Anatomy of a High-Throughput LLM Inference System](https://vllm.ai/blog/2025-09-05-anatomy-of-vllm)

### GPU

- [A history of NVidia Stream Multiprocessor](https://fabiensanglard.net/cuda/)
- [Building a Tiny GPU to Understand AI Hardware Engineering](https://github.com/adam-maj/tiny-gpu)

### CUTLASS

- [Learn CUTLASS the hard way - part 2!](https://kapilsh.github.io/posts/learn-cutlass-the-hard-way-2/)
- [Learn CUTLASS the hard way! (Video)](https://www.youtube.com/watch?v=jGouxuAHIfQ)

### 量化

- [PyTorch 的量化实战项目](#)
- [PyTorch 官方量化资料](https://pytorch.org/docs/stable/quantization.html)

### 推测解码

- [How Speculative Decoding Boosts vLLM Performance by up to 2.8x](https://vllm.ai/blog/2024-10-17-spec-decode)

### CUDA

- [LeetCUDA](https://github.com/xlite-dev/LeetCUDA)
- [How to Optimize a CUDA Matmul Kernel for cuBLAS-like Performance: a Worklog](https://siboehm.com/articles/22/CUDA-MMM)

## 4、Book

- [Build a Large Language Model (From Scratch)](https://www.manning.com/books/build-a-large-language-model-from-scratch)
- [AI Systems Performance Engineering](#)：GPU CUDA Kernel 调优、PyTorch 算法优化、多节点训练推理系统调优...
