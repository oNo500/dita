# dita

一条链的三段：**研究 DITA → 用它建知识库 → 造工具让规则跑起来**。

三个目录不是三个独立项目，是同一条链上前后相接的三层。这份 README 是仓库的入口；边界与契约的正本见 [docs/架构与边界.md](docs/架构与边界.md)。

| 目录 | 是什么 | 产出给下游的 | 状态 |
|---|---|---|---|
| [`docs/`](docs/README.md) | DITA 2.0 研究笔记 + 设计案例 | **判据与规则规格**（为什么这么定） | 笔记 16 篇，设计定案 |
| [`kb/`](kb/README.md) | 知识体系正本 | **内容 + 唯一事实源**（`vocab/subjectScheme.ditamap`） | 建设中，22 篇 |
| [`dita-tools/`](dita-tools/README.md) | Rust 工具链 | **规则的执行**（校验、IA 视图） | 骨架，本机暂不可编译 |

> 依赖方向是单向的：`docs` 定规则 → `kb` 遵守规则并定义合法值 → `dita-tools` 读词表执行规则。**下游不得反向定义上游**——工具里不许再抄一份值集，规则的"为什么"不写在工具的 README 里。

## 现在到哪了

- **内容**：22 篇 topic（12 条术语 + 10 篇内容）。九个领域 map 里 **7 个仍是空骨架**，只有 `ai` 和 `content-engineering` 挂了内容。
- **规则**：R1–R10 已实现并全过；**R11（`@dimension` 枚举校验）尚无实现，且归属未定**（见架构文档的规则归属表）。
- **工具**：`dita-tools ia` 可出知识树与孤儿 topic，但**没有 topic 解析器**——读不到 `@dimension`/`@maturity`，接不上任何业务规则。
- **交付物**：单源 → 双工具变体（`CLAUDE.md` / `AGENTS.md`）已跑通。

## 环境

本机（VPS）工具链装在 `~/ws/tools`，不入仓库：

| 需要 | 用途 | 本机状态 |
|---|---|---|
| Java 17+ / DITA-OT 4.4 | 校验、构建、交付物 | ✅ 已装（`~/ws/tools`） |
| python3 | 覆盖度、术语扫描 | ✅ 系统自带 |
| Rust + **C 链接器** | dita-tools | ⚠️ Rust 已装，**缺 `build-essential`，无法链接** |

```bash
cd kb && sh scripts/review.sh          # 结构校验 + 业务规则 + 覆盖度 + 术语扫描
cd kb && sh scripts/build-agent-rules.sh   # 单源 → out/<tool>.md
```

## 关于旧文档里的路径

`docs/cases/` 下的评审单、阶段总结等是**历史记录，按当时原样保留，不做路径翻新**。它们提到的旧位置对应现在：

| 旧文档里写的 | 现在 |
|---|---|
| `~/code/dita2` | 本仓库 `docs/` |
| `~/code/kb` | 本仓库 `kb/` |
| `~/code/notes` | `~/ws/projects/notes`（只读矿场，从不迁移内容） |

三者在 2026-08-15 合并为本仓库。
