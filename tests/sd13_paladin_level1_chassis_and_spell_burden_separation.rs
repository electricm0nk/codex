//! SD13-E3/E4/E5 Paladin level-1 chassis-and-spell-burden separation proof.
//!
//! Proves the deeper Paladin-only decomposition that sits on top of the accepted
//! SD13-F6 hybrid baseline, through the SD13-E5 level-gate grounding slice: the
//! live rules-core surface grounds Smite Evil for real (SD13-E4) and grounds the
//! lay on hands / divine grace / mercy burdens as correct PF1 CRB level-gate
//! absences (SD13-E5), keeping one distinct claim-blocking diagnostic for the
//! later partial-caster spell burden, and the support-state matrix row for
//! `class.paladin.hybrid_chassis_and_spell_burden` is promoted from Blocked to
//! Partial with a note that names each burden's honest state explicitly.
//!
//! Smite Evil stays grounded for real: at the bounded level-1 baseline a
//! paladin gets 1 use per day, an attack-roll bonus equal to her Charisma
//! modifier (if positive; PF1 Core Rulebook Smite Evil applies the Charisma
//! bonus "if any", never a penalty), and a damage bonus equal to her paladin
//! level. This grounds only that flat numeric formula; it grounds no alignment
//! / evil-subtype target resolution, no swift-action activation bookkeeping,
//! no deflection-AC-vs-target bonus, and no evil-outsider/evil-dragon/undead
//! damage doubling.
//!
//! Lay on hands and divine grace are 2nd-level paladin features and mercy is a
//! 3rd-level paladin feature in the PF1 Core Rulebook, so at level 1 the honest
//! computed surface is their correct ABSENCE: three grounded level-gate records
//! (value 0 each) that name the at-grant formula without computing it. No
//! lay-on-hands heal amount, no divine-grace save bonus, and no mercy effect is
//! fabricated.
//!
//! It is intentionally not a hybrid class engine. It grounds no Paladin level
//! 2+, no lay-on-hands / divine-grace / mercy math, no partial-caster slot
//! math, no deity resolution, no domain mechanics, no alignment-driven target
//! resolution, no healing-resource accounting, and no spell posture
//! computation. It also preserves the accepted Fighter 1-3 truth, the Rogue
//! negative control, the shared F6 hybrid blockers (so the F6 test continues
//! to pass), the Sorcerer F7 baseline truth, and the Human race / interaction
//! seam.

use codex::rules_core::character_input::{AcquisitionMode, SpellSelection};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic,
    HeadlessReceiptStatus,
    PilotBaseChassisComputation,
    build_pilot_headless_receipt,
    compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};
mod common;
use common::{load, explanation, has_explanation};

const PALADIN_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_paladin_level1_sd13_deterministic_input.txt");

const RANGER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_ranger_level1_sd13_deterministic_input.txt");

const FIGHTER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

// The one remaining Paladin-only claim-blocking diagnostic: the partial-caster
// spell burden, distinct from the (now grounded) non-spell class-feature
// burdens, so Paladin's chassis and spell burdens stay separable on the
// runtime path.
const PALADIN_PARTIAL_CASTER_ID: &str = "class_spell.paladin.partial_caster.unsupported";

// The formerly claim-blocking per-feature ids are now grounded (Smite Evil for
// real, lay on hands / divine grace / mercy as correct level-gate absences);
// none of them may appear as a diagnostic any longer.
const PALADIN_SMITE_EVIL_BLOCKER_ID: &str = "class_feature.paladin.smite_evil.unsupported";
const PALADIN_LAY_ON_HANDS_BLOCKER_ID: &str = "class_feature.paladin.lay_on_hands.unsupported";
const PALADIN_DIVINE_GRACE_BLOCKER_ID: &str = "class_feature.paladin.divine_grace.unsupported";
const PALADIN_MERCY_BLOCKER_ID: &str = "class_feature.paladin.mercy.unsupported";

// The three grounded level-gate explanations (value 0 each): lay on hands and
// divine grace are 2nd-level paladin features and mercy is a 3rd-level paladin
// feature in the PF1 Core Rulebook, so at level 1 their honest computed
// surface is their correct absence, with the at-grant formula named but not
// computed.
const PALADIN_LAY_ON_HANDS_GATE_ID: &str = "class_chassis.paladin.level_gate.lay_on_hands";
const PALADIN_DIVINE_GRACE_GATE_ID: &str = "class_chassis.paladin.level_gate.divine_grace";
const PALADIN_MERCY_GATE_ID: &str = "class_chassis.paladin.level_gate.mercy";

// Smite Evil's three grounded numeric explanations (uses per day, attack-roll
// bonus, damage bonus), naming this repo's `class_chassis.<class>.<pillar>`
// convention.
const PALADIN_SMITE_EVIL_USES_PER_DAY_ID: &str = "class_chassis.paladin.smite_evil_uses_per_day";
const PALADIN_SMITE_EVIL_ATTACK_BONUS_ID: &str = "class_chassis.paladin.smite_evil_attack_bonus";
const PALADIN_SMITE_EVIL_DAMAGE_BONUS_ID: &str = "class_chassis.paladin.smite_evil_damage_bonus";

// F6 hybrid blockers are accepted truth and must still be claim-blocking for
// Ranger regression preservation. The F6 test asserts both of these ids.
const F6_HYBRID_PALADIN_FEATURE_ID: &str = "class_feature.hybrid.paladin.unsupported";
const F6_HYBRID_PALADIN_SPELL_ID: &str = "class_spell.hybrid.paladin.unsupported";

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

fn has_diagnostic(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.diagnostics.iter().any(|d| d.id == id)
}

// ----- Retired per-feature blockers and grounded level-gate records -----

#[test]
fn paladin_level1_retires_lay_on_hands_divine_grace_mercy_blockers() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // All four formerly claim-blocking per-feature ids are grounded now: none
    // of them may appear as a diagnostic at all.
    for id in [
        PALADIN_SMITE_EVIL_BLOCKER_ID,
        PALADIN_LAY_ON_HANDS_BLOCKER_ID,
        PALADIN_DIVINE_GRACE_BLOCKER_ID,
        PALADIN_MERCY_BLOCKER_ID,
    ] {
        assert!(
            !has_diagnostic(&computation, id),
            "retired per-feature paladin blocker '{id}' must no longer appear, got {:?}",
            computation.diagnostics
        );
    }

    // Both F6 hybrid blanket blockers are now retired. The non-spell class-feature
    // one went first -- it flatly claimed Smite Evil / lay on hands / divine grace /
    // mercy were unimplemented, which this exact per-class decomposition (dispatched
    // on the same input) contradicts by grounding Smite Evil for real and lay on
    // hands / divine grace / mercy as correct level-1 absences. The later-spell one
    // followed (2026-07-28) for the identical reason: Paladins have no `CAST:` row
    // in `cr_classes.lst` before class level 4, and this same function already
    // grounds the level-1 spell posture (effective caster level 0, access ceiling 0,
    // zero prepared spells), so the blanket "out of scope" claim was false. See
    // `tests/hybrid_diagnostic_grounded_contradiction.rs` and
    // `tests/v06_hybrid_level1_no_spellcasting_is_computed.rs`.
    for retired in [F6_HYBRID_PALADIN_FEATURE_ID, F6_HYBRID_PALADIN_SPELL_ID] {
        assert!(
            !has_diagnostic(&computation, retired),
            "the retired F6 hybrid blocker '{retired}' must not reappear: {:?}",
            computation.diagnostics
        );
    }

    // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25)
    // PALADIN_PARTIAL_CASTER_ID is no longer unconditional: at level 1 no
    // paladin spell level is accessible at all, so a bare fixture with zero
    // prepared spells has a genuinely valid (empty) posture and the blocker
    // correctly does not fire here -- this slice still grounds no spell
    // surface, it just no longer claims a burden that isn't genuinely
    // violated. A genuinely invalid preparation (an off-list spell) still
    // trips it, proving the blocker isn't simply retired outright.
    let mut invalid_input = input;
    invalid_input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Magic Missile".to_owned(),
        source_class_id: "class:paladin".to_owned(),
        acquisition_mode: AcquisitionMode::Prepared,
    });
    let invalid_computation = compute_pilot_base_chassis(&invalid_input);
    let diag = claim_blocking(&invalid_computation, PALADIN_PARTIAL_CASTER_ID);
    assert!(
        !diag.message.is_empty(),
        "the partial-caster blocker must still fire and carry a non-empty message when the \
         posture is genuinely violated"
    );
}

#[test]
fn paladin_level_gate_records_ground_correct_absence_at_level_1() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook level gates: lay on hands and divine grace are
    // 2nd-level paladin features; mercy is a 3rd-level paladin feature. At
    // level 1 the honest computed surface is their correct absence (value 0),
    // with the at-grant formula named but not computed.
    for id in [
        PALADIN_LAY_ON_HANDS_GATE_ID,
        PALADIN_DIVINE_GRACE_GATE_ID,
        PALADIN_MERCY_GATE_ID,
    ] {
        let gate = explanation(&computation, id);
        assert_eq!(
            gate.value, 0,
            "level-gate record '{id}' must carry value 0 (correct absence at level 1): {gate:?}"
        );
        assert!(
            gate.detail.contains("correctly absent at level 1"),
            "level-gate record '{id}' must ground the correct absence at level 1: {}",
            gate.detail
        );
        assert!(
            gate.detail.contains("named but not computed"),
            "level-gate record '{id}' must name the at-grant formula without computing it: {}",
            gate.detail
        );
    }
}

#[test]
fn paladin_lay_on_hands_gate_names_second_level_grant_and_formula() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let gate = explanation(&computation, PALADIN_LAY_ON_HANDS_GATE_ID);
    // Lay on hands is a 2nd-level paladin feature: heals 1d6 per two paladin
    // levels, uses/day = 1/2 paladin level + Charisma modifier.
    assert!(
        gate.detail.contains("2nd-level"),
        "lay-on-hands gate must name the 2nd-level PF1 CRB grant: {}",
        gate.detail
    );
    assert!(
        gate.detail.contains("1d6"),
        "lay-on-hands gate must name the 1d6-per-two-levels heal formula: {}",
        gate.detail
    );
}

#[test]
fn paladin_divine_grace_gate_names_second_level_grant_and_formula() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let gate = explanation(&computation, PALADIN_DIVINE_GRACE_GATE_ID);
    // Divine grace is a 2nd-level paladin feature: +Charisma bonus on all
    // saving throws.
    assert!(
        gate.detail.contains("2nd-level"),
        "divine-grace gate must name the 2nd-level PF1 CRB grant: {}",
        gate.detail
    );
    assert!(
        gate.detail.contains("saving throws"),
        "divine-grace gate must name the Charisma-to-saving-throws formula: {}",
        gate.detail
    );
}

#[test]
fn paladin_mercy_gate_names_third_level_grant_not_level_six() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let gate = explanation(&computation, PALADIN_MERCY_GATE_ID);
    // Mercy is a 3rd-level paladin feature in PF1 (gained at 3rd and every
    // three levels after), chosen from the mercy list and attached to lay on
    // hands. The formerly-catalogued "level-6" claim was a rules error.
    assert!(
        gate.detail.contains("3rd-level"),
        "mercy gate must name the 3rd-level PF1 CRB grant: {}",
        gate.detail
    );
    assert!(
        gate.detail.contains("lay on hands"),
        "mercy gate must name that mercies attach to lay on hands: {}",
        gate.detail
    );
    assert!(
        !gate.detail.contains("level-6") && !gate.detail.contains("level 6"),
        "mercy gate must not repeat the corrected level-6 rules error: {}",
        gate.detail
    );
}

#[test]
fn paladin_smite_evil_uses_per_day_attack_and_damage_bonus_are_grounded() {
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture is a level-1 Human Paladin with Charisma 14 (modifier +2).
    // PF1 Core Rulebook Smite Evil at level 1: 1 use/day, attack-roll bonus =
    // Charisma modifier (if positive), damage bonus = paladin level.
    let uses_per_day = explanation(&computation, PALADIN_SMITE_EVIL_USES_PER_DAY_ID);
    assert_eq!(
        uses_per_day.value, 1,
        "smite evil uses per day must be 1 at paladin level 1: {uses_per_day:?}"
    );

    // CG-03 fix: the Human ability-bonus choice's +2 racial Charisma adjustment is now
    // applied before the modifier is derived (base 14 -> 16, modifier +2 -> +3).
    let attack_bonus = explanation(&computation, PALADIN_SMITE_EVIL_ATTACK_BONUS_ID);
    assert_eq!(
        attack_bonus.value, 3,
        "smite evil attack bonus must equal the Charisma modifier (+3 for CHA 14 + 2 Human \
         racial): {attack_bonus:?}"
    );

    let damage_bonus = explanation(&computation, PALADIN_SMITE_EVIL_DAMAGE_BONUS_ID);
    assert_eq!(
        damage_bonus.value, 1,
        "smite evil damage bonus must equal paladin level (1 at level 1): {damage_bonus:?}"
    );

    // No fabricated math leaks: grounding the level gates must not silently
    // compute lay on hands healing or divine grace's save bonus — the gates
    // name the at-grant formulas without computing them.
    assert!(
        !has_explanation(&computation, "class_chassis.paladin.lay_on_hands_heal_amount"),
        "lay on hands healing must not be fabricated by grounding the level gates"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.paladin.divine_grace_save_bonus"),
        "divine grace's save bonus must not be fabricated by grounding the level gates"
    );
}

// ----- Partial-caster spell burden is separable from the per-feature blockers -----

#[test]
fn paladin_partial_caster_blocker_is_separate_and_partial_caster_specific() {
    // (v0.6 alpha swarm, risks item 8, third slice, 2026-07-25)
    // PALADIN_PARTIAL_CASTER_ID is no longer unconditional: a bare level-1
    // fixture (zero prepared spells, nothing accessible yet anyway) has a
    // genuinely valid posture, so the blocker would not fire at all. This
    // test is specifically about the blocker's MESSAGE content, so a
    // genuinely invalid preparation (an off-list spell) is added to make it
    // fire for real, mirroring the same construction used in
    // `paladin_level1_retires_lay_on_hands_divine_grace_mercy_blockers`.
    let mut input = load(PALADIN_FIXTURE);
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Magic Missile".to_owned(),
        source_class_id: "class:paladin".to_owned(),
        acquisition_mode: AcquisitionMode::Prepared,
    });
    let computation = compute_pilot_base_chassis(&input);

    let spell = claim_blocking(&computation, PALADIN_PARTIAL_CASTER_ID);
    // The slice must name the partial-caster posture explicitly so the later
    // spell-burden closure work cannot confuse Paladin with a full divine caster
    // (Cleric / Druid) or with a hybrid-but-not-yet-spell-bearing hybrid
    // (which Paladin is not).
    assert!(
        spell.message.contains("partial")
            || spell.message.contains("half-spell")
            || spell.message.contains("half caster")
            || spell.message.contains("two levels")
            || spell.message.contains("slower"),
        "paladin partial-caster blocker must name the partial-caster posture: {}",
        spell.message
    );
    assert!(
        !spell.message.contains("full caster"),
        "paladin partial-caster blocker must not collapse into a full-caster claim: {}",
        spell.message
    );

    // The blocker must carry the corrected PF1 Core Rulebook facts: paladin
    // spells begin at 4th level and effective caster level = paladin level - 3.
    // The formerly-catalogued "level - 2 / slots at level 2" claim was a rules
    // error and must not survive anywhere in the message.
    assert!(
        spell.message.contains("level - 3"),
        "paladin partial-caster blocker must state caster level = paladin level - 3: {}",
        spell.message
    );
    assert!(
        spell.message.contains("level 4"),
        "paladin partial-caster blocker must state spells begin at level 4: {}",
        spell.message
    );
    assert!(
        !spell.message.contains("level - 2") && !spell.message.contains("available at level 2"),
        "paladin partial-caster blocker must not repeat the corrected caster-level rules error: {}",
        spell.message
    );
}

#[test]
fn paladin_separated_blockers_do_not_emerge_for_ranger_or_fighter() {
    // Negative control: the separated Paladin-only blockers must not leak onto
    // the Ranger hybrid baseline — that lane owns its own F6 blockers and the
    // Ranger-only decomposition slice (when it lands).
    let ranger_input = load(RANGER_FIXTURE);
    let ranger_computation = compute_pilot_base_chassis(&ranger_input);
    assert!(
        !has_diagnostic(&ranger_computation, PALADIN_PARTIAL_CASTER_ID),
        "ranger must not gain the Paladin-only partial-caster blocker"
    );
    for id in [
        PALADIN_SMITE_EVIL_USES_PER_DAY_ID,
        PALADIN_SMITE_EVIL_ATTACK_BONUS_ID,
        PALADIN_SMITE_EVIL_DAMAGE_BONUS_ID,
        PALADIN_LAY_ON_HANDS_GATE_ID,
        PALADIN_DIVINE_GRACE_GATE_ID,
        PALADIN_MERCY_GATE_ID,
    ] {
        assert!(
            !has_explanation(&ranger_computation, id),
            "ranger must not gain a Paladin-only grounded explanation '{id}'"
        );
    }

    // And the Fighter must stay on the Fighter-shaped accepted truth, never
    // gain any hybrid or partial-caster blocker.
    let fighter_input = load(FIGHTER_FIXTURE);
    let fighter_computation = compute_pilot_base_chassis(&fighter_input);
    for id in [
        PALADIN_PARTIAL_CASTER_ID,
        F6_HYBRID_PALADIN_FEATURE_ID,
        F6_HYBRID_PALADIN_SPELL_ID,
    ] {
        assert!(
            !has_diagnostic(&fighter_computation, id),
            "fighter must not gain a Paladin-only or hybrid blocker '{id}'"
        );
    }
    for id in [
        PALADIN_SMITE_EVIL_USES_PER_DAY_ID,
        PALADIN_SMITE_EVIL_ATTACK_BONUS_ID,
        PALADIN_SMITE_EVIL_DAMAGE_BONUS_ID,
        PALADIN_LAY_ON_HANDS_GATE_ID,
        PALADIN_DIVINE_GRACE_GATE_ID,
        PALADIN_MERCY_GATE_ID,
    ] {
        assert!(
            !has_explanation(&fighter_computation, id),
            "fighter must not gain a Paladin-only grounded explanation '{id}'"
        );
    }
}

// ----- F6 acceptance truth must still be intact -----

#[test]
fn paladin_f6_hybrid_blockers_remain_intact_under_separation() {
    // Both F6 hybrid blanket blockers are retired, each superseded by grounded
    // per-class records on this same input rather than merely dropped: the
    // class-feature one by Smite Evil / the level-gate records, and the
    // later-spell one (2026-07-28) by the partial-caster spell posture this
    // very file pins below. This slice remains an extension, never a downgrade,
    // of the F6 acceptance surface -- what F6 asserted as an unmet burden is now
    // asserted as a computed value. See
    // `tests/hybrid_diagnostic_grounded_contradiction.rs` and
    // `tests/v06_hybrid_level1_no_spellcasting_is_computed.rs`.
    let input = load(PALADIN_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for retired in [F6_HYBRID_PALADIN_FEATURE_ID, F6_HYBRID_PALADIN_SPELL_ID] {
        assert!(
            !has_diagnostic(&computation, retired),
            "the retired F6 hybrid blocker '{retired}' must not reappear: {:?}",
            computation.diagnostics
        );
    }

    // The burden the retired spell blocker used to assert is now a grounded
    // computed value, not an absence: this is what makes the retirement an
    // extension rather than a downgrade.
    assert!(
        has_explanation(
            &computation,
            "class_chassis.paladin.partial_caster.effective_caster_level"
        ) && has_explanation(
            &computation,
            "class_chassis.paladin.partial_caster.spell_level_access"
        ),
        "the retired spell blocker must be superseded by grounded partial-caster \
         records, not merely dropped: {:?}",
        computation.explanations
    );

    // The F6 chassis recognition explanation must still be present so the F6
    // test does not lose its identity-proof surface.
    assert!(
        has_explanation(&computation, "class_chassis.hybrid_baseline.paladin"),
        "F6 chassis recognition explanation must remain"
    );
}

// ----- The integrated posture stays Blocked, never fakes Computed -----

#[test]
fn paladin_level1_still_yields_blocked_headless_receipt_and_view_model() {
    let input = load(PALADIN_FIXTURE);
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Blocked,
        "separated per-burden blockers must keep the integrated paladin posture Blocked"
    );

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked paladin posture must not emit a computed snapshot"
    );
}

// ----- Control plane: matrix row carries the separated burden note -----

#[test]
fn matrix_paladin_row_is_promoted_to_partial_with_honest_burden_note() {
    let matrix = seeded_current_truth();
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin hybrid row must exist");

    // The SD13-E5 level-gate slice promotes the row Blocked -> Partial: all
    // four named non-spell burdens are now grounded (Smite Evil for real,
    // lay on hands / divine grace / mercy as correct level-gate absences),
    // while the hybrid chassis pair and the partial-caster spell burden stay
    // named and claim-blocking. SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-17) later promotes the row to
    // Supported/ProductVisible.
    assert_eq!(paladin.support_state, SupportState::Supported);
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        paladin
            .grounding_ref
            .contains("sd13_paladin_level1_chassis_and_spell_burden_separation"),
        "paladin row must cite this slice as its proof surface: {}",
        paladin.grounding_ref
    );

    // The matrix note must name each burden's honest state explicitly.
    let note = paladin.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "paladin partial row must carry a note");
    for token in ["lay on hands", "divine grace", "mercy"] {
        assert!(
            note.contains(token),
            "paladin note must name the '{token}' level-gate grounding: {note}"
        );
    }
    // The level gates are grounded absences, not still-missing burdens.
    assert!(
        note.contains("level gate") || note.contains("level-gate"),
        "paladin note must name the grounded level gates: {note}"
    );
    assert!(
        note.contains("smite") && note.contains("grounded"),
        "paladin note must name smite evil as grounded, not still-blocked: {note}"
    );
    // The note must also name the partial-caster posture distinctly from the
    // grounded chassis burdens, with the corrected PF1 CRB facts (spells begin
    // at level 4; caster level = paladin level - 3), so the later spell-burden
    // closure cannot accidentally collapse Paladin into a Cleric shape.
    assert!(
        note.contains("partial-caster") || note.contains("partial caster"),
        "paladin note must name the partial-caster posture: {note}"
    );
    assert!(
        note.contains("level - 3") && note.contains("level 4"),
        "paladin note must carry the corrected partial-caster facts: {note}"
    );
    assert!(
        !note.contains("level - 2") && !note.contains("available at level 2"),
        "paladin note must not repeat the corrected caster-level rules error: {note}"
    );
}

#[test]
fn matrix_ranger_row_does_not_borrow_paladin_separated_burden_note() {
    // The non-goal is explicit: do not collapse Paladin into Ranger. The
    // matrix must keep them on distinct rows with their own burden notes.
    let matrix = seeded_current_truth();
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger hybrid row must exist");

    let ranger_note = ranger.blocker_or_lossiness_note;
    for token in [
        "smite evil",
        "lay on hands",
        "divine grace",
        "mercy",
    ] {
    // "partial-caster" was removed from this forbidden-token list by the further
    // SD13-E5 ranger partial-caster slice: the ranger note now legitimately names
    // the ranger's OWN grounded partial-caster identity pair (the ranger is a
    // Wisdom-based partial caster with the same level - 3 rule), not borrowed
    // paladin text; the paladin-specific feature tokens above still may not leak.
        assert!(
            !ranger_note.contains(token),
            "ranger blocked note must not borrow paladin per-feature burden '{token}': {ranger_note}"
        );
    }
}

#[test]
fn matrix_preserves_fighter_rogue_sorcerer_and_other_class_truth() {
    let matrix = seeded_current_truth();

    // Fighter rows were later promoted to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-16).
    for id in ["class.fighter.level_1_pilot", "class.fighter.levels_2_10"] {
        let row = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Supported,
            "row {id} must be Supported after the SD-19 class-row promotion"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    }

    // Rogue was later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-17).
    let rogue = matrix
        .row("class.rogue.bounded_progression")
        .expect("rogue row must exist");
    assert_eq!(rogue.support_state, SupportState::Supported);
    assert_eq!(rogue.evidence_tier, EvidenceTier::ProductVisible);

    // Sorcerer was later promoted to Partial/Computed by its own SD13-E4
    // decomposition slice (Eschew Materials grounded for real), then to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-17).
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer row must exist");
    assert_eq!(sorcerer.support_state, SupportState::Supported);
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);

    // Bard was later promoted to Partial/Computed by its own SD13-E4 decomposition
    // slice (Bardic Knowledge grounded for real), then to Supported/ProductVisible
    // by SD-19's Class Progression Catalog browser UI-surfacing work (2026-07-16).
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard row must exist");
    assert_eq!(
        bard.support_state,
        SupportState::Supported,
        "bard row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);

    // Wizard carried its accepted post-merge-receipt posture (Blocked/Computed) at
    // the time this test was first written, but a later SD13-E4 Wizard
    // decomposition slice grounds Scribe Scroll for real, promoting the row to
    // Partial/Computed, and SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-17) promotes it again to Supported/ProductVisible.
    // This test now pins that current truth.
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Supported,
        "wizard row must keep its later-accepted Supported posture after the paladin-decomposition slice"
    );
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);

    // Monk, Druid, Barbarian, and Cleric were later promoted to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-16).
    for id in [
        "class.monk.bounded_progression",
        "class.druid.progression_and_spell_burden",
        "class.barbarian.bounded_progression",
        "class.cleric.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(id)
            .unwrap_or_else(|| panic!("row {id} must exist"));
        assert_eq!(row.support_state, SupportState::Supported);
        assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    }

    // No row is silently promoted to Supported by this slice. (school.abjuration/
    // illusion.spell_reachability were later promoted to Supported/Product-visible
    // by SD-19's operator-driven UI-surfacing work, 2026-07-16 -- excluded here,
    // not an unintended promotion by this slice.)
    assert!(
        !matrix.rows.iter().any(|r| r.support_state == SupportState::Supported
            && r.row_id != "school.abjuration.spell_reachability"
            && r.row_id != "school.illusion.spell_reachability"
            && r.row_id != "school.conjuration.spell_reachability"
            && r.row_id != "school.divination.spell_reachability"
            && r.row_id != "school.enchantment.spell_reachability"
            && r.row_id != "school.evocation.spell_reachability"
            && r.row_id != "school.necromancy.spell_reachability"
            && r.row_id != "school.transmutation.spell_reachability"
            && r.row_id != "school.universal.spell_reachability"
            && r.row_id != "equipment.arms_armor.equipment_reachability"
            && r.row_id != "equipment.general.equipment_reachability"
            && r.row_id != "equipment.magic_items.equipment_reachability"
            && r.row_id != "race.human.pilot_semantics"
            && r.row_id != "race.dwarf.bounded_semantics"
            && r.row_id != "race.elf.bounded_semantics"
            && r.row_id != "race.gnome.bounded_semantics"
            && r.row_id != "race.half_elf.bounded_semantics"
            && r.row_id != "race.half_orc.bounded_semantics"
            && r.row_id != "race.halfling.bounded_semantics"
            && r.row_id != "class.fighter.level_1_pilot"
            && r.row_id != "class.fighter.levels_2_10"
            && r.row_id != "class.monk.bounded_progression"
            && r.row_id != "class.druid.progression_and_spell_burden"
                && r.row_id != "class.barbarian.bounded_progression"
                && r.row_id != "class.cleric.progression_and_spell_burden"
                && r.row_id != "class.wizard.progression_and_spell_burden"
                && r.row_id != "class.rogue.bounded_progression"
                && r.row_id != "class.sorcerer.progression_and_spell_burden"
                && r.row_id != "class.bard.progression_and_spell_burden"
            && r.row_id != "equipment.equipmods.equipment_reachability"
            && r.row_id != "class.paladin.hybrid_chassis_and_spell_burden"
            && r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"
            && r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"),
        "the paladin-decomposition slice must not promote any row to Supported"
    );
}
