//! advanced_players_guide companion tables, transcribed verbatim from the book's own
//! PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_companion_tables.py advanced_players_guide`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst`.
//!
//! Sources, with the file AND line each record was read from carried per row:
//!   * `apg_races_companion.lst` -- 1 companion creature rows
//!   * `apg_abilities_companion.lst` -- 3 companion ability rows
//!
//! NOT transcribed -- ability rows no creature row of this book owns, so
//! nothing could ever reach them on screen. Dropped rather than emitted
//! unreachable (`decisions.md §50`, adopted from the monster lane; §56.1).
//! These rows keep their `not-ingested` status in
//! `docs/work-inventory.json`, which is where the shortfall is counted; they
//! are NOT a `reach_gate` `OPEN_FINDINGS` entry, because that list is keyed by
//! FAMILY and this book's `companions` family does reach a player:
//!   * `Companion Bonus Skill`
//!   * `Eidolon Bonus Skill`
//!   * `Evolution ~ Ability Increase Cha`
//!   * `Evolution ~ Ability Increase Con LH`
//!   * `Evolution ~ Ability Increase Con SM`
//!   * `Evolution ~ Ability Increase Dex`
//!   * `Evolution ~ Ability Increase Int`
//!   * `Evolution ~ Ability Increase Str LH`
//!   * `Evolution ~ Ability Increase Str SM`
//!   * `Evolution ~ Ability Increase Wis`
//!   * `Evolution ~ Arms`
//!   * `Evolution ~ Aspect`
//!   * `Evolution ~ Aspect Greater`
//!   * `Evolution ~ Bite`
//!   * `Evolution ~ Blindsense`
//!   * `Evolution ~ Blindsight`
//!   * `Evolution ~ Breath Weapon (Cone of Acid)`
//!   * `Evolution ~ Breath Weapon (Cone of Cold)`
//!   * `Evolution ~ Breath Weapon (Cone of Electricity)`
//!   * `Evolution ~ Breath Weapon (Cone of Fire)`
//!   * `Evolution ~ Breath Weapon (Line of Acid)`
//!   * `Evolution ~ Breath Weapon (Line of Cold)`
//!   * `Evolution ~ Breath Weapon (Line of Electricity)`
//!   * `Evolution ~ Breath Weapon (Line of Fire)`
//!   * `Evolution ~ Burrow`
//!   * `Evolution ~ Clawed Feet`
//!   * `Evolution ~ Clawed Hands`
//!   * `Evolution ~ Climb`
//!   * `Evolution ~ Constrict`
//!   * `Evolution ~ DR Chaotic`
//!   * `Evolution ~ DR Evil`
//!   * `Evolution ~ DR Good`
//!   * `Evolution ~ DR Lawful`
//!   * `Evolution ~ Energy Attack Acid`
//!   * `Evolution ~ Energy Attack Cold`
//!   * `Evolution ~ Energy Attack Electricity`
//!   * `Evolution ~ Energy Attack Fire`
//!   * `Evolution ~ Extra Breath Weapon Cone Acid`
//!   * `Evolution ~ Extra Breath Weapon Cone Cold`
//!   * `Evolution ~ Extra Breath Weapon Cone Electricity`
//!   * `Evolution ~ Extra Breath Weapon Cone Fire`
//!   * `Evolution ~ Extra Breath Weapon Line Acid`
//!   * `Evolution ~ Extra Breath Weapon Line Cold`
//!   * `Evolution ~ Extra Breath Weapon Line Electricity`
//!   * `Evolution ~ Extra Breath Weapon Line Fire`
//!   * `Evolution ~ Fast Healing`
//!   * `Evolution ~ Flight Magic`
//!   * `Evolution ~ Flight Winged`
//!   * `Evolution ~ Frightful Presence`
//!   * `Evolution ~ Gills`
//!   * `Evolution ~ Gore`
//!   * `Evolution ~ Grab`
//!   * `Evolution ~ Huge`
//!   * `Evolution ~ Immune Acid`
//!   * `Evolution ~ Immune Cold`
//!   * `Evolution ~ Immune Electricity`
//!   * `Evolution ~ Immune Fire`
//!   * `Evolution ~ Immune Sonic`
//!   * `Evolution ~ Improved Bite`
//!   * `Evolution ~ Improved Climb`
//!   * `Evolution ~ Improved DR Chaotic`
//!   * `Evolution ~ Improved DR Evil`
//!   * `Evolution ~ Improved DR Good`
//!   * `Evolution ~ Improved DR Lawful`
//!   * `Evolution ~ Improved Damage`
//!   * `Evolution ~ Improved Fast Healing`
//!   * `Evolution ~ Improved Flight`
//!   * `Evolution ~ Improved Natural Armor`
//!   * `Evolution ~ Improved Swim`
//!   * `Evolution ~ Large`
//!   * `Evolution ~ Legs`
//!   * `Evolution ~ Magic Attacks`
//!   * `Evolution ~ Mount`
//!   * `Evolution ~ Pincers`
//!   * `Evolution ~ Poison Con`
//!   * `Evolution ~ Poison Str`
//!   * `Evolution ~ Pounce`
//!   * `Evolution ~ Pull`
//!   * `Evolution ~ Push`
//!   * `Evolution ~ Rake`
//!   * `Evolution ~ Reach`
//!   * `Evolution ~ Remove Claws`
//!   * `Evolution ~ Rend`
//!   * `Evolution ~ Resist Acid`
//!   * `Evolution ~ Resist Cold`
//!   * `Evolution ~ Resist Electricity`
//!   * `Evolution ~ Resist Fire`
//!   * `Evolution ~ Resist Sonic`
//!   * `Evolution ~ SR`
//!   * `Evolution ~ Scent`
//!   * `Evolution ~ Skilled`
//!   * `Evolution ~ Slam`
//!   * `Evolution ~ Small`
//!   * `Evolution ~ Sting`
//!   * `Evolution ~ Swallow Whole`
//!   * `Evolution ~ Swim`
//!   * `Evolution ~ Tail`
//!   * `Evolution ~ Tail Slap`
//!   * `Evolution ~ Tentacle`
//!   * `Evolution ~ Trample`
//!   * `Evolution ~ Tremorsense`
//!   * `Evolution ~ Trip`
//!   * `Evolution ~ Weapon Martial`
//!   * `Evolution ~ Weapon Simple`
//!   * `Evolution ~ Web`
//!   * `Evolution ~ Wing Buffet`
//!   * `Temp Evolution ~ Ability Increase Cha`
//!   * `Temp Evolution ~ Ability Increase Con LH`
//!   * `Temp Evolution ~ Ability Increase Con SM`
//!   * `Temp Evolution ~ Ability Increase Dex`
//!   * `Temp Evolution ~ Ability Increase Int`
//!   * `Temp Evolution ~ Ability Increase Str LH`
//!   * `Temp Evolution ~ Ability Increase Str SM`
//!   * `Temp Evolution ~ Ability Increase Wis`
//!   * `Temp Evolution ~ Arms`
//!   * `Temp Evolution ~ Aspect`
//!   * `Temp Evolution ~ Aspect Greater`
//!   * `Temp Evolution ~ Bite`
//!   * `Temp Evolution ~ Blindsense`
//!   * `Temp Evolution ~ Blindsight`
//!   * `Temp Evolution ~ Breath Weapon (Cone of Acid)`
//!   * `Temp Evolution ~ Breath Weapon (Cone of Cold)`
//!   * `Temp Evolution ~ Breath Weapon (Cone of Electricity)`
//!   * `Temp Evolution ~ Breath Weapon (Cone of Fire)`
//!   * `Temp Evolution ~ Breath Weapon (Line of Acid)`
//!   * `Temp Evolution ~ Breath Weapon (Line of Cold)`
//!   * `Temp Evolution ~ Breath Weapon (Line of Electricity)`
//!   * `Temp Evolution ~ Breath Weapon (Line of Fire)`
//!   * `Temp Evolution ~ Burrow`
//!   * `Temp Evolution ~ Clawed Feet`
//!   * `Temp Evolution ~ Clawed Hands`
//!   * `Temp Evolution ~ Climb`
//!   * `Temp Evolution ~ Constrict`
//!   * `Temp Evolution ~ DR Chaotic`
//!   * `Temp Evolution ~ DR Evil`
//!   * `Temp Evolution ~ DR Good`
//!   * `Temp Evolution ~ DR Lawful`
//!   * `Temp Evolution ~ Energy Attack Acid`
//!   * `Temp Evolution ~ Energy Attack Cold`
//!   * `Temp Evolution ~ Energy Attack Electricity`
//!   * `Temp Evolution ~ Energy Attack Fire`
//!   * `Temp Evolution ~ Extra Breath Weapon Cone Acid`
//!   * `Temp Evolution ~ Extra Breath Weapon Cone Cold`
//!   * `Temp Evolution ~ Extra Breath Weapon Cone Electricity`
//!   * `Temp Evolution ~ Extra Breath Weapon Cone Fire`
//!   * `Temp Evolution ~ Extra Breath Weapon Line Acid`
//!   * `Temp Evolution ~ Extra Breath Weapon Line Cold`
//!   * `Temp Evolution ~ Extra Breath Weapon Line Electricity`
//!   * `Temp Evolution ~ Extra Breath Weapon Line Fire`
//!   * `Temp Evolution ~ Fast Healing`
//!   * `Temp Evolution ~ Flight Magic`
//!   * `Temp Evolution ~ Flight Winged`
//!   * `Temp Evolution ~ Frightful Presence`
//!   * `Temp Evolution ~ Gills`
//!   * `Temp Evolution ~ Gore`
//!   * `Temp Evolution ~ Grab`
//!   * `Temp Evolution ~ Huge`
//!   * `Temp Evolution ~ Immune Acid`
//!   * `Temp Evolution ~ Immune Cold`
//!   * `Temp Evolution ~ Immune Electricity`
//!   * `Temp Evolution ~ Immune Fire`
//!   * `Temp Evolution ~ Immune Sonic`
//!   * `Temp Evolution ~ Improved Bite`
//!   * `Temp Evolution ~ Improved Climb`
//!   * `Temp Evolution ~ Improved DR Chaotic`
//!   * `Temp Evolution ~ Improved DR Evil`
//!   * `Temp Evolution ~ Improved DR Good`
//!   * `Temp Evolution ~ Improved DR Lawful`
//!   * `Temp Evolution ~ Improved Damage`
//!   * `Temp Evolution ~ Improved Fast Healing`
//!   * `Temp Evolution ~ Improved Flight`
//!   * `Temp Evolution ~ Improved Natural Armor`
//!   * `Temp Evolution ~ Improved Swim`
//!   * `Temp Evolution ~ Large`
//!   * `Temp Evolution ~ Legs`
//!   * `Temp Evolution ~ Magic Attacks`
//!   * `Temp Evolution ~ Mount`
//!   * `Temp Evolution ~ Pincers`
//!   * `Temp Evolution ~ Poison Con`
//!   * `Temp Evolution ~ Poison Str`
//!   * `Temp Evolution ~ Pounce`
//!   * `Temp Evolution ~ Pull`
//!   * `Temp Evolution ~ Push`
//!   * `Temp Evolution ~ Rake`
//!   * `Temp Evolution ~ Reach`
//!   * `Temp Evolution ~ Rend`
//!   * `Temp Evolution ~ Resist Acid`
//!   * `Temp Evolution ~ Resist Cold`
//!   * `Temp Evolution ~ Resist Electricity`
//!   * `Temp Evolution ~ Resist Fire`
//!   * `Temp Evolution ~ Resist Sonic`
//!   * `Temp Evolution ~ SR`
//!   * `Temp Evolution ~ Scent`
//!   * `Temp Evolution ~ Skilled`
//!   * `Temp Evolution ~ Slam`
//!   * `Temp Evolution ~ Sting`
//!   * `Temp Evolution ~ Swallow Whole`
//!   * `Temp Evolution ~ Swim`
//!   * `Temp Evolution ~ Tail`
//!   * `Temp Evolution ~ Tail Slap`
//!   * `Temp Evolution ~ Tentacle`
//!   * `Temp Evolution ~ Trample`
//!   * `Temp Evolution ~ Tremorsense`
//!   * `Temp Evolution ~ Trip`
//!   * `Temp Evolution ~ Weapon Martial`
//!   * `Temp Evolution ~ Weapon Simple`
//!   * `Temp Evolution ~ Web`
//!   * `Temp Evolution ~ Wing Buffet`

use crate::rules_core::rules_tables::companion_chassis::{CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord, Speed, StatAdjustment};

/// Every advanced_players_guide companion creature (1 rows).
pub(super) static COMPANIONS: &[CompanionRecord] = &[
    CompanionRecord {
        key: "Eidolon",
        name: "Eidolon",
        size: Some("M"),
        speeds: &[Speed { mode: "Walk", feet: 20 }],
        reach_feet: Some(5),
        race_type: None,
        race_subtype: Some("Eidolon"),
        monster_class: Some("Eidolon:1"),
        type_segments: &[],
        natural_attacks: &[],
        stat_adjustments: &[StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "INT", amount: -4 }],
        natural_armor: Some(2),
        source_page: Some("p.56"),
        ability_keys: &["Eidolon ~ Link", "Eidolon ~ Share Spells", "Eidolon ~ Skills"],
        external_ability_refs: &[],
        source_file: "apg_races_companion.lst",
        source_line: 7,
    },
];

/// Every advanced_players_guide companion ability record (3 rows).
pub(super) static COMPANION_ABILITIES: &[CompanionAbilityRecord] = &[
    CompanionAbilityRecord {
        key: "Eidolon ~ Link",
        name: "Link",
        facet: Some(CompanionAbilityFacet::SpecialQuality),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialQuality", "Extraordinary"],
        description: Some("A summoner and his eidolon share a mental link that allows for communication across any distance (as long as they are on the same plane). This communication is a free action, allowing the summoner to give orders to his eidolon at any time. In addition, magic items interfere with the summoner's connection to his eidolon. As a result, the summoner and his eidolon share magic item slots. For example, if the summoner is wearing a ring, his eidolon can wear no more than one ring. In case of a conflict, the items worn by the summoner remain active, and those used by the eidolon become dormant. The eidolon must possess the appropriate appendages to utilize a magic item."),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Eidolon"],
        source_file: "apg_abilities_companion.lst",
        source_line: 67,
    },
    CompanionAbilityRecord {
        key: "Eidolon ~ Share Spells",
        name: "Share Spells",
        facet: Some(CompanionAbilityFacet::SpecialQuality),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialQuality", "Extraordinary"],
        description: Some("The summoner may cast a spell with a target of \"You\" on his eidolon (as a spell with a range of touch) instead of on himself. A summoner may cast spells on his eidolon even if the spells normally do not affect creatures of the eidolon's type (outsider). Spells cast in this way must come from the summoner spell list. This ability does not allow the eidolon to share abilities that are not spells, even if they function like spells."),
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Eidolon"],
        source_file: "apg_abilities_companion.lst",
        source_line: 68,
    },
    CompanionAbilityRecord {
        key: "Eidolon ~ Skills",
        name: "Skills",
        facet: None,
        delivery: None,
        type_segments: &["SkillChoice"],
        description: None,
        description_variables: &[],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: None,
        owners: &["Eidolon"],
        source_file: "apg_abilities_companion.lst",
        source_line: 69,
    },
];
