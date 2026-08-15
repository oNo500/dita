# Phase 4 结论：首个交付物 + 工具链决策门（2026-08-08）

> Phase 4 是工具链决策门——第一次把 DITA 源构建成实际交付物，一直悬着的"工具链未定"在此有了实证答案。

## 验证了什么

核心主张"DITA 单源 → 多工具变体"成立，有 diff 铁证：

- 一份 rule 源（`agent-rules-core.dita`，含通用规则 + tool 条件段）
- 一个交付物 map（`chunk="combine"`）
- 两个 DITAVAL（tool-claude-code / tool-codex）
- 构建两次 → 两个 markdown，**唯一差异是工具特有段，通用部分逐字一致**

改一处源、重跑构建，两个工具变体同步更新——直接兑现痛点第 1 条（agent CLI 切换不平滑、配置重复维护）。

## 工具链决策（"未定"→"实证选定"）

| 环节 | 选择 | 依据 |
|---|---|---|
| 构建器 | DITA-OT 4.4（已装） | 自带 OASIS 2.0 语法 + 校验 |
| markdown transtype | org.lwdita 5.9.1 的 `markdown_github`（已装） | 产出格式干净可用 |
| 单文件合并 | `chunk="combine"` | 多 topic 合成单文件规则 |
| 条件变体 | `@props/tool` + DITAVAL | 按工具过滤 |

**全部现成，不需自定义 transtype 或额外工具。** 工具链方向从"凭感觉"变成"实证选定"。

## 产物与部署

- 构建产物：`out/<tool>/agent-rules.md`（= 该工具的规则文件雏形，即 CLAUDE.md / AGENTS.md）
- 一条命令：`scripts/build-agent-rules.sh`
- 部署（属 Phase 6 工具链衔接）：把产物放到目标位置——项目根 CLAUDE.md、全局 ~/.claude/ 等。这是"最后一公里"，Phase 6 做。

## 待完善（不阻断）

- `markdown_github` 产出的 index.md 是 TOC 壳，部署时取 agent-rules.md 即可（脚本已 cp 出）
- 多 rule topic 时 combine 顺序、标题层级需微调
- 部署脚本（放产物到目标位置）属 Phase 6

## 意义

Phase 4 通过 = 整个方法论闭环验证：**分类（词表）→ schema → 写作规则 → 重写 → 构建交付物**，全链跑通、各环节经审查。核心主张成立，可以放心批量。工具链不再是悬空的未知——它是已装、已验证、已知格式的确定路径。
