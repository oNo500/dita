//! 把真实 topic 的自我声明读出来打印，用于对着实际文件验证解析结果。
//!
//! fixture 是人造的，真库里才有各种没预料到的写法（标题里嵌标记、prolog 结构差异、
//! 属性缺失）。见 docs/architecture.md §五：尽早跑在真实数据上。
//!
//! ```sh
//! cargo run -p dita_parser --example dump_topics -- ../kb/topics
//! ```
#![allow(clippy::print_stdout)]

use std::path::{Path, PathBuf};

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("dita") {
            out.push(path);
        }
    }
}

fn main() -> anyhow::Result<()> {
    let root: PathBuf = std::env::args()
        .nth(1)
        .map_or_else(|| PathBuf::from("../../../kb/topics"), PathBuf::from);

    let mut paths = Vec::new();
    if root.is_dir() {
        collect(&root, &mut paths);
    } else {
        paths.push(root.clone());
    }
    paths.sort();

    println!("{:<34} {:<14} {:<9} {:<9} 维度 / 规划", "文件", "类型", "成熟度", "时效");
    let mut warnings = 0;
    for path in &paths {
        let (m, diag) = dita_parser::parse_topic(path)?;
        warnings += diag.warning_count();
        let name = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .display()
            .to_string();
        let dims = if m.planned_dimensions.is_empty() {
            m.dimensions.join(" ")
        } else {
            format!("规划 {} 项", m.planned_dimensions.len())
        };
        println!(
            "{:<34} {:<14} {:<9} {:<9} {}",
            name,
            m.topic_type.as_str(),
            m.maturity.as_deref().unwrap_or("—"),
            m.volatility.as_deref().unwrap_or("—"),
            dims
        );
    }
    println!("\n共 {} 篇，解析告警 {} 条", paths.len(), warnings);
    Ok(())
}
