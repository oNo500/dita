# notes-to-kb-migration：调研笔记迁入 kb 的案例档案（2026-08-16/17）

> 定位：**档案，不是正本。** 本目录只保留一件东西——迁移过程中产生、而正本里没有位置的溯源素材。内容正本已全部迁入 `kb/`。

---

## 这个 case 是什么

`research/notes/` 下 16 篇 DITA 调研笔记（00–15）在 2026-08-16/17 被拆解、重写、迁入 `kb/` 成为 66 篇 dita 域 topic，另有 5 篇新建的本库治理规约（content-engineering 簇）与 13 条词表条目同批落地。迁移不是搬运：每一篇都重新定题、重新分事实与判断、重新按上游领域知识树给标题。

迁完之后，**笔记全部冻结为调研档案，不再更新**——16 篇标题下都有一行「已迁移」声明，写明正本去了哪几个 topic、哪些小节不迁及原因。

设计与计划：

- 设计稿 [`docs/superpowers/specs/2026-08-16-notes-to-kb-migration-design.md`](../../../docs/superpowers/specs/2026-08-16-notes-to-kb-migration-design.md)
- 上游节点索引设计 [`docs/superpowers/specs/2026-08-16-upstream-node-index-design.md`](../../../docs/superpowers/specs/2026-08-16-upstream-node-index-design.md)（本目录的直接下游）

## 正本已迁何处

| 原笔记 | 正本去向（kb 簇） | 篇数 |
|---|---|---|
| 00 角色与边界、12 哲学与原则 | `kb/topics/dita/principles/` | 5 |
| 01 核心模型、02 复用 | `kb/topics/dita/core-model/` | 8 |
| 03 条件化与分块、14 元数据与分类 | `kb/topics/dita/conditional/` | 7 |
| 05 专门化、09 架构基础、10 寻址与键空间、11 处理模型 | `kb/topics/dita/architecture/` | 20 |
| 04 工具链与构建、06 DITA-OT 插件、07 程序化处理 | `kb/topics/dita/toolchain/` | 14 |
| 08 实践建议、13 翻译与本地化、15 DITA 与 RAG | `kb/topics/dita/practice/` | 10 |
| `research/README.md` 的版本现状 / 权威资源 | `kb/topics/dita/dita-landscape.dita`、`dita-resources.dita` | 2 |
| **dita 域小计** | | **66** |
| `research/cases/kb-redesign/` 两份设计稿 | `kb/topics/content-engineering/` | 2 |
| 规则归并新建（术语 / 命名 / 腐烂检测三篇正本） | `kb/topics/content-engineering/` | 3 |

入口：`kb/maps/domains/dita.ditamap`（域 map）与各簇 map。全景篇 `dita-landscape.dita` 是这个域的读法总纲。

笔记本身仍在 `research/notes/`，作为**调研档案**保留：它们记录的是「怎么查到的、当时核对了哪些页面」，这层过程信息按设计裁定不迁进 kb。

## 本目录有什么

| 文件 | 内容 |
|---|---|
| [`upstream-provenance.md`](upstream-provenance.md) | **「标题 → 上游依据」全量对照表**，一行一篇：slug、定稿标题、上游节点名（或「本库自造」＋三道关摘要）、来源任务 |

### 为什么单独留这一份

标题规则是「标题就是该节点在领域知识树上的标准叫法」。执行时每篇都查过上游（OASIS archSpec / langRef、DITA-OT 4.4 docsrc、社区通行说法），查证结果写进了两个地方：各 `.dita` 文件的头注释（进版本控制，是正本），以及各任务报告里的对照表（在 SDD 临时工作区里）。

上游节点索引方案（R18 声明式溯源）要给这 65 篇补 `<data name="upstream-node">` 声明，**素材就是那些对照表**。而 SDD 工作区是临时目录，收尾即删——设计稿 §七为此写了一条带时限的任务（T3）：删除前必须抽出来，否则回填要重新查 65 遍。`upstream-provenance.md` 就是这次抽取的产物。

另有一层理由：Task 8 / 9 / 10a / 11 / 12 与改名任务在隔离 worktree 内执行，报告未拷回主仓、已随 worktree 清理消失。这几个簇（architecture、conditional、core-model、principles 共 38 篇）的溯源，除文件头注释外**只剩本表这一份抄本**。

## 边界

- **本表是抄本，不是正本。** 正本是各 `.dita` 的头注释，随内容一起演化；本表冻结于抽取时刻。两者不符时以文件头注释为准。
- **不做标题字符串匹配。** 中文标题与英文节点名之间的桥梁只有显式声明，这是上游索引方案刻意的设计（见设计稿 §四之二）。
- **索引查不出「节点存在但选错了」**——声明与标题出自同一判断。自造篇的三道关须由**非原作者**独立复核，这条写在 `kb/topics/content-engineering/rot-detection.dita` 的边界节里。
