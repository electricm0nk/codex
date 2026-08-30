//! SD-20 Epic 1 (boundary contract): `PilotReceipt` types — cycle 2.
//!
//! Per `SD-20-rules-engine-completeness-scope-draft.md` §1.1 and
//! `technical-design.md` §1.1, the boundary contract's "Outputs" section
//! names a `PilotReceipt` shape carrying (a) per-derived-stat fields, (b)
//! per-source-record fields with `TableCellRef` provenance, and (c)
//! diagnostic fields with `claim_blocking: true` preserved. This is the
//! second Epic-1 work-unit (`PilotReceipt` types land after
//! `CharacterInput` types, per the loop instruction's Step 2), landing in
//! `src/rules_core/contract.rs` as `PilotReceipt` plus the
//! `to_pilot_receipt` function that builds it from the existing
//! corpus-aware compute seam's output (`CorpusPilotReceipt`, from
//! `pilot_compute_corpus.rs`) — composing with the existing
//! `PilotBaseChassisComputation` / `CorpusPilotReceipt` shapes rather than
//! duplicating them from scratch, per `pilot_compute_corpus.rs`'s own doc
//! comment and `docs/release/SD-20/boundary-contract.md` §2's stub note.
//!
//! This cycle does not touch the printed-sheet cell map or add a
//! wire-fixture JSON (later Epic-1 work-units per Step 2).

use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    SpellSelection,
};
use codex::rules_core::contract::to_pilot_receipt;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

fn empty_corpus() -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: "sd20_contract_pilot_receipt".to_string(),
        line: 1,
    };
    SourcePackageContent::empty("sd20_contract_pilot_receipt", source_ref)
}

fn fighter_level_1_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_pilot_receipt".to_string()),
        source_package_id: "sd20_contract_pilot_receipt".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 14,
                dexterity: 12,
                constitution: 13,
                intelligence: 10,
                wisdom: 10,
                charisma: 8,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn wizard_level_1_input() -> CharacterInput {
    let mut input = fighter_level_1_input();
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: "wizard".to_string(),
        level: 1,
    }];
    input.chosen.spells_selected = vec![SpellSelection {
        spell_id: "Not A Real Spell".to_string(),
        source_class_id: "wizard".to_string(),
        acquisition_mode: AcquisitionMode::Known,
    }];
    input
}

#[test]
fn pilot_receipt_chassis_matches_compute_pilot_base_chassis_directly() {
    let corpus = empty_corpus();
    let input = fighter_level_1_input();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // (a) per-derived-stat fields: the `chassis` section is exactly what
    // calling the chassis function directly on the same input produces —
    // no drift, no re-derivation.
    assert_eq!(receipt.chassis, compute_pilot_base_chassis(&input));
}

#[test]
fn pilot_receipt_corpus_derived_matches_corpus_pilot_receipt_section() {
    let corpus = empty_corpus();
    let input = fighter_level_1_input();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // (b) per-source-record fields with provenance: the `corpus_derived`
    // section is exactly the seam's own section, unmodified.
    assert_eq!(receipt.corpus_derived, corpus_receipt.corpus_derived);
    assert!(receipt.corpus_derived.school_coverage.is_empty());
    assert!(receipt.corpus_derived.equipped_items.is_empty());
}

#[test]
fn pilot_receipt_diagnostics_preserve_claim_blocking_true() {
    let corpus = empty_corpus();
    // A wizard-only input carries no supported single-class Fighter chassis
    // posture, so `compute_pilot_base_chassis` emits a
    // `class_chassis.unsupported` diagnostic with `claim_blocking: true`.
    let input = wizard_level_1_input();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // (c) diagnostic fields: claim-blocking diagnostics remain
    // `claim_blocking: true`, hoisted to the receipt's top level unchanged
    // from the chassis computation's own diagnostics.
    assert_eq!(receipt.diagnostics, corpus_receipt.base.diagnostics);
    let unsupported = receipt
        .diagnostics
        .iter()
        .find(|d| d.id == "class_chassis.unsupported")
        .expect("expected a class_chassis.unsupported diagnostic for a wizard-only input");
    assert!(
        unsupported.claim_blocking,
        "class_chassis.unsupported must remain claim_blocking: true"
    );
}
