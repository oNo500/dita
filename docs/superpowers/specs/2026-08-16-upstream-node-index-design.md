# 上游节点索引设计（2026-08-16）

> 定位：把「标题取自领域事实标准」这条规则从人工自觉变成机器可查。索引是派生物，不是资产——它的价值不在内容，在于**让上游漂移可见**。
>
> 用户裁定（2026-08-16）：来源 = 本地 DITA-OT docsrc + 克隆 oasis-tcs/dita；校验 = prolog 声明 + 索引核实（不做标题字符串匹配）；治理要求 = 「没人维护就是垃圾场」。

---

## 一、问题

标题规则定为「标题就是该节点在领域知识树上的标准叫法」。执行中它靠 agent 自觉查上游、靠人工抽查兜底。两次翻车都出在这条：先是自造译名（专门化 / 内容仓库），后是拼装标题（14/22 篇带冒号副标题）。规则本身正确，缺的是机器面。

直接的机器化路径——拿标题去上游节点名做字符串匹配——不成立：本库标题是中文且常为组合（`simpletable 与 CALS table 的选用`），上游是英文单节点（`Table elements`）。匹配率会低到全是噪音。

## 二、方案：声明式溯源

不校验标题**像不像**上游节点名，改为校验每篇**声明**了自己对应哪个上游节点，且该声明**解析得到**。

```xml
<prolog>
  <source href="https://dita-lang.org/dita/archspec/base/specialization-structural" scope="external"/>
  <data name="domain" value="dita"/>
  <data name="upstream-node" value="Structural specialization"/>
  <data name="reviewed" value="2026-08"/>
</prolog>
```

- `value` = 上游节点标题原文（英文，逐字）
- 组合篇声明多条（`key space 与 key scope` 声明两条）
- 自造篇写 `value="coined"`，并在文件头注释写明三道关（穷尽查证 / 先怀疑切分 / 只组合不发明）

这样机器能查的是三件确定的事：声明的节点名是否真实存在、自造是否带说明、以及**上游改名后哪些声明失联**。第三件才是这套东西的目的。

## 三、索引的数据形态

**生成物，进版本控制。** 进版本控制是为了 lint 离线可用、diff 可复核；生成物是为了没人需要维护它。

`kb/vocab/upstream-nodes.tsv`（制表符分隔，一行一节点）：

| 列 | 含义 | 示例 |
|---|---|---|
| source | 来源标识 | `oasis-archspec` / `oasis-langref` / `dita-ot` |
| title | 节点标题原文 | `Structural specialization` |
| parent | 父节点标题 | `Configuration and specialization` |
| path | 源文件相对路径 | `specification/archSpec/base/specialization-structural.dita` |
| url | 可访问地址（有则填） | `https://dita-lang.org/dita/archspec/...` |

文件头三行注释：生成命令、来源版本（DITA-OT 版本号 + oasis-tcs/dita 的 commit SHA）、生成日期。**并注明「本文件由工具生成，勿手改」**——手改会被下次生成覆盖。

## 四、生成

`dita-tools upstream-index --out kb/vocab/upstream-nodes.tsv`

两个来源都是 DITA 源，走现有的 `dita_parser`：

- **DITA-OT**：`~/ws/tools/dita-ot-4.4/docsrc/`（302 topic / 55 map，随工具安装即有）
- **OASIS**：`git clone https://github.com/oasis-tcs/dita ~/ws/tools/oasis-dita`（规范源，含 archSpec 与 langRef）

遍历各 map 取 topicref 树，读每个 topic 的 `<title>`，输出扁平表加 parent 列。**克隆与版本记录写进 `scripts/setup-env.sh`**（该脚本是本仓的版本 SSOT，见 CLAUDE.md）。

## 四之二、中英文：不比较，只声明

索引是纯英文、逐字照抄上游节点标题；本库标题是中文或中英混排。两者之间**不做任何自动对应**——桥梁只有 `upstream-node` 那条显式声明。因此「中文标题对不上英文节点」不是需要解决的问题，而是这套方案刻意绕开的问题（见二）。

**索引不设中文列。** 两条理由：翻译只能手工维护，而手工维护的派生数据必然腐烂；更要紧的是，一旦有中文列，它就成了事实上的权威译名——「专门化」那类无出处译名会以数据形式复活。

作者发现节点名的路径（写作时的实际问题）：

1. **正文里已有英文句柄**。术语规则要求机制名一律英文原名，所以作者正文本就写着 `conref` / `keyref` / `specialization`，拿它 grep 索引即可。这是术语规则与索引之间的协同。
2. **词表即中英对照**。`term-*` 条目的 glossterm 为英文、glossAlt 含中文别名，中文→英文的查找已存在且受治理，不在索引里重复一份。
3. **顺父链走**。主题本身无英文机制名的篇（如「元数据的放置位置」），从 `parent` 列自上而下浏览定位——这是索引保留父链列的另一个用途。

## 五、校验（编号落定为 R19，2026-08-18）

落在 `dita_lint`，与 R12–R17 同一套严格度分级（draft 记 warning，curated/verified 记 error）：

| 情形 | 判定 |
|---|---|
| 无 `upstream-node` 声明 | 报缺失（dita 域 topic 强制；其他域暂不强制） |
| 声明值在索引中存在 | 通过 |
| 声明值不在索引中 | 报错：拼写错误、凭记忆编造，或**上游已改名/删除** |
| `value="coined"` 且头注释含三道关关键词 | 通过 |
| `value="coined"` 但无说明 | 报错 |

### 比对的归一化（2026-08-18 用户裁定：避免误报）

**误报会让规则失去可信度，最终被人忽略——那比没有规则更糟。** 比对前双方都做归一化，宁可漏报不可误报：

| 处理 | 理由 |
|---|---|
| **大小写不敏感** | 上游是 `Conditional processing`，本库标题写 `conditional processing`。逐字匹配会让全库误报 |
| **首尾空白去除、内部连续空白折叠为一个** | 抄写与换行带入的差异，不是真差异 |
| **归一化后精确匹配，不做模糊/子串匹配** | 模糊匹配会把 `Specialization` 匹到 `Overview of specialization`，制造假通过；假通过比误报更隐蔽 |

**声明解析不到时，消息必须列出三种可能而不是断言拼错**：拼写有误、上游已改名或删除、**或索引未收录该节点**（提示核对索引头的生成日期与来源版本）。索引本身可能不全——已知的刻意排除有 resource-only 子树、未随发行版发布的页面、conref 素材片段；把索引的空缺报成作者的错误，正是最伤可信度的一类误报。

**允许一篇声明多条**（组合篇如 `key space 与 key scope`），逐条校验，任一条不解析即报该条。

规格记档进 `kb/schema/rules.sch`，归属正本标注为 `naming-rules`。（原设计写的 R18 编号已被「maturity 必标」占用，**实现时落定为 R19**。）

## 六、治理

这是本设计的重点。索引不烂掉靠四条，不靠人的自觉：

1. **永不手改**。文件头声明 + 生成即覆盖。手改的东西活不过一次再生成，因此没人会去手改。
2. **版本锁定 + 失效可检测**。索引记录来源版本；`setup-env.sh` 记录期望版本。两者不符时 `just check` 报「索引落后于工具链」。
3. **接入 benchmark-registry**。词表里已有的对标登记机制（锚点 / 上次核对 / 复核档位 / 触发条件）新增一条：上游索引。走既定纪律——**事件触发为主**（DITA-OT 升级、OASIS 发新 beta），日历到期只 flag 不阻塞。人读面已在 `writing-sourcing` 交代。
4. **改名转为工单**。再生成后 diff 显示节点增删改名；改名会让相应 `upstream-node` 声明失联，lint 直接列出**哪几篇要改标题**。这是索引存在的根本理由：上游漂移从无声腐烂变成一张可执行清单。

## 七、回填

65 篇 dita topic 需要补 `upstream-node` 声明。素材现成：**各迁移任务的报告里都有「标题 → 上游依据」对照表**（Task 6/8/9/10a/10b/11/13 与改名任务的报告，位于 SDD workspace）。

⚠️ **这些报告随 workspace 删除而消失**。收尾（Task 14）前必须把全部对照表抽出，落到 `research/cases/notes-to-kb-migration/upstream-provenance.md`——否则回填要重新查 65 遍。

回填本身是机械活：按对照表写 `<data>`，自造篇写 `coined`（三道关说明已在头注释里，无需重写）。

## 七之二、推广到全库（用户裁定 2026-08-16：一定要推广，方便治理）

**规则是全库的，实现分期。** 「每篇声明自己对应上游哪个节点」是本库的通则，不是 dita 分支的特例。先做 dita 只因为它的上游恰好是可本地解析的 DITA 源，成本最低；证明有用后按分支推广。

**接口已经存在**：词表里的 `benchmark-registry` 早已为每个分支登记了权威锚点（bm-security → OWASP ASVS/WSTG/CWE；bm-web → roadmap.sh/MDN；bm-ai → Anthropic 官方文档结构；bm-engineering → SWEBOK…），并带 `last-benchmarked` / `cadence` / `event-trigger`。上游索引不是另起一套治理，是**给这些既有锚点各配一张可机读的节点表**。

推广时每个分支要补的只有两样：

| 要补的 | 说明 |
|---|---|
| 抓取器 | 把该上游的页面树取成 source/title/parent/path/url 五列。DITA 源直接解析；HTML 站点需按其导航结构抓取；纯 PDF 或无结构的上游则**只登记不建索引**（该分支的 R19 豁免，且豁免须显式登记而非静默跳过） |
| registry 补字段 | 在对应 `bm-*` 条目下加 `index-source`（抓取标识）与 `index-generated`（生成日期），使"锚点已建索引与否、索引是否落后"可查 |

索引表的 `source` 列与 lint 的按域启用，从第一版起就是为此留的——加分支不需要改格式、改声明写法、改 lint 逻辑，只加抓取器与 registry 条目。

**分期建议**：dita（本设计）→ ai（上游为 Claude Code 文档站，形态最接近、篇数已有 10）→ 其余分支随内容落地再建。空分支不必先建索引。

## 八、不做的事

- **不做标题字符串匹配**（理由见二）
- **首版实现只覆盖 dita 域**（规则全库通用，见七之二）
- **不自动改标题**：索引只报告失联，改名是人的判断
- **不索引正文内容**：只要节点标题与层级，不要页面正文

## 九、任务分解

1. **T1 环境**：`setup-env.sh` 增加 oasis-tcs/dita 克隆与版本记录；`docs/security.md` 无需改（只读克隆）
2. **T2 生成器**：`dita-tools upstream-index` 子命令 + 单元测试；产出首版 tsv 并提交
3. **T3 溯源抽取**：从各任务报告抽「标题 → 上游依据」对照表落 `research/cases/`（**必须先于 workspace 删除**）
4. **T4 回填**：65 篇补 `upstream-node` 声明
5. **T5 校验**：R19 落 `dita_lint` + `rules.sch` 记档 + `docs/architecture.md` 能力表登记 + benchmark-registry 加登记项
6. **T6 吸收五关**：实现+测试 → 差分对账 → 删旧法（无旧法可删，此步为确认人工查证不再是唯一路径）→ 同步文档 → `just check` 绿

T3 优先级最高且有时限（workspace 生命周期内）；T1–T2 与 T4–T5 可并行两路。
