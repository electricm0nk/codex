# Cycle 2 — Gate 1 (Shape Closure) / kanban row 17 residual closure

- **Card ID:** `epic-7-shape-categorization-100` (kanban row 17)
- **Commit SHA:** see `git log -1` after push (recorded below at push time)
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs`
  - `scripts/close_row17_provisional_defaults.py`
  - `scripts/tests/test_close_row17_provisional_defaults.py`
  - `data/corpus/occult_adventures/class_feature/psychic/phrenic_pool.json`
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 17 only)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/row17-census.json`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §27a`: *"ROW 17 HONEST SIZE"* must reach **0**
  (`scripts/row17_census.py`'s `row17_honest_size` total).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/fetch-pcgen-oracle.sh --dest`,
  `oa_abilities_class.lst` re-read live to confirm the CHA/WIS discipline split).
- **Status:** complete

## Re-derivation before work (`§17a`)

```
python3 scripts/row17_census.py
```
reconfirmed the brief's figure exactly:
```
ROW 17 HONEST SIZE                  1
```
— the same residual unit named by the prior cycle's receipt
(`row17-categorization-pass_cycle-1_cycle_receipt.md`):
`occult_adventures:class_feature:Psychic ~ Phrenic Pool`.

## What was actually missing

`ground_psychic_class_features` (`src/rules_core/pilot_compute/mod.rs`) hard-coded Charisma for
`Phrenic Pool`. The real rule, per `oa_abilities_class.lst:1188`-1196 (fetched live via
`scripts/fetch-pcgen-oracle.sh`, not recalled): each of the 9 `KEY:Psychic Discipline ~ <Name>`
records carries its own `BONUS:VAR|PhrenicPoolAbility|<CHA|WIS>` token —

- **CHA (4):** Abomination, Dream, Pain, Rapport
- **WIS (5):** Faith, Lore, Psychedelia, Self-Perfection, Tranquility

— matching the split the prior cycle's own module doc comment
(`src/rules_core/rules_tables/occult_adventures/psychic_features.rs`) already cited, per the brief's
own pointer ("PCGen's own answer already cited in this repo's own module docs").

## Mechanism

1. Added `PSYCHIC_DISCIPLINE_CHOICE_ID` (`"choice:psychic_discipline"`) and the 9 selection ids,
   same recognition idiom as `SORCERER_BLOODLINE_CHOICE_ID`/`ORACLE_MYSTERY_CHOICE_ID`.
2. Added `psychic_discipline_pool_ability(input, ability_modifiers) -> Option<(i16, &str)>`, reading
   the choice via the existing `choice_selection` helper. Returns `None` for no selection or an
   unrecognized one — **no ability score is ever fabricated for an unmade choice** (`§1a`).
3. `ground_psychic_class_features` now takes `input: &CharacterInput` (matching the shape every
   sibling `ground_*_class_features` dispatch in this file already uses) and only grounds
   `phrenic_pool.value` when a real discipline is resolved; the other three magnitude-bearing
   features (discipline-independent) are unaffected and still ground unconditionally.
4. Updated the call site (`class:psychic` branch of the class-feature dispatch) to pass `input`.

## Tests (TDD, RED confirmed for the intended reason)

- `psychic_level_20_reaches_every_real_magnitude_with_a_real_charisma_score` (pre-existing) —
  updated to select `discipline:rapport` (CHA); still asserts `phrenic_pool.value == 13`.
- `psychic_phrenic_pool_uses_the_real_ability_for_every_discipline` (new) — all 9 disciplines, each
  asserting the correct ability modifier flows through, via `compute_pilot_base_chassis` (real
  dispatch, not a unit call).
- `psychic_phrenic_pool_is_ungrounded_with_no_chosen_discipline` (new) — no selection: Phrenic Pool
  grounds nothing; every other psychic magnitude still grounds.
- `psychic_phrenic_pool_is_ungrounded_for_an_unrecognized_discipline_selection` (new) — an
  unrecognized `discipline:*` value is treated exactly like no selection.

**RED→GREEN mutation proof, live, at the dispatch altitude** (`compute_pilot_base_chassis` →
`compute_class_chassis`, not unit-call-only): mutated `psychic_discipline_pool_ability` to
unconditionally return `Some((ability_modifiers.charisma, "Charisma"))`. Re-ran the two
no-selection/unrecognized-selection tests: both **FAILED** for the intended reason (Phrenic Pool
grounded a value with no discipline chosen). Reverted; re-ran the full targeted suite: **GREEN**.

```
cargo test --lib psychic_          # 17/17 pass (post-revert)
cargo test --lib rules_core::pilot_compute::   # 901/901 pass
```

## Marker closure

`close_row17_provisional_defaults.py` was scoped to `monster_ability` only (its own docstring
explicitly excluded `Psychic ~ Phrenic Pool` as "a `class_feature` genuinely marked for a different
reason"). Widened it generically (not a one-record special case) with a paired
`close_class_feature_corpus` function driven by a new `_CLASS_FEATURE_PROVISIONAL_RESOLUTIONS`
table — any future `class_feature` marked provisional for this same reason class (a
bloodline/mystery/domain-shaped per-character ability-score choice) closes the same way once named
here and proven compute-side. 6 new TDD tests (`test_close_row17_provisional_defaults.py`), all
green, alongside the pre-existing 6 (including `test_non_monster_ability_kinds_are_never_scanned`,
unchanged and still passing — the new function is additive, not a rewrite of the old one).

```
python3 scripts/close_row17_provisional_defaults.py --dry-run
# 0 monster_ability record(s) would be resolved
# Psychic ~ Phrenic Pool: provisional marker cleared -- ...
# 1 class_feature record(s) would be resolved

python3 scripts/close_row17_provisional_defaults.py
# (same, applied for real)
```

`git status --porcelain` checked before and after every write; only the one named corpus record
changed (re-serialized sorted-keys with the two marker fields removed — no other field's value
changed, confirmed by `git diff`).

## Re-derivation after work (`§17a`)

```
python3 scripts/row17_census.py --output artifacts/gate-1-shape-closure/row17-census.json
```
```
row 17 census (decisions.md §27a/§27b, kanban.md row 17) — population 34397
  §27 provisional default             0   (corpus-wide total incl. done units: 0)
  ROW 17 HONEST SIZE                  0
  not_ingested (no_record)            0
```

`python3 scripts/row17_census.py --check` — exit 0.

## Generalization check (`§17` — generic pass, not per-object)

The brief named bloodline/mystery/domain as the same class of unexpressed per-character choice.
Re-checked: `pilot_compute::mod.rs` already carries recognition-only choice seams for Sorcerer
bloodline (`SORCERER_BLOODLINE_CHOICE_ID`) and Oracle mystery (`ORACLE_MYSTERY_CHOICE_ID`), but
neither of those classes' magnitudes vary their *governing ability score* by sub-choice the way
Psychic Discipline does (Oracle is uniformly Charisma-based; Sorcerer's bloodline choice gates
which power/spell is granted, not which ability score backs a pool). The row 17 census — the
actual measurement instrument for this population, not a fresh grep — confirms the corpus-wide
`§27 provisional default` count is now **0**: no other unit currently carries this marker for this
or any other reason. **Psychic ~ Phrenic Pool was the only unit of this shape**, and it is now
closed; the input added (`psychic_discipline_pool_ability` + the
`_CLASS_FEATURE_PROVISIONAL_RESOLUTIONS`/`close_class_feature_corpus` mechanism) generalizes to any
future unit of the same shape without a second copy of either mechanism.

## Kanban

Row 17: `in-progress` → `complete`, Cycle `1` → `2`. `ROW 17 HONEST SIZE` is 0, the `§27a`
criterion. Rows 11 and 15 left untouched (`in-progress`).

- **Notes:** none deferred; no `## Open blockers` filed; no exemption written.
- **Discovery forwards:** none.
- **Next-cycle plan:** row 17 is closed; `decisions.md §10` closure now depends on the remaining
  non-`complete` Epic cards (rows 11, 15, 18, 19 per this session's other live lanes).
