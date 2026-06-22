//! GE06-E3-F2 failure classifier and owner mapping gate.
//!
//! Proves the GE-06 pilot rules-core classifier can map the merged headless receipt
//! into one primary failure owner from the bounded GE-06 vocabulary:
//! - ModelFlaw
//! - ImporterFlaw
//! - EngineFlaw
//! - OracleGap
//! - UiGap
//!
//! The classifier must preserve the full vocabulary while classifying only what
//! the merged receipt can currently prove, and it must refuse an IntegrationIssue
//! sink or equivalent terminal vague bucket.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, build_pilot_headless_receipt,
};
use codex::rules_core::pilot_failure::{FailureClassification, PrimaryOwner};

fn assert_primary_owner(classification: FailureClassification, expected: PrimaryOwner) {
    let actual: PrimaryOwner = classification.primary_owner;
    assert_eq!(
        actual, expected,
        "classifier must expose one required primary owner directly"
    );
}

const DETERMINISTIC_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

#[test]
fn classifier_exposes_full_primary_owner_vocabulary() {
    // The classifier must preserve the GE-06 doctrine vocabulary explicitly.
    // This test ensures all five owner categories exist and are distinguishable.
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    // All five owners must exist and be constructible.
    let _model_flaw = PrimaryOwner::ModelFlaw;
    let _importer_flaw = PrimaryOwner::ImporterFlaw;
    let _engine_flaw = PrimaryOwner::EngineFlaw;
    let _oracle_gap = PrimaryOwner::OracleGap;
    let _ui_gap = PrimaryOwner::UiGap;

    // The classifier must produce a result for this supported receipt.
    let _classification = FailureClassification::classify(&receipt);
}

#[test]
fn classifier_requires_primary_owner() {
    // Every classification must expose exactly one required primary owner.
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    let classification = FailureClassification::classify(&receipt);
    let _: PrimaryOwner = classification.primary_owner;
}

#[test]
fn no_integration_issue_sink() {
    // The classifier must not have an IntegrationIssue or equivalent terminal bucket.
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    let classification = FailureClassification::classify(&receipt);
    let owner: PrimaryOwner = classification.primary_owner;

    match owner {
        PrimaryOwner::ModelFlaw
        | PrimaryOwner::ImporterFlaw
        | PrimaryOwner::EngineFlaw
        | PrimaryOwner::OracleGap
        | PrimaryOwner::UiGap => {}
    }
}

#[test]
fn computed_receipt_with_no_comparison_evidence_yields_oracle_gap() {
    // A supported deterministic receipt that computed successfully but lacks
    // comparison evidence must classify as OracleGap, not as success or an invented
    // issue. This honest classification reflects that the receipt has evidence
    // of computation but lacks oracle comparison.
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    // Verify the receipt is in the Computed state (has evidence, not blocked).
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "test fixture should produce a Computed receipt"
    );

    // Verify there are no claim-blocking diagnostics.
    assert!(
        !receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "Computed receipt must have no claim-blocking diagnostics"
    );

    // The classifier must map this computed-but-uncompared state to OracleGap.
    let classification = FailureClassification::classify(&receipt);

    assert_primary_owner(classification, PrimaryOwner::OracleGap);
}

#[test]
fn blocked_receipt_with_claim_blocking_diagnostics_yields_engine_flaw() {
    // A blocked receipt with claim-blocking rules diagnostics must classify as
    // EngineFlaw, indicating the rules-core computation surface (or its dependencies)
    // has a gap. This is grounded in the observable receipt state, not speculation.
    let mutated =
        DETERMINISTIC_FIXTURE.replace("class_level=class:fighter:1", "class_level=class:rogue:1");
    assert!(
        mutated.contains("class_level=class:rogue:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);
    let receipt = build_pilot_headless_receipt(&input);

    // Verify the receipt is in the Blocked state.
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Blocked,
        "mutated fixture should produce a Blocked receipt"
    );

    // Verify at least one claim-blocking diagnostic is present.
    let has_blocking = receipt
        .computation
        .diagnostics
        .iter()
        .any(|d| d.claim_blocking);
    assert!(
        has_blocking,
        "Blocked receipt must have at least one claim-blocking diagnostic: {:?}",
        receipt.computation.diagnostics
    );

    // The classifier must map claim-blocking diagnostics to EngineFlaw.
    let classification = FailureClassification::classify(&receipt);

    assert_primary_owner(classification, PrimaryOwner::EngineFlaw);
}
