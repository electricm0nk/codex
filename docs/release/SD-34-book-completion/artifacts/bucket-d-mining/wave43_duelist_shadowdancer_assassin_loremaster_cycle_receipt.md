# Cycle — SD-34 wave 43 — Duelist/Shadowdancer/Assassin/Loremaster: 12 of 12 closed (small, precedented new compute)

- **Commit SHA:** `f3267fe099` (`f3267fe099e456303669f83eb15f5dd5e8ba69fa`)
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (14 new pure formula functions, 1 new
  const, 2 new class-id consts (`DUELIST_CLASS_ID`, `LOREMASTER_CLASS_ID`; `ASSASSIN_CLASS_ID`/
  `SHADOWDANCER_CLASS_ID` already existed), 4 new unconditional-on-chassis grounding functions
  (`ground_duelist_class_features`, `ground_shadowdancer_class_features`,
  `ground_assassin_class_features`, `ground_loremaster_class_features`), 4 new call sites inside
  `compute_pilot_base_chassis`, 1 new test module, 9 tests), `docs/work-inventory.json`
  (regenerated — real corpus-wide regen, not hand-edited),
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (a `--check`
  re-run's own artifact), this receipt, `progress.md`, `kanban.md`. **No `data/corpus/**` file
  touched. No `src/bin/v06_work_inventory.rs` change** (see "Classifier reachability" below).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/
  pilot_compute/mod.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (0 hits for `placeholder`/`STUB`/`MOCK`/
  `not yet implemented`/`fixme`/`hack` in this cycle's own diff).
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** implement 4 new per-class
  dispatch functions covering the remaining 12-unit "small-precedented-new-compute" population
  from `decisions.md §22`'s WAVE 42 UPDATE (Duelist ×4, Shadowdancer ×4, Assassin ×2, Loremaster
  ×2); re-verify every corpus row and precedent function directly rather than trusting the
  dispatch brief's own summary; add real unit tests (formula + reachability + negative control);
  run the guarded regen; run BOTH `cargo test --locked --lib` AND the full `cargo test --locked
  --no-fail-fast` integration suite (the step wave 42 skipped, which let a real regression
  through); re-derive before/after bucket counts naming exactly which of the 12 units moved.

## Pre-state (re-derived fresh, not trusted from the dispatch brief)

`python3 scripts/completion_atlas.py --check` at this cycle's own pre-edit HEAD (`af674409f5`,
wave 42's own closure commit): `population=49438 unclassified=0 overlap=0 citation_failures=0`,
`DONE: 25362`, `D: 2518` (every other bucket unchanged by this cycle). All 12 target units
confirmed, by direct `python3` filter over `docs/work-inventory.json`'s `units`, to be
`engine-does-not-hold` with the identical evidence string
`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`:

| Unit id | corpus_key | status (pre-cycle) |
|---|---|---|
| `core_rulebook:class_feature:duelist_canny_defense` | Duelist ~ Canny Defense | engine-does-not-hold |
| `core_rulebook:class_feature:duelist_improved_reaction` | Duelist ~ Improved Reaction | engine-does-not-hold |
| `core_rulebook:class_feature:duelist_precise_strike` | Duelist ~ Precise Strike | engine-does-not-hold |
| `core_rulebook:class_feature:duelist_elaborate_defense` | Duelist ~ Elaborate Defense | engine-does-not-hold |
| `core_rulebook:class_feature:shadowdancer_shadow_illusion` | Shadowdancer ~ Shadow Illusion | engine-does-not-hold |
| `core_rulebook:class_feature:shadowdancer_shadow_call` | Shadowdancer ~ Shadow Call | engine-does-not-hold |
| `core_rulebook:class_feature:shadowdancer_shadow_jump` | Shadowdancer ~ Shadow Jump | engine-does-not-hold |
| `core_rulebook:class_feature:shadowdancer_summon_shadow` | Shadowdancer ~ Summon Shadow | engine-does-not-hold |
| `core_rulebook:class_feature:assassin_save_against_poisons` | Assassin ~ Save against Poisons | engine-does-not-hold |
| `core_rulebook:class_feature:assassin_death_attack` | Assassin ~ Death Attack | engine-does-not-hold |
| `core_rulebook:class_feature:loremaster_lore` | Loremaster ~ Lore | engine-does-not-hold |
| `core_rulebook:class_feature:loremaster_secret_lore` | Loremaster ~ Secret Lore | engine-does-not-hold |

## Corpus re-verification (read directly, not trusted from `decisions.md §22`)

All rows read directly from `cr_abilities_class.lst` (the operator-supplied pinned oracle copy at
`docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/
data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst`).

- **Duelist ~ Canny Defense** (`:2987`): `DEFINE:CannyDefenseLVL|0` / `BONUS:VAR|CannyDefenseLVL|
  DuelistLVL` / `BONUS:COMBAT|AC|max(0,min(INT,CannyDefenseLVL))|TYPE=Dodge|PREMULT:...` — a dodge
  bonus equal to the lower of Intelligence modifier and duelist level, floored at 0. Grant gate
  `Duelist_CFP_Level,1` (`:363`).
- **Duelist ~ Improved Reaction** (`:2988`): `DEFINE:ImprovedReaction|0` / `BONUS:VAR|
  ImprovedReaction|floor((DuelistLVL+4)/6)*2` — a flat initiative bonus. Grant gate
  `Duelist_CFP_Level,2` (`:364`).
- **Duelist ~ Precise Strike** (`:2991`): `DEFINE:PreciseStrikeDamage|0` / `BONUS:VAR|
  PreciseStrikeDamage|DuelistLVL` — bonus weapon damage equal to duelist level. Grant gate
  `Duelist_CFP_Level,1` (`:363`, same gate as Canny Defense).
- **Duelist ~ Elaborate Defense** (`:2997`): `DEFINE:ElaborateParryLVL|0` `DEFINE:
  ElaborateDefense|0` / `BONUS:VAR|ElaborateParryLVL|DuelistLVL` then `BONUS:VAR|ElaborateDefense|
  ElaborateParryLVL/3` — an additional dodge bonus while fighting defensively/total defense.
  Grant gate `Duelist_CFP_Level,7` (`:369`).
- **Shadowdancer ~ Shadow Illusion** (`:3069`): `DEFINE:ShadowIllusionLVL|0` / `SPELLS:Class|
  TIMES=1|CASTERLEVEL=ShadowIllusionLVL|Silent Image,11+CHA` / `BONUS:VAR|ShadowIllusionLVL|
  ShadowdancerLVL` — an SLA, caster level = shadowdancer level. **Real corpus discrepancy,
  resolved deliberately**: the record's own DESC prose claims "once per day for every two
  shadowdancer levels", but the computed `SPELLS:` token is a literal `TIMES=1` (a flat,
  unconditional 1/day). Transcribed the literal token, the same authoritative-token-over-DESC-
  prose ruling `warpriest_channel_energy_dc`'s own doc comment already established for this
  bundle. Grant gate `Shadowdancer_CFP_Level,3` (`:399`).
- **Shadowdancer ~ Shadow Call** (`:3071`): `DEFINE:ShadowCallLvl|0` `DEFINE:ShadowCallTimes|0` /
  `BONUS:VAR|ShadowCallLvl|ShadowDancerLVL` / `BONUS:VAR|ShadowCallTimes|ShadowDancerLVL/2` — an
  SLA, caster level = shadowdancer level, uses/day = shadowdancer level / 2. Grant gate
  `Shadowdancer_CFP_Level,4` (`:400`).
- **Shadowdancer ~ Shadow Jump** (`:3072`): `DEFINE:ShadowJump|0` `DEFINE:ShadowJumpProgression|0`
  / four separate, cumulative `BONUS:VAR|ShadowJump|<N>|PREVARGTEQ:ShadowdancerLVL,<T>` tokens:
  `20`@4, `20`@6, `40`@8, `80`@10. Verified this is the SAME additive-`BONUS:VAR` multi-threshold
  idiom this codebase's own `alchemist_poison_resistance_bonus` already models as a cumulative
  nested if/else (each higher threshold's contribution adds to every lower threshold already
  met): `20` (levels 4-5), `40` (6-7, `20+20`), `80` (8-9, `20+20+40`), `160` (10+,
  `20+20+40+80`). **Real corpus discrepancy, resolved deliberately**: the record's own DESC
  prose describes exactly double this progression (`40`/`80`/`160`/`320` ft) — the same
  authoritative-token-over-DESC-prose treatment as Shadow Illusion above and Warpriest's own
  Channel Energy DC precedent, so the literal token sum is what this cycle transcribes, not the
  DESC narrative. Grant gate `Shadowdancer_CFP_Level,4` (`:400`, same as Shadow Call).
- **Shadowdancer ~ Summon Shadow** (`:3070`): `DEFINE:ShadowCompanionLVL|0` / `BONUS:VAR|
  ShadowCompanionLVL|ShadowdancerLVL` — a flat level-equivalence fact for the summoned shadow
  companion (its own DESC uses this for HP/BAB/save derivation, none of which this engine
  models). Grant gate `Shadowdancer_CFP_Level,3` (`:399`, same as Shadow Illusion).
- **Assassin ~ Save against Poisons** (`:2945`): `DEFINE:AssassinPoisonSaveBonus|0` / `BONUS:VAR|
  AssassinPoisonSaveBonus|AssassinLVL/2` — a flat poison-save bonus. The formula string
  `AssassinLVL/2` is already a literal in this repo's own `class_feature_grant_consumer.rs:2392`
  test fixture (`resolve_pcgen_var_chain_reproduces_a_single_hop_division_formula`), re-confirmed
  directly. Grant gate `Assassin_CFP_Level,2` (`:346`).
- **Assassin ~ Death Attack** (`:2947`): `BONUS:VAR|DeathAttackDC,DeathAttackDuration|AssassinLVL`
  — adds Assassin level to BOTH shared variables the generic `Death Attack` record (`:2869`,
  `BONUS:VAR|DeathAttackDC|10+INT`) already seeds. Since PCGen `BONUS:VAR` on the same variable
  stacks additively, the real save DC is `10 + INT + AssassinLVL`, matching the record's own DESC
  verbatim ("DC 10 + the assassin's class level + the assassin's Int modifier"). Duration adds
  `AssassinLVL` rounds to the base `1d6` (dice notation, not modelled). Grant gate
  `Assassin_CFP_Level,1` (`:345`).
- **Loremaster ~ Lore** (`:3020`): `BONUS:SKILL|TYPE=Knowledge|LoreMasterLVL/2` — a flat bonus on
  all Knowledge skill checks, usable untrained. Grant gate `Loremaster_CFP_Level,2` (`:377`).
- **Loremaster ~ Secret Lore** (`:3017`): `DEFINE:LoremasterSecretsLVL|0` `DEFINE:
  LoremasterSecretCount|0` / `BONUS:VAR|LoremasterSecretCount|(LoreMasterLVL+1)/2` — the SIZE of
  the loremaster secrets pool. Deliberately does NOT ground `LoremasterSecretsLVL`
  (`LoreMasterLVL+INT`, which secrets are selectable) or which secret is chosen at each slot —
  both are the spending question, mirroring `eidolon_evolution_pool`'s own "pool size is a real
  fact, spending is a different question" split. Grant gate `Loremaster_CFP_Level,1` (`:376`).

## Why unconditional dispatch from `compute_pilot_base_chassis`, not any enum-keyed chain

Confirmed directly (not assumed) that none of the four classes is a `ClassId`-family enum member
anywhere in this file: `grep -c "Duelist\|Shadowdancer\|Assassin\|Loremaster"` across every
`enum ClassId`/`enum AcgClassId`/`enum ApgClassId`/`enum AcgClassId`/`enum PuClassId` definition
returns zero. `ASSASSIN_CLASS_ID`/`SHADOWDANCER_CLASS_ID` already existed (added for the wave-21
weapon-and-armor-proficiency closure); this cycle adds `DUELIST_CLASS_ID`/`LOREMASTER_CLASS_ID`
alongside them. All four new grounding functions are therefore called directly from
`compute_pilot_base_chassis` (the same top-level function, same placement as
`ground_paladin_detect_evil`), keyed on `input.chosen.class_levels`'s raw `class_id` string —
never gated on `chassis_supported`, exactly mirroring `ground_paladin_detect_evil`'s own
reasoning.

## Classifier reachability — checked directly, not assumed

`class_feature_exact_suffix_grounded`'s 3-segment `<owner>.<feature_slug>.<descriptor>` shape
(the same mechanism wave 42's receipt traced in full) recognizes an id whose second-to-last dot
segment equals the corpus record's own `class_feature_engine_join_slug(feature)` slug, gated only
on `owner` matching the record's `group` text. All four owners (`"duelist"`, `"shadowdancer"`,
`"assassin"`, `"loremaster"`) are already PROVEN live by pre-existing sibling explanations before
this cycle touched anything: `class_feature.duelist.corpus_record.deflect_arrows`,
`class_feature.assassin.weapon_and_armor_proficiency`,
`class_feature.shadowdancer.weapon_and_armor_proficiency` (all pre-existing `text-complete`
units), and Loremaster's `owner` resolution depends only on the corpus's own `"Loremaster"` group
text via `class_feature_owner`'s generic `facts.class_books`/`facts.corpus_class_names` lookup,
not on any per-class registration list. Every one of the 15 explanation ids this cycle pushes
(4-segment `class_feature.<owner>.<feature_slug>.<descriptor>` shape) was checked by hand against
`class_feature_engine_join_slug`'s own transform (`"Canny Defense"` → `"canny_defense"`, `"Save
against Poisons"` → `"save_against_poisons"`, etc.) before writing any code. **No
`CLASS_FEATURE_ID_KNOWN_SYNONYMS` table entry and no `canonical_seeds_for()` match arm were
needed for any of the 12 units** — `src/bin/v06_work_inventory.rs` carries zero diff this cycle.

Confirmed the sweep actually reaches every grant level: `SWEEP_LEVELS = [1, 5, 10, 15, 20]`
(`v06_work_inventory.rs`) covers every one of this cycle's own grant gates (1, 2, 3, 4, 7), and
`class_sweep_input` builds a single-class character of the swept class at that level — every new
function here is race-independent by construction, so the fixture's own race is irrelevant.

## Tests (`src/rules_core/pilot_compute/mod.rs`)

New module `wave43_prestige_class_new_compute_tests`, 9 tests, all passing:

- `duelist_formulas_match_the_corpus_tokens` / `shadowdancer_formulas_match_the_corpus_tokens` /
  `assassin_formulas_match_the_corpus_tokens` / `loremaster_formulas_match_the_corpus_tokens` —
  direct pure-function tests for all 14 formulas, including edge cases the fixture cannot
  exercise (a negative Intelligence modifier for Canny Defense flooring at 0; the full level-band
  ladder for Shadow Jump).
- `duelist_class_features_reach_the_real_pipeline` / `shadowdancer_class_features_reach_the_real_
  pipeline` / `assassin_class_features_reach_the_real_pipeline` / `loremaster_class_features_
  reach_the_real_pipeline` — end-to-end: real characters run through `build_pilot_headless_
  receipt` at both a below-grant-gate level and an above-grant-gate level per feature, asserting
  the exact explanation id and value (or its absence below grant level).
- `none_of_the_fifteen_ids_leak_onto_an_unrelated_or_sibling_prestige_class` — a Fighter gains
  none of the 15 ids; each of the four prestige classes, at level 10 (above every grant gate),
  gains ONLY its own ids, never another class's.

**GREEN:**
- `cargo test --locked --lib -j 6 wave43_prestige_class_new_compute_tests` → 9/9 pass.
- `cargo test --locked --lib -j 6` (full workspace) → **3077 passed, 0 failed, 14 ignored** — up
  from the standing `BASELINE_ROOT_LIB_TESTS=3068` (wave 42's own figure) by exactly the 9 new
  top-level `#[test]` functions this cycle added.
- `cargo test --locked --no-fail-fast` (full workspace, ALL integration targets under `tests/*.rs`
  plus every unit test) → **exit 0, zero failures across every logged test block** — run in full
  this cycle (the step wave 42's own cycle skipped, which let a real regression through undetected
  until the wave-end gate). Every one of the ~570 `Running tests/*.rs`/lib/doctest blocks logged
  `test result: ok. ... 0 failed`, confirmed by direct read of the run's own output; the precise
  aggregate `count_passed` figure `scripts/verify.sh` would report was not independently re-derived
  a second time this cycle (a second full run solely to capture it would have cost this cycle's
  own turn budget another ~80 minutes it did not have room for after the first full run already
  ran to completion clean) — `scripts/verify-baselines.env`'s own updated entry states this
  honestly rather than fabricating a re-derived count.
- `cargo check --locked --lib -j 6` → exit 0.

## Guarded regen — ran to completion, prerequisites generated fresh

First attempt (`cargo run --locked --bin v06_work_inventory`, debug profile) correctly **refused**
(the same guard waves 41/42 both hit): `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_
REPORT` were unset. Generated both prerequisite reports fresh against this cycle's own tree
before re-running:

- `cargo run --locked --release --bin corpus_literal_sweep -- --json-out <path>` →
  `corpus-literal-sweep: 48706 records examined of 51476 read, 413314 tokens compared (9
  synthesized), 51463 digests checked, 0 findings` / `3138 tokens exempted under decisions.md §24
  redaction across 1058 codex_generated_name records` / **CLEAN** — byte-identical population
  figures to wave 42's own pre-cycle run (no `data/corpus/**` file touched by either cycle).
- `cargo run --locked --release --bin derived_evaluator_fixture_check -- --json-out <path>` →
  `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` — also byte-identical
  to wave 42's own figures.

Re-ran `cargo run --locked --bin v06_work_inventory` (debug profile) with both reports set —
completed cleanly (exit 0), no refusal, `docs/work-inventory.json` regenerated with
`"generated_at": "2026-09-05T07:09:45Z"`. `git status --porcelain -- docs/work-inventory.json`
shows `M` (modified, not stale).

## Movement — the real, regen-verified delta

Before/after bucket counts: "before" is `completion_atlas.py --check` run directly against
`docs/work-inventory.json` at this cycle's own pre-edit HEAD (`af674409f5`, wave 42's own
closure commit, captured before any edit this cycle made); "after" is the same command run
against the same path once the guarded regen completed. Independently cross-checked via a
direct Python join comparing every `id`'s own `status` and `evidence` fields between the
pre-cycle snapshot and the post-regen file (49438 units each side) — both methods agree exactly:

| Bucket | Before (`af674409f5`) | After (this cycle) | Δ |
|---|---:|---:|---:|
| DONE | 25362 | 25369 | **+7** |
| D | 2518 | 2506 | **−12** |
| V | 322 | 327 | **+5** |
| A / B / C / M / U / X / Z | unchanged | unchanged | 0 |

**Exactly 12 units changed status, zero collateral movement** — a full `id`→`status`/`evidence`
join between the pre- and post-regen inventories (49438 units each) finds precisely these 12
differences and no others. Interestingly, not all 12 landed in bucket DONE: 5 of the 12 landed
in bucket **V** (`"verified by proxy, never by the oracle"`, `literal-verified`/
`fixture-verified` — a distinct, separately-tracked resolved bucket per
`scripts/completion_atlas.py`'s own `BUCKET_ORDER`, not a failure or a lesser outcome; the exact
same D→V movement shape wave 41's own cycle hit for Monk's Stunning Fist). All 12 carry evidence
`explanation_id_observed_in_a_real_computation` in the raw work-inventory record; the DONE-vs-V
split is `completion_atlas.py`'s own downstream classification of which of the two "literal-
verified"/"fixture-verified"/"grounded" statuses `v06_work_inventory` assigned each unit —
neither this cycle's own code nor its own dispatch brief controls which of the two a given unit
lands in; it is a property of `v06_work_inventory`'s own verification logic, observed here rather
than chosen:

| Unit | corpus_key | status before | status after | bucket after |
|---|---|---|---|---|
| `core_rulebook:class_feature:duelist_canny_defense` | Duelist ~ Canny Defense | engine-does-not-hold | **grounded** | DONE |
| `core_rulebook:class_feature:duelist_improved_reaction` | Duelist ~ Improved Reaction | engine-does-not-hold | **fixture-verified** | V |
| `core_rulebook:class_feature:duelist_precise_strike` | Duelist ~ Precise Strike | engine-does-not-hold | **literal-verified** | V |
| `core_rulebook:class_feature:duelist_elaborate_defense` | Duelist ~ Elaborate Defense | engine-does-not-hold | **fixture-verified** | V |
| `core_rulebook:class_feature:shadowdancer_shadow_illusion` | Shadowdancer ~ Shadow Illusion | engine-does-not-hold | **grounded** | DONE |
| `core_rulebook:class_feature:shadowdancer_shadow_call` | Shadowdancer ~ Shadow Call | engine-does-not-hold | **grounded** | DONE |
| `core_rulebook:class_feature:shadowdancer_shadow_jump` | Shadowdancer ~ Shadow Jump | engine-does-not-hold | **grounded** | DONE |
| `core_rulebook:class_feature:shadowdancer_summon_shadow` | Shadowdancer ~ Summon Shadow | engine-does-not-hold | **literal-verified** | V |
| `core_rulebook:class_feature:assassin_save_against_poisons` | Assassin ~ Save against Poisons | engine-does-not-hold | **fixture-verified** | V |
| `core_rulebook:class_feature:assassin_death_attack` | Assassin ~ Death Attack | engine-does-not-hold | **grounded** | DONE |
| `core_rulebook:class_feature:loremaster_lore` | Loremaster ~ Lore | engine-does-not-hold | **grounded** | DONE |
| `core_rulebook:class_feature:loremaster_secret_lore` | Loremaster ~ Secret Lore | engine-does-not-hold | **grounded** | DONE |

**`population=49438 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 citation_failures=0`** — `completion_atlas.py --check`, this
cycle's own post-regen HEAD. `citation_failures=0` both before and after this cycle (no
`src/bin/v06_work_inventory.rs` line-number shift, since that file carries zero diff) — no
`scripts/completion_atlas.py` citation-pin re-derivation was needed this cycle.

## Figures (every number, its command, its denominator)

- `12` of `12` target units closed (named individually above) — this receipt's own before/after
  join, `docs/work-inventory.json`'s `units` array, 49438 total population both snapshots.
- `7` of the 12 landed in bucket DONE (`grounded`), `5` of the 12 landed in bucket V
  (`literal-verified`×2, `fixture-verified`×3) — named individually above.
- `14` new pure formula functions, `2` new class-id consts, `4` new unconditional-on-chassis
  grounding functions, `4` new call sites — this receipt's own code sections above.
- `9`/`9` new tests pass (`wave43_prestige_class_new_compute_tests`).
- `3077` `cargo test --locked --lib -j 6` pass, `0` failed, `14` ignored (up from the standing
  `3068` baseline by exactly 9 new top-level test functions).
- `DONE: 25362 -> 25369 (+7)`, `D: 2518 -> 2506 (-12)`, `V: 322 -> 327 (+5)` —
  `completion_atlas.py --check` against the pre-cycle (`af674409f5`) and post-cycle
  `docs/work-inventory.json`, both explicit denominators stated above.
- `0` diff in `src/bin/v06_work_inventory.rs` — `git diff --stat -- src/bin/v06_work_inventory.rs`.
- `0` citation pins re-derived (no shift; that file carries zero diff) —
  `scripts/completion_atlas.py`'s own `citation_failures=0` both before and after.
- `0` `data/corpus/**` files touched — `git diff --stat -- data/corpus/`.

## Build scope verified

- `cargo check --locked --lib -j 6` → exit 0.
- `cargo test --locked --lib -j 6` (full workspace) → 3077 passed, 0 failed, 14 ignored.
- `cargo test --locked --no-fail-fast` (full workspace) → exit 0, zero failures across every
  logged test block (see "Tests" section above for the honest note on why the exact aggregate
  count was not re-derived a second time this cycle).
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty,
  no file under `apps/desktop/` touched this cycle.

## Sweep population

`corpus_literal_sweep --json-out` (guarded regen chain step 1): `48706 records examined of 51476
read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings CLEAN`. No
`data/corpus/**` record was added, changed, or removed this cycle (`git diff --stat -- data/
corpus/` empty), consistent with `decisions.md §12` L8's rule.

`derived_evaluator_fixture_check --json-out`: `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — clean, required guarded-regen prerequisite.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — no
figure in this receipt was derived from the pinned oracle corpus directly; every magnitude
credited this cycle is a direct transcription of a `cr_abilities_class.lst` `BONUS:VAR`/`DEFINE`
token, re-verified against the pinned copy above. Cited for completeness per the receipt schema.

## Status

**complete.** All 12 target units closed and regen-confirmed, with a clean, fully-attributed
before/after delta and zero collateral movement. Both `cargo test --locked --lib` and the full
`cargo test --locked --no-fail-fast` integration suite were run and confirmed clean this cycle —
the exact step wave 42's own cycle skipped, which is what let that wave's real regression through
undetected until its own wave-end gate. `decisions.md §22`'s WAVE 42 UPDATE's own difficulty
assessment for this 12-unit population held up exactly as characterized: all 12 genuinely new
compute (no explanation id existed for any of them before this cycle), but small and fully
precedented, several with multiple existing byte-for-byte-comparable functions rather than just
one. Shape 2's new-chassis remainder from the original 15-unit list is now down to 1 unit —
Wizard's Arcane Bond.

## Movement, four buckets

- **Closure:** 12 (all 12 target units, `engine-does-not-hold` → `grounded` (7) or
  `literal-verified`/`fixture-verified` (5), named individually above).
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 0 (no citation-pin drift; `src/bin/v06_work_inventory.rs` carries
  zero diff).

## Notes (judgment calls)

- **Why no retro-log (`docs/retro/events/`) entry was added this cycle:** this bundle's own retro
  event log is reserved for `correction`/`incident`/`deferral` type events (confirmed across
  every `sd34-wave*.jsonl` file's own `type` field). This cycle is a plain closure with no
  correction of a prior wave's claim, no incident, and no declined/deferred scope, so no retro
  event applies — the same finding wave 42's own receipt made.
- **Why Shadow Illusion's and Shadow Call's spell-like-ability EFFECTS (not just caster
  level/uses-per-day) stay ungrounded:** the identical "ground the SLA triple, not the effect"
  split `ground_summoner_slice_a_features` already established for Summoner's own Summon Monster
  — no illusion/conjuration effect, spell DC, or Charisma-based save is modelled anywhere in this
  engine, so only the caster-level and uses-per-day FACTS are grounded here.
- **Why Loremaster's Secret Lore grounds only the pool SIZE, never a chosen secret:** mirrors
  `eidolon_evolution_pool`'s own precedent exactly — `LoremasterSecretsLVL` (which secrets are
  selectable, `LoreMasterLVL+INT`) and the actual secret chosen at each slot are the spending
  question, a different burden from the pool-size fact this cycle grounds.
- **Two real corpus discrepancies between DESC prose and the computed token, both resolved by
  transcribing the literal token** (Shadow Illusion's uses/day, literal `1` vs DESC's implied
  `floor(level/2)`; Shadow Jump's daily distance, literal cumulative `20/40/80/160` vs DESC's
  `40/80/160/320`) — both follow the exact "authoritative computed token over DESC narrative"
  ruling `warpriest_channel_energy_dc`'s own doc comment already established for this bundle, not
  a new precedent.

## Next-cycle plan

1. **Shape 2's remaining new-chassis scope after this cycle:** Wizard's Arcane Bond (1 unit) —
   the only unit of the original 15-unit list, per `decisions.md §22`'s own repeated finding,
   that holds up as genuinely open-ended new-chassis work requiring real subsystem modeling
   (weapon enhancement bonuses, no precedent anywhere in the engine). Every other unit in that
   15-unit list is now closed (Paladin's Detect Evil, Cleric's Aura in wave 42; these 12 this
   cycle).
2. **Sub-mechanism 5** (634 units across 60 classes, per wave 37 lane B's own corrected figure)
   remains un-re-audited since `decisions.md §22`'s own correction — unchanged from wave 42's own
   note, out of this cycle's own narrow 12-unit scope.
3. **`THE-BOX.md`/`box_ledger.py --check` staleness** was not checked this cycle — out of this
   cycle's own narrow scope, consistent with every prior wave's own note.
