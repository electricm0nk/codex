//! `sheet_rule_parity` -- the engine side of the oracle parity run (SD-35 AT-35-E2-005,
//! `technical-design.md §1` "Verification, with PCGen as the oracle").
//!
//! Tool side. For every character fixture in a roster directory (the `key=value` character
//! input format `tests/fixtures/rules_core/*_input.txt` uses), it runs the live chassis
//! (`compute_pilot_base_chassis`) and the live evaluator (`render_sheet`, through
//! `with_sheet_rules` -- the same path the desktop's `sheet_lines_for` takes, race traits from
//! the same race resolver over the same book list), and writes every rendered line with its
//! rule's sheet-total target and bonus type as JSON. `scripts/oracle_harness/sheet_parity.py
//! compare` joins that against PCGen's BatchExporter export of the same characters
//! (`scripts/oracle_harness/sheet-totals.txt.ftl`). This binary reads no PCGen data.
//!
//! ```text
//! cargo run --locked --release --bin sheet_rule_parity -- --roster <dir> --output <ours.json>
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use codex::rules_core::character_input::{load_character_input_fixture, CharacterInput};
use codex::rules_core::corpus_loader::{load_sheet_rules, BookCorpusRoot};
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};
use codex::rules_core::race_resolver::{load_race_corpus, RaceCorpus};
use codex::rules_core::sheet_rule::{BonusTarget, BonusType, CharacterFacts, SheetLine, SheetLineValue, SheetRulePackage};
use serde::Serialize;

/// One roster character as the engine computed it.
#[derive(Serialize)]
struct OursCharacter {
    character: String,
    race: String,
    classes: Vec<(String, u8)>,
    /// Str, Dex, Con, Int, Wis, Cha.
    ability_scores: [i16; 6],
    chassis: Chassis,
    facts: FactsOut,
    race_traits: Vec<String>,
    lines: Vec<LineOut>,
    diagnostics: Vec<String>,
}

/// The chassis totals the sheet-rule lines feed (the existing interpreter's numbers).
#[derive(Serialize)]
struct Chassis {
    ability_mods: [i16; 6],
    bab: i16,
    base_saves: [i16; 3],
    total_saves: [i16; 3],
    baseline_ac: i16,
    baseline_melee_attack: i16,
    skills: BTreeMap<String, i16>,
}

#[derive(Serialize)]
struct FactsOut {
    level: i64,
    size: i64,
    speeds: BTreeMap<String, i64>,
    highest_spell_level: i64,
}

/// A rendered line plus what the rule says it feeds.
#[derive(Serialize)]
struct LineOut {
    #[serde(flatten)]
    line: SheetLine,
    target: Option<BonusTarget>,
    bonus_type: Option<BonusType>,
    /// `number` / `dice` / `words` -- the form of `line.value`.
    form: &'static str,
}

fn form_of(v: &SheetLineValue) -> &'static str {
    match v {
        SheetLineValue::Resolved(_) => "number",
        SheetLineValue::Dice(_) => "dice",
        SheetLineValue::Words => "words",
    }
}

/// The desktop's race corpus book list, read from its one declaration so this tool and the
/// live app resolve racial traits over the same books (the same read
/// `race_resolver`'s own pin test performs).
fn race_corpus_books(repo: &Path) -> Vec<String> {
    let src = std::fs::read_to_string(repo.join("apps/desktop/src-tauri/src/race_catalog.rs"))
        .expect("apps/desktop/src-tauri/src/race_catalog.rs is readable");
    let decl = src
        .split("pub(crate) const RACE_CORPUS_BOOKS: &[&str] =")
        .nth(1)
        .expect("RACE_CORPUS_BOOKS is declared in race_catalog.rs");
    let list = decl.split(';').next().expect("the declaration terminates");
    list.split('"').skip(1).step_by(2).map(str::to_owned).collect()
}

fn load_races(repo: &Path) -> RaceCorpus {
    let books = race_corpus_books(repo);
    let dirs: Vec<PathBuf> = books.iter().map(|b| repo.join("data/corpus").join(b)).collect();
    let roots: Vec<BookCorpusRoot<'_>> =
        books.iter().zip(dirs.iter()).map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() }).collect();
    load_race_corpus(&roots)
}

/// Applied racial-trait keys for the character, as the desktop's `resolve_racial_traits_for_character`
/// resolves them: the race's default traits (no alternate selected on a roster fixture).
fn race_trait_keys(races: &RaceCorpus, input: &CharacterInput) -> (Vec<String>, Option<String>) {
    let Some(key) = races.resolve_key(&input.chosen.race_id) else {
        return (Vec::new(), Some(format!("no race in the loaded corpus matches {:?}", input.chosen.race_id)));
    };
    let key = key.to_string();
    match races.resolve(&key, &[]) {
        Some(race) => (race.traits.iter().map(|t| t.key.clone()).collect(), None),
        None => (Vec::new(), Some(format!("race {key:?} has no chassis record"))),
    }
}

fn compute_one(name: &str, text: &str, package: &SheetRulePackage, races: &RaceCorpus) -> OursCharacter {
    let loaded = load_character_input_fixture(text);
    let mut diagnostics: Vec<String> = loaded.diagnostics.iter().map(|d| format!("{d:?}")).collect();
    let Some(input) = loaded.character_input else {
        return OursCharacter {
            character: name.to_string(),
            race: String::new(),
            classes: Vec::new(),
            ability_scores: [0; 6],
            chassis: Chassis {
                ability_mods: [0; 6],
                bab: 0,
                base_saves: [0; 3],
                total_saves: [0; 3],
                baseline_ac: 0,
                baseline_melee_attack: 0,
                skills: BTreeMap::new(),
            },
            facts: FactsOut { level: 0, size: 0, speeds: BTreeMap::new(), highest_spell_level: 0 },
            race_traits: Vec::new(),
            lines: Vec::new(),
            diagnostics,
        };
    };
    let base: PilotBaseChassisComputation = compute_pilot_base_chassis(&input);
    let (race_traits, race_diag) = race_trait_keys(races, &input);
    diagnostics.extend(race_diag);
    let computed = base.with_sheet_rules(&input, package, &race_traits);
    let facts = CharacterFacts::from_character(&input, &computed);
    let s = &input.chosen.ability_scores;
    let m = &computed.ability_modifiers;
    let lines = computed
        .sheet_lines
        .iter()
        .map(|line| {
            let rule = package.rule(&line.id);
            LineOut {
                form: form_of(&line.value),
                target: rule.and_then(|r| r.target.clone()),
                bonus_type: rule.and_then(|r| r.bonus_type.clone()),
                line: line.clone(),
            }
        })
        .collect();
    let mut skills = BTreeMap::new();
    skills.insert("climb".to_string(), computed.selected_skill_modifiers.climb);
    skills.insert("intimidate".to_string(), computed.selected_skill_modifiers.intimidate);
    skills.insert("swim".to_string(), computed.selected_skill_modifiers.swim);
    OursCharacter {
        character: name.to_string(),
        race: input.chosen.race_id.clone(),
        classes: input.chosen.class_levels.iter().map(|c| (c.class_id.clone(), c.level)).collect(),
        ability_scores: [s.strength, s.dexterity, s.constitution, s.intelligence, s.wisdom, s.charisma],
        chassis: Chassis {
            ability_mods: [m.strength, m.dexterity, m.constitution, m.intelligence, m.wisdom, m.charisma],
            bab: computed.base_attack_bonus,
            base_saves: [computed.base_saves.fortitude, computed.base_saves.reflex, computed.base_saves.will],
            total_saves: [computed.total_saves.fortitude, computed.total_saves.reflex, computed.total_saves.will],
            baseline_ac: computed.baseline_armor_class,
            baseline_melee_attack: computed.baseline_melee_attack_bonus,
            skills,
        },
        facts: FactsOut {
            level: facts.level,
            size: facts.size,
            speeds: facts.speeds.clone(),
            highest_spell_level: facts.highest_spell_level,
        },
        race_traits,
        lines,
        diagnostics,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let (Some(roster), Some(output)) = (arg("--roster"), arg("--output")) else {
        eprintln!("usage: sheet_rule_parity --roster <dir of *.txt character inputs> --output <ours.json>");
        std::process::exit(2);
    };
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let started = std::time::Instant::now();
    let load = load_sheet_rules(&repo.join("data/sheet_rules"));
    if !load.diagnostics.is_empty() {
        eprintln!("sheet_rule_parity: {} package diagnostic(s); first: {:?}", load.diagnostics.len(), load.diagnostics[0]);
        std::process::exit(2);
    }
    let races = load_races(&repo);
    eprintln!(
        "sheet_rule_parity: package {} rule files, {} rules; race corpus {} keys; loaded in {:.1}s",
        load.rule_files,
        load.package.rules.len(),
        races.race_keys().len(),
        started.elapsed().as_secs_f64()
    );
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&roster)
        .unwrap_or_else(|e| {
            eprintln!("sheet_rule_parity: reading {roster}: {e}");
            std::process::exit(2);
        })
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();
    let mut out = Vec::new();
    for path in &paths {
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let t = std::time::Instant::now();
        let c = compute_one(&name, &text, &load.package, &races);
        let numbers = c.lines.iter().filter(|l| l.form == "number").count();
        eprintln!(
            "sheet_rule_parity: {name}: {} lines ({numbers} number) in {:.2}s{}",
            c.lines.len(),
            t.elapsed().as_secs_f64(),
            if c.diagnostics.is_empty() { String::new() } else { format!("; {} diagnostic(s)", c.diagnostics.len()) }
        );
        out.push(c);
    }
    let doc = serde_json::json!({
        "generated_by": "sheet_rule_parity",
        "roster": roster,
        "characters": out,
    });
    std::fs::write(&output, serde_json::to_string_pretty(&doc).unwrap()).unwrap_or_else(|e| {
        eprintln!("sheet_rule_parity: writing {output}: {e}");
        std::process::exit(2);
    });
    println!(
        "sheet_rule_parity: characters={} lines={} wall={:.1}s -> {output}",
        doc["characters"].as_array().map_or(0, Vec::len),
        doc["characters"].as_array().map_or(0, |cs| cs.iter().map(|c| c["lines"].as_array().map_or(0, Vec::len)).sum()),
        started.elapsed().as_secs_f64()
    );
}
