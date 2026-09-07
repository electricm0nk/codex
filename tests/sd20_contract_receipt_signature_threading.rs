//! Wiring-project cycle 0 (`contract:receipt_signature_threading`).
//!
//! Per `~/.claude/plans/adaptive-squishing-mccarthy.md` cycle 0:
//! `to_pilot_receipt` currently cannot reach any of SD-20's Epic 2-7
//! engines because it never receives the raw `CharacterInput` or
//! `SourcePackageContent` those engines need. This cycle widens its
//! signature from `to_pilot_receipt(receipt: &CorpusPilotReceipt) ->
//! PilotReceipt` to `to_pilot_receipt(receipt: &CorpusPilotReceipt, input:
//! &CharacterInput, corpus: &SourcePackageContent) -> PilotReceipt`,
//! without changing behavior — the new parameters are unused by the
//! function body this cycle (future cycles 1-6 wire them in).
//!
//! This test is a pure regression pin: it calls the NEW 3-arg signature
//! and asserts the `PilotReceipt` output is byte-identical (via
//! `PartialEq`) to what the OLD 2-arg call used to produce on the same
//! `CorpusPilotReceipt`. It fails to compile against the old 2-arg
//! signature (RED), and passes once the signature is widened with an
//! unused-but-accepted `input`/`corpus` pair (GREEN).

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::contract::to_pilot_receipt;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};

fn empty_corpus() -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: "sd20_contract_receipt_signature_threading".to_string(),
        line: 1,
    };
    SourcePackageContent::empty("sd20_contract_receipt_signature_threading", source_ref)
}

fn fighter_level_1_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_contract_receipt_signature_threading".to_string()),
        source_package_id: "sd20_contract_receipt_signature_threading".to_string(),
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

#[test]
fn to_pilot_receipt_widened_signature_is_behaviorally_identical() {
    let corpus = empty_corpus();
    let input = fighter_level_1_input();

    let corpus_receipt = compute_pilot_with_corpus(&input, &corpus);

    // The widened, 3-arg call — the signature this cycle lands.
    let receipt = to_pilot_receipt(&corpus_receipt, &input, &corpus);

    // Regression pin: this must equal exactly what building a
    // `PilotReceipt` straight from the `CorpusPilotReceipt`'s own
    // sections would produce — i.e. the new `input`/`corpus` parameters
    // change nothing about the output this cycle.
    assert_eq!(receipt.diagnostics, corpus_receipt.base.diagnostics);
    assert_eq!(receipt.chassis, corpus_receipt.base);
    assert_eq!(receipt.corpus_derived, corpus_receipt.corpus_derived);
}
