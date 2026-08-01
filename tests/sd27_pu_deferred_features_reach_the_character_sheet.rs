//! SD-27 — the `decisions.md §28` standing-guard pin for making Pathfinder
//! Unchained's deferred-feature notice reach a player (2026-07-31).
//!
//! # The defect this pins closed
//!
//! Each Unchained class emits an `other_features_deferred.unsupported` record
//! naming what it does *not* compute — for Unchained Rogue that includes
//! Debilitating Injury, one of the class's headline features. It was pushed on
//! the **diagnostic** channel only.
//!
//! No player could ever see it. `CharacterSheet.tsx` renders
//! `LoadSavedCharacterResponse.explanations`; diagnostics reach the frontend
//! only through `buildCreateCharacterOutcomeSurface` when a build comes back
//! `Blocked`, and this diagnostic is deliberately `claim_blocking: false`, so
//! the build is never blocked and the message is never carried. Meanwhile
//! `classFeaturesModel.ts` has always had a "Not computed" lane built
//! specifically for `.unsupported` records — and, before this change, *nothing
//! in this engine emitted an `.unsupported` explanation at all*, so that lane
//! was dead code.
//!
//! Two written claims said otherwise and were both wrong; both were corrected
//! alongside this test (`reach_gate.rs`'s `OPEN_FINDINGS` entry for
//! `pathfinder_unchained`/`class_features`, which asserted "the deferred set
//! is visible to a player").
//!
//! # What this pins, and how the numbers were obtained
//!
//! §28's condition, verbatim: *"Every change to it lands with a test pinning
//! the before/after per affected race or class, so drift is a caught failure
//! rather than a silent recomputation."*
//!
//! [`PU_CLASS_PIN`] carries, per Unchained class, the number of **grounded**
//! `class_feature.pu.<class>.*` explanations — the deferral row itself
//! excluded. That is the "before" number by construction: this change adds one
//! `.unsupported` explanation per class and touches no grounding branch, so
//! the grounded count is exactly what it was. The literals were read off the
//! real compute pipeline at level 10 (`cargo test --test
//! sd27_pu_deferred_features_reach_the_character_sheet`) and are literals, not
//! expressions over the same tables the engine reads — a pin that recomputes
//! its own expectation pins nothing.
//!
//! The chassis and hit-point side of these four classes is already pinned by
//! `tests/sd27_pu_class_wiring_pin.rs`; this file deliberately does not
//! duplicate it.

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterClassLevel, CharacterInput,
};
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;

/// The same shared deterministic fixture `sd27_pu_class_wiring_pin.rs` uses,
/// so the two pins describe the same posture rather than two different ones.
const FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

const LEVEL: u8 = 10;

/// `(class token, grounded `class_feature.pu.<class>.*` explanation count at
/// level 10)`.
///
/// Level 10 rather than 20 on purpose: it is the level the four Unchained
/// classes were verified at on screen, and it is mid-table, so a progression
/// that silently loses its top rows *or* its bottom ones moves this number.
/// Several features gate above level 10 (Unchained Barbarian's Indomitable
/// Will at 14, for one), so these are the counts at 10 and are not the
/// classes' full feature totals.
///
/// **Raised 2026-08-01** from `(10, 10, 9, 6)` by the cycle that gave the 17
/// prose-derived class features a displayed magnitude — Monk `+2` (Evasion,
/// Improved Evasion), Rogue `+2` (Debilitating Injury's two penalties),
/// Summoner `+5` (Life Link, Bond Senses, Shield Ally, Maker's Call, Aspect).
/// Barbarian does not move because all three of its newly-closed features
/// gate above level 10. The before/after pair and the per-feature reasoning
/// live in `tests/sd27_pu_prose_derived_class_features_reach_the_sheet.rs`.
///
/// **Monk raised again 2026-08-01**, 12 -> 14, by the cycle that made the
/// Unchained Monk's unarmed strike damage die reach the sheet: it adds the
/// die-face and die-count rows at every level, so this pin gains exactly two.
/// The before/after pair and the corpus evidence that the progression is the
/// Core Rulebook Monk's own live in
/// `tests/sd27_unchained_monk_unarmed_strike_reaches_the_sheet.rs`.
const PU_CLASS_PIN: &[(&str, usize)] = &[
    ("unchained_barbarian", 10),
    ("unchained_monk", 14),
    ("unchained_rogue", 11),
    ("unchained_summoner", 11),
];

fn fixture() -> CharacterInput {
    let text = std::fs::read_to_string(FIXTURE).expect("shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("shared deterministic fixture loads")
}

fn receipt_for(class_token: &str) -> codex::rules_core::pilot_compute::PilotHeadlessReceipt {
    let mut input = fixture();
    input.case_id =
        Some(format!("sd27_pu_deferred_features.{class_token}.level{LEVEL}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_token}"),
        level: LEVEL,
    }];
    build_pilot_headless_receipt(&input)
}

/// The fix itself: the deferral is on the channel the character sheet reads.
#[test]
fn every_unchained_class_puts_its_deferred_feature_notice_on_the_explanation_channel() {
    for (class_token, _) in PU_CLASS_PIN {
        let receipt = receipt_for(class_token);
        let id = format!("class_feature.pu.{class_token}.other_features_deferred.unsupported");

        let deferred: Vec<_> = receipt
            .computation
            .explanations
            .iter()
            .filter(|explanation| explanation.id == id)
            .collect();

        assert_eq!(
            deferred.len(),
            1,
            "{class_token} must emit exactly one deferred-feature explanation \
             (`{id}`); the character sheet's 'Not computed' lane reads the \
             explanation channel, never the diagnostic channel"
        );
        assert!(
            !deferred[0].detail.trim().is_empty(),
            "{class_token}'s deferred-feature explanation must carry the engine's own \
             text — the sheet renders `detail` verbatim and has nothing else to show"
        );
    }
}

/// The `.unsupported` suffix is not decoration: `classFeaturesModel.ts` keys
/// its "Not computed" lane off it, and a record that loses the suffix silently
/// becomes a feature row rendering its filler zero as a magnitude.
#[test]
fn the_deferred_notice_keeps_the_unsupported_suffix_the_sheet_routes_on() {
    for (class_token, _) in PU_CLASS_PIN {
        let receipt = receipt_for(class_token);
        let id = format!("class_feature.pu.{class_token}.other_features_deferred.unsupported");
        assert!(
            receipt.computation.explanations.iter().any(|e| e.id == id),
            "{class_token}: `{id}` must survive verbatim"
        );
    }
}

/// Both channels still carry it, and they carry the *same* text. The
/// diagnostic was kept rather than replaced — `pf1_adapter`'s creation path
/// and the headless receipt consumers read it — so this guards the two from
/// drifting into telling a player and an auditor different stories.
#[test]
fn the_two_channels_carry_byte_identical_text_and_the_diagnostic_stays_non_blocking() {
    for (class_token, _) in PU_CLASS_PIN {
        let receipt = receipt_for(class_token);
        let id = format!("class_feature.pu.{class_token}.other_features_deferred.unsupported");

        let diagnostic = receipt
            .computation
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.id == id)
            .unwrap_or_else(|| panic!("{class_token}: the deferred-feature diagnostic must still be emitted"));
        let explanation = receipt
            .computation
            .explanations
            .iter()
            .find(|explanation| explanation.id == id)
            .unwrap_or_else(|| panic!("{class_token}: the deferred-feature explanation must be emitted"));

        assert_eq!(
            diagnostic.message, explanation.detail,
            "{class_token}: the diagnostic and the explanation must carry one text from one source"
        );
        assert!(
            !diagnostic.claim_blocking,
            "{class_token}: naming a deferral must never block a build — the class computes real \
             magnitudes and the player is entitled to them"
        );
    }
}

/// The §28 before/after pin. A change here is either a real rules correction
/// (update the literal *and say why in the commit*) or the silent
/// recomputation §28 exists to catch.
#[test]
fn every_unchained_class_grounds_its_pinned_number_of_real_class_features() {
    // Every class is measured before anything is asserted, so one drifted
    // class does not hide the other three behind a fail-fast panic.
    let actual: Vec<(&str, usize, Vec<String>)> = PU_CLASS_PIN
        .iter()
        .map(|(class_token, _)| {
            let receipt = receipt_for(class_token);
            let prefix = format!("class_feature.pu.{class_token}.");
            // The corpus-record roster rows are excluded, and the exclusion is
            // what keeps this pin meaning what it says. It counts **grounded
            // magnitudes** — rows whose `value` is a number this engine
            // derived. `class_feature.pu.<class>.corpus_record.*` rows are the
            // separate per-record roster
            // `pilot_compute::push_pu_class_feature_records` added on
            // 2026-07-31 (so PU's 64 ingested `class_feature` records could be
            // claimed by key rather than in aggregate); their value is the
            // corpus grant level, not a derivation. Folding them in here would
            // silently redefine the number this pin protects and would have
            // moved it from 10/10/9/6 to 17/17/15/13 without any grounding
            // branch changing. Their own counts are pinned by
            // `tests/sd27_pu_class_features_reach_by_corpus_key.rs`.
            let roster_prefix = format!("class_feature.pu.{class_token}.corpus_record.");
            let grounded: Vec<String> = receipt
                .computation
                .explanations
                .iter()
                .filter(|explanation| explanation.id.starts_with(&prefix))
                .filter(|explanation| !explanation.id.starts_with(&roster_prefix))
                .filter(|explanation| !explanation.id.ends_with(".unsupported"))
                .map(|explanation| explanation.id.clone())
                .collect();
            (*class_token, grounded.len(), grounded)
        })
        .collect();

    let drifted: Vec<String> = actual
        .iter()
        .zip(PU_CLASS_PIN.iter())
        .filter(|((_, count, _), (_, pin))| count != pin)
        .map(|((class_token, count, ids), (_, pin))| {
            format!("{class_token}: pinned {pin}, computed {count}; ids: {ids:#?}")
        })
        .collect();

    assert!(
        drifted.is_empty(),
        "grounded class-feature counts drifted at level {LEVEL}:\n{}",
        drifted.join("\n")
    );
}

/// The one feature the defect report named. Unchained Rogue's Debilitating
/// Injury carries no numeric token in the corpus, so nothing is computed for
/// it — which is correct, and was invisible. The player must now be able to
/// read that on the sheet.
#[test]
fn an_unchained_rogue_sheet_can_name_debilitating_injury_as_not_computed() {
    let receipt = receipt_for("unchained_rogue");
    let explanation = receipt
        .computation
        .explanations
        .iter()
        .find(|explanation| {
            explanation.id == "class_feature.pu.unchained_rogue.other_features_deferred.unsupported"
        })
        .expect("Unchained Rogue emits its deferred-feature explanation");

    assert!(
        explanation.detail.contains("Debilitating Injury"),
        "the text a player reads must name the headline feature nothing is computed for: {}",
        explanation.detail
    );
}
