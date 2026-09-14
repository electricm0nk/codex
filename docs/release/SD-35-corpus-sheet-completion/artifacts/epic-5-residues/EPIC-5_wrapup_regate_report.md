# Epic 5 — Residues: wrap-up **re-gate** report

`workflow-instruction.md §10` steps 0–3, run by the isolated read-only worker
(`§2` worker split, `decisions.md §3`). **This worker pushed nothing and committed nothing.**
The orchestrator hands this file to the next cycle agent to commit.

This is the **second** run of the Epic 5 wrap-up gate. The first
(`EPIC-5_wrapup_gate_report.md`, worker `AT-35-E5-WRAPUP`, HEAD `c3500e7984`) returned RED on one
stage; the wrap-up correction cycle `6e4b1f7b4e` claimed that stage fixed and built a control for
it. This re-gate tests both claims.

- **Worker:** `AT-35-E5-REGATE`, worktree `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-41`
- **HEAD gated:** `f1f547a41e7b0205605a1c502ec13f47bcec2e6a` — `docs(sd35): Epic 5 wrap-up correction receipt names its own commit`, re-derived with `git rev-parse HEAD`. The worktree was created at `fe5ae6cd4a` (the SD-34 merge) and was **122 commits stale**; it was re-based with `git fetch origin tranche/15 && git reset --hard FETCH_HEAD` **before** the gate ran. That is the `wrong-base-worktree` key firing again — see §0b.
- **`CARGO_TARGET_DIR`:** `/tmp/cargo-sd35-epic5-regate`, `CARGO_INCREMENTAL=0`
- **Verdict:** **RED** — **48 of 49 stages passed, 1 failed** (`site-dashboard-check`). Status returned to the orchestrator: `blocked-escalated`. (Re-derive the split: `grep -cE "^    PASS " /tmp/claude-1000/sd35gate/verify.log` → 48; `grep -cE "^    FAIL " /tmp/claude-1000/sd35gate/verify.log` → 1.)

---

## 0. The full gate, once

    scripts/verify.sh          # every stage, no --only

- **Log directory:** `/tmp/codex-verify-UkUEER` (per-stage logs; `site-dashboard-check.log` is the red one)
- **Console transcript:** `/tmp/claude-1000/sd35gate/verify.log`
- **Wall time:** **5,408 s = 90 min 8 s** (`date +%s` before launch = 1789024357, after the
  `RESULT:` line = 1789029765; 1789029765 - 1789024357 = 5408). Comparable: the first Epic 5
  gate ran 5,053 s, the Epic 4 gate 5,432 s.
- **Result line:** `RESULT: FAIL — logs in /tmp/codex-verify-UkUEER` (script exit 1)

### Stage table

**49 stages, 48 PASS / 1 FAIL.** Transcribed verbatim from the console log
(`grep -E "^    (PASS|FAIL)" /tmp/claude-1000/sd35gate/verify.log`).

| # | Stage | Result | Measured line |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK |
| 2 | preflight-oracle | PASS | oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6 |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 27 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 13 passed, 0 failed |
| 8 | site-dashboard-pin | PASS | docs/work-inventory.json matches the pin the feed was published from |
| 9 | **site-dashboard-check** | **FAIL** | exit 1 — /tmp/codex-verify-UkUEER/site-dashboard-check.log |
| 10 | site-dashboard-pi-gate | PASS | 22 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | site/status-data.json and site/status-data/*.json are current |
| 13 | site-public-status-pi-gate | PASS | 31 file(s) scanned against 1612 declared-PI name(s), zero leaked |
| 14 | site-asset-stamp-check | PASS | site/*.html cache-busting stamps match site/styles.css |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling 100.00 % — 49,438 of 49,438 units |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6 |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True |
| 24 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS | files_checked=258 violations=0 |
| 26 | figure-provenance | PASS | files_checked=188 figures_examined=334 violations=0 |
| 27 | pcgen-residue-gate | PASS | live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | token-coverage | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS |
| 30 | pi-sweep | PASS | 11 hits over src/rules_core/rules_tables, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | 3261 passed |
| 38 | root-full | PASS | 8772 passed across 413 suites, all 361 tests/*.rs suites executed |
| 39 | desktop | PASS | 576 passed |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings |
| 42 | sheet-rules-check | PASS | records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (115.4s) |
| 43 | corpus-trap-audit | PASS | records_examined=27634 defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249] traps=407 — all defect kinds at their registered counts |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | npm ci |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | tsc --noEmit clean |
| 48 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 49 | class-dump | PASS | 31/31 computing |

### The one red stage, named exactly — and why the control built for it did not catch it

**`site-dashboard-check`** — `timeout 2400s scripts/publish-site-dashboard.sh --check`, exit 1.
Log `/tmp/codex-verify-UkUEER/site-dashboard-check.log`, in full:

    site/dashboard/PF1e-dashboard.json input pin matches docs/work-inventory.json (5a0a0787312b5181d41214cb52abcd6e0c250fc409a75675ed6e839b4142e36f)
    site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh

Read those two lines together: **the new `--check-pin` control passes and the feed is still
stale.** The correction cycle's control is aimed at the wrong input.

**The delta, attributed.** The temp render `--check` builds is deleted on exit, so it was
reproduced independently: seed a scratch dir with the committed `PF1e-dashboard.json` and
`units/` (`cp -r -p`, preserving mtimes, as `--check` does), run
`PF1E_DASHBOARD_STRICT_TIMEOUT=1 python3 scripts/observer/pf1e_dashboard_producer.py --out <scratch>`,
then diff the two with `--check`'s own scrub rules (drop `usage` and `retrospective`, strip
`generated_at`/`generated_by` recursively, strip the `[engine content dump …]` suffix).
Script: `/tmp/claude-1000/sd35gate/diag.sh`; output `/tmp/claude-1000/sd35gate/diag/diff.txt`.

**650 scrubbed differences. Every one is under `/work_inventory`.** Nothing else in the feed
moved — the whole rest of the payload is byte-identical after scrubbing. Re-derive:
`grep -vE " /work_inventory" /tmp/claude-1000/sd35gate/diag/diff.txt` → the single line
`DIFFERENT` (the verdict marker), i.e. zero non-`work_inventory` diffs of 650.

| Figure | Committed feed | Fresh render at `f1f547a41e` |
|---|---|---|
| `work_inventory.by_doneness.done` | 23,650 | **46,965** |
| `work_inventory.by_doneness.in-progress` | 17,563 | **160** |

(Denominator: the work-inventory unit population, ~49.4 k units — *not* the completion atlas's
`DONE=49438 of 49438` bucket count, which is a different instrument. Re-derive the pair with the
`diag.sh` run above; re-derive the atlas figure with `python3 scripts/completion_atlas.py --check`.)

The committed feed is showing the public dashboard a corpus that is **17,563 units in progress**
when the tree at HEAD has 160. It is not slightly stale; it predates Epic 3's corpus-wide closure.

**Why regenerating it in the correction cycle did not fix it.** The feed's `work_inventory` block
is **not rendered from `docs/work-inventory.json`**. `scripts/observer/pf1e_dashboard_producer.py`
derives it by running the `v06_work_inventory` binary out of an **out-of-repo** build directory
(`PF1E_CLASS_STATE_TARGET_DIR`, default `~/swarm-observer/.class-state-target`) into an
**out-of-repo cache** (`PF1E_WORK_INVENTORY_CACHE`, default
`~/swarm-observer/work-inventory-summary.json`). On a **publish** run
`PF1E_DASHBOARD_STRICT_TIMEOUT` is unset by design, so `_run_state_dump` returns `None` on a
subprocess timeout and the producer falls back to whatever is in that cache — silently. `--check`
is the only caller that sets `PF1E_DASHBOARD_STRICT_TIMEOUT=1` and therefore the only caller that
renders honestly. That asymmetry is the mechanism: **the publish path can commit a stale block
that the check path will then correctly reject, forever.**

This is a *silent* incident in the `retro.py` sense — it produced a plausible-looking wrong feed,
not an error. Event `1789025712319-at-35-e5-regate-7a6473`.

**Named mechanical control for the next correction cycle** (`AGENTS.md` rule 8 — a chore is not a
control, and neither is a pin over an input the failing subtree does not read):

1. **Make the publish path fail loudly too.** Set `PF1E_DASHBOARD_STRICT_TIMEOUT=1` on the
   `publish-site-dashboard.sh` regeneration branch, or give the producer a `--require-fresh`
   flag that refuses to write a payload containing any stale-cache-sourced subtree. A publish that
   cannot render fresh must exit nonzero rather than commit last week's numbers. The existing
   stale-cache fallback exists to keep the public site from going blank on one slow build — keep
   that behaviour for the *live* site publish, but never for the artifact that gets committed.
2. **Extend the pin to every input the feed actually reads.** `site/dashboard/inventory-pin.json`
   currently pins `docs/work-inventory.json` only. Add the `work-inventory-summary.json` cache's
   own `generated_at`/digest and the `v06_work_inventory` binary's source revision, so
   `--check-pin` fails in ~1 s instead of `--check` failing after ~15 min of producer time. This
   is the cheap half; item 1 is the real fix.

Both were absent, which is why this key has now fired **four** times (Epic 2, Epic 3, Epic 5 gate,
Epic 5 re-gate) and accounts for 4 of the 16 failed verification runs in the window.

### Baseline notes

`scripts/verify-baselines.env`'s four floors were advanced by the correction cycle; no baseline is
behind the tree at this gate. `verify.sh` printed **no** `baseline is stale` line at this run (re-derive:
`grep -icE "baseline is stale" /tmp/claude-1000/sd35gate/verify.log` → 0), and the three test
floors match the measured values exactly: `BASELINE_ROOT_LIB_TESTS=3261` vs 3,261 measured,
`BASELINE_ROOT_FULL_TESTS=8772` vs 8,772 measured, `BASELINE_DESKTOP_TESTS=576` vs 576 measured.
That part of the correction cycle worked.

---

## 0b. `wrong-base-worktree` fired again — the control named at the last gate is still unbuilt

This worker's worktree (`wf_291be5c8-5f3-41`) was created at `fe5ae6cd4a` and was **122 commits
behind `origin/tranche/15`** — the identical condition that invalidated the first Epic 5 gate's
opening run in worktree `-35`. Caught here before the gate started, with
`git log --oneline -1` (`fe5ae6cd4a`) against `git log --oneline -1 origin/tranche/15`
(`f1f547a41e`); cleared with `git fetch origin tranche/15 && git reset --hard FETCH_HEAD`.

The last gate named the control and nobody built it: **a `preflight-base` stage in
`scripts/verify.sh`, beside `preflight-disk`/`preflight-oracle`, that runs
`git rev-list --count HEAD..@{upstream}` (or against a `VERIFY_BASE_REF`) and fails nonzero.**
It is a few lines. Without it, every dispatched worker's gate result is trustworthy only if the
worker happened to check. This key has a 27-occurrence history in this repo; `AGENTS.md` rule 8
was written about exactly it.

---

## 1. Retrospective fold

    python3 scripts/retro.py summary --since 2026-09-08 --json

**Window note (`AGENTS.md` rule 9).** `--since 2026-09-08` is the window the dispatch named. It
covers **Epics 2 through 6**, not Epic 5 alone — SD-35 ran Epics 2–6 inside those three days.
Figures below are for that window unless a row says otherwise. 240 events across 135 commits
(`events.total`, `git_join.commits`).

### Counts

| Type | Count |
|---|---|
| verification | 95 (16 failed runs; `fail_rate` 0.1684) |
| correction | 73 |
| incident | 26 (+2 emitted by this worker) |
| resolution | 13 |
| note | 11 |
| deferral | 18 (17 open, 1 resolved) |
| rework | 4 |
| near_miss | 0 |

Incident time lost in the window: **424 minutes**. Corrections caught before: implementation 13,
merge 7, release 3, brief 2. Repeat correction subjects (×2 each):
`epic-breakdown.md AT-35-E1-004 criterion text`, `epic-breakdown.md AT-35-E2-001 evidence`.
Failing verification stages in the window: `figure-provenance` 13, `site-dashboard-check` 4,
`reachability-audit-selftest` 1, `shape-engine-boundary-selftest` 1.

### `incident` keys that fired 3+ times — each gets a control or a named escalation

| Key | Firings | Disposition |
|---|---|---|
| `disk-full` | 6 | **Control exists, built inside the bundle.** `at-35-e2-wrapup-fix` changed `scripts/reclaim.sh` (TDD, `scripts/tests/test_reclaim.py`) to read `df -P` used-percent and emit `disk-full` only at or above `RECLAIM_PRESSURE_PERCENT` (default 90), a `reclaim-routine` note otherwise. These 6 are pre-fix cron noise, not disk pressure. `df -h /` at this gate: 1.1 T used of the 1.5 T root filesystem, 421 G free. **No new mechanism owed.** |
| `epic-wrapup-gate-red` | 4 (**5 with this one**) | Structural. It decomposes into `figure-provenance` — **now controlled and PASS here** (`files_checked=188 figures_examined=334 violations=0`, the second consecutive green) — and `site-dashboard-check`, immediately below. Event `1789025719371-at-35-e5-regate-39c087`. |
| `figure-provenance-command-on-next-line` | 3 | **Control built and working.** Root cause was two different flags: `§6` step 3 ran `denominator_gate.py --check` while `verify.sh` runs `--check-provenance`, so no cycle ever ran the failing flag locally. `workflow-instruction.md §6` step 3 now names `--check-provenance` explicitly as a push-blocking gate. PASS at the last two epic gates. |
| `site-dashboard-json-stale-after-inventory-move` | 3 (**4 with this gate**) | **Control was built at the last wrap-up and is aimed at the wrong input.** Full mechanism and the two named replacement controls are in §0 above. **This is the escalation:** the fix touches `scripts/observer/pf1e_dashboard_producer.py` and `scripts/publish-site-dashboard.sh`, and it changes what a publish run is allowed to commit. It needs a second Epic 5 wrap-up correction cycle with that write scope, before Epic 6's second cycle dispatches. |
| `duplicate-criterion-dispatch` | 2 | Below the bar; carried forward. `rework` names its cause: the orchestrator's dispatch list was not reconciled against `kanban.md`, whose row already read `complete` with a receipt path. |

### `rust_lines_changed / units_closed` above 3.0

**No SD-35 receipt row exceeds 3.0.** Every row with a defined denominator:
`closed=21911 rust_lines_changed=337 ratio=0.02`; `closed=786 … 266 ratio=0.34` (×2);
`closed=618 … 232 ratio=0.38` (×2). Re-derive:
`grep -rhoE "closed=[0-9]+ relabeled=[0-9]+ rust_lines_changed=[0-9]+ ratio=[^ ]+" docs/release/SD-35-corpus-sheet-completion/artifacts | sort | uniq -c | sort -rn`.

**But `ratio=n/a` is doing real work in that answer and should not be allowed to hide spend.**
26 receipt rows report `closed=0 … ratio=n/a` because Epic 3 emptied the corpus
(`26bdfa8d5b`, `406003afc3`, `51f91bba11`) and every later cycle's scope gate returns
`PASS_WHOLE_REMAINDER` with `remaining_non_done=0`. Under the §10 rule read literally, a
zero-denominator cycle can spend any number of Rust lines and never be named. The
zero-closure cycles that spent the most, named here with what the lines bought (same re-derive
command; per-cycle attribution from each receipt's own `Files touched` and `Movement` rows):

| Cycle | Rust lines | What they bought |
|---|---|---|
| `AT-35-E6-001` cycle 1 | 6,463 | The Epic 6 module move: `PcgenFormulaEvaluator` and `formula_interpreter*.rs` out of `src/rules_core/` into `src/pcgen_import/`, plus the live side's single arithmetic entry point `sheet_rule::evaluate_expr` and the first three literal-formula callers converted. Status `partial`. |
| `AT-35-E6-001` cycle 2 | 2,520 | Continued the same criterion: further live callers converted off PCGen formula text. Status `partial`. |
| `AT-35-E6-001` cycle 3 | 2,173 | Same criterion again; the deferral it opened names the two remaining live uses (`resolve_pcgen_var_chain`'s fixpoint over `BONUS:VAR` token text, `bonus_stack_reader::extract_addends`) feeding class-feature magnitudes through `pilot_compute`. Status `partial`. |
| `AT-35-E2-001` cycle 1 | 1,906 | The converter itself — the PCGen-token → `SheetRule` mapping table. `SCOPE_GATE: EXEMPT` (converter-building cycle); it closes nothing by construction and is the machine every later closure ran on. |
| `AT-35-E2-002` / `AT-35-E2-003` cycles 1 | 1,818 / 1,168 | The renderer and its fixtures — the sheet lines the converter's output prints as. |
| `AT-35-E6-001` cycle 4 | 613 | Closed the criterion: `pcgen_residue_gate.py --check` shows `PcgenFormulaEvaluator`, `bonus_stack_reader` and `pre_tokens` at **0 files / 0 hits** on the live side. Status `complete`. |
| `AT-35-E5-004` cycle 1 | 844 | Epic 5's largest and its only player-facing capability: the per-character level-up choice filter (`src/rules_core/level_up_option_filter.rs`), joined over `SheetRule.applies` and served on the existing `preview_level_up` IPC as `featOptions`/`refusedFeatOptions`, plus `LevelUpDialog.tsx`. Satisfies SD-34 `decisions.md §17`'s standing operator requirement. |
| `AT-35-E1-003` cycle 1 | 442 | Epic 1's instrument work. |

Four cycles (`AT-35-E6-001` ×4) spent **11,769 Rust lines against a zero denominator** on one
criterion. They were not wasted — the criterion is *"the live side stops reading PCGen"*, which by
its nature closes no corpus units — but the metric `§10` step 1 asks for cannot see them.
**Recommendation for the bundle retrospective:** a criterion whose acceptance is a residue
reduction should report `rust_lines_changed / residue_hits_removed`, not `/ units_closed`. At this
gate that ratio is measurable and healthy: 11,769 lines removed 7 live files and 480 live hits.

### Deferrals

`deferrals.open = 17` of 18 in the window. Epic 5 opened exactly one:
`1788994100821-at-35-e5-005-5973cb` — the 10 sheet-complete units printing a bare label where the
source record carries 224–829 characters of published `DESC` prose. It is correctly shaped and
**gated, not excused**: the fix is converter-side (`src/pcgen_import/sheet_rule/`, Epic 6's
file-touch set) and `AT-35-E5-005_desc_without_prose.py --check` exits 1 until the count is 0.

Two open deferrals are the *same* structural one, recorded by the Epic 2 and Epic 3 wrap-up
workers and now by this one for the third time: **the isolated read-only worker cannot prune
sibling worktrees** (`1788901958384-at-35-e2-wrapup-cb0191`,
`1788937296885-at-35-e3-wrapup-13bc01`). See §2 — it is an orchestrator action, not a worker one.

### Epic 5 acceptance state (`epic-breakdown.md` §Epic 5)

All five kanban rows (19–23) read `complete`, and every content gate that speaks to them is green
at this HEAD: `missing-engine-tables` `population=0 kinds=0 citation_failures=0` (AT-35-E5-001);
`reachability-audit` at the 100.00 % ceiling (49,438 of 49,438, re-derive
`python3 scripts/reachability_audit.py --check`); `token-coverage` `non_done=0 refused_non_done=0`;
`shape-engine-boundary` `magnitude_bearing=26396 not_held_by_engine=0`. **Epic 5's content is
done.** The red stage is a publication artifact defect in a shared file, not an Epic 5 unit defect
— but `§10` step 0 admits no such distinction, and neither does this report's verdict.

---

## 2. Worktree sweep

    df -h /
    git worktree list

- **Disk** (`df -h /`): `/dev/sda1`, **1.1 T used of the 1.5 T root filesystem, 421 G available**.
  `preflight-disk` recorded 1.1 T used of the same 1.5 T, 420 G available, at the start of the run.
  `/tmp` is on the same filesystem.
- **14 worktrees** under `.claude/worktrees/` plus the shared checkout.

**Every non-self worktree head is fully merged into `origin/tranche/15`.** Re-derived one at a
time with `git rev-list --count <head> ^origin/tranche/15`; all thirteen returned **0**:

| Worktree | HEAD | Unmerged commits |
|---|---|---|
| wf_291be5c8-5f3-2 | `4e321d2c6c` | 0 |
| wf_291be5c8-5f3-3 | `986084c5a4` | 0 |
| wf_291be5c8-5f3-4 | `942c8d3ae5` | 0 |
| wf_291be5c8-5f3-5 | `3c43cf0531` | 0 |
| wf_291be5c8-5f3-7 | `928272a444` | 0 |
| wf_291be5c8-5f3-14 | `a542652c5e` | 0 |
| wf_291be5c8-5f3-15 | `8cc4ea1516` | 0 |
| wf_291be5c8-5f3-17 | `9995efa1b6` | 0 |
| wf_291be5c8-5f3-21 | `07e29075b4` | 0 |
| wf_291be5c8-5f3-24 | `cdcfc897ea` | 0 |
| wf_291be5c8-5f3-27 | `5e2c0c8c5b` | 0 |
| wf_291be5c8-5f3-30 | `00d0611e87` | 0 |
| wf_291be5c8-5f3-35 | `c3500e7984` | 0 (the previous gate worker's tree — its outputs are committed; safe to prune) |
| **wf_291be5c8-5f3-41** (this worker, `locked`) | `f1f547a41e` | 0 |

**Nothing was pruned by this worker, for the third gate running, and this is now an escalation
rather than a note.** `git worktree remove` on another agent's tree is a shared-checkout write,
and the harness refuses this worker any git operation naming a path outside its own worktree — so
the worker can neither prune a tree nor even read its dirty state to decide whether pruning is
safe. Recorded as a deferral at the Epic 2 and Epic 3 wrap-ups with the same reason and never
actioned; thirteen prunable trees have accumulated at roughly 0.7–0.9 G each.

**Orchestrator action, named:** prune `wf_291be5c8-5f3-{2,3,4,5,7,14,15,17,21,24,27,30,35}` with
`git worktree remove` **without** `--force`, stopping on any refusal (a refusal means the tree is
dirty and someone's work is in it), reclaiming roughly 9–10 G. Leave `-41` until this report is
collected. This worker's `CARGO_TARGET_DIR` `/tmp/cargo-sd35-epic5-regate` (34 G at the end of the
run, `du -sh /tmp/cargo-sd35-epic5-regate`) has **already been removed by this worker**;
`/tmp/codex-verify-UkUEER` and `/tmp/claude-1000/sd35gate/` are deliberately kept, because §0's
evidence cites them.

**Standing fix worth one line of the bundle retrospective:** agent worktrees live *under* the
shared checkout at `.claude/worktrees/`, which is what made
`cross-worktree-codemod-contamination` possible at the last gate and what makes pruning a
shared-checkout write here. Creating them outside the repo root removes both problems at once.

---

## 3. PR

**None.** `workflow-instruction.md §10` step 3: no PR at an epic wrap-up.

---

## Residue line

    python3 scripts/pcgen_residue_gate.py --check
    live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS

**The live side has moved down, not up:** 7 fewer files and 480 fewer hits than the recorded
baseline, the first reduction in the bundle — Epic 6's four cycles landing. `verify.sh`'s own
`pcgen-residue-gate` stage returned the identical line. `src/saved_character`, `src/campaign` and
`src/homebrew_authoring` remain at 0 files / 0 hits, and `PcgenFormulaEvaluator`,
`bonus_stack_reader`, `pre_tokens` and `SAB:` are all at 0 on the live side.
`scripts/pcgen-residue-baseline.env` is **not** rewritten by this worker — `--rebaseline` after a
reduction belongs to a cycle with write authority (`workflow-instruction.md §5`).

---

## What the next cycle agent must do

1. Commit this report (it is uncommitted by design; this worker pushed nothing), and fold this
   worker's two retro shards: `docs/retro/events/at-35-e5-regate.jsonl` (2 `incident` + 1
   `deferral`) and `docs/retro/events/epic5-regate.jsonl` (1 `verification`, emitted by
   `verify.sh` itself under `RETRO_ACTOR=epic5-regate`: `verify.sh full: FAIL
   (site-dashboard-check failed)`). Re-derive the shard contents with
   `python3 scripts/retro.py summary --since 2026-09-10 --json`. The report itself is green
   under both package gates at this HEAD: `python3 scripts/denominator_gate.py --check`
   → `files_checked=259 violations=0`, and `--check-provenance` → `files_checked=189
   figures_examined=334 violations=0`.
2. Run a **second** Epic 5 wrap-up correction cycle before Epic 6's second cycle dispatches:
   - Build the real control for `site-dashboard-json-stale-after-inventory-move` (4th firing):
     make the publish path refuse to commit a stale-cache-sourced `work_inventory` block, and
     widen `site/dashboard/inventory-pin.json` past `docs/work-inventory.json`. §0 names both.
   - Regenerate and commit `site/dashboard/PF1e-dashboard.json` **from a run that did not fall
     back to the cache** — verify by re-running `--check` and seeing `is current`, not by
     trusting the publish command's exit code.
3. Build `preflight-base` in `scripts/verify.sh` (§0b). Two gates in a row have opened with a
   122-commit-stale worktree.
4. Prune the thirteen merged worktrees (§2).
