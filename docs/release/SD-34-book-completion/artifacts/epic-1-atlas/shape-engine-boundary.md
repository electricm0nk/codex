# The shape-engine boundary

A committed statement, proven by execution, of what a shape engine does and where its output
stops -- so no future bundle re-learns it (SD-34 `AT-34-E1-004`).

Re-derive: `python3 scripts/shape_engine_boundary.py --check`

## The fact

**A shape engine turns a formula string into a number.** `formula_interpreter` covers F1..F9
(`technical-design.md §3`): population 11,652, recognised 10,626, refused 240, unjoined 786
(`content-unit-inventory.md`). It refuses rather than guesses:

```
"var(\"CL=Arcanist\")" -> unrecognised function "var" -- refusing rather than guessing its semantics
```

**It does not place the record, attach it, or display it.** Those are separate, later steps
gated by the engine's own promotion ladder -- the real authority, quoted below with its line
number re-verified at HEAD, not assumed:

```rust
                if has_real_description
                    && is_display_wiring_class_for_promotion(wc_class)
                    && !universal_sheet_modifier
                    && facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)
```

(`src/bin/v06_work_inventory.rs:10857` -- re-checked by content,
not just path/line, on every run of this instrument.)

None of the four conditions is "a value was computed". Fail the last one and the verdict is
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` -- a unit the shape engine
may already compute a correct number for, still refused promotion because no table holds the
record it would attach to.

## The measured consequence

- **26396** units in `docs/work-inventory.json` carry at least one
  magnitude token (`magnitude_token_count > 0`) -- re-derive:
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(sum(1 for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0))"`
  (denominator: 26396 of the corpus's full unit population, printed by
  `scripts/completion_atlas.py --check`)
- Of those **26396**, **9475** are still not
  held by the engine (`status == engine-does-not-hold`) -- re-derive:
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); m=[u for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0]; print(sum(1 for u in m if u.get('status') == 'engine-does-not-hold'))"`
  (denominator: 26396 magnitude-bearing units, computed immediately
  above)

**Roughly a third of the shape engine's own feedstock is still stuck downstream of it** (this
fraction moved from just over half, 13119/26396, at Epic 1's original AT-34-E1-004 cycle
to 9475/26396 here, as Epic 3's per-bucket
work closed real units -- see `decisions.md §12` L10: a count that drops from measurement
work is closure, not a re-measurement artifact). This is exactly the gap Epic 2's tables and
Epics 3-4's per-bucket work close -- the engine already works; the boundary is where its
output goes next.

## Why this is a fact, not an assumption

Both counts above and the citation are re-derived by
`python3 scripts/shape_engine_boundary.py --check` on every invocation, against the live
`docs/work-inventory.json` and the live `src/bin/v06_work_inventory.rs` -- never quoted from an
earlier document (`decisions.md §12` L2). The instrument fails closed (non-zero exit, no
artifact written) if the citation's line numbers stop containing the exact conditions quoted
above, so a refactor that moves this code cannot leave a stale "fact" behind
(`risks-and-open-questions.md §10`).
