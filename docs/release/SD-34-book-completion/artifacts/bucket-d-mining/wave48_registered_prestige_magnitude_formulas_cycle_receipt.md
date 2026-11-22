# Cycle — SD-34 wave 48 — Twilight Talon and Golden Legionnaire's magnitude-only remainder: 16 units closed

- **Commit SHA:** `7261a35281` (`7261a35281a310ef4e32d29255a722f9566619e0`, feat commit; this
  receipt's own SHA fill-in lands in a second, docs-only commit immediately after, following this
  bundle's own established two-commit pattern)
- **Files touched:** `src/rules_core/pilot_compute/mod.rs` (2 new class-id consts
  `TWILIGHT_TALON_CLASS_ID`/`GOLDEN_LEGIONNAIRE_CLASS_ID`; 5 new choice-set-id consts
  `TWILIGHT_TALON_TATTOO_LEVEL_{2,4,6,8,10}_CHOICE_ID`; 1 new tier table
  `TWILIGHT_TALON_TATTOO_TIERS` (with a `TwilightTalonTattooTier` type alias added mid-cycle to
  clear a clippy `type_complexity` warning); 6 new pure formula functions
  (`twilight_talon_sneak_attack_dice`, `twilight_talon_enhanced_tattoo_save_dc`,
  `golden_legionnaire_allied_retribution_bonus`, `golden_legionnaire_authoritative_command_bonus`,
  `golden_legionnaire_improved_aid_bonus`, `golden_legionnaire_united_defense_bonus`); 2 new
  `ground_<class>_class_features` dispatch functions; 2 new call sites inside
  `compute_pilot_base_chassis`; 1 new test module,
  `wave48_registered_prestige_magnitude_formulas_tests`, 9 tests), `src/bin/v06_work_inventory.rs`
  (2 new `EngineFacts` fields, 1 probe-local tier table `TWILIGHT_TALON_TATTOO_TIER_MEMBERS` (same
  type-alias fix), 2 new probe functions (`probe_golden_legionnaire_wiring` reusing
  `probe_wave46_single_owner_class_features`, `probe_twilight_talon_wiring` sweeping all 10
  choice-gated tattoo selections the same `probe_divine_scion_wiring` idiom already established), 2
  new call sites, 2 new `classify()` early-return checks, 1 new
  `canonical_seeds_for("twilight_talon")` arm (5 choice seeds, one per tier), 2 new test modules —
  `wave48_registered_prestige_probe_reachability_tests` (2 real-pipeline reachability tests) and
  `wave48_registered_prestige_classify_tests` (3 classify()-dispatch proofs: 2 positive + 1 negative
  control) — 5 tests total), `scripts/completion_atlas.py` (10 bucket citation-pin re-derivations —
  this cycle's own insertions shifted every downstream construction-site line number in
  `v06_work_inventory.rs`, the same pattern every prior wave in this series hit; the bucket-C
  `explanation_id` pin had gone SILENTLY stale — `--check` did not flag it because the shifted line
  still happened to contain the bare substring, the same hazard waves 35/38/40 already hit, caught
  by reading the line back, not trusted from the string match alone), `scripts/shape_engine_boundary.py`
  / `scripts/missing_engine_tables.py` / `scripts/tests/test_shape_engine_boundary.py` (citation-pin
  re-derivations for the same reason — the pre-existing, unrelated `not_held_by_engine` population
  drift named by waves 44-47 is left named, not fixed, same as those waves' own choice),
  `docs/work-inventory.json` (regenerated via the guarded path), `docs/release/SD-34-book-
  completion/artifacts/epic-1-atlas/completion-atlas.json` / `missing-engine-tables.json` /
  `shape-engine-boundary.md` (each script's own `--check` re-run rewrites its artifact as a side
  effect, matching wave47's own precedent), `scripts/verify-baselines.env`, this receipt,
  `progress.md`, `decisions.md`, `kanban.md`.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/
  pilot_compute/mod.rs src/bin/v06_work_inventory.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}`
  hits outside this wave's own house-style `wave48_*` module names, matching the file's own
  existing `wave4[1-7]_*` convention).
- **Wired-integration audit result:** `OK_NO_TOKENS` (0 hits for `placeholder`/`STUB`/`MOCK`/
  `not yet implemented`/`fixme`/`hack` in this cycle's own diff).

## Fresh sub-mechanism-5 population re-derivation

Queried `docs/work-inventory.json` directly (pre-edit, at wave 47's own wave-end-gate commit
`f207f7781453903c4dec957f821fd4e1adec5dc6`) for units whose evidence contains
`class_feature_of_unmodelled_corpus_class`: **591 total**, matching this wave's own dispatch brief
exactly (no drift since wave 47's close). Cross-referenced against the 74-entry
`tests/fixtures/rules_core/prestige-class-entry-requirements.json` registry: **503 registered / 88
not registered**, the 88 unchanged from prior waves (`psychic_detective` 18, `animal` 17, `eidolon`
16, `phantom` 9, `plant` 9, `undead` 8, `dragon` 8, `gifted_blade` 3). Of the 503 registered, **116**
belong to the excluded cross-class-manifester-level group (Sighted Seeker 21, Thrallherd 19, Psion
Uncarnate 13, Elocater 13, Psicrystal Imprinter 12, Metamind 10, Phrenic Slayer 11, Cerebremancer 9,
Soul Archer 4, Metaforge 4) and **2** are Divine Scion's own True Scion remainder, leaving **385** as
this wave's real working pool across 44 prestige classes — matches the dispatch brief's own
591/88/116/2/385 split exactly, re-derived independently rather than trusted.

## Classes picked and why

**Twilight Talon** (17 units, `adventurers_guide`) and **Golden Legionnaire** (16 units,
`adventurers_guide`) — both already-registered prestige classes with simple, precedented formula
shapes and, critically, no PI-blacklisted class name. Checked directly before picking either: an
ingested `data/corpus/adventurers_guide/class_feature/<slug>/` directory exists for both (unlike
Aldori Swordlord and Magaambyan Arcanist, two other large candidates in the same working pool,
whose class NAMES are Product Identity and whose records ingest only under a redacted
`codex_named_unit_*` marker — see the finding below). Twilight Talon's Sneak Attack is the classic
sneak-attack-dice-by-level idiom; its Enhanced Tattoo is the classic "10 + level factor + ability
modifier" save-DC idiom (`argent_dramaturge_argent_performance_dc`'s own precedent within this same
book) PLUS 5 genuine `ABILITYPOOL` one-of-two choices (gated per this bundle's own wave-47 lesson);
Golden Legionnaire's four magnitude-bearing features are all flat step-bonuses (`1+(LVL>=N)`), the
same `i16::from(level >= n)` idiom already used repeatedly elsewhere in this file.

## The 16 units closed, and what was left named

**Twilight Talon (12 of 17 closed):** Sneak Attack, Enhanced Tattoo's own save DC, and all 10
per-tier tattoo caster-level records (Disguise Self/Undetectable Alignment at tier 2, Alter
Self/Invisibility at tier 4, Glibness/Secret Page at tier 6, Modify Memory/Zone of Silence at tier
8, Mislead/Seeming at tier 10) — every formula read directly from its own corpus record
(`ag_abilities_class.lst:540-557`) and cross-checked against the real, non-ingested PCGen oracle.
Left named, not attempted: Many Hats, Eye for Detail, Dead Drop, Resourceful Agent, Unassuming
Presence — all pure prose, no `BONUS`/`DEFINE` token.

**Golden Legionnaire (4 of 16 closed):** Allied Retribution, Authoritative Command, Improved Aid,
United Defense — every formula read directly from its own corpus record (`ag_abilities_class.lst`
Golden Legionnaire section) and cross-checked against the real, non-ingested PCGen oracle. Left
named, not attempted: Bodyguard/Defy Danger/Guardian of Liberty/Hold the Line/In Harm's Way/
Intercept/Preemptive Strike/Retaliate/Stand Still/Swift Aid (pure prose or automatic single-feat
grants, no magnitude token) and Combat Feat/Legion Feats (a real bonus-feat `ABILITYPOOL` this
cycle chose not to model a bare pool-of-feats magnitude for — a future wave's own scoping
question, not a correctness gap; each of Legion Feats' own 4 threshold pools has exactly one
AUTOMATIC member, not a genuine choice, so it is not the same shape the ABILITYPOOL lesson below
targets, but a "pool size = feats granted so far" magnitude was left unattempted for time rather
than modeled speculatively).

## The ABILITYPOOL choice-gating lesson applied

Before grounding Enhanced Tattoo's 5 per-tier pools, checked every sibling record's own shape per
wave 47's own generalizable finding: each of the 5 "Twilight Talon Tattoo Level N" pools
(`ag_abilities_class.lst:542`'s own 5 `PREVARGTEQ`-gated `BONUS:ABILITYPOOL` tokens) is its own
genuine one-of-two `ABILITYPOOL` selection, gated here via 5 separate `choice_selection(input,
<CHOICE_ID>)` checks (one per tier, since a character accumulates a NEW tattoo at each tier reached
over their career, unlike Divine Scion's single career-long Domain Specialization pick) — never
grounded unconditionally for both candidates at once. A RED-for-the-right-reason test
(`twilight_talon_tattoo_ids_are_absent_with_no_recorded_selection`) proves the fix.

## A new finding for future waves: PI-name-blacklisting is a class-level property, not a book-level one

Twilight Talon and Golden Legionnaire's own DESC prose is scrubbed (`DESCISPI:YES`, matching the
same redaction this whole book carries throughout) but their class names and formula tokens ingest
normally under an ordinary directory. Aldori Swordlord and Magaambyan Arcanist (both investigated
this cycle, both skipped) are the stricter case: their class NAME itself is Product Identity, so
their records ingest under a redacted `codex_named_unit_*` directory with `data.class` replaced by
the redaction marker — confirmed directly against `src/rules_core/cache_gen/class_feature.rs`'s own
existing test
(`generate_redacts_a_class_field_resolved_from_a_real_corpus_class_name_that_is_itself_pi`), which
names Aldori Swordlord as its own worked example. **Check for an ingested directory under the
class's own slug (`data/corpus/<book>/class_feature/<slug>/`) before scoping a future wave against
any `adventurers_guide` prestige class** — its absence is the cheap signal this cycle used to skip
both without writing PI-blacklisted content into new source citations.

## Tests

- `pilot_compute/mod.rs`'s `wave48_registered_prestige_magnitude_formulas_tests` (9 tests): pure
  formula tests for all 6 new functions, a real-pipeline reachability test for the 6 unconditional
  facts, a RED-for-the-right-reason test proving the 10 choice-gated tattoo facts are ABSENT with
  no recorded selection, a test proving exactly the recorded tier/member surfaces and no sibling
  leaks, a level-gate test isolating the tier gate from the choice gate, and a negative control
  proving none of the new ids leak onto an unrelated class.
- `v06_work_inventory.rs`'s `wave48_registered_prestige_probe_reachability_tests` (2 tests, proving
  all 12 Twilight Talon members and all 4 Golden Legionnaire members resolve through the real
  `compute_pilot_base_chassis` pipeline, exact-count-asserted) and
  `wave48_registered_prestige_classify_tests` (3 tests: 2 classify()-dispatch proofs + 1 negative
  control).

## Verification (mandatory)

`cargo check --locked --lib -j 6` → exit 0. `cargo check --locked --bin v06_work_inventory` →
exit 0.

`cargo test --locked --lib -j 6 wave48_registered_prestige` → 9 passed, 0 failed.
`cargo test --locked --bin v06_work_inventory -j 6 wave48` → 5 passed, 0 failed.

`cargo test --locked --lib -j 6` (full lib suite), **run 1** (before the clippy fix) → **3143
passed, 0 failed, 14 ignored** (up from the standing 3134 baseline by exactly this cycle's 9 new
lib tests).

`cargo test --locked --no-fail-fast -j 6` (full workspace), **run 1** → **exit 0**, all 543 test
suites plus the lib suite plus doc-tests green (confirmed via the run's own zero exit code; the
final tail of output showed the last three suites — `v06_wizard_rogue_skill_allocation_grounding`,
`v06_work_inventory`, `wizard_abjuration_school_powers` — all passing, and doc-tests `0 passed`).

`cargo clippy --locked --tests -j 6` (run against the fully-settled tree) → **2 warnings found**:
`clippy::type_complexity` on both `TWILIGHT_TALON_TATTOO_TIERS` (`pilot_compute/mod.rs`) and
`TWILIGHT_TALON_TATTOO_TIER_MEMBERS` (`v06_work_inventory.rs`) — the exact wave-47-named hazard
(this bundle's own zero-warning ceiling). Fixed with a `type` alias in each file
(`TwilightTalonTattooTier` / `TwilightTalonTattooTierMember`) — pure type-level renaming, no
behavior change. Re-ran `cargo clippy --locked --tests -j 6` → **0 warnings**.

This clippy fix is a real `.rs` edit landing AFTER the first full-suite run, so both suites were
re-run a SECOND time end to end against the fully-settled tree: `cargo test --locked --lib -j 6`,
**run 2** → **3143 passed, 0 failed, 14 ignored** (byte-identical to run 1). `cargo test --locked
--no-fail-fast -j 6`, **run 2** → **8576 passed, 0 failed, 67 ignored, across 589 suites, exit 0**.

## Guarded regen

`corpus_literal_sweep --json-out` → `48706 records examined of 51476 read, 413314 tokens compared
(9 synthesized), 51463 digests checked, 0 findings` / `3138 tokens exempted under decisions.md
§24 redaction across 1058 codex_generated_name records` / **CLEAN** — byte-identical to wave 47's
own pre-cycle figures (`git diff --stat -- data/corpus/` empty; no corpus file touched this
cycle).

`derived_evaluator_fixture_check --json-out` → `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — also byte-identical.

`cargo run --locked --bin v06_work_inventory` with both reports set → completed cleanly (exit 0),
no refusal, `docs/work-inventory.json` regenerated (`git status --porcelain` shows `M`, not
stale).

## A second citation-pin hazard, caught before commit: pin re-derivation must happen AFTER the LAST real `.rs` edit

The 10 `completion_atlas.py` bucket pins, the `shape_engine_boundary.py` promotion-ladder block,
the two `missing_engine_tables.py` pins, and `test_shape_engine_boundary.py`'s two anchor
assertions were originally re-derived BEFORE the `clippy::type_complexity` fix (the `type
TwilightTalonTattooTierMember` alias) was applied — that alias sits above every one of these
citation sites in `v06_work_inventory.rs`, so its insertion shifted all of them by a further
uniform **+4** lines. Re-running `python3 scripts/completion_atlas.py --check` (and the sibling
scripts) one final time before committing caught this immediately: **all 10** completion-atlas
citations failed simultaneously (`citation_failures=10`), an unambiguous signal (a single-digit
subset failing would suggest an unrelated per-site staleness; every one failing by the identical
+4 offset means one shared upstream insertion). Corrected every citation (`completion_atlas.py`'s
10, `shape_engine_boundary.py`'s 4-line promotion ladder + its `_read_source_lines` slice + its
anchor constant, `missing_engine_tables.py`'s 2, `test_shape_engine_boundary.py`'s 2 assertions),
re-ran every `--check` and the Python unit-test module: `citation_failures=0` across all three
scripts, `python3 -m unittest scripts.tests.test_shape_engine_boundary` → 11 of 12 pass (the sole
failure is the pre-existing, already-named `not_held_by_engine` population drift — unrelated,
not wired into `verify.sh`, left as-is same as waves 44-47's own choice). **Lesson for a future
wave:** citation-pin re-derivation is not a one-time step done once the code "looks settled" — it
must be the LAST thing done before commit, re-run again after every real `.rs` edit including a
late clippy fix, however small (a single `type` alias is enough to shift every downstream site).

## Before/after bucket movement

`python3 scripts/completion_atlas.py --check` at this cycle's own pre-edit HEAD
(`f207f7781453903c4dec957f821fd4e1adec5dc6`, wave 47's own wave-end-gate commit):
`population=49438 unclassified=0 overlap=0`, `DONE: 25458`, `D: 2398`, `V: 349` — exactly the
wave 47 gate baseline named in this wave's dispatch brief.

**Post-regen:** `python3 scripts/completion_atlas.py --check`: `population=49438 unclassified=0
overlap=0 citation_failures=0`, `DONE: 25458→25474 (+16)`, `D: 2398→2382 (−16)`, `V: 349→349
(unchanged, +0)` — every one of this wave's 16 closures landed straight in `grounded` (DONE); none
landed in `literal-verified`/`fixture-verified` (V), since none of the 16 have a corresponding
`derived_evaluator_fixture_check` fixture row and none needed the `corpus_literal_sweep`-only path
(unlike several prior waves' own D→V shape) — a legitimate, different outcome, not a lesser one.

Independently re-derived via a direct Python `id`→`status` join over both the pre- and post-regen
inventory snapshots (not just `--check`'s own summary): pre/post population both 49438, 0 added, 0
removed, **exactly 16 units changed status, zero collateral movement** — all 16 are
`adventurers_guide:class_feature:{twilight_talon,golden_legionnaire}_*` ids, every one
`engine-does-not-hold` before, every one `grounded` after (12 `wiring_class: derived`, 2
`wiring_class: computed` for the two "parent" records with multiple magnitude tokens — Golden
Legionnaire Authoritative Command and Twilight Talon Enhanced Tattoo — the remaining 12 also
`derived`). No other unit in the 49438-id population moved.

## F1/shape_ledger pin

**Unchanged: 5155 → 5155.** Verified per-id, not assumed: ran `shape_ledger.py` against the
PRE-cycle inventory and checked all 16 closed ids directly — **0 of the 16 are F1-shaped**. Golden
Legionnaire's 4 step-bonuses and Twilight Talon's Sneak Attack/Enhanced Tattoo (the two
"container" records) are `F2` (per-level scaling, since `1+(LVL>=N)` and `(LVL+2)/3` are both
level-arithmetic expressions, not bare literals); the 10 per-tier tattoo caster-level records are
`F0` (their own `SPELLS:Innate|CASTERLEVEL=TL|...` token is not recognized as a formula token by
this classifier's own token scan, the same blind spot Divine Scion's domain records already hit).
Re-ran `shape_ledger.py` against the post-regen inventory to confirm the population-level
arithmetic: `F1 = 5155` exactly, unchanged. No pin update needed in
`formula_interpreter_corpus_wide.rs` (confirmed the file's own current pin already reads `5155`,
matching, before touching anything).

## Findings for a future wave

1. **Sub-mechanism 5's remaining population after this wave: 575** (591 − 16), split **487
   registered** (across the SAME 44 prestige classes as before this wave — neither Twilight Talon
   nor Golden Legionnaire was fully closed, so neither drops out of the working-pool class count;
   largest remaining: cyphermage 17, psychic_fist 16, asavir 15, metamorph 15, war_mind 15,
   hellknight 14, adaptive_warrior 14, aldori_swordlord 13 [PI-name-blacklisted, see above],
   sanguine_angel 13, body_snatcher 13, golden_legionnaire 12 [own remainder], steel_falcon 12,
   lantern_bearer 11, magaambyan_arcanist 11 [PI-name-blacklisted], storm_kindler 11,
   westcrown_devil 11, pyrokineticist 11, and roughly 25 smaller classes) and **88 not-registered**
   (unchanged, named by slug in prior waves' gate entries). The ≥10-class AS/MB/Ma
   cross-class-manifester-level population is unchanged, untouched this wave.
2. **Twilight Talon's own 5 remaining units** (Many Hats, Eye for Detail, Dead Drop, Resourceful
   Agent, Unassuming Presence) — pure prose, no magnitude token, left named.
3. **Golden Legionnaire's own 12 remaining units** — Combat Feat/Legion Feats (bonus-feat
   `ABILITYPOOL`, needs a "pool of feats granted so far" magnitude modeling decision) plus 10
   pure-prose/automatic single-feat-grant records, left named.
4. **PI-name-blacklisting check, generalized:** before scoping a future wave against ANY
   `adventurers_guide` prestige class, confirm `data/corpus/adventurers_guide/class_feature/<slug>/`
   exists as an ingested directory — its absence (Aldori Swordlord, Magaambyan Arcanist confirmed
   this cycle) means the class name itself is Product Identity and needs the
   `codex_named_unit_*`-aware redaction discipline `src/rules_core/cache_gen/class_feature.rs`
   already established, a materially different (harder, PI-sensitive) shape than an ordinary
   closure.

## Build scope verified

- `cargo check --locked --lib -j 6` → exit 0.
- `cargo check --locked --bin v06_work_inventory` → exit 0.
- `cargo test --locked --lib -j 6 wave48_registered_prestige` → 9 passed, 0 failed.
- `cargo test --locked --bin v06_work_inventory -j 6 wave48` → 5 passed, 0 failed.
- `cargo test --locked --lib -j 6` (full lib suite), run 2 (post-clippy-fix) → 3143 passed, 0
  failed, 14 ignored.
- `cargo test --locked --no-fail-fast -j 6` (full workspace), run 2 (post-clippy-fix, the final
  fully-settled run) → 8576 passed, 0 failed, 67 ignored, 589 suites, exit 0.
- `cargo clippy --locked --tests -j 6` → 0 warnings (after the `type_complexity` fix).
- `scripts/verify-baselines.env` updated: `BASELINE_ROOT_LIB_TESTS` 3134→3143,
  `BASELINE_ROOT_FULL_TESTS` 8562→8576 (dated entry appended following the file's own
  convention).
- `python3 scripts/completion_atlas.py --check` (post-regen, re-run independently after the full
  suite): `population=49438 unclassified=0 overlap=0 citation_failures=0`, bucket counts match
  this receipt's own before/after table exactly.
- `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`
  (re-run independently after the full suite): `F1 = 5155`, matching this receipt's own F1/pin
  section exactly.
- `python3 scripts/denominator_gate.py --check`: `files_checked=182 violations=0`.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty, no
  file under `apps/desktop/` touched this cycle.
