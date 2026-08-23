//! Pathfinder Unchained (PU) book-level module. SD-27 Cycle E2.2
//! per-book pre-build
//! (`docs/release/SD-27-future-state-book-content-ingestion/
//! loop-instruction.md §3.3.3`).
//!
//! **First `rules_tables/<book>/` generation for a future-state book.**
//! CRB/APG/ACG/Bestiary-1 are the 4 in-scope books SD-26 already built
//! full caches for; PU is one of the 2 future-state books (alongside
//! Advanced Race Guide) SD-27 resolves out of stub state
//! (`decisions.md §1`). Per `technical-design.md §2.3`, this is a
//! deliberately thin layer -- "just enough to populate the JSON cache" --
//! not a full rules-engine chassis. PU still carries no wired
//! `pilot_compute` integration — this bundle's own partition explicitly
//! does not touch `src/rules_core/pilot_compute.rs` (`decisions.md §8`) —
//! but it is no longer unregistered: a later cycle owning
//! `rules_tables/mod.rs` added `pub mod pathfinder_unchained;` and the
//! `RuleSetId::Pu` variant, so this module is reachable through the
//! `codex` library's public module tree as well as through
//! `src/bin/gen_book_cache.rs`'s `#[path]` include.
//!
//! **Scope, confirmed against the real corpus this cycle (not taken on
//! faith):** `pu_equipmods.lst` (42 real records, `equipment_tables`)
//! and `pu_feats.lst` (17 real, distinct feat records out of 18
//! non-comment rows -- the 18th is a `.MOD` modifier of an existing APG
//! feat, not a new feat; see `feat_tables`'s own doc comment). Real,
//! independently re-verified: `pu_spells.lst` is 224 lines, every single
//! one a `#`-commented-out row (0 active records) -- this book adds no
//! new spells of its own, it only re-lists existing Summoner spells
//! behind a fully commented-out block. `pu_abilities_class.lst` (1,344
//! real lines; the low-level ability/BONUS/DEFINE/PREREQ formula-engine
//! content ARG's own module and CRB's own `class_tables.rs` both
//! deliberately stop short of full-tree ingestion for), `pu_skills.lst`
//! (120 lines) and `pu_templates.lst` (17 lines) are out of this cycle's
//! bounded scope -- no book in this codebase has ever represented
//! skill-content or template-content as a Shape B content-kind, and this
//! book's own ability-tree content shares CRB/ARG's already-documented
//! "no established schema, high hallucination risk" exclusion. Not
//! attempted here; see this cycle's own report for the full reasoning.

// This module tree is now reached ONLY through the library crate.
//
// `src/bin/gen_book_cache.rs` used to pull this `mod.rs` in with
// `#[path]`, a workaround from a cycle that could not touch
// `rules_tables/mod.rs`; that file's own doc comment invited a later cycle
// to undo it once `pub mod pathfinder_unchained;` existed there. It does,
// so the binary now imports `codex::rules_core::rules_tables::pathfinder_unchained`
// like every other consumer. That removal is what lets `class_chassis`
// below use `crate::rules_core::...` paths at all -- under `#[path]` into a
// binary crate there is no `crate::rules_core` to resolve.
//
// The four class-feature modules' `#[allow(dead_code)]` went with it: they
// are ordinary `pub` items on the library's public surface, so the lint
// never applied through this path, and the allow only ever existed to
// silence the binary-crate copy.
pub mod barbarian_features;
pub mod class_chassis;
pub mod equipment_tables;
pub mod feat_tables;
mod monster_data;
pub mod monk_features;
pub mod rogue_features;
pub mod summoner_features;

// `decisions.md §20` no_record-to-zero, round 4: this book's own
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
