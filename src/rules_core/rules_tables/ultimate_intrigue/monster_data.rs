//! ultimate_intrigue monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py ultimate_intrigue`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `ui_abilities_race_pu.lst` -- 6 monster-ability rows
//!
//! 6 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they SHIP with `owners: &[]` rather than being
//! dropped (`decisions.md §20`: an un-ingested row's shape cannot be measured,
//! and Gate 1's DoD needs every unit's shape measured). `list_monster_catalog`
//! only ever walks a monster's OWN `ability_keys`, so an owner-less record here
//! reaches no screen -- reachability is NOT claimed for these, and each key is
//! pinned as a named, provable non-reach in `reach_gate.rs::
//! UNREACHED_RECORD_FINDINGS`, never silently assumed reachable:
//!   * `ui_abilities_race_pu.lst:13`
//!   * `ui_abilities_race_pu.lst:14`
//!   * `ui_abilities_race_pu.lst:15`
//!   * `ui_abilities_race_pu.lst:16`
//!   * `ui_abilities_race_pu.lst:17`
//!   * `ui_abilities_race_pu.lst:18`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock, NaturalAttack, Speed, StatAdjustment};

/// Every ultimate_intrigue monster stat block (0 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
];

/// Every ultimate_intrigue monster-ability record (6 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Fey ~ Unchained Eidolon LVL01",
        name: "Eidolon Progession Lv.1",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["FeyLVL1"],
        description: Some("Starting at 1st level, fey eidolons gain the low-light vision evolution, the skilled evolution (selecting one class skill), and the ability to use either dancing lights, detect magic, ghost sound, or prestidigitation at will as a spell-like ability. Once the summoner selects this and any other spell-like abilities for his fey eidolon, the selection cannot be changed."),
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "ui_abilities_race_pu.lst",
        source_line: 13,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Fey ~ Unchained Eidolon LVL04",
        name: "Eidolon Progession Lv.4",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["FeyLVL4"],
        description: Some("At 4th level, fey eidolons gain woodland stride (as the druid ability)."),
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "ui_abilities_race_pu.lst",
        source_line: 14,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Fey ~ Unchained Eidolon LVL08",
        name: "Eidolon Progession Lv.8",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["FeyLVL8"],
        description: Some("At 8th level, fey eidolons gain the ability to use either charm person, entangle, grease, silent image, obscuring mist, or vanish APG as a spell-like ability three times per day, and they can select an additional spell-like ability from the 1st-level list."),
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "ui_abilities_race_pu.lst",
        source_line: 15,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Fey ~ Unchained Eidolon LVL12",
        name: "Eidolon Progession Lv.12",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["FeyLVL12"],
        description: Some("At 12th level, fey eidolons gain DR 5/cold iron. They also grow gossamer wings, gaining the flight evolution."),
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "ui_abilities_race_pu.lst",
        source_line: 16,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Fey ~ Unchained Eidolon LVL16",
        name: "Eidolon Progession Lv.16",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["FeyLVL16"],
        description: Some("At 16th level, fey eidolons gain the ability to use either blindness/deafness, detect thoughts, glitterdust, hideous laughter, invisibility (self only), minor image, mirror image, or tongues as a spell-like ability three times per day. They also can select an additional spell-like ability from the 1st-level list or the 8th-level list."),
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "ui_abilities_race_pu.lst",
        source_line: 17,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Fey ~ Unchained Eidolon LVL20",
        name: "Eidolon Progession Lv.20",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: None,
        traits: &["FeyLVL20"],
        description: Some("At 20th level, fey eidolons increase their DR to DR 10/cold iron. They gain the ability to use either mass suggestion, mislead, programmed image, or veil as a spell-like ability once per day. They also can select an additional spell-like ability from the 1st-level list, the 8th-level list, or the 16th-level list."),
        description_variables: &[],
        source_page: None,
        owners: &[],
        source_file: "ui_abilities_race_pu.lst",
        source_line: 18,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
];
