//! SD18 Ranger level-15 fourth-favored-enemy widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-14
//! hybrid chassis (`tests/sd18_ranger_level14_widening.rs`, the loop's most
//! recent Ranger ceiling) to Ranger level 15 (`supported_ranger_level` is
//! generalized from `1..=14` to `1..=15` via `MAX_SUPPORTED_RANGER_LEVEL =
//! 15`, exactly as prior cycles widened the sibling `MAX_SUPPORTED_*_LEVEL`
//! constants) — the loop's SIXTH §3.2 level-15 landing, after Barbarian,
//! Rogue, Fighter, Cleric, and Druid. §3.1 race rows and §3.3 interaction
//! rows are structurally exhausted/blocked (cited in the progress doc, not
//! re-derived this cycle); §3.4/§3.5 are structurally blocked (same root
//! cause, also cited, not re-derived). Monk is a confirmed permanent dead
//! end at level 13 (Diamond Soul needs spell resistance, which does not
//! exist anywhere in this codebase). Ranger was picked over Bard, Paladin,
//! Sorcerer, and Wizard because its level-15 "Special" column entry (4th
//! favored enemy) is a magnitude widening on the SAME already-grounded
//! Favored Enemy pillar used at the 1st/5th/10th-level intervals — the
//! seam's own doc comment on `RANGER_FAVORED_ENEMY_THIRD_INTERVAL_LEVEL`
//! explicitly named the 15th-level interval as the next widening target
//! ("Only the 10th-level interval is grounded here; the 15th/20th intervals
//! stay out of scope").
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte on the level-15 class table row: BAB
//! +15/+10/+5 (full-BAB progression), Fort +9, Ref +9, Will +5, Special "4th
//! favored enemy", base spells per day 3/2/2/1.
//!
//! - level 15 base attack bonus GENUINELY RISES to +15 (full BAB
//!   progression, up from +14 at level 14); good Fortitude and good Reflex
//!   BOTH STAY +9 (`15/2+2 = 9`, an integer-division coincidence with level
//!   14, re-verified rather than assumed); poor Will GENUINELY RISES to +5
//!   (`15/3 = 5`, up from +4 at level 14).
//! - the base spells-per-day table's level-15 row is `3/2/2/1`
//!   (1st/2nd/3rd/4th), verified independently against both primary
//!   sources: the 1st/2nd/4th-level columns stay 3/2/1 unchanged from level
//!   14, and the 3rd-level column GENUINELY RISES from 1 to 2 (a literal
//!   table lookup value, not a formula) — the spell-level access ladder
//!   itself stays at 4 (already widened at level 13; ranger spells never
//!   reach a 5th spell level at any level).
//! - the PF1 Core Rulebook Ranger class table's level-15 "Special" column
//!   reads only "4th favored enemy" (verified independently against both
//!   primary sources, byte-for-byte identical). The Favored Enemy rule's own
//!   text ("At 5th level and every five levels thereafter... the ranger may
//!   select an additional favored enemy. In addition, at each such
//!   interval, the bonus against any one favored enemy... increases by 2")
//!   is the EXACT structural mirror of the already-grounded 10th-level
//!   interval (`RANGER_FAVORED_ENEMY_THIRD_INTERVAL_LEVEL`) — not a new
//!   class feature. This slice grounds: a FOURTH favored-enemy TYPE
//!   selection (open-ended, mirroring the first/second/third favored
//!   enemies' own choice-recognition idiom exactly), a restricted
//!   four-option choice recognizing WHICH ONE of the four favored enemies is
//!   this interval's bonus-increase target (`enemy:first` / `enemy:second`
//!   / `enemy:third` / `enemy:fourth`, mirroring the second-interval's own
//!   restricted-choice idiom widened by one option), and the resulting flat
//!   magnitude increase applied only to whichever favored enemy the target
//!   choice actually names. Level 15 is NOT a Combat Style bonus-feat level
//!   (feats land at 2/6/10/14/18), so no fifth combat-style slot appears.
//!
//! It deliberately does not touch the favored-terrain/favored-enemy
//! conditional-application engines, any of the four combat-style bonus
//! feats' own mechanics, Hunter's Bond ally-bonus application or the
//! animal-companion form, Woodland Stride's/Swift Tracker's/Quarry's/
//! Camouflage's own application, or the ranger Wisdom prepared-posture/
//! spell-source-lineage burden (all stay named-but-unproven, unchanged from
//! levels 1-14), and it does not ground Ranger level 16+ or the Favored
//! Enemy rule's own 20th-level interval (stays out of scope). It also
//! preserves the accepted Ranger level-1..level-14 truth (unchanged), the
//! Fighter negative control, and the multiclass negative control.
//!
//! This slice also fixes five pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s,
//! `tests/sd18_ranger_level11_quarry.rs`'s,
//! `tests/sd18_ranger_level12_widening.rs`'s,
//! `tests/sd18_ranger_level13_widening.rs`'s, and
//! `tests/sd18_ranger_level14_widening.rs`'s own
//! `ranger_level_15_is_not_promoted_by_this_slice` (formerly named for level
//! 14/13/etc.), all moved to a level-16 boundary in the same commit,
//! mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/Rogue level-N-to-
//! level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level14_sd18_fourth_combat_style_feat_deterministic_input.txt"
);

const RANGER_LEVEL15_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level15_sd18_fourth_favored_enemy_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const FAVORED_ENEMY_4_CHOICE_ID: &str = "class_chassis.ranger.favored_enemy_4_choice";
const FAVORED_ENEMY_BONUS_INCREASE_3_CHOICE_ID: &str =
    "class_chassis.ranger.favored_enemy_bonus_increase_3_choice";
const FAVORED_ENEMY_4_SKILL_BONUS_ID: &str = "class_chassis.ranger.favored_enemy_4_skill_bonus";
const FAVORED_ENEMY_4_ATTACK_DAMAGE_BONUS_ID: &str =
    "class_chassis.ranger.favored_enemy_4_attack_damage_bonus";

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

// ----- Base attack bonus genuinely rises at level 15 -----

#[test]
fn ranger_level15_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 15,
        "Ranger level 15 full-BAB progression must equal 15, genuinely risen from 14: {}",
        base_attack.detail
    );
}

// ----- Good saves stay unchanged, poor Will genuinely rises -----

#[test]
fn ranger_level15_good_saves_stay_poor_will_genuinely_rises() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Ranger level 15 good Fortitude (15/2+2) must stay 9, an integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(reflex.value, 9, "Ranger level 15 good Reflex (15/2+2) must stay 9");

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(
        will.value, 5,
        "Ranger level 15 poor Will (15/3) must genuinely rise to 5, up from 4"
    );
}

// ----- Base spells per day widen at level 15: the 3rd-level column genuinely rises -----

#[test]
fn ranger_level15_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "level 15 (`3/2/2/1`): the 1st/2nd/4th-level columns stay 3/2/1 unchanged, and the \
         3rd-level column genuinely rises from 1 to 2"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn ranger_level15_spell_level_access_stays_four() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 15 spell-level access must stay 4 (already widened at level 13): {}",
        access.detail
    );
}

// ----- The fourth favored enemy is recognized as an open-ended type selection -----

#[test]
fn ranger_level15_fourth_favored_enemy_is_recognized_open_ended() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_ENEMY_4_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the fourth favored enemy selection must be a +0 recognition record only"
    );
    assert!(
        choice.detail.contains("construct"),
        "the recognition record must name the raw chosen enemy string: {}",
        choice.detail
    );
}

// ----- The 15th-level interval's own bonus-increase target is recognized -----

#[test]
fn ranger_level15_bonus_increase_target_names_the_fourth_favored_enemy() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_3_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the 15th-level-interval bonus-increase target selection must be a +0 recognition \
         record only"
    );
    assert!(
        choice.detail.contains("fourth favored enemy"),
        "the recognition record must name the fourth favored enemy as the target: {}",
        choice.detail
    );
}

// ----- The fourth favored enemy's own bonus genuinely grounds at +4 (targeted at its own interval) -----

#[test]
fn ranger_level15_fourth_favored_enemy_bonus_is_four_when_self_targeted() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let skill_bonus = explanation(&computation, FAVORED_ENEMY_4_SKILL_BONUS_ID);
    assert_eq!(
        skill_bonus.value, 4,
        "the fourth favored enemy skill bonus must be +4 (2 base + 2 from being targeted at \
         its own 15th-level interval): {}",
        skill_bonus.detail
    );

    let attack_bonus = explanation(&computation, FAVORED_ENEMY_4_ATTACK_DAMAGE_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 4,
        "the fourth favored enemy attack/damage bonus must be +4: {}",
        attack_bonus.detail
    );
}

// ----- Absent selections fabricate nothing -----

#[test]
fn ranger_level15_fourth_favored_enemy_absent_grounds_nothing() {
    let without_fourth = RANGER_LEVEL15_FIXTURE
        .replace("choice=choice:ranger_favored_enemy_4:enemy:construct\n", "")
        .replace(
            "choice=choice:ranger_favored_enemy_bonus_increase_target_3:enemy:fourth\n",
            "",
        );
    let input = load(&without_fourth);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_4_CHOICE_ID),
        "absent fourth favored enemy selection must fabricate no recognition record"
    );
    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_BONUS_INCREASE_3_CHOICE_ID),
        "absent 15th-level-interval bonus-increase target must fabricate no recognition record"
    );
    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_4_SKILL_BONUS_ID),
        "absent fourth favored enemy selection must fabricate no skill bonus"
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level15_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL15_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-15 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 14 truth is unchanged by this widening -----

#[test]
fn ranger_level14_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 14, "Ranger level 14 base attack bonus must stay 14");

    assert!(
        !has_explanation(&computation, FAVORED_ENEMY_4_CHOICE_ID),
        "level-14 Ranger must not gain any fourth-favored-enemy record: {:?}",
        computation.explanations
    );

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "Ranger level 14 base spells per day must stay `3/2/1/1`"
    );
}

// ----- Negative control removed: level 16 is now the supported/grounded row -----
// (formerly `ranger_level_16_is_not_promoted_by_this_slice`; retired by the SD18
// cycle-2026-07-15T6100 slice, which promotes level 16 for real —
// see tests/sd18_ranger_level16_improved_evasion.rs)

// ----- Negative control: the ranger path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_ranger_level15_recognition() {
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
fn multiclass_ranger_level15_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL15_FIXTURE.replace(
        "class_level=class:ranger:15",
        "class_level=class:ranger:15\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-15 widening -----

#[test]
fn matrix_ranger_row_names_level_15_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level15_widening"),
        "ranger row must cite the live SD18 level-15 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 15") || note.contains("level-15"),
        "ranger partial note must name the level-15 widening: {note}"
    );
}
