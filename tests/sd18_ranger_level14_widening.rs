//! SD18 Ranger level-14 fourth-combat-style-bonus-feat widening grounding
//! proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-13
//! hybrid chassis (`tests/sd18_ranger_level13_widening.rs`, the loop's most
//! recent Ranger ceiling) to Ranger level 14 — the loop's FOURTH §3.2
//! class-row level-14 landing, after Barbarian, Fighter, and Rogue
//! (`supported_ranger_level` is generalized from `1..=13` to `1..=14` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 14`, exactly as prior cycles widened the
//! sibling `MAX_SUPPORTED_*_LEVEL` constants). §3.1 race rows and §3.3
//! interaction rows are structurally exhausted/blocked (cited in the
//! progress doc, not re-derived this cycle); §3.4/§3.5 are structurally
//! blocked (same root cause, also cited, not re-derived). Monk is a
//! confirmed dead end at level 14 (Diamond Soul needs spell resistance,
//! which does not exist anywhere in this codebase); Barbarian, Fighter, and
//! Rogue are already landed at level 14. Ranger was picked per the prior
//! cycle's own "Next cycle instructions": its level-14 "Special" column
//! entry is a fourth Combat Style bonus feat, mirroring the existing
//! `RANGER_COMBAT_STYLE_BONUS_FEAT_3_LEVEL` precedent.
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! cross-checked against a third mirror (legacy.aonprd.com), and all three
//! agree byte-for-byte on the level-14 class table row: BAB +14/+9/+4
//! (full-BAB progression), Fort +9, Ref +9, Will +4, Special "Combat style
//! feat", base spells per day 3/2/1/1.
//!
//! - level 14 base attack bonus GENUINELY RISES to +14 (full BAB
//!   progression, up from +13 at level 13); good Fortitude and good Reflex
//!   BOTH GENUINELY RISE to +9 (`14/2+2 = 9`, up from 8 at level 13); poor
//!   Will STAYS +4 (`14/3 = 4`), an integer-division coincidence,
//!   re-verified rather than assumed.
//! - the base spells-per-day table's level-14 row is `3/2/1/1`
//!   (1st/2nd/3rd/4th), verified independently against all three primary
//!   sources: the 1st/2nd/3rd-level columns stay 3/2/1 unchanged from level
//!   13, and the 4th-level column GENUINELY RISES from 0 to 1 (a literal
//!   table lookup value, not a formula) — the spell-level access ladder
//!   itself stays at 4 (already widened at level 13; ranger spells never
//!   reach a 5th spell level at any level).
//! - the PF1 Core Rulebook Ranger class table's level-14 "Special" column
//!   reads only "Combat style feat" (verified independently against all
//!   three primary sources). This is the ranger's FOURTH combat-style bonus
//!   feat (bonus feats land at 2nd, 6th, 10th, 14th, and 18th ranger level,
//!   per the class feature's own rule text, re-verified this cycle). Unlike
//!   the 2nd/6th/10th-level grants, the PF1 Core Rulebook's own Ranger
//!   Combat Styles tables (Archery, Two-Weapon Combat) do not tabulate any
//!   NEW named feat options at the 14th-level tier — verified independently
//!   against THREE sources dedicated to the combat-style feat lists
//!   specifically (d20pfsrd's Ranger Combat Styles page, the Archives of
//!   Nethys aonprd.com RangerCombatStyles page, and a Paizo rules-forum
//!   thread addressing the same question directly): all three agree the
//!   printed Core Rulebook list of named options stops after the 10th-level
//!   tier; later sourcebooks (e.g. the Advanced Player's Guide) are the ones
//!   that add named 14th/18th-level options, and those sourcebooks are
//!   outside SD-18's Core-Rulebook-only scope (per the scope doc's §5
//!   non-goals). Because no Core-Rulebook-sourced restricted feat-name list
//!   exists to validate against at this tier, this slice does NOT mirror the
//!   closed-restricted-list idiom used for feats 1-3
//!   (`combat_style_bonus_feat_choice` / `_2_choice` / `_3_choice`).
//!   Instead it grounds the FOURTH bonus-feat SLOT as an open-ended +0
//!   recognition record, mirroring the Favored Terrain/Quarry
//!   choice-recognition idiom exactly (raw string interpolation, no
//!   restricted-list validation, no feat mechanics computed) — the slot's
//!   genuine existence at 14th ranger level is grounded honestly, without
//!   fabricating a restricted list the corpus does not contain.
//!
//! It deliberately does not touch the favored-terrain/favored-enemy
//! conditional-application engines, either of the first three combat-style
//! bonus feats' own mechanics, the newly-recognized fourth bonus feat's own
//! mechanics, Hunter's Bond ally-bonus application or the animal-companion
//! form, Woodland Stride's/Swift Tracker's/Quarry's/Camouflage's own
//! application, or the ranger Wisdom prepared-posture/spell-source-lineage
//! burden (all stay named-but-unproven, unchanged from levels 1-13), and it
//! does not ground Ranger level 15+ or the Favored Terrain/Favored Enemy
//! rules' own next intervals (both land beyond level 14). It also preserves
//! the accepted Ranger level-1..level-13 truth (unchanged), the Fighter
//! negative control, and the multiclass negative control.
//!
//! This slice also fixes four pre-existing stale sibling negative controls
//! that this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s,
//! `tests/sd18_ranger_level11_quarry.rs`'s,
//! `tests/sd18_ranger_level12_widening.rs`'s, and
//! `tests/sd18_ranger_level13_widening.rs`'s own
//! `ranger_level_14_is_not_promoted_by_this_slice`, all moved to a level-15
//! boundary in the same commit, mirroring the
//! Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/Rogue level-N-to-
//! level-(N+1) sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const RANGER_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level13_sd18_third_favored_terrain_deterministic_input.txt"
);

const RANGER_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level14_sd18_fourth_combat_style_feat_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

const COMBAT_STYLE_BONUS_FEAT_4_CHOICE_ID: &str =
    "class_chassis.ranger.combat_style_bonus_feat_4_choice";

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

// ----- Base attack bonus genuinely rises at level 14 -----

#[test]
fn ranger_level14_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 14,
        "Ranger level 14 full-BAB progression must equal 14, genuinely risen from 13: {}",
        base_attack.detail
    );
}

// ----- Good saves genuinely rise, poor Will stays unchanged -----

#[test]
fn ranger_level14_good_saves_genuinely_rise_poor_will_stays() {
    let input = load(RANGER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 9,
        "Ranger level 14 good Fortitude (14/2+2) must genuinely rise to 9, up from 8"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(reflex.value, 9, "Ranger level 14 good Reflex (14/2+2) must genuinely rise to 9");

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(
        will.value, 4,
        "Ranger level 14 poor Will (14/3) must stay 4, an integer-division coincidence"
    );
}

// ----- Base spells per day widen at level 14: the 4th-level column genuinely rises -----

#[test]
fn ranger_level14_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 1),
        ],
        "level 14 (`3/2/1/1`): the 1st/2nd/3rd-level columns stay 3/2/1 unchanged, and the \
         4th-level column genuinely rises from 0 to 1"
    );
}

// ----- The spell-level access ladder stays at 4 (already widened at level 13) -----

#[test]
fn ranger_level14_spell_level_access_stays_four() {
    let input = load(RANGER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.ranger.partial_caster.spell_level_access",
    );
    assert_eq!(
        access.value, 4,
        "Ranger level 14 spell-level access must stay 4 (already widened at level 13): {}",
        access.detail
    );
}

// ----- The fourth combat-style bonus feat is recognized as an open-ended slot -----

#[test]
fn ranger_level14_fourth_combat_style_bonus_feat_is_recognized_open_ended() {
    let input = load(RANGER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, COMBAT_STYLE_BONUS_FEAT_4_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the fourth combat-style bonus feat slot must be a +0 recognition record only"
    );
    assert!(
        choice.detail.contains("improved_precise_shot"),
        "the recognition record must name the raw chosen feat string: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("14th")
            && (choice.detail.contains("does not") || choice.detail.contains("no")),
        "the recognition record must explain that no CRB-sourced restricted list exists at \
         this tier: {}",
        choice.detail
    );
}

#[test]
fn ranger_level14_fourth_combat_style_bonus_feat_absent_grounds_nothing() {
    let without_feat = RANGER_LEVEL14_FIXTURE.replace(
        "choice=choice:ranger_combat_style_bonus_feat_4:feat:improved_precise_shot\n",
        "",
    );
    let input = load(&without_feat);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_4_CHOICE_ID),
        "absent fourth combat-style bonus feat selection must fabricate no recognition record"
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level14_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-14 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 13 truth is unchanged by this widening -----

#[test]
fn ranger_level13_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 13, "Ranger level 13 base attack bonus must stay 13");

    assert!(
        !has_explanation(&computation, COMBAT_STYLE_BONUS_FEAT_4_CHOICE_ID),
        "level-13 Ranger must not gain any fourth-combat-style-bonus-feat record: {:?}",
        computation.explanations
    );

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 3),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 0),
        ],
        "Ranger level 13 base spells per day must stay `3/2/1/0`"
    );
}

// ----- Negative control: level 16 stays unrecognized by this slice -----
//
// A later SD18 widening (cycle-2026-07-15T4000,
// tests/sd18_ranger_level15_widening.rs) now genuinely recognizes Ranger
// level 15 too (base attack rises, poor Will rises, the fourth favored
// enemy and the base spells-per-day table's 3rd-level column are newly
// grounded), so this boundary control moved from level 15 to level 16, and a
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
    let level_21 = RANGER_LEVEL14_FIXTURE.replace("class:ranger:14", "class:ranger:21");
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
fn fighter_does_not_gain_ranger_level14_recognition() {
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
fn multiclass_ranger_level14_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL14_FIXTURE.replace(
        "class_level=class:ranger:14",
        "class_level=class:ranger:14\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_ranger_row_names_level_14_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level14_widening"),
        "ranger row must cite the live SD18 level-14 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "ranger partial note must name the level-14 widening: {note}"
    );
}
