//! SD-31 Epic 2-F2 (`SD31-E2-F2-001-wiringfix`) — validates the
//! `wiring_class::CorpusLines` path-resolution fix, the `signals()`
//! `BONUS:STAT`/`CR:`/`DR:` false-positive fixes, and their D4 over-shoot
//! repair (`SD31-W2-INTEGRATE-001`, Finding 1) against the F1/F1-002
//! ground-truth sample's genuinely-evidenced units.
//!
//! History: wave 1's adversarial review (`OPEN-ISSUES.md` row 3) confirmed
//! 105 of the original 150-unit sample's `token_evidence` strings were
//! canned boilerplate — a byte-identical string that quoted zero tokens
//! from the record — and unusable as a yardstick. `SD31-E2-F1-002-relabel`
//! re-labelled all 105 from the real corpus record and widened the sample
//! to 185 units (`OPEN-ISSUES.md` rows 3-5, Resolved); as of the
//! `SD31-W2-INTEGRATE-001` integration merge, **0 units carry the canned
//! string** — every one of the 185 is genuinely evidenced. The
//! `CANNED_BOILERPLATE` filter below is kept as a live regression guard
//! (`genuine.len()` must equal the sample's total size) rather than
//! removed, so a future re-introduction of the defect shape is caught
//! here too, not only by `scripts/ground_truth_evidence_guard.py`.
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
/// and is excluded from this yardstick. Kept as a live regression guard
/// even though the current sample carries zero matches (see module doc).
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
        185,
        "expected all 185 units genuinely evidenced (0 canned boilerplate remaining after \
         SD31-E2-F1-002-relabel) -- re-derive if the sample file changed"
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
        "SD31-W2-INTEGRATE-001 ground-truth agreement (base-row only, all 185 units, \
         re-derived against the merged tip after the D4 over-shoot repair): {agree}/{}",
        genuine.len()
    );
    for d in &disagree {
        eprintln!("  DISAGREE: {d}");
    }

    // This assertion is intentionally NOT `assert_eq!(disagree.len(), 0)`:
    // per the dispatch brief, a disagreement is a finding to explain, not
    // noise to average away. Every one of the 18 disagreements at 167/185
    // (re-derived `SD31-W2-INTEGRATE-001`, adversarial-review Finding 9's
    // remedy: re-derive against the merged tip, not the pre-wiringfix
    // snapshot) is attributable to an ALREADY-DOCUMENTED, out-of-scope gap
    // -- none is caused by the D4 fix itself (`has_arith_scoped`/
    // `has_scalar_or_arith_for_token`'s DR/BONUS:STAT variable-magnitude
    // repair), which touches no disagreeing unit here:
    //   - Base-row-only test SCOPE LIMITATION (this file's own doc
    //     comment), not a classifier defect (5 units): `core_essentials:
    //     race:aasimar`, `bestiary:race:iron_cobra_darkwood_cobra`,
    //     `bestiary:race:iron_cobra_adamantine_cobra` (`.COPY=` rows whose
    //     real magnitude lives on the COPY target, not the base row),
    //     `mythic_adventures:equipment:chaos_hammer` (magnitude lives on a
    //     `.MOD` row, not the base row), `core_rulebook:spell:bless`
    //     (Derived/prose_expr vs. hand Computed -- the CASTERLEVEL
    //     percent-placeholder resolution the production closure performs
    //     is out of this base-row-only harness's reach).
    //   - `OPEN-ISSUES.md` row 9's three pre-existing classifier gaps (5
    //     units): (a) `case_sensitive_scalar_false_negative` --
    //     `ultimate_magic:class_feature:dragon_shaman_totem_transformation`;
    //     (b) `bare_var_judgement_call` --
    //     `ultimate_combat:class_feature:martial_artist_martial_arts_master`,
    //     `core_essentials:race_trait:favored_enemy_humanoid_changeling`,
    //     `horror_adventures:class_feature:exciter_rapture`; (c) the
    //     `has_arith` uppercase-run/parenthesised-subexpression gap --
    //     `horror_adventures:class_feature:exciter_rapturous_rage`.
    //   - `OPEN-ISSUES.md` row 16's Findings D/E/F (relabel branch,
    //     unscanned field shapes -- 5 units): Finding D (`SPELLS:` field
    //     unscanned) -- `bestiary_4:monster_ability:winter_hag_ice_staff`;
    //     Finding E (`PLUS:` field unscanned) --
    //     `core_rulebook:equipment_modifier:special_ability_ghost_touch_armor`,
    //     `ultimate_combat:equipment_modifier:special_ability_reliable_firearm`;
    //     Finding F (`ASPECT:` field unscanned, and the prose scanner's own
    //     parenthesised-citation false positive) --
    //     `bestiary_5:monster_ability:chuspiki_air_blast`,
    //     `bestiary_3:race_trait:fuath_spell_like_abilities`.
    //   - Widening-batch medium-confidence JUDGEMENT CALLS, documented at
    //     the labeller's own `confidence: medium` (3 units):
    //     `ultimate_campaign:feat:thief_of_legend`,
    //     `core_rulebook:spell:nightmare`, `core_rulebook:spell:blasphemy`
    //     (prose-only per-level/per-caster-level scaling with no
    //     `BONUS:`/`DEFINE:` chassis -- a genuine `display` vs. `ambiguous`
    //     definitional gray zone the methodology names explicitly).
    // 5 + 5 + 5 + 3 = 18, matching the printed disagreement count exactly.
    // None of these is `OPEN-ISSUES.md` rows 1/2's named findings, nor
    // Finding 1's D4 repair; fixing them here would be undispatched scope
    // expansion on a shared, heavily-consumed function. The count is
    // asserted so a regression in either direction is visible on the next
    // run.
    assert_eq!(
        agree,
        167,
        "agreement count moved -- re-derive the per-unit breakdown in progress.md before trusting this number \
         (disagreements printed above)"
    );
}
