//! Core Rulebook (CRB) table data.
//!
//! SD-29 Epic 7 round 8 (`SD29-E7-F2-009`) added `companion_data` -- this
//! book's `companion` family, and the first companion family to land in a
//! module that already held five other kinds.
//!
//! **The module this file lives in is `crb`; the corpus book is
//! `core_rulebook`.** That split is the round's first hazard and it is the same
//! one `decisions.md §54.3` recorded for Bestiary 1's three spellings. The
//! transcriber writes `rules_tables/<module_dir(book)>/companion_data.rs`, and
//! before this round `MODULE_DIR` mapped only `bestiary -> beastiary1`; running
//! it on `core_rulebook` unmapped would have created
//! `rules_tables/core_rulebook/companion_data.rs`, a SECOND module for a book
//! that already has this one. It would have compiled, passed its own tests and
//! been reachable from nothing. Caught by reading the transcriber's own comment
//! before running it -- the gate cannot see an unreferenced module.
//!
//! 84 of the book's 170 `companion` corpus rows ship (38 creature rows, 46
//! ability rows), which is exactly the `reachable remainder`
//! `python3 scripts/classify_companion_rows.py core_rulebook` prints. Shipped
//! count and lane ceiling, derived two independent ways, agree.
//!
//! No new `RuleSetId`: `RuleSetId::Crb` is the oldest one in the enum. Zero
//! units of any other kind moved status.
//!
//! ---------------------------------------------------------------------------
//! THE 86 ROWS THAT DO NOT SHIP. Both groups are named row by row in
//! `companion_data`'s module doc and keep their honest `engine-does-not-hold` status in
//! `docs/work-inventory.json`.
//!
//! * **84 orphan ability rows**, carried per the monster lane's disposition
//!   (`decisions.md §50`). They are not a transcription gap and they are not
//!   near-misses: they are the generic `Animal Companion ~ …`,
//!   `Animal Companion Feat ~ …`, `Animal Trick ~ …` and `Animal Training ~ …`
//!   rows, which hang off the *Animal Companion class* rather than off any
//!   individual creature. No creature row in `cr_races_companion.lst` names one,
//!   and none carries a `PRERACE:` back to a creature, because they apply to
//!   every animal companion equally. Reaching them needs the class-progression
//!   record type this chassis does not model -- see the next group, which is the
//!   same finding from the other side.
//!
//! * **2 `*_classes_companion.lst` CLASS rows** -- `Companion` and
//!   `Shadow Companion` (`decisions.md §65.1`). A PCGen monster class is a
//!   hit-dice progression, not a creature and not an ability. Until this round
//!   the transcriber *refused outright* on any book carrying the shape; round 8
//!   widened that refusal into the same drop-and-name screen the `.COPY=` rows
//!   already used, which is what let this book be ingested at all. Modelling
//!   them was a new record type, declared and not taken by round 8 --
//!   **`AT-34-E3-001` (`decisions.md §17`) takes it**: both rows are now
//!   `CompanionClassRecord`s (`companion_classes_static()` below), held rather
//!   than excluded. This paragraph's own "84 + 2 = 86 excluded" arithmetic is
//!   therefore historical (round-8-era); re-derive the live count from
//!   `docs/work-inventory.json` rather than this comment
//!   (`decisions.md §12` L2).
//!
//! 84 + 2 = 86, and 86 is also the `distinct excluded rows (the UNION, not the
//! sum)` the classifier prints -- the sum and the union agree for this book.
//! They have not always (`§59.2`), which is why this states the check rather
//! than the arithmetic.
//! ---------------------------------------------------------------------------

mod companion_data;

pub use super::companion_chassis::{CompanionAbilityRecord, CompanionClassRecord, CompanionRecord};

/// Every companion creature this book defines, in corpus row order.
pub const fn companions_static() -> &'static [CompanionRecord] {
    companion_data::COMPANIONS
}

/// Every companion ability record this book defines, in corpus row order.
pub const fn companion_abilities_static() -> &'static [CompanionAbilityRecord] {
    companion_data::COMPANION_ABILITIES
}

/// Every `*_classes_companion.lst` row this book defines, in corpus row
/// order. See `companion_chassis::CompanionClassRecord`.
pub const fn companion_classes_static() -> &'static [CompanionClassRecord] {
    companion_data::COMPANION_CLASSES
}

/// Every companion creature this book defines, in corpus row order.
pub fn companions() -> &'static [CompanionRecord] {
    companions_static()
}

/// Every companion ability record this book defines, in corpus row order.
pub fn companion_abilities() -> &'static [CompanionAbilityRecord] {
    companion_abilities_static()
}

pub mod bard_spell_list;
pub mod class_skill_tables;
pub mod class_tables;
pub mod cleric_spell_list;
pub mod druid_spell_list;
pub mod equipment_data;
pub mod equipment_tables;
pub mod weapon_tables;
pub mod feat_data;
pub mod feats;
pub mod json_cache;
pub mod paladin_spell_list;
pub mod race_tables;
pub mod ranger_spell_list;
pub mod sorcerer_spell_list;
pub mod spell_list;
pub mod wizard_spell_list;
