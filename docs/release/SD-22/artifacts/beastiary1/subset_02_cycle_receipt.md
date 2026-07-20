# Bestiary 1 subset 02 cycle receipt — 2026-07-20

## What this cycle landed

Epic 5's second monster-block subset. `beastiary1/mod.rs`'s parser gap
was already resolved by subset 01's cycle (`monster_stat_block.rs`); this
cycle needed no parser widening — every field subset 02's five monsters
use already falls inside that parser's existing recognition surface.
Ran in parallel with a sibling stream working Epic 4 criterion 13 (ACG
shared spell/equipment tables, `src/rules_core/rules_tables/acg/`); this
cycle's file-touch set (`rules_tables/beastiary1/`, `tests/sd22_beastiary1_*`,
`tests/sd17_b_monster_stat_block.rs`) is disjoint from `acg/` per
`loop-instruction.md`'s file-touch partition.

## Roster correction (discovered this cycle, before writing any GREEN code)

`corpus-source-inventory.md` §3.1's illustrative subset-02 sample list is
"Gnoll, Hobgoblin, Lizardfolk, Rat Swarm" (CR band "CR 1"). Verified each
name directly against the real `b1_races.lst` before transcribing
anything:

```
$ grep -n '^Gnoll\t' b1_races.lst        # line 212 — already used in subset 01
$ grep -n '^Lizardfolk\t' b1_races.lst   # line 276 — already used in subset 01
$ grep -n '^Hobgoblin\t' b1_races.lst    # 0 hits
$ grep -n 'Hobgoblin' b1_races_pc.lst
10:Hobgoblin.MOD	SOURCEPAGE:p.175
$ grep -n '^Rat Swarm' b1_races.lst
334:Rat Swarm ... CR:2 ... SOURCEPAGE:p.232
```

Three independent defects:
- **Gnoll** and **Lizardfolk** were already ingested in subset 01
  (`monster_subset_01.rs`) — shipping them again in subset 02 would
  duplicate an existing record, not add a new one.
- **Hobgoblin** has no standalone monster stat-block row in `b1_races.lst`
  at all — same shape as subset 01's Goblin/Kobold/Orc defect: it exists
  only as a `.MOD` override in the separate `b1_races_pc.lst` file,
  layered onto its playable-race base.
- **Rat Swarm** does exist as a real, standalone stat-block row
  (line 334) — but its real `CR:` token is `2`, not `1`. This one was
  initially assumed absent (a first-pass `grep -n '^Rat Swarm\t'`
  returned 0 hits because the row uses many literal spaces before its
  first tab, not `Rat Swarm\t` directly) until a real-corpus-gated test
  written this cycle caught the mistake — corrected before landing rather
  than left in the shipped doc comments (see "Self-correction mid-cycle"
  below).

Per this loop's established self-healing precedent (same shape as the
already-resolved Epic 3 Gunslinger/Magus, Epic 4 "Alchemist (ACG-side)",
and subset 01's own roster corrections), corrected the roster in-cycle:
enumerated every real CR:1 monster stat-block row in `b1_races.lst`
directly (bare tab-delimited rows carrying a literal `CR:1` token,
excluding `.MOD`/`.COPY=` rows), excluded the five names already used in
subset 01, and excluded parenthetical sub-variant names (e.g. "Ghoul
(Ghast)", "Frog (Giant)", "Skeletal Champion (Human)") the same way
subset 01 did. The remaining real, unambiguous, directly-transcribable
CR-1 monsters, alphabetical: **Darkmantle** (line 91), **Horse**
(line 235), **Hyena** (line 242), **Octopus** (line 314), **Spider
Swarm** (line 379).

## Self-correction mid-cycle

The first draft of this cycle's real-corpus-gated grounding test
(`tests/sd17_b_monster_stat_block.rs`) asserted Rat Swarm does not parse
at all. Running that test against the real corpus failed:
`assertion failed: records.iter().all(|r| r.name != "Rat Swarm")` — Rat
Swarm *does* parse as a real record, at CR 2. Rather than weaken the
assertion to make it pass, re-verified the real row directly (`sed -n
'334p' b1_races.lst`), confirmed `CR:2` is the real token, and corrected
every doc comment (`monster_subset_02.rs`, `beastiary1/mod.rs`,
`tests/sd22_beastiary1_subset_02_resolves.rs`, this file,
`corpus-source-inventory.md` §3.1) plus the grounding test itself to
state the accurate fact (Rat Swarm is real but CR 2, not CR 1) instead of
the inaccurate one (Rat Swarm doesn't exist). This is exactly the kind of
red-flag a real-corpus-gated test is supposed to catch before it ships in
a doc comment as an unverified claim.

## Field-coverage scope boundary

Mirrors `monster_subset_01.rs` and every SD-22 Epic 3/4 class chassis
module: only fields literally present as tokens on the real bare row are
transcribed (name, CR, size, walk speed, race type/subtype, source page,
natural attacks). AC, HP, and Fort/Ref/Will saves are PCGen-computed at
runtime from the `MONSTERCLASS:` hit-dice table and ability-score
modifiers, not literal row tokens — deferred to a future ingest slice,
same posture as subset 01.

## Red-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_02_resolves 2>&1 | tail -20
error[E0599]: no variant, associated function, or constant named `Darkmantle` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Horse` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Hyena` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `Octopus` found for enum `MonsterId` in the current scope
error[E0599]: no variant, associated function, or constant named `SpiderSwarm` found for enum `MonsterId` in the current scope
error: could not compile `codex` (test "sd22_beastiary1_subset_02_resolves") due to 10 previous errors
```

Failed for the intended reason: the acceptance test referenced
`MonsterId` variants (`Darkmantle`, `Horse`, `Hyena`, `Octopus`,
`SpiderSwarm`) and `monster_subset_02` functions that didn't exist yet.

## Green-phase evidence

```
$ cargo test --locked --test sd22_beastiary1_subset_02_resolves 2>&1 | tail -10
running 6 tests
test all_five_subset_02_monsters_resolve_via_ruleset_bestiary1 ... ok
test darkmantle_resolves_via_ruleset_bestiary1 ... ok
test octopus_has_both_natural_attacks ... ok
test octopus_returns_none_for_ruleset_apg_acg_crb ... ok
test subset_01_monsters_still_resolve_unchanged ... ok
test spider_swarm_resolves_by_key_via_ruleset_bestiary1_only ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd17_b_monster_stat_block -- --ignored 2>&1 | tail -10
running 2 tests
test parses_real_cr_1_monster_records_from_b1_races_lst ... ok
test parses_real_subset_02_cr_1_monster_records_from_b1_races_lst ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked 2>&1 | grep -c "test result: ok"
421
$ cargo test --locked 2>&1 | grep -c "FAILED"
0

$ cargo clippy --locked --tests -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
$ echo $?
0
```

Full suite: 421 `test result: ok` blocks, 0 `FAILED`, 0 regressions on any
sibling row (subset 01's own tests re-verified passing in the same run;
the concurrent Epic 4 criterion-13 sibling's uncommitted files in the
shared working tree were left untouched — only this cycle's own files
were staged).

## Files touched

- `src/rules_core/rules_tables/beastiary1/mod.rs` (MODIFIED) — adds
  `pub mod monster_subset_02;`, five new `MonsterId` variants, match arms
  in `monster_resolve`/`monster_key_resolve`, and a roster-correction doc
  comment.
- `src/rules_core/rules_tables/beastiary1/monster_subset_02.rs` (NEW) —
  corrected CR-1 roster (Darkmantle, Horse, Hyena, Octopus, Spider Swarm).
- `tests/sd17_b_monster_stat_block.rs` (MODIFIED) — adds a real-corpus-gated
  grounding test for subset 02's five monsters (no parser widening was
  needed; this proves the existing parser already covers them).
- `tests/sd22_beastiary1_subset_02_resolves.rs` (NEW) — acceptance test.
- `docs/release/SD-22/corpus-source-inventory.md` (MODIFIED) — corrects
  §3.1's subset 01 and subset 02 rows to the real, verified rosters.

## Cycle metadata

- cycle_id: 2026-07-20 (this cycle)
- bundle_criterion: criteria 14-17 (Epic 5, Bestiary 1 second monster-block subset)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst` (lines 91, 235, 242, 314, 379 — Darkmantle/Horse/Hyena/Octopus/Spider Swarm real CR-1 records; per `decisions.md §5`)
- RuleSetId: Bestiary1
- ingest_pipeline_version: 2 (per `./ingest.md §6`) — no parser widening needed this cycle (subset 02's fields all fall inside `monster_stat_block.rs`'s existing recognition surface)

## kanban
- card: see `progress.md`/`receipts.md` for the minted card id (Step 10b, this cycle)
