//! Ability-category member typing the corpus did not carry (SD-43 E3.12).
//!
//! PCGen fills an `ABILITYCATEGORY` with the records of its parent `CATEGORY:` that carry one of
//! its `TYPE:` tags. Many of those tags reach a record only through a later row that modifies it,
//! often in another book: `ui_abilities_class.lst:265`
//! `CATEGORY=Special Ability|Rogue Talent ~ Expert Leaper.MOD  TYPE:MaskedPerformerTalent` puts
//! the APG talent into Ultimate Intrigue's `Masked Performer Talent` category. The corpus records
//! were ingested from their base row alone (and feat records keep only the first segment of their
//! `TYPE:` in `data.category`), so E0.1's census found 29 class-granted pools with no members.
//!
//! This module merges, per `(category, key)`, the base row's `TYPE:` with every `.MOD` row's, and
//! reports the tags a corpus record lacks that some ability category filters its members on. The
//! `enrich_member_typing` binary writes them to the record as `data.member_types`; nothing here is
//! read at runtime (the oracle is read only when the binary runs).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use codex::rules_core::pi_screening::classify_field;
use codex::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};
use serde_json::Value;

/// Corpus kinds PCGen loads from ability and feat files: the records a category can select.
pub const ABILITY_KINDS: [&str; 7] =
    ["ability", "class_feature", "feat", "feat_generic", "race_trait", "race_trait_generic", "trait_generic"];

/// The field the binary writes on a record's `data`.
pub const MEMBER_TYPES_FIELD: &str = "member_types";

fn tab_fields(line: &str) -> Vec<&str> {
    line.trim_end_matches(['\n', '\r']).split('\t').filter(|f| !f.trim().is_empty()).collect()
}

fn token<'a>(fields: &[&'a str], key: &str) -> Vec<&'a str> {
    fields.iter().skip(1).filter_map(|f| f.trim().strip_prefix(key)?.strip_prefix(':')).map(str::trim).collect()
}

fn type_tags(fields: &[&str]) -> Vec<String> {
    token(fields, "TYPE").iter().flat_map(|v| v.split('.')).map(str::trim).filter(|t| !t.is_empty()).map(String::from).collect()
}

/// One oracle row that types an ability: a base row (`modifies == false`) or a `.MOD` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypingRow {
    /// The parent category, lowercased; `None` on a `.MOD` row that names none outside a feat file.
    pub category: Option<String>,
    pub key: String,
    pub types: Vec<String>,
    pub modifies: bool,
}

/// The typing rows of one `.lst` file. `feat_file` makes `FEAT` the category of a row naming none
/// (PCGen's feat files default it).
pub fn typing_rows(text: &str, feat_file: bool) -> Vec<TypingRow> {
    let default_category = feat_file.then(|| "feat".to_string());
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with("ABILITYCATEGORY:") {
            continue;
        }
        let fields = tab_fields(line);
        let Some(head) = fields.first().map(|h| h.trim()) else { continue };
        let types = type_tags(&fields);
        if let Some(target) = head.strip_suffix(".MOD") {
            if types.is_empty() {
                continue;
            }
            let (category, key) = match target.strip_prefix("CATEGORY=").and_then(|rest| rest.split_once('|')) {
                Some((category, key)) => (Some(category.trim().to_lowercase()), key.trim()),
                None => (default_category.clone(), target),
            };
            rows.push(TypingRow { category, key: key.to_string(), types, modifies: true });
            continue;
        }
        if head.contains(".COPY") || head.contains(".FORGET") || head.contains(':') {
            continue;
        }
        let category = token(&fields, "CATEGORY").first().map(|c| c.to_lowercase()).or_else(|| default_category.clone());
        let Some(category) = category else { continue };
        let key = token(&fields, "KEY").first().map_or(head, |k| *k);
        rows.push(TypingRow { category: Some(category), key: key.to_string(), types, modifies: false });
    }
    rows
}

/// The member filters of one `ABILITYCATEGORY:` row: (parent category lowercased, its `TYPE:` tags).
pub fn category_filter(line: &str) -> Option<(String, Vec<String>)> {
    if !line.starts_with("ABILITYCATEGORY:") {
        return None;
    }
    let fields = tab_fields(line);
    let parent = token(&fields, "CATEGORY").first()?.to_lowercase();
    Some((parent, type_tags(&fields)))
}

/// Every record's merged typing and every category's member filter tags, over a set of `.lst` files.
#[derive(Debug, Default)]
pub struct MemberTyping {
    /// `(category, key)` -> tags in first-seen order (base row first, then `.MOD` rows in file order).
    merged: BTreeMap<(String, String), Vec<String>>,
    /// parent category -> the lowercased tags any category of that parent filters members on.
    filters: BTreeMap<String, BTreeSet<String>>,
    /// `.MOD` rows whose target no base row in the scanned files defines.
    pub unresolved_mod_rows: usize,
}

fn push_unique(tags: &mut Vec<String>, add: &[String]) {
    for t in add {
        if !tags.iter().any(|have| have.eq_ignore_ascii_case(t)) {
            tags.push(t.clone());
        }
    }
}

impl MemberTyping {
    /// Builds from `(text, feat_file)` per ability file, base rows before `.MOD` rows, and the
    /// `ABILITYCATEGORY:` lines of every scanned file.
    pub fn from_texts<'a>(ability_files: impl IntoIterator<Item = (&'a str, bool)>, category_lines: impl IntoIterator<Item = &'a str>) -> Self {
        let mut typing = MemberTyping::default();
        let mut mods = Vec::new();
        for (text, feat_file) in ability_files {
            for row in typing_rows(text, feat_file) {
                if row.modifies {
                    mods.push(row);
                } else if let Some(category) = row.category.clone() {
                    push_unique(typing.merged.entry((category, row.key.clone())).or_default(), &row.types);
                }
            }
        }
        for row in mods {
            let targets: Vec<(String, String)> = typing
                .merged
                .keys()
                .filter(|(category, key)| key == &row.key && row.category.as_ref().is_none_or(|c| c == category))
                .cloned()
                .collect();
            if targets.is_empty() {
                typing.unresolved_mod_rows += 1;
            }
            for target in targets {
                push_unique(typing.merged.get_mut(&target).expect("listed key"), &row.types);
            }
        }
        for line in category_lines {
            if let Some((parent, tags)) = category_filter(line) {
                typing.filters.entry(parent).or_default().extend(tags.iter().map(|t| t.to_lowercase()));
            }
        }
        typing
    }

    /// Reads every `.lst` under each of `roots` (relative to `oracle_data`) plus `extra_category_files`.
    pub fn from_oracle(oracle_data: &Path, roots: &BTreeSet<String>, extra_category_files: &[PathBuf]) -> std::io::Result<Self> {
        let mut files = Vec::new();
        for root in roots {
            collect_lst(&oracle_data.join(root), &mut files)?;
        }
        files.sort();
        let mut texts = Vec::new();
        for f in files.iter().chain(extra_category_files) {
            let bytes = std::fs::read(f)?;
            let name = f.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_lowercase();
            texts.push((String::from_utf8_lossy(&bytes).into_owned(), name.contains("feat"), name.contains("abilit") || name.contains("feat")));
        }
        let ability_files = texts.iter().filter(|(_, _, ability)| *ability).map(|(t, feat, _)| (t.as_str(), *feat));
        let category_lines = texts.iter().flat_map(|(t, _, _)| t.lines());
        Ok(Self::from_texts(ability_files, category_lines))
    }

    /// The tags the oracle types `(category, key)` with that a category of `category` filters on and
    /// `carried` (lowercased) lacks, in oracle order.
    pub fn missing(&self, category: &str, key: &str, carried: &BTreeSet<String>) -> Vec<String> {
        let category = category.to_lowercase();
        let Some(filters) = self.filters.get(&category) else { return Vec::new() };
        let Some(tags) = self.merged.get(&(category.clone(), key.to_string())) else { return Vec::new() };
        tags.iter().filter(|t| filters.contains(&t.to_lowercase()) && !carried.contains(&t.to_lowercase())).cloned().collect()
    }
}

fn collect_lst(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_lst(&path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("lst") {
            out.push(path);
        }
    }
    Ok(())
}

/// A corpus record's identity and the tags it carries without `member_types`: its parent category
/// (raw `CATEGORY` token, or `FEAT` for a feat kind), key (`data.key`, else `source.record_key`) and
/// lowercased tags from its raw `TYPE` tokens, `data.category` and `data.type_tokens`.
pub fn record_identity(doc: &Value, kind: &str) -> Option<(String, String, BTreeSet<String>)> {
    let data = doc.get("data")?;
    let raw: &[Value] = data.get("raw_tokens").and_then(Value::as_array).map_or(&[], Vec::as_slice);
    let raw_value = |key: &'static str| {
        raw.iter()
            .filter(move |t| t.get("key").and_then(Value::as_str) == Some(key))
            .filter_map(|t| t.get("value").and_then(Value::as_str))
    };
    let category = raw_value("CATEGORY").next().map(String::from).or_else(|| kind.starts_with("feat").then(|| "FEAT".to_string()))?;
    let key = data
        .get("key")
        .and_then(Value::as_str)
        .or_else(|| doc.get("source")?.get("record_key")?.as_str())?
        .to_string();
    let mut carried: BTreeSet<String> = raw_value("TYPE").flat_map(|v| v.split('.')).filter(|t| !t.is_empty()).map(str::to_lowercase).collect();
    if let Some(c) = data.get("category").and_then(Value::as_str) {
        carried.extend(c.split('.').filter(|t| !t.is_empty()).map(str::to_lowercase));
    }
    if let Some(tokens) = data.get("type_tokens").and_then(Value::as_array) {
        carried.extend(tokens.iter().filter_map(Value::as_str).map(str::to_lowercase));
    }
    Some((category, key, carried))
}

/// `tags` with every tag carrying a PI blacklist term (`pi_screening::classify_field`, the screen the
/// sibling enrichment tools apply) replaced by [`REDACTED_PI_MARKER`], deduplicated in order.
pub fn screen_member_tags(tags: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for tag in tags {
        let (license, ..) = classify_field(MEMBER_TYPES_FIELD, &tag);
        let tag = if license == License::PiRedacted { REDACTED_PI_MARKER.to_string() } else { tag };
        push_unique(&mut out, &[tag]);
    }
    out
}

/// `text` (a corpus record as written: two-space JSON whose `data` object opens on its own line)
/// with `data.member_types` set to `tags`, or removed when `tags` is empty. Every other byte is kept.
pub fn with_member_types(text: &str, tags: &[String]) -> Option<String> {
    const OPEN: &str = "\n  \"data\": {\n";
    let at = text.find(OPEN)? + OPEN.len();
    let mut rest = &text[at..];
    let field = format!("    \"{MEMBER_TYPES_FIELD}\": [");
    if rest.starts_with(&field) {
        let end = rest.find("\n    ],\n")? + "\n    ],\n".len();
        rest = &rest[end..];
    }
    let mut out = String::with_capacity(text.len() + 64);
    out.push_str(&text[..at]);
    if !tags.is_empty() {
        let items: Vec<String> = tags.iter().map(|t| format!("      {}", Value::String(t.clone()))).collect();
        out.push_str(&format!("{field}\n{}\n    ],\n", items.join(",\n")));
    }
    out.push_str(rest);
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mod_row_in_another_book_adds_its_type_to_the_base_record() {
        let apg = "Expert Leaper\tKEY:Rogue Talent ~ Expert Leaper\tCATEGORY:Special Ability\tTYPE:RogueTalent.SpecialQuality\n";
        let ui = "CATEGORY=Special Ability|Rogue Talent ~ Expert Leaper.MOD\t\tTYPE:MaskedPerformerTalent\n";
        let cats = "ABILITYCATEGORY:Masked Performer Talent\tVISIBLE:QUALIFY\tCATEGORY:Special Ability\tTYPE:MaskedPerformerTalent\n";
        let typing = MemberTyping::from_texts([(apg, false), (ui, false)], cats.lines());
        let carried: BTreeSet<String> = ["roguetalent".to_string(), "specialquality".to_string()].into();
        assert_eq!(typing.missing("Special Ability", "Rogue Talent ~ Expert Leaper", &carried), vec!["MaskedPerformerTalent"]);
        assert_eq!(typing.unresolved_mod_rows, 0);
    }

    #[test]
    fn a_feat_file_row_without_category_is_a_feat_and_only_filtered_tags_are_reported() {
        let feats = "Bleeding Stare\tCATEGORY:FEAT\tTYPE:General.Combat.Stare\nMartial Weapon Proficiency.MOD\tTYPE:MartialWeaponProficiency\nMartial Weapon Proficiency\tTYPE:Combat\n";
        let cats = "ABILITYCATEGORY:Vexing Daredevil Bonus Stare Feat\tCATEGORY:Feat\tTYPE:Stare\nABILITYCATEGORY:Swashbuckler Weapon Proficiency\tCATEGORY:FEAT\tTYPE:MartialWeaponProficiency\n";
        let typing = MemberTyping::from_texts([(feats, true)], cats.lines());
        let general: BTreeSet<String> = ["general".to_string()].into();
        assert_eq!(typing.missing("FEAT", "Bleeding Stare", &general), vec!["Stare"], "Combat filters no category");
        let combat: BTreeSet<String> = ["combat".to_string()].into();
        assert_eq!(typing.missing("FEAT", "Martial Weapon Proficiency", &combat), vec!["MartialWeaponProficiency"]);
        assert!(typing.missing("Special Ability", "Bleeding Stare", &general).is_empty(), "another category's record is not this one");
    }

    #[test]
    fn member_types_is_written_replaced_and_removed_keeping_every_other_byte() {
        let text = "{\n  \"population\": \"in_scope\",\n  \"data\": {\n    \"key\": \"Bleeding Stare\"\n  }\n}\n";
        let with = with_member_types(text, &["Stare".to_string()]).unwrap();
        assert_eq!(with, "{\n  \"population\": \"in_scope\",\n  \"data\": {\n    \"member_types\": [\n      \"Stare\"\n    ],\n    \"key\": \"Bleeding Stare\"\n  }\n}\n");
        let parsed: Value = serde_json::from_str(&with).unwrap();
        assert_eq!(parsed["data"]["member_types"][0], "Stare");
        let again = with_member_types(&with, &["Stare".to_string(), "Combat".to_string()]).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&again).unwrap()["data"]["member_types"].as_array().unwrap().len(), 2);
        assert_eq!(with_member_types(&again, &[]).unwrap(), text);
    }

    #[test]
    fn a_member_tag_carrying_a_blacklist_term_is_screened_to_the_marker() {
        let tags = screen_member_tags(vec!["MagaambyaReward".into(), "Stare".into(), "LamashtuTeratomaType".into()]);
        assert_eq!(tags, vec![REDACTED_PI_MARKER.to_string(), "Stare".to_string()], "screened tags dedupe to one marker");
    }

    /// E3.22: no `member_types` tag written to the corpus carries a live blacklist term (the screen
    /// `pi_screening::classify_field` applies; `declared-pi-audit` reads the same field).
    #[test]
    fn no_written_member_types_tag_carries_a_blacklist_term() {
        let mut files = Vec::new();
        corpus_json(&crate::repo_root().join("data/corpus"), &mut files);
        let mut live = Vec::new();
        for f in files {
            let text = std::fs::read_to_string(&f).unwrap();
            if !text.contains(MEMBER_TYPES_FIELD) {
                continue;
            }
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            for tag in doc["data"].get(MEMBER_TYPES_FIELD).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str) {
                if tag != REDACTED_PI_MARKER && classify_field(MEMBER_TYPES_FIELD, tag).0 == License::PiRedacted {
                    live.push(format!("{}: {tag}", f.display()));
                }
            }
        }
        assert!(live.is_empty(), "{} member_types tag(s) carry a live blacklist term: {live:?}", live.len());
    }

    /// The 29 class-granted pools E0.1's census (`abilities-census.json` `data_gap_list`, cause
    /// `ingest:`) found empty, with their `ABILITYCATEGORY` parent and member tags; Viking Bonus Feat
    /// is not here: its in-scope category (`uw_abilitycategories.lst:47`, `CATEGORY:FEAT`) has no
    /// FEAT member in the oracle either. Every one must now have a corpus member.
    const INGEST_GAP_POOLS: [(&str, &str, &[&str]); 29] = [
        ("Advanced Masked Performer Talent", "Special Ability", &["MaskedPerformerTalent", "AdvancedMaskedPerformerTalent"]),
        ("Autumn Witch Hex Choice", "Special Ability", &["AutumnWitchHex"]),
        ("Bloody Jake Terrain", "Special Ability", &["BloodyJakeTerrain"]),
        ("Cloaked Wolf Bonus Feat", "Feat", &["CloakedWolfBonusFeat"]),
        ("Death Slayer Bonus Feat", "FEAT", &["DeathSlayerBonusFeat"]),
        ("Druid Bear Shaman Feat", "FEAT", &["DruidShamanBear"]),
        ("Druid Eagle Shaman Feat", "FEAT", &["DruidShamanEagle"]),
        ("Druid Lion Shaman Feat", "FEAT", &["DruidShamanLion"]),
        ("Druid Serpent Shaman Feat", "FEAT", &["DruidShamanSerpent"]),
        ("Druid Wolf Shaman Feat", "FEAT", &["DruidShamanWolf"]),
        ("Finesse Marksman Bonus Feat", "FEAT", &["MarksmanBonus", "FinesseMarksmanBonus"]),
        ("Hexenhammer Major Hex Choice", "Special Ability", &["HexenhammerMinorHex", "HexenhammerMajorHex"]),
        ("Hexenhammer Minor Hex Choice", "Special Ability", &["HexenhammerMinorHex"]),
        ("Lantern Bearer Favored Enemy", "Special Ability", &["LanternBearerFavoredEnemy"]),
        ("Lantern Bearer Favored Enemy Bonus", "Special Ability", &["LanternBearerFavoredEnemyBonus"]),
        ("Life Channeler Domain", "Special Ability", &["LifeChannelerDomain"]),
        ("Mammoth Rider Born Survivor Feat", "FEAT", &["MammothRiderBonusFeat"]),
        ("Masked Performer Talent", "Special Ability", &["MaskedPerformerTalent"]),
        ("Psionic Fighter Starting Feat", "FEAT", &["PsionicFighterStartingFeat"]),
        ("Sniper Marksman Bonus Feat", "FEAT", &["MarksmanBonus", "SniperMarksmanBonus"]),
        ("Spring Witch Hex Choice", "Special Ability", &["SpringWitchHex"]),
        ("Summer Witch Hex Choice", "Special Ability", &["SummerWitchHex"]),
        ("Swashbuckler Weapon Proficiency", "FEAT", &["MartialWeaponProficiency"]),
        ("Tempest Druid Domain Choice", "Special Ability", &["TempestDruidDomain"]),
        ("Undead Master Arcane Bond", "Special Ability", &["UndeadMasterArcaneBond"]),
        ("Vexing Daredevil Bonus Stare Feat", "Feat", &["Stare"]),
        ("Volley Marksman Bonus Feat", "FEAT", &["MarksmanBonus", "VolleyMarksmanBonus"]),
        ("Winter Witch Hex Choice", "Special Ability", &["WinterWitchHex"]),
        ("Winter Witch Patron", "Special Ability", &["WinterWitchPatron"]),
    ];

    fn corpus_json(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                corpus_json(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }

    #[test]
    fn every_ingest_gap_pool_has_a_corpus_member() {
        let corpus = crate::repo_root().join("data/corpus");
        let mut members: BTreeMap<&str, usize> = INGEST_GAP_POOLS.iter().map(|(pool, _, _)| (*pool, 0)).collect();
        for book in std::fs::read_dir(&corpus).unwrap().flatten().filter(|e| e.path().is_dir()) {
            for kind in ABILITY_KINDS {
                let dir = book.path().join(kind);
                if !dir.is_dir() {
                    continue;
                }
                let mut files = Vec::new();
                corpus_json(&dir, &mut files);
                for f in files {
                    let doc: Value = serde_json::from_str(&std::fs::read_to_string(&f).unwrap()).unwrap();
                    let Some((category, _, mut tags)) = record_identity(&doc, kind) else { continue };
                    if let Some(extra) = doc["data"].get(MEMBER_TYPES_FIELD).and_then(Value::as_array) {
                        tags.extend(extra.iter().filter_map(Value::as_str).map(str::to_lowercase));
                    }
                    for (pool, parent, types) in INGEST_GAP_POOLS {
                        if category.eq_ignore_ascii_case(parent) && types.iter().any(|t| tags.contains(&t.to_lowercase())) {
                            *members.get_mut(pool).unwrap() += 1;
                        }
                    }
                }
            }
        }
        let empty: Vec<&str> = members.iter().filter(|(_, n)| **n == 0).map(|(pool, _)| *pool).collect();
        assert!(empty.is_empty(), "{} of 29 ingest-gap pools have no corpus member: {empty:?}", empty.len());
    }
}
