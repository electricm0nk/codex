# ACG shared spell list cycle receipt — 2026-07-20T05:15:52Z

## Red-phase evidence

```
$ cargo test --locked --test sd22_acg_spell_list_resolves 2>&1 | tail -20
   Compiling codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
error[E0432]: unresolved import `codex::rules_core::rules_tables::acg::spell_list`
  --> tests/sd22_acg_spell_list_resolves.rs:26:43
   |
26 | use codex::rules_core::rules_tables::acg::spell_list::{Pf1SchoolId, spell_resolve};
   |                                           ^^^^^^^^^^ could not find `spell_list` in `acg`

error: could not compile `codex` (test "sd22_acg_spell_list_resolves") due to 1 previous error
```

Failed for the intended reason: `rules_tables::acg::spell_list` didn't exist yet.

## Green-phase evidence

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd22_acg_spell_list_resolves -- --include-ignored
running 8 tests
test air_geyser_resolves_via_ruleset_acg ... ok
test anti_incorporeal_shell_resolves_via_ruleset_acg ... ok
test beastspeak_resolves_via_ruleset_acg ... ok
test blade_lash_returns_none_for_ruleset_apg ... ok
test blade_lash_returns_none_for_ruleset_crb ... ok
test blade_lash_resolves_via_ruleset_acg ... ok
test unknown_spell_key_resolves_to_none ... ok
test hand_transcribed_spell_sample_matches_the_real_lst_lines ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test --locked 2>&1 | grep -c "test result:"
421
$ cargo test --locked 2>&1 | grep "test result:" | grep -v "0 failed"
(no output — every one of the 421 test-result blocks shows 0 failed;
 sibling-preservation holds, including all ten untouched ACG class-chassis
 suites, all six APG class-chassis suites, both APG spell/equipment
 suites, the Bestiary 1 subset-01 suite, and every pre-existing
 SD-17/SD-19/SD-21 suite)
```

```
$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 20.26s
```
Clean — zero warnings, exit code 0.

## Files touched

- `src/rules_core/rules_tables/acg/spell_list.rs` (NEW)
- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `pub mod spell_list;` + `pub mod equipment_tables;`)
- `tests/sd22_acg_spell_list_resolves.rs` (NEW)
- (this cycle also lands `acg/equipment_tables.rs` + its own test in the same commit — see the sibling `equipment_tables_cycle_receipt.md`)

## Cycle metadata

- cycle_id: 2026-07-20T05:15:52Z
- bundle_criterion: criterion 13 (ACG spell and equipment resolution)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_spells.lst` — real, active (non-`.MOD`, non-commented) records in the "New Spells" block: `Blade Lash` (line 27, `CLASSES:Bloodrager,Magus=1`), `Air Geyser` (line 14, `CLASSES:Bloodrager,...=3|Shaman=4`), `Beastspeak` (line 25, `CLASSES:Druid,Shaman,Witch=2`), `Anti-Incorporeal Shell` (line 21, `CLASSES:Cleric,Shaman,Witch=4`)
- RuleSetId: Acg
- ingest_pipeline_version: 2 (per `ingest.md §6`; direct real-corpus transcription, no corpus-loader abstraction)

## Scope note

This is a **bootstrap/representative sample**, not exhaustive coverage —
same posture `rules_tables::apg::spell_list`'s own doc comment
establishes.

**Arcanist, Hunter, Investigator, Skald, and Warpriest have no active,
full-definition spell record in the real corpus.** Confirmed by direct
grep of the whole `advanced_class_guide/` tree and the wider
`roleplaying_game/` tree for `CLASSES:.*<ClassName>` — zero hits for
Arcanist, Hunter, Skald, and Warpriest anywhere; Investigator has exactly
one hit, `Bomber's Eye.MOD CLASSES:Investigator=1` (line 797), which is a
`.MOD` cross-reference onto APG's own already-ingested `Bomber's Eye`
spell (`apg_spells.lst:44`), not a full ACG-side spell definition — same
non-`.MOD` exclusion rule `apg/spell_list.rs` already established for its
own Summoner gap. This is a real gap in the source data at this
bootstrap's scope (those five classes draw from other books' existing
spell lists rather than an ACG-specific block), not an omission in this
transcription; left open for a future cycle if a base-spell-list
association approach is chosen instead of requiring a dedicated ACG
record.

## kanban

- card: see `receipts.md` / `progress.md` for this cycle's minted card id
