//! SD13-E3-F6 + SD13-E4-F8 Ranger closeout: hybrid chassis baseline +
//! hybrid spell burden classification slice for the deterministic Human Ranger.
//!
//! Closes out the Ranger row in the SD-13 support-state matrix:
//! `class.ranger.hybrid_chassis_and_spell_burden`. At the time this slice landed,
//! the seed placed the row at `Blocked / Computed / RefreshableFromLiveProof`
//! because the SD13-E3-F6 surface proves the deterministic Human Ranger level-1
//! hybrid chassis identity is recognized on the compute seam. (A later SD13-E3
//! Ranger decomposition slice,
//! `sd13_ranger_level1_chassis_and_class_feature_separation.rs`, grounds Track
//! for real and promotes the row to `Partial`; see that file for the current
//! posture.) This closeout slice pins three properties as a Ranger-only typed
//! check, independently of the Paladin slice that shares the same compute
//! surface:
//!
//! 1. The chassis-baseline recognition (the
//!    `class_chassis.hybrid_baseline.ranger` explanation) survives for a
//!    deterministic Human level-1 Ranger, named distinctly from the Paladin
//!    recognition record.
//! 2. The later hybrid spell burden (`class_spell.hybrid.ranger.unsupported`) is
//!    claim-blocking and explicitly names the partial-caster pressure
//!    (spell slots, spell source, and spells known/prepared posture), distinct
//!    from the non-spell class-feature burden
//!    (`class_feature.hybrid.ranger.unsupported`).
//! 3. A level-2..10 Ranger input — synthesized by replacing the class level in
//!    the bounded level-1 fixture — stays claim-blocked, never gains the
//!    level-1 hybrid recognition record, never fabricates a partial-caster
//!    spell posture, and never produces a computed receipt. This is the explicit
//!    partial-caster pressure preservation the slice is required to prove.
//!
//! The slice is intentionally not a Ranger class engine: it grounds no
//! favored-enemy, no combat-style, no tracking, no animal companion, no
//! favored terrain breadth, no Ranger spell-slot / spells-known math, no
//! Ranger level 2+, and no multiclass. The Paladin chassis-baseline posture
//! (`class_chassis.hybrid_baseline.paladin` recognition) is asserted
//! non-regressed so that the closeout of Ranger does not silently move the
//! Paladin row.

use codex::rules_core::character_input::{load_character_input_fixture, CharacterInput};
use codex::rules_core::pilot_compute::{
    build_pilot_headless_receipt, compute_pilot_base_chassis, ComputationDiagnostic,
    ComputationExplanation, HeadlessReceiptStatus, PilotBaseChassisComputation,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    seeded_sd13_e1_f1_current_truth, EvidenceFreshness, EvidenceTier, SupportState,
};

const RANGER_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const PALADIN_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");

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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

// ----- (1) Direct runtime evidence: Ranger chassis-baseline recognition -----

#[test]
fn ranger_level1_chassis_baseline_recognition_is_ranger_specific() {
    let input = load(RANGER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Ranger chassis identity is recognized
    // on the compute path and the recognition record names the Ranger specifically.
    let chassis = explanation(&computation, "class_chassis.hybrid_baseline.ranger");
    assert!(
        chassis.detail.contains("class:ranger") && chassis.detail.contains("level 1"),
        "ranger chassis-baseline recognition must name the class:ranger:1 identity: {}",
        chassis.detail
    );

    // The recognition record must not be misattributed to Paladin by this slice.
    assert!(
        !has_explanation(&computation, "class_chassis.hybrid_baseline.paladin"),
        "Ranger input must not surface the Paladin chassis-baseline recognition record"
    );

    // Recognition only; no fabricated mechanical value.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "hybrid baseline must not fabricate a base attack bonus"
    );
}

// ----- (2) Later hybrid spell burden is pinned independent of the class-feature burden -----

#[test]
fn ranger_level1_spell_burden_is_pinned_independently() {
    let input = load(RANGER_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The later hybrid spell burden must be claim-blocking and explicitly named for
    // Ranger (not Paladin). It must name the partial-caster pressure so a later slice
    // cannot silently fabricate a Ranger spell posture.
    let spell = claim_blocking(&computation, "class_spell.hybrid.ranger.unsupported");
    for token in ["ranger", "spell slots", "spell source", "known/prepared"] {
        assert!(
            spell.message.to_lowercase().contains(token),
            "ranger spell-burden diagnostic must name the '{token}' partial-caster posture: {}",
            spell.message
        );
    }

    // The non-spell class-feature burden must remain its own claim-blocking
    // diagnostic, separate from the spell burden.
    let feature = claim_blocking(&computation, "class_feature.hybrid.ranger.unsupported");
    for token in ["favored enemy", "combat style", "tracking"] {
        assert!(
            feature.message.contains(token),
            "ranger class-feature diagnostic must name the '{token}' non-spell burden: {}",
            feature.message
        );
    }

    // The two Ranger-burden diagnostics must be emitted as distinct records so the
    // spell burden can be lifted independently in a future slice.
    assert!(
        spell.id != feature.id,
        "ranger spell-burden diagnostic must be a distinct id from the non-spell \
         class-feature burden (got both at id '{}')",
        spell.id
    );
    assert!(
        !spell.id.contains("paladin") && !feature.id.contains("paladin"),
        "ranger burden diagnostics must not be misattributed to paladin (spell='{}', feature='{}')",
        spell.id,
        feature.id
    );

    // No fabricated spell posture.
    assert!(
        !has_explanation(&computation, "class_spell.spontaneous.ranger")
            && !has_explanation(&computation, "class_spell.prepared.ranger")
            && !has_explanation(&computation, "class_spell.slots.ranger"),
        "ranger hybrid baseline must not emit a spell-posture explanation record"
    );

    // The integrated posture stays blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked ranger hybrid baseline must not emit a computed snapshot"
    );
}

// ----- (3) Ranger level 2..10 stay claim-blocked: partial-caster pressure preserved -----

#[test]
fn ranger_levels_2_through_10_do_not_gain_chassis_or_spell_posture() {
    // The bounded level-1 chassis-baseline + spell-burden slice deliberately does
    // not promote Ranger past level 1. A level-2..10 Ranger must not gain the
    // level-1 hybrid recognition record and must not fabricate any partial-caster
    // spell posture. The Ranger partial-caster pressure (1 first-level slot at
    // level 2, growing through level 10) is named by the existing
    // `class_spell.hybrid.ranger.unsupported` diagnostic and never computed by
    // this slice.
    for level in 2u8..=10 {
        let fixture =
            RANGER_LEVEL1_FIXTURE.replace("class:ranger:1", &format!("class:ranger:{level}"));
        let input = load(&fixture);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            !has_explanation(&computation, "class_chassis.hybrid_baseline.ranger"),
            "level-{level} Ranger must not gain the bounded level-1 hybrid \
             chassis-baseline recognition record"
        );
        // No Paladin leakage either.
        assert!(
            !has_explanation(&computation, "class_chassis.hybrid_baseline.paladin"),
            "level-{level} Ranger must not surface the Paladin chassis-baseline record"
        );
        // No fabricated partial-caster spell posture.
        assert!(
            !has_explanation(&computation, "class_spell.prepared.ranger")
                && !has_explanation(&computation, "class_spell.spontaneous.ranger")
                && !has_explanation(&computation, "class_spell.slots.ranger")
                && !has_explanation(&computation, "class_spell.partial_caster_slots.ranger"),
            "level-{level} Ranger must not fabricate any partial-caster spell posture \
             explanations: {:?}",
            computation.explanations
        );

        // Stays claim-blocked — either via `class_chassis.unsupported` (the
        // bounded Fighter-shaped compute path blocker for any non-Fighter /
        // level-4+ Fighter) or via the explicit hybrid burden diagnostics
        // that `hybrid_level1_class` does not emit at level > 1.
        let receipt = build_pilot_headless_receipt(&input);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "level-{level} Ranger must stay claim-blocked on the bounded slice"
        );
        let view_model = PilotViewModel::from_receipt(&receipt);
        assert!(
            view_model.snapshot.is_none(),
            "level-{level} Ranger must not emit a computed snapshot"
        );
        assert!(
            computation.diagnostics.iter().any(|d| d.claim_blocking),
            "level-{level} Ranger must carry at least one claim-blocking diagnostic"
        );

        // If a level-2..10 Ranger is unexpectedly reached by a future relaxed
        // `hybrid_level1_class` matcher, the spell-burden diagnostic must still
        // name the partial-caster pressure so the burden is named even when the
        // blocker surface shifts. We tolerate EITHER: the upstream
        // `class_chassis.unsupported` blocker, OR the hybrid spell-burden blocker.
        // We do NOT tolerate both being absent (that would be silent promotion).
        let upper_blocker = has_diagnostic(&computation, "class_chassis.unsupported");
        let hybrid_spell_blocker =
            has_diagnostic(&computation, "class_spell.hybrid.ranger.unsupported");
        assert!(
            upper_blocker || hybrid_spell_blocker,
            "level-{level} Ranger must be blocked by at least one of \
             `class_chassis.unsupported` or `class_spell.hybrid.ranger.unsupported`"
        );
    }
}

// ----- Paladin chassis-baseline posture is preserved by the Ranger closeout -----

#[test]
fn paladin_chassis_baseline_is_not_regressed_by_ranger_closeout() {
    let input = load(PALADIN_LEVEL1_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "class_chassis.hybrid_baseline.paladin"),
        "Paladin chassis-baseline recognition must remain on a deterministic \
         Human level-1 Paladin input after the Ranger closeout: {:?}",
        computation.explanations
    );
    assert!(
        has_diagnostic(&computation, "class_spell.hybrid.paladin.unsupported"),
        "Paladin spell-burden diagnostic must remain claim-blocking after the \
         Ranger closeout: {:?}",
        computation.diagnostics
    );
    // Ranger-specific diagnostics must not leak into a Paladin input.
    assert!(
        !has_explanation(&computation, "class_chassis.hybrid_baseline.ranger"),
        "Paladin input must not surface the Ranger chassis-baseline recognition record"
    );
    assert!(
        !has_diagnostic(&computation, "class_spell.hybrid.ranger.unsupported"),
        "Paladin input must not surface the Ranger spell-burden diagnostic"
    );
}

// ----- Matrix control plane: the Ranger row stays Blocked/Computed/Refreshable -----

#[test]
fn matrix_ranger_row_is_partial_computed_and_names_remaining_burdens() {
    // The later SD13-E3 Ranger decomposition slice
    // (sd13_ranger_level1_chassis_and_class_feature_separation.rs) grounded
    // Track for real and intentionally promoted this row from Blocked to
    // Partial; favored enemy, combat style, and the later spell burden remain
    // named and unimplemented.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist in the SD-13 support-state matrix");

    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row is Partial: Track is grounded for real, but favored enemy, \
         combat style, and the later spell burden remain named and unimplemented"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        ranger
            .grounding_ref
            .contains("sd13_hybrid_level1_chassis_baseline"),
        "ranger row must continue to cite the SD13-F6 hybrid proof surface: {}",
        ranger.grounding_ref
    );

    let note = ranger.blocker_or_lossiness_note;
    assert!(
        !note.is_empty(),
        "ranger partial row must carry an explicit non-empty note"
    );
    // "SD13-E4" was the original deferral token; the note now names the spell
    // burden as unproven BEYOND the grounded partial-caster identity pair.
    for token in ["favored enemy", "combat style", "partial_caster.spell_level_access"] {
        assert!(
            note.contains(token),
            "ranger partial note must name the '{token}' burden: {note}"
        );
    }

    // The next-required-uplift must point at the remaining favored-enemy /
    // combat-style grounding and then the SD13-E4 spell burden.
    assert!(
        ranger.next_required_uplift.contains("SD13-E4"),
        "ranger next-required-uplift must still name SD13-E4 (spell burden): {}",
        ranger.next_required_uplift
    );
}

#[test]
fn matrix_ranger_row_is_not_misattributed_to_paladin_or_supported() {
    // The Ranger promotion must not silently move the Paladin row or fold the
    // Ranger row into another hybrid chassis, and must not overshoot to
    // Supported (favored enemy and combat style are still unproven).
    let matrix = seeded_sd13_e1_f1_current_truth();
    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences) — a Paladin-owned move, not a Ranger side
    // effect; the misattribution guard below still holds.
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must still exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Partial,
        "paladin row must carry its own-slice Partial posture, never a Ranger-driven move"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);

    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must still exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row is intentionally Partial after the SD13-E3 Ranger \
         decomposition slice grounds Track for real"
    );
    assert_ne!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must not be silently promoted to Supported — favored enemy, \
         combat style, and the later spell burden remain explicitly unresolved"
    );
    assert!(
        ranger.row_id.contains("ranger"),
        "ranger row_id must be ranger-identified: {}",
        ranger.row_id
    );
    assert!(
        !ranger.blocker_or_lossiness_note.contains("paladin"),
        "ranger blocker note must not be a Paladin paraphrase: {}",
        ranger.blocker_or_lossiness_note
    );
}
