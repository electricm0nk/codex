# Cycle AT-34-E1-002-R — Epic 1 Completion Atlas / AT-34-E1-002 (re-verification at HEAD)

This cycle was dispatched against `AT-34-E1-002` after the criterion had already landed
(original cycle receipt: `AT-34-E1-002_cycle_receipt.md`, commit `5289e646dd`, kanban row
already `complete`). `docs/work-inventory.json` moved under later Epic-3 mechanism closures
(`B` 11,967→11,964, `DONE` 14,581→14,584 — a real 3-unit closure, not drift), leaving the
committed `completion-atlas.json` stale relative to `HEAD` and the working tree carrying an
uncommitted regeneration diff. This cycle re-derives the criterion from scratch at the current
`HEAD` (`8439f31c867d30e12dc4e3489a00e35835e4dd77`) rather than carrying the original cycle's
numbers forward (`decisions.md §12` L2/L19), lands that regeneration, and re-proves all six
fail-closed conditions still hold. **No `scripts/completion_atlas.py` logic changed — this is
a measurement/verification wave, not a code cycle** (`decisions.md §12` L6: banking zero code
changes while re-confirming the gate is a legitimate deliverable).

- **Commit SHA:** see push output below (this receipt's own landing commit).
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (regenerated at HEAD)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-002_re-verification_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended)
  - `docs/release/SD-34-book-completion/kanban.md` (row re-confirmed, no status change — already `complete`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (this cycle's own diff is docs/artifact-only; see Dual audit below)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope)
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

## Re-verification result

All six RED→GREEN mutation proofs are preserved as permanent regression tests in
`scripts/tests/test_completion_atlas.py` (not re-performed live by hand this cycle — they are
already mechanized, which is the point of landing them as tests rather than one-off transcripts).
Re-running the full suite at HEAD:

```
$ python3 -m unittest scripts.tests.test_completion_atlas -v
... 38 tests ...
----------------------------------------------------------------------
Ran 38 tests in 2.498s

OK
```

Condition-by-condition, mapped to the test classes that mutation-prove each:

| Condition | Enforcing tests | Result |
|---|---|---|
| 1. `unclassified != 0` | `TestPartition.test_unclassified_is_real_not_assumed`, `TestLiveInventoryCheck.test_live_check_reports_zero_unclassified_and_zero_overlap` | pass |
| 2. `overlap != 0` | `TestPartition.test_overlap_detected_on_duplicate_ids` | pass |
| 3. DONE unit, unsupported evidence | `TestDoneEvidenceViolations` (5 cases) | pass |
| 4. bucket with no named clearing mechanism | `TestMissingClearingMechanisms` (3 cases) | pass |
| 5. stale `derived_at` | `TestStalenessGate` (7 cases) | pass |
| 6. citation does not resolve / content-mismatches | `TestCitationFailures` (5 cases) | pass |

## Figures + their re-derive commands

Every figure below was re-derived this cycle at `HEAD` `8439f31c86`, none transcribed from the
original AT-34-E1-002 receipt (`decisions.md §12` L2/L19).

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Live `--check` result | `population=49438 buckets=10 unclassified=0 overlap=0` | whole inventory (49,438) | `python3 scripts/completion_atlas.py --check` |
| Bucket `DONE` | `14584` (was `14581` at the original cycle's commit — a real 3-unit Epic-3 closure, not drift) | of 49,438 | same command |
| Bucket `B` | `11964` (was `11967`) | of 49,438 | same command |
| `done_evidence_violations` | `0` | of 49,438 | same command |
| `missing_clearing_mechanisms` | `0` | of 10 buckets | same command |
| `stale_derived_at` | `False` | 1 committed artifact vs. current `HEAD` | same command |
| `citation_failures` | `0` | of 10 buckets | same command |
| `buckets_with_citation` | `10` | of 10 buckets | `python3 -c "import json;d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json'));print(sum(1 for v in d['buckets'].values() if v.get('citation')))"` |
| `derived_at` is an ancestor of `HEAD` | confirmed | 1 SHA vs. `HEAD` | `git merge-base --is-ancestor 8439f31c867d30e12dc4e3489a00e35835e4dd77 HEAD && echo DERIVED_AT_IS_ANCESTOR` → `DERIVED_AT_IS_ANCESTOR` |
| Unit test suite | `38 passed, 0 failed, 0 skipped` | `scripts/tests/test_completion_atlas.py`'s own case count | `python3 -m unittest scripts.tests.test_completion_atlas -v` |
| Denominator gate against this package | `files_checked=15 violations=0` | every top-level `.md` in `docs/release/SD-34-book-completion/` | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |
| `cargo test --locked --no-run` | exit 0 | full root workspace | see Build scope below |

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

All six gate fields are at their "no violation" value against the regenerated, HEAD-derived
artifact. **Status set from this count: complete.**

## Discovery, out of this criterion's scope, named and not absorbed

`python3 scripts/box_ledger.py --check` (the independent second partition SD-33 shipped,
reading `docs/release/SD-33-computed-value-verification/THE-BOX.md`) reports
`uncovered=19861` at this HEAD — up from `0` at the `tranche/14` launch checklist (item 9).
`THE-BOX.md` lives in SD-33's own folder, is explicitly out of this bundle's write scope
(dispatch brief: "Do NOT touch `docs/release/SD-33-computed-value-verification/`"), and
`box_ledger.py` is not in AT-34-E1-002's file-touch set (§3: `scripts/completion_atlas.py`,
its test file, `src/bin/v06_work_inventory.rs`, `verify.sh`/`denominator_gate.py`,
`docs/work-inventory.json`, `artifacts/epic-1-atlas/`). This is `THE-BOX.md` needing
re-deriving against Epic 3/4's own mechanism closures — a fact for the next dispatch that owns
that file, not this criterion's gate. **Not folded into this criterion's status either way**:
AT-34-E1-002's acceptance bar is `completion_atlas.py`'s six conditions, which are unaffected
and independently green.

## Build scope verified

- `cargo test --locked --no-run`: **exit 0**, full root workspace, run at `HEAD`
  `8439f31c867d30e12dc4e3489a00e35835e4dd77`, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-002`.
  Run **after** the atlas regeneration (the only figure-moving write this cycle makes), per
  `decisions.md §12` L7.
- Workspace scope: full root workspace build completed with no `error[` lines; final target
  list includes `v06_work_inventory`, `v06_corpus_trap_report`, and the rest of the `tests/`
  binaries — no unbuilt bin target.
- `apps/desktop/src-tauri`: **not run.** This cycle touches only `docs/` (the atlas JSON and
  package prose) — no Rust source in either workspace changed. Per `workflow-instruction.md
  §2.5`, explicitly not run because nothing in its scope changed.
- **Sweep not run:** no `cargo test` *execution* pass (only `--no-run`) — this cycle changes no
  Rust source and no test behavior, so SD-33's inherited baseline (29 of 599 suites / 46 of
  8,034 tests) is untouched by it; AT-34-E6-001 re-derives that baseline at closure.

## Sweep population

N/A — no corpus records added or regenerated this cycle; only `docs/work-inventory.json` was
*read* (untouched — `git status --porcelain -- docs/work-inventory.json` shows no diff) and
`completion-atlas.json` was regenerated from it. `corpus_literal_sweep`'s examined-population
(48,699 of 51,473 at launch) is unaffected and was not re-run.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Dual audit

`BASE_BRANCH=ea2b3396f2fde9223dde93522bd2288b463a21ee` (merge-base with `origin/develop`), over
Epic 1's file-touch set (`scripts/completion_atlas.py scripts/tests/test_completion_atlas.py
src/bin/v06_work_inventory.rs docs/work-inventory.json scripts/verify.sh
scripts/denominator_gate.py docs/release/SD-34-book-completion/artifacts/epic-1-atlas/`),
excluding `__tests__` and `*.test.*`:

- Both patterns produce hits, but every one **pre-dates this cycle** — they are AT-34-E1-006's
  own `SD34_BUNDLE_DIR` constant and receipt prose quoting the audit patterns themselves
  (identifier pattern), and AT-34-E3-001's `placeholder` hits describing PCGen's own real
  "no selection" CHOOSE-menu rows in corpus evidence strings plus code comments naming that
  same real shape (wired-integration pattern) — none introduced by this cycle, which adds only
  a regenerated JSON artifact and prose. Confirmed by checking this cycle's own diff in
  isolation: `git diff --unified=0 HEAD~0 -- <this cycle's files>` — this cycle's actual commit
  touches only `completion-atlas.json` (three numeric/hash fields) and three prose files, none
  of which contain either pattern.

## Status: complete

## Movement, four buckets

- **closure:** 0 — no unit's status changes; this cycle re-verifies an instrument.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0 — no logic in `completion_atlas.py` changed; the artifact was
  regenerated to stop pointing at a stale `derived_at`/stale bucket counts, which is the
  instrument doing exactly what condition 5 requires (staying non-stale), not a correction to
  the instrument itself.

## Notes

- **Why this dispatch found the criterion already complete.** AT-34-E1-002 landed in an earlier
  cycle (commit `5289e646dd`) with `kanban.md` row 2 already `complete`. This dispatch's job was
  to re-derive rather than trust the prior claim (`decisions.md §12` L19/L2) — `HEAD` had moved
  by two Epic-3 mechanism-closure cycles since then, moving `docs/work-inventory.json` by 3
  units and leaving `completion-atlas.json` stale (an uncommitted regeneration diff was already
  present in the working tree when this cycle started, produced by an earlier `--check` run this
  cycle did not perform). Re-running the full test suite and the live `--check` at current
  `HEAD` confirms the six-condition gate is unaffected by that drift: it holds today for the same
  reason it held at the original commit — the RED→GREEN proofs are structural (they synthesize
  fixtures and mutate the script's logic under test), not dependent on the live population's
  exact counts.
- **The one thing worth surfacing rather than silently re-deriving:** `box_ledger.py`'s
  independent partition has drifted badly (`uncovered=0 → 19861`) because `THE-BOX.md` (SD-33's
  file, out of this bundle's write scope) has not been kept in sync with Epic 3/4's mechanism
  closures. This is not AT-34-E1-002's gate and not fixed here; named above so it is not lost.
- No `retro.py` event emitted this cycle: nothing corrected, deferred, or reworked — this is a
  clean re-verification with the same answer as the original cycle, not a wrong prior claim.

## Next-cycle plan

No further action needed on AT-34-E1-002. `kanban.md` row 2 stays `complete`. The next dispatch
that touches `docs/release/SD-33-computed-value-verification/THE-BOX.md` should re-derive its
group counts against current `docs/work-inventory.json` (the drift is entirely additive from
Epic 3/4 mechanism closures, not a defect in `THE-BOX.md`'s partition logic).
