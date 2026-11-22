# Cycle AT-34-E1-004-R — Epic 1 Completion Atlas / AT-34-E1-004 (re-verification at HEAD)

This cycle was dispatched against `AT-34-E1-004` after the criterion had already landed
(original cycle receipt: `AT-34-E1-004_cycle_receipt.md`, commit `4d69afd6e4`, kanban row
already `complete`). Between that commit and this cycle's dispatch, Epic 3's per-unit fixes to
`src/bin/v06_work_inventory.rs` (nine-mechanism `AT-34-E3-001` work, `decisions.md §14`) added
and edited a large amount of code above the promotion ladder's original location, and multiple
`docs/work-inventory.json` regeneration cycles moved units out of `engine-does-not-hold`.
Carrying the original receipt's numbers or line citation forward would violate `decisions.md
§12` L2; this cycle re-derives the criterion from scratch at HEAD.

- **Commit SHA:** see push output below (this receipt's own landing commit).
- **Files touched:**
  - `scripts/shape_engine_boundary.py` (promotion-ladder line citation `9592-9595` -> `10854-10857`;
    docstring/markdown-template counts updated)
  - `scripts/tests/test_shape_engine_boundary.py` (same line-number and count updates)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/shape-engine-boundary.md` (regenerated)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-004_re-verification_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended)
  - `docs/release/SD-34-book-completion/kanban.md` (row re-confirmed with re-verification receipt link, status unchanged — already `complete`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-34-E1-004 — the shape-engine boundary is stated as a fact, not an assumption
  >
  > A committed statement, proven by execution, of what a shape engine does and where its
  > output stops — so no future bundle re-learns it.
  >
  > **Evidence:** `artifacts/epic-1-atlas/shape-engine-boundary.md`, carrying the count of
  > magnitude-bearing units (**26,396**), how many of those the engine still does not hold
  > (**13,119 of 26,396**), and the four-condition promotion ladder quoted from
  > `src/bin/v06_work_inventory.rs` with its line number re-verified at HEAD.

  The `13,119 of 26,396` figure is the criterion's **launch-time** measurement, quoted verbatim
  from `epic-breakdown.md` as written before Epic 3 closed real units against it. It is not a
  live invariant (bucket-shrink is the whole point of Epic 3's work) — the load-bearing part of
  the criterion is that the artifact's counts and the promotion-ladder line citation are
  *re-derived at HEAD*, never carried forward stale.

## RED — confirmed the artifact would have been stale for the intended reason

```
$ python3 scripts/shape_engine_boundary.py --check
STALE_CITATION: promotion-ladder citation no longer resolves at HEAD:
  src/bin/v06_work_inventory.rs:9592: expected to contain 'if has_real_description', found 'return Verdict {'
  src/bin/v06_work_inventory.rs:9593: expected to contain '&& is_display_wiring_class_for_promotion(wc_class)', found 'status: "grounded",'
  src/bin/v06_work_inventory.rs:9594: expected to contain '&& !universal_sheet_modifier', found 'evidence: "equipment_universal_sheet_modifier_pending_compute".to_string(),'
  src/bin/v06_work_inventory.rs:9595: expected to contain '&& facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)', found 'reason: None,'
(exit 1, no artifact written -- fails closed as designed)
```

Confirmed this failure is for the **intended** reason (the exact fail-closed posture
`risks-and-open-questions.md §10` designed for): the code at those four lines is now unrelated
`Verdict` construction from a different match arm, not a broken file or missing function. The
real promotion-ladder block — found by grepping for the exact four-condition text, byte-for-byte
identical to what `technical-design.md §3` / `decisions.md §2a` quote — now lives at
`src/bin/v06_work_inventory.rs:10854-10857`:

```
$ sed -n '10854,10857p' src/bin/v06_work_inventory.rs
                if has_real_description
                    && is_display_wiring_class_for_promotion(wc_class)
                    && !universal_sheet_modifier
                    && facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)
```

Also re-derived the live counts and confirmed the second half of the drift — the population
count held, the not-held count shrank from real Epic-3 closure work, not measurement change:

```
$ python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(sum(1 for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0))"
26396
$ python3 -c "import json; d=json.load(open('docs/work-inventory.json')); m=[u for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0]; print(sum(1 for u in m if u.get('status') == 'engine-does-not-hold'))"
9475
```

Pre-fix unit tests failed for the intended reason too — one `StaleCitationError` from the
mutation-proof harness's own real-content check, and a genuine count mismatch:

```
$ python3 -m unittest scripts.tests.test_shape_engine_boundary -v
...
FAIL: test_live_counts_match_the_committed_fact -- AssertionError: 9475 != 13119
FAIL/ERROR: 4 tests failed on the stale 9592-9595 citation
Ran 12 tests -- FAILED (failures=4, errors=2)
```

## GREEN — fixed and re-verified

Updated `PROMOTION_LADDER_LINES`/`PROMOTION_LADDER_ANCHOR_LINE` in
`scripts/shape_engine_boundary.py` to `10854-10857`/`10857`, updated the docstring's example
output and the "roughly a third" narrative line (previously "half") to state both the launch-time
and current fractions with their command, and updated the test file's line-number and count
expectations to match.

```
$ python3 -m unittest scripts.tests.test_shape_engine_boundary -v
...
Ran 12 tests in 0.700s
OK
```

```
$ python3 scripts/shape_engine_boundary.py --check
magnitude_bearing=26396 not_held_by_engine=9475 citation_ok=True
```

```
$ python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'
files_checked=15
violations=0
```

## Re-verification result

```
$ python3 scripts/shape_engine_boundary.py --check
magnitude_bearing=26396 not_held_by_engine=9475 citation_ok=True
```

- **Figures + their re-derive commands:**
  - `magnitude_bearing=26396` — `python3 scripts/shape_engine_boundary.py --check` (denominator:
    26,396 of `docs/work-inventory.json`'s full 49,438-unit population; unchanged from the
    original AT-34-E1-004 cycle — the magnitude-bearing predicate does not depend on `status`)
  - `not_held_by_engine=9475` — same command (denominator: 9,475 of 26,396 magnitude-bearing
    units; **moved from 13,119 at the original cycle** — a real reclassification, verified by
    `git log --oneline 4d69afd6e4..HEAD -- docs/work-inventory.json` showing 25+ Epic-3
    regeneration commits between the two measurements)
  - Promotion-ladder citation, re-verified at HEAD by content:
    `src/bin/v06_work_inventory.rs:10854-10857` (moved from `9592-9595` — verified with
    `sed -n '10854,10857p' src/bin/v06_work_inventory.rs`, reproducing the exact four-condition
    block `technical-design.md §3` and `decisions.md §2a` quote)
  - `citation_ok=True` — same command; the fail-closed RED->GREEN mutation proof
    (`TestCitationFailsClosedForTheIntendedReason`, now keyed on line `10857`) still passes,
    12/12 tests green
- **Row-count command output:**
  ```
  $ python3 -c "
  content = open('docs/release/SD-34-book-completion/artifacts/epic-1-atlas/shape-engine-boundary.md').read()
  reqs = ['26396', '9475', '10857', 'has_real_description', 'is_display_wiring_class_for_promotion', 'universal_sheet_modifier', 'class_feature_pool_catalog_holds', 'python3 scripts/shape_engine_boundary.py --check']
  present = [r for r in reqs if r in content]
  missing = [r for r in reqs if r not in content]
  print(f'required_elements_present={len(present)} of {len(reqs)}; missing={missing}')
  "
  required_elements_present=8 of 8; missing=[]
  ```
- **Build scope verified:** `cargo test --locked --no-run` exit 0, run at this cycle's HEAD
  before landing (600 test executables built cleanly, 0 `error[` / `error:` lines in the log;
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-004`, `CARGO_INCREMENTAL=0`). No Rust source
  touched this cycle (only `scripts/*.py` and one generated `.md`), so `cargo test --locked
  --lib` is not separately re-run — the `--no-run` build already compiles the lib and every
  test binary. `apps/desktop/src-tauri` not touched, not run.
- **Sweep population:** N/A — this cycle adds no corpus records and regenerates none;
  `docs/work-inventory.json` is read-only this cycle.
- **Oracle pin:** N/A — no figure in this cycle came from the pinned PCGen corpus.
- **Status:** complete
- **Movement, four buckets:** reclassification — the `not_held_by_engine` count itself moved
  (13,119 -> 9,475) because Epic 3's per-unit work promoted 3,644 magnitude-bearing units past
  `engine-does-not-hold` since the original cycle (real closure work, attributed to Epic 3's own
  receipts, not to this cycle); this cycle's own contribution is instrument-correction — the
  stale line citation and stale counts in the AT-34-E1-004 artifact are re-derived to match HEAD,
  landing zero new production behavior.
- **Notes:**
  - The original cycle's citation-content check did exactly what `risks-and-open-questions.md
    §10` designed it to do: it failed closed (non-zero exit, no artifact overwritten with wrong
    numbers) the instant the underlying source moved, rather than silently reporting a stale
    line. That is this cycle's whole reason to exist — the mechanism worked.
  - `technical-design.md §3` and `decisions.md §2a` still print the original `:9595` citation and
    the `13,119 of 26,396` figure. Those are dated decision/design records outside Epic 1's
    declared file-touch set (`workflow-instruction.md §3` row 1) and are not corrected by this
    cycle — the committed, live-checked fact lives in `shape-engine-boundary.md`, which this
    cycle keeps accurate; the two narrative docs are historical record of what was true at
    authoring time, same posture `AT-34-E1-003`'s re-verification cycle took for
    `missing-engine-tables.json` vs. `decisions.md §7`.
  - No successor is told to trust this document's counts beyond this cycle's HEAD
    (`ea2b3396f2...` -> re-verified through this cycle's commit) — a further Epic-3/4 wave that
    keeps closing bucket-A/B/M/etc. units will move `not_held_by_engine` again, and the next
    reader should re-run `python3 scripts/shape_engine_boundary.py --check` rather than quote
    this receipt's number.
- **Next-cycle plan:** none required from this cycle — AT-34-E1-004 is closed and self-verifying.
  A later Epic-3/4/6 cycle that touches `src/bin/v06_work_inventory.rs` again should re-run
  `python3 scripts/shape_engine_boundary.py --check` before trusting `shape-engine-boundary.md`;
  the instrument fails closed and tells the reader exactly what moved if it has not.
