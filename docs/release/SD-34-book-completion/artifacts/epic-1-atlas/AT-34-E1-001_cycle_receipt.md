# Cycle AT-34-E1-001 — Epic 1 Completion Atlas / AT-34-E1-001

- **Commit SHA:** `<pending — set to the commit that lands this receipt and the files below>`
- **Files touched:**
  - `scripts/completion_atlas.py` (new)
  - `scripts/tests/test_completion_atlas.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (new, committed)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-001_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (updated)
  - `docs/release/SD-34-book-completion/kanban.md` (updated)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-34-E1-001 — every unit carries exactly one named remaining-step
  >
  > `scripts/completion_atlas.py` partitions all **49,438** units into the buckets fixed by
  > `decisions.md §2`: `DONE`, `A`, `B`, `C`, `D`, `M`, `V`, `U`, `X`, `Z`. Each bucket carries a
  > count, a **named mechanism that clears it**, and a re-derive command.
  >
  > **Evidence:** `python3 scripts/completion_atlas.py --check` exits 0 and prints
  > `population=49438 buckets=10 unclassified=0 overlap=0`, with the bucket counts summing to the
  > population. A committed `artifacts/epic-1-atlas/completion-atlas.json`.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Inventory population (live `units` array length) | 49,438 | whole inventory | `python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['units']))"` |
| `completion_atlas.py --check` result | `population=49438 buckets=10 unclassified=0 overlap=0`, exit 0 | whole inventory | `python3 scripts/completion_atlas.py --check` |
| Bucket `DONE` | 12,265 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  DONE:'` |
| Bucket `A` | 8,463 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  A:'` (matches `epic-breakdown.md`'s stated 8,463 across 9 kinds) |
| Bucket `B` | 11,921 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  B:'` |
| Bucket `C` | 4,388 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  C:'` |
| Bucket `D` | 1,230 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  D:'` |
| Bucket `M` | 2,455 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  M:'` |
| Bucket `V` | 8,330 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  V:'` |
| Bucket `U` | 321 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  U:'` (matches `epic-breakdown.md`'s named 321: 270 + 51 by evidence, 140+119+62 by kind) |
| Bucket `X` | 46 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  X:'` |
| Bucket `Z` | 19 | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check \| grep '  Z:'` |
| Sum of all 10 buckets | 49,438 | matches population exactly | equal by construction; `unclassified=0 overlap=0` proves it live |
| `box_ledger.py --check` (independent, read-only second partition) | `uncovered=0 overlap=0 population=49438` | full 49,438-unit inventory | `python3 scripts/box_ledger.py --check` — confirms the atlas's population agrees with SD-33's inherited independent partition |
| `denominator_gate.py` against this package | `files_checked=15 violations=0` | every `.md` in `docs/release/SD-34-book-completion/` (explicit glob, per `decisions.md §3` — default scope not yet widened, that is AT-34-E1-006) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |
| Unit test suite | 18 passed, 0 failed, 0 skipped | `scripts/tests/test_completion_atlas.py`'s own case count | `python3 -m unittest scripts.tests.test_completion_atlas -v` |

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json'))
print('population', d['population'])
print('bucket_count', len(d['buckets']))
print('sum_of_buckets', sum(v['count'] for v in d['buckets'].values()))
print('unclassified', d['unclassified'])
print('overlap', d['overlap'])
"
population 49438
bucket_count 10
sum_of_buckets 49438
unclassified 0
overlap 0
```

## Build scope verified

- `cargo test --locked --no-run`: exit 0. Ran at HEAD (the commit landing this receipt, once
  pushed). Full deps/test-binary list built with no compile errors (log tail confirms every
  `Executable tests/...` line through the last test target, no `error[` lines).
- Workspace scope: full root workspace (`cargo test --locked --no-run` from repo root).
- `apps/desktop/src-tauri`: **not run.** This cycle touched only `scripts/` (Python) and
  `docs/` — no Rust source, no desktop-crate surface. Per `workflow-instruction.md §2.5`,
  "test it explicitly or not at all" — explicitly not run because nothing in its scope changed.
- Run this after the last commit that can move a figure this receipt depends on
  (`decisions.md §12` L7): yes — the atlas artifact and this receipt were generated from the
  same working tree state committed together; no regeneration happens after.

## Sweep population

N/A — this cycle added no corpus records and regenerated no `data/corpus/**` files. It reads
`docs/work-inventory.json` only; `corpus_literal_sweep`'s examined-population is unaffected
and was not re-run.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus. (Bucket `V`,
"verified by proxy, never by the oracle," is precisely the population this cycle counts but
does not itself re-verify against the oracle — that is the oracle harness's job, unchanged by
this cycle.)

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle builds the atlas instrument; it does not move any unit's status.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

No unit's status changed in `docs/work-inventory.json`. This cycle's deliverable is the
partition mechanism itself (`decisions.md §2`/`§12` L4 — "sum the piles, always" — the atlas is
the mechanism, not a quote of the rule).

## Notes

- **Population confirmed, not assumed.** Live `len(units)` (49,438) matches `totals.units`
  (49,438); both checked, neither trusted alone, per `AGENTS.md` "every figure carries the
  command that produced it."
- **Bucket derivation keys on `status` **and** `evidence`**, exactly as specified in
  `technical-design.md` §1's "Bucket derivation, as implemented" table — not on `status` alone,
  since `not-ingested` alone does not distinguish A from B from C from D. `_bucket_of` is a
  pure function tested directly (unit-level) and through the live corpus (acceptance-level).
- **A/U bucket figures cross-checked against the epic-breakdown's independently-stated
  numbers** (8,463 for A across 9 kinds; 321 for U, split 270/51 by evidence and 140/119/62 by
  kind) — both matched exactly on the first live run, no correction needed this cycle.
- **`D` and `U` sub-causes are enumerated in the committed artifact**, not shrugged: the
  `sub_causes` field in `completion-atlas.json` carries the full per-evidence-string breakdown
  for both buckets (per `decisions.md §2`: "a holding pen with a census, never a shrug").
- **AT-34-E1-002 is explicitly NOT implemented this cycle.** This is a separate criterion in
  the same file (the six fail-closed conditions). `BUCKET_DEFINITIONS` already carries the
  `clears` and `evidence_source` fields that criterion's conditions 4 and 6 will assert
  against, so the next cycle extends this file rather than reshaping it. Only condition 1
  (`unclassified != 0` -> non-zero exit) is already live, because `--check`'s own evidence bar
  in *this* criterion requires it.
- **RED→GREEN preserved:** (1) the full 18-test suite is GREEN
  (`python3 -m unittest scripts.tests.test_completion_atlas -v`). (2) A live mutation proof:
  `_A_MARKER` was changed from `"has_no_engine_table"` to a string that matches nothing, which
  correctly turned `test_not_ingested_splits_a_by_evidence` (`'D' != 'A'`) and
  `test_bucket_a_matches_named_population` (`0 != 8463`) RED for the intended reason — every
  bucket-A unit silently fell through to `D` rather than raising — then the change was reverted
  and the suite returned to GREEN (`OK`, 18 tests). No committed file was ever left mutated
  (`git status --porcelain` clean of the mutation both before and after).
- **Dual-audit gate re-run on the staged diff** (not just the working tree), scoped to this
  cycle's three new files: `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`.
- Left `docs/retro/events/sd31-transcribe.jsonl`'s pre-existing dirty state untouched — another
  concurrent lane's file, per the one-writer-per-tree rule.
- Also present but untouched (not this cycle's files, not referenced by this criterion):
  the untracked `*.workflow.js` litter under `docs/release/SD-33-computed-value-verification/artifacts/`
  and `docs/release/SD-34-book-completion/artifacts/sd-34-dispatch.workflow.js`.
- No `retro.py` event emitted this cycle: nothing corrected, deferred, or reworked — this is a
  fresh instrument build whose live figures matched the epic-breakdown's own pre-stated numbers
  on the first run.

## Next-cycle plan

`AT-34-E1-002` extends `scripts/completion_atlas.py` in place with the remaining five
fail-closed conditions (a `DONE` unit whose evidence does not support it; a bucket with no
named clearing mechanism; the `derived_at`-is-an-ancestor-of-`HEAD` staleness gate; and the
`file:line` evidence-source citation, asserted against cited *content* not just path/line),
each with its own RED→GREEN mutation proof, written to
`artifacts/epic-1-atlas/fail-closed-proofs.md`. `AT-34-E1-003` then uses this cycle's bucket-A
partition to enumerate the 9 missing-table kinds' per-book coverage.
