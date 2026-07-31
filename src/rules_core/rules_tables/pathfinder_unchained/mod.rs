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
//! `src/bin/sd27_gen_book_cache.rs`'s `#[path]` include.
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

// `src/bin/sd27_gen_book_cache.rs` pulls this `mod.rs` in with `#[path]`
// (see that file's own doc comment for why). That binary uses only
// `feat_tables` and `equipment_tables`, so inside it the four class-feature
// modules below (barbarian, monk, rogue, summoner) are genuinely unreferenced
// and `dead_code` fires on every item. Through the library crate -- the path
// the tests and any future `pilot_compute` integration use -- they are
// ordinary `pub` items and the lint never applies. The allow is scoped to
// these four registrations rather than blanketed inside the files, so real
// dead code inside them would still be reported anywhere it could matter.
#[allow(dead_code)]
pub mod barbarian_features;
pub mod equipment_tables;
pub mod feat_tables;
#[allow(dead_code)]
pub mod monk_features;
#[allow(dead_code)]
pub mod rogue_features;
#[allow(dead_code)]
pub mod summoner_features;
