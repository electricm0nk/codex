//! SD13-E3 Ranger level-1 chassis-and-class-feature separation proof, extended
//! by the SD13-E5 Favored Enemy flat-surface grounding slice and the later
//! SD13-E5 Combat Style level-gate correction slice.
//!
//! Proves the deeper Ranger-only decomposition that sits on top of the accepted
//! SD13-F6 hybrid baseline: the live rules-core surface grounds Track for real
//! as a bounded flat Survival-check bonus (`max(ranger level / 2, 1)`, i.e. `1`
//! at the bounded level-1 baseline), grounds the Favored Enemy flat surface for
//! real (SD13-E5): recognition of the chosen favored-enemy type from the
//! fixture's `choice:ranger_favored_enemy` selection, the flat +2 bonus on
//! Bluff, Knowledge, Perception, Sense Motive, and Survival checks against the
//! favored enemy, and the flat +2 bonus on weapon attack and damage rolls
//! against the favored enemy (PF1 CRB — attack rolls are included, unlike D&D
//! 3.5), and grounds the combat-style pillar as a correct level-1 ABSENCE
//! (value 0, a later SD13-E5 slice): PF1 Core Rulebook grants the
//! archery-vs-two-weapon-combat style choice and its first bonus feat TOGETHER
//! at 2nd level, not split across a level-1 choice and a level-2 grant as an
//! earlier version of this record (the now-retired
//! `class_feature.ranger.combat_style.unsupported` claim-blocking diagnostic)
//! incorrectly claimed. The support-state matrix row for
//! `class.ranger.hybrid_chassis_and_spell_burden` stays `Partial`: the
//! favored-enemy conditional-application engine, the level-2 combat-style feat
//! grant itself, and the later spell burden remain named and unproven.
//!
//! It is intentionally not a Ranger class engine. It grounds no favored-enemy
//! target-type matching or conditional-application engine (the flat magnitudes
//! are grounded, never applied to a specific target), no combat-style feat
//! grant, no animal companion, no favored-terrain breadth, no Ranger level 2+,
//! no multiclass, and no spell posture (spell slots, spell source, spells
//! known/prepared). It also preserves the accepted Fighter 1-3 truth, the
//! Rogue/Barbarian/Monk/Paladin postures, the shared F6 hybrid blockers (so the
//! F6 test continues to pass), the Sorcerer/Bard/Wizard/Cleric/Druid postures,
//! and the Human race/interaction seam.

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

// Exact per-pillar Ranger-only records this slice proves.
//
// The favored-enemy claim-blocking diagnostic is RETIRED by the SD13-E5
// Favored Enemy flat-surface grounding slice; it must no longer be emitted.
const RANGER_FAVORED_ENEMY_RETIRED_BLOCKER_ID: &str =
    "class_feature.ranger.favored_enemy.unsupported";
// The combat-style claim-blocking diagnostic is RETIRED by the later SD13-E5
// Combat Style level-gate correction slice; it must no longer be emitted.
const RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID: &str = "class_feature.ranger.combat_style.unsupported";
const RANGER_TRACK_ID: &str = "class_chassis.ranger.track";

// Grounded Favored Enemy flat-surface records (SD13-E5).
const RANGER_FAVORED_ENEMY_CHOICE_ID: &str = "class_chassis.ranger.favored_enemy_choice";
const RANGER_FAVORED_ENEMY_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_skill_bonus";
const RANGER_FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_attack_damage_bonus";

// Grounded combat-style level-gate absence record (later SD13-E5 slice): PF1
// grants the style choice and its first bonus feat together at 2nd level, so
// this bounded level-1 baseline grounds a correct value-0 ABSENCE.
const RANGER_COMBAT_STYLE_LEVEL_GATE_ID: &str = "class_chassis.ranger.level_gate.combat_style";

// Every Ranger-only per-pillar record id — used by the negative controls to
// prove none of them leak onto sibling classes, level 2+, or multiclass.
const RANGER_PER_PILLAR_RECORD_IDS: [&str; 7] = [
    RANGER_FAVORED_ENEMY_RETIRED_BLOCKER_ID,
    RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID,
    RANGER_TRACK_ID,
    RANGER_FAVORED_ENEMY_CHOICE_ID,
    RANGER_FAVORED_ENEMY_SKILL_BONUS_ID,
    RANGER_FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID,
    RANGER_COMBAT_STYLE_LEVEL_GATE_ID,
];

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

// ----- The remaining per-pillar Ranger chassis blocker must be present and claim-blocking -----

#[test]
fn ranger_level1_grounds_combat_style_level_gate_and_no_retired_blockers() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let combat_style = explanation(&computation, RANGER_COMBAT_STYLE_LEVEL_GATE_ID);
    assert_eq!(
        combat_style.value, 0,
        "combat-style level-gate record is a correct level-1 absence and must carry no \
         fabricated mechanical value, got {}",
        combat_style.value
    );

    // Both the favored-enemy AND the combat-style claim-blocking diagnostics are
    // retired: the flat favored-enemy surface and the combat-style level-gate
    // absence are both grounded for real, so neither blocker must be emitted.
    for id in [
        RANGER_FAVORED_ENEMY_RETIRED_BLOCKER_ID,
        RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID,
    ] {
        assert!(
            !has_diagnostic(&computation, id),
            "the retired blocker '{id}' must no longer be emitted, got {:?}",
            computation.diagnostics
        );
    }
}

// ----- The Favored Enemy flat surface is grounded for real (SD13-E5) -----

#[test]
fn ranger_favored_enemy_choice_is_recognized_from_chosen_input() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, RANGER_FAVORED_ENEMY_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "favored-enemy choice recognition is a recognition record only and must carry no \
         fabricated mechanical value, got {}",
        choice.value
    );
    assert!(
        choice.detail.contains("favored enemy") || choice.detail.contains("favored-enemy"),
        "favored-enemy choice recognition must name the favored-enemy feature: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("enemy:humanoid_orc"),
        "favored-enemy choice recognition must name the chosen enemy type from the fixture's \
         choice:ranger_favored_enemy selection: {}",
        choice.detail
    );
}

#[test]
fn ranger_favored_enemy_skill_bonus_is_grounded_at_plus_two() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill = explanation(&computation, RANGER_FAVORED_ENEMY_SKILL_BONUS_ID);
    assert_eq!(
        skill.value, 2,
        "favored-enemy skill bonus at level 1 must be the flat +2 (PF1 CRB), got {}",
        skill.value
    );
    for token in [
        "Bluff",
        "Knowledge",
        "Perception",
        "Sense Motive",
        "Survival",
    ] {
        assert!(
            skill.detail.contains(token),
            "favored-enemy skill bonus must name the '{token}' skill: {}",
            skill.detail
        );
    }
    // Must be explicit that only the flat magnitude is grounded, not a
    // target-type matching / conditional-application engine.
    assert!(
        skill.detail.contains("conditional-application"),
        "favored-enemy skill bonus must disclaim the conditional-application engine: {}",
        skill.detail
    );
}

#[test]
fn ranger_favored_enemy_attack_damage_bonus_is_grounded_at_plus_two() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_damage = explanation(&computation, RANGER_FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_damage.value, 2,
        "favored-enemy attack/damage bonus at level 1 must be the flat +2 (PF1 CRB), got {}",
        attack_damage.value
    );
    // PF1 grants the bonus on weapon ATTACK rolls as well as damage rolls
    // (unlike D&D 3.5, which granted damage only) — both must be named.
    assert!(
        attack_damage.detail.contains("attack") && attack_damage.detail.contains("damage"),
        "favored-enemy attack/damage bonus must name both weapon attack and damage rolls: {}",
        attack_damage.detail
    );
    assert!(
        attack_damage.detail.contains("conditional-application"),
        "favored-enemy attack/damage bonus must disclaim the conditional-application engine: {}",
        attack_damage.detail
    );
}

#[test]
fn ranger_without_favored_enemy_choice_grounds_magnitudes_but_not_recognition() {
    // The desktop compose path builds ranger inputs without a
    // choice:ranger_favored_enemy selection. The flat level-1 magnitudes are
    // properties of the class feature and stay grounded; the choice-recognition
    // record derives strictly from chosen input and must not be fabricated. The
    // retired blocker must not reappear either way.
    let without_choice =
        RANGER_FIXTURE.replace("choice=choice:ranger_favored_enemy:enemy:humanoid_orc", "");
    let input = load(&without_choice);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, RANGER_FAVORED_ENEMY_CHOICE_ID),
        "without a chosen favored-enemy type there is nothing to recognize; the recognition \
         record must not be fabricated"
    );
    for id in [
        RANGER_FAVORED_ENEMY_SKILL_BONUS_ID,
        RANGER_FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID,
    ] {
        assert!(
            has_explanation(&computation, id),
            "flat favored-enemy magnitude '{id}' must stay grounded without the choice selection"
        );
    }
    assert!(
        !has_diagnostic(&computation, RANGER_FAVORED_ENEMY_RETIRED_BLOCKER_ID),
        "the retired favored-enemy blocker must not reappear when the choice is absent"
    );
}

#[test]
fn ranger_combat_style_level_gate_names_level2_milestone_only() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let combat_style = explanation(&computation, RANGER_COMBAT_STYLE_LEVEL_GATE_ID);
    assert!(
        combat_style.detail.contains("combat style") || combat_style.detail.contains("Combat Style"),
        "ranger combat-style level-gate record must name the combat-style feature: {}",
        combat_style.detail
    );
    // PF1 Core Rulebook grants the style choice AND its first bonus feat
    // together at 2nd level -- the record must name the 2nd-level milestone.
    assert!(
        combat_style.detail.contains("2nd level"),
        "ranger combat-style level-gate record must name the 2nd-level PF1 CRB milestone: {}",
        combat_style.detail
    );
    // Corrected framing: must not resurrect the retired claim that the style
    // choice is itself a level-1 decision separate from the level-2 feat grant.
    assert!(
        !combat_style
            .detail
            .contains("is a level-1 decision, but the bonus feat"),
        "ranger combat-style level-gate record must not resurrect the retired mistaken framing: {}",
        combat_style.detail
    );
    assert!(
        combat_style.detail.contains("correctly absent"),
        "ranger combat-style level-gate record must state the correct level-1 ABSENCE: {}",
        combat_style.detail
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
    for id in RANGER_PER_PILLAR_RECORD_IDS {
        assert!(
            !has_diagnostic(&paladin_computation, id) && !has_explanation(&paladin_computation, id),
            "paladin must not gain a Ranger-only per-pillar record '{id}'"
        );
    }

    let fighter_input = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter_input);
    for id in RANGER_PER_PILLAR_RECORD_IDS {
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
        for id in RANGER_PER_PILLAR_RECORD_IDS {
            assert!(
                !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
                "{class_id} must not gain a Ranger-only per-pillar record '{id}'"
            );
        }
    }
}

// ----- Level-2 Ranger was later widened into the supported tranche, level-3+ and multiclass are not -----

#[test]
fn ranger_level2_was_later_widened_into_the_supported_tranche() {
    // At the time this file was written, Ranger level 2+ was entirely out of scope
    // (the level-1-only gate `is_single_class_ranger_level1` did not recognize it). A
    // later SD13-E5 slice (`tests/sd13_ranger_level2_progression.rs`) widened the gate
    // to a level-range gate (`supported_ranger_level`, 1..=2) and extended Track and
    // the Favored Enemy flat surface to level 2 via the same formulas (this ad-hoc
    // fixture, built by a simple level-number replace, still carries the fixture's
    // original `choice:ranger_favored_enemy` selection, so those records now ground).
    // The retired F6 blockers stay retired either way. The combat-style level-gate
    // ABSENCE record is level-1-ONLY and is correctly retired at level 2 too -- this
    // ad-hoc fixture carries no `choice:ranger_combat_style` selection, so there is
    // nothing for the widened seam to recognize in its place (mirroring the Favored
    // Enemy choice-absence idiom); see `tests/sd13_ranger_level2_progression.rs` for
    // the dedicated fixture that does carry a combat-style selection and grounds the
    // choice-recognition records.
    let level_2 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        RANGER_FAVORED_ENEMY_RETIRED_BLOCKER_ID,
        RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID,
        RANGER_COMBAT_STYLE_LEVEL_GATE_ID,
    ] {
        assert!(
            !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
            "level-2 ranger must not gain the retired/level-1-only record '{id}': {:?} / {:?}",
            computation.diagnostics,
            computation.explanations
        );
    }
    for id in [
        RANGER_TRACK_ID,
        RANGER_FAVORED_ENEMY_CHOICE_ID,
        RANGER_FAVORED_ENEMY_SKILL_BONUS_ID,
        RANGER_FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID,
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-2 ranger must now carry the widened per-pillar record '{id}': {:?}",
            computation.explanations
        );
    }
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 ranger must stay claim-blocked overall (spell burden, favored-enemy \
         conditional-application engine, and combat-style bonus-feat mechanics all remain \
         unproven)"
    );
}

#[test]
fn ranger_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file was written, Ranger level 3+ was entirely out of scope
    // (the level-range gate `supported_ranger_level` only recognized 1..=2). A later
    // SD13-E5 slice (`tests/sd13_ranger_level3_progression.rs`) widened the gate to
    // 1..=3 and extended Track and the Favored Enemy flat surface to level 3 via the
    // same formulas (this ad-hoc fixture, built by a simple level-number replace,
    // still carries the fixture's original `choice:ranger_favored_enemy` selection,
    // so those records now ground). The retired F6 blockers stay retired either way.
    // The combat-style level-gate ABSENCE record is level-1-ONLY and stays correctly
    // retired at level 3 too -- this ad-hoc fixture carries no
    // `choice:ranger_combat_style` selection, so there is nothing for the widened
    // seam to recognize in its place. See `tests/sd13_ranger_level3_progression.rs`
    // for the dedicated fixture that does carry a combat-style selection and
    // Endurance's own grounding.
    let level_3 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        RANGER_FAVORED_ENEMY_RETIRED_BLOCKER_ID,
        RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID,
        RANGER_COMBAT_STYLE_LEVEL_GATE_ID,
    ] {
        assert!(
            !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
            "level-3 ranger must not gain the retired/level-1-only record '{id}': {:?} / {:?}",
            computation.diagnostics,
            computation.explanations
        );
    }
    for id in [
        RANGER_TRACK_ID,
        RANGER_FAVORED_ENEMY_CHOICE_ID,
        RANGER_FAVORED_ENEMY_SKILL_BONUS_ID,
        RANGER_FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID,
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-3 ranger must now carry the widened per-pillar record '{id}': {:?}",
            computation.explanations
        );
    }
    assert!(
        has_explanation(&computation, "class_feature.ranger.endurance"),
        "level-3 ranger must now carry the newly-grounded Endurance record: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-3 ranger must stay claim-blocked overall (spell burden, favored-enemy \
         conditional-application engine, and combat-style bonus-feat mechanics all remain \
         unproven)"
    );
}

#[test]
fn ranger_level4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_ranger_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk level-range
    // gate idiom) and grounded Hunter's Bond; this negative control is
    // superseded, not violated — pin the new truth here too so this file stays
    // internally consistent. The retired blockers and the level-1-only
    // combat-style level-gate marker correctly stay absent at level 4 (the
    // combat-style pillar is recognized differently once the 2nd-level gate is
    // reached); the active per-pillar records stay grounded.
    let level_4 = RANGER_FIXTURE.replace("class:ranger:1", "class:ranger:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        RANGER_FAVORED_ENEMY_RETIRED_BLOCKER_ID,
        RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID,
        RANGER_COMBAT_STYLE_LEVEL_GATE_ID,
    ] {
        assert!(
            !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
            "level-4 ranger must still not carry the retired/level-1-only record '{id}'"
        );
    }
    for id in [
        RANGER_TRACK_ID,
        RANGER_FAVORED_ENEMY_CHOICE_ID,
        RANGER_FAVORED_ENEMY_SKILL_BONUS_ID,
        RANGER_FAVORED_ENEMY_ATTACK_DAMAGE_BONUS_ID,
    ] {
        assert!(
            has_explanation(&computation, id),
            "level-4 ranger is supported since the SD13-E5 level-4 slice and must gain the \
             per-pillar record '{id}'"
        );
    }
    assert!(
        has_explanation(&computation, "class_feature.ranger.endurance"),
        "level-4 ranger must keep the bounded level-3 Endurance record grounded"
    );
}

#[test]
fn multiclass_ranger_is_not_promoted_by_this_slice() {
    // Synthesize a multiclass input by appending a second class_level line; the
    // fixture loader treats each `class_level=` line as a distinct class entry.
    let multiclass_fixture = format!("{RANGER_FIXTURE}\nclass_level=class:fighter:1\n");
    let input = load(&multiclass_fixture);
    let computation = compute_pilot_base_chassis(&input);

    for id in RANGER_PER_PILLAR_RECORD_IDS {
        assert!(
            !has_diagnostic(&computation, id) && !has_explanation(&computation, id),
            "multiclass ranger must not gain the bounded single-class per-pillar record '{id}'"
        );
    }
}

// ----- The integrated posture stays Blocked, never fakes Computed -----

#[test]
fn ranger_level1_still_yields_blocked_headless_receipt_and_view_model() {
    // Grounding Track, the Favored Enemy flat surface, and the combat-style
    // level-gate absence does not unblock the whole character: the shared F6
    // hybrid burdens are still claim-blocking diagnostics, so the
    // per-character receipt status stays Blocked even though the row's
    // SupportState is Partial at the matrix (documentary) level.
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
    // Combat style is named; the level-1 surface is a grounded correct absence,
    // and the level-2 bonus-feat grant remains the unproven remainder.
    assert!(
        note.contains("combat style"),
        "ranger partial note must name the 'combat style' pillar: {note}"
    );
    assert!(
        note.contains("2nd level") || note.contains("level-2"),
        "ranger partial note must name the level-2 combat-style milestone: {note}"
    );
    assert!(
        !note.contains("the level-1 style choice and its level-2 bonus-feat grant"),
        "ranger partial note must not resurrect the retired mistaken framing: {note}"
    );
    // The favored-enemy FLAT SURFACE is grounded; the conditional-application
    // engine (target-type matching) stays named and unproven.
    assert!(
        note.contains("favored enemy") || note.contains("favored-enemy"),
        "ranger partial note must name the favored-enemy pillar: {note}"
    );
    assert!(
        note.contains("conditional-application"),
        "ranger partial note must name the still-unproven favored-enemy conditional-application \
         engine: {note}"
    );
    // The note must not claim Track is unproven — it is grounded.
    assert!(
        note.contains("Track") && (note.contains("grounds") || note.contains("grounded")),
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

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.paladin.hybrid_chassis_and_spell_burden must exist"));
    assert_eq!(
        paladin.support_state,
        SupportState::Partial,
        "paladin row must keep its later-accepted Partial posture after the ranger-decomposition slice"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);

    // Bard, Cleric, Druid, and Sorcerer were later promoted to Partial/Computed by
    // their own SD13-E4 decomposition slices (Bardic Knowledge, Channel Energy,
    // Wild Empathy, Eschew Materials).
    for id in [
        "class.bard.progression_and_spell_burden",
        "class.cleric.progression_and_spell_burden",
        "class.druid.progression_and_spell_burden",
        "class.sorcerer.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {id} must be Partial after its own SD13-E4 decomposition slice"
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

    // Wizard was later promoted to Partial/Computed by its own SD13-E4 Scribe
    // Scroll decomposition slice.
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Partial,
        "wizard row must keep its later-accepted Partial posture after the ranger-decomposition slice"
    );
    assert_eq!(wizard.evidence_tier, EvidenceTier::Computed);

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
