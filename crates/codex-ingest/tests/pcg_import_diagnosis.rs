//! E1.1 diagnosis run (SD-42 `technical-design.md §1`): the built `pcg_import` sidecar over the
//! proof set, one table row per `EQUIPNAME` line, and a summary of counts per file. It fixes
//! nothing and asserts no defect; it writes the evidence the H1–H7 verdicts rest on.
//!
//! Proof set (`decisions.md §3` Q3, Q7): PCGen's seven samples under
//! `$PCGEN_REPO_DIR/code/testsuite/PCGfiles` (tables and `summary.json` under the package's
//! `artifacts/epic_1/E1.1/`), and the operator's file at `$CODEX_SD42_PCG` (else the one `.pcg` in
//! `$HOME/sd42-input/`), whose table and summary stay beside it in `diagnosis/` (`decisions.md §10`).
//!
//! **Does not cover:** the desktop's `import_pcg_json` (the app crate is not linked here, so the
//! Default-set migration and the app's own slot read are retyped: `codex_slot` is
//! `body_slots::worn_slot` with no wield category, so a one-handed/two-handed split is not read),
//! any `.pcg` outside these eight files, and sets other than the file's `CALCEQUIPSET`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use codex::rules_core::body_slots::{BodySlot, slot_conflict, worn_slot};
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState, EquipmentSelection,
    SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::corpus_loader::{BookCorpusRoot, live_sheet_rules, load_book_corpus};
use codex::rules_core::equipment_types::equipment_types;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::pilot_compute_corpus::{compute_combat_baseline_from_corpus, compute_selected_skill_modifiers_from_corpus};
use codex::rules_core::sheet_rule::slug;
use codex::rules_core::source_content::SourcePackageContent;
use codex_ingest::pcg_import::catalog::fold;
use codex_ingest::pcg_import::parse::{Line, Tag, parse};
use codex_ingest::pcg_import::wire::{ImportFile, Outcome};
use serde_json::{Value, json};

const SAMPLES: [&str; 7] = ["pf_Cleric", "pf_Paladin", "pf_Rogue", "pf_goldielocks", "sf_mechanic", "sf_mechanic_drone", "sf_soldier"];
const COLUMNS: &str = "line\tname\tbase\tcustom_name\toutcome\tcodex_key\tsheet_record\tworn\tcarried\tcopies_emitted\tfile_quantity\tpcgen_location\tcodex_slot\tslot_collision\teqmods_not_applied\tdata_fields_not_read";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn committed_dir() -> PathBuf {
    repo_root().join("docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/epic_1/E1.1")
}

/// E1.1's committed tables are frozen evidence: a run writes them only when `PCG_DIAGNOSIS_REFRESH=1`,
/// otherwise under the cargo target tmp dir (never under `docs/`).
fn out_dir_for(refresh: bool) -> PathBuf {
    if refresh { committed_dir() } else { Path::new(env!("CARGO_TARGET_TMPDIR")).join("pcg-diagnosis-e1.1") }
}

fn out_dir() -> PathBuf {
    out_dir_for(std::env::var("PCG_DIAGNOSIS_REFRESH").is_ok_and(|v| v == "1"))
}

#[test]
fn default_output_dir_is_outside_the_repo() {
    // What the default protects is the committed evidence, not "outside the repo": CI's target dir is `<repo>/target`.
    let repo = repo_root().canonicalize().expect("repo root");
    let default = out_dir_for(false);
    for docs in [repo.join("docs"), repo_root().join("docs")] {
        assert!(!default.starts_with(&docs), "default {default:?} is under the committed docs/ tree");
    }
    assert_ne!(default, committed_dir());
    assert_eq!(out_dir_for(true), committed_dir());
}

fn oracle_files() -> PathBuf {
    let repo = match std::env::var("PCGEN_REPO_DIR") {
        Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(std::env::var("HOME").expect("HOME")).join("workspace/repos/pcgen"),
    };
    repo.join("code/testsuite/PCGfiles")
}

/// The operator's file: `$CODEX_SD42_PCG`, else the one `.pcg` in `$HOME/sd42-input/`. No name is
/// written here, so the repo never carries it (`decisions.md §10`).
fn operator_file() -> PathBuf {
    if let Ok(path) = std::env::var("CODEX_SD42_PCG").map(PathBuf::from)
        && !path.as_os_str().is_empty()
    {
        return path;
    }
    let dir = PathBuf::from(std::env::var("HOME").expect("HOME")).join("sd42-input");
    std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.extension().is_some_and(|ext| ext == "pcg"))
        .unwrap_or_else(|| dir.join("<operator>.pcg"))
}

/// Runs the built sidecar binary, exactly as the app does, and reads its JSON back.
fn run_sidecar(pcg: &Path, scratch: &Path) -> ImportFile {
    let out = scratch.join("import.json");
    let status = Command::new(env!("CARGO_BIN_EXE_pcg_import")).arg(pcg).arg("--out").arg(&out).output().expect("runs the sidecar");
    assert!(status.status.success(), "pcg_import refused a proof file: {}", String::from_utf8_lossy(&status.stderr));
    serde_json::from_str(&std::fs::read_to_string(&out).expect("the sidecar wrote its file")).expect("the wire reads back")
}

fn data_fields(line: &Line) -> (Option<String>, BTreeMap<String, String>) {
    let custom = line.tags.iter().find(|tag| tag.key == "CUSTOMIZATION").map(Tag::nested).unwrap_or_default();
    let base = custom.iter().find(|tag| tag.key == "BASEITEM").map(|tag| tag.value.clone());
    let data = custom
        .iter()
        .find(|tag| tag.key == "DATA")
        .map(|tag| tag.value.split('$').filter_map(|field| field.split_once('=')).map(|(k, v)| (k.to_owned(), v.to_owned())).collect())
        .unwrap_or_default();
    (base, data)
}

/// Each item's places in the `CALCEQUIPSET` set: the top-level location word (a body slot,
/// `Equipped`, `Carried`, `Not Carried`) or `in <container>`, with the quantity there.
fn pcgen_locations(lines: &[Line]) -> BTreeMap<String, Vec<String>> {
    let set = lines.iter().find(|line| line.key() == "CALCEQUIPSET").map(|line| line.value().to_owned()).unwrap_or_default();
    let top = set.split('.').count() + 1;
    let mut places: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in lines.iter().filter(|line| line.key() == "EQUIPSET") {
        let (Some(id), Some(value)) = (line.get("ID"), line.get("VALUE")) else { continue };
        if !id.starts_with(&format!("{set}.")) {
            continue;
        }
        let place = if id.split('.').count() == top { line.value().to_owned() } else { format!("in {}", line.value()) };
        let qty = line.get("QUANTITY").unwrap_or("?");
        places.entry(fold(value)).or_default().push(format!("{place} x{qty}"));
    }
    places
}

/// The slot the app's held list would read for `item_id` (`body_slot_of_item`, retyped).
fn codex_slot(item_id: &str) -> Option<BodySlot> {
    let package = live_sheet_rules()?;
    let needle = item_id.strip_prefix("item:").unwrap_or(item_id);
    let record = package.find("equipment", &slug(needle)).and_then(|id| package.rule(id))?;
    let types = equipment_types().as_ref().ok()?.get(needle).map_or(&[][..], Vec::as_slice);
    worn_slot(record, types, None).ok()
}

fn app_corpus() -> &'static SourcePackageContent<'static> {
    static CORPUS: OnceLock<SourcePackageContent<'static>> = OnceLock::new();
    CORPUS.get_or_init(|| {
        let dir = repo_root().join("apps/desktop/src-tauri/resources/corpus_fixtures");
        load_book_corpus(&[BookCorpusRoot { book_id: "desktop_ui_fixtures", dir: dir.as_path() }])
    })
}

/// The JSON as `character_input_from_dto` reads it (the shape of `fixtures_tests::character_input`).
fn character_input(file: &ImportFile) -> CharacterInput {
    let chosen = &file.character_input.chosen;
    CharacterInput {
        case_id: Some("import".to_owned()),
        source_package_id: file.character_input.source_package_id.clone(),
        chosen: ChosenCharacterState {
            race_id: chosen.race_id.clone(),
            class_levels: chosen.class_levels.iter().map(|c| CharacterClassLevel { class_id: c.class_id.clone(), level: c.level }).collect(),
            ability_scores: AbilityScores {
                strength: chosen.ability_scores.strength,
                dexterity: chosen.ability_scores.dexterity,
                constitution: chosen.ability_scores.constitution,
                intelligence: chosen.ability_scores.intelligence,
                wisdom: chosen.ability_scores.wisdom,
                charisma: chosen.ability_scores.charisma,
            },
            selected_feats: chosen.selected_feats.clone(),
            skill_allocations: chosen.skill_allocations.iter().map(|s| SkillAllocation { skill_id: s.skill_id.clone(), ranks: s.ranks }).collect(),
            equipment_selections: chosen
                .equipment_selections
                .iter()
                .map(|row| {
                    let active_state = match row.active_state.as_str() {
                        "EquippedActive" => ActiveState::EquippedActive,
                        "Absent" => ActiveState::Absent,
                        _ => ActiveState::SelectedInactive,
                    };
                    EquipmentSelection {
                        item_id: row.item_id.clone(),
                        equipped_or_active: active_state == ActiveState::EquippedActive,
                        active_state,
                        applied_modifiers: row.applied_modifiers.clone(),
                    }
                })
                .collect(),
            selected_choices: chosen
                .selected_choices
                .iter()
                .map(|c| SelectedChoice { choice_set_id: c.choice_set_id.clone(), selection_id: c.selection_id.clone() })
                .collect(),
            selected_traits: chosen.selected_traits.clone(),
            spells_selected: chosen
                .spells_selected
                .iter()
                .map(|s| SpellSelection {
                    spell_id: s.spell_id.clone(),
                    source_class_id: s.source_class_id.clone(),
                    acquisition_mode: match s.acquisition_mode.as_str() {
                        "Prepared" => AcquisitionMode::Prepared,
                        "Granted" => AcquisitionMode::Granted,
                        _ => AcquisitionMode::Known,
                    },
                })
                .collect(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// `import_character`'s save gate (`fixtures_tests::engine_verdict`, retyped): "computes" or the
/// blocking ids.
fn verdict(input: &CharacterInput) -> String {
    let receipt = build_pilot_headless_receipt(input);
    let mut blocking: Vec<String> = receipt
        .computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id != "combat.baseline_unsupported" && d.id != "skill.selected_modifier.unsupported")
        .map(|d| d.id.clone())
        .collect();
    if blocking.is_empty() {
        if let Err(reasons) = compute_combat_baseline_from_corpus(&receipt.computation, input, app_corpus()) {
            blocking.extend(reasons);
        }
        if let Err(reasons) = compute_selected_skill_modifiers_from_corpus(&receipt.computation, input, app_corpus()) {
            blocking.extend(reasons);
        }
    }
    blocking.sort();
    blocking.dedup();
    if blocking.is_empty() { "computes".to_owned() } else { format!("refused: {}", blocking.join(", ")) }
}

fn warning_kind(warning: &str) -> &'static str {
    if warning.contains("are not applied") {
        "eqmods_not_applied"
    } else if warning.contains("copies listed as") {
        "copies_capped"
    } else if warning.contains("game mode") {
        "game_mode"
    } else if warning.contains("spell list does not take") {
        "spell_made_custom"
    } else if warning.contains("the file states no") {
        "ability_missing"
    } else if warning.contains("+2 to one ability") {
        "floating_bonus_unstated"
    } else if warning.contains("custom records") {
        "custom_record_cap"
    } else {
        "other"
    }
}

/// `equipmentSelections <key>: <w> worn, <c> carried` back into its parts.
fn mapped_parts(to: &str) -> (String, u32, u32) {
    let rest = to.strip_prefix("equipmentSelections ").unwrap_or(to);
    let (key, counts) = rest.rsplit_once(": ").unwrap_or((rest, "0 worn, 0 carried"));
    let num = |word: &str| counts.split(", ").find(|part| part.ends_with(word)).and_then(|part| part.split(' ').next()?.parse().ok()).unwrap_or(0);
    (key.to_owned(), num("worn"), num("carried"))
}

/// One file's table (TSV text) and summary.
fn diagnose(pcg: &Path, scratch: &Path) -> (String, Value) {
    let text = String::from_utf8_lossy(&std::fs::read(pcg).expect("proof file")).into_owned();
    let lines = parse(&text);
    let file = run_sidecar(pcg, scratch);
    let places = pcgen_locations(&lines);
    let equip_lines: Vec<&Line> = lines.iter().filter(|line| line.key() == "EQUIPNAME").collect();
    let reports: Vec<_> = file.report.lines.iter().filter(|line| line.key == "EQUIPNAME").collect();
    assert_eq!(equip_lines.len(), reports.len(), "{}: one report line per EQUIPNAME line", pcg.display());
    let custom_stat = |name: &str, label: &str| -> u32 {
        file.custom
            .equipment
            .iter()
            .find(|record| record.name == name)
            .and_then(|record| record.stats.iter().find(|stat| stat.label == label))
            .and_then(|stat| stat.value.parse().ok())
            .unwrap_or(0)
    };

    let mut tsv = format!("{COLUMNS}\n");
    let mut worn_so_far: Vec<(String, BodySlot)> = Vec::new();
    let mut counts: BTreeMap<&str, u64> = BTreeMap::new();
    let mut dropped_fields: BTreeMap<String, u64> = BTreeMap::new();
    let mut max_copies_one_line = 0u32;
    for (index, (line, report)) in equip_lines.iter().zip(&reports).enumerate() {
        let name = line.value();
        let (base, data) = data_fields(line);
        let custom_name = base.as_deref().is_some_and(|base| fold(base) != fold(name));
        let file_quantity = line.get("QUANTITY").unwrap_or("").to_owned();
        let quantity_value: f64 = file_quantity.trim().parse().unwrap_or(1.0);
        let mapped = report.outcome == Outcome::Mapped;
        let (key, worn, carried) = if mapped {
            mapped_parts(&report.to)
        } else {
            (String::new(), custom_stat(&report.to, "Worn"), custom_stat(&report.to, "Carried"))
        };
        let copies_emitted = if mapped { worn + carried } else { 0 };
        let slot = if mapped && worn > 0 { codex_slot(&key) } else { None };
        let sheet_record = mapped && live_sheet_rules().is_some_and(|p| p.find("equipment", &slug(key.strip_prefix("item:").unwrap_or(&key))).is_some());
        let mut collision = String::new();
        if let Some(slot) = slot.filter(|slot| *slot != BodySlot::None) {
            for _ in 0..worn {
                let worn_view: Vec<(&str, BodySlot)> = worn_so_far.iter().map(|(n, s)| (n.as_str(), *s)).collect();
                if let Ok(Some(message)) = slot_conflict(&worn_view, slot) {
                    collision = message;
                    *counts.entry("worn_copies_in_slot_conflict").or_default() += 1;
                } else {
                    worn_so_far.push((key.clone(), slot));
                }
            }
        }
        let eqmods_other: Vec<&str> = data
            .get("EQMOD")
            .map(|mods| mods.split('.').filter(|m| !m.is_empty() && (!m.starts_with("PLUS") || m.len() != 6)).collect())
            .unwrap_or_default();
        let not_read: Vec<&str> = data.keys().map(String::as_str).filter(|k| !matches!(*k, "KEY" | "EQMOD")).collect();
        for field in &not_read {
            *dropped_fields.entry((*field).to_owned()).or_default() += 1;
        }
        let pcgen_location = places.get(&fold(name)).map(|p| p.join("; ")).unwrap_or_else(|| "(no set entry)".to_owned());
        let is_pcgen_worn = places
            .get(&fold(name))
            .is_some_and(|p| p.iter().any(|place| !place.starts_with("in ") && !place.starts_with("Carried ") && !place.starts_with("Not Carried ")));

        *counts.entry("lines").or_default() += 1;
        *counts.entry(if mapped { "mapped" } else { "custom" }).or_default() += 1;
        *counts.entry("copies_emitted").or_default() += u64::from(copies_emitted);
        *counts.entry("quantity_in_file").or_default() += quantity_value.round().max(0.0) as u64;
        if quantity_value == 0.0 {
            *counts.entry("quantity_0_lines").or_default() += 1;
            *counts.entry("copies_emitted_from_quantity_0_lines").or_default() += u64::from(copies_emitted.max(worn + carried));
        }
        if copies_emitted > 1 {
            *counts.entry("lines_emitting_more_than_one_row").or_default() += 1;
        }
        if mapped && f64::from(copies_emitted) < quantity_value.round() {
            *counts.entry("lines_capped_below_file_quantity").or_default() += 1;
        }
        max_copies_one_line = max_copies_one_line.max(copies_emitted);
        if is_pcgen_worn {
            *counts.entry("lines_worn_in_pcgen").or_default() += 1;
        }
        if worn > 0 {
            *counts.entry("lines_worn_in_codex").or_default() += 1;
        }
        if custom_name {
            *counts.entry("custom_named_lines").or_default() += 1;
            if mapped && fold(&key) != fold(name) {
                *counts.entry("custom_named_lines_mapped_to_another_key").or_default() += 1;
            }
        }
        if !eqmods_other.is_empty() {
            *counts.entry("lines_with_eqmods_not_applied").or_default() += 1;
        }
        if mapped && !sheet_record {
            *counts.entry("mapped_keys_without_sheet_record").or_default() += 1;
        }
        tsv.push_str(&format!(
            "{}\t{name}\t{}\t{}\t{}\t{key}\t{}\t{worn}\t{carried}\t{copies_emitted}\t{file_quantity}\t{pcgen_location}\t{}\t{collision}\t{}\t{}\n",
            index + 1,
            base.as_deref().unwrap_or(""),
            if custom_name { "yes" } else { "no" },
            if mapped { "mapped" } else { "custom" },
            if mapped { if sheet_record { "yes" } else { "no" } } else { "" },
            slot.map(|s| format!("{s:?}")).unwrap_or_default(),
            eqmods_other.join("."),
            not_read.join(","),
        ));
    }

    let mut warnings: BTreeMap<&str, u64> = BTreeMap::new();
    for warning in &file.report.warnings {
        *warnings.entry(warning_kind(warning)).or_default() += 1;
    }
    let added_longsword = file.report.lines.iter().filter(|l| l.outcome == Outcome::Added && l.to.contains("longsword")).count();
    let equipment_rows = file.character_input.chosen.equipment_selections.len();
    let mut summary = json!({
        "game_mode": file.report.game_mode,
        "max_copies_from_one_line": max_copies_one_line,
        "equipment_selection_rows": equipment_rows,
        "custom_equipment_records": file.custom.equipment.len(),
        "data_fields_not_read": dropped_fields,
        "warnings_by_kind": warnings,
        "unmapped_classes": file.report.unmapped_classes.len(),
        "unmapped_race": file.report.unmapped_race.is_some(),
        "added_rows_naming_longsword": added_longsword,
        "class_ids": file.character_input.chosen.class_levels.iter().map(|c| c.class_id.clone()).collect::<Vec<_>>(),
        "verdict": verdict(&character_input(&file)),
    });
    for key in [
        "lines",
        "mapped",
        "custom",
        "copies_emitted",
        "quantity_in_file",
        "quantity_0_lines",
        "copies_emitted_from_quantity_0_lines",
        "lines_emitting_more_than_one_row",
        "lines_capped_below_file_quantity",
        "lines_worn_in_pcgen",
        "lines_worn_in_codex",
        "worn_copies_in_slot_conflict",
        "custom_named_lines",
        "custom_named_lines_mapped_to_another_key",
        "lines_with_eqmods_not_applied",
        "mapped_keys_without_sheet_record",
    ] {
        summary[key] = json!(counts.get(key).copied().unwrap_or(0));
    }
    (tsv, summary)
}

fn scratch_dir(label: &str) -> PathBuf {
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time").as_nanos();
    let dir = std::env::temp_dir().join(format!("codex-pcg-diagnosis-{label}-{}-{unique}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn write_summary(path: &Path, summary: &BTreeMap<String, Value>) {
    std::fs::write(path, serde_json::to_string_pretty(summary).expect("json") + "\n").expect("summary.json");
}

#[test]
fn diagnosis_tables_for_the_proof_set() {
    let samples = oracle_files();
    let missing: Vec<&str> = SAMPLES.iter().copied().filter(|s| !samples.join(format!("{s}.pcg")).is_file()).collect();
    if !missing.is_empty() {
        println!("SKIPPED: PCGen samples absent at {} ({missing:?}); run scripts/fetch-pcgen-oracle.sh", samples.display());
        return;
    }
    let scratch = scratch_dir("samples");
    let out = out_dir();
    std::fs::create_dir_all(&out).expect("artifacts dir");
    let mut summaries = BTreeMap::new();
    // `decisions.md §21` Q-A (E1.6): the drone file is a companion the sidecar refuses by name, so
    // its table is E1.1's, unchanged.
    for sample in SAMPLES.into_iter().filter(|sample| *sample != "sf_mechanic_drone") {
        let (tsv, summary) = diagnose(&samples.join(format!("{sample}.pcg")), &scratch);
        std::fs::write(out.join(format!("{sample}.tsv")), tsv).expect("table");
        summaries.insert(sample.to_owned(), summary);
    }
    write_summary(&out.join("summary.json"), &summaries);
    let shown: BTreeMap<_, _> = summaries.iter().map(|(k, v)| (k.clone(), (v["lines"].clone(), v["copies_emitted"].clone()))).collect();
    println!("samples (lines, copies_emitted): {shown:?}");

    // The operator's file: table and summary stay local; only counts are printed.
    let operator = operator_file();
    if !operator.is_file() {
        println!("SKIPPED: operator .pcg absent at {} (decisions.md §10)", operator.display());
    } else {
        let local = operator.parent().expect("a directory").join("diagnosis");
        std::fs::create_dir_all(&local).expect("local diagnosis dir");
        let (tsv, summary) = diagnose(&operator, &scratch_dir("operator"));
        std::fs::write(local.join("operator.tsv"), tsv).expect("local table");
        write_summary(&local.join("summary.json"), &BTreeMap::from([("operator".to_owned(), summary.clone())]));
        println!("operator file counts: {}", serde_json::to_string(&summary).expect("json"));
    }
    std::fs::remove_dir_all(&scratch).ok();
}
