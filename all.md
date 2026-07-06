# Interstellar · All in One

> 单文件合集。内容来自本仓库全部 Markdown 文件,不包含 `all.md` 自身。

## 目录

1. [README](#readme) — [README.md](README.md)
2. [Roadmap](#roadmap) — [roadmap.md](roadmap.md)
3. [Directions Index](#directions-index) — [directions/README.md](directions/README.md)
4. [数据](#数据) — [directions/data.md](directions/data.md)
5. [算法](#算法) — [directions/algorithm.md](directions/algorithm.md)
6. [模型](#模型) — [directions/model.md](directions/model.md)
7. [硬件](#硬件) — [directions/hardware.md](directions/hardware.md)
8. [架构](#架构) — [directions/architecture.md](directions/architecture.md)
9. [LLM](#llm) — [directions/llm.md](directions/llm.md)
10. [RAG](#rag) — [directions/rag.md](directions/rag.md)
11. [Agents](#agents) — [directions/agents.md](directions/agents.md)
12. [Multimodal](#multimodal) — [directions/multimodal.md](directions/multimodal.md)
13. [Diffusion](#diffusion) — [directions/diffusion-genai.md](directions/diffusion-genai.md)
14. [RL / RLHF](#rl--rlhf) — [directions/rl-rlhf.md](directions/rl-rlhf.md)
15. [CV](#cv) — [directions/cv.md](directions/cv.md)
16. [Speech](#speech) — [directions/speech.md](directions/speech.md)
17. [Systems](#systems) — [directions/systems-infra.md](directions/systems-infra.md)
18. [Papers](#papers) — [papers/README.md](papers/README.md)
19. [Books](#books) — [books/README.md](books/README.md)
20. [Repos](#repos) — [repos/README.md](repos/README.md)
21. [Blogs](#blogs) — [blogs/README.md](blogs/README.md)
22. [Videos](#videos) — [videos/README.md](videos/README.md)
23. [People](#people) — [people/README.md](people/README.md)

---

<a id="readme"></a>

## README

来源:[README.md](README.md)

# Interstellar

1. 数据采集、清洗、标注、评测集 — [data.md](directions/data.md)
2. 线性代数、概率统计、优化方法 — [Mathematics for Machine Learning](https://mml-book.github.io)
3. 经典机器学习、指标、baseline — [CS229](https://www.youtube.com/playlist?list=PLoROMvodv4rMiGQp3WXShtMGgzqpfVfbU)
4. PyTorch、autograd、训练循环 — [Dive into Deep Learning](https://d2l.ai)
5. 反向传播最小实现 — [micrograd](https://github.com/karpathy/micrograd)
6. 深度学习核心结构 — [Understanding Deep Learning](https://udlbook.github.io/udlbook/)
7. Attention 与 Transformer — [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
8. BERT / GPT / LLaMA 语言模型主线 — [papers](papers/README.md)
9. 从零训练小 GPT — [nanoGPT](https://github.com/karpathy/nanoGPT)
10. 从零写 LLM — [LLMs-from-scratch](https://github.com/rasbt/LLMs-from-scratch)
11. LoRA / QLoRA / SFT / DPO 微调 — [model.md](directions/model.md)
12. RAG、embedding、向量检索、rerank — [rag.md](directions/rag.md)
13. Agent、工具调用、工作流编排 — [agents.md](directions/agents.md)
14. 多模态、CV、语音、扩散模型 — [directions](directions/README.md)
15. GPU、CUDA、显存、KV Cache — [hardware.md](directions/hardware.md)
16. FlashAttention、vLLM、llama.cpp 推理优化 — [systems-infra.md](directions/systems-infra.md)
17. API、评测、监控、MLOps、成本控制 — [architecture.md](directions/architecture.md)
18. 完成一个可展示项目:数据集 + 模型/RAG + 服务 + 评测 — [roadmap.md](roadmap.md)

## AI 全栈

1. **数据**: 采集、清洗、标注、切分、评测、反馈闭环。
2. **算法**: 数学、优化、机器学习、深度学习、检索、强化学习。
3. **模型**: Transformer、LLM、多模态、扩散、微调、对齐。
4. **硬件**: GPU、CUDA、显存、量化、并行、推理加速。
5. **架构**: RAG、Agent、API、MLOps、评测、监控、上线。

## 资料

### 数据

1. [Hugging Face Datasets](https://huggingface.co/docs/datasets)
2. [Data-Centric AI](https://dcai.csail.mit.edu/)
3. [Label Studio](https://github.com/HumanSignal/label-studio)
4. [FAISS](https://github.com/facebookresearch/faiss)

### 算法

1. [Mathematics for Machine Learning](https://mml-book.github.io)
2. [CS229](https://www.youtube.com/playlist?list=PLoROMvodv4rMiGQp3WXShtMGgzqpfVfbU)
3. [Dive into Deep Learning](https://d2l.ai)
4. [micrograd](https://github.com/karpathy/micrograd)
5. [scikit-learn](https://github.com/scikit-learn/scikit-learn)

### 模型

1. [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
2. [BERT](https://arxiv.org/abs/1810.04805)
3. [GPT-3](https://arxiv.org/abs/2005.14165)
4. [LLaMA](https://arxiv.org/abs/2302.13971)
5. [LoRA](https://arxiv.org/abs/2106.09685)
6. [QLoRA](https://arxiv.org/abs/2305.14314)
7. [nanoGPT](https://github.com/karpathy/nanoGPT)
8. [transformers](https://github.com/huggingface/transformers)

### 硬件

1. [CUDA C++ Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/)
2. [GPU MODE](https://github.com/gpu-mode/lectures)
3. [FlashAttention](https://arxiv.org/abs/2205.14135)
4. [vLLM](https://github.com/vllm-project/vllm)
5. [llama.cpp](https://github.com/ggml-org/llama.cpp)
6. [Triton](https://github.com/triton-lang/triton)

### 架构

1. [AI Engineering](https://www.oreilly.com/library/view/ai-engineering/9781098166298/)
2. [Designing ML Systems](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/)
3. [llama_index](https://github.com/run-llama/llama_index)
4. [langchain](https://github.com/langchain-ai/langchain)
5. [MLflow](https://github.com/mlflow/mlflow)
6. [open-webui](https://github.com/open-webui/open-webui)

## index

1. [directions](directions/README.md) — AI 全栈方向
2. [roadmap](roadmap.md) — 12 周执行路线
3. [papers](papers/README.md) — 必读论文
4. [books](books/README.md) — 书籍
5. [repos](repos/README.md) — GitHub 仓库
6. [blogs](blogs/README.md) — 博客与网站
7. [videos](videos/README.md) — 视频与课程
8. [people](people/README.md) — 值得关注的人

---

<a id="roadmap"></a>

## Roadmap

来源:[roadmap.md](roadmap.md)

# AI 全栈成长路线

目标:12 周形成 AI 全栈最小闭环,之后持续在一个方向打深。

## 原则

- 每周交付一个可见产物:代码、评测表、笔记、demo、复现报告。
- 每个阶段都同时看五件事:数据、算法、模型、硬件、架构。
- 不只收藏资料,要把资料变成实验记录和作品集。

## 0-2 周:地基

| 任务 | 产物 |
| --- | --- |
| 学 PyTorch 张量、autograd、Dataset/DataLoader | 一个二分类/回归训练脚本 |
| 补线代、概率、优化基础 | 公式 + 代码小抄 |
| 跑通经典 ML baseline | sklearn notebook |
| 学 train/val/test 和指标 | 一个评测表 |

重点资料: [Mathematics for Machine Learning](https://mml-book.github.io)、[Dive into Deep Learning](https://d2l.ai)、[CS229](https://www.youtube.com/playlist?list=PLoROMvodv4rMiGQp3WXShtMGgzqpfVfbU)。

## 3-5 周:模型

| 任务 | 产物 |
| --- | --- |
| 手写 micrograd 或 tiny Transformer | 最小反向传播/Attention 实现 |
| 读 Attention、BERT、GPT-3、LoRA | 论文卡片 |
| 跑 nanoGPT 或 LLMs-from-scratch | loss 曲线 + 采样结果 |
| 用 LoRA 微调小模型 | 微调报告 |

重点资料: [nanoGPT](https://github.com/karpathy/nanoGPT)、[LLMs-from-scratch](https://github.com/rasbt/LLMs-from-scratch)、[Attention Is All You Need](https://arxiv.org/abs/1706.03762)。

## 6-8 周:应用架构

| 任务 | 产物 |
| --- | --- |
| 清洗自己的文章/笔记数据 | Markdown 数据集 |
| 搭建 RAG:切块、embedding、检索、引用 | 个人知识库问答 demo |
| 加评测:golden set、命中率、人工评分 | RAG eval 表 |
| 做 API + 简单 UI | 可演示应用 |

重点资料: [llama_index](https://github.com/run-llama/llama_index)、[langchain](https://github.com/langchain-ai/langchain)、[Designing ML Systems](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/)。

## 9-10 周:硬件与推理

| 任务 | 产物 |
| --- | --- |
| 学 GPU/CUDA 基础和显存估算 | 显存估算笔记 |
| 用 vLLM 或 llama.cpp 部署模型 | 本地推理服务 |
| 测试 FP16/INT8/INT4 | 速度-显存-质量对比表 |
| 了解 FlashAttention、KV Cache、batching | 推理优化笔记 |

重点资料: [vLLM](https://github.com/vllm-project/vllm)、[llama.cpp](https://github.com/ggml-org/llama.cpp)、[GPU MODE](https://github.com/gpu-mode/lectures)。

## 11-12 周:整合作品

做一个可以放进作品集的完整项目:

- 数据:有自己的数据集、清洗脚本、评测集。
- 算法:至少有一个自己实现或改造的核心模块。
- 模型:使用 RAG、微调、多模态或 Agent 中的一种能力。
- 硬件:有部署方案、性能指标、成本估算。
- 架构:有 API、日志、评测、监控和 README。

候选项目:

- 个人 AI 学习助手:对自己的笔记、文章、仓库做 RAG + Agent。
- 论文精读助手:arXiv 抓取、总结、引用定位、复现 checklist。
- AI 代码教练:读取代码仓库,生成学习路线、测试题和改进建议。
- 多模态知识库:图片/截图/文章统一检索和问答。

## 长期节奏

- 每天:读 1 篇短文或论文摘要,记录 3 个要点。
- 每周:复现 1 个模块或做 1 个实验。
- 每月:完成 1 篇系统总结或 1 个可演示 demo。
- 每季度:选一个方向深入,例如 LLM 系统、RAG/Agent、多模态、强化学习、端侧推理。

---

<a id="directions-index"></a>

## Directions Index

来源:[directions/README.md](directions/README.md)

# AI 全栈方向

> 主线解决“怎么成为 AI 高手”,专题解决“某个方向怎么深挖”。每篇尽量按:**是什么 → 必读 → 核心仓库 → 可做项目 → 追踪** 组织。

## 五大主线

- [data.md](directions/data.md) — 数据:采集、清洗、标注、评估集、向量化、数据飞轮。
- [algorithm.md](directions/algorithm.md) — 算法:数学、优化、经典 ML、深度学习、RL、检索。
- [model.md](directions/model.md) — 模型:Transformer、LLM、多模态、扩散、微调、对齐。
- [hardware.md](directions/hardware.md) — 硬件:GPU、CUDA、显存、量化、并行、推理加速。
- [architecture.md](directions/architecture.md) — 架构:RAG、Agent、服务化、MLOps、评测、监控。

## 专题深挖

- [llm.md](directions/llm.md) — 大语言模型:Transformer、预训练、微调、对齐、推理。
- [multimodal.md](directions/multimodal.md) — 多模态:视觉-语言,VLM,图文理解。
- [diffusion-genai.md](directions/diffusion-genai.md) — 生成与扩散:图像/视频/音频生成。
- [rl-rlhf.md](directions/rl-rlhf.md) — 强化学习与对齐:RL、RLHF/DPO、推理模型。
- [agents.md](directions/agents.md) — 智能体:工具使用、规划、多智能体。
- [rag.md](directions/rag.md) — 检索增强:RAG、向量检索、知识注入。
- [cv.md](directions/cv.md) — 计算机视觉:分类/检测/分割/自监督。
- [speech.md](directions/speech.md) — 语音:ASR/TTS/音频。
- [systems-infra.md](directions/systems-infra.md) — 系统与部署:量化、推理引擎、边缘端。

---

<a id="数据"></a>

## 数据

来源:[directions/data.md](directions/data.md)

# 数据

数据决定模型上限,评测决定迭代方向。AI 全栈里,数据不是前置步骤,而是贯穿产品、训练和反馈的循环系统。

## 要学什么

- 数据采集:爬虫、API、日志、公开数据集、用户反馈。
- 数据清洗:去重、脱敏、格式统一、异常检测、质量分层。
- 数据标注:人工标注、弱监督、LLM 辅助标注、主动学习。
- 数据表示:tokenization、embedding、向量库、特征工程。
- 数据评测:训练集/验证集/测试集、golden set、难例集、回归集。
- 数据飞轮:线上反馈 -> 错例分析 -> 数据补充 -> 再训练/再检索。

## 起点资料

- ⭐ [Hugging Face Datasets](https://huggingface.co/docs/datasets) — 数据集加载、处理、分享。
- ⭐ [Data-Centric AI](https://dcai.csail.mit.edu/) — 以数据为中心的 AI 方法。
- [Designing ML Systems](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/) — 数据分布、训练服务、监控。
- [Eugene Yan](https://eugeneyan.com) — 推荐系统、搜索、应用 ML 案例。

## 核心仓库

- [datasets](https://github.com/huggingface/datasets) — 数据集处理事实标准。
- [label-studio](https://github.com/HumanSignal/label-studio) — 开源标注平台。
- [datasketch](https://github.com/ekzhu/datasketch) — MinHash/LSH 去重。
- [faiss](https://github.com/facebookresearch/faiss) — 向量检索。

## 可做项目

- 为自己的笔记/文章构建 RAG 数据集:清洗 Markdown、切块、embedding、检索评测。
- 做一个小型指令数据集:收集 100 条任务,标注输入/输出/评分标准。
- 做难例集:记录模型失败样例,按幻觉、格式、推理、知识缺失分类。

## 检查点

- 能解释一个样本从原始数据到训练/检索输入的全过程。
- 能写脚本统计重复率、长度分布、空值、异常样本。
- 能维护一个小而稳定的评测集,每次改模型或 prompt 都能复测。

---

<a id="算法"></a>

## 算法

来源:[directions/algorithm.md](directions/algorithm.md)

# 算法

算法是理解 AI 的底层语言。目标不是背公式,而是知道模型为什么能学、为什么会错、怎么优化。

## 要学什么

- 数学基础:线性代数、概率统计、信息论、优化。
- 经典机器学习:线性模型、树模型、SVM、聚类、降维、评估指标。
- 深度学习:反向传播、初始化、归一化、优化器、正则化。
- 序列与注意力:RNN、Attention、Transformer、位置编码。
- 搜索与检索:BM25、ANN、HNSW、重排序、图搜索。
- 强化学习:MDP、Q-learning、Policy Gradient、PPO、奖励建模。

## 起点资料

- ⭐ [Mathematics for Machine Learning](https://mml-book.github.io) — 数学基础。
- ⭐ [Dive into Deep Learning](https://d2l.ai) — 理论 + 代码。
- ⭐ [CS229](https://www.youtube.com/playlist?list=PLoROMvodv4rMiGQp3WXShtMGgzqpfVfbU) — 机器学习主线。
- [The Hundred-Page ML Book](https://themlbook.com) — 快速全景。

## 核心仓库

- [scikit-learn](https://github.com/scikit-learn/scikit-learn) — 经典 ML。
- [tinygrad](https://github.com/tinygrad/tinygrad) — 小型深度学习框架。
- [micrograd](https://github.com/karpathy/micrograd) — 反向传播最小实现。
- [annotated deep learning](https://github.com/labmlai/annotated_deep_learning_paper_implementations) — 带注释论文实现。

## 可做项目

- 从零实现一个自动微分引擎,训练二分类模型。
- 手写 Transformer 的 Attention、LayerNorm、MLP、位置编码。
- 实现 BM25 + 向量检索 + rerank 的混合检索。

## 检查点

- 能不用框架推导并实现一次反向传播。
- 能解释过拟合、欠拟合、学习率、batch size 对训练的影响。
- 能读论文时抓住目标函数、模型结构、数据、指标四件事。

---

<a id="模型"></a>

## 模型

来源:[directions/model.md](directions/model.md)

# 模型

模型层关注“能力从哪里来”:架构、训练、微调、对齐、评测和失效模式。

## 要学什么

- LLM:tokenizer、Transformer、预训练、SFT、LoRA/QLoRA、DPO/RLHF。
- 多模态:CLIP、ViT、VLM、图文对齐、视觉编码器 + LLM。
- 扩散生成:DDPM、U-Net、Latent Diffusion、ControlNet、视频生成。
- 语音:ASR、TTS、音频表示、语音大模型。
- 评测:基准测试、人工偏好、任务成功率、幻觉、鲁棒性。
- 失效模式:数据泄漏、过拟合、奖励黑客、分布漂移、提示注入。

## 起点资料

- ⭐ [Build a LLM from Scratch](https://github.com/rasbt/LLMs-from-scratch) — 从零手写 GPT。
- ⭐ [nanoGPT](https://github.com/karpathy/nanoGPT) — 最清晰的 GPT 训练代码。
- ⭐ [Hands-On Large Language Models](https://github.com/HandsOnLLM/Hands-On-Large-Language-Models) — LLM 直觉与实践。
- [Umar Jamil](https://www.youtube.com/@umarjamilai) — 手写 Transformer/LLaMA/SD/RLHF。

## 核心仓库

- [transformers](https://github.com/huggingface/transformers) — 模型加载与训练。
- [peft](https://github.com/huggingface/peft) — LoRA/QLoRA。
- [trl](https://github.com/huggingface/trl) — SFT/DPO/RLHF。
- [diffusers](https://github.com/huggingface/diffusers) — 扩散模型。
- [whisper](https://github.com/openai/whisper) — 语音识别。

## 可做项目

- 微调一个小模型做自己的问答/分类/摘要任务。
- 复现一个最小 GPT:数据、训练、采样、loss 曲线全跑通。
- 给一个 RAG/Agent 应用建立评测集,比较 base model、SFT、RAG 的效果。

## 检查点

- 能解释 Transformer 一次 forward 的张量形状变化。
- 能区分预训练、SFT、DPO/RLHF、RAG 分别解决什么问题。
- 能用实验说明一次模型改动到底有没有变好。

---

<a id="硬件"></a>

## 硬件

来源:[directions/hardware.md](directions/hardware.md)

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

---

<a id="架构"></a>

## 架构

来源:[directions/architecture.md](directions/architecture.md)

# 架构

架构层关注“怎么把 AI 能力变成可靠系统”。它连接数据、模型、硬件和真实用户。

## 要学什么

- 应用模式:RAG、Agent、Workflow、多模态应用、工具调用。
- 服务化:API、队列、缓存、向量库、对象存储、数据库。
- MLOps/LLMOps:训练流水线、模型注册、灰度发布、回滚、监控。
- 评测体系:离线评测、线上 A/B、人工反馈、错误分类、成本指标。
- 安全与可靠性:权限、脱敏、prompt injection、防越权、审计日志。
- 产品闭环:用户行为 -> 日志 -> 数据集 -> 评测 -> 模型/检索迭代。

## 起点资料

- ⭐ [AI Engineering](https://www.oreilly.com/library/view/ai-engineering/9781098166298/) — AI 应用工程全景。
- ⭐ [Designing ML Systems](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/) — ML 系统设计。
- [Chip Huyen Blog](https://huyenchip.com/blog/) — 工程化和数据闭环。
- [Eugene Yan](https://eugeneyan.com) — 工业界应用案例。

## 核心仓库

- [llama_index](https://github.com/run-llama/llama_index) — 数据接入与 RAG。
- [langchain](https://github.com/langchain-ai/langchain) — 应用编排。
- [open-webui](https://github.com/open-webui/open-webui) — 本地 LLM Web UI。
- [mlflow](https://github.com/mlflow/mlflow) — 实验追踪与模型管理。
- [fastapi](https://github.com/fastapi/fastapi) — Python API 服务。

## 可做项目

- 搭建个人知识库 RAG:解析、切块、向量化、检索、答案引用、评测。
- 做一个 Agent 工具系统:搜索、文件读写、代码执行、日志追踪、权限控制。
- 做一个模型服务压测面板:QPS、延迟、错误率、成本、命中率。

## 检查点

- 能画出一个 AI 应用从请求到模型响应再到日志回流的链路。
- 能为一个任务设计离线评测集和线上监控指标。
- 能解释什么时候用 RAG、什么时候微调、什么时候只改 prompt。

---

<a id="llm"></a>

## LLM

来源:[directions/llm.md](directions/llm.md)

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

---

<a id="rag"></a>

## RAG

来源:[directions/rag.md](directions/rag.md)

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

---

<a id="agents"></a>

## Agents

来源:[directions/agents.md](directions/agents.md)

# 智能体 (Agents)

**是什么**:让 LLM 规划、调用工具、与环境交互并迭代。核心:推理 + 工具 + 记忆 + 反馈闭环。

## 必读
1. ReAct(推理+行动)— [2210.03629](https://arxiv.org/abs/2210.03629)
2. Toolformer(自学用工具)— [2302.04761](https://arxiv.org/abs/2302.04761)
3. Reflexion(自我反思)— [2303.11366](https://arxiv.org/abs/2303.11366)
4. Voyager(开放世界终身学习)— [2305.16291](https://arxiv.org/abs/2305.16291)
5. Generative Agents(斯坦福小镇)— [2304.03442](https://arxiv.org/abs/2304.03442)
6. 综述:[Agent survey](https://arxiv.org/abs/2308.11432)

## 核心仓库
- [LangGraph](https://github.com/langchain-ai/langgraph) · [AutoGen](https://github.com/microsoft/autogen) · [CrewAI](https://github.com/crewAIInc/crewAI) · [OpenHands](https://github.com/All-Hands-AI/OpenHands)
- 协议:[MCP (Model Context Protocol)](https://github.com/modelcontextprotocol)

## 追踪
[Simon Willison](https://simonwillison.net) · [HF Papers](https://huggingface.co/papers)

---

<a id="multimodal"></a>

## Multimodal

来源:[directions/multimodal.md](directions/multimodal.md)

# 多模态 (Multimodal / VLM)

**是什么**:把图像/视频/音频与语言对齐。主流范式:视觉编码器(ViT)→ 投影/连接器 → LLM。

## 必读
1. CLIP(图文对比学习)— [2103.00020](https://arxiv.org/abs/2103.00020)
2. ViT — [2010.11929](https://arxiv.org/abs/2010.11929)
3. LLaVA(视觉指令微调)— [2304.08485](https://arxiv.org/abs/2304.08485)
4. Flamingo(交错图文)— [2204.14198](https://arxiv.org/abs/2204.14198)
5. Qwen-VL / Qwen2-VL — [2308.12966](https://arxiv.org/abs/2308.12966) · [2409.12191](https://arxiv.org/abs/2409.12191)
6. BLIP-2(Q-Former 连接器)— [2301.12597](https://arxiv.org/abs/2301.12597)

## 核心仓库
- [LLaVA](https://github.com/haotian-liu/LLaVA) · [Qwen-VL](https://github.com/QwenLM/Qwen-VL) · [open_clip](https://github.com/mlfoundations/open_clip)
- 端侧移植参考:[Penguin-VL-ncnn](https://github.com/lvy010/penguin-vl-ncnn)(VLM→ncnn 全链路)

## 追踪
[HF Papers](https://huggingface.co/papers) · [Papers with Code – VQA](https://paperswithcode.com/task/visual-question-answering)

---

<a id="diffusion"></a>

## Diffusion

来源:[directions/diffusion-genai.md](directions/diffusion-genai.md)

# 生成与扩散 (Generative / Diffusion)

**是什么**:从噪声逐步去噪生成图像/视频/音频。主线:DDPM → Latent Diffusion(SD)→ DiT/流匹配。

## 必读
1. DDPM(扩散奠基)— [2006.11239](https://arxiv.org/abs/2006.11239)
2. Latent Diffusion / Stable Diffusion — [2112.10752](https://arxiv.org/abs/2112.10752)
3. Classifier-Free Guidance — [2207.12598](https://arxiv.org/abs/2207.12598)
4. DiT(Transformer 扩散骨干)— [2212.09748](https://arxiv.org/abs/2212.09748)
5. Flow Matching — [2210.02747](https://arxiv.org/abs/2210.02747)
6. (对比)GAN — [1406.2661](https://arxiv.org/abs/1406.2661)

## 核心仓库
- [diffusers](https://github.com/huggingface/diffusers) · [ComfyUI](https://github.com/comfyanonymous/ComfyUI) · [stable-diffusion](https://github.com/CompVis/stable-diffusion)

## 追踪
[Two Minute Papers](https://www.youtube.com/@TwoMinutePapers) · [HF Papers](https://huggingface.co/papers)

---

<a id="rl--rlhf"></a>

## RL / RLHF

来源:[directions/rl-rlhf.md](directions/rl-rlhf.md)

# 强化学习与对齐 (RL / RLHF)

**是什么**:通过奖励/偏好优化策略。两条线:经典 RL(控制/游戏)与 LLM 后训练(RLHF/DPO/推理模型)。

## 必读
1. Sutton & Barto 教材 — [免费书](http://incompleteideas.net/book/the-book.html)
2. DQN(Atari)— [1312.5602](https://arxiv.org/abs/1312.5602)
3. PPO — [1707.06347](https://arxiv.org/abs/1707.06347)
4. RL from Human Preferences — [1706.03741](https://arxiv.org/abs/1706.03741)
5. InstructGPT(RLHF 落地)— [2203.02155](https://arxiv.org/abs/2203.02155)
6. DPO — [2305.18290](https://arxiv.org/abs/2305.18290) · DeepSeek-R1(RL 驱动推理)— [2501.12948](https://arxiv.org/abs/2501.12948)

## 核心仓库
- [Spinning Up](https://github.com/openai/spinningup) · [CleanRL](https://github.com/vwxyzjn/cleanrl)(单文件实现)· [Gymnasium](https://github.com/Farama-Foundation/Gymnasium)
- LLM 对齐:[trl](https://github.com/huggingface/trl) · [OpenRLHF](https://github.com/OpenRLHF/OpenRLHF)

## 追踪
[Interconnects](https://www.interconnects.ai)(RLHF 最佳追踪)

---

<a id="cv"></a>

## CV

来源:[directions/cv.md](directions/cv.md)

# 计算机视觉 (CV)

**是什么**:图像/视频的分类、检测、分割、生成、自监督表示。经典 CNN → ViT → 基础模型(SAM/DINO)。

## 必读
1. ResNet — [1512.03385](https://arxiv.org/abs/1512.03385)
2. ViT — [2010.11929](https://arxiv.org/abs/2010.11929)
3. U-Net(分割/扩散骨干)— [1505.04597](https://arxiv.org/abs/1505.04597)
4. YOLO(实时检测)— [1506.02640](https://arxiv.org/abs/1506.02640)
5. Segment Anything(SAM)— [2304.02643](https://arxiv.org/abs/2304.02643)
6. DINOv2(自监督)— [2304.07193](https://arxiv.org/abs/2304.07193)

## 核心仓库
- [timm (pytorch-image-models)](https://github.com/huggingface/pytorch-image-models) · [detectron2](https://github.com/facebookresearch/detectron2) · [ultralytics (YOLO)](https://github.com/ultralytics/ultralytics) · [mmdetection](https://github.com/open-mmlab/mmdetection) · [segment-anything](https://github.com/facebookresearch/segment-anything)

## 追踪
[CS231n](https://cs231n.stanford.edu) · [Papers with Code – CV](https://paperswithcode.com/area/computer-vision)

---

<a id="speech"></a>

## Speech

来源:[directions/speech.md](directions/speech.md)

# 语音 (Speech / Audio)

**是什么**:语音识别(ASR)、语音合成(TTS)、音频理解与生成。

## 必读
1. Whisper(鲁棒 ASR)— [2212.04356](https://arxiv.org/abs/2212.04356)
2. wav2vec 2.0(自监督语音)— [2006.11477](https://arxiv.org/abs/2006.11477)
3. Tacotron 2(TTS)— [1712.05884](https://arxiv.org/abs/1712.05884)
4. VALL-E(神经编解码 TTS)— [2301.02111](https://arxiv.org/abs/2301.02111)
5. EnCodec(神经音频编解码)— [2210.13438](https://arxiv.org/abs/2210.13438)

## 核心仓库
- [whisper](https://github.com/openai/whisper) · [whisper.cpp](https://github.com/ggml-org/whisper.cpp)(端侧)· [faster-whisper](https://github.com/SYSTRAN/faster-whisper)
- [coqui TTS](https://github.com/coqui-ai/TTS) · [espnet](https://github.com/espnet/espnet)

## 追踪
[Papers with Code – Speech](https://paperswithcode.com/area/speech)

---

<a id="systems"></a>

## Systems

来源:[directions/systems-infra.md](directions/systems-infra.md)

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

---

<a id="papers"></a>

## Papers

来源:[papers/README.md](papers/README.md)

# 必读论文

按主题排,大致自上而下读。⭐ = 必读。进阶见 [directions/](directions/README.md)。

## 基础构件
- ⭐ Seq2Seq — [1409.3215](https://arxiv.org/abs/1409.3215)
- Attention (Bahdanau) — [1409.0473](https://arxiv.org/abs/1409.0473)
- ⭐ ResNet — [1512.03385](https://arxiv.org/abs/1512.03385)
- Batch Normalization — [1502.03167](https://arxiv.org/abs/1502.03167)
- Adam — [1412.6980](https://arxiv.org/abs/1412.6980)
- Word2Vec — [1301.3781](https://arxiv.org/abs/1301.3781)
- GAN — [1406.2661](https://arxiv.org/abs/1406.2661)

## Transformer / LLM
- ⭐ Attention Is All You Need — [1706.03762](https://arxiv.org/abs/1706.03762)
- ⭐ BERT — [1810.04805](https://arxiv.org/abs/1810.04805)
- ⭐ GPT-3 — [2005.14165](https://arxiv.org/abs/2005.14165)
- Scaling Laws — [2001.08361](https://arxiv.org/abs/2001.08361)
- Chinchilla — [2203.15556](https://arxiv.org/abs/2203.15556)
- ⭐ InstructGPT (RLHF) — [2203.02155](https://arxiv.org/abs/2203.02155)
- Chain-of-Thought — [2201.11903](https://arxiv.org/abs/2201.11903)
- ⭐ LoRA — [2106.09685](https://arxiv.org/abs/2106.09685)
- QLoRA — [2305.14314](https://arxiv.org/abs/2305.14314)
- LLaMA — [2302.13971](https://arxiv.org/abs/2302.13971)
- Mixtral (MoE) — [2401.04088](https://arxiv.org/abs/2401.04088)
- DPO — [2305.18290](https://arxiv.org/abs/2305.18290)
- DeepSeek-R1 — [2501.12948](https://arxiv.org/abs/2501.12948)

## 高效注意力 / 架构
- ⭐ FlashAttention — [2205.14135](https://arxiv.org/abs/2205.14135)
- FlashAttention-2 — [2307.08691](https://arxiv.org/abs/2307.08691)
- Mamba (SSM) — [2312.00752](https://arxiv.org/abs/2312.00752)
- RoPE — [2104.09864](https://arxiv.org/abs/2104.09864)

## 视觉 / 多模态
- ⭐ ViT — [2010.11929](https://arxiv.org/abs/2010.11929)
- ⭐ CLIP — [2103.00020](https://arxiv.org/abs/2103.00020)
- ⭐ SAM — [2304.02643](https://arxiv.org/abs/2304.02643)
- DINOv2 — [2304.07193](https://arxiv.org/abs/2304.07193)
- LLaVA — [2304.08485](https://arxiv.org/abs/2304.08485)
- Qwen-VL — [2308.12966](https://arxiv.org/abs/2308.12966)

## 生成 / 扩散
- ⭐ DDPM — [2006.11239](https://arxiv.org/abs/2006.11239)
- ⭐ Latent Diffusion (SD) — [2112.10752](https://arxiv.org/abs/2112.10752)
- Classifier-Free Guidance — [2207.12598](https://arxiv.org/abs/2207.12598)
- U-Net — [1505.04597](https://arxiv.org/abs/1505.04597)

## RL / 智能体 / 检索
- DQN — [1312.5602](https://arxiv.org/abs/1312.5602)
- PPO — [1707.06347](https://arxiv.org/abs/1707.06347)
- RL from Human Preferences — [1706.03741](https://arxiv.org/abs/1706.03741)
- ⭐ RAG — [2005.11401](https://arxiv.org/abs/2005.11401)
- ReAct — [2210.03629](https://arxiv.org/abs/2210.03629)
- Toolformer — [2302.04761](https://arxiv.org/abs/2302.04761)

---
追新:[arXiv cs.CL](https://arxiv.org/list/cs.CL/recent) · [Papers with Code](https://paperswithcode.com) · [HF Papers](https://huggingface.co/papers)

---

<a id="books"></a>

## Books

来源:[books/README.md](books/README.md)

# 书籍

## 数学 / 基础
- ⭐ [Mathematics for Machine Learning](https://mml-book.github.io) — ML 所需数学,免费。
- [Dive into Deep Learning](https://d2l.ai) — 边讲边写代码,交互式。
- [The Hundred-Page ML Book](https://themlbook.com) — 极简全景。

## 深度学习 / 机器学习
- ⭐ [Understanding Deep Learning](https://udlbook.github.io/udlbook/) — 现代 DL 教材,免费。
- [Deep Learning](https://www.deeplearningbook.org) — Goodfellow,经典奠基,免费。
- [Probabilistic ML](https://probml.github.io/pml-book/) — Murphy,概率视角权威,免费。
- [PRML](https://www.microsoft.com/en-us/research/publication/pattern-recognition-machine-learning/) — Bishop,贝叶斯经典,免费。
- [Hands-On ML](https://github.com/ageron/handson-ml3) — Géron,工程实战。

## LLM / NLP
- ⭐ [Speech and Language Processing (SLP3)](https://web.stanford.edu/~jurafsky/slp3/) — NLP 圣经,免费。
- [Build a LLM from Scratch](https://github.com/rasbt/LLMs-from-scratch) — 从零手写 GPT。
- [Hands-On Large Language Models](https://github.com/HandsOnLLM/Hands-On-Large-Language-Models) — 可视化直觉。

## 强化学习
- ⭐ [RL: An Introduction](http://incompleteideas.net/book/the-book.html) — Sutton & Barto,免费。
- [Spinning Up in Deep RL](https://spinningup.openai.com) — OpenAI,上手教程。

## 工程 / 系统
- ⭐ [AI Engineering](https://www.oreilly.com/library/view/ai-engineering/9781098166298/) — Chip Huyen,2025。
- [Designing ML Systems](https://www.oreilly.com/library/view/designing-machine-learning/9781098107956/) — ML 系统设计。
- [ML Systems (mlsysbook)](https://mlsysbook.ai) — 边缘/TinyML,免费。

---

<a id="repos"></a>

## Repos

来源:[repos/README.md](repos/README.md)

# GitHub 仓库

## 框架 / 训练
- ⭐ [pytorch](https://github.com/pytorch/pytorch) — 框架事实标准。
- ⭐ [transformers](https://github.com/huggingface/transformers) — 预训练模型全家桶。
- [diffusers](https://github.com/huggingface/diffusers) — 扩散模型标准库。
- [peft](https://github.com/huggingface/peft) — LoRA/QLoRA 微调。
- [jax](https://github.com/jax-ml/jax) — 自动微分 + XLA。
- [DeepSpeed](https://github.com/microsoft/DeepSpeed) — 大规模训练/并行。
- ⭐ [unsloth](https://github.com/unslothai/unsloth) — 更快省显存微调。

## 从零学习(读源码)
- ⭐ [nanoGPT](https://github.com/karpathy/nanoGPT) — 最干净的 GPT 训练。
- ⭐ [LLMs-from-scratch](https://github.com/rasbt/LLMs-from-scratch) — 从零手写 LLM。
- [minGPT](https://github.com/karpathy/minGPT) — 极简 GPT。
- [llm.c](https://github.com/karpathy/llm.c) — 纯 C/CUDA 训 GPT-2。
- [annotated DL implementations](https://github.com/labmlai/annotated_deep_learning_paper_implementations) — 带注释论文实现。

## 推理 / 部署
- ⭐ [llama.cpp](https://github.com/ggml-org/llama.cpp) — CPU/端侧推理,GGUF。
- ⭐ [vllm](https://github.com/vllm-project/vllm) — 高吞吐服务。
- ⭐ [ncnn](https://github.com/Tencent/ncnn) — 移动/边缘端推理。
- [ollama](https://github.com/ollama/ollama) — 本地一键跑 LLM。
- [open-webui](https://github.com/open-webui/open-webui) — 本地 LLM Web UI。
- [flash-attention](https://github.com/Dao-AILab/flash-attention) — 官方内核。

## 应用 / Agent / RAG
- [langchain](https://github.com/langchain-ai/langchain) — 应用编排。
- [llama_index](https://github.com/run-llama/llama_index) — RAG/数据接入。
- [faiss](https://github.com/facebookresearch/faiss) — 向量检索。
- [whisper](https://github.com/openai/whisper) — 语音识别。
- [segment-anything](https://github.com/facebookresearch/segment-anything) — 通用分割。

## 清单
- [Awesome-LLM](https://github.com/Hannibal046/Awesome-LLM) — LLM 全景清单。
- [llm-course](https://github.com/mlabonne/llm-course) — 学习路线 + notebook。
- [Prompt-Engineering-Guide](https://github.com/dair-ai/Prompt-Engineering-Guide) — 提示工程。
- [applied-ml](https://github.com/eugeneyan/applied-ml) — 工业界落地案例。

---

<a id="blogs"></a>

## Blogs

来源:[blogs/README.md](blogs/README.md)

# 博客与网站

## 个人技术博客
- ⭐ [Lil'Log](https://lilianweng.github.io) — 每篇都是小综述。
- ⭐ [Ahead of AI](https://magazine.sebastianraschka.com) — LLM 训练/微调拆解。
- ⭐ [Jay Alammar](https://jalammar.github.io) — 图解 Transformer/GPT。
- [Andrej Karpathy](https://karpathy.github.io) — 少而精。
- [Chip Huyen](https://huyenchip.com/blog/) — ML 系统与工程。
- [Interconnects](https://www.interconnects.ai) — 后训练/RLHF/开源前沿。
- [Eugene Yan](https://eugeneyan.com) — 应用 ML/LLM 落地。
- [Simon Willison](https://simonwillison.net) — LLM 实用主义,更新勤。

## 可视化 / 可解释
- ⭐ [Distill.pub](https://distill.pub) — 交互式论文标杆。
- [Transformer Circuits](https://transformer-circuits.pub) — Anthropic 可解释性。

## 实验室 / 公司
- ⭐ [Hugging Face Blog](https://huggingface.co/blog) — 开源模型/方法,最实用。
- [OpenAI](https://openai.com/research) · [Anthropic](https://www.anthropic.com/research) · [DeepMind](https://deepmind.google/discover/blog/) — 前沿一手。
- [BAIR](https://bair.berkeley.edu/blog/) — 伯克利研究。
- [Meta AI](https://ai.meta.com/blog/) — Llama/开源。

## Newsletter
- [The Batch](https://www.deeplearning.ai/the-batch/) — 每周要闻。
- [Import AI](https://importai.substack.com) — 政策+研究。
- [HF Daily Papers](https://huggingface.co/papers) — 每日论文热榜。

---

<a id="videos"></a>

## Videos

来源:[videos/README.md](videos/README.md)

# YouTube 频道与课程

## 建立直觉
- ⭐ [3Blue1Brown](https://www.youtube.com/@3blue1brown) — 神经网络/Transformer 可视化。
- [StatQuest](https://www.youtube.com/@statquest) — 统计与 ML,极易懂。
- [Welch Labs](https://www.youtube.com/@WelchLabsVideo) — 概念可视化。

## 从零写代码
- ⭐ [Andrej Karpathy](https://www.youtube.com/@AndrejKarpathy) — Zero to Hero,micrograd 到 GPT。
- ⭐ [Umar Jamil](https://www.youtube.com/@umarjamilai) — 手写 Transformer/LLaMA/SD/RLHF。
- [Sebastian Raschka](https://www.youtube.com/@SebastianRaschka) — LLM 从零。

## 论文 / 前沿
- [Yannic Kilcher](https://www.youtube.com/@YannicKilcher) — 论文精读。
- [Two Minute Papers](https://www.youtube.com/@TwoMinutePapers) — 前沿速览。
- [AI Coffee Break](https://www.youtube.com/@AICoffeeBreak) — 论文解读。

## 系统课程(免费)
- ⭐ [Stanford CS229](https://www.youtube.com/playlist?list=PLoROMvodv4rMiGQp3WXShtMGgzqpfVfbU) — 机器学习基础。
- ⭐ [Stanford CS231n](https://www.youtube.com/playlist?list=PL3FW7Lu3i5JvHM8ljYj-zLfQRF3EO8sYv) — 视觉。
- [Stanford CS224n](https://www.youtube.com/playlist?list=PLoROMvodv4rOSH4v6133s9LFPRHjEmbmJ) — NLP。
- [Stanford CS25](https://www.youtube.com/playlist?list=PLoROMvodv4rNiJRchCzutFw5ItR_Z27CM) — Transformers 前沿。
- [MIT 6.S191](https://www.youtube.com/playlist?list=PLtBw6njQRU-rwp5__7C0oIVt26ZgjG9NI) — 快速全景。
- [DeepLearning.AI](https://www.youtube.com/@Deeplearningai) — 短课(RAG/Agents/微调)。

## 访谈
- [Lex Fridman](https://www.youtube.com/@lexfridman) — 深度长访谈。
- [Dwarkesh Patel](https://www.youtube.com/@DwarkeshPatel) — 研究者访谈。

---

<a id="people"></a>

## People

来源:[people/README.md](people/README.md)

# 值得关注的人

## 奠基 / 领袖
- [Yann LeCun](https://x.com/ylecun) — CNN、自监督、世界模型。
- [Yoshua Bengio](https://yoshuabengio.org) — 深度学习、AI 安全。
- [Ilya Sutskever](https://x.com/ilyasut) — Seq2Seq、GPT。
- [Andrew Ng](https://x.com/AndrewYNg) — 教育、应用 ML。
- [Demis Hassabis](https://x.com/demishassabis) — DeepMind、AlphaFold。
- [Fei-Fei Li](https://x.com/drfeifei) — ImageNet、空间智能。

## 讲得好 / 能上手学
- ⭐ [Andrej Karpathy](https://x.com/karpathy) — 从零讲透 LLM。
- ⭐ [Lilian Weng](https://x.com/lilianweng) — 深度技术长文。
- ⭐ [Sebastian Raschka](https://x.com/rasbt) — LLM 从零、清晰。
- [Jay Alammar](https://x.com/JayAlammar) — 图解 Transformer/LLM。
- [Jeremy Howard](https://x.com/jeremyphoward) — fast.ai。
- [François Chollet](https://x.com/fchollet) — Keras、ARC 推理。
- [Chris Olah](https://x.com/ch402) — 可解释性、Distill。
- [Umar Jamil](https://www.youtube.com/@umarjamilai) — 逐行手写论文实现。

## 前沿 / 系统与推理
- [Jim Fan](https://x.com/DrJimFan) — 具身智能、机器人。
- [Tri Dao](https://x.com/tri_dao) — FlashAttention、Mamba。
- [Nathan Lambert](https://x.com/natolambert) — RLHF/后训练。
- [Philipp Schmid](https://x.com/_philschmid) — 开源 LLM 工程。
- [Aleksa Gordić](https://x.com/gordic_aleksa) — 从零讲底层。
- [nihui](https://github.com/nihui) — ncnn 作者、端侧推理。
