//! SD13-E3-F6 Paladin and Ranger level-1 hybrid chassis baseline proof, with the
//! SD13-E3 paladin class-feature follow-up slice wired in.
//!
//! Proves the truthful SD13-F6 slice: the live rules-core surface ingests
//! deterministic Human `class:paladin:1` and `class:ranger:1` inputs, leaves direct
//! computed evidence that acknowledges each hybrid level-1 chassis identity rather
//! than treating it as an undocumented packet placeholder. The Paladin row moved
//! from `Blocked` / `Computed` to `Partial` / `Computed` once the SD13-E3
//! paladin class-feature follow-up slice lifted its bounded non-spell
//! class-feature burden: smite evil, lay on hands, and divine grace are surfaced
//! as computed Paladin level-1 feature explanations, and mercy is named as the
//! Paladin level-3 gate (not yet gained at level 1). The Paladin row remains
//! claim-blocked only on its later hybrid spell burden, deferred to SD13-E4.
//! The Ranger row stays `Blocked` / `Computed` on its non-spell class-feature
//! burden and later spell burden; Ranger gets its own follow-up class-feature
//! slice.
//!
//! It is intentionally not a hybrid class engine. It grounds no Paladin/Ranger level
//! 2+, no smite-target alignment execution, no lay-on-hands consumption, no
//! favored-enemy / combat-style / tracking execution, and no spell-slot /
//! known-or-prepared posture. It also preserves the accepted Fighter 1-3 truth,
//! the Rogue blocked negative control, and the Human race/interaction truth.

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
fn paladin_level1_lifts_non_spell_class_feature_burden_with_named_features() {
    // SD13-E3 paladin class-feature slice lifted the non-spell class-feature
    // burden for the deterministic Human Paladin level-1 posture. Three of the
    // four features named in the matrix row's blocker note are now surfaced as
    // computed Paladin level-1 feature explanations; mercy is named as the
    // Paladin level-3 gate (not yet gained at level 1).
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Smite evil is now a computed Paladin level-1 feature explanation. The
    // chosen Charisma 14 yields a +2 Charisma modifier, so the bounded daily-use
    // cap is max(1, +2) = 2.
    let smite = explanation(&computation, "class_feature.paladin.smite_evil");
    assert_eq!(
        smite.value, 2,
        "paladin smite evil must carry the bounded daily-use cap as its computed value, got {}",
        smite.value
    );
    for token in ["smite evil", "+1", "+2", "daily-use cap"] {
        assert!(
            smite.detail.contains(token),
            "paladin smite evil detail must name '{token}': {}",
            smite.detail
        );
    }

    // Lay on hands is now a computed Paladin level-1 feature explanation. With
    // Charisma +2 and level 1 the pool cap is 1 × max(1, +2) = 2 HP.
    let lay_on_hands = explanation(&computation, "class_feature.paladin.lay_on_hands");
    assert_eq!(
        lay_on_hands.value, 2,
        "paladin lay on hands must carry the bounded pool cap as its computed value, got {}",
        lay_on_hands.value
    );
    assert!(
        lay_on_hands.detail.contains("lay on hands")
            && lay_on_hands.detail.contains("daily-healing pool"),
        "paladin lay on hands detail must name the pool: {}",
        lay_on_hands.detail
    );

    // Divine grace is now a computed Paladin level-1 feature explanation. It
    // adds the Charisma modifier (+2) to each saving throw.
    let divine_grace = explanation(&computation, "class_feature.paladin.divine_grace");
    assert_eq!(
        divine_grace.value, 2,
        "paladin divine grace must carry the Charisma modifier as its computed value, got {}",
        divine_grace.value
    );
    for token in ["divine grace", "Charisma modifier", "saving throw"] {
        assert!(
            divine_grace.detail.contains(token),
            "paladin divine grace detail must name '{token}': {}",
            divine_grace.detail
        );
    }

    // Mercy is named as the Paladin level-3 gate (not yet gained at level 1).
    // The diagnostic must be present, non-claim-blocking, and name mercy and
    // the level-3 gate.
    let mercy = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.paladin.mercy.level_3_gate")
        .unwrap_or_else(|| {
            panic!(
                "paladin mercy level-3 gate diagnostic must exist: {:?}",
                computation.diagnostics
            )
        });
    assert!(
        !mercy.claim_blocking,
        "paladin mercy level-3 gate must be non-claim-blocking: {mercy:?}"
    );
    for token in ["mercy", "level 3", "level 1"] {
        assert!(
            mercy.message.contains(token),
            "paladin mercy gate must name '{token}': {}",
            mercy.message
        );
    }

    // The legacy `class_feature.hybrid.paladin.unsupported` claim-block must
    // NOT appear on the runtime path any longer — that burden is lifted.
    assert!(
        computation
            .diagnostics
            .iter()
            .all(|d| d.id != "class_feature.hybrid.paladin.unsupported"),
        "paladin non-spell class-feature burden must be lifted; the legacy unsupported \
         diagnostic must not appear: {:?}",
        computation.diagnostics
    );

    // The hybrid class-feature explanations (smite / lay-on-hands / divine-grace /
    // mercy gate) must each name the four-burden tokens somewhere on the runtime
    // path so the matrix row's blocker-note names stay legible on the path.
    let all_text: String = computation
        .explanations
        .iter()
        .map(|e| e.detail.as_str())
        .chain(computation.diagnostics.iter().map(|d| d.message.as_str()))
        .collect::<Vec<&str>>()
        .join(" ");
    for token in ["smite", "lay on hands", "divine grace", "mercy"] {
        assert!(
            all_text.contains(token),
            "paladin level-1 path must still name the '{token}' feature burden: {all_text}"
        );
    }
}

#[test]
fn paladin_level1_stays_blocked_on_later_spell_burden_only() {
    // After the non-spell class-feature burden is lifted, the deterministic
    // Human Paladin level-1 posture stays claim-blocked ONLY on the later
    // hybrid spell burden. The integrated receipt remains Blocked; the
    // view-model snapshot stays withheld.
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The later spell burden must be named explicitly and stay claim-blocking.
    let spell = claim_blocking(&computation, "class_spell.hybrid.paladin.unsupported");
    assert!(
        spell.message.contains("spell"),
        "paladin spell blocker must name the later spell burden: {}",
        spell.message
    );
    for token in ["spell slots", "spells known/prepared"] {
        assert!(
            spell.message.contains(token),
            "paladin spell blocker must name the '{token}' later-hybrid burden: {}",
            spell.message
        );
    }

    // The integrated posture is blocked (still spell-blocked) — never a
    // counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "spell-blocked paladin baseline must not emit a computed snapshot"
    );
}

#[test]
fn ranger_level1_stays_blocked_naming_class_feature_and_spell_burden() {
    let input = load(RANGER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let feature = claim_blocking(&computation, "class_feature.hybrid.ranger.unsupported");
    for token in ["favored enemy", "combat style", "tracking"] {
        assert!(
            feature.message.contains(token),
            "ranger feature blocker must name the '{token}' burden: {}",
            feature.message
        );
    }

    let spell = claim_blocking(&computation, "class_spell.hybrid.ranger.unsupported");
    assert!(
        spell.message.contains("spell"),
        "ranger spell blocker must name the later spell burden: {}",
        spell.message
    );

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
fn matrix_paladin_row_is_partial_computed_after_non_spell_burden_lift() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must exist");

    // SD13-E3 paladin class-feature slice lifted the non-spell class-feature
    // burden, so the paladin row moves from Blocked/Computed to Partial/Computed
    // while still citing the same SD13-F6 hybrid proof surface.
    assert_eq!(paladin.support_state, SupportState::Partial);
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
    // The note must still name the four non-spell class-feature burdens (so the
    // lift-to-partial transition stays auditable) AND the remaining later spell
    // burden. The legacy `class_feature.hybrid.paladin.unsupported` identifier is
    // preserved for backwards-compatibility tracking.
    let note = paladin.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "paladin partial row must carry a note");
    for token in ["smite", "lay on hands", "divine grace", "mercy", "spell"] {
        assert!(
            note.contains(token),
            "paladin partial note must name the '{token}' burden: {note}"
        );
    }
}

#[test]
fn matrix_ranger_row_is_blocked_computed_and_names_both_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");

    assert_eq!(ranger.support_state, SupportState::Blocked);
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
    let note = ranger.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "ranger blocked row must carry a note");
    for token in ["favored enemy", "combat style", "tracking", "spell"] {
        assert!(
            note.contains(token),
            "ranger blocked note must name the '{token}' burden: {note}"
        );
    }
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
