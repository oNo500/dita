# topic 解析器与 IA 视图深化

> 承 [2026-08-12 架构计划](2026-08-12-dita-tools-architecture.md) 的路线重排（见其「状态与更正」）。
> 边界与依赖方向见 [架构与边界](../../../docs/架构与边界.md)。

**Goal:** 让 IA 视图从"有几篇"走到"缺什么"——能按域回答：哪类内容缺、哪些还是 draft、哪些维度没覆盖、哪些标注非法。

**为什么现在做：** 建库之初的全局观测与设计需要它。现在九个领域里七个是空的，决定"下一批写什么"靠的正是这类判断；等库长大再补，观测缺口会被内容量掩盖。

## 需求 → 规格 对照表

上一份计划栽在"实现规格取消了自己的立项理由"（详见其更正节）。因此本计划的每条规格必须指回它服务的需求；指不回去的不写。

| # | 需求（观测者想知道的） | 规格 | 验证方式 |
|---|---|---|---|
| N1 | 这个域有几篇、都是什么类型 | Task 2 产出 `TopicMeta.topic_type`；Task 3 按域聚合 | kb 实跑，与 `find topics -name '*.dita'` 计数对账 |
| N2 | 哪些内容还不可信（draft / 未核对） | `TopicMeta.maturity` / `volatility`，Task 3 按域分布 | kb 实跑，与 `grep maturity=` 对账 |
| N3 | 这个域规划了哪些维度、还缺哪些 | Task 1 读全景的 `planned-dimension`，Task 3 求差 | **与 `dimension-coverage.py` 输出逐域对账，必须一致** |
| N4 | 域归属不该靠每篇手标 | Task 3 从 map 树推域归属（见下方「关键设计」） | 对账：推出的域归属 ⊇ 手标的 `data name="domain"` |
| N5 | 有没有标了不存在的维度值 | Task 1 读 `subjectScheme` 得合法值，Task 3 报非法值 | 造一个非法值样例，确认被报出 |
| N6 | 空领域、孤儿、断链 | 已完成（commit `7f09ed9`） | 已在 kb 实测 |

**不在本计划内**：Keyref / Conref / 页面渲染 / napi。R11 的**归属**也不在——本计划只让 IA 视图**报告**非法维度值（观测），不决定它是否取代 `check-rules.xsl`（治理），那仍是「架构与边界」待定项 2。

## 关键设计：域归属从 map 推，不从 topic 读

现状：`dimension-coverage.py` 靠每篇 topic 里手写 `<data name="domain" value="…"/>` 判断域归属。**全库 22 篇只有 1 篇标了**，所以覆盖度报告只看得见 `electron` 一个域，其余八域完全不在统计内——这个功能目前近乎空转。

本工具已有 map 树，域归属是**结构事实**：一篇 topic 挂在 `domains/web.ditamap` 下，它就属于 web 域。因此：

- 域归属**从 map 树推**，不要求作者每篇手标。
- 仍读 `data name="domain"`，但只作为**交叉校验**：手标与结构推导不一致时报 warning（通常意味着 topic 挂错了 map，或标注过期）。
- 一篇 topic 被多个域 map 引用时，全部计入（交叉标引是本库明确允许的，见词表设计）。

这是 Rust 工具相对现有脚本的真实增益：脚本只有文件系统视角，工具有结构视角。

## 前置（kb 侧，各一处，需先处理）

- [ ] `electron-landscape.dita` 未挂进 `domains/web.ditamap`（当前是全库唯一的孤儿）。挂上它，N3/N4 才有真实数据可测。
- [ ] `root.ditamap` 里 `ai` 与 `content-engineering` 被 `topichead` 包了一层同名节点，另外七个域是裸 `mapref` → 树里出现 `AI → AI`。**需决策**：删掉这两个 wrapper（结构一致），还是九个都包（语义一致）。不阻塞本计划，但会影响域归属推导的实现细节（要不要跳过同名 topichead）。

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
- [ ] Cargo.toml：依赖 `roxmltree`、`dita_diagnostics`、`anyhow`、`thiserror`
- [ ] 解析 `<subjectdef>` 树（递归，保留层级——维度值集是分组的）
- [ ] 解析 `<enumerationdef>`：`<attributedef name="dimension"/>` + `<subjectdef keyref="…"/>` → 属性到合法值集合的映射
- [ ] 单元测试：用 kb 的真词表做 fixture 的**子集**（不引用 kb 路径，工具不得依赖 kb 存在），断言 `dimension` 合法值含 `dim-concept`、不含 `dim-nonexistent`
- [ ] Commit

## Task 2：`topic_parser` — 产出 `TopicMeta`

**服务需求：** N1、N2、N3

`dita_ast::TopicMeta` 类型早已定义但**没有生产者**（这是它接不上任何业务规则的根因）。本任务补上生产者。

**Files:** `crates/dita_parser/src/topic_parser.rs`、`crates/dita_parser/tests/parse_topic.rs`、fixtures

**Produces:** `fn parse_topic(path: &Path) -> Result<(TopicMeta, DiagnosticBag)>`

**Steps:**
- [ ] 根元素名 → `TopicType`（concept/task/reference/troubleshooting/glossentry，其余 `Unknown` 并报 warning）
- [ ] 读根属性 `@id`、`@maturity`、`@volatility`、`@dimension`（空白分隔多值）、`@xml:lang`
- [ ] 读 `<title>`；读 prolog 下 `<data name="domain">`、`<data name="planned-dimension">`（多值）、`<data name="reviewed">`
- [ ] `TopicMeta` 增加字段承载上述内容（`planned_dimensions`、`reviewed`），并同步更新 Task 1 计划里的类型规格
- [ ] fixtures：一个 concept（全属性齐）、一个全景（带 planned-dimension）、一个属性缺失的、一个非法 `@dimension` 值的
- [ ] 测试断言逐字段，**含缺失属性时不 panic**（`maturity` 缺失 = `None`，由上层按"默认 draft"解释——默认值语义属规则层，不属解析层）
- [ ] Commit

## Task 3：IA 报告深化

**服务需求：** N1–N5

**Files:** `crates/dita_ia/src/{lib.rs,domain.rs,stats.rs}`、`apps/dita_cli/src/commands/ia.rs`

**Steps:**
- [ ] `domain.rs`：遍历 map 树，产出 `topic 路径 → 所属域` 的映射（域 = 直接挂载它的领域 map；跨域引用则多归属）
- [ ] 解析每个被引 topic 与每个孤儿 topic 的 `TopicMeta`（并发不必要，库还小；先直白实现）
- [ ] `stats.rs`：按域聚合——篇数、类型分布、maturity 分布、volatility 分布
- [ ] 维度覆盖：`planned`（来自本域全景的 `planned-dimension`）∩ `covered`（本域各 topic 的 `@dimension`）→ 覆盖度与盲区，**语义与 `dimension-coverage.py` 完全一致**（覆盖度 = |覆盖∩规划| / |规划|；另列"规划外的覆盖"）
- [ ] 诊断新增：非法 `@dimension` 值（N5）、缺全景的域（R9 的观测面）、手标 domain 与结构推导不一致（N4）
- [ ] 终端输出：树之后加「按域概览」段，每域一行摘要 + 盲区明细
- [ ] `--vocab` 参数（默认 `vocab/subjectScheme.ditamap`）；词表缺失时降级——跳过 N5 并提示，不中断（`kb` 不必为工具改结构，工具也不该假定 kb 布局不变）
- [ ] Commit

## Task 4（可选，做完 1–3 再定）：`--format json`

**服务需求：** 未来的页面渲染；以及让 `kb/scripts/` 能消费本工具的结果而不必解析终端文本。

- [ ] `serde` 序列化报告结构，`--format text|json`
- [ ] 快照测试（`insta`，已在依赖表里）

## 验证：与现有脚本差分对账

本工具**不取代** `dimension-coverage.py`（归属待定），但必须与它结果一致——不一致就是至少一边错了，正是这类对账要抓的：

```bash
cd kb
python3 scripts/dimension-coverage.py                 # 现有实现
../dita-tools/target/debug/dita-tools ia --domains    # 新实现（Task 3 后）
# 逐域比对：覆盖度分子分母、盲区集合
```

**预期会出现一处合理的不一致**：新实现从 map 结构推域归属，所以能看见九个域；旧实现只看手标，只看得见 `electron`。这不是 bug，是 N4 要解决的问题本身——对账时按"旧实现的域集合 ⊆ 新实现的域集合，且交集内数字一致"来判定。

## 完成的判据

- [ ] `cargo test --workspace` 全过，`cargo clippy` 零告警
- [ ] `dita-tools ia` 在 kb 上跑通，输出九个域的概览与盲区
- [ ] 与 `dimension-coverage.py` 按上述规则对账通过
- [ ] 本文件每个 checkbox 都基于**实跑**勾选，不基于"代码写完了"
