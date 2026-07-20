# Bestiary 1 subset 06 cycle receipt — 2026-07-20T09:21:36Z

## Roster derivation (band-exhaustion cleanup, not a straight CR-band continuation)

Before writing any RED test, this cycle independently re-enumerated every real,
non-`#`-commented, non-`.MOD`/`.COPY=` CR:1 and CR:2 monster stat-block row in
`b1_races.lst` directly against the live corpus file at
`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`
(not from the prior cycle's summary):

- CR:1 — 27 rows carry a `CR:1` token. Excluding parenthetical sub-variant
  names (the same exclusion rule every prior subset has used), subsets 01+02
  already used every clean CR:1 name except **Squid** (line 380) and
  **Troglodyte** (line 390) — 2 remaining, not enough alone for a
  five-monster subset.
- CR:2 — 34 rows carry a `CR:2` token. Excluding parentheticals leaves 19
  clean species names; subsets 03+04+05 used the first fifteen
  alphabetically, leaving **Vargouille** (line 401), **Wolverine** (line
  416), **Worg** (line 418), and **Yellow Musk Creeper** (line 430) — 4
  remaining, also not enough alone.

Rather than ship an undersized four-monster subset now and strand the two
leftover CR-1 monsters for an even smaller subset later (or jump straight to
CR 3 and leave both remainders unresolved indefinitely), this cycle combines
both remainders into one six-monster subset that fully exhausts CR 1 and
CR 2, so subset 07 can start CR 3 cleanly. The combined pool sorts
alphabetically as Squid, Troglodyte, Vargouille, Wolverine, Worg, Yellow
Musk Creeper — which happens to already be CR-ascending too (both CR-1
leftovers sort before all four CR-2 leftovers), so no further
band-interleaving decision was needed.

Added a new §3.1 row 6 to `corpus-source-inventory.md` for subset 6.

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_06_resolves 2>&1 | tail -40
error[E0599]: no variant, associated function, or constant named `YellowMuskCreeper` found for enum `MonsterId` in the current scope
  --> tests/sd22_beastiary1_subset_06_resolves.rs:56:21
   |
56 |         (MonsterId::YellowMuskCreeper, "Yellow Musk Creeper", 2.0),
   |                     ^^^^^^^^^^^^^^^^^ variant, associated function, or constant not found in `MonsterId`
... (14 total E0599 errors, one per new MonsterId variant call site: YellowMuskCreeper, Troglodyte,
     Vargouille, Wolverine, Worg — all not yet defined on the enum)
error: could not compile `codex` (test "sd22_beastiary1_subset_06_resolves") due to 14 previous errors
```

Failed for the intended reason: the six new `MonsterId` variants did not yet exist.

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_06_resolves 2>&1 | tail -20
running 6 tests
test all_six_subset_06_monsters_resolve_via_ruleset_bestiary1 ... ok
test squid_resolves_via_ruleset_bestiary1 ... ok
test troglodyte_accumulates_four_natural_attacks_vargouille_wolverine_worg_have_none ... ok
test subset_01_through_05_monsters_still_resolve_unchanged ... ok
test yellow_musk_creeper_resolves_by_key_via_ruleset_bestiary1_only ... ok
test yellow_musk_creeper_returns_none_for_ruleset_apg_acg_crb ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b_monster_stat_block -- --ignored 2>&1 | tail -20
running 6 tests
test parses_real_subset_04_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_03_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_02_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_subset_05_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_06_cr_1_and_cr_2_monster_records_from_b1_races_lst ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

```
$ cargo test --locked 2>&1 | tail -20
... (last of 426 `test result: ok` blocks across every suite; grepped full output
     for `test result:` lines not containing `0 failed`, found none)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 28.05s
```

Clean, exit code 0.

## Files touched

- `src/rules_core/rules_tables/beastiary1/monster_subset_06.rs` (NEW) — six monster chassis functions
- `src/rules_core/rules_tables/beastiary1/mod.rs` (MODIFIED) — `monster_subset_06` module registration, six new `MonsterId` variants, `monster_resolve` match arms, `monster_key_resolve` key mappings
- `tests/sd22_beastiary1_subset_06_resolves.rs` (NEW) — 6 acceptance tests
- `tests/sd17_b_monster_stat_block.rs` (MODIFIED) — real-corpus-gated grounding test for all six monsters
- `docs/release/SD-22/corpus-source-inventory.md` (MODIFIED) — §3.1 row 6 added
- `docs/release/SD-22/progress.md` (MODIFIED) — status matrix + cycle log

## Transcribed source data

All six monsters transcribed directly from
`pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst` (verified via `sed -n '<line>p'`
against the live corpus checkout, not from memory or any prior cycle's summary):

| Monster | Line | CR | SIZE | Walk speed | RACETYPE | RACESUBTYPE | Natural attacks |
|---|---|---|---|---|---|---|---|
| Squid | 380 | 1 | M | none (`MOVE:Swim,60,Jet,240`) | Animal | Aquatic | Bite 1d3, Tentacles 1d4 |
| Troglodyte | 390 | 1 | M | 30 | Humanoid | Reptilian | Claw 1d4, Claw (with weapon attack) 1d4, Bite 1d4, Bite (with weapon attack) 1d4 |
| Vargouille | 401 | 2 | S | none (`MOVE:Fly,30`) | Outsider | Evil\|Extraplanar | (none — no `NATURALATTACKS:` token) |
| Wolverine | 416 | 2 | M | 30 | Animal | (none) | (none — no `NATURALATTACKS:` token) |
| Worg | 418 | 2 | M | 50 | Magical Beast | (none) | (none — no `NATURALATTACKS:` token) |
| Yellow Musk Creeper | 430 | 2 | M | 5 | Plant | (none) | Tendril 1d4 |

## Cycle metadata

- cycle_id: 2026-07-20T09:21:36Z
- duration: ~35 minutes
- bundle_criterion: criteria 14-17 (Bestiary 1 monster-block ingest)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst:380` (Squid), `:390` (Troglodyte), `:401` (Vargouille), `:416` (Wolverine), `:418` (Worg), `:430` (Yellow Musk Creeper) — real corpus; `decisions.md §5`
- RuleSetId: Bestiary1
- ingest_pipeline_version: 2 (per `./ingest.md §6`)

## kanban

- card: see `receipts.md` / `progress.md` for the minted card ID (backfilled after `hermes kanban` mint)
- audit_comment: n/a
