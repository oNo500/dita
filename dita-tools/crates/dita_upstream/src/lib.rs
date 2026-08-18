//! 上游节点索引：把领域事实标准的节点树取成一张可 grep 的扁平表。
//!
//! 索引是派生物，不是资产——价值不在内容，在于让上游漂移可见：再生成后的 diff
//! 就是"哪几篇要改标题"的工单。设计正本见
//! `docs/superpowers/specs/2026-08-16-upstream-node-index-design.md`。

mod keyspace;
mod walk;

pub use keyspace::KeySpace;

use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
};

/// 索引的一行：一个上游节点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// 来源标识，如 `oasis-archspec` / `dita-ot`
    pub source: String,
    /// 节点标题原文，逐字照抄上游
    pub title: String,
    /// 父节点标题（顶层为空）
    pub parent: String,
    /// 源文件相对路径（相对来源根目录；无文件的导航节点为空）
    pub path: String,
    /// 可访问地址；推不出的留空，不猜
    pub url: String,
}

/// 一个上游来源：从哪张 map 进、路径相对谁算、按什么规律分 source 与推 url。
pub struct Source {
    /// 相对路径基准（tsv 的 path 列相对它）
    pub root: PathBuf,
    /// 入口 map
    pub entry: PathBuf,
    pub flavor: Flavor,
}

/// 来源的"方言"：决定 source 列怎么分、url 列怎么推。
///
/// 两个真实来源各有各的规律，没有通用解；新增分支时照着加一个变体
/// （设计七之二：加分支只加抓取器，不改格式）。
pub enum Flavor {
    /// DITA-OT 的 docsrc。整包一个 source；url 指向 `www.dita-ot.org/<版本>/`。
    DitaOt { version: String },
    /// oasis-tcs/dita 的 specification。按首层目录分 source；url 指向 dita-lang.org。
    Oasis,
    /// 无 url 规律的通用来源（fixture 与将来"只登记不建索引"的分支用）。
    Plain { id: String },
}

/// 生成结果：条目 + 生成过程中值得报出来的事。
pub struct Index {
    pub entries: Vec<Entry>,
    /// 解析不到的键、标题为空的 topic 等——不阻断生成，但要看得见
    pub notes: Vec<String>,
}

/// dita-lang.org 上已核实过 URL 规律的目录。
///
/// 规律是"路径小写、去扩展名"，但只有规范正文的两支整目录成立：
/// `introduction/` 与 `glossary/` 下不少文件是被 conref 引用的片段，
/// 没有独立页面。推不出的留空，不猜——一列半真的 url 比空列更坏。
const OASIS_URL_DIRS: [&str; 4] = ["archSpec/", "langRef/", "conformance/", "non-normative/"];

impl Flavor {
    /// 这条相对路径归哪个 source。
    #[must_use]
    pub fn source_id(&self, rel: &Path) -> String {
        match self {
            Self::DitaOt { .. } => "dita-ot".to_string(),
            Self::Oasis => match rel.iter().nth(1).and_then(|s| s.to_str()) {
                // archSpec 与 langRef 是规范的两大部分，也是声明溯源时最常引的
                // 两处，单独成源；其余（introduction / terminology / conformance /
                // non-normative…）体量小且性质一致，合成 oasis-spec
                Some("archSpec") => "oasis-archspec".to_string(),
                Some("langRef") => "oasis-langref".to_string(),
                _ => "oasis-spec".to_string(),
            },
            Self::Plain { id } => id.clone(),
        }
    }

    /// 从源文件相对路径推可访问地址；推不出返回空串——一列半真的 url 比空列更坏。
    #[must_use]
    pub fn url(&self, rel: &Path) -> String {
        let Some(rel) = rel.to_str() else {
            return String::new();
        };
        match self {
            Self::DitaOt { version } => {
                let Some(stem) = strip_dita(rel) else {
                    return String::new();
                };
                match stem.strip_prefix("docsrc/") {
                    Some(rest) => format!("https://www.dita-ot.org/{version}/{rest}.html"),
                    None => String::new(),
                }
            }
            Self::Oasis => {
                let Some(stem) = strip_dita(rel) else {
                    return String::new();
                };
                let Some(rest) = stem.strip_prefix("specification/") else {
                    return String::new();
                };
                if OASIS_URL_DIRS.iter().any(|d| rest.starts_with(d)) {
                    format!("https://dita-lang.org/dita/{}", rest.to_lowercase())
                } else {
                    String::new()
                }
            }
            Self::Plain { .. } => String::new(),
        }
    }
}

fn strip_dita(rel: &str) -> Option<&str> {
    rel.strip_suffix(".dita")
}

/// 遍历各来源的 map 树，产出扁平节点表。
///
/// # Errors
///
/// 入口 map 本身读不了或不是良构 XML 时返回 `Err`；单个 topic 的问题记进
/// `notes`，不中断——一篇坏文件不该让整张索引生不出来。
pub fn build_index(sources: &[Source]) -> anyhow::Result<Index> {
    walk::build(sources)
}

/// 渲染成 tsv。文件头把"这是生成物"写死在第一行——治理的第一条是永不手改，
/// 而手改会被下次生成原样覆盖。
#[must_use]
pub fn render_tsv(index: &Index, command: &str, versions: &str, date: &str) -> String {
    let mut out = String::new();
    out.push_str("# 本文件由工具生成，勿手改——下次生成会原样覆盖。\n");
    // 写进 String 不会失败；`let _` 只是让类型系统闭嘴，不是在吞错误
    let _ = writeln!(out, "# 生成命令：{command}");
    let _ = writeln!(out, "# 来源版本：{versions}");
    let _ = writeln!(out, "# 生成日期：{date}");
    out.push_str("# 列：source\ttitle\tparent\tpath\turl\n");
    for e in &index.entries {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}",
            e.source, e.title, e.parent, e.path, e.url
        );
    }
    out
}
