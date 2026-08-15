# 内容类型框架（收敛版，待审）

> 替换之前"九类平铺"的清单。平铺的问题:看不出层次,容易"水多加面"(发现装不下就加)。
> 这版用**框架驱动**:先立维度(判断完整性),再分类型(结构骨架),题材是细分(灵活)。

## 核心区分：维度 > 类型 > 题材（三层，别混）

| 层 | 是什么 | 判断标准 | 变化 |
|---|---|---|---|
| **维度** | 知识的本质形态 | **完整性看这层**(4 个维度够不够) | 极稳,几乎不变 |
| **类型** | 结构骨架(DITA 基础类型 + Diátaxis 象限) | 有限、可论证完整 | 稳定 |
| **题材** | 象限内的写法细分 | 靠 `@outputclass` 标,**按需加,不改框架** | 灵活 |

**关键:"缺不缺"看维度和象限(框架层),不看"某个旧文件装不下"(那是题材,靠 outputclass 容纳)。** 这样既不水多加面,又能装下各种题材。

## 全貌（四维度）

```
知识内容
│
├─ 维度1：解释性文档（给人读，Diátaxis 四象限 + DITA 补充）
│   ├─ tutorial   快速上手 ──────── DITA task
│   ├─ how-to     操作指南 ──────── DITA task
│   ├─ reference  参考/查阅 ─────── DITA reference
│   │              └题材：cheatsheet 速查 · curated-resources 资源清单/策展
│   ├─ explanation 理解/概念 ────── DITA concept
│   │              └题材：best-practice 最佳实践 · tech-landscape 技术栈生态 · deep-dive 深度研究
│   └─ troubleshooting 排障 ─────── DITA troubleshooting
│
├─ 维度2：可复用工件（拿来就用，不是读来理解）← Diátaxis 盖不到，唯一维度缺口
│   └─ prompt · 模板 · 配置/代码片段 ── DITA reference + outputclass=artifact（待细化）
│
├─ 维度3：术语（一词一义）─────────── DITA glossentry（术语库主线在建）
│
└─ 维度4：组织/导航（结构，不是内容）── DITA map（不是 topic！MOC/学习路径都是 map）
```

## 维度1 展开：Diátaxis 象限 × DITA 类型 × 题材

四象限本身是完整的(两轴:学习↔应用 × 实践↔认知),不是凑的。题材落在象限内,靠 outputclass 细分,各有对标来的必校验项:

| 象限/类型 | DITA | 题材(outputclass) | 固定结构 · 必校验 | 依据 |
|---|---|---|---|---|
| tutorial | task | quickstart | 目标→前置→步骤→next | Good Docs ✅ |
| how-to | task | how-to | 问题→步骤→退出点 | Diátaxis ✅ |
| reference | reference | (纯参考) | 结构化条目;禁步骤 | Diátaxis ✅ |
| reference | reference | **cheatsheet** | 至少一张速查表;禁长段 | 设计惯例 ⚠️弱 |
| reference | reference | **curated-resources**(资源清单) | 分类外链 + **逐条点评/入选门槛**;来源必填 | 你的实践(每域一个) |
| explanation | concept | **best-practice** | 场景→做法→**理由**→**反例**→边界 | 模式语言 ✅ |
| explanation | concept | **tech-landscape** | 范围→候选→**选型标准**→**对比表**→推荐 | 评估矩阵 ✅ |
| explanation | concept | **deep-dive** | 问题→背景→分析→结论→**大量来源** | explanation 扩展 |
| troubleshooting | troubleshooting | (排障) | 症状→原因→处置 | DITA 原生 ✅ |

(curated-resources 是把我之前想"加类型"的"优质资源"正确归位:它不是新类型,是 reference 的一个**题材**。ADR/读书笔记同理——explanation 的题材,不新增。)

(tech-landscape 还有一层身份:它承担"领域全景",是**每个领域必需的产物**——见 dimension-completeness.md。别的题材可选,它每领域必有,哪怕先是满是盲区的维度清单骨架。配对的 quickstart 则按需,不强制每领域。)

## 维度2：可复用工件（唯一的框架级缺口）

Diátaxis 是"文档"框架,盖不到"**拿来就用的资产**"——prompt、模板、配置片段。它不是读来理解的,是复制/引用来用的。你做 AI,prompt 是核心工件。

- **不拆成三个类型**(prompt/模板/片段),是**一个维度**,题材靠 outputclass 分(prompt/template/snippet)
- DITA 表达(待细化):`reference` 基础 + `outputclass=artifact`,可取用体用 `codeblock`/`<coderef>` 承载,配元数据(适用场景、参数、来源、成熟度)
- 必校验:有可取用体 + 适用场景 + 来源

## 完整性怎么判断（防"水多加面"）

- **看维度**:4 个维度(文档/工件/术语/组织)覆盖了知识的全部形态吗?—— 目前齐了
- **看象限**:解释性文档的 Diátaxis 4 象限完整吗?—— 完整(框架自证)
- **不看单文件**:"某个旧笔记装不下"→ 先问它是哪个象限的哪个题材,用 outputclass 容纳,**不新增类型**
- 只有出现"**既不是文档、又不是工件/术语/组织**"的东西,才是真维度缺口,才动框架

## 待你审

1. **四维度**(解释性文档 / 可复用工件 / 术语 / 组织)—— 覆盖你知识的全部形态了吗?有没有第五维?
2. **工件维度**——认不认(它是唯一新增的维度)?prompt/模板/片段归一维,对吗?
3. **题材归位**——优质资源=reference题材、ADR/读书=explanation题材,不新增类型,对吗?
4. 这个"维度>类型>题材"三层,比之前九类平铺清楚吗?
