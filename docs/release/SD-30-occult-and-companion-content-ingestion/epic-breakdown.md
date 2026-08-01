---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/10 (operator directive 2026-08-01)
build_version_target: 0.10.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-30 Epic Breakdown

9+ epics × ~3 acceptance criteria each = ~30+ criteria. Mirrors SD-22's
epic shape with the sixteen-book expansion. Per-book epics may group
Inner Sea's nine modules into one shared epic, or split per book; the
boundary is decided at Cycle 2's inventory gate.

Epic 1 fires FIRST. Closure fires LAST. Per-book epics may run in any
order post-Epic 2, but each book is one cycle-batch.

## Epic 1 (SD30-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §6` (Identifier discipline).

### Feature seeds

#### SD30-E1-F1 — Identifier-disclosure audit pass

Acceptance:

- No `sd30_*`, `SD30_*`, `Sd30*`, `sd30-*` patterns in the surface code (`src/rules_core/rules_tables/<book>/` for the sixteen in-scope books).
- No `t_<hex>` kanban tokens.
- Identifier-discipline audit script returns 0 findings.

## Epic 2 (SD30-E2) — Operator Pre-Launch

**Objective:** Pre-launch checklist verification + cycle-0 trap-report + work-inventory validation against all sixteen books.

**Derived from:** `loop-instruction.md §"Pre-launch checklist"` + operator directives 2026-08-01.

### Feature seeds

#### SD30-E2-F1 — Local-file dispatch readiness

Acceptance:

- `kanban.md` lists sixteen per-book epics as ready cards.
- `progress.md` exists with first-cycle placeholder.
- Working tree clean.

#### SD30-E2-F2 — Branch-pushed + cycle-0 trap-report + work-inventory

Acceptance:

- Branch `tranche/10` is pushed to origin.
- `cargo run --locked --bin v06_work_inventory` regenerated `docs/work-inventory.json`; the sixteen books' entries confirm per-book shape.
- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` ran for all sixteen books; output recorded in `artifacts/<book>-cycle0-trap-report.md`.

## Epic 3 (SD30-E3) — Occult Adventures content-source ingest

**Objective:** Per-class / per-monster-block / per-psychic-discipline cycles for Occult Adventures.

**Derived from:** `scope-draft.md §"Book list" slot 1` + `decisions.md §18` (reach-gate = DoD).

### Feature seeds

#### SD30-E3-F1 — Class records (Occultist, Spiritualist, Medium, Mesmerist + others)

Acceptance:

- One canonical record per class in `src/rules_core/rules_tables/occult_adventures/`.
- Reach-gate claim executes the real IPC builder for each class.

#### SD30-E3-F2 — Psychic-discipline records

Acceptance:

- Canonical psychic-discipline records per `psychic_discipline_*` LST files.

## Epic 4 (SD30-E4) — Horror Adventures content-source ingest

**Objective:** Per-monster-block / per-haunt-block / per-corruption-mechanic cycles.

**Derived from:** `scope-draft.md §"Book list" slot 2`.

### Feature seeds

#### SD30-E4-F1 — Monster records

## Epic 5 (SD30-E5) — Mythic Adventures content-source ingest

**Objective:** Per-mythic-path / per-monster-block cycles.

**Derived from:** `scope-draft.md §"Book list" slot 3` + `decisions.md §18` (Mythic Adventures reach-surface prerequisite).

### Feature seeds

#### SD30-E5-F1 — Mythic-path records

#### SD30-E5-F2 — Monster records

## Epic 6 (SD30-E6) — Monster Codex content-source ingest

**Objective:** Per-monster-block cycles for Monster Codex.

**Derived from:** `scope-draft.md §"Book list" slot 4`.

### Feature seeds

#### SD30-E6-F1 — Monster records (300+)

## Epic 7-N+ — Inner Sea World Guide + 9 Inner Sea modules content-source ingest

**Objective:** Per-trait / per-region / per-race / per-deity / per-spell / per-faction / per-rule cycles across the 10 Inner Sea books.

**Derived from:** `scope-draft.md §"Book list" slots 7-16`.

The nine Inner Sea modules may run as one shared epic or split into nine
separate epics; the boundary is decided at Cycle 2's inventory gate.

### Feature seeds (per Inner Sea module)

- Inner Sea World Guide: per-trait / per-feat / per-region cycles.
- Inner Sea Combat: per-trait / per-option cycles.
- Inner Sea Faiths: per-deity / per-trait / per-option cycles.
- Inner Sea Gods: per-deity / per-domain cycles.
- Inner Sea Magic: per-spell / per-magic-trait cycles.
- Inner Sea Races: per-race / per-archetype cycles.
- Inner Sea Temples: per-temple / per-trait cycles.
- Inner Sea Taverns: per-tavern / per-event cycles.
- Inner Sea Bestiary: per-monster-block cycles.
- Inner Sea Intrigue: per-trait / per-faction / per-rule cycles.

## Epic M+ — Book of the Damned ×2 content-source ingest

**Objective:** Per-archetype / per-monster-block / per-tactic cycles for both volumes.

**Derived from:** `scope-draft.md §"Book list" slots 5-6`.

### Feature seeds

#### SD30-E?-F1 — Volume 1 archetypes, monsters, tactics

#### SD30-E?-F2 — Volume 2 archetypes, monsters, tactics

## Closure Epilogue

### Feature seeds

#### Closure-F1 — Closure cycle

Acceptance:

- All per-book epics `complete` in `progress.md`.
- `release-notes.md` populated.
- Tranche promotion PR fires: `tranche/10 → develop`; `0.10.<last_build>` remains the post-closure value.

#### Closure-F2 — Workspace-tree removal (move-not-copy)

Acceptance:

- The source-of-record directory removed on the publish commit per `decisions.md §13`.
- The canonical repo-resident home is `docs/release/SD-30-occult-and-companion-content-ingestion/`.

## Build Version Numbering

#### SD30-E?-F1 — Version patch

Acceptance:

- First concrete value: `0.10.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Closure increments per the 2026-07-17 build-version amendment.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → {E3, E4, E5, E6, E7-N+, M+ Closure M+ Version} (any order post-E2, file-disjoint per book) → Closure → Version
```

Per-book epics are file-disjoint by source path (each writes to its own
`src/rules_core/rules_tables/<book>/`), so they can run in parallel
under operator-pinned concurrency. The Mythic Adventures reach-surface
precycle gating may pause Epic 5 if `reach_gate.rs OPEN_FINDINGS`
records missing-surface gaps.

## Completion gate

SD-30 closes when:

- All sixteen in-scope books' per-book cycles `complete` with reach-gate claims and trap-report outputs.
- Closure fires.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/10 → develop` is opened and merged.
- `docs/release/SD-30-occult-and-companion-content-ingestion/` carries the canonical 13+ file chassis (post-move-not-copy publish).
- Workspace source-of-record tree removed on publish.
