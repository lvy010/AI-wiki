# Interstellar · AI 全栈学习思路

目标:成为能理解原理、训练模型、部署系统、追踪前沿并做出作品的 AI 全栈工程师。

AI 全栈不是只会调 API,而是把 **数据、算法、模型、硬件、架构** 串成一条闭环:

```text
数据 -> 算法 -> 模型 -> 硬件 -> 架构 -> 产品/研究反馈 -> 数据
```

## 五大能力柱

| 主线 | 你要掌握什么 | 能做出的东西 |
| --- | --- | --- |
| [数据](directions/data.md) | 数据采集、清洗、标注、评估集、向量化、数据飞轮 | 一个可持续迭代的数据集和评测面板 |
| [算法](directions/algorithm.md) | 数学、优化、经典 ML、深度学习、RL、检索算法 | 从零实现训练/推理/搜索核心模块 |
| [模型](directions/model.md) | Transformer、LLM、多模态、扩散、微调、对齐 | 训练或微调一个能解决真实任务的模型 |
| [硬件](directions/hardware.md) | GPU、CUDA、显存、量化、并行、推理加速 | 把模型跑得更快、更省、更稳定 |
| [架构](directions/architecture.md) | RAG、Agent、服务化、MLOps、评测、监控 | 一个可上线、可观测、可迭代的 AI 应用 |

## 推荐路线

1. **第 1 阶段:补地基** — Python/PyTorch、线代概率、经典 ML、训练一个小模型。
2. **第 2 阶段:读懂模型** — Transformer、BERT/GPT、LoRA、RAG、评测。
3. **第 3 阶段:做出作品** — 选一个真实场景,完成数据集、微调/RAG、API、前端或机器人入口。
4. **第 4 阶段:深入系统** — vLLM/llama.cpp、量化、KV Cache、并发、GPU 性能分析。
5. **第 5 阶段:追前沿** — 每周读论文、复现关键模块、写总结、把想法变成 demo。

执行版见 [roadmap.md](roadmap.md)。

## 资料库

每条一行:`名称 — 价值`。⭐ = 起点。

- [directions/](directions/) — AI 全栈主线 + LLM / 多模态 / 扩散 / RL·RLHF / 智能体 / RAG / CV / 语音 / 系统部署专题
- [papers/](papers/README.md) — 必读论文(按主题,含 arXiv)
- [books/](books/README.md) — 书籍(多数免费在线)
- [people/](people/README.md) — 值得关注的人
- [videos/](videos/README.md) — YouTube 频道与课程
- [blogs/](blogs/README.md) — 博客与网站
- [repos/](repos/README.md) — GitHub 仓库

## 学习方法

- 每周只抓一个主题:读 1 篇综述/教材章节,看 1 个实现,做 1 个小实验,写 1 页总结。
- 任何知识都落到一个产物:代码、笔记、评测表、架构图、复现报告或 demo。
- 学模型时顺手学数据和评测;学应用时顺手学部署和监控;学系统时顺手看硬件瓶颈。
- 不追求一次学完,追求形成循环:问题 -> 资料 -> 实验 -> 记录 -> 改进。
