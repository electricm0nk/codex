//! **The quantity check** for `class_feature_entries` — the guarantee wave
//! 13's own withdrawal proved was missing, made structural instead of
//! hand-vetted (SD31-W15).
//!
//! # Why this file exists
//!
//! `core_rulebook:class_feature:ranger_favored_terrain` was fixtured, cleared
//! the bar check, and was withdrawn at `SD31-W13-INTEGRATE-001` — not because
//! the fixture was wrong about the corpus, but because it verified a
//! DIFFERENT MAGNITUDE than the one the unit's `grounded` evidence came from.
//! The corpus token `FavoredTerrainPool|(RangerFavoredTerrainLVL+2)/5` is the
//! NUMBER OF TERRAINS granted; the engine's matching explanation id computes
//! the flat +2/+4/+6/+8 SKILL bonus. Both facts were true; they were about
//! two different quantities, and the unit reached `done` on their conjunction.
//!
//! `class_feature` grounding is decided by an explanation-id NAME MATCH
//! (`v06_work_inventory.rs`'s `exact_suffix_grounded` /
//! `suffix_stripped_grounded`), which cannot see a quantity mismatch — so
//! nothing in the pipeline was capable of catching that class of error. This
//! file is that check: it drives the REAL engine
//! (`build_pilot_headless_receipt`, the same path `CharacterSheet.tsx`
//! renders through) at concrete levels, and asserts the value the production
//! consumer publishes equals the FIXTURE's own formula evaluated at that
//! level.
//!
//! The expected values come from the committed fixture — i.e. from
//! `scripts/derive_class_feature_level_scaling_fixtures.py`'s independent
//! read of the pinned upstream `.lst` bytes — never from the consumer, so a
//! consumer that computes the wrong quantity fails here rather than being
//! ratified.
//!
//! # Coverage, stated honestly
//!
//! [`CONSUMERS`] maps the four units this cycle added, each to the production
//! explanation id claimed to compute its fixtured token. The eight entries
//! that predate this cycle are NOT covered here: each of their consumers is
//! reached only under its own posture (a recorded rage-power selection, an
//! archetype-free Fighter, a specific Paladin level gate), which is real
//! per-unit setup work rather than a missing assertion, and is recorded as a
//! named follow-on in the cycle receipt. A partial guard that says which part
//! it covers is worth more than none; it is not worth pretending it covers
//! twelve.

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterClassLevel, CharacterInput,
};
use codex::rules_core::derived_evaluator_fixture_check::load_class_feature_fixtures;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use std::path::PathBuf;

/// The shared deterministic input every sibling engine-driving pin uses.
const FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// `(fixture unit_id, class id to put levels in, the production explanation
/// id claimed to compute the fixtured token, the levels to check)`.
///
/// The levels are chosen to straddle each formula's own step boundaries, so a
/// consumer that agreed by coincidence at one level cannot pass: an off-by-one
/// in a divisor or an offset shows up as soon as two levels either side of a
/// step are both required.
const CONSUMERS: &[(&str, &str, &str, &[u8])] = &[
    (
        "core_rulebook:class_feature:paladin_lay_on_hands",
        "class:paladin",
        "class_chassis.paladin.lay_on_hands_heal_amount",
        &[2, 3, 4, 5, 6, 11, 20],
    ),
    (
        "advanced_class_guide:class_feature:slayer_studied_target",
        "class:slayer",
        "class_feature.acg.slayer.studied_target_bonus",
        &[1, 4, 5, 9, 10, 20],
    ),
    (
        "ultimate_combat:class_feature:ninja_sneak_attack",
        "class:ninja",
        "class_feature.uc.ninja.sneak_attack",
        &[1, 2, 3, 4, 5, 19, 20],
    ),
    (
        "ultimate_combat:class_feature:samurai_resolve",
        "class:samurai",
        "class_feature.uc.samurai.resolve_uses",
        &[1, 2, 3, 4, 5, 19, 20],
    ),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn base_input() -> CharacterInput {
    let path = repo_root().join(FIXTURE);
    let text =
        std::fs::read_to_string(&path).expect("the shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("the shared deterministic fixture loads")
}

/// The value the REAL engine publishes for `explanation_id` when the
/// character has `level` levels in `class_id`, or `None` if the engine emits
/// no such explanation at that level.
fn engine_value(class_id: &str, level: u8, explanation_id: &str) -> Option<i16> {
    let mut input = base_input();
    input.case_id = Some(format!("sd31_w15_quantity.{class_id}.level{level}"));
    input.chosen.class_levels =
        vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
    build_pilot_headless_receipt(&input)
        .computation
        .explanations
        .into_iter()
        .find(|e| e.id == explanation_id)
        .map(|e| e.value)
}

/// The fixture's own formula, evaluated at `level`:
/// `floor((level + offset_pre) / divisor) + offset_post`, with PCGen's
/// truncating integer division (all operands here are non-negative).
fn fixture_value(offset_pre: i32, divisor: i32, offset_post: i32, level: i32) -> i32 {
    let numerator = level + offset_pre;
    let quotient = if numerator < 0 { 0 } else { numerator / divisor };
    quotient + offset_post
}

#[test]
fn every_mapped_consumer_publishes_the_fixtured_formulas_own_quantity() {
    let fixtures = load_class_feature_fixtures(&repo_root());
    let mut wrong = Vec::new();
    let mut checked = 0usize;

    for (unit_id, class_id, explanation_id, levels) in CONSUMERS {
        let Some(fixture) = fixtures.iter().find(|f| &f.unit_id == unit_id) else {
            wrong.push(format!("{unit_id}: no committed class_feature fixture entry"));
            continue;
        };
        for &level in *levels {
            let expected = fixture_value(
                fixture.expected_offset_pre,
                fixture.expected_divisor,
                fixture.expected_offset_post,
                i32::from(level),
            );
            match engine_value(class_id, level, explanation_id) {
                None => wrong.push(format!(
                    "{unit_id}: the engine publishes no {explanation_id} at {class_id} level \
                     {level}, so nothing computes the fixtured token {}",
                    fixture.corpus_field
                )),
                Some(actual) if i32::from(actual) == expected => checked += 1,
                Some(actual) => wrong.push(format!(
                    "{unit_id}: at {class_id} level {level} the corpus token {} states {expected}, \
                     but {explanation_id} publishes {actual} — the fixture and the consumer are \
                     about DIFFERENT QUANTITIES (the ranger_favored_terrain defect)",
                    fixture.corpus_field
                )),
            }
        }
    }

    assert!(
        wrong.is_empty(),
        "{} quantity disagreement(s) between the committed fixtures and the real engine:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
    assert_eq!(
        checked,
        CONSUMERS.iter().map(|(_, _, _, l)| l.len()).sum::<usize>(),
        "every mapped (unit, level) pair must have been compared"
    );
}

/// Mutation-proof: the comparison this file performs must be capable of
/// failing. Evaluating the same formula one level off its own step boundary
/// must produce a different number — if it does not, the level sample is
/// insensitive and the test above proves nothing.
#[test]
fn the_level_samples_straddle_each_formulas_own_step_boundary() {
    let fixtures = load_class_feature_fixtures(&repo_root());
    let mut insensitive = Vec::new();
    for (unit_id, _, _, levels) in CONSUMERS {
        let Some(f) = fixtures.iter().find(|x| &x.unit_id == unit_id) else { continue };
        let values: Vec<i32> = levels
            .iter()
            .map(|&l| {
                fixture_value(f.expected_offset_pre, f.expected_divisor, f.expected_offset_post, i32::from(l))
            })
            .collect();
        if values.windows(2).all(|w| w[0] == w[1]) {
            insensitive.push(format!(
                "{unit_id}: every sampled level yields {}, so a wrong divisor could not be seen",
                values[0]
            ));
        }
    }
    assert!(insensitive.is_empty(), "{}", insensitive.join("\n"));
}
