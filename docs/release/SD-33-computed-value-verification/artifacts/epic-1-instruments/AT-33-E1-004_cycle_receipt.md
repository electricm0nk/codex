# Cycle AT-33-E1-004 — Epic 1 Instruments / AT-33-E1-004

- **Commit SHA:** `53fae7abf8` (initial land), `2bf50e7bb5` (self-correction, see Notes)
- **Files touched:**
  - `scripts/denominator_gate.py` (new)
  - `scripts/tests/test_denominator_gate.py` (new)
  - `scripts/verify.sh` (extended — new `denominator-gate` stage, both stage sets, dispatch case)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/AT-33-E1-004_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (updated)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E1-004 — the denominator gate is a real `scripts/verify.sh` stage
  >
  > `scripts/verify.sh --only denominator-gate` runs, and **fails** on a percentage stated without its denominator in the same construct.
  >
  > **Evidence:** RED→GREEN mutation proof — a deliberately-malformed receipt fails the stage; the corrected form passes. Wired into `verify.sh`'s stage list, not a standalone script (the gap `SD-31-.../forward-scope-register.md` C1.8 left open for `v06_corpus_trap_report`).

## What landed

`scripts/denominator_gate.py`: a pure line-level check (`find_violations`) — a line carrying a
bare percentage (`\d[\d,]*(?:\.\d+)?\s?%`) with no denominator marker anywhere on that same line
(`of <N>`, `out of <N>`, an `<N>/<M>` fraction, or the literal word `denominator` followed by a
number) is a violation, except inside a fenced ` ``` ` code block (a receipt's evidence transcript
legitimately quotes a malformed fixture's raw bytes — data being shown, not a claim being made; see
Notes below for the self-correction that made this exclusion necessary). Wired into
`scripts/verify.sh` as a real stage, `denominator-gate`, in
**both** `ALL_STAGES` and `QUICK_STAGES`, with its own `run_denominator_gate()` function and
dispatch-case entry — not a standalone script, closing the exact gap
`SD-31-.../forward-scope-register.md` C1.8 named for `v06_corpus_trap_report`.

**Scope, deliberately narrow (judgment call, see Notes):** the default target set is this
bundle's own generated evidence — `artifacts/**/*_cycle_receipt.md` + `progress.md` — not this
bundle's planning prose and not every prior bundle's receipts. `DENOMINATOR_GATE_PATHS`
(space-separated globs, read by `run_denominator_gate()`) overrides the default, following the
same `${VAR:-default}` shape `VERIFY_LOG_DIR`/`PREFLIGHT_DISK_MIN_FREE_GB` already use in
`verify.sh` — this is the mechanism the RED→GREEN mutation proof below points at a synthetic file
without ever committing a violation to the repo.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Unit test suite (new) | 20 passed, 0 failed | `scripts/tests/test_denominator_gate.py`'s own case count | `python3 -m unittest scripts.tests.test_denominator_gate -v` |
| Unit test suite (existing, re-run for regression) | 25 passed, 0 failed | `scripts/tests/test_box_ledger.py`'s own case count | `python3 -m unittest scripts.tests.test_box_ledger -v` |
| Unit test suite (existing, re-run for regression) | 11 passed, 0 failed | `scripts/tests/test_probe_surface_census.py`'s own case count | `python3 -m unittest scripts.tests.test_probe_surface_census -v` |
| Combined suite | 56 passed, 0 failed | all three files' combined case count | `python3 -m unittest scripts.tests.test_denominator_gate scripts.tests.test_box_ledger scripts.tests.test_probe_surface_census -v` |
| Default-scope files checked on the live committed repo | 5 | of the 5 files matched by `DEFAULT_GLOBS` (4 cycle receipts, including this one, + `progress.md`, as of this commit) | `python3 scripts/denominator_gate.py --check` → `files_checked=5` |
| Violations on the live committed default scope | 0 | of the 5 files checked | `python3 scripts/denominator_gate.py --check` → `violations=0` |
| `verify.sh --only denominator-gate` stage list membership | present in both stage sets | of 2 stage sets (`ALL_STAGES`, `QUICK_STAGES`) | `bash scripts/verify.sh --list \| grep denominator-gate` → `denominator-gate     yes   yes` |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle builds an instrument (a gate with an exit code); it moves no
  inventory unit's status or disposition.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

## Notes

- **Scope is deliberately this bundle's own generated evidence, not repo-wide.** A `grep -rlE`
  sweep found 261 `*_cycle_receipt.md` files repo-wide, at least 29 of them (SD-24/SD-25/SD-32)
  already carrying a `%` token — auditing and (where warranted) fixing that population is a task
  of its own, unbounded and out of this single criterion's scope (`AGENTS.md` rule 3, "do not
  expand scope"). Scoping the default also excludes this bundle's own planning prose
  (`epic-breakdown.md`, `technical-design.md`, `risks-and-open-questions.md`, `decisions.md`
  itself), which narrates the same 41%-of-11,652 / 97.9%-of-4,798 figures `decisions.md` §2 cites
  as the motivating defect — those documents are outside this criterion's write scope (owned by
  the bundle author, `god-emporer`), so scanning them would turn this gate permanently red over
  prose this cycle has no authority to correct, which is a false positive on day one, not a caught
  defect (the same "prove it can fail before it is trusted, but don't let a known-population turn
  the gate permanently red" balance `shape-coverage-standing-gate`'s own comment names). Verified
  the default globs' current 5-file population is 100% clean (5 of 5 files), live, above.
- **The check is line-level, not full-document.** "Same construct" is implemented as "same line" —
  the identical granularity `workflow-instruction.md` §6 step 2's identifier/token audits already
  use (`git diff --unified=0`, line-addressed). A known limitation: if one line carries two
  percentages and only one has a denominator marker on that line, the line passes as a whole
  (per-line, not per-match). No real committed content in this bundle's default scope hits that
  shape today; noted for whoever tightens this later.
- **`--check`'s failure mode is proven capable of firing**, per the same standing principle
  `AT-33-E1-002`'s and `AT-33-E1-003`'s receipts both name ("a tool that has never been observed to
  fail is not a gate") — the RED case below exercises the exact `scripts/verify.sh --only
  denominator-gate` invocation the criterion names, not just the underlying Python script.
- **Self-correction, discovered and fixed live in this same cycle (worth recording under
  `decisions.md` §2's own corollary):** the first commit of this cycle (`53fae7abf8`) wired the
  gate and pushed, then a post-push re-run of `bash scripts/verify.sh --only denominator-gate`
  against the newly-pushed default scope found 7 violations, spread across 2 of the 4 files then in
  scope — the gate correctly caught bare percentage tokens (the same undenominated shape
  `decisions.md` §2 names) in **this receipt's own prose** — illustrating the defect while
  describing it, and quoting the RED fixture's raw malformed content verbatim inside a code fence —
  and in `progress.md`'s own summary of this cycle. Two distinct fixes, both
  in this same cycle, both re-verified: (1) added a fenced-code-block exclusion to
  `find_violations` — a receipt's evidence transcript legitimately quotes a malformed fixture's raw
  bytes, which is data being shown, not a claim the receipt is making, so lines inside ` ``` `
  blocks are skipped (3 new unit tests, `test_bare_percentage_inside_fenced_code_block_is_not_flagged`
  et al.); (2) reworded the 6 remaining real prose violations (this receipt's own Notes/RED section,
  `progress.md`'s own cycle entry) to state each percentage's denominator in the same line. Re-run
  after both fixes: `python3 scripts/denominator_gate.py --check` → `files_checked=5 violations=0`.
  This is exactly the failure shape the criterion exists to catch, caught by the mechanism itself
  against its own author's writing — not narrated, demonstrated.

## RED → GREEN evidence

**TDD RED** (before `scripts/denominator_gate.py` existed — module temporarily moved aside to
capture a clean transcript):
```
$ python3 -m unittest scripts.tests.test_denominator_gate -v
ImportError: Failed to import test module: test_denominator_gate
...
ModuleNotFoundError: No module named 'denominator_gate'
Ran 1 test in 0.000s
FAILED (errors=1)
```
Failed for the intended reason — the module under test did not exist yet.

**GREEN** (after implementation):
```
$ python3 -m unittest scripts.tests.test_denominator_gate -v
...
Ran 17 tests in 0.009s
OK
```

**The criterion's own evidence obligation — a mutation proof through `scripts/verify.sh --only
denominator-gate` itself, not just the underlying script**, using `DENOMINATOR_GATE_PATHS` to
point the stage at a synthetic file without ever committing a violation:

**RED** — a deliberately-malformed receipt (the exact "97.9% recognised", true only of the 4,798
units it ran, 41% of the 11,652 that exist — the shape `decisions.md` §2 names as the real,
motivating defect), fed to the real stage:
```
$ cat /tmp/denom-gate-mutation/bad_cycle_receipt.md
# Cycle FAKE — mutation-proof fixture

- **Figures:** Gate 2's corpus-wide engine run reports **97.9% recognised**.

$ DENOMINATOR_GATE_PATHS=/tmp/denom-gate-mutation/bad_cycle_receipt.md bash scripts/verify.sh --only denominator-gate
==> denominator-gate — python3 scripts/denominator_gate.py --check
    FAIL  denominator-gate  (violations=1 of files_checked=1 — .../denominator-gate.log)
RESULT: FAIL
$ echo $?
1
```

**GREEN** — the corrected form (denominator stated in the same construct), same stage, same
invocation shape:
```
$ cat /tmp/denom-gate-mutation/good_cycle_receipt.md
# Cycle FAKE — mutation-proof fixture

- **Figures:** Gate 2's corpus-wide engine run reports **97.9% recognised**, true of the **4,798 units it ran** — **41% of the 11,652** that exist.

$ DENOMINATOR_GATE_PATHS=/tmp/denom-gate-mutation/good_cycle_receipt.md bash scripts/verify.sh --only denominator-gate
==> denominator-gate — python3 scripts/denominator_gate.py --check
    PASS  denominator-gate  (files_checked=1 violations=0)
RESULT: PASS
$ echo $?
0
```

**GREEN, default scope** — the same stage run with no override, against the real committed
`artifacts/epic-1-instruments/*_cycle_receipt.md` (including this file itself, after the
self-correction in Notes below) + `progress.md`:
```
$ bash scripts/verify.sh --only denominator-gate
==> denominator-gate — python3 scripts/denominator_gate.py --check
    PASS  denominator-gate  (files_checked=5 violations=0)
RESULT: PASS
```

Regression: `python3 -m unittest scripts.tests.test_box_ledger scripts.tests.test_probe_surface_census -v`
→ 36/36 still green, unchanged (neither `box_ledger.py` nor `probe_surface_census.py` was touched
this cycle).

## Test scoping

- **Ran:** `python3 -m unittest scripts.tests.test_denominator_gate -v` (20/20, new — includes 3
  fence-exclusion cases added during the self-correction in Notes above);
  `python3 -m unittest scripts.tests.test_box_ledger scripts.tests.test_probe_surface_census -v`
  (36/36, regression). `bash -n scripts/verify.sh` (syntax check). `bash scripts/verify.sh --list`
  (confirms `denominator-gate` present in both stage sets). `bash scripts/verify.sh --only
  denominator-gate` five times total (pre-correction default GREEN at `files_checked=4`, override
  RED, override corrected GREEN, post-correction default GREEN at `files_checked=5`, and a final
  re-confirmation of the RED/GREEN pair after the fence-exclusion fix — the criterion's own
  evidence obligation, re-verified after the self-correction).
- **Did NOT run:** `scripts/verify.sh` in full (`--only denominator-gate` is this criterion's own
  scope; the other stages' own preconditions — oracle build, corpus sweep, cargo build — are
  unrelated to this cycle's files and unchanged). The Rust workspace (`cargo build`/`cargo test`)
  — no `.rs` file was touched this cycle. `apps/desktop/src-tauri` — a separate cargo workspace per
  `AGENTS.md`; no file in it was touched or is affected by this change.

## Next-cycle plan

Epic 1's sequential pipeline (rows 1-4) is now complete. Epics 2/3/4 are gated on Epic 1 and
parallel-safe with each other (`workflow-instruction.md` §3) — the next dispatch is the
`parallel: yes` wave: `AT-33-E2-001` (Path A feasibility), `AT-33-E3-001` (root cause of the
6,854-of-11,652 engine-coverage gap, reported today as 41%),
and `AT-33-E4-001` (the cause of `unknown`), each in its own worktree isolation per §3's binding
rule. This cycle's `denominator-gate` stage is a candidate consumer for those cycles' own receipts
(add their `artifacts/epic-{2,3,4}-*/**/*_cycle_receipt.md` globs to `DEFAULT_GLOBS`, or point
`DENOMINATOR_GATE_PATHS` at them explicitly) once they land — not done here, since no Epic 2/3/4
receipt exists yet to check against.
