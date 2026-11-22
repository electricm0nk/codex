# APG shared equipment tables cycle receipt — 2026-07-19T19:51:46Z

## Red-phase evidence

```
$ cargo test --locked --test sd22_apg_equipment_resolves 2>&1 | tail -20
   Compiling codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
error[E0432]: unresolved import `codex::rules_core::rules_tables::apg::equipment_tables`
  --> tests/sd22_apg_equipment_resolves.rs:22:43
   |
22 | use codex::rules_core::rules_tables::apg::equipment_tables::{EquipmentCategory, equipment_resolve};
   |                                           ^^^^^^^^^^^^^^^^ could not find `equipment_tables` in `apg`

error: could not compile `codex` (test "sd22_apg_equipment_resolves") due to 1 previous error
```

Failed for the intended reason: `rules_tables::apg::equipment_tables` didn't exist yet.

## Green-phase evidence

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd22_apg_equipment_resolves -- --include-ignored
running 6 tests
test blunt_arrow_resolves_via_ruleset_apg ... ok
test iron_spike_resolves_via_ruleset_apg ... ok
test iron_spike_returns_none_for_ruleset_crb ... ok
test knucklebone_of_fickle_fortune_resolves_via_ruleset_apg ... ok
test hand_transcribed_equipment_sample_matches_the_real_lst_lines ... ok
test unknown_equipment_key_resolves_to_none ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
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

- `src/rules_core/rules_tables/apg/equipment_tables.rs` (NEW)
- `src/rules_core/rules_tables/apg/mod.rs` (MODIFIED; `pub mod spell_list;` + `pub mod equipment_tables;`)
- `tests/sd22_apg_equipment_resolves.rs` (NEW)
- (this cycle also lands `apg/spell_list.rs` + its own test in the same commit — see the sibling `spell_list_cycle_receipt.md`)

## Cycle metadata

- cycle_id: 2026-07-19T19:51:46Z
- bundle_criterion: criterion 9 (APG spell and equipment resolution)
- corpus_input_path: real, verbatim `COST:`/`WT:` records — `apg_equip_general.lst` (`Iron Spike`, `COST:0.05 WT:1`), `apg_equip_arms_armor.lst` (`Arrow (Blunt)`, `COST:0.1 WT:0.15`), `apg_equip_magic_items.lst` (`Knucklebone of Fickle Fortune`, `COST:0 WT:0.01`)
- RuleSetId: Apg
- ingest_pipeline_version: 2 (per `ingest.md §6`; direct real-corpus transcription, no corpus-loader abstraction)

## Scope note

This is a **bootstrap/representative sample** (one item per
`apg_equip_*.lst` file), not exhaustive coverage — same posture
`rules_tables::crb::equipment_tables`'s own doc comment establishes.

**Alchemist bombs are not equipment-table records.** Bombs are a `Su`
class feature computed by formula from class level and Int modifier, not
a purchasable item — there is no `Bomb`/`Acid Bomb` record in any
`apg_equip_*.lst` file (confirmed by direct inspection of all three
files: `apg_equip_general.lst`, `apg_equip_arms_armor.lst`,
`apg_equip_magic_items.lst`). `corpus-source-inventory.md §1.3`'s
illustrative `apg:alchemist:bomb:acid` cross-book-invariant example is
not authoritative per that file's own corrective banner; this cycle does
not add a fabricated bomb-item row to satisfy it — doing so would be
exactly the fabrication risk `AGENTS.md` and the CRB precedent rule out.

## kanban

- card: no card (hermes unavailable in this session; see `receipts.md` / `progress.md`)
