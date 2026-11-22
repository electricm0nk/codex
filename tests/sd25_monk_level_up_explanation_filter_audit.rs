//! SD-25 Epic 7 (criterion 7.8, per-class residue audit): Monk.
//!
//! SD-24 Epic 4 (criterion 4.1) found and fixed a defect in Wizard's
//! `src/rules_core/level_up/wizard.rs`: a real, later-landing grounding
//! (`ground_wizard_prepared_spellbook`, SD-21 E6b.2) added a new
//! `class_spell.wizard.*`-prefixed family of explanation ids to
//! `pilot_compute.rs`'s Wizard chassis output, but `wizard.rs`'s own
//! explanation-id filter (authored before that grounding landed) was never
//! widened to include the new prefix — so every one of those real facts
//! was silently dropped from the `LevelUpPlan` even once the posture was
//! fully met (`tests/sd24_wizard_level_up_spell_coverage.rs` proves the
//! fix). SD-25's Epic 7 residue backlog (register A6) queued the other 9
//! CRB classes for the identical audit. This file is that audit for Monk
//! (progress.md's 7.1 intake sub-row: "filter admits `class_chassis.monk.`
//! / `class_feature.monk.` only. No `class_spell.monk.*` id exists in
//! `pilot_compute.rs` (non-caster) — audit should confirm no live gap
//! exists.").
//!
//! **Audit method (mirrors `tests/sd25_druid_level_up_explanation_filter_audit.rs`
//! and the Bard/Sorcerer/Barbarian precedents it cites):** rather than
//! probing for one specific known-missing prefix, this file enumerates
//! EVERY real (non-diagnostic) explanation id
//! `pilot_compute::explain_monk_level1_chassis` ever emits across Monk's
//! full grounded level range (1..=20, `MAX_SUPPORTED_MONK_LEVEL` -- widened
//! from 12 to 20 by task #49, 2026-07-28) directly
//! from the public `compute_pilot_base_chassis` seam, then drives every
//! adjacent level transition (1->2, 2->3, ..., 19->20) through the public
//! `compute_monk_level_up_grants` entry point and asserts every one of
//! those real ids surfaces SOMEWHERE in the resulting `LevelUpPlan`
//! (either as an `automatic_features` grant's source id, or — for the ki
//! pool, which is a resource-count magnitude rather than a discrete grant,
//! mirroring Barbarian's rage-rounds-per-day idiom — as a
//! `resource_pool_change` entry's source id) at least once across the
//! sweep. This structurally reproduces whatever proved the Wizard bug: if
//! `monk.rs`'s `is_monk_pillar_id` filter were missing (or had dropped)
//! any real prefix, at least one id in the raw chassis output would never
//! appear anywhere in any accumulated `LevelUpPlan`, and this test would
//! fail.
//!
//! **Verified negative finding:** unlike Wizard (a spellcaster whose
//! spellbook grounding landed as a genuinely later, separately-prefixed
//! family of ids) and unlike Bard (whose recognition id used a
//! `class_chassis.spell_baseline.bard` shape that didn't match either of
//! its two admitted prefixes), Monk is not a spellcaster at all — grepping
//! `pilot_compute.rs` for `"class_spell.monk"` returns zero hits anywhere
//! in the source tree (confirmed by this file's own
//! `no_class_spell_monk_prefixed_id_exists_anywhere` test below), so there
//! is no later-landing spell-family grounding that could have been missed
//! in the first place. Every real explanation id
//! `explain_monk_level1_chassis` emits uses one of the two prefixes
//! `monk.rs`'s `is_monk_pillar_id` already matches
//! (`"class_chassis.monk."` and `"class_feature.monk."`), INCLUDING the
//! module's own recognition id (`"class_chassis.monk.bounded_progression"`)
//! — unlike Bard's/Wizard's/Sorcerer's spell-baseline recognition ids,
//! Monk's recognition id's second dotted segment is already `monk`, so it
//! was never at risk of the Bard-shaped "different second segment" bug
//! either. This test passes cleanly on the as-shipped module — that is the
//! audit's real, verified result, not a skipped check. `monk.rs` is left
//! unmodified by this cycle (per the brief: "do not invent a fix for a bug
//! that isn't there").
//!
//! **Three documented, verified exclusions from the "must eventually
//! surface" requirement** (none is a silent drop — each is confirmed
//! separately below):
//!
//! 1. `BOUNDED_PROGRESSION_RECOGNITION_ID` — a flat +0 recognition record,
//!    present and unchanged at every supported level from Monk level 1
//!    onward (mirrors Druid's own flat `nature_sense` finding): it DOES
//!    pass `is_monk_pillar_id`, but its value never changes between
//!    `from_level` and `to_level` anywhere in the 1..=12 sweep, so it can
//!    never itself drive a value-change grant.
//!
//! 1a. **RESOLVED, no longer an exclusion** (`AT-33-E5-remainder-charbuild`,
//!    2026-08-25): this audit's own first RED run found `AC_BONUS_ID`
//!    (`"class_chassis.monk.ac_bonus"`) flat because `pilot_compute.rs`
//!    grounded only `ability_modifiers.wisdom.max(0)`, omitting the real
//!    PF1 Monk table's level-4+ dodge-bonus progression ("every four
//!    levels thereafter... increases by a further +1"). SD-33 grounded
//!    that progression (`monk_ac_bonus_dodge_progression`, confirmed
//!    against the pinned oracle's real L20 export), so the id is no
//!    longer level-independent: it now legitimately changes at levels
//!    3->4, 7->8, 11->12, 15->16 and 19->20, and this audit's own filter
//!    correctly surfaces every one of those five transitions — verified
//!    below, not assumed, the same way exclusion #1a's flatness was
//!    originally verified rather than assumed.
//! 2. `CLASS_TABLE_COVERED_IDS` (the four base-attack/base-save ids) —
//!    `monk.rs`'s own `is_covered_elsewhere` check deliberately excludes
//!    these from `append_class_feature_grants` because
//!    `append_class_table_grants` already grants the identical fact under
//!    a DIFFERENT, unprefixed `column_key`
//!    (`"base_attack_bonus"`/`"fort_save"`/`"ref_save"`/`"will_save"`, from
//!    `class_tables()` rather than from `pilot_compute.rs`). This is a
//!    real fact reported once under a different provenance, not dropped;
//!    confirmed below by checking the plain-named column key IS granted
//!    every time the dotted id's underlying value changes.
//! 3. The four numbered `bonus_feat*_choice` ids — conditionally pushed
//!    only when a `choice:monk_bonus_feat*` selection is present in
//!    `selected_choices`; this audit's fixture (mirroring
//!    `tests/sd20_levelup_monk.rs`'s own deliberately-empty
//!    `selected_choices`) never sets one, so these ids never enter the raw
//!    explanation set collected here in the first place — there is
//!    nothing for the filter to have dropped.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::monk::compute_monk_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

/// Monk's own grounded ceiling (`MAX_SUPPORTED_MONK_LEVEL` in
/// `pilot_compute.rs`; also cited by `monk.rs`'s own doc comment). Widened
/// from 12 to 20 by task #49 (2026-07-28, the full PF1 Core Rulebook
/// capstone range) -- re-run against the widened range rather than assumed
/// still-clean, since a class module's explanation-id filter is exactly
/// what SD-24 Epic 4 found silently dropping a later-landing grounding for
/// Wizard.
const MAX_SUPPORTED_MONK_LEVEL: u8 = 20;

/// The filter-INCLUDED but structurally-never-diffing recognition id (see
/// this file's own header comment's exclusion #1): passes
/// `is_monk_pillar_id` but is flat (value 0) and granted starting at
/// level 1, so it can never appear as a grant within the `from_level >= 1`
/// sweep this codebase models.
const BOUNDED_PROGRESSION_RECOGNITION_ID: &str = "class_chassis.monk.bounded_progression";

/// The AC Bonus id (see this file's own header comment's exclusion #1a,
/// RESOLVED): passes `is_monk_pillar_id` and, since SD-33's
/// `AT-33-E5-remainder-charbuild` grounded the level-4+ dodge-bonus
/// progression, its value genuinely changes at levels 3->4, 7->8, 11->12,
/// 15->16 and 19->20 for a fixed-ability-score character — no longer a
/// flat, filter-exempt id.
const AC_BONUS_ID: &str = "class_chassis.monk.ac_bonus";

/// The four ids `monk.rs`'s own `CLASS_TABLE_COVERED_EXPLANATION_IDS`
/// deliberately excludes from `append_class_feature_grants` because
/// `append_class_table_grants` already grants the identical fact under a
/// different, unprefixed column key (exclusion #2 above).
const CLASS_TABLE_COVERED_IDS: [&str; 4] = [
    "class_chassis.monk.base_attack_bonus",
    "class_chassis.monk.base_save.fortitude",
    "class_chassis.monk.base_save.reflex",
    "class_chassis.monk.base_save.will",
];

/// The plain, unprefixed column keys `append_class_table_grants` actually
/// grants each `CLASS_TABLE_COVERED_IDS` fact under (same order).
const CLASS_TABLE_COLUMN_KEYS: [&str; 4] =
    ["base_attack_bonus", "fort_save", "ref_save", "will_save"];

/// SD-34 bucket-B batch cycle: `class_feature_grant_consumer.rs`'s own
/// class-wide Monk exclusion (`LEVEL_UP_PILLAR_FILTERED_CLASSES`) was
/// removed, so `push_generic_class_feature_grant_records` now emits new
/// `class_feature.monk.corpus_record.*` ids for real, citation-backed grant
/// facts. Most (Abundant Step, Diamond Soul, Maneuver Training, Perfect
/// Self, Quivering Palm) are granted above level 1 and correctly surface as
/// real grants at their own granted-at transition (proven by this file's
/// own exhaustiveness test needing no exclusion for them). These three are
/// granted AT level 1 -- Flurry of Blows, Stunning Fist, Unarmed Strike --
/// so the character already has each one at the very first level modeled,
/// the identical structural shape `BOUNDED_PROGRESSION_RECOGNITION_ID`
/// already established: `is_monk_pillar_id` correctly passes all three
/// through (proven live, not merely by prefix, in
/// `every_real_monk_explanation_id_survives_the_level_up_filter_except_the_documented_exclusions`'s
/// own real-id set), they simply have no `from_level >= 1` transition to
/// surface a grant on. Verified constant, not assumed, by
/// `the_new_level_one_corpus_record_grant_facts_are_verified_constant_across_the_whole_range`
/// below.
const NEW_LEVEL_ONE_CORPUS_RECORD_GRANT_FACT_IDS: [&str; 3] = [
    "class_feature.monk.corpus_record.flurry_of_blows",
    "class_feature.monk.corpus_record.stunning_fist",
    "class_feature.monk.corpus_record.unarmed_strike",
];

fn human_monk_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd25_monk_level_up_explanation_filter_audit".to_string()),
        source_package_id: "sd25_monk_level_up_explanation_filter_audit".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:monk".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 13,
                dexterity: 16,
                constitution: 13,
                intelligence: 8,
                wisdom: 17,
                charisma: 10,
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

/// Every real, non-diagnostic explanation id `explain_monk_level1_chassis`
/// emits anywhere across Monk's full grounded level range, collected
/// directly from the public `compute_pilot_base_chassis` seam (the
/// identical source `monk.rs` itself reads via `monk_chassis_explanations`).
fn all_real_monk_explanation_ids() -> std::collections::BTreeSet<String> {
    let mut ids = std::collections::BTreeSet::new();
    for level in 1..=MAX_SUPPORTED_MONK_LEVEL {
        let input = human_monk_input(level);
        let computation = compute_pilot_base_chassis(&input);
        for explanation in &computation.explanations {
            if explanation.id.starts_with("class_chassis.monk.")
                || explanation.id.starts_with("class_feature.monk.")
            {
                ids.insert(explanation.id.clone());
            }
        }
    }
    ids
}

/// Drives every adjacent level transition across Monk's full grounded
/// range through the public `compute_monk_level_up_grants` entry point
/// (the same function `level_up.rs`'s dispatcher calls) and collects
/// every grant's AND every resource-pool-delta's source explanation id —
/// both are real surfaces the `LevelUpPlan` exposes to a caller, so
/// either counts as "not silently dropped."
fn all_surfaced_source_ids() -> std::collections::BTreeSet<String> {
    let mut ids = std::collections::BTreeSet::new();
    for from_level in 1..MAX_SUPPORTED_MONK_LEVEL {
        let to_level = from_level + 1;
        let character = human_monk_input(from_level);
        let plan = compute_monk_level_up_grants(&character, from_level, to_level);
        for grant in &plan.automatic_features {
            ids.insert(grant.source_table.column_key.clone());
        }
        for pool in &plan.resource_pool_change.pools {
            ids.insert(pool.source_table.column_key.clone());
        }
    }
    ids
}

#[test]
fn every_real_monk_explanation_id_survives_the_level_up_filter_except_the_documented_exclusions()
{
    let real_ids = all_real_monk_explanation_ids();
    assert!(
        real_ids.len() > 5,
        "sanity check: expected multiple real monk explanation ids across levels 1..=12, got: \
         {real_ids:?}"
    );

    let surfaced_ids = all_surfaced_source_ids();

    let mut silently_dropped: Vec<&String> = real_ids
        .iter()
        .filter(|id| {
            id.as_str() != BOUNDED_PROGRESSION_RECOGNITION_ID
                && !CLASS_TABLE_COVERED_IDS.contains(&id.as_str())
                && !NEW_LEVEL_ONE_CORPUS_RECORD_GRANT_FACT_IDS.contains(&id.as_str())
                && !surfaced_ids.contains(id.as_str())
        })
        .collect();
    silently_dropped.sort();

    assert!(
        silently_dropped.is_empty(),
        "monk.rs's explanation-id filter (is_monk_pillar_id) silently drops real, grounded \
         pilot_compute.rs explanation ids from every LevelUpPlan across the full level 1..=12 \
         sweep -- this is exactly the SD-24 Wizard bug shape (a later-landing real grounding \
         whose prefix was never added to the class module's own filter). Missing ids: \
         {silently_dropped:?}. Full real-id set: {real_ids:?}. Full surfaced-id set: \
         {surfaced_ids:?}"
    );
}

#[test]
fn the_flat_bounded_progression_recognition_id_is_deliberately_never_granted() {
    // Confirms exclusion #1 really is inert (value always 0, never diffs)
    // rather than silently dropped for the same reason a real fact would
    // be -- a different, structural reason documented in this file's own
    // header comment.
    let surfaced_ids = all_surfaced_source_ids();
    assert!(
        !surfaced_ids.contains(BOUNDED_PROGRESSION_RECOGNITION_ID),
        "the flat +0 recognition-only id must never itself drive a grant or resource-pool delta: \
         {surfaced_ids:?}"
    );

    let real_ids = all_real_monk_explanation_ids();
    assert!(
        real_ids.contains(BOUNDED_PROGRESSION_RECOGNITION_ID),
        "sanity check: the recognition id must still be a real explanation pilot_compute.rs \
         emits, confirming its absence from surfaced_ids above is a deliberate structural \
         exclusion and not an artifact of pilot_compute.rs never emitting it in the first place"
    );

    // And confirm it is genuinely constant (value 0) at every level, not
    // merely absent from grants by coincidence.
    for level in 1..=MAX_SUPPORTED_MONK_LEVEL {
        let input = human_monk_input(level);
        let computation = compute_pilot_base_chassis(&input);
        let explanation = computation
            .explanations
            .iter()
            .find(|e| e.id == BOUNDED_PROGRESSION_RECOGNITION_ID)
            .unwrap_or_else(|| {
                panic!(
                    "expected {BOUNDED_PROGRESSION_RECOGNITION_ID} present at every level, \
                     missing at level {level}"
                )
            });
        assert_eq!(
            explanation.value, 0,
            "{BOUNDED_PROGRESSION_RECOGNITION_ID} must stay flat at value 0 at level {level}"
        );
    }
}

#[test]
fn the_ac_bonus_id_now_legitimately_surfaces_the_level_4_plus_dodge_progression() {
    // Confirms exclusion #1a is RESOLVED: the raw pilot_compute.rs value
    // for a fixed-ability-score character (WIS 17 -> +3 modifier) steps at
    // exactly the five real PF1 Monk table breakpoints (4, 8, 12, 16, 20)
    // and is otherwise flat -- verified per-level below, not assumed.
    let expected_by_level: [(u8, i16); 20] = [
        (1, 3),
        (2, 3),
        (3, 3),
        (4, 4),
        (5, 4),
        (6, 4),
        (7, 4),
        (8, 5),
        (9, 5),
        (10, 5),
        (11, 5),
        (12, 6),
        (13, 6),
        (14, 6),
        (15, 6),
        (16, 7),
        (17, 7),
        (18, 7),
        (19, 7),
        (20, 8),
    ];
    for (level, expected_value) in expected_by_level {
        let input = human_monk_input(level);
        let computation = compute_pilot_base_chassis(&input);
        let explanation = computation
            .explanations
            .iter()
            .find(|e| e.id == AC_BONUS_ID)
            .unwrap_or_else(|| {
                panic!("expected {AC_BONUS_ID} present at every level, missing at level {level}")
            });
        assert_eq!(
            explanation.value, expected_value,
            "{AC_BONUS_ID} at level {level}: WIS +3 plus the level-4+ dodge progression"
        );
    }

    // The five real value-change transitions this progression creates.
    const DODGE_STEP_TRANSITIONS: [u8; 5] = [3, 7, 11, 15, 19];

    let surfaced_ids = all_surfaced_source_ids();
    assert!(
        surfaced_ids.contains(AC_BONUS_ID),
        "the AC Bonus id now genuinely changes with level and must surface via the level-up \
         filter (this is the exact SD-24 Wizard bug shape -- a real, later-landing grounding \
         whose value-change the filter must not silently drop): {surfaced_ids:?}"
    );

    // Per-transition proof: exactly the five documented breakpoints emit a
    // real AC_BONUS_ID grant or resource-pool-change entry with the
    // correct magnitude; every other adjacent transition emits none.
    for from_level in 1..MAX_SUPPORTED_MONK_LEVEL {
        let to_level = from_level + 1;
        let character = human_monk_input(from_level);
        let plan = compute_monk_level_up_grants(&character, from_level, to_level);
        let surfaced_here: Vec<i16> = plan
            .automatic_features
            .iter()
            .filter(|grant| grant.source_table.column_key == AC_BONUS_ID)
            .flat_map(|grant| grant.effects.iter().map(|effect| effect.value))
            .chain(
                plan.resource_pool_change
                    .pools
                    .iter()
                    .filter(|pool| pool.source_table.column_key == AC_BONUS_ID)
                    .map(|pool| pool.to_value - pool.from_value),
            )
            .collect();

        if DODGE_STEP_TRANSITIONS.contains(&from_level) {
            assert!(
                !surfaced_here.is_empty(),
                "level {from_level} -> {to_level} is a documented AC Bonus dodge-progression \
                 breakpoint but no {AC_BONUS_ID} grant or resource-pool-change surfaced: \
                 {plan:?}"
            );
        } else {
            assert!(
                surfaced_here.is_empty(),
                "level {from_level} -> {to_level} is NOT a documented AC Bonus dodge-progression \
                 breakpoint, but {AC_BONUS_ID} surfaced anyway (spurious/duplicate emission): \
                 {surfaced_here:?}"
            );
        }
    }
}

#[test]
fn the_class_table_covered_ids_are_reported_once_under_a_different_provenance_not_dropped() {
    // Confirms exclusion #2: every time the dotted pilot_compute.rs id's
    // underlying value changes between from_level and to_level, the SAME
    // fact is granted, just under the plain class_tables()-sourced column
    // key instead -- so the fact is reported once, not silently dropped.
    for from_level in 1..MAX_SUPPORTED_MONK_LEVEL {
        let to_level = from_level + 1;
        let character = human_monk_input(from_level);
        let plan = compute_monk_level_up_grants(&character, from_level, to_level);
        let surfaced_column_keys: std::collections::BTreeSet<&str> = plan
            .automatic_features
            .iter()
            .map(|grant| grant.source_table.column_key.as_str())
            .collect();

        let from_computation = compute_pilot_base_chassis(&character);
        let to_character = human_monk_input(to_level);
        let to_computation = compute_pilot_base_chassis(&to_character);

        for (dotted_id, plain_key) in CLASS_TABLE_COVERED_IDS.iter().zip(CLASS_TABLE_COLUMN_KEYS)
        {
            let from_value = from_computation
                .explanations
                .iter()
                .find(|e| &e.id == dotted_id)
                .map(|e| e.value);
            let to_value = to_computation
                .explanations
                .iter()
                .find(|e| &e.id == dotted_id)
                .map(|e| e.value);
            if from_value != to_value {
                assert!(
                    surfaced_column_keys.contains(plain_key),
                    "{dotted_id} changed ({from_value:?} -> {to_value:?}) at monk level \
                     {from_level} -> {to_level} but its class_tables()-sourced counterpart \
                     column key {plain_key:?} was not granted -- the fact would be silently \
                     dropped under BOTH provenances. Surfaced column keys: \
                     {surfaced_column_keys:?}"
                );
            }
        }
    }
}

#[test]
fn no_class_spell_monk_prefixed_id_exists_anywhere() {
    // Verified-negative-finding proof: Monk is not a spellcaster, so there
    // is no class_spell.monk.*-prefixed family (the exact shape of
    // Wizard's real, once-missed grounding) anywhere in the raw
    // explanation set this codebase ever produces for Monk.
    for level in 1..=MAX_SUPPORTED_MONK_LEVEL {
        let input = human_monk_input(level);
        let computation = compute_pilot_base_chassis(&input);
        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_spell.monk")),
            "no class_spell.monk.*-prefixed explanation should exist anywhere in this codebase \
             (Monk grounds no spell-bearing posture): {:?}",
            computation.explanations
        );
    }
}

#[test]
fn the_bonus_feat_unsupported_diagnostic_is_never_read_as_an_explanation() {
    // Direct negative control: the one
    // class_feature.monk.bounded_progression.bonus_feat.unsupported
    // diagnostic id is a claim-blocking ComputationDiagnostic, not a
    // ComputationExplanation -- monk.rs's monk_chassis_explanations only
    // ever reads .explanations, so this id structurally cannot reach
    // is_monk_pillar_id at all, unlike a real explanation.
    let input = human_monk_input(1);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id == "class_feature.monk.bounded_progression.bonus_feat.unsupported"),
        "the bonus-feat-unsupported diagnostic id must never appear in .explanations: {:?}",
        computation.explanations
    );
    assert!(
        computation.diagnostics.iter().any(|d| d.id
            == "class_feature.monk.bounded_progression.bonus_feat.unsupported"
            && d.claim_blocking),
        "the bonus-feat-unsupported id must stay a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}

/// SD-34 bucket-B batch cycle: confirms `NEW_LEVEL_ONE_CORPUS_RECORD_GRANT_FACT_IDS`'s
/// exclusion above is legitimate the same way `BOUNDED_PROGRESSION_RECOGNITION_ID`'s own proof
/// is -- each id is present at every level in the sweep (granted starting at level 1) and its
/// value never changes, so its absence from `surfaced_ids` is a structural "no `from_level >= 1`
/// transition can ever surface it" fact, not evidence `is_monk_pillar_id` or the widened
/// citation gate dropped it.
#[test]
fn the_new_level_one_corpus_record_grant_facts_are_verified_constant_across_the_whole_range() {
    for &id in NEW_LEVEL_ONE_CORPUS_RECORD_GRANT_FACT_IDS.iter() {
        let mut values = Vec::new();
        for level in 1..=MAX_SUPPORTED_MONK_LEVEL {
            let input = human_monk_input(level);
            let computation = compute_pilot_base_chassis(&input);
            let explanation = computation
                .explanations
                .iter()
                .find(|e| e.id == id)
                .unwrap_or_else(|| panic!("expected {id} present at every level, missing at level {level}"));
            values.push(explanation.value);
        }
        assert!(
            values.iter().all(|&v| v == values[0]),
            "{id} must be constant across levels 1..={MAX_SUPPORTED_MONK_LEVEL} to justify \
             excluding it from the 'must eventually be granted' requirement above: {values:?}"
        );
    }
}
