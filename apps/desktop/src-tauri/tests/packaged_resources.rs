//! A packaged app has only what `tauri.conf.json` bundles, laid out under its resource directory.
//! This test builds exactly that layout (symlinks to each bundled source, nothing else), points the
//! rules crate's data root at it, and asks the engine for the things the shipped app asks for.
//!
//! It exists because the class roster failed in every installed build with
//! `could not read /home/runner/work/codex/codex/tests/fixtures/...`: the engine read test fixtures
//! and data directories from a path baked in at compile time, and nothing checked that the bundle
//! carried what the engine reads. Test builds always found the checkout, so no test could see it.
//!
//! One process, one data root: `set_data_root` is process-wide and first-call-wins, so this file
//! must keep all of its checks in a single `#[test]`.

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use codex::rules_core::class_census::{
    class_creation_roster, load_mix_panel, load_sweep_fixture, MIX_PANEL_RELATIVE_PATH,
};
use codex::rules_core::class_seeds::FIXTURE_RELATIVE_PATH;
use codex::rules_core::equipment_types::{equipment_types, EQUIPMENT_TYPES_PATH};
use codex::rules_core::record_vars::RECORD_VARS_PATH;

fn src_tauri() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Lay out the bundle the way Tauri does: each `resources` entry's source appears at its
/// destination under the resource root.
fn packaged_root() -> PathBuf {
    let conf: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(src_tauri().join("tauri.conf.json")).unwrap()).unwrap();
    let resources = conf["bundle"]["resources"].as_object().expect("bundle.resources is a map");

    let root = std::env::temp_dir().join(format!("codex-packaged-root-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    for (source, dest) in resources {
        let source = fs::canonicalize(src_tauri().join(source))
            .unwrap_or_else(|e| panic!("bundled resource {source} does not exist: {e}"));
        let dest = root.join(dest.as_str().unwrap().trim_end_matches('/'));
        // Entries may nest (`corpus_fixtures/` and `corpus_fixtures/spell/`): the child is already
        // reachable through the parent's link. Accept that only when it is the same source.
        if dest.exists() {
            assert_eq!(
                fs::canonicalize(&dest).unwrap(),
                source,
                "two bundle entries disagree about what lives at {}",
                dest.display()
            );
            continue;
        }
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        symlink(&source, &dest).unwrap_or_else(|e| panic!("cannot lay out {}: {e}", dest.display()));
    }
    root
}

fn assert_bundled(root: &Path, relative: &str) {
    assert!(
        root.join(relative).exists(),
        "the engine reads `{relative}` at runtime but tauri.conf.json does not bundle it"
    );
}

#[test]
fn the_packaged_bundle_carries_everything_the_rules_engine_reads() {
    let root = packaged_root();
    assert!(codex::set_data_root(root.clone()), "data root must install on first call");

    // Every runtime path the engine joins onto its data root.
    for relative in [
        FIXTURE_RELATIVE_PATH,
        MIX_PANEL_RELATIVE_PATH,
        RECORD_VARS_PATH,
        EQUIPMENT_TYPES_PATH,
        "data/sheet_rules",
        "data/starfinder-1e/sheet_rules",
        "data/rules_tables",
        "data/class_feature_grants",
        "data/corpus",
        "data/cyberpunk-red/corpus",
    ] {
        assert_bundled(&root, relative);
    }

    // And the user-visible behaviour: the roster the Create dialog shows.
    load_sweep_fixture().expect("the shared sweep fixture loads from the packaged root");
    load_mix_panel().expect("the multiclass mix panel loads from the packaged root");
    let roster = class_creation_roster().expect("the class roster builds from the packaged root");
    assert!(
        roster.len() > 31,
        "roster has {} classes: the app would offer only the built-in 31-class fallback",
        roster.len()
    );

    assert!(
        equipment_types().is_ok(),
        "the equipment type sidecar does not load from the packaged root: {:?}",
        equipment_types().as_ref().err()
    );

    assert!(
        codex::rules_core::corpus_loader::live_sheet_rules().is_some(),
        "live_sheet_rules found no rules under the packaged root (it returns None silently)"
    );
    assert!(
        codex::rules_core::sheet_rule_package::package().is_ok(),
        "the sheet-rule package does not load from the packaged root"
    );

    // Every game system's package and the rules-table package resolve under the installed root,
    // not the compile-time checkout (the per-system roots read `runtime_repo_root`).
    let installed = codex::rules_core::game_system::runtime_repo_root();
    assert_eq!(installed, root, "the per-system package root ignores the installed data root");
    assert!(
        codex::rules_core::corpus_loader::live_sheet_rules_for(codex::rules_core::game_system::GameSystem::Starfinder1e)
            .is_some(),
        "the Starfinder sheet-rule package does not load from the packaged root"
    );
    // Cyberpunk RED has no sheet-rule package; its corpus root is what the engine reads, and it must
    // resolve under the installed root at its own path (the tracked LICENSE.json proves it is there).
    let cpr = codex::rules_core::game_system::GameSystem::CyberpunkRed.package_roots(&installed);
    assert_eq!(cpr.corpus, root.join("data/cyberpunk-red/corpus"), "the CPR corpus root moved");
    assert!(
        cpr.corpus.join("LICENSE.json").is_file(),
        "the Cyberpunk RED corpus root resolves under the packaged root but carries no LICENSE.json"
    );
    assert_eq!(
        codex::rules_core::rules_data_package::runtime_package_root(),
        root.join("data/rules_tables"),
        "the rules-table package is not read from the packaged root"
    );

    the_cyberpunk_corpus_and_a_starter_sheet_load_from_the_packaged_root(&cpr.corpus);
}

/// Nyx Calder, the Complete Package Netrunner starter (`cpr_adapter::starter_requests`), as the
/// Create screen saves her: `cpr_character_input.txt` and `cpr_sheet_choices.txt` exactly as
/// the starter restore writes them.
const NYX_INPUT: &str = "\
stat.int = 7
stat.ref = 6
stat.dex = 6
stat.tech = 7
stat.cool = 6
stat.will = 5
stat.luck = 6
stat.move = 6
stat.body = 6
stat.emp = 7
";

const NYX_CHOICES: &str = "\
role = Netrunner
role.rank = 4
skill.Athletics = 2
skill.Brawling = 2
skill.Concentration = 4
skill.Conversation = 2
skill.Education = 4
skill.Evasion = 4
skill.First Aid = 2
skill.Human Perception = 2
skill.Language (Streetslang) = 2
skill.Local Expert (Your Home) = 4
skill.Perception = 4
skill.Persuasion = 2
skill.Stealth = 4
skill.Basic Tech = 6
skill.Conceal/Reveal Object = 4
skill.Cryptography = 4
skill.Cybertech = 6
skill.Electronics/Security Tech = 6
skill.Handgun = 6
skill.Library Search = 6
skill.Streetwise = 4
skill.Language (English) = 4
armor.body = Light Armorjack
armor.head = Light Armorjack
cyberware = Neural Link
cyberware = Interface Plugs
weapon = Heavy Pistol
deck = Cyberdeck (Standard Quality)
deck.program = Sword
deck.program = Sword
deck.program = Armor
deck.program = Worm
deck.program = See Ya
deck.program = Eraser
cash = 460
ammo = 50 Basic H Pistol Ammunition
";

/// Every Cyberpunk RED file the engine and the Create screen read sits under the packaged corpus
/// root and parses, and one starter's whole sheet composes from that root alone.
fn the_cyberpunk_corpus_and_a_starter_sheet_load_from_the_packaged_root(corpus: &Path) {
    use codex::rules_core::cpr::input::{parse_cpr_character_input, parse_cpr_sheet_choices};
    use codex::rules_core::cpr::sheet::{compose_sheet, CprSheetCatalogs};

    // The manifest names every per-kind file the corpus carries; each one must be in the bundle
    // and readable as the record list the engine deserialises.
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(corpus.join("MANIFEST.json")).expect("MANIFEST.json is bundled under the packaged root"),
    )
    .expect("MANIFEST.json parses");
    let kinds = manifest["kinds"].as_array().expect("MANIFEST.json lists its kinds");
    assert!(!kinds.is_empty(), "MANIFEST.json under the packaged root lists no kinds");
    let mut expected_files: Vec<String> =
        kinds.iter().map(|k| k["file"].as_str().expect("every kind names its file").to_owned()).collect();
    for file in &expected_files {
        let text = fs::read_to_string(corpus.join(file))
            .unwrap_or_else(|e| panic!("the Cyberpunk RED corpus file {file} is not readable under the packaged root: {e}"));
        let records: serde_json::Value =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("{file} under the packaged root does not parse: {e}"));
        assert!(records.is_array(), "{file} under the packaged root is not a record list");
    }
    // Nothing the bundle carries is outside the manifest (plus the manifest and the licence).
    expected_files.extend(["LICENSE.json".to_owned(), "MANIFEST.json".to_owned()]);
    expected_files.sort();
    let mut bundled: Vec<String> = fs::read_dir(corpus)
        .expect("the packaged CPR corpus root is a directory")
        .map(|e| e.expect("entry").file_name().into_string().expect("utf-8 name"))
        .collect();
    bundled.sort();
    assert_eq!(bundled, expected_files, "the packaged CPR corpus and its MANIFEST.json disagree");

    // One starter computed under the packaged root: every catalog the sheet reads, then the sheet.
    let catalogs = CprSheetCatalogs::load(corpus).expect("every CPR sheet catalog loads from the packaged root");
    let input = parse_cpr_character_input(NYX_INPUT).expect("Nyx Calder's saved STATs parse");
    let choices = parse_cpr_sheet_choices(NYX_CHOICES).expect("Nyx Calder's saved choices parse");
    let sheet = compose_sheet(&catalogs, &input, &choices).expect("Nyx Calder's sheet composes under the packaged root");

    assert!(
        sheet.rows.iter().all(|r| r.id.starts_with("cpr.")),
        "a CPR sheet row does not carry a cpr. id"
    );
    assert_eq!(sheet.rows.len(), 213, "Nyx Calder's sheet has a different row count under the packaged root");
    let value = |id: &str| {
        sheet.rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("Nyx Calder's sheet has no row {id}")).value
    };
    // The figures `artifacts/epic_7/cpr-seed-builds.md` derives for her.
    for (id, expected) in [
        ("cpr.derived.hit_points", 40),
        ("cpr.derived.humanity_current", 56),
        ("cpr.derived.emp_current", 5),
        ("cpr.role.netrunner.rank", 4),
        ("cpr.skill.handgun", 12),
        ("cpr.netrunning.net_actions", 3),
        ("cpr.netrunning.deck_slots_used", 6),
        ("cpr.netrunning.deck_slots_available", 7),
        ("cpr.cash", 460),
    ] {
        assert_eq!(value(id), expected, "Nyx Calder's {id} under the packaged root");
    }
}
