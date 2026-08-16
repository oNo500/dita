# 05 · 专门化（Specialization）

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/architecture/structural-specialization.dita` ← §1；`kb/topics/dita/architecture/domain-specialization.dita` ← §2；`kb/topics/dita/architecture/attribute-specialization.dita` ← §3，并入 [03](03-profiling-and-chunking.md) §5 的条件维度用法；`kb/topics/dita/architecture/constraints-generalization.dita` ← §4–5，并入「本篇在整体框架中的位置」里 constraint 与 specialization 并列这一层级提醒；`kb/topics/dita/architecture/specialization-practice.dita` ← §核心思想 与 §6；不迁小节：「本篇在整体框架中的位置」的三个扩展设施完整框架，归 [09](09-architecture-foundations.md) 与其对应 topic），本文冻结为调研档案，不再更新。

> 二次开发的第一条路：**扩展词汇表**。

## 核心思想

传统做法扩展 schema = 造一个新格式，所有下游工具都得重写。

DITA 的做法：**每个新元素必须能一对一映射回一个已有元素，并在 `@class` 里记录这条派生链**。于是：

- 不认识你的元素的处理器 → 按基类处理，**优雅降级**，输出仍然正确
- 认识的处理器 → 走你的专用模板，得到更好的效果
- 需要交换内容时 → **泛化（generalize）** 回标准 DITA，无损

这条性质叫 **specialization is a subset relationship**：专门化元素的内容模型必须是基类的子集，属性也必须是基类属性的子集或专门化。**只能收紧，不能放宽。**

---

## 本篇在整体框架中的位置

规范的正式框架是**三个扩展设施**：文档类型配置、**专门化**、元素类型配置（含约束与扩展两个子类）。完整框架见 [09-architecture-foundations.md](09-architecture-foundations.md)。

**本篇只讲其中的"专门化"这一个设施**，外加约束的简介。专门化本身分三种：

| 种类 | 做什么 | 何时用 |
|---|---|---|
| **结构化专门化** | 造新的 topic 类型 / map 类型 | 有一类内容有固定的信息结构（如"API 端点"） |
| **域专门化** | 造可跨类型使用的行内/块元素 | 有一类语义标记要到处用（如 `<sensor-id>`） |
| **属性专门化** | 从 `@props` / `@base` 派生新属性 | 需要新的条件化维度 |

三者都是"加"。与之配套的另外两件事：

- **约束（constraint）** —— 收紧已有元素的内容模型，属于**元素类型配置**，不属于专门化。本篇第 4 节简介
- **扩展（expansion）** —— 与约束反向，把专门化元素/属性**只加到特定元素类型上而不全局可用**。同属元素类型配置，见 [09](09-architecture-foundations.md)

> ⚠️ 注意层级：结构化 / 域 / 属性是**专门化**这一个设施下的三种，约束与扩展属于**元素类型配置**。把它们摊平并列成"四种（五种）扩展手段"是常见错误。完整框架以 [09](09-architecture-foundations.md) 为准。

**无论用哪一种，最终都要靠 document-type shell 装配进来** —— shell 是所有定制的汇合点，见 [09](09-architecture-foundations.md)。

---

## 1. 结构化专门化

例：为 API 文档造一个 `apiref` topic 类型，从 `reference` 派生。

### 先画映射表

| 新元素 | 派生自 |
|---|---|
| `apiref` | `reference` |
| `apirefbody` | `refbody` |
| `endpoint` | `section` |
| `params` | `simpletable` |

**映射不清楚说明设计还没想好。** 这一步不能跳。

### RNG 模块（推荐路线）

```xml
<!-- apiref.rng（节选，示意结构） -->
<grammar xmlns="http://relaxng.org/ns/structure/1.0"
         xmlns:a="http://relaxng.org/ns/compatibility/annotations/1.0"
         xmlns:dita="http://dita.oasis-open.org/architecture/2005/">

  <!-- 引入基类的结构模块（referenceMod.rng），不是外壳（reference.rng）。
       模块只依赖模块；外壳（含域、约束的装配）由使用方的 shell 另行负责，
       见 09 的模块 vs 外壳分层 -->
  <include href="referenceMod.rng"/>

  <define name="apiref.content">
    <ref name="title"/>
    <optional><ref name="shortdesc"/></optional>
    <optional><ref name="prolog"/></optional>
    <optional><ref name="apirefbody"/></optional>
  </define>

  <define name="apiref.element">
    <element name="apiref">
      <a:documentation>API 端点参考</a:documentation>
      <ref name="apiref.attlist"/>
      <ref name="apiref.content"/>
    </element>
  </define>

  <!-- 声明 @class 的默认值。规范要求该默认值不能是 fixed。
       dita:defaultValue 来自 RELAX NG DTD Compatibility 规范 —— RNG 本身
       没有默认属性值的概念，DITA 靠这个扩展提供，见 00 第 6 节 -->
  <define name="apiref.attributes">
    <ref name="id-atts"/>
    <ref name="localization-atts"/>
    <optional>
      <attribute name="class"
        dita:defaultValue="- topic/topic reference/reference apiref/apiref "/>
    </optional>
  </define>
</grammar>
```

### `@class` 值的构造规则（写错就全盘失效）

```
class="- topic/topic reference/reference apiref/apiref "
       │ └───────────┘ └─────────────────┘ └──────────┘ │
       │   最泛化          中间层            你的元素     │
       前缀 `-`（结构化）                          结尾空格必需
```

- 结构化专门化用 `-`，域专门化用 `+`
- 列出**完整**祖先链，从最泛化到最特化
- 模块名部分（斜杠前）通常等于该模块的名字，不是元素名
- **首尾必须各有一个空格**

### 注册到 DITA-OT

```xml
<!-- plugin.xml -->
<plugin id="com.example.apiref">
  <feature extension="dita.specialization.catalog.relative" file="catalog-dita.xml"/>
</plugin>
```

`catalog-dita.xml` 把 public ID / system ID 映射到你的 RNG/DTD 文件，然后 `dita install`。

之后 DITA-OT 就能校验和处理你的 `apiref` 文件了 —— **即使你一行 XSLT 都没写**，因为 html5 插件按 `@class` 匹配，会把它当 reference 渲染。

---

## 2. 域专门化（Domain）

域元素**跨 topic 类型可用**，是更轻量的扩展方式。DITA 自带的域就有十几个：`ui-d`（UI 控件）、`sw-d`（软件）、`pr-d`（编程）、`syntaxdiagram-d`（语法图）、`hi-d`（高亮）、`emphasis-d`（`<strong>`/`<em>`）、`ut-d`（实用）、`xml-d`、`mathml-d`、`svg-d`、`multimedia-d`（`<audio>`/`<video>`）、`alternative-titles-d`（`<navtitle>` 等）。

```
<sensor-id class="+ topic/ph iot-d/sensor-id ">     ← 前缀是 +
```

于是任何 topic / concept / task 里都能写 `<sensor-id>TEMP-04</sensor-id>`，不认识它的处理器当 `<ph>` 处理。

**结构化 vs 域，怎么选？**

- 要定义**一整篇内容的骨架** → 结构化专门化
- 要定义**到处能用的一个标记** → 域专门化
- 拿不准 → 优先域专门化，成本低得多，也更容易被复用

### 好用的派生基类

| 基类 | 适合派生成 |
|---|---|
| **`<div>`** | 任何自定义块级结构（通用分组容器，无语义包袱） |
| **`<ph>`** | 任何自定义行内语义标记 |
| **`<include>`** | 任何"从外部拉内容"的场景。**`<coderef>` 就是它的专门化** —— 现成的官方范例 |
| **`<data>`** | 任何要挂在内容上的元数据 |
| **`<foreign>`** | 嵌入非 DITA 的 XML 词汇表（MathML/SVG 就是这么做的） |
| **`<titlealt>`** | 自定义标题角色（但 `@title-role` 支持自定义值，很多时候连专门化都不用做） |

---

## 3. 属性专门化

只有 `@props` 和 `@base` 两个属性可被专门化（`@base` 用于非条件化的语义属性）。

在 doctype shell 里声明，产出实例上的 `@specializations`：

```xml
<task specializations="@props/deliveryTarget @props/platform @props/product
                       @props/region @props/licenseTier">
```

**语法**：`'@', (props|base), ('/', 属性名)+`

```
@props/myNewProp                    从 @props 派生 @myNewProp
@base/myFirstBase                   从 @base 派生
@base/myFirstBase/mySecondBase      两级派生
@props/platform/hardwarePlatform    从已有的 @platform 再派生
```

用：

```xml
<p region="cn" licenseTier="enterprise">仅国内企业版可见。</p>
```

DITAVAL 里直接按 `att="region"` 过滤。泛化后等价于 `props="region(cn) licenseTier(enterprise)"` —— `名称(值)` 就是属性泛化的落地语法。

### 三个要点

1. **`@specializations` 只记录属性专门化，不记录元素域。** 文档实例里没有"本文档用了哪些元素域"的信息
2. **值由 doctype shell 的默认值注入**，作者不手写 —— 和 `@class` 一样，**裸解析拿不到**（见 [07](07-programmatic-processing.md)）
3. **层级路径让多级派生可以正确泛化**：`hardwarePlatform="x86"` → `platform="x86"` → `props="platform(x86)"`，而不是一步跳到 `@props`

> 第 3 点是实质性的能力：如果你在做条件化维度的专门化，多级派生现在是可表达的。

---

## 4. 约束（Constraint）

约束**不引入新元素**，只收紧现有内容模型。

典型用途：

- 强制 `<shortdesc>` 必填（对生成搜索摘要很重要）
- 禁用 `<draft-comment>` 出现在发布分支
- 限制表格只能用 `<simpletable>`（CALS table 太复杂）
- 禁掉 `<steps-informal>`，强制用严格的 `<steps>`
- 禁止 topic 嵌套
- 去掉不用的域（减小 schema，加快校验）

**为什么约束比"写个 lint 脚本"好**：约束是 schema 级的，编辑器直接不给你补全那个元素，错误在输入时就被拦住，而不是提交后被 CI 骂。

---

## 5. 泛化（Generalization）

反向操作：把专门化文档变回基类文档。算法就是读 `@class`：

```
<step class="- topic/li task/step ">   →   <li class="- topic/li ">
```

规则：

- 取 `@class` 里**目标层级**的元素名替换标签名
- 属性同理，`region="cn"` → `props="region(cn)"`
- **可以泛化到任意中间层**，不必一路到 topic

**什么时候用**：把内容交给不支持你专门化的合作方、迁移到别的系统、归档。

这也解释了为什么 `@class` 必须完整列出祖先链 —— 泛化算法只靠这一个属性，不查任何 schema。

**写完专门化先测泛化**：能无损泛化回标准 DITA，说明 `@class` 都对了。

---

## 6. 实战建议

### 1. 先问"真的需要吗"

90% 的场景用 `@outputclass` + CSS 就够了：

```xml
<p outputclass="warning-box">...</p>
<div outputclass="feature-grid">...</div>
```

`@outputclass` 是**通用属性**（任何元素都能用），会被 html5 transtype 原样输出成 HTML 的 `class` 属性。**不需要改 schema、不需要装插件、不影响交换性。**

专门化的成本是：一套 schema 要维护、编辑器要配、DITA-OT 要装插件、规范演进时要跟着走。只有当你需要**语义校验**（强制"每个 API 端点必须有 method"）或**语义驱动的处理**（自动从 apiref 生成 OpenAPI）时，才值得。

### 2. 优先 RNG 而不是 DTD

可读性差一个量级，且是规范性来源。

**RNG 写好后怎么给下游生成 DTD**（[07](07-programmatic-processing.md) 的 lxml 路线等场景仍需要 DTD）：

- **DITA Community 的 `org.dita.rng-converter`**（Eliot Kimber 的 rng2ditadtd）—— OASIS TC 自己发行包里的 DTD 就是用这条管道从 RNG 生成的；装成 DITA-OT 插件即可用
- **Oxygen** 内置 RNG→DTD 转换
- 前提是你的 RNG 遵守了 [00](00-roles-and-boundaries.md) 提到的那条限制：**不使用无法翻译成 DTD 构造的 RELAX NG 特性**——OASIS 的模块刻意如此，你照着模板写自然满足

### 3. 从 OASIS 的模板起步

别从零写。`oasis-tcs/dita` 仓库的 v2.0-beta03 里有完整的模块结构可抄。

### 4. 注意 2.0 仍在演进

规范是 beta，语法文件还会动。做专门化时：

- **锁定你基于的语法文件版本**（记录 beta 号）
- 尽量从**稳定的基类**派生（`<div>` `<ph>` `<section>` `<simpletable>` 这些不太可能再变）
- 避开还在讨论中的区域

---

→ 下一步：[06-dita-ot-plugins.md](06-dita-ot-plugins.md)

---

## 来源

**已逐页核对（2026-08）**

- [specializations 属性规则与语法（架构规范）](https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute) — 形式语法 `'@', props-or-base, ('/', attname)+`；仅覆盖属性专门化、不含元素域；多级派生示例 `@base/myFirstBase/mySecondBase`；完整示例 `specializations="@props/deliveryTarget @props/platform @props/product"`；值由 doctype shell 默认注入而非作者手写
- [include](https://dita-lang.org/dita/langref/base/include) — `<coderef>` 是 `<include>` 的专门化（正文用作官方专门化范例）
- [架构规范目录结构](https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute) — "Configuration and specialization" 子树：扩展设施概览 / 文档类型配置 / 专门化（模块化、词汇模块、元素与属性专门化规则、class 与 specializations 属性、非 DITA 内容、跨专门化共享元素）/ 泛化 / 约束 / **扩展模块**
- [oasis-tcs/dita 发布页](https://github.com/oasis-tcs/dita/releases) — v2.0-beta03 含语法文件，用于"从 OASIS 模板起步"的建议
- [class 属性规则与语法](https://dita-lang.org/dita/archspec/base/specialization-class-attribute) — 本篇 `@class` 构造规则的全部要点（前缀 `-`/`+`、从泛化到特化的完整祖先链含未改名的中间层、结尾至少一个空格、默认值不得为 fixed）均与该页核对一致

**未逐页核对，来自通用 DITA 实践**
- RNG 专门化模块的示例代码（示意结构，非从官方模板逐行抄录）
- 域专门化的模块清单（ui-d / sw-d / pr-d 等）与"好用的派生基类"表
- 约束的典型用途清单、泛化算法的操作性描述
- "先问真的需要吗"的成本论证与 `@outputclass` 优先建议（判断性内容）
- RNG→DTD 生成工具（`org.dita.rng-converter` / Oxygen 内置转换）及"OASIS 发行包的 DTD 由该管道生成"的说法 —— 来自社区通识，未核对其当前维护状态与对 2.0 语法的支持程度，用前先实测