# Bestiary 1 subset 07 cycle receipt — 2026-07-20T09:36:30Z

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_07_resolves 2>&1 | tail -40
error[E0599]: no variant, associated function, or constant named `Derro` found for enum `MonsterId` in the current scope
   --> tests/sd22_beastiary1_subset_07_resolves.rs:108:43
    |
108 |     assert_eq!(monster_resolve(MonsterId::Derro, RuleSetId::Acg), None);
    |                                           ^^^^^ variant, associated function, or constant not found in `MonsterId`

error[E0599]: no variant, associated function, or constant named `Derro` found for enum `MonsterId` in the current scope
   --> tests/sd22_beastiary1_subset_07_resolves.rs:109:43
    |
109 |     assert_eq!(monster_resolve(MonsterId::Derro, RuleSetId::Crb), None);
    |                                           ^^^^^ variant, associated function, or constant not found in `MonsterId`

error[E0599]: no variant, associated function, or constant named `Derro` found for enum `MonsterId` in the current scope
   --> tests/sd22_beastiary1_subset_07_resolves.rs:117:44
    |
117 |     let derro = monster_resolve(MonsterId::Derro, RuleSetId::Bestiary1)
    |                                            ^^^^^ variant, associated function, or constant not found in `MonsterId`

error[E0599]: no variant, associated function, or constant named `AssassinVine` found for enum `MonsterId` in the current scope
   --> tests/sd22_beastiary1_subset_07_resolves.rs:124:52
    |
124 |     let assassin_vine = monster_resolve(MonsterId::AssassinVine, RuleSetId::Bestiary1)
    |                                                    ^^^^^^^^^^^^ variant, associated function, or constant not found in `MonsterId`

error[E0599]: no variant, associated function, or constant named `Centaur` found for enum `MonsterId` in the current scope
   --> tests/sd22_beastiary1_subset_07_resolves.rs:131:46
    |
131 |     let centaur = monster_resolve(MonsterId::Centaur, RuleSetId::Bestiary1)
    |                                              ^^^^^^^ variant, associated function, or constant not found in `MonsterId`

error[E0599]: no variant, associated function, or constant named `Cockatrice` found for enum `MonsterId` in the current scope
   --> tests/sd22_beastiary1_subset_07_resolves.rs:138:49
    |
138 |     let cockatrice = monster_resolve(MonsterId::Cockatrice, RuleSetId::Bestiary1)
    |                                                 ^^^^^^^^^^ variant, associated function, or constant not found in `MonsterId`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `codex` (test "sd22_beastiary1_subset_07_resolves") due to 13 previous errors
```

Failed for the intended reason: `MonsterId::Ankheg` / `AssassinVine` / `Centaur` /
`Cockatrice` / `Derro` did not exist yet (13 call sites across the new test file).

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_07_resolves 2>&1 | tail -15
running 6 tests
test all_five_subset_07_monsters_resolve_via_ruleset_bestiary1 ... ok
test derro_carries_racesubtype_others_do_not ... ok
test ankheg_resolves_via_ruleset_bestiary1 ... ok
test derro_resolves_by_key_via_ruleset_bestiary1_only ... ok
test derro_returns_none_for_ruleset_apg_acg_crb ... ok
test subset_01_through_06_monsters_still_resolve_unchanged ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b_monster_stat_block -- --ignored 2>&1 | tail -15
running 7 tests
test parses_real_subset_04_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_02_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_subset_03_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_06_cr_1_and_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_07_cr_3_monster_records_from_b1_races_lst ... ok
test parses_real_subset_05_cr_2_monster_records_from_b1_races_lst ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

$ cargo test --locked 2>&1 | grep -c "test result: ok"
427
$ cargo test --locked 2>&1 | grep "test result:" | grep -v "0 failed"
(no output -- 0 failed everywhere; sibling-preservation holds)

$ cargo clippy --locked --tests -- -D warnings 2>&1 | tail -5
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 30.84s
$ echo $?
0
```

## Files touched

- `src/rules_core/rules_tables/beastiary1/monster_subset_07.rs` (NEW) — Ankheg, Assassin
  Vine, Centaur, Cockatrice, Derro chassis, each function's doc comment citing the exact
  source line and tokens transcribed.
- `src/rules_core/rules_tables/beastiary1/mod.rs` (MODIFIED) — `pub mod monster_subset_07;`,
  five new `MonsterId` variants, five new `monster_resolve` match arms, five new
  `monster_key_resolve` key entries, updated module doc comment.
- `tests/sd22_beastiary1_subset_07_resolves.rs` (NEW) — 6 tests: per-monster resolution,
  all-five batch resolution, sibling-preservation (subsets 01-06's 31 monsters still
  resolve), cross-book invariant, RACESUBTYPE variety check, key-based resolution.
- `tests/sd17_b_monster_stat_block.rs` (MODIFIED) — real-corpus grounding test
  `parses_real_subset_07_cr_3_monster_records_from_b1_races_lst`, `#[ignore]`-gated on
  `PCGEN_CORPUS_ROOT`.
- `docs/release/SD-22/corpus-source-inventory.md` (MODIFIED) — §3.1 row 7 added (replacing
  the `...` placeholder row), documenting the CR-3 band move and the corrected roster.

## Cycle metadata

- cycle_id: 2026-07-20T09:36:30Z
- duration: ~35 minutes
- bundle_criterion: criteria 14-17 (Bestiary 1 monster-block subset cycles)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst:Ankheg` (line
  18), `:Assassin Vine` (line 29), `:Centaur` (line 60), `:Cockatrice` (line 73), `:Derro`
  (line 104) — real public PCGen corpus; per `decisions.md §5`
- RuleSetId: Bestiary1
- ingest_pipeline_version: 2 (per `../ingest.md §6`)

## Roster derivation

Before writing any GREEN code, independently re-enumerated every real, non-`#`-commented,
non-`.MOD`/`.COPY=` CR:3 monster stat-block row in `b1_races.lst` directly against the live
corpus file:

```
$ grep -P "\tCR:3\t|\tCR:3$" b1_races.lst | grep -v "^#" | cut -f1 | sort -u | wc -l
44
```

Excluding parenthetical sub-variant names (e.g. `Ant (Drone)`, `Ape (Dire)`, `Dragon
(Black)`, `Elemental (Air/Medium)`) and `.MOD`-suffixed override rows (e.g. `Iron Cobra
(Adamantine Cobra).MOD`) — the same exclusion rule every prior subset has used — leaves 20
clean, standalone CR:3 species names:

```
Ankheg, Assassin Vine, Centaur, Cockatrice, Derro, Doppelganger, Dryad, Ettercap,
Gelatinous Cube, Hell Hound, Lion, Ogre, Pegasus, Rust Monster, Shadow, Unicorn,
Violet Fungus, Wasp Swarm, Wight, Yeth Hound
```

None of these 20 names collide with the 31 monsters already used across subsets 01-06.
Subset 07 ships the first five alphabetically: Ankheg, Assassin Vine, Centaur, Cockatrice,
Derro. Remaining 15 CR-3 names carry forward for subset 08+.

All five of this subset's real rows carry no `NATURALATTACKS:` token — each fights via an
`ABILITY:Internal` cross-reference (Ankheg: Bite; Assassin Vine: Slam; Centaur: Hoof;
Cockatrice: Bite) or, for Derro, via weapons (`AUTO:WEAPONPROF|Crossbow (Repeating
Light)|...`) instead. Transcribed as empty `natural_attacks` lists — the literal fact of
the token's absence, not an invented value, same precedent as subset 04's
Choker/Crocodile/Dark Creeper and subset 06's Vargouille/Wolverine/Worg.

## kanban

- card: `t_35de73cd` on `codex-tranche-5` (status=done)
