# Epic 3 — Place and surface — wrap-up gate report

- **Worker:** isolated read-only wrap-up worker (`workflow-instruction.md §2` worker split,
  `decisions.md §3`). Pushes nothing, commits nothing. The orchestrator hands this file to the
  next cycle agent to commit.
- **Worktree:** `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-21`
- **`CARGO_TARGET_DIR`:** `/home/ubuntu/workspace/cargo-sd35-epic3-wrapup` (not under `/tmp` — AGENTS.md
  "Concurrency and Measurement")
- **HEAD the gate ran at:** `07e29075b4` (`docs(sd35): pin AT-35-E3-004 cycle 1's own commit sha in
  its receipt and kanban row`) — the tip of `origin/tranche/15` at gate start
- **Verdict:** **RED — 2 failing stages.** Status returned: `blocked-escalated`.

---

## 0. The full gate, once

```
scripts/verify.sh -j 6          # every stage, no --only
```

- **Log directory:** `/tmp/codex-verify-O7651A`
- **Console transcript:** `/tmp/e3gate_verify.out`
- **Wall time:** **5,385 s = 89 min 45 s** (start epoch 1788931710 → end epoch 1788937095,
  `date +%s` either side of the run)
- **Result line:** `RESULT: FAIL — logs in /tmp/codex-verify-O7651A`, exit 1
- **Stage tally:** **46 PASS, 2 FAIL** of 48 stages.

### Correction found before the gate could run

The dispatched worktree was checked out at `fe5ae6cd4a` — **31 commits behind
`origin/tranche/15`**, i.e. before the `tranche/15` cut, before every Epic 1, Epic 2 and Epic 3
commit. Running the gate there would have produced a green-looking report for a tree containing
none of the work the gate was supposed to certify. Caught by `git log --oneline -1` against
`git log --oneline origin/tranche/15 -1` before the first build; the worktree was reset to
`07e29075b4` and the gate run there. This is `wrong-base-worktree` — the failure AGENTS.md rule 8
names as having fired 27 times behind a warning.

### Full stage table

| # | Stage | Result | Detail |
|---|---|---|---|
| 1 | preflight-disk | PASS | 62% used of the 1.5T root filesystem, 556G available |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 27 cases |
| 5 | pi-redaction-selftest | PASS | 49 cases |
| 6 | provenance-selftest | PASS | 32 cases |
| 7 | site-dashboard-selftest | PASS | 8 passed, 0 failed |
| 8 | **site-dashboard-check** | **FAIL** | exit 1 — `site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh` |
| 9 | site-dashboard-pi-gate | PASS | |
| 10 | build-public-status-selftest | PASS | |
| 11 | site-public-status-check | PASS | |
| 12 | site-public-status-pi-gate | PASS | |
| 13 | site-asset-stamp-check | PASS | |
| 14 | reachability-audit-selftest | PASS | |
| 15 | reachability-audit | PASS | |
| 16 | groundtruth-guard-selftest | PASS | |
| 17 | supersession-gate-selftest | PASS | |
| 18 | shape-coverage-standing-gate-selftest | PASS | |
| 19 | shape-coverage-standing-gate | PASS | |
| 20 | cycle-scope-gate-selftest | PASS | |
| 21 | shape-engine-boundary-selftest | PASS | |
| 22 | shape-engine-boundary | PASS | |
| 23 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 24 | denominator-gate | PASS | files_checked=239 violations=0 |
| 25 | **figure-provenance** | **FAIL** | violations=14 of figures_examined=217 (files_checked=169) |
| 26 | pcgen-residue-gate | PASS | live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS |
| 27 | token-coverage-selftest | PASS | 14 cases |
| 28 | token-coverage | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=232 shapes=1 verdict=PASS |
| 29 | pi-sweep | PASS | 11 hits, 11 baseline rows |
| 30 | declared-pi-audit | PASS | clean |
| 31 | audit-selftest | PASS | 28 passed, 0 failed |
| 32 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 33 | driver-selftest | PASS | 7 passed, 0 failed |
| 34 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 35 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 36 | root-lib | PASS | 3,220 passed |
| 37 | root-full | PASS | **8,727 passed across 411 suites, all 361 `tests/*.rs` suites executed** |
| 38 | desktop | PASS | 574 passed (`apps/desktop/src-tauri`, the separate workspace) |
| 39 | reach | PASS | 32 passed |
| 40 | corpus-sweep | PASS | 48,706 records examined of 51,476 read, 413,314 tokens compared, 51,463 digests checked, **0 findings** |
| 41 | sheet-rules-check | PASS | records=49438 converted=49296 refused=142 rules=69344 var_tables=5269 verdict=PASS (114.4 s) |
| 42 | corpus-trap-audit | PASS | records_examined=27,634, all defect kinds at registered counts, traps=407 |
| 43 | supersession-gate | PASS | 116 objects, all clean |
| 44 | frontend-install | PASS | `npm ci` (node_modules was absent in this fresh worktree) |
| 45 | frontend-test | PASS | 101/101 files |
| 46 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 47 | clippy | PASS | root 0, desktop 0 warnings, 0 errors (BOTH crates) |
| 48 | class-dump | PASS | 31/31 computing |

### The two red stages, named exactly

**RED 1 — `site-dashboard-check`**
`timeout 2400s scripts/publish-site-dashboard.sh --check` → exit 1.
Literal log content (`/tmp/codex-verify-O7651A/site-dashboard-check.log`, one line):
`site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh`.
**Attribution:** Epic 3's own work. AT-35-E3-001 cycle 2 and AT-35-E3-002 cycle 1 regenerated
`docs/work-inventory.json` (corpus to DONE 49,438 of 49,438); the site dashboard reads that
inventory and was never republished after either regeneration. This is the recorded incident key
`site-dashboard-json-stale-after-inventory-move`, now firing a second time in this window.
**Fix:** run `./scripts/publish-site-dashboard.sh` in the wrap-up correction cycle and commit the
refreshed `site/dashboard/PF1e-dashboard.json`. Not self-healable here: this worker pushes nothing.

**RED 2 — `figure-provenance`**
`python3 scripts/denominator_gate.py --check-provenance` → `violations=14 of figures_examined=217
(files_checked=169)`. **All 14 violations are in two Epic 3 receipts, and in no other file:**

| File | Violating lines | Count |
|---|---|---|
| `artifacts/epic-3-place-and-surface/AT-35-E3-002_cycle1_receipt.md` | 111, 112, 114, 117, 123, 127, 129, 130 | 8 |
| `artifacts/epic-3-place-and-surface/AT-35-E3-003_cycle1_receipt.md` | 113, 114, 121, 123, 124, 131 | 6 |

Every one is `[unsourced]` — a figure stated without the command that re-derives it adjacent to it.
This is AGENTS.md rule 9 ("every figure you write down carries the command that produced it") and
the recorded incident key `figure-provenance-command-on-next-line`: the gate requires the command
on the *same* line as the figure, and these receipts put it on the following line or in a nearby
prose sentence. `figure-provenance` is the **top failing stage of the whole SD-35 window — 11 of
the 16 failed verification runs of 140** (`retro.py summary --since 2026-09-07 --json`,
`verification.by_failing_stage`).
**Fix:** in the wrap-up correction cycle, rewrite the 14 lines so each figure carries its
re-derive command inline, then re-run `--check-provenance` to `violations=0`. The figures
themselves are not in question — `denominator-gate` (the sibling stage over the same corpus of
prose) is PASS at `files_checked=239 violations=0`.

### Baseline note (not a failure, but it must be updated deliberately)

`BASELINE_ROOT_FULL_TESTS` in `scripts/verify-baselines.env` reads **8,724**; the gate measured
**8,727**. The +3 are AT-35-E3-003 cycle 1's control tests (its receipt records 37 `#[cfg(test)]`
lines added, including the `strips_the_size_suffix_off_a_pool_magnitude` control). The baseline
belongs in the same wrap-up correction cycle's commit, not folded silently.

---

## 1. Retrospective summary — `retro.py summary --since 2026-09-07 --json`

Command run: `python3 scripts/retro.py summary --since 2026-09-07 --json`. The dispatch named
`--since 53296d80f0`, which `retro.py` rejects (`retro: cannot parse time '53296d80f0'`); that sha
is dated `2026-09-07 21:24:13 -0400`, so `2026-09-07` is the window that contains it and everything
after. Saved at `/tmp/e3_retro.json`.

**Window totals:** 233 events over 107 commits (2.18 events/commit), 21 log shards, **0 invalid
lines, 0 problems**.

| Type | Count |
|---|---|
| verification | 140 (16 failed runs of 140, fail rate **11.43%**) |
| correction | 44 |
| incident | 23 (111 minutes lost, **0 silent**) |
| deferral | 13 (13 open, 0 resolved) |
| resolution | 9 |
| rework | 2 |
| note | 2 |
| near-miss | 0 |

Corrections were caught before: implementation 12, merge 4, brief 1, plus 2 narrative entries.
21 of 44 carry a blast radius; 2 subjects were corrected more than once. Epic 3's own actors
contributed 9 events (E3-001 4, E3-002 2, E3-003 2, E3-004 1) and 6 corrections. The 134 events
attributed to `sd31-transcribe` are another session's verification traffic sharing the log, not
SD-35 cycle events.

### Incident keys firing 3+ times → the required control or escalation

| Recurrence key | Count | Disposition |
|---|---|---|
| **`disk-full`** | **14** | **Mechanical control built and landed — not a warning.** The Epic 2 wrap-up fix cycle proved 12 of the 14 were phantom: `scripts/reclaim.sh` emitted `incident/disk-full` on *every* routine `--apply`, whether or not disk was under pressure, so the control working looked identical to the control failing. The fix (TDD, `scripts/tests/test_reclaim.py`, RED on 2 new cases then 23 tests OK) makes `reclaim.sh` read `df -P` used-percent and emit `incident/disk-full` **only at or above `RECLAIM_PRESSURE_PERCENT` (default 90)**, a `note` tagged `reclaim-routine` below it, recording `used_percent` on the event either way. Verified in this tree: `scripts/reclaim.sh` lines 791, 820, 823, 833. The real disk figure this gate measured is 62% used / 556G free — no pressure. **The residual true count is 2, below the 3-fire threshold.** Without this fix the SD-35 closure retrospective would have read 12 phantom incidents as a missing control. |
| `duplicate-criterion-dispatch` | 2 | Below the threshold, but it caused **both** of the window's 2 rework events, so it is named here rather than ignored. Cause 1: "orchestrator dispatch list not reconciled against `kanban.md` before dispatch — the criterion's row already read `complete` with a receipt path". Cause 2: a cycle returned `partial` on a criterion whose own rule (zero new mappings) makes its population unreachable, and the dispatcher's remainder loop re-dispatches on `partial`; §8's `>10-refused-type` re-scope rule was named in both the receipt and the dispatch prompt, **and nothing mechanical stopped the re-dispatch**. **Escalation, named:** the orchestrator's dispatch loop has no pre-dispatch reconciliation against `kanban.md`, and treats `partial` as unconditionally re-dispatchable. If it fires a third time it is a missing mechanism under AGENTS.md rule 8, not bad luck. |

The remaining 7 incident keys fired once each: `denominator-gate-red-on-package-prose`,
`denominator-gate-red-on-untouched-artifacts`, `epic-wrapup-gate-red`,
`figure-provenance-command-on-next-line`, `package-gate-red-at-cycle-start`,
`pre-existing-red-gate`, `site-dashboard-json-stale-after-inventory-move`. **Two of those seven
are exactly this gate's two red stages, now on their second fire each** — they are the pattern to
watch at Epic 4's wrap-up, and `figure-provenance` is already the window's dominant failing stage
at 11 of 16 failed runs.

### Cycles whose `rust_lines_changed / units_closed` exceeded 3.0

**None. Zero cycles in the window exceed 3.0.** Every cycle that closed a non-zero number of units
is far under the `decisions.md §4` bar:

| Cycle | rust_lines_changed | units_closed | ratio |
|---|---|---|---|
| AT-35-E2-005 cycle 2 | 337 | 21,911 | **0.02** |
| AT-35-E3-001 cycle 2 | 232 | 618 | **0.38** |
| AT-35-E3-002 cycle 1 | 266 | 786 | **0.34** |

Re-derived from `grep -h 'Receipt rows (mechanical)' docs/release/SD-35-corpus-sheet-completion/artifacts/epic-*/*_receipt.md`
across all 26 committed receipts. Denominator: 26 receipts; 23 report `ratio=n/a` because they
closed zero units (a division by zero, never a 0.0).

**What the 3 ratio-bearing cycles' lines bought:** E2-005 c2's 337 lines are one new tool-side
binary (252) plus four count-pin edits — no live-side code — and they moved 21,911 units to DONE.
E3-001 c2's 232 lines are one converter policy (term-level refusal: an unlowerable token degrades
the term instead of deleting the record) which closed 618 units and emptied AT-35-E3-003,
AT-35-E5-001 and AT-35-E5-002 as a side effect. E3-002 c1's 266 lines are two token-less converter
joins plus widening the `sheet-complete` rung's promotable-status list from two statuses to seven,
which closed the entire 786-unit remainder and took the corpus to 49,438 of 49,438.

**The honest caveat the 3.0 bar does not catch.** A zero-closure cycle's ratio is undefined, so a
large Rust expenditure hides behind `n/a`. The four largest are named here so the closure
retrospective does not have to re-derive them: **AT-35-E2-001 cycle 1, 6,463 lines** (built the
whole converter — `src/bin/sheet_rule_convert.rs` and `src/pcgen_import/sheet_rule/`, a
converter-building cycle explicitly exempt from the floor, `decisions.md §9` L6);
**AT-35-E1-003 cycle 1, 1,906 lines** (+1,000/−906, the test-tax cut, its own before/after
measurement); **AT-35-E2-002 cycle 1, 1,818 lines** (the live-side `sheet_rule.rs` evaluator);
**AT-35-E2-003 cycle 1, 442 lines** (the status vocabulary and rung). Each bought a mechanism the
later unit-closing cycles spent 835 lines total consuming. That is the bundle's actual shape: the
lines are front-loaded into converters and rungs, and the closures are cheap.

### Open deferrals: 13, none deferring Epic 3's DoD

All 13 are open, 0 resolved. Read against Epic 3's acceptance bar
(`epic-breakdown.md` AT-35-E3-001..004), **none of them defers scope that was in Epic 3's
Definition of Done**; they are capability deferrals with named revisit points
(`docs/governance/deferral-revisit-doctrine.md`), overwhelmingly of the form "this criterion's own
text forbids the mechanism that would move these units; the successor criterion owns them". The
two that touch Epic 3 directly:

- `1788879242003-at-35-e3-001-bf4043` — AT-35-E3-001 cycle 1 refused to start: `scoped=214`,
  under the 500 floor and not the whole 1,404 remainder, 24 distinct refused token types. Correct
  behaviour under `§8`; the orchestrator re-scoped and cycle 2 closed 618. Superseded.
- `1788899844992-at-35-e3-001-e163d1` — 2 units join to no corpus record at all, so the converter
  has no source row to convert. Revisit named as AT-35-E3-002, whose cycle 1 then closed the whole
  786-unit remainder including them. Superseded.

Epic 3's criteria are all at zero population at this HEAD: `completion_atlas.py --check` →
DONE 49,438 of 49,438 (confirmed by this gate's `missing-engine-tables` PASS at `population=0
kinds=0` and `token-coverage` PASS at `non_done=0 refused_non_done=0`).

---

## 2. Worktree sweep — this epic's worktrees

```
df -h /              → /dev/sda1  1.5T  964G used  489G avail  67% used
git worktree list    → 10 entries (the main checkout + 9 agent worktrees)
```

Disk is **not** under pressure (67% used of the 1.5T root filesystem, below `reclaim.sh`'s 90% threshold; it read 62% of that same 1.5T at gate start,
the delta being this gate's own `CARGO_TARGET_DIR` and `npm ci`).

| Worktree | HEAD | Ancestor of `origin/tranche/15`? | Note |
|---|---|---|---|
| `codex` (main checkout) | 07e29075b4 | — | the shared checkout, `tranche/15` |
| `wf_291be5c8-5f3-14` | a542652c5e | yes (merged) | AT-35-E3-001 cycle 1 base |
| `wf_291be5c8-5f3-15` | 8cc4ea1516 | yes (merged) | AT-35-E3-001 cycle 2 base |
| `wf_291be5c8-5f3-17` | 9995efa1b6 | yes (merged) | AT-35-E3-002 cycle 1 base |
| `wf_291be5c8-5f3-2` | 4e321d2c6c | yes (merged) | Epic 1 |
| `wf_291be5c8-5f3-21` | 07e29075b4 | yes (merged) | **this worker — `locked`** |
| `wf_291be5c8-5f3-3` | 986084c5a4 | yes (merged) | Epic 1 |
| `wf_291be5c8-5f3-4` | 942c8d3ae5 | yes (merged) | Epic 1/2 |
| `wf_291be5c8-5f3-5` | 3c43cf0531 | yes (merged) | Epic 2 |
| `wf_291be5c8-5f3-7` | 928272a444 | yes (merged) | Epic 2 |

Mergedness derived per worktree with `git merge-base --is-ancestor <sha> origin/tranche/15`
(exit 0 for all nine). **No worktree carries an unmerged commit**, so none is at risk of losing
work by content.

**Found 3 Epic 3 worktrees (wf-14, wf-15, wf-17); removed 0.** Two reasons, both hard:

1. This worker is the **isolated read-only worker** (`decisions.md §3`) and the harness refuses any
   git operation naming a path outside its own worktree — `git worktree remove` on a sibling is
   refused, as it was for AT-35-E2-WRAPUP (deferral `1788901958384-at-35-e2-wrapup-cb0191`).
2. **A later epic's lane is observably live on the shared checkout right now.** `ps` shows
   `cargo test --locked --no-fail-fast -j 6` (pid 2814609, 1,485 s elapsed) and an
   `AT-35-E4-001` parity build against `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E4-001` (pid
   2847668). Pruning during a live wave is how work gets destroyed
   (`agent-territories-must-be-file-level`).

`git worktree prune` would remove nothing here regardless: every entry's directory exists, so
there is no stale administrative record to reclaim. The removals are a one-command chore for the
orchestrator (which is not worktree-isolated) after Epic 4's wave lands, and disk gives it no
urgency. **This is a measured, named deferral, not a skipped step.**

## 3. Pull request

None. `workflow-instruction.md §10` step 3: **no PR at an epic wrap-up.**

---

## PCGen residue line

```
python3 scripts/pcgen_residue_gate.py --check
```

Literal verdict line:

```
live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
```

Exit 0. Unchanged from the AT-35-E1-005 baseline of 260 and from every Epic 2 and Epic 3 receipt's
`pcgen_live_files` row — **Epic 3 raised nothing on the live side** (`decisions.md §11`). The
gate's own `pcgen-residue-gate` stage (#26) reports the same line.

---

## What the next cycle must do

The wrap-up correction cycle is exempt from the batch floor (`decisions.md §9` L6 — it closes zero
units by design) but **not** from the residue check. It must:

1. Rewrite the 14 `[unsourced]` figure lines in `AT-35-E3-002_cycle1_receipt.md` (8) and
   `AT-35-E3-003_cycle1_receipt.md` (6) so each figure carries its re-derive command inline;
   re-run `python3 scripts/denominator_gate.py --check-provenance` to `violations=0`.
2. Run `./scripts/publish-site-dashboard.sh` and commit the refreshed
   `site/dashboard/PF1e-dashboard.json`; re-run `scripts/publish-site-dashboard.sh --check` to
   exit 0.
3. Update `BASELINE_ROOT_FULL_TESTS` in `scripts/verify-baselines.env` from 8,724 to 8,727 in the
   same commit, naming AT-35-E3-003 cycle 1's control tests as the cause.
4. Commit this report (the isolated worker wrote it but pushes nothing).

Per `§10` step 0 the fix lands **before Epic 4's second cycle dispatches**. AT-35-E4-001 cycle 1
was observed already running at the time of this report, which is within that rule.
