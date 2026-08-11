---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01, re-cut 2026-08-10, corpus-wide re-scope 2026-08-10)
date: 2026-08-01
canonical_branch: tranche/9 (operator directive 2026-08-01)
build_version_target: 0.9.<build>
companion_to: ./scope-draft.md, ./decisions.md
---

# SD-29 Epic Breakdown

**Re-scoped corpus-wide 2026-08-10 (`decisions.md §38`).** SD-29 is no longer the bestiary line —
it is the corpus-wide catch-up bundle, dispatching lanes across every one of the corpus's 37
in-scope books (`../corpus-work-channels.md §10.2`; `beginner_box` excluded, 19 units), touched and
untouched alike. This supersedes the seven-book pin `§34`/`§37.5` carried forward. Kind-lane
partitioning itself (`decisions.md §36`/`§37`) is unchanged by this re-scope — only the book-list
boundary is retired.

**Re-cut 2026-08-10 (`decisions.md §37`, executing `§36`).** SD-29 is partitioned by *kind*, not by
*book*. The prior 13-epic per-book structure (Epics 3-6, 11-13 as Bestiary 2/3/4/5/6, Bonus
Bestiary, Monster Codex) is retired. This file replaces it with an 11-epic structure: 4 lane epics
(one per kind, merged-kind-pair, or kind-group, per `../corpus-work-channels.md`'s channel analysis)
plus the 7 non-content epics unchanged in purpose, renumbered for coherent ordering.

11 epics × ~3-4 acceptance criteria = ~40 criteria. Epic 1 fires FIRST. Epic 11 (Closure Epilogue)
fires LAST. Epics 4-7 (kind lanes) are file-disjoint by kind and may run concurrently post-Epic 3,
each across every book in the 37-book product that carries units of that lane's kind.

## Epic 1 (SD29-E1) — Code-Side Identifier Cleanup

**Objective:** Establish identifier discipline across all code this bundle introduces.

**Derived from:** `decisions.md §6` (Identifier discipline).

### Feature seeds

#### SD29-E1-F1 — Identifier-disclosure audit pass

Acceptance:

- No `sd29_*`, `SD29_*`, `Sd29*`, `sd29-*` patterns in this bundle's surface code — audited across every `src/rules_core/rules_tables/<book>/` directory a lane writes (all 37 in-scope books per `decisions.md §38`, superseding the retired seven-book dir enumeration) plus any kind-scoped module a lane epic adds.
- No `t_<hex>` kanban tokens in source files.
- Identifier-discipline audit script returns 0 findings.

#### SD29-E1-F2 — Schema-side grep

Acceptance:

- The four-grep dual-audit (identifier-discipline + wired-integration) runs cleanly post-Epic-1 commit.

## Epic 2 (SD29-E2) — Operator Pre-Launch

**Objective:** Pre-launch checklist verification before any content lane fires. **Corpus-wide, not
per-book** — the pre-flight shapes all 37 in-scope books at once (`decisions.md §38`;
`../corpus-work-channels.md §10.2`), since lanes fan out across books within a kind rather than
books fanning out across kinds.

**Derived from:** `loop-instruction.md §"Pre-launch checklist"` + operator directives 2026-08-01,
2026-08-10 (×2 — the kind-lane re-cut and the corpus-wide re-scope).

### Feature seeds

#### SD29-E2-F1 — Local-file dispatch readiness

Acceptance:

- `kanban.md` lists at least one ready card (lane epics 4-7).
- `progress.md` exists with first-cycle placeholder.
- Working tree clean (`git status` returns no uncommitted changes).

#### SD29-E2-F2 — Branch-pushed + cycle-0 corpus shape, corpus-wide

Acceptance:

- Branch `tranche/9` is pushed to origin (`git push -u origin tranche/9` succeeds).
- `cargo run --locked --bin v06_work_inventory` regenerated `docs/work-inventory.json`; all 37
  in-scope books' entries confirm the per-book, per-kind shape (`decisions.md §38.0`-`§38.1`'s
  tables are the reference to reproduce and compare against, not transcribe).
- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` ran for every book a lane will
  touch first (its pilot book, per `decisions.md §38.3`, plus any Tier-1 book a cycle starts
  against); output recorded in `artifacts/<book>-cycle0-trap-report.md`. Corpus-wide up front for
  all 37 books is the ideal but not gating — a lane may run its own book's trap report at
  cycle-batch time if the corpus-wide pass has not reached that book yet, per `loop-instruction.md`.

## Epic 3 (SD29-E3) — Provenance Gate: PI-Screening for Kind-Lane Ingestion

**Objective:** Resolve the blocking provenance question `../corpus-work-channels.md §6` raised
("Blocking before the first channel runs") before any content lane (Epics 4-7) lands a record, for
every book in the corpus-wide 37-book product, not just the retired seven-book set.

**Derived from:** `decisions.md §37.3`, widened corpus-wide by `§38.6`;
`docs/governance/license-matrix.md` (commit `314a7ad9`, all 37 books).

**Status:** Gates Epics 4-7, corpus-wide.

### What is already resolved, cited not re-done

OGL / attribution / publisher provenance for all 37 in-scope books is established by
`docs/governance/license-matrix.md`'s per-book table: real `OGL.txt`, active `.pcc` `COPYRIGHT:`
block, `ISOGL:YES`, publisher, section-15 attribution recoverable from `OGL.txt`. A lane cycle cites
the matrix row for the book it is touching; it does not re-derive OGL status per cycle. (`beginner_box`,
the one excluded book, needs no citation — it is out of scope, `decisions.md §38`.)

### What this epic actually gates

The license matrix found that `rules_tables/*.rs` (Pipeline B — the exact pipeline every SD-29
kind lane writes into) has **zero PI-screening anywhere in the repo**, and a direct sweep already
found three real, unredacted Product-Identity leaks in other bundles' Pipeline B tables. `monster_ability`
(Epic 5's larger half, 3,107 units corpus-wide of prose-bearing special-attack/special-quality text)
is exactly the shape of content the three known leaks came from — and it is now a corpus-wide lane,
not a seven-book one.

### Feature seeds

#### SD29-E3-F1 — Per-lane PI-blacklist sweep wired in

Acceptance:

- Each of Epics 4-7's extraction/table-generation step calls `pi_screening::classify_field` (or
  runs the 55-term blacklist sweep as a pre-commit check) against that lane's own newly-generated
  content before it lands in `rules_tables/`, for whichever book the cycle-batch is touching.
- The sweep's output (clean, or hits found) is recorded in the lane's first cycle receipt per book,
  in `progress.md`.
- A hit is a hard stop for that record (`loop-instruction.md` "Stop vs. press on" — a gate failing
  for a real content finding is a STOP), not something routed around to keep a cycle green.

#### SD29-E3-F2 — OGL/attribution citation, not re-derivation

Acceptance:

- Every lane cycle's receipt cites `docs/governance/license-matrix.md`'s row for the book it
  touched, rather than re-checking `OGL.txt`/`ISOGL`/`COPYRIGHT` from scratch.
- If a lane cycle finds the matrix's row for a book to be stale, it records the discrepancy as a
  finding against the matrix, not a silent local fix.

## Epic 4 (SD29-E4) — Proven-Path Content Lanes (Day-One, Corpus-Wide)

**Objective:** Ingest the Channel A/B kinds with a settled method (SD-28 landed seven books of
feats and four of equipment through it) across **every book in the 37-book product that carries
remaining units** — touched or untouched, no book-boundary decision required. No new mechanism
needed; this tier can start the day Epic 3 clears.

**Derived from:** `decisions.md §38.1`/`§38.3` (re-derived corpus-wide figures);
`../corpus-work-channels.md §3` (Channel A — proven path, wide adoption).

| kind | held | remaining |
|---|---:|---:|
| equipment | 5,064 | 1,163 |
| feat | 1,260 | 1,350 |
| spell | 1,089 | 1,754 |
| equipment_modifier | 768 | 812 |
| race | 7 | 96 |
| class | 27 | 158 |

`race` and `class` are added to this lane beyond the four kinds the re-scoping brief named — they
are the small, settled-method chassis kinds the predecessor per-book Epic 7 already carried (race is
the chassis `race_trait` attaches to; class is the chassis kind, distinct from the blocked
`class_feature`). Both are small (96 and 158 remaining respectively) and use the same proven
per-book table method as the four dominant kinds above.

**`class_feature` is explicitly excluded from this lane, and from every lane** — see §38.4/Epic
scope note below and `decisions.md §38.4`. It is not a small residual kind; it is 40.2% of the
corpus, blocked behind the archetype mechanism (SD-28 `§60`/`§63`), and putting it in a lane scoped
to *settled* methods would misrepresent 15,472 units as ready-to-ingest.

### Feature seeds

#### SD29-E4-F1 — Equipment and equipment-modifier records, corpus-wide

Acceptance:

- Canonical equipment/equipment-modifier records land per book, using the settled per-book table +
  resolver-chain method, for every book with remaining units re-derived from
  `docs/work-inventory.json` at cycle-batch time (not transcribed from `decisions.md §38.1`'s
  snapshot).
- Reach-gate coverage for every record. PI sweep (Epic 3) clean or hits resolved before commit.

#### SD29-E4-F2 — Spell records, corpus-wide

Acceptance:

- Canonical spell records land per book, same method, for every book with remaining `spell` units.
- Reach-gate coverage for every record. PI sweep (Epic 3) clean or hits resolved before commit.

#### SD29-E4-F3 — Feat, race, and class records, corpus-wide

Acceptance:

- Canonical feat, race, and class records land per book, same method, for every book with
  remaining units of these three kinds.
- Reach-gate coverage for every record. PI sweep (Epic 3) clean or hits resolved before commit.

## Epic 5 (SD29-E5) — Monster / Monster-Ability Chassis Lane

**Objective:** Build the merged monster chassis + monster-ability features system, corpus-wide —
`monster` (46 held / 1,224 remaining, 14 books) and `monster_ability` (0 held / 3,107 remaining, 24
books) — the largest mechanism-build lane by a wide margin.

**Derived from:** `../corpus-work-channels.md §9.2` (monster + monster_ability merge into one
system — chassis and features, the same shape as `race`/`race_trait`); `decisions.md §37.2`/`§38.3`
(pilot-then-extend sequencing, now corpus-wide).

**Why merged, not two lanes.** The operator's ruling (`corpus-work-channels.md §9.2`): monsters are
playable, so `monster` (chassis) and `monster_ability` (features on that chassis) are one coherent
build, not two kinds to ingest independently. `Kind::Monster` and `Kind::MonsterAbility` stay
separate corpus kinds (SD-28 `§61`'s deliberate split); the *lane* is merged because the mechanism
(a chassis record plus attached feature records, reach-gated together) is one design.

**Why the mechanism does not exist yet.** The corpus-wide "monster path exercised once" evidence (46
of 1,270 units) is Bestiary 1's own `grounded` count — real, but a single book's worth, and
`monster_ability` has zero ingested units and no engine table anywhere in the corpus (SD-28 `§61`
created the kind).

### Sequencing — pilot, then extend, corpus-wide

**Pilot book: Bonus Bestiary** (14 monster + 17 monster_ability = 31 remaining units), carried
forward from `§37.2`'s reasoning — the smallest *non-degenerate* monster+monster_ability
combination. Two smaller pairs exist corpus-wide (`occult_adventures`: 1+3; `monster_codex`: 2+3)
but both are too thin to prove a chassis-plus-features mechanism; `book_of_the_damned_volume_2`
(4+17=21) is a viable smaller alternative — recorded per `decisions.md §38.3`, not substituted. The
pilot cycle-batch builds the chassis-plus-features mechanism end-to-end against Bonus Bestiary —
reach-gated, PI-screened (Epic 3) — before this lane's remaining cycle-batches dispatch against any
other book.

**Remaining cycle-batches, corpus-wide, in any order after the pilot lands:** every book with
remaining `monster` or `monster_ability` units, re-derived from `docs/work-inventory.json` at
dispatch time — 14 books for `monster`, 24 for `monster_ability`, overlapping but not identical
sets. `decisions.md §38.2` confirms Bestiary 1 (284 monster + 523 monster_ability remaining) is one
of these books, in scope on the same footing as every other — no separate epic or receipt track.

**No representative book** — verified corpus-wide: Monster Codex has 2 monsters remaining (its
weight is elsewhere, Epic 4); Bestiary 5/6 have 0 monsters remaining; several books (e.g.
`occult_adventures`) carry `monster_ability` with a negligible or zero `monster` count in the same
book. Each book's cycle-batch is sized from its own re-derived count, never assumed uniform.

### Feature seeds

#### SD29-E5-F1 — Pilot cycle-batch (Bonus Bestiary)

Acceptance:

- One canonical chassis record per Bonus Bestiary monster (14 remaining), with attached
  monster-ability feature records (17 remaining) reach-gated together.
- Reach-gate claim executes the real IPC builder for both the chassis and its attached features.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.
- Cycle receipt records the real per-unit cost observed, for use sizing the remaining cycle-batches.

#### SD29-E5-F2 — Corpus-wide extension

Acceptance:

- One canonical chassis-plus-features build per remaining book with `monster` and/or
  `monster_ability` units (re-derived from `docs/work-inventory.json`, not the pilot's count
  assumed uniform), touched or untouched by any prior bundle.
- Reach-gate coverage for every chassis and every attached feature record.
- Trap-report output recorded per book (`artifacts/<book>-trap-report.md`).

## Epic 6 (SD29-E6) — Race-Trait Lane

**Objective:** Build a real `race_trait` ingest path corpus-wide — 44 held / 3,412 remaining across
27 books — fixing the classifier's name-coincidence grounding defect **alongside** the build, not
before or after.

**Derived from:** `../corpus-work-channels.md §9.3` (companion and race_trait ruled engine content,
same as monster_ability; defect fixed alongside); `decisions.md §38.1`/`§38.3` (corpus-wide figures).

**The defect, stated so a cycle does not rediscover it as new.** `classify()`'s only source for
grounding a `race_trait` is CRB's own hardcoded table — a non-CRB trait reaches `grounded` today
only by coincidental name match (SD-28 `§56`; UPsi's `Blue ~ Keen Senses` matching Elf's, and three
others). Of the corpus's 44 `grounded` race traits, most sit in `core_essentials` by the same
coincidence, not real per-book support — building the path without fixing the classifier would ship
a lane whose own success criterion is untrustworthy (`corpus-work-channels.md §9.3`, "'Alongside' is
the right call").

### Feature seeds

#### SD29-E6-F1 — Classifier defect fix

Acceptance:

- `classify()`'s race-trait grounding source is widened beyond CRB's hardcoded table to the actual
  per-book source record the trait was ingested from.
- A regression test pins that a non-CRB trait grounds only via its own book's record, never via a
  name match against an unrelated book's trait.

#### SD29-E6-F2 — Pilot cycle-batch (`inner_sea_intrigue`)

Acceptance:

- Canonical race-trait records for `inner_sea_intrigue`'s 9 remaining units — the smallest
  non-degenerate race-trait book corpus-wide (`book_of_the_damned_volume_1`/`_2` at 1 unit each are
  too thin to prove the fixed path) — reach-gated via the fixed classifier (E6-F1), not coincidence.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.

#### SD29-E6-F3 — Corpus-wide extension

Acceptance:

- Canonical race-trait records for every remaining book (re-derive per `decisions.md §38.1`'s
  command before sizing each cycle-batch — this is a live count, not a fixed list).
- Reach-gate coverage for each record, grounded via the fixed classifier, not by coincidence.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.

## Epic 7 (SD29-E7) — Companion Lane

**Objective:** Build a `companion` ingest path corpus-wide — 0 held / 1,683 remaining across 17
books — no path exists anywhere in the corpus.

**Derived from:** `../corpus-work-channels.md §9.3` (companion ruled engine content, same as
`monster_ability`, no path); `decisions.md §38.1`/`§38.3` (corpus-wide figures).

### Feature seeds

#### SD29-E7-F1 — Companion mechanism + pilot cycle-batch (`inner_sea_combat`)

Acceptance:

- A canonical companion record shape lands (chassis or attribute-set, whichever the corpus's
  companion-mod `.lst` shape actually supports — determined by this epic's own trap-report, not
  assumed from `race`/`race_trait`'s shape).
- Reach-gate coverage for `inner_sea_combat`'s 10 remaining companion units — the smallest
  non-degenerate companion book corpus-wide (`horror_adventures`/`inner_sea_intrigue` at 2 units
  each are too thin) — proves the mechanism before the full corpus-wide sweep runs.

#### SD29-E7-F2 — Corpus-wide extension

Acceptance:

- Canonical companion records for every remaining book (re-derive per `decisions.md §38.1`'s
  command before sizing each cycle-batch).
- Reach-gate coverage for every record.
- Per-lane PI sweep (Epic 3) clean or hits resolved before commit.

## Epic 8 (SD29-E8) — DM Toolkit extension (consume Epic 5's monster records)

**Objective:** Extend `src/rules_core/encounters.rs` + `src/rules_core/party_cr.rs` to consume
Epic 5's monster chassis records, book by book as Epic 5's cycle-batches land.

**Derived from:** `scope-draft.md §"Out of cycle ingestion and surfacing"` +
`successor-forward-scope-register.md C1.2` + `decisions.md §19` (reach-gate-doD).

> **RULED 2026-08-11 — DEFERRED. Epic 8 does not land inside SD-29.** Card `epic-8-toolkit` is
> `decision-blocked` (this bundle's one sanctioned instance, `loop-instruction.md` UNATTENDED MODE
> item 4); the extension surfaces as the Class 3 retrofit **C3.1**, now ACTIVE in
> `successor-forward-scope-register.md`, which carries the evidence. Reason: the in-scope condition
> below is unmet — both reach claims Epic 5's pilot landed assess the already-shipped
> `list_monster_catalog`, zero assess a toolkit surface. Cost of deferring is nil for the gate, per
> this section's own parenthetical: the `OPEN_FINDINGS` Bestiary-1-monster-surface prerequisite was
> **already independently satisfied**, so nothing in `reach_gate.rs` was waiting on Epic 8. Both
> feature seeds below (SD29-E8-F1, SD29-E8-F2) move to the retrofit unbuilt. Full receipt:
> `progress.md` cycle `SD29-E8-F1-001`.

**Status:** Operator-pinned whether in scope, per-cycle at Epic 5's pilot-batch closure (gating
event is Epic 5's pilot landing — Bonus Bestiary — not any full book set). If Epic 8 lands inside
SD-29, it satisfies the reach gate's `OPEN_FINDINGS` Bestiary-1-monster-surface prerequisite (already
independently satisfied — see `decisions.md §10`'s supersession note; Epic 8 here is additive
consumption of SD-29's own monster records, not a re-opening of that finding). If deferred, cycles
record `decision-blocked` in `progress.md` and move to the next ready card.

### Feature seeds

#### SD29-E8-F1 — Encounter builder extension

Acceptance:

- `encounters.rs` reads SD-29's monster records as Epic 5's cycle-batches land (incrementally, not
  gated on every book).

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

- A sample of this bundle's rules logic is checked against the source corpus across the 37 in-scope
  books and four lanes (proven-path, monster/monster_ability chassis, race-trait, companion);
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

- All Epic 4-7 lane cycle-batches `complete` in `progress.md` (each book's share of each kind, for
  every one of the 37 in-scope books that carries units of that kind).
- Epic 8 (if in scope) `complete`.
- `release-notes.md` populated with a per-lane rollup (not a per-book rollup — the lane is the unit
  of work under this structure).
- Tranche promotion PR fires: `tranche/9 → develop`; `0.9.<last_build>` remains the post-closure
  value.

#### SD29-E11-F2 — Workspace-tree removal (move-not-copy)

Acceptance:

- The source-of-record directory removed on the publish commit per `decisions.md §14`.
- The canonical repo-resident home is `docs/release/SD-29-corpus-wide-catch-up-lanes/`.

## Recommended sequencing (dependency order, not exclusive scope)

```
E1 → E2 → E3 → {E4, E5, E6, E7} (any order, file-disjoint by kind) → E8 (gated on E5's pilot) → E9 → E10 → E11
```

Lane epics are file-disjoint by kind (each writes to a kind-scoped module or, within Epic 4-7's
per-book cycle-batches, a book-scoped subdirectory under `src/rules_core/rules_tables/<book>/`),
so they can run in parallel under operator-pinned concurrency, gated behind Epic 3's provenance
sweep landing on the extraction path each lane uses. **Epic 4 (proven-path lanes) needs no
mechanism and can start the day Epic 3 clears — day-one parallel.** Epics 5-7 each pilot on one
small book before extending corpus-wide (mechanism-gated). Epic 8 (DM Toolkit extension) is gated
on Epic 5's pilot cycle-batch, not on every lane closing. Epic 10 (Bundle Code Review) runs after
every other epic but Epic 11 — any review finding is fixed before the tranche-promotion PR (part
of Epic 11) opens.

## Completion gate

SD-29 closes when:

- All Epic 4-7 lane cycle-batches `complete` with reach-gate claims, PI-screening sweeps, and
  trap-report outputs, for every one of the 37 in-scope books that carries units of that lane's
  kind — touched or untouched by any prior bundle.
- Epic 8 (DM Toolkit extension) lands (in scope) or surfaces as a Class 1/3 retrofit.
- Epic 10 (Bundle Code Review) closed, all findings triaged with named owners for deferrals.
- `progress.md` carries the closure receipt.
- `release-notes.md` is populated.
- The tranche-promotion PR `tranche/9 → develop` is opened and merged.
- `docs/release/SD-29-corpus-wide-catch-up-lanes/` carries the canonical 14-file chassis
  (post-move-not-copy publish).
