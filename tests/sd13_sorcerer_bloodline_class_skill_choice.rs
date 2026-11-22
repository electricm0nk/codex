//! SD13-E5 Sorcerer Arcane bloodline class-skill choice recognition proof.
//!
//! Grounds the next honest Sorcerer bloodline pillar burden named in the SD13-E5
//! Open Blockers analysis: the Arcane bloodline's own PF1 Core Rulebook class-skill
//! grant, verified against two independent primary sources (d20pfsrd and the legacy
//! Paizo PRD mirror) to read "Class Skill: Knowledge (any one)" — a player's choice of
//! any one Knowledge skill, NOT a fixed grant of Knowledge (arcana) specifically. A
//! prior cycle correctly refused to ground this as a flat "grants Knowledge (arcana)"
//! record because that framing was factually wrong, and backed out with no code
//! changes rather than fabricate the rule.
//!
//! This slice adds a new `choice:sorcerer_bloodline_class_skill` choice-slot to the
//! deterministic Human Sorcerer level-1 fixture (selection: `knowledge:arcana`, an
//! arbitrary but deterministic pick among the legal "any one" Knowledge skills — the
//! rule does not fix which Knowledge skill is chosen) and recognizes whichever
//! Knowledge skill was selected as chosen input, mirroring the already-landed
//! bloodline-choice / Cleric domain-choice / Druid nature-bond-choice / Monk
//! bonus-feat-choice recognition idiom.
//!
//! It is recognition only. Granting a class skill confers no flat modifier by itself
//! in this codebase (no skill-rank allocation or untrained-skill-use engine exists
//! here), so this record carries no fabricated mechanical value (+0). The recognition
//! is specific to the Arcane bloodline (this class-skill grant belongs to it), so it is
//! only recognized when the Arcane bloodline selection itself was recognized.

use codex::rules_core::pilot_compute::{
    ComputationDiagnostic, ComputationExplanation, PilotBaseChassisComputation,
    compute_pilot_base_chassis,
};
mod common;
use common::load;

const SORCERER_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_sorcerer_level1_sd13_deterministic_input.txt");

const CLASS_SKILL_CHOICE_EXPLANATION_ID: &str = "class_chassis.sorcerer.bloodline_class_skill_choice";
const ARCANE_BOND_BLOCKER_ID: &str =
    "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported";

fn find_explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> Option<&'a ComputationExplanation> {
    computation.explanations.iter().find(|e| e.id == id)
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

#[test]
fn sorcerer_level1_recognizes_the_arcane_bloodline_class_skill_choice() {
    let input = load(SORCERER_FIXTURE);
    let computation = compute_pilot_base_chassis(&input);

    let choice = find_explanation(&computation, CLASS_SKILL_CHOICE_EXPLANATION_ID)
        .expect("Arcane bloodline class-skill choice must be recognized on the canonical fixture");
    assert_eq!(
        choice.value, 0,
        "class-skill choice recognition must carry no fabricated mechanical value"
    );
    assert!(
        choice.detail.contains("choice:sorcerer_bloodline_class_skill")
            && choice.detail.contains("knowledge:arcana")
            && choice.detail.contains("Knowledge (arcana)"),
        "class-skill choice recognition must name the recognized selection: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("any one")
            && choice.detail.contains("not a fixed grant of Knowledge (arcana)"),
        "class-skill choice recognition must state the corrected PF1 rule text (a player's \
         choice of any one Knowledge skill, not a fixed Knowledge [arcana] grant): {}",
        choice.detail
    );

    // The class-skill grant is now grounded, so it must be dropped from the "not
    // implemented" arcane-bond blocker's list rather than double-counted.
    let arcane_bond = claim_blocking(&computation, ARCANE_BOND_BLOCKER_ID);
    assert!(
        arcane_bond.message.contains("grounded separately above"),
        "arcane-bond blocker must acknowledge the class-skill grant is now grounded \
         separately rather than still claiming it as unimplemented: {}",
        arcane_bond.message
    );
    // Corrected 2026-07-29. This previously also required the blocker's message to
    // contain "bonus feats", because the bloodline bonus spells and bonus feats at
    // 3rd+ level were genuinely unimplemented. They are implemented now
    // (`ground_sorcerer_arcane_bloodline_progression` transcribes the whole Arcane
    // progression from the corpus), so a blocker that still claimed them as
    // "not implemented anywhere in this codebase" would be asserting a gap that has
    // closed. What genuinely remains for THIS fixture — an Arcane-bloodline Sorcerer
    // that recorded no Arcane Bond choice — is the unrecognized Arcane Bond itself
    // and the bloodline arcana that stays unresolved until it is recognized, so that
    // is what the blocker must now name.
    assert!(
        arcane_bond.message.contains("Arcane Bond")
            && arcane_bond.message.contains("bloodline arcana"),
        "arcane-bond blocker must still name the genuinely remaining Arcane Bond / bloodline \
         arcana burden: {}",
        arcane_bond.message
    );
    assert!(
        !arcane_bond.message.contains("bonus spells and bonus feats at 3rd+ level are not \
             implemented"),
        "the bloodline bonus spells/feats are grounded now; the blocker must not keep claiming \
         them as unimplemented: {}",
        arcane_bond.message
    );
}

#[test]
fn sorcerer_level1_recognizes_any_knowledge_skill_not_just_arcana() {
    // The PF1 rule text is "Knowledge (any one)" — any Knowledge skill is legal, not
    // just Knowledge (arcana). A different deterministic Knowledge selection must be
    // recognized identically.
    let fixture = SORCERER_FIXTURE.replace("knowledge:arcana", "knowledge:planes");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    let choice = find_explanation(&computation, CLASS_SKILL_CHOICE_EXPLANATION_ID)
        .expect("a differently-named Knowledge skill selection must still be recognized");
    assert_eq!(choice.value, 0);
    assert!(
        choice.detail.contains("Knowledge (planes)") && choice.detail.contains("knowledge:planes"),
        "class-skill choice recognition must name whichever Knowledge skill was actually \
         selected, not a hardcoded Knowledge (arcana): {}",
        choice.detail
    );
}

#[test]
fn sorcerer_level1_does_not_fabricate_recognition_for_a_non_knowledge_selection() {
    // A selection present but not itself a "knowledge:<skill>"-shaped token (e.g. an
    // unrelated skill) must not be reported as a recognized Knowledge-skill grant, even
    // though the choice slot is present and the Arcane bloodline is recognized.
    let fixture =
        SORCERER_FIXTURE.replace("choice:sorcerer_bloodline_class_skill:knowledge:arcana",
            "choice:sorcerer_bloodline_class_skill:skill:perception");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    let choice = find_explanation(&computation, CLASS_SKILL_CHOICE_EXPLANATION_ID).expect(
        "a present but non-Knowledge selection still leaves a bounded acknowledgment record",
    );
    assert_eq!(choice.value, 0);
    assert!(
        !choice.detail.contains("recognized:"),
        "a non-Knowledge selection must not be reported as a recognized class-skill grant: {}",
        choice.detail
    );
    assert!(
        choice.detail.contains("skill:perception"),
        "the acknowledgment record must echo the received non-Knowledge selection verbatim: {}",
        choice.detail
    );

    // The arcane-bond blocker must keep naming the class-skill grant as unimplemented,
    // since no Knowledge skill was actually recognized.
    let arcane_bond = claim_blocking(&computation, ARCANE_BOND_BLOCKER_ID);
    assert!(
        arcane_bond.message.contains("class skill")
            && !arcane_bond.message.contains("grounded separately above"),
        "arcane-bond blocker must still list the class-skill grant as unimplemented when no \
         Knowledge skill selection was recognized: {}",
        arcane_bond.message
    );
}

#[test]
fn sorcerer_level1_does_not_recognize_class_skill_choice_for_a_non_arcane_bloodline() {
    // The class-skill grant belongs to the Arcane bloodline specifically. A character
    // whose bloodline choice this seam does not recognize as Arcane must not gain the
    // class-skill recognition record even if the choice slot happens to be present.
    let fixture = SORCERER_FIXTURE.replace("bloodline:arcane", "bloodline:draconic");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        find_explanation(&computation, CLASS_SKILL_CHOICE_EXPLANATION_ID).is_none(),
        "class-skill choice recognition must not fabricate an Arcane-specific grant for a \
         non-Arcane bloodline selection"
    );
}

#[test]
fn sorcerer_level1_does_not_recognize_class_skill_choice_when_absent() {
    // No class-skill choice slot is present at all. No recognition explanation may
    // appear, and the arcane-bond blocker must keep naming the grant as unimplemented.
    let fixture: String = SORCERER_FIXTURE
        .lines()
        .filter(|line| !line.starts_with("choice=choice:sorcerer_bloodline_class_skill"))
        .collect::<Vec<_>>()
        .join("\n");
    let input = load(&fixture);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        find_explanation(&computation, CLASS_SKILL_CHOICE_EXPLANATION_ID).is_none(),
        "no class-skill choice recognition may appear when no choice slot is present"
    );

    let arcane_bond = claim_blocking(&computation, ARCANE_BOND_BLOCKER_ID);
    assert!(
        arcane_bond.message.contains("class skill")
            && !arcane_bond.message.contains("grounded separately above"),
        "arcane-bond blocker must keep naming the class-skill grant as unimplemented when no \
         choice slot is present at all: {}",
        arcane_bond.message
    );
}
