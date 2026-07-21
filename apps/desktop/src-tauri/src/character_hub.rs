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
/// This table documents the full three-operation surface. As of this cycle
/// all three rows are wired to callable `#[tauri::command]`s
/// (`level_up_character`, `add_equipment_selection`, `add_spell_selection`).
/// Per the Wired Integration doctrine (`governance/no-stub-mvp-doctrine.md`),
/// the `wired` flag below is descriptive metadata this table's own
/// dispatch-shape test asserts against, not a runtime dispatcher a caller
/// can reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedCharacterMutationOp {
    LevelUpCharacter,
    AddEquipmentSelection,
    AddSpellSelection,
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

pub const SAVED_CHARACTER_MUTATION_OPERATIONS: [SavedCharacterMutationOpDescriptor; 3] = [
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
];

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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelUpCharacterRequest {
    pub character_id: String,
    pub class_id: String,
    pub saved_at: String,
}

/// Increments `class_id`'s level by 1 on the given `CharacterInput`, or
/// adds a new level-1 entry for `class_id` if the character has none yet
/// (the multiclass "dip" case). Every other field is untouched.
pub fn apply_level_up(character_input: &mut CharacterInput, class_id: &str) {
    if let Some(class_level) = character_input
        .chosen
        .class_levels
        .iter_mut()
        .find(|class_level| class_level.class_id == class_id)
    {
        class_level.level = class_level.level.saturating_add(1);
    } else {
        character_input.chosen.class_levels.push(CharacterClassLevel {
            class_id: class_id.to_owned(),
            level: 1,
        });
    }
}

/// Shared load -> recompute -> re-save -> return-envelope tail for every
/// `mutate_saved_character` operation: applies `mutate` to the loaded
/// envelope's `CharacterInput`, recomputes via the real engine, and either
/// re-saves and returns `Saved` or leaves the on-disk envelope untouched and
/// returns `Blocked`. Never persists an unproven build — mirrors
/// `create_character`/`clone_character`'s own invariant. `mutate` receives
/// only the `CharacterInput`, so it cannot smuggle in a different `saved_at`
/// or bypass the recompute gate.
fn mutate_saved_character_at_root(
    root: &Path,
    saved_at: &str,
    mutate: impl FnOnce(&mut CharacterInput),
) -> Result<CreateCharacterResponse, String> {
    let mut envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    mutate(&mut envelope.character_input);

    let receipt = build_pilot_headless_receipt(&envelope.character_input);
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

    let corpus_receipt =
        compute_pilot_with_corpus(&envelope.character_input, corpus_fixture_bundle());

    envelope.saved_at = saved_at.to_owned();

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    Ok(CreateCharacterResponse::Saved {
        summary: Box::new(summarize_envelope(&envelope)),
        snapshot: map_snapshot_dto(snapshot),
        corpus_derived: map_corpus_derived_dto(&corpus_receipt.corpus_derived),
    })
}

/// `level_up_character`'s real implementation
/// (`SavedCharacterMutationOp::LevelUpCharacter` in the operation table
/// above): load -> mutate -> recompute -> re-save -> return envelope.
/// Split out from the `#[tauri::command]` wrapper below so it is
/// unit-testable against a real `SavedCharacterStore` fixture without an
/// `AppHandle` — the same "pure function under a thin command wrapper"
/// split this module already uses for `compose_character_input`.
///
/// Mirrors `create_character`/`clone_character`'s "never persist an
/// unproven build" invariant: if the leveled-up build does not reach
/// `Computed` (e.g. leveling past the range the compute engine currently
/// supports), the saved character on disk is left exactly as it was and
/// `Blocked` is returned with the real diagnostics — the mutation is never
/// silently applied on disk when the recompute fails.
fn level_up_character_at_root(
    root: &Path,
    class_id: &str,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_level_up(character_input, class_id);
    })
}

/// Loads the saved character, increments/adds the requested class's
/// level, recomputes via the real engine, and re-saves — see
/// `level_up_character_at_root` for the full semantics.
#[tauri::command]
pub fn level_up_character(
    app: tauri::AppHandle,
    request: LevelUpCharacterRequest,
) -> Result<CreateCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    level_up_character_at_root(&root, &request.class_id, &request.saved_at)
}

/// The wire-level projection of `ActiveState` for the `add_equipment_selection`
/// request. A separate DTO (rather than deriving `Deserialize` on
/// `ActiveState` itself) because the `codex` crate has zero dependencies —
/// see this module's own top-of-file note.
#[derive(Debug, Clone, Copy, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEquipmentSelectionRequest {
    pub character_id: String,
    pub item_id: String,
    pub active_state: ActiveStateDto,
    pub saved_at: String,
}

/// Appends one entry to `chosen.equipment_selections`. `equipped_or_active`
/// is derived from `active_state` (matching `EquipmentSelection`'s own doc
/// comment: the flag is a backward-compatible projection of the state, not
/// an independent choice) — true only for `ActiveState::EquippedActive`.
/// Every other field is untouched.
pub fn apply_add_equipment_selection(
    character_input: &mut CharacterInput,
    item_id: &str,
    active_state: ActiveState,
) {
    character_input.chosen.equipment_selections.push(EquipmentSelection {
        item_id: item_id.to_owned(),
        equipped_or_active: active_state == ActiveState::EquippedActive,
        active_state,
    });
}

/// `add_equipment_selection`'s real implementation
/// (`SavedCharacterMutationOp::AddEquipmentSelection` in the operation table
/// above) — see `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
fn add_equipment_selection_at_root(
    root: &Path,
    item_id: &str,
    active_state: ActiveState,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_add_equipment_selection(character_input, item_id, active_state);
    })
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

/// The wire-level projection of `AcquisitionMode` for the
/// `add_spell_selection` request. A separate DTO for the same reason as
/// `ActiveStateDto` above.
#[derive(Debug, Clone, Copy, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSpellSelectionRequest {
    pub character_id: String,
    pub spell_id: String,
    pub source_class_id: String,
    pub acquisition_mode: AcquisitionModeDto,
    pub saved_at: String,
}

/// Appends one entry to `chosen.spells_selected`. Every other field is
/// untouched.
pub fn apply_add_spell_selection(
    character_input: &mut CharacterInput,
    spell_id: &str,
    source_class_id: &str,
    acquisition_mode: AcquisitionMode,
) {
    character_input.chosen.spells_selected.push(SpellSelection {
        spell_id: spell_id.to_owned(),
        source_class_id: source_class_id.to_owned(),
        acquisition_mode,
    });
}

/// `add_spell_selection`'s real implementation
/// (`SavedCharacterMutationOp::AddSpellSelection` in the operation table
/// above) — see `mutate_saved_character_at_root` for the shared
/// load -> mutate -> recompute -> re-save -> return-envelope semantics.
fn add_spell_selection_at_root(
    root: &Path,
    spell_id: &str,
    source_class_id: &str,
    acquisition_mode: AcquisitionMode,
    saved_at: &str,
) -> Result<CreateCharacterResponse, String> {
    mutate_saved_character_at_root(root, saved_at, |character_input| {
        apply_add_spell_selection(character_input, spell_id, source_class_id, acquisition_mode);
    })
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

// ----- `import_character` (Storage Tier Minimal Fix, Criterion 23) -----
//
// Manual re-projection of `CharacterInput`'s full shape into
// `Deserialize`-only DTOs, for the same reason as this module's top-of-file
// note: the `codex` crate has zero dependencies, so `CharacterInput` itself
// is not `Deserialize`. Field names and wire values (e.g. `ActiveStateDto`'s
// `"EquippedActive"`/`"Absent"`/`"SelectedInactive"`,
// `AcquisitionModeDto`'s `"Known"`/`"Prepared"`/`"Granted"`) reuse the exact
// DTOs already established for `add_equipment_selection`/
// `add_spell_selection`, so the import JSON shape is consistent with the
// rest of this module's wire contract rather than a parallel invention.

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterClassLevelDto {
    pub class_id: String,
    pub level: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillAllocationDto {
    pub skill_id: String,
    pub ranks: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentSelectionImportDto {
    pub item_id: String,
    pub active_state: ActiveStateDto,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedChoiceDto {
    pub choice_set_id: String,
    pub selection_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellSelectionImportDto {
    pub spell_id: String,
    pub source_class_id: String,
    pub acquisition_mode: AcquisitionModeDto,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInputDto {
    pub source_package_id: String,
    pub chosen: ChosenCharacterStateDto,
}

/// The on-disk/exported shape `import_character` expects: a `displayLabel`
/// + `characterInput` (the fields needed to rebuild a real `CharacterInput`)
/// — the same two fields a full `SavedCharacterEnvelope` export carries.
/// Every other envelope field (`characterId`, `revisionId`, `savedAt`, ...)
/// is intentionally NOT part of this DTO: importing always mints a fresh
/// identity/revision/timestamp rather than trusting the source file's own
/// claimed identity (mirrors `clone_character`'s own "never blindly trust
/// source identity" stance). `serde` ignores any extra fields the source
/// JSON carries, so a real full-envelope export file is tolerated rather
/// than rejected for having "extra" fields.
#[derive(Debug, Clone, Deserialize)]
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
        },
        selection_provenance: Vec::new(),
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
        snapshot: map_snapshot_dto(snapshot),
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

    // ----- `mutate_saved_character` operation table (Criterion 16) -----

    #[test]
    fn saved_character_mutation_operations_table_documents_three_ops_all_wired() {
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
            ],
            "the table must enumerate exactly these three ops, in this order"
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
            ],
            "all three ops are callable through real Tauri commands as of this cycle"
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

    /// The single most important regression guard for Criterion 17: proves
    /// the real load -> mutate -> recompute -> re-save -> return round trip
    /// against a real `SavedCharacterStore` fixture on disk, not a mock.
    #[test]
    fn level_up_character_at_root_increments_level_and_persists_when_computed() {
        let root = tempdir("golden-path");
        let envelope = level_up_test_envelope("race:human", 1);
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = level_up_character_at_root(&root, FIGHTER_CLASS_ID, "2026-07-21T00:00:00Z")
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

        let response = level_up_character_at_root(&root, FIGHTER_CLASS_ID, "2026-07-21T00:00:00Z")
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

        let result = level_up_character_at_root(&root, FIGHTER_CLASS_ID, "2026-07-21T00:00:00Z");

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
}
