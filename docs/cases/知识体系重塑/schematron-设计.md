# Schematron 业务规则设计（待审）

> 这是设计文档,供审查。审过后落地为 `.sch` 文件。
> 定位:**写作规则三层的中间层**——RNG(schema 层)管结构合法,Schematron 管 RNG 表达不了的**跨属性/语义/业务规则**,topics/writing/(人读层)是给人看的规范。三层各司其职。

## 为什么需要它(RNG 做不到的)

RNG 只能校验"元素/属性能不能出现、顺序、取值范围"。管不了:
- "volatile 的内容**如果**标了 verified,**就必须**有核对日期"——这是跨属性的条件规则
- "每个 topic 必须有非空 shortdesc"——RNG 里 shortdesc 是可选的,要收紧得靠约束模块或 Schematron
- 受控值的即时校验(subjectScheme 的 enumerationdef 是构建期生效,编辑时不报)

这些正是词表治理规则和写作规则里"该强制"的部分,Schematron 把它们变成编辑/审查时能自动拦的检查。

## 规则清单(逐条可审,勾选纳入与否)

| ID | 规则 | 依据 | 严重度 | 纳入? |
|---|---|---|---|---|
| R1 | 每个 topic 必须有非空 `<shortdesc>` | 检索摘要(15 依赖)、writing-llm-friendly | error | ? |
| R2 | 每个 topic 必须显式标 `@volatility` | 词表 volatility **故意不设默认**,漏标应报错 | error | ? |
| R3 | `volatility="volatile"` 且 `maturity="verified"` 的 topic,必须有核对日期 | 词表治理规则"volatile 无核对日期不得 verified" | error | ? |
| R4 | `@maturity` 只能 draft/curated/verified | 词表受控值 | error | ? |
| R5 | `@volatility` 只能 stable/volatile | 词表受控值 | error | ? |
| R6 | `@tool` 只能 tool-claude-code/tool-codex/tool-antigravity | 词表受控值 | error | ? |
| R7 | 正文关键概念术语建议用 `<term keyref>` | 术语一致性(writing-llm-friendly) | warning | ?(难自动判定,可暂缓) |
| R8 | concept/reference 必须有至少一个来源 | 来源驱动(来源与时效-设计) | error | ? |
| R9 | 领域 map 必须挂一个领域全景 topic(tech-landscape) | 维度完整性-方法与约束 | error | ? |
| R10 | quickstart 必须 xref 到所属全景并有取舍声明 | 维度完整性-方法与约束 | error | ? |

说明:
- **R4/R5/R6** 与 subjectScheme 的 enumerationdef 重叠——但那是**构建期**校验,Schematron 让它在**编辑期即时**报,更早拦住。二者不冲突,是互补。
- **R7** 最难:"关键概念术语"无法自动可靠判定。可选做法:只对已在术语库(glossary)存在、但正文用了裸 `<term>`(无 keyref)的词报 warning——但这要术语库先建(Phase 5 前置)。**建议 R7 暂缓,等术语库建成再加。**
- **R8** 来源强制,来自来源与时效-设计。concept/reference 缺来源报错——内容本就该有出处。
- **R9/R10** 维度完整性:R9 保证每个领域有全景 topic,R10 保证 quickstart 挂靠全景并声明取舍。原设想的"全景必有对标来源"不单列——全景是 concept,已被 R8 覆盖,不重复设规则。

## 待你拍板的决策点

**决策 1:核对日期放哪(R3 的前提)。** ~~原推荐 (a) `critdates/reviewed`~~——**已证不成立**:DITA `critdates` 的子元素只有 created/revised/golive/expired,没有 `reviewed`(样板 electron-landscape.dita 校验时报错)。改**采用 (b) `<data name="reviewed" value="..."/>`**,与维度标引的 data 机制(domain/planned-dimension/covers-dimension)统一。若要用标准元素,可退而用 `<critdates><revised modified="..."/></critdates>`,但语义是"修订"非"核对",不如 data 直白。

**决策 2:R1(强制 shortdesc)用 Schematron 还是约束模块?** 两条路:
- Schematron:灵活,和其他规则一起
- 约束模块(RNG):更"硬",编辑器补全时就不给你跳过——但要改 shell(08 成本阶梯第 3 级)
推荐 Schematron(统一在一处,且不用动 shell)。

**决策 3:集成方式(可多选)。**
- (i) **编辑期即时**:`.dita` 里加 `xml-model` 关联 Schematron,VS Code 的 RedHat XML 扩展编辑时即报(像 OASIS shell 的 checkShell 那样)。最顺,需装 RedHat XML 扩展。
- (ii) **审查脚本**:Schematron 编译成 XSLT,一条命令跑全库,批量审查用。
- (iii) **构建阶段**:集成进 DITA-OT 构建的 validate,发布前拦。
推荐 (i)+(ii):编辑即时 + 批量脚本;(iii) 可选。

**决策 4:R7 术语 keyref 现在做还是暂缓?** 推荐暂缓(依赖术语库,且判定难)。

## 处理器(落地时用)

Schematron 需处理器把 `.sch` 编译成 XSLT 执行。DITA-OT 自带 Saxon(XSLT 引擎)。选:
- **SchXslt**(纯 XSLT 编译,用现成 Saxon 跑)——推荐,不装额外东西
- ph-schematron(Java 库)

## 落地产物(审过后)

- ✅ `kb/schema/rules.sch`——R1–R10 规则规格(人读)。
- ✅ `kb/scripts/check-rules.xsl`——可执行实现,DITA-OT 自带 Saxon 跑(DITA-OT 不带 SchXslt/ISO skeleton,故先手写等价 XSLT;接 SchXslt 后可直接编译 rules.sch,删除本份消除重复)。
- ✅ `kb/scripts/review.sh`——串 RNG 结构 + R1–R10 + 覆盖度,有 error 退出非零。已跑通。
- ⬜ 集成 (i) 编辑期 xml-model 关联 rules.sch —— 待接(需编辑器支持 Schematron)。

---

**决策（已定，2026-08-09 按推荐通过）**：R1–R10 全纳入。
- 决策 1（核对日期）：用 `data name="reviewed"`（DITA `critdates` 无 `reviewed` 元素，样板已证）。
- 决策 2（强制 shortdesc）：用 Schematron（R1），不动 shell。
- 决策 3（集成）：(i) 编辑期 `xml-model` 关联 + (ii) 审查脚本 `review.sh`，(iii) 构建期可选。
- 决策 4（R7 术语 keyref）：**不永久暂缓，改为"术语库首批建成即激活"**——术语库正在建（见 术语与双语-设计）。
- R8 来源、R9 领域 map 挂全景、R10 quickstart 挂全景：一并纳入。

下一步落地 `kb/schema/rules.sch`（R1–R10，SchXslt 编译走 DITA-OT 自带 Saxon）+ `kb/scripts/review.sh`（串全套）。
