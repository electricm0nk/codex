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

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::{load, explanation};

const MONK_FIXTURE_DEFLECT_ARROWS: &str =
    include_str!("fixtures/rules_core/pf1_human_monk_level1_sd13_deterministic_input.txt");
const MONK_FIXTURE_COMBAT_REFLEXES: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level1_sd13_bonus_feat_combat_reflexes.txt"
);
const MONK_FIXTURE_UNRECOGNIZED_CHOICE: &str = include_str!(
    "fixtures/rules_core/pf1_human_monk_level1_sd13_bonus_feat_unrecognized.txt"
);

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

// RULES CORRECTION (SD13-E5): an earlier version of this file treated
// feat:stunning_fist as a member of the restricted Monk bonus feat list and
// the seam's list included Improved Trip. Both primary sources (d20pfsrd and
// legacy.aonprd.com, re-read for this correction) give the PF1 Core Rulebook
// 1st/2nd-level list as Catch Off-Guard, Combat Reflexes, Deflect Arrows,
// Dodge, Improved Grapple, Scorpion Style, and Throw Anything — Improved
// Trip joins only at 6th level, and Stunning Fist is an AUTOMATIC 1st-level
// grant ("At 1st level, the monk gains Stunning Fist as a bonus feat, even
// if he does not meet the prerequisites"), never a choice-set member,
// exactly like Improved Unarmed Strike. The deterministic fixtures now
// select feat:deflect_arrows (a genuine list member; Dodge is avoided
// because the Human bonus-feat slot already selects it).
#[test]
fn monk_level1_recognizes_the_deflect_arrows_bonus_feat_choice() {
    let input = load(MONK_FIXTURE_DEFLECT_ARROWS);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(
        choice.value, 0,
        "bonus feat choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("Deflect Arrows")
            && choice.detail.contains("choice:monk_bonus_feat")
            && choice.detail.contains("feat:deflect_arrows"),
        "bonus feat choice recognition must name the recognized selection: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("Catch Off-Guard")
            && choice.detail.contains("Scorpion Style")
            && choice.detail.contains("Throw Anything")
            && choice.detail.contains("including Improved Trip, stay unrecognized"),
        "the recognition must cite the corrected PF1 CRB restricted list (Catch Off-Guard, \
         Combat Reflexes, Deflect Arrows, Dodge, Improved Grapple, Scorpion Style, Throw \
         Anything) and frame Improved Trip as a 6th-level addition outside this seam: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("Improved Unarmed Strike")
            && choice.detail.contains("Stunning Fist"),
        "the recognition must explain why Improved Unarmed Strike AND Stunning Fist are \
         excluded from the restricted choice set (both are automatic 1st-level grants): {}",
        choice.detail
    );

    // Still blocked: the recognized feat's own mechanics remain unproven, and the
    // blocker now names the specific recognized feat.
    let bonus_feat = claim_blocking(
        &computation,
        "class_feature.monk.bounded_progression.bonus_feat.unsupported",
    );
    assert!(
        bonus_feat.message.contains("bonus feat") && bonus_feat.message.contains("Deflect Arrows"),
        "narrowed bonus-feat blocker must name the recognized Deflect Arrows selection: {}",
        bonus_feat.message
    );
}

#[test]
fn monk_level1_stunning_fist_selection_is_the_automatic_grant_not_a_list_member() {
    let stunning = MONK_FIXTURE_DEFLECT_ARROWS
        .replace("feat:deflect_arrows", "feat:stunning_fist");
    let input = load(&stunning);
    let computation = compute_pilot_base_chassis(&input);

    let choice = explanation(&computation, "class_chassis.monk.bonus_feat_choice");
    assert_eq!(choice.value, 0);
    assert!(
        !choice.detail.contains("drawn from the PF1 Core Rulebook restricted"),
        "a stunning_fist selection must NOT be recognized as a restricted-list member — \
         Stunning Fist is the automatic 1st-level grant, not a choice: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("Stunning Fist") && choice.detail.contains("automatic"),
        "the present-but-not-a-list-member branch must explain the automatic grant: {}",
        choice.detail
    );
}

/// **Updated (v0.6 alpha swarm, risks item 8, Monk remaining-feats
/// closure, 2026-07-25):** Combat Reflexes is no longer one of the
/// permanently-unproven restricted-list feats -- its extra-attack-of-
/// opportunity CAPACITY (a flat number derived purely from the Monk's
/// own Dexterity modifier) is now genuinely grounded, so
/// `class_feature.monk.bounded_progression.bonus_feat.unsupported` no
/// longer fires for this selection at all. This test previously asserted
/// the pre-closure behavior (that diagnostic firing and naming Combat
/// Reflexes); updated to assert the new behavior instead: the
/// choice-recognition record still names Combat Reflexes as before, and
/// the real capacity value is now grounded under its own dedicated
/// explanation id.
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

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"),
        "Combat Reflexes' own extra-AoO capacity is genuinely grounded now, so the narrowed \
         bonus-feat blocker must not fire for this selection: {:?}",
        computation.diagnostics
    );

    let capacity = explanation(
        &computation,
        "class_feature.monk.bounded_progression.bonus_feat.combat_reflexes_capacity",
    );
    // This fixture's Dexterity 16 (no Human bonus applied here -- the
    // fixture's own Human ability bonus targets Wisdom) -> +3 modifier ->
    // max(3, 0) = 3 additional attacks of opportunity.
    assert_eq!(
        capacity.value, 3,
        "expected the real, non-fabricated extra-AoO capacity value: {:?}",
        capacity
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
