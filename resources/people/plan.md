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