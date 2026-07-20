# Arcanist (ACG) cycle receipt — 2026-07-19T20:18:28Z

## Roster correction (blocking finding, resolved by proceeding to the correct next class)

`corpus-source-inventory.md §2.1`'s row 1 names "Alchemist (ACG-side)" as the
first ACG class. Before writing any RED test, this cycle verified the real
`acg_classes.lst` directly:

```
$ grep -n "^CLASS:Alchemist" acg_classes.lst
(no output — 0 hits)

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

There is no `CLASS:Alchemist` record anywhere in the real ACG corpus —
Alchemist is APG-only content (already ingested in Epic 3). This is the
identical roster-defect shape as the resolved Gunslinger/Magus blocker in
Epic 3: `corpus-source-inventory.md`'s routing table names a class with no
real record, and (separately) omits `Slayer`, which does have one. Logged
to `progress.md`'s `## Open blockers` rather than fabricating an
Alchemist-ACG chassis. Proceeded with **Arcanist**, the first class in the
real corpus with an actual `CLASS:` record (and also position 2 in
`corpus-source-inventory.md §2.1`'s own listed order).

## Red-phase evidence

Acceptance RED (`tests/sd22_acg_class_arcanist_resolves.rs`, before
`RuleSetId::Acg`/`AcgClassId`/`rules_tables::acg` existed):

```
error[E0432]: unresolved import `codex::rules_core::rules_tables::acg`
error[E0599]: no variant, associated function, or constant named `Acg` found for enum `RuleSetId`
error: could not compile `codex` (test "sd22_acg_class_arcanist_resolves") due to 4 previous errors
```

Widening RED (`parses_real_arcanist_record_from_acg_classes_lst` added to
`tests/sd17_b_spellcasting_class.rs`, real-corpus-gated on
`PCGEN_CORPUS_ROOT`):

```
running 1 test
test parses_real_arcanist_record_from_acg_classes_lst ... FAILED
thread '...' panicked at tests/sd17_b_spellcasting_class.rs:863:10:
Arcanist should be recognized from the real acg_classes.lst once SPELLCASTING_CLASS_NAMES is widened to include it
test result: FAILED. 0 passed; 1 failed; 0 ignored
```

Both failed for the intended reason (missing module/variant; unrecognized
class name), not an unrelated compile or environment error.

## Green-phase evidence

```
$ cargo test --locked --test sd22_acg_class_arcanist_resolves -- --include-ignored
running 6 tests
test arcanist_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test arcanist_chassis_returns_none_for_ruleset_apg ... ok
test arcanist_chassis_returns_none_for_ruleset_crb ... ok
test arcanist_level_1_chassis_resolves_via_ruleset_acg ... ok
test arcanist_level_20_chassis_resolves_via_ruleset_acg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked --test sd17_b_spellcasting_class -- --include-ignored
running 21 tests
... (all 21 ok, including parses_real_arcanist_record_from_acg_classes_lst)
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked 2>&1 | grep -oE "[0-9]+ failed" | sort -u
0 failed

$ cargo clippy --locked --tests -- -D warnings
    Checking codex v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 31.47s
(clean — no warnings)
```

## Files touched

- `src/rules_core/rules_tables/acg/mod.rs` (NEW; `AcgClassId`, `ClassTableRow`, `class_chassis_resolve`, roster-correction doc comment)
- `src/rules_core/rules_tables/acg/class_arcanist.rs` (NEW; BAB/save chassis)
- `src/rules_core/rules_tables/mod.rs` (MODIFIED; `pub mod acg;` + `RuleSetId::Acg` variant)
- `src/pcgen_import/lst_parser/spellcasting_class.rs` (MODIFIED; `SPELLCASTING_CLASS_NAMES` widened by one — `Arcanist`)
- `tests/sd22_acg_class_arcanist_resolves.rs` (NEW)
- `tests/sd17_b_spellcasting_class.rs` (MODIFIED; `real_acg_classes_lst()` helper + widening test for the real-corpus grounding)

## Cycle metadata

- cycle_id: 2026-07-19T20:18:28Z
- bundle_criterion: criteria 10-12 (ACG per-class cycles, first ACG class)
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:11 (CLASS:Arcanist)`
- RuleSetId: Acg
- real record's chassis tokens: `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")/2` (poor/half BAB), `BONUS:SAVE|BASE.Will|classlevel(...)/2+2` (good Will), `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/3` (poor Fort+Reflex), `MAXLEVEL:20`, `SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES` (spellbook posture, same shape as Alchemist)

## kanban

- card: see `receipts.md` / `progress.md` for the minted card id (hermes reachable this session)
