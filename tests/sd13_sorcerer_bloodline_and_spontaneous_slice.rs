//! SD13-E4-F8 Sorcerer level-1 bloodline and spontaneous spell-slot slice.
//!
//! Bounded follow-up to SD13-E4-F7: this slice lifts the Human Sorcerer level-1
//! bloodline burden and the spontaneous known-spell / slot posture burden to
//! direct computed evidence, while keeping level-2+ progression and the broader
//! spell-support surface (prepared posture, school choice, sorcery points,
//! metamagic, general spell totals) explicitly out of scope and claim-blocked.
//!
//! Source-of-truth magnitudes are sourced from PF1 Core Rulebook Sorcerer and
//! Arcane bloodline tables (cr_abilities_class.lst rows for Sorcerer level-1
//! spells known / spells per day, Arcane bloodline level-1 power, and the
//! "High Ability Scores" sidebar bonus-spells mapping). They are not
//! oracle-checked parity.
//!
//! It is intentionally not a full spell engine. It fabricates no school choice,
//! no sorcerer-only metamagic, no sorcery points, no cross-class spell math, no
//! multiclass spell progression, no level-2+ progression, no item/feat spell
//! modifiers, and no non-Human race seam on this slice. The bounded
//! deterministic Human Sorcerer level-1 posture with the chosen Arcane bloodline
//! is the only input proven here. The fixture's CHA 17 is the deterministic
//! driver that fixes the high-CHA bonus-spell bracket (+2 at CHA 17).
//!
//! Negative controls:
//! - the deterministic Human Fighter level-1 input must not gain any Sorcerer
//!   bloodline or spontaneous explanation or diagnostic;
//! - a level-2 Sorcerer must not gain the level-1-only bloodline/spontaneous
//!   recognition and stays blocked on level-2+ progression;
//! - a Sorcerer without a bloodline selection must stay claim-blocked on the
//!   bloodline burden (no silent acceptance of "no choice picked" as a
//!   computed blank).
//!
//! The matrix reclassification: this slice promotes the
//! `class.sorcerer.progression_and_spell_burden` row from `Blocked` /
//! `Computed` to `Partial` / `Computed`, with the blocker note naming only the
//! remaining level-2+ progression gap (and the bounded, intentionally out-of-
//! scope spell-support surface) and the freshness posture staying
//! `RefreshableFromLiveProof`.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, HeadlessReceiptStatus, PilotBaseChassisComputation,
    build_pilot_headless_receipt, compute_pilot_base_chassis,
};
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_sd13_e1_f1_current_truth,
};

const SORCERER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

// ----- Stable, bounded, source-cited identity anchors for this slice -----
//
// The two constants below are documentary anchors only — they pin the bounded
// choice-id vocabulary on the deterministic seam so future slices (or audit
// readers) can verify the spell-bearing identity and bloodline selection
// without re-deriving them. The active assertions in this test file use the
// fully-qualified explanation-id constants that follow, so these two are
// intentionally not referenced by any test body.
#[allow(dead_code)]
const BLOODLINE_SELECTION_CHOICE_ID: &str = "choice:sorcerer_bloodline";
#[allow(dead_code)]
const ARCANE_BLOODLINE_SELECTION: &str = "bloodline:arcane";
const BLOODLINE_POWER_EXPLANATION_ID: &str = "class_feature.sorcerer.bloodline.arcane.arcane_bond";
const SPONTANEOUS_KNOWN_EXPLANATION_ID: &str = "class_spell.sorcerer.spontaneous.spells_known";
const SPONTANEOUS_SLOTS_EXPLANATION_ID: &str = "class_spell.sorcerer.spontaneous.spells_per_day";
const SPONTANEOUS_DC_EXPLANATION_ID: &str = "class_spell.sorcerer.spontaneous.spell_save_dc";

// ----- Bounded CRB-derived Sorcerer level-1 magnitudes (PF1 CRB, not parity) -----
//   cr_abilities_class.lst Sorcerer Level 1: spells known 4, spells per day 3 (1st)
//   cr_abilities_class.lst Arcane bloodline level-1 power: Arcane Bond (familiar or item)
//   cr_abilities_class.lst High Ability Scores sidebar: spontaneous caster converts the
//     high-stat bonus-spells bracket into additional spells known/slots per day at the
//     highest castable level. CHA 17 falls in the +2 bonus bracket (ability 16 -> +2).
const SORCERER_LEVEL_1_SPELLS_KNOWN_BASE: i16 = 4;
const SORCERER_LEVEL_1_SPELLS_PER_DAY_BASE: i16 = 3;
const CHA_BONUS_SPELLS_BRACKET_AT_17: i16 = 2;
const SORCERER_LEVEL_1_SPELLS_KNOWN_EXPECTED: i16 =
    SORCERER_LEVEL_1_SPELLS_KNOWN_BASE + CHA_BONUS_SPELLS_BRACKET_AT_17; // 6
const SORCERER_LEVEL_1_SPELLS_PER_DAY_EXPECTED: i16 =
    SORCERER_LEVEL_1_SPELLS_PER_DAY_BASE + CHA_BONUS_SPELLS_BRACKET_AT_17; // 5
const SORCERER_LEVEL_1_SPELL_SAVE_DC_EXPECTED: i16 = 10 + 1 + 3; // 10 + spell level + CHA mod (CHA 17 -> +3) = 14

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

fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}

// ----- Bloodline selection + level-1 bloodline power become direct evidence -----

#[test]
fn sorcerer_level1_emits_arcane_bloodline_power_as_direct_evidence() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The chosen Arcane bloodline + its level-1 power (Arcane Bond) must surface as
    // direct computed evidence: a recognition explanation carrying the named power
    // and +0 mechanical value (the power itself is a selection, not a numeric bonus
    // at level 1).
    let power = explanation(&computation, BLOODLINE_POWER_EXPLANATION_ID);
    assert_eq!(
        power.value, 0,
        "Arcane bloodline level-1 power (Arcane Bond) is a selection, not a numeric bonus"
    );
    assert!(
        power.detail.contains("arcane"),
        "bloodline power explanation must name the Arcane bloodline: {}",
        power.detail
    );
    assert!(
        power.detail.contains("arcane_bond")
            || power.detail.to_ascii_lowercase().contains("arcane bond"),
        "bloodline power explanation must name Arcane Bond as the level-1 power: {}",
        power.detail
    );

    // The bloodline burden diagnostic is no longer claim-blocking: the slice now
    // surfaces the bloodline selection + level-1 power as direct evidence, and the
    // remaining gap is named explicitly as level-2+ progression / arcana / bonus
    // feats/skills / cross-bloodline-power math, which stays claim-blocked but as
    // a single, distinct diagnostic rather than the original "unsupported" blanket.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.sorcerer.bloodline.unsupported" && d.claim_blocking),
        "the original blanket 'bloodline unsupported' diagnostic must not remain \
         claim-blocking once the slice implements bloodline selection + level-1 power"
    );
}

#[test]
fn sorcerer_level1_without_bloodline_selection_stays_claim_blocked_on_bloodline() {
    // No bloodline choice => the slice must not silently accept "no choice" as
    // computed evidence. The bloodline burden stays claim-blocking.
    let no_bloodline = SORCERER_FIXTURE.replace(
        "choice=choice:sorcerer_bloodline:bloodline:arcane\n",
        "",
    );
    let input = load(&no_bloodline);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, BLOODLINE_POWER_EXPLANATION_ID),
        "a Sorcerer with no bloodline selection must not surface the Arcane Bond power"
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_feature.sorcerer.bloodline.unsupported" && d.claim_blocking),
        "a Sorcerer with no bloodline selection must stay claim-blocked on the bloodline burden: {:?}",
        computation.diagnostics
    );
}

// ----- Spontaneous spell math becomes direct evidence, still bounded -----

#[test]
fn sorcerer_level1_spontaneous_spells_known_is_computed_from_cha_and_level() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Sorcerer level 1 spells known = 4 (Sorcerer table) + 2 (CHA 17 bonus bracket)
    // = 6. The slice computes this directly.
    let known = explanation(&computation, SPONTANEOUS_KNOWN_EXPLANATION_ID);
    assert_eq!(
        known.value, SORCERER_LEVEL_1_SPELLS_KNOWN_EXPECTED,
        "Sorcerer level-1 spells known must be {} (4 base + 2 from CHA 17)",
        SORCERER_LEVEL_1_SPELLS_KNOWN_EXPECTED
    );
    assert!(
        known.detail.contains("spells known"),
        "spells-known explanation must name the magnitude: {}",
        known.detail
    );
    assert!(
        known.detail.contains("charisma") || known.detail.contains("CHA"),
        "spells-known explanation must cite the CHA bonus bracket: {}",
        known.detail
    );
}

#[test]
fn sorcerer_level1_spontaneous_spells_per_day_is_computed_from_cha_and_level() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Sorcerer level 1 spells per day = 3 first-level (Sorcerer table) + 2 bonus
    // slots from CHA 17 bracket = 5 first-level slots.
    let slots = explanation(&computation, SPONTANEOUS_SLOTS_EXPLANATION_ID);
    assert_eq!(
        slots.value, SORCERER_LEVEL_1_SPELLS_PER_DAY_EXPECTED,
        "Sorcerer level-1 spells per day must be {} (3 base + 2 from CHA 17)",
        SORCERER_LEVEL_1_SPELLS_PER_DAY_EXPECTED
    );
    assert!(
        slots.detail.contains("first") || slots.detail.contains("1st"),
        "spells-per-day explanation must name the 1st-level slot level: {}",
        slots.detail
    );
    assert!(
        slots.detail.contains("charisma") || slots.detail.contains("CHA"),
        "spells-per-day explanation must cite the CHA bonus-spells bracket: {}",
        slots.detail
    );
}

#[test]
fn sorcerer_level1_spell_save_dc_is_computed_from_cha_and_spell_level() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // DC = 10 + spell level + CHA modifier. CHA 17 -> +3, 1st-level spell -> DC 14.
    let dc = explanation(&computation, SPONTANEOUS_DC_EXPLANATION_ID);
    assert_eq!(
        dc.value, SORCERER_LEVEL_1_SPELL_SAVE_DC_EXPECTED,
        "Sorcerer level-1 spell save DC must be {} (10 + 1 spell level + 3 CHA mod)",
        SORCERER_LEVEL_1_SPELL_SAVE_DC_EXPECTED
    );
    assert!(
        dc.detail.contains("DC") || dc.detail.contains("save"),
        "spell-save-DC explanation must name the DC / save construct: {}",
        dc.detail
    );
    assert!(
        dc.detail.contains("charisma") || dc.detail.contains("CHA"),
        "spell-save-DC explanation must cite the CHA modifier driver: {}",
        dc.detail
    );
}

// ----- The integrated posture is still blocked on the bounded remaining gap -----

#[test]
fn sorcerer_level1_integrated_posture_remains_blocked_on_level_2_plus_progression() {
    let input = load(SORCERER_FIXTURE);

    // The bloodline + spontaneous math is direct evidence, but the bounded slice
    // still claim-blocks the integrated posture on the level-2+ progression gap:
    // bloodline arcana (level-3+ power), bonus spells/feats/skills from bloodline
    // beyond the level-1 power, school choice, metamagic, sorcery points, and
    // Sorcerer level-2+ class progression are all out of scope for this slice.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Blocked,
        "the integrated posture must remain Blocked; the slice proves only the \
         bounded level-1 bloodline + spontaneous math, not level-2+ progression"
    );
}

#[test]
fn sorcerer_level1_named_remaining_gap_diagnostic_distinguishes_from_pre_slice() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The pre-slice spontaneous-posture burden diagnostic must not remain
    // claim-blocking: the slice now proves the spontaneous math directly.
    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.sorcerer.spontaneous.unsupported" && d.claim_blocking),
        "the pre-slice 'spontaneous unsupported' diagnostic must not remain \
         claim-blocking once the slice computes the spontaneous spell math"
    );

    // The post-slice remaining-gap diagnostic is a single, distinct diagnostic
    // naming the level-2+ progression / broader spell-support gap.
    let remaining_gap: Vec<_> = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.contains("sorcerer"))
        .collect();
    assert_eq!(
        remaining_gap.len(),
        1,
        "the bounded slice must leave exactly one claim-blocking Sorcerer diagnostic, \
         naming only the level-2+ progression / broader spell-support gap: {:?}",
        computation.diagnostics
    );
    let gap = remaining_gap[0];
    assert!(
        gap.id.contains("progression") || gap.id.contains("level_2") || gap.id.contains("level"),
        "the remaining-gap diagnostic must name the level-2+ progression gap: {}",
        gap.id
    );
    assert!(
        gap.message.contains("level")
            && (gap.message.contains("2")
                || gap.message.contains("3")
                || gap.message.contains("arcana")
                || gap.message.contains("metamagic")),
        "the remaining-gap diagnostic message must name the bounded out-of-scope surface: {}",
        gap.message
    );
}

// ----- Negative controls -----

#[test]
fn fighter_does_not_gain_sorcerer_bloodline_or_spontaneous_evidence() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    for id in [
        BLOODLINE_POWER_EXPLANATION_ID,
        SPONTANEOUS_KNOWN_EXPLANATION_ID,
        SPONTANEOUS_SLOTS_EXPLANATION_ID,
        SPONTANEOUS_DC_EXPLANATION_ID,
    ] {
        assert!(
            !has_explanation(&fighter_computation, id),
            "Fighter must not surface {id}: {:?}",
            fighter_computation.explanations
        );
    }
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("sorcerer")),
        "Fighter must not surface any sorcerer diagnostic: {:?}",
        fighter_computation.diagnostics
    );
}

#[test]
fn sorcerer_level_2_is_not_promoted_by_this_slice() {
    // Level-2 Sorcerer must not gain the bounded level-1 bloodline + spontaneous
    // recognition: the slice proves only level 1, and the remaining-gap diagnostic
    // claim-blocks level 2+ in this slice.
    let level_2 = SORCERER_FIXTURE.replace("class:sorcerer:1", "class:sorcerer:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, BLOODLINE_POWER_EXPLANATION_ID),
        "level-2 Sorcerer must not gain the bounded level-1 bloodline power recognition"
    );
    assert!(
        !has_explanation(&computation, SPONTANEOUS_KNOWN_EXPLANATION_ID)
            && !has_explanation(&computation, SPONTANEOUS_SLOTS_EXPLANATION_ID)
            && !has_explanation(&computation, SPONTANEOUS_DC_EXPLANATION_ID),
        "level-2 Sorcerer must not gain the bounded level-1 spontaneous spell math"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Sorcerer must stay claim-blocked in this slice"
    );
}

// ----- Matrix reclassification -----

#[test]
fn matrix_sorcerer_row_is_partial_computed_and_names_only_level_2_plus_gap() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .expect("sorcerer row must exist");

    // The slice lifts the non-spell burden to PARTIAL: the bloodline selection +
    // level-1 bloodline power and the spontaneous known-spell / slot posture are
    // proven, but the bounded remaining gap (level-2+ progression, bloodline
    // arcana, broader spell-support surface) is named in the blocker note.
    assert_eq!(
        sorcerer.support_state,
        SupportState::Partial,
        "sorcerer row must move from Blocked to Partial: this slice proves the \
         bounded bloodline + spontaneous math but names level-2+ as the only gap"
    );
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::Computed);
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
    assert!(
        sorcerer.grounding_ref.contains("sd13_sorcerer_bloodline_and_spontaneous_slice"),
        "sorcerer row must cite the SD13-E4-F8 slice proof surface: {}",
        sorcerer.grounding_ref
    );

    let note = sorcerer.blocker_or_lossiness_note;
    assert!(!note.is_empty(), "sorcerer Partial row must carry a note");
    // The note must name the proven partial surface and the remaining gap.
    for token in ["bloodline", "spontaneous", "level"] {
        assert!(
            note.contains(token),
            "sorcerer Partial note must name '{token}': {note}"
        );
    }
    // The pre-slice language "not implemented" / "is not computed" must not
    // survive in the note for the now-proven surfaces.
    assert!(
        !note.contains("not implemented") && !note.contains("is not computed"),
        "Partial-row note must not describe the now-proven surfaces as un-implemented: {note}"
    );
}

#[test]
fn matrix_keeps_other_class_rows_unchanged_by_this_slice() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    // Bard / Wizard stay Unverified/Observed, Paladin / Ranger stay Blocked/Computed.
    for row_id in [
        "class.bard.progression_and_spell_burden",
        "class.wizard.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Unverified,
            "row {row_id} must stay Unverified after the Sorcerer slice"
        );
        assert_eq!(
            row.evidence_tier,
            EvidenceTier::Observed,
            "row {row_id} must stay Observed after the Sorcerer slice"
        );
    }
    for row_id in [
        "class.paladin.hybrid_chassis_and_spell_burden",
        "class.ranger.hybrid_chassis_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Blocked,
            "hybrid row {row_id} must stay Blocked after the Sorcerer slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }
}

#[test]
fn matrix_does_not_promote_any_row_to_supported_or_lossy() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    assert!(
        !matrix
            .rows
            .iter()
            .any(|r| r.support_state == SupportState::Supported
                || r.support_state == SupportState::Lossy),
        "the Sorcerer bloodline + spontaneous slice must not promote any row to Supported or Lossy"
    );
}