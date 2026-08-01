---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/9 (operator directive 2026-08-01)
build_version_target: 0.9.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-29 Epic Breakdown

9 epics × ~3-4 acceptance criteria = ~30 criteria. Mirrors SD-22's epic
shape with the four-bestiary expansion.

Epic 1 fires FIRST. Epic 8 fires LAST. Epics 3-6 (per-bestiary) may run
in any order post-Epic 2, but each bestiary is one cycle-batch.

## Epic 1 (SD29-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §6` (Identifier discipline).

### Feature seeds

#### SD29-E1-F1 — Identifier-disclosure audit pass

Acceptance:

- No `sd29_*`, `SD29_*`, `Sd29*`, `sd29-*` patterns in the four bestiaries' surface code (`src/rules_core/rules_tables/beastiary{2,3,4,5}/`).
- No `t_<hex>` kanban tokens in source files.
- Identifier-discipline audit script returns 0 findings.

#### SD29-E1-F2 — Schema-side grep

Acceptance:

- The four-grep dual-audit (identifier-discipline + wired-integration) runs cleanly post-Epic-1 commit.

## Epic 2 (SD29-E2) — Operator Pre-Launch

**Objective:** Pre-launch checklist verification before any per-book cycle fires.

**Derived from:** `loop-instruction.md §"Pre-launch checklist"` + operator directives 2026-08-01.

### Feature seeds

#### SD29-E2-F1 — Local-file dispatch readiness

Acceptance:

- `kanban.md` lists at least one ready card (B2-B5 per-book cycles).
- `progress.md` exists with first-cycle placeholder.
- Working tree clean (`git status` returns no uncommitted changes).

#### SD29-E2-F2 — Branch-pushed + cycle-0 trap-report + work-inventory

Acceptance:

- Branch `tranche/9` is pushed to origin (`git push -u origin tranche/9` succeeds).
- `cargo run --locked --bin v06_work_inventory` regenerated `docs/work-inventory.json`; the four bestiaries' entries confirm per-book shape (kinds, files_not_enumerated, trap_hits, reconciliation).
- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` ran for all four bestiaries; output recorded in `artifacts/<book>-cycle0-trap-report.md`.
- For Bestiary 5: the trap-report + inventory confirms zero `monster` units; Epic 5 cycle runs player-options cycles instead. Cycle proceeds.

## Epic 3 (SD29-E3) — Bestiary 2 content-source ingest

**Objective:** Per-monster-block cycles for Bestiary 2.

**Derived from:** `scope-draft.md §"Book list" slot 1` + `decisions.md §11` (per-entity counts generated).

### Feature seeds

#### SD29-E3-F1 — Monster records

Acceptance:

- One canonical record per monster block in `src/rules_core/rules_tables/beastiary2/`.
- Reach-gate claim executes the real IPC builder for each monster.
- Trap-report output recorded in `artifacts/b2-trap-report.md`.

#### SD29-E3-F2 — Race-trait population

Acceptance:

- Bestiary 2 also carries a large `race_trait` population (per the 07-30 shape finding). Cycles write canonical race-trait records alongside the monster blocks.

## Epic 4 (SD29-E4) — Bestiary 3 content-source ingest

**Objective:** Per-monster-block cycles for Bestiary 3.

**Derived from:** `scope-draft.md §"Book list" slot 2`.

### Feature seeds

#### SD29-E4-F1 — Monster records

#### SD29-E4-F2 — Race-trait population

(Parallel to SD29-E3.)

## Epic 5 (SD29-E5) — Bestiary 4 content-source ingest

**Objective:** Per-monster-block cycles for Bestiary 4.

**Derived from:** `scope-draft.md §"Book list" slot 3`.

### Feature seeds

#### SD29-E5-F1 — Monster records

#### SD29-E5-F2 — Race-trait population

(Parallel to SD29-E3.)

## Epic 6 (SD29-E6) — Bestiary 5 content-source ingest

**Objective:** Per-race / per-feat / per-companion-mod cycles for Bestiary 5 (player-options dataset, NOT a monster dataset).

**Derived from:** `scope-draft.md §"Book list" slot 4` + `decisions.md §18` (Bestiary 5 shape-resolution).

### Feature seeds

#### SD29-E6-F1 — Race records

Acceptance:

- Canonical race records in `src/rules_core/rules_tables/beastiary5/races/`.
- Reach-gate coverage for each race.

#### SD29-E6-F2 — Feat records

Acceptance:

- Canonical feat records per LST (e.g., `b5_feats.lst`).

#### SD29-E6-F3 — Companion-modifier records

Acceptance:

- Canonical companion-mod records per LST (e.g., `b5_companionmods.lst`).

## Epic 7 (SD29-E7) — DM Toolkit extension (consume Bestiary 2-5)

**Objective:** Extend `src/rules_core/encounters.rs` + `src/rules_core/party_cr.rs` to consume Bestiary 2-5's monster records.

**Derived from:** `scope-draft.md §"Out of cycle ingestion and surfacing"` + `forward-scope-register.md C1.2` + `decisions.md §19` (reach-gate-doD).

**Status:** Operator-pinned whether in scope per-cycle at Epic 5/6 closure. If Epic 7 lands inside SD-29, it satisfies the reach gate's `OPEN_FINDINGS` Bestiary-1-monster-surface prerequisite. If deferred to a separate bundle, cycles pause on `decision-blocked` until the surface is built.

### Feature seeds

#### SD29-E7-F1 — Encounter builder extension

Acceptance:

- `encounters.rs` reads Bestiary 2-5 monsters (currently Bestiary 1 only).

#### SD29-E7-F2 — Party-CR math extension

Acceptance:

- `party_cr.rs` reads Bestiary 2-5 monsters for CR calculations.

## Epic 8 (SD29-E8) — Closure Epilogue

**Objective:** Tranche promotion PR fires after all per-book + Epic 7 (if in scope) cycles closed.

**Derived from:** `decisions.md §13` (operating form) + the build-version amendment (2026-07-17).

### Feature seeds

#### SD29-E8-F1 — Closure cycle

Acceptance:

- All Epic 3-6 per-book cycles `complete` in `progress.md`.
- Epic 7 (if in scope) `complete`.
- `release-notes.md` populated with the four bestiaries' per-record rollup.
- Tranche promotion PR fires: `tranche/9 → develop`; `0.9.<last_build>` remains the post-closure value.

#### SD29-E8-F2 — Workspace-tree removal (move-not-copy)

Acceptance:

- The source-of-record directory removed on the publish commit per `decisions.md §13`.
- The canonical repo-resident home is `docs/release/SD-29-bestiary-2-3-4-5-content-ingestion/`.

## Epic 9 (SD29-E9) — Build Version Numbering

**Objective:** First concrete build value per the 2026-07-17 amendment.

**Derived from:** `decisions.md §14`.

### Feature seeds

#### SD29-E9-F1 — Version patch

Acceptance:

- First concrete value: `0.9.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Epic 8 increments per the 2026-07-17 build-version amendment.
- Major remains `0` until first main-publish.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → E3, E4, E5, E6 (any order, file-disjoint) → E7 (gated) → E9 → E8
```

Per-book epics are file-disjoint by source path (each writes to its own
`src/rules_core/rules_tables/beastiary<N>/`), so they can run in parallel
under operator-pinned concurrency. Epic 7 (DM Toolkit extension) is
gated on Epic 3-6 closure per `decisions.md §18`.

## Completion gate

SD-29 closes when:

- All Epic 3-6 per-book cycles `complete` with reach-gate claims and trap-report outputs.
- Epic 7 (DM Toolkit extension) lands (in scope) or surfaces as a Class 1/3 retrofit.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/9 → develop` is opened and merged.
- `docs/release/SD-29-bestiary-2-3-4-5-content-ingestion/` carries the canonical 15-file chassis (post-move-not-copy publish).
