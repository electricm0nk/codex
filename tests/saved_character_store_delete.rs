//! Character Hub Storage Tier Minimal Fix — `SavedCharacterStore::delete` proof.
//!
//! Proves: deleting a saved character removes its on-disk directory tree
//! entirely (envelope + character-input files, and the directory itself),
//! and deleting a root that was never saved (or already deleted) is treated
//! as an idempotent success rather than an error — the caller's
//! postcondition ("nothing saved at this root") already holds either way.

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
    std::env::temp_dir().join(format!(
        "codex-{label}-{}-{unique}",
        std::process::id()
    ))
}

fn character_input_fixture_text() -> &'static str {
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt")
}

fn saved_envelope(character_id: &str) -> SavedCharacterEnvelope {
    let result = load_character_input_fixture(character_input_fixture_text());
    assert!(
        result.diagnostics.is_empty(),
        "fixture should parse cleanly: {:?}",
        result.diagnostics
    );
    let character_input = result
        .character_input
        .expect("fixture should produce a valid CharacterInput");

    SavedCharacterEnvelope {
        character_id: character_id.to_owned(),
        revision_id: format!("{character_id}.rev.1"),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: "2026-07-20T00:00:00Z".to_owned(),
        schema_version: 2,
        app_or_runtime_version: "codex-dev".to_owned(),
        content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
        game_system: "pf1".to_owned(),
        latest_authoritative_revision_ref: format!("{character_id}.rev.1"),
        display_label: "Delete Test Character".to_owned(),
        character_input,
    }
}

#[test]
fn delete_removes_the_directory_tree_for_a_saved_character() {
    let root = fresh_temp_dir("delete-golden-path");
    SavedCharacterStore::save(&saved_envelope("char-delete-golden"), &root)
        .expect("seed save should succeed");
    assert!(root.join("envelope.txt").exists(), "precondition: envelope.txt exists");
    assert!(
        root.join("authoritative_character_input.txt").exists(),
        "precondition: authoritative_character_input.txt exists"
    );

    SavedCharacterStore::delete(&root).expect("delete should succeed");

    assert!(
        !root.exists(),
        "the whole saved-character directory must be gone after delete"
    );
}

#[test]
fn delete_on_a_root_that_was_never_saved_is_treated_as_already_deleted() {
    let root = fresh_temp_dir("delete-nonexistent");
    assert!(!root.exists(), "precondition: root must not exist");

    let result = SavedCharacterStore::delete(&root);

    assert!(
        result.is_ok(),
        "deleting a root with nothing saved must be an idempotent success, got: {:?}",
        result.err()
    );
}

#[test]
fn delete_does_not_touch_a_sibling_saved_character() {
    let parent = fresh_temp_dir("delete-sibling-isolation");
    let target = parent.join("char-target");
    let sibling = parent.join("char-sibling");
    SavedCharacterStore::save(&saved_envelope("char-target"), &target)
        .expect("seed save (target) should succeed");
    SavedCharacterStore::save(&saved_envelope("char-sibling"), &sibling)
        .expect("seed save (sibling) should succeed");

    SavedCharacterStore::delete(&target).expect("delete should succeed");

    assert!(!target.exists(), "the target directory must be gone");
    assert!(
        sibling.exists(),
        "a sibling saved character must be untouched by deleting a different root"
    );
    let reloaded_sibling =
        SavedCharacterStore::load(&sibling).expect("sibling should still load after deletion");
    assert_eq!(reloaded_sibling.character_id, "char-sibling");

    fs::remove_dir_all(&parent).ok();
}
