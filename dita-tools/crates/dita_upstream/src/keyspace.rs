use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

/// key → 它指向的本地文件。
///
/// 上游两个来源都靠键寻址组织导航：DITA-OT 的 docsrc 里 590 多个 topicref 只有
/// 35 个直接写了 href，其余全是 `keyref`。不解析键，索引会漏掉九成节点。
///
/// 这层刻意不做键作用域（`@keyscope`）。作用域会让同名键在不同子树指向不同目标，
/// 正确实现要跟着解析走一遍作用域栈；两个来源合计只有 3 张 map 用到它，
/// 且都在词表/示例分支上，不在主题树里。真需要时再补，不在这里假装支持。
#[derive(Debug, Default)]
pub struct KeySpace {
    keys: HashMap<String, PathBuf>,
    /// 扫过的 map 张数，报告用
    pub maps_scanned: usize,
}

impl KeySpace {
    /// 从入口 map 出发，沿所有指向 `.ditamap` 的 href 广搜，收集键定义。
    ///
    /// 从入口出发而不是扫整棵目录树：上游仓库里躺着大量评审快照与历史版本
    /// （oasis-tcs/dita 有 20 多张 `ditaweb-review-*.ditamap`），它们定义同名键
    /// 指向旧文件。扫目录树 = 让评审快照决定"某个键是什么"。
    #[must_use]
    pub fn build(entry: &Path) -> Self {
        let mut space = Self::default();
        let mut seen = HashSet::new();
        let mut queue = vec![entry.to_path_buf()];

        while let Some(path) = queue.pop() {
            let Ok(canonical) = path.canonicalize() else {
                continue;
            };
            if !seen.insert(canonical.clone()) {
                continue;
            }
            let base = canonical.parent().unwrap_or(Path::new(".")).to_path_buf();
            let Ok(xml) = fs::read_to_string(&canonical) else {
                continue;
            };
            let opts = roxmltree::ParsingOptions {
                allow_dtd: true,
                ..roxmltree::ParsingOptions::default()
            };
            let Ok(doc) = roxmltree::Document::parse_with_options(&xml, opts) else {
                continue;
            };
            space.maps_scanned += 1;

            for node in doc.descendants().filter(roxmltree::Node::is_element) {
                let href = node.attribute("href");
                if let Some(href) = href {
                    if href.ends_with(".ditamap") && node.attribute("scope") != Some("external") {
                        queue.push(base.join(href));
                    }
                }
                let (Some(keys), Some(href)) = (node.attribute("keys"), href) else {
                    continue;
                };
                // 外部链接与非 DITA 目标（html/pdf）不是主题树上的节点
                if node.attribute("scope") == Some("external") {
                    continue;
                }
                // 非 DITA 目标（.md / .html / .pdf）不是本索引要收的节点
                let dita_target = Path::new(href).extension().is_some_and(|e| {
                    e.eq_ignore_ascii_case("dita") || e.eq_ignore_ascii_case("ditamap")
                });
                if !dita_target {
                    continue;
                }
                for key in keys.split_whitespace() {
                    // 先定义者胜——与 DITA 的键解析规则同向（最先遇到的定义有效）
                    space
                        .keys
                        .entry(key.to_string())
                        .or_insert_with(|| base.join(href));
                }
            }
        }
        space
    }

    #[must_use]
    pub fn resolve(&self, key: &str) -> Option<&Path> {
        self.keys.get(key).map(PathBuf::as_path)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
