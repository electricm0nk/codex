# Epic 3 — Criteria 7-8 — APG Witch class chassis (sixth and last class in the corrected 6-class ordering)

- cycle_id: 2026-07-19T19:00:00Z
- criterion_section: §1.1 Epic 3 — APG content-source ingest (criteria 7, 8; sixth and last class in the corrected 6-class ordering: Alchemist → Cavalier → Inquisitor → Oracle → Summoner → Witch)
- row_or_kind: ingest:apg_class
- branch_tip_before: 6f2a13e
- rule_set_used: Apg
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:172` — `CLASS:Witch`

## Why this criterion, this cycle

Re-checked state before picking a criterion: `git log` on `decisions.md`,
`corpus-source-inventory.md`, `risks-and-open-questions.md` shows no new
commits past `f8b4aae`/`6f2a13e` (the parallel-session doctrine
reconciliation merge, already reflected in the current tree). `origin/tranche/5`
HEAD (`6f2a13e`) matches this session's local HEAD after the initial
fetch/checkout/pull — no other stream landed work in the interim. Per
Step 1's priority order and the corrected 6-class ordering (Alchemist,
Cavalier, Inquisitor, Oracle, Summoner all `complete`), Witch (class 6 of
6, the last real APG class) is next-eligible.

Verified the real `CLASS:Witch` record directly (not
`corpus-source-inventory.md`'s non-authoritative prose):
`apg_classes.lst:172` carries
`BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")/2` (half BAB, poor
— the first poor-BAB class landed in this roster; every prior class was
either full or three-quarter),
`BONUS:SAVE|BASE.Will|classlevel(...)/2+2` (good Will only),
`BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/3` (poor Fortitude
and Reflex — the identical good/poor split to Oracle/Summoner),
`MAXLEVEL:20` (line 172), and (line 176) `SPELLSTAT:INT` with no
`MEMORIZE:NO` and no `SPELLBOOK:YES` token — the same absent-signals
prepared-casting posture as Cleric/Druid, confirming Witch belongs in
`lst_parser::spellcasting_class`'s allowlist rather than
`lst_parser::class`'s.

Scope for this cycle is bounded to the same shape already established by
Alchemist/Cavalier/Inquisitor/Oracle/Summoner: BAB/save chassis only.
Named per-level features (Hexes, Patron, familiar rules) and the
spell-per-day table are out of scope — same fabrication-risk rationale.

## Red-phase evidence

**Widening RED**. Added `parses_real_witch_record_from_apg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs`, a real-corpus-gated
(`PCGEN_CORPUS_ROOT`) test asserting `parse_spellcasting_class_file`
recognizes `CLASS:Witch` from the real `apg_classes.lst` with
`CastingPosture::Prepared` and `spell_stat == "INT"`. Ran against the
unchanged tree (`Witch` not yet in `SPELLCASTING_CLASS_NAMES`):

```
$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b_spellcasting_class \
    -- --ignored parses_real_witch_record_from_apg_classes_lst
test parses_real_witch_record_from_apg_classes_lst ... FAILED
thread panicked: "Witch should be recognized from the real apg_classes.lst
once SPELLCASTING_CLASS_NAMES is widened to include it"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 19 filtered out
```

Failed for the intended reason: `Witch` was out of the spellcasting
parser's scope and silently skipped (no diagnostic, per that parser's own
documented behavior).

**Acceptance RED**. Added `tests/sd22_apg_class_witch_resolves.rs`,
mirroring `sd22_apg_class_summoner_resolves.rs`'s shape: `ApgClassId::Witch`
at levels 1/20/21, the `RuleSetId::Crb` cross-book invariant, and a
real-corpus-gated grounding test. Ran against the unchanged tree:

```
$ cargo test --locked --test sd22_apg_class_witch_resolves
error[E0599]: no variant or associated item named `Witch` found for enum `ApgClassId`
  --> tests/sd22_apg_class_witch_resolves.rs:29:49
(4 occurrences)
error: could not compile `codex` (test "sd22_apg_class_witch_resolves") due to 4 previous errors
```

Failed for the intended reason: `ApgClassId::Witch` and
`rules_tables/apg/class_witch.rs` did not exist yet.

## Green-phase evidence

Added/modified:
- `src/pcgen_import/lst_parser/spellcasting_class.rs` — widened
  `SPELLCASTING_CLASS_NAMES` by exactly one name (`Witch`), per the
  file-touch-partition's bounded-widening pattern already used for
  Alchemist, Inquisitor, Oracle, and Summoner. Updated the module doc
  comment to record the widening and its rationale (absent-signals
  prepared posture).
- `src/rules_core/rules_tables/apg/class_witch.rs` — `class_table()`,
  half BAB (`level/2`, poor — the first poor-BAB class in this roster),
  good Will (`level/2+2`), poor Fortitude/Reflex (`level/3`), 20-level
  ceiling — read directly off `apg_classes.lst:172`'s
  `BONUS:COMBAT|BASEAB`, `BONUS:SAVE|BASE.Will`,
  `BONUS:SAVE|BASE.Fortitude,BASE.Reflex`, and `MAXLEVEL:20` tokens.
- `src/rules_core/rules_tables/apg/mod.rs` — added `ApgClassId::Witch`
  variant, `pub mod class_witch;`, and a match arm in
  `class_chassis_resolve`.

```
$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd22_apg_class_witch_resolves -- --include-ignored
running 5 tests
test witch_chassis_returns_none_for_ruleset_crb ... ok
test witch_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test witch_level_1_chassis_resolves_via_ruleset_apg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test witch_level_20_chassis_resolves_via_ruleset_apg ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b_spellcasting_class -- --include-ignored
running 20 tests
(all 20 pass, including parses_real_witch_record_from_apg_classes_lst and
 every pre-existing widening test — parses_real_alchemist/_inquisitor/_oracle/_summoner)
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

$ cargo test --locked
(every suite: 0 failed, including the pre-existing sd17_b_spellcasting_class,
 sd22_apg_class_alchemist_resolves, sd22_apg_class_cavalier_resolves,
 sd22_apg_class_inquisitor_resolves, and sd22_apg_class_oracle_resolves,
 sd22_apg_class_summoner_resolves suites — no sibling regression)

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.79s
(clean, zero warnings)
```

No sibling regression: every pre-existing test suite still reports `0
failed` after this change.

## Files touched

- `src/pcgen_import/lst_parser/spellcasting_class.rs` — modified (widened `SPELLCASTING_CLASS_NAMES` by one name: `Witch`)
- `src/rules_core/rules_tables/apg/mod.rs` — modified (added `ApgClassId::Witch`, match arm, doc-comment update)
- `src/rules_core/rules_tables/apg/class_witch.rs` — added
- `tests/sd17_b_spellcasting_class.rs` — modified (added `parses_real_witch_record_from_apg_classes_lst`)
- `tests/sd22_apg_class_witch_resolves.rs` — added

## Cycle metadata

- cycle_id: 2026-07-19T19:00:00Z
- bundle_criterion: criterion-7, criterion-8 (criterion-6's `RuleSetId::Apg`
  registration already landed in the Alchemist cycle; criterion-9's
  spell/equipment resolution remains out of scope this cycle)
- upstream reference: `apg_classes.lst:172` (`CLASS:Witch`), real PCGen
  corpus checkout at `/home/user/pcgen` (`https://github.com/PCGen/pcgen`)
- RuleSetId: Apg

## kanban

- card: no card: hermes unavailable in this cloud sandbox; this receipt +
  `docs/release/SD-22/receipts.md` are the durability backbone per Step
  10a/10b
- audit_comment: n/a

## Epic 3 closure note

With Witch landed, all six real APG classes (Alchemist, Cavalier,
Inquisitor, Oracle, Summoner, Witch) now have chassis tables and
`RuleSetId::Apg` resolution. Criteria 7-8 are complete for the full
roster. Criterion 9 (per-class spell/equipment resolution) remains open
for all six classes — no `apg/spell_list.rs` or `apg/equipment_tables.rs`
exists yet; that is a distinct work-unit for a future cycle, not
automatically covered by the per-class chassis cycles.
