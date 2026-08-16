# 09 · 架构基础

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/architecture/extension-facilities.dita` ← §1，并入 §4 的 constraint 与 expansion 对称一节与 §6 的设施选型表；`kb/topics/dita/architecture/doctype-shell.dita` ← §2；`kb/topics/dita/architecture/vocabulary-modules.dita` ← §3；`kb/topics/dita/architecture/conformance.dita` ← §5；不迁小节：无——§4 与 §6 未单独立篇，理由是 §4 讲的是 element-type configuration 这一个 facility 的两个方向、与 §1 同节点，§6 的成本维度已由 `kb/topics/dita/practice/customization-cost-ladder.dita` 承载，只余「哪件事归哪个 facility」的对应并入 §1 对应 topic），本文冻结为调研档案，不再更新。

> 本篇补充 [05-specialization.md](05-specialization.md) 未涉及的一层：DITA 的语法文件如何组织成模块，以及 shell 如何把模块装配成一个可用的文档类型。约束、扩展和默认属性注入都建立在这一层上。

---

## 1. 三大扩展设施

规范的正式框架是**三个**扩展设施，外加一个补充过程：

```
DITA 扩展设施
├── 文档类型配置（document-type configuration）  ← 选用哪些模块
├── 专门化（specialization）                    ← 造新元素类型
└── 元素类型配置（element-type configuration）   ← 改单个元素
    ├── 约束（constraint）    收紧内容模型与属性表
    └── 扩展（expansion）     放宽内容模型与属性表

泛化（generalization）—— 补充过程，把专门化内容还原为基类标记
```

> **常见的层级错误**：把"结构化 / 域 / 属性专门化 + 约束"摊平并列成"四种扩展手段"。前三者都是**专门化**这一个设施下的不同种类，约束则属于**元素类型配置** —— 正确的框架就是上面三个设施。

三者的分工：

| 设施 | 动什么 | 改词汇模块吗 |
|---|---|---|
| 文档类型配置 | 选哪些模块进来、配默认值 | **不改** |
| 专门化 | 造新元素类型，打包成新的词汇模块 | 不改已有的，新增 |
| 元素类型配置 | 改某个元素的内容模型 / 属性表 | **不改** |

**共同点：都不修改已有的词汇模块。** 这是 DITA 可维护性的根本 —— OASIS 发新版时你的定制不会被覆盖，因为你从没动过它们的文件。

---

## 2. 文档类型外壳（Document-type shell）

### 它是什么

一个 XML 语法文件，**规定一份 DITA 文档里允许出现哪些元素和属性**。它把结构模块、域模块、元素配置模块**装配**在一起。

> 与"文档类型"的区别：**文档类型**是一组模块的组合（抽象），**外壳**是按规范的规则与设计模式实现该组合的 DTD/RNG 声明（具体）。因此**两个外壳若引入同一组模块，实现的是同一个文档类型** —— 这就是下文"等价性"要判定的事。定义原文见 [00 第 5 节](00-roles-and-boundaries.md)。

你写 `<!DOCTYPE topic PUBLIC "-//OASIS//DTD DITA 2.0 Topic//EN" "topic.dtd">` 时，`topic.dtd` 就是一个 shell。

### 能做什么

- **选择要引入哪些结构模块和域模块**
- **应用元素配置模块**（约束与扩展）
- **指定 topic 的嵌套规则**
- **为引入的元素配置默认属性值** ← `@class` 和 `@specializations` 就是这么注入的

### 不能做什么

**shell 绝不能自己定义元素类型。** 它只做聚合与配置，不改动被引入模块的定义。

### 为什么这条约束如此重要

理解了这条，很多现象就自洽了：

| 现象 | 解释 |
|---|---|
| `@class` / `@specializations` 不在源文件里 | 它们是 shell 配置的默认属性值，由校验器注入 |
| 不做 DTD/RNG 校验就拿不到这两个属性 | 同上 —— 见 [07](07-programmatic-processing.md) |
| 同一批 topic 用不同 shell 校验，结果不同 | shell 决定了允许什么 |
| 升级 DITA 版本时定制不丢 | 你的定制在 shell 和自有模块里，OASIS 模块原封不动 |
| 两个文档能否互相 conref | 取决于双方 shell 装配了什么 |

### shell 的三组规则

规范为 shell 单列了三节：

- **Rules for document-type shells** —— 构造规则
- **Equivalence of document-type shells** —— 两个 shell 何时算等价
- **Conformance of document-type shells** —— 什么算一致的 shell

"等价性"这一节在实践中容易被忽略，但它正是**判断两份文档能否安全交换内容**的依据。

---

## 3. 模块化与词汇模块

> **"模块化是 DITA 设计与实现的核心。它使专门化层级的复用与扩展成为可能。"**

DITA 的语法文件不是一整块，而是一堆**模块**，由 shell 挑选组装。

### 整体结构

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 ① 文档实例                                            install.dita
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   <task specializations="@props/platform @props/product">
     <step class="- topic/li task/step ">
       <cmd>点击 <uicontrol class="+ topic/ph ui-d/uicontrol ">保存</uicontrol></cmd>

                              ▲
                              │  校验时注入 @class / @specializations
                              │  （值来自 schema 默认声明，作者通常不写）
                              │
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 ② 文档类型外壳（doctype shell）                            task.rng
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   ❌ 不定义任何元素        ✅ 只做三件事：挑模块、配默认值、定嵌套规则

        │装配            │装配            │装配           │应用
        ▼                ▼                ▼               ▼
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 ③ 模块层                        每个元素/属性「恰好」在一个模块里声明
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

   ┌── 词汇模块（声明新元素 / 新属性）─────────────────────┐
   │                                                       │
   │  结构模块          topicMod.rng   taskMod.rng         │
   │  *Mod.rng          定义一个 topic 或 map 类型          │
   │                    模块名 ≈ 根元素名                   │
   │                                                       │
   │  元素域模块        uiDomain.rng   programmingDomain   │
   │  *Domain.rng       定义跨类型可用的元素                │
   │                    有短名：ui-d、sw-d、pr-d           │
   │                    可含多个元素                        │
   │                                                       │
   │  属性域模块        platformAttDomain.rng              │
   │  *AttDomain.rng    定义 @props / @base 的专门化属性    │
   │                    恰好一个属性，所以一个模块一个文件   │
   │                                                       │
   └───────────────────────────────────────────────────────┘

   ┌── 元素配置模块（不声明新元素，只改已有元素）───────────┐
   │                                                       │
   │  约束模块          strictTaskbodyConstraintMod.rng    │
   │  *ConstraintMod    收紧内容模型，strict task 由它实现   │
   │                                                       │
   │  扩展模块          放宽内容模型，且只对特定元素生效     │
   │                                                       │
   └───────────────────────────────────────────────────────┘
```

模块负责声明元素和属性，shell 负责选取和配置，文档实例则携带 shell 注入的 `@class` 与 `@specializations`。有了这两个属性，处理器不需要拿到 schema 也能判断每个元素的来源。

### 元素与模块的对应关系

> **每一个 DITA 元素类型或属性，恰好在一个词汇模块中声明。**

"恰好一个"是这里的关键：没有重复声明，因此任何元素都能被唯一地追溯到来源模块。`@class` 中"模块名/元素名"这种写法之所以成立，前提就是这条规则。

### 三种词汇模块

| 种类 | 定义什么 | 数量约束 |
|---|---|---|
| **结构模块**<br>structural module | 一个顶层的 map 或 topic 类型 | —— |
| **元素域模块**<br>element domain module | 一个或多个可集成进 map/topic 的**专门化元素** | 一个模块可含多个元素 |
| **属性域模块**<br>attribute domain module | `@base` 或 `@props` 的专门化属性 | **恰好一个属性** |

补充细节：

- **结构模块**的模块名通常**就是根元素名**（`conceptMod` 定义 `<concept>`）。它可以在内部定义下级 topic 类型，但规范建议大多数专门化单独声明
- **元素域模块**有一个**短名**（`hi-d`、`sw-d`、`ui-d`）。短名必须是合法的 XML name token，且在其专门化层级内唯一
- 域元素的派生有方向约束：**给 topic 用的域元素必须最终派生自 topic 模块的元素；给 map 用的必须派生自 map 模块的元素**。map 专有元素不能出现在 topic 里

### 第四种：元素配置模块（不是词汇模块）

约束模块与扩展模块**不声明新元素**，只修改已有元素的内容模型或属性表。它们属于**元素类型配置**（见第 4 节），不属于词汇模块。

### 从文件名判断模块类型

看 v2.0-beta03 的语法文件目录，命名规律直接暴露了模块种类：

| 文件名模式 | 是什么 | 例子 |
|---|---|---|
| `*Mod.rng` | **词汇模块**（结构模块 / 域的实现模块） | `topicMod` `conceptMod` `taskMod` `referenceMod` |
| `*Domain.rng` | **元素域模块** | `uiDomain` `programmingDomain` `highlightDomain` |
| `*AttDomain.rng` | **属性域模块** | `audienceAttDomain` `platformAttDomain` |
| `*ConstraintMod.rng` | **约束模块**（元素配置，非词汇模块） | `strictTaskbodyConstraintMod` |
| **无后缀** | **文档类型外壳** | `concept.rng` `task.rng` `generalTask.rng` `basetopic.rng` |

拿到一套不熟悉的 DITA 语法文件时，文件名可以直接说明每个文件属于哪一类。

### 一个容易被忽略的事实：条件属性是"装配"进来的

`@audience` `@platform` `@product` `@deliveryTarget` `@otherprops` —— 这五个**不是 DITA 内建的**。

它们是 base 包里五个独立的**属性域模块**：

```
audienceAttDomain.rng
platformAttDomain.rng
productAttDomain.rng
deliveryTargetAttDomain.rng
otherpropsAttDomain.rng
```

每个模块定义**恰好一个** `@props` 的专门化属性。这就是为什么它们是五个独立文件。

推论：

1. **你可以做一个不含 `@platform` 的 shell** —— 用不到就不装
2. **你自己的 `@region` 和它们地位完全相同** —— 同样是"`@props` 的一个专门化"，没有等级差别
3. 所谓"DITA 内建的条件维度"，其实只是**OASIS 替你预装了五个常用的属性域**

### `@class` 与 `@specializations` 分别指向什么

这两个属性都是指向模块的引用。理解了模块层之后，它们的写法就可以推出来。

```
                    文档实例里的两处标记
                             │
        ┌────────────────────┴────────────────────────┐
        │                                             │
        ▼                                             ▼

  <uicontrol class="+ topic/ph ui-d/uicontrol ">   <task specializations=
                       │        │                    "@props/platform
                       │        │                     @props/product">
        ┌──────────────┘        └────────┐                  │      │
        ▼                                ▼                  │      │
   ┌─────────────┐              ┌─────────────────┐         │      │
   │ topicMod    │              │ uiDomain.rng    │         │      │
   │ 结构模块     │              │ 元素域模块       │         │      │
   │ 定义 <ph>   │              │ 短名 = ui-d     │         │      │
   └─────────────┘              │ 定义 <uicontrol>│         │      │
                                └─────────────────┘         │      │
                                                            ▼      ▼
                                          ┌──────────────────┐ ┌──────────────────┐
                                          │platformAttDomain │ │productAttDomain  │
                                          │ 属性域模块        │ │ 属性域模块        │
                                          │ 定义 @platform   │ │ 定义 @product    │
                                          └──────────────────┘ └──────────────────┘

   ── @class 指向「元素来自哪些模块」        ── @specializations 指向「装配了哪些属性域」
      写在每个元素上（分散）                     写在根元素上（集中）
```

**`@class` 里的模块名，就是模块的（短）名。** 规范原话：元素域模块的名称（或短名）用于在 `@class` 属性值中标识该模块。

**`@specializations` 列的，就是 shell 装配了哪些属性域。** 对照 base 包的文件列表 —— 那些 token 精确对应 `platformAttDomain.rng`、`productAttDomain.rng` 等属性域模块文件。**它不是抽象声明，它就是装配清单。**

### 为什么 `@specializations` 不记录元素域

因为**元素域的信息已经在 `@class` 里了** —— 每个域元素自己就带着 `ui-d/uicontrol` 这样的派生链。不需要在根元素上再列一遍。

而属性不同：属性值就是个字符串，`region="cn"` 本身看不出它派生自 `@props`。所以必须在根元素上单独声明这份信息。

**两个属性各司其职**：`@class` 管元素派生链（分散在每个元素上），`@specializations` 管属性派生链（集中在根元素上）。

---

## 4. 约束 vs 扩展

两者是**对称**的，同属元素类型配置：

| | 约束（constraint） | 扩展（expansion） |
|---|---|---|
| 方向 | **收紧** —— 移除元素和属性 | **放宽** —— 添加元素和属性 |
| 用途 | 禁掉团队不该用的、强制必填 | 给特定元素加专门化元素/属性，**但不全局可用** |
| 典型 | 禁用 `<draft-comment>`、强制 `<shortdesc>` | 只让 `<step>` 能用某个自定义元素，别处不能 |

### 扩展模块的价值

扩展模块是三大设施里最容易被忽略的一个，但它填补的是一个真实空白：

**问题**：你做了一个域专门化 `<sensor-id>`，从 `<ph>` 派生。一旦装进 shell，它在**所有** `<ph>` 能出现的地方都可用 —— 包括你并不希望它出现的地方。

**扩展模块的答案**：把专门化元素或属性**只加到特定元素类型上**，不让它全局可用。

规范原话是：让架构师"在特定元素类型中引入专门化的属性或元素，而不使这些专门化的属性或元素全局可用"。

### 扩展模块的核心规则

> **泛化之后，受扩展模块影响的元素，其内容模型必须与该元素原本的内容模型一致。**

这条保证了扩展不会破坏可交换性 —— 泛化回去还是合法的基类内容。

**判断**：如果你正打算做域专门化，但担心"这个元素到处都能用太乱"，那你要的是**扩展模块**，不是更精细的专门化。

---

## 5. 一致性（Conformance）

### 一致的处理器

**支持某个特性，就必须完整遵守该特性的全部规则。** 只实现部分特性仍算一致 —— 只要**已实现的部分**完全合规。

规范列出的核心特性：

1. 基于 `@class` 的专门化处理
2. key 引用解析（`@keyref`）
3. 直接 URI 内容复用（conref）
4. 间接 key 内容复用（conkeyref）
5. conref 范围（conrefend）
6. 内容推送（conaction）
7. 基于 DITAVAL 的条件处理
8. 分支过滤
9. `@chunk` 处理

外加对 13 个特定元素（`<image>` `<pre>` `<title>` `<related-links>` 等）的规范性渲染规则。

### 一致的文档

五个条件：

1. 用 OASIS shell 的文档，必须通过语法文件与断言校验
2. 用带**约束**的自定义 shell，必须符合约束规则
3. 用带**扩展模块**的自定义 shell，必须符合扩展模块规则
4. 专门化元素必须符合专门化规则与 `@class` 语法
5. 专门化属性必须符合属性专门化规则

### 为什么这一节对二次开发重要

**"部分实现也算一致"**这条，直接解释了生态里的现象：

- 不同工具行为不同，**未必是 bug** —— 可能一方根本没实现那个特性，而这是规范允许的
- 你自己写处理器时，**可以只实现子集**，但已实现的部分不能打折
- 评估工具（尤其 CCMS）时，正确的问题不是"支不支持 DITA"，而是"**支持上面 9 项中的哪几项**"

这是把"规范支持 ≠ 工具链支持"从模糊感受变成可核查清单的地方。

---

## 6. 对二次开发的实际影响

按你要做的事，对应到哪个设施：

| 你想做的 | 用哪个设施 | 成本 |
|---|---|---|
| 只改外观 | 都不用，`@outputclass` + CSS | 极低 |
| 禁掉团队不该用的元素 | **约束** | 低 |
| 给特定元素加自定义标记，但不全局可用 | **扩展模块** | 中 |
| 造一个到处能用的语义标记 | 域专门化 | 中 |
| 加一个条件化维度 | 属性专门化 | 中 |
| 造一整套新的内容骨架 | 结构化专门化 | 高 |
| 上面任何一项落地 | **都要改 shell** | —— |

**最后一行是重点**：无论走哪条路，最终都要有一个 shell 把它装配进来。**shell 是所有定制的汇合点**，先把它搞清楚，其余才有依托。

---

→ 下一步：[10-addressing-and-key-space.md](10-addressing-and-key-space.md)

---

## 来源

**已逐页核对（2026-08）**

- [DITA 扩展设施概览](https://dita-lang.org/2.0/dita/archspec/base/ditaspecialization) — 规范明确为**三个**扩展设施：文档类型配置、专门化、元素类型配置；元素类型配置含约束与扩展两个子类；泛化为补充过程。本篇对 05 的框架纠正即基于此页
- [文档类型配置](https://dita-lang.org/2.0/dita/archspec/base/configuration) — shell 的定义（"规定一份 DITA 文档中允许的元素和属性"、"集成结构模块、域模块和元素配置模块"）；能做（选模块、应用约束与扩展、指定 topic 嵌套规则、配置默认属性值）与不能做（不得直接定义元素类型、无需修改词汇模块）；四个子页（概览 / 规则 / 等价性 / 一致性）
- [扩展模块](https://dita-lang.org/2.0/dita/archspec/base/expansion-modules) — 与约束互为反向；作用是"在特定元素类型中引入专门化的属性或元素，而不使其全局可用"；可扩展内容模型与属性表；核心规则"泛化后受影响元素的内容模型必须与该元素原本的内容模型一致"
- [一致性](https://dita-lang.org/2.0/dita/conformance/conformance) — "任何实现若支持某特性，MUST 遵守描述该特性的章节中的全部规则"；9 项核心特性清单；13 个元素的规范性渲染规则；部分实现仍算一致；一致文档的 5 个条件
- [specializations 属性规则与语法](https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute) — 架构规范 "Configuration and specialization" 子树结构，以及各子页 URL

- [模块化](https://dita-lang.org/2.0/dita/archspec/base/specialization-modularization) — **"模块化是 DITA 设计与实现的核心。它使专门化层级的复用与扩展成为可能。"**；"DITA XML 语法文件是一组模块文件，声明每个专门化所需的标记与实体；文档类型外壳则集成特定创作与发布场景所需的模块"
- [词汇模块](https://dita-lang.org/2.0/dita/archspec/base/specialization-vocabulary-modules) — **"一个 DITA 元素类型或属性恰好在一个词汇模块中声明"**；三种词汇模块的定义（结构模块 / 元素域模块 / 属性域模块）；结构模块名通常等于根元素名；元素域模块有短名（hi-d、sw-d），**"元素域模块的名称或短名用于在 `@class` 属性值中标识该模块"**，短名须为合法 XML name token 且在其专门化层级内唯一；**属性域模块"定义恰好一个 `@base` 或 `@props` 的专门化"**；域元素的派生方向约束（topic 用的须派生自 topic 模块元素，map 用的须派生自 map 模块元素；map 专有元素不能出现在 topic 中）
- **v2.0-beta03 语法文件目录**（经 GitHub API 直接核对）—— 第 3 节的文件命名对照表、五个属性域模块（`audienceAttDomain` / `platformAttDomain` / `productAttDomain` / `deliveryTargetAttDomain` / `otherpropsAttDomain`）的存在，以及 `@specializations` 三个 token 与三个属性域文件的对应关系

**未逐页核对**

- **约束模块**（`constraints`）的详细规则 —— 仅从扩展模块页的对比描述中获得，未单独核对
- shell 三组规则（构造规则 / 等价性 / 一致性）的**具体条文**未展开，本篇仅列出其存在与用途
- 第 6 节的设施选型表为判断性内容