//! `recompute_character` Tauri command (SD-24 Epic 7, Criterion 7.2).
//!
//! Loads a saved character's authoritative `CharacterInput` from disk and
//! runs it back through the real rules-engine compute path
//! (`build_pilot_headless_receipt` / `PilotViewModel`), returning
//! freshly-computed derived stats (base attack bonus, saves, baseline
//! melee attack bonus, baseline armor class) rather than anything cached.
//!
//! This is the standalone "refresh" operation the Tauri command surface was
//! missing: every existing mutate-op in `crate::character_hub`
//! (`level_up_character`, `add_equipment_selection`, `add_spell_selection`)
//! already recomputes internally, but only ever as *part of* a combined
//! load -> mutate -> recompute -> re-save call. Nothing let a caller ask
//! "give me the current derived stats for this saved character, recomputed
//! fresh, without mutating it" as an independently callable, independently
//! testable operation — the gap Criterion 7.2's RED text names (recomputing
//! after a level-up did not refresh derived stats, because no dedicated
//! recompute path existed at all to call on demand).
//!
//! Never fabricates success: if the saved `CharacterInput` does not reach
//! `HeadlessReceiptStatus::Computed` (e.g. it names a class/level
//! combination outside the compute engine's currently supported range),
//! this returns `success: false` with a real diagnostic-derived `error`
//! string, never a `Computed`-shaped payload built from thin air.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::Manager;

use codex::saved_character::local_store::SavedCharacterStore;

use crate::corpus_fixtures::corpus_fixture_bundle;
use crate::pf1_adapter::{resolve_unified_pilot_snapshot, Pf1Adapter};
use crate::rule_system_adapter::RuleSystemAdapter;
use crate::stub_adapter::StubAdapter;

const CHARACTERS_ROOT_DIR_NAME: &str = "characters";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecomputeCharacterRequest {
    pub character_id: String,
    /// SD-25 Criterion 3.4 (Epic 3 "Hub of Hubs" Tauri command-surface
    /// routing): which rule system's `RuleSystemAdapter` to dispatch this
    /// recompute through — see `resolve_rule_system_adapter` below.
    pub rule_system_id: String,
}

/// Manual re-projection of `codex::rules_core::pilot_compute::BaseSaves` —
/// the `codex` crate has zero dependencies, so its own types are not
/// `Serialize` (mirrors `crate::character_hub::BaseSavesDto`'s own doc
/// comment for the same reason).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSavesDto {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

/// The freshly-recomputed derived-stat surface named by Criterion 7.2's
/// GREEN text (BAB / saves / skill points / caster level / etc.) — the
/// subset the real compute engine's `PilotSnapshot` view model actually
/// exposes today. Caster level / skill points are not yet distinct typed
/// fields on `PilotSnapshot` (the engine currently surfaces caster-level
/// arithmetic as `ComputationExplanation` records, not a summary field), so
/// this DTO carries every derived stat the engine does expose today rather
/// than inventing invented/fabricated fields for ones it does not.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSnapshotDto {
    pub character_id: String,
    pub base_attack_bonus: i16,
    pub base_saves: BaseSavesDto,
    pub baseline_melee_attack_bonus: i16,
    pub baseline_armor_class: i16,
    pub total_saves: BaseSavesDto,
    /// Mirrors `crate::character_hub::PilotSnapshotDto::damage_reduction`
    /// exactly (same source field, `PilotDefenseViewModel::damage_reduction`,
    /// same `skip_serializing_if` shape) — without this, the Defense tab's
    /// DR display would go stale on a manual "Recompute" refresh even though
    /// every other derived-stat field here already refreshes correctly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_reduction: Option<i16>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecomputeCharacterResponse {
    pub success: bool,
    pub character: Option<CharacterSnapshotDto>,
    pub error: Option<String>,
}

fn characters_root_from_app_data_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(CHARACTERS_ROOT_DIR_NAME)
}

fn resolve_character_root(app: &tauri::AppHandle, character_id: &str) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| format!("could not resolve app data directory: {err}"))?;
    Ok(characters_root_from_app_data_dir(&app_data_dir).join(character_id))
}

/// `recompute_character`'s real implementation: load the saved envelope
/// from `root`, run it through the real compute engine, and return the
/// freshly-derived stats. Never mutates or re-saves the on-disk envelope —
/// this is a read-and-recompute operation, not a mutate-op. Split from the
/// `#[tauri::command]` wrapper below so it is unit-testable against a real
/// `SavedCharacterStore` fixture without an `AppHandle`, mirroring
/// `character_hub.rs`'s own `level_up_character_at_root` split.
pub fn recompute_character_at_root(root: &Path, character_id: &str) -> RecomputeCharacterResponse {
    let envelope = match SavedCharacterStore::load(root) {
        Ok(envelope) => envelope,
        Err(_err) => {
            return RecomputeCharacterResponse {
                success: false,
                character: None,
                error: Some("character_not_found".to_owned()),
            };
        }
    };

    let snapshot = match resolve_unified_pilot_snapshot(&envelope.character_input, corpus_fixture_bundle()) {
        Ok((snapshot, _corpus_receipt)) => snapshot,
        Err(diagnostics) => {
            let blocking_messages: Vec<String> = diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.claim_blocking)
                .map(|diagnostic| diagnostic.message.clone())
                .collect();
            return RecomputeCharacterResponse {
                success: false,
                character: None,
                error: Some(format!(
                    "character_not_computable: {}",
                    blocking_messages.join("; ")
                )),
            };
        }
    };

    RecomputeCharacterResponse {
        success: true,
        character: Some(CharacterSnapshotDto {
            character_id: character_id.to_owned(),
            base_attack_bonus: snapshot.base_attack_bonus,
            base_saves: BaseSavesDto {
                fortitude: snapshot.base_saves.fortitude,
                reflex: snapshot.base_saves.reflex,
                will: snapshot.base_saves.will,
            },
            baseline_melee_attack_bonus: snapshot.combat.baseline_melee_attack_bonus,
            baseline_armor_class: snapshot.defense.baseline_armor_class,
            total_saves: BaseSavesDto {
                fortitude: snapshot.defense.total_save.fortitude,
                reflex: snapshot.defense.total_save.reflex,
                will: snapshot.defense.total_save.will,
            },
            damage_reduction: snapshot.defense.damage_reduction,
        }),
        error: None,
    }
}

/// Resolves `rule_system_id` to the `RuleSystemAdapter` implementation the
/// Tauri command dispatches through (SD-25 Criterion 3.4, `cycles/3_4.md`
/// GREEN) — `"pf1"` to the real `Pf1Adapter`; any other id to `StubAdapter`.
/// `StubAdapter::new` requires a `&'static str`; the caller-supplied
/// `rule_system_id` is a runtime `String`, so an unknown id is leaked once
/// per call to satisfy that bound — the same `Box::leak`-to-`'static`
/// pattern this crate already uses at `corpus_fixtures.rs` /
/// `codex::rules_core::equipment_resolver`. Unknown-`rule_system_id` calls
/// are the rare/exceptional path (real traffic is `"pf1"`), so the leak is
/// bounded by how many distinct not-yet-supported ids ever get dispatched.
fn resolve_rule_system_adapter(rule_system_id: &str) -> Box<dyn RuleSystemAdapter> {
    match rule_system_id {
        "pf1" => Box::new(Pf1Adapter),
        other => {
            let leaked: &'static str = Box::leak(other.to_owned().into_boxed_str());
            Box::new(StubAdapter::new(leaked))
        }
    }
}

/// Dispatches `recompute_character` through the `RuleSystemAdapter` trait
/// instead of calling `recompute_character_at_root` by name directly (SD-25
/// Criterion 3.4). For `"pf1"` this produces byte-for-byte the same real
/// behavior as calling `recompute_character_at_root` directly —
/// `Pf1Adapter::recompute` wraps that exact function — so every pre-existing
/// test above that calls `recompute_character_at_root` directly keeps
/// passing unchanged.
pub fn recompute_character_via_rule_system(
    rule_system_id: &str,
    root: &Path,
    character_id: &str,
) -> RecomputeCharacterResponse {
    resolve_rule_system_adapter(rule_system_id).recompute(root, character_id)
}

/// Loads the saved character named by `request.character_id` and returns
/// its freshly-recomputed derived stats — see
/// `recompute_character_at_root` for the full semantics.
#[tauri::command]
pub fn recompute_character(
    app: tauri::AppHandle,
    request: RecomputeCharacterRequest,
) -> Result<RecomputeCharacterResponse, String> {
    let root = resolve_character_root(&app, &request.character_id)?;
    Ok(recompute_character_via_rule_system(
        &request.rule_system_id,
        &root,
        &request.character_id,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use codex::rules_core::character_input::{
        AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput,
        ChosenCharacterState, EquipmentSelection, SelectedChoice, SkillAllocation, SpellSelection,
    };
    use codex::saved_character::{
        SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
    };

    const FIGHTER_CLASS_ID: &str = "class:fighter";
    const SOURCE_PACKAGE_ID: &str = "pf1.core_rulebook";
    const GAME_SYSTEM_ID: &str = "pf1";

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-recompute-character-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    /// A single-class Fighter build at `level`, mirroring
    /// `character_hub::compose_character_input`'s own fixed loadout exactly
    /// (this module cannot import that `fn` — it is private to
    /// `character_hub.rs` — so it is reproduced verbatim here). The compute
    /// engine's combat-baseline / defense / skill pillars each gate on this
    /// *exact* deterministic posture (`unmet_combat_posture_conditions` in
    /// `pilot_compute.rs` requires the longsword/chain-shirt/shield/
    /// power-attack equipment states verbatim), so this reaches `Computed`
    /// for every level the engine currently supports (1-20; see
    /// `MAX_SUPPORTED_FIGHTER_LEVEL` in `pilot_compute.rs`) and `Blocked`
    /// above that ceiling.
    fn fighter_character_input(level: u8) -> CharacterInput {
        CharacterInput {
            case_id: Some("char-recompute-test".to_owned()),
            source_package_id: SOURCE_PACKAGE_ID.to_owned(),
            chosen: ChosenCharacterState {
                race_id: "race:human".to_owned(),
                class_levels: vec![CharacterClassLevel {
                    class_id: FIGHTER_CLASS_ID.to_owned(),
                    level,
                }],
                ability_scores: AbilityScores {
                    strength: 16,
                    dexterity: 14,
                    constitution: 14,
                    intelligence: 10,
                    wisdom: 12,
                    charisma: 8,
                },
                selected_feats: vec![
                    "feat:power_attack".to_owned(),
                    "feat:dodge".to_owned(),
                    "feat:weapon_focus".to_owned(),
                ],
                skill_allocations: vec![
                    SkillAllocation { skill_id: "skill:climb".to_owned(), ranks: 1 },
                    SkillAllocation { skill_id: "skill:intimidate".to_owned(), ranks: 1 },
                    SkillAllocation { skill_id: "skill:swim".to_owned(), ranks: 1 },
                ],
                equipment_selections: vec![
                    EquipmentSelection {
                        item_id: "item:longsword".to_owned(),
                        equipped_or_active: true,
                        active_state: ActiveState::EquippedActive,
                        applied_modifiers: Vec::new(),
                    },
                    EquipmentSelection {
                        item_id: "item:chain_shirt".to_owned(),
                        equipped_or_active: true,
                        active_state: ActiveState::EquippedActive,
                        applied_modifiers: Vec::new(),
                    },
                    EquipmentSelection {
                        item_id: "item:shield".to_owned(),
                        equipped_or_active: false,
                        active_state: ActiveState::Absent,
                        applied_modifiers: Vec::new(),
                    },
                    EquipmentSelection {
                        item_id: "power_attack".to_owned(),
                        equipped_or_active: false,
                        active_state: ActiveState::SelectedInactive,
                        applied_modifiers: Vec::new(),
                    },
                ],
                selected_choices: vec![
                    SelectedChoice {
                        choice_set_id: "choice:level_1_character_feat".to_owned(),
                        selection_id: "feat:power_attack".to_owned(),
                    },
                    SelectedChoice {
                        choice_set_id: "choice:fighter_bonus_feat".to_owned(),
                        selection_id: "feat:weapon_focus:weapon:longsword".to_owned(),
                    },
                    SelectedChoice {
                        choice_set_id: "choice:human_bonus_feat".to_owned(),
                        selection_id: "feat:dodge".to_owned(),
                    },
                    SelectedChoice {
                        choice_set_id: "choice:human_ability_bonus".to_owned(),
                        selection_id: "ability:strength".to_owned(),
                    },
                ],
                selected_traits: Vec::new(),
                spells_selected: vec![
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
                ],
                class_ability_activations: Vec::new(),
            },
            selection_provenance: Vec::new(),
        }
    }

    fn envelope_for(character_id: &str, level: u8, saved_at: &str) -> SavedCharacterEnvelope {
        SavedCharacterEnvelope {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: saved_at.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: SOURCE_PACKAGE_ID.to_owned(),
            game_system: GAME_SYSTEM_ID.to_owned(),
            latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
            display_label: "Recompute Test Character".to_owned(),
            character_input: fighter_character_input(level),
        }
    }

    /// SD-25 Criterion 3.4 GREEN: `recompute_character_via_rule_system("pf1",
    /// ...)` dispatches through `dyn RuleSystemAdapter` and produces
    /// byte-for-byte the same real result `recompute_character_at_root`
    /// itself returns (`Pf1Adapter::recompute` wraps that exact function).
    #[test]
    fn recompute_character_via_rule_system_dispatches_pf1_through_the_trait() {
        let root = tempdir("pf1-dispatch");
        let envelope = envelope_for("char-recompute-pf1-dispatch", 1, "2026-07-21T00:00:00Z");
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let direct = recompute_character_at_root(&root, "char-recompute-pf1-dispatch");
        let dispatched =
            recompute_character_via_rule_system("pf1", &root, "char-recompute-pf1-dispatch");

        assert_eq!(
            dispatched, direct,
            "pf1 dispatch through the trait must match the direct call exactly"
        );
        assert!(dispatched.success);

        std::fs::remove_dir_all(&root).ok();
    }

    /// An unrecognized `rule_system_id` routes to `StubAdapter`
    /// (`cycles/3_4.md` GREEN) rather than silently falling through to PF1
    /// logic or panicking.
    #[test]
    fn recompute_character_via_rule_system_routes_unknown_id_to_stub_adapter() {
        let root = tempdir("unknown-system-dispatch");
        let envelope = envelope_for("char-recompute-unknown", 1, "2026-07-21T00:00:00Z");
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response =
            recompute_character_via_rule_system("starfinder", &root, "char-recompute-unknown");

        assert!(
            !response.success,
            "an unimplemented rule system must never report success"
        );
        assert_eq!(
            response.error.as_deref(),
            Some("Would render for system starfinder; not yet implemented")
        );

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn recompute_character_at_root_returns_fresh_derived_stats_for_a_saved_character() {
        let root = tempdir("golden-path");
        let envelope = envelope_for("char-recompute-1", 1, "2026-07-21T00:00:00Z");
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = recompute_character_at_root(&root, "char-recompute-1");

        assert!(response.success, "a Computed build must report success");
        let character = response
            .character
            .expect("a successful recompute must carry a character snapshot");
        assert_eq!(
            character.base_attack_bonus, 1,
            "Fighter level 1 base attack bonus is +1 (cr_classes.lst:139 BASEAB|classlevel)"
        );
        assert_eq!(character.base_saves.fortitude, 2, "Fighter level 1 good Fortitude save is +2");
        assert_eq!(
            character.damage_reduction, None,
            "Fighter has no grounded DR explanation, so this must be absent, not fabricated"
        );
        assert!(response.error.is_none());

        std::fs::remove_dir_all(&root).ok();
    }

    /// `damage_reduction` mirrors `PilotSnapshotDto`'s own field exactly
    /// (`crate::character_hub`) -- same source (`PilotDefenseViewModel::
    /// damage_reduction`), same wire shape. There is no live path to
    /// exercise a non-`None` value through `recompute_character_at_root`
    /// today (every class the compute engine actually reaches `Computed`
    /// for -- Fighter/Wizard/Rogue -- has no grounded DR explanation; only
    /// Barbarian's does, and Barbarian never reaches `Computed`), so this
    /// proves the wire shape directly against a hand-built `CharacterSnapshotDto`
    /// instead: a real `Some` value serializes as `"damageReduction"` (proving
    /// `#[serde(rename_all = "camelCase")]` alone is sufficient here -- unlike
    /// `PilotSnapshotDto`'s sibling `corpusDerived` field, this name has no
    /// internal word boundary collision with an enum tag, so no per-field
    /// `#[serde(rename = ...)]` is needed), and `None` omits the key
    /// entirely rather than serializing as `null`.
    #[test]
    fn damage_reduction_serializes_as_camel_case_when_present_and_is_omitted_when_absent() {
        let with_dr = CharacterSnapshotDto {
            character_id: "char-dr-present".to_owned(),
            base_attack_bonus: 5,
            base_saves: BaseSavesDto { fortitude: 4, reflex: 1, will: 1 },
            baseline_melee_attack_bonus: 7,
            baseline_armor_class: 14,
            total_saves: BaseSavesDto { fortitude: 4, reflex: 1, will: 1 },
            damage_reduction: Some(3),
        };
        let json = serde_json::to_string(&with_dr).expect("serialization should succeed");
        assert!(
            json.contains("\"damageReduction\":3"),
            "a present DR value must serialize under the camelCase key: {json}"
        );

        let without_dr = CharacterSnapshotDto { damage_reduction: None, ..with_dr };
        let json = serde_json::to_string(&without_dr).expect("serialization should succeed");
        assert!(
            !json.contains("damageReduction"),
            "an absent DR value must omit the key entirely, not serialize null: {json}"
        );
    }

    /// The direct regression guard for Criterion 7.2's RED text: a level
    /// bump that lands on disk through some path other than one of
    /// `character_hub`'s own combined mutate-ops (here simulated by writing
    /// the leveled-up `CharacterInput` straight through
    /// `SavedCharacterStore::save`, bypassing the compute engine entirely)
    /// must still come back with freshly-recomputed, non-stale derived
    /// stats the next time `recompute_character` is called — proving this
    /// command reads and recomputes from whatever is *currently* on disk,
    /// not anything cached from an earlier compute pass.
    #[test]
    fn recompute_character_at_root_reflects_a_level_bump_that_landed_on_disk_without_a_recompute() {
        let root = tempdir("stale-refresh");
        let level_1_envelope = envelope_for("char-recompute-2", 1, "2026-07-21T00:00:00Z");
        SavedCharacterStore::save(&level_1_envelope, &root).expect("seed save should succeed");

        // Simulate a level-up that landed on disk without going through the
        // recompute path (e.g. a future `reSaveCharacter` write, or any
        // other direct envelope mutation).
        let level_2_envelope = envelope_for("char-recompute-2", 2, "2026-07-21T00:05:00Z");
        SavedCharacterStore::save(&level_2_envelope, &root)
            .expect("raw re-save of the leveled-up build should succeed");

        let response = recompute_character_at_root(&root, "char-recompute-2");

        assert!(response.success);
        let character = response.character.expect("Computed build must carry a snapshot");
        assert_eq!(
            character.base_attack_bonus, 2,
            "recompute must reflect the level-2 build now on disk, not the level-1 value it \
             was originally saved with"
        );
        assert_eq!(character.base_saves.fortitude, 3, "Fighter level 2 good Fortitude save is +3");

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn recompute_character_at_root_returns_character_not_found_for_a_missing_saved_character() {
        let root = tempdir("missing-character");

        let response = recompute_character_at_root(&root, "does-not-exist");

        assert!(!response.success);
        assert!(response.character.is_none());
        assert_eq!(response.error.as_deref(), Some("character_not_found"));

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn recompute_character_at_root_returns_an_honest_failure_when_the_saved_build_is_blocked() {
        let root = tempdir("blocked-build");
        // Level 21 is one past the compute engine's supported Fighter ceiling
        // (`MAX_SUPPORTED_FIGHTER_LEVEL` in `pilot_compute.rs`).
        let envelope = envelope_for("char-recompute-3", 21, "2026-07-21T00:00:00Z");
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = recompute_character_at_root(&root, "char-recompute-3");

        assert!(
            !response.success,
            "a build outside the engine's supported range must never report success"
        );
        assert!(
            response.character.is_none(),
            "no character snapshot must be fabricated for a Blocked build"
        );
        let error = response.error.expect("a Blocked build must carry a real error");
        assert!(
            error.starts_with("character_not_computable"),
            "unexpected error for a Blocked build: {error}"
        );
        assert!(
            error.len() > "character_not_computable: ".len(),
            "the error must carry the real diagnostic message, not just the bare tag: {error}"
        );

        std::fs::remove_dir_all(&root).ok();
    }
}
