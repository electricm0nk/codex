//! ultimate_psionics monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py ultimate_psionics`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `up_races.lst` -- 21 monster rows
//!   * `up_abilities_race.lst` -- 13 monster-ability rows
//!
//! 66 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they are deliberately NOT transcribed (a record
//! with no owner loads and is never shown). `not-ingested` is their honest status
//! in the work inventory, and the round's receipt records them by key:
//!   * `up_abilities_race.lst:11`
//!   * `up_abilities_race.lst:12`
//!   * `up_abilities_race.lst:646`
//!   * `up_abilities_race.lst:647`
//!   * `up_abilities_race.lst:648`
//!   * `up_abilities_race.lst:649`
//!   * `up_abilities_race.lst:650`
//!   * `up_abilities_race.lst:651`
//!   * `up_abilities_race.lst:652`
//!   * `up_abilities_race.lst:653`
//!   * `up_abilities_race.lst:654`
//!   * `up_abilities_race.lst:655`
//!   * `up_abilities_race.lst:656`
//!   * `up_abilities_race.lst:657`
//!   * `up_abilities_race.lst:658`
//!   * `up_abilities_race.lst:659`
//!   * `up_abilities_race.lst:660`
//!   * `up_abilities_race.lst:661`
//!   * `up_abilities_race.lst:665`
//!   * `up_abilities_race.lst:666`
//!   * `up_abilities_race.lst:667`
//!   * `up_abilities_race.lst:668`
//!   * `up_abilities_race.lst:669`
//!   * `up_abilities_race.lst:670`
//!   * `up_abilities_race.lst:671`
//!   * `up_abilities_race.lst:672`
//!   * `up_abilities_race.lst:673`
//!   * `up_abilities_race.lst:674`
//!   * `up_abilities_race.lst:675`
//!   * `up_abilities_race.lst:676`
//!   * `up_abilities_race.lst:677`
//!   * `up_abilities_race.lst:678`
//!   * `up_abilities_race.lst:679`
//!   * `up_abilities_race.lst:680`
//!   * `up_abilities_race.lst:681`
//!   * `up_abilities_race.lst:682`
//!   * `up_abilities_race.lst:683`
//!   * `up_abilities_race.lst:684`
//!   * `up_abilities_race.lst:688`
//!   * `up_abilities_race.lst:689`
//!   * `up_abilities_race.lst:690`
//!   * `up_abilities_race.lst:691`
//!   * `up_abilities_race.lst:692`
//!   * `up_abilities_race.lst:693`
//!   * `up_abilities_race.lst:694`
//!   * `up_abilities_race.lst:695`
//!   * `up_abilities_race.lst:696`
//!   * `up_abilities_race.lst:697`
//!   * `up_abilities_race.lst:698`
//!   * `up_abilities_race.lst:699`
//!   * `up_abilities_race.lst:700`
//!   * `up_abilities_race.lst:701`
//!   * `up_abilities_race.lst:706`
//!   * `up_abilities_race.lst:707`
//!   * `up_abilities_race.lst:708`
//!   * `up_abilities_race.lst:709`
//!   * `up_abilities_race.lst:710`
//!   * `up_abilities_race.lst:713`
//!   * `up_abilities_race.lst:714`
//!   * `up_abilities_race.lst:715`
//!   * `up_abilities_race.lst:716`
//!   * `up_abilities_race.lst:719`
//!   * `up_abilities_race.lst:720`
//!   * `up_abilities_race.lst:726`
//!   * `up_abilities_race.lst:734`
//!   * `up_abilities_race.lst:735`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, NaturalAttack, Speed};

/// Every ultimate_psionics monster stat block (21 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
    MonsterStatBlock {
        key: "Blue",
        name: "Blue",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.9"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 14,
    },
    MonsterStatBlock {
        key: "Dromite",
        name: "Dromite",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 20 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.10"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 15,
    },
    MonsterStatBlock {
        key: "Duergar ~ Psionic",
        name: "Duergar ~ Psionic",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 20 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.12"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Light Sensitivity", "Psionic"],
        source_file: "up_races.lst",
        source_line: 16,
    },
    MonsterStatBlock {
        key: "Elan",
        name: "Elan",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.13"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 17,
    },
    MonsterStatBlock {
        key: "Forgeborn",
        name: "Forgeborn",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.15"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 18,
    },
    MonsterStatBlock {
        key: "Half-Giant",
        name: "Half-Giant",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.16"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 19,
    },
    MonsterStatBlock {
        key: "Maenad",
        name: "Maenad",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.18"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 20,
    },
    MonsterStatBlock {
        key: "Noral",
        name: "Noral",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.19"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 21,
    },
    MonsterStatBlock {
        key: "Ophiduan",
        name: "Ophiduan",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.21"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 22,
    },
    MonsterStatBlock {
        key: "Xeph",
        name: "Xeph",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Humanoid"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: None,
        source_page: Some("p.23"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &["Psionic"],
        source_file: "up_races.lst",
        source_line: 23,
    },
    MonsterStatBlock {
        key: "Astral Construct (1st Level)",
        name: "Astral Construct (1st Level)",
        size: Some("S"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("1/2"),
        monster_class: Some("Construct:1"),
        source_page: Some("p.444"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d4") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 33,
    },
    MonsterStatBlock {
        key: "Astral Construct (2nd Level)",
        name: "Astral Construct (2nd Level)",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("1"),
        monster_class: Some("Construct:2"),
        source_page: Some("p.445"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d6") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 34,
    },
    MonsterStatBlock {
        key: "Astral Construct (3rd Level)",
        name: "Astral Construct (3rd Level)",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("2"),
        monster_class: Some("Construct:3"),
        source_page: Some("p.445"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d6") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 35,
    },
    MonsterStatBlock {
        key: "Astral Construct (4th Level)",
        name: "Astral Construct (4th Level)",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("3"),
        monster_class: Some("Construct:5"),
        source_page: Some("p.445"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d6") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 36,
    },
    MonsterStatBlock {
        key: "Astral Construct (5th Level)",
        name: "Astral Construct (5th Level)",
        size: Some("L"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("5"),
        monster_class: Some("Construct:7"),
        source_page: Some("p.446"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d8") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 37,
    },
    MonsterStatBlock {
        key: "Astral Construct (6th Level)",
        name: "Astral Construct (6th Level)",
        size: Some("L"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("7"),
        monster_class: Some("Construct:10"),
        source_page: Some("p.446"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d8") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 38,
    },
    MonsterStatBlock {
        key: "Astral Construct (7th Level)",
        name: "Astral Construct (7th Level)",
        size: Some("L"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("8"),
        monster_class: Some("Construct:13"),
        source_page: Some("p.446"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d8") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 39,
    },
    MonsterStatBlock {
        key: "Astral Construct (8th Level)",
        name: "Astral Construct (8th Level)",
        size: Some("L"),
        speeds: &[Speed { mode: "Walk", feet: 40 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("9"),
        monster_class: Some("Construct:16"),
        source_page: Some("p.447"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("1d8") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 40,
    },
    MonsterStatBlock {
        key: "Astral Construct (9th Level)",
        name: "Astral Construct (9th Level)",
        size: Some("H"),
        speeds: &[Speed { mode: "Walk", feet: 50 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("10"),
        monster_class: Some("Construct:19"),
        source_page: Some("p.447"),
        natural_attacks: &[NaturalAttack { name: "Slam", damage_dice: Some("2d6") }],
        ability_keys: &[],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 41,
    },
    MonsterStatBlock {
        key: "Astral Swarm",
        name: "Astral Swarm",
        size: Some("D"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("7"),
        monster_class: Some("Construct:14"),
        source_page: Some("p.447"),
        natural_attacks: &[NaturalAttack { name: "Swarm", damage_dice: Some("4d6") }],
        ability_keys: &["Astral Swarm ~ Ectoplasmic Poison"],
        external_ability_refs: &["Distraction ~ Swarm", "No Combat Maneuvers", "Swarm Traits"],
        source_file: "up_races.lst",
        source_line: 44,
    },
    MonsterStatBlock {
        key: "Psicrystal",
        name: "Psicrystal",
        size: Some("D"),
        speeds: &[Speed { mode: "Walk", feet: 30 }, Speed { mode: "Climb", feet: 20 }],
        race_type: Some("Construct"),
        race_subtype: None,
        challenge_rating: Some("0"),
        monster_class: Some("Construct:1"),
        source_page: Some("p.48,448"),
        natural_attacks: &[],
        ability_keys: &["Psicrystal ~ Traits", "Psicrystal ~ Hardness", "Psicrystal ~ Skills", "Psicrystal ~ Self Propulsion", "Psicrystal ~ Telepathic Link", "Psicrystal ~ Share Powers", "Psicrystal ~ Deliver Touch Powers", "Psicrystal ~ Telepathic Speech", "Psicrystal ~ Power Resistance", "Psicrystal ~ Sight Link", "Psicrystal ~ Channel Power", "Psicrystal ~ Sighted"],
        external_ability_refs: &[],
        source_file: "up_races.lst",
        source_line: 47,
    },
];

/// Every ultimate_psionics monster-ability record (13 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Astral Swarm ~ Ectoplasmic Poison",
        name: "Ectoplasmic Poison",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("Swarm - injury; save Fort DC %1; frequency 1/round for 2 rounds; effect 1d3 Dexterity drain; cure 1 save."),
        description_variables: &["EctoplasmicSwarmPoisonDC"],
        source_page: Some("p.448"),
        owners: &["Astral Swarm"],
        source_file: "up_abilities_race.lst",
        source_line: 730,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Traits",
        name: "Construct Traits",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &[],
        description: Some("A psicrystal cannot heal damage, but it can be repaired. Psicrystals do not have the usual construct traits of darkvision and low-light vision."),
        description_variables: &[],
        source_page: Some("p.448"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 740,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Hardness",
        name: "Hardness",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &[],
        description: Some("A psicrystal has hardness 8."),
        description_variables: &[],
        source_page: Some("p.448"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 741,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Skills",
        name: "Skills",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &[],
        description: Some("A psicrystal has the same skill ranks as its owner, except that it has a minimum of 4 ranks each in Perception and Stealth."),
        description_variables: &[],
        source_page: Some("p.111"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 742,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Self Propulsion",
        name: "Self Propulsion",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &[],
        description: Some("The psicrystal can grow spidery legs that give it a move of 30', 20' climb."),
        description_variables: &[],
        source_page: Some("p.49"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 743,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Telepathic Link",
        name: "Telepathic Link",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &[],
        description: Some("Owner and psicrystal can communicate telepathically up to 1 mile away."),
        description_variables: &[],
        source_page: Some("p.49"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 744,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Share Powers",
        name: "Share Powers",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &[],
        description: Some("The owner can have any power that he manifests on himself affect the psicrystal, if the psicrystal is within 5 feet.   The owner can manifest a power with Target:You on the Psicrystal instead."),
        description_variables: &[],
        source_page: Some("p.49"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 745,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Deliver Touch Powers",
        name: "Deliver Touch Powers",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &[],
        description: Some("The psicrystal can deliver a touch power if the psicrystal is touching the owner when the owner manifests the power."),
        description_variables: &[],
        source_page: Some("p.50"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 746,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Telepathic Speech",
        name: "Telepathic Speech",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("The psicrystal can communicate with any creature that has a language within 30 feet, if the psicrystal is also within 1 mile of its owner."),
        description_variables: &[],
        source_page: Some("p.50"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 747,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Power Resistance",
        name: "Power Resistance",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("The psicrystal has power resistance %1."),
        description_variables: &["SR"],
        source_page: Some("p.50"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 748,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Sight Link",
        name: "Sight Link",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::SpellLike),
        traits: &[],
        description: Some("The owner can remote view the psicrystal 1/day."),
        description_variables: &[],
        source_page: Some("p.50"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 749,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Channel Power",
        name: "Channel Power",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: Some(MonsterAbilityDelivery::SpellLike),
        traits: &[],
        description: Some("The owner can manifest powers through the psicrystal if the psicrystal is up to 1 mile away."),
        description_variables: &[],
        source_page: Some("p.50"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 750,
    },
    MonsterAbilityRecord {
        key: "Psicrystal ~ Sighted",
        name: "Sighted",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("The psicrystal can sense its environment as though it can see and hear, even in magical darkness and silence."),
        description_variables: &[],
        source_page: Some("p.49"),
        owners: &["Psicrystal"],
        source_file: "up_abilities_race.lst",
        source_line: 751,
    },
];
