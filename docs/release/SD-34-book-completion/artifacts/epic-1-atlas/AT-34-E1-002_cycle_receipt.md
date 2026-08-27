# Cycle AT-34-E1-002 — Epic 1 Completion Atlas / AT-34-E1-002

- **Commit SHA:** filled in below at the commit that lands this receipt (see "Notes" if the
  in-flight SHA differs from the pushed one after rebase).
- **Files touched:**
  - `scripts/completion_atlas.py` (extended: five new fail-closed conditions + citations)
  - `scripts/tests/test_completion_atlas.py` (extended: 20 new permanent unit tests)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (regenerated)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/fail-closed-proofs.md` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-002_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (updated)
  - `docs/release/SD-34-book-completion/kanban.md` (updated)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-34-E1-002 — the atlas fails closed on six conditions
  >
  > Exits non-zero on:
  >
  > 1. `unclassified != 0`
  > 2. `overlap != 0`
  > 3. a unit in `DONE` whose evidence does not support it
  > 4. a bucket with no named clearing mechanism
  > 5. a `derived_at` SHA that is not an ancestor of `HEAD` (**staleness gate**)
  > 6. **a bucket whose definition does not cite the `file:line` that emits the evidence
  >    strings it keys on — or whose citation no longer resolves at `HEAD`**
  >
  > **Evidence:** six RED→GREEN mutation proofs, one per condition, in the receipt. **A tool
  > never observed to fail is not a gate.**

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Live `--check` result | `population=49438 buckets=10 unclassified=0 overlap=0` `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`, exit 0 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check` |
| Condition 1 RED | `unclassified=19` (all real bucket-Z units mis-fell through), exit 1 | whole inventory | see `fail-closed-proofs.md` condition 1 |
| Condition 2 RED | `overlap_ids=[]` on a synthetic duplicate-id pair (real defect went undetected) | 2-unit synthetic fixture | see `fail-closed-proofs.md` condition 2 |
| Condition 3 RED | `_done_evidence_violations` returns `[]` on a synthetic DONE unit carrying an A-bucket marker | 1-unit synthetic fixture | see `fail-closed-proofs.md` condition 3 |
| `explanation_id` false-positive check | 245 real DONE units legitimately carry it | whole inventory (49,438) | `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));print(sum(1 for u in d['units'] if u.get('status') in ('grounded','text-complete') and 'explanation_id' in (u.get('evidence') or '')))"` |
| Condition 4 RED | `missing_clearing_mechanisms=1` (`Z`), exit 1 | 10 buckets | see `fail-closed-proofs.md` condition 4 |
| Condition 5 RED | `stale_derived_at=True` on a hand-edited bogus `derived_at`, exit 1 | 1 committed artifact | see `fail-closed-proofs.md` condition 5 |
| Condition 6 RED | `citation_failures=1` (`A`, content mismatch on a resolving line), exit 1 | 10 buckets | see `fail-closed-proofs.md` condition 6 |
| `box_ledger.py --check` (independent second partition, read-only, unchanged by this cycle) | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False` | full 49,438-unit inventory | `python3 scripts/box_ledger.py --check` |
| Denominator gate against this package | `files_checked=15 violations=0` | every top-level `.md` in `docs/release/SD-34-book-completion/` (explicit glob; default scope not yet widened, that is AT-34-E1-006) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |
| Unit test suite | 38 passed, 0 failed, 0 skipped (20 new + 18 inherited) | `scripts/tests/test_completion_atlas.py`'s own case count | `python3 -m unittest scripts.tests.test_completion_atlas -v` |

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json'))
print('population', d['population'])
print('sum_of_buckets', sum(v['count'] for v in d['buckets'].values()))
print('unclassified', d['unclassified'])
print('overlap', d['overlap'])
print('done_evidence_violations', d['done_evidence_violations'])
print('missing_clearing_mechanisms', d['missing_clearing_mechanisms'])
print('stale_derived_at', d['stale_derived_at'])
print('citation_failures', d['citation_failures'])
print('buckets_with_citation', sum(1 for v in d['buckets'].values() if v.get('citation')))
"
population 49438
sum_of_buckets 49438
unclassified 0
overlap 0
done_evidence_violations 0
missing_clearing_mechanisms 0
stale_derived_at False
citation_failures 0
buckets_with_citation 10
```

All ten buckets carry a resolving, content-matching citation; all six fail-closed conditions
report clean on the live, unmutated committed state. **Status set from this count: complete**
— every one of the six gate fields is at its "no violation" value, matching the six-condition
acceptance bar exactly (10 of 10 buckets cited, 0 of 4 population-scoped violation counts
nonzero).

## Build scope verified

- `cargo test --locked --no-run`: **exit 0**, run at `ceac19da29` (this cycle's parent commit
  — no Rust source changed by this cycle, only `scripts/` Python and `docs/`; per
  `decisions.md §12` L7 the build-scope check need not re-run after a commit that cannot move
  a figure this receipt depends on, and this cycle's only "figure-moving" commit is the one
  landing this receipt itself, which touches no Rust). Full dependency/test-binary list built
  through the last target with no `error[` lines.
- Workspace scope: full root workspace (`cargo test --locked --no-run` from repo root),
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-002`.
- `apps/desktop/src-tauri`: **not run.** This cycle touched only `scripts/` (Python) and
  `docs/` — no Rust source, no desktop-crate surface. Per `workflow-instruction.md §2.5`,
  explicitly not run because nothing in its scope changed.

## Sweep population

N/A — no corpus records added or regenerated this cycle; reads `docs/work-inventory.json`
only. `corpus_literal_sweep`'s examined-population is unaffected and was not re-run.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle hardens the atlas instrument (adds fail-closed gates); it does
  not move any unit's `status` or `evidence` in `docs/work-inventory.json`.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

Verified: `docs/work-inventory.json` is untouched this cycle (`git status --porcelain` shows
no diff on that path); only `completion_atlas.py`, its test file, and this package's own
artifacts changed.

## Notes

- **Condition-3 marker set was deliberately narrowed after a real corpus check.** The naive
  design (reuse `_A_MARKER`/`_B_MARKERS`/`_C_MARKERS` verbatim as "markers that must never
  appear in DONE evidence") would have flagged 245 real, legitimate `DONE` units whose
  evidence contains `explanation_id` — itself the exact "field name vs. field meaning"
  mistake condition 6 exists to guard against (`decisions.md §12` L1), turned inward.
  `_DONE_VIOLATION_MARKERS` excludes `explanation_id`; the exclusion, its rationale, and the
  live corpus count (245, confirmed 0 for the five other markers) are recorded in code comments
  and in `fail-closed-proofs.md`.
- **Condition 5's staleness check reads the artifact as committed, before this run's own
  write.** Checking a freshly-stamped `derived_at == HEAD` against `HEAD` is trivially true
  and would prove nothing; the real gate is whether the *prior* commit's stamped claim still
  resolves as an ancestor after this cycle's history. `_staleness_violation(artifact_path=...)`
  takes an explicit path so tests exercise it against synthetic files without touching the
  real committed artifact between assertions.
- **Condition 6's citations are ten concrete `file:line` pairs into
  `src/bin/v06_work_inventory.rs`**, chosen as the first (or, for `not_held_by_engine`,
  `has_no_engine_table`, `explanation_id`/`diagnostic`, and `not-ingested`, a representative)
  real source line that emits the literal marker/status string each bucket keys on — not the
  generic `not_ingested`/`not_ingested_owned` closures' own line alone for A/B/C, since those
  closures take the marker as a caller-supplied argument and do not themselves contain the
  literal substring. `D`'s citation legitimately points at the shared closure (line 8346,
  `status: "not-ingested"`) because `D` **is** the fallthrough of that same status with no
  further marker to key on.
- **RED→GREEN preserved for all six conditions**, each performed live against the real script
  and/or the real committed artifact, each reverted before the next proof and before this
  receipt was written (`fail-closed-proofs.md` carries every transcript).
  `git status --porcelain` confirmed clean of any mutation residue after every revert.
- **`--book <slug> --check` path is unchanged** by this cycle — the five new conditions are
  population-scoped gates on the atlas instrument itself, not on a single book's partition;
  AT-34-E2-004/AT-34-E3-005/AT-34-E4-002 continue to use the existing `--book` behavior.
- Left `docs/retro/events/sd31-transcribe.jsonl`'s pre-existing dirty state untouched (another
  concurrent lane's file). Also present but untouched: the untracked `*.workflow.js` litter
  under `docs/release/SD-33-computed-value-verification/artifacts/` and
  `docs/release/SD-34-book-completion/artifacts/sd-34-dispatch.workflow.js`.
- No `retro.py` event emitted this cycle: nothing corrected, deferred, or reworked — the
  fail-closed conditions were designed once (accounting for the real corpus check on condition
  3 above) and landed green on the intended shape without a wrong prior claim to correct.

## Next-cycle plan

`AT-34-E1-003` uses this cycle's hardened atlas — specifically bucket A's partition — to
enumerate the 9 missing-table kinds' per-book coverage (units, core-rulebook slice, engine
surface, exercised-by-core flag), written to `artifacts/epic-1-atlas/missing-engine-tables.json`.
