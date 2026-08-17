//! Per-topic content rules R12–R16 and R18: genre, structure, source section,
//! register, split threshold, maturity presence.
//!
//! Spec lives in `kb/schema/rules.sch`; genre values and their metadata live in
//! the subject scheme — nothing here carries a value list of its own.
//!
//! Severity follows maturity: a draft gets warnings, curated and verified get
//! errors. That makes this lint the promotion gate — a draft is free to be
//! unfinished, and claiming curated means passing.
//!
//! R18 is the exception: it always reports as an error, regardless of the
//! topic's maturity. Grading it by maturity would be circular — the grading
//! itself reads `@maturity`, and the violation R18 catches is that attribute's
//! absence. It also runs on `glossentry`, which every other check here skips
//! (their genre/structure/register model doesn't apply to a glossentry's fixed
//! shape) — R18's context matches R2's in `rules.sch`, not `CONTENT_ROOTS`.

use std::{fs, path::Path};

use anyhow::Context;
use dita_diagnostics::{Diagnostic, DiagnosticBag};
use dita_vocab::Vocabulary;

const CONTENT_ROOTS: [&str; 4] = ["concept", "task", "reference", "troubleshooting"];
/// R18's context: same as R2's in `rules.sch` — every content topic type plus
/// `glossentry`, which `CONTENT_ROOTS` deliberately excludes (its genre and
/// structure checks don't apply there).
const MATURITY_REQUIRED_ROOTS: [&str; 5] = [
    "concept",
    "task",
    "reference",
    "troubleshooting",
    "glossentry",
];
/// Genre is what carries a fixed structure; these types have no plain form.
const GENRE_REQUIRED_ROOTS: [&str; 2] = ["concept", "task"];
const DEGREE_WORDS: [&str; 5] = ["特别", "极其", "恰恰", "真正的", "最危险"];
/// Colloquialisms with no place in documentation register. A proxy like the
/// degree words: the full judgement stays human, these catch the frequent
/// offenders.
const COLLOQUIAL_WORDS: [&str; 6] = ["凑合", "挂个", "塞进", "出事", "拦住", "就该"];
/// R16: implementation-layer inline markup counted toward the split threshold.
/// xmlelement/xmlatt/filepath are excluded — the first two are this library's
/// subject matter when writing about DITA, the third is mostly illustrative.
const IMPL_MARKUP: [&str; 10] = [
    "codeph",
    "cmdname",
    "apiname",
    "parmname",
    "varname",
    "option",
    "userinput",
    "systemoutput",
    "synph",
    "codeblock",
];
const SPLIT_THRESHOLD: usize = 8;

/// Lint one topic file against R12–R16 and R18.
///
/// # Errors
///
/// Returns `Err` only when the file cannot be read or is not well-formed XML.
pub fn lint_topic(path: &Path, vocab: &Vocabulary) -> anyhow::Result<DiagnosticBag> {
    let mut diag = DiagnosticBag::default();
    let xml = fs::read_to_string(path)
        .with_context(|| format!("cannot read file: {}", path.display()))?;
    let opts = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
    let doc = roxmltree::Document::parse_with_options(&xml, opts)
        .with_context(|| format!("XML parse error in: {}", path.display()))?;
    let root = doc.root_element();
    let root_name = root.tag_name().name();

    check_maturity_required(root, root_name, path, &mut diag);

    if !CONTENT_ROOTS.contains(&root_name) {
        return Ok(diag); // glossentry (R18 already checked above) and unknown roots are out of scope
    }

    // drafts warn, curated/verified error — the promotion gate
    let strict = matches!(root.attribute("maturity"), Some("curated" | "verified"));
    let mut push = |msg: String| {
        if strict {
            diag.push(Diagnostic::error(path, msg));
        } else {
            diag.push(Diagnostic::warning(path, msg));
        }
    };

    check_title(root, &mut push);
    check_genre(root, root_name, vocab, &mut push);
    check_source_section(root, &mut push);
    check_register(root, &mut push);
    check_split_threshold(root, root_name, &mut push);

    Ok(diag)
}

/// R18: an unmet default. The filter that keeps drafts out of deliverables
/// (`kb/filters/*.ditaval`) is a condition on the literal value of `@maturity`
/// — a topic that never wrote the attribute doesn't match `val="draft"`, so it
/// doesn't match the exclude either, and slips through unreviewed. The
/// vocabulary's "unset counts as draft" is a semantic default for validation
/// and prose, not something DITAVAL can see. This closes that gap by making
/// the attribute mandatory rather than defaulted — always an error, since the
/// thing missing is the one the draft/curated grading itself reads.
fn check_maturity_required(
    root: roxmltree::Node,
    root_name: &str,
    path: &Path,
    diag: &mut DiagnosticBag,
) {
    if MATURITY_REQUIRED_ROOTS.contains(&root_name) && root.attribute("maturity").is_none() {
        diag.push(Diagnostic::error(
            path,
            format!(
                "R18：{root_name} 必须显式标 @maturity（未标注不匹配交付物过滤的 exclude 条件，会从成熟度门下漏过去，与 R2 对称）"
            ),
        ));
    }
}

/// Title proxies for rule seven: a title is a name, not a claim. Full
/// judgement (locating term, upstream category alignment) stays human.
fn check_title(root: roxmltree::Node, push: &mut impl FnMut(String)) {
    let title: String = root
        .children()
        .find(|c| c.has_tag_name("title"))
        .map(|t| {
            t.descendants()
                .filter(roxmltree::Node::is_text)
                .filter_map(|n| n.text())
                .collect()
        })
        .unwrap_or_default();
    for (pat, why) in [("？", "问句"), ("，不是", "论断句式"), ("——", "悬念破折号")]
    {
        if title.contains(pat) {
            push(format!(
                "标题「{title}」含{why}——标题是专业命名（writing-style 规则七）"
            ));
        }
    }
}

/// R12 + R13: genre present where required, legal, type-matched, structure complete.
fn check_genre(
    root: roxmltree::Node,
    root_name: &str,
    vocab: &Vocabulary,
    push: &mut impl FnMut(String),
) {
    let genre = root.attribute("outputclass");
    let Some(genre) = genre else {
        if GENRE_REQUIRED_ROOTS.contains(&root_name) {
            push(format!(
                "R12：{root_name} 必须标体裁 @outputclass（固定结构靠它）"
            ));
        }
        return;
    };

    let Some(subject) = vocab.subject(genre) else {
        push(format!(
            "R12：@outputclass \"{genre}\" 不在词表 genre-values 内"
        ));
        return;
    };

    if let Some(want) = subject.data.get("dita-type") {
        if want != root_name {
            push(format!(
                "R12：体裁 \"{genre}\" 属 {want}，不能标在 {root_name} 上"
            ));
        }
    }

    // R13: required sections, matched by title prefix ("做法：四条" matches "做法")
    let required = subject.data_all("required-section");
    if required.is_empty() {
        return;
    }
    let titles: Vec<String> = root
        .descendants()
        .filter(|n| n.has_tag_name("section"))
        .filter_map(|s| {
            s.children()
                .find(|c| c.has_tag_name("title"))
                .and_then(|t| t.text())
                .map(str::to_string)
        })
        .collect();
    for need in required {
        if !titles.iter().any(|t| t.starts_with(need.as_str())) {
            push(format!("R13：体裁 \"{genre}\" 缺必需节「{need}」"));
        }
    }
}

/// R14: the source section carries the 事实/判断 labels, no legacy label,
/// no prose dates — the date lives once, in prolog's reviewed.
fn check_source_section(root: roxmltree::Node, push: &mut impl FnMut(String)) {
    let Some(section) = root
        .descendants()
        .filter(|n| n.has_tag_name("section"))
        .find(|s| {
            s.children()
                .find(|c| c.has_tag_name("title"))
                .and_then(|t| t.text())
                == Some("来源")
        })
    else {
        return; // source presence is R8's job, not ours
    };

    let labels: Vec<&str> = section
        .descendants()
        .filter(|n| n.has_tag_name("b"))
        .filter_map(|b| b.text())
        .collect();
    for want in ["事实", "判断"] {
        if !labels.iter().any(|l| l.trim() == want) {
            push(format!("R14：来源节缺「{want}」段标签（可空不可省）"));
        }
    }
    let text: String = section
        .descendants()
        .filter(roxmltree::Node::is_text)
        .filter_map(|n| n.text())
        .collect();
    if text.contains("已核对") {
        push("R14：来源节含旧标签「已核对」，应改为「事实」".to_string());
    }
    if text.contains("（20") || text.contains("(20") {
        push("R14：来源节疑似手写日期——日期唯一存放处是 prolog data name=\"reviewed\"".to_string());
    }
}

/// R15: register proxies — always approximations, and the spec says so.
/// Aphorisms and staged openings stay a human call.
fn check_register(root: roxmltree::Node, push: &mut impl FnMut(String)) {
    // bold per section, source-section labels excluded via its two allowed
    for section in root.descendants().filter(|n| n.has_tag_name("section")) {
        let title = section
            .children()
            .find(|c| c.has_tag_name("title"))
            .and_then(|t| t.text())
            .unwrap_or("?");
        let bold = section
            .descendants()
            .filter(|n| n.has_tag_name("b"))
            .count();
        // 上限对来源节同样是 2：恰好容纳「事实」「判断」两个段标签
        if bold > 2 {
            push(format!(
                "R15：节「{title}」有 {bold} 处粗体（上限 2）——粗体不承担语气"
            ));
        }
    }

    for p in root.descendants().filter(|n| n.has_tag_name("p")) {
        let text: String = p
            .descendants()
            .filter(roxmltree::Node::is_text)
            .filter_map(|n| n.text())
            .collect();
        let dashes = text.matches("——").count();
        if dashes > 1 {
            let head: String = text.chars().take(18).collect();
            push(format!(
                "R15：段「{head}…」有 {dashes} 处破折号插入（每段至多 1）"
            ));
        }
    }

    // mention is not use: a rule quoting the words it bans (inside codeph and
    // similar markup contexts) is not violating itself
    let body: String = root
        .descendants()
        .filter(|n| matches!(n.tag_name().name(), "conbody" | "refbody" | "taskbody"))
        .flat_map(|b| b.descendants().filter(roxmltree::Node::is_text))
        .filter(|t| {
            !t.ancestors().any(|a| {
                matches!(
                    a.tag_name().name(),
                    "codeph" | "codeblock" | "xmlelement" | "xmlatt" | "term" | "keyword"
                )
            })
        })
        .filter_map(|n| n.text())
        .collect();
    for word in DEGREE_WORDS {
        let n = body.matches(word).count();
        if n > 0 {
            push(format!(
                "R15：程度词「{word}」出现 {n} 次——判断的强度由理由撑，不由副词撑"
            ));
        }
    }
    for word in COLLOQUIAL_WORDS {
        let n = body.matches(word).count();
        if n > 0 {
            push(format!(
                "R15：口语词「{word}」出现 {n} 次——文档语体用书面表达"
            ));
        }
    }
}

/// R16: a concept carrying more than the threshold of implementation markup is
/// judgement and configuration living in one topic — split per convention 3.
fn check_split_threshold(root: roxmltree::Node, root_name: &str, push: &mut impl FnMut(String)) {
    if root_name != "concept" {
        return;
    }
    let n = root
        .descendants()
        .filter(|d| IMPL_MARKUP.contains(&d.tag_name().name()))
        .count();
    if n > SPLIT_THRESHOLD {
        push(format!(
            "R16：concept 含 {n} 处实现层标记（上限 {SPLIT_THRESHOLD}）——判据留 concept，清单/语法/字段迁成 reference"
        ));
    }
}
