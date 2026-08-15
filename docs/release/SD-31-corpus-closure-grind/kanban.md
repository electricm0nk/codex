---
canonical: true
owner: god-emporer
status: planning-ready (SD-32 absorbed, epics re-sequenced, operator ruling 2026-08-15)
date: 2026-08-15
canonical_branch: tranche/11
---

# SD-31 — Local-file Work Queue

Same local-file dispatch convention as `SD-30-class-feature-archetype-bundle/kanban.md` (Hermes board
retired, operator directive 2026-08-01).

**Origin.** Cards `epic-3-measurement` through `epic-8-cloud-fanout` were moved from SD-30's
`epic-4-measurement`, `epic-5-mechanism`, `epic-6-chassis-sweep`, `epic-10-ingest-lanes`,
`epic-11-book-onboarding` and `epic-14-cloud-fanout` rows (operator ruling 2026-08-14, `SD-30
decisions.md §51`). Cards `epic-1-race-chassis` and `epic-2-verdict-paths` were moved from
`SD-32-engine-capability-builds/kanban.md`, which was absorbed and deleted by operator ruling
2026-08-15 (`decisions.md §2`). `epic-0-reachability-audit` and `epic-9-closure` are new in that same
ruling. No card was `IN-FLIGHT` at either move — verified against both source boards immediately
before each.

**Claim-priority order is the table order, top-down**, and it is not the old order: capability
(Epics 1-2) now precedes the lanes that consume it, because 8,524 units — 22.1 % of the board — cannot
reach `done` without it (`decisions.md §2`).

## Status legend

Identical to `SD-30-class-feature-archetype-bundle/kanban.md`'s legend — `READY`, `READY (gated on
...)`, `IN-FLIGHT`, `BLOCKED`, `COMPLETE`. See that file for the full definitions; not reproduced here
to avoid drift between two copies of the same legend text.

## Cards

| ID | Status | Epic | Cycle-type | Claimed-by | Claimed-at | Cycle-id |
|----|--------|------|-----------|------------|------------|----------|
| `epic-0-reachability-audit` | COMPLETE | **Order 1 — Reachability Audit (standing gate)** | `scripts/reachability_audit.py` built, self-tested (fabricated dead-end proven caught, 11 cases green), wired into `verify.sh` as `reachability-audit`/`reachability-audit-selftest`; baseline run committed at `artifacts/SD31-E0-F1-001-baseline.{md,json,txt}` — reachable ceiling 94.53 %, every dead-end/known-gap owned by Epic 1 or Epic 2, no SER proposal needed. **Standing gate per `decisions.md §4`: re-invoked at every epic closure, not a one-shot card** — this row records the initial build+baseline cycle only. **Re-invoked 2026-08-15 at the `SD31-W1-INTEGRATE-001` integrated tip (`sd31/e2-groundtruth` merged onto `tranche/11`): reachable ceiling unchanged at 94.53 % (36412/38521), same 9 `ambiguous|*` dead-end cells, all still owned by Epic 2, `AUDIT_EXIT=0` — `artifacts/SD31-W1-INTEGRATE-001-audit.json`. Also this cycle: fixed the audit-docstring over-claim CONFIRMED by adversarial review (narrowed to the wiring_class axis; status-axis gap logged `OPEN-ISSUES.md` row 6, non-blocking) — acceptance genuinely still met, COMPLETE stands.** | sd31-e0-audit / sd31-w1-integrate | 2026-08-15 | `SD31-E0-F1-001` / `SD31-W1-INTEGRATE-001` |
| `epic-1-race-chassis` | READY | Race Chassis, 100 % mandate | chassis design → per-race (or batch) build with DoD-8 on-screen verification → ceiling release to `epic-6` per race batch | — | — | — |
| `epic-2-verdict-paths` | READY | Verdict-Path Capability, 100 % mandate | hand-labelled ground-truth sample (gate) → classifier build/accept or close-at-F1 → **`ambiguous` dead-end closed or registered**. **SD31-E2-F1 landed 2026-08-15 (`sd31/e2-groundtruth`, merged onto `tranche/11` this cycle): 150-unit hand-labelled sample committed, `artifacts/SD31-E2-F1-ground-truth-sample-v1.json` + methodology note. Card is NOT closable at F1 as-is — adversarial review CONFIRMED 105 of the 150 labels (including all 40 of the `display_grounded_target` population AT-31-010 binds) carry no record-specific token evidence; the 95.5 %/71.3 % headline agreement figures are WITHDRAWN and Decision 1(e) item 4's "close at F1" path is explicitly barred until re-labelled (`OPEN-ISSUES.md` row 3, `BLOCKER`). F1 real, evidenced findings (Finding A: 97 % of the `no_corpus_line` bucket is a fixable path-join bug, not genuine ambiguity; Findings B/C: two classifier false-positive mechanisms) stand and are load-bearing for F2. Status stays READY, gated on re-labelling before F1 can be cited as closing evidence, F2 not yet dispatched.** | sd31-e2-groundtruth / sd31-w1-integrate | 2026-08-15 | `SD31-E2-F1-001` |
| `epic-3-measurement` | READY (per-class; F4 gated on `epic-2`) | Per-Class Archetype Measurement | class inventory + per-class hand-verification + chooser-primitive design + `unknown`-bucket characterization (F4) | — | — | — |
| `epic-4-mechanism` | READY (per-class, gated on `epic-3` clearing the target class) | Archetype Mechanism | supersession-shape wiring per cleared class; chooser-shape wiring once `epic-3`-F3 lands | — | — | — |
| `epic-5-chassis-sweep` | READY (per-class, gated on `epic-3` + `epic-4` for the target class; F3 additionally gated on `epic-2` and `epic-3`-F4) | Per-Class Chassis Sweep | per-class `class_feature` ingest across the 23 in-scope books, reach-gate claim per record; **F4 (added 2026-08-15) — the 36 `deferred-with-reason` units, each with a real path or a proposed register entry** | — | — | — |
| `epic-6-ingest-lanes` | READY for F1/F2/F5/F6/F7/F8/F9/F10/F11; **F3 and F4 gated on `epic-1` per race batch** | Corpus-Wide Ingest Lanes, folded from SD-29 | per-kind ingest/instrument: F1 `monster` (fixture-coverage lane, rewritten 2026-08-15), F2 `spell`, F3 `race`, F4 `race_trait`, **F5 `equipment`, F6 `equipment_modifier`, F7 `companion`, F8 `feat` (routes SD-30 E0-F3's 217-unit probe-fixture residue), F9 `monster_ability`, F10 `class` (all added 2026-08-15, blocker B2), F11 held static/derived residual (added 2026-08-15, blocker B4)** — each runs the raw-vs-workable split + pre-cycle classifier screen before claiming a book | — | — | — |
| `epic-7-book-onboarding` | READY | Book Onboarding, 100 % mandate | onboard the 7 `future_state` books — PI screen cited clean per book before any record is written | — | — | — |
| `epic-8-cloud-fanout` | READY (per lane shape, after one local proof cycle) | Cloud Fan-Out Protocol (grind **and** capability lanes) | local-proof-then-cloud-scale protocol; local orchestrator owns all `tranche/11` merges (updated from `tranche/10`, `decisions.md §6`); DoD-8 and dashboard-producer work stay local | — | — | — |
| `epic-10-version-numbering` | COMPLETE | Build Version Numbering | version-bump 0.11.0 for the `tranche/11` cut (`decisions.md §6`) — package.json/tauri.conf.json/Cargo.toml/Cargo.lock, the publish-workflow VERSION stamp, and the full test-fixture literal surface (8 files); full gate green (`VERIFY_EXIT=0`, 19/19 stages, `artifacts/sd31-s7-version-verify.log`) | sd31-ready-s7-version | 2026-08-15 | `SD31-S7-VERSION-001` |
| `epic-9-closure` | READY (gated on every other card) | Closure and the 100 % Exit Gate | `epic-0` audit at closing tip → reachable ceiling 100 % or signed register entries → **F3 bundle code review of this package's own diff (added 2026-08-15)** → closure receipt + promotion PR (opened, not merged) | — | — | — |

## The two gates that exist because of the merge

`decisions.md §2` inverted an ordering in which the capability builds were scheduled *after* the lanes
depending on them. Both dependencies were cross-package handoffs; both are now **internal hard gates**,
and a cycle claiming across an open one is out of protocol exactly as a PI-gate violation would be:

1. **`epic-1-race-chassis` → `epic-6-ingest-lanes` F3/F4.** No `race` or `race_trait` ingest cycle
   claims a book before Epic 1 has landed a chassis covering the races that book's rows reference. The
   gate opens **per race batch**, not all-or-nothing — Epic 1-F3 names each landed batch here as it
   lands, so ingest starts as soon as any chassis is real.
2. **`epic-2-verdict-paths` → `epic-3-measurement` F4 and `epic-5-chassis-sweep` F3.** No
   `unknown`-bucket characterization or disposal cycle claims before Epic 2 is `COMPLETE`.

## Cross-SD gate discipline (SD-30's PI gate — satisfied, still cited)

`SD-30-class-feature-archetype-bundle`'s `epic-3-pi-gate` closed `COMPLETE` on 2026-08-14 (all of
F1-F4; SD-30 closed the same day, PR #363 open). The hard block it imposed on `epic-5-chassis-sweep`,
`epic-6-ingest-lanes` and `epic-7-book-onboarding` is therefore **discharged at package level** — but
per-book citation is still required: a cycle claiming a book cites SD-30's `progress.md` receipt for
that book's screen, and calls the documented invocation contracts (`SD-30 decisions.md §52.3` for the
blacklist sweep, `§53.5` for the declared-PI reader) from the production ingest path before writing any
generated record. Discharged is not the same as absent: a cycle that writes records without calling the
readers is out of protocol.

## Deferral is not available to a cycle

The phrase "or named a successor for the remainder" is struck from this package (`decisions.md §2`
item 5). A unit leaves the 100 % denominator only through the **Structural Exclusion Register**
(`acceptance-and-verification.md AT-31-100`), which requires the proving command, the named missing
capability with why building it is impossible rather than merely expensive, an `epic-0` audit run
reproducing it, and **operator sign-off**. A cycle may propose an exclusion; only the operator grants
one. An unsigned proposal leaves the unit in the denominator and its epic open.

## Cycle claims (cycle-supervisor protocol)

Identical procedure to `SD-30-class-feature-archetype-bundle/kanban.md`'s "Cycle claims" section
(edit `Status`→`IN-FLIGHT`, `Claimed-by`, `Claimed-at`, `Cycle-id`; append to `progress.md`; on
completion edit `Status`→`COMPLETE` and append the receipt). `epic-4-mechanism` and
`epic-5-chassis-sweep` cycles name the class explicitly in `Cycle-id`; `epic-1-race-chassis` cycles
name the race batch; `epic-6-ingest-lanes` cycles name the kind and book.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle dispatch honors the post-edit
state.
