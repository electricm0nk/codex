//! What Codex holds, read from the same tables and converted records the app reads. A name the
//! catalog does not hold becomes a custom record, so this is the line between "mapped" and "custom".

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use codex::rules_core::class_census::census;
use codex::rules_core::corpus_loader::BookCorpusRoot;
use codex::rules_core::equipment_resolver::equipment_catalog_rows;
use codex::rules_core::race_creation::race_creation_chassis;
use codex::rules_core::race_resolver::load_race_corpus;
use codex::rules_core::rules_catalog::feats_all::all_feat_tables;

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
    skills: BTreeSet<String>,
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

        let mut catalog = Catalog::default();
        for book in &books {
            let book_dir = sheet_rules.join(book);
            catalog.spells.extend(record_stems(&book_dir.join("spell")));
            catalog.skills.extend(record_stems(&book_dir.join("skill")));
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
        let slug = slug(name);
        self.skills.contains(&slug).then(|| format!("skill:{slug}"))
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
