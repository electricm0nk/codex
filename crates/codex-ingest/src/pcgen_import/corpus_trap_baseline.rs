//! Baseline reconciliation for `v06_corpus_trap_report`'s four corpus-invariant
//! tests over already-registered SD-33 inherited debt
//! (`docs/governance/corpus-trap-baseline.tsv`).
//!
//! **Why this exists.** `AT-34-E1-007`/`AT-34-E1-008` (`decisions.md §13`) rule
//! that four of `v06_corpus_trap_report`'s trap kinds — `mod-record`,
//! `key-differs-from-name`, `shared-name-distinct-records`, `disabled-line` —
//! are SD-33's already-verified, already-out-of-DoD inherited debt,
//! "registered, not absorbed," and are reported "at their unchanged counts."
//! `AT-34-E1-008`'s own bar is `wiring-class-mismatch = 0` — a fifth,
//! unrelated trap kind this module does not touch. The four tests never
//! implemented that ruling: each asserted `violations.is_empty()` against a
//! population §13 explicitly said is not this bundle's to drive to zero, so
//! the stage was red forever and, being red for a known and unchanging
//! reason, told a reader nothing new on any given run.
//!
//! This module is `src/rules_core/pi_table_sweep.rs`'s reconciliation shape,
//! carried over: a baseline row states the expected count, and a live count
//! is checked against it in **both** directions —
//!
//! * live count **above** the baseline is [`TrapBaselineVerdict::Added`] (a
//!   real regression: a new corpus trap of this kind exists that the
//!   baseline did not account for), and
//! * live count **below** the baseline is [`TrapBaselineVerdict::Stale`] (the
//!   debt shrank — good news, but a silently-out-of-date row, exactly the
//!   failure mode `pi_table_sweep::reconcile`'s own `stale` return guards
//!   against; the fix is to update the row deliberately, in the same cycle
//!   that closed the finding, not to let the file drift).
//!
//! Only an exact match is [`TrapBaselineVerdict::Matched`] — the test passes.
//! A trap id with no baseline row at all is [`TrapBaselineVerdict::Unbaselined`]
//! (a malformed or missing row is an error, never a silent zero-tolerance
//! default, matching `pi_table_sweep::parse_baseline`'s own refusal to treat
//! a bad row as a dropped suppression).

use std::path::Path;

/// One registered baseline row from `docs/governance/corpus-trap-baseline.tsv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrapBaselineEntry {
    /// The stable `Trap::id()` string (e.g. `"mod-record"`).
    pub trap_id: String,
    /// The exact `Finding` count this trap kind is registered at.
    pub count: usize,
    /// Free-text note — which test owns this row and why.
    pub note: String,
}

/// The result of checking one trap kind's live count against its baseline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapBaselineVerdict {
    /// Live count equals the registered baseline. The test passes.
    Matched,
    /// Live count exceeds the registered baseline — a real regression: a new
    /// finding of this trap kind that the baseline does not account for.
    Added { trap_id: String, baseline: usize, live: usize },
    /// Live count is below the registered baseline — the debt shrank, but
    /// the row was not updated to match. Also a failure: an un-updated
    /// baseline hides the improvement from every reader after this one.
    Stale { trap_id: String, baseline: usize, live: usize },
    /// No baseline row exists for this trap id at all.
    Unbaselined { trap_id: String, live: usize },
}

impl TrapBaselineVerdict {
    /// `true` only for [`TrapBaselineVerdict::Matched`] — the shape every
    /// caller that just wants a pass/fail boolean needs.
    pub fn is_matched(&self) -> bool {
        matches!(self, TrapBaselineVerdict::Matched)
    }

    /// A human-readable explanation of a non-matched verdict, for an
    /// `assert!` message. Panics if called on `Matched` — callers only need
    /// this on the failure path.
    pub fn explain(&self) -> String {
        match self {
            TrapBaselineVerdict::Matched => {
                unreachable!("explain() is only for a non-matched verdict")
            }
            TrapBaselineVerdict::Added { trap_id, baseline, live } => format!(
                "trap `{trap_id}`: live count {live} exceeds its registered baseline {baseline} \
                 in docs/governance/corpus-trap-baseline.tsv — a real regression, not a stale \
                 row. If this is genuinely new corpus debt, register it deliberately; if it is \
                 the `wiring-class-mismatch` kind, that is AT-34-E1-008's own zero-tolerance bar \
                 and does not belong in this baseline at all."
            ),
            TrapBaselineVerdict::Stale { trap_id, baseline, live } => format!(
                "trap `{trap_id}`: live count {live} is BELOW its registered baseline {baseline} \
                 in docs/governance/corpus-trap-baseline.tsv — the debt shrank without the row \
                 being updated. Update the row deliberately (in the cycle that closed the \
                 finding) rather than leaving a stale suppression that hides the improvement."
            ),
            TrapBaselineVerdict::Unbaselined { trap_id, live } => format!(
                "trap `{trap_id}`: {live} live findings, but no row for it exists in \
                 docs/governance/corpus-trap-baseline.tsv — add one deliberately."
            ),
        }
    }
}

/// Parse `docs/governance/corpus-trap-baseline.tsv`.
///
/// Format: `trap_id<TAB>count<TAB>note`. Blank lines and `#` comment lines
/// are ignored. A malformed row is an error, never a silently dropped
/// suppression — the same contract `pi_table_sweep::parse_baseline` holds.
pub fn parse_baseline(text: &str) -> Result<Vec<TrapBaselineEntry>, String> {
    let mut entries = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim_end_matches(['\r', '\n']);
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.splitn(3, '\t').collect();
        if cols.len() < 2 {
            return Err(format!(
                "corpus-trap baseline line {}: expected at least 2 tab-separated columns \
                 (trap_id, count), got {}",
                idx + 1,
                cols.len()
            ));
        }
        let trap_id = cols[0].trim().to_string();
        let count: usize = cols[1].trim().parse().map_err(|e| {
            format!("corpus-trap baseline line {}: bad count {:?}: {e}", idx + 1, cols[1])
        })?;
        entries.push(TrapBaselineEntry {
            trap_id,
            count,
            note: cols.get(2).map(|s| s.trim().to_string()).unwrap_or_default(),
        });
    }
    Ok(entries)
}

/// Read and parse the baseline file at `path` (repo-relative resolution is
/// the caller's job — tests use `CARGO_MANIFEST_DIR`).
pub fn load_baseline(path: &Path) -> Result<Vec<TrapBaselineEntry>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read corpus-trap baseline {}: {e}", path.display()))?;
    parse_baseline(&text)
}

/// Check one trap kind's live finding count against the baseline, in both
/// directions. See the module doc for what each verdict means.
pub fn reconcile_trap_count(
    trap_id: &str,
    live_count: usize,
    baseline: &[TrapBaselineEntry],
) -> TrapBaselineVerdict {
    match baseline.iter().find(|e| e.trap_id == trap_id) {
        None => TrapBaselineVerdict::Unbaselined { trap_id: trap_id.to_string(), live: live_count },
        Some(entry) if live_count == entry.count => TrapBaselineVerdict::Matched,
        Some(entry) if live_count > entry.count => TrapBaselineVerdict::Added {
            trap_id: trap_id.to_string(),
            baseline: entry.count,
            live: live_count,
        },
        Some(entry) => TrapBaselineVerdict::Stale {
            trap_id: trap_id.to_string(),
            baseline: entry.count,
            live: live_count,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_matching_count_is_matched() {
        let baseline = parse_baseline("mod-record\t2117\tnote\n").unwrap();
        assert_eq!(reconcile_trap_count("mod-record", 2117, &baseline), TrapBaselineVerdict::Matched);
    }

    /// The mutation proof: a live count ABOVE the baseline (a planted
    /// regression) must fire `Added`, never silently pass.
    #[test]
    fn a_live_count_above_baseline_is_added_not_silently_accepted() {
        let baseline = parse_baseline("mod-record\t2117\tnote\n").unwrap();
        let verdict = reconcile_trap_count("mod-record", 2118, &baseline);
        assert_eq!(
            verdict,
            TrapBaselineVerdict::Added { trap_id: "mod-record".into(), baseline: 2117, live: 2118 }
        );
        assert!(!verdict.is_matched());
        assert!(verdict.explain().contains("exceeds its registered baseline"));
    }

    /// The other mutation proof: a live count BELOW the baseline (debt paid
    /// down without the row being updated) must fire `Stale`, never
    /// silently pass as "even better than required."
    #[test]
    fn a_live_count_below_baseline_is_stale_not_silently_accepted() {
        let baseline = parse_baseline("mod-record\t2117\tnote\n").unwrap();
        let verdict = reconcile_trap_count("mod-record", 2116, &baseline);
        assert_eq!(
            verdict,
            TrapBaselineVerdict::Stale { trap_id: "mod-record".into(), baseline: 2117, live: 2116 }
        );
        assert!(!verdict.is_matched());
        assert!(verdict.explain().contains("BELOW its registered baseline"));
    }

    #[test]
    fn a_trap_with_no_baseline_row_is_unbaselined() {
        let baseline = parse_baseline("mod-record\t2117\tnote\n").unwrap();
        let verdict = reconcile_trap_count("disabled-line", 165, &baseline);
        assert_eq!(verdict, TrapBaselineVerdict::Unbaselined { trap_id: "disabled-line".into(), live: 165 });
        assert!(!verdict.is_matched());
    }

    #[test]
    fn zero_baselined_at_zero_is_matched() {
        let baseline = parse_baseline("disabled-line\t0\tclean\n").unwrap();
        assert_eq!(reconcile_trap_count("disabled-line", 0, &baseline), TrapBaselineVerdict::Matched);
    }

    #[test]
    fn comments_and_blank_lines_are_ignored() {
        let baseline = parse_baseline("# a comment\n\nmod-record\t2117\tnote\n\n# trailing\n").unwrap();
        assert_eq!(baseline.len(), 1);
    }

    #[test]
    fn a_bad_count_is_an_error_not_a_silent_default() {
        assert!(parse_baseline("mod-record\tnot-a-number\tnote\n").is_err());
    }

    #[test]
    fn a_row_with_no_count_column_is_an_error() {
        assert!(parse_baseline("mod-record\n").is_err());
    }

    #[test]
    fn a_note_less_row_still_parses() {
        let baseline = parse_baseline("mod-record\t2117\n").unwrap();
        assert_eq!(baseline[0].note, "");
    }
}
