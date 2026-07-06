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