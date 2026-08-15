# 附录 01 · tech-tree 分拣明细（214 篇）

> 范围：`20-areas/20-04-tech-tree/`。判定分布：keep-core **161** ｜ keep-maybe 41 ｜ noise 8 ｜ log 4 ｜ outdated 0。

## 子目录小结

- **root**（1）：全库总索引 MOC，维护勤、有原创分组方法论，但依赖 wiki 链接不自足。
- **agent**（15）：质量极高，14 篇内容笔记全部有原创组织与实战判断；短板是 LLM 生态易变，半数 volatile。
- **api-design**（3）：最薄弱，仅 API-设计参考一篇有真实价值。
- **architecture**（5）：中上，设计原则与架构模式总览是长青原创提炼，微前端含一手坑点。
- **browsers**（11）：质量高且题材稳定，渲染阻塞、事件循环两篇是全库写作水准最高的原创长文。
- **code-style**（6）：小而精，两篇原创方法论是全库思辨密度最高的内容。
- **css**（8）：中等偏上，两篇速查标 stub 实则完整；视觉系统/shadcn 绑定 Tailwind v4，易变。
- **databases**（12）：整体扎实，PG 主线四篇久经不衰；ORM 选型需更新。
- **devops**（6）：参差，Node 版本管理一篇是真实踩坑沉淀的亮点。
- **dsa**（7）：围绕"复杂度"有完整小体系，另有两篇碎片。
- **electron**（21）：全库系统性最强的一组，17 篇编号长文锚定 Electron 30+、自绘架构图、源码定位。
- **git**（4）：整体偏弱，仅模板克隆一篇小而完整。
- **go**（3）：小而全优，2026-07 新鲜调研 + 换栈视角原创。
- **javascript**（12）：两极分化，概念篇规格锚定质量高；三个速查偏 MDN 复述。
- **markdown**（4）：三篇速查界定清晰（基础/GitHub/Obsidian 分工明确）。
- **nestjs**（4）：质量高，架构实践有原创决策框架。
- **networking**（9）：扎实的协议基础，稳定无过期风险，全部可直接入库。
- **nextjs**（6）：概念清晰但受框架迭代影响最大（生态已到 Next 16，笔记停在 15）。
- **nodejs**（7）：两极分化，事件循环/中间件模型是高质量原创长文。
- **python**（7）：整体质量高，仅 FastAPI 篇偏官方复述。
- **react**（19）：全库最成熟的一组，深度笔记统一锚定 React v19.0.0 源码行号，可整体直入正式语料。
- **testing**（5）：体量大但原创度低，三篇长文需提炼压缩。
- **tooling**（4）：质量高，新型工具链-Rust 有全景梳理与原创判断。
- **typescript**（14）：教科书式概念笔记为主，自足性好；工程配置三篇互有重叠。
- **web-security**（21）：全库质量最高的目录，除 MOC 外全为原创 concept 长文，统一引 OWASP/NIST/RFC、带 CVE 与真实案例，全部 keep-core。

## 逐文件表

`相对路径 | 一句话内容 | verdict | type | subject | volatility | 备注`

```
00-MOC-Tech-Tree.md | 全库技术图谱总索引,按学习阶段分组附摘要 | keep-maybe | moc | root | stable | 依赖 wiki 链接,随库迁移即可
agent/00-MOC-Agent.md | Agent 知识体系索引,六段式组织+跨目录引用 | keep-maybe | moc | agent | stable | MOC 中质量最高
agent/Agent-架构模式.md | ReAct/Planning/Multi-Agent/记忆/错误恢复,附循环伪代码 | keep-core | concept | agent | stable |
agent/Agent-优质资源.md | Agent/LLM 生态精选,有入选门槛+逐条点评 | keep-core | reference | agent | volatile | 链接需定期复核
agent/Agent-推理模型.md | o-series/Extended Thinking/R1/QwQ 对比+计费差异 | keep-core | concept | agent | volatile | 型号清单迭代快
agent/Agent-结构化输出.md | Function Calling/JSON Mode/Structured Outputs 各家对照 | keep-core | concept | agent | volatile | 协议形态稳定,模型 ID 会老化
agent/Agent-成本与缓存.md | 按省钱手段优先级组织的计费与 Prompt Caching 实战 | keep-core | concept | agent | volatile | 价格数字易变,组织思路原创
agent/Agent-代码执行沙箱.md | 沙箱威胁模型与隔离技术分层(容器/gVisor/Firecracker) | keep-core | concept | agent | stable | 厂商段落需偶尔刷新
agent/Agent-Eval与可观测性.md | 四类 eval 方法论(含 LLM-as-judge 偏差清单)+工具链 | keep-core | concept | agent | volatile | 方法论稳定,工具生态快变
agent/Agent-LLM调用与SDK.md | 海内外厂商 API 形态/请求体差异/抽象层选型全景 | keep-core | reference | agent | volatile | 全库最易过期,实战发现有原创价值
agent/Agent-MCP协议.md | MCP 架构、三大传输层、原语速查+配置示例 | keep-core | reference | agent | volatile | 需核对现行 spec(SSE→Streamable HTTP)
agent/Agent-Prompt工程.md | 消息结构/Few-shot/CoT 核心技巧+对比示例 | keep-core | concept | agent | stable | 偏基础但自足完整
agent/Agent-Prompt注入与越狱.md | OWASP LLM Top10 v2.0 逐项+与 SQLi 本质对比+防御组合 | keep-core | concept | agent | stable | 与 Sec-* 互引,加工深
agent/Agent-RAG系统.md | RAG 全流程:Embedding/Chunking/检索/Reranking | keep-core | concept | agent | stable | embedding 型号名微过时
agent/Agent-Tokenization与Context.md | tokenizer 差异、中文 token 比例、Lost in the Middle | keep-core | concept | agent | volatile | context 数字随版本变
agent/Agent-Workflow与Agent区分.md | Anthropic 权威源提炼:5 种 workflow 模式+何时上 agent | keep-core | concept | agent | stable |
api-design/00-MOC-API-Design.md | API 设计子域索引,仅 14 行且漏收设计参考篇 | keep-maybe | moc | api-design | stable | 残缺需补链
api-design/API-优质资源.md | 5 条规范/工具链接,点评仅一词级 | keep-maybe | reference | api-design | volatile | 近空壳,宜并入设计参考后删除
api-design/API-设计参考.md | RFC/Google AIP 索引+版本/错误/分页策略对比表 | keep-core | reference | api-design | stable | 标 draft 但自足,分页对比有原创加工
architecture/00-MOC-Architecture.md | 架构子域索引,三段式带摘要 | keep-maybe | moc | architecture | stable |
architecture/Arch-微前端.md | 微前端动机、Qiankun 原理与已知坑、Module Federation | keep-core | concept | architecture | volatile | 一手坑点有价值,Qiankun 渐冷需备注
architecture/Arch-优质资源.md | 架构方法论/模板仓库链接,少量鉴别性点评 | keep-maybe | reference | architecture | volatile | 可并入 MOC
architecture/Arch-设计原则.md | SOLID/DRY/KISS 逐条 TS 正反例+前端场景映射 | keep-core | concept | architecture | stable | 长青,示例原创
architecture/Arch-架构模式总览.md | 架构演进时间线+分层/六边形/VSA/DDD/CQRS 对比 | keep-core | concept | architecture | stable | 高密度原创提炼
browsers/00-MOC-Browsers.md | 浏览器子域索引,按渲染/调度/安全/实践分组 | keep-maybe | moc | browsers | stable |
browsers/Browser-渲染阻塞.md | render-blocking 与 parser-blocking 正交解析+Preload Scanner | keep-core | concept | browsers | stable | 全库写作质量最高之一
browsers/Browser-事件循环.md | HTML Spec 口径 task/microtask/rAF 调度+microtask 设计史 | keep-core | concept | browsers | stable | 标 draft 但规范级准确
browsers/Browser-性能优化.md | 图片格式/响应式/懒加载与 HTTP 缓存实操 | keep-core | task | browsers | stable | AVIF 兼容性需微更新
browsers/Browser-优质资源.md | 通用性能工具链接罗列,无点评,留 TODO | noise | reference | browsers | volatile | 无筛选价值
browsers/Browser-性能指标.md | Core Web Vitals 阈值速查+指标关系+测量代码 | keep-core | reference | browsers | stable | INP 现行标准,未过期
browsers/Browser-渲染流水线.md | Chrome 多进程架构、渲染流水线、重排重绘合成代价表 | keep-core | concept | browsers | stable |
browsers/Browser-运行时差异.md | PC/移动/WebView/Electron/Node 宿主差异速查,逐条附出处 | keep-core | reference | browsers | volatile | 兼容性事实需周期核对
browsers/Browser-定时器精度.md | 定时器误差原因→按时长选方案→rAF 校准与对时代码 | keep-core | troubleshooting | browsers | stable | 现象-原因-处置结构完整
browsers/Browser-关键渲染路径.md | CRP 流程与优化:阻塞资源处理+Resource Hints 四件套 | keep-core | concept | browsers | stable | growing,互链成体系
browsers/Browser-同源策略与安全.md | 同源判定、CORS 简单/预检请求、跨域 Cookie 规则 | keep-core | concept | browsers | stable | growing,自足完整
code-style/00-MOC-CodeStyle.md | 工程约定协议层索引,含 Tooling 分界启发式 | keep-maybe | moc | code-style | stable | 多条待补充
code-style/CodeStyle-配置组合模式.md | 原子配置组合五模式与 ESLint/K8s/Nix 行业教训 | keep-core | concept | code-style | stable | 全库原创度最高之一
code-style/CodeStyle-偏好与原则界定.md | 偏好 vs 原则判据(反转测试/后果测试)与灰区裁法 | keep-core | concept | code-style | stable | 末段引私有 rc-rules,入库可裁剪
code-style/ESM-纯规范.md | 纯 ESM 迁移:package.json/exports/扩展名/__dirname 替代 | keep-core | task | code-style | stable |
code-style/SemVer-语义化版本.md | SemVer 格式、^/~ 范围、预发布与版本管理工具 | keep-core | reference | code-style | stable | 原创不多但自足完整
code-style/todo.md.md | 一条待办碎片(Tailwind class 排序插件适配) | log | - | code-style | volatile | 可删
css/00-MOC-CSS.md | 样式方案索引:基础/速查/工程实践三组 | keep-maybe | moc | css | stable |
css/CSS-单位.md | 按相对锚点分组的单位速查,含 dvh/svh 与高频陷阱 | keep-core | reference | css | stable | 标 stub 但完整
css/CSS-优质资源.md | Tailwind/设计系统/组件库链接,点评极简 | keep-maybe | reference | css | volatile | 接近纯罗列
css/CSS-视觉系统.md | Tailwind+shadcn token 三处联动体系与动画选型 | keep-core | reference | css | volatile | 绑定 Tailwind v4 当前形态
css/CSS-现代特性.md | :has/:is/:where 等现代伪类与兼容性 | keep-core | concept | css | stable | 兼容性数据偶尔刷新
css/CSS-选择器优先级.md | (a,b,c) 权重计算与 :where/@layer 现代规则速查 | keep-core | reference | css | stable | 标 stub 但完整
css/CSS-NextJS全局样式.md | Next.js global.css 初始化模式:重置/变量/排版骨架 | keep-maybe | task | css | stable | 个人模板性质,通用性一般
css/CSS-shadcn.md | shadcn 源码交付哲学、组件分层与 v4 配置、第三方 registry | keep-core | concept | css | volatile | 生态清单易过时
databases/00-MOC-Databases.md | 数据库知识树:PG 主线+Redis+ORM+本地优先 | keep-maybe | moc | databases | stable |
databases/DB-优质资源.md | 带署名点评与质量分级的 DB 博客/文档索引,链接经验证 | keep-core | reference | databases | volatile | 高于一般收藏页
databases/DB-索引怎么加.md | PG 加索引手册:类型/列序/覆盖/部分索引,每条配示例 | keep-core | task | databases | stable |
databases/DB-本地优先资源.md | CRDT/同步框架/SQLite 生态目录,一句话定位 | keep-maybe | reference | databases | volatile | local-first 迭代极快
databases/DB-索引与查询优化.md | 索引三原则、复合列序、EXPLAIN ANALYZE 排查信号 | keep-core | concept | databases | stable |
databases/DB-事务与隔离级别.md | ACID/四级隔离/MVCC/锁,标注 PG 与标准差异 | keep-core | concept | databases | stable |
databases/DB-Drizzle.md | Drizzle 心智模型:双查询 API、schema/relations/迁移 | keep-core | concept | databases | volatile | API 快速演进中
databases/DB-ORM选型.md | Prisma/TypeORM/Drizzle 对比与游标分页 | keep-maybe | concept | databases | volatile | Prisma 缺点描述已偏旧需更新
databases/DB-PostgreSQL核心.md | PG 差异锚点:索引类型/EXPLAIN/JSONB 必踩点 | keep-core | concept | databases | stable |
databases/DB-Python-ORM选型.md | SQLAlchemy 2.0/SQLModel/Django 选型,含 TS 侧对照 | keep-core | concept | databases | volatile | TS↔Python 对照表是独特原创点
databases/DB-Redis数据模型.md | 五大结构场景映射、淘汰策略、缓存三大问题 | keep-core | concept | databases | stable |
databases/DB-SQLite与本地优先.md | 嵌入式/本地优先/CRDT 边界与 PRAGMA 实务 | keep-core | concept | databases | volatile | PRAGMA 稳定,生态清单易过时
devops/00-MOC-DevOps.md | DevOps 索引:本地环境/部署/可观测性 | keep-maybe | moc | devops | stable |
devops/DevOps-Linux.md | 题为 Linux 命令参考实际只讲 ln -s 基础 | noise | reference | devops | stable | 题大文窄,可再生
devops/DevOps-Node版本管理.md | brew/fnm 管理 LTS + zsh 环境变量三层约定,含真实踩坑 | keep-core | task | devops | stable | 全目录最佳
devops/DevOps-SSL配置.md | Certbot+Nginx 申请证书步骤粘贴 | log | task | devops | stable | 格式损坏含具体域名,一次性记录
devops/DevOps-Vercel部署.md | GitHub Actions 部署 Vercel 的三个环境变量获取 | keep-maybe | task | devops | volatile | Actions 集成一节空壳待补
devops/Obs-优质资源.md | 可观测性 4 个工具链接各一句话 | noise | reference | devops | volatile | 常识性链接,无筛选价值
dsa/00-MOC-DSA.md | 算法子域索引,标明与面试 MOC 分工 | keep-maybe | moc | dsa | stable |
dsa/book-算法图解-note.md | 《算法图解》全书 MECE 知识树四层提炼 | keep-core | concept | dsa | stable | 体系化读书笔记
dsa/DSA-算法通俗解释.md | 三条算法生活类比(查字典/扑克/找零) | noise | concept | dsa | stable | 14 行碎片,可并入他篇
dsa/DSA-复杂度速查表.md | 复杂度量级排序+代码模式→量级条件反射表 | keep-core | reference | dsa | stable | 作者标 evergreen,依赖库内图片附件
dsa/DSA-算法刷题资源.md | 题单/入门书/刷题博客精选,逐条点评 | keep-core | reference | dsa | stable | 结尾典型题一节是 stub
dsa/DSA-复杂度学习路径.md | 复杂度学习资源选型与三步计划 | log | concept | dsa | stable | 一次性学习规划
dsa/DSA-大O表示法.md | 大 O 定义与三条核心性质+量级排序 | keep-maybe | concept | dsa | stable | 准确但薄,宜并入速查表
electron/00-Electron-架构与进程模型.md | Chromium+Node 双引擎进程模型全景与事件循环整合 | keep-core | concept | electron | stable |
electron/00-MOC-Electron.md | Electron 知识树索引,17 篇笔记入口 | keep-maybe | moc | electron | stable | 依赖 wiki 链接
electron/01-Electron-IPC通信机制.md | 五种 IPC API 语义差异、mojo 底层、remote 移除始末 | keep-core | concept | electron | stable |
electron/02-Electron-Preload与上下文隔离.md | V8 context 隔离原理、contextBridge 穿越机制,含源码定位 | keep-core | concept | electron | stable |
electron/03-Electron-安全基线与攻击面.md | XSS→IPC→RCE 攻击链分段拆解与每段挡板开关、Fuses | keep-core | concept | electron | stable | 系列枢纽篇
electron/04-Electron-本地数据与凭据存储.md | 敏感度四级分档与 safeStorage/Keychain/DPAPI 策略 | keep-core | concept | electron | stable |
electron/05-Electron-远程内容与协议安全.md | 远程 URL/自定义协议/deep link 三大攻击面与处置 | keep-core | concept | electron | stable |
electron/06-Electron-代码签名与公证.md | macOS 公证链路与 Windows EV/Azure 签名实务 | keep-core | task | electron | volatile | 签名政策年年变,需定期核对
electron/07-Electron-启动性能.md | 冷启动耗时拆解、V8 snapshot、窗口预热与测量 | keep-core | concept | electron | stable |
electron/08-Electron-内存与资源.md | 多进程内存基线、V8 堆 vs 进程内存、泄漏定位 | keep-core | concept | electron | stable |
electron/09-Electron-大数据量渲染.md | 10w 条列表:虚拟列表选型+Worker/utility 进程分工 | keep-core | concept | electron | stable | 库选型段略易变
electron/10-Electron-实时数据流.md | WebSocket hub 放 utility 进程的架构与断线/心跳设计 | keep-core | concept | electron | stable |
electron/11-Electron-窗口与生命周期.md | app/BrowserWindow/webContents 三层事件时序与平台差异 | keep-core | concept | electron | stable |
electron/12-Electron-原生能力集成.md | 通知/托盘/快捷键/协议注册的双平台差异清单 | keep-core | reference | electron | stable |
electron/13-Electron-打包与构建.md | 产物结构、asar、builder vs forge、native module 与 CI | keep-core | concept | electron | volatile | 打包工具链演进快
electron/14-Electron-自动更新.md | Squirrel 机制对比、增量/灰度/强制更新策略 | keep-core | concept | electron | volatile |
electron/15-Electron-崩溃日志与调试.md | 崩溃源分类矩阵与 crashReporter/minidump 收集链路 | keep-core | troubleshooting | electron | stable |
electron/16-Electron-vs-Tauri-WebView选型.md | 三方案本质差异与安全平台场景选型论证 | keep-core | concept | electron | volatile | 体积/内存数据随版本失准
electron/Electron-术语速查.md | ~80 个术语属+种差一句话定义并跳转详篇 | keep-core | reference | electron | stable | evergreen;个别条目混入 React 概念
electron/Electron-API速查.md | main/renderer/preload/utility 各 context API 能力矩阵 | keep-core | reference | electron | stable | evergreen
electron/Electron参考资源.md | 按"何时查哪个"组织的官方文档/源码导航/工具索引 | keep-core | reference | electron | volatile | 高于一般链接收藏
git/00-MOC-Git.md | Git 子域索引,三条链接 | keep-maybe | moc | git | stable |
git/Git-仓库模板克隆.md | degit/giget 无历史克隆模板仓库用法 | keep-core | task | git | stable | 标 stub 但小而完整
git/Git-GitHub协作.md | README 模板/协议/gitignore/PAT 配置杂烩 | keep-maybe | task | git | stable | PAT+VPN 限流有原创语境,余需拆分
git/Git-PR规范.md | 题为 PR 规范实为基础 git 命令罗列 | noise | task | git | stable | 含两处拼写错误(git rest),题文不符
go/00-MOC-Go.md | Go 换栈学习地图,含 2026-07 调研锚点 | keep-core | moc | go | volatile | 罕见的内容型 MOC
go/Go-优质资源.md | 换栈者视角 Go 资源精选,逐条署名点评,URL 已验证 | keep-core | reference | go | volatile | 调研新鲜,细节会漂移
go/Go-JS-TS差异锚点.md | JS/TS→Go 五个需重建的心智模型(并发/错误/接口/指针/零值) | keep-core | concept | go | stable | 原创概念提炼典范
javascript/00-MOC-JavaScript.md | JS 域索引,含跨目录指针 | keep-core | moc | javascript | stable |
javascript/JS-运行时.md | 9 个运行时按事件循环/标准库/模块/IO 四维对比 | keep-core | concept | javascript | volatile | Deno/Bun 演进快
javascript/JS-原型链.md | 从语言史与 Self 谱系讲 [[Prototype]]/new/class,规格锚定 | keep-core | concept | javascript | stable |
javascript/JS-相等比较.md | ECMA-262 四套相等算法与 NaN/±0 边界,== 决策树 | keep-core | concept | javascript | stable |
javascript/JS-数组方法.md | 数组方法按可变性分类+易混淆对照 | keep-maybe | reference | javascript | stable | MDN 复述为主,可合并
javascript/JS-优质资源.md | JS 资源清单,分类清晰点评简短 | keep-maybe | reference | javascript | volatile | 部分链接已旧
javascript/JS-循环遍历.md | 6 种循环选用与三个易错点 | keep-core | concept | javascript | stable | 短小但自足
javascript/JS-对象方法.md | Object 键值遍历/合并/冻结基础速查 | keep-maybe | reference | javascript | stable | hasOwnProperty 写法偏旧
javascript/JS-内置数据结构.md | Object vs Map、Array vs Set、WeakMap 场景速查 | keep-maybe | reference | javascript | stable | 基础复述加工薄
javascript/JS-现代工具链清单.md | JS 全生命周期工具一览(含 Rolldown/oxlint 新生代) | keep-core | reference | javascript | volatile | 需半年级刷新
javascript/JS-元编程的框架应用.md | Pull vs Push 流派、Vue2→3 Proxy 决策的原创长文 | keep-core | concept | javascript | stable |
javascript/JS-ES版本特性.md | ES2015-2024 关键特性一览表 | keep-maybe | reference | javascript | volatile | 缺 ES2025 条目,补一行即可
markdown/00-MOC-Markdown.md | 三篇速查的索引 | keep-maybe | moc | markdown | stable |
markdown/Markdown-语法速查.md | 标准 Markdown 基础语法速查 | keep-maybe | reference | markdown | stable | 完全通用可再生
markdown/Markdown-GitHub扩展语法.md | GitHub 私有扩展(Alerts/折叠/任务列表)速查 | keep-core | reference | markdown | stable | 界定清晰
markdown/Markdown-Obsidian扩展语法.md | Obsidian 私有语法(wikilink/embed/callout)速查 | keep-core | reference | markdown | stable | 对本库高频实用
nestjs/00-MOC-NestJS.md | NestJS 子域索引,三条带定位 | keep-maybe | moc | nestjs | stable |
nestjs/NestJS-优质资源.md | 生态精选,设 18 个月活跃度入选门槛,逐条点评 | keep-core | reference | nestjs | volatile | 门槛化筛选是原创加工
nestjs/NestJS-架构实践.md | VSA+CQRS+DIP 默认骨架与何时升 DDD 的决策流程 | keep-core | concept | nestjs | stable | 原创架构决策框架
nestjs/NestJS-Swagger集成.md | Swagger CLI Plugin/DocumentBuilder 配置速查 | keep-core | reference | nestjs | stable |
networking/00-MOC-Networking.md | 网络专题索引(传输/HTTP/实时/DNS) | keep-core | moc | networking | stable |
networking/Network-DNS.md | 解析流程/记录类型/TTL/CDN | keep-core | concept | networking | stable |
networking/Network-HTTP头.md | 五组高频 HTTP 头速查 | keep-core | reference | networking | stable | 标 stub 但可用
networking/Network-HTTP状态码.md | 分段状态码速查+高频陷阱 | keep-core | reference | networking | stable | 标 stub 但完整
networking/Network-HTTP协议演进.md | HTTP/1.1→2→3 与强/协商缓存 | keep-core | concept | networking | stable |
networking/Network-SSE.md | SSE 协议格式/EventSource/AI 流式 | keep-core | concept | networking | stable |
networking/Network-TCP与UDP.md | 三次握手/四次挥手/拥塞控制 | keep-core | concept | networking | stable |
networking/Network-TLS握手.md | TLS 1.2 vs 1.3 握手与 0-RTT | keep-core | concept | networking | stable |
networking/Network-WebSocket.md | 握手升级/全双工/与 SSE 对比 | keep-core | concept | networking | stable |
nextjs/00-MOC-Nextjs.md | Next.js 专题索引 | keep-core | moc | nextjs | stable |
nextjs/Nextjs-渲染模式.md | CSR/SSR/SSG/ISR/PPR 对比与数据获取 | keep-maybe | concept | nextjs | volatile | PPR 仍写"15 实验性",需更新到 16
nextjs/Nextjs-优质资源.md | App Router/生态/性能资源索引,含入选门槛 | keep-core | reference | nextjs | volatile | 已对齐 Next 16
nextjs/Nextjs-App-Router设计.md | 文件系统路由/特殊文件/动态路由 | keep-core | concept | nextjs | volatile | 基础较稳
nextjs/Nextjs-Edge-Middleware.md | middleware.ts 拦截/重定向/重写 | keep-core | concept | nextjs | volatile | Next 16 已引入 proxy.ts,建议补注
nextjs/Nextjs-Server-Actions.md | 'use server' 定义方式与数据变更 | keep-core | concept | nextjs | volatile |
nodejs/00-MOC-Nodejs.md | Node 运行时专题索引 | keep-core | moc | nodejs | stable |
nodejs/Nodejs-优质资源.md | Node/TS 全栈资源精选,含立场化点评+核对日期 | keep-core | reference | nodejs | volatile |
nodejs/Nodejs-事件循环.md | libuv 6 阶段/nextTick/setImmediate,Ryan Dahl 溯源 | keep-core | concept | nodejs | stable | 内容成熟但 tag 仍 draft
nodejs/Nodejs-依赖管理.md | pnpm overrides/syncpack/taze 链接速查 | keep-maybe | reference | nodejs | volatile | 偏薄,可并入资源篇
nodejs/Nodejs-中间件模型.md | Express→Koa→Fastify→NestJS 洋葱模型演化 | keep-core | concept | nodejs | stable |
nodejs/Nodejs-模块加载机制.md | 运行时分类/CJS-ESM 检测/fnm 版本管理 | keep-core | concept | nodejs | volatile | 工具链推荐随生态变
nodejs/Nodejs-TODO.md | "未命名"三行占位待办 | noise | - | nodejs | volatile | 空壳废稿可删
python/00-MOC-Python.md | Python 学习地图:入门/cheatsheet 点评/生态/速查 | keep-maybe | moc | python | volatile | 与优质资源篇重叠,多处 TODO
python/PyEco-FastAPI基础.md | FastAPI 最小应用到路径/查询参数入门蒸馏 | keep-maybe | task | python | volatile | 官方文档重述为主
python/PyEco-uv工作流.md | uv 统一包管理/venv/版本管理完整工作流 | keep-core | task | python | stable | 含 PEP 723 单文件脚本亮点
python/Python-优质资源.md | Python 书/博客/工具精选,强调看带立场的作者 | keep-core | reference | python | volatile | 判断锚点方法论原创
python/Python-数据结构速查.md | list/dict/set 速查,每段配机制解释与坑点 | keep-core | reference | python | stable | 465 行高密度原创
python/Python-现代工具链清单.md | Python 全生命周期工具一维清单,自创紧凑记法 | keep-core | reference | python | volatile | 工具名单会漂移
python/Python-PEP速查.md | 常用 PEP 按主题分类,标注版本落地时间 | keep-core | reference | python | stable | 增量追加即可
react/00-MOC-React.md | React 知识域索引,带注解的三层导航 | keep-core | moc | react | stable |
react/01-React-状态机制.md | state 快照语义与 setState→重渲染完整链路 | keep-core | concept | react | stable | 标 draft 但完整
react/02-React-Hooks速查.md | 全部 Hooks 按类别速查,含 React 19 的 use() | keep-core | reference | react | stable |
react/03-React-状态管理.md | setState 易错速查:updater 队列、快照闭包、位置绑定 | keep-core | reference | react | stable | 与 01 互补互链
react/04-React-设计模式.md | Hooks 时代 5 个组件 API 设计模式+选型决策树 | keep-core | concept | react | stable |
react/05-React-性能优化.md | 渲染/调度/感知三层视角的优化框架与手段排序 | keep-core | concept | react | stable |
react/06-React-边界处理实践.md | Suspense 与 ErrorBoundary 统一为 catch-throw 机制 | keep-core | concept | react | stable |
react/07-React-TypeScript实践.md | React+TS 类型选择决策树(children/组件/事件) | keep-core | reference | react | stable |
react/08-React版本演进.md | 0.3→19 版本编年史与架构演进主线 | keep-maybe | reference | react | volatile | 编年罗列为主,建议表会过时
react/09-React-优质资源.md | 按 7 段消化路径组织的资源清单,每条定位点评 | keep-core | reference | react | volatile | 锚定 19.x
react/React-渲染机制.md | render/reconcile/commit 三段语义、bailout 四条件,源码锚定 | keep-core | concept | react | stable |
react/React-核心算法.md | 源码 6+ 经典算法:场景/复杂度/选型理由/定位 | keep-core | reference | react | stable | 行号锚定 v19.0.0
react/React-核心函数.md | 25 个核心函数按 Trigger→Commit 五段排序的源码地图 | keep-core | reference | react | stable | 升级需校对行号
react/React-并发模型.md | Lane bitmask 模型、Transition/Deferred 落点 | keep-core | concept | react | stable |
react/React-热点问题.md | 6 个高误解话题辨析(批处理/可中断/flushSync 等) | keep-core | concept | react | volatile | 当前未过期
react/React-Compiler.md | Compiler 机制、编译产物、启用决策(2026-05 现状) | keep-core | concept | react | volatile | 快速演进领域
react/React-Diff算法.md | reconcileChildrenArray 双遍遍历、与 Vue/Inferno 对比 | keep-core | concept | react | stable |
react/React-Fiber架构.md | FiberNode 结构、双缓冲、三指针、可中断取舍 | keep-core | concept | react | stable |
react/React-Hooks原理.md | hook 链表挂载、调用顺序敏感根因、节点字段 | keep-core | concept | react | stable |
testing/00-MOC-Testing.md | 测试体系索引 | keep-maybe | moc | testing | stable |
testing/Test-优质资源.md | 测试工具 4 条链接各一句话 | noise | reference | testing | volatile | 纯罗列无点评
testing/Test-Vitest入门.md | Vitest 安装/配置/断言/mock 通用入门教程 | keep-maybe | task | testing | volatile | 545 行生成式教程体,无原创
testing/Test-Vitest与Playwright选型.md | 80/20 法则论证 Vitest+Playwright 分层组合 | keep-maybe | concept | testing | volatile | 报告体冗长,结论仍成立需压缩
testing/Test-Vitest与Playwright速查.md | 两工具联合项目的目录结构与配置速查 | keep-maybe | reference | testing | volatile | 679 行配置堆砌,版本敏感
tooling/新型工具链-Rust.md | JS 工具链 Rust 重写全景(Rolldown/oxc/Biome)+覆盖判断 | keep-core | reference | tooling | volatile | 标 stub 实为高质量调研
tooling/00-MOC-Tooling.md | 工程化工具实施层索引,含判别启发式 | keep-maybe | moc | tooling | volatile | 半数条目待补充
tooling/JS-依赖更新.md | taze 本地巡检+Renovate/Dependabot 配置与选型 | keep-core | reference | tooling | stable | 收尾有选型结论
tooling/pnpm-包管理.md | pnpm 选型理由/常用命令/workspace/本地包开发 | keep-core | reference | tooling | stable | 实用自足
typescript/00-MOC-TypeScript.md | TS 域索引,按类型系统/高级特性/工程实践分组 | keep-core | moc | typescript | stable |
typescript/todo.md | 一句话学习计划(练类型体操) | log | - | typescript | stable | 可删
typescript/TS-泛型.md | 泛型函数/约束/接口教程式笔记 | keep-core | concept | typescript | stable |
typescript/TS-面试题.md | infer/类型守卫/enum vs 字面量联合三题整理 | keep-maybe | reference | typescript | stable | 与三篇概念笔记重复,宜合并
typescript/TS-类型守卫.md | typeof/instanceof/in/is/asserts 全套守卫 | keep-core | concept | typescript | stable |
typescript/TS-类型体操.md | 条件类型/infer/递归/分布式条件类型 | keep-core | concept | typescript | stable |
typescript/TS-工程实践.md | 类型目录组织、命名规范、as const 替代 enum | keep-core | concept | typescript | stable |
typescript/TS-模块配置.md | module/moduleResolution 按场景完全指南,874 行 | keep-maybe | reference | typescript | volatile | 与 TS-tsconfig 重叠,宜拆并
typescript/TS-优质资源.md | 仅 3 条链接无点评 | noise | reference | typescript | volatile | 碎片,可并入 MOC
typescript/TS-高级类型.md | Union/Intersection/可辨识联合/映射类型 | keep-core | concept | typescript | stable |
typescript/TS-协变与逆变.md | 协变/逆变/双变与 strictFunctionTypes,附直觉解释 | keep-core | concept | typescript | stable |
typescript/TS-类型系统基础.md | 结构化类型/多余属性检查/基础类型 | keep-core | concept | typescript | stable |
typescript/TS-ESLint联动.md | tsconfig 如何影响 ESLint 行为(flat config 现行) | keep-core | reference | typescript | volatile | 视角原创
typescript/TS-tsconfig.md | 按 Vite/Node ESM/CJS/库开发四场景的 tsconfig 速查 | keep-core | reference | typescript | volatile | module Preserve 等推荐现行
web-security/00-MOC-Web-Security.md | 七段组织的 Web 安全总索引,引用关系清晰 | keep-core | moc | web-security | stable | 组织枢纽
web-security/Sec-应急响应.md | PICERL 六阶段+IoC/TTP/MTTD 与合规时限 | keep-core | concept | web-security | stable |
web-security/Sec-反序列化.md | 各语言反序列化 RCE 与 gadget chain、Log4Shell 本质 | keep-core | concept | web-security | stable |
web-security/Sec-威胁建模.md | STRIDE/DFD/Trust Boundary 与工具全景 | keep-core | concept | web-security | stable |
web-security/Sec-密码哈希.md | argon2id/bcrypt/scrypt 选型与 OWASP 现行参数 | keep-core | concept | web-security | stable | 参数随 OWASP 更新
web-security/Sec-供应链安全.md | SBOM/SLSA/依赖混淆/typosquatting/XZ 后门 | keep-core | concept | web-security | stable |
web-security/Sec-密码学基础.md | 哈希/MAC/签名、AEAD、密钥管理、常见误用 | keep-core | concept | web-security | stable |
web-security/Sec-授权与越权.md | RBAC/ABAC/ReBAC 与 IDOR、多租户 | keep-core | concept | web-security | stable |
web-security/Sec-安全响应头.md | CSP/HSTS/nosniff 等现行头+过时头清单 | keep-core | concept | web-security | stable |
web-security/Sec-容器与云原生安全.md | 容器逃逸/镜像/K8s RBAC/IMDSv2 | keep-core | concept | web-security | stable |
web-security/Sec-API安全.md | OWASP API Top 10 (2023) 逐项 | keep-core | concept | web-security | stable |
web-security/Sec-JWT.md | 结构/签名算法/alg=none 等经典漏洞/撤销 | keep-core | concept | web-security | stable |
web-security/Sec-MFA-WebAuthn.md | SMS→TOTP→Push→WebAuthn/Passkey 强度递进 | keep-core | concept | web-security | stable |
web-security/Sec-Node常见漏洞.md | 原型污染/ReDoS/路径穿越/Mass Assignment | keep-core | concept | web-security | stable |
web-security/Sec-OAuth2-OIDC.md | 角色/token/grant/PKCE 与常见误用 | keep-core | concept | web-security | stable |
web-security/Sec-OWASP-Top10.md | 2021 版十项定义+场景+防御 | keep-core | concept | web-security | stable |
web-security/Sec-reference-学习资源.md | 按权威性分级的安全资源,含原创点评 | keep-core | reference | web-security | volatile | 需定期核对链接
web-security/Sec-Session-Cookie.md | Cookie 安全属性/会话攻击/Session vs JWT | keep-core | concept | web-security | stable |
web-security/Sec-SQL注入.md | 六类注入手法+参数化根治 | keep-core | concept | web-security | stable |
web-security/Sec-SSRF.md | 云元数据/IMDSv2/DNS rebinding/Capital One 案例 | keep-core | concept | web-security | stable |
web-security/Sec-XSS深入.md | DOM sink 清单/CSP 现代写法/Trusted Types/mXSS | keep-core | concept | web-security | stable |
```

## 本路 Top 10

1. `code-style/CodeStyle-配置组合模式` — 跨 ESLint/Helm/Kustomize/Nix 的原创横向归纳，网上无现成对应物
2. `browsers/Browser-渲染阻塞` — 正交维度重构叙述，全库最完整的原创技术写作
3. `react/React-热点问题` — "误解-机制-判断准则"三段式辨析，加工密度最高
4. `javascript/JS-元编程的框架应用` — 不讲 API 讲决策，独立成立的好文
5. `agent/Agent-Prompt注入与越狱` — OWASP LLM Top10 与传统 Web 安全建立本质对比
6. `electron/03-Electron-安全基线与攻击面` — 攻击链逐段对应安全开关，系列枢纽
7. `nodejs/Nodejs-事件循环` — 从设计动机到 libuv 六阶段的原创解释性长文
8. `go/Go-JS-TS差异锚点` — 依赖作者自身背景视角、不可再生的原创提炼
9. `web-security/Sec-SSRF` — 攻击面到真实案例，综合度与实战价值兼备
10. `devops/DevOps-Node版本管理` — 真实踩坑沉淀，"网上抄不到"的个人知识

## 行动建议

- nextjs 目录 volatility 最高，优先对齐 Next 16（PPR 表述、proxy.ts 改名）
- `Nodejs-TODO.md`、两个 todo 碎片、`Git-PR规范.md`（拼写错误且题文不符）可直接清理
- testing 三篇长文提炼压缩后再入库
