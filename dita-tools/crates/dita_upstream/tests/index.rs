use std::path::{Path, PathBuf};

use dita_upstream::{Entry, Flavor, Index, Source, build_index};

fn alpha() -> Source {
    Source {
        root: PathBuf::from("tests/fixtures/alpha"),
        entry: PathBuf::from("tests/fixtures/alpha/docsrc/site.ditamap"),
        flavor: Flavor::DitaOt {
            version: "9.9".to_string(),
        },
    }
}

fn beta() -> Source {
    Source {
        root: PathBuf::from("tests/fixtures/beta"),
        entry: PathBuf::from("tests/fixtures/beta/book.ditamap"),
        flavor: Flavor::Plain {
            id: "beta".to_string(),
        },
    }
}

fn find<'a>(index: &'a Index, title: &str) -> &'a Entry {
    index
        .entries
        .iter()
        .find(|e| e.title == title)
        .unwrap_or_else(|| panic!("索引里没有「{title}」：{:?}", titles(index)))
}

fn titles(index: &Index) -> Vec<&str> {
    index.entries.iter().map(|e| e.title.as_str()).collect()
}

#[test]
fn parent_comes_from_the_map_tree_not_the_directory() {
    // guide / chapter / nested 在磁盘上是同一个 topics/ 平铺目录；
    // 层级只存在于 map 里，parent 列要能把它还原出来
    let index = build_index(&[alpha()]).expect("build failed");
    assert_eq!(find(&index, "Guide").parent, "");
    assert_eq!(find(&index, "Chapter two").parent, "Guide");
    assert_eq!(find(&index, "Nested").parent, "Chapter two");
}

#[test]
fn a_referenced_maps_title_is_not_a_node() {
    // DITA 规定被引 map 的 <title> 不产生导航节点。把它当节点，索引里就会多出
    // 上游根本不存在的条目——而索引的唯一用途正是核实节点存不存在
    let index = build_index(&[alpha()]).expect("build failed");
    assert!(
        !titles(&index).contains(&"子 map 的标题不是节点"),
        "{:?}",
        titles(&index)
    );
}

#[test]
fn keyrefs_are_resolved_against_the_key_space() {
    // docsrc 里九成 topicref 只写 keyref；不解析键就只剩一成节点
    let index = build_index(&[alpha()]).expect("build failed");
    assert_eq!(
        find(&index, "Guide").path,
        "docsrc/topics/guide.dita",
        "keyref=\"guide\" 应解析到键定义指的文件"
    );
    assert!(
        index.notes.iter().any(|n| n.contains("no-such-key")),
        "解析不到的键要报出来，不能默默吞掉：{:?}",
        index.notes
    );
}

#[test]
fn resource_only_subtrees_are_not_nodes() {
    let index = build_index(&[alpha()]).expect("build failed");
    assert!(!titles(&index).contains(&"Loose"), "{:?}", titles(&index));
}

#[test]
fn a_topic_referenced_twice_is_one_node() {
    let index = build_index(&[alpha()]).expect("build failed");
    assert_eq!(
        titles(&index)
            .iter()
            .filter(|t| **t == "Chapter two")
            .count(),
        1
    );
}

#[test]
fn folded_titles_are_collapsed_to_one_line() {
    // 源文件为排版折行的标题，原样写进 tsv 会把一行撑成两行
    let index = build_index(&[alpha()]).expect("build failed");
    assert_eq!(find(&index, "Chapter two").title, "Chapter two");
}

#[test]
fn urls_follow_the_source_layout_and_stop_at_chunked_subtrees() {
    let index = build_index(&[alpha()]).expect("build failed");
    assert_eq!(
        find(&index, "Guide").url,
        "https://www.dita-ot.org/9.9/topics/guide.html"
    );
    // chunk="to-content" 把子树并成一个页面，子节点没有自己的地址
    assert_eq!(find(&index, "Nested").url, "");
}

#[test]
fn bookmap_chapters_and_topicheads_both_carry_hierarchy() {
    let index = build_index(&[beta()]).expect("build failed");
    assert_eq!(find(&index, "Only").parent, "");
    // 导航节点没有文件：有标题有父子关系，path 与 url 留空
    let head = find(&index, "无文件的导航节点");
    assert_eq!(head.path, "");
    assert_eq!(head.url, "");
    assert_eq!(find(&index, "Under head").parent, "无文件的导航节点");
    // 封面在 frontmatter/notices 里，不是主题树节点
    assert!(!titles(&index).contains(&"Cover"), "{:?}", titles(&index));
}

#[test]
fn sources_merge_into_one_table_and_keep_their_own_ids() {
    let index = build_index(&[alpha(), beta()]).expect("build failed");
    assert_eq!(find(&index, "Guide").source, "dita-ot");
    assert_eq!(find(&index, "Only").source, "beta");
    // 排序按 (source, path)：合并后的表 diff 才稳定，改一个节点只动一行
    let keys: Vec<(&str, &str)> = index
        .entries
        .iter()
        .map(|e| (e.source.as_str(), e.path.as_str()))
        .collect();
    let mut sorted = keys.clone();
    sorted.sort_unstable();
    assert_eq!(keys, sorted);
}

#[test]
fn oasis_paths_split_into_archspec_and_langref() {
    // source 列按规范的两大部分分——archSpec 与 langRef 是声明溯源时最常引的两处
    let f = Flavor::Oasis;
    let id = |p: &str| f.source_id(Path::new(p));
    assert_eq!(
        id("specification/archSpec/base/chunking.dita"),
        "oasis-archspec"
    );
    assert_eq!(id("specification/langRef/base/p.dita"), "oasis-langref");
    assert_eq!(
        id("specification/conformance/conformance.dita"),
        "oasis-spec"
    );
    // 目录名大小写照抄源码树，url 里才小写——两边不是同一个东西
    assert_eq!(
        f.url(Path::new("specification/langRef/base/p.dita")),
        "https://dita-lang.org/dita/langref/base/p"
    );
    // introduction/ 下不少文件是被 conref 的片段，没有独立页面：推不出就留空
    assert_eq!(
        f.url(Path::new(
            "specification/introduction/about-the-dita-specification.dita"
        )),
        ""
    );
}

#[test]
fn the_header_says_it_is_generated() {
    // 治理第一条：永不手改。靠的是第一行就写着，而不是靠人记得
    let index = build_index(&[beta()]).expect("build failed");
    let tsv = dita_upstream::render_tsv(&index, "cmd", "versions", "2026-08-18");
    let head: Vec<&str> = tsv.lines().take(5).collect();
    assert!(head[0].contains("勿手改"), "{head:?}");
    assert!(head[1].contains("cmd"));
    assert!(head[2].contains("versions"));
    assert!(head[3].contains("2026-08-18"));
    assert!(head.iter().all(|l| l.starts_with('#')));
    // 除文件头外每行五列，不多不少
    for line in tsv.lines().filter(|l| !l.starts_with('#')) {
        assert_eq!(line.split('\t').count(), 5, "{line}");
    }
}
