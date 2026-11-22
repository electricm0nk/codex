//! The `run_bar_check` aggregator: moved out of `codex` (SD-36 Epic A / D1).
//!
//! This is the ONE live read the old crate had of the tool side
//! (`src/rules_core/derived_evaluator_fixture_check.rs:114,121,131` before
//! the move): it stitched together the per-kind `run_*_bar_check` helpers,
//! two of which live in `oracle_validation`
//! (`class_feature_scaling_bar_check`, `race_trait_formula_bar_check`).
//! Moving the aggregator here, rather than moving the oracle calls into
//! `codex`, keeps the oracle harness -- kept for Starfinder, `decisions.md`
//! §11 -- entirely out of the live crate.
//!
//! The per-kind helpers, `BarCheckReport`, `SettledSpellFormulas`,
//! `FIXTURE_RELATIVE_PATH` and `SETTLED_SPELL_KEY_SEPARATOR` all STAY in
//! `codex::rules_core::derived_evaluator_fixture_check` -- they are live
//! reads themselves (`record_vars.rs:42` and three desktop catalogs), so
//! only the aggregator that names the oracle crosses the wall. Those helper
//! fns were `fn` (module-private); this move made them `pub` so this
//! aggregator can call them from the other side of the crate boundary.

use std::path::Path;

/// Runs the `derived` bar over every fixture entry, exactly as
/// `tests/derived_evaluator_fixture_check.rs::engine_evaluator_output_equals_the_corpus_derived_expected_value`
/// does, factored out so both the test and `v06_work_inventory` call the
/// same code.
pub fn run_bar_check(repo_root: &Path) -> codex::rules_core::derived_evaluator_fixture_check::BarCheckReport {
    let equipment = codex::rules_core::derived_evaluator_fixture_check::run_equipment_bar_check(repo_root);
    let monster = codex::rules_core::derived_evaluator_fixture_check::run_monster_bar_check(repo_root);
    let monster_sla = codex::rules_core::derived_evaluator_fixture_check::run_monster_sla_bar_check(repo_root);
    let spell = codex::rules_core::derived_evaluator_fixture_check::run_spell_bar_check(repo_root);
    let spell_range = codex::rules_core::derived_evaluator_fixture_check::run_spell_range_bar_check(repo_root);
    let class_feature =
        crate::oracle_validation::class_feature_scaling_bar_check::run_class_feature_bar_check(
            repo_root,
        );
    let monster_ability = codex::rules_core::derived_evaluator_fixture_check::run_monster_ability_bar_check(repo_root);
    let monster_ability_formula = codex::rules_core::derived_evaluator_fixture_check::run_monster_ability_formula_bar_check(repo_root);
    let companion = codex::rules_core::derived_evaluator_fixture_check::run_companion_bar_check(repo_root);
    let companion_skill = codex::rules_core::derived_evaluator_fixture_check::run_companion_skill_bar_check(repo_root);
    let companion_save_dc = codex::rules_core::derived_evaluator_fixture_check::run_companion_save_dc_bar_check(repo_root);
    let class_feature_description = codex::rules_core::derived_evaluator_fixture_check::run_class_feature_description_bar_check(repo_root);
    let race_trait_formula =
        crate::oracle_validation::race_trait_formula_bar_check::run_race_trait_formula_bar_check(
            repo_root,
        );
    let mut cleared = equipment.cleared;
    cleared.extend(monster.cleared);
    cleared.extend(monster_sla.cleared);
    cleared.extend(spell.cleared);
    cleared.extend(spell_range.cleared);
    cleared.extend(class_feature.cleared);
    cleared.extend(monster_ability.cleared);
    cleared.extend(monster_ability_formula.cleared);
    cleared.extend(companion.cleared);
    cleared.extend(companion_skill.cleared);
    cleared.extend(companion_save_dc.cleared);
    cleared.extend(class_feature_description.cleared);
    cleared.extend(race_trait_formula.cleared);
    let mut failures = equipment.failures;
    failures.extend(monster.failures);
    failures.extend(monster_sla.failures);
    failures.extend(spell.failures);
    failures.extend(spell_range.failures);
    failures.extend(class_feature.failures);
    failures.extend(monster_ability.failures);
    failures.extend(monster_ability_formula.failures);
    failures.extend(companion.failures);
    failures.extend(companion_skill.failures);
    failures.extend(companion_save_dc.failures);
    failures.extend(class_feature_description.failures);
    failures.extend(race_trait_formula.failures);
    let mut engine_does_not_hold = equipment.engine_does_not_hold;
    engine_does_not_hold.extend(monster.engine_does_not_hold);
    engine_does_not_hold.extend(monster_sla.engine_does_not_hold);
    engine_does_not_hold.extend(spell.engine_does_not_hold);
    engine_does_not_hold.extend(spell_range.engine_does_not_hold);
    engine_does_not_hold.extend(class_feature.engine_does_not_hold);
    engine_does_not_hold.extend(monster_ability.engine_does_not_hold);
    engine_does_not_hold.extend(monster_ability_formula.engine_does_not_hold);
    engine_does_not_hold.extend(companion.engine_does_not_hold);
    engine_does_not_hold.extend(companion_skill.engine_does_not_hold);
    engine_does_not_hold.extend(companion_save_dc.engine_does_not_hold);
    engine_does_not_hold.extend(class_feature_description.engine_does_not_hold);
    engine_does_not_hold.extend(race_trait_formula.engine_does_not_hold);
    // A unit that FAILED any seam must never be reported cleared by another
    // one. `cleared` is a union across seams and `failures` is keyed by
    // `unit_id`, so a unit covered by two seams could otherwise be stamped on
    // the strength of the seam it passed while the seam it failed only ever
    // showed up in a report nothing reads. Subtracting here keeps
    // `apply_done_rung_stamps`'s input honest for every seam added later, not
    // just today's.
    for id in failures.keys().chain(engine_does_not_hold.keys()) {
        cleared.remove(id);
    }
    codex::rules_core::derived_evaluator_fixture_check::BarCheckReport {
        cleared,
        failures,
        engine_does_not_hold,
        fixtures_total: equipment.fixtures_total
            + monster.fixtures_total
            + monster_sla.fixtures_total
            + spell.fixtures_total
            + spell_range.fixtures_total
            + class_feature.fixtures_total
            + monster_ability.fixtures_total
            + monster_ability_formula.fixtures_total
            + companion.fixtures_total
            + companion_skill.fixtures_total
            + class_feature_description.fixtures_total
            + race_trait_formula.fixtures_total,
    }
}

