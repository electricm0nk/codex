//! ARG Teamwork feats -- generated from the real `arg_feats.lst`
//! corpus (SD-27 Cycle E2.1/E2.2 per-book pre-build). Not hand-authored --
//! see `feats.rs`'s own doc comment for the generation method.
//! See `feats.rs` for the full corpus-coverage/exclusion accounting.

use super::super::feats::{FeatCategory, FeatEffectBonus, FeatTableEntry};

pub const TEAMWORK_TABLE: &[FeatTableEntry] = &[
    FeatTableEntry { key: "Focusing Blow", category: FeatCategory::Teamwork, name: "Focusing Blow", description: Some("You and your allies work together to shake off mental effects."), effect: None },
    FeatTableEntry { key: "Greater Brand", category: FeatCategory::Teamwork, name: "Greater Brand", description: Some("A kinslayer learns to modify her slayer's brand judgment as she gains levels."), effect: Some(&[FeatEffectBonus { qualifiers: &["ABILITYPOOL", "Greater Brand", "1"] }]) },
    FeatTableEntry { key: "Horde Charge", category: FeatCategory::Teamwork, name: "Horde Charge", description: Some("When you charge with an ally, you are more deadly."), effect: None },
];
