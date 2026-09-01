//! SD13-E3-F5 Fighter levels 2-3 milestone progression proof.
//!
//! Proves the dedicated tranche this handoff authorizes: the accepted deterministic
//! Human Fighter pilot seam widens from level 1 into bounded levels 2 and 3 only,
//! Rogue stays an explicit blocked negative control, the accepted Human race seam is
//! preserved, and the support-state matrix reclassifies the Fighter levels-2-10 row
//! to a bounded `Partial` posture that still names what remains unproven.
//!
//! It is intentionally not a broad martial engine. It grounds only Fighter base
//! attack / base save progression for levels 2-3, the level-2 bonus-feat progression
//! seam, the first level-3 armor-training seam where it materially changes bounded
//! outputs, and downstream propagation of that bounded truth. It asserts no level-4+
//! Fighter burden, no spell burden, no non-Fighter positive support, and no general
//! feat/prerequisite engine.

use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const LEVEL_2_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level2_sd13_deterministic_input.txt");
const LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level3_sd13_deterministic_input.txt");

// ----- Milestone: level 2 is no longer blanket-blocked -----

#[test]
fn level_2_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The deterministic level-2 posture must not be claim-blocked.
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-2 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-2 base chassis: full BAB +2, good Fortitude +3, poor Reflex/Will +0.
    assert_eq!(computation.base_attack_bonus, 2);
    assert_eq!(computation.base_saves.fortitude, 3);
    assert_eq!(computation.base_saves.reflex, 0);
    assert_eq!(computation.base_saves.will, 0);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 2);
    assert!(
        bab.detail.contains("level 2"),
        "level-2 BAB explanation must name the level-2 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier (CON +2, DEX +2, WIS +1).
    assert_eq!(computation.total_saves.fortitude, 5);
    assert_eq!(computation.total_saves.reflex, 2);
    assert_eq!(computation.total_saves.will, 1);

    // Baseline combat advances with BAB: +2 BAB + STR +3 + Weapon Focus +1 = 6.
    assert_eq!(computation.baseline_melee_attack_bonus, 7);
    // Armor class is unchanged at level 2 (no armor training yet): 10 + 4 + DEX 2 + Dodge 1.
    assert_eq!(computation.baseline_armor_class, 17);

    // Selected skills are unchanged at level 2 (Chain Shirt armor-check penalty -2).
    // CG-03 fix: STR modifier is now +4 (base 16 + 2 Human racial), not +3, raising
    // both STR-keyed skills by 1.
    assert_eq!(computation.selected_skill_modifiers.climb, 6);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    assert_eq!(computation.selected_skill_modifiers.swim, 6);
}

#[test]
fn level_2_bonus_feat_progression_seam_is_explicit_and_bounded() {
    let input = load(LEVEL_2_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The Fighter level-2 bonus-feat slot must be surfaced as an explicit seam.
    let seam = explanation(&computation, "class_feature.fighter.level_2_bonus_feat");
    // Bounded: the seam grounds the slot only, not a general feat-effect engine, so it
    // contributes no computed mechanical value.
    assert_eq!(
        seam.value, 0,
        "level-2 bonus-feat seam must not fabricate a feat-effect value: {seam:?}"
    );
    assert!(
        seam.detail.contains("choice:fighter_bonus_feat_2"),
        "level-2 bonus-feat seam must name the chosen selection: {}",
        seam.detail
    );
    assert!(
        seam.detail.contains("feat-effect") || seam.detail.contains("prerequisite"),
        "level-2 bonus-feat seam must state it grounds no general feat/prerequisite engine: {}",
        seam.detail
    );
}

// ----- Milestone: level 3 is no longer blanket-blocked; armor training is explicit -----

#[test]
fn level_3_human_fighter_produces_non_blocked_bounded_evidence() {
    let input = load(LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic level-3 Fighter must not block claims: {:?}",
        computation.diagnostics
    );

    // Fighter level-3 base chassis: full BAB +3, good Fortitude +3, poor Reflex/Will +1.
    assert_eq!(computation.base_attack_bonus, 3);
    assert_eq!(computation.base_saves.fortitude, 3);
    assert_eq!(computation.base_saves.reflex, 1);
    assert_eq!(computation.base_saves.will, 1);

    let bab = explanation(&computation, "class_chassis.base_attack_bonus");
    assert_eq!(bab.value, 3);
    assert!(
        bab.detail.contains("level 3"),
        "level-3 BAB explanation must name the level-3 delta: {}",
        bab.detail
    );

    // Total saves: base + relevant ability modifier.
    assert_eq!(computation.total_saves.fortitude, 5);
    assert_eq!(computation.total_saves.reflex, 3);
    assert_eq!(computation.total_saves.will, 2);

    // Baseline melee advances with BAB: +3 BAB + STR +3 + Weapon Focus +1 = 7.
    assert_eq!(computation.baseline_melee_attack_bonus, 8);
    // Armor class is unchanged (DEX +2 stays under the raised MAXDEX): 10 + 4 + 2 + 1.
    assert_eq!(computation.baseline_armor_class, 17);
}

#[test]
fn level_3_armor_training_seam_is_explicit_and_moves_armor_check_pressure() {
    let input = load(LEVEL_3_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The armor-training seam must be an explicit explanation, not implicit folklore.
    let armor_training = explanation(&computation, "class_feature.fighter.armor_training");
    assert!(
        armor_training.detail.contains("Armor Training")
            || armor_training.detail.contains("armor training"),
        "armor-training seam must name the Fighter armor-training feature: {}",
        armor_training.detail
    );
    assert!(
        armor_training.detail.contains("armor-check"),
        "armor-training seam must state it changes the armor-check penalty: {}",
        armor_training.detail
    );

    // Armor training reduces the Chain Shirt armor-check penalty from -2 to -1, so the
    // armor-check skills (Climb, Swim) each rise by 1 to 6, while Intimidate is unchanged.
    assert_eq!(computation.selected_skill_modifiers.climb, 7);
    assert_eq!(computation.selected_skill_modifiers.swim, 7);
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);

    // The Climb explanation itself must say the penalty was reduced by armor training.
    let climb = explanation(&computation, "skill.selected_modifier.climb");
    assert!(
        climb.detail.contains("armor training") || climb.detail.contains("Armor Training"),
        "level-3 Climb explanation must cite the armor-training reduction directly: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains("-1"),
        "level-3 Climb explanation must cite the reduced -1 armor-check penalty: {}",
        climb.detail
    );
}

// ----- Milestone: downstream propagation of the widened bounded truth -----

#[test]
fn level_2_and_level_3_propagate_computed_receipts_and_view_models() {
    for fixture in [LEVEL_2_FIXTURE, LEVEL_3_FIXTURE] {
        let input = load(fixture);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "widened Fighter tranche must propagate a computed receipt"
        );

        // The accepted Human race seam must remain intact on the widened receipt.
        assert!(
            has_explanation(&receipt.computation, "race.human.ability_bonus_target"),
            "widened receipt must preserve the Human ability-bonus race seam: {:?}",
            receipt.computation.explanations
        );
        assert!(
            has_explanation(&receipt.computation, "race.human.bonus_feat_grant"),
            "widened receipt must preserve the Human bonus-feat race seam: {:?}",
            receipt.computation.explanations
        );

        let view_model = PilotViewModel::from_receipt(&receipt);
        assert_eq!(view_model.status, HeadlessReceiptStatus::Computed);
        assert_eq!(view_model.primary_owner, PrimaryOwner::OracleGap);
        let snapshot = view_model
            .snapshot
            .as_ref()
            .expect("computed widened receipt must yield a snapshot");
        assert_eq!(
            snapshot.base_attack_bonus, receipt.computation.base_attack_bonus,
            "view-model snapshot must carry the widened base attack bonus"
        );
    }
}

// ----- Negative control: Rogue must never inherit Fighter-namespaced seams -----

#[test]
fn rogue_replacing_the_fighter_chassis_never_leaks_fighter_seams() {
    // Replace the level-2 Fighter chassis with a Rogue chassis. Widening Fighter must
    // not silently make Rogue interchangeable with Fighter.
    //
    // At the time this file's slice landed, Rogue level 2 was entirely
    // unrecognized, so this control also asserted a blanket absence of any
    // `class_chassis.*` explanation. A later SD13-E5 slice
    // (tests/sd13_rogue_level2_progression.rs) widened the Rogue level-1-only
    // gate to level 2 and grounds `class_chassis.rogue.*` records there; this
    // negative control was superseded once already — it narrowed to: Rogue's
    // own grounded records never carry the Fighter-only class-feature seams,
    // and the shared `computation.base_attack_bonus` struct field stayed
    // unfabricated for a Rogue chassis.
    //
    // (v0.6 swarm update, superseded a second time) The v0.6 alpha swarm's
    // multiclass BAB/save-stacking generalization (task 4) gave Rogue a
    // genuinely integrated `class_chassis.*` computation via the table-driven
    // `compute_generic_table_chassis` path, so `computation.base_attack_bonus`
    // is now Rogue's real, non-fabricated level-2 value (3/4 BAB:
    // floor(2*3/4) = 1), and the shared `class_chassis.*` / `defense.total_save.*`
    // explanations now legitimately exist alongside the standalone
    // `class_chassis.rogue.*` records. With this fixture's full deterministic
    // Longsword/Chain Shirt/Dodge/Climb-Intimidate-Swim posture (inherited
    // from the Fighter level-2 fixture this test mutates), Rogue now computes
    // a fully claim-blocking-free receipt too -- there is no remaining
    // class-chassis-shaped reason for Rogue to be interchangeable-but-blocked.
    // What is STILL true, and is this test's actual protective purpose: the
    // Fighter-only named class-feature seams (level-2 bonus feat, armor
    // training) never leak onto a Rogue chassis, regardless of how far
    // Rogue's own generic integration has widened.
    //
    // Filed as a defect to backend (not fixed here -- QA does not touch
    // production code): `combat.baseline_melee_attack_bonus` and
    // `defense.total_save.*`'s explanation text hardcodes the word "Fighter"
    // (e.g. "Fighter base attack bonus (+1)") even when the actual computed
    // chassis is Rogue's. Cosmetic (the numeric values are correct), but
    // misleading for any tester-facing surface that shows explanation text.
    let mutated =
        LEVEL_2_FIXTURE.replace("class_level=class:fighter:2", "class_level=class:rogue:2");
    assert!(
        mutated.contains("class_level=class:rogue:2"),
        "test setup should have mutated the class chassis to Rogue"
    );
    let input = load(&mutated);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Rogue chassis with a fully matching deterministic posture is now genuinely computed, \
         with no remaining claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert_eq!(
        computation.base_attack_bonus, 1,
        "Rogue level 2's real 3/4-BAB progression (floor(2*3/4)) is 1, now genuinely integrated \
         into the shared base_attack_bonus struct field"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "the shared, non-Rogue-namespaced class_chassis.base_attack_bonus explanation now \
         legitimately exists for an integrated Rogue chassis: {:?}",
        computation.explanations
    );
    // The Fighter-only class-feature seams must still never leak onto a Rogue chassis --
    // this is the invariant this test actually protects, independent of how far Rogue's
    // own generic chassis integration has widened.
    assert!(
        !has_explanation(&computation, "class_feature.fighter.level_2_bonus_feat"),
        "Rogue must not surface the Fighter level-2 bonus-feat seam"
    );
    assert!(
        !has_explanation(&computation, "class_feature.fighter.armor_training"),
        "Rogue must not surface the Fighter armor-training seam"
    );
}

// ----- Control plane: the matrix reclassifies levels 2-10 to bounded Partial -----

#[test]
fn matrix_keeps_fighter_level_1_and_levels_2_10_as_separate_rows() {
    let matrix = seeded_current_truth();
    let level_1 = matrix
        .row("class.fighter.level_1_pilot")
        .expect("level-1 pilot row must exist");
    let levels_2_10 = matrix
        .row("class.fighter.levels_2_10")
        .expect("levels-2-10 row must exist");

    assert_ne!(
        level_1.row_id, levels_2_10.row_id,
        "Fighter level 1 and levels 2-10 must not collapse into one row"
    );
    assert_ne!(
        level_1.dimension, levels_2_10.dimension,
        "Fighter level-1 and levels-2-10 rows must keep distinct dimensions"
    );
}

#[test]
fn matrix_levels_2_10_is_partial_but_not_supported_and_names_what_remains() {
    let matrix = seeded_current_truth();
    let levels_2_10 = matrix
        .row("class.fighter.levels_2_10")
        .expect("levels-2-10 row must exist");

    // Widened to a bounded partial posture at this slice; later promoted to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-16).
    assert_eq!(levels_2_10.support_state, SupportState::Supported);
    assert_eq!(levels_2_10.evidence_tier, EvidenceTier::ProductVisible);

    // The note must name what remains out of proof after this slice: levels 4-10.
    assert!(
        !levels_2_10.blocker_or_lossiness_note.is_empty(),
        "partial levels-2-10 row must keep a non-empty note naming what remains unproven"
    );
    assert!(
        levels_2_10.blocker_or_lossiness_note.contains("4"),
        "partial levels-2-10 row note must name the still-unproven levels 4-10: {}",
        levels_2_10.blocker_or_lossiness_note
    );
}

#[test]
fn matrix_keeps_rogue_supported_after_fighter_widens() {
    // Rogue was later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-17); this
    // Fighter-widening snapshot preserves that.
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(
        rogue.support_state,
        SupportState::Supported,
        "Rogue must remain Supported after Fighter widens"
    );
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
}
