# Slayer cycle receipt — 2026-07-20T03:xx:xxZ

Class 8 of the corrected 10-class ACG roster (Arcanist, Bloodrager, Brawler,
Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest).

Source: PCGen `acg_classes.lst:327`, `CLASS:Slayer` record:

```
CLASS:Slayer	HD:10		TYPE:Base.PC	MAXLEVEL:20	SOURCEPAGE:p.53	DEFINE:SlayerLVL|0	BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE|PREVAREQ:UseAlternateBABProgression,0	BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2|PREVAREQ:UseAlternateSaveProgression,0	BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3|PREVAREQ:UseAlternateSaveProgression,0	...
```

No `SPELLSTAT:`/`MEMORIZE:`/`SPELLBOOK:` token appears anywhere in the
`CLASS:Slayer` block (confirmed via `sed -n '324,346p' acg_classes.lst |
grep -o "SPELLSTAT...\|MEMORIZE...\|SPELLBOOK..."` → no hits) — same
non-caster posture as Cavalier/Brawler, so this cycle widened
`MARTIAL_CLASS_NAMES` in `src/pcgen_import/lst_parser/class.rs`, not
`SPELLCASTING_CLASS_NAMES`.

Chassis-bearing tokens transcribed:
- `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE` — full BAB (no fractional divisor).
- `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Fortitude and Reflex saves (one combined token).
- `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` — poor Will save.
- `MAXLEVEL:20`.

## Red-phase evidence

Widening RED — `cargo test --locked --test sd17_b1_martial_class
parses_real_slayer_record -- --include-ignored` (PCGEN_CORPUS_ROOT set),
before the `MARTIAL_CLASS_NAMES` widening:

```
test parses_real_slayer_record_from_acg_classes_lst ... FAILED
thread 'parses_real_slayer_record_from_acg_classes_lst' panicked at tests/sd17_b1_martial_class.rs:735:10:
Slayer should be recognized from the real acg_classes.lst once MARTIAL_CLASS_NAMES is widened to include it
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out
```

Acceptance RED — `cargo test --locked --test sd22_acg_class_slayer_resolves`,
before `AcgClassId::Slayer` existed:

```
error[E0599]: no variant, associated function, or constant named `Slayer` found for enum `AcgClassId` in the current scope
  --> tests/sd22_acg_class_slayer_resolves.rs:34:49
   |
34 |     let row = class_chassis_resolve(AcgClassId::Slayer, 1, RuleSetId::Acg)
   |                                                 ^^^^^^ variant, associated function, or constant not found in `AcgClassId`
(5 call sites total)
error: could not compile `codex` (test "sd22_acg_class_slayer_resolves") due to 5 previous errors
```

Both failures confirmed for the intended reason (missing allowlist entry;
missing enum variant), not an unrelated compile/setup error.

## Green-phase evidence

`cargo test --locked --test sd22_acg_class_slayer_resolves -- --include-ignored`:

```
running 7 tests
test slayer_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test slayer_chassis_returns_none_for_ruleset_apg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test slayer_level_1_chassis_resolves_via_ruleset_acg ... ok
test slayer_level_20_chassis_resolves_via_ruleset_acg ... ok
test prior_acg_classes_still_resolve_after_slayer_lands ... ok
test slayer_chassis_returns_none_for_ruleset_crb ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`cargo test --locked --test sd17_b1_martial_class -- --include-ignored`:
18/18 passed, including `parses_real_slayer_record_from_acg_classes_lst`
and every pre-existing martial-class test (Fighter/Barbarian/Monk/Rogue/
Ranger/Paladin/Cavalier/Brawler all unaffected).

Full `cargo test --locked`: 413 `test result: ok` blocks across every
suite, 0 failures anywhere (grepped for `FAILED`/`error[`/`N failed` with
`N > 0`, found none — sibling-preservation holds, including all seven
prior ACG class suites, all six APG class-chassis suites, both APG
spell/equipment suites, and the concurrently in-flight Epic 6 DM-toolkit
work on the same branch).

`cargo clippy --locked --tests -- -D warnings`: clean (exit code 0, no
warnings).

## Files touched

- `src/rules_core/rules_tables/acg/class_slayer.rs` (NEW)
- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `AcgClassId::Slayer` + match arm + `pub mod class_slayer;`)
- `src/pcgen_import/lst_parser/class.rs` (MODIFIED; `MARTIAL_CLASS_NAMES` widened by one — `"Slayer"`)
- `tests/sd22_acg_class_slayer_resolves.rs` (NEW)
- `tests/sd17_b1_martial_class.rs` (MODIFIED; real-corpus grounding test for the widening)

## Cycle metadata

- cycle_id: 2026-07-20T03:xx:xxZ
- bundle_criterion: criteria 10-12 (ACG per-class cycles), class 8 of 10
- corpus_input_path: pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:327:CLASS:Slayer (real corpus; decisions.md §5)
- RuleSetId: Acg

## kanban

- card: `t_8eb18bde` on `codex-tranche-5` (status=done).

## Next-eligible

Swashbuckler (class 9 of the corrected 10-class roster), or a dedicated
cycle for criterion 13's shared ACG spell/equipment tables once more
classes land.
