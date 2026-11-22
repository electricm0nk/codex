//! The Cyberpunk RED saved-character store.
//!
//! A CPR character is its own directory under the same characters root as Pathfinder and
//! Starfinder characters (so a character id resolves to its directory the same way for
//! every system). The directory holds these files:
//! - `envelope.txt`: the saved-character envelope's `key = value` grammar
//!   (`saved_character::local_store`), with `game_system = cyberpunk-red`;
//! - `cpr_character_input.txt`: the [`CprCharacterInput`] (`input::render_cpr_character_input`);
//! - `cpr_sheet_choices.txt`, once any is saved: the [`CprSheetChoices`] the sheet composes from
//!   (`input::render_cpr_sheet_choices`). A character without the file has made no choice yet.
//! - `cpr_lifepath.txt`, once saved: the [`CprLifepath`] answers (`input::render_cpr_lifepath`).
//! - `cpr_role_choices.txt`, once saved: the [`CprRoleChoices`] Role Ability sub-choices
//!   (`input::render_cpr_role_choices`).
//! - `cpr_kit.txt`, once saved: the [`CprKit`] starting kit (`input::render_cpr_kit`).
//!
//! It never holds `authoritative_character_input.txt`, the Pathfinder-shaped payload, and
//! `SavedCharacterStore::list_all` skips it (`local_store::owns_payload`).

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use crate::rules_core::cpr::input::{
    parse_cpr_character_input, parse_cpr_kit, parse_cpr_lifepath, parse_cpr_role_choices, parse_cpr_sheet_choices,
    render_cpr_character_input, render_cpr_kit, render_cpr_lifepath, render_cpr_role_choices, render_cpr_sheet_choices,
    CprCharacterInput, CprKit, CprLifepath, CprRoleChoices, CprSheetChoices,
};
use crate::rules_core::game_system::GameSystem;
use crate::saved_character::local_store::peek_game_system;
use crate::saved_character::SavedCharacterListingError;

const ENVELOPE_FILE: &str = "envelope.txt";
const INPUT_FILE: &str = "cpr_character_input.txt";
const CHOICES_FILE: &str = "cpr_sheet_choices.txt";
const LIFEPATH_FILE: &str = "cpr_lifepath.txt";
const ROLE_CHOICES_FILE: &str = "cpr_role_choices.txt";
const KIT_FILE: &str = "cpr_kit.txt";
/// The envelope's `content_or_rules_provenance` for every CPR save: the one book SD-38 ships.
pub const CPR_CONTENT_PROVENANCE: &str = "cyberpunk-red.corebook";
/// Schema version of a CPR saved character (`envelope.txt` + `cpr_character_input.txt`).
pub const CPR_SAVED_CHARACTER_SCHEMA_VERSION: u16 = 1;

/// One saved Cyberpunk RED character: identity, revision and the player's input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprSavedCharacter {
    pub character_id: String,
    pub revision_id: String,
    pub saved_at: String,
    pub schema_version: u16,
    pub app_or_runtime_version: String,
    pub display_label: String,
    pub input: CprCharacterInput,
}

/// Every CPR character under a characters root, plus each CPR directory that failed to load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprCharacterListing {
    pub characters: Vec<CprSavedCharacter>,
    pub unreadable_entries: Vec<SavedCharacterListingError>,
}

pub struct CprCharacterStore;

impl CprCharacterStore {
    /// Writes `envelope.txt` and `cpr_character_input.txt` under `root` (created if absent).
    /// Both files are written to `.tmp` siblings first and renamed into place, so a failed
    /// save leaves the previous save intact.
    pub fn save(character: &CprSavedCharacter, root: &Path) -> Result<(), String> {
        for (field, value) in [
            ("character_id", &character.character_id),
            ("revision_id", &character.revision_id),
            ("saved_at", &character.saved_at),
            ("app_or_runtime_version", &character.app_or_runtime_version),
            ("display_label", &character.display_label),
        ] {
            if value.contains('\n') || value.contains('\r') {
                return Err(format!(
                    "{field} contains a newline/carriage return; cannot be persisted in {ENVELOPE_FILE}"
                ));
            }
        }
        fs::create_dir_all(root).map_err(|err| format!("{}: {err}", root.display()))?;
        let envelope_path = root.join(ENVELOPE_FILE);
        let input_path = root.join(INPUT_FILE);
        let envelope_tmp = write_tmp(&envelope_path, &render_envelope(character))?;
        let input_tmp = write_tmp(&input_path, &render_cpr_character_input(&character.input))?;
        fs::rename(&envelope_tmp, &envelope_path).map_err(|err| format!("{}: {err}", envelope_path.display()))?;
        fs::rename(&input_tmp, &input_path).map_err(|err| format!("{}: {err}", input_path.display()))?;
        Ok(())
    }

    /// Loads the CPR character at `root`. A directory whose envelope names another system
    /// is refused by name; a missing or malformed file is refused with its reason.
    pub fn load(root: &Path) -> Result<CprSavedCharacter, String> {
        let envelope_path = root.join(ENVELOPE_FILE);
        let envelope_text = fs::read_to_string(&envelope_path)
            .map_err(|err| format!("{ENVELOPE_FILE} missing or unreadable at {}: {err}", envelope_path.display()))?;
        let envelope = parse_envelope(&envelope_text)?;
        let cpr = GameSystem::CyberpunkRed.id();
        if envelope.game_system != cpr {
            return Err(format!("{}: game_system is '{}', not '{cpr}'", root.display(), envelope.game_system));
        }
        let input_path = root.join(INPUT_FILE);
        let input_text = fs::read_to_string(&input_path)
            .map_err(|err| format!("{INPUT_FILE} missing or unreadable at {}: {err}", input_path.display()))?;
        Ok(CprSavedCharacter {
            character_id: envelope.character_id,
            revision_id: envelope.revision_id,
            saved_at: envelope.saved_at,
            schema_version: envelope.schema_version,
            app_or_runtime_version: envelope.app_or_runtime_version,
            display_label: envelope.display_label,
            input: parse_cpr_character_input(&input_text)?,
        })
    }

    /// Writes `cpr_sheet_choices.txt` for the CPR character at `root` (temp file, then renamed
    /// into place). Refused when `root` holds no CPR character: choices never exist without one.
    pub fn save_choices(choices: &CprSheetChoices, root: &Path) -> Result<(), String> {
        Self::load(root)?;
        let path = root.join(CHOICES_FILE);
        let tmp = write_tmp(&path, &render_cpr_sheet_choices(choices))?;
        fs::rename(&tmp, &path).map_err(|err| format!("{}: {err}", path.display()))
    }

    /// The sheet choices of the CPR character at `root`; none when the character has saved none
    /// (no `cpr_sheet_choices.txt`). A malformed file is refused with its reason.
    pub fn load_choices(root: &Path) -> Result<CprSheetChoices, String> {
        let path = root.join(CHOICES_FILE);
        match fs::read_to_string(&path) {
            Ok(text) => parse_cpr_sheet_choices(&text),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(CprSheetChoices::default()),
            Err(err) => Err(format!("{CHOICES_FILE} unreadable at {}: {err}", path.display())),
        }
    }

    /// Writes `cpr_lifepath.txt` for the CPR character at `root` (temp file, then renamed into
    /// place). Refused when `root` holds no CPR character.
    pub fn save_lifepath(lifepath: &CprLifepath, root: &Path) -> Result<(), String> {
        Self::load(root)?;
        let path = root.join(LIFEPATH_FILE);
        let tmp = write_tmp(&path, &render_cpr_lifepath(lifepath))?;
        fs::rename(&tmp, &path).map_err(|err| format!("{}: {err}", path.display()))
    }

    /// The Lifepath answers of the CPR character at `root`; none when it has saved none (no
    /// `cpr_lifepath.txt`). A malformed file is refused with its reason.
    pub fn load_lifepath(root: &Path) -> Result<CprLifepath, String> {
        let path = root.join(LIFEPATH_FILE);
        match fs::read_to_string(&path) {
            Ok(text) => parse_cpr_lifepath(&text),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(CprLifepath::default()),
            Err(err) => Err(format!("{LIFEPATH_FILE} unreadable at {}: {err}", path.display())),
        }
    }

    /// Writes `cpr_role_choices.txt` for the CPR character at `root` (temp file, then renamed
    /// into place). Refused when `root` holds no CPR character.
    pub fn save_role_choices(choices: &CprRoleChoices, root: &Path) -> Result<(), String> {
        Self::load(root)?;
        let path = root.join(ROLE_CHOICES_FILE);
        let tmp = write_tmp(&path, &render_cpr_role_choices(choices))?;
        fs::rename(&tmp, &path).map_err(|err| format!("{}: {err}", path.display()))
    }

    /// The Role Ability sub-choices of the CPR character at `root`; none when it has saved none
    /// (no `cpr_role_choices.txt`). A malformed file is refused with its reason.
    pub fn load_role_choices(root: &Path) -> Result<CprRoleChoices, String> {
        let path = root.join(ROLE_CHOICES_FILE);
        match fs::read_to_string(&path) {
            Ok(text) => parse_cpr_role_choices(&text),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(CprRoleChoices::default()),
            Err(err) => Err(format!("{ROLE_CHOICES_FILE} unreadable at {}: {err}", path.display())),
        }
    }

    /// Writes `cpr_kit.txt` for the CPR character at `root` (temp file, then renamed into
    /// place). Refused when `root` holds no CPR character.
    pub fn save_kit(kit: &CprKit, root: &Path) -> Result<(), String> {
        Self::load(root)?;
        let path = root.join(KIT_FILE);
        let tmp = write_tmp(&path, &render_cpr_kit(kit))?;
        fs::rename(&tmp, &path).map_err(|err| format!("{}: {err}", path.display()))
    }

    /// The starting kit of the CPR character at `root`; none when it has saved none (no
    /// `cpr_kit.txt`). A malformed file is refused with its reason.
    pub fn load_kit(root: &Path) -> Result<CprKit, String> {
        let path = root.join(KIT_FILE);
        match fs::read_to_string(&path) {
            Ok(text) => parse_cpr_kit(&text),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(CprKit::default()),
            Err(err) => Err(format!("{KIT_FILE} unreadable at {}: {err}", path.display())),
        }
    }

    /// Every CPR character directly under `characters_root`, sorted by directory name. A
    /// directory of another system (or with no readable envelope) is not this store's and is
    /// skipped; a CPR directory that fails to load is reported in `unreadable_entries`. A
    /// root that does not exist lists nothing.
    pub fn list_all(characters_root: &Path) -> Result<CprCharacterListing, String> {
        let entries = match fs::read_dir(characters_root) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(CprCharacterListing { characters: Vec::new(), unreadable_entries: Vec::new() });
            }
            Err(err) => return Err(format!("{}: {err}", characters_root.display())),
        };
        let mut directories: Vec<PathBuf> =
            entries.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.is_dir()).collect();
        directories.sort();
        let mut listing = CprCharacterListing { characters: Vec::new(), unreadable_entries: Vec::new() };
        for directory in directories {
            if peek_game_system(&directory).ok().as_deref() != Some(GameSystem::CyberpunkRed.id()) {
                continue;
            }
            match Self::load(&directory) {
                Ok(character) => listing.characters.push(character),
                Err(message) => listing.unreadable_entries.push(SavedCharacterListingError {
                    entry_name: directory.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
                    message,
                }),
            }
        }
        Ok(listing)
    }

    /// Removes the character's directory. Deleting a directory that does not exist succeeds.
    pub fn delete(root: &Path) -> Result<(), String> {
        match fs::remove_dir_all(root) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(format!("{}: {err}", root.display())),
        }
    }
}

fn write_tmp(path: &Path, contents: &str) -> Result<PathBuf, String> {
    let mut name = path.file_name().map(|n| n.to_os_string()).unwrap_or_default();
    name.push(".tmp");
    let tmp = path.with_file_name(name);
    fs::write(&tmp, contents).map_err(|err| format!("{}: {err}", tmp.display()))?;
    Ok(tmp)
}

/// The envelope in `saved_character::local_store`'s grammar and key order.
fn render_envelope(character: &CprSavedCharacter) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "character_id = {}", character.character_id);
    let _ = writeln!(out, "revision_id = {}", character.revision_id);
    let _ = writeln!(out, "revision_kind = authoritative");
    let _ = writeln!(out, "saved_at = {}", character.saved_at);
    let _ = writeln!(out, "schema_version = {}", character.schema_version);
    let _ = writeln!(out, "app_or_runtime_version = {}", character.app_or_runtime_version);
    let _ = writeln!(out, "content_or_rules_provenance = {CPR_CONTENT_PROVENANCE}");
    let _ = writeln!(out, "game_system = {}", GameSystem::CyberpunkRed.id());
    let _ = writeln!(out, "latest_authoritative_revision_ref = {}", character.revision_id);
    let _ = writeln!(out, "display_label = {}", character.display_label);
    out
}

struct CprEnvelope {
    character_id: String,
    revision_id: String,
    saved_at: String,
    schema_version: u16,
    app_or_runtime_version: String,
    game_system: String,
    display_label: String,
}

/// Parses the envelope. Every key the grammar defines is accepted (an unknown key is
/// refused, as `local_store` refuses it); the ones a CPR character carries are required.
fn parse_envelope(text: &str) -> Result<CprEnvelope, String> {
    let mut fields: std::collections::BTreeMap<&str, &str> = std::collections::BTreeMap::new();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("envelope line missing '=': {raw_line}"));
        };
        let key = key.trim();
        match key {
            "character_id" | "revision_id" | "revision_kind" | "saved_at" | "schema_version"
            | "app_or_runtime_version" | "content_or_rules_provenance" | "game_system"
            | "latest_authoritative_revision_ref" | "display_label" => {
                fields.insert(key, value.trim());
            }
            unknown => return Err(format!("envelope unsupported field '{unknown}'")),
        }
    }
    let required = |key: &str| {
        fields
            .get(key)
            .map(|v| (*v).to_owned())
            .ok_or_else(|| format!("envelope missing required field {key}"))
    };
    let schema_version = required("schema_version")?;
    Ok(CprEnvelope {
        character_id: required("character_id")?,
        revision_id: required("revision_id")?,
        saved_at: required("saved_at")?,
        schema_version: schema_version
            .parse()
            .map_err(|_| format!("envelope schema_version '{schema_version}' is not a valid version number"))?,
        app_or_runtime_version: required("app_or_runtime_version")?,
        game_system: required("game_system")?,
        display_label: required("display_label")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::cpr::input::{CprStat, CprStats};
    use crate::saved_character::local_store::SavedCharacterStore;

    fn tempdir(label: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codex-cpr-store-{label}-{}-{unique}", std::process::id()));
        fs::create_dir_all(&path).expect("temp dir should be creatable");
        path
    }

    pub(crate) fn sample(character_id: &str) -> CprSavedCharacter {
        let mut stats = CprStats::default();
        for (i, stat) in CprStat::ALL.into_iter().enumerate() {
            stats.set(stat, 2 + (i as u8 % 7));
        }
        CprSavedCharacter {
            character_id: character_id.to_owned(),
            revision_id: format!("{character_id}.rev.1"),
            saved_at: "2026-10-13T00:00:00Z".to_owned(),
            schema_version: CPR_SAVED_CHARACTER_SCHEMA_VERSION,
            app_or_runtime_version: "codex-dev".to_owned(),
            display_label: "Store Test Edgerunner".to_owned(),
            input: CprCharacterInput { stats },
        }
    }

    #[test]
    fn save_then_load_round_trips_and_writes_the_two_files() {
        let root = tempdir("round-trip").join("cpr-one");
        let character = sample("cpr-one");
        CprCharacterStore::save(&character, &root).expect("saves");

        let mut files: Vec<String> = fs::read_dir(&root)
            .expect("reads")
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        files.sort();
        assert_eq!(files, ["cpr_character_input.txt", "envelope.txt"]);
        let envelope = fs::read_to_string(root.join("envelope.txt")).expect("reads");
        assert!(envelope.contains("\ngame_system = cyberpunk-red\n"), "{envelope}");
        assert_eq!(
            crate::saved_character::local_store::peek_game_system(&root).expect("peeks"),
            "cyberpunk-red"
        );

        assert_eq!(CprCharacterStore::load(&root), Ok(character));
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }

    #[test]
    fn load_refuses_a_directory_of_another_system() {
        let root = tempdir("other-system").join("pf");
        let pf_fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
        let pf = SavedCharacterStore::load(&pf_fixture).expect("the PF fixture loads");
        SavedCharacterStore::save(&pf, &root).expect("saves");

        assert_eq!(
            CprCharacterStore::load(&root),
            Err(format!("{}: game_system is 'pf1', not 'cyberpunk-red'", root.display()))
        );
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }

    #[test]
    fn list_all_lists_cpr_characters_only_and_reports_a_corrupt_one() {
        let characters_root = tempdir("list-all");
        let pf_fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
        let pf = SavedCharacterStore::load(&pf_fixture).expect("the PF fixture loads");
        SavedCharacterStore::save(&pf, &characters_root.join("a-pf")).expect("saves");
        CprCharacterStore::save(&sample("b-cpr"), &characters_root.join("b-cpr")).expect("saves");
        let corrupt = characters_root.join("c-cpr-corrupt");
        CprCharacterStore::save(&sample("c-cpr-corrupt"), &corrupt).expect("saves");
        fs::write(corrupt.join("cpr_character_input.txt"), "stat.int = 6\n").expect("writes");

        let listing = CprCharacterStore::list_all(&characters_root).expect("lists");
        let ids: Vec<&str> = listing.characters.iter().map(|c| c.character_id.as_str()).collect();
        assert_eq!(ids, ["b-cpr"]);
        assert_eq!(listing.unreadable_entries.len(), 1);
        assert_eq!(listing.unreadable_entries[0].entry_name, "c-cpr-corrupt");
        assert_eq!(listing.unreadable_entries[0].message, "cpr_character_input.txt is missing stat.ref");

        assert_eq!(
            CprCharacterStore::list_all(&characters_root.join("absent")).expect("lists").characters.len(),
            0
        );
        fs::remove_dir_all(&characters_root).ok();
    }

    /// The sheet choices are their own file: a character saved before any choice (the STAT
    /// step) loads with none; saved choices round-trip; a re-save of the character keeps them;
    /// choices for a directory that holds no CPR character are refused.
    #[test]
    fn sheet_choices_save_load_and_survive_a_character_re_save() {
        use crate::rules_core::cpr::input::CprSheetChoices;
        use crate::rules_core::cpr::roles::CprRole;
        let root = tempdir("choices").join("cpr-choices");
        let character = sample("cpr-choices");
        CprCharacterStore::save(&character, &root).expect("saves");
        assert_eq!(CprCharacterStore::load_choices(&root), Ok(CprSheetChoices::default()));

        let choices = CprSheetChoices {
            role: Some((CprRole::Fixer, 4)),
            skills: vec![("Trading".to_owned(), 4)],
            cash: Some(120),
            ..CprSheetChoices::default()
        };
        CprCharacterStore::save_choices(&choices, &root).expect("saves the choices");
        assert_eq!(fs::read_to_string(root.join("cpr_sheet_choices.txt")).expect("reads"), "role = Fixer\nrole.rank = 4\nskill.Trading = 4\ncash = 120\n");
        CprCharacterStore::save(&character, &root).expect("re-saves");
        assert_eq!(CprCharacterStore::load_choices(&root), Ok(choices.clone()));
        assert_eq!(CprCharacterStore::load(&root), Ok(character));

        let empty = root.parent().expect("has parent").join("no-character");
        assert!(CprCharacterStore::save_choices(&choices, &empty).is_err_and(|e| e.contains("envelope.txt")));
        assert!(!empty.join("cpr_sheet_choices.txt").exists());
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }

    /// The starting kit is its own file (E3.6): none before any is saved; a saved kit
    /// round-trips and survives a re-save of the character; refused without a character.
    #[test]
    fn kit_saves_loads_and_survives_a_character_re_save() {
        use crate::rules_core::cpr::input::{CprKit, CprKitEntry};
        let root = tempdir("kit").join("cpr-kit");
        let character = sample("cpr-kit");
        CprCharacterStore::save(&character, &root).expect("saves");
        assert_eq!(CprCharacterStore::load_kit(&root), Ok(CprKit::default()));
        let kit = CprKit { items: vec![CprKitEntry { section: "outfit".to_owned(), name: "Agent".to_owned(), quantity: 1 }] };
        CprCharacterStore::save_kit(&kit, &root).expect("saves the kit");
        assert!(root.join("cpr_kit.txt").exists());
        CprCharacterStore::save(&character, &root).expect("re-saves");
        assert_eq!(CprCharacterStore::load_kit(&root), Ok(kit.clone()));

        let empty = root.parent().expect("has parent").join("no-character");
        assert!(CprCharacterStore::save_kit(&kit, &empty).is_err_and(|e| e.contains("envelope.txt")));
        assert!(!empty.join("cpr_kit.txt").exists());
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }

    /// The Role Ability sub-choices are their own file (E3.5): none before any is saved; saved
    /// choices round-trip and survive a re-save of the character; refused without a character.
    #[test]
    fn role_choices_save_load_and_survive_a_character_re_save() {
        use crate::rules_core::cpr::input::{CprRoleChoices, CprRolePick};
        let root = tempdir("role-choices").join("cpr-role-choices");
        let character = sample("cpr-role-choices");
        CprCharacterStore::save(&character, &root).expect("saves");
        assert_eq!(CprCharacterStore::load_role_choices(&root), Ok(CprRoleChoices::default()));
        let choices = CprRoleChoices {
            points: vec![("Surgery".to_owned(), 3), ("Medical Tech (Pharmaceuticals)".to_owned(), 1)],
            picks: vec![CprRolePick { name: "Speedheal".to_owned(), on: None }],
        };
        CprCharacterStore::save_role_choices(&choices, &root).expect("saves the choices");
        assert!(root.join("cpr_role_choices.txt").exists());
        CprCharacterStore::save(&character, &root).expect("re-saves");
        assert_eq!(CprCharacterStore::load_role_choices(&root), Ok(choices.clone()));
        assert_eq!(CprCharacterStore::load(&root), Ok(character));

        let empty = root.parent().expect("has parent").join("no-character");
        assert!(CprCharacterStore::save_role_choices(&choices, &empty).is_err_and(|e| e.contains("envelope.txt")));
        assert!(!empty.join("cpr_role_choices.txt").exists());
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }

    /// The Lifepath answers are their own file: none before any is saved; saved answers
    /// round-trip and survive a re-save of the character; refused without a character.
    #[test]
    fn lifepath_saves_loads_and_survives_a_character_re_save() {
        use crate::rules_core::cpr::input::{CprLifepath, CprLifepathAnswer};
        let root = tempdir("lifepath").join("cpr-lifepath");
        let character = sample("cpr-lifepath");
        CprCharacterStore::save(&character, &root).expect("saves");
        assert_eq!(CprCharacterStore::load_lifepath(&root), Ok(CprLifepath::default()));
        let lifepath = CprLifepath {
            answers: vec![CprLifepathAnswer {
                table: "Your Life Goals".to_owned(),
                entry: 0,
                column: "Life Goal".to_owned(),
                text: "Get rid of a bad reputation.".to_owned(),
                detail: String::new(),
            }],
        };
        CprCharacterStore::save_lifepath(&lifepath, &root).expect("saves the answers");
        assert!(root.join("cpr_lifepath.txt").exists());
        CprCharacterStore::save(&character, &root).expect("re-saves");
        assert_eq!(CprCharacterStore::load_lifepath(&root), Ok(lifepath.clone()));
        assert_eq!(CprCharacterStore::load(&root), Ok(character));

        let empty = root.parent().expect("has parent").join("no-character");
        assert!(CprCharacterStore::save_lifepath(&lifepath, &empty).is_err_and(|e| e.contains("envelope.txt")));
        assert!(!empty.join("cpr_lifepath.txt").exists());
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }

    #[test]
    fn delete_removes_the_directory_and_is_idempotent() {
        let root = tempdir("delete").join("cpr-gone");
        CprCharacterStore::save(&sample("cpr-gone"), &root).expect("saves");
        CprCharacterStore::delete(&root).expect("deletes");
        assert!(!root.exists());
        CprCharacterStore::delete(&root).expect("a second delete is a no-op");
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }

    #[test]
    fn save_refuses_a_field_with_a_newline() {
        let root = tempdir("newline").join("cpr-nl");
        let mut character = sample("cpr-nl");
        character.display_label = "two\nlines".to_owned();
        assert_eq!(
            CprCharacterStore::save(&character, &root),
            Err("display_label contains a newline/carriage return; cannot be persisted in envelope.txt".to_owned())
        );
        fs::remove_dir_all(root.parent().expect("has parent")).ok();
    }
}
