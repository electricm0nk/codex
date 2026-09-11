//! PCGen import bridge.
//!
//! Parsing and semantic conversion are deliberately separate stages. This
//! module currently exposes the PCC entry-file parser and the LST races +
//! race-ability object parser; the token registry, semantic conversion
//! handlers, and the source-map writer are owned by later GE-03 slices and
//! are intentionally absent here.
//!
//! ## Record-aggregation umbrella (SD-17 Slice D)
//!
//! [`ParsedLstRecord`] is re-exported here from the LST parser surface
//! so that downstream consumers of the parser can reach the unified
//! kind-tagged record handle directly via `pcgen_import::ParsedLstRecord`
//! without depending on the IR converter. The canonical home remains
//! `pcgen_import::lst_parser::ParsedLstRecord`.

// SD-35 AT-35-E6-001 (`decisions.md` §11): relocated from `src/rules_core/` -- converter and
// oracle code that was sitting on the live side. Behaviour-identical; KEPT for Starfinder.
pub mod archetype_swap_prereq_tokens;
pub mod feat_gap_prereq_tokens;
pub mod feat_effect_tokens;
pub mod feat_prereq_tokens;
pub mod bonus_stack_reader;
// SD-35 AT-35-E6-002 (`decisions.md` §11): relocated from `src/rules_core/` -- the per-book
// cache generators and the token-closure classifier they call. Converter code that was
// sitting on the live side; behaviour-identical here, and KEPT for Starfinder.
pub mod cache_gen;
pub mod wiring_class;
// SD-35 AT-35-E6-002 cycle 2 (`decisions.md` §11): relocated from `src/rules_core/` -- the
// ingest-format literal-token audit. Every consumer is already tool side (`src/bin/` and
// `src/pcgen_import/cache_gen/`); it reads the ingest record's token array, which is converter
// business. Behaviour-identical here, and KEPT for Starfinder.
pub mod corpus_literal_sweep;
pub mod class_feature_vars;
pub mod formula_interpreter;
pub mod formula_interpreter_corpus_wide;
pub mod formula_reproduction_harness;
pub mod pre_tokens;
pub mod race_trait_formula_binding;
pub mod corpus_trap_baseline;
pub mod corpus_traps;
pub mod ingest_record;
// SD-35 AT-35-E6-002 cycle 4 (`decisions.md` §11, `technical-design.md` §0): the ingest cache's
// per-content-kind `data` payload types and the verbatim token carriers they hold, relocated
// from `src/rules_core/shape_b_v1.rs`. The record envelope (`CorpusRecordV1`, `License`, the PI
// markers) is ours and stays live-side; these are the converter's output format.
// Behaviour-identical, and KEPT for Starfinder.
pub mod ingest_payload;
// SD-35 AT-35-E6-002 cycle 3 (`decisions.md` §11, `technical-design.md` §0): every reading of
// an ingested race/race-trait row's token array that `src/rules_core/race_resolver.rs`,
// `race_creation.rs` and `trait_pool.rs` used to do by hand, one named function per fact.
// Behaviour-identical, and KEPT for Starfinder.
pub mod race_trait_tokens;
// SD-35 AT-35-E6-002 cycle 5 (`decisions.md` §11, `technical-design.md` §0): the sibling of
// `race_trait_tokens` for the ingest record's OTHER array, `raw_bonus_chains` -- every walk of
// it that `src/rules_core/race_resolver.rs`, `race_creation.rs` and the desktop
// `race_catalog.rs` used to do by hand, narrowed to typed values. Behaviour-identical, and
// KEPT for Starfinder.
pub mod bonus_chain_reader;
// SD-35 AT-35-E6-002 cycle 3 (`decisions.md` §11, `technical-design.md` §0): the four
// ingest-row predicates `src/rules_core/class_feature_pool_catalog.rs` gates pool membership
// on. Behaviour-identical, and KEPT for Starfinder.
pub mod pool_member_tokens;
pub mod include_resolver;
pub mod ir_converter;
pub mod lst_parser;
pub mod pcc;
pub mod sheet_rule;
pub mod source_content_payload;

pub use lst_parser::ParsedLstRecord;
