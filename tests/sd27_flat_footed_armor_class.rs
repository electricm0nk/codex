//! SD-27 (`decisions.md` §28) — **flat-footed AC was a third compute twin, and
//! it lived in TypeScript.**
//!
//! # The defect
//!
//! `apps/desktop/src/characterHub/CharacterSheet.tsx` computed the sheet's
//! Flat-Footed cell itself:
//!
//! ```text
//! const flatFooted = ac - Math.max(0, dexMod);
//! ```
//!
//! Introduced by `f5117103` (2026-07-11) and untouched since — verified with
//! `git log -S "const flatFooted = ac - Math.max(0, dexMod)"`, which returns
//! that one commit. Neither `pilot_compute.rs` nor `pilot_compute_corpus.rs`
//! carried the statistic at all, so React was its only author.
//!
//! It subtracts the Dexterity bonus and stops. PF1's dodge-bonus rule
//! (Core Rulebook, *Bonus Types*) is explicit that this is not enough:
//!
//! > A dodge bonus improves Armor Class resulting from physical skill at
//! > avoiding blows. **Any situation that denies you your Dexterity bonus to
//! > Armor Class also denies you dodge bonuses.**
//!
//! Measured on screen: adding CRB's Dodge to a Tiefling moved AC 19 → 20
//! (correct — Dodge grants +1 AC), touch AC 13 → 14 (correct — a touch attack
//! keeps dodge bonuses) and **flat-footed 16 → 17, which PF1 forbids**. A
//! dodge bonus can never raise a flat-footed AC.
//!
//! # Why the one-line fix would have been wrong
//!
//! "Subtract the Dodge feat too" is right for exactly one build. The correct
//! quantity is *every* dodge-typed contribution to the Armor Class the sheet is
//! showing, whatever produced it. That set is enumerated by reading the two
//! Armor Class sums themselves — see this file's `DODGE_TYPED_TERMS` note — and
//! is now derived in the engine, on both compute paths, exactly as touch AC was
//! (`defense.touch_armor_class`). React displays what the engine computed.
//!
//! # The §28 standing guard
//!
//! Every `pilot_compute.rs` change lands with a test pinning the before/after
//! per affected build. Each assertion below differences the **same character**
//! with and without a dodge bonus, and states the pre-fix React value it
//! replaces, so a producer nobody consumes cannot pass here.

use codex::rules_core::character_input::{load_character_input_fixture, CharacterInput};
use codex::rules_core::pilot_compute::{compute_pilot_base_chassis, PilotBaseChassisComputation};

const DETERMINISTIC_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt");

/// The deterministic Fighter 1 posture ships **with** `feat:dodge` selected, so
/// the with-dodge case is the fixture as-is and the without-dodge case removes
/// it. Written that way round deliberately: the removal is one line and cannot
/// silently fail to take effect, whereas an addition that no identity fold
/// recognized would look like a legitimately unchanged number.
fn input_for_race(race_slug: &str, dodge: bool) -> CharacterInput {
    let text =
        DETERMINISTIC_FIXTURE.replace("race_id=race:human", &format!("race_id=race:{race_slug}"));
    let mut input = load_character_input_fixture(&text)
        .character_input
        .expect("the deterministic fixture must load");
    assert!(
        input.chosen.selected_feats.iter().any(|f| f == "feat:dodge"),
        "this fixture is expected to carry feat:dodge; the without-dodge half of every \
         assertion below is built by removing it"
    );
    if !dodge {
        input.chosen.selected_feats.retain(|f| f != "feat:dodge");
    }
    input
}

fn value(computation: &PilotBaseChassisComputation, id: &str) -> Option<i16> {
    computation.explanations.iter().find(|e| e.id == id).map(|e| e.value)
}

fn cells(race: &str, dodge: bool) -> (i16, i16, i16, i16) {
    let computation = compute_pilot_base_chassis(&input_for_race(race, dodge));
    (
        value(&computation, "defense.baseline_armor_class")
            .expect("the deterministic posture must compute an armor class"),
        value(&computation, "defense.touch_armor_class").expect("touch AC is engine-owned"),
        value(&computation, "defense.flat_footed_armor_class")
            .expect("flat-footed AC must be engine-owned, not derived in the view"),
        computation.ability_modifiers.dexterity,
    )
}

/// The exact measured before/after, per §28's pin-every-affected-build rule.
///
/// `ff_before` is what `CharacterSheet.tsx` displayed for this build under the
/// old `ac - Math.max(0, dexMod)` line; `ff_after` is the engine's value. Both
/// columns are real numbers from this fixture, not a worked example.
///
/// ```text
/// build                       AC   touch   ff_before   ff_after
/// Human Fighter 1, no Dodge   16     12        14         14     <- byte-identical
/// Human Fighter 1, Dodge      17     13        15         14     <- drops by exactly 1
/// Goblin Fighter 1, no Dodge  17     13        15         15     <- byte-identical
/// Goblin Fighter 1, Dodge     18     14        16         15     <- drops by exactly 1
/// ```
#[test]
fn a_character_without_a_dodge_bonus_is_byte_identical_to_the_pre_fix_view_value() {
    for race in ["human", "tiefling", "goblin"] {
        let (armor_class, _touch, flat_footed, dexterity_modifier) = cells(race, false);
        let pre_fix_view_value = armor_class - dexterity_modifier.max(0);
        assert_eq!(
            flat_footed, pre_fix_view_value,
            "{race} holds no dodge-typed bonus, so the engine must reproduce exactly what \
             CharacterSheet.tsx's `ac - Math.max(0, dexMod)` produced: no character without a \
             dodge bonus may see their sheet change"
        );
    }
}

#[test]
fn a_character_with_dodge_drops_by_exactly_one_from_the_pre_fix_view_value() {
    for race in ["human", "tiefling", "goblin"] {
        let (armor_class, _touch, flat_footed, dexterity_modifier) = cells(race, true);
        let pre_fix_view_value = armor_class - dexterity_modifier.max(0);
        assert_eq!(
            flat_footed,
            pre_fix_view_value - 1,
            "{race} holds Dodge (+1 dodge bonus). PF1 denies dodge bonuses to a flat-footed \
             character, so the engine's value must be exactly 1 lower than the view's old one"
        );
    }
}

/// The rule stated as a difference rather than as a total: a dodge bonus raises
/// Armor Class and touch AC and must leave flat-footed AC **completely
/// unmoved**. This is the assertion the measured Tiefling failed (16 → 17).
#[test]
fn adding_dodge_raises_armor_class_and_touch_ac_but_never_flat_footed_ac() {
    for race in ["human", "tiefling", "goblin"] {
        let (ac_without, touch_without, ff_without, _) = cells(race, false);
        let (ac_with, touch_with, ff_with, _) = cells(race, true);

        assert_eq!(ac_with - ac_without, 1, "{race}: Dodge grants +1 to Armor Class");
        assert_eq!(
            touch_with - touch_without,
            1,
            "{race}: a touch attack keeps dodge bonuses, so touch AC rises with it"
        );
        assert_eq!(
            ff_with, ff_without,
            "{race}: PF1's dodge-bonus rule -- \"any situation that denies you your Dexterity \
             bonus to AC also denies you dodge bonuses\" -- so Dodge cannot move flat-footed AC \
             by anything at all. The shipped sheet moved it by +1"
        );
    }
}

/// Flat-footed AC is derived by subtraction from the very Armor Class shown
/// beside it, the same shape `touch_armor_class` uses and for the same reason:
/// a separately computed statistic is free to contradict the total it is
/// supposed to be a subset of, which is precisely how `AC 20 / flat-footed 17`
/// reached a screen.
#[test]
fn flat_footed_ac_is_never_higher_than_the_armor_class_it_is_taken_from() {
    for race in ["human", "tiefling", "goblin"] {
        for dodge in [false, true] {
            let (armor_class, _touch, flat_footed, dexterity_modifier) = cells(race, dodge);
            assert!(
                flat_footed <= armor_class,
                "{race} (dodge={dodge}): flat-footed {flat_footed} exceeds the Armor Class \
                 {armor_class} it is derived from"
            );
            assert!(
                flat_footed >= armor_class - dexterity_modifier.max(0) - 1,
                "{race} (dodge={dodge}): only the Dexterity bonus and this posture's single \
                 dodge-typed term (Dodge's +1) may be removed; anything lower means a term that \
                 is not denied to a flat-footed character was subtracted"
            );
        }
    }
}
