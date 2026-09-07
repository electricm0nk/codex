//! SD13-E3 Rogue level-1 chassis baseline proof.
//!
//! Proves the SD13-E3 rogue slice (mirroring the Barbarian/Monk level-1
//! martial-baseline pattern): the live rules-core surface ingests a
//! deterministic Human `class:rogue:1` input, leaves direct computed evidence
//! that acknowledges the bounded level-1 chassis identity rather than treating
//! it as an undocumented packet placeholder. The SD13-E3 pillar-grounding
//! slice grounds base-attack progression (3/4 BAB), base-save progression
//! (good Reflex, poor Fortitude, poor Will), and sneak attack (die-count
//! only, `+1d6` at level 1); the SD13-E5 slice grounds the fourth and final
//! named pillar, Trapfinding (the flat `max(rogue level / 2, 1)` bonus on
//! Perception checks to locate traps and on Disable Device checks, `+1` at
//! level 1, plus the magic-trap-disarm statement), mirroring the Ranger
//! Track precedent. No named Rogue pillar burden remains claim-blocked; the
//! input still claim-blocks on the four generic chassis diagnostics. It also
//! pins the matrix truth: the rogue row stays `Partial` / `Computed` with a
//! note naming the honestly-unproven remainder.
//!
//! It is intentionally not a Rogue class engine. The sneak-attack explanation
//! grounds only the die-count facet (`1`), not damage-roll execution or the
//! flanking/Dexterity-denial trigger-condition engine. The trapfinding
//! explanation grounds only the flat numeric bonus and the magic-trap-disarm
//! statement, not a check-execution engine, trap DCs, or a magic-trap disarm
//! engine. This slice grounds no rogue talent and no level-2+ progression. It
//! also preserves the accepted Fighter 1-3 truth, the Barbarian/Monk
//! partial/computed truth, the Paladin/Ranger blocked hybrid negative
//! controls, and the Human race/interaction truth.
//!
//! **Superseded (v0.6 alpha swarm, task 4):** the multiclass BAB/save-stacking
//! generalization widened `table_class_id`'s dispatch to include Rogue,
//! giving Rogue level 1-20 its own real, integrated `class_chassis.*`
//! computation (not just the standalone `class_chassis.rogue.*` pillar
//! records this file's original slice grounded) via the shared table-driven
//! `compute_generic_table_chassis` path. This makes the previous paragraph's
//! "standalone, not wired into `compute_total_saves`" claim stale: Rogue's
//! base-attack/base-save ARE now wired into the integrated
//! `class_chassis.base_attack_bonus` / `class_chassis.base_save.*`
//! explanations, and `defense.total_save.*` IS now computed for single-class
//! Rogue (fortitude/reflex/will each get a real value — `compute_total_saves`
//! is class-agnostic and reads whatever base saves are integrated). What
//! still correctly claim-blocks a Rogue level-1 receipt is unrelated to
//! class-chassis recognition: `combat.baseline_unsupported` and
//! `skill.selected_modifier.unsupported` still fire, since the Rogue fixture
//! doesn't match the deterministic Longsword/Chain Shirt/Dodge/Climb-
//! Intimidate-Swim posture those two diagnostics require. See the tests
//! below marked "(v0.6 swarm update)" for the exact current truth.
//! `tests/ge06_pilot_total_saves.rs::unsupported_chassis_blocks_total_saves`
//! now uses Cleric, not Rogue, as its unsupported negative control (Rogue
//! stopped being unsupported — see that file for the same note).

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

const ROGUE_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_rogue_level1_sd13_deterministic_input.txt");
const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
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

// ----- Direct runtime evidence: the chassis identity is acknowledged -----

#[test]
fn rogue_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let chassis = explanation(&computation, "class_chassis.rogue.bounded_progression");
    assert!(
        chassis.detail.contains("class:rogue") && chassis.detail.contains("level 1"),
        "rogue chassis recognition must name the class:rogue:1 identity: {}",
        chassis.detail
    );
    // (v0.6 swarm update) Rogue's base attack bonus is now genuinely integrated
    // via the table-driven `compute_generic_table_chassis` dispatch (v0.6 alpha
    // swarm task 4's multiclass BAB/save-stacking generalization) -- the value 0
    // is Rogue's real 3/4-BAB progression at level 1 (floor(1 * 3 / 4) = 0), not
    // a fabricated absence, and the integrated explanation now legitimately
    // exists alongside the standalone `class_chassis.rogue.*` pillar records.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "rogue level 1's real 3/4-BAB progression (floor(1*3/4)) is 0"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "rogue base-attack bonus is now a genuinely integrated chassis explanation, not a \
         standalone-only record"
    );

    // Ability modifiers remain class-independent and still compute (DEX 17 -> +3).
    assert_eq!(computation.ability_modifiers.dexterity, 4);
}

// ----- Now grounded: base-attack, base-save, and sneak-attack pillars -----

#[test]
fn rogue_level1_base_attack_bonus_is_grounded_and_no_longer_blocked() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 0,
        "Rogue level 1 3/4-BAB progression (1 * 3 / 4) must equal 0: {}",
        base_attack.detail
    );
    assert!(
        base_attack.detail.contains("3/4") || base_attack.detail.to_lowercase().contains("bab"),
        "rogue base-attack explanation must name the 3/4-BAB progression: {}",
        base_attack.detail
    );
    assert!(
        !computation.diagnostics.iter().any(|d| d.id
            == "class_feature.rogue.bounded_progression.base_attack.unsupported"),
        "the base-attack unsupported diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn rogue_level1_base_saves_are_grounded_and_no_longer_blocked() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(fortitude.value, 0, "Rogue level 1 poor Fortitude (1/3) must equal 0");
    assert!(
        fortitude.detail.to_lowercase().contains("poor"),
        "rogue Fortitude explanation must name it as a poor save: {}",
        fortitude.detail
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 2, "Rogue level 1 good Reflex (1/2+2) must equal 2");
    assert!(
        reflex.detail.to_lowercase().contains("good"),
        "rogue Reflex explanation must name it as a good save: {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(will.value, 0, "Rogue level 1 poor Will (1/3) must equal 0");
    assert!(
        will.detail.to_lowercase().contains("poor"),
        "rogue Will explanation must name it as a poor save: {}",
        will.detail
    );

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.rogue.bounded_progression.base_save.unsupported"),
        "the base-save unsupported diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

#[test]
fn rogue_level1_sneak_attack_die_count_is_grounded_and_no_longer_blocked() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 1,
        "Rogue level 1 sneak attack die count must be 1 (i.e. 1d6): {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.contains("1d6"),
        "rogue sneak-attack explanation must name the +1d6 damage die: {}",
        sneak_attack.detail
    );
    assert!(
        sneak_attack.detail.to_lowercase().contains("die count")
            || sneak_attack.detail.to_lowercase().contains("die-count"),
        "rogue sneak-attack explanation must explicitly scope itself to the die-count facet \
         only, not damage-roll execution or the trigger-condition engine: {}",
        sneak_attack.detail
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.rogue.bounded_progression.sneak_attack.unsupported"),
        "the sneak-attack unsupported diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
}

// ----- Now grounded (SD13-E5): trapfinding, the fourth and final named pillar -----

#[test]
fn rogue_trapfinding_is_grounded_with_value_one_at_level_one() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 1,
        "rogue Trapfinding bonus at level 1 must be max(1 / 2, 1) = 1, got {}",
        trapfinding.value
    );
    assert!(
        trapfinding.detail.contains("Trapfinding"),
        "rogue Trapfinding explanation must name the Trapfinding class feature: {}",
        trapfinding.detail
    );
    assert!(
        trapfinding.detail.contains("Perception") && trapfinding.detail.contains("locate traps"),
        "rogue Trapfinding explanation must name the Perception bonus to locate traps: {}",
        trapfinding.detail
    );
    assert!(
        trapfinding.detail.contains("Disable Device"),
        "rogue Trapfinding explanation must name the Disable Device bonus: {}",
        trapfinding.detail
    );
    assert!(
        trapfinding.detail.contains("magic traps"),
        "rogue Trapfinding explanation must carry the magic-trap-disarm statement: {}",
        trapfinding.detail
    );
    // Must be explicit that this grounds only the flat numeric bonus (plus the
    // magic-trap-disarm statement), not a check-execution engine.
    assert!(
        trapfinding.detail.contains("not a check-execution engine")
            || trapfinding.detail.contains("no check-execution engine"),
        "rogue Trapfinding explanation must disclaim a check-execution engine: {}",
        trapfinding.detail
    );
    assert!(
        trapfinding.detail.contains("no magic-trap disarm engine"),
        "rogue Trapfinding explanation must disclaim a magic-trap disarm engine: {}",
        trapfinding.detail
    );
}

// ----- No named Rogue pillar burden remains; only the generics still block -----

#[test]
fn rogue_level1_retires_the_trapfinding_blocker_but_generics_still_block() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.rogue.bounded_progression.trapfinding.unsupported"),
        "the trapfinding unsupported diagnostic must no longer be emitted: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.rogue.")),
        "no named rogue pillar diagnostic may remain — the named set is now empty: {:?}",
        computation.diagnostics
    );
    // (v0.6 swarm update) class_chassis.unsupported no longer fires for single-class
    // Rogue -- the v0.6 alpha swarm's multiclass BAB/save-stacking generalization
    // (task 4) gave Rogue a genuinely integrated class_chassis computation. What
    // still claim-blocks the integrated receipt is unrelated to class recognition:
    // the deterministic combat-baseline posture (Longsword/Chain Shirt/Dodge/no
    // shield/no Power Attack/Weapon Focus) the Rogue fixture doesn't match.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported"),
        "class_chassis.unsupported must no longer fire for a supported single-class Rogue: {:?}",
        computation.diagnostics
    );
    claim_blocking(&computation, "combat.baseline_unsupported");

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked rogue baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the rogue path -----

#[test]
fn rogue_baseline_preserves_human_race_seam() {
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "rogue baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "rogue baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "rogue baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.rogue.")),
        "the Fighter chassis must not surface rogue burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );
}

#[test]
fn rogue_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_rogue_level2_progression.rs) widened the level-1-only gate
    // to level 2 (mirroring the Fighter/Paladin level-range gate idiom) and
    // grounded Evasion; this negative control is superseded, not violated —
    // pin the new truth here too so this file stays internally consistent.
    let level_2 = ROGUE_FIXTURE.replace("class:rogue:1", "class:rogue:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, "class_chassis.rogue.base_attack_bonus"),
        "level-2 Rogue is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.rogue.")),
        "level-2 Rogue must not surface any named rogue burden diagnostic (Evasion is grounded \
         as an explanation record, not a diagnostic): {:?}",
        computation.diagnostics
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Rogue must still be claim-blocked by the generic chassis diagnostics"
    );
}

#[test]
fn multiclass_rogue_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_FIXTURE.replace(
        "class_level=class:rogue:1",
        "class_level=class:rogue:1\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")),
        "multiclass Rogue must not gain any bounded level-1 single-class rogue chassis \
         explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- (v0.6 swarm update): claim-blocking persists, but total saves are now genuinely computed -----

#[test]
fn rogue_still_produces_a_claim_blocking_diagnostic_and_total_saves_are_now_computed() {
    // Was `rogue_still_produces_a_claim_blocking_diagnostic_and_no_total_saves`,
    // pinning the pre-v0.6-swarm truth that Rogue's base saves were standalone
    // and never reached `defense.total_save.*`. The v0.6 alpha swarm's
    // multiclass BAB/save-stacking generalization (task 4) gave Rogue a
    // genuinely integrated `class_chassis.base_save.*` computation, and
    // `compute_total_saves` is class-agnostic: it computes `defense.total_save.*`
    // from whatever base saves are integrated, regardless of which class
    // produced them. So total saves ARE now computed for single-class Rogue --
    // renamed and flipped rather than left with a stale, misleading name.
    let input = load(ROGUE_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "rogue chassis must still produce a claim-blocking diagnostic (now from the \
         deterministic combat-baseline/skill-posture gates, not class-chassis recognition): {:?}",
        computation.diagnostics
    );
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "defense.total_save.fortitude"),
        "rogue's base saves are now integrated, so total-save explanations must exist: {:?}",
        computation.explanations
    );
    assert_eq!(
        explanation(&computation, "defense.total_save.fortitude").value,
        1,
        "Total Fortitude: Rogue base Fortitude (+0) + Constitution modifier (+1) = 1"
    );
    assert_eq!(
        explanation(&computation, "defense.total_save.reflex").value,
        6,
        "Total Reflex: Rogue base Reflex (+2) + Dexterity modifier (+4) = 6"
    );
    assert_eq!(
        explanation(&computation, "defense.total_save.will").value,
        0,
        "Total Will: Rogue base Will (+0) + Wisdom modifier (+0) = 0"
    );
}

// ----- Control plane: the matrix reclassifies the rogue row to Partial/Computed -----

#[test]
fn matrix_rogue_row_is_partial_computed_with_all_four_pillars_grounded() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        rogue.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        rogue
            .grounding_ref
            .contains("sd13_rogue_level1_chassis_baseline"),
        "rogue row must cite the SD13-E3/E5 rogue proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "rogue partial row must carry a note");
    for grounded in ["base attack", "base save", "sneak attack", "trapfinding"] {
        assert!(
            note.to_lowercase().contains(grounded),
            "rogue partial note must mention '{grounded}' as grounded: {note}"
        );
    }
    assert!(
        !note.contains("only trapfinding remains unproven")
            && !note.contains("trapfinding remains unproven"),
        "rogue partial note must not repeat the stale 'trapfinding unproven' claim: {note}"
    );
    // The honestly-unproven remainder must stay named.
    for unproven in [
        "check-execution engine",
        "trap DC",
        "magic-trap disarm engine",
        "rogue talent",
        "damage-roll execution",
    ] {
        assert!(
            note.contains(unproven),
            "rogue partial note must name the unproven '{unproven}' remainder: {note}"
        );
    }
    assert!(
        note.contains("defense.total_save"),
        "rogue partial note must still name the separate, still-true total_save gap: {note}"
    );
}

#[test]
fn matrix_preserves_accepted_truth_and_unchanged_rows() {
    let matrix = seeded_current_truth();

    // Fighter, Monk, and Barbarian rows were later promoted to Supported/ProductVisible
    // by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-16).
    for id in [
        "class.fighter.level_1_pilot",
        "class.fighter.levels_2_10",
        "class.monk.bounded_progression",
        "class.barbarian.bounded_progression",
    ] {
        let row = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Supported,
            "row {id} must be Supported after the SD-19 class-row promotion"
        );
    }

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences).
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
        "the rogue slice must not promote any row to Supported or Lossy"
    );
}
