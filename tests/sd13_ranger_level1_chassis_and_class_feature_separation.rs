//! SD13-E3 Ranger level-1 chassis-and-class-feature separation proof.
//!
//! Proves the deeper Ranger-only decomposition that sits on top of the accepted
//! SD13-F6 hybrid baseline: the live rules-core surface emits two distinct
//! claim-blocking diagnostics for the still-missing Ranger non-spell
//! class-feature pillars (favored enemy, combat style), and grounds the third
//! named F6 pillar — Track — for real as a bounded flat Survival-check bonus
//! (`max(ranger level / 2, 1)`, i.e. `1` at the bounded level-1 baseline). The
//! support-state matrix row for `class.ranger.hybrid_chassis_and_spell_burden`
//! is promoted from `Blocked` to `Partial` to reflect that Track is now real,
//! grounded evidence, while favored enemy, combat style, and the later spell
//! burden remain named and unproven.
//!
//! It is intentionally not a Ranger class engine. It grounds no favored-enemy
//! target resolution or skill/damage math, no combat-style feat grant, no
//! animal companion, no favored-terrain breadth, no Ranger level 2+, no
//! multiclass, and no spell posture (spell slots, spell source, spells
//! known/prepared). It also preserves the accepted Fighter 1-3 truth, the
//! Rogue/Barbarian/Monk/Paladin postures, the shared F6 hybrid blockers (so the
//! F6 test continues to pass), the Sorcerer/Bard/Wizard/Cleric/Druid Blocked
//! postures, and the Human race/interaction seam.

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

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const PALADIN_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

// Exact per-pillar Ranger-only diagnostics this slice proves.
const RANGER_FAVORED_ENEMY_ID: &str = "class_feature.ranger.favored_enemy.unsupported";
const RANGER_COMBAT_STYLE_ID: &str = "class_feature.ranger.combat_style.unsupported";
const RANGER_TRACK_ID: &str = "class_chassis.ranger.track";

// F6 hybrid blockers are accepted truth and must still be claim-blocking for
// Ranger regression preservation. The F6 test asserts both of these ids.
const F6_HYBRID_RANGER_FEATURE_ID: &str = "class_feature.hybrid.ranger.unsupported";
const F6_HYBRID_RANGER_SPELL_ID: &str = "class_spell.hybrid.ranger.unsupported";

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

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Per-pillar Ranger chassis blockers must be present and claim-blocking -----

#[test]
fn ranger_level1_emits_separate_favored_enemy_and_combat_style_blockers() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [RANGER_FAVORED_ENEMY_ID, RANGER_COMBAT_STYLE_ID] {
        assert!(
            has_diagnostic(&computation, id),
            "ranger must surface separate claim-blocking diagnostic '{id}', got {:?}",
            computation.diagnostics
        );
        let diag = claim_blocking(&computation, id);
        assert!(
            !diag.message.is_empty(),
            "per-pillar ranger blocker '{id}' must carry a non-empty message"
        );
    }
}

#[test]
fn ranger_favored_enemy_blocker_names_skill_and_weapon_damage_burden() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let favored_enemy = claim_blocking(&computation, RANGER_FAVORED_ENEMY_ID);
    assert!(
        favored_enemy.message.contains("favored enemy")
            || favored_enemy.message.contains("favored-enemy"),
        "ranger favored-enemy blocker must name the favored-enemy burden: {}",
        favored_enemy.message
    );
    for token in [
        "Bluff",
        "Knowledge",
        "Perception",
        "Sense Motive",
        "Survival",
    ] {
        assert!(
            favored_enemy.message.contains(token),
            "ranger favored-enemy blocker must name the '{token}' skill burden: {}",
            favored_enemy.message
        );
    }
    assert!(
        favored_enemy.message.contains("weapon damage"),
        "ranger favored-enemy blocker must name the weapon-damage burden: {}",
        favored_enemy.message
    );
}

#[test]
fn ranger_combat_style_blocker_names_level1_choice_and_level2_feat_grant() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let combat_style = claim_blocking(&computation, RANGER_COMBAT_STYLE_ID);
    assert!(
        combat_style.message.contains("combat style"),
        "ranger combat-style blocker must name the combat-style burden: {}",
        combat_style.message
    );
    assert!(
        combat_style.message.contains("level-1") || combat_style.message.contains("level 1"),
        "ranger combat-style blocker must name the level-1 style choice: {}",
        combat_style.message
    );
    assert!(
        combat_style.message.contains("level-2") || combat_style.message.contains("level 2"),
        "ranger combat-style blocker must name the level-2 bonus-feat milestone: {}",
        combat_style.message
    );
}

// ----- Track is grounded for real -----

#[test]
fn ranger_track_is_grounded_with_value_one_at_level_one() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let track = explanation(&computation, RANGER_TRACK_ID);
    assert_eq!(
        track.value, 1,
        "ranger Track bonus at level 1 must be max(1/2, 1) = 1, got {}",
        track.value
    );
    assert!(
        track.detail.contains("Track"),
        "ranger Track explanation must name the Track class feature: {}",
        track.detail
    );
    assert!(
        track.detail.contains("Survival"),
        "ranger Track explanation must name the Survival-check bonus: {}",
        track.detail
    );
    assert!(
        track.detail.contains("follow tracks"),
        "ranger Track explanation must name following tracks: {}",
        track.detail
    );
    // Must be explicit that this grounds only the flat numeric bonus, not a
    // tracking-check execution engine.
    assert!(
        track
            .detail
            .contains("not a tracking-check execution engine")
            || track.detail.contains("no tracking-check execution engine"),
        "ranger Track explanation must disclaim a tracking-check execution engine: {}",
        track.detail
    );
}

// ----- The OLD combined F6 diagnostics still exist unmodified (negative regression check) -----

#[test]
fn ranger_f6_hybrid_blockers_remain_intact_under_separation() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let feature = claim_blocking(&computation, F6_HYBRID_RANGER_FEATURE_ID);
    for token in ["favored enemy", "combat style", "tracking"] {
        assert!(
            feature.message.contains(token),
            "F6 combined ranger feature blocker must still name '{token}': {}",
            feature.message
        );
    }

    assert!(
        has_diagnostic(&computation, F6_HYBRID_RANGER_SPELL_ID),
        "F6 hybrid spell blocker must remain claim-blocking"
    );

    // The F6 chassis recognition explanation must still be present so the F6
    // test does not lose its identity-proof surface.
    assert!(
        has_explanation(&computation, "class_chassis.hybrid_baseline.ranger"),
        "F6 chassis recognition explanation must remain"
    );
}

// ----- The Human race seam is preserved -----

#[test]
fn ranger_decomposition_preserves_human_race_seam() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "ranger decomposition must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "ranger decomposition must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
}

// ----- Negative controls: no leakage to sibling classes -----

#[test]
fn ranger_separated_blockers_do_not_emerge_for_paladin_or_fighter() {
    let paladin_input = load(PALADIN_FIXTURE);
    let paladin_computation = compute_pilot_base_chassis(&paladin_input);
    for id in [
        RANGER_FAVORED_ENEMY_ID,
        RANGER_COMBAT_STYLE_ID,
        RANGER_TRACK_ID,
    ] {
        assert!(
            !has_diagnostic(&paladin_computation, id) && !has_explanation(&paladin_computation, id),
            "paladin must not gain a Ranger-only per-pillar record '{id}'"
        );
    }

    let fighter_input = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter_input);
    for id in [
        RANGER_FAVORED_ENEMY_ID,
        RANGER_COMBAT_STYLE_ID,
        RANGER_TRACK_ID,
    ] {
        assert!(
            !has_diagnostic(&fighter_computation, id) && !has_explanation(&fighter_computation, id),
            "fighter must not gain a Ranger-only per-pillar record '{id}'"
        );
    }
    for id in [F6_HYBRID_RANGER_FEATURE_ID, F6_HYBRID_RANGER_SPELL_ID] {
        assert!(
            !has_diagnostic(&fighter_computation, id),
            "fighter must not gain a hybrid ranger blocker '{id}'"
        );
    }
}

#[test]
fn ranger_rogue_barbarian_monk_do_not_gain_ranger_pillar_records() {
    // Fighter fixture with the class swapped is used to synthesize each sibling
    // class input, mirroring the pattern used by the F6 baseline test.
    for class_id in ["class:rogue:1", "class:barbarian:1", "class:monk:1"] {
        let fixture = PALADIN_FIXTURE.replace("class:paladin:1", class_id);
        let input = load(&fixture);
        let computation = compute_pilot_base_chassis(&input);
        for id in [
            RANGER_FAVORED_ENEMY_ID,
            RANGER_COMBAT_STYLE_ID,
            RANGER_TRACK_ID,
        ] {
            assert!(
                !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
                "{class_id} must not gain a Ranger-only per-pillar record '{id}'"
            );
        }
    }
}

// ----- Level-2+ Ranger and multiclass are not promoted by this slice -----

#[test]
fn ranger_level2_is_not_promoted_by_this_slice() {
    let level_2 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        RANGER_FAVORED_ENEMY_ID,
        RANGER_COMBAT_STYLE_ID,
        RANGER_TRACK_ID,
    ] {
        assert!(
            !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
            "level-2 ranger must not gain the bounded level-1 per-pillar record '{id}'"
        );
    }
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 ranger must stay claim-blocked in this slice"
    );
}

#[test]
fn multiclass_ranger_is_not_promoted_by_this_slice() {
    // Synthesize a multiclass input by appending a second class_level line; the
    // fixture loader treats each `class_level=` line as a distinct class entry.
    let multiclass_fixture = format!("{RANGER_FIXTURE}\nclass_level=class:fighter:1\n");
    let input = load(&multiclass_fixture);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        RANGER_FAVORED_ENEMY_ID,
        RANGER_COMBAT_STYLE_ID,
        RANGER_TRACK_ID,
    ] {
        assert!(
            !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
            "multiclass ranger must not gain the bounded single-class per-pillar record '{id}'"
        );
    }
}

// ----- The integrated posture stays Blocked, never fakes Computed -----

#[test]
fn ranger_level1_still_yields_blocked_headless_receipt_and_view_model() {
    // Grounding Track alone does not unblock the whole character: favored
    // enemy and combat style are still claim-blocking diagnostics, so the
    // per-character receipt status stays Blocked even though the row's
    // SupportState moves to Partial at the matrix (documentary) level.
    let input = load(RANGER_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Blocked,
        "separated per-pillar blockers must keep the integrated ranger posture Blocked"
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked ranger posture must not emit a computed snapshot"
    );
}

// ----- Control plane: matrix row is promoted to Partial with the right note -----

#[test]
fn matrix_ranger_row_is_promoted_to_partial_and_names_remaining_pillars() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");

    // The one intentional promotion in this batch.
    assert_eq!(ranger.support_state, SupportState::Partial);
    assert_eq!(ranger.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger
            .grounding_ref
            .contains("sd13_ranger_level1_chassis_and_class_feature_separation"),
        "ranger row must cite this slice as its proof surface: {}",
        ranger.grounding_ref
    );
    assert!(
        ranger
            .grounding_ref
            .contains("sd13_hybrid_level1_chassis_baseline"),
        "ranger row must continue to cite the SD13-F6 hybrid proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "ranger partial row must carry a note");
    for token in ["favored enemy", "combat style"] {
        assert!(
            note.contains(token),
            "ranger partial note must name the still-unproven '{token}' pillar: {note}"
        );
    }
    // The note must not claim Track is unproven — it is grounded.
    assert!(
        note.contains("Track") && note.contains("grounds") || note.contains("grounded"),
        "ranger partial note must name Track as grounded, not unproven: {note}"
    );
    // The spell burden is still named as deferred to SD13-E4.
    assert!(
        note.contains("SD13-E4"),
        "ranger partial note must still defer the spell burden to SD13-E4: {note}"
    );
}

// ----- Sibling rows are preserved, no other row is silently promoted -----

#[test]
fn matrix_preserves_sibling_rows_after_ranger_promotion() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    // These rows stay Blocked/Computed.
    for id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.bard.progression_and_spell_burden",
        "class.cleric.progression_and_spell_burden",
        "class.druid.progression_and_spell_burden",
        "class.sorcerer.progression_and_spell_burden",
        "class.wizard.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "row {id} must stay Blocked after the ranger-decomposition slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }

    // These rows stay Partial/Computed.
    for id in [
        "class.rogue.bounded_progression",
        "class.barbarian.bounded_progression",
        "class.monk.bounded_progression",
        "class.fighter.level_1_pilot",
        "class.fighter.levels_2_10",
    ] {
        let row = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must stay Partial after the ranger-decomposition slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }

    // No row is silently promoted to Supported or Lossy by this slice.
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the ranger-decomposition slice must not promote any row to Supported or Lossy"
    );
}
