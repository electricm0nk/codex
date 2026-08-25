//! ultimate_magic monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py ultimate_magic`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `um_abilities_race.lst` -- 13 monster-ability rows
//!
//! 13 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they SHIP with `owners: &[]` rather than being
//! dropped (`decisions.md §20`: an un-ingested row's shape cannot be measured,
//! and Gate 1's DoD needs every unit's shape measured). `list_monster_catalog`
//! only ever walks a monster's OWN `ability_keys`, so an owner-less record here
//! reaches no screen -- reachability is NOT claimed for these, and each key is
//! pinned as a named, provable non-reach in `reach_gate.rs::
//! UNREACHED_RECORD_FINDINGS`, never silently assumed reachable:
//!   * `um_abilities_race.lst:8`
//!   * `um_abilities_race.lst:9`
//!   * `um_abilities_race.lst:10`
//!   * `um_abilities_race.lst:11`
//!   * `um_abilities_race.lst:12`
//!   * `um_abilities_race.lst:13`
//!   * `um_abilities_race.lst:14`
//!   * `um_abilities_race.lst:15`
//!   * `um_abilities_race.lst:16`
//!   * `um_abilities_race.lst:17`
//!   * `um_abilities_race.lst:18`
//!   * `um_abilities_race.lst:19`
//!   * `um_abilities_race.lst:20`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, NaturalAttack, Speed, StatAdjustment};

/// Every ultimate_magic monster stat block (0 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
];

/// Every ultimate_magic monster-ability record (13 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Animated Object ~ Augmented Critical Range",
        name: "Augmented Critical (Range/1 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Increase threat range for melee attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 8,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Augmented Critical Multiplier",
        name: "Augmented Critical (Multiplier/1 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Increase critical multiplier for melee attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 9,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Exceptional Reach",
        name: "Exceptional Reach (One Attack/1 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Gain +5 feet of reach for one melee attack."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 10,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Exceptional Reach All",
        name: "Exceptional Reach (All Attacks/2 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Gain +5 feet of reach for all melee attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 11,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Improved Attack Melee",
        name: "Improved Attack (Melee/1 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("All melee attacks do damage as though the object were one size larger."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 12,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Improved Attack Ranged",
        name: "Improved Attack (Ranged/1 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("All ranged attacks do damage as though the object were one size larger."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 13,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Piercing Attack",
        name: "Piercing Attack (One Attack/1 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Replace one melee attack with an attack that does the same amount of piercing damage and has a x3 multiplier.  Object abilities that specify slam attacks do not work on piercing attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 14,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Piercing Attack All",
        name: "Piercing Attack (All Attacks/2 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Replace all melee attacks with an attack that does the same amount of piercing damage and has a x3 multiplier.  Object abilities that specify slam attacks do not work on piercing attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 15,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Slashing Attack",
        name: "Slashing Attack (One Attack/1 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Replace one slam attack with an attack that does slashing damage and has either a 19-20 threat range (for blade-like attacks) or a x3 threat multiplier (for axe- or scythe-like attacks).  Object abilities that specify slam attacks do not work on slashing attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 16,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Slashing Attack All",
        name: "Slashing Attack (All Attacks/2 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Replace all slam attacks with attacks that do slashing damage and have either a 19-20 threat range (for blade-like attacks) or a x3 threat multiplier (for axe- or scythe-like attacks).  Object abilities that specify slam attacks do not work on slashing attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 17,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Ranged Attack",
        name: "Ranged Attack (One Attack/2 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Replace one slam attack with a ranged attack.  It does the same amount of damage, and has a range of 20 feet.  Object abilities that specify slam attacks do not work on ranged attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 18,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Ranged Attack All",
        name: "Ranged Attack (All Attacks/4 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("Replace all slam attacks with ranged attacks.  They do the same amount of damage, and have a range of 20 feet.  Object abilities that specify slam attacks do not work on ranged attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 19,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Animated Object ~ Trip",
        name: "Trip (2 CP)",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["AnimatedObjectAbility"],
        description: Some("The object gains the trip special ability with one of its slam attacks."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &[],
        source_file: "um_abilities_race.lst",
        source_line: 20,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
];
