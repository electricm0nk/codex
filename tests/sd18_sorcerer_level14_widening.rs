//! SD18 Sorcerer level-14 widening grounding proof.
//!
//! Widens the accepted deterministic Human Sorcerer level-1..level-13 spell-
//! bearing chassis (`tests/sd18_sorcerer_level13_widening.rs`) to Sorcerer
//! level 14, mirroring the sibling-class level-range-gate idiom
//! (`supported_sorcerer_level` is generalized from `1..=13` to `1..=14` via
//! `MAX_SUPPORTED_SORCERER_LEVEL = 14`, exactly as prior cycles widened
//! `MAX_SUPPORTED_BARBARIAN_LEVEL`, `MAX_SUPPORTED_BARD_LEVEL`,
//! `MAX_SUPPORTED_CLERIC_LEVEL`, `MAX_SUPPORTED_DRUID_LEVEL`,
//! `MAX_SUPPORTED_FIGHTER_LEVEL`, `MAX_SUPPORTED_PALADIN_LEVEL`,
//! `MAX_SUPPORTED_ROGUE_LEVEL`, and `MAX_SUPPORTED_RANGER_LEVEL`, all from 13
//! to 14 already this level-14 sweep). §3.1 race rows and §3.3 interaction
//! rows stay fully exhausted / structurally blocked (cited from the progress
//! doc, not re-derived). This cycle re-verified live which §3.2 classes
//! still sat at the level-13 ceiling: Sorcerer and Wizard both did. Per the
//! brief's explicit alphabetical tie-break, this cycle picks **Sorcerer**.
//!
//! Three PF1 CRB primary sources (d20pfsrd, the Archives of Nethys
//! aonprd.com mirror, and legacy.aonprd.com) were read directly before
//! writing any code or test. d20pfsrd and legacy.aonprd.com agreed
//! byte-for-byte on every value below; the aonprd.com fetch was internally
//! inconsistent on the level-13 spells-per-day row (it reported
//! `6/6/6/6/6/3/2`, which contradicts the already-landed and independently
//! re-verified level-13 truth `6/6/6/6/6/4` confirmed by both other sources
//! and by this codebase's own accepted level-13 test), so that fetch is
//! rejected as a tool artifact rather than treated as a genuine conflict,
//! per the sweep's established practice. The level-14 values below are
//! corroborated by two independent, mutually-consistent sources:
//!
//! - level 14 base attack bonus genuinely rises to +7 (`14 / 2 = 7`, up from
//!   level 13's +6) via the same already-grounded formula, not re-derived.
//!   Both poor saves stay numerically unchanged: +4 Fortitude (`14 / 3 = 4`)
//!   and +4 Reflex (`14 / 3 = 4`), an integer-division coincidence with
//!   level 13, not a sign either formula stopped scaling — while the good
//!   Will save genuinely rises to +9 (`14 / 2 + 2 = 9`, up from level 13's
//!   +8).
//! - the PF1 Core Rulebook Sorcerer class table's level-14 "Special" column
//!   is genuinely BLANK — no new class feature is named at level 14 (unlike
//!   level 13's "Bloodline feat, bloodline spell"), mirroring the Wizard's
//!   own genuinely-blank level-13 Special column precedent. This slice
//!   therefore grounds no new grant-only identity record; only the existing
//!   arithmetic pillars widen.
//! - the Spells per Day table's level-14 row is `6/6/6/6/6/5/3` (1st through
//!   7th), genuinely risen from level 13's `6/6/6/6/6/4` at the 6th spell
//!   level (4 -> 5) AND opening a genuinely NEW 7th-level spell-level column
//!   (value 3) — the sweep's first genuinely new spell-level column opening
//!   since level 13 was landed. This widens `sorcerer_base_spells_per_day`
//!   from a 6-element to a 7-element array.
//! - the Spells Known table's level-14 row is `9/5/5/4/4/3/2/1` (0th through
//!   7th), with the 0th through 6th columns numerically UNCHANGED from level
//!   13's `9/5/5/4/4/3/2`, and a genuinely NEW 7th-level known-spells column
//!   opening (value 1). This widens `sorcerer_spells_known` from a
//!   7-element to an 8-element array.
//! - the spell-level access ladder genuinely rises to 7 at level 14 (up from
//!   6 at level 13) — a new `SORCERER_SEVENTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL
//!   = 14` threshold, continuing the sorcerer's two-level cadence
//!   (4/6/8/10/12/14) exactly.
//! - the spell-save-DC and Charisma-bonus-spell formulas are unchanged (live
//!   arithmetic over the newly widened access ladder and the fixture's
//!   Charisma modifier); they automatically extend to the 7th spell level
//!   with no new code, since both loops already iterate generically over
//!   `1..=sorcerer_spell_level_access`.
//! - the bloodline choice and bloodline class-skill choice recognitions are
//!   not level-gated, so both still fire at level 14 for the same fixture
//!   selections.
//!
//! It deliberately does not touch Arcane Bond, bloodline arcana, the
//! bloodline feat selection, the bloodline bonus spell selection, or the
//! spontaneous which-spells-known selection / casting-execution burden (all
//! stay named-but-unproven, unchanged from levels 1-13), and it does not
//! ground Sorcerer level 15+. It also preserves the accepted Sorcerer
//! level-1..level-13 truth (unchanged), the Fighter negative control, and
//! the multiclass negative control. Per the brief's lesson about stale
//! negative controls, this cycle also moves the sibling "level 14 is not
//! promoted" negative controls in `tests/sd13_sorcerer_level10_progression.rs`,
//! `tests/sd18_sorcerer_level11_widening.rs`, and
//! `tests/sd18_sorcerer_level12_widening.rs` to a "level 15 is not promoted"
//! boundary in the same commit.

use codex::rules_core::pilot_compute::{PilotBaseChassisComputation, compute_pilot_base_chassis};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation};

const SORCERER_LEVEL13_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level13_sd18_widening_deterministic_input.txt"
);

const SORCERER_LEVEL14_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_sorcerer_level14_sd18_widening_deterministic_input.txt"
);

const FIGHTER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

const PER_DAY_PREFIX: &str = "class_chassis.sorcerer.spontaneous.base_spells_per_day.";
const KNOWN_PREFIX: &str = "class_chassis.sorcerer.spontaneous.spells_known.";

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

// ----- Base attack bonus at level 14 genuinely rises -----

#[test]
fn sorcerer_level14_base_attack_bonus_is_grounded() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(
        base_attack.value, 7,
        "Sorcerer level 14 1/2-BAB progression (14 / 2) must equal 7 — genuinely risen from \
         level 13's +6: {}",
        base_attack.detail
    );
}

// ----- Base saves at level 14: poor saves unchanged, good Will rises -----

#[test]
fn sorcerer_level14_base_saves_are_grounded() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let fortitude = explanation(&computation, "class_chassis.sorcerer.base_save.fortitude");
    assert_eq!(
        fortitude.value, 4,
        "Sorcerer level 14 poor Fortitude (14/3) must equal 4 — unchanged from level 13, an \
         integer-division coincidence"
    );

    let reflex = explanation(&computation, "class_chassis.sorcerer.base_save.reflex");
    assert_eq!(
        reflex.value, 4,
        "Sorcerer level 14 poor Reflex (14/3) must equal 4 — unchanged from level 13, an \
         integer-division coincidence"
    );

    let will = explanation(&computation, "class_chassis.sorcerer.base_save.will");
    assert_eq!(
        will.value, 9,
        "Sorcerer level 14 good Will (14/2+2) must equal 9 — genuinely risen from level 13's +8"
    );
}

// ----- Base spells per day widen at level 14, opening a genuinely new 7th-level column -----

#[test]
fn sorcerer_level14_base_spells_per_day_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_6"), 5),
            (format!("{PER_DAY_PREFIX}spell_level_7"), 3),
        ],
        "level 14 (`6/6/6/6/6/5/3`): the 6th-level column rises from 4 to 5 and a genuinely new \
         7th-level column opens at 3 — the sweep's first genuinely new spell-level column \
         opening since level 13 landed"
    );
}

// ----- Spells known widen at level 14, opening a genuinely new 7th-level column -----

#[test]
fn sorcerer_level14_spells_known_match_the_raw_table_row() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(
        values_with_prefix(&computation, KNOWN_PREFIX),
        vec![
            (format!("{KNOWN_PREFIX}spell_level_0"), 9),
            (format!("{KNOWN_PREFIX}spell_level_1"), 5),
            (format!("{KNOWN_PREFIX}spell_level_2"), 5),
            (format!("{KNOWN_PREFIX}spell_level_3"), 4),
            (format!("{KNOWN_PREFIX}spell_level_4"), 4),
            (format!("{KNOWN_PREFIX}spell_level_5"), 3),
            (format!("{KNOWN_PREFIX}spell_level_6"), 2),
            (format!("{KNOWN_PREFIX}spell_level_7"), 1),
        ],
        "level 14 (`9/5/5/4/4/3/2/1`): the 0th through 6th columns stay numerically unchanged \
         from level 13's `9/5/5/4/4/3/2`, while a genuinely new 7th-level known-spells column \
         opens at 1"
    );
}

// ----- Spell-level access ladder rises to 7 at level 14 -----

#[test]
fn sorcerer_level14_spell_level_access_rises_to_seven() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let access = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_level_access",
    );
    assert_eq!(
        access.value, 7,
        "Sorcerer level 14 spell-level access must genuinely rise to 7th-level spells — the \
         sorcerer's two-level cadence (4/6/8/10/12/14) continuing exactly: {}",
        access.detail
    );
}

// ----- Bonus spells and spell-save DCs extend to the new 7th spell level with no new code -----

#[test]
fn sorcerer_level14_bonus_spells_and_save_dcs_extend_to_seventh_level() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let dc7 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_7",
    );
    // Fixture Charisma 17 -> Charisma modifier +3 (unchanged deterministic posture).
    // 10 + 7 + 3 = 20.
    assert_eq!(
        dc7.value, 21,
        "Sorcerer level 14 7th-level spell save DC must be 10 + 7 + Charisma modifier: {}",
        dc7.detail
    );

    let bonus7 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.bonus_spells_per_day.spell_level_7",
    );
    // Charisma modifier +3 < spell level 7, so bonus spells at 7th level is 0.
    assert_eq!(
        bonus7.value, 0,
        "Sorcerer level 14 7th-level bonus spells from a +3 Charisma modifier must be 0 (below \
         the spell level threshold): {}",
        bonus7.detail
    );

    let total7 = explanation(
        &computation,
        "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_7",
    );
    assert_eq!(
        total7.value, 3,
        "Sorcerer level 14 7th-level total spells per day must equal the base count (3) plus \
         the zero bonus: {}",
        total7.detail
    );
}

// ----- Bloodline choice recognition still fires at level 14 -----

#[test]
fn sorcerer_level14_still_recognizes_the_bloodline_choice() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.sorcerer.bloodline_choice");
    assert_eq!(
        choice.value, 0,
        "bloodline choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Arcane"),
        "bloodline choice recognition must still name the Arcane bloodline at level 14: {}",
        choice.detail
    );
}

// ----- The spell-bearing baseline recognition and both burden diagnostics persist -----

#[test]
fn sorcerer_level14_still_recognizes_the_spell_bearing_baseline_and_claim_blocks_burdens() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-14 Sorcerer must still recognize the spell-bearing baseline identity: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
            && d.claim_blocking),
        "level-14 Sorcerer must still claim-block on the Arcane Bond / bloodline progression \
         burden: {:?}",
        computation.diagnostics
    );
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported")
    {
        Some(spell_blocker) => assert!(
            spell_blocker.claim_blocking,
            "if the spell blocker fires at all, it must be claim-blocking"
        ),
        None => {
            // (v0.6 alpha swarm, risks item 8) class_spell.sorcerer.spontaneous.unsupported
            // is no longer unconditional -- it's a real, conditional validation of
            // AcquisitionMode::Known selections. This fixture predates spells_selected
            // (zero known spells), so the posture is genuinely valid and the blocker
            // correctly does not fire -- confirmed via the real known-spell count being
            // honestly 0, not fabricated.
            let known_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.sorcerer.known_spells")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                known_count, 0,
                "no spells are fabricated merely because the spell blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
}

// ----- No bloodline-feat/bloodline-spell record is fabricated at level 14 -----

#[test]
fn sorcerer_level14_does_not_fabricate_any_bloodline_entry() {
    let input = load(SORCERER_LEVEL14_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.to_lowercase().contains("bloodline_feat")
                || e.id.to_lowercase().contains("bloodline_power")
                || e.id.to_lowercase().contains("bloodline_spell")),
        "level-14 Sorcerer must not fabricate a bloodline-feat/bloodline-spell record — level \
         14's Special column is genuinely blank, so nothing bloodline-specific is newly named \
         here either: {:?}",
        computation.explanations
    );
}

// ----- Negative control: level 13 truth is unchanged by this widening -----

#[test]
fn sorcerer_level13_truth_is_unchanged_by_this_slice() {
    let input = load(SORCERER_LEVEL13_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let base_attack = explanation(&computation, "class_chassis.sorcerer.base_attack_bonus");
    assert_eq!(base_attack.value, 6, "Sorcerer level 13 base attack bonus must stay 6");

    assert_eq!(
        values_with_prefix(&computation, PER_DAY_PREFIX),
        vec![
            (format!("{PER_DAY_PREFIX}spell_level_1"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_2"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_3"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_4"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_5"), 6),
            (format!("{PER_DAY_PREFIX}spell_level_6"), 4),
        ],
        "Sorcerer level 13 base spells per day must stay `6/6/6/6/6/4` with no 7th-level column"
    );
}

// ----- Negative control: level 21 stays unrecognized by this cycle -----
//
// SD18 widened Sorcerer support to level 20, PF1's level cap
// (tests/sd18_sorcerer_level20_widening.rs), so this boundary moved to 21
// (which does not exist in PF1).

#[test]
fn sorcerer_level_21_is_not_promoted_by_this_slice() {
    let level_21 = SORCERER_LEVEL14_FIXTURE.replace("class:sorcerer:14", "class:sorcerer:21");
    let input = load(&level_21);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "level-21 Sorcerer must not gain any bounded sorcerer chassis explanation: {:?}",
        computation.explanations
    );
}

// ----- Negative control: the sorcerer path must not leak onto other classes -----

#[test]
fn fighter_does_not_gain_sorcerer_level14_recognition() {
    let fighter = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !fighter_computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "the Fighter chassis must not surface any sorcerer-namespaced explanation: {:?}",
        fighter_computation.explanations
    );
}

// ----- Negative control: multiclass Sorcerer is not promoted -----

#[test]
fn multiclass_sorcerer_level14_is_not_promoted_by_this_slice() {
    let multiclass = SORCERER_LEVEL14_FIXTURE.replace(
        "class_level=class:sorcerer:14",
        "class_level=class:sorcerer:14\nclass_level=class:fighter:1",
    );
    let input = load(&multiclass);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_chassis.sorcerer.")
                || e.id == "class_chassis.spell_baseline.sorcerer"),
        "multiclass Sorcerer must not gain any bounded sorcerer chassis explanation: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "multiclass Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix note names the level-14 widening -----

#[test]
fn matrix_sorcerer_row_names_level_14_widening() {
    let matrix = seeded_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer progression_and_spell_burden row must exist");

    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer
            .grounding_ref
            .contains("sd18_sorcerer_level14_widening"),
        "sorcerer row must cite the live SD18 level-14 proof surface: {}",
        sorcerer.grounding_ref
    );
    let note = sorcerer.blocker_or_lossiness_note;
    assert!(
        note.contains("level 14") || note.contains("level-14"),
        "sorcerer partial note must name the level-14 widening: {note}"
    );
}
