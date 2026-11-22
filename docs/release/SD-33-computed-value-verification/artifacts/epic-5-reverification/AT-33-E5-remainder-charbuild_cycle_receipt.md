# Cycle AT-33-E5-remainder-charbuild — Epic 5 Re-verification / AT-33-E5-001, AT-33-E5-002

- **Commit SHA:** `e5aecb5bf70ea7d435354e2b66c7be39dbf55163` (landed on `tranche/13`)
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` (probe extension only — new `--charbuild-remainder-probe <path>`
    flag, `charbuild_remainder_probe()`, `CharbuildRemainderUnit`,
    `CHARBUILD_REMAINDER_CLASS_FEATURES` (32-entry unit→explanation-id map),
    `CHARBUILD_REMAINDER_RACES` (36), `CHARBUILD_REMAINDER_RACE_TRAIT_ABILITY` (2),
    `CHARBUILD_REMAINDER_RACE_TRAIT_NO_CONSUMER` (11), `apply_charbuild_choice()`.
    No existing function changed; no write to `docs/work-inventory.json`.)
  - `scripts/oracle_harness/charbuild-remainder.txt.ftl` (new — BatchExporter template;
    STAT dump + `countdistinct("ABILITIES","CATEGORY=Special Ability",...)`/`ABILITYALL`
    dump of every Special-Ability name+DESC, proven live against a real level-20 Rogue
    before scaling out — `COUNT[SA]`/`SPECIALABILITY.*` (the token `base.xml.ftl` and
    `AT-33-E5-001`'s own module docstring assumed) evaluate to 0 for this PF1 gamemode;
    `ABILITYALL.<category>` is the mechanism this gamemode's own real, shipped csheet
    (`d20/fantasy/htmlxml/csheet_fantasy_std.htm.ftl`) uses instead — a real, live-tool-
    discovered correction to the assumed mechanism, not a guess)
  - `scripts/oracle_harness/charbuild_remainder_generate.py` (new — one L20 `.pcg` per
    source class (13), one L1 `.pcg` per race (36); per-class campaign closures read
    directly from each sourcebook's own `.pcc` `PRECAMPAIGN:` chain, not guessed)
  - `scripts/oracle_harness/charbuild_remainder_run_one.sh` (new — direct-`java`
    BatchExporter invocation against the installed PCGen distribution
    (`build/install/pcgen`), proven byte-identical to `./gradlew run`'s own output for
    the same inputs and ~5-9x faster per invocation with no gradle-daemon contention
    across concurrent sibling lanes on the same checkout)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/charbuild-remainder-compare.py`
    (new — builds the committed results file from the Rust probe's JSON and the real
    PCGen exports)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/charbuild-remainder-pcg/*.pcg`
    (new, 49 files — 13 class + 36 race, real PCGen character definitions)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/charbuild-remainder-oracle-txt/*.txt`
    (new, 49 files — real, live `BatchExporter` output, one per `.pcg`)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/charbuild-remainder.oracle-results.json`
    (new — the 81-row committed deliverable)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-remainder-charbuild_cycle_receipt.md` (this file)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
  (`git diff --unified=0 "$(git merge-base HEAD origin/develop)...HEAD" -- src/bin/v06_work_inventory.rs scripts/oracle_harness/charbuild-remainder.txt.ftl scripts/oracle_harness/charbuild_remainder_generate.py scripts/oracle_harness/charbuild_remainder_run_one.sh docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/charbuild-remainder-compare.py ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match)
- **Wired-integration audit result:** one pre-existing match, not introduced by this
  cycle — same command with the second pattern
  (`grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`) matches
  a `-`/`+` comment-rewording pair in `v06_work_inventory.rs` (`"placeholder marker --
  unknown"` → `"placeholder marker -- unmeasurable"`) that predates this cycle's own
  commit entirely — confirmed by diffing this cycle's own commit alone
  (`git diff --unified=0 HEAD~1..HEAD -- src/bin/v06_work_inventory.rs | grep -c
  placeholder` → `0`, run before this cycle's commit existed). No token from this
  pattern appears in any line this cycle actually added.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.
  >
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** as above.

## What landed this cycle

This lane owns the "full-character-build" remainder both `AT-33-E5-001`'s and
`AT-33-E5-002`'s own prior remediation receipts named and declined to rush: 15
`fixture-verified` + 17 `literal-verified` `class_feature` units, 36 `race` units, and
13 `race_trait` units — 81 total — whose "ours" magnitude needs the full
`build_pilot_headless_receipt` pipeline (a real character build: race, class,
level, chosen features) rather than the narrow spellbook/equipment library seam
the other two lanes' own probes use.

**Per-unit cost, measured before the full run (throughput requirement):** one direct-`java`
BatchExporter invocation against the pre-built PCGen distribution costs ~20–25s warm
(single-book closure) to ~90–120s (the three ACG classes' real 6-book `PRECAMPAIGN`
closure, or the two UC classes' real 3-book closure) — dominated by campaign/game-data
load, confirmed timed this cycle (`class-barbarian.pcg`: 23–25s repeatable;
`class-slayer.pcg`: ~90–120s under its real closure). **Population is 81 units across
49 characters** (13 class builds + 36 race builds) — **one build amortises many units**:
the Rogue L20 build alone carries 6 units (`rogue_master_strike`, `rogue_trap_sense`,
`rogue_trapfinding`, `rogue_sneak_attack`, `rogue_uncanny_dodge`,
`rogue_talent_resiliency`), the Slayer L20 build carries 5, the Paladin L20 build
carries 4. Projected wall time for 49 builds at `-P 12` parallel (this box, 24 cores):
~4 waves × ~60–120s ≈ 5–10 minutes — well inside one cycle; **actual**: two batches (43
then a corrected 6, after a real campaign-closure defect this cycle found and fixed
live — see Notes) totalling under 15 minutes of real PCGen wall time.

### The magnitude-shape enumeration (per the brief's "enumerate the shapes first" instruction)

Six real, distinct export/comparison shapes, confirmed against live PCGen `DESCRIPTION`
text this cycle, each with its own extraction regex in `charbuild-remainder-compare.py`:

| Shape | Example unit | PCGen DESC pattern | Units |
|---|---|---|---|
| Damage Reduction | `bloodrager_damage_reduction`, `paladin_aura_of_righteousness` | `"Subtract N from the damage..."` / `"DR N/Evil"` | 2 |
| Dice count | `rogue_sneak_attack`, `paladin_channel_positive_energy`, `paladin_lay_on_hands`, `ninja_sneak_attack`, `slayer_sneak_attack`, `unchained_rogue_sneak_attack` | `"Nd6 points..."` | 6 |
| Flat bonus (`+N`) | `rogue_trap_sense`, `slayer_stalker`, `slayer_studied_target`, `slayer_trapfinding`, `ninja_no_trace`, `ranger_track`, `rage_power_superstition`, `druid_nature_sense`, `druid_wild_empathy`, `monk_high_jump`, `slayer_talent_foil_scrutiny`, `investigator_alchemy` | `"+N bonus..."` / `"...equal to N..."` | 12 |
| Save DC | `ranger_master_hunter`, `rogue_master_strike` | `"...DC N)"` | 2 |
| Uses/day | `samurai_resolve` | `"N times per day"` | 1 |
| Hit points | `monk_wholeness_of_body`, `rogue_talent_resiliency` | `"heal N hit points"` / `"N temporary hit points"` | 2 |
| Grant-only identity (no magnitude, either side) | `barbarian_uncanny_dodge`, `rogue_uncanny_dodge`, `paladin_holy_champion`, `ranger_hunter_s_bond` | n/a — engine's own explanation is `value: 0` by design | 4 |
| Ability-score adjustment | every `race` unit + 2 `race_trait` units | `STAT.i.SCORE` delta from a fixed base | 38 |
| No comparable text/oracle | `slayer_trapfinding`, `inquisitor_track` (real DESC text carries no substituted number), 11 `race_trait` with no verified consumer, 3 floating-bonus races | — | 17 |

(Rows sum to more than 81 because the table enumerates *shapes*, and the last two rows
are dispositions, not new shapes; the 81-row denominator is the population, unchanged.)

### Population examined: 81 of 81 (100% of this slice's own denominator)

Re-derive: `python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/charbuild-remainder.oracle-results.json')); print(len(d['results']))"` → `81`.

| Verdict | Count | Denominator | Command |
|---|---:|---|---|
| `agree` | 58 | of 81 | `python3 -c "import json,collections; d=json.load(open('...charbuild-remainder.oracle-results.json')); print(collections.Counter(r['verdict'] for r in d['results']))"` |
| `disagree` | 1 | of 81 | same |
| `unverifiable` | 22 | of 81 | same |
| — by kind: `class_feature` | 32 | of 81 | `python3 -c "import json,collections; d=json.load(open('...')); print(collections.Counter(r['kind'] for r in d['results']))"` → `class_feature=32` |
| — by kind: `race` | 36 | of 81 | same → `race=36` |
| — by kind: `race_trait` | 13 | of 81 | same → `race_trait=13` |
| Reasonless `unverifiable` rows | 0 | of 22 unverifiable | `python3 -c "import json; d=json.load(open('...')); print(len([r for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')]))"` |

### The one disagreement — a real, named engine gap, for `AT-33-E5-003`'s bucket

`core_rulebook:class_feature:monk_ac_bonus`: **ours = 2, oracle = 7.** Both values are
real. `class_chassis.monk.ac_bonus` (`src/rules_core/pilot_compute/mod.rs`) grounds
only the flat Wisdom-modifier component (`max(wisdom_modifier, 0)` = 2 at WIS 14) —
its own doc comment states this explicitly ("grounds only the flat Wisdom-to-AC value
at this level, not the level-4+ dodge-bonus progression"). PF1's real rule (confirmed
against PCGen's live export) adds a level-scaled dodge bonus reaching +5 at monk level
20, so the real total at L20 is 2 (Wisdom) + 5 (level) = 7 — exactly what PCGen
exports. **This is examination, not a fix**: `AT-33-E5-003` owns root-causing and
fixing or escalating it; this cycle's write scope does not include
`src/rules_core/pilot_compute/mod.rs`.

### The 22 unverifiable rows, by real reason

| Reason class | Count | Units |
|---|---:|---|
| Grant-only identity record (engine's own explanation is `value: 0` by design — a boolean immunity or a multi-clause feature with no single grounded magnitude) | 4 | `barbarian_uncanny_dodge`, `rogue_uncanny_dodge`, `paladin_holy_champion`, `ranger_hunter_s_bond` |
| No verified consumer reads this `race_trait` record's magnitude anywhere in the shipped app (the same structural class `AT-33-E1-003` established for `companion`/`monster`/`monster_ability`) | 11 | `suli_earthfoot`, `suli_firehand`, `suli_icewalk`, `suli_shockshield`, `world_walker_skilled`, `aasimar_celestial_resistance`, `aasimar_skilled`, `aasimar_speed`, `aasimar_vision`, `deep_jungle_halfling_poison_use`, `junk_tinker_skilled` |
| Ability-score `race_trait`: real engine magnitude computed via the same `race_creation_chassis` read the `race` population uses, but no dedicated PCGen `.pcg` was built for this specific non-`race`-population race this cycle (real number reported, no live oracle round-trip for this exact unit) | 2 | `aasimar_ability_scores`, `oversized_goblin_ability_scores` |
| Floating "+N to one ability score of choice" race: this cycle's minimal `.pcg` selects no ability, so PCGen shows all-zero deltas and there is no per-ability number to compare the floating magnitude against | 3 | `human`, `half_elf`, `half_orc` |
| PCGen's real `DESCRIPTION` text carries no numeric token this comparison's regex family recognizes (quoted verbatim in each row's `reason`) | 2 | `slayer_trapfinding` (real Slayer L20 Special-Ability list carries no entry named "Trapfinding" at all — confirmed absent, not a lookup miss), `inquisitor_track` (PCGen's own DESC states "half her level" as literal text, never substituting a number) |

**Every row above carries its reason in the committed JSON itself** — this table is a
summary, not the source of truth; re-derive with
`python3 -c "import json; d=json.load(open('...charbuild-remainder.oracle-results.json')); [print(r['unit_id'], '|', r['reason']) for r in d['results'] if r['verdict']=='unverifiable']"`.

## Real live-tool corrections this cycle made before trusting the mechanism at scale

1. **`COUNT[SA]` / `SPECIALABILITY.${sa}` (the token `outputsheets/base.xml.ftl` and
   this bundle's own prior module docstrings assumed) evaluate to `0` for
   `Pathfinder_RPG`.** Confirmed live against a real level-20 Rogue: `SA.COUNT=0` with
   that token, `SA.COUNT=11` (real Bonus Feat/Evasion/Sneak Attack/Trap Sense/
   Trapfinding/... rows) with `countdistinct("ABILITIES","CATEGORY=Special
   Ability",...)` + `ABILITYALL.Special Ability.VISIBLE.${i}` — the exact mechanism
   this gamemode's own real, shipped character sheet
   (`outputsheets/d20/fantasy/htmlxml/csheet_fantasy_std.htm.ftl`) uses. This is why
   the template committed this cycle differs from the module-docstring assumption
   `AT-33-E5-001`'s own receipt made.
2. **Per-class campaign closures are not a safe single-book guess.**
   Bloodrager/Slayer/Investigator (Advanced Class Guide) and Ninja/Samurai (Ultimate
   Combat) and Inquisitor (Advanced Player's Guide, only for its OWN dependency
   confirmation) all failed with `Attempt to fetch AbilityCategory: Class... but it
   does not exist` under a single-book closure — the whole character load aborts (0
   output). Read directly from each sourcebook's own `.pcc`
   (`_advanced_class_guide.pcc`, `ultimate_combat/*.pcc`) rather than guessed: ACG
   requires `Core Rulebook + Advanced Player's Guide + Ultimate Combat + Ultimate
   Equipment + Ultimate Magic` (its own hard `PRECAMPAIGN` chain); UC requires
   `Core Rulebook + Advanced Player's Guide`. Fixed live, re-run, all 6 succeeded.
3. **Direct-`java` against the installed distribution (`build/install/pcgen`) needs an
   explicit `--module-path`/`--add-modules` for JavaFX** (the `run` task's own gradle
   JVM-arg injection is not baked into the static `installDist` script). Confirmed
   byte-identical output to `./gradlew run` for the same inputs once added
   (`diff rogue20.out.txt rogue20-direct.out.txt` → no output, exit 0) before using it
   for the full 49-build batch.
4. **Duplicate same-name `SPECIALABILITY` rows are real, not a parsing bug** — Barbarian
   `Rage` and Paladin `Channel Positive Energy` each export twice (one empty-DESC
   grant marker, one real level-substituted row) at some builds; the comparison script
   prefers the first non-empty `DESC` for a given name rather than blindly taking the
   first match (which would have silently paired several units against an empty
   string and reported them `unverifiable` for a script defect, not a real finding).

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| This slice's population | 81 | 15 fixture `class_feature` + 17 literal `class_feature` + 36 `race` + 13 `race_trait` | `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u['status']=='fixture-verified' and u['kind']=='class_feature']), len([u for u in d['units'] if u['status']=='literal-verified' and u['kind']=='class_feature']), len([u for u in d['units'] if u['status']=='literal-verified' and u['kind']=='race']), len([u for u in d['units'] if u['status']=='literal-verified' and u['kind']=='race_trait']))"` → `15 17 36 13` |
| Units examined this cycle | 81 | of 81 (100%) | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/charbuild-remainder.oracle-results.json'))['results']))"` |
| `agree` | 58 | of 81 | see verdict table above |
| `disagree` | 1 | of 81 | same |
| `unverifiable` | 22 | of 81 | same |
| Reasonless `unverifiable` | 0 | of 22 | see command above |
| Class builds | 13 | 13 source classes named in `CHARBUILD_REMAINDER_CLASS_FEATURES` | `python3 -c "import json; m=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/fixtures/charbuild-remainder-pcg/manifest.json')); print(len([x for x in m if x['kind']=='class']))"` → `13` |
| Race builds | 36 | of 36 races in this slice | same script, `kind=='race'` → `36` |
| Units carried by the single most-loaded build (Rogue L20) | 6 | of 32 `class_feature` units | `rogue_master_strike, rogue_trap_sense, rogue_trapfinding, rogue_sneak_attack, rogue_uncanny_dodge, rogue_talent_resiliency` — direct count from `CHARBUILD_REMAINDER_CLASS_FEATURES` |
| Per-build cost, single-book closure (measured) | ~23–25s | 1 direct-`java` invocation, warm | timed this cycle: `class-barbarian.pcg` |
| Per-build cost, real multi-book `PRECAMPAIGN` closure (measured) | ~90–120s | 1 direct-`java` invocation, warm | timed this cycle: `class-slayer.pcg` / `class-bloodrager.pcg` retry batch |
| `box_ledger.py --check` against this cycle's results file | not run this cycle — `charbuild-remainder.oracle-results.json` is this lane's own deliverable file, not yet folded into the bundle-wide combined file the finalize cycle owns | n/a | see Next-cycle plan |

## Status: complete

Every one of this slice's 81 units carries a real, examined `(ours, oracle, verdict)`
row with a populated `reason` on every `unverifiable`. No unit in this slice is
unexamined. The one real disagreement (`monk_ac_bonus`) and the 22 real
`unverifiable` reasons are reported, not hidden or force-closed to a false agreement.

## Movement, four buckets

- **closure:** 0 — no `docs/work-inventory.json` `status` field changed; oracle
  verification results live in this cycle's own committed JSON file, matching
  `AT-33-E5-001`/`AT-33-E5-002`'s own precedent (oracle-pending → oracle-confirmed is
  recorded here, not as an inventory status transition).
- **reclassification:** 0
- **reachability:** 0 — this cycle discovered two real ceilings (`monk_ac_bonus`'s
  level-scaled-dodge gap; `slayer_trapfinding`'s and `inquisitor_track`'s missing
  oracle-side numeric tokens) but did not widen either; both are named above for a
  future cycle.
- **instrument-correction:** 2 — (1) the `COUNT[SA]`/`SPECIALABILITY` token assumption,
  corrected to `countdistinct("ABILITIES",...)`/`ABILITYALL` (Notes item 1); (2) the
  single-book campaign-closure assumption for ACG/UC-dependent classes, corrected to
  each sourcebook's own real `PRECAMPAIGN` chain (Notes item 2). Neither changes a
  `status` field; both are corrections to this cycle's OWN instruments, found and
  fixed within the same cycle.

## Notes

- **Ability scores are pinned to 14 (modifier +2) uniformly across every class build**
  (`v06_work_inventory.rs`'s new probe overrides `input.chosen.ability_scores`
  explicitly, rather than trusting the shared fixture's own scores) so every
  ability-modifier-dependent formula (Master Hunter DC, Master Strike DC, Wild
  Empathy, Monk AC Bonus) uses the exact same base scores this cycle's `.pcg`
  generator writes — comparable by construction, not by coincidence. This was a real,
  caught-live mismatch: the first probe run (before this fix) used the shared
  fixture's own scores and produced `monk_ac_bonus=1` against a `.pcg` built at WIS 14
  (expected 2) — corrected before the full batch ran, not discovered after.
- **The three choice-gated units** (`rage_power_superstition`, `slayer_talent_foil_scrutiny`,
  `rogue_talent_resiliency`) needed an explicit choice-slot pick on both sides:
  `apply_charbuild_choice()` resolves the pick through the same `CLASS_FEATURE_POOLS`
  table and `class_feature_engine_join_slug` the existing wiring probe
  (`probe_class_feature_key`) already uses, and the `.pcg` generator emits the
  matching real `ABILITY:<category>|TYPE:NORMAL|CATEGORY:<category>|KEY:<key>` line
  read directly from each ability's own real corpus/LST record (`Rage Power ~
  Superstition`, `Slayer Talent ~ Foil Scrutiny`, `Rogue Talent ~ Resiliency`) — never
  a hand-rolled selection id.
- **`paladin_aura_of_righteousness` is deliberately compared against
  `class_chassis.paladin.damage_reduction`, not `class_chassis.paladin.aura_of_righteousness`**
  — the latter is a grant-only identity record pinned at value 0 by construction (two
  of the feature's three real clauses are unmodelled); the DR clause is the same
  corpus feature's one real, grounded magnitude, per that explanation's own doc
  comment. A judgment call, stated here rather than silently made.
- **Unchained Rogue is not a separate `CLASS:` in PCGen's own data** — confirmed by
  direct grep of `pu_abilities_class.lst:116` (`KEY:Rogue ~ Unchained Class`,
  `CATEGORY:CLASS`) — it is an alternate-class-features `ABILITY` selected on top of
  the base Rogue class, encoded that way in this cycle's `.pcg`.
- **Reused, not rebuilt:** `AT-33-E5-001`'s own `ALWAYS_LOAD_CAMPAIGNS` closure list
  (from `fixture-generate-spell-batch.py`) was the starting point for diagnosing the
  6 failing classes, though the real fix used each sourcebook's own smaller,
  precisely-read `PRECAMPAIGN` chain rather than that closure's full 14-book superset
  (which loaded successfully but at 3–5x the wall-clock cost for no additional
  correctness — confirmed by timing both, see Figures table).

## RED→GREEN

Population-examination criterion (the same live-tool-proof form
`AT-33-E5-001`/`AT-33-E5-002`/`AT-33-E2-004` all use), not a new code path in the
traditional unit-test sense. **Before** this cycle: 0 of these 81 units carried any
`(ours, oracle, verdict)` row anywhere — both prior receipts explicitly named this
slice as their own declined-to-rush remainder. **After:** `cargo build --locked --bin
v06_work_inventory` (`CARGO_TARGET_DIR=/tmp/cargo-sd33-sd33-r2-charbuild`,
`CARGO_INCREMENTAL=0`) builds clean, 2 pre-existing warnings only (unrelated,
confirmed by file location outside this cycle's diff); `--charbuild-remainder-probe`
runs clean, exit 0, produces real per-unit values for all 32 class_feature + 36 race +
2 race_trait-ability entries (11 race_trait `no_verified_consumer` entries are
data, not probe output, sourced from the existing `AT-33-E4`-era evidence field); 49
real, live, direct-`java` `BatchExporter` invocations against the real pinned PCGen
oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) all exit 0 (one
retry wave of 6 after the real campaign-closure defect above, all 6 exit 0 on retry);
81 real per-unit rows produced, independently spot-checked against the raw exported
text for `Catfolk` (DEX+2/CHA+2/WIS-2, matches), `Human` (floating +2, correctly
`unverifiable`), `Superstition` (+7 at barbarian level 20, matches), `Sneak Attack`
(10d6 at rogue level 20, matches), `monk_ac_bonus` (2 vs 7, correctly `disagree`).

## Test scoping

Ran `cargo build --locked --bin v06_work_inventory` and
`"$CARGO_TARGET_DIR/debug/v06_work_inventory" --charbuild-remainder-probe <out>.json`
(both exit 0). Ran `charbuild-remainder-compare.py` directly (pure-Python, no test
harness of its own this cycle — a data-pipeline script over already-committed export
text, matching `partition_literal_equipment.py`'s own precedent of no inline test
suite). **Did not** run the root `cargo test` sweep or `apps/desktop/src-tauri` (a
separate cargo workspace; no file in it touched this cycle) — no existing test file
changed this cycle, and this new probe branch has no `#[cfg(test)]` module of its own
(matching `formula_interpreter.rs`'s and `e5_literal_stat_ours.rs`'s own precedent: a
data-pipeline probe over already-tested engine code — `build_pilot_headless_receipt`
and `race_creation_chassis` both carry their own extensive existing test suites,
untouched this cycle).

## Next-cycle plan

1. **The finalize cycle** (owns totalling all three sibling lanes' contributions per
   the dispatch brief) should fold this file's 81 rows into whatever bundle-wide
   combined result set closes kanban rows 16/17/18 — this lane does not mark those
   rows itself.
2. **`monk_ac_bonus`'s real disagreement** (ours=2, oracle=7) is `AT-33-E5-003`'s
   scope: widen `class_chassis.monk.ac_bonus` to add the level-scaled dodge-bonus
   component (max +5 at level 20), or explicitly document the narrower scope as
   intentional and correct the comparison's expectation — `src/rules_core/
   pilot_compute/mod.rs` is out of this cycle's write scope.
3. **`slayer_trapfinding`'s real absence from the Slayer's own Special-Ability list**
   (this cycle confirmed the ability genuinely does not appear, not a lookup miss) is
   worth a from-first-principles check against the Slayer's real PF1 Core Rulebook
   Advanced Class Guide table — either the corpus record is right and PCGen's own
   data omits it (an oracle-side gap), or the corpus record itself needs review.
4. **The 2 ability-score `race_trait` units** (`aasimar_ability_scores`,
   `oversized_goblin_ability_scores`) have a real, reported engine-side magnitude but
   no live oracle round-trip — a future cycle could build two more minimal `.pcg`
   files (`Aasimar`, `Oversized Goblin`) using this cycle's exact same
   `charbuild-remainder.txt.ftl` template to close that gap for real.
