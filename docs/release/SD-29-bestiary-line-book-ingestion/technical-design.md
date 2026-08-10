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

Each lane epic's cycle-batch writes to **one book's tree, for that lane's kind only**:

- **Epic 4 (Monster / Monster-Ability Chassis Lane)** — pilot: `src/rules_core/rules_tables/bonus_bestiary/` (14 monster + 17 monster_ability); then `beastiary2/`, `beastiary3/`, `beastiary4/`, `beastiary5/` (monster_ability only), `beastiary6/` (monster_ability only), `monster_codex/`.
- **Epic 5 (Race-Trait Lane)** — all seven books' `race_trait` share: `beastiary2/` (162), `beastiary3/` (799), `beastiary4/` (86), `beastiary5/` (63), `beastiary6/` (0), `bonus_bestiary/` (17), `monster_codex/` (14).
- **Epic 6 (Companion Lane)** — all seven books' `companion` share: `beastiary2/` (16), `beastiary3/` (85), `beastiary4/` (76), `beastiary5/` (57), `beastiary6/` (26), `bonus_bestiary/` (0), `monster_codex/` (15).
- **Epic 7 (Residual Proven-Path Content Lane)** — spell/equipment/feat/race/equipment_modifier/class, wherever each book carries them (Monster Codex carries the largest residual share: 24 spell + 45 equipment + 4 equipment_modifier + 32 feat).

A lane's per-book cycle-batches are file-disjoint from each other (different books) and from other
lanes touching the *same* book (different kind-scoped modules within that book's tree — TR-29-001).
They can run in parallel. Within a single book-and-lane cycle-batch, cycles are sequential (one
cycle per file at a time).

## Monster / monster-ability lane — pilot-then-extend (was "Bestiary 5 shape-resolution")

**Retired shape.** The old "Epic 6 gates on zero-monster inventory, falls back to per-race/per-feat
cycles" design assumed one epic per book. Under kind lanes this collapses: a book with zero
`monster` units (Bestiary 5, Bestiary 6) simply has no Epic 4 monster-chassis cycle-batch — its
`monster_ability` units are still Epic 4's, its `race_trait`/`companion` units are Epic 5's/Epic 6's,
same as every other book. No fallback cycle type is needed because the lane structure already
routes each unit to the lane that owns its kind.

**What replaces it.** Per `decisions.md §37.2`: Epic 4 pilots the merged monster-chassis +
monster-ability mechanism against **Bonus Bestiary** (34 units total, the smallest monster-bearing
book) before extending to the other six books. Bestiary 5's corpus
(`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_5/`) still contains
`b5_races_pc.lst`, `b5_races_companion.lst`, `b5_abilities_race*.lst`, `b5_feats.lst`,
`b5_companionmods.lst`, `_bestiary_5_for_players.pcc` — the generator reports zero `monster` units
and 39 `monster_ability` units for it; Epic 4's extension phase includes Bestiary 5's
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
