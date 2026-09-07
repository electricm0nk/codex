//! SD18 Ranger level-12 Camouflage widening grounding proof.
//!
//! Widens the accepted SD18 deterministic Human Ranger level-1..level-11
//! hybrid chassis (`tests/sd18_ranger_level11_quarry.rs`, the loop's most
//! recent Ranger ceiling) to Ranger level 12 — the twelfth SD-18 §3.2
//! class-row widening, mirroring the sibling-class level-range-gate idiom
//! (`supported_ranger_level` is generalized from `1..=11` to `1..=12` via
//! `MAX_SUPPORTED_RANGER_LEVEL = 12`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_MONK_LEVEL`,
//! `MAX_SUPPORTED_PALADIN_LEVEL`, and `MAX_SUPPORTED_ROGUE_LEVEL`, all from
//! 11 to 12). §3.1 race rows and §3.3 interaction rows are structurally
//! exhausted/blocked (cited in the progress doc, not re-derived this
//! cycle); §3.4/§3.5 are structurally blocked (same root cause, also
//! cited, not re-derived). Ranger was picked over the other two remaining
//! level-11 §3.2 rows (Sorcerer, Wizard) by the brief's own alphabetical
//! class-row priority order, and Wizard specifically carries a live
//! claim-blocker naming "spellbook content" as deliberately out of scope,
//! so Ranger is also the lower-risk pick this cycle.
//!
//! Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com mirror) were read directly before writing any code or test,
//! and both agree byte-for-byte:
//!
//! - level 12 base attack bonus GENUINELY RISES to +12 (full BAB
//!   progression, up from +11 at level 11; the table's own "+12/+7/+2"
//!   iterative notation is not modeled anywhere in this codebase, only the
//!   flat base value); ALL THREE base saves GENUINELY RISE too: Fortitude
//!   and Reflex to +8 (good, `12/2+2 = 8`, up from +7) and Will to +4
//!   (poor, `12/3 = 4`, up from +3) — unlike level 11, where all three
//!   saves stayed numerically unchanged from level 10. Track also
//!   genuinely rises to 6 (`max(12/2, 1) = 6`, up from 5), via the same
//!   pre-existing formula, no re-derivation.
//! - the PF1 Core Rulebook Ranger class table's level-12 "Special" column
//!   reads only "Camouflage" (verified independently against both primary
//!   sources, checked rather than assumed away) — a genuinely NEW named
//!   class feature. Camouflage's own rule text ("A ranger of 12th level or
//!   higher can use the Stealth skill to hide, even while being observed,
//!   as long as she is within any sort of natural terrain that grants at
//!   least partial concealment or partial cover") was genuinely assessed
//!   against the brief's Hard Stops before coding: it carries no numeric
//!   magnitude and no player choice of its own, so it is grounded here as
//!   a bounded grant-only identity record (value 0), mirroring the
//!   Woodland Stride/Swift Tracker idiom exactly — no terrain-detection
//!   engine and no Stealth-check-execution engine exists anywhere in this
//!   codebase, so only the grant itself is recorded.
//! - the base spells-per-day table's level-12 row is `2/2/1/—`
//!   (1st/2nd/3rd/4th), genuinely risen from level 11's `2/1/1/—` at the
//!   2nd-level column (1 -> 2), with the 1st/3rd-level columns staying
//!   2/1 unchanged and the 4th-level column staying inaccessible (no
//!   record; 4th-level ranger spells begin at level 13, outside this
//!   row's ceiling, checked rather than assumed away) — verified
//!   identically on both primary sources. The spell-level access ladder
//!   and the spell-save-DC/Wisdom-bonus formulas are unchanged (live
//!   arithmetic over the already-grounded access ladder, which itself
//!   stays at 3, unchanged from level 11).
//!
//! It deliberately does not touch the favored-enemy conditional-application
//! engine, either combat-style bonus feat's own mechanics, the Favored
//! Terrain level-13th/18th breadth, Hunter's Bond ally-bonus application or
//! the animal-companion form, Woodland Stride's/Swift Tracker's/Quarry's own
//! application, or the ranger prepared-posture/spell-source-lineage burden
//! (all stay named-but-unproven, unchanged from levels 1-11), and it does
//! not ground Ranger level 13+. It also preserves the accepted Ranger
//! level-1..level-11 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control.
//!
//! This slice also fixes a pre-existing stale sibling negative control that
//! this widening would otherwise have broken:
//! `tests/sd13_ranger_level10_progression.rs`'s
//! `ranger_level_12_is_not_promoted_by_this_slice` and
//! `tests/sd18_ranger_level11_quarry.rs`'s
//! `ranger_level_12_is_not_promoted_by_this_slice` both asserted that level
//! 12 stays claim-blocked; both are moved to a level-13 boundary in the same
//! commit, mirroring the Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/
//! Rogue level-11-to-level-12 sibling-fix precedent exactly.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const RANGER_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level11_sd18_quarry_deterministic_input.txt"
);

const RANGER_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level12_sd18_camouflage_deterministic_input.txt"
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

// ----- Base attack bonus genuinely rises at level 12 -----

#[test]
fn ranger_level12_base_attack_bonus_genuinely_rises() {
    let input = load(RANGER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(
        base_attack.value, 12,
        "Ranger level 12 full-BAB progression must equal 12, genuinely risen from 11: {}",
        base_attack.detail
    );
}

// ----- Base saves and Track at level 12 all genuinely rise -----

#[test]
fn ranger_level12_base_saves_and_track_genuinely_rise() {
    let input = load(RANGER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.ranger.base_save.fortitude");
    assert_eq!(
        fortitude.value, 8,
        "Ranger level 12 good Fortitude (12/2+2) must equal 8 — genuinely risen from 7"
    );

    let reflex = explanation(&computation, "class_chassis.ranger.base_save.reflex");
    assert_eq!(reflex.value, 8, "Ranger level 12 good Reflex (12/2+2) must equal 8");

    let will = explanation(&computation, "class_chassis.ranger.base_save.will");
    assert_eq!(will.value, 4, "Ranger level 12 poor Will (12/3) must equal 4");

    let track = explanation(&computation, "class_chassis.ranger.track");
    assert_eq!(
        track.value, 6,
        "Ranger level 12 Track (max(12/2, 1)) must equal 6 — genuinely risen from 5"
    );
}

// ----- Base spells per day widen at level 12 -----

#[test]
fn ranger_level12_base_spells_per_day_match_the_raw_table_row() {
    let input = load(RANGER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
        ],
        "level 12 (`2/2/1`): the 2nd-level column rises from 1 to 2; the 1st/3rd-level columns \
         stay 2/1 unchanged and the 4th-level column stays inaccessible (no record)"
    );
}

// ----- Camouflage is newly grounded at level 12 as a grant-only identity record -----

#[test]
fn ranger_level12_grounds_camouflage_as_a_grant_only_identity_record() {
    let input = load(RANGER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let camouflage = explanation(&computation, "class_feature.ranger.camouflage");
    assert_eq!(
        camouflage.value, 0,
        "Camouflage's grant-only identity record must carry no fabricated mechanical value"
    );
    assert!(
        camouflage.detail.to_lowercase().contains("stealth"),
        "Camouflage's grant-only record must name the Stealth-while-observed behavior: {}",
        camouflage.detail
    );
}

// ----- The bounded Ranger computation stays claim-blocked overall -----

#[test]
fn ranger_level12_still_claim_blocks_overall() {
    let input = load(RANGER_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-12 Ranger must still be claim-blocked overall: {:?}",
        computation.diagnostics
    );
}

// ----- Negative control: level 11 truth is unchanged by this widening -----

#[test]
fn ranger_level11_truth_is_unchanged_by_this_slice() {
    let input = load(RANGER_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.ranger.base_attack_bonus");
    assert_eq!(base_attack.value, 11, "Ranger level 11 base attack bonus must stay 11");

    let camouflage_absence = explanation(&computation, "class_feature.ranger.camouflage");
    assert_eq!(
        camouflage_absence.value, 0,
        "Ranger level 11 Camouflage must be a correct level-gate absence"
    );
    assert!(
        camouflage_absence.detail.contains("absent"),
        "Ranger level 11 Camouflage record must describe the level-gate absence: {}",
        camouflage_absence.detail
    );

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 2),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 1),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 1),
        ],
        "Ranger level 11 base spells per day must stay `2/1/1`"
    );
}

// ----- Negative control: level 16 stays unrecognized by this slice -----
//
// A later SD18 widening (cycle-2026-07-15T1400,
// tests/sd18_ranger_level13_widening.rs) now genuinely recognizes Ranger
// level 13 too (base attack rises, the third favored terrain and the
// spell-level access ladder's 4th column are newly grounded), so this
// boundary control moved from level 13 to level 14. A still later SD18
// widening (cycle-2026-07-15T2100, tests/sd18_ranger_level14_widening.rs)
// now genuinely recognizes Ranger level 14 too, so this boundary control
// moved once more to level 15. A still later SD18 widening
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
    let level_21 = RANGER_LEVEL12_FIXTURE.replace("class:ranger:12", "class:ranger:21");
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
fn fighter_does_not_gain_ranger_level12_recognition() {
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
fn multiclass_ranger_level12_is_not_promoted_by_this_slice() {
    let multiclass = RANGER_LEVEL12_FIXTURE.replace(
        "class_level=class:ranger:12",
        "class_level=class:ranger:12\nclass_level=class:fighter:1",
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

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_ranger_row_names_level_12_widening() {
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
        ranger.grounding_ref.contains("sd18_ranger_level12_widening"),
        "ranger row must cite the live SD18 level-12 proof surface: {}",
        ranger.grounding_ref
    );
    let note = ranger.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "ranger partial note must name the level-12 widening: {note}"
    );
}
