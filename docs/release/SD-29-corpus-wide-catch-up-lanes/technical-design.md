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

**Re-cut 2026-08-10 (`decisions.md §37`).** The architecture below is unchanged in its mechanics
(trap-report → Shape B cache → `rules_tables/<book>/` → reach gate); what changed is which epic
owns which write, and a new provenance stage. Cycles now dispatch by **kind lane** across all seven
books, not by per-book epic against a four-or-seven-book list.

**RE-SCOPED CORPUS-WIDE, 2026-08-10 (`decisions.md §38`).** "All seven books" above is now "all 37
in-scope books" (`../corpus-work-channels.md §10.2`) — the mechanics (trap-report → Shape B cache →
`rules_tables/<book>/` → reach gate) are unaffected, since the architecture was already
book-parameterized; only the range of `<book>` widens. Epic numbers below shifted: Epic 4 is now
the corpus-wide proven-path lane; the Monster/Monster-Ability chassis lane (this file's primary
subject in several sections) is now Epic 5, not Epic 4.

## Architecture overview

SD-29 extends the per-book ingest pipeline established by SD-22 (Bestiary
1 baseline) and refined by SD-27 (Shape B schema + license-stripping
pattern). The architectural surface is the same as SD-28's, with a new
provenance stage inserted before the table lands (`decisions.md §37.3`):

```
                    ┌──────────────────────────┐
                    │  v06_corpus_trap_report  │ (pre-cycle, every book,
                    └──────────┬───────────────┘  corpus-wide pre-flight)
                               │ findings
                               ▼
┌──────────────────┐   ┌──────────────────────┐
│  corpus LST/JSON │──▶│  Shape B cache       │   ← SD-27-owned
└──────────────────┘   │  data/corpus/<book>/ │      schema
                       └──────────┬───────────┘
                                  │ records
                                  ▼
                       ┌──────────────────────┐
                       │  pi_screening::      │   ← Epic 3 gate, new
                       │  classify_field      │      2026-08-10
                       └──────────┬───────────┘
                                  │ screened records
                                  ▼
                       ┌──────────────────────┐
                       │  src/rules_core/     │
                       │  rules_tables/<book>/│   ← per-book source,
                       └──────────┬───────────┘      populated lane by lane
                                  │ slices
                                  ▼
                       ┌──────────────────────┐
                       │  reach_gate.rs claim │   ← definition of done
                       │  (apps/desktop/src-  │      per decisions.md §19
                       │   tauri/src/)        │
                       └──────────────────────┘
```

## Cycle paths — by kind lane, fanning out per book

*(Section rewritten 2026-08-10 to the `decisions.md §38` corpus-wide lane mapping; the previous
body enumerated the retired seven-book cut under the pre-§37 epic numbering.)*

Each lane epic's cycle-batch writes to **one book's tree, for that lane's kind only**:

- **Epic 4 (Proven-Path Content Lanes — day-one, corpus-wide)** — equipment (1,163 remaining),
  feat (1,350), spell (1,754), equipment_modifier (812), race (96), class (158): settled
  per-book table method, every book in the 37-book product with remaining units.
- **Epic 5 (Monster / Monster-Ability Chassis Lane)** — `monster` (1,224 remaining, 14 books) +
  `monster_ability` (3,107 remaining, 24 books) as one chassis-plus-features build; pilot:
  `src/rules_core/rules_tables/bonus_bestiary/` (14 monster + 17 monster_ability), then every
  remaining monster/monster_ability-bearing book.
- **Epic 6 (Race-Trait Lane)** — `race_trait` (3,412 remaining, 27 books); the classifier's
  name-coincidence grounding defect is fixed alongside the build; pilot: `inner_sea_intrigue`
  (9 units), then corpus-wide.
- **Epic 7 (Companion Lane)** — `companion` (1,683 remaining, 17 books; no ingest path exists
  anywhere in the corpus); pilot: `inner_sea_combat` (10 units), then corpus-wide.

Per-kind figures are `decisions.md §38.1`'s 2026-08-10 snapshot; every cycle-batch re-derives its
own live counts from `docs/work-inventory.json` before sizing — the snapshot is not a dispatch
list.

A lane's per-book cycle-batches are file-disjoint from each other (different books) and from other
lanes touching the *same* book (different kind-scoped modules within that book's tree — TR-29-001).
They can run in parallel. Within a single book-and-lane cycle-batch, cycles are sequential (one
cycle per file at a time).

## Monster / monster-ability lane — pilot-then-extend (Epic 5; was "Bestiary 5 shape-resolution")

**Retired shape.** The old "Epic 6 gates on zero-monster inventory, falls back to per-race/per-feat
cycles" design assumed one epic per book. Under kind lanes this collapses: a book with zero
`monster` units (Bestiary 5, Bestiary 6) simply has no Epic 4 monster-chassis cycle-batch — its
`monster_ability` units are still Epic 4's, its `race_trait`/`companion` units are Epic 5's/Epic 6's,
same as every other book. No fallback cycle type is needed because the lane structure already
routes each unit to the lane that owns its kind.

**What replaces it.** Per `decisions.md §37.2` (now corpus-wide per `§38.3`): Epic 5 pilots the
merged monster-chassis + monster-ability mechanism against **Bonus Bestiary** (14 monster + 17
monster_ability = 31 remaining units, the smallest non-degenerate monster-bearing book) before
extending to every remaining monster/monster_ability-bearing book (14 `monster` books, 24
`monster_ability` books per `decisions.md §38.1`). Bestiary 5's corpus
(`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_5/`) still contains
`b5_races_pc.lst`, `b5_races_companion.lst`, `b5_abilities_race*.lst`, `b5_feats.lst`,
`b5_companionmods.lst`, `_bestiary_5_for_players.pcc` — the generator reports zero `monster` units
and 39 `monster_ability` units for it; Epic 5's extension phase includes Bestiary 5's
`monster_ability` share once the pilot lands, same as it includes Bestiary 6's 13.

## Local-file dispatch (no Hermes board)

Cycle dispatch is local-file. The supervisor reads `kanban.md` at top of
each tick to identify the next ready card:

1. The bundle's first cycle (Epic 2's pre-flight) populates `kanban.md`
   with the four lane epics (Epics 4-7) as ready cards, each fanning out
   per book internally.
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
  are supersets or refinements, not contradictions. "Canonical baseline" is
  an identity/authority claim, not a completeness claim — Bestiary 1 is
  4.1% proven (42/1,027 units) as measured 2026-08-02 (`decisions.md §35`).
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
- Epic 8 (DM Toolkit extension) gates have not fired and the cycle's record needs the consumer surface to reach the gate. Cycle records `decision-blocked` in progress.md and moves to the next ready card in kanban.md; Epic 8's safe default is the C3.1 retrofit (no operator contact during unattended mode).
