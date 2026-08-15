# AI 写作规则集：设计（待审）

> 工具链层 0 的缺口，也是 AI 主笔质量的关键。定位：AI 主笔 DITA，质量靠两条腿——**事前 agent 规则（本集，正面教怎么写对）+ 事后机器兜底（review.sh，反面查哪写错）**，两条一一咬合。它本身是 kb 内容（并入 agent-rules 交付物），AI 读它来写 kb——用 kb 管"让 AI 写好 kb 的规则"。

## 一、为什么需要

- 缺了它，AI 靠"感觉"写 DITA：类型选错、精细语义糊成一个 `codeph`、漏标属性/来源/维度 → 全压给机器兜底反复返工。
- 有了它，一次写对率高，`review.sh` 只兜漏网的。这正是当初痛点"AI 帮写、术语被带偏、质量不稳"的完整解法：不指望 AI 自觉，靠**规则约束（事前）+ 机器兜底（事后）**。
- 它是 Phase 4 那条 agent-rules 线（`agent-rules.ditamap`，chunk=combine）的直接延伸。

## 二、规则分八组（AI 写一篇时依次要决定的事）

1. **选类型与题材** —— 照《内容类型框架》：先定维度（讲哪个方面）、再定类型（Diátaxis 象限 + DITA concept/task/reference/troubleshooting）、再定题材（outputclass）。给判断法，不靠感觉。
2. **行内精细语义** —— 方式 A 准确优先：命令用 `cmdname`、路径 `filepath`、API `apiname`、XML 元素 `xmlelement`、代码 `codeph`/`codeblock`。**不为省事糊成一个 `codeph`**——你的内容价值在精确。给"何时用哪个"的对照。
3. **元数据标注** —— `maturity`（默认 draft）、`volatility` 必显式标、`dimension` 标覆盖的维度（可多值）；未标 volatility 会被 R2 拦。
4. **术语一致性** —— 关键术语查术语库：有则 `<term keyref="term-x">`，没有则先建 glossentry 再引，**不打字面词**。
5. **双语** —— 正文中文单源；术语库双语条目；英文按需 AI 翻，不预填。
6. **来源** —— concept/reference 必有来源：一两个主来源用 prolog `<source>`，多来源用末尾来源节，外链统一 `scope="external"`。
7. **维度完整性** —— 建一个新领域，第一步先建全景 topic（tech-landscape）、用 `planned-dimension` 声明完整维度清单、标盲区；内容按 80/20 从完整图上切；quickstart 挂靠全景。（这是《维度完整性-方法与约束》里 agent 规则那环的落地。）
8. **结构模板** —— 每个类型/题材的固定骨架 + 必校验项，直接对接《内容类型框架》的题材表（如 quickstart：目标→前置→步骤→next）。

## 三、每组规则 ↔ 机器兜底 R 的对应（两条腿咬合）

| 写作规则（事前·正面教） | 机器兜底（事后·反面查） |
|---|---|
| 每篇写 shortdesc | R1 |
| 显式标 volatility / maturity | R2、R4、R5 |
| verified 内容标核对日期 `data name="reviewed"` | R3 |
| 受控值只用词表里的 | R4/R5/R6 + 构建期 enumerationdef |
| 关键术语用 term keyref | R7 |
| concept/reference 必有来源 | R8 |
| 建域先全景、quickstart 挂全景 | R9（脚本）/R10 + 覆盖度报告 |

**AI 事前照规则写 = 机器事后必过。** 规则是正面手册，rules.sch 是反面拦网，同一套标准的两面。

## 四、在 kb 里怎么组织

- **存**：`topics/engineering/` 下一组（tooling 分支下"给 AI 的 DITA 写作规则"）。
- **并入交付物**：`maps/deliverables/agent-rules.ditamap`（已有，chunk=combine），单源输出成各工具的 CLAUDE.md/AGENTS.md 变体。
- **分篇**：一篇**总纲**（概览 + "写一篇的决策流程"）+ 各组细则；不重写已有的，引用它们。

## 五、与已有的关系（不重复造）

- `topics/writing/*`（原子化/来源/LLM 友好/类型 4 篇）：**通用写作原则**，人和 AI 都遵循。本集**引用，不重写**。
- 《内容类型框架》：选类型的依据，本集第 1、8 组指向它。
- 《维度完整性-方法与约束》：第 7 组 = 那份 agent 规则部分的落地。
- `agent-rules-core.dita`：现有 agent 规则核心，本集与之并列/并入同一交付物。

**一句话：本集是"操作手册"，把已有原则 + 内容类型框架 + 维度方法串成"AI 主笔时照着做"的流程，只补 DITA 操作层（选类型、用标签、标属性、挂术语/来源/维度），不重复造原则。**

## 六、落地

- 写 `topics/engineering/dita-authoring-guide.dita`（总纲：决策流程 + 八组索引）+ 必要时拆分组细则。
- 并入 `agent-rules.ditamap`，单源多变体输出。
- 用法：AI 主笔前读它（作为上下文/system 规则），写完跑 `review.sh` 兜底。
- **首批建议**：先做和机器兜底 R 直接对应的组（1 类型、2 语义、3 元数据、4 术语、6 来源）——它们能立刻和 `review.sh` 咬合、一次见效；维度（7）、双语（5）组引用已有设计即可。

## 落地进度

- ✅ 首篇 `topics/engineering/dita-authoring-guide.dita`（总纲：决策流程 + 8 组，首批详写类型/语义/元数据/术语/来源，维度双语引用已有）。已并入 `agent-rules.ditamap`；自身 validate + 业务规则零违规（即"照规则写"的活样板）。
- ⬜ 按需拆分组细则；写作原则继续引 `topics/writing/*`，不重写。

## 七、待审（已按推荐通过 2026-08-09）

1. **组织**：一篇总纲 + 引用已有细则，还是独立把 8 组全写全？（建议总纲 + 引用，避免和 writing/* 重复）
2. **存哪**：`engineering`（tooling）分支下，还是单开 `meta/authoring` 目录？（建议 engineering）
3. **与 writing/* 的边界**：本集只做 DITA 操作层、写作原则引 writing/*，对吗？
4. **首批范围**：先做与机器兜底 R 对应的 5 组（类型/语义/元数据/术语/来源），维度双语引用已有？
