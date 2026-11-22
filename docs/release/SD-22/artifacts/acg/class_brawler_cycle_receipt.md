# Brawler cycle receipt — 2026-07-19T22:17:13Z

## Red-phase evidence

### Widening RED — `tests/sd17_b1_martial_class.rs::parses_real_brawler_record_from_acg_classes_lst`

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b1_martial_class -- --ignored parses_real_brawler
running 1 test
test parses_real_brawler_record_from_acg_classes_lst ... FAILED

failures:

---- parses_real_brawler_record_from_acg_classes_lst stdout ----

thread 'parses_real_brawler_record_from_acg_classes_lst' panicked at tests/sd17_b1_martial_class.rs:691:10:
Brawler should be recognized from the real acg_classes.lst once MARTIAL_CLASS_NAMES is widened to include it

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s
```

Failed for the intended reason: `Brawler` was not yet in `MARTIAL_CLASS_NAMES`, so the
real `CLASS:Brawler` line in `acg_classes.lst` was silently skipped (out-of-scope, no
diagnostic).

### Acceptance RED — `tests/sd22_acg_class_brawler_resolves.rs`

```
$ cargo test --locked --test sd22_acg_class_brawler_resolves
error[E0599]: no variant, associated function, or constant named `Brawler` found for enum `AcgClassId` in the current scope
  --> tests/sd22_acg_class_brawler_resolves.rs:33:49
   |
33 |     let row = class_chassis_resolve(AcgClassId::Brawler, 1, RuleSetId::Acg)
   |                                                 ^^^^^^^ variant, associated function, or constant not found in `AcgClassId`
(...4 more identical E0599s at lines 43, 54, 67, 76...)
error: could not compile `codex` (test "sd22_acg_class_brawler_resolves") due to 5 previous errors
```

Failed to compile for the intended reason: `AcgClassId::Brawler` did not exist yet.

## Green-phase evidence

### Widening + acceptance test, real-corpus-gated run

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b1_martial_class -- --include-ignored
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd22_acg_class_brawler_resolves -- --include-ignored
running 7 tests
test brawler_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test brawler_chassis_returns_none_for_ruleset_crb ... ok
test brawler_chassis_returns_none_for_ruleset_apg ... ok
test arcanist_and_bloodrager_chassis_still_resolve_after_brawler_lands ... ok
test brawler_level_20_chassis_resolves_via_ruleset_acg ... ok
test brawler_level_1_chassis_resolves_via_ruleset_acg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Full suite

```
$ cargo test --locked
408 "test result: ok" blocks across every suite; zero "N failed" with N > 0 anywhere in
the full output (grepped explicitly). Sibling-preservation holds — all six untouched APG
class-chassis suites, both untouched APG spell/equipment suites, and the two untouched
ACG suites (Arcanist, Bloodrager) all still pass unmodified.
```

### Clippy

```
$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 33.67s
(exit code 0, zero warnings/errors)
```

## Files touched

- `src/rules_core/rules_tables/acg/class_brawler.rs` (NEW)
- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `AcgClassId::Brawler` + match arm + `pub mod class_brawler;`)
- `src/pcgen_import/lst_parser/class.rs` (MODIFIED; `MARTIAL_CLASS_NAMES` widened by one — `Brawler`)
- `tests/sd22_acg_class_brawler_resolves.rs` (NEW)
- `tests/sd17_b1_martial_class.rs` (MODIFIED; real-corpus grounding test for the widening)
- `docs/release/SD-22/artifacts/acg/class_brawler_cycle_receipt.md` (NEW, this file)
- `docs/release/SD-22/progress.md` (MODIFIED; status matrix + cycle log)
- `docs/release/SD-22/receipts.md` (MODIFIED; receipt block appended)

## Source grounding

Real record verified directly against the corpus before any test was written (not
`corpus-source-inventory.md §2.1`'s non-authoritative "Content shape" prose):

`acg_classes.lst:84`:
```
CLASS:Brawler	HD:10		TYPE:Base.PC	MAXLEVEL:20	SOURCEPAGE:p.23	...
BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE|PREVAREQ:UseAlternateBABProgression,0
BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2|PREVAREQ:UseAlternateSaveProgression,0
BONUS:SAVE|BASE.Will|CL/3
```

- Full BAB (no fractional divisor) — same posture as Bloodrager.
- Good Fortitude **and** Reflex (one combined token, `/2+2`).
- Poor Will (`CL/3`, i.e. `level/3`).
- `MAXLEVEL:20`.
- No `SPELLSTAT:` token anywhere in the Brawler block — non-caster, confirming Brawler
  belongs in `lst_parser::class`'s `MARTIAL_CLASS_NAMES` allowlist (mirrors Cavalier),
  not `lst_parser::spellcasting_class`'s allowlist (which the prior two ACG classes,
  Arcanist and Bloodrager, both used).

## Cycle metadata

- cycle_id: 2026-07-19T22:17:13Z
- bundle_criterion: criteria 10-12 (ACG per-class cycles, class 3 of the corrected
  10-class roster)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:CLASS:Brawler`
  (real corpus; `decisions.md §5`)
- RuleSetId: Acg

## kanban

- card: see `docs/release/SD-22/progress.md` cycle log / `receipts.md` for the minted
  card id (hermes attempted per Step 10b; recorded there, not duplicated here).
