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

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput,
    ChosenCharacterState, EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, HeadlessReceiptStatus};
use codex::rules_core::pilot_compute_corpus::{compute_pilot_with_corpus, CorpusDerivedSection};
use codex::rules_core::pilot_view_model::{PilotSnapshot, PilotViewModel};

use crate::sd19_corpus::corpus_fixture_bundle;
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::{
    SavedCharacterEnvelope, SavedCharacterRevisionKind, SavedCharacterSummary,
    CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
};

const HUMAN_RACE_ID: &str = "race:human";
const SOURCE_PACKAGE_ID: &str = "pf1.core_rulebook";
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorpusDerivedDto {
    pub school_coverage: Vec<SchoolCoverageDto>,
    pub equipped_items: Vec<ResolvedEquipmentDto>,
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
}

/// The `kind` tag stays PascalCase (`Saved` / `Blocked`) — no container-level
/// `rename_all` — matching the `Ge08BaselineArmorClass` precedent so the TS
/// boundary can match on those exact strings.
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
        corpus_derived: CorpusDerivedDto,
    },
    Blocked {
        diagnostics: Vec<DiagnosticDto>,
    },
}

// ----- Pure functions (unit-testable, no AppHandle / filesystem) -----

/// Build a `CharacterInput` for the requested race/class/level. Race, class,
/// and ability scores are the caller's real choices; the feat/skill/
/// equipment loadout is fixed — no class-specific diagnostic in the compute
/// seam reads those selections, so widening them would not change which
/// combinations reach `Computed`. Human additionally receives its own
/// canonical choice-slot values — the ability-bonus target is the caller's
/// real choice (`request.ability_bonus_target`); every other race omits the
/// Human-only slots.
pub fn compose_character_input(request: &CreateCharacterRequest) -> CharacterInput {
    let mut selected_choices = vec![
        SelectedChoice {
            choice_set_id: "choice:level_1_character_feat".to_owned(),
            selection_id: "feat:power_attack".to_owned(),
        },
        SelectedChoice {
            choice_set_id: "choice:fighter_bonus_feat".to_owned(),
            selection_id: "feat:weapon_focus:weapon:longsword".to_owned(),
        },
    ];

    if request.race_id == HUMAN_RACE_ID {
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:human_bonus_feat".to_owned(),
            selection_id: "feat:dodge".to_owned(),
        });
        selected_choices.push(SelectedChoice {
            choice_set_id: "choice:human_ability_bonus".to_owned(),
            selection_id: format!("ability:{}", request.ability_bonus_target),
        });
    }

    CharacterInput {
        case_id: Some(request.character_id.clone()),
        source_package_id: SOURCE_PACKAGE_ID.to_owned(),
        chosen: ChosenCharacterState {
            race_id: request.race_id.clone(),
            class_levels: vec![CharacterClassLevel {
                class_id: request.class_id.clone(),
                level: request.level,
            }],
            ability_scores: AbilityScores {
                strength: request.ability_scores.strength,
                dexterity: request.ability_scores.dexterity,
                constitution: request.ability_scores.constitution,
                intelligence: request.ability_scores.intelligence,
                wisdom: request.ability_scores.wisdom,
                charisma: request.ability_scores.charisma,
            },
            selected_feats: vec![
                "feat:power_attack".to_owned(),
                "feat:dodge".to_owned(),
                "feat:weapon_focus".to_owned(),
            ],
            skill_allocations: vec![
                SkillAllocation {
                    skill_id: "skill:climb".to_owned(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:intimidate".to_owned(),
                    ranks: 1,
                },
                SkillAllocation {
                    skill_id: "skill:swim".to_owned(),
                    ranks: 1,
                },
            ],
            equipment_selections: vec![
                EquipmentSelection {
                    item_id: "item:longsword".to_owned(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                },
                EquipmentSelection {
                    item_id: "item:chain_shirt".to_owned(),
                    equipped_or_active: true,
                    active_state: ActiveState::EquippedActive,
                },
                EquipmentSelection {
                    item_id: "item:shield".to_owned(),
                    equipped_or_active: false,
                    active_state: ActiveState::Absent,
                },
                EquipmentSelection {
                    item_id: "power_attack".to_owned(),
                    equipped_or_active: false,
                    active_state: ActiveState::SelectedInactive,
                },
            ],
            selected_choices,
            spells_selected: demo_spells_selected(),
        },
        selection_provenance: Vec::new(),
    }
}

/// Fixed SD-19 demo spell selections for the fixed loadout, mirroring the
/// existing fixed equipment loadout above. Only Human Fighter levels 1-3
/// reach `Computed` status today, so this loadout is necessarily a
/// Fighter's — these two spells are a reachability-demonstration sample
/// (proving `compute_pilot_with_corpus` resolves real corpus data end to
/// end in the live UI), not a claim that Fighters cast Abjuration/Illusion
/// spells. `source_class_id` is left generic (`"class:demo"`) for the same
/// reason: no class-appropriateness check consumes this field yet (see
/// `SpellSelection.source_class_id`'s own doc comment).
fn demo_spells_selected() -> Vec<SpellSelection> {
    vec![
        SpellSelection {
            spell_id: "Alarm".to_owned(),
            source_class_id: "class:demo".to_owned(),
            acquisition_mode: AcquisitionMode::Granted,
        },
        SpellSelection {
            spell_id: "Blur".to_owned(),
            source_class_id: "class:demo".to_owned(),
            acquisition_mode: AcquisitionMode::Granted,
        },
    ]
}

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

fn map_snapshot_dto(snapshot: &PilotSnapshot) -> PilotSnapshotDto {
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
    }
}

fn map_corpus_derived_dto(section: &CorpusDerivedSection) -> CorpusDerivedDto {
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
            .map(|item| ResolvedEquipmentDto {
                item_id: item.item_id.clone(),
                equipment_record_name: item.equipment_record_name.clone(),
                equipment_record_key: item.equipment_record_key.clone(),
                grounded: item.table_cell.is_some(),
            })
            .collect(),
    }
}

fn map_diagnostics_dto(
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

fn map_summary_dto(summary: &SavedCharacterSummary) -> CharacterSummaryDto {
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

fn summarize_envelope(envelope: &SavedCharacterEnvelope) -> CharacterSummaryDto {
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

// ----- Tauri commands -----

#[tauri::command]
pub fn create_character(
    app: tauri::AppHandle,
    request: CreateCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let character_input = compose_character_input(&request);
    let receipt = build_pilot_headless_receipt(&character_input);

    if receipt.status != HeadlessReceiptStatus::Computed {
        return Ok(CreateCharacterResponse::Blocked {
            diagnostics: map_diagnostics_dto(&receipt.computation.diagnostics),
        });
    }

    let view_model = PilotViewModel::from_receipt(&receipt);
    let snapshot = view_model
        .snapshot
        .as_ref()
        .expect("Computed status guarantees a snapshot");

    let corpus_receipt = compute_pilot_with_corpus(&character_input, corpus_fixture_bundle());

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

    let root = resolve_character_root(&app, &request.character_id)?;
    SavedCharacterStore::save(&envelope, &root).map_err(|err| err.message)?;

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(snapshot),
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

    let receipt = build_pilot_headless_receipt(&envelope.character_input);
    let view_model = PilotViewModel::from_receipt(&receipt);
    let corpus_receipt =
        compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle());

    Ok(LoadSavedCharacterResponse {
        summary: summarize_envelope(&envelope),
        snapshot: view_model.snapshot.as_ref().map(map_snapshot_dto),
        diagnostics: map_diagnostics_dto(&receipt.computation.diagnostics),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

fn resolve_characters_root(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))?;
    Ok(characters_root_from_app_data_dir(&app_data_dir))
}

fn resolve_character_root(app: &tauri::AppHandle, character_id: &str) -> Result<PathBuf, String> {
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

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:paladin", 1),
            generic_plus(&[
                "class_feature.hybrid.paladin.unsupported",
                "class_spell.hybrid.paladin.unsupported",
                "class_spell.paladin.partial_caster.unsupported",
            ])
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:ranger", 1),
            generic_plus(&[
                "class_feature.hybrid.ranger.unsupported",
                "class_spell.hybrid.ranger.unsupported",
            ])
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:barbarian", 1),
            generic_plus(&[
                "class_feature.barbarian.bounded_progression.rage_execution.unsupported",
            ])
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:sorcerer", 1),
            generic_plus(&[
                "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported",
                "class_spell.sorcerer.spontaneous.unsupported",
            ])
        );

        // SD-21 Epic 6 gave Wizard a real compute_wizard_chassis (BAB + saves, via
        // compute_class_chassis's per-class dispatch), so Wizard no longer trips the
        // two chassis-wide generic diagnostics (class_chassis.unsupported,
        // defense.total_save.unsupported) that every other still-unsupported class
        // does. Epic 6b's E6b.1 cycle then widened compute_combat_baseline and
        // compute_selected_skill_modifiers to the same has_supported_class_chassis
        // gate, so those two also no longer trip for Wizard at any input (not just the
        // Epic 6b reproducer's specific one) -- this request has no chosen spellbook,
        // so class_spell.wizard.prepared_spellbook.unsupported and
        // class_feature.wizard.school_powers_and_opposed_school_cost.unsupported
        // (Epic 6b's E6b.2/E6b.3) correctly remain: Epic 6b's Evocation-school
        // grounding only clears them for a genuinely populated, in-budget spellbook.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:wizard", 1),
            BTreeSet::from([
                "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported".to_string(),
                "class_spell.wizard.prepared_spellbook.unsupported".to_string(),
            ])
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:bard", 1),
            generic_plus(&[
                "class_feature.bard.bardic_performance_execution.unsupported",
                "class_spell.bard.spontaneous_known_and_per_day.unsupported",
            ])
        );

        // The SD13-E5 Rogue slice grounds trapfinding, the last named Rogue
        // pillar burden, so the named Rogue set is now empty: only the 4
        // generic chassis diagnostics remain (like the non-Human classes).
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:rogue", 1),
            generic_ids(),
            "Human Rogue L1 must carry only the 4 generic diagnostics after trapfinding grounding"
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:monk", 1),
            generic_plus(&["class_feature.monk.bounded_progression.bonus_feat.unsupported"])
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:cleric", 1),
            generic_plus(&[
                "class_feature.cleric.domain_powers.unsupported",
                "class_spell.cleric.prepared_divine.unsupported",
            ])
        );

        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:druid", 1),
            generic_plus(&[
                "class_feature.druid.animal_companion.unsupported",
                "class_spell.druid.prepared_divine.unsupported",
            ])
        );

        // Proves the Human-only gate: a non-Human race on a partially-supported
        // class collapses to the same 4 generic diagnostics as an unsupported class.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:dwarf", "class:paladin", 1),
            generic_ids(),
            "non-Human Paladin must not receive any named Paladin diagnostic"
        );

        // Proves the SD13-E5 level-2 widening: lay on hands, divine grace, smite evil,
        // and the effective-caster-level gate are all grounded at level 2, so only the
        // still-unproven spell burden diagnostic remains claim-blocking.
        assert_eq!(
            claim_blocking_diagnostic_ids("race:human", "class:paladin", 2),
            generic_plus(&["class_spell.paladin.partial_caster.unsupported"]),
            "Paladin level 2 chassis is grounded; only the spell burden stays claim-blocking"
        );
    }

    #[test]
    fn characters_root_from_app_data_dir_joins_characters_subdirectory() {
        let app_data_dir = Path::new("/tmp/example-app-data");
        let root = characters_root_from_app_data_dir(app_data_dir);
        assert_eq!(root, PathBuf::from("/tmp/example-app-data/characters"));
    }
}
