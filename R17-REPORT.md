# R17 — domain values must be registered subject keys

## 实现说明

**位置**：`dita-tools/crates/dita_ia/src/lib.rs`（ia 载入词表 + topic 处，未新起检查器）。

- `check_values()` 里原有 domain 校验（对照 `vocab.subject("subject")` 的 `all_keys()`）已存在，
  本次把诊断信息补齐为 R17 的完整形制：命中 `R17`、报非法值本身、并给出修复提示——
  `domain 值 "X" 不是词表已注册的 subject key（R17）——请在 subjectScheme 注册该键，或改用已注册值`。
  之所以只能落在 dita-tools 而非 `enumerationdef`：`enumerationdef`/`attributedef` 只能绑属性，
  `domain` 是 `<data name="domain" value="X"/>`，一个元素，没有 attributedef 可挂。
- 新增 `empty_leaves_by_branch()`（`lib.rs`）：反向报表。取 `Subject::leaf_keys()` 的等价递归
  （`count_empty_leaves`），减去所有 topic 已声明的 `domain` 值集合，得到"已注册但零 topic
  挂靠"的空叶子，**按顶层分支（`lang`/`web`/`ai`/… 即 root `subject` 的直接子节点）归并计数**，
  存进 `IaReport::empty_leaves_by_branch: Vec<(String, usize)>`，按计数降序排列（同计数按分支
  key 升序）。只取叶子、不取所有空节点：内部节点（如 `writing`）零挂靠时必然是它所有子节点都
  零挂靠，两处都报等于同一个缺口报两遍。
- `render.rs::print_exceptions()`（ia 现有"需要处理"段）新增一行，**仅 `--details` 时打印**
  （`print_exceptions` 新增 `details: bool` 形参）：
  `词表空叶子（已注册但零 topic 挂靠）合计 N 个：ai(14)、content-engineering(8)、…`——
  按分支归并后条目数天然有限（顶层分支数量级），不再需要像之前扁平列 key 那样做截断。
- `IaReport` 新字段无需改任何调用方结构字面量——workspace 内没有别处直接构造 `IaReport`，
  只有 `build_report()` 一处，`apps/dita_cli` 只读字段不构造。

### 审查后修订（第二轮，三处）

1. **空叶子报表降级到 `--details`，且按顶层分支归并展示**（原实现是默认打印、扁平列 8 个
   key 后截断）。`empty_subject_leaves: Vec<String>` 字段整体替换为
   `empty_leaves_by_branch: Vec<(String, usize)>`；`print_exceptions` 加 `details` 形参，
   该行放进 `details && !empty_leaves_by_branch.is_empty()` 分支。展示格式改为
   `分支(数量)、分支(数量)…合计 N`，与需求给的示例（`ai(12)、security(9)…合计 58`）一致。
2. `docs/architecture.md` 规则归属表：R17 行去粗体，改用 R12–R15 的朴素行样式；同时补上此前
   缺失的 R16 行（既有缺口，本次一并收口，未额外改动其它行）。
3. 本文件"诊断 0 error / 0 warning"一句改写为明确的核实结论（见下方"ia 实跑输出摘要"），
   并说明该行在诊断为零时本就不会打印——之前的写法容易读成"我看到了这行输出"，实际是
   "诊断计数为零，因此这行按 `print_exceptions` 的逻辑被跳过、终端上根本不出现"。

## 测试清单

`dita-tools/crates/dita_ia/tests/report.rs`（复用既有 `tests/fixtures/mini` 词表/topic 夹具，
未改动夹具本身——其 `demo-a`/`demo-b1`/`empty-a`/`nomap` 叶子结构恰好覆盖三种情形）：

- `r17_a_registered_domain_value_is_not_flagged`：`good.dita`/`landscape.dita`（domain=demo）、
  `nested.dita`（domain=demo-b1，祖孙键）均不产生 R17 错误。
- `r17_an_unregistered_domain_value_errors_with_a_fix_hint`：`bogus-domain.dita`
  （domain="not-a-subject-key"）报错，消息含 `R17` 与"注册"提示词。
- `r17_empty_leaves_by_branch_counts_unclaimed_leaves_per_branch`（审查后按新字段改写）：
  - `demo` 分支计数 = 1（只有 `demo-a` 未挂靠；`demo-b1` 被 `nested.dita` 挂靠，不计入）；
  - `empty` 分支计数 = 1（`empty-a`）；
  - `nomap` 分支计数 = 1（它自己既是顶层分支又是叶子）；
  - 三分支合计 = 3。
- `r17_empty_leaves_by_branch_is_empty_without_a_vocabulary`：无词表时该字段为空，不瞎猜。

既有回归测试 `a_typo_in_domain_falls_into_the_bucket_and_errors`（检查消息含被拒绝的值本身）
未改动仍绿——消息模板只是追加了 R17 标记与提示，未改变已断言的子串。

**验证**：`cargo test --workspace`、`cargo clippy --workspace --all-targets`、
`cargo fmt --all -- --check` 三绿，零告警（第二轮修订后重新跑过，结果相同）。

## ia 实跑输出摘要（kb 语料）

`cd kb && ../dita-tools/target/debug/dita-tools ia`（默认，无 `--details`）——空叶子报表不再
出现，"需要处理"段只剩原有几行：

```
需要处理：
  · 2 篇不在任何分支下（只被交付物 map 引用）
  · 维度盲区 35 个
  · @maturity 有 1 个受控值从未被用过
  · @tool 有 1 个受控值从未被用过
  · @dimension 有 30 个受控值从未被用过
```

`../dita-tools/target/debug/dita-tools ia --details`——多出按分支归并的空叶子行：

```
  · 词表空叶子（已注册但零 topic 挂靠）合计 58 个：ai(14)、content-engineering(8)、
    security(8)、engineering(7)、data(6)、web(5)、lang(4)、network(4)、foundations(2)
```

**核实结论：当前 kb 语料下 `report.diagnostics` 的 error/warning 计数均为 0，所以
`print_exceptions` 里"诊断 N error / M warning"那一行本身不会被打印**（该行只在
`errs > 0 || warns > 0` 时才 push 进输出，`errs_present()` 同理决定是否打印"诊断明细"）——
这不是"我看到了 0 error / 0 warning 这行文字"，而是"两个计数器确认都是 0，所以那行按代码逻辑
被跳过、终端上不出现任何诊断相关文字"。已用 `grep -i "诊断\|error"` 对完整输出核实，无匹配。

### 关于"预期红"的核实结果（重要，与任务描述不符，如实记录）

任务描述预期 `writing-typing.dita`（domain=doc-types）与 `writing-llm-friendly.dita`
（domain=llm-friendly）会被 R17 报错，作为"验证有效性的证据"。**实测并非如此**：

- `kb/vocab/subjectScheme.ditamap` 第 150–151 行，`content-engineering/writing` 分支下：
  ```xml
  <subjectdef keys="doc-types"/>          <!-- ADR/README/API 文档 -->
  <subjectdef keys="llm-friendly"/>       <!-- RAG 友好写作/语义分块 -->
  ```
  这两个 key **已经是合法注册的 subject key**（`git blame` 显示这两行来自
  `ee198c4 merge docs and kb as regular directories`，即本 worktree 可见历史里最早的一次提交，
  不是我这次改动引入、也不是最近才补注册的）。
- 按 R17 字面定义（"X 必须是 subjectScheme 主题树里已注册的 subject key"），`doc-types` 与
  `llm-friendly` 两个值本身就满足条件，所以正确实现的检查**不会**把它们标红。
  `dita-authoring-guide.dita`（domain=dita）同理合法、不报错——这部分与任务描述一致。
- 为确认检查机制本身确实端到端接好（而非"因为实现有洞才不报错"），我临时把
  `writing-typing.dita` 的 domain 改成一个真正不存在的键 `zz-not-a-real-key`，跑 `ia --details`
  确认立刻报出：
  ```
  ❌ .../topics/content-engineering/writing-typing.dita: domain 值 "zz-not-a-real-key"
     不是词表已注册的 subject key（R17）——请在 subjectScheme 注册该键，或改用已注册值
  ```
  同时空叶子（按分支归并后）`content-engineering` 分支的计数 +1（`doc-types` 从"已挂靠"变回
  "空叶子"）。验证后已用 `git checkout -- topics/content-engineering/writing-typing.dita`
  完整还原，未提交此改动，`git status` 确认改动范围只剩本次实现涉及的文件。
- **结论**：R17 检查本身工作正常（对未注册值会报错、对已注册值不报错、反向报表能按分支识别
  空叶子）；只是"doc-types / llm-friendly 当前会触发红"这条具体预测与语料实际状态不符——它们
  在这份 worktree 继承的语料里从一开始就已经被注册进 subjectScheme 了。是否要把它们从词表里
  摘除（让它们重新变成真正的孤儿值）是内容/治理层面的判断，超出"实现一条检查"的范围，按指令
  我没有为了让预期变红而去改 `kb/topics` 或 `kb/vocab` 的既有内容。

## 提交

- `feat(dita-tools): R17 — domain values must be registered subject keys`（第一轮实现）
- `fix(dita-tools): R17 review — empty-leaf report gated and grouped`（第二轮，审查后三处修订，
  见上方"审查后修订"）
- 均未 push，未合并回主工作区。
