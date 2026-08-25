# Cycle AT-33-E5-001 — Epic 5 Re-verification / AT-33-E5-001

- **Commit SHA:** (recorded in a follow-up commit to this same file, per this bundle's own
  precedent — `AT-33-E1-004_cycle_receipt.md`'s "self-correction commit SHA" pattern — since
  the schema asks this receipt to name the commit that carries it)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/README.md` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-001_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment.oracle-export.txt` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment.ours.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment.oracle-results.json` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/e5-equip-stats.txt.ftl` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/equipment-pcg/*.pcg` (new, 11 files)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/equipment-oracle-txt/*.txt` (new, 11 files)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/build-transcript-equipment-headband_cha_2-SUCCESS.log` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/build-transcript-equipment-ALL-11-tails.txt` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/ours-derivation/equipment-ours-probe.rs` (new — reference copy; the program itself runs outside the repo, see Notes)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/ours-derivation/equipment-ours-probe.Cargo.toml` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/ours-derivation/equipment-ours-probe.output.json` (new)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (updated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (see command output below)
- **Wired-integration audit result:** OK_NO_TOKENS (see command output below)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.

## What landed

Extended `AT-33-E2-004`'s proven Path A mechanism from Epic 2's one hand-authored fighter
character to a real, population-scoped re-verification batch, and produced a full, honest
partition of the 1,741 `fixture-verified` population by what was actually examined this
cycle versus what was not, and why. Full narrative and per-item methodology:
`README.md` (this directory).

**Live oracle round-trip actually executed and committed: 11 of 1,741** — the entire
`equipment` kind. Per-unit `(ours, oracle, verdict)` rows (source:
`equipment.oracle-results.json`, produced by `scripts/oracle_harness/run.py`, `AT-33-E2-003`'s
CLI, unmodified):

| unit_id | ours | oracle | verdict |
|---|---:|---:|---|
| `ultimate_equipment:equipment:belt_of_mighty_hurling_greater` | 20 | 20 | agree |
| `ultimate_equipment:equipment:belt_of_mighty_hurling_lesser` | 18 | 18 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_cha_2` | 10 | 10 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_cha_4` | 12 | 12 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_cha_6` | 14 | 14 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_int_2` | 12 | 12 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_int_4` | 14 | 14 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_int_6` | 16 | 16 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_wis_2` | 12 | 12 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_wis_4` | 14 | 14 | agree |
| `ultimate_equipment:equipment:shifter_s_headband_wis_6` | 16 | 16 | agree |

`ours` is the real, live output of `codex::rules_core::equipment_effects::compute_equipment_effects`
(the exact function `AT-33-E1-003`'s `probe_equipment_effect_wiring` calls to establish these
units are engine-wired) against the real `data/corpus/ultimate_equipment/equipment/` records —
not read from the corpus's `raw_bonus_chains` field directly, which would only check ingestion,
not computation (`README.md` explains why that distinction matters here). `oracle` is a real
PCGen `BatchExporter` export — 11 separate `.pcg` characters (one Level-1 Human Fighter each,
differing only in which single item is `EQUIPSET`-equipped into its real PCGen slot, `Belt` or
`Headband`), each run through `./gradlew run` for real. **All 11 gradle invocations exited 0**
(`build-transcript-equipment-ALL-11-tails.txt`).

## Not folded into a false 100%: the remaining 1,730

`README.md`'s table gives the full breakdown. Summary: 1,303 of 1,741 (`spell` 1,288 +
`class_feature` 15) carry a real magnitude probe (`AT-33-E1-003`) and are genuinely
re-verifiable but not yet attempted this cycle — real per-unit/per-batch `.pcg` and template
authoring cost, the exact sizing question `AT-33-E2-004`'s receipt named as Epic 5's own scope.
427 of 1,741 (`companion` 187 + `monster` 140 + `monster_ability` 100) carry **no** magnitude
probe at all (`AT-33-E1-003`, `probe_exists: false`, `category: presence_only`) — a
pre-existing, already-established structural gap this cycle did not create and cannot close by
itself (Epic 1's probe surface would need to widen first). Neither group is silently counted as
"examined," "agree," or "unverifiable" — both are named with their own real reason.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| `fixture-verified` population | 1,741 | of 49,438 total inventory units | `jq '[.units[]\|select(.status=="fixture-verified")]\|length' docs/work-inventory.json` |
| `fixture-verified` population by kind | spell 1,288 / companion 187 / monster 140 / monster_ability 100 / class_feature 15 / equipment 11 | of 1,741 | `jq -r '[.units[]\|select(.status=="fixture-verified")]\|group_by(.kind)\|map({kind:.[0].kind,count:length})' docs/work-inventory.json` |
| Units examined against a real, live oracle round-trip this cycle | 11 | of 1,741 (0.63%) | this receipt's per-unit table; source `equipment.oracle-results.json` |
| Agreement among units examined this cycle | 11 | of 11 examined | `python3 scripts/oracle_harness/run.py --oracle-export .../equipment.oracle-export.txt --ours .../equipment.ours.json --output <out>.json` → `agree=11 disagree=0 unverifiable=0` |
| Disagreement among units examined this cycle | 0 | of 11 examined | same command |
| `box_ledger.py --check` against this cycle's real oracle-results | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0 | population 49,438 (whole inventory, unchanged by this cycle) | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment.oracle-results.json` |
| Not yet examined, real magnitude probe exists (`spell`+`class_feature`) | 1,303 | of 1,741 | `jq -r '[.units[]\|select(.status=="fixture-verified" and (.kind=="spell" or .kind=="class_feature"))]\|length' docs/work-inventory.json` |
| Not yet examined, no magnitude probe exists at all (`companion`+`monster`+`monster_ability`) | 427 | of 1,741 | `jq -r '[.units[]\|select(.status=="fixture-verified" and (.kind=="companion" or .kind=="monster" or .kind=="monster_ability"))]\|length' docs/work-inventory.json` |
| Probe existence per kind, cross-checked against `AT-33-E1-003`'s committed census | spell/class_feature/equipment `probe_exists: true`; companion/monster/monster_ability `probe_exists: false` | of the 6 kinds present in the fixture-verified population | `python3 -c "import json;d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/probe-surface-census.json'));[print(x['kind'],x['probe_exists']) for x in d['kinds'] if x['kind'] in ('spell','class_feature','equipment','companion','monster','monster_ability')]"` |

## Status: in-progress

**Not marked `complete`.** The criterion's Evidence line asks for the 1,741 population's
per-unit rows; this cycle delivers 11 real ones plus a fully honest, execution-derived account
of the other 1,730 and a concrete continuation plan — genuine progress on a population that
`AT-33-E2-004`'s own receipt already named as too large to size in one Epic-2 spike cycle. Per
`workflow-instruction.md` §8 / `AGENTS.md`'s blocker-closure doctrine: a blocker bigger than one
cycle is a sequencing problem, not an exemption. This is decomposition and execution of the
first slice, not an escalation — no `## Open blockers` entry is filed, and the bundle is **not**
paused. Marking this row `complete` on 11 of 1,741 would be exactly the false-100% shape
`decisions.md` §2 and `AGENTS.md` rule 2 ("no fake completion") both exist to prevent.

## Movement, four buckets

- **closure:** 0 — no unit's `status` field changed; `fixture-verified` units stay
  `fixture-verified` (oracle-pending → oracle-confirmed-agree is recorded in
  `equipment.oracle-results.json`, not as a `docs/work-inventory.json` status transition —
  `THE-BOX.md`'s group boundaries are unchanged by this cycle).
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0 — this cycle used Epic 2's harness and Epic 1's probe census
  exactly as built; it found no defect in either instrument.

## RED→GREEN

This criterion is population coverage, not a new code path, so RED→GREEN takes the form Epic
2's own receipts used for its live-tool proof: **before** this cycle, `AT-33-E5-001`'s evidence
obligation (per-unit rows against the real 1,741) had zero real rows — `equipment.oracle-results.json`
did not exist, and no `.pcg`/template pair for any `fixture-verified` unit had been authored.
**After:** 11 real per-unit rows, each backed by an independently-derived live PCGen export
(11/11 `./gradlew run` invocations exit 0, `build-transcript-equipment-ALL-11-tails.txt`) and a
live engine call (`equipment-ours-probe.output.json`, `cargo run --release` against the real
`codex` crate as a path dependency, exit 0) — `scripts/oracle_harness/run.py` then compared them
for real: `agree=11 disagree=0 unverifiable=0`, verified independently by
`scripts/box_ledger.py --check --oracle-results ...` (`AT-33-E1-002`'s condition-3 gate) exiting
`0` on the same file.

## Notes

- **Fixture discipline** (`AT-33-E2-003`, `stc-authoring`): the `oracle` value for every one of
  the 11 units comes from a live PCGen export this cycle ran for real — not transcribed by hand,
  not read from a prior fixture. The `ours` value comes from a live call into the real
  `codex::rules_core::equipment_effects` engine (via a scratch Rust program with `codex` as a
  Cargo path dependency, run from outside the repo tree — this criterion's granted write scope is
  `artifacts/epic-5-reverification/` plus Epic 2's harness plus an append-only `THE-BOX.md`, not
  `src/`; the scratch program is committed here only as a reference copy of what produced
  `equipment-ours-probe.output.json`, and it never wrote into the codex repo). Neither side is a
  mirror of the other's read path.
- **Why `equipment` first:** the smallest kind (11 of 1,741) with a resolved magnitude that maps
  directly onto an existing, simple oracle export token (`STAT.<n>.SCORE`) — no character-level,
  class, or spell-list dependency, unlike `class_feature` (needs high-level, class-specific
  characters) or `spell` (needs spellbook population). Chosen for tractability inside one cycle's
  budget, not because it is the largest or most representative slice — `README.md`'s next-cycle
  plan names both remaining sub-populations' real cost drivers.
- **`equipment_id_resolve`'s `RuleSetId::Crb` argument** (read while building the probe) did not
  block resolution of `ultimate_equipment` records — confirmed empirically (all 11 resolved and
  produced a non-`None` `ability_bonus`), consistent with `AT-33-E1-003`'s own finding that this
  probe already observes `ultimate_equipment` as wired.
- Considered writing `ours` directly from the corpus's `raw_bonus_chains` field (the number is
  literally present in `data/corpus/ultimate_equipment/equipment/*.json`) — rejected: that would
  check only that ingestion parsed the LST literal, the same thing `literal-verified` status
  already certifies for a different 6,589-unit population, not that the *fixture-verified*
  disposition's actual computation is correct. Used the real `compute_equipment_effects` call
  instead so this cycle checks the same thing the criterion asks about.

## Test scoping

Ran `scripts/oracle_harness/run.py` (Epic 2's own tool, unmodified — no test changes needed) and
`python3 scripts/box_ledger.py --check --oracle-results equipment.oracle-results.json`, both
against this cycle's real output. Did not re-run `scripts/tests/test_oracle_harness.py` or
`scripts/tests/test_box_ledger.py` (neither file changed this cycle — `AT-33-E2-003`/`AT-33-E1-002`
own those suites; confirmed unmodified via `git status --porcelain` before this cycle's first
write). Did not run the Rust workspace's own `cargo test`/`cargo build` (no `src/` file changed —
the scratch `equip_probe` crate lives outside this repo and depends on `codex` read-only as a
path dependency; it compiled and ran clean, `cargo run --release` exit 0, warnings only,
pre-existing and unrelated to this cycle: `unused imports`/`dead_code` in
`rules_tables/{ultimate_magic,ultimate_wilderness}/monster_data.rs` and `pilot_compute/mod.rs`).
Did not run `apps/desktop/src-tauri` (separate cargo workspace, no file in it touched).

## Next-cycle plan

See `README.md`'s "Next-cycle plan" section: (1) `class_feature` (15 units) — one L20 `.pcg` per
source class, mostly free via class progression, feature-specific export tokens to be read from
`outputsheets/base.xml.ftl` first; (2) `spell` (1,288 units) — batch via `SPELLMEM.*` generic
template iteration against one or a few high-level prepared-caster `.pcg`s; (3)
`companion`/`monster`/`monster_ability` (427 units) — not an Epic 5 task; needs Epic 1's probe
surface to widen first, named here so it is not silently re-scoped into Epic 5 or folded into a
false "examined" count. `AT-33-E5-002` (the 6,589 `literal-verified` units) is a separate
criterion with its own population and is not started by this cycle.
