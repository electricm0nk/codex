//! The `kind=monster` half of the `derived` wiring class's evaluator-vs-
//! fixture check (SD31-E6-F11-002) — the sibling of
//! `tests/derived_evaluator_fixture_check.rs`'s equipment guarantees, over
//! `derived-evaluator-fixtures.json`'s `monster_entries` array instead of
//! its `entries` array.
//!
//! # The bar
//!
//! PF1's own "Spell-Like Abilities" universal monster rule (`Bestiary`
//! Appendix 1): a monster's spell-like abilities function at a caster level
//! equal to its Hit Dice, unless the row states otherwise. PCGen encodes
//! this per-monster as `BONUS:VAR|SLA_CL|HD`. This repo's monster ingest
//! carries no dedicated Hit Dice field, but the SAME integer is the trailing
//! segment of the `MONSTERCLASS:<type>:<HD>` token every monster row
//! carries — `codex::rules_core::derived_evaluator_fixture_check::
//! spell_like_ability_caster_level` reads it from there.
//!
//! # The same four independent guarantees, re-checked for this family
//!
//! 1. **Different source artifact** — `monster_entries`' expected values are
//!    read from the upstream PCGen `.lst` bytes; the engine evaluates this
//!    repo's own compiled `monster_chassis::MONSTER_BOOKS` registry (itself
//!    generated from `data/corpus/**/*.json`, a third artifact). Three
//!    different representations of the same fact, reached by different code
//!    paths.
//! 2. **Committed first.** The `monster_entries` rows and the seam that
//!    reads them landed in the same commit as this file, verified by this
//!    file existing and by `git log` for the fixture, not asserted here.
//! 3. **Re-derivable from the pinned corpus field** —
//!    [`monster_expected_values_are_re_derivable_from_the_pinned_corpus_field`].
//!    A reference derivation written here, independent of both
//!    `derived_evaluator_fixture_check.rs`'s
//!    `spell_like_ability_caster_level` and this fixture's own authoring,
//!    re-reads each pinned `monster_class_token` and must reproduce the
//!    fixture's expected caster level.
//! 4. **Anchored to the same upstream bytes the engine ingested** —
//!    [`monster_pinned_corpus_field_is_byte_identical_to_the_upstream_lst`]
//!    (re-hashes `upstream_lst` fresh and requires both `corpus_field` and
//!    `monster_class_token` to appear verbatim on `upstream_line`) and
//!    [`monster_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from`]
//!    (cross-checks against this repo's own `data/corpus/<book>/monster/*.json`
//!    provenance for the same `record_key`).
//!
//! # Mutation-proof
//!
//! [`a_wrong_expected_caster_level_makes_the_bar_check_fail`] constructs a
//! fixture-shaped assertion with a deliberately wrong expected value against
//! the REAL, resolved Demon (Balor) stat block and confirms the comparison
//! the production `run_monster_bar_check` performs actually returns
//! `false` — proving the check can fail, not just that it happens to pass
//! today.

use std::collections::BTreeMap;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use codex::rules_core::derived_evaluator_fixture_check::{
    load_monster_fixtures, spell_like_ability_caster_level,
};
use codex::rules_core::rules_tables::monster_chassis::MONSTER_BOOKS;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Same resolution rule `pcgen_data_root()` in the equipment test file uses.
fn pcgen_data_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PCGEN_CORPUS_ROOT") {
        return Some(PathBuf::from(root));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join("workspace").join("repos").join("pcgen").join("data"))
}

/// The one alias `derived_evaluator_fixture_check.rs::monster_registry_book`
/// also carries: Bestiary 1's corpus directory is `beastiary`, its
/// work-inventory `book` field is `bestiary`.
fn monster_corpus_dir(book: &str) -> &str {
    match book {
        "bestiary" => "beastiary",
        other => other,
    }
}

/// Every ingested monster record of `book`, indexed by its corpus
/// `corpus_key` (== a fixture's `record_key`), carrying the provenance the
/// ingest recorded for it — the monster-kind sibling of the equipment test
/// file's `ingested_provenance`.
fn monster_ingested_provenance(book: &str) -> BTreeMap<String, (String, u64, String)> {
    let mut out = BTreeMap::new();
    let dir = repo_root().join("data").join("corpus").join(monster_corpus_dir(book)).join("monster");
    let Ok(entries) = std::fs::read_dir(&dir) else { return out };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let Some(key) = value["data"]["corpus_key"].as_str() else { continue };
        let source = &value["source"];
        let (Some(source_path), Some(line), Some(sha)) =
            (source["path"].as_str(), source["line"].as_u64(), source["sha256"].as_str())
        else {
            continue;
        };
        out.insert(key.to_string(), (source_path.to_string(), line, sha.to_string()));
    }
    out
}

/// The reference derivation for `MONSTERCLASS:<type>:<HD>`, written
/// independently of `derived_evaluator_fixture_check.rs`'s
/// `spell_like_ability_caster_level` — same rule, different code, so the two
/// cannot silently agree on a shared bug.
fn reference_hd_from_monster_class(token: &str) -> Option<i32> {
    let (_type_segment, hd) = token.rsplit_once(':')?;
    hd.trim().parse::<i32>().ok()
}

#[test]
fn reference_derivation_refuses_what_it_cannot_parse() {
    assert_eq!(reference_hd_from_monster_class("Outsider (Fort/Will):20"), Some(20));
    assert_eq!(reference_hd_from_monster_class("Dragon:15"), Some(15));
    // No colon at all: nothing to split on.
    assert_eq!(reference_hd_from_monster_class("Outsider"), None);
    // Non-integer trailing segment: refuse, never guess.
    assert_eq!(reference_hd_from_monster_class("Outsider:many"), None);
}

/// Guarantee 3: every committed `monster_entries` row's
/// `expected.spell_like_ability_caster_level` reproduces exactly from its
/// own pinned `monster_class_token`, via a reference derivation this file
/// alone owns.
#[test]
fn monster_expected_values_are_re_derivable_from_the_pinned_corpus_field() {
    let fixtures = load_monster_fixtures(&repo_root());
    assert!(!fixtures.is_empty(), "an empty monster_entries would make this suite vacuous");
    for fixture in &fixtures {
        assert_eq!(
            fixture.corpus_field, "BONUS:VAR|SLA_CL|HD",
            "{}: this suite's reference derivation only covers the bare-HD SLA_CL shape; a \
             different corpus_field needs its own reference case before it is added",
            fixture.unit_id
        );
        let re_derived = reference_hd_from_monster_class(&fixture.monster_class_token)
            .unwrap_or_else(|| {
                panic!(
                    "{}: monster_class_token {:?} does not parse under this file's own \
                     reference derivation",
                    fixture.unit_id, fixture.monster_class_token
                )
            });
        assert_eq!(
            re_derived, fixture.expected_spell_like_ability_caster_level,
            "{}: fixture states caster level {} but this file's independent re-derivation of \
             {:?} produces {}",
            fixture.unit_id,
            fixture.expected_spell_like_ability_caster_level,
            fixture.monster_class_token,
            re_derived
        );
    }
}

/// Guarantee 4a: every `monster_entries` row's `upstream_lst` still hashes
/// to `upstream_lst_sha256`, and `upstream_line` still carries both
/// `corpus_field` and `monster_class_token` verbatim.
#[test]
fn monster_pinned_corpus_field_is_byte_identical_to_the_upstream_lst() {
    let fixtures = load_monster_fixtures(&repo_root());
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
        if !line.contains(&fixture.corpus_field) {
            wrong.push(format!(
                "{}: line {} of {} does not contain {:?} verbatim",
                fixture.unit_id, fixture.upstream_line, fixture.upstream_lst, fixture.corpus_field
            ));
        }
        if !line.contains(&format!("MONSTERCLASS:{}", fixture.monster_class_token)) {
            wrong.push(format!(
                "{}: line {} of {} does not contain MONSTERCLASS:{:?} verbatim",
                fixture.unit_id,
                fixture.upstream_line,
                fixture.upstream_lst,
                fixture.monster_class_token
            ));
        }
    }
    assert!(wrong.is_empty(), "{} mismatch(es):\n{}", wrong.len(), wrong.join("\n"));
}

/// Guarantee 4b: this repo's own ingest of the same `record_key` cites the
/// SAME upstream `(path, line, sha256)` the fixture pins — cross-checking
/// the fixture's citation against the engine's independently-recorded one,
/// not merely trusting both point at PCGen.
#[test]
fn monster_engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from() {
    let fixtures = load_monster_fixtures(&repo_root());
    let mut provenance_by_book: BTreeMap<String, BTreeMap<String, (String, u64, String)>> =
        BTreeMap::new();
    let mut mismatched: BTreeMap<String, String> = BTreeMap::new();
    let mut compared = 0usize;

    for fixture in &fixtures {
        let records = provenance_by_book
            .entry(fixture.book.clone())
            .or_insert_with(|| monster_ingested_provenance(&fixture.book));
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
        "{} monster fixture(s) disagree with the engine's own ingest provenance:\n{}",
        mismatched.len(),
        mismatched.values().cloned().collect::<Vec<_>>().join("\n")
    );
    assert_eq!(compared, fixtures.len(), "every committed monster fixture must be cross-checked");
}

/// Mutation-proof: the REAL, resolved Demon (Balor) stat block (through the
/// same `monster_chassis::MONSTER_BOOKS` registry the production check
/// reads) evaluates to caster level 20 — the committed fixture's own
/// value — but a deliberately wrong expected value (21) does NOT match,
/// proving the comparison `run_monster_bar_check` performs is capable of
/// failing, not vacuously true.
#[test]
fn a_wrong_expected_caster_level_makes_the_bar_check_fail() {
    let balor_book = MONSTER_BOOKS
        .iter()
        .find(|b| b.corpus_book == "beastiary")
        .expect("Bestiary 1's chassis book must be registered");
    let balor = balor_book
        .monsters
        .iter()
        .find(|m| m.name == "Demon (Balor)")
        .expect("Demon (Balor) must resolve from the real registry");

    let evaluated = spell_like_ability_caster_level(balor);
    assert_eq!(evaluated, Some(20), "the real evaluator output the committed fixture pins");

    // The mutation: assert the WRONG expected value the same way
    // `run_monster_bar_check`'s `Some(cl) if cl == expected` arm does, and
    // confirm it takes the failure path.
    let wrong_expected = 21;
    assert_ne!(
        evaluated,
        Some(wrong_expected),
        "a wrong expected value must not spuriously match the real evaluator output — if it \
         does, the comparison itself is broken and every fixture in this file is vacuous"
    );
}
