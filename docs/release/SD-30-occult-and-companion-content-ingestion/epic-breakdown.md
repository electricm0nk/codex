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

21 epics (matching `kanban.md`'s 21 cards: Epic 1-2 gates, 16 per-book
epics, Build Version Numbering, Bundle Code Review, Closure) × ~3
acceptance criteria each. Epic 21's end-of-run code review was added by
operator directive 2026-08-01, numbered per `kanban.md`'s existing
`epic-1`...`epic-20` scheme. Mirrors SD-22's epic shape with the
sixteen-book expansion.
Per-book epics may group Inner Sea's nine modules into one shared epic,
or split per book; the boundary is decided at Cycle 2's inventory gate.

Epic 1 fires FIRST. Closure fires LAST. Per-book epics may run in any
order post-Epic 2, but each book is one cycle-batch. Epic 21 (Bundle
Code Review) fires after Build Version Numbering and every content
epic, before Closure.

## Epic 1 (SD30-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §7` (Identifier discipline).

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

### Feature seeds (per Inner Sea book — World Guide plus the nine modules)

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

#### Closure-F2 — Workspace-tree removal (move-not-copy) — LANDED, verify at closure

Acceptance (the publish landed 2026-08-01; Closure re-verifies rather than performs):

- The source-of-record workspace directory was removed on the publish commit per the move-not-copy doctrine (`forward-scope-register.md` Class 0 anchor "Move-not-copy publish"; `acceptance-and-verification.md AT-30-011`).
- The canonical repo-resident home is `docs/release/SD-30-occult-and-companion-content-ingestion/`.

## Build Version Numbering

#### SD30-E?-F1 — Version patch

Acceptance:

- First concrete value: `0.10.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Closure increments per the 2026-07-17 build-version amendment.

## Epic 21 (SD30-E21) — Bundle Code Review

**Objective:** A full code review of the bundle's entire diff against its
branch point, run after every content-ingest epic (3 through the Inner
Sea/Book of the Damned set) and Build Version Numbering are closed — not in
parallel with them, and not scoped to only the final cycle.
`./scripts/verify.sh` passing is a **precondition** to this epic firing, not
the review itself: a green gate says the tests that exist pass, it says
nothing about whether the code is right.

**Derived from:** operator directive 2026-08-01 (the v0.6 CRB run closed
without an end-of-run code review) + `decisions.md §26`.

### Feature seeds

#### SD30-E21-F1 — Whole-bundle diff review

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

#### SD30-E21-F2 — Correctness, no-stub, reach, test-quality, no-hand-authored-frontend-data sweep

Acceptance:

- A sample of this bundle's rules logic is checked against the source corpus
  across the sixteen in-scope books; disagreements are recorded as findings,
  not assumed away.
- No stub, fixture-only, or mock data in a production path per
  `docs/governance/no-stub-mvp-doctrine.md`; any operator-approved exception
  is entered in `docs/governance/wired-integration-stubs-registry.md`, not
  left unregistered.
- A sample of records this bundle claims reach a player surface is spot-checked
  against `reach_gate.rs`'s `OPEN_FINDINGS` mechanism and the live IPC/UI
  path — including the Mythic Adventures reach-surface prerequisite (Epic 5).
  Reach-gate green is necessary, not sufficient, on its own.
- Test quality, not just count: per
  `docs/governance/book-ingestion-playbook.md §7.4`, a sample of this
  bundle's new gates/tests is checked for a case that actually fails when the
  thing it protects is broken, not only a case that passes.
- No hand-authored rules data under `apps/desktop/src/` — rules content is
  sourced from `src/rules_core/rules_tables/`, never hand-typed into a
  frontend file.

#### SD30-E21-F3 — Findings triage

Acceptance:

- Every finding records a severity and a disposition: `fixed-in-bundle` or
  `deferred`. No finding is silently dropped.
- A `deferred` finding names an owner (a person or a specific successor
  bundle) and is entered in `forward-scope-register.md` — not left
  unrecorded.
- Real defects found are fixed in-bundle before Closure Epilogue fires; the
  review does not become a rubber stamp that defers everything to avoid
  scope growth.
- A `scripts/retro.py` event is emitted per finding, carrying `--verified-by`.

**Note:** the operator can separately trigger `/code-review ultra` (a
multi-agent cloud review of the branch) at any time. That path is
operator-triggered and billed — a cycle running under §21's unattended-mode
protocol cannot launch it itself — so Epic 21 must stand on its own as the
bundle's actual gate; `/code-review ultra` is a supplement, not a dependency.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → {E3, E4, E5, E6, E7-N+, M+} (any order post-E2, file-disjoint per book) → Version → E21 (Bundle Code Review) → Closure
```

Corrected here: the prior diagram embedded "Closure"/"Version" inside the
parallel set and then repeated `→ Closure → Version` at the end, which both
contradicted each other and contradicted `loop-instruction.md §"Epic
ordering"`'s explicit statement that Build Version Numbering fires before
Closure and Closure fires LAST. The corrected order matches
`loop-instruction.md`.

Per-book epics are file-disjoint by source path (each writes to its own
`src/rules_core/rules_tables/<book>/`), so they can run in parallel
under operator-pinned concurrency. The Mythic Adventures reach-surface
precycle gating may pause Epic 5 if `reach_gate.rs OPEN_FINDINGS`
records missing-surface gaps. Epic 21 (Bundle Code Review) runs after every
other epic but Closure — any review finding is fixed before the
tranche-promotion PR (part of Closure) opens.

## Completion gate

SD-30 closes when:

- All sixteen in-scope books' per-book cycles `complete` with reach-gate claims and trap-report outputs.
- Epic 21 (Bundle Code Review) closed, all findings triaged with named owners for deferrals.
- Closure fires.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/10 → develop` is opened and merged.
- `docs/release/SD-30-occult-and-companion-content-ingestion/` carries the canonical 13+ file chassis (post-move-not-copy publish).
- Workspace source-of-record tree removed on publish.
