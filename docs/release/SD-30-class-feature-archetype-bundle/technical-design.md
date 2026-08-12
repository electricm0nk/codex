---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/10 (operator directive 2026-08-01)
build_version_target: 0.10.<build>
companion_to: ./technical-requirements.md, ./decisions.md
---

# SD-30 Technical Design

**Re-scoped 2026-08-10** (`decisions.md §33-38`). This file's "Cycle paths" section below still names
the retired sixteen-per-book-epic shape — read it as historical; the current shape adds a
measurement/mechanism layer in front of the ingest pipeline it describes, detailed in "Per-class
measurement and mechanism layer (new, 2026-08-10)" further down.

## Architecture overview

SD-30 extends the per-book ingest pipeline established by SD-22 (APG,
ACG, Bestiary 1, DM toolkit baseline), refined by SD-27 (Shape B
schema + license-stripping), and reused by SD-28 (Ultimate Psionics
tier) and SD-29 (Bestiary 2-5, now corpus-wide). SD-30's scope, as of
2026-08-10, is `class_feature` corpus-wide — 23 books across every content
source SD-28/SD-29 touch, not a fixed sixteen-book list (`decisions.md §33`).
SD-30 additionally owns a layer none of its predecessors needed: a per-class
archetype-slot **measurement** pass gating a **mechanism-wiring** pass,
both ahead of the familiar ingest pipeline below — see the new section after
"Cycle paths".

```
                    ┌──────────────────────────┐
                    │  v06_corpus_trap_report  │ (pre-cycle, every book)
                    └──────────┬───────────────┘
                               │ findings
                               ▼
┌──────────────────┐   ┌──────────────────────┐
│  corpus LST/JSON │──▶│  Shape B cache       │   ← SD-27-owned
└──────────────────┘   │  data/corpus/<book>/ │      schema
                       └──────────┬───────────┘
                                  │ records
                                  ▼
                       ┌──────────────────────┐
                       │  src/rules_core/     │
                       │  rules_tables/<book>/│   ← per-book source
                       └──────────┬───────────┘
                                  │ slices
                                  ▼
                       ┌──────────────────────┐
                       │  reach_gate.rs claim │   ← prime rule (per §18)
                       │  (apps/desktop/src-  │      definitions of done
                       │   tauri/src/)        │
                       └──────────────────────┘
```

## Cycle paths (RETIRED shape — sixteen-per-book-epic numbering, superseded 2026-08-10)

Each cycle wrote to **one book's tree**, under the old numbering:

- Epic 3 → `src/rules_core/rules_tables/occult_adventures/`
- Epic 4 → `src/rules_core/rules_tables/horror_adventures/`
- Epic 5 → `src/rules_core/rules_tables/mythic_adventures/`
- Epic 6 → `src/rules_core/rules_tables/monster_codex/`
- Epic 7+ → `src/rules_core/rules_tables/inner_sea_<module>/` (ten books including World Guide)
- Epic M+ → `src/rules_core/rules_tables/book_of_the_damned_volume_<1,2>/`

Per-book cycles were file-disjoint; they could run in parallel. Within a
single book, cycles were sequential (one cycle per file at a time). **Retired
2026-08-10** — see the next section for the current shape.

## Per-class measurement and mechanism layer (new, 2026-08-10, `decisions.md §34/§37`)

The current epic-6 chassis-sweep cycle still writes to `src/rules_core/rules_tables/<book>/`,
file-disjoint by book exactly as above — that part of the pipeline is unchanged. What is new is what
gates it:

```
┌────────────────────────────┐
│ Epic 4: per-class hand-    │  no automated proxy (three tried, all failed —
│ verification, no proxy     │  SD-28 §63). Output: wired-able/named per class,
│ (extends SD-28 §60/§63/§64)│  never blended. Also characterizes `unknown`.
└──────────────┬─────────────┘
               │ per-class clearance receipt
               ▼
┌────────────────────────────┐
│ Epic 5: archetype_claims_  │  supersession shape (25 classes, 175 mechanisms,
│ slot wiring, per class     │  proven on Alchemist/Fighter) OR chooser-
│ (pilot_compute.rs)         │  interaction shape (Oracle/Arcanist/Sorcerer,
└──────────────┬─────────────┘  primitive not yet designed)
               │ mechanism landed + reachability-tested
               ▼
┌────────────────────────────┐
│ Epic 6: class_feature      │  same file-disjoint-by-book shape as the retired
│ ingest, per class          │  sixteen-per-book epics, but gated: a class's
│ (rules_tables/<book>/)     │  cycle cannot fire before that class clears Epic 4.
└────────────────────────────┘
```

Different classes' Epic 4→5→6 chains run concurrently (file-disjoint by class and by
`rules_tables/<book>/` path); within one class's chain, the three stages are strictly sequential.
This is the load-bearing difference from every prior SD-2x content bundle's architecture: ingest is
no longer the first stage.

## Cross-bundle precedence (operator directive 2026-08-01)

Per `decisions.md §16`, "those recently published take precident." When
SD-30's records conflict with SD-28 / SD-29's already-published
records, SD-28 / SD-29's records are doctrine and SD-30 references the
canonical id only.

This applies to:

- **Class-grant overlap**: SD-30 owns canonical class definitions for
  Occultist, Spiritualist, Medium, Mesmerist (shared with SD-28's
  Ultimate Intrigue). SD-28 references the SD-30 canonical class id.
- **Record-level overlap**: when a monster or archetype in Occult /
  Mythic / Inner Sea conflicts with a re-publish in SD-28 / SD-29,
  the SD-28 / SD-29 version is doctrine.

The cross-bundle class-grant case is the only exceptional case in §16's
newer-wins rule (the four shared classes follow §5's bundle-owns-
canonical-doctrine rule).

## Local-file dispatch (no Hermes board)

Cycle dispatch is local-file. The supervisor reads `kanban.md` at top
of each tick to identify the next ready card:

1. The bundle's first cycle (Epic 2's pre-flight) populates `kanban.md`
   with the 9 dependency-ordered epics as ready/gated cards (re-cut
   2026-08-10, `decisions.md §33-38`; retired sixteen-per-book cards
   resolve per `kanban.md`'s "Retired cards" table).
2. Cycles claim cards by editing `kanban.md` (marking `claimed by`,
   `claimed at`, `cycle id`, etc.).
3. Per-cycle completion writes the cycle receipt to `progress.md` and
   closes the kanban card.

## Reach-gate = prime rule (per `decisions.md §18`)

Reach is the operator-visible definition of done. Cycles MUST NOT close
without a reach-gate claim that executes the real IPC builder from
`reach_gate.rs`. A gate that returns exit 0 with zero matched tests is a
hard failure (a gate running zero tests asserts nothing).

For SD-30's books, missing consumer surfaces (campaign-tool integration
for Inner Sea, mythic-tool for Mythic Adventures, etc.) may surface in
`reach_gate.rs OPEN_FINDINGS`. Cycles record the gap; remediated per
TR-30-002 (engine or surface-extension where strictly necessary) or
via `forward-scope-register.md C3.x` retrofit.

## Rules-as-data surface

Per `decisions.md §18`:

- Numerical effects are posted as data. Pre-computed values preferred.
- No runtime die-rolling.
- Real-time engines are NOT in this bundle's surface.
- Rules-data engines land only when strictly necessary to satisfy
  TR-30-002.

## Identifier shape

Per `decisions.md §6` and the 2026-07-17 identifier-discipline doctrine:

- `pub const <NAME>: &[<RecordType>]` slices under each book's tree.
- PascalCase for `NAME` and `RecordType`. lowercase camelCase for variables.
- No `sd30_*` / `SD30_*` patterns.
- The reach gate's filesystem-scan invariant is the load-bearing
  identifier shape; preserve it across all per-book cycles.

## Hard-stop conditions

A cycle stops and reports the blocker when:

- The reach gate fails. Cycle reports the record id, gate's IPC output, and the gap.
- The cross-bundle class id (Occultist, Spiritualist, Medium, Mesmerist) cannot be loaded. Cycle pauses; surfaces as a blocker for SD-28's progress.
- The trap-report finds new trap patterns. Cycle records the trap; pre-cycle trap-report for the next book re-runs with the trap catalog updated.
- The 4-grep audit fails. Cycle reports the grep and the offending line.
- A Mythic / Occult / Inner Sea record needs the consumer surface to reach the gate. Cycle pauses on `decision-blocked`; operator decides per-cycle.
