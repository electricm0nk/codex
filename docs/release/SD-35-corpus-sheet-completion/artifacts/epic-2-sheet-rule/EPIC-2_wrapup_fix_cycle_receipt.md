# Cycle EPIC-2-WRAPUP-FIX — Epic 2, Sheet rule / wrap-up correction cycle

The correction cycle `workflow-instruction.md §10` step 0 requires after a red epic-wrap-up
gate. The gate itself was run by an **isolated read-only worker** that pushed and committed
nothing (`decisions.md §3`'s worker split); its report is
`EPIC-2_wrapup_gate_report.md`, committed by **this** cycle. This cycle runs LOCAL on the
shared checkout and does commit and push.

- **Commit SHA:** `e0280a8fea` (all work — the three stage fixes, the `reclaim.sh` control, the
  five baselines, the regenerated feed, and the gate worker's hand-off), and the commit carrying
  this receipt with `kanban.md` / `progress.md`. Cycle start `c62ac91e10`.
- **Scope gate:** `SCOPE_GATE: EXEMPT (wrap-up correction cycle)` — `decisions.md §2`'s named
  exemption; a wrap-up fix cycle closes zero units by design. **Not** exempt from the residue
  check, which was run at start and at end.
- **Files touched:**
  - `scripts/tests/test_reachability_audit.py` (the stale live-figure pin, re-pinned on the property)
  - `scripts/reclaim.sh`, `scripts/tests/test_reclaim.py` (the disk-full incident-key defect, TDD)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/AT-35-E2-005-DISPOSITION_cycle2_receipt.md` (four wrapped figures re-flowed)
  - `site/dashboard/PF1e-dashboard.json`, `site/dashboard/PF1e-dashboard.json.last-good`, `site/dashboard/units/*.json`, `site/status-data.json`, `site/status-data/*.json` (regenerated feed)
  - `scripts/verify-baselines.env` (five stale floors raised to this run's measured actuals)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/EPIC-2_wrapup_gate_report.md` (the worker's hand-off, committed here)
  - `docs/retro/events/at-35-e2-wrapup.jsonl` (the worker's hand-off), `docs/retro/events/at-35-e2-wrapup-fix.jsonl` (this cycle's)
  - `docs/release/SD-35-corpus-sheet-completion/kanban.md`, `progress.md`, this receipt
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS — no shipping code path was added; the two
  code changes are a self-test assertion and an operational script's event-type decision, both
  fully wired and both proven by a test that was RED first.
- **Acceptance criterion:** this cycle has no criterion of its own. Its bar is
  `workflow-instruction.md §10` step 0 verbatim: *"The full gate, once, on the isolated
  worker… Any red stage is fixed in a wrap-up correction cycle before the next epic's second
  cycle dispatches; the fix cycle is exempt from the batch floor (`decisions.md §2`'s
  exemption), never from the residue check."*
- **Receipt rows (mechanical):** `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a
  builds_recorded=0 pcgen_live_files=260` —
  `git show c62ac91e10:docs/work-inventory.json > /tmp/wi-before.json && python3 scripts/cycle_scope_gate.py --receipt --since c62ac91e10 --before /tmp/wi-before.json --after docs/work-inventory.json --target-dir /tmp/cargo-sd35-AT-35-E2-WRAPUP-FIX`
  (also `regressed=0 added=0 dropped=0`). `rust_lines_changed=0` is exact: this cycle touched
  no `*.rs` at all.
- **PCGen residue:** `live_files=260 live_hits=12736 baseline_files=260 baseline_hits=12736 verdict=PASS`
  — `python3 scripts/pcgen_residue_gate.py --check`, identical at cycle start and cycle end,
  and confirmed a third time by the full gate's own `pcgen-residue-gate` stage. Not above the
  previous receipt's.
- **Oracle parity:** N/A — no Number mapping added, no live path touched.
- **Movement, four buckets:** closure **0** / relabel **0** / reachability **0** /
  instrument-correction **3** (the reachability pin, the reclaim event type, the four wrapped
  figures) **+ 1 data refresh** (the published dashboard feed).
- **Refused tokens:** none.
- **Discoveries:** none outside `token-coverage.json` and the atlas. Two findings about the
  **instruments**, both recorded as `correction` events and both fixed here rather than
  reported — see §2.
- **Status:** complete
- **Next-cycle scope:** Epic 2 wrap-up closed. Epic 3's second cycle is unblocked
  (`workflow-instruction.md §10` step 0's gating condition is satisfied).

---

## 1. What the gate found, and what each fix actually was

The worker's gate at `a542652c5e` was **FAIL — 48 stages, 45 PASS / 3 FAIL**, wall clock
1:39:21. Every one of the three was verified against the tree before being fixed; all three
reproduced.

### 1.1 `site-dashboard-check` — the published feed was 43 points stale

`./scripts/publish-site-dashboard.sh --check` →
`site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh`
(reproduced at `c62ac91e10`, not merely inherited from the worker's tree). Epic 2's conversion
moved every doneness figure and the committed feed had never been regenerated: the site's own
headline went **51.4% → 94.3%** of the 37,880 rated items on the refresh
(`git diff -- site/status-data.json` → `"pct": 51.4` → `"pct": 94.3`; `"done": 19454` →
`"done": 35722` over an unchanged `"denominator": 37880`). The last commit to touch the feed
was `2a00af8439`, an **SD-34 wave-51** gate — `git log --oneline -5 -- site/dashboard/PF1e-dashboard.json`.

Fix: `./scripts/publish-site-dashboard.sh` (real time 1m14.744s), which also regenerated
`site/status-data.json` and its 30 book-detail files (`30 books, overall 94.3%` of 37,880 rated items; 46,074 items in all).
53 files, `28761 insertions(+), 29551 deletions(-)` — `git diff --stat -- site/`. The
`site-dashboard-pi-gate` and `site-public-status-pi-gate` stages both passed afterwards, so the
refresh leaked nothing.

### 1.2 `reachability-audit-selftest` — a stale live-figure pin, re-pinned on the PROPERTY

The failure was 1 of 11: `test_real_inventory_ambiguous_is_the_known_no_done_path_class`,
`self.assertIn("ambiguous", no_done)` → `AssertionError: 'ambiguous' not found in set()`.

**The engine is right and the pin was wrong.** `ambiguous` is still a live wiring class
carrying **545** units, but **339** of them are now `sheet-complete`, and
`_doneness_verdict_uncapped("ambiguous", "sheet-complete")` → `done` — AT-35-E2-003's rung,
which is precisely the sheet rule (an unresolvable record renders as the rule's words and the
unit is done). The dead-end set is now empty and the live `reachability-audit` stage passes at
a **100.00%** ceiling of the 49,438-unit inventory. Re-derive:

```
python3 -c "import sys; sys.path.insert(0,'scripts'); import reachability_audit as a; \
d=a.load_inventory(a.DEFAULT_INVENTORY); r=a.audit(d); \
print(r['reachable_ceiling'], len(r['dead_end_cells']), r['dead_end_unit_total'])"
```
→ `1.0 0 0`.

This is the **fourth** SD-34-era live-figure equality pin of the exact shape AT-35-E1-002
already corrected in three other instrument test files. **Copying today's empty set into the
assertion would simply re-arm the same trap in the other direction**, so the test is re-pinned
on the property the audit exists to enforce:
`test_real_inventory_strands_no_wiring_class_that_carries_units` asserts that **no dead-ended
wiring class carries on-board units** — true however the classes are named, red only when a
real dead end exists. A second property test replaces the old `0 < ceiling <= 1` bound with the
arithmetic identity `ceiling == round(1 - dead_end_unit_total/total, 6)` on the live corpus, so
a ceiling that drifts from its own dead-end total is caught. The synthetic
`test_a_dead_ended_class_is_reported_but_does_not_fail_ok` case is **kept** (it is the
prove-it-can-fail exercise for the `no-done-path` path) with its docstring corrected to say
that its vocabulary deliberately omits `sheet-complete`.

`python3 -m unittest scripts.tests.test_reachability_audit` → **`Ran 11 tests … OK`** (11
before, 11 after — one test replaced, one strengthened, none removed).

### 1.3 `figure-provenance` — four figures whose command wrapped onto the next line

`violations=4 of figures_examined=194`, all four in
`AT-35-E2-005-DISPOSITION_cycle2_receipt.md` (lines 117, 121, 126, 130). The worker's
diagnosis is confirmed: every one of the four **does** carry a re-derive command, but on the
following line, and `denominator_gate.find_provenance_violations`
(`scripts/denominator_gate.py:413-451`) accepts a command only when it is reachable on the
**same line** as the figure. The fix re-flows those four bullets so each figure-bearing line
carries its own command; the fourth also gained an explicit re-derive clause for the three
denominators stated in its parenthetical. **No figure changed value and the gate was not
widened.**

`python3 scripts/denominator_gate.py --check-provenance` → `files_checked=165
figures_examined=198 violations=0` (was `files_checked=163 figures_examined=194 violations=4`;
the file count rises by 2 because this cycle commits the worker's report and this receipt).
`python3 scripts/denominator_gate.py --check` → `files_checked=235 violations=0`.

---

## 2. The recurrence the retro named, and the mechanism built for it

The worker's `retro.py summary --since 2026-09-07 --json` read **208 events / 90 commits**,
verification fail rate **0.1119** (15 of 134), and the failing-stage histogram
**figure-provenance 10, site-dashboard-check 4, shape-engine-boundary-selftest 1** — *two of
the three stages red in this gate had already been red 14 times between them during the epic
with no cycle owning either.* Both are now owned and both are closed here.

**The one incident key firing 3+ times was `disk-full`, 12 times — and all 12 were false.**
Verified independently of the worker: `scripts/reclaim.sh`'s `emit_retro_event()` emitted
`incident --recurrence-key disk-full` on **every** `--apply` run that reclaimed anything, with
no reference to disk pressure of any kind, and never populated `used_percent` (which is already
a typed field — `scripts/retro.py:86` `NUMBER_FIELDS`). On the 4-hourly cron that manufactured
~6 "incidents" a day out of a control **working as designed**, while `df -h /` sat at 60% of 1,500 GB used,
594G free, and `verify.sh`'s `preflight-disk` stage passed.

`AGENTS.md` rule 8 says recurrence is data and a key firing more than a handful of times is a
missing mechanism. That reading only works if the counts mean something, and here the repo's
single most serious recurrence key — tranche/7's 120-firing catastrophe — was spending itself
on self-noise and burying every key that was real. **This is exactly a warning-not-a-control
situation inverted: the control existed and worked; its telemetry lied about it.**

Fix, TDD, RED first:

- `scripts/tests/test_reclaim.py` `RetroEventTests` gained
  `test_apply_under_disk_pressure_is_the_disk_full_incident`,
  `test_apply_without_disk_pressure_is_a_note_not_a_disk_full_incident` and
  `test_the_pressure_threshold_is_configurable_and_inclusive`. First run: **`Ran 4 tests …
  FAILED (failures=2)`**, failing for the intended reasons (`'"used_percent": 97' not found`;
  `'"type": "incident"' unexpectedly found`).
- `scripts/reclaim.sh` gained `disk_used_percent()` (reads `df -P "$REPO_ROOT"`, with a
  `RECLAIM_USED_PERCENT_OVERRIDE` hook that exists only so the self-test can drive both
  branches on a box whose real disk it cannot set — the same overridable-for-tests convention
  the script already uses for its scan roots) and `RECLAIM_PRESSURE_PERCENT` (default **90**).
  At or above the threshold a run fired **under pressure** and stays `incident` /
  `recurrence-key disk-full`, now with the measured `used_percent` on the event. Below it the
  run is the control working and is a `note` tagged `reclaim-routine`. The header comment that
  documented the old unconditional behaviour was rewritten to match.
- `python3 -m unittest scripts.tests.test_reclaim` → **`Ran 23 tests … OK`**; the gate's own
  `reclaim-selftest` stage → `13 passed, 0 failed`.

**Named but below threshold** (carried forward, not fixed here — it is a dispatch-protocol
matter, not a repo defect): `duplicate-criterion-dispatch=2` (AT-35-E1-001 and AT-35-E1-005
each re-dispatched after already landing). Its control is `workflow-instruction.md §2.4`'s own
rule — rebuild criteria from `kanban.md` excluding `complete` — applied at **every** relaunch,
not only the quota relaunch.

**Ratio review (`decisions.md §4`), from the worker's report and re-stated here:** no Epic 2
cycle exceeded 3.0. The only cycle with a denominator, AT-35-E2-005 c1, scored **0.02**
(337 lines / 21,911 units); epic-wide **9,475** Rust lines / **21,911** closed = **0.43**. The
four `ratio=n/a` cycles that changed Rust with `closed=0` are itemised in the worker's report
§2b. This cycle's own ratio is `n/a` at `rust_lines_changed=0`.

---

## 3. The five stale baselines

`scripts/verify-baselines.env`'s test counts are **floors**: exceeding one prints a STALE
notice and still passes, so none of these five was a failure. They are raised here in the same
correction commit, to **this** run's measured actuals rather than to the worker's — the tree
moved between the two runs and three of the five numbers differ.

| Key | Was | Worker measured | **This run measured** |
|---|---:|---:|---:|
| `BASELINE_ROOT_LIB_TESTS` | 3186 | 3217 | **3220** |
| `BASELINE_ROOT_FULL_TESTS` | 8656 | 8721 | **8724** |
| `BASELINE_ROOT_TEST_BINARIES` | 408 | 411 | **411** |
| `BASELINE_DESKTOP_TESTS` | 573 | 574 | **574** |
| `BASELINE_FRONTEND_TEST_FILES` | 100 | 101 | **101** |

Every move is **upward**. No floor was lowered and no ceiling was raised: the one deliberate
lowering this file records remains AT-35-E1-003's 589 → 408 test-binary fold.

---

## 4. Figures + their re-derive commands

- gate result, 48 stages — `scripts/verify.sh` (full, no `--only`),
  `CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E2-WRAPUP-FIX CARGO_INCREMENTAL=0`; console log
  `/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/c57a7080-1e94-4af6-9023-fa4f4f21e03f/scratchpad/verify-e2-wrapup-fix.log`, stage logs `/tmp/codex-verify-P7QLe3` → `RESULT: PASS`, `passed: 48`
- reachability ceiling **1.0**, dead-end cells **0**, dead-end units **0** of **49,438** —
  `python3 scripts/reachability_audit.py` → `reachable ceiling 100.00%` of 49,438 units
- `ambiguous` units **545**, of which **339** `sheet-complete` — `python3 -c "import sys,collections; sys.path.insert(0,'scripts'); import reachability_audit as a; d=a.load_inventory(a.DEFAULT_INVENTORY); print(collections.Counter(u.get('wiring_class') for u in d['units'])); print(collections.Counter(u.get('status') for u in d['units'] if u.get('wiring_class')=='ambiguous'))"`
- provenance violations **0 of 198** figures over **165** files — `python3 scripts/denominator_gate.py --check-provenance`
- denominator violations **0 of 235** files — `python3 scripts/denominator_gate.py --check`
- PCGen residue `live_files=260 live_hits=12736` of `baseline 260 / 12736` — `python3 scripts/pcgen_residue_gate.py --check`
- receipt rows `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=260` — the `cycle_scope_gate.py --receipt` invocation quoted above
- site feed **94.3%** overall of **37,880** rated items — `./scripts/publish-site-dashboard.sh` → `30 books, overall 94.3% … 46074 items total`
- retro events emitted by this cycle: **3** `correction` — `python3 scripts/retro.py query --actor at-35-e2-wrapup-fix` (`wc -l docs/retro/events/at-35-e2-wrapup-fix.jsonl`)
- the worker's hand-off: **6** events in `docs/retro/events/at-35-e2-wrapup.jsonl` — `wc -l docs/retro/events/at-35-e2-wrapup.jsonl`. The worker's report says 5; the sixth is the `verification` event `verify.sh` emitted on its own account, which the worker did not count. Its own `python3 scripts/retro.py validate` reports **zero** problems on this shard.

- **Build scope verified:** full `scripts/verify.sh`, all 48 stages, run once at
  `c62ac91e10` + this cycle's working tree. Result: **`RESULT: PASS` — 48 of 48 stages green, 0 red**; per-stage logs `/tmp/codex-verify-P7QLe3`. `root-lib` 3220 passed; `root-full` 8724 passed across 411 suites, all 361 `tests/*.rs` executed, 0 failed; `desktop` 574; `reach` green; `clippy` root:0 desktop:0; `corpus-sweep` 0 findings of 48,706 records examined; `sheet-rules-check` `records=49438 converted=48601 refused=837 verdict=PASS`; `class-dump` 31/31 computing; `frontend-test` 101/101.
- **Sweep population:** N/A — no corpus literal sweep in scope.
- **Oracle pin:** N/A.
- **Notes:** the worker's per-stage logs at `/tmp/codex-verify-4HYI3n/` were kept for this
  cycle and were read where a stage's one-line summary was not enough. Both hand-off files were
  found in the session scratchpad exactly as described and were committed unmodified.
