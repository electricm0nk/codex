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

/// SD-36 Epic F3d (decisions.md §14): the multiclass negative controls' assertion (b),
/// STATUS PARITY with the class alone. Formerly "must stay claim-blocked in this slice".
///
/// The mix (`mix_fixture`, this class's own fixture widened to a Class+Fighter mix) must
/// get the same receipt status as the unmodified class-alone fixture, and the same
/// claim-blocking diagnostic set once the fold's `multiclass.<class>.` re-scope is
/// stripped (the F3b `class_dispatch` precedent,
/// `..._computes_exactly_when_it_computes_alone`). A Computed class stays Computed in a
/// mix; a class blocked alone stays blocked on exactly its own lines -- no line lost, no
/// multiclass-only line added. Vacuity guard: the mix loads at least two classes and more
/// than the class alone.
pub fn assert_multiclass_status_parity(label: &str, alone_fixture: &str, mix_fixture: &str) {
    use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
    use std::collections::BTreeSet;

    let alone = load(alone_fixture);
    let mix = load(mix_fixture);
    assert!(
        mix.chosen.class_levels.len() >= 2,
        "{label}: the mix must load at least two classes: {:?}",
        mix.chosen.class_levels
    );
    assert!(
        alone.chosen.class_levels.len() < mix.chosen.class_levels.len(),
        "{label}: the mix must load more classes than the class alone: alone {:?}, mix {:?}",
        alone.chosen.class_levels,
        mix.chosen.class_levels
    );
    let rescopes: Vec<String> = mix
        .chosen
        .class_levels
        .iter()
        .map(|cl| format!("multiclass.{}.", cl.class_id.strip_prefix("class:").unwrap_or(&cl.class_id)))
        .collect();
    let alone_receipt = build_pilot_headless_receipt(&alone);
    let mix_receipt = build_pilot_headless_receipt(&mix);
    let alone_blockers: BTreeSet<String> = alone_receipt
        .computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .map(|d| d.id.clone())
        .collect();
    let mix_blockers: BTreeSet<String> = mix_receipt
        .computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .map(|d| {
            rescopes
                .iter()
                .find_map(|p| d.id.strip_prefix(p.as_str()))
                .unwrap_or(&d.id)
                .to_owned()
        })
        .collect();
    assert_eq!(
        mix_receipt.status, alone_receipt.status,
        "{label}: the mix's receipt status must equal the class alone's; mix blockers \
         {mix_blockers:?}, alone blockers {alone_blockers:?}"
    );
    assert_eq!(
        mix_blockers, alone_blockers,
        "{label}: the mix's claim-blocking set (multiclass.<class>. re-scope stripped) must \
         equal the class alone's"
    );
}
