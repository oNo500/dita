use dita_ia::{IaReport, build_report};
use std::path::{Path, PathBuf};

const MINI: &str = "tests/fixtures/mini";

fn report() -> IaReport {
    let root = Path::new(MINI);
    build_report(
        &[root.join("maps/root.ditamap")],
        &root.join("topics"),
        Some(&root.join("maps")),
        Some(&root.join("vocab/scheme.ditamap")),
    )
    .expect("report failed")
}

fn report_without_vocab() -> IaReport {
    let root = Path::new(MINI);
    build_report(
        &[root.join("maps/root.ditamap")],
        &root.join("topics"),
        Some(&root.join("maps")),
        Some(&PathBuf::from("does/not/exist.ditamap")),
    )
    .expect("report failed")
}

#[test]
fn coverage_counts_only_planned_dimensions() {
    // same semantics as dimension-coverage.py: dim-usage is covered and planned,
    // dim-nonexistent is covered but not planned, so it lifts neither numerator
    // nor denominator
    let report = report();
    let demo = report
        .coverage
        .iter()
        .find(|c| c.domain == "demo")
        .expect("demo domain");
    assert_eq!(demo.planned.len(), 3);
    // dim-concept 与 dim-usage 直接归 demo；dim-security 来自孙键 demo-b1，按分类法上卷
    assert_eq!(demo.covered.len(), 3);
    assert!(demo.blind.is_empty());
    // 规划外的覆盖不进分子、也不进分母
    assert!(demo.outside_plan.contains("dim-nonexistent"));
    assert_eq!(demo.percent(), 100);
}

#[test]
fn coverage_records_which_branch_the_domain_sits_in() {
    // this is what the map tree adds over the script: the script knows only
    // what each topic declares about itself
    let report = report();
    let demo = report.coverage.iter().find(|c| c.domain == "demo").unwrap();
    assert_eq!(
        demo.branches.iter().map(String::as_str).collect::<Vec<_>>(),
        ["演示分支"]
    );
}

#[test]
fn illegal_values_are_errors_naming_the_attribute() {
    let report = report();
    let messages: Vec<&str> = report
        .diagnostics
        .items
        .iter()
        .filter(|d| d.is_error())
        .map(dita_diagnostics::Diagnostic::message)
        .collect();
    assert!(
        messages.iter().any(|m| m.contains("dim-nonexistent")),
        "illegal @dimension must be reported: {messages:?}"
    );
    assert!(
        messages.iter().any(|m| m.contains("maturity")),
        "illegal @maturity must be reported: {messages:?}"
    );
}

#[test]
fn without_a_vocabulary_values_are_not_guessed_at() {
    // skipping the check and saying so beats inventing a legal-value list
    let report = report_without_vocab();
    assert!(!report.vocab_loaded);
    assert!(
        !report
            .diagnostics
            .items
            .iter()
            .any(|d| d.message().contains("不在词表中")),
        "no vocabulary means no value verdicts"
    );
}

#[test]
fn empty_branch_is_listed_with_zero_topics() {
    // the whole point of the IA view: a branch that exists and holds nothing
    // has to stay visible
    let report = report();
    let empty = report
        .branch_stats
        .iter()
        .find(|b| b.name == "空分支")
        .expect("empty branch must appear");
    assert_eq!(empty.topics, 0);
}

#[test]
fn branch_stats_break_topics_down_by_type_and_maturity() {
    let report = report();
    let demo = report
        .branch_stats
        .iter()
        .find(|b| b.name == "演示分支")
        .expect("demo branch");
    assert_eq!(demo.topics, 5);
    assert_eq!(demo.by_type.get("concept"), Some(&5));
    assert_eq!(demo.by_maturity.get("curated"), Some(&4));
}

#[test]
fn plan_matches_branches_by_map_file_name() {
    // the scheme keys branches in English and the maps title them in Chinese;
    // domains/<key>.ditamap is the only thing carrying the correspondence
    let report = report();
    let demo = report
        .plans
        .iter()
        .find(|p| p.key == "demo")
        .expect("demo plan");
    assert_eq!(demo.matched_branch.as_deref(), Some("演示分支"));
    assert_eq!(demo.built, 5);
    assert_eq!(demo.planned.len(), 2, "direct sub-topics");
    assert_eq!(demo.planned_total, 3, "including the nested one");
}

#[test]
fn unmatched_subject_key_is_not_counted_as_zero() {
    // silently reporting "0 built" for a key with no map would read as "nothing
    // written yet" when the truth is "nothing to compare against"
    let report = report();
    let nomap = report
        .plans
        .iter()
        .find(|p| p.key == "nomap")
        .expect("nomap plan");
    assert_eq!(nomap.matched_branch, None);
}

#[test]
fn benchmark_entries_carry_dates_and_cadence() {
    let report = report();
    let demo = report
        .benchmarks
        .iter()
        .find(|b| b.key == "bm-demo")
        .expect("bm-demo");
    assert_eq!(demo.last_benchmarked.as_deref(), Some("2026-01-01"));
    assert_eq!(demo.due_months(), Some(6));
    // anchors are element text, dates are @value — both forms must be read
    assert_eq!(demo.anchor.as_deref(), Some("某个外部对标锚点"));

    let on_trigger = report
        .benchmarks
        .iter()
        .find(|b| b.key == "bm-empty")
        .unwrap();
    assert_eq!(
        on_trigger.due_months(),
        None,
        "event-triggered has no calendar expiry"
    );
}

#[test]
fn unused_controlled_values_are_listed() {
    let report = report();
    let maturity = report
        .value_usage
        .iter()
        .find(|u| u.attribute == "maturity")
        .expect("maturity usage");
    assert_eq!(maturity.used.get("curated"), Some(&4));
    assert!(
        maturity.unused.contains("draft"),
        "a value the vocabulary defines but no topic uses must show up"
    );

    let dimension = report
        .value_usage
        .iter()
        .find(|u| u.attribute == "dimension")
        .unwrap();
    // an illegal value is not usage: it is already an error elsewhere
    assert!(!dimension.used.contains_key("dim-nonexistent"));
}

// ── 骨架（设计见 docs/plans/2026-08-15-skeleton-design.md）────────────────

fn node<'a>(nodes: &'a [dita_ia::Node], key: &str) -> &'a dita_ia::Node {
    nodes
        .iter()
        .find(|n| n.key == key)
        .unwrap_or_else(|| panic!("node {key}"))
}

#[test]
fn skeleton_is_the_subject_tree_with_content_hung_on_it() {
    let report = report();
    let demo = node(&report.skeleton, "demo");
    assert_eq!(
        demo.children.len(),
        2,
        "planned sub-topics are listed even when empty"
    );
    assert_eq!(demo.state, dita_ia::State::InProgress);
    // planned but unwritten sub-topics must be visible — that is the point
    assert_eq!(
        node(&demo.children, "demo-a").state,
        dita_ia::State::Unbuilt
    );
}

#[test]
fn topics_without_a_domain_land_in_the_unplaced_bucket() {
    // the only link from a topic to a subject key is <data name="domain">;
    // without it nothing can place the topic, and hiding that would hide the
    // largest structural gap in the library
    let report = report();
    let demo = node(&report.skeleton, "demo");
    assert_eq!(
        demo.topics.len() + demo.unplaced.len(),
        4,
        "nested 归在孙键，不计入 demo 节点自身"
    );
}

#[test]
fn branch_without_a_vocabulary_key_is_not_applicable_not_unbuilt() {
    // an organisational branch (the glossary) is not a subject: it can never be
    // "done", and calling it unbuilt would be a permanent false alarm
    let mut report = report();
    report
        .skeleton
        .retain(|n| n.state == dita_ia::State::NotApplicable);
    assert!(
        report.skeleton.iter().all(|n| n.children.is_empty()),
        "unkeyed branches carry topics, not planned children"
    );
}

#[test]
fn an_empty_planned_branch_stays_unbuilt() {
    let report = report();
    assert_eq!(
        node(&report.skeleton, "empty").state,
        dita_ia::State::Unbuilt
    );
}

#[test]
fn a_typo_in_domain_falls_into_the_bucket_and_errors() {
    // the failure mode this guards: "declared anything" counted as placed, so a
    // typo'd domain hung nowhere and the topic vanished from the skeleton —
    // worse than undeclared, which at least stays visible in the bucket
    let report = report();
    let demo = node(&report.skeleton, "demo");
    assert!(
        demo.unplaced
            .iter()
            .any(|n| n.file_name == "bogus-domain.dita"),
        "typo'd domain must stay visible in the unplaced bucket: {:?}",
        demo.unplaced
    );
    assert!(
        report
            .diagnostics
            .items
            .iter()
            .any(|d| d.is_error() && d.message().contains("not-a-subject-key")),
        "and be reported as an error naming the bogus value"
    );
}

#[test]
fn coverage_rolls_up_the_subject_tree() {
    // a subject scheme is a taxonomy: filing under demo-b1 (a grandchild) covers
    // the dimension for demo. Exact matching would force a near-identical
    // landscape under every leaf.
    let report = report();
    let demo = report.coverage.iter().find(|c| c.domain == "demo").unwrap();
    assert!(
        demo.covered.contains("dim-security"),
        "a grandchild's dimension must count toward the ancestor's plan"
    );
    // demo 自身 3 篇（landscape/good/illegal）＋ 孙键 demo-b1 的 1 篇；
    // bogus-domain 的 domain 不是词表键，不计入任何域
    assert_eq!(demo.topics, 4, "rolled-up topics are counted too");
}

// ── R17: domain 必须是 subjectScheme 已注册的 subject key ──────────────────

#[test]
fn r17_a_registered_domain_value_is_not_flagged() {
    // good.dita / landscape.dita declare domain="demo", and nested.dita
    // declares the grandchild key "demo-b1" — both are registered subject
    // keys, so none of these three files should ever carry an R17 error.
    // (bogus-domain.dita is the deliberate counter-example, covered below.)
    let report = report();
    let r17_paths: Vec<String> = report
        .diagnostics
        .items
        .iter()
        .filter(|d| d.is_error() && d.message().contains("R17"))
        .map(|d| d.path().display().to_string())
        .collect();
    for clean in ["good.dita", "landscape.dita", "nested.dita"] {
        assert!(
            !r17_paths.iter().any(|p| p.ends_with(clean)),
            "{clean} declares a registered domain and must not raise R17: {r17_paths:?}"
        );
    }
}

#[test]
fn r17_an_unregistered_domain_value_errors_with_a_fix_hint() {
    // bogus-domain.dita declares domain="not-a-subject-key", which names no
    // subjectdef anywhere in the scheme. The message must name the offending
    // value and point at the fix (register it, or use a registered value) —
    // a bare "illegal" tells the author nothing about what to do next.
    let report = report();
    let message = report
        .diagnostics
        .items
        .iter()
        .find(|d| d.is_error() && d.message().contains("not-a-subject-key"))
        .map(dita_diagnostics::Diagnostic::message)
        .expect("bogus domain must be reported");
    assert!(message.contains("R17"), "message must cite R17: {message}");
    assert!(
        message.contains("注册"),
        "message must hint at registering the key or using a registered one: {message}"
    );
}

#[test]
fn r17_empty_leaves_by_branch_counts_unclaimed_leaves_per_branch() {
    // the reverse report: demo-a, empty-a and nomap are registered leaves no
    // topic ever names as domain — the tree's empty leaves, counted per
    // top-level branch. demo-b1 is a leaf too but nested.dita claims it, so
    // "demo" must count only demo-a (1), not demo-b1 as well (would be 2).
    // "nomap" has no children of its own, so it is simultaneously a
    // top-level branch and the one leaf under it.
    let report = report();
    let by_branch = &report.empty_leaves_by_branch;
    let count = |branch: &str| by_branch.iter().find(|(b, _)| b == branch).map(|(_, n)| *n);
    assert_eq!(
        count("demo"),
        Some(1),
        "demo-b1 is claimed, so only demo-a should count: {by_branch:?}"
    );
    assert_eq!(
        count("empty"),
        Some(1),
        "empty-a is unclaimed: {by_branch:?}"
    );
    assert_eq!(
        count("nomap"),
        Some(1),
        "nomap is a leaf and a branch at once: {by_branch:?}"
    );
    let total: usize = by_branch.iter().map(|(_, n)| n).sum();
    assert_eq!(total, 3);
}

#[test]
fn r17_empty_leaves_by_branch_is_empty_without_a_vocabulary() {
    // no vocabulary means no "ought" to compare against — silence, not a
    // false claim that everything is covered.
    let report = report_without_vocab();
    assert!(report.empty_leaves_by_branch.is_empty());
}

/// 回归锁：`--details` 只该藏「词表空叶子」这一条存量清单，不该连带藏掉缺陷信号。
///
/// 背景：R17 修复轮的指令是把空叶子降级到 `--details`，但实现把整段异常都门控了，
/// 「规划外的覆盖」因此在无参 `just ia` 下不可见——四篇 topic 的维度漂移就这样穿过了
/// 每一个簇的验收（各簇跑的都是无参 `just ia`，报告写「无 ⚠」时是当时口径下的真话）。
/// 本测试同时钉住两件事，缺一件都会让那次回归重演：
///   一、空叶子**确实**被门控（否则降级的初衷丢了）；
///   二、其余异常**确实没有**被门控（否则缺陷又被藏起来）。
#[test]
fn only_empty_leaves_hide_behind_details() {
    let report = report();
    let bare = dita_ia::exception_lines(&report, false);
    let full = dita_ia::exception_lines(&report, true);

    let has = |lines: &[String], needle: &str| lines.iter().any(|l| l.contains(needle));

    // 一、空叶子是存量清单（树先立、内容后填），只在 --details 下列
    assert!(
        !has(&bare, "词表空叶子"),
        "空叶子不该出现在无参输出里：{bare:?}"
    );
    assert!(
        has(&full, "词表空叶子"),
        "--details 下必须列出空叶子：{full:?}"
    );

    // 二、规划外的覆盖是缺陷信号，两种模式下都必须打印
    assert!(
        has(&bare, "规划外的覆盖"),
        "规划外的覆盖被门控了——正是这次回归的形状：{bare:?}"
    );
    assert!(
        has(&bare, "dim-nonexistent"),
        "越界的那个维度名要指名道姓，否则读者无从下手：{bare:?}"
    );
    assert!(has(&full, "规划外的覆盖"), "{full:?}");

    // 三、两种模式的差集**只有**空叶子那一行——任何新增的门控都会让这条断言失败
    let only_in_full: Vec<&String> = full.iter().filter(|l| !bare.contains(l)).collect();
    assert_eq!(
        only_in_full.len(),
        1,
        "--details 只该多出空叶子一行，实际多出：{only_in_full:?}"
    );
    assert!(only_in_full[0].contains("词表空叶子"), "{only_in_full:?}");
}

// ── 重复 topicref（判定边界见 src/duplicates.rs 的模块注释）───────────────

const DUPES: &str = "tests/fixtures/dupes";

fn dupes_report() -> IaReport {
    let root = Path::new(DUPES);
    build_report(
        &[root.join("maps/root.ditamap")],
        &root.join("topics"),
        Some(&root.join("maps")),
        // 这棵 fixture 树没有词表：重复检测不读词表，值检查跳过并说明即可
        Some(&PathBuf::from("does/not/exist.ditamap")),
    )
    .expect("report failed")
}

fn dup_of<'a>(report: &'a IaReport, topic: &str) -> Vec<&'a dita_ia::DuplicateRef> {
    report
        .duplicate_refs
        .iter()
        .filter(|d| d.topic.ends_with(topic))
        .collect()
}

/// 同 map 内重复：a.ditamap 引了 t1 两次。没有合法读法——一个 map 说不出
/// "这一篇在我这里有两个位置"。
#[test]
fn a_topic_referenced_twice_by_one_map_is_reported() {
    let report = dupes_report();
    let hits = dup_of(&report, "t1.dita");
    assert_eq!(hits.len(), 1, "{:?}", report.duplicate_refs);
    assert_eq!(hits[0].kind, dita_ia::DuplicateKind::SameMap);
    assert_eq!(hits[0].count, 2);
    assert!(hits[0].scope.ends_with("a.ditamap"), "{:?}", hits[0]);
}

/// 同一棵树内经不同 map 两次到达：a 与 b 各引一次 t2，展开后导航里是两个节点，
/// 分支统计与覆盖度也各算一遍。
#[test]
fn a_topic_reached_twice_through_different_maps_is_reported() {
    let report = dupes_report();
    let hits = dup_of(&report, "t2.dita");
    assert_eq!(hits.len(), 1, "{:?}", report.duplicate_refs);
    assert_eq!(hits[0].kind, dita_ia::DuplicateKind::SameTree);
    assert!(hits[0].scope.ends_with("root.ditamap"));
    assert_eq!(hits[0].via.len(), 2, "两个来源 map 都要点名：{:?}", hits[0]);
}

/// **不报**：t3 在 root 树与交付物 map 各出现一次。一份内容多处编排正是 DITA
/// 的用途，报了就是误报——这一条塌了，整个检查会因噪声被关掉。
#[test]
fn the_same_topic_in_two_separate_trees_is_legitimate() {
    let report = dupes_report();
    assert!(
        dup_of(&report, "t3.dita").is_empty(),
        "跨编排单位不是重复：{:?}",
        report.duplicate_refs
    );
}

/// **不报**：t4 被两条 `processing-role="resource-only"` 的 topicref 指到。
/// 它们不是导航节点，同一个 href 挂两个 key 是合法的别名手法。
#[test]
fn resource_only_references_are_not_navigation_duplicates() {
    let report = dupes_report();
    assert!(
        dup_of(&report, "t4.dita").is_empty(),
        "resource-only 不是导航节点：{:?}",
        report.duplicate_refs
    );
}

/// 领域 map 既作为 root 的子树、又被 --maps-dir 单独解析一次。同一份内容看两遍，
/// 计数不得翻倍，也不得把 t1 报成两条。
#[test]
fn a_map_seen_from_two_directions_is_counted_once() {
    let report = dupes_report();
    assert_eq!(
        report.duplicate_refs.len(),
        2,
        "只该有 t1（同 map）与 t2（同树）两条：{:?}",
        report.duplicate_refs
    );
}

/// 重复是缺陷信号，不是存量清单——两种模式下都必须出现在「需要处理」里。
/// （空叶子那次的教训：把缺陷信号门控到 --details 之下，四篇漂移穿过了每一次验收。）
#[test]
fn duplicate_refs_are_never_gated_behind_details() {
    let report = dupes_report();
    for details in [false, true] {
        let lines = dita_ia::exception_lines(&report, details);
        assert!(
            lines.iter().any(|l| l.contains("内重复引用")),
            "details={details}: {lines:?}"
        );
        assert!(
            lines.iter().any(|l| l.contains("次到达")),
            "details={details}: {lines:?}"
        );
    }
}

/// 干净的库一条都不报：mini fixture 没有任何重复。
#[test]
fn a_clean_library_reports_no_duplicates() {
    let report = report();
    assert!(
        report.duplicate_refs.is_empty(),
        "{:?}",
        report.duplicate_refs
    );
}

// ── JSON 形（契约见 src/json.rs 的模块注释）──────────────────────────────

/// 顶层字段是契约的骨架。这条断言故意用**相等**而不是 contains：加字段要主动
/// 改这里一次，删字段更要——下游按名取值，删掉一个就是破坏性变更。
#[test]
fn json_top_level_keys_are_the_contract() {
    let json = dita_ia::json_report(&report());
    let mut keys: Vec<&str> = json
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "benchmarks",
            "branches",
            "coverage",
            "diagnostics",
            "exceptions",
            "plans",
            "schema_version",
            "skeleton",
            "totals",
            "value_usage",
        ]
    );
    // 2（2026-08-19）：skeleton[].topics[] / unplaced[] 从字符串（文件名）改成
    // {file_name, title} 对象——元素形状变了，抬版本号。
    assert_eq!(json["schema_version"], 2);
}

/// 每一段都要有内容，不能是空壳：分支树、每域篇数与覆盖度、词表统计。
#[test]
fn json_carries_the_tree_the_coverage_and_the_vocabulary_stats() {
    let json = dita_ia::json_report(&report());

    assert_eq!(json["totals"]["topics"], 5);
    assert!(json["totals"]["planned_nodes"].as_u64().unwrap() > 0);

    let demo = json["skeleton"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["key"] == "demo")
        .expect("demo 节点");
    assert_eq!(demo["state"], "in_progress");
    assert_eq!(demo["label"], "演示分支");
    assert!(!demo["children"].as_array().unwrap().is_empty());
    // 没有全景的节点是 null，不是 0/0——后者会被读成"规划了零维度"
    assert!(demo["children"][0]["coverage"].is_null());

    // topics[] / unplaced[] 元素是 {file_name, title}，不再是裸文件名——
    // 这是本轮改动加的：`just ia` 默认要能显示标题，JSON 面也得跟着给标题。
    let landscape = demo["topics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["file_name"] == "landscape.dita")
        .expect("landscape.dita in demo topics");
    assert_eq!(landscape["title"], "演示域全景");
    let unplaced = demo["unplaced"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["file_name"] == "bogus-domain.dita")
        .expect("bogus-domain.dita in demo unplaced");
    assert_eq!(unplaced["title"], "domain 填了不存在的键");

    let coverage = json["coverage"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["domain"] == "demo")
        .expect("demo 域");
    assert_eq!(coverage["percent"], 100);
    assert_eq!(coverage["topics"], 4);
    assert!(
        coverage["outside_plan"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "dim-nonexistent")
    );

    let maturity = json["value_usage"]
        .as_array()
        .unwrap()
        .iter()
        .find(|u| u["attribute"] == "maturity")
        .expect("maturity 用量");
    assert_eq!(maturity["used"]["curated"], 4);
    assert!(
        maturity["unused"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "draft")
    );
}

/// 异常段与人读的「需要处理」是同一批事实，逐项给结构而不是拼成一句话。
#[test]
fn json_exceptions_are_structured_not_prose() {
    let json = dita_ia::json_report(&report());
    let ex = &json["exceptions"];
    assert_eq!(ex["blind_dimensions"], 0);
    assert_eq!(ex["vocab_loaded"], true);
    assert_eq!(ex["diagnostics"]["errors"], 3);
    let outside = ex["outside_plan"].as_array().unwrap();
    assert_eq!(outside[0]["domain"], "demo");
    assert!(
        outside[0]["dimensions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == "dim-nonexistent")
    );
}

/// 空叶子在人读输出里门控在 `--details` 之下（一屏放不下），JSON 里必须无条件
/// 带上：机器没有"一屏"这个问题，而随开关变形的契约不是契约。
#[test]
fn json_is_never_gated_by_details() {
    let json = dita_ia::json_report(&report());
    let leaves = json["exceptions"]["empty_leaves_by_branch"]
        .as_array()
        .unwrap();
    let total: u64 = leaves.iter().map(|l| l["count"].as_u64().unwrap()).sum();
    assert_eq!(total, 3, "{leaves:?}");
    assert!(
        !dita_ia::exception_lines(&report(), false)
            .iter()
            .any(|l| l.contains("词表空叶子"))
    );
}

/// 重复 topicref 进 JSON 的异常段，形状也进得来（同 map / 同树各一条）。
#[test]
fn json_exceptions_name_duplicate_topicrefs() {
    let json = dita_ia::json_report(&dupes_report());
    let dups = json["exceptions"]["duplicate_topicrefs"]
        .as_array()
        .unwrap();
    assert_eq!(dups.len(), 2, "{dups:?}");
    let kinds: Vec<&str> = dups.iter().map(|d| d["kind"].as_str().unwrap()).collect();
    assert!(kinds.contains(&"same_map"), "{kinds:?}");
    assert!(kinds.contains(&"same_tree"), "{kinds:?}");
    let same_tree = dups.iter().find(|d| d["kind"] == "same_tree").unwrap();
    assert_eq!(same_tree["via"].as_array().unwrap().len(), 2);
}

/// 路径相对 kb 根，map 与 topic 同一基准——绝对路径会把构建目录写进产物，
/// 两次运行就无法比对。
#[test]
fn json_paths_are_relative_to_the_kb_root() {
    let json = dita_ia::json_report(&dupes_report());
    let dups = json["exceptions"]["duplicate_topicrefs"]
        .as_array()
        .unwrap();
    for d in dups {
        let scope = d["scope"].as_str().unwrap();
        let topic = d["topic"].as_str().unwrap();
        assert!(scope.starts_with("maps/"), "{scope}");
        assert!(topic.starts_with("topics/"), "{topic}");
    }
}

/// R9 反向报表：有内容却无全景的技术域要点名——正向覆盖表只对已有全景的域
/// 发言，缺全景的域在那里连一行都没有，恰恰是 R9 要抓的静默通过。
#[test]
fn domains_with_topics_but_no_landscape_are_reported() {
    let report = report();
    // demo 有自己的全景，demo-b1 是 demo 的孙键、被祖先的全景滚算覆盖，
    // 都不该出现；not-a-subject-key 有 1 篇且无处可挂，必须出现
    assert_eq!(
        report.unlandscaped_domains,
        vec![("not-a-subject-key".to_string(), 1)]
    );
    assert!(
        dita_ia::exception_lines(&report, false)
            .iter()
            .any(|l| l.contains("无全景") && l.contains("not-a-subject-key")),
        "summary must surface the R9 violation without --details"
    );
}

/// 词表缺席时滚算无从谈起，报出来的会是降级匹配下的假阳性——整个检查跳过，
/// 与「未读到词表」的总提示一致。
#[test]
fn unlandscaped_check_is_skipped_without_vocab() {
    let report = report_without_vocab();
    assert!(report.unlandscaped_domains.is_empty());
}
