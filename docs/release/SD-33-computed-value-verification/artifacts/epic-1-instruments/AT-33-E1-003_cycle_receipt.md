# Cycle AT-33-E1-003 — Epic 1 Instruments / AT-33-E1-003

- **Commit SHA:** (this receipt's own commit — see the pushed commit that lands this file, `scripts/probe_surface_census.py`, `scripts/tests/test_probe_surface_census.py`, and the artifact JSON together)
- **Files touched:**
  - `scripts/probe_surface_census.py` (new)
  - `scripts/tests/test_probe_surface_census.py` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/probe-surface-census.json` (new — generated artifact)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/AT-33-E1-003_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (updated)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E1-003 — the probe surface is enumerated for real
  >
  > A committed enumeration of **every corpus kind**, stating for each whether a probe exists that can verify a computed magnitude, and naming the probe. **Derived by execution, not from memory or from prior prose** (`decisions.md §7`).
  >
  > **Evidence:** `artifacts/epic-1-instruments/probe-surface-census.json` plus the command that generated it. The count of kinds with **no** probe is a bundle-level figure reported in `progress.md`, not a footnote.

## How the census was derived (by execution, per `decisions.md §7`)

1. Enumerated the live `kind` set from `docs/work-inventory.json` (not from memory):
   `jq -r '.units[].kind' docs/work-inventory.json | sort -u | wc -l` → `19`.
2. Read `src/bin/v06_work_inventory.rs`'s exhaustive `match unit.kind { ... }` verdict function
   line-by-line (this is the sole generator of `docs/work-inventory.json`, per
   `workflow-instruction.md §3`'s Epic 4 file-touch row) — confirmed exhaustive (no `_ =>`
   wildcard arm; the file's own comment at the last arm states this deliberately) and confirmed
   its 19 `Kind::*` arms match the 19 live `kind` values 1:1.
3. For each arm, traced whether it calls a dedicated probe function that changes an input and
   observes a **delta on a rendered computed snapshot**, versus an unconditional
   `not_ingested(...)` (no engine table at all), versus a `holds_key`/`_resolve` **presence
   lookup** with no delta observation.
4. Cross-checked every claim against the **live** inventory: for each of the 8 kinds claimed to
   carry a magnitude probe, confirmed by execution that at least one real unit's `evidence` field
   equals the probe's own positive-evidence string (the probe actually fired, not just exists in
   source); for the 11 no-probe kinds, confirmed by execution that zero units carry any
   `evidence` value containing the substring `probe`.
5. Ran a recursive `find` over the whole repo and over `data/corpus/` (the shallow-glob hazard
   named in `workflow-instruction.md §4`/§2.1) to check for a probe implementation this
   file-by-file read might have missed. Found one duplicate (`feat_probe_input`/
   `probe_feat_effect_wiring` also exist in `src/bin/v06_content_state_dump.rs` — a sibling
   binary, not a second probe surface) and confirmed `data/corpus/*/`'s extra `*_generic`/
   `_parity` directory names (visible only via recursive `find`, invisible to a `maxdepth 2`
   glob) are corpus storage-layout artifacts (per-key overflow buckets and Epic-2 oracle-fixture
   pairs respectively), not a 20th content kind — every file under a `*_generic` directory is
   still typed by its filename's `codex_named_unit_<kind>_...` prefix, matching one of the 19.
6. Wrote `PROBE_SURFACE` (the per-kind mapping) into `scripts/probe_surface_census.py`, with a
   `--check` mode that fails closed if a live `kind` is unmapped, if a `probe_exists: true` kind's
   probe never fires on live data, or if a `probe_exists: false` kind's live data carries
   probe-shaped evidence anyway — so a future regeneration of `docs/work-inventory.json` that adds
   or reshapes a kind cannot leave this census silently stale.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Distinct corpus `kind` values | 19 | whole inventory (49,438 units) | `jq -r '.units[].kind' docs/work-inventory.json \| sort -u \| wc -l` |
| Kinds with a probe that can verify a computed magnitude | **8** | of 19 kinds | `python3 scripts/probe_surface_census.py --check` → `kinds_with_probe=8` |
| Kinds with **no** probe that can verify a computed magnitude | **11** | of 19 kinds | `python3 scripts/probe_surface_census.py --check` → `kinds_without_probe=11` |
| — of which: no engine table at all (8) | 8 | of the 11 no-probe kinds | `python3 -c "import json;d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/probe-surface-census.json'));print(sum(1 for r in d['kinds'] if r['category']=='no_engine_table'))"` |
| — of which: engine table exists, presence-only lookup, no delta probe (3) | 3 | of the 11 no-probe kinds | same command, `category=='presence_only'` |
| Units in the 8 probe-bearing kinds | 34,246 | of 49,438 | `jq '[.units[] \| select(.kind \| IN("class","class_feature","feat","spell","equipment","equipment_modifier","race","race_trait"))] \| length' docs/work-inventory.json` |
| Units in the 11 no-probe kinds | 15,192 | of 49,438 | `jq '[.units[] \| select(.kind \| IN("monster","monster_ability","companion","ability","template","deity","power","domain","skill","language","trait"))] \| length' docs/work-inventory.json` (cross-checked: `49438 - 34246 = 15192`) |
| Per-kind unit counts (all 19) | see artifact | 49,438 | `jq -r '.units[].kind' docs/work-inventory.json \| sort \| uniq -c` |
| Per-kind live probe-fire confirmation (8 kinds, each > 0) | `class`=28, `class_feature`=26, `feat`=108, `spell`=966, `equipment`+`equipment_modifier`=605, `race`=39, `race_trait`=309 | each count is of that kind's own population | `class`/`class_feature`/`feat`/`spell`/`equipment`+`equipment_modifier`: `jq '[.units[] \| select(.kind=="<kind>" and (.evidence\|test("probe")))] \| length' docs/work-inventory.json` (their probe's positive-evidence string itself contains the substring `probe`). `race`/`race_trait`: their positive-evidence strings do not contain that substring, so matched exactly instead — `jq '[.units[] \| select(.kind=="race" and (.evidence=="race_offered_by_the_real_character_creation_roster" or .evidence=="race_offered_by_the_roster_but_no_pilot_compute_magnitude_consumer"))] \| length' docs/work-inventory.json` (and the `race_trait` analogue in `scripts/probe_surface_census.py`'s `PROBE_SURFACE["race_trait"]["positive_evidence_examples"]`). All 8 also re-verified mechanically via `python3 scripts/probe_surface_census.py --check` (exit 0). |
| Per-kind live probe-evidence absence confirmation (11 kinds, each = 0) | 0 for all 11 | of each kind's own population | `jq '[.units[] \| select(.kind=="<kind>" and (.evidence\|test("probe")))] \| length' docs/work-inventory.json` — true for all 11 no-probe kinds (a stricter test than needed for the 8-kind "no engine table" subset, whose evidence is always the single fixed `<kind>_content_has_no_engine_table` string). |
| Sum of the 19 kinds' `unit_count` in the artifact | 49,438 | matches population exactly | `python3 -c "import json;d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/probe-surface-census.json'));print(sum(r['unit_count'] for r in d['kinds']), d['population'])"` |
| Unit test suite (new) | 11 passed, 0 failed | `scripts/tests/test_probe_surface_census.py`'s own case count | `python3 -m unittest scripts.tests.test_probe_surface_census -v` |
| Unit test suite (existing, re-run for regression) | 25 passed, 0 failed | `scripts/tests/test_box_ledger.py`'s own case count | `python3 -m unittest scripts.tests.test_box_ledger -v` |
| Combined suite | 36 passed, 0 failed | both files' combined case count | `python3 -m unittest scripts.tests.test_probe_surface_census scripts.tests.test_box_ledger -v` |

**The generating command for the committed artifact:**
```
python3 scripts/probe_surface_census.py > docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/probe-surface-census.json
```

## The 19-kind census (full table — see the artifact JSON for the machine-readable form)

| Kind | Units | Probe exists (verifies computed magnitude)? | Probe named |
|---|---:|---|---|
| `class` | 185 | **yes** | `probe_class_effect_wiring` / `class_probe_ceiling_report` (`v06_work_inventory.rs:6920`/`:6961`) |
| `class_feature` | 18,043 | **yes** | `probe_class_feature_effect_wiring` (`:10783`) |
| `feat` | 2,764 | **yes** | `probe_feat_effect_wiring` (`:5624`) |
| `spell` | 2,843 | **yes** | `probe_spell_effect_wiring` (`:6567`) |
| `equipment` | 6,223 | **yes** | `probe_equipment_effect_wiring` (`:6265`) |
| `equipment_modifier` | 1,532 | **yes** | `probe_equipment_effect_wiring` (`:6265`, same arm as `equipment`) |
| `race` | 95 | **yes** | `probe_race_creation_roster` + `race_magnitude_consumer_races` (`:5889`) |
| `race_trait` | 2,561 | **yes** | `probe_race_trait_corpus` + `race_trait_magnitude_read_by_creation_chassis` (`:5914`) |
| `monster` | 1,270 | no — presence-only | none (`monster_resolve`/`holds_key` lookup, `:8725`/`:8811`) |
| `monster_ability` | 3,806 | no — presence-only | none (`holds_key` lookup, `:8738`) |
| `companion` | 1,696 | no — presence-only | none (`holds_key` lookup, `:9477`) |
| `ability` | 4,337 | no — no engine table | none (`:9553`) |
| `template` | 2,248 | no — no engine table | none (`:9548`) |
| `deity` | 459 | no — no engine table | none (`:9549`) |
| `power` | 421 | no — no engine table | none (`:9550`) |
| `domain` | 183 | no — no engine table | none (`:9551`) |
| `skill` | 149 | no — no engine table | none (`:9537`) |
| `language` | 136 | no — no engine table | none (`:9552`) |
| `trait` | 487 | no — no engine table | none (`:9554`) |
| **Total** | **49,438** | **8 yes / 11 no** | |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle builds and runs an instrument; it moves no unit's status or disposition.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

## Notes

- **Presence-only is deliberately not counted as a magnitude probe.** `monster`/`monster_ability`/
  `companion` reach `grounded` in `docs/work-inventory.json`'s own vocabulary on a `holds_key`/
  `monster_resolve` table lookup — the record exists with real fields, but no delta is observed on
  a rendered computed snapshot. `rules_core`'s own `monster_resolve()` functions
  (`src/rules_core/rules_tables/monster_chassis.rs:401`,
  `src/rules_core/rules_tables/beastiary1/mod.rs:324`,
  `src/rules_core/rules_tables/bonus_bestiary/mod.rs:76`) return a static `MonsterStatBlock` by
  key — not a value derived through a formula evaluator. Counting this as "a probe that can verify
  a computed magnitude" would be the identical over-claim `decisions.md §7` was written to name
  for `fixture-verified`/`literal-verified`. This is a judgment call worth being explicit about:
  the inventory's own status vocabulary is looser than this criterion's evidence bar, and this
  census answers the criterion's bar, not the inventory's.
- **`race` and `race_trait` are conservative, coarse probes**, confirmed from source: `race`'s
  magnitude check is race-level (does the race have *any* `pilot_compute` magnitude consumer),
  not trait-key-level; `race_trait`'s two checks are a race-level consumer check plus one
  record-level check (the exact trait the creation chassis reads). Both genuinely observe a
  computed magnitude for at least one real unit (39 and 309 units respectively, confirmed live),
  so both count as `probe_exists: true` — but the source comments they cite (`SD31-W12-
  INTEGRATE-001` et al.) are explicit that the trait-key-level question remains open for a race
  that has *some* seam. Not a defect this cycle owns; noted for whichever later cycle re-examines
  the `ingested-magnitude` populations these same arms produce.
- **`equipment_modifier` shares `equipment`'s probe and match arm** — `Kind::Equipment |
  Kind::EquipmentModifier` is one arm in the generator (`v06_work_inventory.rs:8596`); the probe
  does not distinguish the two kinds. Reported as two rows (per-kind, matching the criterion's
  "every corpus kind" bar) rather than collapsed to one, since the artifact's consumer needs the
  per-kind unit count either way.
- **Recursive-find hazard, checked and closed.** `workflow-instruction.md §4`'s named hazard
  ("a shallow glob lies here") was checked against two different shapes here: (1) whether a probe
  implementation existed outside `v06_work_inventory.rs` (recursive `find . -iname "*probe*"` and
  `grep -rn "fn .*probe" --include="*.rs" src/` — found one duplicate in a sibling binary,
  `v06_content_state_dump.rs`, not a new surface); (2) whether `data/corpus/`'s on-disk layout
  hides a 20th kind a `maxdepth 2` glob would miss (`find data/corpus -mindepth 2 -maxdepth 2
  -type d` surfaced `*_generic`/`_parity` directory names not in the inventory's 19-kind list;
  recursive inspection of their contents showed these are storage-layout artifacts — per-key
  overflow buckets still filename-typed to one of the 19 kinds, and Epic-2 oracle round-trip
  fixture pairs — not a missed kind).
- **`--check`'s claim-integrity gate is itself proven capable of failing**, per the same standing
  principle `AT-33-E1-002`'s receipt names ("a tool that has never been observed to fail is not a
  gate") — three RED cases below exercise it directly, on top of the live-corpus GREEN.

## RED → GREEN evidence

**TDD RED** (before `scripts/probe_surface_census.py` existed):
```
$ python3 -m unittest scripts.tests.test_probe_surface_census -v
ImportError: Failed to import test module: test_probe_surface_census
...
ModuleNotFoundError: No module named 'probe_surface_census'
Ran 1 test in 0.000s
FAILED (errors=1)
```
Failed for the intended reason — the module under test did not exist yet.

**GREEN** (after implementation):
```
$ python3 -m unittest scripts.tests.test_probe_surface_census -v
...
Ran 11 tests in 1.718s
OK
```
All 11 pass, including three live-corpus acceptance tests
(`test_live_corpus_kind_count_is_19`, `test_live_corpus_census_checks_clean`,
`test_live_corpus_eight_kinds_carry_a_magnitude_probe`) run against the real, committed
`docs/work-inventory.json` — the execution-derived evidence `decisions.md §7` requires.

**Mutation proofs on top of the passing suite** (the `--check` fail-closed gate, exercised
directly against synthetic fixtures inside the test file, not narrated):
1. `test_check_fails_closed_on_an_unmapped_kind` — a unit of a kind not in `PROBE_SURFACE` is
   correctly detected and reported by name.
2. `test_check_fails_closed_when_a_claimed_probe_never_fires` — a `class` population whose only
   unit carries evidence *other than* the probe's positive-evidence string is correctly detected
   as a false `probe_exists: true` claim.
3. `test_check_fails_closed_when_a_no_probe_kind_shows_probe_evidence` — a `monster` unit carrying
   probe-shaped evidence (which never happens on the real live data, confirmed above) is correctly
   detected as an under-claim.

Regression: `python3 -m unittest scripts.tests.test_box_ledger -v` → 25/25 still green, unchanged
(`box_ledger.py` was not touched this cycle).

## Test scoping

- **Ran:** `python3 -m unittest scripts.tests.test_probe_surface_census -v` (11/11, new),
  `python3 -m unittest scripts.tests.test_box_ledger -v` (25/25, regression check — this cycle
  touched no file `test_box_ledger.py` exercises, run anyway since both live under `scripts/tests`).
  Combined: `python3 -m unittest scripts.tests.test_probe_surface_census scripts.tests.test_box_ledger -v`
  → 36/36.
- **Did NOT run:** `scripts/verify.sh` (any stage) — `AT-33-E1-004` owns wiring the
  `denominator-gate` stage into it; this cycle's files are not yet referenced by any `verify.sh`
  stage. The Rust workspace (`cargo build`/`cargo test`) — no `.rs` file was touched this cycle.
  `apps/desktop/src-tauri` — a separate cargo workspace per `AGENTS.md`; no file in it was touched
  or is affected by this change.

## Next-cycle plan

`AT-33-E1-004` (row 4, `denominator-gate`) is next in Epic 1's sequential pipeline. It should wire
a real `scripts/verify.sh --only denominator-gate` stage; this cycle's artifact
(`probe-surface-census.json`) already carries every percentage-shaped claim with its denominator
stated in the same construct (the "8 of 19" / "11 of 19" figures above), so it is a clean input for
that stage's own acceptance check once it exists, but does not itself implement it.
