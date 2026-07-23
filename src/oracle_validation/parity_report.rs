//! Oracle-harness parity-report writer (SD-26 Epic 2, Criterion 2.3).
//!
//! Renders a real `parity_report_<case-id>.md` document from a
//! [`ComparisonResult`] (Criterion 2.1's `comparator.rs`), following the
//! Summary / Per-Dimension Comparison / Normalization Rules Used / Discovered
//! Deltas shape `technical-design.md §2.3` specifies, and writes it to disk
//! at the real per-case output path `scope-draft.md §1.2` names:
//! `artifacts/oracle_validation/parity_report_<case-id>.md`.
//!
//! This module does not itself run the comparator or the normalization rule
//! engine — it is a pure renderer over their already-produced output
//! (`ComparisonResult`, `&[NormalizationRule]`), consistent with
//! `comparator.rs`'s and `normalization.rs`'s module docs describing 2.3 as a
//! downstream consumer, not a producer, of their public surface.
//!
//! Concurrency: serial (per `epic-breakdown.md` Criterion 2.3).

use std::io;
use std::path::{Path, PathBuf};

use crate::oracle_validation::comparator::{ComparisonResult, DimensionMismatch, MismatchReason};
use crate::oracle_validation::normalization::NormalizationRule;

const REPORT_DIR_REL: &str = "artifacts/oracle_validation";

/// Resolves this codex checkout's root, mirroring
/// `pcgen_runner.rs::codex_repo_root`'s order-of-truth: `CODEX_REPO_ROOT`
/// (an operator/launcher override) wins when set; otherwise the compile-time
/// `CARGO_MANIFEST_DIR` (this crate's own root, which is the repo root).
fn codex_repo_root() -> PathBuf {
    std::env::var("CODEX_REPO_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

/// Resolves the default parity-report output directory,
/// `artifacts/oracle_validation/`, per `scope-draft.md §1.2` Criterion 2.3.
pub fn default_parity_report_dir() -> PathBuf {
    codex_repo_root().join(REPORT_DIR_REL)
}

/// Formats a dimension's `(value_string, value_i16)` pair as the single
/// display value the report's Markdown table cells use. Exactly one of the
/// two is expected to be set per `NormalizedDimensionValue`'s /
/// `SelectedDimension`'s own doc comments; an entirely-absent dimension
/// (one-sided mismatch) renders as an em dash.
fn format_value(value_string: &Option<String>, value_i16: Option<i16>) -> String {
    if let Some(i) = value_i16 {
        i.to_string()
    } else if let Some(s) = value_string {
        s.clone()
    } else {
        "—".to_string()
    }
}

/// Renders the human-legible reason a dimension mismatched, for the
/// Discovered Deltas section. Named plainly (not just the raw enum variant
/// name) since `MissingFromCodex`/`MissingFromPcgen` describe a one-sided
/// dimension set, which is a real, expected case per `comparator.rs`'s
/// `MismatchReason` doc comment.
fn mismatch_reason_text(reason: MismatchReason) -> &'static str {
    match reason {
        MismatchReason::ValueMismatch => "value mismatch",
        MismatchReason::MissingFromCodex => "missing from Codex",
        MismatchReason::MissingFromPcgen => "missing from PCGen",
    }
}

fn render_dimension_table_row(
    dimension_id: &str,
    pcgen_value: String,
    codex_value: String,
    matched: bool,
) -> String {
    format!(
        "| {dimension_id} | {pcgen_value} | {codex_value} | {} | — |\n",
        if matched { "yes" } else { "no" }
    )
}

fn render_discovered_delta(mismatch: &DimensionMismatch) -> String {
    format!(
        "- `{}` — PCGen: {}, Codex: {} ({})\n",
        mismatch.dimension_id,
        format_value(&mismatch.pcgen_value_string, mismatch.pcgen_value_i16),
        format_value(&mismatch.codex_value_string, mismatch.codex_value_i16),
        mismatch_reason_text(mismatch.reason),
    )
}

/// Renders `parity_report_<case-id>.md`'s full Markdown body from a real
/// [`ComparisonResult`] (Criterion 2.1's `compare` output) and the
/// normalization rule set the comparator's normalized inputs were reduced
/// through (Criterion 2.2's `normalization.rs`), per
/// `technical-design.md §2.3`'s Summary / Per-Dimension Comparison /
/// Normalization Rules Used / Discovered Deltas shape.
pub fn render_parity_report(
    case_id: &str,
    comparison: &ComparisonResult,
    normalization_rules_used: &[NormalizationRule],
) -> String {
    let match_count = comparison.matches.len();
    let mismatch_count = comparison.mismatches.len();
    let result = if comparison.all_matched() { "PASS" } else { "FAIL" };

    let mut out = String::new();

    out.push_str(&format!("# Oracle parity report: {case_id}\n\n"));

    out.push_str("## Summary\n\n");
    out.push_str(&format!("- Matches: {match_count}\n"));
    out.push_str(&format!("- Mismatches: {mismatch_count}\n"));
    out.push_str(&format!("- Result: {result}\n\n"));

    out.push_str("## Per-Dimension Comparison\n\n");
    out.push_str("| Dimension | PCGen | Codex | Match | Notes |\n");
    out.push_str("|---|---|---|---|---|\n");
    for m in &comparison.matches {
        out.push_str(&render_dimension_table_row(
            &m.dimension_id,
            format_value(&m.pcgen_value_string, m.pcgen_value_i16),
            format_value(&m.codex_value_string, m.codex_value_i16),
            true,
        ));
    }
    for m in &comparison.mismatches {
        out.push_str(&render_dimension_table_row(
            &m.dimension_id,
            format_value(&m.pcgen_value_string, m.pcgen_value_i16),
            format_value(&m.codex_value_string, m.codex_value_i16),
            false,
        ));
    }
    out.push('\n');

    out.push_str("## Normalization Rules Used\n\n");
    if normalization_rules_used.is_empty() {
        out.push_str("(none)\n");
    } else {
        for rule in normalization_rules_used {
            out.push_str(&format!("- {} (per `normalization.rs`)\n", rule.id));
        }
    }
    out.push('\n');

    out.push_str("## Discovered Deltas\n\n");
    if comparison.mismatches.is_empty() {
        out.push_str("(none)\n");
    } else {
        for mismatch in &comparison.mismatches {
            out.push_str(&render_discovered_delta(mismatch));
        }
    }

    out
}

/// Writes `render_parity_report`'s output to
/// `<output_dir>/parity_report_<case-id>.md`, creating `output_dir` if it
/// does not yet exist. Returns the real path written.
pub fn write_parity_report(
    output_dir: &Path,
    case_id: &str,
    comparison: &ComparisonResult,
    normalization_rules_used: &[NormalizationRule],
) -> io::Result<PathBuf> {
    std::fs::create_dir_all(output_dir)?;
    let report = render_parity_report(case_id, comparison, normalization_rules_used);
    let path = output_dir.join(format!("parity_report_{case_id}.md"));
    std::fs::write(&path, report)?;
    Ok(path)
}
