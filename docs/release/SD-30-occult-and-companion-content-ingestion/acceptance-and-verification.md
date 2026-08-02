---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/10 (operator directive 2026-08-01)
build_version_target: 0.10.<build>
---

# SD-30 Acceptance Tests

Tests are Given/When/Then format, paired with the technical requirements
in `technical-requirements.md` and the epics in `epic-breakdown.md`.

## AT-30-001 — Per-cycle file-touch partition

Given a per-book ingest cycle for `<book>` where `<book>` ∈ sixteen in-scope corpus dirs.

When the cycle writes files.

Then:

- Files written under `src/rules_core/rules_tables/<book>/`, `data/corpus/<book>/`, `src/bin/sd30_*`, `tests/sd30_*`, or `docs/release/SD-30-.../`.
- No file written under `src/rules_core/pilot_compute.rs`, `src/rules_core/rules_tables/<other_book>/`, `docs/release/v0.6/`, `src/oracle_validation/`, or `src/pcgen_import/corpus_traps.rs`.

Evidence: per-cycle receipt carries the audit command and the captured exit code.

## AT-30-002 — Reach-gate claim (PRIME RULE)

Given a per-book record at `src/rules_core/rules_tables/<book>/<record>.rs`.

When the cycle's reach gate runs.

Then:

- The gate's IPC builder executes the record's slice.
- The gate's exit code is `0`.
- The gate's matched-tests count is `> 0` (a gate running zero tests asserts nothing and is a hard failure).
- The cycle receipt captures the gate's per-record output.

## AT-30-003 — Pre-cycle trap-report

Given a per-book cycle.

When the cycle starts.

Then:

- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` has been run.
- The output is recorded in `artifacts/<book>-trap-report.md`.
- The cycle receipt cites the trap-report file.

## AT-30-004 — Definition-of-done audit

Given a cycle's PR.

When the dual-audit runs.

Then:

- The identifier-discipline audit (`scripts/identifier-discipline-audit.sh` or equivalent) exits 0.
- The wired-integration 4-grep audit exits 0.

## AT-30-005 — Build version

Given the bundle's first concrete build.

When the closure fires.

Then:

- `0.10.<build>` is the post-closure value, where `<build>` is the recorded build counter at cycle close.
- The next bundle (post-tranche-promotion) reads `0.10.<last_build>` as its starting point.

## AT-30-006 — Identifier discipline

Given any file written by a cycle.

When the cycle commits.

Then:

- No `sd30_*`, `SD30_*`, `Sd30*`, `sd30-*` patterns in the file.
- No `t_<hex>` kanban tokens.
- The identifier-discipline audit exits 0.

## AT-30-007 — Cross-book conflict rule (newer = doctrine; recently-published precedence)

Given two records — one in SD-30's book and one in SD-28 / SD-29's already-published surface — that conflict on a record id.

When the cycle determines which is doctrine.

Then:

- The SD-28 / SD-29 record is doctrine (recently-published precedence per `decisions.md §16`).
- SD-30 references the SD-28 / SD-29 canonical id; SD-30 does not redefine.

Exception: class-grant overlap (Occultist, Spiritualist, Medium, Mesmerist
in SD-28's Ultimate Intrigue territory) follows the bundle-owns
doctrine rule; SD-30 owns canonical class definitions.

## AT-30-008 — Cycle-0 trap-report + work-inventory gating

Given Epic 2's pre-flight.

When the trap-report + work-inventory run.

Then:

- All sixteen in-scope books have a `artifacts/<book>-cycle0-trap-report.md`.
- Per-book inventory findings are recorded.

## AT-30-009 — Per-entity counts generated

Given a cycle's progress receipt.

When the cycle publishes a figure.

Then:

- The figure cites the `cargo run --locked --bin v06_work_inventory` output that produced it.
- No hand-maintained per-entity counts in the figure.

## AT-30-010 — Rules-as-data, no real-time engines (PRIME RULE)

Given a per-book cycle.

When the cycle writes a numerical effect.

Then:

- The effect is posted as a precomputed value where appropriate.
- The runtime does not call a die-rolling function for the effect.
- Real-time engines are absent from the cycle's source.
- Rules-data engines are present only where strictly necessary to satisfy AT-30-002.

## AT-30-011 — Move-not-copy publish (landed 2026-08-01; Closure re-verifies)

Given the closure.

When the publish commit fires (fired 2026-08-01).

Then:

- The source-of-record directory (`programs/codex/requirements/SD-30-.../`) is removed.
- The canonical repo-resident home (`docs/release/SD-30-occult-and-companion-content-ingestion/`) carries the 13+ file chassis.

## AT-30-012 — Local-file work-queue dispatch

Given the cycle supervisor.

When the supervisor reads `kanban.md` at top of each cycle.

Then:

- The supervisor finds at least one ready card.
- The supervisor claims one card by editing `kanban.md`.
- The supervisor writes the cycle receipt to `progress.md`.
- The supervisor closes the card on cycle completion.

## AT-30-013 — Bundle code review (final epic)

Given Epic 21 (Bundle Code Review), firing after all content-ingest epics and Build Version Numbering are closed, before Closure Epilogue.

When the review runs.

Then:

- `./scripts/verify.sh` has a recorded green run — a precondition to the review, not the review itself.
- The diff scope reviewed is the whole bundle against its branch point (`git diff origin/develop...HEAD`), not the closing cycle alone.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` are re-run at bundle scope.
- The review covers, at minimum: rules-logic correctness sampled against the corpus; no stubs/fixture-only data in production paths (`docs/governance/no-stub-mvp-doctrine.md`); records reaching a player surface per `reach_gate.rs`; test quality per `docs/governance/book-ingestion-playbook.md §7.4`; no hand-authored rules data under `apps/desktop/src/`.
- Every finding records a disposition: `fixed-in-bundle` or `deferred` with a named owner. No finding is silently dropped.
- Real defects are fixed in-bundle before Closure Epilogue fires.

## Exit gate checklist

- [ ] All Epic 3+ per-book cycles complete with reach-gate claims.
- [ ] All trap-reports recorded.
- [ ] AT-30-002 reach-gate claims have `> 0` matched-tests per cycle.
- [ ] AT-30-005 build version reads `0.10.<build>`.
- [ ] AT-30-006 identifier discipline exits 0 across the sixteen books' surface code.
- [ ] AT-30-007 cross-book precedence (SD-28/SD-29 doctrine) verified across shared records.
- [ ] AT-30-010 rules-as-data verified across the sixteen books' numerical effects.
- [ ] AT-30-011 move-not-copy publish landed.
- [ ] AT-30-012 local-file dispatch verified by Epic 2's pre-flight + Closure.
- [ ] AT-30-013 bundle code review (Epic 21) closed; all findings triaged with named owners for deferrals.
- [ ] `release-notes.md` populated.
- [ ] `forward-scope-register.md` reviewed for successor work.
- [ ] The four deferred books (NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes) recorded as future-acquisition candidates.
