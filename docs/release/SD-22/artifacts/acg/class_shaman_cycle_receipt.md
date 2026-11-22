# Shaman cycle receipt — 2026-07-20T01:17:01Z

## Red-phase evidence

### Widening RED — `tests/sd17_b_spellcasting_class.rs::parses_real_shaman_record_from_acg_classes_lst`

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b_spellcasting_class parses_real_shaman_record_from_acg_classes_lst -- --include-ignored
running 1 test
test parses_real_shaman_record_from_acg_classes_lst ... FAILED

failures:

---- parses_real_shaman_record_from_acg_classes_lst stdout ----

thread 'parses_real_shaman_record_from_acg_classes_lst' panicked at tests/sd17_b_spellcasting_class.rs:988:10:
Shaman should be recognized from the real acg_classes.lst once SPELLCASTING_CLASS_NAMES is widened to include it

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.00s
```

Failed for the intended reason: `Shaman` was not yet in `SPELLCASTING_CLASS_NAMES`, so
the real `CLASS:Shaman` line in `acg_classes.lst` was silently skipped (out-of-scope, no
diagnostic).

### Acceptance RED — `tests/sd22_acg_class_shaman_resolves.rs`

```
$ cargo test --locked --test sd22_acg_class_shaman_resolves
error[E0599]: no variant, associated function, or constant named `Shaman` found for enum `AcgClassId` in the current scope
  --> tests/sd22_acg_class_shaman_resolves.rs:40:49
   |
40 |     let row = class_chassis_resolve(AcgClassId::Shaman, 1, RuleSetId::Acg)
   |                                                 ^^^^^^ variant, associated function, or constant not found in `AcgClassId`
(...4 more identical E0599s at lines 50, 61, 74, 83...)
error: could not compile `codex` (test "sd22_acg_class_shaman_resolves") due to 5 previous errors
```

Failed to compile for the intended reason: `AcgClassId::Shaman` did not exist yet.

## Green-phase evidence

### Widening + acceptance test, real-corpus-gated run

```
$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd17_b_spellcasting_class -- --include-ignored
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data cargo test --locked --test sd22_acg_class_shaman_resolves -- --include-ignored
running 7 tests
test arcanist_bloodrager_brawler_hunter_and_investigator_chassis_still_resolve_after_shaman_lands ... ok
test shaman_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test shaman_chassis_returns_none_for_ruleset_apg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test shaman_chassis_returns_none_for_ruleset_crb ... ok
test shaman_level_1_chassis_resolves_via_ruleset_acg ... ok
test shaman_level_20_chassis_resolves_via_ruleset_acg ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Full suite

```
$ cargo test --locked
411 "test result: ok" blocks across every suite; grepped the full output for
"FAILED" / "error[" / "N failed" with N > 0 — zero hits. Sibling-preservation holds —
all six untouched APG class-chassis suites, both untouched APG spell/equipment suites,
and all five untouched ACG suites (Arcanist, Bloodrager, Brawler, Hunter, Investigator)
all still pass unmodified.
```

### Clippy

```
$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (/home/ubuntu/workspace/repos/codex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 27.74s
(exit code 0, zero warnings/errors)
```

## Files touched

- `src/rules_core/rules_tables/acg/class_shaman.rs` (NEW)
- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `AcgClassId::Shaman` + match arm + `pub mod class_shaman;`)
- `src/pcgen_import/lst_parser/spellcasting_class.rs` (MODIFIED; `SPELLCASTING_CLASS_NAMES` widened by one — `Shaman`)
- `tests/sd22_acg_class_shaman_resolves.rs` (NEW)
- `tests/sd17_b_spellcasting_class.rs` (MODIFIED; real-corpus grounding test for the widening)
- `docs/release/SD-22/artifacts/acg/class_shaman_cycle_receipt.md` (NEW, this file)
- `docs/release/SD-22/progress.md` (MODIFIED; status matrix + cycle log)
- `docs/release/SD-22/receipts.md` (MODIFIED; receipt block appended)

## Source grounding

Real record verified directly against the corpus before any test was written (not
`corpus-source-inventory.md §2.1`'s non-authoritative "Content shape" prose):

`acg_classes.lst:221`:
```
CLASS:Shaman	HD:8		TYPE:Base.PC	MAXLEVEL:20	SOURCEPAGE:p.35	DEFINE:ShamanLVL|0	BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4|TYPE=Base.REPLACE|PREVAREQ:UseAlternateBABProgression,0	...	BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2|PREVAREQ:UseAlternateSaveProgression,0	BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3|PREVAREQ:UseAlternateSaveProgression,0	...
```

`acg_classes.lst:225`:
```
CLASS:Shaman	SPELLSTAT:WIS	MEMORIZE:YES	KNOWNSPELLS:LEVEL=0|LEVEL=1|...|LEVEL=9	BONUS:CASTERLEVEL|Shaman|CL
```

- Three-quarter BAB (`*3/4`) — same posture as ACG's Hunter/Investigator and APG's
  Alchemist/Inquisitor/Oracle/Summoner.
- Good Will (`/2+2`), its own single-save token.
- Poor Fortitude **and** Reflex (one combined token, `/3`) — the same combined-pairing
  shape as Brawler's/Hunter's `BASE.Fortitude,BASE.Reflex` token, but poor instead of good.
- `MAXLEVEL:20`.
- `SPELLSTAT:WIS MEMORIZE:YES` with no `SPELLBOOK:YES` and no `MEMORIZE:NO` —
  standard-prepared casting (same posture as APG's Witch), confirming Shaman belongs in
  `lst_parser::spellcasting_class`'s `SPELLCASTING_CLASS_NAMES` allowlist, not
  `lst_parser::class`'s `MARTIAL_CLASS_NAMES`.

## Cycle metadata

- cycle_id: 2026-07-20T01:17:01Z
- bundle_criterion: criteria 10-12 (ACG per-class cycles, class 6 of the corrected
  10-class roster)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:CLASS:Shaman`
  (real corpus; `decisions.md §5`)
- RuleSetId: Acg

## kanban

- card: see `docs/release/SD-22/progress.md` cycle log / `receipts.md` for the minted
  card id (hermes attempted per Step 10b; recorded there, not duplicated here).
