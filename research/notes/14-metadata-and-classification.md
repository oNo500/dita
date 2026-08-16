# 14 · 元数据与分类策略

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/conditional/metadata-two-kinds.dita` ← §1 + §5；`kb/topics/dita/conditional/dublin-core-mapping.dita` ← §2；`kb/topics/dita/conditional/metadata-placement.dita` ← §3；`kb/topics/dita/conditional/subjectscheme-taxonomy.dita` ← §4，与 [03](03-profiling-and-chunking.md) §4 合并为一篇；**§6 对 RAG / 检索的注记已并入 practice 簇**，由 `kb/topics/dita/practice/dita-rag-fit.dita`、`rag-parsed-content.dita`、`rag-chunking-metadata.dita` 三篇承接，其中检索元数据逐项对应的出处一节即本节内容的落点；不迁小节：无），本文冻结为调研档案，不再更新。

> 最佳实践层第一篇。[03](03-profiling-and-chunking.md) 讲了用于**过滤**的元数据（条件属性），本篇讲用于**分类与检索**的元数据：字段放哪、受控值怎么设计、分类树怎么画。
>
> 与前 13 篇不同，本篇的部分依据是**社区经验**而非规范条文 —— 观点种子来自 Scriptorium 的 [Death and tax-onomies](https://www.scriptorium.com/2026/06/death-and-tax-onomies-metadata-with-minimal-pain/)（见来源），机制部分仍按规范核对。

---

## 1. 先分清两类元数据

| | 用于过滤 | 用于分类与检索 |
|---|---|---|
| 回答 | "这段内容**进不进**这个交付物？" | "这个 topic **是关于什么的**？谁写的？多新？" |
| 机制 | 条件属性 + DITAVAL | prolog 元数据、`@subjectrefs`、`<data>` |
| 消费者 | 构建流水线 | 人（检索）、门户（分面导航）、治理（审计）、RAG |
| 笔记位置 | [03](03-profiling-and-chunking.md) | 本篇 |

同一个维度可能两用（`@audience` 既能过滤也能检索），但**设计时要先问它主要给谁消费** —— 这决定它落在哪个机制上（第 3 节）。

---

## 2. 别从空白页开始：Dublin Core 作为字段清单

Dublin Core（DCMI）的 15 个核心元素是图书馆界用了三十年的最小描述集，分三组：

- **内容**：Title、Subject、Description、Source、Language、Relation、Coverage
- **知识产权**：Creator、Publisher、Contributor、Rights
- **实例**：Date、Type、Format、Identifier

它的价值不在字段本身，在**方法**：设计元数据时逐个问"这 15 项里哪些对我们真的有消费者"，比从零头脑风暴可靠得多 —— 三十年的实践已经把"值得记什么"筛过一遍了。

### DC ↔ DITA 对应表

| Dublin Core | DITA 落点 | 备注 |
|---|---|---|
| Title | `<title>`；检索标题另有 `<titlealt title-role="search">` | |
| Creator | `<author type="creator">` | `@type` 的词汇与 DC 一致 |
| Contributor | `<author type="contributor">` | 同上 |
| Subject | `<keywords>` / `<category>`；受控分类走 `@subjectrefs`（第 4 节） | |
| Description | `<shortdesc>` / `<abstract>` | |
| Publisher | `<publisher>` | |
| Date | `<critdates>`（`<created>` / `<revised>`） | |
| Rights | `<copyright>` + `<permissions>` | |
| Source | `<source>` | |
| Language | `@xml:lang`（见 [13](13-translation-and-localization.md)） | |
| Identifier | `@id` + `<resourceid>` | |
| Relation | `<related-links>` / reltable | |
| **Type** | **信息类型本身**（concept / task / reference） | DITA 把它从"填字段"升级成了架构 |
| **Format** | 交付物是构建产物；引用上的 `@format` | 同上 —— 格式不是作者填的 |
| Coverage | 最弱的对应 —— 条件属性或 `<prodinfo>` | |

最后三行是理解 DITA 元数据的关键：**DC 里要人工填写的 Type / Format / Relation，DITA 吸收成了架构机制**（类型化、构建、reltable）。所以在 DITA 里做元数据设计，实际要设计的字段比 15 个少 —— 架构已经替你管了一半。

> ⚠️ 关于"DITA 元数据显式仿照 Dublin Core"的流行说法：`@type` 词汇的对应肉眼可见，但 **2.0 规范原文并未提及 Dublin Core**。对应表是本篇的分析，不是规范条文。

---

## 3. 元数据放在哪：五种机制与取舍

分类与检索用的元数据在 DITA 里有五个可放的位置，能力差异很大：

| 位置 | 受控值校验 | DITAVAL 过滤 | 沿 map 级联 | 处理器默认行为 | 适用 |
|---|---|---|---|---|---|
| **条件属性**（`@props` 专门化） | ✅ subjectScheme | ✅ | ✅（[11](11-processing-model.md) 级联清单） | 过滤 / flag | **影响交付物内容**的维度 |
| **`@subjectrefs`** | ✅ 引用 subjectScheme 主题 | ❌（分类不是条件） | ✅（在级联清单里） | 分类关联，不改内容 | **纯分类**：主题标引、分面导航 |
| **prolog 元数据元素** | 部分（`<category>` 等可约束） | ❌ | map `<topicmeta>` 可下沉到 topic | 标准语义，工具认得 | 作者、日期、版权 —— DC 表里的字段 |
| **`<data>` 专门化** | 自定义 schema | ❌ | ❌ | **默认不渲染**（SHOULD NOT render） | 结构化自定义属性（可嵌套、可挂在 100+ 种容器上） |
| **`<othermeta>`** | ❌ 裸 name/content 对 | ❌ | 随 metadata 下沉 | 无 | **权宜位置** —— 快速试验，成熟后迁走 |

判断顺序：

1. 这个维度**会改变交付物内容**吗？→ 条件属性（并进 subjectScheme）
2. 只是**给内容归类**？→ `@subjectrefs` 绑定 subjectScheme 主题
3. 是 DC 表里的**标准描述字段**？→ prolog 元数据元素
4. 是**自有结构**的属性（传感器参数、合规编号）？→ `<data>` 专门化
5. 都拿不准、先跑起来再说？→ `<othermeta>`，**并给它设个迁移期限** —— 它没有任何校验，值放久了必然失控

一个常见错误是全部塞进 `@otherprops` 或 `<othermeta>`："能用"半年，然后拼写变体开始繁殖，没有任何机制报错。**没有校验的位置，只配放草稿。**

---

## 4. subjectScheme：DITA 内建的分类法机制

[03](03-profiling-and-chunking.md) 把 subjectScheme 当"条件值的拼写检查"介绍，那只是它的最小用法。它实际是一个**受控词表 + 分类树**机制，规范语义如下（本节已逐页核对）：

### 机制要点

- subjectScheme 是一种 **map 类型**，用 keydef 机制定义主题：每个 `<subjectdef>` 既是一个 key，也是一个分类节点，嵌套即成树
- **subjectScheme 里的 key 引用不产生链接、不产生变量文本** —— 同一套 key 基础设施，在这里的语义是"绑定与关联"。这是复用机制的又一次架构复用
- `<enumerationdef>` 把一棵子树绑定到一个属性：`<attributedef>` 指定属性，`<subjectdef keyref>` 指定值集，`<defaultSubject>` 可选地给默认值
- **被绑定的容器节点本身不是合法值** —— 绑定 `users` 这棵树时，合法值是它的子孙，不含 `users` 自己
- **空枚举 = 禁用属性**：绑定一个没有子节点的 `<subjectdef>`，该属性就没有任何合法值 —— 这是"团队禁止使用 `@otherprops`"的 schema 级实现
- `<defaultSubject>` 提供的默认值就是 [11](11-processing-model.md) 五级优先级的**第 4 级**（显式值、语法默认、级联值之后；处理器默认之前）

### 层级的过滤语义：向上查找

树的形状直接决定过滤行为。处理器判定一个属性值时：

> 先找**该值本身**的 DITAVAL 规则；没有，就**沿树向上**逐级找祖先的规则，用最近命中的那条。

所以内容标了 `ubuntu`、DITAVAL 只写了 `exclude linux`，`ubuntu` 被排除；flag 同理向下传。三条配套的处理器期望（都是 SHOULD）：感知层级、校验属性值在受控集内、**检查 DITAVAL 里列的值确实被 subjectScheme 绑定过**（连过滤条件本身的拼写错误都能拦住）。

### 设计推论：分类树按过滤语义画，不按学科正确画

这是本篇最重要的一条实践结论，直接从向上查找规则推出：

> **两个主题该不该是父子，唯一的判据是："排除父亲时，儿子是否也该消失？"**

学科上正确的从属关系，过滤语义可能是错的。比如"云平台"下挂 "AWS" 和 "本地部署迁移指南"——分类学上说得通，但排除"云平台"时迁移指南多半不该消失。它们不该是父子，哪怕目录里想把它们放在一起（目录是 map 的事，不是分类树的事）。

**一棵树管一个正交维度**（[12](12-philosophy-and-principles.md) 推论 5 在分类法上的对应物）：平台一棵、受众一棵、产品一棵。想把多个维度编进一棵树的冲动，就是组合爆炸的开始。

---

## 5. 设计流程

综合 Scriptorium 的建议与上面的机制约束，可操作的流程：

**① 盘点消费者，不是盘点字段。** 列出谁会消费元数据：构建（过滤）、站内检索、门户分面、合规审计、RAG 检索。**没有消费者的字段不建** —— 多建的每一个字段都是长期的维护负担。

**② 用 DC 15 元素当检查单**过一遍第 2 节的对应表：哪些字段有消费者、哪些已被架构吸收、哪些确实要建。

**③ 对每个字段问三个问题**：谁消费？谁维护？**填错了谁会发现？** 第三问最有效 —— 没有任何环节会发现错值的字段，数据必然失真，等于没建。

**④ 系统能得出的信息不让人填。** 日期、版本、修订人这类元数据让 CCMS / git / 构建管道注入 —— 手填的 `<revised>` 永远是错的。git 仓库里 `git log` 就是比 `<critdates>` 更真的 critdates；发布管道可以在构建时把它写回元数据。

**⑤ 受控值一步到位进 subjectScheme。** 命名冲突（"setup" vs "installation"）在建树时一次定版；树按第 4 节的过滤语义画；每个维度一棵。subjectScheme 就是一个 map 文件 —— **进 git、有 diff、可评审**，词表治理等于代码评审。

**⑥ 从小开始，长期维护。** 首版只建有消费者的三五个字段 + 一两棵树。分类法是随内容长出来的，不是启动时一次设计完的 —— 但**每次演进走版本化的 subjectScheme 变更**，不走"作者自由发明新值"。

---

## 6. 对 RAG / 检索的注记

[08](08-practical-advice.md) 看好的方向在这里落地：topic 是自足的检索单元（[12](12-philosophy-and-principles.md) 推论 2），本篇的元数据就是它的检索元数据 —— `@class` 给出类型、`@subjectrefs` 给出主题分面、prolog 给出作者与时效、subjectScheme 树本身就是现成的分面层级。做 RAG 分块时这些全部现成可用，前提是第 5 节的流程真的执行了 —— **失真的元数据喂给检索器，比没有元数据更糟**。

---

→ 回到 [README](../README.md)

---

## 来源

**已逐页核对（2026-08）**

- [subjectScheme map（架构规范）](https://dita-lang.org/2.0/dita/archspec/base/subjectschema) — 受控值与主题定义经 keydef 机制组织；分类与子分类构成树；**subjectScheme 内解析的 key 引用不产生变量文本与链接，语义为绑定/关联**
- [绑定受控值到属性](https://dita-lang.org/2.0/dita/archspec/base/binding-controlled-values-to-attribute) — `<enumerationdef>` / `<attributedef>` / `<subjectdef>` / `<defaultSubject>` 的分工；**容器节点本身不是合法值**；**空枚举使属性无合法值**；处理器 SHOULD 按受控集校验；受控值默认在显式值、语法默认、级联值之后、处理器默认之前求值（与 [11](11-processing-model.md) 五级优先级第 4 级吻合）
- [处理受控属性值](https://dita-lang.org/2.0/dita/archspec/base/processing-controlled-attribute-values) — **向上查找算法**（先精确匹配，无则沿层级上溯至命中）；排除与 flag 沿层级向下继承（原例：exclude therapist 则 novice-therapist / expert-therapist 默认被排除）；三条 SHOULD（感知层级 / 校验属性值 / 校验 DITAVAL 所列值已被绑定）
- [subject scheme maps 及其使用（章节页）](https://dita-lang.org/2.0/dita/archspec/base/subject-scheme-maps-and-usage) — 六个子页清单；`@subjectrefs` 以空格分隔引用主题 key
- [author](https://dita-lang.org/dita/langref/base/author) — `@type` 取值 creator / contributor；**页面未提及 Dublin Core**（第 2 节警示框的依据）
- [data](https://dita-lang.org/dita/langref/base/data) — 用于属性/元数据；`@name` `@value` `@datatype`；可出现于 prolog / metadata / topicmeta 及 100+ 容器；**处理器默认 SHOULD NOT 渲染**；显式定位为专门化基类；"仅用于属性，勿用于正文内容流"
- [othermeta](https://dita-lang.org/dita/langref/base/othermeta) — `@name` / `@content` 必填的键值对；空内容模型；限 `<metadata>` 内；2.0 仍存在；无需专门化即可自定义元数据

**观点来源（社区，非规范）**

- [Scriptorium: Death and tax-onomies — metadata with minimal pain](https://www.scriptorium.com/2026/06/death-and-tax-onomies-metadata-with-minimal-pain/)（Allison Beatty，2026-06）— "别从空白页开始，映射到 Dublin Core"；受控词表一次定版；战略性专门化；**避免元数据膨胀、系统生成的数据交给 CCMS 管理**。其中"DITA 元数据显式仿照 Dublin Core 建模"一句在 2.0 规范中未找到背书，本篇按对应表 + 警示框处理

**属于本篇的分析与判断**

- DC ↔ DITA 对应表及"Type / Format / Relation 被架构吸收"的解读
- 五种放置机制的取舍表与判断顺序；`@subjectrefs` 的可用位置未逐条核对
- 第 4 节"分类树按过滤语义画"的设计推论（从已核对的向上查找规则推出，规范未如此表述）
- 第 5 节全部流程建议、第 6 节 RAG 注记
- Dublin Core 15 元素清单为 DCMI 标准的通识内容，未另行核对
