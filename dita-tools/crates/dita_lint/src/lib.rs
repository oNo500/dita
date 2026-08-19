//! Per-topic content rules R12–R16, R18, R19 and R20: genre, structure, source
//! section, register, split threshold, maturity presence, upstream provenance,
//! and a genre's declared hang-off (quickstart → its domain landscape).
//!
//! R8 and R21 — whether a topic declares its sourcing state at all, and whether
//! that state permits `verified` — live in `kb/scripts/check-rules.xsl`, not
//! here. They share one literal and one judgement; splitting them across two
//! implementations is how a rule drifts out of agreement with itself.
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
//!
//! R20 is the other exception, and always an error too. What it checks is not a
//! degree of finish but the defining condition of the genre: a quickstart is
//! one path selected on a landscape, so a quickstart that hangs off nothing is
//! not an unfinished quickstart, it is a how-to wearing the wrong label. It
//! also inherits R10's severity — that Schematron rule has been an
//! unconditional error since the first ruleset, and absorbing it here must not
//! quietly relax it.

use std::{fs, path::Path};

use anyhow::Context;
use dita_diagnostics::{Diagnostic, DiagnosticBag};
use dita_upstream::NodeIndex;
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

/// One banned colloquialism plus the legitimate compounds that contain it.
///
/// Chinese has no word boundaries, so "whole-word match" does not exist
/// mechanically: scanning for 装 hits 安装/装配/封装, for 挂 hits 挂载/挂靠.
/// The scheme here is one literal to match plus a closed list of compounds in
/// which that literal reads as legitimate — a hit covered by one of them is
/// suppressed.
struct Colloquial {
    /// The literal scanned for in body text.
    word: &'static str,
    /// Compounds containing `word` in which it is legitimate. Empty when the
    /// literal has no legitimate reading of its own.
    allowed_in: &'static [&'static str],
}

/// R15's colloquial face — the machine subset of writing-style 规则四.
///
/// The rule bans five single-character stand-in verbs (拦/装/挂/塞/跑), 凑合,
/// and three colloquial particles/phrases (个 as in 挂个, 就该, 来点, 出事).
/// Only 跑 is scanned as a bare character: no legitimate compound of it occurs
/// in technical prose, and the few that exist (跑道/奔跑/…) enumerate
/// completely, so the exclusion list is closed. The other four characters have
/// open-ended legitimate compounds — a new one (装置, 集装箱, 悬挂) would
/// become a false positive the moment someone writes it — so their machine
/// face is a curated list of colloquial collocations instead. That deliberately
/// under-reports: a colloquial 装 in an unlisted collocation passes. This is the
/// intended trade — R15 declares itself a proxy, and a rule that fires on 安装
/// loses the credibility that makes the rest of it worth running.
///
/// Bare 个 stays out entirely: it is the ordinary measure word (1000+ legitimate
/// uses in this library), machine-undecidable, human review only.
const COLLOQUIAL_WORDS: [Colloquial; 14] = [
    Colloquial {
        word: "凑合",
        allowed_in: &[],
    },
    // 出事：歧义在右侧——产出事实、输出事件都是「出＋事X」的切分
    Colloquial {
        word: "出事",
        allowed_in: &["出事实", "出事件", "出事项", "出事务"],
    },
    Colloquial {
        word: "就该",
        allowed_in: &["成就该", "迁就该"],
    },
    Colloquial {
        word: "来点",
        allowed_in: &["带来点"],
    },
    // 拦：规范动词是阻断；拦截 是书面用法
    Colloquial {
        word: "拦住",
        allowed_in: &[],
    },
    Colloquial {
        word: "拦下",
        allowed_in: &[],
    },
    // 装：规范动词是安装/装配/封装，都以复合词形式出现
    Colloquial {
        word: "装上",
        allowed_in: &["安装上"],
    },
    Colloquial {
        word: "装进",
        allowed_in: &["安装进", "封装进", "组装进"],
    },
    Colloquial {
        word: "装个",
        allowed_in: &["安装个"],
    },
    // 挂：挂载/挂靠/挂钩 是书面用法
    Colloquial {
        word: "挂个",
        allowed_in: &[],
    },
    Colloquial {
        word: "挂上",
        allowed_in: &["悬挂上"],
    },
    // 塞：阻塞/堵塞 是书面用法
    Colloquial {
        word: "塞进",
        allowed_in: &[],
    },
    Colloquial {
        word: "塞满",
        allowed_in: &[],
    },
    // 跑：规范动词是运行/执行；下列复合词穷举得尽，故可扫单字
    Colloquial {
        word: "跑",
        allowed_in: &["跑道", "跑步", "奔跑", "赛跑", "长跑", "跑偏", "跑马"],
    },
];

/// How many times `entry.word` occurs outside any of its legitimate compounds.
fn colloquial_hits(text: &str, entry: &Colloquial) -> usize {
    text.match_indices(entry.word)
        .filter(|(at, _)| {
            !entry
                .allowed_in
                .iter()
                .any(|compound| covers(text, *at, entry.word, compound))
        })
        .count()
}

/// Does an occurrence of `compound` in `text` cover the hit of `word` at `at`?
fn covers(text: &str, at: usize, word: &str, compound: &str) -> bool {
    compound.match_indices(word).any(|(offset, _)| {
        at.checked_sub(offset)
            .and_then(|start| text.get(start..start + compound.len()))
            == Some(compound)
    })
}
/// Inline markup in which a banned word is mentioned rather than used: a rule
/// quoting the words it bans is not violating itself.
const MENTION_MARKUP: [&str; 6] = [
    "codeph",
    "codeblock",
    "xmlelement",
    "xmlatt",
    "term",
    "keyword",
];

/// Everything `node` says in its own voice: descendant text minus whatever sits
/// inside `MENTION_MARKUP`.
fn used_text(node: roxmltree::Node) -> String {
    node.descendants()
        .filter(roxmltree::Node::is_text)
        .filter(|t| {
            !t.ancestors()
                .any(|a| MENTION_MARKUP.contains(&a.tag_name().name()))
        })
        .filter_map(|n| n.text())
        .collect()
}

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

/// R19: the domains whose upstream is indexed. The rule ("every topic declares
/// which upstream node it corresponds to") is library-wide; the enforcement is
/// staged, because it needs an index and only `dita` has one so far (design
/// §七之二). A domain absent from this list is skipped, not passed — adding a
/// branch means adding a crawler and a `bm-*` registry entry, not touching this
/// file's logic.
const INDEXED_DOMAINS: [&str; 1] = ["dita"];
/// R19: the value that says "no upstream node exists for this one".
///
/// This is 标题自造 — the upstream knowledge tree has no node by this *name*.
/// It is not 无外部来源, R8's judgement, which is about the body's assertions
/// having no external provenance. The two are orthogonal: most of the topics
/// declaring `coined` do cite sources for their facts, and a topic whose title
/// comes straight from upstream may cite nothing at all. The rules read each
/// other wrong the moment the two are called by one name, so neither this file
/// nor `rules.sch` writes a bare 「自造」 (terminology-rules 边界 has the split).
const COINED: &str = "coined";
/// R19: the three gates a coined title has to have passed (naming-rules).
/// Their names have to appear in the file header comment — the argument itself
/// is prose and stays a human read, but its absence is machine-visible.
const COINED_GATES: [&str; 3] = ["穷尽查证", "先怀疑切分", "只组合不发明"];

/// Punctuation and whitespace dropped before looking for a gate's name.
///
/// The library really does write 「只组合，不发明」 with a comma, and 「先怀疑
/// 切分」 across a line break. Both are the gate, written naturally; a literal
/// substring search would report them as missing. Same rule as the index
/// comparison: normalize both sides, then match exactly.
fn condensed(text: &str) -> String {
    text.chars()
        .filter(|c| {
            !c.is_whitespace() && !"，。、；：,.;:「」（）()【】“”\"'—-…·《》<>".contains(*c)
        })
        .collect()
}

/// Lint one topic file against R12–R16, R18 and R19.
///
/// `upstream` is `None` when the node index could not be loaded. R19 then does
/// not run at all — and the caller must report it as 未执行, never as passing.
/// Treating a missing index as "nothing resolves" would report the index's own
/// failure as 66 authoring errors.
///
/// # Errors
///
/// Returns `Err` only when the file cannot be read or is not well-formed XML.
pub fn lint_topic(
    path: &Path,
    vocab: &Vocabulary,
    upstream: Option<&NodeIndex>,
) -> anyhow::Result<DiagnosticBag> {
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

    check_hangs_off(root, path, vocab, &mut diag);

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
    if let Some(index) = upstream {
        check_upstream_node(root, &header_comment(&doc), index, &mut push);
    }

    Ok(diag)
}

/// The file header comment: everything commented outside the root element.
/// That is where each topic's provenance record lives (a convention the whole
/// library follows), and it is what R19 reads for the three gates.
fn header_comment(doc: &roxmltree::Document) -> String {
    doc.root()
        .children()
        .filter(roxmltree::Node::is_comment)
        .filter_map(|n| n.text())
        .collect()
}

/// One topic's `prolog/data[@name=…]` values, in document order.
fn prolog_data<'a>(root: roxmltree::Node<'a, 'a>, name: &str) -> Vec<&'a str> {
    root.descendants()
        .filter(|n| n.has_tag_name("data") && n.attribute("name") == Some(name))
        .filter_map(|n| n.attribute("value"))
        .collect()
}

/// R19: declarative provenance. Not "does the title look like an upstream node"
/// — Chinese titles against English node names would be all noise (design §二)
/// — but "does this topic say which node it corresponds to, and does that
/// declaration resolve".
///
/// Three things are machine-checkable here, and the third is the point: whether
/// the declared name exists, whether a coined title carries its argument, and
/// **which declarations go dangling after upstream renames something**.
///
/// A topic may declare several nodes (組合篇 like `key space 与 key scope`);
/// each is checked on its own and each miss is reported on its own.
fn check_upstream_node(
    root: roxmltree::Node,
    header: &str,
    index: &NodeIndex,
    push: &mut impl FnMut(String),
) {
    let domain = prolog_data(root, "domain");
    if !domain.iter().any(|d| INDEXED_DOMAINS.contains(d)) {
        return; // 分支的上游还没建索引：跳过，不是通过（设计七之二）
    }

    let declared = prolog_data(root, "upstream-node");
    if declared.is_empty() {
        push(
            "R19：缺 upstream-node 声明——dita 域 topic 必须写明标题取自上游哪个节点\
             （prolog data name=\"upstream-node\" value=\"节点标题原文\"）；\
             标题自造的写 value=\"coined\"（指上游无此节点名，与 R8 的「无外部来源」是两件事），\
             并在文件头注释写明三道关"
                .to_string(),
        );
        return;
    }

    for value in declared {
        if value.trim().eq_ignore_ascii_case(COINED) {
            let header = condensed(header);
            let missing: Vec<&str> = COINED_GATES
                .iter()
                .copied()
                .filter(|gate| !header.contains(&condensed(gate)))
                .collect();
            if !missing.is_empty() {
                push(format!(
                    "R19：声明 coined（标题自造）但文件头注释缺三道关说明「{}」\
                     ——标题自造得留下论证，否则与「查不到就写 coined」无从区分",
                    missing.join("」「")
                ));
            }
        } else if !index.contains(value) {
            push(format!(
                "R19：upstream-node「{value}」在上游节点索引中解析不到。三种可能，\
                 逐条排除后再改：①声明拼写有误（节点标题须逐字照抄上游原文）；\
                 ②上游已改名或删除——这正是本规则要暴露的漂移，属实则改标题与声明；\
                 ③索引未收录该节点（resource-only 子树、未随发行版发布的页面、\
                 conref 素材片段都不在内），请核对索引头：{}",
                index.provenance()
            ));
        }
    }
}

/// R20: a genre's declared hang-off, resolved across documents.
///
/// The subject scheme is the only place that says which genres hang off which
/// — `hangs-off-genre` on the depending genre (`quickstart` → `tech-landscape`).
/// Nothing here carries that pair, the same discipline the rest of this file
/// follows: no value list in Rust, ever.
///
/// Two things are then checkable, and neither is a matter of finish, so both
/// report as errors regardless of `@maturity`:
///
/// - **挂靠**: some `xref` in the topic has to resolve to a real file whose
///   genre is the one declared, and that file has to belong to the same domain.
///   R10 asked only "is there any xref at all", which any link satisfied; this
///   opens the target and looks.
/// - **取舍**: the topic has to say which of that landscape's planned
///   dimensions it covers, by declaring `@dimension`, and the set has to be a
///   *proper* subset of the plan. Covering the whole plan is not a selection —
///   that topic is the landscape, not a path across it.
///
/// The skipped dimensions are deliberately **not** listed anywhere: they are
/// the complement of a declared set against a declared plan, and hand-copying
/// a derivable set is the defect this library computes its way out of
/// everywhere else (`just ia` does exactly this subtraction). What the author
/// owes is the choice and its reasons — the choice as `@dimension`, the reasons
/// as prose in the 取舍 section, whose presence is R13's job because its title
/// lives in the vocabulary, not here.
fn check_hangs_off(
    root: roxmltree::Node,
    path: &Path,
    vocab: &Vocabulary,
    diag: &mut DiagnosticBag,
) {
    let Some(genre) = root.attribute("outputclass") else {
        return;
    };
    let Some(want) = vocab
        .subject(genre)
        .and_then(|s| s.data.get("hangs-off-genre"))
    else {
        return; // this genre hangs off nothing — R20 does not apply
    };
    let mut err = |msg: String| diag.push(Diagnostic::error(path, msg));

    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut tried: Vec<String> = Vec::new();
    let mut target = None;
    for href in local_hrefs(root) {
        let candidate = dir.join(&href);
        match TopicFacts::read(&candidate) {
            Some(facts) if facts.genre.as_deref() == Some(want.as_str()) => {
                target = Some(facts);
                break;
            }
            _ => tried.push(href),
        }
    }

    let Some(landscape) = target else {
        err(format!(
            "R20：{genre} 缺挂靠——正文里没有一个 xref 指向体裁为 {want} 的篇（本域全景）。\
             {genre} 是在完整维度框架上选出的一条路径，不挂靠框架就无从判断它略过了什么。\
             已解析的库内 xref：{}",
            if tried.is_empty() {
                "无".to_string()
            } else {
                tried.join("、")
            }
        ));
        return;
    };

    let domain = prolog_data(root, "domain");
    if !domain.is_empty()
        && !landscape.domain.is_empty()
        && !domain
            .iter()
            .any(|d| landscape.domain.contains(&(*d).to_string()))
    {
        err(format!(
            "R20：挂靠的全景属域「{}」，与本篇的域「{}」不符——{genre} 要挂的是自己所属领域的全景，\
             挂到别域会让取舍声明对着一份无关的维度清单做",
            landscape.domain.join("、"),
            domain.join("、")
        ));
    }

    let covered: Vec<&str> = root
        .attribute("dimension")
        .unwrap_or_default()
        .split_whitespace()
        .collect();
    if covered.is_empty() {
        err(format!(
            "R20：{genre} 缺取舍声明——根元素必须标 @dimension，声明本篇覆盖了全景规划清单里的哪几个维度。\
             略过的那些不必逐条抄写，它们是规划清单与本声明的差集，由 dita-tools ia 算出"
        ));
        return;
    }

    let outside: Vec<&str> = covered
        .iter()
        .copied()
        .filter(|d| !landscape.planned.iter().any(|p| p == d))
        .collect();
    if !outside.is_empty() {
        err(format!(
            "R20：覆盖声明里的「{}」不在全景的规划清单内——要么本篇标错了维度，\
             要么全景漏登记了这一维（两种都会让覆盖度算不准，先定是哪一种再改）",
            outside.join("」「")
        ));
    }

    let skipped: Vec<&String> = landscape
        .planned
        .iter()
        .filter(|p| !covered.iter().any(|c| c == p))
        .collect();
    if skipped.is_empty() {
        err(format!(
            "R20：本篇覆盖了全景规划的全部 {} 个维度，没有略过任何一维——那不是取舍。\
             按解锁度与使用频率选出少数几维，其余留给全景",
            landscape.planned.len()
        ));
    }
}

/// Local (non-external) `.dita` targets of a topic's `xref`s, fragment
/// stripped, in document order.
fn local_hrefs(root: roxmltree::Node) -> Vec<String> {
    root.descendants()
        .filter(|n| n.has_tag_name("xref"))
        .filter(|n| n.attribute("scope") != Some("external"))
        .filter_map(|n| n.attribute("href"))
        .map(|h| h.split('#').next().unwrap_or(h).to_string())
        .filter(|h| {
            Path::new(h)
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("dita"))
        })
        .collect()
}

/// The few facts R20 needs from a referenced topic, read from its own file.
///
/// A second parse rather than a corpus pass: the only thing being followed is
/// one `xref` the topic itself wrote, so opening that file is the whole job.
struct TopicFacts {
    genre: Option<String>,
    domain: Vec<String>,
    planned: Vec<String>,
}

impl TopicFacts {
    /// `None` when the file is missing or not well-formed — the caller reports
    /// that as "no hang-off found" together with what it tried, since a broken
    /// or dangling reference is exactly one of the ways a hang-off fails.
    fn read(path: &Path) -> Option<Self> {
        let xml = fs::read_to_string(path).ok()?;
        let opts = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        let doc = roxmltree::Document::parse_with_options(&xml, opts).ok()?;
        let root = doc.root_element();
        Some(Self {
            genre: root.attribute("outputclass").map(str::to_string),
            domain: prolog_data(root, "domain")
                .into_iter()
                .map(str::to_string)
                .collect(),
            planned: prolog_data(root, "planned-dimension")
                .into_iter()
                .map(str::to_string)
                .collect(),
        })
    }
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

/// Title proxies for the naming rules: a title is a name, not a claim. Full
/// judgement (locating term, upstream category alignment) stays human.
///
/// The colon is banned outright rather than judged. "Is this colon a subtitle?"
/// would need the half of the naming rule no machine can evaluate — whether
/// what stands before the colon already names the node on its own — and a
/// proxy that guesses at that would misfire, which is the one thing that gets
/// a rule ignored (the same reasoning R19's normalisation ruling rests on).
/// Banned outright, the rule is simply true: the enumeration or gloss that
/// follows a colon belongs in `shortdesc` and the body's opening sentence,
/// never in the name. 2026-08 measured 14 of 22 topics carrying one, and two
/// rounds of hand-renaming were what cleared them.
///
/// Only the full-width `：` is scanned. A half-width colon occurs inside
/// identifiers (`xml:lang`, a URL scheme), where it is not punctuation at all,
/// so banning it would misfire on exactly the titles the naming rule wants —
/// the ones that keep the upstream mechanism name. This library's titles are
/// Chinese, and its subtitle disease is written full-width every time. The gap
/// is real and deliberate: a subtitle spelled with `:` passes, and human review
/// catches it — the same under-reporting trade R15's colloquial list makes.
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
    for (pat, why) in [
        ("？", "问句"),
        ("，不是", "论断句式"),
        ("——", "破折号"),
        (
            "：",
            "冒号副标题（冒号后的枚举与说明写进 shortdesc 与正文首句）",
        ),
    ] {
        if title.contains(pat) {
            push(format!(
                "标题「{title}」含{why}——标题是专业命名（naming-rules 标题规则）"
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

/// R14: the source section carries no paragraph labels at all, no legacy
/// label, no prose dates — the date lives once, in prolog's reviewed.
///
/// Until 2026-08-19 this rule demanded the opposite: two labelled paragraphs,
/// 事实 and 判断, empty-but-never-absent, together exhausting every assertion
/// in the body. The user struck that down. The section now lists only what
/// actually has a source; everything else is this library's own judgement by
/// default, claimed nowhere. With nothing left to divide, the labels have no
/// job, and their absence is what this checks.
///
/// The machine face is "no `b` element in the section", not "the characters
/// 事实 must not appear". The latter would misfire on ordinary prose (「四个
/// 实例各自的事实……」); the former reads a marker, not a word, and has no
/// ambiguity to resolve. Whether the source section *exists* and which of the
/// two shapes it takes is R8's judgement, not this one's — one capability,
/// one place.
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

    let labels: Vec<String> = section
        .descendants()
        .filter(|n| n.has_tag_name("b"))
        .map(|b| {
            b.descendants()
                .filter(roxmltree::Node::is_text)
                .filter_map(|n| n.text())
                .collect::<String>()
                .trim()
                .to_string()
        })
        .collect();
    if !labels.is_empty() {
        push(format!(
            "R14：来源节含段标签「{}」——两段划分（事实／判断）与穷尽归属已于 2026-08-19 废止，\
             本节只列有来源的条目，其余默认为本库判断，不再逐条认领",
            labels.join("」「")
        ));
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
///
/// Scope is the body plus `shortdesc`; `check_shortdesc` records which
/// sub-checks carry over to the latter and which do not.
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
        // 来源节自 2026-08-19 起一个段标签都不留（R14），故这里的上限 2 对它是死条款；
        // 保留统一上限而不为来源节特判，是因为两条查的东西不同：R14 查标记在不在，
        // 本条查粗体有没有被当语气用，重叠的那一格由 R14 先报，消息也更准。
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

    let body: String = root
        .descendants()
        .filter(|n| matches!(n.tag_name().name(), "conbody" | "refbody" | "taskbody"))
        .map(used_text)
        .collect();
    check_words(&body, "", push);

    check_shortdesc(root, push);
}

/// R15's word face — degree words and colloquialisms in one stretch of used
/// (not mentioned) text. `at` names where the text came from, so a hit in a
/// shortdesc reads as one without re-scanning the file; the body passes `""`.
fn check_words(text: &str, at: &str, push: &mut impl FnMut(String)) {
    for word in DEGREE_WORDS {
        let n = text.matches(word).count();
        if n > 0 {
            push(format!(
                "R15：{at}程度词「{word}」出现 {n} 次——判断的强度由理由撑，不由副词撑"
            ));
        }
    }
    for entry in &COLLOQUIAL_WORDS {
        let n = colloquial_hits(text, entry);
        if n > 0 {
            let word = entry.word;
            push(format!(
                "R15：{at}口语词「{word}」出现 {n} 次——文档语体用书面表达"
            ));
        }
    }
}

/// R15 on `shortdesc`. The body's register is read by whoever opens the topic;
/// a shortdesc is extracted and reused — link previews, map TOC entries,
/// retrieval summaries — so one colloquialism in it surfaces once per inbound
/// link, everywhere the topic is referenced. Same rules, wider blast radius.
///
/// Which sub-checks carry over, and why not all of them:
///
/// - 口语词 / 程度词: the register rules are about words, and a shortdesc is
///   prose like any other. Straight carry-over, mention-exclusion included.
/// - 破折号: writing-style 规则三 caps insertions at one per 段 and a shortdesc
///   is exactly one 段 — the same threshold applies unchanged. No tightening is
///   invented here: the number comes from the 人读正本, not from this file.
/// - 粗体: the body cap ("每节至多 2") is quantified per 节 and a shortdesc is
///   not a 节, so the number does not transfer. What transfers is the rule's
///   qualitative half — 粗体只标判据与警示. A summary labels neither, so any
///   bold in it is emphasis, i.e. 语气. Banned outright; the library already
///   holds to this (0 occurrences in 89 shortdescs), the check just locks it.
/// - 标题模式（？/「，不是」/破折号）: that check reads `title` and proxies the
///   naming rules, which govern names, not summaries. Not applicable.
fn check_shortdesc(root: roxmltree::Node, push: &mut impl FnMut(String)) {
    let Some(shortdesc) = root.children().find(|c| c.has_tag_name("shortdesc")) else {
        return; // presence is R1's job, not ours
    };

    let bold = shortdesc
        .descendants()
        .filter(|n| n.has_tag_name("b"))
        .count();
    if bold > 0 {
        push(format!(
            "R15：shortdesc 有 {bold} 处粗体——摘要无判据与警示可标，粗体在这里只剩语气（正文每节至多 2，shortdesc 为 0）"
        ));
    }

    let text = used_text(shortdesc);
    let dashes = text.matches("——").count();
    if dashes > 1 {
        push(format!(
            "R15：shortdesc 有 {dashes} 处破折号插入（单段，至多 1）——它会被抽去做链接预览与目录说明"
        ));
    }

    check_words(&text, "shortdesc 的", push);
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

#[cfg(test)]
mod tests {
    use super::{COLLOQUIAL_WORDS, Colloquial, check_register, colloquial_hits};

    /// R15's messages for one topic, in report order.
    fn register_of(xml: &str) -> Vec<String> {
        let doc = roxmltree::Document::parse(xml).expect("用例必须是良构 XML");
        let mut msgs = Vec::new();
        check_register(doc.root_element(), &mut |m| msgs.push(m));
        msgs
    }

    /// The shortdesc is extracted and reused, so its register is checked too —
    /// a colloquialism there is not shielded by sitting outside the body.
    #[test]
    fn shortdesc_colloquialism_is_reported() {
        let msgs = register_of(
            r#"<concept id="t"><title>切块</title>
               <shortdesc>先跑一次构建，再决定切法。</shortdesc>
               <conbody><p>切块单位是 topic。</p></conbody></concept>"#,
        );
        assert_eq!(msgs.len(), 1, "应恰好报一处：{msgs:?}");
        assert!(msgs[0].contains("shortdesc"), "须点明位置：{}", msgs[0]);
        assert!(msgs[0].contains("跑"), "须点明词：{}", msgs[0]);
    }

    /// Mention is not use — the exclusion that spares a quoted ban in the body
    /// spares it in the shortdesc on the same terms.
    #[test]
    fn shortdesc_mention_inside_keyword_is_spared() {
        let msgs = register_of(
            r#"<concept id="t"><title>用词</title>
               <shortdesc>技术动作用规范动词，不用 <keyword>跑</keyword> 一类口语替身。</shortdesc>
               <conbody><p>规范动词是执行。</p></conbody></concept>"#,
        );
        assert!(msgs.is_empty(), "keyword 内是提及不是使用：{msgs:?}");
    }

    /// The body's "至多 2" is quantified per 节 and a shortdesc is not one; what
    /// carries over is 粗体只标判据与警示, and a summary labels nothing.
    #[test]
    fn shortdesc_bold_is_banned_outright() {
        let msgs = register_of(
            r#"<concept id="t"><title>切块</title>
               <shortdesc>切块单位是 <b>topic</b>。</shortdesc>
               <conbody><p>正文。</p></conbody></concept>"#,
        );
        assert_eq!(msgs.len(), 1, "应恰好报一处：{msgs:?}");
        assert!(msgs[0].contains("粗体"), "{}", msgs[0]);
    }

    /// A shortdesc is one 段, so it gets the one-insertion allowance a body
    /// paragraph gets — no more, and no tightening beyond the 人读正本.
    #[test]
    fn shortdesc_allows_one_dash_and_reports_two() {
        let one = register_of(
            r#"<concept id="t"><title>切块</title>
               <shortdesc>切块单位是 topic——过大时二级切分。</shortdesc>
               <conbody><p>正文。</p></conbody></concept>"#,
        );
        assert!(one.is_empty(), "单段一处破折号合规：{one:?}");
        let two = register_of(
            r#"<concept id="t"><title>切块</title>
               <shortdesc>切块单位是 topic——过大时二级切分——过小时整块入库。</shortdesc>
               <conbody><p>正文。</p></conbody></concept>"#,
        );
        assert_eq!(two.len(), 1, "应恰好报一处：{two:?}");
        assert!(two[0].contains("破折号"), "{}", two[0]);
    }

    fn entry(word: &str) -> &'static Colloquial {
        COLLOQUIAL_WORDS
            .iter()
            .find(|c| c.word == word)
            .unwrap_or_else(|| panic!("{word} 不在 COLLOQUIAL_WORDS 里"))
    }

    /// Every word of writing-style 规则四 that carries a machine face: one
    /// sentence that must fire, one legitimate compound that must not.
    #[test]
    fn each_colloquial_fires_and_spares_its_legitimate_compound() {
        let cases = [
            ("凑合", "先凑合用着", "尚无合法复合词，此处为对照句"),
            ("出事", "出事之后再补", "这一步不会影响产出事实"),
            ("就该", "缺一项就该报错", "这项成就该领域的共识"),
            ("来点", "这里来点例子", "重构带来点滴改进"),
            ("拦住", "把非法值拦住", "审查未能拦截它"),
            ("拦下", "缺一项即被机器拦下", "拦截的前提是先认出机制名"),
            ("装上", "把过滤器装上", "安装上游依赖之后再构建"),
            ("装进", "一条内容装进哪层", "把逻辑封装进模块"),
            ("装个", "先装个插件试试", "只安装个别插件"),
            ("挂个", "挂个钩子上去", "挂载点的选择"),
            ("挂上", "把 keyref 挂上", "悬挂上方的标签"),
            ("塞进", "把字段塞进元数据", "阻塞写作与构建"),
            ("塞满", "把参数塞满", "阻塞的成因"),
            ("跑", "跑一次构建", "跑道与奔跑都不是技术动作"),
        ];
        assert_eq!(cases.len(), COLLOQUIAL_WORDS.len(), "词表与用例须一一对应");
        for (word, hit, spare) in cases {
            let e = entry(word);
            assert_eq!(colloquial_hits(hit, e), 1, "「{word}」应命中：{hit}");
            assert_eq!(colloquial_hits(spare, e), 0, "「{word}」误报：{spare}");
        }
    }

    #[test]
    fn repeated_hits_are_counted() {
        assert_eq!(colloquial_hits("先跑一次，再跑一次", entry("跑")), 2);
    }

    /// The exclusion must not swallow a real hit sitting next to a legitimate
    /// compound in the same sentence.
    #[test]
    fn exclusion_is_positional_not_wholesale() {
        assert_eq!(
            colloquial_hits("安装上游依赖之后，把过滤器装上", entry("装上")),
            1
        );
    }
}
