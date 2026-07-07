//! SD14-E1-F1 integration tests — saved-character envelope and local-store boundary.
//!
//! Proves:
//! 1. round-trip save/load preserves identity, revision, provenance, and CharacterInput payload
//! 2. the checked-in fixture bundle loads as the same bounded envelope shape the save path writes
//! 3. malformed or incomplete saved-character artifacts fail honestly

use codex::rules_core::character_input::load_character_input_fixture;
use codex::saved_character::local_store::SavedCharacterStore;
use codex::saved_character::{SavedCharacterEnvelope, SavedCharacterRevisionKind};

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sd14/pf1_human_fighter_level1_saved_character")
}

fn fresh_temp_dir(label: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("codex-{label}-{}-{unique}", std::process::id()));
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn pilot_character_input() -> codex::rules_core::character_input::CharacterInput {
    let fixture_text = include_str!(
        "fixtures/sd14/pf1_human_fighter_level1_saved_character/authoritative_character_input.txt"
    );
    let result = load_character_input_fixture(fixture_text);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should parse cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("fixture should produce a valid CharacterInput")
}

fn pilot_envelope() -> SavedCharacterEnvelope {
    let character_input = pilot_character_input();
    SavedCharacterEnvelope {
        character_id: "sd14.fixture.pf1-human-fighter-level1".to_owned(),
        revision_id: "sd14.fixture.pf1-human-fighter-level1.rev.1".to_owned(),
        revision_kind: SavedCharacterRevisionKind::Authoritative,
        saved_at: "2026-06-30T00:00:00Z".to_owned(),
        schema_version: 1,
        app_or_runtime_version: "codex-dev".to_owned(),
        content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
        latest_authoritative_revision_ref: "sd14.fixture.pf1-human-fighter-level1.rev.1".to_owned(),
        display_label: "PF1 Human Fighter Level 1".to_owned(),
        character_input,
    }
}

#[test]
fn sd14_round_trip_save_and_load_preserves_envelope_and_character_input() {
    let envelope = pilot_envelope();
    let root = fresh_temp_dir("sd14-round-trip");

    SavedCharacterStore::save(&envelope, &root).expect("save should succeed");

    let reloaded = SavedCharacterStore::load(&root).expect("load should succeed");

    assert_eq!(reloaded.character_id, envelope.character_id);
    assert_eq!(reloaded.revision_id, envelope.revision_id);
    assert_eq!(reloaded.revision_kind, envelope.revision_kind);
    assert_eq!(reloaded.saved_at, envelope.saved_at);
    assert_eq!(reloaded.schema_version, envelope.schema_version);
    assert_eq!(
        reloaded.app_or_runtime_version,
        envelope.app_or_runtime_version
    );
    assert_eq!(
        reloaded.content_or_rules_provenance,
        envelope.content_or_rules_provenance
    );
    assert_eq!(
        reloaded.latest_authoritative_revision_ref,
        envelope.latest_authoritative_revision_ref
    );
    assert_eq!(reloaded.display_label, envelope.display_label);
    assert_eq!(reloaded.character_input, envelope.character_input);
}

#[test]
fn sd14_checked_in_fixture_loads_as_bounded_envelope_shape() {
    let root = fixture_root();
    let envelope = SavedCharacterStore::load(&root).expect("checked-in fixture should load");

    assert_eq!(
        envelope.character_id,
        "sd14.fixture.pf1-human-fighter-level1"
    );
    assert_eq!(
        envelope.revision_id,
        "sd14.fixture.pf1-human-fighter-level1.rev.1"
    );
    assert_eq!(
        envelope.revision_kind,
        SavedCharacterRevisionKind::Authoritative
    );
    assert_eq!(envelope.schema_version, 1);
    assert!(!envelope.display_label.is_empty());

    assert_eq!(
        envelope.character_input.source_package_id,
        "pf1.core_rulebook"
    );
    assert_eq!(envelope.character_input.chosen.race_id, "race:human");
    assert_eq!(envelope.character_input.chosen.class_levels.len(), 1);
    assert_eq!(
        envelope.character_input.chosen.class_levels[0].class_id,
        "class:fighter"
    );
    assert_eq!(envelope.character_input.chosen.class_levels[0].level, 1);
    assert_eq!(envelope.character_input.chosen.ability_scores.strength, 16);
}

#[test]
fn sd14_fixture_round_trip_matches_constructed_pilot_envelope() {
    let root = fixture_root();
    let from_fixture = SavedCharacterStore::load(&root).expect("checked-in fixture should load");
    let constructed = pilot_envelope();

    assert_eq!(from_fixture.character_id, constructed.character_id);
    assert_eq!(from_fixture.revision_id, constructed.revision_id);
    assert_eq!(from_fixture.revision_kind, constructed.revision_kind);
    assert_eq!(from_fixture.saved_at, constructed.saved_at);
    assert_eq!(from_fixture.schema_version, constructed.schema_version);
    assert_eq!(
        from_fixture.app_or_runtime_version,
        constructed.app_or_runtime_version
    );
    assert_eq!(
        from_fixture.content_or_rules_provenance,
        constructed.content_or_rules_provenance
    );
    assert_eq!(
        from_fixture.latest_authoritative_revision_ref,
        constructed.latest_authoritative_revision_ref
    );
    assert_eq!(from_fixture.display_label, constructed.display_label);
    assert_eq!(from_fixture.character_input, constructed.character_input);
}

#[test]
fn sd14_missing_envelope_file_fails_honestly() {
    let root = fresh_temp_dir("sd14-missing-envelope");
    fs::write(
        root.join("authoritative_character_input.txt"),
        include_str!(
            "fixtures/sd14/pf1_human_fighter_level1_saved_character/authoritative_character_input.txt"
        ),
    )
    .expect("should be able to write fixture file");

    let result = SavedCharacterStore::load(&root);
    assert!(result.is_err(), "missing envelope should fail");
    let err = result.unwrap_err();
    assert!(
        err.message.contains("envelope"),
        "error should mention envelope: {}",
        err.message
    );
}

#[test]
fn sd14_missing_character_input_file_fails_honestly() {
    let root = fresh_temp_dir("sd14-missing-character-input");
    fs::write(
        root.join("envelope.txt"),
        include_str!("fixtures/sd14/pf1_human_fighter_level1_saved_character/envelope.txt"),
    )
    .expect("should be able to write fixture file");

    let result = SavedCharacterStore::load(&root);
    assert!(result.is_err(), "missing character input should fail");
    let err = result.unwrap_err();
    assert!(
        err.message.contains("authoritative_character_input"),
        "error should mention authoritative_character_input: {}",
        err.message
    );
}

#[test]
fn sd14_envelope_missing_character_id_fails_honestly() {
    let root = fresh_temp_dir("sd14-missing-character-id");
    fs::write(
        root.join("envelope.txt"),
        "revision_id = sd14.test.rev.1\n\
         revision_kind = authoritative\n\
         saved_at = 2026-06-30T00:00:00Z\n\
         schema_version = 1\n\
         app_or_runtime_version = codex-dev\n\
         content_or_rules_provenance = pf1.core_rulebook\n\
         latest_authoritative_revision_ref = sd14.test.rev.1\n\
         display_label = Test Character\n",
    )
    .expect("should be able to write envelope file");
    fs::write(
        root.join("authoritative_character_input.txt"),
        include_str!(
            "fixtures/sd14/pf1_human_fighter_level1_saved_character/authoritative_character_input.txt"
        ),
    )
    .expect("should be able to write character input file");

    let result = SavedCharacterStore::load(&root);
    assert!(result.is_err(), "missing character_id should fail");
    let err = result.unwrap_err();
    assert!(
        err.message.contains("character_id"),
        "error should mention character_id: {}",
        err.message
    );
}

#[test]
fn sd14_envelope_missing_revision_id_fails_honestly() {
    let root = fresh_temp_dir("sd14-missing-revision-id");
    fs::write(
        root.join("envelope.txt"),
        "character_id = sd14.test\n\
         revision_kind = authoritative\n\
         saved_at = 2026-06-30T00:00:00Z\n\
         schema_version = 1\n\
         app_or_runtime_version = codex-dev\n\
         content_or_rules_provenance = pf1.core_rulebook\n\
         latest_authoritative_revision_ref = sd14.test.rev.1\n\
         display_label = Test Character\n",
    )
    .expect("should be able to write envelope file");
    fs::write(
        root.join("authoritative_character_input.txt"),
        include_str!(
            "fixtures/sd14/pf1_human_fighter_level1_saved_character/authoritative_character_input.txt"
        ),
    )
    .expect("should be able to write character input file");

    let result = SavedCharacterStore::load(&root);
    assert!(result.is_err(), "missing revision_id should fail");
    let err = result.unwrap_err();
    assert!(
        err.message.contains("revision_id"),
        "error should mention revision_id: {}",
        err.message
    );
}

#[test]
fn sd14_save_rejects_selected_choice_that_cannot_round_trip() {
    // The fixture grammar renders a selected choice as
    // `choice={choice_set_id}:{selection_id}` and the loader re-splits on colons,
    // taking the first two segments as choice_set_id and the rest as selection_id.
    // A choice whose ids do not match that segment shape would save fine and then
    // reload as a *different* choice (or fail to load at all). Save must refuse it.
    let root = fresh_temp_dir("sd14-choice-grammar");

    // Loses a segment on reload: ("choice:fighter:bonus_feat", "feat:dodge")
    // would come back as ("choice:fighter", "bonus_feat:feat:dodge").
    let mut envelope = pilot_envelope();
    envelope.character_input.chosen.selected_choices.push(
        codex::rules_core::character_input::SelectedChoice {
            choice_set_id: "choice:fighter:bonus_feat".to_owned(),
            selection_id: "feat:dodge".to_owned(),
        },
    );
    let err = SavedCharacterStore::save(&envelope, &root)
        .expect_err("a selected choice that cannot round-trip must be rejected at save time");
    assert!(
        err.message.contains("choice_set_id"),
        "error must name the offending field: {}",
        err.message
    );

    // Too few segments in total: ("choice:bonus_feat", "dodge") saves as a line the
    // loader rejects as incomplete.
    let mut envelope = pilot_envelope();
    envelope.character_input.chosen.selected_choices.push(
        codex::rules_core::character_input::SelectedChoice {
            choice_set_id: "choice:bonus_feat".to_owned(),
            selection_id: "dodge".to_owned(),
        },
    );
    let err = SavedCharacterStore::save(&envelope, &root)
        .expect_err("a selected choice the loader would reject must be rejected at save time");
    assert!(
        err.message.contains("selection_id"),
        "error must name the offending field: {}",
        err.message
    );

    fs::remove_dir_all(&root).ok();
}

#[test]
fn sd14_save_rejects_newlines_in_character_input_payload_strings() {
    // envelope.txt metadata is already newline-checked; the character-input payload
    // must be too, otherwise one field value can inject arbitrary key=value lines
    // (e.g. a feat id that overrides race_id on reload).
    let root = fresh_temp_dir("sd14-newline-injection");

    let mut envelope = pilot_envelope();
    envelope
        .character_input
        .chosen
        .selected_feats
        .push("feat:power_attack\nrace_id=race:orc".to_owned());

    let err = SavedCharacterStore::save(&envelope, &root)
        .expect_err("a payload string containing a newline must be rejected at save time");
    assert!(
        err.message.contains("newline"),
        "error must explain the newline rejection: {}",
        err.message
    );

    // The malicious payload must not have been persisted.
    assert!(
        !root.join("authoritative_character_input.txt").exists(),
        "no character-input file may be written when validation fails"
    );

    fs::remove_dir_all(&root).ok();
}
