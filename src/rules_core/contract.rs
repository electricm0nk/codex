//! SD-20 boundary contract — Epic 1.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.1 and
//! `technical-design.md` §1.1, the boundary contract is the engine-side
//! API surface every other SD-20 epic produces into: the `CharacterInput`
//! shapes the engine accepts, the `PilotReceipt` shape it returns, and
//! the printed-sheet cell map the GUI renders from. This module is the
//! contract's code-level home; `docs/SD-20/boundary-contract.md` is its
//! prose artifact.
//!
//! This is Epic 1's first cycle (`CharacterInput` types land first, per
//! the loop instruction's Step 2). It lands `CharacterInputPermutation`
//! and `classify_character_input`, which operationalize the contract's
//! "Inputs" clause: a `CharacterInput` for each of three canonical
//! permutations (brand-new, mid-build, multiclass). The permutations are
//! classifications over the existing, SD-19-shaped `CharacterInput` type
//! (`crate::rules_core::character_input::CharacterInput`) — this cycle
//! does not introduce a new, parallel `CharacterInput` struct. Per
//! `technical-design.md` §1.3, no new field lands on `CharacterInput`
//! without first extending this contract; this cycle adds no field, only
//! a read-only classification over the fields that already exist.
//!
//! `PilotReceipt` types and the printed-sheet cell map are later Epic-1
//! work-units (not this cycle) — see the loop instruction's Step 2 and
//! this bundle's progress doc for the current frontier.

use crate::rules_core::character_input::CharacterInput;

/// The three canonical `CharacterInput` permutations the boundary
/// contract documents in its "Inputs" section
/// (`technical-design.md` §1.1): a brand-new character, a character
/// mid-build, or a multiclass character (any subset of the 11 core
/// classes at any class-level distribution).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterInputPermutation {
    /// Exactly zero or one class level at level 1 or below, and no
    /// player choices recorded yet (no feats, skill ranks, equipment
    /// selections, spell selections, or other selected choices).
    BrandNew,
    /// A single class, but with at least one player choice recorded, or
    /// with a class level above 1.
    MidBuild,
    /// More than one entry in `class_levels` — a multiclass character.
    Multiclass,
}

/// Classify a `CharacterInput` into the boundary contract's canonical
/// permutation (`technical-design.md` §1.1 "Inputs"). Multiclass takes
/// precedence over mid-build (a multiclass character with feats selected
/// is still `Multiclass`, not `MidBuild`); mid-build takes precedence
/// over brand-new.
pub fn classify_character_input(input: &CharacterInput) -> CharacterInputPermutation {
    if input.chosen.class_levels.len() > 1 {
        return CharacterInputPermutation::Multiclass;
    }

    let is_level_one_or_absent = input
        .chosen
        .class_levels
        .first()
        .map(|class_level| class_level.level <= 1)
        .unwrap_or(true);

    let has_any_choice = !input.chosen.selected_feats.is_empty()
        || !input.chosen.skill_allocations.is_empty()
        || !input.chosen.equipment_selections.is_empty()
        || !input.chosen.spells_selected.is_empty()
        || !input.chosen.selected_choices.is_empty();

    if is_level_one_or_absent && !has_any_choice {
        CharacterInputPermutation::BrandNew
    } else {
        CharacterInputPermutation::MidBuild
    }
}
