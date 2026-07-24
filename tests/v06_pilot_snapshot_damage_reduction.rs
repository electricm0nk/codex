//! v0.6 alpha swarm: independent tests/** catalogue coverage for the
//! `PilotSnapshot::from_receipt` damage-reduction exposure fix (backend,
//! `f7ce289d`, risks-and-open-questions.md item 6).
//!
//! Backend's own inline tests in `pilot_view_model.rs` exercise
//! `PilotSnapshot::from_receipt` against a hand-built synthetic
//! `PilotHeadlessReceipt` with a fabricated `explanations` list. Verified
//! directly (not assumed) that a real fixture-driven test through the
//! public `build_pilot_headless_receipt` entry point cannot exercise this
//! path today: `PilotSnapshot::from_receipt` is module-private (confirmed
//! by a compile error attempting to call it directly), and the public
//! `PilotViewModel::from_receipt` only invokes it when `receipt.status ==
//! Computed` -- Barbarian is not yet a chassis-dispatch-supported class, so
//! a real Barbarian input's receipt status is always `Blocked`, and the
//! DTO's `snapshot` field (and therefore `damage_reduction`) would always
//! be `None` regardless of the fix, for a reason unrelated to the fix
//! itself. Backend's own commit message says exactly this.
//!
//! This file closes the real gap without reimplementing production logic
//! or duplicating backend's synthetic-explanations shape: it drives a real
//! Barbarian fixture through the real `compute_pilot_base_chassis` (the
//! actual function that produces the `class_feature.barbarian.
//! damage_reduction` explanation from real chosen input, confirmed via
//! `sd13_barbarian_level7_progression.rs`'s own coverage of the same
//! explanation id), then wraps that *real* computation in a
//! `PilotHeadlessReceipt` with the status field set to `Computed` --
//! the one synthesized value, and only because Barbarian's own
//! Computed/Blocked gating is out of scope here, exactly as backend's
//! reasoning already established. Everything upstream of that one field is
//! real: real fixture, real ability/skill/combat computation, real
//! explanations list. Passed through the public `PilotViewModel::from_receipt`
//! entry point end to end.

use codex::rules_core::character_input::load_character_input_fixture;
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, PilotHeadlessReceipt, compute_pilot_base_chassis,
};
use codex::rules_core::pilot_view_model::PilotViewModel;

const BARBARIAN_LEVEL6_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level6_sd13_deterministic_input.txt"
);
const BARBARIAN_LEVEL7_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level7_sd13_deterministic_input.txt"
);
const BARBARIAN_LEVEL12_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_barbarian_level12_sd18_widening_deterministic_input.txt"
);

fn damage_reduction_for(fixture: &str) -> Option<i16> {
    let result = load_character_input_fixture(fixture);
    assert!(
        result.diagnostics.is_empty(),
        "fixture should load cleanly: {:?}",
        result.diagnostics
    );
    let input = result
        .character_input
        .expect("valid fixture should produce a character input record");

    // Real computation from a real fixture through the real production
    // function -- the only synthesized piece is the outer status, since
    // Barbarian's own Computed/Blocked gating is unrelated to this fix.
    let computation = compute_pilot_base_chassis(&input);
    let receipt = PilotHeadlessReceipt {
        case_id: input.case_id.clone(),
        source_package_id: input.source_package_id.clone(),
        status: HeadlessReceiptStatus::Computed,
        computation,
    };

    let view_model = PilotViewModel::from_receipt(&receipt);
    let snapshot = view_model
        .snapshot
        .expect("Computed status must produce a snapshot");
    snapshot.defense.damage_reduction
}

#[test]
fn a_real_level6_barbarian_snapshot_omits_damage_reduction_below_the_level_gate() {
    assert_eq!(
        damage_reduction_for(BARBARIAN_LEVEL6_FIXTURE),
        None,
        "a real level-6 Barbarian input (below the level-7 DR gate) must not surface a \
         fabricated damage_reduction value on the DTO"
    );
}

#[test]
fn a_real_level7_barbarian_snapshot_surfaces_damage_reduction_one() {
    assert_eq!(
        damage_reduction_for(BARBARIAN_LEVEL7_FIXTURE),
        Some(1),
        "a real level-7 Barbarian input (at the DR 1/- gate) must surface damage_reduction = 1 \
         on the DTO, driven end to end from the real compute pipeline"
    );
}

#[test]
fn a_real_level12_barbarian_snapshot_surfaces_damage_reduction_two() {
    assert_eq!(
        damage_reduction_for(BARBARIAN_LEVEL12_FIXTURE),
        Some(2),
        "a real level-12 Barbarian input (past the level-10 DR 2/- step-up) must surface \
         damage_reduction = 2 on the DTO"
    );
}
