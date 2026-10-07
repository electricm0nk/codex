//! ultimate_wilderness monster + monster-ability tables, transcribed verbatim
//! from the book's own PCGen source file rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_monster_tables.py ultimate_wilderness`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the source file (which counts `.MOD`/`.COPY` overlays the
//! inventory correctly excludes).
//!
//! Sources, with the line each record was read from carried per row:
//!   * `uw_abilities_race` -- 2 monster-ability rows
//!
//! 2 further ability row(s) in this book are ORPHANS -- no monster
//! row here claims them, so they SHIP with `owners: &[]` rather than being
//! dropped (`decisions.md §20`: an un-ingested row's shape cannot be measured,
//! and Gate 1's DoD needs every unit's shape measured). `list_monster_catalog`
//! only ever walks a monster's OWN `ability_keys`, so an owner-less record here
//! reaches no screen -- reachability is NOT claimed for these, and each key is
//! pinned as a named, provable non-reach in `reach_gate.rs::
//! UNREACHED_RECORD_FINDINGS`, never silently assumed reachable:
//!   * `uw_abilities_race:25`
//!   * `uw_abilities_race:27`

use crate::rules_core::rules_tables::monster_chassis::{MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock};

/// Every ultimate_wilderness monster stat block (0 rows).
pub(crate) static MONSTERS: &[MonsterStatBlock] = &[
];

/// Every ultimate_wilderness monster-ability record (2 rows).
pub(crate) static MONSTER_ABILITIES: &[MonsterAbilityRecord] = &[
    MonsterAbilityRecord {
        key: "Plant Traits Output (PC)",
        name: "Plant Traits",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("Plants breathe and eat, but do not sleep."),
        description_variables: &[],
        source_page: Some("p.309"),
        owners: &[],
        source_file: "uw_abilities_race",
        source_line: 25,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
    MonsterAbilityRecord {
        key: "Traits Output ~ Leshy (PC)",
        name: "Leshy Traits",
        facet: MonsterAbilityFacet::SpecialQuality,
        delivery: Some(MonsterAbilityDelivery::Extraordinary),
        traits: &[],
        description: Some("A leshy is a nature spirit that inhabits the body of a specially grown plant."),
        description_variables: &[],
        source_page: Some("p.307"),
        owners: &[],
        source_file: "uw_abilities_race",
        source_line: 27,
        codex_generated_name: false,
        rename_reason: None,
        rename_coordinate: None,
    },
];
