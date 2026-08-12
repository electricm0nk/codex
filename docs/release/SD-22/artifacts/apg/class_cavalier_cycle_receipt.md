# Epic 3 — Criteria 7-8 — APG Cavalier class chassis (cycle 2 of 8)

- cycle_id: 2026-07-19T15:00:00Z
- criterion_section: §1.1 Epic 3 — APG content-source ingest (criteria 7, 8; second APG class)
- row_or_kind: ingest:apg_class
- branch_tip_before: 9c187a7
- rule_set_used: Apg
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:42` — `CLASS:Cavalier`

## Why this criterion, this cycle

The prior cycle (`9c187a7`) landed Alchemist (class 1 of 8) and explicitly
left "Next-eligible for Epic 3: Cavalier (class 2 of 8), or a dedicated
cycle for Alchemist's spell/equipment tables (criterion 9)" as the two
open options. This firing picked Cavalier — the per-class chassis unit
mirrors the already-proven Alchemist pattern exactly, while criterion 9's
spell/equipment tables are a distinct shared-file work-unit
(`apg/spell_list.rs` / `apg/equipment_tables.rs`) better suited to its own
cycle.

Before writing any RED test, re-verified the real `.lst` record rather
than trusting `corpus-source-inventory.md`'s non-authoritative "Content
shape" prose (per that file's corrective banner): `apg_classes.lst:42`'s
`CLASS:Cavalier` line carries `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")`
(full BAB, no fractional divisor — unlike Alchemist's three-quarter BAB),
`BONUS:SAVE|BASE.Fortitude|classlevel(...)/2+2` (good Fortitude),
`BONUS:SAVE|BASE.Will,BASE.Reflex|classlevel(...)/3` (poor Will **and**
Reflex — Cavalier's good/poor split differs from Alchemist's), and
`MAXLEVEL:20`. Also confirmed the record carries no `SPELLSTAT:` line,
confirming Cavalier is a non-caster (unlike Alchemist) and belongs in
`lst_parser::class`'s martial-class allowlist rather than
`lst_parser::spellcasting_class`'s.

Scope for this cycle is bounded to the same shape
`rules_tables::apg::class_alchemist` already established: the BAB/save
chassis only. Named per-level features (Order, Challenge, Tactician,
Banner, Expert Trainer, ...) require walking `apg_abilities_class.lst`'s
per-level feature blocks in a dedicated future ingest slice — transcribing
that content from memory here would be exactly the fabrication risk
`AGENTS.md` and `class_alchemist.rs`'s own doc comment rule out.

## Red-phase evidence

**Widening RED** (per `loop-instruction.md`'s file-touch-partition rule:
"a per-class Epic 3/4 cycle widens `MARTIAL_CLASS_NAMES` or
`SPELLCASTING_CLASS_NAMES` by exactly one name, ONLY when that class's
`.lst` record isn't yet recognized"). Added
`parses_real_cavalier_record_from_apg_classes_lst` to
`tests/sd17_b1_martial_class.rs`, a real-corpus-gated
(`PCGEN_CORPUS_ROOT`) test asserting `parse_class_file` recognizes
`CLASS:Cavalier` from the real `apg_classes.lst`. Ran against the
unchanged tree (`Cavalier` not yet in `MARTIAL_CLASS_NAMES`):

```
$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b1_martial_class \
    parses_real_cavalier_record_from_apg_classes_lst -- --ignored
test parses_real_cavalier_record_from_apg_classes_lst ... FAILED
thread panicked: "Cavalier should be recognized from the real apg_classes.lst
once MARTIAL_CLASS_NAMES is widened to include it"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out
```

Failed for the intended reason: `Cavalier` was out of the martial-class
parser's scope and silently skipped (no diagnostic, per that parser's own
documented behavior).

**Acceptance RED**. Added `tests/sd22_apg_class_cavalier_resolves.rs`,
mirroring `sd22_apg_class_alchemist_resolves.rs`'s shape:
`ApgClassId::Cavalier` at levels 1/20/21, the `RuleSetId::Crb` cross-book
invariant, and a real-corpus-gated grounding test. Ran against the
unchanged tree:

```
$ cargo test --locked --test sd22_apg_class_cavalier_resolves
error[E0599]: no variant or associated item named `Cavalier` found for enum `ApgClassId`
  --> tests/sd22_apg_class_cavalier_resolves.rs:27:49
(4 occurrences)
error: could not compile `codex` (test "sd22_apg_class_cavalier_resolves") due to 4 previous errors
```

Failed for the intended reason: `ApgClassId::Cavalier` and
`rules_tables/apg/class_cavalier.rs` did not exist yet.

## Green-phase evidence

Added/modified:
- `src/pcgen_import/lst_parser/class.rs` — widened `MARTIAL_CLASS_NAMES`
  by exactly one name (`Cavalier`), per the file-touch-partition's
  bounded-widening pattern already used for Alchemist in
  `spellcasting_class.rs`. Updated the module doc comment to record the
  widening and its rationale (non-caster posture).
- `src/rules_core/rules_tables/apg/class_cavalier.rs` — `class_table()`,
  full BAB (`level`), good Fortitude (`level/2+2`), poor Reflex/Will
  (`level/3`), 20-level ceiling — read directly off `apg_classes.lst:42`'s
  `BONUS:COMBAT|BASEAB`, `BONUS:SAVE|BASE.Fortitude`,
  `BONUS:SAVE|BASE.Will,BASE.Reflex`, and `MAXLEVEL:20` tokens.
- `src/rules_core/rules_tables/apg/mod.rs` — added `ApgClassId::Cavalier`
  variant, `pub mod class_cavalier;`, and a match arm in
  `class_chassis_resolve`. Lifted the previously Alchemist-only
  `ClassTableRow` struct up to this module (both class modules now
  `use super::ClassTableRow`) so `class_chassis_resolve` has a single
  return type across classes — a mechanical consequence of a second class
  landing, not a scope expansion.

```
$ cargo test --locked --test sd22_apg_class_cavalier_resolves
running 5 tests
test cavalier_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test cavalier_chassis_returns_none_for_ruleset_crb ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ignored, requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data
test cavalier_level_1_chassis_resolves_via_ruleset_apg ... ok
test cavalier_level_20_chassis_resolves_via_ruleset_apg ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd22_apg_class_cavalier_resolves -- --ignored
running 1 test
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b1_martial_class -- --include-ignored
running 16 tests
... (all 16 pass, including parses_real_cavalier_record_from_apg_classes_lst
     and named_martial_class_set_is_exactly_the_six_from_the_slice_card,
     which asserts its own test-local constant and is unaffected by the
     production MARTIAL_CLASS_NAMES widening)
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --locked
(every suite: 0 failed, including the pre-existing sd17_b1_martial_class
 and sd22_apg_class_alchemist_resolves suites — no sibling regression)

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 26.59s
(clean, zero warnings)
```

No sibling regression: every pre-existing test suite still reports `0
failed` after this change.

## Files touched

- `src/pcgen_import/lst_parser/class.rs` — modified (widened `MARTIAL_CLASS_NAMES` by one name: `Cavalier`)
- `src/rules_core/rules_tables/apg/mod.rs` — modified (added `ApgClassId::Cavalier`, `ClassTableRow` lifted up, match arm)
- `src/rules_core/rules_tables/apg/class_alchemist.rs` — modified (uses lifted `ClassTableRow`, no behavior change)
- `src/rules_core/rules_tables/apg/class_cavalier.rs` — added
- `tests/sd17_b1_martial_class.rs` — modified (added `parses_real_cavalier_record_from_apg_classes_lst`)
- `tests/sd22_apg_class_cavalier_resolves.rs` — added

## Cycle metadata

- cycle_id: 2026-07-19T15:00:00Z
- bundle_criterion: criterion-7, criterion-8 (criterion-6's `RuleSetId::Apg`
  registration already landed in the Alchemist cycle; criterion-9's
  spell/equipment resolution remains out of scope this cycle — no APG
  spell/equipment tables exist yet)
- upstream reference: `apg_classes.lst:42` (`CLASS:Cavalier`), real PCGen
  corpus checkout at `/home/user/pcgen` (`https://github.com/PCGen/pcgen`)
- RuleSetId: Apg

## kanban

- card: no card: hermes unavailable in this cloud sandbox; this receipt +
  `docs/release/SD-22/receipts.md` are the durability backbone per Step
  10a/10b
- audit_comment: n/a
