# DM-toolkit happy-path integration test cycle receipt — 2026-07-20T06:14:48Z

## Red-phase evidence

Read first (per Step 4 / `epic-breakdown.md`'s red-green TDD mandate):
`corpus-source-inventory.md` §4 (DM Toolkit routing table, specifically the
"Happy-path integration" row — required corpus input "One ingested
`PartySnapshot` + one ingested `MonsterRef` from Epic 3+4+5's first
cycles"), `epic-breakdown.md` criterion 21's exact wording, `ingest.md` §4
("Epic 6 — DM Toolkit happy-path integration"), and
`risks-and-open-questions.md`'s self-healable/non-self-healable rows.
Confirmed against the live `progress.md` that criterion 21 was
next-eligible: Epic 3 (APG), Epic 4 (ACG), and Epic 5 (Bestiary 1, two
subsets) were all already `complete` per the status matrix, satisfying
`loop-instruction.md` Step 1's "Epic 6 cycles after Epic 3+4+5" gate.

RED test location: `tests/sd22_dm_toolkit_happy_path_integration.rs` (new
file — the last remaining Epic 6 cycle's own reserved test-fixture path per
`loop-instruction.md`'s file-touch partition).

RED was captured by running the test command against the target before the
file existed:

```
$ cargo test --locked --test sd22_dm_toolkit_happy_path_integration 2>&1 | head -10
error: no test target named `sd22_dm_toolkit_happy_path_integration` in default-run packages
help: available test targets:
    character_hub_list_saved_characters
    character_input_record
    ...
```

Failed for the intended reason: the test file/target genuinely did not
exist yet (a target-resolution error, not a compile error inside an
existing file), confirming this is a from-scratch RED, not the "un-intended
reason" Bucket-B shortfall shape (`loop-instruction.md` Step 4.2).

## Green-phase evidence

Wrote `tests/sd22_dm_toolkit_happy_path_integration.rs` with two tests:

1. `happy_path_1_level_1_pc_vs_ingested_ghoul_is_medium_per_grounded_pf1_math`
   — 1 level-1 PC vs. the real ingested Ghoul (Epic 5 subset 01,
   `challenge_rating: 1.0`, `b1_races.lst:200`), resolved via
   `beastiary1::monster_resolve(MonsterId::Ghoul, RuleSetId::Bestiary1)`.
   Also asserts the cross-book invariant that Ghoul returns `None` for
   `RuleSetId::Crb`/`Apg`/`Acg`. Asserts the canonical grounded result: APL
   1, EL 1, `Difficulty::Medium` — identical math to
   `corpus-source-inventory.md` §4.1 case 5 and
   `tests/sd22_dm_toolkit_deterministic.rs`'s
   `encounters_1_level_1_pc_vs_1_cr_1_monster_returns_valid_difficulty`, now
   fed a real ingested monster instead of a synthetic `MonsterRef::new(1.0)`
   literal.
2. `happy_path_4_level_3_pcs_vs_ingested_darkmantle_is_easy_per_grounded_pf1_math`
   — 4 level-3 PCs (§4.1 case 1's party shape) vs. the real ingested
   Darkmantle (Epic 5 subset 02, `challenge_rating: 1.0`, `b1_races.lst:91`).
   Asserts APL 3, EL 1, `Difficulty::Easy` — a second, independent
   ingested-monster breadth check proving the integration point isn't
   accidentally coupled to one specific Epic 5 subset.

```
$ cargo test --locked --test sd22_dm_toolkit_happy_path_integration 2>&1 | tail -10
running 2 tests
test happy_path_1_level_1_pc_vs_ingested_ghoul_is_medium_per_grounded_pf1_math ... ok
test happy_path_4_level_3_pcs_vs_ingested_darkmantle_is_easy_per_grounded_pf1_math ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Full suite:

```
$ cargo test --locked 2>&1 | grep -c "test result: ok"
422
$ cargo test --locked 2>&1 | grep -E "FAILED|error\[|[1-9][0-9]* failed"
(no output — zero failures across all 422 `test result:` blocks; every
pre-existing suite is unaffected, including both Bestiary 1 subset suites,
all six APG and ten ACG class-chassis suites, and criteria 18-20's own
`encounters`/`party_cr`/deterministic suites)
```

```
$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
```

(Clean — no findings.)

## Investigated integration gap (found none requiring production code)

`loop-instruction.md`'s brief anticipated a possible "type mismatch between
`MonsterRef` as defined in `encounters.rs` vs. what
`beastiary1/monster_subset_01.rs` produces." Checked directly:
`beastiary1::MonsterStatBlock` (name, CR, size, speed, race type/subtype,
source page, natural attacks — the full Epic 5 stat-block shape) and
`encounters::MonsterRef` (just `challenge_rating: f32` — the minimal shape
`Encounter::new`'s grounded formula needs, per that module's own doc
comment, "Bestiary 1's own richer `MonsterRef` shape... is a distinct,
not-yet-landed type... a later cycle (criterion 21)... is where the two get
reconciled") are indeed two distinct types, as anticipated. But the
reconciliation is a direct, lossless field read —
`MonsterRef::new(stat_block.challenge_rating)` — because
`MonsterStatBlock::challenge_rating` is already a public `f32` field and
`MonsterRef::new` already takes a public `f32` argument. No `From` impl, no
schema change, no new production code was needed to bridge them; this
cycle ships test-only changes.

## Files touched

- `tests/sd22_dm_toolkit_happy_path_integration.rs` — NEW. Two tests per
  above.

## Cycle metadata

- cycle_id: 2026-07-20T06:14:48Z
- duration: ~40 minutes (includes reading all required-reading docs plus
  the existing `encounters.rs`/`party_cr.rs`/`beastiary1/mod.rs` production
  code before writing the test)
- bundle_criterion: criterion-21 (Epic 6's fourth and final criterion; Epic
  6 now fully complete)
- corpus_input_path: N/A for this cycle directly (no new `.lst` parsing) —
  consumes the already-ingested records at
  `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst:200` (Ghoul,
  Epic 5 subset 01) and `b1_races.lst:91` (Darkmantle, Epic 5 subset 02),
  both already transcribed and cited in
  `src/rules_core/rules_tables/beastiary1/monster_subset_01.rs` and
  `monster_subset_02.rs`.
- RuleSetId: Bestiary1 (the monster half); the party half has no
  `RuleSetId` (DM Toolkit's `CharacterSnapshot` is not book content).
- ingest_pipeline_version: N/A (not an ingest cycle — a consuming
  integration test per `ingest.md` §4)

## kanban

- card: see `docs/release/SD-22/progress.md`'s cycle log for this cycle;
  backfilled once the real card ID / commit SHA are known post-commit (same
  pattern as prior cycles' `hermes kanban` mint + backfill).
- audit_comment: n/a
