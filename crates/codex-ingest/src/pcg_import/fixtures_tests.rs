//! Every `.pcg` the oracle and the repo hold goes through the converter, and what comes out goes
//! through the checks the app runs on import. The filter is `pcg_import_fixtures`.
//!
//! **Does not cover:** the desktop's own `import_character` command (E7.2 runs a built binary through
//! it; the verdict here is that command's engine pipeline, retyped), whether a mapped name means the
//! same rule in a 3e or 3.5e file, or a PCGen file that holds a key no fixture does.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState, EquipmentSelection,
    SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::corpus_loader::{BookCorpusRoot, load_book_corpus};
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::pilot_compute_corpus::{compute_combat_baseline_from_corpus, compute_selected_skill_modifiers_from_corpus};
use codex::rules_core::source_content::SourcePackageContent;

use super::catalog::Catalog;
use super::map::{KeyRule, MAPPING, convert};
use super::parse::parse;
use super::wire::{ImportFile, Outcome};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| Catalog::load(&repo_root()).expect("the checkout holds converted rules"))
}

fn oracle_dir() -> PathBuf {
    match std::env::var("PCGEN_REPO_DIR") {
        Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(std::env::var("HOME").expect("HOME")).join("workspace/repos/pcgen"),
    }
}

fn pcg_files_under(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            pcg_files_under(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "pcg") {
            out.push(path);
        }
    }
}

/// F-16: the repo's fixtures and whatever the oracle checkout holds. A CI checkout is sparse and
/// holds fewer than a full clone; `code/testsuite/PCGfiles` is in its cone.
fn fixtures() -> Vec<PathBuf> {
    let mut files = Vec::new();
    pcg_files_under(&repo_root().join("scripts/oracle_harness/eqm-fixtures"), &mut files);
    let repo_count = files.len();
    assert_eq!(repo_count, 13, "scripts/oracle_harness/eqm-fixtures/*.pcg");
    pcg_files_under(&oracle_dir(), &mut files);
    assert!(files.len() - repo_count >= 42, "the pinned oracle's code/testsuite/PCGfiles holds 42 .pcg; found {}", files.len() - repo_count);
    files.sort();
    files
}

/// PCGen 2.4's positional format, which predates `PCGVERSION` (`none_PdfBuster.pcg`: `VERSION:2.4.4`).
/// `decisions.md` §12 reads PCGen 6.x, so these are refused by name.
fn is_older_than_the_format(path: &Path) -> bool {
    let text = String::from_utf8_lossy(&std::fs::read(path).expect("fixture")).into_owned();
    !text.lines().any(|line| line.starts_with("PCGVERSION:") || line.starts_with("CHARACTERNAME:"))
}

fn converted(path: &Path) -> ImportFile {
    let text = String::from_utf8_lossy(&std::fs::read(path).expect("fixture")).into_owned();
    convert(&text, &path.display().to_string(), catalog()).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

fn oracle_fixture(name: &str) -> ImportFile {
    converted(&oracle_dir().join("code/testsuite/PCGfiles").join(name))
}

/// The JSON read back as the app reads it: a file that does not fit the `CharacterInput` here does
/// not fit `character_input_from_dto` there.
fn character_input(file: &ImportFile) -> CharacterInput {
    let json = serde_json::to_value(file).expect("serializes");
    let chosen = &json["characterInput"]["chosen"];
    let text = |value: &serde_json::Value| value.as_str().expect("a string").to_owned();
    let list = |key: &str| chosen[key].as_array().unwrap_or_else(|| panic!("{key} is a list")).clone();
    let abilities = &chosen["abilityScores"];
    let score = |key: &str| abilities[key].as_i64().expect("a score") as i16;
    CharacterInput {
        case_id: Some("import".to_owned()),
        source_package_id: text(&json["characterInput"]["sourcePackageId"]),
        chosen: ChosenCharacterState {
            race_id: text(&chosen["raceId"]),
            class_levels: list("classLevels")
                .iter()
                .map(|row| CharacterClassLevel { class_id: text(&row["classId"]), level: row["level"].as_u64().expect("a level") as u8 })
                .collect(),
            ability_scores: AbilityScores {
                strength: score("strength"),
                dexterity: score("dexterity"),
                constitution: score("constitution"),
                intelligence: score("intelligence"),
                wisdom: score("wisdom"),
                charisma: score("charisma"),
            },
            selected_feats: list("selectedFeats").iter().map(text).collect(),
            skill_allocations: list("skillAllocations")
                .iter()
                .map(|row| SkillAllocation { skill_id: text(&row["skillId"]), ranks: row["ranks"].as_u64().expect("ranks") as u8 })
                .collect(),
            equipment_selections: list("equipmentSelections")
                .iter()
                .map(|row| {
                    let active_state = match row["activeState"].as_str() {
                        Some("EquippedActive") => ActiveState::EquippedActive,
                        Some("SelectedInactive") => ActiveState::SelectedInactive,
                        Some("Absent") => ActiveState::Absent,
                        other => panic!("not an ActiveStateDto: {other:?}"),
                    };
                    EquipmentSelection {
                        item_id: text(&row["itemId"]),
                        equipped_or_active: active_state == ActiveState::EquippedActive,
                        active_state,
                        applied_modifiers: row["appliedModifiers"].as_array().map(|rows| rows.iter().map(text).collect()).unwrap_or_default(),
                    }
                })
                .collect(),
            selected_choices: list("selectedChoices")
                .iter()
                .map(|row| SelectedChoice { choice_set_id: text(&row["choiceSetId"]), selection_id: text(&row["selectionId"]) })
                .collect(),
            selected_traits: list("selectedTraits").iter().map(text).collect(),
            spells_selected: list("spellsSelected")
                .iter()
                .map(|row| SpellSelection {
                    spell_id: text(&row["spellId"]),
                    source_class_id: text(&row["sourceClassId"]),
                    acquisition_mode: match row["acquisitionMode"].as_str() {
                        Some("Known") => AcquisitionMode::Known,
                        Some("Prepared") => AcquisitionMode::Prepared,
                        Some("Granted") => AcquisitionMode::Granted,
                        other => panic!("not an AcquisitionModeDto: {other:?}"),
                    },
                })
                .collect(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn app_corpus() -> &'static SourcePackageContent<'static> {
    static CORPUS: OnceLock<SourcePackageContent<'static>> = OnceLock::new();
    CORPUS.get_or_init(|| {
        let dir = repo_root().join("apps/desktop/src-tauri/resources/corpus_fixtures");
        load_book_corpus(&[BookCorpusRoot { book_id: "desktop_ui_fixtures", dir: dir.as_path() }])
    })
}

/// `resolve_unified_pilot_snapshot`'s verdict (`pf1_adapter.rs`), the gate `import_character` saves
/// behind: the two posture diagnostics are decided by the corpus pillars, every other one blocks.
fn engine_verdict(input: &CharacterInput) -> Result<(), Vec<String>> {
    let receipt = build_pilot_headless_receipt(input);
    let blocking: Vec<String> = receipt
        .computation
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.claim_blocking && diagnostic.id != "combat.baseline_unsupported" && diagnostic.id != "skill.selected_modifier.unsupported"
        })
        .map(|diagnostic| diagnostic.id.clone())
        .collect();
    if !blocking.is_empty() {
        return Err(blocking);
    }
    let mut unmet = Vec::new();
    if let Err(reasons) = compute_combat_baseline_from_corpus(&receipt.computation, input, app_corpus()) {
        unmet.extend(reasons);
    }
    if let Err(reasons) = compute_selected_skill_modifiers_from_corpus(&receipt.computation, input, app_corpus()) {
        unmet.extend(reasons);
    }
    if unmet.is_empty() { Ok(()) } else { Err(unmet) }
}

#[test]
fn pcg_import_fixtures_every_file_converts_and_every_line_is_reported() {
    let files = fixtures();
    let mut lines_total = 0usize;
    let mut refused = 0usize;
    for path in &files {
        let text = String::from_utf8_lossy(&std::fs::read(path).unwrap()).into_owned();
        if is_older_than_the_format(path) {
            let err = convert(&text, "old.pcg", catalog()).unwrap_err();
            assert!(err.contains("old.pcg is not a PCGen 6 character"), "{err}");
            refused += 1;
            continue;
        }
        let file = converted(path);
        let data_lines = parse(&text).len();
        lines_total += data_lines;
        let read_lines = file.report.lines.iter().filter(|line| line.outcome != Outcome::Added).count();
        assert_eq!(read_lines, data_lines, "{}: every line of the file has one report line", path.display());
        assert!(!file.display_label.trim().is_empty(), "{}", path.display());
        assert!(!file.character_input.chosen.class_levels.is_empty(), "{}", path.display());
        let input = character_input(&file);
        assert_eq!(input.source_package_id, "pf1.core_rulebook");
        let custom = [&file.custom.feats, &file.custom.equipment, &file.custom.spells];
        let mut ids = std::collections::BTreeSet::new();
        for record in custom.iter().flat_map(|records| records.iter()) {
            assert!(ids.insert(record.id.clone()), "{}: duplicate custom id {}", path.display(), record.id);
            assert!(!record.name.trim().is_empty() && record.name.chars().count() <= 120, "{}: {:?}", path.display(), record.name);
            assert!(record.description.chars().count() <= 10_000);
            assert!(record.stats.len() <= 40);
            for stat in &record.stats {
                assert!(!stat.label.trim().is_empty() && !stat.value.trim().is_empty(), "{}: {record:?}", path.display());
            }
        }
    }
    println!("pcg_import_fixtures: {} files, {refused} refused as older than the format, {lines_total} data lines", files.len());
    assert!(refused >= 1, "none_PdfBuster.pcg is in the pinned oracle's PCGfiles");
}

/// The partition is the file's own: a file naming only a race and classes Codex holds must compute.
/// One that names another cannot, and the import's refusal is the engine's, by name; those are
/// PCGen's test characters and other game systems' classes.
#[test]
fn pcg_import_fixtures_a_file_of_held_classes_computes() {
    let mut computed = 0usize;
    let mut refused_by_class = Vec::new();
    let mut wrong = Vec::new();
    for path in fixtures().into_iter().filter(|path| !is_older_than_the_format(path)) {
        let file = converted(&path);
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let verdict = engine_verdict(&character_input(&file));
        println!("{name}: mode={} classes={:?} race={:?} verdict={:?}", file.report.game_mode, file.report.unmapped_classes, file.report.unmapped_race, verdict);
        match (file.report.unmapped_classes.is_empty() && file.report.unmapped_race.is_none(), verdict) {
            (true, Ok(())) => computed += 1,
            (true, Err(reasons)) => wrong.push(format!("{name}: {reasons:?}")),
            (false, _) => refused_by_class.push(name),
        }
    }
    println!("pcg_import_fixtures: {computed} computed; {} name a class or race Codex does not hold: {refused_by_class:?}", refused_by_class.len());
    assert!(wrong.is_empty(), "files of held classes must compute:\n{}", wrong.join("\n"));
    assert!(computed >= 13, "at least the 13 repo fixtures; got {computed}");
}

/// Every SPELLNAME line is a kept spell or a custom spell, and some real ones are kept: a list the
/// engine takes in full must not all fall to custom.
#[test]
fn pcg_import_fixtures_a_spell_is_kept_or_custom_and_most_are_kept() {
    let (mut kept, mut custom) = (0usize, 0usize);
    for path in fixtures().into_iter().filter(|path| !is_older_than_the_format(path)) {
        let file = converted(&path);
        for line in file.report.lines.iter().filter(|line| line.key == "SPELLNAME") {
            match line.outcome {
                Outcome::Mapped => {
                    kept += 1;
                    assert!(file.character_input.chosen.spells_selected.iter().any(|spell| spell.spell_id == line.from), "{}: {}", path.display(), line.from);
                }
                Outcome::Custom => custom += 1,
                other => panic!("{}: a spell line came out {other:?}", path.display()),
            }
        }
    }
    println!("pcg_import_fixtures: SPELLNAME lines, {kept} kept, {custom} custom");
    assert!(kept > 0, "no spell survived, so the proof above would be empty");
}

/// `pf_Paladin.pcg`, a whole level 6 character, field by field against the file's own lines.
#[test]
fn pcg_import_fixtures_paladin_reads_what_the_file_says() {
    let file = oracle_fixture("pf_Paladin.pcg");
    let chosen = &file.character_input.chosen;
    assert_eq!(file.display_label, "Florian Syrkov");
    assert_eq!(chosen.race_id, "race:human");
    assert_eq!((chosen.class_levels[0].class_id.as_str(), chosen.class_levels[0].level), ("class:paladin", 6));
    // STAT:STR|SCORE:16 + ABILITY KEY:+2 Strength; DEX has neither.
    assert_eq!(chosen.ability_scores.strength, 18);
    assert_eq!(chosen.ability_scores.dexterity, 12);
    assert_eq!(chosen.ability_scores.charisma, 14);
    assert!(chosen.selected_choices.iter().any(|c| c.choice_set_id == "choice:human_ability_bonus" && c.selection_id == "ability:strength"));

    for feat in ["Reward of Grace", "Power Attack", "Weapon Focus", "Skill Focus"] {
        assert!(chosen.selected_feats.iter().any(|held| held == feat), "{feat}: {:?}", chosen.selected_feats);
    }
    assert!(chosen.selected_choices.iter().any(|c| c.choice_set_id == "choice:weapon_focus_target" && c.selection_id == "weapon:Flail (Heavy)"));
    assert!(chosen.selected_choices.iter().any(|c| c.choice_set_id == "choice:skill_focus_target" && c.selection_id == "skill:Diplomacy"));

    let ranks = |id: &str| chosen.skill_allocations.iter().find(|skill| skill.skill_id == id).map(|skill| skill.ranks);
    assert_eq!(ranks("skill:diplomacy"), Some(2));
    assert_eq!(ranks("skill:knowledge_religion"), Some(1));
    assert_eq!(ranks("skill:acrobatics"), None, "a SKILL line with no CLASSBOUGHT buys nothing");

    let rows = |id: &str| chosen.equipment_selections.iter().filter(|row| row.item_id == id).collect::<Vec<_>>();
    let plate = rows("Full Plate");
    assert_eq!(plate.len(), 1);
    assert_eq!(plate[0].active_state, "EquippedActive", "EQUIPSET:Armor");
    assert_eq!(plate[0].applied_modifiers, ["Special Ability ~ +1 ~ Armor"]);
    let flail = rows("Flail (Heavy)");
    assert_eq!(flail[0].active_state, "EquippedActive", "EQUIPSET:Both Hands");
    assert_eq!(flail[0].applied_modifiers, ["Special Ability ~ +1 ~ Weapon"]);
    assert_eq!(rows("Dagger")[0].active_state, "SelectedInactive", "EQUIPSET:Carried");
    assert_eq!(rows("Bedroll")[0].active_state, "SelectedInactive", "inside the saddlebags on the horse");
    assert_eq!(rows("Javelin").len(), 5, "QUANTITY:5.0");
    assert!(rows("Javelin").iter().all(|row| row.active_state == "SelectedInactive"));

    // Ultimate Combat's Compel Hostility is on the file's list and not on the engine's paladin list,
    // and one refused spell would refuse the character.
    assert!(file.custom.spells.iter().any(|record| record.name == "Compel Hostility"), "{:?}", file.custom.spells);
    assert!(file.report.lines.iter().any(|line| line.key == "SPELLNAME" && line.from == "Compel Hostility" && line.outcome == Outcome::Custom));

    assert_eq!(file.money.total_copper, 48_800, "MONEY:488");
    assert_eq!(file.bio.alignment, "Lawful Good");
    assert_eq!(file.bio.deity, "Abadar");
    assert_eq!(file.bio.player_name, "James");
    assert_eq!(file.bio.sex, "Male");
    let languages = file.custom.feats.iter().find(|record| record.name == "Languages").expect("languages");
    assert_eq!(languages.description, "Varisian, Common");
    let biography = file.custom.feats.iter().find(|record| record.name == "Biography").expect("biography");
    assert!(biography.description.starts_with("The Syrkov family"));

    // In no line of the file, and each one says so.
    let added: Vec<&str> = file.report.lines.iter().filter(|line| line.outcome == Outcome::Added).map(|line| line.to.as_str()).collect();
    assert!(added.iter().any(|to| to.contains("item:longsword")), "{added:?}");
    assert!(!added.iter().any(|to| to.contains("weapon_focus") && to.contains("selectedFeats")), "the file holds Weapon Focus: {added:?}");
    assert_eq!(engine_verdict(&character_input(&file)), Ok(()));
}

/// A race's adjustment lands on the saved score, and these are PCGen's own totals: run through the
/// real engine (`scripts/pcgen-run-character.sh`, `STAT.n`), `pf_Rogue.pcg` prints Str 12, Dex 22,
/// Int 15, Cha 14, where Dex and Int each carry a worn +2 item the sheet adds for itself, and
/// `pf_Cleric.pcg` prints Cha 17 (15, plus the Human's +2 the file sends to Charisma).
#[test]
fn pcg_import_fixtures_a_race_adjustment_lands_on_the_score() {
    let file = oracle_fixture("pf_Rogue.pcg");
    let scores = &file.character_input.chosen.ability_scores;
    assert_eq!(file.character_input.chosen.race_id, "race:halfling");
    assert_eq!((scores.strength, scores.dexterity, scores.intelligence, scores.charisma), (12, 20, 13, 14));
    let cleric = oracle_fixture("pf_Cleric.pcg");
    assert_eq!(cleric.character_input.chosen.ability_scores.charisma, 17);
}

#[test]
fn pcg_import_fixtures_an_item_codex_does_not_hold_is_custom_with_its_place() {
    let dir = repo_root().join("scripts/oracle_harness/eqm-fixtures");
    let file = converted(&dir.join("sd33r6_eqm_combo.pcg"));
    let leather = file.custom.equipment.iter().find(|record| record.name == "SD33R6 ~ Leather Dragonhide").expect("custom item");
    assert!(leather.stats.iter().any(|stat| stat.label == "Worn" && stat.value == "1"), "{leather:?}");
    assert!(file.report.lines.iter().any(|line| line.outcome == Outcome::Custom && line.to == "SD33R6 ~ Leather Dragonhide"));
}

#[test]
fn pcg_import_fixtures_an_unknown_key_is_a_custom_record_and_a_report_line() {
    let text = "PCGVERSION:2.0\nCHARACTERNAME:Test\nGAMEMODE:Pathfinder_RPG\nRACE:Human\nCLASS:Fighter|LEVEL:1\nSOMENEWKEY:some value\n";
    let file = convert(text, "inline.pcg", catalog()).unwrap();
    assert_eq!(file.report.unknown_keys, ["SOMENEWKEY"]);
    let record = file.custom.feats.iter().find(|record| record.name == "some value").expect("custom record");
    assert!(record.stats.iter().any(|stat| stat.label == "PCGen key" && stat.value == "SOMENEWKEY"));
    assert!(file.report.lines.iter().any(|line| line.key == "SOMENEWKEY" && line.outcome == Outcome::Custom));
}

#[test]
fn pcg_import_fixtures_a_file_that_is_no_character_is_refused_by_name() {
    let err = convert("hello\nworld\n", "notes.txt", catalog()).unwrap_err();
    assert!(err.contains("notes.txt is not a PCGen 6 character"), "{err}");
}

/// A class Codex does not hold stays visible twice: a custom record, and in the report's own list.
#[test]
fn pcg_import_fixtures_a_class_codex_does_not_hold_is_named() {
    let file = oracle_fixture("sf_soldier.pcg");
    assert_eq!(file.report.game_mode, "Starfinder");
    assert_eq!(file.report.unmapped_classes, ["Soldier 10"]);
    assert!(file.custom.feats.iter().any(|record| record.name == "Soldier"));
    assert_eq!(file.character_input.chosen.class_levels[0].class_id, "class:soldier");
    assert!(engine_verdict(&character_input(&file)).is_err(), "no Pathfinder engine row can compute a Starfinder soldier");
}

/// The mapping table doc prints every row of the table it documents.
#[test]
fn pcg_import_fixtures_the_mapping_doc_lists_every_key() {
    let doc = std::fs::read_to_string(repo_root().join("docs/architecture/pcg-import.md")).expect("the mapping table doc");
    let missing: Vec<&str> = MAPPING.iter().map(|(key, _)| *key).filter(|key| !doc.contains(&format!("`{key}`"))).collect();
    assert!(missing.is_empty(), "docs/architecture/pcg-import.md does not list: {missing:?}");
    assert!(MAPPING.iter().any(|(_, rule)| matches!(rule, KeyRule::Setting)));
}

/// The file's rank is the row the engine reads, on all three, with nothing held back. A file with no
/// row in one writes none: the engine reads it untrained, and the character still computes.
#[test]
fn pcg_import_fixtures_a_files_rank_in_climb_intimidate_swim_is_the_row_the_engine_reads() {
    let file = oracle_fixture("35e_L10(fighter-human).pcg");
    let rank_of = |skill: &str| file.character_input.chosen.skill_allocations.iter().find(|row| row.skill_id == skill).map(|row| row.ranks);
    assert_eq!(rank_of("skill:climb"), Some(13));
    assert!(!file.custom.feats.iter().any(|record| record.name == "Skill ranks from PCGen"), "{:?}", file.custom.feats);
    assert!(!file.report.warnings.iter().any(|warning| warning.contains("the file buys")), "{:?}", file.report.warnings);
    assert!(!file.report.lines.iter().any(|line| line.outcome == Outcome::Added && line.to.contains("skillAllocations")), "no rank is added");

    let input = character_input(&file);
    engine_verdict(&input).expect("the file's own ranks compute");
    let total = |input: &CharacterInput| {
        let receipt = build_pilot_headless_receipt(input);
        compute_selected_skill_modifiers_from_corpus(&receipt.computation, input, app_corpus()).expect("computes").climb
    };
    let mut at_eleven = input.clone();
    at_eleven.chosen.skill_allocations.iter_mut().find(|row| row.skill_id == "skill:climb").unwrap().ranks = 11;
    assert_eq!(total(&input) - total(&at_eleven), 2, "the total follows the rank");

    let mut no_rank = input.clone();
    no_rank.chosen.skill_allocations.retain(|row| row.skill_id != "skill:climb");
    engine_verdict(&no_rank).expect("no rank reads untrained, it does not refuse");
}
