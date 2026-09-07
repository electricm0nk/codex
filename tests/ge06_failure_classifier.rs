//! GE06-E3-F2 failure classifier and owner-mapping proof test.
//!
//! Proves the GE-06 pilot compute surface can emit one bounded classifier that maps
//! the merged GE06-E2-F3 headless receipt into one primary failure owner: model flaw,
//! importer flaw, engine flaw, oracle gap, or UI gap. The classifier preserves the
//! full required vocabulary while only claiming the distinctions the current receipt
//! surface can actually support.

use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::pilot_failure::{FailureClassifier, PrimaryOwner};
mod common;
use common::load;

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

#[test]
fn computed_receipt_with_no_comparison_evidence_classifies_as_oracle_gap() {
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    // The deterministic fixture produces a computed receipt (no claim-blocking
    // diagnostics). The merged receipt surface carries computed outputs but no
    // oracle-comparison evidence (parity is not yet claimed). This should classify
    // to OracleGap.
    assert_eq!(
        receipt.status,
        codex::rules_core::pilot_compute::HeadlessReceiptStatus::Computed,
        "test setup: deterministic fixture must yield computed receipt"
    );

    let classifier = FailureClassifier::new(&receipt);

    assert_eq!(
        classifier.primary_owner(),
        PrimaryOwner::OracleGap,
        "computed receipt with no comparison evidence must classify to OracleGap"
    );
}

#[test]
fn blocked_receipt_with_claim_blocking_diagnostics_classifies_as_engine_flaw() {
    // Mutate the supported prerequisite in memory: replace Fighter level-1 with
    // Cleric level-1. This makes the receipt blocked with claim-blocking diagnostics
    // rather than computed. The classifier should distinguish this as EngineFlaw.
    // (Was Rogue level-1 until the v0.6 alpha swarm's multiclass BAB/save-stacking
    // generalization gave Rogue its own real class_chassis.* computation, so Rogue
    // is no longer an unsupported negative control -- Cleric still is.)
    let mutated =
        DETERMINISTIC_FIXTURE.replace("class_level=class:fighter:1", "class_level=class:cleric:1");
    assert!(
        mutated.contains("class_level=class:cleric:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);

    let receipt = build_pilot_headless_receipt(&input);

    // The receipt is now blocked: the class chassis is unsupported, which blocks
    // downstream claims with a claim-blocking diagnostic. The first broken contract
    // is the engine's inability to compute the Fighter level-1 chassis.
    assert_eq!(
        receipt.status,
        codex::rules_core::pilot_compute::HeadlessReceiptStatus::Blocked,
        "test setup: mutated fixture must yield blocked receipt"
    );
    assert!(
        receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "test setup: blocked receipt must have claim-blocking diagnostics"
    );

    let classifier = FailureClassifier::new(&receipt);

    assert_eq!(
        classifier.primary_owner(),
        PrimaryOwner::EngineFlaw,
        "blocked receipt with claim-blocking rules diagnostics must classify to EngineFlaw"
    );
}

#[test]
fn bounded_human_race_note_stays_non_blocking_and_keeps_oracle_gap() {
    let input = load(DETERMINISTIC_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);

    // The explicit Human race seam must surface a bounded race-semantics note, but as a
    // non-claim-blocking diagnostic so it cannot flip a computed receipt into a blocker
    // and mis-own the failure.
    let race_note = receipt
        .computation
        .diagnostics
        .iter()
        .find(|d| d.id == "race.human.bounded_semantics")
        .expect("computed receipt must carry the bounded Human race-semantics note");
    assert!(
        !race_note.claim_blocking,
        "bounded Human race-semantics note must not be claim-blocking: {race_note:?}"
    );

    let classifier = FailureClassifier::new(&receipt);
    assert_eq!(
        classifier.primary_owner(),
        PrimaryOwner::OracleGap,
        "the bounded Human race note must not change the computed-receipt owner mapping"
    );
}
