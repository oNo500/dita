//! 把一份真实词表读出来打印，用于对着实际文件验证解析结果。
//!
//! fixture 是结构子集，过了不代表真文件能读——这个 example 就是"尽早跑在真实数据上"
//! 的那一步（见 docs/architecture.md §五）。
//!
//! ```sh
//! cargo run -p dita_vocab --example dump_vocab -- ../kb/vocab/subjectScheme.ditamap
//! ```
#![allow(clippy::print_stdout)]

use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let path: PathBuf = std::env::args().nth(1).map_or_else(
        || PathBuf::from("../../../kb/vocab/subjectScheme.ditamap"),
        PathBuf::from,
    );

    let (vocab, diag) = dita_vocab::parse_vocab(&path)?;

    println!("顶层 subject：{}", vocab.subjects.len());
    for s in &vocab.subjects {
        println!("  {} （子树 {} 个键）", s.keys, s.all_keys().len());
    }

    println!("\n受控属性：");
    for attr in vocab.attributes() {
        let e = vocab.enumeration(attr).expect("attribute just listed");
        println!(
            "  @{attr} → subject \"{}\"：合法值 {}（其中叶子 {}），默认 {}",
            e.subject_key,
            e.values.len(),
            e.leaf_values.len(),
            e.default.as_deref().unwrap_or("（无，必须显式标）")
        );
    }

    if diag.error_count() > 0 || diag.warning_count() > 0 {
        println!("\n诊断：");
        for d in &diag.items {
            println!("  {}: {}", d.path().display(), d.message());
        }
    }
    Ok(())
}
