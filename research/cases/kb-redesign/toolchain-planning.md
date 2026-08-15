# DITA-OT 工具链规划（版图 + 优先级，待审）

> DITA 已定为选型(无可替代:术语一致性 + 双语 + 单源多变体)。本文把围绕 DITA-OT 的完整工具链铺成八层,标清现状/缺口/优先级,供你定推进顺序。
> 原则不变:两条全程主线(术语、双语)最高优先,其余按需。

## 作者模型：AI 主笔（这决定各层怎么排）

**工作流定了:**
```
你：写 Markdown，或只给需求
 ↓
AI：写成 DITA（主要作者）
 ↓
机器兜底：validate(结构) + Schematron(业务规则) + 术语后处理(字面词→keyref)
          —— 因为 AI 会写错、术语带偏、漏标注
 ↓
入库
```

**这个定位重排了工具链优先级:**

- **编辑层(层1)大幅降级/跳过**:你不手写 DITA,ditacraft/RedHat XML 那些"帮人写 DITA"的补全跳转对你价值低。**不用折腾编辑器了。** 你的编辑体验是 Markdown(哪个编辑器都行)。
- **校验/审查(层3)+ 术语后处理(层4)升为核心兜底**:AI 是主笔,DITA 质量**全靠机器兜**——validate + Schematron R1–R7 + 术语后处理,从"审查/可选"升为"必备"。AI 主笔越多,这条兜底线越关键。
- **产出方式:AI 直接产出精确 DITA,以准确为主(方式 A,2026-08-09 定)**:你写 md/给需求,AI **直接产出精确 DITA**——填进类型模板,用精细行内语义(`cmdname`/`filepath`/`apiname`/`xmlelement`),**不为省 token 简化**。**不走"AI 写 md 再脚本转"(方式 B)**——那省 token 但把 DITA 的精细语义糊成一个 `codeph`,而你的内容(最佳实践/cheatsheet)价值恰在精确,以准确为主。
  - **代价(接受)**:AI 碰 XML 易出语法错 → `validate` 是安全网(A 路线下最关键),可能有"填错→报错→AI 修"的返工;token 比纯 md 多。
  - **但 token 不爆炸**:结构骨架(模板给)、`@class`/`@specializations`(schema 注入)、校验(机器做)都**不烧 AI token**——只有"正文带精细标签"这部分比 md 多,可控。
  - **三重准确保证**:类型模板(结构对)+ 给 AI 的 DITA 写作规则(教何时用 cmdname/filepath/…、何时 concept/task)+ 机器兜底(validate 抓语法/结构错、Schematron 抓业务规则)。
- **闭环:给 AI 的"DITA 写作规则"本身就是 kb 内容**。怎么让 AI 写好 DITA(选类型、用 keyref、标成熟度、双语)——这套规则就是**一个 agent 规则集**(Phase 4 那条线的直接延伸),AI 读它来写 kb。**用 kb 管理"让 AI 写好 kb 的规则"。** 这也正是你最初痛点"AI 帮写、术语被带偏"的完整解法:不是指望 AI 自觉,是规则约束 + 机器兜底。

## 八层版图

| 层 | 现状(已有) | 缺口(待建) | 优先级 |
|---|---|---|---|
| **1 编辑** | ditacraft(体验差)、VS Code/Cursor | ~~换 RedHat XML 扩展~~ **跳过**(AI 主笔,人不手写 DITA;你写 md 用任意编辑器) | **跳过** |
| **0 Markdown→DITA** | **写作规则集首篇 ✅**(dita-authoring-guide 总纲,并入 agent-rules) | AI 转(读 md 写成规范 dita);规则集细则按需扩 | 高(首篇已落) |
| **2 预览** | `preview.sh`(构建+HTTP+导航+CSS,可用) | watch/live-reload 准实时(可选) | 低(够用) |
| **3 校验/审查** | `dita validate`(RNG 结构)、构建报断链 | **Schematron R1–R7**(设计了待落地)、审查脚本(一条命令跑全套) | **高**(主线要 R7) |
| **4 术语**(主线一) | 无 | glossentry 术语库、`<term keyref>` 处理、**AI 后处理脚本**(字面词→keyref)、R7 | **最高** |
| **5 双语**(主线二) | 无 | 语言构建(中/英版)、**形态定稿**(见 terminology-and-bilingual-design.md) | **最高**(待形态定) |
| **6 构建/交付** | `build-agent-rules.sh`(单源多工具变体,验证过) | 项目文件(`dita --project`,多交付物)、部署脚本(产物→目标位置) | 中(Phase 6) |
| **7 自动化/CI** | 无 | git hook / CI(提交时校验 + 构建) | 中(批量后价值大) |
| **8 schema/插件** | concept-kb shell + 3 属性域(过审+验证) | **glossentry-kb shell**、reference/task shell、约束模块、catalog 解耦、CSS 主题插件 | **高**(术语要 glossentry shell) |

## 建议推进顺序（对照两条主线）

**第一批——两条主线前置(最高优先,直接决定 Phase 5 能不能开)**
1. `glossentry-kb` shell + `reference-kb` shell（层 8）
2. 双语术语库首批（层 4）——双语条目(中英首选词)
3. Schematron R1–R10 落地 + 审查脚本（层 3）
4. AI 术语后处理脚本（层 4）
5. 双语形态定稿（层 5,待你审 terminology-and-bilingual-design.md 的 4 个决策点）
6. 维度完整性落地（层 4/8,领域级建设纪律,见 dimension-completeness.md）——subjectScheme 加横切维度值集、agent-rules 加《维度完整性写作规则》、构建报告算覆盖度列盲区。R9/R10 已并入第 3 项。

**第二批——编辑体验(让批量重写顺手)**
6. 换 RedHat XML 编辑扩展(替 ditacraft)（层 1）
7. preview 加 watch/live-reload（层 2,可选）

**第三批——交付与自动化(内容起量后)**
8. 项目文件多交付物 + 部署脚本（层 6）
9. CI/git hook 自动校验构建（层 7）
10. catalog 解耦、约束模块、CSS 主题(按需)（层 8）

## 与已有 todo 的对应

这张版图把之前零散的 todo(#7–13)组织了起来:第一批 = #7(shell)+#8(术语库)+#11(schema/Schematron)+双语;第二批 = 编辑/预览;第三批 = #10(部署)+#13(RAG 远期)+#12(交叉标引)。**不是新增工作,是把已有待办按"层 + 批次"理清优先级。**

## 待你定

1. **推进顺序认不认**(第一批=两条主线前置,对不对)?
2. **编辑层**:换 RedHat XML 扩展?还是先不折腾编辑器、手写+preview 够用?
3. **CI/自动化**:现在就上(提交即校验),还是等内容起量?
4. 双语的 4 个决策点(在 terminology-and-bilingual-design.md,那个定了第一批才完整)
