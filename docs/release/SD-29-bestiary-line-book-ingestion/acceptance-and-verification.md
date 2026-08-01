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

## AT-29-001 — Per-cycle file-touch partition

Given a per-book ingest cycle for `<bestiary>` where `<bestiary>` ∈ `{2, 3, 4, 5}`.

When the cycle writes files.

Then:

- Files written under `src/rules_core/rules_tables/beastiary<bestiary>/`, `data/corpus/beastiary<bestiary>/`, `src/bin/sd29_*`, `tests/sd29_*`, or `docs/release/SD-29-.../`.
- No file written under `src/rules_core/pilot_compute.rs`, `src/rules_core/rules_tables/beastiary<other>/`, `docs/release/v0.6/`, `src/oracle_validation/`, or `src/pcgen_import/corpus_traps.rs`.

Evidence: per-cycle receipt carries the audit command and the captured exit code.

## AT-29-002 — Reach-gate claim

Given a per-book record at `src/rules_core/rules_tables/beastiary<bestiary>/<record>.rs`.

When the cycle's reach gate runs.

Then:

- The gate's IPC builder executes the record's slice.
- The gate's exit code is `0`.
- The cycle receipt captures the gate's per-record output.

## AT-29-003 — Pre-cycle trap-report

Given a per-book cycle (Epic 3-6).

When the cycle starts.

Then:

- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` has been run.
- The output is recorded in `artifacts/<book>-trap-report.md`.
- The cycle receipt cites the trap-report file.

## AT-29-004 — Definition-of-done audit

Given a cycle's PR.

When the dual-audit runs.

Then:

- The identifier-discipline audit (`scripts/identifier-discipline-audit.sh` or equivalent) exits 0.
- The wired-integration 4-grep audit exits 0.

## AT-29-005 — Build version

Given the bundle's first concrete build.

When the closure Epic 8 fires.

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

## AT-29-008 — Bestiary 5 shape-resolution

Given Epic 6's pre-flight (cycle-0 trap-report + work-inventory output).

When the cycle runs `cargo run --locked --bin v06_work_inventory`.

Then:

- The `beastiary5` entry's `kinds` field is inspected.
- If `monster` units = 0, Epic 6's cycle runs per-race / per-feat / per-companion-mod cycles instead of per-monster-block.
- The cycle receipt records the shape finding.

## AT-29-009 — Per-entity counts generated

Given a cycle's progress receipt.

When the cycle publishes a figure.

Then:

- The figure cites the `cargo run --locked --bin v06_work_inventory` output that produced it.
- No hand-maintained per-entity counts in the figure.

## AT-29-010 — Rules-as-data, no real-time engines

Given a per-book cycle.

When the cycle writes a numerical effect.

Then:

- The effect is posted as a precomputed value.
- The runtime does not call a die-rolling function for the effect.
- Real-time engines are absent from the cycle's source.

## AT-29-011 — Move-not-copy publish

Given Epic 8 (Closure Epilogue).

When the publish commit fires.

Then:

- The source-of-record directory (`programs/codex/requirements/SD-29-.../`) is removed.
- The canonical repo-resident home (`docs/release/SD-29-bestiary-2-3-4-5-content-ingestion/`) carries the 15-file chassis.

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

Given Epic 10 (Bundle Code Review), firing after all content-ingest epics (3-6, plus Epic 7 if in scope) and Epic 9 (Build Version Numbering) are closed, before Epic 8 (Closure Epilogue).

When the review runs.

Then:

- `./scripts/verify.sh` has a recorded green run — a precondition to the review, not the review itself.
- The diff scope reviewed is the whole bundle against its branch point (`git diff origin/develop...HEAD`), not the closing cycle alone.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` are re-run at bundle scope.
- The review covers, at minimum: rules-logic correctness sampled against the corpus; no stubs/fixture-only data in production paths (`docs/governance/no-stub-mvp-doctrine.md`); records reaching a player surface per `reach_gate.rs`; test quality per `docs/governance/book-ingestion-playbook.md §7.4`; no hand-authored rules data under `apps/desktop/src/`.
- Every finding records a disposition: `fixed-in-bundle` or `deferred` with a named owner. No finding is silently dropped.
- Real defects are fixed in-bundle before Epic 8 fires.

## Exit gate checklist

- [ ] All Epic 3-6 cycles complete with reach-gate claims.
- [ ] All trap-reports recorded.
- [ ] AT-29-008 Bestiary 5 shape-resolution recorded (Epic 6-F1 receipt).
- [ ] AT-29-005 build version reads `0.9.<build>`.
- [ ] AT-29-006 identifier discipline exits 0 across the four bestiaries' surface code.
- [ ] AT-29-010 rules-as-data verified across the four bestiaries' numerical effects.
- [ ] AT-29-011 move-not-copy publish landed.
- [ ] AT-29-012 local-file dispatch verified by Epic 2's pre-flight + Epic 8's closure.
- [ ] AT-29-013 bundle code review (Epic 10) closed; all findings triaged with named owners for deferrals.
- [ ] `release-notes.md` populated.
- [ ] `successor-forward-scope-register.md` reviewed for successor work.
