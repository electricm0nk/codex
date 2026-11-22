# Cycle 10 — Epic 2 (Build 8 of 9 tables) / AT-34-E2-002

- **Commit SHA:** afbe67a22f849fe01408f1da1dc10c652a6dd535 (base this cycle rebased onto; this
  cycle's own commit SHA is recorded in `progress.md` and `git log` at push time)
- **Files touched:** `src/rules_core/rules_tables/companion_chassis.rs` (new test added),
  `docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md` (new),
  this receipt
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** "A table returns a real record or a named refusal. It never returns
  a fabricated or defaulted entry. **Evidence:** per table, a RED→GREEN pair — observed
  refusing an absent key, and returning a real record for a present one."

## What this cycle built

AT-34-E2-001 already proved fail-closed inline for the 7 new `simple_kind_tables` resolvers
(each kind's test resolves a real record **and** a fabricated-key refusal on the same table, in
one test body). This cycle formalizes AT-34-E2-002 as its own deliverable:

1. Added the one missing per-table proof: `companion` (Epic 2's 8th table, pre-existing from
   SD-29) had no dedicated fail-closed test — its resolver was only exercised incidentally
   through domain-specific tests. New test:
   `companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults`.
2. Wrote `artifacts/epic-2-tables/fail-closed-proofs.md` — the per-table table (8 rows)
   mapping each table to its resolver, its test, and its HELD/REFUSED transcript lines from
   `AT-34-E2-001_table_transcript.txt` (unmodified, already committed).

## RED → GREEN (TDD, `workflow-instruction.md §6` step 3)

**RED, confirmed for the intended reason.** `companion_resolve` was temporarily mutated to
fabricate a fallback (`.or_else(|| self.companions.first())`) instead of refusing an absent
key:

```
$ cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests::companion_resolve_refuses_a_fabricated_key_it_never_defaults
test ... FAILED
thread '...' panicked at src/rules_core/rules_tables/companion_chassis.rs:1313:9:
a fabricated key must never resolve to a companion record, real or defaulted
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2870 filtered out
```

The failure fired on the fabricated-key assertion specifically (not a compile error or
unrelated panic) — proof the test exercises the fail-closed guarantee.

**GREEN, after reverting the mutation:**

```
$ cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 2856 filtered out

$ cargo test --locked --lib rules_core::rules_tables::simple_kind_tables
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 2860 filtered out
```

## Figures + their re-derive commands (every one, with its denominator)

| Figure | Command | Denominator |
|---|---|---|
| 8 of 8 Epic 2 tables carry a proven RED→GREEN fail-closed pair | `grep -c '^\| \`' docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md` | of the 8 tables Epic 2 builds (`technical-design.md §4`; `power` is Epic 5's, out of population) |
| 15 companion_chassis tests pass (14 pre-existing + 1 new) | `cargo test --locked --lib rules_core::rules_tables::companion_chassis::tests` | of the module's own test suite |
| 11 simple_kind_tables tests pass (unchanged this cycle) | `cargo test --locked --lib rules_core::rules_tables::simple_kind_tables` | of the module's own test suite |

## Row-count command output

```
$ grep -c '^| \`' docs/release/SD-34-book-completion/artifacts/epic-2-tables/fail-closed-proofs.md
8
```
8 of 8 tables. Row count matches the required population exactly — status is `complete`.

## Build scope verified

`cargo test --locked --no-run` exits 0 at the widest workspace scope, run at
`afbe67a22f849fe01408f1da1dc10c652a6dd535` (this cycle's base, after the last commit that moves
a figure this receipt depends on — this cycle wrote no corpus records and regenerated no
inventory). `apps/desktop/src-tauri` not touched this cycle, not run.

## Sweep population

N/A — this cycle wrote no corpus records (`data/corpus/**` untouched;
`git status --porcelain data/corpus` empty both before and after). `corpus_literal_sweep`'s
examined population is unmoved by this cycle.

## Oracle pin

N/A — no figure in this cycle came from the pinned PCGen oracle corpus.

- **Status:** complete
- **Movement, four buckets:** **closure** — 0 (this criterion proves an existing property, it
  does not move a unit between buckets). **reclassification** — 0. **reachability** — 0.
  **instrument-correction** — 0 (closes a test-coverage gap on the 8th table without altering
  any counted population).

## Notes (judgment calls)

- The 7 `simple_kind_tables` tables were not given new, redundant tests this cycle — their
  AT-34-E2-001 tests already prove both halves and are cited by name in
  `fail-closed-proofs.md` rather than duplicated.
- `companion_resolve`'s RED-half mutation (`.or_else(|| self.companions.first())`) existed only
  for the duration of the RED test run; it is not present in any committed state.

## Next-cycle plan

`AT-34-E2-003` (measured build rate per table, `table-build-rate.json`) and `AT-34-E2-004`
(bucket A to zero for both vehicle books) are the remaining Epic 2 criteria.
