//! Codex: a character-sheet builder that ingests PCGen source material.
//!
//! This crate currently exposes the GE-03 PCGen import bridge. The first
//! bounded slice (GE03-E1-F1) implements the PCC entry-file parse shape.

pub mod campaign;
pub mod homebrew_authoring;
pub mod oracle_validation;
pub mod pcgen_import;
pub mod rules_core;
pub mod saved_character;
