# Cycle AT-33-E5-002 — Epic 5 Re-verification / AT-33-E5-002

- **Commit SHA:** (recorded after commit — see progress.md/kanban.md entries for the final SHA;
  this receipt is committed in the same commit as the artifacts it describes)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/README.md` (extended — new `AT-33-E5-002` section appended after `AT-33-E5-001`'s)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-002_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.oracle-export.txt` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.ours.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.oracle-results.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/equipment-literal-pcg/*.pcg` (new, 21 files)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/equipment-literal-oracle-txt/*.txt` (new, 21 files)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/build-transcript-equipment-literal-ALL-21-tails.txt` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/build-transcript-equipment-literal-gorgon_belt-SUCCESS.log` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/ours-derivation/equipment-literal-ours-probe.rs` (new — reference copy; the program itself runs outside the repo, see Notes)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/ours-derivation/equipment-literal-ours-probe.Cargo.toml` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/ours-derivation/equipment-literal-ours-probe.output.json` (new)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (updated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (re-run on final diff, see Test scoping)
- **Wired-integration audit result:** OK_NO_TOKENS (re-run on final diff, see Test scoping)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** as above [per-unit `(ours, oracle, verdict)` rows committed; agreement and
  > disagreement counts both stated, with the denominator].

## What landed

Extended the exact mechanism `AT-33-E5-001` proved for the `fixture-verified` population — Epic
2's proven Path A (`AT-33-E2-004`) plus the reusable `e5-equip-stats.txt.ftl` template and
`Belt`/`Headband` `.pcg` slot convention — to a real, population-scoped re-verification slice of
the **separate, non-overlapping** 6,589-unit `literal-verified` population. Full narrative and
per-item methodology: `README.md` (this directory), section "AT-33-E5-002".

**Live oracle round-trip actually executed and committed: 21 of 6,589** — every `literal-verified`
`equipment` record under `ultimate_equipment` carrying a single-ability `STAT|<ability>|<n>|TYPE=Enhancement`
qualifier and a `Belt`/`Headband` slot (the same BONUS shape and `.pcg`/template mechanism
`AT-33-E5-001` already proved, re-used here unmodified except for the item list). Per-unit
`(ours, oracle, verdict)` rows (source: `equipment-literal.oracle-results.json`, produced by
`scripts/oracle_harness/run.py`, `AT-33-E2-003`'s CLI, unmodified):

| unit_id | ours | oracle | verdict |
|---|---:|---:|---|
| `ultimate_equipment:equipment:anaconda_s_coils` | 18 | 18 | agree |
| `ultimate_equipment:equipment:belt_of_thunderous_charging` | 18 | 18 | agree |
| `ultimate_equipment:equipment:belt_of_the_weasel` | 16 | 16 | agree |
| `ultimate_equipment:equipment:cord_of_stubborn_resolve` | 16 | 16 | agree |
| `ultimate_equipment:equipment:elemental_earth_belt` | 18 | 18 | agree |
| `ultimate_equipment:equipment:gorgon_belt` | 20 | 20 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_cha_2` | 10 | 10 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_cha_4` | 12 | 12 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_cha_6` | 14 | 14 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_int_2` | 12 | 12 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_int_4` | 14 | 14 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_int_6` | 16 | 16 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_wis_2` | 12 | 12 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_wis_4` | 14 | 14 | agree |
| `ultimate_equipment:equipment:headband_of_aerial_agility_wis_6` | 16 | 16 | agree |
| `ultimate_equipment:equipment:headband_of_ponderous_recollection` | 12 | 12 | agree |
| `ultimate_equipment:equipment:headband_of_unshakeable_resolve` | 12 | 12 | agree |
| `ultimate_equipment:equipment:minotaur_belt` | 18 | 18 | agree |
| `ultimate_equipment:equipment:monkey_belt` | 16 | 16 | agree |
| `ultimate_equipment:equipment:plague_rat_belt` | 16 | 16 | agree |
| `ultimate_equipment:equipment:shadowform_belt` | 20 | 20 | agree |

`ours` is the real, live output of `codex::rules_core::equipment_effects::compute_equipment_effects`
against the real `data/corpus/ultimate_equipment/equipment/` records via a scratch Rust program
(`ours-derivation/equipment-literal-ours-probe.rs`, outside the codex repo tree — this criterion's
granted write scope is `artifacts/epic-5-reverification/` plus Epic 2's harness plus an
append-only `THE-BOX.md`, not `src/`). `oracle` is a real PCGen `BatchExporter` export — 21
separate `.pcg` characters (one Level-1 Human Fighter each, differing only in which single item is
`EQUIPSET`-equipped into its real PCGen slot, `Belt` or `Headband`), each run through
`./gradlew run` for real against the pinned checkout
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, the same jar `AT-33-E5-001` built,
re-used unmodified — no rebuild needed). **All 21 gradle invocations exited 0**
(`build-transcript-equipment-literal-ALL-21-tails.txt`).

## Not folded into a false 100%: the remaining 6,568

`README.md`'s "AT-33-E5-002" section gives the full breakdown. Summary: 5,478 of 6,589
(`equipment` remainder 5,149 + `spell` 217 + `equipment_modifier` 46 + `race` 36 +
`class_feature` 17 + `race_trait` 13) carry a real magnitude probe (`AT-33-E1-003`) and are
genuinely re-verifiable but not yet attempted this cycle — real per-shape `.pcg`/template
authoring cost. 1,090 of 6,589 (`monster` 843 + `monster_ability` 148 + `companion` 99) carry
**no** magnitude probe at all (`AT-33-E1-003`, `probe_exists: false`, `category: presence_only`)
— the same pre-existing, already-established structural gap `AT-33-E5-001` named for its own
population, not created by this cycle. Neither group is silently counted as "examined," "agree,"
or "unverifiable."

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| `literal-verified` population | 6,589 | of 49,438 total inventory units | `jq '[.units[]\|select(.status=="literal-verified")]\|length' docs/work-inventory.json` |
| `literal-verified` population by kind | equipment 5,170 / monster 843 / monster_ability 148 / spell 217 / companion 99 / equipment_modifier 46 / race 36 / class_feature 17 / race_trait 13 | of 6,589 | `jq -r '[.units[]\|select(.status=="literal-verified")]\|group_by(.kind)\|map({kind:.[0].kind,count:length})' docs/work-inventory.json` |
| Units examined against a real, live oracle round-trip this cycle | 21 | of 6,589 (0.32%) | this receipt's per-unit table; source `equipment-literal.oracle-results.json` |
| Agreement among units examined this cycle | 21 | of 21 examined | `python3 scripts/oracle_harness/run.py --oracle-export docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.oracle-export.txt --ours docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.ours.json --output <out>.json` → `agree=21 disagree=0 unverifiable=0` |
| Disagreement among units examined this cycle | 0 | of 21 examined | same command |
| `box_ledger.py --check` against this cycle's real oracle-results | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0 | population 49,438 (whole inventory, unchanged by this cycle) | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-literal.oracle-results.json` |
| Not yet examined, real magnitude probe exists | 5,478 | of 6,589 | `jq -r '[.units[]\|select(.status=="literal-verified" and (.kind=="spell" or .kind=="equipment_modifier" or .kind=="race" or .kind=="class_feature" or .kind=="race_trait"))]\|length' docs/work-inventory.json` → 329, plus `equipment` remainder 5,170 − 21 = 5,149; 329 + 5,149 = 5,478 |
| Not yet examined, no magnitude probe exists at all | 1,090 | of 6,589 | `jq -r '[.units[]\|select(.status=="literal-verified" and (.kind=="companion" or .kind=="monster" or .kind=="monster_ability"))]\|length' docs/work-inventory.json` |
| `literal-verified` ∩ `fixture-verified` overlap | 0 | of 6,589 and 1,741 (both populations) | `THE-BOX.md`'s partition, `python3 scripts/box_ledger.py --check` → `overlap=0` (unchanged by this cycle — both are pre-existing, disjoint `status` groups) |
| Probe existence per kind, cross-checked against `AT-33-E1-003`'s committed census | equipment/spell/equipment_modifier/race/class_feature/race_trait `probe_exists: true`; companion/monster/monster_ability `probe_exists: false` | of the 9 kinds present in the literal-verified population | `python3 -c "import json;d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/probe-surface-census.json'));[print(x['kind'],x['probe_exists']) for x in d['kinds'] if x['kind'] in ('equipment','monster','monster_ability','spell','companion','equipment_modifier','race','class_feature','race_trait')]"` |

## Status: in-progress

**Not marked `complete`.** The criterion's Evidence line ("as above" — `AT-33-E5-001`'s bar) asks
for the 6,589 population's per-unit rows; this cycle delivers 21 real ones plus a fully honest,
execution-derived account of the other 6,568 and a concrete continuation plan, matching the exact
disposition `AT-33-E5-001` (the sibling criterion, same epic, same evidence shape, same
1,741-unit-scale population) used for its own 11-of-1,741 slice. Per `workflow-instruction.md` §8
/ `AGENTS.md`'s blocker-closure doctrine: a blocker bigger than one cycle is a sequencing problem,
not an exemption. This is decomposition and execution of a first slice, not an escalation — no
`## Open blockers` entry is filed, and the bundle is **not** paused. Marking this row `complete`
on 21 of 6,589 would be exactly the false-100% shape `decisions.md` §2 and `AGENTS.md` rule 2
("no fake completion") both exist to prevent — the same reasoning `AT-33-E5-001`'s receipt
states verbatim for its own population.

**Note on the dispatch template's step 8** (`workflow-instruction.md §6`, generic across all
criteria): "mark the kanban.md row complete" is the template's default action for a cycle that
fully satisfies its criterion. This cycle does not; kanban row 17 is marked `in-progress` with a
pointer to this receipt's real figures, mirroring row 16's disposition, per `AGENTS.md`'s "no
fake completion" rule and `decisions.md §2`'s denominator discipline — both of which override a
template default when they conflict.

## Movement, four buckets

- **closure:** 0 — no unit's `status` field changed; `literal-verified` units stay
  `literal-verified` (oracle-pending → oracle-confirmed-agree is recorded in
  `equipment-literal.oracle-results.json`, not as a `docs/work-inventory.json` status transition —
  `THE-BOX.md`'s group boundaries are unchanged by this cycle, matching `AT-33-E5-001`).
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0 — this cycle used Epic 2's harness, Epic 1's probe census, and
  `AT-33-E5-001`'s own template/`.pcg`-slot convention exactly as built; it found no defect in any
  of them.

## RED→GREEN

This criterion is population coverage, not a new code path, so RED→GREEN takes the same
live-tool-proof form `AT-33-E2-004`'s and `AT-33-E5-001`'s receipts used: **before** this cycle,
`AT-33-E5-002`'s evidence obligation (per-unit rows against the real 6,589) had zero real rows —
`equipment-literal.oracle-results.json` did not exist, and no `.pcg`/template pair for any
`literal-verified` unit had been authored. **After:** 21 real per-unit rows, each backed by an
independently-derived live PCGen export (21/21 `./gradlew run` invocations exit 0,
`build-transcript-equipment-literal-ALL-21-tails.txt`) and a live engine call
(`equipment-literal-ours-probe.output.json`, `cargo run --release` against the real `codex` crate
as a path dependency, exit 0) — `scripts/oracle_harness/run.py` then compared them for real:
`agree=21 disagree=0 unverifiable=0`, verified independently by
`scripts/box_ledger.py --check --oracle-results ...` (`AT-33-E1-002`'s condition-3 gate) exiting
`0` on the same file.

## Notes

- **Fixture discipline** (`AT-33-E2-003`, `stc-authoring`): the `oracle` value for every one of
  the 21 units comes from a live PCGen export this cycle ran for real — not transcribed by hand,
  not read from a prior fixture. The `ours` value comes from a live call into the real
  `codex::rules_core::equipment_effects` engine (a scratch Rust program with `codex` as a Cargo
  path dependency, run from outside the repo tree; it never wrote into the codex repo). Neither
  side is a mirror of the other's read path.
- **Reused, not rebuilt, from `AT-33-E5-001`:** the pinned PCGen checkout (already built and
  jarred — no `./gradlew compileJava`/`jar` re-run needed, only `./gradlew run`), the
  `e5-equip-stats.txt.ftl` template (unmodified), and the `Belt`/`Headband` `.pcg` slot
  convention. This is why 21 units, nearly double `AT-33-E5-001`'s 11, were tractable in this
  cycle: the mechanism-proving cost was already paid by the prior cycle, and the shared scratchpad
  directory (same UUID across this dispatch session's agent invocations) still held the built jar.
- **Why this exact 21-item slice:** the largest same-shape (`STAT|<single ability>|<n>|Enhancement`,
  `Belt`/`Headband`) subset of the `literal-verified` `equipment` kind reachable by re-using
  `AT-33-E5-001`'s proven template without modification — not the largest or most representative
  slice of the full 6,589. `README.md`'s next-cycle plan names the real cost drivers for the
  remaining sub-populations, including the 8 same-shape items excluded here (5 multi-ability, 2
  different-slot, 1 different-book) and the much larger set of `equipment` units carrying other
  magnitude shapes (weapon/armor bonuses, resistance, charges) that need new export tokens.
- Considered widening the template to sum multi-ability `STAT` chains in this same cycle —
  deferred to the next cycle rather than attempted under time pressure inside this one, so the
  21-item result set stays uniformly single-ability and easy to audit by inspection (each row's
  `ours_total == base + declared_mag`, visible in `/tmp/e5_literal_items.json`'s intermediate
  derivation, cross-checked independently against the live probe output before the harness ran).

## Test scoping

Ran `scripts/oracle_harness/run.py` (Epic 2's own tool, unmodified — no test changes needed) and
`python3 scripts/box_ledger.py --check --oracle-results equipment-literal.oracle-results.json`,
both against this cycle's real output. Did not re-run `scripts/tests/test_oracle_harness.py` or
`scripts/tests/test_box_ledger.py` (neither file changed this cycle — confirmed unmodified via
`git status --porcelain` before this cycle's first write). Did not run the Rust workspace's own
`cargo test`/`cargo build` (no `src/` file changed — the scratch `equip_probe` crate lives outside
this repo and depends on `codex` read-only as a path dependency; it compiled and ran clean,
`cargo run --release` exit 0, warnings only, pre-existing and unrelated to this cycle — the same
`unused imports`/`dead_code` warnings `AT-33-E5-001`'s receipt already recorded). Did not run
`apps/desktop/src-tauri` (separate cargo workspace, no file in it touched).

Re-ran both `workflow-instruction.md §6` step 2/4 audits (the identifier-tag grep and the
wired-integration-token grep, exact patterns as specified there, unmodified) against
`BASE_BRANCH=$(git merge-base HEAD origin/develop)` on the final diff, scoped to this criterion's
touched paths (`artifacts/epic-5-reverification/`, `progress.md`, `kanban.md`) under
`docs/release/SD-33-computed-value-verification/`.

Identifier-tag audit → `OK_NO_BUNDLE_TAGS` (`sd-33`/`SD-33` bundle-id text in prose/paths is the
bundle's own canonical directory name and criterion-ID convention — a hyphenated form, not the
underscore-joined generated-fixture-tag shape the pattern targets).

Wired-integration-token audit → `OK_NO_TOKENS`.

## Next-cycle plan

See `README.md`'s "AT-33-E5-002" → "Next-cycle plan" section: (1) the 8 remaining same-shape
`equipment` candidates (5 multi-ability, 2 different-slot, 1 different-book); (2) the bulk of the
`equipment` remainder (5,149) via other magnitude shapes read from `outputsheets/base.xml.ftl`;
(3) `spell` (217) / `class_feature` (17) converged with `AT-33-E5-001`'s own plan for its
overlapping-shape units; (4) `equipment_modifier` (46) / `race` (36) / `race_trait` (13), each
needing a new `.pcg` authoring pattern; (5) `monster`/`monster_ability`/`companion` (1,090) — not
an Epic 5 task, needs Epic 1's probe surface to widen first. `AT-33-E5-003` (disagreement
resolution) has nothing to act on yet from either `AT-33-E5-001` or `AT-33-E5-002` — both cycles'
examined units are 100% `agree`, 0% `disagree`.
