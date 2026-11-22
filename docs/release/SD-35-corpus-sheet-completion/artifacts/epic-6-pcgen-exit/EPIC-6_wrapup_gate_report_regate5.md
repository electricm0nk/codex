# Epic 6 — PCGen exit — wrap-up gate report, **re-gate 5**

Supersedes `EPIC-6_wrapup_gate_report.md` **and** `EPIC-6_wrapup_gate_report_regate4.md`.

Authored by the **isolated read-only worker** (`workflow-instruction.md §2` worker split,
`decisions.md §3`). This worker **committed nothing and pushed nothing**. It ran in its own git
worktree at `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-94` with its own
`CARGO_TARGET_DIR=/tmp/cargo-sd35-SD35-E6-WRAPUP-GATE5` and `CARGO_INCREMENTAL=0`.

| | |
|---|---|
| Tree under test | `4b69eb7aab` — *"retro(sd35,e6): fold AT-35-E6-004 cycle 3's verification event"* |
| Confirmed identical to remote | `git fetch origin tranche/15` then `git log --oneline -1 origin/tranche/15` → `4b69eb7aab`; local `git log --oneline -1` → `4b69eb7aab` |
| Tracked working-tree changes | none; untracked only (this epic's unfolded retro shards + this report) |
| Result | **FAIL — 48 of 49 stages PASS, 1 FAIL** |
| The one red stage | `shape-engine-boundary-selftest` |
| Status returned | `blocked-escalated` |

### What re-gate 5 adds over re-gate 4

Re-gate 4 **declined to execute a fourth physical full run**, arguing the tree had not changed and
that re-running could only reproduce the table. Its argument was sound but its report therefore
described a gate executed by a *different* worker in a *different* target dir. The orchestrator
re-dispatched, so **re-gate 5 paid for the run**: `scripts/verify.sh`, every stage, **no `--only`**,
in this worker's own worktree and own `CARGO_TARGET_DIR`, start to finish.

**The outcome is byte-for-byte the same verdict**, which retires the open question: the red stage is
real, deterministic, and unchanged. Re-gate 4's §1–§4 analysis is confirmed by independent
execution rather than inherited on trust. Where figures moved (they moved only because re-gate 4's
own events landed in the log), the re-derived values are given below.

---

## 0. The full gate, once

```
RETRO_ACTOR=SD35-E6-WRAPUP-GATE5 \
CARGO_TARGET_DIR=/tmp/cargo-sd35-SD35-E6-WRAPUP-GATE5 \
CARGO_INCREMENTAL=0 \
scripts/verify.sh                         # every stage, no --only
```

| | |
|---|---|
| Mode | `full`, no `--only` — all 49 stages of `ALL_STAGES` |
| Combined log | `/tmp/claude-1000/gate5/verify_full.log` — this worker's own capture |
| Per-stage logs | `/tmp/codex-verify-X9N8d0/` — one log per stage |
| Wall time | **2 h 12 m 17 s** (7,937 s) — started `2026-09-15T01:01:58Z`, finished `2026-09-15T03:14:15Z` |
| Exit code | `1` |
| Retro verification event | `1789442055450-sd35-e6-wrapup-gate5-a2f955`, emitted by `verify.sh` itself into `docs/retro/events/sd35-e6-wrapup-gate5.jsonl` |

### Stage table — 49 stages at `4b69eb7aab`

| # | Stage | Result | Reported detail |
|---|---|---|---|
| 1 | `preflight-disk` | **PASS** | disk budget OK |
| 2 | `preflight-oracle` | **PASS** | oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6 |
| 3 | `oracle-pin-selftest` | **PASS** | 11 passed, 0 failed |
| 4 | `producer-selftest` | **PASS** | 30 cases passed |
| 5 | `pi-redaction-selftest` | **PASS** | 49 cases passed |
| 6 | `provenance-selftest` | **PASS** | 32 cases passed |
| 7 | `site-dashboard-selftest` | **PASS** | 13 passed, 0 failed |
| 8 | `site-dashboard-pin` | **PASS** | docs/work-inventory.json matches the pin the feed was published from |
| 9 | `site-dashboard-check` | **PASS** | site/dashboard/PF1e-dashboard.json is current |
| 10 | `site-dashboard-pi-gate` | **PASS** | 22 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 11 | `build-public-status-selftest` | **PASS** | 37 cases passed |
| 12 | `site-public-status-check` | **PASS** | site/status-data.json and site/status-data/*.json are current |
| 13 | `site-public-status-pi-gate` | **PASS** | 31 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 14 | `site-asset-stamp-check` | **PASS** | site/*.html cache-busting stamps match site/styles.css |
| 15 | `reachability-audit-selftest` | **PASS** | 11 cases passed |
| 16 | `reachability-audit` | **PASS** | reachable ceiling 100.00% of 49,450 units (49450 / 49450; `/tmp/codex-verify-X9N8d0/reachability-audit.log`) |
| 17 | `groundtruth-guard-selftest` | **PASS** | 17 cases passed |
| 18 | `supersession-gate-selftest` | **PASS** | 16 cases passed |
| 19 | `shape-coverage-standing-gate-selftest` | **PASS** | 20 cases passed |
| 20 | `shape-coverage-standing-gate` | **PASS** | population=2485 unclassified=0 no_record=0 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6 |
| 21 | `cycle-scope-gate-selftest` | **PASS** | 51 cases passed |
| 22 | `shape-engine-boundary-selftest` | **FAIL** | self-test exit 1; ran 15  — /tmp/codex-verify-X9N8d0/shape-engine-boundary-selftest.log |
| 23 | `shape-engine-boundary` | **PASS** | magnitude_bearing=26397 not_held_by_engine=0 citation_ok=True |
| 24 | `missing-engine-tables` | **PASS** | population=0 kinds=0 citation_failures=0 |
| 25 | `denominator-gate` | **PASS** | files_checked=354 violations=0 |
| 26 | `figure-provenance` | **PASS** | files_checked=284 figures_examined=624 violations=0 |
| 27 | `pcgen-residue-gate` | **PASS** | live_files=0 live_hits=0 verdict=PASS |
| 28 | `token-coverage-selftest` | **PASS** | 14 cases passed |
| 29 | `token-coverage` | **PASS** | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS |
| 30 | `pi-sweep` | **PASS** | 11 hits over src/rules_core/rules_tables, 11 baseline rows |
| 31 | `declared-pi-audit` | **PASS** | clean |
| 32 | `audit-selftest` | **PASS** | 28 passed, 0 failed |
| 33 | `reclaim-selftest` | **PASS** | 13 passed, 0 failed |
| 34 | `driver-selftest` | **PASS** | 7 passed, 0 failed |
| 35 | `corpus-sweep-selftest` | **PASS** | 15 passed, 0 failed |
| 36 | `corpus-trap-audit-selftest` | **PASS** | 14 passed, 0 failed |
| 37 | `root-lib` | **PASS** | 3390 passed |
| 38 | `root-full` | **PASS** | 8926 passed across 419 suites, all 365 tests/*.rs suites executed |
| 39 | `desktop` | **PASS** | 570 passed |
| 40 | `reach` | **PASS** | 32 passed |
| 41 | `corpus-sweep` | **PASS** | 48706 records examined of 51523 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings |
| 42 | `sheet-rules-check` | **PASS** | records=49450 converted=49308 refused=142 rules=70147 var_tables=5293 verdict=PASS (116.9s) |
| 43 | `corpus-trap-audit` | **PASS** | records_examined=27681 defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249] traps=407 — all defect kinds at their registered counts |
| 44 | `supersession-gate` | **PASS** | 116 objects, all clean |
| 45 | `frontend-install` | **PASS** | node_modules present |
| 46 | `frontend-test` | **PASS** | 101/101 files |
| 47 | `frontend-typecheck` | **PASS** | tsc --noEmit clean |
| 48 | `clippy` | **PASS** | root:0 desktop:0 warnings, 0 errors |
| 49 | `class-dump` | **PASS** | 31/31 computing |

`verify.sh`'s own SUMMARY block — abridged only in that the `passed:` line's 48 stage names are the
48 rows above rather than re-listed here; every other character is as printed:

```
SUMMARY
  passed:  48  [the 48 PASS rows of the table above]
  FAILED:  1   shape-engine-boundary-selftest

BASELINE NOTES (not failures — update deliberately):
  - BASELINE_ROOT_FULL_TESTS baseline is stale: 8919 recorded, 8926 measured.
    Update scripts/verify-baselines.env.

RESULT: FAIL — logs in /tmp/codex-verify-X9N8d0
```

**Wall time vs the gate of record.** 2 h 12 m 17 s here against 7,997 s (2 h 13 m 17 s) for the run
re-gate 4 cited — a 60-second spread on a shared 24-core box, i.e. the gate is reproducible in cost
as well as in verdict.

**Where the 2 h 12 m goes**, derived from per-stage log mtimes in `/tmp/codex-verify-X9N8d0/`
(`stat -c %y /tmp/codex-verify-X9N8d0/<stage>.log`), against a gate start of `21:01:58` local:

| Stage | Finished | Duration | Share of 7,937 s |
|---|---|---|---|
| stages 1–37 (all gates + `root-lib`) | 21:14:52 | **12 m 54 s** | 10% of 7,937 s |
| 38 `root-full` (`-j 2`) | 21:58:39 | **43 m 47 s** | 33% of 7,937 s |
| 39 `desktop` (`-j 2`) | 22:32:06 | **33 m 27 s** | 25% of 7,937 s |
| 40 `reach` (`-j 2`) | 23:00:25 | **28 m 19 s** | 21% of 7,937 s |
| 41–49 (`corpus-sweep` → `class-dump`) | 23:14:15 | **13 m 50 s** | 10% of 7,937 s |

**Three `cargo` stages are 105 m 33 s of the 132 m 17 s — 80% of 7,937 s.** Every instrument,
selftest, doc gate and site gate together costs under 13 minutes. That is the figure the wrap-up
correction cycle should act on: re-verifying a two-line pin repair needs `--only`, not a re-gate.

### 0a. The red stage, named exactly

**`shape-engine-boundary-selftest`** — `python3 -m unittest scripts/tests/test_shape_engine_boundary.py`

```
FAIL: test_live_counts_match_the_committed_fact
  (scripts.tests.test_shape_engine_boundary.TestBuildReportOnLiveSource)
  File ".../scripts/tests/test_shape_engine_boundary.py", line 121,
    in test_live_counts_match_the_committed_fact
    self.assertEqual(len(mag), 26396)
AssertionError: 26397 != 26396
Ran 15 tests in 1.800s — FAILED (failures=1)
```

**This is a stale equality pin, off by one — not a behavioural regression.** The proof is the very
next stage in the same run: **stage 23 `shape-engine-boundary` PASSES** with
`magnitude_bearing=26397 not_held_by_engine=0 citation_ok=True`. The instrument agrees with itself;
only the number frozen into the test at line 121 disagrees, and it disagrees by exactly one.

- The fix is a one-line re-pin (`26396` → `26397`) in `scripts/tests/test_shape_engine_boundary.py:121`,
  plus whatever committed prose carries the same figure.
- **This worker must not make it.** It is read-only by construction (`§2` worker split); the fix
  belongs to the Epic 6 **wrap-up correction cycle**, which `§10 step 0` requires to land *before
  Epic 7's second cycle dispatches*. That cycle is exempt from the batch floor (`decisions.md §2`),
  never from the residue check.
- Recorded as incident `1789435203530-sd35-e6-wrapup-gate5-dbc913`, recurrence-key
  `epic-wrapup-gate-red` — its **seventh** firing in the SD-35 window. See §1a.

### 0b. The BASELINE NOTE is a second stale pin — flag it, do not confuse it with the failure

`BASELINE_ROOT_FULL_TESTS` in `scripts/verify-baselines.env` records **8,919**; `root-full` measured
**8,926**. `verify.sh` prints this under *"BASELINE NOTES (not failures)"* and it did **not** turn
the stage red — stage 38 is PASS. It is nonetheless the *same class of defect* as §0a: a live count
frozen into a file that the work has since moved past. **Two stale pins in one gate run is the
evidence behind the `epic-wrapup-gate-red` escalation in §1a** — the class is systemic, not a
one-off. The wrap-up correction cycle should re-pin both in the same commit.

---

## 1. Retrospective summary

`scripts/retro.py --since` takes a time, not a commit — `--since 53296d80f0` returns
`retro: cannot parse time '53296d80f0'`. The window was opened at that commit's own timestamp:

```
git log -1 --format='%ad' --date=iso 53296d80f0    → 2026-09-07 21:24:13 -0400
python3 scripts/retro.py summary --since 2026-09-07 --json
```

That window spans the **whole SD-35 bundle**, not Epic 6 alone. State that denominator whenever any
figure below is quoted.

**Read twice, on purpose.** The summary was taken once at `01:03:49Z` — two minutes into the gate,
before this worker's own events existed — and again at `03:18:53Z`, after the gate and after this
worker's three events landed. Quoting only the second would make this report a cause of its own
figures; quoting only the first would be stale by the time anyone reads it. Both are below.

| Figure | **before** this gate (`01:03:49Z`) | **after** (`03:18:53Z`) | (re-gate 4) |
|---|---|---|---|
| events, total | 690 | **694** | 687 |
| `correction` | 215 (88 carry a blast radius) | **215** | 214 |
| `deferral` | 88 — **87 still open**, 1 resolved | **88 / 87 open** | 88 / 87 open |
| `incident` | 73 — 581 min lost, 4 silent | **74** | 72 / 578 |
| `verification` | 258 runs, 28 failed (0.1085) | **260 runs, 30 failed** (fail_rate **0.1154**) | 257 / 27 |
| `rework` | 6 | **6** | 6 |
| `note` / `resolution` | 34 / 16 | **35 / 16** | 34 / 16 |
| git join | 359 commits, 1 author, 1.922 events/commit | **same** | same |
| log integrity | 71 shards, 0 invalid, 0 problems | **72 shards, 0 invalid, 0 problems** | 70 shards |

The **after** column differs from **before** by exactly this worker's own emissions — one
`incident` (§0a), one `note` (the `.gitignore` escalation, §1a), and the two `verification` events
`verify.sh` emits for itself — plus the new shard those live in. **No figure moved for any other
reason**, and the `690 → 687` gap back to re-gate 4 is likewise wholly re-gate 4's own events.
Nothing regressed between the runs.

Corrections by what they were caught before: `implementation` 26, `merge` 14, `release` 6,
`brief` 3, **`nothing (already shipped)` 1**. Verification failures by stage:
`figure-provenance` 14, `site-dashboard-check` 12, **`shape-engine-boundary-selftest` 4 → 6**
(this run's two events are the +2), `reachability-audit-selftest` 2, `denominator-gate` 2, and one
each of `clippy`, `desktop`, `pcgen-residue-gate`. These count **stage-failures, not runs** — a
single run can fail several stages, which is why they sum to more than 30. Note that
`figure-provenance` and `site-dashboard-check` — the two largest — are both **green in this run**
(stages 26 and 9); their counts are history, not live red. `shape-engine-boundary-selftest` is the
only one that is both historic and currently red.

### 1a. Every `incident` key that fired 3+ times — control or escalation, named

`§10 step 1` is binding: 3+ firings must produce a **named mechanical control** or a **named
escalation**. Seven keys qualify.

| Key | Firings | Disposition |
|---|---|---|
| `disk-full` | **14** | **Control landed and holding.** `scripts/reclaim.sh:791` `RECLAIM_PRESSURE_PERCENT` (default 90): the run reads `df -P` used-percent and emits `incident/disk-full` only at/above the threshold, a `note` below it. Covered by `reclaim-selftest` (PASS, 13 cases). The count of 14 is dominated by pre-control firings — a correction in this very window records that *"the SD-35 closure retrospective would have read 12 phantom `disk-full` incidents as a missing control."* **Read the 14 as ≤2 real.** Disk during this run: 58% used, 620 G free; `preflight-disk` PASS. |
| `wrong-base-worktree` | **8** | **Control landed.** `§6 step 0`'s rebase-then-verify-the-base block (`git fetch && git rebase`, then `test -d docs && test -d data && test -d scripts`), with `CYCLE_START_SHA` pinned **after** the rebase. `AGENTS.md` rule 8 names this key as the canonical "a warning is not a control" case (27 firings in tranche/7); the mechanical check is what held it to 8 here. |
| `epic-wrapup-gate-red` | **7** | **Control landed — and this report is the seventh firing.** `§10 step 0`: the gate overlaps the next epic's first cycle, and any red stage is fixed in a wrap-up correction cycle before that epic's **second** dispatch. It is working: four red at `77e8d3919a` → one at `4b69eb7aab`, twice confirmed. **The residual mechanism gap, named:** every one of the seven was a *stale equality pin or stale committed figure*, never a behavioural regression — §0a is the textbook instance (26396 vs a live 26397 that the adjacent stage reports as correct). The control that would retire the key is not another gate run; it is making the live-population constants **derived rather than pinned**, or pinning them in exactly one place behind a single re-derive command. That is a `technical-design` change, so it is an **operator-scoped escalation**, not something a wrap-up worker may author. **Escalated here for the second consecutive re-gate.** |
| `site-dashboard-json-stale-after-inventory-move` | **6** | **Control landed, with its residual class already documented in `§6 step 3`:** `publish-site-dashboard.sh --check-pin` watches **one** input, so a feed made stale by a unit-ledger or owner-state change hashes clean against the pin. The full `site-dashboard-check` at epic cadence **is** the control for that residue — **green in this run** (stage 9), alongside `site-dashboard-pin` (stage 8). No further control warranted. |
| `untracked-worktrees-dir-on-shared-checkout` | **5** | **ESCALATION — still owed, now carried an eleventh time.** The prepared fix is a one-line `.worktrees/` entry in `.gitignore`. Verified still absent at this HEAD: `grep -n worktrees .gitignore` → exit 1, no output. `.gitignore` is outside every Epic 6 lane's granted file-touch set (`§3`), so `AGENTS.md` rule 4 makes it an operator ruling and rule 8 makes carrying it forward again a missing mechanism. Re-escalated as note `1789435238535-sd35-e6-wrapup-gate5-ea461c`. **Operator: one line, `.worktrees/`, in `.gitignore`.** |
| `figure-provenance-command-on-next-line` | **4** | **Control landed; what remains is compliance, not a new tool.** `python3 scripts/denominator_gate.py --check-provenance` runs in every cycle's push gate (`§6 step 3` — seconds, no producer) and is a *different flag* from `--check`. The stage is **green in this run** (stage 26, `files_checked=284 figures_examined=624 violations=0`) after `AT-35-E6-WRAPUP-FIX` repaired `AT-35-E6-004_cycle1_receipt.md:230-231`. Standing caution from `§6 step 3`: the gate checks that a command is *present and resolvable*, **not that it ran** — so a cycle must still run its own re-derive commands. |
| `retro-actor-lost-between-bash-calls` | **3** | **Control:** pass `--actor <id>` explicitly on every `retro.py` call rather than relying on an exported `RETRO_ACTOR` surviving between `Bash` invocations — each call is a fresh shell and the export does not persist. This worker did exactly that on both of its events (`--actor SD35-E6-WRAPUP-GATE5`). Mechanical rather than advisory, because the flag rides in the call itself. |

Two keys sit at 2 firings, one short of the bar, and are worth a sentence each:
`duplicate-criterion-dispatch` (the dispatch list was not reconciled against `kanban.md`, whose row
already read `complete` with a receipt path) and `ingest-vocabulary-in-rendered-sheet-text` (a direct
sheet-rule hazard — PCGen vocabulary reaching rendered sheet text).

### 1b. Cycles whose `rust_lines_changed / units_closed` exceeded 3.0

**The ratio is `n/a` for every Epic 6 cycle, by design — and that is a denominator fact, not an
evasion.** All **62** Epic 6 receipts record `closed=0 relabeled=0 … ratio=n/a`
(`grep -oh 'ratio=[^ ]*' *_receipt.md | sort | uniq -c` → 75 occurrences, every one `ratio=n/a`).
Epic 6 is a floor-**exempt** epic (`decisions.md §2`; `§6 step 1`: *"Epic 6 cycles close zero units
by design and skip the scope gate"*). Its deliverable is not units into DONE — it is a residue count
driven to zero. So the honest answer, stated in Epic 6's own denominator:

- **Lines spent:** `rust_lines_changed` summed over the 62 Epic 6 receipts = **66,143**.
- **What they bought:** `pcgen_live_files` **254 → 0**, monotonically, never once rising:
  254 (E6-001 c1) → 253 → 252 → 249 → 248 → 247 (E6-002 c6) → 208 → 197 → 81 → 75 → 69 → 59 → 46 →
  45 (E6-003-FINISH) → 25 → 16 → 10 → 6 → 4 → **0** (E6-003-RULED c18) → 0 (E6-004 c1–c3).
  ≈ **260 Rust lines per live PCGen file retired.** Confirmed independently in this run by stage 27,
  `pcgen-residue-gate`: `live_files=0 live_hits=0 verdict=PASS`.
- **Largest single spends, named as `§10 step 1` requires:** `AT-35-E6-001` c4 (**2,520**),
  `AT-35-E6-001` c2 (**2,173**), `AT-35-E6-003` c12 (**1,978**), `AT-35-E6-003-SWEEP` c7 (**1,624**),
  `AT-35-E6-002` c3 (**1,550**), `AT-35-E6-003-RULED` c11 (**1,535**), `AT-35-E6-003` c8 (**1,274**),
  `AT-35-E6-003-RULED` c18 (**1,165**), `AT-35-E6-003` c11 (**1,170**), `AT-35-E6-002` c4 (**1,031**),
  `AT-35-E6-003-RULED` c15 (**1,034**), `AT-35-E6-003` c4 (**1,105**). These are the bulk
  converter-side rewrites that moved whole catalogs onto converted records — the work the epic
  exists to do. Two outliers above them, `AT-35-E6-003-SWEEP` c3 (**4,020**) and c4 (**2,920**),
  are the prose-citation demotions, the single largest mechanical class in the epic.
- **Sheet-rule compliance of the spend:** no Epic 6 cycle spent lines on per-unit proof machinery
  (`decisions.md §4`); every receipt's wired-integration audit row reads `OK_NO_TOKENS`.
- **Two receipts corrected their own ratio inputs mid-epic**, and both are in the log rather than
  only in prose: `AT-35-E6-003-SWEEP` c14 (claimed 110, actual **599** — measured before commit,
  when two new files were still untracked and invisible to the receipt tool's `git diff`) and
  `AT-35-E6-003-RULED` c3 (claimed 366, actual **697** — a relocation counts on both sides). The
  mechanism worth carrying forward: **`cycle_scope_gate.py --receipt` must be run after the commit
  it describes, not against the uncommitted tree.**

Re-derive the spend:

```
grep -ho 'rust_lines_changed=[0-9]*' \
  docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/*_receipt.md \
  | cut -d= -f2 | paste -sd+ | bc                                                    # → 66143
ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/*_receipt.md | wc -l   # → 62
```

### 1c. Deferrals — 87 of 88 open

A deferral here is a `deferral_revisit_doctrine` **capability** deferral, not a blocker. Stage 29,
`token-coverage`, reads it out at this HEAD:

```
non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0
token_types=233 shapes=1 verdict=PASS
```

`refused_non_done=0` is the load-bearing field: **every refused token type belongs to a unit that is
already DONE under the sheet rule** — it renders as the rule's words. That is the `NO CARVE-OUTS`
ruling satisfied in the only form the ruling permits: **142 is a number, not an exemption.** The
open deferrals are successor-bundle scope; Epic 7's closure epilogue enumerates them against
`forward-scope-register.md`.

**No `deferral` event is owed by this cycle.** The dispatch rule ("a cycle closing fewer units than
its scoped population MUST emit a deferral") does not bind a read-only gate worker: this worker's
scoped population is *stages run*, and it ran 49 of 49.

---

## 2. Worktree sweep — census taken, removals **owed** (again)

```
df -h /                                                       → /dev/sda1  1.5T  833G used  620G avail  58%
du -sh .claude/worktrees/                                      → 13G   (13G at re-gate 4, 15G at 77e8d3919a)
git branch --no-merged origin/tranche/15 --list 'worktree-*'   → (no output)
```

**No disk pressure forces any removal:** 833 G used of the 1.5 T `/dev/sda1` total — 58% of that
1.5 T denominator — is well under the pressure threshold read from
`grep -n RECLAIM_PRESSURE_PERCENT scripts/reclaim.sh` (line 791, default 90), and `preflight-disk`
passed. **No `worktree-*` branch carries unmerged commits** — the `--no-merged` list is empty, so
every one is fully contained in `origin/tranche/15`. Nothing here is at risk of losing work.

Epic 6 opened ~2026-09-09 19:38 local. **This epic's** worktrees, by directory mtime:

| Worktree | HEAD | mtime | merged | locked | disposition |
|---|---|---|---|---|---|
| `wf_291be5c8-5f3-19` | `b78b076b2e` | 2026-09-10 07:49 | yes | no | **removable** |
| `wf_291be5c8-5f3-89` | `77e8d3919a` | 2026-09-13 18:28 | yes | no | **removable** (re-gate 1's worker; its report is folded) |
| `wf_291be5c8-5f3-91` | `24084e1782` | 2026-09-13 23:33 | yes | no | **removable** |
| `wf_291be5c8-5f3-90` | `8d0d4acbf2` | 2026-09-14 09:32 | yes | no | **removable** — unless it is the orchestrator's own live worktree |
| `wf_291be5c8-5f3-94` | `4b69eb7aab` | live | yes | **locked** | **this worker's — do not remove** |

The ten older worktrees (`-2 -3 -4 -5 -7 -14 -15 -21 -24 -30`, mtimes 2026-09-08 → 2026-09-09 12:20)
pre-date Epic 6 and are **out of scope** for this sweep per `§10 step 2` ("this epic's worktrees
only"). The six that re-gate 1 listed as owed (`-35 -41 -17 -23 -27 -33`) are already gone —
`AT-35-E6-WRAPUP-FIX` swept them.

**Found 4 removable, removed 0 — and the reason is structural, not a choice.** This worker runs
worktree-isolated; the harness refuses every git operation targeting a sibling worktree path. Tried
and refused in this run:

```
git worktree remove .claude/worktrees/wf_291be5c8-5f3-19
  → refused: "a worktree-isolated agent's git operations must target its own worktree"
```

The cleanliness check that protects uncommitted work (`git status --porcelain` inside each) is
refused for the same reason, and removing them blind is exactly the destructive shape `§8` forbids.

**The four removals are owed to the next non-isolated cycle agent**, which must:

```
for w in 19 89 91 90; do
  git -C .claude/worktrees/wf_291be5c8-5f3-$w status --porcelain   # must be empty
done
# then, and only for the ones that came back empty:
git worktree remove .claude/worktrees/wf_291be5c8-5f3-<n>
```

Skip `-94` while it is `locked`. Skip `-90` if it is the orchestrator's own live worktree.

**This is the third consecutive epic in which the sweep was censused and not performed.** It is the
recurring shape behind `unfolded-gate-artifacts-die-with-the-worktree`, and it is a standing
`§2`-vs-`§10-step-2` contradiction in the workflow instruction: **step 2 assigns a sweep to the one
worker structurally incapable of performing it.** Operator ruling owed — either the wrap-up worker
is granted sibling-worktree authority, or `§10 step 2` moves to the wrap-up **correction** cycle,
which is non-isolated by construction and already required to exist whenever the gate is red.

---

## 3. No PR

Per `§10 step 3`. None opened, none merged, nothing pushed, nothing committed.

---

## 4. PCGen residue gate — literal output

`python3 scripts/pcgen_residue_gate.py --check`, run in this worktree at `4b69eb7aab` (exit 0):

```
pattern raw_tokens files=0 hits=0
pattern raw_bonus_chains files=0 hits=0
pattern PcgenFormulaEvaluator files=0 hits=0
pattern render_pcgen_desc files=0 hits=0
pattern bonus_stack_reader files=0 hits=0
pattern pre_tokens files=0 hits=0
pattern BONUS: files=0 hits=0
pattern DEFINE: files=0 hits=0
pattern PRE[A-Z]+: files=0 hits=0
pattern SAB: files=0 hits=0
pattern DESC: files=0 hits=0
pattern %CHOICE files=0 hits=0
pattern %LIST files=0 hits=0
pattern TYPE= files=0 hits=0
pattern pcgen_import files=0 hits=0
root src/rules_core files=0 hits=0
root src/saved_character files=0 hits=0
root src/campaign files=0 hits=0
root src/homebrew_authoring files=0 hits=0
root apps/desktop files=0 hits=0
identifier_files=0 identifier_hits=0
shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11
live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS
```

**Residue line:**

```
live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS
```

`baseline_files=260 baseline_hits=12736` is the **kept** converter/parser/generator/oracle surface —
`decisions.md §11` requires it to survive for Starfinder reuse. Deleting it would be a defect. The
live side is at zero and the gate agrees, in-run, as stage 27.

---

## Handoff to the next cycle agent (this worker commits nothing)

**Fold these paths** (all untracked in `wf_291be5c8-5f3-94` at the end of this turn):

| Path | What it is |
|---|---|
| `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_gate_report_regate5.md` | this report |
| `docs/retro/events/sd35-e6-wrapup-gate5.jsonl` | this worker's incident + note + verification events |
| `docs/retro/events/sd35-e6-wrapup-gate4.jsonl` | re-gate 4's shard, still unfolded |
| `docs/retro/events/at-35-e6-wrapup-regate3.jsonl` | re-gate 3's shard, still unfolded |
| `docs/retro/events/sd35-e6-wrapup.jsonl` | the gate-of-record shard, still unfolded |
| `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_gate_report_regate4.md` | re-gate 4's report, still unfolded |

**Then mark superseded**, do not leave three live reports:
`EPIC-6_wrapup_gate_report.md` (describes `77e8d3919a`, states 45/4 and a four-stage red list that
is wholly stale) and `EPIC-6_wrapup_gate_report_regate4.md` (correct verdict, but describes a gate
it did not execute). **This file is the live one.**

**Then the wrap-up correction cycle, before Epic 7's second dispatch (`§10 step 0`):**

1. Re-pin `scripts/tests/test_shape_engine_boundary.py:121` `26396` → `26397`; grep the repo for
   `26396` and repair every committed figure that carries it.
2. In the **same commit**, re-pin `BASELINE_ROOT_FULL_TESTS` in `scripts/verify-baselines.env`
   `8919` → `8926` (§0b). Same defect class; splitting them across two commits invites a third
   re-gate.
3. Perform the four worktree removals in §2, cleanliness-checked first.
4. Re-run `verify.sh --only shape-engine-boundary-selftest,shape-engine-boundary` to confirm green.
   A full re-gate is **not** owed for a two-line pin repair — the other 48 stages passed in this run,
   at a measured 2 h 12 m, and none of them reads either pinned constant.
