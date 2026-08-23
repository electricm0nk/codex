//! Ultimate Intrigue (UI). SD-28 Epic 24 (`epic-24-ui-complete`) --
//! from-scratch book ingest, first slice: the 104-record feat catalog. See
//! `feat_tables`'s own module doc comment for the catalog and its
//! text-complete/engine-computed tier ruling.

pub mod equipment_tables;
pub mod feat_tables;
mod monster_data;
pub mod spell_list;
// SD-32 card 11 (T12), cycle 4: real per-feature compute functions for the
// Vigilante, this book's single magnitude-bearing class.
pub mod vigilante_features;

// `decisions.md §20` no_record-to-zero, round 3: this book's own
// `monster_ability` orphans (`monster_data.rs`'s own header derives the
// count). Zero monster rows of its own, so every one ships owner-less by
// construction -- see `monster_data.rs`'s header for the exact keys and
// `reach_gate.rs::UNREACHED_RECORD_FINDINGS` for the pinned non-reach.
pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
};

/// Every monster stat block this book defines (0 rows -- see `monster_data.rs`).
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this book defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}
