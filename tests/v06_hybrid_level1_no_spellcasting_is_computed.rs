//! v0.6 alpha swarm: Paladin and Ranger compute at *every* level 1-20, because
//! "this class has no spellcasting at this level" is a satisfied condition, not a
//! gap.
//!
//! **The corpus fact this rests on.** In
//! `pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`,
//! the `CLASS:Paladin` block (line 164ff) and the `CLASS:Ranger` block (line
//! 206ff) carry *no `CAST:` row at all* for class levels 1, 2 and 3. The first
//! `CAST:` row either class has is at class level 4, and it reads `CAST:0,0` --
//! zero level-0 slots, zero *base* level-1 slots (a high-Charisma/Wisdom paladin
//! or ranger gets a bonus slot there, nothing more). The first nonzero base slot
//! appears at class level 5 (`CAST:0,1`). Both classes' `BONUS:CASTERLEVEL` rows
//! are gated `PRECLASS:1,Paladin=4` / `PRECLASS:1,Ranger=4` with a `CL-3`
//! effective caster level. So a level-1 Paladin or Ranger having no spell slots,
//! no caster level and no prepared-spell posture is *correct PF1*: there is
//! genuinely nothing to compute.
//!
//! **What was actually wrong.** `explain_hybrid_level1_chassis` emitted a blanket
//! `class_spell.hybrid.<class>.unsupported` claim-blocking diagnostic asserting
//! that "spell slots, spell source, and spells known/prepared posture are out of
//! scope for this level-1 chassis baseline". That claim was true when it was
//! written and is now false. The later per-class slices
//! (`explain_paladin_level1_chassis_and_spell_burden_separation`, 2026-07-25, and
//! `explain_ranger_level1_chassis_and_class_feature_separation`, 2026-07-24) run
//! *unconditionally at every level* on the very same input and ground the real
//! spell posture -- effective caster level, spell-level access ceiling, the
//! prepared-spell selection, and the per-day slot budget -- validating each
//! against the real PF1 spell list and slot table. At level 1 those records
//! ground the honest, correct *absence* (caster level 0, access ceiling 0, zero
//! prepared spells, no per-day slot records at all, because
//! `paladin_base_spells_per_day_table(1)` is `[None; 4]`).
//!
//! This is the exact same self-contradiction that already retired this
//! function's sibling `class_feature.hybrid.<class>.unsupported` diagnostic (see
//! `tests/hybrid_diagnostic_grounded_contradiction.rs`): a blanket "not
//! implemented" blocker sitting alongside grounded records that implement
//! precisely the thing it claims is missing.
//!
//! **The level-1-only asymmetry, explained.** It was never rules-driven. The
//! blanket blocker is reached only through `hybrid_level1_class`, which matches
//! `[class_level] if class_level.level == HYBRID_BASELINE_LEVEL` -- a hard
//! equality against 1. Levels 2-20 were never "passing a spell check"; they were
//! structurally never subject to this diagnostic at all. Levels 2 and 3 have
//! exactly as little spellcasting as level 1 and computed fine, which is itself
//! the proof that correct-absence is the right treatment for level 1 too.
//!
//! Nothing here loosens the honest blockers: the per-class
//! `class_spell.<class>.partial_caster.unsupported` diagnostics still fire for a
//! genuinely invalid prepared-spell posture, and this test pins the real slot
//! table at the levels where spellcasting actually begins so the flip cannot be
//! mistaken for "declare it computed and move on".

use codex::rules_core::character_input::{CharacterClassLevel, CharacterInput};
use codex::rules_core::pilot_compute::{
    HeadlessReceiptStatus, build_pilot_headless_receipt, compute_pilot_base_chassis,
};
mod common;
use common::load;

/// The shared deterministic GE-06 loadout fixture, i.e. the *real fixed loadout*
/// `pf1_adapter.rs`'s `compose_character_input` composes for a freshly created
/// character. This is deliberately the same fixture (and the same class-swap
/// shape, mirroring that binary's `input_for`) that
/// `src/bin/v06_class_state_dump.rs` sweeps, because "can a user build this
/// class in the app yet?" is the question this test is about.
///
/// The narrower `pf1_human_{paladin,ranger}_level1_sd13_deterministic_input.txt`
/// fixtures are deliberately NOT used here: they omit the GE-06 feat and skill
/// posture, so they carry their own unrelated pre-existing
/// `combat.baseline_unsupported` / `skill.selected_modifier.unsupported`
/// blockers that have nothing to do with hybrid spellcasting.
const GE06_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// The class level at which both Paladin and Ranger gain their first `CAST:`
/// row in the corpus. The row is `CAST:0,0`: the caster level gate opens, but
/// the *base* level-1 slot count is still 0.
const SPELLCASTING_BEGINS_AT_LEVEL: u8 = 4;

/// The class level at which both classes' first nonzero *base* level-1 spell
/// slot appears in the corpus (`CAST:0,1`).
const FIRST_NONZERO_BASE_SLOT_LEVEL: u8 = 5;

/// Swap the GE-06 fixture onto `class_name` at `level`, mirroring
/// `src/bin/v06_class_state_dump.rs`'s own `input_for` exactly. Neither Paladin
/// nor Ranger takes any canonical choice/spell seed there, so none is applied
/// here either.
fn input_for(class_name: &str, level: u8) -> CharacterInput {
    let mut input = load(GE06_FIXTURE);
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_name}"),
        level,
    }];
    input
}

// ----- The fix: level 1 is Computed, with zero claim-blocking diagnostics -----

#[test]
fn paladin_level1_reaches_computed_with_no_claim_blocking_diagnostics() {
    let input = input_for("paladin", 1);
    let computation = compute_pilot_base_chassis(&input);

    let blocking: Vec<_> = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .collect();
    assert!(
        blocking.is_empty(),
        "a level-1 Paladin has no spellcasting by PF1 (no CAST: row before class \
         level 4), which is a satisfied condition and not a gap; it must emit no \
         claim-blocking diagnostics, got: {blocking:?}"
    );

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "level-1 Paladin must reach Computed"
    );
}

#[test]
fn ranger_level1_reaches_computed_with_no_claim_blocking_diagnostics() {
    let input = input_for("ranger", 1);
    let computation = compute_pilot_base_chassis(&input);

    let blocking: Vec<_> = computation
        .diagnostics
        .iter()
        .filter(|d| d.claim_blocking)
        .collect();
    assert!(
        blocking.is_empty(),
        "a level-1 Ranger has no spellcasting by PF1 (no CAST: row before class \
         level 4), which is a satisfied condition and not a gap; it must emit no \
         claim-blocking diagnostics, got: {blocking:?}"
    );

    let receipt = build_pilot_headless_receipt(&input);
    assert_eq!(
        receipt.status,
        HeadlessReceiptStatus::Computed,
        "level-1 Ranger must reach Computed"
    );
}

#[test]
fn the_retired_blanket_hybrid_spell_blocker_never_reappears() {
    for (class, id) in [
        ("paladin", "class_spell.hybrid.paladin.unsupported"),
        ("ranger", "class_spell.hybrid.ranger.unsupported"),
    ] {
        for level in 1..=20u8 {
            let computation = compute_pilot_base_chassis(&input_for(class, level));
            assert!(
                !computation.diagnostics.iter().any(|d| d.id == id),
                "the retired blanket blocker '{id}' must not reappear at level {level}: {:?}",
                computation.diagnostics
            );
        }
    }
}

#[test]
fn paladin_and_ranger_compute_at_every_level_1_through_20() {
    for class in ["paladin", "ranger"] {
        let blocked: Vec<u8> = (1..=20u8)
            .filter(|level| {
                build_pilot_headless_receipt(&input_for(class, *level)).status
                    != HeadlessReceiptStatus::Computed
            })
            .collect();
        assert!(
            blocked.is_empty(),
            "{class} must reach Computed at every level 1-20; blocked at {blocked:?}"
        );
    }
}

// ----- The guard: real spell slots still appear at the level they really begin -----

#[test]
fn hybrid_spell_posture_is_correctly_absent_before_level_4_and_present_from_level_4() {
    for class in ["paladin", "ranger"] {
        let caster_level_id = format!("class_chassis.{class}.partial_caster.effective_caster_level");
        let access_id = format!("class_chassis.{class}.partial_caster.spell_level_access");
        let base_slot_id =
            format!("class_chassis.{class}.partial_caster.base_spells_per_day.spell_level_1");

        for level in 1..SPELLCASTING_BEGINS_AT_LEVEL {
            let computation = compute_pilot_base_chassis(&input_for(class, level));
            let value = |id: &str| {
                computation
                    .explanations
                    .iter()
                    .find(|e| e.id == id)
                    .map(|e| e.value)
            };
            // Correct ABSENCE, grounded rather than merely missing: the caster
            // level gate and access ceiling are computed and are 0, and no base
            // slot record exists at all (the corpus has no CAST: row here).
            assert_eq!(
                value(&caster_level_id),
                Some(0),
                "{class} level {level} predates the corpus' first CAST: row, so its \
                 effective caster level must be a grounded 0"
            );
            assert_eq!(
                value(&access_id),
                Some(0),
                "{class} level {level} must have a grounded spell-level access ceiling of 0"
            );
            assert_eq!(
                value(&base_slot_id),
                None,
                "{class} level {level} must have no base spells-per-day record at all"
            );
        }

        // Level 4: the corpus' first CAST: row, `CAST:0,0`. The caster level gate
        // opens (CL-3 = 1) and the level-1 spell column becomes accessible, but
        // the BASE slot count is still 0.
        let computation =
            compute_pilot_base_chassis(&input_for(class, SPELLCASTING_BEGINS_AT_LEVEL));
        let value = |id: &str| {
            computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .map(|e| e.value)
        };
        assert_eq!(
            value(&caster_level_id),
            Some(1),
            "{class} level 4 effective caster level is level - 3 = 1"
        );
        assert_eq!(
            value(&access_id),
            Some(1),
            "{class} level 4 opens access to 1st-level spells"
        );
        assert_eq!(
            value(&base_slot_id),
            Some(0),
            "{class} level 4's corpus row is CAST:0,0 -- the BASE level-1 slot count is 0"
        );

        // Level 5: the corpus' first nonzero base slot, `CAST:0,1`.
        let computation =
            compute_pilot_base_chassis(&input_for(class, FIRST_NONZERO_BASE_SLOT_LEVEL));
        let base_at_5 = computation
            .explanations
            .iter()
            .find(|e| e.id == base_slot_id)
            .map(|e| e.value);
        assert_eq!(
            base_at_5,
            Some(1),
            "{class} level 5's corpus row is CAST:0,1 -- the first nonzero base level-1 slot"
        );
    }
}
