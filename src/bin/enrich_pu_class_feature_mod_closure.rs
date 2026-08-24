//! Fixes the 9 `pathfinder_unchained` `class_feature` records
//! `class_feature::generate()`'s own post-row-21 blast-radius re-scan named (`kanban.md` row 21
//! cycle 1): real `.MOD`-appended rows targeting these 9 records' own identity exist in
//! `pu_abilities_class.lst` but were never captured, because `src/bin/ingest_pu_classes.rs`'s
//! `raw_tokens_excluding_bonus`/`raw_bonus_chains` (before this cycle's own fix to that file) read
//! only the record's single base row. `class_feature::generate()` correctly refuses to overwrite
//! these coordinates (`foreign_citations` guard, `data.class_key` present) -- they are owned by
//! `ingest_pu_classes.rs`, not the generic generator, so the row 21 fix never reached them.
//!
//! **Why this tool exists separately from just re-running `ingest_pu_classes`, now that its own
//! generator functions are fixed (this cycle, same brief item):** `ingest_pu_classes::main`
//! unconditionally `fs::remove_dir_all`s the WHOLE `data/corpus/pathfinder_unchained/{class,
//! class_feature}` trees before regenerating -- but `class_feature/` is SHARED with the generic
//! `class_feature::generate()` pipeline, which independently writes 540 non-foreign records into
//! the SAME directory (confirmed live: a real run wiped exactly those 540 files, caught by
//! `git status --porcelain` before any commit, reverted). Fixing that directory-ownership defect
//! is a separate, larger, out-of-scope change this cycle does not attempt (`decisions.md §27b`:
//! not this cycle's mechanism to rebuild). This tool instead does the SAME non-destructive,
//! additive-patch-in-place enrichment `enrich_class_raw_tokens.rs` uses for the sibling `class`
//! shape (b) gap: read the existing JSON, recompute exactly `data.raw_tokens`/
//! `data.raw_bonus_chains` from the SAME `WiringClassIndex::closure_rows` machinery row 21 already
//! proved safe, and write back only those two keys -- every other field, and every OTHER file in
//! the directory, untouched.
//!
//! Scoped to exactly the 9 coordinates the blast-radius re-scan named (re-derived independently
//! here by cross-referencing every `pathfinder_unchained` `class_feature` record whose `data.key`
//! is also a real `.MOD` target in `pu_abilities_class.lst`, CATEGORY-prefix stripped the same way
//! `wiring_class::mod_base_name` already does) -- never a directory-wide sweep.

use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::cache_gen::WiringClassIndex;
use codex::rules_core::corpus_literal_sweep::tab_tokens;
use codex::rules_core::pi_screening::{classify_field, declared_product_identity, DeclaredProductIdentity};
use codex::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};
use serde_json::{json, Value};

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    if let Ok(v) = std::env::var("PCGEN_DATA_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

fn find_class_feature_json_files(corpus_root: &Path, book: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let dir = corpus_root.join(book).join("class_feature");
    if !dir.is_dir() {
        return out;
    }
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        let Ok(entries) = fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out
}

fn declared_pi_on_line(line_text: &str) -> DeclaredProductIdentity {
    let tokens: Vec<(&str, &str)> = line_text.split('\t').filter_map(|field| field.split_once(':')).collect();
    declared_product_identity(tokens)
}

fn screen_field_value(key: &str, value: &str, declared_description: bool) -> String {
    if key.eq_ignore_ascii_case("DESC") && declared_description {
        return REDACTED_PI_MARKER.to_string();
    }
    let (license, ..) = classify_field(key, value);
    if license == License::PiRedacted {
        return REDACTED_PI_MARKER.to_string();
    }
    value.to_string()
}

enum Outcome {
    Patched { tokens: usize, bonus: usize },
    NotForeign,
    NoLstCitation,
    NoRealChange,
    CitationMiss(String),
    DroppedPi(String),
}

fn patch_one(path: &Path, data_root: &Path, wiring_index: &WiringClassIndex) -> Outcome {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    let data = root.get("data").unwrap_or_else(|| panic!("{path:?}: no top-level \"data\" object"));
    // Only ever touches a foreign (`ingest_pu_classes.rs`-owned) class_feature record -- the same
    // discriminator `class_feature::generate()`'s own `foreign_citations` guard uses.
    if data.get("class_key").is_none() {
        return Outcome::NotForeign;
    }
    let key = data.get("key").and_then(Value::as_str).unwrap_or("").to_string();

    let source = root["source"].clone();
    if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
        return Outcome::NoLstCitation;
    }
    let lst_rel_path = source["path"].as_str().expect("lst_token source must carry a path").to_string();
    let line = source["line"].as_u64().expect("lst_token source must carry a line") as u32;

    let lst_full_path = data_root.join(&lst_rel_path);
    if !lst_full_path.is_file() {
        return Outcome::CitationMiss(format!("cited LST file not found: {lst_full_path:?}"));
    }
    let lst_basename = Path::new(&lst_rel_path).file_name().and_then(|n| n.to_str()).unwrap_or(&lst_rel_path);

    let mut lines = wiring_index.lines();
    let closure_rows = wiring_index.closure_rows(&mut lines, lst_basename, line, &key, &key);
    let Some(Some(base_row)) = closure_rows.first() else {
        return Outcome::CitationMiss(format!("{lst_rel_path}:{line} (key={key:?}): no row at the cited line"));
    };
    let cited_line_text = fs::read_to_string(&lst_full_path)
        .unwrap_or_default()
        .lines()
        .nth((line as usize).saturating_sub(1))
        .unwrap_or("")
        .to_string();
    if base_row != &cited_line_text {
        return Outcome::CitationMiss(format!(
            "{lst_rel_path}:{line} (key={key:?}): closure's base row does not match the cited line's own text"
        ));
    }

    let mut declared_name = false;
    let mut declared_description = false;
    for row in closure_rows.iter().flatten() {
        let d = declared_pi_on_line(row);
        declared_name = declared_name || d.name;
        declared_description = declared_description || d.description;
    }
    if declared_name {
        return Outcome::DroppedPi(format!(
            "{lst_rel_path}:{line} (key={key:?}) declares NAMEISPI:YES in its own closure -- refusing to patch"
        ));
    }

    let mut raw_tokens: Vec<Value> = Vec::new();
    let mut raw_bonus_chains: Vec<Value> = Vec::new();
    for row in closure_rows.iter().flatten() {
        for field in tab_tokens(row) {
            let Some((k, v)) = field.split_once(':') else { continue };
            if k == "BONUS" {
                let (license, ..) = classify_field("BONUS", v);
                let qualifiers: Vec<String> = if license == License::PiRedacted {
                    vec![REDACTED_PI_MARKER.to_string()]
                } else {
                    v.split('|').map(str::to_string).collect()
                };
                raw_bonus_chains.push(json!({ "qualifiers": qualifiers }));
            } else {
                let stored = screen_field_value(k, v, declared_description);
                raw_tokens.push(json!({ "key": k, "value": stored }));
            }
        }
    }

    let existing_tokens = data.get("raw_tokens").cloned().unwrap_or(Value::Array(vec![]));
    let existing_bonus = data.get("raw_bonus_chains").cloned().unwrap_or(Value::Array(vec![]));
    if existing_tokens == Value::Array(raw_tokens.clone()) && existing_bonus == Value::Array(raw_bonus_chains.clone())
    {
        return Outcome::NoRealChange;
    }

    let (tcount, bcount) = (raw_tokens.len(), raw_bonus_chains.len());
    let data_obj =
        root.get_mut("data").and_then(Value::as_object_mut).expect("\"data\" must be a JSON object");
    data_obj.insert("raw_tokens".to_string(), Value::Array(raw_tokens));
    data_obj.insert("raw_bonus_chains".to_string(), Value::Array(raw_bonus_chains));

    let new_json = serde_json::to_string_pretty(&root).expect("serialize patched record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    Outcome::Patched { tokens: tcount, bonus: bcount }
}

fn main() {
    let data_root = pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");
    let book = "pathfinder_unchained";
    let pu_book_dir = data_root.join("pathfinder/paizo/roleplaying_game/pathfinder_unchained");
    let wiring_index = WiringClassIndex::build(book, &pu_book_dir);

    let files = find_class_feature_json_files(&corpus_root, book);
    let mut patched = 0u32;
    let mut not_foreign = 0u32;
    let mut no_change = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut dropped_pi: Vec<String> = Vec::new();

    for file in &files {
        match patch_one(file, &data_root, &wiring_index) {
            Outcome::Patched { tokens, bonus } => {
                patched += 1;
                eprintln!("{}: patched, {tokens} raw_tokens, {bonus} raw_bonus_chains", file.display());
            }
            Outcome::NotForeign => not_foreign += 1,
            Outcome::NoLstCitation => {}
            Outcome::NoRealChange => no_change += 1,
            Outcome::CitationMiss(m) => misses.push(format!("{}: {}", file.display(), m)),
            Outcome::DroppedPi(m) => dropped_pi.push(format!("{}: {}", file.display(), m)),
        }
    }

    eprintln!(
        "\nenrich_pu_class_feature_mod_closure: {} class_feature files scanned, {not_foreign} \
         not-foreign (untouched), {no_change} already-correct (untouched), {patched} patched, \
         {} citation misses, {} dropped for declared PI",
        files.len(),
        misses.len(),
        dropped_pi.len()
    );
    for m in &misses {
        eprintln!("  MISS: {m}");
    }
    for m in &dropped_pi {
        eprintln!("  DROPPED-PI: {m}");
    }
}
