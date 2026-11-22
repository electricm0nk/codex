# Wave 51 cycle receipt — Core Rulebook + Ultimate Campaign, buckets B/C/D/M

**Date:** 2026-09-07
**Branch:** `tranche/14`
**Commit:** `244a4dce65`
**Scope granted:** Core Rulebook + Ultimate Campaign only, buckets B + C + D + M.

---

## 1. Fresh population, re-derived (not taken on trust from the brief)

Re-classified every Core Rulebook and Ultimate Campaign unit with
`completion_atlas.py`'s own `_bucket_of()` function directly, against
`docs/work-inventory.json` at `generated_at: 2026-09-07T08:00:34Z`.

| book | bucket | units | denominator |
|---|---|---:|---|
| core_rulebook | M | 778 | of 6,701 core_rulebook units |
| core_rulebook | B | 392 | of 6,701 core_rulebook units |
| core_rulebook | C | 191 | of 6,701 core_rulebook units |
| core_rulebook | D | 138 | of 6,701 core_rulebook units |
| ultimate_campaign | M | 36 | of 265 ultimate_campaign units |
| ultimate_campaign | D | 2 | of 265 ultimate_campaign units |

Actionable scope this wave: **1,537 units, out of the 6,966 total Core Rulebook + Ultimate Campaign units**.
Every figure matches the dispatch brief's own exactly — the brief's numbers were re-derived, not assumed.

---

## 2. Units closed, by bucket and kind

| bucket | kind | evidence closed | units closed | of that bucket's own CR/UC population |
|---|---|---|---:|---|
| M | race_trait | `race_trait_generic_table_holds_record_magnitude_not_yet_computed` | 115 | 115 closed out of 778 core_rulebook bucket-M units |
| D | ability | `ability_content_table_holds_zero_magnitude_record_pending_wiring_class_review` | 102 | 102 closed out of 138 core_rulebook bucket-D units |
| B | class_feature | — | 0 | 0 closed out of 392 core_rulebook bucket-B units |
| C | class_feature | — | 0 | 0 closed out of 191 core_rulebook bucket-C units |
| M+D | (ultimate_campaign) | — | 0 | 0 closed out of 38 ultimate_campaign actionable units |

**Total closed this wave: 217 units, out of the 1,537 actionable Core Rulebook + Ultimate Campaign units in scope.**

Post-wave Core Rulebook buckets: **M 663 / B 392 / C 191 / D 36, out of 6,701 core_rulebook units**.
Corpus-wide `DONE` moved **25,906 -> 26,123, out of a population of 49,438 units** (unchanged both snapshots).

---

## 3. Mechanism 1 — `racial_sla`, a genuinely new engine module (115 bucket-M units)

**The shape.** `cr_abilities_race.lst` declares a `Racial SLA ~ <Spell>` record for every spell any
race in the whole PCGen library grants as a racial spell-like ability. Core Rulebook has 118 of them
ingested under `data/corpus/core_rulebook/race_trait_generic/racial_sla_*.json`, and every one sat
at `ingested-magnitude` — held by the engine's `race_trait_generic` table, carrying a real
magnitude, never computed.

**Read all 118 directly, classified by `raw_tokens` shape: this is ONE mechanism, not 118 pieces of
content.** Every record carries the identical seven-`DEFINE` block, and **115 records, out of the
118 ingested, carry the identical five-token `BONUS:VAR` chain**:

| token | verbatim (with the record's own `<S>` slug) | what it states |
|---|---|---|
| `BONUS:VAR` | `RacialSLA_<S>_LVL\|TL\|TYPE=Base` | caster level IS the character's total level |
| `BONUS:VAR` | `RacialSLA_<S>_SpellLVL\|<n>` | the spell's own level — the ONLY per-record datum |
| `BONUS:VAR` | `RacialSLA_<S>_Times\|1\|TYPE=Base` | one use per day, base |
| `BONUS:VAR` | `RacialSLA_<S>_DCMod\|CHA\|TYPE=Base` | the DC's ability term is Charisma |
| `BONUS:VAR` | `RacialSLA_<S>_DC\|10+RacialSLA_<S>_SpellLVL+RacialSLA_<S>_DCMod` | the save DC |

So the corpus itself states PF1's spell-like-ability save DC rule: `10 + spell level + Charisma
modifier`. `RACIAL_SLA_SAVE_DC_FORMULA` is one shared string; the catalog carries one `spell_level`
per record and no per-record formula at all.

**How each record is grounded, never asserted.** `racial_sla_save_dc_is_grounded_for_corpus_key`
builds a real `CharacterInput`, runs it through the real `compute_pilot_base_chassis`, takes the
**computed** Charisma modifier off that computation, binds it as `CHA`, and evaluates the shared
formula with the crate's real `formula_interpreter::PcgenFormulaEvaluator` — the identical
"corpus-transcribed formula fed to the already-proven evaluator" discipline
`pilot_compute::domain_power` established under `OPERATOR-RULINGS-2026-08-21.md §20`, and the same
`grounded_magnitude` seam `AT-34-E3-003` proved for `Kind::Skill` and `AT-34-E4-002` for
`Kind::Trait`. Three independent guards, any one of which refuses rather than papers over:

1. the engine's computed Charisma modifier must equal this module's own hand-derived `+2`;
2. the evaluator must parse and resolve the formula at all;
3. the evaluated result must equal the independently hand-derived `10 + spell level + 2`.

**The fixture's Charisma is 14 (`+2`), deliberately not 10 (`+0`).** With a `+0` Charisma the
formula's `CHA` term contributes nothing, so a wrong binding — or none at all — would still produce
the right number. This is the "validate the proxy where it makes the confident claim" bar.

**Corpus fixture gate:** `tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`, 4 tests,
reading the live corpus directory rather than `include_str!`ing 115 files (the membership half of
the check is only meaningful if it sees every file, including one this cycle never knew about). It
pins: every entry's `spell_level`/`variable_slug`/`upstream_line` against its own record; that the
catalog covers **every** record carrying the full chain and no others; that exactly 3 records are
excluded and they are exactly the ones lacking the chain; and that the shared formula is the
reduced form of every record's own `DC` token.

**The 3 deliberately-excluded records, named not guessed at.** `Racial SLA ~ Dispel Magic`,
`~ Divine Favor`, `~ Suggestion` (`cr_abilities_race.lst:279/215/287`) carry the same seven
`DEFINE`s and the same `DC` formula but only a bare `SpellLVL` `BONUS` beside it — no `DCMod|CHA`
row — so under PCGen semantics their DC resolves against the `DEFINE`'s own `0` default, i.e. a DC
with no Charisma term at all. That is very likely an upstream `.lst` omission rather than a real
rule; shipping `10 + spell level` for them would be a specific, checkable, probably-wrong DC.

**Not grounded, and why:** uses-per-day (`Times|1|TYPE=Base` is a BASE the granting race's own row
raises, and the same record's three `SPELLS:` tokens carry `TIMES=ATWILL`/`TIMEUNIT=Constant`
variants — reporting "1/day" would misstate an at-will ability), and the spell's own effect (no
spell-resolution engine exists in this crate).

---

## 4. Mechanism 2 — a prose-bearing-raw-token guard, and the CR `ability` rung (102 bucket-D units)

**The shape.** Wave 50 added a rung inside `simple_kind_verdict` closing `core_rulebook`
zero-magnitude records with no real description for `template`/`language`/`skill`/
`race_trait_generic`. `ability` was not in that list, and it is that fallback's single largest
remaining Core Rulebook population.

**Read all 109 CR `ability` records in this shape directly.** 102 are `CATEGORY:Class Skill` +
`CSKILL:<skill>` rows — internal per-class class-skill LIST plumbing PCGen attaches to a class,
never a line item a player reads (the player reads the skill on the Skills panel, off
`skill_allocation`, a different record entirely). 1 is `"Default"`, a bare
`TEMPLATE:Bonus Language ~ Modern Human Language` grant with no other token at all. 1 is
`Belt of Dwarvenkind ~ Languages`, a bare `AUTO:LANG|Dwarven` grant.

**But `has_real_description == false` is NOT sufficient evidence that a record is proseless — and
this wave found the counterexample rather than shipping past it.** Of those 109, **6 records, out of
the 109 CR `ability` records in this evidence shape**, carry a real player-facing sentence in an
`ASPECT:` token their `data.description` does not hold:

- `Cloak of Displacement (Minor) ~ Miss Chance` — an `ASPECT:CombatBonus|...` token whose prose is a
  full sentence stating the item's miss chance against attacks made on the wearer
- `Unarmed Flaming Burst` / `Unarmed Icy Burst` / `Unarmed Shocking Burst` / `Unarmed Thundering` — `ASPECT:UnarmedNotes|...`

Closing those as "genuinely carries no upstream prose by design" would be **false**: they carry
prose the INGESTER dropped, which is a real ingestion gap, not a completion.

**The generic fix.** New `EngineFacts::corpus_json_prose_bearing_ability_tokens`, populated by
`load_corpus_json_prose_bearing_ability_tokens`, walking every observable book's `ability` corpus
directory and recording the coordinate of any record carrying non-trivial text in a raw
`DESC:`/`SPROP:`/`BENEFIT:`/`ASPECT:` token. Those four keys are exactly the four wave 33 lane A's
own "genuinely `description: null` upstream, not merely un-ingested" argument checked by hand for
the 9 wizard-school records — this makes that check mechanical and corpus-wide instead of a
per-cycle manual read. `.CLEAR`/`.CLEARALL`/empty/`[redacted PI]` are not prose, matching
`closure_has_real_description`'s own three exclusions.

The rung is applied in `classify()`'s `Kind::Ability` arm as a post-check on
`simple_kind_verdict`'s own verdict, deliberately NOT inside the shared function — so no other kind
that calls it can move, and the measured result confirms it: zero collateral movement.

**Two more records refused, and the precise root cause, named rather than forced.** `Fly` and
`Heal` are plain `CATEGORY:Class Skill` + `CSKILL:` rows (`cr_abilities.lst:600` / `:602`) with no
`.MOD` and no prose token of their own — read directly against the upstream `.lst` bytes to be
sure. They still resolve `has_real_description == true`, because their bare one-word names collide
with the same-named SKILL and SPELL records, whose `DESC:` the token closure picks up. That is the
"a shared name never implies a shared thing" corpus-scope-collision hazard, in the classifier's own
closure resolution. The guard's conservative direction is the correct one here, and this is a real,
newly-named pre-existing finding — not a defect this wave introduced, and not one it papers over.

---

## 5. Buckets B and C — investigated, deliberately not forced, named by mechanism

**Bucket C (191 units, 103 groups): the biggest single coherent family is `Monk Unarmed Damage LVL
<N> (<Size>)` — 48 units, out of the 191 core_rulebook bucket-C units** (6 band levels x 8
non-Medium sizes). Every one of the 54 size/level cells is unambiguously stated by its own corpus
record's `BONUS:VAR|PrimaryAttackDamage{Dice,Size}` tokens, dumped in full this cycle. The engine's
`monk_unarmed_strike_damage_die_for_size` returns `None` for every size but Small and Medium.

**A generic fix was designed for it and deliberately NOT shipped, for a real reason.** Transcribing
all nine columns is easy and correct. Making the 48 records `grounded` is not: this bundle's bar is
a magnitude OBSERVED reaching a consumer through a real pipeline run, and no character in this
engine can occupy any size but Small or Medium — `race_resolver::RACE_SIZES` gives all 18 playable
races exactly those two, `ChosenCharacterState` carries no size field, and no
template/size-change subsystem exists. The only way to report those 48 grounded would be to make
`monk_unarmed_strike_damage_die_for_size` public and have the probe call it directly — which is
asserting the engine against itself, exactly what the existing probe's own doc comment refuses. A
correct nine-column table with no reachable consumer is a real improvement to ship in a wave that
also builds the size subsystem; shipping the table alone would close 0 units while making the
"grounded" bar mean less. **Named for a future wave, with the mechanism identified and the blocker
named precisely: a character-size subsystem, not a damage table.**

**Bucket B (392 units): the biggest coherent family is `Domain Power ~ <X>` (58) + `Domain Base ~
<X>` (33) = 91 units, out of the 392 core_rulebook bucket-B units.** `pilot_compute::domain_power`
is exactly the right generic mechanism and its own doc comment says so ("adding a new domain means
transcribing ONE more formula string"). Dumping all 58 open `Domain Power` records' `DESC` text
this cycle found that, unlike the domains already in the catalog, most carry multiple `%N`
arguments, `PREVARLT`-gated formula variants, dice notation, or ally/enemy-facing effects that the
catalog's own documented exclusions already refuse. The genuinely catalog-shaped remainder is
small: `Chaos Blade` (`DomainChaosLVL/2`), `Holy Lance` (`DomainGoodLVL`), `Staff of Order`
(`DomainLawLVL/2`), `Scythe of Evil` (`DomainEvilLVL/2`) — all the `grounds_self_application: false`
weapon-quality-duration shape Undead Subdomain's Death's Kiss already established — plus
`Sun's Blessing` (`DomainSunLVL`). **5 catalog-shaped entries, out of the 58 open `Domain Power`
records**, each still needing its own sha256/line provenance and fixture check. Named precisely for
a future wave rather than half-done here.

**Ultimate Campaign (38 units): re-confirmed wave 50's own diagnosis, nothing new closable.** Wave
50 investigated all 38 and named each blocker precisely (the `COST:0` classifier gap for 18
Drawbacks, a downtime-retraining subsystem for 10 Retrains, choice-gating machinery for 2 traits, a
missing corpus file for 1). This wave's own `racial_sla` mechanism does not reach them (they are
`Kind::Trait`/`ability`, not `race_trait_generic`), and re-deriving confirmed the population is
unchanged at 38 actionable units out of 265 ultimate_campaign units.

---

## 6. A pre-existing instrument staleness found and closed (not caused by this wave)

Running the citation checks BEFORE touching anything showed **2 of the 3 citation instruments
already failed at HEAD `5f6b18f4e3`**:

- `shape_engine_boundary.py`: promotion-ladder citation pinned at `15274-15277`, actually at
  `16135-16138` at HEAD.
- `missing_engine_tables.py`: companion/power arms pinned at `16192`/`16292`, actually at
  `16306`/`16406` at HEAD.

Waves 49 and 50 both edited `src/bin/v06_work_inventory.rs` and both re-derived
`completion_atlas.py`'s ten citations only. The `--check` gates did exactly what they exist to do;
the gap was that nobody asked them. Both re-derived here for the post-wave-51 file, by content
grep and line-content read-back, never arithmetic alone.

`scripts/tests/test_shape_engine_boundary.py`'s own `not_held_by_engine` pin was ALSO stale —
carried as a KNOWN, deliberately-deferred open item since wave 44. **Closed here rather than
carried a seventh wave**, because a red pin in that file keeps the whole instrument's test red,
which is precisely how its promotion-ladder citation went two further waves unnoticed. Re-derived
to `8784`, and confirmed NOT moved by this wave: `not_held_by_engine` is 8784 in **both** this
wave's before and after inventory snapshots (this wave's 102 `engine-does-not-hold` closures all
carry `magnitude_token_count == 0`, so none is in the magnitude-bearing population that figure
counts over).

**Recommendation for a future wave, not done here:** neither `shape_engine_boundary.py --check`,
`missing_engine_tables.py --check`, nor `python3 -m unittest scripts/tests/
test_shape_engine_boundary.py` is a `scripts/verify.sh` stage, which is the root cause of all three
drifts. Adding them would change the wave-end gate's own 40-stage count, so it belongs in a wave
that also updates that gate — named, not silently attempted.

---

## 7. Verification — ONE pass, one targeted re-run

- `cargo test --locked --lib -j 6`: **3186 passed, 0 failed, 14 ignored** (up from the 3182 baseline
  by exactly the 4 new `rules_core::racial_sla` unit tests).
- `cargo clippy --locked --tests -j 6`: **3 warnings, all 3 in this wave's own new integration test
  file** (`collapsible_if` on one nested `if let` chain). Collapsed into a single let-chain; re-ran
  clippy on that target: **0 warnings**. The fix is a pure syntactic collapse in `tests/`, touching
  no `src/` line and therefore shifting no citation line number.
- `cargo test --locked --no-fail-fast -j 6`: run TWICE. Run 1 found ONE real failing expectation, which was a genuine catch, not a flake: `race_trait_grounding_tests::a_real_cross_book_sla_library_row_is_placed_by_the_generic_table` pins `Racial SLA ~ Aid`'s terminus, and this wave genuinely moves that unit from `ingested-magnitude` to `grounded` (a 2nd-level spell against the module's `+2` Charisma fixture, so DC 14). Re-derived that expectation and its doc comment, re-ran the bin target (624/624), then run 2: **8656 passed, 0 failed, 67 ignored, across 591 suites**, 0 clippy warnings. Stopped there.
- `cargo test --locked --test sd34_wave51_racial_sla_catalog_matches_the_corpus -j 6`: 4/4, re-run
  after the clippy fix.
- `python3 -m unittest scripts/tests/test_shape_engine_boundary.py`: 12/12.
- F1 pin (`src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs`): **not updated, and
  correctly so** — this wave adds no corpus record and changes no magnitude token, and the lib
  suite's own `corpus_wide_scan_population_matches_the_closed_gate1_census` passed unchanged.
- Guarded regen: `corpus_literal_sweep` **CLEAN** (48,706 records examined out of 51,476 read; 0
  findings), `derived_evaluator_fixture_check` **0 failed out of 1,839 units cleared**, then
  `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run --locked --bin
  v06_work_inventory`.
- Independent `id`->`status`/`evidence` join over the before/after `docs/work-inventory.json`
  snapshots: **exactly 217 units changed, out of a population of 49,438 units identical on both
  sides** — 115 `ingested-magnitude` -> `grounded`, 102 `engine-does-not-hold` -> `grounded`, **zero
  collateral movement** (every other unit's status/evidence pair byte-identical).
- `python3 scripts/completion_atlas.py --check`: `DONE 25906 -> 26123`, `M 4449 -> 4334` (-115),
  `D 2084 -> 1982` (-102), `unclassified=0`, `overlap=0`, `done_evidence_violations=0`,
  `citation_failures=0` (all ten re-derived).
- `python3 scripts/shape_engine_boundary.py --check`: `citation_ok=True`.
- `python3 scripts/missing_engine_tables.py --check`: `citation_failures=0`.
- `python3 scripts/denominator_gate.py --check`: **`files_checked=186 violations=0`** (one violation on the first run, a verbatim corpus `ASPECT:` quote whose literal percent figure the gate reads as a bare percentage -- rephrased to describe the token instead of quoting its number, re-run clean).
- `scripts/verify-baselines.env`: **raised, 3182 -> 3186 lib and 8648 -> 8656 full**, by exactly this wave's own 4 new `racial_sla` unit tests and 4 new corpus-fixture-gate tests -- raised, never lowered.

---

## 8. What remains, named by bucket / kind / mechanism

| bucket | kind | units remaining | the named mechanism it needs |
|---|---|---:|---|
| M | ability | 217 | per-record Spell-Like-Ability / `SPELLS:` chassis; no shared formula family found |
| M | equipment_modifier | 210 | 130 need a real equipment-effect compute path; 99 are `%CHOICE`-parameterized alias rows needing player-choice gating |
| M | template | 96 | `TEMPBONUS`/`TEMPVALUE` temporary-effect subsystem (78 of the 96 carry `TEMPBONUS`) |
| M | domain | 34 | new domain-content compute chassis; no `grounded` precedent for this kind |
| M | feat / skill / equipment / spell / race_trait | 106 | precedented `grounded_magnitude` seams, per-record (feat 47, skill 19, equipment 19, spell 15, race_trait 6) |
| B | class_feature | 392 | `Domain Power`/`Domain Base` (91) is the largest family; 5 of the 58 open `Domain Power` records are catalog-shaped today |
| C | class_feature | 191 | `Monk Unarmed Damage LVL <N> (<Size>)` (48) blocked on a character-size subsystem; ~85 pool groups still need `push_generic_pool_choice_magnitude` wired |
| D | class | 17 | `class_modelled_but_no_observed_delta_on_the_rendered_snapshot` |
| D | ability | 7 | 5 carry `ASPECT:` prose the ingester dropped (a real ingestion gap); `Fly`/`Heal` hit the name-collision closure hazard in §4 |
| D | class_feature / race_trait / template | 12 | unchanged from wave 50's own naming |
| M+D | ultimate_campaign | 38 | unchanged; wave 50's four named blockers all still stand |
