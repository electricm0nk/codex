# Fable Review — repo code review + SD-35 backlog assessment

Started: 2026-08-31. HEAD at start: `3aebc284774cbfa09a84a3d6cb25d60e9b1be447` (tranche/14).
Operator commission: (1) code review — gaps, improvements, bloat; report + low-risk fixes only; (2) TOP PRIORITY: judge whether the SD pipeline addresses the ~29,283-unit remaining backlog properly and whether processing engines can expedite SD-35+ (propose, don't build). Plan: `~/.claude/plans/model-agile-eagle.md`.

## Status: RUNNING

## Lane Status

| Lane | Scope | Model | Status | Findings |
|---|---|---|---|---|
| B1 | forward-plan.json number audit | sonnet | DONE | stale plan; live remainder 22,369 |
| B2 | price the unpriced 45.7% (D/M/V/X/Z) | sonnet | DONE | live unpriced 6,469 = 28.9%; 5/5 buckets rate-less |
| B3 | engine ROI ranking (136 mechanisms) | sonnet | DONE | live census 83 mech / 22 cover 90% (35 books) |
| B4 | bucket-B rate decomposition | sonnet | DONE | fast/slow 9.8×; composition projection overturned by synth |
| B-SYNTH | backlog verdict + engine build order | opus | DONE | see §1 |
| R1–R7 | pilot_compute/mod.rs (7 chunks) | sonnet | PENDING | — |
| R8 | src/bin duplicate families | sonnet | PENDING | — |
| R9 | v06_work_inventory.rs + v06 bins | sonnet | PENDING | — |
| R10 | tests/ structure + templated families | sonnet | PENDING | — |
| R11 | apps/desktop/src-tauri | sonnet | PENDING | — |
| R12 | scripts/ + tools/ (generators first) | sonnet | PENDING | — |
| R13 | oracle_harness + scripts/tests | sonnet | PENDING | — |
| R14 | remaining src/ modules | sonnet | PENDING | — |
| H1–H2 | grep sweeps + denominators | haiku | PENDING | — |
| VERIFY | finding verification (P1/P2 + all auto_fix) | sonnet+opus | PENDING | — |
| FIX | safe-fix application | sonnet ×2 | PENDING | — |

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

## 2. Confirmed Findings

not yet reached

## 3. Applied Fixes (commit log)

not yet reached

## 4. Report-Only Proposals

not yet reached

## 5. Rejected / Unverified

not yet reached

## 6. Verification Log

not yet reached

## Resume Contract

If this run dies (token exhaustion, interrupt), a fresh session resumes as follows:

1. Read this file and the Lane Status table. DONE lanes are done — their JSON sits in `artifacts/fable-review/`. RUNNING/PENDING lanes restart from scratch (all read-only, idempotent).
2. Workflow runs may be resumable via `resumeFromRunId` (run ids recorded below when launched).
3. Fixes: trust only §3 entries with commit shas; cross-check `git log`. Resume fixing at the first CONFIRMED auto_fix finding without a commit sha.
4. Fix denylist (fixers must not touch): anything serving SD-34's open cards — core_rulebook bucket engines and classifiers (`scripts/completion_atlas.py`, `docs/work-inventory.json`, shared inventory/classifier instruments), ultimate_campaign trait_content handling, `data/corpus/**`, generated files (header marker `GENERATED FILE`), `docs/release/**` (except this file and `artifacts/fable-review/`), scripts referenced by `scripts/verify.sh`. When in doubt: report-only.
5. Hazards: never run the inventory regenerator or the dashboard producer from a review/backlog lane (silent stamp-dropping; raise-on-unknown-status). Shared checkout: `git status` before every git write; explicit paths only; never `git add -A`; never `git stash`. Idle session codex-75 may resume SD-34 on this branch — check HEAD freshness before committing.
6. Verification bar: per-fix `cargo check`; per-batch full `scripts/verify.sh` (covers desktop crate, frontend, clippy, corpus gates).

### Run ids

- Backlog assessment (B1–B4 + B-SYNTH): Workflow run `wf_c656b776-955`, launched 2026-08-31. Script: session workflows dir, `fable-backlog-assessment-wf_c656b776-955.js`.
- Code review (PC1–PC10, R8–R14, H1–H2 + verify + clearance): Workflow run `wf_23734e3f-d67`, launched 2026-08-31. Script: `fable-code-review-wf_23734e3f-d67.js`.
- Lane statuses B1–B4, B-SYNTH, all review lanes: RUNNING as of launch.
