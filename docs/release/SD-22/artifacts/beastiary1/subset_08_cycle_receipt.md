# Bestiary 1 subset 08 cycle receipt — 2026-07-20T09:50:25Z

## Roster derivation

Before writing any RED test, independently re-enumerated every real,
non-`#`-commented, non-`.MOD`/`.COPY=` `CR:3` monster stat-block row in
`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`
directly against the live corpus file (not from any prior cycle's
summary). Confirmed 20 clean, standalone (non-parenthetical) CR:3
species names exist total — same count subset 07's cycle found:

```
Ankheg, Assassin Vine, Centaur, Cockatrice, Derro, Doppelganger, Dryad,
Ettercap, Gelatinous Cube, Hell Hound, Lion, Ogre, Pegasus, Rust
Monster, Shadow, Unicorn, Violet Fungus, Wasp Swarm, Wight, Yeth Hound
```

Subset 07 shipped the first five alphabetically (Ankheg, Assassin Vine,
Centaur, Cockatrice, Derro). This subset ships the next five
alphabetically: **Doppelganger** (`b1_races.lst:127`), **Dryad**
(`b1_races.lst:141`), **Ettercap** (`b1_races.lst:175`), **Gelatinous
Cube** (`b1_races.lst:189`), **Hell Hound** (`b1_races.lst:230`).
`Hell Hound (Nessian)` (`b1_races.lst:231`, CR 9) is a parenthetical,
higher-CR sub-variant and was excluded, same rule every prior subset
has used.

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_08_resolves 2>&1 | tail -40
error[E0599]: no variant, associated function, or constant named `HellHound` found for enum `MonsterId` in the current scope
   --> tests/sd22_beastiary1_subset_08_resolves.rs:115:43
    |
115 |     assert_eq!(monster_resolve(MonsterId::HellHound, RuleSetId::Apg), None);
    |                                           ^^^^^^^^^ variant, associated function, or constant not found in `MonsterId`
...
error[E0599]: no variant, associated function, or constant named `Doppelganger` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Dryad` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Ettercap` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `GelatinousCube` found for enum `MonsterId` in the current scope
error: could not compile `codex` (test "sd22_beastiary1_subset_08_resolves") due to 14 previous errors
```

Fails for the intended reason: the five new `MonsterId` variants
(`Doppelganger`, `Dryad`, `Ettercap`, `GelatinousCube`, `HellHound`)
and `monster_subset_08` module do not exist yet.

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_08_resolves 2>&1 | tail -20
running 6 tests
test all_five_subset_08_monsters_resolve_via_ruleset_bestiary1 ... ok
test doppelganger_resolves_via_ruleset_bestiary1 ... ok
test hell_hound_resolves_by_key_via_ruleset_bestiary1_only ... ok
test hell_hound_returns_none_for_ruleset_apg_acg_crb ... ok
test racesubtype_and_natural_attack_variety_across_subset_08 ... ok
test subset_01_through_07_monsters_still_resolve_unchanged ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked 2>&1 | grep "test result"
(428 "test result: ok" lines across the full suite; 0 failed anywhere —
 sibling-preservation held for all prior subsets 01-07, all Epic 3/4/6/8
 tests, and the rest of the pre-existing suite)

$ cargo clippy --locked --tests -- -D warnings 2>&1
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
(exit code 0; no warnings)
```

## Files touched

- `src/rules_core/rules_tables/beastiary1/monster_subset_08.rs` (NEW)
- `src/rules_core/rules_tables/beastiary1/mod.rs` (MODIFIED; `monster_subset_08` module registration, 5 new `MonsterId` variants, `monster_resolve`/`monster_key_resolve` match arms, doc comment)
- `tests/sd22_beastiary1_subset_08_resolves.rs` (NEW)
- `docs/release/SD-22/corpus-source-inventory.md` (MODIFIED; §3.1 subset 8 row added, replacing the `...` placeholder)

## Cycle metadata

- cycle_id: 2026-07-20T09:50:25Z
- bundle_criterion: criteria 14-17 (Bestiary 1 per-monster-block-subset cycle, subset 8 of a default 8-12)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst:127` (Doppelganger), `:141` (Dryad), `:175` (Ettercap), `:189` (Gelatinous Cube), `:230` (Hell Hound) — real corpus; `decisions.md §5`
- RuleSetId: Bestiary1
- ingest_pipeline_version: 2 (per `./ingest.md §6`)

## Closure-readiness assessment (subset count vs. default 8-12 target)

This cycle brings Epic 5 to **8 of a default 8-12 monster-block
subsets** (41 monsters total: 36 from subsets 01-07 + 5 this cycle).
`acceptance-and-verification.md` line 101 states criterion 15's
verification is "per-subset artifacts (**default 8-12**)" — 8 meets the
stated low end of that range. `epic-breakdown.md` and
`loop-instruction.md` both describe Epic 5 closure in terms of "every
APG/ACG/Bestiary 1 class/monster table ships" and the DM-toolkit
consumption criterion (17, already satisfied since subset 01), not a
hard minimum count beyond the "default 8-12" language — there is no
explicit statement anywhere in the read set that Epic 5 must reach the
*high* end (12) before being considered closed, only that 8-12 is the
default range. My assessment: **Epic 5 has now met the default range's
floor and is reasonably closeable at 8 subsets**, pending an explicit
operator/orchestrator call — recorded in `progress.md`'s cycle log for
the orchestrating session to review and make the final call (this
cycle does not unilaterally mark E5.14-17 "complete — Epic 5 fully
closed," only "criteria 14-17 hold; subset count now in-range").

## kanban
- card: see `docs/release/SD-22/receipts.md` cycle log entry for this cycle's card id (or "no card: <reason>" if `hermes` was unreachable)
