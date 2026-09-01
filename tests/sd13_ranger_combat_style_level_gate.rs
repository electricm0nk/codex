//! SD13-E5 Ranger Combat Style level-gate correction slice.
//!
//! This cycle was originally scoped to ground the Ranger's level-1
//! archery-vs-two-weapon-combat style CHOICE as a bounded +0 recognition record,
//! mirroring the Favored Enemy choice-recognition idiom. Sanity-checking that
//! premise against the PF1 Core Rulebook Ranger class table (verified against the
//! d20pfsrd Ranger class reference) showed the premise itself was wrong: PF1 grants
//! Combat Style Feat -- the archery-vs-two-weapon-combat style choice AND its first
//! bonus feat -- TOGETHER at 2nd level, not split across a level-1 choice and a
//! level-2 feat grant. There is no separate level-1 combat-style choice to
//! recognize -- mirroring how a prior cycle discovered the Paladin mercy blocker
//! wrongly cited "level-6" instead of "level-3".
//!
//! Given that correction, and given that Ranger level-2+ progression is out of
//! scope for this cycle (the row's chassis seam, `is_single_class_ranger_level1`,
//! stays gated to level 1 only), the honest grounding available this cycle is a
//! level-gate ABSENCE record (value 0) for combat style at level 1 -- mirroring the
//! Paladin mercy level-gate idiom (`class_chassis.paladin.level_gate.mercy`), not a
//! choice-recognition record. This retires the
//! `class_feature.ranger.combat_style.unsupported` claim-blocking diagnostic, which
//! itself carried the mistaken level-1/level-2 split framing.
//!
//! This proof file pins the corrected framing as its own dedicated test surface.
//! The broader Ranger per-pillar decomposition file
//! (`sd13_ranger_level1_chassis_and_class_feature_separation.rs`) also exercises
//! the same production record as part of its full per-pillar sweep; both files
//! must stay green together.

use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::support_state_matrix::{seeded_current_truth, SupportState};
mod common;
use common::{load, explanation};

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const RANGER_COMBAT_STYLE_LEVEL_GATE_ID: &str = "class_chassis.ranger.level_gate.combat_style";
const RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID: &str = "class_feature.ranger.combat_style.unsupported";

#[test]
fn ranger_level1_combat_style_is_grounded_as_a_correct_level_gate_absence_not_a_choice() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The retired claim-blocking diagnostic (mistaken level-1/level-2 split
    // framing) must be gone.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == RANGER_COMBAT_STYLE_RETIRED_BLOCKER_ID),
        "the retired combat-style blocker must no longer be emitted: {:?}",
        computation.diagnostics
    );

    // No level-1 choice-recognition record was fabricated: PF1 Core Rulebook
    // grants neither the style choice nor a bonus feat at level 1, so there is
    // nothing to recognize at this bounded baseline.
    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_chassis.ranger.combat_style_choice"),
        "no level-1 combat-style CHOICE record may be fabricated -- PF1 grants the style choice \
         together with its first bonus feat at 2nd level, not at 1st: {:?}",
        computation.explanations
    );

    // The correct grounding is a level-gate ABSENCE record (value 0), mirroring
    // the Paladin mercy idiom.
    let combat_style = explanation(&computation, RANGER_COMBAT_STYLE_LEVEL_GATE_ID);
    assert_eq!(
        combat_style.value, 0,
        "combat-style level-gate absence must carry no fabricated mechanical value, got {}",
        combat_style.value
    );
    assert!(
        combat_style.detail.contains("combat style") || combat_style.detail.contains("Combat Style"),
        "combat-style level-gate record must name the combat-style feature: {}",
        combat_style.detail
    );
    assert!(
        combat_style.detail.contains("2nd level"),
        "combat-style level-gate record must name the 2nd-level PF1 CRB milestone: {}",
        combat_style.detail
    );
    assert!(
        combat_style.detail.contains("correctly absent"),
        "combat-style level-gate record must state the correct level-1 absence: {}",
        combat_style.detail
    );
    assert!(
        !combat_style
            .detail
            .contains("is a level-1 decision, but the bonus feat"),
        "combat-style level-gate record must not resurrect the retired mistaken framing: {}",
        combat_style.detail
    );
}

#[test]
fn matrix_ranger_row_note_no_longer_claims_a_level1_combat_style_choice() {
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");

    // Later promoted to Supported/ProductVisible by SD-19's Class Progression
    // Catalog browser UI-surfacing work (2026-07-17).
    assert_eq!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must be Supported after the SD-19 class-row promotion"
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        !note.contains("the level-1 style choice and its level-2 bonus-feat grant"),
        "ranger row note must not resurrect the retired mistaken level-1/level-2 split framing: \
         {note}"
    );
    assert!(
        note.contains("2nd level") || note.contains("level-2"),
        "ranger row note must name the corrected 2nd-level combat-style milestone: {note}"
    );
    assert!(
        note.contains("combat style"),
        "ranger row note must still name the combat-style pillar: {note}"
    );
}
