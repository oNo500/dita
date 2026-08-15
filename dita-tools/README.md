# dita-tools

用 Rust 编写的 DITA 创作与分析工具集，作为 [`kb`](../kb) 知识库的配套工具链。

工程范式参照 [OXC](https://github.com/oxc-project/oxc) monorepo。

> **边界**：本工具是三层链条的执行层——上游 [`docs/`](../docs/README.md) 定规则、[`kb/`](../kb/README.md) 定合法值与内容，本工具只负责把规则跑起来，**不自带值集、不重新定义规则语义**。契约与规则归属见 [架构与边界](../docs/架构与边界.md)。
>
> **当前状态**（2026-08-15，均在 kb 上实测）：`dita-tools ia` 可出知识树、按分支概览、维度覆盖与盲区、孤儿与诊断。map 与 topic 两层解析器都在，受控值直接读 `subjectScheme`。未做：Keyref / Conref 等预处理，以及页面渲染。

## 为什么做这个

[DITA-OT](https://github.com/dita-ot/dita-ot) 是强大的发布引擎，但没有 IA（信息架构）视角——它无法告诉你知识树长什么样、哪些域是空的、哪些 Topic 是孤儿。Heretto、Paligo 等商业 CCMS 通过自己的解析引擎解决了这个问题，完全绕开 DITA-OT 来处理创作层的关切。

本项目用 Rust 构建这套能力，从 IA 全景视图出发，逐步演进为支持 Web 编辑器实时预览的完整 DITA 预处理引擎。

## 使用

```bash
# 装到 PATH 上（~/.cargo/bin），任何目录可调用
cargo install --path apps/dita_cli

# 默认参数全是相对路径，所以在 kb 目录下跑
cd ../kb && dita-tools ia

# 或显式给路径，在哪跑都行
dita-tools ia --map ../kb/maps/root.ditamap --topics ../kb/topics \
              --maps-dir ../kb/maps --vocab ../kb/vocab/subjectScheme.ditamap

# 只编译不安装（产物在 target/debug/dita-tools）
cargo build
```

> **改了代码要重新 `cargo install`**，否则 PATH 上跑的仍是旧二进制——这点最容易踩。

| 参数 | 默认 | 作用 |
|---|---|---|
| `--map` | `maps/root.ditamap` | 要渲染成树的 map，可重复 |
| `--topics` | `topics` | topic 根目录，孤儿检测的扫描范围 |
| `--maps-dir` | `maps` | 判定孤儿时**参考的全部 map**——交付物 map 引用的 topic 不算孤儿 |
| `--root-only` | 关 | 只按 `--map` 判定孤儿，忽略 `--maps-dir` |
| `--vocab` | `vocab/subjectScheme.ditamap` | 受控值来源。读不到就**跳过值检查并说明**，绝不猜一份合法值清单 |

**实际输出（`kb` 仓库，2026-08-15 实跑）：**

```

== 知识树（IA 视角）==
按 map 声明的结构展开，看的是「组织成什么样」而非「发布成什么样」——
空分支在发布产物里不存在，这里保留可见。
图例：[n] 该节点下的 topic 数 · [空] 分支已建但无内容 · ✓/✗ topic 文件在/缺失 · ◦ 不进导航的资源

知识体系  ← 根 map：root.ditamap
├── ◦ subjectScheme.ditamap（resource-only，不进导航）
├── [空] 语言本体
├── [1] Web 技术栈
│   └── ✓ electron-landscape.dita
├── [空] 数据存储
│   ...（其余分支从略）

── 按分支 ──
  每个分支手上有什么，用来决定下一批写哪里。「· 无全景」= 该分支尚无声明维度清单的全景 topic。
  AI          3 篇   类型 concept 3   成熟度 curated 3   时效 stable 2 / volatile 1   · 无全景
  Web 技术栈  1 篇   类型 concept 1   成熟度 draft 1   时效 volatile 1
  基础       空
  ...

── 维度覆盖（按技术域，取自各 topic 声明的 domain）──
  ...
```

**看这份输出该注意什么：**

- `[空]` 与「按分支」是这个工具存在的理由——九个分支里七个是空的、三个有内容的没有全景、唯一一篇 Web 内容还是 draft。这些在 DITA-OT 的任何产物里都看不到（空 map 不产出页面）。
- **两种"域"不是一回事**：「按分支」用的是 map 结构推出的分支（`web`），「维度覆盖」用的是 topic 自己声明的技术域（`electron`）。`planned-dimension` 按技术域声明，一个分支下可以有多个技术域，按分支合并算覆盖度会把多份规划混成一份。
- 孤儿判定默认参考 `maps/` 下**全部** map。若只按根 map 判，`agent-rules-core` 这类只挂在交付物 map 上的 topic 会被误报成孤儿。
- 「无全景」是标注不是告警：术语库这类纯组织分支本就不该有全景。

## 开发

```bash
# 安装任务运行器
cargo install just

just build    # 编译全部 crate
just test     # 运行所有测试
just lint     # clippy 检查
just ready    # fmt + lint + test，提交前运行
```

## 架构

```
dita-tools/
├── crates/
│   ├── dita_ast/          # 核心 AST 类型：DitaMap、TopicRef、MapRef、TopicHead、TopicMeta
│   ├── dita_diagnostics/  # 错误/警告报告
│   ├── dita_parser/       # XML → AST：map（保留 mapref 为节点、环检测）与 topic（元数据）
│   ├── dita_vocab/        # 读 subjectScheme：受控值的唯一来源，Rust 里不内联任何值集
│   └── dita_ia/           # IA 视图：知识树、分支统计、维度覆盖、孤儿、一致性检查
└── apps/
    └── dita_cli/          # `dita-tools` 二进制（clap CLI）
```

所有工具共享同一套 `dita_ast` 和 `dita_parser`——解析逻辑只写一次，处处复用。

### Crate 依赖图

```
dita_ast  ←──────────────────────────┐
    ↑                                │
dita_diagnostics ←───────────────┐   │
    ↑                            │   │
dita_parser（roxmltree）   dita_vocab │
    ↑                            ↑   │
dita_ia ─────────────────────────┴───┘
    ↑
dita_cli（clap）
```

### 与 DITA-OT 的关系

DITA-OT 仍然是**发布引擎**——最终输出 HTML/PDF 时作为黑盒命令行调用。本项目负责 DITA-OT 不提供的**创作与分析层**。

`dita_parser` 的 mapref 解析逻辑参照 DITA-OT 的 `MaprefModule.java`（197 行），并遵循同样的处理顺序（一处刻意的差异：被引 map 保留为自己的节点，不并入父级，否则空领域会消失）：

```
Mapref 展开 → Key Space 构建 → DITAVAL 过滤 → Conref 展开 → Topicpull
```

## 路线图

2026-08-15 按「IA 视角优先」重排（原路线把 IA 当作已完成、直奔预处理引擎）。理由见
[架构与边界](../docs/架构与边界.md) §八。

| 阶段 | 内容 | 状态 |
|---|---|---|
| map 层 IA | 知识树、空分支、孤儿、诊断 | ✅ |
| topic 解析 + 词表 | `TopicMeta` 生产者、受控值直读 `subjectScheme` | ✅ |
| IA 深化 | 按分支统计、维度覆盖与盲区、非法值检测 | ✅ |
| `--format json` | 供页面渲染与 `kb/scripts/` 消费 | 可选，未做 |
| 页面渲染 | 使用者 / 作者视角 | 未做，与 `kb/scripts/preview.sh` 的分工待定 |
| Key Space / Keyref | `KeyrefModule.java` ~3600 行 | 顺延 |
| Conref 展开 | `conrefImpl.xsl` ~1500 行 XSLT | 顺延 |
| napi / Wasm | 有 Web 编辑器需求时再说 | 顺延 |

R11（`@dimension` 枚举校验）的**能力**已具备（IA 视图会报非法值），但它是否取代
`check-rules.xsl` 属治理决策，仍是[架构与边界](../docs/架构与边界.md)的待定项。

实现计划：[topic 解析器与 IA 深化](docs/plans/2026-08-15-topic-parser-and-ia-depth.md)、
[总架构（含规格更正）](docs/plans/2026-08-12-dita-tools-architecture.md)。

## 测试

每个 crate 均有单元测试和集成测试，测试 fixture 放在 `tests/fixtures/` 下。

正确性通过**差分测试**与 DITA-OT 的中间输出对比来验证：

```bash
# 生成 DITA-OT 黄金数据（保留临时目录）
dita -f html5 \
  --input=../kb/maps/root.ditamap \
  --clean.temp=no \
  --temp=./tmp/dita-ot-golden \
  -o /dev/null

# 与 Rust 引擎输出逐阶段对比
diff ./tmp/dita-ot-golden/mapref-expanded.xml ./tmp/rust-engine/mapref-expanded.xml
```
