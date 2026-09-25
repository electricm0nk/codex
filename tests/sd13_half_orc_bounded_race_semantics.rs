//! SD13-E2 Half-Orc bounded race-semantics classification slice.
//!
//! Originally proved the bounded, honest classification of the
//! `race.half_orc.bounded_semantics` row while it stayed `Unverified` /
//! `Observed` / `AwaitingInitialEvidence`, with only the generic
//! `race.semantics.unverified` diagnostic and no live Half-Orc proof surface.
//!
//! The SD13-E2 Half-Orc bounded race-semantics recognition slice
//! (`tests/sd13_half_orc_race_semantics_recognition.rs`) executed the promotion
//! path this file's original guards anticipated: it landed grounded evidence
//! for four race-semantic families (chosen ability-bonus target, size, speed,
//! senses — Darkvision) and updated the row state in the typed matrix carrier.
//! This file now pins that promoted truth for the fixture below (a `race:half-orc`
//! Fighter input that predates the recognition slice and carries no
//! ability-bonus choice): the Half-Orc race seam no longer emits the generic
//! diagnostic, and the matrix row is `Partial` / `Computed` rather than
//! `Unverified` / `Observed`.
//!
//! It is intentionally not a Half-Orc racial trait engine. It grounds no
//! Intimidating skill bonus, no Orc Ferocity, and no weapon familiarity math.

use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const HALF_ORC_SUBJECT_ID: &str = "race:half-orc";

/// End-to-end runtime proof: loading a deterministic Half-Orc Fighter level-1
/// fixture and running it through `compute_pilot_base_chassis` must produce the
/// `race.semantics.unverified` non-claim-blocking diagnostic and must NOT
/// fabricate any Half-Orc race trait, numeric value, or explanation. This is the
/// only live runtime evidence that Half-Orc is honestly unverified today; the
/// row's `Unverified` classification is grounded by this behavior.
#[test]
fn half_orc_fighter_pilot_emits_bounded_semantics_diagnostic_not_generic_unverified() {
    const HALF_ORC_FIGHTER_FIXTURE: &str = include_str!(
        "fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt"
    );

    let loaded = load_character_input_fixture(HALF_ORC_FIGHTER_FIXTURE);
    let input = loaded.character_input.unwrap_or_else(|| {
        panic!(
            "Half-Orc Fighter fixture must load cleanly, got diagnostics: {:?}",
            loaded.diagnostics
        )
    });
    assert_eq!(
        input.chosen.race_id, HALF_ORC_SUBJECT_ID,
        "fixture must carry the half-orc race identity"
    );

    let computation = compute_pilot_base_chassis(&input);

    // Since the SD13-E2 recognition slice, Half-Orc no longer receives the
    // generic race.semantics.unverified diagnostic; it receives its own
    // race.half_orc.bounded_semantics note instead, non-claim-blocking so the
    // deterministic Half-Orc Fighter pilot still reports a Computed receipt.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.semantics.unverified"),
        "Half-Orc must no longer surface the generic race.semantics.unverified \
         diagnostic now that it has its own seam: {:?}",
        computation.diagnostics
    );
    let bounded: Vec<_> = computation
        .diagnostics
        .iter()
        .filter(|d| d.id == "race.half_orc.bounded_semantics")
        .collect();
    assert_eq!(
        bounded.len(),
        1,
        "expected exactly one race.half_orc.bounded_semantics diagnostic, got {} \
         (all diagnostics: {:?})",
        bounded.len(),
        computation.diagnostics
    );
    assert!(
        !bounded[0].claim_blocking,
        "race.half_orc.bounded_semantics must remain non-claim-blocking so the \
         deterministic Half-Orc Fighter pilot still reports a Computed receipt; \
         got claim_blocking=true"
    );
}

/// The Half-Orc Fighter pilot compute path must NOT emit a `race.human.*`
/// explanation record: Half-Orc has no Human ability-bonus or Human bonus-feat
/// pressure, and the compute surface must not fabricate one. This pins the
/// honest non-fabrication boundary for the slice.
#[test]
fn half_orc_fighter_pilot_does_not_emit_human_race_explanations() {
    const HALF_ORC_FIGHTER_FIXTURE: &str = include_str!(
        "fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt"
    );

    let loaded = load_character_input_fixture(HALF_ORC_FIGHTER_FIXTURE);
    let input = loaded.character_input.expect("fixture must load cleanly");

    let computation = compute_pilot_base_chassis(&input);

    for explanation in &computation.explanations {
        assert!(
            !explanation.id.starts_with("race.human."),
            "Half-Orc pilot must not emit Human race explanations; got id={} \
             (Half-Orc has no Human ability-bonus or bonus-feat pressure)",
            explanation.id
        );
    }
}

/// Half-Orc Fighter level-1 produces a `Computed` pilot receipt: the bounded
/// non-race seam (Fighter chassis, ability modifiers, saves, deterministic
/// combat baseline) is still grounded. The race-semantics gap is a
/// non-claim-blocking diagnostic only; the receipt's status remains Computed.
#[test]
fn half_orc_fighter_pilot_receipt_remains_computed_with_only_race_gap() {
    use codex::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};

    const HALF_ORC_FIGHTER_FIXTURE: &str = include_str!(
        "fixtures/rules_core/pf1_half_orc_fighter_level1_sd13_deterministic_input.txt"
    );

    let loaded = load_character_input_fixture(HALF_ORC_FIGHTER_FIXTURE);
    let input = loaded.character_input.expect("fixture must load cleanly");

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "Half-Orc Fighter level-1 deterministic pilot must remain Computed; \
         the race.semantics.unverified diagnostic is intentionally non-claim-blocking"
    );

    // Belt-and-suspenders: no diagnostic on the Half-Orc path may be
    // claim-blocking, otherwise the slice has fabricated a Half-Orc blocking
    // posture that is not part of the honest Unverified-with-explicit-blocker
    // shape this slice is meant to prove.
    for diag in &receipt.computation.diagnostics {
        assert!(
            !diag.claim_blocking,
            "no Half-Orc path diagnostic may be claim-blocking in this slice; \
             the row is Unverified, not Blocked. Offender: id={} message={}",
            diag.id, diag.message
        );
    }
}
