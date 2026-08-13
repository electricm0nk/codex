//! Core Essentials (CE). SD-29 Epic 7 round 7 (`SD29-E7-F2-008`) -- this
//! book's `companion` family, and the first Rust module this book has had.
//!
//! `RuleSetId::Ce` already existed before this round: the race-trait lane
//! compiled it in `decisions.md §49` and serves that family off disk from
//! `data/corpus/core_essentials/race_trait/`. The kanban row dispatching this
//! round flagged "needs a NEW `RuleSetId`, and you must check rather than
//! assume" -- checked, and it does not. Registering this family therefore cost
//! no scope flip: no unit of any other kind moved status.
//!
//! 103 of the book's 145 `companion` corpus rows ship (58 creature rows, 45
//! ability rows), which is exactly the `reachable remainder`
//! `python3 scripts/classify_companion_rows.py core_essentials` prints. The
//! shipped count and the lane's own ceiling are the same number derived two
//! independent ways.
//!
//! ---------------------------------------------------------------------------
//! THE 42 ROWS THAT DO NOT SHIP, and why each is a corpus fact rather than a
//! transcription gap. Both groups are named row by row in `companion_data`'s
//! module doc, and both keep their honest `not-ingested` status in
//! `docs/work-inventory.json`.
//!
//! * **22 `.COPY=` CREATURE rows** (`decisions.md §62.1`). This is the first
//!   companion book whose *creature* rows carry the delta shape.
//!   `ce_races_familiar_cr.lst:33` reads `Bat.COPY=Bat (Celestial)` and carries
//!   `OUTPUTNAME:`, `TEMPLATE:` and `KIT:` -- and nothing else. No `SIZE:`, no
//!   `MOVE:`, no `MONSTERCLASS:`. PCGen copies the base `Bat` whole and applies
//!   the template; the row itself states a delta, not a creature. The eleven
//!   Core Rulebook familiars each appear twice this way, once `(Celestial)` and
//!   once `(Fiendish)`.
//!
//!   `§59.2` built exactly this screen one round earlier and ran it over
//!   ABILITY rows alone, because Bestiary 4 -- the book that forced it --
//!   carries `.COPY=` only there. The screen was correct and its domain was
//!   too narrow, which is a shape worth naming: a guard written for the book
//!   that provoked it holds for that book and quietly does nothing for the next
//!   one. Widening it changed **not one byte** of any of the eleven registered
//!   books -- all eleven were regenerated and `git status --porcelain` listed
//!   none of their `companion_data.rs` files.
//!
//! * **4 `.MOD` ability rows** -- the `Universal Monster Rule ~ …` overlays.
//!   `§59.2` shipped the `mod_only` half of its screen as **stated, not
//!   exercised**, and named this book as where it would first bite. It did.
//!   This round is the first to execute that branch.
//!
//! * **16 orphan ability rows**, carried per the monster lane's disposition
//!   (`decisions.md §50`): rows no shipped creature of this book reaches.
//!
//! The three groups do not overlap -- 22 + 4 + 16 = 42, and 42 is also the
//! `distinct excluded rows (the UNION, not the sum)` the classifier prints, so
//! the sum and the union agree here. They have not always (`§59.2` corrected a
//! corpus-wide ceiling that had been computed as a sum over overlapping sets),
//! which is why this states the check rather than the arithmetic.
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
