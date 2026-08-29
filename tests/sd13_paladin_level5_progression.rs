//! SD13-E5 Paladin level-5 progression grounding proof.
//!
//! Widens the accepted Paladin level-1/level-2/level-3/level-4
//! chassis-and-spell-burden separation
//! (`tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`,
//! `tests/sd13_paladin_level2_lay_on_hands_divine_grace.rs`,
//! `tests/sd13_paladin_base_attack_and_saves.rs`,
//! `tests/sd13_paladin_level3_mercy.rs`,
//! `tests/sd13_paladin_level4_progression.rs`) to paladin level 5, mirroring
//! the Fighter/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer/Wizard/Ranger
//! level-range-gate idiom (`supported_paladin_level` is generalized from
//! `1..=4` to `1..=5` via `MAX_SUPPORTED_PALADIN_LEVEL = 5`). It proves, each
//! formula independently re-verified against the PF1 Core Rulebook Paladin
//! class table (d20pfsrd and legacy.aonprd.com) before writing any code:
//!
//! - base attack bonus at level 5 is grounded by the same full-BAB formula
//!   already grounded at levels 1-4: `5`.
//! - base saves at level 5 are grounded by the same good-Fortitude/good-Will/
//!   poor-Reflex formulas already grounded at levels 1-4, extended to level
//!   5: Fortitude/Will `5 / 2 + 2 = 4`, Reflex `5 / 3 = 1` -- both
//!   numerically unchanged from level 4 (integer-division coincidences:
//!   `5/2` and `4/2` both floor to `2`; `5/3` and `4/3` both floor to `1`),
//!   not a sign either formula stopped scaling.
//! - Smite Evil's uses-per-day formula `1 + (paladin level - 1) / 3`
//!   correctly stays 2/day at level 5 (`1 + 4/3 = 2`), unchanged from level
//!   4; the next increase does not land until level 7, out of scope.
//!   Attack bonus and damage bonus keep scaling via the same pre-existing
//!   formulas (damage bonus = paladin level = 5).
//! - lay on hands / divine grace still scale correctly at level 5 via the
//!   same pre-existing formulas: uses/day = 1/2 paladin level + Charisma
//!   modifier = `5/2 + 2 = 4` and heal dice = paladin level / 2 = `5/2 = 2`
//!   -- both numerically unchanged from level 4 (the same integer-division
//!   coincidence as the base saves above, not a stalled formula). Divine
//!   grace save bonus stays the positive Charisma modifier.
//! - the effective caster level gate GENUINELY changes at level 5:
//!   `max(5 - 3, 0) = 2` (PF1 Core Rulebook: paladin spells begin at level
//!   4, effective caster level = paladin level - 3) -- a real value change
//!   from `1` at level 4, grounded via the same pre-existing formula (no
//!   re-derivation). This grounds only the caster-level gate arithmetic; it
//!   fabricates no spells known, no spells per day, no bonus spell slots,
//!   and no spell save DCs, and the partial-caster spell blocker keeps
//!   firing.
//! - Mercy, granted once at 3rd level, stays granted (not re-derived) at
//!   level 5: the grant and choice-recognition records both persist
//!   unchanged.
//! - Channel Positive Energy's flat die count GENUINELY increases at level
//!   5: `ceil(paladin level / 2) = ceil(5/2) = 3`, up from 2d6 at level 4,
//!   mirroring the same die-count formula already grounded for Cleric's own
//!   Channel Energy. No healing/damage-resolution execution, no
//!   heal-vs-harm target selection, and no lay-on-hands-resource-consumption
//!   bookkeeping is computed.
//! - Divine Bond, the PF1 CRB's OTHER 5th-level paladin class feature,
//!   was checked against a primary source (legacy.aonprd.com's Core
//!   Rulebook Paladin page) per the operator brief's explicit "verify what
//!   the Special column shows" instruction, and confirmed NOT flat: it
//!   requires an activation/resource-consumption engine (usable a limited
//!   number of times per day, for a duration of "1 minute per paladin
//!   level," mirroring the ki-pool-spend gap already named for Monk High
//!   Jump), and it names two structurally different forms -- a weapon bond
//!   (an ongoing weapon-enhancement-bonus subsystem that can also be spent
//!   on named weapon special abilities) or a mount bond (a full
//!   animal-companion-shaped stat-block/advancement subsystem, mirroring
//!   the still-unproven Ranger "companion" form of Hunter's Bond and the
//!   Druid animal companion) -- so it is deliberately left
//!   named-but-unproven, mirroring the Monk High Jump / Wizard level-5
//!   bonus feat precedent exactly. No explanation or diagnostic record is
//!   fabricated for Divine Bond.
//!
//! It deliberately does not implement any lay-on-hands execution engine, any
//! channel-positive-energy healing/damage-resolution engine, any Divine Bond
//! execution engine, and it does not ground Paladin level 6+ or the
//! partial-caster spell burden's actual content (spells known/prepared,
//! spells per day, bonus spell slots, spell save DCs -- the effective-
//! caster-level gate widening to 2 does NOT mean spell slots are grounded
//! here). It also preserves the accepted Paladin level-1/level-2/level-3/
//! level-4 truth (unchanged), the F6 hybrid baseline, the Ranger negative
//! control, and the Fighter negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{SupportState, seeded_current_truth};

const PALADIN_LEVEL4_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level4_sd13_deterministic_input.txt");

const PALADIN_LEVEL5_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level5_sd13_deterministic_input.txt");

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

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

const EFFECTIVE_CASTER_LEVEL_ID: &str = "class_chassis.paladin.partial_caster.effective_caster_level";
const PARTIAL_CASTER_BLOCKER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

const MERCY_GRANTED_ID: &str = "class_chassis.paladin.mercy_granted";
const MERCY_CHOICE_ID: &str = "class_chassis.paladin.mercy_choice";

const CHANNEL_POSITIVE_ENERGY_DICE_ID: &str = "class_chassis.paladin.channel_positive_energy_dice";

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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
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

// ----- Base attack / base save at level 5 -----

#[test]
fn paladin_level5_base_attack_and_saves_are_grounded_by_the_same_formulas() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, BASE_ATTACK_ID);
    assert_eq!(
        base_attack.value, 5,
        "Paladin level 5 full BAB (classlevel) must equal 5: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, BASE_SAVE_FORTITUDE_ID);
    assert_eq!(
        fortitude.value, 4,
        "Paladin level 5 good Fortitude (5/2+2) must equal 4, numerically unchanged from level 4"
    );

    let reflex = explanation(&computation, BASE_SAVE_REFLEX_ID);
    assert_eq!(
        reflex.value, 1,
        "Paladin level 5 poor Reflex (5/3) must equal 1, numerically unchanged from level 4"
    );

    let will = explanation(&computation, BASE_SAVE_WILL_ID);
    assert_eq!(
        will.value, 4,
        "Paladin level 5 good Will (5/2+2) must equal 4, numerically unchanged from level 4"
    );
}

// ----- Smite Evil stays at 2/day at level 5 (next increase is level 7) -----

#[test]
fn paladin_level5_smite_evil_stays_2_per_day() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 2,
        "smite evil uses per day stays 2/day at level 5 (1 + (5-1)/3 = 2); the next increase \
         does not land until level 7: {uses_per_day:?}"
    );

    // CG-03 fix: Charisma modifier is now +3 (base 14 + 2 Human racial), not +2.
    let attack_bonus = explanation(&computation, SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "smite evil attack bonus must equal the Charisma modifier (+3 for CHA 14 + 2 Human \
         racial): {attack_bonus:?}"
    );

    let damage_bonus = explanation(&computation, SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 5,
        "smite evil damage bonus must equal paladin level (5 at level 5): {damage_bonus:?}"
    );
}

// ----- Lay on hands / divine grace stay numerically unchanged at level 5 -----

#[test]
fn paladin_level5_lay_on_hands_and_divine_grace_scale_correctly() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let uses_per_day = explanation(&computation, LAY_ON_HANDS_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 5,
        "lay on hands uses per day at level 5 (1/2 paladin level + Charisma modifier = 2 + 3) \
         must equal 5, numerically unchanged from level 4: {uses_per_day:?}"
    );

    let heal_amount = explanation(&computation, LAY_ON_HANDS_HEAL_AMOUNT_ID);
    assert_eq!(
        heal_amount.value, 2,
        "lay on hands heal amount at level 5 (paladin level / 2 = 2) must equal 2, numerically \
         unchanged from level 4: {heal_amount:?}"
    );

    let save_bonus = explanation(&computation, DIVINE_GRACE_SAVE_BONUS_ID);
    assert_eq!(
        save_bonus.value, 3,
        "divine grace save bonus at level 5 must still equal the positive Charisma modifier: \
         {save_bonus:?}"
    );
}

// ----- Effective caster level GENUINELY becomes 2 at level 5 -----

#[test]
fn paladin_level5_effective_caster_level_genuinely_becomes_2() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 2,
        "effective caster level at paladin level 5 (max(5 - 3, 0)) must genuinely become 2, up \
         from 1 at level 4: {caster_level:?}"
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
                "no spells are fabricated merely because the effective-caster-level gate widened \
                 again at level 5: {daily_prep:?}"
            );
        }
    }
}

// ----- Mercy stays granted (not re-derived) at level 5 -----

#[test]
fn paladin_level5_mercy_stays_granted_unchanged() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let granted = explanation(&computation, MERCY_GRANTED_ID);
    assert_eq!(
        granted.value, 0,
        "mercy grant record still carries no fabricated mechanical value at level 5: {granted:?}"
    );

    let choice = explanation(&computation, MERCY_CHOICE_ID);
    assert!(
        choice.detail.contains("mercy:shaken"),
        "mercy choice detail must still name the selected mercy (mercy:shaken) at level 5: {}",
        choice.detail
    );
}

// ----- Channel Positive Energy dice count GENUINELY increases to 3d6 at level 5 -----

#[test]
fn paladin_level5_channel_positive_energy_dice_increases_to_3() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 3,
        "channel positive energy dice count at level 5 (ceil(paladin level / 2) = ceil(5/2)) \
         must genuinely increase to 3, up from 2 at level 4: {dice:?}"
    );
}

// ----- Divine Bond: checked against a primary source, confirmed NOT flat, deliberately unproven -----

#[test]
fn paladin_level5_does_not_fabricate_divine_bond() {
    let input = load(PALADIN_LEVEL5_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            // SD-34 decisions.md section 18: widened BY CONSTRUCTION, not narrowed --
            // class_feature_grant_consumer now emits the flat, citation-backed grant
            // fact (Paladin gains Divine Bond at this level, joined to a real corpus
            // record). The MECHANICAL magnitude this test guards (activation/resource-
            // consumption, weapon-enhancement, mount stat block) is still fabricated by
            // nothing, so the exact citation-backed id is carved out by NAME while every
            // other id shape remains caught.
            .any(|e| e.id.contains("divine_bond")
                && e.id != "class_feature.paladin.corpus_record.divine_bond"),
        "Divine Bond (the PF1 CRB's other 5th-level paladin class feature) requires an \
         activation/resource-consumption engine and either a weapon-enhancement subsystem or a \
         full mount stat-block/advancement subsystem, neither of which exists in this codebase; \
         no explanation record must be fabricated for it: {:?}",
        computation.explanations
    );
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("divine_bond")),
        "no diagnostic record should be fabricated for Divine Bond either, since this slice \
         deliberately declines to ground it: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 6 stays unrecognized by this slice (level 6
// was later widened into the supported tranche by a subsequent SD13-E5
// slice; see tests/sd13_paladin_level6_progression.rs) -----

#[test]
fn paladin_level_6_was_later_widened_into_the_supported_tranche() {
    let level_6 = PALADIN_LEVEL5_FIXTURE.replace("class:paladin:5", "class:paladin:6");
    let input = load(&level_6);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.paladin.")),
        "level-6 Paladin was later widened into the supported tranche and must now gain bounded \
         paladin chassis explanations: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the grounding must not leak onto other classes -----

#[test]
fn fighter_and_ranger_do_not_gain_paladin_level5_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(!has_explanation(&fighter_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));

    let ranger_computation = compute_pilot_base_chassis(&load(RANGER_FIXTURE));
    assert!(!has_explanation(&ranger_computation, CHANNEL_POSITIVE_ENERGY_DICE_ID));
}

// ----- Sanity: level 4 fixture is unaffected by the level-5 widening -----

#[test]
fn paladin_level4_channel_positive_energy_dice_stays_at_2_after_the_level5_widening() {
    let input = load(PALADIN_LEVEL4_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dice = explanation(&computation, CHANNEL_POSITIVE_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 2,
        "level 4 channel positive energy dice count must remain 2, unaffected by the level-5 \
         widening: {dice:?}"
    );

    let caster_level = explanation(&computation, EFFECTIVE_CASTER_LEVEL_ID);
    assert_eq!(
        caster_level.value, 1,
        "level 4 effective caster level must remain 1, unaffected by the level-5 widening: \
         {caster_level:?}"
    );
}

// ----- Control plane: the matrix row's note names the level-5 widening -----

#[test]
fn matrix_paladin_row_names_level_5_widening_and_effective_caster_level() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");

    assert_eq!(paladin.support_state, SupportState::Supported);
    assert!(
        paladin.grounding_ref.contains("sd13_paladin_level5_progression"),
        "paladin row must cite the live SD13-E5 level-5 progression proof surface: {}",
        paladin.grounding_ref
    );
    let note = paladin.blocker_or_lossiness_note;
    assert!(
        note.to_lowercase().contains("effective caster level")
            || note.to_lowercase().contains("level 5"),
        "paladin partial note must name the level-5 widening: {note}"
    );
}
