//! advanced_race_guide companion tables, transcribed verbatim from the book's own
//! PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_companion_tables.py advanced_race_guide`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst`.
//!
//! Sources, with the file AND line each record was read from carried per row:
//!   * `arg_races_companion.lst` -- 7 companion creature rows
//!   * `arg_abilities_companion.lst` -- 7 companion ability rows
//!
//! NOT transcribed -- ability rows no creature row of this book owns, so
//! nothing could ever reach them on screen. Dropped rather than emitted
//! unreachable (`decisions.md §50`, adopted from the monster lane; §56.1).
//! These rows keep their `engine-does-not-hold` status in
//! `docs/work-inventory.json`, which is where the shortfall is counted; they
//! are NOT a `reach_gate` `OPEN_FINDINGS` entry, because that list is keyed by
//! FAMILY and this book's `companions` family does reach a player:
//!   * `Evolution ~ Major Glitterdust 1`
//!   * `Evolution ~ Major Glitterdust 3`
//!   * `Evolution ~ Major Soften Earth and Stone 1`
//!   * `Evolution ~ Major Soften Earth and Stone 3`
//!   * `Evolution ~ Shadow Blend`
//!   * `Evolution ~ Shadow Form`
//!   * `Evolution ~ Stone Curse`
//!   * `Evolution ~ Ultimate Meld Into Stone 1`
//!   * `Evolution ~ Ultimate Stone Shape 1`
//!   * `Shaitan Binder Eidolon ~ Charisma Bonus`
//!   * `Shaitan Binder Eidolon ~ Constitution Bonus`
//!   * `Shaitan Binder Eidolon ~ Dexterity Bonus`
//!   * `Shaitan Binder Eidolon ~ Earth Glide`
//!   * `Shaitan Binder Eidolon ~ Intelligence Bonus`
//!   * `Shaitan Binder Eidolon ~ Noble Eidolon`
//!   * `Shaitan Binder Eidolon ~ Strength Bonus`
//!   * `Shaitan Binder Eidolon ~ Wisdom Bonus`
//!   * `WCEvolution ~ Skilled`

use crate::rules_core::rules_tables::companion_chassis::{CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord, NaturalAttack, NaturalAttackDamageBonus, SkillAbilityDiffBonus, Speed, StatAdjustment};

/// Every advanced_race_guide companion creature (7 rows).
pub(super) static COMPANIONS: &[CompanionRecord] = &[
    CompanionRecord {
        key: "Familiar (Parrot)",
        name: "Familiar (Parrot)",
        size: Some("T"),
        speeds: &[Speed { mode: "Walk", feet: 10 }, Speed { mode: "Fly", feet: 40 }],
        reach_feet: Some(0),
        race_type: Some("Animal"),
        race_subtype: Some("Familiar|Augmented Magical Beast|FamiliarBase"),
        monster_class: Some("Animal:1"),
        type_segments: &["Companion", "Familiar"],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d3") }],
        natural_attack_damage_bonuses: &[NaturalAttackDamageBonus { attack: "Claw", formula: "max(0,(STR/2))" }],
        skill_ability_diff_bonuses: &[SkillAbilityDiffBonus { skills: &["Climb", "Swim"], formula: "DEX-STR" }],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -8 }, StatAdjustment { ability: "DEX", amount: 4 }, StatAdjustment { ability: "CON", amount: -2 }, StatAdjustment { ability: "INT", amount: -8 }, StatAdjustment { ability: "WIS", amount: 4 }, StatAdjustment { ability: "CHA", amount: -4 }],
        natural_armor: None,
        source_page: Some("p.133"),
        ability_keys: &[],
        external_ability_refs: &["Flight Maneuverability"],
        source_file: "arg_races_companion.lst",
        source_line: 9,
    },
    CompanionRecord {
        key: "Companion (Carnivorous Flower)",
        name: "Companion (Carnivorous Flower)",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 10 }],
        reach_feet: Some(5),
        race_type: Some("Plant"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &["Companion", "PlantCompanion"],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d6") }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[],
        natural_armor: Some(2),
        source_page: Some("p.26"),
        ability_keys: &["Companion Advancement ~ Carnivorous Flower"],
        external_ability_refs: &["Scent"],
        source_file: "arg_races_companion.lst",
        source_line: 14,
    },
    CompanionRecord {
        key: "Companion (Crawling Vine)",
        name: "Companion (Crawling Vine)",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 20 }, Speed { mode: "Climb", feet: 20 }],
        reach_feet: Some(5),
        race_type: Some("Plant"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &["Companion", "PlantCompanion"],
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d4") }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[],
        natural_armor: Some(2),
        source_page: Some("p.26"),
        ability_keys: &["Companion Advancement ~ Crawling Vine", "Crawling Vine ~ Constrict"],
        external_ability_refs: &["Grab", "Scent"],
        source_file: "arg_races_companion.lst",
        source_line: 15,
    },
    CompanionRecord {
        key: "Companion (Puffball)",
        name: "Companion (Puffball)",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 20 }, Speed { mode: "Fly", feet: 60 }],
        reach_feet: Some(5),
        race_type: Some("Plant"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &["Companion", "PlantCompanion"],
        natural_attacks: &[NaturalAttack { name: "Thorn", damage_dice: Some("1d4") }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[],
        natural_armor: Some(1),
        source_page: Some("p.26"),
        ability_keys: &["Puffball ~ Poison", "Companion Advancement ~ Puffball"],
        external_ability_refs: &["Flight Maneuverability"],
        source_file: "arg_races_companion.lst",
        source_line: 16,
    },
    CompanionRecord {
        key: "Companion (Sapling Treant)",
        name: "Companion (Sapling Treant)",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 30 }],
        reach_feet: Some(5),
        race_type: Some("Plant"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &["Companion", "PlantCompanion"],
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d6") }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[],
        natural_armor: Some(1),
        source_page: Some("p.26"),
        ability_keys: &["Companion Advancement ~ Sapling Treant", "Sapling Treant ~ Double Damage"],
        external_ability_refs: &[],
        source_file: "arg_races_companion.lst",
        source_line: 17,
    },
    CompanionRecord {
        key: "Brute Steed (Camel)",
        name: "Brute Steed (Camel)",
        size: Some("L"),
        speeds: &[Speed { mode: "Walk", feet: 50 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &["Companion", "AnimalCompanion"],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: None }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 10 }, StatAdjustment { ability: "DEX", amount: 4 }, StatAdjustment { ability: "CON", amount: 4 }, StatAdjustment { ability: "INT", amount: -8 }, StatAdjustment { ability: "CHA", amount: -6 }],
        natural_armor: Some(1),
        source_page: Some("p.56"),
        ability_keys: &[],
        external_ability_refs: &["Camel ~ Spit", "Scent"],
        source_file: "arg_races_companion.lst",
        source_line: 29,
    },
    CompanionRecord {
        key: "Brute Steed (Horse)",
        name: "Brute Steed (Horse)",
        size: Some("L"),
        speeds: &[Speed { mode: "Walk", feet: 50 }],
        reach_feet: Some(5),
        race_type: Some("Companion"),
        race_subtype: None,
        monster_class: Some("Companion:2"),
        type_segments: &["Companion", "AnimalCompanion"],
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: None }, NaturalAttack { name: "Hoof", damage_dice: None }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 8 }, StatAdjustment { ability: "CON", amount: 4 }, StatAdjustment { ability: "INT", amount: -8 }, StatAdjustment { ability: "WIS", amount: 2 }, StatAdjustment { ability: "CHA", amount: -4 }],
        natural_armor: Some(4),
        source_page: Some("p.56"),
        ability_keys: &[],
        external_ability_refs: &["Scent"],
        source_file: "arg_races_companion.lst",
        source_line: 30,
    },
];

/// Every advanced_race_guide companion ability record (7 rows).
pub(super) static COMPANION_ABILITIES: &[CompanionAbilityRecord] = &[
    CompanionAbilityRecord {
        key: "Puffball ~ Poison",
        name: "Poison",
        facet: None,
        delivery: None,
        type_segments: &["RaceAbility", "SpecialAbility"],
        description: Some("(Frequency 1 round [6], Effect 1 Con damage, Cure 1 save, Con-based DC; DC %1)."),
        description_variables: &["PuffballPoisonDC"],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Puffball)"],
        source_file: "arg_abilities_companion.lst",
        source_line: 9,
    },
    CompanionAbilityRecord {
        key: "Sapling Treant ~ Double Damage",
        name: "Double Damage",
        facet: None,
        delivery: None,
        type_segments: &["RaceAbility", "SpecialAbility"],
        description: Some("Sapling treant attacks do double damage against objects"),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Sapling Treant)"],
        source_file: "arg_abilities_companion.lst",
        source_line: 10,
    },
    CompanionAbilityRecord {
        key: "Crawling Vine ~ Constrict",
        name: "Constrict",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: None,
        type_segments: &["RaceAbility", "SpecialAttack", "SpecialAbility"],
        description: Some("1d6 bludgeoning with a successful grapple check"),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Companion (Crawling Vine)"],
        source_file: "arg_abilities_companion.lst",
        source_line: 11,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Carnivorous Flower",
        name: "Companion Advancement (Carnivorous Flower)",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 4 }, StatAdjustment { ability: "DEX", amount: -2 }, StatAdjustment { ability: "CON", amount: 2 }],
        source_page: None,
        owners: &["Companion (Carnivorous Flower)"],
        source_file: "arg_abilities_companion.lst",
        source_line: 56,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Crawling Vine",
        name: "Companion Advancement (Crawling Vine)",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 8 }, StatAdjustment { ability: "DEX", amount: -2 }, StatAdjustment { ability: "CON", amount: 4 }],
        source_page: None,
        owners: &["Companion (Crawling Vine)"],
        source_file: "arg_abilities_companion.lst",
        source_line: 57,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Puffball",
        name: "Companion Advancement (Puffball)",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 2 }, StatAdjustment { ability: "CON", amount: 2 }],
        source_page: None,
        owners: &["Companion (Puffball)"],
        source_file: "arg_abilities_companion.lst",
        source_line: 58,
    },
    CompanionAbilityRecord {
        key: "Companion Advancement ~ Sapling Treant",
        name: "Companion Advancement (Sapling Treant)",
        facet: Some(CompanionAbilityFacet::CompanionAdvancement),
        delivery: None,
        type_segments: &["CompanionAdvancement"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: 8 }, StatAdjustment { ability: "CON", amount: 4 }, StatAdjustment { ability: "DEX", amount: -2 }],
        source_page: None,
        owners: &["Companion (Sapling Treant)"],
        source_file: "arg_abilities_companion.lst",
        source_line: 59,
    },
];
