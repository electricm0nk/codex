//! Occult Adventures (OA). `SD31-E6-F2-003` — from-scratch book ingest,
//! first slice: the 145-record base spell catalog (see `spell_list`'s own
//! module doc comment for the exact residue named and not ingested this
//! cycle).
//!
//! `decisions.md §27b` — EVERYTHING: this book's second family,
//! `monster_ability` (plus the one `monster` row that owns the `.lst`
//! generator's own auto-included Kami (Shikigami)), overturns the
//! repeatedly-reconfirmed "correctly out of scope" disposition for this
//! book's 5 `monster_ability` units. That disposition was a *reachability*
//! finding (the negated `!PRECAMPAIGN:1,INCLUDES=Bestiary 3` gate this
//! repo's registered campaign set fails), never an *ingest* finding — the
//! objects exist in the book and are ingested here like every other book.
//! Reachability stays a separate, honestly-reported number: all 5 ship
//! owner-less (see `monster_data.rs`'s own header for the exact keys and
//! `reach_gate.rs::UNREACHED_RECORD_FINDINGS` for the pinned non-reach).

pub mod spell_list;

mod monster_data;

// SD-32 card 11 (T12), cycle 4: real per-feature compute functions for the
// six classes sharing `oa_abilities_class.lst`.
pub mod kineticist_features;
pub mod medium_features;
pub mod mesmerist_features;
pub mod occultist_features;
pub mod psychic_features;
pub mod spiritualist_features;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
};

/// Every monster stat block this book defines (1 row -- see `monster_data.rs`).
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this book defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}
