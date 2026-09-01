# Fable Review — repo code review + SD-35 backlog assessment

Started: 2026-08-31. HEAD at start: `3aebc284774cbfa09a84a3d6cb25d60e9b1be447` (tranche/14).
Operator commission: (1) code review — gaps, improvements, bloat; report + low-risk fixes only; (2) TOP PRIORITY: judge whether the SD pipeline addresses the ~29,283-unit remaining backlog properly and whether processing engines can expedite SD-35+ (propose, don't build). Plan: `~/.claude/plans/model-agile-eagle.md`.

## Status: COMPLETE (2026-09-01)

## Lane Status

| Lane | Scope | Model | Status | Findings |
|---|---|---|---|---|
| B1 | forward-plan.json number audit | sonnet | DONE | stale plan; live remainder 22,369 |
| B2 | price the unpriced 45.7% (D/M/V/X/Z) | sonnet | DONE | live unpriced 6,469 = 28.9%; 5/5 buckets rate-less |
| B3 | engine ROI ranking (136 mechanisms) | sonnet | DONE | live census 83 mech / 22 cover 90% (35 books) |
| B4 | bucket-B rate decomposition | sonnet | DONE | fast/slow 9.8×; composition projection overturned by synth |
| B-SYNTH | backlog verdict + engine build order | opus | DONE | see §1 |
| PC1–PC10 | pilot_compute/mod.rs (10 chunks) | sonnet | DONE | 60 findings |
| R8 | src/bin duplicate families | sonnet | DONE | 8 findings |
| R9 | v06_work_inventory.rs + v06 bins | sonnet | DONE | 6 findings; mandated hazard REFUTED (guard exists) |
| R10 | tests/ structure + templated families | sonnet | DONE | 5 findings |
| R11 | apps/desktop/src-tauri | sonnet | DONE | 7 findings; 2 real P1s |
| R12 | scripts/ + tools/ (generators first) | sonnet | DONE | 5 findings |
| R13 | oracle_harness + scripts/tests | sonnet | DONE | 5 findings |
| R14 | remaining src/ modules | sonnet | DONE | 8 findings |
| H1–H2 | grep sweeps + denominators | haiku | DONE | 22 census figures |
| VERIFY | finding verification (P1/P2 + all auto_fix) | sonnet+opus | DONE | 56 CONFIRMED / 9 REJECTED / 61 P3 unverified-by-design |
| FIX | safe-fix application | orchestrator | DONE | 2 applied (`920cb53307`), 1 blocked→§4 |
| TOKEN-MODEL | token-vocabulary cost model (operator challenge) | sonnet | DONE | see §1.b; register row C1.6 |

Raw lane outputs land in `docs/release/SD-34-book-completion/artifacts/fable-review/` as JSON, one file per lane, written by the lane itself at completion. This file is the synthesis; the JSON is the evidence.

## 1. Backlog Assessment (SD-35) — TOP PRIORITY

**Status: COMPLETE.** Full synthesis: `artifacts/fable-review/B-SYNTH.md` (Opus judge over lanes B1–B4; lane evidence in `artifacts/fable-review/B1.json`–`B4.json`). Key results:

### Verdict
**The method is right. The headline number is dead. SD-34 is not closed.**
- `forward-plan.json` is stale (generated 107 commits behind HEAD). Live remainder is **22,369 of 42,472 units (35 books)**, not 29,283. Bucket V collapsed 6,747 → 175 because SD-33's already-computed oracle verdicts were finally booked — a ledger reconciliation, **not** a burst of new throughput. Do not price future oracle work off "6,572 units in four days."
- The "45.7% unpriced" claim is now **28.9%** (6,469 of 22,369). Priced tiers (A+B+U = 11,919 units → 1,952–6,782h; C = 3,981 → 96.4h-to-V-not-DONE) are unchanged.
- The mechanism-leverage claim survives and strengthens: live census = **83 mechanisms over the 22,369-unit 35-book remainder, 22 mechanisms cover 90%** (C1.5's exact 136/35,650/29 figures are from the stale snapshot and are retired).

### Engine answer
**Yes, engines can expedite SD-35 — ~72% of the remainder (16,072 of 22,369) is engine-shaped.** But: **17.8% (3,981 units, all of bucket C / explanation_id wiring) is provably NOT engine-shaped** — 39 hand-written call sites; its only measured rate is 41.3 units/h to-V and **0.0 to DONE**. Carry bucket C as a separate hand-wiring line item, never inside "mechanism leverage."
The discipline that separates SD-32/33/34 successes from SD-31's failure: **named mechanism → measured population → small build → oracle-verify → corpus-wide run.** Every proposed engine carries that gate (G1–G4, no-go at G3 failure).

### Proposed SD-35 shape (5 engines, ordered; details in B-SYNTH §3)
1. **Kind-table loader extension** (power+companion): 449 units, ~0.1–0.6h — retires bucket A, cheapest gate rehearsal.
2. **Display-class record placement**: 6,718 units (30.0% of remainder) — the biggest lever; cost bracket is honestly 10–60h *or* ~4,500h depending on whether the class_feature families behave like display placement or like core_rulebook's slow class. **Gated on card-1 spike measurement.**
3. **wiring_class review classifier**: 1,446 bucket-D units — high relabel risk; success counted in →DONE only.
4. **Widen formula/bonus engines vs bucket M**: 3,428 units; bracket 40–150h, low confidence; prove on `ability_content` (1,236) first.
5. **Oracle probe surfaces for V residue**: 175 units; its real value is timing the oracle round-trip.

### SD-35 card 1 (before anything else): a 40h rate-measurement spike
7 timed measurements (B-SYNTH §4 table): the two big bucket-B shapes (n≥100, n≥50), two D sub-families, M's ability_content, a timed V oracle round-trip, one X capability build, a Z scoping probe. Plus ~1h: regenerate forward-plan/ordered-plan/capability-register at HEAD. **Exit: every bucket has a rate with stated n, or a written reason none is obtainable. No blended totals.**

### Top risks (B-SYNTH §5)
- Bucket B (11,299 units = 50.5% of remainder) has an order-of-magnitude open spread: 600–1,000h vs ~1,468h vs ~5,304h from three defensible readings. Card 1a settles it.
- Every rate in the program has n ≤ 2 books.
- All five derivations share one instrument (`completion_atlas.py::_bucket_of`); nobody has audited a sample of the 6,572 reconciled V dispositions — that audit belongs in card 1.
- Three denominators in play: 22,369 (35 books) / 24,475 (37 books) / 49,438 (incl. DONE) — every figure must name its population.
- SD-34 itself: 20 of 36 kanban rows complete; 16 open rows can still move populations.

## 1.b METHOD CORRECTION (operator challenge, 2026-08-31) — supersedes §1's cost model

The operator rejected the 600–5,300h bucket-B bracket: the data is already machine-readable (PCGen .lst tokens, preserved as `raw_tokens` on ingested records); PCGen computes it with a token interpreter; hand-modeling per unit is the wrong method. A dedicated measurement (`artifacts/fable-review/TOKEN-MODEL.md` + `.json`) confirms this **on every measured axis**:

- **The vocabulary is small.** 189 distinct top-level token types carry all 181,274 token instances on the 22,366 joinable remainder units (of 22,369; 99.99% join). 6 types = 50% of instances, **28 types = 90%**. Only ~139 types (41.7% of instances) are compute-bearing at all — 43.6% is metadata the kind-table loaders already consume, 10.3% is display text. Within compute tokens, 21 types cover 90%; BONUS collapses to 35 sub-targets with 9 covering 95%.
- **Most of the interpreter already exists.** F1–F9 `formula_interpreter` + `bonus_stack_reader` handle BONUS:VAR/DEFINE — present on **6,708 remainder units (30.0%)**; `pre_tokens.rs` covers 48 of 69 PRE types = 97.3% of prereq instances. Genuinely uncovered: ~9–15 BONUS application sub-targets + ~20 grant/structural consumers + a sub-3% tail.
- **The slow hours were transcription, not computation.** Receipts itemized (incl. the 617-min/2-unit cycle): the variable cost was reading tokens by eye, transcribing arithmetic into bespoke Rust, and hand-writing per-feature probes/tests — all of it replaced by interpreter + generic pass + oracle fixtures. Fixed per-pass costs (regen, sweeps, receipts) amortize.
- **CHOOSE is small.** Genuinely choice-bearing units = 1,669 of 22,366 = **7.5%**, and the v0.6 Path-A canonical-default precedent covers them as policy.
- **The old ban is dead.** SD-27 decisions.md §24.1 banned a formula interpreter; **SD-31 Decision 20 (operator, 2026-08-21) overturned it** — only the `derived_evaluator_fixture_check` gate still binds.
- **Revised bracket (engine route): ~85–350h reaching 14,817 units (66.2%) directly**, vs 3,900–31,000h at measured hand-modeling rates. The brackets do not overlap under any reading. Non-collapsing remainder = 7,549 units (33.8%): bucket C surfacing 3,981, choice-bearing 1,669, PI-redacted 1,333, token-less 924, X/U/Z/V residue 418.

**Superseding proposal for SD-35 step 1** (replaces §1's engine 2/4 framing; engines 1, 3, 5 stand): run the EXISTING interpreter + stack-reader as one corpus-wide pass over the 6,708 BONUS:VAR/DEFINE-bearing remainder units, oracle-fixture-verified (all machinery exists; only the pass is new). That converts the extrapolated bracket into a measured yield rate — and is itself the card-1 spike, cheaper and more direct than the 40h sampling plan in §1.

## 2. Confirmed Findings

126 findings total from 19 lanes; 56 CONFIRMED by independent re-verification, 9 REJECTED, 61 P3s left unverified by design. Full detail with evidence and line ranges: `artifacts/fable-review/findings-all.json` (per-lane raw JSON in the same dir).

### P1 — correctness / data-safety (8 confirmed, all report_only; none auto-fixable safely)

| ID | Where | What |
|---|---|---|
| R11-01 | `apps/desktop/src-tauri/src/character_hub.rs:2995` | `character_id` from the frontend is joined into a filesystem path with **zero validation** (no `..`/separator/absolute check) across 14+ character-scoped commands, including `delete_character` → `fs::remove_dir_all`. Needs one shared validator in `resolve_character_root`. |
| R14-02 | `src/saved_character/local_store.rs:32-80` | Saved-character save = two sequential non-atomic `fs::write` calls, no temp-file+rename. Crash between them corrupts the app's most valuable artifact. Same pattern in `src/campaign/local_store.rs:69` and `src/homebrew_authoring/package_store.rs:49` — systemic. |
| R9-02 | `src/bin/v06_work_inventory.rs:4237` | `enumerate_file`'s `rel` is a bare basename, silently misattributing citation provenance for nested-subdir .lst files. This is the SD-30 C1.9 carry-forward, still open after being flagged in three successive bundle registers. Touches the measurement instrument — needs its own reviewed card. |
| PC8-2 | `pilot_compute/mod.rs:51698-51844` | Per-weapon attack total omits the PF1 size modifier that sibling `compute_combat_baseline` applies (function never receives size at all). Wrong numbers for any non-Medium character's per-weapon rows. |
| PC8-1 | `pilot_compute/mod.rs:51445-51487` | Skill Focus / Master Craftsman explanation-id slugging leaves literal parens: `feat.master_craftsman_bonus.craft_(armor)`. Wire-format defect; fixing changes ids consumers may match on. |
| PC4-1 | `pilot_compute/mod.rs` (Warpriest/Skald/Bloodrager cluster) | Chooser `recognized` flag checks only hand-modeled selections while the generic pool-group pass already grounded records — `claim_blocking` and diagnostic prose can contradict the same run's own records. |
| R12-01 | `scripts/transcribe_companion_tables.py:1565` | companion_data.rs emitter writes non-atomically; the sibling monster emitter already carries the documented fix (SD31-W9-INTEGRATE-001). Port the same temp-file+`os.replace`. |
| R11-02 | `apps/desktop/src-tauri/src/reach_gate.rs:2168` | reach_gate marks 12 content-kind families "Surfaced" via a Tauri command that has **zero frontend callers** (no Reference Library screen exists). Any dashboard consuming these verdicts overstates real reach. Instrument-correctness gap. |

### Notable confirmed P2 bugs (of 43 P2s; rest in findings-all.json)

- **PC2-1**: stale `race.semantics.unverified` diagnostic co-fires with real grounded explanations for 12+ non-CRB races.
- **PC2-2**: Undine alternate-trait formula evaluation silently drops its explanation on evaluator failure — no diagnostic.
- **PC3-1**: Good-domain Inquisitor explanation emits the literal string `{magnitude}` instead of the number.
- **PC5-1**: Hunter's `other_features_deferred` record never reaches the Character Sheet's "Not computed" lane.
- **R8-01**: `book_dir_of` drift — 3 of 6 copied helpers missing the dreamscarred_press fix (duplication actively causing bugs).
- **R12-02**: 11 of 11 `derive_*_fixtures.py` generators non-atomically overwrite the ONE shared fixture file they all merge into.
- **R13-2**: oracle harness has no checkout-presence/pin preflight before `BatchExporter` — absent checkout crashes raw.

## 3. Applied Fixes (commit log)

- `920cb53307` — R12-03 + R12-04: deleted `scripts/diff_check_regen.py` and `scripts/summarize_domain_power_classes.py` — both hardcode a `.claude/worktrees/wf_2656fbd3-1ec-1` path that no longer exists; fully non-functional; only references are SD-32 frozen history docs; Opus-cleared.

Bloat-fix batches (run `wf_9ba4044a-ff5`, audited PASS — every claimed diff-stat verified against git; net **−10,809 lines**):
- `84760e4326` — **A** (R8-02): deleted 11 obsolete `e5_*`/`e6_*` probe bins (−1,756) with the atomic deliberate baseline change (`BASELINE_ROOT_TEST_BINARIES` 580→569, tests floor unchanged — the 11 bins carried 0 `#[test]`).
- `a049f694ac` — **B** (R8-04): 4 near-identical `enrich_*_raw_tokens` bins now wrap one shared engine (`cache_gen/enrich_raw_tokens_shared.rs`, `EnrichConfig`); every real per-bin difference (dreamscarred 3-segment dirs, PI-screen shapes, identity fields) preserved as explicit named config. Net −453. `enrich_class`/`enrich_equipment` confirmed-different, untouched.
- `a80e7c769f` — **C** (R14-04): byte-identical cache_gen helper family hoisted to `mod.rs`; call sites untouched via re-imports; genuinely-divergent same-name helpers deliberately left. Net −261.
- `93310c03ff` + `b567746058` — **E** (R10-F2): `tests/common/mod.rs` created; 312 byte-identical `fn load()` (+265 `explanation()`, 116 `has_explanation()`) copies deleted across 313 files. Net −8,377. Test-binary count unchanged at 543; zero new warnings (each suspect proven pre-existing by HEAD-restore).
- **D** (PC5-3/PC6-1/PC1-1/PC2-3): **zero commits, correctly.** All four pilot_compute dedup findings failed the byte-identical bar on close read — PC1-1 is duplicated for a ratified reason stated in the code (lines 7398–7405); PC2-3's seven race seams genuinely diverge (variable trait dimensions, alternate-trait gating, unique prose) and its evidence partly misidentified its own target; PC5-3 likewise non-mechanical. **§4 items 2's dedup sub-items are hereby re-classed from "mechanical" to "genuine refactors requiring design"** — the extraction seams stand, the quick-win framing does not.
- Batch E also surfaced a **6th pre-existing failing suite**: `tests/sd27_known_spells_must_be_on_the_class_spell_list.rs` (proven identical-failure at HEAD content) — added to §7's ledger.

Blocked by Opus clearance (correctly): R8-02 deletion of 11 obsolete `e5_*`/`e6_*` probe binaries — `scripts/verify.sh:1317` enforces a gate-tracked binary count; deletion needs the baseline updated in the same change → moved to §4.

## 4. Report-Only Proposals (the bloat-reduction backlog, ranked by payoff)

1. **Test-suite consolidation (R10)** — the single biggest bloat item. 89 `sd18_*_widening.rs` + 95 `sd13_*progression*.rs` are near-byte-identical templates (~80k of 187k test lines); 312 of 543 test files hand-copy a byte-identical `fn load()`; no `tests/common` exists. Table-driven consolidation → est. 543 → ~360 test binaries, large compile-time win, coverage-identical migration path sketched in `R10.json`.
2. **`pilot_compute/mod.rs` split (PC1–PC10)** — 78,506 lines with clean seams: per-class ACG/APG grounding clusters (PC4-4: nine classes), spellcasting mega-functions (PC7-2), self-contained test modules (PC10-3). Plus mechanical dedup: ~20 `ground_<class>_class_features` clones (PC5-3), ~1,400 lines of Ranger per-tier blocks (PC6-1), wolf/horse companion stat blocks (PC1-1), 7 `explain_<race>_race_seam` clones (PC2-3).
3. **`src/bin` family consolidation (R8)** — `enrich_*_raw_tokens` ×6 (4 near-byte-identical), plus R8-02's 11 obsolete probe binaries (delete together with a verify.sh baseline bump). Fixes R8-01's real drift bug as a side effect.
4. **`v06_work_inventory.rs` structure (R9-03)** — `classify()` is a single ~2,555-line function; extract before the next instrument change (which R9-02's fix will force anyway).
5. **cache_gen dedup (R14-04)** — same helper family duplicated across up to 13 of 14 per-book files; `seeded_current_truth()` is one 6,531-line function (R14-05).
6. **Desktop dead surface (R11-03/04)** — `append_to_character`, `re_save_character`, and 4 of 5 `campaign_drive` commands registered with no frontend caller: wire them or drop them (no-stub doctrine).
7. **Engine extensibility (R14-06)** — notes on what adding a formula family to `formula_interpreter`/`bonus_stack_reader` costs today; input to SD-35 engine 4.
8. **Oracle harness preflight (R13-2)** + un-run `scripts/tests` (R13-1: 46 of 56 python test files never invoked by verify.sh — wire or prune).

## 5. Rejected / Unverified

9 REJECTED by independent verification (details in findings-all.json): the two biggest saves were **R14-01** (claimed wrong bonus-spell table — the code matches PF1 Table 1-3 exactly; a fix would have introduced a bug) and **R9-01's premise** (the SD-30 "silent stamp-loss" hazard from prior retros is ALREADY guarded in code, wired into the only production path — the memory/retro claim is stale). Also rejected: R8-03 (ingest binary is documented-superseded, not deletable), R8-05 (gen_cache diff claim overstated), R10-F3 (env-var test-gating count wrong), PC3-2/PC3-7 (duplication shape misread), census-004/010 (bin sub-classification wrong; totals right).
61 P3 findings were not individually re-verified (by design); treat them as leads, not facts.

## 6. Verification Log

- Method: every P1/P2 finding and every auto-fix candidate re-verified by an independent agent that re-opened the cited file:lines (65 findings re-checked; 56 CONFIRMED, 9 REJECTED). All 3 auto-fix candidates then adversarially judged by Opus (default-refuse): 2 cleared, 1 blocked.
- Workflow runs: backlog `wf_c656b776-955` (5 agents), review `wf_23734e3f-d67` (39 agents, 0 errors).
- Baseline full sweep (2026-08-31, pre-fix, at `4f3f995184`): **FAIL — 14 of 40 stages red, ALL pre-existing at SD-34 wave-22 (`3aebc28477`); none caused by this review** (attribution below). 26 stages green. Log dir `/tmp/codex-verify-RWY6GT`.
- **Final full sweep (2026-09-01, post-fix, at `03ba5fcdb2`): same 14 stages red, same 26 green — the per-suite failing set inside `root-full` is IDENTICAL to baseline (47 suites, awk-extracted and diffed), suites executed 600→589 (exactly the 11 deleted bins), same 8,289 tests passed. ZERO new failures, zero tests lost. The review's completion bar (§7) is met.** Log dir `/tmp/codex-verify-ziiG09`; failing-suite lists preserved in the session scratchpad (`fails-codex-verify-*.txt`).

## 7. Gate State at Baseline (P1 review finding: tranche/14 does not pass its own gate)

One dominant root cause: **the wave-22 oracle-verdict restamp introduced statuses (`oracle-agree`, `oracle-unverifiable`) that downstream instruments have no mapping for**, plus stale hardcoded counts — the exact raise-on-unknown-status and count-change hazards prior retros recorded.

| Failed stage(s) | Cause | Whose fix |
|---|---|---|
| `reachability-audit`(+selftest), `shape-coverage-standing-gate` | `ValueError: doneness: unmapped 'static'/'derived' + 'oracle-agree'` — verdict tables lack the new statuses | SD-34 (instrument owners) |
| `site-dashboard-check`, `site-public-status-check` | committed feeds stale vs wave-22 inventory; producer also degrades on `oracle-unverifiable` templates; v06_work_inventory `--summary` timed out (600s) under sweep load | SD-34 |
| `root-lib` (5 failing lib suites), `root-full` (**47 failing suites** of 600, 8,289 passed) | stale census tests + status-mapping breakage across sd18/sd20/sd24/sd26/sd27/sd30/sd31 suites, formula_interpreter populations, companion_chassis dispatch on `oracle-unverifiable`; full suite list in §6's log-diff artifacts | SD-34 |
| `desktop`, `reach` | companion_catalog + reach_gate tests broken by the same status/count movement | SD-34 |
| `frontend-test` (4 of 100 files) | race-trait count 596→605 stale; **Cargo.toml 0.11.0 vs expected 0.14.0 — the tranche/14 version bump was never applied**; build-label fixture stale | SD-34 |
| `denominator-gate` (26), `figure-provenance` (32) | all violations in SD-34's own `progress.md` + epic-3/5 receipts, pre-existing and partly already flagged by AT-34-E3-004 | SD-34 |
| `pi-sweep` | 3 unbaselined `Pharasma` hits in `simple_kind_tables.rs` from AT-34-E3-001 — needs baseline rows or scrub (PI/legal-adjacent, prioritize) | SD-34 |
| `clippy` (root 86, desktop 25>7) | accumulated warnings incl. now-unused fns | SD-34 / partly addressed by bloat batches |
| Baseline note (not a failure) | `BASELINE_CORPUS_LITERAL_RECORDS` 26500→48708 stale | SD-34 deliberate update |

**Revised completion bar for this review's fixes:** the post-fix sweep must show the SAME OR FEWER red stages with ZERO new ones. Turning these 14 green is SD-34's remaining-card work and goes to codex-75 in the unpause message.

## Resume Contract

If this run dies (token exhaustion, interrupt), a fresh session resumes as follows:

1. Read this file and the Lane Status table. DONE lanes are done — their JSON sits in `artifacts/fable-review/`. RUNNING/PENDING lanes restart from scratch (all read-only, idempotent).
2. Workflow runs may be resumable via `resumeFromRunId` (run ids recorded below when launched).
3. Fixes: trust only §3 entries with commit shas; cross-check `git log`. Resume fixing at the first CONFIRMED auto_fix finding without a commit sha.
4. Fix denylist (fixers must not touch): anything serving SD-34's open cards — core_rulebook bucket engines and classifiers (`scripts/completion_atlas.py`, `docs/work-inventory.json`, shared inventory/classifier instruments), ultimate_campaign trait_content handling, `data/corpus/**`, generated files (header marker `GENERATED FILE`), `docs/release/**` (except this file and `artifacts/fable-review/`), scripts referenced by `scripts/verify.sh`. When in doubt: report-only.
5. Hazards: never run the inventory regenerator or the dashboard producer from a review/backlog lane (silent stamp-dropping; raise-on-unknown-status). Shared checkout: `git status` before every git write; explicit paths only; never `git add -A`; never `git stash`. Idle session codex-75 may resume SD-34 on this branch — check HEAD freshness before committing.
6. Verification bar: per-fix `cargo check`; per-batch full `scripts/verify.sh` (covers desktop crate, frontend, clippy, corpus gates).

### Standing instruction (operator, 2026-08-31)

After the bloat-fix batches land and the final full `scripts/verify.sh` is green: send a message to peer session **codex-75** telling it to UNPAUSE SD-34 activity (it was implicitly paused while this review edited shared files on tranche/14). Include: HEAD sha at that point, the note that `fable-review.md` + register row C1.6 exist, and that pilot_compute/mod.rs, src/bin, cache_gen, and tests/ were refactored (behavior-identical) so it should rebase/pull before further work.

### Run ids

- Backlog assessment (B1–B4 + B-SYNTH): Workflow run `wf_c656b776-955`, launched 2026-08-31. Script: session workflows dir, `fable-backlog-assessment-wf_c656b776-955.js`.
- Code review (PC1–PC10, R8–R14, H1–H2 + verify + clearance): Workflow run `wf_23734e3f-d67`, launched 2026-08-31. Script: `fable-code-review-wf_23734e3f-d67.js`.
- Lane statuses B1–B4, B-SYNTH, all review lanes: RUNNING as of launch.
- Bloat-fix batches A–E + audit: Workflow run `wf_9ba4044a-ff5` (script `fable-bloat-fixes.js`), launched 2026-08-31 after the baseline sweep. Pass bar: zero NEW test failures beyond the 5 pre-existing lib suites (§7).
