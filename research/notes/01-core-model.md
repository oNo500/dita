# 01 · 核心模型

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/core-model/topic-typing.dita` ← §1、§1.5；`kb/topics/dita/core-model/map-structure.dita` ← §2；`kb/topics/dita/core-model/titlealt-system.dita` ← §3；`kb/topics/dita/core-model/table-model-choice.dita` ← §3.5；`kb/topics/dita/core-model/images-multimedia.dita` ← §3.6；`kb/topics/dita/architecture/class-derivation.dita` ← §4–5（该篇 2026-08-16 由 core-model 簇归位到 architecture 簇，路径已随之更正，归位理由见该篇文件头注释），其中 §5 的 `@specializations` 落地用法另见 `kb/topics/dita/architecture/attribute-specialization.dita`；不迁小节：§定位 与 §6「与其他方案的定位差」——两节的内容已并入 `kb/topics/dita/dita-landscape.dita` 的维度框架论述，§定位 的三件事分别落在「核心模型（dim-concept）」「扩展设施内部构造（dim-internals）」两行与「四、扩展与外接」的导语，§6 的七行对比表与适用条件整表落在「与其他方案的定位差」一节），本文冻结为调研档案，不再更新。

## 定位

DITA 不是"一种 XML 格式"，而是**一套用 XML 表达的、可被程序化扩展的内容架构**。全部设计围绕三件事：

1. **类型化（typed topics）** —— 内容按"是什么"分类，不按"在哪一章"组织
2. **map 与内容分离** —— 结构、顺序、层级不写在内容里
3. **`@class` 派生链** —— 任何自定义元素都携带它的基类路径，使下游处理不必认识它也能处理

第 3 点是 DITA 区别于 DocBook / Markdown / AsciiDoc 的根本，也是二次开发的全部基础。

---

## 1. Topic：最小可寻址单元

一个 topic = 一个文件 = 一个可独立阅读、可被任意 map 引用的内容块。

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE topic PUBLIC "-//OASIS//DTD DITA 2.0 Topic//EN" "topic.dtd">
<topic id="intro">
  <title>产品简介</title>
  <shortdesc>一句话说明这个 topic 讲什么。</shortdesc>
  <prolog>
    <titlealt title-role="navigation">简介</titlealt>
    <author>xiu</author>
    <metadata><keywords><keyword>DITA</keyword></keywords></metadata>
  </prolog>
  <body>
    <p>正文。</p>
  </body>
</topic>
```

**内容模型**（严格有序）：

```
title, (shortdesc | abstract)?, prolog?, body?, related-links?, topic*
```

- `@id` **必需** —— 它是 conref / xref 的寻址锚点
- `<shortdesc>` 和 `<abstract>` 互斥，二选一
- 允许嵌套 topic（`topic*`）
- **替代标题放在 `<prolog>` 里的 `<titlealt>`**，见下文第 3 节。注意 `<prolog>` 的内容模型是 `titlealt*, author*, source?, publisher?, copyright*, critdates?, permissions?, metadata*, resourceid*, …` —— **`<titlealt>` 必须排在最前，在 `<author>` 之前**

## 1.5 信息类型化（Information Typing）

**这是 DITA 名字里的 "IT"** —— **D**arwin **I**nformation **T**yping **A**rchitecture。不是附属特性，是架构自我命名时选中的那个核心。

### 规范的定义

> 信息类型化是**识别 topic 的类型**（如 concept、reference、task）的实践，用以**清晰区分不同种类的信息**。

理由：让文档保持**聚焦且模块化**，从而对读者更清晰、更易检索导航、**更适合复用**。

### 最关键的一点：类型 ≠ 主题

**主题（subject）和类型（type）是两个正交的维度。** 这是最常见的误解。

同一个主题会自然产生**多篇不同类型**的 topic：

| 主题 | Concept | Task | Reference |
|---|---|---|---|
| 打印队列 | 什么是打印队列、如何工作 | 如何清空打印队列 | 队列状态码一览 |
| 认证 | OAuth 的授权模型 | 如何配置 OAuth 客户端 | 认证 API 参数表 |

按主题组织会写出"打印机"这一篇 —— 概念、步骤、参数表全混在一起。**那正是类型化要消灭的东西。**

### 三个最常用的类型

| 类型 | 回答 | body | 典型内容 |
|---|---|---|---|
| **Concept** | **What is…?** | `conbody` | 背景、原理、术语 —— 动手前需要理解的 |
| **Task** | **How do I…?** | `taskbody` | 精确分步指令：前提、动作、顺序 |
| **Reference** | **查什么？** | `refbody` | 规格、参数、错误码 —— **用来查而非用来记**的数据 |

Reference 的判据"often looked up rather than memorized（用来查而不是用来记）"很好用 —— 拿不准一段内容该不该进 reference 时，问这一句。

### DITA 2.0 内置的全部类型

三个不是全部。2.0 实际提供的：

**Base 包**

| 类型 | 说明 |
|---|---|
| `<topic>` | 泛型 topic，**所有类型的根基类** |
| `<map>` | 基础 map（map 类型） |
| `<subjectScheme>` | 受控值表（map 类型），见 [03](03-profiling-and-chunking.md) |

**技术内容包（Technical Content）** —— 该包的 6 个结构模块

| 类型 | 回答 / 用途 |
|---|---|
| `<concept>` | What is…? |
| `<task>` | How do I…? |
| `<reference>` | 查什么？ |
| `<glossentry>` | 术语条目 —— 一个术语的一个义项 |
| `<glossgroup>` | 术语组 —— 容纳多个 `<glossentry>` |
| `<troubleshooting>` | 出问题了怎么办？（`condition` / `cause` / `remedy`） |
| `<bookmap>` | 书籍结构（**map 类型**，不是 topic 类型） |

合计：**topic 类型 7 个**（topic、concept、task、reference、glossentry、glossgroup、troubleshooting），**map 类型 3 个**（map、subjectScheme、bookmap）。

### 2.0 砍掉的

| 被删 | 原属 |
|---|---|
| **machinery task**（域 + doctype） | 技术内容 |
| **classification map**（域 + doctype） | 技术内容 |
| `glossary.dtd` | 改用 `glossentry.dtd` |

### DITA 2.0 只有两个包在活跃开发

OASIS 按**分委员会分仓**开发 DITA 2.0。核对各仓库的实际状态：

| 包 | 仓库 | 状态 |
|---|---|---|
| **Base** | `oasis-tcs/dita` | ✅ v2.0-beta03（2026-07-02） |
| **Technical Content** | `oasis-tcs/dita-techcomm` | ✅ 默认分支即 `DITA-2.0`，v2.0-beta03（2026-07-02），**与 base 同步发版** |
| **Learning & Training** | `oasis-tcs/dita-learning-training` | ❌ **空仓** —— 只有 3 个模板文件，README 仍是未填写的占位模板，**最后一次推送为 2017-11-20**，无任何 release |
| **LwDITA** | `oasis-tcs/dita-lwdita` | ⚠️ **独立版本线**（v0.3.0.2，2024-01），不属于 DITA 2.0 |

> **结论：DITA 2.0 = Base 包 + 技术内容包，就这两个。**
>
> 学习与培训包在 2.0 里**没有任何工作产出** —— 仓库建了但从未填充，停在 2017 年。1.2/1.3 的 `learningContent` / `learningOverview` / `learningAssessment` 那一套**不要指望在 2.0 里有对应物**。
>
> LwDITA 是并行的独立规范线（XDITA / HDITA / MDITA），有自己的版本号，不随 DITA 2.0 走。

### 两个包各自装了什么

核对 v2.0-beta03 的 RNG 语法文件目录：

**Base 包**（`doctypes/rng/` 下三个目录：`base` / `ditaval` / `subjectScheme`）

- 文档类型外壳：`basetopic` `basemap` `ditaelement`
- 模块：`topicMod` `mapMod` `commonElementsMod` `metaDeclMod` `tblDeclMod`
- 域：`alternativeTitlesDomain`（`<navtitle>` 等）、`emphasisDomain`（`<strong>`/`<em>`）、`highlightDomain`、`hazardstatementDomain`、`mapGroupDomain`、`utilitiesDomain`、`ditavalrefDomain`
- 属性域：`audienceAttDomain` `platformAttDomain` `productAttDomain` `deliveryTargetAttDomain` `otherpropsAttDomain`
- **DITAVAL 自己也有语法文件**

**技术内容包**（`doctypes/rng/` 下两个目录：`technicalContent` / `bookmap`）

- 文档类型外壳：`concept` `task` `generalTask` `reference` `glossentry` **`glossgroup`** `troubleshooting` `topic` `map` `ditabase`
- 模块：`conceptMod` `taskMod` `referenceMod` `glossentryMod` `glossgroupMod` `troubleshootingMod`
- **约束模块**：`strictTaskbodyConstraintMod` ← strict task 是靠**约束模块**实现的
- 域：`programmingDomain` `softwareDomain` `uiDomain` `hwDomain`（硬件）、`syntaxdiagramDomain`、`markupDomain`、`xmlDomain`、`abbreviateDomain`、`equationDomain`、`glossrefDomain`、`releaseManagementDomain`、`mathmlDomain`、`svgDomain`

两个印证：

1. **`<glossgroup>` 在 2.0 里存在**（`glossgroup.rng` + `glossgroupMod.rng`）。被删的是 `glossary.dtd` 这个 shell，不是 glossgroup 本身
2. **strict task 与 general task 确实是两个 shell**（`task.rng` vs `generalTask.rng`），且 strict 是通过 `strictTaskbodyConstraintMod` 这个**约束模块**实现的 —— 正好印证下一节的层级区分

### 信息类型 ≠ 文档类型外壳

两个不同层级，容易混：

- **strict task 和 general task 不是两个信息类型**，是同一个 `<task>` 类型的**两个外壳**
- 同一个 `<topic>` 可被不同外壳配置出完全不同的允许范围

**数类型时数的是结构模块，不是外壳。**

> 规范里"文档类型"和"文档类型外壳"也是两个词：**文档类型**是一组模块的组合（抽象），**外壳**是实现该组合的 DTD/RNG 文件（具体）。定义见 [00 第 5 节](00-roles-and-boundaries.md)，外壳的作用见 [09](09-architecture-foundations.md)。

### 类型集合是无界的

规范的原话：

> **可能的信息类型集合是无界的（unbounded）。**

新类型通过**专门化**产生 —— 从基础 `<topic>` 派生，或在既有类型（concept / task / reference 等）上细化。

**这里有个容易被忽略的架构事实：内置的那些类型本身就是专门化的产物。**

```
<concept class="- topic/topic concept/concept ">
<task    class="- topic/topic task/task ">
```

OASIS 并没有用什么特权机制造出 concept 和 task —— **它们就是用你会用的那套机制造的**。整个技术内容包本质上是"base 的一组专门化"。

所以内置类型和你自己造的类型，**在架构上是同一等级的公民**，唯一区别是前者写进了规范、有生态支持。这正是名字里 "Darwin"（演化 / 谱系）的含义：**类型从类型繁衍而来**。

详见 [05-specialization.md](05-specialization.md)。

配套的务实建议（规范给的）：**已有类型能匹配你的内容时，就直接用、或以它为专门化基类**，以保证内容的顺畅交换与互操作。别为独特而独特。

### 三种常见的失败模式

1. **全写成 concept**（或全用泛型 `<topic>`）—— 等于没有类型化，却仍要承担 DITA 的成本
2. **按主题切而非按类型切** —— 上面那篇"打印机"
3. **为凑类型硬拆** —— 拆出无法独立理解的碎片。这违背了更根本的约束：**复用单元的边界是自足性**（见 [12](12-philosophy-and-principles.md) 推论 2）

**判断顺序**：先问这块内容能否独立成立，再问它是哪一类。类型化服务于自足性，不能反过来破坏它。

### Task：约束最强的一个

```xml
<task id="install">
  <title>安装 DITA-OT</title>
  <taskbody>
    <prereq>需要 Java 17 或更高版本。</prereq>
    <context>说明这个任务的场景。</context>
    <steps>
      <step>
        <cmd>下载发布包。</cmd>
        <info>可选补充说明。</info>
        <stepresult>下载完成。</stepresult>
      </step>
      <step>
        <cmd>解压并配置。</cmd>
        <!-- 子步骤 = 直接嵌套 <steps> -->
        <steps>
          <step><cmd>解压到目标目录。</cmd></step>
          <step><cmd>把 <filepath>bin/</filepath> 加入 PATH。</cmd></step>
        </steps>
      </step>
      <step>
        <cmd>验证安装。</cmd>
        <choices>
          <choice>执行 <cmdname>dita --version</cmdname></choice>
          <choice>执行 <cmdname>dita plugins</cmdname></choice>
        </choices>
      </step>
    </steps>
    <result>命令行可执行 <cmdname>dita</cmdname>。</result>
    <tasktroubleshooting>如果提示找不到命令，检查 PATH。</tasktroubleshooting>
    <example>...</example>
    <postreq>接下来可以构建示例工程。</postreq>
  </taskbody>
</task>
```

### `<taskbody>` 的内容模型（strict task）

```
prereq?, context?, (steps | steps-unordered)?, result?,
tasktroubleshooting?, example?, postreq?
```

全部可选，但**顺序固定、各出现一次**。

### strict task vs general task

这个区分在 2.0 里**仍然存在**，且由**你选哪个 doctype shell** 决定，不是文档里的开关：

| | strict task | general task |
|---|---|---|
| 内容模型 | 严格按上面的顺序，各一次 | 顺序灵活，元素可重复 |
| 通用 `<section>` | 不允许 | 允许 |
| `<steps-informal>` | 不在模型里 | 可用 |
| 适合 | 真正的操作步骤，机器可处理 | 结构不规整的过程性内容 |

**默认选 strict task。** 它的约束正是 DITA 的价值所在 —— general task 是为了容纳不规整的内容而做的妥协，用了就等于放弃了 task 类型化带来的大部分好处。

### `<steps>` / `<step>` 要点

- `<steps>` 内容模型：`data*, stepsection?, step+`
- 每个 `<step>` **必须**以 `<cmd>` 开头，且**只能有一个** —— "一步一个动作"的机器可执行约束
- **子步骤靠 `<steps>` 直接嵌套在 `<step>` 里**（没有单独的子步骤元素）
- `<stepsection>` 用于在步骤序列中间插入分组说明

### 三种步骤容器

| 元素 | 说明 |
|---|---|
| `<steps>` | 有序步骤，强制 `<cmd>` |
| `<steps-unordered>` | 无序步骤，同样强制 `<cmd>` |
| `<steps-informal>` | **从 `<section>` 专门化而来**，松散描述，不强制 `<cmd>`。一段话可以描述多个步骤，或把操作和说明混在一起。只在 general task 下可用 |

### 另外两个 topic 类型的要点

| 类型 | 结构 | 用法 |
|---|---|---|
| **glossentry** | `glossterm` + `glossdef` | 一个 topic 定义**一个术语的一个义项**。配合 keyref 做全库术语统一：`<keydef keys="term-api" href="glossary/api.dita"/>`，正文用 `<term keyref="term-api"/>` |
| **troubleshooting** | `condition` / `cause` / `remedy` | 症状 → 原因 → 处置。比"把排查写成 task"更适合复用 —— 同一个 remedy 可被多个 condition 引用 |

---

## 2. Map：结构与内容分离

map 里没有正文，只有引用和关系。

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE map PUBLIC "-//OASIS//DTD DITA 2.0 Map//EN" "map.dtd">
<map>
  <title>用户手册</title>
  <topicmeta>
    <titlealt title-role="subtitle">2026 版</titlealt>
  </topicmeta>

  <topicref href="topics/intro.dita"/>

  <!-- 层级：子 topicref 即输出中的下级章节 -->
  <topicref href="topics/getting-started.dita">
    <topicref href="topics/install.dita"/>
    <topicref href="topics/first-build.dita"/>
  </topicref>

  <!-- 只做分组，自身不产出页面、不进导航 -->
  <topicgroup>
    <topicref href="topics/a.dita"/>
    <topicref href="topics/b.dita"/>
  </topicgroup>

  <!-- 有导航标题但无对应文件的节点 -->
  <topichead>
    <topicmeta><navtitle>附录</navtitle></topicmeta>
    <topicref href="topics/faq.dita"/>
  </topichead>

  <!-- 引用另一个 map -->
  <mapref href="reference/api.ditamap"/>

  <!-- key 定义，不出现在导航里 -->
  <keydef keys="product-name">
    <topicmeta><keytext>DITA 工具箱</keytext></topicmeta>
  </keydef>

  <!-- 关系表：自动生成相关链接 -->
  <reltable>
    <relheader>
      <relcolspec type="concept"/>
      <relcolspec type="task"/>
      <relcolspec type="reference"/>
    </relheader>
    <relrow>
      <relcell><topicref href="topics/intro.dita"/></relcell>
      <relcell><topicref href="topics/install.dita"/></relcell>
      <relcell><topicref href="topics/cli-ref.dita"/></relcell>
    </relrow>
  </reltable>
</map>
```

### `<mapref>`：被引 map 的标题**不**产生导航节点

这条最容易踩，因为它违反直觉：引一个 map 进来，产物里看不到那个 map 的名字。

规范定义的处理是——**被引 map 的层级在引用位置并入容器 map**（"The hierarchy of the referenced map is merged into the container map at the position of the reference"）。合并的是 `<topicref>` 们，被引 map 的 `<title>` 只是那个文件的标题，**不会变成一个导航节点**。

```xml
<!-- 有 10 个 topicref 的 domains/web.ditamap，标题「Web 技术栈」 -->
<mapref href="domains/web.ditamap"/>
```

产物里得到的是那 10 篇平铺在当前层级，**没有「Web 技术栈」这一层**。map 越多，顶层越乱——每个被引 map 的组织意图都在合并中消失了。

想保住分组名，两种办法：

| 办法 | 写法 | 代价 |
|---|---|---|
| `<topichead>` 包一层 | navtitle 写分组名，里面放 `mapref` | 分组名有了第二份副本，会与被引 map 的 `<title>` 漂移；节点点进去是空壳，没有对应页面 |
| 用真实 topic 当父节点 | `<topicref href="该分支的落地页.dita">` 下嵌子项 | 需要真写一篇落地页；换来的是分组节点是真实内容，且没有标题副本 |

选哪个取决于那一层**值不值得有一个页面**。纯组织节点（如"术语库"）用 `topichead`；有话可说的分支（如"Web 技术栈"概览）用落地 topic 更好。

> 实测（2026-08-15，DITA-OT 4.4 `-f html5`）：同一个 root map 里，包了 `topichead` 的分支在 TOC 中保有分组，没包的分支其 topic 直接摊到顶层，与其他顶层条目并排。这不是 DITA-OT 的行为偏差，是规范定义的合并语义。

### `<reltable>` 值得单独强调

在 Markdown 世界里"相关链接"要在每篇文章底部手写，且是 N² 维护。reltable 把关系声明在**一处矩阵**里，同一 relrow 内的 topic 自动互链。换一套产品线只要换 reltable，正文一个字不动。这是 map 与内容分离带来的直接收益。

精确的链接规则（同行不同格才互链、同格默认不链、`targetonly` 单向列）见 [11](11-processing-model.md) 第 5.7 节。

### topicref 的关键属性

| 属性 | 作用 |
|---|---|
| `@href` / `@keyref` | 直接寻址 / 间接寻址（**优先用 keyref**） |
| `@keys` | 定义 key，值为 `NMTOKENS`（严格空格分隔） |
| `@format` | `dita`(默认) / `ditamap` / `html` / `markdown` / … |
| `@scope` | `local`(默认) / `peer` / `external` —— 决定是否纳入构建 |
| `@type` | 目标类型提示（`concept`/`task`/…），影响 reltable 和链接文本 |
| `@toc` | `yes` / `no` |
| `@processing-role` | `normal` / `resource-only` —— resource-only 只提供资源不产出页面，`<keydef>` 的本质 |
| `@collection-type` | `sequence`(有前后关系) / `family`(互为兄弟) / `choice` / `unordered` |
| `@linking` | `normal` / `none` / `sourceonly` / `targetonly` |
| `@chunk` | `combine` / `split`，见 [03](03-profiling-and-chunking.md) |

### bookmap

`bookmap` 是 map 的**结构化专门化**：

```xml
<bookmap>
  <booktitle><mainbooktitle>用户手册</mainbooktitle></booktitle>
  <frontmatter><preface href="topics/preface.dita"/></frontmatter>
  <chapter href="topics/ch1.dita">
    <topicref href="topics/ch1-a.dita"/>
  </chapter>
  <appendix href="topics/appx.dita"/>
  <backmatter><indexlist/></backmatter>
</bookmap>
```

注意 `<chapter>` 的 `@class` 是 `- map/topicref bookmap/chapter ` —— 对不认识 bookmap 的处理器，它**就是**一个 topicref。这就是专门化的兑现方式。

---

## 3. 标题体系：`<titlealt>`

一个元素 + 一个**必填**的 `@title-role` 覆盖所有"非正文标题"的场景。

| `@title-role` | 用途 |
|---|---|
| `linking` | 层级链接和 reltable 链接的文本；**是 navigation / search 的兜底** |
| `navigation` | 目录和导航 |
| `search` | 搜索结果里显示的标题 |
| `subtitle` | 副标题 |
| `hint` | 给 map 作者看的提示，**不参与任何处理** |

```xml
<topicref keys="about" href="about.dita">
  <topicmeta>
    <titlealt title-role="linking navigation">关于本产品</titlealt>
    <titlealt title-role="search">关于</titlealt>
    <titlealt title-role="hint">这里指 Acme TextMax 5000</titlealt>
  </topicmeta>
</topicref>
```

要点：

- `@title-role` 支持**多值**（空格分隔），也支持处理器自定义值
- 可出现在：map 根的 `<topicmeta>`、`<topicref>` 的 `<topicmeta>`、topic 的 `<prolog>`
- **合并规则**：`<topicref>` 上的替代标题与 topic 内的合并，**topicref 优先**
- alternative-titles 域提供便捷简写：`<linktitle>` `<navtitle>` `<searchtitle>` `<subtitle>` `<titlehint>`，等价于带预设 role 的 `<titlealt>`

`title-role="hint"` 的用途是在 map 里给作者留一句说明（这个 topicref 指的是什么），不参与任何输出。

---

## 3.5 表格：两套体系怎么选

DITA 自带两套表格，作者几乎每天都要面对这个选择：

| | `<simpletable>` | `<table>`（CALS/OASIS Exchange） |
|---|---|---|
| 模型 | `title?, sthead?, strow+` | `title?, tgroup+`，tgroup 内 colspec/thead/tbody |
| 跨行跨列 | 不支持 | 支持（`@morerows`、`@namest/@nameend`） |
| 列宽 | `@relcolwidth="1* 2*"` | `<colspec>` 逐列声明 |
| 行/列表头 | `sthead` + `@keycol`（第 N 列作为纵向表头） | `<thead>` + `@rowheader` |
| 复杂度 | 低 | 高 —— 是 DITA 里最复杂的结构之一 |

**判断**：默认用 `<simpletable>`，只在真的需要合并单元格时才用 CALS `<table>`。理由有三：

1. **2.0 的 simpletable 已支持 `<title>`**，"简单表没有标题"这个 1.x 时代换用 CALS 的理由不存在了
2. simpletable 是**理想的专门化基类**（[05](05-specialization.md) 的 `params` 就从它派生）；CALS 表几乎无法专门化
3. 输出到 Markdown 等弱格式时，合并单元格必然降级（见 [07](07-programmatic-processing.md) 的映射难点表）—— 源头少用，下游少痛

团队约束里"限制只能用 simpletable"是常见且合理的做法（见 [05](05-specialization.md) 约束一节）。

另外两个专用表：`<choicetable>`（task 里的选择表，simpletable 的专门化）、`<properties>`（reference 里的属性表，同样派生自 simpletable）—— 优先用它们而不是裸表格，语义更明确。

---

## 3.6 图像与多媒体

**`<image>`** 的要点：

- 引用走 `@href` 或 `@keyref`（key 定义里的 `<keytext>` 自动充当 alt 文本，见 [02](02-reuse.md)）
- `@placement`：`inline`（默认，随文）/ `break`（独立成块）
- 缩放：`@width` / `@height`（带单位）或 `@scale`（百分比）；`@scalefit="yes"` 让图适配可用宽度
- **无障碍**：行内提供 `<alt>`；纯装饰图显式写空 `<alt></alt>`，否则读屏器会读文件名

**`<audio>` / `<video>`**（2.0 新增，来自 multimedia 域）：

```xml
<video width="640" height="360" controls="true">
  <video-poster href="media/demo-cover.png"/>
  <media-source href="media/demo.webm"/>
  <media-source href="media/demo.mp4"/>
  <media-track href="media/demo-zh.vtt" kind="subtitles" srclang="zh-CN"/>
  <fallback><p>当前输出格式不支持视频，见 <xref href="media/demo.mp4" format="mp4" scope="external">视频文件</xref>。</p></fallback>
</video>
```

- 源可以两种方式给：单一来源直接写 `@href`；多格式候选用多个 `<media-source>`（处理器/浏览器选第一个能播的）
- 播放行为：`@autoplay` `@controls` `@loop` `@muted`
- `<media-track>` 挂字幕，`<video-poster>` 是加载前的封面帧
- **`<fallback>` 务必写** —— PDF 等静态输出无法内嵌视频，规范允许处理器此时用 fallback；这与 `<include>` 的 fallback 是同一个设计模式

---

## 4. `@class`：元素的派生链

每个 DITA 元素在 DTD/RNG 里都有一个默认的 `@class` 值，记录它从基类一路特化下来的**派生链**：

```
class="- topic/li task/step "
       ↑  ↑         ↑        ↑
       │  最基类    最特化    必须有结尾空格
       前缀
```

规则：

- **前缀 `-`** = 结构化专门化（topic / map 派生）；**前缀 `+`** = 域专门化
- 值是空格分隔的 `模块名/元素名` 对，**从最泛化到最特化**排列；链条必须包含**中间模块**，即使该层没有发生改名 —— 泛化算法依赖完整链
- **首尾各有一个空格**，不是排版洁癖 —— 是为了让 `contains(@class, ' topic/li ')` 精确匹配而不误命中 `topic/link`
- 作者**通常不写** `@class`，值由 DTD/RNG 声明的默认值提供。规范要求这个默认值**不能是 fixed**，以便泛化后的文档能携带显式的 `@class` 并被还原
- **用不做 schema 校验的方式解析，`@class` 会全部取不到**（见 [07](07-programmatic-processing.md)）

### 由此得出的处理规则

> **任何处理 DITA 的代码，都必须按 `@class` 匹配，绝不按元素名匹配。**

```xml
<!-- 对 ✅ 自动覆盖所有 li 的专门化（step、chdeschd…） -->
<xsl:template match="*[contains(@class, ' topic/li ')]">

<!-- 错 ❌ 任何人做一次专门化，你的模板就失效 -->
<xsl:template match="li">
```

域专门化元素同理：`<uicontrol class="+ topic/ph ui-d/uicontrol ">` —— 不认识 UI 域的处理器把它当 `<ph>` 渲染，仍然正确，只是丢了样式。**优雅降级是内建的。**

---

## 5. `@specializations`：属性专门化的派生链

topic / map 根元素上的 `@specializations` 记录**本文档用到了哪些属性专门化**：

```xml
<task specializations="@props/deliveryTarget @props/platform @props/product">
```

语法：`'@', (props|base), ('/', 属性名)+`

```
@props/myNewProp                    从 @props 派生 @myNewProp
@base/myFirstBase/mySecondBase      两级派生
@props/platform/hardwarePlatform    从已有的 @platform 再派生
```

要点：

- **只记录属性专门化，不记录元素域**
- 值由 doctype shell 的默认值注入，作者不手写 —— 和 `@class` 一样，**裸解析拿不到**
- 层级路径让处理器能正确地逐级泛化：`hardwarePlatform="x86"` → `platform="x86"` → `props="platform(x86)"`

详见 [05-specialization.md](05-specialization.md)。

---

## 6. 与其他方案的定位差

| | DITA | DocBook | Markdown/MDX | AsciiDoc |
|---|---|---|---|---|
| 语义粒度 | 极细（`<cmdname>` vs `<filepath>`） | 细 | 无 | 中 |
| 类型化强制 | **有**（concept/task/reference） | 无 | 无 | 无 |
| 内建复用 | **conref + keyref + include** | 有限（XInclude） | 无 | include |
| 条件化 | **DITAVAL，正交多维** | profiling | 无 | ifdef（一维） |
| 可扩展性 | **专门化 + 自动降级** | 改 schema，无降级 | 靠插件 | 靠扩展 |
| 作者门槛 | 高 | 高 | 极低 | 低 |
| 工具成本 | 高 | 中 | 极低 | 低 |

DITA 适用的场景是：**同一批内容要产出多个变体（多产品/多受众/多渠道），且内容量大到人工同步会出错。** 不满足这个条件时，成本会超过收益。

---

→ 下一步：[02-reuse.md](02-reuse.md)

---

## 来源

**已逐页核对（2026-08）**

- [信息类型化（架构规范）](https://dita-lang.org/dita/archspec/base/information-typing) — 定义原文（"识别 topic 的类型…以清晰区分不同种类的信息"）；理由（聚焦、模块化 → 更清晰、易检索导航、**更适合复用**）；**"可能的信息类型集合是无界的"**；新类型经专门化产生（从 `<topic>` 派生或在既有类型上细化）；"已有类型匹配时应直接用或作为专门化基类，以保证交换与互操作"
- [concept](https://dita-lang.org/dita-techcomm/langref/technicalcontent/concept) — 确认 `<concept>` **属技术内容包**（非 base）；内容模型 `title, (abstract|shortdesc)?, prolog?, conbody?, related-links?, concept*`；回答 "what is?"
- [技术内容包元素分组](https://dita-lang.org/dita-techcomm/langref/containers/task-elements) — 该包结构模块的**完整清单**：Book map / Concept / Glossary entry / Reference / Task / Troubleshooting elements（本篇类型清单表的依据）
- [DITA 规范列表](https://www.dita-lang.org/specifications) — DITA 2.0 仅两份草案文档（「DITA 2.0」与「DITA Technical Communication 2.0」）；LwDITA 为独立草案线
- **OASIS 仓库实况**（经 GitHub API 直接核对 v2.0-beta03，2026-08）—— 包结构与 L&T 定论的依据：
  - [`oasis-tcs/dita`](https://github.com/oasis-tcs/dita) — base 规范源与 `doctypes/rng/{base,ditaval,subjectScheme}`；v2.0-beta03（2026-07-02）
  - [`oasis-tcs/dita-techcomm`](https://github.com/oasis-tcs/dita-techcomm) — 默认分支即 `DITA-2.0`；v2.0-beta03（2026-07-02）；`doctypes/rng/{technicalContent,bookmap}`。本篇两个包的 shell / 模块 / 域清单均逐文件列自该目录
  - [`oasis-tcs/dita-learning-training`](https://github.com/oasis-tcs/dita-learning-training) — 仅 3 个模板文件，README 为未填写占位模板，**最后推送 2017-11-20，无 release**（L&T 在 2.0 无产出的依据）
  - [`oasis-tcs/dita-lwdita`](https://github.com/oasis-tcs/dita-lwdita) — 独立版本线，最新 v0.3.0.2（2024-01）
  - `glossgroup.rng` / `glossgroupMod.rng` 与 `strictTaskbodyConstraintMod.rng` 的存在，即"glossgroup 未被删"和"strict task 由约束模块实现"两条的直接证据
- [迁移到 DITA 2.0](https://dita-lang.org/2.0/dita/non-normative/information-about-migrating-to-dita-2-0) — machinery task 域与 doctype、classification map 域与 doctype、`glossary.dtd` 均被移除
- [topic（语言参考）](https://dita-lang.org/dita/langref/base/topic) — `<topic>` 内容模型 `title, (shortdesc|abstract)?, prolog?, body?, related-links?, topic*`；`@id` 必需；shortdesc 与 abstract 互斥
- [prolog](https://dita-lang.org/dita/langref/base/prolog) — 内容模型 `titlealt*, author*, source?, publisher?, copyright*, critdates?, permissions?, metadata*, resourceid*, (data|foreign)*`；**`<titlealt>` 位于最前**（本篇示例的顺序依据）
- [simpletable](https://dita-lang.org/dita/langref/base/simpletable) — 内容模型 `title?, sthead?, strow+`（**2.0 支持 `<title>`**）；`@relcolwidth` 比例列宽；`@keycol` 指定纵向表头列
- [video](https://dita-lang.org/dita/langref/base/video) — 子元素顺序 `desc?, longdescref?, fallback?, video-poster?, media-source*, media-track*, foreign*`；源可走 `@href`（单一）或 `<media-source>`（多格式候选）；播放属性 `@autoplay` `@controls` `@loop` `@muted`
- [class 属性规则与语法](https://dita-lang.org/dita/archspec/base/specialization-class-attribute) — 前缀 `-`（结构模块）/ `+`（域模块）后跟一个或多个空格；token 形如 `模块名/类型名`，**从最泛化到最特化排列，须含未改名的中间模块**；**值必须以至少一个空格结尾**（保证含首尾空格的整 token 匹配可靠）；除 ditabase 根 `<dita>` 外每个 DITA 元素都必须有 `@class`；语法声明的默认值不得为 fixed
- [taskbody](https://dita-lang.org/dita-techcomm/langref/technicalcontent/taskbody) — strict task 内容模型含 `<tasktroubleshooting>`；strict vs general task 由 doctype shell 决定
- [steps](https://dita-lang.org/dita-techcomm/langref/technicalcontent/steps) — 内容模型 `data*, stepsection?, step+`；`<steps>` 可嵌套于 `<step>` 内（子步骤的新写法）
- [steps-informal](https://dita-lang.org/dita-techcomm/langref/technicalcontent/steps-informal) — 从 `<section>` 专门化而来，定义在 task 模块
- [titlealt](https://dita-lang.org/dita/langref/base/titlealt) — `@title-role` 五个取值与语义、多值、可出现位置、topicref 优先的合并规则、alternative-titles 域简写元素
- [topichead](https://dita-lang.org/dita/langref/base/topichead) — 2.0 中仍存在；导航标题改用 `<topicmeta><navtitle>`
- [mapref](https://dita-lang.org/dita/langref/base/mapref)（核对于 2026-08-15）— 等价于 `@format="ditamap"` 的 `<topicref>`；处理期望为**"被引 map 的层级在引用位置并入容器 map"**，示例显示被引 map 的 `<topicref>` 子项**替换** `<mapref>` 元素本身——即被引 map 的 `<title>` 不产生导航节点（本篇「`<mapref>`：被引 map 的标题不产生导航节点」一节的依据）
- [keytext](https://dita-lang.org/dita/langref/base/keytext) — map 示例中的 `<keytext>` 用法
- [specializations 属性规则与语法（架构规范）](https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute) — 语法 `'@',(props|base),('/',attname)+`；仅记录属性专门化，不含元素域；值由 doctype shell 默认注入
- [迁移到 DITA 2.0](https://dita-lang.org/2.0/dita/non-normative/information-about-migrating-to-dita-2-0) — 确认 `<substeps>`、`<titlealts>`、`@navtitle`、`<topicset>` 等已移除，避免写入 1.x 结构

**未逐页核对，来自通用 DITA 实践**

- bookmap 示例结构、reltable 的维护成本论证
- base 包的结构模块清单（`<topic>` / `<map>` / `<subjectScheme>`）—— 未找到与技术内容包等价的分组页面逐条核对
- `<glossentry>` 与 `<troubleshooting>` 的内部结构要点、"类型 vs 主题正交"的示例表、三种失败模式 —— 判断性内容
- 内置类型自身即专门化产物（`<concept class="- topic/topic concept/concept ">`）—— `@class` 值为按规则推导，未从语法文件逐字核对
- topicref 属性表中未在上述页面出现的条目（`@processing-role` 等的取值；`@collection-type` 与 `@linking` 已在 [11](11-processing-model.md) 按 topicref 语言参考核对）
- 与 DocBook / Markdown / AsciiDoc 的对比表（判断性内容）
- 3.5 节的 CALS `<table>` 细节（`@morerows`、`@namest/@nameend`、`@rowheader`）、`<choicetable>` / `<properties>` 派生自 simpletable 的说法，及"默认用 simpletable"的判断
- 3.6 节 `<image>` 的属性细节（`@placement` / `@scale` / `@scalefit`）与空 `<alt>` 的无障碍建议；`<audio>` 未单独核对（结构与 `<video>` 同族）
