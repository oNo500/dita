# notes → kb 迁移设计（2026-08-16）

> 定位：把 research/notes 的 DITA 2.0 研究（16 篇 + README）与 research/cases/kb-redesign 的未吸收干货迁为 kb 正本。本文是执行正本，供 agent 按簇执行；每篇写作遵守 kb 现行全部规则（写作七条 + R1–R16）。
>
> 用户裁定（2026-08-16）：范围 = notes/ 全部 + kb-redesign 未吸收部分；结构 = 6 子目录 + 6 子 map；节奏 = 单件流过晋级门，禁止批量大扫除。

---

## 一、范围

**迁**：`research/notes/00–15` 共 16 篇、`research/README.md` 的产物性内容（版本现状、30 秒上手、权威资源）、kb-redesign 中 2 份未吸收设计（content-type-framework、dimension-completeness）与 1 处可并入内容（README §4 防腐机制 → writing-sourcing）。

**不迁**：kb-redesign 其余 13 份（已吸收或裁定/过程档案，原样冻结）；vault-inventory 全部（过程档案）；notes 各篇的「来源」小节中**调研过程性**内容（核对方式描述留档案，来源结论进 topic）。

## 二、目录与 map 结构

```
kb/topics/dita/
  dita-landscape.dita               全景（tech-landscape）
  dita-resources.dita               权威资源（curated-resources）
  principles/     原理（笔记 12）
  core-model/     核心模型与复用（01、02）
  conditional/    条件与元数据（03、14）
  architecture/   架构机理（05、09、10、11）
  toolchain/      工具链与二开（04、06、07）
  practice/       工程实践（08、13、15）

kb/maps/domains/dita.ditamap        域 map：topichead ×6，各包一个 mapref
kb/maps/domains/dita/<簇>.ditamap   簇 map ×6
```

- mapref 合并语义：被引 map 的 title 不产生导航节点，**六簇的导航标题由域 map 内的 topichead 提供**，簇 map 不再自带包装层。
- 挂载点：root map 是九顶层分支的定稿结构，dita 不进 root——`maps/domains/content-engineering.ditamap` 内以 topichead 包 mapref 挂 dita.ditamap（与主题树挂点一致；navtitle 须与 dita.ditamap 的 title 逐字一致，dita-tools 一致性检查兜底）。`maps/glossary.ditamap` 随新词条增量修改。
- 主题树挂点：DITA 属 content-engineering / structured-content 一系。骨架阶段核实 subjectScheme 现有节点，在正确挂点登记 `dita` 主题节点与 `domain` 受控值——**目录分支（存储）与主题树挂点（分类）允许不同层级，以词表为准**。

## 三、产出清单（67 篇 topic）

命名 ASCII kebab，id = 文件名去后缀，全库唯一。题材即 `@outputclass`，按 R12/R13 带固定结构。表中标题为暂定，落笔时按写作规则七定稿（名词短语、定位词、对齐 OASIS 规范/DITA-OT 文档的类目命名）。

> **已失效（2026-08-16 Task 2/6 裁定）**：how-to 与 quickstart 均改绑 concept，task 类型退役、task-kb.rng 已删除。本节仅存档。

**缺口（骨架阶段补）**：quickstart / how-to 绑定 `dita-type=task`，但 `kb/schema/` 尚无 task shell——照 concept-kb.rng 模式新建 `task-kb.rng`（task 2.0 + maturity/volatility/dimension/tool 属性域），并确认 dita-tools lint 对 task 根元素同样生效。

### 分支根（2）

| 文件 | 标题（暂定） | 题材 | 来源 |
|---|---|---|---|
| dita-landscape | DITA 领域全景 | tech-landscape | README 版本现状 + 全笔记编排 |
| dita-resources | DITA 权威资源 | curated-resources | README 权威资源节 |

### principles/（5，笔记 00、12）

| 文件 | 标题（暂定） | 题材 |
|---|---|---|
| roles-and-boundaries | DITA 的角色分工与阅读边界 | deep-dive |
| schema-authority | schema 定义权与 @class 权限边界 | deep-dive |
| first-principles | DITA 的第一性原理推导 | deep-dive |
| costs-and-legacy | DITA 的代价与历史包袱 | deep-dive |
| portable-principles | 脱离 DITA 可迁移的原则 | deep-dive |

### core-model/（9，笔记 01、02）

| 文件 | 标题（暂定） | 题材 |
|---|---|---|
| topic-typing | topic 类型化与内容模型 | deep-dive |
| map-structure | map 的结构与内容分离 | deep-dive |
| titlealt-system | titlealt 标题体系 | reference |
| table-model-choice | 表格双体系选型：simpletable 与 CALS | best-practice |
| images-multimedia | 图像与多媒体元素 | reference |
| class-derivation | @class 与 @specializations 派生链 | deep-dive |
| conref-pull-push | conref 的拉与推 | deep-dive（含 02§4 处理顺序） |
| keyref-variable-text | keyref 与变量文本 | reference |
| include-non-dita | include 引用非 DITA 内容 | how-to |

### conditional/（7，笔记 03、14）

| 文件 | 标题（暂定） | 题材 |
|---|---|---|
| profiling-ditaval | 条件属性与 DITAVAL | how-to |
| branch-filtering | 分支过滤 ditavalref | deep-dive |
| subjectscheme-taxonomy | subjectScheme 受控值与分类法 | deep-dive（**合并 03§4 + 14§4**） |
| chunking | @chunk 分块 | reference |
| metadata-two-kinds | 过滤用与分类用元数据 | best-practice（14§1+§5） |
| dublin-core-mapping | Dublin Core 与 DITA 字段对应 | reference |
| metadata-placement | 元数据五种放置机制 | best-practice |

### architecture/（19，笔记 05、09、10、11）

| 文件 | 标题（暂定） | 题材 |
|---|---|---|
| structural-specialization | 结构化专门化 | deep-dive |
| domain-specialization | 域专门化 | deep-dive |
| attribute-specialization | 属性专门化 | deep-dive（**吸收 03§5**） |
| constraints-generalization | 约束与泛化 | deep-dive |
| specialization-practice | 专门化实战判据 | best-practice |
| extension-facilities | 三大扩展设施 | deep-dive |
| doctype-shell | document-type shell | deep-dive |
| vocabulary-modules | 词汇模块与约束/扩展模块 | deep-dive |
| conformance | DITA 一致性 | reference |
| addressing-modes | 直接与间接寻址、片段标识符 | deep-dive |
| key-space-model | 键空间模型与 keyscope | deep-dive |
| cross-deliverable-addressing | 跨交付物寻址 | how-to |
| branch-filter-key-space | 分支过滤与键空间的交互 | deep-dive |
| effective-attribute-values | 属性有效值五级优先级 | reference |
| metadata-cascade | 元数据级联与 @cascade | reference |
| conref-attribute-rules | conref 解析的属性规则 | reference |
| sorting-sort-as | sort-as 与中文排序 | how-to |
| nav-generation | TOC、索引与链接生成 | reference |
| processing-checklist | 程序化处理检查清单 | cheatsheet（**合并 10§7 + 11§6**） |

### toolchain/（13，笔记 04、06、07）

| 文件 | 标题（暂定） | 题材 |
|---|---|---|
| dita-ot-quickstart | DITA-OT 安装与 2.0 触发 | quickstart（**吸收 README 30 秒上手**） |
| preprocess-pipeline | preprocess 流水线 | deep-dive |
| validation-schematron | 校验与 Schematron | how-to |
| editors-pdf-reality | 编辑器与 PDF 输出的工具现实 | best-practice |
| plugin-extension-points | plugin.xml 与扩展点 | reference |
| xslt-override | XSLT 覆盖机制 | how-to |
| custom-transtype | 自定义 transtype | how-to（含 Ant 挂钩、界面本地化） |
| plugin-debugging | 插件调试工作流 | how-to |
| parsed-vs-source | 处理解析前还是解析后的 DITA | best-practice |
| programming-by-class | 按 @class 编程 | how-to |
| dita-ot-as-library | DITA-OT 当库用 | how-to（含中间产物版本污染坑） |
| generate-and-convert | 生成 DITA 与 Markdown 互转 | how-to |
| processing-tools | 程序化处理工具速查 | cheatsheet |

### practice/（10，笔记 08、13、15）

| 文件 | 标题（暂定） | 题材 |
|---|---|---|
| pitfalls | DITA 2.0 高频踩坑清单 | cheatsheet |
| adoption-criteria | DITA 采用判据 | best-practice（08§2–4） |
| customization-cost-ladder | 二次开发成本阶梯 | best-practice |
| engineering-ci | DITA 工程化与 CI | best-practice |
| translation-modularity | 模块化与翻译流程架构位 | deep-dive（13§1+§3） |
| localization-attributes | 三个本地化属性 | reference |
| translation-antipatterns | 翻译下的复用反模式 | best-practice（13§4+§5） |
| dita-rag-fit | topic 自足性与 RAG 切块的论证 | deep-dive（15§1+§5+§6） |
| rag-parsed-content | RAG 喂解析前还是解析后 | best-practice |
| rag-chunking-metadata | RAG 切块粒度与检索元数据 | best-practice |

### content-engineering/ 增补（2 + 1 处修改）

| 文件 | 标题（暂定） | 题材 | 来源 |
|---|---|---|---|
| dimension-type-genre | kb 的维度—类型—题材框架 | deep-dive | content-type-framework.md |
| domain-dimension-method | 领域维度框架的建立方法 | best-practice | dimension-completeness.md |
| （改）writing-sourcing | —— 并入两层防腐机制（volatility 管事实、benchmark-registry 管分类树） | —— | kb-redesign README §4 |

**不迁清单**（笔记内）：08§7 学习路径（map 的职责，见十）、08§8 覆盖情况（调研档案）、01§6 与其他方案定位差（并入全景）。

### 词表（按需，预计 +8~12）

conref、keyref、专门化、泛化、shell、键空间、DITAVAL、transtype 等，**第二篇测试触发才建**，双语条目（glossterm 首选词 + glossSynonym @xml:lang="en"），同步挂 glossary.ditamap。

## 四、每篇完成定义（DoD）

1. RNG 校验过（题材对应 shell：concept-kb / reference-kb / task-kb）
2. `dita-tools lint` 0 error 0 warning（R12–R16 全过）
3. 题材固定结构齐（R13）；来源节「事实/判断」两段，日期只在 prolog reviewed（R14）
4. prolog：source、domain=dita（content-engineering 增补篇除外）、reviewed；concept 根标 @dimension
5. 术语首现 keyref；触发第二篇测试的新词先建 glossentry
6. 挂入所属簇 map；`maturity="draft"`——晋 curated 是用户审后动作
7. 事实以笔记的已核对来源为据，注明出处；笔记未核对的点不得写成断言

## 五、单源纪律

每篇笔记全部内容迁完后，笔记头部加冻结声明并列出对应 topic：

```markdown
> **已迁移（YYYY-MM-DD）**：正本已迁 kb（<topic 文件清单>），本文冻结为调研档案，不再更新。
```

部分迁移的笔记不加声明，直到该笔记全部小节处置完（迁走或列入不迁清单）。

## 六、全景写法要求（Electron 教训的反面）

- 正文只交付维度框架的智力内容（为什么是这些维度、每个维度的问题域）
- 覆盖状态**零手抄**：不写"盲区"表格列、不写"当前覆盖度 n/m"，交给 `just ia` 从 planned-dimension 与 @dimension 求差
- 无标题回声（"本页是……"禁用）、无"此刻"式时间自指
- 版本现状表并入全景，`volatility="volatile"`，reviewed 日期随核对更新

## 七、节奏与里程碑

单件流，每步之间用户可审：

1. **骨架**：subjectScheme 挂点核实与登记 → 新建 task-kb.rng（见三）→ 目录 + 7 个 map → 全景 + 权威资源 2 篇 → `just review` / `just ia` 绿
2. **试点**：`02-reuse.md` → conref-pull-push、keyref-variable-text、include-non-dita 3 篇，走完整链（拆分→写作→lint 0→用户审→晋 curated）。**用户认可形状前不进入下一步**
3. **按簇推进**，volatile 优先：toolchain → practice → conditional → architecture → core-model 剩余 → principles。每簇完成停点给用户审，随簇附冻结声明与词表增量
4. **收尾**：content-engineering 增补 2 篇 + writing-sourcing 修改；README 冻结；全库 `just review` / `just ia` / `just links` / 交付物重建绿

## 八、缓建项（本设计不含）

- 角色阅读路径 map（README 五条路径 → audience map）：内容存在后作为独立小项
- dita 分支 reltable（互链关系表）：簇迁完后统一梳理
- kb-redesign 中 dimension-benchmark-report、gap-report 等判据档案的进一步处置：不动
