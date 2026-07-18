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
use crate::rules_core::feat_prereqs::{
    compute_feat_effects, evaluate_feat_prerequisites, FeatEffects, FeatKey,
    PrerequisiteEvaluation,
};
use crate::rules_core::pilot_compute::{ComputationDiagnostic, PilotBaseChassisComputation};
use crate::rules_core::pilot_compute_corpus::{CorpusDerivedSection, CorpusPilotReceipt};
use crate::rules_core::rules_tables::crb::feats::feat_tables;
use crate::rules_core::skill_allocation::{allocate_skill_ranks, SkillTotals};
use crate::rules_core::source_content::SourcePackageContent;
use crate::rules_core::spellbook::{compute_spellbook_coverage, SpellbookCoverage};

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
    /// Epic 4's real skill-rank allocation totals
    /// (`skill_allocation::allocate_skill_ranks`), wired in by the
    /// `contract:skill_wiring` cycle (`adaptive-squishing-mccarthy.md`).
    /// Per that module's own doc comment, every diagnostic
    /// `allocate_skill_ranks` produces is `claim_blocking: false` — it
    /// never fabricates a total, it either computes a real one or omits
    /// the skill from `totals`/`untrained_use` entirely. This replaces
    /// (not supplements) the old narrow single-posture check that used to
    /// gate `sheet.skill.*` cells; see `printed_sheet_cell_map`'s doc
    /// comment for the cell-level consequence.
    pub skills: SkillTotals,
    /// Epic 2's real spellbook coverage
    /// (`spellbook::compute_spellbook_coverage`), wired in by the
    /// `contract:spellbook_wiring` cycle (`adaptive-squishing-mccarthy.md`).
    /// Per that cycle's "Not every epic output becomes a sheet cell"
    /// design decision, only `slots_total`, `slots_used`, and
    /// `spell_save_dc` are flattened into `printed_sheet_cell_map` cells
    /// (one dynamic cell per present `BTreeMap` key); `spells_prepared`,
    /// `spells_known`, and `school_specialization` do not reduce to
    /// `PrintedSheetCellValue::Number(i16) | Blocked` cleanly and stay
    /// reachable only via this field directly. As of this cycle,
    /// `slots_total`/`slots_used` are always empty `BTreeMap`s for every
    /// character (confirmed by reading `spellbook.rs`: no slot-math code
    /// exists yet) -- cell generation below is still correct and ready
    /// for when that lands.
    pub spellbook: SpellbookCoverage,
    /// Epic 3's real feat prerequisite eligibility + effects
    /// (`feat_prereqs::{evaluate_feat_prerequisites, compute_feat_effects}`),
    /// wired in by the `contract:feat_wiring` cycle
    /// (`adaptive-squishing-mccarthy.md`). One `ResolvedFeat` per entry in
    /// `input.chosen.selected_feats` that resolves against
    /// `rules_tables::crb::feats::feat_tables()` (matching
    /// `entry.key == feat_id || entry.name == feat_id`, per that cycle's
    /// "Feat resolution" design decision) -- an unmatched selected-feat
    /// string is honestly skipped, never fabricated into a made-up
    /// category. See `to_pilot_receipt`'s doc comment for the resolution
    /// mechanism.
    ///
    /// **Scope boundary, deliberate**: this cycle adds NO new
    /// `printed_sheet_cell_map` cells. `PrerequisiteEvaluation` and
    /// `FeatEffects` don't reduce to a single `PrintedSheetCellValue`
    /// (`Number(i16) | Blocked`) -- they carry prose (failure reasons,
    /// descriptions) and structured provenance, not a sheet number.
    /// Numeric feat-derived combat bonuses already flow through Epic 6's
    /// separate `resolve_feat_damage_effect` path (`damage_total.rs`),
    /// which is unrelated to this struct. `receipt.feats` is reachable
    /// directly by callers, matching the same "not every epic output
    /// becomes a sheet cell" precedent `spellbook` above already set.
    pub feats: Vec<ResolvedFeat>,
}

/// One selected feat, resolved against the CRB feat catalog and evaluated
/// through Epic 3's engine (`feat_prereqs::{evaluate_feat_prerequisites,
/// compute_feat_effects}`). See `PilotReceipt.feats`'s doc comment for the
/// resolution mechanism and this cycle's cell-map scope boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFeat {
    /// The matched catalog entry's `key` (equivalently `name` -- see
    /// `feats.rs`: every landed record has `key == name` today), not
    /// necessarily byte-identical to the raw `selected_feats` string that
    /// matched it (a namespaced id like `feat:dodge` could match on
    /// either side of the `||`, but this cycle's fixture usage always
    /// matches on the plain catalog name).
    pub feat_id: String,
    pub prerequisites: PrerequisiteEvaluation,
    pub effects: FeatEffects,
}

/// Build the boundary contract's `PilotReceipt` from the corpus-aware
/// compute seam's existing output (`compute_pilot_with_corpus` in
/// `pilot_compute_corpus.rs`). See `PilotReceipt`'s doc comment for why
/// this wraps rather than duplicates the existing shapes.
///
/// `input` and `corpus` are the raw `CharacterInput` and
/// `SourcePackageContent` that produced `receipt`. Widened by the
/// `contract:receipt_signature_threading` cycle (cycle 0) so later cycles
/// can call SD-20's Epic 2-7 engines (spellbook, feat prereqs, skill
/// allocation, equipment effects, damage total), none of which are
/// reachable from `CorpusPilotReceipt` alone.
///
/// The `contract:skill_wiring` cycle (`adaptive-squishing-mccarthy.md`)
/// was the first to actually use `input`: it calls Epic 4's
/// `allocate_skill_ranks(input)` to populate `PilotReceipt.skills`. See
/// that field's doc comment and `skill_allocation.rs` for what it
/// computes. The `contract:spellbook_wiring` cycle is the first to use
/// `corpus`: it calls Epic 2's
/// `compute_spellbook_coverage(input, corpus)` to populate
/// `PilotReceipt.spellbook`. See that field's doc comment and
/// `spellbook.rs` for what it computes.
///
/// The `contract:feat_wiring` cycle (Cycle 3) populates
/// `PilotReceipt.feats`: each entry in `input.chosen.selected_feats` is
/// resolved against `rules_tables::crb::feats::feat_tables()` by matching
/// `entry.key == feat_id || entry.name == feat_id` (per
/// `adaptive-squishing-mccarthy.md`'s "Feat resolution" design decision --
/// `selected_feats` carries no category field of its own, but the catalog
/// already carries `key`/`name` -> `category`). A match yields a
/// `feat_prereqs::FeatKey { feat_id: matched_entry.key, category:
/// matched_entry.category }`, fed to both `evaluate_feat_prerequisites`
/// and `compute_feat_effects` to build one `ResolvedFeat`. An unmatched
/// selected-feat string (e.g. a namespaced id the catalog does not carry,
/// or a typo) produces no `ResolvedFeat` at all -- the same honest-skip
/// discipline `feats.rs`'s own catalog generator already applies to
/// corpus records it cannot classify.
pub fn to_pilot_receipt(
    receipt: &CorpusPilotReceipt,
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> PilotReceipt {
    let feats = input
        .chosen
        .selected_feats
        .iter()
        .filter_map(|feat_id| {
            feat_tables()
                .iter()
                .find(|entry| entry.key == feat_id || entry.name == feat_id)
        })
        .map(|matched_entry| {
            let key = FeatKey {
                feat_id: matched_entry.key.to_string(),
                category: matched_entry.category,
            };
            ResolvedFeat {
                feat_id: matched_entry.key.to_string(),
                prerequisites: evaluate_feat_prerequisites(&key),
                effects: compute_feat_effects(&key),
            }
        })
        .collect();

    PilotReceipt {
        diagnostics: receipt.base.diagnostics.clone(),
        chassis: receipt.base.clone(),
        corpus_derived: receipt.corpus_derived.clone(),
        skills: allocate_skill_ranks(input),
        spellbook: compute_spellbook_coverage(input, corpus),
        feats,
    }
}

/// The diagnostic id that, when `claim_blocking: true`, means the chassis
/// as a whole has no supported single-class posture
/// (`compute_pilot_base_chassis`'s `class_chassis.unsupported`). The
/// chassis-dependent `PilotReceipt` fields it zeroes (base attack bonus,
/// total saves, and the deterministic baseline melee attack bonus / armor
/// class) are not real data in that case — the cell map must render
/// `PrintedSheetCellValue::Blocked` for the cells sourced from them rather
/// than show the zero as if it were a computed number.
///
/// `chassis.selected_skill_modifiers` is also zeroed by this diagnostic
/// at the `PilotBaseChassisComputation` level, but as of the
/// `contract:skill_wiring` cycle no `sheet.skill.*` cell sources from
/// that chassis field any more — they source from
/// `PilotReceipt.skills.totals` (Epic 4's `allocate_skill_ranks`)
/// instead, which is not gated by this diagnostic at all. See
/// `printed_sheet_cell_map`'s doc comment for why.
///
/// This is a *universal* fallback: a wholly-unsupported class posture
/// blocks every chassis-dependent cell. It is additively layered (OR'd)
/// with the two more specific diagnostic ids below — each of those can
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
/// chassis-dependent field: `compute_total_saves` and
/// `compute_combat_baseline` each check their own, more specific posture
/// conditions (beyond just "is the Fighter chassis supported") and push
/// their own claim-blocking diagnostic ids when those are unmet —
/// independently of whether `class_chassis.unsupported` fires. So each of
/// those chassis-dependent cells below is gated on
/// `class_chassis.unsupported` OR'd with whichever of those more specific
/// diagnostic ids actually governs its source field. Ability modifiers
/// are computed directly from ability scores independent of chassis
/// support, so they are never blocked by any of these diagnostics.
///
/// The five `sheet.skill.*` cells (climb, intimidate, swim, diplomacy,
/// disable_device) are a different case as of the `contract:skill_wiring`
/// cycle: they no longer source from the chassis's old
/// `compute_selected_skill_modifiers` single-posture check at all — they
/// source from `PilotReceipt.skills.totals` (Epic 4's real
/// `allocate_skill_ranks`, called in `to_pilot_receipt`). Every
/// diagnostic `allocate_skill_ranks` can produce is `claim_blocking:
/// false` (see `skill_allocation.rs`'s `SkillTotals::diagnostics` doc
/// comment), so none of these five cells is ever blocked by a
/// skill-specific diagnostic, and they are not gated on
/// `class_chassis.unsupported` either (the ability modifiers
/// `allocate_skill_ranks` uses come from the chassis's own
/// `AbilityModifiers`, which — like the standalone ability-modifier cells
/// above — are computed directly from ability scores). The only way one
/// of these five cells renders `Blocked` is genuine absence of data: a
/// skill with no entry at all in `input.chosen.skill_allocations` (never
/// allocated, not even at 0 ranks) has no entry in
/// `receipt.skills.totals` — `allocate_skill_ranks` only produces a
/// result for skills the player actually submitted an allocation for; it
/// does not enumerate its whole bounded skill universe regardless of
/// whether the character touched it, and `SkillTotals.untrained_use` is
/// populated from that same per-allocation loop, so it has no fallback
/// entry for an unallocated skill either. `Blocked` here means "no
/// computed value exists", not "a diagnostic gated this" and not a
/// fabricated `Number(0)`.
///
/// The `sheet.spellbook.*` cells (as of the `contract:spellbook_wiring`
/// cycle) are a third, dynamic case: `receipt.spellbook.slots_total`,
/// `.slots_used`, and `.spell_save_dc` are each `BTreeMap`s keyed by
/// spell level (`u8`) or class id (`String`), not fixed single fields, so
/// this function emits one cell per *present* key
/// (`sheet.spellbook.slots_total.<level>`,
/// `sheet.spellbook.slots_used.<level>`,
/// `sheet.spellbook.spell_save_dc.<class_id>`) rather than a fixed set of
/// cell ids. A non-caster (or any character with an empty map for one of
/// these fields) naturally produces zero cells of that kind — never a
/// fabricated placeholder cell for a key that is not present.
/// `compute_spellbook_coverage` pushes no diagnostics at all (`spellbook.rs`
/// has no `claim_blocking` machinery), so none of these cells is ever
/// `Blocked`; absence is expressed purely by the cell not existing in the
/// returned `Vec`. Per `adaptive-squishing-mccarthy.md`'s "Not every epic
/// output becomes a sheet cell" design decision, `spells_prepared`,
/// `spells_known`, and `school_specialization` are deliberately NOT
/// flattened into cells here — they don't reduce to
/// `PrintedSheetCellValue::Number(i16) | Blocked` cleanly and stay
/// reachable via `receipt.spellbook` directly.
pub fn printed_sheet_cell_map(receipt: &PilotReceipt) -> Vec<PrintedSheetCell> {
    let chassis_unsupported =
        diagnostic_blocking(receipt, CLASS_CHASSIS_UNSUPPORTED_DIAGNOSTIC_ID);
    let total_save_unsupported = diagnostic_blocking(receipt, TOTAL_SAVE_UNSUPPORTED_DIAGNOSTIC_ID);
    let combat_baseline_unsupported =
        diagnostic_blocking(receipt, COMBAT_BASELINE_UNSUPPORTED_DIAGNOSTIC_ID);

    // Base attack bonus has no dedicated diagnostic beyond
    // `class_chassis.unsupported` -- `compute_fighter_chassis` is its only
    // writer, and it pushes only that one id.
    let base_attack_bonus_blocked = chassis_unsupported;
    let save_blocked = chassis_unsupported || total_save_unsupported;
    let combat_baseline_blocked = chassis_unsupported || combat_baseline_unsupported;

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

    // See this function's doc comment for why these five skill cells are
    // sourced from `receipt.skills.totals` (Epic 4's real
    // `allocate_skill_ranks`) rather than the chassis's old single-posture
    // check, and why `Blocked` here means "no entry at all", never a
    // diagnostic gate.
    let skill_cell = |cell_id: &str, source_field: &str, skill_id: &str| PrintedSheetCell {
        cell_id: cell_id.to_owned(),
        source_field: source_field.to_owned(),
        value: match receipt.skills.totals.get(skill_id) {
            Some(total) => PrintedSheetCellValue::Number(total.total_modifier as i16),
            None => PrintedSheetCellValue::Blocked,
        },
    };

    let chassis = &receipt.chassis;

    let mut cells = vec![
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
        skill_cell(
            "sheet.skill.climb",
            "skills.totals.skill:climb.total_modifier",
            "skill:climb",
        ),
        skill_cell(
            "sheet.skill.intimidate",
            "skills.totals.skill:intimidate.total_modifier",
            "skill:intimidate",
        ),
        skill_cell(
            "sheet.skill.swim",
            "skills.totals.skill:swim.total_modifier",
            "skill:swim",
        ),
        skill_cell(
            "sheet.skill.diplomacy",
            "skills.totals.skill:diplomacy.total_modifier",
            "skill:diplomacy",
        ),
        skill_cell(
            "sheet.skill.disable_device",
            "skills.totals.skill:disable_device.total_modifier",
            "skill:disable_device",
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
    ];

    // See this function's doc comment for why these three families are
    // dynamic (one cell per present BTreeMap key) rather than a fixed set
    // of cell ids, and why an empty map naturally yields zero cells of
    // that kind.
    for (&level, &slots) in &receipt.spellbook.slots_total {
        cells.push(independent_cell(
            &format!("sheet.spellbook.slots_total.{level}"),
            &format!("spellbook.slots_total.{level}"),
            slots as i16,
        ));
    }
    for (&level, &slots) in &receipt.spellbook.slots_used {
        cells.push(independent_cell(
            &format!("sheet.spellbook.slots_used.{level}"),
            &format!("spellbook.slots_used.{level}"),
            slots as i16,
        ));
    }
    for (class_id, &dc) in &receipt.spellbook.spell_save_dc {
        cells.push(independent_cell(
            &format!("sheet.spellbook.spell_save_dc.{class_id}"),
            &format!("spellbook.spell_save_dc.{class_id}"),
            dc as i16,
        ));
    }

    cells
}
