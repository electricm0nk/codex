//! `monster_ability`'s counterpart to `enrich_spell_raw_tokens.rs`: populates
//! `data.raw_tokens` on the shipped `monster_ability` corpus JSON records that
//! already carry a `source.kind == "lst_token"` citation but no `raw_tokens`
//! array — the exact gap `corpus_literal_sweep`'s own population rule
//! (`source.kind == "lst_token"` AND `data.raw_tokens` present) leaves a
//! record sitting outside its coverage entirely, at `held` forever, for want
//! of the one field this tool adds.
//!
//! SD31-E6-F9-001 traced this end to end before writing this file: of
//! `monster_ability`'s 1,629 shipped `data/corpus/*/monster_ability/*.json`
//! records, **0 carry `raw_tokens`** (`python3 -c "import json,glob; print(sum(1
//! for f in glob.glob('data/corpus/*/monster_ability/*.json') if 'raw_tokens'
//! in json.load(open(f))['data'])")` -> `0`). A `static`-classified
//! `monster_ability` unit's `wiring_class` is computed independently, straight
//! off the raw PCGen `.lst` row (`wiring_class::classify`); its `status` is
//! computed independently again, off `monster_chassis::MONSTER_BOOKS`'s own
//! table membership check. Neither axis reads `data/corpus/**/*.json` at all.
//! What DOES read it is `corpus_literal_sweep`, whose `--json-out` report is
//! the ONLY thing that can promote a `Static` unit's status to
//! `literal-verified` (`v06_work_inventory::apply_done_rung_stamps`), which is
//! the ONLY status that reaches `done` for a `static` unit
//! (`pf1e_dashboard_producer.doneness_verdict`). A `static` monster ability
//! already `grounded` moves to `done` through this single, narrow gate — never
//! by touching `monster_chassis.rs`'s own tables, never by widening the
//! classifier.
//!
//! **Byte-for-byte, not reconstructed.** Reuses `corpus_literal_sweep`'s OWN
//! `tab_tokens`/`token_closure` functions — the exact code the verifier
//! itself runs — so the tokens this tool writes and the tokens the sweep
//! later re-derives from the same citation are computed by one function, not
//! two independently-written ones that could drift apart. See
//! `enrich_equipment_raw_tokens.rs`'s own doc comment for why a typed
//! re-parse-and-reserialize approach is deliberately avoided.
//!
//! Book-agnostic by construction: walks every `data/corpus/*/monster_ability/`
//! directory that exists on disk rather than naming a fixed book list (unlike
//! `enrich_spell_raw_tokens.rs`'s 5-book scope, which is capped by the
//! engine's own `spell_resolver` chain) — a `monster_ability` unit's `status`
//! is decided by `monster_chassis::MONSTER_BOOKS` table *membership*, not by a
//! fixed enumerable book list the way spell's catalog chain is, so there is no
//! narrower correct scope to name here.

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

/// Every `monster_ability` JSON under a book's `monster_ability/` directory,
/// walked recursively (matches `enrich_spell_raw_tokens.rs`'s own
/// `find_spell_json_files` shape; no currently-registered book nests this
/// kind's records, but a flat `read_dir` silently under-reporting a future
/// nested book is the exact defect that shape already guards against).
fn find_monster_ability_json_files(book_dir: &Path) -> Vec<PathBuf> {
    let dir = book_dir.join("monster_ability");
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
/// here (not imported: that function is private to that binary, not part of
/// the library) rather than reinvented, so this tool and the verifier that
/// checks its output always agree about which book a citation belongs to.
///
/// `dreamscarred_press` (`ultimate_psionics`, `path_of_war`,
/// `psionics_expanded`, `psionics_unleashed`) ships with no `<line>` tier —
/// its oracle layout is `<system>/<publisher>/<book>/<file>`, three
/// directory segments, not four. `corpus_literal_sweep`'s own `book_dir_of`
/// (`src/bin/corpus_literal_sweep.rs`) already special-cases this; this
/// copy had drifted out of sync with it and silently treated every
/// `dreamscarred_press` monster_ability citation as a `CitationMiss`
/// instead -- confirmed live: all 13 `ultimate_psionics` `monster_ability`
/// records sat at `held` for want of this one branch (SD-31 wave 20).
fn book_dir_of(source_path: &str) -> Option<String> {
    let segments: Vec<&str> = source_path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() >= 5 {
        return Some(segments[..4].join("/"));
    }
    if segments.len() == 4 && segments[1] == "dreamscarred_press" {
        return Some(segments[..3].join("/"));
    }
    None
}

/// One book's `.MOD` rows, keyed by the record name they target — the same
/// derivation `corpus_literal_sweep`'s own `Sweep::mod_index` performs,
/// duplicated here (not imported: that method is a private impl on a
/// binary-local struct, not part of the library) at the single-book-map call
/// site `build_mod_index` itself documents as its normal shape. `book_dir` is
/// the FULL corpus-relative directory ([`book_dir_of`]'s return), never the
/// short book slug.
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
    DroppedPi(String),
}

/// PI-screen one closure field's value: blacklist term scan
/// ([`classify_field`]) union'd with the row's own `DESCISPI:YES`
/// declaration for `DESC`-keyed fields specifically -- SD-30 `§52.3`/`§53.5`,
/// byte-identical contract to `enrich_monster_raw_tokens.rs`'s and
/// `enrich_companion_raw_tokens.rs`'s functions of the same name.
///
/// **`SD31-E6-F9-005`: this tool shipped with NO PI screening at all until
/// this fix** -- `raw_tokens` were written verbatim from the closure, the
/// same production-path gap adversarial review found and fixed in
/// `enrich_companion_raw_tokens.rs` (`SD31-E6-F7-001`, "a substituted
/// author-time grep, not a production-path call"). Confirmed live before the
/// fix: `python3 -c "import json,glob; print(sum(1 for f in
/// glob.glob('data/corpus/*/monster_ability/*.json') if 'raw_tokens' in
/// json.load(open(f))['data']))"` found every currently-enriched
/// `monster_ability` record's `raw_tokens` had never passed through either
/// contract -- none of the corpus-wide 724 sampled hit the blacklist
/// (`declared_pi_shipping_audit` confirms clean both before and after this
/// fix, so no exposure occurred), but the NEXT book onboarded through this
/// path had no screen at all until now.
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
    // book), never just the base row alone -- SD-30 `§52.3`/`§53.5`, mirrors
    // `enrich_monster_raw_tokens.rs`'s/`enrich_companion_raw_tokens.rs`'s
    // identical call.
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
    for (key, value) in &pairs {
        let (stored, _redacted) = screen_field_value(key, value, declared.description);
        raw_tokens.push(json!({ "key": key, "value": stored }));
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
    let mut total_dropped = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut dropped: Vec<String> = Vec::new();
    let mut mod_index_cache: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    let Ok(book_entries) = fs::read_dir(&corpus_root) else {
        eprintln!("enrich_monster_ability_raw_tokens: no {corpus_root:?} directory found");
        return;
    };
    let mut book_dirs: Vec<PathBuf> = book_entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    book_dirs.sort();

    for book_dir in &book_dirs {
        let book_name = book_dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let files = find_monster_ability_json_files(book_dir);
        if files.is_empty() {
            continue;
        }
        let mut book_enriched = 0u32;
        for file in &files {
            match enrich_one(file, &data_root, &mut mod_index_cache) {
                Outcome::Enriched => {
                    total_enriched += 1;
                    book_enriched += 1;
                }
                Outcome::NoLstCitation => total_no_citation += 1,
                Outcome::AlreadyEnriched => total_already += 1,
                Outcome::CitationMiss(msg) => misses.push(format!("{}: {}", file.display(), msg)),
                Outcome::DroppedPi(msg) => {
                    total_dropped += 1;
                    dropped.push(format!("{}: {}", file.display(), msg));
                }
            }
        }
        eprintln!("{book_name}: {} monster_ability files scanned, {book_enriched} enriched", files.len());
    }

    eprintln!(
        "\nenrich_monster_ability_raw_tokens: {total_enriched} enriched, {total_no_citation} no-LST-citation (untouched), {total_already} already-enriched, {total_dropped} dropped (NAMEISPI:YES), {} citation misses",
        misses.len()
    );
    if !misses.is_empty() {
        eprintln!("\nCitation misses (not enriched, real gaps to investigate):");
        for miss in &misses {
            eprintln!("  {miss}");
        }
    }
    if !dropped.is_empty() {
        eprintln!("\nDropped for declared Product Identity (record removed, not shipped):");
        for d in &dropped {
            eprintln!("  {d}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway `PCGEN_CORPUS_ROOT`-shaped book directory plus a
    /// throwaway `data/corpus`-shaped monster_ability JSON, both under
    /// `std::env::temp_dir()` and cleaned up on drop — same pattern
    /// `enrich_spell_raw_tokens.rs`'s own `Scratch` fixture uses.
    struct Scratch {
        data_root: PathBuf,
        corpus_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_enrich_monster_ability_raw_tokens_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let data_root = base.join("pcgen_data");
            let corpus_root = base.join("data_corpus");
            fs::create_dir_all(data_root.join("pathfinder/paizo/roleplaying_game/x_book")).unwrap();
            fs::create_dir_all(corpus_root.join("x_book/monster_ability")).unwrap();
            Scratch { data_root, corpus_root }
        }

        fn write_lst(&self, contents: &str) {
            fs::write(
                self.data_root.join("pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst"),
                contents,
            )
            .unwrap();
        }

        fn write_json(&self, name: &str, contents: &str) -> PathBuf {
            let path = self.corpus_root.join("x_book/monster_ability").join(name);
            fs::write(&path, contents).unwrap();
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.data_root.parent().unwrap());
        }
    }

    // ----- book_dir_of -----

    #[test]
    fn book_dir_of_resolves_a_standard_five_segment_publisher_path() {
        assert_eq!(
            book_dir_of("pathfinder/paizo/roleplaying_game/core_essentials/ce_abilities_race.lst"),
            Some("pathfinder/paizo/roleplaying_game/core_essentials".to_string())
        );
    }

    /// `dreamscarred_press` ships with no `<line>` tier in the oracle
    /// checkout -- `pathfinder/dreamscarred_press/ultimate_psionics/
    /// up_abilities_race.lst` is a real, live citation on 13 shipped
    /// `monster_ability` records (`data/corpus/ultimate_psionics/
    /// monster_ability/*.json`), every one of which this function's
    /// pre-fix `segments.len() < 5 -> None` rule silently refused to
    /// enrich, corpus-wide (SD-31 wave 20). Matches `corpus_literal_sweep`'s
    /// own `book_dir_of` special case exactly, and
    /// `enrich_monster_raw_tokens.rs`'s sibling fix for the same drift.
    #[test]
    fn book_dir_of_resolves_the_four_segment_dreamscarred_press_shape() {
        assert_eq!(
            book_dir_of("pathfinder/dreamscarred_press/ultimate_psionics/up_abilities_race.lst"),
            Some("pathfinder/dreamscarred_press/ultimate_psionics".to_string())
        );
    }

    #[test]
    fn book_dir_of_refuses_a_four_segment_path_from_a_non_dreamscarred_publisher() {
        assert_eq!(book_dir_of("pathfinder/paizo/roleplaying_game/x_abilities.lst"), None);
    }

    // ----- split_token_field: the round-trip the whole tool depends on -----

    #[test]
    fn split_token_field_splits_on_the_first_colon_only() {
        assert_eq!(split_token_field("TYPE:SpecialAttack.Extraordinary"), Some(("TYPE", "SpecialAttack.Extraordinary")));
        assert_eq!(
            split_token_field("DESC:It grabs: then it grapples."),
            Some(("DESC", "It grabs: then it grapples."))
        );
    }

    #[test]
    fn split_token_field_every_result_reconstructs_the_original_field() {
        for field in ["TYPE:SpecialAttack.Extraordinary", "CATEGORY:Special Ability", "BONUS:COMBAT|GRAPPLE|8"] {
            let (key, value) = split_token_field(field).unwrap();
            assert_eq!(format!("{key}:{value}"), field);
        }
    }

    #[test]
    fn split_token_field_refuses_a_field_with_no_colon() {
        assert_eq!(split_token_field("NoColonAtAll"), None);
    }

    // ----- enrich_one: the real end-to-end path against a throwaway corpus -----

    /// Mirrors the real `aurumvorax_grab` record traced this cycle
    /// (`bestiary_2/monster_ability/aurumvorax_grab.json`): a `static`,
    /// `grounded` unit whose JSON carries a valid `lst_token` citation but no
    /// `raw_tokens` — the exact shape that keeps every `monster_ability`
    /// record in this corpus at `held` today.
    #[test]
    fn enrich_one_adds_the_full_token_closure_byte_for_byte() {
        let scratch = Scratch::new("basic");
        scratch.write_lst("Grab\tKEY:Aurumvorax ~ Grab\tCATEGORY:Special Ability\tTYPE:SpecialAttack.Extraordinary\tDESC:It can grab a foe.\n");
        let json_path = scratch.write_json(
            "aurumvorax_grab.json",
            r#"{"data":{"key":"bestiary_2:monster_ability:aurumvorax_grab","corpus_key":"Aurumvorax ~ Grab","name":"Grab"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst","line":1,"record_key":"Aurumvorax ~ Grab"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().expect("raw_tokens array present");
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        let expected: BTreeSet<String> = [
            "KEY:Aurumvorax ~ Grab",
            "CATEGORY:Special Ability",
            "TYPE:SpecialAttack.Extraordinary",
            "DESC:It can grab a foe.",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        assert_eq!(joined, expected);
    }

    #[test]
    fn enrich_one_includes_a_mod_rows_tokens_in_the_closure() {
        let scratch = Scratch::new("modrow");
        scratch.write_lst(
            "Grab\tKEY:Aurumvorax ~ Grab\tTYPE:SpecialAttack\nAurumvorax ~ Grab.MOD\tDESC:Updated text.\n",
        );
        let json_path = scratch.write_json(
            "aurumvorax_grab.json",
            r#"{"data":{"key":"bestiary_2:monster_ability:aurumvorax_grab","corpus_key":"Aurumvorax ~ Grab"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst","line":1,"record_key":"Aurumvorax ~ Grab"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        assert!(
            joined.contains("DESC:Updated text."),
            "the .MOD row's own DESC token must appear in the enriched raw_tokens, not just the base row's"
        );
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
        let (stored, redacted) = screen_field_value("DESC", "Blessed by Iomedae", false);
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

    /// MUTATION PROOF for the NAMEISPI drop path -- SD-30 `§50.3`: a name
    /// cannot be redacted, only dropped. This is the production write path
    /// this tool previously carried NO screening at all on
    /// (`SD31-E6-F9-005`, `OPEN-ISSUES.md` row 204's sibling finding) --
    /// this test proves the call is now actually wired.
    #[test]
    fn enrich_one_drops_a_record_whose_base_row_declares_nameispi() {
        let scratch = Scratch::new("nameispi");
        scratch.write_lst("Aura of Locusts\tKEY:Demon Lord (Pazuzu) ~ Aura of Locusts\tNAMEISPI:YES\tCATEGORY:Special Ability\n");
        let json_path = scratch.write_json(
            "aura_of_locusts.json",
            r#"{"data":{"name":"Aura of Locusts","key":"x_book:monster_ability:aura_of_locusts"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst","line":1,"record_key":"Demon Lord (Pazuzu) ~ Aura of Locusts"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::DroppedPi(_)), "expected a drop, not an enrich");
        assert!(!json_path.exists(), "a NAMEISPI:YES record must be removed from disk, never shipped");
    }

    /// Same proof, but the declaration arrives via a `.MOD` row rather than
    /// the base row -- `declared_product_identity` must read the WHOLE
    /// closure, not just the cited line.
    #[test]
    fn enrich_one_drops_a_record_whose_mod_row_declares_nameispi() {
        let scratch = Scratch::new("nameispi_mod");
        scratch.write_lst(
            "Grab\tKEY:Aurumvorax ~ Grab\tCATEGORY:Special Ability\nAurumvorax ~ Grab.MOD\tNAMEISPI:YES\n",
        );
        let json_path = scratch.write_json(
            "aurumvorax_grab.json",
            r#"{"data":{"name":"Grab","key":"x_book:monster_ability:aurumvorax_grab"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst","line":1,"record_key":"Aurumvorax ~ Grab"}}"#,
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
        scratch.write_lst("Aura\tKEY:Herald ~ Aura\tDESC:A herald blessed by Iomedae herself.\tCATEGORY:Special Ability\n");
        let json_path = scratch.write_json(
            "herald_aura.json",
            r#"{"data":{"name":"Aura","key":"x_book:monster_ability:herald_aura"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst","line":1,"record_key":"Herald ~ Aura"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::Enriched));

        let written: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = written["data"]["raw_tokens"].as_array().unwrap();
        let desc = tokens.iter().find(|t| t["key"] == "DESC").unwrap();
        assert_eq!(desc["value"], REDACTED_PI_MARKER, "the deity-name hit must not ship verbatim");
    }

    #[test]
    fn enrich_one_leaves_an_already_enriched_record_untouched() {
        let scratch = Scratch::new("already");
        scratch.write_lst("Grab\tTYPE:SpecialAttack\n");
        let json_path = scratch.write_json(
            "aurumvorax_grab.json",
            r#"{"data":{"key":"K","raw_tokens":[{"key":"TYPE","value":"SpecialAttack"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst","line":1}}"#,
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
        scratch.write_lst("Grab\tTYPE:SpecialAttack\n");
        let json_path = scratch.write_json(
            "ghost.json",
            r#"{"data":{"key":"Ghost Ability"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_abilities_race.lst","line":99}}"#,
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
            r#"{"data":{"key":"Web Second Source Ability"},"source":{"kind":"web_second_source"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = enrich_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::NoLstCitation));
    }

    #[test]
    fn find_monster_ability_json_files_finds_flat_files() {
        let scratch = Scratch::new("flatscan");
        scratch.write_json("a.json", "{}");
        scratch.write_json("b.json", "{}");
        let found = find_monster_ability_json_files(&scratch.corpus_root.join("x_book"));
        assert_eq!(found.len(), 2);
    }
}
