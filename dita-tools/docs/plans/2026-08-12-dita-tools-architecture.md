# dita-tools 架构与实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一套 Rust monorepo DITA 工具链，从 IA 视图出发，逐步演进为完整的 DITA 预处理引擎，最终支持 Web 编辑器的实时解析能力。

**Architecture:** 参照 OXC（oxc-project/oxc）的 monorepo 工程范式，以 `dita_ast`（共享 AST 类型）和 `dita_parser`（XML→AST）为核心基础，所有上层工具（IA 分析、校验、预处理）共享同一套解析层。发布端继续封装 DITA-OT 命令行（黑盒调用），编辑/分析端由 Rust 引擎承担。

**Tech Stack:** Rust 2024 edition · roxmltree（XML DOM 解析）· quick-xml（流式解析，未来 conref 用）· clap（CLI）· insta（快照测试）· napi-rs（未来 Node.js 绑定）

## Global Constraints

- Rust edition = 2024，MSRV = 1.85.0
- workspace resolver = "3"
- 所有 crate 名用下划线：`dita_ast`、`dita_parser` 等
- 测试使用 `insta` 快照测试，黄金数据放 `tests/snapshots/`
- 错误类型：每个 crate 用 `thiserror` 定义自己的 `Error` enum，对外 `anyhow::Result` 透传
- 不依赖 DITA-OT 做任何解析（发布时作为黑盒命令调用，不作为库依赖）
- 所有路径处理用 `std::path::PathBuf`，不用裸字符串

---

## 背景：为什么做这个项目

### 问题起点

`kb`（`~/code/kb`）是一个用 DITA 2.0 组织的个人知识库。DITA-OT 是官方发布引擎，但它：

1. **没有 IA 视角**：看不到整个知识树长什么样、哪些域是空的、哪些 Topic 是孤儿
2. **无法实时预览**：基于 JVM 的批处理管道，启动开销 2-5 秒，不支持编辑时实时渲染
3. **工具链割裂**：治理脚本（Python）、发布（DITA-OT Java）、编辑（Oxygen 商业软件）互不相通

### 参照系

- **Heretto**：DITA 原生 CCMS，内部有自己的解析引擎（非 DITA-OT），支持实时编辑
- **Paligo**：现代 block-based UI，WYSIWYG 编辑体验
- **OXC**：用 Rust 构建 JS 工具链的最佳 monorepo 工程范式
- **DITA-OT 源码**：预处理管道逻辑的权威参考实现（Java + XSLT，约 40,000-60,000 行），借鉴处理顺序和边界条件，不直接复用代码

### 关键架构分层

```
编辑/分析层（Rust 引擎）         发布层（DITA-OT 黑盒）
─────────────────────────        ──────────────────────
dita_parser → dita_ast           dita -f html5 -i root.ditamap
dita_ia（IA 视图）                dita -f pdf
dita_validate（规则校验）          dita -f markdown_github
dita_preprocess（未来）
  → Mapref 展开
  → Keyref 解析（★★★★★）
  → Conref 展开（借鉴 conrefImpl.xsl 1500行 XSLT）
```

DITA-OT 预处理管道的绝对顺序（不可违背）：
```
Mapref 展开 → Key Space 构建 → DITAVAL 过滤 → Conref 展开 → Topicpull
```

---

## Monorepo 目录结构

```
dita-tools/
├── Cargo.toml              # workspace（参照 OXC，resolver = "3"）
├── Justfile                # just build / just test / just ready
├── rust-toolchain.toml
├── .gitignore
│
├── crates/
│   ├── dita_ast/           # Task 1：核心 AST 类型（所有工具共享）
│   ├── dita_diagnostics/   # Task 2：错误/警告类型
│   ├── dita_parser/        # Task 3：XML → AST（递归展开 mapref）
│   └── dita_ia/            # Task 4：IA 视图分析
│
├── apps/
│   └── dita_cli/           # Task 5：CLI 入口（dita-tools ia）
│
├── napi/                   # 未来：Node.js 绑定
├── tasks/                  # 构建辅助脚本（暂空）
└── docs/plans/             # 本文件所在
```

---

## 实现计划

### Task 1：dita_ast — 核心 AST 类型

**Files:**
- Create: `crates/dita_ast/Cargo.toml`
- Create: `crates/dita_ast/src/lib.rs`
- Create: `crates/dita_ast/src/map.rs`
- Create: `crates/dita_ast/src/topic.rs`

**Interfaces — Produces:**
- `DitaMap { title: String, path: PathBuf, lang: Option<String>, children: Vec<MapNode> }`
- `enum MapNode { TopicRef(TopicRef), MapRef(MapRef), TopicHead(TopicHead) }`
- `TopicRef { href: PathBuf, nav_title: Option<String> }`
- `MapRef { href: PathBuf, processing_role: ProcessingRole }`
- `TopicHead { nav_title: String, children: Vec<MapNode> }`
- `enum ProcessingRole { Normal, ResourceOnly }`
- `TopicMeta { path, id, title, topic_type, maturity, volatility, dimension: Vec<String>, domain }`
- `enum TopicType { Concept, Reference, Task, Troubleshooting, GlossEntry, Unknown }`

- [ ] **Step 1：创建目录和 Cargo.toml**

```bash
mkdir -p crates/dita_ast/src
```

```toml
# crates/dita_ast/Cargo.toml
[package]
name = "dita_ast"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
```

- [ ] **Step 2：写 map.rs**

```rust
// crates/dita_ast/src/map.rs
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct DitaMap {
    pub title: String,
    pub path: PathBuf,
    pub lang: Option<String>,
    pub children: Vec<MapNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapNode {
    TopicRef(TopicRef),
    MapRef(MapRef),
    TopicHead(TopicHead),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopicRef {
    pub href: PathBuf,
    pub nav_title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapRef {
    pub href: PathBuf,
    pub processing_role: ProcessingRole,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProcessingRole {
    #[default]
    Normal,
    ResourceOnly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopicHead {
    pub nav_title: String,
    pub children: Vec<MapNode>,
}
```

- [ ] **Step 3：写 topic.rs**

```rust
// crates/dita_ast/src/topic.rs
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct TopicMeta {
    pub path: PathBuf,
    pub id: String,
    pub title: String,
    pub topic_type: TopicType,
    pub maturity: Option<String>,
    pub volatility: Option<String>,
    pub dimension: Vec<String>,
    pub domain: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopicType {
    Concept, Reference, Task, Troubleshooting, GlossEntry, Unknown,
}
```

- [ ] **Step 4：写 lib.rs + 内联测试**

```rust
// crates/dita_ast/src/lib.rs
pub mod map;
pub mod topic;

pub use map::{DitaMap, MapNode, MapRef, ProcessingRole, TopicHead, TopicRef};
pub use topic::{TopicMeta, TopicType};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn map_node_holds_topic_ref() {
        let node = MapNode::TopicRef(TopicRef {
            href: PathBuf::from("topics/foo.dita"),
            nav_title: None,
        });
        assert!(matches!(node, MapNode::TopicRef(_)));
    }

    #[test]
    fn processing_role_defaults_to_normal() {
        assert_eq!(ProcessingRole::default(), ProcessingRole::Normal);
    }
}
```

- [ ] **Step 5：运行测试**

```bash
cargo test -p dita_ast
```
期望：2 tests passed

- [ ] **Step 6：Commit**

```bash
git add crates/dita_ast
git commit -m "feat(dita_ast): add core AST types for Map, TopicRef, MapRef, TopicHead"
```

---

### Task 2：dita_diagnostics — 错误报告类型

**Files:**
- Create: `crates/dita_diagnostics/Cargo.toml`
- Create: `crates/dita_diagnostics/src/lib.rs`

**Interfaces — Produces:**
- `Diagnostic::error(path, message) -> Diagnostic`
- `Diagnostic::warning(path, message) -> Diagnostic`
- `Diagnostic::is_error(&self) -> bool`
- `DiagnosticBag::push(&mut self, Diagnostic)`
- `DiagnosticBag::has_errors(&self) -> bool`

- [ ] **Step 1：Cargo.toml**

```toml
# crates/dita_diagnostics/Cargo.toml
[package]
name = "dita_diagnostics"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
thiserror.workspace = true
```

- [ ] **Step 2：实现 lib.rs**

```rust
// crates/dita_diagnostics/src/lib.rs
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DiagError   { pub path: PathBuf, pub message: String }
#[derive(Debug, Clone)]
pub struct DiagWarning { pub path: PathBuf, pub message: String }

#[derive(Debug, Clone)]
pub enum Diagnostic {
    Error(DiagError),
    Warning(DiagWarning),
}

impl Diagnostic {
    pub fn error(path: impl Into<PathBuf>, msg: impl Into<String>) -> Self {
        Self::Error(DiagError { path: path.into(), message: msg.into() })
    }
    pub fn warning(path: impl Into<PathBuf>, msg: impl Into<String>) -> Self {
        Self::Warning(DiagWarning { path: path.into(), message: msg.into() })
    }
    pub fn is_error(&self) -> bool { matches!(self, Self::Error(_)) }
}

#[derive(Debug, Default)]
pub struct DiagnosticBag { pub items: Vec<Diagnostic> }

impl DiagnosticBag {
    pub fn push(&mut self, d: Diagnostic) { self.items.push(d); }
    pub fn has_errors(&self) -> bool { self.items.iter().any(Diagnostic::is_error) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bag_detects_errors() {
        let mut bag = DiagnosticBag::default();
        bag.push(Diagnostic::warning("a.dita", "unused"));
        assert!(!bag.has_errors());
        bag.push(Diagnostic::error("b.dita", "broken ref"));
        assert!(bag.has_errors());
    }
}
```

- [ ] **Step 3：测试 + Commit**

```bash
cargo test -p dita_diagnostics
git add crates/dita_diagnostics
git commit -m "feat(dita_diagnostics): add Diagnostic and DiagnosticBag types"
```

---

### Task 3：dita_parser — Map 解析器

核心模块。参照 DITA-OT `MaprefModule.java`（197 行）的处理逻辑：递归展开 `<mapref>`，在内存中完成，无需临时文件。处理循环引用检测（对应 DITA-OT 的环检测逻辑）。

**Files:**
- Create: `crates/dita_parser/Cargo.toml`
- Create: `crates/dita_parser/src/lib.rs`
- Create: `crates/dita_parser/src/map_parser.rs`
- Create: `crates/dita_parser/tests/fixtures/simple.ditamap`
- Create: `crates/dita_parser/tests/fixtures/sub.ditamap`
- Create: `crates/dita_parser/tests/parse_map.rs`

**Interfaces:**
- Consumes: `dita_ast::*`, `dita_diagnostics::*`
- Produces: `pub fn parse_map(path: &Path) -> anyhow::Result<(DitaMap, DiagnosticBag)>`

- [ ] **Step 1：Cargo.toml**

```toml
# crates/dita_parser/Cargo.toml
[package]
name = "dita_parser"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
dita_ast.workspace = true
dita_diagnostics.workspace = true
roxmltree.workspace = true
anyhow.workspace = true

[dev-dependencies]
insta.workspace = true
```

- [ ] **Step 2：写 test fixtures**

```xml
<!-- crates/dita_parser/tests/fixtures/simple.ditamap -->
<?xml version="1.0" encoding="UTF-8"?>
<map xml:lang="zh-CN">
  <title>测试知识体系</title>
  <mapref href="sub.ditamap"/>
  <topicref href="topics/foo.dita"/>
</map>
```

```xml
<!-- crates/dita_parser/tests/fixtures/sub.ditamap -->
<?xml version="1.0" encoding="UTF-8"?>
<map>
  <title>子 Map</title>
  <topicref href="../topics/bar.dita"/>
</map>
```

- [ ] **Step 3：写失败的集成测试**

```rust
// crates/dita_parser/tests/parse_map.rs
use std::path::Path;
use dita_parser::parse_map;

#[test]
fn parses_title_and_expands_mapref() {
    let fixture = Path::new("tests/fixtures/simple.ditamap");
    let (map, diag) = parse_map(fixture).expect("parse failed");
    assert_eq!(map.title, "测试知识体系");
    // sub.ditamap 展开后有 bar.dita，加上直接的 foo.dita = 2 个 TopicRef
    assert_eq!(map.children.len(), 2);
    assert!(!diag.has_errors());
}

#[test]
fn detects_missing_topic_file() {
    let fixture = Path::new("tests/fixtures/simple.ditamap");
    let (_map, diag) = parse_map(fixture).expect("parse failed");
    // foo.dita 和 bar.dita 都不存在，应该有 error
    assert!(diag.has_errors());
}
```

- [ ] **Step 4：确认测试失败（compile error）**

```bash
cargo test -p dita_parser 2>&1 | head -5
```

- [ ] **Step 5：实现 map_parser.rs**

```rust
// crates/dita_parser/src/map_parser.rs
use std::{collections::HashSet, fs, path::{Path, PathBuf}};
use anyhow::Context;
use dita_ast::{DitaMap, MapNode, MapRef, ProcessingRole, TopicHead, TopicRef};
use dita_diagnostics::{Diagnostic, DiagnosticBag};

pub fn parse_map(path: &Path) -> anyhow::Result<(DitaMap, DiagnosticBag)> {
    let mut diag = DiagnosticBag::default();
    let mut visited = HashSet::new();
    let map = parse_map_file(path, &mut visited, &mut diag)?;
    Ok((map, diag))
}

fn parse_map_file(
    path: &Path,
    visited: &mut HashSet<PathBuf>,
    diag: &mut DiagnosticBag,
) -> anyhow::Result<DitaMap> {
    let canonical = path.canonicalize()
        .with_context(|| format!("cannot resolve: {}", path.display()))?;

    // 循环引用检测
    if !visited.insert(canonical.clone()) {
        diag.push(Diagnostic::error(&canonical, "circular mapref detected"));
        return Ok(DitaMap { title: String::new(), path: canonical, lang: None, children: vec![] });
    }

    let base = canonical.parent().unwrap_or(Path::new(".")).to_path_buf();
    let xml = fs::read_to_string(&canonical)
        .with_context(|| format!("cannot read: {}", canonical.display()))?;
    let doc = roxmltree::Document::parse(&xml)
        .with_context(|| format!("XML parse error: {}", canonical.display()))?;

    let root = doc.root_element();
    let title = root.children()
        .find(|n| n.has_tag_name("title"))
        .and_then(|n| n.text())
        .unwrap_or("")
        .to_string();
    let lang = root.attribute("xml:lang").map(str::to_string);
    let children = collect_children(root, &base, visited, diag);

    Ok(DitaMap { title, path: canonical, lang, children })
}

fn collect_children(
    node: roxmltree::Node,
    base: &Path,
    visited: &mut HashSet<PathBuf>,
    diag: &mut DiagnosticBag,
) -> Vec<MapNode> {
    let mut result = Vec::new();
    for child in node.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "mapref" => {
                let Some(href_str) = child.attribute("href") else {
                    diag.push(Diagnostic::warning(base, "mapref missing href"));
                    continue;
                };
                let href = base.join(href_str);
                let role = match child.attribute("processing-role") {
                    Some("resource-only") => ProcessingRole::ResourceOnly,
                    _ => ProcessingRole::Normal,
                };
                if role == ProcessingRole::ResourceOnly {
                    result.push(MapNode::MapRef(MapRef { href, processing_role: role }));
                    continue;
                }
                match parse_map_file(&href, visited, diag) {
                    Ok(sub) => result.extend(sub.children),
                    Err(e) => diag.push(Diagnostic::error(&href, e.to_string())),
                }
            }
            "topicref" => {
                if let Some(href_str) = child.attribute("href") {
                    let href = base.join(href_str);
                    let nav_title = nav_title_from(&child);
                    if !href.exists() {
                        diag.push(Diagnostic::error(&href, "referenced file not found"));
                    }
                    result.push(MapNode::TopicRef(TopicRef { href, nav_title }));
                }
            }
            "topichead" => {
                let nav_title = nav_title_from(&child).unwrap_or_else(|| "(unnamed)".into());
                let children = collect_children(child, base, visited, diag);
                result.push(MapNode::TopicHead(TopicHead { nav_title, children }));
            }
            "title" => {}
            _ => {}
        }
    }
    result
}

fn nav_title_from(node: &roxmltree::Node) -> Option<String> {
    node.children()
        .find(|n| n.has_tag_name("topicmeta"))?
        .children()
        .find(|n| n.has_tag_name("navtitle"))?
        .text()
        .map(str::to_string)
}
```

- [ ] **Step 6：写 lib.rs**

```rust
// crates/dita_parser/src/lib.rs
mod map_parser;
pub use map_parser::parse_map;
```

- [ ] **Step 7：运行测试**

```bash
cargo test -p dita_parser
```

- [ ] **Step 8：Commit**

```bash
git add crates/dita_parser
git commit -m "feat(dita_parser): implement map parser with recursive mapref expansion and cycle detection"
```

---

### Task 4：dita_ia — IA 视图分析

**Files:**
- Create: `crates/dita_ia/Cargo.toml`
- Create: `crates/dita_ia/src/lib.rs`
- Create: `crates/dita_ia/src/tree.rs`
- Create: `crates/dita_ia/src/orphan.rs`

**Interfaces:**
- Consumes: `dita_parser::parse_map`, `dita_ast::DitaMap`
- Produces:
  - `pub fn build_report(map_path: &Path, topics_root: &Path) -> anyhow::Result<IaReport>`
  - `pub fn print_report(report: &IaReport)`
  - `struct IaReport { map: DitaMap, diagnostics: DiagnosticBag, orphans: Vec<PathBuf>, topics_root: PathBuf }`

- [ ] **Step 1：Cargo.toml**

```toml
# crates/dita_ia/Cargo.toml
[package]
name = "dita_ia"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
dita_ast.workspace = true
dita_parser.workspace = true
dita_diagnostics.workspace = true
anyhow.workspace = true
owo-colors.workspace = true
```

- [ ] **Step 2：写 tree.rs**

```rust
// crates/dita_ia/src/tree.rs
use dita_ast::{DitaMap, MapNode};
use std::path::Path;

pub fn print_tree(map: &DitaMap, topics_root: &Path) {
    println!("{} (root)", map.title);
    print_nodes(&map.children, "", topics_root);
}

fn print_nodes(nodes: &[MapNode], prefix: &str, topics_root: &Path) {
    let count = nodes.len();
    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == count - 1;
        let conn = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };
        match node {
            MapNode::TopicRef(t) => {
                let name = t.href.file_name()
                    .and_then(|s| s.to_str()).unwrap_or("?");
                let marker = if t.href.exists() { "✓" } else { "✗" };
                println!("{prefix}{conn}{marker} {name}");
            }
            MapNode::TopicHead(h) => {
                let n = count_topics(&h.children);
                let label = if n == 0 {
                    format!("[空] {}", h.nav_title)
                } else {
                    format!("[{n}] {}", h.nav_title)
                };
                println!("{prefix}{conn}{label}");
                print_nodes(&h.children, &child_prefix, topics_root);
            }
            MapNode::MapRef(_) => {} // resource-only, skip
        }
    }
}

pub fn count_topics(nodes: &[MapNode]) -> usize {
    nodes.iter().map(|n| match n {
        MapNode::TopicRef(_) => 1,
        MapNode::TopicHead(h) => count_topics(&h.children),
        MapNode::MapRef(_) => 0,
    }).sum()
}
```

- [ ] **Step 3：写 orphan.rs**

```rust
// crates/dita_ia/src/orphan.rs
use dita_ast::{DitaMap, MapNode};
use std::{collections::HashSet, path::{Path, PathBuf}};

pub fn find_orphans(map: &DitaMap, topics_root: &Path) -> Vec<PathBuf> {
    let referenced = collect_referenced(map);
    let mut all = walkdir(topics_root);
    all.retain(|p| !referenced.contains(p));
    all
}

fn collect_referenced(map: &DitaMap) -> HashSet<PathBuf> {
    let mut set = HashSet::new();
    collect_from_nodes(&map.children, &mut set);
    set
}

fn collect_from_nodes(nodes: &[MapNode], set: &mut HashSet<PathBuf>) {
    for node in nodes {
        match node {
            MapNode::TopicRef(t) => {
                if let Ok(c) = t.href.canonicalize() { set.insert(c); }
            }
            MapNode::TopicHead(h) => collect_from_nodes(&h.children, set),
            MapNode::MapRef(_) => {}
        }
    }
}

fn walkdir(root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() { result.extend(walkdir(&p)); }
            else if p.extension().and_then(|e| e.to_str()) == Some("dita") {
                if let Ok(c) = p.canonicalize() { result.push(c); }
            }
        }
    }
    result.sort();
    result
}
```

- [ ] **Step 4：写 lib.rs**

```rust
// crates/dita_ia/src/lib.rs
mod orphan;
mod tree;

use anyhow::Result;
use dita_ast::DitaMap;
use dita_diagnostics::DiagnosticBag;
use dita_parser::parse_map;
use std::path::{Path, PathBuf};

pub use tree::count_topics;

pub struct IaReport {
    pub map: DitaMap,
    pub diagnostics: DiagnosticBag,
    pub orphans: Vec<PathBuf>,
    pub topics_root: PathBuf,
}

pub fn build_report(map_path: &Path, topics_root: &Path) -> Result<IaReport> {
    let (map, diagnostics) = parse_map(map_path)?;
    let orphans = orphan::find_orphans(&map, topics_root);
    Ok(IaReport { map, diagnostics, orphans, topics_root: topics_root.to_path_buf() })
}

pub fn print_report(report: &IaReport) {
    println!("\n== 知识树（IA 视角）==\n");
    tree::print_tree(&report.map, &report.topics_root);

    if !report.orphans.is_empty() {
        println!("\n⚠ 孤儿 Topic（未被任何 Map 引用，共 {} 个）：", report.orphans.len());
        for p in &report.orphans {
            let rel = p.strip_prefix(&report.topics_root).unwrap_or(p);
            println!("  topics/{}", rel.display());
        }
    }

    if report.diagnostics.has_errors() {
        println!("\n❌ 发现错误：");
        for d in &report.diagnostics.items {
            if d.is_error() { println!("  {d:?}"); }
        }
    }
}
```

- [ ] **Step 5：测试 + Commit**

```bash
cargo test -p dita_ia
git add crates/dita_ia
git commit -m "feat(dita_ia): add IA tree printer and orphan topic detector"
```

---

### Task 5：dita_cli — CLI 入口

**Files:**
- Create: `apps/dita_cli/Cargo.toml`
- Create: `apps/dita_cli/src/main.rs`
- Create: `apps/dita_cli/src/commands/mod.rs`
- Create: `apps/dita_cli/src/commands/ia.rs`

**Interfaces:**
- Produces: `dita-tools ia --map <path> [--topics <path>]`

- [ ] **Step 1：Cargo.toml**

```toml
# apps/dita_cli/Cargo.toml
[package]
name = "dita-tools"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[[bin]]
name = "dita-tools"
path = "src/main.rs"

[dependencies]
dita_ia.workspace = true
clap.workspace = true
anyhow.workspace = true
```

- [ ] **Step 2：main.rs**

```rust
// apps/dita_cli/src/main.rs
mod commands;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dita-tools", about = "DITA authoring and analysis tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show IA overview: knowledge tree, orphan topics, domain stats
    Ia(commands::ia::IaArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Ia(args) => commands::ia::run(args),
    }
}
```

- [ ] **Step 3：commands/ia.rs**

```rust
// apps/dita_cli/src/commands/ia.rs
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct IaArgs {
    /// Path to root.ditamap
    #[arg(long, default_value = "maps/root.ditamap")]
    pub map: PathBuf,
    /// Path to topics root directory
    #[arg(long, default_value = "topics")]
    pub topics: PathBuf,
}

pub fn run(args: IaArgs) -> Result<()> {
    let report = dita_ia::build_report(&args.map, &args.topics)?;
    dita_ia::print_report(&report);
    if report.diagnostics.has_errors() {
        std::process::exit(1);
    }
    Ok(())
}
```

```rust
// apps/dita_cli/src/commands/mod.rs
pub mod ia;
```

- [ ] **Step 4：构建并在 kb 仓库实测**

```bash
cargo build -p dita-tools
./target/debug/dita-tools ia \
  --map /Users/xiu/code/kb/maps/root.ditamap \
  --topics /Users/xiu/code/kb/topics
```

期望输出类似：
```
== 知识树（IA 视角）==

知识体系 (root)
├── [空] lang
├── [1]  web
│   └── ✓ electron-landscape.dita
├── [空] data
...
⚠ 孤儿 Topic（共 N 个）：
  topics/engineering/dita-authoring-guide.dita
```

- [ ] **Step 5：Commit**

```bash
git add apps/dita_cli
git commit -m "feat(dita_cli): add ia subcommand for IA overview"
```

---

## 后续路线图

| Phase | 内容 | 复杂度 | 参考 |
|---|---|---|---|
| Phase 2 | `dita_validate`：R11 @dimension 枚举校验 | ⭐⭐ | `check-rules.xsl` 的 Rust 替代 |
| Phase 3 | Key Space 构建 + Keyref 解析 | ⭐⭐⭐⭐⭐ | `KeyrefModule.java`（~3600 LOC）|
| Phase 4 | Conref 展开 | ⭐⭐⭐⭐⭐ | `conrefImpl.xsl`（1500 行 XSLT）|
| Phase 5 | `napi/`：napi-rs 绑定 | ⭐⭐⭐ | OXC napi 层 |
| Phase 6 | Wasm 编译，浏览器内实时解析 | ⭐⭐⭐ | OXC playground |

## 差分测试策略

用 DITA-OT 输出作为黄金数据，验证 Rust 引擎正确性：

```bash
# 保留 DITA-OT 中间文件作为黄金数据
dita -f html5 \
  --input=~/code/kb/maps/root.ditamap \
  --clean.temp=no \
  --temp=/tmp/kb-golden \
  -o /dev/null

# 对比 Rust 引擎输出
diff /tmp/kb-golden/job.xml /tmp/kb-rust/job.xml
```

## 关键参考

- OXC workspace: `github.com/oxc-project/oxc/blob/main/Cargo.toml`
- DITA-OT MaprefModule.java（197 行）
- DITA-OT KeyrefModule.java（~800 行）
- conrefImpl.xsl（~1500 行 XSLT）
- roxmltree: `docs.rs/roxmltree`
- kb 仓库: `~/code/kb`（本工具的第一个用户）
