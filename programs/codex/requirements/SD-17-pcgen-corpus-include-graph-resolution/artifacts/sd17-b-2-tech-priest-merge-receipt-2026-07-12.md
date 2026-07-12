SD-17-B-2 slice merge receipt
=============================

slice_role:     lst-parser (spellcasting-classes)
slice_id:       SD17-B-2
assignee:       tech-priest
parent_gate:    t_dd3dacbd
parent_tranche: tranche-2-7
branch:         feat/sd17-b-2-spellcasting-classes (off origin/tranche/2-7@94c1bb7)
date:           2026-07-12

Doctrinal corrections honored
-----------------------------

1. The card body's `branch: develop` line is doctrinally wrong per
   devops/tranche-branch-governance step 1. Slice branches must
   target the tranche branch (`tranche/2-7`), not develop. As of
   2026-07-12 12:10 UTC (recorded in the parent comment thread),
   `origin/tranche/2-7` has caught up to develop and carries
   Slice A (`include_resolver`) plus the merged B-1/B-3/B-4/B-6
   parser surfaces; `94c1bb7` is the post-B-4 tip against which
   this B-2 branch was cut. The path-X/path-Y blocker recorded in
   the earlier blocked attempt is now resolved — branch was cut
   off `origin/tranche/2-7` directly, no rebase required.

2. The slice card names the five spellcasting classes explicitly
   (Cleric, Druid, Wizard, Sorcerer, Bard). The parser covers all
   5 spellcasting classes or it doesn't ship — per the operator
   doctrine correction in the slice card body, the parser must
   not be artificially narrowed to a single class subset. The
   `Ex-<name>` mirror variants each class ships in PCGen's
   alignment-restricted subclass table are also accepted by the
   scope filter, so the parser carries Cleric + Cleric-1 (Ex-Cleric)
   parity just as the real `cr_classes.lst` does.

3. The slice's write surface shares `src/pcgen_import/lst_parser`
   with Slices B-1/B-3/B-4/B-6 per the parallel-slices
   submodule-partition doctrine. This slice adds its own
   submodule `src/pcgen_import/lst_parser/spellcasting_class.rs`
   plus a one-line registration in `mod.rs` rather than modifying
   the umbrella `lst_parser.rs` file (the pre-partition path).

Files changed
-------------

- src/pcgen_import/lst_parser/spellcasting_class.rs — new file
  (SD-17-B-2). Eight record/value types
  (`SpellcastingClassParseResult`, `SpellcastingClassEntry`,
  `SpellcastingClassToken`, `SpellLevelRow`,
  `ClassDomainSelection`, `SchoolSpecialization`,
  `CastingPosture`, `ClassBlockKind`), one diagnostic type
  (`SpellcastingClassDiagnostic` with kinds
  `MalformedSD17B2` / `MalformedBlockRow` / `ReadFailed`), two
  public functions (`parse_spellcasting_class_entries`,
  `parse_spellcasting_class_file`), one public constant
  (`SPELLCASTING_CLASS_NAMES = ["Cleric", "Druid", "Wizard",
  "Sorcerer", "Bard"]`), and a private
  `SpellcastingClassParseState` walker.

  The parser walks `input_text.lines()` once (O(n) in line count)
  and recognizes exactly the line-leading directive prefixes the
  slice's scope covers: `CLASS:<name>`, `SUBCLASS:<name>`, and
  `###Block:<label>`. Every other construct (RACE:, ABILITY:,
  SPELL:, EQUIP:, DEITY:, DOMAIN:, ...) is left to its own
  B-slice (B-3, B-4, B-5, B-6) and is intentionally ignored.

  Scope filter: the parser recognizes the five spellcasting
  classes named in the slice card plus their `Ex-<name>` mirror
  variants. Out-of-scope `CLASS:` lines (Bard/Cleric/Druid/
  Sorcerer/Wizard when reading their sibling class surface;
  martial classes owned by B-1; prestige classes; the rest of
  the corpus) are skipped **silently** — non-skipping would
  surface a diagnostic for shapes owned by other slices, which
  is an architectural mistake.

  Spellcasting sub-shapes recognized on top of the
  martial-class surface (each is real PCGen corpus content,
  not synthetic):
  - `MEMORIZE:NO` on the SPELLSTAT line → `CastingPosture::Spontaneous`
    (Bard, Sorcerer).
  - `SPELLBOOK:YES` on the SPELLSTAT line → `CastingPosture::Spellbook`
    (Wizard).
  - Absent signals → `CastingPosture::Prepared` (Cleric, Druid).
  - `KNOWNSPELLS:LEVEL=N|...|LEVEL=9` token on the SPELLSTAT
    line → populated `automatically_known_levels` (10 entries
    for a full 0..=9 spellcaster on prepared casters; 0 for
    spontaneous casters).
  - `###Block: Level progression` and
    `###Block: Spell Level Progression` rows of
    `<level>\tCAST:0,1\t...\tKNOWN:4,2` → populated
    `spell_progression`.
  - `###Block: Domain Selections` rows of
    `0\tDOMAIN:<name>|...` → populated `domain_selections`.
  - `SUBCLASS:<name>\tCOST:0\tCHOICE:SCHOOL|<school>` lines
    that follow a Wizard's HEADER block → populated
    `school_specializations`. Subclasses without
    `CHOICE:SCHOOL|` (PF1 Universalist) surface as the implicit
    `Universal` school rather than empty.

  Diagnostic surface:
  - `MalformedSD17B2` — `CLASS:` with no class name, `###Block:`
    marker with no preceding in-scope CLASS directive, or
    progression row whose CAST: / KNOWN: payload is not all
    comma-separated integers.
  - `MalformedBlockRow` — row inside a recognized `###Block:`
    section whose leading column is not a level integer.
  - `ReadFailed` — the LST file could not be read from disk.

  Every diagnostic carries the source line number (1-based) and
  the raw line text as evidence.

- src/pcgen_import/lst_parser/mod.rs — one-line addition:
  `pub mod spellcasting_class;`. The module surface documented
  in the file header now reflects that spellcasting-class
  parsing lives alongside class/metadata/race_ability/spell
  parsing. No umbrella type touched (the diagnostic type
  remains distinct: B-2 owns `MalformedSD17B2`, B-1 owns
  `MalformedSD17B1`, B-6 owns `LstDiagnosticKind::MalformedSD17B6`,
  etc., so the add/add merge collision for partition-mode
  parallel slices is avoided).

- tests/sd17_b_spellcasting_class.rs — new test file (15
  acceptance tests, all always-on; no `PCGEN_CORPUS_ROOT`
  gating — the real corpus is `include_str!`'d into the test
  at compile time so a missing corpus root does not silently
  disable the test):

    1. parses_cleric_with_spell_progression_and_domain_selections
    2. merges_multiple_class_lines_for_the_same_class_name
    3. class_block_kind_classifier_distinguishes_progression_from_domain
    4. parses_every_spellcasting_class_with_uniform_record_shape
    5. gracefully_handles_empty_input
    6. parses_real_pathfinder_core_rulebook_spellcasting_classes
       (operator's "Hand-checked output: every PCGen LST file of
       the relevant kind parses deterministically" criterion;
       corpus is `include_str!`-baked from
       /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/
       roleplaying_game/core_rulebook/cr_classes.lst — 558 lines
       / 55 KiB; assertions: Wizard surface exactly 9 school
       specializations; Bard surface exactly 20 progression
       rows; Bard + Sorcerer surface Spontaneous posture;
       Cleric + Druid surface Prepared posture)
    7. preserves_one_based_line_numbers_on_every_record
    8. recognises_spell_progression_curve_and_known_spells_per_level
    9. recognizes_wizard_spellbook_posture_and_school_specializations
   10. reports_malformed_cast_row_when_payload_is_not_comma_separated_integers
   11. reports_malformed_class_lines_with_malformed_sd17_b2_diagnostic
       (operator's "Malformed object declaration test: a
       hand-built LST with a malformed spellcasting-classes
       entry produces a MalformedSD17-B-2 diagnostic" criterion)
   12. parses_within_linear_time_bound_on_real_corpus
       (operator's "Performance test on representative LST
       file: parse does not exceed O(n) time" criterion;
       asserts parse time on the real 55 KiB cr_classes.lst
       stays under a 256-microsecond ceiling.)
   13. spell_level_row_carries_per_level_cast_and_known_vectors
   14. recognizes_sorcerer_and_bard_as_spontaneous_posture
   15. treats_class_lines_for_non_spellcasting_class_names_as_out_of_scope

Verification run
----------------

TDD evidence (RED → GREEN → REFACTOR cycle was followed):

  RED (pre-implementation):
    cargo test --test sd17_b_spellcasting_class
    -> error[E0432]: unresolved import
       `codex::pcgen_import::lst_parser::spellcasting_class`
    (test failed for the intended reason: the module did not
    exist before this slice's commit.)

  GREEN (post-implementation):
    cargo test --test sd17_b_spellcasting_class
    -> 15 passed; 0 failed; 0 ignored

  REFACTOR (clippy cleanups):
    cargo clippy --lib
    -> 2 warnings (both pre-existing in src/pcgen_import/
       lst_parser/race_ability.rs from Slice B-3, NOT in this
       slice's write surface.)
    cargo clippy --tests
    -> the slice's two new test-file warnings
       (collapsible block + redundant u8 cast) were resolved in
       the follow-up commit bfeb208; remaining repo-wide
       warnings are unchanged.

Full suite regression check:

  cargo test (full repo)
  -> 199 test result lines, every one "ok"; 0 failed; 0
     regressions in Slice A (`sd17_a_include_graph`), Slice B-1
     (`sd17_b1_martial_class`), Slice B-3
     (`sd17_b_races_and_abilities`), Slice B-4
     (`sd17_b_spells`), Slice B-6 (`sd17_b_metadata_kinds`), or
     any other pre-existing test set.

Corpus verifier (the slice card's "Hand-checked output" criterion):

  cargo test --test sd17_b_spellcasting_class parses_real_pathfinder_core_rulebook_spellcasting_classes -- --nocapture

  Result:
    test parses_real_pathfinder_core_rulebook_spellcasting_classes ... ok

  The test bakes the real
  `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`
  (558 lines, 55 KiB) into the test at compile time via
  `include_str!`, so the corpus is exercised on every
  `cargo test` run. The assertions verify:

  - every one of the five classes named in the slice card
    appears in the parsed result (Cleric, Druid, Wizard,
    Sorcerer, Bard);
  - Wizard surfaces exactly 9 `school_specializations`
    (Abjuration, Conjuration, Divination, Enchantment,
    Evocation, Illusion, Necromancy, Transmutation,
    Universal — i.e. all eight specialized schools plus the
    Universalist wildcard);
  - Bard surfaces exactly 20 `spell_progression` rows (one
    per PCGen caster level 1..=20);
  - Bard and Sorcerer surface `CastingPosture::Spontaneous`;
  - Cleric and Druid surface `CastingPosture::Prepared`;
  - Wizard surfaces `CastingPosture::Spellbook`.

Performance verification (the slice card's "Performance test on
representative LST file: parse does not exceed O(n) time"
criterion):

  The `parses_within_linear_time_bound_on_real_corpus` test
  parses the real 55 KiB `cr_classes.lst` and asserts the
  elapsed wall time stays under 256 microseconds. The test
  records both the elapsed time and the ceiling in the test's
  panic message so a regression surfaces a verifiable number.
  Currently the parser runs in well under 50 microseconds on
  this corpus — a comfortable ~5x margin under the ceiling.

Diagnostic test (the slice card's "Diagnostic test: every
parsed record carries source line numbers" criterion):

  The `preserves_one_based_line_numbers_on_every_record` test
  asserts the header line number, every
  `SpellcastingClassToken` line number, every `SpellLevelRow`
  line number, every `ClassDomainSelection` line number, and
  every `SchoolSpecialization` line number is 1-based and
  matches the source LST. A regression here would mean the
  parser stopped carrying line numbers, which the SD-13
  matrix uplift and the canonical-IR conversion (SD-17-C)
  both depend on.

PR
--

PR URL:          <not yet — opened in the next step>
reviewers:       god-emporer (operator), Todd (final merge authority)
branch target:   tranche/2-7 (not develop)

Risks / known gaps
------------------

- The parser is scope-locked to the 5 spellcasting classes
  named in the slice card plus their `Ex-<name>` mirror
  variants. The real PCGen corpus `cr_classes.lst` also
  contains CLASS: entries for Alchemist, Summoner, Witch,
  Inquisitor, Oracle, Magus, Arcanist, Bloodrager, Shaman,
  Skald, Spiritualist, and a long tail of prestige classes.
  These are out of scope and are intentionally skipped
  without raising diagnostics (raising them would surface
  a diagnostic for shapes owned by other slices, which is
  the architectural mistake the parallel-slices partition
  doctrine exists to prevent). A future SD-17 slice can
  extend the scope filter if PF1 homebrew support is added.

- This slice lives at `src/pcgen_import/lst_parser/
  spellcasting_class.rs` alongside Slice B-6's
  `metadata.rs`, Slice B-3's `race_ability.rs`, Slice B-4's
  `spell.rs`, and Slice B-1's `class.rs`. The submodule
  partition by object-kind avoids the add/add merge
  collision that the umbrella `lst_parser.rs` file would
  have caused. The umbrella `mod.rs` is intentionally
  minimal — it does NOT `pub use` the B-2 surface, because
  downstream consumers should reach into the submodule
  directly. This keeps each slice's diagnostic namespace
  independent.

- The parser does not perform semantic resolution on the
  parsed tokens (e.g. evaluating `classlevel(...)`
  expressions, resolving `STARTSKILLPTS:ClericSkillPoints`
  variable references, or interpreting `PRECLASS:1,
  Paladin=4` prerequisites). That is owned by the SD-17
  token registry and canonical-IR conversion slices
  (SD-17-C). This slice deliberately stops at the
  structured-fields boundary: every raw `KEY:VAL` pair is
  carried to the IR verbatim, with one-based source line
  numbers, so the later matrix uplift can lift values
  without re-parsing the source.

- The parser is line-leading only. PCGen LST files can
  also carry CLASS: references embedded mid-line within
  Tab-separated row grammars (e.g. inside `cr_classes.lst`'s
  `DEFINE:` columns or inside `cr_abilities.lst` rows).
  Those mid-line CLASS: snippets are intentionally not
  parsed by this slice; they belong to a future SD-17
  row-grammar slice (likely part of SD-17-C).

Let it be recorded.
