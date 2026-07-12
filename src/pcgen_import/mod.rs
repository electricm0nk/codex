//! PCGen import bridge.
//!
//! Parsing and semantic conversion are deliberately separate stages. This
//! module currently exposes the PCC entry-file parser and the LST races +
//! race-ability object parser; the token registry, semantic conversion
//! handlers, and the source-map writer are owned by later GE-03 slices and
//! are intentionally absent here.

pub mod include_resolver;
pub mod lst_parser;
pub mod pcc;
