//! v0.6 alpha swarm: carrying-capacity / encumbrance catalogue adoption.
//!
//! `src/rules_core/encumbrance.rs` (commit `d475097`) grounds the alpha
//! bar's "carry capacity" and "encumbrance" calculations, previously a
//! complete production gap (QA's original wave-1 gap-list survey found
//! every prior "encumbrance"/"carrying capacity" hit in `src/rules_core/`
//! was a doc-comment disclaimer or corpus flavor text, never computation).
//! The module carries its own inline `#[cfg(test)] mod tests` (backend's
//! stopgap since `tests/**` is QA's owned surface for this swarm). This
//! file is QA's independent catalogue adoption: real CRB corpus records
//! (transcribed from this repo's own `data/corpus/core_rulebook/equipment/`
//! shape), independently authored assertions, and PCGen cross-check values
//! pulled from the real PCGen checkout during QA's original formula-spec
//! pass (`/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`),
//! not from `encumbrance.rs`'s own inline test module.

use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::encumbrance::{
    compute_encumbrance, carrying_capacity_thresholds, CarryingCapacityThresholds, EncumbranceLevel,
};
use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

/// Independently transcribed real CRB records (Chain Shirt, Longsword,
/// Backpack) -- different items than `encumbrance.rs`'s own inline
/// fixture (Leather Armor, Buckler, Longsword), so this catalogue entry
/// does not merely re-run the implementer's own sample set.
const FIXTURE_TEXT: &str = "\
Chain Shirt\tKEY:Chain Shirt (Base)\tTYPE:Armor.Light\tCOST:100\tWT:25\tACCHECK:-2\tMAXDEX:4\tSPELLFAILURE:20\tBONUS:COMBAT|AC|4|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
Backpack\tKEY:Backpack\tTYPE:General\tCOST:2\tWT:2
";

fn corpus_from(text: &str) -> SourcePackageContent<'static> {
    let result = parse_equipment_entries("cr_equip_arms_armor.lst", text);
    assert!(result.diagnostics.is_empty(), "fixture text must parse cleanly: {:?}", result.diagnostics);
    let source_ref = SourceRef { lst_file: "cr_equip_arms_armor.lst".to_string(), line: 1 };
    let mut corpus = SourcePackageContent::empty("core_rulebook", source_ref);
    for record in result.entries {
        let record: &'static EquipmentRecord = Box::leak(Box::new(record));
        corpus.push(convert_equipment_record(record));
    }
    corpus
}

fn selection(item_id: &str, state: ActiveState) -> EquipmentSelection {
    EquipmentSelection {
        item_id: item_id.to_owned(),
        equipped_or_active: state == ActiveState::EquippedActive,
        active_state: state,
        applied_modifiers: Vec::new(),
    }
}

/// Cross-checks `carrying_capacity_thresholds` against the real PCGen
/// engine's own `load.lst` data (not the Archives of Nethys citation
/// `encumbrance.rs` itself was transcribed from) -- an independent second
/// source for the same table, pulled during QA's original formula-spec
/// pass. `load.lst`'s `LOAD:<Strength>|<value>` entries are the "Heavy"
/// tier (1x multiplier per `load.lst`'s own `ENCUMBRANCE:Heavy|1`); Light
/// is 1/3 and Medium is 2/3 of that same value, per `load.lst`'s
/// `ENCUMBRANCE:Light|1/3` and `ENCUMBRANCE:Medium|2/3` entries.
#[test]
fn carrying_capacity_thresholds_match_the_real_pcgen_load_lst_table() {
    // LOAD:6|60 in load.lst.
    assert_eq!(
        carrying_capacity_thresholds(6),
        CarryingCapacityThresholds { light_max_lbs: 20.0, medium_max_lbs: 40.0, heavy_max_lbs: 60.0 }
    );
    // LOAD:16|230 in load.lst.
    assert_eq!(
        carrying_capacity_thresholds(16),
        CarryingCapacityThresholds { light_max_lbs: 76.0, medium_max_lbs: 153.0, heavy_max_lbs: 230.0 }
    );
    // LOAD:25|800 in load.lst.
    assert_eq!(
        carrying_capacity_thresholds(25),
        CarryingCapacityThresholds { light_max_lbs: 266.0, medium_max_lbs: 533.0, heavy_max_lbs: 800.0 }
    );
}

#[test]
fn carrying_capacity_thresholds_clamps_a_below_minimum_strength_score() {
    // PF1 does not define carrying capacity below Strength 1; encumbrance.rs
    // defensively floors to the Strength-1 row rather than panicking or
    // extrapolating downward. Str 0 (e.g. a not-yet-fully-loaded fixture, or
    // a hypothetical drained-to-zero score) must not crash or produce a
    // negative/nonsensical capacity.
    assert_eq!(carrying_capacity_thresholds(0), carrying_capacity_thresholds(1));
}

#[test]
fn carrying_capacity_thresholds_extrapolate_two_tiers_beyond_strength_29() {
    // Str 40 = Str 20's row (same ones digit) multiplied by 4^2 = 16, per
    // load.lst's LOADMULT:4 applied twice (two full +10 steps above the
    // Str-29 table ceiling). encumbrance.rs's own inline test only checks
    // one tier (Str 30); this checks the recursive step holds at two tiers.
    let base = carrying_capacity_thresholds(20);
    let extrapolated = carrying_capacity_thresholds(40);
    assert_eq!(
        extrapolated,
        CarryingCapacityThresholds {
            light_max_lbs: base.light_max_lbs * 16.0,
            medium_max_lbs: base.medium_max_lbs * 16.0,
            heavy_max_lbs: base.heavy_max_lbs * 16.0,
        }
    );
}

/// The full `LOAD:<Strength>|<value>` column of the real PCGen Pathfinder
/// game mode's `load.lst`, transcribed verbatim (Strength 1-29):
/// `/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`
/// lines 10-38. This is the *heavy* tier (`ENCUMBRANCE:Heavy|1`, i.e. a 1x
/// multiplier on the load score); light is `1/3` and medium is `2/3` of the
/// same value per that file's `ENCUMBRANCE:Light|1/3` / `Medium|2/3`.
const PCGEN_LOAD_LST_HEAVY_BY_STRENGTH: [i64; 29] = [
    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 115, 130, 150, 175, 200, 230, 260, 300, 350, 400, 460,
    520, 600, 700, 800, 920, 1040, 1200, 1400,
];

/// Exhaustive cross-check of *every* Strength row against the real PCGen
/// `load.lst` table, rather than the three-row spot check
/// (`carrying_capacity_thresholds_match_the_real_pcgen_load_lst_table`,
/// Str 6/16/25) that preceded it.
///
/// This matters: the spot check sampled only rows that happened to be
/// correct, so a real off-by-one in the Str 15 medium threshold (134,
/// where `load.lst` derives 133) sat undetected in
/// `encumbrance.rs`'s hand-transcribed table. A row-complete assertion
/// closes the whole class of transcription error, not just the one
/// instance.
///
/// The light/medium values are *derived* from `load.lst`'s own heavy
/// column and multipliers rather than separately transcribed, so this test
/// depends on exactly one hand-copied column instead of three. PF1 load
/// tiers truncate toward zero (integer pounds), matching PCGen's own
/// `BigDecimal` load-score arithmetic.
#[test]
fn carrying_capacity_thresholds_match_every_row_of_the_real_pcgen_load_lst_table() {
    for (index, heavy) in PCGEN_LOAD_LST_HEAVY_BY_STRENGTH.iter().enumerate() {
        let strength_score = (index + 1) as i16;
        let expected = CarryingCapacityThresholds {
            light_max_lbs: (heavy / 3) as f64,
            medium_max_lbs: (heavy * 2 / 3) as f64,
            heavy_max_lbs: *heavy as f64,
        };
        assert_eq!(
            carrying_capacity_thresholds(strength_score),
            expected,
            "Strength {strength_score} must match load.lst's LOAD:{strength_score}|{heavy} row \
             (light = 1/3, medium = 2/3 of the heavy tier)"
        );
    }
}

/// PF1's load tiers impose their own max-Dex cap and armor check penalty,
/// independent of what any individual worn item imposes. Grounded in the
/// real PCGen engine's own implementation, not reconstructed from memory:
///
///  - max Dex by load: `PlayerCharacter.java:5362-5368`
///    (`case MEDIUM -> 3; case HEAVY -> 1; case OVERLOAD -> 0;` with Light
///    imposing no cap of its own).
///  - armor check penalty by load: `PlayerCharacter.java:5331`
///    (`(load == Load.MEDIUM) ? -3 : (load == Load.HEAVY) ? -6 : 0`),
///    matching `load.lst`'s own third `ENCUMBRANCE:` field
///    (`Light|1/3||0`, `Medium|2/3||-3`, `Heavy|1||-6`).
#[test]
fn encumbrance_level_carries_the_real_pcgen_load_penalties() {
    assert_eq!(EncumbranceLevel::Light.max_dex_cap(), None, "a light load imposes no max-Dex cap");
    assert_eq!(EncumbranceLevel::Medium.max_dex_cap(), Some(3));
    assert_eq!(EncumbranceLevel::Heavy.max_dex_cap(), Some(1));
    assert_eq!(EncumbranceLevel::OverHeavyCapacity.max_dex_cap(), Some(0));

    assert_eq!(EncumbranceLevel::Light.armor_check_penalty(), 0);
    assert_eq!(EncumbranceLevel::Medium.armor_check_penalty(), -3);
    assert_eq!(EncumbranceLevel::Heavy.armor_check_penalty(), -6);
    assert_eq!(
        EncumbranceLevel::OverHeavyCapacity.armor_check_penalty(),
        -6,
        "carrying beyond the heavy maximum is at least as penalising as a heavy load"
    );
}

/// A real Strength-6 medium-load build must surface the load's own
/// penalties on the computation itself, not merely name the tier -- the
/// player-visible consequence of being encumbered.
#[test]
fn compute_encumbrance_reports_the_load_penalties_for_a_real_medium_load() {
    let corpus = corpus_from(FIXTURE_TEXT);
    // Strength 6: light 20, medium 40, heavy 60 (load.lst LOAD:6|60).
    // Chain Shirt (25) + Longsword (4) = 29 lbs -> Medium.
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 6);

    assert_eq!(computation.level, EncumbranceLevel::Medium);
    assert_eq!(computation.load_max_dex_cap, Some(3));
    assert_eq!(computation.load_armor_check_penalty, -3);
}

/// Carried *cost* rides along with carried weight: both come from the same
/// `equipment_tables()` entry the weight lookup already resolves, so a
/// loadout's total gp value needs no second resolution pass. Values are the
/// real `COST:` tokens on the fixture's own corpus records (Chain Shirt 100,
/// Longsword 15, Backpack 2 -- verbatim CRB).
#[test]
fn compute_encumbrance_totals_the_real_corpus_cost_of_the_carried_loadout() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
        selection("item:backpack", ActiveState::SelectedInactive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 14);

    assert_eq!(computation.total_carried_cost_gp, 100.0 + 15.0 + 2.0);
    let chain_shirt = computation
        .per_item
        .iter()
        .find(|item| item.item_id == "item:chain_shirt")
        .expect("the equipped Chain Shirt must appear in the per-item breakdown");
    assert_eq!(chain_shirt.weight_lbs, 25.0);
    assert_eq!(chain_shirt.cost_gp, Some(100.0), "the real CRB COST:100 token");
}

#[test]
fn compute_encumbrance_sums_a_different_real_item_set_than_the_inline_fixture() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
        selection("item:backpack", ActiveState::SelectedInactive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 14);

    assert_eq!(computation.total_carried_weight_lbs, 25.0 + 4.0 + 2.0);
    assert!(computation.unresolved_item_ids.is_empty(), "{:?}", computation.unresolved_item_ids);
    // Strength 14: light 58, medium 116, heavy 175 (load.lst LOAD:14|175).
    // 31 lbs total is well within the light threshold.
    assert_eq!(computation.level, EncumbranceLevel::Light);
}

#[test]
fn compute_encumbrance_returns_a_true_zero_for_an_entirely_absent_loadout() {
    let corpus = corpus_from(FIXTURE_TEXT);
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::Absent),
        selection("item:longsword", ActiveState::Absent),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 10);

    assert_eq!(computation.total_carried_weight_lbs, 0.0);
    assert!(computation.per_item.is_empty());
    assert!(computation.unresolved_item_ids.is_empty(), "Absent items must not be flagged unresolved");
    assert_eq!(computation.level, EncumbranceLevel::Light, "0 lbs is always within the light threshold");
}

#[test]
fn compute_encumbrance_reaches_medium_between_light_and_heavy_thresholds() {
    let corpus = corpus_from(FIXTURE_TEXT);
    // Strength 6: light 20, medium 40, heavy 60. Chain Shirt (25) + Longsword
    // (4) = 29 lbs, which is above the light max (20) but at/under the
    // medium max (40) -- the middle tier neither of encumbrance.rs's own
    // inline classification tests (Light, OverHeavyCapacity) exercises.
    let equipment_selections = vec![
        selection("item:chain_shirt", ActiveState::EquippedActive),
        selection("item:longsword", ActiveState::EquippedActive),
    ];

    let computation = compute_encumbrance(&equipment_selections, &corpus, 6);

    assert_eq!(computation.total_carried_weight_lbs, 29.0);
    assert_eq!(computation.level, EncumbranceLevel::Medium);
}
