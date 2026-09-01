//! `class_feature`'s Product-Identity regression gate — SD30-E3-F4.
//!
//! # Why this suite exists
//!
//! Mirrors `tests/sd29_declared_product_identity_in_shipped_race_traits.rs`
//! **exactly in shape**: it reads shipped `data/corpus/*/class_feature/**.json`
//! files, not source `.lst` rows, so both this test and the real player-facing
//! output read the same bytes. `scripts/verify.sh`'s `pi-sweep` stage and
//! `docs/governance/ogl-pi-blacklist.md` are both term-blacklist mechanisms;
//! neither reads the corpus's own `NAMEISPI`/`DESCISPI` declarations. Reading
//! a declaration and scanning for undeclared terms are different questions
//! (`decisions.md §53.1` — "the two are now a union") and need a sibling
//! check, not a merge into either existing mechanism. This suite is that
//! sibling check, scoped to `class_feature`.
//!
//! # The empty-target trap
//!
//! `class_feature`'s corpus roster is 23 books wide by inventory count, but
//! at the time this suite was written only ONE of them
//! (`pathfinder_unchained`, landed by SD-27's `ingest_pu_classes.rs`) has an
//! actual `data/corpus/<book>/class_feature/` directory on disk — the other
//! 22 are Epic 6 (`SD-31-corpus-closure-grind`'s chassis-sweep), not yet run.
//! A suite that walks an empty or near-empty directory tree and finds zero
//! offenders proves nothing by construction — this repo has shipped that
//! exact defect three times (`state-goals-and-lessons.md` §3.1: "a gate that
//! cannot fail"). Three independent guards close that hole here:
//!
//! 1. `shipped_class_feature_records()` itself asserts it found at least one
//!    record — if every `class_feature` directory vanished (a corpus-drift
//!    regression) both tests below would fail loudly instead of vacuously
//!    passing on an empty iterator.
//! 2. At the time this suite was written (2026-08-14, only 1 of 23 in-scope
//!    books ingested) `class_feature`'s live corpus carried **zero**
//!    `DESCISPI:YES` declarations — since then five more books landed
//!    (SD-31/SD-32) and the live count is 402, all correctly redacted
//!    (re-derived at the top of `every_class_feature_description_...`,
//!    printed every run — do not treat either number as pinned; re-derive
//!    it, `decisions.md §12 L2`). This suite does NOT assert `declared > 0`
//!    the way the race-trait one does, because a book roster that is still
//!    filling in could legitimately swing the live count back toward zero
//!    (a fully-DESCISPI-clean book landing last) without that being a
//!    regression — asserting a floor here would make this suite fail for
//!    the wrong reason. Instead the live count is stated explicitly
//!    (`eprintln!`) every run rather than silently varying unnoticed, and
//!    guard 3 below proves the detector itself is live regardless of what
//!    the corpus currently contains.
//! 3. `the_leak_detectors_actually_fire_on_a_planted_leak_and_clear_on_a_
//!    redacted_row` calls the exact same `name_leak`/`description_leak`
//!    functions the two corpus-scanning tests use, against synthetic rows
//!    that plant a leak and rows that ship it correctly redacted/dropped —
//!    proving the detection logic itself can both fail and pass, independent
//!    of whatever the live corpus happens to contain right now
//!    (`state-goals-and-lessons.md` §3.1: "prove new instruments fail by
//!    corrupting input, before trusting a pass").
//!
//! The moment any book's Epic-6-successor ingest lands a `class_feature`
//! record whose corpus row declares `NAMEISPI:YES` (still shipped) or
//! `DESCISPI:YES` (shipped unredacted), the first two tests here go red on
//! that specific record — not a warning, a hard `cargo test` failure that
//! fails `scripts/verify.sh`'s `root-full` stage.
//!
//! # Why a declared name is renamed rather than redacted — and why that is
//! NOT the race-trait suite's "drop the row" rule
//!
//! This suite was written 2026-08-14, mirroring the race-trait suite's
//! shape exactly: a description can be replaced with `REDACTED_PI_MARKER`
//! and the record still works; a NAME cannot be field-redacted, so the
//! race-trait/monster precedent (`decisions.md §50.3`, SD-29) drops the row
//! entirely rather than shipping it under any name at all.
//!
//! **`class_feature` is not covered by that precedent.** Nine days after
//! this suite landed, `SD-32-compute-library-and-cause-closure/decisions.md
//! §24` (operator ruling, 2026-08-23) named `class_feature` (140 units, with
//! `ability` and `deity`) as one of exactly three kinds a cycle may not
//! reclassify unilaterally — because the operator already did, corpus-wide,
//! for these three kinds only. `§24`'s ruling, verbatim: *"ingest them with
//! a Codex-generated neutral name."* `c1505f6497` (2026-08-23) implemented
//! it for `class_feature`: `cache_gen::class_feature::generate` no longer
//! skips a `NAMEISPI:YES` row — it writes it under
//! `codex_neutral_name::neutral_name`, a name derived ONLY from the row's
//! PI-free `(book, source_file, source_line)` coordinates (never from the
//! original name, not even transformed), marks `codex_generated_name: true`,
//! and records `rename.{reason,coordinate}` — never the original string
//! (`§24b`'s six binding conditions; `codex_neutral_name.rs` and
//! `cache_gen::class_feature`'s own extensive unit tests, including a
//! mutation-proof word-boundary regression, prove the no-leak claim
//! independent of this suite).
//!
//! So `name_leak` below does **not** treat every `NAMEISPI:YES` declaration
//! as a leak (that would re-litigate `§24`, which is not this cycle's call
//! to make). It enforces `§24b`'s binding conditions instead — exactly the
//! same shape `description_leak` already uses for `DESCISPI:YES` (checking
//! the redaction actually happened, not banning the declaration outright).
//! A `NAMEISPI:YES` row that ships WITHOUT a properly-formed `§24` rename
//! (wrong prefix, `codex_generated_name` not `true`, wrong/missing
//! `rename.reason`/`rename.coordinate`) is still exactly the leak this
//! suite exists to catch — the correction narrows what counts as a leak to
//! match an operator ruling this suite predates; it does not remove the
//! gate.
//!
//! Reclassifying a kind `§24` does NOT already name (or weakening `§24b`'s
//! own conditions) is still `docs/governance/ogl-pi-blacklist.md` §3's
//! per-book override — an operator decision a cycle may request but not
//! make unilaterally.

use std::fs;
use std::path::{Path, PathBuf};

use codex::rules_core::codex_neutral_name::NAME_PREFIX;
use codex::rules_core::shape_b_v1::{PI_MARKER_REDACTED, REDACTED_PI_MARKER};

fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus")
}

/// Every shipped `class_feature` record, as `(path, parsed json)`, walking
/// `data/corpus/<any book>/class_feature/**`.
fn shipped_class_feature_records() -> Vec<(PathBuf, serde_json::Value)> {
    let mut out = Vec::new();
    let root = corpus_root();
    let books = fs::read_dir(&root).unwrap_or_else(|e| panic!("failed to read {root:?}: {e}"));
    for book in books.flatten() {
        let kind_dir = book.path().join("class_feature");
        if !kind_dir.is_dir() {
            continue;
        }
        collect_json(&kind_dir, &mut out);
    }
    assert!(
        !out.is_empty(),
        "no shipped class_feature records found under {root:?} — either every class_feature \
         directory vanished (a corpus-drift regression this suite must catch, not pass through \
         silently) or the walk itself is broken; a suite that finds zero records here proves \
         nothing about the corpus, per SD30-E3-F4's acceptance criteria"
    );
    out
}

fn collect_json(dir: &Path, out: &mut Vec<(PathBuf, serde_json::Value)>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {dir:?}: {e}")).flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_json(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "json") {
            let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
            let json: serde_json::Value =
                serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path:?} is not valid JSON: {e}"));
            out.push((path, json));
        }
    }
}

/// `true` when the record's own `raw_tokens` carry `<token>:YES`.
fn declares(record: &serde_json::Value, token: &str) -> bool {
    record["data"]["raw_tokens"]
        .as_array()
        .map(|tokens| {
            tokens.iter().any(|t| {
                t["key"].as_str().is_some_and(|k| k.eq_ignore_ascii_case(token))
                    && t["value"].as_str().is_some_and(|v| v.trim().eq_ignore_ascii_case("YES"))
            })
        })
        .unwrap_or(false)
}

/// `true` when `record` declares `NAMEISPI:YES` and either (a) was not
/// renamed through the `decisions.md §24` Codex-generated-neutral-name
/// mechanism at all, or (b) was renamed but not correctly — the original
/// PI name potentially still reachable through a malformed marker. A
/// record that satisfies every `§24b` binding condition below is NOT a
/// leak: `§24` is the operator's own authorization for exactly this shape,
/// for exactly this kind.
fn name_leak(record: &serde_json::Value) -> bool {
    if !declares(record, "NAMEISPI") {
        return false;
    }
    let key = record["data"]["key"].as_str().unwrap_or("");
    let name = record["data"]["name"].as_str().unwrap_or("");
    let codex_generated_name = record["codex_generated_name"].as_bool() == Some(true);
    let rename_reason = record["rename"]["reason"].as_str();
    let rename_coordinate = record["rename"]["coordinate"].as_str();
    let properly_renamed_per_section_24 = codex_generated_name
        && key.starts_with(NAME_PREFIX)
        && name.starts_with(NAME_PREFIX)
        && rename_reason == Some("name_pi_blocked")
        && rename_coordinate.is_some_and(|c| !c.is_empty());
    !properly_renamed_per_section_24
}

/// `true` when `record` declares `DESCISPI:YES` but was not redacted through
/// the shared marker (`pi_screening`'s `PI_MARKER_REDACTED`/`REDACTED_PI_MARKER`,
/// the same constants `ingest_race_traits.rs` and this cycle's transcribers use).
fn description_leak(record: &serde_json::Value) -> bool {
    if !declares(record, "DESCISPI") {
        return false;
    }
    let marker = record["pi_marker"].as_str();
    let description = record["data"]["description"].as_str();
    marker != Some(PI_MARKER_REDACTED) || description != Some(REDACTED_PI_MARKER)
}

#[test]
fn no_shipped_class_feature_record_publishes_a_name_the_corpus_declares_product_identity() {
    let offenders: Vec<String> = shipped_class_feature_records()
        .into_iter()
        .filter(|(_, record)| name_leak(record))
        .map(|(path, record)| {
            format!("{} ({})", record["data"]["key"].as_str().unwrap_or("<no key>"), path.display())
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "a record whose row declares NAMEISPI:YES must ship EITHER not at all OR correctly renamed \
         through the `decisions.md §24` Codex-generated-neutral-name mechanism (SD-32 operator ruling, \
         2026-08-23, naming `class_feature` explicitly) — these shipped with neither: {offenders:#?}"
    );
}

#[test]
fn every_class_feature_description_the_corpus_declares_product_identity_ships_redacted() {
    let mut declared = 0usize;
    let mut leaked: Vec<String> = Vec::new();
    for (path, record) in shipped_class_feature_records() {
        if !declares(&record, "DESCISPI") {
            continue;
        }
        declared += 1;
        if description_leak(&record) {
            leaked.push(format!(
                "{} ({}) — pi_marker {:?}, description {:?}",
                record["data"]["key"].as_str().unwrap_or("<no key>"),
                path.display(),
                record["pi_marker"].as_str(),
                record["data"]["description"].as_str().unwrap_or("<none>"),
            ));
        }
    }
    // class_feature's live corpus declared zero DESCISPI:YES rows when this
    // suite was written (only `pathfinder_unchained` ingested); five more
    // books have landed since and the live count is 402 today, re-derived
    // and printed every run rather than pinned as a literal (`decisions.md
    // §12 L2`) — the assertion below is about `leaked`, never `declared`'s
    // value. Printing it explicitly every run, rather than letting a
    // zero-or-more-iteration loop pass silently either way, is this test's
    // half of the module doc's "empty-target trap" guard; the other half
    // (proving the detector itself can fail) is
    // `the_leak_detectors_actually_fire_on_a_planted_leak_and_clear_on_a_redacted_row`
    // below.
    eprintln!(
        "class_feature shipped records declaring DESCISPI:YES this run: {declared} \
         (0 is the current, re-derived-at-corpus expectation — see module doc)"
    );
    assert!(
        leaked.is_empty(),
        "{declared} shipped class_feature records declare DESCISPI:YES; these publish the declared \
         text anyway because it was never redacted: {leaked:#?}"
    );
}

#[test]
fn the_leak_detectors_actually_fire_on_a_planted_leak_and_clear_on_a_redacted_row() {
    // Proves the mechanism itself can distinguish a leak from a clean row,
    // independent of whatever the live corpus happens to contain — the
    // literal "plant a declared-PI row … confirm the gate goes red … remove
    // it, confirm green" acceptance step, run here as a permanent, always-on
    // regression test rather than only a one-off manual demonstration.

    // 1. A planted NAMEISPI:YES row that still shipped — must be flagged.
    let planted_name_leak = serde_json::json!({
        "data": {
            "key": "Scratch ~ Planted Name Leak",
            "raw_tokens": [{"key": "NAMEISPI", "value": "YES"}],
        },
        "pi_marker": null,
    });
    assert!(
        name_leak(&planted_name_leak),
        "the NAMEISPI:YES detector failed to flag a planted leak — this instrument cannot fail, \
         which means it proves nothing when it passes"
    );

    // 2. A planted DESCISPI:YES row whose description was never redacted — must be flagged.
    let planted_description_leak = serde_json::json!({
        "data": {
            "key": "Scratch ~ Planted Description Leak",
            "description": "the original copyrighted prose, shipped unredacted",
            "raw_tokens": [{"key": "DESCISPI", "value": "YES"}],
        },
        "pi_marker": null,
    });
    assert!(
        description_leak(&planted_description_leak),
        "the DESCISPI:YES detector failed to flag a planted unredacted description"
    );

    // 3. The same DESCISPI:YES row, but correctly redacted — must NOT be flagged (the gate
    //    must go green again once the leak is fixed, not stay permanently red).
    let correctly_redacted = serde_json::json!({
        "data": {
            "key": "Scratch ~ Correctly Redacted",
            "description": REDACTED_PI_MARKER,
            "raw_tokens": [{"key": "DESCISPI", "value": "YES"}],
        },
        "pi_marker": PI_MARKER_REDACTED,
    });
    assert!(
        !description_leak(&correctly_redacted),
        "a correctly redacted DESCISPI:YES row must not be flagged — false positive"
    );

    // 4. An ordinary row with no PI declaration at all — must not be flagged by either detector.
    let clean = serde_json::json!({
        "data": {
            "key": "Scratch ~ Clean",
            "description": "ordinary prose, no PI declared",
            "raw_tokens": [],
        },
        "pi_marker": null,
    });
    assert!(
        !name_leak(&clean) && !description_leak(&clean),
        "a row with no PI declaration must never be flagged by either detector — false positive"
    );

    // 5. A NAMEISPI:YES row correctly renamed through `decisions.md §24`'s
    //    Codex-generated-neutral-name mechanism — must NOT be flagged. This
    //    is the shape `cache_gen::class_feature::generate` actually ships
    //    for `class_feature`'s 124 live name-PI-blocked records; a detector
    //    that still flagged this would make the suite permanently red
    //    against operator-authorized output.
    let properly_renamed = serde_json::json!({
        "data": {
            "key": "Codex-Named Unit (class_feature_scratch_book_scratch_lst_1)",
            "name": "Codex-Named Unit (class_feature_scratch_book_scratch_lst_1)",
            "raw_tokens": [{"key": "NAMEISPI", "value": "YES"}],
        },
        "codex_generated_name": true,
        "rename": {"reason": "name_pi_blocked", "coordinate": "scratch_book:scratch.lst:1"},
        "pi_marker": null,
    });
    assert!(
        !name_leak(&properly_renamed),
        "a NAMEISPI:YES row correctly renamed per §24 must not be flagged — false positive"
    );

    // 6. A NAMEISPI:YES row that CLAIMS `codex_generated_name: true` but is
    //    missing the rename coordinate (a malformed/partial rename) — must
    //    still be flagged. Proves the detector checks the full §24b
    //    contract, not just the one boolean flag.
    let malformed_rename = serde_json::json!({
        "data": {
            "key": "Codex-Named Unit (class_feature_scratch_book_scratch_lst_2)",
            "name": "Codex-Named Unit (class_feature_scratch_book_scratch_lst_2)",
            "raw_tokens": [{"key": "NAMEISPI", "value": "YES"}],
        },
        "codex_generated_name": true,
        "rename": serde_json::Value::Null,
        "pi_marker": null,
    });
    assert!(
        name_leak(&malformed_rename),
        "a NAMEISPI:YES row with codex_generated_name true but no rename coordinate must still be \
         flagged — a partial §24 rename is not a compliant one"
    );
}
