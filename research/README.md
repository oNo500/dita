# DITA 2.0 研究笔记

纯 DITA 2.0，不含 1.3 的历史包袱和迁移内容。目标：**学懂标准 + 具备二次开发能力**（专门化 / DITA-OT 插件 / 程序化处理）。

## 版本现状（2026-08 核对）

| | |
|---|---|
| **DITA 2.0 状态** | **beta，尚未成为 OASIS Standard**。最新发布 **v2.0-beta03（2026-07-02）** |
| **参考实现** | **DITA-OT 4.4**（2026-01-31），Java 17+，基于 **2026-01-25 的 2.0 草案**提供 preview 支持 |
| **注意** | beta03 比 DITA-OT 4.4 的基线新约 5 个月，**两者可能有差异** |
| **TC 主线信号** | dita-lang.org 的**无版本号路径现在指向 2.0**，1.3 被移到 `/1.3/` 下 |

### 两条必须先接受的现实

1. **规范支持 ≠ 工具链支持。** DITA-OT 的 preview 已相当完整，但 Oxygen、各家 CCMS、第三方插件的 2.0 支持程度参差。投入内容前，先用最小样例把你实际要用的工具跑一遍。
2. **规范还会变。** 做专门化时锁定你所基于的 beta 版本号，并尽量从稳定基类（`<div>` `<ph>` `<section>` `<simpletable>` `<data>` `<foreign>`）派生。

### 对二次开发者的好消息

`@class` 记录元素派生链的机制是 DITA 架构的基础，并且是稳定的。按 `@class` 匹配的 XSLT 模板、专门化方法、泛化算法都不受 beta 状态影响，二次开发可以正常进行。

---

## 笔记索引

| 文件 | 内容 |
|---|---|
| [00-roles-and-boundaries.md](notes/00-roles-and-boundaries.md) | **五种角色**（作者 / 信息架构师 / DITA 架构师 / 实现者 / 处理器）、规范写给谁看、各角色该读什么、**谁可以定义 schema**、角色权限边界 |
| [01-core-model.md](notes/01-core-model.md) | topic 类型化与内容模型、task/steps 结构、map & bookmap、reltable、`<titlealt>` 标题体系、表格（simpletable vs CALS）、图像与多媒体、**`@class` 派生链**、`@specializations` |
| [02-reuse.md](notes/02-reuse.md) | conref（拉）与 conref push（推）、keyref 与 `<keytext>` 变量文本、key 解析规则与 key scope、`<include>` 引用非 DITA 内容 |
| [03-profiling-and-chunking.md](notes/03-profiling-and-chunking.md) | 条件属性、DITAVAL 四种 action、分支过滤 `<ditavalref>`、subjectScheme 受控值、属性专门化维度、`@chunk` 的 combine/split |
| [04-toolchain-and-build.md](notes/04-toolchain-and-build.md) | DITA-OT 安装、**怎么让它按 2.0 处理文档**（doctype + catalog）、常用命令、**项目文件（`--project`）**、preprocess 流水线各阶段、校验与 Schematron、编辑器、PDF 现实 |
| [05-specialization.md](notes/05-specialization.md) | 结构化 / 域 / 属性专门化、约束、泛化算法、`@class` 构造规则、好用的派生基类、RNG 路线 |
| [06-dita-ot-plugins.md](notes/06-dita-ot-plugins.md) | `plugin.xml` 与扩展点、XSLT 覆盖机制、处理 2.0 新元素、Ant 挂钩、自定义 transtype、调试工作流 |
| [07-programmatic-processing.md](notes/07-programmatic-processing.md) | 解析前 vs preprocess 后、**如何取到 `@class` / `@specializations`**、按 class 编程、DITA-OT 当库用、生成 DITA、DITA↔Markdown |
| [08-practical-advice.md](notes/08-practical-advice.md) | 踩坑速查表、上 2.0 的现实判断、选型判断、二次开发成本阶梯、工程化与 CI、学习路径、**覆盖情况与剩余缺口** |

### 架构理论（09–11）

| 文件 | 内容 |
|---|---|
| [09-architecture-foundations.md](notes/09-architecture-foundations.md) | **三大扩展设施**的正式框架（文档类型配置 / 专门化 / 元素类型配置）、**document-type shell 深入**、模块化与词汇模块、**约束 vs 扩展模块**、一致性（Conformance） |
| [10-addressing-and-key-space.md](notes/10-addressing-and-key-space.md) | 直接 vs 间接寻址、片段标识符两种语法、**键空间的正式模型**、`@keyscope` 嵌套与跨作用域、跨交付物寻址、**分支过滤与键空间的交互** |
| [11-processing-model.md](notes/11-processing-model.md) | **属性有效值的五级优先级**、**元数据级联完整属性清单与 `@cascade`**、conref 属性合成规则与 `-dita-use-conref-target`、`<sort-as>` 与中文排序、程序化处理的顺序检查清单 |

> 09–11 讲规范定义的模型，01–08 讲用法。自研处理逻辑若不遵循 09–11 的规则，结果会与 DITA-OT 不一致，且不符合规范。

### 原理（12）

| 文件 | 内容 |
|---|---|
| [12-philosophy-and-principles.md](notes/12-philosophy-and-principles.md) | **从第一性原理重构 DITA**：以"同一个事实只应存在于一处"为唯一公理，逐步推出 topic / map / key / 条件化 / `@class` / shell / 处理顺序；同样推出它的四项代价；区分"必然"与"历史包袱"；剥离 DITA 后可迁移的八条原则 |

> 12 是论证，不是规范转述。前 11 篇说明 DITA 是什么样，12 说明它为什么是这样。

### 翻译与本地化（13）

| 文件 | 内容 |
|---|---|
| [13-translation-and-localization.md](notes/13-translation-and-localization.md) | 模块化降翻译成本的机制、`@xml:lang` / `@dir` / `@translate`（含 **topicref 上的 `@xml:lang` 不作用于被引 topic**）、翻译流程与 XLIFF 的工具层定位、**复用机制在翻译下的两个反模式**（用变量拼句子、低于句子粒度的 conref） |

### 最佳实践（14 起）

| 文件 | 内容 |
|---|---|
| [14-metadata-and-classification.md](notes/14-metadata-and-classification.md) | 过滤用与分类用的元数据之分、**Dublin Core ↔ DITA 对应表**、元数据五种放置机制的取舍、**subjectScheme 作为分类法机制**（沿层级向上查找、空枚举禁用属性、defaultSubject）、**分类树按过滤语义画**的设计规则、字段设计流程 |
| [15-dita-and-rag.md](notes/15-dita-and-rag.md) | 论证篇：topic 自足性与检索切块的定义性吻合、**必须用解析后内容**（条件过滤是正确性问题）、按变体分库 vs 单库加检索时过滤（passthrough）、检索元数据对照表、**反面清单**（解析摧毁复用标识、不是上 DITA 的理由） |

> 14 起为最佳实践层：依据是社区经验 + 规范核对的混合，来源档区分"规范条文"与"观点来源"。

## 建议的阅读顺序

先读 [00-角色与边界](notes/00-roles-and-boundaries.md) 确认自己是哪个角色，再按下面选路径。

| 角色 / 目的 | 路径 |
|---|---|
| **作者**（写内容） | 00 → 01 → 02 → 03 |
| **信息架构师** | 00 → 01 → 02 → 03 → 10 → 13 → 14 → 08 |
| **DITA 架构师**（定义词汇与外壳） | 00 → 12 → 09 → 05 → 08 |
| **实现者**（自己写处理逻辑） | 00 → 12 → 09 → 10 → 11 → 07 |
| **工具链维护**（DITA-OT 插件） | 00 → 04 → 06 |
| **从零通读** | 00 → 12 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → 10 → 11 → 13 → 14 → 15 |
| **评估要不要用 DITA** | 只读 12 的第三节（代价）与第四节（必然 vs 包袱） |

---

## 30 秒上手

```bash
# 装 DITA-OT 4.4（需要 Java 17+）
brew install dita-ot
dita --version

# 脚手架（init / validate 是 4.3+ 的预览特性）
dita init --list
dita init --template=<模板名> --output=sample

# 构建
dita --input=sample/root.ditamap --format=html5 --output=out
```

**触发 2.0 处理靠 doctype 声明**（DITA-OT 经 XML catalog 解析到 2.0 语法文件）：

```xml
<!DOCTYPE topic PUBLIC "-//OASIS//DTD DITA 2.0 Topic//EN" "topic.dtd">
<!DOCTYPE map   PUBLIC "-//OASIS//DTD DITA 2.0 Map//EN"   "map.dtd">
```

版本号可以是 `2.0`、`2.x`（最新 2.x），或**整个省略**（等价于最新 2.x）。RNG 走 `urn:pubid:oasis:names:tc:dita:rng:<类型>.rng:<版本>`。

> 确切字符串以你所用 DITA-OT 随附的 `catalog-dita.xml` 为准 —— 草案期间会动。

---

## DITA 2.0 的四个招牌特性

学 2.0 优先掌握这四个，它们既最实用，也最能验证工具链是否真的支持：

| 特性 | 解决的问题 |
|---|---|
| **`<keytext>`** | 变量文本有了正式机制，内容模型支持 `<tm>` `<term>` 等，不只是纯字符串 |
| **`<titlealt>` + `@title-role`** | 导航标题 / 链接标题 / 搜索标题 / 副标题统一到一个元素，支持多值和自定义 role |
| **`<include>`** | 引用**非 DITA** 的外部内容 —— README、配置文件、代码片段从源码仓库直接拉，解决文档与代码漂移 |
| **`@chunk="combine\|split"`** | 源文件结构与输出文件结构解耦，只有两个取值 |

另外可用：`<div>`（通用块容器，理想的专门化基类）、`<audio>`/`<video>`、`<strong>`/`<em>`、`<sort-as>`。

---

## 权威资源

**规范**（无版本路径 = 2.0）
- 规范总入口（草案预览）：<https://dita-lang.org/specifications>
- 语言参考（base）：单页可直达，如 <https://dita-lang.org/dita/langref/base/shortdesc>；封面 <https://dita-lang.org/2.0/dita/resources/oasis-cover>
- 语言参考（技术内容）：concept / task / reference / troubleshooting 在此，如 <https://dita-lang.org/dita-techcomm/langref/technicalcontent/concept>
- 架构规范：单页可直达，如 <https://dita-lang.org/dita/archspec/base/topicstructure>
- `@specializations` 规则：<https://dita-lang.org/dita/archspec/base/specialization-specializations-attribute>
- TC 源码与 beta 发布：<https://github.com/oasis-tcs/dita/releases>

**工具链**
- DITA-OT 的 2.0 preview 支持：<https://www.dita-ot.org/dev/reference/dita-v2-0-support.html>
- DITA-OT 文档：<https://www.dita-ot.org/dev/>
- 扩展点清单：<https://www.dita-ot.org/dev/extension-points/plugin-extension-points>
- 插件注册表：<https://www.dita-ot.org/plugins>

---

## 来源

每篇笔记末尾都有独立的「来源」小节，区分**已逐页核对**与**来自通用实践**两类。本页的版本事实来自：

- [DITA 规范列表](https://www.dita-lang.org/specifications) — DITA 2.0 列为 draft；1.3/1.2/1.1/1.0 为 OASIS Standard
- [oasis-tcs/dita 发布页](https://github.com/oasis-tcs/dita/releases) — **v2.0-beta03（2026-07-02）**、beta02（2024-10-04）、beta01（2024-06-14），均为 pre-release，未标注 OASIS 正式阶段
- [DITA-OT 发布说明](https://www.dita-ot.org/dev/release-notes/) — **DITA-OT 4.4，2026-01-31，要求 Java 17+**
- [DITA 2.0 preview 支持](https://www.dita-ot.org/dev/reference/dita-v2-0-support.html) — DITA-OT 4.4 基于 **2026-01-25** 的 2.0 草案语法文件（"基线比 beta03 旧约 5 个月"的依据）
- [DTD 公共标识符](https://dita-lang.org/dita/non-normative/dtd-public-identifiers) — DOCTYPE 公共标识符格式与版本号规则
- [OASIS DITA TC](https://www.oasis-open.org/committees/tc_home.php?wg_abbrev=dita) — 1.3 及以前各版本的正式批准日期
- 四个招牌特性分别核对自 [keytext](https://dita-lang.org/dita/langref/base/keytext)、[titlealt](https://dita-lang.org/dita/langref/base/titlealt)、[include](https://dita-lang.org/dita/langref/base/include)、[chunk](https://dita-lang.org/dita/archspec/base/chunk-attribute-overview)

**未核对**：`dita init` 的模板名称（故正文写作 `--template=<模板名>`，请先跑 `dita init --list`）。

**覆盖情况**：01–08 为操作层，09–11 为架构理论层（shell / 扩展模块 / 一致性 / 键空间 / 分支过滤交互 / 属性有效值 / 元数据级联 / TOC / 索引 / 链接生成 / 排序），12 为原理，13 为翻译与本地化，14 起为最佳实践层（14 元数据与分类策略、15 DITA 与 RAG）。**剩余缺口**（无障碍——可选仅记录、大规模工程、生态）与**验证规划**的完整清单见 [08-practical-advice.md](notes/08-practical-advice.md)。


## 调研待办

- CCMS 预览对标（Paligo / Heretto / FontoXML）——为 `dita-tools preview` 的设计做参照（能力地图见 [docs/architecture.md](../docs/architecture.md) §四）。
