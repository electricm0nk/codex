---
canonical: true
owner: sd34-at-34-e6-001
bundle_id: SD-34
criterion: AT-34-E6-001 (gate-remediation tracking label -- NOT the final-acceptance scan)
date: 2026-09-02
verdict: complete (this cycle's own obligation: a full, honest, independently-re-derived sweep; the bundle's gate itself remains not-green, 3 of 40 FAIL, none new, none outside the review's original 14)
---

# Wave 29 -- gate-remediation closing sweep (full-sweep, no regeneration)

**This is the closing cycle of a gate-remediation wave whose three lanes (A rust-suites, B
frontend, C docs-gates) had just landed.** Per this cycle's own dispatch brief: full sweep only,
**not** a regeneration -- the inventory regenerator (`v06_work_inventory` for real, not
`--summary`) and the dashboard producer were never invoked outside `verify.sh`'s own read-only
`--check`/`--summary` stages. `docs/work-inventory.json` and `site/dashboard/PF1e-dashboard.json`
were not touched.

- **Commit SHA (before this cycle's own commit):** `d17c784ccd5954728fefff19b06ac07346c896be`
  (`tranche/14`, post-rebase). `git fetch origin tranche/14 && git rebase origin/tranche/14`
  reported "Successfully rebased and updated" -- the three lanes' work (gate-lane-a/b/c wave
  24/26/27 commits, already on `origin/tranche/14`) plus two unrelated same-day
  model-tier-config chores (`6cf6f7b436`, `d17c784ccd`) were pulled in; no new lane commits
  arrived between fetch and this cycle's own rebase.
- **This cycle's own commit:** see bottom (pushed after this receipt).
- **Files touched by this cycle:** this receipt (new) + `progress.md` (prepended entry). **No**
  `src/`, no `apps/desktop/src-tauri/src/`, no `data/corpus/**`, no `scripts/verify-baselines.env`
  -- nothing needed a code fix or a baseline edit this cycle (see §5, Baseline note).
- **Identifier audit result:** 4 hits, all real, existing test-suite-name citations
  (`sd24_wired_integration_audit` x2, `sd27_pathfinder_unchained_cache_shape` x2, naming this
  cycle's own root-full failure attribution) — the same "cite the real filename in prose" shape
  wave-26-lane-a's own receipt already established is not a defect (its own doc-comment citation
  of `sd30_declared...`). Zero fabricated or new bundle-tag-shaped tokens.
- **Wired-integration audit result:** `OK_NO_TOKENS` (zero `STUB`/`MOCK`/`placeholder`/`not yet
  implemented`/`todo`/`fixme`/`hack` tokens in the diff).

## 1. Rebase

`git fetch origin tranche/14 && git rebase origin/tranche/14` -> "Successfully rebased and
updated refs/heads/tranche/14." `HEAD` after rebase: `d17c784ccd`.

## 2. Full sweep, run live, foregrounded

`bash scripts/verify.sh` at HEAD `d17c784ccd` (`-j 2`, this box's default concurrency -- not
`-j 8`; slower than prior waves' sweeps but every stage still ran to completion, not `--quick`,
not `--only`-scoped). **First attempt (started ~21:49 EDT) was killed mid-run** (background-task
kill, cause not conclusively isolated -- possibly this session's own repeated short foreground
polling commands sending an interrupt that propagated to the child, per the standing
`interrupt-can-silently-kill-background-workflows` hazard) while `root-full` was still building
its ~490 test binaries; git state was unaffected (confirmed clean re-check before restart) and no
code had been touched, so nothing was lost. **Re-run, this time fully detached** (`nohup ... &
disown`, stdin closed) so it could not be affected by this session's own foreground command
lifecycle: started 22:50:31 EDT, finished 00:43:21 EDT -- **total wall time ~1h53m**
(`site-dashboard-check` alone accounted for the first ~11 minutes, consistent with its own known
600s-times-two producer timeout; `root-full`'s ~490-binary build+run consumed most of the
remainder, roughly 22:50-00:23). Logs: `/tmp/codex-verify-OCGrDe/`. One single run covering all
40 stages start to finish, not killed a second time.

## 3. The stage table -- all 40, PASS/FAIL, at HEAD `d17c784ccd`

**37 PASS, 3 FAIL.**

| # | Stage | Result | Detail |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK (187G available) |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 21 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 6 passed, 0 failed |
| 8 | site-dashboard-check | **FAIL** | exit 1 -- `v06_work_inventory --summary` timed out at the producer's own internal 600s cap, twice (`/tmp/codex-verify-OCGrDe/site-dashboard-check.log`); pre-existing since wave-26, unchanged, not new |
| 9 | site-dashboard-pi-gate | PASS | 21 files vs 1612 declared-PI names, zero leaked |
| 10 | build-public-status-selftest | PASS | 37 cases passed |
| 11 | site-public-status-check | PASS | status-data current |
| 12 | site-public-status-pi-gate | PASS | 31 files vs 1612 names, zero leaked |
| 13 | site-asset-stamp-check | PASS | stamps match |
| 14 | reachability-audit-selftest | PASS | 11 cases passed |
| 15 | reachability-audit | PASS | reachable ceiling 98.90% (48893/49438) |
| 16 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 17 | supersession-gate-selftest | PASS | 16 cases passed |
| 18 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 19 | shape-coverage-standing-gate | PASS | population=26112 unclassified=0 no_record=0 |
| 20 | denominator-gate | **FAIL** | violations=2 of files_checked=145 -- both at `AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md:138` and `:153` (a CPU-utilization figure restated with no same-line re-derive command); pre-existing, already named by wave-26/27/28, not this cycle's content, not gate-lane-a/b/c/rust-suites/frontend/docs-gates territory |
| 21 | figure-provenance | PASS | files_checked=75 figures_examined=125 violations=0 |
| 22 | pi-sweep | PASS | 11 hits over rules_tables, 11 baseline rows |
| 23 | declared-pi-audit | PASS | clean |
| 24 | audit-selftest | PASS | 28 passed, 0 failed |
| 25 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 26 | driver-selftest | PASS | 7 passed, 0 failed |
| 27 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 28 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 29 | root-lib | PASS | 3028 passed (baseline floor 2336, STALE note printed, not this cycle's to update -- see §5) |
| 30 | root-full | **FAIL** | cargo exit 101; 8369 passed across 589 suites, 2 failing suites / 3 failing tests: `sd24_wired_integration_audit` (4 passed/1 failed -- `placeholder_findings_are_ui_text_prose_or_the_one_documented_deferral`, hit at `apps/desktop/src-tauri/src/reach_gate.rs:3192`) and `sd27_pathfinder_unchained_cache_shape` (5 passed/2 failed -- `equipment_cache_has_all_42_real_pu_equipmods_records` left:38 right:42, `equipment_cache_plus_zero_records_have_no_fabricated_plus_value` left:3 right:7); both already fully named by wave-26/27/28 (the second is the same PU equipment_modifier drift the desktop stage's own fix, row 31, already accounts for); neither new |
| 31 | desktop | PASS | 572 passed (baseline floor 515, STALE note printed, not this cycle's to update -- see §5) |
| 32 | reach | PASS | 31 passed |
| 33 | corpus-sweep | PASS | `48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings` -- exact match to the pinned floor (see §5, Baseline note: no update needed) |
| 34 | corpus-trap-audit | PASS | records_examined=27634, all 4 registered defect kinds at their pinned counts exactly (`wiring-class-mismatch=0`) |
| 35 | supersession-gate | PASS | 116 objects, all clean |
| 36 | frontend-install | PASS | node_modules present |
| 37 | frontend-test | PASS | 100/100 files |
| 38 | frontend-typecheck | PASS | tsc --noEmit clean |
| 39 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 40 | class-dump | PASS | 31/31 computing |

Row-count check (`awk -F'|' '/^\| [0-9]+ \|/{n++; if ($4 ~ /FAIL/) f++} END {print n, f, n-f}'` over
this table): `40 3 37` -- matches `verify.sh`'s own printed summary exactly:
`passed: 37 ... FAILED: 3 site-dashboard-check denominator-gate root-full`.

## 4. The bar this cycle was held to

**"Same-or-fewer red stages than the 14 the review recorded, and ZERO stages that were green
going red."**

- **Same-or-fewer: PASS.** 3 red <= 14 red, by a wide margin. Every one of this run's 3 FAILs
  (`site-dashboard-check`, `denominator-gate`, `root-full`) is a **member** of the review's
  original 14 (`fable-review.md` §7: `reachability-audit(+selftest)`,
  `shape-coverage-standing-gate`, `site-dashboard-check`, `site-public-status-check`, `root-lib`,
  `root-full`, `desktop`, `reach`, `frontend-test`, `denominator-gate`, `figure-provenance`,
  `pi-sweep`, `clippy`). None of this run's FAILs falls outside that set.
- **Zero green->red: PASS.** Cross-checked this run's 3 FAILs against the two most recent
  independent full-sweep receipts:
  - wave-28's gate-sweep (`65f24c9936`, `bfaa288ead`): 5 FAIL = `site-dashboard-check`,
    `denominator-gate`, `figure-provenance`, `root-full`, `desktop`.
  - wave-26 gate-lane-c's own re-measurement (`ea0519a3c2`, this same `tranche/14` HEAD's direct
    ancestor): 37 PASS / 3 FAIL = `site-dashboard-check`, `denominator-gate`, `root-full` --
    **the identical 3** this cycle's live run reproduces, stage for stage.
  This run's failing set (`{site-dashboard-check, denominator-gate, root-full}`) is a **strict
  subset** of wave-28's failing set, and **identical** to wave-26-lane-c's. No stage present in
  either prior PASS list appears in this run's FAIL list. `figure-provenance` and `desktop`,
  both FAIL at wave-28, are confirmed independently PASS here (not a wave-28 regression this
  cycle introduced -- both fixes pre-date this cycle, per wave-26-lane-c/b's own receipts, and
  this run is the second independent confirmation of both).
- **Root-full's own failing set did not grow either**: wave-28 named 3 failing suites
  (`sd24_wired_integration_audit`, `sd27_pathfinder_unchained_cache_shape`,
  `v06_corpus_trap_report`); this run reproduces exactly 2 of those 3
  (`v06_corpus_trap_report`'s 4 failures are now fixed -- gate-lane-a's wave-26 baseline
  reconciler landed since, confirmed here by `corpus-trap-audit-selftest` PASS and
  `v06_corpus_trap_report` no longer appearing in `root-full`'s failure list at all).

**Gate is NOT claimed green.** 3 of 40 stages exit non-zero. This cycle's own obligation (an
honest full sweep, same-or-fewer red, zero new red) is met; the bundle's overall
`final-acceptance-scan` (kanban row 26, `AT-34-E6-001`) remains `not-started` and is not this
cycle's to close.

## 5. Baseline note -- `BASELINE_CORPUS_LITERAL_RECORDS`

The brief flagged `BASELINE_CORPUS_LITERAL_RECORDS 26500 -> 48708` as needing a **deliberate**
update. **That update already happened, twice, before this cycle started** -- both already
committed and on `origin/tranche/14` at the HEAD this cycle rebased onto:

1. `26500 -> 48708` (SD-34 gate-remediation closing sweep, 2026-09-01, per `fable-review.md`
   §7's own flag) -- `scripts/verify-baselines.env`, justified inline with the exact re-derive
   command and cross-checked against two independent sweeps landing on the same figure.
2. `48708 -> 48706` (wave-27 gate-lane-c, 2026-09-01) -- a real `-2` from two stale duplicate
   `class_feature` records (`bloodline_feat-2.json` + `draconic_bloodline-2.json`) deleted by
   gate-lane-a's own wave-26 cycle, re-pinned here because that cycle never updated this floor
   itself.

**This cycle's own live `corpus-sweep` measurement is `48706 records examined`, exactly matching
the currently pinned floor.** No stale note was printed for `BASELINE_CORPUS_LITERAL_RECORDS` in
this run's `BASELINE NOTES` section (only `BASELINE_ROOT_LIB_TESTS` and `BASELINE_DESKTOP_TESTS`
printed as stale -- see below) -- the number is exactly right, re-derived independently a third
time, and needs **no further update** this cycle. `scripts/verify-baselines.env` was not touched.

**Two other baseline floors printed STALE notes this run, deliberately left alone:**
- `BASELINE_ROOT_LIB_TESTS`: 2336 recorded, 3028 measured live (`root-lib` PASS).
- `BASELINE_DESKTOP_TESTS`: 515 recorded, 572 measured live (`desktop` PASS).

Both are growth (tests added, never a shrink -- the direction that is always safe), and both are
**out of this cycle's named scope** (the brief names only the corpus-literal-records baseline as
this cycle's to update), matching wave-26-lane-c's own precedent of leaving
`BASELINE_ROOT_TEST_BINARIES` similarly stale and named rather than silently folded in. Recorded
here as an open staleness note for whichever cycle owns it next.

## 6. What this cycle did NOT do

- Did **not** run `v06_work_inventory` for real (only `--summary`/`--check`, both inside
  `verify.sh`'s own read-only stages). `docs/work-inventory.json` untouched.
- Did **not** run `scripts/publish-site-dashboard.sh` for real. `site/dashboard/PF1e-dashboard.json`
  untouched.
- Did **not** fix any of the 3 remaining FAILs. All 3 are named precisely above with exact cause,
  already attributed to prior waves, and are not this cycle's to fix (a full sweep, not a
  remediation cycle, per the dispatch brief).
- Did **not** update `scripts/verify-baselines.env` -- the one baseline the brief named as a
  candidate for a deliberate update (`BASELINE_CORPUS_LITERAL_RECORDS`) was already correct at
  the value this run independently re-measured.
- Did **not** touch `kanban.md` -- no board row tracks individual gate-remediation waves; row 26
  (`final-acceptance-scan`) stays `not-started`, matching wave-23/25/26/27/28's own precedent.

## 7. Figures + their re-derive commands

| Figure | Value | Re-derive |
|---|---|---|
| Full sweep | 37 PASS / 3 FAIL | `bash scripts/verify.sh` at HEAD `d17c784ccd` |
| `root-full` | 8369 passed / 589 suites, 2 failing suites / 3 failing tests | same run, row 30; `/tmp/codex-verify-OCGrDe/root-full.log` |
| `corpus-sweep` | 48706 records examined, 0 findings | same run, row 33; `cargo run --locked --bin corpus_literal_sweep` |
| `denominator-gate` | violations=2, files_checked=145 | same run, row 20; `python3 scripts/denominator_gate.py --check` |
| `site-dashboard-check` | FAIL, exit 1, 600s internal timeout x2 | same run, row 8; `/tmp/codex-verify-OCGrDe/site-dashboard-check.log` |
| Wall time (this cycle's successful run) | ~1h53m (22:50:31 -> 00:43:21 EDT) | timestamps on `/tmp/codex-verify-OCGrDe/*.log` |

## 8. Verdict

This cycle's own obligation (a full, honest, independently-re-derived sweep, not a regeneration)
is **complete**. The bundle's gate itself is **not** green (3 of 40 FAIL); all 3 are pre-existing,
already named by prior waves, and confirmed here neither new nor grown. The bar the brief set
(same-or-fewer red than 14, zero green->red) is **met**, with the failing set actually the
smallest of any full sweep on record for this bundle (3, down from the review's original 14 and
wave-28's own 5).

**Commit at receipt time:** (filled in at commit)
