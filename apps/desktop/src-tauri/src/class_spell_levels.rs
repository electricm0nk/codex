//! Per-class spell levels — Tauri command adapter over the engine's
//! `rules_tables::class_spell_levels` dispatch.
//!
//! **Why this command exists.** `spell_catalog.rs` serves each spell
//! record's own `level`, which is the MINIMUM spell level across every
//! class named in that record's corpus `CLASSES:` tag — not the level for
//! the character's class. `Hideous Laughter` is
//! `CLASSES:Bard=1|Sorcerer,Wizard=2`, so a Wizard's sheet read "Level 1"
//! for a spell a Wizard learns at 2. Re-derived from the shipped tables,
//! **67 of the 580 spells on the Wizard list** had a record level that was
//! simply the wrong number for a Wizard, every one of them biased low; a
//! further 2 (`Malediction`, `Unravel Destiny`) had no record level at all
//! though their Wizard level is known. Every other casting class has the
//! same defect at its own rate — Druid 19.2%, Cleric 15.6%, Bard 4.9%.
//! The per-class tables holding the true answer were already in the
//! engine; no command routed a class id to them, which is the only reason
//! the sheet fell back to the record's number.
//!
//! **This command reports absence, it never fills it.** A class the engine
//! has no ingested list for comes back `known: false` with no entries,
//! rather than the record-level fallback that caused the bug (see
//! `docs/governance/no-stub-mvp-doctrine.md`). Magus, Summoner and Oracle
//! are the live examples: they name themselves in real corpus `CLASSES:`
//! tags, so their levels are knowable, but nothing has ingested them yet
//! and the honest answer is "not known here".
//!
//! Mirrors `spell_catalog.rs`'s command/pure-fn split so the response
//! shape is testable without a Tauri runtime.

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::sync::OnceLock;

use codex::rules_core::rules_tables::class_spell_levels;

use crate::authoring_workbench::codex_repo_root;
use crate::class_catalog_generic::{tokens_from, walk_json_files};

/// One `(spell key, level)` pair from a single class's spell list. The
/// `key` matches `SpellCatalogEntryDto::key` exactly, so the frontend can
/// join the two responses on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSpellLevelDto {
    pub key: String,
    /// The spell's level **for this class specifically**, 0-9.
    pub level: u8,
}

/// One requested class's answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSpellLevelsDto {
    /// Echoed back verbatim, including a class id the engine cannot
    /// answer for — callers index this response by the same string they
    /// asked with.
    pub class_id: String,
    /// `false` when the engine has no ingested spell list for this class.
    /// Distinct from an empty `entries` on a known class: `false` means
    /// "no level can be reported without inventing one", so the caller
    /// must render the absence rather than substitute a record level.
    pub known: bool,
    /// Sorted by key. Always empty when `known` is `false`.
    pub entries: Vec<ClassSpellLevelDto>,
    /// v0.8 B-9: what `known: false` could not say -- whether this class
    /// casts at all. See [`SpellcastingStatus`].
    pub spellcasting: SpellcastingStatus,
    /// The corpus record's own `FACT:SpellType|<X>` value (`Arcane`,
    /// `Divine`, `Psychic`, ...) verbatim, or `None` when the record
    /// carries no such token (a non-caster) or no record was found.
    pub spell_type: Option<String>,
}

/// v0.8 B-9 (audit item 45 / F-10): `known: false` alone conflated "a
/// Fighter has no spells" with "the Oracle list was never transcribed",
/// which forced the frontend to hand-list the casters -- a rules judgment
/// §3.3 forbids in TypeScript. This reads the fact from the ingested
/// corpus class record instead: PCGen tags every spellcasting class line
/// with `FACT:SpellType|Arcane|Divine|Psychic` and no martial class with
/// one (`data/corpus/<book>/class/<class>.json` `raw_tokens`; 49 of 168
/// records carry it, every one a caster). Never a class-id list in Rust.
///
/// An Unchained record (`"Unchained Summoner"`) is a
/// `"<Base> Class Selection..."` shell whose `TYPE` names its base class
/// and which carries no `SpellType` of its own; it inherits its base
/// record's answer, read from that record -- again a corpus token, not a
/// list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpellcastingStatus {
    /// The engine serves this class's spell list (`known: true`).
    ListIngested,
    /// The corpus record says this class casts, but the engine has no
    /// transcribed list for it -- the "Add Spell" affordance is honest to
    /// offer, with nothing to route to yet.
    CasterListNotIngested,
    /// The corpus record carries no `FACT:SpellType`: this class does not
    /// cast. Do not offer "Add Spell".
    NonCaster,
    /// No ingested class record answers to this id (or the record carries
    /// no tokens at all), so neither of the above can honestly be claimed.
    ClassNotInCorpus,
}

/// One ingested class record's spellcasting-relevant facts.
#[derive(Debug, Clone)]
struct CorpusClassFacts {
    spell_type: Option<String>,
    /// For a `"<Base> Class Selection..."` shell, the base class's
    /// normalized id; `None` for a real base class record.
    base_selection_of: Option<String>,
}

/// `"Unchained Summoner"` / `"oracle"` / `"Fighter"` -> `"class:unchained_summoner"` /
/// `"class:oracle"` / `"class:fighter"` -- the id shape the frontend and
/// `class_spell_levels` already use.
fn normalize_class_id(identity: &str) -> String {
    let slug: String = identity
        .trim()
        .chars()
        .map(|c| if c == ' ' || c == '-' { '_' } else { c.to_ascii_lowercase() })
        .collect();
    format!("class:{slug}")
}

/// Every `data/corpus/<book>/class/*.json` record's facts, keyed by
/// normalized class id. Identity is the record's own `class_id` (CRB/APG/
/// ACG shape) or `name` (every other book), whichever it carries. Records
/// with no `raw_tokens` are skipped -- they cannot answer either way.
fn corpus_class_facts() -> &'static BTreeMap<String, CorpusClassFacts> {
    static INDEX: OnceLock<BTreeMap<String, CorpusClassFacts>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut index = BTreeMap::new();
        let Ok(repo_root) = codex_repo_root() else { return index };
        let corpus_root = repo_root.join("data/corpus");
        let Ok(books) = std::fs::read_dir(&corpus_root) else { return index };
        for book in books.flatten() {
            let dir = book.path().join("class");
            if !dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                let data = &doc["data"];
                let Some(identity) = data["class_id"].as_str().or_else(|| data["name"].as_str()) else {
                    continue;
                };
                let tokens = tokens_from(data);
                if tokens.is_empty() {
                    continue;
                }
                let spell_type = tokens
                    .iter()
                    .find_map(|(k, v)| if k == "FACT" { v.strip_prefix("SpellType|") } else { None })
                    .map(str::to_owned);
                let base_selection_of = tokens
                    .iter()
                    .find(|(k, _)| k == "TYPE")
                    .and_then(|(_, v)| v.split_once(" Class Selection"))
                    .map(|(base, _)| normalize_class_id(base));
                index.insert(normalize_class_id(identity), CorpusClassFacts { spell_type, base_selection_of });
            }
        }
        index
    })
}

/// Resolves a class id to its corpus spell type, following at most one
/// `"<Base> Class Selection"` hop so an Unchained shell answers with its
/// base class's token. `Err(())` when no record with tokens answers.
fn corpus_spell_type(class_id: &str) -> Result<Option<String>, ()> {
    let index = corpus_class_facts();
    let facts = index.get(class_id).ok_or(())?;
    if facts.spell_type.is_some() {
        return Ok(facts.spell_type.clone());
    }
    if let Some(base) = &facts.base_selection_of {
        if let Some(base_facts) = index.get(base) {
            return Ok(base_facts.spell_type.clone());
        }
    }
    Ok(None)
}

fn spellcasting_for(class_id: &str, list_known: bool) -> (SpellcastingStatus, Option<String>) {
    match corpus_spell_type(class_id) {
        Err(()) => (SpellcastingStatus::ClassNotInCorpus, None),
        Ok(None) => (SpellcastingStatus::NonCaster, None),
        Ok(Some(spell_type)) if list_known => (SpellcastingStatus::ListIngested, Some(spell_type)),
        Ok(Some(spell_type)) => (SpellcastingStatus::CasterListNotIngested, Some(spell_type)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSpellLevelsResponse {
    pub classes: Vec<ClassSpellLevelsDto>,
}

/// Builds the per-class spell-level response for `class_ids`, one entry
/// per requested id, in request order. A thin, testable wrapper behind the
/// Tauri command below.
///
/// Duplicate ids in the request are answered once each (the response stays
/// parallel to the request) — de-duplicating is the caller's business, and
/// silently dropping a requested id would break index-by-position callers.
pub fn build_class_spell_levels(class_ids: &[String]) -> ClassSpellLevelsResponse {
    let classes = class_ids
        .iter()
        .map(|class_id| match class_spell_levels::class_spell_list_entries(class_id) {
            Some(entries) => {
                let (spellcasting, spell_type) = spellcasting_for(class_id, true);
                ClassSpellLevelsDto {
                    class_id: class_id.clone(),
                    known: true,
                    entries: entries
                        .into_iter()
                        .map(|(key, level)| ClassSpellLevelDto {
                            key: key.to_string(),
                            level,
                        })
                        .collect(),
                    spellcasting,
                    spell_type,
                }
            }
            None => {
                let (spellcasting, spell_type) = spellcasting_for(class_id, false);
                ClassSpellLevelsDto {
                    class_id: class_id.clone(),
                    known: false,
                    entries: Vec::new(),
                    spellcasting,
                    spell_type,
                }
            }
        })
        .collect();
    ClassSpellLevelsResponse { classes }
}

/// The `list_class_spell_levels` argument.
///
/// Wrapped in a named struct rather than taken as a bare `class_ids:
/// Vec<String>` parameter, matching every other command in this crate
/// (`list_spells`'s `filter`, `load_character_bio`'s `request`, ...). That
/// keeps the JS-side field naming a `serde` concern this crate can test,
/// instead of relying on Tauri's implicit camelCase-to-snake_case
/// conversion of multi-word parameter names — a conversion no existing
/// command here exercises, so nothing would have caught it going wrong.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassSpellLevelsRequest {
    /// The `class:<id>`s to answer for. Empty yields an empty response,
    /// never the whole roster.
    pub class_ids: Vec<String>,
}

/// Returns the per-class spell levels for each requested `class:<id>`.
/// See `ClassSpellLevelsDto`'s own doc comments for how an unknown class
/// is reported.
#[tauri::command]
pub fn list_class_spell_levels(request: ClassSpellLevelsRequest) -> ClassSpellLevelsResponse {
    build_class_spell_levels(&request.class_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spell_catalog::build_spell_catalog;

    fn levels_for(class_id: &str) -> ClassSpellLevelsDto {
        build_class_spell_levels(&[class_id.to_owned()])
            .classes
            .into_iter()
            .next()
            .expect("one requested class yields one answer")
    }

    fn level_of(class_id: &str, key: &str) -> Option<u8> {
        levels_for(class_id)
            .entries
            .into_iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.level)
    }

    /// The bug, at the surface the player actually reads: the catalog
    /// serves Hideous Laughter as level 1 (its record's minimum across
    /// classes), and this command supplies the real per-class answer that
    /// corrects it — 2 for a Wizard, 1 for a Bard.
    // ----- v0.8 B-9: caster-with-no-ingested-list vs non-caster -----

    fn status_of(class_id: &str) -> (SpellcastingStatus, Option<String>) {
        let answer = levels_for(class_id);
        (answer.spellcasting, answer.spell_type)
    }

    /// A martial class's corpus record carries no `FACT:SpellType`, so it
    /// is a non-caster -- distinct from "we never transcribed its list".
    #[test]
    fn a_fighter_is_reported_as_a_non_caster() {
        assert_eq!(status_of("class:fighter"), (SpellcastingStatus::NonCaster, None));
        assert!(!levels_for("class:fighter").known);
    }

    /// The three classes F-10 had to hand-list: each corpus record carries
    /// `FACT:SpellType`, and none has an ingested spell list.
    #[test]
    fn oracle_summoner_and_magus_are_casters_whose_list_is_not_ingested() {
        for (class_id, spell_type) in [
            ("class:oracle", "Divine"),
            ("class:summoner", "Arcane"),
            ("class:magus", "Arcane"),
        ] {
            let (status, kind) = status_of(class_id);
            assert_eq!(status, SpellcastingStatus::CasterListNotIngested, "{class_id}");
            assert_eq!(kind.as_deref(), Some(spell_type), "{class_id}");
            assert!(!levels_for(class_id).known, "{class_id}");
        }
    }

    /// An Unchained record is a `"<Base> Class Selection"` shell with no
    /// `SpellType` of its own; its caster-ness is its base class's, read
    /// from the base record -- never from a class-id list.
    #[test]
    fn unchained_classes_inherit_their_base_records_caster_status() {
        assert_eq!(
            status_of("class:unchained_summoner"),
            (SpellcastingStatus::CasterListNotIngested, Some("Arcane".to_owned()))
        );
        assert_eq!(status_of("class:unchained_barbarian"), (SpellcastingStatus::NonCaster, None));
        assert_eq!(status_of("class:unchained_rogue"), (SpellcastingStatus::NonCaster, None));
    }

    #[test]
    fn a_class_with_an_ingested_list_is_reported_as_such_with_its_spell_type() {
        assert_eq!(
            status_of("class:wizard"),
            (SpellcastingStatus::ListIngested, Some("Arcane".to_owned()))
        );
        assert_eq!(
            status_of("class:cleric"),
            (SpellcastingStatus::ListIngested, Some("Divine".to_owned()))
        );
    }

    /// An id no corpus class record answers to is neither caster nor
    /// non-caster -- it is unknown, and says so.
    #[test]
    fn an_id_with_no_corpus_class_record_is_reported_unknown_not_non_caster() {
        assert_eq!(status_of("class:no_such_class"), (SpellcastingStatus::ClassNotInCorpus, None));
    }

    /// The corpus token and the engine's own transcribed-list set agree:
    /// every class the engine serves a list for carries `FACT:SpellType`.
    /// If this ever fails, the token is not the caster fact we think it is.
    #[test]
    fn every_class_with_an_ingested_list_carries_a_corpus_spell_type() {
        for class_id in class_spell_levels::classes_with_spell_lists() {
            let (status, kind) = status_of(class_id);
            assert_eq!(status, SpellcastingStatus::ListIngested, "{class_id}");
            assert!(kind.is_some(), "{class_id} has a list but no FACT:SpellType");
        }
    }

    #[test]
    fn a_wizard_reads_hideous_laughter_as_second_level_where_the_catalog_says_first() {
        let catalog_level = build_spell_catalog()
            .entries
            .into_iter()
            .find(|entry| entry.key == "Hideous Laughter")
            .expect("Hideous Laughter is a real catalog record")
            .level;
        assert_eq!(catalog_level, Some(1));

        assert_eq!(level_of("class:wizard", "Hideous Laughter"), Some(2));
        assert_eq!(level_of("class:sorcerer", "Hideous Laughter"), Some(2));
        assert_eq!(level_of("class:bard", "Hideous Laughter"), Some(1));
    }

    #[test]
    fn a_class_with_no_ingested_list_is_reported_unknown_with_no_entries() {
        for class_id in ["class:magus", "class:summoner", "class:oracle", "class:fighter"] {
            let answer = levels_for(class_id);
            assert_eq!(answer.class_id, class_id);
            assert!(!answer.known, "{class_id} should not claim a known list");
            assert!(answer.entries.is_empty(), "{class_id} served entries anyway");
        }
    }

    #[test]
    fn every_casting_class_the_engine_knows_comes_back_known_and_non_empty() {
        for class_id in class_spell_levels::classes_with_spell_lists() {
            let answer = levels_for(class_id);
            assert!(answer.known, "{class_id}");
            assert!(!answer.entries.is_empty(), "{class_id}");
        }
    }

    /// **580 and 578 while the per-class tables covered CRB+APG+ACG only;
    /// 642 and 640 since SD-27 ingested the Advanced Race Guide's own
    /// `CLASSES:` token** into
    /// `advanced_race_guide::class_spell_levels`. ARG names Wizard on 62 of
    /// its 92 spells and Sorcerer on the same 62, and none of those keys was
    /// already on either list — hence exactly +62 on both. The engine-side
    /// `the_two_layers_agree_wherever_they_overlap` pins that the two
    /// layers never contradict each other where they do overlap.
    ///
    /// Re-derive rather than relax these when another book lands.
    #[test]
    fn the_wizard_and_sorcerer_lists_carry_every_ingested_books_rows() {
        assert_eq!(levels_for("class:wizard").entries.len(), 642);
        assert_eq!(levels_for("class:sorcerer").entries.len(), 640);
    }

    /// **The join is one-directional, and that is what makes the known
    /// catalog gap harmless.** The Spells tab resolves a selection against
    /// the catalog first and only then looks up that record's per-class
    /// level here, so a class-list key with no catalog record is never
    /// reached by the display path.
    ///
    /// Two classes have such keys, by a documented ruling rather than an
    /// oversight (team lead, 2026-07-27; see
    /// `acg::bloodrager_spell_list`'s doc comment): they are `.MOD` grafts
    /// whose base records live in Ultimate Magic / Ultimate Combat / the
    /// Advanced Race Guide. They are genuinely on those classes' PF1
    /// spell lists; only the record is missing.
    ///
    /// **The gap shrank on 2026-07-31, again on 2026-08-15, and closed
    /// entirely on 2026-08-16 -- and that is the whole point of the pin.**
    /// It was 73 of Bloodrager's 200 entries and 21 of Shaman's while the
    /// catalog served CRB+APG+ACG only. `spell_catalog.rs` then chained
    /// `advanced_race_guide::spell_list`'s 92 records, and since ARG's keys
    /// are the *only* delta between the old catalog and the new one, the 23
    /// Bloodrager and 6 Shaman keys that started joining are necessarily
    /// ARG base records — one of the three books the ruling above named as
    /// un-ingested, now ingested. SD31-E6-F2-002 (2026-08-15) chained
    /// `ultimate_magic::spell_list`'s 269 records
    /// (`spell_resolver::spell_catalog_rows()`'s 6th book) — UM's own base
    /// records for every Shaman-class `.MOD` graft this file carries now
    /// join, closing that gap to zero, and 30 of Bloodrager's 50 remaining
    /// grafts join too (their UM base records now exist), leaving 20 — the
    /// Ultimate Combat remainder (`acg::bloodrager_spell_list`'s doc
    /// comment names it), un-ingested by any book chained into the catalog
    /// at that cycle's tip. **SD31-E6-F2-004 (2026-08-16) chained
    /// `ultimate_combat::spell_list`'s 146 records** (the catalog's 8th
    /// book) — every one of Bloodrager's remaining 20 `.MOD` grafts joins
    /// its now-real Ultimate Combat base record, closing the gap to
    /// **zero**. Re-derive rather than relax this if another book widens
    /// the catalog and a new gap opens.
    ///
    /// Every dispatched class joins completely, which is the part the
    /// frontend actually depends on.
    #[test]
    fn every_served_key_joins_to_a_catalog_record_outside_the_one_documented_gap() {
        let catalog: std::collections::HashSet<String> = build_spell_catalog()
            .entries
            .into_iter()
            .map(|entry| entry.key)
            .collect();

        let mut gaps: Vec<(&str, usize)> = Vec::new();
        for class_id in class_spell_levels::classes_with_spell_lists() {
            let missing = levels_for(class_id)
                .entries
                .into_iter()
                .filter(|entry| !catalog.contains(&entry.key))
                .count();
            if missing > 0 {
                gaps.push((class_id, missing));
            }
        }
        assert_eq!(
            gaps,
            Vec::<(&str, usize)>::new(),
            "SD31-E6-F2-004 chained ultimate_combat::spell_list into the catalog, and the \
             remaining 20-entry Bloodrager gap this test used to pin was exactly that book's \
             remainder (see this test's own doc comment) -- it closes to zero, not shrinks"
        );
    }

    #[test]
    fn a_multi_class_request_answers_each_class_in_request_order() {
        let response = build_class_spell_levels(&[
            "class:wizard".to_owned(),
            "class:magus".to_owned(),
            "class:bard".to_owned(),
        ]);
        let ids: Vec<&str> = response
            .classes
            .iter()
            .map(|entry| entry.class_id.as_str())
            .collect();
        assert_eq!(ids, vec!["class:wizard", "class:magus", "class:bard"]);
        assert!(response.classes[0].known);
        assert!(!response.classes[1].known);
        assert!(response.classes[2].known);
    }

    #[test]
    fn a_duplicate_class_id_is_answered_once_per_occurrence() {
        let response =
            build_class_spell_levels(&["class:wizard".to_owned(), "class:wizard".to_owned()]);
        assert_eq!(response.classes.len(), 2);
        assert_eq!(response.classes[0], response.classes[1]);
    }

    #[test]
    fn an_empty_request_yields_an_empty_response_rather_than_the_whole_roster() {
        assert!(build_class_spell_levels(&[]).classes.is_empty());
    }

    /// Pins the wire field name the frontend actually sends. The command
    /// takes a request struct precisely so this is testable here rather
    /// than being an assumption about Tauri's parameter-name conversion.
    #[test]
    fn the_request_deserialises_from_the_camel_case_field_the_frontend_sends() {
        let request: ClassSpellLevelsRequest =
            serde_json::from_str(r#"{"classIds":["class:wizard","class:bard"]}"#)
                .expect("the frontend's camelCase payload must deserialise");
        assert_eq!(request.class_ids, vec!["class:wizard", "class:bard"]);
    }

    /// And the response serialises to the camelCase shape
    /// `loadClassSpellLevels.ts` declares.
    #[test]
    fn the_response_serialises_to_the_camel_case_shape_the_frontend_reads() {
        let response = build_class_spell_levels(&["class:magus".to_owned()]);
        let json = serde_json::to_value(&response).expect("serialisable");
        let entry = &json["classes"][0];
        assert_eq!(entry["classId"], "class:magus");
        assert_eq!(entry["known"], false);
        assert!(entry["entries"].as_array().expect("array").is_empty());

        let wizard = serde_json::to_value(build_class_spell_levels(&["class:wizard".to_owned()]))
            .expect("serialisable");
        let first = &wizard["classes"][0]["entries"][0];
        assert!(first["key"].is_string());
        assert!(first["level"].is_number());
    }

    #[test]
    fn entries_are_sorted_by_key_and_carry_no_duplicates() {
        for class_id in class_spell_levels::classes_with_spell_lists() {
            let entries = levels_for(class_id).entries;
            let keys: Vec<&str> = entries.iter().map(|entry| entry.key.as_str()).collect();
            let mut sorted = keys.clone();
            sorted.sort_unstable();
            assert_eq!(keys, sorted, "{class_id} entries are not sorted by key");
            let mut deduped = sorted.clone();
            deduped.dedup();
            assert_eq!(deduped.len(), keys.len(), "{class_id} has a duplicate key");
        }
    }

    /// Re-derives, at this boundary, how many rows the old record-level
    /// rendering got wrong for each class — the measurement that justifies
    /// this command, computed from what the two commands actually serve
    /// rather than asserted from a prior agent's note.
    ///
    /// **Every figure rose when SD-27 ingested ARG's per-class levels**
    /// (Wizard 67 -> 79, Sorcerer 67 -> 79, Cleric 46 -> 60, Druid 52 -> 57,
    /// Bard 13 -> 16, Ranger 8 -> 10), because ARG's records are now on
    /// these lists and each one whose record `level` is the minimum across
    /// its own `CLASSES:` groups reads wrong for the higher classes exactly
    /// as CRB's do. Asserted as one vector so a drift shows every class at
    /// once rather than only the first.
    #[test]
    fn the_catalog_level_disagrees_with_the_class_level_at_the_expected_rate() {
        let catalog: std::collections::HashMap<String, Option<u8>> = build_spell_catalog()
            .entries
            .into_iter()
            .map(|entry| (entry.key, entry.level))
            .collect();

        let wrong_for = |class_id: &str| -> usize {
            levels_for(class_id)
                .entries
                .into_iter()
                .filter(|entry| {
                    matches!(catalog.get(&entry.key), Some(Some(shown)) if *shown != entry.level)
                })
                .count()
        };

        let measured: Vec<(&str, usize)> =
            ["class:wizard", "class:sorcerer", "class:druid", "class:cleric", "class:bard",
             "class:ranger"]
                .into_iter()
                .map(|class_id| (class_id, wrong_for(class_id)))
                .collect();
        assert_eq!(
            measured,
            vec![
                ("class:wizard", 79),
                ("class:sorcerer", 79),
                ("class:druid", 57),
                ("class:cleric", 60),
                ("class:bard", 16),
                ("class:ranger", 10),
            ]
        );
    }
}
