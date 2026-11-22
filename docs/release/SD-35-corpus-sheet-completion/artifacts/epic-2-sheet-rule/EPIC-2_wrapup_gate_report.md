# SD-35 Epic 2 — Sheet rule: wrap-up gate report

`workflow-instruction.md §10` steps 0–3, run by the **isolated read-only worker**
(`decisions.md §3`). This worker pushed nothing and committed nothing. The orchestrator hands
this file, and the retrospective events named in §5, to the next cycle agent to commit.

- **Tree:** worktree `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-14`,
  reset to `origin/tranche/15` at **`a542652c5e`** ("docs(sd35): pin AT-35-E2-005-DISPOSITION
  cycle-2 receipt's own commit SHA") — the Epic 2 tip.
- **Build dir:** `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E2-WRAPUP`, `CARGO_INCREMENTAL=0`.
- **Verdict:** **RESULT: FAIL** — 45 stages passed, **3 red**. Epic 2 is **not** green.

---

## 1. Step 0 — the full gate, once

**Command:** `scripts/verify.sh` (full mode, no `--only`), wrapped in `/usr/bin/time -v`.
**Wall clock: 1:39:21** (5,962 s), max RSS 4,716,212 kB. Ran at 2 build jobs (`jobs: 2` —
verify.sh's own concurrency choice under a loaded box; the launch-audit figure of 70 min for
`root-full` was measured at `-j 6`).

- **Console log:** `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/c57a7080-1e94-4af6-9023-fa4f4f21e03f/scratchpad/verify-e2-wrapup.log`
- **Per-stage logs:** `/tmp/codex-verify-4HYI3n/`

### Stage table — 48 stages, 45 PASS / 3 FAIL

| # | Stage | Result | Detail |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK (859G of the 1.5T filesystem used = 60%, 593G available) |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 27 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 8 passed, 0 failed |
| 8 | **site-dashboard-check** | **FAIL** | exit 1 — `site/dashboard/PF1e-dashboard.json is STALE` |
| 9 | site-dashboard-pi-gate | PASS | 21 files scanned vs 1612 declared-PI names, zero leaked |
| 10 | build-public-status-selftest | PASS | 37 cases passed |
| 11 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` current |
| 12 | site-public-status-pi-gate | PASS | 31 files scanned, zero leaked |
| 13 | site-asset-stamp-check | PASS | cache-busting stamps match |
| 14 | **reachability-audit-selftest** | **FAIL** | self-test exit 1; ran 11, 1 failure |
| 15 | reachability-audit | PASS | reachable ceiling 100.00% of the 49,438-unit inventory |
| 16 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 17 | supersession-gate-selftest | PASS | 16 cases passed |
| 18 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 19 | shape-coverage-standing-gate | PASS | population=3497 unclassified=0 no_record=0 |
| 20 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 21 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 22 | shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=**363** citation_ok=True |
| 23 | missing-engine-tables | PASS | population=**1** kinds=1 citation_failures=0 |
| 24 | denominator-gate | PASS | files_checked=233 violations=0 |
| 25 | **figure-provenance** | **FAIL** | violations=**4** of figures_examined=194 (files_checked=163) |
| 26 | pcgen-residue-gate | PASS | `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` |
| 27 | token-coverage-selftest | PASS | 14 cases passed |
| 28 | token-coverage | PASS | non_done=1404 tokened=1399 token_less=5 refused=1810 refused_non_done=659 token_types=231 shapes=81 verdict=PASS |
| 29 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 30 | declared-pi-audit | PASS | clean |
| 31 | audit-selftest | PASS | 28 passed, 0 failed |
| 32 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 33 | driver-selftest | PASS | 7 passed, 0 failed |
| 34 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 35 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 36 | root-lib | PASS | 3217 passed |
| 37 | root-full | PASS | **8721 passed across 411 suites, all 361 `tests/*.rs` suites executed**, 0 failed |
| 38 | desktop | PASS | 574 passed |
| 39 | reach | PASS | 32 passed |
| 40 | corpus-sweep | PASS | 48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, **0 findings** |
| 41 | sheet-rules-check | PASS | records=49438 converted=47628 refused=1810 rules=66514 var_tables=5081 verdict=PASS (111.9 s) |
| 42 | corpus-trap-audit | PASS | records_examined=27634; defects at registered counts (wiring-class-mismatch=0, disabled-line=165, key-differs-from-name=650, mod-record=2117, shared-name-distinct-records=249); traps=407 |
| 43 | supersession-gate | PASS | 116 objects, all clean |
| 44 | frontend-install | PASS | `npm ci` |
| 45 | frontend-test | PASS | 101/101 files |
| 46 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 47 | clippy | PASS | root 0 / desktop 0 warnings, 0 errors |
| 48 | class-dump | PASS | **31/31 computing** |

### The three red stages, named exactly

**R1 — `site-dashboard-check`.** Literal log
(`/tmp/codex-verify-4HYI3n/site-dashboard-check.log`, the whole file):

```
site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh
```

Epic 2's corpus-wide conversion (AT-35-E2-005, `closed=21911`) moved every doneness figure in the
inventory; the published dashboard JSON was never regenerated. **Fix:** run
`./scripts/publish-site-dashboard.sh` and commit `site/dashboard/PF1e-dashboard.json`.
Not a code defect. **This stage was already red 4 times in the SD-35 window** before this run
(`retro.py summary … .verification.by_failing_stage`), so the correction cycle should also decide
who regenerates it — see §3.

**R2 — `reachability-audit-selftest`.** One test of 11 fails
(`/tmp/codex-verify-4HYI3n/reachability-audit-selftest.log`):

```
FAIL: test_real_inventory_ambiguous_is_the_known_no_done_path_class
  (scripts.tests.test_reachability_audit.RealCorpusStandingGateTest)
  File "scripts/tests/test_reachability_audit.py", line 216
    self.assertIn("ambiguous", no_done)
AssertionError: 'ambiguous' not found in set()
```

This is a **fourth SD-34-era live-figure equality pin** of exactly the shape AT-35-E1-002 already
corrected in `test_shape_engine_boundary.py`, `test_completion_atlas.py` and
`test_missing_engine_tables.py` — it pins a pre-conversion inventory fact ("`ambiguous` is a
class with no path to done") that Epic 2's conversion made false: the no-done set is now **empty**.
The gate's own live run (`reachability-audit`, stage 15) is **PASS at a reachable ceiling of
49,438 of 49,438 units (100.00%)**, which is the corrected truth. **Fix:** re-pin line 216 the way AT-35-E1-002 re-pinned
the other three — assert the property, not the SD-34 value.

**R3 — `figure-provenance`.** 4 violations, **all four in one file**, the last Epic 2 commit's
receipt (`/tmp/codex-verify-4HYI3n/figure-provenance.log`):

```
VIOLATION …/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle2_receipt.md:117: [unsourced] - non-DONE **1,404 of 49,438**, DONE **48,034**, bucket row `A 1 B 437 C 79 D 43 M 63 V 392
VIOLATION …/AT-35-E2-005-DISPOSITION_cycle2_receipt.md:121: [unsourced] - refused non-DONE **659 of 1,404**, shapes **81**, token types **231** —
VIOLATION …/AT-35-E2-005-DISPOSITION_cycle2_receipt.md:126: [unsourced] - the hand-off partition (owners 659 / 391 / 217 / 137, sum 1,404; 13 cells; by kind; the 69
VIOLATION …/AT-35-E2-005-DISPOSITION_cycle2_receipt.md:130: [unsourced] 1,404 non-DONE, 1,810 refused records; it imports `scripts/completion_atlas._bucket_of`, so DONE
files_checked=163 / figures_examined=194 / violations=4
```

The figures are **not** actually unsourced — each carries its re-derive command, but wrapped onto
the *following* line, and `denominator_gate.find_provenance_violations` only accepts a command
"reachable from it **on that same line**" (`scripts/denominator_gate.py:413-451`). The two
neighbouring bullets at lines 132–136 pass because their command is inline. **Fix:** docs-only —
put the backticked command on the same line as the figure in those four bullets. The same four
lines are why this stage read red 10 times in the SD-35 window (§3).

### Baseline notes emitted by the gate (not failures — 5 stale pins)

`scripts/verify-baselines.env` is behind at every scope, all in the *upward* direction. The
correction cycle should update them in the same commit:

| Key | Recorded | Measured |
|---|---|---|
| `BASELINE_ROOT_LIB_TESTS` | 3186 | 3217 |
| `BASELINE_ROOT_FULL_TESTS` | 8656 | 8721 |
| `BASELINE_ROOT_TEST_BINARIES` | 408 | 411 |
| `BASELINE_DESKTOP_TESTS` | 573 | 574 |
| `BASELINE_FRONTEND_TEST_FILES` | 100 | 101 |

### PCGen residue gate

`python3 scripts/pcgen_residue_gate.py --check` run standalone as well as inside the gate; the
literal last line, identical both times:

```
live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
```

The live side is exactly at baseline: Epic 2 added **no** PCGen reader to
`src/rules_core` (minus `cache_gen`), `src/saved_character`, `src/campaign`,
`src/homebrew_authoring` or `apps/desktop`. `decisions.md §11` holds.

---

## 2. Step 1 — retrospective summary

`python3 scripts/retro.py summary --since 2026-09-07 --json`. (The dispatch named
`--since 53296d80f0`; `retro.py` takes a *time*, not a SHA — that commit is dated
`2026-09-07 21:24:13 -0400`, so `2026-09-07` is the window that contains the epic.)

**Window totals — 208 events, 90 commits, 2.31 events/commit, 1 author.**

| Type | Count |
|---|---|
| verification | 134 |
| correction | 35 |
| incident | 18 |
| deferral | 10 (open 10, resolved 0) |
| resolution | 9 |
| rework | 2 |
| near-miss | 0 (escaped 0) |

**Corrections — 35, of which 18 carry a blast radius.** Caught before: implementation 12,
merge 2, brief 1. Two repeat subjects, both package prose, both 2×: `epic-breakdown.md
AT-35-E1-004 criterion text` and `epic-breakdown.md AT-35-E2-001 evidence`. Corrections by
corrector: `AT-35-E2-005` 8, `AT-35-E2-001` 5, `AT-35-E2-002` 4, `AT-35-E1-002` 4,
`AT-35-E2-003` 3, `AT-35-E2-004` 3, others 1–2. **The single largest correction class is again
our own package prose, not code** — AGENTS.md rule 9's shape, unchanged.

**Verification runs — 134, fail rate 0.1119 (15 failed).** Failing stages:
`figure-provenance` 10, `site-dashboard-check` 4, `shape-engine-boundary-selftest` 1.
**Two of the three stages red in this wrap-up gate were already red 14 times between them
during the epic**, and no cycle owned either.

**Rework — 2.** The load-bearing one is `AT-35-E2-005` cycle 4 re-running cycle 3's whole
measurement at the same tree, byte-identical: cycle 3 returned `partial` on a criterion whose own
rule (zero new mappings) makes its scoped population unreachable, and the dispatcher's remainder
loop re-dispatches on `partial`. The receipt and the prompt both *named* the §8 re-scope rule;
nothing mechanical stopped the re-dispatch. That is an AGENTS.md rule 8 "a warning is not a
control" instance, and it is what `decisions.md §16`'s amendment then fixed by hand.

### 2a. Incident keys firing 3+ times → control or escalation (mandatory)

Only **one** key reached 3: **`disk-full`, 12.** It requires a named mechanical control or a named
escalation, and the honest answer is that **the control already exists and is working — the count
is an instrument defect.**

All 12 rows are *successful* `scripts/reclaim.sh --apply` cron runs on a 4-hourly schedule, each
logged twice (once as actor `root`, once as `codex`), every one with `used_percent = None` and a
summary of the form `reclaim.sh --apply removed N item(s), <size> total`. **Zero of the 12 is a
disk-pressure failure.** This run's `preflight-disk` stage passed with 859 G used of the 1.5 T
filesystem and 593 G available.

- **Named mechanical control (already built, verified firing):** `scripts/reclaim.sh --apply` on
  the 4-hourly cron, plus `scripts/verify.sh`'s `preflight-disk` stage which refuses to start a
  build under budget. Between them they reclaimed 49.2 GB, 44.9 GB and 42.6 GB on three separate
  occasions inside this window without a human in the loop.
- **The residual defect, and its named owner:** `reclaim.sh` logs a routine success as
  `type=incident recurrence-key=disk-full`, so a working control reads as the tranche/7
  120-times-fired catastrophe key. It buries the keys that *are* real. Filed as a correction
  (`1788901958251-at-35-e2-wrapup-a6a500`); the one-line fix is for reclaim.sh to emit
  `resolution` (or `verification`) on a clean run and reserve `incident` for an actual
  low-space event. **Escalated to the Epic 2 wrap-up correction cycle**, since this worker
  commits nothing.

Below the threshold but worth naming: `duplicate-criterion-dispatch` **2** (AT-35-E1-001 and
AT-35-E1-005 each re-dispatched after they had already landed and pushed; both agents rebased,
found the criterion at zero, and re-verified rather than duplicating work — the loss was two
cycles of wall time, not correctness). One more instance makes it a 3+ key; the mechanical control
is the one `workflow-instruction.md §2.4` already describes — rebuild the criteria list from
`kanban.md` with `complete` rows excluded — applied at *every* relaunch, not only the
quota-exhaustion relaunch.

### 2b. `rust_lines_changed / units_closed` above 3.0 (`decisions.md §4`)

Read from each Epic 2 receipt's literal `cycle_scope_gate.py --receipt` row.

| Cycle | rust_lines_changed | units_closed | ratio |
|---|---|---|---|
| AT-35-E2-001 c1 | 6463 | 0 | n/a |
| AT-35-E2-001 c2 | 0 | 0 | n/a |
| AT-35-E2-002 c1 | 1818 | 0 | n/a |
| AT-35-E2-002 c2 | 0 | 0 | n/a |
| AT-35-E2-003 c1 | 442 | 0 | n/a |
| AT-35-E2-003 c2 | 0 | 0 | n/a |
| AT-35-E2-004 c1 | 295 | 0 | n/a |
| AT-35-E2-004 c2 | 0 | 0 | n/a |
| AT-35-E2-005 c1 | 337 | **21911** | **0.02** |
| AT-35-E2-005 c2 | 120 | 0 | n/a |
| AT-35-E2-005 c3/c4/c5 | 0 | 0 | n/a |
| AT-35-E2-005-DISPOSITION c1/c2 | 0 | 0 | n/a |

**No cycle exceeded 3.0.** The one cycle with a denominator scored **0.02**. Epic-wide:
**9,475 Rust lines / 21,911 units closed = 0.43** — an order of magnitude under the bar, and the
opposite of SD-34's transcription shape the ratio was built to catch.

The §4 review is still owed for the four cycles that changed Rust with `closed=0` (`ratio=n/a`
is not an exemption). **What those lines bought:**

- **AT-35-E2-001 c1 — 6,463 lines.** The converter itself (`sheet_rule_convert`), tool-side, plus
  `data/sheet_rules/_refused.json`. It is the one artefact the other 21,911 closures are produced
  by; per §4 it is explicitly the *permitted* shape ("one converter mapping row per token type"),
  not per-unit machinery. Amortised across cycle 5's closure it is 0.29 lines/unit.
- **AT-35-E2-002 c1 — 1,818 lines.** The live-side evaluator and sheet section
  (`src/rules_core/sheet_rule.rs`, `render_sheet`/`Evaluator::line`) — the surface that turns a
  converted rule into a rendered line. Without it the conversion is data nobody reads; this is the
  "magnitude is not wired until it moves on the twin the player reads" requirement.
- **AT-35-E2-003 c1 — 442 lines.** `v06_work_inventory.rs` (+430/−6) and `companion_chassis.rs`
  (+5/−1): the `sheet-complete` status itself, one generic classifier pass over the whole
  inventory — not a per-family probe.
- **AT-35-E2-004 c1 — 295 lines.** The token-coverage ledger's three inventory fields and their
  corpus-fixture gate (`sheet_rule_convert_gate.rs` +38), one gate per *kind* reading the live
  corpus — again §4's permitted shape.
- **AT-35-E2-005 c2 — 120 lines.** One live-side file, `src/rules_core/sheet_rule.rs` (+117/−3, of
  which +93 is the test hunk). What they bought is stated mechanically in the receipt:
  **11 → 0 oracle-parity disagreements.**

### 2c. Open deferrals at Epic 2 close

**10 open, 0 resolved** in the window. They are enumerated in the retro log; the largest is
AT-35-E1-003's 244 `grounding_ref` / doc-comment citations in `src/` and 4 `docs/architecture`
pages still naming the pre-consolidation test paths, revisit-tagged to AT-35-E7-003. None is a
blocker to Epic 3; all carry a named revisit.

---

## 3. Step 2 — worktree sweep (this epic's worktrees only)

`df -h /` → `/dev/sda1 1.5T, 859G used, 594G available` — 859 G of 1.5 T, i.e. 60% of the one
filesystem this box has. **Not under pressure.**

`git worktree list` at the time of the sweep:

```
/home/ubuntu/workspace/repos/codex                          a542652c5e [tranche/15]
…/.claude/worktrees/wf_291be5c8-5f3-1   06872eff73
…/.claude/worktrees/wf_291be5c8-5f3-2   4e321d2c6c
…/.claude/worktrees/wf_291be5c8-5f3-3   986084c5a4
…/.claude/worktrees/wf_291be5c8-5f3-4   942c8d3ae5
…/.claude/worktrees/wf_291be5c8-5f3-5   3c43cf0531
…/.claude/worktrees/wf_291be5c8-5f3-7   928272a444
…/.claude/worktrees/wf_291be5c8-5f3-14  a542652c5e   ← this worker, **locked**
…/.claude/worktrees/wf_291be5c8-5f3-15  8cc4ea1516
```

**Nothing was pruned, deliberately.** Two independent reasons, both recorded as a deferral
(`1788901958384-at-35-e2-wrapup-cb0191`):

1. This agent is the isolated read-only worker; the harness refuses any git operation that targets
   a worktree other than its own, so a sweep of the siblings is not this role's to run.
2. A sibling lane was observably **live** during the gate — `AT-35-E3-001` running
   `/tmp/cargo-sd35-AT-35-E3-001/debug/v06_work_inventory` (pid 2252660), burning most of one of
   the box's 24 CPU cores. Removing a
   worktree under a running lane is the destructive shape, and `wf_291be5c8-5f3-14` is `locked`
   and must never be removed by anyone else either.

**What the correction cycle should reclaim** (`/tmp/cargo-sd35-*`, ~146 GB total):

| Dir | Size | Status |
|---|---|---|
| `/tmp/cargo-sd35-launch` | 41 G | orphan — the pre-launch audit's, finished 2026-09-07 |
| `/tmp/cargo-sd35-epic-2-wrap-up-gate` | 36 G | orphan — an earlier wrap-up attempt's, superseded by this run |
| `/tmp/cargo-sd35-AT-35-E2-002` | 33 G | Epic 2 lane, complete |
| `/tmp/cargo-sd35-AT-35-E2-005` | 33 G | Epic 2 lane, complete |
| `/tmp/cargo-sd35-AT-35-E2-002-desktop` | 3.6 G | Epic 2 lane, complete |
| `/tmp/cargo-sd35-AT-35-E2-WRAPUP` | 32 G | **this run's — deleted by this worker on exit** |

`/tmp/codex-verify-4HYI3n/` (per-stage logs) is deliberately **kept** — the correction cycle needs
`site-dashboard-check.log`, `reachability-audit-selftest.log` and `figure-provenance.log`.

## 4. Step 3 — no PR

None opened. Correct per `§10` step 3.

---

## 5. Hand-off to the Epic 2 wrap-up correction cycle

Uncommitted in this worker's worktree, for the next cycle agent to commit:

- `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/EPIC-2_wrapup_gate_report.md` (this file)
- `docs/retro/events/at-35-e2-wrapup.jsonl` — 5 events: 3 `incident`
  (`1788901933628-…477167` the gate result, `…933760-…bfe596` the recurring stale dashboard JSON,
  `…933887-…e21e18` the recurring figure-provenance authoring shape), 1 `correction`
  (`1788901958251-…a6a500`, the `disk-full` instrument defect), 1 `deferral`
  (`1788901958384-…cb0191`, the un-swept worktrees).

**The correction cycle's work, in order (exempt from the batch floor per `decisions.md §2`,
never from the residue check):**

1. `./scripts/publish-site-dashboard.sh`; commit `site/dashboard/PF1e-dashboard.json`.
2. Re-pin `scripts/tests/test_reachability_audit.py:216` on the property, not the SD-34 value.
3. Inline the re-derive command on lines 117, 121, 126, 130 of
   `AT-35-E2-005-DISPOSITION_cycle2_receipt.md`.
4. Update the 5 stale keys in `scripts/verify-baselines.env`.
5. Make `scripts/reclaim.sh` log a clean run as something other than `incident/disk-full`.
6. Re-run `scripts/verify.sh` — the whole gate, once — and record it here.

It must land **before Epic 3's second cycle dispatches** (`§10` step 0).
