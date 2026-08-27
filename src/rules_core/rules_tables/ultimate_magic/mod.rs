//! Ultimate Magic (UM). SD-28 Epic 28 (`epic-28-um-complete`) --
//! from-scratch book ingest, first slice: the 144-record feat catalog. See
//! `feat_tables`'s own module doc comment for the catalog, its
//! own-category-enum ruling, and the corpus-shape findings (three real
//! auto-grant wrappers excluded, four real feats with a mechanic but no
//! corpus prose kept with a dedicated `effect` field rather than dropped).

//!
//! SD-29 Epic 7 round 9 (`SD29-E7-F2-010`) added `companion_data` -- this
//! book's `companion` family, and the largest single block left in the lane
//! after Core Rulebook.
//!
//! **32 -> 59 by `SD31-CE-COMPANION-001` (2026-08-18).** `decisions.md §9`
//! retired the `core_essentials` book id; `ce_races_familiar_um.lst` and
//! `ce_abilities_familiar_race_um.lst` both declare `SOURCELONG:Ultimate
//! Magic` in their own headers, so their 19 creature rows and 8 ability rows
//! are this book's. Their corpus records were already sitting in
//! `data/corpus/ultimate_magic/companion/` and reaching no player surface,
//! because the engine served them out of a `core_essentials` table this
//! book's claim never read. The count below is the pre-move figure, kept
//! because the paragraphs after it are about the ROWS THAT DO NOT SHIP, and
//! that population is unchanged:
//!
//! 32 of the book's 170 `companion` corpus rows ship (10 creature rows, 22
//! ability rows), which is exactly the `reachable remainder`
//! `python3 scripts/classify_companion_rows.py ultimate_magic` prints. Shipped
//! count and lane ceiling, derived two independent ways, agree.
//!
//! No new `RuleSetId`: `RuleSetId::Um` was compiled by SD-28 Epic 28 for this
//! book's feats. Zero units of any other kind moved status.
//!
//! ---------------------------------------------------------------------------
//! THE 138 ROWS THAT DO NOT SHIP, both groups named row by row in
//! `companion_data`'s module doc and keeping their honest `engine-does-not-hold` status
//! in `docs/work-inventory.json`:
//!
//! * **135 orphan ability rows**, carried per the monster lane's disposition
//!   (`decisions.md §50`). They are one finding rather than 135, and it is the
//!   SAME finding Core Rulebook's 84 orphans are (`crb/mod.rs`): the block is
//!   the `Evolution ~ …` / `Temp Evolution ~ …` eidolon evolution pool plus the
//!   `Black Blade ~ …` magus records, and both hang off a CLASS feature -- the
//!   summoner's eidolon and the bladebound magus's black blade -- rather than
//!   off any creature row of `um_races_companion.lst`. No creature row names
//!   one and none carries a `PRERACE:` back to a creature, because they apply
//!   to an eidolon a player builds rather than to a species the book defines.
//!
//! * **3 `*_classes_companion.lst` CLASS rows** -- `1`, `Black Blade` and
//!   `Vermin Companion` (`decisions.md §65.1`). A PCGen monster class is a
//!   hit-dice progression, not a creature and not an ability; the chassis drops
//!   and names them rather than emitting a card whose every modelled field is
//!   empty.
//!
//! 135 + 3 = 138, and 138 is also the `distinct excluded rows (the UNION, not
//! the sum)` the classifier prints -- sum and union agree for this book. They
//! have not always (`§59.2`), which is why this states the check rather than
//! the arithmetic.
//! ---------------------------------------------------------------------------

mod companion_data;

pub use super::companion_chassis::{CompanionAbilityRecord, CompanionRecord};

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

mod monster_data;

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

pub mod archetype_tables;
pub mod equipment_tables;
pub mod feat_tables;
// SD-32 card 11 (T12), cycle 4: real per-feature compute functions for the
// Magus, this book's single magnitude-bearing class.
pub mod magus_features;
pub mod spell_list;
