# Bestiary 1 subset 04 cycle receipt — 2026-07-20T07:20:20Z

## Roster derivation (not a roster correction of an existing row)

`corpus-source-inventory.md` §3.1 had no illustrative sample row for
subset 4 — only a placeholder `...` row. Before writing any GREEN code,
this cycle enumerated every real, non-`#`-commented, non-`.MOD`/`.COPY=`
CR:2 monster stat-block row in `b1_races.lst` directly. 34 rows in the
whole file carry a `CR:2` token. Excluding parenthetical sub-variant
names (e.g. "Ant (Giant)", "Cat (Cheetah)", "Demon (Dretch)") — the same
exclusion rule subsets 01-03 all already established (those name a
variant of a broader creature rather than a clean, unambiguous base
species) — leaves 19 clean CR:2 species names:

- subset 03 (already used): Bat Swarm, Boar, Boggard, Bugbear, Cave Fisher
- unused, alphabetical: Choker, Crocodile, Dark Creeper, Iron Cobra,
  Morlock, Rat Swarm, Sahuagin, Shark, Shocker Lizard, Skum, Vargouille,
  Wolverine, Worg, Yellow Musk Creeper

This cycle lands the next five alphabetically after subset 03's "Cave
Fisher": **Choker** (line 70), **Crocodile** (line 83), **Dark Creeper**
(line 89), **Iron Cobra** (line 249), **Morlock** (line 297). The
remaining nine names (Rat Swarm, Sahuagin, Shark, Shocker Lizard, Skum,
Vargouille, Wolverine, Worg, Yellow Musk Creeper) stay available for a
future subset 05.

`corpus-source-inventory.md` §3.1 gets a new row 4 in this cycle's
commit (no correction needed — no prior row existed to correct).

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_04_resolves 2>&1 | tail -40
error[E0599]: no variant, associated function, or constant named `Choker` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Crocodile` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `DarkCreeper` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `IronCobra` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Morlock` found for enum `MonsterId` in the current scope
... (14 total E0599 errors, one per call site referencing the not-yet-existing MonsterId variants)
error: could not compile `codex` (test "sd22_beastiary1_subset_04_resolves") due to 14 previous errors
```

Failed for the intended reason: the five new `MonsterId` variants and
`monster_subset_04` module did not exist yet.

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_04_resolves 2>&1 | tail -15
running 6 tests
test all_five_subset_04_monsters_resolve_via_ruleset_bestiary1 ... ok
test choker_crocodile_dark_creeper_have_no_natural_attacks_iron_cobra_and_morlock_do ... ok
test dark_creeper_returns_none_for_ruleset_apg_acg_crb ... ok
test morlock_resolves_by_key_via_ruleset_bestiary1_only ... ok
test subset_01_02_03_monsters_still_resolve_unchanged ... ok
test choker_resolves_via_ruleset_bestiary1 ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b_monster_stat_block -- --ignored 2>&1 | tail -10
running 4 tests
test parses_real_subset_03_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_04_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_subset_02_cr_1_monster_records_from_b1_races_lst ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked 2>&1 | grep -c "test result: ok"
424
$ cargo test --locked 2>&1 | grep "test result:" | grep -v "0 failed"
(no output — zero failures anywhere; sibling-preservation holds, including
subsets 01/02/03's own tests, Epic 6's happy-path integration test, and
every APG/ACG suite)

$ cargo clippy --locked --tests -- -D warnings 2>&1 | tail -5
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 27.41s
(clean, exit code 0)
```

## Files touched

- `src/rules_core/rules_tables/beastiary1/monster_subset_04.rs` (NEW)
- `src/rules_core/rules_tables/beastiary1/mod.rs` (MODIFIED; additive only
  — `pub mod monster_subset_04;`, 5 new `MonsterId` variants, 5 new match
  arms in `monster_resolve`/`monster_key_resolve`, module doc-comment note)
- `tests/sd22_beastiary1_subset_04_resolves.rs` (NEW)
- `tests/sd17_b_monster_stat_block.rs` (MODIFIED; real-corpus grounding
  test for subset 04, additive)
- `docs/release/SD-22/corpus-source-inventory.md` (MODIFIED; new §3.1 row
  4 for subset 04 — no prior row existed to correct)

## Cycle metadata

- cycle_id: 2026-07-20T07:20:20Z
- bundle_criterion: criteria 14-17 (Bestiary 1 monster-block subsets,
  re-verified against a fourth subset)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst:70` (Choker), `:83` (Crocodile), `:89` (Dark Creeper), `:249` (Iron Cobra), `:297` (Morlock)
- RuleSetId: Bestiary1

## kanban

- card: see `docs/release/SD-22/progress.md` / `receipts.md` for the
  minted card ID (Step 10b runs after this receipt is written).
