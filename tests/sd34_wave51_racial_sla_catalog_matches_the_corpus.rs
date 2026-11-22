//! SD-34 wave 51's corpus fixture gate for `rules_core::racial_sla`.
//!
//! `racial_sla::RACIAL_SLA_CATALOG` is a transcription of 115 real
//! `data/corpus/core_rulebook/race_trait_generic/racial_sla_*.json` records.
//! A transcription can be wrong in exactly two ways, and this file checks
//! both against the live corpus bytes rather than against the catalog itself:
//!
//! 1. **A transcribed value drifted.** Every entry's `spell_level`,
//!    `variable_slug` and `upstream_line` must be what its own corpus record
//!    states right now, and every entry's record must still carry the exact
//!    five-token `BONUS:VAR` chain the module doc tabulates -- including the
//!    `DC` formula the module's own shared
//!    `RACIAL_SLA_SAVE_DC_FORMULA` is the reduced form of.
//! 2. **The catalog's MEMBERSHIP drifted.** Every corpus record carrying that
//!    full chain must be IN the catalog (so a newly-ingested SLA cannot
//!    silently sit uncovered), and the three records the module deliberately
//!    excludes must still be exactly the ones lacking the chain (so the
//!    exclusion cannot silently grow).
//!
//! Reading the corpus from an integration test rather than `include_str!`ing
//! 115 files is deliberate: the membership half of the check is only
//! meaningful if it sees every file in the directory, including one this
//! cycle never knew about.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use codex::rules_core::racial_sla::{
    RACIAL_SLA_CATALOG, RACIAL_SLA_SAVE_DC_FORMULA, RACIAL_SLA_UPSTREAM_LST,
};

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data/corpus/core_rulebook/race_trait_generic")
}

/// One `Racial SLA ~ *` corpus record, reduced to exactly what the catalog
/// claims about it.
struct CorpusSla {
    variable_slug: String,
    /// `<var suffix> -> <BONUS:VAR amount>` for every `BONUS:VAR` token whose
    /// target is one of this record's own `RacialSLA_<slug>_*` variables.
    bonus_vars: BTreeMap<String, String>,
    source_line: u64,
    source_path: String,
}

fn read_racial_sla_records() -> BTreeMap<String, CorpusSla> {
    let mut out = BTreeMap::new();
    for entry in std::fs::read_dir(corpus_dir()).expect("race_trait_generic corpus dir exists") {
        let path = entry.expect("readable dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("readable corpus record");
        let value: serde_json::Value = serde_json::from_str(&text).expect("valid corpus JSON");
        let key = value
            .pointer("/data/key")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned();
        if !key.starts_with("Racial SLA ~ ") {
            continue;
        }
        let tokens = value
            .pointer("/data/raw_tokens")
            .and_then(|v| v.as_array())
            .expect("every corpus record carries raw_tokens")
            .clone();

        // The slug is the record's own `DEFINE:RacialSLA_<slug>_LVL|0` token,
        // read from the corpus rather than derived from the key -- the key's
        // own punctuation ("Speak with Animals (rodents only)") is not the
        // slug.
        let mut variable_slug = None;
        for token in &tokens {
            let (k, v) = token_pair(token);
            if k == "DEFINE"
                && let Some(name) = v.split('|').next()
                && let Some(rest) = name.strip_prefix("RacialSLA_")
                && let Some(slug) = rest.strip_suffix("_LVL")
            {
                variable_slug = Some(slug.to_owned());
            }
        }
        let Some(variable_slug) = variable_slug else { continue };

        let mut bonus_vars = BTreeMap::new();
        for token in &tokens {
            let (k, v) = token_pair(token);
            if k != "BONUS" || !v.starts_with("VAR|") {
                continue;
            }
            let parts: Vec<&str> = v.split('|').collect();
            if parts.len() < 3 {
                continue;
            }
            let prefix = format!("RacialSLA_{variable_slug}_");
            if let Some(suffix) = parts[1].strip_prefix(prefix.as_str()) {
                bonus_vars.insert(suffix.to_owned(), parts[2].to_owned());
            }
        }

        out.insert(
            key,
            CorpusSla {
                variable_slug,
                bonus_vars,
                source_line: value
                    .pointer("/source/line")
                    .and_then(|v| v.as_u64())
                    .expect("every corpus record carries a source line"),
                source_path: value
                    .pointer("/source/path")
                    .and_then(|v| v.as_str())
                    .expect("every corpus record carries a source path")
                    .to_owned(),
            },
        );
    }
    out
}

fn token_pair(token: &serde_json::Value) -> (&str, &str) {
    (
        token.get("key").and_then(|v| v.as_str()).unwrap_or_default(),
        token.get("value").and_then(|v| v.as_str()).unwrap_or_default(),
    )
}

/// Whether this record carries the full five-token `BONUS:VAR` chain
/// `racial_sla`'s module doc tabulates. Written out in full here, against the
/// record's OWN slug, rather than reusing any helper the module under test
/// exports -- the transcription is only checked if the check is independent.
fn carries_the_full_chain(record: &CorpusSla) -> bool {
    let slug = &record.variable_slug;
    record.bonus_vars.get("LVL").map(String::as_str) == Some("TL")
        && record.bonus_vars.get("Times").map(String::as_str) == Some("1")
        && record.bonus_vars.get("DCMod").map(String::as_str) == Some("CHA")
        && record.bonus_vars.get("DC").map(String::as_str)
            == Some(format!("10+RacialSLA_{slug}_SpellLVL+RacialSLA_{slug}_DCMod").as_str())
        && record.bonus_vars.contains_key("SpellLVL")
}

#[test]
fn every_catalog_entry_matches_its_own_corpus_record() {
    let corpus = read_racial_sla_records();
    for entry in RACIAL_SLA_CATALOG {
        let record = corpus.get(entry.corpus_key).unwrap_or_else(|| {
            panic!("catalog names {:?}, which no corpus record carries", entry.corpus_key)
        });
        assert_eq!(record.variable_slug, entry.variable_slug, "{:?}", entry.corpus_key);
        assert!(carries_the_full_chain(record), "{:?} lost its BONUS:VAR chain", entry.corpus_key);
        assert_eq!(
            record.bonus_vars.get("SpellLVL").map(String::as_str),
            Some(entry.spell_level.to_string().as_str()),
            "{:?} spell level drifted",
            entry.corpus_key
        );
        assert_eq!(record.source_line, entry.upstream_line, "{:?} line drifted", entry.corpus_key);
        assert_eq!(record.source_path, RACIAL_SLA_UPSTREAM_LST, "{:?}", entry.corpus_key);
    }
}

#[test]
fn the_catalog_covers_every_corpus_record_carrying_the_full_chain() {
    let corpus = read_racial_sla_records();
    let catalogued: BTreeSet<&str> = RACIAL_SLA_CATALOG.iter().map(|e| e.corpus_key).collect();
    let with_chain: BTreeSet<&str> = corpus
        .iter()
        .filter(|(_, record)| carries_the_full_chain(record))
        .map(|(key, _)| key.as_str())
        .collect();
    assert_eq!(
        with_chain, catalogued,
        "a Racial SLA record carrying the full BONUS:VAR chain is not in RACIAL_SLA_CATALOG (or \
         vice versa) -- re-derive the catalog from the corpus rather than editing this assertion"
    );
    assert_eq!(catalogued.len(), 115);
}

#[test]
fn exactly_three_records_are_excluded_and_they_are_the_ones_without_the_chain() {
    let corpus = read_racial_sla_records();
    let without_chain: BTreeSet<&str> = corpus
        .iter()
        .filter(|(_, record)| !carries_the_full_chain(record))
        .map(|(key, _)| key.as_str())
        .collect();
    let expected: BTreeSet<&str> = ["Racial SLA ~ Dispel Magic", "Racial SLA ~ Divine Favor", "Racial SLA ~ Suggestion"]
        .into_iter()
        .collect();
    assert_eq!(
        without_chain, expected,
        "the set of Racial SLA records lacking the BONUS:VAR chain changed -- racial_sla's module \
         doc names exactly these three and explains why guessing a DC for them would be wrong"
    );
    assert_eq!(corpus.len(), 118, "core_rulebook's ingested Racial SLA population changed");
}

#[test]
fn the_shared_save_dc_formula_is_the_reduced_form_of_every_records_own_dc_token() {
    // The catalog's shared formula substitutes each record's own two bound
    // variables into its `DC` token: `SpellLVL` becomes the catalog's
    // `spell_level`, and `DCMod` becomes `CHA` (from that record's own
    // `BONUS:VAR|RacialSLA_<slug>_DCMod|CHA|TYPE=Base`). Checked here by
    // rebuilding the reduced form from each record's own bytes.
    let corpus = read_racial_sla_records();
    for entry in RACIAL_SLA_CATALOG {
        let record = &corpus[entry.corpus_key];
        let slug = &record.variable_slug;
        let raw = record.bonus_vars.get("DC").expect("a chained record has a DC token");
        let reduced = raw
            .replace(&format!("RacialSLA_{slug}_DCMod"), record.bonus_vars["DCMod"].as_str())
            .replace(&format!("RacialSLA_{slug}_SpellLVL"), "SpellLVL");
        assert_eq!(reduced, RACIAL_SLA_SAVE_DC_FORMULA, "{:?}", entry.corpus_key);
    }
}
