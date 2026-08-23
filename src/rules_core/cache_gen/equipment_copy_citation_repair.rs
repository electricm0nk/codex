//! Repairs a shipped `equipment`/`equipment_modifier` record's stale
//! `.COPY=` citation, created by the `try_files` coincidental-first-column
//! defect `equipment_gap::find_citation` fixed this cycle (SD-32 T9
//! residual, `decisions.md §20`; cycle receipt
//! `artifacts/gate-3-closure-invariant/
//! t9-onboarding-equipment-ue-gap-routing_cycle-1_cycle_receipt.md`'s named
//! "advanced_class_guide's 22 `equipment_modifier`" lead).
//!
//! ## The defect this exists for
//!
//! Before the reorder, `find_citation` let a base template row's own
//! DISPLAY name (bare first column) coincidentally win over the `.COPY=`
//! row that actually mints the record's short key — so a record generated
//! for the SHORT key shipped citing the LONG-key base row's line instead of
//! its own `.COPY=` line, two lines away in the same file. The record's
//! `data` content is unaffected (a `.COPY=` row inherits the base's fields
//! verbatim, confirmed live for every case this module repairs), but its
//! `source.line` — and therefore the census join `scripts/shape_ledger.py`
//! performs on `(book, source_basename, source_line)` — points at the wrong
//! row, leaving the record's OWN short-key census unit permanently
//! `no_record` while a coincidental line match silently "covers" it for an
//! unrelated long-key unit that (this module verifies) already has its own
//! independent citation elsewhere.
//!
//! ## The bar a record has to clear before its citation moves
//!
//! Never a config assumption — every condition below is checked against the
//! real corpus and the real sibling records on disk, and a record failing
//! any one is refused (named, not silently skipped):
//!
//! 1. The record's `source.kind` is `"lst_token"` and its `data.key`/
//!    `data.name` resolve, under the FIXED `find_citation`, to a DIFFERENT
//!    line in the SAME file it already cites (never a different file — a
//!    file change is a different defect shape this module does not touch).
//! 2. The newly-resolved line's own row is a genuine `.COPY=<key>` (or
//!    `.COPY=<name>`) variant — read fresh off the pinned oracle, not
//!    assumed — proving the new line mints the exact same identity the
//!    record already carries, so this is a citation correction and never a
//!    re-identification.
//! 3. The OLD line remains covered by at least one OTHER record in this
//!    book after the move (an independent flat/nested sibling citing the
//!    same `(path, line)`) — proving the move cannot orphan whatever
//!    census unit the old line already, separately, satisfies. Every book
//!    this module runs against writes that sibling via its own
//!    hand-authored generator (`cache_gen::acg`, `cache_gen::hand_authored_
//!    equipment`), independently of this generator's own gap residue.
//! 4. The NEW line is not already the citation of any other record in this
//!    book (no collision created).
//!
//! A record failing any check is left exactly as shipped and reported by
//! name; nothing is fabricated and no field but `source.line` is ever
//! written. `raw_tokens`/`raw_bonus_chains`, when present, are REMOVED (not
//! rewritten) so the existing `enrich_equipment_raw_tokens` binary — the
//! established mechanism for that field, not a fork of it — repopulates
//! them from the corrected line on its own next run.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::rules_core::cache_gen::equipment_gap::find_citation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    NotLstToken(String),
    NoIdentity(String),
    NoBetterCitation(String),
    DifferentFile(String),
    NewLineNotACopyVariant(String),
    OldLineWouldBeOrphaned(String),
    NewLineAlreadyClaimed(String),
}

impl Refusal {
    pub fn describe(&self) -> String {
        match self {
            Refusal::NotLstToken(r) => format!("{r}: source.kind is not lst_token"),
            Refusal::NoIdentity(r) => format!("{r}: record carries neither data.key nor data.name"),
            Refusal::NoBetterCitation(r) => format!("{r}: find_citation agrees with the shipped citation already"),
            Refusal::DifferentFile(r) => format!("{r}: find_citation now resolves to a DIFFERENT file, not just a different line -- out of this module's scope"),
            Refusal::NewLineNotACopyVariant(r) => format!("{r}: the newly-resolved line is not a `.COPY=<key>` variant of the shipped identity -- refusing to move without that proof"),
            Refusal::OldLineWouldBeOrphaned(r) => format!("{r}: no OTHER record cites the shipped (path, line) -- moving would orphan whatever census unit relies on it"),
            Refusal::NewLineAlreadyClaimed(r) => format!("{r}: another record already cites the resolved (path, line)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepairedRecord {
    pub file: PathBuf,
    pub old_line: u32,
    pub new_line: u32,
}

#[derive(Debug, Default)]
pub struct RepairReport {
    pub repaired: Vec<RepairedRecord>,
    pub refused: Vec<Refusal>,
    pub records_seen: usize,
}

fn record_paths(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json")
                && path.file_name().and_then(|f| f.to_str()) != Some("LICENSE.json")
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// `true` when `lst_path`'s 1-indexed `line`'s first tab-delimited column
/// ends with `.COPY=<key>` or `.COPY=<name>` -- read fresh off disk, never
/// cached, so this is a proof against the real pinned oracle bytes at
/// repair time, not an assumption carried over from `find_citation`'s own
/// internal logic.
fn line_is_copy_variant_of(lst_path: &Path, line: u32, key: &str, name: &str) -> bool {
    let Ok(content) = std::fs::read_to_string(lst_path) else { return false };
    let Some(row) = content.lines().nth((line.saturating_sub(1)) as usize) else { return false };
    let first = row.split('\t').next().unwrap_or("");
    first.ends_with(&format!(".COPY={key}")) || first.ends_with(&format!(".COPY={name}"))
}

/// Repairs every eligible record under `records_dir` (recursively) whose
/// pinned-book `.lst` files live under `corpus_root.join(book_rel_dir)`.
/// `write == false` decides and reports without touching disk.
pub fn repair_book(
    corpus_root: &Path,
    book_rel_dir: &str,
    records_dir: &Path,
    write: bool,
) -> std::io::Result<RepairReport> {
    let mut report = RepairReport::default();
    if !records_dir.is_dir() {
        return Ok(report);
    }
    let book_dir = corpus_root.join(book_rel_dir);
    let files = record_paths(records_dir);

    // (path, line) -> how many records currently cite it. Built once, up
    // front, over EVERY record in this book (not just lst_token ones would
    // still not double-count) -- the independent-coverage and collision
    // checks both need the real citation population, not just the subset
    // this module considers repairing.
    let mut citation_counts: BTreeMap<(String, u32), u32> = BTreeMap::new();
    let mut parsed: Vec<(PathBuf, Value)> = Vec::new();
    for file in &files {
        let Ok(text) = std::fs::read_to_string(file) else { continue };
        let Ok(json): Result<Value, _> = serde_json::from_str(&text) else { continue };
        if let Some(source) = json.get("source") {
            if source.get("kind").and_then(Value::as_str) == Some("lst_token") {
                if let (Some(p), Some(l)) =
                    (source.get("path").and_then(Value::as_str), source.get("line").and_then(Value::as_u64))
                {
                    *citation_counts.entry((p.to_string(), l as u32)).or_insert(0) += 1;
                }
            }
        }
        parsed.push((file.clone(), json));
    }

    for (file, json) in &parsed {
        report.records_seen += 1;
        let file_label = file.display().to_string();
        let Some(source) = json.get("source") else {
            report.refused.push(Refusal::NotLstToken(file_label));
            continue;
        };
        if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
            report.refused.push(Refusal::NotLstToken(file_label));
            continue;
        }
        let Some(data) = json.get("data") else {
            report.refused.push(Refusal::NoIdentity(file_label));
            continue;
        };
        let Some(key) = data.get("key").and_then(Value::as_str) else {
            report.refused.push(Refusal::NoIdentity(file_label));
            continue;
        };
        let name = data.get("name").and_then(Value::as_str).unwrap_or(key);
        let Some(old_path) = source.get("path").and_then(Value::as_str) else {
            report.refused.push(Refusal::NoIdentity(file_label));
            continue;
        };
        let Some(old_line) = source.get("line").and_then(Value::as_u64) else {
            report.refused.push(Refusal::NoIdentity(file_label));
            continue;
        };
        let old_line = old_line as u32;

        let Some((resolved_rel, resolved_line)) = find_citation(&book_dir, key, name) else {
            report.refused.push(Refusal::NoBetterCitation(file_label));
            continue;
        };
        let resolved_rel_str = resolved_rel.to_string_lossy().replace('\\', "/");
        let resolved_path_str = format!("{book_rel_dir}/{resolved_rel_str}");

        if resolved_path_str == old_path && resolved_line == old_line {
            report.refused.push(Refusal::NoBetterCitation(file_label));
            continue;
        }
        if resolved_path_str != old_path {
            report.refused.push(Refusal::DifferentFile(file_label));
            continue;
        }
        let lst_abs = book_dir.join(&resolved_rel);
        if !line_is_copy_variant_of(&lst_abs, resolved_line, key, name) {
            report.refused.push(Refusal::NewLineNotACopyVariant(file_label));
            continue;
        }
        let old_key = (old_path.to_string(), old_line);
        let other_coverage = citation_counts.get(&old_key).copied().unwrap_or(0);
        // This record itself is one of the counted citers of `old_key`;
        // moving it needs at least one OTHER record still covering it.
        if other_coverage < 2 {
            report.refused.push(Refusal::OldLineWouldBeOrphaned(file_label));
            continue;
        }
        let new_key = (resolved_path_str.clone(), resolved_line);
        if citation_counts.get(&new_key).copied().unwrap_or(0) > 0 {
            report.refused.push(Refusal::NewLineAlreadyClaimed(file_label));
            continue;
        }

        if write {
            let mut json_mut = json.clone();
            if let Some(source_mut) = json_mut.get_mut("source").and_then(Value::as_object_mut) {
                source_mut.insert("line".to_string(), Value::from(resolved_line));
            }
            if let Some(data_mut) = json_mut.get_mut("data").and_then(Value::as_object_mut) {
                // Let `enrich_equipment_raw_tokens` (the established
                // mechanism for this field) repopulate from the corrected
                // line on its own next run -- never hand-computed here.
                data_mut.remove("raw_tokens");
                data_mut.remove("raw_bonus_chains");
            }
            let out = serde_json::to_string_pretty(&json_mut)
                .expect("a Value round-tripped from valid JSON always re-serializes");
            std::fs::write(file, out)?;
        }

        report.repaired.push(RepairedRecord { file: file.clone(), old_line, new_line: resolved_line });
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, name: &str, content: &str) {
        std::fs::write(dir.join(name), content).unwrap();
    }

    /// The exact `advanced_class_guide:equipment_modifier:answering` shape:
    /// one record shipped citing the coincidental first-column base line,
    /// a SIBLING record independently covering that same base line (proving
    /// the move is safe), and the real `.COPY=` line the record should move
    /// to.
    #[test]
    fn repairs_a_stale_copy_citation_when_the_old_line_has_independent_coverage() {
        let root = std::env::temp_dir().join(format!("eccr_test_{}", std::process::id()));
        let corpus_root = root.join("corpus_root");
        let book_dir = corpus_root.join("book");
        std::fs::create_dir_all(&book_dir).unwrap();
        write(
            &book_dir,
            "book_equipmods.lst",
            "Answering\tKEY:Special Ability ~ Answering ~ Weapon\tSPROP:Real text\n\
             Special Ability ~ Answering ~ Weapon.COPY=Answering\tVISIBLE:NO\n",
        );

        let records_dir = root.join("out").join("equipment").join("equipmods");
        std::fs::create_dir_all(&records_dir).unwrap();
        // The stale record: data.key == data.name == "Answering", cited at
        // line 1 (the coincidental first-column match).
        write(
            &records_dir,
            "answering.json",
            r#"{"data":{"key":"Answering","name":"Answering","raw_tokens":[{"key":"X","value":"Y"}]},
                "source":{"kind":"lst_token","path":"book/book_equipmods.lst","line":1,"record_key":"Answering","sha256":"abc"}}"#,
        );
        // The independent sibling covering line 1 under the LONG key --
        // proves the old line survives the move.
        write(
            &records_dir,
            "special_ability_answering_weapon.json",
            r#"{"data":{"key":"Special Ability ~ Answering ~ Weapon","name":"Answering"},
                "source":{"kind":"lst_token","path":"book/book_equipmods.lst","line":1,"record_key":"Special Ability ~ Answering ~ Weapon","sha256":"abc"}}"#,
        );

        let report = repair_book(&corpus_root, "book", &records_dir, true).unwrap();
        assert_eq!(report.repaired.len(), 1, "{:?}", report.refused);
        assert_eq!(report.repaired[0].old_line, 1);
        assert_eq!(report.repaired[0].new_line, 2);

        let updated: Value =
            serde_json::from_str(&std::fs::read_to_string(records_dir.join("answering.json")).unwrap()).unwrap();
        assert_eq!(updated["source"]["line"], 2);
        assert!(updated["data"].get("raw_tokens").is_none(), "stale raw_tokens must be removed, not carried forward");

        std::fs::remove_dir_all(&root).ok();
    }

    /// The safety gate: no independent sibling covers the old line, so the
    /// move is refused rather than orphaning whatever unit currently relies
    /// on it.
    #[test]
    fn refuses_when_the_old_line_has_no_independent_coverage() {
        let root = std::env::temp_dir().join(format!("eccr_test_orphan_{}", std::process::id()));
        let corpus_root = root.join("corpus_root");
        let book_dir = corpus_root.join("book");
        std::fs::create_dir_all(&book_dir).unwrap();
        write(
            &book_dir,
            "book_equipmods.lst",
            "Answering\tKEY:Special Ability ~ Answering ~ Weapon\tSPROP:Real text\n\
             Special Ability ~ Answering ~ Weapon.COPY=Answering\tVISIBLE:NO\n",
        );

        let records_dir = root.join("out").join("equipment").join("equipmods");
        std::fs::create_dir_all(&records_dir).unwrap();
        write(
            &records_dir,
            "answering.json",
            r#"{"data":{"key":"Answering","name":"Answering"},
                "source":{"kind":"lst_token","path":"book/book_equipmods.lst","line":1,"record_key":"Answering","sha256":"abc"}}"#,
        );

        let report = repair_book(&corpus_root, "book", &records_dir, true).unwrap();
        assert_eq!(report.repaired.len(), 0);
        assert!(matches!(report.refused.as_slice(), [Refusal::OldLineWouldBeOrphaned(_)]));

        let untouched: Value =
            serde_json::from_str(&std::fs::read_to_string(records_dir.join("answering.json")).unwrap()).unwrap();
        assert_eq!(untouched["source"]["line"], 1, "must not move without independent old-line coverage");

        std::fs::remove_dir_all(&root).ok();
    }

    /// `--check`-equivalent (`write = false`): decides and reports without
    /// touching disk.
    #[test]
    fn dry_run_does_not_write() {
        let root = std::env::temp_dir().join(format!("eccr_test_dryrun_{}", std::process::id()));
        let corpus_root = root.join("corpus_root");
        let book_dir = corpus_root.join("book");
        std::fs::create_dir_all(&book_dir).unwrap();
        write(
            &book_dir,
            "book_equipmods.lst",
            "Answering\tKEY:Special Ability ~ Answering ~ Weapon\tSPROP:Real text\n\
             Special Ability ~ Answering ~ Weapon.COPY=Answering\tVISIBLE:NO\n",
        );
        let records_dir = root.join("out").join("equipment").join("equipmods");
        std::fs::create_dir_all(&records_dir).unwrap();
        write(
            &records_dir,
            "answering.json",
            r#"{"data":{"key":"Answering","name":"Answering"},
                "source":{"kind":"lst_token","path":"book/book_equipmods.lst","line":1,"record_key":"Answering","sha256":"abc"}}"#,
        );
        write(
            &records_dir,
            "special_ability_answering_weapon.json",
            r#"{"data":{"key":"Special Ability ~ Answering ~ Weapon","name":"Answering"},
                "source":{"kind":"lst_token","path":"book/book_equipmods.lst","line":1,"record_key":"Special Ability ~ Answering ~ Weapon","sha256":"abc"}}"#,
        );

        let report = repair_book(&corpus_root, "book", &records_dir, false).unwrap();
        assert_eq!(report.repaired.len(), 1);
        let untouched: Value =
            serde_json::from_str(&std::fs::read_to_string(records_dir.join("answering.json")).unwrap()).unwrap();
        assert_eq!(untouched["source"]["line"], 1, "dry run must not write");

        std::fs::remove_dir_all(&root).ok();
    }
}
