//! GE06-E2-F2d selected deterministic skill modifiers gate.
//!
//! Proves the GE-06 pilot compute surface can compute and explain the first
//! bounded selected skill modifiers — Climb, Intimidate, and Swim — for the
//! accepted deterministic pilot, applying the already-grounded Chain Shirt
//! armor-check penalty only to the armor-check skills (Climb, Swim), and refuses
//! to fabricate them when the deterministic Chain Shirt posture is absent or
//! widened. Each total reads the character's own rank, on the headless path and
//! on the corpus path the sheet reads; no rank reads untrained. It does not
//! assert a broad skill engine, racial/item-skill bonuses, encumbrance, parity,
//! or UI.

use std::path::Path;

use codex::rules_core::corpus_loader::{BookCorpusRoot, load_book_corpus};
use codex::rules_core::pilot_compute::{
    PilotBaseChassisComputation, build_pilot_headless_receipt, compute_pilot_base_chassis,
};
use codex::rules_core::pilot_compute_corpus::compute_selected_skill_modifiers_from_corpus;
mod common;
use common::{load, explanation};

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

#[test]
fn computes_selected_skill_modifiers_with_contributors() {
    let input = load(DETERMINISTIC_FIXTURE);

    let computation = compute_pilot_base_chassis(&input);

    // Climb: rank 1 + STR +4 (CG-03 fix: the Human +2 racial bonus is now applied to
    // the base 16 before the modifier is derived) + class-skill +3 + Chain Shirt
    // armor-check -2 = 6.
    assert_eq!(computation.selected_skill_modifiers.climb, 6);
    // Intimidate: rank 1 + CHA -1 + class-skill +3 = 3 (no armor-check; CHA-based;
    // unaffected by the Strength fix).
    assert_eq!(computation.selected_skill_modifiers.intimidate, 3);
    // Swim: rank 1 + STR +4 + class-skill +3 + Chain Shirt armor-check -2 = 6.
    assert_eq!(computation.selected_skill_modifiers.swim, 6);

    let climb = explanation(&computation, "skill.selected_modifier.climb");
    assert_eq!(climb.value, 6);
    assert!(
        climb.detail.contains("rank"),
        "climb detail must cite the rank allocation: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains("Strength"),
        "climb detail must cite the Strength modifier: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains("class-skill"),
        "climb detail must cite the class-skill bonus: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains("armor-check"),
        "climb detail must cite the Chain Shirt armor-check penalty: {}",
        climb.detail
    );
    assert!(
        climb.detail.contains('6'),
        "climb detail must cite the total: {}",
        climb.detail
    );

    let intimidate = explanation(&computation, "skill.selected_modifier.intimidate");
    assert_eq!(intimidate.value, 3);
    assert!(
        intimidate.detail.contains("rank"),
        "intimidate detail must cite the rank allocation: {}",
        intimidate.detail
    );
    assert!(
        intimidate.detail.contains("Charisma"),
        "intimidate detail must cite the Charisma modifier: {}",
        intimidate.detail
    );
    assert!(
        intimidate.detail.contains("class-skill"),
        "intimidate detail must cite the class-skill bonus: {}",
        intimidate.detail
    );
    // Intimidate is not an armor-check skill; it must not claim an armor-check penalty.
    assert!(
        !intimidate.detail.contains("armor-check"),
        "intimidate detail must not cite an armor-check penalty: {}",
        intimidate.detail
    );
    assert!(
        intimidate.detail.contains('3'),
        "intimidate detail must cite the total: {}",
        intimidate.detail
    );

    let swim = explanation(&computation, "skill.selected_modifier.swim");
    assert_eq!(swim.value, 6);
    assert!(
        swim.detail.contains("rank"),
        "swim detail must cite the rank allocation: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains("Strength"),
        "swim detail must cite the Strength modifier: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains("class-skill"),
        "swim detail must cite the class-skill bonus: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains("armor-check"),
        "swim detail must cite the Chain Shirt armor-check penalty: {}",
        swim.detail
    );
    assert!(
        swim.detail.contains('6'),
        "swim detail must cite the total: {}",
        swim.detail
    );

    // The supported deterministic pilot must not raise a claim-blocking diagnostic.
    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "supported deterministic pilot should not block claims: {:?}",
        computation.diagnostics
    );
}

/// The fixture with its Climb, Intimidate and Swim rows swapped for the given ranks; `None` writes no
/// row at all. `feats` go in as extra `feat=` lines.
fn fighter_at_ranks(climb: Option<u8>, intimidate: Option<u8>, swim: Option<u8>, feats: &[&str]) -> String {
    let mut text = String::new();
    for line in DETERMINISTIC_FIXTURE.lines() {
        let ranks = match line {
            "skill=skill:climb:1" => Some(("climb", climb)),
            "skill=skill:intimidate:1" => Some(("intimidate", intimidate)),
            "skill=skill:swim:1" => Some(("swim", swim)),
            _ => None,
        };
        match ranks {
            Some((skill, Some(rank))) => text.push_str(&format!("skill=skill:{skill}:{rank}\n")),
            Some((_, None)) => {}
            None => {
                text.push_str(line);
                text.push('\n');
            }
        }
    }
    for feat in feats {
        text.push_str(&format!("feat={feat}\n"));
    }
    text
}

/// (Climb, Intimidate, Swim) from both twins, after holding them to each other. The corpus path is the
/// one `resolve_unified_pilot_snapshot` gates on, so the one the sheet renders.
fn both_twins(fixture: &str) -> (i16, i16, i16) {
    let input = load(fixture);
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("apps/desktop/src-tauri/resources/corpus_fixtures");
    let corpus = load_book_corpus(&[BookCorpusRoot { book_id: "desktop_ui_fixtures", dir: dir.as_path() }]);
    let receipt = build_pilot_headless_receipt(&input);
    let headless: &PilotBaseChassisComputation = &receipt.computation;
    assert!(
        !headless.diagnostics.iter().any(|d| d.id == "skill.selected_modifier.unsupported"),
        "a rank is never a refusal: {:?}",
        headless.diagnostics
    );
    let corpus_path = compute_selected_skill_modifiers_from_corpus(headless, &input, &corpus)
        .expect("the corpus path computes this character");
    let totals = &headless.selected_skill_modifiers;
    assert_eq!(
        (corpus_path.climb, corpus_path.intimidate, corpus_path.swim),
        (totals.climb, totals.intimidate, totals.swim),
        "the two compute twins must read the same rank"
    );
    (totals.climb, totals.intimidate, totals.swim)
}

/// STR +4, CHA -1, Fighter class skills, Chain Shirt -2 on Climb and Swim. A rank is the same number on
/// both paths and is not capped by the level: a 3.5e file's cross-the-cap rank still reads.
#[test]
fn selected_skill_reads_the_characters_own_rank_on_both_twins() {
    let totals = both_twins(&fighter_at_ranks(Some(13), Some(5), Some(2), &[]));
    assert_eq!(totals, (13 + 4 + 3 - 2, 5 - 1 + 3, 2 + 4 + 3 - 2));

    let computation = compute_pilot_base_chassis(&load(&fighter_at_ranks(Some(13), Some(5), Some(2), &[])));
    let climb = explanation(&computation, "skill.selected_modifier.climb");
    assert_eq!(climb.value, 18);
    assert!(climb.detail.contains("rank 13"), "the row names the real rank: {}", climb.detail);
}

/// PF1 (CRB Chapter 4): no rank is the ability modifier and armor-check penalty, with no +3, on a class
/// skill or not. A row at 0 and no row are the same character.
#[test]
fn selected_skill_with_no_rank_reads_untrained_on_both_twins() {
    let no_row = both_twins(&fighter_at_ranks(None, None, None, &[]));
    assert_eq!(no_row, (4 - 2, -1, 4 - 2));
    assert_eq!(both_twins(&fighter_at_ranks(Some(0), Some(0), Some(0), &[])), no_row);

    let swim_only_missing = both_twins(&fighter_at_ranks(Some(1), Some(1), None, &[]));
    assert_eq!(swim_only_missing, (6, 3, 2));

    let computation = compute_pilot_base_chassis(&load(&fighter_at_ranks(None, None, None, &[])));
    let swim = explanation(&computation, "skill.selected_modifier.swim");
    assert!(swim.detail.contains("rank 0"), "{}", swim.detail);
    assert!(swim.detail.contains("no ranks in Swim"), "{}", swim.detail);
}

/// Seeds sit at rank 1, so this is the fixture's own 6 / 3 / 6.
#[test]
fn selected_skill_at_rank_one_does_not_move() {
    assert_eq!(both_twins(DETERMINISTIC_FIXTURE), (6, 3, 6));
}

/// Athletic and Persuasive (CRB): "If you have 10 or more ranks in one of these skills, the bonus
/// increases to +4 for that skill." One rule per skill, on each skill's own rank; the record is
/// `data/sheet_rules/core_rulebook/feat/{athletic,persuasive}.json`.
#[test]
fn selected_skill_feat_bonus_rises_at_ten_ranks_in_that_skill_alone() {
    let athletic = ["feat:athletic"];
    let persuasive = ["feat:persuasive"];
    assert_eq!(both_twins(&fighter_at_ranks(Some(9), Some(1), Some(1), &athletic)), (9 + 4 + 3 - 2 + 2, 3, 6 + 2));
    assert_eq!(both_twins(&fighter_at_ranks(Some(10), Some(1), Some(1), &athletic)), (10 + 4 + 3 - 2 + 4, 3, 6 + 2));
    assert_eq!(both_twins(&fighter_at_ranks(Some(1), Some(1), Some(10), &athletic)), (6 + 2, 3, 10 + 4 + 3 - 2 + 4));
    assert_eq!(both_twins(&fighter_at_ranks(Some(1), Some(9), Some(1), &persuasive)), (6, 9 - 1 + 3 + 2, 6));
    assert_eq!(both_twins(&fighter_at_ranks(Some(1), Some(10), Some(1), &persuasive)), (6, 10 - 1 + 3 + 4, 6));
}

/// SD-36 F7a (F7-8): a rank in a skill OUTSIDE the Climb/Intimidate/Swim slice feeds none of the
/// three totals the slice computes, so it no longer refuses the slice (it used to: this test was
/// `widened_selected_skill_allocation_blocks_skill_modifiers`, and every real allocation -- a
/// Wizard's Spellcraft -- came back Blocked). The three totals and their explanations are exactly
/// the unwidened fixture's.
#[test]
fn an_allocation_outside_the_slice_leaves_the_three_totals_unchanged() {
    let mutated = DETERMINISTIC_FIXTURE.replace(
        "skill=skill:swim:1\n",
        "skill=skill:swim:1\nskill=skill:stealth:1\n",
    );
    assert!(
        mutated.contains("skill=skill:stealth:1"),
        "test setup should have widened the skill allocations"
    );
    let widened = compute_pilot_base_chassis(&load(&mutated));
    let baseline = compute_pilot_base_chassis(&load(DETERMINISTIC_FIXTURE));

    assert!(
        !widened.diagnostics.iter().any(|d| d.id == "skill.selected_modifier.unsupported"),
        "an out-of-slice allocation must not refuse the slice: {:?}",
        widened.diagnostics
    );
    assert_eq!(widened.selected_skill_modifiers, baseline.selected_skill_modifiers);
    let selected = |c: &codex::rules_core::pilot_compute::PilotBaseChassisComputation| {
        c.explanations
            .iter()
            .filter(|e| e.id.starts_with("skill.selected_modifier."))
            .map(|e| (e.id.clone(), e.value))
            .collect::<Vec<_>>()
    };
    assert_eq!(selected(&widened), selected(&baseline));
    assert_eq!(selected(&widened).len(), 3);
}

#[test]
fn absent_chain_shirt_reads_no_armor_check_penalty() {
    // No armor is required: with the Chain Shirt gone the two armor-check skills lose its penalty
    // and the character still computes. Fixture: Str +4, Cha -1, rank 1 and a class skill in each.
    let mutated = DETERMINISTIC_FIXTURE.replace(
        "equipment=item:chain_shirt:equipped_worn_active",
        "equipment=item:chain_shirt:absent",
    );
    assert!(
        mutated.contains("equipment=item:chain_shirt:absent"),
        "test setup should have removed the Chain Shirt"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation.diagnostics.iter().any(|d| d.claim_blocking),
        "no armor is not a refusal: {:?}",
        computation.diagnostics
    );
    let skills = &computation.selected_skill_modifiers;
    assert_eq!((skills.climb, skills.intimidate, skills.swim), (8, 3, 8));
}

#[test]
fn unsupported_chassis_blocks_skill_modifiers() {
    // Replace the Fighter level-1 chassis with a Monk level-1 chassis. The
    // selected-skill surface is grounded only on the Fighter class posture
    // and must refuse rather than fabricate Fighter class-skill bonuses.
    // The negative control is a SYNTHETIC class id, not a real class. It
    // was Rogue level-1, then Cleric, then Barbarian, then Monk -- each
    // stopped being unsupported the moment `table_class_id` learned it
    // (see ge06_failure_classifier.rs for why Rogue went first). Monk was
    // the LAST real class outside that mapping (v0.6 alpha swarm,
    // Monk/Summoner chassis-recognition closure, 2026-07-29), so all 27
    // base classes are now recognized and no real class can serve here
    // again.
    let mutated = DETERMINISTIC_FIXTURE
        .replace("class_level=class:fighter:1", "class_level=class:not_a_real_pf1_class:1");
    assert!(
        mutated.contains("class_level=class:not_a_real_pf1_class:1"),
        "test setup should have mutated the class chassis"
    );
    let input = load(&mutated);

    let computation = compute_pilot_base_chassis(&input);

    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "unsupported chassis must produce a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("skill.selected_modifier.")),
        "unsupported chassis must withhold selected-skill explanations: {:?}",
        computation.explanations
    );
}
