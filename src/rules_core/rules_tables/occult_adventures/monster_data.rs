//! occult_adventures monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py occult_adventures`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `oa_races_b3.lst` -- 1 monster rows
//!   * `oa_abilities_race_b3.lst` -- 2 monster-ability rows
//!   * `oa_abilities_race.lst` -- 3 monster-ability rows
//!
//! 5 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they SHIP with `owners: &[]` rather than being
//! dropped (`decisions.md §20`: an un-ingested row's shape cannot be measured,
//! and Gate 1's DoD needs every unit's shape measured). `list_monster_catalog`
//! only ever walks a monster's OWN `ability_keys`, so an owner-less record here
//! reaches no screen -- reachability is NOT claimed for these, and each key is
//! pinned as a named, provable non-reach in `reach_gate.rs::
//! UNREACHED_RECORD_FINDINGS`, never silently assumed reachable:
//!   * `oa_abilities_race_b3.lst:9`
//!   * `oa_abilities_race_b3.lst:10`
//!   * `oa_abilities_race.lst:188`
//!   * `oa_abilities_race.lst:189`
//!   * `oa_abilities_race.lst:190`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, NaturalAttack, Speed, StatAdjustment};

/// Every occult_adventures monster stat block (1 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
    MonsterStatBlock {
        key: "Kami (Shikigami)",
        name: "Shikigami",
        size: Some("T"),
        speeds: &[Speed { mode: "Walk", feet: 30 }],
        race_type: Some("Outsider"),
        race_subtype: Some("Kami|Native"),
        challenge_rating: Some("2"),
        monster_class: Some("Outsider (Fort/Will):3"),
        source_page: Some("p.163"),
        natural_attacks: &[],
        ability_keys: &[],
        external_ability_refs: &[],
        stat_adjustments: &[StatAdjustment { ability: "STR", amount: -2 }, StatAdjustment { ability: "DEX", amount: 2 }, StatAdjustment { ability: "CON", amount: 2 }, StatAdjustment { ability: "INT", amount: 0 }, StatAdjustment { ability: "WIS", amount: 6 }, StatAdjustment { ability: "CHA", amount: 4 }],
        has_spell_like_abilities: false,
        sla_cl_token: None,
        spell_like_abilities: &[],
        source_file: "oa_races_b3.lst",
        source_line: 6,
    },
];

/// Every occult_adventures monster-ability record (5 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Shikigami ~ Improvised Weapon Mastery",
        name: "Improvised Weapon Mastery",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &["ShikigamiRacialAbility"],
        description: Some("A shikigami gains Catch Off-Guard and Throw Anything as bonus feats, and adds its Charisma modifier instead of its Strength modifier to damage done with any improvised weapon, as attacks it makes with such weapons seem supernaturally lucky in landing damaging blows. Although a shikigami is Tiny, it never provokes attacks of opportunity when it attacks an adjacent foe with a melee weapon. If a shikigami critically hits an opponent with an improvised weapon, it deals ?3 damage."),
        description_variables: &[],
        source_page: Some("p.163"),
        owners: &[],
        source_file: "oa_abilities_race_b3.lst",
        source_line: 9,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Shikigami ~ Spell-Like Abilities",
        name: "Spell-Like Abilities",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::SpellLike),
        traits: &["ShikigamiRacialAbility"],
        description: Some("(CL 6th; concentration +8) At will-invisibility (self only), statue (self only) 3/day-hide from animals, purify food and drink 1/week-commune with nature (CL 12th)"),
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "oa_abilities_race_b3.lst",
        source_line: 10,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Homunculus Companion ~ Sympathetic Alchemy",
        name: "Sympathetic Alchemy",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &[],
        description: Some("The bond between a promethean alchemist and his homunculus is so close that the alchemist's extracts function for the homunculus as if it were the alchemist, allowing the homunculus companion to benefit from extracts without the alchemist needing the infusion discovery. The homunculus is treated as a humanoid or a construct-whichever is more beneficial-for the purposes of what extracts can affect it. Additionally, the homunculus can prepare its master's extracts from his formula book for him each day, as long as it's within the range of its telepathic link."),
        description_variables: &[],
        source_page: Some("p.113"),
        owners: &[],
        source_file: "oa_abilities_race.lst",
        source_line: 188,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Homunculus Companion ~ Telepathic Link",
        name: "Telepathic Link",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Supernatural),
        traits: &[],
        description: Some("A homunculus can't initially speak, but shares a telepathic link with its creator. It knows what its master knows and can convey to him everything it sees and hears, out to a range of 1,500 feet."),
        description_variables: &[],
        source_page: Some("p.113"),
        owners: &[],
        source_file: "oa_abilities_race.lst",
        source_line: 189,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Homunculus Companion ~ Poison",
        name: "Poison",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("Bite-injury frequency 1/minute for 60 minutes, effect sleep for 1 minute, cure 1 save, DC %1."),
        description_variables: &["12+TL/2+CON"],
        source_page: Some("p.113"),
        owners: &[],
        source_file: "oa_abilities_race.lst",
        source_line: 190,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
];
