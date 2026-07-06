//! SD13-E2-F3a Half-Elf bounded race-semantics proof surface.
//!
//! Pins the recognition-only posture of the `race:half-elf` chosen-race seam
//! that this slice introduces. It deliberately asserts **no** Half-Elf
//! computed mechanic — there is no Half-Elf deterministic pilot fixture, no
//! `race.half_elf.ability_bonus_target` analog, no `race.half_elf.bonus_feat_grant`
//! analog, no sleep-immunity or low-light-vision claim, no favored-class or
//! multiclass adaptability claim, and no +2 Listen/Spot/Search skill-focus
//! claim. The single new bounded artifact is the
//! `race.half_elf.bounded_semantics` non-claim-blocking diagnostic that names
//! every Half-Elf PF1 Core Rulebook racial trait and explicitly states that
//! none of them are computed.
//!
//! Scope parallels the SD13-E2-R2 (Human) slice: visible, bounded, honest. The
//! matrix row stays `Unverified` / `Observed` because this slice grounds
//! recognition only; promoting to `Computed` would falsely assert bounded
//! mechanical truth this slice deliberately does not assert. Promotion is the
//! job of a later SD13-E2 Half-Elf computed-mechanic slice.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, build_pilot_headless_receipt,
};
use codex::rules_core::support_state_matrix::seeded_sd13_e1_f1_current_truth;

const HALF_ELF_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_half_elf_fighter_level1_sd13_deterministic_input.txt"
);

fn load_half_elf_input() -> CharacterInput {
    let result = load_character_input_fixture(HALF_ELF_FIXTURE);
    assert!(
        result.diagnostics.is_empty(),
        "Half-Elf deterministic input fixture must parse without diagnostics, got: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("Half-Elf fixture must produce a CharacterInput record when diagnostics are empty")
}

#[test]
fn half_elf_fixture_parses_as_chosen_input() {
    // Acceptance of chosen input is the bounded surface. The Half-Elf fixture
    // intentionally omits the Human-specific choice slots (`choice:human_ability_bonus`
    // and `choice:human_bonus_feat`) so the named-race dispatch routes to the
    // Half-Elf branch, not the Human one.
    let input = load_half_elf_input();
    assert_eq!(
        input.chosen.race_id, "race:half-elf",
        "fixture must name race:half-elf as the chosen race_id"
    );
    assert_eq!(input.case_id.as_deref(), Some("pf1-crb-half-elf-fighter-level1"));
    assert_eq!(input.source_package_id, "pf1.core_rulebook");
}

#[test]
fn half_elf_receipt_carries_exactly_one_bounded_race_semantics_diagnostic() {
    let input = load_half_elf_input();
    let receipt = build_pilot_headless_receipt(&input);

    let race_diags: Vec<&_> = receipt
        .computation
        .diagnostics
        .iter()
        .filter(|d| d.id.starts_with("race."))
        .collect();

    assert_eq!(
        race_diags.len(),
        1,
        "Half-Elf receipt must carry exactly one race-prefixed diagnostic, got {}: {:?}",
        race_diags.len(),
        race_diags.iter().map(|d| (&d.id, d.claim_blocking)).collect::<Vec<_>>()
    );

    let bounded = race_diags[0];
    assert_eq!(
        bounded.id, "race.half_elf.bounded_semantics",
        "Half-Elf receipt must carry the bounded Half-Elf race-semantics diagnostic"
    );
    assert!(
        !bounded.claim_blocking,
        "the bounded Half-Elf race-semantics diagnostic must be non-claim-blocking so the deterministic pilot still reports computed evidence: {bounded:?}"
    );
    assert!(
        bounded.message.contains("race:half-elf"),
        "bounded diagnostic message must name the chosen race_id for downstream reconciliation: {bounded:?}"
    );
}

#[test]
fn half_elf_receipt_carries_no_half_elf_computed_mechanic_explanations() {
    let input = load_half_elf_input();
    let receipt = build_pilot_headless_receipt(&input);

    // No `race.half_elf.ability_bonus_target` analog because there is no
    // Half-Elf deterministic pilot fixture in this slice; minting one would
    // assert bounded mechanical truth the slice deliberately does not assert.
    for forbidden in [
        "race.half_elf.ability_bonus_target",
        "race.half_elf.bonus_feat_grant",
        "race.half_elf.immunity_sleep",
        "race.half_elf.low_light_vision",
        "race.half_elf.skill_focus_listen",
        "race.half_elf.skill_focus_spot",
        "race.half_elf.skill_focus_search",
        "race.half_elf.favored_class",
        "race.half_elf.multiclass_adaptability",
    ] {
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id == forbidden),
            "Half-Elf receipt must NOT carry a '{forbidden}' explanation: this slice grounds no Half-Elf computed mechanic"
        );
    }
}

#[test]
fn half_elf_path_remains_computed_with_no_claim_blocking_diagnostics() {
    let input = load_half_elf_input();
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "Half-Elf fixture must reach the Computed status: recognition-only is not a claim-blocking posture. Diagnostics: {:?}",
        receipt.computation.diagnostics
    );
    assert!(
        !receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "Half-Elf receipt must carry zero claim-blocking diagnostics; all diagnostics are explicitly non-claim-blocking. Got: {:?}",
        receipt
            .computation
            .diagnostics
            .iter()
            .filter(|d| d.claim_blocking)
            .map(|d| &d.id)
            .collect::<Vec<_>>()
    );
}

#[test]
fn half_elf_path_does_not_emit_any_human_race_seam_explanation() {
    let input = load_half_elf_input();
    let receipt = build_pilot_headless_receipt(&input);

    // The named-race dispatcher must not leak any `race.human.*` explanation
    // into a Half-Elf-only receipt: the Half-Elf fixture omits the
    // Human-specific choice slots, and the dispatcher routes by `race_id`, so
    // no Human explanation or diagnostic should appear.
    assert!(
        !receipt
            .computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("race.human.")),
        "Half-Elf receipt must not carry any race.human.* explanation (named-race dispatcher should route by race_id): {:?}",
        receipt
            .computation
            .explanations
            .iter()
            .map(|e| &e.id)
            .collect::<Vec<_>>()
    );
    assert!(
        !receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics"),
        "Half-Elf receipt must not carry the Human-bounded-semantics diagnostic (named-race dispatcher should route by race_id): {:?}",
        receipt
            .computation
            .diagnostics
            .iter()
            .map(|d| &d.id)
            .collect::<Vec<_>>()
    );
    // The catch-all `race.semantics.unverified` diagnostic must NOT fire for
    // a Half-Elf chosen race either: Half-Elf has a named, recognized branch.
    assert!(
        !receipt
            .computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.semantics.unverified"),
        "Half-Elf receipt must not carry the catch-all race.semantics.unverified diagnostic: Half-Elf has its own named branch"
    );
}

#[test]
fn half_elf_diagnostic_names_the_full_pf1_half_elf_trait_scope() {
    let input = load_half_elf_input();
    let receipt = build_pilot_headless_receipt(&input);

    let bounded = receipt
        .computation
        .diagnostics
        .iter()
        .find(|d| d.id == "race.half_elf.bounded_semantics")
        .expect("Half-Elf receipt must carry the bounded Half-Elf diagnostic");

    // The diagnostic must explicitly name every Half-Elf trait the slice does
    // NOT ground, so a downstream reader cannot infer any of them by omission.
    for trait_token in [
        "ability-bonus",
        "sleep",
        "low-light vision",
        "Listen/Spot/Search",
        "favored class",
        "multiclass",
    ] {
        assert!(
            bounded.message.contains(trait_token),
            "bounded Half-Elf diagnostic must explicitly name the '{trait_token}' trait the slice leaves unverified, so it cannot be silently inferred by omission: {bounded:?}"
        );
    }
}

#[test]
fn half_elf_row_in_matrix_stays_unverified_with_named_blocker_note() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let row = matrix
        .row("race.half_elf.bounded_semantics")
        .expect("seeded matrix must continue to carry the race.half_elf.bounded_semantics row");

    use codex::rules_core::support_state_matrix::{EvidenceTier, SupportState};

    // The Half-Elf row stays Unverified/Observed at this slice because
    // recognition-only is not bounded mechanical proof.
    assert_eq!(
        row.support_state,
        SupportState::Unverified,
        "race.half_elf.bounded_semantics must remain Unverified at this slice (no computed Half-Elf mechanic is grounded)"
    );
    assert_eq!(
        row.evidence_tier,
        EvidenceTier::Observed,
        "race.half_elf.bounded_semantics must remain Observed at this slice (recognition-only is not Computed evidence)"
    );
    assert!(
        !row.blocker_or_lossiness_note.is_empty(),
        "the Half-Elf row must carry a non-empty blocker note naming what remains unverified"
    );
    assert!(
        row.blocker_or_lossiness_note.contains("race:half-elf")
            && row.blocker_or_lossiness_note.contains("no Half-Elf ability-bonus seam"),
        "the Half-Elf blocker note must explicitly name the chosen race and the absence of any Half-Elf ability-bonus computed seam, so a downstream reader cannot infer either by omission: {row:?}"
    );
    assert!(
        row.next_required_uplift.contains("Half-Elf")
            && row.next_required_uplift.contains("computed-mechanic"),
        "the Half-Elf next-required-uplift must name the future computed-mechanic slice explicitly: {row:?}"
    );
}
