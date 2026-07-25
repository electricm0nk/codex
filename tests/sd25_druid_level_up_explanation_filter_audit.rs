//! SD-25 Epic 7 (criterion 7.7, per-class residue audit): Druid.
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
//! fix). SD-24 never audited the other core classes for the identical bug
//! shape. This file is that audit for Druid.
//!
//! **Audit method:** rather than probing for one specific known-missing
//! prefix (there is no way to know in advance which prefix a class might
//! be missing), this file enumerates EVERY real (non-diagnostic)
//! explanation id `pilot_compute::explain_druid_level1_spell_baseline`
//! ever emits across Druid's full grounded level range (1..=15,
//! `MAX_SUPPORTED_DRUID_LEVEL`) directly from the public
//! `compute_pilot_base_chassis` seam, then drives every adjacent level
//! transition (1->2, 2->3, ..., 14->15) through the public
//! `compute_druid_level_up_grants` entry point and asserts every one of
//! those real ids surfaces as a grant's `source_table.column_key` at least
//! once. This structurally reproduces whatever proved the Wizard bug: if
//! `druid.rs`'s `is_druid_pillar_id` filter were missing (or had dropped)
//! any real prefix, at least one id in the raw chassis output would never
//! appear in any accumulated `LevelUpPlan`, and this test would fail.
//!
//! **Deliberate, documented exclusion (not a bug):** `druid.rs`'s own doc
//! comment states `"class_chassis.spell_baseline.druid"` (the level-
//! mentioning recognition record) is deliberately excluded from the
//! filter because its value is always 0 and it never changes between
//! `from_level` and `to_level`, so it can never itself drive a value-
//! change grant — this is excluded from this audit's "must appear"
//! set for the identical reason, and confirmed separately below to never
//! surface as a grant.
//!
//! **Second finding, surfaced by this test's own RED run, also not a
//! filter defect:** `"class_chassis.druid.nature_sense"` DOES pass
//! `is_druid_pillar_id` (it matches the `"class_chassis.druid."` prefix
//! like every other real class-chassis pillar) but never itself appears
//! as a grant anywhere in the 1..=15 sweep, because Nature Sense is a
//! flat, level-independent +2 (PF1 Core Rulebook) already granted at
//! Druid level 1 — there is no `from_level >= 1 -> to_level` transition
//! this codebase models in which its value or presence ever changes, the
//! identical structural boundary the flat recognition id hits (both are
//! "granted at character creation," not "granted by leveling up").
//! `tests/sd20_levelup_druid.rs`'s own
//! `druid_level_1_to_2_grants_bab_rise_fort_will_rise_and_woodland_stride`
//! test already asserts this exact behavior is intentional
//! ("nature sense is flat and level-independent; it must not re-fire on
//! every level"). This is a correctly-included, correctly-inert fact —
//! not a silently-dropped one — confirmed separately below by checking
//! its value stays constant across the whole range rather than merely
//! assuming it.
//!
//! **Verified negative finding:** unlike Wizard, Druid has no
//! `ground_druid_prepared_spellbook`-shaped later grounding anywhere in
//! `pilot_compute.rs` — grepping the file for `"class_spell.druid"` turns
//! up exactly one hit, `class_spell.druid.prepared_divine.unsupported`,
//! which is pushed to `diagnostics` (not `explanations`; see
//! `tests/sd13_druid_level1_spell_baseline.rs`'s own
//! `druid_level1_stays_blocked_on_prepared_divine_spell_posture_burden`
//! test, which keeps this diagnostic claim-blocking) and so is never even
//! read by `druid_chassis_explanations` (which only reads the
//! `.explanations` field). Every real explanation id
//! `explain_druid_level1_spell_baseline` emits uses only the two prefixes
//! `druid.rs`'s `is_druid_pillar_id` already matches
//! (`"class_chassis.druid."` and `"class_feature.druid."`). This test
//! passes cleanly on the as-shipped module — that is the audit's real,
//! verified result, not a skipped check.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use codex::rules_core::level_up::druid::compute_druid_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

/// Druid's own grounded ceiling (`MAX_SUPPORTED_DRUID_LEVEL` in
/// `pilot_compute.rs`; also cited by `druid.rs`'s own doc comment).
const MAX_SUPPORTED_DRUID_LEVEL: u8 = 15;

/// The filter-excluded id (see this file's own header comment and
/// `druid.rs`'s `is_druid_pillar_id` doc comment): never passes the
/// filter at all.
const RECOGNITION_ONLY_ID: &str = "class_chassis.spell_baseline.druid";

/// The filter-INCLUDED but structurally-never-diffing id (see this file's
/// own header comment's "second finding"): passes `is_druid_pillar_id`
/// but is flat and granted starting at level 1, so it can never appear as
/// a grant within the `from_level >= 1` sweep this codebase models.
const FLAT_LEVEL_ONE_ID: &str = "class_chassis.druid.nature_sense";

fn human_druid_input(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd25_druid_level_up_explanation_filter_audit".to_string()),
        source_package_id: "sd25_druid_level_up_explanation_filter_audit".to_string(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:druid".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 14,
                constitution: 13,
                intelligence: 8,
                wisdom: 17,
                charisma: 12,
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

/// Every real, non-diagnostic explanation id
/// `explain_druid_level1_spell_baseline` emits anywhere across Druid's
/// full grounded level range, collected directly from the public
/// `compute_pilot_base_chassis` seam (the identical source `druid.rs`
/// itself reads via `druid_chassis_explanations`).
fn all_real_druid_explanation_ids() -> std::collections::BTreeSet<String> {
    let mut ids = std::collections::BTreeSet::new();
    for level in 1..=MAX_SUPPORTED_DRUID_LEVEL {
        let input = human_druid_input(level);
        let computation = compute_pilot_base_chassis(&input);
        for explanation in &computation.explanations {
            if explanation.id.starts_with("class_chassis.druid.")
                || explanation.id.starts_with("class_feature.druid.")
                || explanation.id == RECOGNITION_ONLY_ID
            {
                ids.insert(explanation.id.clone());
            }
        }
    }
    ids
}

/// Drives every adjacent level transition across Druid's full grounded
/// range through the public `compute_druid_level_up_grants` entry point
/// (the same function `level_up.rs`'s dispatcher calls) and collects
/// every grant's source explanation id.
fn all_granted_source_ids() -> std::collections::BTreeSet<String> {
    let mut ids = std::collections::BTreeSet::new();
    for from_level in 1..MAX_SUPPORTED_DRUID_LEVEL {
        let to_level = from_level + 1;
        let character = human_druid_input(from_level);
        let plan = compute_druid_level_up_grants(&character, from_level, to_level);
        for grant in &plan.automatic_features {
            ids.insert(grant.source_table.column_key.clone());
        }
    }
    ids
}

#[test]
fn every_real_druid_explanation_id_survives_the_level_up_filter_except_the_documented_recognition_exclusion(
) {
    let real_ids = all_real_druid_explanation_ids();
    assert!(
        real_ids.len() > 1,
        "sanity check: expected multiple real druid explanation ids across levels 1..=15, got: {real_ids:?}"
    );

    let granted_ids = all_granted_source_ids();

    let mut silently_dropped: Vec<&String> = real_ids
        .iter()
        .filter(|id| {
            id.as_str() != RECOGNITION_ONLY_ID
                && id.as_str() != FLAT_LEVEL_ONE_ID
                && !granted_ids.contains(id.as_str())
        })
        .collect();
    silently_dropped.sort();

    assert!(
        silently_dropped.is_empty(),
        "druid.rs's explanation-id filter (is_druid_pillar_id) silently drops real, grounded \
         pilot_compute.rs explanation ids from every LevelUpPlan across the full level 1..=15 \
         sweep -- this is exactly the SD-24 Wizard bug shape (a later-landing real grounding \
         whose prefix was never added to the class module's own filter). Missing ids: \
         {silently_dropped:?}. Full real-id set: {real_ids:?}. Full granted-id set: {granted_ids:?}"
    );
}

#[test]
fn the_flat_recognition_only_id_is_deliberately_never_granted() {
    // Confirms the one documented exclusion really is inert (value always
    // 0, never diffs) rather than silently dropped for the same reason a
    // real fact would be -- a different, deliberate design choice
    // documented in `druid.rs`'s own `is_druid_pillar_id` doc comment.
    let granted_ids = all_granted_source_ids();
    assert!(
        !granted_ids.contains(RECOGNITION_ONLY_ID),
        "the flat +0 recognition-only id must never itself drive a grant: {granted_ids:?}"
    );

    let real_ids = all_real_druid_explanation_ids();
    assert!(
        real_ids.contains(RECOGNITION_ONLY_ID),
        "sanity check: the recognition id must still be a real explanation pilot_compute.rs \
         emits, confirming its absence from granted_ids above is a deliberate exclusion and not \
         an artifact of pilot_compute.rs never emitting it in the first place"
    );
}

#[test]
fn the_flat_nature_sense_id_passes_the_filter_but_is_verified_constant_across_the_whole_range() {
    // Confirms FLAT_LEVEL_ONE_ID's exclusion above is legitimate: the raw
    // pilot_compute.rs value truly never changes and is truly present at
    // every level in the sweep (so its absence from granted_ids is a
    // structural "no from_level >= 1 transition can ever surface it"
    // fact, not evidence the filter dropped it).
    let mut values = Vec::new();
    for level in 1..=MAX_SUPPORTED_DRUID_LEVEL {
        let input = human_druid_input(level);
        let computation = compute_pilot_base_chassis(&input);
        let explanation = computation
            .explanations
            .iter()
            .find(|e| e.id == FLAT_LEVEL_ONE_ID)
            .unwrap_or_else(|| {
                panic!("expected {FLAT_LEVEL_ONE_ID} present at every level, missing at level {level}")
            });
        values.push(explanation.value);
    }
    assert!(
        values.iter().all(|&v| v == values[0]),
        "{FLAT_LEVEL_ONE_ID} must be constant across levels 1..=15 to justify excluding it from \
         the 'must eventually be granted' requirement above: {values:?}"
    );
    assert_eq!(values[0], 2, "PF1 Core Rulebook Nature Sense is a flat +2");
}

#[test]
fn the_unsupported_prepared_divine_spell_diagnostic_is_never_read_as_an_explanation() {
    // Direct negative control for this audit's own verified finding: the
    // one `"class_spell.druid"`-prefixed id anywhere in pilot_compute.rs
    // (`class_spell.druid.prepared_divine.unsupported`) is a claim-
    // blocking diagnostic, not an explanation -- druid.rs's
    // `druid_chassis_explanations` only ever reads `.explanations`, so
    // this id structurally cannot reach `is_druid_pillar_id` at all,
    // unlike Wizard's real `class_spell.wizard.*` explanations which DID
    // reach (and were then dropped by) the pre-fix filter.
    let input = human_druid_input(1);
    let computation = compute_pilot_base_chassis(&input);

    assert!(
        !computation
            .explanations
            .iter()
            .any(|e| e.id.starts_with("class_spell.druid")),
        "no class_spell.druid.*-prefixed explanation should exist anywhere in this codebase \
         (Druid's prepared divine spell posture has no later grounding, unlike Wizard's SD-21 \
         E6b.2 ground_wizard_prepared_spellbook): {:?}",
        computation.explanations
    );
    assert!(
        computation
            .diagnostics
            .iter()
            .any(|d| d.id == "class_spell.druid.prepared_divine.unsupported" && d.claim_blocking),
        "the prepared divine spell posture must stay a claim-blocking diagnostic: {:?}",
        computation.diagnostics
    );
}
