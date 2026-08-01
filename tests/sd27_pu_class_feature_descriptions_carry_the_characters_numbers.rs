//! SD-27 — Pathfinder Unchained's seven `%N`-bearing class-feature
//! descriptions reach the character sheet **with the number in them**
//! (2026-08-01).
//!
//! # The defect this closes
//!
//! Seven of PU's 64 ingested `class_feature` records carry a PCGen `DESC:`
//! token whose prose references a `%N` argument. The ingest could not resolve
//! those arguments — they name variables (`RageDuration`, `KiPoints`,
//! `MasterStrikeDC`) whose value is a fact about a *character*, not about the
//! row — so it dropped each placeholder and stored the wreckage. These are the
//! strings that shipped, read straight off
//! `data/corpus/pathfinder_unchained/class_feature/**/*.json` by
//! [`the_six_broken_shipped_descriptions_are_still_broken_on_disk`]:
//!
//! ```text
//! "You can rage for rounds per day"        (x2 records: Rage, Unchained Rage)
//! "[Ki Pool = ]"
//! "Subtract from the damage you take"
//! "The DC of this save is ."
//! "You add to Perception skill checks"
//! null                                      (Unchained Rogue ~ Rogues Edge)
//! ```
//!
//! The operator's ruling (2026-08-01) is that these are *display values*: the
//! engine already computes every one of them in a hand-modelled
//! `rules_tables::pathfinder_unchained` function, so the fix is to state the
//! number under the name PCGen uses for it and re-render the description at
//! compute time, per character.
//!
//! # What this file pins, and what it deliberately leaves to its siblings
//!
//! * The **transcription** is re-derived byte-for-byte off disk, so a corpus
//!   edit cannot silently invalidate the constant
//!   (`every_transcribed_desc_token_is_byte_identical_to_the_corpus_record`).
//! * The **denominator** is re-derived off disk too, so a newly-ingested
//!   `%N` record cannot join the corpus without joining the list
//!   (`the_transcribed_set_is_exactly_the_pu_records_carrying_a_percent_n`).
//! * The **arithmetic** of each formula is pinned in its own table module's
//!   unit tests against the corpus token. This file asserts the number the
//!   sentence renders equals the number the standalone magnitude row beside it
//!   carries, which is the property that makes the two incapable of disagreeing
//!   (`a_resolved_sentence_never_disagrees_with_the_magnitude_row_beside_it`).
//! * The **reach** half of `decisions.md §29.1` — *a magnitude is not wired
//!   until it moves on the twin the player reads* — by driving the real
//!   `build_pilot_headless_receipt` pipeline, which is what `CharacterSheet.tsx`
//!   renders through `classFeaturesModel.ts`.
//!
//! # §28's standing guard
//!
//! `decisions.md §28`: *"Every change to [pilot_compute.rs] lands with a test
//! pinning the before/after per affected race or class."* This change adds text
//! to existing roster rows and adds no row and no magnitude, so the guard is
//! that the counts do **not** move: [`GROUNDED_ROWS_AT_LEVEL_10`] and
//! [`ROSTER_ROWS_AT_LEVEL_20`] carry the sibling pins' numbers verbatim and
//! assert they survived exactly.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterClassLevel, CharacterInput,
};
use codex::rules_core::pcgen_desc::leaked_pcgen_syntax;
use codex::rules_core::pilot_compute::{
    build_pilot_headless_receipt, pu_class_feature_cited_key, pu_resolved_description_from_detail,
    ComputationExplanation, PU_RESOLVABLE_DESCRIPTIONS,
};

/// The same shared deterministic fixture every sibling PU pin uses, so all of
/// them describe the same posture rather than several different ones.
const FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// The four Unchained class tokens, and the corpus directory each one's records
/// live in.
const PU_CLASSES: &[(&str, &str)] = &[
    ("unchained_barbarian", "barbarian_unchained_class"),
    ("unchained_monk", "monk_unchained_class"),
    ("unchained_rogue", "rogue_unchained_class"),
    ("unchained_summoner", "summoner_unchained_class"),
];

/// Grounded `class_feature.pu.<class>.*` magnitude rows at level 10, verbatim
/// from `sd27_pu_class_features_reach_by_corpus_key.rs`'s `GROUNDED_PIN` and
/// `sd27_pu_prose_derived_class_features_reach_the_sheet.rs`'s "after" column.
/// This change adds no magnitude, so every number must survive it unchanged.
const GROUNDED_ROWS_AT_LEVEL_10: &[(&str, usize)] = &[
    ("unchained_barbarian", 10),
    ("unchained_monk", 12),
    ("unchained_rogue", 11),
    ("unchained_summoner", 11),
];

/// Roster rows emitted at level 20, per class — verbatim from
/// `sd27_pu_class_features_reach_by_corpus_key.rs`'s `ROSTER_ROWS_AT_LEVEL_20`.
/// This change adds no row, so every number must survive it unchanged.
const ROSTER_ROWS_AT_LEVEL_20: &[(&str, usize)] = &[
    ("unchained_barbarian", 14),
    ("unchained_monk", 18),
    ("unchained_rogue", 15),
    ("unchained_summoner", 17),
];

/// The fragment each broken shipped description is recognised by, paired with
/// the record key that shipped it. Fragments rather than whole strings because
/// the point is the *hole* — the missing number and the whitespace or
/// punctuation that closed over it.
///
/// `Unchained Rogue ~ Rogues Edge` is absent here and handled separately: it
/// shipped `null`, not a holed sentence, so there is no fragment to match.
const BROKEN_SHIPPED_FRAGMENTS: &[(&str, &str)] = &[
    ("Unchained Barbarian ~ Rage", "You can rage for rounds per day"),
    ("Unchained Rage", "You can rage for rounds per day"),
    (
        "Unchained Barbarian ~ Damage Reduction",
        "Subtract from the damage you take",
    ),
    ("Unchained Monk ~ Ki Pool", "[Ki Pool = ]"),
    ("Unchained Rogue ~ Master Strike", "The DC of this save is ."),
    (
        "Unchained Rogue ~ Trapfinding",
        "You add to Perception skill checks",
    ),
];

/// The record that shipped no description at all.
const EMPTY_SHIPPED_RECORD: &str = "Unchained Rogue ~ Rogues Edge";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// Every ingested PU `class_feature` record on disk, as
/// `key -> (stored description, DESC: tokens in source order)`.
///
/// Read off `data/corpus/`, never off the tables the code under test reads: a
/// pin that recomputes its expectation from the same source pins nothing.
fn corpus_records() -> BTreeMap<String, (Option<String>, Vec<String>)> {
    let mut out = BTreeMap::new();
    for (_, class_dir) in PU_CLASSES {
        let dir = repo_root()
            .join("data/corpus/pathfinder_unchained/class_feature")
            .join(class_dir);
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|error| panic!("{} is readable: {error}", dir.display()));
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("record file is readable");
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("record file is Shape B v1 JSON");
            let data = &value["data"];
            let key = data["key"]
                .as_str()
                .expect("every class_feature record carries data.key")
                .to_owned();
            let description = data["description"].as_str().map(str::to_owned);
            let tokens: Vec<String> = data["raw_tokens"]
                .as_array()
                .expect("every class_feature record carries data.raw_tokens")
                .iter()
                .filter(|token| token["key"].as_str() == Some("DESC"))
                .map(|token| {
                    token["value"]
                        .as_str()
                        .expect("a DESC raw token carries a string value")
                        .to_owned()
                })
                .collect();
            out.insert(key, (description, tokens));
        }
    }
    out
}

/// Whether a raw `DESC:` token's prose references a `%N` argument. `%%` is
/// PCGen's literal-percent escape and is skipped, exactly as
/// `pcgen_desc::max_arg_reference` skips it.
fn references_an_argument(raw: &str) -> bool {
    let chars: Vec<char> = raw.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            i += 2;
            continue;
        }
        if chars[i] == '%' && chars.get(i + 1).is_some_and(|c| c.is_ascii_digit() && *c != '0') {
            return true;
        }
        i += 1;
    }
    false
}

fn fixture() -> CharacterInput {
    let text = std::fs::read_to_string(FIXTURE).expect("shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("shared deterministic fixture loads")
}

fn explanations_for(class_token: &str, level: u8) -> Vec<ComputationExplanation> {
    let mut input = fixture();
    input.case_id = Some(format!("sd27_pu_desc.{class_token}.level{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_token}"),
        level,
    }];
    build_pilot_headless_receipt(&input).computation.explanations
}

/// The resolved rules text the receipt carries for one corpus key at one level,
/// or `None` when the row carries none (or no row exists yet).
fn rules_text_for(class_token: &str, level: u8, record_key: &str) -> Option<String> {
    explanations_for(class_token, level)
        .into_iter()
        .filter(|explanation| pu_class_feature_cited_key(&explanation.detail) == Some(record_key))
        .find_map(|explanation| {
            pu_resolved_description_from_detail(&explanation.detail).map(str::to_owned)
        })
}

/// The magnitude row `id` carries at `level`, or `None` when it emitted none.
fn magnitude(class_token: &str, level: u8, id_tail: &str) -> Option<i16> {
    let id = format!("class_feature.pu.{class_token}.{id_tail}");
    explanations_for(class_token, level)
        .into_iter()
        .find(|explanation| explanation.id == id)
        .map(|explanation| explanation.value)
}

// ---------------------------------------------------------------------------
// The transcription, and its denominator, re-derived off disk
// ---------------------------------------------------------------------------

/// The constant is a hand transcription of corpus rows. This re-reads every one
/// of them and compares byte for byte, so a corpus edit is a failing test rather
/// than a stale string on a player's sheet.
#[test]
fn every_transcribed_desc_token_is_byte_identical_to_the_corpus_record() {
    let corpus = corpus_records();
    for record in PU_RESOLVABLE_DESCRIPTIONS {
        let (_, tokens) = corpus
            .get(record.record_key)
            .unwrap_or_else(|| panic!("`{}` is an ingested PU record", record.record_key));
        assert_eq!(
            record.desc_tokens.len(),
            tokens.len(),
            "`{}` transcribes a different number of DESC tokens than the corpus row carries",
            record.record_key
        );
        for (index, (transcribed, on_disk)) in record.desc_tokens.iter().zip(tokens).enumerate() {
            assert_eq!(
                *transcribed, on_disk,
                "`{}` DESC token {index} drifted from the corpus record",
                record.record_key
            );
        }
    }
}

/// The list is *all of them*, not a selection. The denominator is read off disk
/// so a newly-ingested `%N` record cannot join the corpus without joining the
/// list — which is the failure mode that would quietly re-open this defect for
/// one feature.
#[test]
fn the_transcribed_set_is_exactly_the_pu_records_carrying_a_percent_n() {
    let corpus = corpus_records();
    let on_disk: BTreeSet<&str> = corpus
        .iter()
        .filter(|(_, (_, tokens))| tokens.iter().any(|token| references_an_argument(token)))
        .map(|(key, _)| key.as_str())
        .collect();
    let transcribed: BTreeSet<&str> =
        PU_RESOLVABLE_DESCRIPTIONS.iter().map(|record| record.record_key).collect();

    assert_eq!(
        transcribed, on_disk,
        "the transcribed set and the set of PU records carrying a %N must be the same set"
    );
    assert_eq!(corpus.len(), 64, "PU ingested 64 class_feature records");
    assert_eq!(on_disk.len(), 7, "7 of the 64 carry a %N");
}

// ---------------------------------------------------------------------------
// Before: the shipped strings, read off disk rather than asserted from memory
// ---------------------------------------------------------------------------

/// The "before" half of the before/after pair, derived rather than quoted: the
/// stored `description` on disk is still the holed sentence. Nothing in this
/// change rewrites `data/corpus/`, and this test says so out loud — the repair
/// happens at compute time, per character, which is the only place the number
/// exists.
#[test]
fn the_six_broken_shipped_descriptions_are_still_broken_on_disk() {
    let corpus = corpus_records();
    for (key, fragment) in BROKEN_SHIPPED_FRAGMENTS {
        let (description, _) = corpus.get(*key).unwrap_or_else(|| panic!("`{key}` is ingested"));
        let description = description
            .as_deref()
            .unwrap_or_else(|| panic!("`{key}` shipped a description string"));
        assert!(
            description.contains(fragment),
            "`{key}` no longer ships the holed sentence `{fragment}`; stored text: {description}"
        );
    }

    let (description, tokens) = corpus
        .get(EMPTY_SHIPPED_RECORD)
        .unwrap_or_else(|| panic!("`{EMPTY_SHIPPED_RECORD}` is ingested"));
    assert_eq!(
        *description, None,
        "`{EMPTY_SHIPPED_RECORD}` shipped no description at all"
    );
    assert_eq!(
        tokens.len(),
        4,
        "`{EMPTY_SHIPPED_RECORD}` carries four DESC tokens, every prose-bearing one gated"
    );
}

// ---------------------------------------------------------------------------
// After: the resolved sentence reaches the receipt the sheet renders
// ---------------------------------------------------------------------------

/// The headline: at a level where the character actually has the feature, the
/// sentence carries the number.
///
/// One assertion per broken string, with the level chosen at or above the
/// record's own grant level and the fixture's own ability modifiers doing the
/// rest.
#[test]
fn each_broken_description_reaches_the_sheet_with_its_number() {
    // Con 14 (+2) on the shared fixture: 2 + 2 + 2*10 = 24 rounds at level 10.
    let rage = rules_text_for("unchained_barbarian", 10, "Unchained Barbarian ~ Rage")
        .expect("Rage is granted at level 1 and its description resolves");
    assert!(
        rage.contains("You can rage for 24 rounds per day"),
        "rage rounds must render: {rage}"
    );
    assert!(rage.contains("a +2 bonus on melee attack rolls"), "morale bonus: {rage}");
    assert!(rage.contains("you take a -2 penalty to Armor Class"), "AC penalty: {rage}");
    assert!(rage.contains("You also gain 20 temporary hit points"), "temp hp: {rage}");

    let unchained_rage = rules_text_for("unchained_barbarian", 10, "Unchained Rage")
        .expect("Unchained Rage's own row resolves the same variables");
    assert!(
        unchained_rage.contains("You can rage for 24 rounds per day"),
        "the sibling record renders the same number: {unchained_rage}"
    );
    assert!(
        unchained_rage.ends_with("You are using an alternative raging method."),
        "its PREABILITY-gated second sentence is undecidable here and survives: {unchained_rage}"
    );

    let damage_reduction = rules_text_for(
        "unchained_barbarian",
        10,
        "Unchained Barbarian ~ Damage Reduction",
    )
    .expect("Damage Reduction is granted at level 7");
    assert!(
        damage_reduction.contains("Subtract 2 from the damage you take"),
        "(10 - 4) / 3 = 2: {damage_reduction}"
    );

    // Wis 12 (+1) on the shared fixture: 10 / 2 + 1 = 6 ki points at level 10.
    let ki_pool = rules_text_for("unchained_monk", 10, "Unchained Monk ~ Ki Pool")
        .expect("Ki Pool is granted at level 3");
    assert!(ki_pool.starts_with("[Ki Pool = 6]"), "ki points: {ki_pool}");

    // Int 10 (+0) on the shared fixture: 10 + 20 / 2 + 0 = 20 at level 20.
    let master_strike = rules_text_for("unchained_rogue", 20, "Unchained Rogue ~ Master Strike")
        .expect("Master Strike is granted at level 20");
    assert!(
        master_strike.contains("The DC of this save is 20."),
        "master strike DC: {master_strike}"
    );

    let trapfinding = rules_text_for("unchained_rogue", 10, "Unchained Rogue ~ Trapfinding")
        .expect("Trapfinding is granted at level 1");
    assert!(
        trapfinding.starts_with("You add +5 to Perception skill checks"),
        "max(10 / 2, 1) = 5: {trapfinding}"
    );

    let rogues_edge = rules_text_for("unchained_rogue", 10, EMPTY_SHIPPED_RECORD)
        .expect("Rogue's Edge is granted at level 5 and now renders a sentence at all");
    assert_eq!(
        rogues_edge,
        "You have mastered 2 skills beyond those skill's normal boundaries, gaining results that \
         others can only dream about. You gain the skill unlock powers as appropriate for the \
         number of ranks you have.",
        "the record that shipped `null` renders its whole gated sentence"
    );
}

/// The two mutually exclusive `PREVAR` branches on the same variable pick
/// exactly one each, and the singular branch is a real branch rather than a
/// rounding accident: at level 5 `RogueLVL/5` is 1, so `PREVAREQ:…,1` holds and
/// `PREVARGT:…,1` does not.
#[test]
fn the_rogues_edge_gate_pair_picks_singular_at_five_and_plural_above_it() {
    let at_five = rules_text_for("unchained_rogue", 5, EMPTY_SHIPPED_RECORD)
        .expect("Rogue's Edge is granted at level 5");
    assert!(
        at_five.contains("a single skill beyond that skill's normal boundaries"),
        "the singular branch: {at_five}"
    );
    assert!(
        !at_five.contains("skills beyond those skill's"),
        "and only that branch: {at_five}"
    );

    let at_twenty = rules_text_for("unchained_rogue", 20, EMPTY_SHIPPED_RECORD)
        .expect("Rogue's Edge is still held at level 20");
    assert!(
        at_twenty.contains("4 skills beyond those skill's normal boundaries"),
        "20 / 5 = 4: {at_twenty}"
    );
    assert!(
        !at_twenty.contains("a single skill"),
        "and only that branch: {at_twenty}"
    );
}

/// The property that makes a description incapable of drifting from the number
/// beside it: both read the same hand-modelled function. Asserted as an
/// equality against the standalone magnitude row rather than against a literal,
/// so a formula change moves both or fails here.
#[test]
fn a_resolved_sentence_never_disagrees_with_the_magnitude_row_beside_it() {
    for level in 1..=20u8 {
        if let Some(rage) = rules_text_for("unchained_barbarian", level, "Unchained Barbarian ~ Rage")
        {
            let rounds = magnitude("unchained_barbarian", level, "rage_rounds_per_day")
                .expect("a resolved rage sentence means the magnitude row exists too");
            assert!(
                rage.contains(&format!("You can rage for {rounds} rounds per day")),
                "level {level}: sentence and magnitude row must state the same number: {rage}"
            );
        }
        if let Some(ki) = rules_text_for("unchained_monk", level, "Unchained Monk ~ Ki Pool") {
            let points = magnitude("unchained_monk", level, "ki_points")
                .expect("a resolved ki sentence means the magnitude row exists too");
            assert!(
                ki.starts_with(&format!("[Ki Pool = {points}]")),
                "level {level}: {ki}"
            );
        }
        if let Some(trap) = rules_text_for("unchained_rogue", level, "Unchained Rogue ~ Trapfinding")
        {
            let bonus = magnitude("unchained_rogue", level, "trapfinding_bonus")
                .expect("a resolved trapfinding sentence means the magnitude row exists too");
            assert!(
                trap.starts_with(&format!("You add +{bonus} to Perception")),
                "level {level}: {trap}"
            );
        }
    }
}

/// No PCGen syntax may reach a player: no `%N`, no `%%`, no raw `|` argument
/// tail, no undecoded entity escape. Swept over every class at every level, so
/// this covers all 64 records rather than the seven under test.
#[test]
fn no_resolved_description_ever_carries_pcgen_syntax_to_a_player() {
    let mut resolved_rows = 0usize;
    for (class_token, _) in PU_CLASSES {
        for level in 1..=20u8 {
            for explanation in explanations_for(class_token, level) {
                let Some(text) = pu_resolved_description_from_detail(&explanation.detail) else {
                    continue;
                };
                resolved_rows += 1;
                assert_eq!(
                    leaked_pcgen_syntax(text),
                    None,
                    "{} at level {level} leaked PCGen syntax: {text}",
                    explanation.id
                );
                assert!(
                    !text.is_empty(),
                    "{} at level {level} emitted an empty rules-text clause",
                    explanation.id
                );
            }
        }
    }
    assert!(
        resolved_rows > 0,
        "the sweep must actually have found resolved rows, or it asserts nothing"
    );
}

/// A record stating no `%N` gets no appended rules text. This is what keeps the
/// change to seven rows rather than a prose dump across all 64, and it is
/// checked by counting rather than by inspection.
#[test]
fn only_the_seven_percent_n_records_gain_a_rules_text_clause() {
    let transcribed: BTreeSet<&str> =
        PU_RESOLVABLE_DESCRIPTIONS.iter().map(|record| record.record_key).collect();

    for (class_token, _) in PU_CLASSES {
        for explanation in explanations_for(class_token, 20) {
            let Some(key) = pu_class_feature_cited_key(&explanation.detail) else {
                continue;
            };
            let has_text = pu_resolved_description_from_detail(&explanation.detail).is_some();
            assert_eq!(
                has_text,
                transcribed.contains(key),
                "`{key}` carries rules text on its receipt row only if it is one of the seven"
            );
        }
    }
}

/// A feature the character has not reached carries no sentence at all — not a
/// sentence with a `0` in it. Master Strike is granted only at 20th, so a 19th
/// level rogue must have neither the row nor the text.
#[test]
fn a_feature_below_its_grant_level_renders_no_sentence_rather_than_a_zero() {
    assert_eq!(
        rules_text_for("unchained_rogue", 19, "Unchained Rogue ~ Master Strike"),
        None,
        "Master Strike is not a smaller number at 19th, it is not a thing"
    );
    assert_eq!(
        rules_text_for("unchained_rogue", 4, EMPTY_SHIPPED_RECORD),
        None,
        "Rogue's Edge is granted at 5th"
    );
    assert_eq!(
        rules_text_for("unchained_barbarian", 6, "Unchained Barbarian ~ Damage Reduction"),
        None,
        "Damage Reduction is granted at 7th"
    );
}

// ---------------------------------------------------------------------------
// §28's standing guard: this change moves no count
// ---------------------------------------------------------------------------

/// Adding text to a row must not add, drop or renumber one. Both sibling pins'
/// counts are re-asserted here against the live pipeline.
#[test]
fn no_row_count_moves_because_a_row_gained_rules_text() {
    for (class_token, expected) in GROUNDED_ROWS_AT_LEVEL_10 {
        let prefix = format!("class_feature.pu.{class_token}.");
        let grounded = explanations_for(class_token, 10)
            .into_iter()
            .filter(|explanation| {
                explanation.id.starts_with(&prefix)
                    && !explanation.id.contains(".corpus_record.")
                    && !explanation.id.ends_with(".unsupported")
            })
            .count();
        assert_eq!(grounded, *expected, "{class_token} grounded magnitude rows at level 10");
    }

    for (class_token, expected) in ROSTER_ROWS_AT_LEVEL_20 {
        let prefix = format!("class_feature.pu.{class_token}.corpus_record.");
        let roster = explanations_for(class_token, 20)
            .into_iter()
            .filter(|explanation| explanation.id.starts_with(&prefix))
            .count();
        assert_eq!(roster, *expected, "{class_token} roster rows at level 20");
    }
}
