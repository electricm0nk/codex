# Cycle — SD-34 wave 40, Lane B — Shape 2's Summoner remainder: 5 of 6 units closed via a same-shape synonym-table extension, Greater Aspect declined (no compute function exists)

- **Commit SHA:** `725b5ff1c9` (`725b5ff1c90d1512accd817a758d5ecf05ed1290`)
- **Files touched:** `src/bin/v06_work_inventory.rs` (`CLASS_FEATURE_ID_KNOWN_SYNONYMS` extended
  with 5 new `(owner, feature_slug, exact_full_explanation_id)` entries for `owner: "summoner"`,
  doc comment extended with this cycle's own live-dump verification notes, 3 new tests added to
  `class_feature_known_synonym_grounded_tests`), `scripts/completion_atlas.py` (10 citation-pin
  line numbers re-derived — this cycle's own +112-line insertion, in two separate hunks, shifted
  every pin below the first hunk by +43; the second hunk sits below every citation and shifts
  none of them further), `docs/work-inventory.json` (regenerated — this run also picks up wave
  40 lane A's own 7-unit synonym-table fix for the first time under a `--check` gate, see
  "Population re-derived fresh" below for why that does not conflate with this cycle's own
  5-unit movement), `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json` (a `--check` re-run's own artifact, byte-consistent with the re-derived
  citations), this receipt, `progress.md`, `kanban.md`, `docs/retro/events/sd34-wave40-laneb.jsonl`
  (new, 1 deferral entry). **No `data/corpus/**` file touched.**
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/bin/
  v06_work_inventory.rs`, no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits — this cycle's own
  diff checked in isolation).
- **Wired-integration audit result:** `OK_NO_TOKENS` for this cycle's own diff alone (`git diff
  --unified=0 HEAD -- src/bin/v06_work_inventory.rs`, 0 hits). Re-checked against the full
  bundle-scoped diff (`git diff --unified=0 $(git merge-base HEAD origin/develop)...HEAD --
  src/bin/v06_work_inventory.rs`): 16 `placeholder` hits, all pre-existing from waves 32–39 (spot
  checked by line number against this cycle's own two insertion points, ~10245–10330 and
  ~24230–24320 in the post-edit file — none of the 16 hits falls inside either range), no
  `STUB`/`MOCK`/`not yet implemented`/`fixme`/`hack`, zero introduced by this cycle.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "First-check and, if
  safely fixable, close `Summoner` (6 units, non-Unchained — do NOT confuse with `Unchained
  Summoner`, already closed by wave 39 lane A) from Shape 2's own remaining population... For
  each unit, grep `pilot_compute/mod.rs`... determine which shape applies: (a) a clean
  word-choice synonym... confirm the candidate id by direct `grep -c == 1`... AND by a temporary
  explanation-id dump test... (b) a compound-suffix/wrong-namespace/no-discrete-id/value-0
  variant... do not attempt a fix this cycle if it does not cleanly and safely fit the synonym-
  table shape. (c) no compute function exists at all... Fix only what is safely (a)-shaped."

## Population re-derived fresh (not trusted from the dispatch brief's own count)

`python3` filter over `docs/work-inventory.json`'s `units` at this cycle's own pre-edit HEAD
(`d7184384d8`, wave 40 lane A's own commit): `evidence ==
"class_feature_no_dedicated_magnitude_id_matched_the_record_slug"`, `corpus_key` starting with
`"Summoner ~"` (not `"Unchained Summoner ~"`) — **exactly 6**, matching the brief:

| Unit id | corpus_key | magnitude_token_count | status (pre-cycle) |
|---|---|---:|---|
| `advanced_players_guide:class_feature:summoner_bond_senses` | Summoner ~ Bond Senses | 2 | engine-does-not-hold |
| `advanced_players_guide:class_feature:summoner_greater_aspect` | Summoner ~ Greater Aspect | 2 | engine-does-not-hold |
| `advanced_players_guide:class_feature:summoner_maker_s_call` | Summoner ~ Maker's Call | 2 | engine-does-not-hold |
| `advanced_players_guide:class_feature:summoner_merge_forms` | Summoner ~ Merge Forms | 2 | engine-does-not-hold |
| `advanced_players_guide:class_feature:summoner_summon_monster` | Summoner ~ Summon Monster | 2 | engine-does-not-hold |
| `advanced_players_guide:class_feature:summoner_twin_eidolon` | Summoner ~ Twin Eidolon | 2 | engine-does-not-hold |

**Note on lane A's own 7 units, checked before touching anything so this cycle's own movement
figure is not conflated with theirs:** wave 40 lane A's own receipt reports "blocked-escalated,
0 units confirmed closed" (their guarded regen did not finish in their own cycle's time budget).
Reading the actual commit (`d7184384d8`) shows its own message states the orchestrator
completed the regen and committed it in the same commit — confirmed directly: `git show
d7184384d8:docs/work-inventory.json` (this cycle's own pre-edit baseline) already carries all 7
of lane A's units as `grounded`/`literal-verified`, unchanged by anything in this cycle. This
cycle's own before/after comparison (below) is therefore already differenced against a baseline
that includes lane A's 7 — the 5-unit movement this receipt reports is this cycle's own, and
only this cycle's own.

## Per-unit check: real compute function, dispatch-wired, real magnitude — read directly, not assumed

`grep -n "summoner" src/rules_core/pilot_compute/mod.rs -i` (excluding `unchained`/`ApgClassId`
plumbing) surfaces `ground_summoner_slice_a_features` (`pilot_compute/mod.rs:18281`), dispatched
for real at `pilot_compute/mod.rs:14176-14178` (`ApgClassId::Summoner =>
ground_summoner_eidolon(...); ground_summoner_slice_a_features(...);`), gated by
`is_supported_summoner_single_class` — confirmed wired, not merely present.

Its own doc comment (`pilot_compute/mod.rs:18265-18280`) names the shape precisely: "Grounds
Summoner's five flat, self-scoped class-feature pools (Slice A)... Deliberately does NOT touch
Aspect/Greater Aspect: those divert points out of the eidolon's own evolution pool, making them
a chooser over a shared resource rather than an independent fact." Reading the function body
directly (not the doc comment alone) confirms exactly 5 of the 6 assigned units have a real,
level-gated, non-zero compute path; the 6th (Greater Aspect) has none:

| Corpus feature (`feature_slug`) | Real engine explanation id (verbatim) | Level gate | Shape |
|---|---|---:|---|
| Bond Senses (`bond_senses`) | `class_feature.apg.summoner.bond_senses_rounds_per_day` | 2 | (a), compound-suffix |
| Maker's Call (`makers_call`) | `class_feature.apg.summoner.makers_call_uses_per_day` | 6 | (a), compound-suffix |
| Merge Forms (`merge_forms`) | `class_feature.apg.summoner.merge_forms_rounds_per_day` | 16 | (a), compound-suffix |
| Twin Eidolon (`twin_eidolon`) | `class_feature.apg.summoner.twin_eidolon_minutes_per_day` | 20 | (a), compound-suffix |
| Summon Monster (`summon_monster`) | `class_feature.apg.summoner.summon_monster_uses_per_day` (also `_duration_minutes`, `_spell_level` — 3 real siblings, any one suffices per lane A's own precedent) | 1 | (a), compound-suffix |
| Greater Aspect (`greater_aspect`) | *(none)* | — | (c), no compute function |

Every one of the 5 real ids is a **4-segment id** (`class_feature.apg.<owner>.<descriptor>`) —
`class_feature_engine_join_slug` gives `feature_slug` values of `bond_senses`/`makers_call`/
`merge_forms`/`twin_eidolon`/`summon_monster` (apostrophes swallowed per that function's own
convention: `"Maker's Call"` → `"makers_call"`), none of which equals, nor is a single-suffix-
word strip of (`CLASS_FEATURE_ID_MAGNITUDE_SUFFIXES` only strips ONE trailing word), the id's own
trailing compound descriptor (`_rounds_per_day`, `_uses_per_day`, etc.) — the identical
compound-suffix shape wave 40 lane A's own 4 Monk chassis entries already closed, just with an
extra `apg` namespace segment inserted before `owner` (`class_feature_exact_suffix_grounded`'s
own `.{owner}.` substring needle still matches, since it is not anchored to the id's start — a
4-segment id is not the 3-segment `<owner>.<feature_slug>.<descriptor>` convention either check
recognizes). Neither `class_feature_exact_suffix_grounded` (trailing-segment / second-to-last-
segment checks) nor `id_matches_feature_slug_after_known_magnitude_suffix_strip` (single-word
strip only) can bridge this — confirmed by reading both functions directly, not assumed from the
shape alone.

**Greater Aspect is genuinely case (c).** `grep -n "aspect" src/rules_core/pilot_compute/mod.rs
-i | grep -i summoner` finds only `unchained_summoner.aspect_evolution_points_divertible` and
`unchained_summoner.greater_aspect_evolution_points_divertible` (`pilot_compute/mod.rs:31340`,
`:31359`) — **Unchained Summoner's** own Aspect/Greater Aspect, a different class's different
feature (already closed by wave 39 lane A's own `unchained_summoner` table entries, unrelated to
this unit). Base Summoner's own Greater Aspect has no compute function anywhere in
`pilot_compute/mod.rs`, confirmed by the same grep and by `ground_summoner_slice_a_features`'s
own doc comment naming the exclusion explicitly. Not a synonym-table gap — a genuinely unbuilt
mechanism (the eidolon-evolution-pool diversion this function's own doc comment says makes
Aspect/Greater Aspect "a chooser over a shared resource rather than an independent fact," not a
flat computed quantity). Declined, not attempted, per the brief's own instruction for shape (c).

## Two-factor confirmation for the 5 safe ids (same discipline wave 38 lane C / wave 39 lane A / wave 40 lane A used)

**Factor 1 — `grep -c` for a single real definition site.** Each of the 7 candidate ids
(5 assigned + `summon_monster`'s 2 non-chosen siblings, checked for completeness) has exactly
ONE occurrence inside a `ComputationExplanation { id: "..." }` construction; the raw `grep -c`
count is 2–3 per id because `pilot_compute/mod.rs`'s own test module
(`Slice A: the five flat, self-scoped Summoner class features`, lines ~74718–74801) already
carries pre-existing, already-passing unit tests referencing the same literal string —
individually inspected, confirmed test-only references, never a second definition site:

```
class_feature.apg.summoner.bond_senses_rounds_per_day       : 2 (1 def + 1 test)
class_feature.apg.summoner.makers_call_uses_per_day         : 2 (1 def + 1 test)
class_feature.apg.summoner.merge_forms_rounds_per_day       : 2 (1 def + 1 test)
class_feature.apg.summoner.twin_eidolon_minutes_per_day     : 2 (1 def + 1 test)
class_feature.apg.summoner.summon_monster_uses_per_day      : 2 (1 def + 1 test)
class_feature.apg.summoner.summon_monster_duration_minutes  : 2 (1 def + 1 test)
class_feature.apg.summoner.summon_monster_spell_level       : 3 (1 def + 2 test)
```

**Also found: these 5 features are already unit-tested at the `pilot_compute` layer**, not just
present — `pilot_compute/mod.rs`'s own `bond_senses_grounds_rounds_per_day_from_its_gate_onward`
/ `makers_call_uses_per_day_match_the_corpus_formula` / `merge_forms_grounds_rounds_per_day_
from_level_sixteen` / `twin_eidolon_grounds_minutes_per_day_only_at_level_twenty` /
`summon_monster_grounds_duration_uses_and_spell_level` tests already assert real non-zero values
at specific levels through a dedicated `summoner(level)` fixture helper — independent, pre-
existing, stronger evidence than a grep alone, read directly rather than trusted from a name.

**Factor 2 — a live dump through the classifier's own real sweep pipeline** (`class_sweep_input`
+ `compute_pilot_base_chassis`, the same pipeline `EngineFacts::explanation_ids` unions over
`SWEEP_LEVELS = [1, 5, 10, 15, 20]` for every modelled class, `v06_work_inventory.rs:9358-9378`).
A temporary test module (`wave40_laneb_temp_summoner_candidate_dump`, added, run, then REMOVED
before this cycle's own commit — never landed, confirmed via `git diff --stat` showing 0 net
lines from this step) printed every candidate id's real value at every swept level:

```
SUMMONER_DUMP class_feature.apg.summoner.bond_senses_rounds_per_day       => [(1,None),(5,Some(5)),(10,Some(10)),(15,Some(15)),(20,Some(20))]
SUMMONER_DUMP class_feature.apg.summoner.makers_call_uses_per_day         => [(1,None),(5,None),(10,Some(2)),(15,Some(3)),(20,Some(4))]
SUMMONER_DUMP class_feature.apg.summoner.merge_forms_rounds_per_day       => [(1,None),(5,None),(10,None),(15,None),(20,Some(20))]
SUMMONER_DUMP class_feature.apg.summoner.twin_eidolon_minutes_per_day     => [(1,None),(5,None),(10,None),(15,None),(20,Some(20))]
SUMMONER_DUMP class_feature.apg.summoner.summon_monster_uses_per_day      => [(1,Some(2)),(5,Some(2)),(10,Some(2)),(15,Some(2)),(20,Some(2))]
SUMMONER_DUMP class_feature.apg.summoner.summon_monster_duration_minutes  => [(1,Some(1)),(5,Some(5)),(10,Some(10)),(15,Some(15)),(20,Some(20))]
SUMMONER_DUMP class_feature.apg.summoner.summon_monster_spell_level       => [(1,Some(1)),(5,Some(3)),(10,Some(5)),(15,Some(8)),(20,Some(9))]
```

Every one of the 5 chosen ids reaches a real, non-zero value at some `SWEEP_LEVELS` level under
`canonical_seeds_for("summoner")` (`choice:summoner_eidolon_evolution` ->
`evolution:improved_natural_armor`, the only seed Summoner needs — none of the 5 depend on any
choice this seed omits, unlike wave 40 lane A's Bard/Sorcerer false starts which needed an
activation or spell selection the sweep never provides). **No false start this cycle**: unlike
lane A's Bardic Performance/Sorcerer Spells, every source-obvious candidate here was ALSO the
safe one — the live dump corroborated rather than corrected the source read, the same "Monk
needed no re-aliasing" outcome lane A's own receipt found for its unconditionally-pushed chassis
ids. `summon_monster_uses_per_day` was picked over its two siblings (`_duration_minutes`,
`_spell_level`) as the table entry only because it is reachable one level earlier (gate 1, same
as the others) and its own value is least likely to be confused with an unrelated "duration" or
"spell level" concept in a future reader — any of the 3 would have worked identically per lane
A's own "any one sibling id proves the engine holds the record" precedent (Eidolon/Summon
Monster entries).

## `group == owner` guard, checked for Summoner specifically (no cross-owner leak with Unchained Summoner)

`class_name_as_group_text("summoner")` == `"summoner"`, matching the corpus group text
`"Summoner"` (case-insensitive) exactly — and NOT `"Unchained Summoner"` (a different, already-
registered `owner` in the same table with its own 3 entries). Added a dedicated negative-control
test (`base_summoner_does_not_ground_via_unchained_summoners_own_entries`) asserting that an
Unchained Summoner explanation id does not ground `(summoner, summon_monster)` — proves the two
owners' entries cannot cross-credit each other even though both ids share a `.summoner.`
substring.

## The 5 ids added, each confirmed two ways (grep -c == 1 real definition site, AND live-dump non-zero)

```rust
("summoner", "bond_senses",    "class_feature.apg.summoner.bond_senses_rounds_per_day"),
("summoner", "makers_call",    "class_feature.apg.summoner.makers_call_uses_per_day"),
("summoner", "merge_forms",    "class_feature.apg.summoner.merge_forms_rounds_per_day"),
("summoner", "twin_eidolon",   "class_feature.apg.summoner.twin_eidolon_minutes_per_day"),
("summoner", "summon_monster", "class_feature.apg.summoner.summon_monster_uses_per_day"),
```

`Summoner ~ Greater Aspect` is **deliberately NOT added** — see "Per-unit check" above: no
compute function exists for it anywhere, a genuinely unbuilt mechanism, not a synonym gap.

## Tests (`class_feature_known_synonym_grounded_tests`, `v06_work_inventory.rs`)

3 new tests added (the pre-existing `every_known_synonym_table_entry_grounds_via_its_own_
exact_id` test is parametrized over the live `CLASS_FEATURE_ID_KNOWN_SYNONYMS` const, so it
already covers all 5 new entries without modification, same as every prior wave's own pattern):

- `wave_40_lane_b_entries_match_the_receipts_own_manifest` — pins the exact 5
  `(owner, feature_slug, id)` triples as a literal manifest independent of the table itself.
- `summoner_greater_aspect_is_not_in_the_table` — asserts `(summoner, greater_aspect)` is absent,
  guarding against a future edit silently force-fitting the declined unit.
- `base_summoner_does_not_ground_via_unchained_summoners_own_entries` — the cross-owner
  negative control described above.

**RED confirmed for the intended reason before commit:** temporarily removed the 5-entry block
(leaving the table's prior 27 entries untouched) and re-ran
`cargo test --locked --bin v06_work_inventory class_feature_known_synonym_grounded_tests`:
`wave_40_lane_b_entries_match_the_receipts_own_manifest` FAILED with
`expected ("summoner", "bond_senses", "class_feature.apg.summoner.bond_senses_rounds_per_day")
to be a live table entry` (10 passed, 1 failed) — the other 10 pre-existing tests were
unaffected, confirming the failure was isolated to the intended assertion. Restored the 5-entry
block; **GREEN**: `cargo test --locked --bin v06_work_inventory
class_feature_known_synonym_grounded_tests` → `11/11` pass (3 new).

`cargo test --locked --bin v06_work_inventory` (full bin) → **544/544 pass (3 new, 0
regressed)**, this cycle's own pre-regen HEAD.

`src/rules_core/pilot_compute/mod.rs` carries **zero diff** (`git diff --stat -- src/rules_core/
pilot_compute/mod.rs` empty) — every id this table now recognizes was already shipped; this
cycle only made 5 more of them visible to the classifier.

## Movement — the real, regen-verified delta (isolated to this cycle's own 5 units)

Guarded regen ran to completion this cycle (`cargo run --locked --bin v06_work_inventory`,
debug build, full 51,508-file corpus scan — ran past the first 600s poll window before
completing; exact wall-clock not independently timestamped, estimate only). `docs/work-
inventory.json` regenerated, dated `"generated_at": "2026-09-04T08:02:10Z"` (its own top-level
field). `git status --porcelain -- docs/work-inventory.json` shows `M` (modified, not stale).

Before/after bucket counts, computed twice — once via `completion_atlas.py --check` directly on
each snapshot (old = `git show d7184384d8:docs/work-inventory.json`, swapped into place
temporarily and restored byte-identical afterward, confirmed via a `json.dumps(sort_keys=True)`
diff), once via an independent Python re-implementation of `_bucket_of`'s own DONE/V/M/U/X/Z
logic (both agree):

| Bucket | Before (`d7184384d8`, includes lane A's own 7) | After (this cycle) | Δ |
|---|---:|---:|---:|
| DONE | 25353 | 25358 | **+5** |
| D | 2528 | 2523 | **−5** |
| A / B / C / M / V / U / X / Z | unchanged | unchanged | 0 |

**Exactly this cycle's own 5 units, zero collateral movement, zero unattributed remainder** —
confirmed by naming each unit directly:

| Unit | status before | status after |
|---|---|---|
| `advanced_players_guide:class_feature:summoner_bond_senses` | engine-does-not-hold | **grounded** |
| `advanced_players_guide:class_feature:summoner_maker_s_call` | engine-does-not-hold | **grounded** |
| `advanced_players_guide:class_feature:summoner_merge_forms` | engine-does-not-hold | **grounded** |
| `advanced_players_guide:class_feature:summoner_twin_eidolon` | engine-does-not-hold | **grounded** |
| `advanced_players_guide:class_feature:summoner_summon_monster` | engine-does-not-hold | **grounded** |
| `advanced_players_guide:class_feature:summoner_greater_aspect` | engine-does-not-hold | engine-does-not-hold (unchanged, declined) |

All 5 closed units carry `evidence:
"explanation_id_observed_via_known_class_feature_synonym"` and `status: "grounded"` (bucket
DONE directly — none landed in bucket V, unlike 2 of lane A's own 7).

## Figures (every number, its command, its denominator)

- `6` Summoner (non-Unchained) units in this cycle's own scope, `5` closed, `1` declined —
  `python3 -c` filter over `docs/work-inventory.json`'s `units`, `corpus_key` starting with
  `"Summoner ~"`, `evidence == "class_feature_no_dedicated_magnitude_id_matched_the_record_slug"`.
- `5` synonym-table entries added — this receipt's own table above.
- `11` of `11` `class_feature_known_synonym_grounded_tests` pass (3 new) — `cargo test --locked
  --bin v06_work_inventory class_feature_known_synonym_grounded_tests`.
- `544` of `544` `v06_work_inventory` bin tests pass (3 new, 0 regressed) — `cargo test --locked
  --bin v06_work_inventory`.
- `0` diff in `src/rules_core/pilot_compute/mod.rs` — `git diff --stat -- src/rules_core/
  pilot_compute/mod.rs`.
- `population=49438 unclassified=0 overlap=0 citation_failures=0` — `python3
  scripts/completion_atlas.py --check`, this cycle's own post-regen, post-citation-fix HEAD.
- `DONE: 25353 -> 25358 (+5)`, `D: 2528 -> 2523 (-5)` — `completion_atlas.py --check` run against
  the pre-cycle (`d7184384d8`) and post-cycle `docs/work-inventory.json`, both explicit
  denominators stated above.
- `10` citation pins re-derived (9 in `BUCKET_DEFINITIONS`'s A/B/C/D/M/V/U/X/Z entries + 1 in the
  `DONE` entry, the latter a SILENT staleness of the exact shape wave 38 lane C already found
  once — `scripts/completion_atlas.py`'s own diff, this receipt's "Files touched" section).

## Row-count command output

```
$ grep -n "^| 37 |" docs/release/SD-34-book-completion/kanban.md | tail -1
| 37 | `mine-bucket-d` | 3 | ... (accumulating row, this cycle appends its own sentence) ...
```

Row 37 (`mine-bucket-d`) is the same accumulating row every prior bucket-D mining cycle appends
into.

## Build scope verified

- `cargo test --locked --bin v06_work_inventory` → 544/544 pass (3 new), this cycle's own
  pre-regen HEAD.
- `cargo test --locked --no-run` (full workspace) → **EXIT=0**, run before the guarded regen
  (per `decisions.md §12` L7, this is a pure build-integrity check independent of
  `docs/work-inventory.json`'s content, so its ordering relative to the regen does not un-verify
  it — unlike a figure-bearing assertion).
- Desktop crate (`apps/desktop/src-tauri`) — not run: `git diff --stat HEAD -- apps/desktop/` is
  empty, no file under `apps/desktop/` touched.

## Sweep population

`corpus_literal_sweep --json-out` (guarded regen chain step 1, required before the inventory
regen): `48706 records examined of 51476 read, 413314 tokens compared (9 synthesized), 51463
digests checked, 0 findings CLEAN`. No `data/corpus/**` record was added, changed, or removed
this cycle (`git diff --stat -- data/corpus/` empty, confirmed), so the examined-population is
expected to match the corpus's own current size, not grow — consistent with `decisions.md §12`
L8's rule (a growth is only required when records are ADDED).

`derived_evaluator_fixture_check --json-out`: `1839 unit(s) cleared over 2580 fixture row(s); 0
failed; 0 not ingested` — clean, required guarded-regen prerequisite.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`) — no figure in this receipt was derived from the pinned oracle
corpus; every magnitude credited this cycle was already transcribed and unit-tested (both at the
`pilot_compute` layer and now at the classifier layer) against `data/corpus/**` by a pre-existing
compute function this cycle only made VISIBLE to the classifier, not computed anew. Cited for
completeness per the receipt schema.

## Known pre-existing instrument staleness, observed but NOT this cycle's scope to fix

`python3 scripts/box_ledger.py --check` exits 1 (`uncovered=28223`) both before AND after this
cycle's own change (`28228` at `d7184384d8`, `28223` at this cycle's own HEAD — the delta is
exactly this cycle's own 5 units, so the tool is internally consistent; the failure itself is
pre-existing). `THE-BOX.md` is an SD-33 artifact, not referenced by SD-34's own
`workflow-instruction.md §6/§7` per-cycle gate (which is `completion_atlas.py --check`, confirmed
green above) — SD-34's own §5 states "There is no `THE-BOX.md` in this bundle." Named here for
visibility, not fixed: out of this cycle's own granted write scope (bucket-D mining, this
package's own artifacts), and pre-existing well before this cycle started.

## Status

**complete.** 5 of 6 assigned units closed to DONE, regen-confirmed with a clean, fully-
attributed before/after delta and zero collateral movement. The 6th (Greater Aspect) is
correctly, honestly declined with a direct-read reason (no compute function exists for base
Summoner's Greater Aspect anywhere in `pilot_compute/mod.rs`) — genuinely unbuilt scope, not a
synonym-table gap, not forced.

## Movement, four buckets

- **Closure:** 5 (Bond Senses, Maker's Call, Merge Forms, Twin Eidolon, Summon Monster — all
  `engine-does-not-hold` → `grounded`).
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 10 `completion_atlas.py` citation pins re-derived (9 shifted by this
  cycle's own first insertion hunk, 1 — the `DONE` bucket's own `"grounded"` pin — caught going
  SILENTLY stale a second time in the exact shape wave 38 lane C already found once, landing on
  an unrelated doc-comment line that coincidentally still contained the substring "grounded")
  + 1 retro-logged deferral (Greater Aspect).

## Notes (judgment calls)

- **Why no re-aliasing was needed this cycle, unlike lane A's Bard/Sorcerer:** every one of the
  5 real ids is unconditionally pushed once its own level gate is reached (no
  `class_ability_activations`/`spells_selected` dependency the classifier's own sweep fixture
  fails to seed) — confirmed by the live dump showing a real non-zero value at the first swept
  level at or above each feature's own gate, with no `{0}`-only or absent-across-every-level
  candidate anywhere in the 7-id candidate set (5 assigned + 2 unused Summon Monster siblings).
- **Why `summon_monster_uses_per_day` was chosen over its two real siblings:** all three
  (`_duration_minutes`, `_uses_per_day`, `_spell_level`) are equally valid per lane A's own "any
  one sibling id proves the engine holds the record" precedent — `_uses_per_day` was picked
  because it reads least ambiguously against a future reader skimming the table (a "duration" or
  "spell level" id could be misread as belonging to a different feature's own descriptor).
- **Why the DONE-bucket citation caught a second silent staleness:** this cycle's own insertion
  landed exactly where wave 38 lane C's fix once did (`simple_kind_verdict`'s doc comment gained
  more prose above the real construction site) — the failure mode (`must_contain` passing on an
  unrelated doc-comment line containing the same bare word) recurs whenever a doc comment grows
  above a citation's own pinned line without the pin being re-verified by content, not just
  presence. `--check`'s own `citation_failures` count did not flag it (it silently still passed
  `must_contain`), so this was caught only by manually walking every one of the 10 pins by
  content this cycle, not by trusting the automated check's silence.

## Next-cycle plan

1. **Shape 2's remaining population after this cycle:** the 15 confirmed genuinely-different
   (new-chassis) units from wave 39 lane B's own table (Duelist 4, Shadowdancer 4, Assassin 2,
   Loremaster 2, Cleric's Aura 1, Paladin's Detect Evil 1, Wizard's Arcane Bond 1) plus Fighter's
   Weapon Training (1, needs a genuine new engine-side explanation id) and Psychic's Phrenic Pool
   (1, needs the classifier's own probe input widened) — all real, un-closed, Epic 4/5-shaped
   work per wave 39 lane B's own disposition. Re-derive the exact figure fresh at whatever HEAD
   the next lane starts from, per this bundle's own repeated lesson, rather than trusting this
   arithmetic.
2. **Summoner's own Greater Aspect** (1 unit) needs the eidolon evolution-point economy itself
   built out before it can carry a real, non-fabricated magnitude — the same larger blocker
   `ground_summoner_eidolon`'s own doc comment already names for the rest of Summoner's
   evolution-pool-dependent scope. Not a cheap next step; tracked as a deferral.
3. **`THE-BOX.md` staleness** (`box_ledger.py --check` exit 1, pre-existing) is visible but out
   of this bundle's own write scope — flag for whoever owns SD-33's remaining instrument debt, if
   still open.
