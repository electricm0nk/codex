//! SD18 Rogue level-12 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-11 chassis
//! (`tests/sd18_rogue_level11_sneak_attack.rs`, the loop's most recent
//! Rogue ceiling) to Rogue level 12 -- mirroring the sibling-class
//! level-range-gate idiom (`supported_rogue_level` is generalized from
//! `1..=11` to `1..=12` via `MAX_SUPPORTED_ROGUE_LEVEL = 12`, exactly as
//! the Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin level-12 widening
//! cycles each widened their own `MAX_SUPPORTED_<CLASS>_LEVEL` from 11 to
//! 12). Both PF1 CRB primary sources (d20pfsrd and the Archives of Nethys
//! aonprd.com Rogue class table) were read directly before writing any code
//! or test, and both agree byte-for-byte:
//!
//! - level 12 base attack bonus is +9 (`12 * 3 / 4 = 9`, the Rogue's
//!   3/4-BAB progression, genuinely risen from +8 at level 11) and base
//!   saves genuinely rise too: Fortitude +4 (`12 / 3 = 4`, up from +3),
//!   Reflex +8 (`12 / 2 + 2 = 8`, up from +7), and Will +4 (`12 / 3 = 4`,
//!   up from +3) -- all four checked directly against both primary sources,
//!   not assumed, unlike level 11's coincidental save plateau.
//! - the PF1 Core Rulebook Rogue class table's level-12 "Special" column
//!   reads "Rogue talent, trap sense +4" (both primary sources agree). Trap
//!   Sense genuinely rises to +4 via the pre-existing flat-magnitude
//!   formula (`level / 3`), up from +3 at level 11, not a new record. Rogue
//!   Talent is the sixth numbered choice-recognition slot (talents land at
//!   2/4/6/8/10/12), surfaced via the same open-ended, non-validated
//!   raw-string idiom already proven at slots 1-5
//!   (`choice:rogue_talent_6` -> `class_chassis.rogue.talent_6_choice`):
//!   the selected talent's own effect stays unproven, exactly as slots 1-5
//!   left their own effects unproven -- no talent-effect engine exists in
//!   this codebase.
//! - the sneak-attack die count stays 6d6 (`(12 + 1) / 2 = 6`, an
//!   integer-division coincidence with level 11 -- the odd-level rise
//!   cadence, next rise at level 13) but Trapfinding genuinely rises to +6
//!   (`max(12 / 2, 1) = 6`, up from +5 at level 11, via the pre-existing
//!   formula, not a new record -- this rise is NOT named in the level-12
//!   "Special" column, since Trapfinding's own formula is independent of
//!   the class-table Special column and simply keeps climbing); Evasion,
//!   Uncanny Dodge, and Improved Uncanny Dodge all stay granted, not
//!   re-derived.
//!
//! It deliberately does not touch the rogue-talent tree's own effects
//! (standard or advanced, including the newly-surfaced slot 6), any
//! check-execution engine, or sneak-attack damage application (all stay
//! named-but-unproven, unchanged from levels 1-11), and it does not ground
//! Rogue level 13+. It also preserves the accepted Rogue level-1..level-11
//! truth (unchanged), the Fighter negative control, and the multiclass
//! negative control.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const ROGUE_LEVEL11_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level11_sd18_sneak_attack_deterministic_input.txt"
);

const ROGUE_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level12_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";

// ----- Base attack bonus and base saves genuinely rise at level 12 -----

#[test]
fn rogue_level12_base_attack_bonus_and_saves_genuinely_rise() {
    let input = load(ROGUE_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 9,
        "Rogue level 12 3/4-BAB progression (12 * 3 / 4) must genuinely rise to 9, up from 8 \
         at level 11: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Rogue level 12 poor Fortitude (12/3) must genuinely rise to 4, up from 3 at level 11"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 8,
        "Rogue level 12 good Reflex (12/2+2) must genuinely rise to 8, up from 7 at level 11"
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 4,
        "Rogue level 12 poor Will (12/3) must genuinely rise to 4, up from 3 at level 11"
    );
}

// ----- Trap Sense genuinely rises to +4 (matches the level-12 Special column) -----

#[test]
fn rogue_level12_trap_sense_genuinely_rises() {
    let input = load(ROGUE_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 4,
        "Rogue level 12 Trap Sense (12 / 3) must genuinely rise to +4, up from +3 at level 11, \
         per the PF1 Core Rulebook Rogue class table's level-12 'Special' column ('Rogue \
         talent, trap sense +4'): {}",
        trap_sense.detail
    );
}

// ----- Sneak attack stays 6d6 but Trapfinding genuinely rises to +6 -----

#[test]
fn rogue_level12_sneak_attack_stays_and_trapfinding_rises() {
    let input = load(ROGUE_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 6,
        "Rogue level 12 sneak attack die count ((12 + 1) / 2) must stay 6 (6d6), an \
         integer-division coincidence with level 11 -- the next rise lands at level 13: {}",
        sneak_attack.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 6,
        "Rogue level 12 Trapfinding (max(12/2, 1)) must genuinely rise to 6, up from 5 at \
         level 11, via the pre-existing formula: {}",
        trapfinding.detail
    );
}

// ----- The sixth numbered rogue-talent choice slot is surfaced -----

#[test]
fn rogue_level12_surfaces_the_sixth_talent_choice_slot() {
    let input = load(ROGUE_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot = explanation(&computation, "class_chassis.rogue.talent_6_choice");
    assert_eq!(
        slot.value, 0,
        "the sixth rogue-talent choice slot names the selection only and contributes no \
         computed mechanical value: {slot:?}"
    );
    assert!(
        slot.detail.contains("combat_trick"),
        "level-12 sixth talent slot must name the chosen raw talent string: {}",
        slot.detail
    );
    assert!(
        slot.detail.contains("no talent-effect engine") || slot.detail.contains("not computed"),
        "level-12 sixth talent slot must state the talent's own effect stays unproven: {}",
        slot.detail
    );

    // Earlier numbered slots are not fabricated by this fixture (it selects
    // only the sixth slot, mirroring how the level-11 fixture carries no
    // talent selections at all).
    for earlier in [
        "class_chassis.rogue.talent_choice",
        "class_chassis.rogue.talent_2_choice",
        "class_chassis.rogue.talent_3_choice",
        "class_chassis.rogue.talent_4_choice",
        "class_chassis.rogue.talent_5_choice",
    ] {
        assert!(
            !computation.explanations.iter().any(|e| e.id == earlier),
            "the level-12 widening fixture carries no selection for '{earlier}', so it must \
             not be fabricated: {:?}",
            computation.explanations
        );
    }
}

// ----- Granted features stay granted at level 12 -----

#[test]
fn rogue_level12_still_recognizes_the_granted_feature_records() {
    let input = load(ROGUE_LEVEL12_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 12"
        );
    }
}

// ----- Without a slot-6 selection, no slot-6 record is fabricated -----

#[test]
fn rogue_level12_without_selection_fabricates_no_slot_6_record() {
    let no_selection = ROGUE_LEVEL12_FIXTURE
        .lines()
        .filter(|line| !line.starts_with("choice=choice:rogue_talent_6"))
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&no_selection);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_6_choice"),
        "without a slot-6 selection, no slot-6 record may be fabricated: {:?}",
        computation.explanations
    );
    // The rest of the level-12 chassis still genuinely applies.
    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 9, "level 12 base attack bonus must still apply");
}

// ----- Negative control: the level-11 fixture is unaffected by this widening -----

#[test]
fn rogue_level11_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL11_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 8, "Rogue level 11 base attack bonus must stay 8");

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(trap_sense.value, 3, "Rogue level 11 Trap Sense must stay +3");

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(trapfinding.value, 5, "Rogue level 11 Trapfinding must stay 5");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_6_choice"),
        "level-11 Rogue must not gain the sixth talent choice slot"
    );
}

// ----- Negative control: level 18 stays claim-blocked (beyond the bounded L1-17 row) -----
//
// SD18 widening (cycle-2026-07-15T1100, tests/sd18_rogue_level13_widening.rs)
// now genuinely recognizes level 13, so this boundary control moved to
// level 14. A further SD18 widening (cycle-2026-07-15T2000,
// tests/sd18_rogue_level14_widening.rs) now genuinely recognizes level 14
// too, so this boundary control moved again, to level 16, per SD18
// cycle-2026-07-15T2900 (tests/sd18_rogue_level15_widening.rs). A further
// SD18 widening (cycle-2026-07-15T5200, tests/sd18_rogue_level16_widening.rs)
// now genuinely recognizes level 16 too, so this boundary control moved
// again, to level 17. A further SD18 widening (cycle-2026-07-15T8100,
// tests/sd18_rogue_level17_widening.rs) now genuinely recognizes level 17
// too, so this boundary control moved again, to level 18. A further SD18
// widening (cycle-2026-07-16T0212, tests/sd18_rogue_level18_widening.rs)
// now genuinely recognizes level 18 too, so this boundary control moved
// again, to level 19. A further SD18 widening (cycle-2026-07-16T3600,
// tests/sd18_rogue_level19_widening.rs) now genuinely recognizes level 19
// too, so this boundary control moved again, to level 20. A further SD18
// widening (cycle-2026-07-16T1431, tests/sd18_rogue_level20_widening.rs)
// now genuinely recognizes level 20 too, so this boundary control moves
// again, to level 21 (PF1 has no 21st character level; this is a pure
// implementation-gate check).

#[test]
fn rogue_level_21_stays_claim_blocked() {
    let level_21 = ROGUE_LEVEL12_FIXTURE.replace("class:rogue:12", "class:rogue:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-21 Rogue must stay claim-blocked beyond the bounded levels-1-20 row: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.base_attack_bonus"),
        "level-21 Rogue must not fabricate a base-attack-bonus explanation"
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level12_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "the Fighter chassis must not surface any rogue-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Rogue is not promoted -----

#[test]
fn multiclass_rogue_level12_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL12_FIXTURE.replace(
        "class_level=class:rogue:12",
        "class_level=class:rogue:12\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| (e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue."))
                // SD-34 wave 34 lane A (`docs/release/SD-34-book-completion/artifacts/
                // bucket-d-mining/wave34_laneA_weapon_and_armor_proficiency_cycle_
                // receipt.md`): Rogue's own Weapon and Armor Proficiency identity
                // grant is now genuinely grounded as a level-independent, always-on
                // +0 record (true since level 1, mirrors the same "no gate to lift"
                // idiom as Jack-of-All-Trades) -- not a bounded, level-gated feature
                // this slice's negative control is checking for.
                && e.id != "class_feature.rogue.weapon_and_armor_proficiency"),
        "multiclass Rogue must not gain any bounded rogue explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-12 widening -----

#[test]
fn matrix_rogue_row_names_level_12_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level12_widening"),
        "rogue row must cite the live SD18 level-12 widening proof surface: {}",
        rogue.grounding_ref
    );
    let note = rogue.blocker_or_lossiness_note;
    assert!(
        note.contains("level 12") || note.contains("level-12"),
        "rogue partial note must name the level-12 widening: {note}"
    );
}
