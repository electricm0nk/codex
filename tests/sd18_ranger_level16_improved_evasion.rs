//! SD18 Ranger level-16 Improved Evasion widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-15
//! hybrid chassis (`tests/sd18_ranger_level15_widening.rs`, the loop's most
//! recent Ranger ceiling) to Ranger level 16 (`supported_ranger_level` is
//! generalized from `1..=15` to `1..=16` via `MAX_SUPPORTED_RANGER_LEVEL =
//! 16`, exactly as prior cycles widened the sibling `MAX_SUPPORTED_*_LEVEL`
//! constants) — the loop's NINTH §3.2 level-16 landing, after Barbarian,
//! Fighter, Wizard, Rogue, Cleric, Paladin, Sorcerer, and Bard.
//!
//! §3.1 race rows and §3.3 interaction rows are structurally
//! exhausted/blocked (cited in the progress doc, not re-derived this
//! cycle); §3.4/§3.5 are structurally blocked (same root cause, also
//! cited, not re-derived). Monk is a confirmed permanent dead end at level
//! 13 (Diamond Soul needs spell resistance, which does not exist anywhere
//! in this codebase). Druid's level-16 "Wild shape (7/day)" was
//! investigated last cycle (cycle-2026-07-15T6000) and found NOT eligible:
//! Wild Shape's frequency-per-day has never been grounded as a standalone
//! pillar at ANY prior level (4/6/8/10/12/14 all declined it), so widening
//! it at level 16 would be a first-time grounding, out of scope under the
//! Wild-Shape-execution-engine hard stop.
//!
//! Ranger was picked as the last remaining eligible §3.2 level-16
//! candidate. Both PF1 CRB primary sources (d20pfsrd and the Archives of
//! Nethys aonprd.com mirror) were read directly before writing this test,
//! requesting the level-14/15/16 rows together in the same fetch to guard
//! against level misattribution (the exact failure mode that struck the
//! carried-forward Sorcerer and Bard level-16 risk-map notes in the two
//! prior cycles): both sources agree byte-for-byte on the level-16 class
//! table row: BAB +16/+11/+6/+1 (full-BAB progression, base value 16), Fort
//! +10, Ref +10, Will +5, Special "Improved evasion", base spells per day
//! 3/3/2/1 (1st/2nd/3rd/4th).
//!
//! - level 16 base attack bonus GENUINELY RISES to 16 (full BAB
//!   progression, up from 15 at level 15); good Fortitude and good Reflex
//!   BOTH GENUINELY RISE to 10 (`16/2+2 = 10`, up from 9 at level 15); poor
//!   Will STAYS 5 (`16/3 = 5`, an integer-division coincidence with level
//!   15).
//! - the base spells-per-day table's level-16 row is `3/3/2/1`
//!   (1st/2nd/3rd/4th), verified independently against both primary
//!   sources: the 1st-level column GENUINELY RISES from 2 to 3, and the
//!   2nd/3rd/4th-level columns stay 3/2/1 unchanged from level 15 — a
//!   literal table lookup value, not a formula. The spell-level access
//!   ladder itself stays at 4 (already widened at level 13; ranger spells
//!   never reach a 5th spell level at any level).
//! - the PF1 Core Rulebook Ranger class table's level-16 "Special" column
//!   reads only "Improved evasion" (verified independently against both
//!   primary sources, byte-for-byte identical). The prior cycle's
//!   investigation (cycle-2026-07-15T6000) found the carried-forward
//!   risk-map claim that "Improved Evasion is a mechanic no class in this
//!   codebase currently grounds" was FALSE, checked directly against
//!   `pilot_compute.rs`: Monk's own Improved Evasion
//!   (`MONK_IMPROVED_EVASION_LEVEL`, granted at 9th level) and Ranger's own
//!   base Evasion (`RANGER_EVASION_LEVEL`, granted at 9th level,
//!   `class_feature.ranger.evasion`) are BOTH already grounded as bounded
//!   `+0` identity/recognition records only — a genuinely NEW named entry
//!   naming the mechanic honestly, with no saving-throw-resolution or
//!   damage-resolution engine anywhere in this codebase, so no damage math
//!   is fabricated from it. This slice grounds Ranger's own Improved
//!   Evasion (`RANGER_IMPROVED_EVASION_LEVEL`,
//!   `class_feature.ranger.improved_evasion`) the same way, mirroring both
//!   precedents exactly. Level 16 is NOT a Combat Style bonus-feat level
//!   (feats land at 2/6/10/14/18) and NOT a Favored Enemy/Favored Terrain
//!   interval (next intervals land at 18/20), so no other new record
//!   appears.
//!
//! It deliberately does not touch the favored-terrain/favored-enemy
//! conditional-application engines, any of the four combat-style bonus
//! feats' own mechanics, Hunter's Bond ally-bonus application or the
//! animal-companion form, Woodland Stride's/Swift Tracker's/Quarry's/
//! Camouflage's own application, the ranger Wisdom prepared-posture/
//! spell-source-lineage burden (all stay named-but-unproven, unchanged from
//! levels 1-15), or any actual save-resolution/damage-halving math for
//! Evasion or Improved Evasion (both stay bounded identity records only, no
//! saving-throw-resolution or damage-resolution engine exists anywhere in
//! this codebase). It does not ground Ranger level 17+. It also preserves
//! the accepted Ranger level-1..level-15 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s,
//! `tests/sd18_ranger_level11_quarry.rs`'s,
//! `tests/sd18_ranger_level12_widening.rs`'s,
//! `tests/sd18_ranger_level13_widening.rs`'s, and
//! `tests/sd18_ranger_level14_widening.rs`'s own
//! `ranger_level_16_is_not_promoted_by_this_slice` (formerly named for level
//! 16), all moved to a level-17 boundary in the same commit;
//! `tests/sd18_ranger_level15_widening.rs`'s own level-16 negative-control
//! test is removed rather than moved, since level 16 is now itself the
//! supported/grounded row, mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/Rogue/Sorcerer
//! level-N-to-level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level15_sd18_fourth_favored_enemy_deterministic_input.txt"
);

const RANGER_LEVEL16_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level16_sd18_improved_evasion_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const IMPROVED_EVASION_ID: &str = "class_feature.ranger.improved_evasion";
const EVASION_ID: &str = "class_feature.ranger.evasion";

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

// ----- Base attack bonus genuinely rises at level 16 -----

#[test]
fn ranger_level16_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 16,
        "Ranger level 16 full-BAB progression must equal 16, genuinely risen from 15: {}",
        base_attack.detail
    );
}

// ----- Good saves genuinely rise, poor Will stays unchanged -----

#[test]
fn ranger_level16_good_saves_genuinely_rise_poor_will_stays() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 10,
        "Ranger level 16 good Fortitude (16/2+2) must genuinely rise to 10, up from 9"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(
        reflex.value, 10,
        "Ranger level 16 good Reflex (16/2+2) must genuinely rise to 10, up from 9"
    );

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(
        will.value, 5,
        "Ranger level 16 poor Will (16/3) must stay 5, an integer-division coincidence"
    );
}

// ----- Base spells per day widen at level 16: the 1st-level column genuinely rises -----

#[test]
fn ranger_level16_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "level 16 (`3/3/2/1`): the 2nd/3rd/4th-level columns stay 3/2/1 unchanged, and the \
         1st-level column genuinely rises from 2 to 3"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn ranger_level16_spell_level_access_stays_four() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 16 spell-level access must stay 4 (already widened at level 13): {}",
        access.detail
    );
}

// ----- Improved Evasion is granted at level 16 as a bounded +0 recognition record -----

#[test]
fn ranger_level16_improved_evasion_is_granted_as_bounded_recognition_record() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let improved_evasion = explanation(&computation, IMPROVED_EVASION_ID);
    assert_eq!(
        improved_evasion.value, 0,
        "Ranger Improved Evasion must be a bounded +0 identity/recognition record only: {}",
        improved_evasion.detail
    );
    assert!(
        improved_evasion.detail.contains("half damage"),
        "the recognition record must name the rule text honestly: {}",
        improved_evasion.detail
    );
    assert!(
        improved_evasion.detail.contains("no")
            && (improved_evasion.detail.contains("saving-throw-resolution")
                || improved_evasion.detail.contains("damage-resolution")),
        "the recognition record must explicitly disclaim that no save-resolution or \
         damage-halving math is executed: {}",
        improved_evasion.detail
    );
}

// ----- Base Evasion (granted at level 9) stays granted and unchanged at level 16 -----

#[test]
fn ranger_level16_base_evasion_stays_granted_unchanged() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let evasion = explanation(&computation, EVASION_ID);
    assert_eq!(
        evasion.value, 0,
        "Ranger base Evasion must stay a bounded +0 identity/recognition record: {}",
        evasion.detail
    );
}

// ----- Below the level-16 gate, Improved Evasion is absent -----

#[test]
fn ranger_level15_improved_evasion_is_absent_below_the_gate() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, IMPROVED_EVASION_ID),
        "level-15 Ranger must not gain any Improved Evasion record: {:?}",
        computation.explanations
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level16_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL16_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-16 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 15 truth is unchanged by this widening -----

#[test]
fn ranger_level15_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 15, "Ranger level 15 base attack bonus must stay 15");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "Ranger level 15 base spells per day must stay `3/2/2/1`"
    );
}

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level16_recognition() {
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
fn multiclass_ranger_level16_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL16_FIXTURE.replace(
        "class_level=class:ranger:16",
        "class_level=class:ranger:16\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-16 widening -----

#[test]
fn matrix_ranger_row_names_level_16_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level16_improved_evasion"),
        "ranger row must cite the live SD18 level-16 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 16") || note.contains("level-16"),
        "ranger partial note must name the level-16 widening: {note}"
    );
}
