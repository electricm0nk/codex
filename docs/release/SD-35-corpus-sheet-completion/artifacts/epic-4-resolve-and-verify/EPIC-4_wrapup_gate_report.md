# Epic 4 — Resolve and verify — wrap-up gate report

`workflow-instruction.md §10` steps 0–3, run on the **isolated read-only worker**
(`§2` worker split, `decisions.md §3`). The worker pushed nothing and committed nothing; this
file is handed to the next cycle agent to commit.

- **Status:** **BLOCKED-ESCALATED** — 47 of 48 stages PASS, **1 FAIL (`figure-provenance`)**.
- **Tree under test:** `5e2c0c8c5b` (`docs(sd35): AT-35-E4-003 cycle 1 progress entry, kanban row
  18 and its own commit sha`), the tip of `origin/tranche/15` at gate start.
- **Worker worktree:** `.claude/worktrees/wf_291be5c8-5f3-27` (locked), synced to
  `origin/tranche/15` with `git reset --hard FETCH_HEAD` from a verified-empty
  `git status --porcelain`.
- **`CARGO_TARGET_DIR`:** `/tmp/cargo-sd35-epic4-wrapup-gate` (own directory; 33 G at peak).
- **Command:** `scripts/verify.sh -j 3` — **every stage, no `--only`**.
- **Wall time:** **5,432 s = 90 m 32 s** (started 2026-09-09T09:09:35-04:00, ended
  2026-09-09T10:40:36-04:00). `VERIFY_EXIT=1`.
- **Run log:** `/tmp/sd35-e4-verify-run.log`; per-stage logs `/tmp/codex-verify-AYZpqZ/`.

Both paths are on the worker's `/tmp` and do **not** survive worker teardown; the failing stage's
content is transcribed verbatim below so the correction cycle needs neither.

---

## Step 0 — the full gate, once

### Stage table (48 stages, in run order)

| # | stage | result |
|---|---|---|
| 1 | preflight-disk | PASS — disk budget OK |
| 2 | preflight-oracle | PASS — oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS — 11 passed, 0 failed |
| 4 | producer-selftest | PASS — 27 cases |
| 5 | pi-redaction-selftest | PASS — 49 cases |
| 6 | provenance-selftest | PASS — 32 cases |
| 7 | site-dashboard-selftest | PASS — 8 passed, 0 failed |
| 8 | site-dashboard-check | **PASS** — `site/dashboard/PF1e-dashboard.json` is current |
| 9 | site-dashboard-pi-gate | PASS — 21 files vs 1,612 declared-PI names, zero leaked |
| 10 | build-public-status-selftest | PASS — 37 cases |
| 11 | site-public-status-check | PASS — `site/status-data.json` and `site/status-data/*.json` current |
| 12 | site-public-status-pi-gate | PASS — 31 files, zero leaked |
| 13 | site-asset-stamp-check | PASS — cache-busting stamps match |
| 14 | reachability-audit-selftest | PASS — 11 cases |
| 15 | reachability-audit | PASS — reachable ceiling 100.00% (49,438 of 49,438), `python3 scripts/reachability_audit.py` |
| 16 | groundtruth-guard-selftest | PASS — 17 cases |
| 17 | supersession-gate-selftest | PASS — 16 cases |
| 18 | shape-coverage-standing-gate-selftest | PASS — 20 cases |
| 19 | shape-coverage-standing-gate | PASS — population=2485 unclassified=0 no_record=0 |
| 20 | cycle-scope-gate-selftest | PASS — 51 cases |
| 21 | shape-engine-boundary-selftest | PASS — 15 cases |
| 22 | shape-engine-boundary | PASS — magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True |
| 23 | missing-engine-tables | PASS — population=0 kinds=0 citation_failures=0 |
| 24 | denominator-gate | PASS — files_checked=244 violations=0 |
| 25 | **figure-provenance** | **FAIL — violations=2 of figures_examined=228 (files_checked=174)** |
| 26 | pcgen-residue-gate | PASS — live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS |
| 27 | token-coverage-selftest | PASS — 14 cases |
| 28 | token-coverage | PASS — non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 |
| 29 | pi-sweep | PASS — 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 30 | declared-pi-audit | PASS — clean |
| 31 | audit-selftest | PASS — 28 passed, 0 failed |
| 32 | reclaim-selftest | PASS — 13 passed, 0 failed |
| 33 | driver-selftest | PASS — 7 passed, 0 failed |
| 34 | corpus-sweep-selftest | PASS — 15 passed, 0 failed |
| 35 | corpus-trap-audit-selftest | PASS — 14 passed, 0 failed |
| 36 | root-lib | PASS — 3,220 passed |
| 37 | root-full | PASS — **8,727 passed across 412 suites, all 361 `tests/*.rs` suites executed** |
| 38 | desktop | PASS — 574 passed |
| 39 | reach | PASS — 32 passed |
| 40 | corpus-sweep | PASS — 48,706 records examined of 51,476 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, **0 findings** |
| 41 | sheet-rules-check | PASS — records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (113.5 s) |
| 42 | corpus-trap-audit | PASS — records_examined=27634, all defect kinds at registered counts, traps=407 |
| 43 | supersession-gate | PASS — 116 objects, all clean |
| 44 | frontend-install | PASS — `npm ci` |
| 45 | frontend-test | PASS — 101/101 files |
| 46 | frontend-typecheck | PASS — `tsc --noEmit` clean |
| 47 | clippy | PASS — root:0 desktop:0 warnings, 0 errors |
| 48 | class-dump | PASS — 31/31 computing |

`verify.sh` summary line: `passed: 47 … FAILED: 1 figure-provenance` / `RESULT: FAIL`.

### The one red stage, named exactly

**`figure-provenance`** — `python3 scripts/denominator_gate.py --check-provenance`
(`scripts/verify.sh` line 1374). Literal stage log:

```
VIOLATION …/artifacts/epic-4-resolve-and-verify/AT-35-E4-003_cycle1_receipt.md:129: [unsourced] - **Figures + their re-derive commands** (denominator: **0 units non-DONE of 49,438** at Epic 4's
VIOLATION …/artifacts/epic-4-resolve-and-verify/AT-35-E4-003_cycle1_receipt.md:131: [unsourced] authoring-time population of 4,726 was already closed by Epic 3):
files_checked=174
figures_examined=228
violations=2
```

Both violations are in **one file, one sentence** — the header sentence that introduces
`AT-35-E4-003_cycle1_receipt.md`'s own figure table (lines 129–131). It carries three inline
figures (`0` units non-DONE, `49,438`, `4,726`) whose provenance is written as a cross-reference
("see the ledger's `totals.denominator`") rather than as a re-derive command on the same line,
which is the only shape `--check-provenance` accepts. **Every figure inside the table below it
passes**; the table is correctly sourced.

This is **not a code defect**. No Rust, corpus, PI, PCGen-residue, site or frontend stage is red.

### Fix for the correction cycle (exempt from the batch floor, never from the residue check)

1. Rewrite `AT-35-E4-003_cycle1_receipt.md` lines 129–131 so each figure carries a same-line
   re-derive command, e.g. move the denominator into the table as its own row with
   `python3 -c "import json;print(json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['totals']['denominator'])"`,
   or state it as an estimate in the text (`AGENTS.md` rule 9).
2. Re-run **`python3 scripts/denominator_gate.py --check-provenance`** — seconds, no build.
3. Land the mechanical control in §1 below in the same cycle.
4. Bump `BASELINE_ROOT_TEST_BINARIES` 411 → 412 (see "Baseline note").

### Baseline note (reported by `verify.sh`, not a failure)

`BASELINE_ROOT_TEST_BINARIES` is stale: **411 recorded** in `scripts/verify-baselines.env`,
**412 measured**. The `+1` is `AT-35-E4-002_cycle1`'s new tool-side binary
`src/bin/sheet_rule_bucket_v_render.rs` (`rate-ledger.json` `cycles[1].note`: 171 Rust lines, one
new binary, no existing Rust file changed). Update it deliberately, with that attribution.

---

## Step 1 — retrospective summary, read

`python3 scripts/retro.py summary --since 2026-09-07T21:24:13-04:00 --json`
(`retro.py --since` takes a time, not a sha; `53296d80f0`'s commit timestamp is
`2026-09-07T21:24:13-04:00`, from `git log -1 --format='%cI' 53296d80f0`).

| figure | value |
|---|---|
| events in window | **184** (agent 95 / derived 89) |
| corrections | **54** (22 with blast radius; caught before: implementation 13, merge 4, brief 1) |
| deferrals | **15** — **15 open, 0 resolved** |
| incidents | **17** — 0 silent, **201 minutes lost** |
| rework | **2** |
| verifications | **80 runs, 13 failed, fail_rate 0.1625** |
| commits joined | 91 (`events_per_commit` 2.022) |
| log integrity | 27 shards, **0 invalid lines, 0 problems** |

**Failing verification stages in the window:** `figure-provenance` **12**,
`site-dashboard-check` 2, `reachability-audit-selftest` 1, `shape-engine-boundary-selftest` 1.

### Incident keys that fired 3+ times → each gets a named mechanical control

Two keys reach 3 once this gate's own events are counted. Neither is answered with a caution
(`AGENTS.md` rule 8).

**1. `figure-provenance-command-on-next-line` — 2 prior + this gate = 3.**
Root cause found, and it is *not* authoring sloppiness. `workflow-instruction.md §6` step 3's
per-cycle gate block (line 556) runs `python3 scripts/denominator_gate.py --check`. The
`figure-provenance` stage runs the **different flag** `--check-provenance`
(`scripts/verify.sh` line 1374). **No cycle has ever run `--check-provenance` locally**, so every
instance of the shape escapes the cycle and surfaces only at the ~90-minute wrap-up gate — which
is exactly the observed pattern (Epic 2, Epic 3, Epic 4).

> **NAMED MECHANICAL CONTROL:** add
> `python3 scripts/denominator_gate.py --check-provenance` to `workflow-instruction.md §6`
> step 3's gate block, immediately under the existing `--check` line, nonzero exit blocking the
> cycle's push. Python only, no build, seconds. Land it in the Epic 4 wrap-up correction cycle.

**2. `epic-wrapup-gate-red` — 2 prior + this gate = 3.**
Fully absorbed by control 1: all three red wrap-up gates were `figure-provenance`
(Epic 2 and Epic 3 also carried `site-dashboard-check`, which is now fixed — see below). No
separate control is proposed; if a wrap-up gate goes red again *after* control 1 lands, that is a
distinct mechanism and gets escalated on its own evidence.

**3. `disk-full` — 5 in the window; control already exists and is verified.**
All five events are the control's own **success reports**, not failures — each reads
`reclaim.sh --apply removed N item(s) …`. The mechanism is
`/home/ubuntu/workspace/repos/codex/scripts/reclaim.sh --apply` on a **4-hourly crontab**
(`0 */4 * * *`, verified with `crontab -l | grep reclaim` → 2 lines), backed by the
`reclaim-selftest` (13 passed) and `preflight-disk` (PASS) stages. **No new control is owed.**
A prior SD-35 correction already recorded that reading these as incidents would have made the
closure retrospective report 12 phantom disk-full incidents as a missing control; that reading is
not repeated here.

Keys at 2 (below the threshold, tracked): `duplicate-criterion-dispatch`,
`site-dashboard-json-stale-after-inventory-move`.

**`site-dashboard-check` is green for the first time in three wrap-ups.** It was red at Epic 2
and Epic 3 by the same mechanism (a cycle regenerates `docs/work-inventory.json` and does not
republish the dashboard that reads it). It **PASSED** here. The Epic 3 fix cycle's control held
across Epic 4's three cycles, one of which regenerated the inventory.

### `rust_lines_changed / units_closed` — every Epic 4 cycle named

Source: `artifacts/epic-4-resolve-and-verify/rate-ledger.json`, re-summed with
`python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['units_relabeled'] for x in c),sum(x['rust_lines_changed'] for x in c),sum(x['builds_recorded'] for x in c))"`
→ `3 0 0 302 1`.

**No cycle exceeded 3.0, because no cycle has a defined ratio.** All three closed **0 units**, so
`ratio` is `null` — a division by zero, never `0.0`. `decisions.md §4`'s question ("what did the
lines buy?") is *more* pressing in that case, not less, so all three are named:

| cycle | rust lines | units closed | ratio | what the lines bought |
|---|---|---|---|---|
| `AT-35-E4-001_cycle1` | **131** | 0 | null | 24 converter mapping rows + 1 head alias (`GLOBALVAR:ABILITY` → the existing `ABILITY` row). Took `token-coverage.json`'s `unmapped_token_types` **25 → 0** and `degraded_records` **974 → 603**, and moved 127 units *inside* `sheet-complete` (126 `sheet_rule_rendered:words` → `:number`, 1 → `:dice`). Bucket M and the whole remainder were already 0 at cycle start, so the scope gate read `PASS_WHOLE_REMAINDER`, **not** a floor exemption. |
| `AT-35-E4-002_cycle1` | **171** | 0 | null | One new tool-side binary `src/bin/sheet_rule_bucket_v_render.rs`; **no existing Rust file changed**. Bought the corpus-wide oracle run bucket V's closure was owed but had never been made: `compared=392 oracle_agree=184 oracle_disagreement=10 oracle_unverifiable=198`, every disagreement named in the receipt. `export` tier ran in 264.1 s at `--jobs 3` against a projection of ≈329 s measured from the first 3 carriers — the projection discipline was followed. |
| `AT-35-E4-003_cycle1` | **0** | 0 | null | Docs only — the rate ledger itself, 3 rows, 2 transcribed from committed receipts with **0 transcription discrepancies**. `builds_recorded=0` is correct for a cycle that changed no Rust. |

**Epic totals:** 3 cycles, 0 units closed, 0 relabeled, 302 Rust lines, 1 build recorded,
`ratio_over_the_epic` **null**. `pcgen_live_files` **260 → 260** — the live-side count did not
rise (`decisions.md §11`).

**Reading (`decisions.md §4`, the non-mechanical half of lesson 29).** Epic 4 closed 0 of a
0-unit population because Epic 3 had already closed its authoring-time population (M 4,334 +
V 392 = 4,726) in `AT-35-E3-001_cycle2` (618 units) and `AT-35-E3-002_cycle1` (786 units, the
whole remainder). The 302 lines are **not** a bad ratio hidden by a zero denominator: 171 bought
the oracle evidence bucket V's closure had been asserted without, and 131 closed the last 25
unmapped compute-bearing token types. Both are mechanisms corpus-wide, per lesson 27. The honest
statement is that Epic 4's *scoping* was overtaken by Epic 3's throughput, not that its cycles
were unproductive.

**Deferrals: 15 open, 0 resolved.** This gate closes no units and refuses no tokens, so it emits
no `deferral` of its own. The standing 15 are Epic 1–3 dispositions (the largest is
`AT-35-E2-005`'s, naming its refused token types: `FORMULA:var(COUNT)`=210,
`unmapped:STARTSKILLPTS`=119, PI-redacted `SPELLS`=66, of 659 refused). **They remain the bundle's
open remainder and are Epic 7's closure business** — `workflow-instruction.md §11` step 1 forbids
closing over them.

---

## Step 2 — worktree sweep (this epic's worktrees only)

- `df -h /` at gate start: **62% used of the 1.5 T root filesystem** (556 G available); at root-full
  peak **67% used of that same 1.5 T** (490 G available). Both are point-in-time samples of a shared box taken
  by the worker with `df -h /` during the run — **estimates**, not reproducible after the fact.
  Worker `CARGO_TARGET_DIR` peaked at **33 G** (`du -sh /tmp/cargo-sd35-epic4-wrapup-gate`,
  likewise a point-in-time **estimate**; the directory was deleted at teardown per `AGENTS.md`).
- `git worktree list`: **12 entries** — the main checkout, 10 `wf_291be5c8-5f3-*` worker
  worktrees, and this worker's own.

**Epic 4's worktrees are already gone — 0 found, 0 removed.** No listed worktree's HEAD is an
Epic 4 commit; the three Epic 4 cycle commits (`9bae2cfa1f`, `2645a3c85a`, `ea9650ffc9` and their
follow-ups) are all in `origin/tranche/15` with no worktree still parked on them.

The 10 remaining worktrees are **Epic 1–3 leftovers**, outside this step's scope
(`§10` step 2 says *this epic's* worktrees only). Every one sampled is an ancestor of
`origin/tranche/15` (`git merge-base --is-ancestor <head> origin/tranche/15` → 0 for
`wf_…-14`, `-15`, `-17`, `-21`, `-24`), i.e. merged and prunable at the bundle-closure sweep
(`§11` step 3). They are **listed, not removed**: this worker is read-only, and its isolation
guard refuses cross-worktree git operations by design.

**Never removed:** `wf_291be5c8-5f3-27` — this worker's own worktree, and **`locked`**.

## Step 3 — no PR

None opened. Correct for an epic wrap-up (`§10` step 3).

---

## PCGen residue gate

`python3 scripts/pcgen_residue_gate.py --check`, literal final line:

```
live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
```

Live-side roots: `src/rules_core` 208 files / 12,242 hits; `apps/desktop` 52 / 494;
`src/saved_character`, `src/campaign`, `src/homebrew_authoring` **0 / 0**. Baseline held exactly —
the live-side count did **not** rise (`decisions.md §11`, lesson 35). The same gate ran as
verify.sh stage 26 and passed there independently.

## Retrospective events emitted by this gate

Appended to `docs/retro/events/at-35-e4-wrapup.jsonl` **in this worker's worktree**, which is
never pushed. **The next cycle agent must fold this shard** along with committing this report.

| id | type | subject |
|---|---|---|
| `1788964868079-at-35-e4-wrapup-40a533` | incident | Epic 4 wrap-up gate RED — 47/1, `figure-provenance` |
| `1788964900910-at-35-e4-wrapup-5a211d` | incident | `figure-provenance` 3rd wrap-up; `--check` vs `--check-provenance` root cause + named control |
| `1788964914235-at-35-e4-wrapup-f6cacf` | note | `BASELINE_ROOT_TEST_BINARIES` 411 → 412, attributed |

`verify.sh` emitted its own `verification` event for the run.
