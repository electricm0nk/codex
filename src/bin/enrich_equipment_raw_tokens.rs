//! Book-agnostic Shape B v1 enrichment: populates `raw_tokens`/
//! `raw_bonus_chains` on every existing on-disk equipment record.
//!
//! Discovered 2026-07-30 while wiring the desktop app's real corpus loading
//! (`docs/release/v0.6/book-agnostic-backend-gaps-scoping.md` finding 4):
//! every book's equipment codegen pipeline has independently evolved (CRB
//! reads from a hand-curated 2,977-entry static table, APG/ACG/Bestiary
//! from their own pre-compiled tables using a `weight` field name instead
//! of CRB's `weight_lbs`, ARG/PU parse raw LST directly) -- retrofitting
//! each one individually to also capture raw mechanical tokens would mean
//! touching and re-verifying up to 6 different pipelines with their own
//! histories and, as this file's own first version proved the hard way,
//! their own field-name divergences.
//!
//! Every Shape B v1 record already carries an exact citation
//! (`source.path` + `source.line`, an `LstToken`-kind source) back to its
//! real PCGen LST source line -- regardless of which pipeline produced it.
//! This tool uses that citation directly: re-parse the cited raw LST file,
//! find the record whose `header_line_number` matches `source.line`, and
//! add `raw_tokens`/`raw_bonus_chains` keys onto the on-disk JSON's `data`
//! object.
//!
//! **Deliberately operates on raw `serde_json::Value`, never a typed Rust
//! struct.** The first version of this tool deserialized into
//! `CorpusRecordV1<EquipmentCacheData>` and re-serialized the whole record
//! -- which silently *dropped* every field that struct doesn't know about
//! (real, caught-in-review data loss: APG/ACG/Bestiary's `weight` field --
//! a different name than CRB's `weight_lbs` -- and PU's `equip_type`/
//! `plus` fields all vanished on the first run, reverted before commit).
//! Operating on `Value` and only ever inserting the 2 new keys means every
//! book-specific field this tool doesn't know about survives untouched,
//! by construction, regardless of what else diverges between books' schemas.
//!
//! Records whose `source.kind` is not `"lst_token"` (a `web_second_source`
//! or `same_book_fallback` record -- no raw LST line to enrich from) are
//! left untouched and counted separately, not treated as an error.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use codex::pcgen_import::lst_parser::equipment::parse_equipment_entries;
use serde_json::{json, Value};

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = env::var("PCGEN_DATA_ROOT") {
        return PathBuf::from(v);
    }
    let home = env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

fn find_equipment_json_files(book_dir: &Path) -> Vec<PathBuf> {
    let equipment_dir = book_dir.join("equipment");
    let mut out = Vec::new();
    if !equipment_dir.is_dir() {
        return out;
    }
    let mut stack = vec![equipment_dir];
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
    out
}

enum Outcome {
    Enriched,
    NoLstCitation,
    AlreadyEnriched,
    CitationMiss(String),
    MergedEntryMismatch(String),
}

fn enrich_one(path: &Path, data_root: &Path) -> Outcome {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let mut root: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {path:?} as JSON: {e}"));

    {
        let data = root.get("data").unwrap_or_else(|| panic!("{path:?}: no top-level \"data\" object"));
        if data.get("raw_tokens").is_some() || data.get("raw_bonus_chains").is_some() {
            return Outcome::AlreadyEnriched;
        }
    }

    let source = root["source"].clone();
    if source.get("kind").and_then(Value::as_str) != Some("lst_token") {
        return Outcome::NoLstCitation;
    }
    let lst_rel_path = source["path"].as_str().expect("lst_token source must carry a path").to_string();
    let line = source["line"].as_u64().expect("lst_token source must carry a line") as usize;
    let record_key = source.get("record_key").and_then(Value::as_str).unwrap_or("<unknown>").to_string();

    let lst_full_path = data_root.join(&lst_rel_path);
    let Ok(lst_text) = fs::read_to_string(&lst_full_path) else {
        return Outcome::CitationMiss(format!("cited LST file not found: {lst_full_path:?}"));
    };
    let parsed = parse_equipment_entries(&lst_rel_path, &lst_text);
    let Some(raw_record) = parsed.entries.iter().find(|e| e.header_line_number == line) else {
        return Outcome::CitationMiss(format!("no record at {lst_rel_path}:{line} (record_key={record_key:?})"));
    };

    // `SD31-W4-INTEGRATE-001` (`OPEN-ISSUES.md` row 48/49): `open_record`'s
    // same-name merge (this parser's own documented handling of PCGen
    // restating one logical item across multiple lines) also fires on
    // rows that are NOT restatements at all -- two separate `.lst` rows
    // sharing a base template name (a duplicate corpus row, or
    // `extract_record_name` returning a `.COPY=` row's BASE name rather
    // than its own distinct post-`.COPY=` name) merge into one parsed
    // entry whose token list can carry tokens that never appear on the
    // SPECIFIC cited line at all -- confirmed for `bastard_s_sting`
    // (line 447, a bare `.COPY=` row with no tokens of its own, shipped
    // an unrelated `EQMOD:Material ~ Steel` and a duplicated
    // `VISIBLE:YES`), `mountain_pattern_armor` (every token doubled from
    // a second, near-identical row at line 46) and `hunter_s_stand`
    // (three genuinely distinct `.COPY=` items' tokens merged into one).
    // Guard: every token this enrichment is about to ship must be
    // byte-present on the record's OWN cited line -- the same invariant
    // `corpus_literal_sweep` independently re-checks after the fact, run
    // here BEFORE writing so a merged-entry defect is refused at the
    // source rather than caught downstream. A record that fails this
    // check is left un-enriched (`MergedEntryMismatch`, counted and
    // reported, never silently dropped) rather than shipping a citation
    // whose tokens the cited line does not actually carry.
    let cited_line_text = lst_text.lines().nth(line.saturating_sub(1)).unwrap_or("");
    for token in &raw_record.tokens {
        let rendered = format!("{}:{}", token.key, token.value);
        if !cited_line_text.contains(&rendered) {
            return Outcome::MergedEntryMismatch(format!(
                "{lst_rel_path}:{line} (record_key={record_key:?}): token {rendered:?} not byte-present on the cited line -- likely a same-name merge with a different row"
            ));
        }
    }

    let raw_tokens: Vec<Value> = raw_record
        .tokens
        .iter()
        .map(|t| json!({ "key": t.key, "value": t.value }))
        .collect();
    let raw_bonus_chains: Vec<Value> =
        raw_record.bonus_chains.iter().map(|b| json!({ "qualifiers": b.qualifiers })).collect();

    let data_obj = root
        .get_mut("data")
        .and_then(Value::as_object_mut)
        .expect("\"data\" must be a JSON object");
    data_obj.insert("raw_tokens".to_string(), Value::Array(raw_tokens));
    data_obj.insert("raw_bonus_chains".to_string(), Value::Array(raw_bonus_chains));

    let new_json = serde_json::to_string_pretty(&root).expect("serialize enriched record");
    fs::write(path, new_json + "\n").unwrap_or_else(|e| panic!("write {path:?}: {e}"));
    Outcome::Enriched
}

fn main() {
    let data_root = pcgen_data_root();
    let corpus_root = PathBuf::from("data/corpus");
    let books = [
        "core_rulebook",
        "advanced_players_guide",
        "advanced_class_guide",
        "beastiary",
        "advanced_race_guide",
        "pathfinder_unchained",
        // SD-31 SD31-E6-F5-001: Ultimate Equipment's cache
        // (`data/corpus/ultimate_equipment/equipment/*.json`,
        // `gen_cache_ultimate_equipment`) is real `lst_token` records now
        // and needs the same `raw_tokens`/`raw_bonus_chains` enrichment
        // every other equipment book gets -- omitting it here would
        // silently leave every new UE record at its thin
        // KEY:-token-only fallback (`corpus_loader.rs`'s
        // `equipment_record_from_json`), never wiring a single
        // `BONUS:STAT` effect despite the cache existing on disk.
        "ultimate_equipment",
        // SD-31 SD31-E6-F5-002: `gen_cache_equipment_gap` (`cache_gen::
        // equipment_gap`) landed real `lst_token` equipment/equipment_modifier
        // records for these 4 books' `not-ingested` gap residue this cycle --
        // same reasoning as the `ultimate_equipment` entry above, these need
        // the same enrichment pass or their new records stay at the thin
        // KEY:-token-only fallback.
        "ultimate_combat",
        "ultimate_intrigue",
        "ultimate_psionics",
        "ultimate_wilderness",
    ];

    let mut total_enriched = 0u32;
    let mut total_no_citation = 0u32;
    let mut total_already = 0u32;
    let mut misses: Vec<String> = Vec::new();
    let mut merged_entry_mismatches: Vec<String> = Vec::new();

    for book in books {
        let book_dir = corpus_root.join(book);
        if !book_dir.is_dir() {
            continue;
        }
        let files = find_equipment_json_files(&book_dir);
        let mut book_enriched = 0u32;
        for file in &files {
            match enrich_one(file, &data_root) {
                Outcome::Enriched => {
                    total_enriched += 1;
                    book_enriched += 1;
                }
                Outcome::NoLstCitation => total_no_citation += 1,
                Outcome::AlreadyEnriched => total_already += 1,
                Outcome::CitationMiss(msg) => misses.push(format!("{}: {}", file.display(), msg)),
                Outcome::MergedEntryMismatch(msg) => {
                    merged_entry_mismatches.push(format!("{}: {}", file.display(), msg))
                }
            }
        }
        eprintln!("{book}: {} equipment files scanned, {book_enriched} enriched", files.len());
    }

    eprintln!(
        "\nenrich_equipment_raw_tokens: {total_enriched} enriched, {total_no_citation} no-LST-citation (untouched), {total_already} already-enriched, {} citation misses, {} merged-entry mismatches (left un-enriched)",
        misses.len(),
        merged_entry_mismatches.len()
    );
    if !misses.is_empty() {
        eprintln!("\nCitation misses (not enriched, real gaps to investigate):");
        for miss in &misses {
            eprintln!("  {miss}");
        }
    }
    if !merged_entry_mismatches.is_empty() {
        eprintln!(
            "\nMerged-entry mismatches (parse_equipment_entries's same-name merge pulled in a \
             token from a DIFFERENT row than the one cited -- left un-enriched, OPEN-ISSUES.md row 48/49):"
        );
        for mismatch in &merged_entry_mismatches {
            eprintln!("  {mismatch}");
        }
    }
}
