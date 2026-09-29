//! SD-36 Epic F3c3: PCGen `SUBCLASS:` lines, converted as a class choice.
//!
//! # The oracle
//!
//! - `docs/listfilepages/datafilestagpages/datafilesclasses.html` (pinned oracle checkout), "The
//!   Sub-Class Line": a line beginning `SUBCLASS:<sub-class name>`, placed between the class lines
//!   and the class level lines, carrying sub-class-line or class-line tags; "The Sub-Class Level
//!   Line": `SUBCLASSLEVEL:<level number>`, placed after its `SUBCLASS:` line, defining the
//!   sub-class's level-dependent grants ("will define a Subclass level dependent ability for the
//!   SUBCLASS immediately above it"). `CSKILL` is a global tag
//!   (`globalfilestagpages/globalfilesother.html`, `CSKILL:x|x`), so it reads the same on a
//!   sub-class line as on a class line.
//! - `pcgen/core/SubClass.java`: `SubClass extends PCClass` -- a sub-class line is a class object.
//! - `pcgen/core/analysis/SubClassApplication.checkForSubClass`: the class offers ONE pick from
//!   its `ListKey.SUB_CLASS` list, in list order, each sub-class filtered by its own
//!   prerequisites; the base class is offered at index 0 only when `ALLOWBASECLASS` holds (both
//!   SUBCLASS-bearing classes in the pinned tree state `ALLOWBASECLASS:NO`: `cr_classes.lst:277`,
//!   `up_classes.lst:216`). No token marks a default pick.
//!
//! # The rule (one mechanism, no per-class case)
//!
//! A `SUBCLASS:` row belongs to the class whose converted closure carries the `CLASS:` row it
//! follows (a base or continuation row, or a `.MOD` row -- the same rows the class record already
//! reads). For each class record with at least one:
//!
//! - one choice rule on the class record, a sibling of the principal: `<class id>#subclass`,
//!   `offers: Rules { pool: subclass, tags: [<Class> Subclass] }`, count 1, `print: false`, gated
//!   by the class record's own gates like every other line of the record (set where it is
//!   attached, `super::run`);
//! - one `subclass` rule per line, `<book>:subclass:<class>_<sub-class>`, granted by that choice
//!   (`Granter::Choice`), converted by the SAME `convert_record` every record goes through: the
//!   `SUBCLASS:` row is its base row, each `SUBCLASSLEVEL:<n>` row a level line gated at class
//!   level `n`. So `CSKILL` becomes class-skill facts, a `PRE*` its gate, and a level line's
//!   `ABILITY` a grant edge from the option (`Granter::Rule(<option>)`, gated at the level) --
//!   the shapes the converter already emits.
//! - Tokens: `CSKILL`, `CCSKILL`, `ABILITY`, `BONUS`, `AUTO`, `DEFINE`, `ADD` and prerequisites
//!   convert; `SOURCEPAGE` / `SORTKEY` are filing metadata. Every other token (`COST`,
//!   `PROHIBITCOST`, `CHOICE`, `SPELLLIST`, `KNOWNSPELLSFROMSPECIALTY`, ...) is NOT carried and is
//!   a named `_defects/subclass-token-unconverted.json` row, as is any term `convert_record`
//!   degrades.
//! - Order is oracle order: the class closure's row order (base, continuation, `.MOD`), then the
//!   lines' own order under each row.
//! - A sub-class name declared twice for one class, token-identical in two books with a
//!   `SOURCEDATE:` each, is one object printed twice: the newest printing is kept and the older
//!   one is an informational `_defects/subclass-superseded-reprint.json` row (the standing
//!   supersession ruling, `reprint.rs`). Any other repeat keeps the first and names the rest
//!   (`subclass-name-collision`).

use std::collections::{BTreeMap, BTreeSet};

use super::closure::{row_identity, tokenize_row, Closure, ClosureRow, ClosureRowKind, FileFamily, PinnedTree, RowRef, RowShape};
use super::convert::{convert_record, Converted, CONVERTER_VERSION};
use super::ctx::{own_class_id, slug, CorpusIndex, RecordRef};
use super::prose::pi_hit;
use super::reprint::VARIANT_LINE_BOOKS;
use codex::rules_core::sheet_rule::*;

/// The kind directory (and rule-id kind) a converted sub-class is written under.
pub const SUBCLASS_KIND: &str = "subclass";
/// The pool every sub-class option (and the choice's option set) names.
pub const SUBCLASS_POOL: &str = "subclass";
/// The suffix of the class record's choice sibling (`<class id>#subclass`).
pub const CHOICE_SUFFIX: &str = "subclass";

/// One `SUBCLASS:` line of the pinned tree, with the `CLASS:` row it follows and its level rows.
#[derive(Debug, Clone)]
pub struct SubclassDecl {
    pub header: RowRef,
    pub name: String,
    pub row: RowRef,
    pub level_rows: Vec<(u8, RowRef)>,
}

/// Every `SUBCLASS:` line of the class files, in tree order, plus the cites of any
/// `SUBCLASS:`/`SUBCLASSLEVEL:` row that follows no `CLASS:` / `SUBCLASS:` row (named, never
/// guessed).
pub fn declarations(tree: &PinnedTree) -> (Vec<SubclassDecl>, Vec<String>) {
    let mut out: Vec<SubclassDecl> = Vec::new();
    let mut orphans = Vec::new();
    for (fi, file) in tree.files.iter().enumerate() {
        if file.family != FileFamily::Class || file.is_pfs {
            continue;
        }
        let mut header: Option<RowRef> = None;
        let mut current: Option<usize> = None;
        for (li, raw) in file.lines.iter().enumerate() {
            let row = RowRef { file: fi, line: li + 1 };
            let upper = raw.trim_start().to_ascii_uppercase();
            if upper.starts_with("SUBCLASSLEVEL:") {
                let (name, _) = tokenize_row(raw);
                let level = name.split_once(':').and_then(|(_, n)| n.trim().parse::<u8>().ok());
                match (current, level) {
                    (Some(i), Some(l)) => out[i].level_rows.push((l, row)),
                    _ => orphans.push(tree.cite(row)),
                }
            } else if upper.starts_with("SUBCLASS:") {
                let (name, _) = tokenize_row(raw);
                let sub = name.split_once(':').map(|(_, n)| n.trim().to_string()).unwrap_or_default();
                match header {
                    Some(h) if !sub.is_empty() => {
                        out.push(SubclassDecl { header: h, name: sub, row, level_rows: Vec::new() });
                        current = Some(out.len() - 1);
                    }
                    _ => {
                        orphans.push(tree.cite(row));
                        current = None;
                    }
                }
            } else if upper.starts_with("CLASS:") {
                header = Some(row);
                current = None;
            } else if row_identity(raw).shape != RowShape::NotARecord {
                current = None;
            }
        }
    }
    (out, orphans)
}

/// One converted sub-class option: its rules (principal first) and what it hands out.
#[derive(Debug)]
pub struct SubclassOption {
    /// The class record whose choice offers it.
    pub class_record: RuleId,
    /// The option's principal rule.
    pub rule: SheetRule,
    /// Further lines the option's rows yield (`<option id>#...`), held with it.
    pub siblings: Vec<SheetRule>,
    /// Grant edges onto other records.
    pub grants_out: Vec<(RuleId, Grant)>,
    /// Everything else the conversion produced (defects, variable contributions); `rules` and
    /// `grants_out` moved out above.
    pub converted: Converted,
}

/// Every class's sub-class choice and options.
#[derive(Debug, Default)]
pub struct SubclassRun {
    /// `(class record id, choice rule)`, one per class with at least one converted option.
    pub choosers: Vec<(RuleId, SheetRule)>,
    pub options: Vec<SubclassOption>,
    /// Defect rows by `_defects/<kind>.json`.
    pub defects: BTreeMap<String, Vec<String>>,
    /// `"<superseded option id>: superseded by <kept option id>"`.
    pub superseded: Vec<String>,
}

/// The tokens of a line the option carries, and the ones it does not.
fn converts(head: &str) -> bool {
    matches!(head, "CSKILL" | "CCSKILL" | "ABILITY" | "BONUS" | "AUTO" | "DEFINE" | "ADD") || head.trim_start_matches('!').starts_with("PRE")
}

/// A level line carries only the heads `convert_record` reads on a class level line (a standalone
/// prerequisite there is not read by it, so it is named, never silently dropped).
fn converts_on_level_line(head: &str) -> bool {
    matches!(head, "CSKILL" | "ABILITY" | "BONUS" | "AUTO" | "DEFINE" | "ADD")
}

fn filing_metadata(head: &str) -> bool {
    matches!(head, "SOURCEPAGE" | "SORTKEY")
}

/// The identity of a declaration for the reprint test: its rows' tokens (`SOURCE*` aside),
/// each row sorted, the level rows keyed by level.
fn printed_identity(tree: &PinnedTree, d: &SubclassDecl) -> Vec<(u8, Vec<(String, String)>)> {
    let tokens = |r: RowRef| {
        let mut t: Vec<(String, String)> = tokenize_row(tree.row_text(r)).1.into_iter().filter(|(k, _)| !k.starts_with("SOURCE")).map(|(k, v)| (k, v.trim().to_string())).collect();
        t.sort();
        t
    };
    let mut out = vec![(0u8, tokens(d.row))];
    out.extend(d.level_rows.iter().map(|(l, r)| (*l, tokens(*r))));
    out
}

pub fn option_id(book: &str, class_slug: &str, name: &str) -> RuleId {
    format!("{book}:{SUBCLASS_KIND}:{class_slug}_{}", slug(name))
}

/// Convert every class's `SUBCLASS:` lines (module doc).
pub fn convert_subclasses(tree: &PinnedTree, index: &CorpusIndex, closures: &[Closure]) -> SubclassRun {
    let mut run = SubclassRun::default();
    let (decls, orphans) = declarations(tree);
    if !orphans.is_empty() {
        run.defects.entry("subclass-row-without-class".into()).or_default().extend(orphans);
    }
    let mut by_header: BTreeMap<RowRef, Vec<usize>> = BTreeMap::new();
    for (i, d) in decls.iter().enumerate() {
        by_header.entry(d.header).or_default().push(i);
    }
    let mut attached: BTreeSet<usize> = BTreeSet::new();
    for (record, closure) in index.records.iter().zip(closures.iter()) {
        if record.kind != "class" || !record.joined || record.rel_path.is_empty() {
            continue;
        }
        // Oracle order: the closure's own row order, then each row's lines in file order.
        let mut mine: Vec<usize> = Vec::new();
        let mut seen: BTreeSet<RowRef> = BTreeSet::new();
        for row in closure.rows.iter().filter_map(|r| r.row) {
            if seen.insert(row)
                && let Some(ids) = by_header.get(&row)
            {
                mine.extend(ids.iter().copied());
            }
        }
        if mine.is_empty() {
            continue;
        }
        attached.extend(mine.iter().copied());
        convert_class(tree, index, record, &decls, &mine, &mut run);
    }
    for (i, d) in decls.iter().enumerate() {
        if !attached.contains(&i) {
            run.defects.entry("subclass-row-without-class".into()).or_default().push(format!("{} (SUBCLASS {})", tree.cite(d.row), d.name));
        }
    }
    for v in run.defects.values_mut() {
        v.sort();
        v.dedup();
    }
    run
}

fn convert_class(tree: &PinnedTree, index: &CorpusIndex, record: &RecordRef, decls: &[SubclassDecl], mine: &[usize], run: &mut SubclassRun) {
    let class_slug = own_class_id(record);
    let class_label = record.name.clone();
    let tag = format!("{class_label} Subclass");
    let chooser_id = format!("{}#{CHOICE_SUFFIX}", record.id);

    // Reprints and collisions: group by name, keep the newest token-identical printing.
    let mut keep: Vec<usize> = Vec::new();
    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for &i in mine {
        by_name.entry(decls[i].name.to_ascii_uppercase()).or_default().push(i);
    }
    let book_of = |i: usize| tree.files[decls[i].row.file].book.clone();
    let mut dropped: BTreeSet<usize> = BTreeSet::new();
    for group in by_name.values().filter(|g| g.len() > 1) {
        let first = printed_identity(tree, &decls[group[0]]);
        let identical = group.iter().all(|&i| printed_identity(tree, &decls[i]) == first);
        let dates: Option<Vec<&String>> = group.iter().map(|&i| tree.source_dates.get(&book_of(i))).collect();
        let variant = group.iter().any(|&i| VARIANT_LINE_BOOKS.contains(&book_of(i).as_str()));
        let newest = match (identical && !variant, dates) {
            (true, Some(d)) => {
                let latest = d.iter().max().copied();
                let winners: Vec<usize> = group.iter().zip(d.iter()).filter(|(_, x)| Some(**x) == latest).map(|(i, _)| *i).collect();
                if winners.len() == 1 { Some(winners[0]) } else { None }
            }
            _ => None,
        };
        match newest {
            Some(w) => {
                for &i in group.iter().filter(|&&i| i != w) {
                    dropped.insert(i);
                    run.superseded.push(format!(
                        "{}: superseded by {} ({} vs {})",
                        option_id(&book_of(i), &class_slug, &decls[i].name),
                        option_id(&book_of(w), &class_slug, &decls[w].name),
                        tree.cite(decls[i].row),
                        tree.cite(decls[w].row)
                    ));
                }
            }
            None => {
                // Not one object printed twice: every printing in its own book keeps its own id;
                // two in ONE book cannot, and the later is named.
                let mut books: BTreeSet<String> = BTreeSet::new();
                for &i in group {
                    if !books.insert(book_of(i)) {
                        dropped.insert(i);
                        run.defects.entry("subclass-name-collision".into()).or_default().push(format!("{}: {}", record.id, tree.cite(decls[i].row)));
                    }
                }
            }
        }
    }
    keep.extend(mine.iter().copied().filter(|i| !dropped.contains(i)));

    let mut chooser_rows: Vec<String> = Vec::new();
    for i in keep {
        let d = &decls[i];
        let file = &tree.files[d.row.file];
        let id = option_id(&file.book, &class_slug, &d.name);
        if pi_hit(&d.name).is_some() {
            run.defects.entry("subclass-token-unconverted".into()).or_default().push(format!("{id}: name withheld (product identity) ({})", tree.cite(d.row)));
            continue;
        }
        let mut rows: Vec<ClosureRow> = Vec::new();
        let mut filter = |r: RowRef, kind: ClosureRowKind, level_gate: Option<u8>, run: &mut SubclassRun| {
            let (_, tokens) = tokenize_row(tree.row_text(r));
            let mut kept = Vec::new();
            for (k, v) in tokens {
                let head = k.trim();
                let carried = if kind == ClosureRowKind::LevelLine { converts_on_level_line(head) } else { converts(head) };
                if carried {
                    kept.push((k, v));
                } else if !filing_metadata(head) {
                    run.defects.entry("subclass-token-unconverted".into()).or_default().push(format!("{id}: {head} ({})", tree.cite(r)));
                }
            }
            rows.push(ClosureRow { cite: tree.cite(r), row: Some(r), tokens: kept, kind, level_gate });
        };
        filter(d.row, ClosureRowKind::Base, None, run);
        for (l, r) in &d.level_rows {
            filter(*r, ClosureRowKind::LevelLine, Some(*l), run);
        }
        let closure = Closure { rows, own_rows: BTreeSet::new(), overlay: None };
        let rec = RecordRef {
            id: id.clone(),
            book: file.book.clone(),
            kind: SUBCLASS_KIND.into(),
            name: d.name.clone(),
            key: d.name.clone(),
            category: "Subclass".into(),
            type_facet: tag.clone(),
            rel_path: file.rel_path.clone(),
            line: d.row.line,
            shipped_tokens: None,
            prerequisites: Vec::new(),
            copy_base_key: None,
            license_pi: record.license_pi,
            pi_fields: Vec::new(),
            description: None,
            class_name: Some(class_label.clone()),
            class_selection_of: None,
            joined: true,
        };
        let mut c = convert_record(tree, index, &rec, &closure);
        for (shape, under) in c.refusal_under.iter().chain(c.degraded_under.iter()) {
            for u in under {
                run.defects.entry("subclass-token-unconverted".into()).or_default().push(format!("{id}: {u} -> {shape}"));
            }
        }
        if !c.refusals.is_empty() {
            continue;
        }
        let mut rules = std::mem::take(&mut c.rules);
        if rules.is_empty() {
            continue;
        }
        let mut principal = rules.remove(0);
        principal.granted_by.push(Grant { by: Granter::Choice(chooser_id.clone()), when: Applies::Always });
        let grants_out = std::mem::take(&mut c.grants_out);
        chooser_rows.push(tree.cite(d.row));
        run.options.push(SubclassOption { class_record: record.id.clone(), rule: principal, siblings: rules, grants_out, converted: c });
    }
    if chooser_rows.is_empty() {
        return;
    }
    let chooser = SheetRule {
        id: chooser_id.clone(),
        label: tag.clone(),
        value: SheetValue::Text,
        also: Vec::new(),
        prose: Vec::new(),
        applies: Applies::Always,
        target: None,
        bonus_type: None,
        print: false,
        pool: SUBCLASS_POOL.into(),
        tags: Vec::new(),
        subject: Subject::Character,
        repeatable: false,
        granted_by: Vec::new(),
        offers: Some(Choice { id: chooser_id, count: Expr::Const(1), from: OptionSet::Rules { pool: SUBCLASS_POOL.into(), tags: vec![tag], requires: Applies::Always } }),
        grants: Vec::new(),
        closure_complete: false,
        always_held: false,
        provenance: Provenance {
            book: record.book.clone(),
            kind: record.kind.clone(),
            closure_rows: chooser_rows,
            oracle_pin: super::oracle_pin(),
            converter_version: CONVERTER_VERSION.into(),
            ..Provenance::default()
        },
    };
    run.choosers.push((record.id.clone(), chooser));
}
