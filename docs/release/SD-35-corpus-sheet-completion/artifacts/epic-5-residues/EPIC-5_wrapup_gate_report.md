# Epic 5 — Residues: wrap-up gate report

`workflow-instruction.md §10` steps 0–3, run by the isolated read-only worker
(`§2` worker split, `decisions.md §3`). **This worker pushed nothing and committed nothing.**
The orchestrator hands this file to the next cycle agent to commit.

- **Worker:** `AT-35-E5-WRAPUP`, worktree `/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-35`
- **HEAD gated:** `c3500e79843086b2c4547a33428b01e27f963d53` (`docs(sd34): re-stamp the atlas artifact's derived_at at AT-35-E5-005's final HEAD`), re-derived with `git rev-parse HEAD`
- **`CARGO_TARGET_DIR`:** `/tmp/cargo-sd35-AT-35-E5-WRAPUP`, `CARGO_INCREMENTAL=0`
- **Verdict:** **RED** — 47 of 48 stages passed, 1 failed. Status returned to the orchestrator:
  `blocked-escalated`.

---

## 0. The full gate, once

    scripts/verify.sh -j 4      # every stage, no --only

- **Log directory:** `/tmp/codex-verify-LhBNRa` (per-stage logs; `site-dashboard-check.log` is the red one)
- **Console transcript:** `/tmp/claude-1000/sd35gate/verify-stdout.log`
- **Wall time:** **5,053 s = 84 min 13 s** (`date +%s` before launch = 1788996828, after
  `RESULT:` = 1789001881; `1789001881 - 1788996828 = 5053`). Comparable: the Epic 4 wrap-up gate
  ran 5,432 s.
- **Result line:** `RESULT: FAIL — logs in /tmp/codex-verify-LhBNRa`

### Stage table — 48 stages, 47 PASS / 1 FAIL

| # | Stage | Result | Measured line |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK (65% used, 517 G available of the 1.5 T root filesystem) |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 27 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 8 passed, 0 failed |
| 8 | **site-dashboard-check** | **FAIL** | exit 1 — `site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh` |
| 9 | site-dashboard-pi-gate | PASS | 21 files scanned against 1,612 declared-PI names, zero leaked |
| 10 | build-public-status-selftest | PASS | 37 cases passed |
| 11 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 12 | site-public-status-pi-gate | PASS | 31 files scanned against 1,612 declared-PI names, zero leaked |
| 13 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 14 | reachability-audit-selftest | PASS | 11 cases passed |
| 15 | reachability-audit | PASS | reachable ceiling 100.00% (49,438 of 49,438) |
| 16 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 17 | supersession-gate-selftest | PASS | 16 cases passed |
| 18 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 19 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 |
| 20 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 21 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 22 | shape-engine-boundary | PASS | magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True |
| 23 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 24 | denominator-gate | PASS | files_checked=252 violations=0 |
| 25 | figure-provenance | PASS | files_checked=182 figures_examined=268 violations=0 |
| 26 | pcgen-residue-gate | PASS | live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS |
| 27 | token-coverage-selftest | PASS | 14 cases passed |
| 28 | token-coverage | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 |
| 29 | pi-sweep | PASS | — |
| 30 | declared-pi-audit | PASS | — |
| 31 | audit-selftest | PASS | — |
| 32 | reclaim-selftest | PASS | — |
| 33 | driver-selftest | PASS | — |
| 34 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 35 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 36 | root-lib | PASS | 3,230 passed |
| 37 | root-full | PASS | 8,741 passed across 412 suites, all 361 `tests/*.rs` suites executed |
| 38 | desktop | PASS | 576 passed |
| 39 | reach | PASS | 32 passed |
| 40 | corpus-sweep | PASS | 48,706 records examined of 51,476 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, 0 findings |
| 41 | sheet-rules-check | PASS | records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 (116.3 s) |
| 42 | corpus-trap-audit | PASS | records_examined=27634; defects wiring-class-mismatch=0, disabled-line=165, key-differs-from-name=650, mod-record=2117, shared-name-distinct-records=249; traps=407 — all at registered counts |
| 43 | supersession-gate | PASS | 116 objects, all clean |
| 44 | frontend-install | PASS | `npm ci` (node_modules was absent) |
| 45 | frontend-test | PASS | 101/101 files |
| 46 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 47 | clippy | PASS | root 0 / desktop 0 warnings, 0 errors |
| 48 | class-dump | PASS | 31/31 computing |

### The one red stage, named exactly

**`site-dashboard-check`** — `timeout 2400s scripts/publish-site-dashboard.sh --check`, exit 1.
Log `/tmp/codex-verify-LhBNRa/site-dashboard-check.log`, single line:

    site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh

Fix in the Epic 5 wrap-up correction cycle, before Epic 6's second cycle dispatches
(`workflow-instruction.md §10` step 0). The immediate fix is one command
(`./scripts/publish-site-dashboard.sh`, then commit `site/dashboard/PF1e-dashboard.json`).
**That command is a chore, not the fix** — see §1's control below.

### Baseline notes (verify.sh reports these as NOT failures; update deliberately)

Three counts in `scripts/verify-baselines.env` are behind the tree and should be advanced in the
same correction cycle:

| Baseline | Recorded | Measured |
|---|---|---|
| `BASELINE_ROOT_LIB_TESTS` | 3,223 | 3,230 |
| `BASELINE_ROOT_FULL_TESTS` | 8,734 | 8,741 |
| `BASELINE_DESKTOP_TESTS` | 574 | 576 |

The deltas (+7, +7, +2) are consistent with AT-35-E5-004's `level_up_option_filter` unit tests and
its two desktop `character_hub` IPC tests; that cycle changed the counts and did not re-stamp the
baselines.

---

## 0b. Two incidents that invalidated earlier runs of this same gate

Both are recorded in `docs/retro/events/at-35-e5-wrapup.jsonl`. Neither is an Epic 5 content
defect; both are dispatch-infrastructure defects that made a gate result untrustworthy, which is
exactly the shape `AGENTS.md` "Concurrency and Measurement" was written for.

**(a) `wrong-base-worktree` — the dispatched worktree was 122 commits stale.**
`wf_291be5c8-5f3-35` was created at `fe5ae6cd4a` (the SD-34 merge, PR #383) and contained none of
SD-35 Epics 1–5. Caught with
`git rev-list --left-right --count HEAD...tranche/15` → `0	122`, before the first run finished.
Fixed here with `git fetch origin tranche/15 && git reset --hard origin/tranche/15`.
*Named mechanical control:* add a `preflight-base` stage to `scripts/verify.sh` beside
`preflight-disk`/`preflight-oracle` that runs `git rev-list --count HEAD..@{upstream}` and fails
nonzero. A dispatch-prompt caution is not a control (`AGENTS.md` rule 8); this key has a
27-occurrence history in this repo.
Event id `1788995389793-at-35-e5-wrapup-156d6c`.

**(b) `cross-worktree-codemod-contamination` — a concurrent Epic 6 agent edited this worktree.**
At 19:11–19:12 local, 15 Rust files and one atlas JSON inside *this* worktree were rewritten from
`use crate::rules_core::pilot_compute::formula_interpreter` (module exists) to
`use crate::pcgen_import::formula_interpreter` (module does not exist here), while the matching
file *moves* into `src/pcgen_import/` landed only in the shared checkout
(`formula_interpreter.rs` mtime 19:12, `formula_reproduction_harness.rs` 19:19, both uncommitted
there). The gate went red on `root-full` with `cargo` exit 101 and 10 `E0432` unresolved-import
errors, **0 of ~490 suites executed**. Direction confirmed against
`git show HEAD:src/rules_core/trait_effects.rs`, which carries the correct `rules_core` path.
Restored with `git checkout -- src apps tests docs`; the tree was verified clean before, during
and after the run that this report certifies (`git status --porcelain` → only this worker's own
untracked retro shard).
*Root cause:* agent worktrees live **under** the shared checkout at `.claude/worktrees/`, so any
codemod anchored at the repo root that walks `.` edits every other agent's tree.
*Named mechanical control:* either (a) create agent worktrees **outside** the repo root, or
(b) require every codemod in `workflow-instruction.md §6` to be anchored on explicit top-level
directories (`src apps tests scripts docs`) and never `.`, with `.claude/worktrees` in a
repo-root ignore file the dispatch template's `sed`/`grep` recipes honour.
Event id `1788996810915-at-35-e5-wrapup-e2468e`.

---

## 1. Retrospective fold

    python3 scripts/retro.py summary --since 2026-09-07 --json \
      --events-dir /home/ubuntu/workspace/repos/codex/docs/retro/events

**Window note (`AGENTS.md` rule 9).** The dispatch named `--since 53296d80f0`.
`scripts/retro.py` does not accept a commit-ish (`retro: cannot parse time 53296d80f0`), so the
window was resolved by date:
`git log -1 --format='%ci' 53296d80f0` → `2026-09-07 21:24:13 -0400`, and the summary was run with
`--since 2026-09-07`. The window is therefore **bundle-wide (SD-35 Epics 1–5)**, not Epic-5-only.
Epic-5-only figures below are derived separately from `docs/retro/events/at-35-e5-*.jsonl`.
The `--events-dir` override is required because the SD-35 shards are uncommitted on the shared
checkout and absent from this worktree.

### Counts — SD-35 window (283 events, 19 commits)

| Type | Count |
|---|---|
| verification | 153 (18 failed runs; fail_rate 0.1176) |
| correction | 63 |
| incident | 28 |
| deferral | 16 |
| resolution | 11 |
| note | 9 |
| rework | 3 |
| near_miss | 0 |

Corrections caught before: implementation 13, merge 7, release 3, brief 2 (plus 2 free-text).
Repeat correction subjects: `epic-breakdown.md AT-35-E1-004 criterion text` ×2,
`epic-breakdown.md AT-35-E2-001 evidence` ×2.
Failing verification stages in the window: `figure-provenance` 13, `site-dashboard-check` 6,
`reachability-audit-selftest` 1, `shape-engine-boundary-selftest` 1.

### Counts — Epic 5 only (8 events across `at-35-e5-00{1,2,3,5}.jsonl`)

6 corrections, 1 deferral, 1 rework, 0 incidents. Substance:

- **AT-35-E5-002:** `class_feature_of_unmodelled_corpus_class` was **446 units over 58 classes** at
  the cut, not the 634 / 60 the criterion text claimed.
- **AT-35-E5-003:** 166 package files were still printing upstream PCGen's editorial
  `[NOT IMPLEMENTED]` marker on the rendered sheet line (161 in prose, 5 in a record label);
  and of 221 bucket-U/Z units, **217 render a line, 4 carry the source record's own `print:false`**
  and are correctly absent from the whole-sheet render.
- **AT-35-E5-001:** kanban row 19 read `complete` with only the first of two Evidence clauses paid.
- **AT-35-E5-005:** SD-34's `capability-register.json` `oracle_probe_surface_for_no_table_kinds`
  row read **130**, not the recorded 2,062, under the row's own re-derive command; and **10 of
  49,438** sheet-complete units print a bare label where the source record carries 224–829
  characters of published `DESC` prose.

### Recurring incident keys (3+ firings) and their disposition

| Key | Firings in window | Disposition |
|---|---|---|
| `disk-full` | 14 | **Control exists and was already fixed inside this window.** The Epic 2 wrap-up proved 0 of the first 12 were disk pressure: all were successful 4-hourly `scripts/reclaim.sh --apply` cron runs with `used_percent=None`, logged as `type=incident recurrence_key=disk-full`. `at-35-e2-wrapup-fix` changed `reclaim.sh` (TDD, `scripts/tests/test_reclaim.py`, 23 tests OK) to read `df -P` used-percent and emit `disk-full` only at or above `RECLAIM_PRESSURE_PERCENT` (default 90), a `reclaim-routine` note otherwise. The 14 here are pre-fix noise; **no new mechanism is owed**. |
| `figure-provenance-command-on-next-line` | 3 | **Control built.** Root cause (found at the Epic 4 wrap-up) was two different flags: `§6` step 3 ran `denominator_gate.py --check` while `verify.sh` runs `--check-provenance`, so no cycle ever ran the failing flag locally. The control is now in `workflow-instruction.md` line 557 (`python3 scripts/denominator_gate.py --check-provenance   # DIFFERENT flag from --check above; nonzero exit BLOCKS the push`). **It works:** `figure-provenance` was **PASS** at this gate (files_checked=182, figures_examined=268, violations=0), the first SD-35 epic wrap-up at which it was green. |
| `epic-wrapup-gate-red` | 3 (4 with this one) | Structural, not a defect key of its own: each firing decomposes into `figure-provenance` (now controlled) and `site-dashboard-check` (below). |
| `site-dashboard-json-stale-after-inventory-move` | 2 (**3 with this gate**) | **Control owed and named here.** No mechanism has ever been built; the disposition each time was "run `./scripts/publish-site-dashboard.sh` in the wrap-up correction cycle" — a chore, which `AGENTS.md` rule 8 forbids as a response to recurrence. **Named mechanical control:** (a) make the producer of `docs/work-inventory.json` also write `site/dashboard/PF1e-dashboard.json` in the same run, so the two cannot diverge — this is the real fix; or, weaker, (b) add `./scripts/publish-site-dashboard.sh --check` to `workflow-instruction.md §6` step 3's per-cycle gate block with nonzero exit blocking the push, the same shape that worked for `--check-provenance`. Option (b) alone only moves the red earlier. Event id `1788996930267-at-35-e5-wrapup-75e4c1`. |
| `duplicate-criterion-dispatch` | 2 | Below the 3+ bar; carried forward. |

### Deferrals

`deferrals.open = 16`, `resolved = 0` at the time of the summary. One of those was already closed
in prose and never closed in the log: `1788922132640-at-35-e3-002-ac4da5` (AT-35-E3-002's two named
evidence artefacts). AT-35-E5-004 built the level-up choice filter
(`src/rules_core/level_up_option_filter.rs`, commit `a56096b861`, served on `preview_level_up` as
`featOptions`/`refusedFeatOptions`) and `kanban.md` row 22 names the deferral id as cleared, but no
`resolution` event was emitted. This worker emitted it:
`1788996902744-at-35-e5-wrapup-3027ca`. **Open deferrals after that: 15.**

The one deferral Epic 5 itself opened is `1788994100821-at-35-e5-005-5973cb` — the 10
`DESC`-without-prose units. It is correctly shaped: the fix is converter-side
(`src/pcgen_import/sheet_rule/`), which is Epic 6's file-touch set, and it is **gated, not
excused** — `AT-35-E5-005_desc_without_prose.py --check` exits 1 until the count reaches 0.

### `rust_lines_changed / units_closed` — cycles above 3.0

**None, bundle-wide.** Every SD-35 receipt row where a denominator exists is far below the bar:
`closed=21911 rust_lines_changed=337 ratio=0.02`, `closed=786 … 266 ratio=0.34` (×2),
`closed=618 … 232 ratio=0.38` (×2). Re-derived with
`grep -rhoE "closed=[0-9]+ relabeled=[0-9]+ rust_lines_changed=[0-9]+ ratio=[^ ]+" docs/release/SD-35-corpus-sheet-completion/artifacts | sort | uniq -c | sort -rn`.

Epic 5's own five cycles all report `ratio=n/a` because **all five closed zero units by design** —
their populations were already emptied by Epic 3 (`26bdfa8d5b`, `406003afc3`, `51f91bba11`) and the
scope gate returned `PASS_WHOLE_REMAINDER` with `remaining_non_done=0` for the entire corpus. What
Epic 5's **1,283 Rust lines** bought, cycle by cycle:

| Cycle | Rust lines | What they bought |
|---|---|---|
| AT-35-E5-001 | 252 | The unpaid second Evidence clause for bucket A: the refusal/success transcript pair for the `power` (421) and `companion` (28) tables — `--epic5-table-transcript` in `src/bin/v06_work_inventory.rs` and `artifacts/epic-5-residues/table-proofs.md`. Row 19 had read `complete` with the clause unpaid. |
| AT-35-E5-002 | 0 | Evidence only: `AT-35-E5-002_bucket_d_sub_causes.py` enumerates bucket D's 14 sub-cause families over 1,982 units, `not_sheet_complete_at_HEAD=0`. Also produced the 634→446 correction. |
| AT-35-E5-003 | 187 | Removed upstream PCGen's editorial `[NOT IMPLEMENTED]` marker from the rendered sheet line — `strip_editorial_not_implemented_markers`/`scrub_editorial_markers` in `src/pcgen_import/sheet_rule/prose.rs`, wired into both `DESC`-like and positional prose conversion and into every line's label in `convert.rs`. 166 package files were printing it. Directly the sheet rule: a player was reading a converter's admission instead of a rule. |
| AT-35-E5-004 | 844 | The per-character level-up choice filter — `src/rules_core/level_up_option_filter.rs` joining over `SheetRule.applies`, served on the existing `preview_level_up` IPC as `featOptions`/`refusedFeatOptions`, plus `LevelUpDialog.tsx`. Also fixed the converter defect it exposed: `PreStatScore_<AB>` read 0, 354 record files regenerated. This is the largest single spend in the epic and the only one that adds a player-facing capability (SD-34 `decisions.md §17`'s standing operator requirement). |
| AT-35-E5-005 | 0 | Closure accounting: `completion-manifest.json` (49,438 rows, 0 non-DONE), `capability-register-rederived.json` (11 of 11 rows closed: 5 built, 6 `unnecessary-under-sheet-rule`, 0 open, over 11,055 units), `desc-without-prose.json` + its `--check` gate. |

Two of the five cycles wrote code that a player sees (E5-003, E5-004); three paid evidence clauses
that had been marked complete without their evidence. That pattern — a criterion's row reading
`complete` while its second Evidence clause is unpaid — is the epic's own recurring shape and is
worth a line in the bundle retrospective.

### Epic 5 acceptance state (`epic-breakdown.md` §Epic 5)

All five kanban rows (19–23) read `complete`. Independently re-derived at the gated HEAD:

- `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0`, `DONE: 49438`, **A/B/C/D/M/V/U/X/Z all 0**, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`, `stale_derived_at=False`, `citation_failures=0`.
- `missing-engine-tables` stage → `population=0 kinds=0 citation_failures=0` (AT-35-E5-001).
- `reachability-audit` stage → reachable ceiling 100.00% (49,438 of 49,438).
- `token-coverage` stage → `non_done=0 refused_non_done=0`.

---

## 2. Worktree sweep

    df -h /
    git worktree list

- **Disk** (`df -h /`, at the end of the run): 1,004 G used of the 1.5 T `/dev/sda1` root
  filesystem, 449 G available. At `preflight-disk`, before the run: 935 G used of the same 1.5 T,
  517 G available. The delta is this gate's own 33 G `CARGO_TARGET_DIR`. `/tmp` is on the same
  filesystem.
- **13 worktrees** under `.claude/worktrees/`, 606 M–869 M each, ~10.3 G total.

**Every non-self worktree head is fully merged into `origin/tranche/15`.** Re-derived one at a
time with `git rev-list --count <head> ^origin/tranche/15`; all twelve returned **0**:

| Worktree | HEAD | Unmerged commits | Size |
|---|---|---|---|
| wf_291be5c8-5f3-2 | `4e321d2c6c` | 0 | 713 M |
| wf_291be5c8-5f3-3 | `986084c5a4` | 0 | 713 M |
| wf_291be5c8-5f3-4 | `942c8d3ae5` | 0 | 714 M |
| wf_291be5c8-5f3-5 | `3c43cf0531` | 0 | 714 M |
| wf_291be5c8-5f3-7 | `928272a444` | 0 | 606 M |
| wf_291be5c8-5f3-14 | `a542652c5e` | 0 | 859 M |
| wf_291be5c8-5f3-15 | `8cc4ea1516` | 0 | 858 M |
| wf_291be5c8-5f3-17 | `9995efa1b6` | 0 | 864 M |
| wf_291be5c8-5f3-21 | `07e29075b4` | 0 | 867 M |
| wf_291be5c8-5f3-24 | `cdcfc897ea` | 0 | 867 M |
| wf_291be5c8-5f3-27 | `5e2c0c8c5b` | 0 | 869 M |
| wf_291be5c8-5f3-30 | `00d0611e87` | 0 | 869 M |
| **wf_291be5c8-5f3-35** (this worker, `locked`) | `c3500e7984` | 0 | 740 M |

**Nothing was pruned by this worker, deliberately.** Two reasons, both binding:

1. This is the isolated read-only worker; it pushes nothing and does not administer the shared
   checkout. `git worktree remove` on another agent's tree is a shared-checkout write. The Epic 3
   wrap-up recorded the same disposition (deferral: *"the orchestrator (not worktree-isolated)
   removes all nine agent worktrees"*).
2. The harness refuses this worker's `git -C <other-worktree> status`, so **dirty state could not
   be checked from here**. `git worktree remove` refuses a dirty tree by default, which is the
   safety net — the orchestrator must run it **without** `--force` and must stop, not force, on any
   refusal.

Also live and relevant to sequencing: an **Epic 6 agent is mid-flight on the shared checkout**
(uncommitted `src/pcgen_import/formula_interpreter.rs` and `formula_reproduction_harness.rs`,
mtimes 19:12 and 19:19). Do not prune worktrees or run a sweep that touches `.claude/worktrees`
until it lands — that is the mechanism behind incident (b).

Recommended for the orchestrator: prune the twelve merged worktrees (`wf_291be5c8-5f3-2, -3, -4,
-5, -7, -14, -15, -17, -21, -24, -27, -30`), reclaiming ~9.6 G; leave `-35` until this run's
outputs are collected. This worker's `CARGO_TARGET_DIR` `/tmp/cargo-sd35-AT-35-E5-WRAPUP` (33 G)
should be removed once this report is committed.

---

## 3. PR

**None.** `workflow-instruction.md §10` step 3: no PR at an epic wrap-up.

---

## Residue line

    python3 scripts/pcgen_residue_gate.py --check
    live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS

Unchanged against baseline: the live side (`src/rules_core` minus `cache_gen`,
`src/saved_character`, `src/campaign`, `src/homebrew_authoring`, `apps/desktop`) took on no new
PCGen reads across Epic 5. `src/saved_character`, `src/campaign` and `src/homebrew_authoring` are
at 0 files / 0 hits. The same numbers came back from `verify.sh`'s own `pcgen-residue-gate` stage.

---

## What the next cycle agent must do

1. Commit this report (it is uncommitted by design; this worker pushed nothing).
2. Run the Epic 5 wrap-up **correction cycle** before Epic 6's second cycle dispatches:
   - `./scripts/publish-site-dashboard.sh`, commit `site/dashboard/PF1e-dashboard.json` — clears the one red stage.
   - Build the named control for `site-dashboard-json-stale-after-inventory-move` (3rd firing). A chore is not a control.
   - Advance the three stale counts in `scripts/verify-baselines.env` (3,230 / 8,741 / 576).
3. Consider the two dispatch-infrastructure controls named in §0b — `preflight-base` in
   `verify.sh`, and moving agent worktrees out from under the repo root. Both cost one gate run
   each time they are skipped.
