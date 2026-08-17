# 04 · 工具链与构建

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/toolchain/dita-ot-quickstart.dita` ← §1–2，并吸收 README「30 秒上手」；`kb/topics/dita/toolchain/preprocess-pipeline.dita` ← §3；`kb/topics/dita/toolchain/validation-schematron.dita` ← §4；`kb/topics/dita/toolchain/editors-pdf-reality.dita` ← §5–6；`kb/topics/dita/toolchain/project-files.dita` ← §2 的「项目文件（`--project`）」；不迁小节：无），本文冻结为调研档案，不再更新。

> **声明修订（2026-08-16，Task 13b 切分复议后）**：原声明写「§2 的项目文件小节不迁，留待 practice 簇 engineering-ci 承接」。该小节先随 §工程化建议 进了 `engineering-ci`，13b 复议时又从中拆出为独立篇 `toolchain/project-files.dita`（上游 DITA-OT 4.4 docsrc 确有 Publishing with project files 节点）。故 §2 已全部迁出，无不迁小节。

## 1. 现实前提

**DITA 2.0 仍是 beta（v2.0-beta03，2026-07-02），尚未成为 OASIS 正式标准。**

这决定了工具链的样子：

| | 状态 |
|---|---|
| **DITA-OT 4.4**（2026-01-31） | 基于 **2026-01-25 的 2.0 草案语法文件**提供 preview 支持。是目前最完整的 2.0 实现 |
| **beta03 vs DITA-OT 基线** | beta03 比 4.4 的基线新 ~5 个月，**两者可能有差异**。遇到怪问题先想到这一点 |
| **Oxygen / CCMS / 第三方插件** | 2.0 支持程度参差，**必须逐个验证** |

> **规范支持 ≠ 工具链支持。** 投入内容之前，先用最小样例把你实际要用的工具跑一遍。

---

## 2. DITA-OT

开源（Apache 2.0），Java + Ant + XSLT(Saxon)。**4.4 要求 Java 17+。**

### 安装

```bash
# macOS
brew install dita-ot

# 或手动
curl -LO https://github.com/dita-ot/dita-ot/releases/download/4.4/dita-ot-4.4.zip
unzip dita-ot-4.4.zip -d ~/tools
export PATH="$HOME/tools/dita-ot-4.4/bin:$PATH"

dita --version
```

### 怎么让 DITA-OT 按 2.0 处理文档

**靠文档的 doctype 声明。** DITA-OT 通过 XML catalog 解析到 2.0 的语法文件，从而识别版本并走对应的处理分支。

公共标识符命名规范：

```
-//OASIS//DTD DITA <版本> <信息类型>//EN
```

- 版本可以是具体的 `2.0`
- 也可以是 `2.x`（最新的 2.x）
- **也可以整个省略** —— 等价于最新的 2.x

```xml
<!DOCTYPE topic PUBLIC "-//OASIS//DTD DITA 2.0 Topic//EN" "topic.dtd">
<!DOCTYPE task  PUBLIC "-//OASIS//DTD DITA 2.0 Task//EN"  "task.dtd">
<!DOCTYPE map   PUBLIC "-//OASIS//DTD DITA 2.0 Map//EN"   "map.dtd">
```

RNG 走 URN：

```
urn:pubid:oasis:names:tc:dita:rng:<信息类型>.rng:<版本>
```

> **确切字符串以你所用 DITA-OT 随附的语法文件和 `catalog-dita.xml` 为准** —— 草案期间这些会动。装好后直接去 DITA-OT 的 `plugins/` 下找 2.0 语法插件目录，看实际的 catalog 条目最可靠。

### 常用命令

```bash
# 脚手架（init / validate 是 4.3+ 的预览特性）
dita init --list
dita init --template=<模板名> --output=docs

# 发布前校验（比 xmllint 更懂 DITA，能查断链）
dita validate --input=docs/root.ditamap

# 构建
dita --input=docs/root.ditamap --format=html5 --output=out

# 带条件过滤
dita -i root.ditamap -f html5 -o out --filter=linux.ditaval

# 常用输出格式
dita -i m.ditamap -f html5     -o out    # 静态站
dita -i m.ditamap -f pdf       -o out    # PDF（默认走 Apache FOP）
dita -i m.ditamap -f markdown  -o out    # 需装 org.lwdita 插件
dita -i m.ditamap -f dita      -o out    # 只跑 preprocess，产出解析后的 DITA ← 便于调试

# 用 .properties 管理复杂构建
dita --propertyfile=build-cn.properties

# 插件管理
dita plugins
dita install <目录|zip|URL|注册表ID>
dita uninstall <plugin-id>
dita install                              # 无参数 = 重建集成配置（改了插件后必跑）
```

### 调试三件套

```bash
dita -i m.ditamap -f html5 -o out \
  --debug \                       # 保留临时目录 + 详细堆栈
  --temp=/tmp/dita-temp \         # 指定临时目录位置（否则随机）
  -v                              # verbose
```

`--temp` + `--debug` 是**排查 conref / keyref / 条件化问题的唯一可靠手段**：去临时目录里看每个 preprocess 阶段之后的中间 XML，一眼就知道是哪一步没按预期走。

另一个技巧：`-f dita` 把 preprocess 结果直接输出成正常的 DITA 文件，可以用编辑器打开检查所有引用是否解析正确。

### 项目文件（`--project`）—— 多交付物构建的正式机制

一个产品要出「HTML×3 个平台变体 + PDF」时，与其写 shell 循环逐个跑 `dita` 命令，不如用**项目文件**把全部交付物声明在一处：

```xml
<!-- project.xml -->
<project xmlns="https://www.dita-ot.org/project">
  <context id="guide" name="用户手册">
    <input href="maps/user-guide.ditamap"/>
  </context>

  <deliverable name="HTML（Linux 版）">
    <context idref="guide">
      <profile><ditaval href="filters/linux.ditaval"/></profile>
    </context>
    <output href="out/html-linux"/>
    <publication transtype="html5">
      <param name="args.css" href="brand.css"/>
    </publication>
  </deliverable>

  <deliverable name="PDF">
    <context idref="guide"/>
    <output href="out/pdf"/>
    <publication transtype="pdf"/>
  </deliverable>
</project>
```

```bash
dita --project=project.xml                          # 构建全部交付物
dita --project=project.xml --deliverable=PDF        # 只构建其中一个
```

要点：

- 三种等价格式：**XML（规范性）**、JSON、YAML —— 后两种更紧凑，适合手写
- 结构是三层：**context**（输入 map + 过滤条件）→ **publication**（transtype + 参数）→ **deliverable**（context × publication × 输出目录）
- context 和 publication 可被多个 deliverable 用 `idref` 复用，参数支持覆盖 —— **变体矩阵声明一次，处处引用**
- CI 里一条 `dita --project` 顶替一串 for 循环（对比 [08](08-practical-advice.md) 的 CI 脚本 —— 那是无项目文件时的写法）

---

## 3. preprocess 流水线

理解这条流水线是二次开发的前提 —— 你的插件挂在哪一环，决定它能看到什么。

```
gen-list      扫描 map，建立所有文件的清单（.job.xml）
     ↓
debug-filter  注入调试信息 + 执行 DITAVAL 过滤
     ↓
copy-files    拷贝非 DITA 资源（图片等）到临时目录
     ↓
mapref        展开 mapref，把多个 map 合成一个
     ↓
keyref        解析 @keyref / @conkeyref / @keys，替换成实际 href 和文本（含 <keytext>）
     ↓
conrefpush    执行 conref push（@conaction）
     ↓
conref        解析 @conref / @conrefend，把内容拉过来
     ↓
profile       再次条件过滤，处理 conref 拉进来的内容
     ↓
topicpull     从目标 topic 拉取标题填充空的 <xref>/<link>
     ↓
mappull       从 topic 拉取标题填充 map 里的替代标题
     ↓
maplink       根据 map 层级和 reltable 生成 related-links
     ↓
chunk         按 @chunk 合并（combine）/ 拆分（split）
     ↓
move-meta-entries   把 map 上的元数据下沉到 topic
     ↓
flag-module   处理 DITAVAL 的 flag 动作
     ↓
──────── 以上是 preprocess，以下是各 transtype 自己的 XSLT ────────
     ↓
html5 / pdf2 / ...
```

> DITA-OT 3.x 起引入了纯 XSLT 重写的 **preprocess2**，减少 Java/Ant 往返、大幅提速。4.x 里部分 transtype 默认走 preprocess2。行为基本兼容，但若你的插件挂在某个 Ant target 上，升级时要确认那个 target 还在。

### `.job.xml`

临时目录里的 `.job.xml` 是 DITA-OT 的"编译数据库"，记录所有输入文件、角色（`normal` / `resource-only`）、格式、是否有 conref。写扩展时直接读它拿全量文件清单，不用自己再扫盘。

---

## 4. 校验

DITA 2.0 的 OASIS 发行提供 **RNG 与 DTD 两种**语法文件，**RNG 是规范性版本**（两者有出入时以 RNG 为准）。**2.0 不再随发行提供 XSD**，需要的话可自行从 RNG 生成。

| 方式 | 命令 |
|---|---|
| DTD | `xmllint --noout --valid topic.dita` |
| RNG | `java -jar jing.jar topic.rng topic.dita` |
| DITA 层面 | `dita validate --input=root.ditamap` |
| 业务规则 | Schematron（可查"task 的 shortdesc 不能为空"这类） |

**做专门化时优先基于 RNG** —— 语法可读得多，且能生成 DTD/XSD 给下游用。

### Schematron：业务规则校验的实战形态

schema 管"结构合法"，管不了"team 规定 task 必须有 shortdesc"这类**业务规则**（对照 [00](00-roles-and-boundaries.md) 的"schema 管什么不管什么"表）。约束模块能解决其中一部分（结构性收紧），剩下的靠 Schematron —— 它用 XPath 断言表达规则：

```xml
<!-- rules.sch -->
<schema xmlns="http://purl.oclc.org/dsdl/schematron" queryBinding="xslt2">
  <pattern>
    <rule context="*[contains(@class, ' task/task ')]">
      <assert test="*[contains(@class, ' topic/shortdesc ')]
                    | *[contains(@class, ' topic/abstract ')]">
        task 必须有 shortdesc 或 abstract（用于搜索摘要）。
      </assert>
    </rule>
    <rule context="*[contains(@class, ' topic/xref ')]">
      <report test="@scope='external' and not(@format)">
        外部链接应显式标注 @format。
      </report>
    </rule>
  </pattern>
</schema>
```

注意规则里的 context 同样**按 `@class` 匹配** —— 这样规则自动覆盖专门化元素。但这意味着 Schematron 要跑在**能拿到 `@class` 的文档**上：要么经 DTD 校验解析，要么跑在 preprocess 后的中间产物上（见 [07](07-programmatic-processing.md)）。

运行方式：

| 场景 | 方式 |
|---|---|
| 编辑时 | Oxygen 原生支持，可关联到框架配置，边写边报 |
| CI | Schematron 本质是编译成 XSLT 执行 —— 用 SchXslt / ph-schematron 等实现跑批 |
| DITA-OT 内 | 无内置支持，可自写插件在 preprocess 后挂一步（见 [06](06-dita-ot-plugins.md) 的 Ant 挂钩） |

**与约束模块怎么分工**：能用约束表达的（元素禁用、必填子元素）优先用约束 —— 编辑器输入时就拦截；跨元素、带条件的规则（"external 链接必须有 @format"）才用 Schematron。

### CI 里的最小校验

```bash
#!/bin/bash
set -e
# 1. XML 良构 + DTD 有效性
find . -name '*.dita' -o -name '*.ditamap' | xargs -n1 xmllint --noout --valid
# 2. DITA 层面校验
dita validate --input=root.ditamap
# 3. 构建（最严，会暴露 conref/keyref 断链、条件化后不合法等）
dita -i root.ditamap -f html5 -o /tmp/ci-out
# 4. 每个变体都构建一遍
for f in filters/*.ditaval; do
  dita -i root.ditamap -f html5 -o "/tmp/ci-$(basename "$f" .ditaval)" --filter="$f"
done
```

第 3 步很重要：**schema 校验查不出 conref 指向不存在的文件**，只有真正构建会报。

第 4 步常被忽略：**只构建默认变体，会漏掉某个条件组合下内容变不合法的问题**。

---

## 5. 编辑器

| 工具 | 说明 |
|---|---|
| **Oxygen XML Editor / Author** | 事实标准。商业。所见即所得、conref/keyref 补全、DITA-OT 集成、差异合并。**用 2.0 前务必确认版本对 2.0 语法文件的支持** |
| **VS Code + DITA 扩展** | 社区扩展（语法高亮、片段、构建任务）。够写、不够管。适合开发者自己维护文档 |
| **Emacs nXML / vim** | RNG 驱动的补全，够用 |
| **CCMS**（IXIASOFT / Tridion Docs / Paligo / easyDITA） | 管版本/工作流/翻译/复用追踪。**2.0 支持是最大的不确定项** |

> 判断：只有你 + 几个工程师写文档 → VS Code 足够。有专职 technical writer 团队 → Oxygen。上百人 + 多语言 + 合规审计 → 才需要 CCMS。
>
> **用 2.0 的话，工具链自控（DITA-OT + 文本编辑器）是最稳的路线** —— 商业工具的 2.0 支持不由你决定。

---

## 6. 输出 PDF 的现实

DITA-OT 默认的 `pdf` transtype 走 **Apache FOP**（XSL-FO），中文排版效果一般，字体配置繁琐。

选项：

1. **FOP + 自配中文字体**（免费，需写 `fop.xconf` 注册字体，效果凑合）
2. **Antenna House / RenderX XEP**（商业 FO 引擎，排版质量高，DITA-OT 有官方对接）
3. **PDF via HTML**（用 Prince / WeasyPrint / Paged.js 从 HTML+CSS 出 PDF）—— **对熟悉 CSS 的人成本最低**，中文支持也最好

如果 PDF 是主要交付物且团队没有 XSL-FO 经验，方案 3 优先评估。

---

→ 下一步：[05-specialization.md](05-specialization.md)

---

## 来源

**已逐页核对（2026-08）**

- [DITA-OT 发布说明](https://www.dita-ot.org/dev/release-notes/) — DITA-OT **4.4**，发布日期 **2026-01-31**，要求 **Java 17 或更高**；含 DITA 2.0 preview 支持与 JSON 日志
- [DITA 2.0 preview 支持](https://www.dita-ot.org/dev/reference/dita-v2-0-support.html) — 基于 **2026-01-25** 的 2.0 DTD/RELAX NG 草案语法文件；并列出 3.5→4.4 各版本加入的 2.0 特性
- [oasis-tcs/dita 发布页](https://github.com/oasis-tcs/dita/releases) — **v2.0-beta03（2026-07-02）**、beta02（2024-10-04）、beta01（2024-06-14）；均为 pre-release，未标注 OASIS 正式阶段
- [DITA 规范列表](https://www.dita-lang.org/specifications) — DITA 2.0 列为 draft；1.3/1.2/1.1/1.0 为 OASIS Standard
- [使用 dita 命令构建](https://www.dita-ot.org/dev/topics/build-using-dita-command) — `--input` / `--format` 必需，以及 `--output` `--filter` `--temp` `--debug` `--propertyfile` `--verbose` 等选项
- [使用项目文件](https://www.dita-ot.org/dev/topics/using-project-files) — 三种等价格式（**XML 为规范性**，JSON/YAML 为紧凑替代）；context / publication / deliverable 三层结构与 `idref` 复用；`dita --project=<文件>` 与 `--deliverable=<名称>` 的用法；publication 参数支持覆盖继承
- [DTD 公共标识符](https://dita-lang.org/dita/non-normative/dtd-public-identifiers) — 格式 `-//OASIS//DTD DITA <版本> <信息类型>//EN`；版本可为具体号、`2.x`、或省略（等价于最新 2.x）；RNG 走 `urn:pubid:oasis:names:tc:dita:rng:...`
- [dita-ot GitHub 发布页](https://github.com/dita-ot/dita-ot/releases) — 版本序列交叉核对

**部分核对**

- `dita init` / `dita validate` 为 **DITA-OT 4.3 引入的预览特性**，`dita init --list` 可列出模板 —— 来自 DITA-OT 4.3 发布说明的检索结果，未逐页核对；**模板名称未经验证**，故正文写作 `--template=<模板名>`

**未逐页核对，来自通用 DITA-OT 实践**

- preprocess 流水线各阶段的名称与顺序（来自实现观察，非规范定义；随版本会变，正文已提示用 `--debug` 亲自确认）
- `.job.xml` 的作用与内容
- `dita install` / `uninstall` / `plugins` 的行为，以及"改插件后必须重跑 `dita install`"
- 编辑器与 CCMS 的对比、PDF 输出三条路线的取舍（判断性内容）
- Schematron 一节：示例规则、SchXslt / ph-schematron 等运行方式、Oxygen 的原生支持、与约束模块的分工建议 —— 均来自通用实践；"DITA-OT 无内置 Schematron 支持"未逐版本核对
