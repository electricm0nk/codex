//! SD13-E2-R3 race + class interaction-pressure generalization proof.
//!
//! Proves the bounded race + class interaction-pressure model generalizes across
//! non-Human and non-Fighter combinations. Every chosen race + class combination —
//! the bounded Human Fighter levels 1-3 pilot path included — receives the same
//! shape of `interaction.bounded_posture` explanation record and a non-claim-blocking
//! `interaction.bounded` posture diagnostic naming which race-specific and
//! class-specific pressure surfaces are grounded versus unverified.
//!
//! The proof pins three concrete claims:
//! 1. The bounded Human Fighter level 1 input keeps its existing deterministic truth
//!    (Human race-extras such as `race.human.ability_bonus_target` /
//!    `race.human.bonus_feat_grant`, Fighter base-chassis explanation records, and the
//!    `interaction.bounded` diagnostic naming the grounded surfaces) plus the new
//!    generalized `interaction.bounded_posture` record.
//! 2. A Half-Orc Barbarian level 1 input — a non-Human and non-Fighter combination —
//!    is acknowledged on the same surface: the `interaction.bounded_posture` record
//!    is present with `value = 0` and names the Half-Orc + Barbarian:1 combination,
//!    the `interaction.bounded` diagnostic is present and non-claim-blocking and
//!    names only the unverified race-specific and class-specific pressure surfaces,
//!    and the existing non-Human race seam (`race.semantics.unverified`) and
//!    non-Fighter chassis seam (`class_chassis.unsupported`) keep firing exactly as
//!    they did before this slice.
//! 3. The Half-Orc Barbarian fixture parses without diagnostics so the generalized
//!    interaction-pressure model rests on a real, validated input record.
//!
//! It is intentionally not a Half-Orc race engine or a Barbarian class engine: it
//! grounds no Half-Orc racial trait (no ferocity, darkvision, intimidation racial
//! trait, or ability-bonus / bonus-feat interaction translation), and it grounds no
//! Barbarian class feature (rage, rage powers, uncanny dodge, trap sense, or any
//! other martial progression). It only proves the interaction-row model generalizes
//! the same shape of posture record / posture diagnostic beyond the bounded Human
//! Fighter pilot path.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, compute_pilot_base_chassis,
};

const HUMAN_FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");
const HALF_ORC_BARBARIAN_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_half_orc_barbarian_level1_sd13_deterministic_input.txt");

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
    computation: &'a codex::rules_core::pilot_compute::PilotBaseChassisComputation,
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

fn diagnostic_with_id<'a>(
    computation: &'a codex::rules_core::pilot_compute::PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationDiagnostic {
    computation
        .diagnostics
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            panic!(
                "expected diagnostic id '{id}', got {:?}",
                computation.diagnostics
            )
        })
}

fn has_explanation(
    computation: &codex::rules_core::pilot_compute::PilotBaseChassisComputation,
    id: &str,
) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ===== Fixture load proof =====

#[test]
fn half_orc_barbarian_fixture_loads_cleanly_as_chosen_input() {
    let input = load(HALF_ORC_BARBARIAN_FIXTURE);

    // Identity preserves the SD-13 canonical race + class ids verbatim.
    assert_eq!(
        input.case_id.as_deref(),
        Some("pf1-crb-half-orc-barbarian-level1")
    );
    assert_eq!(input.source_package_id, "pf1.core_rulebook");
    assert_eq!(input.chosen.race_id, "race:half-orc");
    assert_eq!(input.chosen.class_levels.len(), 1);
    assert_eq!(input.chosen.class_levels[0].class_id, "class:barbarian");
    assert_eq!(input.chosen.class_levels[0].level, 1);

    // No Human-specific selections and no Fighter-specific selections should leak
    // into the chosen record for a non-Human non-Fighter combination.
    let chosen_choices = input.chosen.selected_choices;
    for forbidden_choice in [
        "choice:human_ability_bonus",
        "choice:human_bonus_feat",
        "choice:fighter_bonus_feat",
        "choice:fighter_bonus_feat_2",
        "choice:level_1_character_feat",
    ] {
        assert!(
            !chosen_choices
                .iter()
                .any(|c| c.choice_set_id == forbidden_choice),
            "Half-Orc Barbarian fixture must not carry a {forbidden_choice} selection, \
             got {chosen_choices:?}"
        );
    }
}

// ===== Generalized interaction-pressure proof on Human Fighter (the bounded pilot) =====

#[test]
fn human_fighter_pilot_path_carries_the_generalized_interaction_posture_record() {
    let input = load(HUMAN_FIGHTER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The generalized posture record is present for Human Fighter just as it would be
    // for any other race + class combination.
    let posture = explanation(&computation, "interaction.bounded_posture");
    assert_eq!(
        posture.value, 0,
        "interaction.bounded_posture must carry value = 0 by design (posture declaration, \
         not a fabricated mechanical pressure contribution), got {}",
        posture.value
    );
    assert!(
        posture.detail.contains("race:human") && posture.detail.contains("class:fighter"),
        "interaction.bounded_posture detail must name the chosen race and class levels for \
         Human Fighter, got {}",
        posture.detail
    );
    assert!(
        posture.detail.contains("Human Fighter levels 1-3"),
        "interaction.bounded_posture detail must distinguish grounded vs unverified paths, \
         got {}",
        posture.detail
    );

    // The posture diagnostic is present, non-claim-blocking, and explicitly names the
    // grounded Human + Fighter surfaces.
    let diagnostic = diagnostic_with_id(&computation, "interaction.bounded");
    assert!(
        !diagnostic.claim_blocking,
        "interaction.bounded is the non-claim-blocking posture diagnostic, got {:?}",
        diagnostic
    );
    assert!(
        diagnostic.message.contains("Human Fighter levels 1-3"),
        "interaction.bounded for Human Fighter must name the bounded Human Fighter \
         pilot path, got {}",
        diagnostic.message
    );
    assert!(
        diagnostic.message.contains("choice:human_ability_bonus"),
        "interaction.bounded for Human Fighter must name the grounded Human \
         ability-bonus surface, got {}",
        diagnostic.message
    );
    assert!(
        diagnostic.message.contains("choice:human_bonus_feat"),
        "interaction.bounded for Human Fighter must name the grounded Human bonus-feat \
         surface, got {}",
        diagnostic.message
    );

    // Existing Human race-extras still surface verbatim. The generalized model does not
    // replace them.
    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "Human race ability-bonus target explanation record must remain present, got {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "Human race bonus-feat grant explanation record must remain present, got {:?}",
        computation.explanations
    );

    // The existing race-specific bounded diagnostic still fires.
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "race.human.bounded_semantics non-claim-blocking diagnostic must remain present"
    );
}

// ===== Generalized interaction-pressure proof on Half-Orc Barbarian (a non-pilot combination) =====

#[test]
fn half_orc_barbarian_path_carries_the_generalized_interaction_posture_record() {
    let input = load(HALF_ORC_BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The generalized posture record is present and explicit about the chosen
    // non-Human, non-Fighter combination.
    let posture = explanation(&computation, "interaction.bounded_posture");
    assert_eq!(
        posture.value, 0,
        "interaction.bounded_posture must carry value = 0 by design (posture declaration, \
         not a fabricated mechanical pressure contribution), got {}",
        posture.value
    );
    assert!(
        posture.detail.contains("race:half-orc"),
        "interaction.bounded_posture detail must name the chosen race:half-orc \
         identity, got {}",
        posture.detail
    );
    assert!(
        posture.detail.contains("class:barbarian"),
        "interaction.bounded_posture detail must name the chosen class:barbarian \
         identity, got {}",
        posture.detail
    );

    // The posture diagnostic is present, non-claim-blocking, and names only the
    // unverified race-specific and class-specific pressure surfaces.
    let diagnostic = diagnostic_with_id(&computation, "interaction.bounded");
    assert!(
        !diagnostic.claim_blocking,
        "interaction.bounded is the non-claim-blocking posture diagnostic, got {:?}",
        diagnostic
    );
    assert!(
        diagnostic.message.contains("race:half-orc")
            && diagnostic.message.contains("class:barbarian"),
        "interaction.bounded for Half-Orc Barbarian must name the chosen combination, \
         got {}",
        diagnostic.message
    );
    assert!(
        diagnostic.message.contains("unverified"),
        "interaction.bounded for a non-pilot combination must explicitly mark the race \
         and class pressure surfaces as unverified, got {}",
        diagnostic.message
    );
}

#[test]
fn half_orc_barbarian_path_keeps_the_ungrounded_race_and_chassis_posture_honest() {
    let input = load(HALF_ORC_BARBARIAN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The non-Human race seam still fires a `race.semantics.unverified` non-claim-blocking
    // diagnostic on top of the new generalized posture diagnostic.
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| { d.id == "race.semantics.unverified" && !d.claim_blocking }),
        "the existing race.semantics.unverified non-claim-blocking diagnostic must still \
         fire for race:half-orc, got {:?}",
        computation.diagnostics
    );

    // The non-Fighter chassis seam keeps its claim-blocking diagnostic because the
    // generalized interaction model does not lift Barbarian into the bounded fighter-shaped
    // chassis seam; that lifting belongs to its own slice.
    let unsupported = diagnostic_with_id(&computation, "class_chassis.unsupported");
    assert!(
        unsupported.claim_blocking,
        "class_chassis.unsupported must remain claim-blocking for Barbarian, got {:?}",
        unsupported
    );

    // The posture diagnostic does not get promoted to claim-blocking just because the
    // pressure was declared: it is explicitly non-claim-blocking so the deterministic
    // pilot still reports computed evidence at the post-blocker posture level.
    let posture_diag = diagnostic_with_id(&computation, "interaction.bounded");
    assert!(
        !posture_diag.claim_blocking,
        "interaction.bounded must remain non-claim-blocking even on a non-pilot \
         combination, got {:?}",
        posture_diag
    );

    // No Human race-extras surface on the non-Human path: the generalized posture
    // record is the only race-related non-claim-blocking explanation for this input.
    assert!(
        !has_explanation(&computation, "race.human.ability_bonus_target"),
        "Human ability-bonus target must not be fabricated for a non-Human input, got {:?}",
        computation.explanations
    );
    assert!(
        !has_explanation(&computation, "race.human.bonus_feat_grant"),
        "Human bonus-feat grant must not be fabricated for a non-Human input, got {:?}",
        computation.explanations
    );
}

// ===== Diagnostic shape invariant for the bounded pilot path itself =====

#[test]
fn generalized_interaction_pressure_runs_for_every_chosen_combination() {
    // A short invariant-style proof: the bounded posture record and posture diagnostic
    // must be present for both the bounded grounded path and the unverified
    // generalization, with the same ids and the same claim_blocking posture. This is
    // the audit shape the SD-13 matrix row rests on.
    let human_fighter = load(HUMAN_FIGHTER_FIXTURE);
    let half_orc_barbarian = load(HALF_ORC_BARBARIAN_FIXTURE);

    for (label, input) in [
        ("Human Fighter level 1", &human_fighter),
        ("Half-Orc Barbarian level 1", &half_orc_barbarian),
    ] {
        let computation = compute_pilot_base_chassis(input);

        let posture = explanation(&computation, "interaction.bounded_posture");
        assert_eq!(
            posture.value, 0,
            "{label}: interaction.bounded_posture must carry value = 0, got {}",
            posture.value
        );

        let diagnostic = diagnostic_with_id(&computation, "interaction.bounded");
        assert!(
            !diagnostic.claim_blocking,
            "{label}: interaction.bounded must remain non-claim-blocking, got {:?}",
            diagnostic
        );
    }
}
