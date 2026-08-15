# dita-tools

用 Rust 编写的 DITA 创作与分析工具集，作为 [`kb`](../kb) 知识库的配套工具链。

工程范式参照 [OXC](https://github.com/oxc-project/oxc) monorepo。

> **边界**：本工具是三层链条的执行层——上游 [`docs/`](../docs/README.md) 定规则、[`kb/`](../kb/README.md) 定合法值与内容，本工具只负责把规则跑起来，**不自带值集、不重新定义规则语义**。契约与规则归属见 [架构与边界](../docs/架构与边界.md)。
>
> **当前状态**：`dita-tools ia` 已可用（2026-08-15 在 kb 上实测，`cargo test` 全过）；尚无 topic 解析器，因此读不到 `@dimension`/`@maturity`，还接不上业务规则。

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

**实际输出（`kb` 仓库）：**

```
== 知识树（IA 视角）==

知识体系 (root)
├── ◦ subjectScheme.ditamap（resource-only，不进导航）
├── [空] 语言本体
├── [空] Web 技术栈
├── [空] 数据存储
├── [空] 网络协议
├── [空] 安全
├── [3] AI
│   └── [3] AI
│       ├── ✓ agent-rule-loading.dita
│       └── ...
├── [空] 工程化
├── [空] 基础
├── [4] 知识工程
│   └── [4] 知识工程
│       └── [4] 写作规则
│           └── ...
└── [12] 术语库
    └── ...

── 孤儿判定：参考了 12 个 map ──
⚠  孤儿 Topic（未被任何 map 引用，共 1 个）：
   web/electron-landscape.dita
```

**看这份输出该注意什么：**

- `[空]` 是这个工具存在的理由——九个领域里七个是空的，这件事在 DITA-OT 的任何输出里都看不到（空 map 不产出页面）。
- 树里 `AI → AI`、`知识工程 → 知识工程` 的重复层级不是 bug，是 `root.ditamap` 里 `topichead` 包了一层同名的 `mapref`。工具照实显示源结构；要消掉得改 kb 的 map，不是改工具。
- 孤儿判定默认参考 `maps/` 下**全部** map。若只按根 map 判，`agent-rules-core` 这类只挂在交付物 map 上的 topic 会被误报成孤儿。

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
│   ├── dita_ast/          # 核心 AST 类型：DitaMap、TopicRef、MapRef、TopicHead
│   ├── dita_diagnostics/  # 错误/警告报告
│   ├── dita_parser/       # XML → AST 解析器，递归解析 mapref（保留为节点），环检测
│   └── dita_ia/           # IA 视图：知识树 + 孤儿 Topic 检测
└── apps/
    └── dita_cli/          # `dita-tools` 二进制（clap CLI）
```

所有工具共享同一套 `dita_ast` 和 `dita_parser`——解析逻辑只写一次，处处复用。

### Crate 依赖图

```
dita_ast  ←────────────────────────┐
    ↑                              │
dita_diagnostics                   │
    ↑                              │
dita_parser（roxmltree）            │
    ↑                              │
dita_ia ───────────────────────────┘
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

| 阶段 | Crate | 内容 | 参考 |
|---|---|---|---|
| ✅ Phase 1 | `dita_ia` | IA 全景：知识树 + 孤儿 Topic | — |
| Phase 2 | `dita_validate` | @dimension 枚举值校验（R11） | `check-rules.xsl` 的 Rust 替代 |
| Phase 3 | `dita_preprocess` | Keyref / Key Space 引擎 | `KeyrefModule.java`（~3600 行）|
| Phase 4 | `dita_preprocess` | Conref 展开 | `conrefImpl.xsl`（~1500 行 XSLT）|
| Phase 5 | `napi/` | Node.js 绑定（napi-rs） | OXC napi 层 |
| Phase 6 | `wasm/` | 浏览器端实时解析 | OXC playground 架构 |

完整实现计划见 [`docs/plans/2026-08-12-dita-tools-architecture.md`](docs/plans/2026-08-12-dita-tools-architecture.md)。

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
