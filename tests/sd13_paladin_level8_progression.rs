//! SD13-E5 Paladin level-8 progression grounding proof.
//!
//! Widens the accepted Paladin level-1..level-7 hybrid chassis baseline
//! (`tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`,
//! `tests/sd13_paladin_base_attack_and_saves.rs`,
//! `tests/sd13_paladin_level2_lay_on_hands_divine_grace.rs`,
//! `tests/sd13_paladin_level3_mercy.rs`,
//! `tests/sd13_paladin_level4_progression.rs`,
//! `tests/sd13_paladin_level5_progression.rs`,
//! `tests/sd13_paladin_level6_progression.rs`,
//! `tests/sd13_paladin_level7_progression.rs`,
//! `tests/sd13_paladin_partial_caster_effective_caster_level.rs`) to Paladin
//! level 8, mirroring the sibling-class level-range-gate idiom
//! (`supported_paladin_level` is generalized from `1..=7` to `1..=8` via
//! `MAX_SUPPORTED_PALADIN_LEVEL = 8`). Both PF1 CRB primary sources (d20pfsrd
//! and legacy.aonprd.com Paladin class table) were read directly before
//! writing any code or test:
//!
//! - level 8 base attack bonus is +8 (full BAB, genuinely risen from +7 at
//!   level 7 — the class table's own "+8/+3" iterative-attack notation is not
//!   modeled anywhere in this codebase, only the flat base value) and base
//!   saves are +6 Fortitude (good, `8 / 2 + 2 = 6`, genuinely risen from +5),
//!   +2 Reflex (poor, `8 / 3 = 2`, numerically unchanged from level 7, an
//!   integer-division coincidence), and +6 Will (good, `8 / 2 + 2 = 6`,
//!   genuinely risen from +5) — confirmed by the same formulas already
//!   grounded at levels 1-7, not re-derived.
//! - Smite Evil stays 3/day (the uses rise at 4th and every three levels
//!   thereafter — 7th, 10th — so the next rise lands at 10th, not 8th, a
//!   threshold stasis checked rather than assumed); its attack bonus stays
//!   the flat Charisma modifier (+2 on this fixture) and its damage bonus
//!   genuinely rises to 8 (equal to paladin level) via the same formulas.
//! - Lay on Hands genuinely rises on both axes: uses per day to 6
//!   (`8 / 2 + 2` Charisma modifier) and heal amount to 4 (d6 dice count,
//!   `8 / 2`), via the same pre-existing formulas.
//! - Divine Grace stays the flat Charisma-modifier save bonus (+2),
//!   level-independent.
//! - the partial-caster effective caster level genuinely rises to 5
//!   (`8 - 3`, per the PF1 CRB "caster level is equal to her paladin level
//!   – 3" rule), while the partial-caster spell burden itself stays
//!   claim-blocked.
//! - Channel Positive Energy's dice count stays 4 (the paladin channels as
//!   an effective cleric of her paladin level; a cleric's channel dice rise
//!   at odd levels — 4d6 spans cleric levels 7-8 — so the next rise lands at
//!   9th, a threshold stasis checked rather than assumed).
//! - Mercy stays granted once (3rd level; the next repeat grant lands at
//!   9th), so the mercy-choice and mercy-granted recognitions still fire
//!   unchanged for the same fixture selection.
//! - the PF1 Core Rulebook Paladin class table's level-8 "Special" column
//!   reads "Aura of resolve" (verified independently against both primary
//!   sources, checked rather than assumed away) — a genuinely NEW class
//!   feature at 8th level, and confirmed NOT flat/identity-shaped: immunity
//!   to charm spells and spell-like abilities plus a +4 morale bonus against
//!   charm effects for allies within 10 feet while the paladin is conscious
//!   — it needs a condition-immunity engine and an ally-aura/positional
//!   engine, neither of which exists in this codebase, so it is deliberately
//!   left named-but-unproven, exactly like Aura of Courage (3rd) and Divine
//!   Health (3rd) before it. A dedicated negative test pins that no
//!   aura-of-resolve record or diagnostic is fabricated.
//!
//! It deliberately does not touch the mercy-effect resolution, channel
//! execution, Divine Bond, or the partial-caster spell posture burden (all
//! stay named-but-unproven, unchanged from levels 1-7), and it does not
//! ground Paladin level 9+. It also preserves the accepted Paladin
//! level-1..level-7 truth (unchanged), the Fighter negative control, and the
//! multiclass negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const PALADIN_LEVEL7_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level7_sd13_deterministic_input.txt");

const PALADIN_LEVEL8_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level8_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const BASE_ATTACK_ID: &str = "class_chassis.paladin.base_attack_bonus";
const BASE_SAVE_FORTITUDE_ID: &str = "class_chassis.paladin.base_save.fortitude";
const BASE_SAVE_REFLEX_ID: &str = "class_chassis.paladin.base_save.reflex";
const BASE_SAVE_WILL_ID: &str = "class_chassis.paladin.base_save.will";
const SMITE_EVIL_USES_PER_DAY_ID: &str = "class_chassis.paladin.smite_evil_uses_per_day";
const SMITE_EVIL_ATTACK_BONUS_ID: &str = "class_chassis.paladin.smite_evil_attack_bonus";
const SMITE_EVIL_DAMAGE_BONUS_ID: &str = "class_chassis.paladin.smite_evil_damage_bonus";
const LAY_ON_HANDS_USES_PER_DAY_ID: &str = "class_chassis.paladin.lay_on_hands_uses_per_day";
const LAY_ON_HANDS_HEAL_AMOUNT_ID: &str = "class_chassis.paladin.lay_on_hands_heal_amount";
const DIVINE_GRACE_SAVE_BONUS_ID: &str = "class_chassis.paladin.divine_grace_save_bonus";
const EFFECTIVE_CASTER_LEVEL_ID: &str =
    "class_chassis.paladin.partial_caster.effective_caster_level";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";
const MERCY_GRANTED_ID: &str = "class_chassis.paladin.mercy_granted";
const MERCY_CHOICE_ID: &str = "class_chassis.paladin.mercy_choice";
const CHANNEL_POSITIVE_ENERGY_DICE_ID: &str =
    "class_chassis.paladin.channel_positive_energy_dice";

// ----- Base attack bonus at level 8 -----

#[test]
fn paladin_level8_base_attack_bonus_is_grounded_by_the_same_formula() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 8,
        "Paladin level 8 full-BAB progression must equal 8, genuinely risen from 7 at level \
         7: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 8 (good Fortitude/Will, poor Reflex) -----

#[test]
fn paladin_level8_base_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 6,
        "Paladin level 8 good Fortitude (8/2+2) must equal 6, genuinely risen from 5"
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(reflex.value, 2, "Paladin level 8 poor Reflex (8/3) must equal 2");

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 6,
        "Paladin level 8 good Will (8/2+2) must equal 6, genuinely risen from 5"
    );
}

// ----- Smite Evil at level 8: uses/attack stay, damage rises -----

#[test]
fn paladin_level8_smite_evil_uses_stay_three_and_damage_rises_to_eight() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 3,
        "Paladin level 8 Smite Evil must stay 3/day (rises at 4th and every three levels \
         thereafter — the next rise lands at 10th, not 8th): {}",
        uses_per_day.detail
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let attack_bonus = explanation(&computation, SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "Paladin level 8 Smite Evil attack bonus must stay the flat Charisma modifier (+3)"
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 8,
        "Paladin level 8 Smite Evil damage bonus (equal to paladin level) must equal 8, \
         genuinely risen from 7: {}",
        damage_bonus.detail
    );
}

// ----- Lay on Hands genuinely rises on both axes at level 8 -----

#[test]
fn paladin_level8_lay_on_hands_rises_on_both_axes() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 7,
        "Paladin level 8 Lay on Hands uses per day (8/2 + Charisma modifier 3) must equal 7, \
         genuinely risen from 6: {}",
        uses_per_day.detail
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 4,
        "Paladin level 8 Lay on Hands heal dice count (8/2 d6) must equal 4, genuinely risen \
         from 3: {}",
        heal_amount.detail
    );
}

// ----- Divine Grace stays the flat Charisma-modifier bonus at level 8 -----

#[test]
fn paladin_level8_divine_grace_stays_the_charisma_modifier() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "Paladin Divine Grace must stay the flat Charisma-modifier save bonus (+3) at level 8"
    );
}

// ----- Partial-caster effective caster level genuinely rises to 5 -----

#[test]
fn paladin_level8_effective_caster_level_rises_to_five() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 5,
        "Paladin level 8 effective caster level (8 - 3) must equal 5, genuinely risen from 4: \
         {}",
        caster_level.detail
    );

    // (v0.6 alpha swarm, risks item 8, 2026-07-25) `PARTIAL_CASTER_BLOCKER_ID`
    // is no longer unconditional: it's a real, conditional validation of
    // AcquisitionMode::Prepared selections. This fixture predates
    // spells_selected (zero prepared), so the posture is genuinely valid and
    // the blocker correctly does not fire -- the real "no spell slots are
    // fabricated" guarantee now comes from the daily-preparation record's own
    // count being honestly 0.
    match computation.diagnostics.iter().find(|d| d.id == PARTIAL_CASTER_BLOCKER_ID) {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            let daily_prep = explanation(&computation, "class_spell.paladin.daily_preparation");
            assert_eq!(
                daily_prep.value, 0,
                "no spells are fabricated at paladin level 8: {daily_prep:?}"
            );
        }
    }
}

// ----- Channel Positive Energy dice count stays 4 at level 8 -----

#[test]
fn paladin_level8_channel_positive_energy_dice_stay_four() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 4,
        "Paladin level 8 Channel Positive Energy (as an effective cleric of paladin level 8) \
         must stay 4d6 — a cleric's channel dice rise at odd levels, so the next rise lands \
         at 9th: {}",
        dice.detail
    );
}

// ----- Mercy stays granted once and its recognitions still fire at level 8 -----

#[test]
fn paladin_level8_still_recognizes_the_mercy_choice_and_grant() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy-granted recognition must carry no fabricated mechanical value at level 8"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "mercy-choice recognition must carry no fabricated mechanical value at level 8"
    );
    assert!(
        choice.detail.contains("shaken"),
        "mercy-choice recognition must still name the shaken selection at level 8: {}",
        choice.detail
    );
}

// ----- Aura of Resolve stays entirely named-but-unproven at level 8 -----

#[test]
fn paladin_level8_does_not_fabricate_aura_of_resolve() {
    let input = load(PALADIN_LEVEL8_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // SD18 (cycle-2026-07-13T2334) grounded a DIFFERENT, distinct paladin
    // feature that also happens to contain the substring "aura" in its name
    // -- Aura of Justice, an 11th-level feature -- as a bounded grant-only
    // identity record with a correct PF1 CRB level-gate absence below level
    // 11 (so it legitimately DOES emit an absence-placeholder record at
    // level 8, mirroring lay on hands / divine grace / mercy / channel
    // positive energy's own absence-gate idiom). This assertion is narrowed
    // from a blanket "aura" substring match to specifically "resolve" so it
    // keeps guarding against Aura of Resolve (still named-but-unproven,
    // unaffected by SD18's unrelated Aura of Justice grounding) without
    // colliding with the new, distinct, legitimately-grounded record.
    // SD-34 decisions.md section 18: widened BY CONSTRUCTION, not narrowed --
    // class_feature_grant_consumer now emits `class_feature.paladin.corpus_record.
    // aura_of_resolve`, a flat, citation-backed GRANT FACT (Paladin gains this
    // feature at level 8, joined to a real, non-fabricated corpus record). This is
    // NOT the mechanical magnitude this test guards against -- no condition-immunity
    // engine or ally-aura/positional engine exists here, and this module's own grant
    // record carries no such computation, only the level-gate fact. The exact id is
    // carved out below by NAME, not by prefix or substring, so this assertion keeps
    // catching any OTHER fabricated Aura of Resolve record (a mechanical value, a
    // different id shape) exactly as before.
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("resolve")
                && e.id != "class_feature.paladin.corpus_record.aura_of_resolve"),
        "level-8 Paladin must not fabricate any Aura of Resolve explanation record (Aura of \
         Resolve needs a condition-immunity engine and an ally-aura/positional engine, \
         neither of which exists in this codebase) beyond the flat, citation-backed grant \
         fact: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.to_lowercase().contains("resolve")),
        "level-8 Paladin must not fabricate any Aura of Resolve diagnostic either: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: the level-7 fixture is unaffected by this widening -----

#[test]
fn paladin_level7_truth_is_unchanged_by_this_slice() {
    let input = load(PALADIN_LEVEL7_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(base_attack.value, 7, "Paladin level 7 base attack bonus must stay 7");

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(heal_amount.value, 3, "Paladin level 7 Lay on Hands heal dice must stay 3");

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(caster_level.value, 4, "Paladin level 7 effective caster level must stay 4");
}

// ----- Level 9 was later widened into the supported tranche by a further slice -----

#[test]
fn paladin_level_9_was_later_widened_into_the_supported_tranche() {
    let level_9 = PALADIN_LEVEL8_FIXTURE.replace("class:paladin:8", "class:paladin:9");
    let input = load(&level_9);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "level-9 Paladin is now recognized by the later level-9 widening slice \
         (tests/sd13_paladin_level9_progression.rs carries its proof): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the paladin path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_paladin_level8_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "the Fighter chassis must not surface any paladin-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Paladin is not promoted -----

#[test]
fn multiclass_paladin_level8_is_not_promoted_by_this_slice() {
    let multiclass = PALADIN_LEVEL8_FIXTURE.replace(
        "class_level=class:paladin:8",
        "class_level=class:paladin:8\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "multiclass Paladin must not gain any bounded paladin chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-8 widening -----

#[test]
fn matrix_paladin_row_names_level_8_widening() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_paladin_level8_progression"),
        "paladin row must cite the live SD13-E5 level-8 proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.contains("level 8") || note.contains("level-8"),
        "paladin partial note must name the level-8 widening: {note}"
    );
}
