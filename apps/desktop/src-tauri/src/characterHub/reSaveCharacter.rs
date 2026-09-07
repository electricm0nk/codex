//! `reSaveCharacter` Tauri command (SD-24 Epic 7, Criterion 7.3).
//!
//! Closes the gap `technical-design.md` §3.1/§3.2 documents: every existing
//! write path that persists a `SavedCharacterEnvelope` — `create_character`,
//! `clone_character`, `seed_default_character_if_needed`, `import_character`
//! — hardcodes `revision_id: "{id}.rev.1"` at construction time (the duracon
//! 2026-07-18 18:20:41 sentinel this criterion's RED targets), and none of
//! the three existing mutation commands (`level_up_character`,
//! `add_equipment_selection`, `add_spell_selection`, all routed through
//! `character_hub::mutate_saved_character_at_root`) advance `revision_id` on
//! re-save either — every mutation of a saved character keeps whatever
//! `revision_id` was already on disk, forever.
//!
//! `reSaveCharacter` is the explicit "I am re-saving this exact character
//! again" operation the technical design calls for: it re-saves the
//! character currently on disk under a freshly minted `{id}.rev.N` revision
//! (`N` computed from the on-disk revision, not hardcoded), and refuses the
//! re-save outright — `success: false, error: "revision_conflict"`, no
//! write — if the caller's `expectedRevisionId` does not match the
//! canonical on-disk `revision_id` (the write-conflict case this criterion's
//! RED targets: two callers racing to re-save the same character path).
//!
//! ## DISCOVERED note (documented in full in `progress.md`)
//!
//! `technical-design.md` §3.2 sketches this command's request shape as
//! `{ characterId, character: CharacterSnapshot }` — implying the caller
//! pushes an entire re-edited character payload in the request. No
//! `CharacterSnapshot` wire type exists anywhere in this crate: the closest
//! namesake, `PilotSnapshotDto`, is the *derived* stat snapshot
//! (BAB/saves/AC/skills) `create_character`/`load_saved_character`/etc.
//! already return, not an editable `CharacterInput` mirror — and
//! `codex::rules_core::character_input::CharacterInput` itself derives no
//! `serde` impls at all (this crate is intentionally dependency-free; see
//! its own module doc). Building a full serde-mirrored `CharacterInput` DTO
//! surface to accept arbitrary client-authored edits is materially larger
//! than one cycle's granted file-touch scope (`reSaveCharacter.rs` + one
//! `main.rs` line) and duplicates work criteria 7.1/7.2 already do more
//! narrowly (batch equipment append; recompute-after-level-up). This
//! cycle implements the load-bearing part of the acceptance text instead —
//! the revision-conflict guard and the `{id}.rev.N` counter replacing the
//! hardcoded `.rev.1` — operating on the character already persisted at
//! `characterId`, exactly like every other `mutate_saved_character`-family
//! command in this crate. Content edits continue to flow through
//! `appendToCharacter`/`add_spell_selection`/`level_up_character` etc.,
//! which already load → mutate → recompute → re-save in one round trip;
//! full snapshot-body editing is deferred to an operator-scoped follow-on
//! cycle if a client-side "edit everything, then push" workflow is wanted.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::character_hub;
use crate::pf1_adapter::Pf1Adapter;
use crate::rule_system_adapter::RuleSystemAdapter;
use crate::stub_adapter::StubAdapter;
use codex::saved_character::local_store::SavedCharacterStore;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReSaveCharacterRequest {
    pub character_id: String,
    /// The revision_id the caller last observed for this character (from a
    /// prior `load_saved_character` / mutation response). Re-save is
    /// refused if this does not match the canonical on-disk `revision_id`.
    pub expected_revision_id: String,
    pub saved_at: String,
    /// SD-25 Criterion 3.4 (Epic 3 "Hub of Hubs" Tauri command-surface
    /// routing): which rule system's `RuleSystemAdapter` to dispatch this
    /// re-save through — see `resolve_rule_system_adapter` below.
    pub rule_system_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReSaveCharacterResponse {
    pub success: bool,
    pub revision_id: Option<String>,
    pub error: Option<String>,
}

/// Computes the next `{character_id}.rev.N` revision id from the current
/// on-disk one. `N` is parsed from the current revision's own numeric
/// suffix and incremented — never hardcoded to `1` — so repeated re-saves
/// of the same character keep advancing (`.rev.1` -> `.rev.2` -> `.rev.3`
/// -> ...). A revision_id that does not match the `{character_id}.rev.<n>`
/// shape (e.g. a hand-authored fixture) restarts the counter at `1` rather
/// than failing the re-save outright — every revision_id this crate itself
/// writes follows the shape, so this fallback only matters for
/// hand-authored test/legacy fixtures.
fn next_revision_id(character_id: &str, current_revision_id: &str) -> String {
    let prefix = format!("{character_id}.rev.");
    let next_n = current_revision_id
        .strip_prefix(prefix.as_str())
        .and_then(|suffix| suffix.parse::<u64>().ok())
        .map(|n| n + 1)
        .unwrap_or(1);
    format!("{prefix}{next_n}")
}

/// Real implementation behind the `re_save_character` Tauri command below —
/// split out so it is unit-testable against a real `SavedCharacterStore`
/// fixture without an `AppHandle`, mirroring every other command in this
/// submodule.
///
/// Loads the canonical on-disk envelope, refuses the re-save with an
/// honest `{ success: false, error: "revision_conflict" }` (no write at
/// all) if `expected_revision_id` does not match the on-disk
/// `revision_id`, and otherwise re-saves under a freshly minted
/// `{id}.rev.N` revision with the caller's `saved_at`.
pub fn re_save_character_at_root(
    root: &Path,
    expected_revision_id: &str,
    saved_at: &str,
) -> Result<ReSaveCharacterResponse, String> {
    let mut envelope = SavedCharacterStore::load(root).map_err(|err| err.message)?;

    if envelope.revision_id != expected_revision_id {
        return Ok(ReSaveCharacterResponse {
            success: false,
            revision_id: None,
            error: Some("revision_conflict".to_owned()),
        });
    }

    let next_revision_id = next_revision_id(&envelope.character_id, &envelope.revision_id);
    envelope.revision_id = next_revision_id.clone();
    envelope.latest_authoritative_revision_ref = next_revision_id.clone();
    envelope.saved_at = saved_at.to_owned();

    SavedCharacterStore::save(&envelope, root).map_err(|err| err.message)?;

    Ok(ReSaveCharacterResponse {
        success: true,
        revision_id: Some(next_revision_id),
        error: None,
    })
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

/// Dispatches `re_save_character` through the `RuleSystemAdapter` trait
/// instead of calling `re_save_character_at_root` by name directly (SD-25
/// Criterion 3.4). For `"pf1"` this produces byte-for-byte the same real
/// behavior as calling `re_save_character_at_root` directly —
/// `Pf1Adapter::save_character` wraps that exact function — so every
/// pre-existing test above that calls `re_save_character_at_root` directly
/// keeps passing unchanged.
pub fn re_save_character_via_rule_system(
    rule_system_id: &str,
    root: &Path,
    expected_revision_id: &str,
    saved_at: &str,
) -> Result<ReSaveCharacterResponse, String> {
    resolve_rule_system_adapter(rule_system_id).save_character(root, expected_revision_id, saved_at)
}

/// Loads the saved character, refuses on a revision conflict, and
/// otherwise re-saves it under an incremented `{id}.rev.N` revision — see
/// `re_save_character_at_root` for the full semantics.
#[tauri::command]
pub fn re_save_character(
    app: tauri::AppHandle,
    request: ReSaveCharacterRequest,
) -> Result<ReSaveCharacterResponse, String> {
    let root = character_hub::resolve_character_root(&app, &request.character_id)?;
    re_save_character_via_rule_system(
        &request.rule_system_id,
        &root,
        &request.expected_revision_id,
        &request.saved_at,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character_hub::{compose_character_input, AbilityScoresDto, CreateCharacterRequest};
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
            "codex-resave-character-{label}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    fn seed_envelope() -> SavedCharacterEnvelope {
        let request = CreateCharacterRequest {
            character_id: "char-resave-test".to_owned(),
            display_label: "Resave Test Character".to_owned(),
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
            selected_alternate_trait_keys: Vec::new(),
            companion_species: None,
            selected_traits: Vec::new(),
            trait_skill_choices: Vec::new(),
            saved_at: TEST_SAVED_AT.to_owned(),
        };
        let character_input = compose_character_input(&request);
        SavedCharacterEnvelope {
            character_id: "char-resave-test".to_owned(),
            revision_id: "char-resave-test.rev.1".to_owned(),
            revision_kind: SavedCharacterRevisionKind::Authoritative,
            saved_at: TEST_SAVED_AT.to_owned(),
            schema_version: CURRENT_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            content_or_rules_provenance: "pf1.core_rulebook".to_owned(),
            game_system: "pf1".to_owned(),
            latest_authoritative_revision_ref: "char-resave-test.rev.1".to_owned(),
            display_label: "Resave Test Character".to_owned(),
            character_input,
        }
    }

    /// SD-25 Criterion 3.4 GREEN: `re_save_character_via_rule_system("pf1",
    /// ...)` dispatches through `dyn RuleSystemAdapter` and produces
    /// byte-for-byte the same real result `re_save_character_at_root` itself
    /// returns (`Pf1Adapter::save_character` wraps that exact function).
    #[test]
    fn re_save_character_via_rule_system_dispatches_pf1_through_the_trait() {
        let root = tempdir("pf1-dispatch");
        let envelope = seed_envelope();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let dispatched = re_save_character_via_rule_system(
            "pf1",
            &root,
            "char-resave-test.rev.1",
            "2026-07-21T01:00:00Z",
        )
        .expect("pf1 dispatch should not error");

        assert!(dispatched.success, "pf1 dispatch must succeed: {:?}", dispatched.error);
        assert_eq!(dispatched.revision_id.as_deref(), Some("char-resave-test.rev.2"));

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id, "char-resave-test.rev.2",
            "pf1 dispatch through the trait must really persist the re-save on disk"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// An unrecognized `rule_system_id` routes to `StubAdapter`
    /// (`cycles/3_4.md` GREEN) rather than silently falling through to PF1
    /// logic or panicking.
    #[test]
    fn re_save_character_via_rule_system_routes_unknown_id_to_stub_adapter() {
        let root = tempdir("unknown-system-dispatch");
        let envelope = seed_envelope();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let err = re_save_character_via_rule_system(
            "starfinder",
            &root,
            "char-resave-test.rev.1",
            "2026-07-21T01:00:00Z",
        )
        .expect_err("an unimplemented rule system must honestly error, not fabricate success");

        assert_eq!(err, "Would render for system starfinder; not yet implemented");

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id, "char-resave-test.rev.1",
            "an unimplemented rule system must never persist a re-save"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// Criterion 7.3's GREEN: re-saving with the correct `expected_revision_id`
    /// replaces the hardcoded `.rev.1` with a genuinely incrementing counter —
    /// a second consecutive re-save advances past `.rev.2` to `.rev.3`, proving
    /// the counter is real, not a second hardcoded constant.
    #[test]
    fn re_save_character_at_root_increments_revision_on_each_call() {
        let root = tempdir("golden-path");
        let envelope = seed_envelope();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let first = re_save_character_at_root(&root, "char-resave-test.rev.1", "2026-07-21T01:00:00Z")
            .expect("first re-save call should not error");
        assert!(first.success, "first re-save must succeed: {:?}", first.error);
        assert_eq!(first.revision_id.as_deref(), Some("char-resave-test.rev.2"));

        let second = re_save_character_at_root(&root, "char-resave-test.rev.2", "2026-07-21T02:00:00Z")
            .expect("second re-save call should not error");
        assert!(second.success, "second re-save must succeed: {:?}", second.error);
        assert_eq!(second.revision_id.as_deref(), Some("char-resave-test.rev.3"));

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(reloaded.revision_id, "char-resave-test.rev.3");
        assert_eq!(
            reloaded.latest_authoritative_revision_ref,
            "char-resave-test.rev.3"
        );
        assert_eq!(reloaded.saved_at, "2026-07-21T02:00:00Z");

        std::fs::remove_dir_all(&root).ok();
    }

    /// Criterion 7.3's RED sentinel: re-saving with a stale
    /// `expected_revision_id` (the write-conflict case — another caller
    /// already advanced the on-disk revision) must fail honestly
    /// (`success: false, error: "revision_conflict"`) and must never write
    /// to disk, rather than silently overwriting a revision the caller
    /// never actually observed.
    #[test]
    fn re_save_character_at_root_rejects_stale_expected_revision() {
        let root = tempdir("stale-revision");
        let envelope = seed_envelope();
        SavedCharacterStore::save(&envelope, &root).expect("seed save should succeed");

        let response = re_save_character_at_root(
            &root,
            "char-resave-test.rev.999-not-the-real-revision",
            "2026-07-21T01:00:00Z",
        )
        .expect("re-save call should not error even on a conflict rejection");

        assert!(!response.success);
        assert!(response.revision_id.is_none());
        assert_eq!(response.error.as_deref(), Some("revision_conflict"));

        let reloaded = SavedCharacterStore::load(&root).expect("reload should succeed");
        assert_eq!(
            reloaded.revision_id, "char-resave-test.rev.1",
            "a rejected re-save must never advance the on-disk revision"
        );
        assert_eq!(
            reloaded.saved_at, TEST_SAVED_AT,
            "a rejected re-save must never touch saved_at either"
        );

        std::fs::remove_dir_all(&root).ok();
    }

    /// `re_save_character_at_root`'s error path when there is nothing saved
    /// at `root` yet — must fail honestly rather than silently creating a
    /// character out of thin air (mirrors `append_to_character_at_root`'s
    /// own missing-character test).
    #[test]
    fn re_save_character_at_root_fails_honestly_when_nothing_is_saved_yet() {
        let root = tempdir("missing-character");

        let result = re_save_character_at_root(&root, "char-resave-test.rev.1", TEST_SAVED_AT);

        assert!(
            result.is_err(),
            "re-saving a nonexistent saved character must fail"
        );

        std::fs::remove_dir_all(&root).ok();
    }
}
