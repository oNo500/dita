# dita-tools

用 Rust 编写的 DITA 创作与分析工具集，作为 [`kb`](../kb) 知识库的配套工具链。

工程范式参照 [OXC](https://github.com/oxc-project/oxc) monorepo。

## 为什么做这个

[DITA-OT](https://github.com/dita-ot/dita-ot) 是强大的发布引擎，但没有 IA（信息架构）视角——它无法告诉你知识树长什么样、哪些域是空的、哪些 Topic 是孤儿。Heretto、Paligo 等商业 CCMS 通过自己的解析引擎解决了这个问题，完全绕开 DITA-OT 来处理创作层的关切。

本项目用 Rust 构建这套能力，从 IA 全景视图出发，逐步演进为支持 Web 编辑器实时预览的完整 DITA 预处理引擎。

## 使用

```bash
# 编译
cargo build -p dita-tools

# IA 全景——展示知识树、孤儿 Topic、诊断信息
./target/debug/dita-tools ia \
  --map /path/to/maps/root.ditamap \
  --topics /path/to/topics/

# 在 kb 仓库根目录下直接运行（使用默认路径）
dita-tools ia
```

**实际输出示例（`kb` 仓库）：**

```
== 知识树（IA 视角）==

知识体系 (root)
├── [3]  AI
│   ├── ✓ agent-rule-loading.dita
│   ├── ✓ agent-skill-orchestration.dita
│   └── ✓ agent-context-verification.dita
├── [4]  知识工程
│   └── [4]  写作规则
│       ├── ✓ writing-atomicity.dita
│       ├── ✓ writing-typing.dita
│       ├── ✓ writing-sourcing.dita
│       └── ✓ writing-llm-friendly.dita
├── ✓ term-context-engineering.dita
└── ...（术语库 12 项）

⚠  孤儿 Topic（未被任何 Map 引用，共 3 个）：
   topics/engineering/agent-rules-core.dita
   topics/engineering/dita-authoring-guide.dita
   topics/web/electron-landscape.dita

✓  无诊断错误
```

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
│   ├── dita_parser/       # XML → AST 解析器，递归展开 mapref，循环引用检测
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

`dita_parser` 的 mapref 展开逻辑参照 DITA-OT 的 `MaprefModule.java`（197 行），并遵循同样的处理顺序：

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
