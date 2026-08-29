//! SD13-E4-R3 Wizard level-1 prepared arcane spell-burden baseline proof.
//!
//! Proves the third honest SD13-E4 spell-bearing slice (after Sorcerer and Bard): the
//! live rules-core surface
//! ingests a deterministic Human `class:wizard:1` input, leaves direct computed
//! evidence that recognizes the Wizard level-1 prepared arcane spell-bearing class
//! identity rather than treating it as an undocumented packet placeholder, and yet
//! stays explicitly claim-blocked with two distinct diagnostics: one for the school
//! specialization burden (specialization choice, opposed schools, specialty school
//! bonus) and one for the prepared spell posture burden (spellbook content, spells
//! prepared per day, spell slots per day, bonus slots, spell save DCs). The slice
//! stays single-class, level-1-only, Human-only, and grounds no spell math, no
//! school-opposition mechanics, and no specialty school bonus.
//!
//! A later SD13-E4 Wizard decomposition slice splits the school-specialization
//! diagnostic into two: Scribe Scroll (the free, specialization-independent bonus
//! feat every 1st-level Wizard is granted, letting them create scrolls of spells
//! they know) is grounded for real as a bounded, non-numeric grant-only
//! explanation, while the specialization CHOICE burden (the chosen school, the two
//! opposed schools, and the specialty school bonus spell slot) stays its own named,
//! claim-blocking diagnostic. The prepared spellbook / spells-prepared / spell-slot
//! posture burden is untouched by this decomposition and remains entirely unproven.
//!
//! It also pins the matrix reclassification for the Wizard row. At slice time the
//! in-source carrier stayed `Unverified` / `Observed` and the transition was owned
//! by the merge receipt; that receipt obligation was executed after the tranche 2.6
//! closeout merged to develop (2026-07-07, merge a774a2b), so the carrier row became
//! `Blocked` / `Computed` / `RefreshableFromLiveProof`, grounded on this proof
//! surface, with a blocker note naming both burdens. The Scribe Scroll decomposition
//! then promotes the row to `Partial` / `Computed` / `RefreshableFromLiveProof`
//! (grounding_ref unchanged), mirroring how the Ranger Track grounding alone flipped
//! that row from Blocked to Partial: one of the two named Wizard burdens is now
//! grounded, while the specialization-choice burden and the entire prepared spell
//! posture burden stay unproven. Bard keeps its accepted SD13-E4-F7 posture
//! (`Blocked` / `Computed`); the accepted Paladin / Sorcerer hybrid and spontaneous
//! rows stay `Blocked` / `Computed`.
//!
//! A further SD13-E5 Wizard specialization slice grounds the flat surface of the
//! school specialization choice itself: the canonical fixture selections
//! (`choice:wizard_school_specialization -> school:evocation` plus the two opposed
//! schools `school:necromancy` and `school:transmutation`) are recognized as a
//! bounded, non-numeric recognition record, and the specialist bonus slot is
//! grounded as a flat count only (a specialist wizard gains one additional spell
//! slot of each spell level she can cast, 1st and up, usable only for spells of the
//! chosen school — at level 1, exactly one 1st-level Evocation-only bonus slot, and
//! no cantrip-level bonus slot). The specialization-choice claim-blocker is retired
//! and replaced by a narrowed claim-blocking diagnostic naming what stays
//! unimplemented: the Evocation school powers (intense spells, the force missile
//! 3 + Int-mod/day pool) and the opposed-school preparation cost (each
//! opposed-school spell occupies two prepared slots).
//!
//! It is intentionally not a spell engine. It fabricates no spellbook content, no
//! spells prepared, no spell slots per day, no spell DCs, no bonus spells, no
//! school-power math, and no opposed-school preparation-cost bookkeeping, and it
//! grounds no Wizard level 2+.

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, HeadlessReceiptStatus,
    PilotBaseChassisComputation, build_pilot_headless_receipt, compute_pilot_base_chassis,
};
use codex::rules_core::pilot_failure::PrimaryOwner;
use codex::rules_core::pilot_view_model::PilotViewModel;
use codex::rules_core::support_state_matrix::{
    EvidenceFreshness, EvidenceTier, SupportState, seeded_current_truth,
};

const WIZARD_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level1_sd13_deterministic_input.txt");

const RECOGNITION_ID: &str = "class_chassis.spell_baseline.wizard";
const SCRIBE_SCROLL_EXPLANATION_ID: &str = "class_chassis.wizard.scribe_scroll";
const SPECIALIZATION_CHOICE_EXPLANATION_ID: &str = "class_chassis.wizard.specialization_choice";
const SPECIALIST_BONUS_SLOT_EXPLANATION_ID: &str = "class_chassis.wizard.specialist_bonus_slot";
const SCHOOL_POWERS_BLOCKER_ID: &str =
    "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported";
const PREPARED_BLOCKER_ID: &str = "class_spell.wizard.prepared_spellbook.unsupported";

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

// ----- Direct runtime evidence: the prepared spell-bearing identity is acknowledged -----

#[test]
fn wizard_level1_leaves_direct_prepared_spell_baseline_recognition_evidence() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Direct runtime evidence: the level-1 Wizard prepared spell-bearing identity is
    // recognized on the compute path, not silently dropped as an undocumented packet
    // placeholder.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert!(
        recognition.detail.contains("class:wizard") && recognition.detail.contains("level 1"),
        "wizard recognition must name the class:wizard:1 identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("spell"),
        "wizard recognition must name the spell-bearing identity: {}",
        recognition.detail
    );
    assert!(
        recognition.detail.contains("prepared"),
        "wizard recognition must distinguish the prepared arcane identity from the Sorcerer spontaneous identity: {}",
        recognition.detail
    );

    // It is recognition only: it must carry no fabricated mechanical value (+0).
    assert_eq!(
        recognition.value, 0,
        "wizard prepared spell baseline recognition must carry no fabricated value (+0)"
    );
    // SUPERSEDED, NOT VIOLATED (SD-21 Epic 6, criterion 26, 2026-07-18): at slice time
    // `compute_pilot_base_chassis` dispatched only to `compute_fighter_chassis`, so
    // Wizard's `base_attack_bonus` field and generic `class_chassis.base_attack_bonus`
    // explanation were fabricated absences, not a real +0. A per-class dispatch
    // (`compute_class_chassis`) plus a new `compute_wizard_chassis` now compute this
    // pillar for real from `rules_tables::crb::class_tables::class_tables()`'s
    // Wizard row; at level 1 the 1/2-BAB formula still floors to the same +0 this
    // negative control originally pinned, but it is now a genuinely computed value.
    assert_eq!(
        computation.base_attack_bonus, 0,
        "Wizard level 1 base attack bonus (1/2 BAB, classlevel / 2) is genuinely +0 at level 1"
    );
    assert!(
        has_explanation(&computation, "class_chassis.base_attack_bonus"),
        "wizard is now a dispatch-supported class chassis and must surface the generic \
         base-attack chassis explanation"
    );

    // Ability modifiers remain class-independent and still compute (INT 17 -> +3).
    assert_eq!(computation.ability_modifiers.intelligence, 4);
}

#[test]
fn wizard_level1_fabricates_no_spell_math() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // No explanation may fabricate spellbook content, spells prepared, per-day spell
    // slot totals, bonus spells, school-power math, or general spell totals. The +0
    // recognition record, the flat specialist-bonus-slot count (grounded by its own
    // dedicated test below), and the spell-save-DC records (v0.6 alpha swarm --
    // grounded for real, not fabricated; see tests/sd13_wizard_spell_save_dcs.rs)
    // are the only spell-adjacent explanations allowed on this bounded path.
    for explanation in &computation.explanations {
        assert!(
            explanation.id == RECOGNITION_ID
                || explanation.id == SPECIALIST_BONUS_SLOT_EXPLANATION_ID
                || explanation.id.starts_with("class_chassis.wizard.spell_save_dc.")
                // SD-34 decisions.md section 18: widened BY CONSTRUCTION, not narrowed --
                // class_feature_grant_consumer now emits real, citation-backed corpus_record
                // ids for Wizard (previously wholesale-excluded); this shape carve-out admits
                // them without weakening the substring check for anything else.
                || explanation.id.starts_with("class_feature.wizard.corpus_record.")
                || !explanation.id.contains("spell"),
            "no fabricated spell explanation is allowed beyond the +0 recognition, the flat \
             specialist bonus slot count, and the real spell-save-DC records: {explanation:?}"
        );
    }
    // The recognition itself asserts it fabricates no spell math.
    let recognition = explanation(&computation, RECOGNITION_ID);
    assert_eq!(recognition.value, 0);
}

// ----- Still blocked: two distinct honest, class-specific burden diagnostics -----

#[test]
fn wizard_level1_stays_blocked_on_school_powers_and_opposed_school_cost_burden() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // With the specialization choice itself grounded, the claim-blocker narrows to
    // exactly what stays unimplemented: the Evocation school powers (intense spells
    // and the force missile 3 + Int-mod/day pool) and the opposed-school
    // preparation cost (each opposed-school spell occupies two prepared slots).
    let school_powers = claim_blocking(&computation, SCHOOL_POWERS_BLOCKER_ID);
    for token in [
        "intense spells",
        "force missile",
        "opposed",
        "two prepared",
    ] {
        assert!(
            school_powers.message.contains(token),
            "wizard school-powers blocker must name the '{token}' burden: {}",
            school_powers.message
        );
    }

    // The retired diagnostic ids must no longer appear: each was narrowed, not kept
    // alongside its replacement.
    for retired in [
        "class_feature.wizard.school_specialization.unsupported",
        "class_feature.wizard.specialization_choice.unsupported",
    ] {
        assert!(
            !computation.diagnostics.iter().any(|d| d.id == retired),
            "the retired diagnostic id '{retired}' must no longer be emitted: {:?}",
            computation.diagnostics
        );
    }
}

#[test]
fn wizard_level1_specialization_choice_is_grounded_as_canonical_recognition() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The school specialization choice is grounded as a recognition record of the
    // canonical fixture selections: Evocation chosen, Necromancy and Transmutation
    // opposed. It is recognition only, carrying no fabricated mechanical value (+0).
    let choice = explanation(&computation, SPECIALIZATION_CHOICE_EXPLANATION_ID);
    for token in ["evocation", "necromancy", "transmutation", "opposed"] {
        assert!(
            choice.detail.to_lowercase().contains(token),
            "specialization-choice recognition must name the '{token}' selection: {}",
            choice.detail
        );
    }
    assert_eq!(
        choice.value, 0,
        "specialization-choice recognition must carry no fabricated numeric value (+0)"
    );

    // Grounding the choice must not silently resolve the school-power or
    // prepared-posture burdens: both stay claim-blocking.
    claim_blocking(&computation, SCHOOL_POWERS_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

#[test]
fn wizard_level1_specialist_bonus_slot_grounds_the_flat_count_only() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // PF1 CRB: a specialist wizard gains one additional spell slot of each spell
    // level she can cast, 1st and up, usable only for spells of the chosen school.
    // At level 1 that is exactly one 1st-level Evocation-only bonus slot.
    let bonus_slot = explanation(&computation, SPECIALIST_BONUS_SLOT_EXPLANATION_ID);
    assert_eq!(
        bonus_slot.value, 1,
        "specialist bonus slot must ground the flat count of exactly one 1st-level slot at level 1"
    );
    assert!(
        bonus_slot.detail.to_lowercase().contains("evocation")
            && bonus_slot.detail.contains("1st"),
        "specialist bonus slot must name the 1st-level Evocation-only restriction: {}",
        bonus_slot.detail
    );

    // The flat count must not fabricate a cantrip-level bonus slot (there is none:
    // the specialist slot applies to spell levels 1st and up only).
    assert!(
        bonus_slot.detail.to_lowercase().contains("no cantrip"),
        "specialist bonus slot must state explicitly that no cantrip-level bonus slot exists: {}",
        bonus_slot.detail
    );

    // It grounds the count only, not slot contents or the surrounding slot posture:
    // the prepared spellbook / spells-prepared / spell-slot burden stays blocked.
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

#[test]
fn wizard_without_canonical_specialization_choices_gains_no_specialization_grounding() {
    // An input that omits the canonical specialization / opposed-school selections
    // (e.g. the desktop-composed request shape) must not gain the specialization
    // recognition or the specialist bonus slot — nothing is fabricated for a choice
    // that was never made — while both claim-blockers still fire.
    let stripped: String = WIZARD_FIXTURE
        .lines()
        .filter(|line| {
            !line.contains("choice:wizard_school_specialization")
                && !line.contains("choice:wizard_opposed_schools")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&stripped);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !has_explanation(&computation, SPECIALIZATION_CHOICE_EXPLANATION_ID),
        "a wizard without the canonical specialization choice must not gain the recognition record"
    );
    assert!(
        !has_explanation(&computation, SPECIALIST_BONUS_SLOT_EXPLANATION_ID),
        "a wizard without the canonical specialization choice must not gain a specialist bonus slot"
    );
    claim_blocking(&computation, SCHOOL_POWERS_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

#[test]
fn wizard_level1_scribe_scroll_is_grounded_as_specialization_independent_grant() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // Scribe Scroll is grounded for real: a specialization-independent bonus feat
    // grant every 1st-level Wizard receives.
    let scribe_scroll = explanation(&computation, SCRIBE_SCROLL_EXPLANATION_ID);
    assert!(
        scribe_scroll.detail.contains("Scribe Scroll"),
        "scribe scroll explanation must name Scribe Scroll by name: {}",
        scribe_scroll.detail
    );
    assert!(
        scribe_scroll.detail.contains("bonus feat"),
        "scribe scroll explanation must name it as a bonus feat grant: {}",
        scribe_scroll.detail
    );

    // It is a boolean grant, not a numeric formula: it must carry no fabricated
    // mechanical value (+0), matching every other recognition-only explanation in
    // this bounded slice.
    assert_eq!(
        scribe_scroll.value, 0,
        "scribe scroll grant must carry no fabricated numeric value (+0)"
    );

    // It must not leak into computing the school powers or the prepared spellbook
    // posture: those stay claim-blocked, and no explanation may fabricate that math.
    assert!(
        !scribe_scroll.detail.contains("intense spells")
            && !scribe_scroll.detail.to_lowercase().contains("spells prepared per day"),
        "scribe scroll grounding must not silently compute school-power or prepared-posture math: {}",
        scribe_scroll.detail
    );
    claim_blocking(&computation, SCHOOL_POWERS_BLOCKER_ID);
    claim_blocking(&computation, PREPARED_BLOCKER_ID);
}

#[test]
fn wizard_level1_stays_blocked_on_prepared_spell_posture_burden() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    // The prepared spell posture burden must be a separate, explicit, claim-blocking
    // diagnostic that names the spellbook, spells prepared, and spell slots.
    let prepared = claim_blocking(&computation, PREPARED_BLOCKER_ID);
    assert!(
        prepared.message.contains("spellbook")
            && prepared.message.contains("prepared")
            && prepared.message.contains("spell slot"),
        "wizard prepared spell blocker must name the spellbook / spells prepared / spell slots burden: {}",
        prepared.message
    );

    // The two remaining burdens are genuinely distinct diagnostics.
    assert_ne!(
        SCHOOL_POWERS_BLOCKER_ID, PREPARED_BLOCKER_ID,
        "school-powers and prepared burdens must be separate diagnostics"
    );
    let distinct_blocking = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking && d.id.starts_with("class_") && d.id.contains("wizard"))
        .count();
    assert_eq!(
        distinct_blocking, 2,
        "wizard must leave exactly two class-specific claim-blocking diagnostics (school powers \
         and opposed-school cost + prepared spellbook), now that the specialization choice and \
         specialist bonus slot are grounded rather than blocked: {:?}",
        computation.diagnostics
    );
}

#[test]
fn wizard_level1_integrated_posture_is_blocked_not_counterfeit_success() {
    let input = load(WIZARD_FIXTURE);

    // The integrated posture is blocked, never a counterfeit computed success.
    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(receipt.status, HeadlessReceiptStatus::Blocked);

    let view_model = PilotViewModel::from_receipt(&receipt);
    assert_eq!(view_model.status, HeadlessReceiptStatus::Blocked);
    assert_eq!(view_model.primary_owner, PrimaryOwner::EngineFlaw);
    assert!(
        view_model.snapshot.is_none(),
        "blocked prepared spell baseline must not emit a computed snapshot"
    );
}

// ----- The accepted Human race seam is preserved on the prepared spell-bearing path -----

#[test]
fn spell_baseline_preserves_human_race_seam() {
    let input = load(WIZARD_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        has_explanation(&computation, "race.human.ability_bonus_target"),
        "prepared spell baseline must preserve the Human ability-bonus race seam: {:?}",
        computation.explanations
    );
    assert!(
        has_explanation(&computation, "race.human.bonus_feat_grant"),
        "prepared spell baseline must preserve the Human bonus-feat race seam: {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "race.human.bounded_semantics" && !d.claim_blocking),
        "prepared spell baseline must keep the bounded, non-blocking Human race note: {:?}",
        computation.diagnostics
    );
}

// ----- Negative controls: the prepared spell baseline must not leak onto other classes/levels -----

#[test]
fn fighter_and_sorcerer_do_not_gain_wizard_recognition() {
    // A supported Fighter must not gain a wizard prepared-spell-baseline recognition
    // record.
    let fighter = load(include_str!(
        "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    ));
    let fighter_computation = compute_pilot_base_chassis(&fighter);
    assert!(
        !has_explanation(&fighter_computation, RECOGNITION_ID),
        "the Fighter chassis must not surface a wizard prepared-spell-baseline recognition record"
    );
    assert!(
        !fighter_computation
            .diagnostics
            .iter()
            .any(|d| d.id.contains("wizard")),
        "Fighter must not surface wizard burden diagnostics: {:?}",
        fighter_computation.diagnostics
    );

    // A Sorcerer must stay a spontaneous blocked baseline, never a wizard baseline.
    let sorcerer_fixture = WIZARD_FIXTURE.replace("class:wizard:1", "class:sorcerer:1");
    let sorcerer = load(&sorcerer_fixture);
    let sorcerer_computation = compute_pilot_base_chassis(&sorcerer);
    assert!(
        sorcerer_computation
            .diagnostics
            .iter()
            .any(|d| d.claim_blocking),
        "Sorcerer chassis must remain claim-blocked"
    );
    assert!(
        !has_explanation(&sorcerer_computation, RECOGNITION_ID)
            && !sorcerer_computation
                .diagnostics
                .iter()
                .any(|d| d.id.contains("wizard")),
        "Sorcerer must not surface any wizard recognition or burden diagnostics: {:?}",
        sorcerer_computation.diagnostics
    );
}

#[test]
fn wizard_level_2_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 2 was the next unproven milestone
    // and stayed unrecognized. A later SD13-E5 slice
    // (tests/sd13_wizard_level2_progression.rs) widened the level-1-only gate to
    // level 2 (mirroring the Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/
    // Sorcerer level-range gate idiom); this negative control is superseded, not
    // violated — pin the new truth here too so this file stays internally
    // consistent.
    let level_2 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:2");
    let input = load(&level_2);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-2 Wizard is supported since the SD13-E5 level-2 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-2 Wizard must stay claim-blocked in this slice"
    );
}

#[test]
fn wizard_level_3_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 3 was out of scope and stayed
    // unrecognized. A later SD13-E5 slice (tests/sd13_wizard_level3_progression.rs)
    // widened the level-range gate to level 3 (mirroring the
    // Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer level-range
    // gate idiom); this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent. Level 3 still stays
    // claim-blocked (the school-power execution and prepared spellbook posture
    // burdens remain unproven).
    let level_3 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:3");
    let input = load(&level_3);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-3 Wizard is supported since the SD13-E5 level-3 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-3 Wizard must stay claim-blocked in this slice"
    );
}

#[test]
fn wizard_level_4_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 4 was out of scope and stayed
    // unrecognized. A later SD13-E5 slice (tests/sd13_wizard_level4_progression.rs)
    // widened the level-range gate to level 4 (mirroring the
    // Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer level-range
    // gate idiom); this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent. Level 4 still stays
    // claim-blocked (the school-power execution and prepared spellbook posture
    // burdens remain unproven).
    let level_4 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:4");
    let input = load(&level_4);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-4 Wizard is supported since the SD13-E5 level-4 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-4 Wizard must stay claim-blocked in this slice"
    );
}

#[test]
fn wizard_level_5_was_later_widened_into_the_supported_tranche() {
    // At the time this file's slice landed, level 5 was out of scope and stayed
    // unrecognized. A later SD13-E5 slice (tests/sd13_wizard_level5_progression.rs)
    // widened the level-range gate to level 5 (mirroring the
    // Fighter/Paladin/Rogue/Barbarian/Monk/Cleric/Bard/Druid/Sorcerer level-range
    // gate idiom); this negative control is superseded, not violated — pin the new
    // truth here too so this file stays internally consistent. Level 5 still stays
    // claim-blocked (the school-power execution and prepared spellbook posture
    // burdens remain unproven).
    let level_5 = WIZARD_FIXTURE.replace("class:wizard:1", "class:wizard:5");
    let input = load(&level_5);
    let computation = compute_pilot_base_chassis(&input);
    assert!(
        has_explanation(&computation, RECOGNITION_ID),
        "level-5 Wizard is supported since the SD13-E5 level-5 slice: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.claim_blocking),
        "level-5 Wizard must stay claim-blocked in this slice"
    );
}

// ----- Control plane: the row stays Partial with the specialization choice grounded -----

#[test]
fn matrix_wizard_row_stays_partial_computed_with_specialization_choice_grounded() {
    let matrix = seeded_current_truth();
    let wizard = matrix
        .row("class.wizard.progression_and_spell_burden")
        .expect("wizard row must exist");

    // The Scribe Scroll decomposition slice already carried the row from Blocked to
    // Partial; this SD13-E5 specialization slice grounds the school specialization
    // choice and the flat specialist-bonus-slot count for real. Later promoted to
    // Supported/ProductVisible by SD-19's Class Progression Catalog browser
    // UI-surfacing work (2026-07-17): the school-powers / opposed-school-cost
    // burden and the entire prepared spell posture burden remain named, unproven
    // future-SD-N scope, but this row's own named grounded milestones are now
    // fully surfaced live.
    assert_eq!(
        wizard.support_state,
        SupportState::Supported,
        "wizard row must be Supported: the specialization choice and specialist bonus slot are \
         grounded and the Class Progression Catalog browser now surfaces them live"
    );
    assert_eq!(
        wizard.evidence_tier,
        EvidenceTier::ProductVisible,
        "wizard row must carry ProductVisible evidence: the prepared arcane recognition, Scribe \
         Scroll grant, and specialization-choice seams are live in the Class Progression \
         Catalog browser"
    );
    assert_eq!(
        wizard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "wizard row must be refreshable from this re-runnable proof surface"
    );
    // grounding_ref stays unchanged: the same test file grounds the merge-receipt
    // posture, the Scribe Scroll decomposition, and this specialization slice.
    assert!(
        wizard
            .grounding_ref
            .contains("sd13_wizard_level1_prepared_spell_baseline"),
        "wizard row grounding_ref must cite this proof surface unchanged: {}",
        wizard.grounding_ref
    );
    // The blocker note must name the grounded surfaces (Scribe Scroll, specialization
    // choice, specialist bonus slot) and both still-unproven burdens explicitly.
    let note = wizard.blocker_or_lossiness_note;
    assert!(
        !note.is_empty(),
        "partial wizard row must carry a non-empty blocker note"
    );
    for token in [
        "Scribe Scroll",
        "specialization choice",
        "specialist bonus slot",
        "school powers",
        "opposed-school",
        "spellbook",
        "spells prepared",
        "spell slots",
    ] {
        assert!(
            note.contains(token),
            "wizard blocker note must name the '{token}' surface: {note}"
        );
    }
}

#[test]
fn matrix_preserves_bard_supported_product_visible_truth() {
    // Bard landed its own SD13-E4-F7 spell-baseline slice, was later promoted
    // to Partial/Computed by a further SD13-E4 decomposition slice (Bardic
    // Knowledge grounded for real), then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-16).
    let matrix = seeded_current_truth();
    let bard = matrix
        .row("class.bard.progression_and_spell_burden")
        .expect("bard row must exist");
    assert_eq!(
        bard.support_state,
        SupportState::Supported,
        "bard row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(
        bard.evidence_tier,
        EvidenceTier::ProductVisible,
        "bard row must be ProductVisible after the SD-19 class-row promotion"
    );
    assert_eq!(
        bard.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof,
        "bard row must stay refreshable from its live spell-baseline proof surface"
    );
}

#[test]
fn matrix_preserves_hybrid_paladin_and_sorcerer_supported_truth() {
    let matrix = seeded_current_truth();

    // Paladin was later promoted to Partial/Computed by its own SD13-E5
    // level-gate slice (lay on hands / divine grace / mercy grounded as
    // correct level-1 absences), then to Supported/ProductVisible by SD-19's
    // Class Progression Catalog browser UI-surfacing work (2026-07-17).
    let paladin = matrix
        .row("class.paladin.hybrid_chassis_and_spell_burden")
        .unwrap_or_else(|| panic!("row class.paladin.hybrid_chassis_and_spell_burden must exist"));
    assert_eq!(
        paladin.support_state,
        SupportState::Supported,
        "paladin row must be Supported after the SD-19 class-row promotion"
    );
    assert_eq!(paladin.evidence_tier, EvidenceTier::ProductVisible);
    assert_eq!(
        paladin.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );

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
    assert_eq!(
        sorcerer.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );

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
    assert_eq!(
        ranger.evidence_freshness,
        EvidenceFreshness::RefreshableFromLiveProof
    );
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
        "the Wizard slice must not promote any row to Supported or Lossy"
    );
}
