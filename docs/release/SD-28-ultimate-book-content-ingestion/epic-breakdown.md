---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/8 (operator directive 2026-08-01)
build_version_target: 0.8.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-28 Epic Breakdown

12 epics × ~3 acceptance criteria = ~36 criteria. Mirrors SD-22's epic
shape with the seven-book expansion (UC, UM, UE, UI, UCam, UW, UPsi) →
seven per-book content-source-ingest epics, plus Epic 12's end-of-run
code review (operator directive 2026-08-01, added post-launch).

Epic 1 fires FIRST. Epic 10 (Closure Epilogue) fires LAST — corrected
here from a prior "Epic 11 fires LAST" statement that was never true
against this file's own "Recommended sequencing" diagram below, which
has always ordered `E11 → E10`. Epics 3-9 (per-book) may run in any
order post-Epic 2, but each book is one cycle-batch (no Epic 3 cycle
interleaves with Epic 4 cycle). Epic 12 (Bundle Code Review) fires
after Epic 11 and all content epics, before Epic 10.

## Epic 1 (SD28-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §6` (Identifier discipline).

### Feature seeds

#### SD28-E1-F1 — Identifier-disclosure audit pass

Acceptance:

- No `sd28_*`, `SD28_*`, `Sd28*`, `sd28-*` patterns in the seven books' surface code (`src/rules_core/rules_tables/ultimate_*/`, `src/rules_core/rules_tables/dreamscarred_press/`).
- No `t_<hex>` kanban tokens in source files.
- Identifier-discipline audit script returns 0 findings.

#### SD28-E1-F2 — Schema-side grep

Acceptance:

- The four-grep dual-audit (identifier-discipline + wired-integration) runs cleanly post-Epic-1 commit.
- The CLI flags any future-leaked `SD28-*` patterns with a one-line exit-code-1 message.

## Epic 2 (SD28-E2) — Operator Pre-Launch

**Objective:** Pre-launch checklist verification before any per-book cycle fires.

**Derived from:** `loop-instruction.md §"Pre-launch checklist"` + operator directives 2026-08-01.

### Feature seeds

#### SD28-E2-F1 — Local-file dispatch readiness

Acceptance:

- `kanban.md` lists at least one ready card.
- `progress.md` exists with first-cycle placeholder.
- Working tree clean (`git status` returns no uncommitted changes).

#### SD28-E2-F2 — Branch-pushed + licensing pre-cycle verification

Acceptance:

- Branch `tranche/8` is pushed to origin (`git push -u origin tranche/8` succeeds).
- The Dreamscarred Press licensing pre-cycle verification ran against `dreamscarred_press/ultimate_psionics/` and recorded its output in `artifacts/dreamscarred-license-precheck.md`.

## Epic 3 (SD28-E3) — Ultimate Combat content-source ingest

**Objective:** Per-class / per-chooser cycles for Ultimate Combat (Gunslinger, Ninja, Samurai, new martial rules).

**Derived from:** `scope-draft.md §"Book list" slot 1` + `decisions.md §11`.

### Feature seeds

#### SD28-E3-F1 — Class records (Gunslinger, Ninja, Samurai, others)

Acceptance:

- One canonical record per class in `src/rules_core/rules_tables/ultimate_combat/` (per `decisions.md §6` PascalCase / camelCase discipline).
- Reach-gate claim executes the real IPC builder for each class.
- Trap-report output recorded in `artifacts/uc-trap-report.md`.

#### SD28-E3-F2 — Chooser-shaped cycles (panache, grit, martial flexibility)

Acceptance:

- Each chooser mechanism has a record slice that the rules-engine can read.
- Per-cycle tier-2 model swap authorized per `decisions.md §11` (free/discounted) if dispatch is templated.

## Epic 4 (SD28-E4) — Ultimate Magic content-source ingest

**Objective:** Per-class + per-spell-subsystem cycles (new casting variants, words of power, truename).

**Derived from:** `scope-draft.md §"Book list" slot 2`.

### Feature seeds

#### SD28-E4-F1 — Class records (Magus, others)

#### SD28-E4-F2 — Spell subsystems (Words of Power, Truename)

## Epic 5 (SD28-E5) — Ultimate Equipment content-source ingest

**Objective:** Per-equipment-entry cycles.

**Derived from:** `scope-draft.md §"Book list" slot 3` + `decisions.md §10 (legacy §18-resolution direction)`.

### Feature seeds

#### SD28-E5-F1 — Equipment records

Acceptance:

- One canonical record per equipment entry in `src/rules_core/rules_tables/ultimate_equipment/`.
- Reach-gate coverage for equipment. **§10 (legacy) / §18 forces the catalog widening**: if `equipment_catalog.rs` is still CRB-only when this epic closes, the cycle records the gap as `decision-blocked` and surfaces the widening as C3.1 in the forward-scope register.

## Epic 6 (SD28-E6) — Ultimate Intrigue content-source ingest

**Objective:** Per-class / per-social-rule cycles (Vigilante, and other Ultimate Intrigue classes).

**Derived from:** `scope-draft.md §"Book list" slot 4` + `decisions.md §5` cross-bundle class overlap.

### Feature seeds

#### SD28-E6-F1 — Class records (Vigilante, others)

Acceptance:

- Canonical class records in `src/rules_core/rules_tables/ultimate_intrigue/`.
- For the four classes shared with SD-30 (Occultist, Spiritualist, Medium, Mesmerist), SD-28 references the SD-30 canonical id; SD-28 does not redefine.

## Epic 7 (SD28-E7) — Ultimate Campaign content-source ingest

**Objective:** Per-system-subsystem cycles (downtime, kingdom-building, traits, retraining).

**Derived from:** `scope-draft.md §"Book list" slot 5`.

### Feature seeds

#### SD28-E7-F1 — Player-options subsystems

Acceptance:

- Trait, downtime, kingdom-building, and retraining rules have representation slices.
- Pre-computed effects (no execution engines); see `decisions.md §18` rules-as-data doctrine.

## Epic 8 (SD28-E8) — Ultimate Wilderness content-source ingest

**Objective:** Per-class + per-Companion-rules cycles.

**Derived from:** `scope-draft.md §"Book list" slot 6`.

### Feature seeds

#### SD28-E8-F1 — Class records

#### SD28-E8-F2 — Companion-rules records

## Epic 9 (SD28-E9) — Ultimate Psionics content-source ingest (Dreamscarred Press tier)

**Objective:** Per-class + per-power cycles, gated on licensing pre-cycle verification per `decisions.md §17`.

**Derived from:** `scope-draft.md §"Book list" slot 7` + `decisions.md §17`.

### Feature seeds

#### SD28-E9-F1 — Pre-cycle licensing verification

Acceptance:

- Trap-report output against `dreamscarred_press/ultimate_psionics/` records license-conformance findings.
- Any record not matching open-content tier is dropped from per-cycle scope (recorded as cycle finding).

#### SD28-E9-F2 — Class + power records

Acceptance:

- One canonical record per class and power in `src/rules_core/rules_tables/ultimate_psionics/`.
- Reach-gate coverage for each class/power.

## Epic 10 (SD28-E10) — Closure Epilogue

**Objective:** Standard part-of-handoff; tranche promotion PR fires after all per-book epics closed.

**Derived from:** `decisions.md §22` (operating form) + the build-version amendment (2026-07-17).

### Feature seeds

#### SD28-E10-F1 — Closure cycle

Acceptance:

- All Epic 3-9 per-book cycles `complete` in `progress.md`.
- `release-notes.md` populated with the seven books' per-record rollup.
- Tranche promotion PR fires: `tranche/8 → develop`; `0.8.<last_build>` remains the post-closure value.

#### SD28-E10-F2 — Workspace-tree removal (move-not-copy)

Acceptance:

- The source-of-record directory (`programs/codex/requirements/SD-28-ultimate-book-content-ingestion/`) is removed on the publish commit per `decisions.md §22`.
- The canonical repo-resident home is `docs/release/SD-28-ultimate-book-content-ingestion/`.

## Epic 11 (SD28-E11) — Build Version Numbering

**Objective:** Establish the first concrete value of the build version scheme.

**Derived from:** `decisions.md §15`.

### Feature seeds

#### SD28-E11-F1 — Version patch

Acceptance:

- First concrete value: `0.8.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Epic 10 increments per the 2026-07-17 build-version amendment.
- Major remains `0` until first main-publish.

## Epic 12 (SD28-E12) — Bundle Code Review

**Objective:** A full code review of the bundle's entire diff against its
branch point, run after every content-ingest epic (3-9) and Epic 11 (Build
Version Numbering) are closed — not in parallel with them, and not scoped to
only the final cycle. `./scripts/verify.sh` passing is a **precondition** to
this epic firing, not the review itself: a green gate says the tests that
exist pass, it says nothing about whether the code is right.

**Derived from:** operator directive 2026-08-01 (the v0.6 CRB run closed
without an end-of-run code review) + `decisions.md §26`.

### Feature seeds

#### SD28-E12-F1 — Whole-bundle diff review

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

#### SD28-E12-F2 — Correctness, no-stub, reach, test-quality, no-hand-authored-frontend-data sweep

Acceptance:

- A sample of this bundle's rules logic is checked against the source corpus
  for the seven books; disagreements are recorded as findings, not assumed
  away.
- No stub, fixture-only, or mock data in a production path per
  `docs/governance/no-stub-mvp-doctrine.md`; any operator-approved exception
  is entered in `docs/governance/wired-integration-stubs-registry.md`, not
  left unregistered.
- A sample of records this bundle claims reach a player surface is spot-checked
  against `reach_gate.rs`'s `OPEN_FINDINGS` mechanism and the live IPC/UI
  path — reach-gate green is necessary, not sufficient, on its own.
- Test quality, not just count: per
  `docs/governance/book-ingestion-playbook.md §7.4`, a sample of this
  bundle's new gates/tests is checked for a case that actually fails when the
  thing it protects is broken, not only a case that passes.
- No hand-authored rules data under `apps/desktop/src/` — rules content is
  sourced from `src/rules_core/rules_tables/`, never hand-typed into a
  frontend file.

#### SD28-E12-F3 — Findings triage

Acceptance:

- Every finding records a severity and a disposition: `fixed-in-bundle` or
  `deferred`. No finding is silently dropped.
- A `deferred` finding names an owner (a person or a specific successor
  bundle) and is entered in `forward-scope-register.md` — not left
  unrecorded.
- Real defects found are fixed in-bundle before Epic 10 (Closure Epilogue)
  fires; the review does not become a rubber stamp that defers everything to
  avoid scope growth.
- A `scripts/retro.py` event is emitted per finding, carrying `--verified-by`.

**Note:** the operator can separately trigger `/code-review ultra` (a
multi-agent cloud review of the branch) at any time. That path is
operator-triggered and billed — a cycle running under §21's unattended-mode
protocol cannot launch it itself — so Epic 12 must stand on its own as the
bundle's actual gate; `/code-review ultra` is a supplement, not a dependency.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → E3, E4, E5, E6, E7, E8, E9 (any order, file-disjoint) → E11 → E12 → E10
```

The per-book epics are **file-disjoint** by source path (each writes to its own
`src/rules_core/rules_tables/<book>/`), so they can run in parallel under
operator-pinned concurrency. The classic repo-level wiring epics (E1, E2,
E11, E12, E10) are sequential. E12 (Bundle Code Review) runs after every
other epic but E10 — any review finding is fixed before the tranche-promotion
PR (part of E10) opens.

## Completion gate

SD-28 closes when:

- All Epic 3-9 per-book cycles `complete` with reach-gate claims and trap-report outputs.
- Epic 12 (Bundle Code Review) closed, all findings triaged with named owners for deferrals.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/8 → develop` is opened and merged.
- `docs/release/SD-28-ultimate-book-content-ingestion/` carries the canonical
  12-file chassis (post-move-not-copy publish).
