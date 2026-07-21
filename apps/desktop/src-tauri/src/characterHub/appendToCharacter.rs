//! `appendToCharacter` Tauri command (SD-24 Epic 7, Criterion 7.1).
//!
//! Closes the gap `technical-design.md` §3.1/§3.2 documents: none of
//! `character_hub`'s existing commands can append a *batch* of equipment
//! items to a saved character in one round trip, and none of them validate
//! `item_id` against the real CRB equipment corpus before mutating —
//! `add_equipment_selection` (SD-23) will happily push an entry for an
//! item_id that does not exist anywhere in the corpus, silently "adding"
//! something that isn't real (the duracon 2026-07-18 18:20:41 sentinel this
//! criterion's RED targets). `appendToCharacter` appends a batch and
//! validates every item first: either every item in the batch is a real
//! corpus record and gets appended, or none are.
//!
//! Reuses `character_hub::mutate_saved_character_at_root` /
//! `character_hub::resolve_character_root` (bumped to `pub(crate)` this
//! cycle) rather than re-deriving the load -> mutate -> recompute ->
//! re-save invariant a second time.

use std::collections::HashSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use codex::rules_core::character_input::EquipmentSelection;
use codex::rules_core::rules_tables::crb::equipment_tables::equipment_tables;

use crate::character_hub::{
    self, ActiveStateDto, CharacterSummaryDto, CorpusDerivedDto, CreateCharacterResponse,
    PilotSnapshotDto,
};

/// One equipment item to append, as requested over the wire.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemToAppendDto {
    pub item_id: String,
    pub active_state: ActiveStateDto,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendToCharacterRequest {
    pub character_id: String,
    pub items_to_append: Vec<ItemToAppendDto>,
    pub saved_at: String,
}

/// The successful-append projection of the saved character — the same
/// summary/snapshot/corpus-derived triad `CreateCharacterResponse::Saved`
/// already returns, flattened into the flat `{ success, character, error }`
/// shape `technical-design.md` §3.2 specifies for this command.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendedCharacterDto {
    pub summary: CharacterSummaryDto,
    pub snapshot: PilotSnapshotDto,
    pub corpus_derived: CorpusDerivedDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendToCharacterResponse {
    pub success: bool,
    pub character: Option<AppendedCharacterDto>,
    pub error: Option<String>,
}

fn known_equipment_keys() -> HashSet<&'static str> {
    equipment_tables().iter().map(|entry| entry.key).collect()
}

/// Real implementation behind the `append_to_character` Tauri command below
/// — split out so it is unit-testable against a real `SavedCharacterStore`
/// fixture without an `AppHandle`, mirroring every other mutation op in
/// `character_hub`.
///
/// Validates every `itemsToAppend[].itemId` against the real CRB equipment
/// corpus *before* touching the saved character — either every item is
/// real and gets appended, or none are (the no-partial-append,
/// no-silent-failure contract `technical-design.md` §3.3 sets for this
/// command). A `recompute_blocked` failure (the appended loadout doesn't
/// reach `Computed`) also leaves the on-disk character untouched, matching
/// `mutate_saved_character_at_root`'s own "never persist an unproven
/// build" invariant.
pub fn append_to_character_at_root(
    root: &Path,
    items_to_append: &[ItemToAppendDto],
    saved_at: &str,
) -> Result<AppendToCharacterResponse, String> {
    let known_keys = known_equipment_keys();
    if let Some(unknown) = items_to_append
        .iter()
        .find(|item| !known_keys.contains(item.item_id.as_str()))
    {
        return Ok(AppendToCharacterResponse {
            success: false,
            character: None,
            error: Some(format!("equipment_not_found: {}", unknown.item_id)),
        });
    }

    let outcome =
        character_hub::mutate_saved_character_at_root(root, saved_at, |character_input| {
            for item in items_to_append {
                character_input.chosen.equipment_selections.push(EquipmentSelection {
                    item_id: item.item_id.clone(),
                    equipped_or_active: matches!(item.active_state, ActiveStateDto::EquippedActive),
                    active_state: item.active_state.into(),
                });
            }
        })?;

    Ok(match outcome {
        CreateCharacterResponse::Saved {
            summary,
            snapshot,
            corpus_derived,
        } => AppendToCharacterResponse {
            success: true,
            character: Some(AppendedCharacterDto {
                summary: *summary,
                snapshot,
                corpus_derived,
            }),
            error: None,
        },
        CreateCharacterResponse::Blocked { diagnostics } => AppendToCharacterResponse {
            success: false,
            character: None,
            error: Some(format!(
                "recompute_blocked: {}",
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message.clone())
                    .collect::<Vec<_>>()
                    .join("; ")
            )),
        },
    })
}

/// Loads the saved character, validates + appends every requested item,
/// recomputes via the real engine, and re-saves — see
/// `append_to_character_at_root` for the full semantics.
#[tauri::command]
pub fn append_to_character(
    app: tauri::AppHandle,
    request: AppendToCharacterRequest,
) -> Result<AppendToCharacterResponse, String> {
    let root = character_hub::resolve_character_root(&app, &request.character_id)?;
    append_to_character_at_root(&root, &request.items_to_append, &request.saved_at)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character_hub::{compose_character_input, AbilityScoresDto, CreateCharacterRequest};
    use codex::saved_character::local_store::SavedCharacterStore;
    use codex::saved_character::{
        SavedCharacterEnvelope, SavedCharacterRevisionKind, CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
    };
    use std::path::PathBuf;

    const TEST_SAVED_AT: &str = "2026-07-21T00:00:00Z";

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-append-to-character-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    fn seed_envelope() -> SavedCharacterEnvelope {
        let request = CreateCharacterRequest {
            character_id: "char-append-test".to_owned(),
            display_label: "Append Test Character".to_owned(),
            race_id: "race:human".to_owned(),
            class_id: "class:fighter".to_owned(),
            level: 1,
            ability_scores: AbilityScoresDto {
                strength: 16,
                dexterity: 14,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            ability_bonus_target: "strength".to_owned(),
            saved_at: TEST_SAVED_AT.to_owned(),
        };
        let character_input = compose_character_input(&request);
        SavedCharacterEnvelope {
            character_id: "char-append-test".to_owned(),
            revision_id: "char-append-test.rev.1".to_owned(),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
            game_system: "pf1".to_owned(),
            latest_authoritative_revision_ref: "char-append-test.rev.1".to_owned(),
            display_label: "Append Test Character".to_owned(),
            character_input,
        }
    }

    /// The golden path: appending a real CRB equipment item actually
    /// appends it, and the saved character round-trips through
    /// load -> append -> recompute -> re-save (Criterion 7.1's GREEN).
    #[test]
    fn append_to_character_at_root_appends_real_item_and_persists_when_computed() {
        let root = tempdir("golden-path");
        let envelope = seed_envelope();
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = append_to_character_at_root(
            &root,
            &[ItemToAppendDto {
                item_id: "Dagger (Base)".to_owned(),
                active_state: ActiveStateDto::EquippedActive,
            }],
            "2026-07-21T01:00:00Z",
        )
        .expect("append call should not error");

        assert!(
            response.success,
            "appending a real corpus item must succeed: {:?}",
            response.error
        );
        let character = response
            .character
            .expect("a success response must carry the character");
        assert_eq!(character.summary.character_id, "char-append-test");

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len + 1,
            "the on-disk envelope must reflect the appended item"
        );
        let added = reloaded
            .character_input
            .chosen
            .equipment_selections
            .last()
            .expect("an entry was just pushed");
        assert_eq!(added.item_id, "Dagger (Base)");
        assert_eq!(reloaded.saved_at, "2026-07-21T01:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Criterion 7.1's RED sentinel: appending an item that is not in the
    /// real CRB equipment corpus must fail honestly (`success: false`, a
    /// real `error`) rather than lying with `success: true` for an append
    /// that never really happened.
    #[test]
    fn append_to_character_at_root_rejects_unknown_equipment_id() {
        let root = tempdir("unknown-item");
        let envelope = seed_envelope();
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = append_to_character_at_root(
            &root,
            &[ItemToAppendDto {
                item_id: "item:not-a-real-corpus-key".to_owned(),
                active_state: ActiveStateDto::EquippedActive,
            }],
            "2026-07-21T01:00:00Z",
        )
        .expect("append call should not error even on a validation rejection");

        assert!(!response.success);
        assert!(response.character.is_none());
        assert_eq!(
            response.error.as_deref(),
            Some("equipment_not_found: item:not-a-real-corpus-key")
        );

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len,
            "an unknown item must never be appended to the on-disk character"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// A batch append is all-or-nothing: if any item in the batch is
    /// unknown, no item in the batch is appended — not even the real ones.
    #[test]
    fn append_to_character_at_root_rejects_whole_batch_if_any_item_is_unknown() {
        let root = tempdir("mixed-batch");
        let envelope = seed_envelope();
        let starting_len = envelope.character_input.chosen.equipment_selections.len();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = append_to_character_at_root(
            &root,
            &[
                ItemToAppendDto {
                    item_id: "Dagger (Base)".to_owned(),
                    active_state: ActiveStateDto::EquippedActive,
                },
                ItemToAppendDto {
                    item_id: "item:not-a-real-corpus-key".to_owned(),
                    active_state: ActiveStateDto::EquippedActive,
                },
            ],
            "2026-07-21T01:00:00Z",
        )
        .expect("append call should not error");

        assert!(!response.success);

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.character_input.chosen.equipment_selections.len(),
            starting_len,
            "no item should be appended when the batch contains an unknown id"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// `append_to_character_at_root`'s error path when there is nothing
    /// saved at `root` yet — must fail honestly rather than silently
    /// creating a character out of thin air (mirrors
    /// `add_equipment_selection_at_root`'s own missing-character test in
    /// `character_hub.rs`).
    #[test]
    fn append_to_character_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("missing-character");

        let result = append_to_character_at_root(
            &root,
            &[ItemToAppendDto {
                item_id: "Dagger (Base)".to_owned(),
                active_state: ActiveStateDto::EquippedActive,
            }],
            TEST_SAVED_AT,
        );

        assert!(
            result.is_err(),
            "appending to a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }
}
