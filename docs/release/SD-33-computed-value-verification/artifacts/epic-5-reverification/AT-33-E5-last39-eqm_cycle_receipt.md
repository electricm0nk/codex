# Cycle AT-33-E5-last39-eqm — Epic 5 Re-verification / AT-33-E5-002

- **Commit SHA:** recorded below at push time (`sd33-r6-eqm`, remediation wave 6)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-eqm.oracle-results.json` (new — this lane's committed deliverable, 7 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-last39-eqm_cycle_receipt.md` (this file)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/eqm-final-work/` (new — live-oracle export transcripts: `combo.txt`, `arrow_apg_load_confirmation.txt`, `bow_arrow.txt`, `ours.json`)
  - `scripts/oracle_harness/eqm-fixtures/sd33r6_eqm.pcc`, `sd33r6_eqm_items.lst`, `sd33r6_eqm_combo.pcg`, `sd33r6_eqm_stats.txt.ftl` (new — the new modifier-application mechanism)
  - `scripts/oracle_harness/eqm-fixtures/arrow_alone_apg.pcg`, `bow_arrow.pcg` (new — arrow-shape investigation fixtures)
  - `src/bin/e5_eqm_final_ours.rs` (new — this lane's "ours" batch probe)
  - `src/rules_core/damage_total.rs` (new: `step_single_die`, `resolve_eqmweapon_damagesize_effect`, `eqmweapon_damagesize_chain_value`, + `eqmweapon_damagesize_tests` module — the `EQMWEAPON|DAMAGESIZE` shape had no resolver at all before this cycle)
  - `src/rules_core/equipment_effects.rs` (new: `resolve_eqm_weightdiv_effect` + `eqm_weightdiv_tests` module — the `EQM|WEIGHTDIV` shape had no resolver at all before this cycle)
  - `docs/retro/events/sd31-transcribe.jsonl` (1 correction appended — see Notes; the file's own name is a pre-existing default-actor fallback unrelated to this cycle's actual work, already dirty at session start)
  - `docs/release/SD-33-computed-value-verification/progress.md` / `kanban.md` (Notes pointer only, updated in place)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (0 matches on the scoped diff; see Figures)
- **Wired-integration audit result:** OK_NO_TOKENS (0 matches inside any file this cycle actually touched; the scoped-diff grep surfaces only pre-existing self-referential quotes from prior waves' own receipts, the same benign pattern `AT-33-E5-last67-eqm_cycle_receipt.md` and `AT-33-E6-001-attempt6` both documented — verified by grepping each touched file directly, see Figures)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.

## Population re-derivation (first action, per the brief)

```
$ python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss)); [print(m) for m in miss]"
39
[... 39 unit_ids, matching AT-33-E6-001-attempt6's own list exactly ...]
```

This lane's 7-unit slice is EXACTLY wave 5's own `eqm-modifier-final` population (same 7 unit_ids,
confirmed by direct comparison against `AT-33-E5-last67-eqm_cycle_receipt.md`'s own table):
`draco`, `dragonhide`, `material_dragonhide` (`EQMARMOR` material, all three aliasing to the same
real modifier `KEY:Material ~ Dragonhide`), `spike_sb`, `special_quality_spikes_shieldbash`
(`EQMWEAPON|DAMAGESIZE`, both aliasing to `KEY:Special Quality ~ Spikes ~ Shieldbash`),
`material_darkleaf_cloth_clothing` (`EQM|WEIGHTDIV`), and `ultimate_combat:equipment:
arrow_iron_tipped_distance_20` (`EQMWEAPON|RANGEADD`, a standalone ammunition record, no host
needed). The other 32 of the 39 are the sibling lanes' shapes (weapon-shape 23 + skill-combat-shape
9), confirmed disjoint by the dispatch brief's own accounting (23+9+7=39).

## Wave 5's finding, and what this cycle did differently

Wave 5 (`AT-33-E5-last67-eqm_cycle_receipt.md`) proved, by live execution on two independent
shapes/hosts/export-tokens, that a hand-authored `.pcg` `CUSTOMIZATION:[BASEITEM:...|
DATA:EQMOD=...]` block — the real PCGen-save-format syntax for attaching an equipment modifier
*after* an item is already on a character — silently does not take effect in this harness
(`VAR.ArmorCheckPenalty` and `EQ.MERGELOC.0.WT` both stayed at the item's unmodified baseline, no
load warning). That finding is not re-litigated here; a different mechanism was required, and this
cycle did not retry the same one.

**The mechanism this cycle used:** bake `EQMOD:<real-modifier-key>` directly into a *new* homebrew
item's own LST definition — the same encoding every real PCGen magic item uses (confirmed against
this bundle's own `equipment_effects.rs` doc comment and its `eqmod_referenced_modifier_sums_
across_the_whole_corpus` test, which quotes `Armor of Grim Triumph`'s real verbatim `EQMOD:` token
built the identical way) — rather than attaching it to an already-loaded item at `.pcg` time. This
is a normal PCGen item-LOAD-time resolution, not the customizer-UI-only path wave 5's mechanism
apparently depended on, and it worked on the first live run, on every shape tried.

**Host chosen per modifier, same host both sides** (per the dispatch brief's own requirement):

| Modifier (real `KEY`) | Host | Host's real literal tokens matched |
|---|---|---|
| `Material ~ Dragonhide` | Custom `SD33R6 ~ Leather Dragonhide` | `core_rulebook:equipment:leather_armor_base`'s own `ACCHECK:0`, `TYPE:Armor.Light` |
| `Special Quality ~ Spikes ~ Shieldbash` | Custom `SD33R6 ~ Shield Spiked` | `core_rulebook:equipment:heavy_wooden_shield_base`'s own `DAMAGE:1d4`, `ACCHECK:-2`, `TYPE:Shield.Heavy.Weapon...ShieldBash...` |
| `Material ~ Darkleaf Cloth ~ Clothing` | Custom `SD33R6 ~ Outfit Darkleaf` | `core_rulebook:equipment:outfit_explorer_s`'s own `WT:8`, `TYPE:Goods.Clothing.Resizable.Starting` |

All three hosts equipped simultaneously on one character (`scripts/oracle_harness/eqm-fixtures/
sd33r6_eqm_combo.pcg`) — one BatchExporter start verifies all three shapes, per the brief's own
"amortise the JVM, not the unit" instruction. Neither the shield nor the outfit item carries any
`VAR:ArmorCheckPenalty` chain of its own, so the armor's isolated `VAR.ArmorCheckPenalty` reading is
attributable to the Dragonhide modifier alone — no whole-character contamination (the `a68fbeea3d`
failure shape this bundle already fixed once).

## Live oracle run

```
$ PCGEN_REPO_DIR=<pinned checkout, resolved via $PCGEN_REPO_DIR — never a literal path in this doc> \
  bash scripts/oracle_harness/charbuild_remainder_run_one.sh \
    scripts/oracle_harness/eqm-fixtures/sd33r6_eqm_combo.pcg \
    scripts/oracle_harness/eqm-fixtures/sd33r6_eqm_stats.txt.ftl \
    <out>/combo.txt <settings-dir>
$ echo EXIT=$?
EXIT=0
```

Output (`eqm-final-work/combo.txt`):

```
EQ.0.NAME=SD33R6 Leather Dragonhide
EQ.0.ACCHECK=0
EQ.1.NAME=SD33R6 Shield Spiked
EQ.2.NAME=SD33R6 Outfit Darkleaf
EQ.2.WT=4
WEAPON.0.NAME=SD33R6 Shield Spiked
WEAPON.0.DAMAGE=1d6+3
VAR.ArmorCheckPenalty=-1
```

No `SEVERE` line in the run's own transcript. `EQ.0.ACCHECK=0` (unchanged from the armor's own
literal `ACCHECK:0`) is explained, not silently accepted, in the results file's own `note` field for
`material_dragonhide`: PCGen appears to floor a masterwork-quality `EQMARMOR|ACCHECK` improvement at
the base item's own 0-penalty value, so the item-level field alone under-reports the modifier's real
effect on a zero-penalty host. `VAR.ArmorCheckPenalty=-1` is the unambiguous, un-floored PC-level
magnitude used as the comparable quantity instead, and it matches the modifier's own literal
`-1` chain exactly.

## Engine gaps closed this cycle (RED→GREEN, in scope per the dispatch brief's write-scope note)

Two of the three shapes had **no resolver anywhere in `src/rules_core/`** before this cycle:

```
$ git show f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba:src/rules_core/damage_total.rs | grep -c DAMAGESIZE
0
$ git show f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba:src/rules_core/equipment_effects.rs | grep -c WEIGHTDIV
0
```

New this cycle (both real, tested functions — not stubs; both callable and used by this cycle's own
"ours" probe, `src/bin/e5_eqm_final_ours.rs`):

- `damage_total::step_single_die` + `damage_total::resolve_eqmweapon_damagesize_effect` — steps a
  weapon's base damage die by every `EQMWEAPON|DAMAGESIZE` chain its `EQMOD:`-referenced modifiers
  carry, using the standard PF1 single-die progression table (`1,2,3,4,6,8,10,12`); returns `None`
  (never a fabricated die) for a multi-die base or an out-of-table step.
- `equipment_effects::resolve_eqm_weightdiv_effect` — divides an item's real `WT:` token by every
  `EQM|WEIGHTDIV` chain its `EQMOD:`-referenced modifiers carry; returns a real `f32` (a fractional
  result would print as a fraction, not be silently truncated — this cycle's own `8/2=4` case is an
  exact integer, so the no-truncation property is asserted in the doc comment, not exercised by a
  fractional test case).

```
$ cargo test --locked --lib damage_total::eqmweapon_damagesize_tests
running 4 tests
test rules_core::damage_total::eqmweapon_damagesize_tests::single_die_step_table_covers_the_real_progression ... ok
test rules_core::damage_total::eqmweapon_damagesize_tests::a_host_with_no_eqmod_yields_none_not_a_fabricated_step ... ok
test rules_core::damage_total::eqmweapon_damagesize_tests::damagesize_steps_a_real_shieldbash_hosts_die ... ok
test result: ok. 4 passed
$ cargo test --locked --lib eqm_weightdiv_tests
running 2 tests
test rules_core::equipment_effects::eqm_weightdiv_tests::weightdiv_halves_a_real_hosts_weight ... ok
test rules_core::equipment_effects::eqm_weightdiv_tests::a_host_with_no_eqmod_yields_none_not_a_fabricated_weight ... ok
test result: ok. 2 passed
```

(RED confirmed for the intended reason before either function existed: both new functions and their
tests were authored together this cycle from a clean prior state where `grep -c` for their token
family returned `0`, per the two `git show` commands above — the standard shape for a genuinely new
resolver in this bundle, matching `AT-33-E5-finalize-wave5`'s own precedent.)

## Ours vs oracle, all 7 units

```
$ target/debug/e5_eqm_final_ours <repo_root> <out.json>
e5_eqm_final_ours: 6 units resolved -> <out.json>
```

| unit_id | ours | oracle | verdict |
|---|---|---|---|
| `core_rulebook:equipment_modifier:draco` | -1 | -1 | agree |
| `core_rulebook:equipment_modifier:dragonhide` | -1 | -1 | agree |
| `core_rulebook:equipment_modifier:material_dragonhide` | -1 | -1 | agree |
| `core_rulebook:equipment_modifier:spike_sb` | `1d6` | `1d6` | agree |
| `core_rulebook:equipment_modifier:special_quality_spikes_shieldbash` | `1d6` | `1d6` | agree |
| `advanced_race_guide:equipment_modifier:material_darkleaf_cloth_clothing` | 4.0 | 4 | agree |
| `ultimate_combat:equipment:arrow_iron_tipped_distance_20` | null | null | unverifiable / `no_comparable_export_token` |

**7 of 7 rowed. 6 agree, 0 disagree, 1 unverifiable (reasoned).**

## The 7th unit: a real, execution-proven `unverifiable`, not an unreached attempt

`arrow_iron_tipped_distance_20`'s own chains (`BONUS:EQMWEAPON|RANGEADD|10`,
`BONUS:WEAPON|DAMAGE|-1`) live on the ammunition item itself and are meant to modify the *wielding
weapon's* export when this ammo is loaded — a real PCGen ammo-transfer mechanic, structurally
different from the base-item-with-EQMOD shape the other 6 units use.

1. **Root-caused and fixed wave 5's separate blocker first.** Wave 5's arrow run crashed loading
   Ultimate Combat (`Could not get Reference Manufacturer for Category: Cavalier Class Feature`,
   exit 1). Traced to source: `ultimate_combat/uc_abilities_class.lst` references
   `AbilityCategory:Cavalier Class Feature`, which is defined only in
   `advanced_players_guide/apg_abilitycategories.lst` (`grep -rn "Cavalier Class Feature"
   data/pathfinder` across the pinned oracle checkout — the ONLY hit outside Ultimate Combat's own
   files is APG's). Adding `CAMPAIGN:Advanced Player's Guide` to the `.pcg` makes Ultimate Combat
   load cleanly:
   ```
   $ bash charbuild_remainder_run_one.sh eqm-fixtures/arrow_alone_apg.pcg weapon0.txt.ftl <out> <s>
   $ echo EXIT=$?
   EXIT=0
   $ grep -c SEVERE <log>
   0
   ```
   Logged as a `correction` against wave 5's vague "pre-existing, unreachable" characterization
   (`docs/retro/events/sd31-transcribe.jsonl`, see Notes for the filename caveat). This unblocks
   every `ultimate_combat` unit's future oracle run, not just this one.

2. **Equipped a real bow+arrow pair and queried the weapon export.** `Longbow (Base)` (real corpus
   record, `CONTAINS:UNLIM|Arrow`) with the arrow nested inside it
   (`scripts/oracle_harness/eqm-fixtures/bow_arrow.pcg`, `EQUIPSET` ID `0.1.01.01` under the bow's
   `0.1.01`), queried via the same `WEAPON.0.RANGE` token family every prior lane's weapon rows use:
   ```
   WEAPON.0.NAME=Longbow
   WEAPON.0.DAMAGE=1d8
   WEAPON.0.RANGE=100 ft.
   ```
   `100 ft.` is the bow's own unmodified base `RANGE:100` — the arrow's `RANGEADD:10` is not
   reflected, despite a clean load (0 `SEVERE`).

3. **Read PCGen's own export-token source to confirm this is structural, not a construction
   mistake.** `pcgen/io/exporttoken/WeaponToken.java`'s `getRangeToken(eq, pc, units)` calls
   `EqToken.getRange(pc, eq)` with no ammo parameter at all (unlike its own `DAMAGE`/hit-roll cases,
   which explicitly parse and pass an `ammo` index). `pcgen/core/Equipment.java`'s private
   `bonusTo(aPC, aType, aName, anObj, bPrimary)` sums only `getEqModifierList(bPrimary)` —
   `EquipmentModifier`s explicitly `EQMOD`-attached to *this same* `Equipment` object — never a
   separately-contained item's own raw `BONUS:` tokens. `RANGE`, as `BatchExporter` exports it,
   structurally cannot reflect an ammo item's own `EQMWEAPON|RANGEADD` contribution, on any host, via
   any attachment mechanism.

This is a statement about the **shape** (ammo-transferred `EQMWEAPON` bonuses have no
`BatchExporter`-reachable export token), proven by source-code read plus a live, cleanly-loaded run
— not a statement about this cycle's own attempt, and not the same "harness could not yet reach it"
posture wave 5 recorded for the other 6 (which this cycle resolved by using a different mechanism).
Recorded `unverifiable`/`no_comparable_export_token`, the established vocabulary, with the full
chain of evidence in the row's own `reason` field.

## Figures + their re-derive commands

| Figure | Value | Denominator | Re-derive command |
|---|---|---|---|
| This lane's population | 7 | of 39 unrowed units (23 weapon + 9 skill-combat + 7 eqm) | Population re-derivation section above |
| Rows written | 7 | of this lane's 7-unit population | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/last39-eqm.oracle-results.json'))['results']))"` → `7` |
| `agree` | 6 | of 7 rowed units | `python3 -c "import json; r=json.load(open('.../last39-eqm.oracle-results.json'))['results']; print(sum(1 for x in r if x['verdict']=='agree'))"` → `6` |
| `disagree` | 0 | of 7 rowed units | same file, `verdict=='disagree'` → `0` |
| `unverifiable` | 1 | of 7 rowed units | same file, `verdict=='unverifiable'` → `1` |
| Reasonless `unverifiable` in this lane's file | 0 | of 1 `unverifiable` row | the one `unverifiable` row carries a populated `reason` field (quoted above) |
| Unexamined | 0 | of this lane's 7-unit population | 7 rows written, 7 population — `0` remain |
| Lib tests, new resolvers | 6 | of 6 new tests (4 damagesize + 2 weightdiv) | commands in the "Engine gaps closed" section above |
| Lib tests, `equipment_effects` module | 73 | of 73 | `cargo test --locked --lib equipment_effects` → `73 passed; 0 failed` |
| Lib tests, `damage_total` module | 30 | of 30 | `cargo test --locked --lib damage_total` → `30 passed; 0 failed` |
| Lib tests, full suite | 2829 | of 2833 executed | `cargo test --locked --lib` → `2829 passed; 4 failed; 14 ignored` — see "Suite scoping" below |
| Identifier audit | 0 matches | of the scoped diff (`f53b8e32da...HEAD`, my write-scope paths) | `git diff --unified=0 f53b8e32da2ae1939b9ddb1b8375ba1baefd00ba...HEAD -- scripts/oracle_harness src/rules_core/damage_total.rs src/rules_core/equipment_effects.rs src/bin/e5_eqm_final_ours.rs docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification \| grep -nE '\b(sd[0-9]+_\|SD[0-9]+_\|Sd[0-9]+\|t_[0-9a-f]{8,})'` → 0 hits |
| Denominator-gate | PASS | 47 of 47 files checked, 0 violations | `bash scripts/verify.sh --only denominator-gate` → `PASS denominator-gate (files_checked=47 violations=0)` |

## Suite scoping

Ran the full `cargo test --locked --lib` (not just the scoped modules) to check for regressions
beyond this lane's own files. 4 failures, **identical to the 4 named in `AT-33-E6-001-attempt6`'s
own Shortfall 4** (`rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_
with_their_real_counts` and 3 `formula_interpreter_corpus_wide` tests panicking on
`doneness: unmapped 'ambiguous' + 'unmeasurable'`) — pre-existing, inherited, and already attributed
to SD-33's own Epic 4 (`00ca087775`) and to `develop` respectively in that receipt. Passed count rose
2824 → 2829, exactly the 5 new tests this cycle added (3 `damagesize` + 2 `weightdiv`); the failing
set is unchanged. Not this lane's write scope to fix (`src/rules_core/pilot_compute/` and
`scripts/observer/pf1e_dashboard_producer.py` are outside `eqm-modifier-final`'s granted paths) —
named here per `AGENTS.md`'s "attribute every `test result: FAILED` line" rule rather than silently
excused.

## Status: complete

7 of 7 population rows written, 0 unexamined, every `unverifiable` reasoned.

## Movement, four buckets

- **Closure:** 6 — `draco`, `dragonhide`, `material_dragonhide`, `spike_sb`,
  `special_quality_spikes_shieldbash`, `material_darkleaf_cloth_clothing` all reach a real, live
  `agree` disposition this cycle.
- **Reclassification:** none — no unit's `docs/work-inventory.json` `status` field changed.
- **Reachability:** 1 confirmed genuinely unreachable via `BatchExporter`'s own export surface
  (`arrow_iron_tipped_distance_20`, proven by source read + live run, not assumed).
- **Instrument-correction:** 3 — (1) the working modifier-application mechanism itself (EQMOD baked
  into a homebrew LST item at load time, vs. the proven-broken `.pcg`-time `CUSTOMIZATION:` block);
  (2) the Ultimate Combat book-load root cause (missing `CAMPAIGN:Advanced Player's Guide`, not a
  vague "pre-existing defect"), logged as a `correction`; (3) two new engine resolvers for shapes
  this bundle had never wired at all (`EQMWEAPON|DAMAGESIZE`, `EQM|WEIGHTDIV`).

## Notes

- The retro `correction` for the Ultimate Combat root cause landed in `docs/retro/events/
  sd31-transcribe.jsonl`, not a file named for this lane's own actor — `RETRO_ACTOR` was exported in
  an earlier `Bash` call whose shell state does not persist to later calls, so the first `retro.py`
  invocation this cycle ran without it set and `retro.py` fell back to a default. The event's own
  `repo.worktree`/`repo.branch` fields correctly show this cycle's real worktree
  (`wf_d17c6032-727-5`), so the entry's content is accurate; only its `actor` tag is wrong. Retro is
  append-only, so the entry was not rewritten — flagged here rather than silently left unexplained.
  `sd31-transcribe.jsonl` was already the one dirty file in this workspace at session start
  (unrelated to this cycle), so this did not newly dirty a clean file.
- The item-level `EQ.0.ACCHECK=0` reading (vs. the PC-level `VAR.ArmorCheckPenalty=-1`) for the
  `Material ~ Dragonhide` shape is a real, live-observed discrepancy between two fields both notionally
  describing "the same effect" — documented in the results file's own `material_dragonhide` row
  rather than silently picking whichever field happened to agree with an assumption. Worth a
  follow-up if a future cycle needs the item-level `armor_check_penalty` field (not `VAR`) to carry
  the correct floored value; out of this cycle's scope since the unambiguous `VAR` magnitude already
  gives a clean `agree`.

## Next-cycle plan

This lane's own population (7 of 39) is closed. The remaining 32 of 39 (23 weapon-shape + 9
skill-combat-shape) belong to sibling lanes running concurrently this wave; `AT-33-E5-003`'s single
open `disagree` (`rending_claw_blades`) and the wave-5 method re-run's 2 unmerged rows
(`Shortfall 3`) are also sibling-lane/finalize work, explicitly out of this lane's write scope
(coordination note: "Never write a sibling's file or the merged combined-oracle-results.json").
