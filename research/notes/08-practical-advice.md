# 08 · 实践建议

> **已迁移（2026-08-16）**：正本已迁 kb（`kb/topics/dita/practice/pitfalls.dita` ← §高频踩坑清单；`kb/topics/dita/practice/adoption-criteria.dita` ← §用 DITA 2.0 的现实判断 + §什么时候 DITA 是对的 + §什么时候 DITA 是错的，并吸收 `research/README.md`「四个招牌特性」的框架句；`kb/topics/dita/practice/customization-cost-ladder.dita` ← §二次开发的成本阶梯；`kb/topics/dita/practice/engineering-ci.dita` ← §工程化建议（[04](04-toolchain-and-build.md) §2 的项目文件小节一度并入本篇，13b 切分复议后拆为 `kb/topics/dita/toolchain/project-files.dita`，正本归 04）；不迁小节：§学习路径——学习顺序属 map 的编排职责，其中「四个新特性最能验证工具链边界」一句已并入 adoption-criteria；§本笔记集的覆盖情况与剩余缺口、§值得关注的方向——两者是调研档案，不属知识库内容），本文冻结为调研档案，不再更新。

## 高频踩坑清单

### `@class` / `@specializations` 相关

| 症状 | 原因 |
|---|---|
| Python/JS 解析拿不到 `@class` 或 `@specializations` | 它们的值由 schema 的默认值声明提供，必须开 `attribute_defaults` 或处理 preprocess 后的文件 |
| 自定义 XSLT 模板对专门化元素不生效 | 用了元素名匹配。必须 `contains(@class, ' topic/x ')` |
| `contains(@class, 'topic/li')` 误命中 | 忘了前后空格，会匹配到 `topic/lines` 之类 |
| 泛化后元素名不对 | `@class` 祖先链写漏了中间层，或结尾少了空格 |
| 重写文档后派生链错乱 | 转换时保留了旧的 `@class`，应该丢掉让目标 schema 重新注入 |

### 构建相关

| 症状 | 原因 |
|---|---|
| 改了插件没生效 | 忘了跑 `dita install`。DITA-OT 在安装时编译配置，不是运行时读 |
| DITA-OT 不按 2.0 处理我的文档 | doctype 声明没解析到 2.0 语法文件。检查 catalog 和 public ID |
| 构建报错但看不出哪个文件 | 加 `--debug -v` |
| 中间结果不对但不知道哪一步 | `--temp=./tmp --debug` 后逐阶段看临时目录 |
| schema 解析慢/需要联网 | 配 `XML_CATALOG_FILES` 指向本地 catalog |
| 行为和规范文档对不上 | DITA-OT 4.4 的基线是 2026-01-25 草案，**比 beta03 旧** |
| PDF 中文乱码/方块 | FOP 没注册中文字体，需配 `fop.xconf`；或改走 CSS 出 PDF |

### 内容相关

| 症状 | 原因 |
|---|---|
| conref 拉过来是空的 | 目标元素被 DITAVAL 排除了。**内容仓库文件不要做条件化** |
| key 覆盖不生效 | key 解析是**先定义者胜**。要覆盖就把 keydef 放在被覆盖的 mapref **之前** |
| `@keys` 报校验错 | 值是 `NMTOKENS`，只能空格分隔，不能用逗号 |
| 条件过滤后构建报"内容不合法" | 把必需子元素单独排掉了（如 `<steps>` 里所有 `<step>`，而模型要求 `step+`）。条件要打在能整体去掉的层级 |
| 输出里有内容但源码搜不到 | conref push（`@conaction`）从别的文件推进来的 |
| 条件值拼错但不报错 | 裸字符串匹配。上 subjectScheme 做受控值 |
| 同一 topic 在两个 map 里表现不同 | 正常 —— key 定义不同。这是特性不是 bug |
| 变量文本没替换 | 检查是不是写在 `<keytext>` 里，以及工具链版本是否支持 |

---

## 用 DITA 2.0 的现实判断

### 前提：2.0 仍是 beta

**v2.0-beta03（2026-07-02），未成为 OASIS Standard。规范还会变。**

| 场景 | 建议 |
|---|---|
| **研究 / 学习** | **直接学 2.0**。删掉的那些历史包袱本来就不该学 |
| **新项目，工具链自控**（DITA-OT + VS Code/Emacs） | **可以上**，但锁定 DITA-OT 版本、记录你基于的 beta 号 |
| **新项目，依赖 Oxygen / CCMS** | **先验证工具支持**。商业工具的 2.0 支持进度不由你决定 |
| **做二次开发** | **可以上** —— `@class` 架构稳定，插件和专门化方法论不受 beta 状态影响 |

### 上 2.0 要接受的三件事

1. **规范会变** —— 语法文件还在动，做专门化时锁定版本
2. **DITA-OT 的基线滞后于规范** —— 4.4 基于 2026-01-25 草案，beta03 更新
3. **生态参差** —— 编辑器、CCMS、第三方插件的支持要逐个验证

### 降低风险的做法

- **从稳定基类派生**：`<div>` `<ph>` `<section>` `<simpletable>` `<data>` `<foreign>` 这些不太可能再变
- **避开还在讨论中的区域**
- **先用最小样例把工具链跑通**再投入内容
- **锁定 DITA-OT 版本**（写进 CI 镜像或 `.tool-versions`）

---

## 什么时候 DITA 是对的

同时满足以下多数条件：

- ✅ 同一批内容要产出**多个变体**（多产品线 / 多受众 / 多平台 / 多渠道）
- ✅ 内容量大到人工同步会出错（几百篇以上）
- ✅ 有**专职或半专职**的内容维护者，愿意学一套体系
- ✅ 有**多语言翻译**需求（模块化能显著降低翻译成本 —— 只翻改动的 topic）
- ✅ 有合规/审计要求，需要内容的版本追溯和结构一致性保证

## 什么时候 DITA 是错的

- ❌ 一个产品、一份文档、一个渠道 → Markdown + 静态站生成器（Docusaurus / VitePress / MkDocs）完胜
- ❌ 团队全是工程师、文档跟代码走 → Markdown in repo，门槛就是一切
- ❌ 内容变化极快、结构不稳定 → 类型化会成为阻力
- ❌ 没人愿意维护工具链 → DITA-OT + 插件 + 字体配置是持续的运维成本

**中间路线**：核心手册用 DITA（需要复用和多变体的部分），周边内容用 Markdown，靠 `org.lwdita` 统一到一套 map 里构建。这条路在实践中比"全面 DITA 化"务实得多。

---

## 二次开发的成本阶梯

按成本从低到高，能用前面的就别用后面的：

```
1. @outputclass + CSS            ← 只改外观，零 schema 成本
2. DITA-OT 插件（XSLT 覆盖）      ← 改输出逻辑，不动内容模型
3. 约束（constraint）             ← 收紧内容模型，禁用不该用的元素
4. 属性专门化                     ← 加条件化维度
5. 域专门化                       ← 加跨类型的语义标记
6. 结构化专门化                   ← 造新 topic 类型，成本最高
```

**第 6 项的真实成本**：一套 RNG/DTD 模块 + catalog 插件 + 编辑器配置 + XSLT 渲染模板 + 团队培训 + 规范演进时的跟进。只有当"语义校验"或"语义驱动的自动化"能带来明确回报时才做。

---

## 工程化建议

### 目录组织

```
docs/
├── maps/
│   ├── user-guide.ditamap
│   ├── admin-guide.ditamap
│   └── keys/
│       ├── product-keys.ditamap     # <keytext> 变量
│       └── link-keys.ditamap        # 外链
├── topics/
│   ├── concepts/
│   ├── tasks/
│   └── reference/
├── warehouse/                        # conref 内容仓库，禁止条件化
│   ├── notes.dita
│   └── snippets.dita
├── filters/
│   ├── linux.ditaval
│   └── enterprise.ditaval
└── images/
```

### 命名与 ID

- topic 文件名 = topic 的 `@id`，全库唯一。这样看到 `#install/step3` 就知道在哪个文件
- **`@id` 一旦发布就不要改** —— 它是 conref、xref、外部深链的契约

### CI

```bash
#!/bin/bash
set -e
find docs -name '*.dita' -o -name '*.ditamap' | xargs -n1 xmllint --noout --valid
dita validate --input=docs/maps/user-guide.ditamap
dita -i docs/maps/user-guide.ditamap -f html5 -o /tmp/ci --debug
for f in docs/filters/*.ditaval; do
  dita -i docs/maps/user-guide.ditamap -f html5 \
       -o "/tmp/ci-$(basename "$f" .ditaval)" --filter="$f"
done
```

最后一步常被忽略：**只构建默认变体，会漏掉某个条件组合下内容变不合法的问题**。

变体矩阵一旦变大（多 transtype × 多 ditaval），把这串 for 循环换成 **DITA-OT 项目文件**（`dita --project`，见 [04](04-toolchain-and-build.md)）—— 交付物声明一次，CI 里一条命令。

---

## 学习路径

给"学标准 + 做二次开发"这个目标的顺序：

1. **动手跑通**（半天）：装 DITA-OT 4.4，写一个最小的 2.0 topic + map（带正确的 doctype），出 HTML。**先确认工具链真的按 2.0 处理了**
2. **用满四个新特性**（半天）：`<keytext>` + `<titlealt>` + `<include>` + `chunk="combine"`。这四个是 2.0 里最实用的，也最能验证工具链边界
3. **读规范的架构篇**（1–2 天）：重点是 specialization 那几章。语言参考篇（元素字典）当手册查，不用通读
4. **做一遍完整的复用练习**（1 天）：造一个有 2 个平台变体、2 个受众变体的小手册，conref + keyref + DITAVAL 全用上。**这一步能否顺利，决定你是否真的理解了 DITA**
5. **写第一个插件**（半天）：覆盖 `<note>` 或 `<div>` 的 HTML 渲染
6. **做一次域专门化**（1 天）：加一个行内元素，走完 RNG → catalog → 插件 → XSLT 全流程；再加一个属性专门化（`@props/region`），验证泛化和 DITAVAL 过滤
7. **读 org.lwdita 源码**：真实世界里插件 + 自定义解析器的最佳范例

跳过第 4 步直接学专门化是最常见的错误 —— **不理解复用机制，就不知道该专门化什么**。

---

## 值得关注的方向

- **2.0 何时转正**：beta03 是 2026-07，转正后生态才会真正跟上
- **CSS-based PDF** 正在取代 XSL-FO：对没有 FO 经验的团队门槛低太多
- **`<include>` 打通文档与代码同源**：README、配置样例、代码片段直接从源码仓库拉，这是 2.0 最被低估的特性
- **LwDITA / Markdown 混合**：降低作者门槛的现实路径，比"全员学 XML"可行
- **结构化内容 + LLM**：topic 是语义完整的检索单元，`@class` 与条件属性提供现成的结构化元数据 —— 完整论证与工程决定（解析形态、分库还是检索时过滤）见 [15-DITA与RAG](15-dita-and-rag.md)

---

## 本笔记集的覆盖情况与剩余缺口

### 架构与处理模型主题的位置索引

| 主题 | 位置 |
|---|---|
| 条件处理的规范语义（判定逻辑 / flag 优先级 / passthrough） | [03-条件化与分块](03-profiling-and-chunking.md) |
| 目录（TOC）生成规则、索引设施 | [11-处理模型](11-processing-model.md) |
| 翻译与本地化（`@xml:lang` / `@dir` / `@translate`、复用与翻译的反模式） | [13-翻译与本地化](13-translation-and-localization.md) |
| subjectScheme 的规范语义（沿层级向上查找 / 绑定 / defaultSubject） | [03](03-profiling-and-chunking.md) + [14](14-metadata-and-classification.md) |
| 链接生成（`@collection-type` / `@linking` / reltable 规则） | [11](11-processing-model.md) |
| DITA 与 RAG（切块 / 元数据 / 解析形态的选择） | [15-DITA与RAG](15-dita-and-rag.md) |
| 元数据放在哪、Dublin Core 对应、分类树设计 | [14-元数据与分类策略](14-metadata-and-classification.md) |
| 文档类型外壳（document-type shell） | [09-架构基础](09-architecture-foundations.md) |
| 扩展模块（expansion module） | [09](09-architecture-foundations.md) |
| 三大扩展设施的正式框架 | [09](09-architecture-foundations.md) |
| Conformance —— 一致的处理器/文档/shell | [09](09-architecture-foundations.md) |
| 寻址体系（直接 vs 间接、片段标识符两种语法） | [10-寻址与键空间](10-addressing-and-key-space.md) |
| Key space 的正式模型、`@keyscope` 嵌套、跨交付物寻址 | [10](10-addressing-and-key-space.md) |
| **分支过滤 × 键空间**（含"必须先过滤再建键空间"的顺序约束） | [10](10-addressing-and-key-space.md) |
| **确定属性有效值**的五级优先级 | [11-处理模型](11-processing-model.md) |
| 元数据级联的完整属性清单与 `@cascade` merge/nomerge | [11](11-processing-model.md) |
| conref 解析时的属性规则与 `-dita-use-conref-target` | [11](11-processing-model.md) |
| 排序与 `<sort-as>`（含中文注音排序键） | [11](11-processing-model.md) |

### 剩余缺口

**处理模型** —— 已全部覆盖（TOC / 索引 / 链接生成在 [11](11-processing-model.md)，条件处理在 [03](03-profiling-and-chunking.md)，subjectScheme 在 [03](03-profiling-and-chunking.md)/[14](14-metadata-and-classification.md)）。层级链接的具体生成属实现惯例，规范未单列算法。

**架构**

- **模块化与词汇模块**的正式分类与命名规则（09 只给了轮廓）
- **约束模块**的详细规则（09 只从与扩展的对比中带过）
- shell 三组规则的具体条文（构造规则 / 等价性 / 一致性）
- 跨专门化共享元素、`<foreign>` 与 fallback、`@base` 属性专门化
- 约束/扩展不同组合的文档之间的**互操作性判定**

**完全未覆盖**

- **无障碍（Accessibility）**（**可选，仅记录**）—— 规范有独立章节；目前仅 [01](01-core-model.md) 3.6 节的 `<alt>` 一句带过，暂不展开
- 大规模内容库性能与增量构建、XSLT 3.0 高级特性（streaming / accumulator / packages）
- CCMS 集成模型、LwDITA（XDITA/HDITA/MDITA）、动态内容交付（DCD）

### 工程规划（未动工）

先立一条纪律：**两类验证的判定依据不同，不得混用** —— DITA-OT 4.4 的基线比 beta03 旧约 5 个月，构建结果**不能用来裁决规范层的断言**；让滞后的实现当裁判，理论会被修剪成"能在旧基线上跑通的样子"。据此，示例与断言的验证分成两件事：

**用语法文件校验示例（规范层，可独立先行）** —— 示例文件用 **v2.0-beta03 自带的 RNG/DTD** 直接校验（jing / xmllint + OASIS catalog，不经 DITA-OT）。语法文件是规范的组成部分（RNG 为规范性版本，见 [00](00-roles-and-boundaries.md)），这样校验示例合法性仍属规范层验证，不掺入任何实现的行为。已修正的 prolog 顺序类错误由这一层兜住。

**用 DITA-OT 构建验证实现行为（实现层，待工具链规划启动后随之进行）** —— 可构建的示例工程只能验证本来就标注"来自实现观察"的断言：preprocess 流水线顺序（[04](04-toolchain-and-build.md)）、`@class` 模块名实际取值（[06](06-dita-ot-plugins.md)）、lwdita 对 2.0 元素的覆盖（[07](07-programmatic-processing.md)）、`dita init` 模板名（README）。其结论只进"实现观察"档，**不得回写规范层内容**；构建结果与规范冲突时，默认解释是基线滞后而非笔记有错（见本篇踩坑表）。

**其他待办**：无障碍降为**可选项，仅保留记录**，暂不展开。（lwdita 的 TypeScript 实现已核实并更新进 [07](07-programmatic-processing.md)；LLM/RAG 已成篇为 [15](15-dita-and-rag.md)。）

---

## 来源

**已逐页核对（2026-08）**

- [DITA-OT 发布说明](https://www.dita-ot.org/dev/release-notes/) — 4.4（2026-01-31）、Java 17+
- [DITA 2.0 preview 支持](https://www.dita-ot.org/dev/reference/dita-v2-0-support.html) — DITA-OT 基线为 2026-01-25 草案（正文"基线滞后于规范"一条的依据）
- [oasis-tcs/dita 发布页](https://github.com/oasis-tcs/dita/releases) — v2.0-beta03（2026-07-02），pre-release
- [DITA 规范列表](https://www.dita-lang.org/specifications) — 2.0 仍列为 draft
- [关于 chunk 属性（架构规范）](https://dita-lang.org/dita/archspec/base/chunk-attribute-overview) — "DITA processing" 章节子树，用于上方缺口清单
- [specializations 属性规则与语法](https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute) — 架构规范完整目录（根章节与 "Configuration and specialization" 子树），用于上方缺口清单
- [迁移到 DITA 2.0](https://dita-lang.org/2.0/dita/non-normative/information-about-migrating-to-dita-2-0) — `@keys` 为 NMTOKENS（踩坑表一条）

**未逐页核对，来自通用实践与判断**

- 踩坑速查表的绝大部分条目（来自 DITA/DITA-OT 使用经验，非规范条文）
- 选型判断、二次开发成本阶梯、目录组织建议、CI 脚本、学习路径 —— 均为判断性内容
- "值得关注的方向"一节为个人判断，非事实陈述
