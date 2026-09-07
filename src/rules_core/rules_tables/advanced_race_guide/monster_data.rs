//! advanced_race_guide monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py advanced_race_guide`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst` (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `arg_abilities_race.lst` -- 1 monster-ability rows
//!
//! 1 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they SHIP with `owners: &[]` rather than being
//! dropped (`decisions.md §20`: an un-ingested row's shape cannot be measured,
//! and Gate 1's DoD needs every unit's shape measured). `list_monster_catalog`
//! only ever walks a monster's OWN `ability_keys`, so an owner-less record here
//! reaches no screen -- reachability is NOT claimed for these, and each key is
//! pinned as a named, provable non-reach in `reach_gate.rs::
//! UNREACHED_RECORD_FINDINGS`, never silently assumed reachable:
//!   * `arg_abilities_race.lst:912`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock};

/// Every advanced_race_guide monster stat block (0 rows).
pub(super) static MONSTERS: &[MonsterStatBlock] = &[
];

/// Every advanced_race_guide monster-ability record (1 rows).
pub(super) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Grippli ~ Toxic Skin ~ Grippli Poison",
        name: "Grippli Poison",
        facet: MonsterAbilityFacet::SpecialAttack,
        delivery: None,
        traits: &["Special Ability"],
        description: Some("Skin or weapon--contact or injury; save Fort DC %1 (10 + 1/2 the grippli's Hit Dice plus its Constitution modifier); frequency 1/round for 6 rounds; effect 1d2 Dexterity damage; cure 1 save."),
        description_variables: &["Grippli_ToxicSkin_PoisonDC"],
        source_page: Some("p.190"),
        owners: &[],
        source_file: "arg_abilities_race.lst",
        source_line: 912,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
];
