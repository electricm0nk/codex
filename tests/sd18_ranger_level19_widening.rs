//! SD18 Ranger level-19 Improved Quarry widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-18
//! hybrid chassis (`tests/sd18_ranger_level18_widening.rs`, the loop's most
//! recent Ranger ceiling) to Ranger level 19
//! (`supported_ranger_level` is generalized from `1..=18` to `1..=19` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 19`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants) — the loop's SIXTH §3.2
//! level-19 landing, after Barbarian, Cleric, Fighter, Bard, and Paladin.
//!
//! §3.1 race rows and §3.3 interaction rows are structurally
//! exhausted/blocked (cited in the progress doc, not re-derived this
//! cycle); §3.4/§3.5 are structurally blocked (same root cause, also
//! cited, not re-derived). Druid stays capped at level 15 and Monk at
//! level 12, both documented structural exceptions, not re-attempted here.
//!
//! Ranger was picked per the prior cycle's own "Next cycle instructions"
//! (alphabetically first among the untouched-at-level-19 classes: Ranger,
//! Rogue, Sorcerer, Wizard), verified fresh rather than assumed. Both PF1
//! CRB primary sources (d20pfsrd and the Archives of Nethys aonprd.com
//! mirror) were fetched directly via raw curl + tag-strip (not summarized
//! WebFetch) before writing any code or test, and both agree byte-for-byte
//! on the level-19 class table row: BAB +19/+14/+9/+4 (full-BAB
//! progression, base value 19), Fort +11, Ref +11, Will +6, Special
//! "Improved quarry", base spells per day 4/3/3/2 (1st/2nd/3rd/4th). The
//! level-17/18/20 neighboring rows were fetched in the same pass to rule
//! out level misattribution: level 17 = "Hide in plain sight", level 18 =
//! "4th favored terrain, combat style feat", level 20 = "5th favored
//! enemy, master hunter" — all distinct from level 19's own "Improved
//! quarry," confirming this is genuinely level 19's own row, not an
//! adjacent level's text. The Improved Quarry rule text itself was also
//! fetched fresh from both sources and agrees byte-for-byte: "At 19th
//! level, the ranger's ability to hunt his quarry improves. He can now
//! select a quarry as a free action, and can now take 20 while using
//! Survival to track his quarry, while moving at normal speed without
//! penalty. His insight bonus to attack his quarry increases to +4. If his
//! quarry is killed or dismissed, he can select a new one after 10 minutes
//! have passed."
//!
//! - level 19 base attack bonus GENUINELY RISES to 19 (full BAB
//!   progression, up from 18 at level 18); good Fortitude and good Reflex
//!   BOTH STAY 11 (`19/2+2 = 11`, an integer-division coincidence with
//!   level 18); poor Will STAYS 6 (`19/3 = 6`, an integer-division
//!   coincidence with level 18).
//! - the base spells-per-day table's level-19 row is `4/3/3/2`
//!   (1st/2nd/3rd/4th), verified independently against both primary
//!   sources: the 1st/2nd/4th-level columns stay 4/3/2 unchanged from
//!   level 18, and the 3rd-level column GENUINELY RISES from 2 to 3 (a
//!   literal table lookup value, not a formula) — the spell-level access
//!   ladder itself stays at 4 (already widened at level 13; ranger spells
//!   never reach a 5th spell level at any level).
//! - Improved Quarry is an UPGRADE of the already-grounded 11th-level
//!   Quarry identity (`RANGER_QUARRY_LEVEL`), the exact structural mirror
//!   of Improved Evasion's own upgrade of Evasion: the free-action
//!   reselection and take-20-while-tracking behaviors are grounded as a
//!   new bounded grant-only identity record (value 0, mirroring the
//!   Improved Evasion idiom exactly, pushed only at/above the level-19
//!   gate with no separate absence record below it — mirroring
//!   Evasion/Improved Evasion's own no-else-branch shape, not the
//!   Camouflage/Hide-in-Plain-Sight absent/present shape); the reduced
//!   10-minute reselection cooldown is named in the same record's detail
//!   text (not tracked as state, mirroring the base Quarry record's own
//!   cooldown-naming idiom); and the rule's own insight attack-roll bonus
//!   genuinely rises from +2 to +4 on the ALREADY-EXISTING
//!   `class_chassis.ranger.quarry_attack_bonus` explanation id, mirroring
//!   the Bard Inspire Competence tiered-magnitude idiom exactly (same
//!   explanation id, larger value at the higher tier) rather than minting
//!   a new id for the same magnitude family. 19th level is NOT a Favored
//!   Enemy interval (next lands at 20th), NOT a Favored Terrain interval
//!   (next lands at 23rd, out of scope), and NOT a Combat Style Feat level
//!   (feats land at 2/6/10/14/18), so no other new record appears.
//!
//! It deliberately does not touch the favored-terrain/favored-enemy
//! conditional-application engines, any of the five combat-style bonus
//! feats' own mechanics, Hunter's Bond ally-bonus application or the
//! animal-companion form, Woodland Stride's/Swift Tracker's/Camouflage's/
//! Hide in Plain Sight's own application, the ranger Wisdom
//! prepared-posture/spell-source-lineage burden (all stay
//! named-but-unproven, unchanged from levels 1-18), any actual
//! save-resolution/damage-resolution math for Evasion or Improved Evasion,
//! nor any target-selection/conditional-application engine for Quarry's
//! or Improved Quarry's own effects (no action-economy engine and no
//! Survival-check-execution engine exists anywhere in this codebase). It
//! does not ground Ranger level 20 (the Favored Enemy rule's own 20th-
//! level interval and Master Hunter stay out of scope). It also preserves
//! the accepted Ranger level-1..level-18 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s,
//! `tests/sd18_ranger_level11_quarry.rs`'s,
//! `tests/sd18_ranger_level12_widening.rs`'s,
//! `tests/sd18_ranger_level13_widening.rs`'s, and
//! `tests/sd18_ranger_level14_widening.rs`'s own
//! `ranger_level_19_is_not_promoted_by_this_slice` (formerly named for
//! level 19 already since the level-18 cycle moved them there), all moved
//! to a level-20 boundary in the same commit;
//! `tests/sd18_ranger_level18_widening.rs`'s own level-19
//! negative-control test is removed rather than moved, since level 19 is
//! now itself the supported/grounded row, mirroring the
//! Barbarian/Bard/Cleric/Fighter/Paladin level-N-to-level-(N+1) sibling-fix
//! precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL18_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level18_sd18_fourth_favored_terrain_and_fifth_combat_style_feat_deterministic_input.txt"
);

const RANGER_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level19_sd18_improved_quarry_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const IMPROVED_QUARRY_ID: &str = "class_feature.ranger.improved_quarry";
const QUARRY_ATTACK_BONUS_ID: &str = "class_chassis.ranger.quarry_attack_bonus";

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

// ----- Base attack bonus genuinely rises at level 19 -----

#[test]
fn ranger_level19_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 19,
        "Ranger level 19 full-BAB progression must equal 19, genuinely risen from 18: {}",
        base_attack.detail
    );
}

// ----- All three base saves stay unchanged (integer-division coincidences) -----

#[test]
fn ranger_level19_base_saves_stay_unchanged() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 11,
        "Ranger level 19 good Fortitude (19/2+2) must stay 11, unchanged from level 18"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(
        reflex.value, 11,
        "Ranger level 19 good Reflex (19/2+2) must stay 11, unchanged from level 18"
    );

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(
        will.value, 6,
        "Ranger level 19 poor Will (19/3) must stay 6, unchanged from level 18"
    );
}

// ----- Base spells per day widen at level 19: the 3rd-level column genuinely rises -----

#[test]
fn ranger_level19_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 2),
        ],
        "level 19 (`4/3/3/2`): the 1st/2nd/4th-level columns stay 4/3/2 unchanged, and the \
         3rd-level column genuinely rises from 2 to 3"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn ranger_level19_spell_level_access_stays_four() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 19 spell-level access must stay 4 (already widened at level 13): {}",
        access.detail
    );
}

// ----- Improved Quarry is granted as a bounded identity record at level 19 -----

#[test]
fn ranger_level19_improved_quarry_is_granted() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_quarry = explanation(&computation, IMPROVED_QUARRY_ID);
    assert_eq!(
        improved_quarry.value, 0,
        "Improved Quarry must be a bounded grant-only identity record (value 0): {}",
        improved_quarry.detail
    );
    assert!(
        improved_quarry.detail.contains("free action"),
        "the record must name the free-action reselection upgrade: {}",
        improved_quarry.detail
    );
    assert!(
        improved_quarry.detail.contains("take 20"),
        "the record must name the take-20-while-tracking upgrade: {}",
        improved_quarry.detail
    );
    assert!(
        improved_quarry.detail.contains("10 minutes"),
        "the record must name the reduced 10-minute reselection cooldown: {}",
        improved_quarry.detail
    );
}

// ----- Improved Quarry is absent below the level-19 gate (no record at all) -----

#[test]
fn ranger_level18_improved_quarry_grounds_no_record() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, IMPROVED_QUARRY_ID),
        "Improved Quarry must not appear at all below its level-19 gate (mirroring Evasion/ \
         Improved Evasion's own no-else-branch shape): {:?}",
        computation.explanations
    );
}

// ----- The quarry attack bonus genuinely rises from +2 to +4 at level 19 -----

#[test]
fn ranger_level19_quarry_attack_bonus_rises_to_four() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, QUARRY_ATTACK_BONUS_ID);
    assert_eq!(
        bonus.value, 4,
        "the Quarry insight attack-roll bonus must genuinely rise to +4 at level 19 (Improved \
         Quarry), up from +2 at level 18: {}",
        bonus.detail
    );
}

// ----- The quarry attack bonus stays +2 at level 18 (negative control on the same id) -----

#[test]
fn ranger_level18_quarry_attack_bonus_stays_two() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bonus = explanation(&computation, QUARRY_ATTACK_BONUS_ID);
    assert_eq!(
        bonus.value, 2,
        "the Quarry insight attack-roll bonus must stay +2 at level 18, below the Improved \
         Quarry gate: {}",
        bonus.detail
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level19_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-19 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 18 truth is unchanged by this widening -----

#[test]
fn ranger_level18_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL18_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 18, "Ranger level 18 base attack bonus must stay 18");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 4),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 2),
        ],
        "Ranger level 18 base spells per day must stay `4/3/2/2`"
    );
}

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level19_recognition() {
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
fn multiclass_ranger_level19_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL19_FIXTURE.replace(
        "class_level=class:ranger:19",
        "class_level=class:ranger:19\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-19 widening -----

#[test]
fn matrix_ranger_row_names_level_19_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level19_widening"),
        "ranger row must cite the live SD18 level-19 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 19") || note.contains("level-19"),
        "ranger partial note must name the level-19 widening: {note}"
    );
}
