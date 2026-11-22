# Bloodrager (ACG) cycle receipt — 2026-07-19T21:15:57Z

## State check before picking a criterion

`git log 3f8df8a..origin/tranche/5` showed no new commits — `3f8df8a` (the
prior Arcanist cycle's own commit) is still the tip, tree clean. With Epic 3
(APG) fully closed out and Arcanist (Epic 4's first ACG class) landed, the
prior cycle's own `next_required_uplift` named Bloodrager (class 2 of the
corrected 10-class roster) as next-eligible. Re-verified the real
`acg_classes.lst` roster directly rather than trusting the doc text:

```
$ grep -oP "^CLASS:\K[A-Za-z-]+" acg_classes.lst | sort -u
Arcanist
Bloodrager
Brawler
Ex-Warpriest
Hunter
Investigator
Shaman
Skald
Slayer
Swashbuckler
Warpriest
```

`Bloodrager` has a real `CLASS:Bloodrager` record (`acg_classes.lst:40`, plus
continuation lines 42 and 44) — confirmed before writing any test.

## Real record verification

`acg_classes.lst:40`'s `CLASS:Bloodrager` line carries
`BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE` (full
BAB — no fractional divisor, unlike Arcanist's poor/half BAB),
`BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2` (good
Fortitude), `BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3`
(poor Reflex and Will), `MAXLEVEL:20`, and (line 44)
`SPELLSTAT:CHA MEMORIZE:NO` (spontaneous casting, same posture as
Sorcerer/Bard/Oracle/Summoner) — confirming Bloodrager belongs in
`spellcasting_class.rs`'s allowlist, not `class.rs`'s.

## Red-phase evidence

Widening RED (`parses_real_bloodrager_record_from_acg_classes_lst` added to
`tests/sd17_b_spellcasting_class.rs`, real-corpus-gated on
`PCGEN_CORPUS_ROOT`), run against the unchanged tree:

```
$ cargo test --locked --test sd17_b_spellcasting_class -- --ignored parses_real_bloodrager
running 1 test
test parses_real_bloodrager_record_from_acg_classes_lst ... FAILED
thread '...' panicked at tests/sd17_b_spellcasting_class.rs:894:10:
Bloodrager should be recognized from the real acg_classes.lst once SPELLCASTING_CLASS_NAMES is widened to include it
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 21 filtered out
```

Acceptance RED (`tests/sd22_acg_class_bloodrager_resolves.rs`, before
`AcgClassId::Bloodrager` existed), run against the unchanged tree:

```
$ cargo test --locked --test sd22_acg_class_bloodrager_resolves
error[E0599]: no variant, associated function, or constant named `Bloodrager` found for enum `AcgClassId`
(5 occurrences, one per call site)
error: could not compile `codex` (test "sd22_acg_class_bloodrager_resolves") due to 5 previous errors
```

Both failed for the intended reason (unrecognized class name; missing enum
variant), not an unrelated compile or environment error.

## Green-phase evidence

```
$ cargo test --locked --test sd22_acg_class_bloodrager_resolves -- --include-ignored
running 7 tests
test arcanist_chassis_still_resolves_after_bloodrager_lands ... ok
test bloodrager_chassis_returns_none_for_ruleset_apg ... ok
test bloodrager_chassis_returns_none_for_ruleset_crb ... ok
test bloodrager_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test bloodrager_level_1_chassis_resolves_via_ruleset_acg ... ok
test bloodrager_level_20_chassis_resolves_via_ruleset_acg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked --test sd17_b_spellcasting_class -- --include-ignored
running 22 tests
... (all 22 ok, including parses_real_bloodrager_record_from_acg_classes_lst)
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked 2>&1 | grep -E "FAILED|error\[|^error:|result: FAILED"
(no output -- 0 failures anywhere; sibling-preservation holds, including the
untouched Arcanist suite, all six APG class-chassis suites, and both APG
spell/equipment suites)

$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 26.24s
(clean -- no warnings)
```

## Files touched

- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `AcgClassId::Bloodrager` variant + match arm, doc comment updated)
- `src/rules_core/rules_tables/acg/class_bloodrager.rs` (NEW; BAB/save chassis)
- `src/pcgen_import/lst_parser/spellcasting_class.rs` (MODIFIED; `SPELLCASTING_CLASS_NAMES` widened by one — `Bloodrager`)
- `tests/sd22_acg_class_bloodrager_resolves.rs` (NEW)
- `tests/sd17_b_spellcasting_class.rs` (MODIFIED; widening test for the real-corpus grounding, reusing the existing `real_acg_classes_lst()` helper)

## Cycle metadata

- cycle_id: 2026-07-19T21:15:57Z
- bundle_criterion: criteria 10-12 (ACG per-class cycles, second ACG class)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:40 (CLASS:Bloodrager)`
- RuleSetId: Acg
- real record's chassis tokens: `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE` (full BAB), `BONUS:SAVE|BASE.Fortitude|classlevel(...)/2+2` (good Fortitude), `BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel(...)/3` (poor Reflex+Will), `MAXLEVEL:20`, `SPELLSTAT:CHA MEMORIZE:NO` (spontaneous posture, same shape as Oracle/Summoner)

## kanban

- card: see `receipts.md` / `progress.md` for the minted card id or the "no card" fallback reason.
