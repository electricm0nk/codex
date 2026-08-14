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

Given a cycle about to claim `epic-3-chassis-sweep`, `epic-4-ingest-lanes`, or `epic-5-book-onboarding`
for a specific book.

When the claim is recorded in `kanban.md`/`progress.md`.

Then the cycle's first receipt cites the specific `SD-30-class-feature-archetype-bundle/progress.md`
entry showing that book's declared-PI screen (SD30-E3-F2/F3) as `COMPLETE`. A claim without that
citation is a protocol violation, flagged at this package's own Bundle Code Review (deferred to
`SD-30-.../epic-8-code-review`, since this package does not run a separate code-review epic — see
`README.md`'s "Out of scope").

## AT-31-004 — Raw-vs-workable split precedes any ingest-lane cycle (SD-29 lesson, inherited)

Given any card under Epic 4 (`monster`/`spell`/`race`/`race_trait` ingest lanes).

When a cycle plans to claim a book for that card.

Then the raw-vs-workable split (structurally-blocked units excluded, e.g. the ~2,894 chassis-blind
`race_trait` units, the 719 negated-PCC-gate-excluded units corpus-wide) is computed and cited with its
command **before** the cycle claims the book — not after, and not assumed unchanged from a prior
session's figure without re-running the command.

## AT-31-005 — Per-kind `done`-floor table (moved from `SD-30-.../acceptance-and-verification.md` AT-30-015)

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

Given one of the 7 `future_state` books (Epic 5).

When the first record for that book lands in `rules_tables/`.

Then the book's declared-PI screen (SD30-E3-F2/F3, cross-SD gate, AT-31-003) is cited `COMPLETE`
first, and the 55-term blacklist sweep (`scripts/verify.sh` `pi-sweep`) is clean for the book's newly
generated content.

## Exit gate checklist

- [ ] AT-31-001 — no blended per-class measurement figure reported, ever.
- [ ] AT-31-002 — reach-gate claims `> 0` matched-tests per cycle, every record ingested under Epic 3/4/5.
- [ ] AT-31-003 — every ingest-cycle claim cites its cross-SD PI-gate `COMPLETE` receipt.
- [ ] AT-31-004 — raw-vs-workable split recorded with command, every Epic 4 card, before the cycle that used it.
- [ ] AT-31-005 — per-kind `done` figures re-derived and checked against the floor table at closure.
- [ ] AT-31-006 — all 7 `future_state` books onboarded PI-clean.
- [ ] `forward-scope-register.md` reviewed for successor work.
- [ ] `release-notes.md` populated.
