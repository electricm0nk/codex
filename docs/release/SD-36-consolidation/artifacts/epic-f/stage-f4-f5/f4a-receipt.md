# SD-36 Epic F4a: census roster reason, single-source seeds, level-up seeding (receipt)

Scope: the engine side of F4 (`epic-f-class-completion.md` §6; acceptance F4.3 and F4.5 in
`epic-breakdown.md`). No desktop picker change here (that is F4b), and no `data/**` change.

- Branch `sd36/epic-f4-f5`, worktree `/home/ubuntu/workspace/worktrees/codex-epic-f4`, on top of
  F4pre (`4248733c1d`).
- Tests added: `class_census::tests::no_computed_class_is_unoffered_without_a_named_reason`,
  `class_census::tests::ex_state_reads_the_record_not_the_name`,
  `pf1_adapter::tests::compose_character_input_seeds_every_census_class_exactly_as_class_seeds_does`,
  `character_hub::tests::apply_level_up_seeds_an_added_class_the_way_the_census_mix_does`.
- Logs: `f4a-red.log`, `f4a-green.log`, `f4a-verify.log` (this directory). Census:
  `../census-f4a.json`.

## 1. Roster reason (F4.5)

`src/rules_core/class_census.rs` gains a closed enum, `RosterReason { offered, hit_die_absent,
not_computed, prestige, ex_state }` (serde snake_case). There is one rule, `roster_reason`: a
class is `offered` iff it is not prestige, not an Ex-* state, Computed at every level of its own
sweep, and states a hit die. Every other row gets exactly one named reason. When more than one
applies, the order is prestige, then ex_state, then not_computed, then hit_die_absent.

- **Ex-* state reads the record, not the name alone.** The id is `ex_<class>` AND the converted
  class principal is hidden (`print: false`, the oracle's `VISIBLE:NO`). The four census Ex
  records have tags `[Base, PC]` and no Ex tag, so the hidden flag is the registry mark.
  `ex_state_reads_the_record_not_the_name` pins that an `ex_` id with no hidden record is not
  an Ex state. (`advanced_class_guide/ex_warpriest` is hidden too, but it is not a census id.)
- **Hit die source.** `class_chassis_sheet_rules::hit_die_from_package` reads the principal's
  `StatBlock "Hit die"` row. This is the same row `ClassChassis::hit_die` reads, but it also works
  when the record has no chassis. That matters for the CRB Monk: it states `d8`, but its
  progressions degraded to text, so `chassis_record` is `None` (FS-23). Reading
  `ClassChassis.hit_die` alone would have named the Monk `hit_die_absent` even though its
  single-class sheet is Computed with HP.
- **JSON.** Every `classes[]` and `prestige[]` row in the census JSON carries `in_desktop_roster`
  and `roster_reason`. The top level carries `roster_offered`.
- **Desktop entry point.** `pub fn class_creation_roster() -> Result<Vec<ClassRosterEntry>,
  String>` returns entries `{id, display_name, family, book, hit_die, max_level}`, only for rows
  whose reason is `offered`. They are grouped by `ClassFamily`, and within a family they keep
  registry order (new `ClassCensusEntry::registry_order`, the first-claim position in
  `census()`). The result is cached per process. `Err` names why the fixture failed to load; it
  never returns an empty roster.

Measured counts. Denominator: the 137 census ids. Commands: `cargo run --locked -j 8 --bin
class_census -- --json docs/release/SD-36-consolidation/artifacts/epic-f/census-f4a.json` and the
test.

| reason | count | ids |
|---|---|---|
| offered | **59** | CRB 11, ACG 10, APG 6, CRB NPC 5, Unchained 4, UC 3, untabled exotic 20 |
| prestige | 74 | every Prestige-tagged id (levels up only, in a mix, §9) |
| ex_state | 4 | `ex_antipaladin`, `ex_barbarian`, `ex_inquisitor`, `ex_paladin` |
| not_computed | 0 | all 63 non-prestige classes are Computed at every level |
| hit_die_absent | 0 | none |

Deviation from F4.5's wording, stated rather than hidden: §0.4's 7 hit-die-absent RECORDS
(`psychic_detective`, `gifted_blade`, `gifted_blade_marksman_power_list`, `unlocked_talent`,
`sorcerer_cleric_arcane`, `vwarlock`, `vcabalist`) are **not census ids**. None of them carries
a chassis (`the_seven_hit_die_absent_records_have_no_chassis_at_all`), so no registry claims
them. The test therefore asserts, by id, that each one is absent from the census, and it asserts
`hit_die_absent == 0`. If one of the 7 ever becomes a census id, the test fails and names it.

## 2. Canonical seeds single-sourced (F4.3)

`class_seeds::canonical_seeds_for(class_name, class_level)` is now the only seed table:

- `pf1_adapter.rs::compose_character_input` lost its roughly 380-line per-class `if` chain. It now
  calls `canonical_seeds_for(slug, request.level)`.
- `apply_level_up` also calls it (§3).
- `v06_class_state_dump` imports it and reports the seeds it applied per class
  (`canonical_seeds`, each with `from_level`).
- 32 adapter constants that nothing read any more were deleted. 17 constants now read only by
  tests are `#[cfg(test)]`.

The signature gained `class_level` so that one table can state a level-gated seed: the
Barbarian's and Unchained Barbarian's first rage power arrives at 2nd level and is returned only
from level 2. Every caller now passes its real level:

- the census sweep and mixes;
- the proficiency and class-skill readers (`class_level` threaded through);
- `carrier_pick_for_option`, which asks which of a class's choices offers an option, passes the
  PF1 cap, 20.

The parity test found that the two tables had drifted apart. Before F4a it reported 98
class-level mismatches over the census × levels (`f4a-red.log`):

| class | levels | drift | resolved to |
|---|---|---|---|
| oracle | 1-20 | adapter: Battle Mystery + Battlecry + Clouded Vision; class_seeds: Life + Clouded Vision | the adapter's (the DoD-8 seed since SD31-E4-F2-002) |
| barbarian | 2-20 | adapter seeds Superstition; class_seeds none | seeded from level 2 |
| unchained_barbarian | 2-20 | same | seeded from level 2 |
| expert | 1-20 | class_seeds seeds ten class-skill picks; adapter none | seeded (the desktop Expert now records the census posture) |
| psion | 1-20 | class_seeds seeds the Egoist discipline; adapter none | seeded |

Acceptance, run on the committed tree:

```
$ git grep -c 'fn canonical_seeds_for' -- src apps
src/rules_core/class_seeds.rs:1
$ git grep -n 'use .*canonical_seeds_for' -- src/bin apps
apps/desktop/src-tauri/src/pf1_adapter.rs:53:use codex::rules_core::class_seeds::canonical_seeds_for;
src/bin/v06_class_state_dump.rs:60:use codex::rules_core::class_seeds::{FIXTURE_RELATIVE_PATH, canonical_seeds_for, input_for};
```

**Seed parity.** Denominator: every census class (137) at every level of its own `max_level`.
For each one, `compose_character_input`'s choices and spells equal the two fixed Fighter feat
slots plus `canonical_seeds_for(slug, level)`.

## 3. A class added at level-up is seeded

`apply_level_up`'s new-class branch previously seeded only Wizard, Monk and Arcanist. It now
records `canonical_seeds_for(slug, 1)` for any class it adds. The branch runs only the first
time a class joins `class_levels`, so a seed is never recorded twice; the existing
`..._does_not_reseed_wizard_choices_on_a_second_level_up_within_wizard` test still passes.

RED (HEAD adapter, `f4a-red.log`):

```
fighter 1 + cavalier 1: census computed=true []; apply_level_up computed=false
  ["multiclass.cavalier.class_feature.apg.cavalier.order_powers.unsupported",
   "multiclass.cavalier.class_feature.apg.cavalier.other_features_deferred.unsupported"]
```

GREEN: for Cavalier, Inquisitor and Oracle, `fighter 1 + <class> 1` through `apply_level_up`
reaches the same status the census mix reaches (`sweep_mix_panel_row`, seeded by the same
function). All three are Computed.

## 4. RED → GREEN

| test | RED (before) | GREEN |
|---|---|---|
| `no_computed_class_is_unoffered_without_a_named_reason` | compile failure: `RosterReason`, `roster_reasons`, `is_ex_state`, `roster_hit_die`, `class_creation_roster` absent (24 errors) | ok |
| `compose_character_input_seeds_every_census_class_exactly_as_class_seeds_does` | 98 mismatches (table in §2) | ok |
| `apply_level_up_seeds_an_added_class_the_way_the_census_mix_does` | Cavalier dip Blocked on its own choice lines while the census mix is Computed | ok |

## 5. Verification

| gate | command | result |
|---|---|---|
| census tests | `cargo test --locked -j 8 --lib class_census -- --test-threads=8` | 35 passed, 0 failed |
| desktop adapter + hub | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8 pf1_adapter character_hub` | 223 passed, 0 failed |
| clippy root / desktop | `cargo clippy --locked --tests -j 8 [--manifest-path apps/desktop/src-tauri/Cargo.toml] -- -D warnings` | exit 0 / 0 |
| full root | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | 293 result lines, 6,391 passed, 0 failed, 27 ignored (61m53s) |
| full desktop | `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 617 passed, 0 failed |
| census | `cargo run --locked -j 8 --bin class_census -- --json .../census-f4a.json` | ids 137; non-prestige 63 of 63 Computed; prestige alone 74 of 74 Blocked; prestige mixes 68 of 74 Computed (0 Unknown); mix panel 185 of 185 Computed; `roster_offered` 59 (25m15s) |
| no status change | per-row `status`/`levels_computed`/`levels_blocked`, per-mix carrier+status, mix-panel status: `census-f4a.json` vs `census-f3c5.json` and vs `census-f4pre.json` | 0 differences against either |
| dump smoke | `cargo run --locked -j 8 --quiet --bin v06_class_state_dump` | 31 of 31 Computed; oracle seeds Battle/Battlecry/Clouded Vision; barbarian rage power `from_level: 2` |

## 6. Carried to F4b (named, not done here)

- **The roster's own cost.** `class_creation_roster()` sweeps the 63 non-prestige classes at
  every level once per process. Its wall time was not measured on its own. The full census run
  (which also sweeps 74 prestige classes × their carriers and the 185-row mix panel) took 25m15s
  single-threaded in a debug build. F4b must measure the desktop call before wiring it to the
  Create picker, and then choose between a live sweep and a generated roster artifact.
- The desktop `list_class_creation_roster` command, `classRoster.ts`, and the `CLASS_OPTIONS`
  fallback (F4.1, F4.2, F4.4).
