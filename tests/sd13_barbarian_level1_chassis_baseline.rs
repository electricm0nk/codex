//! SD13-E3/E5 Barbarian level-1 martial chassis baseline proof.
//!
//! Proves the SD13-E3 barbarian slice: the live rules-core surface ingests a
//! deterministic Human `class:barbarian:1` input, leaves direct computed
//! evidence that acknowledges the bounded level-1 martial chassis identity
//! rather than treating it as an undocumented packet placeholder, and
//! grounds three of the four originally-named martial pillar burdens
//! directly: base-attack progression, base-save progression, and the
//! fast-movement +10 ft. speed value. The SD13-E5 slice then resolves the
//! fourth originally-named burden (the illiteracy trait) as vacuous — the
//! PF1 Core Rulebook Barbarian is NOT illiterate; illiteracy is a D&D 3.5e
//! Barbarian trait that never existed in PF1 — and grounds Rage's flat
//! numeric surface: rage rounds per day (4 + Constitution modifier) and the
//! flat rage constants (+4 morale Strength, +4 morale Constitution, +2
//! morale on Will saves, -2 AC), values only. The honest remaining burden is
//! now the rage-state execution engine, named by its own claim-blocking
//! diagnostic. It also pins the matrix reclassification of the barbarian row
//! at `Partial` / `Computed`.
//!
//! It is intentionally not a martial class engine. The grounded base-attack
//! and base-save explanations mirror the Fighter formula shape (full BAB,
//! good Fortitude, poor Reflex/Will) but are standalone records: they are not
//! wired into `PilotBaseChassisComputation.base_attack_bonus` or into
//! `compute_total_saves`/`compute_combat_baseline`, so the integrated pilot
//! surface still reports a blocked posture. The grounded fast-movement
//! explanation asserts only the flat +10 ft. value; it grounds no
//! armor/encumbrance-state check engine (no such engine exists anywhere in
//! this codebase yet). The grounded rage records assert only flat values;
//! they ground no rage-state engine: no activation/deactivation, no
//! round-by-round consumption of rage rounds, no fatigue after rage, and no
//! temporary application of the rage constants to any integrated total. This
//! slice still grounds no weapon familiarity, no level-2+ martial
//! progression, and no skill-list expansion (barbarian class skills). It
//! also preserves the accepted Fighter 1-3 truth, the Rogue blocked negative
//! control, the Paladin/Ranger blocked hybrid negative controls, and the
//! Human race/interaction truth.

use codex::rules_core::character_input::{ActiveState, ClassAbilityActivation};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    HeadlessReceiptStatus,
    PilotBaseChassisComputation,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const BARBARIAN_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level1_sd13_deterministic_input.txt"
);

const BARBARIAN_LOW_CONSTITUTION_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level1_low_constitution_sd13_deterministic_input.txt"
);

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

// ----- Direct runtime evidence: the martial chassis identity is acknowledged -----

#[test]
fn barbarian_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Barbarian chassis identity is recognized on
    // the compute path, not silently dropped as an undocumented packet placeholder.
    let chassis =
        explanation(&computation, "class_chassis.barbarian.bounded_progression");
    assert!(
        chassis.detail.contains("class:barbarian") && chassis.detail.contains("level 1"),
        "barbarian chassis recognition must name the class:barbarian:1 identity: {}",
        chassis.detail
    );
    // (v0.6 alpha swarm, risks item 8) Barbarian is now recognized by
    // table_class_id (full BAB), so the integrated base_attack_bonus field
    // and the generic class-chassis explanation ARE real: 1 * 1 = 1 at
    // level 1. This is no longer a "recognition only, no fabrication" case
    // for the integrated field -- it's genuinely computed.
    assert_eq!(
        computation.base_attack_bonus, 1,
        "barbarian is now recognized by table_class_id; level 1 full BAB is +1"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "barbarian is now recognized by table_class_id and must surface its base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (STR 16 -> +3).
    assert_eq!(computation.ability_modifiers.strength, 4);
}

// ----- Grounded: base-attack, base-save, and fast-movement pillar burdens -----

#[test]
fn barbarian_level1_grounds_base_attack_base_save_and_fast_movement() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Base-attack progression is now grounded as a standalone explanation record
    // (full BAB, same formula shape as Fighter's cr_classes.lst:139 base-attack
    // progression), and its old "unsupported" diagnostic no longer exists.
    let base_attack = explanation(&computation, "class_chassis.barbarian.base_attack_bonus");
    assert_eq!(base_attack.value, 1, "Barbarian level 1 full BAB must be +1");
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.barbarian.bounded_progression.base_attack.unsupported"),
        "the grounded base-attack burden must no longer surface its old unsupported diagnostic: {:?}",
        computation.diagnostics
    );

    // Base-save progression is now grounded: good Fortitude, poor Reflex/Will.
    let fortitude = explanation(&computation, "class_chassis.barbarian.base_save.fortitude");
    assert_eq!(fortitude.value, 2, "Barbarian level 1 good Fortitude save must be +2");
    let reflex = explanation(&computation, "class_chassis.barbarian.base_save.reflex");
    assert_eq!(reflex.value, 0, "Barbarian level 1 poor Reflex save must be +0");
    let will = explanation(&computation, "class_chassis.barbarian.base_save.will");
    assert_eq!(will.value, 0, "Barbarian level 1 poor Will save must be +0");
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.barbarian.bounded_progression.base_save.unsupported"),
        "the grounded base-save burden must no longer surface its old unsupported diagnostic: {:?}",
        computation.diagnostics
    );

    // Fast movement is now grounded as a flat +10 ft. value only, not a computed
    // armor/encumbrance-state check.
    let fast_movement = explanation(&computation, "class_chassis.barbarian.fast_movement");
    assert_eq!(fast_movement.value, 10, "Barbarian fast movement must be +10 ft.");
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.barbarian.bounded_progression.fast_movement.unsupported"),
        "the grounded fast-movement burden must no longer surface its old unsupported diagnostic: {:?}",
        computation.diagnostics
    );

    // (v0.6 alpha swarm, risks item 8) The standalone barbarian-namespaced
    // record above and the integrated base_attack_bonus field are two
    // separate computations that now happen to agree (both real full-BAB
    // formulas): the integrated field is no longer "unsupported for
    // Barbarian" -- table_class_id recognizes Barbarian too.
    assert_eq!(
        computation.base_attack_bonus, 1,
        "barbarian is now recognized by table_class_id; the integrated base_attack_bonus field \
         is genuinely computed (full BAB), independent of the standalone record above"
    );
}

// ----- Rules correction: the illiteracy burden was vacuous under pf1.core_rulebook -----

#[test]
fn barbarian_level1_resolves_illiteracy_as_vacuous_under_pf1_core_rulebook() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The PF1 Core Rulebook Barbarian is NOT illiterate: illiteracy is a D&D 3.5e
    // Barbarian trait that never existed in PF1, so the burden named by the old
    // diagnostic was vacuous under the fixture's `pf1.core_rulebook` source package.
    // The resolution is documented as a grounded value-0 record, not silently dropped.
    let illiteracy_absent = explanation(&computation, "class_chassis.barbarian.illiteracy_absent");
    assert_eq!(
        illiteracy_absent.value, 0,
        "the illiteracy-absent record documents a rules correction; it carries no mechanical value"
    );
    for token in ["3.5", "PF1 Core Rulebook", "vacuous"] {
        assert!(
            illiteracy_absent.detail.contains(token),
            "the illiteracy-absent record must document the rules correction ('{token}'): {}",
            illiteracy_absent.detail
        );
    }

    // The old vacuous claim-blocking diagnostic is retired outright.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.barbarian.bounded_progression.illiteracy.unsupported"),
        "the vacuous illiteracy blocker must be retired: {:?}",
        computation.diagnostics
    );
}

// ----- Grounded: Rage's flat numeric surface (values only, no application) -----

#[test]
fn barbarian_level1_grounds_rage_rounds_per_day_from_constitution() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Rage rounds per day = 4 + Constitution modifier (PF1 Core Rulebook Rage).
    // The fixture's Con 16 gives modifier +3, so 4 + 3 = 7.
    let rage_rounds = explanation(&computation, "class_chassis.barbarian.rage_rounds_per_day");
    assert_eq!(
        rage_rounds.value, 7,
        "Barbarian level 1 rage rounds per day must be 4 + Con modifier (+3) = 7"
    );
    assert!(
        rage_rounds.detail.contains("4 + ") && rage_rounds.detail.contains("Constitution"),
        "rage rounds per day must document the 4 + Constitution modifier formula: {}",
        rage_rounds.detail
    );
}

#[test]
fn barbarian_level1_claim_blocks_rage_rounds_per_day_at_low_constitution() {
    let input = load(BARBARIAN_LOW_CONSTITUTION_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Constitution 1 gives modifier -5, so 4 + Constitution modifier = -1: a
    // non-positive count with no PF1 Core Rulebook meaning as "rounds per day".
    // The compute seam must not assert a fabricated negative value; it must
    // claim-block the record instead.
    assert!(
        computation
            .explanations
            .iter()
            .all(|e| e.id != "class_chassis.barbarian.rage_rounds_per_day"),
        "a non-positive rage rounds per day must not be grounded as an explanation: {:?}",
        computation.explanations
    );
    let blocked = claim_blocking(
        &computation,
        "class_chassis.barbarian.rage_rounds_per_day.unsupported",
    );
    assert!(
        blocked.message.contains("-1"),
        "the claim-blocking diagnostic must name the computed non-positive sum: {}",
        blocked.message
    );

    // The flat rage constants are unaffected by Constitution: they still ground
    // exactly as on the Con 16 fixture.
    for id in [
        "class_chassis.barbarian.rage.strength_morale_bonus",
        "class_chassis.barbarian.rage.constitution_morale_bonus",
        "class_chassis.barbarian.rage.will_save_morale_bonus",
        "class_chassis.barbarian.rage.armor_class_penalty",
    ] {
        explanation(&computation, id);
    }

    // (v0.6 alpha swarm, risks item 8) The old unconditional
    // "rage_execution.unsupported" diagnostic is retired outright -- Rage
    // execution is now a real, conditional engine (ground_or_block_barbarian_rage).
    // This fixture has no class_ability_activations entry for rage, a genuinely
    // valid "not currently raging" posture, so it grounds an honest explanation,
    // not a claim-blocking diagnostic.
    let not_raging = explanation(&computation, "class_feature.barbarian.rage_execution.not_raging");
    assert_eq!(
        not_raging.value, 0,
        "the not-raging record carries no fabricated mechanical value"
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.rage_execution")),
        "a genuinely valid not-raging posture must not claim-block on rage execution: {:?}",
        computation.diagnostics
    );
}

#[test]
fn barbarian_level1_grounds_flat_rage_constants_as_values_only() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The flat while-raging constants are grounded as value-only records: +4 morale
    // Strength, +4 morale Constitution, +2 morale on Will saves, -2 AC.
    for (id, expected) in [
        ("class_chassis.barbarian.rage.strength_morale_bonus", 4),
        ("class_chassis.barbarian.rage.constitution_morale_bonus", 4),
        ("class_chassis.barbarian.rage.will_save_morale_bonus", 2),
        ("class_chassis.barbarian.rage.armor_class_penalty", -2),
    ] {
        let record = explanation(&computation, id);
        assert_eq!(record.value, expected, "rage constant '{id}' must be {expected}");
    }

    // Values only, no application: the rage constants must not leak into any
    // integrated total — ability modifiers stay the un-raged fixture truth
    // (Str 16 -> +3, Con 16 -> +3), and no Fighter-style chassis is fabricated.
    assert_eq!(
        computation.ability_modifiers.strength, 4,
        "the +4 morale Strength rage constant must not be applied to the integrated Strength modifier"
    );
    assert_eq!(
        computation.ability_modifiers.constitution, 3,
        "the +4 morale Constitution rage constant must not be applied to the integrated Constitution modifier"
    );
    // (v0.6 alpha swarm, risks item 8) base_attack_bonus is genuinely 1 now
    // (Barbarian's real full-BAB formula via table_class_id), not a rage
    // constant leaking in -- the rage constants themselves stay unapplied
    // to any integrated total (proven above via the ability modifiers).
    assert_eq!(
        computation.base_attack_bonus, 1,
        "the integrated base_attack_bonus field reflects barbarian's real full-BAB formula, not \
         a fabricated rage-constant leak"
    );
}

// ----- Still blocked: a genuine rage-execution posture violation -----

#[test]
fn barbarian_level1_stays_blocked_on_a_genuine_rage_execution_violation() {
    // (v0.6 alpha swarm, risks item 8) The old unconditional
    // "rage_execution.unsupported" diagnostic is retired outright -- Rage
    // execution is now a real, conditional engine
    // (ground_or_block_barbarian_rage), mirroring the spell-posture classes'
    // shape: a bare fixture (not raging) is a genuinely valid PF1 posture
    // and grounds no claim-blocking diagnostic at all (see
    // `barbarian_level1_claim_blocks_rage_rounds_per_day_at_low_constitution`'s
    // own not-raging assertion). This test's purpose -- proving Rage still
    // claim-blocks on a genuine posture violation -- now needs a real
    // violation injected: an active rage activation whose rounds_consumed_today
    // exceeds the grounded rounds-per-day budget (4 + Con modifier +3 = 7 at
    // level 1 on this fixture).
    let mut input = load(BARBARIAN_FIXTURE);
    input.chosen.class_ability_activations.push(ClassAbilityActivation {
        ability_id: "rage".to_owned(),
        active_state: ActiveState::EquippedActive,
        rounds_consumed_today: Some(8),
    });
    let computation = compute_pilot_base_chassis(&input);

    let rage_execution = claim_blocking(
        &computation,
        "class_feature.barbarian.rage_execution.rounds_exceeded",
    );
    for token in ["rounds consumed", "exceeding", "rounds-per-day budget"] {
        assert!(
            rage_execution.message.contains(token),
            "barbarian rage-execution blocker must name the '{token}' burden: {}",
            rage_execution.message
        );
    }

    // No rage bonus/penalty is fabricated for an over-budget activation.
    assert!(
        !has_explanation(&computation, "class_feature.barbarian.rage_execution.active"),
        "an over-budget rage activation must not ground the active-rage explanation record"
    );

    // The integrated posture is still blocked overall (the generic GE-06
    // combat-baseline gate already claim-blocks this fixture regardless, and
    // the rage-execution violation adds a second, genuine claim-blocking
    // reason), never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked barbarian baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the barbarian path -----

#[test]
fn barbarian_baseline_preserves_human_race_seam() {
    let input = load(BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "barbarian baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "barbarian baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    // The bounded Human race-semantics note stays present and non-claim-blocking.
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "barbarian baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the barbarian path must not leak onto other classes -----

#[test]
fn fighter_paladin_ranger_do_not_gain_barbarian_recognition() {
    // A supported Fighter must not gain a barbarian-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(
            &fighter_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "the Fighter chassis must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "the Fighter chassis must not surface barbarian class-feature burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "the Fighter chassis must not surface barbarian-namespaced chassis explanations: {:?}",
        fighter_computation.explanations
    );

    // Paladin must stay a blocked hybrid baseline, never a barbarian baseline.
    let paladin = load(include_str!(
        "fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt"
    ));
    let paladin_computation = compute_pilot_base_chassis(&paladin);
    assert!(
        !has_explanation(
            &paladin_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "Paladin must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !paladin_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "Paladin must not surface barbarian class-feature burden diagnostics"
    );
    assert!(
        !paladin_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "Paladin must not surface barbarian-namespaced chassis explanations"
    );

    // Ranger must stay a blocked hybrid baseline, never a barbarian baseline.
    let ranger = load(include_str!(
        "fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt"
    ));
    let ranger_computation = compute_pilot_base_chassis(&ranger);
    assert!(
        !has_explanation(
            &ranger_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "Ranger must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !ranger_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "Ranger must not surface barbarian class-feature burden diagnostics"
    );
    assert!(
        !ranger_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "Ranger must not surface barbarian-namespaced chassis explanations"
    );

    // Rogue must stay a plain blocked negative control, never a barbarian baseline.
    let rogue_fixture = BARBARIAN_FIXTURE.replace("class:barbarian:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        !has_explanation(
            &rogue_computation,
            "class_chassis.barbarian.bounded_progression"
        ),
        "Rogue must not surface a barbarian-baseline recognition record"
    );
    assert!(
        !rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.barbarian.")),
        "Rogue must not surface barbarian class-feature burden diagnostics"
    );
    assert!(
        !rogue_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "Rogue must not surface barbarian-namespaced chassis explanations"
    );
}

#[test]
fn barbarian_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_barbarian_level2_progression.rs) widened the level-1-only gate
    // to level 2 (mirroring the Fighter/Paladin/Rogue level-range gate idiom) and
    // extended the base-attack/base-save/fast-movement/rage-rounds formulas; this
    // negative control is superseded, not violated — pin the new truth here too
    // so this file stays internally consistent.
    let level_2 = BARBARIAN_FIXTURE.replace("class:barbarian:1", "class:barbarian:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.barbarian.bounded_progression"),
        "level-2 Barbarian is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "class_chassis.barbarian.base_attack_bonus"),
        "level-2 Barbarian is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    // (v0.6 alpha swarm, risks item 8) No named Barbarian pillar diagnostic
    // remains at level 2 at all: rage execution is now conditional
    // (ground_or_block_barbarian_rage), and this bare fixture's genuinely
    // valid "not raging" posture grounds an honest explanation, not a
    // diagnostic -- see `barbarian_level1_stays_blocked_on_a_genuine_rage_
    // execution_violation` for the real, conditional rage-execution blocker.
    assert!(
        computation
            .diagnostics
            .iter()
            .all(|d| !d.id.starts_with("class_feature.barbarian.")),
        "level-2 Barbarian must not surface any named barbarian burden diagnostic on this \
         genuinely valid (not-raging) posture: {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Barbarian must still be claim-blocked by the generic chassis diagnostics \
         (the GE-06 combat-baseline/selected-skill gates)"
    );
}

#[test]
fn multiclass_barbarian_is_not_promoted_by_this_slice() {
    // A multiclass mix (Barbarian + Fighter) must not gain the bounded level-1
    // single-class martial recognition record and stays blocked.
    let multiclass = BARBARIAN_FIXTURE.replace(
        "class_level=class:barbarian:1",
        "class_level=class:barbarian:1\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.barbarian.bounded_progression"),
        "multiclass Barbarian must not gain the bounded level-1 single-class martial recognition record"
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.barbarian.")),
        "multiclass Barbarian must not surface the level-1 barbarian chassis explanations: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Barbarian must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies the barbarian row to Partial/Computed -----

#[test]
fn matrix_barbarian_row_is_partial_computed_and_names_rage_execution_as_still_unproven() {
    let matrix = seeded_current_truth();
    let barbarian = matrix
        .row("class.barbarian.bounded_progression")
        .expect("barbarian bounded_progression row must exist");

    // NOTE: this test's name and the original comment here ("Stays Partial/
    // Computed. The slice is bounded; we are not claiming Supported.")
    // reflect this row's state as of the SD13-E3 slice this file proves.
    // Later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    assert_eq!(barbarian.support_state, SupportState::Supported);
    assert_eq!(barbarian.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        barbarian.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        barbarian
            .grounding_ref
            .contains("sd13_barbarian_level1_chassis_baseline"),
        "barbarian row must cite the barbarian proof surface: {}",
        barbarian.grounding_ref
    );
    let note = barbarian.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "barbarian partial row must carry a note");
    // The illiteracy burden is resolved as vacuous (a rules correction, not an
    // uplift): the PF1 Core Rulebook Barbarian is not illiterate.
    for token in ["illiteracy", "vacuous", "3.5"] {
        assert!(
            note.contains(token),
            "barbarian partial note must record the illiteracy rules correction ('{token}'): {note}"
        );
    }
    // Rage's flat numeric surface is grounded (across level 1 and the later SD13-E5
    // level-2 through level-10 widenings plus the SD18 level-11 Greater Rage
    // widening); the
    // rage-state execution engine is the named remaining burden, and weapon
    // familiarity / level-12+ stay unclaimed.
    for token in [
        "base attack",
        "base save",
        "fast movement",
        "rage rounds",
        "rage execution",
        "weapon familiarity",
        "level-12+",
    ] {
        assert!(
            note.contains(token),
            "barbarian partial note must name '{token}': {note}"
        );
    }
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_current_truth();

    // Fighter rows were later promoted to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-16).
    for id in ["class.fighter.level_1_pilot", "class.fighter.levels_2_10"] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Supported,
            "row {id} must be Supported after the SD-19 class-row promotion"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    }

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences), then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Supported,
        "paladin row must be Supported after the SD-19 class-row promotion"
    );

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must be Supported after the SD-19 class-row promotion"
    );

    // Rogue was later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-17).
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Supported);
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);

    // No row is silently promoted to Supported or Lossy by this slice.
    assert!(
        !matrix
            .rows
            .iter()
            // school.abjuration/illusion.spell_reachability were later promoted to
            // Supported/Product-visible by SD-19's operator-driven UI-surfacing work
            // (2026-07-16) -- excluded here, not an unintended promotion by this slice.
            .any(|r| (r.support_state == SupportState::Supported
                && r.row_id != "school.abjuration.spell_reachability"
                && r.row_id != "school.illusion.spell_reachability"
                && r.row_id != "school.conjuration.spell_reachability"
                && r.row_id != "school.divination.spell_reachability"
                && r.row_id != "school.enchantment.spell_reachability"
                && r.row_id != "school.evocation.spell_reachability"
                && r.row_id != "school.necromancy.spell_reachability"
                && r.row_id != "school.transmutation.spell_reachability"
                && r.row_id != "school.universal.spell_reachability"
                && r.row_id != "equipment.arms_armor.equipment_reachability"
                && r.row_id != "equipment.general.equipment_reachability"
                && r.row_id != "equipment.magic_items.equipment_reachability"
                && r.row_id != "race.human.pilot_semantics"
                && r.row_id != "race.dwarf.bounded_semantics"
                && r.row_id != "race.elf.bounded_semantics"
                && r.row_id != "race.gnome.bounded_semantics"
                && r.row_id != "race.half_elf.bounded_semantics"
                && r.row_id != "race.half_orc.bounded_semantics"
                && r.row_id != "race.halfling.bounded_semantics"
                && r.row_id != "class.fighter.level_1_pilot"
                && r.row_id != "class.fighter.levels_2_10"
                && r.row_id != "class.monk.bounded_progression"
                && r.row_id != "class.druid.progression_and_spell_burden"
                && r.row_id != "class.barbarian.bounded_progression"
                && r.row_id != "class.cleric.progression_and_spell_burden"
                && r.row_id != "class.wizard.progression_and_spell_burden"
                && r.row_id != "class.rogue.bounded_progression"
                && r.row_id != "class.sorcerer.progression_and_spell_burden"
                && r.row_id != "class.bard.progression_and_spell_burden"
                && r.row_id != "class.paladin.hybrid_chassis_and_spell_burden"
                && r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"
                && r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "the barbarian slice must not promote any row to Supported or Lossy"
    );
}
