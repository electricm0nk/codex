# Bestiary 1 subset 01 cycle receipt — 2026-07-20T04:25:03Z

## What this cycle resolved

Epic 5's first cycle was blocked since its first attempt: `race_ability.rs`'s
`parse_lst_entry` only recognizes `RACE:`/`RACES:` pointer lines and
`ABILITY:` declarations, but the real Bestiary 1 monster records in
`b1_races.lst` are bare tab-delimited rows with the monster name as the
unprefixed first field (confirmed: `grep -c "RACE:" b1_races.lst` -> 0).
This cycle wrote a new sibling parser,
`src/pcgen_import/lst_parser/monster_stat_block.rs`, to close that gap
(kept as a sibling module rather than folded into `race_ability.rs`, so
that file's own documented scope boundary stays accurate), then landed
Epic 5's first monster-block subset on top of it.

## Roster correction (discovered this cycle, before writing any GREEN code)

`corpus-source-inventory.md` §3.1's illustrative subset-01 sample list is
"Goblin, Kobold, Orc, Skeleton, Zombie" (CR band "CR 1"). Before
transcribing anything, this cycle verified each of the five names
directly against the real `b1_races.lst` and found none of them is a
real, standalone CR-1 monster stat-block row:

```
$ grep -n '^Goblin\t' b1_races.lst   # 0 hits
$ grep -n '^Kobold\t' b1_races.lst   # 0 hits
$ grep -n '^Orc\t' b1_races.lst      # 0 hits
$ cat b1_races_pc.lst
###Block: Playable Races
Goblin.MOD   SOURCEPAGE:p.156
Kobold.MOD   SOURCEPAGE:p.183
Orc.MOD      SOURCEPAGE:p.222
```

Goblin, Kobold, and Orc exist in this book's data only as `.MOD`
overrides in the separate `b1_races_pc.lst` file, layered onto their
*playable-race* base defined under `core_essentials/races/<race>/` — not
as an independent Bestiary 1 monster stat block with its own CR/combat
data. Parsing a `.MOD` row as a fresh record would fabricate a stat block
this parser never actually read (the new parser explicitly skips
`.MOD`/`.COPY=` rows for this reason).

```
$ grep -n '^Skeleton (Human)' b1_races.lst
364:Skeleton (Human)  ...  CR:1/3  SOURCEPAGE:p.250
$ grep -n '^Zombie (Human)' b1_races.lst
436:Zombie (Human)  ...  CR:1/2  SOURCEPAGE:p.288
$ grep -n '^Skeleton\t\|^Zombie\t' b1_races.lst
439:Skeleton  MOVE:Walk,0  ...  (no CR: token — template-application shim)
440:Zombie    MOVE:Walk,0  ...  (no CR: token — template-application shim)
```

Skeleton (Human) is CR 1/3 and Zombie (Human) is CR 1/2, not CR 1; the
bare `Skeleton`/`Zombie` rows carry no `CR:` token at all (they only
exist to attach the Skeleton/Zombie template onto a base creature).

This is the same shape as the already-resolved Epic 3 Gunslinger/Magus
and Epic 4 "Alchemist (ACG-side)" roster defects: an illustrative sample
list authored before the real corpus was checked. Per this loop's
established self-healing precedent, this cycle corrected the roster
in-line rather than blocking: the five real CR-1 monsters with complete,
unambiguous, directly-transcribable stat-block rows are **Ghoul**
(line 200), **Gnoll** (line 212), **Goblin Dog** (line 213),
**Lizardfolk** (line 276), **Wolf** (line 414) — alphabetical order,
matching `corpus-source-inventory.md` §3's own default subset-ordering
rule. Full real-row token dumps for all five are in this cycle's session
log; the exact tokens transcribed are cited per-function in
`src/rules_core/rules_tables/beastiary1/monster_subset_01.rs`.

## Field-coverage scope boundary

Mirrors `rules_tables::crb::class_tables` and every SD-22 Epic 3/4 class
chassis module: only fields literally present as tokens on the real bare
row are transcribed (name, CR, size, walk speed, race type/subtype,
source page, natural attacks). AC, HP, and Fort/Ref/Will saves are
PCGen-computed at runtime from the `MONSTERCLASS:` hit-dice table and
ability-score modifiers, not literal row tokens — transcribing invented
values for them would be the fabricated-data risk `AGENTS.md` rules out,
so they are deferred to a future ingest slice (same posture as APG/ACG
class cycles deferring named per-level features).

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_01_resolves 2>&1 | tail -20
   Compiling codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
error[E0583]: file not found for module `beastiary1`
  --> src/rules_core/rules_tables/mod.rs:11:1
   |
11 | pub mod beastiary1;
   | ^^^^^^^^^^^^^^^^^^^
   |
   = help: to create the module `beastiary1`, create file "src/rules_core/rules_tables/beastiary1.rs" or "src/rules_core/rules_tables/beastiary1/mod.rs"
error: could not compile `codex` (lib) due to 1 previous error
```

```
$ cargo test --locked --test sd17_b_monster_stat_block 2>&1 | tail -20
   Compiling codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
error[E0583]: file not found for module `beastiary1`
  --> src/rules_core/rules_tables/mod.rs:11:1
   |
11 | pub mod beastiary1;
   | ^^^^^^^^^^^^^^^^^^^
error: could not compile `codex` (lib) due to 1 previous error
```

Both fail for the intended reason: `RuleSetId::Bestiary1` and the
`beastiary1` module were declared (this cycle's first edit, to make the
gap concrete) but the module's file didn't exist yet, and
`pcgen_import::lst_parser::monster_stat_block` didn't exist yet either
before this cycle wrote it.

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_01_resolves 2>&1 | tail -10
running 4 tests
test all_five_subset_01_monsters_resolve_via_ruleset_bestiary1 ... ok
test ghoul_resolves_by_key_via_ruleset_bestiary1_only ... ok
test ghoul_returns_none_for_ruleset_apg_and_acg ... ok
test ghoul_resolves_via_ruleset_bestiary1 ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd17_b_monster_stat_block -- --ignored 2>&1 | tail -10
running 1 test
test parses_real_cr_1_monster_records_from_b1_races_lst ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked --lib monster_stat_block 2>&1 | tail -10
running 6 tests
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 148 filtered out

$ cargo test --locked 2>&1 | grep -c "0 failed"
418
$ cargo test --locked 2>&1 | grep -c "FAILED"
0

$ cargo clippy --locked --tests -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
$ echo $?
0
```

Full suite: 0 regressions on any sibling row (the concurrent ACG
Warpriest cycle's uncommitted files in the shared working tree were left
untouched — only this cycle's own files were staged).

## Files touched

- `src/pcgen_import/lst_parser/monster_stat_block.rs` (NEW) — bare
  tab-delimited monster stat-block parser.
- `src/pcgen_import/lst_parser/mod.rs` (MODIFIED) — registers
  `pub mod monster_stat_block;`.
- `src/rules_core/rules_tables/mod.rs` (MODIFIED) — adds
  `RuleSetId::Bestiary1` + `pub mod beastiary1;`.
- `src/rules_core/rules_tables/beastiary1/mod.rs` (NEW) — book-level
  module, `MonsterId`, `MonsterStatBlock`, `monster_resolve`,
  `monster_key_resolve`.
- `src/rules_core/rules_tables/beastiary1/monster_subset_01.rs` (NEW) —
  corrected CR-1 roster (Ghoul, Gnoll, Goblin Dog, Lizardfolk, Wolf).
- `tests/sd17_b_monster_stat_block.rs` (NEW) — real-corpus-gated
  parser-gap widening test.
- `tests/sd22_beastiary1_subset_01_resolves.rs` (NEW) — acceptance test.

## Cycle metadata

- cycle_id: 2026-07-20T04:25:03Z
- duration: ~50 minutes
- bundle_criterion: criteria 14-17 (Epic 5, Bestiary 1 first monster-block subset)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst` (lines 200, 212, 213, 276, 414 — Ghoul/Gnoll/Goblin Dog/Lizardfolk/Wolf real CR-1 records; per `decisions.md §5`)
- RuleSetId: Bestiary1
- ingest_pipeline_version: 2 (per `./ingest.md §6`), extended this cycle with a new parser (§9.7 "when a content type doesn't fit the pipeline's class-shape" — a monster stat block needed its own bare-row parser, not a class-name allowlist widening)

## kanban
- card: see `progress.md`/`receipts.md` for the minted card id (Step 10b, this cycle)
