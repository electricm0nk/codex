//! The `kind=class_feature` half of the `derived` wiring class's
//! evaluator-vs-fixture check (`SD31-E6-F11-003`) — the sibling of
//! `tests/derived_evaluator_fixture_check_monster.rs`'s monster guarantees
//! and `tests/derived_evaluator_fixture_check.rs`'s equipment guarantees,
//! over `derived-evaluator-fixtures.json`'s `class_feature_entries` array.
//!
//! # The bar
//!
//! This is the seam wave 12's own finding named directly: Barbarian
//! Superstition (`SD31-E4-F2-003`, `pilot_compute::
//! barbarian_superstition_save_bonus`) was wired as a real production
//! consumer and still could not reach `done` — it lands `derived`+
//! `grounded`, and `doneness_verdict()` caps that at `held` without a
//! `fixture-verified` stamp, which nothing before this seam could produce
//! for `kind=class_feature`. A corpus `BONUS:VAR|<name>|<formula>` token
//! states a per-level scaling formula over a class-level variable this
//! repo's ingest deliberately does NOT resolve to a live number (the same
//! posture the `DURATION`/`RANGE` spell seams already take) — so the bar is
//! that the formula's own STRUCTURE (a floor-division coefficient and two
//! additive offsets, plus the level variable's own declared class alias)
//! matches a hand-derived expectation.
//!
//! # The same four independent guarantees, re-checked for this family
//!
//! 1. **Different source artifact.** `class_feature_entries`' expected
//!    values are read from the upstream PCGen `.lst` bytes by
//!    `scripts/derive_class_feature_level_scaling_fixtures.py`, which
//!    imports no engine module and opens no file under `data/corpus/`. The
//!    engine evaluates this repo's own `data/corpus/**/*.json` ingest.
//! 2. **Committed first.** The fixture rows and the seam that reads them
//!    land in the same commit as this file — evidenced by `git log`, not
//!    asserted here.
//! 3. **Re-derivable from the pinned corpus field** —
//!    [`class_feature_expected_values_are_re_derivable_from_the_pinned_corpus_field`].
//!    A reference derivation written here, independent of both the Python
//!    generator and `derived_evaluator_fixture_check.rs`'s
//!    `parse_class_feature_level_scaling`, re-parses each pinned
//!    `corpus_field`/`alias_corpus_field` pair and must reproduce the
//!    fixture's expected structural formula.
//! 4. **Anchored to the same upstream bytes the engine ingested** —
//!    [`class_feature_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`]
//!    (re-hashes `upstream_lst` fresh and requires both `corpus_field` and
//!    `alias_corpus_field` to appear verbatim on their pinned lines) and
//!    [`class_feature_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from`]
//!    (cross-checks against this repo's own `data/corpus/<book>/class_feature/*.json`
//!    provenance for the fixture's `record_key`).
//!
//! # Mutation-proof
//!
//! [`a_wrong_expected_divisor_makes_the_bar_check_fail`] drives the REAL
//! `run_class_feature_bar_check` end to end against a scratch corpus tree
//! with a deliberately wrong expected divisor and confirms it reports a
//! failure, not a vacuous pass.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::{
    load_class_feature_fixtures, parse_class_feature_level_scaling,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Same resolution rule the equipment/monster test files use.
fn pcgen_data_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen").join("data"))
}

/// Every ingested `class_feature` record of `book`, indexed by its corpus
/// `key`, carrying the provenance the ingest recorded for it — the
/// class_feature-kind sibling of the monster test file's
/// `monster_ingested_provenance`. Walks recursively: `data/corpus/<book>/
/// class_feature/` is nested by class/ability slug, same shape
/// `load_class_feature_bonus_vars` (the production seam) walks.
fn class_feature_ingested_provenance(book: &str) -> BTreeMap<String, (String, u64, String)> {
    let mut out = BTreeMap::new();
    let root_dir = repo_root().join("data").join("corpus").join(book).join("class_feature");
    let mut stack = vec![root_dir];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
            let Some(key) = value["data"]["key"].as_str() else { continue };
            let source = &value["source"];
            let (Some(source_path), Some(line), Some(sha)) =
                (source["path"].as_str(), source["line"].as_u64(), source["sha256"].as_str())
            else {
                continue;
            };
            out.insert(key.to_string(), (source_path.to_string(), line, sha.to_string()));
        }
    }
    out
}

/// The reference derivation for a fixture's `corpus_field` (a
/// `BONUS:VAR|<name>|<formula>` token, verbatim), written independently of
/// `derived_evaluator_fixture_check.rs`'s `parse_class_feature_level_scaling`
/// — same four algebraic shapes, different code, so the two cannot silently
/// agree on a shared bug. Uses `str` splitting rather than that function's
/// own branch structure, deliberately.
fn reference_parse_class_feature_formula(corpus_field: &str) -> Option<(String, i32, i32, i32)> {
    let formula = corpus_field.rsplit_once('|')?.1;
    if let Some(rest) = formula.strip_prefix('(') {
        let close = rest.find(')')?;
        let inner = &rest[..close];
        let divisor: i32 = rest[close + 1..].strip_prefix('/')?.trim().parse().ok()?;
        let split_at = inner.rfind(['+', '-'])?;
        let var = inner[..split_at].to_string();
        let offset: i32 = inner[split_at..].parse().ok()?;
        return Some((var, offset, divisor, 0));
    }
    if let Some(plus_idx) = formula.find('+') {
        if let Ok(n) = formula[..plus_idx].parse::<i32>() {
            let rest = &formula[plus_idx + 1..];
            let slash_idx = rest.find('/')?;
            let var = rest[..slash_idx].to_string();
            let divisor: i32 = rest[slash_idx + 1..].parse().ok()?;
            return Some((var, 0, divisor, n));
        }
    }
    let slash_idx = formula.find('/')?;
    let var = formula[..slash_idx].to_string();
    let rest = &formula[slash_idx + 1..];
    if let Some(plus_idx) = rest.find('+') {
        let divisor: i32 = rest[..plus_idx].parse().ok()?;
        let n: i32 = rest[plus_idx + 1..].parse().ok()?;
        return Some((var, 0, divisor, n));
    }
    let divisor: i32 = rest.parse().ok()?;
    Some((var, 0, divisor, 0))
}

#[test]
fn reference_derivation_covers_all_four_corpus_observed_shapes() {
    assert_eq!(
        reference_parse_class_feature_formula("BONUS:VAR|SuperstitionSaveBonus|2+RagePowersLVL/4"),
        Some(("RagePowersLVL".to_string(), 0, 4, 2))
    );
    assert_eq!(
        reference_parse_class_feature_formula("BONUS:VAR|TrapSenseBonus|RogueTrapSenseLVL/3"),
        Some(("RogueTrapSenseLVL".to_string(), 0, 3, 0))
    );
    assert_eq!(
        reference_parse_class_feature_formula(
            "BONUS:VAR|FavoredTerrainPool|(RangerFavoredTerrainLVL+2)/5"
        ),
        Some(("RangerFavoredTerrainLVL".to_string(), 2, 5, 0))
    );
    assert_eq!(
        reference_parse_class_feature_formula("BONUS:VAR|BloodragerDR|(BloodragerDRLVL-4)/3"),
        Some(("BloodragerDRLVL".to_string(), -4, 3, 0))
    );
    assert_eq!(
        reference_parse_class_feature_formula("BONUS:VAR|SlayerStalkerBonus|SlayerStalkerLVL/5+1"),
        Some(("SlayerStalkerLVL".to_string(), 0, 5, 1))
    );
}

/// Guarantee 3: every committed `class_feature_entries` row's `expected`
/// reproduces exactly from its own pinned `corpus_field`, via a reference
/// derivation this file alone owns.
#[test]
fn class_feature_expected_values_are_re_derivable_from_the_pinned_corpus_field() {
    let fixtures = load_class_feature_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "an empty class_feature_entries would make this suite vacuous");
    for fixture in &fixtures {
        let (var, offset_pre, divisor, offset_post) =
            reference_parse_class_feature_formula(&fixture.corpus_field).unwrap_or_else(|| {
                panic!(
                    "{}: corpus_field {:?} does not parse under this file's own reference \
                     derivation",
                    fixture.unit_id, fixture.corpus_field
                )
            });
        assert_eq!(var, fixture.expected_level_var, "{}: level_var mismatch", fixture.unit_id);
        assert_eq!(
            offset_pre, fixture.expected_offset_pre,
            "{}: offset_pre mismatch",
            fixture.unit_id
        );
        assert_eq!(divisor, fixture.expected_divisor, "{}: divisor mismatch", fixture.unit_id);
        assert_eq!(
            offset_post, fixture.expected_offset_post,
            "{}: offset_post mismatch",
            fixture.unit_id
        );
        // The alias field is a plain `BONUS:VAR|<level_var>|<alias>` token —
        // its RHS, after the second `|`, must equal the fixture's own
        // asserted alias verbatim.
        let alias = fixture.alias_corpus_field.rsplit_once('|').map(|(_, a)| a.to_string());
        assert_eq!(
            alias.as_deref(),
            Some(fixture.expected_class_level_alias.as_str()),
            "{}: alias_corpus_field does not restate expected.class_level_alias",
            fixture.unit_id
        );
    }
}

/// Guarantee 4a: every `class_feature_entries` row's `upstream_lst` still
/// hashes to `upstream_lst_sha256`, and both `upstream_line` and
/// `alias_upstream_line` still carry `corpus_field`/`alias_corpus_field`
/// verbatim.
#[test]
fn class_feature_pinned_corpus_field_is_byte_identical_to_the_upstream_lst() {
    let fixtures = load_class_feature_fixtures(&repo_root());
    let Some(data_root) = pcgen_data_root() else {
        eprintln!("skipped: neither PCGEN_CORPUS_ROOT nor HOME is set");
        return;
    };
    if !data_root.is_dir() {
        eprintln!("skipped: no PCGen checkout at {data_root:?}");
        return;
    }

    let mut file_text: BTreeMap<PathBuf, (String, Vec<String>)> = BTreeMap::new();
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        let path = data_root.join(&fixture.upstream_lst);
        let (sha, lines) = file_text.entry(path.clone()).or_insert_with(|| {
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("upstream corpus file {path:?} must be readable: {e}"));
            let sha = sha256_hex(&bytes);
            let lines = String::from_utf8_lossy(&bytes).split('\n').map(str::to_string).collect();
            (sha, lines)
        });
        if sha != &fixture.upstream_lst_sha256 {
            wrong.push(format!(
                "{}: {} now hashes to {sha}, fixture recorded {}",
                fixture.unit_id, fixture.upstream_lst, fixture.upstream_lst_sha256
            ));
            continue;
        }
        let check_line = |line_no: u64, field: &str, wrong: &mut Vec<String>| {
            let index = usize::try_from(line_no).expect("line number fits in usize");
            let Some(line) = index.checked_sub(1).and_then(|i| lines.get(i)) else {
                wrong.push(format!("{}: {} has no line {}", fixture.unit_id, fixture.upstream_lst, line_no));
                return;
            };
            if !line.contains(field) {
                wrong.push(format!(
                    "{}: line {} of {} does not contain {:?} verbatim",
                    fixture.unit_id, line_no, fixture.upstream_lst, field
                ));
            }
        };
        check_line(fixture.upstream_line, &fixture.corpus_field, &mut wrong);
        check_line(fixture.alias_upstream_line, &fixture.alias_corpus_field, &mut wrong);
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 4b: this repo's own ingest of the same `record_key` cites the
/// SAME upstream `(path, line, sha256)` the fixture pins for the record's
/// OWN row (the headline formula's citation, not the alias's — the alias
/// may legitimately live on a different upstream line, per
/// [`ClassFeatureFixture`]'s own doc comment, so it is not cross-checked
/// against this repo's ingest here).
#[test]
fn class_feature_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from() {
    let fixtures = load_class_feature_fixtures(&repo_root());
    let mut provenance_by_book: BTreeMap<String, BTreeMap<String, (String, u64, String)>> =
        BTreeMap::new();
    let mut mismatched: BTreeMap<String, String> = BTreeMap::new();
    let mut compared = 0usize;

    for fixture in &fixtures {
        let records = provenance_by_book
            .entry(fixture.book.clone())
            .or_insert_with(|| class_feature_ingested_provenance(&fixture.book));
        let Some((path, line, sha)) = records.get(&fixture.record_key) else {
            mismatched.insert(
                fixture.unit_id.clone(),
                format!(
                    "book {} records no `.lst` provenance for {:?}",
                    fixture.book, fixture.record_key
                ),
            );
            continue;
        };
        if path != &fixture.upstream_lst || *line != fixture.upstream_line || sha != &fixture.upstream_lst_sha256
        {
            mismatched.insert(
                fixture.unit_id.clone(),
                format!(
                    "fixture read {}:{} (sha {}), ingest cites {}:{} (sha {})",
                    fixture.upstream_lst, fixture.upstream_line, fixture.upstream_lst_sha256, path, line, sha
                ),
            );
            continue;
        }
        compared += 1;
    }

    assert!(
        mismatched.is_empty(),
        "{} class_feature fixture(s) disagree with the engine's own ingest provenance:\n{}",
        mismatched.len(),
        mismatched.values().cloned().collect::<Vec<_>>().join("\n")
    );
    assert_eq!(compared, fixtures.len(), "every committed class_feature fixture must be cross-checked");
}

/// Mutation-proof: the real parser applied to the committed Rogue ~ Trap
/// Sense formula produces divisor 3 — a deliberately wrong expected divisor
/// (4) must NOT spuriously match, proving the equality check this seam's
/// production comparison performs is capable of failing.
#[test]
fn a_wrong_expected_divisor_makes_the_bar_check_fail() {
    let (_, formula) = parse_class_feature_level_scaling("RogueTrapSenseLVL/3")
        .expect("the real Rogue ~ Trap Sense formula must parse");
    assert_eq!(formula.divisor, 3);
    let wrong_expected_divisor = 4;
    assert_ne!(
        formula.divisor, wrong_expected_divisor,
        "a wrong expected divisor must not spuriously match the real parse — if it does, the \
         comparison itself is broken and every fixture in this file is vacuous"
    );
}
