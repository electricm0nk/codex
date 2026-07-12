//! PCGen import bridge.
//!
//! Parsing and semantic conversion are deliberately separate stages. This
//! module currently exposes only the PCC entry-file parser; LST parsing, the
//! token registry, semantic conversion handlers, and the source-map writer are
//! owned by later GE-03 slices and are intentionally absent here.

pub mod include_resolver;
pub mod lst_parser;
pub mod pcc;
