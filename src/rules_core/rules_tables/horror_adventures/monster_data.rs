//! horror_adventures monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py horror_adventures`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `ha_races.lst` -- 3 monster rows
//!   * `ha_abilities_race.lst` -- 6 monster-ability rows
//!
//! 65 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they are deliberately NOT transcribed (a record
//! with no owner loads and is never shown). `not-ingested` is their honest status
//! in the work inventory, and the round's receipt records them by key:
//!   * `ha_abilities_race.lst:204`
//!   * `ha_abilities_race.lst:207`
//!   * `ha_abilities_race.lst:209`
//!   * `ha_abilities_race.lst:210`
//!   * `ha_abilities_race.lst:211`
//!   * `ha_abilities_race.lst:212`
//!   * `ha_abilities_race.lst:213`
//!   * `ha_abilities_race.lst:243`
//!   * `ha_abilities_race.lst:246`
//!   * `ha_abilities_race.lst:247`
//!   * `ha_abilities_race.lst:251`
//!   * `ha_abilities_race.lst:252`
//!   * `ha_abilities_race.lst:253`
//!   * `ha_abilities_race.lst:254`
//!   * `ha_abilities_race.lst:256`
//!   * `ha_abilities_race.lst:257`
//!   * `ha_abilities_race.lst:258`
//!   * `ha_abilities_race.lst:259`
//!   * `ha_abilities_race.lst:260`
//!   * `ha_abilities_race.lst:261`
//!   * `ha_abilities_race.lst:262`
//!   * `ha_abilities_race.lst:263`
//!   * `ha_abilities_race.lst:266`
//!   * `ha_abilities_race.lst:273`
//!   * `ha_abilities_race.lst:274`
//!   * `ha_abilities_race.lst:289`
//!   * `ha_abilities_race.lst:290`
//!   * `ha_abilities_race.lst:291`
//!   * `ha_abilities_race.lst:292`
//!   * `ha_abilities_race.lst:293`
//!   * `ha_abilities_race.lst:294`
//!   * `ha_abilities_race.lst:297`
//!   * `ha_abilities_race.lst:298`
//!   * `ha_abilities_race.lst:299`
//!   * `ha_abilities_race.lst:300`
//!   * `ha_abilities_race.lst:301`
//!   * `ha_abilities_race.lst:302`
//!   * `ha_abilities_race.lst:305`
//!   * `ha_abilities_race.lst:308`
//!   * `ha_abilities_race.lst:311`
//!   * `ha_abilities_race.lst:312`
//!   * `ha_abilities_race.lst:313`
//!   * `ha_abilities_race.lst:314`
//!   * `ha_abilities_race.lst:315`
//!   * `ha_abilities_race.lst:318`
//!   * `ha_abilities_race.lst:321`
//!   * `ha_abilities_race.lst:324`
//!   * `ha_abilities_race.lst:325`
//!   * `ha_abilities_race.lst:326`
//!   * `ha_abilities_race.lst:329`
//!   * `ha_abilities_race.lst:330`
//!   * `ha_abilities_race.lst:331`
//!   * `ha_abilities_race.lst:332`
//!   * `ha_abilities_race.lst:333`
//!   * `ha_abilities_race.lst:334`
//!   * `ha_abilities_race.lst:337`
//!   * `ha_abilities_race.lst:338`
//!   * `ha_abilities_race.lst:339`
//!   * `ha_abilities_race.lst:340`
//!   * `ha_abilities_race.lst:341`
//!   * `ha_abilities_race.lst:342`
//!   * `ha_abilities_race.lst:345`
//!   * `ha_abilities_race.lst:346`
//!   * `ha_abilities_race.lst:347`
//!   * `ha_abilities_race.lst:348`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, NaturalAttack, Speed};

/// Every horror_adventures monster stat block (3 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
    MonsterStatBlock {
        key: "Hive Larva Swarm",
        name: "Hive Larva Swarm",
        size: Some("F"),
        speeds: &[Speed { mode: "Walk", feet: 10 }, Speed { mode: "Climb", feet: 10 }],
        race_type: Some("Aberration"),
        race_subtype: Some("Hive|Swarm"),
        challenge_rating: Some("5"),
        monster_class: Some("Aberration:7"),
        source_page: Some("p.236"),
        natural_attacks: &[NaturalAttack { name: "Swarm", damage_dice: Some("2d6") }],
        ability_keys: &["Hive Larva Swarm ~ Infest", "Hive Larva Swarm ~ Poison"],
        external_ability_refs: &[],
        source_file: "ha_races.lst",
        source_line: 3,
    },
    MonsterStatBlock {
        key: "Hive Queen",
        name: "Hive Queen",
        size: Some("H"),
        speeds: &[Speed { mode: "Walk", feet: 50 }, Speed { mode: "Climb", feet: 20 }],
        race_type: Some("Aberration"),
        race_subtype: Some("Hive"),
        challenge_rating: Some("10"),
        monster_class: Some("Aberration:15"),
        source_page: Some("p.236"),
        natural_attacks: &[NaturalAttack { name: "Claw", damage_dice: Some("1d10") }, NaturalAttack { name: "Bite", damage_dice: None }, NaturalAttack { name: "Tail Slap", damage_dice: None }],
        ability_keys: &["Hive Queen ~ Egg Layer", "Hive Queen ~ Telepathy"],
        external_ability_refs: &[],
        source_file: "ha_races.lst",
        source_line: 4,
    },
    MonsterStatBlock {
        key: "Hive Warrior",
        name: "Hive Warrior",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 20 }],
        race_type: Some("Aberration"),
        race_subtype: Some("Hive"),
        challenge_rating: Some("5"),
        monster_class: Some("Aberration:8"),
        source_page: Some("p.237"),
        natural_attacks: &[NaturalAttack { name: "Acid Spit", damage_dice: Some("7d6") }, NaturalAttack { name: "Bite", damage_dice: None }, NaturalAttack { name: "Claw", damage_dice: None }, NaturalAttack { name: "Tail Slap", damage_dice: None }],
        ability_keys: &["Hive Warrior ~ Acid Spit", "Hive Warrior ~ Rending Mandibles"],
        external_ability_refs: &[],
        source_file: "ha_races.lst",
        source_line: 5,
    },
];

/// Every horror_adventures monster-ability record (6 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Hive Larva Swarm ~ Infest",
        name: "Infest",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["HiveLarvaSwarm"],
        description: Some("A hive larva swarm can enter the body of a single Small or larger helpless living creature. When it does so, a portion of the swarm enters a creature's mouth and gestates for 1d4 minutes. The gestated larvae ravage the host for 24 hours, during which time the infested creature falls unconscious and can't be woken by any means. During this period the larvae are vulnerable to expulsion by remove disease (DC %1). If successful, the hive larvae die inside of the host, dealing another 1d6 points of acid damage. In some cases, metabolized hive larvae corpses can infect the creature with the hive corruption (see page 24). After 24 hours, the hive larvae achieve symbiosis. The host awakens feeling healthy but hungry. Spells such as diagnose disease and Heal checks used to detect disease no longer detect the hive infestation, and the host creature is now treated as both its original creature type and an aberration for purposes of spells and effects (whichever is worse). A successful DC 25 Knowledge (dungeoneering) check can identify the symbiosis. If the host dies at this point, the larval infestation dies as well. However, if the host is then returned from the dead, the larvae are returned to life as well. The swarm fully matures 2d12 hours after symbiosis. By this point, there is no way to save the host short of a miracle or wish spell. Each round for 2d12 rounds, the host suffers agonizing pain and must succeed at a Fortitude save (DC %1) or be nauseated for 1 round. On each failed save, the host takes 4d6 points of damage as the mature hive creature forcibly separates itself. At the end of the 2d12 rounds, or after the host is slain by the damage, a fully formed hive creature (usually a hive warrior) explodes from within, destroying the host's body."),
        description_variables: &["10+HD/2+CON"],
        source_page: Some("p.236"),
        owners: &["Hive Larva Swarm"],
        source_file: "ha_abilities_race.lst",
        source_line: 277,
    },
    MonsterAbilityRecord {
        key: "Hive Larva Swarm ~ Poison",
        name: "Poison",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["HiveLarvaSwarm"],
        description: Some("Swarm-injury; save Fort DC %1; frequency 1/round for 6 rounds; effect 1d4 Strength; cure 2 consecutive saves."),
        description_variables: &["HD+10+HD/2+CON"],
        source_page: Some("p.236"),
        owners: &["Hive Larva Swarm"],
        source_file: "ha_abilities_race.lst",
        source_line: 278,
    },
    MonsterAbilityRecord {
        key: "Hive Queen ~ Egg Layer",
        name: "Egg Layer",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["HiveQueen"],
        description: Some("Once per week, a hive queen can lay a cluster of eggs, which hatch into 2d6 hive larvae swarms after 1d4 days."),
        description_variables: &[],
        source_page: Some("p.237"),
        owners: &["Hive Queen"],
        source_file: "ha_abilities_race.lst",
        source_line: 281,
    },
    MonsterAbilityRecord {
        key: "Hive Queen ~ Telepathy",
        name: "Telepathy",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &["HiveQueen"],
        description: Some("A hive queen can communicate telepathically with any creature with the hive subtype within 1 mile. This telepathy conveys empathic concepts rather than true language."),
        description_variables: &[],
        source_page: Some("p.237"),
        owners: &["Hive Queen"],
        source_file: "ha_abilities_race.lst",
        source_line: 282,
    },
    MonsterAbilityRecord {
        key: "Hive Warrior ~ Acid Spit",
        name: "Acid Spit",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["HiveWarrior"],
        description: Some("A hive warrior can spray acid as a ranged touch attack out to a maximum range of 20 feet. Creatures struck by this spray takes %1d6 acid damage. Additionally, creatures damaged by a hive warrior's acid spit continue to take the same amount of acid damage for the next 1d3 rounds. A successful Reflex save (DC %2) halves the initial damage and negates the ongoing damage."),
        description_variables: &["HD", "10+HD/2+DEX"],
        source_page: Some("p.237"),
        owners: &["Hive Warrior"],
        source_file: "ha_abilities_race.lst",
        source_line: 285,
    },
    MonsterAbilityRecord {
        key: "Hive Warrior ~ Rending Mandibles",
        name: "Rending Mandibles",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["HiveWarrior"],
        description: Some("When a hive warrior confirms a critical hit with its bite, it deals equal damage to the creature's armor."),
        description_variables: &[],
        source_page: Some("p.237"),
        owners: &["Hive Warrior"],
        source_file: "ha_abilities_race.lst",
        source_line: 286,
    },
];
