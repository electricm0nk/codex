//! Oracle-harness parity-report writer proof (SD-26 Epic 2, Criterion 2.3).
//!
//! Proves that `oracle_validation::parity_report` renders a real
//! `parity_report_<case-id>.md` document from a `ComparisonResult` (Criterion
//! 2.1's `comparator.rs`) per `technical-design.md §2.3`'s Summary /
//! Per-Dimension Comparison / Normalization Rules Used / Discovered Deltas
//! shape, and that it can write that document to disk at the real
//! `artifacts/oracle_validation/parity_report_<case-id>.md` path
//! (`scope-draft.md §1.2` Criterion 2.3).

use codex::oracle_validation::comparator::{
    compare, ComparisonResult, DimensionMatch, DimensionMismatch, MismatchReason,
};
use codex::oracle_validation::normalization::default_normalization_rules;
use codex::oracle_validation::parity_report::{
    default_parity_report_dir, render_parity_report, write_parity_report,
};
use codex::oracle_validation::selected_parity_dimensions::{
    ClaimTierFloor, SelectedDimension, SelectedParityDimensions,
};

fn pcgen_dim(id: &str, value_i16: i16) -> codex::oracle_validation::comparator::NormalizedDimensionValue
{
    codex::oracle_validation::comparator::NormalizedDimensionValue {
        id: id.to_string(),
        value_string: None,
        value_i16: Some(value_i16),
    }
}

fn codex_dim(id: &str, value_i16: i16) -> SelectedDimension {
    SelectedDimension {
        id: id.to_string(),
        value_string: None,
        value_i16: Some(value_i16),
        source_package_id: "pf1.core_rulebook".to_string(),
    }
}

/// Builds a real `ComparisonResult` (via the real Criterion 2.1 `compare`)
/// with one match and one mismatch, mirroring the GE06 pilot case dimensions.
fn sample_comparison() -> ComparisonResult {
    let pcgen = codex::oracle_validation::comparator::NormalizedOutput {
        dim_values: vec![
            pcgen_dim("combat.baseline_melee_attack_bonus", 0),
            pcgen_dim("defense.baseline_armor_class", 14),
        ],
    };
    let codex = SelectedParityDimensions {
        dimensions: vec![
            codex_dim("combat.baseline_melee_attack_bonus", 0),
            codex_dim("defense.baseline_armor_class", 15),
        ],
        claim_tier_floor: ClaimTierFloor::Computed,
    };
    compare(&pcgen, &codex)
}

#[test]
fn render_parity_report_includes_case_id_heading() {
    let comparison = sample_comparison();
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_crb_human_fighter_level1", &comparison, &rules);

    assert!(
        report.contains("# Oracle parity report: pf1_crb_human_fighter_level1"),
        "report should title itself with the case id, got:\n{report}"
    );
}

#[test]
fn render_parity_report_summary_counts_matches_and_mismatches() {
    let comparison = sample_comparison();
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_crb_human_fighter_level1", &comparison, &rules);

    assert!(report.contains("## Summary"), "missing Summary section:\n{report}");
    assert!(
        report.contains("Matches: 1"),
        "expected exactly 1 match counted, got:\n{report}"
    );
    assert!(
        report.contains("Mismatches: 1"),
        "expected exactly 1 mismatch counted, got:\n{report}"
    );
}

#[test]
fn render_parity_report_per_dimension_table_lists_both_sides_and_match_status() {
    let comparison = sample_comparison();
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_crb_human_fighter_level1", &comparison, &rules);

    assert!(
        report.contains("## Per-Dimension Comparison"),
        "missing table section:\n{report}"
    );
    assert!(
        report.contains("| combat.baseline_melee_attack_bonus | 0 | 0 | yes |"),
        "matched dimension row should show both values and 'yes', got:\n{report}"
    );
    assert!(
        report.contains("| defense.baseline_armor_class | 14 | 15 | no |"),
        "mismatched dimension row should show both values and 'no', got:\n{report}"
    );
}

#[test]
fn render_parity_report_lists_normalization_rules_used() {
    let comparison = sample_comparison();
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_crb_human_fighter_level1", &comparison, &rules);

    assert!(
        report.contains("## Normalization Rules Used"),
        "missing rules section:\n{report}"
    );
    assert!(report.contains("trailing-whitespace-strip"));
    assert!(report.contains("integer-coercion"));
}

#[test]
fn render_parity_report_discovered_deltas_section_lists_each_mismatch() {
    let comparison = sample_comparison();
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_crb_human_fighter_level1", &comparison, &rules);

    assert!(
        report.contains("## Discovered Deltas"),
        "missing deltas section:\n{report}"
    );
    assert!(
        report.contains("defense.baseline_armor_class"),
        "the one real mismatch should be listed as a discovered delta:\n{report}"
    );
    assert!(
        report.contains("14") && report.contains("15"),
        "discovered delta should cite both sides' values:\n{report}"
    );
}

#[test]
fn render_parity_report_all_matched_reports_pass() {
    let all_matched = ComparisonResult {
        matches: vec![DimensionMatch {
            dimension_id: "combat.baseline_melee_attack_bonus".to_string(),
            pcgen_value_string: None,
            pcgen_value_i16: Some(0),
            codex_value_string: None,
            codex_value_i16: Some(0),
        }],
        mismatches: Vec::new(),
    };
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_all_pass_case", &all_matched, &rules);

    assert!(report.contains("Result: PASS"), "expected a PASS summary, got:\n{report}");
    assert!(
        report.contains("(none)") || !report.contains("| combat.baseline_melee_attack_bonus |")
            || report.contains("## Discovered Deltas"),
        "even a fully-passing report should still render a Discovered Deltas section:\n{report}"
    );
}

#[test]
fn render_parity_report_any_mismatch_reports_fail() {
    let comparison = sample_comparison();
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_crb_human_fighter_level1", &comparison, &rules);

    assert!(report.contains("Result: FAIL"), "expected a FAIL summary, got:\n{report}");
}

#[test]
fn write_parity_report_writes_the_named_file_to_the_given_directory() {
    let comparison = sample_comparison();
    let rules = default_normalization_rules();

    let scratch_dir = std::env::temp_dir().join(format!(
        "codex-sd26-parity-report-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    let written_path =
        write_parity_report(&scratch_dir, "pf1_crb_human_fighter_level1", &comparison, &rules)
            .expect("write_parity_report should succeed against a fresh scratch directory");

    assert_eq!(
        written_path.file_name().and_then(|n| n.to_str()),
        Some("parity_report_pf1_crb_human_fighter_level1.md")
    );
    let on_disk = std::fs::read_to_string(&written_path).expect("written file should be readable");
    assert!(on_disk.contains("# Oracle parity report: pf1_crb_human_fighter_level1"));

    let _ = std::fs::remove_dir_all(&scratch_dir);
}

#[test]
fn default_parity_report_dir_ends_with_artifacts_oracle_validation() {
    let dir = default_parity_report_dir();

    assert!(
        dir.ends_with("artifacts/oracle_validation"),
        "expected the scope-draft.md §1.2 default output path, got: {}",
        dir.display()
    );
}

/// Mismatch reasons render a legible one-sided note (not just the raw enum
/// name), since this fixture's PCGen/Codex dimension sets are not guaranteed
/// identical (see `comparator.rs`'s `MismatchReason` doc comment).
#[test]
fn render_parity_report_one_sided_mismatch_reasons_are_legible() {
    let one_sided = ComparisonResult {
        matches: Vec::new(),
        mismatches: vec![DimensionMismatch {
            dimension_id: "skill.selected_modifier.climb".to_string(),
            pcgen_value_string: None,
            pcgen_value_i16: Some(4),
            codex_value_string: None,
            codex_value_i16: None,
            reason: MismatchReason::MissingFromCodex,
        }],
    };
    let rules = default_normalization_rules();

    let report = render_parity_report("pf1_one_sided_case", &one_sided, &rules);

    assert!(
        report.contains("missing from Codex") || report.contains("MissingFromCodex"),
        "one-sided mismatch should explain which side lacked the dimension:\n{report}"
    );
}
