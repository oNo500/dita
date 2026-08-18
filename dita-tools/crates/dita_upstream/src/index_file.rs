//! 读回已生成的索引文件。
//!
//! 放在本 crate 而不是 lint 里：TSV 的格式正本是 [`crate::render_tsv`]，读与写
//! 摆在同一处才不会各自漂——换了列序或注释头，两边一起改。

use std::{collections::HashSet, fs, path::Path};

use anyhow::{Context, bail};

/// 比对前的归一化（设计稿五之「比对的归一化」，2026-08-18 用户裁定）。
///
/// 三件事：首尾空白去除、内部连续空白折叠为一个、大小写不敏感。归一化之后是
/// **精确**匹配——不做模糊或子串匹配，那会把 `Specialization` 匹到
/// `Overview of specialization`，制造比误报更隐蔽的假通过。
#[must_use]
pub fn normalize(title: &str) -> String {
    title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// 读回的上游节点索引：归一化后的标题集合 ＋ 头部的来源版本与生成日期。
///
/// 只保留标题：校验要回答的是「这个声明解析得到吗」，父链与 url 是给人查的，
/// 不进这份内存结构。
pub struct NodeIndex {
    titles: HashSet<String>,
    provenance: String,
}

impl NodeIndex {
    /// 从 tsv 读回索引。
    ///
    /// # Errors
    ///
    /// 文件读不到、或读到了却一个节点行都没有（截断、写坏、只剩注释头）时返回
    /// `Err`。两种情况都必须往上传成「未执行」，不能当成「零个节点因而全都解析
    /// 不到」——那会把索引的故障报成 66 篇的错误。
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("上游节点索引读不了：{}", path.display()))?;

        let mut titles = HashSet::new();
        let (mut versions, mut date) = (String::new(), String::new());
        for line in text.lines() {
            if let Some(comment) = line.strip_prefix('#') {
                let comment = comment.trim();
                if let Some(v) = comment.strip_prefix("来源版本：") {
                    versions = v.to_string();
                } else if let Some(v) = comment.strip_prefix("生成日期：") {
                    date = v.to_string();
                }
                continue;
            }
            let Some(title) = line.split('\t').nth(1) else {
                continue;
            };
            if !title.trim().is_empty() {
                titles.insert(normalize(title));
            }
        }

        if titles.is_empty() {
            bail!(
                "上游节点索引里一个节点都没有：{}（重新生成：just upstream-index）",
                path.display()
            );
        }
        Ok(Self {
            titles,
            provenance: format!("生成日期 {date}，来源版本 {versions}"),
        })
    }

    /// 这个声明解析得到吗（归一化后精确匹配）。
    #[must_use]
    pub fn contains(&self, title: &str) -> bool {
        self.titles.contains(&normalize(title))
    }

    /// 索引头记的生成日期与来源版本，原样转述进解析不到时的消息里——
    /// 作者据此判断「是不是索引太旧了」。
    #[must_use]
    pub fn provenance(&self) -> &str {
        &self.provenance
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.titles.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.titles.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::normalize;

    #[test]
    fn normalize_folds_case_and_whitespace() {
        assert_eq!(
            normalize("  Structural   Specialization\n"),
            "structural specialization"
        );
        assert_eq!(normalize("DITA maps"), normalize("dita  MAPS"));
    }
}
