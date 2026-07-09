//! SD13-E4 Cleric level-1 prepared divine spell-burden baseline proof.
//!
//! Proves the fourth honest SD13-E4 spell-bearing slice (after Sorcerer, Bard, and
//! Wizard): the live rules-core surface ingests a deterministic Human `class:cleric:1`
//! input, leaves direct computed evidence that recognizes the Cleric level-1 prepared
//! divine spell-bearing class identity rather than treating it as an undocumented
//! packet placeholder, and grounds its Channel Energy class feature for real (PF1 Core
//! Rulebook: `ceil(cleric level / 2)` d6, minimum 1d6; usable `3 + Charisma modifier`
//! times per day). The SD13-E5 Cleric domain slice additionally grounds the domain
//! choice seam (the two canonical fixture selections `choice:cleric_domain ->
//! domain:good` and `choice:cleric_domain -> domain:healing`, surfaced as an explicit
//! choice seam carrying no mechanical value, mirroring the Fighter bonus-feat
//! choice-slot seam) and the flat domain spell slot count (PF1 Core Rulebook Domains:
//! one domain spell slot per level of cleric spells she can cast, 1st and up — exactly
//! one 1st-level domain slot at level 1). It yet stays explicitly claim-blocked with
//! two distinct diagnostics: one for the domain powers burden (the granted powers of
//! the chosen domains — Good: Touch of Good; Healing: Rebuke Death, each 3 + Wisdom
//! modifier uses per day — and the domain spell-list contents) and one for the prepared
//! divine spell posture burden (spells prepared from the full Cleric list, spontaneous
//! cure/inflict conversion, spell slots per day, bonus spells from a high Wisdom, spell
//! save DCs). The slice stays single-class, level-1-only, Human-only, and grounds no
//! domain power execution, no domain spell-list contents, no channel energy save DC or
//! damage/healing resolution, and no spell math.
//!
//! The in-source carrier keeps the Cleric row at `Partial` / `Computed` /
//! `RefreshableFromLiveProof`, grounded on this same test file (grounding_ref
//! unchanged), with a blocker note naming Channel Energy, the domain choice seam, and
//! the domain spell slot count as grounded and the two remaining burdens as still
//! unproven.
//!
//! It is intentionally not a spell engine. It fabricates no domain spell-list contents,
//! no domain powers, no channel energy save DC or damage/healing resolution, no
//! spellbook content, no spells prepared, no general spell slots per day, no spell DCs,
//! no bonus spells, and it grounds no Cleric level 2+.

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

const CLERIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.cleric";
const CHANNEL_ENERGY_DICE_ID: &str = "class_chassis.cleric.channel_energy_dice";
const CHANNEL_ENERGY_USES_PER_DAY_ID: &str = "class_chassis.cleric.channel_energy_uses_per_day";
const DOMAIN_CHOICE_ID: &str = "class_chassis.cleric.domain_choice";
const DOMAIN_SPELL_SLOT_ID: &str = "class_chassis.cleric.domain_spell_slot";
const DOMAIN_BLOCKER_ID: &str = "class_feature.cleric.domain_powers.unsupported";
const PREPARED_BLOCKER_ID: &str = "class_spell.cleric.prepared_divine.unsupported";

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

// ----- Direct runtime evidence: the prepared divine spell-bearing identity is acknowledged -----

#[test]
fn cleric_level1_leaves_direct_prepared_divine_spell_baseline_recognition_evidence() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:cleric") && recognition.detail.contains("level 1"),
        "cleric recognition must name the class:cleric:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "cleric recognition must name the spell-bearing identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("divine"),
        "cleric recognition must distinguish the divine identity from the arcane Sorcerer/Wizard/Bard identities: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("prepared"),
        "cleric recognition must distinguish the prepared divine identity from spontaneous arcane casters: {}",
        recognition.detail
    );

    assert_eq!(
        recognition.value, 0,
        "cleric prepared divine spell baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "prepared divine spell baseline must not fabricate a base attack bonus"
    );
    assert!(
        !has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "prepared divine spell baseline must not surface a supported Fighter base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (WIS 17 -> +3).
    assert_eq!(computation.ability_modifiers.wisdom, 3);
}

#[test]
fn cleric_level1_fabricates_no_spell_math() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID
                || explanation.id == CHANNEL_ENERGY_DICE_ID
                || explanation.id == CHANNEL_ENERGY_USES_PER_DAY_ID
                || explanation.id == DOMAIN_SPELL_SLOT_ID
                || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition, the grounded \
             Channel Energy pillars, and the flat domain spell slot count: {explanation:?}"
        );
    }
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Grounded for real: Channel Energy die count and uses per day -----

#[test]
fn cleric_level1_grounds_channel_energy_dice_and_uses_per_day() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 Core Rulebook Channel Energy: ceil(cleric level / 2) d6, minimum 1d6.
    // At level 1: ceil(1 / 2) = 1.
    let dice = explanation(&computation, CHANNEL_ENERGY_DICE_ID);
    assert_eq!(
        dice.value, 1,
        "cleric level 1 Channel Energy must ground exactly 1d6"
    );
    assert!(
        dice.detail.contains("d6") && dice.detail.contains("Channel Energy"),
        "channel energy dice explanation must name the d6 die count and Channel Energy: {}",
        dice.detail
    );

    // PF1 Core Rulebook Channel Energy: usable 3 + Charisma modifier times per day.
    // Fixture Charisma is 14 -> modifier +2 -> 3 + 2 = 5.
    let uses = explanation(&computation, CHANNEL_ENERGY_USES_PER_DAY_ID);
    assert_eq!(
        uses.value, 5,
        "cleric level 1 with CHA 14 (+2) must ground 3 + 2 = 5 channel energy uses per day"
    );
    assert!(
        uses.detail.contains("Charisma") && uses.detail.contains("Channel Energy"),
        "channel energy uses-per-day explanation must name Charisma and Channel Energy: {}",
        uses.detail
    );

    // Grounding Channel Energy must not silently fabricate domain spell math or
    // the prepared-spell posture: no domain-spell or prepared-spell explanation
    // is allowed, and both remaining named burdens must still be present and
    // claim-blocking.
    assert!(
        !has_explanation(&computation, "class_feature.cleric.domain_spells"),
        "grounding Channel Energy must not fabricate domain spell math"
    );
    assert!(
        !has_explanation(&computation, "class_spell.cleric.prepared_divine"),
        "grounding Channel Energy must not fabricate the prepared divine spell posture"
    );
    claim_blocking(&computation, DOMAIN_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

// ----- Grounded for real: domain choice seam and flat domain spell slot count -----

#[test]
fn cleric_level1_grounds_domain_choice_seam_and_domain_spell_slot_count() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The two canonical fixture domain selections are surfaced as an explicit
    // choice seam only (mirroring the Fighter bonus-feat choice-slot seam): a
    // recognition record carrying no fabricated mechanical value.
    let choice = explanation(&computation, DOMAIN_CHOICE_ID);
    assert_eq!(
        choice.value, 0,
        "the cleric domain choice seam is a recognition record and must carry no mechanical value (+0)"
    );
    assert!(
        choice.detail.contains("choice:cleric_domain")
            && choice.detail.contains("domain:good")
            && choice.detail.contains("domain:healing"),
        "the domain choice seam must name the choice set and both canonical selections: {}",
        choice.detail
    );

    // PF1 Core Rulebook Domains: a cleric gains one domain spell slot per level
    // of cleric spells she can cast, 1st and up. At level 1 she casts only
    // 1st-level cleric spells, so exactly one 1st-level domain slot is granted.
    let slot = explanation(&computation, DOMAIN_SPELL_SLOT_ID);
    assert_eq!(
        slot.value, 1,
        "cleric level 1 must ground exactly one domain spell slot"
    );
    assert!(
        slot.detail.contains("domain spell slot"),
        "the domain spell slot explanation must name the domain spell slot: {}",
        slot.detail
    );
    assert!(
        slot.detail.contains("content"),
        "the domain spell slot explanation must state plainly that the slot's contents are not grounded: {}",
        slot.detail
    );

    // Grounding the choice seam and the flat slot count must not silently
    // fabricate the domain powers or any domain spell-list content, and both
    // remaining named burdens must still be present and claim-blocking.
    assert!(
        !has_explanation(&computation, "class_feature.cleric.domain_spells"),
        "grounding the domain seam must not fabricate domain spell-list contents"
    );
    assert!(
        !has_explanation(&computation, "class_feature.cleric.domain_powers"),
        "grounding the domain seam must not fabricate domain power math"
    );
    claim_blocking(&computation, DOMAIN_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

#[test]
fn cleric_level1_without_domain_selections_does_not_fabricate_the_choice_seam() {
    // Absent choice slots are not fabricated (Fighter choice-slot precedent),
    // but the flat domain spell slot count is class-chassis math independent of
    // which domains were chosen, so it still grounds.
    let stripped = CLERIC_FIXTURE
        .replace("choice=choice:cleric_domain:domain:good\n", "")
        .replace("choice=choice:cleric_domain:domain:healing\n", "");
    let input = load(&stripped);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, DOMAIN_CHOICE_ID),
        "an absent domain choice slot must not be fabricated as a recognized selection"
    );
    assert!(
        has_explanation(&computation, DOMAIN_SPELL_SLOT_ID),
        "the flat domain spell slot count does not depend on which domains were chosen"
    );
    claim_blocking(&computation, DOMAIN_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn cleric_level1_stays_blocked_on_domain_powers_burden() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let domain = claim_blocking(&computation, DOMAIN_BLOCKER_ID);
    assert!(
        domain.message.contains("domain power"),
        "cleric domain blocker must name the domain powers burden: {}",
        domain.message
    );
    assert!(
        domain.message.contains("Touch of Good") && domain.message.contains("Rebuke Death"),
        "cleric domain powers blocker must name the concrete unimplemented granted powers: {}",
        domain.message
    );
    assert!(
        domain.message.contains("spell-list"),
        "cleric domain powers blocker must also name the unproven domain spell-list contents: {}",
        domain.message
    );
    assert!(
        !domain.message.contains("channel energy"),
        "cleric domain powers blocker must not name channel energy, which is grounded: {}",
        domain.message
    );
}

#[test]
fn cleric_level1_stays_blocked_on_prepared_divine_spell_posture_burden() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let prepared = claim_blocking(&computation, PREPARED_BLOCKER_ID);
    assert!(
        prepared.message.contains("prepared") && prepared.message.contains("spell slot"),
        "cleric prepared divine spell blocker must name the prepared / spell slots burden: {}",
        prepared.message
    );

    assert_ne!(
        DOMAIN_BLOCKER_ID, PREPARED_BLOCKER_ID,
        "domain and prepared burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("cleric"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "cleric must leave exactly two class-specific claim-blocking diagnostics: {:?}",
        computation.diagnostics
    );
}

#[test]
fn cleric_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(CLERIC_FIXTURE);

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked prepared divine spell baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the prepared divine spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "prepared divine spell baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "prepared divine spell baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "prepared divine spell baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: the cleric baseline must not leak onto other classes/levels -----

#[test]
fn fighter_sorcerer_and_wizard_do_not_gain_cleric_recognition() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a cleric prepared-divine-spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("cleric")),
        "Fighter must not surface cleric burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    let sorcerer_fixture = CLERIC_FIXTURE.replace("class:cleric:1", "class:sorcerer:1");
    let sorcerer = load(&sorcerer_fixture);
    let sorcerer_computation = compute_pilot_base_chassis(&sorcerer);
    assert!(
        !has_explanation(&sorcerer_computation, RECOGNITION_ID)
            && !sorcerer_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("cleric")),
        "Sorcerer must not surface any cleric recognition or burden diagnostics: {:?}",
        sorcerer_computation.diagnostics
    );

    let wizard_fixture = CLERIC_FIXTURE.replace("class:cleric:1", "class:wizard:1");
    let wizard = load(&wizard_fixture);
    let wizard_computation = compute_pilot_base_chassis(&wizard);
    assert!(
        !has_explanation(&wizard_computation, RECOGNITION_ID)
            && !wizard_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("cleric")),
        "Wizard must not surface any cleric recognition or burden diagnostics: {:?}",
        wizard_computation.diagnostics
    );
}

#[test]
fn cleric_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_cleric_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk level-range
    // gate idiom) and confirmed every one of the formulas below extends to level
    // 2 unchanged; this negative control is superseded, not violated — pin the
    // new truth here too so this file stays internally consistent.
    let level_2 = CLERIC_FIXTURE.replace("class:cleric:1", "class:cleric:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-2 Cleric is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, CHANNEL_ENERGY_DICE_ID)
            && has_explanation(&computation, CHANNEL_ENERGY_USES_PER_DAY_ID),
        "level-2 Cleric is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, DOMAIN_CHOICE_ID)
            && has_explanation(&computation, DOMAIN_SPELL_SLOT_ID),
        "level-2 Cleric is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Cleric must stay claim-blocked in this slice"
    );
}

#[test]
fn cleric_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_cleric_level3_progression.rs) widened the level-range gate to
    // level 3 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/
    // Sorcerer/Wizard level-range gate idiom) and confirmed Channel Energy's die
    // count and the domain spell slot count both change for real at level 3;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_3 = CLERIC_FIXTURE.replace("class:cleric:1", "class:cleric:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-3 Cleric is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, CHANNEL_ENERGY_DICE_ID)
            && has_explanation(&computation, CHANNEL_ENERGY_USES_PER_DAY_ID),
        "level-3 Cleric is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, DOMAIN_CHOICE_ID)
            && has_explanation(&computation, DOMAIN_SPELL_SLOT_ID),
        "level-3 Cleric is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-3 Cleric must stay claim-blocked in this slice"
    );
}

#[test]
fn cleric_level_4_is_not_promoted() {
    let level_4 = CLERIC_FIXTURE.replace("class:cleric:1", "class:cleric:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        !has_explanation(&computation, RECOGNITION_ID),
        "level-4 Cleric must not gain the bounded prepared-divine-spell-baseline recognition record"
    );
    assert!(
        !has_explanation(&computation, CHANNEL_ENERGY_DICE_ID)
            && !has_explanation(&computation, CHANNEL_ENERGY_USES_PER_DAY_ID),
        "level-4 Cleric must not gain the bounded Channel Energy grounding"
    );
    assert!(
        !has_explanation(&computation, DOMAIN_CHOICE_ID)
            && !has_explanation(&computation, DOMAIN_SPELL_SLOT_ID),
        "level-4 Cleric must not gain the bounded domain choice seam or domain spell slot count"
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-4 Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix row is promoted inline (Ranger Track precedent) -----

#[test]
fn matrix_cleric_row_is_partial_computed_and_names_grounded_and_remaining_burdens() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric row must exist");

    assert_eq!(cleric.support_state, SupportState::Partial);
    assert_ne!(cleric.support_state, SupportState::Blocked);
    assert_ne!(cleric.support_state, SupportState::Supported);
    assert_eq!(cleric.evidence_tier, EvidenceTier::Computed);
    assert_eq!(cleric.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        cleric
            .grounding_ref
            .contains("sd13_cleric_level1_spell_baseline"),
        "carrier grounding_ref must cite this slice's proof surface (unchanged)"
    );
    assert!(
        cleric.blocker_or_lossiness_note.contains("Channel Energy")
            && cleric.blocker_or_lossiness_note.contains("domain choice")
            && cleric.blocker_or_lossiness_note.contains("domain spell slot")
            && cleric.blocker_or_lossiness_note.contains("prepared"),
        "cleric blocker note must name Channel Energy, the domain choice seam, and the domain \
         spell slot count as grounded, and the prepared divine spell burden as still unproven: {}",
        cleric.blocker_or_lossiness_note
    );
    assert!(
        cleric.blocker_or_lossiness_note.contains("Touch of Good")
            && cleric.blocker_or_lossiness_note.contains("Rebuke Death"),
        "cleric blocker note must name the still-unproven granted domain powers: {}",
        cleric.blocker_or_lossiness_note
    );
    assert!(
        cleric.next_required_uplift.contains("Cleric")
            && cleric.next_required_uplift.contains("domain power"),
        "cleric next uplift must point at the domain powers burden next: {}",
        cleric.next_required_uplift
    );
}

#[test]
fn matrix_preserves_sorcerer_bard_wizard_and_hybrid_blocked_computed_truth() {
    let matrix = seeded_sd13_e1_f1_current_truth();

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Partial,
        "paladin row must keep its later-accepted Partial posture after the Cleric slice"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::Computed);

    // Sorcerer and Bard were later promoted to Partial/Computed by their own
    // SD13-E4 decomposition slices (Eschew Materials, Bardic Knowledge).
    for row_id in [
        "class.sorcerer.progression_and_spell_burden",
        "class.bard.progression_and_spell_burden",
    ] {
        let row = matrix
            .row(row_id)
            .unwrap_or_else(|| panic!("row {row_id} must exist"));
        assert_eq!(
            row.support_state,
            SupportState::Partial,
            "row {row_id} must be Partial after its own SD13-E4 decomposition slice"
        );
        assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    }

    // Wizard was later promoted to Partial/Computed by its own SD13-E4 Scribe
    // Scroll decomposition slice.
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Partial,
        "wizard row must keep its later-accepted Partial posture after the Cleric slice"
    );
    assert_eq!(wizard.evidence_tier, EvidenceTier::Computed);

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Partial,
        "ranger row must keep its later-accepted Partial posture after the Cleric slice"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::Computed);
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
        "the Cleric slice must not promote any row to Supported or Lossy"
    );
}
