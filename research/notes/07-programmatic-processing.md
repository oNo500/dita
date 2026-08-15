# 07 · 程序化处理

> 二次开发的第三条路：**不用 DITA-OT，自己写代码消费 / 生产 DITA**。

---

## 1. 第一个决定：处理"解析前"还是"解析后"的 DITA

这是所有 DITA 编程的分水岭。

| | 源文件（authoring source） | preprocess 后 |
|---|---|---|
| conref | 未展开，是引用 | 已展开成实际内容 |
| keyref / keytext | 未解析 | 已替换成 href / 文本 |
| include | 未拉取 | 已插入外部内容 |
| 条件化 | 全部内容都在 | 已按 DITAVAL 过滤 |
| map 层级 | 分散在多个 map | 已合并 |
| `@class` | **不在文件里**（默认属性） | **已写进文件** |
| 适合做 | 编辑器、内容管理、复用分析 | 检索索引、格式转换、内容分析、发布 |

**90% 的场景你想要的是 preprocess 后的结果**。不要自己实现 conref / keyref 解析 —— 那是个深坑（间接引用、循环检测、key scope、conref push、泛化匹配）。

获取 preprocess 结果的两种方式：

```bash
# 方式 A：命令行，产出解析后的 DITA 文件
dita -i root.ditamap -f dita -o ./resolved

# 方式 B：保留临时目录
dita -i root.ditamap -f html5 -o /tmp/out --temp=./tmp --debug
```

---

## 2. 第二个决定：怎么拿到 `@class` 和 `@specializations`

**这两个属性的值由 schema 声明的默认值提供，创作时通常不写在源文件里。**

```python
# ❌ 两个都拿不到
tree = etree.parse("topic.dita")
tree.getroot().get("class")            # None
tree.getroot().get("specializations")  # None
```

### 解法 A：开启 DTD 加载

```python
from lxml import etree

parser = etree.XMLParser(
    load_dtd=True,
    attribute_defaults=True,   # ★ 补齐默认属性
    resolve_entities=True,
    no_network=True,
)
tree = etree.parse("topic.dita", parser)
tree.getroot().get("class")   # "- topic/topic concept/concept "
```

需要能解析到 schema。用 XML catalog 指向本地 DITA-OT，避免每次联网：

```bash
export XML_CATALOG_FILES=/path/to/dita-ot-4.4/catalog-dita.xml
```

> **注意：这条路依赖 DTD，不能用 RNG。**
>
> lxml 的 `attribute_defaults=True` 只对 DTD 生效。RNG 的默认属性值来自 **RELAX NG DTD Compatibility** 注解（`dita:defaultValue`），多数 RNG 校验器不会据此注入属性 —— 校验能过，但 `@class` 仍然是 `None`。
>
> 所以走 Python 路线时，文档的 doctype 要声明 **DTD** 版本的外壳；若你的内容用的是 RNG，改走解法 B。
>
> 这与"RNG 是规范性版本"并不矛盾：规范性说的是**两者有出入时以谁为准**，不是说工具必须用哪个。见 [00 第 6 节](00-roles-and-boundaries.md)。

### 解法 B：处理 preprocess 后的文件

DITA-OT 的中间产物**已经把 `@class` 写进文件了**，可以裸解析。最省事、也不受语法格式限制的一条路。

---

## 3. 按 `@class` 编程

```python
def is_class(el, cls):
    """cls 形如 'topic/li'，注意前后空格"""
    c = el.get("class")
    return c is not None and f" {cls} " in c

# 找出所有 li 及其任何专门化（step、chdeschd…）
for el in tree.iter():
    if is_class(el, "topic/li"):
        ...
```

XPath 版：

```python
tree.xpath("//*[contains(@class, ' topic/section ')]")
```

**判断 topic 类型**：

```python
def topic_type(root):
    """返回最特化的类型名"""
    c = (root.get("class") or "").strip()
    if not c.startswith("-"):
        return None
    return c.split()[-1].split("/")[-1]   # "- topic/topic task/task " → "task"
```

**解析 `@specializations`**：

```python
def attr_specializations(root):
    """返回 {属性名: 祖先链} —— 例如 {'hardwarePlatform': ['props','platform']}"""
    result = {}
    for token in (root.get("specializations") or "").split():
        parts = token.lstrip("@").split("/")   # "@props/platform/hardwarePlatform"
        if len(parts) >= 2:
            result[parts[-1]] = parts[:-1]
    return result
```

有了这个就能自己实现属性泛化：`hardwarePlatform="x86"` 逐级退回 `platform="x86"` → `props="platform(x86)"`。

---

## 4. Java：把 DITA-OT 当库用

```java
import org.dita.dost.ProcessorFactory;
import org.dita.dost.Processor;
import java.io.File;

ProcessorFactory pf = ProcessorFactory.newInstance(new File("/path/to/dita-ot-4.4"));
pf.setBaseTempDir(new File("/tmp/dita-temp"));

Processor p = pf.newProcessor("html5")
    .setInput(new File("root.ditamap"))
    .setOutputDir(new File("out"))
    .setProperty("args.filter", "linux.ditaval")
    .cleanOnFailure(false);

p.run();
```

适用场景：文档服务、CI 服务、CCMS 集成 —— 避免为每篇文档 fork 一个 JVM 进程。

### 读 `.job.xml`

临时目录里的 `.job.xml` 是 DITA-OT 的"编译数据库"。自己写分析工具时直接读它，比重新扫盘可靠：

```java
import org.dita.dost.util.Job;
Job job = new Job(new File("/tmp/dita-temp"));
job.getFileInfo().forEach(fi ->
    System.out.println(fi.file + " format=" + fi.format + " role=" + fi.isResourceOnly));
```

---

## 5. 生成 DITA

从结构化数据（OpenAPI、数据库、代码注释）生成 reference topic 是很好的落地场景。

```python
from lxml import etree

DOCTYPE = ('<!DOCTYPE reference PUBLIC "-//OASIS//DTD DITA 2.0 Reference//EN" '
           '"reference.dtd">')

def make_api_topic(endpoint):
    ref = etree.Element("reference", id=endpoint["id"])
    etree.SubElement(ref, "title").text = endpoint["summary"]
    etree.SubElement(ref, "shortdesc").text = endpoint["description"]
    body = etree.SubElement(ref, "refbody")

    sec = etree.SubElement(body, "section")
    etree.SubElement(sec, "title").text = "请求"
    p = etree.SubElement(sec, "p")
    etree.SubElement(p, "codeph").text = f'{endpoint["method"]} {endpoint["path"]}'

    st = etree.SubElement(body, "simpletable", relcolwidth="1* 1* 3*")
    hd = etree.SubElement(st, "sthead")
    for h in ("参数", "类型", "说明"):
        etree.SubElement(hd, "stentry").text = h
    for param in endpoint["params"]:
        row = etree.SubElement(st, "strow")
        for v in (param["name"], param["type"], param["desc"]):
            etree.SubElement(row, "stentry").text = v

    return etree.tostring(ref, xml_declaration=True, encoding="UTF-8",
                          doctype=DOCTYPE, pretty_print=True)
```

**关键**：给生成内容的 `@id` 必须**稳定**（从源数据派生，不要用序号）—— 否则每次重新生成，所有 conref 和外链都会断。

生成时不要手写 `@class` / `@specializations`，让 schema 注入。

---

## 6. DITA ↔ Markdown

### 官方路线：org.lwdita 插件

```bash
dita install org.lwdita
```

**DITA → Markdown**：

```bash
dita -i root.ditamap -f markdown         -o out   # 通用
dita -i root.ditamap -f markdown_github  -o out   # GitHub 风味
dita -i root.ditamap -f markdown_gitbook -o out   # GitBook（带 SUMMARY.md）
```

**这一步是有损的**。Markdown 没有 concept/task/reference 语义、没有 `@class`、没有条件化属性。转出去就回不来了。

**Markdown → DITA**：lwdita 支持把 `.md` 直接当 topic 引用：

```xml
<map>
  <topicref href="notes/quickstart.md" format="markdown"/>
</map>
```

DITA-OT 在 preprocess 时把它转成 DITA topic 参与构建。约定：第一个 `#` 标题 → `<title>`，紧跟的一段 → `<shortdesc>`，YAML front matter 可提供 `id` 等元数据。

**这条路很实用**：核心内容用 DITA 保证复用和条件化，边缘内容（release notes、FAQ）用 Markdown 降低门槛，一套 map 统一构建。

> lwdita 对 2.0 元素的支持程度要实测 —— 它的映射表是按 DITA 词汇表写的，新元素（`<div>` `<keytext>` `<titlealt>`）不一定都覆盖到。

### 自己写转换器时的映射难点

| DITA | Markdown | 难点 |
|---|---|---|
| `<task>` 的 `<steps>` | 有序列表 | `<cmd>`/`<info>`/`<stepresult>` 的层级信息丢失 |
| 嵌套 `<steps>` | 嵌套列表 | 能转，但语义降级 |
| `<conref>` | 无 | 只能展开（失去复用） |
| `<ph keyref>` / `<keytext>` | 无 | 只能展开成字面量 |
| `<include>` | 无 | 只能展开 |
| 条件化属性 | 无 | 只能预过滤，或输出成 HTML 注释/自定义容器 |
| `<titlealt>` 各 role | front matter | 可映射到 YAML 字段 |
| `<simpletable>` | 表格 | 能转 |
| `<table>`（CALS，带跨行跨列） | 表格 | **合并单元格转不了**，要降级成 HTML table |
| `<xref href="a.dita#t/e">` | 链接 | 需重算文件名映射（.dita → .md，锚点也变） |
| 嵌套 topic | 无 | 要么拆文件，要么降标题层级 |

**建议**：`DITA → Markdown` 做成"发布通道"（单向、可重跑），永远不要试图往回同步。双向同步在这个语义落差下必然产生数据丢失。

---

## 7. 一个真实的坑：中间产物的版本污染

DITA-OT 的 `org.dita.normalize` 插件有过一个已知问题：**normalized 输出里带上了 `@specializations`**，而这个属性只在 DITA 2.0 里有效 —— 结果产出的"标准化 DITA"在旧版校验下不合法。

通用形态是：**中间产物混入了目标版本没有的属性/元素**。写自己的转换管道时，要在输出阶段**显式过滤掉不属于目标 schema 的东西**，而不是假设"从 DITA 来的一定是合法 DITA"。

特别地，如果你的管道会重写文档，**记得丢掉 `@class` 和 `@specializations`**，让目标 schema 重新注入 —— 留着旧值会导致派生链错乱。

---

## 8. 工具速查

| 需求 | 工具 |
|---|---|
| XSLT 3.0 | Saxon-HE（DITA-OT 自带） |
| RNG 校验 | Jing |
| DTD 校验 | xmllint、Xerces |
| DITA 层面校验 | `dita validate`（4.3+） |
| Python 解析 | lxml（必须 `attribute_defaults=True`） |
| Java 解析 | DITA-OT 的 `org.dita.dost` + Saxon |
| Node.js | 完整 DITA 无成熟库，用 `libxmljs` / `saxon-js` 自己搭；**LwDITA（仅 XDITA 词汇表）** 有活跃的 TypeScript 实现（Evolved Binary 的 `@evolvedbinary/lwdita-xdita` + `lwdita-ast`，兼容 LwDITA v0.3） |
| 链接检查 | 直接跑构建，DITA-OT 会报断链 |
| DITA ↔ Markdown | org.lwdita |

---

→ 下一步：[08-practical-advice.md](08-practical-advice.md)

---

## 来源

**已逐页核对（2026-08）**

- [specializations 属性规则与语法](https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute) — `@specializations` 由 doctype shell 默认注入而非作者手写（本篇"默认属性陷阱"的依据）；形式语法用于 `attr_specializations()` 的解析逻辑
- [DTD 公共标识符](https://dita-lang.org/dita/non-normative/dtd-public-identifiers) — 生成 DITA 时 DOCTYPE 声明的写法

**部分核对**

- `org.dita.normalize` 输出混入 `@specializations` 导致跨版本不合法 —— 来自 dita-ot 相关 issue 的检索结果标题与摘要，**未打开 issue 逐条核对**。正文将其作为"版本污染"这一通用问题的例证，结论本身独立成立

**未逐页核对，来自通用实践**

- lxml 需要 `load_dtd=True` + `attribute_defaults=True` 才能取到默认属性（Python XML 处理的通用行为）
- **`attribute_defaults` 只对 DTD 生效、多数 RNG 校验器不注入 RELAX NG DTD Compatibility 声明的默认属性值** —— 来自使用经验，未逐工具验证；结论方向可靠，具体到某个校验器请自行实测
- `XML_CATALOG_FILES` 环境变量的用法
- DITA-OT 的 Java API（`ProcessorFactory` / `Processor` / `Job`）示例代码 —— 依 DITA-OT 版本可能有出入，使用前请对照你所装版本的 API
- `org.lwdita` 的 transtype 名称（`markdown` / `markdown_github` / `markdown_gitbook`）与 Markdown 作为 topic 引用的约定
- Node.js 一行中的 LwDITA TypeScript 实现已核对（[evolvedbinary/lwdita](https://github.com/evolvedbinary/lwdita)，2026-08：活跃维护、npm 发布、兼容 LwDITA v0.3.0.2；仅覆盖 XDITA，非完整 DITA）
- DITA↔Markdown 映射难点表、生成 DITA 的代码示例、工具速查表（判断性内容）

> ⚠️ 本篇未覆盖规范定义的**处理模型**（元数据级联、确定属性有效值、conref 的属性解析规则）。自研处理逻辑若不遵循该模型，结果会与 DITA-OT 不一致。见 [08 实践建议](08-practical-advice.md)。
