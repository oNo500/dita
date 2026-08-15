# kb — 知识体系

用 DITA 组织的个人知识库正本。从零重塑，不迁移旧内容——按主题重写，避免把噪音与过时表述带入。

## 这是什么，与其他两处的关系

| 路径 | 是什么 | 状态 |
|---|---|---|
| `~/code/notes` | 旧 Obsidian 库 | 只读矿场，永不迁移，重写时查原文用 |
| `~/code/dita2` | DITA 2.0 研究笔记 + 分类法实践案例 | 方法论来源（本库的设计依据都在其 cases/） |
| `~/code/kb`（本库） | 重塑后的知识体系正本 | 建设中（Phase 2 骨架） |

**dita2 是研究，kb 是成品。** 本库的分类树、角色分工、验证纪律全部承自 dita2 的笔记与 `cases/知识体系重塑/`。

## 结构

```
kb/
├── vocab/subjectScheme.ditamap   受控词表正本（分类 + 成熟度/时效/工具值集 + 治理登记）
├── maps/
│   ├── root.ditamap              全库总组织，引所有领域 map
│   ├── domains/*.ditamap         九个领域 map（lang/web/data/network/security/ai/engineering/foundations/content-engineering）
│   └── glossary.ditamap          术语库
├── topics/<领域>/                内容 topic，按领域分目录
│   └── writing/                  写作规则（人读 concept）
├── glossary/                     术语 glossentry
└── schema/                       DITA 架构师的零件（RNG 属性域 + shell + 约束，待造）
```

## 两个角色（一人分饰）

- **信息架构师**：内容怎么组织——词表、map。产物在 `vocab/`、`maps/`。
- **DITA 架构师**：内容用什么语法——shell、模块、约束。产物在 `schema/`。

切换帽子的判断表见 dita2 `cases/知识体系重塑/角色分工.md`。

## 纪律（承自 dita2）

- **准入测试**：一篇内容进库，须自足（脱离原语境仍成立）+ 可归型（concept/task/reference/troubleshooting/glossentry 之一）。过不了的是噪音，不进。
- **来源核对 + 日期戳**：volatile 内容无核对日期不得标 verified。
- **文件先于工具**：所有 `.dita`/`.ditamap` 是合法 DITA 文件，用 beta03 语法文件校验，不依赖 DITA-OT。
- **分类树防腐**：词表内 benchmark-registry 记录对标锚点与复核纪律，防结构过时。

## 当前进度

Phase 0 盘点 → 1 评审 → 2 骨架 → 3 试点重写 → 4 首个交付物，均已走通。
- `schema/`：三属性域 + concept-kb shell（过审 + 校验闭环）
- `topics/`：写作规则 4 篇 + agent 方法论 3 篇（Phase 3 试点）+ agent-rules-core（Phase 4）
- 交付物：`maps/deliverables/agent-rules.ditamap` + `filters/tool-*.ditaval`

下一步 Phase 5 批量重写（前置见 dita2 `cases/知识体系重塑/Phase3-回顾.md`）。规划全貌见 dita2 `cases/知识体系重塑/`。

## 构建 agent 规则集（Phase 4，单源 → 多工具变体）

一份规则源 + 各工具的 DITAVAL → 各工具的配置文件。**改一处源，所有工具变体同步更新**——不用手动同步多份。

```bash
scripts/build-agent-rules.sh          # 一条命令出所有变体 → out/<tool>.md
# 或手动单个变体：
dita -f markdown_github \
  --input=maps/deliverables/agent-rules.ditamap \
  --filter=filters/tool-claude-code.ditaval -o out/claude-code
# 产物 out/claude-code/agent-rules.md 即 CLAUDE.md 雏形；换 filter 出 codex 变体。
```

工具链：DITA-OT 4.4 + org.lwdita 的 `markdown_github` transtype + `chunk="combine"`（合成单文件）。均现成，不需自定义。
