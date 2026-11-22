# Epic 3 — Criteria 7-8 — APG Inquisitor class chassis (cycle 3 of 8; Gunslinger skipped as blocked)

- cycle_id: 2026-07-19T16:00:00Z
- criterion_section: §1.1 Epic 3 — APG content-source ingest (criteria 7, 8; fourth class in the operator-pinned ordering, third to actually land)
- row_or_kind: ingest:apg_class
- branch_tip_before: 675ca65
- rule_set_used: Apg
- corpus_input_path: `pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:50` — `CLASS:Inquisitor`

## Why this criterion, this cycle

Per Step 1's priority order and `epic-breakdown.md`'s operator-pinned APG
ordering (Alchemist → Cavalier → Gunslinger → Inquisitor → Magus → Oracle
→ Summoner → Witch), Gunslinger (class 3) was next. Before writing any
RED test, verified the real record — and found `apg_classes.lst` has no
`CLASS:Gunslinger` line at all:

```
$ grep -n "^CLASS:Gunslinger" apg_classes.lst   # 0 hits
$ grep -n "Gunslinger" apg_classes.lst          # 0 hits, anywhere in the file
$ grep -rln "^CLASS:Gunslinger" data/pathfinder/paizo/roleplaying_game/
data/pathfinder/paizo/roleplaying_game/ultimate_combat/uc_classes.lst
$ grep -rln "^CLASS:Magus" data/pathfinder/paizo/roleplaying_game/
data/pathfinder/paizo/roleplaying_game/ultimate_magic/um_classes.lst
```

Gunslinger and Magus are real Pathfinder 1e content, but they belong to
*Ultimate Combat* and *Ultimate Magic* respectively — not the Advanced
Player's Guide. `apg_classes.lst`'s actual `CLASS:` roster (confirmed by
listing every `CLASS:` line in the file) is exactly the 6 real APG base
classes: Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch. This
matches Paizo's real APG table of contents — the "8 APG classes" list in
`corpus-source-inventory.md` §1.1 is itself in error for 2 of its 8 rows,
not just the "Content shape" prose the corrective banner already flagged.
`decisions.md §1` explicitly excludes Ultimate-line books from SD-22
scope ("SD-22 does NOT own Ultimate Combat / Ultimate Magic / any other
'Ultimate'-line book"), so pulling Gunslinger/Magus from `uc_classes.lst`
/ `um_classes.lst` under the APG epic would itself be a scope violation,
not just a routing fix.

This is the loop-instruction's own SD-22-specific hard stop ("the
specific record isn't present in the resolved tree") for exactly the
Gunslinger and Magus rows — logged as a new `## Open blockers` entry
rather than force-generating that content from memory or silently
re-routing it to the wrong book. Did not write any Gunslinger/Magus
files this cycle. Per Step 1 ("pick the smallest unclaimed eligible
acceptance criterion"), skipped to the next-eligible class in the
ordering that *does* have a real record: Inquisitor (class 4 of 8).

Verified the real `CLASS:Inquisitor` record directly (not
`corpus-source-inventory.md`'s non-authoritative prose):
`apg_classes.lst:50` carries
`BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4` (three-quarter
BAB, same posture as Alchemist), `BONUS:SAVE|BASE.Fortitude,BASE.Will|classlevel(...)/2+2`
(good Fortitude **and** Will — a different good/poor split than either
Alchemist or Cavalier), `BONUS:SAVE|BASE.Reflex|classlevel(...)/3` (poor
Reflex only), `MAXLEVEL:20`, and `SPELLSTAT:WIS MEMORIZE:NO` (spontaneous
divine caster — the same posture-bearing shape as Sorcerer/Bard),
confirming Inquisitor belongs in `lst_parser::spellcasting_class`'s
allowlist rather than `lst_parser::class`'s.

Scope for this cycle is bounded to the same shape already established by
Alchemist/Cavalier: BAB/save chassis only. Named per-level features
(Judgment, Monster Lore, Solo Tactics, Bane, Stalwart, ...) and the
spell-per-day table are out of scope — same fabrication-risk rationale.

## Red-phase evidence

**Widening RED**. Added `parses_real_inquisitor_record_from_apg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs`, a real-corpus-gated
(`PCGEN_CORPUS_ROOT`) test asserting `parse_spellcasting_class_file`
recognizes `CLASS:Inquisitor` from the real `apg_classes.lst`. Ran against
the unchanged tree (`Inquisitor` not yet in `SPELLCASTING_CLASS_NAMES`):

```
$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b_spellcasting_class \
    parses_real_inquisitor_record_from_apg_classes_lst -- --ignored
test parses_real_inquisitor_record_from_apg_classes_lst ... FAILED
thread panicked: "Inquisitor should be recognized from the real apg_classes.lst
once SPELLCASTING_CLASS_NAMES is widened to include it"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 16 filtered out
```

Failed for the intended reason: `Inquisitor` was out of the spellcasting
parser's scope and silently skipped (no diagnostic, per that parser's own
documented behavior).

**Acceptance RED**. Added `tests/sd22_apg_class_inquisitor_resolves.rs`,
mirroring `sd22_apg_class_alchemist_resolves.rs`'s shape: `ApgClassId::Inquisitor`
at levels 1/20/21, the `RuleSetId::Crb` cross-book invariant, and a
real-corpus-gated grounding test. Ran against the unchanged tree:

```
$ cargo test --locked --test sd22_apg_class_inquisitor_resolves
error[E0599]: no variant or associated item named `Inquisitor` found for enum `ApgClassId`
  --> tests/sd22_apg_class_inquisitor_resolves.rs:27:49
(4 occurrences)
error: could not compile `codex` (test "sd22_apg_class_inquisitor_resolves") due to 4 previous errors
```

Failed for the intended reason: `ApgClassId::Inquisitor` and
`rules_tables/apg/class_inquisitor.rs` did not exist yet.

## Green-phase evidence

Added/modified:
- `src/pcgen_import/lst_parser/spellcasting_class.rs` — widened
  `SPELLCASTING_CLASS_NAMES` by exactly one name (`Inquisitor`), per the
  file-touch-partition's bounded-widening pattern already used for
  Alchemist. Updated the module doc comment to record the widening and
  its rationale (spontaneous-divine posture).
- `src/rules_core/rules_tables/apg/class_inquisitor.rs` — `class_table()`,
  three-quarter BAB (`level*3/4`), good Fortitude/Will (`level/2+2`),
  poor Reflex (`level/3`), 20-level ceiling — read directly off
  `apg_classes.lst:50`'s `BONUS:COMBAT|BASEAB`,
  `BONUS:SAVE|BASE.Fortitude,BASE.Will`, `BONUS:SAVE|BASE.Reflex`, and
  `MAXLEVEL:20` tokens.
- `src/rules_core/rules_tables/apg/mod.rs` — added `ApgClassId::Inquisitor`
  variant, `pub mod class_inquisitor;`, a match arm in
  `class_chassis_resolve`, and a doc-comment note recording why
  Gunslinger/Magus are skipped in this book's ordering.

```
$ cargo test --locked --test sd22_apg_class_inquisitor_resolves
running 5 tests
test inquisitor_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test inquisitor_chassis_returns_none_for_ruleset_crb ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ignored, requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data
test inquisitor_level_1_chassis_resolves_via_ruleset_apg ... ok
test inquisitor_level_20_chassis_resolves_via_ruleset_apg ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd22_apg_class_inquisitor_resolves -- --ignored
running 1 test
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

$ PCGEN_CORPUS_ROOT=/home/user/pcgen/data cargo test --locked --test sd17_b_spellcasting_class -- --ignored
running 4 tests
test parses_real_alchemist_record_from_apg_classes_lst ... ok
test parses_real_inquisitor_record_from_apg_classes_lst ... ok
test parses_within_linear_time_bound_on_real_corpus ... ok
test parses_real_pathfinder_core_rulebook_spellcasting_classes ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

$ cargo test --locked
(every suite: 0 failed, including the pre-existing sd17_b_spellcasting_class,
 sd22_apg_class_alchemist_resolves, and sd22_apg_class_cavalier_resolves
 suites — no sibling regression)

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.29s
(clean, zero warnings)
```

No sibling regression: every pre-existing test suite still reports `0
failed` after this change.

## Files touched

- `src/pcgen_import/lst_parser/spellcasting_class.rs` — modified (widened `SPELLCASTING_CLASS_NAMES` by one name: `Inquisitor`)
- `src/rules_core/rules_tables/apg/mod.rs` — modified (added `ApgClassId::Inquisitor`, match arm, doc-comment note on Gunslinger/Magus)
- `src/rules_core/rules_tables/apg/class_inquisitor.rs` — added
- `tests/sd17_b_spellcasting_class.rs` — modified (added `parses_real_inquisitor_record_from_apg_classes_lst`)
- `tests/sd22_apg_class_inquisitor_resolves.rs` — added

## Cycle metadata

- cycle_id: 2026-07-19T16:00:00Z
- bundle_criterion: criterion-7, criterion-8 (criterion-6's `RuleSetId::Apg`
  registration already landed in the Alchemist cycle; criterion-9's
  spell/equipment resolution remains out of scope this cycle)
- upstream reference: `apg_classes.lst:50` (`CLASS:Inquisitor`), real PCGen
  corpus checkout at `/home/user/pcgen` (`https://github.com/PCGen/pcgen`)
- RuleSetId: Apg

## kanban

- card: no card: hermes unavailable in this cloud sandbox; this receipt +
  `docs/release/SD-22/receipts.md` are the durability backbone per Step
  10a/10b
- audit_comment: n/a
