---
canonical: true
purpose: Known-wrong things, with blast radius. Distinct from sweeps: a defect is one broken thing;
  a sweep is a question about how widely a pattern applies.
---

# Defects

| # | Defect | Found | Blast radius | Status |
|---|---|---|---|---|
| D1 | **Equipment cache generators revert narrowed citations.** Re-running `cache_gen::apg::generate_equipment` or `gen_core_rulebook_cache` silently reverts 412 narrowed citations and withdraws up to 346 units. Standing mitigation: re-run `repair_lst_provenance` after either generator, before committing. | wave 17 (OPEN-ISSUES 264) | up to 346 units | DETECTION LANDED (`sd31_lst_provenance_repair_is_durable.rs`); the two generators themselves are still not fixed |
| D2 | **`classlevel()` does not verify its class-name argument.** `formula_interpreter.rs` documents that no consumer may bank a value through a `classlevel()`-bearing formula until this is resolved. | wave 25b | ~4 known class_feature units, likely more | OPEN — standing precondition, respected by wave 26 |
| D3 | **Four formula shapes unreadable.** 431 of 2,671 corpus formulas (16.1%) refuse: PREVARGTEQ-embedded conditional addends (a different PCGen subsystem, BonusObj — the largest bucket), boolean-to-int coercion of a bare comparison, the `&&` operator, and `skillinfo(...)`. | wave 25b | unknown unit count behind them | IN FLIGHT (wave 26 uncovered-shapes lane) |
| D4 | **Anti-fabrication gates exclude seven classes.** Wizard, Bard, Paladin, Cleric, Sorcerer sit behind nine gates; Druid and Monk behind `is_druid_pillar_id`/`is_monk_pillar_id` LevelUpPlan allowlists that silently drop real ids. | waves 20/23 (OPEN-ISSUES 330/338) | caps the grant-fact consumer | OPEN — deferred across waves 21, 22, 23, 24 |
| D5 | **A silent drop is its own defect.** If the LevelUpPlan id-prefix allowlists discard real ids, what else have they been discarding? Never measured. | wave 24 dispatch | unknown | NOT STARTED |
