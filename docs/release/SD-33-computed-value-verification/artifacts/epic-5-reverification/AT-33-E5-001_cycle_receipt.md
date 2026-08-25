# Cycle AT-33-E5-001 (remediation) — Epic 5 Re-verification / AT-33-E5-001

- **Commit SHA:** `73fdbb8803` (landed on `tranche/13`; this line updated in a follow-up commit per this bundle's own precedent)
- **Files touched:**
  - `src/bin/fixture_verified_oracle_probe.rs` (new) — repo-local batch "ours" probe binary. Reads `docs/work-inventory.json`, filters `fixture-verified`, and for every `spell` unit calls the REAL `codex::rules_core::spellbook::compute_spellbook_coverage` (the same library function `probe_spell_key` in `v06_work_inventory.rs` calls) against the real per-book corpus — never a hand-derived formula standing in for the engine. Also enumerates the 427 `companion`/`monster`/`monster_ability` units with no magnitude probe (`AT-33-E1-003`) as `unverifiable`, per-unit, with their real reason.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-generate-spell-batch.py` (new) — batches every DC-probe-eligible spell unit into one `.pcg` per casting class (6 files: Wizard/Cleric/Druid/Bard/Paladin/Ranger; Sorcerer's list is a subset of Wizard's for this population, 0 units), so one PCGen JVM start per class verifies dozens to hundreds of units at once.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-compare-spell-batch.py` (new) — joins the batch export back to `ours` by `(level, name)` and calls `scripts/oracle_harness/compare.py::compare_unit` (`AT-33-E2-003`, unmodified) for the verdict — a join layer in front of the proven harness, not a fork of it.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/fixture-spell-pcg/{wizard,cleric,druid,bard,paladin,ranger}.pcg` (new, 6 files, 911 lines total) — real, live-loaded PCGen characters.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/fixture-spell-batch.txt.ftl` (new) — shared, class-index-generic BatchExporter template.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/fixture-spell-oracle-txt/{wizard,cleric,druid,bard,paladin,ranger}.export.txt` (new, 6 files) — real, live `./gradlew run` BatchExporter output, one per class.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-spell.oracle-results.json` (new) — 690 per-unit `(ours, oracle, verdict)` rows.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-spell-probe-output.json` (new) — the probe binary's own output (spell rows + unverifiable rows + spell_unresolved rows), committed so the batch/compare scripts' input is reproducible without re-running the binary.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json` (new) — the FULL 1,128-row combined result: 11 equipment (folded forward from attempt 1's own real, live, already-committed oracle round-trip, not re-run) + 690 spell (this cycle) + 427 companion/monster/monster_ability (this cycle, `unverifiable`, per-unit, real structural reason).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-001_cycle_receipt.md` (this file, overwritten in place per the remediation brief).
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated in place).
  - `docs/release/SD-33-computed-value-verification/kanban.md` (row 16 updated in place).
  - `docs/retro/events/sd33-r-e5-fixture.jsonl` (new — one `resolution` event closing attempt 1's open deferral `1787634716478-sd33-e5-fixture-c725c5`).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (see command output below)
- **Wired-integration audit result:** OK_NO_TOKENS (see command output below)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.

## What landed this cycle

Attempt 1 hand-authored one `.pcg` per unit via a scratch crate outside the repo — a method that
cannot reach 1,741 in any number of turns (11 units cost one full cycle). This cycle replaced that
method with a **repo-local batch generator + a repo-local batch "ours" probe binary**, per the
remediation brief's four-step lever (generate `.pcg`s programmatically, batch the oracle side,
batch our side, then run the population).

**Measured per-unit cost, stated before the full run (remediation brief requirement):** one PCGen
JVM start (`./gradlew run`, warm Gradle daemon, pre-built jar) costs **~22–58s**, dominated by
PCGen's own game-data load, not by character complexity or spell count. A 1,741-character,
one-.pcg-per-unit run would cost **~10–28 hours** — infeasible. **Batching by casting class cost
6 JVM starts for 690 spell units — ~4 minutes total** — because PCGen's own `SPELLMEM.*` export
is generic over an arbitrary number of spells per character (confirmed empirically this cycle: a
single `.pcg` can carry hundreds of `SPELLNAME:` lines and export a real, independently-computed
DC for every one in one JVM start). **Units per character this cycle: up to 424** (Wizard).

### Population re-examined this cycle: 1,128 of 1,741 (64.8%)

| Sub-population | Count | Method | Result |
|---|---:|---|---|
| `equipment` | 11 | Folded forward from attempt 1's own real, live, already-committed oracle round-trip (`equipment.oracle-results.json`) — not re-run, per the remediation brief ("KEEP them, fold them into your full result set"). | 11 agree, 0 disagree |
| `spell`, DC-probe-eligible | 690 | **New this cycle.** Real, live PCGen `BatchExporter` export (6 batched characters, one per casting class: Wizard 424, Cleric 102, Druid 92, Bard 37, Paladin 24, Ranger 11) vs. the real, live `compute_spellbook_coverage` output (`fixture_verified_oracle_probe`). | 268 agree, 103 disagree, 319 unverifiable |
| `companion`/`monster`/`monster_ability` | 427 | **New this cycle.** No magnitude probe exists for these kinds at all (`AT-33-E1-003`: `probe_exists: false`, `presence_only`) — recorded `unverifiable`, per-unit, with that real reason. `unverifiable` is a first-class verdict (`AT-33-E2-003`), not a synonym for unexamined: every one of these 427 units WAS fed to this cycle's classification and got a real, individually-reasoned verdict. | 427 unverifiable |
| **Total examined** | **1,128** | | **279 agree, 103 disagree, 746 unverifiable** |

Re-derive: `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results'])))"` → `1128`, `Counter({'unverifiable': 746, 'agree': 279, 'disagree': 103})`.

### Population NOT examined this cycle: 613 of 1,741 (35.2%) — two genuine, newly-discovered structural boundaries, named per unit-group, not a throughput shortfall

**598 `spell` units carrying evidence `spell_list_entry_with_resolved_level`, not `spell_effect_probe_observed_computed_delta`.** Attempt 1's own population statement treated `spell` as one
homogeneous 1,288-unit group; this cycle discovered it is not. `jq -r '[.units[]|select(.status=="fixture-verified" and .kind=="spell")]|group_by(.evidence)|map({evidence:.[0].evidence,count:length})' docs/work-inventory.json` → `spell_effect_probe_observed_computed_delta: 690`, `spell_list_entry_with_resolved_level: 598`. The 598 were promoted to `fixture-verified` via a *different* mechanism (`v06_work_inventory.rs`'s `derived_fixture_verified` upgrade path, not the save-DC consumer-delta probe), and this cycle confirmed **why they cannot be examined by the same DC-comparison mechanism**: `codex::rules_core::spellbook::casting_ability_for_class` (`src/rules_core/spellbook.rs:143-150`) maps a casting ability for **exactly seven classes** — `wizard`, `cleric`, `druid`, `ranger`, `sorcerer`, `bard`, `paladin` — and no others. These 598 units' spell keys are not on any of those seven classes' spell lists (confirmed: `fixture_verified_oracle_probe`'s `spell_unresolved` array, 598 entries, e.g. `Bomber's Eye`, `Amplify Elixir` — real Alchemist/Investigator/Magus/Witch/Oracle/Summoner spells this engine has no casting-ability mapping for at all). There is genuinely no `spell_save_dc` "ours" value this engine's own `compute_spellbook_coverage` can produce for them via the mechanism that examined the other 690 — a different, wider engine seam (or a different comparison shape entirely, since the underlying magnitude these carry may not be a save DC) would need to exist first. **This is not a throughput problem; batching would not help it.**

**15 `class_feature` units.** Not attempted this cycle. Each carries a *different* magnitude shape (DR / sneak-attack dice / channel-energy dice+uses / trap-sense / fixed numeric bonuses — `Bloodrager ~ Damage Reduction`, `Slayer ~ Sneak Attack`, `Paladin ~ Channel Positive Energy`, `Rogue ~ Trap Sense`, `Ninja ~ No Trace`, `Samurai ~ Resolve`, ... — 15 distinct features, at least 6 distinct export-token families per `outputsheets/base.xml.ftl`), and the engine-side "ours" value is produced by `probe_class_feature_effect_wiring`'s consumer-delta mechanism in `v06_work_inventory.rs`, which needs the FULL pilot-compute pipeline (`build_pilot_headless_receipt`) with a real character build (race/feats/full class progression), not the narrow library seam the spell/equipment probes use. Replicating that correctly for 15 units in the time remaining in this cycle risked a rushed, unverified implementation; attempt 1's own next-cycle plan (one L20 `.pcg` per source class) remains the concrete, correct next step.

## A real, uniform finding among the 103 disagreements — for `AT-33-E5-003` to root-cause

Every one of the 103 disagreements carries **exactly the same delta: `ours - oracle == 4`** (re-derive: `python3 -c "import json,collections; d=json.load(open('.../fixture-spell.oracle-results.json')); print(collections.Counter(r['ours']-r['oracle'] for r in d['results'] if r['verdict']=='disagree'))"` → `Counter({4: 103})`). `4` is exactly `SPELL_PROBE_ABILITY_MODIFIER`, the fixed ability modifier both sides are pinned to. **Candidate root cause, stated as a hypothesis, not confirmed further this cycle:** these 103 are plausibly spells with no actual saving throw (buff/summon/touch-ally spells — the sampled names include `Align Weapon (Communal)`, `Guardian of Faith`, `Marching Chant`, none of which grant a save in the real PF1 rules), and PCGen's `SPELLMEM.*.DC` token reports a bare `10 + level` baseline for such spells rather than adding the ability modifier, while our own `probe_spell_key`/`compute_spellbook_coverage` formula (`10 + level + modifier`) is applied uniformly regardless of whether the spell actually has a save. **Not fixed in this cycle** — this criterion (`AT-33-E5-001`) is examination; `AT-33-E5-003` (a separate criterion) owns root-causing and fixing or escalating every disagreement this cycle produced, and this cycle's write scope does not include the engine files (`src/rules_core/spellbook.rs`, `v06_work_inventory.rs`) a fix would touch.

## Deferral resolved, none filed

Attempt 1's open deferral (`1787634716478-sd33-e5-fixture-c725c5`, "the 1,730 of 1,741 not-yet-examined
fixture-verified units", revisit: "next AT-33-E5-001 cycle picks up class_feature (15) and spell
(1288)") is **resolved** this cycle via `scripts/retro.py resolution --resolves
1787634716478-sd33-e5-fixture-c725c5 ...` (event `1787639860118-sd33-r-e5-fixture-14ee14`,
`docs/retro/events/sd33-r-e5-fixture.jsonl`) — superseded by this cycle's own more precise
613-unit remainder (598 + 15, both named above with their own real reasons), not the coarser
1,730 the original deferral described. **No replacement deferral is filed**, per the remediation
brief. The 613-unit remainder is recorded directly in this receipt's Notes/Next-cycle plan below.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| `fixture-verified` population | 1,741 | of 49,438 total inventory units | `jq '[.units[]\|select(.status=="fixture-verified")]\|length' docs/work-inventory.json` |
| Units examined this cycle + folded forward | 1,128 | of 1,741 (64.8%) | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json'))['results']))"` |
| Agreement among units examined | 279 | of 1,128 examined (24.7%) | see combined-file Counter command above |
| Disagreement among units examined | 103 | of 1,128 examined (9.1%) | same |
| Unverifiable among units examined | 746 | of 1,128 examined (66.1%) — 319 real PCGen-absence + 427 structural no-probe | same |
| Units NOT examined this cycle | 613 | of 1,741 (35.2%) — 598 spell (no casting-ability mapping) + 15 class_feature (different magnitude shape, out of this cycle's budget) | `jq '[.units[]\|select(.status=="fixture-verified" and .kind=="spell" and .evidence=="spell_list_entry_with_resolved_level")]\|length' docs/work-inventory.json` → 598; `jq '[.units[]\|select(.status=="fixture-verified" and .kind=="class_feature")]\|length' docs/work-inventory.json` → 15 |
| Per-unit cost, measured | ~22–58s per PCGen JVM start, independent of spell count per character | 12 real `./gradlew run` invocations timed this cycle | `time ./gradlew run --args=...` (see cycle transcript) |
| Units per character, spell batch | up to 424 (Wizard); 102/92/37/24/11 (Cleric/Druid/Bard/Paladin/Ranger) | of 690 spell units, 6 characters | `python3 -c "import json; m=json.load(open('.../fixture-spell-batch.manifest.json')); print({k:v['count'] for k,v in m['by_class'].items()})"` |
| `box_ledger.py --check` against this cycle's combined results | `uncovered=0 overlap=0 population=49438 oracle_disagreement=103 unverifiable_done=0 stale=False`, exit **1** (correctly — 103 real disagreements exist; the fail-closed gate is doing its job) | population 49,438 | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json` |
| Disagreement delta uniformity | all 103 disagreements carry `ours-oracle=4` | of 103 disagreements | `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-spell.oracle-results.json')); print(collections.Counter(r['ours']-r['oracle'] for r in d['results'] if r['verdict']=='disagree'))"` |

## Status: in-progress

**Not `complete`.** 1,128 of 1,741 units are genuinely examined this cycle (a 100x increase over
attempt 1's 11) with real, per-unit `(ours, oracle, verdict)` rows and a real, batched, repo-local
generator+probe pipeline that reaches this scale in one cycle. The remaining 613 are named with
concrete, real, structural reasons (not "ran out of time" vaguely) and a concrete next-cycle plan
below. Marking this row `complete` while 613 of the 1,741-unit population stay unexamined would
repeat exactly the false-completion shape this remediation exists to close.

## Movement, four buckets

- **closure:** 0 — no unit's `status` field in `docs/work-inventory.json` changed; oracle
  verification results live in this directory's own JSON files, per `THE-BOX.md`'s existing
  convention (oracle-pending → oracle-confirmed is recorded here, not as an inventory status
  transition).
- **reclassification:** 0
- **reachability:** 0 — this cycle discovered a real ceiling (`casting_ability_for_class`'s
  seven-class limit) but did not widen it; that widening is future scope, named above.
- **instrument-correction:** 0 — `scripts/oracle_harness/compare.py`/`oracle_export.py` were used
  unmodified; the batch generator/probe binary are new instruments, not corrections to existing
  ones; and the 103-disagreement finding is *reported*, not fixed, here (that is `AT-33-E5-003`'s
  bucket).

## Notes

- **Real, live PCGen data-loading hazard found and fixed this cycle:** loading only the campaigns
  a spell's own book needs is insufficient — `advanced_class_guide`'s own `.pcc` declares
  `PRECAMPAIGN:1,INCLUDES=Ultimate Magic` (among others) and, without it, PCGen throws
  `IllegalStateException: Cannot ask for resolution: Reference Prodigy (%LIST) has not been
  resolved` while loading `acg_abilities_class.lst:3865`, and the WHOLE character load aborts
  (0 output, not partial output). Fix: every class `.pcg` this cycle loads the full closure of all
  8 spell-source books plus their own transitive `PRECAMPAIGN` dependencies (`Advanced Race
  Guide`, `Bestiary`, `Bestiary 2`, `Bestiary 3`) — see `fixture-generate-spell-batch.py`'s
  `ALWAYS_LOAD_CAMPAIGNS`.
- **PCGen's own internal `class` array index is not always 0** for a one-class character — Wizard/
  Cleric/Druid/Bard/Ranger landed on `SPELLLISTCLASS` index 0, Paladin landed on index 1. The
  shared template loops `class` 0..2 to cover every case observed rather than assuming 0
  (`fixture-spell-batch.txt.ftl`).
- **The oracle mechanism is a real, validating check, not a rubber stamp** — confirmed empirically
  before trusting it at scale: a `.pcg` `SPELLNAME:` line with a deliberately WRONG `SPELLLEVEL:`
  (real level 1, claimed level 5, `Magic Missile`) does not get silently accepted or relocated —
  PCGen drops it from the export entirely at every level. This is why an "unverifiable" verdict in
  this cycle's results can genuinely mean "our claimed level did not match PCGen's own data," not
  only "PCGen has no opinion" — a caveat carried forward for `AT-33-E5-003`'s root-causing of the
  319 unverifiable spell rows, which this cycle did not further disambiguate (absence alone cannot
  distinguish a wrong level from a name mismatch from a genuinely uncomparable spell).
- **Attempt 1's 11 equipment rows and its methodology discipline (README.md, the honest
  partition-of-what-was-examined framing) are kept, not rebuilt** — per the remediation brief.
- `SPELL_PROBE_ABILITY_SCORE`/`SPELL_PROBE_ABILITY_MODIFIER` (18 / +4) are restated as independent
  literals in `fixture_verified_oracle_probe.rs` rather than imported, because
  `v06_work_inventory.rs`'s constants are private to that binary; pinned to the identical value so
  this cycle's posture matches the original engine-wiring probe's posture by construction.

## RED→GREEN

Population coverage, not a new code path — same discipline Epic 2/attempt 1 used. **Before** this
cycle: `equipment.oracle-results.json` held 11 real rows (attempt 1, kept); no `spell` row existed
anywhere, and no repo-local batch mechanism existed at all (attempt 1's own equipment mechanism was
a scratch crate compiled OUTSIDE the repo). **After:** `src/bin/fixture_verified_oracle_probe.rs`
compiles and runs clean inside the repo (`cargo build --locked --bin
fixture_verified_oracle_probe`, `cargo run --locked --bin fixture_verified_oracle_probe -- --output
...`, exit 0); 6 real, live `./gradlew run` BatchExporter invocations against the real pinned
oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) all exit 0; 690 real per-unit
rows produced and independently verified by `scripts/box_ledger.py --check` (which correctly
flags the 103 real disagreements, exit 1 — the fail-closed gate firing as designed).

## Test scoping

Ran `cargo build --locked --bin fixture_verified_oracle_probe` and `cargo run --locked --bin
fixture_verified_oracle_probe -- --output ...` (both exit 0, warnings only — pre-existing,
unrelated to this cycle's diff: unused-import/dead-code warnings in `bestiary_5`/`bestiary_6`/
`mythic_adventures`/`occult_adventures`/`pathfinder_unchained`/`ultimate_intrigue`/`ultimate_magic`
monster_data modules and `pilot_compute/mod.rs`, none touched this cycle). Ran
`scripts/oracle_harness/compare.py`/`oracle_export.py` (`AT-33-E2-003`'s own module, imported not
modified) via `fixture-compare-spell-batch.py`. Ran `python3 scripts/box_ledger.py --check
--oracle-results ...` against the new combined file. **Did not** run the root `cargo test` sweep or
`apps/desktop/src-tauri` (a separate cargo workspace; no file in it touched this cycle) — no test
file changed this cycle (new `src/bin/` binary, no existing test suite covers it; a unit test for
this binary is future scope, named in next-cycle plan).

## Next-cycle plan

1. **The 15 `class_feature` units:** attempt 1's plan stands — one L20 `.pcg` per source class
   (Rogue/Paladin/Ranger/Bloodrager/Slayer/Ninja/Samurai cover 14 of 15 for free via class
   progression; Barbarian + `rage_power_superstition` needs one explicit rage-power selection).
   Reading each feature's real export token out of `outputsheets/base.xml.ftl` (DR/sneak-attack
   dice/channel-energy dice+uses/trap-sense/fixed bonus — at least 6 distinct token families) is
   the real remaining cost, not character authoring. The engine-side "ours" value needs the full
   `build_pilot_headless_receipt` pipeline (`probe_class_feature_effect_wiring`'s own mechanism),
   not the narrow spellbook/equipment library seam this cycle used — a new probe binary, or an
   extension granted access to that pipeline.
2. **The 598 `spell_list_entry_with_resolved_level` units:** first establish whether this engine
   models a "computed value" for them AT ALL (per-book, per-non-7-class spell lists and any
   consumer of them) before attempting oracle comparison — if no consumer exists, examining them
   may correctly resolve to "unverifiable, no ours value exists," a real, first-class outcome, not
   a gap to force-close. `fixture_verified_oracle_probe.rs`'s `spell_unresolved` array (598 real
   entries, committed in `fixture-spell-probe-output.json`) is the exact worklist.
3. **The 103 disagreements' root cause** (the uniform `ours-oracle=4` finding above) is
   `AT-33-E5-003`'s scope: confirm the no-save-spell hypothesis against the real corpus `SAVE:`
   token per spell, then fix `probe_spell_key`/`compute_spellbook_coverage`'s DC formula (only add
   the ability modifier when the spell actually grants a save) or the harness's comparison
   assumption, whichever is wrong — and re-run every prior oracle-results file it touches.
4. **`AT-33-E5-002`** (the 6,589 `literal-verified` units) is a separate criterion, running in
   parallel this cycle per `workflow-instruction.md §3` — this cycle's generator/probe pattern
   (`fixture_verified_oracle_probe.rs`, `fixture-generate-spell-batch.py`,
   `fixture-compare-spell-batch.py`) is directly reusable for its own spell/equipment sub-populations
   once that lane's own inventory join is built; not started by this cycle.

## Remediation wave 2 — sibling lane contribution (`equipment-remainder`)

Reported here per the wave-2 dispatch's own instruction ("report your slice's totals into both
AT-33-E5-00{1,2} receipts' figure rows"); this lane (`sd33-r2-equipment`) does not alter this
receipt's own figures above, which remain this cycle's own 1,128-of-1,741 result.

- **Population:** 494 (448 equipment `other_bonus_shape` + 46 `equipment_modifier`) — its own named
  slice of the 1,390-unit Epic-5 remainder (32 -> 6,940 of 8,330 after wave 1).
- **Examined:** 103 of 494 — 65 agree / 1 disagree / 37 unverifiable (each `unverifiable` row
  carries a populated `reason`).
- **The 1 real disagreement** (`ultimate_equipment:equipment:ring_of_the_sea_strider`, `ours=8`
  `oracle=16`): a real, root-caused engine gap — `compute_general_effect` does not model PF1's
  "a granted swim speed implies an automatic +8 racial Swim bonus" rule, which stacks with the
  item's own explicit `+8` racial token in PCGen's real output. Named for `AT-33-E5-003` to fix or
  escalate; not fixed by this lane.
- **Remaining 391:** named per-shape (`VAR` 108, `COMBAT` 92, `STAT_multi_or_other_slot` 43,
  `SITUATION` 34, `SAVE` 24, `WEAPON` 18, ... ) with a concrete next-cycle plan.
- **Receipt:** `artifacts/epic-5-reverification/AT-33-E5-remainder-equipment_cycle_receipt.md`.
- **Results file:** `artifacts/epic-5-reverification/equipment-remainder.oracle-results.json`.

## Remediation wave 2 — sibling lane contribution (`spell-remainder`)

Reported here per the wave-2 dispatch's own instruction ("report your slice's totals into both
AT-33-E5-00{1,2} receipts' figure rows"); this lane (`sd33-r2-spell`) does not alter this receipt's
own figures above, which remain this cycle's own 1,128-of-1,741 result.

- **Population:** 815 (598 `fixture-verified` `spell` units, `evidence=spell_list_entry_with_resolved_level`
  + 217 `literal-verified` `spell` units) — its own named slice of the 1,390-unit Epic-5 remainder.
- **Examined via live oracle:** 100 of 815 — 55 agree / 0 disagree / 45 unverifiable (each
  `unverifiable` row carries a populated `reason`).
- **The real blocker, found by execution, corrects this receipt's own framing**: this receipt's
  Notes attributed the 598-unit remainder to "no casting-ability mapping." This cycle built that
  mapping (36 classes, derived from the pinned PCGen oracle's own `SPELLSTAT` data) and found it is
  NOT the dominant blocker — a live `compute_spellbook_coverage` attempt against every mapped-class
  candidate on each of the 708 named-blocker units' own corpus `CLASSES:` token resolved **zero**.
  The real ceiling: `src/rules_core/spellbook/*.rs`'s per-school `resolve_<school>_spell_effect`
  functions only read `core_rulebook`/`advanced_players_guide`/`advanced_class_guide`'s own
  `SPELL_LIST` tables (469 of 708 blocked by this alone); the remaining 239 are genuinely blocked
  by class-mapping scope (192) or a missing class/domain binding in the corpus record (47).
- **A real defect surfaced, named for `AT-33-E5-003`**: 14 of the 100 examined units' declared
  spell level (this engine's per-school generic table) was silently dropped by PCGen's own
  `BatchExporter` — real, live evidence the per-school table's level disagrees with the per-class
  table's level for those 14 spells (`Blood Biography`, `Bestow Curse`, `Contagion`, and 11 others,
  named individually in this lane's own receipt).
- **Remaining 715:** every unit carries a real, per-unit, execution-derived reason (469 book-scope
  + 192 class-unmapped + 47 no-class-binding + 7 no-corpus-level) — none reasonless.
- **Receipt:** `artifacts/epic-5-reverification/AT-33-E5-remainder-spell_cycle_receipt.md`.
- **Results file:** `artifacts/epic-5-reverification/spell-remainder.oracle-results.json`.
