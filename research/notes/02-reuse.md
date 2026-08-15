# 02 · 复用

DITA 2.0 的三套复用机制：

| 机制 | 复用什么 | 粒度 |
|---|---|---|
| **conref** | DITA 内容 | 元素级（拉 / 推） |
| **keyref** | 引用目标、变量文本、图片 | 引用级（间接寻址） |
| **include** | **非 DITA** 的外部内容 | 元素级 |

三者可叠加。条件化（DITAVAL）是第四套机制，见 [03](03-profiling-and-chunking.md)。

---

## 1. conref —— 内容引用（拉）

### 基本用法

在一个"内容仓库" topic 里定义可复用片段：

```xml
<!-- warehouse/notes.dita -->
<topic id="reuse">
  <title>可复用片段库</title>
  <body>
    <note id="java-req" type="important">
      需要 Java 17 或更高版本。
    </note>
    <ol id="common-steps">
      <li id="s1">下载安装包。</li>
      <li id="s2">解压到目标目录。</li>
      <li id="s3">配置环境变量。</li>
    </ol>
  </body>
</topic>
```

引用：

```xml
<note conref="../warehouse/notes.dita#reuse/java-req"/>
```

**语法**：`@conref="路径#topicID/elementID"`。同文件内引用可省略路径部分，写作 `#topicID/elementID`（这仍是完整语法）；**同一 topic 内**寻址非 topic 元素另有缩写语法 `#./elementID`。两种片段标识符语法的正式定义见 [10](10-addressing-and-key-space.md)。

### 三条硬约束

1. **元素类型必须匹配** —— `<note>` 只能 conref `<note>`。允许引用**更特化**的元素（拉过来会被泛化），反之不行
2. **目标必须有 `@id`**，且在其 topic 内唯一
3. **占位元素的内容会被完全替换**。所以本地写不写内容都无所谓，但**写上 fallback 内容是好实践** —— 目标缺失时至少还有东西，且编辑器里能看见

### 范围引用（`@conrefend`）

一次拉一段连续的兄弟元素：

```xml
<ol>
  <li conref="../warehouse/notes.dita#reuse/s1"
      conrefend="../warehouse/notes.dita#reuse/s3"/>
  <li>本地追加的第四步。</li>
</ol>
```

拉的是 s1、s2、s3 三个 `<li>`。起止元素必须是同父的兄弟且同类型。

### conref push（`@conaction`）—— 反向注入

"推"模式：**由引用方决定往被引用方插入内容**，被引用文件本身不改一个字。用于"基础手册不能动，但某产品线需要在第 3 步后加一条警告"。

```xml
<steps>
  <!-- 第一个元素标记推送目标 -->
  <step conaction="mark" conref="../base/install.dita#install/step3"/>
  <!-- 紧跟的元素是要推的内容 -->
  <step conaction="pushafter">
    <cmd>额外：为本产品线设置 LICENSE_KEY。</cmd>
  </step>
</steps>
```

| `@conaction` | 语义 |
|---|---|
| `mark` | 标记目标位置（必须与 pushbefore/pushafter 配对） |
| `pushbefore` | 插到目标之前 |
| `pushafter` | 插到目标之后 |
| `pushreplace` | 替换目标（**自带 conref 指向目标，不需要 mark**） |

**警告**：conref push 是 DITA 里最难调试的特性。内容出现在输出里，但源文件里搜不到 —— 因为它是被别的文件推进去的。团队协作时慎用，或至少建立"哪些文件可被 push"的登记制度。

---

## 2. keyref —— 间接寻址

conref 的问题：路径写死了，文件一挪全网崩。keyref 加一层间接层。

### 定义 key

```xml
<map>
  <!-- key → 文件 -->
  <keydef keys="install-guide" href="topics/install.dita"/>

  <!-- key → 外部 URL -->
  <keydef keys="ot-site" href="https://www.dita-ot.org/" scope="external" format="html">
    <topicmeta><keytext>DITA-OT 官网</keytext></topicmeta>
  </keydef>

  <!-- key → 一段文本（变量！） -->
  <keydef keys="product-name">
    <topicmeta><keytext>数据罗盘</keytext></topicmeta>
  </keydef>

  <!-- key → 图片（keytext 充当 alt 文本） -->
  <keydef keys="logo" href="images/logo.png" format="png">
    <topicmeta><keytext>数据罗盘标志</keytext></topicmeta>
  </keydef>

  <!-- 一个 keydef 可定义多个 key（别名） -->
  <keydef keys="app appname product" href="topics/app.dita"/>
</map>
```

`<keydef>` 本质是 `@processing-role` 默认为 `resource-only` 的 `<topicref>` 专门化 —— 这个默认值保证 key 定义只提供资源、不产出页面也不进目录。

### 使用 key

```xml
<p>欢迎使用 <ph keyref="product-name"/>。</p>
<p>详见 <xref keyref="install-guide"/> 和 <xref keyref="ot-site"/>。</p>
<image keyref="logo" placement="break"/>
<p>内容引用也能走 key：<note conkeyref="warehouse/java-req"/></p>
```

### `<keytext>`：变量文本的正式机制

```xml
<keydef keys="product-name">
  <topicmeta>
    <keytext>数据罗盘<tm tmtype="reg">®</tm></keytext>
  </topicmeta>
</keydef>
```

内容模型：零或多个文本 + `<cite>` `<data>` `<keyword>` `<ph>` `<q>` `<term>` `<text>` `<tm>`。

也就是说**变量文本里可以带商标符、术语标记、条件属性** —— 不只是一个纯字符串。

用于图片时，`<keytext>` 自动充当 alt 文本，等同于显式写了 `<alt>`。

### keyref 的实际价值

**变量文本**：`<ph keyref="product-name"/>` 让"产品改名"变成改一行 map。Markdown 世界要做到这件事得靠模板引擎，而模板引擎和内容管理是割裂的。

**同一份内容，不同 map 出不同结果**：

```xml
<!-- map-cn.ditamap -->
<keydef keys="product-name"><topicmeta><keytext>数据罗盘</keytext></topicmeta></keydef>

<!-- map-en.ditamap -->
<keydef keys="product-name"><topicmeta><keytext>DataCompass</keytext></topicmeta></keydef>
```

topic 文件完全不动。

### key 解析规则（必须记住）

1. **map 中先定义的 key 获胜** —— 深度优先、按文档顺序，**第一个**定义有效，后面的重复定义被忽略。这与"后定义覆盖"的直觉相反
2. 因此**要覆盖某个 key，把你的 keydef 放在被覆盖 map 之前**：
   ```xml
   <map>
     <keydef keys="product-name">...我的覆盖...</keydef>
     <mapref href="base/common-keys.ditamap"/>   <!-- 里面的同名 key 失效 -->
   </map>
   ```
3. `@keyref` 和 `@href` 同时存在时，**key 能解析则 key 赢**；解析不了才回落到 href。所以可以写 `<xref keyref="k" href="fallback.dita"/>`
4. `@keys` 的值是 `NMTOKENS` —— 只能空格分隔，不能有逗号等分隔符

### key scope（`@keyscope`）

解决"同一个 submap 在一本书里用两次，但两次要绑不同的值"：

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

作用域内直接用 `ver`；从外部用 `v1.ver` / `v2.ver`。

> key scope 是最复杂的特性，实现差异也最大。用之前先用你的 DITA-OT 版本实测。

---

## 3. `<include>` —— 引用非 DITA 内容

conref 只能复用 DITA 内容。`<include>` 补上"把外部文件塞进来"。

```xml
<!-- 纯文本，带兜底 -->
<include href="../src/README.txt" parse="text" encoding="UTF-8">
  <fallback>见源码包里的 README.txt。</fallback>
</include>

<!-- 代码块 -->
<pre>
  <include href="../src/config.json" format="json" parse="text" encoding="UTF-8"/>
</pre>

<!-- XML（只能放在 <foreign> 里） -->
<foreign outputclass="tld">
  <include href="../src/jsp-tag-library.tld" parse="xml" format="tld"/>
</foreign>
```

| 属性 | 说明 |
|---|---|
| `@href` | 外部资源 URI |
| `@parse` | `text`（当纯文本，XML 字符原样显示）/ `xml`（当 XML 插入，**限 `<foreign>` 内**） |
| `@encoding` | 字符编码，未指定时处理器可自行探测 |
| `@keyref` | 间接引用 |
| `@format` / `@scope` / `@type` | 链接关系属性 |

内容模型：`data*, fallback?, foreign*`

可出现在段落、列表项、表格单元、图、章节，以及行内元素里。

**`<coderef>` 是 `<include>` 的专门化**（专用于引入预格式化代码）—— 这给了你一个官方的专门化范例。

### 为什么这个元素重要

**文档与代码同源。** README、配置样例、代码片段直接从源码仓库拉，不再手工同步。这是技术文档里最常见的"内容漂移"来源，`<include>` 从架构层面解决了它。

```xml
<section>
  <title>默认配置</title>
  <pre><include href="../../src/main/resources/application.yml"
                parse="text" format="yaml"/></pre>
</section>
```

配置文件改了，文档自动跟上。

---

## 4. 三者的处理顺序

DITA-OT 的 preprocess 大致按这个顺序（与 [04](04-toolchain-and-build.md) 的流水线图一致）：

```
条件过滤 → mapref 展开 → keyref 解析 → conref push → conref 解析 → include → topicpull → chunk
```

**推论**：

- 可以用条件化去掉一个 `<keydef>`，从而让某个变体走不同的 key 定义 ✅
- conref 的目标元素若被 DITAVAL 排除，拉过来的内容会缺失 —— **内容仓库文件通常不应该被条件化**
- keyref 在 conref 之前解析，所以 `conkeyref` 可用

准确顺序随 DITA-OT 版本有变化，写复杂逻辑前用 `--debug` 保留临时目录亲眼确认（见 [04](04-toolchain-and-build.md)）。

---

→ 下一步：[03-profiling-and-chunking.md](03-profiling-and-chunking.md)

---

## 来源

**已逐页核对（2026-08）**

- [keytext](https://dita-lang.org/dita/langref/base/keytext) — 出现位置（`<topicmeta>` 内）、双重用途（变量文本 + 图片 alt）、内容模型允许 `<cite> <data> <keyword> <ph> <q> <term> <text> <tm>`、keydef 示例
- [include](https://dita-lang.org/dita/langref/base/include) — 用途（引用非 DITA 内容）、`@parse` 取值 `text`/`xml` 及 `xml` 限于 `<foreign>` 内、`@encoding` 探测行为、`@keyref` 支持、内容模型 `data*, fallback?, foreign*`、可出现位置、与 conref 及 coderef 的区别、三个官方示例
- [迁移到 DITA 2.0](https://dita-lang.org/2.0/dita/non-normative/information-about-migrating-to-dita-2-0) — `@keys` 收紧为 `NMTOKENS`；map 中 `<linktext>` 由 `<linktitle>` 取代
- [keydef](https://dita-lang.org/dita/langref/base/keydef) — `@processing-role` 默认值为 `resource-only`；规范未把 `@toc` 列为 keydef 的显式默认属性（正文表述据此修正）

**未逐页核对，来自通用 DITA 实践**

- conref 的三条硬约束、`@conrefend` 范围引用、`@conaction` 四种取值的语义（规范的 conref 处理一章尚未逐页核对，见 [08 实践建议](08-practical-advice.md) 中列出的待补主题）
- key 解析"先定义者胜"规则与覆盖技巧、`@keyscope` 的用法示例
- preprocess 处理顺序（来自 DITA-OT 实现观察，非规范定义；规范的处理模型章节尚未核对）
- "文档与代码同源"的工程论证
