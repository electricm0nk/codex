---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-08-01)
date: 2026-08-01
canonical_branch: tranche/8 (operator directive 2026-08-01)
build_version_target: 0.8.<build>
---

# SD-28 Acceptance Tests

Tests are Given/When/Then format, paired with the technical requirements
in `technical-requirements.md` and the epics in `epic-breakdown.md`.

## AT-28-001 — Per-cycle file-touch partition

Given a per-book ingest cycle for `<book>` where `<book>` ∈ `{ultimate_combat, ultimate_magic, ultimate_equipment, ultimate_intrigue, ultimate_campaign, ultimate_wilderness, ultimate_psionics}`.

When the cycle writes files.

Then:

- Files written under `src/rules_core/rules_tables/<book>/`, `data/corpus/<book>/`, new bins under `src/bin/`, new tests under `tests/` (named per identifier-discipline doctrine; no `sd28_` prefix), or `docs/release/SD-28-.../`.
- No file written under `src/rules_core/pilot_compute.rs`, `src/rules_core/rules_tables/<other_book>/`, `docs/release/v0.6/`, `src/oracle_validation/`, or `src/pcgen_import/corpus_traps.rs`.

Evidence: per-cycle receipt carries the audit command and the captured exit code.

## AT-28-002 — Reach-gate claim

Given a per-book record at `src/rules_core/rules_tables/<book>/<record>.rs`.

When the cycle's reach gate runs.

Then:

- The gate's IPC builder executes the record's slice.
- The gate's exit code is `0`.
- The cycle receipt captures the gate's per-record output.

## AT-28-003 — Pre-cycle trap-report

Given a per-book cycle (Epic 3-9).

When the cycle starts.

Then:

- `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` has been run.
- The output is recorded in `artifacts/<book>-trap-report.md`.
- The cycle receipt cites the trap-report file.

## AT-28-003a — Per-book trap-report audit gate

Given a per-book cycle (Epic 3-9) at the Definition of done.

When the cycle runs `cargo run --locked --bin v06_corpus_trap_report -- --audit`.

Then:

- The audit exit code is `0` **for all defects in this book's own records**.
- A pre-existing defect in another bundle's content (e.g., ACG data referenced as a cross-bundle dependency) is recorded as a cross-bundle blocker against that bundle via this cycle's progress receipt, not against this book.
- The scope of "this book's own records" means records filed under the book's corpus directory; cross-bundle dependencies are out of scope for this book's gate.
- **Rationale (Decision 31):** As originally written, the gate was repo-wide, so a single out-of-scope defect anywhere halted all seven books at once (Run 1, 2026-08-02). This narrowing permits each book to proceed once its own records are clean, while documenting cross-bundle blockers for later remediation by the responsible bundle.

## AT-28-004 — Definition-of-done audit

Given a cycle's PR.

When the dual-audit runs.

Then:

- The identifier-discipline audit (`scripts/identifier-discipline-audit.sh` or equivalent) exits 0.
- The wired-integration 4-grep audit exits 0.
- The Dreamscarred Press PI-blacklist audit (when Epic 9 fires) exits 0.

## AT-28-005 — Build version

Given the bundle's first concrete build.

When the closure Epic 10 fires.

Then:

- `0.8.<build>` is the post-closure value, where `<build>` is the recorded build counter at cycle close.
- The next bundle (post-tranche-promotion) reads `0.8.<last_build>` as its starting point.

## AT-28-006 — Identifier discipline

Given any file written by a cycle.

When the cycle commits.

Then:

- No `sd28_*`, `SD28_*`, `Sd28*`, `sd28-*` patterns in the file.
- No `t_<hex>` kanban tokens.
- The identifier-discipline audit exits 0.

## AT-28-007 — Cross-book conflict rule

Given two records — one in SD-28's book and one in a closed/adjacent SD-N's book — that conflict on a record id.

When the cycle determines which is doctrine.

Then:

- The newer book is doctrine.
- The older book carries an erratum note referencing the newer book's record id.

Exception: class-grant overlap (per `decisions.md §5`) follows the
class-grant rule; SD-30 owns canonical class definitions.

## AT-28-008 — Dreamscarred Press license gate

Given the Epic 9 license precheck.

When the trap-report runs against `dreamscarred_press/ultimate_psionics/`.

Then:

- The cycle records a license-conformance finding per record.
- Records not matching open-content tier are dropped from cycle scope with a per-record justification.
- The precheck output is captured in `artifacts/dreamscarred-license-precheck.md`.

## AT-28-009 — Per-entity counts generated

Given a cycle's progress receipt.

When the cycle publishes a figure.

Then:

- The figure cites the `cargo run --locked --bin v06_work_inventory` output that produced it.
- No hand-maintained per-entity counts in the figure.

## AT-28-010 — Rules-as-data, no real-time engines

Given a per-book cycle.

When the cycle writes a numerical effect.

Then:

- The effect is posted as a precomputed value (e.g., `6d6` for a 1d6/level effect at caster level 6).
- The runtime does not call a die-rolling function for the effect.
- Real-time engines are absent from the cycle's source.

## AT-28-011 — Move-not-copy publish

Given Epic 10 (Closure Epilogue).

When the publish commit fires.

Then:

- The source-of-record directory (`programs/codex/requirements/SD-28-ultimate-book-content-ingestion/`) is removed.
- The canonical repo-resident home (`docs/release/SD-28-ultimate-book-content-ingestion/`) carries the 12-file chassis.

## AT-28-012 — Local-file work-queue dispatch

Given the cycle supervisor.

When the supervisor reads `kanban.md` at top of each cycle.

Then:

- The supervisor finds at least one ready card.
- The supervisor claims one card by editing `kanban.md` (claimed-by, claimed-at, cycle-id).
- The supervisor writes the cycle receipt to `progress.md`.
- The supervisor closes the card on cycle completion.

No Hermes-board interaction is required.

## AT-28-013 — Bundle code review (final epic)

Given Epic 12 (Bundle Code Review), firing after all content-ingest epics (3-9) and Epic 11 (Build Version Numbering) are closed, before Epic 10 (Closure Epilogue).

When the review runs.

Then:

- `./scripts/verify.sh` has a recorded green run — a precondition to the review, not the review itself.
- The diff scope reviewed is the whole bundle against its branch point (`git diff origin/develop...HEAD`), not the closing cycle alone.
- `scripts/identifier-discipline-audit.sh` and `scripts/wired-integration-audit.sh` are re-run at bundle scope.
- The review covers, at minimum: rules-logic correctness sampled against the corpus; no stubs/fixture-only data in production paths (`docs/governance/no-stub-mvp-doctrine.md`); records reaching a player surface per `reach_gate.rs`; test quality per `docs/governance/book-ingestion-playbook.md §7.4`; no hand-authored rules data under `apps/desktop/src/`.
- Every finding records a disposition: `fixed-in-bundle` or `deferred` with a named owner. No finding is silently dropped.
- Real defects are fixed in-bundle before Epic 10 fires.

## Exit gate checklist

- [ ] All Epic 3-9 cycles complete with reach-gate claims.
- [ ] All trap-reports recorded.
- [ ] AT-28-008 license precheck passed (or recorded exceptions).
- [ ] AT-28-005 build version reads `0.8.<build>`.
- [ ] AT-28-006 identifier discipline exits 0 across the seven books' surface code.
- [ ] AT-28-010 rules-as-data verified across the seven books' numerical effects.
- [ ] AT-28-011 move-not-copy publish landed.
- [ ] AT-28-012 local-file dispatch verified by Epic 2's pre-flight + Epic 10's closure.
- [ ] AT-28-013 bundle code review (Epic 12) closed; all findings triaged with named owners for deferrals.
- [ ] `release-notes.md` populated.
- [ ] `forward-scope-register.md` reviewed for successor work.
