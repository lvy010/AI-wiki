
[taac26 record](https://github.com/lvy010/TAAR)

<p align="left">
	<img src="https://i-blog.csdnimg.cn/direct/b53cbabfe4654018bafd9d0138f5987a.jpg" alt="LLM4Rec 架构示意图" width="50%">
</p>
 
`Transformer` 和 `Scaling Laws` 的框架下逐渐合流？
 
一、推荐系统正在==从“`打分排序`”走向“`序列生成`”==
 
传统推荐系统的主流程，是工业界非常熟悉的多阶段级联架构：
 
先从海量物品中召回几千个候选，再粗排到几百个，再精排到几十个，最后经过`重排`、`多样性`、`规则策略`等模块，生成用户真正看到的列表
 
这个流程非常强大，也非常工程化。它最大的优点是稳定、可控、延迟低、容易拆分优化。

每个阶段都有明确职责：
`召回负责覆盖，粗排负责效率，精排负责精准，重排负责业务约束和体验`。但它也有天然问题。
 
第一，`阶段之间会有信息损失`。召回阶段没拿到的物品，后面模型再强也排不出来。
 
第二，不同模块各自优化，目标并不完全一致。召回看 `Recall`，粗排看 AUC，精排看 CTR/CVR，重排还要看多样性和策略规则，最终用户体验其实是这些`局部目标拼起来的结果`
 
第三，模型规模继续变大时，传统判别式模型很容易遇到瓶颈。尤其是推荐系统里有大量稀疏 ID 特征，用户、物品、场景都在变化，`参数一大就容易过拟合`，不像 LLM 那样“越大越强”的规律那么自然。
 
生成式推荐想做的事情更激进：能不能用一个模型直接理解用户行为序列，然后生成推荐结果？也就是说，把推荐系统==从“多阶段候选集打分”改造成“端到端序列生成”==
 
这听起来很像 LLM。语言模型给定上文生成下一个 token，`生成式推荐给定用户历史行为生成下一个 item`。一个续写文字，一个续写兴趣。
 
所以很多人说，推荐系统正在迎来自己的“ChatGPT 时刻”。
 
 
 
llm4rec 论文list
 
* OneRec Technical Report 
* OneRec-V2 Technical Report 
* OneRec: Unifying Retrieve and Rank with Generative Recommender and Preference Alignment 
* OneSearch: A Preliminary Exploration of the Unified End-to-End Generative Framework for E-commerce Search 
* QARM: Quantitative Alignment Multi-Modal Recommendation at Kuaishou 
* RankMixer: Scaling Up Ranking Models in Industrial Recommenders 
* Next-User Retrieval: Enhancing Cold-Start Recommendations via Generative Next-User Modeling 
* OneTrans: Unified Feature Interaction and Sequence Modeling with One Transformer in Industrial Recommender 
* MTGR: Industrial-Scale Generative Recommendation Framework in Meituan 
* EGA-V2: An End-to-end Generative Framework for Industrial Advertising 
* UniROM-One Model to Rank Them All: Unifying Online Advertising with End-to-End Learning 
* SortGen-A Generative Re-ranking Model for List-level Multi-objective Optimization at Taobao 
* RecGPT: A Foundation Model for Sequential Recommendation 
* COBRA-Sparse Meets Dense: Unified Generative Recommendations with Cascaded Sparse-Dense Representations 
* Towards Large-scale Generative Ranking
* Google TIGER-Recommender Systems with Generative Retrieval 
* Google Better Generalization with Semantic IDS: A Case Study in Ranking for Recommendations 
* Meta HSTU-Actions Speak Louder than Words: Trillion-Parameter Sequential Transducers for Generative Recommendations 
Snap GRID-Generative Recommendation with Semantic IDs: A Practitioner’s Handbook 
 
 