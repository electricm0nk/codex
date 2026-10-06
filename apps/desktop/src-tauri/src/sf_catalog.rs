//! Starfinder 1e catalogs (SD-37 E6.4, `docs/release/SD-37-starfinder-1e/epic-breakdown.md`
//! Epic E6): the races, themes, classes, feats, spells and equipment of the converted
//! Starfinder package (`data/starfinder-1e/sheet_rules`), served to the landing screen's
//! "Browse" links when Starfinder 1e is the selected rule set.
//!
//! # What each catalog lists
//!
//! | Catalog | Records | Name |
//! |---|---|---|
//! | races | every `race` record | the bare race name its `race`-pool ability carries (the race record's own label names its first ability row, `Ysoki (Str)`), else the record's label |
//! | themes | every `theme`-pool ability tagged `Theme Selection` (the set the creation flow offers) | the record's label |
//! | classes | every `class` record | the record's label |
//! | feats, spells, equipment | every record of that kind | the record's label |
//!
//! "Record" is a principal rule (no `#` in its id); its `#` siblings are rows of the same record.
//! A row's description is the engine's catalog prose for the record
//! ([`codex::rules_core::sheet_rule_catalog::catalog_description_or_fields`]): the record's own
//! words, else its stat block, else its typed fields as words -- with no character in hand, so a
//! formula over a character's level prints as words, never as a number.
//!
//! The package is the one the Starfinder adapter reads ([`crate::sf_adapter`]). Nothing here
//! reads the Pathfinder tables: the Pathfinder catalogs (`race_catalog.rs`, `spell_catalog.rs`,
//! ...) serve Pathfinder 1e only.

use serde::{Deserialize, Serialize};

use codex::rules_core::sheet_rule::{SheetRule, SheetRulePackage};
use codex::rules_core::sheet_rule_catalog::{catalog_description_or_fields, catalog_field_summary, DescriptionTier};

use crate::sf_adapter;
use crate::sf_sheet_print::HIT_DIE_ROW;

/// One Starfinder catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SfCatalogKind {
    Race,
    Theme,
    Class,
    Feat,
    Spell,
    Equipment,
}

impl SfCatalogKind {
    pub const ALL: [SfCatalogKind; 6] = [
        SfCatalogKind::Race,
        SfCatalogKind::Theme,
        SfCatalogKind::Class,
        SfCatalogKind::Feat,
        SfCatalogKind::Spell,
        SfCatalogKind::Equipment,
    ];
}

/// One catalog row: a converted Starfinder record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfCatalogEntryDto {
    /// The package rule id (`core:feat:adaptive_fighting`).
    pub id: String,
    pub name: String,
    /// The package's book id (`core`, `armory`, ...): the record's provenance.
    pub book: String,
    pub tags: Vec<String>,
    /// The engine's catalog prose for the record; `None` when the record states nothing.
    pub description: Option<String>,
    /// Which tier answered the description: `prose`, `statBlock` or `fields`.
    pub description_tier: Option<String>,
    /// The record's own rows (the record and its `#` siblings) that feed a sheet total, each in
    /// the engine's words: a race's ability adjustments and racial Hit Points, a class's Hit
    /// Points, Stamina and saves per level, an armour's EAC and KAC.
    pub rows: Vec<SfCatalogRowDto>,
}

/// One row of a record that feeds a sheet total.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfCatalogRowDto {
    pub id: String,
    pub label: String,
    /// [`catalog_field_summary`]: the row's value, the total it adds to and its condition, as words.
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SfCatalogResponse {
    pub kind: SfCatalogKind,
    /// The package root the rows were read from (repo-relative).
    pub source: String,
    pub entries: Vec<SfCatalogEntryDto>,
}

/// The tag the converter carries on every selectable theme (the set the creation flow offers).
const THEME_SELECTION_TAG: &str = "Theme Selection";

fn is_record(rule: &SheetRule) -> bool {
    !rule.id.contains('#')
}

fn records(package: &SheetRulePackage, kind: SfCatalogKind) -> Vec<&SheetRule> {
    match kind {
        SfCatalogKind::Theme => package
            .rules_of_kind("ability")
            .filter(|r| is_record(r) && r.pool == "theme" && r.tags.iter().any(|t| t == THEME_SELECTION_TAG))
            .collect(),
        SfCatalogKind::Race => package.rules_of_kind("race").filter(|r| is_record(r)).collect(),
        SfCatalogKind::Class => package.rules_of_kind("class").filter(|r| is_record(r)).collect(),
        SfCatalogKind::Feat => package.rules_of_kind("feat").filter(|r| is_record(r)).collect(),
        SfCatalogKind::Spell => package.rules_of_kind("spell").filter(|r| is_record(r)).collect(),
        SfCatalogKind::Equipment => package.rules_of_kind("equipment").filter(|r| is_record(r)).collect(),
    }
}

/// A race's bare name: the `race`-pool ability the race grants carries it; the race record's
/// own label names its first ability row (`Ysoki (Str)`).
fn race_name(package: &SheetRulePackage, race: &SheetRule) -> String {
    package
        .granted_from(&race.id)
        .iter()
        .filter_map(|id| package.rule(id))
        .find(|r| r.pool == "race" && is_record(r))
        .map_or_else(|| race.label.clone(), |r| r.label.clone())
}

/// The record's rows that feed a sheet total, in package order (the record, then its siblings).
fn total_rows(package: &SheetRulePackage, record: &SheetRule) -> Vec<SfCatalogRowDto> {
    let prefix = format!("{}#", record.id);
    std::iter::once(record)
        .chain(package.rules.range(prefix.clone()..).take_while(|(k, _)| k.starts_with(&prefix)).map(|(_, r)| r))
        .filter(|r| r.target.is_some())
        .filter_map(|r| {
            catalog_field_summary(package, r).map(|summary| SfCatalogRowDto { id: r.id.clone(), label: r.label.clone(), summary })
        })
        .collect()
}

/// A Starfinder class has no hit die: the class record's `Hit die: d1` stat-block row is
/// PCGen's `HD:1` device, not printed on the sheet ([`HIT_DIE_ROW`]) and not in a catalog.
fn without_hit_die(text: &str) -> String {
    text.split('\n').filter(|row| !row.starts_with(HIT_DIE_ROW)).collect::<Vec<_>>().join("\n")
}

fn tier_name(tier: DescriptionTier) -> &'static str {
    match tier {
        DescriptionTier::Prose => "prose",
        DescriptionTier::StatBlock => "statBlock",
        DescriptionTier::Fields => "fields",
    }
}

/// The rows of one Starfinder catalog, in name order (then id).
pub fn entries(package: &SheetRulePackage, kind: SfCatalogKind) -> Vec<SfCatalogEntryDto> {
    let mut out: Vec<SfCatalogEntryDto> = records(package, kind)
        .into_iter()
        .map(|rule| {
            let description = catalog_description_or_fields(package, rule)
                .map(|d| (without_hit_die(&d.text), tier_name(d.tier).to_owned()))
                .filter(|(text, _)| !text.is_empty());
            let (description, description_tier) = description.unzip();
            SfCatalogEntryDto {
                id: rule.id.clone(),
                name: if kind == SfCatalogKind::Race { race_name(package, rule) } else { rule.label.clone() },
                book: rule.provenance.book.clone(),
                tags: rule.tags.clone(),
                description,
                description_tier,
                rows: total_rows(package, rule),
            }
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
    out
}

/// One Starfinder catalog, read from the package the Starfinder adapter reads.
#[tauri::command]
pub fn list_starfinder_catalog(kind: SfCatalogKind) -> Result<SfCatalogResponse, String> {
    let package = sf_adapter::package().map_err(|r| format!("{}: {}", r.id, r.message))?;
    Ok(SfCatalogResponse {
        kind,
        source: codex::rules_core::game_system::GameSystem::Starfinder1e.sheet_rules_relative().to_owned(),
        entries: entries(package, kind),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    use codex::rules_core::game_system::GameSystem;

    fn package() -> &'static SheetRulePackage {
        sf_adapter::package().expect("the Starfinder package loads")
    }

    fn sheet_rules_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(GameSystem::Starfinder1e.sheet_rules_relative())
    }

    fn kind_dir(kind: SfCatalogKind) -> &'static str {
        match kind {
            SfCatalogKind::Race => "race",
            SfCatalogKind::Theme => "ability",
            SfCatalogKind::Class => "class",
            SfCatalogKind::Feat => "feat",
            SfCatalogKind::Spell => "spell",
            SfCatalogKind::Equipment => "equipment",
        }
    }

    /// A second implementation of "which records a catalog lists": the record files under
    /// `data/starfinder-1e/sheet_rules/<book>/<kind>/*.json` read as plain JSON, not through the
    /// package loader. Returns `id -> book`.
    fn records_on_disk(kind: SfCatalogKind) -> BTreeMap<String, String> {
        let mut out = BTreeMap::new();
        let root = sheet_rules_root();
        for book in std::fs::read_dir(&root).expect("sheet_rules root").flatten() {
            let dir = book.path().join(kind_dir(kind));
            let Ok(files) = std::fs::read_dir(&dir) else { continue };
            for file in files.flatten() {
                let text = std::fs::read_to_string(file.path()).expect("record file");
                let rows: Vec<serde_json::Value> = serde_json::from_str(&text).expect("record JSON");
                for row in rows {
                    let id = row["id"].as_str().expect("id").to_owned();
                    if id.contains('#') {
                        continue;
                    }
                    if kind == SfCatalogKind::Theme {
                        let theme = row["pool"].as_str() == Some("theme")
                            && row["tags"].as_array().is_some_and(|t| t.iter().any(|t| t.as_str() == Some("Theme Selection")));
                        if !theme {
                            continue;
                        }
                    }
                    out.insert(id, row["provenance"]["book"].as_str().expect("book").to_owned());
                }
            }
        }
        out
    }

    /// Every catalog lists exactly the Starfinder records of its kind -- the same id set the
    /// record files hold on disk, each once, each with its book -- and nothing else.
    #[test]
    fn every_starfinder_catalog_lists_the_records_of_its_kind_from_the_starfinder_package() {
        let mut counts = Vec::new();
        for kind in SfCatalogKind::ALL {
            let rows = entries(package(), kind);
            let listed: BTreeMap<String, String> = rows.iter().map(|r| (r.id.clone(), r.book.clone())).collect();
            assert_eq!(listed.len(), rows.len(), "{kind:?}: each record listed once");
            let on_disk = records_on_disk(kind);
            assert!(!on_disk.is_empty(), "{kind:?}: the package holds records of this kind");
            assert_eq!(listed, on_disk, "{kind:?}: the catalog lists exactly the package's records");
            for row in &rows {
                assert!(!row.name.trim().is_empty(), "{}: a name", row.id);
                assert!(package().rule(&row.id).is_some(), "{}: a Starfinder package record", row.id);
            }
            let mut sorted = rows.clone();
            sorted.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
            assert_eq!(rows, sorted, "{kind:?}: rows in name order");
            counts.push(format!("{kind:?}={}", rows.len()));
        }
        println!("sf-catalog counts: {}", counts.join(" "));
    }

    /// Named Starfinder records from the Core Rulebook appear under their real names; no
    /// Pathfinder record does.
    #[test]
    fn the_catalogs_carry_core_rulebook_records_by_name_and_no_pathfinder_record() {
        let name_of = |kind: SfCatalogKind, id: &str| -> String {
            entries(package(), kind).into_iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{id} listed")).name
        };
        assert_eq!(name_of(SfCatalogKind::Race, "core:race:ysoki"), "Ysoki");
        assert_eq!(name_of(SfCatalogKind::Race, "core:race:lashunta"), "Lashunta");
        assert_eq!(name_of(SfCatalogKind::Theme, "core:ability:mercenary"), "Mercenary");
        assert_eq!(name_of(SfCatalogKind::Class, "core:class:soldier"), "Soldier");
        assert_eq!(name_of(SfCatalogKind::Class, "core:class:technomancer"), "Technomancer");
        assert_eq!(name_of(SfCatalogKind::Feat, "core:feat:adaptive_fighting"), "Adaptive Fighting");
        let spells = entries(package(), SfCatalogKind::Spell);
        assert!(spells.iter().any(|r| r.name == "Magic Missile" && r.book == "core"), "Magic Missile listed");
        for kind in SfCatalogKind::ALL {
            for row in entries(package(), kind) {
                assert!(!row.book.is_empty() && row.book != "core_rulebook", "{}: a Starfinder book, got {}", row.id, row.book);
            }
        }
        // A Pathfinder-only record is absent: there is no Pathfinder "Fighter" class here.
        assert!(entries(package(), SfCatalogKind::Class).iter().all(|r| r.name != "Fighter"));
    }

    /// A row's description is the engine's catalog prose for that record, with no character:
    /// a formula over the caster's level reads as words.
    #[test]
    fn a_rows_description_is_the_engines_catalog_prose() {
        for kind in SfCatalogKind::ALL {
            for row in entries(package(), kind) {
                let rule = package().rule(&row.id).expect("record");
                let expected = catalog_description_or_fields(package(), rule).map(|d| {
                    d.text.split('\n').filter(|l| !l.starts_with(HIT_DIE_ROW)).collect::<Vec<_>>().join("\n")
                });
                assert_eq!(row.description, expected.filter(|t| !t.is_empty()), "{}", row.id);
                assert_eq!(row.description.is_some(), row.description_tier.is_some(), "{}: a tier only with a description", row.id);
            }
        }
        let feat = entries(package(), SfCatalogKind::Feat).into_iter().find(|r| r.id == "core:feat:adaptive_fighting").unwrap();
        assert!(feat.description.as_deref().is_some_and(|d| !d.is_empty()), "a feat carries its benefit text");
        assert_eq!(feat.description_tier.as_deref(), Some("prose"));
    }

    /// A Starfinder class has no hit die (SRD: Hit Points = racial HP + class HP per level);
    /// the class record's `Hit die: d1` stat-block row is PCGen's `HD:1` device and is never
    /// printed -- the sheet's rule (`sf_sheet_print::HIT_DIE_ROW`), held in the catalog too.
    #[test]
    fn no_starfinder_catalog_prints_a_hit_die() {
        for kind in SfCatalogKind::ALL {
            for row in entries(package(), kind) {
                let text = [row.description.clone().unwrap_or_default()]
                    .into_iter()
                    .chain(row.rows.iter().map(|r| format!("{}\n{}", r.label, r.summary)))
                    .collect::<Vec<_>>()
                    .join("\n");
                assert!(!text.lines().any(|l| l.starts_with(HIT_DIE_ROW)), "{}: {text}", row.id);
            }
        }
    }

    /// Each row lists the record's own rows that feed a sheet total -- a race's ability
    /// adjustments and racial Hit Points, a class's Hit Points, Stamina and saves per level, an
    /// armour's EAC and KAC -- in the engine's words, one per row id.
    #[test]
    fn a_rows_totals_are_the_records_own_rows_that_feed_a_sheet_total() {
        let find = |kind: SfCatalogKind, id: &str| entries(package(), kind).into_iter().find(|r| r.id == id).expect(id);
        let ysoki = find(SfCatalogKind::Race, "core:race:ysoki");
        let ids: Vec<&str> = ysoki.rows.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["core:race:ysoki", "core:race:ysoki#bonus1", "core:race:ysoki#bonus2", "core:race:ysoki#race_hp"]);
        let hp = ysoki.rows.iter().find(|r| r.id == "core:race:ysoki#race_hp").unwrap();
        assert_eq!(hp.label, "Ysoki (racial Hit Points)");
        assert!(hp.summary.starts_with("Value: 2; Adds to hit points"), "{}", hp.summary);
        let soldier = find(SfCatalogKind::Class, "core:class:soldier");
        let stamina = soldier.rows.iter().find(|r| r.id == "core:class:soldier#bonus2").expect("stamina row");
        assert!(stamina.summary.starts_with("Value: 7 times soldier level; Adds to Stamina Points"), "{}", stamina.summary);
        let second_skin = find(SfCatalogKind::Equipment, "core:equipment:second_skin");
        let kac = second_skin.rows.iter().find(|r| r.id == "core:equipment:second_skin#bonus1").expect("KAC row");
        assert!(kac.summary.starts_with("Value: 2; Adds to Kinetic Armor Class"), "{}", kac.summary);
        // The rows are exactly the record's rows with a target, each summarised by the engine.
        for kind in SfCatalogKind::ALL {
            for row in entries(package(), kind) {
                let prefix = format!("{}#", row.id);
                let expected: Vec<String> = std::iter::once(package().rule(&row.id).unwrap())
                    .chain(package().rules.range(prefix.clone()..).take_while(|(k, _)| k.starts_with(&prefix)).map(|(_, r)| r))
                    .filter(|r| r.target.is_some())
                    .map(|r| r.id.clone())
                    .collect();
                assert_eq!(row.rows.iter().map(|r| r.id.clone()).collect::<Vec<_>>(), expected, "{}", row.id);
                for total in &row.rows {
                    let rule = package().rule(&total.id).unwrap();
                    assert_eq!(Some(total.summary.clone()), catalog_field_summary(package(), rule), "{}", total.id);
                }
            }
        }
    }

    /// No Starfinder catalog imports `rules_tables`: the module's code (comments removed)
    /// names no path into the Pathfinder tables.
    #[test]
    fn no_starfinder_catalog_imports_the_pathfinder_tables() {
        let needle = ["rules", "_tables"].concat();
        let source = include_str!("sf_catalog.rs");
        let code: String = source
            .split("#[cfg(test)]")
            .next()
            .expect("module code")
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(code.contains("use codex::rules_core::sheet_rule"), "the scan reads the module's real imports");
        assert!(!code.contains(&needle), "sf_catalog.rs imports {needle}");
    }

    /// The frontend catalog test (`starfinderCatalog.test.ts`) renders the race and class
    /// catalogs' real responses. This keeps those files equal to what the command returns
    /// today. Regenerate: `SF_CATALOG_FIXTURES_WRITE=1 cargo test --bin codex-desktop
    /// the_frontend_catalog_fixtures_are_the_commands_responses`.
    #[test]
    fn the_frontend_catalog_fixtures_are_the_commands_responses() {
        let write = std::env::var_os("SF_CATALOG_FIXTURES_WRITE").is_some();
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/starfinderCatalog/starfinderCatalogFixtures");
        let mut stale = Vec::new();
        for (kind, name) in [(SfCatalogKind::Race, "race"), (SfCatalogKind::Class, "class")] {
            let json = serde_json::to_string_pretty(&list_starfinder_catalog(kind).expect("catalog")).expect("serialises") + "\n";
            let path = dir.join(format!("{name}.json"));
            if write {
                std::fs::create_dir_all(&dir).expect("fixture dir");
                std::fs::write(&path, &json).expect("writes the fixture");
            } else if std::fs::read_to_string(&path).ok().as_deref() != Some(json.as_str()) {
                stale.push(path.display().to_string());
            }
        }
        assert!(
            stale.is_empty(),
            "stale or missing Starfinder catalog fixtures {stale:?}; regenerate with SF_CATALOG_FIXTURES_WRITE=1 cargo test --bin codex-desktop the_frontend_catalog_fixtures_are_the_commands_responses"
        );
    }

    /// The command answers each catalog from the package the Starfinder adapter reads.
    #[test]
    fn the_command_serves_each_catalog() {
        for kind in SfCatalogKind::ALL {
            let response = list_starfinder_catalog(kind).expect("catalog");
            assert_eq!(response.kind, kind);
            assert_eq!(response.source, GameSystem::Starfinder1e.sheet_rules_relative());
            assert_eq!(response.entries, entries(package(), kind));
        }
    }
}
