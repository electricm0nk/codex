//! SD-27 Cycle E2.2 — Shape B v1 key-set + key-order conformance test for
//! the Pathfinder Unchained (PU) corpus cache at
//! `data/corpus/pathfinder_unchained/{feat,equipment}/**/*.json`, generated
//! by `cargo run --bin gen_book_cache -- pathfinder_unchained`
//! (`src/bin/gen_book_cache.rs`). Reads only the already-generated
//! files — this test does not require a live PCGen corpus checkout,
//! mirroring `tests/sd26_cache_acg.rs`'s own "generated-cache
//! conformance, not live-corpus" scope, adapted to Shape B **v1**'s
//! additive `license`/`pi_field`/`pi_marker` fields
//! (`src/rules_core/shape_b_v1.rs`).
//!
//! Real, independently re-verified ceilings this test asserts against
//! (this cycle, directly against the live PCGen corpus checkout, not
//! taken on faith): 17/17 real, distinct feat records in `pu_feats.lst`
//! (18 non-comment rows minus 1 `.MOD` modifier of an existing APG feat)
//! and 42/42 real `KEY:`-bearing equipment-modifier records in
//! `pu_equipmods.lst`. `pu_spells.lst` has 0 active records (every row is
//! `#`-commented out) — this book adds no new spells, so there is no
//! `data/corpus/pathfinder_unchained/spell/` directory at all, an honest
//! absence rather than a missed cycle.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn cache_dir() -> PathBuf {
    repo_root().join("data/corpus/pathfinder_unchained")
}

/// Walks `data/corpus/pathfinder_unchained/<kind>/` **recursively**.
///
/// Recursion is load-bearing, not defensive tidying. `gen_book_cache` writes a
/// record either flat (`<kind>/<slug>.json`) or category-nested
/// (`<kind>/<category>/<slug>.json`), and a record can move between the two
/// layouts without its content changing at all. `b34bf2b4f0` did exactly that
/// to PU's four `+0 ABP (Enhancement to ...)` equipmods, relocating them from
/// `equipment/` to `equipment/equipmods/`; a flat `read_dir` then stopped
/// seeing them and this file's ceilings read 38/42 and 3/7.
///
/// **Those four records were never deleted from the corpus.** What
/// `e5fd8dddb1` deleted was the four *stale flat duplicates* a regen had
/// re-created beside the relocated originals. Repinning the ceilings to the
/// short counts would therefore have deleted a real assertion — the loader was
/// wrong, not the corpus — so the loader is what changes here. The ceilings
/// stay at their independently re-verified corpus values of 42 and 7.
fn load_all(kind: &str) -> Vec<(PathBuf, Value)> {
    let dir = cache_dir().join(kind);
    let mut out = Vec::new();
    load_all_under(&dir, &mut out);
    out
}

fn load_all_under(dir: &Path, out: &mut Vec<(PathBuf, Value)>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display())) {
        let path = entry.unwrap().path();
        if path.is_dir() {
            load_all_under(&path, out);
        } else if path.extension().map(|e| e == "json").unwrap_or(false) {
            let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            let value: Value =
                serde_json::from_str(&text).unwrap_or_else(|e| panic!("invalid JSON in {}: {e}", path.display()));
            out.push((path, value));
        }
    }
}

const ALLOWED_SOURCE_KINDS: &[&str] =
    &["lst_token", "lst_inherited_copy", "lst_corrected_ingest", "web_second_source", "same_book_fallback"];

/// Asserts the record matches Shape B **v1**'s shape: every v0 field
/// (`population`/`completeness`/`ingested_at`/`data`/`source`) plus the 3
/// additive license fields (`license`/`pi_field`/`pi_marker`), all
/// populated — this cycle emits v1 records directly (per
/// `technical-design.md §2`: "The 2 in-scope future-state books' cycles
/// (2.1-2.2) emit v1 records directly"), never the bare v0 shape the 4
/// in-scope books started from.
fn assert_shape_b_v1_record(path: &Path, record: &Value) {
    let obj = record.as_object().unwrap_or_else(|| panic!("{}: not a JSON object", path.display()));

    assert_eq!(
        obj.get("population").and_then(Value::as_str),
        Some("in_scope"),
        "{}: population must be in_scope (PU is one of SD-27's 2 in-scope future-state books)",
        path.display()
    );

    let completeness = obj.get("completeness").and_then(Value::as_str);
    assert!(
        matches!(completeness, Some("chassis_only") | Some("chassis_plus_extract") | Some("full")),
        "{}: completeness must be one of the 3 Shape B discriminants, got {completeness:?}",
        path.display()
    );

    let ingested_at = obj.get("ingested_at").and_then(Value::as_str).unwrap_or_else(|| panic!("{}: missing ingested_at", path.display()));
    assert!(
        ingested_at.contains('T') && ingested_at.ends_with('Z'),
        "{}: ingested_at {ingested_at:?} is not ISO-8601 UTC-shaped",
        path.display()
    );

    assert!(obj.contains_key("data"), "{}: missing data", path.display());

    let source = obj.get("source").and_then(Value::as_object).unwrap_or_else(|| panic!("{}: missing source object", path.display()));
    let kind = source.get("kind").and_then(Value::as_str).unwrap_or_else(|| panic!("{}: source missing kind", path.display()));
    assert!(ALLOWED_SOURCE_KINDS.contains(&kind), "{}: source.kind {kind:?} is not an allowed discriminant", path.display());
    assert_eq!(kind, "lst_token", "{}: every PU record in this cache has a real, direct LST citation", path.display());
    for field in ["path", "sha256", "line", "record_key"] {
        assert!(source.contains_key(field), "{}: source missing required field {field}", path.display());
    }
    let line = source.get("line").and_then(Value::as_u64).unwrap_or(0);
    assert!(line > 0, "{}: source.line must be a real (>0) line number, not a placeholder", path.display());
    let sha256 = source.get("sha256").and_then(Value::as_str).unwrap_or_default();
    assert_eq!(sha256.len(), 64, "{}: source.sha256 must be a real 64-hex-char SHA-256 digest", path.display());
    assert!(sha256.chars().all(|c| c.is_ascii_hexdigit()), "{}: source.sha256 must be hex", path.display());

    // Shape B v1's additive fields — never absent, never defaulted-away
    // (decisions.md §17's "every record has a license field" output
    // requirement, checkable from this cycle onward per
    // src/rules_core/shape_b_v1.rs's own `validate_license`).
    let license = obj.get("license").and_then(Value::as_str);
    assert!(
        matches!(license, Some("OGL") | Some("PI") | Some("PI-REDACTED")),
        "{}: license must be one of the 3 Shape B v1 discriminants, got {license:?} (v1 records must never ship with license: null)",
        path.display()
    );
    if license == Some("PI") || license == Some("PI-REDACTED") {
        assert!(obj.get("pi_field").and_then(Value::as_str).is_some(), "{}: PI/PI-REDACTED record must carry pi_field", path.display());
    }
    if license == Some("PI-REDACTED") {
        assert_eq!(
            obj.get("pi_marker").and_then(Value::as_str),
            Some("redacted"),
            "{}: PI-REDACTED record must carry pi_marker: \"redacted\"",
            path.display()
        );
    }
}

#[test]
fn feat_cache_has_all_17_real_distinct_pu_feat_records() {
    let records = load_all("feat");
    assert_eq!(
        records.len(),
        17,
        "real, independently re-verified pu_feats.lst count: 18 non-comment rows minus 1 .MOD modifier of an existing APG feat"
    );

    let mut seen_keys = HashSet::new();
    for (path, record) in &records {
        assert_shape_b_v1_record(path, record);
        let data = &record["data"];
        let key = data["key"].as_str().unwrap().to_string();
        assert!(seen_keys.insert(key.clone()), "{}: duplicate feat key {key}", path.display());
        assert_eq!(record["source"]["path"], "pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_feats.lst");
        assert!(!data["name"].as_str().unwrap().is_empty(), "{}: feat name must not be empty", path.display());
        assert!(!data["description"].as_str().unwrap_or_default().is_empty(), "{}: every real PU feat carries a DESC: token", path.display());
    }
}

#[test]
fn feat_cache_covers_all_4_corpus_blocks() {
    // The 4 real ###Block: groupings this book's corpus carries (see
    // rules_tables::pathfinder_unchained::feat_tables's own doc comment).
    let records = load_all("feat");
    let mut categories: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for (_, record) in &records {
        let category = record["data"]["category"].as_str().unwrap().to_string();
        *categories.entry(category).or_insert(0) += 1;
    }
    assert_eq!(categories.get("Alignment").copied(), Some(9), "9 real Champion of <alignment> feats");
    assert_eq!(categories.get("CombatStamina").copied(), Some(3), "Combat Stamina, Extra Stamina, Push the Limits");
    assert_eq!(categories.get("WoundThreshold").copied(), Some(3), "Critical Cure, Endurance, Twist the Knife");
    assert_eq!(categories.get("General").copied(), Some(2), "Extra Unchained Rogue Talent, Signature Skill");
}

#[test]
fn feat_cache_line_citations_match_the_real_corpus_exactly() {
    // Real pu_feats.lst line numbers (independently confirmed this cycle
    // by direct read of the live PCGen corpus checkout).
    let expected_lines: &[(&str, u64)] = &[
        ("Champion of Anarchy", 6),
        ("Champion of Balance", 7),
        ("Champion of Destruction", 8),
        ("Champion of Freedom", 9),
        ("Champion of Grace", 10),
        ("Champion of Malevolence", 11),
        ("Champion of Righteousness", 12),
        ("Champion of Tranquility", 13),
        ("Champion of Tyranny", 14),
        ("Combat Stamina", 18),
        ("Extra Stamina", 19),
        ("Push the Limits", 20),
        ("Critical Cure", 25),
        ("Endurance", 26),
        ("Twist the Knife", 27),
        ("Extra Unchained Rogue Talent", 29),
        ("Signature Skill", 32),
    ];
    assert_eq!(expected_lines.len(), 17);
    let records: std::collections::HashMap<String, Value> =
        load_all("feat").into_iter().map(|(_, v)| (v["data"]["key"].as_str().unwrap().to_string(), v)).collect();
    for (key, line) in expected_lines {
        let record = records.get(*key).unwrap_or_else(|| panic!("missing feat record {key}"));
        assert_eq!(record["source"]["line"].as_u64(), Some(*line), "{key}: real corpus line citation mismatch");
    }
}

#[test]
fn feat_cache_excludes_the_dot_mod_record_without_special_casing_a_fake_feat() {
    // "Extra Rogue Talent" (no "Unchained") is the APG base feat this
    // book's .MOD row patches; it must never appear as its own PU feat
    // record — that would fabricate a feat this book's corpus never
    // defines.
    let records = load_all("feat");
    for (_, record) in &records {
        assert_ne!(record["data"]["key"], "Extra Rogue Talent", "the .MOD record must never surface as a fabricated new feat");
        assert_ne!(record["data"]["key"], "CATEGORY=FEAT|Extra Rogue Talent.MOD");
    }
}

#[test]
fn equipment_cache_has_all_42_real_pu_equipmods_records() {
    let records = load_all("equipment");
    assert_eq!(records.len(), 42, "real, independently re-verified KEY:-bearing pu_equipmods.lst record count");

    let mut seen_keys = HashSet::new();
    for (path, record) in &records {
        assert_shape_b_v1_record(path, record);
        let data = &record["data"];
        let key = data["key"].as_str().unwrap().to_string();
        assert!(seen_keys.insert(key.clone()), "{}: duplicate equipment key {key}", path.display());
        assert_eq!(record["source"]["path"], "pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_equipmods.lst");
        assert_eq!(data["category"], "equipmods");
        assert!(!data["description"].as_str().unwrap_or_default().is_empty(), "{}: every real pu_equipmods.lst record carries a DESC: token", path.display());
        // No COST:/WT: token exists anywhere in pu_equipmods.lst
        // (independently confirmed this cycle) — cost_gp/weight_lbs must
        // stay an honest null, never a fabricated number.
        assert!(data["cost_gp"].is_null(), "{}: pu_equipmods.lst carries no COST: token; cost_gp must stay null", path.display());
        assert!(data["weight_lbs"].is_null(), "{}: pu_equipmods.lst carries no WT: token; weight_lbs must stay null", path.display());
    }
}

#[test]
fn equipment_cache_plus_zero_records_have_no_fabricated_plus_value() {
    // The 7 real "+0" records (one per ladder) genuinely carry no PLUS:
    // token in the corpus — `plus` must stay null, never a fabricated
    // `0`. 4 ABP Enhancement ladders (Weapon/Ammunition/Armor/Shield) + 3
    // Attunement ladders (Weapon/Armor/Shield) = 7.
    let records = load_all("equipment");
    let zero_records: Vec<_> = records.iter().filter(|(_, r)| r["data"]["name"].as_str().unwrap().starts_with("+0 ")).collect();
    assert_eq!(zero_records.len(), 7, "7 real +0 records: 4 ABP Enhancement ladders + 3 Attunement ladders");
    for (path, record) in zero_records {
        assert!(record["data"]["plus"].is_null(), "{}: +0 record must have plus: null, not a fabricated 0", path.display());
    }
}

#[test]
fn equipment_cache_covers_all_4_abp_slot_types_and_3_attunement_slot_types() {
    let records = load_all("equipment");
    let mut abp_types: HashSet<String> = HashSet::new();
    let mut attune_count = 0u32;
    for (_, record) in &records {
        let name = record["data"]["name"].as_str().unwrap();
        let equip_type = record["data"]["equip_type"].as_str().unwrap().to_string();
        if name.contains("ABP (Enhancement") {
            abp_types.insert(equip_type);
        } else if name.contains("Attuned") {
            attune_count += 1;
        }
    }
    let expected: HashSet<String> =
        ["Weapon", "Ammunition", "Armor", "Shield"].iter().map(|s| s.to_string()).collect();
    assert_eq!(abp_types, expected, "4 ABP Enhancement slot types, each with a full 0-5 ladder (24 records)");
    assert_eq!(attune_count, 18, "3 Attunement slot types (Weapon/Armor/Shield), each with a full 0-5 ladder");
}
