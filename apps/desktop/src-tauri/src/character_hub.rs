//! Character Hub — Tauri command adapter over the real rules-core compute
//! engine and saved-character persistence.
//!
//! Bridges the desktop shell and the headless `codex` crate: composes a
//! `CharacterInput` from the caller's race/class/level choice with a fixed
//! feat/skill/equipment loadout (none of the compute seam's class-specific
//! diagnostics read those selections, so widening them would not change
//! which combinations compute), computes it via the real engine, and
//! persists successful builds through `SavedCharacterStore`. A `Blocked`
//! result is never written to disk — this module never saves an unproven
//! build. Only single-class Fighter at any of levels 1-3 reaches `Computed`
//! today; every other class/level combination returns real claim-blocking
//! diagnostics from the engine, verbatim.

use std::path::{Path, PathBuf};

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use uuid::Uuid;

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput,
    ChosenCharacterState, EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::durability::{classify_durability, compute_max_hp, DurabilityStatus};
use codex::rules_core::feat_effects;
use codex::rules_core::money;
use codex::rules_core::pilot_compute::{
    ability_modifier, apply_human_ability_bonus, build_pilot_headless_receipt, HeadlessReceiptStatus,
};
use codex::rules_core::pilot_compute_corpus::{
    compute_pilot_with_corpus, CorpusDerivedSection, ResolvedEquipment,
};
use codex::rules_core::pilot_view_model::PilotSnapshot;

use crate::corpus_fixtures::corpus_fixture_bundle;
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::{
    SavedCharacterEnvelope, SavedCharacterRevisionKind, SavedCharacterSummary,
    CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
};

// `pub(crate)` (rather than private) so `pf1_adapter.rs`'s
// `compose_character_input` (moved there SD-25 Criterion 3.2) can read the
// same two constants this module's own `create_character`/`clone_character`/
// `seed_default_character_if_needed`/`import_character_from_json` still use.
pub(crate) const HUMAN_RACE_ID: &str = "race:human";
pub(crate) const SOURCE_PACKAGE_ID: &str = "pf1.core_rulebook";
const GAME_SYSTEM_ID: &str = "pf1";
const CHARACTERS_ROOT_DIR_NAME: &str = "characters";

// ----- DTOs (manual re-projection: the `codex` crate has zero dependencies,
// so its types are not `Serialize`/`Deserialize`) -----

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityScoresDto {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifiersDto {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSavesDto {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedSkillModifiersDto {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PilotSnapshotDto {
    pub ability_modifiers: AbilityModifiersDto,
    pub base_attack_bonus: i16,
    pub base_saves: BaseSavesDto,
    pub baseline_melee_attack_bonus: i16,
    pub baseline_armor_class: i16,
    pub total_saves: BaseSavesDto,
    pub selected_skill_modifiers: SelectedSkillModifiersDto,
    /// v0.6 alpha swarm (risks-and-open-questions.md item 6): the flat DR
    /// magnitude from a grounded class-feature DR explanation (currently
    /// only Barbarian's), or absent when no such record exists or its
    /// magnitude is the level-gate absence value of 0. See
    /// `PilotDefenseViewModel::damage_reduction`'s own doc comment
    /// (`pilot_view_model.rs`) for the full derivation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_reduction: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolCoverageDto {
    /// e.g. "Abjuration" — the `Pf1SchoolId` variant name, verbatim.
    pub school: String,
    /// Corpus spell identities resolved for this school, sorted.
    pub spells: Vec<String>,
    /// Whether the resolved spell(s) also ground through the foundation
    /// slice's bootstrap table cell (`TableCellRef`), not just the corpus.
    pub grounded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedEquipmentDto {
    /// The `CharacterInput.equipment_selections[].item_id` verbatim.
    pub item_id: String,
    pub equipment_record_name: String,
    pub equipment_record_key: String,
    /// Whether this item also grounds through the foundation slice's
    /// bootstrap table cell (`TableCellRef`), not just the corpus.
    pub grounded: bool,
    /// v0.6 alpha swarm items 1+27 sub-task 6: this selection's own
    /// resolved `applied_modifiers` (e.g. a resolved "+1 Enhancement to
    /// Weapon" attached to this Longsword) -- reuses this same DTO shape
    /// rather than a new type, since a resolved modifier is structurally
    /// just another resolved equipment record. Empty for a selection with
    /// no attached modifiers, or whose modifiers all failed to resolve
    /// (those surface via `CorpusDerivedDto.unresolvedEquipmentItemIds`
    /// instead, same list a top-level unresolvable selection already
    /// uses).
    pub applied_modifiers: Vec<ResolvedEquipmentDto>,
}

/// v0.6 alpha swarm item 1, shape (c) (`item-1-architecture-wall-design.md`):
/// the real, corpus-resolved aggregate equipment-effect totals for the
/// character's currently `EquippedActive` items. Explicitly NOT claim-gated
/// -- unlike `PilotSnapshotDto.baselineArmorClass` (the deterministic-posture
/// value the `Computed`/`Blocked` gate itself is built on), these numbers
/// reflect whatever real gear is actually equipped, corpus-resolved,
/// regardless of whether the build reaches `Computed`.
///
/// `attack_bonus_delta` (the bounded single-weapon slice) is `null` on the
/// wire whenever zero or two-or-more weapons are `EquippedActive` --
/// `CharacterInput`'s schema has no field recording which weapon a modifier
/// item attaches to, so with more than one weapon equipped, which one an
/// enhancement modifies is genuinely ambiguous (see
/// `EquipmentEffects.attack_bonus_delta`'s own doc comment,
/// `equipment_effects.rs`). With exactly one weapon equipped it is a real,
/// unambiguous value, including a real `0` when no enhancement applies --
/// the frontend must treat `null` as "not shown" (ambiguous), not as zero.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentEffectsDto {
    pub armor_class_delta: i16,
    pub armor_check_penalty_total: i16,
    /// v0.6 alpha swarm (QA finding, 2026-07-24): without `skip_serializing_if`,
    /// a Rust `None` here serialized as `"maxDexCap":null` -- key present,
    /// literal `null`, not an omitted key -- so the frontend's `!== undefined`
    /// hide-checks (`null !== undefined` is `true`) never fired, rendering
    /// garbled `"+null"`/`"null%"` strings instead of hiding the field. Mirrors
    /// the precedent `PilotSnapshotDto.damage_reduction` already set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dex_cap: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spell_failure_chance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_bonus_delta: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorpusDerivedDto {
    pub school_coverage: Vec<SchoolCoverageDto>,
    pub equipped_items: Vec<ResolvedEquipmentDto>,
    pub equipment_effects: EquipmentEffectsDto,
    /// v0.6 alpha swarm (QA finding, 2026-07-24): every `spellId`/`itemId`
    /// the caller selected that did NOT resolve against this build's
    /// corpus -- e.g. a real, disk-persisted selection outside the
    /// desktop app's deliberately tiny bundled demo corpus
    /// (`corpus_fixtures.rs`, ~4 records total). Before this field, such a
    /// selection simply vanished from `schoolCoverage`/`equippedItems`
    /// with no signal at all, indistinguishable from "nothing selected"
    /// even though the underlying data was never lost. The frontend
    /// should render these as an honest "not shown -- outside demo
    /// corpus" indicator, not silence.
    pub unresolved_spell_ids: Vec<String>,
    pub unresolved_equipment_item_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticDto {
    pub id: String,
    pub message: String,
    pub claim_blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCharacterRequest {
    pub character_id: String,
    pub display_label: String,
    pub race_id: String,
    pub class_id: String,
    pub level: u8,
    pub ability_scores: AbilityScoresDto,
    pub ability_bonus_target: String,
    pub saved_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSummaryDto {
    pub character_id: String,
    pub display_label: String,
    pub game_system: String,
    pub schema_version: u16,
    pub saved_at: String,
    pub race_id: String,
    pub class_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSavedCharactersResponse {
    pub characters: Vec<CharacterSummaryDto>,
    pub unreadable_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadSavedCharacterRequest {
    pub character_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadSavedCharacterResponse {
    pub summary: CharacterSummaryDto,
    pub snapshot: Option<PilotSnapshotDto>,
    pub diagnostics: Vec<DiagnosticDto>,
    pub corpus_derived: CorpusDerivedDto,
    /// The character's full persisted `chosen.selected_feats`, verbatim —
    /// not just feats added in the current session. Backlog item 8
    /// (`risks-and-open-questions.md`): the Feat picker previously had no
    /// way to render a loaded character's existing feat list.
    pub selected_feats: Vec<String>,
    /// The character's full persisted `chosen.spells_selected`, verbatim —
    /// not just spells added in the current session. Backlog item 9a
    /// (`risks-and-open-questions.md`): same shape of gap as item 8
    /// (`selected_feats`) — frontend had no way to detect a Wizard's
    /// current spell count/contents without this, which was the root of
    /// their "always route through record_and_prepare_spell_selection"
    /// workaround since they couldn't tell whether a spell add was truly
    /// "the first spell." Reuses `SpellSelectionImportDto` (already a
    /// general-purpose round-trip wire shape for export/import, not
    /// import-only despite the name) rather than inventing a near-duplicate
    /// type.
    pub spells_selected: Vec<SpellSelectionImportDto>,
}

/// The `kind` tag stays PascalCase (`Saved` / `Blocked`) — no container-level
/// `rename_all` — matching the `Ge08BaselineArmorClass` precedent so the TS
/// boundary can match on those exact strings. v0.6 alpha swarm (real
/// render-staleness root cause, frontend-found): a bare
/// `#[serde(rename_all = "camelCase")]` on this enum would ALSO camelCase
/// (lowercase-first) the `"Saved"`/`"Blocked"` tag values themselves,
/// breaking every `outcome.kind === 'Saved'` check across the frontend —
/// that's exactly why it was never added despite `corpus_derived` staying
/// snake_case on the wire ever since. The real fix is a per-field rename
/// on just the one field that actually has an underscore (every other
/// field here already happens to have none), not an enum-wide attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CreateCharacterResponse {
    Saved {
        // Boxed solely to close the `clippy::large_enum_variant` gap against the
        // `Blocked` variant (SD-21 Epic 4 closure, criterion E4.24) — `Box<T>`
        // serializes identically to `T` via serde, so the wire shape to the TS
        // boundary is unchanged.
        summary: Box<CharacterSummaryDto>,
        snapshot: PilotSnapshotDto,
        #[serde(rename = "corpusDerived")]
        corpus_derived: CorpusDerivedDto,
    },
    Blocked {
        diagnostics: Vec<DiagnosticDto>,
    },
}

// ----- Pure functions (unit-testable, no AppHandle / filesystem) -----
//
// `compose_character_input` / `apply_level_up` / `mutate_saved_character_at_root`
// / `level_up_character_at_root` / `apply_add_equipment_selection` /
// `add_equipment_selection_at_root` / `apply_add_spell_selection` /
// `add_spell_selection_at_root` moved to `pf1_adapter.rs` (SD-25 Epic 3
// "Hub of Hubs" refactor, Criterion 3.2 — the `Pf1Adapter` extraction).
// Re-exported below (where an external file's existing import path needs
// it) and brought into this module's own scope via `use` (for this
// module's `#[tauri::command]` wrappers and its own `mod tests`, which
// still call them unqualified via `use super::*`).
pub(crate) use crate::pf1_adapter::{
    add_equipment_selection_at_root, add_feat_selection_at_root, add_spell_selection_at_root,
    apply_attach_equipment_modifier, compose_character_input, level_up_character_at_root,
    mutate_saved_character_at_root, record_and_prepare_spell_selection_at_root,
    resolve_unified_pilot_snapshot, set_skill_allocations_at_root,
};
// `apply_level_up` / `apply_add_equipment_selection` / `apply_add_spell_selection`
// / `apply_add_feat_selection` / `apply_set_skill_allocations` are only
// referenced directly by this module's own `#[cfg(test)] mod tests` (the
// non-test `#[tauri::command]` wrappers only ever call the `_at_root`
// variants re-exported above) — `#[cfg(test)]` on the import itself avoids
// an `unused_imports` warning on non-test builds while keeping
// `use super::*` resolving them inside `mod tests` unchanged.
#[cfg(test)]
pub(crate) use crate::pf1_adapter::{
    apply_add_equipment_selection, apply_add_feat_selection, apply_add_spell_selection,
    apply_level_up, apply_record_and_prepare_spell_selection, apply_set_skill_allocations,
};

/// Join the OS app-data directory with the characters-root subdirectory.
/// Split out as a pure function so it's unit-testable without a real
/// `AppHandle`.
fn characters_root_from_app_data_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(CHARACTERS_ROOT_DIR_NAME)
}

fn map_ability_modifiers_dto(
    modifiers: codex::rules_core::pilot_compute::AbilityModifiers,
) -> AbilityModifiersDto {
    AbilityModifiersDto {
        strength: modifiers.strength,
        dexterity: modifiers.dexterity,
        constitution: modifiers.constitution,
        intelligence: modifiers.intelligence,
        wisdom: modifiers.wisdom,
        charisma: modifiers.charisma,
    }
}

fn map_base_saves_dto(saves: codex::rules_core::pilot_compute::BaseSaves) -> BaseSavesDto {
    BaseSavesDto {
        fortitude: saves.fortitude,
        reflex: saves.reflex,
        will: saves.will,
    }
}

fn map_selected_skill_modifiers_dto(
    modifiers: codex::rules_core::pilot_compute::SelectedSkillModifiers,
) -> SelectedSkillModifiersDto {
    SelectedSkillModifiersDto {
        climb: modifiers.climb,
        intimidate: modifiers.intimidate,
        swim: modifiers.swim,
    }
}

// `pub(crate)` (rather than private): `pf1_adapter.rs`'s
// `mutate_saved_character_at_root` (moved there SD-25 Criterion 3.2) builds
// the same `Saved` response shape this module's own commands do, and reuses
// this mapping rather than re-deriving it.
pub(crate) fn map_snapshot_dto(snapshot: &PilotSnapshot) -> PilotSnapshotDto {
    PilotSnapshotDto {
        ability_modifiers: map_ability_modifiers_dto(snapshot.ability_modifiers),
        base_attack_bonus: snapshot.base_attack_bonus,
        base_saves: map_base_saves_dto(snapshot.base_saves),
        baseline_melee_attack_bonus: snapshot.combat.baseline_melee_attack_bonus,
        baseline_armor_class: snapshot.defense.baseline_armor_class,
        total_saves: map_base_saves_dto(snapshot.defense.total_save),
        selected_skill_modifiers: map_selected_skill_modifiers_dto(
            snapshot.skill.selected_modifier,
        ),
        damage_reduction: snapshot.defense.damage_reduction,
    }
}

/// Maps one `ResolvedEquipment` to its DTO, recursing into
/// `applied_modifiers` (v0.6 alpha swarm sub-task 6) -- a resolved
/// modifier is structurally identical to a resolved top-level selection,
/// so this one function handles both without a near-duplicate.
/// `pub(crate)` — same reason as `map_spells_selected_dto`:
/// `rule_system_adapter.rs`'s `TestPf1Delegate` test double reuses this
/// rather than hand-rolling its own mirror a second time.
pub(crate) fn map_resolved_equipment_dto(item: &ResolvedEquipment) -> ResolvedEquipmentDto {
    ResolvedEquipmentDto {
        item_id: item.item_id.clone(),
        equipment_record_name: item.equipment_record_name.clone(),
        equipment_record_key: item.equipment_record_key.clone(),
        grounded: item.table_cell.is_some(),
        applied_modifiers: item.applied_modifiers.iter().map(map_resolved_equipment_dto).collect(),
    }
}

// `pub(crate)` — same reason as `map_snapshot_dto` above.
pub(crate) fn map_corpus_derived_dto(section: &CorpusDerivedSection) -> CorpusDerivedDto {
    CorpusDerivedDto {
        school_coverage: section
            .school_coverage
            .values()
            .map(|coverage| SchoolCoverageDto {
                school: format!("{:?}", coverage.school),
                spells: coverage.spells.clone(),
                grounded: coverage.table_cell.is_some(),
            })
            .collect(),
        equipped_items: section
            .equipped_items
            .iter()
            .map(map_resolved_equipment_dto)
            .collect(),
        equipment_effects: EquipmentEffectsDto {
            armor_class_delta: section.equipment_effects.armor_class_delta,
            armor_check_penalty_total: section.equipment_effects.armor_check_penalty_total,
            max_dex_cap: section.equipment_effects.max_dex_cap,
            spell_failure_chance: section.equipment_effects.spell_failure_chance,
            attack_bonus_delta: section.equipment_effects.attack_bonus_delta,
        },
        unresolved_spell_ids: section.unresolved_spell_ids.clone(),
        unresolved_equipment_item_ids: section.unresolved_equipment_item_ids.clone(),
    }
}

// `pub(crate)` — same reason as `map_snapshot_dto` above.
pub(crate) fn map_diagnostics_dto(
    diagnostics: &[codex::rules_core::pilot_compute::ComputationDiagnostic],
) -> Vec<DiagnosticDto> {
    diagnostics
        .iter()
        .map(|diagnostic| DiagnosticDto {
            id: diagnostic.id.clone(),
            message: diagnostic.message.clone(),
            claim_blocking: diagnostic.claim_blocking,
        })
        .collect()
}

// `pub(crate)` — same reason as `map_snapshot_dto` above. Reuses
// `SpellSelectionImportDto` (already a general-purpose export/import
// round-trip wire shape, not import-only despite the name) rather than
// inventing a near-duplicate type for `LoadSavedCharacterResponse`.
pub(crate) fn map_spells_selected_dto(spells: &[SpellSelection]) -> Vec<SpellSelectionImportDto> {
    spells
        .iter()
        .map(|spell| SpellSelectionImportDto {
            spell_id: spell.spell_id.clone(),
            source_class_id: spell.source_class_id.clone(),
            acquisition_mode: spell.acquisition_mode.into(),
        })
        .collect()
}

// `pub(crate)` — `pf1_adapter.rs`'s `impl RuleSystemAdapter for Pf1Adapter`
// (criterion 3.1's trait, landed after this cycle's own dispatch began;
// wired here rather than left as a `## DISCOVERED` follow-up) reuses this
// mapping for `list_saved_characters` rather than re-deriving it.
pub(crate) fn map_summary_dto(summary: &SavedCharacterSummary) -> CharacterSummaryDto {
    CharacterSummaryDto {
        character_id: summary.character_id.clone(),
        display_label: summary.display_label.clone(),
        game_system: summary.game_system.clone(),
        schema_version: summary.schema_version,
        saved_at: summary.saved_at.clone(),
        race_id: summary.race_id.clone(),
        class_summary: summary.class_summary.clone(),
    }
}

// `pub(crate)` — same reason as `map_snapshot_dto` above.
pub(crate) fn summarize_envelope(envelope: &SavedCharacterEnvelope) -> CharacterSummaryDto {
    let class_summary = envelope
        .character_input
        .chosen
        .class_levels
        .iter()
        .map(|class_level| format!("{}:{}", class_level.class_id, class_level.level))
        .collect::<Vec<_>>()
        .join(",");

    CharacterSummaryDto {
        character_id: envelope.character_id.clone(),
        display_label: envelope.display_label.clone(),
        game_system: envelope.game_system.clone(),
        schema_version: envelope.schema_version,
        saved_at: envelope.saved_at.clone(),
        race_id: envelope.character_input.chosen.race_id.clone(),
        class_summary,
    }
}

// ----- `mutate_saved_character` operation table -----

/// The character-mutation surface this module documents. Every operation
/// shares the same `load -> mutate -> recompute -> re-save -> return
/// envelope` semantics: load the saved envelope via
/// `SavedCharacterStore::load`, apply one bounded mutation to its
/// `CharacterInput`, recompute via `compute_pilot_with_corpus`, re-save via
/// `SavedCharacterStore::save`, and return the updated envelope (as a
/// `CreateCharacterResponse::Saved`/`Blocked`, matching every other
/// character-hub command's response shape).
///
/// This table documents the full six-operation surface. As of this cycle
/// all six rows are wired to callable `#[tauri::command]`s
/// (`level_up_character`, `add_equipment_selection`, `add_spell_selection`,
/// `set_skill_allocations`, `add_feat_selection`,
/// `record_and_prepare_spell_selection`).
/// Per the Wired Integration doctrine (`docs/governance/no-stub-mvp-doctrine.md`),
/// the `wired` flag below is descriptive metadata this table's own
/// dispatch-shape test asserts against, not a runtime dispatcher a caller
/// can reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedCharacterMutationOp {
    LevelUpCharacter,
    AddEquipmentSelection,
    AddSpellSelection,
    SetSkillAllocations,
    AddFeatSelection,
    RecordAndPrepareSpellSelection,
}

/// One row of the `mutate_saved_character` operation table.
#[derive(Debug, Clone, Copy)]
pub struct SavedCharacterMutationOpDescriptor {
    pub op: SavedCharacterMutationOp,
    /// The operation name, matching its `#[tauri::command]` function name
    /// once wired.
    pub name: &'static str,
    /// The mutation this operation applies to the loaded `CharacterInput`,
    /// on top of the load -> mutate -> recompute -> re-save ->
    /// return-envelope semantics every row shares.
    pub description: &'static str,
    /// Whether this operation is reachable through a real, callable Tauri
    /// command in this build (registered in `main.rs`'s
    /// `invoke_handler!`). `false` means the row is documented here but
    /// intentionally not yet registered.
    pub wired: bool,
}

pub const SAVED_CHARACTER_MUTATION_OPERATIONS: [SavedCharacterMutationOpDescriptor; 6] = [
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::LevelUpCharacter,
        name: "level_up_character",
        description: "Increments the requested class's level (or adds it \
            at level 1 if the character has none yet), then recomputes and \
            re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::AddEquipmentSelection,
        name: "add_equipment_selection",
        description: "Appends an entry to chosen.equipment_selections, then \
            recomputes and re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::AddSpellSelection,
        name: "add_spell_selection",
        description: "Appends an entry to chosen.spells_selected, then \
            recomputes and re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::SetSkillAllocations,
        name: "set_skill_allocations",
        description: "Replaces chosen.skill_allocations wholesale with the \
            caller's full allocation set, then recomputes and re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::AddFeatSelection,
        name: "add_feat_selection",
        description: "Appends an entry to chosen.selected_feats, then \
            recomputes and re-saves.",
        wired: true,
    },
    SavedCharacterMutationOpDescriptor {
        op: SavedCharacterMutationOp::RecordAndPrepareSpellSelection,
        name: "record_and_prepare_spell_selection",
        description: "Appends BOTH a Known and a Prepared entry for the \
            same spell to chosen.spells_selected in one atomic mutation, \
            then recomputes and re-saves -- breaks the Wizard spellbook \
            bootstrap deadlock a single-mode add_spell_selection call \
            cannot cross alone.",
        wired: true,
    },
];

// ----- Tauri commands -----

/// `create_character`'s real implementation, split from the `#[tauri::command]`
/// wrapper below so it is unit-testable against a real `SavedCharacterStore`
/// fixture without an `AppHandle` -- mirrors every other command's own
/// `_at_root` split (`level_up_character_at_root`, `purchase_equipment_at_root`,
/// `recompute_character_at_root`, ...). `app_version` is passed explicitly
/// (rather than an `AppHandle`) since it is the only piece of this function's
/// original body that ever needed one.
///
/// v0.6 alpha swarm item 7 (risks-and-open-questions.md): once the build
/// reaches `Computed` and saves, this also initializes the character's
/// starting money balance via `money::starting_wealth_gp`, for any class
/// that function recognizes -- today that means every character that gets
/// this far at all, since `starting_wealth_gp` covers all 11 CRB classes
/// and only Fighter/Wizard/Rogue currently reach `Computed` in the first
/// place (a class outside that set never reaches this line, having already
/// returned `Blocked` above). An unrecognized class id (`None`) leaves
/// `money.json` uninitialized -- the existing "no file yet" convention
/// already means a 0 balance, so this never fabricates a value for a class
/// this table doesn't cover.
pub(crate) fn create_character_at_root(
    root: &Path,
    request: &CreateCharacterRequest,
    app_version: String,
) -> Result<CreateCharacterResponse, String> {
    let character_input = compose_character_input(request);

    let (snapshot, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&character_input, corpus_fixture_bundle()) {
            Ok(result) => result,
            Err(diagnostics) => {
                return Ok(CreateCharacterResponse::Blocked {
                    diagnostics: map_diagnostics_dto(&diagnostics),
                });
            }
        };

    let envelope = SavedCharacterEnvelope {
        character_id: request.character_id.clone(),
        revision_id: format!("{}.rev.1", request.character_id),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: request.saved_at.clone(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app_version,
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{}.rev.1", request.character_id),
        display_label: request.display_label.clone(),
        character_input,
    };

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    if let Some(starting_wealth_gp) = money::starting_wealth_gp(&request.class_id) {
        let starting_copper = money::gp_to_copper(f64::from(starting_wealth_gp));
        adjust_character_money_at_root(root, starting_copper as i64)?;
    }

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(&snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

#[tauri::command]
pub fn create_character(
    app: tauri::AppHandle,
    request: CreateCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    create_character_at_root(&root, &request, app.package_info().version.to_string())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloneCharacterRequest {
    pub character_id: String,
    pub new_character_id: String,
    pub new_display_label: String,
    pub saved_at: String,
}

/// Duplicates a saved character's full `CharacterInput` (race, classes,
/// ability scores, feats, skills, equipment, spells) under a new id and
/// display label, recomputing and saving it exactly like `create_character`.
/// Never persists an unproven build — a source that no longer computes
/// (e.g. after an engine change) returns `Blocked` rather than saving a copy.
#[tauri::command]
pub fn clone_character(
    app: tauri::AppHandle,
    request: CloneCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let source_root = resolve_character_root(&app, &request.character_id)?;
    let source_envelope = SavedCharacterStore::load(&source_root).map_err(|err| err.message)?;

    let mut character_input = source_envelope.character_input.clone();
    character_input.case_id = Some(request.new_character_id.clone());

    let (snapshot, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&character_input, corpus_fixture_bundle()) {
            Ok(result) => result,
            Err(diagnostics) => {
                return Ok(CreateCharacterResponse::Blocked {
                    diagnostics: map_diagnostics_dto(&diagnostics),
                });
            }
        };

    let envelope = SavedCharacterEnvelope {
        character_id: request.new_character_id.clone(),
        revision_id: format!("{}.rev.1", request.new_character_id),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: request.saved_at.clone(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app.package_info().version.to_string(),
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{}.rev.1", request.new_character_id),
        display_label: request.new_display_label.clone(),
        character_input,
    };

    let new_root = resolve_character_root(&app, &request.new_character_id)?;
    SavedCharacterStore::save(&envelope, &new_root).map_err(|err| err.message)?;

    // Best-effort: carry the portrait over too, if the source has one.
    let source_portrait = source_root.join(PORTRAIT_FILE_NAME);
    if source_portrait.exists() {
        let _ = std::fs::copy(&source_portrait, new_root.join(PORTRAIT_FILE_NAME));
    }

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(&snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

const DEFAULT_CHARACTER_SEED_MARKER: &str = ".default_character_seeded";
const DEFAULT_CHARACTER_ID: &str = "00000000-0000-0000-0000-000000000001";
const DEFAULT_CHARACTER_SAVED_AT: &str = "2026-01-01T00:00:00.000Z";

/// Seeds a starter character ("Aldric Ironhand": Human Fighter 3) into a
/// fresh install so there's something to open immediately instead of an
/// empty character list.
///
/// Aldric is a single-class Fighter, not the Fighter 3 / Wizard 1 multiclass
/// build shown in the browser-preview sample data (`previewData.ts`) — the
/// real compute engine only reaches `Computed` for a single-class Fighter
/// today (`compute_fighter_chassis` in `src/rules_core/pilot_compute.rs`
/// gates base attack bonus / base saves on that alone; verified directly,
/// not assumed — a single-class Wizard build was tried and still comes back
/// `Blocked`). Ability scores are chosen to reproduce the same ability
/// modifiers as the preview's Aldric (+3/+1/+2/+2/+1/-1).
///
/// Gated on a marker file, not on whether the characters directory is
/// currently empty — so deleting the starter character does not bring it
/// back on next launch. Reuses `compose_character_input`/`create_character`'s
/// own invariant: only saves if the build actually computes, never writes an
/// unproven build.
pub fn seed_default_character_if_needed(app: &tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))?;
    let marker_path = app_data_dir.join(DEFAULT_CHARACTER_SEED_MARKER);
    if marker_path.exists() {
        return Ok(());
    }

    let request = CreateCharacterRequest {
        character_id: DEFAULT_CHARACTER_ID.to_owned(),
        display_label: "Aldric Ironhand".to_owned(),
        race_id: HUMAN_RACE_ID.to_owned(),
        class_id: "class:fighter".to_owned(),
        level: 3,
        ability_scores: AbilityScoresDto {
            strength: 17,
            dexterity: 13,
            constitution: 14,
            intelligence: 14,
            wisdom: 12,
            charisma: 8,
        },
        ability_bonus_target: "strength".to_owned(),
        saved_at: DEFAULT_CHARACTER_SAVED_AT.to_owned(),
    };

    let character_input = compose_character_input(&request);
    let receipt = build_pilot_headless_receipt(&character_input);
    if receipt.status != HeadlessReceiptStatus::Computed {
        return Err("default starter character build did not compute; not seeding".to_owned());
    }

    let envelope = SavedCharacterEnvelope {
        character_id: request.character_id.clone(),
        revision_id: format!("{}.rev.1", request.character_id),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: request.saved_at.clone(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app.package_info().version.to_string(),
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{}.rev.1", request.character_id),
        display_label: request.display_label.clone(),
        character_input,
    };

    let root = characters_root_from_app_data_dir(&app_data_dir).join(&request.character_id);
    SavedCharacterStore::save(&envelope, &root).map_err(|err| err.message)?;

    std::fs::create_dir_all(&app_data_dir).map_err(|err| format!("{}: {err}", app_data_dir.display()))?;
    std::fs::write(&marker_path, "seeded\n").map_err(|err| format!("{}: {err}", marker_path.display()))?;

    Ok(())
}

#[tauri::command]
pub fn list_saved_characters(app: tauri::AppHandle) -> Result<ListSavedCharactersResponse, String> {
    let characters_root = resolve_characters_root(&app)?;
    let listing = SavedCharacterStore::list_all(&characters_root).map_err(|err| err.message)?;

    Ok(ListSavedCharactersResponse {
        characters: listing.characters.iter().map(map_summary_dto).collect(),
        unreadable_count: listing.unreadable_entries.len(),
    })
}

#[tauri::command]
pub fn load_saved_character(
    app: tauri::AppHandle,
    request: LoadSavedCharacterRequest,
) -> Result<LoadSavedCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let envelope = SavedCharacterStore::load(&root).map_err(|err| err.message)?;

    let (snapshot, diagnostics, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&envelope.character_input, corpus_fixture_bundle()) {
            Ok((snapshot, corpus_receipt)) => (Some(snapshot), Vec::new(), corpus_receipt),
            Err(diagnostics) => (
                None,
                diagnostics,
                compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle()),
            ),
        };

    Ok(LoadSavedCharacterResponse {
        summary: summarize_envelope(&envelope),
        snapshot: snapshot.as_ref().map(map_snapshot_dto),
        diagnostics: map_diagnostics_dto(&diagnostics),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
        selected_feats: envelope.character_input.chosen.selected_feats.clone(),
        spells_selected: map_spells_selected_dto(&envelope.character_input.chosen.spells_selected),
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpCharacterRequest {
    pub character_id: String,
    pub class_id: String,
    /// Additional player choices this level-up records (a hit-die roll or
    /// "take average" record, feat picks at feat-gaining levels, ...),
    /// appended to `chosen.selected_choices` verbatim. Empty by default —
    /// callers that only want the bare level increment (the pre-v0.6
    /// behavior) omit this field entirely.
    #[serde(default)]
    pub additional_choices: Vec<SelectedChoiceDto>,
    /// When present, replaces `chosen.skill_allocations` wholesale with
    /// this level-up's skill-point spend (same semantics as
    /// `set_skill_allocations`). `None`/omitted leaves the character's
    /// existing skill allocations untouched.
    #[serde(default)]
    pub skill_allocations: Option<Vec<SkillAllocationDto>>,
    pub saved_at: String,
}

/// Loads the saved character, increments/adds the requested class's level,
/// records any additional level-up choices (hit-die roll, feat picks) and
/// an optional skill-allocation update, recomputes via the real engine, and
/// re-saves — see `level_up_character_at_root` for the full semantics.
#[tauri::command]
pub fn level_up_character(
    app: tauri::AppHandle,
    request: LevelUpCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let additional_choices = request
        .additional_choices
        .into_iter()
        .map(|choice| SelectedChoice {
            choice_set_id: choice.choice_set_id,
            selection_id: choice.selection_id,
        })
        .collect();
    let skill_allocations = request.skill_allocations.map(|allocations| {
        allocations
            .into_iter()
            .map(|skill| SkillAllocation {
                skill_id: skill.skill_id,
                ranks: skill.ranks,
            })
            .collect()
    });
    level_up_character_at_root(
        &root,
        &request.class_id,
        additional_choices,
        skill_allocations,
        &request.saved_at,
    )
}

/// The wire-level projection of `ActiveState` for the `add_equipment_selection`
/// request. A separate DTO (rather than deriving `Deserialize` on
/// `ActiveState` itself) because the `codex` crate has zero dependencies —
/// see this module's own top-of-file note.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActiveStateDto {
    EquippedActive,
    Absent,
    SelectedInactive,
}

impl From<ActiveStateDto> for ActiveState {
    fn from(dto: ActiveStateDto) -> Self {
        match dto {
            ActiveStateDto::EquippedActive => ActiveState::EquippedActive,
            ActiveStateDto::Absent => ActiveState::Absent,
            ActiveStateDto::SelectedInactive => ActiveState::SelectedInactive,
        }
    }
}

/// Reverse of the above — needed by `export_character`'s
/// `CharacterInput -> CharacterInputDto` projection (Criterion 24's
/// round-trip fix).
impl From<ActiveState> for ActiveStateDto {
    fn from(state: ActiveState) -> Self {
        match state {
            ActiveState::EquippedActive => ActiveStateDto::EquippedActive,
            ActiveState::Absent => ActiveStateDto::Absent,
            ActiveState::SelectedInactive => ActiveStateDto::SelectedInactive,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEquipmentSelectionRequest {
    pub character_id: String,
    pub item_id: String,
    pub active_state: ActiveStateDto,
    pub saved_at: String,
}

/// Loads the saved character, appends the requested equipment selection,
/// recomputes via the real engine, and re-saves — see
/// `add_equipment_selection_at_root` for the full semantics.
#[tauri::command]
pub fn add_equipment_selection(
    app: tauri::AppHandle,
    request: AddEquipmentSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    add_equipment_selection_at_root(
        &root,
        &request.item_id,
        request.active_state.into(),
        &request.saved_at,
    )
}

/// v0.6 alpha swarm (risks-and-open-questions.md item 9): the outcome of an
/// atomic equipment purchase. A THIRD case beyond `CreateCharacterResponse`'s
/// `Saved`/`Blocked` doesn't exist here on purpose -- `Blocked` already
/// carries `diagnostics: Vec<DiagnosticDto>`, so an unaffordable purchase or
/// an item with no known cost is represented as a `Blocked` response with
/// one hand-authored diagnostic, the same wire shape frontend already
/// handles for every other mutation command, rather than a new response
/// shape to integrate.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind")]
pub enum PurchaseEquipmentResponse {
    Purchased {
        summary: Box<CharacterSummaryDto>,
        snapshot: PilotSnapshotDto,
        // Same reasoning as `CreateCharacterResponse::Saved`'s own
        // `corpus_derived` field: a per-field rename, not an enum-wide
        // `rename_all`, which would also lowercase the `"Purchased"`/
        // `"Blocked"` tag values themselves.
        #[serde(rename = "corpusDerived")]
        corpus_derived: CorpusDerivedDto,
        money: CharacterMoneyDto,
    },
    Blocked {
        diagnostics: Vec<DiagnosticDto>,
    },
}

/// `purchase_equipment`'s real implementation: real correctness fix for
/// risks-and-open-questions.md item 9 ("Money panel not coupled to
/// equipment purchases... deliberately not built as two non-atomic
/// mutations... risking a partial-apply correctness bug").
///
/// Sequencing (the actual transaction shape, chosen deliberately): resolve
/// the item's real `cost_gp` (headless, no corpus needed --
/// `equipment_cost_gp_headless_resolve`'s own doc comment explains why) and
/// pre-check affordability against the CURRENT balance BEFORE mutating
/// anything. Only once both checks pass does this call
/// `add_equipment_selection_at_root` (the existing equipment mutation,
/// unchanged); only if THAT reaches `Computed` and saves does this deduct
/// the cost via `adjust_character_money_at_root`. This ordering means the
/// only case where equipment is added without a successful matching charge
/// is a true I/O failure on the money-file write immediately after an
/// already-verified-affordable, already-persisted equipment save -- an
/// honestly narrow residual window (same disk, same moment, would likely
/// also have broken the equipment save itself), not the two-independent-
/// frontend-calls-with-no-pre-check gap this fix actually closes. A full
/// two-phase-commit / journaled rollback across the two separate files
/// (`character_input.txt`, `money.json`) would be real engineering but is
/// not proportionate to this codebase's current maturity level or this
/// swarm's bar -- noted here rather than silently assumed away.
///
/// An item with no known `cost_gp` (a `(Base)` template record or a
/// formula-priced equipment modifier) is treated the same as
/// insufficient funds: `Blocked`, nothing mutated, never a free item.
pub(crate) fn purchase_equipment_at_root(
    root: &Path,
    item_id: &str,
    active_state: ActiveState,
    saved_at: &str,
) -> Result<PurchaseEquipmentResponse, String> {
    let Some(cost_gp) =
        codex::rules_core::equipment_resolver::equipment_cost_gp_headless_resolve(item_id)
    else {
        return Ok(PurchaseEquipmentResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "money.equipment_purchase.unknown_cost".to_owned(),
                message: format!(
                    "'{item_id}' has no known gold-piece cost in the equipment catalog (a \
                     template/base record with no independent price, or a formula-priced \
                     equipment modifier), so affordability cannot be verified. The purchase \
                     was not applied and no funds were charged."
                ),
                claim_blocking: true,
            }],
        });
    };

    let cost_copper = money::gp_to_copper(cost_gp);
    let balance_copper = load_character_money_at_root(root)?.total_copper;
    if balance_copper < cost_copper {
        return Ok(PurchaseEquipmentResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "money.equipment_purchase.insufficient_funds".to_owned(),
                message: format!(
                    "'{item_id}' costs {cost_copper} cp but the character's balance is only \
                     {balance_copper} cp. The purchase was not applied and no funds were \
                     charged."
                ),
                claim_blocking: true,
            }],
        });
    }

    match add_equipment_selection_at_root(root, item_id, active_state, saved_at)? {
        CreateCharacterResponse::Blocked { diagnostics } => {
            Ok(PurchaseEquipmentResponse::Blocked { diagnostics })
        }
        CreateCharacterResponse::Saved { summary, snapshot, corpus_derived } => {
            let cost_signed = i64::try_from(cost_copper)
                .map_err(|_| "purchase cost overflows a signed 64-bit total".to_owned())?;
            let money = adjust_character_money_at_root(root, -cost_signed)?;
            Ok(PurchaseEquipmentResponse::Purchased { summary, snapshot, corpus_derived, money })
        }
    }
}

/// Same wire shape as `PurchaseEquipmentResponse` (`Attached`/`Blocked`,
/// same field set) -- deliberately not reused verbatim so the two
/// commands' response `kind` tags stay distinct on the wire
/// (`"Attached"` vs `"Purchased"`), matching frontend's own
/// per-command-outcome convention.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind")]
pub enum AttachEquipmentModifierResponse {
    Attached {
        summary: Box<CharacterSummaryDto>,
        snapshot: PilotSnapshotDto,
        #[serde(rename = "corpusDerived")]
        corpus_derived: CorpusDerivedDto,
        money: CharacterMoneyDto,
    },
    Blocked {
        diagnostics: Vec<DiagnosticDto>,
    },
}

/// `attach_equipment_modifier`'s real implementation (v0.6 alpha swarm
/// items 1+27 sub-task 6, frontend-proposed shape). Mirrors
/// `purchase_equipment_at_root`'s atomic resolve-cost -> check-affordability
/// -> mutate -> charge sequencing, with two deliberate differences:
///
/// 1. **Validates `modifier_item_id` against the real equipment catalog
///    first** (`equipment_tables()`, the same check
///    `append_to_character_at_root` already runs), before any cost or
///    target-selection check -- a typo'd or fabricated modifier id must
///    never silently attach.
/// 2. **An unknown `cost_gp` is treated as free to attach, not blocked** --
///    a deliberate deviation from `purchase_equipment_at_root`'s own
///    "unknown cost = blocked, same as unaffordable" rule. Checked against
///    the real static table before choosing this: the actual magical
///    weapon/armor enhancement records (`"Special Ability ~ +1 ~
///    Weapon"` through `~ +10 ~`) all resolve `cost_gp: None` (real PF1
///    enhancement pricing is a bonus-squared formula, not a flat catalog
///    price) -- mirroring `purchase_equipment`'s block-on-unknown-cost
///    behavior here would block exactly the headline use case this
///    command exists for. Only a modifier with a real, known `cost_gp`
///    (e.g. Masterwork, `Some(0.0)` in the current table -- itself a
///    known pre-existing pricing gap, not introduced here) is ever
///    actually charged.
///
/// The target `item_id` must already exist in `equipment_selections` --
/// checked via a read before any charge, so a not-found target is a
/// `Blocked` response with zero side effects, never a charge with nothing
/// to attach to.
pub(crate) fn attach_equipment_modifier_at_root(
    root: &Path,
    item_id: &str,
    modifier_item_id: &str,
    saved_at: &str,
) -> Result<AttachEquipmentModifierResponse, String> {
    let is_known_modifier = codex::rules_core::rules_tables::crb::equipment_tables::equipment_tables()
        .iter()
        .any(|entry| entry.key == modifier_item_id);
    if !is_known_modifier {
        return Ok(AttachEquipmentModifierResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "equipment.attach_modifier.unknown_item".to_owned(),
                message: format!(
                    "'{modifier_item_id}' is not a recognized equipment catalog item. Nothing \
                     was attached and no funds were charged."
                ),
                claim_blocking: true,
            }],
        });
    }

    let envelope = codex::saved_character::local_store::SavedCharacterStore::load(root)
        .map_err(|err| err.message)?;
    let target_exists = envelope
        .character_input
        .chosen
        .equipment_selections
        .iter()
        .any(|selection| selection.item_id == item_id);
    if !target_exists {
        return Ok(AttachEquipmentModifierResponse::Blocked {
            diagnostics: vec![DiagnosticDto {
                id: "equipment.attach_modifier.target_not_found".to_owned(),
                message: format!(
                    "'{item_id}' is not an equipped selection on this character. Nothing was \
                     attached and no funds were charged."
                ),
                claim_blocking: true,
            }],
        });
    }

    let cost_copper =
        match codex::rules_core::equipment_resolver::equipment_cost_gp_headless_resolve(modifier_item_id) {
            Some(cost_gp) => money::gp_to_copper(cost_gp),
            None => 0,
        };

    if cost_copper > 0 {
        let balance_copper = load_character_money_at_root(root)?.total_copper;
        if balance_copper < cost_copper {
            return Ok(AttachEquipmentModifierResponse::Blocked {
                diagnostics: vec![DiagnosticDto {
                    id: "money.equipment_attach_modifier.insufficient_funds".to_owned(),
                    message: format!(
                        "'{modifier_item_id}' costs {cost_copper} cp but the character's \
                         balance is only {balance_copper} cp. Nothing was attached and no \
                         funds were charged."
                    ),
                    claim_blocking: true,
                }],
            });
        }
    }

    match mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_attach_equipment_modifier(character_input, item_id, modifier_item_id);
    })? {
        CreateCharacterResponse::Blocked { diagnostics } => {
            Ok(AttachEquipmentModifierResponse::Blocked { diagnostics })
        }
        CreateCharacterResponse::Saved { summary, snapshot, corpus_derived } => {
            let money = if cost_copper > 0 {
                let cost_signed = i64::try_from(cost_copper)
                    .map_err(|_| "attach cost overflows a signed 64-bit total".to_owned())?;
                adjust_character_money_at_root(root, -cost_signed)?
            } else {
                load_character_money_at_root(root)?
            };
            Ok(AttachEquipmentModifierResponse::Attached { summary, snapshot, corpus_derived, money })
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachEquipmentModifierRequest {
    pub character_id: String,
    pub item_id: String,
    pub modifier_item_id: String,
    pub saved_at: String,
}

/// See `attach_equipment_modifier_at_root` for the full transaction-shape
/// reasoning (free-attach on unknown cost, target/modifier validation
/// before any charge).
#[tauri::command]
pub fn attach_equipment_modifier(
    app: tauri::AppHandle,
    request: AttachEquipmentModifierRequest,
) -> Result<AttachEquipmentModifierResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    attach_equipment_modifier_at_root(
        &root,
        &request.item_id,
        &request.modifier_item_id,
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseEquipmentRequest {
    pub character_id: String,
    pub item_id: String,
    pub active_state: ActiveStateDto,
    pub saved_at: String,
}

/// Atomically resolves `item_id`'s real catalog cost, verifies the
/// character can afford it, appends the equipment selection, and deducts
/// the cost from the persisted money balance — or applies none of it and
/// returns a `Blocked` diagnostic. See `purchase_equipment_at_root` for the
/// full transaction-shape reasoning.
#[tauri::command]
pub fn purchase_equipment(
    app: tauri::AppHandle,
    request: PurchaseEquipmentRequest,
) -> Result<PurchaseEquipmentResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    purchase_equipment_at_root(
        &root,
        &request.item_id,
        request.active_state.into(),
        &request.saved_at,
    )
}

/// The wire-level projection of `AcquisitionMode` for the
/// `add_spell_selection` request. A separate DTO for the same reason as
/// `ActiveStateDto` above.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AcquisitionModeDto {
    Known,
    Prepared,
    Granted,
}

impl From<AcquisitionModeDto> for AcquisitionMode {
    fn from(dto: AcquisitionModeDto) -> Self {
        match dto {
            AcquisitionModeDto::Known => AcquisitionMode::Known,
            AcquisitionModeDto::Prepared => AcquisitionMode::Prepared,
            AcquisitionModeDto::Granted => AcquisitionMode::Granted,
        }
    }
}

/// Reverse of the above — needed by `export_character`'s
/// `CharacterInput -> CharacterInputDto` projection (Criterion 24's
/// round-trip fix).
impl From<AcquisitionMode> for AcquisitionModeDto {
    fn from(mode: AcquisitionMode) -> Self {
        match mode {
            AcquisitionMode::Known => AcquisitionModeDto::Known,
            AcquisitionMode::Prepared => AcquisitionModeDto::Prepared,
            AcquisitionMode::Granted => AcquisitionModeDto::Granted,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSpellSelectionRequest {
    pub character_id: String,
    pub spell_id: String,
    pub source_class_id: String,
    pub acquisition_mode: AcquisitionModeDto,
    pub saved_at: String,
}

/// Loads the saved character, appends the requested spell selection,
/// recomputes via the real engine, and re-saves — see
/// `add_spell_selection_at_root` for the full semantics.
#[tauri::command]
pub fn add_spell_selection(
    app: tauri::AppHandle,
    request: AddSpellSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    add_spell_selection_at_root(
        &root,
        &request.spell_id,
        &request.source_class_id,
        request.acquisition_mode.into(),
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordAndPrepareSpellSelectionRequest {
    pub character_id: String,
    pub spell_id: String,
    pub source_class_id: String,
    pub saved_at: String,
}

/// Appends BOTH a `Known` and a `Prepared` entry for the same spell in one
/// atomic mutation — breaks the Wizard spellbook bootstrap deadlock
/// `add_spell_selection` alone cannot: `unmet_wizard_spellbook_conditions`
/// requires a non-empty recorded set AND a non-empty prepared set
/// simultaneously, but `add_spell_selection` only ever appends one spell in
/// one mode per call, and `mutate_saved_character_at_root` discards any
/// call that doesn't independently reach `Computed` — so a `Known`-only
/// call is Blocked (nothing prepared yet) and never persists, and a
/// `Prepared`-only call is *also* Blocked (the prepared spell isn't in the
/// still-empty recorded set) and never persists either. See
/// `apply_record_and_prepare_spell_selection`'s own doc comment for the
/// full analysis.
///
/// Use this once, for the character's first spell. After that, the plain
/// `add_spell_selection` (either mode) works normally for every subsequent
/// spell — this command is not a general replacement for it.
#[tauri::command]
pub fn record_and_prepare_spell_selection(
    app: tauri::AppHandle,
    request: RecordAndPrepareSpellSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    record_and_prepare_spell_selection_at_root(
        &root,
        &request.spell_id,
        &request.source_class_id,
        &request.saved_at,
    )
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFeatSelectionRequest {
    pub character_id: String,
    pub feat_id: String,
    pub saved_at: String,
}

/// Loads the saved character, appends the requested feat selection,
/// recomputes via the real engine, and re-saves — see
/// `add_feat_selection_at_root` for the full semantics.
#[tauri::command]
pub fn add_feat_selection(
    app: tauri::AppHandle,
    request: AddFeatSelectionRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    add_feat_selection_at_root(&root, &request.feat_id, &request.saved_at)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSkillAllocationsRequest {
    pub character_id: String,
    /// The caller's complete skill-allocation set (not a delta) — reuses
    /// the `SkillAllocationDto` shape already established for
    /// import/export so the wire contract stays consistent across this
    /// module's DTOs.
    pub skill_allocations: Vec<SkillAllocationDto>,
    pub saved_at: String,
}

/// Loads the saved character, replaces its skill allocations wholesale,
/// recomputes via the real engine, and re-saves — see
/// `set_skill_allocations_at_root` for the full semantics. Replaces rather
/// than appends because `SkillAllocationDialog.onAccept` always sends its
/// complete draft allocation, not an incremental change.
#[tauri::command]
pub fn set_skill_allocations(
    app: tauri::AppHandle,
    request: SetSkillAllocationsRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let skill_allocations = request
        .skill_allocations
        .into_iter()
        .map(|skill| SkillAllocation {
            skill_id: skill.skill_id,
            ranks: skill.ranks,
        })
        .collect();
    set_skill_allocations_at_root(&root, skill_allocations, &request.saved_at)
}

fn resolve_characters_root(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))?;
    Ok(characters_root_from_app_data_dir(&app_data_dir))
}

/// `pub(crate)` (rather than private) so the `characterHub` submodule's
/// commands (e.g. `appendToCharacter` — SD-24 Epic 7, Criterion 7.1) can
/// resolve the same on-disk character root this module's own commands use,
/// without re-deriving the app-data-dir resolution a second time.
pub(crate) fn resolve_character_root(
    app: &tauri::AppHandle,
    character_id: &str,
) -> Result<PathBuf, String> {
    Ok(resolve_characters_root(app)?.join(character_id))
}

const PORTRAIT_FILE_NAME: &str = "portrait.png";
// The frontend crops/resizes to a small fixed square before ever sending
// bytes here; this is a defensive backstop against a buggy or malicious
// caller, not the primary size control.
const MAX_PORTRAIT_BYTES: usize = 3 * 1024 * 1024;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCharacterPortraitRequest {
    pub character_id: String,
    /// PNG image bytes, base64-encoded (no `data:` URL prefix).
    pub image_base64: String,
}

/// Persists a character portrait as `portrait.png` alongside the character's
/// existing envelope/input files. Requires the character to already be
/// saved — a portrait is never the first write to a character directory.
#[tauri::command]
pub fn save_character_portrait(
    app: tauri::AppHandle,
    request: SaveCharacterPortraitRequest,
) -> Result<(), String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(request.image_base64.as_bytes())
        .map_err(|err| format!("invalid base64 image data: {err}"))?;
    if bytes.len() > MAX_PORTRAIT_BYTES {
        return Err(format!(
            "portrait image is {} bytes, over the {MAX_PORTRAIT_BYTES} byte limit",
            bytes.len()
        ));
    }

    let root = resolve_character_root(&app, &request.character_id)?;
    if !root.exists() {
        return Err(format!(
            "no saved character found for id {}",
            request.character_id
        ));
    }

    let path = root.join(PORTRAIT_FILE_NAME);
    std::fs::write(&path, &bytes).map_err(|err| format!("{}: {err}", path.display()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCharacterJsonRequest {
    /// Destination path chosen by the frontend via the dialog plugin's
    /// `save()` picker — this command never resolves its own path.
    pub file_path: String,
    pub contents: String,
}

/// Writes arbitrary JSON text (the character export payload) to a path the
/// user picked themselves. Unlike the other character-hub commands, the
/// destination is outside the app's own data directory, so it's taken as-is
/// rather than resolved from a character id.
#[tauri::command]
pub fn export_character_json(request: ExportCharacterJsonRequest) -> Result<(), String> {
    let path = PathBuf::from(&request.file_path);
    std::fs::write(&path, request.contents.as_bytes()).map_err(|err| format!("{}: {err}", path.display()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterPortraitRequest {
    pub character_id: String,
}

/// Returns the character's portrait as a `data:image/png;base64,...` URL, or
/// `None` if no portrait has been uploaded for this character.
#[tauri::command]
pub fn load_character_portrait(
    app: tauri::AppHandle,
    request: LoadCharacterPortraitRequest,
) -> Result<Option<String>, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let path = root.join(PORTRAIT_FILE_NAME);
    if !path.exists() {
        return Ok(None);
    }

    let bytes = std::fs::read(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok(Some(format!("data:image/png;base64,{encoded}")))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCharacterPortraitRequest {
    pub character_id: String,
}

#[tauri::command]
pub fn delete_character_portrait(
    app: tauri::AppHandle,
    request: DeleteCharacterPortraitRequest,
) -> Result<(), String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let path = root.join(PORTRAIT_FILE_NAME);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    }
    Ok(())
}

// ----- `update_character_bio` / `load_character_bio` (v0.6 alpha swarm) -----
//
// Bio fields (alignment/deity/sex/age/height/weight/hair/eyes) are pure
// display flavor text -- no rules-engine calculation reads any of them.
// Rather than adding a field to `ChosenCharacterState` (constructed as a
// struct literal at ~70 call sites across this crate's own test suite,
// mostly qa-owned `tests/**` -- a schema change there would break all of
// them for data the compute engine never touches), bio is persisted as its
// own sidecar file (`bio.json`) alongside the character's existing
// envelope/input files, mirroring `save_character_portrait`/
// `load_character_portrait`/`delete_character_portrait`'s own established
// sidecar-file precedent (`portrait.png`) exactly -- same directory, same
// "requires the character to already be saved" invariant, same shape.

const BIO_FILE_NAME: &str = "bio.json";

/// One character's bio/flavor fields. Every field defaults to an empty
/// string (`#[serde(default)]`) so a bio file written before a future field
/// is added, or a character with no bio file at all, still deserializes
/// (via `Default`) rather than failing to load.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterBioDto {
    #[serde(default)]
    pub alignment: String,
    #[serde(default)]
    pub deity: String,
    #[serde(default)]
    pub sex: String,
    #[serde(default)]
    pub age: String,
    #[serde(default)]
    pub height: String,
    #[serde(default)]
    pub weight: String,
    #[serde(default)]
    pub hair: String,
    #[serde(default)]
    pub eyes: String,
}

/// Writes `bio` as `bio.json` in the character's root directory. Requires
/// the character to already be saved -- mirrors
/// `save_character_portrait`'s own "a portrait is never the first write to
/// a character directory" invariant. Checks via
/// `SavedCharacterStore::load` (not merely `root.exists()` -- the
/// characters-root directory itself may already exist without a saved
/// envelope in it, e.g. under this function's own test fixtures, whose
/// `tempdir` helper always creates the directory) so a bio is never the
/// first write to a character directory either. Split from the
/// `#[tauri::command]` wrapper below so it is unit-testable without an
/// `AppHandle`.
fn save_character_bio_at_root(root: &Path, bio: &CharacterBioDto) -> Result<(), String> {
    SavedCharacterStore::load(root).map_err(|err| err.message)?;
    let path = root.join(BIO_FILE_NAME);
    let json = serde_json::to_string_pretty(bio)
        .map_err(|err| format!("failed to serialize character bio: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))
}

/// Reads `bio.json` from the character's root directory, or an all-empty
/// `CharacterBioDto::default()` when no bio has ever been saved for this
/// character -- never an error for the common "no bio yet" case, matching
/// `load_character_portrait`'s own `Ok(None)`-for-absent shape (bio's
/// equivalent "absent" value is the default-empty DTO, since every field is
/// already optional-shaped as an empty string rather than an `Option`).
fn load_character_bio_at_root(root: &Path) -> Result<CharacterBioDto, String> {
    let path = root.join(BIO_FILE_NAME);
    if !path.exists() {
        return Ok(CharacterBioDto::default());
    }
    let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    serde_json::from_str(&contents).map_err(|err| format!("{}: invalid bio JSON: {err}", path.display()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCharacterBioRequest {
    pub character_id: String,
    pub bio: CharacterBioDto,
}

/// Persists the caller's full bio field set as `bio.json` alongside the
/// character's existing saved files. Requires the character to already be
/// saved. Always the character's *complete* bio (not a delta) -- the
/// frontend's bio editor already holds every field's current value, so it
/// always sends the full set on save.
#[tauri::command]
pub fn update_character_bio(
    app: tauri::AppHandle,
    request: UpdateCharacterBioRequest,
) -> Result<(), String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    save_character_bio_at_root(&root, &request.bio)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterBioRequest {
    pub character_id: String,
}

/// Reads the character's persisted bio, or an all-empty `CharacterBioDto`
/// when none has been saved yet (including when the character itself does
/// not exist -- resolving a nonexistent root still yields a real path, and
/// reading a bio file that isn't there is the same "nothing saved yet"
/// case either way, so this command does not separately error on a missing
/// character the way the mutation commands do).
#[tauri::command]
pub fn load_character_bio(
    app: tauri::AppHandle,
    request: LoadCharacterBioRequest,
) -> Result<CharacterBioDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    load_character_bio_at_root(&root)
}

// ----- `load_character_money` / `adjust_character_money` (v0.6 alpha swarm) -----
//
// Persisted the same sidecar-file way as bio (a `money.json` file, not a
// `ChosenCharacterState` field) -- the alpha bar's "money conversion" calc
// is the denomination-conversion math itself (`codex::rules_core::money`),
// which needs a canonical balance to convert, not a rules-engine-visible
// character-build field the way skill/feat/equipment selections are. Only
// the canonical `total_copper` is ever persisted; the pp/gp/sp/cp
// breakdown in `CharacterMoneyDto` is always derived fresh from it via
// `money::copper_to_denominations`, never stored redundantly (matching
// `money.rs`'s own "never two numbers that could drift apart" doc
// comment).

const MONEY_FILE_NAME: &str = "money.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredMoney {
    #[serde(default)]
    total_copper: u64,
}

/// The wire response for both money commands: the canonical
/// `total_copper` balance plus its derived platinum/gold/silver/copper
/// breakdown, so the frontend never re-implements the conversion math.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterMoneyDto {
    pub total_copper: u64,
    pub platinum: u64,
    pub gold: u64,
    pub silver: u64,
    pub copper: u64,
}

fn money_dto_from_total(total_copper: u64) -> CharacterMoneyDto {
    let denominations = money::copper_to_denominations(total_copper);
    CharacterMoneyDto {
        total_copper,
        platinum: denominations.platinum,
        gold: denominations.gold,
        silver: denominations.silver,
        copper: denominations.copper,
    }
}

/// Reads the character's persisted money balance, or a zero balance when
/// no `money.json` has ever been saved for this character -- mirrors
/// `load_character_bio_at_root`'s own "no error for the common absent
/// case" shape.
fn load_character_money_at_root(root: &Path) -> Result<CharacterMoneyDto, String> {
    let path = root.join(MONEY_FILE_NAME);
    if !path.exists() {
        return Ok(money_dto_from_total(0));
    }
    let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let stored: StoredMoney = serde_json::from_str(&contents)
        .map_err(|err| format!("{}: invalid money JSON: {err}", path.display()))?;
    Ok(money_dto_from_total(stored.total_copper))
}

/// Applies `delta_copper` (positive to add funds, negative to spend) to
/// the character's persisted balance and returns the new total's
/// denomination breakdown. Requires the character to already be saved
/// (checked via `SavedCharacterStore::load`, same as `save_character_bio_at_root`).
/// Fails honestly with an insufficient-funds error rather than silently
/// allowing a negative balance -- PF1 characters cannot carry negative
/// money.
fn adjust_character_money_at_root(root: &Path, delta_copper: i64) -> Result<CharacterMoneyDto, String> {
    SavedCharacterStore::load(root).map_err(|err| err.message)?;

    let current_total_copper = load_character_money_at_root(root)?.total_copper;
    let new_total = i64::try_from(current_total_copper)
        .map_err(|_| "current balance overflows a signed 64-bit total".to_owned())?
        + delta_copper;
    if new_total < 0 {
        return Err(format!(
            "insufficient funds: balance is {current_total_copper} cp, requested change is \
             {delta_copper} cp"
        ));
    }
    let new_total_copper = new_total as u64;

    let path = root.join(MONEY_FILE_NAME);
    let json = serde_json::to_string_pretty(&StoredMoney { total_copper: new_total_copper })
        .map_err(|err| format!("failed to serialize character money: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))?;

    Ok(money_dto_from_total(new_total_copper))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterMoneyRequest {
    pub character_id: String,
}

#[tauri::command]
pub fn load_character_money(
    app: tauri::AppHandle,
    request: LoadCharacterMoneyRequest,
) -> Result<CharacterMoneyDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    load_character_money_at_root(&root)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCharacterMoneyRequest {
    pub character_id: String,
    /// Positive to add funds (e.g. selling an item, starting gold),
    /// negative to spend (e.g. buying equipment at its `cost_gp`, converted
    /// via `money::gp_to_copper`).
    pub delta_copper: i64,
}

#[tauri::command]
pub fn adjust_character_money(
    app: tauri::AppHandle,
    request: AdjustCharacterMoneyRequest,
) -> Result<CharacterMoneyDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    adjust_character_money_at_root(&root, request.delta_copper)
}

// ----- `load_character_durability` / `adjust_character_hp` (v0.6 alpha swarm) -----
//
// Max HP is a real, derived-from-the-build value (`durability::compute_max_hp`,
// scoped to single-class Fighter/Wizard/Rogue -- see that module's own doc
// comment for why multiclass is honestly out of scope rather than guessed
// at). Current HP / nonlethal damage are live-tracking values persisted as
// a `hp.json` sidecar (same pattern as bio/money): `current_hp` defaults to
// the computed `max_hp` the first time a character is loaded (no file on
// disk yet), then only ever changes via `adjust_character_hp`.

const HP_FILE_NAME: &str = "hp.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredHp {
    /// `None` (the field absent, or no file at all) means "never
    /// initialized yet" -- distinct from `Some(0)`, a real character at 0
    /// HP. Defaulted to the computed `max_hp` on first load, not before.
    current_hp: Option<i16>,
    #[serde(default)]
    nonlethal_damage: i16,
}

fn durability_status_label(status: DurabilityStatus) -> &'static str {
    match status {
        DurabilityStatus::Normal => "Normal",
        DurabilityStatus::Staggered => "Staggered",
        DurabilityStatus::Disabled => "Disabled",
        DurabilityStatus::Unconscious => "Unconscious",
        DurabilityStatus::Dying => "Dying",
        DurabilityStatus::Dead => "Dead",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterDurabilityDto {
    pub max_hp: i16,
    pub current_hp: i16,
    pub nonlethal_damage: i16,
    /// One of `"Normal"` / `"Staggered"` / `"Disabled"` / `"Unconscious"` /
    /// `"Dying"` / `"Dead"` -- see `durability::classify_durability`'s own
    /// doc comment for the exact threshold rules.
    pub status: String,
}

/// Computes `max_hp` from the saved character's real build (class levels +
/// effective, racial-bonus-aware Constitution score, same
/// `apply_human_ability_bonus` reuse `encumbrance`'s wiring in
/// `contract.rs` already established) and reads persisted current-HP/
/// nonlethal-damage from `hp.json`, defaulting `current_hp` to the freshly
/// computed `max_hp` when no file exists yet. Fails honestly (rather than
/// fabricating a value) when the build is multiclass or an unsupported
/// class -- `compute_max_hp` returns `None` for exactly those cases.
fn load_character_durability_at_root(root: &Path) -> Result<CharacterDurabilityDto, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    let mut discarded_explanations = Vec::new();
    let effective_scores = apply_human_ability_bonus(&envelope.character_input, &mut discarded_explanations);
    let constitution_score = effective_scores.constitution;
    let constitution_modifier = ability_modifier(constitution_score);

    let base_max_hp =
        compute_max_hp(&envelope.character_input.chosen.class_levels, constitution_modifier)
            .ok_or_else(|| {
                "durability is only computed for a single-class Fighter, Wizard, or Rogue build \
                 today; this character's class levels are not one of those"
                    .to_owned()
            })?;
    // v0.6 alpha swarm item 17 (feat-effects engine): a grounded feat's real
    // hit-point bonus (currently just Toughness's flat +3) is added on top
    // of the class/Constitution-derived base, not folded into
    // compute_max_hp itself -- feat effects are a per-character add-on, not
    // part of the class hit-die table durability.rs owns.
    let max_hp = base_max_hp
        + feat_effects::hp_bonus_from_feats(&envelope.character_input.chosen.selected_feats);

    let path = root.join(HP_FILE_NAME);
    let stored: StoredHp = if path.exists() {
        let contents = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        serde_json::from_str(&contents)
            .map_err(|err| format!("{}: invalid hp JSON: {err}", path.display()))?
    } else {
        StoredHp::default()
    };
    let current_hp = stored.current_hp.unwrap_or(max_hp);
    let status = classify_durability(current_hp, stored.nonlethal_damage, constitution_score);

    Ok(CharacterDurabilityDto {
        max_hp,
        current_hp,
        nonlethal_damage: stored.nonlethal_damage,
        status: durability_status_label(status).to_owned(),
    })
}

/// Applies `delta_hp` (positive to heal, negative to take lethal damage)
/// and/or `delta_nonlethal` (positive to take nonlethal damage, negative to
/// recover from it) to the character's persisted HP state, clamping
/// `current_hp` at the computed `max_hp` ceiling (healing cannot exceed
/// max) and `nonlethal_damage` at a floor of 0 (cannot recover past no
/// nonlethal damage). `current_hp` is allowed to go negative (dying/dead is
/// a real, trackable state, not an error) but `adjust_character_hp` still
/// requires the character to already be saved and be a durability-
/// supported build, same as `load_character_durability_at_root`.
fn adjust_character_hp_at_root(
    root: &Path,
    delta_hp: i16,
    delta_nonlethal: i16,
) -> Result<CharacterDurabilityDto, String> {
    let current = load_character_durability_at_root(root)?;

    let new_current_hp = (current.current_hp + delta_hp).min(current.max_hp);
    let new_nonlethal_damage = (current.nonlethal_damage + delta_nonlethal).max(0);

    let path = root.join(HP_FILE_NAME);
    let json = serde_json::to_string_pretty(&StoredHp {
        current_hp: Some(new_current_hp),
        nonlethal_damage: new_nonlethal_damage,
    })
    .map_err(|err| format!("failed to serialize character hp: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("{}: {err}", path.display()))?;

    load_character_durability_at_root(root)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCharacterDurabilityRequest {
    pub character_id: String,
}

#[tauri::command]
pub fn load_character_durability(
    app: tauri::AppHandle,
    request: LoadCharacterDurabilityRequest,
) -> Result<CharacterDurabilityDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    load_character_durability_at_root(&root)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCharacterHpRequest {
    pub character_id: String,
    /// Positive to heal, negative to take lethal damage.
    #[serde(default)]
    pub delta_hp: i16,
    /// Positive to take nonlethal damage, negative to recover from it.
    #[serde(default)]
    pub delta_nonlethal: i16,
}

#[tauri::command]
pub fn adjust_character_hp(
    app: tauri::AppHandle,
    request: AdjustCharacterHpRequest,
) -> Result<CharacterDurabilityDto, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    adjust_character_hp_at_root(&root, request.delta_hp, request.delta_nonlethal)
}

// ----- `delete_character` (Storage Tier Minimal Fix, Criterion 22) -----

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCharacterRequest {
    pub character_id: String,
}

/// The wire response for `delete_character`. Unlike every other
/// character-hub command, a failure is carried inside the payload
/// (`ok: false`, `error: Some(..)`) rather than raised as a rejected Tauri
/// IPC call — this matches the criterion's literal `{ ok, error? }` return
/// contract, so the Load Character screen can show an inline status message
/// without a try/catch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCharacterResponse {
    pub ok: bool,
    pub error: Option<String>,
}

/// Removes the saved character's on-disk directory tree at `root`.
///
/// Deleting a character that has no saved directory (already deleted, or
/// never existed) is a deliberate `ok: true` — see
/// `SavedCharacterStore::delete`'s own doc comment for the idempotent-delete
/// rationale (mirrors this module's existing `delete_character_portrait`
/// idiom of not erroring when there is nothing to remove). Any other
/// failure (I/O error, permissions) is surfaced honestly as `ok: false`
/// with a real message, never silently swallowed.
///
/// Split from the `#[tauri::command]` wrapper below so it is unit-testable
/// against a real `SavedCharacterStore` fixture without an `AppHandle` —
/// mirrors every other `_at_root` function in this module.
fn delete_character_at_root(root: &Path) -> DeleteCharacterResponse {
    match SavedCharacterStore::delete(root) {
        Ok(()) => DeleteCharacterResponse { ok: true, error: None },
        Err(err) => DeleteCharacterResponse {
            ok: false,
            error: Some(err.message),
        },
    }
}

/// Resolves the character's root directory and deletes it — see
/// `delete_character_at_root` for the full semantics.
#[tauri::command]
pub fn delete_character(
    app: tauri::AppHandle,
    request: DeleteCharacterRequest,
) -> DeleteCharacterResponse {
    let root = match resolve_character_root(&app, &request.character_id) {
        Ok(root) => root,
        Err(err) => return DeleteCharacterResponse {
            ok: false,
            error: Some(err),
        },
    };
    delete_character_at_root(&root)
}

// ----- `import_character` / `export_character` (Storage Tier Minimal Fix,
// Criteria 23-24) -----
//
// Manual re-projection of `CharacterInput`'s full shape into DTOs, for the
// same reason as this module's top-of-file note: the `codex` crate has zero
// dependencies, so `CharacterInput` itself is neither `Serialize` nor
// `Deserialize`. These DTOs derive both: `import_character` deserializes an
// `ImportedCharacterFileDto` from the picked file's JSON, and
// `export_character` serializes one back out — the exact same wire shape in
// both directions, so a file this module exports is always a file it can
// import (Criterion 24's export/import round-trip fix). Field names and wire
// values (e.g. `ActiveStateDto`'s `"EquippedActive"`/`"Absent"`/
// `"SelectedInactive"`, `AcquisitionModeDto`'s `"Known"`/`"Prepared"`/
// `"Granted"`) reuse the exact DTOs already established for
// `add_equipment_selection`/`add_spell_selection`, so the import/export JSON
// shape is consistent with the rest of this module's wire contract rather
// than a parallel invention.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterClassLevelDto {
    pub class_id: String,
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillAllocationDto {
    pub skill_id: String,
    pub ranks: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentSelectionImportDto {
    pub item_id: String,
    pub active_state: ActiveStateDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedChoiceDto {
    pub choice_set_id: String,
    pub selection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellSelectionImportDto {
    pub spell_id: String,
    pub source_class_id: String,
    pub acquisition_mode: AcquisitionModeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChosenCharacterStateDto {
    pub race_id: String,
    pub class_levels: Vec<CharacterClassLevelDto>,
    pub ability_scores: AbilityScoresDto,
    #[serde(default)]
    pub selected_feats: Vec<String>,
    #[serde(default)]
    pub skill_allocations: Vec<SkillAllocationDto>,
    #[serde(default)]
    pub equipment_selections: Vec<EquipmentSelectionImportDto>,
    #[serde(default)]
    pub selected_choices: Vec<SelectedChoiceDto>,
    #[serde(default)]
    pub spells_selected: Vec<SpellSelectionImportDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInputDto {
    pub source_package_id: String,
    pub chosen: ChosenCharacterStateDto,
}

/// The on-disk/exported shape `import_character` expects (and
/// `export_character` produces): a `displayLabel` + `characterInput` (the
/// fields needed to rebuild a real `CharacterInput`) — the same two fields a
/// full `SavedCharacterEnvelope` export carries. Every other envelope field
/// (`characterId`, `revisionId`, `savedAt`, ...) is intentionally NOT part of
/// this DTO: importing always mints a fresh identity/revision/timestamp
/// rather than trusting the source file's own claimed identity (mirrors
/// `clone_character`'s own "never blindly trust source identity" stance).
/// `serde` ignores any extra fields the source JSON carries on import, so a
/// real full-envelope export file is tolerated rather than rejected for
/// having "extra" fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedCharacterFileDto {
    pub display_label: String,
    pub character_input: CharacterInputDto,
}

fn character_input_from_dto(dto: CharacterInputDto, fresh_character_id: &str) -> CharacterInput {
    CharacterInput {
        case_id: Some(fresh_character_id.to_owned()),
        source_package_id: dto.source_package_id,
        chosen: ChosenCharacterState {
            race_id: dto.chosen.race_id,
            class_levels: dto
                .chosen
                .class_levels
                .into_iter()
                .map(|class_level| CharacterClassLevel {
                    class_id: class_level.class_id,
                    level: class_level.level,
                })
                .collect(),
            ability_scores: AbilityScores {
                strength: dto.chosen.ability_scores.strength,
                dexterity: dto.chosen.ability_scores.dexterity,
                constitution: dto.chosen.ability_scores.constitution,
                intelligence: dto.chosen.ability_scores.intelligence,
                wisdom: dto.chosen.ability_scores.wisdom,
                charisma: dto.chosen.ability_scores.charisma,
            },
            selected_feats: dto.chosen.selected_feats,
            skill_allocations: dto
                .chosen
                .skill_allocations
                .into_iter()
                .map(|skill| SkillAllocation {
                    skill_id: skill.skill_id,
                    ranks: skill.ranks,
                })
                .collect(),
            equipment_selections: dto
                .chosen
                .equipment_selections
                .into_iter()
                .map(|equipment| EquipmentSelection {
                    item_id: equipment.item_id,
                    equipped_or_active: matches!(
                        equipment.active_state,
                        ActiveStateDto::EquippedActive
                    ),
                    active_state: equipment.active_state.into(),
                    applied_modifiers: Vec::new(),
                })
                .collect(),
            selected_choices: dto
                .chosen
                .selected_choices
                .into_iter()
                .map(|choice| SelectedChoice {
                    choice_set_id: choice.choice_set_id,
                    selection_id: choice.selection_id,
                })
                .collect(),
            spells_selected: dto
                .chosen
                .spells_selected
                .into_iter()
                .map(|spell| SpellSelection {
                    spell_id: spell.spell_id,
                    source_class_id: spell.source_class_id,
                    acquisition_mode: spell.acquisition_mode.into(),
                })
                .collect(),
            // v0.6 alpha swarm, risks item 8 (combat-time activation state):
            // no DTO field exists for this yet -- out of scope for this
            // backend-only slice, mirroring how `applied_modifiers` just
            // above is also not yet carried through the import DTO.
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Reverse of `character_input_from_dto` — projects a real `CharacterInput`
/// into the wire DTO `export_character` serializes. `case_id` and
/// `selection_provenance` are intentionally NOT carried into the DTO: they
/// have no field in `CharacterInputDto` (mirroring `character_input_from_dto`
/// always minting its own `case_id` and discarding `selection_provenance` on
/// import), so a round trip through export -> import always ends with a
/// fresh identity, never the source character's own.
fn character_input_to_dto(input: &CharacterInput) -> CharacterInputDto {
    CharacterInputDto {
        source_package_id: input.source_package_id.clone(),
        chosen: ChosenCharacterStateDto {
            race_id: input.chosen.race_id.clone(),
            class_levels: input
                .chosen
                .class_levels
                .iter()
                .map(|class_level| CharacterClassLevelDto {
                    class_id: class_level.class_id.clone(),
                    level: class_level.level,
                })
                .collect(),
            ability_scores: AbilityScoresDto {
                strength: input.chosen.ability_scores.strength,
                dexterity: input.chosen.ability_scores.dexterity,
                constitution: input.chosen.ability_scores.constitution,
                intelligence: input.chosen.ability_scores.intelligence,
                wisdom: input.chosen.ability_scores.wisdom,
                charisma: input.chosen.ability_scores.charisma,
            },
            selected_feats: input.chosen.selected_feats.clone(),
            skill_allocations: input
                .chosen
                .skill_allocations
                .iter()
                .map(|skill| SkillAllocationDto {
                    skill_id: skill.skill_id.clone(),
                    ranks: skill.ranks,
                })
                .collect(),
            equipment_selections: input
                .chosen
                .equipment_selections
                .iter()
                .map(|equipment| EquipmentSelectionImportDto {
                    item_id: equipment.item_id.clone(),
                    active_state: equipment.active_state.into(),
                })
                .collect(),
            selected_choices: input
                .chosen
                .selected_choices
                .iter()
                .map(|choice| SelectedChoiceDto {
                    choice_set_id: choice.choice_set_id.clone(),
                    selection_id: choice.selection_id.clone(),
                })
                .collect(),
            spells_selected: input
                .chosen
                .spells_selected
                .iter()
                .map(|spell| SpellSelectionImportDto {
                    spell_id: spell.spell_id.clone(),
                    source_class_id: spell.source_class_id.clone(),
                    acquisition_mode: spell.acquisition_mode.into(),
                })
                .collect(),
        },
    }
}

/// Parses `contents` as an `ImportedCharacterFileDto`, converts it to a real
/// `CharacterInput` under `fresh_character_id`, recomputes via the real
/// engine, and — mirroring `create_character`/`clone_character`/
/// `level_up_character_at_root`'s own "never persist an unproven build"
/// invariant — only saves and returns `Saved` if the import reaches
/// `Computed`; a structurally valid import the engine cannot compute
/// returns `Blocked` with real diagnostics rather than silently writing a
/// broken build. Malformed or schema-invalid JSON (parse failure) is a
/// distinct failure mode from `Blocked` and is surfaced as a real `Err`,
/// never papered over as an empty/successful import.
///
/// Split from the `#[tauri::command]` wrapper below so it is unit-testable
/// against a real `SavedCharacterStore` fixture without an `AppHandle` or a
/// real file on disk — mirrors every other `_at_root`/`_from_*` function in
/// this module.
fn import_character_from_json(
    contents: &str,
    root: &Path,
    fresh_character_id: &str,
    saved_at: &str,
    app_version: &str,
) -> Result<CreateCharacterResponse, String> {
    let parsed: ImportedCharacterFileDto = serde_json::from_str(contents)
        .map_err(|err| format!("invalid character import JSON: {err}"))?;

    let character_input = character_input_from_dto(parsed.character_input, fresh_character_id);

    let (snapshot, corpus_receipt) =
        match resolve_unified_pilot_snapshot(&character_input, corpus_fixture_bundle()) {
            Ok(result) => result,
            Err(diagnostics) => {
                return Ok(CreateCharacterResponse::Blocked {
                    diagnostics: map_diagnostics_dto(&diagnostics),
                });
            }
        };

    let envelope = SavedCharacterEnvelope {
        character_id: fresh_character_id.to_owned(),
        revision_id: format!("{fresh_character_id}.rev.1"),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: saved_at.to_owned(),
        schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
        app_or_runtime_version: app_version.to_owned(),
        content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
        game_system: GAME_SYSTEM_ID.to_owned(),
        latest_authoritative_revision_ref: format!("{fresh_character_id}.rev.1"),
        display_label: parsed.display_label,
        character_input,
    };

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(&snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportCharacterRequest {
    /// Path to a JSON file shaped like `ImportedCharacterFileDto` (a real
    /// `displayLabel` + `characterInput`; extra full-envelope fields are
    /// tolerated and ignored). This command reads the file itself — the
    /// caller (the file-open dialog) only needs to supply the path, mirroring
    /// how `export_character_json`'s `file_path` names a destination it
    /// writes to directly.
    pub file_path: String,
    pub saved_at: String,
}

/// Reads `file_path`, mints a fresh `character_id` (a v4 UUID, matching the
/// shape the frontend's own `crypto.randomUUID()` ids already use for
/// `create_character`), and imports it — see `import_character_from_json`
/// for the full parse -> recompute -> save semantics.
#[tauri::command]
pub fn import_character(
    app: tauri::AppHandle,
    request: ImportCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let contents = std::fs::read_to_string(&request.file_path)
        .map_err(|err| format!("{}: {err}", request.file_path))?;

    let fresh_character_id = Uuid::new_v4().to_string();
    let root = resolve_character_root(&app, &fresh_character_id)?;
    let app_version = app.package_info().version.to_string();

    import_character_from_json(
        &contents,
        &root,
        &fresh_character_id,
        &request.saved_at,
        &app_version,
    )
}

// ----- `export_character` (Storage Tier Minimal Fix, Criterion 24) -----

/// Loads the saved character at `root` and serializes it into the exact
/// `ImportedCharacterFileDto` shape `import_character_from_json` parses —
/// this is what makes Export -> Import a real round trip rather than the
/// old lossy `{summary, detail}` export payload (see this module's own
/// history: the Load Character screen used to build its export payload from
/// computed summary/snapshot data, which carries no raw `CharacterInput` and
/// so could never be re-imported).
///
/// Split from the `#[tauri::command]` wrapper below so it is unit-testable
/// against a real `SavedCharacterStore` fixture without an `AppHandle` —
/// mirrors every other `_at_root`/`_from_*`/`_to_*` function in this module.
fn export_character_to_json(root: &Path) -> Result<String, String> {
    let envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    let dto = ImportedCharacterFileDto {
        display_label: envelope.display_label,
        character_input: character_input_to_dto(&envelope.character_input),
    };

    serde_json::to_string_pretty(&dto).map_err(|err| format!("failed to serialize character for export: {err}"))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCharacterRequest {
    pub character_id: String,
    /// Destination path chosen by the frontend via the dialog plugin's
    /// `save()` picker, mirroring `export_character_json`'s own `file_path`
    /// (this command writes directly, same as that one).
    pub file_path: String,
}

/// Resolves the character's root directory, serializes its real saved build
/// into the `ImportedCharacterFileDto` shape, and writes it to `file_path` —
/// see `export_character_to_json` for the full semantics. Unlike
/// `export_character_json` (which writes whatever `contents` string the
/// caller already built), this command builds the export payload itself
/// from the real on-disk envelope, so what it writes is always importable.
#[tauri::command]
pub fn export_character(app: tauri::AppHandle, request: ExportCharacterRequest) -> Result<(), String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    let contents = export_character_to_json(&root)?;

    let path = PathBuf::from(&request.file_path);
    std::fs::write(&path, contents.as_bytes()).map_err(|err| format!("{}: {err}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex::rules_core::pilot_compute::HeadlessReceiptStatus;
    use std::collections::BTreeSet;

    const CURATED_RACE_IDS: [&str; 7] = [
        "race:human",
        "race:dwarf",
        "race:elf",
        "race:gnome",
        "race:half-elf",
        "race:half-orc",
        "race:halfling",
    ];
    const FIGHTER_CLASS_ID: &str = "class:fighter";

    const GENERIC_DIAGNOSTIC_IDS: [&str; 4] = [
        "class_chassis.unsupported",
        "combat.baseline_unsupported",
        "defense.total_save.unsupported",
        "skill.selected_modifier.unsupported",
    ];

    fn request_for(race_id: &str, level: u8) -> CreateCharacterRequest {
        request_for_class(race_id, FIGHTER_CLASS_ID, level)
    }

    fn request_for_class(race_id: &str, class_id: &str, level: u8) -> CreateCharacterRequest {
        CreateCharacterRequest {
            character_id: "char-test".to_owned(),
            display_label: "Test Character".to_owned(),
            race_id: race_id.to_owned(),
            class_id: class_id.to_owned(),
            level,
            ability_scores: AbilityScoresDto {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            ability_bonus_target: "strength".to_owned(),
            saved_at: "2026-07-08T00:00:00Z".to_owned(),
        }
    }

    fn claim_blocking_diagnostic_ids(race_id: &str, class_id: &str, level: u8) -> BTreeSet<String> {
        let input = compose_character_input(&request_for_class(race_id, class_id, level));
        let receipt = build_pilot_headless_receipt(&input);
        receipt
            .computation
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.claim_blocking)
            .map(|diagnostic| diagnostic.id.clone())
            .collect()
    }

    fn generic_ids() -> BTreeSet<String> {
        GENERIC_DIAGNOSTIC_IDS.iter().map(|id| id.to_string()).collect()
    }

    fn generic_plus(named: &[&str]) -> BTreeSet<String> {
        let mut ids = generic_ids();
        ids.extend(named.iter().map(|id| id.to_string()));
        ids
    }

    #[test]
    fn compose_character_input_for_human_includes_human_only_choice_slots() {
        let input = compose_character_input(&request_for("race:human", 1));

        let has_choice = |choice_set_id: &str| {
            input
                .chosen
                .selected_choices
                .iter()
                .any(|choice| choice.choice_set_id == choice_set_id)
        };

        assert!(has_choice("choice:human_bonus_feat"));
        assert!(has_choice("choice:human_ability_bonus"));
        assert!(has_choice("choice:level_1_character_feat"));
        assert!(has_choice("choice:fighter_bonus_feat"));
    }

    #[test]
    fn compose_character_input_honors_the_requested_human_ability_bonus_target() {
        let mut request = request_for("race:human", 1);
        request.ability_bonus_target = "dexterity".to_owned();

        let input = compose_character_input(&request);

        let selection = input
            .chosen
            .selected_choices
            .iter()
            .find(|choice| choice.choice_set_id == "choice:human_ability_bonus")
            .map(|choice| choice.selection_id.as_str());

        assert_eq!(selection, Some("ability:dexterity"));
    }

    /// SD-24 Criterion 7.5's own RED sentinel: `compose_character_input`
    /// (the composer behind `create_character` / `seed_default_character_if_needed`)
    /// must not fabricate a demo spell loadout for a freshly composed
    /// character — `CreateCharacterRequest` carries no spell selections at
    /// all, so any non-empty `spells_selected` on the composed input can
    /// only be a hardcoded default, not a real caller choice. A real
    /// character's spells come from the wired `add_spell_selection` /
    /// `appendToCharacter` command surface (SD-23/SD-24 Epic 7) after
    /// creation, not from a baked-in placeholder at creation time.
    #[test]
    fn compose_character_input_does_not_fabricate_a_default_spell_loadout() {
        let input = compose_character_input(&request_for("race:human", 1));

        assert!(
            input.chosen.spells_selected.is_empty(),
            "compose_character_input must not hardcode a demo spell loadout; got {:?}",
            input.chosen.spells_selected
        );
    }

    #[test]
    fn compose_character_input_for_non_human_omits_human_only_choice_slots() {
        let input = compose_character_input(&request_for("race:half-orc", 1));

        let has_choice = |choice_set_id: &str| {
            input
                .chosen
                .selected_choices
                .iter()
                .any(|choice| choice.choice_set_id == choice_set_id)
        };

        assert!(!has_choice("choice:human_bonus_feat"));
        assert!(!has_choice("choice:human_ability_bonus"));
        // The race-agnostic slots remain present.
        assert!(has_choice("choice:level_1_character_feat"));
        assert!(has_choice("choice:fighter_bonus_feat"));
        assert_eq!(input.chosen.race_id, "race:half-orc");
    }

    #[test]
    fn compose_character_input_threads_the_requested_class_id() {
        let input = compose_character_input(&request_for_class("race:human", "class:paladin", 1));
        assert_eq!(input.chosen.class_levels[0].class_id, "class:paladin");
    }

    /// The single most important regression guard: proves the golden-path
    /// claim against the real engine, not just against this module's own
    /// description of it. If the compute engine's requirements ever drift,
    /// this test fails loudly instead of the character-hub UI silently
    /// showing "Blocked" for what users were told was the supported path.
    #[test]
    fn compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3() {
        for race_id in CURATED_RACE_IDS {
            for level in 1..=3u8 {
                let input = compose_character_input(&request_for(race_id, level));
                let receipt = build_pilot_headless_receipt(&input);
                assert_eq!(
                    receipt.status,
                    HeadlessReceiptStatus::Computed,
                    "race {race_id} level {level} must reach Computed status, got diagnostics: {:?}",
                    receipt.computation.diagnostics
                );
            }
        }
    }

    /// Pins the exact claim-blocking diagnostic id set the picker's
    /// grouping UI is built against, for every class/race/level shape the
    /// frontend catalogue treats as distinct. A drift in the compute
    /// engine's diagnostic ids must fail this test loudly, not surface as a
    /// silently-empty "what's missing" panel in the app.
    #[test]
    fn claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class() {
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:fighter", 1),
            BTreeSet::new(),
            "Human Fighter L1 is the golden path and must reach Computed with zero claim-blocking diagnostics"
        );

        // v0.6 alpha swarm, risks item 8, third slice (2026-07-25): `table_class_id`
        // now recognizes Paladin too (mirroring the Ranger widening), so the 4
        // generic chassis-wide diagnostics no longer trip. Paladin's own
        // `class_spell.paladin.partial_caster.unsupported` diagnostic is also no
        // longer unconditional -- it's now a real validation
        // (`unmet_paladin_prepared_spell_conditions`) that only fires on a genuine
        // posture violation, and `compose_character_input` seeds no Paladin spell
        // selections, so the (valid, empty) posture no longer trips it. Only the
        // still-untouched F6 hybrid-level-1 diagnostics remain, naming the
        // non-spell class-feature burden and a separate, more general spell
        // burden than the real one this slice grounds.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:paladin", 1),
            BTreeSet::from([
                "class_feature.hybrid.paladin.unsupported".to_owned(),
                "class_spell.hybrid.paladin.unsupported".to_owned(),
            ]),
            "Human Paladin L1 keeps only the F6 hybrid diagnostics; the real \
             per-class spell-posture diagnostic no longer fires on a valid (empty) posture"
        );

        // v0.6 alpha swarm, risks item 8 (2026-07-24): `table_class_id` now
        // recognizes Ranger (`class-multiclass-breadth-scoping.md`'s
        // recommended first slice), so Ranger no longer trips any of the 4
        // generic chassis-wide diagnostics -- same shape as Wizard's own
        // transition, below. A first version of this assertion here still
        // named `class_spell.ranger.partial_caster.unsupported` because that
        // diagnostic was unconditional at the time; the 2026-07-25 slice made
        // it a real, conditional validation (mirrors the Paladin update just
        // above), and `compose_character_input` seeds no Ranger spell
        // selections, so a Human Ranger L1's (valid, empty) posture no longer
        // trips it -- only the still-untouched F6 hybrid diagnostics remain.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:ranger", 1),
            BTreeSet::from([
                "class_feature.hybrid.ranger.unsupported".to_owned(),
                "class_spell.hybrid.ranger.unsupported".to_owned(),
            ]),
            "Human Ranger L1 keeps only the F6 hybrid diagnostics; the real per-class \
             spell-posture diagnostic no longer fires on a valid (empty) posture"
        );

        // v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25):
        // `table_class_id` now recognizes Barbarian too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The rage-execution
        // burden is also no longer unconditional -- it's a real, conditional
        // engine now (`ground_or_block_barbarian_rage`), and
        // `compose_character_input` seeds no `class_ability_activations`
        // for Barbarian, so the (valid, "not raging") posture no longer
        // trips it either. Human Barbarian L1 reaches Computed with zero
        // claim-blocking diagnostics, the same golden path as Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:barbarian", 1),
            BTreeSet::new(),
            "Human Barbarian L1 (not raging) reaches Computed with zero claim-blocking \
             diagnostics now that the rage-execution engine is real and conditional"
        );

        // v0.6 alpha swarm, risks item 8, fifth slice (2026-07-25):
        // `table_class_id` now recognizes Sorcerer too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The spell-posture
        // diagnostic is also no longer unconditional -- it's a real
        // validation now, and compose_character_input seeds no Sorcerer
        // spell selections, so the (valid, empty) known-spell posture no
        // longer trips it. The bloodline-power diagnostic remains here too
        // -- not because it's still permanently unconditional (a later
        // slice made it real/conditional for a genuinely recognized Arcane
        // bloodline + Arcane Bond choice), but because compose_character_input
        // seeds no bloodline or Arcane Bond choice at all for this bare
        // fixture, so it falls into the still-blocking "no bloodline
        // recognized" branch.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:sorcerer", 1),
            BTreeSet::from([
                "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported"
                    .to_owned(),
            ])
        );

        // SD-21 Epic 6 gave Wizard a real compute_wizard_chassis (BAB + saves, via
        // compute_class_chassis's per-class dispatch), so Wizard no longer trips the
        // two chassis-wide generic diagnostics (class_chassis.unsupported,
        // defense.total_save.unsupported) that every other still-unsupported class
        // does. Epic 6b's E6b.1 cycle then widened compute_combat_baseline and
        // compute_selected_skill_modifiers to the same has_supported_class_chassis
        // gate, so those two also no longer trip for Wizard at any input. v0.6 alpha
        // swarm (bootstrap-deadlock fix): compose_character_input now seeds one
        // canonical starter spell (Known+Prepared) for every Wizard, so the
        // previously-remaining class_spell.wizard.prepared_spellbook.unsupported and
        // class_feature.wizard.school_powers_and_opposed_school_cost.unsupported
        // (Epic 6b's E6b.2/E6b.3) are now genuinely cleared by that seeded,
        // in-budget spellbook -- Wizard L1 is now claim-blocking-diagnostic-free,
        // same golden-path shape as Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:wizard", 1),
            BTreeSet::new(),
            "Human Wizard L1 now reaches Computed with zero claim-blocking diagnostics, thanks \
             to the seeded canonical starter spell"
        );

        // v0.6 alpha swarm, risks item 8, Inspire Courage slice (2026-07-25):
        // `table_class_id` now recognizes Bard too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The bardic-performance-
        // execution burden is also no longer unconditional -- it's a real,
        // conditional engine now (`ground_or_block_bard_bardic_performance`),
        // and `compose_character_input` seeds no `class_ability_activations`
        // for Bard, so the (valid, "not performing") posture no longer trips
        // it either (the "other performances not modeled" diagnostic it
        // pushes unconditionally is deliberately non-claim-blocking, so it
        // never appears in this claim-blocking-only set). The known-spell/
        // per-day spell-posture diagnostic is ALSO no longer unconditional
        // (a later slice made it real/conditional, mirroring Sorcerer's own
        // known-spell closure) -- compose_character_input seeds no Bard
        // spell selections, so the (valid, empty) known-spell posture no
        // longer trips it either. Human Bard L1 now reaches Computed with
        // zero claim-blocking diagnostics, the same golden path as Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:bard", 1),
            BTreeSet::new(),
            "Human Bard L1 (not performing, valid empty known-spell posture) reaches Computed \
             with zero claim-blocking diagnostics"
        );

        // The SD13-E5 Rogue slice grounds trapfinding, the last named Rogue
        // pillar burden, so the named Rogue set was already empty. v0.6 alpha
        // swarm task 4 then widened compute_class_chassis's per-class
        // dispatch (previously Fighter/Wizard only) to also recognize Rogue,
        // the same way Epic 6 did for Wizard above -- so the 4 generic
        // chassis-wide diagnostics (class_chassis.unsupported,
        // combat.baseline_unsupported, defense.total_save.unsupported,
        // skill.selected_modifier.unsupported) no longer trip for Rogue
        // either. Human Rogue L1 now reaches a fully Computed receipt with
        // zero claim-blocking diagnostics, the same golden-path shape as
        // Fighter.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:rogue", 1),
            BTreeSet::new(),
            "Human Rogue L1 now reaches Computed with zero claim-blocking diagnostics, matching \
             Fighter's golden path, since task 4 widened the generic chassis dispatch to Rogue"
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:monk", 1),
            generic_plus(&["class_feature.monk.bounded_progression.bonus_feat.unsupported"])
        );

        // v0.6 alpha swarm, risks item 8, sixth slice (2026-07-25):
        // `table_class_id` now recognizes Cleric too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The spell-posture
        // diagnostic is also no longer unconditional -- it's a real
        // validation now, and compose_character_input seeds no Cleric
        // spell selections, so the (valid, empty) prepared-spell posture no
        // longer trips it. The domain-powers diagnostic remains here too --
        // not because it's still permanently unconditional (a later slice
        // made Good domain's Touch of Good genuinely closable, self-scoped),
        // but because compose_character_input seeds no domain choice at
        // all for this bare fixture, so it falls into the still-blocking
        // catch-all branch (no domain chosen).
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:cleric", 1),
            BTreeSet::from(["class_feature.cleric.domain_powers.unsupported".to_owned()])
        );

        // v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25):
        // `table_class_id` now recognizes Druid too, so the 4 generic
        // chassis-wide diagnostics no longer trip. The spell-posture
        // diagnostic is also no longer unconditional -- it's a real
        // validation now, and compose_character_input seeds no Druid
        // spell selections, so the (valid, empty) prepared-spell posture
        // no longer trips it. The animal-companion/nature-bond diagnostic
        // remains here too -- not because it's still permanently
        // unconditional (a later slice made an animal companion's own
        // Wolf stat block genuinely closable at Druid level 1), but
        // because compose_character_input seeds no nature-bond choice at
        // all for this bare fixture, so it falls into the still-blocking
        // catch-all branch (no nature bond chosen).
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:druid", 1),
            BTreeSet::from(["class_feature.druid.animal_companion.unsupported".to_owned()])
        );

        // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25) This
        // previously proved the Human-only gate collapsed a non-Human Paladin to
        // the 4 generic diagnostics. That's no longer true: `table_class_id`
        // recognizes Paladin regardless of race (real BAB/save/HP via the generic
        // table dispatch), the F6 hybrid diagnostics are Human-gated so they don't
        // fire for a Dwarf, and the real spell-posture check is race-independent
        // and valid (empty) here too -- so a non-Human Paladin L1 now reaches
        // Computed with ZERO claim-blocking diagnostics, same as Fighter/Wizard/
        // Rogue/Ranger's golden path.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:dwarf", "class:paladin", 1),
            BTreeSet::new(),
            "non-Human Paladin L1 now reaches Computed too -- table_class_id and the real \
             spell-posture check are both race-independent"
        );

        // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25) Previously
        // proved only the spell burden remained claim-blocking at level 2. Now
        // that burden is a real, conditional validation instead of an
        // unconditional blocker, and a level-2 Paladin has no spells accessible
        // yet (access ceiling 0) so there's nothing to violate -- Paladin level 2
        // now reaches Computed with zero claim-blocking diagnostics too.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:paladin", 2),
            BTreeSet::new(),
            "Paladin level 2 now reaches Computed: chassis is grounded, and the real spell \
             posture is valid (no spells accessible yet, none prepared)"
        );
    }

    #[test]
    fn characters_root_from_app_data_dir_joins_characters_subdirectory() {
        let app_data_dir = Path::new("/tmp/example-app-data");
        let root = characters_root_from_app_data_dir(app_data_dir);
        assert_eq!(root, PathBuf::from("/tmp/example-app-data/characters"));
    }

    // ----- `mutate_saved_character` operation table (Criterion 16) -----

    #[test]
    fn saved_character_mutation_operations_table_documents_six_ops_all_wired() {
        let names: Vec<&str> = SAVED_CHARACTER_MUTATION_OPERATIONS
            .iter()
            .map(|descriptor| descriptor.name)
            .collect();
        assert_eq!(
            names,
            vec![
                "level_up_character",
                "add_equipment_selection",
                "add_spell_selection",
                "set_skill_allocations",
                "add_feat_selection",
                "record_and_prepare_spell_selection",
            ],
            "the table must enumerate exactly these six ops, in this order"
        );

        let wired: Vec<&str> = SAVED_CHARACTER_MUTATION_OPERATIONS
            .iter()
            .filter(|descriptor| descriptor.wired)
            .map(|descriptor| descriptor.name)
            .collect();
        assert_eq!(
            wired,
            vec![
                "level_up_character",
                "add_equipment_selection",
                "add_spell_selection",
                "set_skill_allocations",
                "add_feat_selection",
                "record_and_prepare_spell_selection",
            ],
            "all six ops are callable through real Tauri commands as of this cycle"
        );

        for descriptor in SAVED_CHARACTER_MUTATION_OPERATIONS.iter() {
            assert!(
                !descriptor.description.is_empty(),
                "{} must document its mutation semantics",
                descriptor.name
            );
        }
    }

    // ----- `add_equipment_selection` (Criterion 18) -----

    #[test]
    fn apply_add_equipment_selection_appends_to_equipment_selections() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.equipment_selections.len();

        apply_add_equipment_selection(&mut input, "item:dagger", ActiveState::EquippedActive);

        assert_eq!(input.chosen.equipment_selections.len(), starting_len + 1);
        let added = input
            .chosen
            .equipment_selections
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.item_id, "item:dagger");
        assert_eq!(added.active_state, ActiveState::EquippedActive);
        assert!(added.equipped_or_active);
    }

    /// The single most important regression guard for Criterion 18's
    /// equipment half: proves the real load -> mutate -> recompute ->
    /// re-save -> return round trip against a real `SavedCharacterStore`
    /// fixture on disk, mirroring `level_up_character_at_root`'s own golden
    /// path test.
    #[test]
    fn add_equipment_selection_at_root_appends_and_persists_when_computed() {
        let root = tempdir("add-equipment-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T00:00:00Z",
        )
        .expect("add equipment selection call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with an added equipment selection must still \
                     reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len + 1,
            "the on-disk envelope must reflect the appended equipment selection"
        );
        let added = reloaded
            .character_input
            .chosen
            .equipment_selections
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.item_id, "item:dagger");
        assert_eq!(added.active_state, ActiveState::EquippedActive);
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// `add_equipment_selection_at_root`'s error path when there is nothing
    /// saved at `root` yet — must fail honestly rather than silently
    /// creating a character out of thin air.
    #[test]
    fn add_equipment_selection_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("add-equipment-missing-character");

        let result = add_equipment_selection_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-21T00:00:00Z",
        );

        assert!(
            result.is_err(),
            "adding an equipment selection to a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- corpus_derived.equipment_effects (risks item 1, shape (c)) -----

    /// A freshly created Fighter's fixed loadout already equips a real
    /// Chain Shirt (`item:chain_shirt`, `EquippedActive`) -- this proves
    /// the new `corpus_derived.equipment_effects` section carries the real,
    /// corpus-resolved armor-check penalty for it end to end through the
    /// actual creation command, not just a unit test on the lower-level
    /// `compute_pilot_with_corpus` seam. The loadout's only other
    /// `EquippedActive` item is the Longsword -- exactly one weapon, no
    /// equipmods equipped -- so `attack_bonus_delta` must land on the
    /// unambiguous real `Some(0)`, proving the bounded single-weapon slice
    /// end to end too.
    #[test]
    fn create_character_at_root_surfaces_the_real_armor_check_penalty_for_the_fixed_loadout() {
        let root = tempdir("create-character-equipment-effects");
        let request = request_for("race:human", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        match response {
            CreateCharacterResponse::Saved { corpus_derived, .. } => {
                assert_ne!(
                    corpus_derived.equipment_effects.armor_check_penalty_total, 0,
                    "the fixed loadout's real Chain Shirt must carry a real, nonzero ACCHECK"
                );
                assert!(
                    corpus_derived.equipment_effects.armor_class_delta > 0,
                    "the fixed loadout's real Chain Shirt must carry a real AC bonus"
                );
                assert_eq!(
                    corpus_derived.equipment_effects.attack_bonus_delta,
                    Some(0),
                    "exactly one weapon (Longsword), no equipmods equipped: real, unambiguous zero"
                );
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 must reach Computed, got: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm (QA finding, 2026-07-24): a real equipment
    /// selection outside the desktop app's tiny bundled demo corpus
    /// (`corpus_fixtures.rs`, ~4 records total) must be traceable in
    /// `corpus_derived.unresolvedEquipmentItemIds`, not silently vanish --
    /// end to end through the real `add_equipment_selection` command
    /// against the real bundled corpus, not a hand-built fixture.
    #[test]
    fn add_equipment_selection_surfaces_an_unresolvable_item_instead_of_silently_dropping_it() {
        let root = tempdir("create-character-unresolved-equipment");
        let request = request_for("race:human", 1);
        create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        let response = add_equipment_selection_at_root(
            &root,
            "Wand of Cure Light Wounds",
            ActiveState::EquippedActive,
            "2026-07-24T00:00:00Z",
        )
        .expect("add call should not error");

        match response {
            CreateCharacterResponse::Saved { corpus_derived, .. } => {
                // The fixed loadout's own `item:shield` (Absent) and
                // `power_attack` (a synthetic feat-toggle id, never a real
                // corpus item) are pre-existing, already-unresolvable
                // entries unrelated to this test -- `contains`, not exact
                // equality, so this doesn't assert on behavior this test
                // isn't about.
                assert!(
                    corpus_derived
                        .unresolved_equipment_item_ids
                        .contains(&"Wand of Cure Light Wounds".to_string()),
                    "an item outside the bundled demo corpus must be traceable, not silently \
                     dropped: {:?}",
                    corpus_derived.unresolved_equipment_item_ids
                );
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("adding an inert extra item must not block an already-Computed build, got: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- create_character: starting wealth (risks item 7) -----

    /// A freshly created Fighter is granted the operator-cited average
    /// starting wealth (175 gp = 17,500 cp) atomically as part of creation,
    /// not as a separate call the caller has to remember to make.
    #[test]
    fn create_character_at_root_grants_the_operator_cited_starting_wealth_for_fighter() {
        let root = tempdir("create-character-starting-wealth-fighter");
        let request = request_for("race:human", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("Human Fighter level 1 must reach Computed, got: {diagnostics:?}")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            17_500,
            "175 gp (5d6 x 10, operator-cited average) = 17,500 cp"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// The other two classes that currently reach `Computed` also get their
    /// own correct, distinct starting wealth -- not a single hardcoded value
    /// applied regardless of class.
    #[test]
    fn create_character_at_root_grants_the_operator_cited_starting_wealth_for_wizard_and_rogue() {
        for (class_id, expected_copper) in [("class:wizard", 7_000_u64), ("class:rogue", 14_000_u64)] {
            let root = tempdir(&format!("create-character-starting-wealth-{class_id}"));
            let request = request_for_class("race:human", class_id, 1);

            let response = create_character_at_root(&root, &request, "test-version".to_owned())
                .expect("create call should not error");

            match response {
                CreateCharacterResponse::Saved { .. } => {}
                CreateCharacterResponse::Blocked { diagnostics } => {
                    panic!("Human {class_id} level 1 must reach Computed, got: {diagnostics:?}")
                }
            }

            assert_eq!(
                load_character_money_at_root(&root).unwrap().total_copper,
                expected_copper,
                "{class_id}'s starting wealth"
            );

            std::fs::remove_dir_all(&root).ok();
        }
    }

    /// A build that does not reach `Computed` must never be granted starting
    /// wealth -- proves the wealth grant is gated on the same successful-save
    /// path as everything else, not a side effect that fires unconditionally.
    #[test]
    fn create_character_at_root_grants_no_wealth_when_the_build_is_blocked() {
        let root = tempdir("create-character-starting-wealth-blocked");
        // Cleric does not reach Computed today (no supported chassis) even
        // though starting_wealth_gp itself recognizes "class:cleric".
        let request = request_for_class("race:human", "class:cleric", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        match response {
            CreateCharacterResponse::Blocked { .. } => {}
            CreateCharacterResponse::Saved { .. } => {
                panic!("Human Cleric level 1 is not expected to reach Computed in this build")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            0,
            "a Blocked build must never be granted wealth, fabricated or otherwise"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm item 7 (second phase, 2026-07-24): explicitly
    /// verifies, rather than assumes, that a non-CRB class id
    /// `starting_wealth_gp` newly recognizes (Alchemist) still reaches
    /// `Blocked` exactly like any other unsupported class -- recognizing
    /// the id for wealth purposes carries no risk of ever granting wealth
    /// to a build that hasn't proven `Computed`, since nothing else in the
    /// compute/chassis dispatch has ever heard of "class:alchemist" either.
    #[test]
    fn create_character_at_root_grants_no_wealth_for_a_newly_recognized_non_crb_class() {
        let root = tempdir("create-character-starting-wealth-non-crb-blocked");
        let request = request_for_class("race:human", "class:alchemist", 1);

        let response = create_character_at_root(&root, &request, "test-version".to_owned())
            .expect("create call should not error");

        match response {
            CreateCharacterResponse::Blocked { .. } => {}
            CreateCharacterResponse::Saved { .. } => {
                panic!(
                    "Human Alchemist level 1 is not expected to reach Computed -- no chassis \
                     dispatch exists for this class id"
                )
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            0,
            "a non-CRB class newly recognized for wealth purposes must still grant zero wealth \
             while genuinely Blocked"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- purchase_equipment: atomic money-purchase coupling (risks item 9) -----

    /// Golden path: an affordable real item is added AND its real catalog
    /// cost is deducted from the balance, atomically, in one call.
    #[test]
    fn purchase_equipment_at_root_succeeds_and_deducts_the_real_catalog_cost() {
        let root = tempdir("purchase-equipment-affordable");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000).expect("funding the character should succeed");

        let response = purchase_equipment_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-23T00:00:00Z",
        )
        .expect("purchase call should not error");

        match response {
            PurchaseEquipmentResponse::Purchased { money, .. } => {
                // A dagger's real catalog cost is 2 gp = 200 cp.
                assert_eq!(
                    money.total_copper, 800,
                    "1000 cp funded minus a dagger's real 200 cp cost must leave 800 cp"
                );
            }
            PurchaseEquipmentResponse::Blocked { diagnostics } => {
                panic!("an affordable real item must be purchased, got Blocked: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len + 1,
            "the equipment must actually be added on the accepted path"
        );
        assert_eq!(load_character_money_at_root(&root).unwrap().total_copper, 800);

        std::fs::remove_dir_all(&root).ok();
    }

    /// An unaffordable item must be honestly rejected -- no equipment
    /// added, no money charged. Proves the pre-flight affordability check
    /// runs BEFORE the equipment mutation, not after.
    #[test]
    fn purchase_equipment_at_root_blocks_and_charges_nothing_when_unaffordable() {
        let root = tempdir("purchase-equipment-unaffordable");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        // Balance starts at 0 cp; a dagger costs 200 cp -- unaffordable.

        let response = purchase_equipment_at_root(
            &root,
            "item:dagger",
            ActiveState::EquippedActive,
            "2026-07-23T00:00:00Z",
        )
        .expect("purchase call should not error");

        match response {
            PurchaseEquipmentResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "money.equipment_purchase.insufficient_funds"),
                    "must carry the real insufficient-funds diagnostic: {diagnostics:?}"
                );
            }
            PurchaseEquipmentResponse::Purchased { .. } => {
                panic!("an unaffordable item must never be silently purchased")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len,
            "nothing should have been added on the rejected path"
        );
        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            0,
            "nothing should have been charged on the rejected path"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// An item with no known catalog cost (a `(Base)` template record) must
    /// be treated the same as unaffordable -- never a free item.
    #[test]
    fn purchase_equipment_at_root_blocks_an_item_with_no_known_cost() {
        let root = tempdir("purchase-equipment-unknown-cost");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000_000).expect("funding should succeed");

        // The bare "Dagger" KEY resolves to the (Base) template record,
        // which carries no independent cost_gp (None) -- a genuine corpus
        // absence, not zero.
        let response = purchase_equipment_at_root(
            &root,
            "Dagger",
            ActiveState::EquippedActive,
            "2026-07-23T00:00:00Z",
        )
        .expect("purchase call should not error");

        match response {
            PurchaseEquipmentResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "money.equipment_purchase.unknown_cost"),
                    "must carry the real unknown-cost diagnostic: {diagnostics:?}"
                );
            }
            PurchaseEquipmentResponse::Purchased { .. } => {
                panic!("an item with no known cost must never be treated as free")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            1_000_000,
            "nothing should have been charged when cost is unknown"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `attach_equipment_modifier` (v0.6 alpha swarm items 1+27 sub-task 6) -----

    /// A real, known-but-formula-priced modifier (an actual magical weapon
    /// enhancement -- the headline use case this command exists for)
    /// attaches for free, since its real `cost_gp` is unknown (a formula,
    /// not a flat catalog price) -- the deliberate deviation from
    /// `purchase_equipment`'s block-on-unknown-cost rule this command's
    /// own doc comment explains.
    #[test]
    fn attach_equipment_modifier_at_root_attaches_a_real_enhancement_for_free_when_cost_is_unknown() {
        let root = tempdir("attach-modifier-free-enhancement");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        // Balance starts at 0 cp -- if this were charged at all, it would
        // be Blocked as unaffordable, proving the free-attach path for
        // real.

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Ability ~ +1 ~ Weapon",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { money, .. } => {
                assert_eq!(money.total_copper, 0, "an unknown-cost modifier must attach for free");
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("a real enhancement with unknown cost must attach for free, got Blocked: {diagnostics:?}")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        let longsword = reloaded
            .character_input
            .chosen
            .equipment_selections
            .iter()
            .find(|selection| selection.item_id == "item:longsword")
            .expect("item:longsword must still be present");
        assert_eq!(
            longsword.applied_modifiers,
            vec!["Special Ability ~ +1 ~ Weapon".to_string()],
            "the modifier must attach to the target selection's applied_modifiers, not a new top-level entry"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A modifier with a real, known `cost_gp` is still charged (not
    /// treated as free just because it's a modifier) -- proves the
    /// free-attach path is specifically for unknown cost, not blanket.
    #[test]
    fn attach_equipment_modifier_at_root_charges_a_modifier_with_a_known_cost() {
        let root = tempdir("attach-modifier-known-cost");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 10_000).expect("funding should succeed");

        // Masterwork (Item) resolves a real, known cost_gp of 50.
        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Quality ~ Masterwork ~ Item",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { money, .. } => {
                assert_eq!(money.total_copper, 5_000, "10,000 cp minus a real 5,000 cp (50 gp) cost");
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("an affordable known-cost modifier must attach, got Blocked: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    /// A known-cost modifier the character cannot afford is `Blocked`,
    /// same as any other charged mutation -- the free-attach deviation
    /// only applies to genuinely unknown cost, never to unaffordability.
    #[test]
    fn attach_equipment_modifier_at_root_blocks_a_known_cost_modifier_when_unaffordable() {
        let root = tempdir("attach-modifier-unaffordable");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        // Balance starts at 0 cp; Masterwork (Item) costs 5,000 cp.

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Quality ~ Masterwork ~ Item",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics
                        .iter()
                        .any(|d| d.id == "money.equipment_attach_modifier.insufficient_funds"),
                    "must carry the real insufficient-funds diagnostic: {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("an unaffordable modifier must never be silently attached")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        let longsword = reloaded
            .character_input
            .chosen
            .equipment_selections
            .iter()
            .find(|selection| selection.item_id == "item:longsword")
            .expect("item:longsword must still be present");
        assert!(longsword.applied_modifiers.is_empty(), "nothing should have attached on the rejected path");

        std::fs::remove_dir_all(&root).ok();
    }

    /// A `modifier_item_id` that is not a real catalog item at all must be
    /// rejected before any cost/target check -- never silently attached.
    #[test]
    fn attach_equipment_modifier_at_root_rejects_an_unknown_modifier_item() {
        let root = tempdir("attach-modifier-unknown-item");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000_000).expect("funding should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "totally-fabricated-modifier-id",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "equipment.attach_modifier.unknown_item"),
                    "must carry the real unknown-item diagnostic: {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("a fabricated modifier id must never be silently attached")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            1_000_000,
            "nothing should have been charged when the modifier item itself is unrecognized"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A target `item_id` that does not exist among the character's
    /// `equipment_selections` must be `Blocked` before any charge -- a
    /// not-found target is never silently a free no-op after money moves.
    #[test]
    fn attach_equipment_modifier_at_root_rejects_a_target_that_does_not_exist() {
        let root = tempdir("attach-modifier-missing-target");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 1_000_000).expect("funding should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:greatsword",
            "Special Ability ~ +1 ~ Weapon",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                assert!(
                    diagnostics.iter().any(|d| d.id == "equipment.attach_modifier.target_not_found"),
                    "must carry the real target-not-found diagnostic: {diagnostics:?}"
                );
            }
            AttachEquipmentModifierResponse::Attached { .. } => {
                panic!("attaching to a nonexistent target must never silently succeed")
            }
        }

        assert_eq!(
            load_character_money_at_root(&root).unwrap().total_copper,
            1_000_000,
            "nothing should have been charged when the target selection does not exist"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// End-to-end proof that an attached-but-corpus-unresolvable modifier
    /// (the desktop app's bundled demo corpus has zero equipmods records)
    /// surfaces honestly through the real `corpusDerived` response rather
    /// than silently vanishing -- closes the loop this sub-task's
    /// `unresolved_equipment_item_ids` extension exists for.
    #[test]
    fn attach_equipment_modifier_at_root_surfaces_the_real_corpus_resolution_gap() {
        let root = tempdir("attach-modifier-corpus-gap");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = attach_equipment_modifier_at_root(
            &root,
            "item:longsword",
            "Special Ability ~ +1 ~ Weapon",
            "2026-07-23T00:00:00Z",
        )
        .expect("attach call should not error");

        match response {
            AttachEquipmentModifierResponse::Attached { corpus_derived, .. } => {
                assert!(
                    corpus_derived
                        .unresolved_equipment_item_ids
                        .contains(&"Special Ability ~ +1 ~ Weapon".to_string()),
                    "the bundled demo corpus has no equipmods records, so the attached modifier \
                     must be traceable as unresolved rather than silently inert: {corpus_derived:?}"
                );
            }
            AttachEquipmentModifierResponse::Blocked { diagnostics } => {
                panic!("the attach itself must succeed even though the modifier won't resolve, got Blocked: {diagnostics:?}")
            }
        }

        std::fs::remove_dir_all(&root).ok();
    }

    /// Same wire-shape proof as `purchase_equipment`'s own precedent test:
    /// the `corpusDerived` field must be camelCase (a per-field rename,
    /// not an enum-wide `rename_all`, which would also lowercase the
    /// `"Attached"`/`"Blocked"` tag values themselves).
    #[test]
    fn attach_equipment_modifier_response_attached_serializes_corpus_derived_as_camel_case_without_touching_the_tag()
    {
        let response = AttachEquipmentModifierResponse::Attached {
            summary: Box::new(CharacterSummaryDto {
                character_id: "c".to_owned(),
                display_label: "d".to_owned(),
                game_system: "pf1".to_owned(),
                schema_version: 1,
                saved_at: "2026-07-23T00:00:00Z".to_owned(),
                race_id: "race:human".to_owned(),
                class_summary: "class:fighter:1".to_owned(),
            }),
            snapshot: PilotSnapshotDto {
                ability_modifiers: AbilityModifiersDto {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                },
                base_attack_bonus: 0,
                base_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                selected_skill_modifiers: SelectedSkillModifiersDto {
                    climb: 0,
                    intimidate: 0,
                    swim: 0,
                },
                damage_reduction: None,
            },
            corpus_derived: CorpusDerivedDto {
                school_coverage: Vec::new(),
                equipped_items: Vec::new(),
                equipment_effects: EquipmentEffectsDto {
                    armor_class_delta: 0,
                    armor_check_penalty_total: 0,
                    max_dex_cap: None,
                    spell_failure_chance: None,
                    attack_bonus_delta: None,
                },
                unresolved_spell_ids: Vec::new(),
                unresolved_equipment_item_ids: Vec::new(),
            },
            money: money_dto_from_total(0),
        };

        let value = serde_json::to_value(&response).expect("response should serialize");
        let object = value.as_object().expect("response should serialize as a JSON object");

        assert_eq!(object.get("kind").and_then(|v| v.as_str()), Some("Attached"));
        assert!(object.contains_key("corpusDerived"), "{object:?}");
        assert!(!object.contains_key("corpus_derived"), "{object:?}");
    }

    // ----- `add_feat_selection` (v0.6 alpha swarm) -----

    #[test]
    fn apply_add_feat_selection_appends_to_selected_feats() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.selected_feats.len();

        apply_add_feat_selection(&mut input, "feat:toughness");

        assert_eq!(input.chosen.selected_feats.len(), starting_len + 1);
        assert_eq!(
            input.chosen.selected_feats.last(),
            Some(&"feat:toughness".to_owned())
        );
    }

    /// Mirrors `add_spell_selection_at_root_appends_and_persists_when_computed`'s
    /// golden-path shape below.
    #[test]
    fn add_feat_selection_at_root_appends_and_persists_when_computed() {
        let root = tempdir("add-feat-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.selected_feats.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_feat_selection_at_root(&root, "feat:toughness", "2026-07-21T00:00:00Z")
            .expect("add feat selection call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with an added feat selection must still reach \
                     Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.character_input.chosen.selected_feats.len(), starting_len + 1);
        assert_eq!(
            reloaded.character_input.chosen.selected_feats.last(),
            Some(&"feat:toughness".to_owned())
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn add_feat_selection_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("add-feat-missing-character");

        let result = add_feat_selection_at_root(&root, "feat:toughness", "2026-07-21T00:00:00Z");

        assert!(
            result.is_err(),
            "adding a feat selection to a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `add_spell_selection` (Criterion 18) -----

    #[test]
    fn apply_add_spell_selection_appends_to_spells_selected() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.spells_selected.len();

        apply_add_spell_selection(
            &mut input,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
        );

        assert_eq!(input.chosen.spells_selected.len(), starting_len + 1);
        let added = input
            .chosen
            .spells_selected
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.spell_id, "Mage Armor");
        assert_eq!(added.source_class_id, "class:wizard");
        assert_eq!(added.acquisition_mode, AcquisitionMode::Known);
    }

    /// v0.6 alpha swarm item 3: the pure half of the Wizard spellbook
    /// bootstrap fix -- see `apply_record_and_prepare_spell_selection`'s
    /// own doc comment for the full deadlock analysis. The `_at_root`
    /// golden-path proof lives in `pf1_adapter.rs`'s own test module
    /// (that's where the deadlock reproduction against the real
    /// persistence layer lives too).
    #[test]
    fn apply_record_and_prepare_spell_selection_appends_both_known_and_prepared_entries() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        let starting_len = input.chosen.spells_selected.len();

        apply_record_and_prepare_spell_selection(&mut input, "evocation.0.light", "class:wizard");

        assert_eq!(input.chosen.spells_selected.len(), starting_len + 2);
        let added = &input.chosen.spells_selected[starting_len..];
        assert!(added.iter().all(|s| s.spell_id == "evocation.0.light" && s.source_class_id == "class:wizard"));
        assert!(added.iter().any(|s| s.acquisition_mode == AcquisitionMode::Known));
        assert!(added.iter().any(|s| s.acquisition_mode == AcquisitionMode::Prepared));
    }

    /// The single most important regression guard for Criterion 18's spell
    /// half: proves the real load -> mutate -> recompute -> re-save ->
    /// return round trip against a real `SavedCharacterStore` fixture on
    /// disk, mirroring `level_up_character_at_root`'s own golden path test.
    #[test]
    fn add_spell_selection_at_root_appends_and_persists_when_computed() {
        let root = tempdir("add-spell-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        let starting_len = envelope.character_input.chosen.spells_selected.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = add_spell_selection_at_root(
            &root,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
            "2026-07-21T00:00:00Z",
        )
        .expect("add spell selection call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with an added spell selection must still \
                     reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.spells_selected.len(),
            starting_len + 1,
            "the on-disk envelope must reflect the appended spell selection"
        );
        let added = reloaded
            .character_input
            .chosen
            .spells_selected
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.spell_id, "Mage Armor");
        assert_eq!(added.source_class_id, "class:wizard");
        assert_eq!(added.acquisition_mode, AcquisitionMode::Known);
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// `add_spell_selection_at_root`'s error path when there is nothing
    /// saved at `root` yet — must fail honestly rather than silently
    /// creating a character out of thin air.
    #[test]
    fn add_spell_selection_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("add-spell-missing-character");

        let result = add_spell_selection_at_root(
            &root,
            "Mage Armor",
            "class:wizard",
            AcquisitionMode::Known,
            "2026-07-21T00:00:00Z",
        );

        assert!(
            result.is_err(),
            "adding a spell selection to a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `set_skill_allocations` (v0.6 alpha swarm, task 2) -----

    #[test]
    fn apply_set_skill_allocations_replaces_skill_allocations_wholesale() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        assert_eq!(
            input.chosen.skill_allocations.len(),
            3,
            "the fixed demo loadout starts with three skill allocations"
        );

        apply_set_skill_allocations(
            &mut input,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 2 }],
        );

        assert_eq!(
            input.chosen.skill_allocations,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 2 }],
            "the whole set must be replaced, not appended to"
        );
    }

    /// The single most important regression guard for task 2: proves the
    /// real load -> mutate -> recompute -> re-save -> return round trip
    /// against a real `SavedCharacterStore` fixture on disk, mirroring
    /// `add_equipment_selection_at_root`'s own golden path test. Re-orders
    /// the supported Climb/Intimidate/Swim triple (rather than sending it
    /// back unchanged) so the assertion actually proves replacement, not a
    /// no-op.
    #[test]
    fn set_skill_allocations_at_root_replaces_and_persists_when_computed() {
        let root = tempdir("set-skill-allocations-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = set_skill_allocations_at_root(
            &root,
            vec![
                SkillAllocation { skill_id: "skill:swim".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:intimidate".to_owned(), ranks: 1 },
                SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 1 },
            ],
            "2026-07-21T00:00:00Z",
        )
        .expect("set skill allocations call should not error");

        match response {
            CreateCharacterResponse::Saved { .. } => {}
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 with a re-ordered but still-supported skill \
                     triple must still reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded
                .character_input
                .chosen
                .skill_allocations
                .iter()
                .map(|allocation| allocation.skill_id.as_str())
                .collect::<Vec<_>>(),
            vec!["skill:swim", "skill:intimidate", "skill:climb"],
            "the on-disk envelope must reflect the caller's full replacement set/order"
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Mirrors `level_up_character_at_root`'s own "never persist an unproven
    /// build" proof: an out-of-posture skill allocation must leave the
    /// on-disk envelope exactly as it was, not silently apply.
    #[test]
    fn set_skill_allocations_at_root_does_not_persist_when_resulting_build_is_blocked() {
        let root = tempdir("set-skill-allocations-blocked");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = set_skill_allocations_at_root(
            &root,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 2 }],
            "2026-07-21T00:00:00Z",
        )
        .expect("set skill allocations call should not error even when blocked");

        match response {
            CreateCharacterResponse::Blocked { .. } => {}
            CreateCharacterResponse::Saved { .. } => {
                panic!("rank 2 Climb alone is outside the supported posture and must not reach Computed")
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.skill_allocations,
            envelope.character_input.chosen.skill_allocations,
            "a blocked mutation must leave the on-disk skill allocations untouched"
        );
        assert_eq!(
            reloaded.saved_at, LEVEL_UP_TEST_SAVED_AT,
            "a blocked mutation must not advance saved_at either"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn set_skill_allocations_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("set-skill-allocations-missing-character");

        let result = set_skill_allocations_at_root(
            &root,
            vec![SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 1 }],
            "2026-07-21T00:00:00Z",
        );

        assert!(
            result.is_err(),
            "setting skill allocations on a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `update_character_bio` / `load_character_bio` (v0.6 alpha swarm) -----

    fn sample_bio() -> CharacterBioDto {
        CharacterBioDto {
            alignment: "Lawful Good".to_owned(),
            deity: "Iomedae".to_owned(),
            sex: "Female".to_owned(),
            age: "27".to_owned(),
            height: "5'8\"".to_owned(),
            weight: "150 lbs".to_owned(),
            hair: "Auburn".to_owned(),
            eyes: "Green".to_owned(),
        }
    }

    #[test]
    fn load_character_bio_at_root_returns_all_empty_default_when_no_bio_file_exists_yet() {
        let root = tempdir("bio-default");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let bio = load_character_bio_at_root(&root).expect("loading an absent bio should not error");

        assert_eq!(bio, CharacterBioDto::default());

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn update_character_bio_at_root_persists_and_round_trips_through_load() {
        let root = tempdir("bio-round-trip");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        save_character_bio_at_root(&root, &sample_bio()).expect("saving a bio should not error");
        let reloaded = load_character_bio_at_root(&root).expect("reloading the saved bio should not error");

        assert_eq!(reloaded, sample_bio());

        std::fs::remove_dir_all(&root).ok();
    }

    /// A second save fully replaces the first -- proves this is a real
    /// overwrite, not an append or a write-once no-op.
    #[test]
    fn update_character_bio_at_root_overwrites_a_previously_saved_bio() {
        let root = tempdir("bio-overwrite");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        save_character_bio_at_root(&root, &sample_bio()).expect("first save should not error");
        let updated = CharacterBioDto { alignment: "Chaotic Neutral".to_owned(), ..sample_bio() };
        save_character_bio_at_root(&root, &updated).expect("second save should not error");

        let reloaded = load_character_bio_at_root(&root).expect("reload should not error");
        assert_eq!(reloaded, updated);

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn update_character_bio_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("bio-missing-character");

        let result = save_character_bio_at_root(&root, &sample_bio());

        assert!(result.is_err(), "saving a bio for a nonexistent saved character must fail");

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `load_character_money` / `adjust_character_money` (v0.6 alpha swarm) -----

    #[test]
    fn load_character_money_at_root_returns_a_zero_balance_when_no_money_file_exists_yet() {
        let root = tempdir("money-default");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let money = load_character_money_at_root(&root).expect("loading absent money should not error");

        assert_eq!(
            money,
            CharacterMoneyDto { total_copper: 0, platinum: 0, gold: 0, silver: 0, copper: 0 }
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_money_at_root_adds_funds_and_persists_the_new_total() {
        let root = tempdir("money-add");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let money = adjust_character_money_at_root(&root, 1234).expect("adding funds should not error");

        assert_eq!(
            money,
            CharacterMoneyDto { total_copper: 1234, platinum: 1, gold: 2, silver: 3, copper: 4 }
        );
        let reloaded = load_character_money_at_root(&root).expect("reload should not error");
        assert_eq!(reloaded, money, "the balance must be persisted, not just returned in-memory");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Two consecutive adjustments accumulate against the persisted total,
    /// not each other's in-memory return value -- proves the balance is
    /// genuinely read-modify-write, not overwritten from a stale snapshot.
    #[test]
    fn adjust_character_money_at_root_accumulates_across_repeated_calls() {
        let root = tempdir("money-accumulate");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        adjust_character_money_at_root(&root, 1000).expect("first adjustment should not error");
        let after_second =
            adjust_character_money_at_root(&root, -300).expect("second adjustment should not error");

        assert_eq!(after_second.total_copper, 700);

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_money_at_root_rejects_spending_more_than_the_current_balance() {
        let root = tempdir("money-insufficient-funds");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        adjust_character_money_at_root(&root, 100).expect("seed funds should not error");

        let result = adjust_character_money_at_root(&root, -200);

        assert!(result.is_err(), "spending more than the current balance must fail honestly");
        let reloaded = load_character_money_at_root(&root).expect("reload should not error");
        assert_eq!(
            reloaded.total_copper, 100,
            "a rejected spend must not partially apply or corrupt the persisted balance"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_money_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("money-missing-character");

        let result = adjust_character_money_at_root(&root, 100);

        assert!(result.is_err(), "adjusting money for a nonexistent saved character must fail");

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `load_character_durability` / `adjust_character_hp` (v0.6 alpha swarm) -----

    #[test]
    fn load_character_durability_at_root_defaults_current_hp_to_computed_max_hp() {
        let root = tempdir("hp-default");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let durability =
            load_character_durability_at_root(&root).expect("loading durability should not error");

        // Fighter d10 level 1, CON mod +2 (score 14, unaffected by the fixture's
        // Human ability-bonus target of strength): 10 + 2 = 12.
        assert_eq!(durability.max_hp, 12);
        assert_eq!(durability.current_hp, 12);
        assert_eq!(durability.nonlethal_damage, 0);
        assert_eq!(durability.status, "Normal");

        std::fs::remove_dir_all(&root).ok();
    }

    /// v0.6 alpha swarm item 17: the feat-effects engine's first real,
    /// wired case. A Fighter with Toughness selected must show the correct
    /// 15 HP (12 base + Toughness's real +3), not the pre-fix 12 QA found
    /// (zero explanations mentioning the feat at all).
    #[test]
    fn load_character_durability_at_root_includes_toughnesss_real_plus_three_hp() {
        let root = tempdir("hp-toughness");
        let mut envelope = level_up_test_envelope("race:human", 1);
        envelope.character_input.chosen.selected_feats.push("Toughness".to_owned());
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let durability =
            load_character_durability_at_root(&root).expect("loading durability should not error");

        assert_eq!(durability.max_hp, 15, "12 base + Toughness's real flat +3 = 15");
        assert_eq!(
            durability.current_hp, 15,
            "current HP must default to the feat-inclusive max, not the pre-feat base"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_persists_damage_and_clamps_healing_at_max_hp() {
        let root = tempdir("hp-damage-heal");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let after_damage = adjust_character_hp_at_root(&root, -5, 0).expect("damage should not error");
        assert_eq!(after_damage.current_hp, 7);

        let after_overheal =
            adjust_character_hp_at_root(&root, 100, 0).expect("healing should not error");
        assert_eq!(after_overheal.current_hp, 12, "healing must clamp at max_hp, not exceed it");

        let reloaded =
            load_character_durability_at_root(&root).expect("reload should not error");
        assert_eq!(reloaded.current_hp, 12, "the healed total must be persisted");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_tracks_nonlethal_damage_and_reflects_status() {
        let root = tempdir("hp-nonlethal");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let staggered =
            adjust_character_hp_at_root(&root, 0, 12).expect("nonlethal damage should not error");
        assert_eq!(
            staggered.status, "Staggered",
            "nonlethal damage exactly equal to current HP (12) is staggered"
        );

        let unconscious =
            adjust_character_hp_at_root(&root, 0, 1).expect("more nonlethal damage should not error");
        assert_eq!(unconscious.nonlethal_damage, 13);
        assert_eq!(unconscious.status, "Unconscious");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_reflects_dying_and_dead_thresholds() {
        let root = tempdir("hp-dying-dead");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let dying = adjust_character_hp_at_root(&root, -13, 0).expect("lethal damage should not error");
        assert_eq!(dying.current_hp, -1);
        assert_eq!(dying.status, "Dying");

        // -1 - 13 = -14, which equals -constitution_score (14) -> Dead.
        let dead = adjust_character_hp_at_root(&root, -13, 0).expect("further lethal damage should not error");
        assert_eq!(dead.current_hp, -14);
        assert_eq!(dead.status, "Dead");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn adjust_character_hp_at_root_floors_nonlethal_recovery_at_zero() {
        let root = tempdir("hp-nonlethal-floor");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        adjust_character_hp_at_root(&root, 0, 3).expect("nonlethal damage should not error");
        let recovered =
            adjust_character_hp_at_root(&root, 0, -100).expect("nonlethal recovery should not error");

        assert_eq!(recovered.nonlethal_damage, 0, "nonlethal damage must floor at 0, not go negative");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn load_character_durability_at_root_fails_honestly_for_a_multiclass_build() {
        let root = tempdir("hp-multiclass");
        let mut envelope = level_up_test_envelope("race:human", 2);
        envelope.character_input.chosen.class_levels.push(CharacterClassLevel {
            class_id: "class:rogue".to_owned(),
            level: 1,
        });
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let result = load_character_durability_at_root(&root);

        assert!(
            result.is_err(),
            "durability for a multiclass build must fail honestly, not fabricate a value"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn load_character_durability_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("hp-missing-character");

        let result = load_character_durability_at_root(&root);

        assert!(
            result.is_err(),
            "loading durability for a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- Render-staleness real root cause (v0.6 alpha swarm) -----
    //
    // Frontend traced the "Spells/Gear tab looks stale right after a
    // mutation" bug to a real serde defect, not a React timing issue:
    // CreateCharacterResponse's `Saved` variant serialized `corpus_derived`
    // literally as `"corpus_derived"` on the wire (no `#[serde(tag =
    // "kind")]`-level `rename_all`, since that would also camelCase the
    // `"Saved"`/`"Blocked"` tag strings the frontend matches on) while the
    // TS boundary's asserted-not-validated type declared `corpusDerived` --
    // silently `undefined` at runtime, invisible to any Rust-side test that
    // only checks struct field values rather than the actual JSON shape.
    // These tests serialize a real response value and assert on the JSON
    // keys directly -- the only way this exact bug class is actually
    // caught.

    #[test]
    fn create_character_response_saved_serializes_corpus_derived_as_camel_case_without_touching_the_tag(
    ) {
        let response = CreateCharacterResponse::Saved {
            summary: Box::new(CharacterSummaryDto {
                character_id: "c".to_owned(),
                display_label: "d".to_owned(),
                game_system: "pf1".to_owned(),
                schema_version: 1,
                saved_at: "2026-07-23T00:00:00Z".to_owned(),
                race_id: "race:human".to_owned(),
                class_summary: "class:fighter:1".to_owned(),
            }),
            snapshot: PilotSnapshotDto {
                ability_modifiers: AbilityModifiersDto {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                },
                base_attack_bonus: 0,
                base_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                selected_skill_modifiers: SelectedSkillModifiersDto {
                    climb: 0,
                    intimidate: 0,
                    swim: 0,
                },
                damage_reduction: None,
            },
            corpus_derived: CorpusDerivedDto {
                school_coverage: Vec::new(),
                equipped_items: Vec::new(),
                equipment_effects: EquipmentEffectsDto {
                    armor_class_delta: 0,
                    armor_check_penalty_total: 0,
                    max_dex_cap: None,
                    spell_failure_chance: None,
                    attack_bonus_delta: None,
                },
                unresolved_spell_ids: Vec::new(),
                unresolved_equipment_item_ids: Vec::new(),
            },
        };

        let value = serde_json::to_value(&response).expect("response should serialize");
        let object = value.as_object().expect("response should serialize as a JSON object");

        assert_eq!(
            object.get("kind").and_then(|v| v.as_str()),
            Some("Saved"),
            "the kind tag must stay exactly \"Saved\" (PascalCase, untouched) so \
             outcome.kind === 'Saved' checks on the frontend keep working: {object:?}"
        );
        assert!(
            object.contains_key("corpusDerived"),
            "corpus_derived must serialize as camelCase corpusDerived on the wire: {object:?}"
        );
        assert!(
            !object.contains_key("corpus_derived"),
            "corpus_derived must NOT also/still appear as snake_case on the wire: {object:?}"
        );
    }

    /// v0.6 alpha swarm (QA finding, 2026-07-24): `EquipmentEffectsDto`'s
    /// three `Option` fields must genuinely omit their key when `None`
    /// (mirroring `PilotSnapshotDto.damage_reduction`'s own precedent), not
    /// serialize as `"maxDexCap":null` -- a present key with a literal
    /// `null` value defeats the frontend's `!== undefined` hide-checks
    /// (`null !== undefined` is `true`), rendering garbled `"+null"`/
    /// `"null%"` strings instead of hiding the field. Only one character
    /// build (Chain Shirt + exactly one weapon) had been live-verified
    /// before this bug was found, and it happened to resolve `Some(...)`
    /// for all three fields -- any zero- or two-weapon build (a fresh
    /// character, most Wizards, a dual-wielder) would have hit this.
    #[test]
    fn equipment_effects_dto_omits_its_optional_fields_when_none_and_includes_them_when_some() {
        let with_none = EquipmentEffectsDto {
            armor_class_delta: 4,
            armor_check_penalty_total: -2,
            max_dex_cap: None,
            spell_failure_chance: None,
            attack_bonus_delta: None,
        };
        let json = serde_json::to_string(&with_none).expect("serialization should succeed");
        assert!(
            !json.contains("maxDexCap") && !json.contains("spellFailureChance") && !json.contains("attackBonusDelta"),
            "None fields must omit their key entirely, not serialize as null: {json}"
        );
        // The two non-Option fields are unaffected by this fix -- always present.
        assert!(json.contains("\"armorClassDelta\":4"));
        assert!(json.contains("\"armorCheckPenaltyTotal\":-2"));

        let with_some = EquipmentEffectsDto {
            armor_class_delta: 4,
            armor_check_penalty_total: -2,
            max_dex_cap: Some(4),
            spell_failure_chance: Some(20.0),
            attack_bonus_delta: Some(1),
        };
        let json = serde_json::to_string(&with_some).expect("serialization should succeed");
        assert!(json.contains("\"maxDexCap\":4"));
        assert!(json.contains("\"spellFailureChance\":20.0"));
        assert!(json.contains("\"attackBonusDelta\":1"));
    }

    #[test]
    fn purchase_equipment_response_purchased_serializes_corpus_derived_as_camel_case_without_touching_the_tag(
    ) {
        let response = PurchaseEquipmentResponse::Purchased {
            summary: Box::new(CharacterSummaryDto {
                character_id: "c".to_owned(),
                display_label: "d".to_owned(),
                game_system: "pf1".to_owned(),
                schema_version: 1,
                saved_at: "2026-07-23T00:00:00Z".to_owned(),
                race_id: "race:human".to_owned(),
                class_summary: "class:fighter:1".to_owned(),
            }),
            snapshot: PilotSnapshotDto {
                ability_modifiers: AbilityModifiersDto {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                },
                base_attack_bonus: 0,
                base_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                baseline_melee_attack_bonus: 0,
                baseline_armor_class: 0,
                total_saves: BaseSavesDto { fortitude: 0, reflex: 0, will: 0 },
                selected_skill_modifiers: SelectedSkillModifiersDto {
                    climb: 0,
                    intimidate: 0,
                    swim: 0,
                },
                damage_reduction: None,
            },
            corpus_derived: CorpusDerivedDto {
                school_coverage: Vec::new(),
                equipped_items: Vec::new(),
                equipment_effects: EquipmentEffectsDto {
                    armor_class_delta: 0,
                    armor_check_penalty_total: 0,
                    max_dex_cap: None,
                    spell_failure_chance: None,
                    attack_bonus_delta: None,
                },
                unresolved_spell_ids: Vec::new(),
                unresolved_equipment_item_ids: Vec::new(),
            },
            money: money_dto_from_total(0),
        };

        let value = serde_json::to_value(&response).expect("response should serialize");
        let object = value.as_object().expect("response should serialize as a JSON object");

        assert_eq!(object.get("kind").and_then(|v| v.as_str()), Some("Purchased"));
        assert!(object.contains_key("corpusDerived"), "{object:?}");
        assert!(!object.contains_key("corpus_derived"), "{object:?}");
    }

    // ----- `level_up_character` (Criterion 17) -----

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-character-hub-level-up-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    const LEVEL_UP_TEST_SAVED_AT: &str = "2026-07-08T00:00:00Z";

    fn level_up_test_envelope(race_id: &str, level: u8) -> SavedCharacterEnvelope {
        let character_input = compose_character_input(&request_for(race_id, level));
        SavedCharacterEnvelope {
            character_id: "char-level-up-test".to_owned(),
            revision_id: "char-level-up-test.rev.1".to_owned(),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: LEVEL_UP_TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: "char-level-up-test.rev.1".to_owned(),
            display_label: "Level Up Test Character".to_owned(),
            character_input,
        }
    }

    #[test]
    fn apply_level_up_increments_existing_class_level() {
        let mut input = compose_character_input(&request_for("race:human", 1));

        apply_level_up(&mut input, FIGHTER_CLASS_ID);

        assert_eq!(input.chosen.class_levels.len(), 1);
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        assert_eq!(input.chosen.class_levels[0].level, 2);
    }

    #[test]
    fn apply_level_up_adds_a_new_class_level_entry_for_an_unheld_class() {
        let mut input = compose_character_input(&request_for("race:human", 1));

        apply_level_up(&mut input, "class:wizard");

        assert_eq!(input.chosen.class_levels.len(), 2);
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);
        assert_eq!(
            input.chosen.class_levels[0].level, 1,
            "the existing class level must be untouched"
        );
        assert_eq!(input.chosen.class_levels[1].class_id, "class:wizard");
        assert_eq!(input.chosen.class_levels[1].level, 1);
    }

    /// v0.6 alpha swarm: `compose_character_input`'s Wizard school-choice
    /// seeding fix only covered fresh creation -- multiclassing Wizard onto
    /// an existing character via `apply_level_up`'s new-class-entry branch
    /// hit the same unconditional spellbook-posture block, unchanged.
    /// Frontend verified this live before this fix landed.
    #[test]
    fn apply_level_up_seeds_the_canonical_wizard_school_choices_when_dipping_into_wizard() {
        let mut input = compose_character_input(&request_for("race:human", 1));

        apply_level_up(&mut input, "class:wizard");

        assert!(
            input.chosen.selected_choices.iter().any(|c| c.choice_set_id
                == "choice:wizard_school_specialization"
                && c.selection_id == "school:evocation"),
            "multiclassing into Wizard must seed the canonical Evocation specialization: {:?}",
            input.chosen.selected_choices
        );
        let opposed: Vec<&str> = input
            .chosen
            .selected_choices
            .iter()
            .filter(|c| c.choice_set_id == "choice:wizard_opposed_schools")
            .map(|c| c.selection_id.as_str())
            .collect();
        assert_eq!(opposed.len(), 2, "must seed exactly two opposed schools: {opposed:?}");
        assert!(opposed.contains(&"school:necromancy"));
        assert!(opposed.contains(&"school:transmutation"));
    }

    /// A second consecutive level-up within Wizard (not a fresh dip) must
    /// not re-seed the choices -- `wizard_has_canonical_specialization_selections`
    /// requires *exactly* two opposed-school entries, so a duplicate pair
    /// would silently break it.
    #[test]
    fn apply_level_up_does_not_reseed_wizard_choices_on_a_second_level_up_within_wizard() {
        let mut input = compose_character_input(&request_for("race:human", 1));
        apply_level_up(&mut input, "class:wizard");
        apply_level_up(&mut input, "class:wizard");

        assert_eq!(input.chosen.class_levels[1].level, 2, "the second call must increment, not re-dip");
        let opposed_count = input
            .chosen
            .selected_choices
            .iter()
            .filter(|c| c.choice_set_id == "choice:wizard_opposed_schools")
            .count();
        assert_eq!(
            opposed_count, 2,
            "a second level-up within Wizard must not duplicate the seeded opposed schools"
        );
    }

    /// The single most important regression guard for Criterion 17: proves
    /// the real load -> mutate -> recompute -> re-save -> return round trip
    /// against a real `SavedCharacterStore` fixture on disk, not a mock.
    #[test]
    fn level_up_character_at_root_increments_level_and_persists_when_computed() {
        let root = tempdir("golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T00:00:00Z",
        )
        .expect("level up call should not error");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.class_summary, "class:fighter:2");
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "Human Fighter level 1 -> 2 must reach Computed, got diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.character_input.chosen.class_levels.len(), 1);
        assert_eq!(
            reloaded.character_input.chosen.class_levels[0].level, 2,
            "the on-disk envelope must reflect the leveled-up build"
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Mirrors `create_character`/`clone_character`'s "never persist an
    /// unproven build" invariant: leveling past the engine's currently
    /// supported range must return `Blocked` and must not touch the file
    /// on disk at all.
    #[test]
    fn level_up_character_at_root_does_not_persist_when_leveled_up_build_is_blocked() {
        let root = tempdir("blocked-path");
        // Level 20 is the top of the compute engine's supported Fighter range
        // (`MAX_SUPPORTED_FIGHTER_LEVEL` in `pilot_compute.rs`); leveling to 21 must
        // fall back to Blocked.
        let envelope = level_up_test_envelope("race:human", 20);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T00:00:00Z",
        )
        .expect("level up call should not error even when the build is Blocked");

        match response {
            CreateCharacterResponse::Blocked { diagnostics } => {
                assert!(
                    !diagnostics.is_empty(),
                    "a Blocked response should carry real diagnostics"
                );
            }
            CreateCharacterResponse::Saved { .. } => {
                panic!(
                    "Human Fighter level 20 -> 21 is outside the compute engine's supported \
                     range and must not reach Computed"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.class_levels[0].level, 20,
            "a Blocked leveled-up build must never be persisted"
        );
        assert_eq!(
            reloaded.saved_at, LEVEL_UP_TEST_SAVED_AT,
            "saved_at must be unchanged when the mutation is not persisted"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// `level_up_character_at_root`'s error path when there is nothing saved
    /// at `root` yet — must fail honestly (via `SavedCharacterStore::load`'s
    /// own error) rather than silently creating a character out of thin air.
    #[test]
    fn level_up_character_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("missing-character");

        let result = level_up_character_at_root(
            &root,
            FIGHTER_CLASS_ID,
            Vec::new(),
            None,
            "2026-07-21T00:00:00Z",
        );

        assert!(result.is_err(), "leveling up a nonexistent saved character must fail");

        std::fs::remove_dir_all(&root).ok();
    }

    // ----- `delete_character` (Criterion 22) -----

    #[test]
    fn delete_character_at_root_removes_the_directory_for_a_saved_character() {
        let root = tempdir("delete-golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");
        assert!(root.exists(), "precondition: the character root must exist");

        let response = delete_character_at_root(&root);

        assert!(response.ok, "delete should report ok: true, got error: {:?}", response.error);
        assert!(response.error.is_none());
        assert!(!root.exists(), "the character root must be gone after delete");
    }

    /// Builds a path guaranteed not to exist yet (unlike `tempdir`, which
    /// pre-creates its directory) — mirrors the root `codex` crate's own
    /// `list_all_returns_empty_listing_for_nonexistent_root` idiom.
    fn nonexistent_path(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "codex-character-hub-{label}-{}-{unique}",
            std::process::id()
        ))
    }

    #[test]
    fn delete_character_at_root_is_ok_when_nothing_is_saved_yet() {
        let root = nonexistent_path("delete-missing-character");
        assert!(!root.exists(), "precondition: nothing saved at this root");

        let response = delete_character_at_root(&root);

        assert!(
            response.ok,
            "deleting an already-nonexistent character must be an idempotent success, got error: {:?}",
            response.error
        );
        assert!(response.error.is_none());
    }

    // ----- `import_character` (Criterion 23) -----

    /// A known fixture shaped like a full saved-character export (extra
    /// envelope fields `characterId`/`revisionId`/`savedAt` included, to
    /// prove they are tolerated and ignored — importing always mints a
    /// fresh identity). The `characterInput` payload mirrors
    /// `compose_character_input`'s own Human Fighter level 1 golden-path
    /// shape, which the existing
    /// `compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3`
    /// test already proves reaches `Computed`.
    const HUMAN_FIGHTER_IMPORT_FIXTURE_JSON: &str = r#"{
        "characterId": "source-character-id-should-be-ignored",
        "revisionId": "source-character-id-should-be-ignored.rev.1",
        "savedAt": "2020-01-01T00:00:00Z",
        "displayLabel": "Imported Aldric",
        "characterInput": {
            "sourcePackageId": "pf1.core_rulebook",
            "chosen": {
                "raceId": "race:human",
                "classLevels": [{"classId": "class:fighter", "level": 1}],
                "abilityScores": {
                    "strength": 16,
                    "dexterity": 14,
                    "constitution": 14,
                    "intelligence": 10,
                    "wisdom": 12,
                    "charisma": 8
                },
                "selectedFeats": ["feat:power_attack", "feat:dodge", "feat:weapon_focus"],
                "skillAllocations": [
                    {"skillId": "skill:climb", "ranks": 1},
                    {"skillId": "skill:intimidate", "ranks": 1},
                    {"skillId": "skill:swim", "ranks": 1}
                ],
                "equipmentSelections": [
                    {"itemId": "item:longsword", "activeState": "EquippedActive"},
                    {"itemId": "item:chain_shirt", "activeState": "EquippedActive"},
                    {"itemId": "item:shield", "activeState": "Absent"},
                    {"itemId": "power_attack", "activeState": "SelectedInactive"}
                ],
                "selectedChoices": [
                    {"choiceSetId": "choice:level_1_character_feat", "selectionId": "feat:power_attack"},
                    {"choiceSetId": "choice:fighter_bonus_feat", "selectionId": "feat:weapon_focus:weapon:longsword"},
                    {"choiceSetId": "choice:human_bonus_feat", "selectionId": "feat:dodge"},
                    {"choiceSetId": "choice:human_ability_bonus", "selectionId": "ability:strength"}
                ],
                "spellsSelected": [
                    {"spellId": "Alarm", "sourceClassId": "class:demo", "acquisitionMode": "Granted"},
                    {"spellId": "Blur", "sourceClassId": "class:demo", "acquisitionMode": "Granted"}
                ]
            }
        }
    }"#;

    /// The single most important regression guard for Criterion 23: proves
    /// the real parse -> mint-fresh-id -> recompute -> save -> return round
    /// trip against a real `SavedCharacterStore` fixture on disk, not a
    /// mock. Also proves the source JSON's own `characterId` is ignored in
    /// favor of a freshly minted one.
    #[test]
    fn import_character_from_json_saves_a_fresh_character_when_computed() {
        let root = tempdir("import-golden-path");
        let fresh_id = "char-import-fresh-id";

        let response = import_character_from_json(
            HUMAN_FIGHTER_IMPORT_FIXTURE_JSON,
            &root,
            fresh_id,
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        )
        .expect("import call should not error for well-formed JSON");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.character_id, fresh_id);
                assert_eq!(summary.display_label, "Imported Aldric");
                assert_eq!(summary.class_summary, "class:fighter:1");
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!(
                    "the Human Fighter level 1 import fixture must reach Computed, got \
                     diagnostics: {diagnostics:?}"
                );
            }
        }

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_id, fresh_id,
            "the saved envelope must carry the freshly minted id, not the source JSON's own \
             (ignored) characterId"
        );
        assert_ne!(
            reloaded.character_id, "source-character-id-should-be-ignored",
            "the source JSON's own characterId must never be trusted"
        );
        assert_eq!(reloaded.display_label, "Imported Aldric");
        assert_eq!(reloaded.character_input.chosen.race_id, "race:human");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            4,
            "the full equipment_selections payload must round-trip"
        );
        assert_eq!(
            reloaded.character_input.chosen.spells_selected.len(),
            2,
            "the full spells_selected payload must round-trip"
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T00:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Malformed JSON is a distinct failure mode from a structurally valid
    /// but non-computing import: it must be a real `Err`, not a silently
    /// empty/successful import and not a `Blocked` response (which implies
    /// the JSON was at least well-formed enough to reach the compute
    /// engine).
    #[test]
    fn import_character_from_json_rejects_malformed_json() {
        let root = nonexistent_path("import-malformed-json");

        let result = import_character_from_json(
            "{ this is not valid json",
            &root,
            "char-import-malformed",
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        );

        assert!(result.is_err(), "malformed JSON must be rejected as a real error");
        assert!(
            !root.exists(),
            "no character directory may be created for a malformed import"
        );
    }

    /// JSON that parses but is missing a required `CharacterInput` field
    /// (schema-invalid, not merely malformed) must also be rejected, not
    /// half-imported with a default/garbage value.
    #[test]
    fn import_character_from_json_rejects_json_missing_a_required_field() {
        let root = nonexistent_path("import-missing-field");
        // Missing "chosen" entirely, and "sourcePackageId" too.
        let incomplete_json = r#"{"displayLabel": "Incomplete", "characterInput": {}}"#;

        let result = import_character_from_json(
            incomplete_json,
            &root,
            "char-import-incomplete",
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        );

        assert!(
            result.is_err(),
            "JSON missing required CharacterInput fields must be rejected"
        );
        assert!(!root.exists());
    }

    /// Mirrors `create_character`/`clone_character`/`level_up_character_at_root`'s
    /// "never persist an unproven build" invariant: a structurally valid
    /// import that the compute engine cannot reach `Computed` for must
    /// return `Blocked` with real diagnostics and must not write anything to
    /// disk.
    #[test]
    fn import_character_from_json_returns_blocked_without_saving_when_the_import_does_not_compute() {
        let root = nonexistent_path("import-blocked-path");
        // A Paladin build is outside the compute engine's currently
        // supported set (see claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class
        // above), so this must come back Blocked, not Saved.
        let unsupported_class_json = r#"{
            "displayLabel": "Unsupported Import",
            "characterInput": {
                "sourcePackageId": "pf1.core_rulebook",
                "chosen": {
                    "raceId": "race:human",
                    "classLevels": [{"classId": "class:paladin", "level": 1}],
                    "abilityScores": {
                        "strength": 16, "dexterity": 14, "constitution": 14,
                        "intelligence": 10, "wisdom": 12, "charisma": 8
                    },
                    "selectedFeats": [],
                    "skillAllocations": [],
                    "equipmentSelections": [],
                    "selectedChoices": [],
                    "spellsSelected": []
                }
            }
        }"#;

        let response = import_character_from_json(
            unsupported_class_json,
            &root,
            "char-import-blocked",
            "2026-07-21T00:00:00Z",
            "codex-dev-test",
        )
        .expect("import call should not error even when the build is Blocked");

        match response {
            CreateCharacterResponse::Blocked { diagnostics } => {
                assert!(!diagnostics.is_empty(), "a Blocked response should carry real diagnostics");
            }
            CreateCharacterResponse::Saved { .. } => {
                panic!("a Human Paladin level 1 import is outside supported range and must not reach Computed");
            }
        }

        assert!(!root.exists(), "a Blocked import must never be persisted");
    }

    // ----- `export_character` (Criterion 24) -----

    /// The single most important regression guard for Criterion 24: proves
    /// the real save -> export -> re-import round trip against two real
    /// `SavedCharacterStore` fixtures on disk, not mocks or a hand-built
    /// `{summary, detail}` payload. `level_up_test_envelope`'s Human Fighter
    /// build carries a non-trivial `chosen` (feats, skills, all four
    /// equipment active-state variants, selected choices, and spells), so
    /// this exercises every field `character_input_to_dto`/
    /// `character_input_from_dto` touch, not just the trivial ones.
    #[test]
    fn export_character_to_json_round_trips_through_import_character_from_json() {
        let source_root = tempdir("export-round-trip-source");
        let source_envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&source_envelope, &source_root).expect("seed save should succeed");

        let exported_json = export_character_to_json(&source_root).expect("export should succeed for a real saved character");

        // The exported file is exactly what `import_character` expects:
        // parsing it back out and rebuilding a `CharacterInput` (mirroring
        // what `import_character_from_json` does internally) must produce a
        // `CharacterInput` equal to the source in every field the DTO
        // carries — proving the export DTO and import DTO are structurally
        // compatible by construction, not merely by convention.
        let reparsed: ImportedCharacterFileDto =
            serde_json::from_str(&exported_json).expect("exported JSON must itself be valid ImportedCharacterFileDto JSON");
        assert_eq!(reparsed.display_label, source_envelope.display_label);
        let rebuilt_input = character_input_from_dto(reparsed.character_input, "irrelevant-for-this-comparison");
        assert_eq!(
            rebuilt_input.source_package_id,
            source_envelope.character_input.source_package_id
        );
        assert_eq!(rebuilt_input.chosen, source_envelope.character_input.chosen, "the full `chosen` payload — race, class levels, ability scores, feats, skills, equipment, choices, and spells — must round-trip through export -> import byte-for-byte");

        // Now prove the real end-to-end command path: importing the exported
        // JSON via `import_character_from_json` (the same function
        // `import_character` calls) reaches `Computed` and saves a fresh
        // character whose on-disk `chosen` matches the source.
        let dest_root = tempdir("export-round-trip-dest");
        let response = import_character_from_json(
            &exported_json,
            &dest_root,
            "char-export-round-trip-fresh-id",
            "2026-07-22T00:00:00Z",
            "codex-dev-test",
        )
        .expect("importing a real export must not error");

        match response {
            CreateCharacterResponse::Saved { summary, .. } => {
                assert_eq!(summary.character_id, "char-export-round-trip-fresh-id");
                assert_eq!(summary.display_label, source_envelope.display_label);
            }
            CreateCharacterResponse::Blocked { diagnostics } => {
                panic!("a real exported Human Fighter level 1 build must reach Computed on re-import, got diagnostics: {diagnostics:?}");
            }
        }

        let reimported = SavedCharacterStore::load(&dest_root).expect("reimported character should reload");
        assert_eq!(
            reimported.character_input.chosen, source_envelope.character_input.chosen,
            "the character reloaded after export -> import must carry the exact same chosen build as the original"
        );
        assert_ne!(
            reimported.character_id, source_envelope.character_id,
            "import always mints a fresh identity, never reuses the exported character's own id"
        );

        std::fs::remove_dir_all(&source_root).ok();
        std::fs::remove_dir_all(&dest_root).ok();
    }

    /// `export_character_to_json`'s error path when there is nothing saved
    /// at `root` yet — must fail honestly (via `SavedCharacterStore::load`'s
    /// own error) rather than fabricating an empty export.
    #[test]
    fn export_character_to_json_fails_honestly_when_nothing_is_saved_yet() {
        let root = nonexistent_path("export-missing-character");

        let result = export_character_to_json(&root);

        assert!(result.is_err(), "exporting a nonexistent saved character must fail");
    }
}
