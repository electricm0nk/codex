//! Ingest-format reader for an already-converted `data/corpus/` **race** and
//! **race-trait** record, and the settled canonical records built from them.
//!
//! # Why this module is on the converter side
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 14, the race-side sibling of cycle 13's
//! [`corpus_equipment_json`](crate::pcgen_import::corpus_equipment_json).
//!
//! `rules_core::race_resolver` used to deserialize the ingest cache payloads
//! [`RaceCacheData`] / [`RaceTraitCacheData`] — which still carry the `.lst`
//! row's `raw_tokens` and `raw_bonus_chains` arrays — and then read eleven
//! separate facts back out of those arrays at run time through
//! [`race_trait_tokens`] and [`bonus_chain_reader`]. That was the live race
//! resolver's whole converter dependency: three `use crate::pcgen_import::…`
//! lines (`decisions.md` §11, §19 ruling B16).
//!
//! The payload shapes, the token grammar and the chain grammar are the
//! converter's own vocabulary, so the reading happens here now. The live
//! resolver asks one question at the ingest boundary — *"what canonical record
//! does this corpus JSON object stand for?"* — and never names a token, a token
//! array, a bonus chain or a cache payload to ask it.
//!
//! # What is NOT settled here
//!
//! This is the ingest boundary, not a second converter. Every field below is
//! filled by **the same function, with the same arguments**, that
//! `race_resolver` called at run time before the move; the call sites are
//! listed one per line so a reader can diff them against the old ones. Nothing
//! is re-derived, no value is re-parsed, and no reading was widened, narrowed
//! or reordered. The whole-corpus parity proof at the bottom of this module is
//! that claim, measured over every book under `data/corpus/` rather than
//! asserted.
//!
//! KEPT for Starfinder, like the rest of `src/pcgen_import/`.

use crate::pcgen_import::bonus_chain_reader;
use crate::pcgen_import::ingest_payload::{RaceCacheData, RaceTraitCacheData};
use crate::pcgen_import::race_trait_tokens;
use crate::rules_core::race_record::{CorpusRaceRecord, CorpusRaceTraitRecord};

/// The settled [`CorpusRaceRecord`] for one already-converted
/// `data/corpus/<book>/race/<slug>.json` record's `data` object.
///
/// `None` when the object is not a race payload at all — a malformed file must
/// not take down a whole book's real data, and the caller records the skip.
pub fn corpus_race_source_record(data: &serde_json::Value) -> Option<CorpusRaceRecord> {
    let payload: RaceCacheData = serde_json::from_value(data.clone()).ok()?;
    Some(race_record_from_payload(&payload))
}

/// The settled [`CorpusRaceTraitRecord`] for one already-converted
/// `data/corpus/<book>/race_trait/<race>/<slug>.json` record's `data` object.
///
/// `None` when the object is not a race-trait payload at all.
pub fn corpus_race_trait_source_record(data: &serde_json::Value) -> Option<CorpusRaceTraitRecord> {
    let payload: RaceTraitCacheData = serde_json::from_value(data.clone()).ok()?;
    Some(race_trait_record_from_payload(&payload))
}

/// Settles one deserialized chassis payload. No reading is involved: every
/// field on [`RaceCacheData`] outside its token array is already a settled
/// value, and this is the transcription that drops the array.
fn race_record_from_payload(payload: &RaceCacheData) -> CorpusRaceRecord {
    CorpusRaceRecord {
        key: payload.key.clone(),
        name: payload.name.clone(),
        base_size: payload.base_size.clone(),
        base_move_walk: payload.base_move_walk,
        race_type: payload.race_type.clone(),
        type_tokens: payload.type_tokens.clone(),
        legs: payload.legs,
        hands: payload.hands,
    }
}

/// Settles one deserialized racial-trait payload.
///
/// The second block is the eleven run-time readings `race_resolver` used to
/// perform, each by the same call it used to make. `skinwalker_change_shape_kin`
/// is derived from `automatic_trait_grants` here exactly as
/// `RaceTraitRecord::skinwalker_change_shape_kin` derived it there: first grant
/// whose pool-name prefix the reader recognises.
fn race_trait_record_from_payload(payload: &RaceTraitCacheData) -> CorpusRaceTraitRecord {
    let automatic_trait_grants = race_trait_tokens::automatic_ability_grants(payload);
    let skinwalker_change_shape_kin = automatic_trait_grants
        .iter()
        .find_map(|grant| race_trait_tokens::skinwalker_change_shape_kin(grant).map(str::to_string));
    CorpusRaceTraitRecord {
        key: payload.key.clone(),
        name: payload.name.clone(),
        race_key: payload.race_key.clone(),
        category: payload.category.clone(),
        type_tokens: payload.type_tokens.clone(),
        is_racial_default: payload.is_racial_default,
        suppressed_by_flag: payload.suppressed_by_flag.clone(),
        sets_replace_flags: payload.sets_replace_flags.clone(),
        description: payload.description.clone(),
        source_page: payload.source_page.clone(),

        exclusion_guard_flags: race_trait_tokens::exclusion_guard_flags(payload),
        negated_fact_gates: race_trait_tokens::negated_fact_gates(payload),
        declares_negated_ability_guard: race_trait_tokens::declares_preability_negated_guard(payload),
        automatic_trait_grants,
        skinwalker_change_shape_kin,
        positive_prefact_flag: race_trait_tokens::positive_prefact_flag(payload),
        declared_walk_speed_ft: race_trait_tokens::declared_walk_speed_ft(payload),
        declared_size: race_trait_tokens::declared_size(payload),
        declared_vision: race_trait_tokens::declared_vision_segments(payload),
        declared_bonuses: bonus_chain_reader::declared_bonuses(payload),
        template_bonus_languages: race_trait_tokens::declared_template_bonus_languages(&payload.raw_tokens),
        adopted_race_pool_suffix: race_trait_tokens::adopted_race_pool_suffix(payload),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    /// Every `data/corpus/<book>/<kind>/**/*.json` file the live race resolver
    /// walks, by exactly its traversal rule: recurse, skip `_parity/` and
    /// `LICENSE.json`, take `*.json`.
    fn corpus_files(kind: &str) -> Vec<PathBuf> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/corpus");
        let mut out = Vec::new();
        let Ok(books) = std::fs::read_dir(&root) else { return out };
        for book in books.flatten() {
            let dir = book.path().join(kind);
            if !dir.is_dir() {
                continue;
            }
            let mut stack = vec![dir];
            while let Some(current) = stack.pop() {
                let Ok(entries) = std::fs::read_dir(&current) else { continue };
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if path.is_dir() {
                        if name != "_parity" {
                            stack.push(path);
                        }
                    } else if name.ends_with(".json") && name != "LICENSE.json" {
                        out.push(path);
                    }
                }
            }
        }
        out.sort();
        out
    }

    fn data_objects(kind: &str) -> Vec<(PathBuf, serde_json::Value)> {
        corpus_files(kind)
            .into_iter()
            .filter_map(|path| {
                let text = std::fs::read_to_string(&path).ok()?;
                let value: serde_json::Value = serde_json::from_str(&text).ok()?;
                let data = value.get("data")?.clone();
                Some((path, data))
            })
            .collect()
    }

    /// **The parity proof for the chassis half of cycle 14's move.**
    ///
    /// Re-derives every settled field the way `race_resolver` derived it before
    /// the move — straight off the deserialized [`RaceCacheData`] — and compares
    /// field for field over **every** race record under `data/corpus/`.
    #[test]
    fn every_live_corpus_race_record_carries_the_same_values_the_payload_read_produced() {
        let objects = data_objects("race");
        assert!(
            objects.len() >= 20,
            "expected the live race corpus to hold records; found {}",
            objects.len()
        );
        let mut compared = 0usize;
        for (path, data) in &objects {
            let Ok(payload) = serde_json::from_value::<RaceCacheData>(data.clone()) else { continue };
            let settled = corpus_race_source_record(data).expect("payload deserialized once already");
            assert_eq!(settled.key, payload.key, "{}", path.display());
            assert_eq!(settled.name, payload.name, "{}", path.display());
            assert_eq!(settled.base_size, payload.base_size, "{}", path.display());
            assert_eq!(settled.base_move_walk, payload.base_move_walk, "{}", path.display());
            assert_eq!(settled.race_type, payload.race_type, "{}", path.display());
            assert_eq!(settled.type_tokens, payload.type_tokens, "{}", path.display());
            assert_eq!(settled.legs, payload.legs, "{}", path.display());
            assert_eq!(settled.hands, payload.hands, "{}", path.display());
            compared += 1;
        }
        assert!(compared >= 20, "compared only {compared} race records");
    }

    /// **The parity proof for the racial-trait half of cycle 14's move.**
    ///
    /// Each assertion re-runs the exact call `race_resolver` made at run time,
    /// on the exact same payload, and compares it against the field the settled
    /// record now carries. A reading that had been widened, narrowed or
    /// reordered by the move would fail here, on the real corpus, not on a
    /// fixture.
    #[test]
    fn every_live_corpus_race_trait_record_carries_the_same_values_the_token_reads_produced() {
        let objects = data_objects("race_trait");
        assert!(
            objects.len() >= 100,
            "expected the live race-trait corpus to hold records; found {}",
            objects.len()
        );
        let mut compared = 0usize;
        for (path, data) in &objects {
            let Ok(payload) = serde_json::from_value::<RaceTraitCacheData>(data.clone()) else { continue };
            let settled = corpus_race_trait_source_record(data).expect("payload deserialized once already");
            let at = path.display();

            assert_eq!(settled.key, payload.key, "{at}");
            assert_eq!(settled.name, payload.name, "{at}");
            assert_eq!(settled.race_key, payload.race_key, "{at}");
            assert_eq!(settled.category, payload.category, "{at}");
            assert_eq!(settled.type_tokens, payload.type_tokens, "{at}");
            assert_eq!(settled.is_racial_default, payload.is_racial_default, "{at}");
            assert_eq!(settled.suppressed_by_flag, payload.suppressed_by_flag, "{at}");
            assert_eq!(settled.sets_replace_flags, payload.sets_replace_flags, "{at}");
            assert_eq!(settled.description, payload.description, "{at}");
            assert_eq!(settled.source_page, payload.source_page, "{at}");

            assert_eq!(
                settled.exclusion_guard_flags,
                race_trait_tokens::exclusion_guard_flags(&payload),
                "{at}"
            );
            assert_eq!(
                settled.negated_fact_gates,
                race_trait_tokens::negated_fact_gates(&payload),
                "{at}"
            );
            assert_eq!(
                settled.declares_negated_ability_guard,
                race_trait_tokens::declares_preability_negated_guard(&payload),
                "{at}"
            );
            let grants = race_trait_tokens::automatic_ability_grants(&payload);
            assert_eq!(settled.automatic_trait_grants, grants, "{at}");
            assert_eq!(
                settled.skinwalker_change_shape_kin,
                grants
                    .iter()
                    .find_map(|g| race_trait_tokens::skinwalker_change_shape_kin(g).map(str::to_string)),
                "{at}"
            );
            assert_eq!(
                settled.positive_prefact_flag,
                race_trait_tokens::positive_prefact_flag(&payload),
                "{at}"
            );
            assert_eq!(
                settled.declared_walk_speed_ft,
                race_trait_tokens::declared_walk_speed_ft(&payload),
                "{at}"
            );
            assert_eq!(settled.declared_size, race_trait_tokens::declared_size(&payload), "{at}");
            assert_eq!(
                settled.declared_vision,
                race_trait_tokens::declared_vision_segments(&payload),
                "{at}"
            );
            assert_eq!(
                settled.declared_bonuses,
                bonus_chain_reader::declared_bonuses(&payload),
                "{at}"
            );
            assert_eq!(
                settled.template_bonus_languages,
                race_trait_tokens::declared_template_bonus_languages(&payload.raw_tokens),
                "{at}"
            );
            assert_eq!(
                settled.adopted_race_pool_suffix,
                race_trait_tokens::adopted_race_pool_suffix(&payload),
                "{at}"
            );
            compared += 1;
        }
        assert!(compared >= 100, "compared only {compared} race-trait records");
    }

    /// A `data` object that is not a race payload at all yields `None` rather
    /// than a record with empty fields — the caller must be able to tell "no
    /// record here" from "a record that states nothing".
    #[test]
    fn a_data_object_with_no_key_is_not_a_race_record() {
        let data = serde_json::json!({ "name": "Nameless" });
        assert!(corpus_race_source_record(&data).is_none());
        assert!(corpus_race_trait_source_record(&data).is_none());
    }
}
