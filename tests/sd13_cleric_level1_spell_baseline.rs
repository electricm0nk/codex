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

const CLERIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.cleric";
const CHANNEL_ENERGY_DICE_ID: &str = "class_chassis.cleric.channel_energy_dice";
const CHANNEL_ENERGY_USES_PER_DAY_ID: &str = "class_chassis.cleric.channel_energy_uses_per_day";
const DOMAIN_CHOICE_ID: &str = "class_chassis.cleric.domain_choice";
const DOMAIN_SPELL_SLOT_ID: &str = "class_chassis.cleric.domain_spell_slot";
// (v0.6 alpha swarm, risks item 8, Good domain closure) the old flat
// "class_feature.cleric.domain_powers.unsupported" no longer fires for
// this Good+Healing fixture -- Rebuke Death (Healing domain) is the real,
// still-claim-blocking equivalent for this fixture's own domain selection
// (Touch of Good, Good domain, can now genuinely close).
const DOMAIN_BLOCKER_ID: &str = "class_feature.cleric.healing_domain.rebuke_death.unsupported";
const PREPARED_BLOCKER_ID: &str = "class_spell.cleric.prepared_divine.unsupported";

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

/// (v0.6 alpha swarm, risks item 8) PREPARED_BLOCKER_ID is no longer
/// unconditional -- a bare fixture with zero prepared spells is a genuinely
/// valid posture, so the blocker correctly does not fire. If it's absent,
/// confirm no spell is fabricated merely because the blocker stopped firing.
fn assert_prepared_blocker_state_is_valid_or_blocking(computation: &PilotBaseChassisComputation) {
    match computation
        .diagnostics
        .iter()
        .find(|d| d.id == PREPARED_BLOCKER_ID)
    {
        Some(blocker) => assert!(
            blocker.claim_blocking,
            "if the blocker fires, it must be claim-blocking"
        ),
        None => {
            let prepared_count = computation
                .explanations
                .iter()
                .find(|e| e.id == "class_spell.cleric.daily_preparation")
                .map(|e| e.value)
                .unwrap_or(-1);
            assert_eq!(
                prepared_count, 0,
                "no spells are fabricated merely because the blocker stopped firing: {:?}",
                computation.diagnostics
            );
        }
    }
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
        "cleric level 1's real 3/4 base attack bonus formula floors to 0 (floor(3/4 * 1) = 0)"
    );
    // (v0.6 alpha swarm, risks item 8) Cleric is now recognized by
    // table_class_id, so the generic class-chassis base-attack-bonus
    // explanation IS surfaced (unlike the earlier unsupported-chassis state);
    // the value still floors to 0 at level 1, only presence changed.
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "cleric is now recognized by table_class_id and must surface its base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (WIS 17 -> +3).
    assert_eq!(computation.ability_modifiers.wisdom, 4);
}

#[test]
fn cleric_level1_fabricates_no_spell_math() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID
                // (v0.6 alpha swarm, risks item 8) the bare fixture has zero
                // prepared spells, a genuinely valid posture, so the real
                // daily-preparation count (honestly 0) and the real base/total
                // slot budget it's validated against now surface too -- all
                // grounded, non-fabricated records once the posture is valid
                // (mirrors ground_cleric_prepared_spells and its base-table
                // lookups exactly, both class_chassis.cleric.* and
                // class_spell.cleric.* namespaces).
                || explanation.id.starts_with("class_chassis.cleric.")
                || explanation.id.starts_with("class_spell.cleric.")
                // SD-34 decisions.md section 18: widened BY CONSTRUCTION, not narrowed --
                // class_feature_grant_consumer now emits real, citation-backed corpus_record
                // ids for Cleric (previously wholesale-excluded); this shape carve-out admits
                // them without weakening the substring check for anything else.
                || explanation.id.starts_with("class_feature.cleric.corpus_record.")
                || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition and the honest \
             grounded cleric class-chassis/class-spell records: {explanation:?}"
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
    assert_prepared_blocker_state_is_valid_or_blocking(&computation);
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
    assert_prepared_blocker_state_is_valid_or_blocking(&computation);
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
    // (v0.6 alpha swarm, risks item 8, Good domain closure) with NO domain
    // selection at all, this falls into the original catch-all branch, so
    // the old flat diagnostic still fires here (unlike the Good+Healing
    // fixture, where it's replaced by the narrowed rebuke_death id).
    claim_blocking(&computation, "class_feature.cleric.domain_powers.unsupported");
    assert_prepared_blocker_state_is_valid_or_blocking(&computation);
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn cleric_level1_stays_blocked_on_domain_powers_burden() {
    let input = load(CLERIC_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // (v0.6 alpha swarm, risks item 8, Good domain closure) Touch of Good
    // (Good domain) can now genuinely close; Rebuke Death (Healing domain)
    // is the real, still-claim-blocking burden for this Good+Healing
    // fixture, under its own narrowed id.
    let domain = claim_blocking(&computation, DOMAIN_BLOCKER_ID);
    assert!(
        domain.message.contains("Rebuke Death"),
        "rebuke death blocker must name the concrete unimplemented granted power: {}",
        domain.message
    );
    assert!(
        domain.message.contains("Touch of Good"),
        "rebuke death blocker must contrast against the closed Touch of Good burden: {}",
        domain.message
    );
    assert!(
        domain.message.contains("heal amount"),
        "rebuke death blocker must name the unproven heal amount: {}",
        domain.message
    );
    assert!(
        !domain.message.contains("channel energy"),
        "rebuke death blocker must not name channel energy, which is grounded: {}",
        domain.message
    );

    // The separate, non-blocking domain spell-list-contents note still fires.
    let spell_list_note = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.cleric.domain_spell_list_contents.unmodeled")
        .expect("the domain spell-list-contents note must still fire");
    assert!(!spell_list_note.claim_blocking);
    assert!(spell_list_note.message.contains("spell-list"));
}

#[test]
fn cleric_level1_stays_blocked_on_prepared_divine_spell_posture_burden() {
    // (v0.6 alpha swarm, risks item 8) PREPARED_BLOCKER_ID is no longer
    // unconditional -- the bare fixture is a genuinely valid posture, so this
    // test (whose whole purpose is proving the blocker still fires) now needs
    // a real violation: "Aid" is a real PF1 Core Rulebook 2nd-level cleric
    // spell, not yet accessible at cleric level 1 (access ceiling 1st level).
    let mut input = load(CLERIC_FIXTURE);
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Aid".to_owned(),
        source_class_id: "class:cleric".to_owned(),
        acquisition_mode: AcquisitionMode::Prepared,
    });
    let computation = compute_pilot_base_chassis(&input);

    let prepared = claim_blocking(&computation, PREPARED_BLOCKER_ID);
    assert!(
        prepared.message.contains("prepared") && prepared.message.contains("Aid"),
        "cleric prepared divine spell blocker must name the prepared-spell burden and the \
         violating spell: {}",
        prepared.message
    );
    assert!(
        prepared.message.contains("not yet accessible"),
        "cleric prepared divine spell blocker must explain why the spell is not yet accessible: {}",
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
fn cleric_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_cleric_level4_progression.rs) widened the level-range gate to
    // level 4 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/
    // Sorcerer/Wizard level-range gate idiom) and confirmed the Good domain's
    // Touch of Good sacred bonus genuinely changes at level 4, while Channel
    // Energy's die count and the domain spell slot count both stay unchanged;
    // this negative control is superseded, not violated — pin the new truth
    // here too so this file stays internally consistent.
    let level_4 = CLERIC_FIXTURE.replace("class:cleric:1", "class:cleric:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-4 Cleric is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, CHANNEL_ENERGY_DICE_ID)
            && has_explanation(&computation, CHANNEL_ENERGY_USES_PER_DAY_ID),
        "level-4 Cleric is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, DOMAIN_CHOICE_ID)
            && has_explanation(&computation, DOMAIN_SPELL_SLOT_ID),
        "level-4 Cleric is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-4 Cleric must stay claim-blocked in this slice"
    );
}

#[test]
fn cleric_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_cleric_level5_progression.rs) widened the level-range gate to
    // level 5 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Bard/Druid/
    // Sorcerer/Wizard/Ranger level-range gate idiom) and confirmed Channel
    // Energy's die count and the domain spell slot count both change for real
    // at level 5; this negative control is superseded, not violated — pin the
    // new truth here too so this file stays internally consistent.
    let level_5 = CLERIC_FIXTURE.replace("class:cleric:1", "class:cleric:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-5 Cleric is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, CHANNEL_ENERGY_DICE_ID)
            && has_explanation(&computation, CHANNEL_ENERGY_USES_PER_DAY_ID),
        "level-5 Cleric is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, DOMAIN_CHOICE_ID)
            && has_explanation(&computation, DOMAIN_SPELL_SLOT_ID),
        "level-5 Cleric is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-5 Cleric must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix row is promoted inline (Ranger Track precedent) -----

#[test]
fn matrix_cleric_row_is_partial_computed_and_names_grounded_and_remaining_burdens() {
    let matrix = seeded_current_truth();
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric row must exist");

    // NOTE: this test's name reflects this row's state as of the SD13-E4
    // slice this file proves. Later promoted to Supported/ProductVisible by
    // SD-19's Class Progression Catalog browser UI-surfacing work
    // (2026-07-16).
    assert_eq!(cleric.support_state, SupportState::Supported);
    assert_ne!(cleric.support_state, SupportState::Blocked);
    assert_eq!(cleric.evidence_tier, EvidenceTier::ProductVisible);
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
fn matrix_preserves_wizard_hybrid_blocked_computed_and_sorcerer_bard_supported_truth() {
    let matrix = seeded_current_truth();

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .expect("paladin row must exist");
    assert_eq!(
        paladin.support_state,
        SupportState::Supported,
        "paladin row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);

    // Sorcerer was later promoted to Partial/Computed by its own SD13-E4
    // decomposition slice (Eschew Materials grounded for real), then to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-17).
    let sorcerer = matrix
        .row("class.sorcerer.progression_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.sorcerer.progression_and_spell_burden must exist"));
    assert_eq!(
        sorcerer.support_state,
        SupportState::Supported,
        "sorcerer row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(sorcerer.evidence_tier, EvidenceTier::ProductVisible);

    // Bard was later promoted to Partial/Computed by its own SD13-E4
    // decomposition slice (Bardic Knowledge grounded for real), then to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-16).
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.bard.progression_and_spell_burden must exist"));
    assert_eq!(
        bard.support_state,
        SupportState::Supported,
        "bard row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(bard.evidence_tier, EvidenceTier::ProductVisible);

    // Wizard was later promoted to Partial/Computed by its own SD13-E4 Scribe
    // Scroll decomposition slice, then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17).
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Supported,
        "wizard row must keep its later-accepted Supported posture after the Cleric slice"
    );
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);

    // Ranger was later promoted to Partial/Computed by its own SD13-E3 Ranger
    // decomposition slice (Track grounded for real).
    let ranger = matrix
        .row("class.ranger.hybrid_chassis_and_spell_burden")
        .expect("ranger row must exist");
    assert_eq!(
        ranger.support_state,
        SupportState::Supported,
        "ranger row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(ranger.evidence_tier, EvidenceTier::ProductVisible);
}

#[test]
fn matrix_does_not_promote_any_row_to_supported_or_lossy() {
    let matrix = seeded_current_truth();
    assert!(
        !matrix
            .rows
            .iter()
            // school.abjuration/illusion.spell_reachability were later promoted to
            // Supported/Product-visible by SD-19's operator-driven UI-surfacing work
            // (2026-07-16) -- excluded here, not an unintended promotion by this slice.
            .any(|r| (r.support_state == SupportState::Supported
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
                && r.row_id != "class.paladin.hybrid_chassis_and_spell_burden"
                && r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"
                && r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"
                && r.row_id != "equipment.equipmods.equipment_reachability")
                || r.support_state == SupportState::Lossy),
        "the Cleric slice must not promote any row to Supported or Lossy"
    );
}
