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

13 epics × ~3-4 acceptance criteria = ~46 criteria. Mirrors SD-22's epic
shape with the seven-book expansion (Epics 3-6 and 11-13; `decisions.md
§34`), plus Epic 10's end-of-run code review (operator directive
2026-08-01, added post-authoring).

Epic 1 fires FIRST. Epic 8 fires LAST. Epics 3-6, 11-13 (per-book) may run
in any order post-Epic 2, but each book is one cycle-batch. Epic 10
(Bundle Code Review) fires after Epic 9 and every content-ingest epic
(including Epic 7, if in scope), before Epic 8.

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
- For Bestiary 5: the trap-report + inventory confirms zero `monster` units; Epic 6 cycle runs player-options cycles instead. Cycle proceeds.

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

**Derived from:** `scope-draft.md §"Out of cycle ingestion and surfacing"` + `successor-forward-scope-register.md C1.2` + `decisions.md §19` (reach-gate-doD).

**Status:** Operator-pinned whether in scope per-cycle at Epics 5 and 6 closure. If Epic 7 lands inside SD-29, it satisfies the reach gate's `OPEN_FINDINGS` Bestiary-1-monster-surface prerequisite. If deferred to a separate bundle, cycles record `decision-blocked` in `progress.md` and move to the next ready card.

### Feature seeds

#### SD29-E7-F1 — Encounter builder extension

Acceptance:

- `encounters.rs` reads Bestiary 2-5 monsters (currently Bestiary 1 only).

#### SD29-E7-F2 — Party-CR math extension

Acceptance:

- `party_cr.rs` reads Bestiary 2-5 monsters for CR calculations.

## Epic 8 (SD29-E8) — Closure Epilogue

**Objective:** Tranche promotion PR fires after all per-book + Epic 7 (if in scope) cycles closed.

**Derived from:** `decisions.md §23` (operating form) + the build-version amendment (2026-07-17).

### Feature seeds

#### SD29-E8-F1 — Closure cycle

Acceptance:

- All Epic 3-6 per-book cycles `complete` in `progress.md`.
- Epic 7 (if in scope) `complete`.
- `release-notes.md` populated with the four bestiaries' per-record rollup.
- Tranche promotion PR fires: `tranche/9 → develop`; `0.9.<last_build>` remains the post-closure value.

#### SD29-E8-F2 — Workspace-tree removal (move-not-copy)

Acceptance:

- The source-of-record directory removed on the publish commit per `decisions.md §14`.
- The canonical repo-resident home is `docs/release/SD-29-bestiary-line-book-ingestion/`.

## Epic 9 (SD29-E9) — Build Version Numbering

**Objective:** First concrete build value per the 2026-07-17 amendment.

**Derived from:** `decisions.md §14`.

### Feature seeds

#### SD29-E9-F1 — Version patch

Acceptance:

- First concrete value: `0.9.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Epic 8 increments per the 2026-07-17 build-version amendment.
- Major remains `0` until first main-publish.

## Epic 10 (SD29-E10) — Bundle Code Review

**Objective:** A full code review of the bundle's entire diff against its
branch point, run after every content-ingest epic (3-6, 11-13), Epic 7 (DM
Toolkit extension, if in scope), and Epic 9 (Build Version Numbering) are closed —
not in parallel with them, and not scoped to only the final cycle.
`./scripts/verify.sh` passing is a **precondition** to this epic firing, not
the review itself: a green gate says the tests that exist pass, it says
nothing about whether the code is right.

**Derived from:** operator directive 2026-08-01 (the v0.6 CRB run closed
without an end-of-run code review) + `decisions.md §27`.

### Feature seeds

#### SD29-E10-F1 — Whole-bundle diff review

Acceptance:

- The reviewed diff scope is the bundle's full change set against its branch
  point (`git diff origin/develop...HEAD`, the same merge-base triple-dot
  comparison `scripts/identifier-discipline-audit.sh` and
  `scripts/wired-integration-audit.sh` already default to via
  `BASE_BRANCH`), not the closing cycle's slice alone.
- `./scripts/verify.sh` has a recorded green run for that diff, cited as a
  precondition in the epic's receipt.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh`
  (this bundle's standing per-cycle dual-audit) are re-run once more at
  bundle scope.

#### SD29-E10-F2 — Correctness, no-stub, reach, test-quality, no-hand-authored-frontend-data sweep

Acceptance:

- A sample of this bundle's rules logic is checked against the source corpus
  for the four bestiaries (monster blocks, race-traits, Bestiary 5's
  player-options); disagreements are recorded as findings, not assumed away.
- No stub, fixture-only, or mock data in a production path per
  `docs/governance/no-stub-mvp-doctrine.md`; any operator-approved exception
  is entered in `docs/governance/wired-integration-stubs-registry.md`, not
  left unregistered.
- A sample of records this bundle claims reach a player surface is spot-checked
  against `reach_gate.rs`'s `OPEN_FINDINGS` mechanism and the live IPC/UI
  path — including the Epic 7 DM Toolkit consumer surface, if in scope.
  Reach-gate green is necessary, not sufficient, on its own.
- Test quality, not just count: per
  `docs/governance/book-ingestion-playbook.md §7.4`, a sample of this
  bundle's new gates/tests is checked for a case that actually fails when the
  thing it protects is broken, not only a case that passes.
- No hand-authored rules data under `apps/desktop/src/` — rules content is
  sourced from `src/rules_core/rules_tables/`, never hand-typed into a
  frontend file.

#### SD29-E10-F3 — Findings triage

Acceptance:

- Every finding records a severity and a disposition: `fixed-in-bundle` or
  `deferred`. No finding is silently dropped.
- A `deferred` finding names an owner (a person or a specific successor
  bundle) and is entered in `successor-forward-scope-register.md` — not left
  unrecorded.
- Real defects found are fixed in-bundle before Epic 8 (Closure Epilogue)
  fires; the review does not become a rubber stamp that defers everything to
  avoid scope growth.
- A `scripts/retro.py` event is emitted per finding, carrying `--verified-by`.

**Note:** the operator can separately trigger `/code-review ultra` (a
multi-agent cloud review of the branch) at any time. That path is
operator-triggered and billed — a cycle running under §22's unattended-mode
protocol cannot launch it itself — so Epic 10 must stand on its own as the
bundle's actual gate; `/code-review ultra` is a supplement, not a dependency.

## Epic 11 (SD29-E11) — Bestiary 6 content-source ingest

**Objective:** Per-race / per-class-feature / per-companion cycles for
Bestiary 6 — a player-options dataset, the same shape as Bestiary 5. Zero
monsters is the correct shape for this book, confirmed by
`forward-scope-register.md §1.3` and the cycle-0 work-inventory run; it is
not a sign of an incomplete cycle plan.

**Derived from:** `decisions.md §34` (seven-book scope) +
`forward-scope-register.md §1.1/§1.3` (63 units: 22 class_feature, 13
race_trait, 2 spell, 26 companion).

### Feature seeds

#### SD29-E11-F1 — Race-trait records

Acceptance:

- Canonical race-trait records for Bestiary 6's 13 `race_trait` units in
  `src/rules_core/rules_tables/beastiary6/`.
- Reach-gate coverage for each race trait.

#### SD29-E11-F2 — Class-feature records

Acceptance:

- Canonical class-feature records for Bestiary 6's 22 `class_feature`
  units.
- Reach-gate coverage for each class feature.

#### SD29-E11-F3 — Companion + spell records

Acceptance:

- Canonical companion records for Bestiary 6's 26 `companion` units and
  spell records for its 2 `spell` units.
- Cycle-0 trap-report output confirms zero `monster` units before the
  cycle proceeds (per Epic 2's inventory gate).

## Epic 12 (SD29-E12) — Bonus Bestiary content-source ingest

**Objective:** Per-monster-block cycles for Bonus Bestiary's 14 monsters,
plus its 17 race-trait and 3 class units. The smallest of the seven books.

**Derived from:** `decisions.md §34` (seven-book scope) +
`forward-scope-register.md §1.1/§1.3` (34 units: 3 class, 17 race_trait,
14 monster; 4 `.lst` files including `bb_races.lst`).

### Feature seeds

#### SD29-E12-F1 — Monster records

Acceptance:

- One canonical record per monster block for Bonus Bestiary's 14 monsters
  in `src/rules_core/rules_tables/bonus_bestiary/`.
- Reach-gate claim executes the real IPC builder for each monster.
- Trap-report output recorded in `artifacts/bb-trap-report.md`.

#### SD29-E12-F2 — Race-trait population

Acceptance:

- Canonical race-trait records for the 17 `race_trait` units, including
  `bb_races.lst`'s base declarations.

#### SD29-E12-F3 — Class records

Acceptance:

- Canonical class records for the 3 `class` units.

## Epic 13 (SD29-E13) — Monster Codex content-source ingest

**Objective:** Per-record-family cycles for Monster Codex — **not**
per-monster-block. The book's payload is player-and-NPC options for races
that already exist (72 class features, 32 feats, 24 spells, 45 equipment,
4 equipment modifiers, 19 race-trait rows, 15 companion kits), plus only 2
monster declarations. Sizing this epic as a monster ingest produces a
cycle plan that does not match the book (`forward-scope-register.md
§1.3` Correction 2).

**Derived from:** `decisions.md §34` (seven-book scope) +
`forward-scope-register.md §1.1/§1.3` (213 units total) + `§1.2` (the
Duergar upstream-blocker finding).

### Feature seeds

#### SD29-E13-F1 — Class-feature and feat records

Acceptance:

- Canonical records for the 72 `class_feature` and 32 `feat` units in
  `src/rules_core/rules_tables/monster_codex/`.

#### SD29-E13-F2 — Spell and equipment records

Acceptance:

- Canonical records for the 24 `spell`, 45 `equipment`, and 4
  `equipment_modifier` units.

#### SD29-E13-F3 — Race-trait records, including the Duergar upstream-blocker retirement attempt

Acceptance:

- Canonical records for the 19 `race_trait` units read from
  `mc_abilities_race.lst` and the book's other race-trait sources.
- **This epic is the upstream blocker of the surviving
  `beastiary1/race_traits` `OPEN_FINDINGS` entry** — the Duergar
  `Spell-Like Ability ~ Invisibility` record
  (`forward-scope-register.md §1.2`, `tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs`).
  The cycle that ingests `mc_abilities_race.lst`'s
  `FACT:Duergar_ReplaceSLAEnlargePerson|True` row (`Duergar ~ Ironskinned`,
  line 16) is expected to retire that finding. The cycle records the
  outcome either way — the finding closing, or a documented reason it did
  not — rather than silently ingesting the row without checking.

#### SD29-E13-F4 — Companion-kit records

Acceptance:

- Canonical records for the 15 `companion` units (companion kits).
- Trap-report output recorded in `artifacts/mc-trap-report.md`.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → {E3, E4, E5, E6, E11, E12, E13} (any order, file-disjoint) → E7 (gated) → E9 → E10 → E8
```

Per-book epics are file-disjoint by source path (each writes to its own
`src/rules_core/rules_tables/<book>/`), so they can run in parallel
under operator-pinned concurrency. Epic 7 (DM Toolkit extension) is
gated on Epic 3-6, 11-13 closure per `decisions.md §19`. Epic 10 (Bundle Code
Review) runs after every other epic but Epic 8 — any review finding is
fixed before the tranche-promotion PR (part of Epic 8) opens.

## Completion gate

SD-29 closes when:

- All Epic 3-6, 11-13 per-book cycles `complete` with reach-gate claims and trap-report outputs.
- Epic 7 (DM Toolkit extension) lands (in scope) or surfaces as a Class 1/3 retrofit.
- Epic 10 (Bundle Code Review) closed, all findings triaged with named owners for deferrals.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/9 → develop` is opened and merged.
- `docs/release/SD-29-bestiary-line-book-ingestion/` carries the canonical 14-file chassis (post-move-not-copy publish).
