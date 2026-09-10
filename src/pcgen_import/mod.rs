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
pub mod bonus_stack_reader;
// SD-35 AT-35-E6-002 (`decisions.md` §11): relocated from `src/rules_core/` -- the per-book
// cache generators and the token-closure classifier they call. Converter code that was
// sitting on the live side; behaviour-identical here, and KEPT for Starfinder.
pub mod cache_gen;
pub mod wiring_class;
pub mod class_feature_vars;
pub mod formula_interpreter;
pub mod formula_interpreter_corpus_wide;
pub mod formula_reproduction_harness;
pub mod pre_tokens;
pub mod race_trait_formula_binding;
pub mod corpus_trap_baseline;
pub mod corpus_traps;
pub mod include_resolver;
pub mod ir_converter;
pub mod lst_parser;
pub mod pcc;
pub mod sheet_rule;
pub mod source_content_payload;

pub use lst_parser::ParsedLstRecord;
