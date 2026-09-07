//! horror_adventures companion tables, transcribed verbatim from the book's own
//! PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_companion_tables.py horror_adventures`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst`.
//!
//! Sources, with the file AND line each record was read from carried per row:
//!   * `ha_races_companion.lst` -- 1 companion creature rows
//!   * `ha_abilities_companion.lst` -- 1 companion ability rows

use crate::rules_core::rules_tables::companion_chassis::{CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord, NaturalAttack, Speed, StatAdjustment};

/// Every horror_adventures companion creature (1 rows).
pub(super) static COMPANIONS: &[CompanionRecord] = &[
    CompanionRecord {
        key: "Companion (Devolved Humanoid)",
        name: "Companion (Devolved Humanoid)",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 30 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &["Companion", "AnimalCompanion"],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: None }, NaturalAttack { name: "Claw", damage_dice: None }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 2 }, StatAdjustment { ability: "DEX", amount: 6 }, StatAdjustment { ability: "INT", amount: -8 }, StatAdjustment { ability: "WIS", amount: 2 }, StatAdjustment { ability: "CHA", amount: -4 }],
        natural_armor: Some(1),
        source_page: Some("p.50"),
        ability_keys: &["Companion Advancement ~ Devolved Humanoid"],
        external_ability_refs: &["Scent"],
        source_file: "ha_races_companion.lst",
        source_line: 3,
    },
];

/// Every horror_adventures companion ability record (1 rows).
pub(super) static COMPANION_ABILITIES: &[CompanionAbilityRecord] = &[
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Devolved Humanoid",
        name: "Companion Advancement (Devolved Humanoid)",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 8 }, StatAdjustment { ability: "DEX", amount: -2 }, StatAdjustment { ability: "CON", amount: 4 }],
        source_page: None,
        owners: &["Companion (Devolved Humanoid)"],
        cross_book_owners: &[],
        source_file: "ha_abilities_companion.lst",
        source_line: 3,
    },
];
