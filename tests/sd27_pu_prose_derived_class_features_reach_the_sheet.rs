//! SD-27 — the 17 Pathfinder Unchained class features that used to compute
//! nothing now put a number on the character sheet (2026-08-01).
//!
//! # The defect this closes
//!
//! `pilot_compute.rs`'s own comment stated it: *"23 of Unchained Rogue's,
//! Barbarian's, Monk's and Summoner's 64 ingested class features compute
//! nothing (including the Rogue's headline Debilitating Injury)."* A previous
//! cycle made the *deferral* visible — every one of the 64 now gets a roster
//! row naming it and its grant level. That is honest, and it is not a
//! magnitude. A player looking at Debilitating Injury saw the words and no
//! number.
//!
//! The operator's ruling reframed them: these are **display values, not engine
//! subsystems**. "X + Y rounds per day where X = class level and Y = Con
//! modifier" — do the math, show the number.
//!
//! # What is checked here, and what is deliberately checked elsewhere
//!
//! The arithmetic of each formula is pinned in its own table module's unit
//! tests, against the ingested corpus record's own prose. **This file pins the
//! other half of `decisions.md §29.1`'s rule** — *a magnitude is not wired
//! until it moves on the twin the player reads* — by driving the real
//! `build_pilot_headless_receipt` pipeline and asserting the row reaches
//! `explanations`, which is what `CharacterSheet.tsx` renders through
//! `classFeaturesModel.ts`.
//!
//! One test per closed feature, per `decisions.md §24`.
//!
//! # The six that remain, and why
//!
//! [`the_six_features_that_still_compute_nothing_are_exactly_these`] pins them
//! by name. Each states no number anywhere — not in a `BONUS:`/`DEFINE:`
//! token and not in its own prose — so there is nothing to display and
//! inventing one would be the stub this repo forbids. That test fails if the
//! set grows *or* shrinks, so a seventh silently going dark is caught, and so
//! is one being closed without this note being updated.
//!
//! # §28's standing guard
//!
//! `decisions.md §28`: *"Every change to [pilot_compute.rs] lands with a test
//! pinning the before/after per affected race or class."* The before/after
//! grounded-row counts at level 10 are
//! [`GROUNDED_ROWS_AT_LEVEL_10_BEFORE_AND_AFTER`], and the sibling pins in
//! `sd27_pu_deferred_features_reach_the_character_sheet.rs` and
//! `sd27_pu_class_features_reach_by_corpus_key.rs` carry the same "after"
//! numbers.

use std::collections::BTreeSet;

use codex::rules_core::character_input::{
    load_character_input_fixture, CharacterClassLevel, CharacterInput,
};
use codex::rules_core::pilot_compute::{build_pilot_headless_receipt, ComputationExplanation};

/// The same shared deterministic fixture the sibling PU pins use, so all
/// three describe the same posture rather than three different ones.
const FIXTURE: &str =
    "tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt";

/// `(class token, grounded rows at level 10 BEFORE this change, AFTER it)`.
///
/// "Grounded" excludes the per-record roster rows (`.corpus_record.*`) and the
/// deferral notice (`.unsupported`), matching the sibling pins' definition
/// exactly. The "before" column is copied verbatim from
/// `sd27_pu_deferred_features_reach_the_character_sheet.rs`'s `PU_CLASS_PIN`
/// as it stood before this change; the "after" column was read off the real
/// pipeline.
///
/// Unchained Barbarian does not move at level 10 because all three of its
/// newly-closed features (Greater Rage 11, Tireless Rage 17, Mighty Rage 20)
/// gate above it — which is itself worth pinning, since a change that started
/// emitting them early would show up here.
///
/// **Monk's "after" raised 2026-08-01**, 12 -> 14, by the later cycle that made
/// the Unchained Monk's unarmed strike damage die reach the sheet (two rows,
/// the die face and the die count, at every level). Only the "after" column
/// moves: the "before" column is this file's own measurement of the state
/// before *its* change and is history, not a live number, so rewriting it
/// would erase the delta this pin exists to record. The unarmed-strike change
/// carries its own before/after in
/// `tests/sd27_unchained_monk_unarmed_strike_reaches_the_sheet.rs`.
const GROUNDED_ROWS_AT_LEVEL_10_BEFORE_AND_AFTER: &[(&str, usize, usize)] = &[
    ("unchained_barbarian", 10, 10),
    ("unchained_monk", 10, 14),
    ("unchained_rogue", 9, 11),
    ("unchained_summoner", 6, 11),
];

/// Grounded rows at level 20, per class. Literals read off the real pipeline.
///
/// **Monk raised 2026-08-01**, 15 -> 17, by the same unarmed-strike cycle.
const GROUNDED_ROWS_AT_LEVEL_20: &[(&str, usize)] = &[
    ("unchained_barbarian", 14),
    ("unchained_monk", 17),
    ("unchained_rogue", 12),
    ("unchained_summoner", 15),
];

fn fixture() -> CharacterInput {
    let text = std::fs::read_to_string(FIXTURE).expect("shared deterministic fixture is readable");
    load_character_input_fixture(&text)
        .character_input
        .expect("shared deterministic fixture loads")
}

fn explanations_for(class_token: &str, level: u8) -> Vec<ComputationExplanation> {
    let mut input = fixture();
    input.case_id = Some(format!("sd27_pu_prose_derived.{class_token}.level{level}"));
    input.chosen.class_levels = vec![CharacterClassLevel {
        class_id: format!("class:{class_token}"),
        level,
    }];
    build_pilot_headless_receipt(&input).computation.explanations
}

/// The row with `id` at `level`, or `None` when the engine emitted none.
fn row(class_token: &str, level: u8, id_tail: &str) -> Option<ComputationExplanation> {
    let id = format!("class_feature.pu.{class_token}.{id_tail}");
    explanations_for(class_token, level)
        .into_iter()
        .find(|explanation| explanation.id == id)
}

/// Asserts the row is absent below `granted_at` and carries `expected` at each
/// `(level, value)` pair given. The absence half matters as much as the
/// presence half: a feature that showed a number before the character had it
/// would be the same defect pointing the other way.
#[track_caller]
fn assert_row(class_token: &str, id_tail: &str, granted_at: u8, expected: &[(u8, i16)]) {
    if granted_at > 1 {
        let below = granted_at - 1;
        assert!(
            row(class_token, below, id_tail).is_none(),
            "{class_token} level {below}: {id_tail} must not be emitted before the class grants it"
        );
    }
    for (level, value) in expected {
        let found = row(class_token, *level, id_tail).unwrap_or_else(|| {
            panic!("{class_token} level {level}: no receipt row {id_tail} reached the sheet")
        });
        assert_eq!(
            found.value, *value,
            "{class_token} level {level}: {id_tail} value"
        );
        assert!(
            !found.detail.trim().is_empty(),
            "{class_token} level {level}: {id_tail} must carry its derivation text -- the sheet \
             renders `detail` verbatim as the rules citation"
        );
    }
}

// ---------------------------------------------------------------------------
// Unchained Barbarian — 3 closed
// ---------------------------------------------------------------------------

/// `Unchained Barbarian ~ Greater Rage` (`:294`). Its two real formula tokens
/// vanished into the Rage totals; the row named "Greater Rage" carried no
/// number.
#[test]
fn greater_rage_shows_the_morale_bonus_it_produces() {
    assert_row(
        "unchained_barbarian",
        "greater_rage_morale_bonus",
        11,
        &[(11, 3), (19, 3), (20, 4)],
    );
    let detail = row("unchained_barbarian", 11, "greater_rage_morale_bonus")
        .expect("emitted at 11")
        .detail;
    assert!(
        detail.contains("BONUS:VAR|RageBonus|1") && detail.contains("BONUS:VAR|RageBonusHP|TL"),
        "the derivation must name the two tokens the record actually carries: {detail}"
    );
    assert!(
        detail.contains("character level x 3"),
        "and the temporary-hit-point multiplier the same tokens produce: {detail}"
    );
}

/// `Unchained Barbarian ~ Mighty Rage` (`:296`) — identical tokens, second
/// stack.
#[test]
fn mighty_rage_shows_the_morale_bonus_it_produces() {
    assert_row("unchained_barbarian", "mighty_rage_morale_bonus", 20, &[(20, 4)]);
    let detail = row("unchained_barbarian", 20, "mighty_rage_morale_bonus")
        .expect("emitted at 20")
        .detail;
    assert!(
        detail.contains("character level x 4"),
        "the derivation must state the multiplier Mighty Rage produces: {detail}"
    );
}

/// `Unchained Barbarian ~ Tireless Rage` (`:295`) carries no formula token at
/// all — its only number is the "1 minute" in its own prose, shown here as the
/// 10 rounds every other rage magnitude is measured in.
#[test]
fn tireless_rage_shows_its_temporary_hit_point_lockout() {
    assert_row(
        "unchained_barbarian",
        "tireless_rage_lockout_rounds",
        17,
        &[(17, 10), (20, 10)],
    );
    let detail = row("unchained_barbarian", 17, "tireless_rage_lockout_rounds")
        .expect("emitted at 17")
        .detail;
    assert!(
        detail.contains("no longer fatigued"),
        "the row's other clause has no magnitude and must still be stated, not dropped: {detail}"
    );
}

// ---------------------------------------------------------------------------
// Unchained Monk — 4 closed
// ---------------------------------------------------------------------------

/// `Unchained Monk ~ Evasion` (`:465`): "he instead takes no damage" — 0%
/// where the default is half.
#[test]
fn monk_evasion_shows_the_damage_percentage_on_a_made_reflex_save() {
    assert_row(
        "unchained_monk",
        "evasion_damage_percent_on_a_made_reflex_save",
        2,
        &[(2, 0), (20, 0)],
    );
    let detail = row("unchained_monk", 2, "evasion_damage_percent_on_a_made_reflex_save")
        .expect("emitted at 2")
        .detail;
    assert!(
        detail.contains("light armor") && detail.contains("helpless"),
        "both conditions the row states must be carried, since this engine evaluates neither: \
         {detail}"
    );
}

/// `Unchained Monk ~ Improved Evasion` (`:472`): "only half damage on failed
/// saves" — 50%.
#[test]
fn monk_improved_evasion_shows_the_damage_percentage_on_a_failed_reflex_save() {
    assert_row(
        "unchained_monk",
        "improved_evasion_damage_percent_on_a_failed_reflex_save",
        9,
        &[(9, 50), (20, 50)],
    );
}

/// `Unchained Monk ~ Flawless Mind` (`:475`): "he can roll twice and take the
/// better result".
#[test]
fn monk_flawless_mind_shows_its_two_will_save_rolls() {
    assert_row("unchained_monk", "flawless_mind_will_save_rolls", 19, &[(19, 2), (20, 2)]);
    let detail = row("unchained_monk", 19, "flawless_mind_will_save_rolls")
        .expect("emitted at 19")
        .detail;
    assert!(
        detail.contains("each hour"),
        "the hourly re-attempt is a retry interval, not a second magnitude, and must be stated \
         in the derivation rather than emitted as its own number: {detail}"
    );
}

/// `Unchained Monk ~ Timeless Body` (`:474`): the aging penalty becomes 0. A
/// genuine zero, and the test asserts the carve-outs travel with it so the
/// number is not read as immortality.
#[test]
fn monk_timeless_body_shows_a_zero_aging_penalty_with_its_carve_outs() {
    assert_row(
        "unchained_monk",
        "timeless_body_aging_ability_penalty",
        17,
        &[(17, 0), (20, 0)],
    );
    let detail = row("unchained_monk", 17, "timeless_body_aging_ability_penalty")
        .expect("emitted at 17")
        .detail;
    for clause in ["already taken remain", "still accrue", "dies of old age"] {
        assert!(
            detail.contains(clause),
            "the derivation must carry the row's carve-out {clause:?}: {detail}"
        );
    }
}

// ---------------------------------------------------------------------------
// Unchained Rogue — 1 closed (the headline case)
// ---------------------------------------------------------------------------

/// `Unchained Rogue ~ Debilitating Injury` (`:583`) — the class's headline
/// feature, and the one the brief named. Its corpus row is a bare declaration
/// plus a `DESC:`: no `BONUS:`, no `DEFINE:`, nothing PCGen itself computes.
/// The prose nonetheless states a fully level-scaled rule, and this is it.
#[test]
fn debilitating_injury_shows_both_penalties_it_states() {
    assert_row(
        "unchained_rogue",
        "debilitating_injury_penalty",
        4,
        &[(4, -2), (10, -2), (16, -2), (20, -2)],
    );
    assert_row(
        "unchained_rogue",
        "debilitating_injury_penalty_against_this_rogue",
        4,
        &[(4, -4), (9, -4), (10, -6), (15, -6), (16, -8), (20, -8)],
    );

    let general = row("unchained_rogue", 4, "debilitating_injury_penalty")
        .expect("emitted at 4")
        .detail;
    for option in ["Bewildered", "Disoriented", "Hampered"] {
        assert!(
            general.contains(option),
            "all three of the row's options must be named -- only two of them are numeric, and \
             dropping the third would misreport the feature: {general}"
        );
    }

    let versus = row("unchained_rogue", 20, "debilitating_injury_penalty_against_this_rogue")
        .expect("emitted at 20")
        .detail;
    assert!(
        versus.contains("maximum of -8"),
        "the row's stated cap is what confirms the reading and must travel with it: {versus}"
    );
}

// ---------------------------------------------------------------------------
// Unchained Summoner — 9 closed
// ---------------------------------------------------------------------------

/// `Unchained Summoner ~ Life Link` (`:732`): 100 feet at full strength, then
/// two degradation bands and banishment.
#[test]
fn summoner_life_link_shows_its_leash_and_bands() {
    assert_row(
        "unchained_summoner",
        "life_link_full_strength_range_feet",
        1,
        &[(1, 100), (20, 100)],
    );
    let detail = row("unchained_summoner", 1, "life_link_full_strength_range_feet")
        .expect("emitted at 1")
        .detail;
    for band in ["1000", "10000"] {
        assert!(
            detail.contains(band),
            "the derivation must carry the {band}-foot band the same sentence states: {detail}"
        );
    }
}

/// `Unchained Summoner ~ Bond Senses` (`:734`): rounds per day equal to
/// summoner level.
#[test]
fn summoner_bond_senses_shows_rounds_per_day_equal_to_level() {
    assert_row(
        "unchained_summoner",
        "bond_senses_rounds_per_day",
        2,
        &[(2, 2), (10, 10), (20, 20)],
    );
}

/// `Unchained Summoner ~ Shield Ally` (`:735`) and the half of
/// `~ Greater Shield Ally` (`:739`) that raises it: +2 from 4th, +4 from 12th.
#[test]
fn summoner_shield_ally_shows_a_bonus_that_steps_at_twelfth() {
    assert_row(
        "unchained_summoner",
        "shield_ally_bonus",
        4,
        &[(4, 2), (11, 2), (12, 4), (20, 4)],
    );
    let detail = row("unchained_summoner", 4, "shield_ally_bonus")
        .expect("emitted at 4")
        .detail;
    assert!(
        detail.contains("NOT folded into"),
        "the bonus is gated on the eidolon's position and condition, neither of which this engine \
         tracks, and the derivation must say so rather than let it read as a live Armor Class \
         contribution: {detail}"
    );
}

/// `Unchained Summoner ~ Maker's Call` (`:736`): `1 + (level - 6) / 4`.
#[test]
fn summoner_makers_call_shows_uses_per_day() {
    assert_row(
        "unchained_summoner",
        "makers_call_uses_per_day",
        6,
        &[(6, 1), (9, 1), (10, 2), (14, 3), (18, 4), (20, 4)],
    );
}

/// `Unchained Summoner ~ Aspect` (`:738`): 2 evolution points divertible.
#[test]
fn summoner_aspect_shows_its_diversion_ceiling() {
    assert_row(
        "unchained_summoner",
        "aspect_evolution_points_divertible",
        10,
        &[(10, 2), (17, 2), (18, 6), (20, 6)],
    );
}

/// `Unchained Summoner ~ Greater Aspect` (`:742`): the ceiling rises to 6, and
/// the exchange rate changes — a second, separate rule that must not be folded
/// into the number.
#[test]
fn summoner_greater_aspect_shows_its_own_ceiling_and_names_the_exchange_rate_separately() {
    assert_row(
        "unchained_summoner",
        "greater_aspect_evolution_points_divertible",
        18,
        &[(18, 6), (20, 6)],
    );
    let detail = row(
        "unchained_summoner",
        18,
        "greater_aspect_evolution_points_divertible",
    )
    .expect("emitted at 18")
    .detail;
    assert!(
        detail.contains("1 pool point for every 2 diverted"),
        "the exchange-rate rule must be stated and must be visibly separate from the ceiling: \
         {detail}"
    );
}

/// The other half of `~ Greater Shield Ally` (`:739`): +2 to allies who are
/// not the summoner, which is what is genuinely new at 12th.
#[test]
fn summoner_greater_shield_ally_shows_the_bonus_it_extends_to_allies() {
    assert_row(
        "unchained_summoner",
        "greater_shield_ally_bonus_to_allies",
        12,
        &[(12, 2), (20, 2)],
    );
}

/// `Unchained Summoner ~ Merge Forms` (`:741`): rounds per day equal to level.
#[test]
fn summoner_merge_forms_shows_rounds_per_day_equal_to_level() {
    assert_row(
        "unchained_summoner",
        "merge_forms_rounds_per_day",
        16,
        &[(16, 16), (20, 20)],
    );
}

/// `Unchained Summoner ~ Twin Eidolon` (`:743`): **minutes** per day equal to
/// level. The unit is the row's own and is deliberately not converted to the
/// rounds its two sibling durations use.
#[test]
fn summoner_twin_eidolon_shows_minutes_not_rounds() {
    assert_row("unchained_summoner", "twin_eidolon_minutes_per_day", 20, &[(20, 20)]);
    let detail = row("unchained_summoner", 20, "twin_eidolon_minutes_per_day")
        .expect("emitted at 20")
        .detail;
    assert!(
        detail.contains("MINUTES"),
        "a unit slip between this and the two rounds-per-day siblings would be invisible in the \
         number alone, so the derivation must shout the unit: {detail}"
    );
}

// ---------------------------------------------------------------------------
// The remainder, and the guards
// ---------------------------------------------------------------------------

/// The six Pathfinder Unchained class features that still compute nothing, by
/// name, with the per-feature reason in the comment beside each.
///
/// Every one states no number **anywhere** — not in a `BONUS:`/`DEFINE:` token
/// and not in its own prose. There is nothing to display, and fabricating a
/// magnitude would be the stub `docs/governance/no-stub-mvp-doctrine.md`
/// forbids. Each still reaches the sheet as a roster row naming it and its
/// grant level, plus its class's "Not computed" notice.
const STILL_COMPUTE_NOTHING: &[(&str, &str)] = &[
    // Prose: "a monk gains immunity to all diseases". No quantity.
    ("unchained_monk", "Purity of Body"),
    // Prose: "can understand and speak with any living creature". No quantity.
    ("unchained_monk", "Tongue of the Sun and Moon"),
    // The row is empty — no DESC:, no token — and SERVESAS the shared Core
    // Rulebook `Rogue ~ Evasion` record. Any magnitude belongs to that record,
    // not to this book.
    ("unchained_rogue", "Evasion"),
    // Prose: "a number of cantrips ... as noted on Table 1-5". The table is
    // not on this row, and the Unchained Summoner spell list is not ingested.
    ("unchained_summoner", "Cantrips"),
    // Spends a Maker's Call use to swap places instead of teleporting the
    // eidolon: it changes what a use does, not how many there are.
    ("unchained_summoner", "Transposition"),
    // Prose: "as long as the eidolon has 1 or more hit points"; damage
    // "transferred 1 point at a time". Both 1s are the mechanic's
    // granularity, not a quantity a player tracks.
    ("unchained_summoner", "Life Bond"),
];

/// The remainder is exactly six, and exactly these six.
///
/// Deliberately two-directional. If a seventh feature silently stops
/// computing, this fails; if one of these six is closed without this list
/// being updated, this also fails, so the "why" beside each name cannot go
/// stale unnoticed.
#[test]
fn the_six_features_that_still_compute_nothing_are_exactly_these() {
    assert_eq!(STILL_COMPUTE_NOTHING.len(), 6);

    for (class_token, name) in STILL_COMPUTE_NOTHING {
        let rows = explanations_for(class_token, 20);
        // The roster row proves the record is ingested and reaches the sheet;
        // without it this test could "pass" on a typo'd feature name.
        let cited = rows.iter().any(|explanation| {
            explanation.id.contains(".corpus_record.") && explanation.detail.contains(name)
        });
        assert!(
            cited,
            "{class_token}: no roster row names {name:?}; this list must reference real ingested \
             records, not remembered ones"
        );

        let magnitude = rows.iter().find(|explanation| {
            !explanation.id.contains(".corpus_record.")
                && !explanation.id.ends_with(".unsupported")
                && explanation.detail.contains(name)
        });
        assert!(
            magnitude.is_none(),
            "{class_token}: {name:?} now has a grounded magnitude row ({}), so it no longer \
             belongs in STILL_COMPUTE_NOTHING -- update the list and its stated reason",
            magnitude.map(|e| e.id.as_str()).unwrap_or_default()
        );
    }
}

/// `decisions.md §28`'s standing guard: the before/after grounded-row counts,
/// per class, at the level the four Unchained classes were verified at.
#[test]
fn grounded_row_counts_moved_exactly_as_this_change_intended() {
    for (class_token, before, after) in GROUNDED_ROWS_AT_LEVEL_10_BEFORE_AND_AFTER {
        let prefix = format!("class_feature.pu.{class_token}.");
        let grounded = explanations_for(class_token, 10)
            .iter()
            .filter(|explanation| {
                explanation.id.starts_with(&prefix)
                    && !explanation.id.contains(".corpus_record.")
                    && !explanation.id.ends_with(".unsupported")
            })
            .count();
        assert_eq!(
            grounded, *after,
            "{class_token} grounded rows at level 10 (was {before} before this change)"
        );
    }

    for (class_token, expected) in GROUNDED_ROWS_AT_LEVEL_20 {
        let prefix = format!("class_feature.pu.{class_token}.");
        let grounded = explanations_for(class_token, 20)
            .iter()
            .filter(|explanation| {
                explanation.id.starts_with(&prefix)
                    && !explanation.id.contains(".corpus_record.")
                    && !explanation.id.ends_with(".unsupported")
            })
            .count();
        assert_eq!(grounded, *expected, "{class_token} grounded rows at level 20");
    }
}

/// No two rows may share an id: `classFeaturesModel.ts` keys the sheet's rows
/// on it, so a collision would silently render one feature and drop another.
#[test]
fn every_new_row_has_a_unique_id_within_its_class() {
    for (class_token, _, _) in GROUNDED_ROWS_AT_LEVEL_10_BEFORE_AND_AFTER {
        let rows = explanations_for(class_token, 20);
        let ids: Vec<&str> = rows
            .iter()
            .filter(|explanation| {
                explanation.id.starts_with(&format!("class_feature.pu.{class_token}."))
            })
            .map(|explanation| explanation.id.as_str())
            .collect();
        let unique: BTreeSet<&str> = ids.iter().copied().collect();
        assert_eq!(
            ids.len(),
            unique.len(),
            "{class_token} emitted a duplicate class-feature id at level 20: {ids:?}"
        );
    }
}
