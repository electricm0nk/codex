//! `monster`'s counterpart to `enrich_monster_ability_raw_tokens.rs` (which
//! is itself `enrich_spell_raw_tokens.rs`'s counterpart): populates
//! `data.raw_tokens` on the shipped `monster` corpus JSON records that
//! already carry a `source.kind == "lst_token"` citation but no
//! `raw_tokens` array — the exact gap `corpus_literal_sweep`'s own
//! population rule (`source.kind == "lst_token"` AND `data.raw_tokens`
//! present) leaves a record sitting outside its coverage entirely, at
//! `held` forever, for want of the one field this tool adds.
//!
//! `SD31-E6-F2-004` traced this end to end before writing this file: of
//! `monster`'s 1,242 shipped `data/corpus/*/monster/*.json` records with a
//! real `lst_token` citation, **0 carry `raw_tokens`**
//! (`python3 -c "import json,glob; print(sum(1 for f in
//! glob.glob('data/corpus/*/monster/*.json') if 'raw_tokens' in
//! json.load(open(f))['data'])")` -> `0`). `monster`'s `wiring_class` is
//! `static` for 863 of the kind's 1,270 units (`wiring_class::classify`,
//! computed straight off the raw PCGen `.lst` row); its `status` is
//! computed independently again, off `monster_chassis::MONSTER_BOOKS`'s own
//! table membership check. Neither axis reads `data/corpus/**/*.json` at
//! all. What DOES read it is `corpus_literal_sweep`, whose `--json-out`
//! report is the ONLY thing that can promote a `Static` unit's status to
//! `literal-verified` (`v06_work_inventory::apply_done_rung_stamps`), which
//! is the ONLY status that reaches `done` for a `static` unit
//! (`pf1e_dashboard_producer.doneness_verdict`). A `static` monster already
//! `grounded` moves to `done` through this single, narrow gate — never by
//! touching `monster_chassis.rs`'s own tables, never by widening the
//! classifier.
//!
//! **Byte-for-byte, not reconstructed.** Reuses `corpus_literal_sweep`'s OWN
//! `tab_tokens`/`token_closure` functions — the exact code the verifier
//! itself runs — so the tokens this tool writes and the tokens the sweep
//! later re-derives from the same citation are computed by one function,
//! not two independently-written ones that could drift apart.
//!
//! Book-agnostic by construction: walks every `data/corpus/*/monster/`
//! directory that exists on disk rather than naming a fixed book list
//! (matches `enrich_monster_ability_raw_tokens.rs`'s own shape) — a
//! `monster` unit's `status` is decided by `monster_chassis::MONSTER_BOOKS`
//! table *membership*, not by a fixed enumerable book list.
//!
//! **PI screening, wired directly into the write path, not left to a
//! post-hoc audit alone.** A monster chassis row's own `KEY:`/first-column
//! token IS the record's name, and PCGen declares some monster-shaped rows
//! (Demon Lords, Empyreal Lords, named unique creatures) `NAMEISPI:YES` —
//! confirmed real in this corpus's own oracle: `bestiary_4/b4_races.lst`
//! carries 14 such rows (Dagon, Kostchtchie, Pazuzu, Cernunnos, Korada, …),
//! `inner_sea_world_guide/iswg_races.lst`/`iswg_races_bestiary.lst` carry 5
//! more (Daughter of Urgathoa, Sandpoint Devil, Treerazer, Boar (Sargavan),
//! Herd Animal (Storval Aurochs)). None of those specific rows are among
//! today's 1,242 shipped records (verified by exact `source.path`+`line`
//! match, corpus-wide, before writing this tool) — the population this
//! tool enriches today needs no drop — but the NEXT monster ingest that
//! adds a book with a genuinely PI-declared row must not silently ship it
//! via this tool's own closure read, so the guard is built in rather than
//! assumed. Per `decisions.md §50.3` ("a key cannot be redacted"), a
//! `declared.name` hit on a shipped record's own base row DROPS the file
//! (never redacts a name in place); a `declared.description` hit or a
//! `PI_BLACKLIST_TERMS` hit on any closure field redacts that field's
//! value to [`codex::rules_core::shape_b_v1::REDACTED_PI_MARKER`] before it
//! is written into `raw_tokens`, mirroring `cache_gen::ultimate_equipment`
//! (`SD31-PI-REPAIR-001`)'s own two-contract pattern.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::corpus_literal_sweep::token_closure;
use codex::rules_core::pi_screening::{classify_field, declared_product_identity};
use codex::rules_core::shape_b_v1::REDACTED_PI_MARKER;
use codex::rules_core::wiring_class::build_mod_index;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// Every `monster` JSON under a book's `monster/` directory, walked
/// recursively (matches `enrich_monster_ability_raw_tokens.rs`'s own
/// `find_monster_ability_json_files` shape).
fn find_monster_json_files(book_dir: &Path) -> Vec<PathBuf> {
    let dir = book_dir.join("monster");
    let mut out = Vec::new();
    let mut stack = vec![dir];
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
/// here (not imported: that function is private to that binary, not part
/// of the library) rather than reinvented, so this tool and the verifier
/// that checks its output always agree about which book a citation
/// belongs to.
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
/// binary-local struct, not part of the library).
fn mod_index_for_book(data_root: &Path, book_dir: &str) -> BTreeMap<String, Vec<String>> {
    let mut book_paths = BTreeMap::new();
    book_paths.insert(book_dir.to_string(), data_root.join(book_dir));
    build_mod_index(&book_paths).into_iter().map(|((_, name), rows)| (name, rows)).collect()
}

enum Outcome {
    Enriched { redacted_fields: usize },
    DroppedPi(String),
    NoLstCitation,
    AlreadyEnriched,
    CitationMiss(String),
}

/// Split one closure field (`"COST:150"`, `"DESC:some text: with colons"`)
/// into a `{key, value}` pair on the FIRST colon. Round-trips exactly for
/// any field that carries at least one colon — every PCGen `TAG:VALUE`
/// token does, by construction of the format this closure was built from.
fn split_token_field(field: &str) -> Option<(&str, &str)> {
    field.split_once(':')
}

/// PI-screen one closure field's value: blacklist term scan
/// ([`classify_field`]) union'd with the row's own `DESCISPI:YES`
/// declaration for `DESC`-keyed fields specifically — mirrors
/// `pi_screening::classify_optional_field_declared`'s "union, never a
/// substitution" rule, applied per-token rather than to one whole-record
/// description field, since a monster chassis row's free text (if any)
/// lives inside individual closure tokens, not a single `data.description`.
fn screen_field_value(key: &str, value: &str, declared_description: bool) -> (String, bool) {
    if key.eq_ignore_ascii_case("DESC") && declared_description {
        return (REDACTED_PI_MARKER.to_string(), true);
    }
    let (license, ..) = classify_field(key, value);
    if license == codex::rules_core::shape_b_v1::License::PiRedacted {
        return (REDACTED_PI_MARKER.to_string(), true);
    }
    (value.to_string(), false)
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
    for candidate in [data_obj_ref.get("key"), data_obj_ref.get("name"), data_obj_ref.get("corpus_key")] {
        if let Some(name) = candidate.and_then(Value::as_str) {
            identities.insert(name.to_string());
        }
    }
    if let Some(record_key) = source.get("record_key").and_then(Value::as_str) {
        identities.insert(record_key.to_string());
    }

    let closure = token_closure(base_row, &identities, mod_index, None);
    if closure.is_empty() {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path}:{line}: base row carries no tab-separated fields at all -- \
             a genuinely malformed citation, not a missing token set"
        ));
    }

    let mut pairs: Vec<(&str, &str)> = Vec::with_capacity(closure.len());
    for field in &closure {
        let Some(pair) = split_token_field(field) else {
            return Outcome::CitationMiss(format!(
                "{lst_rel_path}:{line}: closure field {field:?} carries no ':' -- cannot be \
                 decomposed into a {{key,value}} pair that round-trips"
            ));
        };
        pairs.push(pair);
    }

    // `declared_product_identity` reads the WHOLE closure (base row + every
    // `.MOD` row targeting this record's own identities within the same
    // book), never just the base row alone -- a `.MOD` row can add a
    // declaration the base row never carried.
    let declared = declared_product_identity(pairs.iter().copied());
    if declared.name {
        fs::remove_file(path).unwrap_or_else(|e| panic!("remove {path:?}: {e}"));
        return Outcome::DroppedPi(format!(
            "{lst_rel_path}:{line} (record_key={:?}) declares NAMEISPI:YES in its own closure -- \
             a name cannot be redacted, dropped per decisions.md §50.3",
            source.get("record_key").and_then(Value::as_str).unwrap_or("?")
        ));
    }

    let mut raw_tokens: Vec<Value> = Vec::with_capacity(pairs.len());
    let mut redacted_fields = 0usize;
    for (key, value) in &pairs {
        let (stored, redacted) = screen_field_value(key, value, declared.description);
        if redacted {
            redacted_fields += 1;
        }
        raw_tokens.push(json!({ "key": key, "value": stored }));
    }

    let data_obj = root.get_mut("data").and_then(Value::as_object_mut).expect("checked above");
    data_obj.insert("raw_tokens".to_string(), Value::Array(raw_tokens));
    if redacted_fields > 0 {
        let root_obj = root.as_object_mut().expect("record root is an object");
        root_obj.insert("license".to_string(), json!("PI-REDACTED"));
        root_obj.insert("pi_field".to_string(), json!("raw_tokens"));
        root_obj.insert("pi_marker".to_string(), json!("redacted"));
    }

    let new_json = serde_json::to_string_pretty(&root).expect("serialize enriched record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    Outcome::Enriched { redacted_fields }
}

fn main() {
    let data_root = pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");

    let mut total_enriched = 0u32;
    let mut total_redacted_fields = 0u32;
    let mut total_dropped = 0u32;
    let mut total_no_citation = 0u32;
    let mut total_already = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut drops: Vec<String> = Vec::new();
    let mut mod_index_cache: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    let Ok(book_entries) = fs::read_dir(&corpus_root) else {
        eprintln!("enrich_monster_raw_tokens: no {corpus_root:?} directory found");
        return;
    };
    let mut book_dirs: Vec<PathBuf> = book_entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    book_dirs.sort();

    for book_dir in &book_dirs {
        let book_name = book_dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let files = find_monster_json_files(book_dir);
        if files.is_empty() {
            continue;
        }
        let mut book_enriched = 0u32;
        for file in &files {
            match enrich_one(file, &data_root, &mut mod_index_cache) {
                Outcome::Enriched { redacted_fields } => {
                    total_enriched += 1;
                    book_enriched += 1;
                    total_redacted_fields += redacted_fields as u32;
                }
                Outcome::DroppedPi(msg) => {
                    total_dropped += 1;
                    drops.push(format!("{}: {}", file.display(), msg));
                }
                Outcome::NoLstCitation => total_no_citation += 1,
                Outcome::AlreadyEnriched => total_already += 1,
                Outcome::CitationMiss(msg) => misses.push(format!("{}: {}", file.display(), msg)),
            }
        }
        eprintln!("{book_name}: {} monster files scanned, {book_enriched} enriched", files.len());
    }

    eprintln!(
        "\nenrich_monster_raw_tokens: {total_enriched} enriched ({total_redacted_fields} PI-redacted fields \
         across them), {total_dropped} dropped for NAMEISPI, {total_no_citation} no-LST-citation (untouched), \
         {total_already} already-enriched, {} citation misses",
        misses.len()
    );
    if !drops.is_empty() {
        eprintln!("\nDropped for NAMEISPI:YES (not fabricated, not shipped):");
        for drop in &drops {
            eprintln!("  {drop}");
        }
    }
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
    /// throwaway `data/corpus`-shaped monster JSON, both under
    /// `std::env::temp_dir()` and cleaned up on drop.
    struct Scratch {
        data_root: PathBuf,
        corpus_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base =
                std::env::temp_dir().join(format!("codex_enrich_monster_raw_tokens_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let data_root = base.join("pcgen_data");
            let corpus_root = base.join("data_corpus");
            fs::create_dir_all(data_root.join("pathfinder/paizo/roleplaying_game/x_book")).unwrap();
            fs::create_dir_all(corpus_root.join("x_book/monster")).unwrap();
            Scratch { data_root, corpus_root }
        }

        fn write_lst(&self, contents: &str) {
            fs::write(self.data_root.join("pathfinder/paizo/roleplaying_game/x_book/x_races.lst"), contents)
                .unwrap();
        }

        fn write_json(&self, name: &str, contents: &str) -> PathBuf {
            let path = self.corpus_root.join("x_book/monster").join(name);
            fs::write(&path, contents).unwrap();
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.data_root.parent().unwrap());
        }
    }

    // ----- split_token_field -----

    #[test]
    fn split_token_field_splits_on_the_first_colon_only() {
        assert_eq!(split_token_field("SIZE:L"), Some(("SIZE", "L")));
        assert_eq!(split_token_field("SOURCEPAGE:p.15: see errata"), Some(("SOURCEPAGE", "p.15: see errata")));
    }

    #[test]
    fn split_token_field_refuses_a_field_with_no_colon() {
        assert_eq!(split_token_field("NoColonAtAll"), None);
    }

    // ----- screen_field_value -----

    #[test]
    fn screen_field_value_passes_through_a_clean_value() {
        let (stored, redacted) = screen_field_value("SIZE", "L", false);
        assert_eq!(stored, "L");
        assert!(!redacted);
    }

    #[test]
    fn screen_field_value_redacts_a_blacklist_term_hit_on_any_key() {
        let (stored, redacted) = screen_field_value("SPECIALS", "Blessed by Iomedae", false);
        assert_eq!(stored, REDACTED_PI_MARKER);
        assert!(redacted);
    }

    #[test]
    fn screen_field_value_redacts_a_desc_field_when_description_is_declared_even_without_a_blacklist_hit() {
        let (stored, redacted) = screen_field_value("DESC", "A perfectly ordinary sentence.", true);
        assert_eq!(stored, REDACTED_PI_MARKER);
        assert!(redacted);
    }

    #[test]
    fn screen_field_value_leaves_a_non_desc_field_alone_even_when_description_is_declared() {
        let (stored, redacted) = screen_field_value("SIZE", "L", true);
        assert_eq!(stored, "L");
        assert!(!redacted);
    }

    // ----- enrich_one: the real end-to-end path against a throwaway corpus -----

    /// Mirrors the real `ankheg` record traced this cycle
    /// (`data/corpus/beastiary/monster/ankheg.json`): a `static`,
    /// `grounded` unit whose JSON carries a valid `lst_token` citation but
    /// no `raw_tokens` -- the exact shape that keeps 842 `monster` records
    /// in this corpus at `held` today.
    #[test]
    fn enrich_one_adds_the_full_token_closure_byte_for_byte() {
        let scratch = Scratch::new("basic");
        scratch.write_lst("Ankheg\tKEY:Ankheg\tSIZE:L\tCR:3\tSOURCEPAGE:p.15\n");
        let json_path = scratch.write_json(
            "ankheg.json",
            r#"{"data":{"name":"Ankheg","key":"beastiary1:monster:ankheg"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_races.lst","line":1,"record_key":"Ankheg"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched { redacted_fields: 0 }));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().expect("raw_tokens array present");
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        let expected: BTreeSet<String> =
            ["KEY:Ankheg", "SIZE:L", "CR:3", "SOURCEPAGE:p.15"].into_iter().map(str::to_string).collect();
        assert_eq!(joined, expected);
        assert_eq!(written.get("pi_field"), None, "a clean record must not gain a pi_field it never earned");
    }

    #[test]
    fn enrich_one_includes_a_mod_rows_tokens_in_the_closure() {
        let scratch = Scratch::new("modrow");
        scratch.write_lst("Ankheg\tKEY:Ankheg\tSIZE:L\nAnkheg.MOD\tCR:4\n");
        let json_path = scratch.write_json(
            "ankheg.json",
            r#"{"data":{"name":"Ankheg","key":"beastiary1:monster:ankheg"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_races.lst","line":1,"record_key":"Ankheg"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched { .. }));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        assert!(joined.contains("CR:4"), "the .MOD row's own CR token must appear in the enriched raw_tokens");
    }

    /// MUTATION PROOF for the NAMEISPI drop path: a monster row declaring
    /// `NAMEISPI:YES` (the real `bestiary_4` Demon Lord shape, e.g. Dagon)
    /// must never ship its name -- the file is dropped, not redacted.
    #[test]
    fn enrich_one_drops_a_record_whose_base_row_declares_nameispi() {
        let scratch = Scratch::new("nameispi");
        scratch.write_lst("Dagon\tKEY:Demon Lord (Dagon)\tNAMEISPI:YES\tSIZE:H\tCR:28\n");
        let json_path = scratch.write_json(
            "demon_lord_dagon.json",
            r#"{"data":{"name":"Demon Lord (Dagon)","key":"bestiary_4:monster:demon_lord_dagon"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_races.lst","line":1,"record_key":"Demon Lord (Dagon)"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::DroppedPi(_)), "expected a drop, not an enrich");
        assert!(!json_path.exists(), "a NAMEISPI:YES record must be removed from disk, never shipped");
    }

    /// The same proof, but the declaration arrives via a `.MOD` row rather
    /// than the base row -- `declared_product_identity` must read the
    /// WHOLE closure, not just the cited line.
    #[test]
    fn enrich_one_drops_a_record_whose_mod_row_declares_nameispi() {
        let scratch = Scratch::new("nameispi_mod");
        scratch.write_lst("Dagon\tKEY:Demon Lord (Dagon)\tSIZE:H\nDemon Lord (Dagon).MOD\tNAMEISPI:YES\n");
        let json_path = scratch.write_json(
            "demon_lord_dagon.json",
            r#"{"data":{"name":"Demon Lord (Dagon)","key":"bestiary_4:monster:demon_lord_dagon"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_races.lst","line":1,"record_key":"Demon Lord (Dagon)"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::DroppedPi(_)));
        assert!(!json_path.exists());
    }

    /// MUTATION PROOF for the blacklist redaction path: a closure token
    /// whose value contains a `PI_BLACKLIST_TERMS` hit is redacted in the
    /// WRITTEN `raw_tokens`, not shipped as prose.
    #[test]
    fn enrich_one_redacts_a_blacklist_term_hit_anywhere_in_the_closure() {
        let scratch = Scratch::new("blacklist");
        scratch.write_lst("Herald\tKEY:Herald\tSPECIALS:Blessed by Iomedae\tSIZE:M\n");
        let json_path = scratch.write_json(
            "herald.json",
            r#"{"data":{"name":"Herald","key":"x_book:monster:herald"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_races.lst","line":1,"record_key":"Herald"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched { redacted_fields: 1 }));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let specials = tokens.iter().find(|t| t["key"] == "SPECIALS").unwrap();
        assert_eq!(specials["value"], REDACTED_PI_MARKER, "the deity-name hit must not ship verbatim");
        assert_eq!(written["license"], "PI-REDACTED");
        assert_eq!(written["pi_field"], "raw_tokens");
        // The clean SIZE token must still ship untouched.
        let size = tokens.iter().find(|t| t["key"] == "SIZE").unwrap();
        assert_eq!(size["value"], "M");
    }

    #[test]
    fn enrich_one_leaves_an_already_enriched_record_untouched() {
        let scratch = Scratch::new("already");
        scratch.write_lst("Ankheg\tSIZE:L\n");
        let json_path = scratch.write_json(
            "ankheg.json",
            r#"{"data":{"key":"K","raw_tokens":[{"key":"SIZE","value":"L"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_races.lst","line":1}}"#,
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
        scratch.write_lst("Ankheg\tSIZE:L\n");
        let json_path = scratch.write_json(
            "ghost.json",
            r#"{"data":{"key":"Ghost Monster"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_races.lst","line":99}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::CitationMiss(_)));
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        assert!(after["data"].get("raw_tokens").is_none());
    }

    #[test]
    fn enrich_one_skips_a_non_lst_token_source_without_error() {
        let scratch = Scratch::new("nonlst");
        let json_path = scratch.write_json(
            "web.json",
            r#"{"data":{"key":"Web Second Source Monster"},"source":{"kind":"web_second_source"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::NoLstCitation));
    }

    #[test]
    fn find_monster_json_files_finds_flat_files() {
        let scratch = Scratch::new("flatscan");
        scratch.write_json("a.json", "{}");
        scratch.write_json("b.json", "{}");
        let found = find_monster_json_files(&scratch.corpus_root.join("x_book"));
        assert_eq!(found.len(), 2);
    }
}
