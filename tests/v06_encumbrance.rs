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
