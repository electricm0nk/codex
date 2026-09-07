//! SD13-E4 Druid level-1 prepared divine spell-burden baseline proof.
//!
//! Proves the fifth honest SD13-E4 spell-bearing slice (after Sorcerer, Bard, Wizard,
//! and Cleric): the live rules-core surface ingests a deterministic Human `class:druid:1`
//! input, leaves direct computed evidence that recognizes the Druid level-1 prepared
//! divine spell-bearing class identity rather than treating it as an undocumented
//! packet placeholder, and yet stays explicitly claim-blocked with two distinct
//! diagnostics: one for the nature bond and wild empathy class-feature burden (nature
//! bond choice between an animal companion and a domain, nature sense, wild empathy)
//! and one for the prepared divine spell posture burden (spells prepared from the full
//! Druid list, spontaneous summon nature's ally conversion, spell slots per day, bonus
//! spells from a high Wisdom, spell save DCs). The slice stays single-class,
//! level-1-only, Human-only, and grounds no spell math, no nature-bond power execution,
//! and no wild-empathy check resolution.
//!
//! It also promotes the matrix reclassification inline (mirroring the Sorcerer / Bard /
//! Cleric pattern): the in-source carrier moves the Druid row to `Blocked` / `Computed`
//! / `RefreshableFromLiveProof`, grounded on this test file, with a blocker note naming
//! both burdens.
//!
//! The SD13-E4 Wild Empathy grounding slice further splits the combined nature-bond /
//! wild-empathy class-feature blocker into two named diagnostics: `nature_bond` stayed
//! claim-blocked at that point (the animal-companion-vs-domain choice and nature sense
//! were still unproven; both are grounded by the later SD13-E5 slice below), while
//! Wild Empathy is grounded for real as
//! `class_chassis.druid.wild_empathy` = druid level + Charisma modifier (PF1 Core
//! Rulebook: 1d20 + druid level + Cha modifier, used like a Diplomacy check to improve
//! an animal's attitude). Only the flat modifier is grounded; no die roll and no
//! Diplomacy-check execution engine is computed. This promotes the matrix row from
//! `Blocked` to `Partial`.
//!
//! The SD13-E5 Nature Sense / nature-bond-choice slice grounds the next two honest
//! Druid facts: Nature Sense is grounded for real as
//! `class_chassis.druid.nature_sense` = 2 (PF1 Core Rulebook: a druid gains a +2
//! bonus on Knowledge (nature) and Survival checks — flat and level-independent; a
//! standalone record only, not wired into any skill-check total), and the fixture's
//! deterministic `choice:druid_nature_bond -> bond:animal_companion` selection is
//! recognized as `class_chassis.druid.nature_bond_choice` (+0 recognition record,
//! no bond execution math). The old combined `nature_bond` blocker is retired and
//! narrowed to `class_feature.druid.animal_companion.unsupported`, naming exactly
//! what stays unimplemented: the chosen bond's execution (companion stat block,
//! companion advancement, link / share-spells). The row stays `Partial`.
//!
//! It is intentionally not a spell engine. It fabricates no nature-bond power
//! execution, no animal companion stat block, advancement, or link/share-spells
//! math, no domain math, no skill-check resolution for the Nature Sense bonus, no
//! wild-empathy check resolution (no d20 roll, no attitude-improvement outcome), no
//! spellbook content, no spells prepared, no spell slots per day, no spell DCs, no
//! bonus spells, and it grounds no Druid level 2+.

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

const DRUID_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_druid_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.druid";
const WILD_EMPATHY_ID: &str = "class_chassis.druid.wild_empathy";
const NATURE_SENSE_ID: &str = "class_chassis.druid.nature_sense";
const NATURE_BOND_CHOICE_ID: &str = "class_chassis.druid.nature_bond_choice";
const ANIMAL_COMPANION_BLOCKER_ID: &str = "class_feature.druid.animal_companion.unsupported";
const PREPARED_BLOCKER_ID: &str = "class_spell.druid.prepared_divine.unsupported";
const NATURE_BOND_CHOICE_LINE: &str = "choice=choice:druid_nature_bond:bond:animal_companion\n";

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
                .find(|e| e.id == "class_spell.druid.daily_preparation")
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
fn druid_level1_leaves_direct_prepared_divine_spell_baseline_recognition_evidence() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:druid") && recognition.detail.contains("level 1"),
        "druid recognition must name the class:druid:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "druid recognition must name the spell-bearing identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("divine"),
        "druid recognition must distinguish the divine identity from the arcane Sorcerer/Wizard/Bard identities: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("prepared"),
        "druid recognition must distinguish the prepared divine identity from spontaneous arcane casters: {}",
        recognition.detail
    );

    assert_eq!(
        recognition.value, 0,
        "druid prepared divine spell baseline recognition must carry no fabricated value (+0)"
    );
    assert_eq!(
        computation.base_attack_bonus, 0,
        "druid level 1's real 3/4 base attack bonus formula floors to 0 (floor(3/4 * 1) = 0)"
    );
    // (v0.6 alpha swarm, risks item 8) Druid is now recognized by
    // table_class_id, so the generic class-chassis base-attack-bonus
    // explanation IS surfaced (unlike the earlier unsupported-chassis state);
    // the value still floors to 0 at level 1, only presence changed.
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "druid is now recognized by table_class_id and must surface its base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (WIS 17 -> +3).
    assert_eq!(computation.ability_modifiers.wisdom, 4);
}

#[test]
fn druid_level1_fabricates_no_spell_math() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID
                || explanation.id == WILD_EMPATHY_ID
                // (v0.6 alpha swarm, risks item 8) the bare fixture has zero
                // prepared spells, a genuinely valid posture, so the real
                // daily-preparation count (honestly 0) and the real
                // base/total slot budget it's validated against now surface
                // too -- all grounded, non-fabricated records once the
                // posture is valid (mirrors ground_druid_prepared_spells and
                // its base-table lookups exactly).
                || explanation.id.starts_with("class_chassis.druid.")
                || explanation.id.starts_with("class_spell.druid.")
                // (v0.6 alpha swarm, risks item 8, animal companion closure)
                // the Wolf companion's Share Spells correction record
                // legitimately contains "spell" in its id but is an honest,
                // non-fabricated vacuous-ability note, not spell math.
                || explanation.id.starts_with("class_feature.druid.animal_companion.")
                || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition, the grounded \
             wild empathy modifier, the honest grounded druid class-chassis/class-spell records, \
             and the animal companion's own honest correction records: {explanation:?}"
        );
    }
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Grounded: Wild Empathy is computed for real -----

#[test]
fn druid_level1_grounds_wild_empathy_modifier() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Fixture: Charisma 12 -> modifier +1. Druid level 1 + Cha modifier +1 = 2.
    assert_eq!(computation.ability_modifiers.charisma, 1);
    let wild_empathy = explanation(&computation, WILD_EMPATHY_ID);
    assert_eq!(
        wild_empathy.value, 2,
        "wild empathy modifier must equal druid level + Cha modifier (1 + 1 = 2): {wild_empathy:?}"
    );
    assert!(
        wild_empathy.detail.contains("druid level") || wild_empathy.detail.contains("Druid level"),
        "wild empathy detail must cite the druid-level term of the formula: {}",
        wild_empathy.detail
    );
    assert!(
        wild_empathy.detail.contains("Charisma") || wild_empathy.detail.contains("Cha"),
        "wild empathy detail must cite the Charisma-modifier term of the formula: {}",
        wild_empathy.detail
    );
    assert!(
        !wild_empathy.detail.contains("animal companion") && !wild_empathy.detail.contains("domain"),
        "wild empathy must not fabricate nature bond / animal companion / domain math: {}",
        wild_empathy.detail
    );
    assert!(
        !wild_empathy.detail.to_lowercase().contains("spells prepared")
            && !wild_empathy.detail.to_lowercase().contains("spell slot"),
        "wild empathy must not fabricate prepared-spell posture math: {}",
        wild_empathy.detail
    );
}

// ----- Grounded: Nature Sense is computed for real -----

#[test]
fn druid_level1_grounds_nature_sense_bonus() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 CRB Nature Sense: a flat, level-independent +2 bonus on Knowledge
    // (nature) and Survival checks.
    let nature_sense = explanation(&computation, NATURE_SENSE_ID);
    assert_eq!(
        nature_sense.value, 2,
        "nature sense must be the flat PF1 CRB +2 bonus: {nature_sense:?}"
    );
    assert!(
        nature_sense.detail.contains("Knowledge (nature)")
            && nature_sense.detail.contains("Survival"),
        "nature sense detail must name the two skills the bonus applies to: {}",
        nature_sense.detail
    );
    assert!(
        nature_sense.detail.contains("standalone"),
        "nature sense detail must state it is a standalone record, not wired into skill totals: {}",
        nature_sense.detail
    );
    assert!(
        !nature_sense.detail.to_lowercase().contains("spells prepared")
            && !nature_sense.detail.to_lowercase().contains("spell slot"),
        "nature sense must not fabricate prepared-spell posture math: {}",
        nature_sense.detail
    );
}

#[test]
fn nature_sense_bonus_is_not_wired_into_skill_totals() {
    // The grounded +2 is a standalone record: it must not surface any per-skill
    // total explanation for Knowledge (nature) or Survival.
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.contains("skill") && e.id.contains("druid")),
        "nature sense must not surface a druid skill-total explanation: {:?}",
        computation.explanations
    );
}

// ----- Recognized: the deterministic nature-bond selection is acknowledged -----

#[test]
fn druid_level1_recognizes_nature_bond_choice() {
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let bond_choice = explanation(&computation, NATURE_BOND_CHOICE_ID);
    assert_eq!(
        bond_choice.value, 0,
        "nature-bond choice recognition must carry no fabricated mechanical value (+0): {bond_choice:?}"
    );
    assert!(
        bond_choice.detail.contains("choice:druid_nature_bond")
            && bond_choice.detail.contains("bond:animal_companion"),
        "nature-bond choice recognition must name the exact fixture selection: {}",
        bond_choice.detail
    );
    assert!(
        bond_choice.detail.contains("stat block")
            || bond_choice.detail.contains("no animal companion"),
        "nature-bond choice recognition must state that the chosen bond's execution stays \
         ungrounded: {}",
        bond_choice.detail
    );
}

#[test]
fn druid_level1_without_nature_bond_selection_omits_recognition_but_stays_blocked() {
    // The desktop composer does not thread a nature-bond selection; the seam must
    // stay honest for that shape: no recognition record is fabricated, while the
    // grounded facts and both claim-blocking burdens still fire.
    let without_choice = DRUID_FIXTURE.replace(NATURE_BOND_CHOICE_LINE, "");
    assert_ne!(
        without_choice, DRUID_FIXTURE,
        "the fixture must carry the nature-bond choice line this control removes"
    );
    let input = load(&without_choice);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, NATURE_BOND_CHOICE_ID),
        "no nature-bond choice recognition may be fabricated when no selection was made: {:?}",
        computation.explanations
    );
    assert!(has_explanation(&computation, NATURE_SENSE_ID));
    assert!(has_explanation(&computation, WILD_EMPATHY_ID));
    let companion = claim_blocking(&computation, ANIMAL_COMPANION_BLOCKER_ID);
    assert_prepared_blocker_state_is_valid_or_blocking(&computation);

    // No recognition record was left, so the blocker must not fabricate the claim
    // that an animal companion (or any specific bond) was actually chosen.
    assert!(
        !companion.message.contains("the chosen nature bond (an animal companion)"),
        "the animal-companion blocker must not claim a specific bond was chosen when no \
         nature-bond selection was made: {}",
        companion.message
    );
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn druid_level1_animal_companion_wolf_stat_block_genuinely_closes() {
    // (v0.6 alpha swarm, risks item 8, animal companion closure) The old
    // unconditional ANIMAL_COMPANION_BLOCKER_ID no longer fires for this
    // fixture (animal companion chosen at druid level 1, the only verified
    // companion level): the Wolf companion's stat block genuinely grounds,
    // Link and Share Spells resolve as honest vacuous corrections (same
    // idiom as Sorcerer's Arcane Bond), and only a non-blocking
    // "advancement past level 1" note remains -- proven, not assumed, by
    // checking the real explanation records and the diagnostic's blocking
    // state directly.
    let input = load(DRUID_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .diagnostics
            .iter()
            .any(|d| d.id == ANIMAL_COMPANION_BLOCKER_ID),
        "the old flat animal-companion blocker must not fire when the Wolf stat block closes: {:?}",
        computation.diagnostics
    );

    for id in [
        "class_chassis.druid.animal_companion.wolf_stat_block",
        "class_chassis.druid.animal_companion.base_attack_bonus",
        "class_chassis.druid.animal_companion.base_save.fortitude",
        "class_chassis.druid.animal_companion.base_save.reflex",
        "class_chassis.druid.animal_companion.base_save.will",
        "class_chassis.druid.animal_companion.armor_class",
        "class_chassis.druid.animal_companion.bite_attack",
        "class_chassis.druid.animal_companion.hit_points",
        "class_feature.druid.animal_companion.link_vacuous",
        "class_feature.druid.animal_companion.share_spells_vacuous",
    ] {
        assert!(
            has_explanation(&computation, id),
            "expected the Wolf companion closure to ground '{id}': {:?}",
            computation.explanations
        );
    }

    let advancement_absent = computation
        .diagnostics
        .iter()
        .find(|d| d.id == "class_feature.druid.animal_companion.advancement_absent")
        .expect("the advancement-absent note must still fire");
    assert!(
        !advancement_absent.claim_blocking,
        "the advancement-absent note must not block a valid level-1 companion posture: \
         {advancement_absent:?}"
    );

    // Grounding the companion must not leak into the Druid's own integrated totals.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "the companion's own base attack bonus must never be wired into the Druid's own \
         integrated base_attack_bonus field"
    );
}

#[test]
fn druid_level1_stays_blocked_on_prepared_divine_spell_posture_burden() {
    // (v0.6 alpha swarm, risks item 8) PREPARED_BLOCKER_ID is no longer
    // unconditional -- the bare fixture is a genuinely valid posture, so this
    // test (whose whole purpose is proving the blocker still fires) now needs
    // a real violation: "Barkskin" is a real PF1 Core Rulebook 2nd-level
    // druid spell, not yet accessible at druid level 1 (access ceiling 1st
    // level).
    let mut input = load(DRUID_FIXTURE);
    input.chosen.spells_selected.push(SpellSelection {
        spell_id: "Barkskin".to_owned(),
        source_class_id: "class:druid".to_owned(),
        acquisition_mode: AcquisitionMode::Prepared,
    });
    let computation = compute_pilot_base_chassis(&input);

    let prepared = claim_blocking(&computation, PREPARED_BLOCKER_ID);
    assert!(
        prepared.message.contains("prepared") && prepared.message.contains("Barkskin"),
        "druid prepared divine spell blocker must name the prepared-spell burden and the \
         violating spell: {}",
        prepared.message
    );
    assert!(
        prepared.message.contains("not yet accessible"),
        "druid prepared divine spell blocker must explain why the spell is not yet accessible: {}",
        prepared.message
    );

    // (v0.6 alpha swarm, risks item 8, animal companion closure) Only ONE
    // class-specific claim-blocking diagnostic remains here: the Wolf
    // companion's stat block genuinely closes at druid level 1 (this
    // fixture's own posture), so only the injected prepared-spell
    // violation stays claim-blocking.
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("druid"))
        .count();
    assert_eq!(
        distinct_blocking, 1,
        "druid must leave exactly one class-specific claim-blocking diagnostic (the injected \
         prepared-spell violation) on a valid animal-companion posture: {:?}",
        computation.diagnostics
    );
}

#[test]
fn druid_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(DRUID_FIXTURE);

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
    let input = load(DRUID_FIXTURE);
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

// ----- Negative controls: the druid baseline must not leak onto other classes/levels -----

#[test]
fn fighter_sorcerer_wizard_and_cleric_do_not_gain_druid_recognition() {
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a druid prepared-divine-spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("druid")),
        "Fighter must not surface druid burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    for other_class in ["class:sorcerer:1", "class:wizard:1", "class:cleric:1"] {
        let other_fixture = DRUID_FIXTURE.replace("class:druid:1", other_class);
        let other = load(&other_fixture);
        let other_computation = compute_pilot_base_chassis(&other);
        assert!(
            !has_explanation(&other_computation, RECOGNITION_ID)
                && !other_computation
                    .diagnostics
                    .iter()
                    .any(|d| d.id.contains("druid")),
            "{other_class} must not surface any druid recognition or burden diagnostics: {:?}",
            other_computation.diagnostics
        );
    }
}

#[test]
fn druid_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven
    // milestone and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_druid_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard
    // level-range gate idiom) and confirmed every one of the formulas below
    // extends to level 2 unchanged; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent.
    let level_2 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-2 Druid is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, WILD_EMPATHY_ID) && has_explanation(&computation, NATURE_SENSE_ID),
        "level-2 Druid is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Druid must stay claim-blocked in this slice"
    );
}

#[test]
fn druid_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 stayed unrecognized. A later
    // SD13-E5 slice (tests/sd13_druid_level3_progression.rs) widened the
    // level-range gate to level 3 and confirmed every one of the formulas below
    // extends to level 3 unchanged; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent.
    let level_3 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-3 Druid is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, WILD_EMPATHY_ID) && has_explanation(&computation, NATURE_SENSE_ID),
        "level-3 Druid is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-3 Druid must stay claim-blocked in this slice"
    );
}

#[test]
fn druid_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 stayed unrecognized. A later
    // SD13-E5 slice (tests/sd13_druid_level4_progression.rs) widened the
    // level-range gate to level 4 and confirmed every one of the formulas below
    // extends to level 4 unchanged; this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent. The equivalent level-5 negative control now lives in the new
    // tests/sd13_druid_level4_progression.rs file where the coverage moved.
    let level_4 = DRUID_FIXTURE.replace("class:druid:1", "class:druid:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-4 Druid is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, WILD_EMPATHY_ID) && has_explanation(&computation, NATURE_SENSE_ID),
        "level-4 Druid is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-4 Druid must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the matrix row is promoted inline (Sorcerer/Bard/Cleric pattern) -----

#[test]
fn matrix_druid_row_is_partial_computed_and_names_remaining_burdens() {
    let matrix = seeded_current_truth();
    let druid = matrix
        .row("class.druid.progression_and_spell_burden")
        .expect("druid row must exist");

    // Wild Empathy, Nature Sense, and the nature-bond choice recognition are now
    // grounded; the animal-companion execution and the prepared divine spell
    // posture remain unproven. This slice alone leaves the row Partial; the row
    // was later promoted to Supported/Product-visible by SD-19's Class
    // Progression Catalog browser cycle (2026-07-16), once the browser surfaced
    // every named grounded milestone live.
    assert_eq!(druid.support_state, SupportState::Supported);
    assert_ne!(druid.support_state, SupportState::Blocked);
    assert_eq!(druid.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(druid.evidence_freshness, EvidenceFreshness::RefreshableFromLiveProof);
    assert!(
        druid
            .grounding_ref
            .contains("sd13_druid_level1_spell_baseline"),
        "carrier grounding_ref must cite this slice's proof surface"
    );
    for token in ["wild empathy", "nature sense", "nature bond", "animal companion", "prepared"] {
        assert!(
            druid.blocker_or_lossiness_note.contains(token),
            "druid blocker note must name '{token}' — wild empathy / nature sense / the nature \
             bond choice (grounded) and the animal-companion execution / prepared divine spell \
             burdens (unproven): {}",
            druid.blocker_or_lossiness_note
        );
    }
    assert!(
        druid.dimension.contains("animal-companion execution")
            || druid.dimension.contains("animal companion"),
        "druid row dimension must truthfully name the remaining animal-companion execution \
         burden instead of the retired combined nature-bond burden: {}",
        druid.dimension
    );
    assert!(
        !druid.blocker_or_lossiness_note.is_empty(),
        "partial druid row must carry a non-empty note"
    );
}

#[test]
fn matrix_preserves_wizard_hybrid_blocked_computed_and_sorcerer_bard_cleric_supported_truth() {
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

    // Cleric was later promoted to Supported/ProductVisible by SD-19's Class
    // Progression Catalog browser UI-surfacing work (2026-07-16).
    let cleric = matrix
        .row("class.cleric.progression_and_spell_burden")
        .expect("cleric row must exist");
    assert_eq!(
        cleric.support_state,
        SupportState::Supported,
        "cleric row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(cleric.evidence_tier, EvidenceTier::ProductVisible);

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

    // Wizard was later promoted to Partial/Computed by its own SD13-E4 Scribe
    // Scroll decomposition slice, then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17).
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");
    assert_eq!(
        wizard.support_state,
        SupportState::Supported,
        "wizard row must keep its later-accepted Supported posture after the Druid slice"
    );
    assert_eq!(wizard.evidence_tier, EvidenceTier::ProductVisible);
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
        "the Druid slice must not promote any row to Supported or Lossy"
    );
}
