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

use codex::rules_core::rules_tables::class_spell_levels;

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
            Some(entries) => ClassSpellLevelsDto {
                class_id: class_id.clone(),
                known: true,
                entries: entries
                    .into_iter()
                    .map(|(key, level)| ClassSpellLevelDto {
                        key: key.to_string(),
                        level,
                    })
                    .collect(),
            },
            None => ClassSpellLevelsDto {
                class_id: class_id.clone(),
                known: false,
                entries: Vec::new(),
            },
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

    #[test]
    fn the_wizard_list_is_the_full_five_hundred_and_eighty() {
        assert_eq!(levels_for("class:wizard").entries.len(), 580);
        assert_eq!(levels_for("class:sorcerer").entries.len(), 578);
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
    /// **The gap shrank on 2026-07-31, and that is the whole point of the
    /// pin.** It was 73 of Bloodrager's 200 entries and 21 of Shaman's
    /// while the catalog served CRB+APG+ACG only. `spell_catalog.rs` then
    /// chained `advanced_race_guide::spell_list`'s 92 records, and since
    /// ARG's keys are the *only* delta between the old catalog and the new
    /// one, the 23 Bloodrager and 6 Shaman keys that started joining are
    /// necessarily ARG base records — one of the three books the ruling
    /// above named as un-ingested, now ingested. 50 and 15 remain, and
    /// those are the Ultimate Magic / Ultimate Combat remainder. Re-derive
    /// rather than relax these when another book lands.
    ///
    /// Every OTHER dispatched class joins completely, which is the part
    /// the frontend actually depends on.
    #[test]
    fn every_served_key_joins_to_a_catalog_record_outside_the_two_documented_gaps() {
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
        assert_eq!(gaps, vec![("class:bloodrager", 50), ("class:shaman", 15)]);
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

        assert_eq!(wrong_for("class:wizard"), 67);
        assert_eq!(wrong_for("class:sorcerer"), 67);
        assert_eq!(wrong_for("class:druid"), 52);
        assert_eq!(wrong_for("class:cleric"), 46);
        assert_eq!(wrong_for("class:bard"), 13);
        assert_eq!(wrong_for("class:ranger"), 8);
    }
}
