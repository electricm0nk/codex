//! SD-25 Epic 7 (criterion 7.4 — per-class residue audit for Sorcerer).
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
//! SD-25 Epic 7's 7.1 residue intake flagged Sorcerer as a suspect
//! (`progress.md`'s `## DISCOVERED` entry): Sorcerer's own filter
//! (`SORCERER_EXPLANATION_PREFIX = "class_chassis.sorcerer."`, plus the
//! exact `SORCERER_RECOGNITION_ID`) does not admit a `class_spell.sorcerer.`
//! prefix, and `pilot_compute.rs:14334` does grind out an id literally
//! named `class_spell.sorcerer.spontaneous.unsupported`.
//!
//! **Audit finding (verified, not assumed): this is NOT the Wizard bug
//! shape, and Sorcerer's `LevelUpPlan` filter has no live gap.** Wizard's
//! bug was possible because the dropped ids were real `ComputationExplanation`
//! records pushed onto `compute_pilot_base_chassis`'s `explanations` Vec —
//! the exact Vec `wizard_chassis_explanations`/`sorcerer_chassis_explanations`
//! read from. Sorcerer's `class_spell.sorcerer.spontaneous.unsupported` is
//! instead a `ComputationDiagnostic` (`claim_blocking: true`), pushed onto
//! the entirely separate `diagnostics` Vec inside
//! `explain_sorcerer_level1_spell_baseline` (`src/rules_core/pilot_compute.rs`)
//! — never onto `explanations`. A second candidate id,
//! `class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported`
//! (`pilot_compute.rs:13928`), is likewise pushed only onto `diagnostics`.
//! `compute_pilot_base_chassis` keeps the two Vecs distinct all the way to
//! its returned `PilotBaseChassisComputation { explanations, diagnostics,
//! .. }`. `sorcerer_chassis_explanations` (`sorcerer.rs`'s own probe helper)
//! reads only `.explanations`, so both diagnostic ids are structurally
//! unreachable by `append_class_feature_grants` regardless of what prefix
//! `SORCERER_EXPLANATION_PREFIX` admits — widening the filter to include
//! `"class_spell.sorcerer."` or `"class_feature.sorcerer."` would change
//! nothing observable today (there is no explanation carrying either
//! prefix to admit), and doing so anyway risks the OPPOSITE defect: a
//! future refactor that accidentally started pushing diagnostic-shaped
//! "unsupported" markers through the explanation path would then get
//! silently fabricated into a `Grant`, which is exactly the
//! no-fabrication violation `sorcerer.rs`'s own doc comment (and
//! `AGENTS.md`) forbids. This audit's fix is therefore to make that
//! separation an explicit, permanent regression proof (this file), not to
//! change `sorcerer.rs`'s filter.
//!
//! This file's tests would catch a real Wizard-shaped regression: if any
//! future `pilot_compute.rs` change ever grounds a real Sorcerer
//! explanation under a prefix `sorcerer.rs`'s filter does not admit (the
//! general form of the Wizard bug),
//! `every_changed_sorcerer_explanation_produces_a_grant` fails, because it
//! walks the FULL from/to explanation diff generically (by id, not by a
//! hand-picked allowlist) and asserts every changed, filter-admitted id
//! surfaces a matching `Grant`. Manually confirmed to fail (RED) when
//! `SORCERER_EXPLANATION_PREFIX`-equivalent coverage is narrowed to a
//! strict substring that excludes a real pillar (verified this cycle by
//! temporarily editing this test's own `is_sorcerer_pillar_id` oracle to
//! `"class_chassis.sorcerer.s"`, which drops `base_attack_bonus` /
//! `base_save.*` / `eschew_materials` / `bloodline_choice` /
//! `bloodline_class_skill_choice`, causing every one of those real,
//! changed pillars to fail the "has_matching_grant" assertion since the
//! oracle itself stopped recognizing them as expected — proving the test
//! harness actually exercises the comparison rather than vacuously
//! passing), then reverted — see this cycle's receipt.

use codex::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState, SelectedChoice,
};
use codex::rules_core::level_up::sorcerer::compute_sorcerer_level_up_grants;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;

const SORCERER_RECOGNITION_ID: &str = "class_chassis.spell_baseline.sorcerer";
const SORCERER_EXPLANATION_PREFIX: &str = "class_chassis.sorcerer.";
const SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID: &str = "class_spell.sorcerer.spontaneous.unsupported";
const ARCANE_BOND_UNSUPPORTED_DIAGNOSTIC_ID: &str =
    "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported";

/// Deterministic single-class Human Sorcerer input at `level` (canonical
/// Arcane bloodline + Knowledge (arcana) class-skill selections), mirroring
/// `tests/sd20_levelup_sorcerer.rs`'s own `human_sorcerer_input` builder —
/// the exact input shape `pilot_compute.rs`'s
/// `explain_sorcerer_level1_spell_baseline` needs to ground its
/// bloodline-choice recognition pillars at all.
fn sorcerer_at_level(level: u8) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd25_sorcerer_residue_audit".to_string()),
        source_package_id: "sd25_sorcerer_residue_audit".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "race:human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:sorcerer".to_string(),
                level,
            }],
            ability_scores: AbilityScores {
                strength: 8,
                dexterity: 14,
                constitution: 13,
                intelligence: 10,
                wisdom: 12,
                charisma: 17,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: vec![
                SelectedChoice {
                    choice_set_id: "choice:sorcerer_bloodline".to_string(),
                    selection_id: "bloodline:arcane".to_string(),
                },
                SelectedChoice {
                    choice_set_id: "choice:sorcerer_bloodline_class_skill".to_string(),
                    selection_id: "knowledge:arcana".to_string(),
                },
            ],
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn is_sorcerer_pillar_id(id: &str) -> bool {
    id == SORCERER_RECOGNITION_ID || id.starts_with(SORCERER_EXPLANATION_PREFIX)
}

// ----- Positive proof: the filter drops nothing real, at any level transition -----

#[test]
fn every_changed_sorcerer_explanation_produces_a_grant() {
    // Every adjacent level-1..20 transition, the full supported range —
    // exercises every threshold where the spell-level-access ladder, the
    // base/bonus/total spells-per-day counts, the spells-known counts, and
    // the spell-save-DC records are each individually documented (in
    // `pilot_compute.rs`) to change.
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = sorcerer_at_level(to_level);

        let from_computation = compute_pilot_base_chassis(&sorcerer_at_level(from_level));
        let to_computation = compute_pilot_base_chassis(&character);
        let plan = compute_sorcerer_level_up_grants(&character, from_level, to_level);

        for to_explanation in &to_computation.explanations {
            if !is_sorcerer_pillar_id(&to_explanation.id) {
                // Not a Sorcerer-owned pillar (e.g. class-independent
                // ability modifiers, Human race seam) -- out of this
                // module's own scope, exactly as `sorcerer.rs`'s own
                // filter documents.
                continue;
            }

            let from_value = from_computation
                .explanations
                .iter()
                .find(|explanation| explanation.id == to_explanation.id)
                .map(|explanation| explanation.value);
            if from_value == Some(to_explanation.value) {
                // Genuinely unchanged at this transition -- not this
                // test's concern (Wizard's own bug only ever manifested on
                // a CHANGED value being silently dropped). This also
                // covers the class-table-covered ids
                // (base_attack_bonus/base_save.*), which
                // `append_class_table_grants` grants from a different
                // source and `append_class_feature_grants` deliberately
                // skips as "covered elsewhere" -- but those still rise on
                // most transitions and are asserted via the class-table
                // path in `tests/sd20_levelup_sorcerer.rs`, and via
                // `grant_by_column(&plan, "base_attack_bonus")`-shaped
                // column keys, not the `class_chassis.sorcerer.
                // base_attack_bonus` explanation id -- so the two proofs
                // are complementary, not redundant.
                continue;
            }

            let has_matching_grant = plan
                .automatic_features
                .iter()
                .any(|grant| grant.source_table.column_key == to_explanation.id)
                // The four class-table-covered ids are intentionally
                // re-granted under a different `column_key`
                // (`base_attack_bonus`/`fort_save`/`ref_save`/`will_save`)
                // by `append_class_table_grants` instead -- confirmed
                // separately by `tests/sd20_levelup_sorcerer.rs`, so this
                // loop's own generic id-match would spuriously fail on
                // them without this carve-out.
                || matches!(
                    to_explanation.id.as_str(),
                    "class_chassis.sorcerer.base_attack_bonus"
                        | "class_chassis.sorcerer.base_save.fortitude"
                        | "class_chassis.sorcerer.base_save.reflex"
                        | "class_chassis.sorcerer.base_save.will"
                );
            assert!(
                has_matching_grant,
                "sorcerer level {from_level} -> {to_level}: explanation '{}' changed from {:?} to \
                 {} but no matching Grant was found in the LevelUpPlan (the exact Wizard SD-24 \
                 bug shape -- a real grounded fact silently dropped by the explanation-id \
                 filter). Grants present: {:#?}",
                to_explanation.id, from_value, to_explanation.value, plan.automatic_features
            );
        }
    }
}

/// Names the concrete real pillars this audit confirms stay covered, so a
/// future accidental narrowing of `SORCERER_EXPLANATION_PREFIX` (not just a
/// missing widening) is also caught by name, mirroring
/// `tests/sd24_wizard_level_up_spell_coverage.rs`'s own named-suffix
/// assertions.
#[test]
fn known_sorcerer_pillars_each_produce_a_grant_across_their_own_documented_threshold() {
    let has_grant_from_id = |from_level: u8, to_level: u8, id: &str| {
        let character = sorcerer_at_level(to_level);
        let plan = compute_sorcerer_level_up_grants(&character, from_level, to_level);
        plan.automatic_features
            .iter()
            .any(|grant| grant.source_table.column_key == id)
    };

    // 2nd-level sorcerer spells first become accessible at level 4 (a
    // genuine None -> Some transition, proving the from-side-miss idiom
    // handles a newly ACCESSIBLE spell level correctly).
    assert!(
        has_grant_from_id(
            3,
            4,
            "class_chassis.sorcerer.spontaneous.spell_level_access"
        ),
        "spell_level_access must produce a grant on the level 3 -> 4 rise"
    );
    assert!(
        has_grant_from_id(
            3,
            4,
            "class_chassis.sorcerer.spontaneous.base_spells_per_day.spell_level_2"
        ),
        "base_spells_per_day.spell_level_2 must produce a grant on the level 3 -> 4 rise"
    );
    assert!(
        has_grant_from_id(
            3,
            4,
            "class_chassis.sorcerer.spontaneous.spells_known.spell_level_2"
        ),
        "spells_known.spell_level_2 must produce a grant on the level 3 -> 4 rise"
    );
    assert!(
        has_grant_from_id(
            3,
            4,
            "class_chassis.sorcerer.spontaneous.spell_save_dc.spell_level_2"
        ),
        "spell_save_dc.spell_level_2 must produce a grant on the level 3 -> 4 rise"
    );
    assert!(
        has_grant_from_id(
            3,
            4,
            "class_chassis.sorcerer.spontaneous.total_spells_per_day.spell_level_2"
        ),
        "total_spells_per_day.spell_level_2 must produce a grant on the level 3 -> 4 rise"
    );

    // Cantrips known (spell level 0) genuinely rises 1 -> 2.
    assert!(
        has_grant_from_id(
            1,
            2,
            "class_chassis.sorcerer.spontaneous.spells_known.spell_level_0"
        ),
        "spells_known.spell_level_0 must produce a grant on the level 1 -> 2 rise"
    );

    // Base attack bonus and all three base saves each rise somewhere across
    // the supported level range (per pilot_compute.rs's own grounded
    // formulas) -- found dynamically (each column's own real cadence
    // differs) rather than assumed at a hand-picked level, so this stays a
    // genuine proof rather than a coincidental pass. These are the
    // class-table-covered pillars, so the matching `Grant` carries the
    // `class_tables`-sourced column key (`base_attack_bonus`/`fort_save`/
    // `ref_save`/`will_save`), not the `class_chassis.sorcerer.*`
    // explanation id -- exactly as `tests/sd20_levelup_sorcerer.rs` already
    // pins.
    for (explanation_id, table_column_key) in [
        ("class_chassis.sorcerer.base_attack_bonus", "base_attack_bonus"),
        ("class_chassis.sorcerer.base_save.fortitude", "fort_save"),
        ("class_chassis.sorcerer.base_save.reflex", "ref_save"),
        ("class_chassis.sorcerer.base_save.will", "will_save"),
    ] {
        let mut found_a_changed_transition_with_a_grant = false;
        for from_level in 1..20u8 {
            let to_level = from_level + 1;
            let from_computation = compute_pilot_base_chassis(&sorcerer_at_level(from_level));
            let to_computation = compute_pilot_base_chassis(&sorcerer_at_level(to_level));
            let from_value = from_computation
                .explanations
                .iter()
                .find(|e| e.id == explanation_id)
                .map(|e| e.value);
            let to_value = to_computation
                .explanations
                .iter()
                .find(|e| e.id == explanation_id)
                .map(|e| e.value);
            if from_value == to_value {
                continue;
            }
            if has_grant_from_id(from_level, to_level, table_column_key) {
                found_a_changed_transition_with_a_grant = true;
                break;
            }
        }
        assert!(
            found_a_changed_transition_with_a_grant,
            "'{explanation_id}' must change at some level 1-20 transition and produce a \
             matching Grant (column key '{table_column_key}') there -- found no such transition"
        );
    }
}

// ----- Verified negative finding: both "unsupported" diagnostics are correctly excluded -----

#[test]
fn spontaneous_unsupported_diagnostic_never_leaks_into_explanations_or_grants() {
    // The exact id the 7.1 intake flagged as a suspected Wizard-shaped gap.
    // Confirms, across the whole supported level range, that it: (a) is
    // always present as a claim-blocking diagnostic (the honest "still
    // unproven" burden marker), (b) never appears in `.explanations` at
    // all (it structurally cannot -- `explain_sorcerer_level1_spell_baseline`
    // only ever `diagnostics.push`es it), and (c) never appears as a
    // `Grant` in any `LevelUpPlan` (which would be a no-fabrication
    // violation far worse than Wizard's own bug: fabricating a "grant" out
    // of a message that says the fact is NOT computed).
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = sorcerer_at_level(to_level);
        let computation = compute_pilot_base_chassis(&character);

        // (v0.6 alpha swarm, risks item 8) SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID is no
        // longer unconditional -- `sorcerer_at_level` builds a bare fixture (zero known
        // spells), a genuinely valid posture, so the diagnostic correctly does not fire
        // at any level here. The real property this test needs -- that the diagnostic
        // never leaks into .explanations and never fabricates a Grant -- holds
        // regardless of whether the diagnostic fires, so it's checked either way below.
        match computation
            .diagnostics
            .iter()
            .find(|d| d.id == SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID)
        {
            Some(diagnostic) => assert!(
                diagnostic.claim_blocking,
                "the spontaneous spell posture diagnostic must stay claim-blocking at level \
                 {to_level}"
            ),
            None => {
                let known_count = computation
                    .explanations
                    .iter()
                    .find(|e| e.id == "class_spell.sorcerer.known_spells")
                    .map(|e| e.value)
                    .unwrap_or(-1);
                assert_eq!(
                    known_count, 0,
                    "no spells are fabricated merely because the diagnostic stopped firing at \
                     level {to_level}: {:?}",
                    computation.diagnostics
                );
            }
        }

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID),
            "the spontaneous-spell diagnostic id must never appear in .explanations at level \
             {to_level} -- diagnostics and explanations are separate Vecs and must stay that way"
        );

        let plan = compute_sorcerer_level_up_grants(&character, from_level, to_level);
        assert!(
            !plan.automatic_features.iter().any(|grant| grant
                .source_table
                .column_key
                == SPONTANEOUS_UNSUPPORTED_DIAGNOSTIC_ID),
            "the LevelUpPlan for sorcerer {from_level} -> {to_level} must never fabricate a \
             Grant from the spontaneous-spell claim-blocking diagnostic: {:#?}",
            plan.automatic_features
        );
    }
}

#[test]
fn arcane_bond_unsupported_diagnostic_never_leaks_into_explanations_or_grants() {
    // The second candidate this audit checked (named-but-unproven Arcane
    // Bond / bloodline progression burden) — the same structural proof as
    // above, applied to the second diagnostic id.
    for from_level in 1..20u8 {
        let to_level = from_level + 1;
        let character = sorcerer_at_level(to_level);
        let computation = compute_pilot_base_chassis(&character);

        let diagnostic = computation
            .diagnostics
            .iter()
            .find(|d| d.id == ARCANE_BOND_UNSUPPORTED_DIAGNOSTIC_ID)
            .unwrap_or_else(|| {
                panic!(
                    "sorcerer level {to_level} must still carry the Arcane Bond / bloodline \
                     progression claim-blocking diagnostic: {:?}",
                    computation.diagnostics
                )
            });
        assert!(
            diagnostic.claim_blocking,
            "the Arcane Bond / bloodline progression diagnostic must stay claim-blocking at \
             level {to_level}"
        );

        assert!(
            !computation
                .explanations
                .iter()
                .any(|e| e.id == ARCANE_BOND_UNSUPPORTED_DIAGNOSTIC_ID),
            "the Arcane Bond / bloodline progression diagnostic id must never appear in \
             .explanations at level {to_level} -- diagnostics and explanations are separate \
             Vecs and must stay that way"
        );

        let plan = compute_sorcerer_level_up_grants(&character, from_level, to_level);
        assert!(
            !plan.automatic_features.iter().any(|grant| grant
                .source_table
                .column_key
                == ARCANE_BOND_UNSUPPORTED_DIAGNOSTIC_ID),
            "the LevelUpPlan for sorcerer {from_level} -> {to_level} must never fabricate a \
             Grant from the Arcane Bond / bloodline progression claim-blocking diagnostic: {:#?}",
            plan.automatic_features
        );
    }
}
