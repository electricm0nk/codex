//! The two-master checks on the desktop's writers (SD-42 `decisions.md §16`, E4.3).
//!
//! Two devices are two app-data roots (each mints its own `device.json`); a campaign is a temp
//! folder. Device A's character is written by the app's own writers (`set_character_campaigns_at_root`,
//! `save_character_bio_at_root`, both through `after_write`), and device B's copy comes from the
//! Characters list's import (`characters::import`, which `import_campaign_character` runs; this
//! module is also compiled into `tests/old_save_loads_unchanged.rs`, which has no `campaign_folder`).
//!
//! **Does not cover:** a sync client renaming mid-write (SD-r, `campaign_mirror`'s own test); the
//! Starfinder and Cyberpunk RED writers (the same `after_write`, held to it by
//! `tests/every_character_root_writer_mirrors.rs`); the screens.

use std::path::{Path, PathBuf};

use codex::campaign::characters::{self, ImportMode};
use codex::saved_character::campaign_mirror::{self, Membership};
use codex::saved_character::local_store::SavedCharacterStore;

use super::*;

const FOLDER: &str = "aldric-char-1";

fn tempdir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("codex-two-master-{label}-{}-{unique}", std::process::id()));
    std::fs::create_dir_all(&path).unwrap();
    path
}

/// Device A (app data `<base>/app-a`) holds Aldric and has joined `<base>/campaign`; its first
/// copy is written. Returns (base, campaign, A's character root).
fn device_a_in_a_campaign(label: &str) -> (PathBuf, PathBuf, PathBuf) {
    let base = tempdir(label);
    let campaign = base.join("campaign");
    std::fs::create_dir_all(&campaign).unwrap();
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures/sd14/pf1_human_fighter_level1_saved_character");
    let mut envelope = SavedCharacterStore::load(&fixture).expect("fixture loads");
    envelope.character_id = "char-1".to_owned();
    envelope.display_label = "Aldric".to_owned();
    let root = characters_root_from_app_data_dir(&base.join("app-a")).join("char-1");
    std::fs::create_dir_all(&root).unwrap();
    assert!(save_and_mirror(&envelope, &root).unwrap().is_empty(), "no campaign yet, nothing to warn");
    let warnings = set_character_campaigns_at_root(&root, &[joined(&campaign)]).expect("joins");
    assert!(warnings.is_empty(), "{warnings:?}");
    (base, campaign, root)
}

/// The membership as the sheet's dialog sends it for a newly checked folder.
fn joined(campaign: &Path) -> Membership {
    Membership { campaign_id: campaign.display().to_string(), campaign_name: "Runelords".to_owned(), campaign_dir: campaign.to_path_buf(), active: true, stand_in: false, last_backup_counter: None }
}

fn backup(campaign: &Path) -> PathBuf {
    campaign.join(campaign_mirror::BACKUP_DIR).join(FOLDER)
}

fn counter(campaign: &Path) -> u64 {
    campaign_mirror::read_master(&backup(campaign)).unwrap().expect("master.json").save_counter
}

fn bio(alignment: &str) -> CharacterBioDto {
    CharacterBioDto { alignment: alignment.to_owned(), ..Default::default() }
}

fn import_on(app_data: &Path, campaign: &Path, mode: ImportMode) -> PathBuf {
    let characters_root = characters_root_from_app_data_dir(app_data);
    let imported = characters::import(campaign, FOLDER, &characters_root, mode).expect("imports");
    characters_root.join(imported.character_id)
}

#[test]
fn save_writes_backup_and_bumps_counter() {
    let (base, campaign, root) = device_a_in_a_campaign("bump");
    assert_eq!(counter(&campaign), 1, "joining writes the first copy");
    // The membership as the sheet loaded it, before the next save moved the counter on.
    let as_loaded = campaign_mirror::load_memberships(&root).unwrap();

    save_character_bio_at_root(&root, &bio("Lawful Good")).expect("saves");
    assert_eq!(counter(&campaign), 2, "a save bumps the counter");
    assert_eq!(std::fs::read(backup(&campaign).join("bio.json")).unwrap(), std::fs::read(root.join("bio.json")).unwrap(), "and copies the file");
    let master = campaign_mirror::read_master(&backup(&campaign)).unwrap().unwrap();
    assert_eq!(master.device_id, campaign_mirror::device_identity(&base.join("app-a")).unwrap().id, "master.json names this device");

    // The dialog saves the memberships it loaded (counter 1); the file's counter (2) is kept.
    let warnings = set_character_campaigns_at_root(&root, &as_loaded).expect("the dialog saves");
    assert!(warnings.is_empty(), "the dialog's stale counter is not a second master: {warnings:?}");
    save_character_bio_at_root(&root, &bio("Neutral Good")).expect("saves");
    assert!(mirror_warnings_at_root(&root).is_empty(), "{:?}", mirror_warnings_at_root(&root));
    assert_eq!(counter(&campaign), 4);
    assert_eq!(campaign_mirror::load_memberships(&root).unwrap()[0].last_backup_counter, Some(4), "the counter this install wrote is kept");
    std::fs::remove_dir_all(&base).ok();
}

#[test]
fn stale_counter_warns_and_keeps_local() {
    let (base, campaign, root_a) = device_a_in_a_campaign("stale");
    let app_b = base.join("app-b");
    campaign_mirror::set_device_name(&app_b, "Bea's laptop").unwrap();
    let root_b = import_on(&app_b, &campaign, ImportMode::MoveMaster);
    save_character_bio_at_root(&root_b, &bio("Chaotic Good")).expect("B saves");
    assert_eq!(counter(&campaign), 2, "the moved master wrote the backup");
    let theirs = std::fs::read(backup(&campaign).join("bio.json")).unwrap();

    save_character_bio_at_root(&root_a, &bio("Lawful Evil")).expect("A's local save succeeds");

    assert!(String::from_utf8(std::fs::read(root_a.join("bio.json")).unwrap()).unwrap().contains("Lawful Evil"), "the local save is kept");
    assert_eq!(std::fs::read(backup(&campaign).join("bio.json")).unwrap(), theirs, "the backup is not overwritten");
    assert_eq!(counter(&campaign), 2);
    let warnings = mirror_warnings_at_root(&root_a);
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert!(warnings[0].contains("changed by Bea's laptop on ") && warnings[0].contains("your local save is safe"), "names who and when: {}", warnings[0]);
    let loaded = load_saved_character_at_root(&root_a).expect("loads");
    assert_eq!(loaded.mirror_warnings, warnings, "the sheet's load carries the warning");
    std::fs::remove_dir_all(&base).ok();
}

#[test]
fn stand_in_save_never_writes_backup() {
    let (base, campaign, _root_a) = device_a_in_a_campaign("stand-in");
    let app_b = base.join("app-b");
    let root_b = import_on(&app_b, &campaign, ImportMode::StandIn);
    let before = std::fs::read(backup(&campaign).join(campaign_mirror::MASTER_FILE)).unwrap();

    save_character_bio_at_root(&root_b, &bio("Chaotic Neutral")).expect("B saves");
    // The dialog sends a membership without the stand-in mark (it never edits it); the mark is kept.
    let warnings = set_character_campaigns_at_root(&root_b, &[joined(&campaign)]).expect("the dialog saves");
    assert!(warnings.is_empty(), "{warnings:?}");
    save_character_bio_at_root(&root_b, &bio("True Neutral")).expect("B saves again");

    assert!(campaign_mirror::load_memberships(&root_b).unwrap()[0].stand_in, "still a stand-in");
    assert!(!backup(&campaign).join("bio.json").exists(), "no stand-in save reached the backup");
    assert_eq!(std::fs::read(backup(&campaign).join(campaign_mirror::MASTER_FILE)).unwrap(), before, "master.json is untouched");
    assert!(mirror_warnings_at_root(&root_b).is_empty());
    std::fs::remove_dir_all(&base).ok();
}
