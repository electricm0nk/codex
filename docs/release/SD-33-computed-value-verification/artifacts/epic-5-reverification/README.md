# Epic 5 — Re-verification of the blessed

`AT-33-E5-001`'s population: the **1,741** `fixture-verified` units in `docs/work-inventory.json`
(`THE-BOX.md`'s `fixture-verified` group, disposition "fixture-verified, oracle-pending" —
verified against our own artifacts, never against PCGen, per `decisions.md` §7).

## Path A / Path B input (`AT-33-E2-004`)

Epic 2's closing receipt ruled **Path A**: the pinned PCGen builds headless and a real
character round-trips through `BatchExporter`. This cycle extends that proven mechanism —
the same `./gradlew run --args="-s run-settings -E <template.ftl> -c <input.pcg> -o <output>"`
invocation, the same `scripts/oracle_harness/` comparison code — from Epic 2's one
hand-authored fighter to a real, population-scoped batch.

`AT-33-E2-004`'s own receipt named the sizing question explicitly: "sizing and executing
that authoring effort across 8,330 units is `AT-33-E5-001`'s and `AT-33-E5-002`'s own
scope, not pre-decided here." This cycle answers a first slice of that sizing question by
execution, not by estimate.

## What this cycle actually re-verified against the live oracle

**11 of 1,741** — the entire `equipment` kind, the smallest and most tractable slice of the
population (fixed `BONUS:STAT|<ability>|<n>|TYPE=Enhancement` magic items, no character level
or class dependency, one PCGen equip-slot per item).

For each of the 11 items:

1. **`ours`** — the real, live output of `codex::rules_core::equipment_effects::compute_equipment_effects`
   (the same function `AT-33-E1-003`'s `probe_equipment_effect_wiring` calls to establish these
   units are engine-wired at all), run against the real `data/corpus/ultimate_equipment/equipment/`
   records via a small scratch Rust program (`ours-derivation/equipment-ours-probe.rs`, kept
   outside the codex repo tree — `AT-33-E5-001`'s granted write scope is this `artifacts/` directory
   plus Epic 2's harness plus an append-only `THE-BOX.md`, not `src/`; the program only *reads*
   `codex` as a Cargo path dependency, it writes nothing into the repo). Output:
   `ours-derivation/equipment-ours-probe.output.json`.
2. **`oracle`** — a real, live PCGen export. One `.pcg` per item (`fixtures/equipment-pcg/`,
   a Level-1 Human Fighter, `CAMPAIGN:Core Rulebook` + `CAMPAIGN:Ultimate Equipment`, the one
   item `EQUIPNAME`d and `EQUIPSET`-equipped into its real PCGen slot — `Belt` or `Headband`,
   confirmed against `code/testsuite/PCGfiles/pf_Rogue.pcg`'s real `EQUIPSET` usage before
   authoring), run through `./gradlew run` with a small new template
   (`fixtures/e5-equip-stats.txt.ftl`) that emits all six ability scores generically
   (`STAT.0..5.NAME`/`.SCORE`/`.MOD`, modeled on `AT-33-E2-002`'s `<#list pc.checks as check>`
   generic-loop pattern so no per-item token authoring was needed). Raw per-item output:
   `fixtures/equipment-oracle-txt/<slug>.txt`. All 11 gradle runs exited 0
   (`build-transcript-equipment-ALL-11-tails.txt`; one full transcript committed as a sample,
   `build-transcript-equipment-headband_cha_2-SUCCESS.log`).
3. **Comparison** — `equipment.ours.json` maps each real inventory `unit_id` to
   `(oracle_key, ours_expected_score)` where `ours_expected_score` is the item's declared base
   ability score (16/14/14/10/10/8 for STR/DEX/CON/INT/WIS/CHA on this `.pcg`) plus the real
   engine's resolved bonus — i.e. this checks the *applied* total, not merely that the raw bonus
   number was parsed correctly (which `literal-verified` already establishes for a different
   population). `equipment.oracle-export.txt` merges all 11 raw exports (slug-prefixed keys, so
   one file holds all 11 without key collisions — `scripts/oracle_harness/oracle_export.py`'s
   parser is a flat `KEY=VALUE` reader with no other coupling to this run's shape).
   `scripts/oracle_harness/run.py` (`AT-33-E2-003`'s CLI, unmodified) produced
   `equipment.oracle-results.json`; `scripts/box_ledger.py --check --oracle-results
   equipment.oracle-results.json` (`AT-33-E1-002`'s condition-3 gate) read it and exited 0.

**Result: 11 of 11 agree, 0 disagree, 0 unverifiable.**

## The remaining 1,730 — not folded into a false 100%, split by real structural reason

| Sub-population | Count | Probe status (`AT-33-E1-003`, re-cited not re-derived) | This cycle's disposition |
|---|---:|---|---|
| `equipment` | 11 | magnitude probe exists (`probe_equipment_effect_wiring`) | **examined this cycle — see above** |
| `spell` | 1,288 | magnitude probe exists (`probe_spell_effect_wiring`) | not yet examined — real PCGen re-verification needs a spellcasting character per class/level plus `SPELLMEM.*` template tokens (`base.xml.ftl`'s `<@loop ... COUNT[SPELLSINBOOK...]>` pattern); queued, next-cycle plan below |
| `class_feature` | 15 | magnitude probe exists (`probe_class_feature_effect_wiring`) | not yet examined — needs one high-level character per source class (Rogue/Paladin/Ranger/Barbarian/Bloodrager/Slayer/Ninja/Samurai) plus feature-specific export tokens (DR/sneak-attack-dice/channel-energy tokens, not plain `STAT.*`); queued, next-cycle plan below |
| `companion` | 187 | **no probe** — `probe_exists: false`, `presence_only` | structurally not comparable to a PCGen numeric export this cycle's harness shape checks; `holds_key()` presence lookup, not a magnitude |
| `monster` | 140 | **no probe** — `probe_exists: false`, `presence_only` | same structural reason |
| `monster_ability` | 100 | **no probe** — `probe_exists: false`, `presence_only` | same structural reason |
| **Total** | **1,741** | | 11 examined, 1,730 not yet examined |

**427 of 1,741 (`companion`+`monster`+`monster_ability`) carry a probe-surface gap that
predates this cycle** (`AT-33-E1-003`, executed 2026-08-24/25) — our own engine holds no
magnitude-producing computation for these kinds to compare against an oracle value at all
(`monster_resolve()`/`holds_key()` are table lookups, not formula evaluation). This is not
this cycle's effort gap; it is named here because `AT-33-E5-001`'s population statement
("the 1,741") would otherwise silently imply all 1,741 are equally re-verifiable, which
`AT-33-E1-003`'s own finding already contradicts. Re-verifying these 427 against PCGen would
first require widening the engine's own probe surface — out of this criterion's scope, and
not silently absorbed into "examined" or "unverifiable" here.

**1,303 of 1,741 (`spell`+`class_feature`) are genuinely re-verifiable and not yet attempted**
this cycle — real per-unit (or per-batch) `.pcg`/template authoring cost, exactly the sizing
question `AT-33-E2-004` named. This is a `decisions.md §5`-shaped authoring-cost issue, not an
operator ruling — the next cycle's plan is concrete and mechanical (see below), so this is
**not** filed as an `## Open blockers` entry (`workflow-instruction.md` §8): decomposing and
running the next cycles, not escalating.

## Next-cycle plan (concrete, not a restated goal)

1. **`class_feature` (15 of 1,741):** one L20 `.pcg` per source class already present in the
   pinned oracle's `data/pathfinder` (Rogue/Paladin/Ranger/Bloodrager/Slayer/Ninja/Samurai cover
   14 of 15 features for free via class progression — no per-feature authoring, matching this
   cycle's discovery that PCGen grants inherent class features automatically); Barbarian +
   `rage_power_superstition` needs one explicit rage-power selection. Export tokens differ per
   feature (DR, sneak-attack dice, channel-energy dice/uses, trap-sense bonus) — reading each
   token name out of `outputsheets/base.xml.ftl` first, the same discipline `AT-33-E2-002` used,
   is the bulk of the remaining work, not character authoring.
2. **`spell` (1,288 of 1,741):** batch via `SPELLMEM.${class}.${spellbook}.${level}.${spell}.*`
   generic iteration (confirmed present in `base.xml.ftl`, not requiring one template line per
   spell) against one or a few high-level prepared-caster `.pcg`s with spells added to their
   spellbook — sizing the number of characters needed by class-spell-list coverage is this
   sub-population's own first step.
3. **`companion`/`monster`/`monster_ability` (427 of 1,741):** not an Epic 5 authoring task —
   requires Epic 1's probe surface to widen first (a `holds_key()` presence check has no
   magnitude to compare). Named here so a future cycle does not silently fold these into
   "examined" or re-litigate whether they are in scope.

---

# AT-33-E5-002 — the 6,589 `literal-verified` units

`AT-33-E5-002`'s population: the **6,589** `literal-verified` units in `docs/work-inventory.json`
(`THE-BOX.md`'s `literal-verified` group — a corpus record carries a real magnitude token, but no
verified engine consumer was observed reading it, per `AT-33-E1-003`'s `evidence` field on these
records, e.g. `"race_trait_applied_by_the_race_corpus_but_no_verified_consumer"` /
`"equipment_table_entry_with_corpus_magnitude"`). This is a **different** population from
`AT-33-E5-001`'s 1,741 `fixture-verified` units — no unit appears in both groups
(`THE-BOX.md`'s partition, `uncovered=0 overlap=0`).

By kind (`jq -r '[.units[]|select(.status=="literal-verified")]|group_by(.kind)|map({kind:.[0].kind,count:length})' docs/work-inventory.json`):

| kind | count | probe (`AT-33-E1-003`) |
|---|---:|---|
| `equipment` | 5,170 | magnitude_probe |
| `monster` | 843 | presence_only (no probe) |
| `monster_ability` | 148 | presence_only (no probe) |
| `spell` | 217 | magnitude_probe |
| `companion` | 99 | presence_only (no probe) |
| `equipment_modifier` | 46 | magnitude_probe |
| `race` | 36 | magnitude_probe |
| `class_feature` | 17 | magnitude_probe |
| `race_trait` | 13 | magnitude_probe |
| **Total** | **6,589** | 5,499 probe-bearing / 1,090 presence-only |

## What this cycle actually re-verified against the live oracle

**21 of 6,589** — a real slice of the `equipment` kind: every `literal-verified` `equipment`
record under `ultimate_equipment` whose `raw_bonus_chains` carries a single-ability
`STAT|<ability>|<n>|TYPE=Enhancement` qualifier and a `Belt` or `Headband` `TYPE` (the exact
BONUS shape `AT-33-E5-001` already proved end-to-end for the `fixture-verified` population, so
this cycle reuses the same mechanism — template, `.pcg` slot convention, comparison harness,
`ours`-derivation crate — rather than re-inventing it). 8 further candidates exist in the
`literal-verified` `equipment` population carrying the same `STAT|...|Enhancement` shape but were
excluded from this slice for tractability, named honestly rather than silently dropped: 5 are
multi-ability items (`spindle_of_perfect_knowledge` `STAT|INT,WIS,CHA|...`,
`headband_of_mental_resilience` `STAT|INT,WIS,CHA|...`, `monkey_belt_greater` `STAT|STR,DEX|...`,
`plague_rat_belt_greater` `STAT|DEX,CON|...`, `serpent_belt_greater` `STAT|STR,DEX|...` — the
existing template only resolves a single ability index per item cleanly), 1
(`staff_of_mithral_might`) is a `Staff`/weapon slot rather than `Belt`/`Headband`, 1
(`snakeskin_tunic`) is a `Shirt` slot, and 1 (`gutbite_belt`) is from `inner_sea_gods`, a
different book than the pinned oracle's already-proven `CAMPAIGN:Ultimate Equipment` — none of
these shape differences was attempted this cycle.

For each of the 21 items — same three-step mechanism as `AT-33-E5-001`:

1. **`ours`** — real, live output of
   `codex::rules_core::equipment_effects::compute_equipment_effects`, run against the real
   `data/corpus/ultimate_equipment/equipment/` records via a scratch Rust program
   (`ours-derivation/equipment-literal-ours-probe.rs`, outside the codex repo tree — this
   criterion's granted write scope is `artifacts/epic-5-reverification/` plus Epic 2's harness
   plus an append-only `THE-BOX.md`, not `src/`). Output:
   `ours-derivation/equipment-literal-ours-probe.output.json`. All 21 items resolved a real
   `ability_bonus` (no `None`/unresolved cases).
2. **`oracle`** — a real, live PCGen export. One `.pcg` per item
   (`fixtures/equipment-literal-pcg/`), reusing `AT-33-E5-001`'s exact template
   (`fixtures/e5-equip-stats.txt.ftl`, unmodified) and slot convention (`Belt` or `Headband`
   `EQUIPSET`, matching each item's real `TYPE` token), run through `./gradlew run` against the
   already-built pinned checkout (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
   same jar `AT-33-E5-001` built and this cycle re-used unmodified). Raw per-item output:
   `fixtures/equipment-literal-oracle-txt/<slug>.txt`. **All 21 gradle invocations exited 0**
   (`build-transcript-equipment-literal-ALL-21-tails.txt`; one full sample,
   `build-transcript-equipment-literal-gorgon_belt-SUCCESS.log`).
3. **Comparison** — `equipment-literal.ours.json` maps each real inventory `unit_id` to
   `(oracle_key, ours_expected_score)`; `equipment-literal.oracle-export.txt` merges all 21 raw
   exports (slug-prefixed keys). `scripts/oracle_harness/run.py` (`AT-33-E2-003`'s CLI,
   unmodified) produced `equipment-literal.oracle-results.json`;
   `scripts/box_ledger.py --check --oracle-results equipment-literal.oracle-results.json`
   (`AT-33-E1-002`'s condition-3 gate) exited 0.

**Result: 21 of 21 agree, 0 disagree, 0 unverifiable.**

## The remaining 6,568 — not folded into a false 100%, split by real structural reason

| Sub-population | Count | Probe status | This cycle's disposition |
|---|---:|---|---|
| `equipment` (this cycle's slice) | 21 | magnitude probe | **examined this cycle — see above** |
| `equipment` (remaining) | 5,149 | magnitude probe | not yet examined — 8 same-shape STAT/Enhancement candidates identified and named above; the bulk (thousands) carry other magnitude shapes (weapon/armor enhancement bonuses, resistance bonuses, charges, DCs) needing their own template tokens, not the `STAT.*.SCORE` token this cycle's template reads |
| `spell` | 217 | magnitude probe | not yet examined — same `SPELLMEM.*` batching approach `AT-33-E5-001`'s next-cycle plan names for its own 1,288 `spell` units |
| `equipment_modifier` | 46 | magnitude probe | not yet examined — same probe function as `equipment` (`probe_equipment_effect_wiring`, one shared match arm per `AT-33-E1-003`), but a different corpus shape (weapon/armor special-ability modifiers, not standalone items) needing its own `.pcg` authoring pattern |
| `race` | 36 | magnitude probe | not yet examined — needs a `.pcg` per race (`RACE:<name>`) plus `STAT.*.SCORE` deltas against a race-less baseline, not item-equip |
| `class_feature` | 17 | magnitude probe | not yet examined — same population/approach `AT-33-E5-001`'s next-cycle plan names for its own 15 `class_feature` units |
| `race_trait` | 13 | magnitude probe | not yet examined — needs a `.pcg` per race+trait combination (`RACESUBTYPE`/alternate racial trait selection), a new authoring pattern |
| `monster` | 843 | **no probe** | structurally not comparable — `holds_key()` presence lookup, not a magnitude (`AT-33-E1-003`) |
| `monster_ability` | 148 | **no probe** | same structural reason |
| `companion` | 99 | **no probe** | same structural reason |
| **Total** | **6,589** | | 21 examined, 6,568 not yet examined |

**1,090 of 6,589 (`monster`+`monster_ability`+`companion`) carry the same pre-existing probe-surface
gap `AT-33-E5-001` already named** — not this cycle's effort gap, not silently folded into
"examined" or "unverifiable."

**5,478 of 6,589 (`equipment` remainder + `spell` + `equipment_modifier` + `race` + `class_feature`
+ `race_trait`) are genuinely re-verifiable and not yet attempted** — real per-shape `.pcg`/template
authoring cost, the same `AT-33-E2-004`-named sizing question `AT-33-E5-001` answered a first slice
of for its own population. Not filed as an `## Open blockers` entry — decomposition and execution
of the next slice, not escalation.

## Next-cycle plan

1. **`equipment` remainder, same-shape candidates first (8 of 5,149):** the 5 multi-ability items
   (extend the `ours`-comparison to sum two-or-three `STAT.*.SCORE` deltas) and the 3
   different-slot/book items (`spindle_of_perfect_knowledge`, `staff_of_mithral_might`,
   `gutbite_belt`) named above — smallest remaining increment before needing a new template.
2. **`equipment` remainder, other magnitude shapes (thousands):** requires reading
   `outputsheets/base.xml.ftl` for the weapon/armor-enhancement, resistance-bonus, and
   charges/DC export tokens — the bulk of the `equipment` kind's real authoring cost.
3. **`spell` (217) / `class_feature` (17):** converge with `AT-33-E5-001`'s own next-cycle plan for
   its overlapping-shape 1,288 `spell` / 15 `class_feature` units — one shared authoring effort
   should cover both populations' units of the same kind, not two independent lanes.
4. **`equipment_modifier` (46) / `race` (36) / `race_trait` (13):** each needs its own `.pcg`
   authoring pattern (named above); not yet started.
5. **`monster`/`monster_ability`/`companion` (1,090):** not an Epic 5 task — needs Epic 1's probe
   surface to widen first, same as `AT-33-E5-001`'s equivalent 427.

## AT-33-E5-003 — disagreement-resolution

**Criterion:** every disagreement produced by `AT-33-E5-001`/`AT-33-E5-002` is a named defect,
fixed or escalated; never closed by adjusting the expectation to match our output. Evidence: one
entry per disagreement in `progress.md`, each resolved to a commit or an operator escalation.

**Independently re-derived combined state, this cycle** (not transcribed from the two prior
receipts' prose — computed directly from the two committed oracle-results files):

```
$ python3 -c "
import json,collections
a = json.load(open('equipment.oracle-results.json'))
b = json.load(open('equipment-literal.oracle-results.json'))
merged = a['results'] + b['results']
print('combined records:', len(merged))
print(collections.Counter(r['verdict'] for r in merged))
"
combined records: 32
Counter({'agree': 32})
```

Merged, committed as `AT-33-E5-003.combined-oracle-results.json`, and re-checked independently
through Epic 1's own fail-closed instrument:

```
$ python3 scripts/box_ledger.py --check --oracle-results \
    docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
```

**Disagreement population as of this cycle: 0 of 32 units examined by `AT-33-E5-001`/`AT-33-E5-002`
to date.** Zero entries are required in `progress.md`'s disagreement ledger because zero
disagreements exist — a legitimate, verified outcome (`workflow-instruction.md §12` row 6:
"measurement waves that bank zero units are legitimate deliverables"), not an unexamined gap. This
is **not** a claim that the full 8,330-unit `fixture-verified`+`literal-verified` population has no
disagreement anywhere — that population is 0.38% examined (32 of 8,330) and `AT-33-E5-001`/
`AT-33-E5-002` are `in-progress`, not `complete`, precisely because most of it is still unexamined.
That is those criteria's own scope, not this one's.

**The reopening condition is mechanical, not a promise to remember it** (`decisions.md §4`): any
future `AT-33-E5-001`/`AT-33-E5-002` cycle that lands an oracle-results file containing a
`"verdict": "disagree"` record will make `scripts/box_ledger.py --check --oracle-results <that
file>` exit non-zero and name the offending `unit_id` (`AT-33-E1-002`'s condition 3, already wired
and already proven able to fire — see the mutation proof below). That failing exit code is what
reopens this criterion's work; nobody has to remember to check.

**RED→GREEN (mutation proof that the reopening mechanism actually fires):**

```
$ python3 -c "
import json
d = json.load(open('AT-33-E5-003.combined-oracle-results.json'))
d['results'][0]['verdict'] = 'disagree'; d['results'][0]['ours'] = 999
json.dump(d, open('/tmp/mutated.json','w'))
"
$ python3 scripts/box_ledger.py --check --oracle-results /tmp/mutated.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: ultimate_equipment:equipment:belt_of_mighty_hurling_greater
$ echo $?
1
```

RED: a single injected `disagree` record is caught by name, exit 1. GREEN: the real, unmutated,
committed `AT-33-E5-003.combined-oracle-results.json` — exit 0. The temp mutated copy was never
committed.
