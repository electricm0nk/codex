//! Ultimate Wilderness (UW). SD-28 Epic 26 (`epic-26-uw-complete`) --
//! from-scratch book ingest, first slice: the 136-record feat catalog. See
//! `feat_tables`'s own module doc comment for the catalog and its
//! own-category-enum ruling (UW's `Animal`/`Mount` facets have no shared
//! `crb::feats::FeatCategory` equivalent).
//!
//! SD-29 Epic 7 round 6 added this book's `companion` family, documented at the
//! bottom of this file. It shares nothing with the feats above but a
//! `RuleSetId`: different `.lst` file, different chassis, different screen.

pub mod archetype_tables;
mod companion_data;
pub mod feat_tables;
mod monster_data;
// SD-32 card 11 (T12), cycle 4: real per-feature compute functions for the
// Shifter, this book's single magnitude-bearing class.
pub mod shifter_features;
pub mod spell_list;

pub use super::companion_chassis::{CompanionAbilityRecord, CompanionRecord};
// `decisions.md §20` no_record-to-zero, round 3: this book's own `monster_ability`
// orphans (`monster_data.rs`'s own header derives the count). Zero monster
// rows of its own, so every one ships owner-less by construction -- see
// `monster_data.rs`'s header for the exact keys and `reach_gate.rs::
// UNREACHED_RECORD_FINDINGS` for the pinned non-reach.
pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
};

// ---------------------------------------------------------------------------
// SD-29 Epic 7 round 6 (`SD29-E7-F2-007`) -- this book's `companion` family.
//
// **The largest single companion block in the corpus**: 169 creature rows, more
// than every previously registered companion book combined, and 158 of the
// book's 406 ability rows. `327` is exactly the `reachable remainder`
// `python3 scripts/classify_companion_rows.py ultimate_wilderness` prints, so
// what ships and what the lane's ceiling says should ship are the same number
// derived two ways.
//
// Registration cost NO scope flip and no new `RuleSetId`: SD-28 Epic 26
// compiled `RuleSetId::Uw` for this book's feats.
//
// **This is the first companion book with a real shortfall, and it is 247 rows
// wide.** They are not a transcription gap -- they are a different KIND of
// thing wearing the `companion` kind's file name. 149 of the 247 belong to the
// book's 30 `CATEGORY:Archetype` companion/familiar archetype rows (`Familiar
// Archetype ~ Valet`, `Companion Archetype ~ Draconic Companion`) and the
// ability rows namespaced under their DISPLAY names (`Valet ~ Deliver Aid`);
// 72 more are the generic option groups `Animal Trick ~ …` (39) and
// `Animal Companion Feat ~ …` (33), which attach to ANY animal companion rather
// than to a creature. An archetype is not a creature: it has no `SIZE:`, no
// `MOVE:` and no `MONSTERCLASS:`, so `CompanionRecord` cannot hold one and the
// catalog has no screen that would show it. Ingesting them under this chassis
// would ship exactly the stub `docs/governance/no-stub-mvp-doctrine.md` forbids,
// so they are dropped and named row by row in `companion_data`'s module doc.
//
// They are deliberately NOT a `reach_gate` `OPEN_FINDINGS` entry, and the
// reason is worth stating because the transcriber's own boilerplate claimed
// otherwise for two rounds: that list is keyed by (book, FAMILY) and its
// consistency test fails an entry naming a family that DOES reach a player.
// `ultimate_wilderness/companions` reaches one. A dropped row is also not an
// ingested record, so it is outside the reach gate's denominator entirely. The
// shortfall is counted where it is real -- these 247 rows keep their honest
// `not-ingested` status in `docs/work-inventory.json` (`decisions.md §61.2`).
//
// The mechanism this book DID need is conditional `DESC:` variants
// (`decisions.md §61.1`): 22 of its ability rows -- including `Poison`,
// `Constrict`, `Breath Weapon` and `Camouflage` -- state their rules text
// between 2 and 9 times, each token gated on a different
// `PREVARGTEQ:`/`PREVARLT:`/`PREALIGN:` predicate. Every one is carried, gate
// verbatim, in `CompanionAbilityRecord::description_variants`.
// ---------------------------------------------------------------------------

/// Every companion creature this book defines, in corpus row order.
pub const fn companions_static() -> &'static [CompanionRecord] {
    companion_data::COMPANIONS
}

/// Every companion ability record this book defines, in corpus row order.
pub const fn companion_abilities_static() -> &'static [CompanionAbilityRecord] {
    companion_data::COMPANION_ABILITIES
}

/// Every companion creature this book defines, in corpus row order.
pub fn companions() -> &'static [CompanionRecord] {
    companions_static()
}

/// Every companion ability record this book defines, in corpus row order.
pub fn companion_abilities() -> &'static [CompanionAbilityRecord] {
    companion_abilities_static()
}

/// Every monster stat block this book defines (0 rows -- see `monster_data.rs`).
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this book defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}
