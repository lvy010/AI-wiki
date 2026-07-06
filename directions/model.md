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