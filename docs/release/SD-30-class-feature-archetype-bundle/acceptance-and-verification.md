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

**Re-scoped 2026-08-10** (`decisions.md §33-38`). "Sixteen in-scope corpus dirs" below is retired
language — read as "the 23 `class_feature`-bearing corpus dirs, `decisions.md §33`" throughout this
file unless a criterion explicitly says otherwise. AT-30-001 is updated inline for the Epic
5-mechanism exception to the `pilot_compute.rs` touch restriction; a new AT-30-012 covers the
per-class measurement gate.

## AT-30-001 — Per-cycle file-touch partition

Given an Epic 6 chassis-sweep cycle for `<book>` where `<book>` ∈ the 23 `class_feature`-bearing
corpus dirs (`decisions.md §33`), or an Epic 5 mechanism cycle for `<class>`.

When the cycle writes files.

Then:

- Epic 6: files written under `src/rules_core/rules_tables/<book>/`, `data/corpus/<book>/`,
  `src/bin/sd30_*`, `tests/sd30_*`, or `docs/release/SD-30-.../`. No file written under
  `src/rules_core/pilot_compute.rs`, `src/rules_core/rules_tables/<other_book>/`,
  `docs/release/v0.6/`, `src/oracle_validation/`, or `src/pcgen_import/corpus_traps.rs`.
- Epic 5 (the one exception): files may be written under `src/rules_core/pilot_compute.rs` and
  `src/rules_core/archetype_resolver.rs`, scoped to the target class's own supersession/chooser
  branch only — verified by diff review, not just a path check.

**Book → directory mapping (added 2026-08-14, C12):** `src/rules_core/rules_tables/<book>/` cannot
be resolved literally for every one of the 23 books — three long book names carry a short directory
under `src/rules_core/rules_tables/`, verified directly against the checked-out tree:

| book (long name, as used everywhere else in this package) | `rules_tables/` directory |
|---|---|
| `core_rulebook` | `crb` |
| `advanced_players_guide` | `apg` |
| `advanced_class_guide` | `acg` |

Every other book in the 23-book roster uses its long name as the directory verbatim (e.g.
`ultimate_magic` → `ultimate_magic`). A book in the roster with no `src/rules_core/rules_tables/`
directory at all yet (as of 2026-08-14: `adventurers_guide`, `occult_adventures`,
`inner_sea_taverns`, `inner_sea_magic` have none) means "not yet ingested," not a partition
violation — the audit predicate for such a book resolves against an as-yet-nonexistent path and a
first Epic 6 cycle creates it.

Evidence: per-cycle receipt carries the audit command and the captured exit code.

## AT-30-012 — Per-class measurement gate (NEW, 2026-08-10)

Given an Epic 5 or Epic 6 cycle claiming a class-scoped card.

When the cycle claims the card in `kanban.md`.

Then:

- The cycle's receipt in `progress.md` cites that class's Epic 4 measurement receipt
  (`wired-able / named`, direct evidence, no proxy).
- A cycle that claims a class with no such receipt is a protocol violation, recorded as a finding at
  the next Bundle Code Review (Epic 8) if not caught earlier.

Evidence: `progress.md` receipt cross-reference; `kanban.md` claim log.

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

- All 23 `class_feature`-bearing corpus dirs (`decisions.md §33`) have a `artifacts/<book>-cycle0-trap-report.md`.
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
- The canonical repo-resident home (`docs/release/SD-30-class-feature-archetype-bundle/`) carries the 13+ file chassis.

## AT-30-016 — Local-file work-queue dispatch (renumbered 2026-08-14, was duplicate AT-30-012)

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

## AT-30-014 — `static`/`derived` `class_feature` shipments pass SD-32's corpus-wide gates (NEW, 2026-08-13, `decisions.md §41`)

Given an Epic 6 cycle has shipped `class_feature` records into `data/corpus/<book>/`.

When `./scripts/verify.sh` runs (per `AT-30-002`'s standing per-cycle requirement).

Then:

- The `corpus-sweep`/`corpus-sweep-selftest` stages (`corpus_literal_sweep`) examine the newly-shipped
  records as part of their whole-corpus sweep — no bundle-specific static-sweep test is written or
  needed.
- `tests/derived_evaluator_fixture_check.rs` examines the newly-shipped `derived`-class records the
  same way — no bundle-specific evaluator-vs-fixture test is written or needed.
- A cycle whose receipt claims it "needed to build a static-sweep or evaluator-vs-fixture gate for
  class_feature" is a protocol violation — both already exist, corpus-wide, landed by SD-32
  (`decisions.md §41`), and duplicating them is scope creep.
- This criterion does NOT cover the `computed` wiring class (4,178 of 15,472 units) — no gate exists
  for it; a `computed`-bucket criterion is out of scope until the operator resolves ownership of a
  `class_feature` consumer-delta probe (`decisions.md §41`'s flagged question).

Evidence: cycle's `verify.sh` full-run log shows `corpus-sweep`, `corpus-sweep-selftest`, and the
`derived_evaluator_fixture_check` test suite passing over a corpus that includes the cycle's new
records (examined-record count increases, per `scripts/verify.sh`'s own floor checks).

## AT-30-015 — Widened charter: criteria are `done`, never `ingested`/`grounded`, corpus-wide (NEW, 2026-08-13, `decisions.md §43`)

Given the widened charter (`decisions.md §43`, `scope-draft.md`'s "Widened charter" section) —
SD-30 now drives all kinds, corpus-wide, to `done`, not merely `grounded` or `ingested`.

When any cycle or bundle-level closure claim cites a movable-mass or progress figure for any kind.

Then:

- The figure MUST be a `done`-verdict count (per the dashboard producer's `doneness_verdict()` table,
  transcribed and re-validated by `SD-32-.../artifacts/derive-movable-mass.py`), never a `status ==
  "grounded"` count, `status == "ingested-magnitude"` count, or any other process-step count cited as
  if it were closure.
- A receipt or closure claim that reports "`grounded`" or "ingested" figures as evidence of progress
  without also reporting the corresponding `done` figure is a protocol violation, flagged at the next
  Bundle Code Review (Epic 8).
- Per-kind `done` floors below are the closure bar; a kind at 0% `done` (e.g. `race`, `class_feature`
  at effectively 0.1%) is a **structural closure blocker**, not a kind the bundle can silently skip
  while reporting overall progress on kinds that move faster.

**Per-kind `done` floors/targets (re-derived 2026-08-14, superseding the 2026-08-13 table below,
which predated the `static`/`derived` done-rung landing that moved the corpus-wide board from
3,464 to 5,837 `done` units and left this table's equipment denominator, feats/other figures, and
per-kind `done` counts stale). Derivation command (writes no files, reads
`docs/work-inventory.json` directly — the SD-32 `derive-movable-mass.py` script itself currently
raises `ValueError` on the new `('static'|'derived', 'literal-verified')` pair and needs its own
fix before it can be used again for this table; this re-derivation instead applies the *live*
`_doneness_verdict_uncapped()` table from `scripts/observer/pf1e_dashboard_producer.py`, which does
recognize `literal-verified`/`fixture-verified`, by hand against the same inventory file):

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
units = d['units']
def uncapped(wc, st):
    if st == 'deferred-with-reason': return 'deferred'
    if st in ('not-ingested', 'not-started'): return 'not-started'
    if st == 'unknown': return 'unmeasurable'
    if wc == 'ambiguous':
        return 'held' if st in ('grounded', 'text-complete', 'ingested-magnitude') else (_ for _ in ()).throw(ValueError((wc, st)))
    if wc == 'display':
        return 'done' if st == 'text-complete' else 'held' if st == 'grounded' else 'in-progress'
    if wc in ('static', 'derived'):
        if st in ('literal-verified', 'fixture-verified'): return 'done'
        return 'held' if st in ('ingested-magnitude', 'grounded', 'text-complete') else (_ for _ in ()).throw(ValueError((wc, st)))
    if wc == 'computed':
        return 'done' if st == 'grounded' else 'in-progress'
    raise ValueError(wc)
NO_GROUNDING_PROBE = ('companion', 'spell')
def verdict(wc, st, k):
    v = uncapped(wc, st)
    return 'held' if v == 'in-progress' and k in NO_GROUNDING_PROBE else v
by_kind = collections.defaultdict(collections.Counter)
for u in units:
    k = u['kind']; by_kind[k]['total'] += 1; by_kind[k][verdict(u.get('wiring_class'), u['status'], k)] += 1
for k in sorted(by_kind):
    c = by_kind[k]; print(k, c['total'], c.get('done', 0), c.get('held', 0))
"
```
(re-derive at each cycle-0 per the standing "generated, never hand-maintained" rule, `decisions.md
§12`; this table's totals intentionally do NOT exclude `beginner_box`, matching
`state-goals-and-lessons.md`'s equipment figure of 2,626/6,227/42.2% and the corpus-wide 5,837-done
board total — `derive-movable-mass.py`'s own `EXCLUDED_BOOKS = {"beginner_box"}` filter undercounts
equipment by 19 units relative to that reference and is not applied here.)

| kind | total units | `done` today | `held` today | `done` floor for bundle closure | rationale |
|---|---:|---:|---:|---:|---|
| class | 185 | 27 (14.6%) | 0 | 185 (100%) | small population, already instrument-covered, cheapest full closure in the roster |
| class_feature | 15,472 | 25 (0.2%) | 88 | measured per-class via Epic 4 gate, not a blanket % — `decisions.md §37` | population too large and heterogeneous for a flat floor; Epic 4's per-class measurement is the closure instrument |
| companion | 1,696 | 416 (24.5%) | 506 | 922 (54.4%, = today's done+held) | `NO_GROUNDING_PROBE`-capped kind; floor is the `held` ceiling until a companion consumer-delta probe is built |
| equipment | 6,227 | 2,626 (42.2%) | 2,327 | 4,953 (79.5%, = today's done+held) | the `static`/`derived` done rung landed 2026-08-13 and moved this kind from 277 to 2,626 `done`; remaining floor gap is held units awaiting the same rung |
| equipment_modifier | 1,580 | 911 (57.7%) | 19 | 930 (58.9%, = today's done+held) | already the best-covered kind; floor closes the remaining held units |
| feat | 2,610 | 1,178 (45.1%) | 127 | 1,305 (50.0%, done+held) + `unknown` (329) classified | second-best-covered kind; `unknown` residue needs its own characterization pass (new scope, unassigned) |
| monster | 1,270 | 7 (0.6%) | 1,235 | 1,242 (97.8%, = today's done+held) | almost entirely `held`, blocked on the `derived` done rung |
| monster_ability | 3,107 | 334 (10.7%) | 1,295 | 1,629 (52.4%, done+held) | same rung gap as monster |
| race | 103 | 0 (0.0%) | 7 | 103 (100%) | smallest population in the roster; 0% done is a structural blocker per this criterion, not skippable |
| race_trait | 3,447 | 266 (7.7%) | 247 | 513 (14.9%, = today's done+held, the `held` ceiling) | classifier-lever-capped (`ambiguous`/`display` mix); see B1/B2 buckets in the movable-mass tool |
| spell | 2,843 | 47 (1.7%) | 1,235 | 1,282 (45.1%, = today's done+held, `NO_GROUNDING_PROBE`-capped) | no consumer reads a spell magnitude corpus-wide; floor is the `held` ceiling until that changes |

Evidence: per-cycle receipt re-runs the movable-mass derivation command and cites the resulting
`done`/`held`/`floor` figures for the kind(s) it touched, cross-referenced against this table.

## Exit gate checklist

- [ ] All Epic 3+ per-book cycles complete with reach-gate claims.
- [ ] All trap-reports recorded.
- [ ] AT-30-002 reach-gate claims have `> 0` matched-tests per cycle.
- [ ] AT-30-005 build version reads `0.10.<build>`.
- [ ] AT-30-006 identifier discipline exits 0 across the 23-book `class_feature` roster's surface code.
- [ ] AT-30-007 cross-book precedence (SD-28/SD-29 doctrine) verified across shared records.
- [ ] AT-30-010 rules-as-data verified across the 23-book `class_feature` roster's numerical effects.
- [ ] AT-30-011 move-not-copy publish landed.
- [ ] AT-30-012 per-class measurement gate cross-referenced for every Epic 5/6 cycle's claim this closure (added 2026-08-14 — previously absent from this checklist).
- [ ] AT-30-016 local-file dispatch verified by Epic 2's pre-flight + Closure.
- [ ] AT-30-013 bundle code review (Epic 21) closed; all findings triaged with named owners for deferrals.
- [ ] `release-notes.md` populated.
- [ ] `forward-scope-register.md` reviewed for successor work.
- [ ] The four deferred books (NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes) recorded as future-acquisition candidates.
- [ ] AT-30-015 (NEW, 2026-08-13) per-kind `done` figures re-derived and checked against the floor table for every kind touched this closure; no kind sitting at or near 0% `done` reported as "in progress" without naming it a structural blocker.
