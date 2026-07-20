SD-17-B-3 slice merge receipt
=============================

slice_role:   lst-parser (races-and-abilities)
slice_id:     SD17-B-3
assignee:     tech-priest
parent_gate:  t_dd3dacbd
parent_tranche: tranche-2-7
base_sha:     6d6d7a7 (tranche/2-7 tip, post Slice A merge)
branch:       feat/sd17-slice-b-3-races-abilities (off tranche/2-7)
date:         2026-07-12

Doctrinal corrections honored
-----------------------------

1. The card body's `branch: develop` line is doctrinally wrong per
   devops/tranche-branch-governance step 1. Slice branches must target the
   tranche branch (`tranche/2-7`). The promotion to develop is the
   operator's PR after tranche/2-7 has accumulated its slices. The slice
   worker therefore opened its PR against `tranche/2-7`, not `develop`.

2. The card body refers to `RACES:` lines but the PCGen corpus uses `RACE:`
   (singular, the convention used by every core Pathfinder file). The
   parser accepts both spellings; each is preserved verbatim in the
   `raw_directive` field.

Files changed
-------------

- src/pcgen_import/mod.rs                  — add `pub mod lst_parser;`
  module declaration; update doc comment to reflect the new module
  surface.
- src/pcgen_import/lst_parser.rs           — new file (SD-17-B-3). Three
  record types (`RaceDeclaration`, `AbilityDeclaration`,
  `AbilityParsedFields`), one diagnostic type (`LstDiagnostic`,
  `LstDiagnosticKind::{MalformedRacePointer,
  MalformedAbilityDeclaration}`), and one entry function
  (`parse_lst_entry`). The parser walks `input_text.lines()` once
  (O(n) in line count) and recognizes exactly the line-leading
  directive prefixes the slice's scope covers: `RACE:`, `RACES:`,
  `ABILITY:`. Every other construct (`CLASS:`, `SKILL:`, `TEMPLATE:`,
  inline tab-keyword columns, etc.) is left to its own slice and
  intentionally ignored.

Tests added
-----------

- tests/sd17_b_races_and_abilities.rs      — 14 acceptance tests
  covering: race-pointer recognition (`RACE:` + `RACES:`), race
  malformed-target diagnostic, ability pointer + pipe-delimited
  declaration + CATEGORY= prefix + trailing modifier preservation,
  AUTOMATIC/VIRTUAL/NORMAL kind discrimination, malformed ability
  surface, source line numbers on every record, isolation from non-
  RACE/ABILITY lines, hand-built MalformedSD17-B-3 diagnostic test,
  the seven core Pathfinder races deterministically parse with no
  diagnostics, and the linear-time budget test on a ~hundreds-line
  representative LST file. Internally the parser has 3 unit smoke
  tests for the helpers (`AbilityKind::Display`,
  `split_trailing_modifiers` boundary + no-boundary cases, and the
  SD17-B-3 slice tag on diagnostics).

Verification run
----------------

`cargo test --test sd17_b_races_and_abilities -- --nocapture` — 14/14
pass.

`cargo test` (whole suite, including the prior Slice A `pcc_entry_parse`
test set) — every test set passes, no regressions.

Corpus-sweep guard (audit evidence):
    234 files in /home/ubuntu/workspace/repos/pcgen/data containing
    at least one line-leading `RACE:`, `RACES:`, or `ABILITY:` line
    were parsed end-to-end:

        SUMMARY: 234 files, total_records=17847, total_diagnostics=0

    No parse panics, no surfaced diagnostics. The slice satisfies the
    operator directive "accept every PCGen LST of the relevant kind"
    without an artificial scope narrowing.

PR
--

PR URL:        <not yet — opened in the next step>
reviewers:     god-emporer (operator), Todd (final merge authority)
branch target: tranche/2-7 (not develop)

Risks / known gaps
------------------

- The parser is line-leading only. PCGen LST files also carry
  `ABILITY:`-prefixed record snippets embedded mid-line within
  Tab-separated row grammars (notably in the `_races.lst` files
  themselves, where the `Ability:` column carries
  `ABILITY:Internal|AUTOMATIC|Racial Traits ~ <Race>`). Those
  mid-line snippets are intentionally not parsed by this slice;
  they belong to a future SD-17-B-x row-grammar slice. The
  `parses_seven_core_pathfinder_races_deterministically` test
  explicitly asserts that `race_pointers` is empty on the seven
  core race files, so a regression here is caught loudly.
- The parser's `AbilityParsedFields` does NOT perform semantic
  resolution on the parsed target (e.g. resolving a `%LIST` virtual
  reference). That is owned by the SD-17 token registry + semantic
  conversion slices. This slice deliberately stops at the
  structured-fields boundary.

Let it be recorded.
