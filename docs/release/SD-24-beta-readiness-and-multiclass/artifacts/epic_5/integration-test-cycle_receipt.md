# Cycle integration-test-cycle — Epic 5 (Multiclass Stacking Real and Full) / Criterion 5.3

- **Card ID:** t_75b0fb36 (kanban board `codex-tranche-5`, status `done`)
- **Commit SHA:** `b503c47`
- **Files touched:**
  - `src/rules_core/rules_tables/crb/class_tables.rs` (production; new `pub fn good_saves_for`)
  - `src/rules_core/pilot_compute.rs` (production; `multiclass_good_saves` now delegates to `class_tables::good_saves_for` instead of a hand-maintained duplicate)
  - `tests/sd24_multiclass_integration.rs` (new)
  - `docs/release/SD-24-beta-readiness-and-multiclass/artifacts/epic_5/integration-test-cycle_receipt.md` (this file)
  - `docs/release/SD-24-beta-readiness-and-multiclass/progress.md` (state update)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (clean before and after; the new test file's own `sd24_` filename sits outside the audited `src/**/*.rs` / `apps/desktop/**` globs, matching every prior Epic-5 test file's own precedent)
- **Wired-integration audit result:** OK_NO_TOKENS (clean before and after)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "### Criterion 5.3 — Integration test consumes ingested content — **Cycle artifact:** `./artifacts/epic_5/integration-test-cycle_receipt.md`."
- **Status:** complete
- **Notes:**
  - **Task-brief discrepancy noted, again (see `## DISCOVERED` below):** this cycle's granted file list (`pilot_compute.rs`, `level_up/fighter.rs`, `level_up/wizard.rs` + the three `*_lv10.rs`/`*_split.rs` test files) was a stale copy of criterion 5.1's own file set — the same drift the prior cycle (5.2) already flagged and predicted would recur if invocation file lists keep being copied forward instead of regenerated from the current criterion's own `epic-breakdown.md` row. `pilot_compute.rs` happened to be genuinely relevant this cycle (see below), but the granted test-file trio was not; the real target, per `acceptance-and-verification.md`'s own verification command (`cargo test --locked --test sd24_multiclass_integration`) and criterion 5.3's own cycle-artifact path, is a brand-new `tests/sd24_multiclass_integration.rs`.
  - **Interpreting "integration test consumes ingested content":** criterion 5.2's `sd24_multiclass_deterministic.rs` (already landed) explicitly asserts against "canonical PF1 formulas computed independently in the test file (not copied from `pilot_compute.rs`'s own internals)" — a hand-typed oracle. Criterion 5.3 is written immediately after 5.2 in `epic-breakdown.md` and is the only remaining Epic-5 criterion whose acceptance text names "ingested content" rather than a fixture format or an audit; the codebase's own established term for "ingested content" is `rules_tables::crb::class_tables::class_tables()`, documented across every `level_up::<class>.rs` module as "SD-19's foundation" / "the more authoritative, class-generic source." This cycle interprets 5.3 as: build the integration test whose oracle is the actually-ingested `class_tables()` table (not a second hand-typed formula twin), so that a real defect class — the production dispatch drifting from the single ingested source of truth — is the thing this test can catch that 5.2's independently-hand-typed-and-therefore-could-drift-in-lockstep oracle cannot.
  - **Genuine production finding, not cosmetic:** while building the ingested-content oracle, found that `pilot_compute.rs`'s `multiclass_good_saves` hardcoded a second, independently-maintained copy of Fighter/Wizard's good/poor save classification (`Some((true, false, false))` / `Some((false, false, true))`), justified only by a doc comment cross-reference to `class_tables.rs`'s `GoodSaves` rows rather than by actually reading them. `class_tables.rs` had no public accessor for that classification (only the already-floored per-class `ClassTableRow.{fort,ref,will}_save` cells, which cannot reconstruct the un-rounded fractional value the multiclass rule needs). Added `pub fn good_saves_for(class_id: ClassId) -> Option<(bool, bool, bool)>` to `class_tables.rs` and refactored `multiclass_good_saves` to delegate to it, removing the duplicate. This closes a genuine (if currently dormant) drift risk between the two copies, consistent with this codebase's own stated preference (every `level_up::<class>.rs` doc comment) for composing `class_tables()` directly over re-deriving/duplicating its data.
  - `fractional_save_value`'s arithmetic (`level/2+2` / `level/3`) is left as-is: it is the PF1 rule's inherent formula, not a data table, and is identical to `class_tables.rs`'s own `save_bonus` formula (already cross-referenced by comment); only the *which classes get which classification* fact was duplicated data, and that duplication is what this cycle removed.
- **Discovery forwards:** 1 entry (`epic-5-task-brief-drift-recurrence`, see `## DISCOVERED` in `progress.md`)
- **Next-cycle plan:** Criterion 5.4 (Multiclass dispatch passes the four-check audit — dual-audit gate output captured) per `## TODO` deterministic order; this cycle's own dual-audit output above can be reused/re-verified as that criterion's evidence if the next cycle finds it still applies to the same diff.

## RED -> GREEN evidence

**RED:** `tests/sd24_multiclass_integration.rs` (new) imports
`rules_tables::crb::class_tables::good_saves_for`, which did not yet exist.
Compiling the new test target failed for the intended reason:

```
$ cargo test --locked --test sd24_multiclass_integration
error[E0432]: unresolved import `codex::rules_core::rules_tables::crb::class_tables::good_saves_for`
  --> tests/sd24_multiclass_integration.rs:40:81
   |
40 | use codex::rules_core::rules_tables::crb::class_tables::{ClassId, class_tables, good_saves_for};
   |                                                                                 ^^^^^^^^^^^^^^ no `good_saves_for` in `rules_core::rules_tables::crb::class_tables`
error: could not compile `codex` (test "sd24_multiclass_integration") due to 1 previous error
```

**GREEN:** added `good_saves_for` to `class_tables.rs` and refactored
`pilot_compute.rs`'s `multiclass_good_saves` to delegate to it:

```
$ cargo test --locked --test sd24_multiclass_integration
running 5 tests
test fighter4_wizard1_split_matches_ingested_class_tables_content ... ok
test fighter9_wizard1_lv10_matches_ingested_class_tables_content ... ok
test wizard9_fighter1_lv10_matches_ingested_class_tables_content ... ok
test solo_fighter_level_1_to_10_matches_ingested_class_tables_content ... ok
test solo_wizard_level_1_to_10_matches_ingested_class_tables_content ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

All pre-existing Epic 5 test files re-run clean against the refactor
(no behavior change, only the source of the good/poor classification moved):

```
$ cargo test --locked --test sd24_multiclass_fighter_lv10 --test sd24_multiclass_wizard_lv10 \
    --test sd24_multiclass_fighter_wizard_split --test sd24_multiclass_deterministic \
    --test sd24_multiclass_integration
... (fighter_lv10) 4 passed; 0 failed
... (wizard_lv10) 4 passed; 0 failed
... (fighter_wizard_split) 8 passed; 0 failed
... (deterministic) 4 passed; 0 failed
... (integration) 5 passed; 0 failed
```

Full regression:

```
$ cargo test --locked --tests --no-fail-fast       # repo root
438 test binaries / 3987 passed / 0 failed          # (3982 prior + 5 new)

$ cargo test --locked --tests --no-fail-fast        # apps/desktop/src-tauri/
113 passed / 0 failed
```

## Dual-audit gate (BASE_BRANCH = merge-base HEAD origin/develop)

Baseline (before this cycle's changes):

```
OK_NO_BUNDLE_TAGS
OK_NO_TOKENS
```

Final (after this cycle's changes):

```
OK_NO_BUNDLE_TAGS
OK_NO_TOKENS
```
