//! SD18 Ranger level-13 third-favored-terrain widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-12
//! hybrid chassis (`tests/sd18_ranger_level12_widening.rs`, the loop's most
//! recent Ranger ceiling) to Ranger level 13 — the fourth SD-18 §3.2
//! class-row level-13 landing, after Rogue, Barbarian, and Fighter, and the
//! first hybrid/spell-bearing class to reach level 13
//! (`supported_ranger_level` is generalized from `1..=12` to `1..=13` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 13`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants). §3.1 race rows and §3.3
//! interaction rows are structurally exhausted/blocked (cited in the
//! progress doc, not re-derived this cycle); §3.4/§3.5 are structurally
//! blocked (same root cause, also cited, not re-derived). Monk is a
//! confirmed dead end at level 13 (Diamond Soul needs spell resistance,
//! which does not exist anywhere in this codebase); Rogue, Barbarian, and
//! Fighter are already landed at level 13. Ranger was picked over the other
//! five remaining level-12 §3.2 rows (Bard, Cleric, Druid, Paladin,
//! Sorcerer, Wizard) because its own level-13 "Special" column entry (the
//! Favored Terrain rule's 13th-level interval) is the exact structural
//! mirror of the already-grounded Favored Enemy 10th-level interval — a
//! well-precedented, small widening rather than a fresh assessment.
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! cross-checked against a third mirror (legacy.aonprd.com) once the base
//! spells-per-day table showed a change beyond the Special column, and all
//! three agree byte-for-byte:
//!
//! - level 13 base attack bonus GENUINELY RISES to +13 (full BAB
//!   progression, up from +12 at level 12; the table's own "+13/+8/+3"
//!   iterative notation is not modeled anywhere in this codebase, only the
//!   flat base value); all three base saves STAY numerically unchanged from
//!   level 12 (Fortitude/Reflex `13/2+2 = 8`, Will `13/3 = 4`) —
//!   integer-division coincidences, re-verified rather than assumed.
//! - the PF1 Core Rulebook Ranger class table's level-13 "Special" column
//!   reads only "3rd favored terrain" (verified independently against all
//!   three primary sources) — the Favored Terrain rule's own 13th-level
//!   interval (8th level + 5), the exact structural mirror of the
//!   already-grounded Favored Enemy 10th-level interval: a THIRD favored
//!   terrain type selection (`choice:ranger_favored_terrain_3`, open-ended
//!   raw-string recognition, mirroring the second favored terrain's own
//!   idiom exactly) plus the 13th-level interval's OWN bonus-increase
//!   TARGET choice (`choice:ranger_favored_terrain_bonus_increase_target_2`,
//!   restricted to `terrain:first` / `terrain:second` / `terrain:third`),
//!   STACKING with the already-grounded 8th-level interval's own increase
//!   when both target the same terrain (this fixture's first favored
//!   terrain rises to +6: 2 base + 2 at 8th + 2 at 13th).
//! - the base spells-per-day table's level-13 row is `3/2/1/0`
//!   (1st/2nd/3rd/4th), verified independently against all three primary
//!   sources: the 1st-level column GENUINELY RISES from 2 to 3 (a literal
//!   table lookup value, not a formula), the 2nd/3rd-level columns stay 2/1
//!   unchanged, and the 4th-level column NEWLY OPENS at 0 (a genuine table
//!   entry, not an absence) — 4th-level ranger spells begin at ranger level
//!   13 exactly, checked rather than assumed away. The spell-level access
//!   ladder (`class_chassis.ranger.partial_caster.spell_level_access`)
//!   correspondingly widens from 3 to 4 for the first time
//!   (`RANGER_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`), and the base
//!   spell-save-DC and Wisdom-bonus-spells families both extend to the new
//!   4th spell level automatically (live arithmetic over the widened
//!   access ladder, no new formula invented).
//!
//! It deliberately does not touch the favored-terrain conditional-
//! application engine (no terrain-detection or Stealth/Initiative/
//! skill-check-execution engine exists anywhere in this codebase, so no
//! total is ever modified by any of these records), the favored-enemy
//! conditional-application engine, either combat-style bonus feat's own
//! mechanics, Hunter's Bond ally-bonus application or the animal-companion
//! form, Woodland Stride's/Swift Tracker's/Quarry's/Camouflage's own
//! application, or the ranger Wisdom prepared-posture/spell-source-lineage
//! burden (all stay named-but-unproven, unchanged from levels 1-12), and it
//! does not ground Ranger level 14+ or the Favored Terrain rule's own
//! 18th-level interval. It also preserves the accepted Ranger
//! level-1..level-12 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.
//!
//! This slice also fixes two pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s
//! `ranger_level_12_is_not_promoted_by_this_slice` and
//! `tests/sd18_ranger_level12_widening.rs`'s
//! `ranger_level_13_is_not_promoted_by_this_slice`, both moved to a
//! level-14 boundary in the same commit, mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/Rogue level-N-to-
//! level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level12_sd18_camouflage_deterministic_input.txt"
);

const RANGER_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level13_sd18_third_favored_terrain_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const FAVORED_TERRAIN_ID: &str = "class_feature.ranger.favored_terrain";
const FAVORED_TERRAIN_2_ID: &str = "class_feature.ranger.favored_terrain_2";
const FAVORED_TERRAIN_3_CHOICE_ID: &str = "class_chassis.ranger.favored_terrain_3_choice";
const FAVORED_TERRAIN_3_ID: &str = "class_feature.ranger.favored_terrain_3";
const FAVORED_TERRAIN_BONUS_INCREASE_2_CHOICE_ID: &str =
    "class_chassis.ranger.favored_terrain_bonus_increase_2_choice";

fn values_with_prefix(
    computation: &PilotBaseChassisComputation,
    prefix: &str,
) -> Vec<(String, i16)> {
    computation
        .explanations
        .iter()
        .filter(|e| e.id.starts_with(prefix))
        .map(|e| (e.id.clone(), e.value))
        .collect()
}

// ----- Base attack bonus genuinely rises at level 13 -----

#[test]
fn ranger_level13_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 13,
        "Ranger level 13 full-BAB progression must equal 13, genuinely risen from 12: {}",
        base_attack.detail
    );
}

// ----- Base saves stay numerically unchanged at level 13 (integer-division coincidences) -----

#[test]
fn ranger_level13_base_saves_stay_unchanged() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Ranger level 13 good Fortitude (13/2+2) must stay 8, an integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(reflex.value, 8, "Ranger level 13 good Reflex (13/2+2) must stay 8");

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(will.value, 4, "Ranger level 13 poor Will (13/3) must stay 4");
}

// ----- Base spells per day widen at level 13, including a genuinely new 4th-level column -----

#[test]
fn ranger_level13_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 0),
        ],
        "level 13 (`3/2/1/0`): the 1st-level column rises from 2 to 3, the 2nd/3rd-level \
         columns stay 2/1 unchanged, and the 4th-level column newly opens at 0"
    );
}

// ----- The spell-level access ladder widens to 4 for the first time -----

#[test]
fn ranger_level13_spell_level_access_widens_to_four() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 13 spell-level access must genuinely widen to 4: {}",
        access.detail
    );

    let dc4 = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_save_dc.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &dc4.id),
        "the 4th-level spell save DC must be grounded now that the access ladder reaches 4"
    );

    let bonus4 = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.bonus_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &bonus4.id),
        "the 4th-level Wisdom bonus-spells record must be grounded now that the access ladder \
         reaches 4"
    );

    let total4 = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.total_spells_per_day.spell_level_4",
    );
    assert!(
        has_explanation(&computation, &total4.id),
        "the 4th-level total spells-per-day record must be grounded now that the access ladder \
         reaches 4"
    );
}

// ----- The third favored terrain is newly grounded, and stacking is honored -----

#[test]
fn ranger_level13_third_favored_terrain_is_recognized() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_TERRAIN_3_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the third favored-terrain selection must be a +0 recognition record only"
    );
    assert!(
        choice.detail.contains("swamp"),
        "the recognition record must name the raw chosen terrain string (terrain:swamp): {}",
        choice.detail
    );

    let third = explanation(&computation, FAVORED_TERRAIN_3_ID);
    assert_eq!(
        third.value, 2,
        "the third favored terrain's own bonus must be the flat base +2 on this fixture (the \
         13th-level interval target names the FIRST terrain, not the third): {}",
        third.detail
    );
}

#[test]
fn ranger_level13_thirteenth_level_interval_stacks_the_first_terrain_to_six() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let target = explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_2_CHOICE_ID);
    assert_eq!(
        target.value, 0,
        "the 13th-level bonus-increase target selection must be a +0 recognition record only"
    );
    assert!(
        target.detail.contains("first favored terrain"),
        "the recognition record must name the first favored terrain as the boosted one: {}",
        target.detail
    );

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        first.value, 6,
        "the first favored terrain's bonus must genuinely rise to +6 when both the 8th-level \
         and 13th-level interval increases target it (2 + 2 + 2): {}",
        first.detail
    );

    let second = explanation(&computation, FAVORED_TERRAIN_2_ID);
    assert_eq!(
        second.value, 2,
        "the second favored terrain must stay at the flat base +2, untargeted at either interval"
    );
}

#[test]
fn ranger_level13_targeting_the_third_terrain_boosts_the_third_terrain() {
    let swapped = RANGER_LEVEL13_FIXTURE.replace(
        "choice=choice:ranger_favored_terrain_bonus_increase_target_2:terrain:first",
        "choice=choice:ranger_favored_terrain_bonus_increase_target_2:terrain:third",
    );
    let input = load(&swapped);
    let computation = compute_pilot_base_chassis(&input);

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        first.value, 4,
        "the first favored terrain must keep only its 8th-level increase (+4) when the \
         13th-level target names the third terrain"
    );

    let third = explanation(&computation, FAVORED_TERRAIN_3_ID);
    assert_eq!(
        third.value, 4,
        "the third favored terrain's bonus must genuinely rise to +4 when the 13th-level \
         increase targets it (\"including the one just selected, if so desired\"): {}",
        third.detail
    );
}

#[test]
fn ranger_level13_unrecognized_thirteenth_level_target_boosts_nothing() {
    let unrecognized = RANGER_LEVEL13_FIXTURE.replace(
        "choice=choice:ranger_favored_terrain_bonus_increase_target_2:terrain:first",
        "choice=choice:ranger_favored_terrain_bonus_increase_target_2:terrain:fourth",
    );
    let input = load(&unrecognized);
    let computation = compute_pilot_base_chassis(&input);

    let target = explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_2_CHOICE_ID);
    assert_eq!(target.value, 0);
    assert!(
        target.detail.contains("restricted"),
        "an unrecognized target must be surfaced as outside the restricted set, with no target \
         identity grounded: {}",
        target.detail
    );

    let first = explanation(&computation, FAVORED_TERRAIN_ID);
    assert_eq!(
        first.value, 4,
        "the first terrain must keep only its 8th-level increase; no 13th-level boost may be \
         fabricated from an unrecognized target"
    );
    let third = explanation(&computation, FAVORED_TERRAIN_3_ID);
    assert_eq!(third.value, 2, "no boost may be fabricated from an unrecognized target");
}

// ----- Camouflage stays granted, not re-derived -----

#[test]
fn ranger_level13_camouflage_stays_granted_unchanged() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let camouflage = explanation(&computation, "class_feature.ranger.camouflage");
    assert_eq!(
        camouflage.value, 0,
        "Camouflage's grant-only identity record must carry no fabricated mechanical value"
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level13_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-13 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 12 truth is unchanged by this widening -----

#[test]
fn ranger_level12_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 12, "Ranger level 12 base attack bonus must stay 12");

    assert!(
        !has_explanation(&computation, FAVORED_TERRAIN_3_CHOICE_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_3_ID)
            && !has_explanation(&computation, FAVORED_TERRAIN_BONUS_INCREASE_2_CHOICE_ID),
        "level-12 Ranger must not gain any third-favored-terrain record: {:?}",
        computation.explanations
    );

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
        ],
        "Ranger level 12 base spells per day must stay `2/2/1` with no 4th-level column"
    );
}

// ----- Negative control: level 16 stays unrecognized by this slice -----
//
// A later SD18 widening (cycle-2026-07-15T2100,
// tests/sd18_ranger_level14_widening.rs) now genuinely recognizes Ranger
// level 14 too (base attack and both good saves rise, the fourth
// combat-style bonus feat and the base spells-per-day table's 4th-level
// column are newly grounded), so this boundary control moved from level 14
// to level 15. A still later SD18 widening (cycle-2026-07-15T4000,
// tests/sd18_ranger_level15_widening.rs) now genuinely recognizes Ranger
// level 15 too, so this boundary control moved once more to level 16, and a
// still further SD18 widening (cycle-2026-07-15T6100,
// tests/sd18_ranger_level16_improved_evasion.rs) now genuinely recognizes
// Ranger level 16 too, and a still further SD18 widening
// (cycle-2026-07-15T7000, tests/sd18_ranger_level17_hide_in_plain_sight.rs)
// now genuinely recognizes Ranger level 17 too, and a still further SD18
// widening (cycle-2026-07-16T0244, tests/sd18_ranger_level18_widening.rs)
// now genuinely recognizes Ranger level 18 too, and a still further SD18
// widening (cycle-2026-07-16T3200, tests/sd18_ranger_level19_widening.rs)
// now genuinely recognizes Ranger level 19 too, and a still further SD18
// widening (cycle-2026-07-16T1600, tests/sd18_ranger_level20_widening.rs)
// now genuinely recognizes Ranger level 20 too, so this boundary control
// moves once more to level 21 (a pure implementation-gate check, since PF1
// has no 21st character level).

#[test]
fn ranger_level_21_is_not_promoted_by_this_slice() {
    let level_21 = RANGER_LEVEL13_FIXTURE.replace("class:ranger:13", "class:ranger:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger."))
                // SD-34 wave 34 lane A (`docs/release/SD-34-book-completion/artifacts/
                // bucket-d-mining/wave34_laneA_weapon_and_armor_proficiency_cycle_
                // receipt.md`): Ranger's own Weapon and Armor Proficiency identity
                // grant is now genuinely grounded as a level-independent, always-on
                // +0 record (true since level 1, mirrors the same "no gate to lift"
                // idiom as Jack-of-All-Trades) -- not a bounded, level-gated feature
                // this slice's negative control is checking for.
                && e.id != "class_feature.ranger.weapon_and_armor_proficiency"),
        "level-21 Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level13_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "the Fighter chassis must not surface any ranger-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Ranger is not promoted -----

#[test]
fn multiclass_ranger_level13_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL13_FIXTURE.replace(
        "class_level=class:ranger:13",
        "class_level=class:ranger:13\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger."))
                // SD-34 wave 34 lane A (`docs/release/SD-34-book-completion/artifacts/
                // bucket-d-mining/wave34_laneA_weapon_and_armor_proficiency_cycle_
                // receipt.md`): Ranger's own Weapon and Armor Proficiency identity
                // grant is now genuinely grounded as a level-independent, always-on
                // +0 record (true since level 1, mirrors the same "no gate to lift"
                // idiom as Jack-of-All-Trades) -- not a bounded, level-gated feature
                // this slice's negative control is checking for.
                && e.id != "class_feature.ranger.weapon_and_armor_proficiency"),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-13 widening -----

#[test]
fn matrix_ranger_row_names_level_13_widening() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid_chassis_and_spell_burden row must exist");

    assert_eq!(ranger.support_state, SupportState::Supported);
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger.grounding_ref.contains("sd18_ranger_level13_widening"),
        "ranger row must cite the live SD18 level-13 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 13") || note.contains("level-13"),
        "ranger partial note must name the level-13 widening: {note}"
    );
}
