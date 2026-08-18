use dita_lint::lint_topic;
use dita_upstream::NodeIndex;
use dita_vocab::parse_vocab;
use std::path::Path;

/// With the upstream index loaded — R19 runs.
fn lint(name: &str) -> dita_diagnostics::DiagnosticBag {
    let index = NodeIndex::load(Path::new("tests/fixtures/upstream-nodes.tsv")).unwrap();
    lint_with(name, Some(&index))
}

/// Without it — R19 must not run at all (the skip channel).
fn lint_no_index(name: &str) -> dita_diagnostics::DiagnosticBag {
    lint_with(name, None)
}

fn lint_with(name: &str, index: Option<&NodeIndex>) -> dita_diagnostics::DiagnosticBag {
    let (vocab, _) = parse_vocab(Path::new("tests/fixtures/scheme.ditamap")).unwrap();
    lint_topic(
        Path::new("tests/fixtures").join(name).as_path(),
        &vocab,
        index,
    )
    .unwrap()
}

fn messages(d: &dita_diagnostics::DiagnosticBag) -> Vec<&str> {
    d.items
        .iter()
        .map(dita_diagnostics::Diagnostic::message)
        .collect()
}

#[test]
fn clean_topic_passes() {
    let d = lint("clean.dita");
    assert_eq!(
        d.items.len(),
        0,
        "{:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn curated_violations_are_errors() {
    // the promotion gate: curated claims compliance, so failures block
    let d = lint("violations.dita");
    assert!(
        d.error_count() >= 5,
        "缺节×2 + 旧标签 + 手写日期 + 粗体/程度词: {:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
    assert_eq!(d.warning_count(), 0);
    let msgs: Vec<&str> = d
        .items
        .iter()
        .map(dita_diagnostics::Diagnostic::message)
        .collect();
    assert!(msgs.iter().any(|m| m.contains("缺必需节「做法」")));
    assert!(msgs.iter().any(|m| m.contains("已核对")));
    assert!(msgs.iter().any(|m| m.contains("手写日期")));
}

#[test]
fn draft_violations_are_warnings_only() {
    // a draft is free to be unfinished; the report is a worklist, not a gate
    let d = lint("draft-violations.dita");
    assert_eq!(d.error_count(), 0);
    assert!(d.warning_count() >= 1);
}

#[test]
fn genre_must_match_dita_type() {
    let d = lint("wrong-type.dita");
    assert!(
        d.items
            .iter()
            .any(|i| i.message().contains("不能标在 concept 上"))
    );
}

#[test]
fn concept_over_split_threshold_is_flagged() {
    let d = lint("over-threshold.dita");
    assert!(
        d.items.iter().any(|i| i.message().contains("R16")),
        "{:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn claim_shaped_titles_are_flagged() {
    let d = lint("claim-title.dita");
    assert!(d.items.iter().any(|i| i.message().contains("论断句式")));
}

#[test]
fn missing_maturity_is_flagged_as_error() {
    // R18: an unmet @maturity slips past the DITAVAL exclude entirely (it
    // only matches val="draft"), so this must always be an error, not graded
    // by the (absent) maturity like R12-R16 are.
    let d = lint("missing-maturity.dita");
    assert_eq!(
        d.error_count(),
        1,
        "{:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
    assert!(
        d.items
            .iter()
            .any(|i| i.is_error() && i.message().contains("R18"))
    );
}

#[test]
fn present_maturity_passes_r18() {
    // clean.dita carries maturity="curated" already; R18 must stay silent.
    let d = lint("clean.dita");
    assert!(d.items.iter().all(|i| !i.message().contains("R18")));
}

#[test]
fn glossentry_missing_maturity_is_flagged() {
    // R18's context matches R2's in rules.sch, not CONTENT_ROOTS: glossentry
    // is out of scope for R12-R16 (genre/structure/register) but must still
    // be checked here, since it wasn't covered by any lint gate before.
    let d = lint("glossentry-missing-maturity.dita");
    assert_eq!(
        d.error_count(),
        1,
        "{:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
    assert!(
        d.items
            .iter()
            .any(|i| i.is_error() && i.message().contains("R18"))
    );
}

#[test]
fn glossentry_with_maturity_passes() {
    let d = lint("glossentry-clean.dita");
    assert_eq!(
        d.items.len(),
        0,
        "{:?}",
        d.items
            .iter()
            .map(dita_diagnostics::Diagnostic::message)
            .collect::<Vec<_>>()
    );
}

// ── R19：声明式溯源 ──────────────────────────────────────────────

#[test]
fn declared_node_present_in_index_passes() {
    let d = lint("upstream-ok.dita");
    assert!(d.items.is_empty(), "{:?}", messages(&d));
}

/// 上游写 `DITA maps`，本库抄成 `dita MAPS`——同一个节点。逐字匹配会让
/// 全库误报，而误报会让规则被忽略（设计五之「比对的归一化」）。
#[test]
fn case_difference_still_resolves() {
    let d = lint("upstream-case.dita");
    assert!(d.items.is_empty(), "{:?}", messages(&d));
}

/// 首尾空白与内部连续空白是抄写与换行带入的差异，不是真差异。
#[test]
fn whitespace_difference_still_resolves() {
    let d = lint("upstream-space.dita");
    assert!(d.items.is_empty(), "{:?}", messages(&d));
}

/// 解析不到时不得断言"拼错了"：索引本身可能不全，把索引的空缺报成作者的
/// 错误是最伤可信度的一类误报。消息必须把三种可能都列出来。
#[test]
fn unresolvable_node_reports_all_three_possibilities() {
    let d = lint("upstream-unknown.dita");
    let msgs = messages(&d);
    let r19: Vec<&&str> = msgs.iter().filter(|m| m.contains("R19")).collect();
    assert_eq!(r19.len(), 1, "{msgs:?}");
    let m = r19[0];
    assert!(m.contains("Structural specialization"), "{m}");
    assert!(m.contains("拼写有误"), "缺「拼写有误」这一可能：{m}");
    assert!(m.contains("改名或删除"), "缺「上游改名/删除」这一可能：{m}");
    assert!(m.contains("索引未收录"), "缺「索引未收录」这一可能：{m}");
    assert!(m.contains("生成日期"), "须提示核对索引头：{m}");
    assert!(m.contains("来源版本"), "须提示核对索引头：{m}");
}

/// 归一化后是精确匹配：`Structural specialization` 不得被
/// `Overview of specialization` 以子串或模糊的方式匹上——假通过比误报更隐蔽。
#[test]
fn matching_is_exact_not_fuzzy() {
    let index = NodeIndex::load(Path::new("tests/fixtures/upstream-nodes.tsv")).unwrap();
    assert!(index.contains("Overview of specialization"));
    assert!(!index.contains("specialization"));
    assert!(!index.contains("Structural specialization"));
    assert!(!index.contains("DITA maps and their usage"));
}

/// 三道关的名字在头注释里是自然中文，会带标点、会跨行——本库真的写着
/// 「只组合，不发明」。字面子串搜索会把它报成缺失，那正是最伤可信度的误报，
/// 所以比对前两边都去标点与空白（fixture 里就是带逗号加换行的写法）。
#[test]
fn coined_with_three_gates_passes() {
    let d = lint("upstream-coined-ok.dita");
    assert!(d.items.is_empty(), "{:?}", messages(&d));
}

#[test]
fn coined_without_three_gates_is_reported() {
    let d = lint("upstream-coined-bare.dita");
    let msgs = messages(&d);
    let m = msgs.iter().find(|m| m.contains("R19")).expect("应报 R19");
    assert!(m.contains("三道关"), "{m}");
    for gate in ["穷尽查证", "先怀疑切分", "只组合不发明"] {
        assert!(m.contains(gate), "缺哪一关要点名：{m}");
    }
}

#[test]
fn missing_declaration_is_reported_for_dita_domain() {
    let d = lint("upstream-absent.dita");
    let msgs = messages(&d);
    assert!(
        msgs.iter()
            .any(|m| m.contains("R19") && m.contains("缺 upstream-node")),
        "{msgs:?}"
    );
}

/// 上游尚未建索引的分支跳过（设计七之二：规则全库通用，实现分期）。
#[test]
fn other_domain_is_out_of_scope_for_now() {
    let d = lint("upstream-other-domain.dita");
    assert!(
        messages(&d).iter().all(|m| !m.contains("R19")),
        "{:?}",
        messages(&d)
    );
}

/// 组合篇可声明多条，逐条校验。
#[test]
fn combined_topic_may_declare_several_nodes() {
    let d = lint("upstream-multi.dita");
    assert!(d.items.is_empty(), "{:?}", messages(&d));

    let bad = lint("upstream-multi-one-bad.dita");
    let msgs = messages(&bad);
    let r19: Vec<&&str> = msgs.iter().filter(|m| m.contains("R19")).collect();
    assert_eq!(r19.len(), 1, "只报解析不到的那一条：{msgs:?}");
    assert!(r19[0].contains("Structural specialization"), "{:?}", r19[0]);
}

/// 索引缺失或不可读 → R19 一条都不报（调用方负责报"未执行"）。
/// 静默通过与满屏假错同样不可接受，故这里既不报错也不算通过。
#[test]
fn missing_index_skips_r19_entirely() {
    for f in [
        "upstream-unknown.dita",
        "upstream-absent.dita",
        "upstream-coined-bare.dita",
    ] {
        let d = lint_no_index(f);
        assert!(
            messages(&d).iter().all(|m| !m.contains("R19")),
            "{f}: {:?}",
            messages(&d)
        );
    }
}

#[test]
fn unreadable_index_is_an_error_not_an_empty_index() {
    assert!(NodeIndex::load(Path::new("tests/fixtures/does-not-exist.tsv")).is_err());
    assert!(NodeIndex::load(Path::new("tests/fixtures/scheme.ditamap")).is_err());
}

/// R19 随 maturity 分级，与 R12–R16 一致：draft 记 warning，curated 记 error。
#[test]
fn r19_severity_follows_maturity() {
    let curated = lint("upstream-unknown.dita");
    assert!(
        curated
            .items
            .iter()
            .any(|i| i.is_error() && i.message().contains("R19"))
    );

    let src = std::fs::read_to_string("tests/fixtures/upstream-unknown.dita").unwrap();
    let draft = src.replace("maturity=\"curated\"", "maturity=\"draft\"");
    let tmp = std::env::temp_dir().join("r19-draft.dita");
    std::fs::write(&tmp, draft).unwrap();
    let (vocab, _) = parse_vocab(Path::new("tests/fixtures/scheme.ditamap")).unwrap();
    let index = NodeIndex::load(Path::new("tests/fixtures/upstream-nodes.tsv")).unwrap();
    let d = lint_topic(&tmp, &vocab, Some(&index)).unwrap();
    assert!(
        d.items
            .iter()
            .any(|i| !i.is_error() && i.message().contains("R19")),
        "{:?}",
        messages(&d)
    );
    assert_eq!(d.error_count(), 0, "{:?}", messages(&d));
}
