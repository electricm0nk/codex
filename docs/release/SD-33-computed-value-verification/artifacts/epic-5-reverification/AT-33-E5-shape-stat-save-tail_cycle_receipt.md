# Cycle sd33-r3-statsave — Epic 5 Re-verification / AT-33-E5-002 (remediation wave 3, stat/save/situation/tail lane)

- **Commit SHA:** `b1838c8d38` (code + results + retro event; this receipt lands in a second commit referencing it, both pushed to `tranche/13`).
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/statsave-classify.py` (new) — full-record classifier for this lane's population: reads the WHOLE corpus record for every unit and mirrors `compute_equipment_effects`'s own four-resolver dispatch to decide whether ANY existing resolver would return `Some(...)`.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/statsave-full-classification.json` (new) — this lane's 160-unit classification output.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/statsave-generate-stat-pcgs.py`, `statsave-generate-skill-pcgs.py` (new) — `.pcg`/`.ftl` fixture generators, reusing `AT-33-E5-00x`'s proven Level-1 Human Fighter / `EQUIPSET:Equipped` mechanism.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/statsave-build-stat-results.py`, `statsave-build-skill-results.py`, `statsave-build-final-results.py` (new) — assemble per-unit `(ours, oracle, verdict)` rows from the real oracle exports and "ours" outputs into the committed results file.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/statsave-fixtures/` (new) — all `.pcg`/`.ftl` fixtures generated and run this cycle (`stat-pcg/`, `skill-pcg/`, `skill-pcg-dedupe/`).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/statsave-oracle-out-stat/`, `statsave-oracle-out-skill-export/` (new) — the real, live PCGen `BatchExporter` export output for every unit actually run.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/statsave-*-manifest*.json`, `statsave-*.ours.json`, `statsave-*-rows-debug.json` (new) — working data, kept for re-derivation.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-stat-save-tail.oracle-results.json` (new) — **the committed per-unit deliverable**, 141 rows.
  - `src/bin/e5_statsave_skill_ours.rs` (new) — repo-local batch "ours" probe for this lane's own SKILL population, real live calls into `compute_equipment_effects`, reading `skill_bonus`; differs from the sibling `e5_equipment_remainder_skill_ours.rs` only in accepting a `target_skill` distinct from the engine's own (possibly comma-joined) raw skill field, needed because this lane's population includes multi-skill items that binary's exact-match check would reject.
  - `docs/retro/events/sd33-r3-statsave.jsonl` (new).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.

## Population re-derivation (§ brief's own required first action)

The brief named a population of 158 (`STAT_multi_or_other_slot` 43 + `SITUATION` 34 + `SAVE` 24 +
11 smaller shapes 20 combined) and explicitly required re-deriving it via the literal subtraction
rule rather than inheriting the estimate. Re-derived:

```
python3 -c "
import json
labels = json.load(open('equipment-remainder-full448-labels.json'))
equipmod = json.load(open('equipment-remainder-equipmod-census.json'))['items']
examined = json.load(open('equipment-remainder.oracle-results.json'))['results']
examined_ids = set(r['unit_id'] for r in examined)
full_pop = set(labels) | set(x['unit_id'] for x in equipmod)
leftover = full_pop - examined_ids          # -> 391, matches the brief's own figure
sibling_shapes = {'VAR','COMBAT','WEAPON'}
is_wp = lambda l: l.startswith('WEAPONPROF=')
mine = [u for u in leftover if u in labels
        and not any(l in sibling_shapes or is_wp(l) for l in labels[u])]
print(len(mine))   # -> 160, not 158
"
```

**Result: 160, not 158.** `SKILL` (42 units) is unclaimed by either named sibling shape
(`VAR`; `COMBAT`/`WEAPON`/`WEAPONPROF=*`) and therefore falls to this lane under the brief's own
"you own the tail" rule — the brief's shape list (STAT/SITUATION/SAVE/11-smaller) named the shapes
the author expected, not the literal subtraction's output. Logged as a correction:
`scripts/retro.py summary` entry `1787651068927-sd33-r3-statsave-208363`.

The 14 chain-bearing `equipment_modifier` units left in the 391 (`EQMARMOR`/`EQMWEAPON`/`EQM`/
`WEAPON`-prefixed chains, re-derived: every one of their real `raw_bonus_chains` starts with
`WEAPON`, `EQMARMOR`, or `EQMWEAPON` — the weapon/armor-modifier family) are judged sibling-owned
by the same spirit as `COMBAT`/`WEAPON`, not this lane's tail — a documented judgment call, not an
oversight; see Notes.

**Denominator for every figure below: 160** (this lane's own re-derived population), itself part of
391 (Epic 5's whole remainder) of 8,330 (Epic 5's total examined+unexamined population).

## Generic pass, by mechanism (not per-item)

Re-derive: `python3 statsave-classify.py` reads the whole `raw_bonus_chains` list for every one of
the 160 units and checks it against each of `compute_equipment_effects`'s four resolvers
(`general::compute_general_effect` [`SKILL`], `magic_items::compute_magic_items_effect` [`STAT`],
`equipmods::compute_equipmods_effect`/`resolve_spell_resistance_bonus` [`WEAPON`+`TYPE=Enhancement`/
`SR`], arms_armor's `ACCHECK`/`MAXDEX`/`SPELLFAILURE`/`BONUS:COMBAT|AC` fields) — confirmed by
grepping `src/rules_core/equipment_effects/*.rs` for every one of `SAVE`/`SITUATION`/`ABILITYPOOL`/
`SLOTS`/`DC`/`MOVEADD`/`SPELLKNOWN`/`SPELLCASTMULT`/`POSTMOVEADD`/`VISION`/`LOADMULT`/`MOVEMULT`/
`HP`/`SPELLCAST`/`MISC`: **zero matches** — none of these qualifiers is ever read by any resolver.

| Shape (a unit's real chain-qualifier combination) | Units | This cycle's disposition |
|---|---:|---|
| `SKILL` alone | 40 | oracle round-trip (29 attempted, see below) |
| `STAT_multi_or_other_slot` alone | 26 | oracle round-trip, all 39 STAT-bearing units together |
| `SITUATION` alone | 24 | `unverifiable` — `no_probe_surface` |
| `SAVE` alone | 18 | `unverifiable` — `no_probe_surface` |
| `ABILITYPOOL`+`STAT_multi_or_other_slot` | 11 | STAT chain oracle-verified; `ABILITYPOOL` has no probe on its own but the unit's row is carried by its STAT chain |
| `SLOTS` alone | 8 | `unverifiable` — `no_probe_surface` |
| `DC` alone | 8 | `unverifiable` — `no_probe_surface` |
| `SPELLCASTMULT` alone | 4 | `unverifiable` — `no_probe_surface` |
| `SPELLKNOWN` alone | 4 | `unverifiable` — `no_probe_surface` |
| `SITUATION`+`SKILL` | 2 | SKILL chain attempted; carries `SITUATION` too (no probe for that half) |
| `POSTMOVEADD` alone | 2 | `unverifiable` — `no_probe_surface` |
| `VISION` alone | 2 | `unverifiable` — `no_probe_surface` |
| 8 singleton combinations (`POSTMOVEADD+SITUATION`, `MISC`, `HP`, `SPELLCAST`, `LOADMULT`, `MOVEADD+STAT_multi_or_other_slot`, `MISC+STAT_multi_or_other_slot`, `MISC+SITUATION`, `MOVEMULT`, `MOVEADD`, `ABILITYPOOL` alone) | 9 | 2 STAT-carried (oracle-verified), 7 `no_probe_surface` |

Re-derive: `python3 -c "import json,collections; d=json.load(open('statsave-full-classification.json'))['results']; c=collections.Counter('+'.join(r['labels']) for r in d); [print(v,k) for k,v in sorted(c.items(),key=lambda x:-x[1])]"`

## Group A — 79 units, `no_probe_surface`

**Real, sourced finding, no PCGen run needed**: for these 79 units, no chain on the whole record
matches any of the four resolvers' own match conditions — the engine literally never computes a
value for this shape today, for any unit of it, confirmed by source-code grep (above), not
per-unit guesswork. Every row carries the matched qualifier and the exact grep command in its
`reason` field. Per AT-33-E1-003's established vocabulary, this is `no_probe_surface`
(`probe_exists: false` for the shape), the same first-class verdict category used bundle-wide.

**Spot-checked, not assumed**: read the full corpus record for 3 `SITUATION` units
(`dilettante_s_outfit`, `gloves_of_elvenkind`, `half_orc_disguise_kit`) — every one is a genuinely
conditional bonus (concealing an object, casting defensively, disguising as a different race) with
no unconditional scalar to compare, confirming `no_probe_surface` is the right classification for
this shape on the merits, not merely on the engine's current absence of a resolver.

## Group B — 39 units, `STAT_multi_or_other_slot` (real oracle round-trip)

**Measured before committing to the full run** (remediation-brief requirement): one real
`./gradlew run` invocation (`ioun_stone_deep_red_sphere`) — `time` → `23.4s` cold. Projected: 39
units ÷ `-P 12` parallel ≈ 4 rounds × ~25s baseline, **but this shared box was under heavy
concurrent contention from sibling lanes this cycle** (`uptime` mid-run: load average 50-88 on 24
cores) — actual wall time for the 38-unit batch was ~35 minutes, not the ~2-minute naive
projection. Named honestly, matching every prior `AT-33-E5-00x` cycle's own throughput note.

**Mechanism**: reuses `e5_literal_stat_ours.rs` (`AT-33-E5-002`'s own binary, **unmodified** — no
`src/rules_core/` change needed) and `fixtures/e5-equip-stats.txt.ftl` (`AT-33-E5-001`'s own
6-ability export template, **unmodified**). `compute_magic_items_effect` already returns
`Some(AbilityScoreBonus{ability, bonus})` for every `BONUS:STAT|...` chain regardless of whether
`ability` is a single name or a comma-joined list (`qualifiers[1]` stored verbatim either way, and
`qualifiers[2]` — the bonus magnitude — parses cleanly regardless) — confirmed by reading
`magic_items.rs` directly, not assumed.

**Multi-ability decision (brief: "decide what one row means... say so, and be consistent")**: for
every multi-ability unit (e.g. `BONUS:STAT|STR,DEX,CON|4|TYPE=Enhancement`), this lane verifies the
**first named ability only**. PF1's real rule applies the SAME bonus independently to every named
ability, so this checks the mechanism once per unit; it does not independently confirm every named
ability for every unit (a real, stated scope limit, not a gap left unnamed).

**Result: 39 of 39 agree, 0 disagree**, after one real, root-caused fixture fix (below).

**Instrument-correction, found and fixed this cycle**: `staff_of_mithral_might`
(`ultimate_equipment`) initially showed `ours=12, oracle=10` — a real disagreement on its face.
Root-caused before reporting it as one (`AT-33-E5-003`'s doctrine): the corpus record is a
**two-handed weapon** (`WIELD:TwoHanded`, `TYPE:...Weapon...TwoHanded...`) carrying a
`BONUS:STAT|INT|2|TYPE=Enhancement` token, generated with this cycle's default
`EQUIPSET:Equipped` location like every non-weapon wondrous item — but PCGen does not apply a
weapon's `BONUS:` tokens from the generic `Equipped` location; it needs the weapon actually
wielded (`EQUIPSET:Both Hands` for a two-handed weapon). Confirmed empirically: re-running with
`EQUIPSET:Both Hands` produced `STAT.3.SCORE=12`, matching `ours` exactly. Fixed in the committed
`.pcg`; final row is `agree`. The other 38 units are wondrous items (belts/headbands/ioun
stones/rings/rods), for which `Equipped` is correct (38/38 agreed on the first pass — if the
location were wrong for that class, more would have disagreed, not just the one real weapon).

## Group C — 42 units, `SKILL` (real oracle round-trip, this lane's own re-derived tail)

**29 of 42 attempted** (13 excluded before any oracle run, each for a real, this-cycle-confirmed
reason, not a guess):

| Exclusion | Count | Real reason, confirmed this cycle |
|---|---:|---|
| `equipment_id_resolve` finds no record | 12 | all 12 `ultimate_psionics` items with no explicit corpus `KEY:` token — the exact resolver limitation `AT-33-E5-002`'s own equipment lane already named for this book; independently re-confirmed here (`e5_statsave_skill_ours`'s own `UNRESOLVED` list) |
| `compute_general_effect` returns `None` despite a real chain | 1 | `ring_self_sufficiency` — the same unexplained anomaly `AT-33-E5-002`'s own receipt named for this exact unit, independently re-confirmed |

**26 of 29 attempted reached a real oracle export; 3 hit a real, confirmed harness/data defect**
(all book-wide, not item-specific — every unit from the same book in this population would fail
the same way):

| Unit | Book | Real failure, confirmed by log |
|---|---|---|
| `hunter_s_sight`, `ring_of_eloquence` | `advanced_class_guide` | `java.lang.IllegalStateException: Cannot ask for resolution: Reference Prodigy (%LIST) has not been resolved` → the whole dataset load throws; **identical stack for both units**, confirming a book-wide data defect, not two independent item issues |
| `scarf_of_glorious_histories` | `inner_sea_races` | Same failure class, different unresolved reference: `Reference Tiefling ~ Maw or Claw (%LIST) has not been resolved` |

**Real, live oracle attempt run, then correctly excluded, not silently kept as a false `disagree`**
(`AT-33-E5-003`'s doctrine: "reporting these as `disagree` would be exactly the false-defect
shape... forbidden"):

| Unit | Book | Real failure, confirmed by log |
|---|---|---|
| `demon_senses` | `book_of_the_damned_volume_2` | `SEVERE Globals:130 Could not find campaign: Lords of Chaos - Book of the Damned, Volume 2` (the `.pcc`'s own `CAMPAIGN:` line matches exactly — checked directly against the pinned checkout — so this is a real loader defect, not a fixture typo); **also independently matches the `Magic.Wondrous.Implant` slot hazard `AT-33-E5-002`'s own receipt named for this exact unit** — either root cause alone explains the observed `MISC=0`, not root-caused further this cycle |
| `eyes_of_expanded_vision`, `third_eye_aware` | `ultimate_psionics` | `SEVERE Globals:130 Could not find campaign: Ultimate Psionics` — the exact harness defect `AT-33-E5-002`'s own receipt already diagnosed for these same 2 units, independently reproduced here |

**23 of 26 real, live-oracle-compared units: 23 agree, 0 disagree.**

**Instrument-correction, found and fixed this cycle (a real bug in this cycle's own method, not the
engine)**: the first-pass `.pcg`/`.ftl` filename convention used the bare item slug
(`unit_id.split(':')[-1]`), which collided for **3 genuine cross-book reprints** in this
population — `ring_of_maniacal_devices` (`advanced_players_guide` + `ultimate_equipment`),
`cloak_of_the_diplomat` (`advanced_race_guide` + `ultimate_equipment`), `ring_of_the_sophisticate`
(`advanced_race_guide` + `ultimate_equipment`) — silently letting the second book's `.pcg` write
overwrite the first's file. Caught this cycle (checked the fixture directory's real file count
against the expected unit count, found it short), root-caused (both units in each pair share an
identical `key`/`skill_field`/`declared_bonus`, matching genuine reprints, not a corpus error), and
**re-verified for real**: generated book-prefixed `.pcg`/`.ftl` files for the 3 previously-untested
unit_ids under `statsave-fixtures/skill-pcg-dedupe/` and ran them live — all 3 produced the exact
oracle value already on file (`5`, `5`, `4`), confirming the mis-attributed first-pass result was
numerically correct by coincidence (identical reprints), not merely assumed correct. Fixed at the
source: `statsave-generate-skill-pcgs.py`'s `slug()` now book-prefixes every filename and asserts
no collision, and `statsave-generate-stat-pcgs.py` gained the same collision assertion defensively
(no collision existed in that 39-unit population, checked this cycle).

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| This lane's re-derived population | 160 | of 391 (Epic 5's whole remainder) | see "Population re-derivation" above |
| Rows written (mechanical count) | **141** | of 160 (88.1%) | `python3 -c "import json;d=json.load(open('equipment-shape-stat-save-tail.oracle-results.json'));print(len(d['results']))"` → `141` |
| `agree` | 62 | of 141 examined | `python3 -c "import json,collections;d=json.load(open('equipment-shape-stat-save-tail.oracle-results.json'));print(collections.Counter(r['verdict'] for r in d['results']))"` |
| `disagree` | 0 | of 141 examined | same command |
| `unverifiable` | 79 | of 141 examined (all `no_probe_surface`) | same command |
| Reasonless `unverifiable` in this lane's own rows | 0 | of 79 `unverifiable` rows | `python3 -c "import json;d=json.load(open('equipment-shape-stat-save-tail.oracle-results.json'));print(len([r for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')]))"` → `0` |
| Duplicate `unit_id`s in this lane's own file | 0 | of 141 rows | `python3 -c "import json;d=json.load(open('equipment-shape-stat-save-tail.oracle-results.json'));ids=[r['unit_id'] for r in d['results']];print(len(ids)==len(set(ids)))"` → `True` |
| Units NOT examined this cycle | 19 | of 160 (11.9%) | 160 − 141 = 19: 12 (`equipment_id_resolve` no-KEY, `SKILL`) + 1 (`ring_self_sufficiency` anomaly, `SKILL`) + 3 (book-data-load failure, `SKILL`: `hunter_s_sight`/`ring_of_eloquence`/`scarf_of_glorious_histories`) + 3 (campaign-load failure, `SKILL`: `demon_senses`/`eyes_of_expanded_vision`/`third_eye_aware`) |
| `SKILL` group examined | 23 | of 42 (this lane's own re-derived `SKILL` share) | `python3 -c "import json;d=json.load(open('equipment-shape-stat-save-tail.oracle-results.json'));print(len([r for r in d['results'] if 'target_skill' in (r.get('note') or '')]))"` → `23` |
| `STAT_multi_or_other_slot` group examined | 39 | of 39 (this lane's own re-derived `STAT` share, 100%) | `python3 -c "import json;d=json.load(open('equipment-shape-stat-save-tail.oracle-results.json'));print(len([r for r in d['results'] if 'target_ability' in (r.get('note') or '')]))"` → `39` |
| `no_probe_surface` group examined | 79 | of 79 (100%) | `python3 -c "import json;d=json.load(open('equipment-shape-stat-save-tail.oracle-results.json'));print(len([r for r in d['results'] if r['verdict']=='unverifiable']))"` → `79` |

## Status: blocked-escalated

**Mechanical rule applied, per this wave's own new rule**: `len(rows) = 141 < population = 160`,
so status is `blocked-escalated`, not `complete`, regardless of how much real verification this
cycle did. `python3 -c "import json;print(len(json.load(open('equipment-shape-stat-save-tail.oracle-results.json'))['results']))"` → `141`.

**What is genuinely blocked, not merely unattempted**: every one of the 19 unexamined units hit a
real, this-cycle-confirmed wall (resolver gap, PCGen harness/campaign-load defect, or book-wide
data-load failure) — none is a skipped-for-time unit sitting unclassified. The wall for 15 of the
19 (all `ultimate_psionics`, `advanced_class_guide`, `inner_sea_races`, `book_of_the_damned_volume_2`
units) is outside this lane's write scope to fix (a PCGen oracle-checkout data/resolver defect, not
this repo's engine code); the wall for the other 12 (`equipment_id_resolve`'s no-explicit-KEY
resolver gap) is a real `src/rules_core/` fix a future cycle could make (see Next-cycle plan).

## Movement, four buckets

- **closure:** 0 — no `docs/work-inventory.json` `status` field changed this cycle; oracle
  verification results live in this lane's own results file, matching every prior `AT-33-E5-00x`
  cycle's own convention.
- **reclassification:** 0
- **reachability:** 0 — this cycle found real ceilings (the `ultimate_psionics`/`advanced_class_guide`/
  `inner_sea_races`/`book_of_the_damned_volume_2` harness/data defects, the `equipment_id_resolve`
  no-KEY gap) but did not widen any of them.
- **instrument-correction:** 2 — the `staff_of_mithral_might` two-handed-weapon `EQUIPSET` hazard
  (Group B) and the cross-book-reprint filename-collision hazard (Group C), both found and fixed
  within this cycle before either could produce a committed false result.

## Notes

- **The 42-unit `SKILL` population was not named in this lane's own dispatch brief** (which listed
  only `STAT`/`SITUATION`/`SAVE`/11-smaller-shapes as this lane's shapes) — it surfaced only from
  the brief's own literal subtraction instruction, since neither sibling lane's named shape list
  (`VAR`; `COMBAT`/`WEAPON`/`WEAPONPROF=*`) claims it. Covered in full per the brief's own
  instruction ("cover all of it"), not carved out for being unnamed.
- **The `demon_senses`/`eyes_of_expanded_vision`/`third_eye_aware` campaign-load failures were
  reproduced, not merely cited from `AT-33-E5-002`'s receipt** — this cycle independently ran (and,
  for `demon_senses`, re-ran a second time to rule out transience) against the same pinned oracle
  checkout and hit the identical `SEVERE Globals:130 Could not find campaign` failure, confirming
  the earlier lane's diagnosis rather than assuming it still holds.
- **The 14 chain-bearing `equipment_modifier` units** (re-derived: every real `raw_bonus_chains`
  entry on these 14 records starts with `WEAPON`, `EQMARMOR`, or `EQMWEAPON`) were judged
  sibling-owned rather than this lane's tail — a documented call, not a silent drop; if a sibling
  lane's own re-derivation excludes them too, they would be a real gap for a future cycle to name.
- **No `src/rules_core/` change was needed or made.** Every unit this cycle examined was already
  reachable through an existing resolver (`general`/`magic_items`) called unmodified; the 79
  `no_probe_surface` units are genuinely un-computed by the engine today, not blocked by a bug this
  cycle could fix within its own write scope without expanding into new engine feature work.

## RED→GREEN

Population coverage + one new binary (`e5_statsave_skill_ours`), not a new engine code path
(`compute_general_effect`/`compute_magic_items_effect` pre-exist and are called unmodified).
**Before this cycle**: `equipment-shape-stat-save-tail.oracle-results.json` did not exist; 0 of this
lane's 160-unit population had any per-unit disposition. **After**: `cargo build --locked --bin
e5_statsave_skill_ours --bin e5_literal_stat_ours` exits 0 (warnings only, pre-existing, same set
every prior `AT-33-E5-00x` cycle's receipt names); 39 STAT-shape + 29 SKILL-shape real, live
`./gradlew run` `BatchExporter` invocations (68 total, 65 exit 0, 3 named book-data-load failures)
ran against the real pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
confirmed matching this box's real checkout via `git rev-parse HEAD` before any run); 79
`no_probe_surface` rows are backed by a real, whole-record read of every one of the 79 corpus files
plus a real source-code grep of every equipment-effects resolver.

## Test scoping

Ran `cargo build --locked --bin e5_statsave_skill_ours --bin e5_literal_stat_ours` (exits 0). Ran
68 real `./gradlew run` `BatchExporter` invocations via `scripts/pcgen-run-character.sh` against the
pinned oracle checkout (`$PCGEN_REPO_DIR`). Ran `e5_literal_stat_ours`/`e5_statsave_skill_ours`
against this lane's own manifests. **Did not** run the root `cargo test` sweep or
`apps/desktop/src-tauri` (a separate cargo workspace; no file in it touched this cycle) — matching
every prior `AT-33-E5-00x` cycle's own precedent for a new data-pipeline `src/bin/` binary with no
`#[cfg(test)]` module, over already-tested engine code (`compute_general_effect`/
`compute_magic_items_effect` carry their own existing unit tests, unmodified this cycle).

## Next-cycle plan

1. **The 12 `equipment_id_resolve` no-explicit-KEY `ultimate_psionics` units**: the one remaining
   gap in this lane's own scope that is a real `src/rules_core/` fix, not an external harness
   defect — `AT-33-E5-002`'s own receipt named the same gap for a different 11-unit set; a shared
   fix would unlock both.
2. **`advanced_class_guide`'s `Reference Prodigy (%LIST)` and `inner_sea_races`'s
   `Reference Tiefling ~ Maw or Claw (%LIST)`** unresolved-reference failures: both book-wide
   defects in the pinned PCGen oracle checkout's own data, outside this repo's write scope to fix
   directly — worth escalating to whoever owns the oracle pin, since every unit from either book in
   any future SKILL/other lane will hit the same wall.
3. **`ultimate_psionics`'s `Could not find campaign` failure** (3 named units here, matching
   `AT-33-E5-002`'s own 4): still not root-caused past "ruled out `GAMEMODE`/`PRECAMPAIGN`/
   `BOOKTYPE` mismatch" — a genuine open question for whoever next touches this population.
4. **`ring_self_sufficiency`'s anomaly** (`compute_general_effect` returns `None` despite a real
   chain): still unexplained, independently re-confirmed this cycle, not root-caused further.
5. **The 14 chain-bearing `equipment_modifier` units** and the **331 `VAR`/`COMBAT`/`WEAPON`/
   `WEAPONPROF=*`-shaped units** remain the sibling lanes' own scope (or, if excluded there too, a
   real remaining gap for whichever cycle finalizes Epic 5).
