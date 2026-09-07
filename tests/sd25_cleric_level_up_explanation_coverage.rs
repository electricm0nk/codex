//! SD-25 Epic 7 (criterion 7.2 — per-class residue audit for Cleric).
//!
//! **Background**: SD-24 Epic 4 found and fixed a bug in
//! `src/rules_core/level_up/wizard.rs`: a later `pilot_compute.rs`
//! grounding (SD-21 E6b.2's `ground_wizard_prepared_spellbook`) landed
//! real `class_spell.wizard.*` explanations (spellbook contents, daily
//! preparation, per-spell-level spells-per-day) into
//! `compute_pilot_base_chassis`'s `.explanations` vector, but Wizard's own
//! `LevelUpPlan` explanation-id filter (`WIZARD_EXPLANATION_PREFIX =
//! "class_chassis.wizard."`) was never widened to admit the new
//! `class_spell.wizard.` prefix, so those real facts were silently dropped
//! from every `LevelUpPlan` even once a wizard's prepared-spellbook
//! posture was fully met (see `tests/sd24_wizard_level_up_spell_coverage.rs`).
//!
//! This module never audited the other 9 CRB classes for the identical
//! bug class. SD-25 Epic 7's 7.1 residue intake flagged Cleric as a
//! suspect (`progress.md`'s `## DISCOVERED` entry): Cleric's own filter
//! (`CLERIC_EXPLANATION_PREFIX = "class_chassis.cleric."`, plus the exact
//! `CLERIC_RECOGNITION_ID`) does not admit a `class_spell.cleric.` prefix,
//! and `pilot_compute.rs` does grind out an id literally named
//! `class_spell.cleric.prepared_divine.unsupported`.
//!
//! **Audit finding (verified, not assumed): this is NOT the Wizard bug
//! shape, and Cleric's `LevelUpPlan` filter has no live gap.** Wizard's
//! bug was possible because the dropped ids were real `ComputationExplanation`
//! records pushed onto `compute_pilot_base_chassis`'s `explanations` Vec —
//! the exact Vec `wizard_chassis_explanations`/`cleric_chassis_explanations`
//! read from. Cleric's `class_spell.cleric.prepared_divine.unsupported` is
//! instead a `ComputationDiagnostic` (`claim_blocking: true`), pushed onto
//! the entirely separate `diagnostics` Vec inside
//! `explain_cleric_level1_spell_baseline`
//! (`src/rules_core/pilot_compute.rs`) — never onto `explanations`.
//! `compute_pilot_base_chassis` keeps the two Vecs distinct all the way to
//! its returned `PilotBaseChassisComputation { explanations, diagnostics,
//! .. }`. `cleric_chassis_explanations` (this module's own probe helper)
//! reads only `.explanations`, so this diagnostic id is structurally
//! unreachable by `append_class_feature_grants` regardless of what prefix
//! `CLERIC_EXPLANATION_PREFIX` admits — widening the filter to include
//! `"class_spell.cleric."` would change nothing observable today (there is
//! no explanation carrying that prefix to admit), and doing so anyway
//! risks the OPPOSITE defect: a future refactor that accidentally started
//! pushing diagnostic-shaped "unsupported" markers through the explanation
//! path would then get silently fabricated into a `Grant`, which is
//! exactly the no-fabrication violation `cleric.rs`'s own doc comment (and
//! `AGENTS.md`) forbids. This audit's fix is therefore to make that
//! separation an explicit, permanent regression proof (this file), not to
//! change `cleric.rs`'s filter.
//!
//! This file's tests would catch a real Wizard-shaped regression: if any
//! future `pilot_compute.rs` change ever grounds a real Cleric explanation
//! under a prefix `cleric.rs`'s filter does not admit (the general form of
//! the Wizard bug), `every_changed_cleric_explanation_produces_a_grant`
//! fails, because it walks the FULL from/to explanation diff generically
//! (by id, not by a hand-picked allowlist) and asserts every changed,
//! filter-admitted id surfaces a matching `Grant`. Manually confirmed to
//! fail (RED) when `CLERIC_EXPLANATION_PREFIX` is narrowed to a strict
//! substring that excludes a real pillar (e.g. truncated to
//! `"class_chassis.cleric.c"`, which drops `base_attack_bonus` /
//! `base_save.*` / `domain_choice` / `domain_spell_slot` /
//! `domain_power_*`) during this cycle's own verification pass, then
//! reverted — see this cycle's receipt.

use codex::rules_core::character_input::CharacterInput;
use codex::rules_core::level_up::cleric::compute_cleric_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
mod common;
use common::load;

const CLERIC_LEVEL1_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_cleric_level1_sd13_deterministic_input.txt");

const CLERIC_RECOGNITION_ID: &str = "class_chassis.spell_baseline.cleric";
const CLERIC_EXPLANATION_PREFIX: &str = "class_chassis.cleric.";
const PREPARED_DIVINE_UNSUPPORTED_DIAGNOSTIC_ID: &str = "class_spell.cleric.prepared_divine.unsupported";

/// Builds a deterministic single-class Human Cleric input at `level`
/// (canonical Good + Healing domain selections), by swapping the level
/// token on the level-1 fixture -- the identical technique
/// `tests/sd13_cleric_level1_spell_baseline.rs`'s own `..._was_later_widened...`
/// tests use.
fn cleric_at_level(level: u8) -> CharacterInput {
    let text = CLERIC_LEVEL1_FIXTURE.replace("class:cleric:1", &format!("class:cleric:{level}"));
    load(&text)
}

fn is_cleric_pillar_id(id: &str) -> bool {
    id == CLERIC_RECOGNITION_ID || id.starts_with(CLERIC_EXPLANATION_PREFIX)
}

// ----- Positive proof: the filter drops nothing real, at any level transition -----

#[test]
fn every_changed_cleric_explanation_produces_a_grant() {
    // Every adjacent level-1..20 transition, the full supported range
    // (`MAX_SUPPORTED_CLERIC_LEVEL = 20`) -- exercises every threshold
    // where Channel Energy's die count, the domain spell slot count, and
    // the Good/Healing domain-power magnitudes are each individually
    // documented (in `pilot_compute.rs`) to change.
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = cleric_at_level(to_level);

        let from_computation = compute_pilot_base_chassis(&cleric_at_level(from_level));
        let to_computation = compute_pilot_base_chassis(&character);
        let plan = compute_cleric_level_up_grants(&character, from_level, to_level);

        for to_explanation in &to_computation.explanations {
            if !is_cleric_pillar_id(&to_explanation.id) {
                // Not a Cleric-owned pillar (e.g. class-independent ability
                // modifiers, Human race seam) -- out of this module's own
                // scope, exactly as `cleric.rs`'s own filter documents.
                continue;
            }

            let from_value = from_computation
                .explanations
                .iter()
                .find(|explanation| explanation.id == to_explanation.id)
                .map(|explanation| explanation.value);
            if from_value == Some(to_explanation.value) {
                // Genuinely unchanged at this transition -- not this test's
                // concern (Wizard's own bug only ever manifested on a
                // CHANGED value being silently dropped).
                continue;
            }

            let has_matching_grant = plan
                .automatic_features
                .iter()
                .any(|grant| grant.source_table.column_key == to_explanation.id);
            assert!(
                has_matching_grant,
                "cleric level {from_level} -> {to_level}: explanation '{}' changed from {:?} to \
                 {} but no matching Grant was found in the LevelUpPlan (the exact Wizard SD-24 \
                 bug shape -- a real grounded fact silently dropped by the explanation-id \
                 filter). Grants present: {:#?}",
                to_explanation.id, from_value, to_explanation.value, plan.automatic_features
            );
        }
    }
}

/// Names the concrete real pillars this audit confirms stay covered, so a
/// future accidental narrowing of `CLERIC_EXPLANATION_PREFIX` (not just a
/// missing widening) is also caught by name, mirroring
/// `tests/sd24_wizard_level_up_spell_coverage.rs`'s own named-suffix
/// assertions.
#[test]
fn known_cleric_pillars_each_produce_a_grant_across_their_own_documented_threshold() {
    let has_grant_from_id = |from_level: u8, to_level: u8, id: &str| {
        let character = cleric_at_level(to_level);
        let plan = compute_cleric_level_up_grants(&character, from_level, to_level);
        plan.automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == id)
    };

    // Channel Energy's die count rises at level 3 (ceil(3/2) = 2, up from
    // ceil(2/2) = 1) -- per pilot_compute.rs's own documented odd-level cadence.
    assert!(
        has_grant_from_id(2, 3, "class_chassis.cleric.channel_energy_dice"),
        "channel_energy_dice must produce a grant on the level 2 -> 3 rise"
    );

    // The domain spell slot count rises at level 3 (2nd-level cleric spells
    // first become accessible).
    assert!(
        has_grant_from_id(2, 3, "class_chassis.cleric.domain_spell_slot"),
        "domain_spell_slot must produce a grant on the level 2 -> 3 rise"
    );

    // The Good domain's Touch of Good sacred bonus rises at level 4 (half
    // cleric level, minimum 1: max(4/2, 1) = 2, up from max(3/2, 1) = 1).
    assert!(
        has_grant_from_id(
            3,
            4,
            "class_feature.domain.good_touch_of_good_bonus"
        ),
        "domain_power_good_touch_of_good_bonus must produce a grant on the level 3 -> 4 rise"
    );

    // Base attack bonus and all three base saves each rise somewhere across
    // the supported level range (per pilot_compute.rs's own grounded
    // formulas) -- found dynamically (each column's own real cadence
    // differs: BAB and the two good saves change most levels, the single
    // poor save changes on a slower cadence) rather than assumed at a
    // hand-picked level, so this stays a genuine proof rather than a
    // coincidental pass.
    for column in [
        "class_chassis.cleric.base_attack_bonus",
        "class_chassis.cleric.base_save.fortitude",
        "class_chassis.cleric.base_save.reflex",
        "class_chassis.cleric.base_save.will",
    ] {
        let mut found_a_changed_transition_with_a_grant = false;
        for from_level in 1..20u8 {
            let to_level = from_level + 1;
            let from_computation = compute_pilot_base_chassis(&cleric_at_level(from_level));
            let to_computation = compute_pilot_base_chassis(&cleric_at_level(to_level));
            let from_value = from_computation
                .explanations
                .iter()
                .find(|e| e.id == column)
                .map(|e| e.value);
            let to_value = to_computation
                .explanations
                .iter()
                .find(|e| e.id == column)
                .map(|e| e.value);
            if from_value == to_value {
                continue;
            }
            if has_grant_from_id(from_level, to_level, column) {
                found_a_changed_transition_with_a_grant = true;
                break;
            }
        }
        assert!(
            found_a_changed_transition_with_a_grant,
            "'{column}' must change at some level 1-20 transition and produce a matching Grant \
             there -- found no such transition"
        );
    }
}

// ----- Verified negative finding: the "unsupported" diagnostic is correctly excluded -----

#[test]
fn prepared_divine_unsupported_diagnostic_never_leaks_into_explanations_or_grants() {
    // The exact id the 7.1 intake flagged as a suspected Wizard-shaped gap.
    // Confirms, across the whole supported level range, that it: (a) is
    // always present as a claim-blocking diagnostic (the honest "still
    // unproven" burden marker), (b) never appears in `.explanations` at
    // all (it structurally cannot -- `explain_cleric_level1_spell_baseline`
    // only ever `diagnostics.push`es it), and (c) never appears as a
    // `Grant` in any `LevelUpPlan` (which would be a no-fabrication
    // violation far worse than Wizard's own bug: fabricating a "grant"
    // out of a message that says the fact is NOT computed).
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = cleric_at_level(to_level);
        let computation = compute_pilot_base_chassis(&character);

        // (v0.6 alpha swarm, risks item 8) PREPARED_DIVINE_UNSUPPORTED_DIAGNOSTIC_ID
        // is no longer unconditional -- these fixtures have zero prepared spells, a
        // genuinely valid posture, so the diagnostic correctly does not fire. If
        // absent, confirm no spell is fabricated merely because it stopped firing.
        match computation
            .diagnostics
            .iter()
            .find(|d| d.id == PREPARED_DIVINE_UNSUPPORTED_DIAGNOSTIC_ID)
        {
            Some(diagnostic) => assert!(
                diagnostic.claim_blocking,
                "the prepared divine spell posture diagnostic must stay claim-blocking at level {to_level}"
            ),
            None => {
                let prepared_count = computation
                    .explanations
                    .iter()
                    .find(|e| e.id == "class_spell.cleric.daily_preparation")
                    .map(|e| e.value)
                    .unwrap_or(-1);
                assert_eq!(
                    prepared_count, 0,
                    "no spells are fabricated merely because the blocker stopped firing at level \
                     {to_level}: {:?}",
                    computation.diagnostics
                );
            }
        }

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == PREPARED_DIVINE_UNSUPPORTED_DIAGNOSTIC_ID),
            "the prepared-divine-spell diagnostic id must never appear in .explanations at \
             level {to_level} -- diagnostics and explanations are separate Vecs and must stay \
             that way"
        );

        let plan = compute_cleric_level_up_grants(&character, from_level, to_level);
        assert!(
            !plan
                .automatic_features
                .iter()
                .any(|grant| grant.source_table.column_key == PREPARED_DIVINE_UNSUPPORTED_DIAGNOSTIC_ID),
            "the LevelUpPlan for cleric {from_level} -> {to_level} must never fabricate a Grant \
             from the prepared-divine-spell claim-blocking diagnostic: {:#?}",
            plan.automatic_features
        );
    }
}
