# APG shared spell list cycle receipt — 2026-07-19T19:51:46Z

## Red-phase evidence

```
$ cargo test --locked --test sd22_apg_spell_list_resolves 2>&1 | tail -20
   Compiling codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
error[E0432]: unresolved import `codex::rules_core::rules_tables::apg::spell_list`
  --> tests/sd22_apg_spell_list_resolves.rs:20:43
   |
20 | use codex::rules_core::rules_tables::apg::spell_list::{Pf1SchoolId, spell_resolve};
   |                                           ^^^^^^^^^^ could not find `spell_list` in `apg`

error: could not compile `codex` (test "sd22_apg_spell_list_resolves") due to 1 previous error
```

Failed for the intended reason: `rules_tables::apg::spell_list` didn't exist yet.

## Green-phase evidence

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd22_apg_spell_list_resolves -- --include-ignored
running 7 tests
test bombers_eye_resolves_via_ruleset_apg ... ok
test bombers_eye_returns_none_for_ruleset_crb ... ok
test borrow_fortune_resolves_via_ruleset_apg ... ok
test burst_bonds_resolves_via_ruleset_apg ... ok
test ill_omen_resolves_via_ruleset_apg ... ok
test unknown_spell_key_resolves_to_none ... ok
test hand_transcribed_spell_sample_matches_the_real_lst_lines ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test --locked 2>&1 | grep -E "test result|FAILED"
(every suite: "test result: ok" — 0 "FAILED" anywhere in the full run output;
 sibling-preservation holds, including all six untouched APG class-chassis
 suites and every pre-existing SD-17/SD-19/SD-21 suite)
```

```
$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 31.23s
```
Clean — zero warnings.

## Files touched

- `src/rules_core/rules_tables/apg/spell_list.rs` (NEW)
- `src/rules_core/rules_tables/apg/mod.rs` (MODIFIED; `pub mod spell_list;` + `pub mod equipment_tables;`)
- `tests/sd22_apg_spell_list_resolves.rs` (NEW)
- (this cycle also lands `apg/equipment_tables.rs` + its own test in the same commit — see the sibling `equipment_tables_cycle_receipt.md`)

## Cycle metadata

- cycle_id: 2026-07-19T19:51:46Z
- bundle_criterion: criterion 9 (APG spell and equipment resolution)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_spells.lst` — real, active (non-`.MOD`, non-commented) records: `Bomber's Eye` (line 44, `CLASSES:Alchemist=1`), `Burst Bonds` (line 53, `CLASSES:Inquisitor=1`), `Borrow Fortune` (line 277, `CLASSES:Oracle=3`), `Ill Omen` (line 150, `CLASSES:Witch=1`)
- RuleSetId: Apg
- ingest_pipeline_version: 2 (per `ingest.md §6`; direct real-corpus transcription, no corpus-loader abstraction)

## Scope note

This is a **bootstrap/representative sample**, not exhaustive coverage —
same posture `rules_tables::crb::equipment_tables`'s own doc comment
establishes ("one representative item per category... exhaustive
per-category coverage is the loop's job, one category per cycle").

**Summoner has no active spell record in the real corpus.** The
"Summoner Spells - APG" block (`apg_spells.lst:471` onward) is entirely
`#`-commented out in the real corpus (every line prefixed
`#Acid Pit.MOD`, `#Ant Haul.MOD`, ...) — confirmed by direct grep, not
assumed. This is a real gap in the source data, not an omission in this
transcription; left open for a future cycle. Cavalier casts no spells
(no `SPELLSTAT:` token on the real `CLASS:Cavalier` record, confirmed in
the Cavalier cycle), so it has no row here by design, not omission.

## kanban

- card: no card (hermes unavailable in this session; see `receipts.md` / `progress.md`)
