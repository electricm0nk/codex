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
//! `PilotReceipt` types landed in cycle 2. The printed-sheet cell map
//! (`PrintedSheetCell` / `PrintedSheetCellValue` / `printed_sheet_cell_map`)
//! lands in this cycle (cycle 3) — see the loop instruction's Step 2 and
//! this bundle's progress doc for the current frontier.

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::pilot_compute::{ComputationDiagnostic, PilotBaseChassisComputation};
use crate::rules_core::pilot_compute_corpus::{CorpusDerivedSection, CorpusPilotReceipt};
use crate::rules_core::source_content::SourcePackageContent;

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
///
/// `input` and `corpus` are the raw `CharacterInput` and
/// `SourcePackageContent` that produced `receipt`. This cycle
/// (`contract:receipt_signature_threading`, per
/// `adaptive-squishing-mccarthy.md`) widens the signature to accept them
/// so later cycles can call SD-20's Epic 2-7 engines (spellbook, feat
/// prereqs, skill allocation, equipment effects, damage total), none of
/// which are reachable from `CorpusPilotReceipt` alone. This cycle does
/// not change behavior: `input`/`corpus` are unused by the body below.
pub fn to_pilot_receipt(
    receipt: &CorpusPilotReceipt,
    _input: &CharacterInput,
    _corpus: &SourcePackageContent,
) -> PilotReceipt {
    PilotReceipt {
        diagnostics: receipt.base.diagnostics.clone(),
        chassis: receipt.base.clone(),
        corpus_derived: receipt.corpus_derived.clone(),
    }
}

/// The diagnostic id that, when `claim_blocking: true`, means the chassis
/// as a whole has no supported single-class posture
/// (`compute_pilot_base_chassis`'s `class_chassis.unsupported`). The
/// chassis-dependent `PilotReceipt` fields it zeroes (base attack bonus,
/// total saves, the deterministic baseline melee attack bonus / armor
/// class, and the selected skill modifiers) are not real data in that
/// case — the cell map must render `PrintedSheetCellValue::Blocked` for
/// the cells sourced from them rather than show the zero as if it were a
/// computed number.
///
/// This is a *universal* fallback: a wholly-unsupported class posture
/// blocks every chassis-dependent cell. It is additively layered (OR'd)
/// with the three more specific diagnostic ids below — each of those can
/// fire independently of this one (e.g. a supported Fighter chassis whose
/// combat-baseline equipment posture is wrong still leaves
/// `class_chassis.unsupported` un-fired), so `printed_sheet_cell_map` must
/// check whichever ids are actually relevant to each specific cell's
/// computation, not just this one uniformly for every chassis-dependent
/// cell (see `tests/sd20_tabletop_readiness_integration.rs`'s Finding 2,
/// which originally pinned this gap as a regression test).
const CLASS_CHASSIS_UNSUPPORTED_DIAGNOSTIC_ID: &str = "class_chassis.unsupported";

/// The diagnostic id `pilot_compute.rs::compute_total_saves` pushes
/// (`claim_blocking: true`) when its own supported-Fighter-chassis check
/// fails, zeroing `chassis.total_saves.{fortitude,reflex,will}`. Gates the
/// `sheet.save.*` cells, additively with
/// `CLASS_CHASSIS_UNSUPPORTED_DIAGNOSTIC_ID`.
const TOTAL_SAVE_UNSUPPORTED_DIAGNOSTIC_ID: &str = "defense.total_save.unsupported";

/// The diagnostic id `pilot_compute.rs::compute_combat_baseline` pushes
/// (`claim_blocking: true`) whenever the exact deterministic
/// Longsword/Chain Shirt/Dodge/no-shield combat posture
/// (`unmet_combat_posture_conditions`) is not fully met, zeroing
/// `chassis.baseline_melee_attack_bonus` and `chassis.baseline_armor_class`.
/// `unmet_combat_posture_conditions` checks equipment/feat conditions
/// (Longsword and Chain Shirt equipped, no shield, Power Attack selected
/// but inactive, Dodge and Weapon Focus selected) in addition to the
/// supported-Fighter-chassis check, so this can fire even when the Fighter
/// chassis itself is fully supported (`class_chassis.unsupported` does not
/// fire) — it must be checked independently. Gates the `sheet.armor_class`
/// / `sheet.melee_attack_bonus` cells, additively with
/// `CLASS_CHASSIS_UNSUPPORTED_DIAGNOSTIC_ID`.
const COMBAT_BASELINE_UNSUPPORTED_DIAGNOSTIC_ID: &str = "combat.baseline_unsupported";

/// The diagnostic id
/// `pilot_compute.rs::compute_selected_skill_modifiers` pushes
/// (`claim_blocking: true`) whenever the exact deterministic
/// Climb/Intimidate/Swim rank-1 posture (plus the grounded Chain Shirt
/// armor-check posture the Climb/Swim totals depend on,
/// `unmet_selected_skill_posture_conditions`) is not fully met, zeroing
/// `chassis.selected_skill_modifiers.{climb,intimidate,swim}`. Like the
/// combat-baseline diagnostic above, this checks conditions beyond the
/// supported-Fighter-chassis check (exact skill allocations, no widening
/// beyond the three selected skills, Chain Shirt equipped), so it can fire
/// even when `class_chassis.unsupported` does not — it must be checked
/// independently. Gates the `sheet.skill.*` cells, additively with
/// `CLASS_CHASSIS_UNSUPPORTED_DIAGNOSTIC_ID`.
const SKILL_SELECTED_MODIFIER_UNSUPPORTED_DIAGNOSTIC_ID: &str =
    "skill.selected_modifier.unsupported";

/// A single row of the printed PF1 character sheet
/// (`technical-design.md` §1.1 "Cells"): a stable cell id and the value
/// resolved from exactly one named `PilotReceipt` field. The GUI cannot
/// invent a value; it renders what this map gives it.
#[derive(Debug, Clone, PartialEq)]
pub struct PrintedSheetCell {
    /// Stable cell id (e.g. `sheet.base_attack_bonus`).
    pub cell_id: String,
    /// The exact `PilotReceipt` field path this cell renders, for
    /// auditability (e.g. `chassis.base_attack_bonus`).
    pub source_field: String,
    pub value: PrintedSheetCellValue,
}

/// A printed-sheet cell's rendered value. `Blocked` is the "blocked — see
/// diagnostics" rendering `technical-design.md` §1.1 requires when the
/// cell's source field is claim-blocked — never a fabricated number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintedSheetCellValue {
    Number(i16),
    Blocked,
}

/// Return whether `receipt` carries a `claim_blocking: true` diagnostic
/// with the given id. Used to gate `printed_sheet_cell_map`'s cells on
/// exactly the diagnostic ids that are relevant to each cell's underlying
/// computation (see the diagnostic id constants' doc comments above).
fn diagnostic_blocking(receipt: &PilotReceipt, diagnostic_id: &str) -> bool {
    receipt
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.id == diagnostic_id && diagnostic.claim_blocking)
}

/// Build the printed-sheet cell map (`technical-design.md` §1.1 "Cells")
/// from a `PilotReceipt`. Each cell points at exactly one `PilotReceipt`
/// field; cells sourced from a chassis field that a relevant claim-blocking
/// diagnostic has zeroed render `PrintedSheetCellValue::Blocked` instead of
/// that zero, rather than presenting the fabricated zero as real data.
///
/// `class_chassis.unsupported` is a universal fallback that blocks every
/// chassis-dependent cell (a wholly-unsupported class posture invalidates
/// all of them), but it is not the *only* diagnostic that can zero a
/// chassis-dependent field: `compute_total_saves`,
/// `compute_combat_baseline`, and `compute_selected_skill_modifiers` each
/// check their own, more specific posture conditions (beyond just "is the
/// Fighter chassis supported") and push their own claim-blocking
/// diagnostic ids when those are unmet — independently of whether
/// `class_chassis.unsupported` fires. So each chassis-dependent cell below
/// is gated on `class_chassis.unsupported` OR'd with whichever of those
/// more specific diagnostic ids actually governs its source field. Ability
/// modifiers are computed directly from ability scores independent of
/// chassis support, so they are never blocked by any of these diagnostics.
pub fn printed_sheet_cell_map(receipt: &PilotReceipt) -> Vec<PrintedSheetCell> {
    let chassis_unsupported =
        diagnostic_blocking(receipt, CLASS_CHASSIS_UNSUPPORTED_DIAGNOSTIC_ID);
    let total_save_unsupported = diagnostic_blocking(receipt, TOTAL_SAVE_UNSUPPORTED_DIAGNOSTIC_ID);
    let combat_baseline_unsupported =
        diagnostic_blocking(receipt, COMBAT_BASELINE_UNSUPPORTED_DIAGNOSTIC_ID);
    let skill_modifier_unsupported =
        diagnostic_blocking(receipt, SKILL_SELECTED_MODIFIER_UNSUPPORTED_DIAGNOSTIC_ID);

    // Base attack bonus has no dedicated diagnostic beyond
    // `class_chassis.unsupported` -- `compute_fighter_chassis` is its only
    // writer, and it pushes only that one id.
    let base_attack_bonus_blocked = chassis_unsupported;
    let save_blocked = chassis_unsupported || total_save_unsupported;
    let combat_baseline_blocked = chassis_unsupported || combat_baseline_unsupported;
    let skill_modifier_blocked = chassis_unsupported || skill_modifier_unsupported;

    let cell = |cell_id: &str, source_field: &str, value: i16, blocked: bool| PrintedSheetCell {
        cell_id: cell_id.to_owned(),
        source_field: source_field.to_owned(),
        value: if blocked {
            PrintedSheetCellValue::Blocked
        } else {
            PrintedSheetCellValue::Number(value)
        },
    };

    let independent_cell = |cell_id: &str, source_field: &str, value: i16| PrintedSheetCell {
        cell_id: cell_id.to_owned(),
        source_field: source_field.to_owned(),
        value: PrintedSheetCellValue::Number(value),
    };

    let chassis = &receipt.chassis;

    vec![
        cell(
            "sheet.base_attack_bonus",
            "chassis.base_attack_bonus",
            chassis.base_attack_bonus,
            base_attack_bonus_blocked,
        ),
        cell(
            "sheet.save.fortitude",
            "chassis.total_saves.fortitude",
            chassis.total_saves.fortitude,
            save_blocked,
        ),
        cell(
            "sheet.save.reflex",
            "chassis.total_saves.reflex",
            chassis.total_saves.reflex,
            save_blocked,
        ),
        cell(
            "sheet.save.will",
            "chassis.total_saves.will",
            chassis.total_saves.will,
            save_blocked,
        ),
        cell(
            "sheet.armor_class",
            "chassis.baseline_armor_class",
            chassis.baseline_armor_class,
            combat_baseline_blocked,
        ),
        cell(
            "sheet.melee_attack_bonus",
            "chassis.baseline_melee_attack_bonus",
            chassis.baseline_melee_attack_bonus,
            combat_baseline_blocked,
        ),
        cell(
            "sheet.skill.climb",
            "chassis.selected_skill_modifiers.climb",
            chassis.selected_skill_modifiers.climb,
            skill_modifier_blocked,
        ),
        cell(
            "sheet.skill.intimidate",
            "chassis.selected_skill_modifiers.intimidate",
            chassis.selected_skill_modifiers.intimidate,
            skill_modifier_blocked,
        ),
        cell(
            "sheet.skill.swim",
            "chassis.selected_skill_modifiers.swim",
            chassis.selected_skill_modifiers.swim,
            skill_modifier_blocked,
        ),
        independent_cell(
            "sheet.ability_modifier.strength",
            "chassis.ability_modifiers.strength",
            chassis.ability_modifiers.strength,
        ),
        independent_cell(
            "sheet.ability_modifier.dexterity",
            "chassis.ability_modifiers.dexterity",
            chassis.ability_modifiers.dexterity,
        ),
        independent_cell(
            "sheet.ability_modifier.constitution",
            "chassis.ability_modifiers.constitution",
            chassis.ability_modifiers.constitution,
        ),
        independent_cell(
            "sheet.ability_modifier.intelligence",
            "chassis.ability_modifiers.intelligence",
            chassis.ability_modifiers.intelligence,
        ),
        independent_cell(
            "sheet.ability_modifier.wisdom",
            "chassis.ability_modifiers.wisdom",
            chassis.ability_modifiers.wisdom,
        ),
        independent_cell(
            "sheet.ability_modifier.charisma",
            "chassis.ability_modifiers.charisma",
            chassis.ability_modifiers.charisma,
        ),
    ]
}
