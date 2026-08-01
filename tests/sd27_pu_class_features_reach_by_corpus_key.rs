//! SD-27 — Pathfinder Unchained's 64 ingested `class_feature` records reach the
//! character sheet **by corpus key**, one row each (2026-07-31).
//!
//! # The defect this closes
//!
//! `reach_gate.rs`'s `OPEN_FINDINGS` entry stated it precisely: PU's class
//! features *did* influence a real player surface, but *which* of the 64 could
//! not be claimed, "because `pilot_compute` names its receipt rows semantically
//! (`class_feature.pu.unchained_rogue.sneak_attack_dice`) while the corpus
//! record is keyed `Unchained Rogue ~ Sneak Attack`, so nothing can join the two
//! without a hand-written mapping — which would be exactly the unexecuted claim
//! that file forbids."
//!
//! Two things were wrong, and only one of them was an identity problem:
//!
//! 1. **Identity.** No receipt row named the record it derived from.
//! 2. **Coverage.** Even with perfect identity, the magnitude groundings could
//!    not have covered the roster: several rows derive from one record (four
//!    come out of `Unchained Barbarian ~ Rage` alone), and many records state no
//!    number at all, so they produced no row. An Unchained Monk 20's sheet named
//!    nothing called "Timeless Body" or "Tongue of the Sun and Moon".
//!
//! `pilot_compute::push_pu_class_feature_records` fixes both: one roster row per
//! record the character holds, each carrying
//! [`pu_class_feature_citation`](codex::rules_core::pilot_compute::pu_class_feature_citation)'s
//! verbatim corpus key.
//!
//! # The denominator is read off disk, never off the tables the engine reads
//!
//! Every count below is derived from `data/corpus/pathfinder_unchained/
//! class_feature/**/*.json` at test time. A pin that recomputes its expectation
//! from the same table the code under test reads pins nothing.
//!
//! # §28's standing guard
//!
//! `decisions.md §28`: *"Every change to [pilot_compute.rs] lands with a test
//! pinning the before/after per affected race or class."* [`GROUNDED_PIN`]
//! carries the pre-change grounded-magnitude counts at level 10, verbatim from
//! `tests/sd27_pu_deferred_features_reach_the_character_sheet.rs`'s own
//! `PU_CLASS_PIN`, and asserts the two agree — this file's own change adds
//! roster rows and touches no grounding branch, so its numbers had to survive
//! it exactly. (They were raised once since, on 2026-08-01, by a later cycle
//! that *did* add grounding branches; see [`GROUNDED_PIN`].)

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterClassLevel, CharacterInput,
};
use codex::rules_core::pilot_compute::{
    build_pilot_headless_receipt, pu_class_feature_cited_key, ComputationExplanation,
};

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

/// `(class token, grounded `class_feature.pu.<class>.*` magnitude rows at level
/// 10)` — copied verbatim from
/// `sd27_pu_deferred_features_reach_the_character_sheet.rs`. Roster rows and the
/// deferral row are excluded, so this moves only when a grounding branch
/// changes, which is exactly what makes it a useful guard for cycles that
/// only add roster rows.
///
/// **Raised 2026-08-01** from `(10, 10, 9, 6)` alongside its source pin, by
/// the cycle that gave 17 prose-derived class features a displayed magnitude.
/// See that file's note, and
/// `tests/sd27_pu_prose_derived_class_features_reach_the_sheet.rs` for the
/// before/after pair and the per-feature reasoning.
///
/// **Monk raised again 2026-08-01**, 12 -> 14, by the cycle that made the
/// Unchained Monk's unarmed strike damage die reach the sheet — two rows, the
/// die face and the die count, at every level. See
/// `tests/sd27_unchained_monk_unarmed_strike_reaches_the_sheet.rs`.
const GROUNDED_PIN: &[(&str, usize)] = &[
    ("unchained_barbarian", 10),
    ("unchained_monk", 14),
    ("unchained_rogue", 11),
    ("unchained_summoner", 11),
];

/// Roster rows emitted at level 20, per class — i.e. every ingested record,
/// since 20 is `MAX_SUPPORTED_LEVEL` for all four classes. Literals, so a
/// dropped record fails here by number before it fails by key.
const ROSTER_ROWS_AT_LEVEL_20: &[(&str, usize)] = &[
    ("unchained_barbarian", 14),
    ("unchained_monk", 18),
    ("unchained_rogue", 15),
    ("unchained_summoner", 17),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// Every corpus `KEY:` on disk for one Unchained class.
fn corpus_keys(class_dir: &str) -> BTreeSet<String> {
    let dir = repo_root()
        .join("data/corpus/pathfinder_unchained/class_feature")
        .join(class_dir);
    let entries = std::fs::read_dir(&dir)
        .unwrap_or_else(|error| panic!("{} is readable: {error}", dir.display()));
    entries
        .flatten()
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
        .map(|entry| {
            let text = std::fs::read_to_string(entry.path()).expect("record file is readable");
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("record file is Shape B v1 JSON");
            value["data"]["key"]
                .as_str()
                .expect("every class_feature record carries data.key")
                .to_owned()
        })
        .collect()
}

fn fixture() -> CharacterInput {
    let text = std::fs::read_to_string(repo_root().join(FIXTURE))
        .expect("shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("shared deterministic fixture loads")
}

fn explanations_for(class_token: &str, level: u8) -> Vec<ComputationExplanation> {
    let mut input = fixture();
    input.case_id = Some(format!("sd27_pu_corpus_records.{class_token}.level{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_token}"),
        level,
    }];
    build_pilot_headless_receipt(&input)
        .computation
        .explanations
}

/// The claim itself: every ingested record is named, by its own corpus key, on
/// a row the character sheet renders.
#[test]
fn every_ingested_class_feature_record_is_cited_by_a_receipt_row_at_level_20() {
    for (class_token, class_dir) in PU_CLASSES {
        let ingested = corpus_keys(class_dir);
        assert!(
            !ingested.is_empty(),
            "{class_dir} must hold ingested records; an empty denominator would make this test \
             pass while checking nothing"
        );

        let cited: BTreeSet<String> = explanations_for(class_token, 20)
            .iter()
            .filter_map(|explanation| {
                pu_class_feature_cited_key(&explanation.detail).map(str::to_owned)
            })
            .collect();

        let missing: Vec<&String> = ingested.difference(&cited).collect();
        assert!(
            missing.is_empty(),
            "{class_token}: {} ingested class_feature record(s) are cited by no receipt row, so \
             nothing carries them to the sheet: {missing:?}",
            missing.len()
        );

        let unknown: Vec<&String> = cited.difference(&ingested).collect();
        assert!(
            unknown.is_empty(),
            "{class_token}: receipt rows cite record key(s) that exist in no ingested corpus \
             record — a citation nothing backs is worse than none: {unknown:?}"
        );
    }
}

/// The roster is complete and the count is a literal, so a record silently
/// dropped from a table fails here even if the corpus file is deleted with it.
#[test]
fn each_class_emits_one_roster_row_per_ingested_record() {
    for (class_token, expected) in ROSTER_ROWS_AT_LEVEL_20 {
        let prefix = format!("class_feature.pu.{class_token}.corpus_record.");
        let rows = explanations_for(class_token, 20)
            .iter()
            .filter(|explanation| explanation.id.starts_with(&prefix))
            .count();
        assert_eq!(rows, *expected, "{class_token} roster rows at level 20");
    }

    let total: usize = ROSTER_ROWS_AT_LEVEL_20.iter().map(|(_, count)| count).sum();
    let ingested: usize = PU_CLASSES
        .iter()
        .map(|(_, class_dir)| corpus_keys(class_dir).len())
        .sum();
    assert_eq!(
        total, ingested,
        "the four rosters must sum to the ingested record count on disk"
    );
    assert_eq!(ingested, 64, "PU's ingested class_feature roster is 64 records");
}

/// A row above the character's level is **not** emitted. The sheet lists the
/// features a character has, not the ones they might one day get, and a roster
/// that overstated would be the same defect in the other direction.
#[test]
fn a_feature_the_character_has_not_reached_yet_is_not_listed() {
    let level_one = explanations_for("unchained_rogue", 1);
    let ids: BTreeSet<&str> = level_one.iter().map(|e| e.id.as_str()).collect();

    assert!(
        ids.contains("class_feature.pu.unchained_rogue.corpus_record.sneak_attack"),
        "Sneak Attack is granted at rogue level 1"
    );
    assert!(
        !ids.contains("class_feature.pu.unchained_rogue.corpus_record.master_strike"),
        "Master Strike is granted at rogue level 20 and must not appear on a level-1 sheet"
    );

    let level_twenty = explanations_for("unchained_rogue", 20);
    assert!(
        level_twenty
            .iter()
            .any(|e| e.id == "class_feature.pu.unchained_rogue.corpus_record.master_strike"),
        "and it must appear once the character reaches level 20"
    );
}

/// The five records no progression row grants reach the sheet's "Not computed"
/// lane rather than reaching nothing.
///
/// Derived, not assumed: these are exactly the rows whose tables carry
/// `is_granted: false` (Barbarian) or a `None` `min_level` (Rogue), each
/// documented in its own module as reached indirectly through a sibling record
/// the class does grant.
#[test]
fn the_five_never_granted_records_are_stated_rather_than_dropped() {
    let expected: &[(&str, &str)] = &[
        ("unchained_barbarian", "Unchained Barbarian ~ Uncanny Dodge"),
        (
            "unchained_barbarian",
            "Unchained Barbarian ~ Improved Uncanny Dodge",
        ),
        ("unchained_barbarian", "Unchained Rage"),
        ("unchained_rogue", "Unchained Rogue ~ Uncanny Dodge"),
        (
            "unchained_rogue",
            "Unchained Rogue ~ Improved Uncanny Dodge",
        ),
    ];

    let mut found = 0usize;
    for (class_token, key) in expected {
        let row = explanations_for(class_token, 20)
            .into_iter()
            .find(|explanation| pu_class_feature_cited_key(&explanation.detail) == Some(key))
            .unwrap_or_else(|| panic!("{key} must be cited by a receipt row"));
        assert!(
            row.id.ends_with(".unsupported"),
            "{key} is never granted, so its row must land in the sheet's 'Not computed' lane, \
             not among the computed features: {}",
            row.id
        );
        assert!(
            row.detail.contains("no progression row grants it"),
            "{key}'s row must state the corpus fact rather than showing an empty magnitude: {}",
            row.detail
        );
        found += 1;
    }
    assert_eq!(found, 5);

    // And the count is pinned both ways: a sixth `.unsupported` roster row is a
    // record that stopped being granted, which is a real change to check.
    let unsupported: usize = PU_CLASSES
        .iter()
        .map(|(class_token, _)| {
            let prefix = format!("class_feature.pu.{class_token}.corpus_record.");
            explanations_for(class_token, 20)
                .iter()
                .filter(|e| e.id.starts_with(&prefix) && e.id.ends_with(".unsupported"))
                .count()
        })
        .sum();
    assert_eq!(unsupported, 5);
}

/// Receipt ids are the sheet's row keys and its audit handles. Two records
/// slugging to the same id would silently merge on screen.
#[test]
fn pu_class_feature_receipt_ids_are_unique_within_each_class() {
    for (class_token, _) in PU_CLASSES {
        let ids: Vec<String> = explanations_for(class_token, 20)
            .iter()
            .map(|e| e.id.clone())
            .collect();
        let unique: BTreeSet<&String> = ids.iter().collect();
        assert_eq!(
            unique.len(),
            ids.len(),
            "{class_token} emits a duplicate explanation id"
        );
    }
}

/// Every roster row carries readable payload beyond the citation — the bar
/// `reach_gate.rs` applies. A row that were only a key would be the Feats-tab
/// defect again.
#[test]
fn every_roster_row_says_more_than_the_key_it_cites() {
    for (class_token, _) in PU_CLASSES {
        let prefix = format!("class_feature.pu.{class_token}.corpus_record.");
        for explanation in explanations_for(class_token, 20)
            .iter()
            .filter(|e| e.id.starts_with(&prefix))
        {
            let key = pu_class_feature_cited_key(&explanation.detail)
                .unwrap_or_else(|| panic!("{} carries no citation", explanation.id));
            let without_citation = explanation.detail.replace(key, "");
            assert!(
                without_citation.len() > 60,
                "{} says nothing beyond its own key: {}",
                explanation.id,
                explanation.detail
            );
        }
    }
}

/// §28's standing guard: the grounded magnitude rows are byte-for-byte the same
/// population they were before this change.
#[test]
fn the_grounded_magnitude_rows_are_unchanged_at_level_10() {
    for (class_token, expected) in GROUNDED_PIN {
        let prefix = format!("class_feature.pu.{class_token}.");
        let roster_prefix = format!("class_feature.pu.{class_token}.corpus_record.");
        let grounded = explanations_for(class_token, 10)
            .iter()
            .filter(|e| {
                e.id.starts_with(&prefix)
                    && !e.id.starts_with(&roster_prefix)
                    && !e.id.ends_with(".unsupported")
            })
            .count();
        assert_eq!(
            grounded, *expected,
            "{class_token}'s grounded magnitude rows at level 10 must match GROUNDED_PIN; the \
             roster-row change this file was written for touches no grounding branch, and the \
             2026-08-01 prose-derived-magnitude cycle raised the literals in step with the pin \
             it is copied from"
        );
    }
}

/// The deferral row this file's sibling test pins is untouched: it is the
/// class's honest prose remainder, and the per-record roster does not replace
/// it.
#[test]
fn the_per_class_deferral_row_still_reaches_the_explanation_channel() {
    for (class_token, _) in PU_CLASSES {
        let id = format!("class_feature.pu.{class_token}.other_features_deferred.unsupported");
        assert_eq!(
            explanations_for(class_token, 20)
                .iter()
                .filter(|e| e.id == id)
                .count(),
            1,
            "{class_token} must still emit exactly one `{id}`"
        );
    }
}

/// The citation seam round-trips, including across the prefix collision that
/// makes the backtick delimiters load-bearing.
#[test]
fn the_citation_seam_round_trips_and_resists_the_prefix_collision() {
    use codex::rules_core::pilot_compute::pu_class_feature_citation;

    let rage = pu_class_feature_citation("Unchained Barbarian ~ Rage", 290);
    let rage_powers = pu_class_feature_citation("Unchained Barbarian ~ Rage Powers", 291);

    assert_eq!(
        pu_class_feature_cited_key(&rage),
        Some("Unchained Barbarian ~ Rage")
    );
    assert_eq!(
        pu_class_feature_cited_key(&rage_powers),
        Some("Unchained Barbarian ~ Rage Powers")
    );
    assert_ne!(
        pu_class_feature_cited_key(&rage_powers),
        Some("Unchained Barbarian ~ Rage"),
        "a substring match here would report the wrong record as reached"
    );
    assert_eq!(pu_class_feature_cited_key("no citation in this text"), None);
}

/// The four rosters carry the same keys the four corpus directories do — a
/// cross-check that the normalisation over the four differently-shaped tables
/// did not silently rename anything.
#[test]
fn the_receipt_keys_and_the_corpus_directories_agree_exactly() {
    let mut by_class: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    for (class_token, class_dir) in PU_CLASSES {
        by_class.insert(class_token, corpus_keys(class_dir));
    }

    let all_on_disk: BTreeSet<String> = by_class.values().flatten().cloned().collect();
    let all_cited: BTreeSet<String> = PU_CLASSES
        .iter()
        .flat_map(|(class_token, _)| {
            explanations_for(class_token, 20)
                .into_iter()
                .filter_map(|e| pu_class_feature_cited_key(&e.detail).map(str::to_owned))
                .collect::<Vec<String>>()
        })
        .collect();

    assert_eq!(all_on_disk, all_cited);
    assert_eq!(all_on_disk.len(), 64);
}
