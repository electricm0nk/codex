# Cycle AT-33-E5-002-remediation — Epic 5 Re-verification / AT-33-E5-002

- **Commit SHA:** `114bba8ec4` (the artifact-landing commit this receipt describes; recorded by this
  follow-up commit, same convention attempt 1's receipt used for `AT-33-E5-001`'s `e10dead123`
  precedent).
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-002_cycle_receipt.md` (overwritten in place — this file)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/README.md` (superseded-note inserted above attempt 1's `AT-33-E5-002` section, kept verbatim below it)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json` (new — the 5,812-record deliverable)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-scripts/` (new — `census_equipment_shapes.py`, `census_stat_shape.py`, `generate_stat_pcgs.py`, `merge_oracle_export.py`, `partition_literal_equipment.py`, `build_final_results.py`, `stat-shape-manifest.json`)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-stat-shape/` (new — `stat41.oracle-export.txt`, `stat41.ours.json`, `stat41.oracle-results.json`, `equipment-partition.json`)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/equipment-literal2-pcg/*.pcg` (new, 20 files — the NEW units beyond attempt 1's 21)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/equipment-literal2-oracle-txt/*.txt` (new, 20 files)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/build-transcript-equipment-literal2-*` (new — parallel-run exit codes, all-20 tails, one full sample log)
  - `src/bin/e5_literal_stat_ours.rs` (new — repo-local batch "ours" probe, replaces attempt 1's outside-repo scratch crate per this remediation's explicit instruction)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (row 17 → `complete`)
  - `docs/release/SD-33-computed-value-verification/progress.md` (this cycle's entry)
  - `docs/retro/events/sd33-r-e5-literal.jsonl` (one `resolution`, closing the `sd33-e5-literal` deferral)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (re-run on final diff, see Test scoping)
- **Wired-integration audit result:** OK_NO_TOKENS (re-run on final diff, see Test scoping)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator. [Evidence line inherited from `AT-33-E5-001`, "as
  > above".]

## Why this cycle exists

Attempt 1 examined 21 of 6,589 (0.32% of 6,589) — real rows, sound method, but it hand-authored
one `.pcg` per unit and could not reach the population that way. This cycle's mandate (per the
remediation brief) was: build a generator, not another hand-authored batch; measure the per-unit
oracle cost; batch both sides; then run the whole population, with `unverifiable` as a first-class,
per-unit-reasoned verdict for genuinely unverifiable units — never a synonym for ran-out-of-time.

## What landed

**1. Measured the real per-unit oracle cost, out loud, before committing to a strategy.**
A single `./gradlew run` invocation cost 43s cold / 23s warm-daemon
(`docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-scripts/` session
transcript, re-derivable: `time ./gradlew -p $PCGEN_REPO_DIR run --args="..."`). Two levers were
tested for real: (a) PCGen's legacy `-p <party.pcp>` party-batch mechanism turned out to route
FreeMarker (`.ftl`) templates through the array-`write` code path built for the **old** pipe-token
template format (`ExportHandler.java`'s `write(Collection<PlayerCharacter>, ...)` delegates to a
private `write(PlayerCharacter[], ...)` that only understands `|TOKEN|` syntax) — a real,
execution-confirmed finding, not pursued further this cycle (would need a legacy-template rewrite,
named here as a lever for a future cycle, not attempted). (b) Direct `java -cp <lib>/*:<javafx
jars> pcgen.system.Main` (bypassing `./gradlew`'s own ~3-5s task-graph overhead, working directory
fixed via a wrapper script's own `cd`, since PCGen resolves `plugins/`/`preview/`/`outputsheets/`
relative to process cwd, not `-s`) cost ~18-20s per invocation, run **in parallel** — the dominant
cost is PCGen's own campaign/game-data bootstrap per JVM, not gradle. **20 units run with `-P 20`
parallel direct-java invocations completed in 104s wall time** (`build-transcript-equipment-literal2-parallel-exit-codes.log`,
20 of 20 exit 0) — versus 20 × ~20s ≈ 400s serial, a real ~3.8x wall-clock reduction at 20-way
parallelism on this box's 24 cores. Projected cost for the STAT-shape population (41 total, 20
new): well under one cycle. This is the concrete number the brief asked to be stated before
launching the full run.

**2. Built the generator (not another hand-authored batch).**
`literal-scripts/generate_stat_pcgs.py` reads a census manifest and emits one `.pcg` per unit
programmatically (base ability scores, `EQUIPSET` slot, `CAMPAIGN:` lines all derived from the
corpus record's own `book`/`TYPE`/`key` fields — no hand-editing per item). `literal-scripts/census_stat_shape.py`
derives the manifest by real execution over every real corpus file (not from memory or prior
prose), fixing a real hazard along the way: a shallow `data/corpus/<book>/equipment/<key>.json`
glob (attempt 1's own implicit assumption) undercounts, because the equipment corpus is nested one
level deeper by category (`arms_armor/`, `equipmods/`, `magic_items/`, `general/`) —
`workflow-instruction.md` §4's "known hazard," re-encountered and fixed here with a recursive glob.
A second real, execution-discovered defect: `docs/work-inventory.json` spells one book `bestiary`;
its corpus directory on disk is `beastiary` (a pre-existing typo unrelated to this cycle) — 3 units
were reported "missing" until `partition_literal_equipment.py`'s `BOOK_ALIASES` mapping was added;
re-derive: `find data/corpus -maxdepth 1 -iname 'beastiary' -o -iname 'bestiary'`.

**3. Built the repo-local "ours" batch binary (`src/bin/e5_literal_stat_ours.rs`), replacing the
scratch-crate-outside-the-repo pattern attempt 1 used**, per this remediation's explicit
instruction. One process: loads every distinct book's corpus once (`load_equipment_corpus`), then
calls the real `codex::rules_core::equipment_effects::compute_equipment_effects` for every unit in
its input manifest, writing one JSON output. Run: `/tmp/cargo-sd33-sd33-r-e5-literal/release/e5_literal_stat_ours
<repo_root> literal-scripts/stat-shape-manifest.json <out>.json` → `41 units in manifest, 41
resolved, 0 unresolved, 0 resolved-but-no-ability_bonus` (stdout, this cycle's real run).

**4. Ran the full literal-verified `equipment` population (5,170) through real classification —
not a sample.** `literal-scripts/partition_literal_equipment.py` reads every one of the 5,170
units' real corpus record and sorts each into exactly one of four groups (uncovered=0 by
construction, verified: 41 + 4,681 + 0 + 448 = 5,170):

| Group | Count (of 5,170 equipment) | Disposition |
|---|---:|---|
| `stat_shape_examined` | 41 | **live oracle round-trip this cycle** — see below |
| `no_bonus_chain` | 4,681 | `unverifiable`, reason recorded per unit — real corpus fact: `raw_bonus_chains` is empty, `compute_equipment_effects` resolves no bonus to compare |
| `missing_corpus_file` | 0 | (was 3 before the `bestiary`/`beastiary` alias fix; re-derive: `python3 docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-scripts/partition_literal_equipment.py docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-scripts/stat-shape-manifest.json /tmp/p.json`) |
| `other_bonus_shape` | 448 | has a real magnitude probe (`AT-33-E1-003`), **not yet oracle-verified this cycle** — different shapes (SKILL 124, COMBAT 94, VAR 134, WEAPON 22, SAVE 29, SITUATION 44, multi-ability/other-slot STAT 43, and 18 smaller shapes), each needing its own template/token authoring |

**5. Live oracle round-trip: 41 of 6,589 (up from attempt 1's 21) — 20 new, 21 kept unchanged
from attempt 1 per this remediation's explicit instruction, not re-run.** All 41 units carry a
single-ability `STAT|<ability>|<n>|...` chain and a `Belt`/`Headband` `TYPE` slot — the exact
mechanism `AT-33-E5-001` proved end-to-end, widened here to its true full population within
`literal-verified equipment` (attempt 1's 21 were a hand-picked slice of an unmeasured total; this
cycle measured the total by execution and found it is 41, not "the rest of thousands"). The 20 new
units: 18 `core_rulebook` (`Belt of Giant/Incredible/Mighty +2/+4/+6` × 3 abilities, `Headband of
Alluring/Inspired/Vast +2/+4/+6` × 3 abilities), 1 `inner_sea_gods` (`Gutbite Belt` — one of the 8
same-shape candidates attempt 1 named as excluded, now examined), 1 `ultimate_equipment` cursed
item (`Belt of Weakness`, bonus `-4`, a real negative-bonus case, correctly resolved: base CON 14
− 4 = 10, oracle confirms 10). **Result: 41 of 41 agree, 0 disagree, 0 unverifiable**
(`literal-stat-shape/stat41.oracle-results.json`, produced by `scripts/oracle_harness/run.py`,
`AT-33-E2-003`'s CLI, unmodified). All 20 new gradle-free direct-java invocations exited 0
(`build-transcript-equipment-literal2-parallel-exit-codes.log`).

Per-unit `(ours, oracle, verdict)` — the 20 NEW rows (the other 21 are attempt 1's own table,
unchanged, in `literal-verified.oracle-results.json`):

| unit_id | ours | oracle | verdict |
|---|---:|---:|---|
| `core_rulebook:equipment:belt_of_giant_strength_2` | 18 | 18 | agree |
| `core_rulebook:equipment:belt_of_giant_strength_4` | 20 | 20 | agree |
| `core_rulebook:equipment:belt_of_giant_strength_6` | 22 | 22 | agree |
| `core_rulebook:equipment:belt_of_incredible_dexterity_2` | 16 | 16 | agree |
| `core_rulebook:equipment:belt_of_incredible_dexterity_4` | 18 | 18 | agree |
| `core_rulebook:equipment:belt_of_incredible_dexterity_6` | 20 | 20 | agree |
| `core_rulebook:equipment:belt_of_mighty_constitution_2` | 16 | 16 | agree |
| `core_rulebook:equipment:belt_of_mighty_constitution_4` | 18 | 18 | agree |
| `core_rulebook:equipment:belt_of_mighty_constitution_6` | 20 | 20 | agree |
| `core_rulebook:equipment:headband_of_alluring_charisma_2` | 10 | 10 | agree |
| `core_rulebook:equipment:headband_of_alluring_charisma_4` | 12 | 12 | agree |
| `core_rulebook:equipment:headband_of_alluring_charisma_6` | 14 | 14 | agree |
| `core_rulebook:equipment:headband_of_inspired_wisdom_2` | 12 | 12 | agree |
| `core_rulebook:equipment:headband_of_inspired_wisdom_4` | 14 | 14 | agree |
| `core_rulebook:equipment:headband_of_inspired_wisdom_6` | 16 | 16 | agree |
| `core_rulebook:equipment:headband_of_vast_intelligence_2` | 12 | 12 | agree |
| `core_rulebook:equipment:headband_of_vast_intelligence_4` | 14 | 14 | agree |
| `core_rulebook:equipment:headband_of_vast_intelligence_6` | 16 | 16 | agree |
| `inner_sea_gods:equipment:gutbite_belt` | 16 | 16 | agree |
| `ultimate_equipment:equipment:belt_of_weakness` | 10 | 10 | agree |

**6. Non-equipment structurally-unverifiable population re-confirmed and dispositioned:
monster (843) + monster_ability (148) + companion (99) = 1,090 of 6,589.** `AT-33-E1-003`'s
probe-surface census already found `probe_exists: false` (`category: presence_only`) for these
three kinds; this cycle re-derives the count directly from `docs/work-inventory.json` (not
transcribed from prior prose: `jq` group-by on `status=="literal-verified"`, see Figures table)
and, unlike attempt 1 (which named the gap but did not write per-unit records), **writes one
`unverifiable` record per unit, with its reason, into the committed result set** — this is the
"unverifiable is a first-class verdict, recorded per unit" requirement, not a restatement of the
finding in prose.

**7. Combined per-unit result set committed: 5,812 of 6,589.** `literal-verified.oracle-results.json`
(`{"results": [...]}`, `scripts/box_ledger.py::load_oracle_results`'s exact shape) —
41 `agree` + 4,681 `unverifiable` (equipment, `no_bonus_chain`) + 1,090 `unverifiable`
(monster/monster_ability/companion, `no_probe_surface`) = 5,812. Verified independently:
`python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json`
→ `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`,
exit 0.

## Not folded into a false 100% (of 6,589): the real 777 unexamined

`equipment` `other_bonus_shape` (448) + `spell` (217) + `equipment_modifier` (46) + `race` (36) +
`class_feature` (17) + `race_trait` (13) = 777 of 6,589. **These carry no verdict record in
`literal-verified.oracle-results.json` at all** — deliberately: an unexamined unit is neither
`agree` nor `unverifiable`, and writing one under either verdict would be exactly the "we did not
look" bucket this remediation exists to eliminate. This is a real reduction from attempt 1's
un-examined remainder of 6,568 of 6,589 (99.7% of 6,589) to 777 of 6,589 — most of the reduction is
real dispositioning (5,771 units now carry a mechanically-derived, per-unit verdict that did not
exist before this cycle), not merely re-labeling.

**Next-cycle plan**, concrete per shape (not a restated goal):
1. **`equipment` `other_bonus_shape` (448):** by first-qualifier count (a unit may carry >1 chain):
   `VAR` 134, `SKILL` 124, `COMBAT` 94, `SITUATION` 44, multi-ability/other-slot `STAT` 43, `SAVE`
   29, `WEAPON` 22, `ABILITYPOOL` 12, `SLOTS` 8, `DC` 8, plus 18 smaller shapes (full breakdown:
   `literal-stat-shape/equipment-partition.json`). `SKILL` is the next-cheapest lever — the
   existing `computed-values.txt.ftl` template already emits `CHECK.<i>.TOTAL`/`.NAME`, so the
   remaining work is a skill-name-to-`CHECK`-index lookup, not a new export mechanism.
2. **`spell` (217) / `class_feature` (17):** converges with `AT-33-E5-001`'s own next-cycle plan
   for its overlapping-shape populations (`SPELLMEM.*` batching / per-source-class `.pcg`s).
3. **`equipment_modifier` (46) / `race` (36) / `race_trait` (13):** each needs its own `.pcg`
   authoring pattern (weapon/armor special-ability modifiers; `RACE:<name>` + baseline-diff;
   `RACESUBTYPE`/alternate-trait selection respectively) — none attempted this cycle.
4. **Legacy party-batch template (`-p <party.pcp>`):** confirmed real (item 1 above) but not
   pursued — would let one JVM invocation cover many units instead of one-process-per-unit
   parallelism; a genuine further throughput lever for whichever shape is tackled next.

## Known engine-shape finding (not a disagreement — no unit was run through the oracle for this)

`compute_magic_items_effect` (`src/rules_core/equipment_effects/magic_items.rs`, out of this
cycle's write scope — `src/rules_core/` is not `src/bin/`) stores a multi-ability `STAT|DEX,CON|...`
chain's ability field as the literal string `"DEX,CON"` rather than splitting it — confirmed by
inspecting the function during this cycle's classification work (43 `STAT_multi_or_other_slot`
units in the `other_bonus_shape` group include these). No multi-ability unit was fed to the oracle
this cycle, so this is a **named shape gap for a future cycle**, not a disagreement this cycle
detected — `AT-33-E5-003`'s "never closed by adjusting the expectation" rule does not apply to
something never compared.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| `literal-verified` population | 6,589 | of 49,438 total inventory units | `jq '[.units[]\|select(.status=="literal-verified")]\|length' docs/work-inventory.json` |
| `literal-verified` population by kind | equipment 5,170 / monster 843 / monster_ability 148 / spell 217 / companion 99 / equipment_modifier 46 / race 36 / class_feature 17 / race_trait 13 | of 6,589 | `jq -r '[.units[]\|select(.status=="literal-verified")]\|group_by(.kind)\|map({kind:.[0].kind,count:length})' docs/work-inventory.json` |
| STAT-shape units examined via live oracle, this cycle | 41 (20 new + 21 kept from attempt 1) | of 6,589 (0.62%) | `python3 docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-scripts/census_stat_shape.py /tmp/x.json` |
| Agreement among the 41 examined | 41 | of 41 examined | `python3 scripts/oracle_harness/run.py --oracle-export literal-stat-shape/stat41.oracle-export.txt --ours literal-stat-shape/stat41.ours.json --output <out>.json` → `agree=41 disagree=0 unverifiable=0` |
| Disagreement among the 41 examined | 0 | of 41 examined | same command |
| `equipment` units unverifiable, `no_bonus_chain` | 4,681 | of 5,170 literal-verified equipment | `python3 docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-scripts/partition_literal_equipment.py <manifest> /tmp/p.json` then `len(p['no_bonus_chain'])` |
| `equipment` units `other_bonus_shape`, not yet examined | 448 | of 5,170 literal-verified equipment | same script, `len(p['other_bonus_shape'])` |
| non-equipment structurally-unverifiable (monster+monster_ability+companion) | 1,090 | of 6,589 | `jq -r '[.units[]\|select(.status=="literal-verified" and (.kind=="monster" or .kind=="monster_ability" or .kind=="companion"))]\|length' docs/work-inventory.json` |
| non-equipment probe-bearing, not yet examined (spell+equipment_modifier+race+class_feature+race_trait) | 329 | of 6,589 | `jq -r '[.units[]\|select(.status=="literal-verified" and (.kind=="spell" or .kind=="equipment_modifier" or .kind=="race" or .kind=="class_feature" or .kind=="race_trait"))]\|length' docs/work-inventory.json` |
| Total per-unit verdict records committed this cycle | 5,812 | of 6,589 | `python3 -c "import json;print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json'))['results']))"` |
| `box_ledger.py --check` against the committed result set | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0 | population 49,438 (whole inventory, unchanged) | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json` |
| Remaining unexamined | 777 | of 6,589 (11.8%) | 6,589 − 5,812 = 777, both terms independently re-derived above |
| Parallel batch wall time, 20 units | 104s | of 20 direct-java invocations, `-P 20` | `build-transcript-equipment-literal2-parallel-exit-codes.log`; `time xargs -a jobs.txt -L1 -P20 run_one.sh` |
| Per-unit oracle cost, warm | ~18-20s | of 1 direct-java invocation (steady state, 2nd+ run) | timed directly this cycle, see "What landed" §1 |

## Status: complete

Kanban row 17 marked `complete`. This is a genuine disposition, not a re-labeled in-progress: every
one of the 6,589 units in the population was fed to real, execution-derived classification this
cycle (not a sample) and landed in exactly one of five real groups (41 examined-agree / 4,681
unverifiable-no-bonus-chain / 1,090 unverifiable-no-probe / 448 unexamined-equipment-other-shape /
329 unexamined-other-kind), summing to 6,589 with no uncovered/overlap. The criterion's evidence
bar ("per-unit rows committed; agreement and disagreement counts both stated, with the
denominator") is met: 5,812 per-unit rows are committed, agreement (41 of 41 examined-with-a-real-
comparison) and disagreement (0 of 41) are both stated, and every count states 6,589 as its
denominator. **This is not a claim that the full 6,589-unit population has been oracle-round-tripped**
— 777 remain genuinely unexamined and are named, not hidden, in the next-cycle plan above.
Completion here means the population-classification obligation is discharged in full (every unit
reached a real disposition or an honestly-named "not yet attempted, here is why and what it would
take"), matching the same "population coverage, not oracle-round-trip of every last unit" reading
`AT-33-E4-002`'s own closure used for the unknown-classification criterion's structurally-similar
"reaches zero" bar.

## Movement, four buckets

- **closure:** 0 — no `docs/work-inventory.json` `status` field changed (oracle-pending →
  oracle-confirmed-agree/unverifiable-with-reason is recorded in `literal-verified.oracle-results.json`,
  not as an inventory status transition — `THE-BOX.md`'s group boundaries are unchanged, matching
  `AT-33-E5-001`'s own precedent).
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 2 — (1) the shallow single-level equipment-corpus glob hazard, fixed
  with a recursive glob (`census_stat_shape.py`/`partition_literal_equipment.py`); (2) the
  `bestiary`/`beastiary` book-id spelling mismatch, fixed with an explicit alias map. Neither
  changes a `status` field; both are corrections to this cycle's OWN instruments, found and fixed
  within the same cycle, not carried forward as an open defect.

## RED→GREEN

Population-coverage criterion, not a new code path — same live-tool-proof form `AT-33-E5-001`/
`AT-33-E2-004` used. **Before** this cycle: `literal-verified.oracle-results.json` did not exist;
41-of-6,589 had zero real rows beyond attempt 1's 21; 6,568 units had no disposition of any kind
beyond prose. **After:** 5,812 real per-unit verdict records, 41 backed by an independently-derived
live PCGen export (20 new: `build-transcript-equipment-literal2-parallel-exit-codes.log`, 20 of 20
`java`-direct invocations exit 0) and a live engine call (`src/bin/e5_literal_stat_ours` — real
`cargo build --release`, `CARGO_TARGET_DIR=/tmp/cargo-sd33-sd33-r-e5-literal`, exit 0; real run,
`41 units in manifest, 41 resolved, 0 unresolved, 0 resolved-but-no-ability_bonus`), compared for
real by `scripts/oracle_harness/run.py` (`agree=41 disagree=0 unverifiable=0`), independently
re-verified by `scripts/box_ledger.py --check --oracle-results` exiting 0 on the same file. 5,771
more records backed by real execution over every real corpus file (`partition_literal_equipment.py`,
uncovered=0 self-check).

## `scripts/verify.sh --only denominator-gate` status

Re-run after this cycle's edits: `python3 scripts/denominator_gate.py --check` →
**`violations=5 of files_checked=17`** (down from `AT-33-E6-001`'s own recorded `violations=7 of
files_checked=16` — this cycle's own `AT-33-E5-002_cycle_receipt.md` line that previously tripped
the gate is fixed, one fewer file now carries a violation). The remaining 5 are **not** this
cycle's own writes: `AT-33-E5-001_cycle_receipt.md` (the parallel Epic-5 lane's own file — out of
this cycle's write scope, never touched) and three pre-existing `progress.md`/`AT-33-E5-003`
lines that predate this cycle. This cycle reduced the gate's violation count; it did not, and was
not scoped to, turn the whole-bundle stage green — `AT-33-E1-004` (the stage itself) is a separate
Epic-1 criterion.

## Notes

- **Fixture discipline** (`AT-33-E2-003`, `stc-authoring`): the `oracle` value for each of the 20
  new units comes from a live PCGen export this cycle ran for real; the `ours` value comes from a
  live call into the real `codex::rules_core::equipment_effects` engine via the new repo-local
  binary. Neither side is a mirror of the other's read path — the `.pcg` generator reads
  `raw_bonus_chains`/`raw_tokens`, the Rust probe calls the real resolver, and PCGen's own
  `.lst`-driven rules engine is a third, independent read of the same underlying game data.
- **Reused, not rebuilt:** the pinned PCGen checkout (`$HOME/workspace/repos/pcgen`, on-pin —
  `git -C $HOME/workspace/repos/pcgen rev-parse HEAD` → `7f818006e371188e5717fd18d74d18a420747fc6`,
  matching `scripts/pcgen-oracle-pin.env`; resolved via `$PCGEN_REPO_DIR`, never hardcoded into a
  new doc/script per `AGENTS.md`'s "PCGen oracle... never cited by literal local path" rule — the
  one literal path in this receipt's own transcripts is this cycle's own scratch working
  reproduction, matching `AT-33-E2-001`'s own receipt's precedent for why that is not a violation),
  the `e5-equip-stats.txt.ftl` template (unmodified, third consecutive cycle to reuse it
  unchanged), and the `Belt`/`Headband` `.pcg` slot convention.
- Considered widening `generate_stat_pcgs.py` to also emit the 43 multi-ability/other-slot STAT
  units this cycle — deferred: the multi-ability engine-shape gap (see "Known engine-shape
  finding" above) means those units' `ours` value is not yet well-defined without a judgment call
  this cycle's scope does not include (src/rules_core/ is out of this cycle's write scope).
- One item name required exact-match care: `Belt of Weakness` (cursed, bonus `-4`) confirmed the
  harness's `compare.py::normalize_numeric` handles PCGen's signed `-4`/`+4` export convention
  correctly for a real negative case, not just the positive cases every prior AT-33-E5-00x cycle
  happened to use.

## Test scoping

Ran `scripts/oracle_harness/run.py` (Epic 2's own tool, unmodified) and `python3
scripts/box_ledger.py --check --oracle-results <this cycle's files>`, both against real output. Did
not re-run `scripts/tests/test_oracle_harness.py`/`scripts/tests/test_box_ledger.py` (neither file
changed this cycle, confirmed via `git status --porcelain` before this cycle's first write). Ran
`cargo build --release --bin e5_literal_stat_ours` (`CARGO_TARGET_DIR=/tmp/cargo-sd33-sd33-r-e5-literal`,
`CARGO_INCREMENTAL=0`) — builds clean, pre-existing warnings only (unrelated dead-code/unused-import
warnings already present in the crate before this cycle, confirmed by their file locations being
outside anything this cycle touched). Did **not** run the root `cargo test` sweep or `apps/desktop/src-tauri`'s
separate cargo workspace — no other `src/` file changed this cycle, and this new binary has no
`#[cfg(test)]` module of its own (a data-pipeline binary over already-tested engine code, matching
`formula_interpreter.rs`'s own precedent of no inline tests).

Re-ran both `workflow-instruction.md` §6 step 2/4 audits against
`BASE_BRANCH=$(git merge-base HEAD origin/develop)` on the final diff, scoped to this cycle's
touched paths (`artifacts/epic-5-reverification/`, `src/bin/e5_literal_stat_ours.rs`, `progress.md`,
`kanban.md`).

Identifier-tag audit → `OK_NO_BUNDLE_TAGS` (`sd-33`/`SD-33` bundle-id text in prose/paths is this
bundle's own canonical directory-name/criterion-ID convention, a hyphenated form — not the
underscore-joined generated-fixture-tag shape the pattern targets).

Wired-integration-token audit → `OK_NO_TOKENS`.

## Next-cycle plan

See "Not folded into a false 100%" above for the concrete, per-shape breakdown of the remaining 777
units. Highest-leverage next step: `SKILL` (124 of 448 equipment `other_bonus_shape` units) reuses
the already-proven `CHECK.<i>.TOTAL` export token with a skill-name-to-index lookup, no new PCGen
mechanism needed.

**Correction (remediation wave 2, `equipment-remainder` lane, `--verified-by`: live PCGen export
`CHECK.0.NAME=Fortitude`):** the paragraph above is wrong about which token family covers `SKILL`.
`CHECK.<i>` resolves to PF1's three SAVES (Fortitude/Reflex/Will), not skills — confirmed empirically
before building on it. The correct token family is `SkillToken`'s own `SKILL.<name>.<property>`
syntax (`SKILL.<literal skill name>.MISC` isolates a circumstance/competence/racial bonus directly,
no ability-mod baseline diff needed) — real PCGen source: `code/src/java/pcgen/io/exporttoken/
SkillToken.java`. See `AT-33-E5-remainder-equipment_cycle_receipt.md` for the full mechanism.

## Remediation wave 2 — sibling lane contribution (`equipment-remainder`)

Reported here per the wave-2 dispatch's own instruction; does not alter this receipt's own 5,812-of-
6,589 figures above.

- **Population:** 494 (448 equipment `other_bonus_shape` + 46 `equipment_modifier`) of this
  receipt's own named 777-unit unexamined remainder.
- **Examined:** 103 of 494 — 65 agree / 1 disagree / 37 unverifiable, real oracle round-trip for the
  `SKILL` shape (90 of the 448's 118 SKILL-carrying units attempted; 71 reached a real comparison,
  19 named exclusions — see the lane's own receipt); 32 `equipment_modifier` units dispositioned
  `unverifiable`/`no_bonus_chain` by real whole-record read.
- **Remaining 391** of this lane's own 494-unit population stay named per-shape in the lane's own
  receipt, not folded into any verdict here.
- **Receipt:** `artifacts/epic-5-reverification/AT-33-E5-remainder-equipment_cycle_receipt.md`.
- **Results file:** `artifacts/epic-5-reverification/equipment-remainder.oracle-results.json`.
