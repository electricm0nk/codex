//! GE05-E2-F2 — Golden-case governed fixture instance tests.
//!
//! These tests pin the smallest oracle-validation fixture evolution that upgrades
//! the PF1 Core Rulebook Human Fighter level 1 pilot case from provisional seed
//! evidence to a governed first-case instance. The fixture must preserve legacy
//! oracle evidence exactly, consume the accepted deterministic GE-06 input
//! contract, and keep Codex/parity posture explicit without claiming parity has
//! passed.

use codex::oracle_validation::golden_fixture::{
    load_golden_case_fixture, ClaimTier, CodexOutputState, DiagnosticClass, DiagnosticSeverity,
    DimensionStatus, OracleEvidenceKind, RawOutputRetention,
};

/// A complete, valid set of fixture lines for the governed first-case instance.
/// Tests omit or override individual lines (by key prefix) to exercise specific
/// behaviors without restating the whole fixture.
fn base_fixture_lines() -> Vec<&'static str> {
    vec![
        "case_id=pf1-crb-human-fighter-level1",
        "case_version=0",
        "scope=PF1 CRB Human Fighter level 1 pilot oracle-comparison case",
        "source_system=pathfinder-1e",
        "source_package=core_rulebook",
        "source_campaign=Core Rulebook",
        "source_game_mode=Pathfinder_RPG",
        "character_input_ref=fixture:rules_core/pf1_human_fighter_level1_ge06_deterministic_input",
        "legacy_route=headless Gradle run batch export",
        "legacy_evidence_kind=runtime_behavior_evidence",
        "legacy_raw_output_ref=/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml",
        "legacy_raw_output_retention=local_generated_only",
        "legacy_raw_output_sha256=3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1",
        "legacy_reduced_facts_ref=artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md",
        "codex_output_state=unresolved",
        "dimension=derived_values:candidate",
        "known_gap_ref=gap-class:codex-output-unavailable",
        "known_gap_ref=gap-class:non-comparable-output",
        "claim_target=oracle_checked",
        "current_claim_status=not_yet_grounded",
    ]
}

/// Builds fixture text from [`base_fixture_lines`], dropping any line whose key
/// is in `omit_keys` and appending any `extra` lines.
fn fixture_text(omit_keys: &[&str], extra: &[&str]) -> String {
    base_fixture_lines()
        .into_iter()
        .filter(|line| {
            let key = line.split_once('=').map(|(k, _)| k).unwrap_or(line);
            !omit_keys.contains(&key)
        })
        .chain(extra.iter().copied())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn loads_governed_pf1_human_fighter_fixture_with_deterministic_input_and_explicit_known_gaps() {
    let fixture_text =
        include_str!("fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt");

    let result = load_golden_case_fixture(fixture_text);

    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
    let fixture = result
        .fixture
        .expect("valid golden-case fixture should load");

    // Stable case identity.
    assert_eq!(fixture.case_id, "pf1-crb-human-fighter-level1");
    assert_eq!(fixture.case_version, 0);

    // Source / campaign identity is preserved exactly.
    assert_eq!(fixture.source_package.campaign, "Core Rulebook");
    assert_eq!(fixture.source_package.game_mode, "Pathfinder_RPG");

    // The governed instance now grounds inherited inputs from the accepted GE-06
    // deterministic contract instead of the minimal placeholder fixture.
    assert_eq!(
        fixture.character_input_ref,
        "fixture:rules_core/pf1_human_fighter_level1_ge06_deterministic_input"
    );

    // Old-system evidence is runtime behavior evidence, not static source truth.
    assert_eq!(
        fixture.legacy_oracle.evidence_kind,
        OracleEvidenceKind::RuntimeBehaviorEvidence
    );
    assert_ne!(
        fixture.legacy_oracle.evidence_kind,
        OracleEvidenceKind::StaticSourceTruth
    );

    // Raw-output retention posture and SHA-256 are preserved exactly.
    assert_eq!(
        fixture.legacy_oracle.raw_output_retention,
        RawOutputRetention::LocalGeneratedOnly
    );
    assert_eq!(
        fixture.legacy_oracle.raw_output_sha256,
        "3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1"
    );

    // Codex/new-system output is still unresolved; parity is not claimed.
    assert_eq!(fixture.codex_output.state, CodexOutputState::Unresolved);
    assert!(!fixture.parity_claimed());
    assert_ne!(fixture.current_claim_status, ClaimTier::OracleChecked);
    assert_eq!(fixture.claim_target, ClaimTier::OracleChecked);

    // Closed pilot-input selections are no longer carried as provisional truth.
    assert_eq!(fixture.provisional_assumption("human_ability_bonus"), None);
    assert_eq!(
        fixture.provisional_assumption("final_equipment_loadout"),
        None
    );
    assert_eq!(fixture.provisional_assumption("skill_allocation"), None);
    assert_eq!(
        fixture.provisional_assumption("additional_feat_slot_closure"),
        None
    );

    // Known-gap posture remains explicit while Codex/parity outputs stay below
    // OracleChecked.
    assert_eq!(
        fixture.known_gap_refs,
        vec![
            "gap-class:codex-output-unavailable".to_string(),
            "gap-class:non-comparable-output".to_string(),
        ]
    );

    // Comparison dimensions exist and none are in a passing state (the schema has
    // no "passed" variant), so the case cannot masquerade as oracle-checked.
    assert!(!fixture.dimensions.is_empty());
    assert!(fixture
        .dimensions
        .iter()
        .any(|dimension| dimension.status == DimensionStatus::NotYetGrounded));
}

#[test]
fn missing_legacy_oracle_hash_returns_claim_blocking_diagnostic() {
    let result = load_golden_case_fixture(&fixture_text(&["legacy_raw_output_sha256"], &[]));

    assert!(
        result.fixture.is_none(),
        "a fixture missing the legacy oracle SHA-256 must not load"
    );

    let hash_diagnostic = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.subject_ref == "legacy_raw_output_sha256")
        .expect("missing legacy oracle hash should produce a diagnostic");

    assert_eq!(hash_diagnostic.class, DiagnosticClass::MissingFixtureField);
    assert_eq!(hash_diagnostic.severity, DiagnosticSeverity::Error);
    assert!(
        hash_diagnostic.claim_blocking,
        "a missing required oracle hash must be claim-blocking"
    );
    assert!(hash_diagnostic.message.contains("missing"));
}

#[test]
fn fixture_can_represent_blocked_or_unresolved_codex_output_without_passing_parity() {
    // Codex output absent, a blocked dimension, and a still-unresolved claim
    // status — the fixture must represent this without claiming parity.
    let result = load_golden_case_fixture(&fixture_text(
        &["codex_output_state", "dimension"],
        &[
            "codex_output_state=absent",
            "dimension=exportable_summary:blocked",
        ],
    ));

    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
    let fixture = result.fixture.expect("blocked-output fixture should load");

    assert_eq!(fixture.codex_output.state, CodexOutputState::Absent);
    assert!(
        !fixture.parity_claimed(),
        "an unresolved/blocked case must never report parity as claimed"
    );
    assert_ne!(fixture.current_claim_status, ClaimTier::OracleChecked);
    assert!(fixture
        .dimensions
        .iter()
        .any(|dimension| dimension.status == DimensionStatus::Blocked));
}
