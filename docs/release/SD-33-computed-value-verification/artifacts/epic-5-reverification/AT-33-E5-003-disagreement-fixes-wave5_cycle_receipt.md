# Cycle sd33-r5-disagreements — Epic 5 Re-verification / AT-33-E5-003 (last 4 disagreements)

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-r5-disagreements`; a small
  follow-up commit records it back into this receipt, per this bundle's own established
  precedent — `AT-33-E5-finalize-wave4`'s receipt did the same).
- **Files touched:**
  - `src/bin/e5_ac_isolator.rs` (new) — repo-local probe: (1) recomputes each AC-shape item's
    CURRENT `armor_class_bonus` via the real `compute_equipment_effects` (catches a row whose real
    fix landed in a commit later than the row was last written); (2) emits the exact set of PCGen
    bonus-`TYPE` labels that value is built from (mirrors `arms_armor::armor_class_bonus_from_bonus_chains`'s
    match/skip-Circumstance/first-match predicate read-only, for isolation — never a second compute
    path).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work-wave5/`
    (new — working data, kept for re-derivability, same convention every prior `AT-33-E5-00x` lane's
    `combat-shape-work/` used): `ac-isolate.txt.ftl` (the isolating BatchExporter template),
    `ac_isolate_run.py` (the batch driver), `e5_ac_isolator.output.json` (the 82-item fresh-`ours`+type
    probe output), `ac-isolate-txt/` (66 raw live PCGen exports, one per re-run unit).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/disagreement-fixes-wave5.oracle-results.json`
    (new — this cycle's primary deliverable, 4 rows, all `agree`).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/full-rerun-wave5.oracle-results.json`
    (new — the full 66-unit re-run evidence file, the harness-fix route's re-run-everything-it-judged
    obligation).
  - `docs/release/SD-33-computed-value-verification/progress.md` (Disagreement ledger + Cycles entry,
    prepended).
  - `docs/retro/events/sd33-r5-disagreements.jsonl` (new, 1 `correction`).
  - **`kanban.md` NOT touched this cycle** — per this wave's own coordination instructions, three
    sibling lanes are running concurrently on the 67 unrowed units and a dedicated finalize cycle owns
    the row 16/17/18 kanban call once all lanes land, so this cycle deliberately leaves rows 16-18
    exactly as it found them (still `in-progress`/`blocked-escalated` from wave 4) and records its own
    result in `progress.md` only, for the finalize cycle to read and reconcile.

- **Identifier audit result:**
```
$ grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' src/bin/e5_ac_isolator.rs \
    docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work-wave5/ac_isolate_run.py \
    docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work-wave5/ac-isolate.txt.ftl \
  || echo OK_NO_BUNDLE_TAGS
OK_NO_BUNDLE_TAGS
```

- **Wired-integration audit result:**
```
$ grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' src/bin/e5_ac_isolator.rs \
    docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work-wave5/ac_isolate_run.py \
    docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work-wave5/ac-isolate.txt.ftl \
  || echo OK_NO_TOKENS
OK_NO_TOKENS
```

- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  >
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is
  > root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the
  > harness, and re-run everything it already judged).
  >
  > **Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an
  > operator escalation. **A filed blocker does not satisfy this criterion.**

## What this cycle owns

The 4 disagreements wave 4 (`abc72f75ec`) named `baseline_diff_harness_limitation` and escalated:
`advanced_class_guide:equipment:full_plate_of_the_corpse`, `inner_sea_world_guide:equipment:field_plate`,
`inner_sea_world_guide:equipment:stoneplate`, `ultimate_equipment:equipment:snakeskin_tunic`.

## Establishing the diagnosis by execution, before changing anything

Read the already-committed raw exports first (no new PCGen run needed for this step):

```
$ cat .../combat-shape-work/ac-oracle-txt/full_plate_of_the_corpse.txt   -> AC.TOTAL=22
$ cat .../combat-shape-work/ac-oracle-txt/baseline_advanced_class_guide.txt -> AC.TOTAL=12
$ cat .../combat-shape-work/ac-oracle-txt/field_plate.txt   -> AC.TOTAL=18   (baseline_inner_sea_world_guide=12)
$ cat .../combat-shape-work/ac-oracle-txt/stoneplate.txt    -> AC.TOTAL=20  (baseline_inner_sea_world_guide=12)
$ cat .../combat-shape-work/ac-oracle-txt/snakeskin_tunic.txt -> AC.TOTAL=14 (baseline_ultimate_equipment=12)
```
Every one of the 4 recorded diffs (10, 6, 8, 2) reproduces exactly. Read each unit's corpus record
directly:
- `full_plate_of_the_corpse`: base `COMBAT|AC|9|TYPE=Armor` + `EQMOD:...Special Ability ~ +2 ~ Armor...`
  (a second, separately-resolvable `+2` `TYPE=ArmorEnhancement` record) + `MAXDEX:1`.
- `field_plate`/`stoneplate`: base `COMBAT|AC|7 or 9|TYPE=Armor`, no enhancement EQMOD, `MAXDEX:1`.
- `snakeskin_tunic`: `COMBAT|AC|1|TYPE=Armor` **and a second, independent**
  `STAT|DEX|2|TYPE=Enhancement` chain on the SAME record.

**Diagnosis test, run for real (`AT-33-E6-001`'s own two hypotheses):** is the gap (1) the diff
conflating a `MAXDEX`-cap Dex loss / co-located Dex-enhancement gain with the item's own bonus, or
(2) a base-armor/masterwork term counted on one side only? Cross-checking the corpus records against
the raw exports rules out (2) directly: neither `field_plate`/`stoneplate` carries any second
armor-value token at all (their only `COMBAT|AC` chain is the one base value), and
`full_plate_of_the_corpse`'s second value IS the item's own real `EQMOD`-referenced enhancement (a
real magnitude, not a duplicate-counting artifact). (1) is the real mechanism, live-confirmed below.

**The method itself is the defect, confirmed by execution, not assumed.** `e5_ac_isolator`
(`src/bin/e5_ac_isolator.rs`) recomputes each unit's CURRENT `armor_class_bonus` (via the real
`compute_equipment_effects` — the same general engine path every prior probe calls) and reports the
exact PCGen bonus-`TYPE` set it is built from:

```
$ cargo run --locked --bin e5_ac_isolator -- . \
    artifacts/epic-5-reverification/combat-shape-work/ac-manifest.json \
    artifacts/epic-5-reverification/combat-shape-work-wave5/e5_ac_isolator.output.json
e5_ac_isolator: 82 items, 3 unresolved ours, 3 with no AC type -> .../e5_ac_isolator.output.json
```
```
full_plate_of_the_corpse -> ours=11, types=[Armor, ArmorEnhancement]   (was 9 in the combined file — STALE, never re-run since abc72f75ec's general EQMOD resolver landed)
field_plate               -> ours=7,  types=[Armor]                    (unchanged)
stoneplate                 -> ours=9,  types=[Armor]                    (unchanged)
snakeskin_tunic             -> ours=1,  types=[Armor]                    (unchanged)
```
`full_plate_of_the_corpse`'s committed row (`ours=9`) was stale, not wrong-by-design: `abc72f75ec`'s
general `eqmod_referenced_records`/`apply_eqmod_armor_class_bonus` resolver already sums the EQMOD-
referenced `+2` for **any** item, unconditionally — it was simply never re-run for this specific unit
(wave 4's own manifest deliberately excluded it from the 22-unit compute-fix batch, moving it to the
harness bucket instead, per its own receipt). **No new `src/rules_core/` code is needed for this
unit** — only a fresh recompute with the code that already exists.

Built a new BatchExporter template, `ac-isolate.txt.ftl`, using PCGen's own `BONUS.COMBAT.AC.<Type>`
export token (`code/src/java/pcgen/io/exporttoken/BonusToken.java`, `pc.getBonusDueToType`) — a
**per-type bonus subtotal**, not a whole-character total — plus the composite isolator
`AC.ISOLATED=BONUS.COMBAT.AC.TOTAL.!BASE.!Ability.!Size` (negates the flat base, the Dex/Ability-to-AC
component, and size — leaving exactly Armor+Shield+enhancement+NaturalArmor+Deflection+Insight+
untyped, the SAME set `armor_class_bonus` is defined over). **No baseline character is used at all**:
this reads a bonus subtotal directly off the single-item character already committed under
`combat-shape-work/ac-pcg/`, so nothing about Dex or a `MAXDEX` cap can leak in structurally, by
construction — not because the baseline happens to avoid triggering the cap.

Live-run against the pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
repo-local checkout, `scripts/oracle_harness/charbuild_remainder_run_one.sh`, the SAME already-
committed `.pcg` fixtures, unmodified):

```
full_plate_of_the_corpse: AC.TOTAL=22 AC.ISOLATED=11 AC.BASE=10 AC.ABILITY=1 AC.SIZE=0 AC.ARMOR=9 AC.ARMORENH=2  (11 = 22-10-1-0; matches Armor+ArmorEnhancement exactly)
field_plate:               AC.TOTAL=18 AC.ISOLATED=7  AC.BASE=10 AC.ABILITY=1 AC.SIZE=0 AC.ARMOR=7
stoneplate:                 AC.TOTAL=20 AC.ISOLATED=9  AC.BASE=10 AC.ABILITY=1 AC.SIZE=0 AC.ARMOR=9
snakeskin_tunic:             AC.TOTAL=14 AC.ISOLATED=1  AC.BASE=10 AC.ABILITY=3 AC.SIZE=0 AC.ARMOR=1  (AC.ABILITY=3 = the Dex mod AFTER the item's own +2 enhancement — confirms the co-located Dex effect is real and is now correctly excluded, not merely assumed away)
```
`AC.ISOLATED` matches the per-type breakdown sum exactly in all 4 cases (cross-check, not assumed) and
matches this engine's fresh `armor_class_bonus` exactly in all 4 cases: **11=11, 7=7, 9=9, 1=1.**

## Root cause

`baseline_diff_harness_limitation` confirmed by execution: `combat-shape-work/ac_build_results.py`'s
`oracle_value = int(item_AC.Total) - int(baseline_AC.Total)` is the ONLY place in the whole
`scripts/` + `docs/release/SD-33-computed-value-verification/artifacts/` tree that derives an oracle
value this way for an AC/armor-shaped unit (`grep -rl "AC.TOTAL\|baseline_diff\|item_AC.Total"
scripts/oracle_harness docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/*.py`
→ only `combat-shape-work/ac_build_results.py`). A whole-character `AC.Total` diff cannot separate the
item's own `COMBAT|AC` bonus-chain magnitude from two real, distinct second-order effects that flow
through the SAME `AC.Total` number: a `MAXDEX` cap reducing the baseline's own Dex-to-AC contribution
once the item is worn (`field_plate`/`stoneplate`/`full_plate_of_the_corpse`), and a co-located
ability-score-enhancement chain on the same record raising `AC.Total` via the normal Dex-to-AC path
(`snakeskin_tunic`). **The method itself was the defect** — not our compute (which was already
correct for 3 of 4, and needed only a fresh recompute, not new code, for the 4th).

## Route taken: harness (not our-compute)

No `src/rules_core/` change lands this cycle. `armor_class_bonus` was already correct for all 4 units
under the code that already exists (`abc72f75ec`, wave 4). The fix is entirely in
`scripts/oracle_harness/` + this cycle's own working data: a new isolating template
(`ac-isolate.txt.ftl`) and driver (`ac_isolate_run.py`), replacing the flawed diff-based oracle
derivation for the AC-shape population.

## The second limb: re-run everything this harness already judged

**Scope of "already judged" by THIS harness, precisely, with its own denominator:**
`combat-shape-work/ac_build_results.py`'s diff method produced a real numeric `oracle` value for
**66 of its own 82-item manifest** (40 `agree` + 26 `disagree` in
`equipment-shape-combat.oracle-results.json`; the other 16 never got a numeric oracle at all — 14 hit
`oracle_harness_ultimate_psionics_campaign_load_failure`, 2 hit `engine_id_resolve_fails_templated_variant_record`
— so were never "judged" by the diff, and are unaffected/out of this cycle's scope). This 66-unit
population is a **different, smaller, precisely-bounded construct** than the bundle's 8,263-unit
grand total: confirmed by grep (above) that no other oracle-generation script in this bundle uses
`AC.Total`/baseline-diff at all — every other examined unit (SKILL/STAT/spell/VAR/direct-export
shapes) was never touched by this specific defect, so re-running them would not re-verify anything
this harness actually judged; it would just burn PCGen-invocation budget on an unrelated code path.
Re-running the 66 is the criterion's own re-run clause honored exactly, not narrowed.

**Full, real, live re-run of all 66** (`combat-shape-work-wave5/ac_isolate_run.py`, `--workers 8`,
same pinned oracle, same already-committed `.pcg` fixtures, no baseline character used):

```
$ python3 .../ac_isolate_run.py . .../ac-manifest.json .../equipment-shape-combat.oracle-results.json \
    .../e5_ac_isolator.output.json .../ac-isolate-txt .../full-rerun-wave5.oracle-results.json \
    .../disagreement-fixes-wave5.oracle-results.json --workers 8
ac_isolate_run: 66 already-judged units, 0 run failures, 0 unresolved isolated tokens
full re-run: 66 rows -- agree=66 disagree=0 unverifiable=0
oracle value moved vs prior diff-based oracle: 5 of 66
moved unit_ids: ['advanced_class_guide:equipment:full_plate_of_the_corpse', 'inner_sea_races:equipment:goblin_plate', 'inner_sea_world_guide:equipment:field_plate', 'inner_sea_world_guide:equipment:stoneplate', 'ultimate_equipment:equipment:snakeskin_tunic']
disagreement-fix rows written: 4
```

**66 of 66 re-run, 0 unresolved, 66/66 agree.** Full detail:
`docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/full-rerun-wave5.oracle-results.json`
(66 rows, 0 duplicate `unit_id`s: `python3 -c "import json,collections; d=json.load(open('...full-rerun-wave5.oracle-results.json')); ids=[r['unit_id'] for r in d['results']]; print(len(ids)-len(set(ids)))"` → `0`).

**A 5th unit moved, discovered only because the FULL re-run ran, not just the 4 named disagreements —
exactly why the re-run-everything clause exists.** `inner_sea_races:equipment:goblin_plate` was
recorded `agree` (`ours=9, oracle=9`) under the old diff method — but `ours=9` was ALSO stale (the
same never-re-run-after-`abc72f75ec` staleness as `full_plate_of_the_corpse`; goblin_plate has its own
`EQMOD`-referenced `+1 Armor` enhancement). The old diff-oracle (9) and the old stale `ours` (9)
happened to match by coincidence — a **double error that canceled**, not a real agreement. The full
re-run recomputes both in lockstep: fresh `ours=10` (9 base + 1 EQMOD enhancement, matching this
cycle's `e5_ac_isolator` output exactly), fresh isolated oracle `=10` (confirmed live,
`ac-isolate-txt/goblin_plate.txt`: `AC.ISOLATED=10`). **Still `agree`, correctly, at the true value** —
not a new disagreement, but a real instrument-correction this cycle is disclosing rather than hiding
(a coincidental-agreement risk this criterion's doctrine exists to catch).

**One driver-script defect found and fixed before committing (never shipped):** the first full-66 pass
recorded `advanced_race_guide:equipment:sea_knife` as `disagree` (`ours=None, oracle=0`) — not a real
defect. `armor_class_bonus_from_bonus_chains` correctly returns `None` for `sea_knife` (its only
`COMBAT|AC` chain is `TYPE=Circumstance`, excluded by wave 4's own fix), meaning "no chain applies" —
a real, honest zero contribution, the SAME semantic wave 4's own `disagreement-fixes-manifest.json`
already encoded via its `allow_none_ac` flag. `compare_unit`'s numeric branch does not fire on a bare
`None`, so the first pass fell into the string-equality branch (`"None" != "0"`) and manufactured a
false `disagree`. Fixed in `ac_isolate_run.py` (`ours_val = 0` when `None`, scoped to this exact,
already-established convention) and re-run: `sea_knife` now correctly `agree` (`0`/`0`), matching wave
4's own already-committed row. Live isolated confirmation:
`ac-isolate-txt/sea_knife.txt`: `AC.TOTAL=12 AC.ISOLATED=0 AC.BASE=10 AC.ABILITY=2` — the standing
(non-swimming/prone) test character genuinely has 0 Circumstance AC, independently confirming wave 4's
exclusion is correct on live data, not merely inferred.

## Disagree-capability re-proof on the current batch path

```
$ python3 scripts/box_ledger.py --check --oracle-results /tmp/probe-disagree-wave5.json   # scratch copy, never committed
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: ultimate_equipment:equipment:belt_of_mighty_hurling_greater
$ echo $?
1
```
A known-mutated row (one of the simulated-merge's own now-agreeing units, flipped to `disagree` with
`ours=999`) on the CURRENT batch path correctly returns `disagree`, exit 1. Probe file removed
immediately after (`rm /tmp/probe-disagree-wave5.json /tmp/sim-merged-wave5.json`), never committed.

## Before / after

```
$ python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=4 unverifiable_done=0 stale=False   # BEFORE, exit 1
ORACLE_DISAGREEMENT: advanced_class_guide:equipment:full_plate_of_the_corpse, inner_sea_world_guide:equipment:field_plate, inner_sea_world_guide:equipment:stoneplate, ultimate_equipment:equipment:snakeskin_tunic
```
Simulated merge (temp copy, `/tmp`, never committed — the real merge is the finalize cycle's own §5
job) of this cycle's 4 corrected rows into a copy of the combined file:
```
$ python3 scripts/box_ledger.py --check --oracle-results /tmp/sim-merged-wave5.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False   # AFTER, exit 0
$ echo $?
0
```
4 → 0, all four genuinely resolved (not reclassified, not dropped, not moved to `unverifiable`).

## Per-unit disposition

| unit_id | ours (was, stale in combined file) | oracle (was, diff-based) | ours (now) | oracle (now, isolated) | verdict | root cause | route |
|---|---:|---:|---:|---:|---|---|---|
| `advanced_class_guide:equipment:full_plate_of_the_corpse` | 9 | 10 | 11 | 11 | agree | `baseline_diff_harness_limitation` (MAXDEX cap) + a stale `ours` never re-run after `abc72f75ec` | harness (re-run, no new code) |
| `inner_sea_world_guide:equipment:field_plate` | 7 | 6 | 7 | 7 | agree | `baseline_diff_harness_limitation` (MAXDEX cap) | harness |
| `inner_sea_world_guide:equipment:stoneplate` | 9 | 8 | 9 | 9 | agree | `baseline_diff_harness_limitation` (MAXDEX cap) | harness |
| `ultimate_equipment:equipment:snakeskin_tunic` | 1 | 2 | 1 | 1 | agree | `baseline_diff_harness_limitation` (co-located Dex-enhancement chain) | harness |

Every commit for this table's "resolution" column is this cycle's own landing commit (see top of this
receipt) — `progress.md`'s Disagreement ledger entry carries the identical table with the commit SHA
filled in.

## Figures + their re-derive commands

- 4 disagreements in (of 8,263 examined) — `AT-33-E5-003.combined-oracle-results.json`'s own
  `disagree` count, `python3 scripts/box_ledger.py --check --oracle-results .../AT-33-E5-003.combined-oracle-results.json`
  → `oracle_disagreement=4`.
- 66 of 66 already-judged-by-this-harness units re-run (of the AC-shape lane's own 82-item manifest;
  16 never got a numeric oracle from this method, out of scope) —
  `python3 -c "import json; print(len(json.load(open('.../full-rerun-wave5.oracle-results.json'))['results']))"`
  → `66`.
- 0 of 66 re-run rows disagree — `python3 -c "import json,collections; print(collections.Counter(r['verdict'] for r in json.load(open('.../full-rerun-wave5.oracle-results.json'))['results']))"`
  → `Counter({'agree': 66})`.
- 5 of 66 oracle values moved (the 4 disagreements resolved + `goblin_plate`'s coincidental-agreement
  correction) — `ac_isolate_run.py`'s own `moved` list, printed above.
- 4 of 4 this cycle's own disagreement-fix rows agree —
  `python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/disagreement-fixes-wave5.oracle-results.json`
  → `oracle_disagreement=0`.
- 0 of 4 remaining bundle-wide (projected, simulated merge, not committed) — shown above.
- 70 of 70 `equipment_effects` tests still green, unchanged — `cargo test --locked --lib equipment_effects::`
  (no `src/rules_core/` file touched this cycle).

## Status: complete

`AT-33-E5-003`'s own bar: **all 4 disagreements resolved to this cycle's own commit** (not filed
under `## Open blockers`), the harness-fix route's re-run obligation honored over its own real,
precisely-bounded population (66 of 66, not a subset), the `disagree` capability re-proven live on
the current batch path, and the population-wide before/after shown by real command output.

**Row 18 (`AT-33-E5-003`) is NOT independently `complete` for the bundle** until Epic 5's other open
item (row 17, `AT-33-E5-002`, 67 of 8,330 units still unrowed — a disjoint, sibling-lane scope this
cycle does not touch) also closes; that is `AT-33-E6-001`'s own gate to re-check, not this cycle's
criterion. This receipt closes `AT-33-E5-003` on its own terms: 0 of 8,263 examined units disagree.

## Movement, four buckets

- **Closure:** 0 — no `docs/work-inventory.json` `status` field changed (oracle results live in this
  directory's own JSON files, matching every prior `AT-33-E5-00x` cycle's convention).
- **Reclassification:** 0.
- **Reachability:** 0 — no examined-population widening (the 66-unit population was already examined
  by the AC-shape lane; this cycle corrects the METHOD that judged it, not the examined count).
- **Instrument-correction:** 66 (the entire AC-shape already-judged population's oracle value
  re-derived by an absolute, non-diff method) — of which 5 values actually moved (4 disagreements
  resolved + `goblin_plate`'s coincidental double-error caught), 61 confirmed unchanged at their
  already-correct value.

## Notes

The brief's own diagnosis (two candidate causes: a Dex-cap/co-located-enhancement conflation, or a
masterwork/base-armor term counted on one side only) was tested by execution, not assumed: reading
each of the 4 corpus records directly ruled out the second cause (no double-counted base term exists
on any of the 4), and the live isolator run confirmed the first cause exactly, matching this engine's
fresh compute to the digit on all 4. The harness route was chosen over patching the diff arithmetic
(e.g. "add back 1 when `MAXDEX` caps") because a formula-level patch would be a second, parallel
implicit model of PF1's AC stacking rules living in a comparison script — fragile, and exactly the
kind of un-isolated inference `AT-33-E5-003`'s own doctrine (root-cause, don't patch the symptom)
warns against. The absolute per-type token, by contrast, asks PCGen's own engine for the isolated
quantity directly, with no arithmetic assumption about which components sum which way.

## Next-cycle plan

1. `AT-33-E5-002`'s 67 remaining unrowed units (owned by a disjoint sibling lane this cycle does not
   touch — see `AT-33-E5-last75_cycle_receipt.md`'s own shape table).
2. Once both this criterion and `AT-33-E5-002` are `complete`, `AT-33-E6-001` can re-run the
   final-acceptance scan as its next attempt.
3. Disclosed, not blocking: other `AT-33-E5-00x` lanes with a script named `*baseline*` (SKILL/STAT/
   spell shapes) were NOT audited this cycle for the same diff-vs-isolator risk class — confirmed by
   grep that none of them touch `AC.Total`/`baseline_diff` (a different quantity), so they are outside
   THIS criterion's 4-disagreement scope, but a future cycle auditing those shapes' own methodology
   for the same structural risk (whole-character-diff conflating an unrelated component) would be
   worthwhile.
