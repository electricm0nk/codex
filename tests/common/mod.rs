//! Shared helpers hand-copied across 300+ integration test files
//! (SD-34 fable-review finding R10-F2). This module is NOT compiled as
//! its own test binary -- `tests/common/mod.rs` is only pulled in by
//! `mod common;` from the individual `tests/*.rs` binaries that need it,
//! exactly like the copies it replaces.
#![allow(dead_code)]

use codex::rules_core::character_input::{CharacterInput, load_character_input_fixture};
use codex::rules_core::pilot_compute::{ComputationExplanation, PilotBaseChassisComputation};

pub fn load(fixture: &str) -> CharacterInput {
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

pub fn explanation<'a>(
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

pub fn has_explanation(computation: &PilotBaseChassisComputation, id: &str) -> bool {
    computation.explanations.iter().any(|e| e.id == id)
}
