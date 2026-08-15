# 维度对标报告（五路，待审）

> 目标:定出一套受控的"维度"——理解或覆盖一个主题时反复出现的方面(区别于"领域"讲哪个技术、"体裁"用什么文档形式)。
> 方法:五路并行对标,四路查外部权威结构、一路归纳你自己的笔记实据。每路只报能指到真实来源的维度,查不到就说、不编。

## 一、五路来源

| 路 | 查什么 | 主要来源 |
|---|---|---|
| 矿场实据 | 你实际记笔记时反复出现的方面 | ~/code/notes 抽样 22 篇 + vault-inventory四份 |
| 技术制品族 | 语言/库/框架/数据库/协议/工具官方文档顶层结构 | PostgreSQL·MySQL·Rust Book·Go·React·Docker·MDN HTTP |
| 工程治理族 | 工程实践与治理的权威结构 | Google SRE Book·eng-practices·DORA·SPACE·Team Topologies·Nx |
| AI/agent 族 | AI 应用构建的一线活文档 | Anthropic 工程博客·OpenAI 文档·Chip Huyen《AI Engineering》·OWASP LLM Top10 |
| 写作/知识工程族 | 写作与知识工程的框架标准 | Diátaxis·Good Docs·Google 风格指南·ISO 25964·NISO Z39.19 |

一个方法学发现(技术制品路报的):**roadmap.sh 不是维度来源**——它切的是"该学哪些技术/概念"(领域清单),不是"一个技术拆成哪些方面",所以没拿它当维度依据。

## 二、领域分四族（对标归纳，不是拍脑袋）

四路外部 + 矿场都印证:主题按性质分四族,每族维度组不同。

| 族 | 对象 | 核心问的问题 | 印证 |
|---|---|---|---|
| **A 技术制品** | 一件可安装配置的东西(语言/库/框架/数据库/协议/工具) | 怎么装、怎么用、怎么调、出错怎么查 | 官方文档全按此组织;矿场"具体技术"类 |
| **B 工程实践与治理** | 一套多团队采纳的做法(治理/规模化/发布/可观测/平台工程) | 该不该做、按什么原则、谁来做、怎么衡量、怎么推广 | SRE/DORA/Team Topologies 均不按"安装"组织;矿场"架构+治理"类 |
| **C AI 应用构建** | 概率性、能力不确定的模型与 agent | 模型能做什么、怎么给上下文、怎么评估、成本与失败、注入安全 | Anthropic/OpenAI/Huyen;矿场"AI/agent"类 |
| **D 方法与规范** | 文档、词表、术语、知识组织本身 | 怎么算对、怎么分类、该长什么样、怎么保持一致 | Diátaxis/Good Docs/ISO;矿场"写作方法"类 |

矿场四类主题都找到实际笔记,没缺类,印证四族划分成立。

## 三、维度分两层：通用核心 + 族特有

五路交叉后最清楚的结论:有一批维度跨族都出现(名字通用,内涵按族定),再各族带一批本族特有的。

### 通用核心维度（跨族,几乎哪族都有）

| 维度 | 定义 | 哪几路印证 |
|---|---|---|
| 概念/心智模型 | 是什么、解决什么、核心模型 | 技术制品·AI·矿场(全库最密) |
| 机制/原理 | 内部怎么运转、为什么这样 | 技术制品(内部原理)·矿场 |
| 对比/选型/权衡 | 几个方案并列、讲差异与代价、给依据 | 五路全部(治理"核心权衡"、AI"模型选型"、矿场高频) |
| 决策准则/判断 | 岔路口按什么标准选;常写成判断树 | 矿场(高频、独立)·治理(弱) |
| 术语/界定 | 这个词指什么、和近义词怎么分 | 写作族·矿场(glossary) |
| 演进/版本 | 随时间怎么变;历史版本与分阶段长大 | 技术制品·矿场 |
| 生态/资源选材 | 有哪些工具资料、按什么门槛挑 | 矿场·技术制品(集成生态) |
| 度量/衡量 | 用什么指标、阈值、方法量它 | 三族(内涵不同,见下"张力") |
| 边界/职责·禁区/反模式 | 界线在哪、什么绝不能做 | 治理(反模式)·写作(边界)·矿场(高频) |
| 流程/维护治理 | 可反复执行的步骤与长期维持 | 治理·写作·矿场 |
| 排错/诊断 | 现象→原因→处置 | 技术制品·矿场(偏 A 族) |

**一个张力(待处理):同名维度跨族,内涵不同。** "度量"在技术制品是性能指标、在治理是 DORA/SPACE、在 AI 是 evals;"安全"在技术制品是认证加密、在 AI 是提示注入。处理办法二选一:(a) 维度名通用,内涵在各族全景里细化;(b) 拆成不同受控名(metrics-perf / metrics-dora / evals)。见待审第 3 点。

### 各族特有维度

- **A 技术制品**:安装/搭建、配置、用法操作、调优/性能、集成/接口、参考手册、内部原理/架构。
  - 语言子类:类型系统、内存/所有权、并发模型、错误处理、模块组织
  - 数据库子类:数据类型、事务与隔离、索引与查询优化、备份恢复、复制高可用、存储引擎、服务端编程
  - 协议子类:报文格式、方法与状态码、头部、缓存、连接管理、内容协商
  - 框架子类:组件/描述界面、状态管理、副作用/生命周期
- **B 工程治理**:目标与场景、组织与角色分工、可观测与反馈、治理审批合规、规模化考量、失效处理/事故响应、采用推广/落地。
- **C AI 构建**:模型能力边界、提示设计、**上下文治理(context engineering)**、检索/知识接入(RAG)、agent 构建编排、工具设计、评估(evals)、token 成本与延迟、失败模式与可靠性、注入与滥用安全、安全防护/人类把关。
  - 上下文治理细目(Anthropic):token 预算、上下文腐化、compaction、工具结果清理、结构化笔记/记忆、即时检索、渐进式披露、子 agent。
- **D 方法与规范**:判据/原则、区分轴与分类、结构与模板、元素关系(等同/层级/关联)、正例与反例、一致性/规范化、互操作/交换。

## 四、矿场补出的、外部框架漏掉的维度（重要）

外部四路都没有、但你笔记里高频,盘点评为"不可再生的个人视角":

- **换栈差异/心智模型迁移** —— "从我已知的东西迁过来,哪里不一样"(Go↔JS/TS 锚点、TS↔Python ORM 对照)。外部零覆盖。
- **决策准则/判断树** —— 你几乎每篇治理类笔记以"判断树/选型结论"收尾。外部治理族只到"核心权衡",没到这一步。
- **禁区/反模式/不变量** —— 你把它当独立高频维度(NestJS 禁区、索引不该建的、CSS @import 反模式)。
- **原则/公理** —— constitution/atomicity/MECE/SSoT 这类底层信条,推导其他规则。

这是本次对标最该记的一笔:**只对标外部权威,会系统性漏掉这四个你最珍视的维度。** 矿场那一路的价值就在把它们捞出来。

(另:你几乎每篇笔记以"判断清单/检查清单"收尾——这是你统一的**体例**,不是维度,不进值集。)

## 五、覆盖校验（用你的散点验证框架不漏）

你随口列的都能落位:

| 你说的 | 落哪族 | 主维度 |
|---|---|---|
| 技术栈(react/postgresql) | A | 概念·机制·用法·调优·排错·参考 |
| 超大型项目/治理工具 | B | 目标·权衡·角色分工·度量·规模化·采用推广 |
| 写作方面 | D | 判据·结构模板·正反例·术语·一致性 |
| 知识工程/分类术语 | D | 分类·元素关系·术语·互操作·维护 |
| AI 方面 | C | 模型边界·提示·评估·成本·注入安全 |
| agent 方面 | C | agent 编排·工具设计·失败模式 |
| AI 上下文治理 | C | 上下文治理(token预算/compaction/即时检索/子agent) |

全部有位置,框架不漏。

## 六、受控维度值集草案（可编进 subjectScheme）

分层:一个横切的通用核心维度集(所有族可用)+ 四个族特有维度子集。领域全景 topic 从中选出本领域适用的维度、声明为完整清单;每篇 topic 用 `@subjectrefs` 标覆盖的维度;覆盖度 = 全景声明 vs 实际有 topic,构建期可算。

- **通用核心**:concept · mechanism · comparison · decision · terminology · evolution · ecosystem · metrics · boundary-pitfalls · process · troubleshooting
- **个人特有(矿场,建议纳入通用层)**:mental-model-migration(换栈差异)· principles(原则公理)
- **A 技术制品特有**:install · config · usage · tuning · integration · reference · internals(+ 语言/数据库/协议/框架子类如上)
- **B 工程治理特有**:goals · roles · observability · governance · scaling · incident · adoption
- **C AI 构建特有**:model-capability · prompt · context-engineering · retrieval · agent-orchestration · tool-design · evaluation · cost-latency · failure-modes · injection-safety · human-oversight
- **D 方法规范特有**:criteria · classification · structure-template · relations · examples · consistency · interop

## 七、已定（2026-08-09，按推荐）

1. **四族划分**:采纳,五路印证,不改。
2. **两层结构**(通用核心 + 族特有):采纳。
3. **同名维度张力**:从通用层**移除"度量/安全/排错",按族走**——度量→治理 metrics;安全→技术制品 security + AI injection-safety;排错→技术制品 troubleshooting;评估→AI evaluation;可观测→治理 observability。通用层只留内涵稳定的。理由:同名不同实会让覆盖度统计和检索失真,拆开更准,代价只是名字多一点。
4. **矿场四个个人维度**:全纳入。换栈差异 `mental-model-migration`、原则 `principles` 作为一等通用维度显式登记;判断树并入 `decision`、禁区并入 `boundary-pitfalls`(已在通用层,不另立名)。换栈差异必须留——它是知识库带不带你自己视角的分水岭。
5. **受控维度名**:起步精简,只上通用核心 + 各族最强的几个;A 族语言/数据库/协议/框架子类按需再加,不一次铺全。见第八节。
6. **subjectScheme 放法**:采纳。一个横切 `dimension` 值集,内部按族分子集。
   - **标引机制(查 DITA 2.0 规范后定,几经修正)**:①先误用 topic 根 `@subjectrefs`——非法,它只在 map 层合法。②改 prolog `<data>`——能过 validate 但丢语义、不受控。③查证发现 **DITA 2.0 移除了 classification domain**(`topicsubject`/`subjectref` 等元素 2.0 不存在),受控标引正道是"`@props` 专门化属性 + `enumerationdef` 绑受控值"。**定案**:维度做专门化属性 `@dimension`(照 maturity/volatility/tool 加第四个属性域 `dimensionAttDomain.rng`),内容 topic 根标 `dimension="dim-…"`(可多值、`enumerationdef` 绑本值集校验);全景规划清单与域用 prolog `data`(`planned-dimension`/`domain`,属治理元数据、DITA 无对应分类概念)。覆盖度=planned vs 各 topic `@dimension`,按 domain 求差。已建属性域、改 shell、加 enumerationdef,多值与样板 `dita validate` 均通过。
   - **附带修正**:核对日期 `critdates` 无 `reviewed` 元素,改用 `data name="reviewed"`。

## 八、起步维度值集（定稿，待编入 subjectScheme 升 v0.3）

**通用核心(横切,所有族可用)**
`concept` 概念 · `mechanism` 机制原理 · `comparison` 对比选型 · `decision` 决策判断 · `terminology` 术语界定 · `evolution` 演进版本 · `ecosystem` 生态选材 · `boundary-pitfalls` 边界禁区反模式 · `process` 流程维护 · `principles` 原则公理 · `mental-model-migration` 换栈差异

**A 技术制品特有**
`install` 安装 · `config` 配置 · `usage` 用法 · `tuning` 调优 · `troubleshooting` 排错 · `security` 安全 · `integration` 集成 · `reference` 参考 · `internals` 内部原理
(语言/数据库/协议/框架子类:按需再加。已补:`dim-packaging` 打包分发/自动更新——桌面框架子类首个,electron 样板触发)

**B 工程治理特有**
`goals` 目标场景 · `roles` 角色分工 · `metrics` 度量指标 · `observability` 可观测 · `governance` 治理审批 · `scaling` 规模化 · `incident` 事故响应 · `adoption` 采用推广

**C AI 构建特有**
`model-capability` 模型边界 · `prompt` 提示设计 · `context-engineering` 上下文治理 · `retrieval` 检索RAG · `agent-orchestration` agent编排 · `tool-design` 工具设计 · `evaluation` 评估 · `cost-latency` 成本延迟 · `failure-modes` 失败模式 · `injection-safety` 注入安全

**D 方法规范特有**
`criteria` 判据 · `classification` 分类 · `structure-template` 结构模板 · `relations` 元素关系 · `examples` 正反例 · `consistency` 一致性 · `interop` 互操作

下一步:把本节编入 `kb/vocab/subjectScheme.ditamap`(新增横切 `dimension` 值集),词表升 v0.3,走轻量评审登记。
