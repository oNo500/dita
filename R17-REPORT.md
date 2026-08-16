# R17 — domain values must be registered subject keys

## 实现说明

**位置**：`dita-tools/crates/dita_ia/src/lib.rs`（ia 载入词表 + topic 处，未新起检查器）。

- `check_values()` 里原有 domain 校验（对照 `vocab.subject("subject")` 的 `all_keys()`）已存在，
  本次把诊断信息补齐为 R17 的完整形制：命中 `R17`、报非法值本身、并给出修复提示——
  `domain 值 "X" 不是词表已注册的 subject key（R17）——请在 subjectScheme 注册该键，或改用已注册值`。
  之所以只能落在 dita-tools 而非 `enumerationdef`：`enumerationdef`/`attributedef` 只能绑属性，
  `domain` 是 `<data name="domain" value="X"/>`，一个元素，没有 attributedef 可挂。
- 新增 `empty_leaves()`（`lib.rs`）：反向报表，取 `Subject::leaf_keys()`（`dita_vocab` 既有 API，
  只返回子树最末端、无子节点的 key）减去所有 topic 已声明的 `domain` 值集合，即"已注册但零 topic
  挂靠"的空叶子清单，排序后放进 `IaReport::empty_subject_leaves: Vec<String>` 新字段。
  只取叶子、不取所有空节点：内部节点（如 `writing`）零挂靠时必然是它所有子节点都零挂靠，
  两处都报等于同一个缺口报两遍。
- `render.rs::print_exceptions()`（ia 现有"需要处理"段）新增一行：
  `词表空叶子 N 个（已注册但零 topic 挂靠）：key1、key2、…（超过 8 个截断，标注共 N 个，
  与既有的"受控值使用"截断风格一致）`。
- `IaReport` 新字段无需改任何调用方结构字面量——workspace 内没有别处直接构造 `IaReport`，
  只有 `build_report()` 一处，`apps/dita_cli` 只读字段不构造。

## 测试清单

新增于 `dita-tools/crates/dita_ia/tests/report.rs`（复用既有 `tests/fixtures/mini` 词表/topic 夹具，
未改动夹具本身——其 `demo-a`/`demo-b1`/`empty-a`/`nomap` 叶子结构恰好覆盖三种情形）：

- `r17_a_registered_domain_value_is_not_flagged`：`good.dita`/`landscape.dita`（domain=demo）、
  `nested.dita`（domain=demo-b1，祖孙键）均不产生 R17 错误。
- `r17_an_unregistered_domain_value_errors_with_a_fix_hint`：`bogus-domain.dita`
  （domain="not-a-subject-key"）报错，消息含 `R17` 与"注册"提示词。
- `r17_empty_subject_leaves_lists_registered_but_unclaimed_leaves`：空叶子报表含
  `demo-a`/`empty-a`/`nomap`；不含被 `nested.dita` 挂靠的 `demo-b1`；不含非叶子的 `demo`/`empty`
  （即便它们自己也没有 topic 直接挂）。
- `r17_empty_subject_leaves_is_empty_without_a_vocabulary`：无词表时该字段为空，不瞎猜。

既有回归测试 `a_typo_in_domain_falls_into_the_bucket_and_errors`（检查消息含被拒绝的值本身）
未改动仍绿——消息模板只是追加了 R17 标记与提示，未改变已断言的子串。

**验证**：`cargo test --workspace`、`cargo clippy --workspace --all-targets`、
`cargo fmt --all -- --check` 全绿，零告警。

## ia 实跑输出摘要（kb 语料，`cd kb && ../dita-tools/target/debug/dita-tools ia --details`）

"需要处理"段：

```
需要处理：
  · 2 篇不在任何分支下（只被交付物 map 引用）
  · 维度盲区 35 个
  · @maturity 有 1 个受控值从未被用过
  · @tool 有 1 个受控值从未被用过
  · @dimension 有 30 个受控值从未被用过
  · 词表空叶子 58 个（已注册但零 topic 挂靠）：agent-patterns、ai-security、antigravity、
    api-design、appsec、architecture、authz、browsers …（共 58）
```

**诊断 0 error / 0 warning** —— 当前 kb 语料没有触发 R17 的红。

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
  同时空叶子计数从 58 变 59（`doc-types` 从"已挂靠"变回"空叶子"）。验证后已用
  `git checkout -- topics/content-engineering/writing-typing.dita` 完整还原，未提交此改动，
  `git status` 确认改动范围只剩本次实现涉及的 5 个文件。
- **结论**：R17 检查本身工作正常（对未注册值会报错、对已注册值不报错、反向报表能识别空叶子）；
  只是"doc-types / llm-friendly 当前会触发红"这条具体预测与语料实际状态不符——它们在这份
  worktree 继承的语料里从一开始就已经被注册进 subjectScheme 了。是否要把它们从词表里摘除
  （让它们重新变成真正的孤儿值）是内容/治理层面的判断，超出"实现一条检查"的范围，
  按指令我没有为了让预期变红而去改 `kb/topics` 或 `kb/vocab` 的既有内容。

## 提交

- `feat(dita-tools): R17 — domain values must be registered subject keys`
  （见下方 commit 哈希；单个提交，改动 5 个文件：`dita_ia` 的 `lib.rs`/`render.rs`/
  `tests/report.rs`、`kb/schema/rules.sch`、`docs/architecture.md`）
- 未 push，未合并回主工作区。
