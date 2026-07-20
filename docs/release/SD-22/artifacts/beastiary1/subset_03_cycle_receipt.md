# Bestiary 1 subset 03 cycle receipt — 2026-07-20T06:19:56Z

## CR-band move (not a roster correction of an existing row)

`corpus-source-inventory.md` §3.1 had no illustrative sample row for
subset 3 — only a placeholder `...` row. Before writing any GREEN code,
this cycle enumerated every real, non-`#`-commented, non-`.MOD`/`.COPY=`
CR:1 monster stat-block row in `b1_races.lst` directly, excluding
parenthetical sub-variant names (e.g. "Ghoul (Ghast)", "Ant (Worker)",
"Frog (Giant)") the same way subset 01 and subset 02 both already
established (those name a variant of a broader creature rather than a
clean, unambiguous base species). Only 12 real CR:1 monster names exist
in the whole file under that rule:

- subset 01 (already used): Ghoul, Gnoll, Goblin Dog, Lizardfolk, Wolf
- subset 02 (already used): Darkmantle, Horse, Hyena, Octopus, Spider Swarm
- unused: **Squid** (line 380), **Troglodyte** (line 390) — two monsters,
  not the five a subset needs

CR 1 is exhausted for a clean five-monster subset (`Squid` and
`Troglodyte` remain real and available for a future small/mixed subset,
but this cycle did not use them). Per the loop-instruction's explicit
"move to the next CR band if CR 1 is exhausted — verify directly against
the real corpus, don't assume" guidance, this cycle moved to CR 2 and
enumerated every real, non-commented, non-parenthetical CR:2 monster
stat-block row, picking the first five alphabetically:
**Bat Swarm** (line 41), **Boar** (line 49), **Boggard** (line 51),
**Bugbear** (line 52), **Cave Fisher** (line 59).

`corpus-source-inventory.md` §3.1 gets a new row 3 in this cycle's
commit (no correction needed — no prior row existed to correct).

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_03_resolves 2>&1 | tail -40
error[E0599]: no variant, associated function, or constant named `BatSwarm` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Boar` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Boggard` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Bugbear` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `CaveFisher` found for enum `MonsterId` in the current scope
... (11 total E0599 errors, one per call site referencing the not-yet-existing MonsterId variants)
error: could not compile `codex` (test "sd22_beastiary1_subset_03_resolves") due to 11 previous errors
```

Failed for the intended reason: the five new `MonsterId` variants and
`monster_subset_03` module did not exist yet.

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_03_resolves 2>&1 | tail -15
running 6 tests
test all_five_subset_03_monsters_resolve_via_ruleset_bestiary1 ... ok
test bat_swarm_resolves_via_ruleset_bestiary1 ... ok
test boar_and_bugbear_have_no_natural_attacks ... ok
test cave_fisher_resolves_by_key_via_ruleset_bestiary1_only ... ok
test boggard_returns_none_for_ruleset_apg_acg_crb ... ok
test subset_01_and_02_monsters_still_resolve_unchanged ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b_monster_stat_block -- --ignored 2>&1 | tail -10
running 3 tests
test parses_real_subset_03_cr_2_monster_records_from_b1_races_lst ... ok
test parses_real_subset_02_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_cr_1_monster_records_from_b1_races_lst ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked 2>&1 | grep -c "test result: ok"
423
$ cargo test --locked 2>&1 | grep "test result:" | grep -v "0 failed"
(no output — zero failures anywhere; sibling-preservation holds, including
the concurrently-landing Epic 6 criterion-21 happy-path integration test
which reads subsets 01/02 unchanged)

$ cargo clippy --locked --tests -- -D warnings 2>&1 | tail -5
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 33.67s
(clean, exit code 0)
```

## Files touched

- `src/rules_core/rules_tables/beastiary1/monster_subset_03.rs` (NEW)
- `src/rules_core/rules_tables/beastiary1/mod.rs` (MODIFIED; additive only
  — `pub mod monster_subset_03;`, 5 new `MonsterId` variants, 5 new match
  arms in `monster_resolve`/`monster_key_resolve`, module doc-comment note)
- `tests/sd22_beastiary1_subset_03_resolves.rs` (NEW)
- `tests/sd17_b_monster_stat_block.rs` (MODIFIED; real-corpus grounding
  test for subset 03, additive)
- `docs/release/SD-22/corpus-source-inventory.md` (MODIFIED; new §3.1 row
  3 for subset 03 — no prior row existed to correct)

## Cycle metadata

- cycle_id: 2026-07-20T06:19:56Z
- bundle_criterion: criteria 14-17 (Bestiary 1 monster-block subsets,
  re-verified against a third subset)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst:41` (Bat Swarm), `:49` (Boar), `:51` (Boggard), `:52` (Bugbear), `:59` (Cave Fisher)
- RuleSetId: Bestiary1

## Parallel-cycle coordination note

Ran alongside a sibling stream working Epic 6 criterion 21
(`tests/sd22_dm_toolkit_happy_path_integration.rs`), which reads from but
does not modify `beastiary1/`. This cycle's file-touch set
(`monster_subset_03.rs`, its test) is additive-only; the `mod.rs` diff is
purely additive (new `pub mod` line, new enum variants, new match arms) —
no existing lines were changed. All RED/GREEN/verification work was done
before touching `progress.md`/`receipts.md`, per the coordination
protocol.

## kanban

- card: see `docs/release/SD-22/progress.md` / `receipts.md` for the
  minted card ID (Step 10b runs after this receipt is written).
