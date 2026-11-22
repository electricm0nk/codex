# Epic 3 — Place and surface — wrap-up gate report (RE-GATE)

- **Actor:** `AT-35-E3-REGATE` (isolated read-only worker, `workflow-instruction.md §2` worker split)
- **Worktree:** `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-24`
- **Gated sha:** `cdcfc897ea31e832825f70ec13a2c41db6a0f9d5` (`origin/tranche/15` at gate start)
- **CARGO_TARGET_DIR:** `/tmp/cargo-sd35-AT-35-E3-REGATE` (agent-private, per `AGENTS.md` Concurrency)
- **Log:** `/tmp/claude-1000/gate/verify-regate.log`; verify.sh's own stage logs in `/tmp/codex-verify-9z26lh`
- **This worker pushed nothing and committed nothing.** The report is handed to the orchestrator.

## 0. The full gate, once

`scripts/verify.sh` — every stage, no `--only`.

- **Wall time:** 5,616 s = **1 h 33 m 36 s**
- **Result:** `RESULT: PASS` — **48 of 48 stages PASS, 0 FAIL** (exit 0)

This re-gate is the confirmation run for the `AT-35-E3-WRAPUP-FIX` correction cycle
(`2dc322ae32`), which fixed the two stages that were red at `07e29075b4`
(`site-dashboard-check`, `figure-provenance`). Both are green here, at a HEAD three commits later
than the fix cycle's own run, so the green is not an artifact of the fix cycle's tree.

### Stage table

| # | Stage | Result | Detail |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 27 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 8 passed, 0 failed |
| 8 | site-dashboard-check | PASS | `site/dashboard/PF1e-dashboard.json` is current — **was red at `07e29075b4`** |
| 9 | site-dashboard-pi-gate | PASS | 21 files scanned against 1612 declared-PI names, zero leaked |
| 10 | build-public-status-selftest | PASS | 37 cases passed |
| 11 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` current |
| 12 | site-public-status-pi-gate | PASS | 31 files scanned, zero leaked |
| 13 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 14 | reachability-audit-selftest | PASS | 11 cases passed |
| 15 | reachability-audit | PASS | reachable ceiling 100.00%, i.e. 0 dead-end cells of 49438 units |
| 16 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 17 | supersession-gate-selftest | PASS | 16 cases passed |
| 18 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 19 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 |
| 20 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 21 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 22 | shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=**0** citation_ok=True |
| 23 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 24 | denominator-gate | PASS | files_checked=242 violations=0 |
| 25 | figure-provenance | PASS | files_checked=172 figures_examined=224 violations=0 — **was red (14 violations) at `07e29075b4`** |
| 26 | pcgen-residue-gate | PASS | live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 |
| 27 | token-coverage-selftest | PASS | 14 cases passed |
| 28 | token-coverage | PASS | non_done=0 refused=142 refused_non_done=0 token_types=231 |
| 29 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 30 | declared-pi-audit | PASS | clean |
| 31 | audit-selftest | PASS | 28 passed, 0 failed |
| 32 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 33 | driver-selftest | PASS | 7 passed, 0 failed |
| 34 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 35 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 36 | root-lib | PASS | 3220 passed |
| 37 | root-full | PASS | **8727 passed across 411 suites, all 361 `tests/*.rs` suites executed** |
| 38 | desktop | PASS | 574 passed |
| 39 | reach | PASS | 32 passed |
| 40 | corpus-sweep | PASS | 48706 records examined of 51476 read, 413314 tokens compared, 0 findings |
| 41 | sheet-rules-check | PASS | records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 (114.0 s) |
| 42 | corpus-trap-audit | PASS | records_examined=27634, all defect kinds at their registered counts, traps=407 |
| 43 | supersession-gate | PASS | 116 objects, all clean |
| 44 | frontend-install | PASS | `npm ci` |
| 45 | frontend-test | PASS | 101/101 files |
| 46 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 47 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 48 | class-dump | PASS | 31/31 computing |

**No red stage.** Nothing to hand to a wrap-up correction cycle.

### PCGen residue gate (`decisions.md §11`)

`python3 scripts/pcgen_residue_gate.py --check` — literal final line:

```
live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
```

The live surface has not grown by a single file or a single token hit against the
`AT-35-E1-005` baseline of 260. The converter, parser, generators and oracle harness are intact
(`root-full` 8727 passing and `sheet-rules-check` converting 49,296 records prove they still run).

## 1. Retrospective read (`python3 scripts/retro.py summary --since 2026-09-08 --json`)

Window `2026-09-08T00:00:00Z` → open. 175 events over 88 commits (1.99 events/commit, 1 author).

| Type | Count |
|---|---|
| verification | 78 |
| correction | 49 |
| incident | 18 |
| deferral | 14 (all 14 `open`) |
| resolution | 11 |
| note | 3 |
| rework | 2 |
| near-miss escaped | 0 |

Verify-stage health over the window: 78 runs, 13 failed (fail rate 0.167), failing stages
`figure-provenance` 12, `site-dashboard-check` 2, `reachability-audit-selftest` 1,
`shape-engine-boundary-selftest` 1.

Corrections caught before: implementation 13, merge 4, brief 1 — plus two singleton
catch-points (the Epic 2 wrap-up gate; the SD-35 closure retrospective).

### Incident keys firing 3+ times → mechanical control or named escalation

Only **one** incident `recurrence_key` reached 3 in this window.

- **`disk-full` — 6 firings. CONTROL LANDED, and the firings were phantom.**
  A correction in this window established that all 12 same-key rows since 2026-09-07 were
  `reclaim.sh --apply removed N item(s)` routine cleanups with `used_percent` unset — the control
  working, mis-recorded as the failure it prevents. `scripts/reclaim.sh` now reads `df -P`
  used-percent and emits `incident/disk-full` **only at or above `RECLAIM_PRESSURE_PERCENT`
  (default 90)**, emitting a `note` tagged `reclaim-routine` below it and recording
  `used_percent` on the event either way (`scripts/reclaim.sh:780-833`, TDD'd by
  `scripts/tests/test_reclaim.py`, 23 tests; `reclaim-selftest` PASS above). Disk during this
  1 h 34 m gate ran from 896 G to 929 G used of 1.5 T (62% → 64%), leaving 524 G free — nowhere
  near the threshold of 90% of that same 1.5 T volume.

Every other key fired at most twice and needs no control by the `AGENTS.md` rule:
`epic-wrapup-gate-red` 2, `duplicate-criterion-dispatch` 2,
`figure-provenance-command-on-next-line` 2, `site-dashboard-json-stale-after-inventory-move` 2,
`denominator-gate-red-on-package-prose` 1, `denominator-gate-red-on-untouched-artifacts` 1,
`package-gate-red-at-cycle-start` 1, `pre-existing-red-gate` 1.

### One named escalation (a *stage* recurrence the incident keys under-count)

`figure-provenance` was the **failing stage on 12 of the window's 13 red verify runs**, while its
incident key reads only 2. The mechanism is single and known:
`denominator_gate.find_provenance_violations` (`scripts/denominator_gate.py:413`) accepts a
re-derive command only when it is reachable **on the same line** as the figure, so a markdown
line-wrap reads as an unsourced figure. Twice the "violation" was a false positive on a figure
that did carry its command. **Escalating to the operator, not deferring:** the choice is
(a) widen the parser to follow a wrapped continuation line, or (b) leave the same-line rule as a
deliberate authoring constraint and add an author-side check that fails before commit rather than
90 minutes into a wrap-up gate. Both are outside a read-only worker's write scope and outside any
Epic 3 criterion's file-touch set; the stage is **green at this HEAD**, so this is a durability
request, not a blocker on Epic 3's closure. Candidate owner: an Epic 5 residue criterion or an
Epic 7 closure cycle.

### Cycles whose `rust_lines_changed / units_closed` exceeded 3.0 (`decisions.md §4`)

**None.** Read from
`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/rate-ledger.json`
(re-summed with
`python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['rust_lines_changed'] for x in c))"` → `5 1404 535`):

| Cycle | units_closed | rust_lines_changed | ratio |
|---|---|---|---|
| AT-35-E3-001_cycle1 | 0 | 0 | null (measurement only; blocked-escalated under the floor) |
| AT-35-E3-001_cycle2 | 618 | 232 | **0.38** |
| AT-35-E3-002_cycle1 | 786 | 266 | **0.34** |
| AT-35-E3-003_cycle1 | 0 | 37 | null (division by zero, not 0.0) |
| AT-35-E3-004_cycle1 | 0 | 0 | null |
| **Epic total** | **1404** | **535** | **0.38** |

The two ratio-bearing cycles are each an order of magnitude under the 3.0 line: 535 Rust lines
closed 1,404 units. The 37 lines in `AT-35-E3-003_cycle1` are entirely `#[cfg(test)]` plus one
comment (`git diff --stat` = 37 insertions, 0 deletions; the guarded suffix list is byte-identical),
so they bought a **control test** that keeps a half-wrong carried one-liner out — not a behaviour
change, which is why that cycle closed zero units and correctly reports `ratio: null`.

`builds_recorded` totals 7 against `decisions.md §3`'s one-build-per-cycle target. Two cycles
recorded 3 each; the ledger's own `note` fields attribute both to the same shape — the converter
build, the stamp-guard prerequisite build and the test build are **sequential prerequisites of one
another**, not three verification passes. That is a real overrun against the letter of §3 and is
already named in the ledger; it did not produce a second full gate.

## 2. Worktree sweep (this epic's worktrees only)

`df -h /` at gate end: **929 G used of 1.5 T total, 524 G available (64% used)** — no pressure.

`git worktree list --porcelain` shows the root checkout plus 10 agent worktrees. Every one of the
nine non-mine branches is **fully merged**: `git rev-list --count origin/tranche/15..<branch>` = **0**
for all of `worktree-wf_291be5c8-5f3-{14,15,17,2,21,3,4,5,7}`. Only
`wf_291be5c8-5f3-24` (this worker's) carries a `locked` line.

Epic 3's own worktrees are `wf_291be5c8-5f3-17` (`9995efa1b6`, the `AT-35-E3-002` /
`AT-35-E3-003` base) and `wf_291be5c8-5f3-21` (`07e29075b4`, the `AT-35-E3-WRAPUP` base). Both
are merged, unlocked and prunable.

**Not pruned by this worker, deliberately.** Two reasons, both rules rather than caution:
(a) a worktree-isolated agent's git writes are confined to its own worktree — `git worktree remove`
against the parent checkout is refused by the harness; and (b) an unlocked worktree is not proof
of an idle agent, and removing a directory another live cycle holds destroys its work. The prune
is handed to the orchestrator as a named, verified-merged list:

```
git worktree remove .claude/worktrees/wf_291be5c8-5f3-17 && git branch -d worktree-wf_291be5c8-5f3-17
git worktree remove .claude/worktrees/wf_291be5c8-5f3-21 && git branch -d worktree-wf_291be5c8-5f3-21
```

(`git branch -d`, never `-D`: it refuses if the merge premise above ever stops holding.)
`wf_291be5c8-5f3-24` must be left alone until this worker's turn ends.

## 3. PR

None. `workflow-instruction.md §10` step 3: no PR at an epic wrap-up.

## Verdict

**Epic 3 — Place and surface: gate GREEN, 48/48, at `cdcfc897ea`.** No red stage, no carve-out,
no unattributed failure. The corpus stands at DONE 49,438 of 49,438.
`reachability-audit` reports a reachable ceiling of 100.00% of 49,438 units, 0 dead-end cells;
`shape-engine-boundary` `not_held_by_engine=0`, `token-coverage`
`refused_non_done=0`, and the PCGen live surface unmoved at its 260-file baseline. One durability
escalation (`figure-provenance` same-line parsing) is named above for an operator ruling; it does
not gate Epic 3.

## Figures + their re-derive commands

- 48 of 48 stages PASS, wall 5,616 s — `bash scripts/verify.sh` (log `/tmp/claude-1000/gate/verify-regate.log`, `RESULT: PASS`)
- live_files=260 live_hits=12736 verdict=PASS — `python3 scripts/pcgen_residue_gate.py --check`
- 175 events / 49 corrections / 18 incidents / 14 deferrals / 2 rework — `python3 scripts/retro.py summary --since 2026-09-08 --json`
- `disk-full` 6 firings, top and only key at 3+ — `python3 scripts/retro.py summary --since 2026-09-08 --json` field `incidents.by_recurrence_key`
- `figure-provenance` 12 of 13 failed runs of 78 — `python3 scripts/retro.py summary --since 2026-09-08 --json` field `verification.by_failing_stage`
- 5 cycles, 1,404 units closed, 535 Rust lines, epic ratio 0.38, max cycle ratio 0.38 — `python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-3-place-and-surface/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['rust_lines_changed'] for x in c))"`
- 0 unmerged commits on all nine non-mine worktree branches — `git rev-list --count origin/tranche/15..worktree-wf_291be5c8-5f3-<n>` for n in 14 15 17 2 21 3 4 5 7
- 929 G used of 1.5 T, 64%, 524 G available — `df -h /`
