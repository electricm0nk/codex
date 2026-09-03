//! SD18 Ranger level-11 Quarry widening grounding proof.
//!
//! Widens the accepted deterministic Human Ranger level-1..level-10 hybrid
//! chassis (`tests/sd13_ranger_level10_progression.rs`, the SD13 tranche's
//! declared ceiling) to Ranger level 11 — the eleventh SD-18 §3.2 class-row
//! widening, mirroring the sibling-class level-range-gate idiom
//! (`supported_ranger_level` is generalized from `1..=10` to `1..=11` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 11`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_MONK_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, `MAX_SUPPORTED_ROGUE_LEVEL`, and
//! `MAX_SUPPORTED_SORCERER_LEVEL`/`MAX_SUPPORTED_WIZARD_LEVEL`, all from 10
//! to 11). §3.1 race rows and §3.3 interaction rows were re-checked live
//! this cycle per priority order and remain, respectively, fully exhausted
//! and non-advanceable (no class row yet branches its compute path on a
//! specific non-Human race identity), and Ranger was the last fully-
//! untouched §3.2 class row (Barbarian, Bard, Cleric, Druid, Fighter, Monk,
//! Paladin, Rogue, Sorcerer, and Wizard all already have their own level-11
//! widening landed).
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte:
//!
//! - level 11 base attack bonus GENUINELY RISES to +11 (full BAB
//!   progression, up from +10 at level 10; the table's own "+11/+6/+1"
//!   iterative notation is not modeled anywhere in this codebase, only the
//!   flat base value) while base saves stay +7 Fortitude, +7 Reflex (good,
//!   `11/2+2 = 7`), and +3 Will (poor, `11/3 = 3`) — all three numerically
//!   IDENTICAL to level 10, integer-division coincidences, confirmed by the
//!   same formulas already grounded at levels 1-10, not re-derived. Track
//!   also stays 5 (`max(11/2, 1) = 5`), an integer-division coincidence.
//! - the PF1 Core Rulebook Ranger class table's level-11 "Special" column
//!   reads only "Quarry" (verified independently against both primary
//!   sources, checked rather than assumed away) — a genuinely NEW named
//!   class feature, unlike the sibling classes' level-11 rows, which were
//!   each either a magnitude-rise on an already-grounded flat pillar or a
//!   bloodline-specific/blank entry left unproven. Quarry's own rule text
//!   ("a ranger can select one target within line of sight as his quarry...
//!   take 10 on his Survival skill checks while moving at normal speed
//!   without penalty... a +2 insight bonus on attack rolls made against his
//!   quarry... confirms all critical threats against the quarry
//!   automatically") was genuinely assessed against the brief's Hard Stops
//!   before coding anything: it is grounded here as a bundle mirroring
//!   precedent exactly, NOT as a new subsystem —
//!     * the take-10-while-tracking and auto-confirm-critical-threats
//!       behaviors are grant-only identity records (value 0), mirroring the
//!       Woodland Stride/Swift Tracker idiom exactly: no
//!       Survival-check-execution engine and no critical-confirmation-roll
//!       engine exists anywhere in this codebase, so only the grant itself
//!       is recorded;
//!     * the quarry target (when present in chosen input) is an open-ended
//!       +0 recognition record, mirroring the Favored Enemy/Favored Terrain
//!       choice-recognition idiom exactly (raw string interpolation, no
//!       restricted-list validation, no matching against the ranger's own
//!       favored-enemy types — nothing is fabricated when the choice is
//!       absent); and
//!     * the rule's own flat +2 insight attack-roll bonus is a standalone,
//!       non-applied magnitude, mirroring the Favored Enemy
//!       attack/damage-bonus idiom exactly: no target-selection engine and
//!       no conditional-application engine decides whether any specific
//!       attack is actually made against the quarry.
//!
//!   No active-quarry state (the 24-hour reselection cooldown, the 1-hour
//!   post-kill cooldown, or "only one quarry at a time") is tracked.
//! - the Spells per Day table's level-11 row is `2/1/1/—` (1st/2nd/3rd/4th),
//!   genuinely risen from level 10's `2/1/0/—` at the 3rd spell level
//!   (0 -> 1), with the 4th-level column staying "—" (4th-level ranger
//!   spells begin at level 13, outside this row's ceiling, checked rather
//!   than assumed away) — verified identically on both primary sources. The
//!   spell-level access ladder and the spell-save-DC/Wisdom-bonus formulas
//!   are unchanged (live arithmetic over the already-grounded access
//!   ladder, which itself stays at 3, unchanged from level 10).
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, either combat-style bonus feat's own mechanics, the Favored
//! Terrain level-13th/18th breadth, Hunter's Bond ally-bonus application or
//! the animal-companion form, Woodland Stride's/Swift Tracker's own
//! application, or the ranger prepared-posture/spell-source-lineage burden
//! (all stay named-but-unproven, unchanged from levels 1-10), and it does
//! not ground Ranger level 12+. It also preserves the accepted Ranger
//! level-1..level-10 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const RANGER_LEVEL10_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level10_sd13_deterministic_input.txt");

const RANGER_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level11_sd18_quarry_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.ranger.partial_caster.base_spells_per_day.";

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

// ----- Base attack bonus genuinely rises at level 11 -----

#[test]
fn ranger_level11_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 11,
        "Ranger level 11 full-BAB progression must equal 11, genuinely risen from 10: {}",
        base_attack.detail
    );
}

// ----- Base saves and Track at level 11 stay numerically unchanged -----

#[test]
fn ranger_level11_base_saves_and_track_are_grounded_and_unchanged() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 7,
        "Ranger level 11 good Fortitude (11/2+2) must equal 7 — unchanged from level 10"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(reflex.value, 7, "Ranger level 11 good Reflex (11/2+2) must equal 7");

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(will.value, 3, "Ranger level 11 poor Will (11/3) must equal 3");

    let track = explanation(&computation, "class_chassis.ranger.track");
    assert_eq!(
        track.value, 5,
        "Ranger level 11 Track (max(11/2, 1)) must equal 5 — unchanged from level 10"
    );
}

// ----- Base spells per day widen at level 11 -----

#[test]
fn ranger_level11_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
        ],
        "level 11 (`2/1/1`): the 3rd-level column rises from 0 to 1; the 1st/2nd-level columns \
         stay 2/1 unchanged and the 4th-level column stays inaccessible (no record)"
    );
}

// ----- Quarry is newly grounded at level 11 -----

#[test]
fn ranger_level11_grounds_quarry_as_a_grant_only_identity_record() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let quarry = explanation(&computation, "class_feature.ranger.quarry");
    assert_eq!(
        quarry.value, 0,
        "Quarry's grant-only identity record must carry no fabricated mechanical value"
    );
    assert!(
        quarry.detail.contains("critical")
            && quarry.detail.to_lowercase().contains("survival"),
        "Quarry's grant-only record must name both the auto-confirm-critical-threats and \
         take-10-while-tracking behaviors: {}",
        quarry.detail
    );
}

#[test]
fn ranger_level11_grounds_the_quarry_target_choice() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.ranger.quarry_choice");
    assert_eq!(
        choice.value, 0,
        "the quarry target choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("humanoid_orc"),
        "the quarry target choice recognition must name the chosen target: {}",
        choice.detail
    );
}

#[test]
fn ranger_level11_grounds_the_quarry_attack_bonus() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let attack_bonus = explanation(&computation, "class_chassis.ranger.quarry_attack_bonus");
    assert_eq!(
        attack_bonus.value, 2,
        "Quarry's flat insight attack-roll bonus must equal +2 (PF1 Core Rulebook): {}",
        attack_bonus.detail
    );
}

// ----- No target-type matching or conditional application is fabricated -----

#[test]
fn ranger_level11_does_not_fabricate_quarry_conditional_application() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("quarry") && e.id.contains("cooldown")),
        "level-11 Ranger must not fabricate any quarry cooldown-state record: {:?}",
        computation.explanations
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level11_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Ranger is not a Fighter, so the generic Fighter-shaped chassis path
    // (`compute_fighter_chassis` / `supported_fighter_level`) still emits its
    // own claim-blocking diagnostic at level 11, exactly as it did at every
    // prior Ranger level: this slice widens the per-pillar decomposition
    // records, not the generic claim-blocking posture.
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-11 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 10 truth is unchanged by this widening -----

#[test]
fn ranger_level10_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL10_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 10, "Ranger level 10 base attack bonus must stay 10");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.ranger.quarry" && e.value != 0),
        "Ranger level 10 must not fabricate a nonzero Quarry record: {:?}",
        computation.explanations
    );

    let quarry_absence = explanation(&computation, "class_feature.ranger.quarry");
    assert_eq!(
        quarry_absence.value, 0,
        "Ranger level 10 Quarry must be a correct level-gate absence"
    );
    assert!(
        quarry_absence.detail.contains("absent"),
        "Ranger level 10 Quarry record must describe the level-gate absence: {}",
        quarry_absence.detail
    );

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 0),
        ],
        "Ranger level 10 base spells per day must stay `2/1/0`"
    );
}

// ----- Negative control: level 15 stays unrecognized by this cycle -----
//
// SD18 widening (cycle-2026-07-15T0900, tests/sd18_ranger_level12_widening.rs):
// Ranger level 12 is now genuinely recognized (base attack/saves/Track all
// rise and Camouflage is newly grounded), so this boundary control moved to
// level 13, mirroring the exact same boundary move each of the sibling
// classes' own level-11-to-level-12 widening cycles made for their own
// level-11 progression tests. A still further SD18 widening
// (cycle-2026-07-15T1400, tests/sd18_ranger_level13_widening.rs) now
// genuinely recognizes Ranger level 13 too, so this boundary control moved
// once more to level 14. A still further SD18 widening
// (cycle-2026-07-15T2100, tests/sd18_ranger_level14_widening.rs) now
// genuinely recognizes Ranger level 14 too, so this boundary control moved
// once more to level 15. A still further SD18 widening
// (cycle-2026-07-15T4000, tests/sd18_ranger_level15_widening.rs) now
// genuinely recognizes Ranger level 15 too, so this boundary control moved
// once more to level 16, and a still further SD18 widening
// (cycle-2026-07-15T6100, tests/sd18_ranger_level16_improved_evasion.rs) now
// genuinely recognizes Ranger level 16 too, and a still further SD18
// widening (cycle-2026-07-15T7000,
// tests/sd18_ranger_level17_hide_in_plain_sight.rs) now genuinely recognizes
// Ranger level 17 too, and a still further SD18 widening
// (cycle-2026-07-16T0244, tests/sd18_ranger_level18_widening.rs) now
// genuinely recognizes Ranger level 18 too, and a still further SD18
// widening (cycle-2026-07-16T3200, tests/sd18_ranger_level19_widening.rs)
// now genuinely recognizes Ranger level 19 too, and a still further SD18
// widening (cycle-2026-07-16T1600, tests/sd18_ranger_level20_widening.rs)
// now genuinely recognizes Ranger level 20 too, so this boundary control
// moves once more to level 21 (a pure implementation-gate check, since PF1
// has no 21st character level).
#[test]
fn ranger_level_21_is_not_promoted_by_this_slice() {
    let level_21 = RANGER_LEVEL11_FIXTURE.replace("class:ranger:11", "class:ranger:21");
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
fn fighter_does_not_gain_ranger_level11_recognition() {
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
fn multiclass_ranger_level11_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL11_FIXTURE.replace(
        "class_level=class:ranger:11",
        "class_level=class:ranger:11\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-11 widening -----

#[test]
fn matrix_ranger_row_names_level_11_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level11_quarry"),
        "ranger row must cite the live SD18 level-11 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 11") || note.contains("level-11"),
        "ranger partial note must name the level-11 widening: {note}"
    );
}
