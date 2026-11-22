# ACG shared equipment tables cycle receipt — 2026-07-20T05:15:52Z

## Red-phase evidence

```
$ cargo test --locked --test sd22_acg_equipment_resolves 2>&1 | tail -20
   Compiling codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
error[E0432]: unresolved import `codex::rules_core::rules_tables::acg::equipment_tables`
  --> tests/sd22_acg_equipment_resolves.rs:19:43
   |
19 | use codex::rules_core::rules_tables::acg::equipment_tables::{EquipmentCategory, equipment_resolve};
   |                                           ^^^^^^^^^^^^^^^^ could not find `equipment_tables` in `acg`

error: could not compile `codex` (test "sd22_acg_equipment_resolves") due to 1 previous error
```

Failed for the intended reason: `rules_tables::acg::equipment_tables` didn't exist yet.

## Green-phase evidence

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd22_acg_equipment_resolves -- --include-ignored
running 7 tests
test headsmans_blade_resolves_via_ruleset_acg ... ok
test marlinspike_resolves_via_ruleset_acg ... ok
test marlinspike_returns_none_for_ruleset_apg ... ok
test marlinspike_returns_none_for_ruleset_crb ... ok
test hand_transcribed_equipment_sample_matches_the_real_lst_lines ... ok
test ring_of_eloquence_resolves_via_ruleset_acg ... ok
test unknown_equipment_key_resolves_to_none ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
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

- `src/rules_core/rules_tables/acg/equipment_tables.rs` (NEW)
- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `pub mod spell_list;` + `pub mod equipment_tables;`)
- `tests/sd22_acg_equipment_resolves.rs` (NEW)
- (this cycle also lands `acg/spell_list.rs` + its own test in the same commit — see the sibling `spell_list_cycle_receipt.md`)

## Cycle metadata

- cycle_id: 2026-07-20T05:15:52Z
- bundle_criterion: criterion 13 (ACG spell and equipment resolution)
- corpus_input_path: real, verbatim `COST:` records from the single `acg_equip.lst` file — `Marlinspike` (line 179, `TYPE:Goods.Tools`, `COST:0.8`), `Headsman's Blade` (line 262, `TYPE:Weapon...`, `COST:50`), `Ring of Eloquence` (line 271, `TYPE:Magic.Ring`, `COST:3500`)
- RuleSetId: Acg
- ingest_pipeline_version: 2 (per `ingest.md §6`; direct real-corpus transcription, no corpus-loader abstraction)

## Scope note

This is a **bootstrap/representative sample** (one item per category),
not exhaustive coverage — same posture `rules_tables::apg::equipment_tables`'s
own doc comment establishes. Unlike APG, which splits equipment across
three separate `.lst` files (general/arms_armor/magic_items), ACG carries
all equipment in a single `acg_equip.lst` file disambiguated by the
`TYPE:` token (`Goods.*` → General, `Weapon.*`/`Armor.*` → ArmsArmor,
`Magic.*` → MagicItems) — confirmed by direct inspection of the file's
distinct top-level `TYPE:` categories before picking the three
representative rows.

## kanban

- card: see `receipts.md` / `progress.md` for this cycle's minted card id
