# 剩余待办（2026-08-08 搁置）

> Phase 0–4 已完成——核心验证全部通过、工具链已定（DITA-OT 4.4 + org.lwdita，全现成）。
> 此后**搁置kb-redesign，转去调研 agent CLI 工具链生态**。回来从本清单接续。
> （TaskList 里的 #7–13 是会话级副本，会随会话丢失；以本文件为准。）

## ⭐ 两条全程主线（最高优先，贯穿每篇 —— 详见 terminology-and-bilingual-design.md）

这两条是**全程重点**,不是一次性任务,Phase 5 每篇重写都在它们的约束下进行:

- **术语一致性**:glossentry 术语库 + `<term keyref>` + Schematron R7 + AI 后处理脚本。治"AI 帮写、术语被带偏"。落地=下面 #7(glossentry shell)+ #8(术语库,做双语条目)+ 激活 R7。
- **双语**(英文给 AI、中文给你):形态待定(推荐 A 分两条线 + 术语库双语条目)。见设计文档待审决策点。

## 领域级建设纪律：维度完整性（2026-08-09 定，见 dimension-completeness.md）

不是主线(不是每篇都带的属性),是建领域时的一次性动作 + 领域级产物：梳理一个领域先对标建完整维度框架,再在完整框架上按 80/20 切快速上手。落地三件,并入第一批:

- subjectScheme 加横切维度值集;每个领域全景 topic(tech-landscape)声明本领域完整维度清单;topic 用 `@subjectrefs` 标覆盖的维度。
- Schematron R9(领域 map 必挂全景)、R10(quickstart 必 xref 全景 + 取舍声明)。
- agent-rules 加《维度完整性写作规则》。
- ~~构建报告算维度覆盖度、列盲区~~ ✅（kb/scripts/dimension-coverage.py，样板验证 0/10→2/10）。

## Phase 5 批量重写（核心剩余，串行主线 #7→#8→#9）

1. ~~**造 reference-kb + glossentry-kb shell**~~ ✅（2026-08-09 完成）— 照 concept-kb 模板装配，含四属性域（含新增 @dimension），从 DITA-OT 4.4 实模块核对 URN/依赖，dita validate 通过。前置已就位。
2. ~~**建术语库首批**~~ ✅（2026-08-09）— 矿场 30-06 的 27 条按重塑原则甄别：通用 9 + 工具名 3 = 12 条双语 glossentry，Obsidian 特有等 18 条弃（重塑非迁移）。挂进 glossary.ditamap，keys=term-* 供 `<term keyref>` 引用，全 validate 通过。
3. **批量重写内容** — 按 264 篇 keep-core 选题池滚动，volatile 域优先（ai/coding-agents、security 生态类，时效在流失）。遵循重写约定（见 Phase3-回顾），边写边回填 keyref。依赖 1、2。

## Phase 6 部署与衔接

4. **部署脚本** — 把构建产物放到目标位置（项目根 CLAUDE.md、全局 ~/.claude/、AGENTS.md 等）。build-agent-rules.sh 已出产物，可随时做。
5. **检索/RAG 衔接** — 远期/可选，dita2 笔记 15 的路线。独立立项。

## 技术债（按需，不阻断）

6. **schema 补齐** — task-kb/troubleshooting-kb shell（写那类内容时造）、约束模块（强制 shortdesc）、catalog 解耦。
7. **交叉标引机制** — `@subjectrefs` 多值，落实"知识图谱三落点"与横跨多域主题（如性能）的关联。

## 回来时的入口

- **状态与规划**：本目录（设计稿 README、评审单、差距报告、角色分工、Phase3-回顾、Phase4-结论、本清单）。
- **成品仓库**：`~/code/kb`（词表 v0.2、schema 零件、topics、构建脚本、README 含构建命令）。
- **重写约定**：phase3-review.md 末尾五条。
- **核心判断**：剩下全是**扩量 + 收尾，无验证性风险**——shell/写作规则/构建管道三个模板都验证过，约定也定了，是可预期的重复劳动，不是探索。

## 与"调研工具链"的关系

接下来要调研的 agent CLI 工具链（claude-code/codex/antigravity 的安装/配置/生态/最佳实践）本身就是本知识库 **ai/coding-agents 分支**将来要装的内容。调研产出可以直接按写作规则重写进库——即调研成果天然是 Phase 5 的第一批高价值语料（且是 volatile、优先级高）。搁置期的工作和回来的工作是连着的，不是断的。
