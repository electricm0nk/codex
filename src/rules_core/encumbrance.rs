//! Carrying capacity / encumbrance (v0.6 alpha swarm, task 5).
//!
//! Before this file, carrying capacity and encumbrance were a complete gap
//! (`character_input.rs`'s own `ActiveState` doc comment: "no equipment
//! effect, encumbrance, or inventory behavior is computed here"; confirmed
//! by grep across `src/rules_core/` -- every prior "encumbrance"/"carrying
//! capacity" hit was a doc-comment disclaimer or corpus flavor text, never
//! computation). `equipment_effects.rs`'s per-item stats (AC bonus, max
//! Dex, spell failure) were already real and wired before this task; this
//! file adds the missing weight/capacity pillar alongside it, following the
//! same corpus-resolution pattern (`equipment_id_resolve` +
//! `rules_tables::crb::equipment_tables()` lookup by key).
//!
//! PF1's Table: Carrying Capacity (Strength score -> light/medium/heavy
//! load maximum weight in pounds) is not present anywhere in this repo's
//! ingested corpus or `rules_tables` -- it is core-rulebook prose/table
//! content, not a `.lst` token stream. Per this codebase's own established
//! "cited web second-source pass" convention for exactly this situation
//! (`docs/architecture/status.md`'s equipment/spell `description` field
//! note: "every value identity-matched from aonprd.com/d20pfsrd.com"), the
//! table below is transcribed verbatim from Archives of Nethys's SRD
//! mirror: <https://www.aonprd.com/Rules.aspx?ID=118> ("Carrying
//! Capacity"), fetched 2026-07-23. Each row is the *maximum* weight for
//! that load tier (the top of the Light/Medium/Heavy ranges the source
//! table prints), for Strength scores 1-29; Strength scores above 29
//! extrapolate via the source's own stated rule ("multiply the numbers in
//! [the closest 20-29 row with the same ones digit] by 4 for every 10
//! points the creature's Strength is above" that row).

use crate::rules_core::character_input::{ActiveState, EquipmentSelection};
use crate::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use crate::rules_core::rules_tables::crb::equipment_tables::equipment_tables;
use crate::rules_core::rules_tables::RuleSetId;
use crate::rules_core::source_content::SourcePackageContent;

/// Max light/medium/heavy load in pounds for one Strength score, per PF1's
/// Table: Carrying Capacity (see module doc comment for the cited source).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarryingCapacityThresholds {
    pub light_max_lbs: f64,
    pub medium_max_lbs: f64,
    pub heavy_max_lbs: f64,
}

/// Row `i` (0-indexed) is Strength score `i + 1`'s (light, medium, heavy)
/// maximum load in pounds, verbatim from the cited source table (Strength
/// 1-29). See module doc comment for the citation.
const CARRYING_CAPACITY_TABLE: [(f64, f64, f64); 29] = [
    (3.0, 6.0, 10.0),
    (6.0, 13.0, 20.0),
    (10.0, 20.0, 30.0),
    (13.0, 26.0, 40.0),
    (16.0, 33.0, 50.0),
    (20.0, 40.0, 60.0),
    (23.0, 46.0, 70.0),
    (26.0, 53.0, 80.0),
    (30.0, 60.0, 90.0),
    (33.0, 66.0, 100.0),
    (38.0, 76.0, 115.0),
    (43.0, 86.0, 130.0),
    (50.0, 100.0, 150.0),
    (58.0, 116.0, 175.0),
    (66.0, 134.0, 200.0),
    (76.0, 153.0, 230.0),
    (86.0, 173.0, 260.0),
    (100.0, 200.0, 300.0),
    (116.0, 233.0, 350.0),
    (133.0, 266.0, 400.0),
    (153.0, 306.0, 460.0),
    (173.0, 346.0, 520.0),
    (200.0, 400.0, 600.0),
    (233.0, 466.0, 700.0),
    (266.0, 533.0, 800.0),
    (306.0, 613.0, 920.0),
    (346.0, 693.0, 1040.0),
    (400.0, 800.0, 1200.0),
    (466.0, 933.0, 1400.0),
];

/// PF1 Table: Carrying Capacity, Strength score -> max light/medium/heavy
/// load. Strength scores below 1 are defensively floored to the Strength-1
/// row (PF1 does not define a player character's carrying capacity below
/// 1). Strength scores above 29 extrapolate via the source table's own
/// stated rule: find the 20-29 row sharing the same "ones" digit and
/// multiply its three values by 4 for every 10 points above that row's
/// Strength.
pub fn carrying_capacity_thresholds(strength_score: i16) -> CarryingCapacityThresholds {
    let clamped = strength_score.max(1);
    let (light, medium, heavy) = if clamped <= 29 {
        CARRYING_CAPACITY_TABLE[(clamped - 1) as usize]
    } else {
        let ones_digit = ((clamped - 20) % 10 + 10) % 10;
        let base_strength = 20 + ones_digit;
        let (base_light, base_medium, base_heavy) =
            CARRYING_CAPACITY_TABLE[(base_strength - 1) as usize];
        let tens_above = f64::from((clamped - base_strength) / 10);
        let multiplier = 4.0_f64.powf(tens_above);
        (base_light * multiplier, base_medium * multiplier, base_heavy * multiplier)
    };
    CarryingCapacityThresholds { light_max_lbs: light, medium_max_lbs: medium, heavy_max_lbs: heavy }
}

/// PF1's three named load tiers, plus a fourth honest state for weight
/// beyond even the heavy maximum (PF1: a character carrying more than its
/// heavy load "can't move" -- this crate does not model movement, so this
/// variant only names the state rather than computing any further effect).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncumbranceLevel {
    Light,
    Medium,
    Heavy,
    OverHeavyCapacity,
}

fn classify_encumbrance(total_weight_lbs: f64, thresholds: &CarryingCapacityThresholds) -> EncumbranceLevel {
    if total_weight_lbs <= thresholds.light_max_lbs {
        EncumbranceLevel::Light
    } else if total_weight_lbs <= thresholds.medium_max_lbs {
        EncumbranceLevel::Medium
    } else if total_weight_lbs <= thresholds.heavy_max_lbs {
        EncumbranceLevel::Heavy
    } else {
        EncumbranceLevel::OverHeavyCapacity
    }
}

/// One resolved carried item's contribution to total weight.
#[derive(Debug, Clone, PartialEq)]
pub struct CarriedItemWeight {
    pub item_id: String,
    pub weight_lbs: f64,
}

/// Real, corpus-grounded carrying-capacity/encumbrance computation for a
/// character's full carried loadout (v0.6 alpha swarm, task 5).
///
/// Weight is summed across every selection with `ActiveState::EquippedActive`
/// or `ActiveState::SelectedInactive` (both represent items the character
/// actually possesses/carries; `Absent` means not carried at all -- mirrors
/// `ActiveState`'s own doc comment). Each selection is resolved to a real
/// corpus record via the same `equipment_id_resolve` + `equipment_tables()`
/// key lookup `equipment_effects::compute_equipment_effects` already
/// establishes, then that record's `weight_lbs` (parsed from the corpus's
/// own `WT:` token, `rules_tables::crb::equipment_tables`) is added to the
/// total. An item that does not resolve to a corpus record, or resolves but
/// carries no `weight_lbs` value, is recorded in `unresolved_item_ids`
/// rather than silently contributing a fabricated zero -- so a caller can
/// tell "this loadout weighs exactly 0 lbs" apart from "this loadout's
/// weight could not be fully verified."
#[derive(Debug, Clone, PartialEq)]
pub struct EncumbranceComputation {
    pub per_item: Vec<CarriedItemWeight>,
    pub total_carried_weight_lbs: f64,
    pub thresholds: CarryingCapacityThresholds,
    pub level: EncumbranceLevel,
    pub unresolved_item_ids: Vec<String>,
}

pub fn compute_encumbrance(
    equipment_selections: &[EquipmentSelection],
    corpus: &SourcePackageContent,
    strength_score: i16,
) -> EncumbranceComputation {
    let mut per_item = Vec::new();
    let mut unresolved_item_ids = Vec::new();
    let mut total_carried_weight_lbs = 0.0_f64;

    for selection in equipment_selections {
        if selection.active_state == ActiveState::Absent {
            continue;
        }

        let Some((record, _table_cell)) =
            equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)
        else {
            unresolved_item_ids.push(selection.item_id.clone());
            continue;
        };
        let key = equipment_key_token(record).unwrap_or(&record.name).to_string();
        let weight_lbs = equipment_tables()
            .iter()
            .find(|entry| entry.key == key)
            .and_then(|entry| entry.weight_lbs);

        let Some(weight_lbs) = weight_lbs else {
            unresolved_item_ids.push(selection.item_id.clone());
            continue;
        };

        total_carried_weight_lbs += weight_lbs;
        per_item.push(CarriedItemWeight { item_id: selection.item_id.clone(), weight_lbs });
    }

    let thresholds = carrying_capacity_thresholds(strength_score);
    let level = classify_encumbrance(total_carried_weight_lbs, &thresholds);

    EncumbranceComputation {
        per_item,
        total_carried_weight_lbs,
        thresholds,
        level,
        unresolved_item_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcgen_import::ir_converter::convert_equipment_record;
    use crate::pcgen_import::lst_parser::equipment::{parse_equipment_entries, EquipmentRecord};
    use crate::rules_core::source_content::SourceRef;

    /// Real verbatim rows mirroring `tests/sd20_equipment_arms_armor.rs`'s
    /// own fixture (same three CRB records, same `WT:` values) -- kept
    /// in-module rather than shared so this file has no dependency on
    /// `tests/**`, which this task does not own.
    const FIXTURE_TEXT: &str = "\
Leather Armor\tKEY:Leather Armor (Base)\tTYPE:Armor.Light\tCOST:10\tWT:15\tACCHECK:0\tMAXDEX:6\tSPELLFAILURE:10\tBONUS:COMBAT|AC|2|TYPE=Armor|PREVAREQ:DisableArmorBonus,0
Buckler\tKEY:Buckler (Base)\tTYPE:Shield.Buckler\tCOST:5\tWT:5\tACCHECK:-1\tSPELLFAILURE:5\tBONUS:COMBAT|AC|1|TYPE=Shield|PREVAREQ:DisableShieldBonus,0
Longsword\tKEY:Longsword (Base)\tTYPE:Weapon.Melee.Martial\tCOST:15\tWT:4\tCRITMULT:x2\tCRITRANGE:2\tDAMAGE:1d8
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

    #[test]
    fn carrying_capacity_thresholds_match_the_cited_table_at_known_rows() {
        assert_eq!(
            carrying_capacity_thresholds(10),
            CarryingCapacityThresholds { light_max_lbs: 33.0, medium_max_lbs: 66.0, heavy_max_lbs: 100.0 }
        );
        assert_eq!(
            carrying_capacity_thresholds(18),
            CarryingCapacityThresholds { light_max_lbs: 100.0, medium_max_lbs: 200.0, heavy_max_lbs: 300.0 }
        );
        assert_eq!(
            carrying_capacity_thresholds(29),
            CarryingCapacityThresholds { light_max_lbs: 466.0, medium_max_lbs: 933.0, heavy_max_lbs: 1400.0 }
        );
    }

    /// Strength 30 = Strength 20's row (same ones digit, one tier of +10
    /// above) multiplied by 4 -- the source table's own extrapolation rule.
    #[test]
    fn carrying_capacity_thresholds_extrapolate_beyond_strength_29() {
        assert_eq!(
            carrying_capacity_thresholds(30),
            CarryingCapacityThresholds { light_max_lbs: 133.0 * 4.0, medium_max_lbs: 266.0 * 4.0, heavy_max_lbs: 400.0 * 4.0 }
        );
    }

    #[test]
    fn compute_encumbrance_sums_real_corpus_weight_for_carried_items() {
        let corpus = corpus_from(FIXTURE_TEXT);
        let equipment_selections = vec![
            selection("item:leather_armor", ActiveState::EquippedActive),
            selection("item:buckler", ActiveState::SelectedInactive),
            selection("item:longsword", ActiveState::EquippedActive),
        ];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10);

        assert_eq!(computation.total_carried_weight_lbs, 15.0 + 5.0 + 4.0);
        assert!(computation.unresolved_item_ids.is_empty(), "{:?}", computation.unresolved_item_ids);
        assert_eq!(computation.per_item.len(), 3);
        assert_eq!(computation.thresholds, carrying_capacity_thresholds(10));
        // 24 lbs total is within Strength 10's light max (33 lbs).
        assert_eq!(computation.level, EncumbranceLevel::Light);
    }

    #[test]
    fn compute_encumbrance_excludes_absent_selections_and_flags_unresolvable_ones() {
        let corpus = corpus_from(FIXTURE_TEXT);
        let equipment_selections = vec![
            selection("item:leather_armor", ActiveState::EquippedActive),
            selection("item:buckler", ActiveState::Absent),
            selection("item:not_a_real_item", ActiveState::EquippedActive),
        ];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 10);

        assert_eq!(computation.total_carried_weight_lbs, 15.0, "Absent items must not contribute weight");
        assert_eq!(computation.unresolved_item_ids, vec!["item:not_a_real_item".to_owned()]);
    }

    #[test]
    fn compute_encumbrance_classifies_medium_and_heavy_loads() {
        let corpus = corpus_from(FIXTURE_TEXT);
        // Strength 1: light max 3, medium max 6, heavy max 10. Leather Armor
        // alone (15 lbs) exceeds even the heavy max.
        let equipment_selections = vec![selection("item:leather_armor", ActiveState::EquippedActive)];

        let computation = compute_encumbrance(&equipment_selections, &corpus, 1);

        assert_eq!(computation.level, EncumbranceLevel::OverHeavyCapacity);
    }
}
