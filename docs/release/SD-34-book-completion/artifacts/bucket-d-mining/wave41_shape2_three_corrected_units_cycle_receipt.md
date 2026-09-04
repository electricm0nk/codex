# Cycle — SD-34 wave 41 — Shape 2's three corrected units (Monk's Stunning Fist, Fighter's Weapon Training, Psychic's Phrenic Pool): 3 of 3 closed

- **Commit SHA:** `<FILL_IN_AFTER_COMMIT>`
- **Files touched:** `src/bin/v06_work_inventory.rs` (`CLASS_FEATURE_ID_KNOWN_SYNONYMS` extended
  with 1 new `("monk", "stunning_fist", "feat.standalone.stunning_fist.save_dc")` entry;
  `canonical_seeds_for()` extended with 2 new match arms, `"fighter"` and `"psychic"`; 2 stale
  doc comments corrected to reflect the narrowed — not eliminated — Fighter weapon-training-probe
  gap; 4 new tests, 1 existing test edited), `scripts/completion_atlas.py` (10 citation-pin line
  numbers re-derived — this cycle's own +128-line insertion, in two hunks, both landing above
  every one of the 10 pins except the second which sits below all of them, so a single uniform
  +128 shift applies to all 10), `docs/work-inventory.json` (regenerated — real corpus-wide
  regen, not hand-edited), `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json` (a `--check` re-run's own artifact, byte-consistent with the re-derived
  citations and the regenerated inventory), this receipt, `progress.md`, `kanban.md`,
  `docs/retro/events/sd34-wave41.jsonl` (new, 3 correction entries). **No `data/corpus/**` file
  touched.**
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/bin/
  v06_work_inventory.rs scripts/completion_atlas.py`, no `sd[0-9]+_`/`SD[0-9]+_`/
  `t_[0-9a-f]{8,}` hits — this cycle's own diff checked in isolation).
- **Wired-integration audit result:** `OK_NO_TOKENS` for this cycle's own diff alone (0 hits for
  `placeholder`/`STUB`/`MOCK`/`not yet implemented`/`fixme`/`hack`). Re-checked against the full
  bundle-scoped diff (`git diff --unified=0 ea2b3396f2...HEAD -- src/bin/v06_work_inventory.rs
  scripts/completion_atlas.py`, `ea2b3396f2` = SD-33's own merge commit, this branch's fork
  point): 31 `placeholder` hits, all pre-existing from waves 32–40 (none inside this cycle's own
  two insertion points — the synonym-table entry near the end of `CLASS_FEATURE_ID_KNOWN_
  SYNONYMS`, and the two new `canonical_seeds_for()` match arms plus the new
  `wave_41_canonical_seeds_tests` module — confirmed by re-running the isolated-diff grep above,
  which returns 0), zero introduced by this cycle.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** implement all three fixes
  named in `decisions.md §22`'s CORRECTION (2026-09-04) — Monk's Stunning Fist (a
  `CLASS_FEATURE_ID_KNOWN_SYNONYMS` table entry, the classifier's newer synonym-table matcher has
  no id-substring requirement the record's real `group: "standalone"` id fails), Fighter's Weapon
  Training and Psychic's Phrenic Pool (both a `canonical_seeds_for()` match arm, the same
  "give the sweep one canonical default choice" pattern already used for wizard/arcanist/
  sorcerer/cleric/druid). Add or extend unit tests. Run the guarded regen to completion and
  re-derive the real before/after bucket counts, naming exactly which of the 3 target units moved.

## Pre-state (re-derived fresh, not trusted from the dispatch brief)

`python3 scripts/completion_atlas.py --check` at this cycle's own pre-edit HEAD (`2d830bb5fc`,
wave 40's own wave-end gate commit): `population=49438 unclassified=0 overlap=0
citation_failures=0`, `DONE: 25358`, `D: 2523`, `V: 321` (every other bucket unchanged by this
cycle, listed in the Movement table below). Each of the 3 target units confirmed, by direct
`python3` filter over `docs/work-inventory.json`'s `units`, to be `engine-does-not-hold` with the
identical evidence string `class_feature_no_dedicated_magnitude_id_matched_the_record_slug`:

| Unit id | corpus_key | status (pre-cycle) |
|---|---|---|
| `core_rulebook:class_feature:monk_stunning_fist` | Monk ~ Stunning Fist | engine-does-not-hold |
| `core_rulebook:class_feature:fighter_weapon_training` | Fighter ~ Weapon Training | engine-does-not-hold |
| `occult_adventures:class_feature:psychic_phrenic_pool` | Psychic ~ Phrenic Pool | engine-does-not-hold |

## Per-unit fix, read directly from source (not assumed from `decisions.md §22`'s own claim)

### 1. Monk's Stunning Fist — a `CLASS_FEATURE_ID_KNOWN_SYNONYMS` table entry

`grep -n "stunning_fist" src/rules_core/feat_effects.rs src/rules_core/pilot_compute/mod.rs`
confirms the real compute is fully wired: `feat_effects::stunning_fist_facts_from_feats`
(`feat_effects.rs:1274`) computes the save DC and daily-uses count off the character's own
effective feat set (a Monk is granted the feat at 1st level and never selects it — the function
reads it from the resolved feat list regardless), and `pilot_compute/mod.rs:51393-51416` pushes
two real explanations, `feat.standalone.stunning_fist.save_dc` and
`feat.standalone.stunning_fist.uses_per_day` — both already exercised by existing pre-cycle tests
(`pilot_compute/mod.rs:75187`, `a_monk_grounds_stunning_fist_without_ever_selecting_the_feat`).

The corpus record itself (`data/corpus/core_rulebook/class_feature/monk/stunning_fist.json`) has
`key: "Monk ~ Stunning Fist"` — `class_feature_owner` resolves `owner = "monk"`, `group = "Monk"`
(the part before `" ~ "`), `feature_slug = "stunning_fist"`. The OLDER matcher
(`class_feature_exact_suffix_grounded`, requires the id's own dot-segment structure to name the
feature, with a `.{owner}.` substring needle) correctly refuses: `feat.standalone.stunning_fist.
save_dc` carries `standalone` where `owner` would need to appear, never `monk` — this was wave
40 lane A's own (correct, as far as it went) finding. The NEWER matcher
(`class_feature_known_synonym_grounded`, live since wave 39) does a **pure literal
`(owner, feature_slug)` → exact-id lookup**, gated only by `group.eq_ignore_ascii_case(&
class_name_as_group_text(owner))` — a check on this record's own corpus `group` ("Monk"), never
on the id string itself. `class_name_as_group_text("monk") == "monk"`, matching `"Monk"`
case-insensitively — the guard already passes. **Fix:** one literal entry,
`("monk", "stunning_fist", "feat.standalone.stunning_fist.save_dc")`, added to
`CLASS_FEATURE_ID_KNOWN_SYNONYMS`. No classifier code change, no new mechanism — exactly
`decisions.md §22`'s own correction.

### 2. Fighter's Weapon Training — a `canonical_seeds_for()` match arm

`grep -n "class_feature.fighter.weapon_training\|fighter_weapon_training_attack_bonus"
src/rules_core/pilot_compute/mod.rs` confirms `pilot_compute/mod.rs:13078`
(`fighter_weapon_training_attack_bonus`) computes a real, level-gated rank bonus, and
`pilot_compute/mod.rs:32499-32544` pushes it as a real, single, exact-match explanation id,
`class_feature.fighter.weapon_training` (tier 1 only — tiers 2-4 carry their own sibling ids,
`_group_2`/`_group_3`/`_group_4`, untouched by this cycle). The corpus record
(`data/corpus/core_rulebook/class_feature/fighter/weapon_training.json`) has `key: "Fighter ~
Weapon Training"` — a SINGLE record (not per-tier/per-group), `owner = "fighter"`,
`feature_slug = "weapon_training"`, which **already equals** the explanation id's own trailing
dot segment. This is the plain `class_feature_exact_suffix_grounded` shape, needing no synonym
table entry at all — the sole reason it never grounded is that `canonical_seeds_for("fighter")`
seeded no `choice:fighter_weapon_training_group` selection, so the id never appeared in
`EngineFacts::explanation_ids` in the first place. **Fix:** added a `"fighter" =>` match arm to
`canonical_seeds_for()` seeding `choice:fighter_weapon_training_group -> group:heavy_blades` (one
of the 14 canonical PF1 weapon-training groups the engine's own `WEAPON_TRAINING_GROUPS` const
already enumerates, not a guess), the identical "give the sweep one canonical default choice"
pattern the function already uses for wizard/arcanist/sorcerer/cleric/druid/monk/witch/
shaman/alchemist/investigator/warpriest/bloodrager/summoner/cavalier/inquisitor/oracle.

**Not conflated with the pre-existing, SEPARATE `AT-34-E3-001` mechanism-3 probe**
(`probe_fighter_weapon_training_wiring`/`fighter_weapon_training_wired`): that probe answers a
DIFFERENT question, for a DIFFERENT corpus record shape — the 52 individually-keyed
`"Weapon Training <tier> <group>"` records (e.g. `"Weapon Training 1 Blades Heavy"`), which carry
no `" ~ "` separator and can never resolve an `owner` through `class_feature_owner` at all. The
single `"Fighter ~ Weapon Training"` record this cycle closes is a different, already-owner-
resolvable record; the probe's own retain-before-select logic
(`probe_fighter_weapon_training_wiring`'s own `input.chosen.selected_choices.retain(|c| ...)`)
already strips and replaces ANY pre-seeded `choice:fighter_weapon_training_group*` selection
before running its own controlled matrix, so this cycle's new seed cannot interfere with it —
confirmed by re-reading the probe's own body before editing anything nearby. Corrected two stale
doc comments (the `fighter_weapon_training_wired` field, and `probe_fighter_weapon_training_
wiring`'s own doc comment) that both asserted "`canonical_seeds_for("fighter")` never seeds ANY
weapon-training-group choice at all" — now narrowed to state accurately that tier 1 alone is now
seeded, tiers 2-4 and the other 13 non-canonical tier-1 groups still depend on the probe alone.

### 3. Psychic's Phrenic Pool — same shape, a second `canonical_seeds_for()` match arm

`grep -n "phrenic_pool\|PSYCHIC_DISCIPLINE_CHOICE_ID" src/rules_core/pilot_compute/mod.rs`
confirms `ground_psychic_class_features` (`pilot_compute/mod.rs:30002-30028`) already computes a
real, tested magnitude via `psychic_discipline_pool_ability`, which resolves the character's own
chosen Psychic Discipline (`PSYCHIC_DISCIPLINE_CHOICE_ID = "choice:psychic_discipline"`) to the
correct ability modifier (Charisma for `PSYCHIC_DISCIPLINE_CHA_SELECTION_IDS`, Wisdom for
`PSYCHIC_DISCIPLINE_WIS_SELECTION_IDS`) and pushes `class_feature.untabled.psychic.
phrenic_pool.value` — already unit-tested pre-cycle (`pilot_compute/mod.rs:54326`,
`assert_eq!(value("class_feature.untabled.psychic.phrenic_pool.value"), 13)`). The corpus record
(`data/corpus/occult_adventures/class_feature/psychic/phrenic_pool.json`) has `key: "Psychic ~
Phrenic Pool"` — `owner = "psychic"`, `feature_slug = "phrenic_pool"`. The id's own SECOND-TO-LAST
dot segment (`phrenic_pool`, the mechanism-3 `class_feature.untabled.<owner>.<feature_slug>.
<descriptor>` shape wave 38 lane C's own fix already recognizes for exactly this "untabled"
dispatch-chain family, which Psychic belongs to per that check's own doc comment) equals
`feature_slug` exactly — no synonym table entry needed here either. The sole reason it never
grounded is `psychic_discipline_pool_ability` returning `None` for every swept Psychic (no
`choice:psychic_discipline` selection ever supplied). **Fix:** added a `"psychic" =>` match arm
seeding `choice:psychic_discipline -> discipline:rapport` (one of the engine's own recognized
Charisma-keyed disciplines, `PSYCHIC_DISCIPLINE_CHA_SELECTION_IDS`), same pattern as Fighter's.

`src/rules_core/pilot_compute/mod.rs` carries **zero diff** (`git diff --stat -- src/rules_core/
pilot_compute/mod.rs` empty) — every id all three fixes now recognize was already shipped and
tested; this cycle only made them reachable to the classifier's own sweep.

## Tests (`v06_work_inventory.rs`)

4 new tests added, 1 existing test edited:

- `wave_41_canonical_seeds_tests::canonical_seeds_for_fighter_makes_weapon_training_fire_through_
  the_real_sweep` — calls `class_sweep_input(&fixture(), "fighter", 5)` (the SAME entry point the
  corpus-wide union sweep uses, `main`'s own `explanation_ids` loop) with NO bespoke override, and
  asserts `class_feature.fighter.weapon_training` fires with value `1` (level 5, tier-1 rank 1).
- `wave_41_canonical_seeds_tests::canonical_seeds_for_psychic_makes_phrenic_pool_fire_through_the_
  real_sweep` — same shape for `"psychic"`, asserts `class_feature.untabled.psychic.
  phrenic_pool.value` fires non-zero.
- `class_feature_known_synonym_grounded_tests::monk_stunning_fist_grounds_via_the_synonym_table` —
  direct unit test of `class_feature_known_synonym_grounded(["feat.standalone.stunning_fist.
  save_dc"], "monk", "Monk", "stunning_fist")` returning `true`.
- `class_feature_known_synonym_grounded_tests::fighter_and_psychic_are_fixed_via_canonical_seeds_
  not_the_synonym_table` — negative control proving neither `("fighter", "weapon_training")` nor
  `("psychic", "phrenic_pool")` was also (redundantly, or by mistake) added to the table.
- `class_feature_known_synonym_grounded_tests::declined_units_are_not_in_the_table` (edited) — was
  wave 40 lane A's own list of 4 declined `(owner, slug)` pairs (Druid/Monk/Fighter/Psychic);
  narrowed to Druid's Nature Bond alone (still correctly declined — a permanent `+0`-by-design
  recognition record, unaffected by this cycle) since the other 3 are no longer declined.

**Honest note on RED→GREEN discipline this cycle:** the code fix (table entry + 2 match arms) and
its own tests were written together in one pass, not RED-verified by a separate temporary-removal
step the way wave 40 lane B's own Summoner entries were — a real gap against this bundle's usual
discipline, disclosed rather than glossed over. What WAS verified directly: `class_feature_known_
synonym_grounded`'s own pre-existing test module already proves the general lookup mechanism
correct (literal, full-string, `group == owner`-gated, no substring widening) via 10 pre-existing
tests untouched by this cycle; the new `monk_stunning_fist_grounds_via_the_synonym_table` test
exercises that SAME already-proven mechanism against the new entry, and the two
`wave_41_canonical_seeds_tests` run the real `compute_pilot_base_chassis` pipeline end-to-end
(not a mock), so a wrong choice-set id or selection literal in either new match arm fails these
tests outright — confirmed directly: intentionally typo-ed `"choice:fighter_weapon_
training_group"` to `"choice:fighter_weapon_training_groups"` and re-ran
`cargo test --locked --bin v06_work_inventory -j 6 wave_41`: the Fighter test FAILED with the
expected panic (`class_feature.fighter.weapon_training must fire once canonical_seeds_for
supplies a weapon-training group choice`), the Psychic test unaffected (1 passed, 1 failed);
reverted the typo and re-ran, both GREEN again (2/2 pass) — the one negative check this cycle
performed, isolating the pipeline test's own sensitivity to a wrong id rather than declaring it
correct by construction alone.

**GREEN, all scopes:**
- `cargo test --locked --bin v06_work_inventory -j 6 wave_41` → 2/2 pass.
- `cargo test --locked --bin v06_work_inventory -j 6 known_synonym` → 13/13 pass (2 new; the
  4th new test, the fighter/psychic pipeline pair, lives in a different module and is counted
  above).
- `cargo test --locked --bin v06_work_inventory -j 6` (full) → **548/548 pass, 0 regressed**, run
  twice — once pre-regen, once again post-regen against the freshly-regenerated
  `docs/work-inventory.json` (some tests in this file read the committed inventory directly) —
  identical result both times.
- `cargo test --locked --lib -j 6` (full workspace lib) → **3063 passed, 0 failed, 14 ignored** —
  matches the standing `BASELINE_ROOT_LIB_TESTS=3063` exactly (no `src/rules_core` file touched
  this cycle, confirmed by `git diff --stat -- src/rules_core` being empty).
- `cargo test --locked --no-run` (full workspace) → **exit 0**, all test binaries (including
  every `tests/v06_*` integration target) compiled clean.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` is
  empty, no file under `apps/desktop/` touched this cycle.

## Guarded regen — ran to completion, prerequisites generated fresh

First attempt (`cargo run --locked --release --bin v06_work_inventory`) correctly **refused**:
"this run would drop 9623 of the 9623 verification stamp(s) it currently carries" —
`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` were unset, the same guard wave 40
lane A's own cycle hit. Generated both prerequisite reports fresh against this cycle's own tree
before re-running:

- `cargo run --locked --release --bin corpus_literal_sweep -- --json-out <path>` →
  `corpus-literal-sweep: 48706 records examined of 51476 read, 413314 tokens compared (9
  synthesized), 51463 digests checked, 0 findings` / `3138 tokens exempted under decisions.md §24
  redaction across 1058 codex_generated_name records` / **CLEAN**.
- `cargo run --locked --release --bin derived_evaluator_fixture_check -- --json-out <path>` →
  `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested`.

Re-ran `cargo run --locked --release --bin v06_work_inventory` with both `CORPUS_LITERAL_SWEEP_
REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` set to those two reports — completed cleanly, no
refusal, `docs/work-inventory.json` regenerated with `"generated_at": "2026-09-04T14:19:21Z"`.
`git status --porcelain -- docs/work-inventory.json` shows `M` (modified, not stale). A
`--release` build was used throughout (compile once, ~2 min; each run itself a few minutes) —
faster than wave 40 lane A's own debug-build attempt that ran past its cycle's time budget.

## Movement — the real, regen-verified delta

Before/after bucket counts: "before" is `completion_atlas.py --check` run directly against
`docs/work-inventory.json` at this cycle's own pre-edit HEAD (`2d830bb5fc`, captured before any
edit this cycle made); "after" is the same command run against the same path once the guarded
regen completed. Independently cross-checked via a direct Python join comparing every `id`'s own
`status` field between `git show 2d830bb5fc:docs/work-inventory.json` and the post-regen file
(49438 units each side) — both methods agree exactly:

| Bucket | Before (`2d830bb5fc`) | After (this cycle) | Δ |
|---|---:|---:|---:|
| DONE | 25358 | 25360 | **+2** |
| D | 2523 | 2520 | **−3** |
| V | 321 | 322 | **+1** |
| A / B / C / M / U / X / Z | unchanged | unchanged | 0 |

**Exactly 3 units changed status, zero collateral movement** — a full `id`→`status` join between
the pre- and post-regen inventories (49438 units each) finds precisely these 3 differences and no
others:

| Unit | corpus_key | status before | status after | bucket |
|---|---|---|---|---|
| `core_rulebook:class_feature:fighter_weapon_training` | Fighter ~ Weapon Training | engine-does-not-hold | **grounded** | DONE |
| `occult_adventures:class_feature:psychic_phrenic_pool` | Psychic ~ Phrenic Pool | engine-does-not-hold | **grounded** | DONE |
| `core_rulebook:class_feature:monk_stunning_fist` | Monk ~ Stunning Fist | engine-does-not-hold | **literal-verified** | V |

Fighter's Weapon Training and Psychic's Phrenic Pool both carry `evidence:
"explanation_id_observed_in_a_real_computation"` (the plain `class_feature_exact_suffix_grounded`
rung — no synonym table involved, matching the "no synonym table even needed" finding above).
Monk's Stunning Fist carries `evidence:
"explanation_id_observed_via_known_class_feature_synonym"` and lands in **V** (`literal-verified`)
rather than DONE directly — the same evidence-classification split wave 40's own wave-end gate
entry already documented for 2 of its 7 units (a synonym-table match resolves to a different
done-rung evidence path than a plain exact-suffix match; both are real closures, just filed under
different bucket letters per this bundle's own `status_vocabulary`).

**`population=49438 unclassified=0 overlap=0 done_evidence_violations=0
missing_clearing_mechanisms=0 citation_failures=0`** — `completion_atlas.py --check`, this
cycle's own post-regen, post-citation-fix HEAD.

## Figures (every number, its command, its denominator)

- `3` of `3` target units closed (named individually above) — this receipt's own before/after
  join, `docs/work-inventory.json`'s `units` array, 49438 total population both snapshots.
- `1` synonym-table entry added, `2` `canonical_seeds_for()` match arms added — this receipt's
  own code sections above.
- `2`/`2` `wave_41` tests pass, `13`/`13` `known_synonym` tests pass (2 new in that module) —
  `cargo test --locked --bin v06_work_inventory -j 6 <filter>`.
- `548`/`548` `v06_work_inventory` bin tests pass (4 new, 0 regressed) — run twice, pre- and
  post-regen, identical result both times.
- `3063` `cargo test --locked --lib -j 6` pass, `0` failed, `14` ignored — matches
  `BASELINE_ROOT_LIB_TESTS=3063` exactly, no `src/rules_core` diff this cycle.
- `0` diff in `src/rules_core/pilot_compute/mod.rs` — `git diff --stat -- src/rules_core/
  pilot_compute/mod.rs`.
- `DONE: 25358 -> 25360 (+2)`, `D: 2523 -> 2520 (-3)`, `V: 321 -> 322 (+1)` — `completion_atlas.py
  --check` against the pre-cycle (`2d830bb5fc`) and post-cycle `docs/work-inventory.json`, both
  explicit denominators stated above.
- `10` citation pins re-derived, all shifted uniformly by `+128` lines — `scripts/
  completion_atlas.py`'s own diff, verified individually by content (not just line-number
  arithmetic) before committing.
- `0` `data/corpus/**` files touched — `git diff --stat -- data/corpus/`.

## Build scope verified

- `cargo build --locked --bin v06_work_inventory` → exit 0.
- `cargo test --locked --bin v06_work_inventory -j 6` → 548/548 pass (4 new), run twice
  (pre-regen and post-regen).
- `cargo test --locked --lib -j 6` (full workspace) → 3063 passed, 0 failed, 14 ignored.
- `cargo test --locked --no-run` (full workspace) → **exit 0**.
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat -- apps/desktop/` empty,
  no file under `apps/desktop/` touched.

## Sweep population

`corpus_literal_sweep --json-out` (guarded regen chain step 1): `48706 records examined of 51476
read, 413314 tokens compared (9 synthesized), 51463 digests checked, 0 findings CLEAN`. No
`data/corpus/**` record was added, changed, or removed this cycle (`git diff --stat -- data/
corpus/` empty), so the examined-population matches the corpus's own current size, consistent
with `decisions.md §12` L8's rule (a growth is only required when records are ADDED).

`derived_evaluator_fixture_check --json-out`: `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — clean, required guarded-regen prerequisite.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — no
figure in this receipt was derived from the pinned oracle corpus directly; every magnitude
credited this cycle was already transcribed and unit-tested (both at the `pilot_compute` layer
pre-cycle and now at the classifier layer) against `data/corpus/**` by pre-existing compute
functions this cycle only made VISIBLE to the classifier, not computed anew. Cited for
completeness per the receipt schema.

## Status

**complete.** All 3 target units closed and regen-confirmed, with a clean, fully-attributed
before/after delta and zero collateral movement. `decisions.md §22`'s CORRECTION is now fully
applied: none of the three units needed the "genuinely different, structurally larger" fix its
own superseded framing (wave 39 lane B / wave 40 lane A) had claimed — all three were cheap,
already-precedented classifier-visibility fixes (one table entry, two match arms), reusing
mechanisms proven since waves 38-40 rather than inventing anything new.

## Movement, four buckets

- **Closure:** 3 (Fighter ~ Weapon Training, Psychic ~ Phrenic Pool — both `engine-does-not-hold`
  → `grounded`/DONE; Monk ~ Stunning Fist — `engine-does-not-hold` → `literal-verified`/V).
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 10 `completion_atlas.py` citation pins re-derived (uniform +128
  shift, each verified by content) + 3 retro-logged corrections (`docs/retro/events/
  sd34-wave41.jsonl`) against wave 39 lane B's and wave 40 lane A's own superseded "structurally
  different fix needed" framing for these exact three units, per `decisions.md §22`'s own
  CORRECTION.

## Notes (judgment calls)

- **Why Monk's Stunning Fist lands in V, not DONE directly:** its evidence is
  `explanation_id_observed_via_known_class_feature_synonym` (the synonym-table rung), which this
  bundle's own `status_vocabulary` files under `literal-verified` rather than `grounded` — the
  identical split wave 40's own wave-end gate entry found for 2 of its 7 synonym-table closures.
  Both `grounded` and `literal-verified` are real, regen-confirmed closures; only the bucket
  LETTER differs by which rung supplied the evidence.
- **Why `group:heavy_blades`/`discipline:rapport` were chosen over other valid options:** both are
  simply the first canonical option in their own respective closed enumerations
  (`WEAPON_TRAINING_GROUPS`, `PSYCHIC_DISCIPLINE_CHA_SELECTION_IDS`) — any of the 14 weapon-
  training groups or 5+5 Psychic Disciplines would have produced an equally real, equally
  non-fabricated magnitude for their respective explanation ids; no group/discipline-specific
  reasoning favored one choice over another for THIS cycle's closure (unlike, say, `canonical_
  seeds_for("cleric")`'s choice of Good's own domain, which was already fixed before this cycle
  and left untouched).
- **Why the pre-existing `AT-34-E3-001` Fighter weapon-training PROBE was not touched or
  conflated:** it answers a genuinely different question (the 52 individually-keyed
  `"Weapon Training <tier> <group>"` records, which can never resolve an `owner` through
  `class_feature_owner` at all) from the single `"Fighter ~ Weapon Training"` record this cycle
  closes — read both code paths directly before touching either, rather than assuming the probe's
  own doc comment (which asserted a now-partially-stale claim about `canonical_seeds_for
  ("fighter")`) still described post-cycle behavior; corrected the 2 stale doc comments that
  claimed the gap this cycle narrows was total rather than partial.

## Next-cycle plan

1. **Shape 2's remaining scope after this cycle:** the 15 confirmed genuinely-different
   (new-chassis) units from wave 39 lane B's own table (Duelist 4, Shadowdancer 4, Assassin 2,
   Loremaster 2, Cleric's Aura 1, Paladin's Detect Evil 1, Wizard's Arcane Bond 1) — real Epic
   4/5-shaped chassis-building work per `decisions.md §22`'s own standing scope ruling. Per that
   same section's CORRECTION, re-verify each against the real `pilot_compute/mod.rs` before
   trusting the "no compute exists" framing at face value — this cycle is the second time in this
   bundle a "structurally harder" claim for this exact area turned out to be a cheap classifier-
   visibility fix once someone actually read the code (Wizard's Arcane Bond was the one unit in
   the 15-unit table that WAS spot-checked in full and held up as genuinely unbuilt — the other 14
   were only partially or not spot-checked at all).
2. **Sub-mechanism 5** (634 units across 60 classes, per wave 37 lane B's own corrected figure)
   remains un-re-audited since `decisions.md §22`'s own correction — its "genuinely too large"
   framing predates that correction and should be treated as unverified difficulty, not confirmed,
   until someone re-checks it the same way this cycle re-checked the three named units.
3. **`THE-BOX.md`/`box_ledger.py --check` staleness** (an SD-33 instrument, not part of SD-34's
   own per-cycle gate) was not checked this cycle — out of this cycle's own narrow scope (3 named
   units); flagged for whoever owns that debt, consistent with every prior wave's own note.
