# 递归自我改进（RSI）

**是什么**：让改进后的系统参与产生后续改进。关键不在循环次数，而在被改进的对象是否重新成为更强的改进者。

该目录区分直接自修改、有界优化、自训练、理论与评测，并为每项工作记录改进对象、反馈、保留状态和能力边界。


## 判断框架

阅读一项“self-improvement”工作时，先回答：

1. 谁是 improver，谁是被改进对象？
2. 哪些组件可以修改，哪些组件保持固定？
3. Feedback 来自训练集、验证集、独立 evaluator，还是模型自评？
4. 哪些状态会保留到下一轮？
5. 改进后的系统是否重新进入循环，并提高后续改进能力？

可以将 claim 依次分为：

- **Task Improvement**：改答案、reasoning、trajectory 或局部 memory。
- **Model Improvement**：改数据、reward、curriculum 或 weights。
- **System Improvement**：改 prompt、skill、tool、workflow、code 或 harness。
- **Improver Improvement**：改 trainer、environment engineer 或 researcher。
- **Recursive Improvement**：新的 improver 回到下一轮，继续提高改进能力。

## 1. 答案、推理与轨迹

- [Self-Refine](https://arxiv.org/abs/2303.17651)：生成、反馈、修订同一任务输出。
- [Reflexion](https://arxiv.org/abs/2303.11366)：把失败总结为语言反馈并写入 episodic memory。
- [STaR](https://arxiv.org/abs/2203.14465)：将成功 reasoning 回灌为训练数据。
- [Voyager](https://arxiv.org/abs/2305.16291)：从交互经验中积累可检索、组合的技能。

这类工作主要证明固定机制中的行为改进，不应仅因多轮执行就称为 RSI。更多相邻方法及排除理由见 [Related Methods](../../../awesome-rsi/docs/related_methods.md)。

## 2. 数据、奖励与模型权重

- [Self-Instruct](https://arxiv.org/abs/2212.10560)：用模型生成 instruction data。
- [WizardLM / Evol-Instruct](https://arxiv.org/abs/2304.12244)：通过演化规则提高指令复杂度。
- [SPIN](https://arxiv.org/abs/2401.01335)：让模型与历史版本生成的数据进行迭代训练。
- [Self-Rewarding Language Models](https://arxiv.org/abs/2401.10020)：模型同时参与回答与 reward 生成。
- [Absolute Zero](https://arxiv.org/abs/2505.03335)：自动生成、求解并验证可检验任务。
- [SEAL](https://arxiv.org/abs/2506.10943)：生成 adaptation data 和 update directives，再更新模型。

这一层需要区分“模型参与产生训练信号”和“模型自主决定训练机制”。训练算法、评测协议和接受门槛通常仍由人规定。

## 3. Prompt、Skill、Tool 与 Harness

- [Darwin Gödel Machine](https://arxiv.org/abs/2505.22954)：修改编码代理实现，评测后代并从档案继续分支。
- [Automated Design of Agentic Systems](https://arxiv.org/abs/2408.08435)：把 agent architecture 作为搜索对象。
- [Self-Taught Optimizer](https://arxiv.org/abs/2310.02304)：将程序优化器应用于自身代码。
- [DSPy](https://github.com/stanfordnlp/dspy)：针对任务指标优化 LM program 的指令与示例。
- [GEPA](https://github.com/gepa-ai/gepa)：依据执行轨迹和反馈演化 prompt 候选。
- [Prime Agent](https://github.com/PrimeIntellect-ai/prime-agent)：保留 prompt、memory、skill 和 subagent 的 harness 修订。

Harness 比 weights 更容易审计、比较和回滚，但“成功写入经验”不等于“后续能够检索、理解并稳定使用经验”。

## 4. 环境、任务与 Curriculum

- [PAIRED](https://arxiv.org/abs/2010.03934)：通过对抗式环境生成构造适合当前 learner 的任务。
- [POET](https://arxiv.org/abs/1901.01753)：共同演化环境与解决方案。
- [R-Zero](https://github.com/Chengsong-Huang/R-Zero)：通过 challenger/solver 共演化扩展训练问题分布。
- [General Agent](https://www.primeintellect.ai/blog/general-agent)：按 solver 表现保留校准难度的合成任务族。

环境演化的核心问题是：更强的 policy 是否也变成了更好的 task 或 environment designer，而不只是更高分的 solver。

## 5. 自动化 AI 研究

- [The AI Scientist](https://arxiv.org/abs/2408.06292)：覆盖想法、实验、分析、写作与评审的自动科研流程。
- [MLE-bench](https://arxiv.org/abs/2410.07095)：用机器学习竞赛任务评估 agent 的端到端工程与建模能力。
- [CORE-Bench](https://arxiv.org/abs/2409.11363)：评估计算研究复现能力。
- [RD-Agent](https://github.com/microsoft/RD-Agent)：让 agent 生成训练配置、训练目标模型并使用验证反馈迭代。
- [AI Scientist 官方项目](https://github.com/SakanaAI/AI-Scientist)：可执行的自动科研实现与实验框架。

端到端 benchmark 测“最终能否完成”，service-native benchmark 测“能力提升来自哪里”。两者互补，不能用最终分数替代归因。

## 6. Trainer、Researcher 与递归闭环

这一层关注 improver 本身：它能否更准确地诊断失败、选择实验、识别虚假提升、保留最佳版本，并将研究经验迁移到新任务

- [Anthropic: When AI builds itself](https://www.anthropic.com/institute/recursive-self-improvement)：严格的 successor-oriented RSI 定义与研究议程。
- [Sakana AI RSI Lab](https://sakana.ai/rsi-lab/)：从自修改 agent 到 AI researcher 的研究路线。

## 实验架构：Everything as Service

可复现的 RSI 实验应把 Model、Data、Train、Serve、Eval、Environment、Harness 和 Agent 拆成可版本化、可替换、可审计的 service。每轮实验明确：

- 允许修改什么。
- 哪些 service 固定。
- 谁提供 feedback。
- 谁决定保留、回滚或晋升。
- 新版本如何重新进入下一轮。

推荐同时报告 Discovery、Selection、Retention、Transfer、Efficiency、Improver Gain 和 Compounding，而不只报告最终 task score。