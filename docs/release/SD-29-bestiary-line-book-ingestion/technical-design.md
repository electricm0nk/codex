---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/9 (operator directive 2026-08-01)
build_version_target: 0.9.<build>
companion_to: ./technical-requirements.md, ./decisions.md
---

# SD-29 Technical Design

## Architecture overview

SD-29 extends the per-book ingest pipeline established by SD-22 (Bestiary
1 baseline) and refined by SD-27 (Shape B schema + license-stripping
pattern). The architectural surface is the same as SD-28's:

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
                       │  reach_gate.rs claim │   ← definition of done
                       │  (apps/desktop/src-  │      per decisions.md §19
                       │   tauri/src/)        │
                       └──────────────────────┘
```

## Cycle paths

Each cycle writes to **one bestiary's tree**:

- Epic 3 → `src/rules_core/rules_tables/beastiary2/`
- Epic 4 → `src/rules_core/rules_tables/beastiary3/`
- Epic 5 → `src/rules_core/rules_tables/beastiary4/`
- Epic 6 → `src/rules_core/rules_tables/beastiary5/` (player-options shape; gated on cycle-0 trap-report + inventory per TR-29-008)

Per-bestiary cycles are file-disjoint; they can run in parallel. Within
a single bestiary, cycles are sequential (one cycle per file at a time).

## Bestiary 5 shape-resolution

Bestiary 5's corpus is `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_5/`.
Per the 07-30 shape finding, the corpus contains:

- `b5_races_pc.lst` (player-character races)
- `b5_races_companion.lst` (companion races)
- `b5_abilities_race*.lst` (race abilities, multiple files)
- `b5_feats.lst` (feats)
- `b5_companionmods.lst` (companion modifiers)
- `_bestiary_5_for_players.pcc` (the entry-file PCC marker)

The generator (`v06_work_inventory`) reports **zero `monster` units** for
Bestiary 5. Epic 6's cycle dispatch reads the inventory's `kinds` field;
if `monster` units = 0, the cycle runs per-record cycles against
`beastiary5/races/`, `beastiary5/feats/`, `beastiary5/companionmods/`
instead of the per-monster-block shape applied to Bestiary 2-4.

**Operator-on-request drop-in.** Epic 2's pre-flight runs the trap-report
+ inventory. If operator prefers Bestiary 6 + Bonus Bestiary over
Bestiary 5's player-options cycles, the swap fires before Epic 6 dispatches.
Record the swap in `progress.md` and Epic-6-F1's cycle receipt.

## Local-file dispatch (no Hermes board)

Cycle dispatch is local-file. The supervisor reads `kanban.md` at top of
each tick to identify the next ready card:

1. The bundle's first cycle (Epic 2's pre-flight) populates `kanban.md`
   with the four bestiaries' per-book epics (Epics 3-6) as ready cards.
2. Cycles claim cards by editing `kanban.md` (marking `claimed by`, `claimed at`,
   `cycle id`, etc.); the supervisor's file-touch partition ensures only one
   cycle claims a card at a time.
3. Per-cycle completion writes the cycle receipt to `progress.md` and
   closes the kanban card. The supervisor reads `progress.md` to verify
   completion before the next cycle claim.

## Cross-book monster reprint rule

Per `decisions.md §16`:

- Famous monsters frequently reappear across Bestiary 2-6 with wording
  changes. The newer book's stat block is doctrine; the older book's is
  errata.
- Bestiary 1 (closed in SD-22) carries the canonical baseline; new reprints
  are supersets or refinements, not contradictions.
- The trap-report's per-book `KEY:` namespace listing tells the cycle
  which prefix to search for when joining records across bestiaries.

## Rules-as-data surface

Per `decisions.md §19`:

- Numerical monster abilities (e.g., damage dice, attack bonuses) are
  posted as data. The runtime reads the precomputed value.
- No runtime die-rolling.
- Real-time engines are NOT in this bundle's surface.
- Rules-data engines land only when strictly necessary to satisfy
  TR-29-002.

## Identifier shape

Per `decisions.md §6` and the 2026-07-17 identifier-discipline doctrine:

- `pub const <NAME>: &[<RecordType>]` slices under each bestiary's tree.
- PascalCase for `NAME` and `RecordType`. lowercase camelCase for variables.
- No `sd29_*` / `SD29_*` patterns.
- The reach gate's filesystem-scan invariant
  (`pub const <NAME>: &[<RecordType>]` slices) is the load-bearing
  identifier shape; preserve it across all per-book cycles.

## Hard-stop conditions

A cycle stops and reports the blocker when:

- The reach gate fails. Cycle reports the record id, gate's IPC output, and the gap (record not surfaced).
- The cross-bundle monster-record join yields a duplicate canonical id. Cycle reports the conflict and the per-source-record-state; cycle-0 trap-report re-runs.
- The trap-report finds new trap patterns. Cycle records the trap; pre-cycle trap-report for the next bestiary re-runs with the trap catalog updated.
- The 4-grep audit fails. Cycle reports the grep and the offending line.
- Epic 7 (DM Toolkit extension) gates have not fired and the cycle's record needs the consumer surface to reach the gate. Cycle records `decision-blocked` in progress.md and moves to the next ready card in kanban.md; Epic 7's safe default is the C3.1 retrofit (no operator contact during unattended mode).
