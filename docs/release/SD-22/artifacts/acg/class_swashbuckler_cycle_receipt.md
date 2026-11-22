# Swashbuckler cycle receipt — 2026-07-20T03:17:19Z

Class 9 of the corrected 10-class ACG roster (Arcanist, Bloodrager, Brawler,
Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest).

Ran in parallel with a sibling stream working Epic 6 criterion 20
(`tests/sd22_dm_toolkit_deterministic.rs`, and possibly
`corpus-source-inventory.md` reconciliation edits). File-touch set for
this cycle (`acg/`, `tests/sd22_acg_class_swashbuckler_resolves.rs`,
`tests/sd17_b1_martial_class.rs`'s widening test,
`src/pcgen_import/lst_parser/class.rs`'s `MARTIAL_CLASS_NAMES`) is
disjoint from the sibling's per `loop-instruction.md`'s file-touch
partition. Did all RED/GREEN/verification work before touching
`progress.md`/`receipts.md`.

Source: PCGen `acg_classes.lst:347`, `CLASS:Swashbuckler` record:

```
CLASS:Swashbuckler	HD:10		TYPE:Base.PC	MAXLEVEL:20	SOURCEPAGE:p.56	DEFINE:SlayerLVL|0	DEFINE:SwashbucklerLVL|0	BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE|PREVAREQ:UseAlternateBABProgression,0	BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/3|PREVAREQ:UseAlternateSaveProgression,0	BONUS:SAVE|BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2|PREVAREQ:UseAlternateSaveProgression,0	BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3|PREVAREQ:UseAlternateSaveProgression,0	...
```

No `SPELLSTAT:`/`MEMORIZE:`/`SPELLBOOK:` token appears anywhere in the
`CLASS:Swashbuckler` block (confirmed via `sed -n '345,352p' acg_classes.lst
| grep -o "SPELLSTAT...\|MEMORIZE...\|SPELLBOOK..."` → no hits) — same
non-caster posture as Cavalier/Brawler/Slayer, so this cycle widened
`MARTIAL_CLASS_NAMES` in `src/pcgen_import/lst_parser/class.rs`, not
`SPELLCASTING_CLASS_NAMES`.

Chassis-bearing tokens transcribed:
- `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE` — full BAB (no fractional divisor).
- `BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/3` — poor Fortitude save.
- `BONUS:SAVE|BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Reflex save (this class's only good save — unlike Slayer's combined Fortitude+Reflex token, Swashbuckler carries three separate single-save tokens).
- `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` — poor Will save.
- `MAXLEVEL:20`.

## Red-phase evidence

Widening RED — `cargo test --locked --test sd17_b1_martial_class
parses_real_swashbuckler_record -- --include-ignored` (PCGEN_CORPUS_ROOT set),
before the `MARTIAL_CLASS_NAMES` widening:

```
running 1 test
test parses_real_swashbuckler_record_from_acg_classes_lst ... FAILED

thread 'parses_real_swashbuckler_record_from_acg_classes_lst' panicked at tests/sd17_b1_martial_class.rs:779:10:
Swashbuckler should be recognized from the real acg_classes.lst once MARTIAL_CLASS_NAMES is widened to include it
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s
```

Acceptance RED — `cargo test --locked --test sd22_acg_class_swashbuckler_resolves`,
before `AcgClassId::Swashbuckler` existed:

```
error[E0599]: no variant, associated function, or constant named `Swashbuckler` found for enum `AcgClassId` in the current scope
  --> tests/sd22_acg_class_swashbuckler_resolves.rs:35:49
   |
35 |     let row = class_chassis_resolve(AcgClassId::Swashbuckler, 1, RuleSetId::Acg)
   |                                                 ^^^^^^^^^^^^ variant, associated function, or constant not found in `AcgClassId`
(5 call sites total)
error: could not compile `codex` (test "sd22_acg_class_swashbuckler_resolves") due to 5 previous errors
```

Both failures confirmed for the intended reason (missing allowlist entry;
missing enum variant), not an unrelated compile/setup error.

## Green-phase evidence

`cargo test --locked --test sd22_acg_class_swashbuckler_resolves -- --include-ignored`:

```
running 7 tests
test prior_acg_classes_still_resolve_after_swashbuckler_lands ... ok
test swashbuckler_chassis_returns_none_for_ruleset_apg ... ok
test swashbuckler_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test swashbuckler_chassis_returns_none_for_ruleset_crb ... ok
test swashbuckler_level_1_chassis_resolves_via_ruleset_acg ... ok
test swashbuckler_level_20_chassis_resolves_via_ruleset_acg ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`cargo test --locked --test sd17_b1_martial_class -- --include-ignored`:
19/19 passed, including `parses_real_swashbuckler_record_from_acg_classes_lst`
and every pre-existing martial-class test (Fighter/Barbarian/Monk/Rogue/
Ranger/Paladin/Cavalier/Brawler/Slayer all unaffected).

Full `cargo test --locked`: 415 `test result: ok` blocks across every
suite, 0 failures anywhere (grepped for `FAILED`/`error[`/`N failed` with
`N > 0`, found none — sibling-preservation holds, including all eight
prior ACG class suites, all six APG class-chassis suites, both APG
spell/equipment suites, and the concurrently in-flight Epic 6
`tests/sd22_dm_toolkit_deterministic.rs` suite from the sibling stream).

`cargo clippy --locked --tests -- -D warnings`: clean (no warnings).

## Files touched

- `src/rules_core/rules_tables/acg/class_swashbuckler.rs` (NEW)
- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `AcgClassId::Swashbuckler` + match arm + `pub mod class_swashbuckler;`)
- `src/pcgen_import/lst_parser/class.rs` (MODIFIED; `MARTIAL_CLASS_NAMES` widened by one — `"Swashbuckler"`)
- `tests/sd22_acg_class_swashbuckler_resolves.rs` (NEW)
- `tests/sd17_b1_martial_class.rs` (MODIFIED; real-corpus grounding test for the widening)

## Cycle metadata

- cycle_id: 2026-07-20T03:17:19Z
- bundle_criterion: criteria 10-12 (ACG per-class cycles), class 9 of 10
- corpus_input_path: pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:347:CLASS:Swashbuckler (real corpus; decisions.md §5)
- RuleSetId: Acg

## kanban

- card: `t_1d251219` on `codex-tranche-5` (status=done).

## Next-eligible

Warpriest (class 10 of the corrected 10-class roster — the last real ACG
class), or a dedicated cycle for criterion 13's shared ACG spell/equipment
tables once all classes land. Note: the real `acg_classes.lst` roster also
carries an internal `Ex-Warpriest` `VISIBLE:NO` variant alongside
`Warpriest` itself — confirm which is the player-facing record before
that cycle starts (mirrors this bundle's own roster-verification
discipline).
