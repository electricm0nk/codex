//! SD18 Rogue level-20 widening grounding proof.
//!
//! Widens the accepted Human Rogue level-1..level-19 chassis
//! (`tests/sd18_rogue_level19_widening.rs`, the loop's most recent Rogue
//! ceiling) to Rogue level 20, the loop's EIGHTH §3.2 level-20 landing
//! (after Cleric, Wizard, Barbarian, Bard, Fighter, Paladin, and Ranger),
//! mirroring the sibling-class level-range-gate idiom
//! (`supported_rogue_level` is generalized from `1..=19` to `1..=20` via
//! `MAX_SUPPORTED_ROGUE_LEVEL = 20`, exactly as every prior level-11..19
//! cycle widened its own `MAX_SUPPORTED_<CLASS>_LEVEL`). Both PF1 CRB
//! primary sources (d20pfsrd and the Archives of Nethys aonprd.com mirror)
//! were read directly before writing any code or test, fetching the class
//! table and the Master Strike class-feature description in full (a raw
//! curl fetch with a Python tag-strip, not summarized WebFetch), and both
//! agree byte-for-byte:
//!
//! - level 18: BAB +13/+8/+3, Fort +6, Ref +11, Will +6, Special "Rogue
//!   talent, trap sense +6"
//! - level 19: BAB +14/+9/+4, Fort +6, Ref +11, Will +6, Special "Sneak
//!   attack +10d6"
//! - level 20: BAB +15/+10/+5, Fort +6, Ref +12, Will +6, Special "Master
//!   strike, rogue talent"
//!
//! At level 20: base attack bonus GENUINELY RISES to +15 (`20 * 3 / 4 =
//! 15`, up from +14 at level 19). Good Reflex GENUINELY RISES to +12
//! (`20 / 2 + 2 = 12`, up from +11 at level 19); poor Fortitude/Will both
//! stay +6 (`20 / 3 = 6`, integer-division coincidences with level 19). The
//! level-20 "Special" column reads "Master strike, rogue talent" -- TWO
//! things happen at once: (1) 20 IS a rogue-talent cadence level (talents
//! land at 2/4/6/8/10/12/14/16/18/20 per "she gains an additional rogue
//! talent for every 2 levels of rogue attained after 2nd level"), the
//! FINAL talent slot within PF1's 1-20 character-level cap, so a TENTH
//! numbered talent slot (`choice:rogue_talent_10`, gate 20) is appended to
//! the existing tuple-array idiom, mirroring the level-18 cycle's own
//! ninth-slot landing exactly; and (2) Master Strike, the rogue's 20th-level
//! capstone, is newly granted and grounded as a bounded GRANT-only identity
//! record (value 0, non-fabricated) mirroring exactly the already-proven
//! Paladin Holy Champion / Ranger Master Hunter capstone idiom -- no
//! action-economy, attack-resolution, or saving-throw-resolution engine
//! exists anywhere in this codebase, so this grounds no actual mechanic.
//! Sneak attack stays 10d6 (`(20 + 1) / 2 = 10`, an integer-division
//! coincidence with level 19: the die-count formula's own final PF1 CRB
//! tier, confirmed unchanged at level 20 by both primary sources). Trap
//! Sense stays +6 (`20 / 3 = 6`, an integer-division coincidence with level
//! 19, its own final PF1 CRB tier) and Trapfinding GENUINELY RISES to 10
//! (`max(20 / 2, 1) = 10`, up from 9 at level 19, via the pre-existing
//! formula, not a new record), neither named in the level-20 "Special"
//! column. Evasion, Uncanny Dodge, and Improved Uncanny Dodge all stay
//! granted, not re-derived.
//!
//! This widening needs ZERO new tier constants for base attack/saves/
//! sneak-attack/trap-sense/trapfinding (all already level-generic formulas)
//! -- the production-code change is raising `MAX_SUPPORTED_ROGUE_LEVEL`
//! from 19 to 20, appending the tenth numbered talent slot, and adding the
//! Master Strike grant-only identity record. This closes Rogue's own
//! per-level arithmetic-widening frontier: level 20 is the final level
//! within PF1's 1-20 cap.
//!
//! This slice deliberately does not touch the rogue-talent tree's own
//! effects (standard or advanced), any check-execution engine, or
//! sneak-attack damage application (all stay named-but-unproven, unchanged
//! from levels 1-19), and it does not compute Master Strike's own save DC
//! or effect resolution (sleep/paralysis/death), only the grant-only
//! identity record. It also preserves the accepted Rogue level-1..level-19
//! truth (unchanged), the Fighter negative control, and the multiclass
//! negative control.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};

const ROGUE_LEVEL19_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level19_sd18_widening_deterministic_input.txt"
);

const ROGUE_LEVEL20_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_rogue_level20_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const ROGUE_EVASION_ID: &str = "class_feature.rogue.evasion";
const ROGUE_TRAP_SENSE_ID: &str = "class_feature.rogue.trap_sense";
const ROGUE_UNCANNY_DODGE_ID: &str = "class_feature.rogue.uncanny_dodge";
const ROGUE_IMPROVED_UNCANNY_DODGE_ID: &str = "class_feature.rogue.improved_uncanny_dodge";
const ROGUE_MASTER_STRIKE_ID: &str = "class_feature.rogue.master_strike";

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

// ----- Base attack bonus and Reflex genuinely rise at level 20; Fort/Will stay put -----

#[test]
fn rogue_level20_base_attack_and_reflex_rise() {
    let input = load(ROGUE_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(
        base_attack.value, 15,
        "Rogue level 20 3/4-BAB progression (20 * 3 / 4) must genuinely rise to 15, up from 14 \
         at level 19: {}",
        base_attack.detail
    );

    let fortitude = explanation(&computation, "class_chassis.rogue.base_save.fortitude");
    assert_eq!(
        fortitude.value, 6,
        "Rogue level 20 poor Fortitude (20/3) must stay 6, an integer-division coincidence with \
         level 19"
    );

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(
        reflex.value, 12,
        "Rogue level 20 good Reflex (20/2+2) must genuinely rise to 12, up from 11 at level 19: \
         {}",
        reflex.detail
    );

    let will = explanation(&computation, "class_chassis.rogue.base_save.will");
    assert_eq!(
        will.value, 6,
        "Rogue level 20 poor Will (20/3) must stay 6, an integer-division coincidence with level \
         19"
    );
}

// ----- Sneak attack stays at 10d6 (its final PF1 CRB tier) -----

#[test]
fn rogue_level20_sneak_attack_stays_at_ten() {
    let input = load(ROGUE_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(
        sneak_attack.value, 10,
        "Rogue level 20 sneak attack die count ((20 + 1) / 2) must stay 10 (10d6), an \
         integer-division coincidence with level 19, its own final PF1 CRB tier: {}",
        sneak_attack.detail
    );
}

// ----- Tenth talent slot grants at level 20: 20 IS a rogue-talent cadence level -----

#[test]
fn rogue_level20_gains_tenth_talent_slot() {
    let input = load(ROGUE_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let slot_ten = explanation(&computation, "class_chassis.rogue.talent_10_choice");
    assert_eq!(
        slot_ten.value, 0,
        "the tenth talent slot must be a bounded +0 recognition record at level 20: {}",
        slot_ten.detail
    );

    // The ninth slot (granted at level 18) still carries over unchanged.
    let slot_nine = explanation(&computation, "class_chassis.rogue.talent_9_choice");
    assert_eq!(
        slot_nine.value, 0,
        "the ninth talent slot must stay a bounded +0 recognition record at level 20: {}",
        slot_nine.detail
    );
}

// ----- Master Strike grants at level 20 as a bounded grant-only identity record -----

#[test]
fn rogue_level20_gains_master_strike() {
    let input = load(ROGUE_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // SD-32 Epic 1 (compute-library wiring): the fixture's Intelligence 12
    // (modifier +1) is now run through the SAME formula_interpreter-backed
    // `resolve_pcgen_var_chain` mechanism already fixture-checked at
    // `tests/fixtures/rules_core/derived-evaluator-fixtures.json`'s
    // `class_feature_description_entries` (`rogue_master_strike`), which
    // pins `10+(MasterStrikeLVL/2)+INT` byte-identical against the pinned
    // oracle's `cr_abilities_class.lst:1619`. At level 20 with INT modifier
    // +1: `10 + 20/2 + 1 = 21`. This is no longer a fabricated value -- it
    // is the corpus's own formula, evaluated by the already-authorised
    // interpreter (`decisions.md §3`, operator ruling §20).
    let master_strike = explanation(&computation, ROGUE_MASTER_STRIKE_ID);
    assert_eq!(
        master_strike.value, 21,
        "Master Strike's save DC at rogue level 20 with Intelligence modifier +1 is \
         10 + level/2 + INT = 21, computed via the corpus's own BONUS:VAR formula: {}",
        master_strike.detail
    );
    assert!(
        master_strike.detail.contains("21"),
        "the detail text must name the computed DC, not just describe the rule: {}",
        master_strike.detail
    );
}

// ----- Trap Sense stays put, Trapfinding genuinely rises at level 20 -----

#[test]
fn rogue_level20_trap_sense_stays_trapfinding_rises() {
    let input = load(ROGUE_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let trap_sense = explanation(&computation, ROGUE_TRAP_SENSE_ID);
    assert_eq!(
        trap_sense.value, 6,
        "Rogue level 20 Trap Sense (20/3) must stay +6, an integer-division coincidence with \
         level 19, its own final PF1 CRB tier: {}",
        trap_sense.detail
    );

    let trapfinding = explanation(&computation, "class_chassis.rogue.trapfinding");
    assert_eq!(
        trapfinding.value, 10,
        "Rogue level 20 Trapfinding (max(20/2, 1)) must genuinely rise to 10, up from 9 at \
         level 19, not named in the level-20 'Special' column: {}",
        trapfinding.detail
    );
}

// ----- Remaining pillars carry over unchanged -----

#[test]
fn rogue_level20_remaining_pillars_carry_over() {
    let input = load(ROGUE_LEVEL20_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for id in [
        ROGUE_EVASION_ID,
        ROGUE_UNCANNY_DODGE_ID,
        ROGUE_IMPROVED_UNCANNY_DODGE_ID,
    ] {
        let record = explanation(&computation, id);
        assert_eq!(
            record.value, 0,
            "'{id}' must stay a granted +0 identity record at level 20"
        );
    }
}

// ----- Negative control: the level-19 fixture is unaffected by this widening -----

#[test]
fn rogue_level19_truth_is_unchanged_by_this_slice() {
    let input = load(ROGUE_LEVEL19_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let sneak_attack = explanation(&computation, "class_chassis.rogue.sneak_attack");
    assert_eq!(sneak_attack.value, 10, "Rogue level 19 sneak attack must stay 10d6");

    let base_attack = explanation(&computation, "class_chassis.rogue.base_attack_bonus");
    assert_eq!(base_attack.value, 14, "Rogue level 19 base attack bonus must stay 14");

    let reflex = explanation(&computation, "class_chassis.rogue.base_save.reflex");
    assert_eq!(reflex.value, 11, "Rogue level 19 Reflex save must stay +11");

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.rogue.talent_10_choice"),
        "Rogue level 19 must not gain a tenth talent slot: {:?}",
        computation.explanations
    );

    let master_strike = explanation(&computation, ROGUE_MASTER_STRIKE_ID);
    assert_eq!(
        master_strike.value, 0,
        "Rogue level 19 Master Strike must still be a correct level-gate absence record"
    );
    assert!(
        master_strike.detail.contains("correctly absent"),
        "Rogue level 19 Master Strike must read as absent, not granted: {}",
        master_strike.detail
    );
}

// ----- Negative control: level 21 stays unrecognized by this slice -----

#[test]
fn rogue_level_21_is_not_promoted_by_this_slice() {
    let level_21 = ROGUE_LEVEL20_FIXTURE.replace("class:rogue:20", "class:rogue:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "level-21 Rogue must not gain any bounded rogue explanation (PF1 has no 21st character \
         level; this is a pure implementation-gate check): {:?}",
        computation.explanations
    );
}

// ----- Negative control: the rogue path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_rogue_level20_recognition() {
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
fn multiclass_rogue_level20_is_not_promoted_by_this_slice() {
    let multiclass = ROGUE_LEVEL20_FIXTURE.replace(
        "class_level=class:rogue:20",
        "class_level=class:rogue:20\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.rogue.")
                || e.id.starts_with("class_feature.rogue.")),
        "multiclass Rogue must not gain any bounded rogue explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Rogue must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-20 widening -----

#[test]
fn matrix_rogue_row_names_level_20_widening() {
    let matrix = seeded_current_truth();
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue bounded_progression row must exist");

    assert_eq!(rogue.support_state, SupportState::Supported); // promoted by SD-19 Class Progression Catalog browser
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(rogue.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        rogue.grounding_ref.contains("sd18_rogue_level20_widening"),
        "matrix grounding_ref must name the level-20 widening test: {}",
        rogue.grounding_ref
    );
}
