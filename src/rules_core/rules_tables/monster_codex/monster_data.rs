//! monster_codex monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py monster_codex`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `mc_races.lst` -- 2 monster rows
//!   * `mc_abilities_race.lst` -- 3 monster-ability rows

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, NaturalAttack, Speed, StatAdjustment};

/// Every monster_codex monster stat block (2 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
    MonsterStatBlock {
        key: "Seru",
        name: "Seru",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 20 }, Speed { mode: "Fly", feet: 40 }],
        race_type: Some("Magical Beast"),
        race_subtype: None,
        challenge_rating: Some("3"),
        monster_class: Some("Magical Beast:3"),
        source_page: Some("p.208"),
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d6") }, NaturalAttack { name: "Venom", damage_dice: None }],
        ability_keys: &["Seru ~ Poison", "Seru ~ Spit Venom"],
        external_ability_refs: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -4 }, StatAdjustment { ability: "DEX", amount: 2 }, StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "WIS", amount: 2 }],
        has_spell_like_abilities: false,
        sla_cl_token: None,
        spell_like_abilities: &[],
        source_file: "mc_races.lst",
        source_line: 5,
    },
    MonsterStatBlock {
        key: "Bat (Sootwing)",
        name: "Sootwing Bat",
        size: Some("T"),
        speeds: &[Speed { mode: "Walk", feet: 5 }, Speed { mode: "Fly", feet: 40 }],
        race_type: Some("Undead"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: Some("Undead:2"),
        source_page: Some("p.88"),
        natural_attacks: &[NaturalAttack { name: "Bite", damage_dice: Some("1d3") }],
        ability_keys: &["Bat (Sootwing) ~ Disease"],
        external_ability_refs: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -6 }, StatAdjustment { ability: "DEX", amount: 2 }, StatAdjustment { ability: "INT", amount: -8 }, StatAdjustment { ability: "WIS", amount: 2 }, StatAdjustment { ability: "CHA", amount: -2 }],
        has_spell_like_abilities: false,
        sla_cl_token: None,
        spell_like_abilities: &[],
        source_file: "mc_races.lst",
        source_line: 6,
    },
];

/// Every monster_codex monster-ability record (3 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Bat (Sootwing) ~ Disease",
        name: "Disease",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &[],
        description: Some("Ghoul Fever; See Pathfinder RPG Bestiary entry on ghouls."),
        description_variables: &[],
        source_page: None,
        owners: &["Bat (Sootwing)"],
        source_file: "mc_abilities_race.lst",
        source_line: 71,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Seru ~ Poison",
        name: "Poison",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("Bite or spit venom-injury; save Fort DC 15; frequency 1/minute for 6 minutes; effect 1 Con damage plus blindness for 1 minute; cure 1 save."),
        description_variables: &[],
        source_page: None,
        owners: &["Seru"],
        source_file: "mc_abilities_race.lst",
        source_line: 85,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Seru ~ Spit Venom",
        name: "Spit Venom",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("As a standard action, a seru can spit venom up to 30 feet. This is a ranged touch attack with no range increment. Any opponent hit by this attack is exposed to the seru's poison."),
        description_variables: &[],
        source_page: None,
        owners: &["Seru"],
        source_file: "mc_abilities_race.lst",
        source_line: 86,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
];
