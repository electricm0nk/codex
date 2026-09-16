# Epic 2 — Sheet rule — wrap-up gate, RE-GATE

`workflow-instruction.md §10` steps 0–3, run a second time after the wrap-up correction cycle
(`EPIC-2_wrapup_fix_cycle_receipt.md`, commit `e0280a8fea`) fixed the three stages the first
gate found red.

- **Run by:** the isolated read-only worker (`decisions.md §3` worker split). This worker
  **committed nothing and pushed nothing.** This report is its hand-off; the next cycle agent
  commits it.
- **Gated SHA:** `9995efa1b6` — `docs(sd35): Epic 2 wrap-up correction cycle receipt, progress
  entry and kanban row 31`, i.e. `origin/tranche/15` HEAD at the time of the run.
- **Worktree:** `.claude/worktrees/wf_291be5c8-5f3-17`, `CARGO_TARGET_DIR=/tmp/cargo-sd35-epic2-regate`,
  `CARGO_INCREMENTAL=0`, `RETRO_ACTOR=epic2-sheet-rule-regate`.
- **Verdict:** **PASS — 48 stages, 48 PASS / 0 FAIL.**
- **Wall time:** **5,367 s = 1 h 29 m 27 s** (`duration_seconds` on the `verification` event
  `1788918674163-epic2-sheet-rule-regate-549c84`; the first gate at `a542652c5e` took 1 h 39 m 21 s).
- **Command:** `RETRO_ACTOR=epic2-sheet-rule-regate CARGO_TARGET_DIR=/tmp/cargo-sd35-epic2-regate
  CARGO_INCREMENTAL=0 bash scripts/verify.sh -j 3` — every stage, no `--only`.
- **Log paths:** driver log `/tmp/cargo-sd35-epic2-regate/verify-full.log`; per-stage logs
  `/tmp/codex-verify-TsuBeY/` (both are scratch, not committed).

---

## 0. The full gate, once — stage table

Copied from the driver log's own `PASS`/`FAIL` lines, in execution order.

| # | Stage | Result | Detail as printed |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 27 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 8 passed, 0 failed |
| 8 | site-dashboard-check | PASS | `site/dashboard/PF1e-dashboard.json` is current |
| 9 | site-dashboard-pi-gate | PASS | 21 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 10 | build-public-status-selftest | PASS | 37 cases passed |
| 11 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 12 | site-public-status-pi-gate | PASS | 31 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 13 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 14 | reachability-audit-selftest | PASS | 11 cases passed |
| 15 | reachability-audit | PASS | reachable ceiling 100.00% of the 49,438-unit inventory |
| 16 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 17 | supersession-gate-selftest | PASS | 16 cases passed |
| 18 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 19 | shape-coverage-standing-gate | PASS | `population=2879 unclassified=0 no_record=0 corpus_sha=7f818006e3…` |
| 20 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 21 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 22 | shape-engine-boundary | PASS | `magnitude_bearing=26396 not_held_by_engine=2 citation_ok=True` |
| 23 | missing-engine-tables | PASS | `population=0 kinds=0 citation_failures=0` |
| 24 | denominator-gate | PASS | `files_checked=236 violations=0` |
| 25 | figure-provenance | PASS | `files_checked=166 figures_examined=198 violations=0` |
| 26 | pcgen-residue-gate | PASS | `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` |
| 27 | token-coverage-selftest | PASS | 14 cases passed |
| 28 | token-coverage | PASS | `non_done=786 tokened=781 token_less=5 refused=837 refused_non_done=5 token_types=231 shapes=2 verdict=PASS` |
| 29 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 30 | declared-pi-audit | PASS | clean |
| 31 | audit-selftest | PASS | 28 passed, 0 failed |
| 32 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 33 | driver-selftest | PASS | 7 passed, 0 failed |
| 34 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 35 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 36 | root-lib | PASS | 3220 passed |
| 37 | root-full | PASS | 8724 passed across 411 suites, all 361 `tests/*.rs` suites executed |
| 38 | desktop | PASS | 574 passed |
| 39 | reach | PASS | 32 passed |
| 40 | corpus-sweep | PASS | 48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings |
| 41 | sheet-rules-check | PASS | `records=49438 converted=48601 refused=837 rules=68648 var_tables=5268 verdict=PASS (111.0s)` |
| 42 | corpus-trap-audit | PASS | `records_examined=27634 defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249] traps=407` — all defect kinds at their registered counts |
| 43 | supersession-gate | PASS | 116 objects, all clean |
| 44 | frontend-install | PASS | `npm ci` |
| 45 | frontend-test | PASS | 101/101 files |
| 46 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 47 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 48 | class-dump | PASS | 31/31 computing |

**No red stage. No wrap-up correction cycle is owed.** The three stages the first gate found
red are each confirmed green here at the identical stage name:

| Stage red at `a542652c5e` | Result at `9995efa1b6` |
|---|---|
| `site-dashboard-check` | PASS — feed regenerated by the fix cycle |
| `reachability-audit-selftest` | PASS — the stale live-figure pin re-pinned on the property |
| `figure-provenance` | PASS — `files_checked=166 figures_examined=198 violations=0` |

### PCGen residue gate

`python3 scripts/pcgen_residue_gate.py --check`, run standalone as well as inside the gate,
literal final line:

```
live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
```

Identical to the fix cycle's receipt. `src/saved_character`, `src/campaign` and
`src/homebrew_authoring` are at `files=0 hits=0`; nothing new on the live side reads a PCGen
token.

---

## 1. Retrospective read — `retro.py summary --since 2026-09-08 --json`

Window `2026-09-08T00:00:00+00:00` → now, read at `9995efa1b6`. **152 events.**

| Type | Count |
|---|---|
| verification | 72 (12 failed runs) |
| correction | 40 |
| incident | 15 |
| deferral | 12 (all 12 open) |
| resolution | 9 |
| note | 2 |
| rework | 2 |

Failed verification runs by stage: `figure-provenance` 11, `reachability-audit-selftest` 1,
`shape-engine-boundary-selftest` 1, `site-dashboard-check` 1. Incident time lost: **111 minutes**.

### 1a. Incident keys at 3+ firings — the mechanical control owed

| Recurrence key | Firings | Control |
|---|---|---|
| `disk-full` | **6** | **Already landed, in the wrap-up correction cycle.** `scripts/reclaim.sh` was mis-keying every successful preventive `--apply` as an `incident`/`disk-full` — the key that owns tranche/7's real disk-exhaustion catastrophe — so a 4-hourly cron minted ~6 "incidents" a day out of a control working as designed. The script now emits `incident`/`disk-full` **only** when the run fires above `RECLAIM_PRESSURE_PERCENT`, and a routine below-threshold sweep is no longer an incident (`scripts/reclaim.sh` lines 806–833, covered by `test_reclaim.py`, `reclaim-selftest` PASS at 13 cases above). **All 6 firings in this window are the pre-fix shape; the key is expected to stop growing.** No escalation owed. |
| `duplicate-criterion-dispatch` | 2 | Below the 3+ bar but named by the first gate report; the control is `workflow-instruction.md §2.4`'s — rebuild the criteria list from `kanban.md` with `complete` rows excluded at *every* relaunch. One more instance makes it a 3+ key. |

Every other key fired once.

**One new incident, emitted by this worker:** `wrong-base-worktree`
(`1788918735685-at-35-e2-regate-f3a591`). This re-gate worker's isolated worktree was created at
`fe5ae6cd4a` — the tranche/14 merge, **100+ commits behind** `origin/tranche/15`, missing every
Epic 1 and Epic 2 commit including the wrap-up fix cycle it was dispatched to re-gate. A full
`verify.sh` ran ~14 minutes against that tree before the worker caught it. It was detected only
because `retro.py summary --since 2026-09-08` returned **0 events** — a figure that cannot be
true after an epic of work, so the worker cross-checked `git log HEAD` against
`git log origin/tranche/15`. Cleared by `git fetch origin tranche/15 && git reset --hard
FETCH_HEAD` inside the worker's own worktree and a relaunch. ~20 minutes lost.

**The control this key needs, named:** a worker dispatched into a fresh worktree must assert its
base before it spends a build — one line, `git merge-base --is-ancestor origin/<branch>
HEAD || (fetch && reset --hard)` — at the top of the §2.1 environment-setup block, alongside the
`CARGO_TARGET_DIR` export. `retro.py`'s own help text already lists "wrong-base worktrees hitting
four or more agents" as an observed incident shape, so this is a recurrence of a known class, not
a novel one. Recommended for `workflow-instruction.md §2.1` and carried into the SD-35
retrospective.

### 1b. `rust_lines_changed / units_closed` above 3.0 — `decisions.md §4`

Re-derived from each Epic 2 receipt's literal `cycle_scope_gate.py --receipt` row; unchanged
since the first gate report, and reproduced here so this re-gate stands alone.

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
| AT-35-E2-005 c3 / c4 / c5 | 0 | 0 | n/a |
| AT-35-E2-005-DISPOSITION c1 / c2 | 0 | 0 | n/a |
| EPIC-2-WRAPUP-FIX | 0 | 0 | n/a |

**No cycle exceeded 3.0.** The only cycle with a denominator scored **0.02**. Epic-wide
**9,475 Rust lines / 21,911 units closed = 0.43**.

`ratio=n/a` is not an exemption, so what the four Rust-changing zero-closure cycles bought,
restated:

- **AT-35-E2-001 c1 — 6,463 lines:** the `sheet_rule_convert` converter itself plus
  `data/sheet_rules/_refused.json`. Tool-side. It is the single artefact that produces all
  21,911 closures; amortised across them, 0.29 lines/unit. `decisions.md §4`'s explicitly
  *permitted* shape ("one converter mapping row per token type"), not per-unit machinery.
- **AT-35-E2-002 c1 — 1,818 lines:** `src/rules_core/sheet_rule.rs` — `render_sheet` /
  `Evaluator::line`, the live surface that turns a converted rule into a rendered sheet line.
  Without it the conversion is data nobody reads.
- **AT-35-E2-003 c1 — 442 lines:** `v06_work_inventory.rs` (+430/−6) and `companion_chassis.rs`
  (+5/−1) — the `sheet-complete` status, one generic classifier pass over the whole inventory,
  not a per-family probe.
- **AT-35-E2-004 c1 — 295 lines:** the token-coverage ledger's three inventory fields and their
  corpus-fixture gate (`sheet_rule_convert_gate.rs` +38) — one gate per *kind*, reading the live
  corpus.
- **AT-35-E2-005 c2 — 120 lines:** one live file, `src/rules_core/sheet_rule.rs` (+117/−3, of
  which +93 is the test hunk). Bought mechanically: **11 → 0 oracle-parity disagreements.**

### 1c. Open deferrals — 12, all open

Every one names its mechanism and its revisit condition. The load-bearing ones for Epic 3+:

- **AT-35-E2-005 (×5) and AT-35-E2-005-DISPOSITION (×2):** the 1,404-unit non-DONE remainder,
  of which **659 are converter refusals** across 69 refusal strings (multiplicity 851) — top
  types `FORMULA:var(COUNT)=210`, `unmapped:STARTSKILLPTS=119`, `SPELLS`(PI-redacted)`=66`,
  `FORMULA:malformed=62`. Every one needs a converter mapping row, which AT-35-E2-005's own text
  forbids. Re-scoped by the orchestrator under `decisions.md §16`; owned by **AT-35-E4-001**
  (mapping rows) and **AT-35-E4-002 / Epic 5** (the 745 non-promotable units).
- **AT-35-E3-001 (×2):** cycle 1 refused to start at `scoped=214` (under the 500 floor, and not
  the whole remainder) — handed to AT-35-E4-001. Cycle 2's residue is **2 units that join to no
  corpus record at all**, so the converter has no source row; owned by AT-35-E3-002.
- **AT-35-E1-003:** citation-string stems, outside its file-touch set.
- **AT-35-E2-001:** unmapped heads are table defects, counted per shape by AT-35-E2-004.
- **AT-35-E2-WRAPUP:** the first gate's worktree sweep, superseded by §2 below.

None of the 12 defers Epic 2 DoD scope.

---

## 2. Worktree sweep — Epic 2's worktrees only

`df -h /` before the gate: **859G of the one 1.5T filesystem `/dev/sda1` used = 60% of that
filesystem, 593G available.** After the gate's builds: **891G of the same 1.5T filesystem used
= 62% of it, 562G available.** No disk pressure; `scripts/reclaim.sh`'s threshold is not met.

`git worktree list` at start: 10 entries (the main checkout + 9 worktrees). Each sibling branch
was checked with `git rev-list --count origin/tranche/15..<branch>` — **all returned 0**, i.e.
none carries an unmerged commit. Bases identified by `git log -1` on each worktree's HEAD:

| Worktree | Base | Owning epic | Action |
|---|---|---|---|
| `wf_291be5c8-5f3-1` | `06872eff73` AT-35-E1-003 re-dispatch | Epic 1 | **removed** (clean; removed before the epic mapping was complete — out of this sweep's stated scope, but it carried 0 unmerged commits and no uncommitted files, so nothing was lost) |
| `wf_291be5c8-5f3-2` | `4e321d2c6c` AT-35-E3-001 c1 blocked-escalated | Epic 3 | left alone — not this epic |
| `wf_291be5c8-5f3-3` | `986084c5a4` AT-35-E1-001 | Epic 1 | left alone — not this epic |
| `wf_291be5c8-5f3-4` | `942c8d3ae5` AT-35-E1-002 c2 | Epic 1 | left alone — not this epic |
| `wf_291be5c8-5f3-5` | `3c43cf0531` AT-35-E1-005 | Epic 1 | left alone — not this epic |
| `wf_291be5c8-5f3-7` | `928272a444` AT-35-E1-004 c1 | Epic 1 | left alone — not this epic |
| `wf_291be5c8-5f3-14` | `a542652c5e` AT-35-E2-005-DISPOSITION c2 | **Epic 2** | **retained** — `git worktree remove` refused: *"contains modified or untracked files, use --force to delete it"*. 859M. |
| `wf_291be5c8-5f3-15` | `8cc4ea1516` AT-35-E2-005-DISPOSITION c1 | **Epic 2** | **retained** — same refusal. 858M. |
| `wf_291be5c8-5f3-17` | `9995efa1b6` | this re-gate | **locked** — never removed |

**No `--force` was used.** The standing rule protects a worktree carrying work; `--force` here
would delete uncommitted files no operator has looked at, to reclaim 1.7G on a filesystem with
562G free. Recorded as deferral `1788918748686-at-35-e2-regate-18a292`, revisit at the Epic 7
closure epilogue or at the first sign of disk pressure.

`ps -eo pid,args` confirmed no process was rooted in any sibling worktree at removal time; the
only live processes were this gate's own.

---

## 3. Pull request

**None.** `workflow-instruction.md §10` step 3: no PR at an epic wrap-up.

---

## Retrospective events emitted by this worker

Written to `docs/retro/events/at-35-e2-regate.jsonl` (this worker's hand-off; not committed by
this worker), plus one auto-emitted by `verify.sh` to
`docs/retro/events/epic2-sheet-rule-regate.jsonl`:

| id | type | what |
|---|---|---|
| `1788918674163-epic2-sheet-rule-regate-549c84` | verification | `verify.sh full: PASS`, 48 stages, `duration_seconds` 5367, head `9995efa1b6` |
| `1788918735685-at-35-e2-regate-f3a591` | incident | `wrong-base-worktree` — the re-gate worker's stale base |
| `1788918748686-at-35-e2-regate-18a292` | deferral | 2 Epic 2 worktrees retained rather than force-removed |

## Verdict

**Epic 2's wrap-up gate is green at `9995efa1b6`: 48 of 48 stages PASS, 1 h 29 m 27 s.**
`workflow-instruction.md §10` step 0's gating condition is satisfied — Epic 3's second cycle is
unblocked, and no wrap-up correction cycle is owed.
