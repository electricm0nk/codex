# Epic 3 — Criteria 6-8 — APG Alchemist class chassis (cycle 1 of 8)

- cycle_id: 2026-07-19T14:00:00Z
- criterion_section: §1.1 Epic 3 — APG content-source ingest (criteria 6, 7, 8)
- row_or_kind: ingest:apg_class
- branch_tip_before: e2d7194
- rule_set_used: Apg
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:11` — `CLASS:Alchemist`

## Why this criterion, this cycle

The `E3.6-9` blocker (`progress.md`'s cycle log, `cycle-2026-07-19T13:xx:xxZ`)
was narrowed by an operator-side session that widened
`spellcasting_class.rs`'s `SPELLCASTING_CLASS_NAMES` allowlist to recognize
`CLASS:Alchemist`, added a real-corpus-gated parser test, and explicitly
noted "Epic 3's Alchemist criterion (E3.6-9) is not yet complete... the
actual ingest cycle... is still open work for the next loop firing." This
firing picks up exactly that open work: populate
`rules_tables/apg/mod.rs`, register `RuleSetId::Apg`, and land the
Alchemist class chassis table + cross-book invariant test.

Scope for this cycle is bounded to the same shape
`rules_tables/crb/class_tables.rs` already established: the BAB/save
chassis only. Named per-level features (Bombs, Discoveries, Mutagen, Brew
Potion, ...) require walking `apg_abilities_class.lst`'s per-level feature
blocks in a dedicated future ingest slice — transcribing that content from
memory here would be exactly the fabrication risk `AGENTS.md` and
`class_tables.rs`'s own doc comment rule out. The chassis data below is
read directly off the real record's `BONUS:COMBAT`/`BONUS:SAVE` formula
tokens, not from memory.

## Red-phase evidence

Added `tests/sd22_apg_class_alchemist_resolves.rs`, asserting:
1. `class_chassis_resolve(ApgClassId::Alchemist, 1, RuleSetId::Apg)` and
   `..., 20, RuleSetId::Apg)` resolve to the expected BAB/save cells.
2. `class_chassis_resolve(..., 21, RuleSetId::Apg)` returns `None` (the
   real record's `MAXLEVEL:20` bound).
3. `class_chassis_resolve(..., RuleSetId::Crb)` returns `None` — the Epic
   3 cross-book invariant (`corpus-source-inventory.md` §1.3).
4. A real-corpus-gated (`PCGEN_CORPUS_ROOT`) grounding test that re-parses
   the real `CLASS:Alchemist` line and asserts the exact `BASEAB`/`SAVE`
   bonus-formula tokens the hand-transcribed constants below are derived
   from.

Ran against the unchanged tree (`rules_tables::apg` and `RuleSetId::Apg`
did not yet exist):

```
$ cargo test --locked --test sd22_apg_class_alchemist_resolves
error[E0432]: unresolved import `codex::rules_core::rules_tables::apg`
  --> tests/sd22_apg_class_alchemist_resolves.rs:23:38
   |
23 | use codex::rules_core::rules_tables::apg::{ApgClassId, class_chassis_resolve};
   |                                      ^^^ could not find `apg` in `rules_tables`

error[E0599]: no variant or associated item named `Apg` found for enum `RuleSetId` in the current scope
  --> tests/sd22_apg_class_alchemist_resolves.rs:27:74
...
error: could not compile `codex` (test "sd22_apg_class_alchemist_resolves") due to 4 previous errors
```

Failed for the intended reason: the production surface this test targets
did not exist yet.

## Green-phase evidence

Added:
- `src/rules_core/rules_tables/apg/mod.rs` — `ApgClassId` enum,
  `class_chassis_resolve` (returns `None` for any `RuleSetId` other than
  `Apg`, and for any level beyond the class's real `MAXLEVEL`).
- `src/rules_core/rules_tables/apg/class_alchemist.rs` — `class_table()`,
  three-quarter BAB (`level*3/4`), good Fortitude/Reflex
  (`level/2+2`), poor Will (`level/3`), 20-level ceiling — read directly
  off `apg_classes.lst:11`'s `BONUS:COMBAT|BASEAB`,
  `BONUS:SAVE|BASE.Fortitude,BASE.Reflex`, `BONUS:SAVE|BASE.Will`, and
  `MAXLEVEL:20` tokens.
- `src/rules_core/rules_tables/mod.rs` — added `RuleSetId::Apg` and
  `pub mod apg;`.

```
$ cargo test --locked --test sd22_apg_class_alchemist_resolves
running 5 tests
test alchemist_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test alchemist_chassis_returns_none_for_ruleset_crb ... ok
test alchemist_level_1_chassis_resolves_via_ruleset_apg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ignored, requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data
test alchemist_level_20_chassis_resolves_via_ruleset_apg ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd22_apg_class_alchemist_resolves -- --ignored
running 1 test
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

$ cargo test --locked 2>&1 | grep -E "^test result:"
(every suite: 0 failed; new sd22_apg_class_alchemist_resolves suite: 4 passed, 1 ignored)

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.04s
(clean, zero warnings)
```

No sibling regression: every pre-existing test suite still reports `0
failed` after this change (grep across the full `cargo test --locked`
output confirms no `N failed` line with `N > 0` anywhere).

## Files touched

- `src/rules_core/rules_tables/mod.rs` — modified (added `RuleSetId::Apg`, `pub mod apg;`)
- `src/rules_core/rules_tables/apg/mod.rs` — added
- `src/rules_core/rules_tables/apg/class_alchemist.rs` — added
- `tests/sd22_apg_class_alchemist_resolves.rs` — added

## Cycle metadata

- cycle_id: 2026-07-19T14:00:00Z
- duration: ~40 minutes
- bundle_criterion: criterion-6, criterion-7, criterion-8 (criterion-9's
  spell/equipment resolution is explicitly out of scope this cycle — no
  APG spell/equipment tables exist yet; that lands with `apg/spell_list.rs`
  / `apg/equipment_tables.rs` in a later cycle per the file-touch
  partition)
- upstream reference: `apg_classes.lst:11` (`CLASS:Alchemist`), real PCGen
  corpus checkout at `/home/user/pcgen` (`https://github.com/PCGen/pcgen`)
- RuleSetId: Apg

## kanban

- card: no card: hermes unavailable in this cloud sandbox; this receipt +
  `docs/release/SD-22/receipts.md` are the durability backbone per Step
  10a/10b
- audit_comment: n/a
