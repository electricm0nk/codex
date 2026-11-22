# Epic 3 — Criteria 7-8 — APG Oracle class chassis (fourth class in the corrected 6-class ordering)

- cycle_id: 2026-07-19T17:00:00Z
- criterion_section: §1.1 Epic 3 — APG content-source ingest (criteria 7, 8; fourth class in the corrected 6-class ordering: Alchemist → Cavalier → Inquisitor → Oracle → Summoner → Witch)
- row_or_kind: ingest:apg_class
- branch_tip_before: 6923e54
- rule_set_used: Apg
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:107` — `CLASS:Oracle`

## Why this criterion, this cycle

Re-checked state before picking a criterion: `6923e54` (landed since the
Inquisitor cycle) corrected `corpus-source-inventory.md`, `decisions.md`,
`epic-breakdown.md`, `risks-and-open-questions.md`, `scope-draft.md`,
`acceptance-and-verification.md`, and `technical-design.md` to remove
Gunslinger and Magus from APG's roster (they're Ultimate Combat / Ultimate
Magic content, confirmed by the Inquisitor cycle's own direct `.lst`
search, not APG) — this resolves the open blocker via the operator's
own recommended option 1 ("narrow Epic 3's class count from 8 to 6").
The corrected roster is Alchemist → Cavalier → Inquisitor → Oracle →
Summoner → Witch; Alchemist/Cavalier/Inquisitor are already `complete`,
so Oracle (class 4 of 6) is next-eligible per Step 1's priority order.

Verified the real `CLASS:Oracle` record directly (not
`corpus-source-inventory.md`'s non-authoritative prose):
`apg_classes.lst:107` carries
`BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4` (three-quarter
BAB, same posture as Alchemist/Inquisitor), `BONUS:SAVE|BASE.Will|classlevel(...)/2+2`
(good Will only), `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/3`
(poor Fortitude **and** Reflex — a different good/poor split than
Alchemist, Cavalier, or Inquisitor), `MAXLEVEL:20`, and
`SPELLSTAT:CHA MEMORIZE:NO` (spontaneous divine caster — the same
posture-bearing shape as Sorcerer/Bard/Inquisitor), confirming Oracle
belongs in `lst_parser::spellcasting_class`'s allowlist rather than
`lst_parser::class`'s.

Scope for this cycle is bounded to the same shape already established by
Alchemist/Cavalier/Inquisitor: BAB/save chassis only. Named per-level
features (Mystery, Curse, Revelation, ...) and the spell-per-day table
are out of scope — same fabrication-risk rationale.

## Red-phase evidence

**Widening RED**. Added `parses_real_oracle_record_from_apg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs`, a real-corpus-gated
(`PCGEN_CORPUS_ROOT`) test asserting `parse_spellcasting_class_file`
recognizes `CLASS:Oracle` from the real `apg_classes.lst`. Ran against
the unchanged tree (`Oracle` not yet in `SPELLCASTING_CLASS_NAMES`):

```
$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b_spellcasting_class \
    parses_real_oracle_record_from_apg_classes_lst -- --ignored
test parses_real_oracle_record_from_apg_classes_lst ... FAILED
thread panicked: "Oracle should be recognized from the real apg_classes.lst
once SPELLCASTING_CLASS_NAMES is widened to include it"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out
```

Failed for the intended reason: `Oracle` was out of the spellcasting
parser's scope and silently skipped (no diagnostic, per that parser's own
documented behavior).

**Acceptance RED**. Added `tests/sd22_apg_class_oracle_resolves.rs`,
mirroring `sd22_apg_class_inquisitor_resolves.rs`'s shape: `ApgClassId::Oracle`
at levels 1/20/21, the `RuleSetId::Crb` cross-book invariant, and a
real-corpus-gated grounding test. Ran against the unchanged tree:

```
$ cargo test --locked --test sd22_apg_class_oracle_resolves
error[E0599]: no variant or associated item named `Oracle` found for enum `ApgClassId`
  --> tests/sd22_apg_class_oracle_resolves.rs:28:49
(4 occurrences)
error: could not compile `codex` (test "sd22_apg_class_oracle_resolves") due to 4 previous errors
```

Failed for the intended reason: `ApgClassId::Oracle` and
`rules_tables/apg/class_oracle.rs` did not exist yet.

## Green-phase evidence

Added/modified:
- `src/pcgen_import/lst_parser/spellcasting_class.rs` — widened
  `SPELLCASTING_CLASS_NAMES` by exactly one name (`Oracle`), per the
  file-touch-partition's bounded-widening pattern already used for
  Alchemist and Inquisitor. Updated the module doc comment to record the
  widening and its rationale (spontaneous-divine posture).
- `src/rules_core/rules_tables/apg/class_oracle.rs` — `class_table()`,
  three-quarter BAB (`level*3/4`), good Will (`level/2+2`), poor
  Fortitude/Reflex (`level/3`), 20-level ceiling — read directly off
  `apg_classes.lst:107`'s `BONUS:COMBAT|BASEAB`,
  `BONUS:SAVE|BASE.Will`, `BONUS:SAVE|BASE.Fortitude,BASE.Reflex`, and
  `MAXLEVEL:20` tokens.
- `src/rules_core/rules_tables/apg/mod.rs` — added `ApgClassId::Oracle`
  variant, `pub mod class_oracle;`, a match arm in
  `class_chassis_resolve`, and updated the doc comment (the Gunslinger/
  Magus exclusion is now permanent per `6923e54`, not "operator-pinned
  ordering skip").

```
$ cargo test --locked --test sd22_apg_class_oracle_resolves -- --include-ignored
running 5 tests
test oracle_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test oracle_level_20_chassis_resolves_via_ruleset_apg ... ok
test oracle_chassis_returns_none_for_ruleset_crb ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test oracle_level_1_chassis_resolves_via_ruleset_apg ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b_spellcasting_class -- --ignored
running 5 tests
test parses_real_inquisitor_record_from_apg_classes_lst ... ok
test parses_real_oracle_record_from_apg_classes_lst ... ok
test parses_real_alchemist_record_from_apg_classes_lst ... ok
test parses_within_linear_time_bound_on_real_corpus ... ok
test parses_real_pathfinder_core_rulebook_spellcasting_classes ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.06s

$ cargo test --locked
(every suite: 0 failed, including the pre-existing sd17_b_spellcasting_class,
 sd22_apg_class_alchemist_resolves, sd22_apg_class_cavalier_resolves, and
 sd22_apg_class_inquisitor_resolves suites — no sibling regression)

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.35s
(clean, zero warnings)
```

No sibling regression: every pre-existing test suite still reports `0
failed` after this change.

## Files touched

- `src/pcgen_import/lst_parser/spellcasting_class.rs` — modified (widened `SPELLCASTING_CLASS_NAMES` by one name: `Oracle`)
- `src/rules_core/rules_tables/apg/mod.rs` — modified (added `ApgClassId::Oracle`, match arm, doc-comment update)
- `src/rules_core/rules_tables/apg/class_oracle.rs` — added
- `tests/sd17_b_spellcasting_class.rs` — modified (added `parses_real_oracle_record_from_apg_classes_lst`)
- `tests/sd22_apg_class_oracle_resolves.rs` — added

## Cycle metadata

- cycle_id: 2026-07-19T17:00:00Z
- bundle_criterion: criterion-7, criterion-8 (criterion-6's `RuleSetId::Apg`
  registration already landed in the Alchemist cycle; criterion-9's
  spell/equipment resolution remains out of scope this cycle)
- upstream reference: `apg_classes.lst:107` (`CLASS:Oracle`), real PCGen
  corpus checkout at `/home/user/pcgen` (`https://github.com/PCGen/pcgen`)
- RuleSetId: Apg

## kanban

- card: no card: hermes unavailable in this cloud sandbox; this receipt +
  `docs/release/SD-22/receipts.md` are the durability backbone per Step
  10a/10b
- audit_comment: n/a
