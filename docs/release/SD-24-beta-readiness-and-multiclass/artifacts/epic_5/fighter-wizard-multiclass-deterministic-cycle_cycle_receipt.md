# Cycle fighter-wizard-multiclass-deterministic-cycle — Epic 5 (Multiclass) / Criterion 5.2

- **Card ID:** t_placeh01 (placeholder; backfilled with the real kanban card ID in a follow-up commit)
- **Commit SHA:** (recorded post-commit; see `progress.md` cycle log entry for this cycle)
- **Files touched:** `tests/sd24_multiclass_deterministic.rs` (new), `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_5/multiclass-fixture.md` (new), this receipt.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > Criterion 5.2 — Deterministic test surface: 30 character-advancement cycles
  > **Cycle artifact:** `./artifacts/epic_5/multiclass-fixture.md` with per-cycle input/output.
  > **Files touched:** test fixture file `tests/sd24_multiclass_deterministic.rs`.
- **Status:** complete
- **Notes:**
  - **Hard-stop precondition re-confirmed:** Epic 4's per-class coverage matrix (Fighter+Wizard scope) is genuinely committed (`per-class-coverage-matrix.md`, commits `66f9be8`/`f25dc7b`/`2e074ce`/`ac3b130`), and criterion 5.1's Fighter+Wizard multiclass dispatch fix (`0068818`) is already landed — this cycle builds the deterministic breadth surface on top of both, per `loop-instruction.md §2.2`.
  - **Task-brief file-set discrepancy (recorded per the harness's own instruction to append a `## DISCOVERED` entry rather than silently deviate):** this cycle's invocation was handed the identical granted file set as the prior 5.1 cycle (`pilot_compute.rs`, `level_up/fighter.rs`, `level_up/wizard.rs`, plus the 3 already-landed 5.1 test files) — none of which is `tests/sd24_multiclass_deterministic.rs`, the file `epic-breakdown.md`'s own criterion 5.2 row and `content-unit-inventory.md` both name as this criterion's actual target. Treated `epic-breakdown.md`/`content-unit-inventory.md` as ground truth (per the harness's own stale-plan-vs-reality correction policy) and wrote the new test file; the three source files were touched only transiently, to construct and then discharge the RED demonstration (reverted to `HEAD`'s committed state before finishing, `git status --porcelain` confirms 0 diff against them). See `## DISCOVERED` below.
  - **Design of the 30 cycles**, per `technical-design.md §2.2`: 10 solo-Fighter (level 1→10), 10 solo-Wizard (level 1→10), 10 split-advance (5 Fighter-side + 5 Wizard-side, total level 6→10 — the split step itself, total level 5, is criterion 5.1's own test surface and is not re-counted). All 30 points reuse the two existing level-10 fixtures' own posture (feat/choice/ability arrays) via the same isolated-clone-and-mutate-`class_levels` technique `sd24_multiclass_fighter_lv10.rs`/`sd24_multiclass_wizard_lv10.rs` already established — no new fixture files needed.
  - Every cycle asserts against canonical PF1 formulas computed independently in the test file (not copied from `pilot_compute.rs`'s internals): Fighter full BAB / good Fort / poor Ref+Will; Wizard half BAB / poor Fort+Ref / good Will; multiclass mix sums each class's own unrounded fractional save value and floors once for the total. All expected values were independently cross-checked with a standalone Python script before being written into the artifact table — 100% agreement with the test's own live output.
- **Discovery forwards:**
  - `2026-07-21T01:00:00Z | epic-5 | criterion-5.2 | epic-5-task-brief-drift | This cycle's invocation carried forward criterion 5.1's own granted file set verbatim rather than 5.2's real target (tests/sd24_multiclass_deterministic.rs, per epic-breakdown.md's own criterion 5.2 row and content-unit-inventory.md's row-level file mapping). No production files were permanently touched as a result (the 3 source files were only transiently reverted-then-restored for the RED demonstration); this is a harness-invocation-authoring nit, not a scope violation. | suggested: if a future cycle's invocation text is templated from the prior cycle's own "Files/area you own" block, regenerate it from the current criterion's own epic-breakdown.md row instead of copying the previous cycle's block forward.`
- **Next-cycle plan:** Criterion 5.3 (Integration test consumes ingested content, `./artifacts/epic_5/integration-test-cycle_receipt.md`) per `## TODO` deterministic order.

## RED → GREEN evidence

**RED** (live, via temporary revert): `git checkout c3330b6 -- src/rules_core/pilot_compute.rs src/rules_core/level_up/fighter.rs src/rules_core/level_up/wizard.rs` (the commit immediately preceding criterion 5.1's fix commit `0068818`), then:

```
cargo test --locked --test sd24_multiclass_deterministic
```

Result: `2 passed; 2 failed`. The two solo-class cycles (`cycles_01_to_10_solo_fighter_...`, `cycles_11_to_20_solo_wizard_...`) passed unaffected (they never touch the multiclass gate). The two mix cycles
(`cycles_21_to_25_fighter_side_split_advance_...`, `cycles_26_to_30_wizard_side_split_advance_...`) both FAILED, each panicking at the assertion that `class_chassis.spell_baseline.wizard` stays present inside the mix — reproducing, across the full split-advance walk (not just the two single-point endpoints criterion 5.1's own tests already covered), the exact defect criterion 5.1's fix closed.

**GREEN** (restore): `git checkout HEAD -- src/rules_core/pilot_compute.rs src/rules_core/level_up/fighter.rs src/rules_core/level_up/wizard.rs`, then:

```
cargo test --locked --test sd24_multiclass_deterministic
```

Result: `running 4 tests ... test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`.

**Full regression suites** (after restore, `git status --porcelain` confirms the 3 source files are byte-identical to `HEAD` — only the new test file + this cycle's artifacts are new):

```
cargo test --locked --tests --no-fail-fast   # root
```
→ 437 test binaries, 3982 tests passed, 0 failed.

```
cd apps/desktop/src-tauri && cargo test --locked --tests --no-fail-fast
```
→ 113 passed, 0 failed.

## Dual-audit gate

Run against `git diff --unified=0 "${BASE_BRANCH}...HEAD"` where `BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `09e43c3` (SD-23 closure PR #329, HEAD of `origin/develop`):

- **Before:** `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS` (inherited clean from `5.1`'s already-landed commit `0068818`).
- **After:** `OK_NO_BUNDLE_TAGS` / `OK_NO_TOKENS`. (The new test file's own `sd24_` filename identifier is outside the audited glob set — `'src/**/*.rs'` / `apps/desktop/**` only, matching every prior Epic-5 test file's own precedent, e.g. `sd24_multiclass_fighter_lv10.rs` etc.)
