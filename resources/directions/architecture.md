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