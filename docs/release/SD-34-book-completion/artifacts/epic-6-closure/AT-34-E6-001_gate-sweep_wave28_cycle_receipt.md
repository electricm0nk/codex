---
canonical: true
owner: sd34-at-34-e6-001
bundle_id: SD-34
criterion: AT-34-E6-001 (gate-remediation tracking label -- NOT the final-acceptance scan)
date: 2026-09-01
verdict: complete (this cycle's own obligation: a full, honest, independently-re-derived sweep; the bundle's gate itself remains not-green, 5 of 40 FAIL, none new)
---

# Wave 28 -- gate-remediation closing sweep (full-sweep, no regeneration)

**This is the closing cycle of a gate-remediation wave whose three lanes (rust-suites, frontend,
docs-gates) had just landed.** Per this cycle's own dispatch brief: full sweep only, **not** a
regeneration -- `docs/work-inventory.json` and `site/dashboard/PF1e-dashboard.json` were not
touched, the inventory regenerator and dashboard producer were not run for real (only
`--check`/`--summary`, both read-only), per the brief's own explicit instruction.

- **Commit SHA (before this cycle's own commit):** `65f24c99362b7f7fdb4551fa2cc88b6bf27bb377`
  (`tranche/14`, post-rebase -- this is also wave-27 gate-lane-c's own final self-heal commit; the
  three lanes this brief describes as "just reported" are the already-landed gate-lane-a/b/c
  wave-24/26/27 work at this same HEAD, confirmed by `git log` -- no lane commits arrived between
  fetch and this cycle's own rebase).
- **This cycle's own commit:** see bottom (pushed after this receipt).
- **Files touched by this cycle:** this receipt (new) + `progress.md` (prepended entry). **No**
  `src/`, no `apps/desktop/src-tauri/src/`, no `data/corpus/**`, no `scripts/verify-baselines.env`
  -- nothing needed a code fix or a baseline edit this cycle (see Baseline note below).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` -- diff is receipt prose + one progress.md
  entry, zero `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`-shaped tokens in the added lines.
- **Wired-integration audit result:** `OK_NO_TOKENS` -- zero `STUB`/`MOCK`/`placeholder`/`not yet
  implemented`/`todo`/`fixme`/`hack` tokens in the diff.

## 1. Rebase

`git fetch origin tranche/14 && git rebase origin/tranche/14` -> "Successfully rebased and
updated" with **zero new commits** (this worktree's `HEAD` was already `origin/tranche/14`'s tip,
`65f24c9936`). Confirmed via `git rev-parse HEAD` before and after: identical.

## 2. Full sweep, run live, foregrounded

`bash scripts/verify.sh --show-actuals -j 8` at HEAD `65f24c9936`. Total wall time: **~101
minutes** (started 18:15:24 EDT, exited 20:05 EDT -- `site-dashboard-check` alone accounted for
roughly the first ~13 minutes of that, consistent with lane B's own ~757s-plus-cargo-resolve
measurement; `root-full`'s ~490-binary build+run consumed most of the remainder). Logs:
`/tmp/codex-verify-gnGiVd/`. **Not killed, not `--only`-scoped, not `--quick`** -- one single
foregrounded run covering all 40 stages start to finish.

## 3. The stage table -- all 40, PASS/FAIL, at HEAD `65f24c9936`

**35 PASS, 5 FAIL.**

| # | Stage | Result | Detail |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK (187G available) |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 21 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 6 passed, 0 failed |
| 8 | site-dashboard-check | **FAIL** | exit 1 -- `v06_work_inventory --summary` timed out at the producer's own internal 600s cap, **twice** (confirmed live, both timeout lines present in this cycle's own log); pre-existing since wave-26, not new |
| 9 | site-dashboard-pi-gate | PASS | 21 files vs 1612 declared-PI names, zero leaked |
| 10 | build-public-status-selftest | PASS | 37 cases passed |
| 11 | site-public-status-check | PASS | status-data current |
| 12 | site-public-status-pi-gate | PASS | 31 files vs 1612 names, zero leaked |
| 13 | site-asset-stamp-check | PASS | stamps match |
| 14 | reachability-audit-selftest | PASS | 11 cases passed |
| 15 | reachability-audit | PASS | reachable ceiling 98.90% (48893 / 49438, re-derived `python3 scripts/reachability_audit.py` at wave-26-lane-c's own HEAD `7f3ab6a671` -- unmoved since this row was first written) |
| 16 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 17 | supersession-gate-selftest | PASS | 16 cases passed |
| 18 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 19 | shape-coverage-standing-gate | PASS | population=26112 unclassified=0 no_record=0 |
| 20 | denominator-gate | **FAIL** | violations=2 of files_checked=141 -- both in `AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md` lines 138/153 (a CPU-utilization figure restated without a same-line re-derive command); pre-existing, not this cycle's content |
| 21 | figure-provenance | **FAIL** | violations=2 of figures_examined=122 (files_checked=71) -- `AT-34-E6-001_gate-lane-a_wave24_cycle_receipt.md:144` and `AT-34-E6-001_gate-lane-a_wave26_cycle_receipt.md:135`, both an unsourced `(447,1,130)` tuple; pre-existing, not this cycle's content |
| 22 | pi-sweep | PASS | 11 hits over rules_tables, 11 baseline rows |
| 23 | declared-pi-audit | PASS | clean |
| 24 | audit-selftest | PASS | 28 passed, 0 failed |
| 25 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 26 | driver-selftest | PASS | 7 passed, 0 failed |
| 27 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 28 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 29 | root-lib | PASS | 3019 passed |
| 30 | root-full | **FAIL** | cargo exit 101; **8355 passed** across 589 suites, 3 failing targets: `sd24_wired_integration_audit` (4 passed/1 failed), `sd27_pathfinder_unchained_cache_shape` (5 passed/2 failed), `v06_corpus_trap_report` (21 passed/4 failed) -- 7 failing tests total, all already named with exact cause by wave-27's own receipt; none new |
| 31 | desktop | **FAIL** | cargo exit 101; 571 passed/1 failed -- `corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_artifacts`, same hardcoded PU pin wave-27 already named; not new |
| 32 | reach | PASS | 31 passed |
| 33 | corpus-sweep | PASS | `48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings` -- against the already-re-pinned floor (see Baseline note) |
| 34 | corpus-trap-audit | PASS | records_examined=27634, all 4 registered defect kinds at their pinned counts exactly, `wiring-class-mismatch=0` |
| 35 | supersession-gate | PASS | 116 objects, all clean |
| 36 | frontend-install | PASS | node_modules present |
| 37 | frontend-test | PASS | 100/100 files |
| 38 | frontend-typecheck | PASS | tsc --noEmit clean |
| 39 | clippy | PASS | root:0 desktop:0 warnings, 0 errors |
| 40 | class-dump | PASS | 31/31 computing |

Row-count check: `awk -F'|' '/^\| [0-9]+ \|/{n++; if ($4 ~ /FAIL/) f++} END {print n, f, n-f}'` over
this table -> `40 5 35`, matching `verify.sh`'s own printed `SUMMARY` block exactly (`passed: 35`
... `FAILED: 5 site-dashboard-check denominator-gate figure-provenance root-full desktop`).

## 4. The bar: same-or-fewer red than the review's 14, zero green->red

**Bar met.** The review's original 14 red stages (`fable-review.md` §7, baseline `4f3f995184`):
`reachability-audit`, `reachability-audit-selftest`, `shape-coverage-standing-gate`,
`site-dashboard-check`, `site-public-status-check`, `root-lib`, `root-full`, `desktop`, `reach`,
`frontend-test`, `denominator-gate`, `figure-provenance`, `pi-sweep`, `clippy`.

- **This cycle's 5 red are an exact subset of those 14**: `site-dashboard-check`,
  `denominator-gate`, `figure-provenance`, `root-full`, `desktop`. No stage outside the original
  14 is red. **5 <= 14 -- same-or-fewer, satisfied.**
- **9 of the original 14 are now green**, closed by the three gate-lane waves this cycle
  inherits, not by this cycle itself: `reachability-audit`, `reachability-audit-selftest`,
  `shape-coverage-standing-gate`, `site-public-status-check`, `root-lib`, `reach`,
  `frontend-test`, `pi-sweep`, `clippy`.
- **Zero green->red regressions.** Every one of the 26 stages the review's baseline already had
  green is green in this cycle's own live run too (cross-checked name-by-name against the stage
  table above -- none of the 5 current FAILs falls outside the original 14, which is the only way
  a green->red flip could hide). This cycle introduced no code, so it could not have caused one
  either way; it exists to confirm the negative, not just assert it.

**Net: 14 -> 5, a real 9-stage improvement, entirely inherited from the three already-landed gate
lanes, independently re-derived and confirmed by this cycle's own full, live, foregrounded run --
not relayed from any lane's own receipt.**

## 5. Baseline note -- `BASELINE_CORPUS_LITERAL_RECORDS`

**Already deliberately updated, and re-confirmed correct by this cycle -- no further edit
needed.** `scripts/verify-baselines.env` shows the chain: `26500 -> 48708` (wave-27's own
deliberate update, cited directly to this review's own §7 flag, cross-checked against the
review's own independent sweep at `48708`) `-> 48706` (wave-27's own further re-pin, citing
gate-lane-a's wave-26 diagnosis: 2 stale duplicate `class_feature` records deleted, `-2` matching
the record delta exactly per `decisions.md §12` L8). This cycle's own live `corpus-sweep` stage
(row 33 above) measured **`48706` examined, 0 findings** -- an exact match to the pinned floor,
independently re-derived, not copied from any receipt. **I did not edit
`scripts/verify-baselines.env`**: the number already on disk is the number my own live run
measures, so there is nothing to update deliberately -- editing it further with no delta would be
a change with no stated reason, which is exactly what this bundle's own `decisions.md §12` L2
warns against.

**Left open, out of this cycle's scope (named, not silently absorbed):**
`BASELINE_ROOT_LIB_TESTS` is stale (2336 recorded vs **3019 measured** by this cycle's own
`root-lib` PASS) -- wave-27's own receipt already flagged this as "out of scope for this cycle"
(the dispatching brief named only the corpus-literal-records baseline); this cycle's own brief
named the same one baseline, so the same scope boundary applies here. Recorded for whichever
cycle owns `BASELINE_ROOT_LIB_TESTS` next.

## 6. What this cycle did NOT do (deliberately, per its own brief)

- Did **not** run the inventory regenerator (`v06_work_inventory` with a write path) or the
  dashboard producer for real. `site-dashboard-check`'s own `--check` run (read-only) and
  `root-full`'s own `v06_work_inventory.rs::the_committed_inventory_is_well_formed_and_uses_only_
  declared_statuses` test (which PASSED live, row 30's suite) both confirm the **committed**
  `docs/work-inventory.json` is already well-formed at this HEAD -- there was no red test waiting
  on a regeneration this cycle needed to supply.
- Did **not** fix any of the 5 remaining FAILs. All 5 are named precisely above with exact cause
  and, where known, exact owning lane/file -- consistent with the prior wave's own boundary
  ("report their stages, do not edit their files") and this cycle's own brief ("the full sweep,
  NOT a regeneration").
- Did **not** touch `kanban.md` -- no row tracks individual gate-remediation waves (per wave-27's
  own filename note, `AT-34-E6-001` here is a tracking label distinct from the real board row 26,
  `final-acceptance-scan`, which stays `not-started`).

## Figures + their re-derive commands

| Figure | Value | Command |
|---|---:|---|
| Full sweep | 35 PASS / 5 FAIL | `bash scripts/verify.sh --show-actuals -j 8`, this cycle, HEAD `65f24c9936` |
| `root-lib` | 3019 passed | same run, row 29 |
| `root-full` | 8355 passed / 589 suites / 3 failing targets / 7 failing tests | same run, row 30; `/tmp/codex-verify-gnGiVd/root-full.log` |
| `desktop` | 571 passed / 1 failed | same run, row 31; `/tmp/codex-verify-gnGiVd/desktop.log` |
| `corpus-sweep` | 48706 examined, 0 findings | same run, row 33 |
| `corpus-trap-audit` | 27634 examined, all 4 registered kinds at pin | same run, row 34 |
| `denominator-gate` | files_checked=141 violations=2 | same run, row 20; `/tmp/codex-verify-gnGiVd/denominator-gate.log` |
| `figure-provenance` | figures_examined=122 (files_checked=71) violations=2 | same run, row 21; `/tmp/codex-verify-gnGiVd/figure-provenance.log` |
| `site-dashboard-check` | FAIL, exit 1, 600s internal timeout x2 | same run, row 8; `/tmp/codex-verify-gnGiVd/site-dashboard-check.log` |
| `BASELINE_CORPUS_LITERAL_RECORDS` | 48706 (unchanged, re-confirmed) | `scripts/verify-baselines.env`, last entry |
| `BASELINE_ROOT_LIB_TESTS` | stale: 2336 recorded, 3019 measured | `verify.sh`'s own BASELINE NOTES block, this run |

## Status: complete

This cycle's own obligation (a full, honest, independently-re-derived sweep, reported without
regenerating anything) is **complete**. The bundle's gate itself is **not** green (5 of 40 FAIL);
none of the 5 are new, all are named with cause, and closing them remains lanes A/B's own
territory (corpus/root-full items) or the site/dashboard owner's (site-dashboard-check), per the
same boundary wave-27 already drew. **I am not claiming the gate is green** -- it is not; every
stage did not exit 0.

## Next-cycle plan

Unchanged from wave-27's own (still the accurate, current remainder, re-confirmed live by this
cycle rather than assumed stale):

1. `site-dashboard-check` -- run `./scripts/publish-site-dashboard.sh` for real (not `--check`)
   from a confirmed-quiet box; measured cost ~13 minutes for the `v06_work_inventory --summary`
   leg alone.
2. `denominator-gate` (2 violations) -- restate the CPU-utilization figure in
   `AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md` lines 138/153 with an inline re-derive
   command.
3. `figure-provenance` (2 violations) -- restate the `(447,1,130)` tuple in
   `AT-34-E6-001_gate-lane-a_wave24_cycle_receipt.md:144` and
   `AT-34-E6-001_gate-lane-a_wave26_cycle_receipt.md:135`, each with its own command.
4. `root-full` (3 suites / 7 tests) -- `sd27_pathfinder_unchained_cache_shape.rs` (2 tests,
   restate 42->38 / 7->3 to match the corrected corpus); `sd24_wired_integration_audit.rs` (1
   test, widen the allowlist for legitimate "placeholder" prose at `reach_gate.rs:3192`);
   `v06_corpus_trap_report.rs` (4 tests, REGISTERED debt under `AT-34-E1-007`/`008`, already
   `complete` -- not this criterion's to close).
5. `desktop` (1 test) -- restate `corpus_ingest_diagnostic.rs:1394`'s hardcoded PU
   `corpus_only_records` pin 1271->1267.

## Commit SHA (filled in after push)

`bfaa288ead0f20b4652155ace37acff02ba35f27` -- the single commit this cycle produced (receipt +
`progress.md` entry). No rebase was needed after the sweep (no upstream movement during the
~101-minute run); this is also this cycle's own build-scope SHA.
