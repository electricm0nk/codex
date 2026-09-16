# Epic 4 — Resolve and verify — wrap-up **re-gate** report

`workflow-instruction.md §10` steps 0–3, re-run on the **isolated read-only worker**
(`§2` worker split, `decisions.md §3`) after the Epic 4 wrap-up correction cycle. The worker
pushed nothing and committed nothing; this file and
`docs/retro/events/at-35-e4-regate.jsonl` are handed to the next cycle agent to commit.

- **Status:** **COMPLETE — GREEN, 48 of 48 stages PASS.** `RESULT: PASS`, `VERIFY_EXIT=0`.
- **Tree under test:** `00d0611e87` (`docs(sd35): fill the Epic 4 wrap-up correction receipt's
  docs-commit sha (20812af147)`), the tip of `origin/tranche/15` at gate start.
- **Worker worktree:** `.claude/worktrees/wf_291be5c8-5f3-30` (locked), synced with
  `git fetch origin tranche/15` then `git reset --hard origin/tranche/15` from a verified-empty
  `git status --porcelain`.
- **`CARGO_TARGET_DIR`:** `/tmp/cargo-sd35-AT-35-E4-regate` (own directory, `CARGO_INCREMENTAL=0`).
- **Command:** `scripts/verify.sh -j 6` — **every stage, no `--only`**.
- **Wall time:** **5,042 s = 1 h 24 m 02 s** (started 2026-09-09T12:21:06-04:00, ended
  2026-09-09T13:45:08-04:00). Peak RSS 4,742,816 kB; CPU 196% of the 2,400% this 24-core box can
  offer (24 cores / 143 GB, load 0.67 at start — the gate is I/O- and link-bound, not CPU-bound).
  Re-derive: `/usr/bin/time -v` block at the foot of the run log.
- **Run log:** `/tmp/sd35-e4-regate-verify-run.log` (copy of
  `/tmp/cargo-sd35-AT-35-E4-regate/verify-regate.out`); per-stage logs `/tmp/codex-verify-3Cti8W/`.

Both paths are on the worker's `/tmp` and do **not** survive worker teardown, so the full stage
table is transcribed verbatim below.

**What this re-gate settles.** The Epic 4 wrap-up gate at `5e2c0c8c5b` was **BLOCKED-ESCALATED**,
47 of 48, sole red stage `figure-provenance` (`violations=2` of `figures_examined=228`,
`files_checked=174`). The correction cycle (`1f4c0ad9fe`, receipt
`EPIC-4_wrapup_correction_cycle_receipt.md`) claimed green. **Independently re-measured here at
`00d0611e87`: `figure-provenance` is `files_checked=178 figures_examined=230 violations=0`.** The
claim holds.

---

## Step 0 — the full gate, once

### Stage table (48 stages, in run order)

| # | stage | result |
|---|---|---|
| 1 | preflight-disk | PASS — disk budget OK (898G used of the 1.5T filesystem, 62% of it, 554G available) |
| 2 | preflight-oracle | PASS — oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS — 11 passed, 0 failed |
| 4 | producer-selftest | PASS — 27 cases passed |
| 5 | pi-redaction-selftest | PASS — 49 cases passed |
| 6 | provenance-selftest | PASS — 32 cases passed |
| 7 | site-dashboard-selftest | PASS — 8 passed, 0 failed |
| 8 | site-dashboard-check | PASS — `site/dashboard/PF1e-dashboard.json` is current |
| 9 | site-dashboard-pi-gate | PASS — 21 files scanned against 1,612 declared-PI names, zero leaked |
| 10 | build-public-status-selftest | PASS — 37 cases passed |
| 11 | site-public-status-check | PASS — `site/status-data.json` and `site/status-data/*.json` are current |
| 12 | site-public-status-pi-gate | PASS — 31 files scanned against 1,612 declared-PI names, zero leaked |
| 13 | site-asset-stamp-check | PASS — `site/*.html` cache-busting stamps match `site/styles.css` |
| 14 | reachability-audit-selftest | PASS — 11 cases passed |
| 15 | reachability-audit | PASS — reachable ceiling 100.00%, i.e. 49,438 of 49,438 units (`python3 scripts/reachability_audit.py`) |
| 16 | groundtruth-guard-selftest | PASS — 17 cases passed |
| 17 | supersession-gate-selftest | PASS — 16 cases passed |
| 18 | shape-coverage-standing-gate-selftest | PASS — 20 cases passed |
| 19 | shape-coverage-standing-gate | PASS — `population=2485 unclassified=0 no_record=0 corpus_sha=7f818006e3…` |
| 20 | cycle-scope-gate-selftest | PASS — 51 cases passed |
| 21 | shape-engine-boundary-selftest | PASS — 15 cases passed |
| 22 | shape-engine-boundary | PASS — `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True` |
| 23 | missing-engine-tables | PASS — `population=0 kinds=0 citation_failures=0` |
| 24 | denominator-gate | PASS — `files_checked=248 violations=0` |
| 25 | **figure-provenance** | **PASS — `files_checked=178 figures_examined=230 violations=0`** (red at the previous gate) |
| 26 | pcgen-residue-gate | PASS — `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS` |
| 27 | token-coverage-selftest | PASS — 14 cases passed |
| 28 | token-coverage | PASS — `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS` |
| 29 | pi-sweep | PASS — 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 30 | declared-pi-audit | PASS — clean |
| 31 | audit-selftest | PASS — 28 passed, 0 failed |
| 32 | reclaim-selftest | PASS — 13 passed, 0 failed |
| 33 | driver-selftest | PASS — 7 passed, 0 failed |
| 34 | corpus-sweep-selftest | PASS — 15 passed, 0 failed |
| 35 | corpus-trap-audit-selftest | PASS — 14 passed, 0 failed |
| 36 | root-lib | PASS — 3,220 passed |
| 37 | root-full | PASS — **8,730 passed across 412 suites, all 361 `tests/*.rs` suites executed** |
| 38 | desktop | PASS — 574 passed |
| 39 | reach | PASS — 32 passed |
| 40 | corpus-sweep | PASS — 48,706 records examined of 51,476 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, **0 findings** |
| 41 | sheet-rules-check | PASS — `records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS` (113.7 s) |
| 42 | corpus-trap-audit | PASS — `records_examined=27634`, defects `[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249]`, `traps=407` — all defect kinds at their registered counts |
| 43 | supersession-gate | PASS — 116 objects, all clean |
| 44 | frontend-install | PASS — `npm ci` |
| 45 | frontend-test | PASS — 101/101 files |
| 46 | frontend-typecheck | PASS — `tsc --noEmit` clean |
| 47 | clippy | PASS — root:0 desktop:0 warnings, 0 errors |
| 48 | class-dump | PASS — 31/31 computing |

`verify.sh` summary line: `passed:  48 …` / `RESULT: PASS`. **No red stage. Nothing is owed to a
correction cycle.**

**Baselines held exactly, no drift:** `BASELINE_ROOT_FULL_TESTS=8730` (measured 8,730) and
`BASELINE_ROOT_TEST_BINARIES=412` (measured 412 suites) — both were raised by the Epic 4 wrap-up
correction cycle and by `AT-35-E5-001`, and this independent run measures the raised values with
no stale-baseline warning. Re-derive: `grep -nE '^BASELINE_ROOT_(FULL_TESTS|TEST_BINARIES)='
scripts/verify-baselines.env | tail -2` → `3590:BASELINE_ROOT_FULL_TESTS=8730`,
`3624:BASELINE_ROOT_TEST_BINARIES=412`.

### PCGen residue gate (`decisions.md §11`)

Run separately as well as inside the gate, at `00d0611e87`:

```
python3 scripts/pcgen_residue_gate.py --check   # EXIT=0
live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS
```

Unchanged from the `AT-35-E1-005` baseline of 260 files / 12,736 hits. Nothing new on the live
side reads a PCGen token; the converter, parser, generators and oracle harness are intact
(`root src/rules_core files=208 hits=12242`, `src/saved_character 0`, `src/campaign 0`,
`src/homebrew_authoring 0`, `apps/desktop files=52 hits=494`).

---

## Step 1 — retrospective summary, read

`python3 scripts/retro.py summary --since 2026-09-08 --json` at `00d0611e87` (window
`2026-09-08T00:00:00+00:00` → open; 31 shards, `invalid_lines=0`, `problems=0`).

| figure | value | re-derive command |
|---|---|---|
| events in window | **196** of 196 emitted since 2026-09-08 across 32 actors | `python3 scripts/retro.py summary --since 2026-09-08 --json \| python3 -c "import json,sys;print(json.load(sys.stdin)['events']['total'])"` |
| corrections | **57** — 24 correctors, 55 distinct subjects, 25 with a blast radius recorded | `… ['corrections']['total']` |
| incidents | **20** over 9 recurrence keys, **383 minutes lost**, 0 silent | `… ['incidents']['total']`, `['incidents']['time_lost_minutes']` |
| deferrals | **15 open, 0 resolved** | `… ['deferrals']['open']` |
| rework | **2** | `… ['rework']['total']` |
| near-misses | **0** | `… ['near_misses']['total']` |
| verification runs | **83**, of which 14 failed — fail rate **16.87%** of 83 | `… ['verification']` |
| failing stages | `figure-provenance 13`, `site-dashboard-check 2`, `reachability-audit-selftest 1`, `shape-engine-boundary-selftest 1` | `… ['verification']['by_failing_stage']` |
| corrections caught before | implementation 13, merge 6, brief 1, plus 2 narrative values | `… ['corrections']['caught_before']` |

`caught_before` is the number worth reading: **13 of the window's corrections were caught before
implementation and 6 before merge** — no correction in this window escaped to a shipped surface.

### Recurring incident keys and their dispositions (§10 step 1: 3+ firings ⇒ a named control)

| recurrence key | count | disposition |
|---|---|---|
| **`disk-full`** | **6** | **CONTROL LANDED** — `scripts/reclaim.sh` now reads `df -P` used-percent and emits `incident/disk-full` only at or above `RECLAIM_PRESSURE_PERCENT` (default 90, i.e. 90% of whatever the filesystem's own size is); below it, the same run emits a `note` tagged `reclaim-routine`, recording `used_percent` either way. Landed at `e0280a8fea` (Epic 2 wrap-up correction). Re-derive: `grep -n RECLAIM_PRESSURE_PERCENT scripts/reclaim.sh` → 5 hits, lines 90, 791, 820, 823, 833. **All 6 of the 6 firings predate the fix**; the last is `2026-09-08T20:00:07Z` and none has fired since. All 6 were phantoms — cron reclaim runs on a disk around 900G used of 1.5T, none carrying `time_lost_minutes`. |
| **`figure-provenance-command-on-next-line`** | **3** (Epic 2, Epic 3, Epic 4 wrap-ups; 13 of the window's 14 failed verification runs) | **CONTROL LANDED** — root cause was not authoring sloppiness: `workflow-instruction.md §6` step 3's per-cycle gate list ran `denominator_gate.py --check`, while `verify.sh`'s `figure-provenance` stage runs the **different flag** `--check-provenance`. No cycle had ever run `--check-provenance` locally, so every instance of the shape escaped the cycle and surfaced only at the ~90-minute wrap-up gate. `§6` step 3 now runs **both**, `--check-provenance` on its own line with "nonzero exit BLOCKS the push" (landed `1f4c0ad9fe`; re-derive `grep -n 'check-provenance' docs/release/SD-35-corpus-sheet-completion/workflow-instruction.md` → lines 557, 561–566). Python-only, no build, seconds. **This re-gate is the first evidence it works: `figure-provenance` PASS.** |
| **`epic-wrapup-gate-red`** | **3** (Epic 2 at `a542652c5e`, 3 red of 48; Epic 3 at `07e29075b4`, 2 red of 48; Epic 4 at `5e2c0c8c5b`, 1 red of 48) | **DERIVATIVE — no separate control needed, and the trend is the evidence.** Every firing decomposes entirely into the two keys above plus `site-dashboard-json-stale-after-inventory-move`; each of those now has a landed control, and the red-stage count fell 3 → 2 → 1 → **0 at this re-gate**. 280 of the window's 383 lost minutes sit in these three firings (99 + 90 + 91). **Named escalation if it fires a fourth time:** the wrap-up gate is no longer catching a *class* of defect that cycles cannot catch, and §10 step 0 should be re-scoped rather than repeated. |
| `duplicate-criterion-dispatch` | 2 (below threshold) | Cause on record: the orchestrator's dispatch list was not reconciled against `kanban.md`, whose rows already read `complete` with receipt paths. Both re-dispatches (`AT-35-E1-001`, `AT-35-E1-005`) cost ~10 min and re-verified at zero. |
| `site-dashboard-json-stale-after-inventory-move` | 2 (below threshold) | Standing control is `decisions.md §L15` — the producer runs at the epic wrap-up and its `--check` is the gate there, never a per-wave red stage. Green at this re-gate. |

### Cycle efficiency (§10 step 1 / `decisions.md §4`): `rust_lines_changed / units_closed`

Source: `artifacts/epic-4-resolve-and-verify/rate-ledger.json` at `00d0611e87`.

| cycle | units_closed | rust_lines_changed | ratio | builds_recorded | pcgen_live_files |
|---|---|---|---|---|---|
| `AT-35-E4-001_cycle1` | 0 | 131 | **null** | 0 | 260 |
| `AT-35-E4-002_cycle1` | 0 | 171 | **null** | 1 | 260 |
| `AT-35-E4-003_cycle1` | 0 | 0 | **null** | 0 | 260 |
| **epic total** | **0** | **302** | **null** | **1** | 260 → 260 |

Re-derive: `python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'));print([(x['cycle'],x['units_closed'],x['rust_lines_changed'],x['ratio']) for x in c['cycles']], c['totals']['ratio_over_the_epic'])"`

**No cycle exceeded 3.0, because no ratio is defined.** Every Epic 4 ratio is `null` — a division
by zero, never a `0.0`. Epic 4's authoring-time population (M 4,334 + V 392 = 4,726 of the 23,315
then non-DONE) had already been closed by Epic 3's `AT-35-E3-001_cycle2` (618 units) and
`AT-35-E3-002_cycle1` (786 units, the whole remainder), so the scoped population at Epic 4's first
cycle start (`07e29075b4`) was **0 of 0 non-DONE of 49,438**, and every cycle's scope gate read
`PASS_WHOLE_REMAINDER`, not a floor exemption.

**What the 302 Rust lines bought, since no unit closed:**

- **`AT-35-E4-001_cycle1`, 131 lines** — 24 converter mapping rows plus one head alias
  (`GLOBALVAR:ABILITY` → the existing `ABILITY` row). Took `token-coverage.json`'s
  `unmapped_token_types` **25 → 0** and `data/sheet_rules/_report.json` `degraded_records`
  **974 → 603**, and the oracle-compared line count **42/41 → 146/145 with zero new
  disagreements**. This is the table gap behind bucket M's Evidence clause: the units were
  already DONE, but the converter was refusing token types they contained.
- **`AT-35-E4-002_cycle1`, 171 lines** — the bucket-V oracle harness
  (`scripts/oracle_harness/bucket_v_parity.py`, `src/bin/sheet_rule_bucket_v_render.rs`, and the
  carrier-character twin the comparison needs). Produced the corpus-wide verdicts bucket V's
  closure was owed: **`compared=392 oracle_agree=184 oracle_disagreement=10 oracle_unverifiable=198`**
  at `PCGEN_ORACLE_SHA=7f818006e…`, export tier 286 of 392 over 21 carriers, 0 export failures.
  All 10 disagreements are named and booked as Epic 6's parity baseline
  (deferral `1788955474431-at-35-e4-002-0f136c`) — deliberately not fixed here, because fixing
  them would move the baseline the PCGen exit is measured against.
- **`AT-35-E4-003_cycle1`, 0 lines** — the rate ledger itself; docs-only.

`builds_recorded=1` over the epic satisfies `decisions.md §3`. The two `0` rows are the counter's
reading for cycles whose cargo runs do not bump the repo's build-number file, not builds skipped —
`AT-35-E4-001_cycle1` alone paid four cargo builds and four corpus-scale runs (`v06_work_inventory`
755.9 s, `corpus_literal_sweep` 154.4 s, `sheet_rule_convert` 110.9 s, `sheet_rule_parity` 17.9 s).

---

## Step 2 — worktree sweep (Epic 4's worktrees only)

```
df -h /   →  /dev/sda1  1.5T  898G  555G  62% /
git worktree list  →  13 entries (the shared checkout + 12 sibling worktrees)
git worktree prune -v  →  no output, exit 0 (no stale admin entries)
```

**Nothing was pruned. Nothing was at risk.** Every one of the 11 sibling worktrees carries **zero
unmerged commits** — re-derive `git rev-list --count origin/tranche/15..<sha>` = 0 for each of
`a542652c5e 8cc4ea1516 9995efa1b6 4e321d2c6c 07e29075b4 cdcfc897ea 5e2c0c8c5b 986084c5a4
942c8d3ae5 3c43cf0531 928272a444`. Only this worker's own worktree is `locked`
(`find /home/ubuntu/workspace/repos/codex/.git/worktrees -maxdepth 2 -name locked` → one hit,
`wf_291be5c8-5f3-30/locked`).

Epic 4's two worktrees, and why each stayed:

| worktree | base | owner | outcome |
|---|---|---|---|
| `wf_291be5c8-5f3-24` (867 M) | `cdcfc897ea` | `AT-35-E4-002` cycle 1 | `git worktree remove` → `fatal: … contains modified or untracked files, use --force to delete it` |
| `wf_291be5c8-5f3-27` (869 M) | `5e2c0c8c5b` | Epic 4 wrap-up gate worker | same refusal |

`wf_291be5c8-5f3-21` (`07e29075b4`) is **Epic 3's** wrap-up worktree, not this epic's, and was
left alone by scope (`correction 1788937296885-at-35-e3-wrapup-13bc01` names it).

`--force` was **not** used. The harness refuses any git command targeting a sibling worktree
(`cd …/wf_291be5c8-5f3-24 && git status --porcelain` is refused, as is `git -C`), so this worker
cannot see what the dirty set contains before deleting it, and `NEVER remove a worktree carrying
unmerged commits` generalizes to `never delete content you cannot read`. Disk is nowhere near
pressure — 898G used of the 1.5T filesystem, 555G free, against `reclaim.sh`'s threshold of 90% of
that same 1.5T. Recorded as deferral
`1788975965944-at-35-e4-regate-aad5ca`.

**This is the third consecutive epic wrap-up deferring the same step for the same reason**
(Epic 2 `…cb0191`, Epic 3 `…13bc01`, Epic 4 `…aad5ca`). Per `AGENTS.md` rule 8 a repeated
explanation is not a control. **Named escalation** (`note 1788975976415-at-35-e4-regate-263119`):
move `§10` step 2 off the read-only worker — either to the wrap-up **correction** cycle, which runs
on the shared checkout with write authority, or to `scripts/reclaim.sh`, which already scans and
removes worktrees under a pressure threshold from the shared checkout. Left as-is, Epic 5 will
produce a fourth identical deferral.

---

## Step 3 — no PR

None opened. `workflow-instruction.md §10` step 3 forbids it; the bundle's PR is the Epic 7
closure epilogue (`§11`).

---

## Worker discipline

- Pushed nothing, committed nothing, opened no PR.
- One `git reset --hard origin/tranche/15` inside this worker's own worktree, from a
  verified-empty `git status --porcelain`.
- No `git stash`, no `git add -A`, no force-push, no branch deletion.
- Working tree at hand-off carries exactly one untracked file, this worker's own retro shard
  `docs/retro/events/at-35-e4-regate.jsonl` (3 events: `verification` emitted by `verify.sh`,
  `deferral …aad5ca`, `note …263119`), plus this report. Both are for the next cycle agent to
  commit.

## SELF-CHECK — this report is itself gated content

The Epic 3 wrap-up learned this the expensive way (`correction 1788942545933-at-35-e3-wrapup-fix-b6ddea`:
"the wrap-up report is itself gated content"). Both gates were therefore re-run **with this file
present and intent-to-added**, and its first draft was **red**:

| check | first draft | after the fix |
|---|---|---|
| `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` | `files_checked=62 violations=6` — 6 percentages of this file's own with no denominator on their line (run wall time, `preflight-disk`, `reachability-audit`, the retro fail rate, the `disk-full` row, the step-2 disk reading) | `files_checked=62 violations=0`, `EXIT=0` |
| `python3 scripts/denominator_gate.py --check` (default paths) | — | `files_checked=249 violations=0`, `EXIT=0` |
| `python3 scripts/denominator_gate.py --check-provenance` | — | `files_checked=179 figures_examined=230 violations=0`, `EXIT=0` |

`files_checked` rises by exactly 1 over the gate stage's own readings (`--check` 248 → 249,
`--check-provenance` 178 → 179) — this file, and nothing else, is the delta. **The next cycle agent
can commit this report without turning the package gate red.**

## Outputs to commit

1. `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/EPIC-4_regate_report.md` (this file)
2. `docs/retro/events/at-35-e4-regate.jsonl`
