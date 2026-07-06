//! SD13-E3-F6 Paladin and Ranger level-1 hybrid chassis baseline proof, with the
//! bounded ranger class-feature uplift proven by the SD13-E3 ranger class-feature
//! slice (this card).
//!
//! Proves the truthful SD13-F6 slice plus the bounded ranger class-feature uplift:
//! the live rules-core surface ingests deterministic Human `class:paladin:1` and
//! `class:ranger:1` inputs, leaves direct computed evidence that acknowledges each
//! hybrid level-1 chassis identity rather than treating it as an undocumented packet
//! placeholder. For the ranger slice, the bounded level-1 non-spell class-feature
//! burden (favored enemy, combat style, tracking) is surfaced as recognized
//! non-claim-blocking explanation records naming each feature and its bounded
//! level-1 status, lifting the matrix row from `Blocked` to `Partial` while keeping
//! the later hybrid spell burden as the only claim-blocking diagnostic. The Paladin
//! row still stays `Blocked` / `Computed` because its non-spell class-feature
//! burden (smite, lay on hands, divine grace, mercy) is intentionally left for the
//! Paladin class-feature slice to lift.
//!
//! It is intentionally not a hybrid class engine. It grounds no Paladin/Ranger level
//! 2+, no smite / lay-on-hands / divine-grace / mercy execution, no combat-style
//! execution at level 2+, no favored-terrain breadth, no animal companion, no
//! general spell engine, and no spell-slot / known-or-prepared posture. It also
//! preserves the accepted Fighter 1-3 truth, the Rogue blocked negative control,
//! and the Human race/interaction truth.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, HeadlessReceiptStatus,
    PilotBaseChassisComputation, build_pilot_headless_receipt, compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const PALADIN_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt"
);
const RANGER_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt"
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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Direct runtime evidence: the hybrid chassis identity is acknowledged -----

#[test]
fn paladin_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Paladin chassis identity is recognized on
    // the compute path, not silently dropped as an undocumented packet placeholder.
    let chassis = explanation(&computation, "class_chassis.hybrid_baseline.paladin");
    assert!(
        chassis.detail.contains("class:paladin") && chassis.detail.contains("level 1"),
        "paladin chassis recognition must name the class:paladin:1 identity: {}",
        chassis.detail
    );
    // It is recognition only; it must not fabricate a Fighter-style computed chassis.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "hybrid baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "hybrid baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (CHA 14 -> +2).
    assert_eq!(computation.ability_modifiers.charisma, 2);
}

#[test]
fn ranger_level1_leaves_direct_chassis_recognition_evidence() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let chassis = explanation(&computation, "class_chassis.hybrid_baseline.ranger");
    assert!(
        chassis.detail.contains("class:ranger") && chassis.detail.contains("level 1"),
        "ranger chassis recognition must name the class:ranger:1 identity: {}",
        chassis.detail
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "hybrid baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "hybrid baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (STR 16 -> +3).
    assert_eq!(computation.ability_modifiers.strength, 3);
}

// ----- Still blocked: honest, class-specific burden diagnostics -----

#[test]
fn paladin_level1_stays_blocked_naming_class_feature_and_spell_burden() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The non-spell class-feature burden must be named explicitly, not hidden behind a
    // generic "unsupported hybrid" label.
    let feature = claim_blocking(&computation, "class_feature.hybrid.paladin.unsupported");
    for token in ["smite", "lay on hands", "divine grace", "mercy"] {
        assert!(
            feature.message.contains(token),
            "paladin feature blocker must name the '{token}' burden: {}",
            feature.message
        );
    }

    // The later spell burden must be named explicitly and stay claim-blocking.
    let spell = claim_blocking(&computation, "class_spell.hybrid.paladin.unsupported");
    assert!(
        spell.message.contains("spell"),
        "paladin spell blocker must name the later spell burden: {}",
        spell.message
    );

    // The integrated posture is blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked hybrid baseline must not emit a computed snapshot"
    );
}

// ----- Ranger class-feature uplift (this slice): non-spell burden lifted, spell burden stays blocked -----

#[test]
fn ranger_level1_surfaces_favored_enemy_combat_style_and_tracking_features() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Favored enemy is a level-1 Ranger feature: the named selection is surfaced as
    // an explicit, non-claim-blocking class-feature seam (recognition + bounded
    // mechanical contribution), not hidden behind a generic unsupported-hybrid label.
    let favored = explanation(&computation, "class_feature.ranger.favored_enemy");
    assert!(
        favored.detail.contains("favored enemy") && favored.detail.contains("level 1"),
        "ranger favored-enemy seam must name the favored enemy feature at level 1: {}",
        favored.detail
    );
    assert_eq!(
        favored.value, 0,
        "ranger favored-enemy seam must surface the bounded selection without inventing type-specific bonuses"
    );

    // Combat style is a level-2 PF1 feature; at level 1 the ranger has the named
    // selection slot only, no computed bonus. The seam names the level-1 status
    // honestly and contributes no fabricated mechanical value.
    let style = explanation(&computation, "class_feature.ranger.combat_style");
    assert!(
        style.detail.contains("combat style"),
        "ranger combat-style seam must name the combat style feature: {}",
        style.detail
    );
    assert!(
        style.detail.contains("level 2") || style.detail.contains("level 2+"),
        "ranger combat-style seam must honestly name the level-2+ activation: {}",
        style.detail
    );
    assert_eq!(
        style.value, 0,
        "ranger combat-style seam at level 1 must contribute no fabricated bonus"
    );

    // Tracking is the Track bonus feat granted at level 1; surface it as an
    // explicit, non-claim-blocking recognition seam rather than a hidden block.
    let tracking = explanation(&computation, "class_feature.ranger.tracking");
    assert!(
        tracking.detail.contains("tracking") || tracking.detail.contains("track"),
        "ranger tracking seam must name the tracking bonus feat: {}",
        tracking.detail
    );
    assert_eq!(
        tracking.value, 0,
        "ranger tracking seam must surface the bonus feat without inventing a feat-effect engine"
    );
}

#[test]
fn ranger_level1_drops_non_spell_class_feature_blocker_keeps_spell_blocker() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The non-spell class-feature burden is now lifted off the claim-blocking path:
    // no diagnostic with id `class_feature.hybrid.ranger.unsupported` may remain
    // claim-blocking. The three features are surfaced as non-claim-blocking
    // explanation records instead.
    let blocking_feature = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.hybrid.ranger.unsupported");
    assert!(
        blocking_feature.is_none() || !blocking_feature.unwrap().claim_blocking,
        "ranger non-spell class-feature burden must no longer be claim-blocking, got {:?}",
        computation.diagnostics
    );

    // The later spell burden still stays claim-blocking: it is the only remaining
    // blocked burden on the ranger slice and will be lifted by the separate SD13-E4
    // ranger spell-burden slice.
    let spell = claim_blocking(&computation, "class_spell.hybrid.ranger.unsupported");
    assert!(
        spell.message.contains("spell"),
        "ranger spell blocker must name the later spell burden: {}",
        spell.message
    );

    // A bounded, non-claim-blocking note keeps the lifted features visible as the
    // named Ranger class-feature family, so the audit surface can still see what
    // is recognized at level 1 and what remains unsupported at level 2+.
    let note = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.ranger.bounded_seam");
    let note = note.unwrap_or_else(|| {
        panic!(
            "expected bounded non-spell class-feature seam note, got {:?}",
            computation.diagnostics
        )
    });
    assert!(
        !note.claim_blocking,
        "ranger bounded class-feature seam note must not be claim-blocking"
    );
    for token in ["favored enemy", "combat style", "tracking"] {
        assert!(
            note.message.contains(token),
            "ranger bounded class-feature seam note must name the '{token}' feature: {}",
            note.message
        );
    }

    // The integrated posture is still blocked (by the spell burden), not a counterfeit
    // computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(view_model.snapshot.is_none());
}

// ----- The accepted Human race seam is preserved on the hybrid path -----

#[test]
fn hybrid_baseline_preserves_human_race_seam() {
    for fixture in [PALADIN_FIXTURE, RANGER_FIXTURE] {
        let input = load(fixture);
        let computation = compute_pilot_base_chassis(&input);

        assert!(
            has_explanation(&computation, "race.human.ability_bonus_target"),
            "hybrid baseline must preserve the Human ability-bonus race seam: {:?}",
            computation.explanations
        );
        assert!(
            has_explanation(&computation, "race.human.bonus_feat_grant"),
            "hybrid baseline must preserve the Human bonus-feat race seam: {:?}",
            computation.explanations
        );
        // The bounded Human race-semantics note stays present and non-claim-blocking.
        assert!(
            computation
                .diagnostics
                .iter()
                .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
            "hybrid baseline must keep the bounded, non-blocking Human race note: {:?}",
            computation.diagnostics
        );
    }
}

// ----- Negative control: the hybrid path must not leak onto other classes -----

#[test]
fn fighter_and_rogue_do_not_gain_hybrid_recognition() {
    // A supported Fighter must not gain a hybrid-baseline recognition record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, "class_chassis.hybrid_baseline.paladin")
            && !has_explanation(&fighter_computation, "class_chassis.hybrid_baseline.ranger"),
        "the Fighter chassis must not surface a hybrid-baseline recognition record"
    );

    // A Rogue must stay a plain blocked negative control, never a hybrid baseline.
    let rogue_fixture = PALADIN_FIXTURE.replace("class:paladin:1", "class:rogue:1");
    let rogue = load(&rogue_fixture);
    let rogue_computation = compute_pilot_base_chassis(&rogue);
    assert!(
        rogue_computation.diagnostics.iter().any(|d| d.claim_blocking),
        "Rogue chassis must remain claim-blocked"
    );
    assert!(
        !has_explanation(&rogue_computation, "class_chassis.hybrid_baseline.paladin")
            && !has_explanation(&rogue_computation, "class_chassis.hybrid_baseline.ranger"),
        "Rogue must not surface any hybrid-baseline recognition record"
    );
    // The ranger class-feature uplift stays ranger-only; a Rogue must not gain the
    // favored-enemy / combat-style / tracking explanations.
    for id in [
        "class_feature.ranger.favored_enemy",
        "class_feature.ranger.combat_style",
        "class_feature.ranger.tracking",
    ] {
        assert!(
            !has_explanation(&rogue_computation, id),
            "Rogue must not surface the ranger class-feature seam '{id}'"
        );
    }
    assert!(
        !rogue_computation
            .diagnostics
            .iter()
            .any(|d| d.id.starts_with("class_feature.hybrid.")
                || d.id.starts_with("class_spell.hybrid.")),
        "Rogue must not surface hybrid class-feature/spell burden diagnostics: {:?}",
        rogue_computation.diagnostics
    );
}

#[test]
fn paladin_level_2_is_not_promoted_by_this_slice() {
    // The slice is bounded to level 1; a level-2 Paladin must not gain the level-1
    // hybrid recognition record and stays blocked.
    let level_2 = PALADIN_FIXTURE.replace("class:paladin:1", "class:paladin:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, "class_chassis.hybrid_baseline.paladin"),
        "level-2 Paladin must not gain the bounded level-1 hybrid recognition record"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Paladin must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix reclassifies both hybrid rows to Blocked/Computed -----

#[test]
fn matrix_paladin_row_is_blocked_computed_and_names_both_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must exist");

    // Moves off the pure Unverified/Observed placeholder, but only to Blocked/Computed.
    assert_eq!(paladin.support_state, SupportState::Blocked);
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_hybrid_level1_chassis_baseline"),
        "paladin row must cite the SD13-F6 hybrid proof surface: {}",
        paladin.grounding_ref
    );
    // The note must name both the non-spell class-feature burden and the later spell burden.
    let note = paladin.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "paladin blocked row must carry a note");
    for token in ["smite", "lay on hands", "divine grace", "mercy", "spell"] {
        assert!(
            note.contains(token),
            "paladin blocked note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_ranger_row_is_partial_computed_and_names_only_spell_burden() {
    // The SD13-E3 ranger class-feature slice lifts the bounded non-spell
    // class-feature burden (favored enemy, combat style, tracking) off the
    // claim-blocking path; the row now reports Partial/Computed with only the
    // later spell burden remaining in the blocker note.
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");

    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row must be Partial after the class-feature uplift"
    );
    assert_ne!(
        ranger.support_state,
        SupportState::Blocked,
        "ranger row must no longer stay Blocked after the class-feature uplift"
    );
    assert_ne!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must not silently reach Supported without the spell burden"
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
        "ranger row must cite the SD13-F6 hybrid proof surface: {}",
        ranger.grounding_ref
    );

    // The blocker note is non-empty because the spell burden remains. The note
        // may reference the lifted non-spell features to explain what was lifted, but
        // it must not assert those features remain blockers — the first sentence / the
        // primary posture of the note must be that the non-spell burden is lifted and
        // the remaining blocker is the spell burden.
        let note = ranger.blocker_or_lossiness_note;
        assert!(
            !note.is_empty(),
            "ranger partial row must keep a non-empty blocker note naming the spell burden"
        );
        assert!(
            note.contains("spell"),
            "ranger partial row note must name the spell burden: {note}"
        );
        assert!(
            note.contains("lift") || note.contains("lifted"),
            "ranger partial row note must reflect the lift posture on the non-spell burden: {note}"
        );
        // It must not say these features are still blockers or that the row "stays
        // blocked" on them — those phrasings would conflate the lifted non-spell burden
        // with the remaining spell burden.
        assert!(
            !note.contains("stays blocked on the non-spell")
                && !note.contains("non-spell class-feature burden (favored enemy, combat style, tracking) is not implemented"),
            "ranger partial row note must not claim the non-spell burden remains a blocker: {note}"
        );

    // The next uplift points at the remaining burden and level-2+ widening slices.
    // It must explicitly call out the SD13-E4 spell burden as the next blocker-
    // posture uplift, and may reference Ranger level-2+ widening (combat style
    // activation, favored enemy bonus resolution, Track feat effect) — but it must
    // not say the non-spell class-feature burden is still a blocker.
    let uplift = ranger.next_required_uplift;
    assert!(
        uplift.contains("spell"),
        "ranger next uplift must point at the SD13-E4 spell burden: {uplift}"
    );
    assert!(
        !uplift.contains("non-spell class-feature slice")
            && !uplift.contains("lift the non-spell"),
        "ranger next uplift must no longer request a non-spell class-feature lift: {uplift}"
    );
}

#[test]
fn matrix_preserves_fighter_and_rogue_accepted_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    // Fighter level-1 and levels-2-10 remain Partial/Computed (not downgraded).
    for id in ["class.fighter.level_1_pilot", "class.fighter.levels_2_10"] {
        let fighter = matrix.row(id).unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            fighter.support_state,
            SupportState::Partial,
            "row {id} must stay Partial after the hybrid slice"
        );
        assert_eq!(fighter.evidence_tier, EvidenceTier::Computed);
    }

    // Rogue stays the blocked negative control.
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Blocked);

    // No row is silently promoted to Supported or Lossy by this slice.
    assert!(
        !matrix.rows.iter().any(|r| r.support_state == SupportState::Supported
            || r.support_state == SupportState::Lossy),
        "the hybrid slice must not promote any row to Supported or Lossy"
    );
}
