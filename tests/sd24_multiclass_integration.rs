//! SD-24 Epic 5 (criterion 5.3): Integration test consumes ingested content.
//!
//! Distinct from criterion 5.2's `sd24_multiclass_deterministic.rs`, which
//! asserts against canonical PF1 formulas *hand-typed independently in the
//! test file*, this test derives its expected values from the actually
//! ingested per-class-per-level content table
//! (`rules_tables::crb::class_tables::class_tables()`, "SD-19's foundation"
//! per every `level_up::<class>.rs` module's own doc comment) instead of
//! re-deriving the formula a second time. If the production dispatch
//! (`compute_pilot_base_chassis` / `compute_multiclass_base_chassis`) ever
//! drifted from the single ingested source of truth this table represents,
//! this test -- not a hand-typed twin that could drift in lockstep -- is
//! what catches it.
//!
//! Three real ingested fixtures already loaded via
//! `load_character_input_fixture` (the same "case_id=... / provenance=..."
//! ingestion format criterion 5.1/5.2's own fixtures use) are driven end to
//! end through the real compute pipeline:
//!
//! - `pf1_human_fighter9_wizard1_sd24_multiclass_lv10_input.txt` (Fighter 9 /
//!   Wizard 1)
//! - `pf1_human_wizard9_fighter1_sd24_multiclass_lv10_input.txt` (Wizard 9 /
//!   Fighter 1)
//! - `pf1_human_fighter4_wizard1_sd24_multiclass_split_input.txt` (Fighter 4 /
//!   Wizard 1, the split-transition point)
//!
//! Base attack bonus is summed directly from each class's own
//! `class_tables()` row (already an integer, floored per class -- BAB does
//! not need PF1's sum-fractions-then-round-down-once save rule). Saves use
//! `good_saves_for` (this cycle's own new `class_tables` accessor,
//! `rules_tables::crb::class_tables::good_saves_for`) to look up each
//! class's good/poor classification from the ingested table itself, then
//! applies PF1's sum-of-unrounded-fractions-then-floor-once multiclass rule
//! -- the fractional arithmetic is inherent to the PF1 rule (not a data
//! table), but which classes get which classification is now read from the
//! ingested table rather than re-declared.

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::rules_tables::crb::class_tables::{ClassId, class_tables, good_saves_for};
mod common;
use common::load;

const FIGHTER9_WIZARD1_LV10_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter9_wizard1_sd24_multiclass_lv10_input.txt"
);
const WIZARD9_FIGHTER1_LV10_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_wizard9_fighter1_sd24_multiclass_lv10_input.txt"
);
const FIGHTER4_WIZARD1_SPLIT_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter4_wizard1_sd24_multiclass_split_input.txt"
);

/// This class-id string's `class_tables::ClassId`, per the fixed Fighter+
/// Wizard multiclass scope (SD-24 Epic 5).
fn table_class_id(class_id: &str) -> ClassId {
    match class_id {
        "class:fighter" => ClassId::Fighter,
        "class:wizard" => ClassId::Wizard,
        other => panic!("unsupported class id for this integration test: {other}"),
    }
}

/// `class_tables()`'s own base-attack-bonus cell for `class_id` at `level` --
/// the ingested content this test consumes as its oracle, not a re-derived
/// formula.
fn ingested_bab(class_id: &str, level: u8) -> i16 {
    let table_id = table_class_id(class_id);
    class_tables()
        .into_iter()
        .find(|row| row.class_id == table_id && row.level == level)
        .unwrap_or_else(|| panic!("class_tables() must carry a {class_id} row at level {level}"))
        .base_attack_bonus
}

/// The un-rounded fractional pre-floor save value PF1's multiclass rule sums
/// across classes, using `good_saves_for` (ingested classification) rather
/// than a hand-typed per-class twin.
fn ingested_fractional_saves(class_id: &str, level: u8) -> (f64, f64, f64) {
    let table_id = table_class_id(class_id);
    let (fort_good, ref_good, will_good) = good_saves_for(table_id)
        .unwrap_or_else(|| panic!("class_tables() must carry a good-save row for {class_id}"));
    let fraction = |good: bool| -> f64 {
        let level = f64::from(level);
        if good { level / 2.0 + 2.0 } else { level / 3.0 }
    };
    (fraction(fort_good), fraction(ref_good), fraction(will_good))
}

/// Expected multiclass base attack bonus / base saves for `class_levels`,
/// derived entirely from `class_tables()`'s ingested content -- the same
/// oracle the production dispatch (`compute_multiclass_base_chassis`) is
/// under test against.
fn expected_from_ingested_content(class_levels: &[(&str, u8)]) -> (i16, i16, i16, i16) {
    let mut total_bab: i16 = 0;
    let mut fort_fraction = 0.0_f64;
    let mut ref_fraction = 0.0_f64;
    let mut will_fraction = 0.0_f64;

    for (class_id, level) in class_levels {
        total_bab += ingested_bab(class_id, *level);
        let (fort, refl, will) = ingested_fractional_saves(class_id, *level);
        fort_fraction += fort;
        ref_fraction += refl;
        will_fraction += will;
    }

    (
        total_bab,
        fort_fraction.floor() as i16,
        ref_fraction.floor() as i16,
        will_fraction.floor() as i16,
    )
}

fn assert_matches_ingested_content(input: &CharacterInput, class_levels: &[(&str, u8)], context: &str) {
    let computation = compute_pilot_base_chassis(input);

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_chassis.unsupported" && d.claim_blocking),
        "{context} must not be claim-blocked on its own base chassis: {:?}",
        computation.diagnostics
    );

    let (expected_bab, expected_fort, expected_ref, expected_will) =
        expected_from_ingested_content(class_levels);

    assert_eq!(
        computation.base_attack_bonus, expected_bab,
        "{context} base attack bonus must match class_tables()'s ingested rows: {:?}",
        computation.explanations
    );
    assert_eq!(
        computation.base_saves.fortitude, expected_fort,
        "{context} Fortitude save must match class_tables()'s ingested good-save rows: {:?}",
        computation.explanations
    );
    assert_eq!(
        computation.base_saves.reflex, expected_ref,
        "{context} Reflex save must match class_tables()'s ingested good-save rows: {:?}",
        computation.explanations
    );
    assert_eq!(
        computation.base_saves.will, expected_will,
        "{context} Will save must match class_tables()'s ingested good-save rows: {:?}",
        computation.explanations
    );
}

#[test]
fn fighter9_wizard1_lv10_matches_ingested_class_tables_content() {
    let input = load(FIGHTER9_WIZARD1_LV10_FIXTURE);
    assert_matches_ingested_content(
        &input,
        &[("class:fighter", 9), ("class:wizard", 1)],
        "Fighter 9 / Wizard 1",
    );
}

#[test]
fn wizard9_fighter1_lv10_matches_ingested_class_tables_content() {
    let input = load(WIZARD9_FIGHTER1_LV10_FIXTURE);
    assert_matches_ingested_content(
        &input,
        &[("class:fighter", 1), ("class:wizard", 9)],
        "Wizard 9 / Fighter 1",
    );
}

#[test]
fn fighter4_wizard1_split_matches_ingested_class_tables_content() {
    let input = load(FIGHTER4_WIZARD1_SPLIT_FIXTURE);
    assert_matches_ingested_content(
        &input,
        &[("class:fighter", 4), ("class:wizard", 1)],
        "Fighter 4 / Wizard 1 (split)",
    );
}

/// Solo-class control: the Fighter-only chassis (single-class dispatch, not
/// `compute_multiclass_base_chassis`) must also agree with the ingested
/// `class_tables()` row at every level 1-10 -- proving the ingested content
/// is the oracle for both the single-class and multiclass dispatch paths,
/// not just the multiclass-summation arithmetic.
#[test]
fn solo_fighter_level_1_to_10_matches_ingested_class_tables_content() {
    use codex::rules_core::character_input::CharacterClassLevel;

    let base = load(FIGHTER9_WIZARD1_LV10_FIXTURE);
    for level in 1..=10u8 {
        let mut input = base.clone();
        input.chosen.class_levels = vec![CharacterClassLevel {
            class_id: "class:fighter".to_owned(),
            level,
        }];
        assert_matches_ingested_content(
            &input,
            &[("class:fighter", level)],
            &format!("solo Fighter level {level}"),
        );
    }
}

/// Solo-class control: the Wizard-only chassis at every level 1-10.
#[test]
fn solo_wizard_level_1_to_10_matches_ingested_class_tables_content() {
    use codex::rules_core::character_input::CharacterClassLevel;

    let base = load(WIZARD9_FIGHTER1_LV10_FIXTURE);
    for level in 1..=10u8 {
        let mut input = base.clone();
        input.chosen.class_levels = vec![CharacterClassLevel {
            class_id: "class:wizard".to_owned(),
            level,
        }];
        assert_matches_ingested_content(
            &input,
            &[("class:wizard", level)],
            &format!("solo Wizard level {level}"),
        );
    }
}
