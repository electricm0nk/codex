---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01, re-cut 2026-08-10)
date: 2026-08-01
canonical_branch: tranche/9 (operator directive 2026-08-01)
build_version_target: 0.9.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-29 Epic Breakdown

**Re-cut 2026-08-10 (`decisions.md §37`, executing `§36`).** SD-29 is partitioned by *kind*, not by
*book*. The prior 13-epic per-book structure (Epics 3-6, 11-13 as Bestiary 2/3/4/5/6, Bonus
Bestiary, Monster Codex) is retired. This file replaces it with an 11-epic structure: 4 lane epics
(one per kind or merged-kind-pair, per `../corpus-work-channels.md`'s channel analysis) plus the
7 non-content epics unchanged in purpose, renumbered for coherent ordering.

11 epics × ~3-4 acceptance criteria = ~40 criteria. Epic 1 fires FIRST. Epic 11 (Closure Epilogue)
fires LAST. Epics 4-7 (kind lanes) are file-disjoint by kind and may run concurrently post-Epic 3.

## Epic 1 (SD29-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §6` (Identifier discipline).

### Feature seeds

#### SD29-E1-F1 — Identifier-disclosure audit pass

Acceptance:

- No `sd29_*`, `SD29_*`, `Sd29*`, `sd29-*` patterns in this bundle's surface code (`src/rules_core/rules_tables/{beastiary2,beastiary3,beastiary4,beastiary5,beastiary6,bonus_bestiary,monster_codex}/` plus any kind-scoped module a lane epic adds).
- No `t_<hex>` kanban tokens in source files.
- Identifier-discipline audit script returns 0 findings.

#### SD29-E1-F2 — Schema-side grep

Acceptance:

- The four-grep dual-audit (identifier-discipline + wired-integration) runs cleanly post-Epic-1 commit.

## Epic 2 (SD29-E2) — Operator Pre-Launch

**Objective:** Pre-launch checklist verification before any content lane fires. **Corpus-wide, not
per-book** — the pre-flight shapes all seven books at once, since lanes fan out across books within
a kind rather than books fanning out across kinds.

**Derived from:** `loop-instruction.md §"Pre-launch checklist"` + operator directives 2026-08-01,
2026-08-10.

### Feature seeds

#### SD29-E2-F1 — Local-file dispatch readiness

Acceptance:

- `kanban.md` lists at least one ready card (lane epics 4-7).
- `progress.md` exists with first-cycle placeholder.
- Working tree clean (`git status` returns no uncommitted changes).

#### SD29-E2-F2 — Branch-pushed + cycle-0 corpus shape, corpus-wide

Acceptance:

- Branch `tranche/9` is pushed to origin (`git push -u origin tranche/9` succeeds).
- `cargo run --locked --bin v06_work_inventory` regenerated `docs/work-inventory.json`; all seven
  books' entries confirm the per-book, per-kind shape (`decisions.md §37.0`'s table is the
  reference to reproduce and compare against, not transcribe).
- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` ran for all seven books; output
  recorded in `artifacts/<book>-cycle0-trap-report.md`. Run once per book here, at the corpus-wide
  pre-flight — not re-run per lane, since a lane touches every book for its one kind.

## Epic 3 (SD29-E3) — Provenance Gate: PI-Screening for Kind-Lane Ingestion

**Objective:** Resolve the blocking provenance question `../corpus-work-channels.md §6` raised
("Blocking before the first channel runs") before any content lane (Epics 4-7) lands a record.

**Derived from:** `decisions.md §37.3`; `docs/governance/license-matrix.md` (commit `314a7ad9`).

**Status:** New epic, added by the 2026-08-10 re-cut. Gates Epics 4-7.

### What is already resolved, cited not re-done

OGL / attribution / publisher provenance for all seven SD-29 books (Bestiary 2, 3, 4, 5, 6, Bonus
Bestiary, Monster Codex) is established by `docs/governance/license-matrix.md`'s per-book table:
real `OGL.txt`, active `.pcc` `COPYRIGHT:` block, `ISOGL:YES`, Paizo Inc. publisher, section-15
attribution recoverable from `OGL.txt` — all seven, no exceptions. A lane cycle cites the matrix
row for the book it is touching; it does not re-derive OGL status per cycle.

### What this epic actually gates

The license matrix found that `rules_tables/*.rs` (Pipeline B — the exact pipeline every SD-29
kind lane writes into) has **zero PI-screening anywhere in the repo**, and a direct sweep already
found three real, unredacted Product-Identity leaks in other bundles' Pipeline B tables. All seven
of SD-29's books are marked `unscreened` for Pipeline B. `monster_ability` (Epic 4's larger half,
1,346 units of prose-bearing special-attack/special-quality text) is exactly the shape of content
the three known leaks came from.

### Feature seeds

#### SD29-E3-F1 — Per-lane PI-blacklist sweep wired in

Acceptance:

- Each of Epics 4-7's extraction/table-generation step calls `pi_screening::classify_field` (or
  runs the 55-term blacklist sweep as a pre-commit check) against that lane's own newly-generated
  content before it lands in `rules_tables/`.
- The sweep's output (clean, or hits found) is recorded in the lane's first cycle receipt in
  `progress.md`.
- A hit is a hard stop for that record (`loop-instruction.md` "Stop vs. press on" — a gate failing
  for a real content finding is a STOP), not something routed around to keep a cycle green.

#### SD29-E3-F2 — OGL/attribution citation, not re-derivation

Acceptance:

- Every lane cycle's receipt cites `docs/governance/license-matrix.md`'s row for the book it
  touched, rather than re-checking `OGL.txt`/`ISOGL`/`COPYRIGHT` from scratch.
- If a lane cycle finds the matrix's row for a book to be stale (corpus moved since 2026-08-10),
  it records the discrepancy as a finding against the matrix, not a silent local fix.

## Epic 4 (SD29-E4) — Monster / Monster-Ability Chassis Lane

**Objective:** Build the merged monster chassis + monster-ability features system across all seven
books' `monster` (813 units) and `monster_ability` (1,346 units) content — 2,159 units, the largest
lane by a wide margin (56% of SD-29's 3,851 in-scope units).

**Derived from:** `../corpus-work-channels.md §9.2` (monster + monster_ability merge into one
system — chassis and features, the same shape as `race`/`race_trait`); `decisions.md §37.2`
(pilot-then-extend sequencing).

**Why merged, not two lanes.** The operator's ruling (`corpus-work-channels.md §9.2`): monsters are
playable, so `monster` (chassis) and `monster_ability` (features on that chassis) are one coherent
build, not two kinds to ingest independently — the precedent is `race`/`race_trait`, not two
unrelated content types. `Kind::Monster` and `Kind::MonsterAbility` stay separate corpus kinds
(SD-28 `§61`'s deliberate split); the *lane* is merged because the mechanism (a chassis record plus
attached feature records, reach-gated together) is one design.

**Why no path exists within SD-29's own scope.** All 2,159 units are `not-started`
(`decisions.md §37.0`). The corpus-wide "monster path exercised once" evidence (46 of 1,270 units,
`../corpus-work-channels.md §2`) is from Bestiary 1 — SD-22's book, not one of SD-29's seven.
`monster_ability` has no ingest path or engine table anywhere in the corpus (SD-28 `§61` created the
kind; zero units ingested corpus-wide).

### Sequencing — pilot, then extend

**Pilot book: Bonus Bestiary** (14 monster + 17 monster_ability + 3 class = 34 units, the smallest
footprint of any monster-bearing SD-29 book). The pilot cycle-batch builds the chassis-plus-features
mechanism end-to-end against this one book — reach-gated, PI-screened (Epic 3) — before Epic 4's
remaining cycle-batches dispatch. This is the SD-29 instance of the same discipline SD-28 applied to
archetype-class sizing (`§63`): get the real per-book cost from one book before committing to six
more.

**Remaining cycle-batches, in any order after the pilot lands:**

- Bestiary 2 — 316 monster + 466 monster_ability.
- Bestiary 3 — 261 monster + 40 monster_ability.
- Bestiary 4 — 220 monster + 768 monster_ability.
- Bestiary 5 — 0 monster + 39 monster_ability (features with no in-book chassis; still this lane's).
- Bestiary 6 — 0 monster + 13 monster_ability (same shape as Bestiary 5).
- Monster Codex — 2 monster + 3 monster_ability.

**No representative book** — verified per `decisions.md §37.2`: Monster Codex has 2 monsters (of
207 units — its weight is elsewhere, see Epic 7); Bestiary 5 and 6 have 0 monsters; Bestiary 3 is
799 `race_trait` of 1,194 units (Epic 5's, not this lane's); Bestiary 4 is 768 `monster_ability` of
1,218. Each book's cycle-batch is sized from its own re-derived count, never assumed from another
book's shape.

### Feature seeds

#### SD29-E4-F1 — Pilot cycle-batch (Bonus Bestiary)

Acceptance:

- One canonical chassis record per Bonus Bestiary monster (14), with attached monster-ability
  feature records (17) reach-gated together.
- Reach-gate claim executes the real IPC builder for both the chassis and its attached features.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.
- Cycle receipt records the real per-unit cost observed, for use sizing the remaining cycle-batches.

#### SD29-E4-F2 — Remaining cycle-batches

Acceptance:

- One canonical chassis-plus-features build per remaining book (Bestiary 2, 3, 4, 5, 6, Monster
  Codex), sized from `docs/work-inventory.json`'s own per-book counts, not the pilot's count
  assumed uniform.
- Reach-gate coverage for every chassis and every attached feature record.
- Trap-report output recorded per book (`artifacts/<book>-trap-report.md`).

## Epic 5 (SD29-E5) — Race-Trait Lane

**Objective:** Build a real `race_trait` ingest path across all seven books' 1,124 `race_trait`
units, fixing the classifier's name-coincidence grounding defect **alongside** the build, not
before or after.

**Derived from:** `../corpus-work-channels.md §9.3` (companion and race_trait ruled engine content,
same as monster_ability; defect fixed alongside); `../corpus-work-channels.md §3` (the corpus-wide
finding that the real count of legitimately grounded race traits, outside `core_essentials`'
name-coincidence hits, is approximately one).

**The defect, stated so a cycle does not rediscover it as new.** `classify()`'s only source for
grounding a `race_trait` is CRB's own hardcoded table — a non-CRB trait reaches `grounded` today
only by coincidental name match (SD-28 `§56`; UPsi's `Blue ~ Keen Senses` matching Elf's, and three
others). None of SD-29's seven books' 1,124 `race_trait` units are `core_essentials` or CRB, so
none of them can reach `grounded` under the current classifier regardless of how well a cycle
ingests them — building the path without fixing the classifier would ship a lane whose own success
criterion is untrustworthy (`corpus-work-channels.md §9.3`, "'Alongside' is the right call").

### Feature seeds

#### SD29-E5-F1 — Classifier defect fix

Acceptance:

- `classify()`'s race-trait grounding source is widened beyond CRB's hardcoded table to the actual
  per-book source record the trait was ingested from.
- A regression test pins that a non-CRB trait grounds only via its own book's record, never via a
  name match against an unrelated book's trait.

#### SD29-E5-F2 — Per-book race-trait ingest

Acceptance:

- Canonical race-trait records for each book's share of the 1,124 units (Bestiary 2: 162, Bestiary
  3: 799, Bestiary 4: 86, Bestiary 5: 63, Bestiary 6: 0, Bonus Bestiary: 17, Monster Codex: 14 —
  re-derive per `decisions.md §37.0`'s command before sizing a cycle-batch, these are a snapshot).
- Reach-gate coverage for each record, grounded via the fixed classifier (E5-F1), not by
  coincidence.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.

## Epic 6 (SD29-E6) — Companion Lane

**Objective:** Build a `companion` ingest path across all seven books' 275 `companion` units — no
path exists anywhere in the corpus (`../corpus-work-channels.md §3`, 17 books corpus-wide, zero
ingested).

**Derived from:** `../corpus-work-channels.md §9.3` (companion ruled engine content, same as
`monster_ability`, no path).

### Feature seeds

#### SD29-E6-F1 — Companion mechanism

Acceptance:

- A canonical companion record shape lands (chassis or attribute-set, whichever the corpus's
  companion-mod `.lst` shape actually supports — determined by this epic's own trap-report, not
  assumed from `race`/`race_trait`'s shape).
- Reach-gate coverage for a sample companion record proves the mechanism before the full 275-unit
  sweep runs.

#### SD29-E6-F2 — Per-book companion ingest

Acceptance:

- Canonical companion records for each book's share of the 275 units (Bestiary 2: 16, Bestiary 3:
  85, Bestiary 4: 76, Bestiary 5: 57, Bestiary 6: 26, Bonus Bestiary: 0, Monster Codex: 15 —
  re-derive before sizing).
- Reach-gate coverage for every record.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.

## Epic 7 (SD29-E7) — Residual Proven-Path Content Lane

**Objective:** Ingest the Channel A/B kinds SD-29's seven books carry in small volume, using the
already-proven per-book method (SD-28 landed seven books of feats and four of equipment through it)
rather than building new mechanism: `spell` (82), `equipment` (65), `feat` (32), `race` (12),
`equipment_modifier` (9), `class` (3) — **203 units total**.

**Derived from:** `../corpus-work-channels.md §3` (Channel A — proven path, wide adoption) and
Channel B for `spell` specifically (3/26 books corpus-wide, method exercised).

**`class_feature` (90 units) is explicitly excluded from this lane** — see `decisions.md §37.4`.
It is Channel D: blocked behind the archetype mechanism and per-class chassis (SD-28 `§60`/`§63`),
funded corpus-wide but not yet sized for the specific classes these 90 units belong to. Folding it
into a lane scoped to *settled* methods would misrepresent it as ready. It is tracked in
`successor-forward-scope-register.md`, inheriting SD-28 `§9.1`'s per-class measurement once it
reaches these classes — not silently dropped, not silently ingested with the wrong method.

### Feature seeds

#### SD29-E7-F1 — Spell and equipment records

Acceptance:

- Canonical spell records for the 82 `spell` units and equipment/equipment-modifier records for the
  65 `equipment` + 9 `equipment_modifier` units, using the settled per-book table + resolver-chain
  method (`../corpus-work-channels.md §3`).
- Reach-gate coverage for every record.

#### SD29-E7-F2 — Feat, race, and class records

Acceptance:

- Canonical feat records for the 32 `feat` units, race records for the 12 `race` units (the
  chassis records these books' own `race_trait` content — Epic 5 — attaches to), and class records
  for the 3 `class` units.
- Reach-gate coverage for every record.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.

## Epic 8 (SD29-E8) — DM Toolkit extension (consume Epic 4's monster records)

**Objective:** Extend `src/rules_core/encounters.rs` + `src/rules_core/party_cr.rs` to consume
Epic 4's monster chassis records, book by book as Epic 4's cycle-batches land.

**Derived from:** `scope-draft.md §"Out of cycle ingestion and surfacing"` +
`successor-forward-scope-register.md C1.2` + `decisions.md §19` (reach-gate-doD).

**Status:** Operator-pinned whether in scope, per-cycle at Epic 4's pilot-batch closure (was
"Epics 5 and 6 closure" under the old per-book numbering; the gating event is now the same idea
against the lane structure — Epic 4's pilot landing, not all seven books). If Epic 8 lands inside
SD-29, it satisfies the reach gate's `OPEN_FINDINGS` Bestiary-1-monster-surface prerequisite (already
independently satisfied — see `decisions.md §10`'s supersession note; Epic 8 here is additive
consumption of SD-29's own monster records, not a re-opening of that finding). If deferred, cycles
record `decision-blocked` in `progress.md` and move to the next ready card.

### Feature seeds

#### SD29-E8-F1 — Encounter builder extension

Acceptance:

- `encounters.rs` reads SD-29's monster records as Epic 4's cycle-batches land (incrementally, not
  gated on all seven books).

#### SD29-E8-F2 — Party-CR math extension

Acceptance:

- `party_cr.rs` reads SD-29's monster records for CR calculations, same incremental basis.

## Epic 9 (SD29-E9) — Build Version Numbering

**Objective:** First concrete build value per the 2026-07-17 amendment.

**Derived from:** `decisions.md §14`.

### Feature seeds

#### SD29-E9-F1 — Version patch

Acceptance:

- First concrete value: `0.9.<build>` (read from current build counter at cycle close).
- Closing-PR iteration on Epic 11 increments per the 2026-07-17 build-version amendment.
- Major remains `0` until first main-publish.

## Epic 10 (SD29-E10) — Bundle Code Review

**Objective:** A full code review of the bundle's entire diff against its branch point, run after
Epics 4-7 (content lanes), Epic 8 (DM Toolkit extension, if in scope), and Epic 9 (Build Version
Numbering) are closed — not in parallel with them, and not scoped to only the final cycle.
`./scripts/verify.sh` passing is a **precondition** to this epic firing, not the review itself: a
green gate says the tests that exist pass, it says nothing about whether the code is right.

**Derived from:** operator directive 2026-08-01 (the v0.6 CRB run closed without an end-of-run code
review) + `decisions.md §27`.

### Feature seeds

#### SD29-E10-F1 — Whole-bundle diff review

Acceptance:

- The reviewed diff scope is the bundle's full change set against its branch point (`git diff
  origin/develop...HEAD`, the same merge-base triple-dot comparison `scripts/identifier-
  discipline-audit.sh` and `scripts/wired-integration-audit.sh` already default to via
  `BASE_BRANCH`), not the closing cycle's slice alone.
- `./scripts/verify.sh` has a recorded green run for that diff, cited as a precondition in the
  epic's receipt.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` (this bundle's
  standing per-cycle dual-audit) are re-run once more at bundle scope.

#### SD29-E10-F2 — Correctness, no-stub, reach, test-quality, no-hand-authored-frontend-data sweep

Acceptance:

- A sample of this bundle's rules logic is checked against the source corpus across the seven
  books and four lanes (monster/monster_ability chassis, race-trait, companion, residual);
  disagreements are recorded as findings, not assumed away.
- No stub, fixture-only, or mock data in a production path per
  `docs/governance/no-stub-mvp-doctrine.md`; any operator-approved exception is entered in
  `docs/governance/wired-integration-stubs-registry.md`, not left unregistered.
- A sample of records this bundle claims reach a player surface is spot-checked against
  `reach_gate.rs`'s `OPEN_FINDINGS` mechanism and the live IPC/UI path — including the Epic 8 DM
  Toolkit consumer surface, if in scope. Reach-gate green is necessary, not sufficient, on its own.
- Every content lane's PI-screening sweep (Epic 3) is spot-checked, not just cited as run.
- Test quality, not just count: per `docs/governance/book-ingestion-playbook.md §7.4`, a sample of
  this bundle's new gates/tests is checked for a case that actually fails when the thing it
  protects is broken, not only a case that passes.
- No hand-authored rules data under `apps/desktop/src/` — rules content is sourced from
  `src/rules_core/rules_tables/`, never hand-typed into a frontend file.

#### SD29-E10-F3 — Findings triage

Acceptance:

- Every finding records a severity and a disposition: `fixed-in-bundle` or `deferred`. No finding
  is silently dropped.
- A `deferred` finding names an owner (a person or a specific successor bundle) and is entered in
  `successor-forward-scope-register.md` — not left unrecorded.
- Real defects found are fixed in-bundle before Epic 11 (Closure Epilogue) fires; the review does
  not become a rubber stamp that defers everything to avoid scope growth.
- A `scripts/retro.py` event is emitted per finding, carrying `--verified-by`.

**Note:** the operator can separately trigger `/code-review ultra` (a multi-agent cloud review of
the branch) at any time. That path is operator-triggered and billed — a cycle running under §22's
unattended-mode protocol cannot launch it itself — so Epic 10 must stand on its own as the bundle's
actual gate; `/code-review ultra` is a supplement, not a dependency.

## Epic 11 (SD29-E11) — Closure Epilogue

**Objective:** Tranche promotion PR fires after every lane epic (4-7) + Epic 8 (if in scope) +
Epic 9 + Epic 10 have closed.

**Derived from:** `decisions.md §23` (operating form) + the build-version amendment (2026-07-17).

### Feature seeds

#### SD29-E11-F1 — Closure cycle

Acceptance:

- All Epic 4-7 lane cycle-batches `complete` in `progress.md` (each book's share of each kind).
- Epic 8 (if in scope) `complete`.
- `release-notes.md` populated with a per-lane rollup (not a per-book rollup — the lane is the unit
  of work under this structure).
- Tranche promotion PR fires: `tranche/9 → develop`; `0.9.<last_build>` remains the post-closure
  value.

#### SD29-E11-F2 — Workspace-tree removal (move-not-copy)

Acceptance:

- The source-of-record directory removed on the publish commit per `decisions.md §14`.
- The canonical repo-resident home is `docs/release/SD-29-bestiary-line-book-ingestion/`.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → E3 → {E4, E5, E6, E7} (any order, file-disjoint by kind) → E8 (gated on E4's pilot) → E9 → E10 → E11
```

Lane epics are file-disjoint by kind (each writes to a kind-scoped module or, within Epic 4-7's
per-book cycle-batches, a book-scoped subdirectory under `src/rules_core/rules_tables/<book>/`),
so they can run in parallel under operator-pinned concurrency, gated behind Epic 3's provenance
sweep landing on the extraction path each lane uses. Epic 8 (DM Toolkit extension) is gated on
Epic 4's pilot cycle-batch, not on every lane closing. Epic 10 (Bundle Code Review) runs after
every other epic but Epic 11 — any review finding is fixed before the tranche-promotion PR (part
of Epic 11) opens.

## Completion gate

SD-29 closes when:

- All Epic 4-7 lane cycle-batches `complete` with reach-gate claims, PI-screening sweeps, and
  trap-report outputs, for every book that carries units of that lane's kind.
- Epic 8 (DM Toolkit extension) lands (in scope) or surfaces as a Class 1/3 retrofit.
- Epic 10 (Bundle Code Review) closed, all findings triaged with named owners for deferrals.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/9 → develop` is opened and merged.
- `docs/release/SD-29-bestiary-line-book-ingestion/` carries the canonical 14-file chassis
  (post-move-not-copy publish).
