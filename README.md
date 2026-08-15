# dita

一条链的三段：**研究 DITA → 用它建知识库 → 造工具让规则跑起来**。

四个目录承载三层一契约：`research` 定判据、`kb` 是资产正本、`dita-tools` 是工具平台（终态见能力地图），`docs` 放管全局的文档。这份 README 是仓库的入口；边界、契约与工具生态终态见 [docs/architecture.md](docs/architecture.md)。日常操作统一走根目录 `just`（`just --list` 看全部）。

| 目录 | 是什么 | 产出给下游的 | 状态 |
|---|---|---|---|
| [`docs/`](docs/architecture.md) | 仓库级文档 | 架构与边界、能力地图 | 正本 |
| [`research/`](research/README.md) | DITA 2.0 研究笔记 + 设计案例（原 dita2） | **判据与规则规格**（为什么这么定） | 笔记 16 篇，设计定案 |
| [`kb/`](kb/README.md) | 知识体系正本 | **内容 + 唯一事实源**（`vocab/subjectScheme.ditamap`） | 建设中，22 篇 |
| [`dita-tools/`](dita-tools/README.md) | Rust 工具链 | **规则的执行**（校验、IA 视图） | map 层 IA 视图可用；topic 层在建 |

> 依赖方向是单向的：`docs` 定规则 → `kb` 遵守规则并定义合法值 → `dita-tools` 读词表执行规则。**下游不得反向定义上游**——工具里不许再抄一份值集，规则的"为什么"不写在工具的 README 里。

## 现在到哪了

- **内容**：22 篇 topic（12 条术语 + 10 篇内容）。九个领域 map 里 **7 个仍是空骨架**，只有 `ai` 和 `content-engineering` 挂了内容。
- **规则**：R1–R10 已实现并全过；**R11（`@dimension` 枚举校验）尚无实现，且归属未定**（见架构文档的规则归属表）。
- **工具**：`dita-tools ia` 已可出知识树、空领域、孤儿 topic（2026-08-15 实测）。但**没有 topic 解析器**——读不到 `@dimension`/`@maturity`，因此还答不了"哪个域缺哪类内容"。补它是当前主线。
- **交付物**：单源 → 双工具变体（`CLAUDE.md` / `AGENTS.md`）已跑通。

## 环境

工具链装在 `~/ws/tools`（用户态，不入仓库），**引导脚本在仓库里**：`just setup` 幂等装齐全部依赖，版本号以 `scripts/setup-env.sh` 顶部为唯一出处。当前状态：

| 需要 | 用途 | 本机状态 |
|---|---|---|
| Java 17+ / DITA-OT 4.4 | 校验、构建、交付物 | ✅ 已装（`~/ws/tools`） |
| python3 | 覆盖度、术语扫描 | ✅ 系统自带 |
| Rust + C 链接器 | dita-tools | ✅ 已装（rustup + build-essential），`cargo test` 全过 |

```bash
cd kb && sh scripts/review.sh               # 结构校验 + 业务规则 + 覆盖度 + 术语扫描
cd kb && sh scripts/build-agent-rules.sh    # 单源 → out/<tool>.md
cd kb && dita-tools ia                      # IA 全景：分支概览、维度盲区、孤儿
```

> `dita-tools` 需先安装：`cd dita-tools && cargo install --path apps/dita_cli`。它默认按 kb 的
> 相对路径找文件，所以在 `kb/` 下运行；改了代码要重新 install 才会生效。

## 关于旧文档里的路径

`docs/cases/` 下的评审单、阶段总结等是**历史记录，按当时原样保留，不做路径翻新**。它们提到的旧位置对应现在：

| 旧文档里写的 | 现在 |
|---|---|
| `~/code/dita2` | 本仓库 `research/` |
| `~/code/kb` | 本仓库 `kb/` |
| `~/code/notes` | `~/ws/projects/notes`（只读矿场，从不迁移内容） |

三者在 2026-08-15 合并为本仓库。
