//! SD13-E3-F6 Paladin and Ranger level-1 hybrid chassis baseline proof.
//!
//! Proves the first truthful SD13-F6 slice: the live rules-core surface ingests
//! deterministic Human `class:paladin:1` and `class:ranger:1` inputs, leaves direct
//! computed evidence that acknowledges each hybrid level-1 chassis identity rather
//! than treating it as an undocumented packet placeholder, and yet stays explicitly
//! claim-blocked on the still-missing non-spell class-feature burden and the later
//! hybrid spell burden. It also pins the matrix reclassification of both hybrid rows
//! from `Unverified` / `Observed` to `Blocked` / `Computed`.
//!
//! It is intentionally not a hybrid class engine. It grounds no Paladin/Ranger level
//! 2+, no smite / lay-on-hands / divine-grace / mercy execution, no favored-enemy /
//! combat-style / tracking execution, and no spell-slot / known-or-prepared posture.
//! It also preserves the accepted Fighter 1-3 truth, the Rogue blocked negative
//! control, and the Human race/interaction truth.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, HeadlessReceiptStatus,
    PilotBaseChassisComputation, build_pilot_headless_receipt, compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const PALADIN_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");
const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

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

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Direct runtime evidence: the hybrid chassis identity is acknowledged -----

#[test]
fn paladin_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Paladin chassis identity is recognized on
    // the compute path, not silently dropped as an undocumented packet placeholder.
    let chassis = explanation(&computation, "class_chassis.hybrid_baseline.paladin");
    assert!(
        chassis.detail.contains("class:paladin") && chassis.detail.contains("level 1"),
        "paladin chassis recognition must name the class:paladin:1 identity: {}",
        chassis.detail
    );
    // It is recognition only; it must not fabricate a Fighter-style computed chassis.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "hybrid baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "hybrid baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (CHA 14 -> +2).
    assert_eq!(computation.ability_modifiers.charisma, 2);
}

#[test]
fn ranger_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let chassis = explanation(&computation, "class_chassis.hybrid_baseline.ranger");
    assert!(
        chassis.detail.contains("class:ranger") && chassis.detail.contains("level 1"),
        "ranger chassis recognition must name the class:ranger:1 identity: {}",
        chassis.detail
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "hybrid baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "hybrid baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (STR 16 -> +3).
    assert_eq!(computation.ability_modifiers.strength, 3);
}

// ----- Still blocked: honest, class-specific burden diagnostics -----

#[test]
fn paladin_level1_stays_blocked_naming_class_feature_and_spell_burden() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The non-spell class-feature burden must be named explicitly, not hidden behind a
    // generic "unsupported hybrid" label.
    let feature = claim_blocking(&computation, "class_feature.hybrid.paladin.unsupported");
    for token in ["smite", "lay on hands", "divine grace", "mercy"] {
        assert!(
            feature.message.contains(token),
            "paladin feature blocker must name the '{token}' burden: {}",
            feature.message
        );
    }

    // The later spell burden must be named explicitly and stay claim-blocking.
    let spell = claim_blocking(&computation, "class_spell.hybrid.paladin.unsupported");
    assert!(
        spell.message.contains("spell"),
        "paladin spell blocker must name the later spell burden: {}",
        spell.message
    );

    // The integrated posture is blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked hybrid baseline must not emit a computed snapshot"
    );
}

#[test]
fn ranger_level1_stays_blocked_naming_class_feature_and_spell_burden() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let feature = claim_blocking(&computation, "class_feature.hybrid.ranger.unsupported");
    for token in ["favored enemy", "combat style", "tracking"] {
        assert!(
            feature.message.contains(token),
            "ranger feature blocker must name the '{token}' burden: {}",
            feature.message
        );
    }

    let spell = claim_blocking(&computation, "class_spell.hybrid.ranger.unsupported");
    assert!(
        spell.message.contains("spell"),
        "ranger spell blocker must name the later spell burden: {}",
        spell.message
    );

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(view_model.snapshot.is_none());
}

// ----- The accepted Human race seam is preserved on the hybrid path -----

#[test]
fn hybrid_baseline_preserves_human_race_seam() {
    for fixture in [PALADIN_FIXTURE, RANGER_FIXTURE] {
        let input = load(fixture);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            has_explanation(&computation, "race.human.ability_bonus_target"),
            "hybrid baseline must preserve the Human ability-bonus race seam: {:?}",
            computation.explanations
        );
        assert!(
            has_explanation(&computation, "race.human.bonus_feat_grant"),
            "hybrid baseline must preserve the Human bonus-feat race seam: {:?}",
            computation.explanations
        );
        // The bounded Human race-semantics note stays present and non-claim-blocking.
        assert!(
            computation
                .diagnostics
                .iter()
                .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
            "hybrid baseline must keep the bounded, non-blocking Human race note: {:?}",
            computation.diagnostics
        );
    }
}

// ----- Negative control: the hybrid path must not leak onto other classes -----

#[test]
fn fighter_and_rogue_do_not_gain_hybrid_recognition() {
    // A supported Fighter must not gain a hybrid-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(
            &fighter_computation,
            "class_chassis.hybrid_baseline.paladin"
        ) && !has_explanation(&fighter_computation, "class_chassis.hybrid_baseline.ranger"),
        "the Fighter chassis must not surface a hybrid-baseline recognition record"
    );

    // A Rogue must stay a plain blocked negative control, never a hybrid baseline.
    let rogue_fixture = PALADIN_FIXTURE.replace("class:paladin:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "Rogue chassis must remain claim-blocked"
    );
    assert!(
        !has_explanation(&rogue_computation, "class_chassis.hybrid_baseline.paladin")
            && !has_explanation(&rogue_computation, "class_chassis.hybrid_baseline.ranger"),
        "Rogue must not surface any hybrid-baseline recognition record"
    );
    assert!(
        !rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.hybrid.")
                || d.id.starts_with("class_spell.hybrid.")),
        "Rogue must not surface hybrid class-feature/spell burden diagnostics: {:?}",
        rogue_computation.diagnostics
    );
}

#[test]
fn paladin_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Paladin must not gain the level-1
    // hybrid recognition record and stays blocked.
    let level_2 = PALADIN_FIXTURE.replace("class:paladin:1", "class:paladin:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.hybrid_baseline.paladin"),
        "level-2 Paladin must not gain the bounded level-1 hybrid recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix carries both hybrid rows as Computed evidence -----

#[test]
fn matrix_paladin_row_is_partial_computed_and_names_both_burdens() {
    // F6 moved this row off the pure Unverified/Observed placeholder to
    // Blocked/Computed; the SD13-E4 slice grounded Smite Evil and the SD13-E5
    // level-gate slice grounded lay on hands / divine grace / mercy as correct
    // level-1 absences and promoted the row to Partial. The F6 chassis and
    // blocker truth this file otherwise pins is unchanged.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must exist");

    assert_eq!(paladin.support_state, SupportState::Partial);
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_hybrid_level1_chassis_baseline"),
        "paladin row must cite the SD13-F6 hybrid proof surface: {}",
        paladin.grounding_ref
    );
    // The note must name both the non-spell class-feature burden and the later spell burden.
    let note = paladin.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "paladin blocked row must carry a note");
    for token in ["smite", "lay on hands", "divine grace", "mercy", "spell"] {
        assert!(
            note.contains(token),
            "paladin blocked note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_ranger_row_is_partial_computed_and_names_remaining_burdens() {
    // The SD13-E3 Ranger decomposition slice
    // (sd13_ranger_level1_chassis_and_class_feature_separation.rs) later
    // grounded Track for real and promoted this row from Blocked to Partial.
    // The F6 chassis-recognition and hybrid-blocker truth this file otherwise
    // pins is unchanged; only the row's matrix-level posture moved.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");

    assert_eq!(ranger.support_state, SupportState::Partial);
    assert_eq!(ranger.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger
            .grounding_ref
            .contains("sd13_hybrid_level1_chassis_baseline"),
        "ranger row must still cite the SD13-F6 hybrid proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "ranger partial row must carry a note");
    // "SD13-E4" was the original deferral token; a further SD13-E5 slice grounded
    // the ranger partial-caster identity pair, and the note now names the remaining
    // spell burden as unproven BEYOND that pair instead of deferring it wholesale.
    for token in ["favored enemy", "combat style", "partial_caster.spell_level_access"] {
        assert!(
            note.contains(token),
            "ranger partial note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_preserves_fighter_and_rogue_accepted_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    // Fighter rows were later promoted to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-16).
    for id in ["class.fighter.level_1_pilot", "class.fighter.levels_2_10"] {
        let fighter = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            fighter.support_state,
            SupportState::Supported,
            "row {id} must be Supported after the SD-19 class-row promotion"
        );
        assert_eq!(fighter.evidence_tier, EvidenceTier::ProductVisible);
    }

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
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "the hybrid slice must not promote any row to Supported or Lossy"
    );
}
