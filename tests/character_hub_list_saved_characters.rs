//! Character Hub Phase 1 — SavedCharacterStore::list_all proof.
//!
//! Proves: listing an empty/missing characters root returns an empty listing
//! (not an error), listing a populated root returns every saved character as
//! a summary, and one corrupt saved-character subdirectory does not fail the
//! whole listing.

use codex::rules_core::character_input::load_character_input_fixture;
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::{SavedCharacterEnvelope, SavedCharacterRevisionKind};

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn fresh_temp_dir(label: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("codex-{label}-{}-{unique}", std::process::id()));
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn character_input_fixture_text() -> &'static str {
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt")
}

fn envelope(character_id: &str, display_label: &str, race_id_override: Option<&str>) -> SavedCharacterEnvelope {
    let result = load_character_input_fixture(character_input_fixture_text());
    assert!(
        result.diagnostics.is_empty(),
        "fixture should parse cleanly: {:?}",
        result.diagnostics
    );
    let mut character_input = result
        .character_input
        .expect("fixture should produce a valid CharacterInput");
    if let Some(race_id) = race_id_override {
        character_input.chosen.race_id = race_id.to_owned();
    }

    SavedCharacterEnvelope {
        character_id: character_id.to_owned(),
        revision_id: format!("{character_id}.rev.1"),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: "2026-07-08T00:00:00Z".to_owned(),
        schema_version: 2,
        app_or_runtime_version: "codex-dev".to_owned(),
        content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
        game_system: "pf1".to_owned(),
        latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
        display_label: display_label.to_owned(),
        character_input,
    }
}

#[test]
fn list_all_returns_empty_listing_for_nonexistent_root() {
    let root = std::env::temp_dir().join(format!(
        "codex-list-all-nonexistent-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos()
    ));
    assert!(!root.exists(), "precondition: root must not exist");

    let listing = SavedCharacterStore::list_all(&root).expect("missing root should not error");

    assert!(listing.characters.is_empty());
    assert!(listing.unreadable_entries.is_empty());
}

#[test]
fn list_all_returns_every_saved_character_under_root() {
    let root = fresh_temp_dir("list-all-populated");

    SavedCharacterStore::save(&envelope("char-one", "Aldric", None), &root.join("char-one"))
        .expect("save one should succeed");
    SavedCharacterStore::save(
        &envelope("char-two", "Borin", Some("race:half-orc")),
        &root.join("char-two"),
    )
    .expect("save two should succeed");

    let listing = SavedCharacterStore::list_all(&root).expect("list_all should succeed");

    assert_eq!(listing.characters.len(), 2, "both saved characters must be listed");
    assert!(listing.unreadable_entries.is_empty());

    let mut labels: Vec<&str> = listing
        .characters
        .iter()
        .map(|c| c.display_label.as_str())
        .collect();
    labels.sort_unstable();
    assert_eq!(labels, vec!["Aldric", "Borin"]);

    let aldric = listing
        .characters
        .iter()
        .find(|c| c.display_label == "Aldric")
        .expect("Aldric summary must be present");
    assert_eq!(aldric.character_id, "char-one");
    assert_eq!(aldric.game_system, "pf1");
    assert_eq!(aldric.schema_version, 2);
    assert_eq!(aldric.race_id, "race:human");
    assert_eq!(aldric.class_summary, "class:fighter:1");

    fs::remove_dir_all(&root).ok();
}

#[test]
fn list_all_reports_unreadable_entries_without_failing_the_whole_listing() {
    let root = fresh_temp_dir("list-all-partial-corrupt");

    SavedCharacterStore::save(&envelope("char-good", "Good Character", None), &root.join("char-good"))
        .expect("save should succeed");

    // A corrupt saved-character bundle: directory exists but envelope.txt is missing.
    let corrupt_dir = root.join("char-corrupt");
    fs::create_dir_all(&corrupt_dir).expect("corrupt dir should be creatable");
    fs::write(
        corrupt_dir.join("authoritative_character_input.txt"),
        character_input_fixture_text(),
    )
    .expect("should be able to write the character-input half of the corrupt bundle");

    let listing = SavedCharacterStore::list_all(&root).expect("list_all should still succeed overall");

    assert_eq!(listing.characters.len(), 1, "only the good character should be listed");
    assert_eq!(listing.characters[0].display_label, "Good Character");

    assert_eq!(listing.unreadable_entries.len(), 1, "the corrupt entry must be reported");
    assert_eq!(listing.unreadable_entries[0].entry_name, "char-corrupt");
    assert!(
        listing.unreadable_entries[0].message.contains("envelope"),
        "unreadable-entry message should explain why: {}",
        listing.unreadable_entries[0].message
    );

    fs::remove_dir_all(&root).ok();
}
