# DITA 2.0 研究笔记

> **已冻结（2026-08-17）**：16 篇笔记的正本已全部迁入 `kb/`，本页与 `notes/` 冻结为调研档案，不再更新。本页原有的三块内容也已各有正本：
>
> | 本页原有的节 | 正本去向 |
> |---|---|
> | 版本现状（含两条现实、对二次开发者的好消息） | [`kb/topics/dita/dita-landscape.dita`](../kb/topics/dita/dita-landscape.dita)《DITA 领域全景》的「版本现状」一节 |
> | 30 秒上手 | [`kb/topics/dita/toolchain/dita-ot-quickstart.dita`](../kb/topics/dita/toolchain/dita-ot-quickstart.dita)《DITA-OT 安装与 2.0 doctype 触发》 |
> | 权威资源 | [`kb/topics/dita/dita-resources.dita`](../kb/topics/dita/dita-resources.dita)《DITA 权威资源入口》（按可信度分层收录） |
>
> 本页留下的只有两样：**调研过程信息**（笔记索引、阅读顺序、来源核对记录）与**尚未落地的待办**。过程信息按迁移设计裁定不进 kb——它记录的是「当时查了哪些页面、核对到什么程度」，属调研档案而非领域知识。
>
> 迁移全貌与溯源对照表见 [`cases/notes-to-kb-migration/`](cases/notes-to-kb-migration/README.md)。

纯 DITA 2.0，不含 1.3 的历史包袱和迁移内容。目标：**学懂标准 + 具备二次开发能力**（专门化 / DITA-OT 插件 / 程序化处理）。

---

## 版本现状

> **已迁 kb**：[`kb/topics/dita/dita-landscape.dita`](../kb/topics/dita/dita-landscape.dita) —— 版本状态、参考实现基线、两条必须先接受的现实（规范支持 ≠ 工具链支持；规范还会变）、`@class` 机制的稳定性，全部并入全景篇。该篇标 `volatility="volatile"`，时效由 `reviewed` 承载，比本页的静态快照可靠。
>
> 本页原表的版本事实核对记录保留在下方[「来源」](#来源)一节，作为当时的调研留痕。

---

## 笔记索引

16 篇全部已迁并冻结。每篇标题下有各自的「已迁移」声明，写明本篇哪些小节落到哪个 topic、哪些小节不迁及理由。

| 文件 | 内容 | 已迁 → 目标簇 |
|---|---|---|
| [00-roles-and-boundaries.md](notes/00-roles-and-boundaries.md) | **五种角色**（作者 / 信息架构师 / DITA 架构师 / 实现者 / 处理器）、规范写给谁看、各角色该读什么、**谁可以定义 schema**、角色权限边界 | ✅ `dita/principles/`（2 篇） |
| [01-core-model.md](notes/01-core-model.md) | topic 类型化与内容模型、task/steps 结构、map & bookmap、reltable、`<titlealt>` 标题体系、表格（simpletable vs CALS）、图像与多媒体、**`@class` 派生链**、`@specializations` | ✅ `dita/core-model/`（5 篇）＋ `dita/architecture/`（1 篇）＋ 全景 |
| [02-reuse.md](notes/02-reuse.md) | conref（拉）与 conref push（推）、keyref 与 `<keytext>` 变量文本、key 解析规则与 key scope、`<include>` 引用非 DITA 内容 | ✅ `dita/core-model/`（3 篇） |
| [03-profiling-and-chunking.md](notes/03-profiling-and-chunking.md) | 条件属性、DITAVAL 四种 action、分支过滤 `<ditavalref>`、subjectScheme 受控值、属性专门化维度、`@chunk` 的 combine/split | ✅ `dita/conditional/`（4 篇）＋ `dita/architecture/`（§5） |
| [04-toolchain-and-build.md](notes/04-toolchain-and-build.md) | DITA-OT 安装、**怎么让它按 2.0 处理文档**（doctype + catalog）、常用命令、**项目文件（`--project`）**、preprocess 流水线各阶段、校验与 Schematron、编辑器、PDF 现实 | ✅ `dita/toolchain/`（5 篇，含 §2 拆出的 `project-files`） |
| [05-specialization.md](notes/05-specialization.md) | 结构化 / 域 / 属性专门化、约束、泛化算法、`@class` 构造规则、好用的派生基类、RNG 路线 | ✅ `dita/architecture/`（5 篇） |
| [06-dita-ot-plugins.md](notes/06-dita-ot-plugins.md) | `plugin.xml` 与扩展点、XSLT 覆盖机制、处理 2.0 新元素、Ant 挂钩、自定义 transtype、调试工作流 | ✅ `dita/toolchain/`（4 篇） |
| [07-programmatic-processing.md](notes/07-programmatic-processing.md) | 解析前 vs preprocess 后、**如何取到 `@class` / `@specializations`**、按 class 编程、DITA-OT 当库用、生成 DITA、DITA↔Markdown | ✅ `dita/toolchain/`（5 篇） |
| [08-practical-advice.md](notes/08-practical-advice.md) | 踩坑速查表、上 2.0 的现实判断、选型判断、二次开发成本阶梯、工程化与 CI、学习路径、**覆盖情况与剩余缺口** | ✅ `dita/practice/`（4 篇）；学习路径与覆盖情况不迁 |

### 架构理论（09–11）

| 文件 | 内容 | 已迁 → 目标簇 |
|---|---|---|
| [09-architecture-foundations.md](notes/09-architecture-foundations.md) | **三大扩展设施**的正式框架（文档类型配置 / 专门化 / 元素类型配置）、**document-type shell 深入**、模块化与词汇模块、**约束 vs 扩展模块**、一致性（Conformance） | ✅ `dita/architecture/`（4 篇） |
| [10-addressing-and-key-space.md](notes/10-addressing-and-key-space.md) | 直接 vs 间接寻址、片段标识符两种语法、**键空间的正式模型**、`@keyscope` 嵌套与跨作用域、跨交付物寻址、**分支过滤与键空间的交互** | ✅ `dita/architecture/`（4 篇 ＋ §7 并入 `processing-checklist`） |
| [11-processing-model.md](notes/11-processing-model.md) | **属性有效值的五级优先级**、**元数据级联完整属性清单与 `@cascade`**、conref 属性合成规则与 `-dita-use-conref-target`、`<sort-as>` 与中文排序、程序化处理的顺序检查清单 | ✅ `dita/architecture/`（6 篇） |

> 09–11 讲规范定义的模型，01–08 讲用法。自研处理逻辑若不遵循 09–11 的规则，结果会与 DITA-OT 不一致，且不符合规范。
>
> 迁入后这层分工由 `kb/maps/domains/dita/architecture.ditamap` 的分组注释承载。

### 原理（12）

| 文件 | 内容 | 已迁 → 目标簇 |
|---|---|---|
| [12-philosophy-and-principles.md](notes/12-philosophy-and-principles.md) | **从第一性原理重构 DITA**：以"同一个事实只应存在于一处"为唯一公理，逐步推出 topic / map / key / 条件化 / `@class` / shell / 处理顺序；同样推出它的四项代价；区分"必然"与"历史包袱"；剥离 DITA 后可迁移的八条原则 | ✅ `dita/principles/`（3 篇） |

> 12 是论证，不是规范转述。前 11 篇说明 DITA 是什么样，12 说明它为什么是这样。迁入的三篇一律把推理与判断留在来源节的判断段，不写成规范断言。

### 翻译与本地化（13）

| 文件 | 内容 | 已迁 → 目标簇 |
|---|---|---|
| [13-translation-and-localization.md](notes/13-translation-and-localization.md) | 模块化降翻译成本的机制、`@xml:lang` / `@dir` / `@translate`（含 **topicref 上的 `@xml:lang` 不作用于被引 topic**）、翻译流程与 XLIFF 的工具层定位、**复用机制在翻译下的两个反模式**（用变量拼句子、低于句子粒度的 conref） | ✅ `dita/practice/`（3 篇） |

### 最佳实践（14 起）

| 文件 | 内容 | 已迁 → 目标簇 |
|---|---|---|
| [14-metadata-and-classification.md](notes/14-metadata-and-classification.md) | 过滤用与分类用的元数据之分、**Dublin Core ↔ DITA 对应表**、元数据五种放置机制的取舍、**subjectScheme 作为分类法机制**（沿层级向上查找、空枚举禁用属性、defaultSubject）、**分类树按过滤语义画**的设计规则、字段设计流程 | ✅ `dita/conditional/`（4 篇）；§6 的 RAG 注记并入 `dita/practice/` |
| [15-dita-and-rag.md](notes/15-dita-and-rag.md) | 论证篇：topic 自足性与检索切块的定义性吻合、**必须用解析后内容**（条件过滤是正确性问题）、按变体分库 vs 单库加检索时过滤（passthrough）、检索元数据对照表、**反面清单**（解析摧毁复用标识、不是上 DITA 的理由） | ✅ `dita/practice/`（3 篇） |

> 14 起为最佳实践层：依据是社区经验 + 规范核对的混合，来源档区分"规范条文"与"观点来源"。这一分档在迁入后由每篇的「来源」节（事实 / 判断两段）承载。

## 建议的阅读顺序

> **缓建项**（迁移设计 §八）：五条角色路径的正本形态是 **audience map**（按角色编排 topicref 的 map），不是一张表。内容已全部存在于 kb，建 map 是独立小项，未做。下表暂留，路径中的编号指本页笔记，对应的 kb topic 见上方索引表。

先读 [00-角色与边界](notes/00-roles-and-boundaries.md) 确认自己是哪个角色，再按下面选路径。

| 角色 / 目的 | 路径 |
|---|---|
| **作者**（写内容） | 00 → 01 → 02 → 03 |
| **信息架构师** | 00 → 01 → 02 → 03 → 10 → 13 → 14 → 08 |
| **DITA 架构师**（定义词汇与外壳） | 00 → 12 → 09 → 05 → 08 |
| **实现者**（自己写处理逻辑） | 00 → 12 → 09 → 10 → 11 → 07 |
| **工具链维护**（DITA-OT 插件） | 00 → 04 → 06 |
| **从零通读** | 00 → 12 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → 10 → 11 → 13 → 14 → 15 |
| **评估要不要用 DITA** | 只读 12 的第三节（代价）与第四节（必然 vs 包袱） |

---

## 30 秒上手

> **已迁 kb**：[`kb/topics/dita/toolchain/dita-ot-quickstart.dita`](../kb/topics/dita/toolchain/dita-ot-quickstart.dita)

---

## DITA 2.0 的四个招牌特性

> **已迁 kb，不单独成篇**（2026-08-16 裁定）。「这四个最能验证工具链是否真的支持」这句框架判断并入 [`kb/topics/dita/practice/adoption-criteria.dita`](../kb/topics/dita/practice/adoption-criteria.dita)《DITA 采用判据》的最小验证段；四个特性各自的正文归各自的 topic：
>
> | 特性 | 正本 |
> |---|---|
> | `<keytext>` | [`dita/core-model/keyref-variable-text.dita`](../kb/topics/dita/core-model/keyref-variable-text.dita)《keyref 与 keytext 速查》 |
> | `<titlealt>` + `@title-role` | [`dita/core-model/titlealt-system.dita`](../kb/topics/dita/core-model/titlealt-system.dita)《titlealt 速查》 |
> | `<include>` | [`dita/core-model/include-non-dita.dita`](../kb/topics/dita/core-model/include-non-dita.dita)《include 元素》 |
> | `@chunk="combine\|split"` | [`dita/conditional/chunking.dita`](../kb/topics/dita/conditional/chunking.dita)《chunking 速查》 |
>
> 不单独成篇的理由：这是一份学习路径建议而非领域节点，四个机制在上游各有自己的节点，聚成一篇会与四篇重复。

---

## 权威资源

> **已迁 kb**：[`kb/topics/dita/dita-resources.dita`](../kb/topics/dita/dita-resources.dita)《DITA 权威资源入口》。迁入版按可信度分层收录并给出取用规则（规范一侧 vs 工具链一侧的可信度不同），不是一张平铺的链接表；链接可达性由 `just links` 持续校验。
>
> 本页原清单的调研留痕保留在下方「来源」一节。

---

## 来源

每篇笔记末尾都有独立的「来源」小节，区分**已逐页核对**与**来自通用实践**两类。本页的版本事实来自：

- [DITA 规范列表](https://www.dita-lang.org/specifications) — DITA 2.0 列为 draft；1.3/1.2/1.1/1.0 为 OASIS Standard
- [oasis-tcs/dita 发布页](https://github.com/oasis-tcs/dita/releases) — **v2.0-beta03（2026-07-02）**、beta02（2024-10-04）、beta01（2024-06-14），均为 pre-release，未标注 OASIS 正式阶段
- [DITA-OT 发布说明](https://www.dita-ot.org/dev/release-notes/) — **DITA-OT 4.4，2026-01-31，要求 Java 17+**
- [DITA 2.0 preview 支持](https://www.dita-ot.org/dev/reference/dita-v2-0-support.html) — DITA-OT 4.4 基于 **2026-01-25** 的 2.0 草案语法文件（"基线比 beta03 旧约 5 个月"的依据）
- [DTD 公共标识符](https://dita-lang.org/dita/non-normative/dtd-public-identifiers) — DOCTYPE 公共标识符格式与版本号规则
- [OASIS DITA TC](https://www.oasis-open.org/committees/tc_home.php?wg_abbrev=dita) — 1.3 及以前各版本的正式批准日期
- 四个招牌特性分别核对自 [keytext](https://dita-lang.org/dita/langref/base/keytext)、[titlealt](https://dita-lang.org/dita/langref/base/titlealt)、[include](https://dita-lang.org/dita/langref/base/include)、[chunk](https://dita-lang.org/dita/archspec/base/chunk-attribute-overview)

**未核对**：`dita init` 的模板名称（故正文写作 `--template=<模板名>`，请先跑 `dita init --list`）。

**覆盖情况**：01–08 为操作层，09–11 为架构理论层（shell / 扩展模块 / 一致性 / 键空间 / 分支过滤交互 / 属性有效值 / 元数据级联 / TOC / 索引 / 链接生成 / 排序），12 为原理，13 为翻译与本地化，14 起为最佳实践层（14 元数据与分类策略、15 DITA 与 RAG）。**剩余缺口**（无障碍——可选仅记录、大规模工程、生态）与**验证规划**的完整清单见 [08-practical-advice.md](notes/08-practical-advice.md)。

> 冻结后这套覆盖判断的接班机制是 `just ia` 的维度覆盖求差——它按各 topic 声明的 `@dimension` 与全景篇的 `planned-dimension` 求差，算出盲区，不需要人工维护清单。dita 域现为 23/23（满格）。

## 调研待办

迁移不覆盖的，仍留在本页：

- **CCMS 预览对标**（Paligo / Heretto / FontoXML）——为 `dita-tools preview` 的设计做参照（能力地图见 [docs/architecture.md](../docs/architecture.md) §四）。
- **conref 篇补核对**——规范「内容引用处理」章与「处理模型」章尚未逐页核对，`kb/topics/dita/core-model/conref-pull-push.dita` 晋 `curated` 前需补（迁移设计 §八缓建项）。
- **角色阅读路径 map**——见上方「建议的阅读顺序」的缓建说明。
- **dita 分支 reltable**（互链关系表）——各簇已迁完，可统一梳理（迁移设计 §八缓建项）。
