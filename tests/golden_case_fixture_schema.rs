//! GE05-E2-F1 — Golden-case fixture schema tests.
//!
//! These tests pin the smallest oracle-validation fixture shape that can represent
//! the PF1 Core Rulebook Human Fighter level 1 pilot case while binding it to
//! old-system PCGen runtime evidence and an unresolved Codex output, without
//! claiming parity has passed.

use codex::oracle_validation::golden_fixture::{
    ClaimTier, CodexOutputState, DiagnosticClass, DiagnosticSeverity, DimensionStatus,
    OracleEvidenceKind, RawOutputRetention, load_golden_case_fixture,
};

/// A complete, valid set of fixture lines. Tests omit or override individual
/// lines (by key prefix) to exercise specific behaviors without restating the
/// whole fixture.
fn base_fixture_lines() -> Vec<&'static str> {
    vec![
        "case_id=pf1-crb-human-fighter-level1",
        "case_version=0",
        "scope=PF1 CRB Human Fighter level 1 pilot oracle-comparison case",
        "source_system=pathfinder-1e",
        "source_package=core_rulebook",
        "source_campaign=Core Rulebook",
        "source_game_mode=Pathfinder_RPG",
        "character_input_ref=fixture:rules_core/pf1_human_fighter_level1_minimal_character_input",
        "legacy_route=headless Gradle run batch export",
        "legacy_evidence_kind=runtime_behavior_evidence",
        "legacy_raw_output_ref=/tmp/codex-ge05-e1-f2/pf1-crb-human-fighter-level1-attempt3.xml",
        "legacy_raw_output_retention=local_generated_only",
        "legacy_raw_output_sha256=3c0e38e7837dbcd6c5003ba100eb35e3f0992366d086228c4706d6f165d281e1",
        "legacy_reduced_facts_ref=artifacts/ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md",
        "codex_output_state=unresolved",
        "dimension=derived_values:candidate",
        "claim_target=oracle_checked",
        "current_claim_status=not_yet_grounded",
        "provisional=human_ability_bonus:+2 Strength",
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
fn loads_pf1_human_fighter_fixture_with_oracle_hash_and_provisional_assumptions() {
    let fixture_text =
        include_str!("fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt");

    let result = load_golden_case_fixture(fixture_text);

    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
    let fixture = result.fixture.expect("valid golden-case fixture should load");

    // Stable case identity.
    assert_eq!(fixture.case_id, "pf1-crb-human-fighter-level1");
    assert_eq!(fixture.case_version, 0);

    // Source / campaign identity is preserved exactly.
    assert_eq!(fixture.source_package.campaign, "Core Rulebook");
    assert_eq!(fixture.source_package.game_mode, "Pathfinder_RPG");

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

    // Codex/new-system output is unresolved; parity is not claimed.
    assert_eq!(fixture.codex_output.state, CodexOutputState::Unresolved);
    assert!(!fixture.parity_claimed());
    assert_ne!(fixture.current_claim_status, ClaimTier::OracleChecked);
    assert_eq!(fixture.claim_target, ClaimTier::OracleChecked);

    // Provisional, non-canonical assumptions are preserved as such.
    assert_eq!(
        fixture.provisional_assumption("human_ability_bonus"),
        Some("+2 Strength")
    );
    assert_eq!(
        fixture.provisional_assumption("final_equipment_loadout"),
        Some("unresolved")
    );
    assert_eq!(
        fixture.provisional_assumption("skill_allocation"),
        Some("unresolved")
    );
    assert_eq!(
        fixture.provisional_assumption("additional_feat_slot_closure"),
        Some("unresolved")
    );

    // Comparison dimensions exist and none are in a passing state (the schema has
    // no "passed" variant), so the case cannot masquerade as oracle-checked.
    assert!(!fixture.dimensions.is_empty());
    assert!(
        fixture
            .dimensions
            .iter()
            .any(|dimension| dimension.status == DimensionStatus::NotYetGrounded)
    );
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
    assert!(
        fixture
            .dimensions
            .iter()
            .any(|dimension| dimension.status == DimensionStatus::Blocked)
    );
}
