# Cycle AT-33-E1-002 — Epic 1 Instruments / AT-33-E1-002

- **Commit SHA:** `9a52667cc5`
- **Files touched:**
  - `scripts/box_ledger.py` (extended — same file `AT-33-E1-001` created)
  - `scripts/tests/test_box_ledger.py` (extended)
  - `docs/release/SD-33-computed-value-verification/THE-BOX.md` (extended — `"unverifiable"` field added to every ledger group)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/AT-33-E1-002_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (updated)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E1-002 — `box_ledger.py` fails closed on all five conditions
  >
  > The tool exits non-zero on: `uncovered != 0`; `overlap != 0`; oracle disagreement; an `unverifiable` unit dispositioned `done`; and a `derived_at` SHA that is not an ancestor of `HEAD` (**staleness gate**).
  >
  > **Evidence:** five RED→GREEN mutation proofs, one per condition, in the cycle receipt. A tool that has never been observed to fail is not a gate.

## What this cycle built

`AT-33-E1-001` (prior cycle) implemented conditions 1–2 (`uncovered`, `overlap`) only.
This cycle extends the same `scripts/box_ledger.py` with the remaining three
mechanisms named in `decisions.md` §1:

- **Condition 3 — oracle disagreement.** `--oracle-results <path>` (default
  `artifacts/epic-2-oracle-harness/oracle-results.json`, which does not exist
  yet — Epic 2 hasn't landed) reads a list of `{"unit_id", "ours", "oracle",
  "verdict"}` records — the exact shape `AT-33-E2-003`'s comparison harness is
  specified to return (`ours`, `oracle`, `agree|disagree|unverifiable`).
  `verdict == "disagree"` gates; `"unverifiable"` is accepted as a first-class
  non-failing outcome, per that criterion's own bar. When no oracle-results
  file exists, the check prints an `INFO` line naming that fact and evaluates
  to zero disagreements — **wired, not stubbed**: it activates automatically
  the moment Epic 2 writes a real file at that path, with no second cycle
  needed to turn it on.
- **Condition 4 — an `unverifiable` unit dispositioned `done`.** `THE-BOX.md`'s
  ledger schema gained a `"unverifiable": true/false` boolean per group (every
  one of the 9 groups now states it explicitly — no group can silently omit
  it). `box_ledger.py` fails if any group carries both `"unverifiable": true`
  and `"disposition": "done"` at once — exactly the over-claim SD-32's
  `doneness_verdict()` made for 8,330 units (`decisions.md` §7).
- **Condition 5 — the `derived_at` staleness gate.** `load_front_matter()`
  parses `THE-BOX.md`'s YAML front matter; `check_staleness()` runs
  `git merge-base --is-ancestor <derived_at> HEAD` and fails if the recorded
  SHA is missing, garbage, or not an ancestor of current `HEAD`.

`--check`'s summary line now reads:
`uncovered=<n> overlap=<n> population=<n> oracle_disagreement=<n> unverifiable_done=<n> stale=<bool>`,
and the tool's exit code is non-zero if **any** of the five conditions fires.

## Five RED→GREEN mutation proofs (live evidence, against real/real-derived files)

All five run from repo root at commit `08bfa4931d`'s tree plus this cycle's
changes (i.e. against the real committed `THE-BOX.md`, or a temp copy of it
with one deliberate mutation — never the temp copy overwriting the committed
file; `git status --porcelain` was clean of these paths before and after
each proof).

**1. `uncovered != 0`** — temp copy of `THE-BOX.md` with the `unknown` group's
ledger entry deleted:
```
$ python3 scripts/box_ledger.py --check --box /tmp/.../THE-BOX-cond1.md
uncovered=4224 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
UNCOVERED: advanced_class_guide:class_feature:abjuration_opposition_savant_school, ... (+4214 more)
exit=1
$ python3 scripts/box_ledger.py --check --box docs/release/SD-33-computed-value-verification/THE-BOX.md
exit=0
```

**2. `overlap != 0`** — temp copy with an extra `overlap-probe` group added
whose `match` collides with `text-complete`'s status:
```
$ python3 scripts/box_ledger.py --check --box /tmp/.../THE-BOX-cond2.md
uncovered=0 overlap=5099 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
OVERLAP: advanced_class_guide:class_feature:bloodrager_blood_casting, ... (+5089 more)
exit=1
```
(GREEN is condition 1's second command above — the untouched committed file,
exit 0, `overlap=0`.)

**3. oracle disagreement** — a real `--oracle-results` fixture with one
`verdict: "disagree"` record against a real unit id from the live inventory:
```
$ python3 scripts/box_ledger.py --check --box THE-BOX.md --oracle-results /tmp/.../oracle-results-red.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: advanced_class_guide:class_feature:add_dare
exit=1
$ python3 scripts/box_ledger.py --check --box THE-BOX.md --oracle-results /tmp/.../oracle-results-green.json   # same record, verdict corrected to "agree"
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
exit=0
```

**4. an `unverifiable` unit dispositioned `done`** — temp copy of `THE-BOX.md`
with the real `unknown` group's `disposition` changed from `"unverifiable"`
to `"done"` (its `"unverifiable": true` flag left untouched — the real SD-32
over-claim, reproduced and caught):
```
$ python3 scripts/box_ledger.py --check --box /tmp/.../THE-BOX-cond4.md
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=4224 stale=False
UNVERIFIABLE_DISPOSITIONED_DONE: group 'unknown': advanced_class_guide:class_feature:abjuration_opposition_savant_school, ... (+4214 more)
exit=1
$ python3 scripts/box_ledger.py --check --box docs/release/SD-33-computed-value-verification/THE-BOX.md
exit=0    # committed file's 'unknown' group still says "unverifiable", not "done"
```

**5. `derived_at` not an ancestor of `HEAD`** — temp copy with the front
matter's real `derived_at: 2dcf2aebebcb662ca7d280b145cd7cc67ebd469b` replaced
with a fabricated, non-existent SHA:
```
$ python3 scripts/box_ledger.py --check --box /tmp/.../THE-BOX-cond5.md
STALE: derived_at=0000000000000000000000000000000000dead is NOT an ancestor of HEAD (fatal: Not a valid object name 0000000000000000000000000000000000dead)
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=True
exit=1
$ python3 scripts/box_ledger.py --check --box docs/release/SD-33-computed-value-verification/THE-BOX.md
exit=0    # committed derived_at is a real ancestor of current HEAD
```

**Preserved TDD RED (before implementation existed):** `python3 -m unittest
scripts.tests.test_box_ledger` failed with 12 `AttributeError`s (`module
'box_ledger' has no attribute 'check_staleness'` / `'unverifiable_done_violations'`
/ `'load_front_matter'`) and 3 `AssertionError`s (CLI mutation tests for
conditions 3–5 observing exit 0 / empty output where a failure was expected)
— all for the intended reason: the mechanisms did not exist yet. GREEN after
implementation: 25/25 passing.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Unit test suite (this file) | 25 passed, 0 failed, 0 skipped | `scripts/tests/test_box_ledger.py`'s own case count | `python3 -m unittest scripts.tests.test_box_ledger -v` |
| Tests added this cycle | 16 | 25 total (9 carried from `AT-33-E1-001`) | diff of test file, `git diff --stat -- scripts/tests/test_box_ledger.py` |
| `box_ledger.py --check` against committed `THE-BOX.md` (all 5 conditions) | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False` | full 49,438-unit inventory, 9-group ledger | `python3 scripts/box_ledger.py --check` |
| `unknown` (unverifiable) group population, used in mutation proofs 1 and 4 | 4,224 | whole inventory (49,438) | `jq '[.units[] | select(.status=="unknown")] | length' docs/work-inventory.json` |
| Live inventory population, cross-checked unchanged this cycle | 49,438 | whole inventory | `jq '.units | length' docs/work-inventory.json` |
| RED→GREEN mutation proofs delivered | 5 of 5 (one per condition) | the criterion's own 5-condition bar | transcript above; each command re-runnable verbatim except for the `/tmp/...` mutated fixture paths, which are ephemeral by design (mutations are never committed) |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle extends the instrument; it moves no inventory unit's status.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

No unit's status changed. `THE-BOX.md`'s group *counts* are unchanged
(still summing to 49,438); the only content change is the new
`"unverifiable"` field on each group, which is ledger metadata, not a unit
disposition.

## Notes

- **Design choice: ledger-level `"unverifiable"` flag, not a per-unit field.**
  `docs/work-inventory.json` units carry only `status` (no separate
  "disposition" field distinct from the group they land in), and `uncovered==0
  overlap==0` already guarantees every unit belongs to exactly one group. So a
  unit's disposition *is* its group's disposition, definitionally — checking
  at the group level is equivalent to per-unit checking and is the simplest
  mechanism that is still real (non-stub): flipping either field on a real
  group produces a real, observed failure (proof 4 above), and the flag is
  data the document declares, not something the tool infers by guessing at
  status-name semantics.
- **Oracle-disagreement check is wired, not deferred.** Per `workflow-instruction.md
  §12` row 8's standing lesson ("a lesson without a mechanism is a quote") and
  the module docstring's own prior promise ("it activates automatically once
  it does"), this cycle did not leave a TODO for Epic 2 to come back and wire
  the check — the check exists now, reads a real (harness-shaped) file format,
  and is proven to fail on a real fixture (proof 3). It only has nothing to
  examine yet because no oracle output exists yet, which is stated visibly
  (`INFO:` line), never swallowed silently.
- **`THE-BOX.md`'s `derived_at` front matter was not touched.** It already
  named `2dcf2aebebcb662ca7d280b145cd7cc67ebd469b` (set by `AT-33-E1-001`),
  which remains a real ancestor of this cycle's `HEAD` (`stale=False`,
  confirmed by execution) — no re-derivation of the group counts happened
  this cycle, so no new `derived_at` stamp was needed.
- **Test-fixture change, backward compatible.** `test_box_ledger.py`'s
  `_box_doc()` helper now always emits YAML front matter with a `derived_at`
  defaulting to the real current `git rev-parse HEAD` (trivially its own
  ancestor), so the 9 pre-existing tests from `AT-33-E1-001` keep their
  original pass/fail behavior unchanged under the new mandatory staleness
  gate — verified by re-running them (all `ok`) both before and after this
  cycle's implementation landed.
- **Test scoping.** Ran `python3 -m unittest scripts.tests.test_box_ledger -v`
  (25/25 green) — the only suite this criterion's file-touch set requires.
  Did **not** run `scripts/verify.sh` (that stage is `AT-33-E1-004`'s
  deliverable, not yet wired), the Rust workspace (`cargo test` — untouched by
  this cycle, no Rust files changed), or `apps/desktop/src-tauri` (a separate
  cargo workspace, untouched, out of scope for a Python-only change).
- Left `docs/retro/events/sd31-transcribe.jsonl`'s pre-existing dirty state
  and the untracked `artifacts/sd-33-dispatch.workflow.js` untouched — neither
  belongs to this cycle's tree per `workflow-instruction.md` §5's "one writer
  per tree" rule; confirmed via `git status --porcelain` before and after.

## Next-cycle plan

`AT-33-E1-003` enumerates the probe surface by execution
(`artifacts/epic-1-instruments/probe-surface-census.json`), stating for every
corpus kind whether a probe exists that can verify a computed magnitude.
`AT-33-E1-004` wires the denominator gate into `scripts/verify.sh` as a real
stage. Once Epic 2 lands `oracle-results.json`, condition 3's check activates
against real data with no further `box_ledger.py` change required — the next
cycle to touch that path should confirm this by running `--check` again and
reporting whatever `oracle_disagreement` count comes back, not by re-wiring
the check.
