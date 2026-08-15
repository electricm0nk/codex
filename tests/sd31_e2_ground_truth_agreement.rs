//! SD-31 Epic 2-F2 (`SD31-E2-F2-001-wiringfix`) — validates the
//! `wiring_class::CorpusLines` path-resolution fix and the
//! `signals()` `BONUS:STAT`/`CR:`/`DR:` false-positive fixes against the
//! F1 ground-truth sample's genuinely-evidenced units.
//!
//! Deliverable 3 of the dispatch brief: wave 1's adversarial review
//! (`OPEN-ISSUES.md` row 3) confirmed 105 of the sample's 150
//! `token_evidence` strings are canned boilerplate — a byte-identical
//! string that quotes zero tokens from the record — and unusable as a
//! yardstick. This test filters to the ~45 units whose evidence is real
//! and record-specific (does NOT start with that boilerplate), and reports
//! per-unit agreement, not a blended rate.
//!
//! Real-corpus-gated (`PCGEN_CORPUS_ROOT`), same pattern as
//! `tests/sd22_acg_class_arcanist_resolves.rs`'s
//! `hand_transcribed_chassis_matches_the_real_lst_bonus_tokens`.
//!
//! Scope note: this reads each unit's BASE corpus row only
//! (`wiring_class::determine`, not `determine_closure`) — it does not
//! union in `.MOD` rows targeting the unit's name/key, because the
//! ground-truth JSON does not carry the `corpus_key` a `.MOD` lookup needs
//! and reconstructing one risks a second, drifting definition of it. A
//! unit whose true magnitude lives ONLY on a `.MOD` row (not its own base
//! row) will report `display`/`static`/`ambiguous` here even though the
//! real production classifier (which always closes over the full token
//! closure) may correctly promote it further — this is called out
//! per-unit below where it applies, not silently absorbed into the
//! agreement count.

use std::collections::BTreeMap;
use std::path::PathBuf;

use codex::rules_core::wiring_class::{self, CorpusLines, WiringClass};
use serde_json::Value;

/// Canned boilerplate `OPEN-ISSUES.md` row 3 identified — a unit whose
/// `token_evidence` starts with this quotes zero record-specific tokens
/// and is excluded from this yardstick.
const CANNED_BOILERPLATE: &str = "confirmed from the unit's full token closure";

fn corpus_root() -> PathBuf {
    PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    )
}

/// Locate a book's real directory by basename search under the corpus's
/// three known Paizo/third-party roots — same roster shape as
/// `v06_work_inventory`'s `book_paths` (`roleplaying_game` +
/// `EXTRA_BOOK_DIRS`), reconstructed here rather than imported because
/// that map is private to the binary crate.
fn find_book_dir(root: &std::path::Path, book: &str) -> Option<PathBuf> {
    for parent in [
        root.join("pathfinder/paizo/roleplaying_game"),
        root.join("pathfinder/paizo/campaign_setting"),
        root.join("pathfinder/dreamscarred_press"),
    ] {
        let candidate = parent.join(book);
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    None
}

fn wiring_class_from_str(s: &str) -> WiringClass {
    match s {
        "display" => WiringClass::Display,
        "static" => WiringClass::Static,
        "derived" => WiringClass::Derived,
        "computed" => WiringClass::Computed,
        "ambiguous" => WiringClass::Ambiguous,
        other => panic!("unknown wiring_class in ground truth JSON: {other}"),
    }
}

#[test]
#[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
fn genuinely_evidenced_ground_truth_units_agree_with_the_fixed_engine() {
    let sample_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E2-F1-ground-truth-sample-v1.json",
    );
    let sample_text = std::fs::read_to_string(&sample_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", sample_path.display()));
    let sample: Vec<Value> =
        serde_json::from_str(&sample_text).expect("ground truth sample is valid JSON");

    let genuine: Vec<&Value> = sample
        .iter()
        .filter(|r| {
            !r["token_evidence"]
                .as_str()
                .unwrap_or("")
                .starts_with(CANNED_BOILERPLATE)
        })
        .collect();
    assert_eq!(
        genuine.len(),
        45,
        "expected 45 genuinely-evidenced units (150 total - 105 canned boilerplate, \
         OPEN-ISSUES.md row 3) -- re-derive if the sample file changed"
    );

    let root = corpus_root();
    let mut book_dirs: BTreeMap<String, PathBuf> = BTreeMap::new();
    let mut book_paths: BTreeMap<String, PathBuf> = BTreeMap::new();
    let mut agree = 0usize;
    let mut disagree: Vec<String> = Vec::new();

    for r in &genuine {
        let id = r["id"].as_str().unwrap();
        let book = r["book"].as_str().unwrap();
        let source_file = r["source_file"].as_str().unwrap();
        let source_line = r["source_line"].as_u64().unwrap() as usize;
        let hand = wiring_class_from_str(r["hand_wiring_class"].as_str().unwrap());

        let dir = book_dirs
            .entry(book.to_string())
            .or_insert_with(|| {
                find_book_dir(&root, book)
                    .unwrap_or_else(|| panic!("could not locate book directory for `{book}` under {}", root.display()))
            })
            .clone();
        book_paths.entry(book.to_string()).or_insert(dir);

        let mut lines = CorpusLines::new(&book_paths);
        let row = lines.line(book, source_file, source_line);
        let (engine_class, engine_reason, _) = wiring_class::determine(row.as_deref());

        if engine_class == hand {
            agree += 1;
        } else {
            disagree.push(format!(
                "{id}: engine(fixed)={engine_class:?}/{engine_reason} hand={hand:?} row={row:?}"
            ));
        }
    }

    eprintln!(
        "SD31-E2-F2-001-wiringfix ground-truth agreement (base-row only, 45 genuinely-evidenced units): \
         {agree}/{}",
        genuine.len()
    );
    for d in &disagree {
        eprintln!("  DISAGREE: {d}");
    }

    // This assertion is intentionally NOT `assert_eq!(disagree.len(), 0)`:
    // per the dispatch brief, a disagreement is a finding to explain, not
    // noise to average away. All 5 disagreements at 40/45 are attributable
    // to THREE documented, out-of-scope classifier gaps the ground-truth
    // labeller's own `token_evidence` names -- not to this cycle's two
    // named fixes being wrong (`progress.md`'s SD31-E2-F2-001-wiringfix
    // receipt has the full per-unit breakdown):
    //   - `case_sensitive_scalar_false_negative` (1 unit): `has_scalar`'s
    //     SCALARS_SUBSTRING check is case-sensitive and misses PCGen's
    //     lowercase `classlevel(...)` function-call form.
    //   - `bare_var_judgement_call` (3 units): whether a bare
    //     non-literal, non-scalar variable reference (`MonkLVL`,
    //     `FavoredBaseBonus`, `RaptureLVL`) should be `ambiguous` rather
    //     than fall to the `static` default is a classifier DESIGN
    //     decision, not a bug -- the labeller themselves records it as a
    //     judgement call (one at only medium confidence).
    //   - `has_arith`'s uppercase-run rule requires a `+` be followed by a
    //     WORD (`\+\s*\w*[A-Z]{2,}`); it does not match a parenthesised
    //     sub-expression immediately after `+`
    //     (`10+(SpiritualistLVL>=14)+(SpiritualistLVL>=18)`), a second,
    //     distinct `has_arith` fidelity gap from either fix this cycle made.
    // None of the three is one of `OPEN-ISSUES.md` rows 1/2's named
    // findings; fixing them here would be undispatched scope expansion on
    // a shared, heavily-consumed function. The count is asserted so a
    // regression in either direction is visible on the next run.
    assert_eq!(
        agree,
        40,
        "agreement count moved -- re-derive the per-unit breakdown in progress.md before trusting this number \
         (disagreements printed above)"
    );
}
