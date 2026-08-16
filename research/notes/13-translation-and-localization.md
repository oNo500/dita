# 13 · 翻译与本地化

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/practice/translation-modularity.dita` ← §1 + §3；`kb/topics/dita/practice/localization-attributes.dita` ← §2；`kb/topics/dita/practice/translation-antipatterns.dita` ← §4 + §5；不迁小节：无），本文冻结为调研档案，不再更新。

> [08](08-practical-advice.md) 的选型表里，"多语言翻译"是 DITA 的核心收益项 —— 本篇交代这个论断的机制依据，以及复用机制在翻译场景下的反面。

---

## 1. 为什么模块化能降翻译成本

翻译成本的大头不是"翻"，是**重翻**和**同步**。DITA 的三个机制分别砍掉一块：

| 机制 | 砍掉的成本 |
|---|---|
| **topic 即翻译单元** | 改了哪个 topic 只重翻哪个 —— 翻译记忆（TM）和 CCMS 按文件级跟踪，未动的 topic 直接复用旧译文 |
| **复用即"翻一次"** | conref / keyref 的片段只有一份源，也就只需一份译文；副本制的世界里同一段话在 N 处出现就要翻 N 次、同步 N 次 |
| **单源多变体** | N 个变体 × M 种语言，副本制要维护 N×M 份内容；DITA 只维护一份源 + M 份译文，变体靠构建时过滤产生 |

收益公式（[12](12-philosophy-and-principles.md) 代价 3 的分子）里"语言数"是乘数 —— 这就是为什么多语言场景最能摊平 DITA 的固定成本。

但复用在翻译下也有专属的坑，见第 4 节 —— **那一节比本页其余部分都重要**。

---

## 2. 三个本地化属性

### `@xml:lang` —— 内容的语言与区域

- 值遵循 XML 规范（BCP 47 风格，如 `zh-CN` `en-US`）
- **沿包含层级继承**：下层元素不写就取上层的值，写了就覆盖 —— 段落级的语言切换（术语原文、引文）靠这个
- 未指定时**处理器自定默认值** —— 所以要**显式写**，别赌处理器的默认
- **关键规则：在 `<topicref>` 上指定 `@xml:lang` 不作用于被引用的 topic。** map 不会替 topic 声明语言 —— **每个 topic 文件必须自己声明**

最后一条与 [11](11-processing-model.md) 级联清单的关系要分清：`@xml:lang` 在**map 内部**级联（作用于嵌套的 topicref、topicmeta 里的元数据），但**不穿透到被引用的 topic 文件**。级联沿的是 map 的包含层级，不是引用关系。

```xml
<!-- 每个 topic 根上显式声明 -->
<topic id="intro" xml:lang="zh-CN">
  ...
  <p>此特性即 <term xml:lang="en">content reference</term>（内容引用）。</p>
</topic>
```

### `@dir` —— 双向文本

| 值 | 语义 |
|---|---|
| `ltr` / `rtl` | 左到右 / 右到左 |
| `lro` / `rlo` | **覆盖** Unicode 双向算法，强制左到右 / 右到左 |
| `-dita-use-conref-target` | 方向取 conref 目标的 |

处理器若完整支持 Unicode 双向算法（BiDi），**通常只需在最高层元素上设对 `@xml:lang`**，不必逐处写 `@dir`；`lro`/`rlo` 留给算法判错的局部（如混排的代号、电话号码）。

### `@translate` —— 该不该翻

- 取值 `yes` / `no` / `-dita-use-conref-target`
- 多数元素无默认；**为承载非发布内容而设计的元素默认 `translate="no"`**（`<draft-comment>`、`<required-cleanup>` 等）
- 规范附有一个**非规范性附录**，逐元素给出译者建议：哪些内容不宜翻译（代码片段、邮寄地址等）、哪些元素的**属性值**可能需要本地化 —— 给翻译供应商做 DITA 配置时，这个附录就是底稿

```xml
<p>执行 <codeph translate="no">dita --project=all.xml</codeph> 完成构建。</p>
```

---

## 3. 翻译流程的架构位置

DITA 规范只管源标记；**翻译的交换与回填是工具层的事**。典型流程：

```
源语言写作 → 冻结（版本标记）→ 按 map 收集 topic 打翻译包
   → 交换格式（XLIFF 为业界主流；配合 ITS 规则声明可译性）
   → 译文回填为平行的 DITA 文件 → 各语言独立构建
```

要点：

- **XLIFF / ITS 不是 DITA 规范的一部分** —— 是否支持、分段质量如何，是评估 CCMS / 翻译管理系统的核对项，不是 DITA 自带的能力
- **变量文本天然是翻译点**：[02](02-reuse.md) 里 map-cn / map-en 各绑一套 `<keytext>` 的例子，本质就是翻译场景 —— key 定义所在的 map 按语言分份，topic 不动
- **先翻还是先过滤**：一般**全量翻译**（一份译文源服务所有变体，保持单源性质）；只有按字数计费且变体差异极大时才考虑先过滤再翻 —— 代价是译文侧失去单源
- 文件名与 `@id` **不随语言变**：各语言目录平行同构，工具靠路径对应源文与译文

```
docs/
├── zh-CN/topics/install.dita
├── en-US/topics/install.dita     ← 同名同 id，只有内容语言不同
└── maps/…（每语言一份，或共用 map + 语言 keydef）
```

---

## 4. 复用机制在翻译下的坑

**这一节是本篇的重点。** 复用降低翻译成本的前提是：复用单元在**目标语言**里仍然成立。两个高频反模式：

### 反模式一：用变量拼句子

```xml
<!-- ❌ 源语言读着通顺，翻译必然出错 -->
<p>点击<ph keyref="button-name"/>以保存设置。</p>
```

问题：目标语言的**词序、冠词、格变化**围绕变量发生。德语的动词位置、法语的性数配合、日语的助词 —— 变量两侧的文字在不同取值下需要不同形态，而源里只有一份。

**规则：变量文本只用于名词性、位置无关的内容**（产品名、版本号、公司名）。凡是"变量参与句法"的地方，写完整句子，让每个变体各自成句（用条件化区分，而不是用变量拼接）。

### 反模式二：低于句子粒度的 conref

conref 半句话、共享句段 —— 源语言省了几个字，目标语言里那半句的语法形态取决于上下文，一份译文无法同时适配多处。

**规则：conref 的最小安全粒度是完整句，更稳的是完整块**（note、li、p）。这是 [12](12-philosophy-and-principles.md) 推论 2"复用单元的边界是自足性"在翻译维度的加强版 —— 单语言下"半句复用"只是别扭，多语言下是**必然出错**。

### 其余要点

- **术语一致性**：[01](01-core-model.md) 的 glossentry + `<term keyref>` 模式在多语言下就是术语库 —— 每语言一套 glossary topic，术语改译只动一处
- **中文/日文排序**：索引与术语表的 `<sort-as>` 注音策略见 [11](11-processing-model.md) 第 4 节 —— 选型阶段就要测处理器的 CJK collation
- **图片里的文字**：`<image>` 内嵌文字无法进翻译流程 —— 截图按语言出多套（目录平行，同名覆盖），示意图用 SVG（文字可译）或干脆不放文字

---

## 5. 实践清单

1. **每个 topic 与 map 的根元素显式写 `@xml:lang`** —— 不依赖处理器默认
2. 代码、命令、路径统一用语义元素（`<codeph>` `<cmdname>` `<filepath>`）—— 配合逐元素翻译建议，翻译工具可整类跳过
3. 需要成段排除翻译的内容显式 `translate="no"`
4. 写作规范里**禁止用变量拼句子**、**禁止低于句子粒度的 conref** —— 进 lint（Schematron 可查 `<ph keyref>` 出现在句中的启发式模式，见 [04](04-toolchain-and-build.md)）
5. 评估工具链时核对：XLIFF 分段是否 DITA 感知（按元素而非按行）、TM 是否按 topic 复用、`@translate` 是否被尊重
6. 翻译启动前冻结源 —— 边翻边改源是成本失控的最快路径

---

→ 回到 [README](../README.md)

---

## 来源

**已逐页核对（2026-08）**

- [翻译与本地化（架构规范）](https://dita-lang.org/2.0/dita/archspec/base/translation) — 该节结构：`@xml:lang` / `@dir` / `@translate` 三个子页
- [xml:lang 属性](https://dita-lang.org/2.0/dita/archspec/base/xmllang) — 指定内容的语言与区域；沿包含层级为未指定的下层元素供值、下层可覆盖；未指定时处理器自定默认；**"在 topicref 上指定的 `@xml:lang` 不作用于被引用的资源"**
- [dir 属性](https://dita-lang.org/2.0/dita/archspec/base/diratt) — 取值 `ltr` / `rtl` / `lro` / `rlo` / `-dita-use-conref-target`，`lro`/`rlo` 为对 Unicode 双向算法的覆盖；完整支持 BiDi 的处理器在最高层设 `@xml:lang` 即可正确渲染双向文本
- [translate 属性](https://dita-lang.org/2.0/dita/archspec/base/the-translate-attribute) — 取值 `yes`/`no`/`-dita-use-conref-target`；多数元素无默认，**面向非发布内容的元素（`<draft-comment>` `<required-cleanup>` 等）默认 `translate="no"`**；存在逐元素译者建议的非规范性附录（不宜翻译的内容类型、属性值可能需本地化的元素）

**未逐页核对，来自通用实践与判断**

- XLIFF / ITS 的工具层流程、"先翻还是先过滤"的取舍、目录平行布局 —— 行业通行做法，非规范内容
- 第 4 节两个反模式及其规则、第 5 节实践清单 —— 判断性内容；"变量拼句在目标语言必然出错"的论证基于翻译通识，规范未如此表述
- `@translate` 的继承行为规范页未明确说明，本篇未做断言
- 逐元素译者建议附录的**具体内容**未逐条核对（仅确认其存在与定位）
