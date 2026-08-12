//! Shape B v1 JSON-cache *payload* types for the Advanced Race Guide
//! (ARG), SD-27 Cycle E2.1/E2.2. Unlike CRB's `json_cache.rs` (SD-26,
//! Shape B v0), this module deliberately does NOT redeclare
//! `Population`/`Completeness`/`CorpusSource`/a record wrapper — this is a
//! new book landing directly on Shape B v1
//! (`crate::rules_core::shape_b_v1::CorpusRecordV1`), which is already the
//! single shared, book-agnostic authority for those types (see
//! `shape_b_v1.rs`'s own module doc comment on why v1 was deliberately
//! consolidated out of the CRB-local module once the sibling
//! `isolation: 'worktree'` cycles that motivated CRB's book-local v0 types
//! no longer applied). Only the 3 book-specific `data` payload shapes live
//! here.
//!
//! **Generation only, not runtime parsing.** Records of this shape are
//! written by `src/bin/sd27_gen_advanced_race_guide_cache.rs` (a one-off
//! codegen tool, matching `gen_core_rulebook_cache.rs`'s established
//! pattern) which dumps the *already-completed* Rust
//! `rules_tables::advanced_race_guide` module state — it never re-derives
//! any `data` field's *value* from the raw LST at generation time, only
//! real per-record source citations (path/sha256/line).

use serde::{Deserialize, Serialize};

/// `data/corpus/advanced_race_guide/spell/<slug>.json` payload. Mirrors
/// `rules_tables::crb::json_cache::SpellCacheData` field-for-field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellCacheData {
    pub key: String,
    pub school: String,
    pub level: u8,
    pub description: String,
}

/// `data/corpus/advanced_race_guide/equipment/<category>/<slug>.json`
/// payload. Mirrors `rules_tables::crb::json_cache::EquipmentCacheData`
/// field-for-field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentCacheData {
    pub key: String,
    pub category: String,
    pub name: String,
    pub cost_gp: Option<f64>,
    pub weight_lbs: Option<f64>,
    pub description: Option<String>,
}

/// `data/corpus/advanced_race_guide/feat/<category>/<slug>.json` payload.
/// This is the first book in this codebase to cache `feat` records to
/// Shape B JSON (CRB's own `feats.rs`/`feat_data/` has never had a
/// `data/corpus/core_rulebook/feat/` cache directory) — the shape mirrors
/// `rules_tables::advanced_race_guide::feats::FeatTableEntry` directly,
/// following the same "one payload struct per content kind" convention
/// `SpellCacheData`/`EquipmentCacheData` already establish.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatCacheData {
    pub key: String,
    pub category: String,
    pub name: String,
    pub description: Option<String>,
    /// Every `BONUS:` token's pipe-delimited qualifier list, verbatim, in
    /// source order — mirrors `FeatTableEntry.effect`'s shape as plain
    /// `Vec<Vec<String>>` (JSON has no tuple-struct distinction to
    /// preserve). Empty vec, not `null`, when the record has no `BONUS:`
    /// token — JSON's own natural "no entries" shape for a list field,
    /// distinguished from `FeatTableEntry`'s Rust-side `None` only by the
    /// serialization boundary, not a meaning change.
    pub effect: Vec<Vec<String>>,
}
