# Cycle wave-26-gate-lane-a-trap-baseline — Epic 6 (gate remediation) / AT-34-E6-001 (Lane A)

**Filename note (same convention wave-23/24/26's own receipts use).** This cycle's dispatch
brief (`artifacts/sd-34-dispatch.workflow.js`'s `ucLanePrompt()`) reuses the label
`AT-34-E6-001` for a gate-remediation lane, distinct from `kanban.md` row 26's canonical
`AT-34-E6-001` (`final-acceptance-scan`, still `not-started` — untouched by this cycle).
Writing to the literal path the brief names
(`artifacts/epic-6-closure/AT-34-E6-001_cycle_receipt.md`) would silently overwrite the genuine
2026-08-29 `canonical: true` final-acceptance-scan FAIL-verdict receipt already on disk there.
Filed here instead. This cycle is also, by `scripts/wave_ledger.py`'s own `KNOWN_WAVES` table
(worktree prefix `wf_4cb8e9fe-c43`), **wave 26's own Lane A** — the same wave and lane as
`AT-34-E6-001_gate-lane-a_wave26_cycle_receipt.md`, which closed 3 of that dispatch's 4 named
mechanisms and explicitly **routed** the 4th (this one, `v06_corpus_trap_report.rs`) rather than
implementing it. This receipt is that routed item, closed — filed under a distinguishing
`_trap-baseline` suffix rather than overwriting the sibling wave-26 receipt that already recorded
real, separate work.

- **Commit SHA:** `a5eafad137` (last figure-moving commit; also this receipt's own build-scope
  SHA — no further commits landed on Rust source, tests, or governance docs after it before this
  receipt was written)
- **Files touched:**
  - `docs/governance/corpus-trap-baseline.tsv` (new — the registered baseline, 4 rows)
  - `src/pcgen_import/corpus_trap_baseline.rs` (new — parse/reconcile module, 9 unit tests)
  - `src/pcgen_import/mod.rs` (+1 line, `pub mod corpus_trap_baseline;`)
  - `tests/v06_corpus_trap_report.rs` (4 corpus-invariant tests rewired to the baseline
    reconciler; +1 new mutation-proof integration test)
  - `docs/retro/events/sd34-at-34-e6-001.jsonl` (1 `correction` event, this cycle's own)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` with one noted exception, the same
  repo's-own-test-filename shape wave-24/26's receipts already established as not a defect: one
  hit is a doc-comment citation of `sd30_declared_product_identity_in_shipped_class_features`
  (this repo's own permanent test filename, cited as the pattern this cycle's mutation-proof test
  is modelled on — the acceptance brief itself names that exact test as the model to follow).
  Command:
  `git diff --unified=0 HEAD~1 -- tests/v06_corpus_trap_report.rs src/pcgen_import/corpus_trap_baseline.rs src/pcgen_import/mod.rs docs/governance/corpus-trap-baseline.tsv | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  (the trailing `\b` is deliberately omitted from this pattern per the workflow instruction —
  adding it back would falsely report `OK_NO_BUNDLE_TAGS` here, since `sd30_declared...` has no
  word boundary between its trailing `_` and the next word character)
  → 1 hit, the citation above (re-derivable: `BASE_BRANCH=$(git merge-base HEAD origin/develop)`
  = `ea2b3396f2`; the committed-history form `BASE_BRANCH...HEAD` over the same paths returns
  `OK_NO_BUNDLE_TAGS` with zero hits, since this cycle's only commit is entirely new content —
  the working-tree form against the immediately-prior `HEAD` is the meaningful one and is quoted
  above).
- **Wired-integration audit result:** `OK_NO_TOKENS` — zero hits of
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` in the same diff.
- **Acceptance criterion (verbatim from this cycle's dispatch brief, `ucLanePrompt()` in
  `artifacts/sd-34-dispatch.workflow.js`):** "GATE LANE A — root-full: implement the ruling the
  trap tests never implemented... Implement §13: baseline them, exactly the way `pi-sweep`
  already does it in this repo... This is NOT weakening the test and your receipt must show why
  not... prove it fires with a mutation test, the way `sd30_declared_product_identity...`'s own
  `the_leak_detectors_actually_fire_on_a_planted_leak` does. Do NOT drive the 3,181 to zero —
  that is `AT-34-E1-008`'s scope, not yours."

## What this cycle did

`decisions.md §13` rules that four of `v06_corpus_trap_report.rs`'s corpus-invariant tests guard
a population that is SD-33's already-verified, already-out-of-DoD inherited debt
(`forward-scope-register.md` D1.1), "registered, not absorbed" — `AT-34-E1-008`'s own bar is
`wiring-class-mismatch = 0`, a fifth, unrelated trap kind. All four tests still asserted
`violations.is_empty()`, so `root-full` was red forever for a known, unchanging reason and told
no reader anything new on any given run.

**New module `src/pcgen_import/corpus_trap_baseline.rs`** parses
`docs/governance/corpus-trap-baseline.tsv` (`trap_id<TAB>count<TAB>note`, the same shape
`pi_table_sweep::parse_baseline` uses for `pi-sweep-baseline.tsv`) and reconciles a live count
against it via `reconcile_trap_count`, returning one of four verdicts: `Matched` (equal — the
test passes), `Added` (live > baseline — a real regression), `Stale` (live < baseline — the debt
shrank without the row being updated), `Unbaselined` (no row for that trap id). This is
symmetric by construction: `Matched` is the only passing case, so both directions
`pi_sweep_rules_tables` enforces are enforced here too.

**All four tests** (`no_two_ingested_records_share_a_record_key`,
`ingested_record_keys_match_their_cited_line`, `every_mod_sourced_ingest_has_a_live_base_
declaration`, `no_ingested_record_is_sourced_from_a_disabled_line`) now call
`reconcile_trap_count` and assert `verdict.is_matched()` instead of `violations.is_empty()`.
`ingested_record_keys_match_their_cited_line`'s pre-existing `KNOWN_KEY_MISMATCH_DEBT` allowlist
(the ACG-Naturalist / equipment-citation debt already paid to zero) is left in place unchanged —
it still filters those specific already-fixed rows out of scope before the baseline check runs,
so a regression of *those* rows would show up as `unexpected` exceeding the baseline just as
readily as any other new finding.

**Data changed: none.** `data/corpus/**` is not in this cycle's diff. No trap count was driven
toward zero — that is `AT-34-E1-008`'s scope per `decisions.md §13`, explicitly not this one's.

**Mutation proof, two layers** (the brief's own bar: "if you cannot make it detect a planted
regression, you have weakened it"):

1. **Unit-level** (`src/pcgen_import/corpus_trap_baseline.rs`'s own `#[cfg(test)]` module, 9
   tests): `a_live_count_above_baseline_is_added_not_silently_accepted`,
   `a_live_count_below_baseline_is_stale_not_silently_accepted`, plus matched/unbaselined/
   parse-error cases.
2. **Integration-level, modelled on `sd30_declared_product_identity_in_shipped_class_features::
   the_leak_detectors_actually_fire_on_a_planted_leak_and_clear_on_a_redacted_row`** (the exact
   test the brief names): new test
   `the_trap_baseline_reconciler_actually_fires_on_a_planted_regression_and_a_stale_row` in
   `tests/v06_corpus_trap_report.rs`, against a synthetic in-memory baseline, never touching the
   real corpus or the real baseline file. Runs unconditionally (no `PCGEN_CORPUS_ROOT` needed).
3. **End-to-end, against the real corpus and the real baseline file** (beyond what either proof
   above covers, and beyond what the brief strictly required — done to be certain the wiring
   itself, not just the reconciler function, actually fires): temporarily edited
   `docs/governance/corpus-trap-baseline.tsv`'s `disabled-line` row `165 → 164`, ran
   `no_ingested_record_is_sourced_from_a_disabled_line` against the live PCGen corpus — **FAILED**,
   message `live count 165 exceeds its registered baseline 164 ... a real regression`. Then
   `164 → 166`, re-ran — **FAILED**, message `live count 165 is BELOW its registered baseline
   166 ... the debt shrank without the row being updated`. Then restored `165` and re-ran the
   full suite — **26 passed / 0 failed**. Logs kept at `/tmp/cargo-sd34-at-34-e6-001/mut1.log`,
   `mut2.log` (not committed — scratch, per the environment's own `/tmp` convention).

## Figures + their re-derive commands

| Figure | Value | Command / denominator |
|---|---:|---|
| `shared-name-distinct-records` (baseline row) | 249 | `cargo test --locked --test v06_corpus_trap_report no_two_ingested_records_share_a_record_key -- --nocapture 2>&1 \| grep -c "Finding {"` — count of `Trap::SharedNameDistinctRecords` findings over `audit_ingested_cache(data/corpus, $PCGEN_CORPUS_ROOT)` |
| `key-differs-from-name` (baseline row) | 650 | `cargo test --locked --test v06_corpus_trap_report ingested_record_keys_match_their_cited_line -- --nocapture 2>&1 \| grep -c "Finding {"` — count of `Trap::KeyDiffersFromName` findings, same audit; identical to `unexpected.len()` since `KNOWN_KEY_MISMATCH_DEBT` is empty |
| `mod-record` (baseline row) | 2,117 | `cargo test --locked --test v06_corpus_trap_report every_mod_sourced_ingest_has_a_live_base_declaration -- --nocapture 2>&1 \| grep -c "Finding {"` — count of `Trap::ModRecord` findings with `Severity::Defect` only (the `Severity::Trap` subset, legitimate `.MOD` usage, is not in this population) |
| `disabled-line` (baseline row) | 165 | `cargo test --locked --test v06_corpus_trap_report no_ingested_record_is_sourced_from_a_disabled_line -- --nocapture 2>&1 \| grep -c "Finding {"` — count of `Trap::DisabledLine` findings, same audit |
| Sum of the four | 3,181 | 249 + 650 + 2,117 + 165 — matches `decisions.md §13`'s stated total and wave-24/26's own identical re-derivations exactly; **unchanged**, confirming this cycle is instrument-correction only, not corpus content change |
| `v06_corpus_trap_report` suite | 21 passed / 4 failed → 26 passed / 0 failed | `cargo test --locked --test v06_corpus_trap_report` before/after (the +1 beyond 21+4=25 is this cycle's own new mutation-proof test) |
| `corpus_trap_baseline` unit tests | 9 passed / 0 failed | `cargo test --locked --lib corpus_trap_baseline` |
| `cargo test --locked --no-run` (whole workspace) | exit 0, 589 executables built | see Build scope below |
| `apps/desktop/src-tauri` `cargo test --locked --no-run` | exit 0 | see Build scope below |

`$PCGEN_CORPUS_ROOT` resolved via the standing `~/workspace/repos/pcgen` checkout, pinned at
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) —
load-bearing for every figure above, none of which came from any other source.

## Row-count command output

```
$ cargo test --locked --test v06_corpus_trap_report -- --nocapture
```
```
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 51.41s
```

This cycle's own artifact is the four rewired tests plus the new mutation-proof test — **26
passed / 0 failed** is the literal count that sets this cycle's status (`decisions.md §4`): all
26 including the 4 named in the acceptance criterion, and 0 outstanding.

## Build scope verified

- `cargo test --locked --no-run` (whole workspace, isolated `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001`, run at commit `a5eafad137`): **exit 0**, `Finished \`test\` profile [unoptimized + debuginfo] target(s) in 3m 23s`, 589 `Executable` lines.
- `apps/desktop/src-tauri` (separate cargo workspace, isolated `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001-desktop`, run at the same commit): **exit 0**, `Finished \`test\` profile [unoptimized + debuginfo] target(s) in 3m 02s`.
- Full untargeted `cargo test --locked --no-fail-fast` over the ~600-suite root workspace was
  **not run** this cycle — the memory/wall-time hazard this bundle's brief warns against, same
  gap wave-23/24/26's own receipts name. This cycle's own touched surface (the new module's unit
  tests, the full `v06_corpus_trap_report` integration target) was run directly and is green; the
  `--no-run` pass above proves nothing else in the workspace was broken by this cycle's change
  (a Rust build fails at the whole-crate/whole-target level on any type error, so a clean
  `--no-run` after this cycle's commit rules out a compile-time regression anywhere in either
  workspace).

## Sweep population

**N/A.** This cycle does not touch `data/corpus/**` — `corpus_literal_sweep`'s examined
population is unaffected (`decisions.md §12` L8 applies only when corpus records are added or
regenerated; none were).

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`),
resolved via the standing `~/workspace/repos/pcgen` checkout — load-bearing for every figure in
the Figures table above (each came from `audit_ingested_cache` reading that checkout).

## Status

**complete.**

The acceptance criterion is: `root-full`'s blocking `v06_corpus_trap_report.rs` tests implement
`decisions.md §13`'s ruling, without driving the 3,181 to zero, proven not to be a weakened test
via a mutation test. All three parts are done: the 4 tests are baselined (row-count above: 26/26
green, 0 failed), `data/corpus/**` is untouched (3,181 unchanged, confirmed by re-derivation),
and the mechanism is mutation-proved at both the unit level and the integration level (modelled
on the named `sd30` test), plus an end-to-end proof against the real corpus and the real baseline
file that neither this receipt's brief nor `sd30`'s own precedent strictly required.

This cycle does **not** claim `root-full` itself is green — wave-28's own sweep named two other
failing targets in that stage (`sd24_wired_integration_audit`, 1 test;
`sd27_pathfinder_unchained_cache_shape`, 2 tests) that are outside this criterion's named scope
(`tests/v06_corpus_trap_report.rs` only) and outside this cycle's territory diff. Re-derived live
this cycle (not carried from wave-28): `cargo test --locked --test sd24_wired_integration_audit`
and `cargo test --locked --test sd27_pathfinder_unchained_cache_shape` were **not re-run** this
cycle — naming them here rather than silently assuming wave-28's figure is still current is the
honest disclosure; the next cycle closing `root-full` must re-derive them at its own HEAD, not
carry this receipt's mention of them forward as a count.

## Movement, four buckets

- **Closure:** 0 inventory-bucket units moved (no `docs/work-inventory.json` touch — this is a
  gate-remediation lane over a test instrument, not a content-completion cycle).
- **Reclassification:** 0.
- **Reachability:** N/A.
- **Instrument-correction:** 1 mechanism — the 4 `v06_corpus_trap_report.rs` corpus-invariant
  tests, re-pinned from an un-implementable zero-tolerance bar to the baseline-reconciliation bar
  `decisions.md §13` actually specifies, with the underlying population (3,181) independently
  re-derived and confirmed unchanged, not assumed. This is squarely an instrument-correction, not
  a closure: the test now measures the right thing, and the thing it measures did not move.

## Notes (judgment calls)

- **The `ingested_record_keys_match_their_cited_line`'s `KNOWN_KEY_MISMATCH_DEBT` allowlist was
  kept, not removed**, even though it is currently empty and therefore contributes nothing to
  today's pass/fail outcome (baseline count 650 == `unexpected.len()` == `mismatches.len()`
  exactly while the allowlist is empty). Removing it would have deleted institutional memory of
  the ACG-Naturalist and equipment-citation defects it already ratcheted to zero, and it remains
  the correct home for any *future* narrowly-scoped debt this repo chooses to enumerate by
  specific `(book, key)` identity rather than folding into the aggregate baseline.
- **This receipt's filename departs from the literal path the dispatch brief names**, for the
  same reason wave-26's own sibling receipt did (see the filename note at the top) — the literal
  path is a real, `canonical: true`, distinct artifact (the final-acceptance-scan verdict) that
  must not be overwritten by a differently-scoped gate-remediation cycle reusing the same
  criterion id.
- **`kanban.md` is intentionally left untouched.** Row 26 (`AT-34-E6-001`,
  `final-acceptance-scan`) is a different, broader criterion this cycle does not satisfy — same
  precedent wave-26's own receipt set.
- **This cycle went beyond the brief's literal mutation-test bar** (one integration test modelled
  on `sd30`'s own) by also proving the wiring end-to-end against the real corpus and the real
  baseline file (the temporary `disabled-line` edits, §"What this cycle did" item 3). The brief's
  bar (a synthetic mutation test proving the reconciler function itself can fail) is fully met by
  the unit and integration tests alone; the end-to-end pass is extra assurance, not a substitute
  for either.

## Next-cycle plan

`root-full`'s two other named-red targets (from wave-28's own sweep, not re-verified live this
cycle — see Status above): `sd24_wired_integration_audit.rs` (1 test — widen the allowlist for
legitimate "placeholder" prose at `reach_gate.rs:3192`) and
`sd27_pathfinder_unchained_cache_shape.rs` (2 tests — restate `42→38` / `7→3` to match the
corrected corpus). Neither is this criterion's or this cycle's to close — named here so the next
lane re-derives rather than re-discovers them.
