---
canonical: false
owner: at-35-e2-regate2
bundle_id: SD-35
status: gate-report
date: 2026-09-10
---

# Epic 2 — Sheet rule — wrap-up gate, RE-GATE 2 (`workflow-instruction.md §10` steps 0–3)

Run by the **isolated read-only worker** (`decisions.md §3` / `workflow-instruction.md §2` worker
split). **This worker committed nothing and pushed nothing.** This report, and the retro shard
`docs/retro/events/at-35-e2-regate2.jsonl`, are its hand-off; the next cycle agent commits them.

## Headline

**BLOCKED — 49 stages, 48 PASS / 1 FAIL.** The single red stage is **`site-dashboard-check`**.
Everything Epic 2 itself owns is green, including `sheet-rules-check`, `token-coverage` and
`pcgen-residue-gate`. `workflow-instruction.md §10` step 0 therefore owes a **wrap-up correction
cycle before the next epic's second cycle dispatches** — and this time the correction is not a
republish (that was tried one commit ago and did not hold); see §0b.

| Field | Value |
|---|---|
| **Gated SHA** | `f1f547a41e7b0205605a1c502ec13f47bcec2e6a` — `docs(sd35): Epic 5 wrap-up correction receipt names its own commit`, i.e. `origin/tranche/15` tip at run time |
| **Worktree** | `.claude/worktrees/wf_291be5c8-5f3-17` |
| **Command** | `RETRO_ACTOR=epic2-sheet-rule-regate CARGO_TARGET_DIR=/tmp/cargo-sd35-epic2-regate CARGO_INCREMENTAL=0 bash scripts/verify.sh -j 3` — every stage, no `--only` |
| **Wall time** | **4,378 s = 1 h 12 m 58 s** (epoch 1789030353 → 1789034731) |
| **Driver log** | `/tmp/cargo-sd35-epic2-regate/verify-full-run2.log` |
| **Per-stage logs** | `/tmp/codex-verify-FYQLkl/` |
| **Residue line** | `live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS` |

### Two things the worker had to fix about its own environment before it could gate anything

Both are recorded as `incident` events; both are the sort of thing a "read-only" worker is
supposed to be immune to and was not.

1. **`wrong-base-worktree` — third firing of that key in this window.** The worktree was pinned
   at `9995efa1b6`, **63 commits behind** `origin/tranche/15`. Epics 4, 5 and 6 had all landed in
   the meantime, including the `AT-35-E6-001` module move and two later wrap-up correction
   cycles. Gating that SHA would have re-proved a tree nobody ships and produced a report whose
   "gated SHA" is not the branch tip. Caught **before** any build was spent, by comparing
   `git worktree list`'s main-checkout SHA against this worktree's HEAD. Cleared with
   `git fetch origin tranche/15 && git merge --ff-only FETCH_HEAD` inside this worktree only.
   *A key at three firings owes a mechanical control — see §1a.*
2. **`stale-wip-in-reused-worktree` — new key.** The worktree arrived **dirty**: 15 tracked
   `.rs` files carried an uncommitted partial codemod repointing `formula_interpreter` /
   `formula_reproduction_harness` imports from `rules_core::pilot_compute` to
   `crate::pcgen_import` — **modules that did not exist at that SHA**. Left in place, the tree
   would not compile and every cargo stage would have gone red for a reason having nothing to do
   with Epic 2. The same refactor later landed correctly as `ea1a61e672` / `60fdeeb6ec`
   (AT-35-E6-001), so this was superseded dead work, not lost work. The diff was saved before it
   was reverted (`preexisting-wip-pcgen-import-refactor.patch`, in the worker's scratchpad) and
   the 15 paths were restored with `git checkout -- <explicit paths>`. **No `git stash`, no
   `git add -A`, no force.**

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
| 7 | site-dashboard-selftest | PASS | 13 passed, 0 failed |
| 8 | site-dashboard-pin | PASS | `docs/work-inventory.json` matches the pin the feed was published from |
| 9 | **site-dashboard-check** | **FAIL** | **exit 1 — `/tmp/codex-verify-FYQLkl/site-dashboard-check.log`** |
| 10 | site-dashboard-pi-gate | PASS | 22 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 13 | site-public-status-pi-gate | PASS | 31 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 14 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling 100.00% of 49438 units (49438 / 49438) |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True |
| 24 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS | files_checked=259 violations=0 |
| 26 | figure-provenance | PASS | files_checked=189 figures_examined=334 violations=0 |
| 27 | **pcgen-residue-gate** | PASS | **live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS** |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | **token-coverage** | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS |
| 30 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | 3261 passed |
| 38 | root-full | PASS | 8772 passed across 413 suites, all 361 `tests/*.rs` suites executed |
| 39 | desktop | PASS | 576 passed |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48706 records examined of 51476 read, 413314 tokens compared, 0 findings |
| 42 | **sheet-rules-check** | PASS | records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (114.1 s) |
| 43 | corpus-trap-audit | PASS | records_examined=27634, all defect kinds at their registered counts |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | node_modules present |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 48 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 49 | class-dump | PASS | 31/31 computing |

`RESULT: FAIL — logs in /tmp/codex-verify-FYQLkl`

### 0a. The red stage, named exactly

**`site-dashboard-check`** — `timeout 2400s scripts/publish-site-dashboard.sh --check`, exit 1.
Its log is two lines:

```
site/dashboard/PF1e-dashboard.json input pin matches docs/work-inventory.json (5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f)
site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh
```

### 0b. Why the standing disposition for this key does not apply this time

This is the **fourth** firing of `site-dashboard-json-stale-after-inventory-move` in the SD-35
window, and it has a sub-mechanism the key's name does not describe:

- **`site-dashboard-pin` PASSED.** `docs/work-inventory.json` still hashes to the pin the feed
  was published from. **The dominant input did not move.** The key's name — *…-after-inventory-move*
  — is not what happened here.
- **The feed was republished one commit earlier.** `6e4b1f7b4e` is the Epic 5 wrap-up correction,
  whose own commit message says "the stale feed republished". The only commit above it at gate
  time is `f1f547a41e`, a **docs-only receipt**. A feed that is STALE with its input pin intact,
  one docs commit above its own republish, is drifting on an input the `--check` comparison does
  not scrub and no commit pins.
- Therefore **"regenerate it in the correction cycle" will not hold on its own** — that is
  exactly what `6e4b1f7b4e` did, and the stage was red again within one commit. `AGENTS.md`
  rule 8: a chore is not a control.

### 0c. Root cause, reproduced and named — the published feed is wrong, not merely stale

This worker did not stop at the stage name. It re-ran the comparison by hand — seeding a scratch
copy exactly as `--check` does (`cp` the committed feed, `cp -r -p` the shard dir), running
`PF1E_DASHBOARD_STRICT_TIMEOUT=1 python3 scripts/observer/pf1e_dashboard_producer.py` to
completion (exit 0), applying `--check`'s own scrub, and walking both trees key by key.

**Every one of the 650 differing paths is under `/work_inventory`.** `channels`, `content_state`,
`matrix`, `manifests`, `unit_index`, `books` and `workchannels` are identical. The retro-drift /
`channels`-prose hypothesis this worker first drafted is **wrong and is retracted**
(correction `1789035894094-at-35-e2-regate2-af6092`).

| `work_inventory.by_doneness` | Committed feed (live on the site) | Fresh strict render, same pinned inventory |
|---|---|---|
| `done` | **23,650** | **46,965** |
| `in-progress` | **17,563** | **160** |

**The public dashboard is understating SD-35's headline by roughly 23,300 units.** It shows
~17.5 k units in progress on a corpus that `sheet-rules-check` (stage 42) reports at
`records=49438 converted=49296`, and that `docs/work-inventory.json` itself carries as
`sheet-complete 23,315 / text-complete 11,599 / oracle-unverifiable 8,491 / grounded 5,222 /
oracle-agree 811`, sum 49,438, with **`not-started 0`**.

**Root cause, at one line of `scripts/publish-site-dashboard.sh`:** the **publish** branch runs

```sh
python3 "$PRODUCER" --out "$OUT" >/dev/null
…
write_input_pin
```

with **no `PF1E_DASHBOARD_STRICT_TIMEOUT=1`** — `--check` is the only caller that sets it, by
deliberate design, so that a live republish never leaves the public site blank over one slow
build. The consequence is the one the script's own `StateDumpTimeout` comment warns about, now
realised on the publish side instead of the check side: a state dump that times out **silently
falls back to a stale cache**, and `write_input_pin` then stamps that stale render as
authoritative **in the same run**. A producer-version change is excluded —
`git log -- scripts/observer/pf1e_dashboard_producer.py` shows it last changed at `a81c2a005c`,
*below* the `6e4b1f7b4e` republish.

**So `site-dashboard-pin` certifies the INPUT, never that the published OUTPUT was rendered from
it.** That is precisely why stage 8 passes while stage 9 fails, and why the cheap control added
in `6e4b1f7b4e` — correct as far as it goes — cannot catch this shape.

**What the wrap-up correction cycle is owed, as work:**

1. **Republish, strictly.** Re-run the publish path with `PF1E_DASHBOARD_STRICT_TIMEOUT=1` so a
   timing-out dump fails loudly instead of publishing stale numbers, and commit the corrected
   `site/dashboard/PF1e-dashboard.json`, its `units/` shards, `inventory-pin.json` and
   `site/status-data*`. This is a genuine artifact repair — ~23,300 units of public
   under-reporting — not a re-pinned assertion.
2. **The control:** make the publish branch strict, or — if the blank-site protection must stay —
   make `write_input_pin` refuse to stamp a render that came back off the stale-cache path, so a
   fallback render can never be certified as current. Either way the invariant to land is
   *the pin means the output was rendered from the pinned input*, which is what everyone reading
   stage 8 already believes it means.

---

## 1. Retrospective read — `python3 scripts/retro.py summary --since 2026-09-08 --json`

Read in full. Window `2026-09-08T00:00:00+00:00` → run time. **243 events, 0 invalid lines,
40 shards.** (The brief's `--since` spans Epics 1–6, not Epic 2 alone; both readings are given.)

| Type | Count |
|---|---|
| verification | 96 (16 failed runs, fail rate 0.167) |
| correction | 73 (30 with blast radius) |
| incident | 27 (0 silent; 444 minutes lost) |
| deferral | 19 (18 open, 1 resolved) |
| resolution | 13 |
| note | 11 |
| rework | 4 |

Failed verification runs by stage: `figure-provenance` 13, `site-dashboard-check` 4,
`reachability-audit-selftest` 1, `shape-engine-boundary-selftest` 1. Git join: 135 commits,
1 author, 1.8 events per commit. Corrections caught before: implementation 13, merge 7,
release 3, brief 2.

### 1a. Incident keys at 3+ firings — the control or escalation each one owes

| Key | Firings | Control / escalation |
|---|---|---|
| `disk-full` | **6** | **Control landed, and the key is already repaired.** `scripts/reclaim.sh` was mis-keying every successful preventive `--apply` as `incident`/`disk-full` — the key that owns tranche/7's real disk-exhaustion catastrophe — so a 4-hourly cron minted phantom incidents out of a control working as designed. It now emits that key **only** above `RECLAIM_PRESSURE_PERCENT`; covered by `test_reclaim.py`, `reclaim-selftest` PASS at 13 cases in this run. All 6 firings are pre-fix. **No escalation owed; the key should stop growing, and the SD-35 retrospective must not read these 6 as a missing control.** |
| `epic-wrapup-gate-red` | **4** | **Escalation, named.** Epics 2, 3, 4 and 5 each finished with a red wrap-up gate, and this re-gate makes a fifth red. The gate is doing its job; what is missing is *earlier* detection — every one of these was found ~90 minutes into a wrap-up, after the breaking cycle had pushed. `6e4b1f7b4e` began the fix by adding the ~seconds-cost `--check-pin` fast path so a moved inventory is caught per cycle. **Escalated to the operator: extend that principle to the rest of the wrap-up stage set — a per-cycle cheap proxy for each stage that has ever gone red at a wrap-up — or accept that a red wrap-up gate is the normal terminal state of an epic and stop calling it an incident.** It cannot stay a 4-firing key with no control. |
| `site-dashboard-json-stale-after-inventory-move` | **3 → 4 with this run** | **Named control owed, and the existing one is proven insufficient — see §0b/§0c.** `--check-pin` (landed `6e4b1f7b4e`) catches the *inventory-move* shape cheaply and correctly; it passed here. The shape this run hit is different and worse: the publish path renders without `PF1E_DASHBOARD_STRICT_TIMEOUT=1` and pins whatever came back, so a stale-cache fallback is published **and certified**. Control: make the publish branch strict, or make `write_input_pin` refuse a fallback render. The key should also be renamed — `-after-inventory-move` names only one of its two mechanisms. |
| `figure-provenance-command-on-next-line` | **3** | **Control landed.** The `figure-provenance` gate's `--check-provenance` control was added in the Epic 4 wrap-up correction (`1f4c0ad9fe`); the stage is **PASS** in this run at `files_checked=189 figures_examined=334 violations=0`, and the 13 failing runs in the window are the pre-control shape. No escalation owed. |
| `duplicate-criterion-dispatch` | 2 | Below the bar. Control is `workflow-instruction.md §2.4`'s — rebuild the criteria list from `kanban.md` with `complete` rows excluded at **every** relaunch. One more firing makes it 3+. |
| `wrong-base-worktree` | 2 → **3 with this run** | **Now at the 3+ bar. Control named:** a worker dispatched into a fresh worktree must assert its base **before it spends a build** — one line, `git merge-base --is-ancestor origin/<branch> HEAD \|\| (git fetch origin <branch> && git merge --ff-only FETCH_HEAD)`, at the top of `workflow-instruction.md §2.1`'s environment-setup block, beside the `CARGO_TARGET_DIR` export. Recommended twice now (the first re-gate report recommended it and it was not adopted, which is why this worker hit it). **Recommended for §2.1 and for the SD-35 retrospective.** |

Every other key fired once. **New key this run:** `stale-wip-in-reused-worktree` (see the
headline). Its control is the same one line as `wrong-base-worktree`'s, plus a
`git status --porcelain` assertion that a read-only gate worker's tree is clean before it builds.

### 1b. `rust_lines_changed / units_closed` above 3.0 — `decisions.md §4`

Re-derived from each cycle's literal `cycle_scope_gate.py --receipt` row as recorded in
`progress.md`. **No cycle in the window exceeded 3.0.** Every cycle with a real denominator:

| Cycle | rust_lines_changed | units_closed | ratio |
|---|---|---|---|
| AT-35-E2-005 c1 | 337 | 21,911 | **0.02** |
| AT-35-E3-001 c2 (whole remainder) | 266 | 786 | **0.34** |
| AT-35-E3 follow-on | 232 | 618 | **0.38** |
| Epic 1 span cross-check | 11,339 | 21,911 | **0.52** |

`ratio=n/a` is not an exemption, so the Rust-changing **zero-closure** cycles, and what their
lines bought:

- **AT-35-E2-001 c1 — 6,463 lines:** the `sheet_rule_convert` converter and
  `data/sheet_rules/_refused.json`. Tool-side. It is the single artefact behind all 21,911
  closures — amortised, **0.29 lines/unit**. `decisions.md §4`'s explicitly permitted shape (one
  converter mapping row per token type), not per-unit machinery.
- **AT-35-E2-002 c1 — 1,818 lines:** `src/rules_core/sheet_rule.rs` — `render_sheet` /
  `Evaluator::line`, the live surface that turns a converted rule into a rendered sheet line.
  Without it the conversion is data nobody reads.
- **AT-35-E1-003 — 1,906 lines** and **AT-35-E1-002 c1 — 2,520 / 1,168 / 2,173 lines:** the
  Epic 1 build-time tax cut and the citation-anchor instruments. Bought the measured build-time
  figure in `artifacts/epic-1-tax-cut/build-time.json` and the three `--check` gates that are
  now `verify.sh` stages 25/26 — both green in this run.
- **AT-35-E2-003 c1 — 442 lines:** the `sheet-complete` status, one generic classifier pass over
  the whole inventory (`v06_work_inventory.rs` +430/−6), not a per-family probe.
- **AT-35-E2-004 c1 — 295 lines:** the token-coverage ledger's three inventory fields and their
  corpus-fixture gate. One gate per **kind**, reading the live corpus. Stage 29 above is what it
  bought: `verdict=PASS`, `refused_non_done=0`.
- **AT-35-E2-005 c2 — 120 lines:** one live file (`sheet_rule.rs`, +117/−3, of which +93 is the
  test hunk). Bought mechanically: **11 → 0 oracle-parity disagreements.**
- **AT-35-E6-001 (Epic 6) — the `pcgen_import` module move:** bought the residue line in stage
  27 — `live_files` 260 → **253**, `live_hits` 12,736 → **12,256**, `verdict=PASS`.

**Epic-wide, Epic 2: 9,475 Rust lines / 21,911 units closed = 0.43.** Nothing near the 3.0 bar,
and no cycle in the window shows the per-unit-machinery shape `decisions.md §4` forbids.

### 1c. Deferrals — 19 total, **18 open**

No open deferral defers Epic 2 DoD scope. The load-bearing clusters:

- **AT-35-E2-005 (×5) + AT-35-E2-005-DISPOSITION (×2):** the 1,404-unit non-DONE remainder
  Epic 2 handed on — 659 converter refusals across 69 refusal strings, 745 non-promotable.
  **Since discharged**: `token-coverage` now reads `non_done=0 refused_non_done=0` and
  `sheet-rules-check` reads `records=49438 converted=49296 refused=142`. Epics 4 and 5 closed it.
- **AT-35-E3-001 (×2):** cycle 1's sub-floor scope (`scoped=214`), and a residue of **2 units
  that join to no corpus record at all** — the converter has no source row to read.
- **AT-35-E6-001 (×2):** the last live files carrying one of the three PCGen identifiers.
  Explicitly *not* a refused-token remainder.
- **AT-35-E1-003, AT-35-E5-005, at-35-e4-002:** citation-string stems outside a file-touch set;
  10 `DESC`-without-prose units; 10 oracle disagreements, 8 of which are the
  value-role-number-the-oracle-never-prints shape.
- **Three worktree-sweep deferrals** (AT-35-E2-WRAPUP, AT-35-E2-REGATE, AT-35-E3-WRAPUP) — plus
  this worker's, §2 below. **That is four consecutive wrap-ups deferring the same sweep**; it is
  now unambiguously Epic 7's (`workflow-instruction.md §11 step 3`) and should be run by an
  agent on the main checkout, not by an isolated worker.

**No `deferral` for refused token types is owed by this worker** — it closed no units and scoped
no population; it is a read-only gate.

---

## 2. Worktree sweep — deferred, with the reason stated

`df -h /` at gate start **and** at gate end: `/dev/sda1`, 1.5 T, **1.1 T used = 72 % of the 1.5 T, 420 G
available.** No disk pressure; `scripts/reclaim.sh`'s threshold is not met, and stage 1
(`preflight-disk`) passed on its own budget check.

`git worktree list`: **15 entries** — the main checkout (`f1f547a41e`, `tranche/15`) plus 14
worktrees, **11.3 G total**, 606 M–887 M each. `wf_291be5c8-5f3-17` (this worker's) is **locked**
and is never removed.

**The sweep was not executed.** This worker runs under a worktree-isolation guard that refuses
any `git` invocation naming a path outside its own worktree — so `git worktree remove` and
`git rev-list --count origin/tranche/15..<branch>` on the 13 siblings are **not available to it
at all**. It can measure them; it cannot prove unmerged-ness, and removing a worktree without
that proof is the one thing the standing rule forbids. **No `--force` was used and none should
be.** The two Epic 2 worktrees the previous re-gate identified — `wf_291be5c8-5f3-14`
(`a542652c5e`, AT-35-E2-005-DISPOSITION c2, 859 M) and `wf_291be5c8-5f3-15` (`8cc4ea1516`,
AT-35-E2-005-DISPOSITION c1, 858 M) — were already retained by that gate because
`git worktree remove` refused them for modified or untracked files. Nothing has changed that.

Recorded as deferral `1789030473734-at-35-e2-regate2-38d919`, revisit at
`workflow-instruction.md §11 step 3`, or immediately on any disk-pressure incident.

---

## 3. Pull request

**None.** `workflow-instruction.md §10` step 3: no PR at an epic wrap-up.

---

## Residue gate

`python3 scripts/pcgen_residue_gate.py --check`, run standalone at `f1f547a41e` and again as
`verify.sh` stage 27. Literal final line, identical both times:

```
live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
```

Per-root: `src/rules_core` 202 files / 11,779 hits; `apps/desktop` 51 / 477;
`src/saved_character`, `src/campaign`, `src/homebrew_authoring` all **0 / 0**. Zero hits for
`PcgenFormulaEvaluator`, `bonus_stack_reader`, `pre_tokens` and `SAB:`. The converter, parser,
generators and oracle harness are **kept** — `decisions.md §11`; the residue gate measures the
**live** side only, and it is falling (260 → 253 files, 12,736 → 12,256 hits).

---

## Retrospective events emitted by this worker

Shard `docs/retro/events/at-35-e2-regate2.jsonl` (hand-off; **not committed by this worker**),
plus one auto-emitted by `verify.sh` to `docs/retro/events/epic2-sheet-rule-regate.jsonl`.

| id | type | what |
|---|---|---|
| `1789030428094-at-35-e2-regate2-6f202f` | incident | `wrong-base-worktree` — 63 commits behind the tip, caught before any build |
| `1789030438533-at-35-e2-regate2-658288` | incident | `stale-wip-in-reused-worktree` — 15 dirty `.rs` files, a superseded half-codemod |
| `1789030473734-at-35-e2-regate2-38d919` | deferral | worktree sweep not executable under the isolation guard |
| `1789034786341-at-35-e2-regate2-26a448` | incident | `site-dashboard-json-stale-after-inventory-move`, 4th firing, new sub-mechanism |
| `1789035894094-at-35-e2-regate2-af6092` | correction | the published feed's doneness is wrong by ~23,300 units, root cause named; this worker's own first-draft hypothesis retracted |
| (verify.sh, auto) | verification | `verify.sh full: FAIL`, 49 stages, 4,378 s, head `f1f547a41e` |

Two uncommitted shards from the **previous** re-gate attempt were found in this worktree and are
left in place for the next cycle agent to commit: `docs/retro/events/at-35-e2-regate.jsonl`
(2 events) and `docs/retro/events/epic2-sheet-rule-regate.jsonl`. The previous attempt's report,
`EPIC-2_wrapup_regate_report.md` (a PASS at the now-stale `9995efa1b6`), is also still
uncommitted and untouched beside this one.

---

## Verdict

**Epic 2's wrap-up re-gate is RED at `f1f547a41e`: 48 of 49 stages PASS, `site-dashboard-check`
FAIL, 1 h 12 m 58 s.** Every stage Epic 2 owns is green — `sheet-rules-check`, `token-coverage`
and `pcgen-residue-gate` included. `workflow-instruction.md §10` step 0's gating condition is
**not** satisfied: a wrap-up correction cycle is owed before the next epic's second cycle
dispatches.

The red stage is not Epic 2's work and not a formality. **`site/dashboard/PF1e-dashboard.json`,
as published, tells the public that ~17.5 k units are still in progress on a corpus that is at
49,438 of 49,438** — it is understating the bundle's headline by ~23,300 units, and the pin
control certified it. §0c names the root cause at one line of `publish-site-dashboard.sh` and
states the repair and the control the correction cycle owes.

Two further things this gate found that are not about the dashboard, both recorded as
`incident`s: `wrong-base-worktree` reached **three** firings (the control was recommended by the
previous re-gate and not adopted — §1a states it again), and `epic-wrapup-gate-red` reached
**four**, which is escalated to the operator rather than absorbed.
