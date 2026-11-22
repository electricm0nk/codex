---
canonical: true
owner: god-emporer
status: planning-ready (SD-32 absorbed, epics re-sequenced, operator ruling 2026-08-15)
date: 2026-08-15
canonical_branch: tranche/11
companion_to: ./README.md
mirror_of: ./README.md
build_version_target: 0.11.<build>
---

# SD-31 — Corpus Closure: the Grind and the Capability Builds, scope

See `README.md` for Purpose/In-scope/Out-of-scope/Dependency-position/Exit-statement — this file adds
the operative figures and the ceiling math this package inherits from SD-30's decisions §43/§44
(re-derived 2026-08-13/14, not re-run at split time; the next cycle that opens a card must re-derive
before claiming, per this program's standing "generated, never hand-maintained" rule).

## Inherited figures at split time (2026-08-14), cited not re-derived by this file

Per `SD-30-class-feature-archetype-bundle/decisions.md §43` (per-kind `grounded`/`done` table) and
`§44`/`acceptance-and-verification.md AT-30-015` (per-kind `done` floors):

| kind | total units | done (2026-08-14) | held | done floor | closure instrument |
|---|---:|---:|---:|---:|---|
| class_feature | 15,472 | 25 (0.2%) | 88 | measured per-class via Epic 3 (this package) | Epics 3/4/5 |
| monster | 1,270 | 7 (0.6%) | 1,235 | 1,242 (97.8%, done+held) | Epic 6-F1 + SD-30 Epic 0's static/derived rung (closed 2026-08-14 — verify by content) |
| spell | 2,843 | 47 (1.7%) | 1,235 ~~stale, see correction~~ | 1,282 (45.1%) ~~stale, see correction~~ | Epic 6-F2 + SD-30 Epic 0's spell probe (verify by content what landed) |
| race | 103 | 0 (0.0%) | 7 | 103 (100%) — **gated on this package's Epic 1 race chassis** | Epic 6-F3, gated on Epic 1 per race batch |
| race_trait | 3,447 | 266 (7.7%) | 247 | 513 (14.9%, done+held) *without* chassis; up to 3,447 *with* Epic 1's chassis | Epic 6-F4, gated on Epic 1 per race batch |

**Correction, `spell` row, 2026-08-15 (launch-readiness remediation Step 5, drift D5).** The `held`
(1,235) and `done+held` floor (1,282, 45.1%) figures above are a copy-paste artifact of the `monster`
row directly above (`monster`'s own `held` is 1,235; `1,282` is `feat`'s `done+held` from a different
table entirely) — the same defect `acceptance-and-verification.md AT-31-005` already diagnosed and
corrected for its own copy of this table (`SD30-E0-F4-001`), but this file's copy was never brought
into agreement, leaving an in-package figure disagreement (a STOP condition under SD-30's "Hard
stops" doctrine). Re-derived this cycle, matching `AT-31-005` exactly:

```
python3 -c "
import json, importlib.util, collections
spec = importlib.util.spec_from_file_location('m', 'scripts/observer/pf1e_dashboard_producer.py')
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
d = json.load(open('docs/work-inventory.json'))['units']
c = collections.Counter()
for u in d:
    if u.get('book') == 'beginner_box' or u.get('kind') != 'spell': continue
    c[mod.doneness_verdict(u.get('wiring_class'), u.get('status'), 'spell')] += 1
print(dict(c))
"
# {'held': 1103, 'in-progress': 132, 'done': 47, 'not-started': 1561}
```

The true `spell` `held` is **1,103** (`done+held` = **1,150**, 40.4%), not 1,235/1,282. `retro.py
correction` emitted for this file's copy specifically (see `progress.md`'s S5-drift receipt).

**Note on `race`/`race_trait`, rewritten 2026-08-15 (`decisions.md §2`).** The chassis-blind ceiling
(513 of 3,447 `race_trait`, 0 of 103 `race`) is **not this package's ceiling** — it is the ceiling of
the ingest lanes *before* Epic 1 runs. Epic 1 (race chassis) is now the first capability epic in this
package and opens Epic 6-F3/F4 per race batch. The 553-unit workable figure is therefore a function of
Epic 1's output, re-derived after each batch, not a constant to plan the lane around. Under the prior
split this was a cross-package dependency pointing at a package scheduled *after* these lanes; that
inversion is what the merge fixed.

## Combined ceiling (instruments + ingest together), inherited from `SD-30-.../decisions.md §44`

The ceiling via instrument-application alone (SD-30 Epic 0, closed 2026-08-14) is 12,919 of 38,521
units (33.5 %) — `done` 3,464 + `held` 9,455 at the 2026-08-13 stamp. This package's ingest work
(Epics 5/6/7) closes the remaining `not-started`/`not-ingested` gap, and its capability builds
(Epics 1-2) are what make the rest reachable at all.

**The ceiling without the capability builds is 77.9 %** — re-derived 2026-08-15, `decisions.md §2`:
8,524 distinct units (22.1 % of the board) have no path to `done` until Epic 1 and Epic 2 land, of
which 2,109 (`wiring_class == ambiguous`) reach `done` from no status at all. That figure is an
independent reproduction of the ~81 % "honest ceiling" `SD-30 state-goals-and-lessons.md §2.3`
recorded before `§45` superseded it. The mandate is reachable **because this package now contains its
own prerequisites**, not because the ceiling moved.

## Binding rules this package inherits (copied, not by reference — see `decisions.md` Decision 1)

- The anti-gaming rule (`SD-30-.../decisions.md §50(a)`, originally SD-32 Decision 1).
- The table-sheet doneness doctrine (`SD-30-.../decisions.md §49`).
- The PI-gate hard-block: no ingest card in this package's Epics 5/6/7 claims a book before
  `SD-30-class-feature-archetype-bundle`'s Epic 3 (PI-screening) is `COMPLETE` for that book — SD-30
  closed with it `COMPLETE`, so the block is discharged at package level, but per-book citation and
  the production-path reader calls are still required.
- **No per-cycle deferral** (`decisions.md §3`): a unit leaves the 100 % denominator only through an
  operator-signed Structural Exclusion Register entry.
- The concurrency/cloud fan-out protocol (`SD-30-.../decisions.md §47`).

## What this file does not restate

Everything else — the SD-29 operating lessons (raw-remainder splitting, pre-cycle screening, corpus
shape traps, PI-gate discipline), the reach-gate prime rule, cross-book conflict resolution, and the
class-grant boundary with SD-28 — is inherited unchanged from `SD-30-class-feature-archetype-bundle/`
and is not re-derived here; cite the SD-30 decision directly.
