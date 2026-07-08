//! SD13-E5 Monk level-1 bonus feat choice recognition proof.
//!
//! Grounds the next honest Monk martial pillar burden: the level-1 bonus feat
//! choice-slot selection, drawn from the PF1 Core Rulebook restricted Monk bonus
//! feat list (Combat Reflexes, Deflect Arrows, Improved Grapple, Improved Trip,
//! Stunning Fist), is recognized as chosen input, mirroring the already-landed
//! Sorcerer bloodline choice / Cleric domain choice / Druid nature-bond choice
//! recognition records. Improved Unarmed Strike is deliberately excluded from the
//! restricted set: the PF1 Core Rulebook grants it to every monk automatically at
//! level 1, separate from this chosen bonus feat, and this codebase does not
//! ground that automatic grant either.
//!
//! This is recognition only. It does not implement what any of the five feats
//! actually do (no attack-of-opportunity engine for Combat Reflexes, no
//! grapple-check engine for Improved Grapple, no trip-check engine for Improved
//! Trip, no DC/save engine for Stunning Fist, no ranged-deflection engine for
//! Deflect Arrows) — those stay named-but-unproven in the blocker note, exactly
//! like Arcane Bond stayed unproven after the Sorcerer bloodline choice was
//! grounded.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};

const MONK_FIXTURE_STUNNING_FIST: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level1_sd13_deterministic_input.txt");
const MONK_FIXTURE_COMBAT_REFLEXES: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level1_sd13_bonus_feat_combat_reflexes.txt"
);
const MONK_FIXTURE_UNRECOGNIZED_CHOICE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level1_sd13_bonus_feat_unrecognized.txt"
);

fn load(fixture: &str) -> CharacterInput {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    result
        .character_input
        .expect("valid fixture should produce a character input record")
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected explanation id '{id}', got {:?}",
                computation.explanations
            )
        })
}

fn claim_blocking<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    let diag = computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        });
    assert!(
        diag.claim_blocking,
        "diagnostic '{id}' must be claim-blocking: {diag:?}"
    );
    diag
}

#[test]
fn monk_level1_recognizes_the_stunning_fist_bonus_feat_choice() {
    let input = load(MONK_FIXTURE_STUNNING_FIST);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(
        choice.value, 0,
        "bonus feat choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Stunning Fist")
            && choice.detail.contains("choice:monk_bonus_feat")
            && choice.detail.contains("feat:stunning_fist"),
        "bonus feat choice recognition must name the recognized selection: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("Improved Unarmed Strike"),
        "bonus feat choice recognition must explain why Improved Unarmed Strike is excluded \
         from the restricted choice set: {}",
        choice.detail
    );

    // Still blocked: the recognized feat's own mechanics remain unproven, and the
    // blocker now names the specific recognized feat.
    let bonus_feat = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.bonus_feat.unsupported",
    );
    assert!(
        bonus_feat.message.contains("bonus feat") && bonus_feat.message.contains("Stunning Fist"),
        "narrowed bonus-feat blocker must name the recognized Stunning Fist selection: {}",
        bonus_feat.message
    );
}

#[test]
fn monk_level1_recognizes_combat_reflexes_as_an_alternate_restricted_selection() {
    let input = load(MONK_FIXTURE_COMBAT_REFLEXES);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(choice.value, 0);
    assert!(
        choice.detail.contains("Combat Reflexes"),
        "bonus feat choice recognition must name Combat Reflexes: {}",
        choice.detail
    );

    let bonus_feat = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.bonus_feat.unsupported",
    );
    assert!(
        bonus_feat.message.contains("Combat Reflexes"),
        "narrowed bonus-feat blocker must name the recognized Combat Reflexes selection: {}",
        bonus_feat.message
    );
}

#[test]
fn monk_level1_does_not_fabricate_recognition_for_an_out_of_list_selection() {
    let input = load(MONK_FIXTURE_UNRECOGNIZED_CHOICE);
    let computation = compute_pilot_base_chassis(&input);

    // A choice slot present but naming a feat outside the restricted list still
    // leaves a bounded acknowledgment record (mirroring the Sorcerer bloodline
    // choice's "present but not recognized" branch), but it must not claim that a
    // specific restricted-list feat was the recognized selection.
    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(choice.value, 0);
    assert!(
        !choice.detail.contains("bonus feat choice recognized"),
        "an out-of-list selection must not be reported as a recognized choice: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("feat:toughness"),
        "the acknowledgment record must echo the received out-of-list selection verbatim: {}",
        choice.detail
    );

    let bonus_feat = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.bonus_feat.unsupported",
    );
    assert!(
        bonus_feat.message.contains("bonus feat"),
        "bonus-feat blocker must stay generic when no restricted-list feat is recognized: {}",
        bonus_feat.message
    );
}
