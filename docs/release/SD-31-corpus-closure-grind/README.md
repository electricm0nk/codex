---
canonical: true
owner: god-emporer
status: planning-ready (SD-32 absorbed, epics re-sequenced, operator ruling 2026-08-15)
date: 2026-08-15
canonical_branch: tranche/10 (unchanged, inherited from SD-30)
build_version_target: 0.10.<build> (shared with SD-30 for the remainder of the tranche/10 program)
companion_to: ./scope-draft.md, ./decisions.md
split_from: SD-30-class-feature-archetype-bundle (decisions.md §51)
absorbed: SD-32-engine-capability-builds (decisions.md §2, 2026-08-15)
---

# SD-31 — Corpus Closure: the Grind and the Capability Builds

**Scope note on the directory name.** The slug still reads `corpus-closure-grind`, but as of
2026-08-15 this package carries **both** the grind lanes and the engine-capability builds that
unblock them (`decisions.md §2`, which absorbed `SD-32-engine-capability-builds/` and deleted it).
The path is deliberately unchanged: it is cited from a shipped Rust test, a production Python script,
and SD-29/SD-30/SD-33 docs, and a rename buys slug accuracy at the cost of touching code. Read the
scope from this file, not from the directory name.

**Note on the `SD-31` number.** This package reuses the number `SD-31`, previously held by
`SD-31-pcgen-character-import`. That package was renamed to
`docs/release/SD-33-pcgen-character-import/` on the operator ruling that created this package
(2026-08-14: "ok, let's split phase 3 and phase 4 into their own SD's. SD-31 and SD-32. Take the
existing SD-31 and rename it to SD-33"). This package is unrelated to PCGen import.

**Note on the `SD-32` number.** `SD-32-engine-capability-builds/` was absorbed into this package and
deleted on 2026-08-15, following the precedent `SD-30 decisions.md §50` set when it absorbed the
*previous* SD-32 (`SD-32-instrument-coverage-and-consumer-wiring`). The number `SD-32` is free for
reuse. Two different packages have now held it; neither exists.

## Purpose

SD-31 owns **the whole remaining path to the 100 % mandate** (`SD-30 decisions.md §45`): the engine
capability that makes units reachable, and the volume work that then reaches them.

**The bar this package closes against is a doneness bar, not a reachability bar** (`decisions.md §5`,
operator ruling 2026-08-15): `done / denominator == 100 %`, where `denominator` is every unit in
`docs/work-inventory.json` except `EXCLUDED_BOOKS` — today **38,521** units, **5,837 done (15.15 %)**.
Epic 0's standing reachability audit (below) answers a related but different question — *can* a unit
reach `done` given current capability — and is a necessary gate, not a substitute: a 100 % reachable
ceiling with the board still at 15 % `done` does not close this package. Both bars must clear, or every
shortfall unit must carry an operator-signed Structural Exclusion Register entry
(`decisions.md §3`/`AT-31-100`).

It was created 2026-08-14 by splitting SD-30's Phase 3 ("the grind") into its own package, alongside a
sibling SD-32 holding Phase 4 ("capability builds"). On 2026-08-15 the operator merged the two —
*"SD-31 will be next, if there are prereqs in SD-32, then they need to be moved into SD-31"* — because
the split had scheduled the capability builds **after** the grind lanes that cannot reach `done`
without them.

**The dependency, measured** (`decisions.md §2` carries the derivation and the commands):

| population gated on capability, not on grind | units |
|---|---:|
| `wiring_class == ambiguous` — no path to `done` at any status | 2,109 |
| `unmeasurable` (`status == unknown`) | 3,989 |
| `race` + `race_trait` not yet `done` | 3,284 |
| **distinct union** | **8,524 — 22.1 % of the 38,521-unit board** |
| **ceiling reachable without the capability builds** | **77.9 %** |

Run in the numbered order, SD-31 would have closed with those lanes deferred to a successor, SD-32
would have built the capability that unblocked them, and the deferred work would have had to be
reopened — the ordering failure discovered only after both packages reported closure. Capability now
runs first, the handoffs are internal hard gates, and per-cycle deferral has been replaced by an
operator-signed Structural Exclusion Register (`decisions.md §3`).

## Source STC contents

Same chassis shape as `SD-30-class-feature-archetype-bundle/` (this package's origin package) — see
that package's `README.md` "Source STC contents" section for the convention. This package's files:

- `scope-draft.md` — the full scope: capability builds (race chassis, verdict paths), the
  `class_feature` measurement→mechanism→chassis-sweep chain, corpus-wide per-kind ingest lanes,
  7-book onboarding, cloud fan-out, closure.
- `decisions.md` — Decision 1 records the original split and reproduces the binding rules this
  package's cycles need (anti-gaming rule, table-sheet doctrine, PI-gate dependency, concurrency/cloud
  protocol). Decision 2 records the SD-32 absorption and the re-sequencing; Decision 3 the Structural
  Exclusion Register; Decision 4 the standing reachability audit.
- `loop-instruction.md` — short; governed by `SD-30-class-feature-archetype-bundle/loop-instruction.md`
  by reference, with this package's overrides only.
- `epic-breakdown.md` — Epics 0-9, with the origin of each recorded — see the renumber map below.
- `acceptance-and-verification.md` — AT-31-* criteria, including the per-kind `done`-floor table
  (formerly SD-30's AT-30-015) and `AT-31-100`, the Structural Exclusion Register.
- `kanban.md` — the cards, in claim-priority order, with the two internal capability gates stated.
- `progress.md` — fresh; SD-30's own per-cycle receipts for this scope stay in SD-30's `progress.md`
  as history, cross-referenced here.
- `forward-scope-register.md`, `risks-and-open-questions.md`, `release-notes.md`,
  `state-goals-and-lessons.md`, `technical-requirements.md`, `technical-design.md` — scoped to this
  package's charter.
- `artifacts/` — the corpus-wide instrument tooling stays canonical in
  `../SD-30-class-feature-archetype-bundle/artifacts/` and is used by path reference rather than
  duplicated. `scripts/reachability_audit.py` (Epic 0) is repo tooling, not a package artifact,
  because `scripts/verify.sh` runs it.

## Epic renumber map

| SD-31 epic | Origin | Objective |
|---|---|---|
| Epic 0 (SD31-E0) | **NEW** (`decisions.md §4`) | Reachability Audit — standing gate |
| Epic 1 (SD31-E1) | SD32-E1 ← SD30-E12 | Race Chassis (gates Epic 6-F3/F4) |
| Epic 2 (SD31-E2) | SD32-E2 ← SD30-E13 | Verdict-Path Capability (gates Epic 3-F4, Epic 5-F3) |
| Epic 3 (SD31-E3) | SD31-E1 ← SD30-E4 | Per-Class Archetype Measurement (gates Epics 4/5) |
| Epic 4 (SD31-E4) | SD31-E2 ← SD30-E5 | Archetype Mechanism |
| Epic 5 (SD31-E5) | SD31-E3 ← SD30-E6 | Per-Class Chassis Sweep (`class_feature` ingest) |
| Epic 6 (SD31-E6) | SD31-E4 ← SD30-E10 | Corpus-Wide Ingest Lanes (`monster`/`spell`/`race`/`race_trait`) |
| Epic 7 (SD31-E7) | SD31-E5 ← SD30-E11 | Book Onboarding (7 `future_state` books) |
| Epic 8 (SD31-E8) | SD31-E6 **+** SD32-E3 ← SD30-E14 | Cloud Fan-Out Protocol (both lane families, one copy) |
| Epic 9 (SD31-E9) | **NEW** (`decisions.md §2`/`§3`) | Closure and the 100 % Exit Gate |

SD-30's own Epics 0/1/2/3/7/8/9 are not moved — SD-30 closed 2026-08-14 with all of them `COMPLETE`
and its promotion PR (#363) open.

## Authority surface

Canonical (repo-resident) home: `docs/release/SD-31-corpus-closure-grind/`.

## Objective

Reach the 100 % dashboard mandate, in dependency order:

1. **Reachability audit** (Epic 0) — know, mechanically and continuously, which units *can* reach
   `done` at all. Runs first and re-runs at every epic closure.
2. **Capability first** (Epics 1-2) — the race chassis and the verdict paths, because 22.1 % of the
   board cannot reach `done` until they land, and no amount of ingest moves those units.
3. **The `class_feature` chain** (Epics 3-5) — per-class measurement gates mechanism gates chassis
   sweep. File-disjoint from the capability track; runs concurrently with it from the start, except
   the `unknown`-bucket seeds that wait on Epic 2.
4. **The ingest lanes** (Epic 6) — `monster` and `spell` may start immediately; `race` and
   `race_trait` open per race batch as Epic 1 delivers.
5. **Book onboarding** (Epic 7) — the 7 `future_state` books, PI screen cited clean per book.
6. **Cloud fan-out** (Epic 8) — the dispatch protocol for scaling any lane shape with one local proof.
7. **Closure** (Epic 9) — against Epic 0's audit, with no deferral hatch.

## In scope

- Engine capability: the race chassis (`race` 103 units at 0 %, plus the chassis-blind `race_trait`
  remainder) and the verdict-path work for the `unmeasurable` ∪ `ambiguous` union (~5,979 units,
  re-derive).
- `class_feature`, corpus-wide, all 23 books (unchanged list, `SD-30-class-feature-archetype-bundle/README.md`).
- The four SD-29-folded kind lanes: `monster`, `spell`, `race`, `race_trait`.
- **Six previously-unowned kinds, cards opened 2026-08-15 (`epic-breakdown.md` Epic 6 F5-F10,
  launch-readiness remediation Step 2, blocker B2):** `equipment` (3,582 not-done), `equipment_modifier`
  (669), `companion` (1,280), `feat` (1,432), `monster_ability` (2,773), and `class` (158) — all six
  are in the 38,521-unit mandate denominator (`decisions.md §5`); none had a card before this
  correction.
- The 7 `future_state` books: `occult_adventures`, `adventurers_guide`, `mythic_adventures`,
  `inner_sea_magic`, `inner_sea_temples`, `inner_sea_taverns`, `inner_sea_faiths`.
- The cloud fan-out dispatch protocol for all of the above.
- The reachability audit (`scripts/reachability_audit.py`) and the Structural Exclusion Register.

## Out of scope

- Instrument-application work (the `static`/`derived` `done` rung, consumer-delta probes, `unknown`
  characterization at corpus level) — that was SD-30's Epic 0, closed 2026-08-14. **Verify what it
  landed by content before assuming a probe exists**; several of its receipts flag per-kind gaps.
- Identifier cleanup, pre-launch, PI-gate ownership, version numbering, SD-30's bundle code review and
  closure epilogue — all SD-30's, all closed. This package's ingest epics *consume* SD-30's PI gate
  (cite per book, call the documented contracts) but do not own or re-run it.
- Two named carry-forwards SD-30 handed over, tracked in `forward-scope-register.md`: C1.8 (wire
  `v06_corpus_trap_report --audit` into `scripts/verify.sh`) and C1.9 (`v06_work_inventory.rs`'s
  `enumerate_file` bare-basename nested-citation bug).

## Dependency position

- **Depends on:** `SD-30-class-feature-archetype-bundle` Epics 1, 2 and 3 — all `COMPLETE` as of
  2026-08-14, so no SD-30 work blocks this package's launch. Per-book PI citation is still required
  (`kanban.md` "Cross-SD gate discipline").
- **Contains its own prerequisites.** As of `decisions.md §2` there is no sibling package holding
  capability this one needs. That was the defect the merge fixed.
- **Blocks:** nothing external. The `tranche/10` promotion PR (#363) is SD-30's and is open; the
  operator holds sole merge authority.

## Exit statement

SD-31 is complete when **both** of the following hold, at the closing tip:

1. **Epic 0's reachability audit reports a reachable ceiling of 100 %** — or every shortfall unit
   carries a Structural Exclusion Register entry with **operator sign-off** (`decisions.md §3`), each
   naming the missing capability and why building it is genuinely impossible rather than merely
   expensive.
2. **The doneness bar clears** (`decisions.md §5`, `AT-31-103`, operator ruling 2026-08-15):
   `done / denominator == 100 %` against the full mandate denominator (every unit in
   `docs/work-inventory.json` except `EXCLUDED_BOOKS`) — or every shortfall unit carries the same
   signed Structural Exclusion Register entry as above.

Neither bar substitutes for the other: (1) is a capability-gap check ("could this unit reach `done`
given what the engine can do today"), (2) is the actual count ("did it"). AT-31-005's per-kind
`done`/`held` floors are progress signal only and satisfy neither bar on their own.

There is no third option. The phrase *"or named a successor for the remainder"* — which appeared twice
in this package's original completion gate, pointed at exactly the lanes the capability builds unblock
— is struck (`decisions.md §2` item 5). A cycle may **propose** an exclusion; only the operator grants
one, and an unsigned proposal leaves the unit in the denominator and its epic open.

The full per-epic completion gate is in `epic-breakdown.md`.
