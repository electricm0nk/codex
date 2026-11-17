//! What Codex holds, read from the same tables and converted records the app reads. A name the
//! catalog does not hold becomes a custom record, so this is the line between "mapped" and "custom".

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use codex::rules_core::class_census::census;
use codex::rules_core::corpus_loader::{BookCorpusRoot, load_sheet_rules};
use codex::rules_core::game_system::GameSystem;
use codex::rules_core::pilot_compute::sf_loadout::is_buyable;
use codex::rules_core::equipment_resolver::equipment_catalog_rows;
use codex::rules_core::race_creation::race_creation_chassis;
use codex::rules_core::race_resolver::load_race_corpus;
use codex::rules_core::rules_catalog::feats_all::all_feat_tables;
use codex::rules_core::sheet_rule::{SheetRulePackage, split_rule_id};

/// One race's ability numbers, as creation applies them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaceEntry {
    pub id: String,
    pub adjustments: BTreeMap<String, i16>,
    pub floating_bonus_points: u8,
}

#[derive(Debug, Default)]
pub struct Catalog {
    classes: BTreeMap<String, String>,
    races: BTreeMap<String, RaceEntry>,
    feats: BTreeMap<String, String>,
    equipment: BTreeMap<String, String>,
    spells: BTreeSet<String>,
    /// A skill's slug -> the id a skill allocation names (`skill:climb`; Starfinder's bare `climb`).
    skills: BTreeMap<String, String>,
    /// Starfinder's theme records (`core:ability:ace_pilot`, package pool `theme`).
    themes: BTreeMap<String, String>,
    /// The Starfinder package this catalog was read from; `None` for Pathfinder's.
    starfinder_package: Option<SheetRulePackage>,
    /// Where `data/` is, so a Starfinder file can read its own catalog ([`Catalog::for_game_mode`]).
    data_root: PathBuf,
    starfinder: OnceLock<Result<Box<Catalog>, String>>,
}

/// Case and punctuation are not part of a name's identity: `Rope (Silk/50 ft.)` is
/// `Rope (Silk/50 ft)`, and `Half-Elf` is `Half Elf`.
pub fn fold(name: &str) -> String {
    name.chars().filter(char::is_ascii_alphanumeric).map(|ch| ch.to_ascii_lowercase()).collect()
}

/// The slug a converted record's file is named by: `Knowledge (Religion)` is `knowledge_religion`.
pub fn slug(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_end_matches('_').to_owned()
}

impl Catalog {
    /// Reads the catalog under `data_root`, the directory holding `data/`. A root with no
    /// converted records is an error: every name would otherwise read as unknown.
    pub fn load(data_root: &Path) -> Result<Catalog, String> {
        let sheet_rules = data_root.join("data/sheet_rules");
        let books = sub_directories(&sheet_rules);
        if books.is_empty() {
            return Err(format!("no converted rules under {}", sheet_rules.display()));
        }

        let mut catalog = Catalog { data_root: data_root.to_owned(), ..Catalog::default() };
        for book in &books {
            let book_dir = sheet_rules.join(book);
            catalog.spells.extend(record_stems(&book_dir.join("spell")));
            catalog.skills.extend(record_stems(&book_dir.join("skill")).into_iter().map(|stem| (stem.clone(), format!("skill:{stem}"))));
        }
        // The engine's own merged class census, so a class here is a class the app can compute.
        for class_id in census().into_keys() {
            let slug = class_id.strip_prefix("class:").unwrap_or(&class_id).to_owned();
            catalog.classes.insert(fold(&slug), class_id);
        }
        for table in all_feat_tables() {
            for feat in table.entries {
                catalog.feats.entry(fold(feat.key)).or_insert_with(|| feat.key.to_owned());
            }
        }
        // Keys first: `Full Plate (Base)` is also named `Full Plate`, and a key is the identity.
        for row in equipment_catalog_rows() {
            catalog.equipment.entry(fold(row.key)).or_insert_with(|| row.key.to_owned());
        }
        for row in equipment_catalog_rows() {
            catalog.equipment.entry(fold(row.name)).or_insert_with(|| row.key.to_owned());
        }
        catalog.load_races(&data_root.join("data/corpus"));
        Ok(catalog)
    }

    /// The Starfinder catalog: the records of the converted Starfinder package under `data_root`
    /// (`data/starfinder-1e/sheet_rules`), read by the same loader the app's Starfinder adapter
    /// reads it with (`corpus_loader::load_sheet_rules`). A name is a record's slug or its label;
    /// a core record wins a name two books share. Races carry no adjustments here: the Starfinder
    /// engine folds the race's own ability rows (`sf_abilities`).
    pub fn load_sf(data_root: &Path) -> Result<Catalog, String> {
        let dir = GameSystem::Starfinder1e.package_roots(data_root).sheet_rules.ok_or("Starfinder has no sheet-rule package")?;
        let package = load_sheet_rules(&dir).package;
        if package.rules.is_empty() {
            return Err(format!("no converted Starfinder rules under {}", dir.display()));
        }
        let mut catalog = Catalog { data_root: data_root.to_owned(), ..Catalog::default() };
        for (id, rule) in package.rules.iter().filter(|(id, _)| !id.contains('#')) {
            let (book, kind, slug) = split_rule_id(id);
            let names = [fold(slug), fold(&rule.label)];
            let map = match kind {
                "class" => &mut catalog.classes,
                "feat" => &mut catalog.feats,
                // A record the engine cannot carry (no price: the configurable Computer) is no
                // record to hold; its line becomes a Create item (`decisions.md §7`).
                "equipment" if is_buyable(rule) => &mut catalog.equipment,
                "ability" if rule.pool == "theme" => &mut catalog.themes,
                "race" => {
                    let entry = RaceEntry { id: id.clone(), adjustments: BTreeMap::new(), floating_bonus_points: 0 };
                    for name in names {
                        if book == "core" || !catalog.races.contains_key(&name) {
                            catalog.races.insert(name, entry.clone());
                        }
                    }
                    continue;
                }
                "skill" => {
                    catalog.skills.entry(slug.to_owned()).or_insert_with(|| slug.to_owned());
                    continue;
                }
                _ => continue,
            };
            for name in names {
                if book == "core" || !map.contains_key(&name) {
                    map.insert(name, id.clone());
                }
            }
        }
        catalog.starfinder_package = Some(package);
        Ok(catalog)
    }

    /// The catalog a file of PCGen's `game_mode` reads: Starfinder's for `Starfinder`, read once
    /// from this catalog's `data_root`; this one for every other mode.
    pub fn for_game_mode(&self, game_mode: &str) -> Result<&Catalog, String> {
        if game_mode != "Starfinder" || self.starfinder_package.is_some() {
            return Ok(self);
        }
        self.starfinder.get_or_init(|| Catalog::load_sf(&self.data_root).map(Box::new)).as_deref().map_err(String::clone)
    }

    /// The Starfinder package, when this is the Starfinder catalog.
    pub fn starfinder_package(&self) -> Option<&SheetRulePackage> {
        self.starfinder_package.as_ref()
    }

    pub fn theme_id(&self, name: &str) -> Option<&str> {
        self.themes.get(&fold(name)).map(String::as_str)
    }

    fn load_races(&mut self, corpus_root: &Path) {
        let books = sub_directories(corpus_root);
        let dirs: Vec<_> = books.iter().map(|book| corpus_root.join(book)).collect();
        let roots: Vec<BookCorpusRoot<'_>> = books
            .iter()
            .zip(dirs.iter())
            .map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() })
            .collect();
        let corpus = load_race_corpus(&roots);
        for key in corpus.race_keys() {
            let Some(resolved) = corpus.resolve(key, &[]) else { continue };
            let Ok(chassis) = race_creation_chassis(&resolved) else { continue };
            self.races.insert(
                fold(key),
                RaceEntry {
                    id: format!("race:{}", chassis.race_key.to_lowercase()),
                    adjustments: chassis.ability_adjustments,
                    floating_bonus_points: chassis.floating_bonus_points,
                },
            );
        }
    }

    pub fn class_id(&self, name: &str) -> Option<&str> {
        self.classes.get(&fold(name)).map(String::as_str)
    }

    pub fn race(&self, name: &str) -> Option<&RaceEntry> {
        self.races.get(&fold(name))
    }

    pub fn feat_key(&self, name: &str) -> Option<&str> {
        self.feats.get(&fold(name)).map(String::as_str)
    }

    pub fn equipment_key(&self, name: &str) -> Option<&str> {
        self.equipment.get(&fold(name)).map(String::as_str)
    }

    pub fn holds_spell(&self, name: &str) -> bool {
        self.spells.contains(&slug(name))
    }

    pub fn skill_id(&self, name: &str) -> Option<String> {
        self.skills.get(&slug(name)).cloned()
    }
}

fn sub_directories(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| !name.starts_with('_') && name != "GENERATED")
        .collect();
    names.sort();
    names
}

fn record_stems(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension()? == "json").then(|| path.file_stem()?.to_str().map(str::to_owned))?
        })
        .collect()
}
