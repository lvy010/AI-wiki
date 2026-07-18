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