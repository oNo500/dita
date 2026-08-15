# 附录 04 · inbox + areas 其余 + archive 分拣明细（63 篇）

> 范围：`00-inbox/`（排除 claude-backup）、`20-02-ai-assistant/`、`20-05-rc/`、`20-06-学习与成长/`、`90-archive/`、根目录 README/INDEX。keep-core **22**（inbox 1、20-02 8、20-05 6、20-06 7）。
>
> 勘误：inbox 下 `未命名.md`、`agent哲学与路径依赖.md` 实为**普通文件而非目录**；后者正文为空（仅 frontmatter）。

## 各范围小结

- **00-inbox**（25）：典型草稿混杂区——约 1/3 是高价值未整理原创（hooks 策略、上下文管理、AI 使用心得、生态调研），1/3 是项目工作日志碎片，1/3 是三五行空壳 stub。
- **20-02-ai-assistant**（17）：库中质量最高的操作区，MOC 完整、type 字段齐全；Claude Code 系列 7 篇成体系；OpenClaw/VertexAI 4 篇偏个人环境配置、时效风险高。
- **20-05-rc**（8）：设计意图清晰（个人 runtime configuration，派生到 ~/.claude/rules 的源头），6 篇有实质内容，2 篇纯 TBD 占位。
- **20-06-学习与成长**（10）：最稳定的一组，写作规范四件套 + SuperMemo/Anki 均低时效、自足性好，几乎全部可留。
- **90-archive/2026**（3）：飞连/CorpLink 专题调研，日期与来源齐全、自足，归档位置正确。

## 逐文件表

**00-inbox/**

| 相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注 |
|---|---|---|---|---|---|---|
| README | inbox 使用原则与处理流程 | keep-core | reference | pkm | low | 结构性自述 |
| hooks-渐进增强 | hooks 分层策略（Must/Should/Must-Not + exit code 约定） | keep-maybe | reference | claude-code | high | 接近成稿，建议移 20-02 转正 |
| 上下文管理 | 上下文管理规则（subagent 分派、rules 锚点、MEMORY.md） | keep-maybe | reference | claude-code | high | 成体系草稿，可与 20-02 合并 |
| 使用AI的心的 | 原创心得：不对抗默认行为、规则=行为+范围+失败模式 | keep-maybe | concept | ai-usage | medium | 标题错字；值得抢救 |
| 值得消化的来源 | 2025-2026 Claude Code 生态调研（来源已核实） | keep-maybe | reference | claude-code | high | 密度高但时效衰减快，尽快拆条 |
| 模糊命令和具体命令 | 模糊指令靠 LLM 补全 vs 具体命令确定性的权衡 | keep-maybe | concept | agent | medium | 短但有原创判断 |
| ai项目注意要点 | AI 生成工具必须可观测（OTel/pino）才能验证 | keep-maybe | concept | ai | medium | 一条有效洞察 |
| 终端工具开发 | agent CLI 选型（Charm vs ink）+ 探测→建议→执行→报告流程 | keep-maybe | concept | infra-ai | medium | infra-ai 的设计源头 |
| 项目笔记 | imeta/iuse 职责划分的项目设计 scratch | keep-maybe | log/concept | infra-ai | high | 含真实架构决策，宜提炼进 ADR |
| 添加规则 | skills/mcp/rule 资产的溯源登记与 404 检测流程构想 | keep-maybe | task | infra-ai | medium | 未整理的原创流程设计 |
| 关于Matt Pocock's skills 的简单调研 | 29 个 skills 四组分类与收录结论 | keep-maybe | reference | agent-skills | high | 结论可操作；对话转录未清洗 |
| claude-config | 鼠标/滚轮/fullscreen 配置排错（带版本号） | keep-maybe | troubleshooting | claude-code | high | 绑定 v2.1.x |
| skills-list | 全局已装 skills 清单 | log | reference | agent-skills | high | 机器状态快照，会漂移 |
| 技术文章阅读清单 | 4 条待读链接 | log | task | reading | high | 读完即清 |
| TODO | 个人任务看板 | log | task | personal | high | 部分已过期 |
| 0726todo | RC/rule 分层杂记 + DITA 疑问 | log | log | rules | high | rule 三分层想法可提取 |
| rule | rule 管理工具链接 + DITA 写作计划混杂 | log | log | rules | high | 4 个 repo 链接值得提取 |
| 我常用的agent终端工具 | 在用 agent CLI 清单 | log | reference | agent-cli | high | 三行快照，可并入 rc-stack |
| ruletodo | 一句反思 | noise | log | rules | - | 3 行 |
| 终端概念 | TUI/TTY/PTY 待学名词清单 | noise | task | terminal | - | 纯 stub |
| 未命名 | 无上下文的 DITA 规则集反馈片段 | noise | log | dita | - | 脱离对话不可解 |
| 受控词汇表 | subject scheme ditamap 三行备忘 | noise | log | dita | - | stub |
| 系统全局环境 | uv 替代 pip、pnpm 替代 npm 两行偏好 | noise | reference | env | - | 内容有效但应并入 rc-stack 后删 |
| langchain-start | langsmith studio 一条链接 | noise | log | langchain | - | 2 行 |
| agent哲学与路径依赖 | 正文为空 | noise | - | - | - | 标题有潜力但零内容 |

**20-02-ai-assistant/**

| 相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注 |
|---|---|---|---|---|---|---|
| 00-MOC-ai-coding | AI Coding 区索引，边界声明清晰 | keep-core | moc | ai-coding | medium | 与实际文件对齐 |
| AI-资产与查证分层 | 原创概念：做事链（rule/skill）与查证链（context7→llms.txt→源码）并行 | keep-core | concept | ai-coding | medium | 库内最有原创性的概念笔记之一 |
| AI-Claude-Code速查 | 快捷键/内置命令/输入前缀速查 | keep-core | reference | claude-code | high | 高频使用 |
| AI-Claude-Code快速上手 | 启动参数、权限模式、会话恢复 | keep-core | task | claude-code | high | |
| AI-Claude-Code权限配置 | allow/deny 语法、三层优先级、通配符 | keep-core | reference | claude-code | medium | |
| AI-Claude-Code上下文配置 | .claude/ 目录结构全解 | keep-core | reference | claude-code | medium | 有外部 origin 标注 |
| AI-coding方法论 | spec-kit/BMAD/GSD/Superpowers/OpenSpec 对比与组合 | keep-core | concept | ai-methodology | high | 有自己的判断 |
| AI-Create-Rules | 写 Rules 的方法论：Rule 设边界、Prompt 选路径、RFC 2119 | keep-core | concept | rules | low | 原理层，衰减慢 |
| AI-Claude-Code-Chrome集成 | Chrome 扩展连接与场景 | keep-maybe | task | claude-code | high | beta 功能 |
| AI-Claude-Code安装问题 | 中文路径乱码、fnm/nvm MCP 路径排查 | keep-maybe | troubleshooting | claude-code | medium | |
| AI-Claude产品线 | Chat/Code/Cowork 三产品定位 | keep-maybe | concept | claude | high | 产品线信息易过期 |
| AI-agent-browser | agent-browser CLI 速查 | keep-maybe | reference | agent-browser | high | 单一工具绑定 |
| AI-飞书Bot-MCP集成 | 飞书自建应用 + lark-mcp 接入 | keep-maybe | task | feishu-mcp | high | 一次性配置记录 |
| AI-GCloud-VertexAI初始化 | Vertex AI 开通步骤 | keep-maybe | task | gcloud | medium | |
| AI-OpenClaw配置 | exec 四层权限模型 + bug 规避 | keep-maybe | reference | openclaw | high | workaround 需回查 |
| AI-OpenClaw-VertexAI配置 | 接 Vertex 的 env 配置与踩坑 | keep-maybe | troubleshooting | openclaw | high | 踩坑记录有独立价值 |
| AI-OpenClaw-OpenRouter免费模型 | models scan 参数速查 | keep-maybe | reference | openclaw | high | |

**20-05-rc/**

| 相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注 |
|---|---|---|---|---|---|---|
| 00-MOC-rc | rc 区索引，rc/CLAUDE.md/memory 三者边界定义 | keep-core | moc | rc | low | 边界声明是设计核心 |
| rc-ai | AI 元规则（触发词 CoT 原理 + 可派生规则块） | keep-core | reference | rc | medium | 派生源定位明确 |
| rc-claudemd | CLAUDE.md 模板骨架（十段结构） | keep-core | reference | rc | medium | |
| rc-modes | 场景化模式模板（触发式规则块） | keep-core | reference | rc | medium | |
| rc-rules | 跨项目硬规则（目录语义、测试并置、kebab-case、zod） | keep-core | reference | rc | low | 协作/安全两节 TBD |
| rc-stack | 技术栈偏好及理由（Pure ESM/UnJS/e18e/zod/AI SDK） | keep-core | reference | rc | medium | 可吸收 inbox 的 uv/pnpm |
| rc-habits | 全部 TBD 占位 | noise | - | rc | - | 空壳 |
| rc-style | 全部 TBD 占位 | noise | - | rc | - | 空壳 |

**20-06-学习与成长/**

| 相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注 |
|---|---|---|---|---|---|---|
| 00-MOC-学习与成长 | 写作规范/学习方法/Anki 索引 | keep-core | moc | learning | low | |
| 写作-PKM写作规范 | 主规范：原子化判据 + Diátaxis 四象限 + frontmatter | keep-core | reference | pkm-writing | low | 本次分拣判据与之同构 |
| 写作-LLM友好型文档 | RAG 友好写作：自包含章节、语义分块、元数据 | keep-core | reference | pkm-writing | low | |
| 写作-ADR架构决策记录 | ADR 六章节结构与模板 | keep-core | reference | writing | low | |
| 写作-GitHub-README指南 | README 必备/可选章节 | keep-core | howto | writing | low | |
| 学习-SuperMemo20条原则 | 制卡 20 原则 | keep-core | reference | learning | low | |
| Anki-面试卡片模版 | 六种卡片模版含选题原则 | keep-core | practice | anki | low | 原创整理 |
| 写作-资源清单 | 写作/PKM/RAG 外链汇总 | keep-maybe | reference | writing | medium | 纯链接 |
| Anki-配置速查 | Anki 概念与文档入口 | keep-maybe | reference | anki | low | 薄 |
| GitHub知识发现技巧 | GitHub 搜索关键词与路径 | keep-maybe | reference | github | low | 通用常识 |

**90-archive/2026/**

| 相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注 |
|---|---|---|---|---|---|---|
| 飞连-产品与客户端全方位调研 | 双下载入口版本差异 + macOS 客户端盘点（833 行） | keep-maybe | reference | corplink | high | 完结快照，归档正确 |
| 飞连-客户端升级回滚诊断卸载流程专题 | 四链路梳理，区分已确认/未确认 | keep-maybe | reference | corplink | high | |
| 飞连-Electron-版本落后调研 | 本机取证：Electron 22.3.27 已 EOL | keep-maybe | troubleshooting | corplink | high | 含 plist 证据链 |

## 库设计意图（README/INDEX 还原）

基于 Obsidian 的 Johnny-Decimal + PARA 变体：inbox 捕获、projects 有 deadline、areas 持续关注（超 3 月降级）、resources 纯查阅、archive 按年归档、99-system 放模板与规范；frontmatter 用 jd_id 定位、draft→growing→evergreen 三级成熟度，文件操作强制走 obsidian-cli 保护 wikilink。INDEX 按"当前注意力优先级"排列，当前重点在 AI 工具链、rc 与 infra-ai。

## inbox 抢救清单（按价值排序）

1. `hooks-渐进增强` — 接近成稿，直接移 20-02
2. `上下文管理` — 成体系，与 20-02 上下文配置篇合并
3. `使用AI的心的` — 原创心得，并入 AI-Create-Rules 体系
4. `值得消化的来源` — 高密度生态调研，尽快拆条（时效衰减中）
5. `模糊命令和具体命令`、`ai项目注意要点` — 原创散点
6. `终端工具开发`、`项目笔记`、`添加规则` — 提炼进 infra-ai ADR/文档
7. `关于Matt Pocock's skills 的简单调研` — 清洗后留结论删对话
8. 并入后删：`系统全局环境`（→ rc-stack）、`0726todo` 的 rule 三分层想法（→ rc 或 20-02）
