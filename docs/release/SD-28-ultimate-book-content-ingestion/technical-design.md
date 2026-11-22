---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/8 (operator directive 2026-08-01)
build_version_target: 0.8.<build>
companion_to: ./technical-requirements.md, ./decisions.md
---

# SD-28 Technical Design

## Architecture overview

SD-28 extends the per-book ingest pipeline established by SD-22 (APG, ACG,
Bestiary 1, DM toolkit) and refined by SD-27 (Shape B schema + license-
stripping pattern). The architectural surface is:

```
                    ┌──────────────────────┐
                    │  v06_corpus_trap_report│ (pre-cycle)
                    └──────────┬───────────┘
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
                       │  reach_gate.rs claim │   ← definition of done
                       │  (apps/desktop/src-  │      per decisions.md §18
                       │   tauri/src/)        │
                       └──────────────────────┘
```

## Cycle paths

Each cycle writes to **one book's tree**:

- Epic 3 → `src/rules_core/rules_tables/ultimate_combat/`
- Epic 4 → `src/rules_core/rules_tables/ultimate_magic/`
- Epic 5 → `src/rules_core/rules_tables/ultimate_equipment/`
- Epic 6 → `src/rules_core/rules_tables/ultimate_intrigue/`
- Epic 7 → `src/rules_core/rules_tables/ultimate_campaign/`
- Epic 8 → `src/rules_core/rules_tables/ultimate_wilderness/`
- Epic 9 → `src/rules_core/rules_tables/ultimate_psionics/` (third-party tier)

Per-book cycles are file-disjoint; they can run in parallel. Within a
single book, cycles are sequential (one cycle per file at a time).

## Dreamscarred Press third-party tier

`ultimate_psionics` is gated on:

1. Pre-cycle licensing verification (TR-28-008). Trap-report output against
   `dreamscarred_press/ultimate_psionics/`; license-conformance per record.
2. Each per-cycle record reviewed against the PSPF PI-list (Product Identity
   in Pi-based games; analog to the OGL PI-blacklist per `decisions.md §17`).
3. Records that fail the licensing audit drop from cycle scope. The cycle
   records the drop with a per-record justification.

The third-party tier's read-from corpus is the same Shape B cache shape as
the Paizo tiers, but the LST source directory differs. The cache build
command (`cargo run --locked --bin sd28_gen_book_cache -- <book>`) takes a
book id and reads the right corpus dir; for UPsi, it reads
`dreamscarred_press/ultimate_psionics/`.

## Local-file dispatch (no Hermes board)

Cycle dispatch is local-file. The supervisor reads `kanban.md` at top of
each tick to identify the next ready card:

1. The bundle's first cycle (Epic 2's pre-flight) populates `kanban.md`
   with the seven books' per-book epics (Epics 3-9) as ready cards.
2. Cycles claim cards by editing `kanban.md` (marking `claimed by`, `claimed at`,
   `cycle id`, etc.); the supervisor's file-touch partition ensures only one
   cycle claims a card at a time.
3. Per-cycle completion writes the cycle receipt to `progress.md` and
   closes the kanban card. The supervisor reads `progress.md` to verify
   completion before the next cycle claim.

## Cross-bundle class overlap (UI ∩ SD-30)

Per `decisions.md §5`:

- For the four shared classes (Occultist, Spiritualist, Medium, Mesmerist),
  the canonical class definition lives in SD-30's `src/rules_core/rules_tables/occult_adventures/`.
- SD-28 references the canonical class id from SD-30 in
  `src/rules_core/rules_tables/ultimate_intrigue/`. No redefinition.
- A `use crate::occult_adventures::occultist::*;` import (or the
  Rust-canonical equivalent) carries the dependency.

Cross-book conflict rule (TR-28-007) applies after both bundles land:
records that are the same class but have different feature definitions
between the two books resolve via the newer-book-wins rule.

## Rules-as-data surface

Per `decisions.md §18`:

- Numerical effects are posted as data. A 1d6/level fireball for a caster
  level of 6 carries `6d6` in the spell description's precomputed field.
- No runtime die-rolling. `pilot_compute.rs` reads the precomputed value.
- Real-time engines are NOT in this bundle's surface.
- Rules-data engines (e.g., a feat with branching condition evaluator)
  land in the cycle's source file. Branching cases resolve to
  pre-computed values where possible; runtime evaluation is the last
  resort.

## Identifier shape

Per `decisions.md §6` and the 2026-07-17 identifier-discipline doctrine:

- `pub const <NAME>: &[<RecordType>]` slices under each book's tree.
- PascalCase for `NAME` and `RecordType`. lowercase camelCase for
  variables. No `sd28_*` / `SD28_*` patterns.
- The reach gate's filesystem-scan invariant
  (`pub const <NAME>: &[<RecordType>]` slices) is the load-bearing
  identifier shape; preserve it across all per-book cycles.

## Hard-stop conditions

A cycle stops and reports the blocker when:

- The reach gate fails (definition-of-done not met). The cycle reports the
  record id, the gate's IPC output, and the gap (record not surfaced).
- The cross-bundle class id (`occult_adventures`) cannot be loaded. Cycle
  pauses; surfaces as a blocker for SD-30's progress.
- The trap-report finds new trap patterns. Cycle records the trap;
  pre-cycle trap-report for the next book re-runs with the trap catalog
  updated.
- The 5-grep audit fails. Cycle reports the grep and the offending line.
