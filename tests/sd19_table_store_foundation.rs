//! SD-19 foundation slice: canonical Paizo-table store structural proof.
//!
//! This is the RED/GREEN test for atomic commit 1 (foundation slice) per
//! `SD-19-core-rules-spell-equipment-reachability-scope-draft.md` §1.0.
//! It asserts the CRB table store is present and structurally complete —
//! it does not assert exhaustive corpus coverage (that is the loop's job,
//! one school/category per cycle, per scope-draft.md §2.4/§2.5).

use codex::rules_core::rules_tables::crb::class_tables::{class_tables, ClassId};
use codex::rules_core::rules_tables::crb::equipment_tables::{
    EquipmentCategory, EQUIPMENT_TABLES,
};
use codex::rules_core::rules_tables::crb::spell_list::{Pf1SchoolId, SPELL_LIST};
use codex::rules_core::rules_tables::RuleSetId;

#[test]
fn rule_set_id_crb_resolves() {
    let rule_set = RuleSetId::Crb;
    assert_eq!(rule_set, RuleSetId::Crb);
}

#[test]
fn every_class_table_has_entries_for_every_supported_level() {
    let all_rows = class_tables();
    for class_id in ClassId::ALL {
        let rows: Vec<_> = all_rows
            .iter()
            .filter(|row| row.class_id == *class_id)
            .collect();
        assert!(
            !rows.is_empty(),
            "class {:?} has no rows in CLASS_TABLES",
            class_id
        );
        let max_level = rows.iter().map(|row| row.level).max().unwrap();
        for level in 1..=max_level {
            assert!(
                rows.iter().any(|row| row.level == level),
                "class {:?} is missing a row for level {} (max supported {})",
                class_id,
                level,
                max_level
            );
        }
    }
}

#[test]
fn every_school_has_at_least_one_spell() {
    for school in Pf1SchoolId::ALL {
        let count = SPELL_LIST.iter().filter(|s| s.school == *school).count();
        assert!(count >= 1, "school {:?} has zero spells in SPELL_LIST", school);
    }
}

#[test]
fn every_equipment_category_is_non_empty() {
    for category in EquipmentCategory::ALL {
        let count = EQUIPMENT_TABLES
            .iter()
            .filter(|e| e.category == *category)
            .count();
        assert!(
            count >= 1,
            "equipment category {:?} has zero items in EQUIPMENT_TABLES",
            category
        );
    }
}
