//! Product-Identity blacklist sweep for the Rust-literal table pipeline
//! (`src/rules_core/rules_tables/**/*.rs`).
//!
//! **Why this exists.** [`crate::rules_core::pi_screening`] screens a record's
//! free-text field *at the moment a JSON corpus cache row is built*. It never
//! reached the other pipeline: `docs/governance/license-matrix.md` established
//! that **no** file under `rules_tables/` calls `pi_screening`,
//! `PI_BLACKLIST_TERMS`, or `classify_field`, and that a manual sweep of the
//! blacklist over those files already found real, unredacted Product-Identity
//! text in committed source. Kind-scoped ingestion lanes write into exactly
//! that pipeline, so the screen has to exist where the lane writes.
//!
//! **Two entry points, one term list** (the shared
//! [`crate::rules_core::pi_screening::PI_BLACKLIST_TERMS`] — this module does
//! not fork it, which is the failure mode `pi_screening`'s own header
//! documents):
//!
//! 1. [`screen_generated_table`] — a lane's extraction/table-generation step
//!    calls this on the table text it is about to write, *before* the write.
//!    A non-empty result is a hard stop for that record.
//! 2. [`sweep_dir`] + [`reconcile`] — the standing gate over the whole tree,
//!    run by the `pi_sweep_rules_tables` binary and by
//!    `tests/pi_table_sweep.rs`, so a leak cannot land through any path,
//!    including a hand edit.
//!
//! Hits that pre-date the gate and are owned by other bundles live in
//! `docs/governance/pi-sweep-baseline.tsv` with an explicit disposition.
//! [`reconcile`] reports both directions: a hit the baseline does not account
//! for, **and** a baseline row the tree no longer carries — the second keeps
//! the file from decaying into a blanket suppression.
//!
//! The term list is a bounded, documented heuristic
//! (`docs/governance/ogl-pi-blacklist.md`), not a legal review, and it is a
//! plain substring match: `Nex` matches inside `Next`, `Geb` inside `Gebr`.
//! Those are recorded as `false-positive` baseline rows rather than removed
//! from the list.

use crate::rules_core::pi_screening::PI_BLACKLIST_TERMS;
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

/// One blacklist-term occurrence in one line of one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiSweepHit {
    /// Path as swept — repo-relative when [`sweep_dir`] is rooted at the repo's
    /// `rules_tables` tree, otherwise whatever the caller passed.
    pub file: String,
    /// 1-indexed line number.
    pub line: usize,
    /// The blacklist term that matched.
    pub term: &'static str,
    /// The matching line, trimmed and truncated for a readable report.
    pub context: String,
}

/// One acknowledged pre-existing occurrence, from
/// `docs/governance/pi-sweep-baseline.tsv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiSweepBaselineEntry {
    pub file: String,
    pub term: String,
    /// How many occurrences of `term` this file is known to carry.
    pub count: usize,
    /// `real-leak` (a genuine PI string, owned by the bundle that wrote it) or
    /// `false-positive` (substring collision).
    pub disposition: String,
    pub note: String,
}

/// The result of checking a sweep against the baseline.
#[derive(Debug, Default)]
pub struct PiSweepVerdict {
    /// Hits with no baseline row, or beyond a baseline row's recorded count.
    pub unbaselined: Vec<PiSweepHit>,
    /// Baseline rows the tree no longer carries at the recorded count.
    pub stale: Vec<PiSweepBaselineEntry>,
}

const CONTEXT_MAX: usize = 160;

fn truncate_context(line: &str) -> String {
    let trimmed = line.trim();
    match trimmed.char_indices().nth(CONTEXT_MAX) {
        None => trimmed.to_string(),
        Some((idx, _)) => format!("{}…", &trimmed[..idx]),
    }
}

/// Every blacklist-term occurrence in `text`, attributed to `file`.
///
/// One hit per (line, term) pair: a line naming the same term twice is one
/// finding for the reader, and the baseline counts findings, not substrings.
pub fn sweep_text(file: &str, text: &str) -> Vec<PiSweepHit> {
    let mut hits = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        for term in PI_BLACKLIST_TERMS {
            if line.contains(term) {
                hits.push(PiSweepHit {
                    file: file.to_string(),
                    line: idx + 1,
                    term,
                    context: truncate_context(line),
                });
            }
        }
    }
    hits
}

/// The lane-facing screen: call this on generated table text **before** writing
/// it into `rules_tables/`. A non-empty return is a hard stop for the record
/// per `docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md §37.3` —
/// the caller records the hits in its cycle receipt and does not write.
pub fn screen_generated_table(file: &str, generated: &str) -> Vec<PiSweepHit> {
    sweep_text(file, generated)
}

/// Sweep every `.rs` file under `root`, recursively. Paths in the returned hits
/// are relative to `root`'s parent chain as walked — i.e. `root`'s own prefix
/// is preserved so a report is copy-pasteable.
pub fn sweep_dir(root: &Path) -> io::Result<Vec<PiSweepHit>> {
    let mut files = Vec::new();
    collect_rs_files(root, &mut files)?;
    files.sort();
    let mut hits = Vec::new();
    for path in files {
        let text = std::fs::read_to_string(&path)?;
        let shown = display_path(&path);
        hits.extend(sweep_text(&shown, &text));
    }
    Ok(hits)
}

/// Repo-relative from `src/` onward when the path contains it, so a sweep run
/// from any absolute root reports the same stable key the baseline file uses.
fn display_path(path: &Path) -> String {
    let full = path.to_string_lossy().replace('\\', "/");
    match full.find("src/rules_core/rules_tables/") {
        Some(idx) => full[idx..].to_string(),
        None => full,
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

/// Parse `docs/governance/pi-sweep-baseline.tsv`.
///
/// Format: `file<TAB>term<TAB>count<TAB>disposition<TAB>note`. Blank lines and
/// `#` comment lines are ignored. A malformed row is an error, never a silently
/// dropped suppression.
pub fn parse_baseline(text: &str) -> Result<Vec<PiSweepBaselineEntry>, String> {
    let mut entries = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim_end_matches(['\r', '\n']);
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 4 {
            return Err(format!(
                "pi-sweep baseline line {}: expected 4 or 5 tab-separated columns, got {}",
                idx + 1,
                cols.len()
            ));
        }
        let count: usize = cols[2]
            .trim()
            .parse()
            .map_err(|e| format!("pi-sweep baseline line {}: bad count {:?}: {e}", idx + 1, cols[2]))?;
        let disposition = cols[3].trim().to_string();
        if disposition != "real-leak" && disposition != "false-positive" {
            return Err(format!(
                "pi-sweep baseline line {}: disposition must be `real-leak` or `false-positive`, got {disposition:?}",
                idx + 1
            ));
        }
        entries.push(PiSweepBaselineEntry {
            file: cols[0].trim().to_string(),
            term: cols[1].trim().to_string(),
            count,
            disposition,
            note: cols.get(4).map(|s| s.trim().to_string()).unwrap_or_default(),
        });
    }
    Ok(entries)
}

/// Check a sweep's hits against the baseline, in both directions.
pub fn reconcile(hits: &[PiSweepHit], baseline: &[PiSweepBaselineEntry]) -> PiSweepVerdict {
    let mut allowed: BTreeMap<(String, String), usize> = BTreeMap::new();
    for entry in baseline {
        *allowed.entry((entry.file.clone(), entry.term.clone())).or_insert(0) += entry.count;
    }

    let mut seen: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut unbaselined = Vec::new();
    for hit in hits {
        let key = (hit.file.clone(), hit.term.to_string());
        let used = seen.entry(key.clone()).or_insert(0);
        *used += 1;
        if *used > allowed.get(&key).copied().unwrap_or(0) {
            unbaselined.push(hit.clone());
        }
    }

    let stale = baseline
        .iter()
        .filter(|entry| {
            let key = (entry.file.clone(), entry.term.clone());
            seen.get(&key).copied().unwrap_or(0) < allowed.get(&key).copied().unwrap_or(0)
        })
        .cloned()
        .collect();

    PiSweepVerdict { unbaselined, stale }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_naming_two_terms_yields_two_hits() {
        let hits = sweep_text("x.rs", "Absalom and Cheliax\n");
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn screen_generated_table_is_the_lane_facing_alias() {
        assert_eq!(
            screen_generated_table("gen.rs", "praise Pharasma\n"),
            sweep_text("gen.rs", "praise Pharasma\n")
        );
    }

    #[test]
    fn a_baseline_row_short_of_the_real_count_leaves_the_extra_hit_unbaselined() {
        let hits = sweep_text("x.rs", "Desna\nDesna\n");
        let baseline = parse_baseline("x.rs\tDesna\t1\treal-leak\tone known\n").unwrap();
        let verdict = reconcile(&hits, &baseline);
        assert_eq!(verdict.unbaselined.len(), 1);
        assert!(verdict.stale.is_empty());
    }

    #[test]
    fn a_bad_disposition_is_an_error_not_a_silent_suppression() {
        assert!(parse_baseline("x.rs\tDesna\t1\twhatever\tnote\n").is_err());
    }
}
