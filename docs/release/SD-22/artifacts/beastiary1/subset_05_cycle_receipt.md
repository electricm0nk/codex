# Bestiary 1 subset 05 cycle receipt — 2026-07-20T08:20:01Z

## Roster derivation (not a roster correction of an existing row)

`corpus-source-inventory.md` §3.1 had no illustrative sample row for
subset 5 — only a placeholder `...` row. Before writing any GREEN code,
this cycle independently re-enumerated every real, non-`#`-commented,
non-`.MOD`/`.COPY=` CR:2 monster stat-block row in `b1_races.lst`
directly:

```
$ grep -P "CR:2\t" b1_races.lst | grep -v "^#" | grep -v "\.MOD" | grep -v "\.COPY=" | wc -l
34
```

34 rows in the whole file carry a `CR:2` token. Excluding parenthetical
sub-variant names (e.g. "Ant (Giant)", "Cat (Cheetah)", "Demon
(Dretch)") — the same exclusion rule subsets 01-04 all already
established (those name a variant of a broader creature rather than a
clean, unambiguous base species) — leaves 19 clean CR:2 species names:

- subsets 03+04 (already used): Bat Swarm, Boar, Boggard, Bugbear, Cave
  Fisher, Choker, Crocodile, Dark Creeper, Iron Cobra, Morlock (10)
- unused, alphabetical: Rat Swarm, Sahuagin, Shark, Shocker Lizard,
  Skum, Vargouille, Wolverine, Worg, Yellow Musk Creeper (9)

This cycle lands the next five alphabetically after subset 04's
"Morlock": **Rat Swarm** (line 334), **Sahuagin** (line 345), **Shark**
(line 360), **Shocker Lizard** (line 362), **Skum** (line 366). The
remaining four names (Vargouille, Wolverine, Worg, Yellow Musk Creeper)
stay available for a future subset 06.

`corpus-source-inventory.md` §3.1 gets a new row 5 in this cycle's
commit (no correction needed — no prior row existed to correct).

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_05_resolves 2>&1 | tail -50
error[E0599]: no variant, associated function, or constant named `ShockerLizard` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Skum` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Skum` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Skum` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Skum` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Shark` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Sahuagin` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Skum` found for enum `MonsterId` in the current scope
error: could not compile `codex` (test "sd22_beastiary1_subset_05_resolves") due to 12 previous errors
```

Failed for the intended reason: the five new `MonsterId` variants and
`monster_subset_05` module did not exist yet.

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_05_resolves 2>&1 | tail -15
running 6 tests
test rat_swarm_resolves_via_ruleset_bestiary1 ... ok
test shocker_lizard_resolves_by_key_via_ruleset_bestiary1_only ... ok
test all_five_subset_05_monsters_resolve_via_ruleset_bestiary1 ... ok
test shark_has_no_walk_token_sahuagin_and_skum_accumulate_multiple_natural_attack_tokens ... ok
test skum_returns_none_for_ruleset_apg_acg_crb ... ok
test subset_01_02_03_04_monsters_still_resolve_unchanged ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b_monster_stat_block -- --ignored 2>&1 | tail -10
running 5 tests
test parses_real_subset_04_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_02_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_subset_03_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_05_cr_2_monster_records_from_b1_races_lst ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked 2>&1 | grep -c "test result: ok"
425
$ cargo test --locked 2>&1 | grep "test result:" | grep -v "0 failed"
(no output — zero failures anywhere; sibling-preservation holds, including
subsets 01/02/03/04's own tests, Epic 6's happy-path integration test, and
every APG/ACG suite)

$ cargo clippy --locked --tests -- -D warnings 2>&1 | tail -5
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 30.63s
(clean, exit code 0)
```

## New parsing shapes this subset exercises

- **Shark's row has no `Walk` pair in its `MOVE:` token at all** —
  `MOVE:Swim,60` only. Every prior subset's monsters (01-04) had a Walk
  pair. `parse_walk_speed` correctly returns `None` for this case
  (confirmed via the `sd17_b_monster_stat_block` grounding test:
  `shark.speed_ft == None`). The production module transcribes this as
  `speed_ft: 0`, documented as the literal fact the real row records no
  land-movement token — not an invented value; it matches the real,
  published Shark stat block's "Speed 0 ft., swim 60 ft."
- **Sahuagin and Skum's rows each carry two separate `NATURALATTACKS:`
  tab fields** (one plain, one pipe-separated for Sahuagin; both
  pipe-separated for Skum), which the parser's existing `.extend()` loop
  accumulates into one combined `natural_attacks` list (3 entries for
  Sahuagin, 4 for Skum). No parser widening was needed — the existing
  per-field accumulation logic already handles this correctly.

## Files touched

- `src/rules_core/rules_tables/beastiary1/monster_subset_05.rs` (NEW)
- `src/rules_core/rules_tables/beastiary1/mod.rs` (MODIFIED; additive only
  — `pub mod monster_subset_05;`, 5 new `MonsterId` variants, 5 new match
  arms in `monster_resolve`/`monster_key_resolve`, module doc-comment note)
- `tests/sd22_beastiary1_subset_05_resolves.rs` (NEW)
- `tests/sd17_b_monster_stat_block.rs` (MODIFIED; real-corpus grounding
  test for subset 05, additive)
- `docs/release/SD-22/corpus-source-inventory.md` (MODIFIED; new §3.1 row
  5 for subset 05 — no prior row existed to correct)

## Cycle metadata

- cycle_id: 2026-07-20T08:20:01Z
- bundle_criterion: criteria 14-17 (Bestiary 1 monster-block subsets,
  re-verified against a fifth subset)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst:334` (Rat Swarm), `:345` (Sahuagin), `:360` (Shark), `:362` (Shocker Lizard), `:366` (Skum)
- RuleSetId: Bestiary1

## kanban

- card: see `docs/release/SD-22/progress.md` / `receipts.md` for the
  minted card ID (Step 10b runs after this receipt is written).
