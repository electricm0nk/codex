//! SD-20 boundary contract — Epic 1.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.1 and
//! `technical-design.md` §1.1, the boundary contract is the engine-side
//! API surface every other SD-20 epic produces into: the `CharacterInput`
//! shapes the engine accepts, the `PilotReceipt` shape it returns, and
//! the printed-sheet cell map the GUI renders from. This module is the
//! contract's code-level home; `docs/release/SD-20/boundary-contract.md` is its
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

use crate::rules_core::character_input::{ActiveState, CharacterInput, EquipmentSelection};
use crate::rules_core::damage_total::{resolve_weapon_damage_breakdown, WeaponDamageBreakdown};
use crate::rules_core::encumbrance::{compute_encumbrance, EncumbranceComputation};
use crate::rules_core::race_resolver::race_size_for_race_token;
use crate::rules_core::size::SizeCategory;
use crate::rules_core::equipment_effects::{compute_equipment_effects, EquipmentEffects};
use crate::rules_core::feat_prereqs::{
    compute_feat_effects, evaluate_feat_prerequisites, FeatEffects, FeatKey,
    PrerequisiteEvaluation,
};
use crate::rules_core::level_up::{compute_level_up_grants, LevelUpPlan};
use crate::rules_core::pilot_compute::{
    apply_human_ability_bonus, ComputationDiagnostic, PilotBaseChassisComputation,
};
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
    /// design decision, only `spell_save_dc` is flattened into
    /// `printed_sheet_cell_map` cells (one dynamic cell per present
    /// `BTreeMap` key); `spells_prepared`, `spells_known`, and
    /// `school_specialization` do not reduce to
    /// `PrintedSheetCellValue::Number(i16) | Blocked` cleanly and stay
    /// reachable only via this field directly. `SpellbookCoverage` never
    /// carried `slots_total`/`slots_used` fields at all (epic-31-spell-wiring
    /// gap closure, `decisions.md` Decision 37 -- populating them would
    /// have duplicated the already-real, already-tested
    /// `class_spell.*.total_spells_per_day.*` chassis computation the
    /// desktop app's "Spells per day" section already renders).
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
    /// Epic 5's real equipment-effect aggregate
    /// (`equipment_effects::compute_equipment_effects`), wired in by the
    /// `contract:equipment_wiring` cycle (`adaptive-squishing-mccarthy.md`).
    /// Computed over `input.chosen.equipment_selections` filtered to
    /// `active_state == ActiveState::EquippedActive` only -- a selection
    /// that is merely `SelectedInactive` or `Absent` contributes nothing,
    /// matching the same filtering `to_pilot_receipt`'s doc comment
    /// documents.
    ///
    /// **Scope boundary, deliberate**: `EquipmentEffects.spell_failure_chance:
    /// Option<f32>` is NOT flattened into a `printed_sheet_cell_map` cell
    /// this cycle -- a fractional percentage doesn't cleanly fit
    /// `PrintedSheetCellValue::Number(i16)` without a real type extension,
    /// which is out of this cycle's scope (same "not every epic output
    /// becomes a sheet cell" precedent `spellbook`/`feats` above already
    /// set). It stays reachable via `receipt.equipment_effects.spell_failure_chance`
    /// directly. `armor_class_delta` (a plain `i16`, always real) and
    /// `max_dex_cap` (an `Option<i16>`, cell present only when `Some`) DO
    /// become cells -- see `printed_sheet_cell_map`'s doc comment.
    pub equipment_effects: EquipmentEffects,
    /// Epic 6's real per-weapon damage breakdown
    /// (`damage_total::resolve_weapon_damage_breakdown`), wired in by the
    /// `contract:damage_wiring` cycle (Cycle 5b,
    /// `adaptive-squishing-mccarthy.md`). Computed by reusing the exact
    /// same `equipment_effects` local `to_pilot_receipt` already built for
    /// `PilotReceipt.equipment_effects` above (not recomputed a second
    /// time -- see that field's doc comment and `to_pilot_receipt`'s doc
    /// comment for why the local was kept separate precisely for this
    /// reuse) and `chassis.ability_modifiers.strength` (the
    /// already-computed STR modifier from the unchanged chassis
    /// computation, `PilotBaseChassisComputation::ability_modifiers`).
    /// One `WeaponDamageBreakdown` per `EquippedActive` equipped item that
    /// `resolve_base_damage_dice` identifies as a weapon (carries a
    /// `DAMAGE:` corpus token); a non-weapon equipped item (e.g. armor)
    /// is silently absent from this `Vec`, never represented with `None`
    /// fields -- see `resolve_weapon_damage_breakdown`'s own doc comment
    /// for the full identification/limitation discipline (including its
    /// `WeaponHandSlot::Primary`-only bound).
    ///
    /// **Scope boundary, deliberate**: this cycle adds NO new
    /// `printed_sheet_cell_map` cells. No summed "damage roll total"
    /// formula (base dice + STR + weapon enhancement + feat bonuses,
    /// combined into one number) exists anywhere in this codebase --
    /// inventing one here would be exactly the fabrication this project's
    /// discipline forbids (see `adaptive-squishing-mccarthy.md`'s "No
    /// fabricated damage total" design decision). `receipt.weapon_damage`
    /// stays the structured per-weapon breakdown, reachable directly by
    /// callers, matching the same "not every epic output becomes a sheet
    /// cell" precedent `spellbook`/`feats`/`equipment_effects` above
    /// already set. A future, separate cycle owns turning this structured
    /// data into a summed display number, if that is ever wanted.
    pub weapon_damage: Vec<WeaponDamageBreakdown>,
    /// v0.6 alpha swarm task 5's real carrying-capacity/encumbrance
    /// computation (`encumbrance::compute_encumbrance`). Computed over
    /// every `EquippedActive` or `SelectedInactive` equipment selection
    /// (both represent items the character possesses; `Absent` does not
    /// contribute weight -- see `EncumbranceComputation`'s own doc
    /// comment), against the character's real *effective* Strength score
    /// (`apply_human_ability_bonus`, the same Human-racial-bonus-aware
    /// value the chassis computation itself derives -- not the raw
    /// pre-bonus `input.chosen.ability_scores.strength`).
    pub encumbrance: EncumbranceComputation,
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
///
/// The `contract:equipment_wiring` cycle (Cycle 4) populates
/// `PilotReceipt.equipment_effects`: `input.chosen.equipment_selections` is
/// filtered to `active_state == ActiveState::EquippedActive` first (a
/// `SelectedInactive` or `Absent` selection contributes nothing), then the
/// filtered slice is fed to Epic 5's real
/// `equipment_effects::compute_equipment_effects(&equipped, corpus)`. Both
/// the filtered `equipped` slice and the resulting `EquipmentEffects` are
/// kept as their own local variables (not inlined into the `PilotReceipt`
/// literal below) precisely so Cycle 5b (damage wiring) can reuse the
/// exact same `EquipmentEffects` value when calling
/// `damage_total::resolve_weapon_damage_breakdown` without recomputing it.
///
/// The `contract:damage_wiring` cycle (Cycle 5b) populates
/// `PilotReceipt.weapon_damage`: it calls Epic 6's real
/// `damage_total::resolve_weapon_damage_breakdown(input, corpus,
/// &equipment_effects, chassis_str_modifier)`, reusing this same
/// `equipment_effects` local (computed once, immediately above, for
/// `PilotReceipt.equipment_effects`) rather than recomputing
/// `compute_equipment_effects` a second time, and reusing
/// `receipt.base.ability_modifiers.strength` -- the already-computed STR
/// modifier from the unchanged chassis computation -- rather than
/// re-deriving it from `input.chosen.ability_scores.strength`.
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

    // Pre-filtered to EquippedActive only -- see this function's doc
    // comment. Kept as its own local (not inlined) so Cycle 5b can reuse
    // both `equipped` and `equipment_effects` verbatim.
    let equipped: Vec<EquipmentSelection> = input
        .chosen
        .equipment_selections
        .iter()
        .filter(|selection| selection.active_state == ActiveState::EquippedActive)
        .cloned()
        .collect();
    let equipment_effects = compute_equipment_effects(&equipped, corpus);

    // Reuses `equipment_effects` (built immediately above for
    // `PilotReceipt.equipment_effects`) and the chassis's already-computed
    // STR modifier -- see this function's doc comment and
    // `PilotReceipt.weapon_damage`'s doc comment. Neither is recomputed.
    let weapon_damage = resolve_weapon_damage_breakdown(
        input,
        corpus,
        &equipment_effects,
        receipt.base.ability_modifiers.strength,
    );

    // v0.6 alpha swarm task 5: unlike `equipment_effects`/`weapon_damage`,
    // carrying capacity is not scoped to `EquippedActive` only -- a
    // `SelectedInactive` item (owned, in the character's pack, just not
    // worn/wielded) still weighs something, so `compute_encumbrance` reads
    // `input.chosen.equipment_selections` directly (unfiltered) rather than
    // the `equipped` local above. Needs the real *effective* Strength
    // score, not the STR modifier `weapon_damage` reuses above -- derived
    // via the same `apply_human_ability_bonus` the chassis computation
    // itself already applied; the explanation it would push is discarded
    // here (already pushed once, for real, by the chassis computation that
    // produced `receipt.base` -- pushing it again into a throwaway `Vec`
    // avoids a duplicate id in `PilotReceipt.diagnostics`/`chassis`).
    let mut discarded_explanations = Vec::new();
    let effective_ability_scores = apply_human_ability_bonus(input, &mut discarded_explanations);
    // PF1 scales carrying capacity by creature size. All 18 ingested races
    // resolve their real size; anything else reports itself rather than
    // quietly computing at Medium -- see `encumbrance_size_for_race`.
    let (size, size_diagnostic) = encumbrance_size_for_race(&input.chosen.race_id);
    let encumbrance = compute_encumbrance(
        &input.chosen.equipment_selections,
        corpus,
        effective_ability_scores.strength,
        size,
    );
    let mut diagnostics = receipt.base.diagnostics.clone();
    diagnostics.extend(size_diagnostic);

    PilotReceipt {
        diagnostics,
        chassis: receipt.base.clone(),
        corpus_derived: receipt.corpus_derived.clone(),
        skills: allocate_skill_ranks(input),
        spellbook: compute_spellbook_coverage(input, corpus),
        feats,
        equipment_effects,
        weapon_damage,
        encumbrance,
    }
}

/// Cycle 6's standalone Level-Up preview seam
/// (`contract:level_up_preview`, `adaptive-squishing-mccarthy.md`). A
/// thin pass-through to Epic 7's
/// `level_up::compute_level_up_grants(character, from_level, to_level)`
/// -- this function adds no logic of its own.
///
/// **Deliberately NOT part of `PilotReceipt`.** Per the plan's Q1
/// design-decision rationale: Level-Up models a level *transition* (it
/// needs `from_level`/`to_level`, two extra parameters no other
/// `PilotReceipt` consumer has), not current-state snapshot data like
/// every other field on `PilotReceipt`. Folding it into `PilotReceipt`
/// would force one of two bad outcomes -- fabricating `from_level`/
/// `to_level` values for every snapshot-only consumer that never asked
/// for a transition, or contaminating the whole contract with
/// transition-only params that only this one use case needs. So
/// `compute_level_up_preview` stays a standalone function that coexists
/// with `PilotReceipt` (and `to_pilot_receipt`) rather than being
/// embedded in it. This cycle adds no `PilotReceipt` field and no
/// `printed_sheet_cell_map` cell -- see
/// `tests/sd20_contract_level_up_preview.rs` for the parity proof
/// against calling `compute_level_up_grants` directly.
pub fn compute_level_up_preview(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan {
    compute_level_up_grants(character, from_level, to_level)
}

/// The diagnostic id emitted when a character's `race_id` cannot be resolved
/// to a real creature size, so its carrying capacity had to be computed at an
/// assumed Medium. `claim_blocking: true`: the resulting thresholds, load
/// tier, load max-Dex cap and load armor check penalty are all a guess, and a
/// consumer must not present them as computed numbers.
pub const UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID: &str = "encumbrance.race_size.unknown";

/// The single creature-size seam for carrying capacity, shared by both
/// encumbrance call sites (`to_pilot_receipt` here and
/// `pilot_compute_corpus::compute_pilot_with_corpus`).
///
/// # The defect this replaces
///
/// Both sites previously read
/// `rules_tables::crb::race_tables::race_size_for_race_id(...).unwrap_or(SizeCategory::Medium)`.
/// That function is a seven-variant `RaceId` lookup over the hardcoded CRB
/// races, so it returned `None` for all 11 ingested Bestiary 1 races and the
/// `unwrap_or` silently turned that into Medium. **Goblin, Kobold and
/// Svirfneblin are Small**, and every one of them was therefore handed 4/3 of
/// its real carrying capacity, a wrong load tier, and the wrong max-Dex cap and
/// armor check penalty that follow from the tier — the identical defect
/// `size.rs` was written to remove for Gnome and Halfling, still live for three
/// more races. `race_resolver::race_size_for_race_token` covers all 18.
///
/// # Why there is still a fallback, and why it is no longer silent
///
/// `compute_encumbrance` needs *a* size to compute with, and Medium is the
/// unmultiplied baseline `load.lst`'s own `LOAD:` column is expressed in, so it
/// is the only defensible assumption. What was wrong before was not the choice
/// of fallback but that taking it was invisible. It now comes back with a
/// claim-blocking [`UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID`] naming the unresolvable
/// token, so a receipt built on an assumed size says so.
pub fn encumbrance_size_for_race(race_id: &str) -> (SizeCategory, Option<ComputationDiagnostic>) {
    match race_size_for_race_token(race_id) {
        Some(size) => (size, None),
        None => (
            SizeCategory::Medium,
            Some(ComputationDiagnostic {
                id: UNKNOWN_RACE_SIZE_DIAGNOSTIC_ID.to_string(),
                message: format!(
                    "race {race_id:?} resolves to no ingested race, so its creature size is unknown; \
                     carrying capacity was computed at an assumed Medium and is not real data"
                ),
                claim_blocking: true,
            }),
        ),
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
/// cycle) are a third, dynamic case: `receipt.spellbook.spell_save_dc` is a
/// `BTreeMap` keyed by class id (`String`), not a fixed single field, so
/// this function emits one cell per *present* key
/// (`sheet.spellbook.spell_save_dc.<class_id>`) rather than a fixed set of
/// cell ids. A non-caster (or any character with an empty `spell_save_dc`
/// map) naturally produces zero cells of that kind — never a fabricated
/// placeholder cell for a key that is not present.
/// `compute_spellbook_coverage` pushes no diagnostics at all (`spellbook.rs`
/// has no `claim_blocking` machinery), so none of these cells is ever
/// `Blocked`; absence is expressed purely by the cell not existing in the
/// returned `Vec`. Per `adaptive-squishing-mccarthy.md`'s "Not every epic
/// output becomes a sheet cell" design decision, `spells_prepared`,
/// `spells_known`, and `school_specialization` are deliberately NOT
/// flattened into cells here — they don't reduce to
/// `PrintedSheetCellValue::Number(i16) | Blocked` cleanly and stay
/// reachable via `receipt.spellbook` directly. `SpellbookCoverage` never
/// carried `slots_total`/`slots_used` fields at all -- see `decisions.md`
/// Decision 37 (epic-31-spell-wiring gap closure).
///
/// The two `sheet.equipment.*` cells (as of the `contract:equipment_wiring`
/// cycle) follow a fourth, distinct discipline: `armor_class_delta` is a
/// plain `i16` (not `Option`), so `sheet.equipment.armor_class_delta` is
/// ALWAYS present -- `0` for "no armor bonus contributed" is a real,
/// honest value, not a fabricated placeholder. `max_dex_cap` is
/// `Option<i16>`, so `sheet.equipment.max_dex_cap` is present ONLY when
/// `Some` -- an unarmored/shieldless loadout has no cap at all, and this
/// cycle's explicit discipline is to omit the cell entirely in that case
/// rather than fabricate a cell for "no cap exists" (never `Blocked`
/// either -- `compute_equipment_effects` pushes no diagnostics of its
/// own). Per that same cycle's scope boundary,
/// `EquipmentEffects.spell_failure_chance: Option<f32>` is deliberately
/// EXCLUDED from cells entirely -- a fractional percentage doesn't reduce
/// to `PrintedSheetCellValue::Number(i16)` cleanly -- and stays reachable
/// only via `receipt.equipment_effects.spell_failure_chance` directly.
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

    // See this function's doc comment for why this family is dynamic (one
    // cell per present BTreeMap key) rather than a fixed set of cell ids,
    // and why an empty map naturally yields zero cells of that kind.
    for (class_id, &dc) in &receipt.spellbook.spell_save_dc {
        cells.push(independent_cell(
            &format!("sheet.spellbook.spell_save_dc.{class_id}"),
            &format!("spellbook.spell_save_dc.{class_id}"),
            dc as i16,
        ));
    }

    // See this function's doc comment for the armor_class_delta /
    // max_dex_cap cell discipline: armor_class_delta is a plain i16 so it
    // is always present; max_dex_cap is Option<i16> so its cell is
    // present only when Some (never fabricated for "no cap exists").
    cells.push(independent_cell(
        "sheet.equipment.armor_class_delta",
        "equipment_effects.armor_class_delta",
        receipt.equipment_effects.armor_class_delta,
    ));
    if let Some(max_dex_cap) = receipt.equipment_effects.max_dex_cap {
        cells.push(independent_cell(
            "sheet.equipment.max_dex_cap",
            "equipment_effects.max_dex_cap",
            max_dex_cap,
        ));
    }

    cells
}
