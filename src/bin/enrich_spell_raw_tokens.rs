//! `spell`'s counterpart to `enrich_equipment_raw_tokens.rs`: populates
//! `data.raw_tokens` on the shipped spell corpus JSON records that already
//! carry a `source.kind == "lst_token"` citation but no `raw_tokens` array —
//! the exact gap `corpus_literal_sweep`'s own population rule
//! (`source.kind == "lst_token"` AND `data.raw_tokens` present) leaves a
//! record sitting outside its coverage entirely, at `held` forever, for want
//! of the one field this tool adds.
//!
//! SD31-E6-F2 traced this end to end before writing this file: a `spell`
//! unit's `wiring_class` (`static`/`derived`/…) is computed independently,
//! straight off the raw PCGen `.lst` row (`wiring_class::classify`); its
//! `status` is computed independently again, off the engine's own
//! `spell_resolver::spell_catalog_rows()` table. Neither axis reads
//! `data/corpus/**/*.json` at all. What DOES read it is
//! `corpus_literal_sweep`, whose `--json-out` report is the ONLY thing that
//! can promote a `wiring_class == Static` unit's status to
//! `literal-verified` (`v06_work_inventory::apply_done_rung_stamps`), which
//! is the ONLY status that reaches `done` for a `static` unit
//! (`pf1e_dashboard_producer.doneness_verdict`). A `static` spell already
//! `held` at `ingested-magnitude`/`grounded`/`text-complete` moves to `done`
//! through this single, narrow gate — never by widening the classifier, never
//! by touching the engine's spell tables.
//!
//! **Byte-for-byte, not reconstructed.** This tool does not re-parse the
//! spell row into named fields and re-serialize them (that is what
//! `pcgen_import::lst_parser::spell` is for, and it is deliberately NOT used
//! here — see `enrich_equipment_raw_tokens.rs`'s own doc comment on why a
//! typed re-serialization silently drops fields the struct does not know
//! about). It reuses `corpus_literal_sweep`'s OWN `tab_tokens`/`token_closure`
//! functions — the exact code the verifier itself runs — so the tokens this
//! tool writes and the tokens the sweep later re-derives from the same
//! citation are computed by one function, not two independently-written
//! ones that could drift apart.
//!
//! Scoped to the spell books the engine's `spell_resolver` actually models
//! (`spell_book_slug_for`'s corpus slugs) — enriching an unchained book's
//! spell records would add real, correct data that no `wiring_class`/
//! `status` path can ever promote to `done`, since that book is absent
//! from `spell_catalog_rows()` entirely; narrower scope, same reason
//! `enrich_equipment_raw_tokens.rs` names its own six books rather than
//! walking `data/corpus/*` blindly.
//!
//! Widened 5 -> 8 by `SD31-E6-F2-005`: `ultimate_magic`, `occult_adventures`
//! and `ultimate_combat` had no `data/corpus/<book>/spell/*.json` at all
//! until `cache_gen::spell_lane_dump`/`gen_cache_spell_lane_dump` (this
//! same cycle) generated one (`OPEN-ISSUES.md` rows 76/77's named missing
//! piece) — this list is widened in the SAME cycle that made the
//! directories exist, so this tool's own "carries a source.kind but no
//! raw_tokens" scan no longer reads as permanently empty for them.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::corpus_literal_sweep::token_closure;
use codex::rules_core::pi_screening::{self, declared_product_identity};
use codex::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};
use codex::rules_core::wiring_class::build_mod_index;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// The corpus book directories `spell_resolver::spell_catalog_rows()`
/// models (`spell_book_slug_for` in `v06_work_inventory.rs`) — the only
/// books whose `static` spell units this enrichment can ever move to `done`.
/// Widened 5 -> 8 by `SD31-E6-F2-005`: `ultimate_magic`/`occult_adventures`/
/// `ultimate_combat` now have a `data/corpus/<book>/spell/*.json` cache
/// (`cache_gen::spell_lane_dump`, same cycle) for this tool to enrich.
const TARGET_BOOKS: &[&str] = &[
    "core_rulebook",
    "advanced_players_guide",
    "advanced_class_guide",
    "advanced_race_guide",
    "ultimate_intrigue",
    "ultimate_magic",
    "occult_adventures",
    "ultimate_combat",
];

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// Every `spell` JSON under a book's `spell/` directory, walked recursively —
/// `core_rulebook`'s own corpus nests one subdirectory per spell level
/// (`spell/level_0/`, `spell/level_1/`, …), a shape a single-level `read_dir`
/// silently under-reports (0 of ~380 real files) rather than errors on, the
/// same one-level-join hazard `OPEN-ISSUES.md` row 1 already named for
/// `wiring_class::CorpusLines::line()`.
fn find_spell_json_files(book_dir: &Path) -> Vec<PathBuf> {
    let spell_dir = book_dir.join("spell");
    let mut out = Vec::new();
    let mut stack = vec![spell_dir];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// The corpus-relative book directory a `source.path` citation belongs to:
/// its first four path segments (`<system>/<publisher>/<line>/<book>`) —
/// byte-identical to `corpus_literal_sweep`'s own `book_dir_of`, duplicated
/// here (not imported: that function is private to that binary, not part of
/// the library) rather than reinvented, so this tool and the verifier that
/// checks its output always agree about which book a citation belongs to.
fn book_dir_of(source_path: &str) -> Option<String> {
    let segments: Vec<&str> = source_path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() < 5 {
        return None;
    }
    Some(segments[..4].join("/"))
}

/// One book's `.MOD` rows, keyed by the record name they target — the same
/// derivation `corpus_literal_sweep`'s own `Sweep::mod_index` performs,
/// duplicated here (not imported: that method is a private impl on a
/// binary-local struct, not part of the library) at the single-book-map call
/// site `build_mod_index` itself documents as its normal shape. `book_dir` is
/// the FULL corpus-relative directory ([`book_dir_of`]'s return), never the
/// short book slug -- the two differ (`pathfinder/paizo/roleplaying_game/
/// core_rulebook` vs `core_rulebook`), and a mismatch here silently builds an
/// empty index, which the earlier draft of this tool caught as a failing
/// test rather than a passing one.
fn mod_index_for_book(data_root: &Path, book_dir: &str) -> BTreeMap<String, Vec<String>> {
    let mut book_paths = BTreeMap::new();
    book_paths.insert(book_dir.to_string(), data_root.join(book_dir));
    build_mod_index(&book_paths).into_iter().map(|((_, name), rows)| (name, rows)).collect()
}

enum Outcome {
    Enriched,
    NoLstCitation,
    AlreadyEnriched,
    CitationMiss(String),
    /// SD-30 §53.5 hard-stop: the corpus row itself declares
    /// `NAMEISPI:YES`. A record's name cannot be redacted (it is half of
    /// its key), so this tool refuses to publish `raw_tokens` for it at
    /// all -- mirroring `cache_gen::spell_lane_dump`'s own name-PI drop,
    /// applied here to the enrichment write path rather than the initial
    /// generation path.
    NameIsProductIdentity,
}

/// Split one closure field (`"COST:150"`, `"DESC:some text: with colons"`)
/// into a `{key, value}` pair on the FIRST colon. Round-trips exactly:
/// `format!("{key}:{value}")` always reconstructs the original field
/// (`corpus_literal_sweep::ShippedToken::joined`), for any field that
/// contains at least one colon — every PCGen `TAG:VALUE` token does, by
/// construction of the format this closure was built from.
fn split_token_field(field: &str) -> Option<(&str, &str)> {
    field.split_once(':')
}

fn enrich_one(
    path: &Path,
    data_root: &Path,
    mod_index_cache: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>,
) -> Outcome {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    {
        let data = root.get("data").unwrap_or_else(|| panic!("{path:?}: no top-level \"data\" object"));
        if data.get("raw_tokens").is_some() {
            return Outcome::AlreadyEnriched;
        }
    }

    let source = root["source"].clone();
    if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
        return Outcome::NoLstCitation;
    }
    let lst_rel_path = source["path"].as_str().expect("lst_token source must carry a path").to_string();
    let line = source["line"].as_u64().expect("lst_token source must carry a line") as usize;
    let Some(book_dir) = book_dir_of(&lst_rel_path) else {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path} is not <system>/<publisher>/<line>/<book>/<file>-shaped"
        ));
    };
    let mod_index = mod_index_cache
        .entry(book_dir.clone())
        .or_insert_with(|| mod_index_for_book(data_root, &book_dir));

    let lst_full_path = data_root.join(&lst_rel_path);
    let Ok(lst_text) = fs::read_to_string(&lst_full_path) else {
        return Outcome::CitationMiss(format!("cited LST file not found: {lst_full_path:?}"));
    };
    let lines: Vec<&str> = lst_text.split('\n').collect();
    if line == 0 || line > lines.len() {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path} has {} lines, record claims line {line}",
            lines.len()
        ));
    }
    let base_row = lines[line - 1];

    let data_obj_ref = root.get("data").and_then(Value::as_object).expect("checked above");
    let mut identities: BTreeSet<String> = BTreeSet::new();
    for candidate in [data_obj_ref.get("key"), data_obj_ref.get("name")] {
        if let Some(name) = candidate.and_then(Value::as_str) {
            identities.insert(name.to_string());
        }
    }
    if let Some(record_key) = source.get("record_key").and_then(Value::as_str) {
        identities.insert(record_key.to_string());
    }

    let closure = token_closure(base_row, &identities, mod_index);
    if closure.is_empty() {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path}:{line}: base row carries no tab-separated fields at all -- \
             a genuinely malformed citation, not a missing token set"
        ));
    }

    let mut fields: Vec<(&str, &str)> = Vec::with_capacity(closure.len());
    for field in &closure {
        let Some((key, value)) = split_token_field(field) else {
            return Outcome::CitationMiss(format!(
                "{lst_rel_path}:{line}: closure field {field:?} carries no ':' -- cannot be \
                 decomposed into a {{key,value}} pair that round-trips"
            ));
        };
        fields.push((key, value));
    }

    // SD-30 PI screen, both invocation contracts, unioned -- applied to
    // `raw_tokens` specifically (the gap named against this generator: the
    // only other spell write path, `cache_gen::spell_lane_dump`, screens
    // NAME and DESCRIPTION but never the raw closure this tool writes).
    // Contract 53.5: the row's own `NAMEISPI:YES`/`DESCISPI:YES` declaration,
    // read off the SAME closure fields being written, not re-fetched.
    // Contract 52.3: the shared blacklist term scan, run per-field so a hit
    // in one field (e.g. `DESC`) does not force blocking the whole record.
    let declared = declared_product_identity(fields.iter().copied());
    if declared.name {
        // A name-PI record's identity itself cannot be redacted -- refuse
        // to publish `raw_tokens` for it at all, hard stop before write.
        return Outcome::NameIsProductIdentity;
    }

    let mut raw_tokens: Vec<Value> = Vec::with_capacity(fields.len());
    for (key, value) in &fields {
        let key_upper = key.to_ascii_uppercase();
        let is_desc_field = key_upper == "DESC" || key_upper == "BENEFIT" || key_upper == "SPECIAL";
        let (blacklist_license, ..) = pi_screening::classify_field(key, value);
        let blacklisted = blacklist_license != License::Ogl;
        let redact = blacklisted || (declared.description && is_desc_field);
        let stored_value = if redact { REDACTED_PI_MARKER } else { *value };
        raw_tokens.push(json!({ "key": key, "value": stored_value }));
    }

    let data_obj = root.get_mut("data").and_then(Value::as_object_mut).expect("checked above");
    data_obj.insert("raw_tokens".to_string(), Value::Array(raw_tokens));

    let new_json = serde_json::to_string_pretty(&root).expect("serialize enriched record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    Outcome::Enriched
}

fn main() {
    let data_root = pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");

    let mut total_enriched = 0u32;
    let mut total_no_citation = 0u32;
    let mut total_already = 0u32;
    let mut total_name_pi_blocked = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut mod_index_cache: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    for book in TARGET_BOOKS {
        let book_dir = corpus_root.join(book);
        if !book_dir.is_dir() {
            continue;
        }
        let files = find_spell_json_files(&book_dir);
        let mut book_enriched = 0u32;
        for file in &files {
            match enrich_one(file, &data_root, &mut mod_index_cache) {
                Outcome::Enriched => {
                    total_enriched += 1;
                    book_enriched += 1;
                }
                Outcome::NoLstCitation => total_no_citation += 1,
                Outcome::AlreadyEnriched => total_already += 1,
                Outcome::NameIsProductIdentity => total_name_pi_blocked += 1,
                Outcome::CitationMiss(msg) => misses.push(format!("{}: {}", file.display(), msg)),
            }
        }
        eprintln!("{book}: {} spell files scanned, {book_enriched} enriched", files.len());
    }

    eprintln!(
        "\nenrich_spell_raw_tokens: {total_enriched} enriched, {total_no_citation} no-LST-citation (untouched), {total_already} already-enriched, {total_name_pi_blocked} name-PI-blocked, {} citation misses",
        misses.len()
    );
    if !misses.is_empty() {
        eprintln!("\nCitation misses (not enriched, real gaps to investigate):");
        for miss in &misses {
            eprintln!("  {miss}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway `PCGEN_CORPUS_ROOT`-shaped book directory plus a
    /// throwaway `data/corpus`-shaped spell JSON, both under
    /// `std::env::temp_dir()` and cleaned up on drop -- the same pattern
    /// `v06_work_inventory.rs`'s own `ScratchBook`/`ScratchSpellBook` test
    /// fixtures use, not a new one invented here.
    struct Scratch {
        data_root: PathBuf,
        corpus_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_enrich_spell_raw_tokens_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let data_root = base.join("pcgen_data");
            let corpus_root = base.join("data_corpus");
            fs::create_dir_all(data_root.join("pathfinder/paizo/roleplaying_game/x_book")).unwrap();
            fs::create_dir_all(corpus_root.join("x_book/spell")).unwrap();
            Scratch { data_root, corpus_root }
        }

        fn write_lst(&self, contents: &str) {
            fs::write(
                self.data_root.join("pathfinder/paizo/roleplaying_game/x_book/x_spells.lst"),
                contents,
            )
            .unwrap();
        }

        fn write_json(&self, name: &str, contents: &str) -> PathBuf {
            let path = self.corpus_root.join("x_book/spell").join(name);
            fs::write(&path, contents).unwrap();
            path
        }

        /// A record nested one level under `spell/`, the `core_rulebook`
        /// corpus shape (`spell/level_0/…json`) a single-level `read_dir`
        /// silently misses entirely.
        fn write_json_nested(&self, subdir: &str, name: &str, contents: &str) -> PathBuf {
            let dir = self.corpus_root.join("x_book/spell").join(subdir);
            fs::create_dir_all(&dir).unwrap();
            let path = dir.join(name);
            fs::write(&path, contents).unwrap();
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.data_root.parent().unwrap());
        }
    }

    // ----- split_token_field: the round-trip the whole tool depends on -----

    #[test]
    fn split_token_field_splits_on_the_first_colon_only() {
        assert_eq!(split_token_field("COST:150"), Some(("COST", "150")));
        assert_eq!(
            split_token_field("DESC:You place a curse: it triggers later."),
            Some(("DESC", "You place a curse: it triggers later."))
        );
    }

    #[test]
    fn split_token_field_every_result_reconstructs_the_original_field() {
        for field in ["SCHOOL:Transmutation", "CLASSES:Wizard=1|Sorcerer=1", "COMPS:V, S, M"] {
            let (key, value) = split_token_field(field).unwrap();
            assert_eq!(format!("{key}:{value}"), field);
        }
    }

    #[test]
    fn split_token_field_refuses_a_field_with_no_colon() {
        assert_eq!(split_token_field("NoColonAtAll"), None);
    }

    // ----- find_spell_json_files: the recursive-scan fix, pinned -----

    /// Caught live against `core_rulebook`'s real corpus before this test
    /// existed: a single-level `read_dir` reported "0 spell files scanned"
    /// for a book whose `spell/` directory nests one subdirectory per spell
    /// level and therefore contains zero files directly in `spell/` itself.
    #[test]
    fn find_spell_json_files_walks_into_level_subdirectories() {
        let scratch = Scratch::new("nested_scan");
        scratch.write_json_nested("level_0", "acid_splash.json", "{}");
        scratch.write_json_nested("level_4", "curse_of_burning_sleep.json", "{}");
        let found = find_spell_json_files(&scratch.corpus_root.join("x_book"));
        let names: BTreeSet<String> = found
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            ["acid_splash.json".to_string(), "curse_of_burning_sleep.json".to_string()]
                .into_iter()
                .collect(),
            "both nested files must be found, not just files directly under spell/"
        );
    }

    // ----- enrich_one: the real end-to-end path against a throwaway corpus -----

    #[test]
    fn enrich_one_adds_the_full_token_closure_byte_for_byte() {
        let scratch = Scratch::new("basic");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\tCLASSES:Bloodrager=1\tDESC:Elongated blade.\n");
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let mut cache = BTreeMap::new();
        // Signature test note: `enrich_one`'s `data_root` parameter is the raw
        // PCGen checkout root, matching `pcgen_data_root()`'s real return
        // value -- the scratch fixture keeps the same two-root split so this
        // test cannot pass by accident of the two roots coinciding.
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched));

        let written: Value =
            serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().expect("raw_tokens array present");
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        let expected: BTreeSet<String> = [
            "SCHOOL:Transmutation",
            "CLASSES:Bloodrager=1",
            "DESC:Elongated blade.",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        assert_eq!(joined, expected);

        // The round-trip this whole tool exists to satisfy: re-deriving the
        // closure independently (as `corpus_literal_sweep` itself would) and
        // checking every written token is a member of it.
        let base_row = fs::read_to_string(
            scratch.data_root.join("pathfinder/paizo/roleplaying_game/x_book/x_spells.lst"),
        )
        .unwrap();
        let mod_index =
            mod_index_for_book(&scratch.data_root, "pathfinder/paizo/roleplaying_game/x_book");
        let closure = token_closure(
            base_row.split('\n').next().unwrap(),
            &["Blade Lash".to_string()].into_iter().collect(),
            &mod_index,
        );
        for field in &joined {
            assert!(closure.contains(field), "{field} must be a member of the independently-derived closure");
        }
    }

    #[test]
    fn enrich_one_includes_a_mod_rows_tokens_in_the_closure() {
        let scratch = Scratch::new("modrow");
        scratch.write_lst(
            "Blade Lash\tSCHOOL:Transmutation\tCLASSES:Bloodrager=1\nBlade Lash.MOD\tDESC:Use like a whip.\n",
        );
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched));

        let written: Value =
            serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        assert!(
            joined.contains("DESC:Use like a whip."),
            "the .MOD row's own DESC token must appear in the enriched raw_tokens, not just the base row's"
        );
    }

    #[test]
    fn enrich_one_leaves_an_already_enriched_record_untouched() {
        let scratch = Scratch::new("already");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\n");
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash","raw_tokens":[{"key":"SCHOOL","value":"Transmutation"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let before = fs::read_to_string(&json_path).unwrap();
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::AlreadyEnriched));
        assert_eq!(fs::read_to_string(&json_path).unwrap(), before, "already-enriched records must not be rewritten");
    }

    #[test]
    fn enrich_one_reports_a_citation_miss_rather_than_inventing_tokens_for_a_missing_line() {
        let scratch = Scratch::new("miss");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\n");
        let json_path = scratch.write_json(
            "ghost.json",
            r#"{"data":{"key":"Ghost Spell"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":99}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::CitationMiss(_)));
        assert!(!json_path.exists() || {
            let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
            after["data"].get("raw_tokens").is_none()
        });
    }

    // ----- PI screen on raw_tokens: the SD-30 hard-stop, both contracts -----

    #[test]
    fn enrich_one_hard_stops_when_the_row_declares_nameispi_yes() {
        let scratch = Scratch::new("name_pi");
        scratch.write_lst("Secret Rite\tNAMEISPI:YES\tSCHOOL:Transmutation\tDESC:A rite.\n");
        let json_path = scratch.write_json(
            "secret_rite.json",
            r#"{"data":{"key":"Secret Rite"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::NameIsProductIdentity));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        assert!(
            after["data"].get("raw_tokens").is_none(),
            "a NAMEISPI:YES row must never get raw_tokens written"
        );
    }

    #[test]
    fn enrich_one_redacts_a_raw_token_whose_value_hits_the_blacklist() {
        let scratch = Scratch::new("blacklist_hit");
        scratch.write_lst(
            "Iomedae's Blessing\tSCHOOL:Evocation\tDESC:A blessing granted by Iomedae herself.\n",
        );
        let json_path = scratch.write_json(
            "iomedaes_blessing.json",
            r#"{"data":{"key":"Iomedae's Blessing"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = after["data"]["raw_tokens"].as_array().unwrap();
        let desc_value = tokens
            .iter()
            .find(|t| t["key"].as_str() == Some("DESC"))
            .and_then(|t| t["value"].as_str())
            .unwrap();
        assert_eq!(
            desc_value, "[redacted PI]",
            "a raw_tokens value carrying a blacklisted term must be redacted, not shipped verbatim"
        );
        let school_value = tokens
            .iter()
            .find(|t| t["key"].as_str() == Some("SCHOOL"))
            .and_then(|t| t["value"].as_str())
            .unwrap();
        assert_eq!(school_value, "Evocation", "a clean field must ship unredacted");
    }

    #[test]
    fn enrich_one_redacts_a_desc_field_when_the_row_declares_descispi_yes() {
        let scratch = Scratch::new("desc_pi");
        scratch.write_lst("Hidden Ward\tDESCISPI:YES\tSCHOOL:Abjuration\tDESC:A ward of great secrecy.\n");
        let json_path = scratch.write_json(
            "hidden_ward.json",
            r#"{"data":{"key":"Hidden Ward"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = after["data"]["raw_tokens"].as_array().unwrap();
        let desc_value = tokens
            .iter()
            .find(|t| t["key"].as_str() == Some("DESC"))
            .and_then(|t| t["value"].as_str())
            .unwrap();
        assert_eq!(desc_value, "[redacted PI]", "a DESCISPI:YES-declared row's DESC token must be redacted");
    }

    #[test]
    fn enrich_one_skips_a_non_lst_token_source_without_error() {
        let scratch = Scratch::new("nonlst");
        let json_path = scratch.write_json(
            "web.json",
            r#"{"data":{"key":"Web Second Source Spell"},"source":{"kind":"web_second_source"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::NoLstCitation));
    }
}
