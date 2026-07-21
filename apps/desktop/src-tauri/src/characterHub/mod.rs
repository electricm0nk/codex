//! Tauri command-surface repair submodule (SD-24 Epic 7).
//!
//! `character_hub.rs` already carries the character-create/load/list/
//! level-up/single-item-append surface. This submodule is the home for the
//! *iterative mutation* commands `technical-design.md` §3.2 specifies
//! (`appendToCharacter`, and — in follow-on cycles — `recomputeCharacter`,
//! `reSaveCharacter`), kept in their own module rather than growing the
//! already-large `character_hub.rs` further.

#[allow(non_snake_case)]
pub mod appendToCharacter;
