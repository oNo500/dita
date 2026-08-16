# notes → kb 迁移实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 research/notes 16 篇 DITA 2.0 研究 + README 产物性内容 + kb-redesign 2 份未吸收设计迁为 67 篇 kb topic，全部通过现行规则（写作七条 + R1–R16）。

**Architecture:** 六簇子目录 + 域 map/簇 map 两级结构，挂 content-engineering 分支；单件流——每篇独立走"写作→校验→lint→挂 map→提交"循环；试点后有用户门，每簇完成有停点。

**Tech Stack:** DITA 2.0（RNG shell）、DITA-OT 4.4（`dita validate`）、dita-tools（lint/ia）、just、uv（term-normalize）。

**Spec:** `docs/superpowers/specs/2026-08-16-notes-to-kb-migration-design.md`（执行前通读，本计划的裁定都出自它）。

## Global Constraints

- 仓库根：`/home/dev/ws/projects/dita`。所有命令从仓库根执行；`just review` / `just ia` / `just links` 在根跑。
- **每篇 topic `maturity="draft"`**。晋 curated 只能由用户审后决定——执行者永远不写 curated。
- 文件名 ASCII kebab；`id` = 文件名去 `.dita`；标题按写作规则七定稿（名词短语、含定位词、对齐 OASIS 规范 / DITA-OT 文档类目），spec 清单里的标题是暂定值。
- 事实只来自对应笔记已核对的内容；笔记标注"未核对"的点不得写成断言。来源节固定两段：`<p><b>事实</b>：…</p><p><b>判断</b>：…</p>`，正文不写日期（日期只在 prolog reviewed）。
- 禁：标题回声（"本页是…"）、场景化开头、排比、加粗每节 >2、破折号每段 >1、口语词（凑合/挂个/塞进/出事/拦住/就该）、"此刻"式时间自指。提及被禁词或产品名作例子时包 `<keyword>`。
- 术语首现用 `<term keyref="term-x">`；新词第二篇用到时才建 glossentry（模板 D）并挂 `kb/maps/glossary.ditamap`。
- 每篇一个 commit，消息 `content(dita): <slug> — <一句话>`；基础设施改动单独 commit。
- 破坏性操作零容忍：本计划只新增文件和定点编辑，不删不改已有 topic（Task 13 的 writing-sourcing 定点修改除外）。

### 通用工作循环（"写一篇"的定义，下文各任务的 per-topic 步骤即执行本循环）

1. 读源笔记对应小节（任务表里给了小节号），按模板起草到目标路径
2. `dita validate --input=kb/topics/dita/<簇>/<slug>.dita` → 必须 0 error
3. `dita-tools lint --vocab kb/vocab/subjectScheme.ditamap kb/topics/dita` → 本篇 0 error 0 warning
4. 把 topicref 加进所属簇 map（按清单顺序；簇 map 位于 maps/domains/dita/，故 href 为 `../../../topics/dita/<簇>/<slug>.dita`）
5. `cd kb && uv run --script scripts/term-normalize.py && cd ..` → 本篇无裸术语建议（有则回填 keyref）
6. `git add -A && git commit -m "content(dita): <slug> — <一句>"`

### 模板 A：concept 题材（best-practice / deep-dive / tech-landscape）

簇内文件 shell 路径为 `../../../schema/…`；分支根（landscape/resources）少一层，用 `../../schema/…`。

```xml
<?xml version="1.0" encoding="UTF-8"?>
<?xml-model href="../../../schema/concept-kb.rng" schematypens="http://relaxng.org/ns/structure/1.0"?>
<!-- 迁自 research/notes/<篇>§<节>（2026-08-XX）。 -->
<concept id="<slug>" xml:lang="zh-CN"
         outputclass="<题材>"
         maturity="draft" volatility="<stable|volatile>"
         dimension="<dim-…>">
  <title>…</title>
  <shortdesc>…（与标题并排读必须有增量；禁"本页/本篇"开头）</shortdesc>
  <prolog>
    <source href="<笔记来源节里的权威 URL>" scope="external"/>
    <data name="domain" value="dita"/>
    <data name="reviewed" value="<沿用笔记来源节的核对日期>"/>
  </prolog>
  <conbody>
    <!-- best-practice 必需节（R13，标题前缀匹配）：场景/做法/理由/反例/边界 + 来源
         deep-dive / tech-landscape 无必需节，但一律以 来源 节收尾 -->
  </conbody>
</concept>
```

> **已失效（2026-08-16 Task 2/6 裁定）**：how-to 与 quickstart 均改绑 concept，task 类型退役、task-kb.rng 已删除。本节仅存档。

### 模板 B：task 题材（how-to / quickstart）

```xml
<?xml version="1.0" encoding="UTF-8"?>
<?xml-model href="../../../schema/task-kb.rng" schematypens="http://relaxng.org/ns/structure/1.0"?>
<task id="<slug>" xml:lang="zh-CN" outputclass="<how-to|quickstart>"
      maturity="draft" volatility="<…>" dimension="<dim-…>">
  <title>…</title>
  <shortdesc>…</shortdesc>
  <prolog>（同模板 A）</prolog>
  <taskbody>
    <!-- quickstart 必需节（R13）：目标/前置/步骤；how-to 无必需节。
         来源 节的落位以 Task 2 试样验证的结论为准 -->
  </taskbody>
</task>
```

### 模板 C：reference 题材（cheatsheet / curated-resources）

同现有 `kb/topics/ai/claude-code-hooks.dita` 的形制：`reference-kb.rng` shell、refbody、section+simpletable；注意 **refbody 不允许裸 `<p>`**，导语要包 `<section><title>定位</title>`。

### 模板 D：glossentry（双语）

同现有 `kb/topics/glossary/term-mcp.dita` 形制：`glossentry-kb.rng`、`<glossterm>`首选词、`<glossSynonym xml:lang="en">`英文名、挂 `kb/maps/glossary.ditamap`。

### 冻结声明（笔记全部小节处置完时加在笔记标题下）

```markdown
> **已迁移（2026-08-XX）**：正本已迁 kb（<topic 文件清单>；不迁小节：<清单或"无">），本文冻结为调研档案，不再更新。
```

---

### Task 1: 词表登记（主题树挂点）

**Files:**
- Modify: `kb/vocab/subjectScheme.ditamap`（structured-content 分支下）

**Interfaces:**
- Produces: 主题节点 `dita`，供 topic 的 `<data name="domain" value="dita"/>` 与 ia 骨架归位使用

- [ ] **Step 1: 核实挂点现状**

Run: `rg -n "structured-content" kb/vocab/subjectScheme.ditamap`
确认 content-engineering 下存在 structured-content 节点，并确认其下**尚无** dita 子节点。

- [ ] **Step 2: 登记 dita 节点**

在 structured-content 的 subjectdef 内加子节点（缩进对齐邻近节点）：

```xml
<subjectdef keys="dita">
  <!-- DITA 标准知识（2026-08 迁自 research/notes，见 docs/superpowers/specs/2026-08-16-notes-to-kb-migration-design.md） -->
</subjectdef>
```

- [ ] **Step 3: 验证**

Run: `just review && just ia`
Expected: 均绿（词表可解析，无新告警）。

- [ ] **Step 4: Commit**

```bash
git add kb/vocab/subjectScheme.ditamap
git commit -m "feat(vocab): register dita subject under structured-content"
```

### Task 2: task-kb.rng shell

**Files:**
- Create: `kb/schema/task-kb.rng`
- Test: 试样文件 `/tmp/claude-任意/task-specimen.dita`（用后即弃，不入库）

**Interfaces:**
- Produces: task shell，模板 B 的 `?xml-model` 指向它；带 maturity/volatility/dimension/tool 四个属性域

- [ ] **Step 1: 读现有 shell 作模板**

Read `kb/schema/concept-kb.rng` 全文，记下其结构：外部引用哪些 OASIS RNG 模块、四个属性域（maturityAttDomain / volatilityAttDomain / dimensionAttDomain / toolAttDomain）怎么 include。

- [ ] **Step 2: 写 task-kb.rng**

复制 concept-kb.rng，把 concept 模块引用换成 task 模块（OASIS RNG urn 中 `concept.rng` → `task.rng`，模块名对应替换），四个属性域 include 原样保留，注释头写明"task shell：how-to / quickstart 题材用（2026-08-16 迁移计划 Task 2 建）"。

- [ ] **Step 3: 试样验证（含 来源 节落位试验）**

写试样 task：含 shortdesc、prolog、taskbody 内 `context → steps（2 步）→ 尾部 <section><title>来源</title>`。

Run: `dita validate --input=<试样路径>`

- 通过 → 记录"taskbody 允许尾部 section"，模板 B 的来源节照 concept 写法
- 报 section 不允许 → **停，向用户报告**：给出两个选项（来源节放 steps 之前 / how-to 改绑 concept 类型）等裁定，不得自行选择

- [ ] **Step 4: lint 冒烟**

把试样临时拷入 `kb/topics/`（随后删除），跑 `dita-tools lint --vocab kb/vocab/subjectScheme.ditamap kb/topics`，确认 lint 对 task 根正常执行 R12（genre 检查）而非跳过或崩溃。删除试样。

- [ ] **Step 5: Commit**

```bash
git add kb/schema/task-kb.rng
git commit -m "feat(schema): task-kb shell for how-to and quickstart genres"
```

### Task 3: 目录、7 个 map、挂载

**Files:**
- Create: `kb/topics/dita/{principles,core-model,conditional,architecture,toolchain,practice}/`（目录）
- Create: `kb/maps/domains/dita.ditamap`、`kb/maps/domains/dita/{principles,core-model,conditional,architecture,toolchain,practice}.ditamap`
- Modify: `kb/maps/domains/content-engineering.ditamap`

**Interfaces:**
- Produces: 六个簇 map，后续任务把 topicref 加进对应簇 map；域 map 与挂载不再改动

- [ ] **Step 1: 建簇 map ×6**

每个簇 map 形制（以 toolchain 为例，title 分别为：原理 / 核心模型与复用 / 条件与元数据 / 架构机理 / 工具链与二开 / 工程实践）：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE map PUBLIC "-//OASIS//DTD DITA 2.0 Map//EN" "map.dtd">
<!-- dita 分支簇 map：工具链与二开（笔记 04/06/07）。topicref 由迁移任务逐篇追加。 -->
<map xml:lang="zh-CN">
  <title>工具链与二开</title>
</map>
```

- [ ] **Step 2: 建域 map**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE map PUBLIC "-//OASIS//DTD DITA 2.0 Map//EN" "map.dtd">
<!-- 领域 map：DITA（subjectScheme 挂点：content-engineering/structured-content/dita）
     六簇结构见 docs/superpowers/specs/2026-08-16-notes-to-kb-migration-design.md。
     navtitle 必须与被引簇 map 的 <title> 逐字一致（dita-tools 一致性检查兜底）。 -->
<map xml:lang="zh-CN">
  <title>DITA</title>
  <topicref href="../../topics/dita/dita-landscape.dita"/>
  <topicref href="../../topics/dita/dita-resources.dita"/>
  <topichead>
    <topicmeta><navtitle>原理</navtitle></topicmeta>
    <mapref href="dita/principles.ditamap"/>
  </topichead>
  <topichead>
    <topicmeta><navtitle>核心模型与复用</navtitle></topicmeta>
    <mapref href="dita/core-model.ditamap"/>
  </topichead>
  <topichead>
    <topicmeta><navtitle>条件与元数据</navtitle></topicmeta>
    <mapref href="dita/conditional.ditamap"/>
  </topichead>
  <topichead>
    <topicmeta><navtitle>架构机理</navtitle></topicmeta>
    <mapref href="dita/architecture.ditamap"/>
  </topichead>
  <topichead>
    <topicmeta><navtitle>工具链与二开</navtitle></topicmeta>
    <mapref href="dita/toolchain.ditamap"/>
  </topichead>
  <topichead>
    <topicmeta><navtitle>工程实践</navtitle></topicmeta>
    <mapref href="dita/practice.ditamap"/>
  </topichead>
</map>
```

两篇分支根 topic 此时尚不存在——先建 map，Task 4 建完 topic 再跑链接检查。

- [ ] **Step 3: 挂进 content-engineering.ditamap**

读该 map，在其 topicref 清单末尾（与现有条目同级）追加：

```xml
<topichead>
  <topicmeta><navtitle>DITA</navtitle></topicmeta>
  <mapref href="dita.ditamap"/>
</topichead>
```

- [ ] **Step 4: Commit**（验证推迟到 Task 4——此刻两个 dangling topicref 会让 links 红，属预期）

```bash
git add kb/maps
git commit -m "feat(maps): dita branch — domain map, six cluster maps, mounted under content-engineering"
```

### Task 4: 分支根两篇（全景 + 权威资源）

**Files:**
- Create: `kb/topics/dita/dita-landscape.dita`（tech-landscape，模板 A，shell 路径 `../../schema/`）
- Create: `kb/topics/dita/dita-resources.dita`（curated-resources，模板 C，shell 路径 `../../schema/`）

**Interfaces:**
- Consumes: research/README.md 的「版本现状」「权威资源」两节 + notes 00–15 的编排结构
- Produces: 全景的 planned-dimension 清单，`just ia` 据此算 dita 域覆盖度

- [ ] **Step 1: 写 dita-landscape**

要求（spec §六，逐条硬性）：prolog 带 `planned-dimension` 清单（依 notes 编排定维度，参考现有值集：dim-concept / dim-internals / dim-usage / dim-integration / dim-decision / dim-reference / dim-comparison / dim-evolution / dim-ecosystem 等，不够用先查 `rg 'dim-' kb/vocab/subjectScheme.ditamap` 再议）；正文只写维度框架的理由（为什么理解 DITA 是这些维度、每个维度的问题域），**零手抄覆盖状态**（不写"盲区"列、不写"n/m"）；版本现状表迁入（`volatility="volatile"`，reviewed 沿 README 的 2026-08 核对）；无标题回声。走通用工作循环 2–6（map 已在 Task 3 引它）。

- [ ] **Step 2: 写 dita-resources**

README「权威资源」节迁入：规范 / 工具链两组 simpletable（链接 + 一句定位），refbody 导语包 section。走工作循环 2–6。

- [ ] **Step 3: 骨架整体验证**

Run: `just review && just ia && just links`
Expected: 全绿；`just ia` 出现 dita 域与 planned 维度。

- [ ] **Step 4: 骨架完成报告**

向用户报告骨架形状（目录树 + ia 输出摘要），**不等批复可继续 Task 5**（试点本身就是给用户审的样张）。

### Task 5: 试点——笔记 02 拆三篇 ⚠️ 用户门

**Files:**
- Create: `kb/topics/dita/core-model/conref-pull-push.dita`（deep-dive，含 02§4 处理顺序）
- Create: `kb/topics/dita/core-model/keyref-variable-text.dita`（reference）
- Create: `kb/topics/dita/core-model/include-non-dita.dita`（how-to，模板 B——**首个 task topic**）
- Modify: `kb/maps/domains/dita/core-model.ditamap`、`research/notes/02-reuse.md`（冻结声明）

**Interfaces:**
- Consumes: `research/notes/02-reuse.md` 全文；Task 2 关于来源节落位的结论
- Produces: 三篇样张，用户审阅后裁定是否晋 curated、形状是否继续

- [ ] **Step 1–3: 三篇各走通用工作循环**（一篇一个 commit，按上面顺序）

- [ ] **Step 4: 冻结笔记 02**

02 的 §1–4 已全部处置（§4 并入 conref-pull-push），按冻结声明格式加头，列出三个 topic 文件。

```bash
git add research/notes/02-reuse.md
git commit -m "docs(research): freeze note 02 — migrated to three core-model topics"
```

- [ ] **Step 5: 🛑 停——用户门**

向用户提交：三篇正文 + lint/review 输出。等待裁定：①形状是否认可（不认可则修完再审，不进入 Task 6）；②是否晋 curated（晋级由用户改 maturity 或明确授权）。**未获准前禁止开始 Task 6。**

### Task 6: toolchain 簇（13 篇 ← 笔记 04 / 06 / 07）

**Files:**
- Create: `kb/topics/dita/toolchain/` 下 13 篇
- Modify: `kb/maps/domains/dita/toolchain.ditamap`、`research/notes/{04,06,07}-*.md`、`research/README.md`（30 秒上手节替换为指向 topic 的一行）

**Interfaces:**
- Consumes: 笔记 04/06/07 + README「30 秒上手」；Task 2 的 task shell

| slug | 标题（暂定） | 题材 | 源 |
|---|---|---|---|
| dita-ot-quickstart | DITA-OT 安装与 2.0 触发 | quickstart | 04§1–2 + README 30 秒 |
| preprocess-pipeline | preprocess 流水线 | deep-dive | 04§3 |
| validation-schematron | 校验与 Schematron | how-to | 04§4 |
| editors-pdf-reality | 编辑器与 PDF 输出的工具现实 | best-practice | 04§5–6 |
| plugin-extension-points | plugin.xml 与扩展点 | reference | 06§插件是什么+plugin.xml+现成插件 |
| xslt-override | XSLT 覆盖机制 | how-to | 06§完整示例+处理 2.0 元素 |
| custom-transtype | 自定义 transtype | how-to | 06§transtype+Ant+本地化 |
| plugin-debugging | 插件调试工作流 | how-to | 06§调试 |
| parsed-vs-source | 处理解析前还是解析后的 DITA | best-practice | 07§1 |
| programming-by-class | 按 @class 编程 | how-to | 07§2–3 |
| dita-ot-as-library | DITA-OT 当库用 | how-to | 07§4+§7 版本污染坑 |
| generate-and-convert | 生成 DITA 与 Markdown 互转 | how-to | 07§5–6 |
| processing-tools | 程序化处理工具速查 | cheatsheet | 07§8 |

- [ ] **Step 1–13: 逐篇走通用工作循环**（表序即写作序，一篇一 commit）
- [ ] **Step 14: 冻结笔记 04、06、07**（各加冻结声明，一个 commit）
- [ ] **Step 15: README 30 秒上手节替换**为 `> 已迁 kb：topics/dita/toolchain/dita-ot-quickstart.dita`（README 整体冻结在 Task 14）
- [ ] **Step 16: 簇验证** `just review && just ia && just links` 全绿
- [ ] **Step 17: 🛑 簇停点**——报告 13 篇清单 + 新建词表条目，等用户审阅意见后进 Task 7

### Task 7: practice 簇（10 篇 ← 笔记 08 / 13 / 15）

**Files:**
- Create: `kb/topics/dita/practice/` 下 10 篇
- Modify: `kb/maps/domains/dita/practice.ditamap`、`research/notes/{08,13,15}-*.md`

| slug | 标题（暂定） | 题材 | 源 |
|---|---|---|---|
| pitfalls | DITA 2.0 高频踩坑清单 | cheatsheet | 08§踩坑 |
| adoption-criteria | DITA 采用判据 | best-practice | 08§现实判断+对的+错的 |
| customization-cost-ladder | 二次开发成本阶梯 | best-practice | 08§成本阶梯 |
| engineering-ci | DITA 工程化与 CI | best-practice | 08§工程化 |
| translation-modularity | 模块化与翻译流程架构位 | deep-dive | 13§1+§3 |
| localization-attributes | 三个本地化属性 | reference | 13§2 |
| translation-antipatterns | 翻译下的复用反模式 | best-practice | 13§4–5 |
| dita-rag-fit | topic 自足性与 RAG 切块的论证 | deep-dive | 15§1+§5–6 |
| rag-parsed-content | RAG 喂解析前还是解析后 | best-practice | 15§2 |
| rag-chunking-metadata | RAG 切块粒度与检索元数据 | best-practice | 15§3–4 |

- [ ] **Step 1–10: 逐篇工作循环**
- [ ] **Step 11: 冻结 13、15；08 加冻结声明并列不迁小节**（§7 学习路径→map 职责、§8 覆盖情况→调研档案、§值得关注的方向→调研档案）
- [ ] **Step 12: 簇验证** 三命令全绿
- [ ] **Step 13: 🛑 簇停点**

### Task 8: conditional 簇（7 篇 ← 笔记 03 / 14）

**Files:**
- Create: `kb/topics/dita/conditional/` 下 7 篇
- Modify: `kb/maps/domains/dita/conditional.ditamap`、`research/notes/{03,14}-*.md`

| slug | 标题（暂定） | 题材 | 源 |
|---|---|---|---|
| profiling-ditaval | 条件属性与 DITAVAL | how-to | 03§1–2 |
| branch-filtering | 分支过滤 ditavalref | deep-dive | 03§3 |
| subjectscheme-taxonomy | subjectScheme 受控值与分类法 | deep-dive | **03§4 + 14§4 合并** |
| chunking | @chunk 分块 | reference | 03§6 |
| metadata-two-kinds | 过滤用与分类用元数据 | best-practice | 14§1+§5 |
| dublin-core-mapping | Dublin Core 与 DITA 字段对应 | reference | 14§2 |
| metadata-placement | 元数据五种放置机制 | best-practice | 14§3 |

03§5（属性专门化）**不在本簇写**——归 Task 9 的 attribute-specialization；14§6（RAG 注记）已由 Task 7 的 rag 三篇覆盖，冻结声明里列为"并入 practice 簇"。

- [ ] **Step 1–7: 逐篇工作循环**（合并篇的来源节须同时给出两篇笔记的出处）
- [ ] **Step 8: 冻结 14；03 暂不冻结**（§5 待 Task 9 处置完再冻，在 03 头部加临时注释 `<!-- §5 待迁 architecture/attribute-specialization -->` 不算冻结声明）
- [ ] **Step 9: 簇验证 + 🛑 簇停点**

### Task 9: architecture 簇 I——专门化（5 篇 ← 笔记 05 + 03§5）

**Files:**
- Create: `kb/topics/dita/architecture/` 下 5 篇
- Modify: `kb/maps/domains/dita/architecture.ditamap`、`research/notes/{03,05}-*.md`

| slug | 标题（暂定） | 题材 | 源 |
|---|---|---|---|
| structural-specialization | 结构化专门化 | deep-dive | 05§1 |
| domain-specialization | 域专门化 | deep-dive | 05§2 |
| attribute-specialization | 属性专门化 | deep-dive | **05§3 + 03§5 合并** |
| constraints-generalization | 约束与泛化 | deep-dive | 05§4–5 |
| specialization-practice | 专门化实战判据 | best-practice | 05§核心思想+§6 |

- [ ] **Step 1–5: 逐篇工作循环**
- [ ] **Step 6: 冻结 03 与 05**（03 的全部小节至此处置完）
- [ ] **Step 7: 簇 I 验证**（三命令绿；停点合并到 Task 10 末尾，本任务不停）

### Task 10: architecture 簇 II——架构与处理模型（14 篇 ← 笔记 09 / 10 / 11）

**Files:**
- Create: `kb/topics/dita/architecture/` 下 14 篇
- Modify: `kb/maps/domains/dita/architecture.ditamap`、`research/notes/{09,10,11}-*.md`

| slug | 标题（暂定） | 题材 | 源 |
|---|---|---|---|
| extension-facilities | 三大扩展设施 | deep-dive | 09§1 |
| doctype-shell | document-type shell | deep-dive | 09§2 |
| vocabulary-modules | 词汇模块与约束/扩展模块 | deep-dive | 09§3–4 |
| conformance | DITA 一致性 | reference | 09§5–6 |
| addressing-modes | 直接与间接寻址、片段标识符 | deep-dive | 10§1–2 |
| key-space-model | 键空间模型与 keyscope | deep-dive | 10§3–4 |
| cross-deliverable-addressing | 跨交付物寻址 | how-to | 10§5 |
| branch-filter-key-space | 分支过滤与键空间的交互 | deep-dive | 10§6 |
| effective-attribute-values | 属性有效值五级优先级 | reference | 11§1 |
| metadata-cascade | 元数据级联与 @cascade | reference | 11§2 |
| conref-attribute-rules | conref 解析的属性规则 | reference | 11§3 |
| sorting-sort-as | sort-as 与中文排序 | how-to | 11§4 |
| nav-generation | TOC、索引与链接生成 | reference | 11§5–5.9 |
| processing-checklist | 程序化处理检查清单 | cheatsheet | **11§6 + 10§7 合并** |

- [ ] **Step 1–14: 逐篇工作循环**
- [ ] **Step 15: 冻结 09、10、11**
- [ ] **Step 16: 簇验证 + 🛑 簇停点**（I+II 合并报告，19 篇）

### Task 11: core-model 簇剩余（6 篇 ← 笔记 01）

**Files:**
- Create: `kb/topics/dita/core-model/` 下 6 篇
- Modify: `kb/maps/domains/dita/core-model.ditamap`、`research/notes/01-core-model.md`

| slug | 标题（暂定） | 题材 | 源 |
|---|---|---|---|
| topic-typing | topic 类型化与内容模型 | deep-dive | 01§1+§1.5 |
| map-structure | map 的结构与内容分离 | deep-dive | 01§2 |
| titlealt-system | titlealt 标题体系 | reference | 01§3 |
| table-model-choice | 表格双体系选型：simpletable 与 CALS | best-practice | 01§3.5 |
| images-multimedia | 图像与多媒体元素 | reference | 01§3.6 |
| class-derivation | @class 与 @specializations 派生链 | deep-dive | 01§4–5 |

01§定位、§6（定位差）并入全景的维度框架论述——冻结声明列"并入 dita-landscape"；若全景未涵盖则回补全景（改全景须重跑其工作循环 2–6）。

- [ ] **Step 1–6: 逐篇工作循环**
- [ ] **Step 7: 冻结 01 + 簇验证 + 🛑 簇停点**

### Task 12: principles 簇（5 篇 ← 笔记 00、12）

**Files:**
- Create: `kb/topics/dita/principles/` 下 5 篇
- Modify: `kb/maps/domains/dita/principles.ditamap`、`research/notes/{00,12}-*.md`

| slug | 标题（暂定） | 题材 | 源 |
|---|---|---|---|
| roles-and-boundaries | DITA 的角色分工与阅读边界 | deep-dive | 00§1–4（§3 各角色该读什么→并入本篇边界论述） |
| schema-authority | schema 定义权与 @class 权限边界 | deep-dive | 00§5–8 |
| first-principles | DITA 的第一性原理推导 | deep-dive | 12§一–二 |
| costs-and-legacy | DITA 的代价与历史包袱 | deep-dive | 12§三–五 |
| portable-principles | 脱离 DITA 可迁移的原则 | deep-dive | 12§六–七 |

12 是论证篇：来源节的「判断」段要如实写"为本库/原笔记推导，可被反驳"，「事实」段只列推导引用的规范条文出处。

- [ ] **Step 1–5: 逐篇工作循环**
- [ ] **Step 6: 冻结 00、12 + 簇验证 + 🛑 簇停点**

### Task 13: content-engineering 增补（2 篇 + 1 处修改）

**Files:**
- Create: `kb/topics/content-engineering/dimension-type-genre.dita`（deep-dive，模板 A，shell `../../schema/`，**无 domain=dita**——prolog domain 值沿 content-engineering 现有篇的写法）
- Create: `kb/topics/content-engineering/domain-dimension-method.dita`（best-practice，同上）
- Modify: `kb/topics/content-engineering/writing-sourcing.dita`、`kb/maps/domains/content-engineering.ditamap`、`research/cases/kb-redesign/{content-type-framework,dimension-completeness}.md`

**Interfaces:**
- Consumes: content-type-framework.md（维度>类型>题材三层）、dimension-completeness.md（对标建框架+80/20）、kb-redesign README §4（两层防腐）

- [ ] **Step 1: dimension-type-genre**——三层框架论证迁入，走工作循环（validate/lint 命令路径换成 content-engineering）
- [ ] **Step 2: domain-dimension-method**——方法三步 + 约束方式迁入，best-practice 五节
- [ ] **Step 3: writing-sourcing 修改**——在其「做法」或对应节内并入两层防腐机制（volatility 管单条事实、benchmark-registry 管分类树，两层同构：来源核对+日期戳）。定点 Edit，先 `rg` 找锚文本，改完重跑该篇工作循环 2–3 + `just review`
- [ ] **Step 4: 冻结两份 kb-redesign 设计稿**（加"已吸收为 kb topic"头，指向新 topic）
- [ ] **Step 5: Commit + 🛑 停点**

### Task 14: 收尾——README 冻结 + 全库终验

**Files:**
- Modify: `research/README.md`、`docs/architecture.md`（若能力地图有涉及迁移状态的行）

- [ ] **Step 1: README 冻结头**——标题下加：版本现状/30 秒/权威资源已迁 kb（各指向 topic），笔记索引表每行加"已迁"标记；README 保留调研待办与阅读路径（阅读路径→缓建项，见 spec §八）
- [ ] **Step 2: 全库终验**

Run: `just review && just ia && just links && sh kb/scripts/build-agent-rules.sh && just test && just clippy`
Expected: 全绿；ia 骨架内 topic 数 = 迁移前 38 + 本计划新增（67 + 词表新增数）。

- [ ] **Step 3: 终验报告 + push**

```bash
git push
```

报告：总 topic 数、各簇清单、词表新增、覆盖度变化（ia 输出）、遗留缓建项（spec §八）。

---

## 计划外事项（发现即停）

- 笔记内容与 DITA-OT 4.4 / beta03 差异相关的事实拿不准 → 按笔记原文写并保留其限定语，不自行"更新"事实
- lint 对新题材出现规则误报（如 task 的 R14）→ 停，报告用户裁定是规则改还是内容改
- 任何一篇 R16 计数超阈（concept >8 个实现标记）→ 按既定纪律拆篇，拆出的新篇按同簇 cheatsheet/reference 处理并入清单，报告用户
