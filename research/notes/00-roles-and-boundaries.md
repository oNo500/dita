# 00 · 角色与边界

DITA 把不同的人分成不同角色，规范、工具和文档各自面向不同角色。搞清自己是哪个角色，能省掉读错材料的时间。

> 说明：这些角色在规范里**大量使用但基本没有正式定义**。术语章定义的 10 个术语全部是关于文档与语法构件的（见第 5 节），角色中只有 `user agent` 有定义条目。下面的角色划分是从规范正文的实际用法归纳的。

---

## 1. 五种角色

| 角色 | 职责 | 产出物 |
|---|---|---|
| **作者**<br>author | 写内容 | topic、map |
| **信息架构师**<br>information architect | 设计信息架构 | root map、subject scheme（受控值）、新文档类型的规划 |
| **DITA 架构师**<br>DITA architect | 定义词汇与文档类型 | 词汇模块（专门化）、约束模块、扩展模块、文档类型外壳 |
| **实现者**<br>implementer | 开发工具 | 处理器、编辑器、CCMS、插件 |
| **处理器**<br>DITA processor | 处理 DITA 的软件本身 | 输出、校验结果 |

两点补充：

- 作者细分出 **map 作者**，规范里单独出现（"map 作者使用标题来标注 map 和 map 结构"）
- 信息架构师与 DITA 架构师在小团队常由同一人担任。规范里两者的侧重不同：前者偏内容组织（root map、受控值），后者偏语法定义（模块、外壳）

规范在描述内容生产链条时用的是这个组合："信息架构师、写作者和发布者可以用它们来规划、开发和交付内容。"

---

## 2. 规范是写给谁的

规范里明写了自己的读者：

> **规范是为 DITA 标准的实现者而写的，包括工具开发者和开发专门化的 XML 架构师。**

**规范不是写给作者的。** 作者应该读的是所在组织的写作指南。

这解释了规范的行文方式：通篇是"处理器 MUST/SHOULD……"，而不是"你应该这样写"。拿规范当写作教材会很痛苦，因为它根本不在回答作者的问题。

---

## 3. 各角色该读什么

| 角色 | 规范 | 本笔记 |
|---|---|---|
| **作者** | 不必读。需要时查语言参考（元素字典） | [01](01-core-model.md) [02](02-reuse.md) [03](03-profiling-and-chunking.md) |
| **信息架构师** | 架构规范的 map、寻址、条件处理部分 | [01](01-core-model.md)–[03](03-profiling-and-chunking.md)、[10](10-addressing-and-key-space.md)、[08](08-practical-advice.md) |
| **DITA 架构师** | 架构规范的"配置与专门化"整章、一致性 | [09](09-architecture-foundations.md) [05](05-specialization.md) [12](12-philosophy-and-principles.md) |
| **实现者** | 架构规范的处理章、一致性、RFC-2119 语句汇总 | [09](09-architecture-foundations.md) [10](10-addressing-and-key-space.md) [11](11-processing-model.md) [07](07-programmatic-processing.md) |
| **工具链维护** | DITA-OT 文档而非规范 | [04](04-toolchain-and-build.md) [06](06-dita-ot-plugins.md) |

---

## 4. 处理器：一个尚未定义的角色

`processor` 是规范里出现最频繁的角色词（约 475 处），几乎每条规范性要求都以它为主语。但**它至今没有正式定义**。

规范源码（v2.0-beta03）的术语章里留着编辑批注：

> 需要一个 "DITA Processor" 的定义。按规范目前的用法，它涵盖任何以任何方式处理 DITA 的工具，而不只是把 DITA 当作源的渲染工具。

批注中给出的暂定范围：

- **算**：会内联解析 conref 或 key 的 DITA 编辑器；把 DITA 渲染为其他格式的处理器；处理 DITA 文档间关系的 CCMS
- **不算**：纯文本编辑器 —— 它不需要求值任何 DITA 特性

批注标注为 `TO RESOLVE 11 May 2026`，正式定义尚未落地。

实践含义：判断一个工具是否受规范约束，看它**是否求值 DITA 特性**，而不是看它是否声称支持 DITA。

---

## 5. 规范定义的基本术语

术语章共定义 10 个术语，全部关于文档与语法构件。

### 组织原则：类型与实例

术语表建立在**类型（type）/ 实例（instance）** 的区分上。类型是 schema 层面的定义，实例是文档里的一次实际出现。

| 类型 | 实例 |
|---|---|
| DITA element type | DITA element |
| topic type | topic instance |
| map type | map instance |
| —— | structural type instance（topic instance 与 map instance 的统称） |

定义原文：

- **DITA element type** —— 要么是规范定义的基础元素类型之一，要么是其中某个的专门化
- **DITA element** —— 类型为某个 DITA 元素类型的 XML 元素实例。**DITA 元素必须带有 `@class` 属性，其值符合专门化层级的书写规则**
- **topic type** —— `topic` 或 `topic` 的专门化，**定义一个完整的内容单元**
- **map type** —— `map` 或 `map` 的专门化，定义一组 topic 实例之间的关系

`topic type` 定义里"完整的内容单元"这句，是 [12](12-philosophy-and-principles.md) 里"复用单元的边界由自足性决定"那条推论在规范中的对应表述。

### 文档相关的三个术语

**DITA document**

> 符合本规范要求的 XML 文档。根元素必须是以下之一：`map` 或 `map` 的专门化；`topic` 或 `topic` 的专门化；`dita`（不可被专门化，但允许一个文档包含多个平级 topic）。

**DITA document type**

> 一组唯一的结构模块、域模块和约束模块，它们合起来提供定义 DITA 文档结构的 XML 元素与属性声明。

**DITA document-type shell**

> 一组 DTD 或 RELAX NG 声明，按规范给出的规则与设计模式**实现**一个 DITA 文档类型。外壳引入并配置一个或多个结构模块、零个或多个域模块、零个或多个约束模块。除 `dita` 元素及其属性的可选声明外，外壳不直接声明任何元素或属性类型。

### 文档类型与文档类型外壳的区别

这两个词容易混用，规范里是分开的：

| | 是什么 | 形态 |
|---|---|---|
| **文档类型** | 一组模块的组合 | 抽象 |
| **文档类型外壳** | 实现那个组合的 DTD/RNG 文件 | 具体 |

推论：**两个外壳若引入了同一组模块，它们实现的是同一个文档类型。** 这正是 [09](09-architecture-foundations.md) 提到的"外壳的等价性"那一节所要判定的事，也是判断两份文档能否安全交换内容的依据。

### 这些术语分布在三层

每个术语存在于不同的地方，由不同的人产出：

| 层 | 术语 | 存在于 | 谁产出 |
|---|---|---|---|
| **语法层** | DITA element type、topic type、map type、document-type shell | `.rng` / `.dtd` 文件 | OASIS，或你们的 DITA 架构师 |
| **设计层** | DITA document type | **不是文件** —— 是"某个外壳引入了哪一组模块"这个事实 | 架构师的设计决定 |
| **文档层** | DITA document、DITA element、topic instance、map instance | `.dita` / `.ditamap` 文件 | 作者 |

这三层正好对应第 1 节的角色分工：**架构师产出语法层与设计层，作者产出文档层。**

### 每个术语是为了让某条规则能被写出来

规范性语句需要精确的主语。这些词不是为了分类而分类，每一个都在支撑某条具体规则：

| 术语 | 它使哪条规则得以表述 |
|---|---|
| **DITA element** | "DITA 元素必须带 `@class`，且值符合专门化层级的书写规则" —— 这条把 DITA 元素和普通 XML 元素区分开：一个 XML 元素只有带了合法 `@class` 才算 DITA 元素 |
| **DITA element type** | 专门化发生在**类型**层面而非实例层面。你专门化的是 `<li>` 这个类型，不是文档里某一个具体的 `<li>` |
| **topic type / map type** | "DITA 文档的根元素必须是 topic 类型或 map 类型之一" |
| **topic instance / map instance** | 一份文档可以包含**多个** topic 实例（嵌套 topic，或 `<dita>` 下的平级 topic）。讨论"这个 topic"时需要区分说的是类型还是某次出现 |
| **DITA document** | "什么算一份 DITA 文档"的判据 —— 即根元素的限制 |
| **DITA document type** | "两份文档是不是同一类"、"两个外壳是否等价" |
| **DITA document-type shell** | 把**设计**与**实现**分开 |

### 最后一条的实例：同一个文档类型，两个外壳

OASIS 的 v2.0-beta03 里同时提供：

```
doctypes/dtd/base/basetopic.dtd     ← DTD 实现
doctypes/rng/base/basetopic.rng     ← RNG 实现
```

两者引入的模块完全一致 —— `topicMod`，加上 alternativeTitles / emphasis / hazardStatement / highlight / utilities 五个元素域，以及 audience / deliveryTarget / otherprops / platform / product 五个属性域。

**它们是同一个文档类型的两种实现。**

如果没有"文档类型"与"外壳"的区分，这句话就说不出来，也就无法说明为什么两种语法可以互相替换、以及为什么"有出入时以 RNG 为准"这条规则有意义 —— 因为出入指的是**同一个文档类型的两个实现之间**的出入。

### 一处不一致

文档类型与外壳的定义都只提到**约束模块**，没有提**扩展模块**。扩展模块是 DITA 2.0 新增的设施，术语章可能尚未同步。beta 期间遇到这类出入，以功能章节（扩展模块那一节）为准。

---

## 6. schema 是什么

先说清这个对象，再说谁能定义它。

**schema 是一份机器可读的、对"某一类文档允许包含什么"的形式化描述**：哪些元素可以出现、按什么顺序、可以带哪些属性、属性的取值范围。校验器读入 schema 和一份文档，回答"这份文档是否有效"。

在 DITA 里，schema **不是一个文件，而是装配出来的一组文件**：

```
文档里的 doctype 声明
   └─ 指向一个文档类型外壳（如 task.rng）
        └─ 引入若干词汇模块（topicMod、taskMod、uiDomain、platformAttDomain…）
        └─ 应用若干元素配置模块（约束、扩展）
```

所以"这份文档的 schema"指的是**外壳加上它引入的全部模块**。这与多数 XML 词汇表不同——一般的词汇表只有一份 schema，DITA 提供的是一套**schema 装配系统**。

### DITA 2.0 提供两种语法

| 语法 | 状态 |
|---|---|
| **RELAX NG（RNG）** | **规范性版本** |
| **DTD** | 提供，但有出入时以 RNG 为准 |
| XSD | **2.0 的 OASIS 发行中不再提供**（可自行从 RNG 生成） |

规范的原话：

> XML 语法文件以 RELAX NG（RNG）和 XML 文档类型定义（DTD）两种形式提供。这些文件应当定义相同的 DITA 元素，**若存在出入，以 RELAX NG 语法为准**。

> 因此，标准 DITA 词汇表的 RELAX NG 定义是规范性版本。

OASIS 提供的 RNG 文件**刻意避开了无法翻译成 DTD 构造的 RELAX NG 特性**，以保证两种语法可以互相生成。你自己写模块时若打算生成 DTD/XSD，也要遵守这条限制。

### schema 在 DITA 里还多做一件事

除了校验，DITA 的 schema 还负责**注入默认属性值**——`@class` 和 `@specializations` 都是这么来的。

RNG 本身没有默认属性值的概念，DITA 用的是 **RELAX NG DTD Compatibility 规范**，它提供了定义默认属性值和内嵌文档的机制。[05](05-specialization.md) 示例里的 `dita:defaultValue` 就是这个机制。

由此得出一条对程序化处理很关键的结论：**不走 schema，就拿不到 `@class`**。详见 [07](07-programmatic-processing.md)。

### schema 文件从哪里来

三个不同的问题，分开答。

**谁发布**

| 来源 | 内容 |
|---|---|
| OASIS DITA TC | 标准模块与外壳。base 包在 `oasis-tcs/dita`，技术内容包在 `oasis-tcs/dita-techcomm` |
| 你所在组织的 DITA 架构师 | 自有的词汇模块、约束模块、扩展模块、外壳 |

**物理上怎么拿到**

- **随 DITA-OT 分发** —— 最常见。装完 DITA-OT 就有了一套完整的语法文件
- **随编辑器分发** —— Oxygen 等自带
- **从 OASIS 仓库下载** —— 需要特定 beta 版本时
- **放在你自己的仓库里** —— 自定义模块

**运行时怎么被找到**

这一层最容易出问题。文档里的 doctype 声明**不含可用的文件路径**：

```xml
<!DOCTYPE topic PUBLIC "-//OASIS//DTD DITA 2.0 Base Topic//EN" "basetopic.dtd">
                       └────────── 公共标识符 ──────────┘  └─ 系统标识符 ─┘
```

后面那个 `"basetopic.dtd"` 是相对路径，从你的文档所在目录几乎不可能解析成功。真正起作用的是前面的**公共标识符**，它经 **XML catalog** 映射到磁盘上的实际文件。

OASIS 的 catalog 里就是这样一条条映射（取自 v2.0-beta03 的 `doctypes/dtd/base/catalog.xml`）：

```xml
<public publicId="-//OASIS//DTD DITA 2.0 Base Topic//EN" uri="basetopic.dtd"/>
<public publicId="-//OASIS//DTD DITA 2.x Base Topic//EN" uri="basetopic.dtd"/>
```

注意 `2.0` 和 `2.x` 各有一条、指向同一个文件——这就是 [04](04-toolchain-and-build.md) 里说的"版本号可以写 `2.x` 或整个省略"在实现上的样子。

catalog 之间用 `<nextCatalog>` 串成链：

```
doctypes/catalog.xml
  ├── dtd/catalog.xml
  │     ├── dtd/base/catalog.xml           58 条 <public> 映射
  │     ├── dtd/ditaval/catalog.xml
  │     └── dtd/subjectScheme/catalog.xml
  └── rng/catalog.xml
        ├── rng/base/catalog.xml           40 条 <uri> + 40 条 <system>
        ├── rng/ditaval/catalog.xml
        └── rng/subjectScheme/catalog.xml
```

DTD 侧用 `<public>` 匹配公共标识符；RNG 侧用 `<uri>` 和 `<system>` 匹配 URN，形如：

```xml
<uri name="urn:pubid:oasis:names:tc:dita:rng:mapMod.rng:2.0" uri="mapMod.rng"/>
```

**这层间接的作用**：文档只声明"我是哪一类"，不声明"schema 文件在哪"。换一套语法文件、升级版本、改目录结构，都只需要改 catalog，文档一个字不动。这和 keyref 对内容做的事是同一种解耦。

**怎么告诉工具用哪份 catalog**

| 场景 | 方式 |
|---|---|
| DITA-OT | 自带 `catalog-dita.xml`，插件用 `dita.specialization.catalog.relative` 注册自有 schema（见 [06](06-dita-ot-plugins.md)） |
| 命令行（xmllint、jing 等） | 环境变量 `XML_CATALOG_FILES` |
| 编辑器 | 各自的 catalog 配置项 |

**没配 catalog 会怎样**：解析器要么报错找不到 schema，要么尝试联网抓取，要么静默跳过校验——最后一种最麻烦，因为文档看起来"通过"了，但 `@class` 一个都没注入。这是 [07](07-programmatic-processing.md) 里那个坑的根源。

### schema 管什么、不管什么

| 管 | 不管 |
|---|---|
| 元素能否出现在此处、顺序、基数 | conref 的目标文件是否存在 |
| 属性是否允许、取值范围 | key 能否解析 |
| 注入默认属性值 | 条件过滤后内容是否仍然合法 |
| —— | 业务规则（如"task 必须有 shortdesc"，需 Schematron 或约束模块） |

右列那些只有真正构建才会暴露。校验层次见 [04](04-toolchain-and-build.md)。

---

## 7. 谁可以定义 schema

**任何人都可以，不需要 OASIS 批准。** 这是 DITA 设计的基本前提之一。

但分两层：

| 层 | 谁 | 定义什么 |
|---|---|---|
| **标准层** | OASIS DITA TC 及其分委员会 | base 包与技术内容包的模块。这些不该改，也不需要改 |
| **组织层** | 你们的 DITA 架构师 | 自有的词汇模块、约束模块、扩展模块、文档类型外壳 |

组织层内部还要再分两类，规则完全不同：

| 做什么 | 规则 |
|---|---|
| **定义词汇模块**（新元素、新属性） | 必须遵守专门化规则与 `@class` 语法；内容模型必须是基类的子集 |
| **定义文档类型外壳**（选模块、配默认值） | **不得直接声明任何元素或属性类型** |

外壳那条限制，规范的原文是：

> 除了 `dita` 元素及其属性的可选声明之外，DITA 文档类型外壳不直接声明任何元素或属性类型。

所以"定义 schema"实际是两件事：**造零件**（词汇模块，受专门化规则约束）和**选零件**（外壳，不允许自己造）。把两者混在一起做，等于分叉了词汇表，升级标准版本时会出问题。

规则细节见 [09-architecture-foundations.md](09-architecture-foundations.md)。

---

## 8. 各角色对 `@class` 的权限

`@class` 是观察角色边界的一个具体例子。规范对不同角色的要求不同：

| 角色 | 规范要求 |
|---|---|
| 作者 | **SHOULD NOT** 修改 `@class`；它通常不出现在创作的 topic 中 |
| 词汇模块（架构师编写） | **MUST NOT** 修改那些它只是引用、并未专门化的元素的 `@class` |
| 文档类型外壳（架构师编写） | 声明 `@class` 的默认值。规范要求该默认值**不能是 fixed**，以支持泛化往返 |
| 处理器 | 依据 `@class` 匹配，而不是依据元素名 |

同一个属性，四个角色四种约束。判断某个操作是否合规，先确认自己此刻是哪个角色。

---

→ 下一步：[01-core-model.md](01-core-model.md)

---

## 来源

**已逐页核对（2026-08）**

- [基本 DITA 术语](https://dita-lang.org/dita/archspec/base/basic-dita-terminology)（读自规范源码 v2.0-beta03）— 术语章定义的全部是文档与语法构件而非角色：DITA document、DITA document type、DITA document-type shell、DITA element；**文档类型外壳"除 `dita` 元素及其属性的可选声明外，不直接声明任何元素或属性类型"**；以及 **"DITA Processor" 尚无定义**的编辑批注全文与 `TO RESOLVE 11 May 2026` 标记
- [其他术语](https://dita-lang.org/dita/archspec/base/other-terminology) — `user agent` 是角色类术语中唯一有定义条目的
- **`basetopic.dtd` 与 `basetopic.rng` 的模块引入清单**（v2.0-beta03 实际文件）— 两者引入同一组模块（`topicMod` + alternativeTitles / emphasis / hazardstatement / highlight / utilities 元素域 + audience / deliveryTarget / otherprops / platform / product 属性域），构成"同一文档类型的两个外壳"这一实例
- **基本 DITA 术语的 10 条定义**（逐条读自规范源码 v2.0-beta03 的 `basic-dita-terminology.dita`）— DITA document / DITA document type / DITA document-type shell / DITA element / DITA element type / map instance / map type / structural type instance / topic instance / topic type。第 5 节引用的定义均为这些条目的译文
- [class 属性规则与语法](https://dita-lang.org/dita/archspec/base/specialization-class-attribute) — 作者 SHOULD NOT 修改 `@class`；词汇模块 MUST NOT 修改其未专门化元素的 `@class`；默认值不能是 fixed
- **规范源码中关于语法文件的规范性陈述**（`oasis-tcs/dita` v2.0-beta03 全文检索）— "XML 语法文件以 RELAX NG（RNG）和 XML 文档类型定义（DTD）提供"；**"若存在出入，以 RELAX NG 语法为准"**；"标准 DITA 词汇表的 RELAX NG 定义是规范性版本"；OASIS 语法文件"不使用任何无法翻译为等价 DTD 构造的 RELAX NG 特性"；**DITA 对 RELAX NG 的使用依赖 RELAX NG DTD Compatibility 规范，该规范提供定义默认属性值与内嵌文档的机制**
- `doctypes/` 目录实际内容（v2.0-beta03）— 只有 `dtd` 与 `rng` 两个子目录，**未随发行提供 XSD**；`doctypes/README.md` 自述为 "DITA Version 2.0 Relax NG and DTD grammar files"
- **catalog 链与映射条目**（v2.0-beta03 实际文件）— 顶层 `doctypes/catalog.xml` 经 `<nextCatalog>` 串接 `dtd/catalog.xml` 与 `rng/catalog.xml`，二者再各自串接 `base` / `ditaval` / `subjectScheme` 三层；`dtd/base/catalog.xml` 含 58 条 `<public>` 条目，`rng/base/catalog.xml` 含 40 条 `<uri>` 与 40 条 `<system>` 条目；正文引用的两条映射（`-//OASIS//DTD DITA 2.0 Base Topic//EN` → `basetopic.dtd`，以及 `urn:pubid:oasis:names:tc:dita:rng:mapMod.rng:2.0` → `mapMod.rng`）均逐字取自这些文件；每个标识符均有 `2.0` 与 `2.x` 两条并存
- **规范源码全文检索**（`oasis-tcs/dita` v2.0-beta03，去除 `<draft-comment>` 后统计）— 各角色词的出现频次与用法语境：`processor` 约 475 处、`author(s)` 约 419 处、`DITA architect` 61 处、`information architect` 25 处、`implementer(s)` 7 处、`user agent` 12 处。第 1、2 节引用的句子均取自该检索结果，其中包括规范自述读者的那句："规范是为 DITA 标准的实现者而写的，包括工具开发者和开发专门化的 XML 架构师"

**未逐页核对，属于归纳与判断**

- 五种角色的划分与职责表 —— 规范未给出角色清单，本表由正文用法归纳
- 第 3 节"各角色该读什么"为本笔记的编排建议
- 信息架构师与 DITA 架构师的侧重差异，为基于用法语境的判断
- 第 4 节末"判断工具是否受规范约束看它是否求值 DITA 特性"，是对编辑批注暂定范围的推论，非规范条文
- 第 6 节 schema 的通用定义、"schema 装配系统"的表述、以及"管什么/不管什么"对照表，为归纳性说明
- "没配 catalog 会怎样"的三种后果、以及各类工具指定 catalog 的方式，来自使用经验，非规范条文
- 第 5 节"三层"划分、"每个术语支撑哪条规则"对照表 —— 规范未如此编排，为归纳
- 第 5 节"类型/实例"作为术语表组织原则的提法，以及"两个外壳引入同一组模块即实现同一文档类型"的推论，为归纳；"术语章未提扩展模块"是对源文件的直接观察，但其成因（尚未同步）为推测