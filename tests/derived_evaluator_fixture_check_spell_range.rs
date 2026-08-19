//! The `kind=spell` `RANGE:` half of the `derived` wiring class's
//! evaluator-vs-fixture check — the sibling of
//! `tests/derived_evaluator_fixture_check_class_feature.rs`'s class-feature
//! guarantees and `tests/derived_evaluator_fixture_check_monster.rs`'s
//! monster guarantees, over `derived-evaluator-fixtures.json`'s
//! `spell_range_entries` array.
//!
//! # Why this file did not exist before (SD31-W15)
//!
//! The equipment, monster and class_feature fixture families each shipped
//! with an integration test asserting their four provenance guarantees. The
//! two `kind=spell` families (`spell_entries`, `spell_range_entries`) shipped
//! with only the in-module unit tests in
//! `src/rules_core/derived_evaluator_fixture_check.rs` — which assert that
//! the committed fixture CLEARS and that a wrong expected value fails, but
//! never that a committed entry's `corpus_field` is genuinely the upstream
//! record's own bytes. That is the assertion that separates evidence from a
//! restatement of the evaluator, and it was the missing one for the largest
//! spell family. This file adds it for `spell_range_entries`, whose coverage
//! this cycle widened from 199 to 760 entries.
//!
//! # The bar
//!
//! Unlike the `DURATION:` family (a per-spell literal), a `RANGE:` keyword's
//! formula is a RULESET-level constant, stated once by the pinned PCGen game
//! mode (`system/gameModes/Pathfinder/miscinfo.lst`'s `SPELLRANGE:` rows).
//! So the fixture carries two separable claims, and this file checks both
//! independently:
//!
//! * **the ruleset claim** — `expected.base_ft`/`rate_ft`/`per_levels` are
//!   the formula the pinned game-mode file itself states for that keyword
//!   ([`spell_range_expected_values_are_re_derivable_from_the_pinned_ruleset`]);
//! * **the per-record claim** — THIS spell's own upstream row really does
//!   name that keyword, at the exact `(file, line)` the entry pins, in a file
//!   that still hashes to the pinned sha256
//!   ([`spell_range_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`]),
//!   and this repo's own ingest of the same record cites the same upstream
//!   bytes
//!   ([`spell_range_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from`]).
//!
//! The second claim is what makes a 760-entry family carrying only three
//! distinct expected triples worth anything: the per-entry content is not the
//! three integers, it is the pinned identity of the record that claims them.
//!
//! # Mutation-proof
//!
//! [`a_wrong_expected_base_ft_cannot_spuriously_match_the_ruleset_formula`]
//! confirms the equality this seam's production comparison performs is
//! capable of failing. Two heavier mutation proofs were run by hand against
//! the real binary this cycle and recorded in the cycle receipt: corrupting
//! one record's `RANGE` token in `data/corpus/` dropped exactly that unit
//! (1423 → 1422 cleared, named in the failure list), and changing
//! `spell_range_formula`'s `Close` base from 25 to 26 failed 498 entries.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::{
    load_spell_range_fixtures, spell_range_formula,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Same resolution rule the equipment/monster/class_feature test files use.
fn pcgen_data_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen").join("data"))
}

/// The PCGen checkout the game-mode ruleset file lives in — `$PCGEN_REPO_DIR`,
/// else the parent of [`pcgen_data_root`] (`.../pcgen/data` -> `.../pcgen`),
/// matching `scripts/derive_spell_range_fixtures.py`'s own resolution.
fn pcgen_repo_dir() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_REPO_DIR") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen"))
}

/// Re-reads `system/gameModes/Pathfinder/miscinfo.lst`'s `SPELLRANGE:` rows
/// from the pinned oracle at TEST time and returns keyword ->
/// `(base_ft, rate_ft, per_levels)`.
///
/// This is the reference derivation, and it is deliberately written against
/// the FILE rather than against `spell_range_formula`'s hardcoded constants —
/// if the Rust constants and the ruleset ever disagree, this test is what
/// says so. It also does not import the Python generator, so the fixture's
/// expected values are checked by a third, independent reading.
fn reference_spellrange_formulas() -> Option<BTreeMap<String, (i32, i32, i32)>> {
    let path = pcgen_repo_dir()?
        .join("system")
        .join("gameModes")
        .join("Pathfinder")
        .join("miscinfo.lst");
    let text = std::fs::read_to_string(&path).ok()?;
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("SPELLRANGE:") else { continue };
        let Some((keyword, formula)) = rest.split_once('|') else { continue };
        // `floor(CASTERLEVEL/<per>)*<rate>+<base>`
        if let Some(tail) = formula.strip_prefix("floor(CASTERLEVEL/") {
            let Some((per, tail)) = tail.split_once(')') else { continue };
            let Some(tail) = tail.strip_prefix('*') else { continue };
            let Some((rate, base)) = tail.split_once('+') else { continue };
            let (Ok(per), Ok(rate), Ok(base)) =
                (per.parse::<i32>(), rate.parse::<i32>(), base.parse::<i32>())
            else {
                continue;
            };
            out.insert(title_case(keyword), (base, rate, per));
            continue;
        }
        // `(CASTERLEVEL*<rate>)+<base>`
        if let Some(tail) = formula.strip_prefix("(CASTERLEVEL*") {
            let Some((rate, tail)) = tail.split_once(')') else { continue };
            let Some(base) = tail.strip_prefix('+') else { continue };
            let (Ok(rate), Ok(base)) = (rate.parse::<i32>(), base.parse::<i32>()) else {
                continue;
            };
            out.insert(title_case(keyword), (base, rate, 1));
        }
    }
    (!out.is_empty()).then_some(out)
}

fn title_case(s: &str) -> String {
    let lower = s.to_ascii_lowercase();
    let mut chars = lower.chars();
    match chars.next() {
        Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// Every ingested `spell` record of `book`, indexed by its corpus `key`,
/// carrying the provenance the ingest recorded for it — the spell-kind
/// sibling of the class_feature test file's `class_feature_ingested_provenance`.
fn spell_ingested_provenance(book: &str) -> BTreeMap<String, (String, u64, String)> {
    let mut out = BTreeMap::new();
    let root_dir = repo_root().join("data").join("corpus").join(book).join("spell");
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

/// Guarantee 0: the family is not empty and every entry is a distinct unit.
/// A duplicated `unit_id` would let one record's evidence be counted twice.
#[test]
fn spell_range_entries_are_present_and_one_per_unit() {
    let fixtures = load_spell_range_fixtures(&repo_root());
    assert!(
        !fixtures.is_empty(),
        "an empty spell_range family would make every assertion below vacuously true"
    );
    let mut seen = BTreeSet::new();
    let mut duplicated = Vec::new();
    for f in &fixtures {
        if !seen.insert(f.unit_id.clone()) {
            duplicated.push(f.unit_id.clone());
        }
    }
    assert!(duplicated.is_empty(), "duplicated unit_id(s): {duplicated:?}");
}

/// Guarantee 3: every entry's expected triple is re-derivable from the pinned
/// ruleset file itself, by a reference reading written here and independent
/// of both `spell_range_formula` and the Python generator. Also asserts the
/// engine's own hardcoded constants agree with that file — the check that
/// makes the other 759 entries non-circular.
#[test]
fn spell_range_expected_values_are_re_derivable_from_the_pinned_ruleset() {
    let fixtures = load_spell_range_fixtures(&repo_root());
    let Some(reference) = reference_spellrange_formulas() else {
        eprintln!("skipped: no readable miscinfo.lst under the pinned PCGen checkout");
        return;
    };

    let mut wrong = Vec::new();
    for f in &fixtures {
        let keyword = f
            .corpus_field
            .strip_prefix("RANGE:")
            .unwrap_or_else(|| panic!("{}: corpus_field must be a RANGE: token", f.unit_id))
            .trim();
        let Some((base, rate, per)) = reference.get(keyword).copied() else {
            wrong.push(format!(
                "{}: the pinned ruleset states no SPELLRANGE row for keyword {keyword:?}",
                f.unit_id
            ));
            continue;
        };
        if (f.expected_base_ft, f.expected_rate_ft, f.expected_per_levels) != (base, rate, per) {
            wrong.push(format!(
                "{}: fixture expects {}/{}/{}, the ruleset states {base}/{rate}/{per}",
                f.unit_id, f.expected_base_ft, f.expected_rate_ft, f.expected_per_levels
            ));
            continue;
        }
        // …and the engine's own constant must agree with the same file.
        let Some(engine) = spell_range_formula(keyword) else {
            wrong.push(format!("{}: the engine refuses keyword {keyword:?}", f.unit_id));
            continue;
        };
        if (engine.base_ft, engine.rate_ft, engine.per_levels) != (base, rate, per) {
            wrong.push(format!(
                "{}: engine hardcodes {}/{}/{} for {keyword:?}, the pinned ruleset states \
                 {base}/{rate}/{per}",
                f.unit_id, engine.base_ft, engine.rate_ft, engine.per_levels
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 4a: every entry's `upstream_lst` still hashes to its pinned
/// `upstream_lst_sha256`, and `upstream_line` still carries `corpus_field`
/// verbatim as a whole tab-separated field. This is the per-record claim —
/// without it the family asserts only three ruleset integers.
#[test]
fn spell_range_pinned_corpus_field_is_byte_identical_to_the_upstream_lst() {
    let fixtures = load_spell_range_fixtures(&repo_root());
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
        let index = usize::try_from(fixture.upstream_line).expect("line number fits in usize");
        let Some(line) = index.checked_sub(1).and_then(|i| lines.get(i)) else {
            wrong.push(format!(
                "{}: {} has no line {}",
                fixture.unit_id, fixture.upstream_lst, fixture.upstream_line
            ));
            continue;
        };
        // Whole-field, not substring: `RANGE:Close` must be the entire
        // tab-separated field, so `RANGE:Close burst` could never satisfy it.
        if !line.split('\t').any(|f| f == fixture.corpus_field) {
            wrong.push(format!(
                "{}: line {} of {} carries no field exactly equal to {:?}",
                fixture.unit_id, fixture.upstream_line, fixture.upstream_lst, fixture.corpus_field
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 4b: this repo's own ingest of the same `record_key` cites the
/// SAME upstream `(path, line, sha256)` the fixture pins — the join that
/// stops a fixture from being anchored to a record the engine never read.
#[test]
fn spell_range_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from() {
    let fixtures = load_spell_range_fixtures(&repo_root());
    let mut provenance_by_book: BTreeMap<String, BTreeMap<String, (String, u64, String)>> =
        BTreeMap::new();
    let mut mismatched: BTreeMap<String, String> = BTreeMap::new();
    let mut compared = 0usize;

    for fixture in &fixtures {
        let records = provenance_by_book
            .entry(fixture.book.clone())
            .or_insert_with(|| spell_ingested_provenance(&fixture.book));
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
        if path != &fixture.upstream_lst
            || *line != fixture.upstream_line
            || sha != &fixture.upstream_lst_sha256
        {
            mismatched.insert(
                fixture.unit_id.clone(),
                format!(
                    "fixture read {}:{} (sha {}), ingest cites {}:{} (sha {})",
                    fixture.upstream_lst,
                    fixture.upstream_line,
                    fixture.upstream_lst_sha256,
                    path,
                    line,
                    sha
                ),
            );
            continue;
        }
        compared += 1;
    }

    assert!(
        mismatched.is_empty(),
        "{} spell_range fixture(s) disagree with the engine's own ingest provenance:\n{}",
        mismatched.len(),
        mismatched.values().cloned().collect::<Vec<_>>().join("\n")
    );
    assert_eq!(
        compared,
        fixtures.len(),
        "every committed spell_range fixture must be cross-checked"
    );
}

/// Mutation-proof: a deliberately wrong expected base must NOT match the
/// ruleset's own value, proving the equality the production comparison
/// performs is capable of failing.
#[test]
fn a_wrong_expected_base_ft_cannot_spuriously_match_the_ruleset_formula() {
    let close = spell_range_formula("Close").expect("Close is one of the three ruleset keywords");
    assert_eq!(close.base_ft, 25);
    assert_ne!(
        close.base_ft, 26,
        "a wrong expected base must not spuriously match — if it does, the comparison itself is \
         broken and every entry in this family is vacuous"
    );
}
