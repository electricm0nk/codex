# Cycle — SD-34 wave 50 — Core Rulebook + Ultimate Campaign, buckets B/C/D/M: 244 units closed (75 class_feature B, 169 D-shaped), plus 99 real cross-book units closed as an unplanned, honest side effect

- **Commit SHA:** `1a07f08bdc` (feat commit; this receipt's own SHA fill-in lands in a second,
  docs-only commit immediately after, following this bundle's own established two-commit pattern)
- **Files touched:** `src/bin/v06_work_inventory.rs` (`classify()`'s `Kind::ClassFeature` owner-matched
  text_only arm gains two new rungs; `simple_kind_verdict`'s shared zero-magnitude fallback gains one
  new CRB-scoped rung; one existing test updated to its new correct terminus), `src/rules_core/
  class_feature_pool_catalog.rs` (one existing live-query test's pinned `excluded` count re-derived
  213 -> 138 -- caught by this cycle's own second full-suite run, see Verification below),
  `scripts/completion_atlas.py` (all ten `BUCKET_DEFINITIONS` citations re-derived after this cycle's
  own two pure-insertion hunks), `docs/work-inventory.json` (regenerated via the guarded path),
  this receipt, `progress.md`, `decisions.md`, `kanban.md`.

## Why the wave's own scope had to be re-derived, not trusted

The dispatch brief characterized bucket M ("magnitude ingested, never computed") as the cleanest,
most mechanical, highest-yield bucket, and bucket B ("table exists, record not in it") as
mechanical table-entry additions. Direct investigation of the real engine found both
characterizations do not hold for Core Rulebook's own population:

- **Bucket M (778 CR units)** is not one shape. `equipment_modifier`'s 210 units split into 111
  records needing genuine per-item formula work through `compute_equipment_effects` and 99
  `%CHOICE`-parameterized alias rows (`BNS_AC_DEFL` etc, `CHOOSE:NUMBER` cost-scaled modifiers)
  needing real player-choice-gated computation, not a lookup-table fix. `ability`/`template`/
  `domain` (217+96+34 = 347 units) have **no existing `grounded` precedent anywhere in this
  engine at all** (confirmed: `grep` of every `status: "grounded"` unit corpus-wide by kind shows
  zero for these three) — each is genuinely new-chassis work, not a missing formula in an existing
  path. `race_trait`/`feat`/`skill`/`equipment`/`spell` (121+47+19+19+15 = 221 units) DO have
  established `grounded` precedent and are the genuinely mechanical slice of M — not attempted
  this wave given the time already spent characterizing the other three-quarters honestly.
- **Bucket B (467 CR units, all `class_feature`)** is not "add missing table entries" — there is
  no single static table. It is `classify()`'s `Kind::ClassFeature` arm failing to resolve an
  `owner`/promotion path for ~140 distinct option-pool group prefixes (Rage Power, Rogue Talent,
  Dragon Disciple, Sorcerer bloodline sub-groups, Domain sub-groups, ...), each a real, separate
  investigation. Two of those groups (Core Domain/Sorcerer Domain, Sorcerer Bonus Spell L1-L9)
  turned out to share the exact "internal, no-upstream-description, set-shaped chassis grant"
  shape `decisions.md §20`/`§21` already established precedent for — genuinely mechanical once
  found, but finding it required reading the real corpus records, not guessing from evidence-string
  text.

Given this, the wave targeted the concentrated, verified, low-risk mechanisms it found rather than
spreading effort thin across ~140 individually-unverified class_feature groups or three brand-new
compute subsystems.

## Fresh Core Rulebook + Ultimate Campaign population, re-derived via `_bucket_of()`'s own logic

| Book | Bucket | Units | Notes |
|---|---|---:|---|
| core_rulebook | M | 778 | by kind: ability 217, equipment_modifier 210, race_trait 121, template 96, domain 34, feat 47, spell 15, equipment 19, skill 19 |
| core_rulebook | B | 467 | all `class_feature`; 3 evidence shapes (owner-matched-not-held 237, option-pool-with-magnitude-not-held 205, option-pool-not-held 25) |
| core_rulebook | C | 191 | all `class_feature`, single evidence shape, spread across 103 distinct owner groups (largest: Rage Power 13, Rogue Talent 10, Dragon Disciple 9, Druid Domain 7) |
| core_rulebook | D | 307 | template 130, ability 109, language 22, class 17, skill 15, class_feature 9, race_trait 5 |
| ultimate_campaign | M | 36 | ability 30, trait 6 |
| ultimate_campaign | D | 2 | trait 2 |
| **Total actionable** | | **1,781** | matches the dispatch brief's own figure exactly |

## Root-cause naming (as requested)

**Bucket D's dominant shape** (`<kind>_content_table_holds_zero_magnitude_record_pending_wiring_class_review`,
`simple_kind_verdict` in `v06_work_inventory.rs`): fires when a record is held by its kind's table,
carries zero magnitude tokens, but fails the 3-part promotion gate
(`has_real_description && is_display_wiring_class_for_promotion(wc_class) && !universal_sheet_modifier`).
Sampled every Core Rulebook `template` (262 corpus files), `language` (22), `skill`
(the 15 D-bucket ones), and `race_trait_generic` (3) record directly: **all 169 of 169 CR D-bucket
units in this shape genuinely fail on the FIRST condition alone** — `has_real_description`
is `false` for all of them, and it is false BY DESIGN, not by an ingestion gap:
- `template` (130 units, 123 of them additionally carry the corpus's own `VISIBLE:NO`): internal
  PCGen kit/auto-application chassis (`"PC Level 11"`, `"Wild Shape"`, `"Righteous Might (Damage
  Reduction/Good)"`) — never player-read prose, confirmed against the upstream `.lst` rows too
  (no `DESC:`/`SPROP:`/`BENEFIT:` token on any of the 262).
- `language` (22 units): a language's own real "content" is its bare name — CRB's own
  `cr_languages.lst` carries no `DESC:` for any of its 22 languages by design (`Abyssal`'s only
  token is `TYPE:Spoken.Written.Read.Planar`).
- `skill` (15 units): all `"<Skill> (Untrained)"`/`"Untrained <Ability>"`/`"Untrained ~ <Skill>"` —
  internal usable-untrained bookkeeping records, never their own line item on a character sheet.
- `race_trait_generic` (3 units): vacuous placeholder sentinels (`"No Race Trait Available"`,
  `"Region ~ None"`, `"Region ~ Unknown"`), the same shape `class_feature_pool_catalog::
  vacuous_placeholder_reason` already names for `Kind::ClassFeature`.

This is genuinely the SAME "genuinely has no real upstream prose, set-shaped/internal record, by
design" ruling `decisions.md §20`/`§21` already established for two `class_feature` sub-causes —
extended here, for the first time, to four OTHER kinds sharing the identical shared classifier
fallback. Closed 169 of 169 sampled-and-confirmed CR units this way (one CR template unit, `Wild
Shape`, left open — its `has_real_description` resolves `true` through a wider token-closure read
this cycle did not trace further, so it correctly did not close).

**Bucket C's wiring gap** (`no_explanation_id_and_no_diagnostic_names_this_feature`, 191 CR units,
103 distinct owner groups): this bundle's own `AT-34-E3-002` mechanism
(`push_generic_pool_group_selection_magnitude` + `push_generic_pool_choice_magnitude` in
`pilot_compute/mod.rs`, wired into the classifier via `probe_cleric_domain_generic_member_wiring`/
`probe_sorcerer_bloodline_generic_member_wiring` in `v06_work_inventory.rs`) **already closes the
resolvable majority** of this shape for Cleric Domain and Sorcerer Bloodline — confirmed by direct
read: siblings of the same group (`Elemental Bloodline ~ Elemental Blast`/`~ Elemental Ray`/`~
Elemental Movement (Air)`) already carry `grounded`/`literal-verified` via
`generic_pool_group_selection_probe_observed_a_real_computed_magnitude`. What remains in the 191 is
the residual the generic formula interpreter genuinely cannot resolve: dice notation, non-numeric
qualitative grants (movement types, resistances), and multi-terminal records beyond its current
grammar — a genuine FORMULA-INTERPRETER gap, not a wiring gap, for the Cleric-Domain/
Sorcerer-Bloodline-shaped groups already wired; the remaining ~85 groups (Rage Power, Rogue Talent,
Dragon Disciple, Monk, Barbarian, Arcane Archer, ...) have never had this generic pass wired to
them at all (`push_generic_pool_choice_magnitude` exists and already serves Alchemist
Discovery/Spiritualist Phantom Emotional Focus, but no call site exists yet for Rage Power or Rogue
Talent) — real, per-pool wiring work for a future wave, not attempted this cycle given the
per-group verification cost (103 groups, ~2 units/group average).

## Units closed by bucket/kind

| Bucket | Kind | Group / evidence | CR closed | Cross-book bonus closed (same fix, other books) | Skipped, reason |
|---|---|---|---:|---:|---|
| B | class_feature | Core Domain | 31 | 71 (advanced_players_guide 64, bestiary_6 2, inner_sea_world_guide 2, bestiary_4 1, horror_adventures 1, ultimate_wilderness 1) | 2 CR (Destruction/Darkness — real `description`, but a leaked SPELL description, not domain-selection content; correctly left open) |
| B | class_feature | Sorcerer Domain | 22 | 0 | — |
| B | class_feature | Sorcerer Bonus Spell L1-L9 | 22 | 28 (occult_adventures 14, ultimate_magic 8, advanced_players_guide 4, advanced_race_guide 1, monster_codex 1) | 68 CR (real spell-derived description present — a genuine, different registered-pool-catalog opportunity for a future wave, not this "no description" shape) |
| D | template | (generic, CRB-scoped) | 129 | 0 (book-scoped deliberately) | 1 (`Wild Shape`, `has_real_description` resolves true via a wider closure) |
| D | language | (generic, CRB-scoped) | 22 | 0 | 0 |
| D | skill | (generic, CRB-scoped) | 15 | 0 | 0 |
| D | race_trait_generic | (generic, CRB-scoped) | 3 | 0 | 0 |
| **Total** | | | **244** | **99** | |

**Grand total this wave: 343 units closed** (244 inside this wave's CR+UC granted scope, 99 as a
real, independently-verified cross-book bonus from the same two engine fixes — the identical
"keyed on the record's own shape, not the book" payoff `AT-34-E4-002` cycle 3 already established
as legitimate rather than out-of-scope creep; every one of the 99 was spot-checked against its own
book's real corpus JSON before trusting it, not assumed from the classifier's own count).

**Ultimate Campaign: 0 units closed, all 38 investigated and named.** M-bucket (36): 18 `Drawback`
records carry only `COST:0` as their sole "magnitude" token (per `MAGNITUDE_TOKENS`, `COST:` counts
even at value 0) — 17 are genuinely narrative with no mechanical formula anywhere in their token
closure, 1 (`Drawback ~ Meticulous`) is a `PRESKILL`-gated cross-skill penalty; a real, precisely
named gap (`COST:0`-only records are forced into `ingested-magnitude` purgatory by the same
non-text_only branch a real flat bonus would use, rather than the zero-magnitude `text-complete`
path — recognizing "one token present, value functionally zero" as equivalent to `text_only` is a
real, scoped future fix). 10 `Retrain ~ *` records are a genuinely different subsystem (downtime
retraining bookkeeping, `BONUS:VAR|RetrainingDaysSpent|5`-shaped) this engine has no tracker for at
all. 2 `trait ~ *` records (`Blood of Dragons`, `Deathtouched`) are real `BONUS:ABILITYPOOL|<X>|1`
one-of-N choices needing the same new choice-gating machinery this bundle's own standing rule #3
already flags as out of scope for a quick close (`decisions.md`, Sanguine Angel precedent, wave
49). 1 (`Fate's Favored`) needs a cross-cutting "increase every active luck bonus by 1" mechanic no
part of this engine currently supports. 1 (`Loyalty across Lifetimes`) targets an eidolon (Summoner
companion subsystem) this engine does not model. 1 (`Sacred Conduit`) is tractable in principle
(flat +1 to several named Channel Energy DC variables) but is a single unit, not pursued given time.
D-bucket (2): `Trait ~ Alchemical Intuition` is `wc_class: "ambiguous"` (a once-per-day,
apply-after-rolling reroll-aid mechanic, not a flat bonus); `Trait ~ Wrecking Wrath (Rovagug)` has
no matching corpus file under `trait_generic/` by any name tried — a genuine, pre-existing corpus
data gap, the same shape already named for `trait_shadow_whispers` in `decisions.md`/`progress.md`.

## What remains, named for a future wave

- **CR bucket M, 778 units**: 221 (race_trait/feat/skill/equipment/spell) have established
  `grounded` precedent and are the highest-confidence next slice. 347 (ability/template/domain)
  need genuinely new compute-path chassis. 210 (equipment_modifier) split 111 real-formula / 99
  `%CHOICE`-parameterized (choice-gating machinery).
- **CR bucket B, remainder 392** (467 − 75 CR-closed this wave: 31 Core Domain + 22 Sorcerer Domain
  + 22 Sorcerer Bonus Spell L1-L9): ~140 remaining distinct option-pool groups, none individually
  verified this wave.
- **CR bucket C, 191 units, 103 groups**: the Rage Power/Rogue Talent/Monk/Barbarian-shaped flat
  pools need `push_generic_pool_choice_magnitude` call sites wired (the function already exists and
  already serves two other classes); the Dragon Disciple/Druid Domain-shaped group-selection pools
  need the SAME generic pass Cleric Domain/Sorcerer Bloodline already use, but for classes it has
  never been called for.
- **CR bucket D remainder, 138**: `ability` 109 + `class` 17 (`class_modelled_but_no_observed_
  delta_on_the_rendered_snapshot`, 17 distinct classes, each needing its own per-class probe
  investigation) + `class_feature` 9 (`class_feature_no_dedicated_magnitude_id_matched_the_record_
  slug` 6 + `class_feature_of_unmodelled_corpus_class:animal` 3) + `race_trait` 2
  (`race_trait_record_loaded_but_never_applies` — Human Ethnicity None/Unknown, confirmed the same
  "genuine upstream data gap, nothing fires the row" shape already ruled correctly-open for the
  Tribalistic Languages precedent, `v06_work_inventory.rs:13986`-area comment).
- **UC, 38 units**: see disposition above — 0 safely closable this wave, all named precisely.

## Verification

- `cargo build --locked --bin v06_work_inventory`: clean.
- `cargo test --locked --bin v06_work_inventory -j 6`: 624 passed, 0 failed (includes the one
  updated test, `a_real_zero_magnitude_pool_bookkeeping_row_is_placed_not_left_race_not_modelled`).
- `cargo test --locked --lib -j 6`: run 1, 3181 passed / **1 FAILED** (`class_feature_pool_catalog::
  tests::class_feature_owner_matched_non_excluded_remainder_is_24_and_named_by_subcause`, `excluded`
  left: 138, right (stale pin): 213 — a live-query test whose pinned excluded-class population
  needed re-deriving after this wave's own 75-unit closure moved Cleric/Sorcerer-owned units out of
  the evidence string it filters on). Fixed (`213 -> 138`, see doc comment at the assertion site).
  Run 2: 3182 passed, 0 failed, 14 ignored — the SAME 3182 as baseline (this wave adds no new lib
  test, only corrects one existing assertion's pinned value).
- `cargo test --locked --no-fail-fast -j 6`: run 1 stopped early (the lib failure above, 216 of
  589 suites observed clean before this cycle moved on to fix it). Run 2 (post-fix): **589 of 589
  suites, 8648 of 8648 tests passed** (byte-identical to the `BASELINE_ROOT_FULL_TESTS=8648`
  baseline -- this wave adds no new integration test), with one incidental exception: `--test
  sd13_paladin_level8_progression` reported `SIGKILL` mid-run -- traced to this cycle's own `pkill`
  cleanup of a duplicated, stale first invocation of this same command (an operator error, not a
  code defect: two `cargo test --no-fail-fast` processes were briefly running against the same
  `CARGO_TARGET_DIR` at once, and the cleanup for the first collaterally killed one of the second's
  own child processes). Re-ran that ONE target standalone: 14/14 passed, 0 failed, confirming the
  kill was environmental, not a real failure. `scripts/verify-baselines.env`: NOT changed (both
  counts already match the baseline exactly).
- `cargo clippy --locked --tests -j 6`: 0 warnings, both runs.
- F1/`shape_ledger.py` census: unchanged at 5124 (this wave's engine code touches no corpus data
  and closes only zero-magnitude/no-formula-token units, never a formula-bearing one).
- Guarded regen (`corpus_literal_sweep` CLEAN, `derived_evaluator_fixture_check` no findings,
  `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run --locked --bin
  v06_work_inventory`): `docs/work-inventory.json` regenerated, population unchanged at 49438,
  `generated_at: 2026-09-07T08:00:34Z`.
- Independently re-derived via a direct Python `id`→`status`/`evidence` join over the before/after
  snapshots (not just `completion_atlas.py`'s own bucket counts): exactly **343** units changed
  status, all `engine-does-not-hold` → `grounded`, **zero collateral movement** (id-set unchanged
  at 49438 both sides; every other status/evidence pair byte-identical).
- `completion_atlas.py --check`: before `DONE 25563 / B 11763 / D 2253`, after `DONE 25906 / B
  11589 / D 2084` (population 49438 unchanged both snapshots; 11763−11589=174 matches the
  class_feature closures exactly, 2253−2084=169 matches the template/language/skill/race_trait
  closures exactly). `citation_failures=0` (all ten `BUCKET_DEFINITIONS` citations re-derived this
  cycle after the two pure-insertion hunks this wave's own edits made — `git diff -U0` gave the
  exact +53/+61 breakpoints, each new line's content read back and confirmed, never the arithmetic
  alone).
- `denominator_gate.py --check`: first run found 3 violations (a bare percentage figure with no
  same-line denominator, one each in this receipt, `decisions.md`, and `progress.md` -- each one's
  real denominator was already stated, but on a markdown-wrapped adjacent line rather than the same
  raw line the gate scans) -- rephrased all three to state the denominator on the same line, re-ran
  clean: `files_checked=185 violations=0`.
