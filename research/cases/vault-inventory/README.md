# notes 库价值盘点（2026-08-08）

> [14-元数据与分类策略](../../notes/14-metadata-and-classification.md) 的实战案例：对 `~/code/notes`（Obsidian 库）全量 426 篇笔记做价值分拣。四路并行完成，判据统一。逐文件明细见本目录四份附录。

## 判据（准入测试）

一篇笔记有资格进正式语料库，当且仅当：

1. **自足性**（12 推论 2）：脱离原语境能独立理解和使用
2. **可归型**（01 信息类型化）：能归入 reference / task / troubleshooting / concept 之一（MOC 组织文件、glossary 术语条目单独归类）
3. **时效**：volatile 内容未明显过期（2026-08 视角）

判定五档：`keep-core`（高价值直接可用）/ `keep-maybe`（有价值需加工）/ `outdated` / `noise` / `log`（日志类，不进语料库）。

## 总览

| 范围 | 篇数 | keep-core | 附录 |
|---|---|---|---|
| 20-areas/20-04-tech-tree | 214 | **161** | [01-tech-tree](01-tech-tree.md) |
| 30-resources | 84 | **60** | [02-resources](02-resources.md) |
| 10-projects | 61 | **21** | [03-projects](03-projects.md) |
| 00-inbox + 20-areas 其余 + 90-archive | 63 | **22** | [04-inbox-areas](04-inbox-areas.md) |
| **合计** | **422** | **264（63%）** | |

（另：根目录 README/INDEX 为库自述，99-system 1 篇模板未纳入判定。）

**核心结论：这是一座失守但矿脉完整的库。** 63% 的内容直接达到核心价值档，`outdated` 全库仅 1 篇——"已经无用"的观感来自三个可修复的原因：污染（claude-backup 万级文件）、失守（inbox 积压、生命周期字段停摆）、缺检索维度（价值/类型不可见），而不是内容本身。

## 六个高价值资产簇

1. **知识主线**：PKM 三公理（MECE/SSoT/Atomicity）→ 审计流程 → DITA 理论研判（10-08）→ infra-ai 规则落地——四组笔记互引、判据一致，是全库的骨架，与 dita2 仓库是同一研究线的两段
2. **glossary 术语库**（27 条）：属+种差式定义、弃用词内嵌、与 ISO 704/TBX 笔记互相印证，自洽度全库最高，可直接作为术语库正本
3. **web-security**（21 篇全部 keep-core）：原创加工的 concept 长文，统一引 OWASP/NIST/RFC，带 CVE 与真实案例
4. **react + electron 深度系列**（19+21 篇）：源码行号锚定、互链成体系，全库系统性最强
5. **agent 配置资产（已在运行）**：20-05-rc 六篇派生源 + 10-07-infra-ai 的 rules/* 五篇 + 软链机制——"单源→派生"体系已有两层实现，缺的是持续运转
6. **策展与方法论层**（30-05-ai）：prompt 评估方法论、CLAUDE.md 样本五原型分类、Rule/Skill 三层加载判据——多篇被评为超出社区通行水平的原创

## 全库最有价值（四路 Top 合并，节选）

- `code-style/CodeStyle-配置组合模式` — 跨 ESLint/Helm/Nix 的配置组合总纲，网上无现成对应物
- `10-08-dita-as-code/DITA-CMS-理论基础综合研判` — ISO 704/1087/25964 + S1000D + DITA 2.0 与 PKM 公理打通
- `browsers/Browser-渲染阻塞` — 全库技术写作水准最高
- `react/React-热点问题` — "误解-机制-判断准则"三段式，加工密度最高
- `boilerplate/AI-AGENTS写作标准` — 元层面规则写作规范，杠杆最大
- `30-05/claude-code/Rule与Skill分层` — 三层加载判据原创框架
- `go/Go-JS-TS差异锚点` — 只讲需重建的心智模型，不可再生的个人视角
- `devops/DevOps-Node版本管理` — 真实踩坑沉淀，"网上抄不到"的典型

## 旧库处置定案（2026-08-08，用户裁定）

**不迁移、不治理、不抢救。** 原 Obsidian 库整体保留为**只读矿场**——它唯一的作用是作为信息架构师与 DITA 架构师两个角色实践分类法的经验原料。知识体系在 DITA 中重新塑造，旧内容不搬运（避免把噪音、过时表述与旧组织方式一并带入）；重写时按主题回矿场查原文即可。

据此，本盘点的用途定为两项：

1. **分类法实践数据**——422 篇的 type / subject / volatility 实测分布，是新体系受控词表（subjectScheme）的经验输入
2. **重写选题池**——264 篇 keep-core 标出了"哪些主题值得在新体系中重写"；volatility 标注给出重写优先级参考（volatile 主题应带核对日期重写，stable 主题不急）

新体系设计见 [../kb-redesign/](../kb-redesign/README.md)。

## 与分类法的衔接

分拣所用维度（type / subject / volatility / verdict）与 [14](../../notes/14-metadata-and-classification.md) 定稿的维度直接对应。实测发现两点：**真实主题树比草案细得多**（react / electron / databases / networking…），词表正本已按实际语料修订；库内 `20-06/写作-PKM写作规范` 的原子化判据 + Diátaxis 四象限与本盘点判据同构——原库的写作规范与 DITA 研究殊途同归，其判据精华并入新体系的写作规则，原文件同样不迁移。
