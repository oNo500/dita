# topic 解析器与 IA 视图深化

> 承 [2026-08-12 架构计划](2026-08-12-dita-tools-architecture.md) 的路线重排（见其「状态与更正」）。
> 边界与依赖方向见 [架构与边界](../../../docs/architecture.md)。

**Goal:** 让 IA 视图从"有几篇"走到"缺什么"——能按域回答：哪类内容缺、哪些还是 draft、哪些维度没覆盖、哪些标注非法。

**为什么现在做：** 建库之初的全局观测与设计需要它。现在九个领域里七个是空的，决定"下一批写什么"靠的正是这类判断；等库长大再补，观测缺口会被内容量掩盖。

## 需求 → 规格 对照表

上一份计划栽在"实现规格取消了自己的立项理由"（详见其更正节）。因此本计划的每条规格必须指回它服务的需求；指不回去的不写。

| # | 需求（观测者想知道的） | 规格 | 验证方式 |
|---|---|---|---|
| N1 | 这个域有几篇、都是什么类型 | Task 2 产出 `TopicMeta.topic_type`；Task 3 按域聚合 | kb 实跑，与 `find topics -name '*.dita'` 计数对账 |
| N2 | 哪些内容还不可信（draft / 未核对） | `TopicMeta.maturity` / `volatility`，Task 3 按域分布 | kb 实跑，与 `grep maturity=` 对账 |
| N3 | 这个域规划了哪些维度、还缺哪些 | Task 1 读全景的 `planned-dimension`，Task 3 求差 | **与 `dimension-coverage.py` 输出逐域对账，必须一致** |
| N4 | 域归属不该靠每篇手标 | **已修正，见下方「关键设计」**：map 推的是*分支*，手标的是*技术域*，两者并存各司其职 | 覆盖度按技术域与脚本对账；分支信息为新增 |
| N5 | 有没有标了不存在的维度值 | Task 1 读 `subjectScheme` 得合法值，Task 3 报非法值 | 造一个非法值样例，确认被报出 |
| N6 | 空领域、孤儿、断链 | 已完成（commit `7f09ed9`） | 已在 kb 实测 |

**不在本计划内**：Keyref / Conref / 页面渲染 / napi。R11 的**归属**也不在——本计划只让 IA 视图**报告**非法维度值（观测），不决定它是否取代 `check-rules.xsl`（治理），那仍是「架构与边界」待定项 2。

## 关键设计：域归属从 map 推，不从 topic 读

现状：`dimension-coverage.py` 靠每篇 topic 里手写 `<data name="domain" value="…"/>` 判断域归属。**全库 22 篇只有 1 篇标了**，所以覆盖度报告只看得见 `electron` 一个域，其余八域完全不在统计内——这个功能目前近乎空转。

**动手时发现原方案有个前提是错的**（记在此处，不抹掉）：本库的"域"有两个粒度，且不重合。

| | 来自 | 粒度 | 例 |
|---|---|---|---|
| **分支** | map 结构（root 的直接子节点） | 粗，九个 | `web` |
| **技术域** | topic 自己声明的 `data name="domain"` | 细 | `electron` |

`planned-dimension` 是**按技术域**声明的。一个分支下将来会有 electron / react / nextjs 各自的全景，**按分支合并算覆盖度会把三份规划混成一份**，是错的。所以不是"用 map 推导取代手标"，而是两者并存、各司其职：

- **分支归属（map 推导）** → 篇数、类型/成熟度/时效分布、空分支、无全景标注。作者不必手标。
- **技术域归属（手标 `domain`）** → 维度覆盖度与盲区。这个粒度 map 结构里没有，只能声明。
- **交叉校验**：同一技术域的 topic 若分散在多个分支下，报 warning（通常是挂错 map 或标注过期）。

Rust 工具相对脚本的真实增益仍然成立，只是位置变了：脚本只有文件系统视角，看不到一个域坐落在哪个分支、哪个分支是空的、哪个分支没有全景。

## 前置（kb 侧，各一处，需先处理）

- [x] `electron-landscape.dita` 挂进 `domains/web.ditamap` —— **已挂，全库零孤儿**，N3/N4 有真实数据可测了。
- [x] `root.ditamap` 的 wrapper 不一致 —— **已定：九个域 + 术语库全部包 `topichead`**。
  先实测搞清了 wrapper 到底解决什么：规范规定 mapref 是"被引 map 的层级并入容器 map"，
  **被引 map 的 `<title>` 不产生导航节点**。`dita -f html5` 实测印证——没包 wrapper 的
  `web` 域，其 topic 被直接摊到 TOC 顶层，"Web 技术栈"在产物里根本不存在；术语库同理。
  所以 wrapper 补的是真实发布缺陷，**不是 DITA 2.0 语法要求**。
  一度考虑的替代方案（领域 map 内部用全景 topic 当父节点）**解决不了这个问题**：全景是
  技术域级的（`domain="electron"`），而丢失的是分支级的名字。
  代价（分支名两处副本会漂移）由工具兜住，见下。
  迁移路径写进了 map 注释：将来分支有了真实落地 topic，就把 topichead 换成指向它的 topicref。

---

## Task 1：`dita_vocab` — 读受控词表

**服务需求：** N3、N5

**边界纪律：** 合法值一律从 `kb/vocab/subjectScheme.ditamap` 读，**不得在 Rust 里内联任何值清单**（架构与边界 §二红线一）。`check-rules.xsl` 里那份手抄副本是已知欠账，本任务不动它，但新代码不许再抄一份。

**Files:** `crates/dita_vocab/{Cargo.toml,src/lib.rs}`

**Produces:**
- `Vocabulary { subjects: Vec<Subject>, enums: HashMap<String, HashSet<String>> }`
- `Subject { keys: String, nav_title: Option<String>, children: Vec<Subject> }`
- `fn parse_vocab(path: &Path) -> Result<(Vocabulary, DiagnosticBag)>`
- `fn legal_values(&self, attribute: &str) -> Option<&HashSet<String>>` —— 由 `<enumerationdef>` 的 `<attributedef name>` + `<subjectdef keys>` 解析而来

**Steps:**
- [x] Cargo.toml：依赖 `roxmltree`、`dita_diagnostics`、`anyhow`（`thiserror` 没用上——本 crate 的失败只有一类「文件读不了」，`anyhow` 足够）
- [x] 解析 `<subjectdef>` 树（递归，保留层级——维度值集是分组的）
- [x] 解析 `<enumerationdef>` → 属性到合法值集合的映射。另读 `<defaultSubject>`：它的**缺席**是有意义的（volatility 故意不设默认，漏标应报错而非静默取默认）
- [x] 单元测试：fixture 是真词表的结构**子集**，不引用 kb 路径。6 个测试，含「分组键合法但不是叶子」「被绑 subject 自身不是值」「悬空 keyref 报 error 但不拖垮其余属性」
- [x] **在真词表上实跑**（新增 `examples/dump_vocab.rs`——fixture 过了不等于真文件能读）：@dimension 51 值（5 分组 + 46 叶子）、@maturity 3 值默认 draft、@volatility 2 值无默认、@tool 3 值，零诊断
- [x] 对账 `check-rules.xsl` 的手抄副本：三个小值集**当前无漂移**（同批所建、词表未再改），SSOT 欠账仍是潜伏状态；45 个维度值从未抄进 XSL，正是 R11 缺失的原因
- [x] Commit

## Task 2：`topic_parser` — 产出 `TopicMeta`

**服务需求：** N1、N2、N3

`dita_ast::TopicMeta` 类型早已定义但**没有生产者**（这是它接不上任何业务规则的根因）。本任务补上生产者。

**Files:** `crates/dita_parser/src/topic_parser.rs`、`crates/dita_parser/tests/parse_topic.rs`、fixtures

**Produces:** `fn parse_topic(path: &Path) -> Result<(TopicMeta, DiagnosticBag)>`

**Steps:**
- [x] 根元素名 → `TopicType`。**偏离计划一处**：另立 `TopicType::Topic` 承载通用 `<topic>`——它是合法 DITA，「本库要不要允许」属规则层判断，解析层只报事实；`Unknown` 仅留给真正不认识的根元素，只有它报 warning
- [x] 读根属性 `@id`、`@maturity`、`@volatility`、`@dimension`（空白分隔多值）、`@xml:lang`
- [x] 读 `<title>`（**含子元素内的文字**——标题里嵌 `<term>`/`<xmlelement>` 很常见，只取直接文本节点会静默截断）；glossentry 的标题取 `<glossterm>`；读 prolog 下三种 `<data>`
- [x] `TopicMeta` 扩字段：`planned_dimensions`、`reviewed`、`lang`，`id` 改 `Option`（缺失不是空串），`dimension` 改 `dimensions`（多值）。2026-08-12 计划里的旧类型规格已加注以本计划为准
- [x] fixtures 六个：全属性 concept、全景、属性全缺、glossentry、非法维度值、根元素不认识的
- [x] 7 个测试逐字段断言，含"缺失属性是 `None` 而非默认值"（"忘了标"和"选了默认"必须可区分，R2 靠的就是这个区别）
- [x] **在真库实跑**（新增 `examples/dump_topics.rs`）：22 篇全解析、**零告警**；与 grep 对账一致——concept 10 / glossentry 12、curated 21 / draft 1、volatile 7 / stable 15。顺带发现：全库唯一的 draft 正是 web 域的全景 `electron-landscape`，即该域的维度规划本身还没过审
- [x] Commit

## Task 3：IA 报告深化

**服务需求：** N1–N5

**Files:** `crates/dita_ia/src/{lib.rs,domain.rs,stats.rs}`、`apps/dita_cli/src/commands/ia.rs`

**Steps:**
- [x] `domain.rs`：遍历 map 树，产出 `topic 路径 → 所属**分支**`（跨分支引用则多归属）
- [x] 解析 `topics/` 下**全部** topic 的 `TopicMeta`（含孤儿——孤儿的元数据往往更值得看）
- [x] `stats.rs`：按分支聚合——篇数、类型/maturity/volatility 分布，另标注该分支有无全景
- [x] 维度覆盖：语义与 `dimension-coverage.py` 完全一致，**差分对账通过**（域名、分子分母、盲区集合逐项相同）；另附该域位于哪个分支——这是脚本给不出的
- [x] 诊断：`topichead` navtitle 与被引 map 标题漂移（提前做——它是上面那个决定的安全网，2 个测试）
- [x] 诊断新增：非法 `@dimension` / `@maturity` / `@volatility` 值（N5，error）、同一技术域的 topic 分散在多个分支（warning）。**缺全景改为标注而非 warning**——"术语库"这类纯组织分支本就不该有全景，报 warning 是制造假阳性
- [x] 终端输出：树之后加「按分支」与「维度覆盖」两段。列宽按**显示宽度**对齐（CJK 占两列，`{:<n}` 数的是字符，混排必错位）；并明说有几篇不属任何分支，免得分支合计与总数对不上却无人察觉
- [x] `--vocab` 参数；词表缺失即跳过值检查**并明说跳过了**，绝不猜一份合法值清单（有测试守着）
- [x] 端到端测试：迷你 kb fixture（含空分支、全景、非法值），6 个测试
- [x] Commit

## Task 4（可选，做完 1–3 再定）：`--format json`

**服务需求：** 未来的页面渲染；以及让 `kb/scripts/` 能消费本工具的结果而不必解析终端文本。

- [ ] `serde` 序列化报告结构，`--format text|json`
- [ ] 快照测试（`insta`，已在依赖表里）

## 验证：与现有脚本差分对账

~~本工具**不取代** `dimension-coverage.py`（归属待定）~~（后记：终态落定后，该脚本已于 2026-08-15 走完吸收五关退役，本工具即其归宿），但吸收前必须与它结果一致——不一致就是至少一边错了，正是这类对账要抓的：

```bash
cd kb
python3 scripts/dimension-coverage.py                 # 现有实现
../dita-tools/target/debug/dita-tools ia --domains    # 新实现（Task 3 后）
# 逐域比对：覆盖度分子分母、盲区集合
```

**预期会出现一处合理的不一致**：新实现从 map 结构推域归属，所以能看见九个域；旧实现只看手标，只看得见 `electron`。这不是 bug，是 N4 要解决的问题本身——对账时按"旧实现的域集合 ⊆ 新实现的域集合，且交集内数字一致"来判定。

## 完成的判据

- [x] `cargo test --workspace` 全过（30 个测试），`cargo clippy` 零告警
- [x] `dita-tools ia` 在 kb 上跑通，输出九个分支的概览与盲区
- [x] 与 `dimension-coverage.py` 对账通过
- [ ] 本文件每个 checkbox 都基于**实跑**勾选，不基于"代码写完了"
