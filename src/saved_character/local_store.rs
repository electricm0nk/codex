//! Deterministic local-store boundary for saved-character artifacts.
//!
//! A saved-character bundle rooted at a single directory contains exactly two
//! files:
//! - `envelope.txt` — identity, revision, and provenance metadata
//! - `authoritative_character_input.txt` — the authoritative CharacterInput payload
//!   in the existing fixture grammar

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use crate::rules_core::character_input::{
    AcquisitionMode, ActiveState, CharacterInput, load_character_input_fixture,
};

use super::{
    SavedCharacterEnvelope, SavedCharacterListing, SavedCharacterListingError,
    SavedCharacterRevisionKind, SavedCharacterStoreError, SavedCharacterSummary,
};

const ENVELOPE_FILE: &str = "envelope.txt";
const CHARACTER_INPUT_FILE: &str = "authoritative_character_input.txt";

pub struct SavedCharacterStore;

impl SavedCharacterStore {
    /// Save a saved-character envelope to a root directory.
    ///
    /// Writes `envelope.txt` and `authoritative_character_input.txt` under `root`.
    /// Creates `root` if it does not exist.
    pub fn save(
        envelope: &SavedCharacterEnvelope,
        root: &Path,
    ) -> Result<(), SavedCharacterStoreError> {
        fs::create_dir_all(root).map_err(|err| io_error(root, err))?;

        let ensure_single_line = |field: &str,
                                  value: &str|
         -> Result<(), SavedCharacterStoreError> {
            if value.contains('\n') || value.contains('\r') {
                return Err(SavedCharacterStoreError {
                    message: format!(
                        "{field} contains a newline/carriage return; cannot be persisted in envelope.txt"
                    ),
                });
            }
            Ok(())
        };

        ensure_single_line("character_id", &envelope.character_id)?;
        ensure_single_line("revision_id", &envelope.revision_id)?;
        ensure_single_line("saved_at", &envelope.saved_at)?;
        ensure_single_line("app_or_runtime_version", &envelope.app_or_runtime_version)?;
        ensure_single_line(
            "content_or_rules_provenance",
            &envelope.content_or_rules_provenance,
        )?;
        ensure_single_line("game_system", &envelope.game_system)?;
        ensure_single_line(
            "latest_authoritative_revision_ref",
            &envelope.latest_authoritative_revision_ref,
        )?;
        ensure_single_line("display_label", &envelope.display_label)?;

        validate_character_input(&envelope.character_input)?;

        let envelope_path = root.join(ENVELOPE_FILE);
        fs::write(&envelope_path, render_envelope(envelope))
            .map_err(|err| io_error(&envelope_path, err))?;

        let character_input_path = root.join(CHARACTER_INPUT_FILE);
        fs::write(
            &character_input_path,
            render_character_input(&envelope.character_input),
        )
        .map_err(|err| io_error(&character_input_path, err))?;

        Ok(())
    }

    /// Load a saved-character envelope from a root directory.
    ///
    /// Reads `envelope.txt` and `authoritative_character_input.txt` from `root`.
    /// Returns `Err` if either file is missing, malformed, or missing required fields.
    pub fn load(root: &Path) -> Result<SavedCharacterEnvelope, SavedCharacterStoreError> {
        let envelope_path = root.join(ENVELOPE_FILE);
        let envelope_text =
            fs::read_to_string(&envelope_path).map_err(|err| SavedCharacterStoreError {
                message: format!(
                    "envelope.txt missing or unreadable at {}: {err}",
                    envelope_path.display()
                ),
            })?;

        let character_input_path = root.join(CHARACTER_INPUT_FILE);
        let character_input_text =
            fs::read_to_string(&character_input_path).map_err(|err| SavedCharacterStoreError {
                message: format!(
                    "authoritative_character_input.txt missing or unreadable at {}: {err}",
                    character_input_path.display()
                ),
            })?;

        let metadata = parse_envelope(&envelope_text)?;
        let character_input = parse_character_input(&character_input_text)?;

        Ok(SavedCharacterEnvelope {
            character_id: metadata.character_id,
            revision_id: metadata.revision_id,
            revision_kind: metadata.revision_kind,
            saved_at: metadata.saved_at,
            schema_version: metadata.schema_version,
            app_or_runtime_version: metadata.app_or_runtime_version,
            content_or_rules_provenance: metadata.content_or_rules_provenance,
            game_system: metadata.game_system,
            latest_authoritative_revision_ref: metadata.latest_authoritative_revision_ref,
            display_label: metadata.display_label,
            character_input,
        })
    }

    /// List every saved character under `characters_root`.
    ///
    /// A nonexistent root returns an empty listing rather than an error — a
    /// character hub with no characters yet is not a failure. Each
    /// subdirectory is loaded independently: one corrupt/unreadable saved
    /// character is reported in `unreadable_entries` without failing the
    /// rest of the listing.
    pub fn list_all(characters_root: &Path) -> Result<SavedCharacterListing, SavedCharacterStoreError> {
        let entries = match fs::read_dir(characters_root) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(SavedCharacterListing {
                    characters: Vec::new(),
                    unreadable_entries: Vec::new(),
                });
            }
            Err(err) => return Err(io_error(characters_root, err)),
        };

        let mut subdirectories: Vec<_> = entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .collect();
        subdirectories.sort_by_key(|entry| entry.file_name());

        let mut characters = Vec::new();
        let mut unreadable_entries = Vec::new();
        for entry in subdirectories {
            match Self::load(&entry.path()) {
                Ok(envelope) => characters.push(summarize(&envelope)),
                Err(err) => unreadable_entries.push(SavedCharacterListingError {
                    entry_name: entry.file_name().to_string_lossy().into_owned(),
                    message: err.message,
                }),
            }
        }

        Ok(SavedCharacterListing {
            characters,
            unreadable_entries,
        })
    }

    /// Delete a saved-character bundle by removing its root directory tree.
    ///
    /// Deleting a root that does not exist is treated as an idempotent
    /// success — the caller's postcondition ("nothing saved at this root")
    /// already holds, mirroring `list_all`'s own not-found handling above.
    /// Any other I/O failure (permissions, a path component that is a file
    /// rather than a directory, etc.) is a real error and is not swallowed.
    pub fn delete(root: &Path) -> Result<(), SavedCharacterStoreError> {
        match fs::remove_dir_all(root) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(io_error(root, err)),
        }
    }
}

fn summarize(envelope: &SavedCharacterEnvelope) -> SavedCharacterSummary {
    let class_summary = envelope
        .character_input
        .chosen
        .class_levels
        .iter()
        .map(|class_level| format!("{}:{}", class_level.class_id, class_level.level))
        .collect::<Vec<_>>()
        .join(",");

    SavedCharacterSummary {
        character_id: envelope.character_id.clone(),
        display_label: envelope.display_label.clone(),
        game_system: envelope.game_system.clone(),
        schema_version: envelope.schema_version,
        saved_at: envelope.saved_at.clone(),
        race_id: envelope.character_input.chosen.race_id.clone(),
        class_summary,
    }
}

// --- Save-time validation ---

/// Rejects a `CharacterInput` whose rendered fixture lines the loader could not
/// read back as the same record. The fixture grammar is line- and colon-based,
/// so every persisted string must be single-line, and a selected choice must
/// match the loader's segment shape (`choice_set_id` = exactly two
/// colon-segments, `selection_id` = at least two) or it would reload as a
/// different choice — or not at all.
fn validate_character_input(input: &CharacterInput) -> Result<(), SavedCharacterStoreError> {
    let single_line = |field: &str, value: &str| -> Result<(), SavedCharacterStoreError> {
        if value.contains('\n') || value.contains('\r') {
            return Err(SavedCharacterStoreError {
                message: format!(
                    "character input {field} contains a newline/carriage return; cannot be \
                     persisted in authoritative_character_input.txt"
                ),
            });
        }
        Ok(())
    };

    if let Some(case_id) = &input.case_id {
        single_line("case_id", case_id)?;
    }
    single_line("source_package_id", &input.source_package_id)?;
    single_line("race_id", &input.chosen.race_id)?;
    for cl in &input.chosen.class_levels {
        single_line("class_level class_id", &cl.class_id)?;
    }
    for feat in &input.chosen.selected_feats {
        single_line("selected feat", feat)?;
    }
    for character_trait in &input.chosen.selected_traits {
        single_line("selected trait", character_trait)?;
    }
    for skill in &input.chosen.skill_allocations {
        single_line("skill allocation skill_id", &skill.skill_id)?;
    }
    for eq in &input.chosen.equipment_selections {
        single_line("equipment selection item_id", &eq.item_id)?;
        for modifier in &eq.applied_modifiers {
            single_line("equipment selection applied_modifiers modifier_item_id", modifier)?;
        }
    }
    for spell in &input.chosen.spells_selected {
        single_line("spell selection spell_id", &spell.spell_id)?;
        single_line("spell selection source_class_id", &spell.source_class_id)?;
    }
    for choice in &input.chosen.selected_choices {
        single_line("selected choice choice_set_id", &choice.choice_set_id)?;
        single_line("selected choice selection_id", &choice.selection_id)?;
        if choice.choice_set_id.split(':').count() != 2 {
            return Err(SavedCharacterStoreError {
                message: format!(
                    "selected choice choice_set_id '{}' must have exactly two colon-segments \
                     to round-trip through the fixture grammar",
                    choice.choice_set_id
                ),
            });
        }
        if choice.selection_id.split(':').count() < 2 {
            return Err(SavedCharacterStoreError {
                message: format!(
                    "selected choice selection_id '{}' must have at least two colon-segments \
                     to round-trip through the fixture grammar",
                    choice.selection_id
                ),
            });
        }
    }
    for prov in &input.selection_provenance {
        single_line("selection provenance source_ref", &prov.source_ref)?;
    }

    Ok(())
}

// --- Rendering ---

fn render_envelope(envelope: &SavedCharacterEnvelope) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "character_id = {}", envelope.character_id);
    let _ = writeln!(out, "revision_id = {}", envelope.revision_id);
    let _ = writeln!(out, "revision_kind = {}", envelope.revision_kind.as_str());
    let _ = writeln!(out, "saved_at = {}", envelope.saved_at);
    let _ = writeln!(out, "schema_version = {}", envelope.schema_version);
    let _ = writeln!(
        out,
        "app_or_runtime_version = {}",
        envelope.app_or_runtime_version
    );
    let _ = writeln!(
        out,
        "content_or_rules_provenance = {}",
        envelope.content_or_rules_provenance
    );
    let _ = writeln!(out, "game_system = {}", envelope.game_system);
    let _ = writeln!(
        out,
        "latest_authoritative_revision_ref = {}",
        envelope.latest_authoritative_revision_ref
    );
    let _ = writeln!(out, "display_label = {}", envelope.display_label);
    out
}

fn render_character_input(input: &CharacterInput) -> String {
    let mut out = String::new();
    if let Some(case_id) = &input.case_id {
        let _ = writeln!(out, "case_id={case_id}");
    }
    let _ = writeln!(out, "source_package_id={}", input.source_package_id);
    let _ = writeln!(out, "race_id={}", input.chosen.race_id);
    for cl in &input.chosen.class_levels {
        let _ = writeln!(out, "class_level={}:{}", cl.class_id, cl.level);
    }
    let _ = writeln!(
        out,
        "ability=strength:{}",
        input.chosen.ability_scores.strength
    );
    let _ = writeln!(
        out,
        "ability=dexterity:{}",
        input.chosen.ability_scores.dexterity
    );
    let _ = writeln!(
        out,
        "ability=constitution:{}",
        input.chosen.ability_scores.constitution
    );
    let _ = writeln!(
        out,
        "ability=intelligence:{}",
        input.chosen.ability_scores.intelligence
    );
    let _ = writeln!(out, "ability=wisdom:{}", input.chosen.ability_scores.wisdom);
    let _ = writeln!(
        out,
        "ability=charisma:{}",
        input.chosen.ability_scores.charisma
    );
    for feat in &input.chosen.selected_feats {
        let _ = writeln!(out, "feat={feat}");
    }
    for character_trait in &input.chosen.selected_traits {
        let _ = writeln!(out, "trait={character_trait}");
    }
    for skill in &input.chosen.skill_allocations {
        let _ = writeln!(out, "skill={}:{}", skill.skill_id, skill.ranks);
    }
    for eq in &input.chosen.equipment_selections {
        let state = match eq.active_state {
            ActiveState::EquippedActive => "equipped",
            ActiveState::SelectedInactive => "selected_inactive",
            ActiveState::Absent => "absent",
        };
        let _ = writeln!(out, "equipment={}:{}", eq.item_id, state);
        // Must come after this selection's own `equipment=` line --
        // `apply_equipment_modifier`'s own parser looks up the matching
        // selection by `item_id` and requires it to already exist.
        for modifier in &eq.applied_modifiers {
            let _ = writeln!(out, "equipment_modifier={}:{}", eq.item_id, modifier);
        }
    }
    for spell in &input.chosen.spells_selected {
        let mode = match spell.acquisition_mode {
            AcquisitionMode::Known => "known",
            AcquisitionMode::Prepared => "prepared",
            AcquisitionMode::Granted => "granted",
        };
        let _ = writeln!(
            out,
            "spell={}:{}:{mode}",
            spell.spell_id, spell.source_class_id
        );
    }
    for choice in &input.chosen.selected_choices {
        let _ = writeln!(
            out,
            "choice={}:{}",
            choice.choice_set_id, choice.selection_id
        );
    }
    for prov in &input.selection_provenance {
        let _ = writeln!(out, "provenance={}", prov.source_ref);
    }
    out
}

// --- Parsing ---

#[derive(Default)]
struct ParsedEnvelopeMetadata {
    character_id: Option<String>,
    revision_id: Option<String>,
    revision_kind: Option<SavedCharacterRevisionKind>,
    saved_at: Option<String>,
    schema_version: Option<u16>,
    app_or_runtime_version: Option<String>,
    content_or_rules_provenance: Option<String>,
    game_system: Option<String>,
    latest_authoritative_revision_ref: Option<String>,
    display_label: Option<String>,
}

struct EnvelopeMetadata {
    character_id: String,
    revision_id: String,
    revision_kind: SavedCharacterRevisionKind,
    saved_at: String,
    schema_version: u16,
    app_or_runtime_version: String,
    content_or_rules_provenance: String,
    game_system: String,
    latest_authoritative_revision_ref: String,
    display_label: String,
}

/// Back-compat default for schema_version=1 envelopes saved before the
/// `game_system` field existed. Every pre-existing saved character is PF1e;
/// this derives the canonical short id from the existing
/// `content_or_rules_provenance` lineage prefix (e.g. "pf1.core_rulebook" ->
/// "pf1") instead of fabricating an unrelated default.
const LEGACY_DEFAULT_GAME_SYSTEM: &str = "pf1";

fn derive_legacy_game_system(content_or_rules_provenance: &str) -> String {
    content_or_rules_provenance
        .split('.')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or(LEGACY_DEFAULT_GAME_SYSTEM)
        .to_owned()
}

fn parse_envelope(text: &str) -> Result<EnvelopeMetadata, SavedCharacterStoreError> {
    let mut parsed = ParsedEnvelopeMetadata::default();

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            return Err(parse_error(format!(
                "envelope line missing '=': {raw_line}"
            )));
        };
        let key = key.trim();
        let value = value.trim();

        match key {
            "character_id" => parsed.character_id = Some(value.to_owned()),
            "revision_id" => parsed.revision_id = Some(value.to_owned()),
            "revision_kind" => {
                let kind = SavedCharacterRevisionKind::parse(value).ok_or_else(|| {
                    parse_error(format!("envelope revision_kind '{value}' is unsupported"))
                })?;
                parsed.revision_kind = Some(kind);
            }
            "saved_at" => parsed.saved_at = Some(value.to_owned()),
            "schema_version" => {
                let version = value.parse::<u16>().map_err(|_| {
                    parse_error(format!(
                        "envelope schema_version '{value}' is not a valid version number"
                    ))
                })?;
                parsed.schema_version = Some(version);
            }
            "app_or_runtime_version" => parsed.app_or_runtime_version = Some(value.to_owned()),
            "content_or_rules_provenance" => {
                parsed.content_or_rules_provenance = Some(value.to_owned())
            }
            "game_system" => parsed.game_system = Some(value.to_owned()),
            "latest_authoritative_revision_ref" => {
                parsed.latest_authoritative_revision_ref = Some(value.to_owned())
            }
            "display_label" => parsed.display_label = Some(value.to_owned()),
            unknown => {
                return Err(parse_error(format!(
                    "envelope unsupported field '{unknown}'"
                )));
            }
        }
    }

    let content_or_rules_provenance = parsed.content_or_rules_provenance.ok_or_else(|| {
        parse_error("envelope missing required field content_or_rules_provenance")
    })?;
    // Pre-existing (schema_version=1) envelopes have no game_system line at
    // all; derive an honest default from the content/rules lineage rather
    // than failing to load a legacy artifact.
    let game_system = parsed
        .game_system
        .unwrap_or_else(|| derive_legacy_game_system(&content_or_rules_provenance));

    Ok(EnvelopeMetadata {
        character_id: parsed
            .character_id
            .ok_or_else(|| parse_error("envelope missing required field character_id"))?,
        revision_id: parsed
            .revision_id
            .ok_or_else(|| parse_error("envelope missing required field revision_id"))?,
        revision_kind: parsed
            .revision_kind
            .ok_or_else(|| parse_error("envelope missing required field revision_kind"))?,
        saved_at: parsed
            .saved_at
            .ok_or_else(|| parse_error("envelope missing required field saved_at"))?,
        schema_version: parsed
            .schema_version
            .ok_or_else(|| parse_error("envelope missing required field schema_version"))?,
        app_or_runtime_version: parsed
            .app_or_runtime_version
            .ok_or_else(|| parse_error("envelope missing required field app_or_runtime_version"))?,
        content_or_rules_provenance,
        game_system,
        latest_authoritative_revision_ref: parsed.latest_authoritative_revision_ref.ok_or_else(
            || parse_error("envelope missing required field latest_authoritative_revision_ref"),
        )?,
        display_label: parsed
            .display_label
            .ok_or_else(|| parse_error("envelope missing required field display_label"))?,
    })
}

fn parse_character_input(text: &str) -> Result<CharacterInput, SavedCharacterStoreError> {
    let result = load_character_input_fixture(text);
    if !result.diagnostics.is_empty() {
        let messages: Vec<&str> = result
            .diagnostics
            .iter()
            .map(|d| d.message.as_str())
            .collect();
        return Err(SavedCharacterStoreError {
            message: format!(
                "authoritative_character_input.txt failed to parse: {}",
                messages.join("; ")
            ),
        });
    }
    result
        .character_input
        .ok_or_else(|| SavedCharacterStoreError {
            message: "authoritative_character_input.txt produced no valid CharacterInput"
                .to_owned(),
        })
}

fn io_error(path: &Path, err: std::io::Error) -> SavedCharacterStoreError {
    SavedCharacterStoreError {
        message: format!("{}: {err}", path.display()),
    }
}

fn parse_error(message: impl Into<String>) -> SavedCharacterStoreError {
    SavedCharacterStoreError {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::character_input::{
        AbilityScores, CharacterClassLevel, ChosenCharacterState, EquipmentSelection,
    };
    use crate::saved_character::CURRENT_SAVED_CHARACTER_SCHEMA_VERSION;

    fn tempdir(label: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "codex-local-store-{label}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    fn envelope_with(equipment_selections: Vec<EquipmentSelection>) -> SavedCharacterEnvelope {
        SavedCharacterEnvelope {
            character_id: "char-local-store-test".to_owned(),
            revision_id: "char-local-store-test.rev.1".to_owned(),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: "2026-07-24T00:00:00Z".to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
            game_system: "pf1".to_owned(),
            latest_authoritative_revision_ref: "char-local-store-test.rev.1".to_owned(),
            display_label: "Local Store Test Character".to_owned(),
            character_input: CharacterInput {
                case_id: None,
                source_package_id: "core_rulebook".to_owned(),
                chosen: ChosenCharacterState {
                    race_id: "race:human".to_owned(),
                    class_levels: vec![CharacterClassLevel {
                        class_id: "class:fighter".to_owned(),
                        level: 1,
                    }],
                    ability_scores: AbilityScores {
                        strength: 16,
                        dexterity: 14,
                        constitution: 14,
                        intelligence: 10,
                        wisdom: 12,
                        charisma: 8,
                    },
                    selected_feats: Vec::new(),
                    skill_allocations: Vec::new(),
                    equipment_selections,
                    selected_choices: Vec::new(),
                    selected_traits: Vec::new(),
                    spells_selected: Vec::new(),
                    class_ability_activations: Vec::new(),
                },
                selection_provenance: Vec::new(),
            },
        }
    }

    /// v0.6 alpha swarm items 1+27 sub-task 6: `applied_modifiers` must
    /// survive a real save/load round-trip through this store, not just
    /// exist as an in-memory field. Found by an end-to-end desktop test
    /// (`attach_equipment_modifier_at_root`) reloading empty after a
    /// reported-successful attach -- traced to this exact gap: the
    /// fixture-text *parser* (`load_character_input_fixture`) already
    /// supported `equipment_modifier=` lines (sub-task 1), but this
    /// store's own *writer* never emitted them, silently dropping every
    /// attached modifier on save.
    #[test]
    fn save_and_load_round_trips_applied_modifiers() {
        let root = tempdir("applied-modifiers-round-trip");
        let envelope = envelope_with(vec![EquipmentSelection {
            item_id: "item:longsword".to_owned(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: vec![
                "Special Ability ~ +1 ~ Weapon".to_owned(),
                "Special Quality ~ Masterwork ~ Weapon".to_owned(),
            ],
        }]);

        SavedCharacterStore::save(&envelope, &root).expect("save should succeed");
        let reloaded = SavedCharacterStore::load(&root).expect("load should succeed");

        assert_eq!(
            reloaded.character_input.chosen.equipment_selections[0].applied_modifiers,
            vec![
                "Special Ability ~ +1 ~ Weapon".to_owned(),
                "Special Quality ~ Masterwork ~ Weapon".to_owned(),
            ]
        );

        fs::remove_dir_all(&root).ok();
    }

    /// A selection with no attached modifiers must round-trip to a real
    /// empty list, not an error or a fabricated entry -- the common case
    /// every pre-sub-task-1 saved character already exercises.
    #[test]
    fn save_and_load_round_trips_an_empty_applied_modifiers_list() {
        let root = tempdir("applied-modifiers-empty-round-trip");
        let envelope = envelope_with(vec![EquipmentSelection {
            item_id: "item:longsword".to_owned(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        }]);

        SavedCharacterStore::save(&envelope, &root).expect("save should succeed");
        let reloaded = SavedCharacterStore::load(&root).expect("load should succeed");

        assert!(reloaded.character_input.chosen.equipment_selections[0].applied_modifiers.is_empty());

        fs::remove_dir_all(&root).ok();
    }

    /// **AT-34-E4-002**: `selected_traits` must survive a real save/load
    /// round-trip through this store, the same discipline
    /// `save_and_load_round_trips_applied_modifiers`' own doc comment
    /// records finding missing for `applied_modifiers` -- the parser
    /// (`load_character_input_fixture`) and the writer
    /// (`render_character_input`) must agree on the `trait=` line, or a
    /// player's trait selection silently vanishes on save.
    #[test]
    fn save_and_load_round_trips_selected_traits() {
        let root = tempdir("selected-traits-round-trip");
        let mut envelope = envelope_with(Vec::new());
        envelope.character_input.chosen.selected_traits = vec![
            "trait:trait_acrobat".to_owned(),
            "trait:trait_ease_of_faith".to_owned(),
        ];

        SavedCharacterStore::save(&envelope, &root).expect("save should succeed");
        let reloaded = SavedCharacterStore::load(&root).expect("load should succeed");

        assert_eq!(
            reloaded.character_input.chosen.selected_traits,
            vec!["trait:trait_acrobat".to_owned(), "trait:trait_ease_of_faith".to_owned()]
        );

        fs::remove_dir_all(&root).ok();
    }

    /// No trait selections at all must round-trip to a real empty list --
    /// the common case for every character created before this cycle.
    #[test]
    fn save_and_load_round_trips_an_empty_selected_traits_list() {
        let root = tempdir("selected-traits-empty-round-trip");
        let envelope = envelope_with(Vec::new());

        SavedCharacterStore::save(&envelope, &root).expect("save should succeed");
        let reloaded = SavedCharacterStore::load(&root).expect("load should succeed");

        assert!(reloaded.character_input.chosen.selected_traits.is_empty());

        fs::remove_dir_all(&root).ok();
    }
}
