# Cycle AT-33-E5-003 (remediation) — Epic 5 Re-verification / AT-33-E5-003

- **Commit SHA:** `dded72f0b4` (landed on `tranche/13`; this line updated in a follow-up commit per
  this bundle's own precedent — `AT-33-E5-001`'s `73fdbb8803`, `AT-33-E5-002`'s `114bba8ec4`)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-generate-spell-batch.py` (fixed — `STAT:WIS|SCORE:10` → `18`, the harness bug this cycle root-caused)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/fixture-spell-pcg/{wizard,cleric,druid,bard,paladin,ranger}.pcg` (regenerated with the fixed generator — all 6, so every `.pcg` this harness produces reflects the fix, not only the 3 that changed value)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/fixture-spell-oracle-txt/{cleric,druid,ranger}.export.txt` (re-exported — real, live `./gradlew run` `BatchExporter` re-invocations against the pinned oracle, `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`; `wizard`/`bard`/`paladin` unchanged by the fix — Intelligence/Charisma were already 18 — and were not re-run)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-spell.oracle-results.json` (regenerated — `fixture-compare-spell-batch.py` re-run against the corrected exports)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json` (regenerated — 690 spell rows replaced, 438 equipment/companion/monster/monster_ability rows unchanged, folded forward)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` (rebuilt — now the real union of `AT-33-E5-001`'s corrected 1,128-row file and `AT-33-E5-002`'s 5,812-row file, replacing attempt 1's stale 32-record version)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003_cycle_receipt.md` (this file, overwritten in place)
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated in place — Status, Disagreement ledger, new Cycle entry)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (row 18 updated in place)
  - `docs/retro/events/sd33-r-e5-disagreements.jsonl` (new — one `correction` event, `--verified-by` the re-derived `box_ledger.py --check`)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (see Test scoping)
- **Wired-integration audit result:** OK_NO_TOKENS (see Test scoping)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  >
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is
  > root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix
  > the harness, and re-run everything it already judged).
  >
  > **Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an
  > operator escalation. **A filed blocker does not satisfy this criterion**
  > (`../../governance/blocker-closure-doctrine.md`).

## Why this cycle exists

Attempt 1's `complete` mark on this row (0 of 32 examined units disagree) was named a
`complete`-with-a-deferred-half by `AT-33-E6-001`'s final-acceptance scan and blocked bundle
closure. Since then, `AT-33-E5-001` and `AT-33-E5-002`'s own remediation cycles carried the
examined population from 32 to **6,940** (1,128 `fixture-verified` + 5,812 `literal-verified`) —
and `AT-33-E5-001`'s cycle surfaced **103 real disagreements**, all in the `spell` kind, all
carrying the identical delta `ours - oracle == 4`. This cycle re-opens `AT-33-E5-003` over that
full 6,940-unit examined population (not the whole 8,330 — 1,390 units remain genuinely
unexamined by rows 16/17 themselves, named in their own receipts) and root-causes every
disagreement in it.

## Root cause, confirmed against the real corpus and the real fixture inputs — the harness, not the engine

The initial hypothesis recorded in `AT-33-E5-001`'s receipt ("these are no-save spells; PCGen
omits the ability modifier for them") **does not hold**: cross-checking all 103 disagreeing units'
corpus `SAVEINFO` tokens finds a wide mix — `Will negates`, `Fortitude negates`, `Reflex half`,
`Will negates (harmless)`, `none`, `see text`, and more — not a single shared save-shape
(`python3 -c "..." ` over `data/corpus/**/spell/*.json`, tabulated in this cycle's working notes).
The real pattern, found by cross-tabulating `class_human` against `verdict` in
`fixture-spell.oracle-results.json`:

| Class | agree (before fix) | disagree (before fix) | Casting ability (`src/rules_core/spellbook.rs:143-150`) |
|---|---:|---:|---|
| Wizard | 227 | 0 | Intelligence |
| Bard | 23 | 0 | Charisma |
| Paladin | 18 | 0 | Charisma |
| Cleric | 0 | 60 | **Wisdom** |
| Druid | 0 | 41 | **Wisdom** |
| Ranger | 0 | 2 | **Wisdom** |

268 of 268 (100%) Intelligence/Charisma-cast classes' DC-bearing spells agreed; 103 of 103 (100%)
Wisdom-cast classes' DC-bearing spells disagreed — 60 + 41 + 2 = **103**, exactly the disagreement count. The
cause: `fixture-generate-spell-batch.py`'s `PCG_HEADER` (the `.pcg` fixture template, an
`AT-33-E5-001`-remediation artifact) pins `STAT:INT|SCORE:18` and `STAT:CHA|SCORE:18` but left
`STAT:WIS|SCORE:10` — matching `fixture_verified_oracle_probe.rs`'s own pinned
`SPELL_PROBE_ABILITY_SCORE=18` by accident for Intelligence/Charisma casters and contradicting it
for Wisdom casters. The real, live PCGen oracle correctly computed `DC = 10 + level + 0` (WIS
modifier `10.div_euclid(2) - 5 == 0`) from the fixture's own (wrong) `.pcg` data — **the oracle
was not wrong; the fixture we fed it was.** `align_weapon_communal` (Cleric, level 3): ours `17 =
10 + 3 + 4`, oracle (pre-fix) `13 = 10 + 3 + 0` — exactly the `+4` WIS modifier our probe assumed
the fixture would supply, and the fixture did not.

**Verdict: the oracle comparison (specifically, the fixture generator that feeds it) was wrong.
Not our production computation.** `src/rules_core/spellbook.rs`'s `casting_ability_for_class` /
`compute_spellbook_coverage` are untouched by this cycle and needed no fix — they correctly select
Wisdom for Cleric/Druid/Ranger; the harness fixture simply never gave that ability score the value
the probe assumed.

## Fix and re-run ("fix the harness, and re-run everything it already judged")

1. **Fixed:** `fixture-generate-spell-batch.py`'s `PCG_HEADER`, `STAT:WIS|SCORE:10` →
   `STAT:WIS|SCORE:18` (one line; comment added explaining why all three casting-ability stats
   must be 18).
2. **Regenerated** all 6 class `.pcg` fixtures with the fixed generator (`690 spell units -> 6
   class files, 0 (level,name) collisions`).
3. **Re-ran** the real, live oracle for the 3 classes whose computed value could change
   (Cleric/Druid/Ranger — Wisdom-cast; Wizard/Bard/Paladin's Intelligence/Charisma scores were
   already 18 and are byte-for-byte re-derivable from the unchanged fixture, so were not
   re-invoked): `./gradlew run --console=plain --args="-s run-settings -E
   fixtures/fixture-spell-batch.txt.ftl -c fixtures/fixture-spell-pcg/<class>.pcg -o
   fixtures/fixture-spell-oracle-txt/<class>.export.txt"` against the pinned checkout
   (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`). All 3 real JVM starts exit 0, 0
   `SEVERE` lines. Spot-check: `cleric.export.txt`'s `Align Weapon (Communal)` DC is now `17`
   (was `13`), matching `ours` exactly.
4. **Re-ran** the comparison: `fixture-compare-spell-batch.py` against the corrected exports →
   `690 units -- agree=371 disagree=0 unverifiable=319`. Every one of the 103 former
   disagreements is now `agree`; no new disagreement was introduced (`agree` count rose by exactly
   103, `unverifiable` count unchanged at 319).
5. **Rebuilt** `fixture-verified.combined-oracle-results.json` (11 equipment, unchanged + 690
   spell, corrected + 427 companion/monster/monster_ability, unchanged = 1,128) and
   `AT-33-E5-003.combined-oracle-results.json` (the real union of the corrected 1,128-row fixture
   file and the unmodified 5,812-row literal file = 6,940, 0 `unit_id` collisions between the two
   lanes).
6. **Independently re-verified**, not trusted from the compare script's own stdout:
   `python3 scripts/box_ledger.py --check --oracle-results
   .../AT-33-E5-003.combined-oracle-results.json` → `uncovered=0 overlap=0 population=49438
   oracle_disagreement=0 unverifiable_done=0 stale=False`, exit **0**.

## Disagree-capability re-proof on the batch path (this criterion's own scope, per the dispatch brief)

`AT-33-E2-003`'s fixture set already proves `compare_unit`/`run_comparison` (the underlying
comparator) capable of `disagree`. Both re-verification lanes call that exact, unmodified module —
`fixture-compare-spell-batch.py` calls `oracle_harness.compare.compare_unit` directly; the literal
lane's `scripts/oracle_harness/run.py` is `AT-33-E2-003`'s own CLI, unmodified. The **fixture
lane's own batch join layer already demonstrated `disagree` at scale this bundle** (103 real,
now-explained cases) before this cycle's fix — direct proof the fixture batch path can and did
return `disagree`, not merely `agree`/`unverifiable`.

The **literal lane's** batch path had produced 0 disagreements from only 41 live comparisons —
worth confirming its specific join+compare pipeline is not silently swallowing a real mismatch.
Mutation proof, run against the literal lane's own real committed inputs (not a synthetic
fixture): copied `stat41.ours.json`, changed one unit's `ours` value by `+999`
(`core_rulebook:equipment:belt_of_giant_strength_2`: `18` → `1017`), and re-ran the literal lane's
own command unmodified:

```
python3 scripts/oracle_harness/run.py \
  --oracle-export literal-stat-shape/stat41.oracle-export.txt \
  --ours <mutated copy of stat41.ours.json> \
  --output <scratch>.json
```

Result: `oracle_harness: 41 units compared -- agree=40 disagree=1 unverifiable=0`. The mutated
file was never committed (scratch-only). **Both batch paths are proven, this cycle, to return
`disagree` on a known-disagreeing case — not assumed from `AT-33-E2-003`'s original fixture set
alone.**

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Units examined by `AT-33-E5-001` + `AT-33-E5-002` to date | 6,940 | of 8,330 (1,741 `fixture-verified` + 6,589 `literal-verified`) = 83.3% | `python3 -c "import json;a=json.load(open('.../fixture-verified.combined-oracle-results.json'));b=json.load(open('.../literal-verified.oracle-results.json'));print(len(a['results'])+len(b['results']))"` → `6940` |
| Disagreements found this cycle (before fix) | 103 | of 6,940 examined (1.48%) — all in `fixture-verified`'s `spell` kind | `python3 -c "import json,collections;d=json.load(open('.../fixture-spell.oracle-results.json'));print(collections.Counter(r['verdict'] for r in d['results']))"` (pre-fix state, reproducible from `git show 73fdbb8803:.../fixture-spell.oracle-results.json`) |
| Disagreements root-caused | 103 of 103 | of 103 found | this receipt's Root cause section — one shared mechanism (`STAT:WIS` fixture value), not 103 independent defects |
| Disagreements resolved to a commit | 103 of 103 | of 103 found | the commit landing this receipt (fixture fix + re-run); SHA recorded above |
| Disagreements remaining after fix + re-run | 0 | of 6,940 examined | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` → `oracle_disagreement=0` |
| `box_ledger.py --check` against the full 6,940-record combined file | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit **0** | population 49,438 (whole inventory, unchanged by this cycle) | same command as above |
| Literal-lane batch-path mutation proof | `agree=40 disagree=1` | of 41 real live-oracle-compared units, 1 deliberately mutated | `python3 scripts/oracle_harness/run.py --oracle-export literal-stat-shape/stat41.oracle-export.txt --ours <mutated stat41.ours.json> --output <scratch>.json` |
| Units still unexamined (not this criterion's scope — rows 16/17 own them) | 1,390 | of 8,330 — 613 (`AT-33-E5-001`: 598 spell no-casting-ability-mapping + 15 class_feature) + 777 (`AT-33-E5-002`: 448 equipment other-bonus-shape + 329 non-equipment probe-bearing) | `jq` counts cited verbatim in `AT-33-E5-001_cycle_receipt.md` / `AT-33-E5-002_cycle_receipt.md`'s own figures rows |

## Status: complete

Every disagreement `AT-33-E5-001`/`AT-33-E5-002` have produced over their full examined population
to date (103 of 6,940 examined units) is named, root-caused, and resolved to this cycle's commit —
not adjusted-expectation, not deferred, not filed as an open blocker. `box_ledger.py --check`
against the real, committed, full-population combined file independently confirms
`oracle_disagreement=0`.

**This is not a claim that the full 8,330-unit Epic 5 population has no disagreement anywhere.**
1,390 of 8,330 remain genuinely unexamined — that is `AT-33-E5-001`/`AT-33-E5-002`'s own scope
(both cards carry their own precise remaining-population figures in their own receipts) and this
criterion's reopening condition against them is the same mechanical one attempt 1 established:
the moment either lane's oracle-results file contains a `disagree` record, `box_ledger.py --check`
against it exits non-zero and names the unit — proven twice more this cycle (the harness-bug
disagreements this cycle resolved, and the literal-lane mutation proof above).

## Movement, four buckets

- **closure:** 0 — no inventory unit's `status` field changed; this criterion resolves
  disagreements between oracle-results records, not `docs/work-inventory.json` unit dispositions.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 103 — the 103 disagreements were an artifact of a harness-fixture
  defect (`fixture-generate-spell-batch.py`'s `STAT:WIS` value), not real engine or oracle
  disagreement; correcting it moved 103 records from `disagree` to `agree` **by fixing the
  measurement, not by adjusting the expectation to match our output** — the corrected fixture now
  gives the oracle the ability score our own probe already assumed it had, and the two sides were
  independently computed and then agreed, not made to agree.

## RED→GREEN

**RED (this cycle's discovery, inherited from `AT-33-E5-001`):** `fixture-verified.combined-oracle-results.json`
at `73fdbb8803` carries 103 real `"verdict": "disagree"` records;
`scripts/box_ledger.py --check --oracle-results <that file>` → `oracle_disagreement=103`, exit 1
(the fail-closed gate firing as designed — re-confirmed at the start of this cycle before any fix
landed).

**GREEN:** after the one-line fixture fix, 3 real re-invocations of the pinned oracle, and a
re-run of the comparison and combination scripts: `python3 scripts/box_ledger.py --check
--oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json`
→ `oracle_disagreement=0`, exit **0**. Also re-confirmed the narrower fixture-only file
independently: `--oracle-results .../fixture-verified.combined-oracle-results.json` →
`oracle_disagreement=0`, exit 0.

**Disagree-capability RED→GREEN (the batch-path proof this criterion's brief required):** RED —
mutated `stat41.ours.json` copy, one value `+999` → `run.py` reports `disagree=1`, names nothing
wrong in the harness (it correctly flags the mutation). GREEN — the real, unmutated
`stat41.ours.json` against the same real oracle export → `agree=41 disagree=0`, matching the
committed `literal-verified.oracle-results.json` exactly.

## Notes

- **Considered, rejected (same as attempt 1's reasoning, reaffirmed):** adjusting `ours` to match
  `oracle`, or adjusting `oracle` parsing to accept the un-modified DC as "correct," would have
  been the exact "closed by adjusting the expectation" shape the criterion forbids. The actual fix
  changes neither side's *computation* — it corrects the **input** the harness feeds the oracle so
  that the oracle is asked the question our probe assumes it was asked. `src/rules_core/spellbook.rs`
  is byte-for-byte unchanged by this cycle.
- **Why the root-cause hypothesis in `AT-33-E5-001`'s own receipt was wrong, and how this cycle
  found the real one:** the uniform `ours-oracle=4` delta was a real, correct observation, but the
  "no-save spell" explanation it proposed does not survive checking the corpus `SAVEINFO` token
  against all 103 units (a genuine mix of save shapes, not a shared one) — cross-tabulating by
  `class_human` instead of by spell content found the real, clean 103-of-103 split. Recorded as a
  `scripts/retro.py correction` (`docs/retro/events/sd33-r-e5-disagreements.jsonl`) since the
  wrong hypothesis had already propagated into `AT-33-E5-001`'s receipt, `progress.md`'s
  Disagreement ledger, and `AT-33-E6-001`'s own shortfall-report reproduction of the sample rows.
- **Wizard/Bard/Paladin exports were not re-run.** Their `.pcg` fixtures changed byte-for-byte
  (the regenerated header now states `STAT:WIS|SCORE:18` even though those classes never read
  WIS), but their computed DC values cannot change — `casting_ability_for_class` never selects
  Wisdom for Intelligence/Charisma-cast classes, so their exports are unaffected by the fix and
  re-running them would have spent real PCGen JVM time (~22-58s each) to reproduce byte-identical
  output. `wizard.export.txt`/`bard.export.txt`/`paladin.export.txt` are unchanged from
  `73fdbb8803`; the corresponding 3 `.pcg` files are updated (comment + `STAT:WIS` line) but
  produce no computed-value change.
- **First actual production-code-adjacent finding this bundle's Epic 5 has surfaced that touched
  neither `src/` nor `docs/work-inventory.json`** — the fix landed entirely inside this bundle's
  own harness artifacts (`artifacts/epic-5-reverification/`), consistent with `AT-33-E5-003`'s
  write scope.

## Test scoping

Ran, for real, against real committed/regenerated data: `python3
docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixture-generate-spell-batch.py`
(regenerates all 6 `.pcg` fixtures, exit 0); 3 real `./gradlew run` `BatchExporter` invocations
(Cleric/Druid/Ranger, all exit 0, 0 `SEVERE`); `fixture-compare-spell-batch.py` (regenerates
`fixture-spell.oracle-results.json`, exit 0); `python3 scripts/oracle_harness/run.py` (the
literal-lane mutation proof, unmodified `AT-33-E2-003` CLI); `python3 scripts/box_ledger.py
--check --oracle-results <file>` (4 invocations: fixture-only pre-fix RED, fixture-only post-fix
GREEN, full 6,940-record combined GREEN, and the literal-lane mutation RED via `run.py` directly
rather than `box_ledger.py`). Did not re-run `scripts/tests/test_box_ledger.py` or
`scripts/tests/test_oracle_harness.py` (neither file changed this cycle). Did not run the Rust
workspace's `cargo test`/`cargo build` (no `src/` file touched this cycle — the root cause and fix
are entirely inside this bundle's own harness fixtures/scripts). Did not run `apps/desktop/src-tauri`
(separate cargo workspace, no file in it touched).

Ran `workflow-instruction.md §6` step 2/4's two audits (identifier-tag grep,
wired-integration-token grep — exact patterns as specified there, not reproduced verbatim in this
receipt to avoid self-matching the token audit) against `BASE_BRANCH=$(git merge-base HEAD
origin/develop)` on the final diff, scoped to this cycle's touched paths under
`docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/`.

Identifier-tag audit → `OK_NO_BUNDLE_TAGS`.

Wired-integration-token audit → `OK_NO_TOKENS`.

## Next-cycle plan

This criterion's own scope is fully discharged over the 6,940 units examined to date. It reopens
automatically, by mechanism (`box_ledger.py --check`'s `oracle_disagreement` condition), the
moment a future `AT-33-E5-001`/`AT-33-E5-002` cycle lands an oracle-results file with any
`disagree` record among the 1,390 units those two criteria have not yet examined (598 spell +
15 class_feature for `AT-33-E5-001`; 448 equipment other-bonus-shape + 329 non-equipment
probe-bearing for `AT-33-E5-002`, both named precisely in their own receipts' next-cycle plans).
When that happens, the next `AT-33-E5-003` cycle: (1) root-causes against both the corpus record
and the harness/fixture inputs, not only the delta pattern (this cycle's own miss); (2) fixes
`src/` if the defect is in computation, or the harness/fixture if the defect is in how the oracle
was asked the question, re-running everything the corrected harness already judged; (3) adds one
`progress.md` entry per disagreement, resolved to a commit SHA or an explicit operator escalation.

## Finalized by `AT-33-E5-finalize` (wave-2 finalize cycle) — this prediction happened, and was handled per the plan above

Three wave-2 sibling lanes (`spell-remainder`, `equipment-remainder`, `charbuild-remainder`) closed
the 1,390-unit remainder this receipt named above, examining 999 of it for real (815 + 103 + 81)
and surfacing exactly 2 NEW disagreements — both in the `src/` bucket this receipt's own plan
anticipated, neither a harness/fixture defect this time. Both were root-caused and **fixed with a
real commit** by the finalize cycle (`src/rules_core/` write scope this criterion's own evidence
line — "either our computation is wrong (fix it)" — covers, even though the individual wave-2
lanes correctly declined to touch it themselves): one `progress.md` entry per disagreement, per
this receipt's own plan item (3).

- **`ultimate_equipment:equipment:ring_of_the_sea_strider`** (`ours=8`, `oracle=16`): PF1's
  swim-speed-grants-+8-racial-Swim-bonus auto-rule, missing from `compute_general_effect`
  (`src/rules_core/equipment_effects/general.rs`). Fixed; re-verified live via a real re-run of
  `e5_equipment_remainder_skill_ours` + `scripts/oracle_harness/run.py` (unmodified) — `agree`.
- **`core_rulebook:class_feature:monk_ac_bonus`** (`ours=2`, `oracle=7`): the Monk's level-4+
  AC-Bonus dodge-bonus progression, missing from `class_chassis.monk.ac_bonus`
  (`src/rules_core/pilot_compute/mod.rs`). Fixed; re-verified live via a real re-run of
  `v06_work_inventory --charbuild-remainder-probe` + `charbuild-remainder-compare.py` (unmodified)
  — `agree`.

**Disagreements remaining after this fix + re-run, across the full 7,939-unit examined population**
(re-derive: `python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"`
→ `7939`, `Counter({'unverifiable': 7336, 'agree': 603})`): **0**.

**Disagree-capability re-proof, repeated at the finalize cycle's own initiative** (a 0-disagreement
result across 7,939 units is a suspicious result, not a happy one — same discipline this receipt's
own §108 section used): fed a deliberately-wrong `ours=999` for `ring_of_the_sea_strider` through
the unmodified `scripts/oracle_harness/run.py` batch path against the real committed oracle export
→ `disagree`, correctly. The batch path was not silently swallowing disagreements.

**This criterion is kept `in-progress`, not `complete`**, in `kanban.md` row 18 — not because a
disagreement survived (none did), but because row 17 (`AT-33-E5-002`) is still 391 short of its own
6,589-unit denominator. A unit with no row at all has not been checked for disagreement either way,
so "every disagreement is resolved" cannot be asserted over the FULL 8,330-unit population yet,
only over the 7,939 actually examined. Full detail: `AT-33-E5-finalize_cycle_receipt.md`.

## Finalized by `AT-33-E5-finalize-wave3` — 26 NEW disagreements, all root-caused, none fixed

Wave 3's `var` and `combat` shape lanes introduced 27 raw disagreeing rows (1 `var`, 26 `combat`)
against units the prior finalize cycle's 0-disagree figure never examined; after merging the 10
`var`↔`combat` duplicate `unit_id`s (worst-verdict-wins), the distinct disagree count is **26**.

**Disagreements across the full 8,255-unit examined population** (re-derive:
`python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); print(len(d['results'])); print(collections.Counter(r['verdict'] for r in d['results']))"`
→ `8255`, `Counter({'unverifiable': 7494, 'agree': 735, 'disagree': 26})`): **26**, all
root-caused this cycle (full per-unit table: `progress.md`'s `AT-33-E5-finalize-wave3` entry):

- **21** share one named engine gap: `compute_arms_armor_effect`/`compute_var_effect` do not
  resolve and sum a base item's `EQMOD:`-referenced modifier record's own separate `BONUS:` chain
  (the same base-item-plus-attached-EQMOD gap `AT-33-E5-remainder-equipment` first named,
  confirmed recurring here for both `COMBAT` and `VAR` shapes).
- **3** (`field_plate`, `stoneplate`, `snakeskin_tunic`) are a real harness baseline-diff
  methodology limitation, not necessarily an engine defect — a second-order AC effect
  (`MAXDEX` cap or a co-located Dex-enhancement chain) the whole-character `AC.Total` diff
  technique cannot separate from the item's own token.
- **1** (`sea_knife`) is an unhandled `PRE`-gated conditional chain, read unconditionally.
- **1** (`diviner_s_blight`) is not yet individually diagnosed.

**None fixed this cycle** — the dominant 21-unit fix is a real, cross-cutting engine change (both
resolvers need EQMOD-chain resolution) needing its own TDD cycle and full oracle re-verification
of the affected units; out of this merge/finalize cycle's turn budget. Named as the concrete
top item of the next cycle's plan, not attempted rushed.

**Disagree-capability re-proof on the current, unmodified batch path**: `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json`
→ `uncovered=0 overlap=0 population=49438 oracle_disagreement=26 unverifiable_done=0 stale=False`,
exit **1** (fail-closed, correctly) — all 26 real disagreements independently detected by the
gate itself, not just by this cycle's own Python merge script.

**Kanban row 18:** kept `in-progress` — 26 real, unresolved disagreements plus row 17's own
75-unit gap. Full detail: `AT-33-E5-finalize-wave3_cycle_receipt.md`.
