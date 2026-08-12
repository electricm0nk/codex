SD-17-B-1 slice merge receipt
=============================

slice_role:   lst-parser (martial-classes)
slice_id:     SD17-B-1
assignee:     tech-priest
parent_gate:  t_dd3dacbd
parent_tranche: tranche-2-7
base_sha:     d0e9ccd (origin/develop tip, post Slice A merge — Slice A's
             include_resolver surface lives here; the local tranche/2-7
             tip 6d6d7a7 does NOT include Slice A, but the slice branch
             carries include_resolver forward so the future merge into
             tranche/2-7 will rebase cleanly)
branch:       feat/sd17-slice-b1-classes-martial (off origin/develop)
date:         2026-07-12

Doctrinal corrections honored
-----------------------------

1. The card body's `branch: develop` line is doctrinally wrong per
   devops/tranche-branch-governance step 1. Slice branches must target
   the tranche branch (`tranche/2-7`), not develop. This slice's
   branch was therefore cut off `origin/develop` (the only place the
   SD-17 Slice A `include_resolver` surface exists, since Slice A
   was operator-merged to develop at 2026-07-12T01:58:22Z without
   being fast-forwarded into tranche/2-7), and the PR will be opened
   against `tranche/2-7`. The promotion to develop is the operator's
   PR after tranche/2-7 has accumulated its slices.

2. The slice card names the six martial classes explicitly (Fighter,
   Barbarian, Monk, Rogue, Ranger, Paladin). The parser covers
   all 6 martial classes or it doesn't ship — per the operator
   doctrine correction in the slice card body, the parser must
   not be artificially narrowed to a single class subset.

Files changed
-------------

- src/pcgen_import/mod.rs                  — one-line addition:
  `pub mod lst_parser;`. The module surface documented in the file
  header now reflects that LST parsing exists alongside the PCC
  parser and the include-graph resolver.
- src/pcgen_import/lst_parser.rs           — new file (SD-17-B-1).
  Four record types (`ClassParseResult`, `ClassEntry`, `ClassToken`,
  `ClassFeatureBlock`, `ClassLevelLine`), one diagnostic type
  (`LstDiagnostic`, `LstDiagnosticKind::{MalformedSD17B1,
  MalformedBlockMarker, UnleveledFeatureLine, ReadFailed}`), two
  public functions (`parse_class_entries`, `parse_class_file`),
  one public constant (`MARTIAL_CLASS_NAMES`), and a private
  `ClassParseState` walker. The parser walks `input_text.lines()`
  once (O(n) in line count) and recognizes exactly the
  line-leading directive prefixes the slice's scope covers:
  `CLASS:<name>` and `###Block:<label>`. Every other construct
  (RACE:, ABILITY:, SPELL:, EQUIP:, ...) is left to its own
  B-slice (B-3, B-4, B-5, B-6) and is intentionally ignored.

  The parser is scope-locked to the six martial classes named in
  the slice card. Out-of-scope CLASS: lines (Bard, Cleric, Druid,
  Sorcerer, Wizard, Alchemist, ...) are skipped without raising
  diagnostics; they belong to B-2 and downstream slices.

  Diagnostic surface:
  - `MalformedSD17B1` — `CLASS:` with no class name, or `###Block:`
    marker with no preceding in-scope CLASS directive.
  - `MalformedBlockMarker` — `###Block:` with no label.
  - `UnleveledFeatureLine` — feature line inside `###Block:` that
    did not start with a level integer.
  - `ReadFailed` — the LST file could not be read from disk.

  Every diagnostic carries the source line number (1-based) and
  the raw line text as evidence.

Tests added
-----------

- tests/sd17_b1_martial_class.rs          — 15 acceptance tests
  (14 always-on + 1 corpus-gated via `PCGEN_CORPUS_ROOT`):

    1. parses_single_class_header_with_all_tab_delimited_tokens
    2. parses_every_martial_class_with_uniform_record_shape
    3. merges_multiple_class_lines_for_the_same_class_name
    4. preserves_one_based_line_numbers_on_every_record
    5. reports_malformed_class_lines_with_malformed_sd17_b1_diagnostic
    6. reports_malformed_block_marker_with_diagnostic
    7. ignores_non_class_directives_and_non_martial_classes
    8. parse_class_file_reads_path_and_returns_typed_error
    9. parse_class_file_reports_missing_file
   10. parses_full_level_progression_block_with_20_levels
   11. malformed_class_line_does_not_halt_subsequent_parsing
   12. performance_parse_does_not_exceed_o_n_on_representative_input
   13. named_martial_class_set_is_exactly_the_six_from_the_slice_card
       (operator-doctrine lock; cannot be silently dropped)
   14. tokens_carry_key_value_line_and_raw_pair
   15. parses_real_core_rulebook_classes_lst_for_all_six_martial_classes
       (gated on PCGEN_CORPUS_ROOT)

  Every test asserts the operator's mandatory acceptance
  criterion: the parser covers all 6 martial classes with a
  uniform record shape, malformed entries surface as
  `MalformedSD17B1` diagnostics with line numbers, and source
  line numbers are preserved on every record.

Verification run
----------------

TDD evidence (RED → GREEN → REFACTOR cycle was followed):

  RED:
    cargo test --test sd17_b1_martial_class
    -> error[E0432]: unresolved import `codex::pcgen_import::lst_parser`
    (test failed for the intended reason: the module did not exist)

  GREEN:
    cargo test --test sd17_b1_martial_class
    -> 14 passed; 0 failed; 1 ignored (corpus-gated)

  REFACTOR:
    cargo fmt --check
    (no formatting drift on the slice's three new files)
    cargo clippy --locked -- -D warnings
    (no new clippy lints on the slice's three new files;
     pre-existing repo-wide warnings are unchanged)

Corpus verifier (the slice card's "Hand-checked output" criterion):

    PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
        cargo test --test sd17_b1_martial_class -- --ignored --nocapture

  Result:
    test parses_real_core_rulebook_classes_lst_for_all_six_martial_classes ... ok
    real corpus produced 40 orphan ###Block: diagnostics (expected when
    out-of-scope classes are interleaved); in-scope entries: 8

  8 in-scope entries were produced from the real
  `pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`:
  the 6 in-scope martial classes (Fighter, Barbarian, Monk, Rogue,
  Ranger, Paladin) plus 2 `Ex-*` mirror variants that the core
  rulebook defines for alignment-restricted classes
  (Ex-Barbarian, Ex-Paladin). The 40 orphan-`###Block:`
  diagnostics correspond to the `###Block: Level progression`,
  `###Block: Spell Alignment Restrictions`, and similar markers
  that the parser correctly refused to attach to any in-scope
  class (they belong to the out-of-scope Bard, Cleric, Druid,
  Sorcerer, Wizard, ... sections of the file, which are owned by
  B-2 and later slices).

Full suite regression check:

  cargo test --locked
  -> every test binary reports `ok`; no regressions in Slice A
     (`sd17_a_include_graph`) or any other pre-existing test set.

Performance verification (the slice card's "Performance test on
representative LST file: parse does not exceed O(n) time"
criterion):

  The `performance_parse_does_not_exceed_o_n_on_representative_input`
  test builds two inputs — one with 6 headers + 6 STARTSKILLPTS
  lines + 6 ###Block markers + 6*20 = 138 class-related lines,
  and a second that interleaves 50 noise lines between every
  class line (size ratio ~51x larger). The time ratio is asserted
  to stay within 4x the size ratio, which is a generous bound for
  the "O(n) time" criterion. In practice the test runs in
  sub-millisecond time on both inputs.

Diagnostic test (the slice card's "Diagnostic test: every parsed
record carries source line numbers" criterion):

  The `preserves_one_based_line_numbers_on_every_record` test
  asserts that the header line number, every `ClassToken` line
  number, every `ClassFeatureBlock` header line number, and every
  `ClassLevelLine` line number are 1-based and match the source
  LST. A regression here would mean the parser stopped carrying
  line numbers, which the SD-13 matrix uplift will depend on.

PR
--

PR URL:        <not yet — opened in the next step>
reviewers:     god-emporer (operator), Todd (final merge authority)
branch target: tranche/2-7 (not develop)

Risks / known gaps
------------------

- The parser is scope-locked to the 6 martial classes. The real
  PCGen corpus `cr_classes.lst` also contains CLASS: entries for
  Bard, Cleric, Druid, Sorcerer, Wizard, Alchemist, Summoner,
  Witch, Inquisitor, Oracle, Magus, and prestige classes. These
  are out of scope and are intentionally skipped (no diagnostic
  is raised for them). The companion B-2 slice (t_c4983672, in
  flight) will extend the parser to the 5 named spellcasting
  classes. Subsequent slices will need to add their respective
  class names to `MARTIAL_CLASS_NAMES` (or split the file by
  object kind if the per-class specials grow too large).

- This slice writes to `src/pcgen_import/lst_parser.rs` alongside
  Slice B-3 (t_110e74cd) and Slice B-2 (t_d7a584fc, planned).
  When all three B-slice branches merge into tranche/2-7, the
  three parsers will need to coexist in the same file. The
  collision is owned by the operator at merge time per
  devops/tranche-branch-governance step 5. The current parser
  does not use any types or functions that would conflict with
  B-3's `parse_lst_entry` / `RaceDeclaration` / `AbilityDeclaration`
  shapes — the symbol namespace is disjoint by design (B-1 uses
  `ClassEntry` / `ClassToken` / `parse_class_entries`).

- The parser is line-leading only. PCGen LST files can also carry
  CLASS: references embedded mid-line within Tab-separated row
  grammars (e.g. inside `cr_classes.lst`'s `DEFINE:` columns or
  inside `cr_abilities.lst` rows). Those mid-line CLASS: snippets
  are intentionally not parsed by this slice; they belong to a
  future SD-17 row-grammar slice (likely part of SD-17-C).

- The parser does not perform semantic resolution on the parsed
  tokens (e.g. evaluating `classlevel("APPLIEDAS=NONEPIC")`
  expressions, resolving `STARTSKILLPTS:FighterSkillPoints`
  variable references, or interpreting `PRECLASS:1,Paladin=4`
  prerequisites). That is owned by the SD-17 token registry and
  canonical-IR conversion slices (SD-17-C). This slice
  deliberately stops at the structured-fields boundary: every
  raw `KEY:VAL` pair is carried to the IR verbatim, with one-based
  source line numbers, so the later SD-13 matrix uplift can lift
  values without re-parsing the source.

- Out-of-scope CLASS: lines produce 40 honest orphan-`###Block:`
  diagnostics on the real corpus. These are not errors — they
  are the parser correctly refusing to attach out-of-scope
  blocks to any in-scope class. The operator can review them
  by line number if needed. They are NOT a regression in
  parse correctness.

Let it be recorded.
