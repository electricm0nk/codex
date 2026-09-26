# Stage F4–F5 fixture receipts

Fixture protocol (per suite step): every failing test is classified (A) pinned count/status moved
by this batch — re-baselined only with a receipt line here; (B) genuine defect — fixed at the root,
RED test first; (C) unrelated pre-existing — attributed from git. A changed printed sheet value
without a PF1 citation is a STOP.

## f4:suite-root (worktree `sd36/epic-f4-f5` at `2a17695258`)

| what | command | result | receipt |
|---|---|---|---|
| root suite | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | exit 0; 293 `test result` lines, **6,391 passed, 0 failed**, 27 ignored | `f4-suite-root-test.log` (condensed: Running + result lines) |
| root clippy | `cargo clippy --locked -j 8 --all-targets -- -D warnings` | exit 0, no warnings | `f4-suite-root-clippy.log` |

Before / after. Before: the last full root run on this branch (`f4pre-full-root.log`, after the
F4pre converter step) was 293 result lines, 6,387 passed, 2 failed, 27 ignored. Both failures were
class (A) pins moved by F4pre and were re-baselined in that step with PF1 citations (CRB p.46 Plant,
CRB p.40 Animal; `f4pre-receipt.md` §9). After (this step, HEAD `2a17695258` = F4pre + F4a–F4d):
6,391 passed, 0 failed, 27 ignored. The 6,391 − 6,389 = +2 net new passing tests come from root tests added after that run (F4pre re-run through F4d); denominator is
the sum of the 293 `test result` lines, `grep '^test result:' f4-suite-root-test.log`.

Failing tests this step: **0**. Classified: (A) 0, (B) 0, (C) 0. Re-baselined tests: none.
Fixed defects: none. Printed sheet values changed: none. STOPs: none.

## f4:suite-ingest (worktree `sd36/epic-f4-f5` at `b269beacf3`)

| what | command | result | receipt |
|---|---|---|---|
| ingest suite | `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | exit 0; 167 `test result` lines, **1,764 passed, 0 failed**, 43 ignored | `f4-suite-ingest-test.log` (condensed: Running + result lines) |

Before / after. Before: the last full ingest run on this branch (`f4pre-full-ingest.log`, committed
in F4pre `4248733c1d`, after the converter step) was 167 result lines, 1,764 passed, 0 failed,
43 ignored. After (this step, HEAD `b269beacf3` = F4pre + F4a–F4d + f4:suite-root): identical,
167 / 1,764 / 0 / 43. F4a–F4d changed root `src/` (class census, seeds, pilot_compute sheet rules),
desktop and tests; `git diff --stat 4248733c1d HEAD -- crates/` is empty, so no ingest test or
source moved. Denominator: the sum of the 167 `test result` lines,
`grep '^test result:' f4-suite-ingest-test.log`.

Failing tests this step: **0**. Classified: (A) 0, (B) 0, (C) 0. Re-baselined tests: none.
Fixed defects: none. Printed sheet values changed: none. STOPs: none.

## f4:suite-desktop (worktree `sd36/epic-f4-f5` at `497de47490`)

| what | command | result | receipt |
|---|---|---|---|
| desktop crate | `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | exit 0; 1 `test result` line, **621 passed, 0 failed**, 0 ignored | `f4-suite-desktop-test.log` (condensed: Running + result + parity lines) |
| corpus-bundle parity | `corpus_bundle_parity_test::sanitised_corpus_bundle_has_the_same_race_equipment_and_spell_population_as_the_raw_corpus` (inside the desktop crate run; regenerates via `scripts/gen-corpus-bundle.mjs`) | ok; unfiltered `git status` empty after the run (regenerated bundle byte-identical to tracked) | `f4-suite-desktop-test.log` |
| frontend typecheck | `cd apps/desktop && npm run typecheck` | exit 0 (`tsc --noEmit`, no errors) | `f4-suite-desktop-frontend.log` |
| frontend tests | `cd apps/desktop && npm test` | exit 0; **126 of 126** `src/**/*.test.ts` files passed | `f4-suite-desktop-frontend.log` |

Before / after. Before: the last full desktop run on this branch (`f4pre-full-desktop.log`, committed
in F4pre `4248733c1d`) was 615 passed, 0 failed, 0 ignored; the last frontend run (`f4d-verify.log`)
was 126 of 126 files passed. After (HEAD `497de47490` = F4pre + F4a–F4d + f4:suite-root +
f4:suite-ingest): 621 passed, 0 failed; 126 of 126 files. The +6 are exactly the six desktop
`#[test]` functions added by F4a–F4d in `character_hub.rs` / `pf1_adapter.rs`
(`git diff 4248733c1d HEAD -- apps/desktop/src-tauri | grep -cE '^\+\s*#\[test\]'` = 6, removed = 0):
`apply_level_up_seeds_an_added_class_the_way_the_census_mix_does`,
`list_class_creation_roster_offers_exactly_the_census_roster`,
`list_class_roster_wire_carries_hit_die_and_skill_ranks_for_every_census_class`,
`list_level_up_class_options_offers_prestige_with_printed_requirements`,
`list_level_up_class_options_advances_a_held_prestige_class_to_its_own_max_and_keeps_the_cap`,
`compose_character_input_seeds_every_census_class_exactly_as_class_seeds_does`.
Denominators: desktop = the single `test result` line of the `codex_desktop` unittest binary;
frontend = the 126 `*.test.ts` files `apps/desktop/scripts/run-tests.mjs` collects under `src/`.

Failing tests this step: **0**. Classified: (A) 0, (B) 0, (C) 0. Re-baselined tests: none.
Fixed defects: none. Printed sheet values changed: none. STOPs: none.
