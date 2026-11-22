# Cycle AT-33-E1-004-scope-widening — Epic 1 Instruments / AT-33-E1-004

- **Why this cycle exists:** `AT-33-E6-001` attempt 3's final-acceptance scan recorded an
  `instrument-correction`: a first probe at the bundle root was **never scanned** —
  `files_checked` stayed at 23/24 (`artifacts/**/*_cycle_receipt.md` + `progress.md` only). The
  gate's own module docstring already anticipated this: *"A later bundle extends `DEFAULT_GLOBS`
  ... for its own receipts."* This cycle is that extension: SD-33's headline package documents
  (`README.md`, `decisions.md`, `epic-breakdown.md`, `release-notes.md`, `scope-draft.md`,
  `kanban.md`, `THE-BOX.md`) join the scanned scope, and every violation the widening surfaces is
  fixed, not narrowed back around.
- **Commit SHA:** recorded by the commit that lands this receipt on `tranche/13` (rebase base at
  dispatch time: `95098aeb37`, merge-base with `origin/develop`: `f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba`)
- **Files touched:**
  - `scripts/denominator_gate.py` — `DEFAULT_GLOBS` widened (7 new literal paths); module
    docstring's "Scope" section rewritten to describe the widened scope and why (no matcher logic
    changed).
  - `scripts/tests/test_denominator_gate.py` — 2 new tests pinning the widened scope
    (`TestDefaultGlobsCoverHeadlinePackageDocs`).
  - `docs/release/SD-33-computed-value-verification/epic-breakdown.md` — 3 lines fixed (prose,
    denominators added/heading reworded).
  - `docs/release/SD-33-computed-value-verification/scope-draft.md` — 2 lines fixed (prose,
    denominators added).
  - `docs/release/SD-33-computed-value-verification/progress.md` (this cycle's entry, prepended).
  - `docs/release/SD-33-computed-value-verification/kanban.md` (row 4 Notes — pointer appended,
    row stays `complete`).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/AT-33-E1-004-scope-widening_cycle_receipt.md`
    (this file).
  - `docs/retro/events/sd33-r3-gate-scope.jsonl` (new, one `correction` event).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba...HEAD -- docs/release/SD-33-computed-value-verification/epic-breakdown.md docs/release/SD-33-computed-value-verification/scope-draft.md scripts/denominator_gate.py scripts/tests/test_denominator_gate.py | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no match)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**

  > ### AT-33-E1-004 — the denominator gate is a real `scripts/verify.sh` stage
  >
  > `scripts/verify.sh --only denominator-gate` runs, and **fails** on a percentage stated
  > without its denominator in the same construct.
  >
  > **Evidence:** RED→GREEN mutation proof — a deliberately-malformed receipt fails the stage; the
  > corrected form passes. Wired into `verify.sh`'s stage list, not a standalone script (the gap
  > `SD-31-.../forward-scope-register.md` C1.8 left open for `v06_corpus_trap_report`).

  This cycle does not re-open that base criterion's original evidence obligation (already met,
  `AT-33-E1-004_cycle_receipt.md` + `AT-33-E1-004-remediation_cycle_receipt.md`); it closes the
  scan-scope gap `AT-33-E6-001` attempt 3 found in *how far* the passing stage actually looks.

## Scope widened — the extension mechanism the docstring named, no second mechanism invented

`scripts/denominator_gate.py`'s module docstring, before this cycle, already named the intended
path: *"A later bundle extends `DEFAULT_GLOBS` (or passes its own paths / sets
`DENOMINATOR_GATE_PATHS`, the env var `scripts/verify.sh`'s stage reads) for its own receipts --
the same per-bundle-hardcoded-path shape `supersession-gate` already uses for SD-31's register."*
This cycle extends `DEFAULT_GLOBS` directly (the first-named option), not `DENOMINATOR_GATE_PATHS`
(an override, not an extension — it would *replace* the receipt+progress.md scope rather than add
to it) and not a new env var or a second glob-resolution code path.

**Seven root-level literal paths added**, matching the operator-named set exactly (not the
package's full document list — `technical-design.md`, `risks-and-open-questions.md`,
`acceptance-and-verification.md`, `content-unit-inventory.md`, `forward-scope-register.md`,
`technical-requirements.md`, and `receipts.md` stay out of scope; `receipts.md` was already
explicitly named out-of-scope in the pre-existing docstring for an unrelated reason — it is the
Epic 6 closure-pipeline's YAML log, not a place cycle figures are reported):

```
README.md
decisions.md
epic-breakdown.md
release-notes.md
scope-draft.md
kanban.md
THE-BOX.md
```

`files_checked` moves **24 → 31** (`python3 scripts/denominator_gate.py --check`, before/after
this cycle's own edit — see Figures table).

## New violations surfaced by the widening — 5, all real (prose), zero matcher false positives

Ran `python3 scripts/denominator_gate.py --check` immediately after widening `DEFAULT_GLOBS`,
before touching any prose:

```
$ python3 scripts/denominator_gate.py --check
VIOLATION .../epic-breakdown.md:87: **41% coverage is a symptom.** A committed root-cause statement explains *why* 6,854 units were never run — per family, since the gap is uneven (F1 28%, F8 21%, F2 64%).
VIOLATION .../epic-breakdown.md:97: ### AT-33-E3-003 — every remaining family closes to 100%
VIOLATION .../epic-breakdown.md:103: ### AT-33-E3-004 — the corpus-wide run reports 100% with its denominator
VIOLATION .../scope-draft.md:30: 2. **Engine coverage closed to 100%** of the formula-bearing population: the 6,854 units never run through an engine, run.
VIOLATION .../scope-draft.md:67: | 3 | **Engine-coverage closure** — the 6,854 units, 41% → 100% | Epic 1 | Epic 2, Epic 4 |
files_checked=31
violations=5
```

**Per-violation disposition** — for each, the question asked was: does this construct genuinely
state its own denominator somewhere on the line (a matcher bug), or does it not (a prose bug)?

| # | File:line | Verdict | Why | Fix |
|---|---|---|---|---|
| 1 | `epic-breakdown.md:87` | **real-violation** | `41%` (corpus-wide) and the three per-family rates (`F1 28%`, `F8 21%`, `F2 64%`) carry no denominator anywhere on the line — `6,854` (the *complement*, not the denominator: `11,652 − 4,798 = 6,854`) is the only number present, and using it as a stand-in denominator would itself be a wrong-number bug. Real denominators (`11,652`; `6,308`/`196`/`2,337` per family, from `README.md §4` row E and `coverage-gap-rootcause.md`'s per-family table) were added inline. | prose |
| 2 | `epic-breakdown.md:97` | **real-violation** | Heading `... closes to 100%` — no denominator, and none *can* be stated as a single number (F2–F9 are 8 different families with 8 different true populations; the per-family evidence lives in the criterion's own `**Evidence:**` line below, not this one). Reworded to drop the bare percent rather than fabricate a fake single denominator. | prose |
| 3 | `epic-breakdown.md:103` | **real-violation** | Heading `... reports 100% with its denominator` — the actual denominator (`11,652`) sits one line below, outside the gate's same-line construct (`workflow-instruction.md §6` step 2's own line-addressed granularity). Reworded to state the number in the heading itself. | prose |
| 4 | `scope-draft.md:30` | **real-violation** | `100%` and `6,854` are both present on the line, genuinely related (100% *of* the 6,854-unit population), but separated by `of the formula-bearing population: the` — 39 characters, past `DENOMINATOR_RE`'s documented 24-char tolerance. Checked whether the 24-char window itself is the bug: no — it is deliberately tight to avoid a real false-negative (an unrelated `of <N>` elsewhere on a long line being mistaken for the percent's actual denominator); widening it generically would weaken the gate for every future construct, not just this one. Reworded to bring the number adjacent to `of` instead. | **prose** (matcher confirmed correct; see Notes) |
| 5 | `scope-draft.md:67` | **real-violation** | Table cell `the 6,854 units, 41% → 100%` — no `of`/`out of`/fraction/`denominator` marker at all, in any position. `6,854` here is also the complement, not either percentage's denominator. Reworded with both percentages' real denominators (`4,798`/`11,652`) inline. | prose |

**Zero matcher false positives.** Every one of the 5 was a genuine missing (or misplaced) real
denominator, corrected with the real number, re-derived from the same sources the rest of this
bundle already cites (`README.md §4`, `coverage-gap-rootcause.md`) — not invented, not the nearby
complement number substituted for the true denominator.

Re-ran after the prose fixes:

```
$ python3 scripts/denominator_gate.py --check
files_checked=31
violations=0
```

**All 5 closed by fixing the prose. The matcher (`DEFAULT_GLOBS` aside) was not touched** — no
regex in `PERCENT_RE`, `DENOMINATOR_RE`, or `FALSE_100_IDIOM_RE` changed. This is the discipline
the dispatch brief named explicitly: widen the scope, then fix what it finds — never relax the
matcher until the widened scope goes quiet.

## Detection re-proven live, inside the real widened default scope

Not a synthetic path, not `DENOMINATOR_GATE_PATHS`-pointed — the real `README.md`, scanned by the
real (no-args) `DEFAULT_GLOBS` invocation, exactly as `scripts/verify.sh --only denominator-gate`
calls it.

**RED — a bare percentage, and a bare hundred-percent token, both with no denominator on their own
line:**

```
$ cat >> docs/release/SD-33-computed-value-verification/README.md <<'EOF'

<!-- AT-33-E1-004 wave-3 live probe -- REMOVED before commit -->
PROBE_LINE_A: recognition sits at 88.4% this cycle.
PROBE_LINE_B: coverage is fully closed at 100%.
EOF
$ python3 scripts/denominator_gate.py --check
VIOLATION .../README.md:138: PROBE_LINE_A: recognition sits at 88.4% this cycle.
VIOLATION .../README.md:139: PROBE_LINE_B: coverage is fully closed at 100%.
files_checked=31
violations=2
$ echo $?
1
```

**RED confirmed** — both the plain-rate and the bare-hundred-percent shape fail, live, inside the
real widened scope (`README.md` is one of the 7 new files; before this cycle's widening it would
not have been checked at all).

**GREEN — corrected form, same two lines, denominator added inline:**

```
$ sed -i \
  -e 's/PROBE_LINE_A: recognition sits at 88.4% this cycle./PROBE_LINE_A: recognition sits at 88.4%, true of 200 of the 226 units sampled./' \
  -e 's/PROBE_LINE_B: coverage is fully closed at 100%./PROBE_LINE_B: coverage is fully closed at 100% (226 of 226)./' \
  docs/release/SD-33-computed-value-verification/README.md
$ python3 scripts/denominator_gate.py --check
files_checked=31
violations=0
$ echo $?
0
```

**GREEN confirmed.**

**Probe lines fully removed, baseline re-confirmed clean:**

```
$ git diff docs/release/SD-33-computed-value-verification/README.md
$ # (empty — no diff, probe text left no residue)
$ python3 scripts/denominator_gate.py --check
files_checked=31
violations=0
```

**The stage itself, not just the underlying script:**

```
$ bash scripts/verify.sh --only denominator-gate
==> denominator-gate — python3 scripts/denominator_gate.py --check
    PASS  denominator-gate  (files_checked=31 violations=0)
RESULT: PASS
$ echo $?
0
```

## Regression pinned in tests

Two new tests in `scripts/tests/test_denominator_gate.py`
(`TestDefaultGlobsCoverHeadlinePackageDocs`): one asserts all 7 headline docs' paths are present
in `DEFAULT_GLOBS`; the other asserts `expand_paths(DEFAULT_GLOBS)` actually resolves each of them
to a real file (not just present in the pattern list, but actually read by `run_check`).

**RED confirmed against the pre-change module** (not narrated — executed):

```
$ git show f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba:scripts/denominator_gate.py > /tmp/dg-red-check/denominator_gate.py
$ python3 -c "
import sys, os
sys.path.insert(0, '/tmp/dg-red-check')
import denominator_gate as dg_old
docs = ('README.md','decisions.md','epic-breakdown.md','release-notes.md','scope-draft.md','kanban.md','THE-BOX.md')
expected = {os.path.join(dg_old.BUNDLE_DIR, n) for n in docs}
missing = expected - set(dg_old.DEFAULT_GLOBS)
print('missing from OLD DEFAULT_GLOBS:', len(missing))
"
missing from OLD DEFAULT_GLOBS: 7
```

All 7 missing, for the intended reason (the old `DEFAULT_GLOBS` had exactly 2 entries, neither
matching a root-level bundle doc).

**GREEN — current module, full suite:**

```
$ python3 -m unittest scripts.tests.test_denominator_gate -v
...
test_default_globs_include_every_headline_package_doc ... ok
test_headline_docs_are_real_files_the_gate_actually_reads ... ok
...
Ran 26 tests in 0.062s

OK
```

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| `DEFAULT_GLOBS` entries, before this cycle | 2 (a glob + a literal path) | — | `git show f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba:scripts/denominator_gate.py \| grep -A3 'DEFAULT_GLOBS = \['` |
| `DEFAULT_GLOBS` entries, after this cycle | 9 (the original 2 + 7 new literal root docs) | — | `python3 -c "import sys; sys.path.insert(0,'scripts'); import denominator_gate as dg; print(len(dg.DEFAULT_GLOBS))"` |
| `files_checked`, before widening | 24 | of `DEFAULT_GLOBS`'s pre-cycle resolution | `python3 scripts/denominator_gate.py --check` (run before `DEFAULT_GLOBS` edit) |
| `files_checked`, after widening | 31 | of `DEFAULT_GLOBS`'s post-cycle resolution (24 receipts+progress.md + 7 new headline docs) | `python3 scripts/denominator_gate.py --check` |
| New violations surfaced by the widening | 5 | of the 7 new files scanned | `python3 scripts/denominator_gate.py --check` (run immediately after the `DEFAULT_GLOBS` edit, before any prose fix) |
| Violations after prose fixes | 0 | of 31 files checked | `python3 scripts/denominator_gate.py --check` |
| `scripts/verify.sh --only denominator-gate` | PASS | — | `bash scripts/verify.sh --only denominator-gate` → `RESULT: PASS`, exit 0 |
| `test_denominator_gate.py` suite | 26 passed, 0 failed | file's own case count (24 pre-existing + 2 new) | `python3 -m unittest scripts.tests.test_denominator_gate -v` |
| Combined regression (`test_denominator_gate` + `test_box_ledger` + `test_probe_surface_census`) | 62 passed, 0 failed | all three files' combined case count | `python3 -m unittest scripts.tests.test_denominator_gate scripts.tests.test_box_ledger scripts.tests.test_probe_surface_census -v` |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle repairs an instrument's scan scope; it moves no inventory unit's
  status or disposition.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 6 — the scan-scope gap itself (1: `DEFAULT_GLOBS` widened) plus the 5
  denominator-less prose lines the widening made visible and this cycle fixed. Recorded as one
  `correction` retro event (`docs/retro/events/sd33-r3-gate-scope.jsonl`,
  `--verified-by python3 scripts/denominator_gate.py --check`), not narrated only.

## `scripts/verify.sh` in full — reported, not fixed

Launched at cycle start (`bash scripts/verify.sh`, `ALL_STAGES`, 40 stages) and left running the
whole turn in a `nohup`-detached background process (own scratch
`CARGO_TARGET_DIR=/tmp/cargo-sd33-sd33-r3-gate-scope` per §2.1, ruling out this cycle's own cargo
activity as a cause of anything slow).

**Result: did not finish inside this turn.** Two independent observations, ~4 minutes apart, both
show the run stalled at the **same** stage, `site-dashboard-check` (`scripts/publish-site-dashboard.sh
--check`) — 8th of 40 `ALL_STAGES`:

```
==> preflight-disk        PASS
==> preflight-oracle      PASS
==> oracle-pin-selftest   PASS  (11 passed, 0 failed)
==> producer-selftest     PASS  (19 cases passed)
==> pi-redaction-selftest PASS  (49 cases passed)
==> provenance-selftest   PASS  (32 cases passed)
==> site-dashboard-selftest PASS (6 passed, 0 failed)
==> site-dashboard-check   <-- no PASS/FAIL line, still running
```

This exactly reproduces `AT-33-E1-004-remediation`'s own prior full-sweep observation (same stage,
same stall point) — a standing, not new, hazard on this shared checkout, consistent with that
receipt's own hypothesis (another concurrent lane's I/O contention or the dashboard producer's
runtime on the full corpus; three sibling lanes are actively writing to
`artifacts/epic-5-reverification/` this same wave per this cycle's dispatch brief). **7 of 40
stages independently confirmed PASS; `site-dashboard-check` and the 32 stages after it (including
`denominator-gate` itself in the full-sweep ordering, `root-full`, `desktop`, `clippy`) have an
unknown status from this specific full-sweep run** — not red, not green, not measured by it.
`denominator-gate`'s own status is independently known GREEN from the `--only denominator-gate`
invocations above, which is the criterion this cycle owns.

**Not investigated further** — `publish-site-dashboard.sh` is unrelated to
`denominator_gate.py`/`epic-breakdown.md`/`scope-draft.md`, the only files this cycle's scope
covers, and diagnosing a shared-checkout stall is out of this criterion's write scope.

## Test scoping

- **Ran:** `python3 -m unittest scripts.tests.test_denominator_gate -v` (26/26, includes 2 new
  scope-pin cases); `python3 -m unittest scripts.tests.test_denominator_gate
  scripts.tests.test_box_ledger scripts.tests.test_probe_surface_census -v` (62/62, regression,
  neither `box_ledger.py` nor `probe_surface_census.py` touched this cycle); `python3
  scripts/denominator_gate.py --check` (repeatedly, live, at every step above); `bash
  scripts/verify.sh --only denominator-gate` (multiple times — pre-widening baseline, post-widening
  RED, post-fix GREEN, live probe RED/GREEN pair, final baseline); `bash scripts/verify.sh` in full,
  once, per this cycle's own finish line (result reported above, not fixed).
- **Did NOT run:** the Rust workspace (`cargo build`/`cargo test`) or `apps/desktop/src-tauri`
  (a separate cargo workspace per `AGENTS.md`) as standalone steps — no `.rs` file was touched this
  cycle; the full `verify.sh` run above is the only path that would exercise
  `root-lib`/`root-full`/`reach`/`class-dump`/`desktop`, and it is a report-only obligation this
  cycle does not own, not a gate this cycle's file-touch set requires green.

## Notes

- **Judgment calls, each stated with its reasoning above:** (1) widen `DEFAULT_GLOBS`, not
  `DENOMINATOR_GATE_PATHS` — the docstring names both, but only the former is additive; the latter
  is an override that would have *shrunk* the scanned set back to the 7 new files alone. (2) the
  exact 7-file set matches the operator-named list verbatim, not the bundle's full document
  inventory — widening further is a task of its own the module docstring's rewritten "Scope"
  section states explicitly, not a silent omission. (3) `scope-draft.md:30`'s violation was
  checked as a possible matcher bug (the 24-char window) before being dispositioned prose — see
  disposition table row 4 — because the brief explicitly asked to distinguish the two, not assume
  prose is always the answer.
- **Coordination:** three sibling lanes are writing to `artifacts/epic-5-reverification/` this same
  wave. This cycle's file-touch set (`scripts/denominator_gate.py`, its tests,
  `epic-breakdown.md`, `scope-draft.md`, this receipt, `progress.md`/`kanban.md` in the shared-file
  append-only pattern) is disjoint from theirs by construction. Scanned and fixed against what was
  committed at this cycle's rebase point (`f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba` merge-base,
  `95098aeb37` branch tip at dispatch); any further prose those lanes land in the newly-widened
  scope after this commit is the finalize cycle's responsibility to keep green, per this cycle's
  own dispatch brief.
- **Why the F1/F8/F2 numbers in disposition #1 are trustworthy, not invented:** cross-checked
  against two independent sources already committed in this bundle —
  `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md`'s per-family table, quoted verbatim:
  ```
  F1 | old-run 1,790 | true 6,308 | old-run coverage 28.4%
  F2 | old-run 1,490 | true 2,337 | old-run coverage 63.8%
  F8 | old-run    41 | true   196 | old-run coverage 20.9%
  ```
  and `progress.md:885`'s independent restatement of the F1 figure ("F1 (largest family): true =
  run = 6,308 of 6,308"). Both agree; neither is this cycle's own claim.

## Next-cycle plan

`AT-33-E1-004`'s scan-scope gap is closed: `DEFAULT_GLOBS` now covers the 7 headline package docs,
every violation the widening surfaced is fixed with a real denominator (not a relaxed matcher),
and detection is re-proven live inside the real widened scope. Kanban row 4 stays `complete`; the
scope-widening pointer is appended, not a story. `verify.sh` in full remains unmeasured past
`site-dashboard-check` in this turn's observation window — the next cycle that needs a full-sweep
verdict should re-run it fresh (the stall point has now been independently reproduced twice at the
same stage, which is itself worth a dedicated diagnosis cycle if it recurs a third time, per
`AGENTS.md` rule 8's "a key firing more than a handful of times is a missing mechanism, not bad
luck").
