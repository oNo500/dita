# 03 · 条件化与分块

两件事：**同一份源产出多个变体**（条件化），以及**源文件结构与输出文件结构解耦**（分块）。

---

## 1. 条件属性

在**任意元素**上打标：

| 属性 | 语义 |
|---|---|
| `@audience` | 受众（admin / developer / novice…） |
| `@platform` | 平台（linux / windows / macos…） |
| `@product` | 产品（basic / pro / enterprise…） |
| `@deliveryTarget` | 交付目标（html / pdf / epub…） |
| `@otherprops` | 其他 |
| `@props` | 通用条件属性，**可被专门化**成你自己的维度 |
| `@rev` | 修订标识（只能 flag，不能 exclude） |

```xml
<p>通用说明。</p>
<p platform="linux">Linux 上执行 <cmdname>./gradlew</cmdname>。</p>
<p platform="windows">Windows 上执行 <cmdname>gradlew.bat</cmdname>。</p>
<section audience="developer" product="enterprise">
  <title>内部扩展点</title>
  <p>仅企业版开发者可见。</p>
</section>

<!-- 多值 = OR -->
<p platform="linux macos">类 Unix 平台。</p>

<!-- 只在 PDF 里出现 -->
<p deliveryTarget="pdf">纸质版请参见封底。</p>

<!-- 分组值：给取值再分组，DITAVAL 可按组名或组内值过滤 -->
<p platform="linux database(oracle mysql)">Linux 且用 Oracle/MySQL 时相关。</p>
```

---

## 2. DITAVAL 文件

```xml
<?xml version="1.0" encoding="UTF-8"?>
<val>
  <style-conflict foreground-conflict-color="red"/>

  <prop att="platform" action="exclude"/>              <!-- 整个维度默认排除 -->
  <prop att="platform" val="linux" action="include"/>  <!-- 只保留 linux -->

  <prop att="audience" val="internal" action="exclude"/>

  <!-- flag：不删，加视觉标记 -->
  <prop att="product" val="enterprise" action="flag" backcolor="#fff3cd">
    <startflag imageref="images/ent.png"><alt-text>企业版</alt-text></startflag>
  </prop>

  <!-- 修订标记 -->
  <revprop val="2.0" action="flag" changebar="color:blue"/>
</val>
```

`@action` 四种：

| 值 | 语义 |
|---|---|
| `include` | 保留（默认） |
| `exclude` | 删除 |
| `passthrough` | **保留属性到输出**，交给 CSS/JS 做运行时过滤 |
| `flag` | 保留并加视觉标记 |

`passthrough` 值得注意：它让**在线文档做客户端动态切换**成为可能 —— 输出 HTML 里保留 `data-platform="linux"`，前端加个下拉框即可。

构建时传入：

```bash
dita -i root.ditamap -f html5 -o out-linux --filter=linux.ditaval
dita -i root.ditamap -f html5 -o out-win   --filter=windows.ditaval
```

### 过滤的判定逻辑（规范语义）

一个元素带多个条件属性、每个属性带多个值时，按这三条判定：

1. **默认是 include** —— 任何没被 DITAVAL 规则命中的属性或取值，处理默认为保留
2. **同一属性内部，有一个取值被保留，元素就保留** —— **全部**取值都被 exclude，该属性才判排除。带分组值时按组独立套用这条，**任一组判排除则该属性判排除**
3. **属性之间，任何一个判了排除，元素就被排除** —— 其余属性不再有救

规范给的例子把第 3 条说得很直白：一个段落标了三个产品，出版方把三个产品全排除了，段落就被排除 —— **即使它同时标注的受众或平台并没有被排除**。

直觉记法：**留下来需要每个维度都放行；每个维度内部只需一个值放行。**

### 排除的传播规则

如果一个元素被排除，**它的所有子元素一起消失**。父元素保留时子元素独立判定。

**一个坑**：如果排除会导致文档不合法（例如 `<steps>` 里所有 `<step>` 都被排除，而 `<steps>` 要求 `step+`），构建会报错或产出无效中间结果。**条件化要打在能整体去掉的层级上**，别把必需子元素单独排掉。

### flag 的优先级规则

- 多个 flag 命中同一元素时，**多个 flag 都渲染**，通常按遇到的顺序
- 不同层级给出**冲突**的 flag 样式时，**最低层级（离内容最近）的生效** —— 绿色 section 里的红字段落显示为红
- **同一元素既判 flag 又判 exclude 时，exclude 赢** —— 先过滤后 flag
- 2.0 的 DITAVAL 支持 `@add-outputclass`：给命中元素追加输出 class，交给 CSS 处理 —— 比逐个指定颜色/图片更工程化的 flag 方式

---

## 3. 分支过滤（`<ditavalref>`）

在**一次构建里**产出多个变体，而不是跑多次：

```xml
<map>
  <topicref href="install.ditamap" format="ditamap">
    <ditavalref href="linux.ditaval">
      <ditavalmeta>
        <dvrResourceSuffix>-linux</dvrResourceSuffix>
        <dvrTitleSuffix> (Linux)</dvrTitleSuffix>
      </ditavalmeta>
    </ditavalref>
  </topicref>
  <topicref href="install.ditamap" format="ditamap">
    <ditavalref href="windows.ditaval">
      <ditavalmeta>
        <dvrResourceSuffix>-windows</dvrResourceSuffix>
        <dvrTitleSuffix> (Windows)</dvrTitleSuffix>
      </ditavalmeta>
    </ditavalref>
  </topicref>
</map>
```

同一批 topic 被处理两遍，输出 `install-linux.html` 和 `install-windows.html`。适用于在一本手册里并列介绍多个平台的情况。

---

## 4. subjectScheme —— 给条件值上类型

裸的 `platform="linu"` 拼错不会报错，只会静默地什么都不匹配。subjectScheme 把可选值变成受控词表，编辑器能补全、构建能校验：

```xml
<subjectScheme>
  <subjectdef keys="platform">
    <subjectdef keys="linux">
      <subjectdef keys="ubuntu"/>
      <subjectdef keys="rhel"/>
    </subjectdef>
    <subjectdef keys="windows"/>
    <subjectdef keys="macos"/>
  </subjectdef>

  <enumerationdef>
    <attributedef name="platform"/>
    <subjectdef keyref="platform"/>
  </enumerationdef>
</subjectScheme>
```

**层级是重点**，其规范语义是**向上查找**：处理器判定一个属性值时，先找该值本身的 DITAVAL 规则，没有就沿树逐级上溯，用最近命中的祖先规则。所以内容标了 `ubuntu`、DITAVAL 只写 `exclude linux`，`ubuntu` 一样被排除（flag 同理向下传）。这是纯字符串匹配做不到的。

另外四条已核对的规则：

- **被绑定的容器节点本身不是合法值** —— 上例中合法值是 `linux`/`ubuntu`/`rhel`/`windows`/`macos`，不含容器 `platform` 自己
- **空枚举 = 禁用属性**：绑定一个无子节点的 `<subjectdef>`，该属性就没有任何合法值 —— "禁止使用 `@otherprops`"的 schema 级写法
- `<defaultSubject>` 可给属性提供默认值 —— 即 [11](11-processing-model.md) 五级优先级的**第 4 级**
- 处理器 SHOULD **连 DITAVAL 一起校验**：DITAVAL 里列出的值也必须在受控集内 —— 过滤条件本身的拼写错误同样能拦住

> 实践建议：条件维度超过 3 个、或团队超过 3 人，就上 subjectScheme。否则拼写错误会成为最难查的 bug。
>
> subjectScheme 不只是拼写检查 —— 它是 DITA 内建的**分类法**机制（分类树怎么画、`@subjectrefs` 主题标引、元数据字段怎么选落点），见 [14-元数据与分类策略](14-metadata-and-classification.md)。

---

## 5. 属性专门化：造自己的条件维度

这几个内置维度不够用时（比如你需要 `@region` 和 `@licenseTier`），不要滥用 `@otherprops`，而是**专门化 `@props`**。

在 doctype shell 里声明：

```xml
<task specializations="@props/deliveryTarget @props/platform @props/product
                       @props/region @props/licenseTier">
```

用：

```xml
<p region="cn" licenseTier="enterprise">仅国内企业版可见。</p>
```

DITAVAL 里直接按 `att="region"` 过滤。

**支持多级派生**：

```
@props/platform/hardwarePlatform
```

处理器能逐级泛化：`hardwarePlatform="x86"` → `platform="x86"` → `props="platform(x86)"`。`名称(值)` 就是属性泛化的落地语法。

细节见 [05-specialization.md](05-specialization.md)。

---

## 6. 分块：`@chunk`

控制**源文件结构 → 输出文件结构**的映射。只有两个值。

### `combine`

作用于 map / 分支 / mapref 时，该分组下**所有**源文档合成**一个**输出文档。

```xml
<!-- 整章合成单页 -->
<topicref href="chapter.ditamap" format="ditamap" chunk="combine"/>
```

内部元素上的 `@chunk` 一律被忽略。

### `split`

作用时，被引用文档里的**每个 topic** 单独成一个输出文档。

```xml
<!-- 一个多 topic 文件拆成多页 -->
<topicref href="multi-topic.dita" chunk="split"/>
```

### 组合

```xml
<map chunk="split">                                    <!-- 全局默认：全拆 -->
  <topicref href="a.dita"/>
  <topicref href="ref.ditamap" format="ditamap" chunk="combine"/>  <!-- 这一支合并 -->
</map>
```

规则：根上设默认，遇到 `combine` 分支则该分支整体走 combine。

**无论怎么切，输出的层级结构应与源保持一致。**

### 典型用法

| 目标 | 做法 |
|---|---|
| 在线帮助：每个 topic 一页 | 默认（不设 chunk）或根上 `split` |
| 打印/单页 HTML：整本一个文件 | 根上 `chunk="combine"` |
| 参考手册整章一页，教程逐页 | 参考章分支上 `combine`，其余不设 |
| 源文件写成一个大文件但要多页输出 | 该 topicref 上 `split` |

---

→ 下一步：[04-toolchain-and-build.md](04-toolchain-and-build.md)

---

## 来源

**已逐页核对（2026-08）**

- [关于 chunk 属性（架构规范）](https://dita-lang.org/dita/archspec/base/chunk-attribute-overview) — `@chunk` 的定位；同时确认了 "DITA processing" 章节的完整子树（导航 / 索引 / conref / 条件处理 / 元数据级联 / 分块 / 分支过滤 / 排序 / 确定属性有效值）
- [处理 chunk="combine"](https://dita-lang.org/dita/archspec/base/chunk-attribute-combine) 与 [处理 chunk="split"](https://dita-lang.org/dita/archspec/base/chunk-attribute-split) — combine 作用于 map/分支/mapref 时合并该分组全部源文档且内部 `@chunk` 被忽略；split 使每个 topic 单独成文档；根上设默认、combine 分支覆盖；层级应与源保持一致
- [迁移到 DITA 2.0](https://dita-lang.org/2.0/dita/non-normative/information-about-migrating-to-dita-2-0) — 1.x 的全部 chunk 令牌被移除，仅保留 `combine` / `split`；`@print` 由 `@deliveryTarget` 取代
- [specializations 属性规则与语法](https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute) — 属性专门化的声明语法与多级派生 `@props/platform/hardwarePlatform`
- [条件处理](https://dita-lang.org/2.0/dita/archspec/base/condproc) — 定义（按处理期条件对信息做过滤或标记）；属性值支持空格分隔与**分组**两种形态
- [过滤](https://dita-lang.org/2.0/dita/archspec/base/filtering) — **默认 include**（"未列出的属性或取值，处理默认为 include"）；同一属性内全部取值被排除才判排除，分组值按组独立判定；**任一属性判排除则元素被排除**（含"三个产品全被排除，即使受众/平台未被排除，段落也被排除"的原例）
- [标记（flagging）](https://dita-lang.org/2.0/dita/archspec/base/flagging) — 多 flag 按遇到顺序都渲染；样式冲突时**最低层级生效**；**同时命中 flag 与 exclude 时按 exclude 处理**；`@add-outputclass`
- [prop（DITAVAL 语言参考）](https://dita-lang.org/dita/langref/ditaval/prop) — `@att`/`@val`/`@action` 三属性；四种 action 的语义原文，含 **passthrough ＝ 保留内容且把属性值留在输出中供下游使用**；`<prop>` 不带 `@val` 时为该属性的整体默认，`@att` `@val` 都不带时为全局默认
- [绑定受控值到属性](https://dita-lang.org/2.0/dita/archspec/base/binding-controlled-values-to-attribute) 与 [处理受控属性值](https://dita-lang.org/2.0/dita/archspec/base/processing-controlled-attribute-values) — 第 4 节的向上查找算法、容器节点非合法值、空枚举禁用属性、`<defaultSubject>` 的求值位置、对 DITAVAL 所列值的校验期望（详细摘录见 [14](14-metadata-and-classification.md) 的来源）

**未逐页核对，来自通用 DITA 实践**

- 排除的传播规则（子元素随父消失）与"必需子元素被排空导致不合法"这一坑 —— 过滤一章未见对后代行为的明文规定
- `<ditavalref>` 的 `<ditavalmeta>` 子元素用法（分支过滤的规范语义已在 [10](10-addressing-and-key-space.md) 按 branch-filtering 一章核对）
- `@chunk` 典型用法建议表（判断性内容）

> 分支过滤与键空间的交互（含"必须先过滤再建键空间"的顺序约束）见 [10-寻址与键空间](10-addressing-and-key-space.md) 第 6 节。
