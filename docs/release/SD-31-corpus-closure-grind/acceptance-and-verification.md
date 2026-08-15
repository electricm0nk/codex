---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
---

# SD-31 Acceptance and Verification

Given/When/Then criteria for this package's own epics. General discipline (reach-gate prime rule,
cross-book conflict resolution, identifier discipline, `verify.sh` gate stages) is inherited unchanged
from `SD-30-class-feature-archetype-bundle/acceptance-and-verification.md` and is not re-derived here.

## AT-31-001 — Per-class measurement is never blended (moved from SD30-E4's standing discipline)

Given a class this package's Epic 1 measures.

When a `wired-able / named` figure is reported for that class.

Then:

- The figure is direct evidence from `pilot_compute.rs` (or the class's own compute module), never an
  extrapolation from another class's ratio.
- No blended percentage is reported across classes.
- A near-miss slot name is not counted as evidence without direct confirmation.

## AT-31-002 — Reach-gate satisfaction per ingested record (inherited prime rule, restated for this package's ingest epics)

Given a `class_feature`, `monster`, `spell`, `race`, or `race_trait` record ingested under this
package's Epic 3, 4, or 5.

When the record lands in `src/rules_core/rules_tables/<book>/`.

Then the reach-gate claim executes the real IPC builder (`apps/desktop/src-tauri/src/reach_gate.rs`) —
not a stub, not a mocked builder — matching `SD-30-.../decisions.md §18`'s prime rule unchanged.

## AT-31-003 — Cross-SD PI-gate citation is mandatory before any ingest cycle claims a book

Given a cycle about to claim `epic-5-chassis-sweep`, `epic-6-ingest-lanes`, or `epic-7-book-onboarding`
for a specific book.

When the claim is recorded in `kanban.md`/`progress.md`.

Then the cycle's first receipt cites the specific `SD-30-class-feature-archetype-bundle/progress.md`
entry showing that book's declared-PI screen (SD30-E3-F2/F3) as `COMPLETE`. A claim without that
citation is a protocol violation. The production path must also *call* the documented readers
(`forward-scope-register.md` `G1.4` blacklist sweep, `G1.5` declared-PI reader) — a cited receipt with
no reader call in the ingest binary satisfies neither this criterion nor the gate it stands for.

**Note on where a violation is caught.** An earlier revision deferred this to
`SD-30-.../epic-8-code-review`. SD-30 closed 2026-08-14 and its Epic 8 reviewed SD-30's own diff, which
does not include this package's work — so that deferral now points at a review that already happened.
Whether this package runs a review step of its own is an open question
(`risks-and-open-questions.md` open question 2), and Epic 9's exit gate is the current backstop.

## AT-31-004 — Raw-vs-workable split precedes any ingest-lane cycle (SD-29 lesson, inherited)

Given any card under Epic 4 (`monster`/`spell`/`race`/`race_trait` ingest lanes).

When a cycle plans to claim a book for that card.

Then the raw-vs-workable split (structurally-blocked units excluded, e.g. the ~2,894 chassis-blind
`race_trait` units, the 719 negated-PCC-gate-excluded units corpus-wide) is computed and cited with its
command **before** the cycle claims the book — not after, and not assumed unchanged from a prior
session's figure without re-running the command.

## AT-31-005 — Per-kind `done`-floor table: PROGRESS FLOORS, NOT CLOSURE CRITERIA (moved from `SD-30-.../acceptance-and-verification.md` AT-30-015; relabelled `decisions.md §5`, operator ruling 2026-08-15)

**Relabelled 2026-08-15 (`decisions.md §5`, launch-readiness blocker B1).** The table below states
`done+held` floors — a floor a cycle can satisfy without any single unit reaching `done` (e.g. the
`race_trait` floor is `513/3,447, 14.9%, done+held`, satisfiable entirely by `held` units). Decision
1(a)'s anti-gaming rule explicitly forbids counting `held` as `done`. These floors are **progress
floors** — a per-cycle "is this kind moving" signal — and are **never**, on their own, sufficient to
close an epic or the package. The package's actual closure criterion is the doneness bar,
`AT-31-103` below.

Given the widened, split program's charter (`SD-30-.../decisions.md §43-§45`, `§51`) — the kinds this
package's epics own (`class_feature`, `monster`, `spell`, `race`, `race_trait`) are driven to `done`,
never merely `grounded`/`ingested`.

When any cycle or this package's own closure claim cites a movable-mass or progress figure for any of
these kinds.

Then:

- The figure MUST be a `done`-verdict count, never a `status == "grounded"` or `"ingested-magnitude"`
  count cited as if it were closure.
- Per-kind `done` floors (re-derive at time of use; figures below are re-derived as of SD-30's Epic 0
  closure, 2026-08-14, `SD30-E0-F4-001` — superseding the original split-time snapshot inherited from
  `SD-30-.../acceptance-and-verification.md AT-30-015`, which is left visible in that package per its
  own standing convention, not this table's live source. **Correction (`SD30-E0-F4-001`):** the
  split-time snapshot's `spell` row read `held: 1,235` / `done+held: 1,282 (45.1%)` — both values are
  copy-paste artifacts of the `monster` row directly above (`monster`'s own `held` is 1,235; `1,282`
  is `feat`'s done+held% from a different table entirely). The true `spell` `held` figure, re-derived
  by importing `pf1e_dashboard_producer.doneness_verdict()` and replaying it over
  `docs/work-inventory.json`, is **1,103** (`done+held` = 1,150, 40.4%), not 1,235/1,282. The
  `NO_GROUNDING_PROBE`-capped rationale is also stale: SD-30 Epic 0-F2 (`decisions.md`-cited,
  `kanban.md` `SD30-E0-F2-001`) removed `spell` from `NO_GROUNDING_PROBE` and confirmed it reaches a
  nonzero `grounded` count under `computed`; 132 `spell` units moved `held`→`in-progress` as a result
  (visible below), not a cap that still applies.):

| kind | total units | `done` (re-derived) | `held` (re-derived) | `done` floor for this package's closure | rationale |
|---|---:|---:|---:|---:|---|
| class_feature | 15,472 | 25 (0.2%) | 88 | measured per-class via Epic 1 gate, not a blanket % | population too large/heterogeneous for a flat floor |
| monster | 1,270 | 7 (0.6%) | 1,235 | 1,242 (97.8%, done+held) | almost entirely `held`, blocked on SD-30 Epic 0's `derived` done rung |
| spell | 2,843 | 47 (1.7%) | 1,103 | 1,150 (40.4%, done+held) — `NO_GROUNDING_PROBE` cap lifted (SD-30 Epic 0-F2); 132 units already `in-progress` toward `grounded` under the new probe | no consumer reads a spell magnitude corpus-wide beyond the probe's current coverage |
| race | 103 | 0 (0.0%) | 7 | 103 (100%) — **needs SD-32's race chassis** to clear past the ~7-grounded ceiling | smallest population; 0% is a structural blocker, not skippable |
| race_trait | 3,447 | 266 (7.7%) | 247 | 513 (14.9%, done+held) *without* SD-32's chassis; up to 3,447 *with* it | classifier-lever-capped without the chassis |

Command (re-run this cycle, cross-checked byte-identical against the live
`/home/ubuntu/swarm-observer/PF1e-dashboard.json` `work_inventory.by_doneness_kind`,
`generated_at` 2026-08-14T21:26:18Z):

```
python3 -c "
import json, importlib.util, collections
spec = importlib.util.spec_from_file_location('m', 'scripts/observer/pf1e_dashboard_producer.py')
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
d = json.load(open('docs/work-inventory.json'))['units']
by_kind = collections.defaultdict(collections.Counter)
for u in d:
    if u.get('book') == 'beginner_box': continue
    by_kind[u.get('kind')][mod.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind'))] += 1
for k in ('class_feature','monster','spell','race','race_trait'):
    print(k, dict(by_kind[k]))
"
```

Evidence: per-cycle receipt re-runs this derivation command against `docs/work-inventory.json` and
cites the resulting `done`/`held`/`floor` figures for the kind(s) it touched.

## AT-31-006 — Book onboarding is PI-clean before any record ships

Given one of the 7 `future_state` books (Epic 7).

When the first record for that book lands in `rules_tables/`.

Then the book's declared-PI screen (SD30-E3-F2/F3, cross-SD gate, AT-31-003) is cited `COMPLETE`
first, and the 55-term blacklist sweep (`scripts/verify.sh` `pi-sweep`) is clean for the book's newly
generated content.

## AT-31-007 — Race chassis: DoD-8 on-screen verification mandatory (absorbed from `SD-32` AT-32-001; orig. SD30-E12)

Given a new race chassis entry (Epic 1).

When the chassis lands and a character sheet is built with that race.

Then a real, on-screen character sheet (not a static/derived instrument report alone) shows the race's
true ability score modifiers, size, speed, and languages — DoD-8 verification is not satisfied by a
passing unit test or a green dashboard cell alone.

## AT-31-008 — Race chassis does not break the 18 already-modeled races (absorbed from `SD-32` AT-32-002)

Given any chassis-build change under Epic 1.

When the change lands.

Then `RaceCorpus::resolve` still returns the correct value for every one of the 18 previously-modeled
races — a regression test proves this, not an assertion.

## AT-31-009 — Classifier accepted on accuracy, never on movement (absorbed from `SD-32` AT-32-003; rule at `decisions.md` Decision 1(e))

Given the verdict-path classifier (Epic 2).

When its output is evaluated for acceptance.

Then:

- The acceptance criterion is agreement rate against a ≥100-unit hand-labelled sample (stratified
  across the five wiring classes and at least four kinds, labelled before the classifier existed),
  reported per class and per kind, with a full confusion matrix.
- Movement is reported in both directions; a net-negative effect on `done` is a passing outcome if the
  sample supports it.
- There is no target count of units moved anywhere in this epic's acceptance. A classifier presented
  with "we moved N units to done" as its primary evidence is rejected pending the sample-agreement
  evidence instead.

## AT-31-010 — The `ambiguous` dead-end is closed or signed off, AND the `display|grounded` held population is examined (Epic 2-F3) (widened 2026-08-15, launch-readiness remediation Step 2, blocker B4)

Given `wiring_class == ambiguous`, which at authoring time reached `done` from **no status at all**
(2,109 units — re-derive, `decisions.md §2`).

When Epic 2 closes.

Then either `scripts/reachability_audit.py` shows `ambiguous` reaching `done` from at least one status,
or every affected unit carries a signed `AT-31-100` register entry. Epic 2 may not close leaving the
class structurally unreachable and unregistered — that outcome silently caps the board below 100 % and
is the specific failure the merge exists to prevent.

**Widened 2026-08-15 (launch-readiness remediation Step 2, blocker B4).** This criterion previously
bound only `ambiguous`. Re-derived this cycle, corpus-wide:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
U = [u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS
     and u.get('wiring_class')=='display' and u.get('status')=='grounded']
print(len(U)); print(dict(collections.Counter(u.get('kind') for u in U)))
"
```
→ **1,243 units**, all `held` (matches `decisions.md §2`'s cited B4 population exactly):
`monster_ability 981`, `companion 182`, `class_feature 54`, `race_trait 23`, `feat 3`. This is
Decision 1(e)'s own named target ("re-examines `display`+`grounded` (1,416 units)" — that 1,416 was
the split-time snapshot; 1,243 is this cycle's re-derivation, drift expected and not itself a defect)
but this AT previously bound only the `ambiguous` half of Decision 1(e)'s classifier scope, leaving
the `display|grounded` half's examination criterion unstated anywhere in `acceptance-and-
verification.md`.

Given `wiring_class == display, status == grounded` (1,243 units, re-derive at time of use).

When Epic 2 closes.

Then the ground-truth-sample classifier (`SD31-E2-F1`/`F2`) is applied to this population exactly as
Decision 1(e) specifies: agreement-rate acceptance, both-direction movement reported, and — per
Decision 1(e) item 4 — if the sample shows the current classification substantially correct, this
population is reported "examined, correctly classified, left alone" (a passing, `COMPLETE` outcome,
not a mandate to force reclassification). What is **not** acceptable is Epic 2 closing having never
examined this population at all — the same silent-cap failure this AT already forbids for
`ambiguous`, applied to the second half of the same classifier's scope.

## AT-31-100 — The Structural Exclusion Register (`decisions.md §3`)

Given a unit this package cannot bring to `done`.

When a cycle proposes to remove it from the 100 % denominator.

Then the register entry carries **all four** of: (1) the exact command, run this cycle, showing no path
to `done` exists for that unit; (2) the named missing capability and why building it is genuinely
impossible or out-of-charter — **cost is never an exclusion reason**; (3) the Epic 0 audit run that
reproduces the finding independently; (4) **operator sign-off, with its date**.

A cycle may **propose**; only the operator **grants**. An unsigned proposal leaves the unit in the
denominator and its epic open. The register lives in this file, below, and is the only mechanism by
which this package's 100 % bar can be reduced.

### Register

| ID | Unit(s) | Missing capability | Proving command | Epic 0 run | Operator sign-off |
|---|---|---|---|---|---|
| — | *(empty at authoring; entries added only with sign-off)* | — | — | — | — |

## AT-31-101 — The two internal capability gates are observed

Given a cycle about to claim `epic-6-ingest-lanes` F3/F4, or `epic-3-measurement` F4, or
`epic-5-chassis-sweep` F3.

When it makes the claim.

Then its receipt cites, respectively: the named Epic 1 race batch covering the races the target book's
rows reference (an epic-level "in flight" is **not** an open gate — the batch list in `kanban.md` is);
or `epic-2-verdict-paths` at `COMPLETE`. A claim across an open gate is out of protocol, identical in
standing to a PI-gate violation.

## AT-31-102 — Reachable ceiling is reported, never assumed

Given any epic closure, and the package closure.

When the receipt is written.

Then `scripts/reachability_audit.py` was run at that tip and its **reachable ceiling** figure and
dead-end list are quoted in the receipt. At package closure the ceiling reads **100 %**, or every
shortfall unit carries a signed `AT-31-100` entry. A receipt that quotes a sub-100 % ceiling and
proceeds to close without signed entries is not a valid closure.

## AT-31-103 — Doneness bar (`decisions.md §5`, operator ruling 2026-08-15)

Given any epic closure, and the package closure.

When the receipt is written.

Then the receipt quotes the **mandate denominator replay** — the strict, whole-board denominator, not
the in-scope-books-only secondary — run at that tip:

```
python3 -c "import json,sys,collections; sys.path.insert(0,'scripts/observer'); import
pf1e_dashboard_producer as P; U=[u for u in json.load(open('docs/work-inventory.json'))['units'] if
u.get('book') not in P.EXCLUDED_BOOKS]; c=collections.Counter(P.doneness_verdict(u.get('wiring_class'),
u.get('status'), u.get('kind')) for u in U); print(c, len(U))"
```

and either:

- `done / denominator == 100 %` (denominator = `len(U)` above, every unit in
  `docs/work-inventory.json` except `EXCLUDED_BOOKS`; 38,521 at this decision's authoring, re-derive at
  time of use), **or**
- every shortfall unit (`denominator - done`) carries a signed `AT-31-100` register entry.

This bar is **distinct from and additional to `AT-31-102`'s reachable-ceiling bar** — reachability asks
whether a path to `done` exists for a unit given current capability; doneness asks whether the unit
actually reached `done`. A reachable ceiling of 100 % does not satisfy this criterion by itself, and a
receipt satisfying only `AT-31-102` while `done / denominator < 100 %` (without signed entries covering
every shortfall unit) is not a valid closure under this test. `AT-31-005`'s per-kind `done+held` floors
do not satisfy this bar either — they are progress floors, not closure criteria (`AT-31-005`, as
relabelled).

**Invariance note.** Neither Epic 2 (resolving `unmeasurable`/`ambiguous` units) nor Epic 7 (onboarding
the 7 `future_state` books, already inside the 38,521-unit denominator today) moves the denominator —
only a unit reaching `done` moves this figure. A receipt that shows the denominator changing between
two runs of this replay (other than by an operator-signed `AT-31-100` exclusion) is a protocol
violation and must be investigated before the closure proceeds.

## Exit gate checklist

- [ ] AT-31-001 — no blended per-class measurement figure reported, ever.
- [ ] AT-31-002 — reach-gate claims `> 0` matched-tests per cycle, every record ingested under Epics 5/6/7.
- [ ] AT-31-003 — every ingest-cycle claim cites the PI-gate `COMPLETE` receipt for its specific book.
- [ ] AT-31-004 — raw-vs-workable split recorded with command, every Epic 6 card, before the cycle that used it.
- [ ] AT-31-005 — per-kind `done` figures re-derived and checked against the floor table at closure
      (progress floors only — passing this row alone does not satisfy AT-31-103).
- [ ] AT-31-006 — all 7 `future_state` books onboarded PI-clean.
- [ ] AT-31-007 — DoD-8 on-screen verification recorded for every race added.
- [ ] AT-31-008 — the 18 previously-modeled races still resolve, proven by regression test.
- [ ] AT-31-009 — classifier accepted on sample agreement, with both-direction movement reported.
- [ ] AT-31-010 — `ambiguous` reaches `done`, or every affected unit is signed off; the 1,243-unit
      `display|grounded` population is examined by the same classifier (widened 2026-08-15).
- [ ] AT-31-100 — no unit left the denominator without an operator-signed register entry.
- [ ] AT-31-101 — no cycle claimed across an open capability gate.
- [ ] AT-31-102 — reachability audit run at the closing tip; **reachable ceiling 100 %**, or every
      shortfall unit signed off.
- [ ] AT-31-103 — **doneness bar**: mandate denominator replay run at the closing tip;
      `done / denominator == 100 %` (38,521-unit strict denominator), or every shortfall unit signed
      off. Distinct from AT-31-102; both must pass.
- [ ] `forward-scope-register.md` reviewed for successor work.
- [ ] `release-notes.md` populated.
