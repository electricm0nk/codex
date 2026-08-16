//! Repairs the spell-citation defect `SD31-E6-F2-001` root-caused and logged
//! at `docs/release/SD-31-corpus-closure-grind/artifacts/OPEN-ISSUES.md` row
//! 33: **101 `spell` corpus records cite a `.MOD` row instead of the base
//! declaration row**, so `v06_work_inventory::apply_done_rung_stamps` (which
//! joins `corpus_literal_sweep`'s verified set on the UNIT's own
//! independently-scanned `(book, file, line)` — always the base declaration,
//! see `wiring_class::CorpusLines::line()`) can never match the shipped
//! record's `(book, file, line)` even though `corpus_literal_sweep` itself
//! verifies the `.MOD`-cited record cleanly. The 101 units sit `held`
//! forever, not because anything is wrong with their transcription, but
//! because the citation and the unit disagree about which line the record
//! lives on.
//!
//! **The fix is the citation, not the transcription.** A `.MOD` row is a
//! bookkeeping PATCH, not a declaration — it never carries `SCHOOL:`/
//! `CLASSES:`, only ever a subset of tokens (here, always `DESC:`). The row
//! that "actually declares the record" (this cycle's own instruction) is the
//! one whose field 0 exactly equals the record's own identity with no
//! `.MOD`/`.COPY=` suffix. Repointing `source.line` there does not lose the
//! `.MOD` row's rich description: `corpus_literal_sweep::token_closure`
//! (imported, not reimplemented — the same discipline
//! `enrich_spell_raw_tokens.rs` follows) walks from the base row PLUS every
//! `.MOD` row targeting the record's own identity, so the regenerated
//! `raw_tokens` closure still contains the full-text `DESC:` token the old,
//! `.MOD`-only closure had, in addition to the `SCHOOL:`/`CLASSES:` tokens
//! the `.MOD` row never carried. Nothing about `data.description`/`school`/
//! `level` — validated content this cycle does not touch — changes; only the
//! citation (`source.line`, `source.record_key`) and the derived
//! `raw_tokens` array move.
//!
//! **Self-contained, no `docs/work-inventory.json` dependency.** The correct
//! line is found the same way `resolve_citation`'s own base-record branch in
//! `src/bin/gen_book_cache.rs` finds it — an exact match on field 0 — with
//! ONE added guard this tool applies that that function does not need
//! (because it never faces a `.MOD`-citation record to begin with): the
//! matched row must itself carry a `SCHOOL:` or `CLASSES:` token, so a
//! bookkeeping row that happens to share field 0 (there are none in this
//! corpus today, but the guard costs nothing and turns a silent wrong
//! answer into a reported miss) can never be mistaken for a declaration.
//!
//! Run with `cargo run --locked --bin repair_spell_citations`.
//! `PCGEN_CORPUS_ROOT` overrides the default `$HOME/workspace/repos/pcgen/data`.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::corpus_literal_sweep::token_closure;
use codex::rules_core::wiring_class::build_mod_index;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// Same five books `enrich_spell_raw_tokens.rs` targets — the only books
/// whose spell citations can affect a `done`-reachable unit at all.
const TARGET_BOOKS: &[&str] = &[
    "core_rulebook",
    "advanced_players_guide",
    "advanced_class_guide",
    "advanced_race_guide",
    "ultimate_intrigue",
];

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// Every `spell` JSON under a book's `spell/` directory, walked recursively —
/// see `enrich_spell_raw_tokens.rs::find_spell_json_files`'s own doc comment
/// on why a single-level `read_dir` is wrong here (`core_rulebook` nests one
/// subdirectory per spell level).
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

/// The corpus-relative book directory a `source.path` citation belongs to —
/// byte-identical derivation to `enrich_spell_raw_tokens.rs::book_dir_of`
/// and `corpus_literal_sweep`'s own private `book_dir_of`, duplicated here
/// for the same "not part of the library" reason those two document.
fn book_dir_of(source_path: &str) -> Option<String> {
    let segments: Vec<&str> = source_path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() < 5 {
        return None;
    }
    Some(segments[..4].join("/"))
}

fn mod_index_for_book(data_root: &Path, book_dir: &str) -> BTreeMap<String, Vec<String>> {
    let mut book_paths = BTreeMap::new();
    book_paths.insert(book_dir.to_string(), data_root.join(book_dir));
    build_mod_index(&book_paths).into_iter().map(|((_, name), rows)| (name, rows)).collect()
}

/// The 1-indexed line whose field 0 is an EXACT match for `record_name` —
/// never a `.MOD`/`.COPY=` variant, since those always carry a suffix on
/// field 0. Additionally requires the row to carry a `SCHOOL:` or
/// `CLASSES:` token, so a bookkeeping row that happens to share field 0
/// (none exist in this corpus today; the guard is defensive) is never
/// mistaken for the declaration.
fn find_declaration_line(lst_text: &str, record_name: &str) -> Option<u32> {
    for (idx, line) in lst_text.split('\n').enumerate() {
        let trimmed = line.trim_end_matches(['\r']);
        let mut fields = trimmed.split('\t');
        let first_col = fields.next().unwrap_or("");
        if first_col == record_name
            && fields.any(|f| f.starts_with("SCHOOL:") || f.starts_with("CLASSES:"))
        {
            return Some((idx + 1) as u32);
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Repaired { old_line: u32, new_line: u32 },
    AlreadyCorrect,
    NotApplicable,
    NoDeclarationFound(String),
}

fn repair_one(
    path: &Path,
    data_root: &Path,
    mod_index_cache: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>,
) -> Outcome {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    let has_raw_tokens = root
        .get("data")
        .and_then(|d| d.get("raw_tokens"))
        .is_some();
    if !has_raw_tokens {
        // Nothing to repair: `enrich_spell_raw_tokens` has not run on this
        // record yet, and citation repair is a refinement of an existing
        // citation, not a substitute for enrichment.
        return Outcome::NotApplicable;
    }
    let source = root["source"].clone();
    if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
        return Outcome::NotApplicable;
    }
    let Some(key) = root["data"]["key"].as_str().map(str::to_string) else {
        return Outcome::NotApplicable;
    };
    let lst_rel_path = source["path"].as_str().expect("lst_token source must carry a path").to_string();
    let old_line = source["line"].as_u64().expect("lst_token source must carry a line") as u32;
    let Some(book_dir) = book_dir_of(&lst_rel_path) else {
        return Outcome::NoDeclarationFound(format!(
            "{lst_rel_path} is not <system>/<publisher>/<line>/<book>/<file>-shaped"
        ));
    };

    let lst_full_path = data_root.join(&lst_rel_path);
    let Ok(lst_text) = fs::read_to_string(&lst_full_path) else {
        return Outcome::NoDeclarationFound(format!("cited LST file not found: {lst_full_path:?}"));
    };

    let Some(new_line) = find_declaration_line(&lst_text, &key) else {
        return Outcome::NoDeclarationFound(format!(
            "{lst_rel_path}: no SCHOOL:/CLASSES:-bearing row with field 0 == {key:?} found"
        ));
    };
    if new_line == old_line {
        return Outcome::AlreadyCorrect;
    }

    let lines: Vec<&str> = lst_text.split('\n').collect();
    let base_row = lines[(new_line - 1) as usize];

    let mod_index = mod_index_cache
        .entry(book_dir.clone())
        .or_insert_with(|| mod_index_for_book(data_root, &book_dir));

    let mut identities: BTreeSet<String> = BTreeSet::new();
    identities.insert(key.clone());
    if let Some(name) = root["data"]["name"].as_str() {
        identities.insert(name.to_string());
    }
    if let Some(record_key) = source.get("record_key").and_then(Value::as_str) {
        // Some generators stamped `record_key` WITH the `.MOD` suffix
        // (`"Arcane Mark.MOD"`); `mod_index` is keyed by the base name, so a
        // suffixed identity would silently fail to look itself up. Strip it
        // for the identity set — `token_closure` itself de-duplicates.
        identities.insert(
            record_key.strip_suffix(".MOD").unwrap_or(record_key).to_string(),
        );
    }

    let closure = token_closure(base_row, &identities, mod_index);
    if closure.is_empty() {
        return Outcome::NoDeclarationFound(format!(
            "{lst_rel_path}:{new_line}: declaration row carries no tab-separated fields"
        ));
    }
    let mut raw_tokens: Vec<Value> = Vec::with_capacity(closure.len());
    for field in &closure {
        let Some((k, v)) = field.split_once(':') else {
            return Outcome::NoDeclarationFound(format!(
                "{lst_rel_path}:{new_line}: closure field {field:?} carries no ':'"
            ));
        };
        raw_tokens.push(json!({ "key": k, "value": v }));
    }

    let data_obj = root.get_mut("data").and_then(Value::as_object_mut).expect("checked above");
    data_obj.insert("raw_tokens".to_string(), Value::Array(raw_tokens));

    let source_obj = root.get_mut("source").and_then(Value::as_object_mut).expect("checked above");
    source_obj.insert("line".to_string(), json!(new_line));
    source_obj.insert("record_key".to_string(), json!(key));

    let new_json = serde_json::to_string_pretty(&root).expect("serialize repaired record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));

    Outcome::Repaired { old_line, new_line }
}

fn main() {
    let data_root = pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");

    let mut total_repaired = 0u32;
    let mut total_already_correct = 0u32;
    let mut total_not_applicable = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut mod_index_cache: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    for book in TARGET_BOOKS {
        let book_dir = corpus_root.join(book);
        if !book_dir.is_dir() {
            continue;
        }
        let files = find_spell_json_files(&book_dir);
        let mut book_repaired = 0u32;
        for file in &files {
            match repair_one(file, &data_root, &mut mod_index_cache) {
                Outcome::Repaired { old_line, new_line } => {
                    total_repaired += 1;
                    book_repaired += 1;
                    eprintln!("  repaired {}: line {old_line} -> {new_line}", file.display());
                }
                Outcome::AlreadyCorrect => total_already_correct += 1,
                Outcome::NotApplicable => total_not_applicable += 1,
                Outcome::NoDeclarationFound(msg) => misses.push(format!("{}: {}", file.display(), msg)),
            }
        }
        eprintln!("{book}: {} spell files scanned, {book_repaired} repaired", files.len());
    }

    eprintln!(
        "\nrepair_spell_citations: {total_repaired} repaired, {total_already_correct} already-correct, \
         {total_not_applicable} not-applicable (no raw_tokens or non-lst_token), {} misses",
        misses.len()
    );
    if !misses.is_empty() {
        eprintln!("\nMisses (not repaired, real gaps to investigate):");
        for miss in &misses {
            eprintln!("  {miss}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch {
        data_root: PathBuf,
        corpus_root: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_repair_spell_citations_{name}_{}", std::process::id()));
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
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            if let Some(base) = self.data_root.parent() {
                let _ = fs::remove_dir_all(base);
            }
        }
    }

    /// The worked example from this tool's own doc comment, reproduced as a
    /// fixture: a base declaration row (SCHOOL/CLASSES, short DESC) plus a
    /// `.MOD` row carrying the real full-text DESC, with the shipped record
    /// mis-citing the `.MOD` row (exactly `accelerate_poison.json`'s shape
    /// before this cycle).
    #[test]
    fn repair_one_repoints_a_mod_row_citation_to_the_base_declaration() {
        let scratch = Scratch::new("modcite");
        scratch.write_lst(
            "Accelerate Poison\tCLASSES:Wizard=2\tSCHOOL:Transmutation\tDESC:short|!PRERULE:1,DisplayFullSpell\n\
             filler\n\
             Accelerate Poison.MOD\tDESC:the real full text|PRERULE:1,DisplayFullSpell\n",
        );
        let json_path = scratch.write_json(
            "accelerate_poison.json",
            r#"{"data":{"key":"Accelerate Poison","school":"Transmutation","level":2,"description":"the real full text","raw_tokens":[{"key":"DESC","value":"the real full text|PRERULE:1,DisplayFullSpell"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":3,"record_key":"Accelerate Poison"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = repair_one(&json_path, &scratch.data_root, &mut cache);
        assert_eq!(outcome, Outcome::Repaired { old_line: 3, new_line: 1 });

        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        assert_eq!(after["source"]["line"], 1);
        assert_eq!(after["source"]["record_key"], "Accelerate Poison");
        let tokens = after["data"]["raw_tokens"].as_array().unwrap();
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        assert!(joined.contains("SCHOOL:Transmutation"), "base row's SCHOOL token must now be present: {joined:?}");
        assert!(joined.contains("CLASSES:Wizard=2"), "base row's CLASSES token must now be present: {joined:?}");
        assert!(
            joined.contains("DESC:the real full text|PRERULE:1,DisplayFullSpell"),
            "the .MOD row's own rich-text DESC must still be present via closure: {joined:?}"
        );
        // Content fields untouched by a citation repair.
        assert_eq!(after["data"]["description"], "the real full text");
        assert_eq!(after["data"]["school"], "Transmutation");
        assert_eq!(after["data"]["level"], 2);
    }

    /// A `record_key` carrying the `.MOD` suffix (the `core_rulebook`
    /// generator's own convention, e.g. `"Arcane Mark.MOD"`) must still
    /// resolve the closure's `.MOD` row -- the suffix is stripped before use
    /// as an identity, not looked up verbatim against `mod_index`.
    #[test]
    fn repair_one_strips_a_dot_mod_suffixed_record_key_before_using_it_as_an_identity() {
        let scratch = Scratch::new("suffixedkey");
        scratch.write_lst(
            "Arcane Mark\tCLASSES:Wizard=0\tSCHOOL:Universal\n\
             Arcane Mark.MOD\tDESC:the real full text\n",
        );
        let json_path = scratch.write_json(
            "arcane_mark.json",
            r#"{"data":{"key":"Arcane Mark","raw_tokens":[{"key":"DESC","value":"the real full text"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":2,"record_key":"Arcane Mark.MOD"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = repair_one(&json_path, &scratch.data_root, &mut cache);
        assert_eq!(outcome, Outcome::Repaired { old_line: 2, new_line: 1 });
        let after: Value = serde_json::from_str(&fs::read_to_string(&json_path).unwrap()).unwrap();
        let tokens = after["data"]["raw_tokens"].as_array().unwrap();
        let joined: BTreeSet<String> = tokens
            .iter()
            .map(|t| format!("{}:{}", t["key"].as_str().unwrap(), t["value"].as_str().unwrap()))
            .collect();
        assert!(joined.contains("DESC:the real full text"), "{joined:?}");
        assert!(joined.contains("SCHOOL:Universal"), "{joined:?}");
    }

    #[test]
    fn repair_one_leaves_an_already_correct_citation_untouched() {
        let scratch = Scratch::new("correct");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\tCLASSES:Bloodrager=1\n");
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash","raw_tokens":[{"key":"SCHOOL","value":"Transmutation"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1,"record_key":"Blade Lash"}}"#,
        );
        let before = fs::read_to_string(&json_path).unwrap();
        let mut cache = BTreeMap::new();
        let outcome = repair_one(&json_path, &scratch.data_root, &mut cache);
        assert_eq!(outcome, Outcome::AlreadyCorrect);
        assert_eq!(fs::read_to_string(&json_path).unwrap(), before, "an already-correct citation must not be rewritten");
    }

    #[test]
    fn repair_one_skips_a_record_with_no_raw_tokens_yet() {
        let scratch = Scratch::new("noraw");
        scratch.write_lst("Blade Lash\tSCHOOL:Transmutation\n");
        let json_path = scratch.write_json(
            "blade_lash.json",
            r#"{"data":{"key":"Blade Lash"},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1}}"#,
        );
        let before = fs::read_to_string(&json_path).unwrap();
        let mut cache = BTreeMap::new();
        let outcome = repair_one(&json_path, &scratch.data_root, &mut cache);
        assert_eq!(outcome, Outcome::NotApplicable);
        assert_eq!(fs::read_to_string(&json_path).unwrap(), before);
    }

    #[test]
    fn repair_one_reports_a_miss_rather_than_inventing_a_line_when_no_declaration_row_exists() {
        let scratch = Scratch::new("miss");
        // Only a `.MOD` row exists for this key -- no base declaration at all
        // (a genuinely different, worse corpus shape than the ones this tool
        // repairs; must be reported, never silently left as-is or guessed at).
        scratch.write_lst("Ghost Spell.MOD\tDESC:orphaned patch\n");
        let json_path = scratch.write_json(
            "ghost.json",
            r#"{"data":{"key":"Ghost Spell","raw_tokens":[{"key":"DESC","value":"orphaned patch"}]},"source":{"kind":"lst_token","path":"pathfinder/paizo/roleplaying_game/x_book/x_spells.lst","line":1,"record_key":"Ghost Spell"}}"#,
        );
        let before = fs::read_to_string(&json_path).unwrap();
        let mut cache = BTreeMap::new();
        let outcome = repair_one(&json_path, &scratch.data_root, &mut cache);
        assert!(matches!(outcome, Outcome::NoDeclarationFound(_)));
        assert_eq!(fs::read_to_string(&json_path).unwrap(), before, "a miss must not rewrite the record");
    }

    #[test]
    fn repair_one_skips_a_non_lst_token_source_without_error() {
        let scratch = Scratch::new("nonlst");
        let json_path = scratch.write_json(
            "web.json",
            r#"{"data":{"key":"Web Spell","raw_tokens":[{"key":"DESC","value":"x"}]},"source":{"kind":"web_second_source"}}"#,
        );
        let mut cache = BTreeMap::new();
        let outcome = repair_one(&json_path, &scratch.data_root, &mut cache);
        assert_eq!(outcome, Outcome::NotApplicable);
    }
}
