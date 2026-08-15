# 附录 03 · 10-projects 分拣明细（61 篇）

> 范围：`10-projects/`。keep-core **21**（dev-config 1 / pkm 3 / 面试 1 / boilerplate 5 / infra-ai 8 / dita 3）。

## 子目录小结

- **10-00-dev-config**（1）：仅 1 篇，但是"规则文件在 Obsidian 维护、软链接到项目"这一机制的正本说明，与 infra-ai 联动。
- **10-02-pkm**（6）：质量整体高。维护流程 + 插件清单是可执行的审计工作流；仅"Obsidian-配置"偏旧。
- **10-03-面试准备**（12）：典型"已降温项目"。内容多为常青技术知识（Anki 卡、cheatsheet），无一是垃圾，但行动距离已远——整体 keep-maybe，宜下沉；Electron 面试题质量最高。
- **10-05-blog**（3）：停滞项目。备忘录含活配置事实（服务器 IP/域名）值得留；参考资源链接集 deadline 已过。
- **10-06-boilerplate**（14）：两极分化。NestJS-架构、AGENTS 写作标准、CodeStyle 两篇是全库最能直接转化为项目模板的素材；LiteBot 是空壳，MiniProgram/Vercel-AI-SDK/TODO 是过程性 log。MOC 引用的 [[API-设计参考]] 等在 tech-tree（已验证存在）。
- **10-07-infra-ai**（22）：结构清晰的"活项目"。rules/* 5 篇实为软链生效的运行时配置资产；features/p0-p5 是已完成的 spec（log），但 p2/p3/p5 含可复用数据（hook 模式、MCP token 预算）；tools/ 中 git-advanced 最扎实。
- **10-08-dita-as-code**（3）：全部 keep-core。2026-08-05 新建，理论研判 250 行是全库理论密度最高的笔记；MOC 用 `file:///` 绝对路径而非 wikilink，是规范瑕疵。

## 逐文件表

| 相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注 |
|---|---|---|---|---|---|---|
| 10-00-dev-config/AI-ClaudeCode-项目规则管理 | CLAUDE.md 在 Obsidian 维护+软链到项目的机制约定 | keep-core | task | claude-code | medium | dev-config 机制正本 |
| 10-02-pkm/00-MOC-PKM | PKM 主题索引：方法论/配置/维护 | keep-core | moc | pkm | low | 建 vs 维护的区分有价值 |
| 10-02-pkm/PKM-ssp | Simon Späti PKM 体系整理（四支柱） | keep-maybe | concept | pkm | low | 外部文献消化笔记 |
| 10-02-pkm/Obsidian-配置 | 主题/同步方案（git+iCloud）早期随笔 | keep-maybe | task | pkm | medium | 同步决策仍有效 |
| 10-02-pkm/Obsidian-插件清单 | 孤岛/坏链检测插件的配置与审计工作流 | keep-core | task | pkm | medium | 直接支撑审计流程 |
| 10-02-pkm/Obsidian-生态优质资源 | 经 curl 验证的生态仓库/博客清单 | keep-maybe | reference | pkm | high | 治学严谨但链接易腐 |
| 10-02-pkm/PKM-维护流程 | 审计工作流的哲学溯源（MECE/SSoT/Zettelkasten→检查动作） | keep-core | concept | pkm | low | 与 DITA 理论研判同源 |
| 10-03-面试准备/00-MOC-面试准备 | 面试知识体系 MOC | keep-maybe | moc | interview | low | 项目降温 |
| 10-03-面试准备/算法-Grind75题单 | 按解题模式重组的题单 | keep-maybe | reference | dsa | low | 模式分类是自己的加工 |
| 10-03-面试准备/Anki-JavaScript核心 | JS 核心 Q/A 卡 | keep-maybe | reference | javascript | low | 常青 |
| 10-03-面试准备/Anki-TypeScript核心 | TS 核心 Q/A 卡 | keep-maybe | reference | typescript | low | |
| 10-03-面试准备/Anki-React核心 | React Q/A 卡 | keep-maybe | reference | react | medium | |
| 10-03-面试准备/Anki-CSS核心 | CSS Q/A 卡 | keep-maybe | reference | css | low | |
| 10-03-面试准备/Anki-浏览器与性能 | 渲染流水线 Q/A 卡 | keep-maybe | reference | browsers | low | |
| 10-03-面试准备/Anki-前端工程化 | Webpack/Vite Q/A 卡 | keep-maybe | reference | web | medium | 工具演进快 |
| 10-03-面试准备/Anki-算法模式识别 | "题目特征→套路"模式识别卡 | keep-maybe | reference | dsa | low | 卡组中加工度最高 |
| 10-03-面试准备/Anki-NextJS面试题速查 | Next.js 渲染模式速查 | keep-maybe | reference | nextjs | high | 无 frontmatter |
| 10-03-面试准备/Electron面试题 | 30+ 架构/IPC/安全 cheatsheet，深至源码层 | keep-core | reference | electron | medium | 唯一 evergreen 标记 |
| 10-03-面试准备/JSX面试题 | JSX 编译产物与 runtime 细节 | keep-maybe | reference | react | low | 质量好 |
| 10-05-blog/00-MOC-博客项目 | 博客项目索引 | keep-maybe | moc | blog | low | |
| 10-05-blog/01-我的博客备忘录 | 服务器 IP/域名/技术选型 | keep-maybe | reference | blog | medium | 含活的基础设施事实 |
| 10-05-blog/02-博客参考资源 | UI 参考站与教程链接集 | outdated | reference | blog | high | deadline 已过 |
| 10-06-boilerplate/00-MOC-boilerplate | 模板与架构笔记索引 | keep-core | moc | boilerplate | low | |
| 10-06-boilerplate/NestJS-架构 | Clean Architecture+DDD 后端架构说明，受众是 AI agent | keep-core | concept | nestjs | low | 365 行，全目录最重 |
| 10-06-boilerplate/AI-AGENTS写作标准 | AGENTS.md/CLAUDE.md 写作规范（加载时机/三排除测试/RFC 措辞） | keep-core | reference | agent-rules | medium | 自身即样本，可直接投产 |
| 10-06-boilerplate/CodeStyle-分类 | 以 AST 为锚区分 Formatting/Style/Logical | keep-core | concept | code-style | low | 制定规则的前置 |
| 10-06-boilerplate/CodeStyle-约定 | oxlint/oxfmt/dependency-cruiser 工具链与约束清单 | keep-core | reference | code-style | medium | 含 NestJS DI 实战坑 |
| 10-06-boilerplate/Agentic-项目记录 | agentic 仓库目标/技术栈/路线+参考链接 | keep-maybe | log/reference | agent | high | 决策和链接值得拆出 |
| 10-06-boilerplate/Agentic-常见范式 | ReAct/Plan-Execute/自主性谱系 | keep-maybe | concept | agent | medium | 与 tech-tree/agent 可能重复 |
| 10-06-boilerplate/AI-Agent工程路线图-2026 | 路线图文章消化 | keep-maybe | reference | agent | high | 二手内容 |
| 10-06-boilerplate/NestJS-学习参考 | 5 条入门链接 | keep-maybe | reference | nestjs | medium | 太薄 |
| 10-06-boilerplate/TS-启动模版参考 | create-typescript-app 链接+评价 | keep-maybe | reference | typescript | medium | 13 行 |
| 10-06-boilerplate/Vercel-AI-SDK | 适合轻度/原型的边界判断草稿 | log | note | ai | high | 有判断未成文 |
| 10-06-boilerplate/MiniProgram-架构记录 | 多端框架调研心路 | log | note | web | high | 结论两行可留 |
| 10-06-boilerplate/LiteBot-项目记录 | 空壳骨架 | noise | log | ai | - | 仅 3 条链接 |
| 10-06-boilerplate/TODO | 待决问题/组件选型表混杂 | log | note | boilerplate | high | 选型表值得迁出再弃 |
| 10-07-infra-ai/infra-ai-overview | 仓库现状 MOC+选型判断树（CLAUDE.md/Skill/MCP/Hook/Subagent） | keep-core | moc | claude-code | medium | 判断树是最浓缩的决策知识 |
| 10-07-infra-ai/infra-ai-settings | settings.json schema+权限白名单速查 | keep-core | reference | claude-code | high | 直接可复制 |
| 10-07-infra-ai/infra-ai-references | 3 个参考仓库+"皆无架构学规范"研判 | keep-maybe | reference | claude-code | high | 判断比链接有价值 |
| 10-07-infra-ai/infra-ai-roadmap | P0-P5 全部完成的路线图 | log | note | project | - | 归档即可 |
| 10-07-infra-ai/需求 | 多 agent-cli 差异抹平的头脑风暴 | log | note | agent-cli | high | 3 个链接可迁出 |
| 10-07-infra-ai/features/p0-project-init | 新项目初始化 spec | log | task-spec | init | - | 已交付 |
| 10-07-infra-ai/features/p1-rules | 三规范 spec | log | task-spec | rules | - | 已交付 |
| 10-07-infra-ai/features/p2-hooks | 四个 hook 脚本 spec | keep-maybe | task-spec | hooks | high | JSON 格式与防循环模式可复用 |
| 10-07-infra-ai/features/p3-skills | PRP 工作流+agent spec | keep-maybe | task-spec | skills | high | frontmatter 模式可复用 |
| 10-07-infra-ai/features/p4-tools-docs | 文档补全 spec | log | task-spec | tools | - | 已实现 |
| 10-07-infra-ai/features/p5-mcp | MCP 选型 spec：≤5-6 个、每个约 1 万 token | keep-maybe | task-spec | mcp | high | 约束数据值得沉淀 |
| 10-07-infra-ai/rules/constitution | 核心原则：Library/MVP/FP-First | keep-core | reference | agent-rules | low | 运行时配置资产（软链生效） |
| 10-07-infra-ai/rules/architecture | 仓库结构与 SKILL.md frontmatter 规范 | keep-core | reference | agent-rules | medium | 同上 |
| 10-07-infra-ai/rules/frontend | Feature-based 前端架构+命名+禁止行为 | keep-core | reference | agent-rules | medium | 可直接复制到 Next.js 项目 |
| 10-07-infra-ai/rules/nestjs | NestJS 模块化分层规则 | keep-core | reference | agent-rules | medium | NestJS-架构的执行期投影 |
| 10-07-infra-ai/rules/testing | TDD/并置/命名测试规则模板 | keep-maybe | reference | agent-rules | medium | 占位符未填 |
| 10-07-infra-ai/skills/README | 自研+镜像 skills 维护约定（giget/mirrors.json） | keep-core | task | skills | medium | 镜像同步机制是原创工作流 |
| 10-07-infra-ai/skills/ROADMAP | 两条想法草稿 | log | note | skills | - | 8 行 |
| 10-07-infra-ai/tools/git-advanced | 多 agent 并行：worktree vs GitButler 选型 | keep-core | reference | git | medium | 自足、有明确边界 |
| 10-07-infra-ai/tools/github | gh/repomix 备注 | keep-maybe | reference | tools | medium | 太薄 |
| 10-07-infra-ai/tools/skills | ccpi 包管理器速查 | keep-maybe | reference | skills | high | 生态存活性待观察 |
| 10-07-infra-ai/tools/ROADMAP | 分类自问 | noise | note | - | - | 无沉淀 |
| 10-08-dita-as-code/00-MOC-DITA | DITA 项目 MOC：理论→蓝图→骨架→迁移 | keep-core | moc | dita | low | file:// 路径是规范瑕疵 |
| 10-08-dita-as-code/DITA-CMS-理论基础综合研判 | ISO 704/1087/25964+S1000D+DITA 2.0 理论地基 | keep-core | concept | dita | low | 全库理论密度最高 |
| 10-08-dita-as-code/DITA-CMS-信息架构与蓝图 | 1 基座+6 组件域 MECE 划分与物理目录蓝图 | keep-core | concept | dita | medium | 当前研究的执行蓝图 |

## 可直接转化为"项目模板 / agent 配置"的素材

1. `AI-AGENTS写作标准` — 写作规范本身可作 skill 或 reference 投产
2. `CodeStyle-约定` + `CodeStyle-分类` — 可直接生成 code-style 规则文件
3. `NestJS-架构` — 自述"受众：AI agent"，与 rules/nestjs 成对迁入模板
4. `TODO` 中的组件选型表与 @Global() 规则 — 迁入模板后原文件可弃
5. `TS-启动模版参考` — TS lib 模板基线
6. `AI-ClaudeCode-项目规则管理` — 软链机制应写入 init 脚本文档
7. `rules/*` 5 篇 + `infra-ai-settings` — 已是成品配置资产，是上述素材的落地目的地

## 本路 Top 5

1. `NestJS-架构` — 架构决策+AI 受众定位+与 rules 的分工设计
2. `DITA-CMS-理论基础综合研判` — 打通 ISO 标准与自家 PKM 公理，统领当前研究
3. `AI-AGENTS写作标准` — 元层面规则写作规范，杠杆最大
4. `infra-ai-overview` — 选型判断树是高度浓缩的原创决策知识
5. `PKM-维护流程` — "审计=公理的可执行投影"，同时支撑 PKM 与 DITA 两条线

**横向观察**：库内存在清晰的知识主线——PKM 三公理（MECE/SSoT/Atomicity）→ 审计流程 → DITA 理论研判 → infra-ai 规则落地，四组笔记互相引用且判据一致，是最值得保护的资产簇；log 类约占 1/4，可统一归档。
