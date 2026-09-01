# Cycle — Epic 6 Closure epilogue / AT-34-E6-001 — Gate-Remediation Closing Sweep

**Not the final-acceptance scan** (that is `AT-34-E6-001_cycle_receipt.md`, verdict FAIL at attempt
1, separate criterion). This is the single closing cycle for the three territory-disjoint
gate-remediation lanes (`AT-34-E6-001_gate-lane-{a,b,c}_cycle_receipt.md`) fable-review.md §7
dispatched — a full `scripts/verify.sh` sweep over their combined landing, NOT a regeneration.
The dispatch brief was explicit that the inventory regenerator and the dashboard producer must not
be run directly from this cycle (both are silent stamp-droppers per the review's own hazard note);
neither was run by hand — the dashboard producer only ran as `site-dashboard-check`'s own
sub-invocation, which is `verify.sh`'s job, not mine.

- **Commit SHA:** see bottom (pushed after this receipt)
- **Files touched:**
  - `docs/retro/events/codex.jsonl` (pre-existing uncommitted cron append, committed in isolation
    before the rebase so the tree was clean — not this cycle's own content)
  - `scripts/verify-baselines.env` (one deliberate baseline update, see below)
  - `docs/release/SD-34-book-completion/artifacts/epic-6-closure/AT-34-E6-001_gate-sweep_cycle_receipt.md`
    (new, this file)

## What this cycle did

1. `git fetch origin tranche/14 && git rebase origin/tranche/14` — clean rebase, fast-forward onto
   the three lanes' work (`a4082f6836` gate-lane-C's post-rebase fixups at tip). Pushed immediately
   (`d5ee7f84f2`).
2. Ran `./scripts/verify.sh --full --show-actuals -j 8` in full, foregrounded, twice — the first
   run's underlying process was killed by an unrelated harness/task-limit issue partway through
   `root-full` (traced to a self-matching `pgrep -f "scripts/verify.sh --full"` in my own wait-loop
   command, which matched its own command line and looped forever, accumulating background wait
   tasks until something evicted the oldest — that dead loop was killed and the run was restarted
   cleanly with a PID-file-based wait instead). The **second run completed cleanly to RESULT: FAIL**,
   logs in `/tmp/codex-verify-cMdsw1`.
3. No regeneration was run: the inventory regenerator (`v06_work_inventory` outside of `verify.sh`'s
   own stages) and the dashboard producer were never invoked directly by this cycle.

## Stage table — full sweep, 40/40 stages executed

| Stage | Result | Stage | Result |
|---|---|---|---|
| preflight-disk | PASS | corpus-sweep-selftest | PASS |
| preflight-oracle | PASS | corpus-trap-audit-selftest | PASS |
| oracle-pin-selftest | PASS | root-lib | **PASS** (was FAIL) |
| producer-selftest | PASS | root-full | FAIL |
| pi-redaction-selftest | PASS | desktop | FAIL |
| provenance-selftest | PASS | reach | FAIL |
| site-dashboard-selftest | PASS | corpus-sweep | PASS |
| site-dashboard-check | FAIL | corpus-trap-audit | PASS |
| site-dashboard-pi-gate | PASS | supersession-gate | PASS |
| build-public-status-selftest | PASS | frontend-install | PASS |
| site-public-status-check | **PASS** (was FAIL) | frontend-test | **PASS** (was FAIL) |
| site-public-status-pi-gate | PASS | frontend-typecheck | PASS |
| site-asset-stamp-check | PASS | clippy | FAIL |
| reachability-audit-selftest | **PASS** (was FAIL) | class-dump | PASS |
| reachability-audit | **PASS** (was FAIL) | | |
| groundtruth-guard-selftest | PASS | | |
| supersession-gate-selftest | PASS | | |
| shape-coverage-standing-gate-selftest | **PASS** (was FAIL) | | |
| shape-coverage-standing-gate | **PASS** (was FAIL) | | |
| denominator-gate | **PASS** (was FAIL) | | |
| figure-provenance | **PASS** (was FAIL) | | |
| pi-sweep | **PASS** (was FAIL) | | |
| declared-pi-audit | PASS | | |
| audit-selftest | PASS | | |
| reclaim-selftest | PASS | | |
| driver-selftest | PASS | | |

**Totals: 35 PASS / 5 FAIL** (verify.sh's own SUMMARY line, not hand-counted).

## Bar check (per dispatch brief)

- **Same-or-fewer red stages than the review's 14:** 5 ≤ 14. **PASS.**
- **Zero stages that were green at baseline and are now red:** the 5 stages still failing —
  `site-dashboard-check`, `root-full`, `desktop`, `reach`, `clippy` — are **all five members of the
  original 14-red set** named in `fable-review.md` §7's table. No stage outside that set failed.
  **PASS — zero regressions.**
- 9 of the original 14 are now green: `reachability-audit-selftest`, `reachability-audit`,
  `shape-coverage-standing-gate-selftest`, `shape-coverage-standing-gate`,
  `site-public-status-check`, `root-lib`, `frontend-test`, `denominator-gate`,
  `figure-provenance`, `pi-sweep` — that's 10 names, but `site-public-status-check` was not in the
  review's row 5 (`site-dashboard-check`, `site-public-status-check` were listed together; only
  `site-dashboard-check` is still red) so it counts too. Net: 14 - 5 = **9 stages closed this
  wave.**

### Within the still-red stages, no widening either

- `root-full`: **7 failing suites** this run (`sd27_ability_automatic_granted_race_traits`,
  `sd27_book_license_record_counts`,
  `sd27_equipment_modifier_price_matches_corpus_cost_token`,
  `sd27_known_spells_must_be_on_the_class_spell_list`,
  `sd30_declared_product_identity_in_shipped_class_features`,
  `sd31_class_feature_corpus_key_uniqueness`, `v06_corpus_trap_report`), down from the review's
  baseline **47** — a real reduction, not a coincidence of scope. `8349 passed across 589 suites`
  (exit 101), zero unexpected new failures.
- `desktop` / `reach`: both fail on the same 2-3 `reach_gate::tests::*` cases
  (`unreached_records_are_exactly_the_recorded_findings`,
  `unsurfaced_families_are_exactly_the_recorded_findings`, `reach`'s narrower filter drops one) —
  consistent with the baseline's "companion_catalog + reach_gate tests broken by the same
  status/count movement" attribution; not a new failure shape.
- `clippy`: root 86 / desktop 25, matching the review's baseline figures exactly
  (`fable-review.md` §7: "clippy (root 86, desktop 25>7)") — unchanged, not worse.
- `site-dashboard-check`: same cause as baseline — `v06_work_inventory --summary` under
  `pf1e_dashboard_producer.py` times out at 600s (observed twice in this run's own log,
  `/tmp/codex-verify-cMdsw1/site-dashboard-check.log`), leaving `PF1e-dashboard.json` STALE. This
  is the exact hazard the dispatch brief named and told this cycle not to fix by regenerating —
  it is a pre-existing, already-attributed SD-34 remaining-card item.

## Baseline note — `BASELINE_CORPUS_LITERAL_RECORDS` 26500 → 48708 (updated)

The dispatch brief flagged this note and said to update it **only if I can state why the new
number is right.** I can:

- The `corpus-sweep` stage **PASSED** this run and printed its own MEASURED line (not a hand
  count — `verify.sh` only prints MEASURED for stages that pass, and this one did):
  `corpus-sweep: PASS (48708 records examined of 51482 read, 413336 tokens compared (9
  synthesized), 51469 digests checked, 0 findings)` → `MEASURED: BASELINE_CORPUS_LITERAL_RECORDS=48708`.
- This is not a one-off: `fable-review.md` §7 independently cited the identical figure
  (`26500→48708 stale`) from its own separate baseline/final sweeps
  (`/tmp/codex-verify-RWY6GT`, `/tmp/codex-verify-ziiG09`). Two independent sweeps on two
  different working trees landed on the same number.
- Per this file's own standing convention (`verify-baselines.env` header, "Test counts are
  FLOORS... a floor rising is the only safe direction"), and `verify.sh:1895-1902`'s own logic
  (`examined > BASELINE_...` only ever prints a STALE note and still PASSES; it never fails the
  gate on its own), raising this floor is a safe, deliberate, reviewable one-line move — not a
  fix for a red stage (`corpus-sweep` was already green at 26500).
- The corpus itself was not touched by this cycle or by the three gate-remediation lanes (their
  diffs are code/docs, not `data/corpus/**`); the +22208 predates this wave and reflects SD-33/
  SD-34 book-completion content that landed on tranche/14 earlier.
- Re-verified after the edit: `./scripts/verify.sh --only corpus-sweep --show-actuals` → `PASS`,
  same `48708 records examined`, **no STALE note printed** (confirming the floor now matches the
  measurement exactly).

**Update applied.** `scripts/verify-baselines.env` gained one new dated block; `BASELINE_ROOT_LIB_TESTS`
was deliberately left at its old value (2336) even though `root-lib` PASSED this run at 3022 —
raising that floor is out of this cycle's assigned scope (the brief named only the corpus-records
baseline) and is recorded in the new block as an open staleness note for whichever cycle owns
`root-lib`'s baseline next, not silently folded in alongside the one change I was told to make.

## Honest note: a self-inflicted delay, not a gate problem

The first `verify.sh --full` attempt was killed mid-`root-full` by what turned out to be my own
bug: a wait-loop `while pgrep -f "scripts/verify.sh --full"; do sleep 10; done` matches its OWN
command line (the pattern text is present in the loop's own invocation), so it never exits and
kept re-backgrounding itself across repeated 600s Bash timeouts, accumulating six live loop
processes before something (task-count pressure, most likely) reaped the actual `verify.sh` run
instead of one of the loops. Diagnosed by finding six live `sleep 10` processes and a `[killed]`
task marker with no reboot in `uptime`/`last -x`; fixed by killing the self-matching loops and
restarting `verify.sh` under `nohup` with its PID captured to a file, then polling that specific
PID (`kill -0 $PID`) instead of a text pattern for the rest of the run. No verify.sh stage, code,
or baseline was affected by this — it cost wall-clock time, not gate correctness. Recorded here so
the next cycle that writes a wait-loop over a command whose own invocation contains distinctive
text avoids the same self-match.

## Not done in this cycle (explicitly out of scope)

- No inventory regeneration, no dashboard-producer invocation outside of `verify.sh`'s own
  `site-dashboard-check`/`site-public-status-check` stages.
- No fix attempted for the 5 still-red stages — that is SD-34's remaining-card work per
  `fable-review.md` §7's own routing ("Turning these 14 green is SD-34's remaining-card work").
- `BASELINE_ROOT_LIB_TESTS` not raised (see above).
- `kanban.md` / `progress.md` not touched — no kanban row tracks this 3-lane gate-remediation wave
  as its own card (only row 26, `final-acceptance-scan`, AT-34-E6-001, `not-started`, which is the
  separate criterion this sweep feeds evidence into, not a card this cycle owns).

## Next-cycle plan

None named by this cycle beyond what `fable-review.md` §7 already routes: the 5 still-red stages
(`site-dashboard-check`, `root-full` [7 named suites], `desktop`/`reach` [`reach_gate::tests::*`],
`clippy` [root 86, desktop 25]) are SD-34's remaining-card work. The final-acceptance scan
(`AT-34-E6-001_cycle_receipt.md`) should re-derive this sweep's own headline numbers itself per
`acceptance-and-verification.md §3` obligation 2, rather than trust this receipt's figures.

---

**Commit SHA:** `<filled after commit — see `git log -1`>`
**Full sweep log:** `/tmp/codex-verify-cMdsw1` (second, completed run)
