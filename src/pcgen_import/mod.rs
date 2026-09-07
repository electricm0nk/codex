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

pub mod corpus_trap_baseline;
pub mod corpus_traps;
pub mod include_resolver;
pub mod ir_converter;
pub mod lst_parser;
pub mod pcc;
pub mod source_content_payload;

pub use lst_parser::ParsedLstRecord;
