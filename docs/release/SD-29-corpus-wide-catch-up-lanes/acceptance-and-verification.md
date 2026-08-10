---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/9 (operator directive 2026-08-01)
build_version_target: 0.9.<build>
---

# SD-29 Acceptance Tests

Tests are Given/When/Then format, paired with the technical requirements
in `technical-requirements.md` and the epics in `epic-breakdown.md`.

**Re-cut 2026-08-10 (`decisions.md §37`).** SD-29 dispatches by kind lane
(Epics 4-7), not per-book epic. `<bestiary>` below ranges over all seven
in-scope books (`bestiary_2`-`bestiary_6`, `bonus_bestiary`, `monster_codex`)
rather than the retired four-book `{2, 3, 4, 5}` set. AT-29-003a is new,
gating provenance.

**RE-SCOPED CORPUS-WIDE, 2026-08-10 (`decisions.md §38`).** `<bestiary>`/`<book>` below now ranges
over all 37 in-scope books (`../corpus-work-channels.md §10.2`), not the seven named above — the
seven-book set is preserved as the historical origin of these criteria's shape, not their current
boundary. Epic numbering also shifted: Epic 4 is now the corpus-wide proven-path lane (was the
Monster/Monster-Ability lane); Monster/Monster-Ability is now Epic 5, Race-Trait Epic 6, Companion
Epic 7. Read every `Epic 4`/`Epic 5`/`Epic 6`/`Epic 7` reference below against `epic-breakdown.md`'s
current numbering, not this file's pre-§38 authoring.

## AT-29-001 — Per-cycle file-touch partition

Given a lane cycle-batch for `<book>`, where `<book>` ranges over all 37 in-scope books
(`../corpus-work-channels.md §10.2`; generalized 2026-08-10 from the retired
`{2, 3, 4, 5, 6, bonus, monster_codex}` enumeration per `decisions.md §38`).

When the cycle writes files.

Then:

- Files written under `src/rules_core/rules_tables/<book>/`, `data/corpus/<book>/`, new bins under `src/bin/`, new tests under `tests/` (named per identifier-discipline doctrine; no `sd29_` prefix), or `docs/release/SD-29-.../`.
- No file written under `src/rules_core/pilot_compute.rs`, another book's `rules_tables/<other_book>/` tree, `docs/release/v0.6/`, `src/oracle_validation/`, or `src/pcgen_import/corpus_traps.rs`.

Evidence: per-cycle receipt carries the audit command and the captured exit code.

## AT-29-002 — Reach-gate claim

Given a lane record at `src/rules_core/rules_tables/<book>/<record>.rs`, for any of Epic 4's
(proven-path: spell/equipment/feat/race/equipment_modifier/class), Epic 5's (monster chassis +
monster-ability features), Epic 6's (race-trait), or Epic 7's (companion) records. *(Epic
mapping corrected 2026-08-10 to the `decisions.md §38` numbering.)*

When the cycle's reach gate runs.

Then:

- The gate's IPC builder executes the record's slice.
- The gate's exit code is `0`.
- The cycle receipt captures the gate's per-record output.

## AT-29-003 — Pre-cycle trap-report

Given a lane cycle-batch (Epic 4, 5, 6, or 7) for a specific book.

When the cycle starts.

Then:

- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` has been run.
- The output is recorded in `artifacts/<book>-trap-report.md`.
- The cycle receipt cites the trap-report file.

## AT-29-003a — Provenance gate: PI-screening wired into the lane's extraction step (new, Epic 3)

Given a lane's (Epic 4, 5, 6, or 7) first content commit for any book.

When the lane's extraction/table-generation step runs.

Then:

- `pi_screening::classify_field` (or the 55-term blacklist sweep it implements) has run against the
  lane's own newly-generated content before it lands in `rules_tables/`.
- The sweep's output (clean, or hits found and their disposition) is recorded in the lane's first
  cycle receipt for that book.
- A hit is a hard stop for that record — not routed around, not silently redacted without a
  retro-logged `correction` event.
- The cycle receipt cites `docs/governance/license-matrix.md`'s row for the book's OGL/attribution
  status rather than re-deriving it.

Evidence: `docs/governance/license-matrix.md` (commit `314a7ad9`); `decisions.md §37.3`.

## AT-29-004 — Definition-of-done audit

Given a cycle's PR.

When the dual-audit runs.

Then:

- The identifier-discipline audit (`scripts/identifier-discipline-audit.sh` or equivalent) exits 0.
- The wired-integration 4-grep audit exits 0.

## AT-29-005 — Build version

Given the bundle's first concrete build.

When the closure Epic 11 fires.

Then:

- `0.9.<build>` is the post-closure value, where `<build>` is the recorded build counter at cycle close.
- The next bundle (post-tranche-promotion) reads `0.9.<last_build>` as its starting point.

## AT-29-006 — Identifier discipline

Given any file written by a cycle.

When the cycle commits.

Then:

- No `sd29_*`, `SD29_*`, `Sd29*`, `sd29-*` patterns in the file.
- No `t_<hex>` kanban tokens.
- The identifier-discipline audit exits 0.

## AT-29-007 — Cross-book conflict rule

Given two monster records — one in SD-29's bestiary and one in a closed/adjacent SD-N's book — that conflict on a record id.

When the cycle determines which is doctrine.

Then:

- The newer book is doctrine.
- The older book carries an erratum note referencing the newer book's record id.

Exception: class-grant overlap (per `decisions.md §5`) follows the
class-grant rule; SD-30 owns canonical class definitions.

## AT-29-008 — Zero-monster books route to the correct lane, not a skipped cycle

**Retired shape (pre-2026-08-10):** "Epic 6 (Bestiary 5) cycle picks per-race/per-feat/
per-companion-mod cycles if `monster` units = 0." Superseded — Bestiary 5 and Bestiary 6 are no
longer their own epics; their units are distributed across Epic 5 (monster_ability only — 39 and 13
units respectively, zero monster chassis), Epic 6 (race_trait — 63 and 0), Epic 7 (companion — 57
and 26), and Epic 4 (proven-path kinds) has none from either book. *(Epic mapping corrected
2026-08-10 to the `decisions.md §38` numbering.)*

Given Epic 2's corpus-wide pre-flight (cycle-0 trap-report + work-inventory output, all 37
in-scope books per `decisions.md §38`).

When a lane epic (4, 5, 6, or 7) reads a book's `kinds` field for its own kind.

Then:

- A book carrying zero units of a lane's kind (e.g., Bestiary 5/6 carry zero `monster`) is simply
  absent from that lane's per-book cycle-batch list — not a skipped cycle, not a `decision-blocked`
  entry, because the lane structure means "zero of this kind in this book" is an ordinary shape
  fact, not an exception requiring a fallback cycle type.
- The cycle receipt records the per-book, per-kind counts it read, citing the re-derivation command
  (`decisions.md §37.0`), not a transcribed figure.

## AT-29-009 — Per-entity counts generated

Given a cycle's progress receipt.

When the cycle publishes a figure.

Then:

- The figure cites the `cargo run --locked --bin v06_work_inventory` output that produced it.
- No hand-maintained per-entity counts in the figure.

## AT-29-010 — Rules-as-data, no real-time engines

Given a lane cycle (Epic 4, 5, 6, or 7) for a specific book.

When the cycle writes a numerical effect.

Then:

- The effect is posted as a precomputed value.
- The runtime does not call a die-rolling function for the effect.
- Real-time engines are absent from the cycle's source.

## AT-29-011 — Move-not-copy publish

Given Epic 11 (Closure Epilogue).

When the publish commit fires.

Then:

- The source-of-record directory (`programs/codex/requirements/SD-29-.../`) is removed.
- The canonical repo-resident home (`docs/release/SD-29-corpus-wide-catch-up-lanes/`) carries the 14-file chassis.

## AT-29-012 — Local-file work-queue dispatch

Given the cycle supervisor.

When the supervisor reads `kanban.md` at top of each cycle.

Then:

- The supervisor finds at least one ready card.
- The supervisor claims one card by editing `kanban.md` (claimed-by, claimed-at, cycle-id).
- The supervisor writes the cycle receipt to `progress.md`.
- The supervisor closes the card on cycle completion.

No Hermes-board interaction is required.

## AT-29-013 — Bundle code review (final epic)

Given Epic 10 (Bundle Code Review), firing after all content lanes (Epics 4-7, plus Epic 8 if in scope) and Epic 9 (Build Version Numbering) are closed, before Epic 11 (Closure Epilogue).

When the review runs.

Then:

- `./scripts/verify.sh` has a recorded green run — a precondition to the review, not the review itself.
- The diff scope reviewed is the whole bundle against its branch point (`git diff origin/develop...HEAD`), not the closing cycle alone.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` are re-run at bundle scope.
- The review covers, at minimum: rules-logic correctness sampled against the corpus; no stubs/fixture-only data in production paths (`docs/governance/no-stub-mvp-doctrine.md`); records reaching a player surface per `reach_gate.rs`; every lane's PI-screening sweep (AT-29-003a); test quality per `docs/governance/book-ingestion-playbook.md §7.4`; no hand-authored rules data under `apps/desktop/src/`.
- Every finding records a disposition: `fixed-in-bundle` or `deferred` with a named owner. No finding is silently dropped.
- Real defects are fixed in-bundle before Epic 11 fires.

## Exit gate checklist

- [ ] All Epic 4-7 lane cycle-batches complete with reach-gate claims, for every book carrying units of that lane's kind.
- [ ] All trap-reports recorded (Epic 2's corpus-wide pre-flight).
- [ ] AT-29-003a provenance gate (Epic 3) recorded for every lane's first content commit per book.
- [ ] AT-29-008 zero-of-kind books correctly absent from the affected lane's cycle-batch list, not skipped as an exception.
- [ ] AT-29-005 build version reads `0.9.<build>`.
- [ ] AT-29-006 identifier discipline exits 0 across all 37 in-scope books' surface code.
- [ ] AT-29-010 rules-as-data verified across all 37 in-scope books' numerical effects.
- [ ] AT-29-011 move-not-copy publish landed.
- [ ] AT-29-012 local-file dispatch verified by Epic 2's pre-flight + Epic 11's closure.
- [ ] AT-29-013 bundle code review (Epic 10) closed; all findings triaged with named owners for deferrals.
- [ ] `release-notes.md` populated with a per-lane rollup.
- [ ] `successor-forward-scope-register.md` reviewed for successor work, including the `class_feature` (15,472-unit corpus-wide per `decisions.md §38.4`; was 90-unit under the seven-book cut's `§37.4`) deferral, owned by SD-30's class_feature/archetype bundle.
