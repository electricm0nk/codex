# Cycle — SD-34 wave 46 — seven registered prestige classes' magnitude-only remainder, 20 of 20 units closed

- **Commit SHA:** `2296746fb9` (`2296746fb9fec808e193162158f9441036739dde`, feat commit; this
  receipt's own SHA fill-in lands in a second, docs-only commit immediately after, following
  this bundle's own established two-commit pattern)
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (6 new class-id consts
  `ARGENT_DRAMATURGE_CLASS_ID`/`HORIZON_WALKER_CLASS_ID`/`NATURE_WARDEN_CLASS_ID`/
  `RAGE_PROPHET_CLASS_ID`/`HOLY_VINDICATOR_CLASS_ID`/`STALWART_DEFENDER_CLASS_ID`; 6 new pure
  formula functions extending `ground_pathfinder_delver_class_features` (Guardbreaker's own
  record, Master Explorer, Thrilling Escape, Vigilant Combatant, Fortunate Soul, True Seeing);
  5 new `ground_<class>_class_features` dispatch functions (Argent Dramaturge, Horizon Walker,
  Nature Warden, Rage Prophet, Holy Vindicator, Stalwart Defender) each with its own pure formula
  function(s); 6 new call sites inside `compute_pilot_base_chassis`; 1 new test module,
  `wave46_registered_prestige_magnitude_formulas_tests`, 26 tests), `src/bin/v06_work_inventory.rs`
  (7 new `EngineFacts` fields, 1 shared `probe_wave46_single_owner_class_features` helper + 7
  per-class probe functions, 7 new call sites, 7 new `classify()` early-return checks, 2 new test
  modules — `wave46_registered_prestige_probe_reachability_tests` (7 real-pipeline reachability
  tests) and `wave46_registered_prestige_classify_tests` (7 classify()-dispatch proofs + 1 negative
  control) — 15 tests total), `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`
  (F1 census pin 5196 → 5193, dated doc-comment update following the file's own established
  convention), `scripts/completion_atlas.py` (10 bucket citation-pin
  re-derivations — this cycle's own insertions shifted every downstream construction-site line
  number in `v06_work_inventory.rs`, the same pattern every prior wave in this series hit),
  `scripts/shape_engine_boundary.py` / `scripts/missing_engine_tables.py` /
  `scripts/tests/test_shape_engine_boundary.py` (citation-pin re-derivations for the same reason —
  wave 44/45 had already flagged these two scripts as "nobody watches this, not wired into
  `verify.sh`"; fixed the citations this cycle's own edits require, left the pre-existing,
  unrelated population-count drift named, not fixed, same as wave 44/45's own choice),
  `docs/work-inventory.json` (regenerated via the guarded path), `docs/release/SD-34-book-
  completion/artifacts/epic-1-atlas/completion-atlas.json` (a `--check` re-run's own artifact),
  `scripts/verify-baselines.env`, this receipt, `progress.md`, `decisions.md`, `kanban.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/
  pilot_compute/mod.rs src/bin/v06_work_inventory.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`
  hits outside this wave's own house-style `wave46_*` module names, matching the file's own
  existing `wave4[1-5]_*` convention).
- **Wired-integration audit result:** `OK_NO_TOKENS` (0 hits for `placeholder`/`STUB`/`MOCK`/
  `not yet implemented`/`fixme`/`hack` in this cycle's own diff).
- **Acceptance criterion (verbatim from this wave's dispatch brief):** re-derive sub-mechanism-5's
  current population fresh (not trusted from wave 45's own 686/598/88), group the registered-and-
  magnitude-missing units by owning prestige class, pick 2-4 (or more, if genuinely cheap) whose
  remaining units look most precedented, close as many as can be safely verified this cycle with
  real pure-formula + reachability tests, verify with both `cargo test --locked --lib` and the
  full `cargo test --locked --no-fail-fast` integration suite (a second full run if any code
  changes after the first), run the guarded regen, re-derive the F1/shape_ledger pin if the
  change is F1-shaped, and report real before/after bucket deltas plus what remains named for a
  future wave.

## Fresh population re-derivation (not trusted from wave 45's own count)

**Method:** `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(sum(1
for u in d['units'] if 'class_feature_of_unmodelled_corpus_class' in (u.get('evidence') or '')))"`
against this cycle's own pre-edit HEAD (`eae14d4b50`, wave 45's own wave-end-gate commit).

**Result: 654**, exactly matching wave 45's own post-cycle figure (686 − 32 = 654) — unlike every
prior wave's own re-derivation in this section, this one did NOT find a stale count: the registry
fixture and classifier have not changed since wave 45's own close.

## Cross-reference against the 74-entry prestige-class registry fixture

Same method as wave 45: extract each unit's evidence suffix after
`class_feature_of_unmodelled_corpus_class:`, check membership against
`tests/fixtures/rules_core/prestige-class-entry-requirements.json`'s 74 `class_id` entries.

**Result: 566 registered / 88 not registered**, summing exactly to 654 — again exactly matching
wave 45's own post-cycle split. The 88 not-registered units are the SAME population wave 45 named
by slug (`psychic_detective` 18, `animal` 17, `eidolon` 16, `phantom` 9, `plant` 9, `undead` 8,
`dragon` 8, `gifted_blade` 3) — re-confirmed unchanged, not re-attempted this cycle.

## Grouping the 566 registered units by owning prestige class

`python3` group-by over the 566 registered units' evidence slug: **58 distinct prestige
classes**, ranging from 45 units (Inner Sea Magic's Divine Scion) down to 1 (Ultimate Intrigue's
Sentinel). Every class's own corpus record(s) were read directly (never a prior wave's summary)
before any class was picked or skipped.

**Phrenic Slayer's own remaining 11 units are explicitly excluded from consideration** per the
dispatch brief and wave 45's own naming — they key off cross-class prime-stat/parent-class-entry
variables (`AegisCL`, `MndBladeLVL`, `PhrenicSlayerPrimeStat`), a genuinely separate, harder
subsystem question this cycle does not attempt.

**Classes scanned and explicitly skipped as NOT cheap, named so a future wave does not re-scan
them from scratch:**
- Every Ultimate Psionics prestige class whose remaining units carry the `AS`/`MB`/`MBAS`/`Ma`/
  `MaAS`/`MaMB`/`MaMBAS` shape (Sighted Seeker, Thrallherd, Psion Uncarnate, Cerebremancer,
  Metamind, Elocater, Psicrystal Imprinter, Soul Archer, Metaforge, and others) — this is the
  IDENTICAL "Advance Astral Suit/Mind Blade/Manifesting" cross-class parent-entry shape Phrenic
  Slayer's own excluded 11 units already carry, confirmed by directly reading several of these
  records (e.g. `data/corpus/ultimate_psionics/class_feature/cerebremancer/mb.json`,
  `.../maas.json`) — same genuinely-harder bucket, not reattempted per-class.
- Large heterogeneous classes (Divine Scion 45 units, Twilight Talon 17, Golden Legionnaire 16,
  Cyphermage 17, and similarly-sized others) where every remaining unit is a DIFFERENTLY-named
  feature needing its own formula — read but not selected this cycle, since a handful of small,
  fully-self-contained classes (below) offered the same or better cost-per-unit at far lower risk
  of a per-unit misread; named here as a real, larger remaining population, not silently dropped.

## Seven prestige classes closed this cycle (20 units)

Every unit's real corpus JSON record was read directly, then independently cross-checked against
the real, non-ingested PCGen oracle (`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_
game/{adventurers_guide,advanced_players_guide}/`) — the same discipline wave 45 established,
never trusting the ingested corpus JSON alone for a formula that (in three of these seven cases)
is not even fully captured on the class_feature record's own `raw_tokens`.

### 1. Pathfinder Delver — 6 units (extends wave 44's `ground_pathfinder_delver_class_features`)

This class's own remaining sm5 units after wave 44/45's PaDFE/Favored-Enemy closures. All six
formulas are keyed on `PaDLVL = CL` (`ag_classes.lst:279`, the class's own raw level) — no
cross-class dependency, unlike this class's own already-excluded siblings.

- **Guardbreaker** (`ag_abilities_class.lst:382`): `BONUS:VAR|FavoredConstruct,FavoredOoze,
  FavoredUndead|TrapSenseBonus` — the SAME `TrapSenseBonus` magnitude
  `pathfinder_delver_padfe_bonus` (wave 44) already grounds for the three PaDFE sub-records, but
  as this record's OWN unit (a distinct corpus key never previously given its own explanation).
- **Master Explorer** (`ag_abilities_class.lst:379`): `PaDSkillBonus = max(1,CL/2)`, formula
  sourced from the class's own level-1 grant row (`ag_classes.lst:285`), NOT the class_feature
  record's own tokens (which carry only `DEFINE:PaDSkillBonus|0` and the consuming
  `BONUS:SKILL` token) — the same cross-file idiom wave 44's own Guardbreaker citation already
  established for this class.
- **Thrilling Escape** (`ag_abilities_class.lst:381`): `PaDEscapeTimes`, a cumulative `+1` at
  levels 3, 7, and 9 (`ag_classes.lst:287,291,292`) — real running total 1/2/3 uses per day.
- **Vigilant Combatant** (`ag_abilities_class.lst:384`): `PaDInitiative = CL/2`, granted level 4
  (`ag_classes.lst:288`).
- **Fortunate Soul** (`ag_abilities_class.lst:386`): `PaDFortunateTimes`, a cumulative `+1` at
  levels 6 and 10 (`ag_classes.lst:290,293`) — real running total 1/2 uses per day.
- **True Seeing** (`ag_abilities_class.lst:387`): `SPELLS:...TIMES=1|CASTERLEVEL=PaDLvl|True
  Seeing,...`, granted level 9 (`ag_classes.lst:292`) — caster level equal to raw class level,
  literal 1/day.

### 2. Argent Dramaturge — 2 units (new `ground_argent_dramaturge_class_features`)

- **Argent Performance** (`ag_abilities_class.lst:24`): TWO magnitudes on one record —
  `ArgentPerformanceRounds = ArgentDramaturgeLVL*2` (a bardic-performance-rounds-style total) and
  `ArgentPerformanceDC = 10+ArgentDramaturgeLVL+CHA` (the classic "10 + level factor + ability
  modifier" save-DC idiom named in this wave's own brief, matching `warpriest_channel_energy_dc`'s
  precedent exactly).
- **Dramaturgical Flourish** (`ag_abilities_class.lst:25`): `BONUS:ABILITYPOOL|Dramaturgical
  Flourish Choice|ArgentDramaturgeLVL/2` — a pool-SIZE-only grounding, the same shape
  `loremaster_secret_lore_pool_size` (wave 43) already established.

### 3. Horizon Walker — 3 units (new `ground_horizon_walker_class_features`)

Three pool-size-only groundings, all self-contained on `HorizonWalkerLVL`
(`apg_classes.lst:412`): **Favored Terrain** (`(2*(HorizonWalkerFavoredTerrainLVL+1))/3`,
`apg_abilities_class.lst:1295`), **Terrain Mastery** (`HorizonWalkerLVL/2`,
`apg_abilities_class.lst:1313`), **Terrain Dominance** (`HorizonWalkerLVL/3`,
`apg_abilities_class.lst:1337`).

### 4. Nature Warden — 2 units of 3 (new `ground_nature_warden_class_features`)

**Companion Bond** (`CompanionBondLVL = NatureWardenLVL`, formula sourced from the class's own
level-1 grant row, `apg_classes.lst:460`) and **Survivalist** (`SurvivalistLVL =
NatureWardenLVL`, in the record's own tokens, `apg_abilities_class.lst:1429`) — both raw
level-tracking pass-throughs. **Woodforging (this class's third open sm5 unit) is NOT attempted**:
its own corpus record carries no `DEFINE`/`BONUS` token anywhere (`wiring_class: "display"`,
`display:no_magnitude_token`) — a genuinely different, text-only shape this cycle does not
speculate a fix for.

### 5. Rage Prophet — 2 units of 3 (new `ground_rage_prophet_class_features`)

**Rage Prophet Mystery** (`RageProphetMysteryLVL = RageProphetLVL`,
`apg_abilities_class.lst:1440`) and **Ragecaster** (`RagecasterLVL = RageProphetLVL`,
`apg_abilities_class.lst:1443`) — both raw level-tracking pass-throughs, both fully in the
record's own tokens. **Spirit Warrior (this class's third open sm5 unit) is NOT attempted**: same
`wiring_class: "display"`, no-magnitude-token shape as Nature Warden's Woodforging above.

### 6. Holy Vindicator — 1 unit of 2 (new `ground_holy_vindicator_class_features`)

**Stigmata** (`StigmataLVL = floor(HolyVindicatorLVL/2)`, `apg_abilities_class.lst:1278`) — the
same "half level, floored" idiom `duelist_elaborate_defense_dodge_bonus` (wave 43) already
established. **Channel Smite (this class's other open sm5 unit) is NOT attempted**: a bonus-feat
grant (`ABILITY:FEAT|AUTOMATIC|Channel Smite`) with no magnitude token at all — same display shape
as the two units above.

### 7. Stalwart Defender — 4 units of 6 (new `ground_stalwart_defender_class_features`)

- **AC Bonus**: `StalwartDefenderDodgeACBonus = 1+(SDL>=4)+(SDL>=7)+(SDL>=10)`, a step table
  transcribed literally (`apg_abilities_class.lst:1451`).
- **Damage Reduction**: `DamageReductionLVL = (SDL>4)+(SDL>6)+(SDL>6)+(SDL>9)+(SDL>9)`, transcribed
  literally — the `>6`/`>9` terms each appear TWICE in the real corpus token, i.e. DR increases by
  2 (not 1) at levels 7 and 10; verified directly against the real `.lst` line before writing the
  function, not assumed from the DESC's own "DR %1/-" phrasing (`apg_abilities_class.lst:1470`).
- **Defensive Powers**: `DefensivePowerLVL = StalwartDefenderLVL/2`, a pool-SIZE-only grounding
  (`apg_abilities_class.lst:1453`).
- **Defensive Stance**: `DefensiveStanceDuration = 4+CON+(SDL-1)*2`, rounds-per-day
  (`apg_abilities_class.lst:1452`).

**Increased Damage Reduction and Renewed Defense (this class's other two open sm5 units) are NOT
attempted:** Increased Damage Reduction is a `Defensive Stance Power` pool MEMBER
(`BONUS:VAR|DamageReductionLVL|1`, selectable up to twice) whose own magnitude depends on a real
recorded pool selection this engine does not track for this class's own chooser; Renewed Defense
heals `%1d8 + %2` (`CL/2`, `CON`) — dice notation `formula_interpreter.rs` does not parse, the
same "grounds the level-derived factor only, never the die roll" boundary as Assassin's Death
Attack (wave 43) — left unattempted rather than guessed at, following that same precedent.

## Verification (mandatory)

`cargo check --locked --lib -j 6` → exit 0. `cargo check --locked --bin v06_work_inventory` →
exit 0. `cargo test --locked --lib -j 6 wave46_registered_prestige_magnitude_formulas_tests` → 26
passed, 0 failed. `cargo test --locked --bin v06_work_inventory -j 6 wave46_` → 15 passed, 0
failed (one negative-control test was removed after it was found to test an invalid premise — see
"A real, honest correction caught mid-cycle" below).

`cargo test --locked --lib -j 6` (full lib suite) → **3121 passed, 0 failed, 14 ignored** (up from
the standing 3095 baseline by exactly this cycle's 26 new lib tests).

`cargo test --locked --no-fail-fast -j 6` (full workspace), **run 1** → **8545 passed, 0 failed, 67
ignored, across 590 suites** (up from the standing 8504 baseline by exactly +41 = the same 26 new
lib tests, counted again since root-full runs the lib suite too, plus 15 new bin tests). A small,
comment-only fix (correcting a stale line citation in Guardbreaker's own doc comment, plus two
citation-pin re-derivations in `scripts/shape_engine_boundary.py`/`scripts/missing_engine_tables.py`)
was made after this run started, triggering this wave's own "re-run the full suite a SECOND time
if any code change happens after the first run" rule. **Run 2 was started, then deliberately
killed partway through** (rather than let it finish and immediately go stale again) once the
guarded regen's own before/after join (below) surfaced a further, GENUINELY required code edit:
3 of the 20 closed units are F1-shaped (`shape_ledger.py --output`, verified per-id against the
pre-cycle snapshot), so `formula_interpreter_corpus_wide.rs`'s own pinned F1 census test needed
re-deriving (5196 → 5193) before any run could be trusted as final. **Run 3**, against the
fully-settled tree (F1 pin included) → **8545 passed, 0 failed, 67 ignored, across 590 suites** —
byte-identical to run 1's own count, confirming neither the citation fix nor the F1 pin update
introduced any regression.

## A real, honest correction caught mid-cycle

**A test expectation was wrong, caught by the test itself (RED for the right reason).** The first
draft of `pathfinder_delver_ids_reach_the_real_pipeline_at_level_ten` asserted Guardbreaker's
level-10 bonus as `5` (mentally conflating it with Vigilant Combatant's own `CL/2` shape); the
real formula is `TrapSenseBonus = RogueTrapSenseLVL/3`, `RogueTrapSenseLVL = PaDLVL+1`, so the
correct level-10 value is `3`. The test failed on its first run with the real value, the
expectation was corrected (not the formula), and the doc comment cites the real derivation.

**A negative-control test was removed after it was found to test an invalid premise.**
`none_of_the_seven_probes_wire_an_unrelated_class` initially asserted that
`probe_pathfinder_delver_wave46_wiring(&some_other_fixture)` returns empty — but every one of
this wave's seven probes (like `probe_pathfinder_delver_padfe_wiring`/`probe_phrenic_slayer_
favored_enemy_wiring` before them) calls `class_sweep_input(fixture, "<slug>", level)` internally,
which OVERWRITES `class_levels` regardless of what is passed in — so the test's own premise (that
an unrelated fixture would leave the probe's target class un-swept) was false by construction, not
a real negative control. Removed and replaced with a code comment explaining why no such test is
meaningful here, pointing to the two negative controls that DO cover the real concern (the
explanation-id leak test in `mod.rs`, and the classify()-dispatch negative control in
`v06_work_inventory.rs`).

## Guarded regen

First attempt would have refused (the same guard every prior wave hits) had the two prerequisite
reports not been generated first. Generated both fresh against this cycle's own tree (release
profile):

- `cargo run --locked --release --bin corpus_literal_sweep -- --json-out <path>` →
  `48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463 digests
  checked, 0 findings` / `3138 tokens exempted under decisions.md §24 redaction across 1058
  codex_generated_name records` / **CLEAN** — byte-identical to wave 45's own pre-cycle figures (no
  `data/corpus/**` file touched this cycle, confirmed via `git diff --stat -- data/corpus/` empty).
- `cargo run --locked --release --bin derived_evaluator_fixture_check -- --json-out <path>` →
  `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — also byte-identical.

Re-ran `cargo run --locked --bin v06_work_inventory` with both reports set — completed cleanly
(exit 0), no refusal, `docs/work-inventory.json` regenerated (`git status --porcelain` shows `M`,
not stale).

## Before/after bucket movement

`python3 scripts/completion_atlas.py --check` at this cycle's own pre-edit HEAD (`eae14d4b50`):
`population=49438 unclassified=0 overlap=0`, `DONE: 25407`, `D: 2461`, `V: 337` (matching wave 45's
own wave-end-gate figures exactly).

| Unit id | corpus_key | status (pre-cycle) | status (post-regen) |
|---|---|---|---|
| `advanced_players_guide:class_feature:holy_vindicator_stigmata` | Holy Vindicator ~ Stigmata | engine-does-not-hold | grounded |
| `advanced_players_guide:class_feature:horizon_walker_favored_terrain` | Horizon Walker ~ Favored Terrain | engine-does-not-hold | grounded |
| `advanced_players_guide:class_feature:horizon_walker_terrain_dominance` | Horizon Walker ~ Terrain Dominance | engine-does-not-hold | grounded |
| `advanced_players_guide:class_feature:horizon_walker_terrain_mastery` | Horizon Walker ~ Terrain Mastery | engine-does-not-hold | grounded |
| `advanced_players_guide:class_feature:nature_warden_companion_bond` | Nature Warden ~ Companion Bond | engine-does-not-hold | literal-verified |
| `advanced_players_guide:class_feature:nature_warden_survivalist` | Nature Warden ~ Survivalist | engine-does-not-hold | literal-verified |
| `advanced_players_guide:class_feature:rage_prophet_rage_prophet_mystery` | Rage Prophet ~ Rage Prophet Mystery | engine-does-not-hold | literal-verified |
| `advanced_players_guide:class_feature:rage_prophet_ragecaster` | Rage Prophet ~ Ragecaster | engine-does-not-hold | literal-verified |
| `advanced_players_guide:class_feature:stalwart_defender_ac_bonus` | Stalwart Defender ~ AC Bonus | engine-does-not-hold | grounded |
| `advanced_players_guide:class_feature:stalwart_defender_damage_reduction` | Stalwart Defender ~ Damage Reduction | engine-does-not-hold | grounded |
| `advanced_players_guide:class_feature:stalwart_defender_defensive_powers` | Stalwart Defender ~ Defensive Powers | engine-does-not-hold | grounded |
| `advanced_players_guide:class_feature:stalwart_defender_defensive_stance` | Stalwart Defender ~ Defensive Stance | engine-does-not-hold | grounded |
| `adventurers_guide:class_feature:argent_dramaturge_argent_performance` | Argent Dramaturge ~ Argent Performance | engine-does-not-hold | grounded |
| `adventurers_guide:class_feature:argent_dramaturge_dramaturgical_flourish` | Argent Dramaturge ~ Dramaturgical Flourish | engine-does-not-hold | grounded |
| `adventurers_guide:class_feature:pathfinder_delver_fortunate_soul` | Pathfinder Delver ~ Fortunate Soul | engine-does-not-hold | literal-verified |
| `adventurers_guide:class_feature:pathfinder_delver_guardbreaker` | Pathfinder Delver ~ Guardbreaker | engine-does-not-hold | grounded |
| `adventurers_guide:class_feature:pathfinder_delver_master_explorer` | Pathfinder Delver ~ Master Explorer | engine-does-not-hold | literal-verified |
| `adventurers_guide:class_feature:pathfinder_delver_thrilling_escape` | Pathfinder Delver ~ Thrilling Escape | engine-does-not-hold | literal-verified |
| `adventurers_guide:class_feature:pathfinder_delver_true_seeing` | Pathfinder Delver ~ True Seeing | engine-does-not-hold | grounded |
| `adventurers_guide:class_feature:pathfinder_delver_vigilant_combatant` | Pathfinder Delver ~ Vigilant Combatant | engine-does-not-hold | literal-verified |

All 20 pre-cycle: `status="engine-does-not-hold"`,
`evidence="class_feature_of_unmodelled_corpus_class:<class slug>"`.

**Post-regen:** `python3 scripts/completion_atlas.py --check` on the regenerated
`docs/work-inventory.json`: `population=49438 unclassified=0 overlap=0 citation_failures=0`,
`DONE: 25407→25419 (+12)`, `D: 2461→2441 (−20)`, `V: 337→345 (+8)`, every other bucket unchanged.
Independently re-derived via a direct Python `id`→`status` join over both the pre- and post-regen
inventory snapshots (not just `--check`'s own summary): pre/post population both 49438, 0 added,
0 removed, **exactly 20 units changed status, zero collateral movement** — 12 landed `grounded`
(Holy Vindicator Stigmata; all 3 Horizon Walker units; all 4 Stalwart Defender units; both Argent
Dramaturge units; Pathfinder Delver's Guardbreaker and True Seeing), 8 landed `literal-verified`
(both Nature Warden units; both Rage Prophet units; Pathfinder Delver's Fortunate Soul, Master
Explorer, Thrilling Escape, Vigilant Combatant) — the same D→V shape waves 41/43/44 already hit,
a legitimately-resolved bucket rather than a lesser outcome (`decisions.md`'s own standing
framing). All 8 `literal-verified` units carry `wiring_class` `static` (matching `completion_
atlas.py`'s own established "static/derived + literal-verified reaches DONE" idiom, separate from
`pf1e_dashboard_producer.py`'s own stricter, unrelated `wiring_class`-gated doneness table named
below).

## F1/shape_ledger pin

**Moved: 5196 → 5193.** This cycle's 20 closures include exactly 3 F1-shaped units (a bare-literal
magnitude token, no per-level/ability/pool expression), verified per-id via `python3 scripts/
shape_ledger.py --inventory <pre-cycle snapshot> --corpus-root data/corpus --output <path>` against
the PRE-cycle inventory (since a unit that leaves the not-done population no longer appears in a
post-regen scan): **Nature Warden's Companion Bond** (`BONUS:VAR|CompanionBondLVL|
NatureWardenLVL`, a bare single-variable token — the same "F1 classifies the record's OWN token
shape, not its full resolution chain" idiom wave 44's own Pathfinder Delver PaDFE entry already
established) and **Pathfinder Delver's Thrilling Escape / Fortunate Soul** (each record's OWN
`raw_tokens` carry only a bare `DEFINE:<X>|0` with no `BONUS` token at all — the real cumulative
`+1` formula lives entirely on the class's OWN level-table file, `ag_classes.lst`, outside this
record's own tokens, so the per-record token scan reads only the bare default and classifies it
F1). The other 17 closed units, verified per-id via the same `--output` dump, are NOT F1-shaped:
`F0` (no formula token in the record at all — Pathfinder Delver's True Seeing, 1 unit), `F2`
(per-level scaling — Horizon Walker's three units, Nature Warden's Survivalist, Rage Prophet's
two units, Stalwart Defender's four units, Argent Dramaturge's two units, 12 units), `F4` (named-
counter/pool-variable reference — Pathfinder Delver's Guardbreaker, Master Explorer, and Vigilant
Combatant, each consuming a variable set on the class's own `ag_classes.lst` level-table row
rather than a per-level arithmetic expression in the record's own tokens, 3 units), or `F5`
(clamped/capped per-level scaling — Holy Vindicator's Stigmata, 1 unit) — 1+12+3+1 = 17. `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root
data/corpus` re-run against the post-regen inventory to confirm the population-level arithmetic:
`F1 = 5193` exactly (5196 − 3). `formula_interpreter_corpus_wide.rs`'s own pinned census test
updated to match (`f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_
sd32_census`, 5196 → 5193), following its own established dated doc-comment convention.

**A real, honest secondary finding while re-deriving this pin:** the "not-done" population this
script/test actually reads is gated by a DIFFERENT, older, stricter doneness instrument
(`scripts/observer/pf1e_dashboard_producer.py::doneness_verdict`, the same one wave 45 already
named) — it maps `wiring_class` `display`/`static`/`derived` + `grounded` to `HELD` (stays in the
not-done population), while `literal-verified`/`fixture-verified` reach `DONE` there for
`static`/`derived` too, and `wiring_class` `computed` + `grounded` always reaches `DONE`. Applying
this to the 20 closed units: 11 leave that instrument's own not-done population (the 8
`literal-verified` `static` units, plus the 3 `computed`+`grounded` units — Pathfinder Delver's
Guardbreaker, Argent Dramaturge's Argent Performance, Stalwart Defender's Defensive Stance), and 9
stay (the remaining `derived`+`grounded` units — Pathfinder Delver's True Seeing, Argent
Dramaturge's Dramaturgical Flourish, all 3 Horizon Walker units, Holy Vindicator's Stigmata, and
3 of Stalwart Defender's 4 units). That instrument's own total not-done population moved
25751 → 25740 (−11), independently confirmed — matching this cycle's own F1 arithmetic exactly
(3 of the 11 units that left are F1-shaped), not a discrepancy.

## Findings for a future wave

1. **Sub-mechanism 5's remaining population after this cycle: 634** (654 − 20), split **546
   registered** across 55 remaining prestige classes (the highest-value remaining target — every
   AS/MB/Ma-shaped Ultimate Psionics class named above excluded as genuinely harder, same bucket
   as Phrenic Slayer's own remaining 11; the large heterogeneous classes named above are real,
   closable-but-slower work) and **88 not registered** (unchanged, named by slug above).
2. **Phrenic Slayer's own remaining 11 units, Nature Warden's Woodforging, Rage Prophet's Spirit
   Warrior, Holy Vindicator's Channel Smite, and Stalwart Defender's Increased Damage
   Reduction/Renewed Defense** all remain named, not attempted, each for a distinct, real reason
   stated in its own class's section above (cross-class prime-stat dependency, no-magnitude-token
   display shape, pool-selection dependency, or dice notation).
3. **The AS/MB/MBAS/Ma/MaAS/MaMB/MaMBAS Ultimate Psionics cross-class-manifester-level shape** now
   spans at least 9 confirmed prestige classes (Sighted Seeker, Thrallherd, Psion Uncarnate,
   Cerebremancer, Metamind, Elocater, Psicrystal Imprinter, Soul Archer, Metaforge) — a future wave
   scoping real subsystem modeling for Phrenic Slayer's own remaining 11 units should treat this as
   the SAME underlying question across all 9+ classes, not a per-class rediscovery.
4. **`scripts/shape_engine_boundary.py`'s own population-count pin remains stale** (named by waves
   44/45, not wired into `verify.sh`) — this cycle's own insertions DID shift its citation LINE
   number (fixed, along with `scripts/missing_engine_tables.py`'s two citations, both now
   `citation_ok`/`citation_failures=0`), but the deeper, unrelated `not_held_by_engine` population
   drift (pinned `9475`, live `8995`) is left named, not fixed, same as wave 44/45's own choice
   (no established multi-wave update convention for this pin, and the test carrying it is not
   wired into `verify.sh`).

## Build scope verified

- `cargo check --locked --lib -j 6` → exit 0.
- `cargo check --locked --bin v06_work_inventory` → exit 0.
- `cargo test --locked --lib -j 6 wave46_registered_prestige_magnitude_formulas_tests` → 26
  passed, 0 failed.
- `cargo test --locked --bin v06_work_inventory -j 6 wave46_` → 15 passed, 0 failed.
- `cargo test --locked --lib -j 6` (full lib suite) → 3121 passed, 0 failed, 14 ignored.
- `cargo test --locked --no-fail-fast -j 6` (full workspace), run to completion twice against the
  fully-settled tree → **8545 passed, 0 failed, 67 ignored, 590 suites, exit 0, identically both
  times**.
- `scripts/verify-baselines.env` updated: `BASELINE_ROOT_LIB_TESTS` 3095→3121,
  `BASELINE_ROOT_FULL_TESTS` 8504→8545 (dated entry appended following the file's own
  convention).
- `python3 scripts/completion_atlas.py --check` (post-regen, re-run independently after the full
  suite): `population=49438 unclassified=0 overlap=0 citation_failures=0`, bucket counts match
  this receipt's own before/after table exactly.
- `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`
  (re-run independently after the full suite): `F1 = 5193`, matching this receipt's own F1/pin
  section exactly.
- `python3 scripts/denominator_gate.py --check`: `files_checked=181 violations=0`.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty, no
  file under `apps/desktop/` touched this cycle.
