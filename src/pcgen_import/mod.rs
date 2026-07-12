//! PCGen import bridge.
//!
//! Parsing and semantic conversion are deliberately separate stages. This
//! module exposes the PCC entry-file parser and the LST spell-row parser
//! (SD-17 Slice B-4). The remaining LST object kinds (CLASS:, RACE:,,
//! ABILITY:, DATACONTROL:, ...), the PCC include-graph resolver (SD-17
//! Slice A on the develop channel), the token registry, the semantic
//! conversion handlers, and the source-map writer are owned by later
//! slices and are intentionally absent here.

pub mod lst_parser;
pub mod pcc;
