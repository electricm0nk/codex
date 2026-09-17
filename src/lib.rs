//! Codex: a character-sheet builder.
//!
//! The PCGen converter and oracle harness that used to live in this crate
//! moved to `crates/codex-ingest` (SD-36 Epic A / operator ruling D1): PCGen
//! is a converter input and a test oracle, never a run-time dependency of
//! the live sheet engine. See `codex_ingest::pcgen_import` /
//! `codex_ingest::oracle_validation`.

pub mod campaign;
pub mod homebrew_authoring;
pub mod rules_core;
pub mod saved_character;
