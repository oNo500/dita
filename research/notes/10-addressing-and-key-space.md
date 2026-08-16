# 10 · 寻址与键空间

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/architecture/addressing-modes.dita` ← §1–2；`kb/topics/dita/architecture/key-space-model.dita` ← §3–4；`kb/topics/dita/architecture/cross-deliverable-addressing.dita` ← §5；`kb/topics/dita/architecture/branch-filter-key-space.dita` ← §6；§7「给程序化处理的启示」由 Task 10b 迁入 `kb/topics/dita/architecture/processing-checklist.dita`，与 [11](11-processing-model.md) §6 合并为一篇；不迁小节：无），本文冻结为调研档案，不再更新。

> [02-reuse.md](02-reuse.md) 给出的是 key 的操作规则（"先定义者胜"）。本篇讲规范定义的模型。keyscope 与分支过滤叠加时，只有理解模型才能预测结果。

---

## 1. 两种寻址

DITA 只有两种寻址机制：

| | 直接寻址（direct） | 间接寻址（indirect） |
|---|---|---|
| 载体 | URI（`@href` / `@conref`） | key（`@keyref` / `@conkeyref`） |
| 绑定时机 | 写死在文档里 | 处理时经 map 解析 |
| 定义位置 | 引用处 | **map 层面** |
| 可复用性 | 低 —— 换位置就断 | 高 —— 同一 topic 在不同 map 里可指向不同资源 |

规范对 key 的定位是：**"提供一层间接，使资源（URI、元数据、变量文本字符串）可以在 DITA map 层面定义，而不是在每个 topic 里各自定义。"**

这句话点出了 key 的三种用途 —— 不只是链接目标，还有**元数据**和**变量文本**。

---

## 2. 直接寻址的细节

### `@id` 与片段标识符

元素通过 `@id` 获得唯一标识。规范定义了**两种片段标识符语法**：

```
完整语法：  file.dita#topicID/elementID
缩写语法：  #./elementID          ← 同一 topic 内寻址非 topic 元素
```

要点：

- topic 的 `@id` 在**文档内**必须唯一
- 非 topic 元素的 `@id` 在**其所在 topic 内**唯一
- 因此 `#topicID/elementID` 两段式是必要的 —— 单靠 elementID 不足以定位

### 链接属性

`@format`、`@href`、`@scope`、`@type` 共同描述一条链接：

| 属性 | 作用 |
|---|---|
| `@href` | URI 引用 |
| `@format` | 目标的格式（`dita` / `ditamap` / `html` / `pdf` / `markdown`…） |
| `@scope` | `local`（纳入构建）/ `peer`（同批交付但不由本次构建处理）/ `external`（外部） |
| `@type` | 目标的类型提示 |

`@scope="peer"` 常被误用。它的含义是"属于同一套交付物、但不在本次构建范围内" —— 用于**跨交付物链接**，见第 5 节。

### `<resourceid>`

提供**上下文钩子（context hooks）**，用于把文档 topic 挂接到应用程序的联机帮助调用上。DITA 2.0 里它同时承担了 1.x 中 `@copy-to` 的部分职责。

---

## 3. 键空间（key space）

### 模型

一个 key 空间是**从 key 名到 key 定义的映射**。它由 root map 出发、按 map 层级构造。

关键规则：

1. **深度优先、按文档顺序遍历**
2. **同名 key 的第一个定义获胜**，后续同名定义被忽略
3. 因此**覆盖的写法是把你的定义放在前面**：

```xml
<map>
  <keydef keys="product-name">...我的覆盖...</keydef>
  <mapref href="base/common-keys.ditamap"/>   <!-- 里面的同名定义失效 -->
</map>
```

> 这条与"后赋值覆盖前赋值"的编程直觉相反，是最高频的困惑来源。记法：**key 空间是"先到先得"，不是"后来居上"**。

### key 解析成什么

同一个 key，在不同引用上下文里解析成不同东西：

| 引用上下文 | 解析为 |
|---|---|
| `<xref keyref>` / `<link keyref>` | 链接目标 + 链接文本 |
| `<topicref keyref>` | 被引用的资源 |
| `<image keyref>` | 图片资源 + alt 文本（取 `<keytext>`） |
| `<ph keyref>` / `<keyword keyref>` | 变量文本（取 `<keytext>`） |

规范为这几种情况分别单列了处理规则。**一个 key 定义可以同时提供 `@href` 和 `<keytext>`**，由引用方的上下文决定用哪个。

---

## 4. key 作用域（`@keyscope`）

### 解决什么

同一个 submap 在一本书里被引用两次，但两次要绑不同的值。没有作用域时，按"先定义者胜"的规则，第二次引用的 key 定义不会生效。

```xml
<map>
  <topicref href="shared/module.ditamap" keyscope="v1">
    <keydef keys="ver"><topicmeta><keytext>1.0</keytext></topicmeta></keydef>
  </topicref>
  <topicref href="shared/module.ditamap" keyscope="v2">
    <keydef keys="ver"><topicmeta><keytext>2.0</keytext></topicmeta></keydef>
  </topicref>
</map>
```

### 规则

- **作用域内**直接用 `ver`
- **从外部**引用要限定：`v1.ver` / `v2.ver`
- 作用域**可嵌套**，形成层级
- 一个 `@keyscope` 可指定**多个作用域名**（空格分隔）

### 心智模型

把 key 作用域想成**嵌套的命名空间**：内层可见外层，外层看内层要加前缀。但注意"先到先得"仍然在各自作用域内生效。

---

## 5. 跨交付物寻址

`@keyscope` 加上 `@scope="peer"`，可以让**两个独立交付物之间**的链接在各自发布后仍然可用。

场景：《用户手册》要链接到《API 参考》里的某个 topic，但两者是分开构建、分开发布的。

做法是把对方的 root map 作为 peer 作用域引用进来，然后用限定 key 名引用。处理器在构建 A 时知道"这个 key 属于 B，B 会在别处发布"，从而生成指向 B 发布位置的链接，而不是把 B 拉进 A 的构建。

> 这是 DITA 中处理多套文档互链的机制。实现成熟度需要按你的工具链实测。

---

## 6. 分支过滤与键空间的交互

这两个机制叠加时规则较为复杂，也是各家实现差异最大的地方。

### 分支复制

`<ditavalref>` 把一个 DITAVAL 的过滤规则应用到 map 的**某个分支**。当一个分支下有**多个 `<ditavalref>` 兄弟**时，处理器把这个分支**克隆多份**，每份用一个 DITAVAL 独立过滤：

```xml
<topicref href="install.ditamap" format="ditamap">
  <ditavalref href="linux.ditaval">
    <ditavalmeta>
      <dvrResourceSuffix>-linux</dvrResourceSuffix>
      <dvrTitleSuffix> (Linux)</dvrTitleSuffix>
    </ditavalmeta>
  </ditavalref>
  <ditavalref href="windows.ditaval">
    <ditavalmeta>
      <dvrResourceSuffix>-windows</dvrResourceSuffix>
      <dvrTitleSuffix> (Windows)</dvrTitleSuffix>
    </ditavalmeta>
  </ditavalref>
</topicref>
```

`<ditavalmeta>` 的四个元素负责给克隆出来的副本改名：

| 元素 | 作用 |
|---|---|
| `<dvrResourcePrefix>` / `<dvrResourceSuffix>` | 改**资源名**（输出文件名） |
| `<dvrTitlePrefix>` / `<dvrTitleSuffix>` | 改**生成的标题** |

**为什么必须改名**：规范明确要求，分支被克隆时**处理器必须处理资源名和 key 名的冲突**。同一个 topic 被复制成两份，如果不改名，两份会抢同一个输出路径和同一个 key。

### 关键的处理顺序约束

> **分支过滤会改变 root map 的全局键空间。因此处理器必须先求值分支过滤，才能构造键空间。**

这条规范要求的后果：

1. **key 空间不是静态的** —— 它依赖于分支过滤的结果
2. **不能先建 key 空间再做分支过滤** —— 顺序反了结果就错
3. 克隆出的每份分支里，key 名会因 `<ditavalmeta>` 的改名而**各自不同**
4. 你自己写处理逻辑时，**这个顺序是硬约束**，不是优化选择

### 实践建议

- **同一分支里同时用多 `<ditavalref>` 和 `@keyscope` 时，务必实测**。这是规范最复杂、实现最容易分歧的组合
- 克隆分支里的 key 要被外部引用时，**显式规划改名规则**，别依赖处理器的默认行为
- 先用 `-f dita` 出中间结果，**亲眼看克隆后的 key 长什么样**，再写依赖它的逻辑

---

## 7. 给程序化处理的启示

如果你在 [07](07-programmatic-processing.md) 的路上自己实现 key 解析：

1. **必须先做分支过滤，再建 key 空间** —— 顺序颠倒会导致 key 解析全部出错
2. **"先到先得"而非"后来居上"**
3. **key 空间是有层级的**（keyscope），不是扁平字典
4. **同一 key 在不同引用上下文解析成不同东西** —— 解析结果不是单值，要按上下文取 href 或 keytext
5. 真的不建议自己实现。用 `dita -f dita` 拿 DITA-OT 解析好的结果，见 [07](07-programmatic-processing.md)

---

→ 下一步：[11-processing-model.md](11-processing-model.md)

---

## 来源

**已逐页核对（2026-08）**

- [DITA 寻址](https://dita-lang.org/dita/archspec/base/ditaaddressing) — 两种寻址机制（直接 URI / 间接 key）；`@id` 提供元素唯一标识；**规范定义两种片段标识符语法**（完整式与用于同 topic 内非 topic 元素的缩写式）；链接属性 `@format` `@href` `@scope` `@type`；`<resourceid>` 提供上下文钩子；子树 URL（id / dita-linking / uri-based-addressing / key-based-addressing / context-hooks-for-user-assistance）
- [基于 key 的寻址](https://dita-lang.org/2.0/dita/archspec/base/key-based-addressing) — key 的定位原话（"提供一层间接，使资源——URI、元数据、变量文本字符串——可在 map 层面定义而非在每个 topic 中各自定义"）；`@keys` 由 `<topicref>` 或其专门化定义；key 可解析为 `@href`、key 定义元素的内容、或两者；`@keyscope` 控制作用域边界，跨作用域引用需以作用域名限定；**跨交付物寻址**使不同 root map 间的链接可用；key 依上下文解析为链接 / 文本 / 两者；该节子页清单（核心概念 / keys 属性 / keyref 属性 / 用于寻址 / keyscope / 跨作用域 / 跨交付物 / 四类处理规则 / 两组示例）
- [分支过滤](https://dita-lang.org/2.0/dita/archspec/base/branch-filtering) — `<ditavalref>` 的作用范围（过滤分支本身及其引用的本地 map 与 topic）；多个 `<ditavalref>` 兄弟导致**分支克隆**，每份独立过滤；`<ditavalmeta>` 的 `<dvrResourcePrefix>` / `<dvrResourceSuffix>` / `<dvrTitlePrefix>` / `<dvrTitleSuffix>` 分别改资源名与生成标题；**"分支被克隆时处理器必须处理资源名与 key 名冲突"**；**"分支过滤可导致 root map 全局键空间发生变化，因此处理器必须求值分支过滤以构造键空间"**（本篇第 6 节处理顺序约束的依据）；子页清单

**未逐页核对**

- key 空间的**遍历算法细节**（深度优先、按文档顺序）—— 来自通用实践与 02 沿用；`key-based-addressing` 的"核心概念"子页未单独打开
- `@keyscope` 支持多值、作用域嵌套的可见性规则 —— 未打开 `keyscope` 子页逐条核对
- 跨交付物寻址的**具体配置方式**（第 5 节的做法描述）—— 概念来自已核对页面，落地写法为推断，正文已提示需按工具链实测
- `@scope="peer"` 的精确语义表述
- 第 7 节为面向实现的判断性建议