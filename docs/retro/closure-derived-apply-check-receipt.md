# Receipt — applying the `derived` evaluator-vs-fixture check

Actor `closure-derived`. Card `apply-derived-check`. Branch `tranche/9`, base `9fec7f14`.

Filed here rather than in `docs/release/SD-30-class-feature-archetype-bundle/progress.md`
because the sibling agent `sd30-refresh` holds that package concurrently. That package's
docs are untouched by this run.

## Headline

**`done` movement: 0. Not because the check failed — it passes — but because no `derived`
unit can reach `done` at all.**

The check was applied to the whole held `derived` population. 49 units now demonstrably
clear the `derived` bar. None of them can be counted `done`, and neither can any of the
other 2,830, because the live doneness verdict table has no `done` rung for `derived`.

Reporting `done` movement, never `grounded` movement, as the card requires: **+0 done,
every kind.**

## Corrections to the brief

### 1. Held `derived` is 2,879, not ~6,175

6,175 is the *total* `derived` population. It includes 2,112 `not-ingested`, 689
`unknown` and 495 `not-started` — none of which are held, and none of which this check
could ever apply to.

```
python3 scripts/derive_derived_evaluator_fixtures.py --report
#   held `derived` units in the inventory: 2879
```

Independently, over `docs/work-inventory.json`, counting `wiring_class == "derived"` and
`status in (ingested-magnitude, grounded, text-complete)`: **2,879**. The two agree.

Held `derived` by kind: monster 1,229 · spell 941 · companion 303 · monster_ability 219 ·
equipment 122 · feat 39 · class_feature 20 · equipment_modifier 5 · race_trait 1.

(For the sibling static card, by the same method: held `static` is **4,801**, not ~4,617.)

### 2. The held bucket is not "the largest cheap lever". It is not a lever at all.

The brief's central premise is that the instruments exist and applying them converts held
mass to done. For `static` and `derived` that is false, and the reason is structural.

From `_doneness_verdict_uncapped()` in the live producer
(`~/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/pf1e_dashboard_producer.py`,
lines 3381–3495):

```python
if wiring_class in ("static", "derived"):
    if status in ("ingested-magnitude", "grounded", "text-complete"):
        return DONENESS_HELD
    raise ValueError(f"doneness: unmapped {wiring_class!r} + {status!r}")
```

Enumerating `verdict("derived", status, kind)` across every status and kind in the
inventory yields exactly `{deferred, held, not-started, unmeasurable}`. **`done` is
unreachable.** Same for `static`.

The verdict function's only inputs are `(wiring_class, status, kind)`. None of the three
can carry the fact "this unit's evaluator-vs-fixture check passed". So passing the check
is *necessary but not sufficient*, and the missing half is a `done` rung in a table that
lives outside this repo.

This repo already knew. `docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py`
labels its largest bucket, 6,619 units, in its own words:

```
6619  B3  needs the missing check AND a done rung the verdict table lacks
```

The check is the first half. The second half was never in scope for this card.

### 3. A correct classifier fix cost the dashboard 26 `done` units today

The `%N` placeholder fix (`99efb504`, inventory regenerated at `8d00d0b1`) moved 975 units
`display → derived`. It is a correct fix. But `display + text-complete → done` while
`derived + text-complete → held`, so:

```
python3 docs/retro/closure-derived-doneness-delta.py 5ed6bdc0 8d00d0b1
#   5ed6bdc0  DONE=3444   display 15341  derived 5109
#   8d00d0b1  DONE=3418   display 14366  derived 6175
#   DELTA vs 5ed6bdc0: -26 done  {'equipment': -1, 'feat': -25}
```

**−26 done** (feat −25, equipment −1) with zero content regression.

The product board is therefore *anti-correlated with classifier quality*: every future
improvement that moves a unit into `static` or `derived` subtracts from the number the
operator judges this work by. That is worth more operator attention than any count in
this receipt.

## What was actually applied, and what it found

`tests/derived_evaluator_fixture_check.rs`, 5 tests, against
`tests/fixtures/rules_core/derived-evaluator-fixtures.json` (94 entries).

```
cargo test --locked --test derived_evaluator_fixture_check -- --nocapture
# EXIT=0 — test result: ok. 5 passed; 0 failed; 0 ignored
```

```
derived evaluator-vs-fixture: 49 of 94 covered units cleared the bar;
  1 did NOT and stays `held`; 44 belong to books with no ingest ({"ultimate_equipment"})
```

- **49 cleared** the `derived` bar — evaluator output equals the independently-derived
  expected value. These meet their class's bar and still cannot be counted `done`.
- **1 real defect**, `advanced_players_guide:equipment:spindle_of_perfect_knowledge`. The
  corpus row grants `BONUS:STAT|INT,WIS,CHA|4` and the player receives nothing. Not
  item-local: of APG's 338 ingested equipment records, 331 are `source.kind ==
  web_second_source` and 332 carry neither `raw_tokens` nor `raw_bonus_chains`, so
  `corpus_loader` reconstructs a thin record for nearly the whole book. This is an
  ingest-lane defect, not a check defect. Already pinned in the suite's `UNCLEARED`
  register; confirmed still failing on re-application.
- **44 unevaluatable**, all `ultimate_equipment`. Verified structural, not an excuse:
  `data/corpus/ultimate_equipment` does not exist — the book has no ingest at all.

Coverage is 94 of 2,879 held `derived` units. The 2,785 uncovered are reported by the
instrument with a reason each; the dominant reasons are kinds with no equipment-effect
evaluator (monster 1,229 · spell 941 · companion 303 · monster_ability 219) and small
token families with no numeric evaluator (`BONUS:WEAPON` 10, `BONUS:CASTERLEVEL` 5,
various `COST:` formulas). Widening coverage would clear more units against the bar and
still move `done` by zero, for the reason in correction 2.

## How fixture independence was guaranteed

Three independent mechanisms, all re-verified this run rather than taken on trust:

1. **Different tree.** Every value in the fixture is a function of the *upstream PCGen*
   `.lst` bytes (`$HOME/workspace/repos/pcgen/data`). The generator imports no engine
   module, runs no engine binary, and opens no file under `data/corpus/` — which is the
   ingest the engine actually evaluates. `docs/work-inventory.json` is read for unit
   identity and source-line provenance only; no magnitude or engine-computed value is
   copied out of it.
2. **Re-derivation by disjoint code.** `reference_bonus_stat()` in the check re-derives
   every `expected` value from the pinned `corpus_field`, sharing no code with the Python
   generator. Editing `expected.bonus` without editing `corpus_field` turns the suite red.
3. **Byte anchor.** `pinned_corpus_field_is_byte_identical_to_the_upstream_lst` and
   `engine_ingest_cites_the_same_upstream_bytes_the_fixture_was_read_from` pin the
   fixture to `upstream_lst_sha256` / `upstream_line` and cross-check that this repo's
   ingest cites the same upstream bytes. 49 entries agreed across both provenance
   recordings.

Freshly re-derived this run: `python3 scripts/derive_derived_evaluator_fixtures.py`
rewrote the fixture and `git status --porcelain` came back **empty** — the committed
fixture is byte-identical to a fresh derivation from the current inventory and the
current upstream corpus. That also confirms the fixture already covers the post-`%N`
population; the ~975 newly-`derived` units are inside the 2,879 the report counts.

The check is gated: `expected_test_suites()` in `scripts/verify.sh` is derived from the
filesystem (`find tests -maxdepth 1 -name '*.rs'`), so `root-full` must execute this
suite or the stage fails.

## The real ceiling

This instrument can never move `done` for any unit, at any coverage, at any pass rate.
Its ceiling is "proves a `derived` unit meets its bar" — and the dashboard has no cell
that records that fact. Closing the gap needs two changes made together:

1. the work-inventory generator emitting per-unit evidence that the check covered and
   cleared the unit, and
2. a `derived + <that evidence> → done` rung in the producer's verdict table.

Neither was attempted. (2) is a doneness-bucket definition change, which the card's
anti-gaming rule forbids, and the producer is outside this repo's write scope.

**Hazard for whoever does take it on:** the `static`/`derived` branch `raise`s on any
status it does not know. Emitting a new status word from the generator without landing
the producer rule in the same change does not degrade — it crashes the dashboard.

## Method validation

The verdict table transcribed here reproduces the live dashboard exactly, and reproduces
a known-truth movement independently:

```
python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py
#   re-derived: {'done': 3464, 'held': 9455, 'in-progress': 716,
#                'not-started': 21303, 'unmeasurable': 3547, 'deferred': 36}
#   dashboard : {'done': 3464, 'held': 9455, ...}
#   transcription validated against live dashboard: True
```

and across `90bd9975` the same transcription reports spell `done` 1 → 47, i.e. **+46**,
matching the brief's independently-stated figure for the spell consumer-delta probe:

```
python3 docs/retro/closure-derived-doneness-delta.py 8d00d0b1 90bd9975 HEAD
#   DELTA vs 8d00d0b1: +46 done  {'spell': 46}
#   DELTA vs 90bd9975: +0 done  (no kind moved)
```

## Per-kind `done` movement

| kind | before | after |
|---|---|---|
| every kind | unchanged | unchanged |

Total `done` 3,464 → 3,464. **+0.**
