# 附录 02 · 30-resources 分拣明细（84 篇）

> 范围：`30-resources/`。判定分布：keep-core **60** ｜ keep-maybe 18 ｜ noise 4 ｜ log 2 ｜ outdated 0。

## 子目录小结

- **30-03-开发工具**（10）：两极分化——quietpaper 主题、算法题 Anki、前端工程化是有加工的干货；VSCode/GitHub-Profile/链接清单是碎片或纯书签。
- **30-04-APP**（8）：整体最弱，多为"软件信息卡片"（官网转述）；仅 Docker/Surge/Telegram 有实操信息。
- **30-05-ai**（19）：全库价值最高的目录。claude-code 8 篇是有选型判断的策展索引，prompts 6 篇是大规模消化产物（引文+出处齐全），claude-md 2 篇有原创分类框架。仅 config/CLAUDE.md 是**坏 symlink**。
- **30-06-glossary**（27）：质量与一致性俱佳，见末尾专项评估。
- **30-07-standards**（9）：**全部有加工，不是转述**——ISO 25964 提炼出"回本条件"，DITA/S1000D 综合实践给出"失控教训"，且都回链本库实例；note-conventions 2 篇是原创内部约定（宜考虑挪出 standards）。
- **30-08-readings**（7）：定位明确的摄取层（带 origin frontmatter），4 篇 Medium 全文翻译搬运，价值中低且两篇科普互相重叠。
- **30-09-library**（3）：藏书卡片 stub，仅作 PDF 入口。

## 逐文件表

| 相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注 |
|---|---|---|---|---|---|---|
| 30-03/逆向工具链清理记录-2026-07 | frida/mitmproxy/UnityPy 逆向环境安装与清理全记录 | log | task | security | volatile | 有排查教训段，重建环境可复用 |
| 30-03/日常浏览 | RSS/博客/前端资讯链接清单 | keep-maybe | reference | web | volatile | 纯书签 |
| 30-03/设计资源 | 设计灵感/图标/配色外链清单 | keep-maybe | reference | web | volatile | 纯书签 |
| 30-03/前端工程化面试知识点 | Webpack vs Vite、tree shaking、sideEffects 速查 | keep-core | reference | web | stable | 有自己的组织，2026-08 仍准确 |
| 30-03/00-MOC-开发工具 | 目录索引，带状态标注 | keep-core | moc | - | stable | 与实际文件一致 |
| 30-03/GitHub-Profile配置 | Profile README 搭建要点与徽标工具 | keep-maybe | task | web | stable | 结构散，需整理 |
| 30-03/算法题JS解法-Anki | P1/P2 高频题 JS 解法含思路与复杂度 | keep-core | reference | dsa/lang-ts | stable | 直接可喂 Anki |
| 30-03/Obsidian-quietpaper主题 | 自建主题的三个反共识决策+CSS 撞墙记录 | keep-core | troubleshooting | web/css | stable | 全库最佳原创之一 |
| 30-03/pnpm-本地调试 | pnpm link 三方案速查 | keep-core | task | web | stable | 短而完整 |
| 30-03/VSCode配置 | 几条快捷键 | noise | reference | terminal | stable | 26 行碎片 |
| 30-04/00-MOC-软件工具 | 按平台分类的软件索引 | keep-core | moc | - | stable | 准确 |
| 30-04/Anki | 仅"卡组嵌套 :: 语法"一条 | noise | reference | tooling | stable | 空壳 |
| 30-04/Docker | Docker 下 PostgreSQL 彻底重置步骤 | keep-maybe | task | terminal | stable | 绑定特定项目 |
| 30-04/HiBitUninstaller | Windows 卸载工具功能卡片 | keep-maybe | reference | tooling | stable | 官网转述 |
| 30-04/Microsoft | 一条 Office for Mac 下载链接 | noise | reference | tooling | volatile | 单链接 |
| 30-04/QuantumultX | iOS 代理工具+两个配置仓库 | keep-maybe | reference | security | volatile | 配置仓库时效风险 |
| 30-04/Surge | Surge CLI 路径与官方 agent skill 位置 | keep-maybe | reference | security | volatile | skill/symlink 安装法有独有信息 |
| 30-04/Telegram | SOCKS5 代理配置+接码注册 | keep-maybe | reference | tooling | volatile | 接码平台信息易过期 |
| 30-05/00-MOC-AI | AI 资源库入口，MECE 三分区 | keep-core | moc | ai | stable | 边界声明清晰 |
| 30-05/claude-code/插件市场 | Plugin 定位说明+2 条外链 | keep-maybe | reference | ai | volatile | 太薄，可并入学习索引 |
| 30-05/claude-code/终端工具 | AI 可调用 CLI 清单，含挑工具三判据 | keep-core | reference | ai/terminal | volatile | 有策展判断 |
| 30-05/claude-code/方法论框架 | GSD/superpowers/spec-kit 等框架外链 | keep-maybe | reference | ai | volatile | 纯外链但有分类 |
| 30-05/claude-code/ContextEngineering | CLAUDE.md 机制/写法/样本的官方+社区资源导航 | keep-core | reference | ai | volatile | 注释密度高 |
| 30-05/claude-code/Hooks实现参考 | hooks 注册结构、exit code 语义、实现样例 | keep-core | reference | ai | volatile | 含"确定性 vs 建议性"关键洞察 |
| 30-05/claude-code/MCP服务器 | MCP 推荐 top-5 含选型理由与安装命令 | keep-core | reference | ai | volatile | 含上游归档警告 |
| 30-05/claude-code/Rule与Skill分层 | 无条件/按路径/按意图三层加载的原创判据框架 | keep-core | concept | ai | stable | 原创思考，全库最佳之一 |
| 30-05/claude-code/settings模板 | 注释完整的 settings.json 三层配置模板 | keep-core | reference | ai | volatile | 跟版本走（标注 v2.1.166） |
| 30-05/claude-code/Skills生态 | 社区 skills 仓库逐个评估（含 fork 建议） | keep-core | reference | ai | volatile | "评估："段有真判断 |
| 30-05/claude-code/学习索引 | CC 学习路径外链总导航 | keep-core | reference | ai | volatile | 子文件的枢纽 |
| 30-05/claude-code/config/CLAUDE.md | 坏 symlink → ~/.claude/CLAUDE.md | noise | - | ai | - | **目标不存在，删除或修复** |
| 30-05/claude-md/调研样本 | 真实 CLAUDE.md 样本五原型分类+可抄片段 | keep-core | concept | ai | stable | 原创分类学 |
| 30-05/claude-md/规范与实践 | 按作用层的写作规范，含指令预算 | keep-core | concept | ai | stable | frontmatter 仍 draft，内容已成熟 |
| 30-05/prompts/原子技巧 | 24 条 prompt 技巧，每条带何时用/副作用/评测信号/出处 | keep-core | reference | ai | stable | 消化质量高 |
| 30-05/prompts/前端美学 | 可直接嵌入的 anti-slop 前端审美 prompt | keep-maybe | reference | ai | stable | 单段粘贴但即取即用 |
| 30-05/prompts/样本对比 | 36 份真实 system prompt 的风格分类与共性差异 | keep-core | concept | ai | stable | 断言附原文引块 |
| 30-05/prompts/评估方法 | eval 方法论消化（Hamel/Shreya/官方），773 行 | keep-core | concept | ai | stable | 引文扎实 |
| 30-05/prompts/资源索引 | 129 条外链，HEAD 校验+死链修正 | keep-core | reference | ai | volatile | 有维护纪律的链接库 |
| 30-05/prompts/任务模式 | 8 类任务的 prompt 骨架+技巧组合+失败模式 | keep-core | concept | ai | stable | |
| 30-06/00-MOC-Glossary | 术语库入口+使用规则（首现必链、概念优先） | keep-core | moc | pkm | stable | 规则本身有价值 |
| 30-06/glossary-aggregate | DDD 聚合定义+弃用写法 | keep-core | glossary | arch/ddd | stable | 缺 type 字段 |
| 30-06/glossary-atomicity | 笔记原子性公理与判据 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-bases | Obsidian .base 数据库视图 vs dataview | keep-core | glossary | pkm | volatile | 跟 Obsidian 版本 |
| 30-06/glossary-block-id | 块级锚点 ^id 语法与场景 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-callout | 标注块语法与内置类型 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-canvas | JSON Canvas 白板格式与 MOC 对比 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-context-engineering | 声明式指令注入的定义+与 prompt/config 消歧 | keep-core | glossary | ai | stable | 弃用写法清单好 |
| 30-06/glossary-context | DDD bounded context，完整 ISO-704 式条目 | keep-core | glossary | arch/ddd | stable | 模板标杆 |
| 30-06/glossary-domain-event | 领域事件定义+与 command/message 消歧 | keep-core | glossary | arch/ddd | stable | 模板标杆 |
| 30-06/glossary-embed | ![[]] 嵌入语法与 wikilink 区别 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-frontmatter | YAML 元数据块必填字段与约束 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-jd-id | 本库唯一 ID 格式与取值纪律 | keep-core | glossary | pkm | stable | 库内约定 |
| 30-06/glossary-jd | Johnny Decimal 与本库映射 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-kebab-case | 命名风格与本库适用范围 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-lifecycle | 5 枚举成熟度（stub→evergreen）与约束 | keep-core | glossary | pkm | stable | 分拣本身依赖它 |
| 30-06/glossary-mece | MECE 原则+检验方法+20% 兜底红线 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-moc | MOC 命名/结构/150 行上限 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-para | PARA 四象限与 JD 叠加决策链 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-prefix | 文件名首段大小写与砍 prefix 规则 | keep-core | glossary | pkm | stable | 库内约定 |
| 30-06/glossary-properties | Obsidian Properties 与 frontmatter 关系及本库立场 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-ssot | 单一事实源两约束与检验 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-tag | 两种 tag 语法与本库规则（不用嵌套） | keep-core | glossary | pkm | stable | |
| 30-06/glossary-theme | 文件名第二段规则 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-type | 6 枚举笔记类型与易混边界 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-wikilink | 内链语法+别名强制+反链立场 | keep-core | glossary | pkm | stable | |
| 30-06/glossary-zettelkasten | 卡片盒两公理与 PARA/JD 分工 | keep-core | glossary | pkm | stable | |
| 30-07/00-MOC-Standards | de jure/de facto 双轨索引+9 个 Agent 协议注解 | keep-core | moc | docs/ai | volatile | Agent 协议区更新压力大 |
| 30-07/note-conventions/资源页结构 | 本库资源页 7 段骨架+判定边界 | keep-core | reference | pkm | stable | 原创内部约定 |
| 30-07/note-conventions/知识库审计 | 公理→检查动作的审计清单+勘探命令 | keep-core | task | pkm | stable | 原创，直接可执行 |
| 30-07/tech-docs/DITA | topic/map/profiling/conref 四件套+粒度失控教训 | keep-core | concept | docs | stable | 综合实践下判断，有加工 |
| 30-07/tech-docs/S1000D | data module/DMRL/适用性三表，与 DITA 差异视角 | keep-core | concept | docs | stable | "计划 vs 实际"对账语义是提炼 |
| 30-07/terminology/ISO-704 | 概念-术语-定义三角+属种差定义写法 | keep-core | concept | terminology | stable | 用本库 aggregate 举例 |
| 30-07/terminology/ISO-1087 | 术语学元词汇 | keep-core | reference | terminology | stable | 为 704/TBX 提供地基 |
| 30-07/terminology/ISO-25964 | 受控词表最小核心+分面分类+pre/post-coordination | keep-core | concept | terminology | stable | "回本条件""3-7 facet"是评估结论 |
| 30-07/terminology/TBX | conceptEntry 三层结构+方言选择建议 | keep-core | reference | terminology | stable | 落到"个人场景 TBX-Min 够用" |
| 30-08/00-MOC-Readings | 摄取层索引，边界声明清晰 | keep-core | moc | ai | stable | |
| 30-08/AI-博主-ArminRonacher | 博主订阅卡片 | keep-maybe | reference | ai | stable | stub |
| 30-08/AI-论文待阅读列表 | 待读论文追踪，含摘要与读因 | log | reference | ai | volatile | 追踪型文档 |
| 30-08/AI-2026年AI增强开发者工作流设计 | Medium 长文全文翻译（400 行） | keep-maybe | concept | ai | volatile | 搬运，无个人批注层 |
| 30-08/AI-20个最重要的AI概念 | 科普文翻译 | keep-maybe | concept | ai | stable | 与下一篇重叠 |
| 30-08/AI-开发者必须了解的8个AI概念 | 同类科普翻译 | keep-maybe | concept | ai | stable | 二存一即可 |
| 30-08/AI-生产级AI系统的六大核心概念 | Towards AI 长文翻译 | keep-maybe | concept | ai | stable | 搬运但质量尚可 |
| 30-09/00-MOC-Library | 藏书索引（2 本） | keep-core | moc | dsa | stable | |
| 30-09/算法图解 | 书籍卡片+PDF 嵌入+代码仓库 | keep-maybe | reference | dsa | stable | stub |
| 30-09/Hello-算法 | 书籍卡片+PDF 嵌入 | keep-maybe | reference | dsa | stable | stub，阅读笔记段为空 |

## glossary 专项评估

结构一致性好——每条首行都是属+种差式内涵定义，后接特征/易混点/相关小节，frontmatter 规范统一（仅 `glossary-aggregate` 缺 `type: glossary` 字段）。存在两级模板并存：DDD 条目用完整 ISO-704 式 frontmatter（preferred/admitted/deprecated_terms/concept_field/source），其余 24 条用简化版——这是分层而非不一致，但迁移为术语库正本时建议统一到完整模板。**值得作为正本迁移**：严格"一概念一条目"、弃用词内嵌、与 30-07 的 ISO 704/TBX 元标准互相印证，是全库自洽度最高的子系统；唯一系统性欠账是 26 条全部停在 `draft` 生命周期，与实际成熟度不符。

## 本路 Top 5

1. `30-03/Obsidian-quietpaper主题` — 反共识决策+撞墙记录，纯原创经验密度最高
2. `30-05/claude-code/Rule与Skill分层` — 三层加载判据框架，超出社区通行说法
3. `30-05/prompts/评估方法` — 30 份资料消化、断言带原文引块的 eval 方法论
4. `30-05/claude-md/调研样本` — 真实样本五原型分类学，判断是自己的
5. `30-07/terminology/ISO-25964` — 把 ISO 标准翻译成"什么规模才回本"的工程决策

另需处理：`30-05-ai/claude-code/config/CLAUDE.md` 为坏 symlink（指向不存在的 `~/.claude/CLAUDE.md`），删除或改为快照拷贝。
