//! Pathfinder whole-character parity at the desktop (SD-42 `decisions.md §9`, E4.13; OPERATOR Q3, Q4).
//!
//! The sidecar's gate (`crates/codex-ingest/tests/pcg_import_parity.rs`, E1.7) cannot link this crate,
//! so it reads every Pathfinder total `not-computed`. Here each Pathfinder sample (`pf_*.pcg` under
//! `$PCGEN_REPO_DIR/code/testsuite/PCGfiles`) and, on the operator's machine, the operator's file go
//! the way a player's import goes: the converter the sidecar runs, the app's `import_pcg_json` into a
//! fresh root, then `load_saved_character_at_root`. The sheet's own numbers -- the effective ability
//! scores and the engine's explanation rows -- are compared with PCGen's export of the same file
//! (`scripts/oracle_harness/pcg_import_parity/exports/<sample>.txt`, the real engine at the pin):
//! the six scores, HP, AC, base attack, melee and ranged attack, initiative, the three saves, every
//! skill PCGen lists, every weapon's attack, and spells per day and DC by class and level.
//!
//! A difference is a row of `explained_desktop.tsv` beside the sidecar's `explained.tsv`, in its shape
//! and with its rules (`build, field, pcgen, codex, why, decision`; a field ending `*` is a prefix; a
//! `*` value matches any; a row that explains no difference fails). The operator's rows stay in
//! `$HOME/sd42-input/parity/explained_desktop.tsv` (`decisions.md §10`); only counts are printed.
//!
//! **Does not cover:** a skill PCGen leaves out of its export (a trained-only skill with no ranks;
//! Codex's number for it is not compared); a weapon's damage, critical or range (attack only); a
//! weapon only one side lists (PCGen lists every weapon held, a mount's too; the sidecar's gate
//! compares where each line is); a thrown use or a Power Attack line of a weapon; a weapon is matched
//! by its name's words with `+N`, `Masterwork` and `Small` removed, so two weapons of one kind compare
//! as one (E4.15); feats, spells, gear and money (the sidecar's gate compares them);
//! Starfinder (E1.7, E1.10, E4.14); any `.pcg` but the 4 samples and the operator's file.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex_ingest::pcg_import::catalog::{Catalog, fold};

use super::*;

const HARNESS: &str = "scripts/oracle_harness/pcg_import_parity";
const LEDGER: &str = "explained_desktop.tsv";
const SAMPLES: [&str; 4] = ["pf_Cleric", "pf_Paladin", "pf_Rogue", "pf_goldielocks"];
/// The sidecar's `why` vocabulary (`pcg_import_parity.rs` `WHYS`).
const WHYS: [&str; 6] = ["fixable", "desktop-computes", "engine-does-not-compute", "oracle-departs", "codex-rule", "import-refuses"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
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

/// `+3` / `-1` / `+10/+5` / `3+1` -> the first number, or the sum of a `3+1` (a domain slot).
fn number(text: &str) -> Option<i64> {
    let first = text.trim().split('/').next()?.trim();
    let body = first.strip_prefix('+').unwrap_or(first);
    body.parse::<i64>().ok().or_else(|| body.parse::<f64>().ok().map(|f| f.round() as i64)).or_else(|| body.split('+').map(|p| p.trim().parse::<i64>().ok()).sum())
}

/// A weapon as both sides name it: its words, lower case and sorted, without `+N`, `Masterwork` or
/// `Small` (the character's size, not the weapon's kind).
fn weapon_key(name: &str) -> String {
    let name = name.trim_start_matches('*');
    let mut words: Vec<String> = name
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '+')
        .filter(|w| !w.is_empty() && !w.starts_with('+') && !["masterwork", "small"].iter().any(|skip| w.eq_ignore_ascii_case(skip)))
        .map(str::to_ascii_lowercase)
        .collect();
    words.sort();
    words.join("_")
}

/// A skill as both sides key it; PCGen's `Craft (Untrained)` and `Linguistics(Varisian)` are Codex's
/// one Craft and one Linguistics.
fn skill_key(name: &str) -> String {
    let name = name.strip_suffix(" (Untrained)").unwrap_or(name);
    let name = if name.starts_with("Linguistics") { "Linguistics" } else { name };
    catalog().skill_id(name).map_or_else(|| fold(name), |id| fold(id.rsplit(':').next().unwrap_or(&id)))
}

/// PCGen's export of a Pathfinder character, as `field -> value` (the module doc's totals).
fn pcgen_view(text: &str) -> BTreeMap<String, String> {
    let kv: BTreeMap<&str, &str> = text.lines().filter_map(|line| line.split_once('=')).collect();
    let get = |key: &str| kv.get(key).copied().unwrap_or("");
    // A gear line's output name -> its long name (the `.pcg`'s, the one Codex keys a record by).
    let long_of: BTreeMap<&str, &str> = kv
        .iter()
        .filter(|(key, _)| key.starts_with("pcg.gear."))
        .filter_map(|(_, value)| {
            let parts: Vec<&str> = value.split('|').collect();
            Some((parts[0], parts.iter().find_map(|p| p.strip_prefix("long="))?))
        })
        .collect();
    let mut out = BTreeMap::new();
    let mut put = |key: String, value: Option<i64>| {
        if let Some(v) = value {
            out.entry(key).or_insert_with(|| v.to_string());
        }
    };
    for (key, value) in &kv {
        let parts: Vec<&str> = key.split('.').collect();
        match parts.as_slice() {
            ["STAT", i, "SCORE"] => put(format!("stat.{}", get(&format!("STAT.{i}.NAME"))), number(value)),
            ["HP"] => put("hp".into(), number(value)),
            ["AC", "TOTAL"] => put("ac".into(), number(value)),
            ["BAB"] => put("bab".into(), number(value)),
            ["ATTACK", kind @ ("MELEE" | "RANGED"), "TOTAL"] => put(format!("attack.{}", kind.to_ascii_lowercase()), number(value)),
            ["INITIATIVEMOD"] => put("initiative".into(), number(value)),
            ["CHECK", i, "TOTAL"] => put(format!("save.{}", get(&format!("CHECK.{i}.NAME")).to_ascii_lowercase()), number(value)),
            ["SKILL", i, "TOTAL"] => put(format!("skill.{}", skill_key(get(&format!("SKILL.{i}.NAME")))), number(value)),
            ["WEAPON", i, "TOTALHIT"] => {
                // A thrown use and a Power Attack line are modes of a weapon, not weapons.
                let name = get(&format!("WEAPON.{i}.NAME")).trim_start_matches('*');
                if !name.contains("Thrown") && !name.contains('[') {
                    put(format!("weapon.{}.attack", weapon_key(long_of.get(name).copied().unwrap_or(name))), number(value));
                }
            }
            ["SPELLLISTCAST", sc, level] => {
                let per_day = number(value).unwrap_or(0);
                if per_day > 0 {
                    let class = fold(get(&format!("SPELLLISTCLASS.{sc}")));
                    put(format!("spells.{class}.{level}.per_day"), Some(per_day));
                    put(format!("spells.{class}.{level}.dc"), number(get(&format!("SPELLLISTDC.{sc}.{level}"))));
                }
            }
            _ => {}
        }
    }
    out
}

/// The sheet's view of the imported character, keyed like [`pcgen_view`]: the load response's
/// effective scores and the engine's explanation rows, nothing summed here.
fn codex_view(sheet: &LoadSavedCharacterResponse) -> BTreeMap<String, String> {
    let s = &sheet.ability_scores;
    let mut out: BTreeMap<String, String> = [("STR", s.strength), ("DEX", s.dexterity), ("CON", s.constitution), ("INT", s.intelligence), ("WIS", s.wisdom), ("CHA", s.charisma)]
        .into_iter()
        .map(|(name, score)| (format!("stat.{name}"), score.to_string()))
        .collect();
    for row in &sheet.explanations {
        let id = row.id.as_str();
        let field = match id {
            "hit_points.max" => Some("hp".to_owned()),
            "defense.baseline_armor_class" => Some("ac".to_owned()),
            "class_chassis.base_attack_bonus" => Some("bab".to_owned()),
            "combat.baseline_melee_attack_bonus" => Some("attack.melee".to_owned()),
            "combat.baseline_ranged_attack_bonus" => Some("attack.ranged".to_owned()),
            "initiative.total" => Some("initiative".to_owned()),
            _ => None,
        }
        .or_else(|| id.strip_prefix("defense.total_save.").map(|save| format!("save.{save}")))
        .or_else(|| id.strip_prefix("skill.total.").map(|skill| format!("skill.{}", fold(skill))))
        .or_else(|| {
            // The row's own words name the weapon: "Attack bonus with the equipped <name>: ...".
            id.strip_prefix("combat.weapon_attack_bonus.")?;
            let name = row.detail.split_once("with the equipped ")?.1.split_once(':')?.0;
            Some(format!("weapon.{}.attack", weapon_key(name)))
        })
        .or_else(|| {
            let rest = id.strip_prefix("class_chassis.")?;
            let (class, rest) = rest.split_once('.')?;
            let rest = rest.strip_prefix("partial_caster.").unwrap_or(rest);
            let (kind, level) = rest.split_once(".spell_level_")?;
            let kind = match kind {
                "total_spells_per_day" => "per_day",
                "spell_save_dc" => "dc",
                _ => return None,
            };
            Some(format!("spells.{class}.{level}.{kind}"))
        });
        if let Some(field) = field {
            out.entry(field).or_insert_with(|| row.value.to_string());
        }
    }
    out
}

/// `(field, pcgen, codex)` over PCGen's fields; a skill PCGen leaves out of its export, and a weapon
/// only one side lists, are not compared (module doc).
fn compare(pcgen: &BTreeMap<String, String>, codex: &BTreeMap<String, String>) -> Vec<(String, String, String)> {
    let fields: BTreeSet<&String> = pcgen.keys().filter(|f| !f.starts_with("weapon.") || codex.contains_key(*f)).collect();
    fields
        .into_iter()
        .map(|f| (f.clone(), pcgen.get(f).cloned().unwrap_or_else(|| "absent".into()), codex.get(f).cloned().unwrap_or_else(|| "absent".into())))
        .collect()
}

#[derive(Debug)]
struct Explained {
    build: String,
    field: String,
    pcgen: String,
    codex: String,
}

impl Explained {
    fn covers(&self, field: &str, pcgen: &str, codex: &str) -> bool {
        let cell = |row: &str, value: &str| row == "*" || row == value;
        let field_matches = match self.field.strip_suffix('*') {
            Some(prefix) => field.starts_with(prefix),
            None => self.field == field,
        };
        field_matches && cell(&self.pcgen, pcgen) && cell(&self.codex, codex)
    }
}

fn ledger(path: &Path) -> Vec<Explained> {
    let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
    text.lines()
        .skip(1)
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let c: Vec<&str> = line.split('\t').collect();
            assert_eq!(c.len(), 6, "{}: 6 tab-separated cells: {line:?}", path.display());
            assert!(WHYS.contains(&c[4]), "{}: why is one of {WHYS:?}: {line:?}", path.display());
            assert!(!c[5].trim().is_empty(), "{}: a decision: {line:?}", path.display());
            Explained { build: c[0].into(), field: c[1].into(), pcgen: c[2].into(), codex: c[3].into() }
        })
        .collect()
}

/// Imports `pcg` as a player does and loads it, or the ids of the engine's refusal; the scratch root
/// is removed after.
fn sheet_of(pcg: &Path, label: &str) -> Result<LoadSavedCharacterResponse, String> {
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let scratch = std::env::temp_dir().join(format!("codex-pcg-parity-{label}-{}-{unique}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    let out = scratch.join("out.json");
    codex_ingest::pcg_import::run(pcg, &out, &repo_root()).unwrap_or_else(|e| panic!("{label}: the converter reads the file: {e}"));
    let root = scratch.join("characters").join(label);
    let response = import_pcg_json(&std::fs::read_to_string(&out).unwrap(), &root, label, "2026-11-19T00:00:00Z", "0.0.0-test").expect("the import runs");
    let sheet = match response.outcome {
        CreateCharacterResponse::Saved { .. } => Ok(load_saved_character_at_root(&root).expect("the imported character loads")),
        CreateCharacterResponse::Blocked { diagnostics } => {
            Err(diagnostics.iter().filter(|d| d.claim_blocking).map(|d| d.id.as_str()).collect::<Vec<_>>().join(","))
        }
    };
    let _ = std::fs::remove_dir_all(&scratch);
    sheet
}

/// One build against its ledger rows: `Ok((fields, equal, explained))`, or every failure.
fn gate(build: &str, pcg: &Path, export: &str, rows: &[Explained]) -> Result<(usize, usize, usize), Vec<String>> {
    let compared = match sheet_of(pcg, build) {
        Ok(sheet) => compare(&pcgen_view(export), &codex_view(&sheet)),
        // A refused import compares one field: whether it computes.
        Err(refused) => vec![("import".to_owned(), "computes".to_owned(), format!("refused: {refused}"))],
    };
    let mine: Vec<&Explained> = rows.iter().filter(|row| row.build == build).collect();
    let mut used = vec![false; mine.len()];
    let (mut equal, mut explained, mut failures) = (0, 0, Vec::new());
    for (field, pcgen, codex) in &compared {
        if pcgen == codex {
            equal += 1;
            continue;
        }
        match mine.iter().position(|row| row.covers(field, pcgen, codex)) {
            Some(at) => {
                used[at] = true;
                explained += 1;
            }
            None => failures.push(format!("{build}\t{field}\t{pcgen}\t{codex}\t(unexplained)")),
        }
    }
    failures.extend(mine.iter().zip(used).filter(|(_, used)| !used).map(|(row, _)| format!("stale ledger row (explains no difference): {row:?}")));
    if failures.is_empty() { Ok((compared.len(), equal, explained)) } else { Err(failures) }
}

fn sample(name: &str) {
    let pcg = oracle_dir().join("code/testsuite/PCGfiles").join(format!("{name}.pcg"));
    if !pcg.is_file() {
        println!("SKIPPED: the oracle's {name}.pcg is absent at {} (decisions.md §22)", pcg.display());
        return;
    }
    let harness = repo_root().join(HARNESS);
    let export = std::fs::read_to_string(harness.join("exports").join(format!("{name}.txt"))).expect("the committed export; run run.sh");
    match gate(name, &pcg, &export, &ledger(&harness.join(LEDGER))) {
        Ok((fields, equal, explained)) => println!("{name} (desktop): fields compared {fields}, equal {equal}, explained {explained}"),
        Err(failures) => panic!("{} failures:\n{}", failures.len(), failures.join("\n")),
    }
}

#[test]
fn pcg_import_parity_desktop_pf_cleric() {
    sample("pf_Cleric");
}

#[test]
fn pcg_import_parity_desktop_pf_paladin() {
    sample("pf_Paladin");
}

#[test]
fn pcg_import_parity_desktop_pf_rogue() {
    sample("pf_Rogue");
}

#[test]
fn pcg_import_parity_desktop_pf_goldielocks() {
    sample("pf_goldielocks");
}

#[test]
fn pcg_import_parity_desktop_ledger_names_only_the_samples() {
    let rows = ledger(&repo_root().join(HARNESS).join(LEDGER));
    let unknown: Vec<&Explained> = rows.iter().filter(|row| !SAMPLES.contains(&row.build.as_str())).collect();
    assert!(unknown.is_empty(), "ledger rows for no sample: {unknown:?}");
}

/// The operator's file (`decisions.md §10`): `$CODEX_SD42_PCG`, else the one `.pcg` in
/// `$HOME/sd42-input/`; its export and ledger stay under `$HOME/sd42-input/parity/`. Counts only.
#[test]
fn pcg_import_parity_desktop_operator_file() {
    let input = PathBuf::from(std::env::var("HOME").expect("HOME")).join("sd42-input");
    let pcg = match std::env::var("CODEX_SD42_PCG") {
        Ok(path) if !path.is_empty() => PathBuf::from(path),
        _ => std::fs::read_dir(&input).into_iter().flatten().flatten().map(|e| e.path()).find(|p| p.extension().is_some_and(|ext| ext == "pcg")).unwrap_or_else(|| input.join("<operator>.pcg")),
    };
    if !pcg.is_file() {
        println!("SKIPPED: operator .pcg absent at {} (decisions.md §10)", pcg.display());
        return;
    }
    let dir = input.join("parity");
    let Ok(export) = std::fs::read_to_string(dir.join(format!("{}.txt", pcg.file_stem().unwrap().to_string_lossy()))) else {
        println!("SKIPPED: the operator's export is absent under {} (run.sh --operator; decisions.md §10)", dir.display());
        return;
    };
    if export.lines().any(|line| line.starts_with("name=")) {
        println!("SKIPPED: the operator's file is a Starfinder character (E1.7 compares it)");
        return;
    }
    match gate("operator", &pcg, &export, &ledger(&dir.join(LEDGER))) {
        Ok((fields, equal, explained)) => println!("operator (desktop): fields compared {fields}, equal {equal}, explained {explained}"),
        Err(failures) => panic!("{} failures:\n{}", failures.len(), failures.join("\n")),
    }
}
