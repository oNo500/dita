# Phase 3 试点回顾（决策门，2026-08-08）

> 试点重写 2 篇 agent 规则 concept（`kb/topics/ai/agent-rule-loading` + `agent-context-verification`），走通完整工作流。本文回顾成本、暴露的缺口、批量前置——这是"要不要照此批量"的决策门。

## 试点做了什么

完整工作流跑了一遍：**定位矿场原文 → 读原文 → 按写作规则提炼重写 → 标注 maturity/volatility → 填领域 map → dita validate → commit**。两篇 topic + ai 领域 map 全部校验通过，一次过、无返工。

## 顺畅的部分

- **写作规则真的在指导重写**：自足性（去掉 Obsidian 的 wikilink/callout）、类型化（都是 concept）、来源纪律（标 maturity/volatility）、提炼（砍掉最易腐的具体配置字段）——四条都自然用上了。
- **schema 够用**：concept-kb shell 一次通过；审查后补回的 softwareDomain 正好用上（`filepath` 标 SKILL.md、`cmdname` 标命令）——印证补域判断正确，不是过度。
- **单篇机械成本低**：读 + 重写 + 校验，瓶颈不在操作。

## 暴露的缺口（批量前要处理）

1. **术语 keyref 悬空**：两篇用了 `<term>`（rule/skill/glob）但没 keyref——因为 glossary 还空。写作规则说"术语首现建 glossentry + keyref"，试点没做到。**批量前须先建术语库首批**，让 keyref 有目标。
2. **只有 concept 的 shell**：批量必然需要 reference（速查/配置类矿场笔记很多）和 glossentry（术语库）。**这两个 shell 是批量前置**（照 concept-kb 模式造，已验证机制）。
3. **1:1 重写 vs 拆分**：原文《Rule与Skill分层》含大量具体配置（token 预算、配置字段名）。我砍进了 volatile / 直接略去——但这些对"实操"有用。更好的做法可能是**一篇矿场笔记拆成 concept（判据）+ reference（具体配置）**，而非 1:1。批量前要定一条拆分标准。
4. **交叉标引还没机制**：agent-rule-loading 与 context-engineering 交叉相关，现在只在 map 注释里提了一句，没有真正的 `@subjectrefs` 多值标引。设计里"知识图谱三落点"靠交叉标引，实操层还缺这个机制。
5. **map 组织约定未定**：现在用 `<topichead>` 手动分组，批量时该按 subjectScheme 子分支系统化组织。

## 成本判断

**机械操作顺畅，真正的成本在判断**——提炼度（砍什么留什么）、拆分（concept 还是 concept+reference）、术语归口。这些判断不难，但需要一致标准，否则批量出来的东西风格漂移。

## 决策门结论

**工作流可行，可以批量——但先补三个前置，否则批量成本不稳定：**

1. 造 reference-kb + glossentry-kb shell（照 concept-kb 模式，机械）
2. 建术语库首批（从矿场 30-06 的 27 条 glossary 重写，让 keyref 有目标）
3. 定重写约定（见下）

补齐后进入 Phase 5 批量重写（volatile 域优先）。这三件本身就是 Phase 5 的第一批工作，不是额外开销。

## 重写约定（试点 + 审查提炼，批量前定稿）

1. **提炼度**：砍"实现层"（token 数字、配置字段名、工具专有字段），保"判据层"（触发条件、判据、例外、取舍）。检验：砍完判据是否仍自洽、可操作。
2. **提炼护栏（审查发现，必配）**：提炼是"篇内抽薄细节"，**不能变成"整节丢原创判断"**。砍掉的整节若含独立原创判断，必须有去处——独立成篇，或回矿场挂"待独立成篇"标记。试点中 agent-rule-loading 砍掉的"编排"节即按此另立为 agent-skill-orchestration。否则批量会系统性漏掉原创思考。
3. **拆分**：一篇矿场笔记若判据 + 大量具体配置并存，拆成 concept（判据）+ reference（配置），而非 1:1。
4. **术语**：概念术语过渡期用 `<b>`，术语库建成后回填 `<term keyref>`——首批候选就是各篇加粗的概念词。
5. **map 组织**：按 subjectScheme 子分支系统化组织，`<topichead>` 分组对齐分支。
