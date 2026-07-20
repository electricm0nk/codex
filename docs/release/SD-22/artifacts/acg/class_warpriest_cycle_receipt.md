# Warpriest cycle receipt — 2026-07-20

Class 10 of 10 — the last real ACG class in the corrected roster (Arcanist,
Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer,
Swashbuckler, Warpriest). With this cycle, Epic 4 (ACG) has all ten real
classes chassis'd — criteria 10-12 complete for the full roster.

Ran in parallel with a sibling stream attempting to unblock Epic 5 (Bestiary
1), touching `src/pcgen_import/lst_parser/race_ability.rs`,
`src/pcgen_import/lst_parser/monster_stat_block.rs`, and
`src/rules_core/rules_tables/beastiary1/`. This cycle's file-touch set
(`acg/`, `tests/sd22_acg_class_warpriest_resolves.rs`,
`tests/sd17_b_spellcasting_class.rs`'s widening test,
`src/pcgen_import/lst_parser/spellcasting_class.rs`) is disjoint per
`loop-instruction.md`'s file-touch partition. Did all RED/GREEN/verification
work before touching `progress.md`/`receipts.md`. Both agents shared this
checkout (not separate worktrees) — the sibling's own uncommitted work
(`src/pcgen_import/lst_parser/mod.rs`, `src/rules_core/rules_tables/mod.rs`,
`monster_stat_block.rs`, `beastiary1/`, `tests/sd17_b_monster_stat_block.rs`,
`tests/sd22_beastiary1_subset_01_resolves.rs`) was present unstaged
throughout and was left completely untouched — this cycle's own `git add`
is scoped strictly to its own files by explicit path. Mid-cycle, the
sibling's in-progress `pub mod beastiary1;` registration (in the shared
`src/rules_core/rules_tables/mod.rs`) briefly broke the full-crate build
before the corresponding `beastiary1/monster_subset_01.rs` file existed;
this cycle polled `cargo check --locked --lib` in the background until the
sibling's own work reached a compilable state, rather than touching any of
the sibling's files itself.

Source: PCGen `acg_classes.lst:364`, `CLASS:Warpriest` record (verified
directly before writing any test):

```
CLASS:Warpriest	HD:8		TYPE:Base.PC	MAXLEVEL:20	EXCLASS:Ex-Warpriest	SOURCEPAGE:p.60	DEFINE:WarpriestLVL|0	BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4|TYPE=Base.REPLACE|...	BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2|...	BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2|...	BONUS:SAVE|BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3|...	ROLE:Combat.Cleric	FACT:ClassType|PC	FACT:Abb|Wrp	FACT:SpellType|Divine

CLASS:Warpriest	SPELLSTAT:WIS	KNOWNSPELLS:LEVEL=0|LEVEL=1|LEVEL=2|LEVEL=3|LEVEL=4|LEVEL=5|LEVEL=6	BONUS:CASTERLEVEL|Warpriest|CL	BONUS:CASTERLEVEL|Cleric|CL	SPELLLIST:1|Cleric
```

`SPELLSTAT:WIS` with no `MEMORIZE:NO` and no `SPELLBOOK:YES` token confirms
standard-prepared casting posture, same shape as Shaman/Witch — so this
cycle widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs`, not `class.rs`'s
`MARTIAL_CLASS_NAMES`.

**Roster verification before starting** (per this cycle's explicit
instruction): confirmed the real `acg_classes.lst` roster with
`grep -oP "^CLASS:\K[A-Za-z-]+" acg_classes.lst | sort -u`, which returns
both `Warpriest` and `Ex-Warpriest`. Read both full records directly:
`CLASS:Warpriest` (line 364) has `EXCLASS:Ex-Warpriest`, no `VISIBLE:NO`,
`ROLE:Combat.Cleric`, `FACT:ClassType|PC` — the real, player-facing class.
`CLASS:Ex-Warpriest` (line 413) has `VISIBLE:NO`, no `EXCLASS:` token, no
`SPELLSTAT:` line anywhere in its block (confirmed: only two `CLASS:
Ex-Warpriest` lines total, neither carrying `SPELLSTAT:`) — the internal
fallen-Warpriest NPC variant PCGen uses when a Warpriest violates their
deity's code (the same "ex-class" mechanic as Paladin → ex-Paladin).
This cycle chassis'd only the real, player-facing `Warpriest` class, per
the explicit instruction not to conflate the two.

Chassis-bearing tokens transcribed:
- `BONUS:COMBAT|BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4|TYPE=Base.REPLACE` — three-quarter BAB.
- `BONUS:SAVE|BASE.Fortitude|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Fortitude save.
- `BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/2+2` — good Will save.
- `BONUS:SAVE|BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/3` — poor Reflex save (the class's only poor save — the classic Cleric-shaped save spread).
- `MAXLEVEL:20`.

## Red-phase evidence

Widening RED — `cargo test --locked --test sd17_b_spellcasting_class
parses_real_warpriest_record -- --include-ignored` (PCGEN_CORPUS_ROOT set),
before the `SPELLCASTING_CLASS_NAMES` widening:

```
running 1 test
test parses_real_warpriest_record_from_acg_classes_lst ... FAILED

thread 'parses_real_warpriest_record_from_acg_classes_lst' panicked at tests/sd17_b_spellcasting_class.rs:1055:10:
Warpriest should be recognized from the real acg_classes.lst once SPELLCASTING_CLASS_NAMES is widened to include it
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s
```

Acceptance RED — `cargo test --locked --test sd22_acg_class_warpriest_resolves`,
before `AcgClassId::Warpriest` existed:

```
error[E0599]: no variant, associated function, or constant named `Warpriest` found for enum `AcgClassId` in the current scope
  --> tests/sd22_acg_class_warpriest_resolves.rs:47:49
   |
47 |     let row = class_chassis_resolve(AcgClassId::Warpriest, 1, RuleSetId::Acg)
   |                                                 ^^^^^^^^^ variant, associated function, or constant not found in `AcgClassId`
(5 call sites total)
error: could not compile `codex` (test "sd22_acg_class_warpriest_resolves") due to 5 previous errors
```

Both failures confirmed for the intended reason (missing allowlist entry;
missing enum variant), not an unrelated compile/setup error.

**Self-caught test-setup correction (documented for the audit trail):**
the widening test's first draft asserted `Ex-Warpriest` should be *absent*
from the parser's entries after the widening. That assertion was itself
wrong — `SpellcastingClassParseState::new` (pre-existing, not touched by
this cycle) already mirrors every name in `SPELLCASTING_CLASS_NAMES` into
an `Ex-{name}` scope entry too, so widening the array to include
`Warpriest` also widens the parser's recognized scope to `Ex-Warpriest`
generically (as a distinct parsed entry with no `casting_posture`/
`spell_stat`, since the real `CLASS:Ex-Warpriest` block carries no
`SPELLSTAT:` line at all). Caught this by reading the actual parser scope
logic rather than trusting the initial assumption; corrected the test to
assert the real behavior (`Ex-Warpriest` is parsed as a distinct entry
with `casting_posture: None`, `spell_stat: None`) instead of forcing the
production parser to match an incorrect premise. No `AcgClassId` chassis
variant exists for `Ex-Warpriest` regardless — that boundary is enforced
structurally by the enum.

## Green-phase evidence

`cargo test --locked --test sd22_acg_class_warpriest_resolves -- --include-ignored`
(PCGEN_CORPUS_ROOT set):

```
running 8 tests
test warpriest_chassis_is_none_for_level_beyond_maxlevel_20 ... ok
test ex_warpriest_variant_is_a_distinct_internal_record_not_chassis_d_here ... ok
test warpriest_chassis_returns_none_for_ruleset_apg ... ok
test hand_transcribed_chassis_matches_the_real_lst_bonus_tokens ... ok
test prior_acg_classes_still_resolve_after_warpriest_lands ... ok
test warpriest_chassis_returns_none_for_ruleset_crb ... ok
test warpriest_level_20_chassis_resolves_via_ruleset_acg ... ok
test warpriest_level_1_chassis_resolves_via_ruleset_acg ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`cargo test --locked --test sd17_b_spellcasting_class -- --include-ignored`:
27/27 passed, including `parses_real_warpriest_record_from_acg_classes_lst`
and every pre-existing spellcasting-class test (Cleric/Druid/Wizard/
Sorcerer/Bard/Alchemist/Inquisitor/Oracle/Summoner/Witch/Arcanist/
Bloodrager/Hunter/Investigator/Shaman/Skald all unaffected).

Full `cargo test --locked`: 418 `test result: ok` blocks across every
suite, 0 failures anywhere (grepped for `FAILED`/`error[`/`N failed` with
`N > 0`, found none — sibling-preservation holds, including all nine prior
ACG class suites, all six APG class-chassis suites, both APG
spell/equipment suites, the Epic 6 DM-toolkit deterministic-test suite,
and the concurrently in-flight sibling Bestiary-1 work once it reached a
compilable state). Doc-tests: 0 run, 0 failed (clean).

`cargo clippy --locked --tests -- -D warnings`: clean (no warnings).

## Files touched

- `src/rules_core/rules_tables/acg/class_warpriest.rs` (NEW)
- `src/rules_core/rules_tables/acg/mod.rs` (MODIFIED; `AcgClassId::Warpriest` + match arm + `pub mod class_warpriest;` + roster doc comment updated to reflect the full 10-class roster)
- `src/pcgen_import/lst_parser/spellcasting_class.rs` (MODIFIED; `SPELLCASTING_CLASS_NAMES` widened by one — `"Warpriest"` — plus doc comment)
- `tests/sd22_acg_class_warpriest_resolves.rs` (NEW)
- `tests/sd17_b_spellcasting_class.rs` (MODIFIED; real-corpus grounding test for the widening, including the Ex-Warpriest-distinction assertion)

## Cycle metadata

- cycle_id: 2026-07-20T (this cycle)
- bundle_criterion: criteria 10-12 (ACG per-class cycles), class 10 of 10 — Epic 4's class-roster criteria now complete for the full roster
- corpus_input_path: pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:364:CLASS:Warpriest (real corpus; decisions.md §5)
- RuleSetId: Acg

## Next-eligible

Epic 4's class roster (criteria 10-12) is now fully complete (10/10 real
classes). Criterion 13 (shared ACG spell/equipment tables, mirroring APG's
criterion 9) remains open as Epic 4's last piece — a future cycle's job,
not attempted this cycle. Epic 5 (Bestiary 1) remains a separate,
concurrently-worked lane (sibling stream, this same cycle window).
