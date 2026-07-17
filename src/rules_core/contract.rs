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
//! `PilotReceipt` types land in this cycle (cycle 2). The printed-sheet
//! cell map is a later Epic-1 work-unit (not this cycle) — see the loop
//! instruction's Step 2 and this bundle's progress doc for the current
//! frontier.

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::pilot_compute::{ComputationDiagnostic, PilotBaseChassisComputation};
use crate::rules_core::pilot_compute_corpus::{CorpusDerivedSection, CorpusPilotReceipt};

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

/// The boundary contract's `PilotReceipt` shape (`technical-design.md`
/// §1.1 "Outputs"): per-derived-stat fields, per-source-record fields
/// with provenance, and diagnostic fields with `claim_blocking`
/// preserved.
///
/// This does not duplicate the existing `PilotBaseChassisComputation` /
/// `CorpusPilotReceipt` shapes from scratch — `pilot_compute_corpus.rs`'s
/// own doc comment already notes "the real chassis function returns
/// `PilotBaseChassisComputation`... `PilotReceipt` in the doctrine doc's
/// illustrative code does not exist in this repo". Instead this composes
/// with the existing shapes: `chassis` is the unchanged per-derived-stat
/// surface (`PilotBaseChassisComputation`), `corpus_derived` is the
/// unchanged per-source-record-with-provenance surface
/// (`CorpusDerivedSection`, carrying `TableCellRef`s), and `diagnostics`
/// hoists the chassis's diagnostics (`claim_blocking` preserved
/// bit-for-bit) to the receipt's top level per the contract's
/// "Diagnostic fields" clause.
#[derive(Debug, Clone, PartialEq)]
pub struct PilotReceipt {
    /// Per-derived-stat fields (BAB, saves, HP, AC, attack bonus, ability
    /// mods, selected skill modifiers) — the unchanged chassis
    /// computation.
    pub chassis: PilotBaseChassisComputation,
    /// Per-source-record fields with `TableCellRef` provenance
    /// (spell-school coverage, resolved equipment) — the unchanged
    /// corpus-derived section.
    pub corpus_derived: CorpusDerivedSection,
    /// Diagnostic fields; `claim_blocking: true` diagnostics are
    /// preserved unchanged from the chassis computation.
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// Build the boundary contract's `PilotReceipt` from the corpus-aware
/// compute seam's existing output (`compute_pilot_with_corpus` in
/// `pilot_compute_corpus.rs`). See `PilotReceipt`'s doc comment for why
/// this wraps rather than duplicates the existing shapes.
pub fn to_pilot_receipt(receipt: &CorpusPilotReceipt) -> PilotReceipt {
    PilotReceipt {
        diagnostics: receipt.base.diagnostics.clone(),
        chassis: receipt.base.clone(),
        corpus_derived: receipt.corpus_derived.clone(),
    }
}
