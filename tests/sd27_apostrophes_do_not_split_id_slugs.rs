//! SD-27 — an apostrophe no longer splits a name into two id segments, so
//! `Maker's Call` stops rendering as **"Maker S Call"** (2026-08-01).
//!
//! # The defect
//!
//! `pilot_compute.rs` builds explanation-id segments with two sluggers that
//! turn *every* non-alphanumeric character into a `_` separator:
//!
//! | slugger | input | id it produced | label the sheet rendered |
//! |---|---|---|---|
//! | `pu_feature_slug` | `Unchained Summoner ~ Maker's Call` | `…corpus_record.maker_s_call` | **Maker S Call** |
//! | `slugify_id_segment` | `Scavenger's Eye` | `feat.arg_skill_bonus.scavenger_s_eye.appraise` | **Scavenger S Eye …** |
//!
//! An apostrophe sits *inside* a word, so promoting it to a separator splits
//! the word and the desktop's `classFeaturesModel.ts::humanise` — which splits
//! on `[\s._]+` and title-cases each part — then capitalises the orphaned
//! letter. `slugify_id_segment`'s own doc comment claimed it produced
//! `scavengers_eye`; it did not, and now it does.
//!
//! The repo already had the right convention in a third slugger,
//! `class_feature_id_slug`, which drops apostrophes outright
//! (`Swashbuckler's Edge` -> `swashbucklers_edge`). Both offenders were moved
//! onto it rather than a new one being invented.
//!
//! # The sweep, and why it is only two rows
//!
//! Derived by command rather than by fixing the reported instance:
//!
//! * `pu_feature_slug`'s whole input domain is the 64 ingested Pathfinder
//!   Unchained `class_feature` keys. Scanning
//!   `data/corpus/pathfinder_unchained/class_feature/*/*.json` for characters
//!   that are neither ASCII-alphanumeric nor a space returns exactly one
//!   character, `'`, on exactly one record: `Unchained Summoner ~ Maker's Call`.
//! * `slugify_id_segment`'s domain is the `feat_key` / `skill_name` /
//!   `maneuver` fields of `feat_effects`' five ARG fact tables — 28 distinct
//!   strings. One carries an apostrophe (`Scavenger's Eye`); the parenthesised
//!   ones (`Craft (Alchemy)`, `Profession (Brewer)`,
//!   `Expanded Fiendish Resistance (Acid)`) already collapsed correctly,
//!   because a parenthesis is adjacent to a space and the run collapses to one
//!   separator.
//!
//! [`no_live_id_segment_splits_a_word_on_punctuation`] is the guard that keeps
//! it at zero: it drives the real pipeline over both books' affected surfaces
//! and fails on any id segment that is a bare single letter.

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterClassLevel, CharacterInput,
};
use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, ComputationExplanation};

const FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// The two ids this change corrects, `(before, after)`. Both "before" strings
/// were read off the real pipeline before the fix.
const CORRECTED_IDS: &[(&str, &str)] = &[
    (
        "class_feature.pu.unchained_summoner.corpus_record.maker_s_call",
        "class_feature.pu.unchained_summoner.corpus_record.makers_call",
    ),
    (
        "feat.arg_skill_bonus.scavenger_s_eye.appraise",
        "feat.arg_skill_bonus.scavengers_eye.appraise",
    ),
];

/// Every ARG feat that reaches `slugify_id_segment`, so the sweep runs over the
/// whole domain rather than the one reported row.
const ARG_FEATS_REACHING_THE_SLUGGER: &[&str] = &[
    "Scavenger's Eye",
    "Angelic Flesh",
    "Brewmaster",
    "Seen and Unseen",
    "Sure and Fleet",
    "Carrion Feeder",
    "Echoes of Stone",
    "Feline Grace",
    "Tree Hanger",
    "Flame Heart",
    "Expanded Fiendish Resistance (Acid)",
    "Expanded Fiendish Resistance (Cold)",
    "Expanded Fiendish Resistance (Electricity)",
    "Expanded Fiendish Resistance (Fire)",
];

fn fixture() -> CharacterInput {
    let text = std::fs::read_to_string(FIXTURE).expect("shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("shared deterministic fixture loads")
}

fn summoner_rows(level: u8) -> Vec<ComputationExplanation> {
    let mut input = fixture();
    input.case_id = Some(format!("sd27_apostrophe.summoner.{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "class:unchained_summoner".to_owned(),
        level,
    }];
    build_pilot_headless_receipt(&input).computation.explanations
}

fn arg_feat_rows() -> Vec<ComputationExplanation> {
    let mut input = fixture();
    input.case_id = Some("sd27_apostrophe.arg_feats".to_owned());
    input.chosen.selected_feats =
        ARG_FEATS_REACHING_THE_SLUGGER.iter().map(|feat| (*feat).to_owned()).collect();
    build_pilot_headless_receipt(&input).computation.explanations
}

fn all_affected_ids() -> Vec<String> {
    let mut ids: Vec<String> =
        summoner_rows(20).into_iter().map(|explanation| explanation.id).collect();
    ids.extend(arg_feat_rows().into_iter().map(|explanation| explanation.id));
    ids
}

/// The two corrected rows, each pinned in both directions: the mangled id is
/// gone and the readable one is present.
#[test]
fn both_mangled_ids_are_replaced_by_their_readable_form() {
    let ids = all_affected_ids();
    for &(before, after) in CORRECTED_IDS {
        assert!(
            !ids.iter().any(|id| id == before),
            "the split-word id {before} must no longer be emitted"
        );
        assert!(ids.iter().any(|id| id == after), "expected {after}; got {ids:?}");
    }
}

/// The label the desktop derives, reproduced here so the fix is pinned at the
/// thing a player actually reads rather than only at the id.
///
/// This is `classFeaturesModel.ts::humanise` transcribed: split the remaining
/// id segments on `[\s._]+`, title-case each word, join with spaces.
#[test]
fn the_rendered_label_reads_makers_call() {
    fn humanise(id_tail: &str) -> String {
        id_tail
            .split(['.', '_', ' '])
            .filter(|word| !word.is_empty())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    let ids = all_affected_ids();
    let makers_call = ids
        .iter()
        .find(|id| id.ends_with("makers_call"))
        .expect("the Maker's Call roster row is emitted");
    let tail = makers_call
        .rsplit_once("corpus_record.")
        .expect("the roster row carries the record-family segment")
        .1;
    assert_eq!(humanise(tail), "Makers Call", "the label a player reads");
}

/// The guard, not the fix: no id segment emitted anywhere across the affected
/// surfaces may be a bare single letter.
///
/// A single-letter segment is the signature of a word split on intra-word
/// punctuation — it is what `maker_s_call` and `scavenger_s_eye` both were —
/// and no legitimate name produces one. This fails for a record that has not
/// been ingested yet as readily as for the two fixed here, which is the point:
/// the next book's `Hunter's Bond` cannot reopen the defect silently.
#[test]
fn no_live_id_segment_splits_a_word_on_punctuation() {
    let mut offenders: Vec<String> = Vec::new();
    for id in all_affected_ids() {
        if id.split(['.', '_']).any(|segment| segment.len() == 1 && segment.chars().all(|c| c.is_ascii_alphabetic()))
        {
            offenders.push(id);
        }
    }
    assert!(offenders.is_empty(), "id segments split on punctuation: {offenders:?}");
}
