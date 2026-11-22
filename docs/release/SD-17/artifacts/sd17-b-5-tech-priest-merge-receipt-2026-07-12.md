SD-17-B-5 slice merge receipt
=============================

slice_role:   lst-parser (equipment + equipment-modifier)
slice_id:     SD17-B-5
assignee:     tech-priest
parent_gate:  t_dd3dacbd
parent_tranche: tranche-2-7
base_sha:     94c1bb7 (origin/tranche/2-7 tip, post Slice A + B-1/B-3/B-4/B-6 merges)
branch:       feat/sd17-slice-b-5-equipment (off origin/tranche/2-7)
pr:           https://github.com/electricm0nk/codex/pull/296
pr_target:    tranche/2-7
date:         2026-07-12

Doctrinal corrections honored
-----------------------------

1. The card body's `branch: develop` and `base_sha: c78287c + Slice A
   merge tip` are stale. Per `devops/tranche-branch-governance`, slice
   branches target the tranche branch (`tranche/2-7`), not develop.
   This slice's branch was cut off `origin/tranche/2-7` directly (the
   tranche tip at the time of dispatch already contained Slice A +
   B-1/B-3/B-4/B-6 parser surfaces), so no `develop`-first detour was
   needed. The card-body line and base_sha are inaccurate; this slice
   uses the live tranche tip `94c1bb7` as the real base.

2. The card body names `EQUIP:` and `EQUIPMOD:` as line-start directives
   to parse. The real PCGen corpus uses two shapes:
   a. The directive-prefix form: `EQUIP:<name>` / `EQUIPMOD:<name>`
      rows at line start (used in some homebrew corpora and matches
      the card body's literal reading).
   b. The corpus-typical row form: a row whose column 0 is the
      equipment or equipment-modifier name, with tab-delimited `KEY:VAL`
      tags in subsequent columns. This is the actual shape in
      `cr_equip_arms_armor.lst`, `cr_equip_general.lst`,
      `cr_equip_magic_items.lst`, `cr_equipmods.lst`, and every other
      equipment file in the Pathfinder / D&D 3.5e / D&D 5e /
      Pathfinder 2e / Starfinder corpora on this host. Per operator
      directive 2026-07-12 (the "do not artificially narrow scope"
      clause), the parser must cover the entire equipment corpus — so
      both shapes are recognized under one document. The corpus-typical
      row form's kind (Equip vs EquipMod) is inferred from the source
      path (any path containing the substring `equipmods` is
      EquipMod; anything else is Equip).

3. EQUIPMOD records frequently carry deeply nested BONUS: chains like
   `BONUS:COMBAT|AC|1|TYPE=Armor|PREVAREQ:DisableArmorBonus,0`. The
   card body calls out "no unbounded recursion" as a verification
   criterion. The parser flattens each BONUS: clause into a flat list
   of pipe-delimited qualifiers (no tree structure, no recursion).
   Verified by an internal 200-qualifier synthetic chain test that
   would overflow a recursive parser.

4. PCC-side `EQUIP:` / `EQUIPMOD:` directives (which reference
   equipment LST files from PCC files, similar to `SPELL:`) are
   intentionally out of scope for this slice. The PCC parser /
   resolver surface is owned by a separate card. The card body says
   "Parses every EQUIP: line ... in every PCGen LST file" — that is
   the LST surface, not the PCC surface.

Files changed
-------------

- src/pcgen_import/lst_parser/equipment.rs           — NEW (781 LOC).
  Module doc-comment explains the two on-disk shapes and the BONUS:
  flat-chain semantics. Public types:
    - `pub enum EquipmentRecordKind { Equip, EquipMod }`
      with `token()` and `from_token()` canonicalizers.
    - `pub struct EquipmentRecord` — kind, name, header_line_number,
      header_raw_line, tokens, bonus_chains, is_record_start,
      diagnostics.
    - `pub struct EquipmentToken` — key, value, line_number, raw_pair.
    - `pub struct BonusToken` — line_number, raw_bonus, qualifiers
      (flat pipe-split list; no recursion).
    - `pub enum EquipmentDiagnosticKind` —
      `MalformedSD17B5`, `MalformedBonusChain`, `MalformedBlockMarker`,
      `OrphanContinuationRow`, `ReadFailed`.
    - `pub struct EquipmentDiagnostic` — line_number, raw_line, kind,
      message.
    - `pub struct EquipmentParseResult` — source_path, entries,
      diagnostics.
    - `pub fn parse_equipment_entries(source_path, input_text)` —
      the parser entry point; O(n) single pass over `input_text.lines()`.
    - `pub fn parse_equipment_file(path: &Path)` — disk-reading
      convenience; returns the result or a `ReadFailed` diagnostic.
    - `pub fn records_by_kind()` — group records by kind for callers
      that want one kind at a time.
  Plus 5 internal unit tests (bonus-token flattening, name
  extraction, kind inference, directive-prefix recognition).

- src/pcgen_import/lst_parser/mod.rs                  — re-export
  surface extended. New `pub mod equipment;` + `pub use
  equipment::{...}` for the public types. No collisions with the
  existing class / metadata / race_ability / spell re-exports
  (SD-17 partition recipe: each slice owns its own submodule).

- tests/fixtures/lst/equipment_minimal.lst           — NEW (29 LOC).
  Synthetic fixture exercising both on-disk shapes:
    - Directive-prefix EQUIP: rows in `###Block: Directive-prefix
      equipment rows`.
    - Corpus-typical rows in `###Block: Corpus-typical equipment
      rows`.
    - Empty `###Block:` marker for the MalformedBlockMarker diagnostic.
    - `EQUIP:\tKEY:MalformedTest` for the MalformedSD17B5 diagnostic.
    - Leading `BONUS:` continuation row for the OrphanContinuationRow
      diagnostic.

- tests/fixtures/lst/equipmods_minimal.lst           — NEW (24 LOC).
  Same shape as equipment_minimal.lst but the source path contains
  the substring `equipmods`, so every record lands in the
  EquipMod bucket via path-based inference.

- tests/sd17_b5_equipment.rs                         — NEW (542 LOC).
  21 acceptance tests covering every verification criterion in the
  card body. Includes 2 corpus-gated real-file tests
  (`real_corpus_cr_equip_arms_armor_parses_with_line_numbers_preserved`
  and `real_corpus_cr_equipmods_parses_with_kind_inferred_from_path`)
  that exercise the live PCGen corpus files when
  `PCGEN_CORPUS_ROOT` is set. Both pass on this host.

- programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/sd17-b-5-source-stc-2026-07-12.md
  — NEW (128 LOC). Source STC draft for this slice, written before
  implementation, captures scope decisions and out-of-scope items
  explicitly.

Tests added
-----------

21 acceptance tests in tests/sd17_b5_equipment.rs:

  1.  recognizes_every_equipment_kind_declared_in_the_slice
  2.  rejects_unknown_equipment_kinds
  3.  parses_directive_prefix_equip_rows_with_line_numbers_preserved
  4.  parses_directive_prefix_equipmod_rows_with_line_numbers_preserved
  5.  parses_corpus_typical_equip_rows_with_inferred_kind
  6.  parses_corpus_typical_equip_rows_without_equipmods_substring
  7.  strips_copy_suffix_from_equipment_record_name
  8.  emits_malformed_diagnostic_when_directive_has_no_value
  9.  emits_malformed_diagnostic_for_empty_equipmod_directive
  10. emits_malformed_block_marker_diagnostic_for_empty_label
  11. emits_orphan_continuation_row_diagnostic
  12. parses_deeply_nested_bonus_chain_without_stack_overflow
  13. parses_real_world_bonus_with_prev_qualifier_chain
  14. multiple_bonus_tokens_on_same_record_are_all_captured
  15. ignores_blank_lines_and_comment_lines
  16. parses_a_fixture_from_disk_with_line_numbers_preserved
  17. parses_equipmods_fixture_from_disk_with_kind_inferred_from_path
  18. parse_runs_in_linear_time_on_a_synthetic_large_file
  19. records_by_kind_groups_records_correctly
  20. real_corpus_cr_equip_arms_armor_parses_with_line_numbers_preserved
      (corpus-gated; skipped if PCGEN_CORPUS_ROOT is absent)
  21. real_corpus_cr_equipmods_parses_with_kind_inferred_from_path
      (corpus-gated; skipped if PCGEN_CORPUS_ROOT is absent)

Plus 5 internal unit tests in src/pcgen_import/lst_parser/equipment.rs:
  - bonus_token_flattening_does_not_recurse (200 qualifiers)
  - bonus_token_strips_bonus_prefix
  - extract_record_name_handles_copy_suffix
  - infer_kind_uses_equipmods_substring
  - recognize_directive_prefix_only_matches_equip_or_equipmod

Verification
------------

- `cargo test --test sd17_b5_equipment` → 21 passed, 0 failed.
- `cargo test --locked` (whole suite) → 0 failures across all test
  files. B-1 (15 tests), B-3 (16 tests), B-4 (13 tests), B-6 (12 tests),
  and all prior slices remain green.
- `git diff --cached --check` → no whitespace errors.
- `git merge-tree $(git merge-base origin/tranche/2-7 HEAD) origin/tranche/2-7 HEAD` →
  empty conflict markers. Merge to tranche/2-7 is conflict-free.

Non-goals honored
-----------------

- src/rules_core/ unmodified.
- Canonical IR shape unchanged — records stay attached to their LST
  source rows.
- UI untouched.
- SD-13 matrix file untouched.
- Release lane not invoked.
- Scope not artificially narrowed: both on-disk shapes (directive-
  prefix + corpus-typical) are recognized across every PCGen LST
  file of the equipment kind.

Doctrinal notes
---------------

- SD-17 partition recipe honored: equipment.rs owns its own
  EquipmentDiagnostic + EquipmentDiagnosticKind types; the umbrella
  EquipmentRecordKind is a NEW public type (not a re-export of
  the umbrella LstDiagnosticKind), so no collision with the metadata
  slice's umbrella name.
- Slice branch topology honored: branch off origin/tranche/2-7,
  PR target = tranche/2-7. No develop detour.
- Single slice, single purpose, single submodule, single PR. The
  follow-up "PCC EQUIP:/EQUIPMOD: directive vocabulary extension"
  is intentionally tracked as a separate slice, not folded into
  this one.

Let it be recorded.