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
//!    the bounded level-1 fixture — stays claim-blocked and never gains the
//!    level-1 hybrid recognition record.
//!
//! The slice is intentionally not a Ranger class engine: it grounds no
//! favored-enemy, no combat-style, no tracking, no animal companion, no
//! favored terrain breadth, no Ranger spell-slot / spells-known math, no
//! Ranger level 2+, and no multiclass. The Paladin chassis-baseline posture
//! (`class_chassis.hybrid_baseline.paladin` recognition) is asserted
//! non-regressed so that the closeout of Ranger does not silently move the
//! Paladin row.
//!
//! **Superseded twice over (v0.6 alpha swarm, risks item 8):** `table_class_id`
//! was widened to recognize Ranger (first slice), then Ranger's own real
//! prepared-spell posture was grounded (`class_spell.ranger.partial_caster.unsupported`
//! now fires only on a genuine unmet condition -- an off-list spell, a spell
//! level beyond the ranger's own access ceiling, or a per-level slot-budget
//! overrun -- not unconditionally; an empty prepared-spell list is honestly
//! valid). Point 3 above is now stale in one specific way worth stating
//! plainly: the level-2..10 Ranger input this file constructs (by
//! text-substituting the class level in `RANGER_LEVEL1_FIXTURE`) stays
//! `Blocked` at every level 2..10 for a *different* reason than originally
//! documented -- `combat.baseline_unsupported` /
//! `skill.selected_modifier.unsupported` (the same exact-posture gates that
//! block any class whose selected feats/equipment/skills don't match the
//! deterministic GE-06 combat/skill posture), not
//! `class_spell.ranger.partial_caster.unsupported`, which correctly never
//! fires here since this fixture carries no `spells_selected` at all (an
//! honestly valid, empty posture). `ranger_levels_2_through_10_stay_blocked_on_the_bounded_fixture`
//! below states this precisely rather than misattributing the block to
//! Ranger's own spell gate. The two tests after it
//! (`ranger_levels_2_through_10_reach_computed_with_a_valid_spell_posture` /
//! `..._stay_blocked_with_a_genuine_spell_posture_violation`) independently
//! exercise Ranger's own spell-posture gate directly, using the same
//! Fighter-fixture-plus-swapped-class-levels construction
//! `pilot_compute.rs`'s own `ranger_dispatch_widening_safety_tests` module
//! uses, since this file's own fixture can't reach that gate at all (it
//! fails the unrelated combat/skill posture first).

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

    // (v0.6 swarm update, risks item 8) No longer recognition-only: `table_class_id`
    // was widened to recognize Ranger via the shared table-driven
    // `compute_generic_table_chassis` dispatch, so this is now a real, non-fabricated
    // base attack bonus (Ranger's full-BAB progression, 1 at level 1). Mirrors the
    // identical Rogue-widening flip in `sd13_rogue_level1_chassis_baseline.rs`.
    assert_eq!(
        computation.base_attack_bonus, 1,
        "ranger level 1's real full-BAB progression (classlevel) is now genuinely integrated"
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

    // The non-spell class-feature burden diagnostic
    // (`class_feature.hybrid.ranger.unsupported`) is retired: it flatly claimed
    // favored enemy / combat style / tracking were unimplemented, which the
    // per-class decomposition dispatched on this same input contradicts by
    // grounding Track and the Favored Enemy flat surface for real. See
    // `tests/hybrid_diagnostic_grounded_contradiction.rs`.
    assert!(
        !has_diagnostic(&computation, "class_feature.hybrid.ranger.unsupported"),
        "the retired non-spell class-feature blocker must not reappear: {:?}",
        computation.diagnostics
    );
    assert!(
        !spell.id.contains("paladin"),
        "ranger spell-burden diagnostic must not be misattributed to paladin (spell='{}')",
        spell.id
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

// ----- (3) Ranger level 2..10 on THIS file's bounded fixture: still blocked, but honestly for an unrelated reason -----

#[test]
fn ranger_levels_2_through_10_stay_blocked_on_the_bounded_fixture() {
    // This file's own level-2..10 input (text-substituting the class level in
    // `RANGER_LEVEL1_FIXTURE`) never gains the level-1-only hybrid recognition
    // record, and stays claim-blocked at every level -- but (v0.6 swarm update,
    // see this file's own module doc comment) NOT because of Ranger's own spell
    // posture: this fixture carries no `spells_selected` at all, so
    // `class_spell.ranger.partial_caster.unsupported` correctly never fires
    // (an empty posture is honestly valid). The real reason it stays blocked
    // is the same deterministic-posture gate that blocks any class whose
    // selected feats/equipment/skills don't match the exact GE-06 combat/skill
    // posture -- `combat.baseline_unsupported` and
    // `skill.selected_modifier.unsupported` -- which this Ranger-specific
    // fixture was never built to satisfy. Ranger's own spell-posture gate is
    // exercised directly (with a fixture that DOES satisfy that unrelated
    // posture) by the two tests immediately below.
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
        // (Note: `class_spell.ranger.daily_preparation` DOES legitimately appear
        // here even on this bounded fixture -- an empty prepared-spell list is a
        // real, honest "0 spells, valid posture" state, not a fabrication, and
        // `ground_ranger_prepared_spells` correctly runs and records it. Not
        // asserted absent here; that would be asserting a false claim.)

        let receipt = build_pilot_headless_receipt(&input);
        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "level-{level} Ranger on this bounded fixture must stay claim-blocked: {:?}",
            receipt.computation.diagnostics
        );
        let view_model = PilotViewModel::from_receipt(&receipt);
        assert!(
            view_model.snapshot.is_none(),
            "level-{level} Ranger must not emit a computed snapshot"
        );

        // The real reason: the unrelated combat/skill exact-posture gates, not
        // Ranger's own (correctly silent, since the posture is empty and
        // therefore valid) spell-posture diagnostic.
        assert!(
            has_diagnostic(&computation, "combat.baseline_unsupported")
                && has_diagnostic(&computation, "skill.selected_modifier.unsupported"),
            "level-{level} Ranger on this bounded fixture must be blocked by the unrelated \
             combat/skill exact-posture gates: {:?}",
            computation.diagnostics
        );
        assert!(
            !has_diagnostic(&computation, "class_spell.ranger.partial_caster.unsupported"),
            "level-{level} Ranger's own spell-posture diagnostic must NOT fire here -- an \
             empty prepared-spell list is honestly valid, and this fixture's block comes \
             entirely from the unrelated combat/skill posture gates: {:?}",
            computation.diagnostics
        );
    }
}

// ----- (3b) Ranger's OWN spell-posture gate, exercised directly (this file's bounded fixture can't reach it) -----

/// Builds a real, combat/skill-posture-satisfying input for a single-class
/// Ranger at `level` by starting from the Fighter fixture (already
/// deterministic-posture-satisfying) and swapping only the class identity --
/// the same construction `pilot_compute.rs`'s own
/// `ranger_dispatch_widening_safety_tests` module uses, so this file can
/// independently re-verify the same claim through the shared
/// `tests/**`-owned fixture-loading path rather than duplicating the exact
/// in-crate test.
fn ranger_at_level_with_satisfied_combat_posture(level: u8) -> CharacterInput {
    let fighter_fixture = include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    let mut input = load(fighter_fixture);
    input.chosen.class_levels = vec![codex::rules_core::character_input::CharacterClassLevel {
        class_id: "class:ranger".to_owned(),
        level,
    }];
    input
}

#[test]
fn ranger_levels_2_through_10_reach_computed_with_a_valid_spell_posture() {
    for level in 2u8..=10 {
        let input = ranger_at_level_with_satisfied_combat_posture(level);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "level-{level} Ranger with a valid (empty) spell posture and a satisfied \
             combat/skill posture must reach Computed: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported"),
            "level-{level} Ranger's spell-posture diagnostic must not fire on a valid \
             (empty) posture: {:?}",
            receipt.computation.diagnostics
        );
    }
}

#[test]
fn ranger_levels_2_through_10_stay_blocked_with_a_genuine_spell_posture_violation() {
    // "Magic Missile" is never on the real PF1 ranger spell list
    // (`rules_tables::crb::ranger_spell_list`, all ingested books) at any
    // level, so it is a
    // genuine off-list violation uniformly across the whole 2..10 range --
    // unlike a specific spell level, whose accessibility shifts as the
    // ranger's own access ceiling grows through this range.
    for level in 2u8..=10 {
        let mut input = ranger_at_level_with_satisfied_combat_posture(level);
        input.chosen.spells_selected.push(codex::rules_core::character_input::SpellSelection {
            spell_id: "Magic Missile".to_owned(),
            source_class_id: "class:ranger".to_owned(),
            acquisition_mode: codex::rules_core::character_input::AcquisitionMode::Prepared,
        });

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Blocked,
            "level-{level} Ranger preparing an off-list spell must stay Blocked: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_spell.ranger.partial_caster.unsupported"
                    && d.claim_blocking),
            "level-{level} Ranger must carry the real spell-posture diagnostic: {:?}",
            receipt.computation.diagnostics
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
    // named and unimplemented. Later still, promoted to Supported/ProductVisible
    // by SD-19's Class Progression Catalog browser UI-surfacing work
    // (2026-07-17) — condition 2 (every named grounded milestone) was already
    // satisfied, so only the UI surface was missing.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist in the SD-13 support-state matrix");

    assert_eq!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
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
fn matrix_ranger_row_is_not_misattributed_to_paladin() {
    // The Ranger promotion must not silently move the Paladin row or fold the
    // Ranger row into another hybrid chassis. Ranger's own promotion to
    // Supported/ProductVisible (SD-19 Class Progression Catalog browser
    // UI-surfacing work, 2026-07-17) is its own intentional, named move, not
    // a Paladin side effect.
    let matrix = seeded_sd13_e1_f1_current_truth();
    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences), then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17) —
    // both Paladin-owned moves, not a Ranger side effect; the
    // misattribution guard below still holds.
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must still exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Supported,
        "paladin row must carry its own-slice Supported posture, never a Ranger-driven move"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);

    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must still exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must carry its own-slice Supported posture, never a Paladin-driven move"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
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
