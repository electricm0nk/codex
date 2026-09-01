//! SD18 Ranger level-17 Hide in Plain Sight widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-16
//! hybrid chassis (`tests/sd18_ranger_level16_improved_evasion.rs`, the
//! loop's most recent Ranger ceiling) to Ranger level 17
//! (`supported_ranger_level` is generalized from `1..=16` to `1..=17` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 17`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants) — the loop's genuinely open
//! level-17 frontier, per the prior cycle's own "Next cycle instructions"
//! (cycle-2026-07-15T6100): the §3.2 level-16 sweep is closed except for
//! Druid (a documented structural blocker, not re-attempted here), and
//! Ranger level 17+ is the loop's next open frontier for any of the nine
//! classes already at level 16.
//!
//! §3.1 race rows and §3.3 interaction rows are structurally
//! exhausted/blocked (cited in the progress doc, not re-derived this
//! cycle); §3.4/§3.5 are structurally blocked (same root cause, also
//! cited, not re-derived). Monk is a confirmed permanent dead end at level
//! 13. Druid is a documented structural blocker at level 16 (Wild Shape
//! frequency has never been grounded as a standalone pillar at any level).
//!
//! Ranger was picked because its own level-17 row was fetched fresh (not
//! trusted from any carried-forward risk-map note, per the loop's standing
//! discipline) against THREE independent primary sources — d20pfsrd, the
//! Archives of Nethys aonprd.com mirror, and legacy.aonprd.com — all
//! agreeing byte-for-byte on the level-17 class table row: BAB
//! +17/+12/+7/+2 (full-BAB progression, base value 17), Fort +10, Ref +10,
//! Will +5, Special "Hide in plain sight", base spells per day 4/3/2/1
//! (1st/2nd/3rd/4th). The level-15/16/18 neighboring rows were fetched in
//! the same passes to rule out level misattribution (the exact failure
//! mode that struck the carried-forward Sorcerer and Bard level-16
//! risk-map notes in two prior cycles): level 16 = "Improved evasion",
//! level 18 = "4th favored terrain, combat style feat" — both distinct
//! from level 17's own "Hide in plain sight," confirming this is genuinely
//! level 17's own row, not an adjacent level's text.
//!
//! - level 17 base attack bonus GENUINELY RISES to 17 (full BAB
//!   progression, up from 16 at level 16); good Fortitude and good Reflex
//!   BOTH STAY 10 (`17/2+2 = 10`, an integer-division coincidence with
//!   level 16); poor Will STAYS 5 (`17/3 = 5`, an integer-division
//!   coincidence with level 16).
//! - the base spells-per-day table's level-17 row is `4/3/2/1`
//!   (1st/2nd/3rd/4th), verified independently against all three primary
//!   sources: the 1st-level column GENUINELY RISES from 3 to 4, and the
//!   2nd/3rd/4th-level columns stay 3/2/1 unchanged from level 16 — a
//!   literal table lookup value, not a formula. The spell-level access
//!   ladder itself stays at 4 (already widened at level 13; ranger spells
//!   never reach a 5th spell level at any level).
//! - the PF1 Core Rulebook Ranger class table's level-17 "Special" column
//!   reads only "Hide in plain sight" (verified independently against all
//!   three primary sources, byte-for-byte identical). The ability text
//!   itself (legacy.aonprd.com): "While in any of his favored terrains, a
//!   ranger of 17th level or higher can use the Stealth skill even while
//!   being observed." This is the exact structural mirror of Camouflage
//!   (the already-grounded 12th-level "Special" column entry,
//!   `RANGER_CAMOUFLAGE_LEVEL`): an automatic, no-choice grant with no
//!   numeric magnitude of its own, that only modifies a
//!   hide-while-observed check resolution that does not exist anywhere in
//!   this codebase. This slice grounds Hide in Plain Sight
//!   (`RANGER_HIDE_IN_PLAIN_SIGHT_LEVEL`,
//!   `class_feature.ranger.hide_in_plain_sight`) the same way, mirroring
//!   Camouflage's idiom exactly: a bounded +0 identity/recognition record
//!   only, since no terrain-classification engine and no
//!   Stealth-check-execution engine exists anywhere in this codebase.
//!   Level 17 is NOT a Combat Style bonus-feat level (feats land at
//!   2/6/10/14/18) and NOT a Favored Enemy/Favored Terrain interval (next
//!   intervals land at 18/19), so no other new record appears.
//!
//! It deliberately does not touch the favored-terrain/favored-enemy
//! conditional-application engines, any of the four combat-style bonus
//! feats' own mechanics, Hunter's Bond ally-bonus application or the
//! animal-companion form, Woodland Stride's/Swift Tracker's/Quarry's/
//! Camouflage's own application, the ranger Wisdom prepared-posture/
//! spell-source-lineage burden (all stay named-but-unproven, unchanged from
//! levels 1-16), or any actual save-resolution/damage-halving math for
//! Evasion or Improved Evasion (both stay bounded identity records only, no
//! saving-throw-resolution or damage-resolution engine exists anywhere in
//! this codebase). It does not ground any hide-while-observed
//! check-execution or terrain-classification engine for Hide in Plain
//! Sight itself. It does not ground Ranger level 18+. It also preserves
//! the accepted Ranger level-1..level-16 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s,
//! `tests/sd18_ranger_level11_quarry.rs`'s,
//! `tests/sd18_ranger_level12_widening.rs`'s,
//! `tests/sd18_ranger_level13_widening.rs`'s, and
//! `tests/sd18_ranger_level14_widening.rs`'s own
//! `ranger_level_17_is_not_promoted_by_this_slice` (formerly named for
//! level 17), all moved to a level-18 boundary in the same commit;
//! `tests/sd18_ranger_level16_improved_evasion.rs`'s own level-17
//! negative-control test is removed rather than moved, since level 17 is
//! now itself the supported/grounded row, mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/Rogue/Sorcerer
//! level-N-to-level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const RANGER_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level16_sd18_improved_evasion_deterministic_input.txt"
);

const RANGER_LEVEL17_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level17_sd18_hide_in_plain_sight_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const HIDE_IN_PLAIN_SIGHT_ID: &str = "class_feature.ranger.hide_in_plain_sight";
const IMPROVED_EVASION_ID: &str = "class_feature.ranger.improved_evasion";

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

// ----- Base attack bonus genuinely rises at level 17 -----

#[test]
fn ranger_level17_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 17,
        "Ranger level 17 full-BAB progression must equal 17, genuinely risen from 16: {}",
        base_attack.detail
    );
}

// ----- Good saves and poor Will all stay unchanged (integer-division coincidences) -----

#[test]
fn ranger_level17_saves_stay_unchanged() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Ranger level 17 good Fortitude (17/2+2) must stay 10, an integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(
        reflex.value, 10,
        "Ranger level 17 good Reflex (17/2+2) must stay 10, an integer-division coincidence"
    );

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(
        will.value, 5,
        "Ranger level 17 poor Will (17/3) must stay 5, an integer-division coincidence"
    );
}

// ----- Base spells per day widen at level 17: the 1st-level column genuinely rises -----

#[test]
fn ranger_level17_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "level 17 (`4/3/2/1`): the 2nd/3rd/4th-level columns stay 3/2/1 unchanged, and the \
         1st-level column genuinely rises from 3 to 4"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn ranger_level17_spell_level_access_stays_four() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 17 spell-level access must stay 4 (already widened at level 13): {}",
        access.detail
    );
}

// ----- Hide in Plain Sight is granted at level 17 as a bounded +0 recognition record -----

#[test]
fn ranger_level17_hide_in_plain_sight_is_granted_as_bounded_recognition_record() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hide_in_plain_sight = explanation(&computation, HIDE_IN_PLAIN_SIGHT_ID);
    assert_eq!(
        hide_in_plain_sight.value, 0,
        "Ranger Hide in Plain Sight must be a bounded +0 identity/recognition record only: {}",
        hide_in_plain_sight.detail
    );
    assert!(
        hide_in_plain_sight.detail.contains("Stealth"),
        "the recognition record must name the rule text honestly: {}",
        hide_in_plain_sight.detail
    );
    assert!(
        hide_in_plain_sight.detail.contains("no")
            && (hide_in_plain_sight.detail.contains("terrain-classification")
                || hide_in_plain_sight.detail.contains("Stealth-check-execution")),
        "the recognition record must explicitly disclaim that no terrain-classification or \
         Stealth-check-execution engine is executed: {}",
        hide_in_plain_sight.detail
    );
}

// ----- Improved Evasion (granted at level 16) stays granted and unchanged at level 17 -----

#[test]
fn ranger_level17_improved_evasion_stays_granted_unchanged() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_evasion = explanation(&computation, IMPROVED_EVASION_ID);
    assert_eq!(
        improved_evasion.value, 0,
        "Ranger Improved Evasion must stay a bounded +0 identity/recognition record: {}",
        improved_evasion.detail
    );
}

// ----- Below the level-17 gate, Hide in Plain Sight is a correct level-gate absence -----

#[test]
fn ranger_level16_hide_in_plain_sight_is_absent_below_the_gate() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let hide_in_plain_sight_absence = explanation(&computation, HIDE_IN_PLAIN_SIGHT_ID);
    assert_eq!(
        hide_in_plain_sight_absence.value, 0,
        "Ranger level 16 Hide in Plain Sight must be a correct level-gate absence"
    );
    assert!(
        hide_in_plain_sight_absence.detail.contains("absent"),
        "Ranger level 16 Hide in Plain Sight record must describe the level-gate absence: {}",
        hide_in_plain_sight_absence.detail
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level17_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL17_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-17 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 16 truth is unchanged by this widening -----

#[test]
fn ranger_level16_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 16, "Ranger level 16 base attack bonus must stay 16");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "Ranger level 16 base spells per day must stay `3/3/2/1`"
    );
}

// ----- Negative control removed: level 18 is now the supported/grounded row -----
// (formerly `ranger_level_18_is_not_promoted_by_this_slice`; retired by the SD18
// cycle-2026-07-16T0244 slice, which promotes level 18 for real —
// see tests/sd18_ranger_level18_widening.rs)

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level17_recognition() {
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
fn multiclass_ranger_level17_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL17_FIXTURE.replace(
        "class_level=class:ranger:17",
        "class_level=class:ranger:17\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.ranger.")
                || e.id.starts_with("class_feature.ranger.")),
        "multiclass Ranger must not gain any bounded ranger chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Ranger must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-17 widening -----

#[test]
fn matrix_ranger_row_names_level_17_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level17_hide_in_plain_sight"),
        "ranger row must cite the live SD18 level-17 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 17") || note.contains("level-17"),
        "ranger partial note must name the level-17 widening: {note}"
    );
}
