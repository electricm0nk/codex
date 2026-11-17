//! Whole-character parity of the import against the PCGen engine (SD-42 `decisions.md §9`, E1.7).
//!
//! Each proof file -- PCGen's 7 sample characters (`$PCGEN_REPO_DIR/code/testsuite/PCGfiles`) and,
//! on the operator's machine, the operator's file (`$CODEX_SD42_PCG`) -- is rendered twice: by the
//! REAL PCGen engine at the pinned oracle (`scripts/oracle_harness/pcg_import_parity/run.sh`; the
//! samples' exports are committed under `exports/`, the operator's stay under
//! `/home/ubuntu/sd42-input/parity/`), and by Codex: `pcg_import::convert`, then, for Starfinder,
//! the engine the sidecar links (the root crate's `sf_*` modules, `map::starfinder_build`). Every
//! field is compared: classes and levels, race, the six ability scores, HP (and Stamina, Resolve),
//! the saves, AC (EAC, KAC), initiative, base, melee and ranged attack, every skill total, every
//! feat (a Starfinder theme with them), every chosen spell, spells per day and DC, every weapon's
//! attack, every gear line's pieces at its place, and the money.
//!
//! A difference is a row of `explained.tsv` (`build, field, pcgen, codex, why, decision`); a field
//! ending `*` covers every field with that prefix, and a `*` value matches any value. A ledger row
//! that explains no difference fails (a stale row hides a fix nobody checked). Each export must be
//! of its file: its character name is the file's.
//!
//! Every Pathfinder total (HP, AC, base and melee/ranged attack, initiative, saves, skills, weapons,
//! spells per day and DC) is assembled in the desktop crate (`character_hub.rs`, `pf1_adapter.rs`,
//! over the desktop's own corpus), which this crate cannot link (SD-36 D1): it reads `not-computed`
//! here and its ledger row names E4.13, which compares it there. A Pathfinder ability score here is
//! the import's (after the race, as Create saves it).
//!
//! **Does not cover:** the Pathfinder totals above (E4.13); a `.pcg` outside the 8 files; the
//! drone's totals (Codex refuses the file, `§21` Q-A); a parameterised feat's parameter (`Spell
//! Focus (Enchantment)` compares as Spell Focus); a prepared caster's class list (only spells it
//! chose or prepared); the Starfinder bulk condition the desktop folds in (`sf_adapter.rs`
//! `apply_bulk_condition`); weights and prices per line; PCGen prose.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::pilot_compute::{sf_attack, sf_chassis, sf_defense, sf_loadout, sf_skills, sf_spells};
use codex::saved_character::equipment_sets::{CopyKey, Location, location_of};
use codex_ingest::pcg_import::catalog::{Catalog, fold};
use codex_ingest::pcg_import::map::{character_input_of, convert, starfinder_build};
use codex_ingest::pcg_import::parse::parse;
use codex_ingest::pcg_import::wire::ImportFile;

const SAMPLES: [&str; 7] = ["pf_Cleric", "pf_Paladin", "pf_Rogue", "pf_goldielocks", "sf_mechanic", "sf_mechanic_drone", "sf_soldier"];
const HARNESS: &str = "scripts/oracle_harness/pcg_import_parity";
const NOT_COMPUTED: &str = "not-computed";
/// The Pathfinder totals the desktop crate computes: a field, or a prefix ending `.`.
const PF_DESKTOP_TOTALS: [&str; 9] = ["hp", "ac", "bab", "initiative", "attack.", "save.", "skill.", "weapon.", "spells."];
/// Why a difference stands: `fixable` (the decision names the discovered card), `desktop-computes`
/// (a total this crate cannot reach; the card that compares it there), `engine-does-not-compute`,
/// `oracle-departs` (PCGen's number is not the rule's), `codex-rule` (a deliberate Codex rule),
/// `import-refuses` (`§21` Q-A).
const WHYS: [&str; 6] = ["fixable", "desktop-computes", "engine-does-not-compute", "oracle-departs", "codex-rule", "import-refuses"];

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

/// `+3` / `-1` / `+9/+4` / `20 ft.` / `3+1` -> the first number, or the sum of a `3+1`.
fn number(text: &str) -> Option<i64> {
    let first = text.trim().split('/').next()?.trim();
    let first = first.split_whitespace().next()?;
    let body = first.strip_prefix('+').unwrap_or(first);
    if let Ok(n) = body.parse::<i64>() {
        return Some(n);
    }
    if let Ok(f) = body.parse::<f64>() {
        return Some(f.round() as i64);
    }
    // `3+1`: PCGen's per-day with a domain slot.
    body.split('+').map(|part| part.trim().parse::<i64>().ok()).sum()
}

fn id_tail(id: &str) -> String {
    fold(id.rsplit(':').next().unwrap_or(id))
}

/// PCGen's export, normalised to `field -> value` (module doc).
fn pcgen_view(text: &str, starfinder: bool) -> BTreeMap<String, String> {
    let catalog = if starfinder { catalog().for_game_mode("Starfinder").expect("the Starfinder catalog") } else { catalog() };
    let kv: Vec<(&str, &str)> = text.lines().filter_map(|line| line.split_once('=')).collect();
    let get = |key: &str| kv.iter().find(|(k, _)| *k == key).map(|(_, v)| *v);
    let mut out = BTreeMap::new();
    let put = |out: &mut BTreeMap<String, String>, key: String, value: Option<i64>| {
        if let Some(v) = value {
            out.insert(key, v.to_string());
        }
    };
    let race = get(if starfinder { "race" } else { "RACE" }).unwrap_or("");
    // `Human (Varisian)`: the race by its stem when the full name is no record.
    let race_entry = catalog.race(race).or_else(|| catalog.race(race.split(" (").next().unwrap_or(race)));
    out.insert("race".into(), race_entry.map_or_else(|| fold(race), |entry| id_tail(&entry.id)));
    // PCGen prints a gear line, a container and a weapon by its output name; Codex keys a record by
    // the `.pcg`'s own name, which is the line's long name.
    let long_of: BTreeMap<&str, &str> = kv
        .iter()
        .filter(|(key, _)| key.starts_with("pcg.gear."))
        .filter_map(|(_, value)| {
            let parts: Vec<&str> = value.split('|').collect();
            Some((parts[0], parts.iter().find_map(|p| p.strip_prefix("long="))?))
        })
        .collect();
    let long = |name: &str| long_of.get(name).copied().unwrap_or(name).to_owned();
    let mut gear: BTreeMap<String, i64> = BTreeMap::new();
    for (key, value) in &kv {
        if let Some(class) = key.strip_prefix("pcg.class.") {
            out.insert(format!("class.{}", catalog.class_id(class).map_or_else(|| fold(class), id_tail)), value.to_string());
        } else if *key == "pcg.gold" {
            put(&mut out, "money".into(), value.trim().replace(',', "").parse::<f64>().ok().map(|gp| (gp * 100.0).round() as i64));
        } else if key.starts_with("pcg.gear.") {
            let parts: Vec<&str> = value.split('|').collect();
            let qty = parts.iter().find_map(|p| p.strip_prefix("qty=")).and_then(number).unwrap_or(0);
            let location = parts.iter().find_map(|p| p.strip_prefix("location=")).unwrap_or("");
            let place = match location.trim() {
                // In no carried or worn place of the set (a mount): what Codex calls Not Carried.
                "" => "notcarried".to_owned(),
                word @ ("Equipped" | "Carried" | "Not Carried") => fold(word),
                container => gear_key(catalog, &long(container)),
            };
            *gear.entry(format!("gear.{}@{place}", gear_key(catalog, &long(parts[0])))).or_insert(0) += qty;
        }
    }
    out.extend(gear.into_iter().map(|(k, v)| (k, v.to_string())));
    if starfinder {
        for (key, value) in &kv {
            let parts: Vec<&str> = value.split('|').collect();
            if let Some(stat) = key.strip_prefix("stat.") {
                put(&mut out, format!("stat.{stat}"), number(value));
            } else if let Some(skill) = key.strip_prefix("skill.") {
                let field = |p: &str| parts.iter().find_map(|x| x.strip_prefix(p)).unwrap_or("");
                let untrained = field("untrained=") == "NO" && number(field("ranks=")) == Some(0);
                let id = catalog.skill_id(skill).map_or_else(|| fold(skill), |id| id_tail(&id));
                if untrained {
                    out.insert(format!("skill.{id}"), "untrained".into());
                } else {
                    put(&mut out, format!("skill.{id}"), number(parts[0]));
                }
            } else if let Some(weapon) = key.strip_prefix("weapon.") {
                let name = gear_key(catalog, &long(weapon.trim_start_matches('*')));
                put(&mut out, format!("weapon.{name}.attack"), number(parts[0]));
            } else if let Some(kind) = key.strip_prefix("abilities.").filter(|a| matches!(*a, "feat" | "theme")) {
                // Codex holds the theme among the feats; PCGen's theme list is the theme, then its features.
                let names: Vec<&str> = value.split(',').map(str::trim).filter(|f| !f.is_empty()).collect();
                let names = if kind == "theme" { &names[..names.len().min(1)] } else { &names[..] };
                for feat in names.iter().copied() {
                    out.insert(format!("feat.{}", feat_name(catalog, feat)), "held".into());
                }
            } else if let Some(rest) = key.strip_prefix("spells.") {
                let Some((class, level)) = rest.rsplit_once('.') else { continue };
                let field = |p: &str| parts.iter().find_map(|x| x.strip_prefix(p)).unwrap_or("");
                if number(field("per_day:")).unwrap_or(0) == 0 && number(field("known:")).unwrap_or(0) == 0 {
                    continue;
                }
                put(&mut out, format!("spells.{}.{level}.per_day", fold(class)), number(field("per_day:")));
                put(&mut out, format!("spells.{}.{level}.known", fold(class)), number(field("known:")));
            } else if matches!(*key, "hp" | "stamina" | "resolve" | "eac" | "kac" | "initiative" | "bab" | "attack.melee" | "attack.ranged") || key.starts_with("save.") {
                put(&mut out, (*key).to_owned(), number(value));
            }
        }
        return out;
    }
    let stat_name = |i: &str| get(&format!("STAT.{i}.NAME")).unwrap_or("").to_owned();
    let mut spell_class: BTreeMap<String, String> = BTreeMap::new();
    for (key, value) in &kv {
        let parts: Vec<&str> = key.split('.').collect();
        match parts.as_slice() {
            ["STAT", i, "SCORE"] => put(&mut out, format!("stat.{}", stat_name(i)), number(value)),
            ["HP"] => put(&mut out, "hp".into(), number(value)),
            ["AC", "TOTAL"] => put(&mut out, "ac".into(), number(value)),
            ["BAB"] => put(&mut out, "bab".into(), number(value)),
            ["ATTACK", kind @ ("MELEE" | "RANGED"), "TOTAL"] => put(&mut out, format!("attack.{}", kind.to_ascii_lowercase()), number(value)),
            ["INITIATIVEMOD"] => put(&mut out, "initiative".into(), number(value)),
            ["CHECK", i, "TOTAL"] => put(&mut out, format!("save.{}", get(&format!("CHECK.{i}.NAME")).unwrap_or("")), number(value)),
            ["SKILL", i, "TOTAL"] => {
                let name = get(&format!("SKILL.{i}.NAME")).unwrap_or("");
                put(&mut out, format!("skill.{}", catalog.skill_id(name).map_or_else(|| fold(name), |id| id_tail(&id))), number(value));
            }
            ["FEAT", _, "NAME"] => {
                out.insert(format!("feat.{}", feat_name(catalog, value)), "held".into());
            }
            // A spell the character chose: in the Known Spells book (0) at a level where its class
            // has a number known (a prepared caster's book 0 is its whole class list), or in a book
            // of its own (`Prepared Spells`, a spellbook) -- not the Innate or Class books.
            ["SPELLMEM", sc, book, level, _, "NAME"] => {
                let chosen = match (*book, get(&format!("SPELLBOOK.{book}.NAME")).unwrap_or("")) {
                    ("0", _) => get(&format!("SPELLLISTKNOWN.{sc}.{level}")).and_then(number).unwrap_or(0) > 0,
                    (_, "Innate" | "Class") => false,
                    _ => true,
                };
                if chosen {
                    out.insert(format!("spell.{}", fold(value)), "held".into());
                }
            }
            ["SPELLLISTCLASS", sc] => {
                spell_class.insert((*sc).to_owned(), catalog.class_id(value).map_or_else(|| fold(value), id_tail));
            }
            ["WEAPON", i, "TOTALHIT"] => {
                let name = get(&format!("WEAPON.{i}.NAME")).unwrap_or("").trim_start_matches('*');
                put(&mut out, format!("weapon.{}.attack", gear_key(catalog, &long(name))), number(value));
            }
            _ => {}
        }
    }
    for (key, value) in &kv {
        let parts: Vec<&str> = key.split('.').collect();
        if let ["SPELLLISTCAST", sc, level] = parts.as_slice() {
            let Some(class) = spell_class.get(*sc) else { continue };
            let per_day = number(value).unwrap_or(0);
            if per_day == 0 {
                continue;
            }
            out.insert(format!("spells.{class}.{level}.per_day"), per_day.to_string());
            if let Some(dc) = get(&format!("SPELLLISTDC.{sc}.{level}")).and_then(number) {
                out.insert(format!("spells.{class}.{level}.dc"), dc.to_string());
            }
        }
    }
    out
}

/// A feat as both sides key it: the record the catalog resolves it to; a parameterised feat
/// (`Spell Focus (Enchantment)`) by its stem when the full name is no record.
fn feat_name(catalog: &Catalog, name: &str) -> String {
    let stem = name.split(" (").next().unwrap_or(name);
    catalog.feat_key(name).or_else(|| catalog.feat_key(stem)).map_or_else(|| fold(name), id_tail)
}

/// A gear or weapon name as both sides key it: the record the import's own catalog resolves it to
/// (`Gold Piece` is `coin_gold_piece`), else its own words -- in either case folded and sorted, so a
/// name's word order (`Explorer's Outfit`, `Outfit (Explorer's)`) does not part two names.
fn gear_key(catalog: &Catalog, name: &str) -> String {
    gear_name(catalog.equipment_key(name).map_or(name, |key| key.rsplit(':').next().unwrap_or(key)))
}

fn gear_name(name: &str) -> String {
    let mut words: Vec<String> = name.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_ascii_lowercase).collect();
    words.sort();
    words.join("_")
}

/// Codex's view of one imported file, keyed like [`pcgen_view`].
fn codex_view(file: &ImportFile) -> BTreeMap<String, String> {
    let starfinder = file.report.game_mode == "Starfinder";
    let catalog = if starfinder { catalog().for_game_mode("Starfinder").expect("the Starfinder catalog") } else { catalog() };
    let input = character_input_of(file);
    let chosen = &input.chosen;
    let mut out = BTreeMap::new();
    out.insert("race".into(), id_tail(&chosen.race_id));
    for class in &chosen.class_levels {
        out.insert(format!("class.{}", id_tail(&class.class_id)), class.level.to_string());
    }
    out.insert("money".into(), file.money.total_copper.to_string());
    for feat in &chosen.selected_feats {
        out.insert(format!("feat.{}", id_tail(feat)), "held".into());
    }
    if !starfinder {
        for spell in &chosen.spells_selected {
            out.insert(format!("spell.{}", id_tail(&spell.spell_id)), "held".into());
        }
    }
    // Every stack, at its place: its custom name keyed as PCGen's name is ([`gear_name`]), else the
    // Create item's name, else the record.
    let custom_name = |item_id: &str| item_id.strip_prefix("custom:").and_then(|id| file.custom.equipment.iter().find(|r| r.id == id)).map(|r| gear_name(&r.name));
    let set = file.equipment_sets.active_set();
    let name_of = |key: &CopyKey| {
        let facts = file.equipment_sets.copies.get(&key.map_key());
        facts.and_then(|f| f.name.as_deref().map(|name| gear_key(catalog, name))).or_else(|| custom_name(&key.item_id)).unwrap_or_else(|| gear_name(key.item_id.rsplit(':').next().unwrap_or(&key.item_id)))
    };
    let mut gear: BTreeMap<String, u64> = BTreeMap::new();
    for (map_key, facts) in &file.equipment_sets.copies {
        let mut parts = map_key.rsplitn(3, '|');
        let (ordinal, bonus, item_id) = (parts.next().unwrap_or("0"), parts.next().unwrap_or("0"), parts.next().unwrap_or(""));
        let key = CopyKey { item_id: item_id.to_owned(), bonus: bonus.parse().unwrap_or(0), ordinal: ordinal.parse().unwrap_or(0) };
        let place = match location_of(set, &key) {
            Location::Slot { .. } | Location::Equipped => "equipped".to_owned(),
            Location::Carried => "carried".to_owned(),
            Location::NotCarried => "notcarried".to_owned(),
            Location::Inside { container } => name_of(&container),
        };
        *gear.entry(format!("gear.{}@{place}", name_of(&key))).or_insert(0) += u64::from(facts.quantity.unwrap_or(1));
    }
    out.extend(gear.into_iter().map(|(k, v)| (k, v.to_string())));
    if starfinder { starfinder_totals(&input, &mut out) } else { pathfinder_totals(&input, &mut out) }
    out
}

/// The Starfinder engine's totals (`map::starfinder_verdict`'s pipeline; the app's `compute_sheet`).
fn starfinder_totals(input: &CharacterInput, out: &mut BTreeMap<String, String>) {
    let package = catalog().for_game_mode("Starfinder").unwrap().starfinder_package().unwrap();
    let (build, loadout) = starfinder_build(package, input);
    let chassis = sf_chassis::compute(package, &build.chassis).expect("the chassis computes");
    let held = sf_defense::held(package, &build).expect("held");
    let defense = sf_defense::compute_with(package, &build, &held).expect("defense");
    let skills = sf_skills::compute_with(package, &build, &held).expect("skills");
    let spells = sf_spells::compute_with(package, &build, &held).expect("spells");
    let carried = sf_loadout::compute(package, &build, &loadout).expect("loadout");
    let attacks = sf_attack::compute_with(package, &held, &chassis.base_attack_bonus, &loadout.carried);
    for (name, score) in ["STR", "DEX", "CON", "INT", "WIS", "CHA"].iter().zip(build.chassis.ability_scores) {
        out.insert(format!("stat.{name}"), score.to_string());
    }
    for (key, total) in [
        ("hp", &chassis.hit_points),
        ("stamina", &chassis.stamina),
        ("resolve", &chassis.resolve),
        ("bab", &chassis.base_attack_bonus),
        ("save.Fortitude", &chassis.fortitude),
        ("save.Reflex", &chassis.reflex),
        ("save.Will", &chassis.will),
        ("eac", &defense.eac),
        ("kac", &defense.kac),
        ("initiative", &defense.initiative),
        ("attack.melee", &attacks.melee),
        ("attack.ranged", &attacks.ranged),
    ] {
        out.insert(key.into(), total.total.to_string());
    }
    for skill in &skills.skills {
        out.insert(format!("skill.{}", id_tail(&skill.skill)), skill.total.as_ref().map_or_else(|| "untrained".into(), |t| t.total.to_string()));
    }
    for casting in &spells {
        for level in &casting.levels {
            for (field, total) in [("per_day", &level.per_day), ("known", &level.known)] {
                if let Some(t) = total {
                    out.insert(format!("spells.{}.{}.{field}", id_tail(&casting.class), level.level), t.total.to_string());
                }
            }
        }
    }
    for weapon in &attacks.weapons {
        out.insert(format!("weapon.{}.attack", gear_name(weapon.item.rsplit(':').next().unwrap_or(&weapon.item))), weapon.attack.total.to_string());
    }
    let _ = carried;
}

/// A Pathfinder character's ability scores are the import's (the score after the race, as Create
/// saves it); every Pathfinder total is the desktop's (module doc) and reads [`NOT_COMPUTED`].
fn pathfinder_totals(input: &CharacterInput, out: &mut BTreeMap<String, String>) {
    let s = &input.chosen.ability_scores;
    for (name, score) in [("STR", s.strength), ("DEX", s.dexterity), ("CON", s.constitution), ("INT", s.intelligence), ("WIS", s.wisdom), ("CHA", s.charisma)] {
        out.insert(format!("stat.{name}"), score.to_string());
    }
}

/// One `explained.tsv` row.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Explained {
    build: String,
    field: String,
    pcgen: String,
    codex: String,
    why: String,
    decision: String,
}

impl Explained {
    fn covers(&self, build: &str, field: &str, pcgen: &str, codex: &str) -> bool {
        let cell = |row: &str, value: &str| row == "*" || row == value;
        self.build == build
            && match self.field.strip_suffix('*') {
                Some(prefix) => field.starts_with(prefix),
                None => self.field == field,
            }
            && cell(&self.pcgen, pcgen)
            && cell(&self.codex, codex)
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
            let row = Explained { build: c[0].into(), field: c[1].into(), pcgen: c[2].into(), codex: c[3].into(), why: c[4].into(), decision: c[5].into() };
            assert!(WHYS.contains(&row.why.as_str()), "{}: why is one of {WHYS:?}: {row:?}", path.display());
            assert!(!row.decision.trim().is_empty(), "{}: a decision: {row:?}", path.display());
            row
        })
        .collect()
}

/// One build compared: `(field, pcgen, codex)` over the union of both views.
fn compare(pcgen: &BTreeMap<String, String>, codex: &BTreeMap<String, String>) -> Vec<(String, String, String)> {
    let fields: BTreeSet<&String> = pcgen.keys().chain(codex.keys()).collect();
    fields
        .into_iter()
        .map(|f| (f.clone(), pcgen.get(f).cloned().unwrap_or_else(|| "absent".into()), codex.get(f).cloned().unwrap_or_else(|| "absent".into())))
        // PCGen's export leaves out a trained-only skill with no ranks (`VIEW=VISIBLE_EXPORT`); Codex
        // prints it untrained. The two say the same thing.
        .filter(|(f, p, c)| !(f.starts_with("skill.") && p == "absent" && c == "untrained"))
        .collect()
}

/// The file's character name, as its export must carry it.
fn character_name(pcg: &str) -> String {
    parse(pcg).iter().find(|line| line.key() == "CHARACTERNAME").map(|line| line.value().trim().to_owned()).unwrap_or_default()
}

/// Every build's difference set, checked against its ledger: returns `(fields, equal, explained)`
/// or the failures. Writes both views to `PCG_PARITY_OUT/<build>.json` when it is set (`compare.py`).
fn gate(build: &str, pcg: &str, export: &str, rows: &[Explained]) -> Result<(usize, usize, usize), Vec<String>> {
    let starfinder = pcg.lines().any(|line| line == "GAMEMODE:Starfinder");
    let name_key = if starfinder { "name=" } else { "NAME=" };
    let exported = export.lines().find_map(|line| line.strip_prefix(name_key)).unwrap_or("");
    assert_eq!(exported.trim(), character_name(pcg), "{build}: the export is of another character; re-run run.sh");
    let mut failures = Vec::new();
    let compared = match convert(pcg, &format!("{build}.pcg"), catalog()) {
        Ok(file) => {
            let pcgen = pcgen_view(export, starfinder);
            let mut codex = codex_view(&file);
            if !starfinder {
                // Every Pathfinder total is the desktop's (module doc).
                for field in pcgen.keys().filter(|f| PF_DESKTOP_TOTALS.iter().any(|p| f.as_str() == *p || (p.ends_with('.') && f.starts_with(p)))) {
                    codex.entry(field.clone()).or_insert_with(|| NOT_COMPUTED.into());
                }
            }
            compare(&pcgen, &codex)
        }
        // A refused file compares one field: whether it imports.
        Err(reason) => vec![("import".to_owned(), "computes".to_owned(), format!("refused: {}", reason.split(':').next().unwrap_or("")))],
    };
    if let Ok(dir) = std::env::var("PCG_PARITY_OUT") {
        std::fs::create_dir_all(&dir).expect("dump dir");
        let rows: Vec<[&str; 3]> = compared.iter().map(|(f, p, c)| [f.as_str(), p.as_str(), c.as_str()]).collect();
        std::fs::write(Path::new(&dir).join(format!("{build}.json")), serde_json::to_string_pretty(&rows).unwrap() + "\n").expect("writes");
    }
    let mine: Vec<&Explained> = rows.iter().filter(|row| row.build == build).collect();
    let mut used = vec![false; mine.len()];
    let (mut equal, mut explained) = (0, 0);
    for (field, pcgen, codex) in &compared {
        if pcgen == codex {
            equal += 1;
            continue;
        }
        match mine.iter().position(|row| row.covers(build, field, pcgen, codex)) {
            Some(at) => {
                used[at] = true;
                explained += 1;
            }
            None => failures.push(format!("{build}\t{field}\t{pcgen}\t{codex}\t(unexplained)")),
        }
    }
    for (row, used) in mine.iter().zip(used) {
        if !used {
            failures.push(format!("stale ledger row (explains no difference): {row:?}"));
        }
    }
    if failures.is_empty() { Ok((compared.len(), equal, explained)) } else { Err(failures) }
}

#[test]
fn pcg_import_parity_every_difference_is_equal_or_explained() {
    let harness = repo_root().join(HARNESS);
    let rows = ledger(&harness.join("explained.tsv"));
    assert!(!rows.is_empty(), "{HARNESS}/explained.tsv holds the ledger");
    let mut failures = Vec::new();
    let mut checked = 0;
    for sample in SAMPLES {
        let path = oracle_dir().join("code/testsuite/PCGfiles").join(format!("{sample}.pcg"));
        let Ok(bytes) = std::fs::read(&path) else {
            println!("SKIPPED: the oracle's {sample}.pcg is absent at {} (decisions.md §22, P-9)", path.display());
            continue;
        };
        let export_path = harness.join("exports").join(format!("{sample}.txt"));
        let export = std::fs::read_to_string(&export_path).unwrap_or_else(|e| panic!("{}: {e}; run run.sh", export_path.display()));
        match gate(sample, &String::from_utf8_lossy(&bytes), &export, &rows) {
            Ok((fields, equal, explained)) => println!("{sample}: fields compared {fields}, equal {equal}, explained {explained}"),
            Err(f) => failures.extend(f),
        }
        checked += 1;
    }
    // Every committed row names a sample this run could read, or it is stale by construction.
    let unknown: Vec<_> = rows.iter().filter(|row| !SAMPLES.contains(&row.build.as_str())).collect();
    assert!(unknown.is_empty(), "ledger rows for no sample: {unknown:?}");
    operator_file(&mut failures);
    println!("pcg_import_parity: {checked} of {} samples compared", SAMPLES.len());
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}

/// The operator's file (`decisions.md §10`): its export and its ledger stay under
/// `$HOME/sd42-input/parity/`; the build is called `operator` so no name is printed.
fn operator_file(failures: &mut Vec<String>) {
    // `$CODEX_SD42_PCG`, else the one `.pcg` in `$HOME/sd42-input/` (`pcg_import_diagnosis.rs`): no
    // name is written here, so the repo never carries it.
    let input = PathBuf::from(std::env::var("HOME").expect("HOME")).join("sd42-input");
    let pcg = match std::env::var("CODEX_SD42_PCG") {
        Ok(path) if !path.is_empty() => PathBuf::from(path),
        _ => std::fs::read_dir(&input)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .find(|path| path.extension().is_some_and(|ext| ext == "pcg"))
            .unwrap_or_else(|| input.join("<operator>.pcg")),
    };
    let Ok(bytes) = std::fs::read(&pcg) else {
        println!("SKIPPED: operator .pcg absent at {} (decisions.md §10)", pcg.display());
        return;
    };
    let stem = pcg.file_stem().unwrap().to_string_lossy().into_owned();
    let dir = input.join("parity");
    let Ok(export) = std::fs::read_to_string(dir.join(format!("{stem}.txt"))) else {
        println!("SKIPPED: the operator's export is absent under {} (run.sh --operator; decisions.md §10)", dir.display());
        return;
    };
    let rows: Vec<Explained> = ledger(&dir.join("explained.tsv")).into_iter().filter(|row| row.build == "operator").collect();
    match gate("operator", &String::from_utf8_lossy(&bytes), &export, &rows) {
        Ok((fields, equal, explained)) => println!("operator: fields compared {fields}, equal {equal}, explained {explained}"),
        Err(f) => failures.extend(f),
    }
}
