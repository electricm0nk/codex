//! v0.6 alpha swarm: Rogue UI-reachability, rules_core-layer catalogue proof.
//!
//! The operator asked whether Rogue can reach `Computed` end-to-end through
//! the real UI (creation and level-up), same rigor as the Wizard
//! investigation. Backend investigated and proved it via two tests in
//! `apps/desktop/src-tauri/src/pf1_adapter.rs` (commit `0bb37521`):
//! `rogue_level1_reaches_computed_from_compose_character_input_alone` and
//! `rogue_multiclass_dip_reaches_computed_from_apply_level_up_alone`, both
//! calling the real `compose_character_input`/`apply_level_up` production
//! functions.
//!
//! Those two functions live in the separate `codex-desktop` crate
//! (`apps/desktop/src-tauri`, backend's owned lane per the file-ownership
//! partition), not the `codex` library crate `tests/**` targets — they are
//! **not** structurally reachable from this directory the way
//! `src/rules_core/pilot_compute.rs`'s inline tests were for the BAB/save
//! and spell-save-DC work (same crate, direct adoption). `codex-desktop`
//! already has its own established, complete test convention for its own
//! surface (this is the same shape as the spell-slot-budget-enforcement fix
//! and the `SavedCharacterMutationOp` registry, both correctly left as
//! `codex-desktop`-owned coverage earlier in this swarm, not migrated).
//!
//! What this file provides instead: `compose_character_input`'s fixed
//! loadout is byte-for-byte the same shape as
//! `fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
//! (confirmed by reading `compose_character_input`'s source directly — race,
//! ability scores, feats, skill allocations, and equipment selections all
//! match exactly), and `apply_level_up`'s multiclass-dip branch is exactly
//! "push a new `CharacterClassLevel` at level 1" — both fully expressible as
//! a `CharacterInput` fixture run through `build_pilot_headless_receipt`,
//! the same rules_core entry point backend's tests call. This proves the
//! identical underlying claim (Rogue reaches `Computed` with the real fixed
//! loadout, both fresh and as a multiclass dip) from the rules_core layer,
//! as a permanent, independently-authored catalogue entry under QA's
//! ownership — complementary to, not a duplicate of, backend's
//! Tauri-command-layer proof.

use codex::rules_core::pilot_compute::{HeadlessReceiptStatus, build_pilot_headless_receipt};
mod common;
use common::load;

const FIGHTER_LEVEL1_FIXTURE: &str = include_str!(
    "fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
);

#[test]
fn rogue_level1_with_the_real_fixed_loadout_reaches_computed() {
    // compose_character_input's fixed loadout for ANY class (the
    // level_1_character_feat/fighter_bonus_feat choice ids are unconditional,
    // not fighter-gated -- confirmed by reading the source) is exactly this
    // fixture's shape with the class swapped. class:rogue:1's BAB/save chassis
    // is genuinely integrated (v0.6 alpha swarm task 4), and this fixture's
    // posture is the exact one combat.baseline_unsupported/
    // skill.selected_modifier.unsupported require -- so nothing should remain
    // claim-blocking.
    let rogue_input = FIGHTER_LEVEL1_FIXTURE.replace("class_level=class:fighter:1", "class_level=class:rogue:1");
    let input = load(&rogue_input);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "a freshly composed Rogue level 1, with the real fixed loadout, must reach Computed \
         with no gap analogous to Wizard's spellbook posture: {:?}",
        receipt.computation.diagnostics
    );
}

#[test]
fn rogue_multiclass_dip_onto_an_existing_fighter_reaches_computed() {
    // apply_level_up's new-class-entry branch pushes CharacterClassLevel { class_id,
    // level: 1 } with no other mutation -- mirrored here as a second class_level
    // line appended to the same real fixed-loadout fixture apply_level_up would
    // have started from (a fresh Fighter 1).
    let multiclass = FIGHTER_LEVEL1_FIXTURE.replace(
        "class_level=class:fighter:1",
        "class_level=class:fighter:1\nclass_level=class:rogue:1",
    );
    let input = load(&multiclass);
    let receipt = build_pilot_headless_receipt(&input);

    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "multiclassing Rogue onto an existing Fighter, with the real fixed loadout, must reach \
         Computed with no seeding fix needed (unlike Wizard): {:?}",
        receipt.computation.diagnostics
    );
}
