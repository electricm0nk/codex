//! v0.6 alpha swarm: durability (character survivability) catalogue adoption.
//!
//! `src/rules_core/durability.rs` (commit `0aeed25a`) grounds the alpha
//! bar's "durability" calculation, per the lead's ruling (§4 of
//! `risks-and-open-questions.md`): max/current HP, nonlethal damage
//! tracking, and dying/unconscious/death thresholds. Before this file, the
//! only grounded HP number anywhere in this crate was Fighter's own
//! level-1 value; no running total, no per-level formula beyond level 1,
//! and no threshold classification existed for any class. The module
//! carries 13 inline `#[cfg(test)]` tests (backend's stopgap since
//! `tests/**` is QA's owned surface for this swarm). This file is QA's
//! independent catalogue adoption: different classes/levels/values than
//! the inline tests exercise, plus one direct cross-check against a value
//! already independently confirmed against a real PCGen export
//! (`tests/sd26_pilot_case_verification.rs`'s `durability.max_hp = 12`).

use codex::rules_core::character_input::CharacterClassLevel;
use codex::rules_core::durability::{classify_durability, compute_max_hp, DurabilityStatus};

const FIGHTER_CLASS_ID: &str = "class:fighter";
const WIZARD_CLASS_ID: &str = "class:wizard";
const ROGUE_CLASS_ID: &str = "class:rogue";
// v0.6 alpha swarm, risks item 8, seventh slice (2026-07-25): Monk, not
// Barbarian -- table_class_id now recognizes Barbarian too (this test's
// previous example), so it genuinely resolves a real max HP now. Monk
// remains genuinely unrecognized (not in table_class_id, APG, or ACG), so
// it's still a real negative-control example.
const MONK_CLASS_ID: &str = "class:monk";

fn class_level(class_id: &str, level: u8) -> CharacterClassLevel {
    CharacterClassLevel { class_id: class_id.to_owned(), level }
}

// ----- Cross-check against the already real-PCGen-verified pilot case -----

#[test]
fn compute_max_hp_matches_the_real_pcgen_verified_fighter_level1_case() {
    // tests/sd26_pilot_case_verification.rs independently confirmed durability.max_hp
    // = 12 against a real PCGen engine export for this exact build (Human Fighter
    // level 1, Constitution modifier +1 pre-CG-03 / the fixture's real effective
    // Constitution). Re-deriving it here via a second, independent path (direct
    // compute_max_hp call, not the from_pilot_receipt/comparator pipeline) is a
    // genuine cross-check, not a duplicate of that test's own assertion.
    let max_hp = compute_max_hp(&[class_level(FIGHTER_CLASS_ID, 1)], 2);
    assert_eq!(max_hp, Some(12), "Fighter d10 max hit die (10) + Constitution modifier 2 = 12");
}

// ----- Multi-level sums for classes other than the inline tests' own single-level cases -----

#[test]
fn compute_max_hp_sums_four_levels_for_rogue() {
    // Rogue d8, level 4, Constitution modifier +1: level 1 maximized (8+1=9),
    // levels 2-4 average (average_hit_die_value(8)=5, +1=6 each) = 9 + 6 + 6 + 6 = 27.
    let max_hp = compute_max_hp(&[class_level(ROGUE_CLASS_ID, 4)], 1);
    assert_eq!(max_hp, Some(27));
}

#[test]
fn compute_max_hp_sums_six_levels_for_wizard() {
    // Wizard d6, level 6, Constitution modifier 0: level 1 maximized (6+0=6),
    // levels 2-6 average (average_hit_die_value(6)=4, +0=4 each, 5 levels) = 6 + 20 = 26.
    let max_hp = compute_max_hp(&[class_level(WIZARD_CLASS_ID, 6)], 0);
    assert_eq!(max_hp, Some(26));
}

#[test]
fn compute_max_hp_floors_a_rogue_level_at_one_hp_against_a_severe_constitution_penalty() {
    // Rogue d8, level 3, Constitution modifier -6: level 1 would be 8-6=2 (above
    // the floor), levels 2-3 average (5-6=-1) each floor up to 1. Total 2+1+1=4.
    // Uses Rogue and a different (more severe) penalty than the inline module's
    // own Wizard/-5 floor test, to exercise the floor independently.
    let max_hp = compute_max_hp(&[class_level(ROGUE_CLASS_ID, 3)], -6);
    assert_eq!(max_hp, Some(4));
}

// ----- Negative controls: different class combinations than the inline tests -----

#[test]
fn compute_max_hp_returns_none_for_a_wizard_rogue_multiclass_build() {
    let max_hp = compute_max_hp(
        &[class_level(WIZARD_CLASS_ID, 3), class_level(ROGUE_CLASS_ID, 2)],
        1,
    );
    assert_eq!(
        max_hp, None,
        "multiclass builds are out of scope (which single level was character-level-1, and \
         gets the maximized die, is ambiguous from CharacterClassLevel's cumulative-level \
         shape) regardless of which two classes are mixed"
    );
}

/// Updated 2026-07-29 (v0.6 alpha swarm, Monk/Summoner chassis-recognition
/// closure). This test previously asserted `None` for Monk, on the grounds
/// that "durability is scoped to the same table_class_id allowlist as the
/// multiclass BAB/save dispatch -- Monk is not in it". Monk IS in it now,
/// so the premise is gone and the assertion is inverted rather than
/// deleted: durability is still scoped to that allowlist, and the
/// allowlist grew.
///
/// Monk d10 (`cr_classes.lst:147`, `CLASS:Monk HD:10`), level 1, CON mod
/// +3: the level-1 die is maximized, so 10 + 3 = 13.
#[test]
fn compute_max_hp_now_resolves_monk_since_table_class_id_recognizes_it() {
    let max_hp = compute_max_hp(&[class_level(MONK_CLASS_ID, 1)], 3);
    assert_eq!(
        max_hp,
        Some(13),
        "Monk joined the table_class_id allowlist, so durability must resolve its real d10"
    );
}

/// The negative control the Monk case above used to provide.
///
/// It is deliberately a synthetic id: Monk was the LAST real base class
/// missing from `table_class_id`, so between it, `ApgClassId` and
/// `AcgClassId` all 27 base classes now resolve a hit die, and no real
/// class can express the "unrecognized" branch any more.
#[test]
fn compute_max_hp_returns_none_for_a_class_id_no_book_recognizes() {
    let max_hp = compute_max_hp(&[class_level("class:not_a_real_pf1_class", 1)], 3);
    assert_eq!(
        max_hp, None,
        "an unrecognized class id must resolve no hit die rather than a fabricated one"
    );
}

// ----- classify_durability: the full threshold spectrum, different values than inline -----

#[test]
fn classify_durability_normal_at_low_but_positive_hp() {
    // A character down to their last few real hit points is still Normal, not
    // Staggered -- Staggered requires nonlethal damage to have caught up to
    // current HP, not merely a low HP total. Different concrete values than
    // the module's own inline "normal when healthy" test (20/0/14).
    assert_eq!(classify_durability(3, 0, 12), DurabilityStatus::Normal);
}

#[test]
fn classify_durability_staggered_with_a_different_constitution_score() {
    assert_eq!(classify_durability(6, 6, 10), DurabilityStatus::Staggered);
}

#[test]
fn classify_durability_unconscious_when_nonlethal_exceeds_current_hp_by_more_than_one() {
    // The module's own inline test exercises nonlethal exceeding current HP by
    // exactly 1 (10/11/14) -- this checks a larger excess still classifies the
    // same way, not a boundary-adjacent coincidence.
    assert_eq!(classify_durability(5, 20, 12), DurabilityStatus::Unconscious);
}

#[test]
fn classify_durability_dying_at_the_boundary_just_short_of_the_constitution_score() {
    // Constitution 16: dead at current_hp <= -16. -15 is one short of that --
    // still Dying, not yet Dead. A tighter boundary check than the module's own
    // inline test (-3 with Constitution 14, far from the death threshold).
    assert_eq!(classify_durability(-15, 0, 16), DurabilityStatus::Dying);
}

#[test]
fn classify_durability_dead_exactly_at_the_negative_constitution_boundary() {
    // Constitution 16: current_hp == -16 is the exact boundary (<=), must be Dead,
    // not Dying -- the off-by-one case the module's own inline test (-14 and -20
    // with Constitution 14) doesn't isolate as a single exact-boundary value.
    assert_eq!(classify_durability(-16, 0, 16), DurabilityStatus::Dead);
}

#[test]
fn classify_durability_disabled_regardless_of_accumulated_nonlethal_damage() {
    // At exactly 0 current HP, the character is Disabled per the lethal-damage
    // rule, even if nonlethal damage happens to also be present -- Disabled takes
    // priority in the threshold ordering (checked before the nonlethal branches).
    assert_eq!(classify_durability(0, 5, 12), DurabilityStatus::Disabled);
}
