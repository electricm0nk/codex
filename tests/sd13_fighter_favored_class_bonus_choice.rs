//! SD13-E5 Fighter level-1 favored-class bonus choice recognition proof.
//!
//! Grounds the next honest burden named in `explain_fighter_level1_hit_points`'s
//! own doc comment: the PF1 Core Rulebook Favored Class rule (Core Rulebook pg.
//! 31, verified against the Archives of Nethys primary source before writing any
//! code): "Whenever a character gains a level in his favored class, he receives
//! either +1 hit point or +1 skill rank." A Human's favored class is Any (PF1
//! Core Rulebook Human racial traits), which trivially includes Fighter, so the
//! choice applies at Fighter level 1 on this codebase's Human-only Fighter
//! chassis seam.
//!
//! This mirrors the already-landed Sorcerer bloodline choice / Cleric domain
//! choice / Druid nature-bond choice / Monk bonus-feat choice recognition
//! records: a bounded, standalone recognition of WHICH of the two legal options
//! (hp or skill rank) was chosen, plus the rule's own genuinely flat +1
//! magnitude. It is recognition only:
//!
//! - it does NOT apply the +1 to the level-1 hit-point total grounded by
//!   `class_chassis.fighter.level_1_hit_points` (that would require wiring into
//!   the integrated hit-point computation, out of scope for this slice);
//! - it does NOT apply the +1 to any selected-skill-rank total either;
//! - it does NOT implement hit points at levels 2+, Toughness/feat interplay, a
//!   class-skill-rank allocation engine, a general feat-selection engine, or a
//!   general feat-prerequisite engine — all of these stay named-but-unproven.

use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
mod common;
use common::load;

const FIXTURE_HP: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_sd13_favored_class_bonus_hp.txt");
const FIXTURE_SKILL_RANK: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_sd13_favored_class_bonus_skill_rank.txt"
);
const FIXTURE_UNRECOGNIZED: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_sd13_favored_class_bonus_unrecognized.txt"
);
const FIXTURE_ABSENT: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

fn find_explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> Option<&'a ComputationExplanation> {
    computation.explanations.iter().find(|e| e.id == id)
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    find_explanation(computation, id).unwrap_or_else(|| {
        panic!(
            "expected explanation id '{id}', got {:?}",
            computation.explanations
        )
    })
}

#[test]
fn fighter_level1_recognizes_the_hit_point_favored_class_bonus_choice() {
    let input = load(FIXTURE_HP);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.fighter.favored_class_bonus_choice");
    assert_eq!(
        choice.value, 1,
        "the Favored Class rule's own bonus magnitude is a genuinely flat, verifiable +1, \
         regardless of which of the two options is chosen"
    );
    assert!(
        choice.detail.contains("choice:favored_class_bonus")
            && choice.detail.contains("bonus:hp")
            && choice.detail.contains("hit point"),
        "detail must name the recognized hp selection: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("not applied")
            || choice.detail.contains("never applied")
            || choice.detail.contains("standalone"),
        "detail must make clear the +1 is not wired into the level-1 hit-point total: {}",
        choice.detail
    );
}

#[test]
fn fighter_level1_recognizes_the_skill_rank_favored_class_bonus_choice() {
    let input = load(FIXTURE_SKILL_RANK);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.fighter.favored_class_bonus_choice");
    assert_eq!(choice.value, 1);
    assert!(
        choice.detail.contains("choice:favored_class_bonus")
            && choice.detail.contains("bonus:skill_rank")
            && choice.detail.contains("skill rank"),
        "detail must name the recognized skill-rank selection: {}",
        choice.detail
    );
}

#[test]
fn fighter_level1_does_not_fabricate_a_resolved_choice_for_an_unrecognized_selection() {
    let input = load(FIXTURE_UNRECOGNIZED);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.fighter.favored_class_bonus_choice");
    assert_eq!(
        choice.value, 0,
        "an unrecognized selection must not carry the fabricated +1 magnitude"
    );
    assert!(
        !choice.detail.contains("hit point option") && !choice.detail.contains("skill rank option"),
        "an unrecognized selection must not be reported as a resolved hp/skill-rank choice: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("bonus:gold"),
        "the acknowledgment record must echo the received out-of-list selection verbatim: {}",
        choice.detail
    );
}

#[test]
fn fighter_level1_emits_no_favored_class_bonus_record_when_the_choice_slot_is_absent() {
    let input = load(FIXTURE_ABSENT);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        find_explanation(&computation, "class_chassis.fighter.favored_class_bonus_choice")
            .is_none(),
        "no favored-class bonus choice record should be fabricated when the fixture carries no \
         choice:favored_class_bonus selection at all"
    );
}

#[test]
fn fighter_level1_favored_class_bonus_choice_changes_no_derived_output_total() {
    let input = load(FIXTURE_HP);
    let computation = compute_pilot_base_chassis(&input);

    // Standalone: every previously proven derived output on the deterministic L1
    // pilot surface keeps its exact value, and the level-1 hit-point record's own
    // value is unaffected by the new choice-recognition record.
    assert_eq!(computation.base_attack_bonus, 1);
    assert_eq!(computation.baseline_melee_attack_bonus, 6);
    assert_eq!(computation.baseline_armor_class, 17);
    let hit_points = explanation(&computation, "class_chassis.fighter.level_1_hit_points");
    assert_eq!(hit_points.value, 12);
}

#[test]
fn fighter_level1_golden_path_still_emits_zero_claim_blocking_diagnostics_with_the_new_choice() {
    let input = load(FIXTURE_HP);
    let computation = compute_pilot_base_chassis(&input);

    let claim_blocking: Vec<_> = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .collect();
    assert!(
        claim_blocking.is_empty(),
        "recognizing the favored-class bonus choice must not introduce a claim-blocking \
         diagnostic on the Human Fighter L1 golden path, got {claim_blocking:?}"
    );
}
