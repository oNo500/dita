# 裁定台账与遗留清单（notes → kb 迁移，2026-08-16/17）

> 定位：**档案，不是 topic。** 本文件记录迁移过程中做过的每一次裁定（决定、理由、决定错了的代价）与收尾时仍开放的遗留项。它给人读，不进 `kb/`，不过 lint、不受 R1–R17 约束。
>
> 为什么单独留档：这批裁定原本只存在于 SDD 工作区 `.superpowers/sdd/2026-08-16-notes-to-kb-migration/progress.md`。`.superpowers/` 在 `.gitignore` 内，工作区一删即失。同一目录下的 [`upstream-provenance.md`](upstream-provenance.md) 出于同一理由抽出。这条纪律的正本是 [`kb/topics/content-engineering/rot-detection.dita`](../../../kb/topics/content-engineering/rot-detection.dita) 的「留证与产物同寿」：**临时工作区里的东西，从写下的那一刻起就该假定它会丢。**
>
> ⚠️ **与 kb 正本冲突时以 kb 为准。** 台账记的是"当时为什么这么定"，不是现行规则。规则的正本在 `kb/topics/content-engineering/` 的九篇规则正本与 `kb/schema/rules.sch`。

迁移全貌见 [`README.md`](README.md)；标题与上游节点的对照见 [`upstream-provenance.md`](upstream-provenance.md)。

---

## 一、怎么读这份文件

- **第二节裁定台账**按执行顺序排，每条给三样：决定、理由、决定错了要付的代价。列"代价"是刻意的——它是当时判断"这个决定敢不敢在没有用户在场时做"的依据，也是日后推翻它时的成本估算。
- **第三节**单列用户直接下的裁定与批准，它们的效力高于执行者裁定。
- **第四节遗留清单**按三级分诊：合入后近期做 / 可长期留 / 待用户裁定。分诊标准写在该节开头。
- **第五节**记已关闭项，含关闭它的那一轮。

计数：执行者裁定 45 条、用户裁定与批准 9 条、parked/deferred 若干（并入第四节）。

---

## 二、裁定台账

### 预检与执行方式

1. **直接在 master 执行，不建分支。** 理由：本仓一直是 trunk-based（用户全程直接 commit/push master，计划 Task 14 即 push），建分支反而偏离既定流程。代价：master 上的提交序列需 revert。
2. **新建 `.gitignore` 忽略 `.superpowers/`。** 理由：工作循环的 `git add -A` 否则会把 ledger/brief 卷进历史。代价：无（纯防护）——但正是这条使本文件成为必要。

### Task 1–3（词表、shell、域 map）

3. **Task 2 BLOCKED 后改绑：how-to 改绑 concept，quickstart 独留 task，来源节置 steps 前为单题材例外。** 理由：`taskbody` 的 OASIS 内容模型拒绝 steps 后置 section，而 R14 要求来源节收尾。代价：词表一行改回 + 约 11 篇 shell 调整；Task 5 用户门可低成本推翻。

### Task 4（全景与域归位）

4. **`reviewed="2026-08"` 月粒度保留。** 理由：笔记来源即月粒度，补日号是捏造；诚实优先于格式一致。代价：与库内日粒度惯例并存，终审可再统一。
5. **全景自标 `@dimension="dim-comparison dim-evolution"`。** 理由：spec 把"定位对比/版本现状"永久判给全景，不标造成假盲区。代价：与两个既有全景形制不一致，用户可在试点门推翻。
6. **README「四个招牌特性」不单独成篇。** 理由：四特性各归其 topic（keyref-variable-text / titlealt-system / include-non-dita / chunking），"最能验证工具链支持"的框架句并入 adoption-criteria。代价：若用户认为值得独立篇，补一篇 curated-resources 即可。
7. **I1 采用方案一：`dim-process` 回归词表 gloss（流程/治理义），处理顺序类内容挂 `dim-mechanism`/`dim-internals`。** 理由：保 `just ia` 求差可信。代价：若日后确需"处理模型"专用维度，另行扩词表。
8. **审查者观察 1（writing-style 的 tech-landscape 固定结构与 spec §六 冲突）并入 Task 13 范围；观察 4（`dim-terminology` 更贴笔记 13）随 Task 7 携带。**

### Task 5（试点篇与术语返工）

9. **dita-landscape 两处裸术语授权回填。** 理由：该篇是本迁移 Task 4 产物，全局约束"不改已有 topic"指迁移前存量，不适用。代价：无。
10. **conref 篇 prolog `source` 改 `https://dita-lang.org/specifications`。** 理由：`source` 是元数据非事实断言，原 include 页对下游误导。代价：无。
11. **母版通则入 authoring-standards：凡不在笔记已核对清单内的机制陈述，一律进判断段或加篇级 caveat。** 理由：诚实档位需可执行。代价：来源节稍冗长。**⚠ 后果**：authoring-standards 随工作区删除，这批通则一度无版本化正本——2026-08-17 终审修复轮把其中三条落进 writing-style / writing-sourcing（见第五节 ④）。
12. **Important-2（writing-style 题材表 how-to→task 过期 + 中段节名）并入 Task 13 范围。**
13. **术语规则定型：DITA 元素/属性/机制名一律英文原名作首选词，中文随行解释；glossentry 首选词同改；自造分类框架降级为平实表述。** 理由：对齐 ai 分支 hook/skill/MCP 先例与英文上游。代价：三篇 + 词表 + 全景返工一轮。**根因**：用户门反馈"看不懂，造的词与概念"。
14. **轮 2 触碰存量 `dita-authoring-guide.dita`（3 处裸 keyref 包 `<xmlatt>`）接受。** 理由：glossterm 改英文后的零裸字面必然连带，markup-only 不改文意。代价：无。

### Task 6 / 6.5（工具链簇与 domain 归一）

15. **quickstart 改绑 concept，task 类型退役、`task-kb.rng` 删除。** 理由：steps 元素、R13 步骤节、来源收尾三者不可兼得，sections-only 的 task 是假类型。代价：词表一行 + 一篇改壳；未来出现真步骤型题材时从 git 复活 shell。
16. **04 §2 `--project` 留待 practice 簇 engineering-ci 承接。** 代价：见第五节 ⑤——13b 又把它拆成独立篇，两篇笔记的冻结声明因此过期了一整轮。
17. **4 处术语回填授权**（conref-pull-push / dita-landscape / dita-resources 均本迁移产物，不受存量纪律限制）。
18. **authoring-guide 的叶子键示例 `doc-types` → `writing-principles`，随后删两孤儿 key。** 理由：真实叶子，示例更贴切。代价：无。

### 计划修订与并行

19. **标题规则收紧（用户第二次纠偏）→ 随即由用户定稿为一句话。** 见第三节第 5 条。执行者侧的处置：存量 14 篇改名列入专项修复轮；lint 代理（标题含「：」报 warning）排 dita-tools 队列。代价：存量改名一轮 + 可能误伤合理冒号用法（暂无）。
20. **自造口子加三道关（穷尽查证 → 先怀疑切分 → 只组合不发明）+ 头注释标注依据。** 理由：自造聚合篇是既往所有自造词事故的来处，不设纪律等于请回病灶。
21. **specialization / generalization / constraint / extension facilities / vocabulary module 等自造译名全部改英文原名；判据改为"规范里有对应条目即用英文原名"。** 理由：不再依赖"看起来像不像术语"——「专门化」伪装成通用语混过了全部审查。代价：存量一轮替换。

### Task 7（practice 簇）

22. **疑虑①②接受现状：RAG 三篇事实段「无」但 prolog `source` 保留；不写 `reviewed`。** 理由：`source` 是溯源元数据非事实认领（R8 强制）；笔记 15 无核对日期，诚实优先。代价：与其余篇形式不齐，终审可再议。
23. **疑虑③全库标题过账**：ai 与 content-engineering 分支的存量标题并入 13b（naming-rules 建立时顺带）。
24. **疑虑④ titlealt/chunk 两篇建成后回补 adoption-criteria 的 xref**，登记给 Task 8（chunk）与 Task 11（titlealt）。**⚠ 只执行了 chunk 一半**，titlealt 一半到 2026-08-17 终审才补上（第五节 ②）。
25. **Critical-1（10 篇缺自造依据头注释）与 Important-3（6 处 xref 链接文字失效）并入 13b 同一轮过账。** 理由：13b 本就要为译名改这批文件，分两轮做等于在 load 已 15 的 4 核机上多跑一整轮验证。代价：这两项在 13b 前一直存在（href 正常、links 全绿，不影响可用）。
26. **Important-4（rag-chunking-metadata 的 reltable/`@collection-type`/critdates 反向退让为中文）并入 13b 译名轮。**
27. **切分复议两项交 13b 前置评估**：① customization-cost-ladder「成本阶梯」系自造概念名，须查上游后改名或补依据；② engineering-ci 埋掉上游确有节点 Using project files，是否升为独立篇由 13b 评估。

### Task 8–12（并行三簇 + 第二波）

28. **疑虑3「constraints 与 generalization」一篇承两类目，保持不拆。** 理由：先例 conref 与 conref push；两类目互为依赖（收紧 vs 还原），拆开会得到两篇薄篇。代价：标题与上游类目非一对一。
29. **疑虑4「specialization 的启用判据」中的「判据」不算自造。** 理由：本库既有词（writing-atomicity 内容准入判据、adoption-criteria DITA 采用判据），属"只组合不发明"；但因无上游类目须补三道关头注释。
30. **疑虑1 → Task 8 冻结笔记 03 时必须补「§5 已由 attribute-specialization 承接」的对应关系。**
31. **Task 12 五个标题全部塞满，根因是已废止的"标题必含英文机制名"条款。** 处置：5 篇标题连同自造依据注释并入 13b 标题过账；建议方向为回归笔记原名的简短形态。代价：13b 范围再增 5 篇标题。
32. **疑虑1（class-derivation 与 architecture 簇重叠）**：class-derivation 保留为「规则与语法」正本（上游确有该页），但 ① 归位到 architecture/ 簇，② structural-specialization 里重复的 `@class` 四条构造规则改为 xref 指向它。执行交 Task 10b。代价：跨簇移文件 + map 两处改动。
33. **疑虑3 CALS 保留**（领域事实标准叫法，DITA 规范自身亦用 "CALS table"），glossdef 注明规范原称 OASIS Exchange Table Model。代价：无。
34. **Task 10a 疑虑1 conformance 用 reference 素体不标 outputclass。** 理由：R12 只对 concept/task 强制；标题规则优先于题材后缀惯例。代价：该篇无 R13 结构保证，篇幅小可控。
35. **Task 10a 疑虑2「DITA addressing」可能被误读为 key 三篇之父——接受现状**，map 内已加分组注释；若日后 preview 显示混淆再议。
36. **Task 10a 疑虑3 笔记 10 的 key space 算法等属未核对、正文逐处标注**——正确做法，补核对单开任务。
37. **Task 8 疑虑1 的 26 处跨簇 keyref 回填交 13b 统一执行。**
38. **Task 8 疑虑3 branch-filtering 与 key-space-model 互链回补、疑虑4 metadata-two-kinds 无 `reviewed`（比照 rag 三篇诚实处置）——均接受**，互链回补并入 13b。

### Task 13 / 13b / 13c（规则归并与过账）

39. **`kb` 前缀沿用。** 理由：规则禁的是"堆"定位词，单个限定词不构成堆叠；讲本库方法论去掉前缀会读成普适断言。代价：若 13b 的 naming-rules 裁定统一去前缀，六篇同批改，不留新旧两套。
40. **13b 复议：`kb` 前缀保留。** 理由：八篇中七篇已带，统一成本最低。代价：若用户判断相反须反向改齐 11 篇 + 约 10 处链接文字（已列入待用户裁定）。
41. **13c：29 处"链接文字为标题简写"不动。** 理由：那些标题从未改过，简写是自然行文而非失效；强行等同完整标题反而伤可读性。代价：链接文字风格不统一，可接受。
42. **class-derivation 归位 architecture 并去重已执行；attribute-specialization 与 class-derivation 在 `@specializations` 形式语法上的残余重叠接受现状**（class-derivation 已声明分工），列为 13b 复核项。

### Task 14 终验（§八 四项修复的裁定）

43. **`just ia` 的 ⚠ 门控是口径漏洞，不是执行者疏忽。** 规划外的覆盖改为无条件进「需要处理」段，只有"空叶子"一行保留 `--details` 门控，并加回归测试钉住"两种模式的差集恰好只有空叶子那一行"。
44. **四篇「规划外的覆盖」全部判为"标错了"，无一需要扩全景**；两篇的结果是单维度 topic——库内不是异常，且强行凑一个维度正是造成漂移的动作本身。
45. **交付物的 10 条悬空链接在构建脚本改写，不动 DITA 源。** 理由：属媒介转换；把源改成 `scope="external"` 会为迁就交付物而废掉库内链接校验，方向反了。并加断言：改写后若还剩 `](../` 则构建失败。
46. **DOTX031E 判为可忽略但必须显式白名单**（三条放行条件逐条核实：不是源缺陷、不丢信息、不影响交付物），白名单命中数打印不静默，且写明作废条件。

---

## 三、用户裁定与批准

效力高于上节的执行者裁定。

1. **插入 Task 6.5（domain 归一）**（2026-08-16）：3 篇 domain 改 `writing-principles` + 全景摘 `dim-criteria`；机制上锁（R17 domain ∈ subject keys）排迁移收尾后队列。
2. **push 推迟至全部完成**（Task 14 一并）；Task 9 与 Task 12 用隔离 worktree 并行，控制器负责合并冲突。
3. **新增 Task 13b（规则归并）**：建 terminology-rules 与 naming-rules 两篇正本、writing-style 瘦身改标题、authoring-guide 升路由总纲、rules.sch 标注归属、存量 8 篇 35 处译名清理。总篇数 67→69。
4. **术语纠偏三次**：① 英文机制名不得被中文译名顶替（用户门"看不懂，造的词与概念"）；② 标题规则收紧；③ 译名禁令补充（specialization 一族）。教训写进 terminology-rules：**读起来像普通中文词的机制名最危险**。
5. **标题规则定稿为一句话**：「标题就是这个节点在领域知识树上的标准叫法，用领域内的事实标准决定」，取代此前五条堆叠。代价：无（简化）。
6. **建上游节点索引**：来源 = 本地 docsrc + 克隆 `oasis-tcs/dita`；校验 = prolog 声明 `upstream-node` + 索引核实（非字符串匹配）。治理要求原话：**"没人维护就是垃圾场"**——索引须为派生物、一条命令再生成、版本锁定、接 benchmark-registry 对标登记、上游改名转为 lint 工单。
7. **索引细则**：① 首次发现 = 人工调研 + 索引留存结果；② 中英对照表从声明生成（零维护，兼作覆盖表）；③ 渲染成读者可见链接押后；④ **一定要推广到全库**，理由是方便治理。
8. **索引边界（用户指定写入设计与腐烂检测篇）**：索引查不出"节点存在但选错了"——声明与标题出自同一判断。存量按声明数分诊：1 个抽查、0 个（coined）核三道关、2+ 个核是否切分错。**T4 由"机械回填"重定级为"独立复核 + 回填"，不得由原作者复核。**
9. **用户门通过（Task 5 二次）**："比之前清晰了不少"，形状认可，术语规则定型。晋级与词表口径未获明示，维持 draft。

---

## 四、遗留清单（三级分诊）

分诊标准：

- **A 合入后近期**——拖延会涨利息：素材会失效、抄本会被当成正本、或它挡着别的事。
- **B 可长期留**——已有裁定或现状可用，不做不产生新债；多数是晋级门的前置。
- **C 待用户裁定**——执行者权限之外，方向需要用户先定。

### A. 合入后近期

| # | 项 | 为什么急 |
|---|---|---|
| A1 | 上游节点索引 **T1 环境**：`setup-env.sh` 增加 `oasis-tcs/dita` 克隆与版本记录 | T2/T4 的前置 |
| A2 | **T2 生成器**：`dita-tools upstream-index` 子命令 + 单元测试，产出首版 `kb/vocab/upstream-nodes.tsv` | 同上 |
| A3 | **T4 回填** 66 篇 `upstream-node` 声明（素材已备：`upstream-provenance.md`）。**独立复核 + 回填，不得由原作者复核** | `upstream-provenance.md` 是冻结抄本，长期存在就会被当成权威表来维护——那正是设计稿拒绝给索引加中文列的同一理由。**回填完成后即删除或标记已消费** |
| A4 | **T5 校验**：R18 落 `dita_lint` + `rules.sch` 记档 + `docs/architecture.md` 能力表登记 + benchmark-registry 加 `index-source` / `index-generated` 两字段 | R18 目前在 rot-detection 与 naming-rules 里按"已定案、实现在建"表述，方案变更需同步这两篇 |
| A5 | **T6 吸收五关**；**推广分期** dita → ai → 其余，无结构上游的分支豁免须显式登记 | 用户裁定"一定要推广" |
| A6 | **15 篇自造声明的 OASIS 侧全树查证**（DITA-OT 侧已全量实查，OASIS 侧只以各篇已核对页面为基础） | 进 T4 存量分诊；按 rot-detection 边界二不得由原执行者复核 |
| A7 | **`engineering-ci` 事实段现为空**（project-files 拆出后），补外部支撑或维持 draft | 空事实段的篇不该被误当成有据 |
| A8 | **dita-tools 队列**：重复 topicref 检测、报告 JSON 化、lint 标题含「：」代理项 | 三项都是已发现缺陷的机器面，缺一项就靠人盯 |
| A9 | **`project-files` 的 `dim-process` 疑似误标**（从 engineering-ci 继承而来；词表释义是"写作与维护流程"，本篇讲构建变体矩阵） | 它在规划内，`just ia` 不会报——正是 rot-detection 写的"机器查不出"那一类 |

### B. 可长期留

| # | 项 | 现状 |
|---|---|---|
| B1 | `reviewed` 月粒度（`2026-08`）与库内日粒度并存 | 已裁定保留（诚实优先），终审可再统一 |
| B2 | RAG 三篇形式张力：事实段「无」但 prolog `source` 挂规范总入口；三篇不带 `reviewed` | 已裁定接受（R8 强制 + 诚实优先） |
| B3 | 全景自标 `@dimension`（`dita-landscape` 标，另两个全景不标） | 已裁定接受，用户可推翻 |
| B4 | 29 处"链接文字为标题简写"不统一 | 已裁定不动 |
| B5 | 切分复核两处：`nav-generation`（合三个上游类目，真正解法是重切）、`generate-and-convert`（生成一侧无上游节点，建议单独成篇） | 后者的原始边界建议随 `retitle-report.md` 丢失，只知结论 |
| B6 | `topic-typing` 承两个上游节点偏大 | strict/general task 段是日后拆分切口 |
| B7 | `first-principles` 与 `portable-principles` 约三成论点同源，可压短；`roles-and-boundaries` 的阅读边界表第三列可升 xref | 不影响可用 |
| B8 | conref 篇补核对（规范「内容引用处理」与「处理模型」两章）；笔记 10 的 key space 算法等未核对项（正文已逐处标注） | 均为晋 curated 的前置，不晋级则无损 |
| B9 | 动过冻结笔记的声明行：Task 10b 动过笔记 01 与 10（回滚点 `606115c`）；终审修复轮又动过笔记 04 与 08 | 改的都是 bookkeeping 而非调研正文，"不再更新"按意图而非字面执行 |
| B10 | 缓建项（设计 §八）：角色阅读路径 audience map、dita 分支 reltable、CCMS 预览对标 | 已收进冻结后的 README 待办 |
| B11 | 未建的词表候选：Diátaxis（5 篇，最强候选，有权威 URL）、tech-landscape、sort-as、indexterm、collection-type、reltable、metadata cascading、key space / key scope、fallback、expansion module、titlealt、simpletable、CALS table、topicref、passthrough、processor、`-dita-use-conref-target`、benchmark-registry | 宁缺毋滥后留下 |
| B12 | 各任务 minor（deferred）：Task 1 dita 节点溯源注释的字面张力；Task 2 `93a6972` commit message 措辞；Task 5 keyref 篇样例片段 id `java-req` 未展示；Task 7 `engineering-ci`「此刻」措辞、`translation-antipatterns` 两处样例 key 未定义、笔记 13 §5 两个核对项被概括、`localization-attributes` 缺 `-dita-use-conref-target` 语义；Task 12 未跑 preview 目视确认 `dl` 渲染 | Task 7 的两处"样例 key 未定义"现已有正本可依：writing-style 做法五「样例自洽」 |

### C. 待用户裁定

| # | 项 | 需要定什么 |
|---|---|---|
| C1 | **glossentry 成熟度口径** | 新建 13 条词条标 `draft`，与库内既有 20 条 `curated` 不一致。依据是全局约束"执行者永远不写 curated"；若术语条目另有口径（定义写完即 curated），需给规则 |
| C2 | **全库晋级**（原计划 Task 15） | 122 篇里 102 篇 draft、0 篇本次产出被晋 curated。晋级需逐篇过 lint 零 warning 门 + 来源核对 |
| C3 | **`kb` 前缀** | 两次裁定保留。若判断相反，需反向改齐 11 篇标题 + 约 10 处链接文字 |
| C4 | **Task 13 两篇的语气档位** | 源设计稿标题带「待审」、正文末各有 4 条待审问题，本次按已定稿处理；若这些问题实际尚未裁定，两篇语气需下调 |
| C5 | **`kb/out/` 是否纳入版本控制** | 目前在 `.gitignore` 内，交付物重建无 commit，历史上无法追溯某次交付物长什么样。2026-08-15 那次"连续多次构建成功、发的都是陈旧内容"的事故与此同源 |

---

## 五、已关闭

### Task 14 §八（终验四项）

- **D25 交付物 10 条悬空相对链接** —— 构建脚本改写 + 断言，`6577dd5`..`5cbb574` 段。
- **D26 `build-agent-rules.sh` 吞掉 DOTX031E** —— 加错误闸门 + 显式白名单，闸门有效性已变异验证。
- **D27 `just ia` 的 ⚠ 被 `--details` 门控** —— 规划外覆盖回到默认输出，加回归测试。
- **A8 规划外覆盖待裁** —— 四篇逐条判定后关闭。

### 终审修复轮（2026-08-17，本文件同批）

- **① `review.sh` 丢弃 Saxon 退出码** —— 批量化后 R1–R10 是一次 Saxon 调用，原代码只判 stdout 非空，Saxon 失败（stdout 常为空）时整层被静默跳过而脚本仍报 ✅ 并 exit 0。改为捕获退出码走 skipped 通道（exit 2），并加 Saxon ≥ 9.8 版本门与 `sort -V` 选版。变异验证：故意破坏入口点与版本号，两次均报出并 exit 2。
- **② adoption-criteria 的「前两者」错话 + titlealt 漏链** —— 台账第 24 条那一半裁定补执行。
- **③ dita-authoring-guide 的「五个学科」** —— 与自己的九行路由表不符，且第 26 行会构建进交付物。
- **④ 母版通则三条规则落 kb 正本** —— 台账第 11 条的后果收口：deep-dive 节标题体例与样例自洽落 writing-style，来源节穷尽归属落 writing-sourcing；`rot-detection.dita` 头注释里按名引用「母版通则」的悬空引用改指 kb 正本。**这是 rot-detection「留证与产物同寿」的第一次自我适用。**
- **⑤ 笔记 04 与 08 的冻结声明过期** —— 台账第 16 条与 13b 切分复议的落差：`project-files` 已拆为独立篇，两篇笔记仍写"不迁"。全库唯一"声明为不迁而实际已迁"的小节。
- **⑥ R16 缺人读正本** —— `rules.sch` 与路由表都把归属标给 `writing-atomicity`，完整表述却留在路由总纲。表述落归属篇，总纲降为一句加 xref。
- **⑦ 台账与遗留清单落进版本控制** —— 即本文件。

---

## 六、这份台账自己的腐烂风险

按 rot-detection 的四问自查，登记已知盲区：

- **锚点在库外吗？** 部分不在。第二、三节记的是历史事件，不会腐烂；**第四节遗留清单会腐烂**——项做完了、方向变了，这里不会自己更新。
- **谁在维护？** 无人。本文件按档案定位冻结于 2026-08-17，**不是待办跟踪器**。遗留项真正开工时应转成 issue 或计划文档，回来在此标一行"已转 X"，而不是在此长期维护状态。
- **失联时产出什么？** 无机器面。第四节与 kb 实际状态之间没有任何自动校验——A3 说的"回填完成后删除 upstream-provenance.md"若没做，也没有东西会报警。
- **同构盲区**：本文件与 `upstream-provenance.md` 都是从同一批工作区素材抽出的抄本，抽漏的部分两份都查不出来。已知一处：`retitle-report.md` 与 Task 8/9/10a/11/12 五份报告已随 worktree 清理消失，这几个簇的裁定若有未记进 `progress.md` 的，已不可复原。
