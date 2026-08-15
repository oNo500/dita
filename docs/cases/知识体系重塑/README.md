# 知识体系重塑：两角色设计稿（v0.1，2026-08-08）

> 定位：在 DITA 中从零重塑知识体系的设计正本。经验输入是 [../notes-盘点/](../notes-盘点/README.md)（422 篇实测），方法依据是 dita2 笔记（引用处标注篇号）。旧 Obsidian 库为只读矿场，内容不迁移——重写只带走主题与判据，不带走旧文本。
>
> 边界：工具链未定。本设计只产出**规范层构件**——[subjectScheme.ditamap](subjectScheme.ditamap) 是合法 DITA 文件，可先于任何工具存在；shell 与属性域的 RNG 实现留到工具链阶段。

---

## 一、信息架构师产出（00：内容组织）

### 1. 受控词表正本

见 [subjectScheme.ditamap](subjectScheme.ditamap)（现行 **v0.2**，评审全数通过）。要点：

- **主题树**九个顶层分支（lang / web / data / network / security / ai / engineering / foundations / content-engineering），三种依据齐备：文献依据（422 篇实测）+ 用户依据（检索直觉）+ 结构依据（五路外部对标）。九顶层经对标确认无一需改。设计裁定记录：
  - nodejs、electron 归 `web`（平台/运行时知识）而非 `lang`
  - coding-agents（原 agent-cli）归 `ai` 下——检索"AI 相关"时应命中；产品名叶子接受高周转
  - prompt 注入等 AI 安全内容设 `ai-security`，与 `security` 交叉标引而非塞进任一边
  - 原 PKM 方法论重写为去工具化的 `knowledge-mgmt`——Obsidian 专属知识（wikilink 语法等）不进新体系
  - v0.2 对标增补：db-fundamentals/vector-search、concurrency、context-engineering、docs-as-code、reverse-eng
  - v0.2 结构重构：content-engineering 分四组（structured-content / knowledge-organization / knowledge-mgmt / writing）；"知识图谱"横跨三域按交叉标引落点
- **三个属性值集**：maturity（draft/curated/verified，**默认 draft**——未审内容无法冒充成品）、volatility（stable/volatile，**不设默认**——时效必须显式判断）、tool（三个 agent CLI，按工具出变体的条件维度）
- **治理规则进词表**：volatile 无核对日期不得标 verified（dita2 来源纪律的 schema 化）

### 2. root map 规划

结构外置（12 推论 4），四类 map：

| map | 内容 | 说明 |
|---|---|---|
| **领域 map** × 9 | 每个顶层主题分支一个，组织该域的 topic | 对应旧库 MOC 的职能——**MOC 的 DITA 对应物是 map，不是 topic** |
| **glossary map** | 全库术语 glossentry | 术语一致性靠 `<term keyref>`（01）；旧库 27 条 glossary 是重写起点中最成熟的 |
| **交付物 map** | agent 规则集、按项目类型的知识包等 | 每个交付物 = map + DITAVAL（按 tool/maturity 过滤）+ key 绑定 |
| **root map** | 引所有领域 map | 全库键空间的根（10） |

### 3. 重写工作流

1. 从盘点的 keep-core 池选主题（volatile 主题优先——时效在流失；stable 主题不急）
2. 回矿场查原文，**按类型重写**为新 topic（不是搬运：重写时按 15 的自足性标准检验每个块）
3. 标注 `@subjectrefs` + maturity/volatility；verified 晋级必须完成来源核对并注日期
4. 挂入领域 map；术语首现建 glossentry 并改用 keyref

### 4. "更前沿"的机制保证（两层对称）

前沿性不是一次性属性，是维护属性。体系用两层对称的机制守它：

- **内容层**（防事实腐烂）：volatility 维度 + verified 的日期戳强制 + 交付物按 `maturity="verified"` 过滤——过期内容自动失去进入交付物的资格，不靠人记得它旧了。
- **结构层**（防分类树腐烂）：词表内的 `benchmark-registry`——九分支各登记对标锚点、上次核对日期、复核档位、事件触发条件。**灵活优先**：事件触发为主（笔记无处安放/外部体系改版），日历到期只兜底且仅 flag 不阻塞，到期默认轻量确认、发现大变才跑全量对标。这条把 2026-08-08 五路对标从一次性动作固化成可复现的纪律——五年后不成黑洞，靠的不是这次校准多准，而是"到期/触发即重新对标"一直在跑。详见评审单 G 节。

两层是同一个思路（来源核对 + 日期戳）作用在不同对象上：一层管一条事实，一层管一棵树。

---

## 二、DITA 架构师产出（00：词汇与文档类型）

### 1. 类型映射（05：先画映射表；01：已有类型能匹配就直接用）

| 内容类型 | 文档类型 | 盘点实测占比印证 |
|---|---|---|
| 原理 / 最佳实践 | `<concept>` | 实测最大类（web-security/react/electron 深度长文全是它） |
| 查阅型参考 / 速查 | `<reference>` | 第二大类（速查表/资源清单/API 矩阵） |
| 操作步骤 / skill | `<task>`（strict） | uv 工作流、Node 版本管理、各配置类 |
| 踩坑记录 | `<troubleshooting>` | quietpaper 撞墙记录、定时器精度、OpenClaw 踩坑 |
| 术语条目 | `<glossentry>` | 27 条现成语料 |
| ~~MOC~~ | **map**（非 topic 类型） | 组织归 map，内容归 topic——旧库 MOC 的正确去处 |
| ~~log~~ | **不进语料库** | 无自足性（12 推论 2），准入测试的定义性出局 |

**首版零结构化专门化**（08 成本阶梯）。唯一预留的升级点：agent 规则若将来需要语义校验（强制声明适用范围），再从 `<concept>` 专门化出 `<rule>`。

### 2. 属性专门化（03/05：从 `@props` 派生，不滥用 `@otherprops`）

| 属性 | 值集来源 | 过滤用途 |
|---|---|---|
| `@props/maturity` | subjectScheme | 交付物排除 draft——**喂给 agent 的输出只含 curated 以上** |
| `@props/volatility` | subjectScheme | 构建时 flag 无日期戳的 volatile 内容（03 flag 机制） |
| `@props/tool` | subjectScheme | 按工具出规则变体——**切换 agent CLI = 换一份 DITAVAL**（12 推论 5） |

### 3. shell 规划

一个外壳装配：concept / task / reference / troubleshooting / glossentry 结构模块 + 三个自有属性域 + strict taskbody 约束。shell 不定义任何元素（00 §7）。RNG 实现按 05 的路线从 OASIS 模板起步，属工具链阶段工作。

### 4. 写作规则三层（00：作者读写作指南，不读规范）

| 层 | 机制 | 内容来源 |
|---|---|---|
| schema 层 | 约束模块 | 强制 shortdesc（检索摘要，15 依赖它）、禁 draft-comment 入发布 |
| 业务规则层 | Schematron（04） | "volatile 且 verified 必须有核对日期"、"术语首现必须 keyref" |
| 人读层 | 数篇 concept topic | 原库《PKM写作规范》《AI-AGENTS写作标准》的判据精华重写（原文不迁移） |

---

## 三、评审结论（2026-08-08，全数通过）

评审单 A–G 全部裁定 ✅，词表升 v0.2。下一步进 **Phase 2 骨架搭建**：
- 新仓库 `~/code/kb`（独立 git，不混 dita2）
- root map + 领域 map 骨架 + glossary map
- 写作规则人读层 3–4 篇（判据精华重写）
- 首个交付物定为 **agent 规则集**（Phase 4）

裁定记录见 [评审单-v0.1.md](评审单-v0.1.md)。
