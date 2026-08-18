# 上游依据对照表（notes → kb 迁移，2026-08-16/17）

> 定位：**上游节点索引方案的回填素材**。每一行记录一篇迁移产出的 slug、定稿标题、标题所依据的上游节点（或「本库自造」及其三道关摘要）、以及产出它的迁移任务。
>
> 为什么单独留档：这些对照表原本散在 SDD workspace（`.superpowers/sdd/2026-08-16-notes-to-kb-migration/`）的各任务报告里，那个目录是临时工作区，收尾后删除。设计稿 [`docs/superpowers/specs/2026-08-16-upstream-node-index-design.md`](../../../docs/superpowers/specs/2026-08-16-upstream-node-index-design.md) 第七节明确：**不抽出来，回填要重新查 65 遍**。抽取动作即该设计的 T3。
>
> 回填怎么用：按「上游依据」列写 `<data name="upstream-node" value="…"/>`，值为上游节点标题原文（英文逐字）；本表标「本库自造」的写 `value="coined"`（三道关说明已在各篇文件头注释里，无需重写）。组合篇声明多条。
>
> ⚠️ **本表是抄本，不是正本**。正本是各 `.dita` 文件的头注释（进版本控制，随内容一起演化）。本表冻结于抽取时刻，只为回填与复核提供一次性索引；两者不符时以文件头注释为准。

---

## 抽取方法与可信度

- **主源**：122 篇 topic 的文件头 XML 注释（`kb/topics/**/*.dita`）。头注释是当时各执行 agent 按纪律逐篇写下的依据，是全库唯一覆盖完整的溯源载体。
- **旁证**：workspace 内尚存的 12 份任务报告（task-1/2/3/4/5/6/6.5/7/10b/13/13b/13c）。Task 8/9/10a/11/12 与改名（retitle）任务在隔离 worktree 内执行，报告未拷回主仓，**其内容已随 worktree 清理而不存在**——这几个簇的溯源仅存于文件头注释，本表即其唯一二手抄本。
- **一处已知缺口**：`generate-and-convert` 的头注释引用了「见 retitle-report 的边界建议」，该报告全盘检索（`fd -HI retitle /`）已不存在。其边界建议的内容未被抄出，只知结论是「建议后续把生成一侧单独成篇」。

**证据等级**（与本库来源纪律同一口径）：

| 等级 | 含义 | 本表标记 |
|---|---|---|
| 逐页核对 | 上游节点名经 WebFetch 打开实页或本机 docsrc 全量抽取后逐个匹配 | 依据列写出节点全名与路径 |
| 归纳 | 依据来自笔记与既往调研，未在本轮重新打开上游页面 | 依据列只写节点名，无路径 |
| 自造 | 上游确无对应节点，走三道关 | 「本库自造」＋三道关摘要 |

**一条已登记的系统性弱点**（Task 13b 疑虑 3、progress.md 索引边界裁定）：`principles` 与 `practice` 两簇 15 篇的「穷尽查证」中，DITA-OT 一侧是本机 `~/ws/tools/dita-ot-4.4/docsrc` 全量 `<title>` 抽取后实查；**OASIS 一侧无本地克隆，以各篇来源节已逐页核对过的 archSpec / langRef 页面为证据基础，不是对规范全树的遍历**。这正是上游节点索引要解决的问题。按 `rot-detection` 的边界二，这批自造声明的复核**不得由原执行者做**。

---

## 一、分支根（Task 4）

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `dita-landscape` | DITA 领域全景 | **本库自造**。上游无「领域全景」这一体裁（OASIS 规范与 DITA-OT 文档均无）；「领域全景」是本库 tech-landscape 题材的固定后缀，DITA 为领域名，未造新词。迁自 `research/README.md`「版本现状」节 + 笔记 01 §6。 | Task 4 |
| `dita-resources` | DITA 权威资源入口 | **本库自造（半）**。上游有同类目节点：DITA-OT 4.4 文档 `topics/dita-resources.ditamap`「**DITA and DITA-OT resources**」；本篇按可信度分层收录，故取「权威资源入口」而非直译。迁自 `research/README.md`「权威资源」节。 | Task 4 |

> **改名注记（2026-08-17）**：`tech-landscape` 题材的中文后缀由「领域全景」改为「**领域概览**」，三篇标题随之改为《DITA 领域概览》《Electron 领域概览》《Coding Agents 领域概览》。上表 `dita-landscape` 一行保留改名前的原文——它记录的是迁移当时的定稿标题，且其自造论证本身就是针对「全景」这个词的，就地替换会让论证与被论证对象脱节。文件名、`id` 与题材键 `tech-landscape` 均未变，回填时按新标题写正文、按本行读溯源。

---

## 二、core-model 簇（Task 5 三篇 + Task 11 五篇）

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `conref-pull-push` | conref 与 conref push | **非自造，两个机制名并列**。OASIS DITA 2.0 的 conref 与 conref push 机制名；DITA-OT 4.4 `reference/preprocess-conref`「**Resolve content references (conref)**」与 `reference/preprocess-conrefpush`「**Conref push (conrefpush)**」。迁自笔记 02 §1＋§4。 | Task 5 |
| `keyref-variable-text` | keyref 与 keytext 速查 | **非自造**。OASIS DITA 2.0 的 `@keyref` 属性与 `<keytext>` 元素；「速查」为本库 cheatsheet 题材固定后缀。迁自笔记 02 §2。 | Task 5 |
| `include-non-dita` | include 元素 | **非自造**。OASIS DITA 2.0 语言参考元素页 `<include>`；非 DITA 内容一侧对应架构规范「**Specializing to include non-DITA content**」，归 shortdesc 交代。迁自笔记 02 §3。 | Task 5 |
| `topic-typing` | topic 与 information typing | **非自造，两个子节点并列**。架构规范章节「DITA topics」（`archspec/base/topicover`）下六个子节点中的「**Topic structure**」与「**Information typing**」。「A 与 B」形制沿用 conref-pull-push。迁自笔记 01 §1＋§1.5。 | Task 11 |
| `map-structure` | DITA map | **非自造，直取章名**。架构规范章节「**DITA maps**」（`archspec/base/dita-maps`），其子节点「DITA maps and their usage」的分节与本篇覆盖范围逐条对应。迁自笔记 01 §2。 | Task 11 |
| `titlealt-system` | titlealt 速查 | **非自造**。OASIS DITA 2.0 语言参考元素 `<titlealt>`（`langref/base/titlealt`，父节点 Basic topic elements）；「速查」为 cheatsheet 后缀惯例。迁自笔记 01 §3。 | Task 11 |
| `table-model-choice` | simpletable 与 CALS table 的选用 | **本库组合（非新造概念）**。上游有容器节点「**Table elements**」（`langref/containers/table-elements`，称 DITA 提供 complex table 与 simple table 两种），两元素各有语言参考页 `<table>` 与 `<simpletable>`；`<table>` 页明确其模型为 **OASIS Exchange Table Model**（社区通行叫法 CALS）。上游无「在两者之间选型」节点，标题以既有词素组合：simpletable ＋ CALS table ＋ 领域词「选用」。迁自笔记 01 §3.5。 | Task 11 |
| `images-multimedia` | image 与 multimedia elements 速查 | **非自造，两个节点并列**。`<image>` 语言参考页（`langref/base/image`，父节点 Body elements）与容器节点「**Multimedia elements**」（`langref/containers/multimedia-elements`，含 audio / video / media-source / media-track / video-poster 五个元素页）。上游无合并节点，故并列；「速查」为 cheatsheet 后缀。迁自笔记 01 §3.6。 | Task 11 |

---

## 三、architecture 簇（Task 9 五篇 + Task 10a 八篇 + Task 10b 六篇 + 归位一篇）

### 3.1 specialization 主题（Task 9）

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `structural-specialization` | structural specialization | **非自造，直取子树标题**。架构规范「**Structural specialization**」。迁自笔记 05 §1。 | Task 9 |
| `domain-specialization` | domain specialization | **非自造，直取子树标题**。架构规范「**Domain specialization**」。迁自笔记 05 §2。 | Task 9 |
| `attribute-specialization` | attribute specialization | **非自造，直取子树标题**。架构规范「**Attribute specialization**」。迁自笔记 05 §3 ＋ 笔记 03 §5，两处合并。 | Task 9 |
| `constraints-generalization` | constraints 与 generalization | **非自造，两个子树标题并列**。架构规范「**Constraints**」与「**Generalization**」。裁定不拆：两类目内容互为依赖（收紧 vs 还原），拆开得两篇薄篇。迁自笔记 05 §4＋§5。 | Task 9 |
| `specialization-practice` | specialization 的启用判据 | **本库自造（组合）**。上游无「何时启用 specialization」这一节点。「判据」非自造词——本库既有（writing-atomicity 内容准入判据、adoption-criteria DITA 采用判据），属「只组合不发明」。迁自笔记 05 §核心思想＋§6。 | Task 9（三道关补于 Task 13b） |

### 3.2 架构基础与寻址（Task 10a）

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `extension-facilities` | DITA extension facilities | **非自造，直取类目名**。架构规范「Configuration and specialization」子树下的「**Overview of DITA extension facilities**」（原文 "DITA provides three extension facilities"）；导航性前缀 Overview of 不入标题。迁自笔记 09 §1＋§4＋§6（§4 按第 2 关「先怀疑切分」从 vocabulary-modules 重切入本篇）。 | Task 10a |
| `vocabulary-modules` | vocabulary modules | **非自造，直取节点名**。架构规范「Specialization」下的「**Vocabulary modules**」（原文 "A DITA element type or attribute is declared in exactly one vocabulary module"）。§4 移出后本篇才对齐上游单节点，不再是拼盘。迁自笔记 09 §3。 | Task 10a |
| `doctype-shell` | document-type shell | **非自造，直取节点名**。架构规范「Document-type configuration」下的「**Overview of document-type shells**」，术语章条目名为 **DITA document-type shell**；Overview of 前缀与 DITA 前缀按上游正文用法省去。迁自笔记 09 §2。 | Task 10a |
| `conformance` | conformance | **非自造，直取章名**。规范第 10 章「**Conformance**」（两子节 Conformance of DITA implementations / Conformance of DITA documents），本篇覆盖该章整体。注：cheatsheet 的「速查」后缀惯例让位于标题规则。迁自笔记 09 §5。 | Task 10a |
| `addressing-modes` | DITA addressing | **非自造，直取章名**。架构规范「**DITA addressing**」章（子页 id attribute / DITA linking / URI-based (direct) addressing / Indirect key-based addressing / Context hooks for user assistance）。本篇覆盖该章总述与除 key 子树外的全部子页。迁自笔记 10 §1＋§2。 | Task 10a |
| `key-space-model` | key space 与 key scope | **非自造，两个上游词并列**。**key space** 是架构规范术语（分支过滤页原文 "the global key space for a root map"、"construct the key space"）；**key scope** 是「Indirect key-based addressing」下「**Scoping keys with the keyscope attribute**」子页的主题（DITA 1.3 同一节点名为 **Key scopes**）。并列形制先例：constraints 与 generalization。迁自笔记 10 §3＋§4。 | Task 10a |
| `cross-deliverable-addressing` | cross-deliverable addressing | **非自造，直取子页名**。架构规范「Indirect key-based addressing」下的「**Cross-deliverable addressing and linking**」；尾部 and linking 是同一机制的用途并列，按「不拼装」不入标题。迁自笔记 10 §5。 | Task 10a |
| `branch-filter-key-space` | branch filtering 与 key space | **非自造，两个上游词组合**。架构规范「**Branch filtering**」章，其子页含「Branch filtering: Impact on resource and key names」与「Branch filtering: Implications of processing order」，两者共同主题即分支过滤与 key space 的交互；key space 为规范术语。迁自笔记 10 §6。 | Task 10a |

### 3.3 处理模型（Task 10b）

> 本组上游节点名**逐个用 WebFetch 核对 dita-lang.org 2.0 archspec 实页（2026-08-16）**，非凭记忆。`DITA processing` 章九个子节点已确认为：Navigation / Indexes / Content reference (conref) / Conditional processing / Metadata cascading / Chunking / Branch filtering / Sorting / Determining effective attribute values。

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `effective-attribute-values` | effective attribute values 的确定 | **非自造**。「DITA processing」章子节点「**Determining effective attribute values**」（`archspec/base/determining-effective-attribute-values`）。动名词 Determining 不入标题（标题规则禁动词短语），名词化为「…的确定」；先例：extension-facilities 舍去 Overview of。迁自笔记 11 §1。 | Task 10b |
| `metadata-cascade` | metadata cascading | **非自造，直用类目名**。「DITA processing」章子节点「**Metadata cascading**」（`archspec/base/map-cascading`），四子页已核（Cascading of metadata attributes in a DITA map / Reconciling topic and map metadata elements / Map-to-map cascading behaviors / Examples of metadata cascading）。迁自笔记 11 §2。 | Task 10b |
| `conref-attribute-rules` | conref 解析时的属性处理 | **非自造**。「Content reference (conref)」章下子节点「**Processing attributes when resolving conrefs**」（`archspec/base/conref-attributes-specified-on-elements`）。动名词 Processing 名词化，机制名 conref 保留英文。迁自笔记 11 §3。 | Task 10b |
| `sorting-sort-as` | sorting 与 sort-as | **非自造，两个上游词并列**。「DITA processing」章子节点「**Sorting**」（`archspec/base/sort-as-processing`，无子页）＋语言参考元素 `<sort-as>`。单以 Sorting 取不到本篇另一半（那个元素本身），按同簇并列形制以「与」连接。迁自笔记 11 §4。 | Task 10b |
| `nav-generation` | navigation、indexes 与 generated links | **本库自造（组合式，三个上游类目）**。①穷尽查证：archspec 与本篇对应的是三处而非一处——「**Navigation**」（子页仅 Table of contents 与 Alternative titles）、「**Indexes**」，以及散在 topicref / reltable 页的 `@collection-type` / `@linking` / reltable 链接规则；DITA-OT 4.4 docsrc 全部节点标题亦无上位节点（最近的 `reference/preprocess-maplink`「Map-based linking (maplink)」只讲一个 preprocess 阶段）。②先怀疑切分：篇边界由迁移计划固定为 §5–5.9，重切须动整簇篇目，故不重切，改为标题里如实列出三个上游类目。③只组合不发明：navigation / indexes 直取上游节点名，**generated links** 为 DITA-OT 文档既有说法（`release-notes/rel3.0`「generated links」、`migrating-to-1.8`「Link generation」）。迁自笔记 11 §5＋§5.5＋§5.7＋§5.9。 | Task 10b |
| `processing-checklist` | DITA processing 顺序速查 | **本库自造**。①穷尽查证：archspec 只逐项规定各处理主题，不给主题之间的执行次序，无「处理顺序」节点；DITA-OT 侧最近的 `reference/preprocess`「Pre-processing modules」讲的是某一实现的阶段划分，非规范要求的次序。②先怀疑切分：本篇边界即「规范约束下自研处理逻辑的执行次序」，是规范未设节点而实现方必须面对的空缺，不是切分错位。③只组合不发明：**DITA processing** 直取上游章名，「顺序」「速查」均为既有词素。迁自笔记 11 §6 ＋ 笔记 10 §7。 | Task 10b |

### 3.4 归位篇

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `class-derivation` | @class 与 @specializations 属性规则与语法 | **非自造，两个节点并列**。架构规范「**The @class attribute rules and syntax**」（`archspec/base/specialization-class-attribute`）与同章「**The specializations attribute rules and syntax**」（`archspec/base/specialization-specializations-attribute`）。两节点共用「attribute rules and syntax」词尾，并列后共用一次词尾。迁自笔记 01 §4＋§5。**2026-08-16 由 core-model 归位 architecture**（两属性规则出自「Configuration and specialization」章，与 specialization 三篇同一节点域）。 | 建于 Task 11，归位于 Task 10b |

---

## 四、conditional 簇（Task 8，7 篇）

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `profiling-ditaval` | conditional processing | **非自造，直取类目名**。架构规范「DITA processing」章下的「**Conditional processing**」一节（子页含 About the DITAVAL document / Conditional processing attribute values / Filtering based on metadata attributes / Flagging based on metadata attributes）。迁自笔记 03 §1＋§2。 | Task 8 |
| `branch-filtering` | branch filtering | **非自造，直取类目名**。架构规范「DITA processing」章下与 Conditional processing、Chunking 并列的「**Branch filtering**」；DITA-OT 对应节点为「Branch filtering: re-using profiled content」与流水线阶段 `branch-filter`。迁自笔记 03 §3（事实段取自笔记 10 §6 已逐页核对的分支过滤章）。 | Task 8 |
| `chunking` | chunking 速查 | **非自造，直取类目名**。架构规范「DITA processing」章下的「**Chunking**」（子页 About the chunk attribute / Processing chunk="combine" / Processing chunk="split"）；DITA-OT 对应节点亦为 **Chunking**。「速查」为 cheatsheet 后缀。迁自笔记 03 §6。 | Task 8 |
| `subjectscheme-taxonomy` | subject scheme map | **非自造，直取子页名**。架构规范章节页「Subject scheme maps and their usage」的首个子页即「**Subject scheme maps**」；DITA-OT 对应节点为「**Subject schemes**」。迁自笔记 03 §4 ＋ 笔记 14 §4（同一节点的两种用法，合并成篇）。 | Task 8 |
| `metadata-two-kinds` | 过滤用元数据与分类用元数据 | **本库自造**。①穷尽查证：archspec 有 DITA metadata 章（子页 Metadata elements / Metadata attributes / Metadata in maps and topics / Window metadata for user assistance）与 Conditional processing 章，前者讲描述性元数据的语法位置、后者讲条件属性，**均无「同一维度该按哪类用途设计」这个节点**；DITA-OT 侧只有流水线阶段名（move-meta-entries 等）；社区侧 Scriptorium 谈流程建议，未给节点名。②先怀疑切分：上游按语法形态切（元素 / 属性），本篇按用途切；按上游重切会让「这个维度给谁消费」的判断散到两个语法节点里无处安放。③只组合不发明：「过滤」取自 Filtering 节点、「分类」取自 subject scheme 一侧的分类语义、「元数据」取自 DITA metadata 节点。迁自笔记 14 §1＋§5。 | Task 8 |
| `metadata-placement` | DITA 元数据的放置位置 | **本库自造**。①穷尽查证：archspec 的 DITA metadata 章下有 Metadata elements / Metadata attributes / Metadata in maps and topics 三子页，切法是语法形态与文件归属，其中 Metadata in maps and topics 讲 map 侧与 topic 侧的分工与覆盖，**不是「同一个维度该落在哪个机制上」**；DITA-OT 侧只有流水线阶段；社区无通行节点名。②先怀疑切分：本篇按机制能力切（能否校验 / 能否过滤 / 是否级联 / 默认渲染与否）；按上游语法形态重切会把同一个放置决定拆到元素与属性两个节点里。③只组合不发明：「DITA 元数据」取自 DITA metadata 节点，「放置位置」为普通词。迁自笔记 14 §3。 | Task 8 |
| `dublin-core-mapping` | Dublin Core 与 DITA 元数据对应速查 | **本库自造**。①穷尽查证：archspec 的 DITA metadata 章及其四个子页、langref 的 author 页（笔记 14 已逐页核对，明确未提及 Dublin Core）均无 DITA↔Dublin Core 对应节点；DITA-OT 只在发行说明有「Dublin Core metadata removed from HTML5」一条，讲的是 HTML 产物里的 meta 标签而非字段对应；社区流行的「DITA 元数据仿照 Dublin Core 建模」在 2.0 规范原文里**找不到背书**。②先怀疑切分：本篇边界即两套字段集之间的映射，重切只能并回元数据设计一篇，而那篇是流程、本篇是查表用的对照。③只组合不发明：**Dublin Core** 为 DCMI 标准名，「DITA 元数据」取自 DITA metadata 节点，「对应」为普通词，「速查」依 cheatsheet 惯例。迁自笔记 14 §2。 | Task 8 |

---

## 五、toolchain 簇（Task 6 十三篇 + Task 13b 一篇）

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `dita-ot-quickstart` | DITA-OT 安装与 2.0 doctype 触发 | **本库自造（聚合）**。标题由两个上游节点合成——DITA-OT 4.4 安装一组（`topics/installing-client`、`topics/installing-via-homebrew` 等，类目名「**Installing DITA-OT**」）与 `reference/dita-v2-0-support`「**DITA 2.0 preview support**」；上游无合并二者的节点，合并出自本库 quickstart 题材。迁自笔记 04 §1＋§2，并吸收 `research/README.md`「30 秒上手」节。 | Task 6 |
| `preprocess-pipeline` | preprocess 流水线 | **非自造**。DITA-OT 4.4 `reference/preprocessing`「**Pre-processing modules**」；「流水线」取自同套文档 `topics/plugin-antpreprocess`「Adding an Ant target to the pre-processing pipeline」。迁自笔记 04 §3。 | Task 6 |
| `validation-schematron` | 校验与 Schematron | **本库自造（上游无「校验」总节点）**。已查 OASIS DITA 2.0 规范、DITA-OT 4.4 docsrc 全部节点标题（只有 `dita validate` 子命令与各 transtype 处理节点）、**ISO Schematron**（独立标准，非 DITA 节点）——四种校验的合并视角为本篇自己的组织。标题由领域既有词素「校验」＋上游标准名 **Schematron** 组合；DITA 前缀按上游用语省去（子命令即 `dita validate`，无「DITA 校验」页）。迁自笔记 04 §4。 | Task 6 |
| `editors-pdf-reality` | DITA 编辑器与 PDF 输出选型 | **本库自造**。OASIS 规范不涉工具；DITA-OT 4.4 docsrc 只到 `topics/pdf-customization-approaches`「**PDF customization approaches**」，无编辑器选型节点；社区有「**DITA editors**」这一通行类目，故保留 DITA 前缀。迁自笔记 04 §5＋§6。 | Task 6 |
| `plugin-extension-points` | DITA-OT 扩展点 | **非自造**。DITA-OT 4.4 `extension-points/extension-points.ditamap`「**DITA-OT extension points**」。plugin.xml 一节对应 `topics/plugin-configfile`「Plug-in descriptor file」、安装一节对应 `topics/plugins-installing`「Installing plug-ins」，两者并入本篇故不进标题。迁自笔记 06。 | Task 6 |
| `xslt-override` | XSLT 覆盖 | **非自造**。DITA-OT 4.4 `topics/plugin-overridestyle`「**Overriding an XSLT-processing step**」；另见 `extension-points/plugin-extension-points-xslt-import`「XSLT-import extension points」。迁自笔记 06。 | Task 6 |
| `custom-transtype` | 自定义 transtype | **非自造**。DITA-OT 4.4 `topics/plugin-newtranstype`「**Adding a new transformation type**」。迁自笔记 06。 | Task 6 |
| `plugin-debugging` | plugin 调试 | **本库自造**。已查 OASIS 规范（无工具链节点）、DITA-OT 4.4 docsrc 全部节点标题、社区通行说法，最接近的是**三个分立节点**：`topics/enabling-debug-mode`「Enabling debug mode」、`topics/plugins-installing`「Installing plug-ins」、`topics/troubleshooting-overview`「Error messages and troubleshooting」，无合并节点。迁自笔记 06。 | Task 6 |
| `parsed-vs-source` | 程序化处理的输入选择 | **本库自造**。已查 OASIS DITA 2.0 规范、DITA-OT 4.4 docsrc 全部节点标题（最近的 `reference/preprocessing`「Pre-processing modules」只讲阶段本身，不讲消费方该取哪一份输入）、社区通行说法，均无「输入选择」这个节点。迁自笔记 07 §1。 | Task 6 |
| `programming-by-class` | @class 与派生链解析 | **非自造，两个节点**。架构规范「**The class attribute rules and syntax**」与「**The specializations attribute rules and syntax**」（同属 Specialization 章）。迁自笔记 07 §2＋§3。 | Task 6 |
| `dita-ot-as-library` | DITA-OT Java API | **非自造**。DITA-OT 4.4 `reference/java-api`「**Using the Java API**」；`.job.xml` 与输出过滤两节归 shortdesc 交代。迁自笔记 07 §4＋§7。 | Task 6 |
| `generate-and-convert` | DITA 生成与 Markdown 互转 | **本库自造（半）**。Markdown 一侧有上游节点：DITA-OT 4.4 `topics/markdown-input`「**Markdown input**」与 `topics/dita2markdown`「**Generating Markdown output**」；**生成一侧**（从结构化数据生成 topic）在 OASIS 规范与 DITA-OT docsrc 中均无对应节点，社区亦无通行名。⚠️ 本篇跨两个节点，**边界待复核，建议后续把生成一侧单独成篇**。迁自笔记 07 §5＋§6。 | Task 6 |
| `processing-tools` | 程序化处理工具速查 | **本库自造**。表内工具（Saxon-HE、Jing、xmllint、lxml、org.lwdita）分属各自项目，OASIS 规范与 DITA-OT 4.4 docsrc 均无跨项目的工具清单节点，社区亦无通行名。迁自笔记 07 §8。 | Task 6 |
| `project-files` | project file 与 dita --project | **非自造，取自上游节点**。DITA-OT 4.4 docsrc「**Publishing with project files**」（`topics/using-project-files.dita`，挂 `publishing.ditamap`，带三个 Sample … project files 子节点）。**2026-08-16 切分复议拆出**：内容原埋在 `practice/engineering-ci` 做法节里，而该篇标题不含任何可检索到它的名字；且它是笔记 04 §2 中唯一逐页核对过的事实，档位与 engineering-ci 其余（全属判断）不同。顺带纠出自造译名「项目文件」→ **project file**。 | Task 13b（源出笔记 04 §2，经 Task 6→Task 7 转手） |

---

## 六、practice 簇（Task 7，10 篇）

> 本簇 10 篇的自造依据头注释由 Task 13b 补齐（Task 7 的改名 pass 曾按指令跳过 `practice/`）。

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `pitfalls` | DITA 2.0 故障速查 | **本库自造**。①穷尽查证：OASIS DITA 2.0 规范无故障或排错类节点，最近的是「**Information about migrating to DITA 2.0**」（讲迁移改动，不讲症状）；DITA-OT 4.4 有「**Error messages and troubleshooting**」，但讲的是工具自身的报错编号，不覆盖语言层与派生链的踩坑；社区无对应节点。②先怀疑切分：三张症状表按主题已可分篇，但每张只有数行，读者按症状查时要一次扫完，聚合后仍自足。③只组合不发明：「故障」「速查」均普通词，后者依 cheatsheet 惯例。迁自笔记 08 §高频踩坑清单。 | Task 7 |
| `adoption-criteria` | DITA 采用判据 | **本库自造**。①穷尽查证：OASIS 规范只规定语言本身，不讨论要不要采用它；DITA-OT 4.4 docsrc 全部节点标题为安装、配置与发布操作，最近的「**DITA 2.0 support**」讲工具支持到哪一步、不讲选型；社区无对应节点。②先怀疑切分：本簇边界已核——customization 一篇管用起来之后改造到哪一级，editors-pdf-reality 管工具一路的取舍，两篇均不含「要不要用、能不能现在用」。③只组合不发明：「采用」「判据」均普通词，与本库既有准入判据一族同构。迁自笔记 08 三节 ＋ `research/README.md`「四个招牌特性」框架句（2026-08-16 裁定：不单独成篇，框架句归本篇，四个机制各归其 topic）。 | Task 7 |
| `customization-cost-ladder` | customization 与 specialization 的选用次序 | **非自造（改名后）——只用上游两个伞节点原名**。①穷尽查证：上游把这件事分在两处而无合并节点——OASIS archSpec「**Configuration, specialization, and constraints**」覆盖第三至第六级（constraint 与 attribute / domain / structural specialization），DITA-OT 4.4 docsrc「**Customizing DITA Open Toolkit**」覆盖第一、二级（outputclass 加样式、plugin 与 XSLT 覆盖）。②先怀疑切分：按上游切成两篇即失去本篇唯一内容（六级的相对代价与「能用前面的就不用后面的」这条判据，正建立在跨越两处之上），故保留为一个节点。③只组合不发明：两词均为上游节点原名，「选用次序」为普通词组。**原标题《DITA 二次开发的成本阶梯》的「成本阶梯」查无出处，已废**；文件名与 id 沿用 `customization-cost-ladder` 不改（id 是引用契约）。迁自笔记 08 §二次开发的成本阶梯。 | Task 7（标题重拟于 Task 13b） |
| `engineering-ci` | DITA 变体构建与 CI | **本库自造**。①穷尽查证：OASIS DITA 2.0 规范定义 conditional processing 机制本身，不讲持续集成该跑哪些构建；DITA-OT 4.4 docsrc 有「**Filtering and flagging content**」与「**Publishing with project files**」，各讲怎么跑一次构建，无「变体矩阵该全跑」这一节点；社区无对应节点。②先怀疑切分：本簇边界已核——profiling-ditaval 管条件属性与 DITAVAL 怎么写、留删怎么判，project-files 管变体矩阵怎么声明，validation-schematron 管校验分几种，三篇均不含「过滤改变合法性，故 CI 必须逐变体构建」。③只组合不发明：「变体」（本库词表 tool 值集既有用法）、「构建」、CI 均为既有词。迁自笔记 08 §工程化建议（笔记 04 §2 的 project file 一节已于 Task 13b 拆出）。**2026-08-17 人审修复轮主轴重写并改名**：原标题《DITA 工程化与 CI》下的三条做法有两条（目录按角色、id 契约）换成任何文档系统或代码库都成立，按新增的准入判据三（领域特有性）不属 dita 分支，已压缩为一句并指向 `naming-rules`；文件名与 id 不动（引用契约）。 | Task 7（主轴与标题重拟于 2026-08-17 人审修复轮） |
| `translation-modularity` | DITA 模块化的翻译收益 | **本库自造**。①穷尽查证：OASIS archSpec 的「**Translation**」一节（即本篇来源节所引）讲翻译要处理哪些属性与流程，不算模块化在翻译成本上的回报；DITA-OT 4.4 docsrc 全为工具操作；社区无对应节点。②先怀疑切分：与 translation-antipatterns 的分界已核；与 customization 一族无交集，无处可并。③只组合不发明：模块化与翻译均为上游既有说法，「收益」为普通词。迁自笔记 13 §1＋§3。 | Task 7 |
| `localization-attributes` | DITA 本地化属性速查 | **本库自造（聚合）**。①穷尽查证：OASIS DITA 2.0 archSpec 把这三个属性分列为三个并列节点——「**The dir attribute**」/「**The translate attribute**」/「**The xml:lang attribute**」（同在 Translation 一节之下，即本篇来源节所引三处），**无把三者合起来讲的节点**；DITA-OT 4.4 docsrc 全为工具操作。②先怀疑切分：三个属性各自只有数行，拆成三篇均不自足，且读者是一次查三个；聚合为速查是切分的结果而非回避。③只组合不发明：三属性同属上游 Translation 一节，「本地化属性」只是这一节点组的并列称呼；「速查」依 cheatsheet 惯例。迁自笔记 13 §2。 | Task 7 |
| `translation-antipatterns` | DITA 复用机制在翻译下的反模式 | **本库自造**。①穷尽查证：OASIS archSpec 有「**The translate attribute**」与「**Translation**」一节，讲属性语义与翻译流程，**不讲复用机制在翻译下会怎么坏**；DITA-OT docsrc 全为工具操作；社区无对应节点。②先怀疑切分：与 translation-modularity 的分界已核——那篇算模块化带来的收益，本篇列它被误用时的两个坏法，合篇会让收益与代价互相冲淡。③只组合不发明：复用机制与翻译均为上游既有说法，「反模式」为软件工程通行词组。迁自笔记 13 §4＋§5。 | Task 7 |
| `dita-rag-fit` | DITA topic 自足性与 RAG 切块的对应关系 | **本库自造（聚合）**。①穷尽查证：OASIS DITA 2.0 规范成文早于检索增强生成的实践，全篇无 RAG 相关节点；DITA-OT 4.4 docsrc 同样无；RAG 一侧的通行说法（分块、嵌入、召回）不讨论 DITA。**两个领域的上游各自都没有把它们对起来的节点。**②先怀疑切分：拆成「DITA 自足性」与「RAG 切块」两篇即失去本篇的全部内容——本篇要说的正是两者的对应关系。③只组合不发明：topic、自足性、RAG、切块四个词素分属两个领域且各自既有。迁自笔记 15 §1＋§5＋§6。 | Task 7 |
| `rag-parsed-content` | RAG 入库内容的解析形态选择 | **本库自造**。①穷尽查证：OASIS 规范无 RAG 相关节点；DITA-OT 4.4 docsrc 最近的是「**Pre-processing modules**」（讲各阶段做什么，不讲消费方该取哪一份输出）；RAG 一侧的通行说法不以 DITA 的中间产物为对象。②先怀疑切分：与 `toolchain/parsed-vs-source` 的分界已核——那篇给通用程序化处理该读哪一份输入，本篇只给 RAG 入库这一个用途下的取舍与代价，读者不同。③只组合不发明：四词素均既有，与 parsed-vs-source 的「输入选择」同构。迁自笔记 15 §2。 | Task 7 |
| `rag-chunking-metadata` | RAG 切块粒度与检索元数据 | **本库自造（聚合）**。①穷尽查证：OASIS DITA 2.0 规范与 DITA-OT 4.4 docsrc 全部节点标题均无 RAG 相关节点；RAG 一侧的通行说法讲分块策略与元数据过滤，但不以 DITA 的 topic 与 prolog 为对象。②先怀疑切分：粒度与元数据是同一次入库决策的两半——切多大决定了每块要带哪些字段，分成两篇会让判据落在两处。③只组合不发明：四词素均为两领域内既有词。迁自笔记 15 §3＋§4。 | Task 7 |

---

## 七、principles 簇（Task 12，5 篇）

> 5 篇标题于 Task 13b 重定（根因是已废止的「标题必含英文机制名」条款），自造依据头注释同批补齐。**本簇全部为自造聚合**——笔记 12 与 00 是论证篇，上游无对应节点。

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `first-principles` | DITA 的第一性原理推导 | **本库自造**。①穷尽查证：OASIS DITA 2.0 archSpec 逐一给出各机制节点（DITA specialization、Configuration, specialization, and constraints、Key-based addressing、Branch filtering、Determining effective attribute values、Expansion modules——即本篇来源节所引七处），**但无把这些机制从一条前提推导出来的节点**；DITA-OT 4.4 docsrc 全部节点标题均为工具操作，无原理类节点；社区亦无。②先怀疑切分：本簇边界已核——roles-and-boundaries 讲角色、schema-authority 讲定义权、costs-and-legacy 讲代价、portable-principles 讲剥离 DITA 后剩下什么，四篇均不含推导链本身。③只组合不发明：「第一性原理」「推导」均通用词组。迁自笔记 12 §一＋§二。原标题《DITA 单一真相源公理下的 topic、map 与 @class 推导》。 | Task 12（标题与三道关于 Task 13b） |
| `costs-and-legacy` | DITA 的代价与历史包袱 | **本库自造**。①穷尽查证：OASIS DITA 2.0 规范只在 Conformance 与各机制节点里给出规则，**不评价自身的代价**；DITA-OT 4.4 docsrc 全为工具操作与迁移说明（最近的「**Legacy constructs removed**」讲被移除的构造，不讲代价的成因）；社区无对应节点。②先怀疑切分：与 first-principles 的分界已核——那篇推导机制何以如此，本篇算这些机制要付的代价与哪些属历史包袱。③只组合不发明：「代价」「历史包袱」均普通词。迁自笔记 12 §三＋§四＋§五。原标题《DITA 的晚绑定代价与 conref push 等历史包袱》。 | Task 12（标题与三道关于 Task 13b） |
| `portable-principles` | DITA 之外可迁移的原则 | **本库自造**。①穷尽查证：OASIS DITA 2.0 规范只讲 DITA 自身，不讲剥离 DITA 之后剩下什么；DITA-OT 4.4 docsrc 全为工具操作；社区无对应节点。②先怀疑切分：与 first-principles 的分界已在根元素注释写明——那篇推 DITA 内的机制，本篇取与语法无关、可搬去别处的部分。③只组合不发明：「可迁移」「原则」均普通词。迁自笔记 12 §六＋§七。原标题《DITA 之外可迁移的 @class 自描述与晚绑定原则》。 | Task 12（标题与三道关于 Task 13b） |
| `roles-and-boundaries` | DITA 的角色分工与 processor 边界 | **本库自造**。①穷尽查证：OASIS DITA 2.0 archSpec 的「**Basic DITA terminology**」与「**Other terminology**」两节逐条定义 author / information architect / DITA architect / implementer / processor 这些角色名（即本篇来源节所引两处），**但没有把它们的分工与阅读边界合起来讲的节点**；DITA-OT 4.4 docsrc 全为工具操作，仅有「ToC navigation role」一条与 role 同形而不同义；社区无对应节点。②先怀疑切分：角色名的逐条定义属上游 terminology 两节，本库不重述；本篇讲的是分工与边界，无处可并。③只组合不发明：角色名与 processor 均为上游原词，「分工」「边界」为普通词。迁自笔记 00 §1–§4。原标题《DITA 规范中的角色分工与 processor 边界》。 | Task 12（标题与三道关于 Task 13b） |
| `schema-authority` | DITA schema 的定义权 | **本库自造**。①穷尽查证：OASIS DITA 2.0 archSpec 有「**The class attribute**」与「**Basic DITA terminology**」两节（即本篇来源节所引两处），前者定义 `@class` 的构造、后者定义 document type 与 document-type shell，**均不讲「谁有权定义 schema」**；DITA-OT 4.4 docsrc 全为工具操作（最近的「Configuring DITA-OT」讲工具配置）；社区无对应节点。②先怀疑切分：与 doctype-shell 的分界已核——那篇讲 shell 怎么装配，本篇讲装配权归谁、各角色能改什么。③只组合不发明：schema 为上游原词，「定义权」为普通词组。迁自笔记 00 §5–§8。原标题《DITA schema 的定义权与 @class 角色约束》（第二个论题已降入正文）。 | Task 12（标题与三道关于 Task 13b） |

---

## 八、content-engineering 簇（Task 13 两篇 + Task 13b 四篇）

> 本簇讲的是**本库自身的治理规约**，上游（OASIS / DITA-OT / Diátaxis / Good Docs Project）均无对应节点，**全部为自造**。标题一律以 `kb` 前缀起头，标出适用范围——这是本库规约而非领域公认知识（裁定：保留，全簇统一；正本见 `naming-rules`）。

| slug | 定稿标题 | 上游依据 | 来源任务 |
|---|---|---|---|
| `dimension-type-genre` | kb 内容分类的维度、类型与题材 | **本库自造聚合**。①穷尽查证：OASIS DITA 规范只定义 topic 类型（concept / task / reference / troubleshooting / glossentry）与 `@outputclass` 这一自由标注位，**不给类型之上的分层**；Diátaxis 只给一层四象限，无「维度 / 题材」层；Good Docs Project 给的是模板清单，同样不分层。三处查过，领域内确无「分层判定完整性」这个节点。②先怀疑切分：本簇既有边界已核——writing-typing 管单篇选哪个 DITA 类型，writing-atomicity 管进不进库，两篇均不含「框架本身分几层、完整性看哪层」。③只组合不发明：维度 / 类型 / 题材三词均为本库 subjectScheme 已登记的层名。迁自 `research/cases/kb-redesign/content-type-framework.md` 全文。 | Task 13 |
| `domain-dimension-method` | kb 领域维度框架与 80/20 取舍 | **本库自造聚合**。①穷尽查证：OASIS DITA 规范给的是 subjectScheme 与受控值绑定这类机制，**不给「一个知识领域怎么建起来」的方法节点**；DITA-OT 文档只谈处理与构建；Diátaxis 与 Good Docs Project 谈的是单篇文档形态。均无「先建完整维度再取舍」这一节点。②先怀疑切分：与 dimension-type-genre 的边界已核——那篇讲框架分几层（分类框架自身的形状），本篇讲单个领域内维度清单怎么建、怎么取舍、怎么约束（建域的顺序）。③只组合不发明：领域（prolog `data name="domain"` 既有字段名）、维度（dimension 值集）、80/20（通行说法）、取舍，四词素均既有。迁自 `research/cases/kb-redesign/dimension-completeness.md` 全文。 | Task 13 |
| `terminology-rules` | kb 术语治理 | **本库自造**。已查 OASIS DITA 2.0 archSpec 与 langRef 节点标题（最近的是「**Basic DITA terminology**」与「**Other terminology**」，规定的是**规范自己用词**的定义，不管写作方一侧的用词纪律）、DITA-OT 4.4 docsrc 全部节点标题（无术语治理类节点）、社区通行说法，均无对应节点。 | Task 13b |
| `naming-rules` | kb 命名与归属 | **本库自造**。已查 OASIS DITA 2.0 archSpec 与 langRef 节点标题（最近的是「**Basic DITA terminology**」，定义规范用词，不管内容库一侧的命名）、DITA-OT 4.4 docsrc 全部节点标题（命名相关的只有「Sample project file」一类具体样例）、社区通行说法，均无对应节点。**本篇是 R18 声明式溯源规则的归属正本。** | Task 13b |
| `rot-detection` | kb 腐烂检测机制 | **本库自造**。已查 OASIS DITA 2.0 archSpec 与 langRef 节点标题（最近的是 prolog 的 `<source>` 与 `<critdates>` 两个元素条目，**只给字段，不给复核纪律**）、DITA-OT 4.4 docsrc 全部节点标题（无内容治理类节点）、社区通行说法，均无对应节点。 | Task 13b |
| `writing-style` | kb 文体与语体 | **本库自造**。已查 OASIS DITA 2.0 规范与 DITA-OT 4.4 docsrc 全部节点标题（两者都只管标记与处理，**不管文体**）、Diátaxis 与 Good Docs Project（给的是结构模板，不是语体规约），均无对应节点。原标题《文体与结构：一篇长什么样，不由作者当场决定》，Task 13b 瘦身后重定。 | 存量篇，Task 13b 瘦身重定 |

**存量未标注篇**（迁移前既有，本次未补依据声明，回填时需另行查证）：`dita-authoring-guide`（已升为路由总纲）、`writing-atomicity`、`writing-typing`、`writing-sourcing`、`writing-llm-friendly`。

---

## 九、词表条目（glossary，随各簇建）

词表条目的「上游依据」是术语本身，不走节点索引（设计稿 §四之二：词表即中英对照，不在索引里重复一份）。本迁移新建 11 条，登记如下备查：

| 批次 | 条目 | 来源任务 |
|---|---|---|
| 第三批 | `term-conref`、`term-keyref` | Task 5 |
| 第四批 | `term-transtype`、`term-preprocess`、`term-plugin` | Task 6 |
| 第五批 | `term-ditaval`、`term-rag` | Task 7 |
| 第六批 | `term-specialization`、`term-generalization`、`term-constraint`、`term-document-type-shell`、`term-schema`、`term-subject-scheme` | Task 8 |

（`term-*` 形制：glossterm 为英文原名，中文释义进 glossdef，`<glossAlt>` 给中英别名。术语规则正本见 `terminology-rules`。）

---

## 十、非 dita 分支（迁移前存量，未在本次范围内）

`kb/topics/ai/`（10 篇）、`kb/topics/web/electron-landscape`、`kb/topics/engineering/agent-rules-core` 均为迁移前既有 topic，**头注释无上游依据声明**。按设计稿 §七之二「一定要推广到全库」，这批的回填需各自建立上游锚点（ai 分支的上游为 Claude Code 文档站，已在 benchmark-registry 登记为 `bm-ai`），不能从本表取材。

---

## 附之零：回填后的校订（2026-08-18，T4 执行结果）

回填已完成（R19，`kb/vocab/upstream-nodes.tsv` 逐条核对）。**本表下文各行的「上游依据」列有若干处经核实不成立**，正本以各 `.dita` 文件头注释里的「R19 回填核对」段与 prolog 的 `upstream-node` 声明为准：

| 本表所写 | 索引核实结果 |
|---|---|
| `structural-specialization` / `domain-specialization` 直取子树标题 | DITA 2.0 撤销了这两个子树，两者降为 `Overview of specialization` 节点内的两个 `<dt>` 术语 |
| `attribute-specialization` 直取子树标题 | 2.0 无该节点；属性一侧的节点名是 `Specialization rules for attributes` |
| `customization-cost-ladder` 取 `Configuration, specialization, and constraints` ＋ `Customizing DITA Open Toolkit` | 前者是 1.3 章名（2.0 为 `Configuration and specialization`）；后者 DITA-OT 4.4 根本没有，顶层只有 `Configuring DITA-OT` 与 `Extending DITA-OT with plug-ins` |
| `plugin-extension-points` 取 `DITA-OT extension points` | 那是 ditamap 的 map 标题，不是 topic 节点；索引里对应 `Extension point reference` |
| `project-files` 取 `Using project files` | 上游标题是 `Publishing with project files`（`using-project-files.dita` 是文件名） |
| `key-space-model` / `branch-filter-key-space` 以 key space 为「上游词」 | key space 不是节点名，是 `Key terminology` 里的 `<dt>` 术语 |
| `conref-pull-push` 取 DITA-OT 的两个 preprocess 阶段名 | 语言层节点是 `Content reference (conref)` 与 `Pushing reusable content to a new location` |
| `subjectscheme-taxonomy` 取 `Subject scheme maps`／DITA-OT `Subject schemes` | 前者对（复数，逐字）；后者 DITA-OT 4.4 索引内不存在 |

另有六篇由「自造」改判为**组合篇**——两端在索引里都有节点，声明多条比 `coined` 更能报出漂移：`nav-generation`（Navigation ＋ Indexes）、`localization-attributes`（三个属性节点）、`dita-ot-quickstart`、`generate-and-convert`、`table-model-choice`、`dita-resources`。

下文分诊表的三档篇数（25 / 11 / 30）随之改为 **28 / 14 / 24**。

---

## 附：回填时的分诊建议

按 progress.md 的索引边界裁定（2026-08-16），存量按声明数分诊：

| 情形 | 本表中的篇数（dita 域，共 66） | 处置 |
|---|---|---|
| 1 个上游节点声明 | 25 篇 | 抽查即可 |
| 2 个以上（纯组合，两端都有上游节点） | 11 篇 | **核是否切分错位** |
| 0 个（`coined`，含半自造与组合式自造） | 30 篇 | **核三道关**，且**不得由原作者复核**（`rot-detection` 边界二） |

（设计稿写的是「65 篇」，那是 Task 10b 收口时的数；Task 13b 切分复议拆出 `project-files` 后为 66。content-engineering 的 6 篇治理规约另计，全部为自造。）

边界情形（分诊时按两栏都算一遍）：`nav-generation` 三个上游类目却仍属组合式自造；`dita-ot-quickstart` 由两个上游节点聚合而成但合并动作出自本库；`dita-resources`、`generate-and-convert` 为「半自造」（一侧有节点、一侧无）；`table-model-choice` 两个元素页各有节点但「选用」这一节点不存在。

已知待切分复核两处：`nav-generation`（三个上游类目合一，真正解法是重切，超 Task 10b 授权）、`generate-and-convert`（生成一侧无上游节点，建议单独成篇）。

> 索引查不出「节点存在但选错了」——声明与标题出自同一判断。这是这套方案的固有盲区，靠独立复核而非机器兜底。
