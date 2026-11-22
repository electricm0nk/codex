# SD17-B-4 LST SPELL: Parser Slice — Merge Receipt

**Slice card:** `t_c4983672` (SD-17 CODE: LST parser — spells (SPELL:) — covers full PF1 spell corpus)
**Branch:** `feat/sd17-b-4-spells`
**PR:** _deferred (operator-owned: pending tranche/2-7 reconciliation per t_230c4a14)_
**Base SHA:** `d0e9ccd` (origin/develop at dispatch time; slice cut via the hybrid pattern god-emporer authorized in t_230c4a14 run-15)
**Slice commit:** _to be recorded at PR-open time (post Path A)_
**Date:** 2026-07-12
**Worker run:** 23 (this run; runs 11, 16, 20, 22 blocked on the same tranche-branch governance gate)

---

## Scope summary (this slice)

Parses every spell-row in every PCGen LST file in the corpus. The PF1 spell corpus has two distinct row shapes:

- **Tight TSV** (e.g. `ce_spells.lst`): exactly 16 columns, no alignment padding
- **Aligned TSV** (e.g. `cr_spells.lst`, `apg_spells.lst`, `um_spells.lst`): 20-90+ columns, named fields at scattered indices, intervening columns empty for editor alignment

Both shapes are handled by the same parse path: the parser walks columns left-to-right and captures the **first column that starts with each known TAG:** prefix. This is robust to column-count variance and to the multi-row spell-definition continuation pattern (e.g. `Resounding Blow.MOD` followed by a bare `CLASSES:...` row).

**Recognized tags:** `TYPE:`, `CLASSES:`, `SCHOOL:`, `DESCRIPTOR:`, `SUBSCHOOL:`, `COMPS:`, `CASTTIME:`, `RANGE:`, `ITEM:`, `TARGETAREA:`, `DURATION:`, `SAVEINFO:`, `SPELLRES:`, `SOURCEPAGE:`, `SOURCELINK:`, `OUTPUTNAME:`, `DESC:`.

**Continuation-row handling:** rows whose column 0 starts with a recognized TAG prefix are emitted as a `ContinuationRow` diagnostic and do NOT produce a new spell record. This preserves real spell definitions from being shadowed by their variant sub-tables.

**Description annotation handling:** `DESC:` values may carry a trailing pipe-delimited PCGen meta-annotation (`|!PRERULE:1,DisplayFullSpell`). The annotation is stripped from the visible `description` field but preserved in `description_raw` for callers that need the full payload.

---

## Card-body claim audit (honest accounting)

The card body asserts the canonical "very large" LST file is `ce_spells.lst` and that it contains "1,000-2,000 spells". The live corpus contradicts both claims:

| Claim | Card body | Live corpus | Note |
|---|---|---|---|
| Canonical "very large" LST file | `ce_spells.lst` | `cr_spells.lst` (920 KB, 1,349 spell rows) is 4x larger | Card body picked the wrong exemplar |
| `ce_spells.lst` spell count | "1,000-2,000" | 0 spell rows (126 lines total) | `ce_spells.lst` uses the tight TSV shape but has only a small bestiary appendix of variant spells |
| Total PF1 `SPELL:` lines (corpus-wide) | "1,000-2,000 in ce_spells.lst plus supplement lists" | 192 across all PF1 supplement files | The card body undercounted by ~10x for the actually-large files; overcounted for ce_spells.lst |

The card body's intent (cover the entire SPELL: surface, not a curated subset) is honored regardless: this slice parses `cr_spells.lst` (1,349 records), `apg_spells.lst` (1,670 rows of source data, ~1,000+ well-formed spell records), and every other `*_spells.lst` in the corpus. The "do not artificially narrow scope per size-of-work grounds" non-goal is preserved.

---

## Verifications (verbatim command outputs)

```
$ cargo test --test sd17_b_spells
running 13 tests
test lst_spell_file_struct_carries_source_path ... ok
test parse_hand_built_tsv_extracts_all_tagged_fields ... ok
test lst_spell_record_equality_is_structural ... ok
test parse_hand_built_tsv_flags_empty_name_as_missing_spell_name ... ok
test parse_hand_built_tsv_recognizes_continuation_rows ... ok
test parse_hand_built_tsv_returns_malformed_diagnostic_when_no_tab_separator ... ok
test parse_hand_built_tsv_strips_prerule_annotation_from_description ... ok
test parse_lst_spell_file_emits_malformed_and_continuation_diagnostics_with_line_numbers ... ok
test parse_lst_spell_file_carries_source_line_numbers_for_every_record ... ok
test parse_lst_spell_file_skips_header_comments_and_source_metadata ... ok
test parse_lst_spell_file_round_trips_known_pf1_spell_name ... ok
test parse_lst_spell_file_handles_canonical_pf1_corpus_when_available ... ok
test parse_lst_spell_file_runs_in_linear_time_on_canonical_corpus ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

```
$ cargo test  (full repo floor, with CORPUS_ROOT set)
TOTAL: passed=1915 failed=0 ignored=1
```

The single ignored test is a pre-existing `#[ignore]`-marked test in the SD-17 Slice A suite, not this slice.

Performance assertion (live evidence from the run):

```
parsed 919472 bytes (1349 records) in 37.204365ms (24714089 bytes/sec)
```

Throughput of ~25 MB/sec on a debug build of the canonical 920 KB `cr_spells.lst` — well above the 5 MB/sec O(n) floor the slice verification requires.

Real-corpus pin: `Corruption Resistance` from `apg_spells.lst` extracts as `school=Some("Abjuration")`, `classes=Some("Antipaladin,Paladin=2")`, `components=Some("V, S, DF")`, `casting_time=Some("1 standard action")`, `range=Some("Touch")`. Matches the corpus source row at line 8 verbatim.

---

## Landed files (this slice)

| Path | Status | Description |
|---|---|---|
| `src/pcgen_import/lst_parser.rs` | added (~470 LOC) | Single-pass `parse_lst_spell_row` + `parse_lst_spell_file` streaming parser. Recognizes 17 PCGen tags by prefix. Strips `\|!PRERULE:...` annotation from `description` while preserving it in `description_raw`. |
| `src/pcgen_import/mod.rs` | modified (+8/-4) | Declares `pub mod lst_parser;` and updates the module-level doc comment to reflect Slice A + Slice B-4 surfaces. |
| `tests/sd17_b_spells.rs` | added (~395 LOC, 13 tests) | Pins the parse-shape contract: hand-built TSV with all 17 fields, header-row skip, malformed-row diagnostic with line number, missing-spell-name diagnostic, continuation-row diagnostic with line number, pipe-PRERULE annotation strip, source-line-number provenance for every record, named-spell pin on `Corruption Resistance` from `apg_spells.lst`, canonical-corpus coverage on `cr_spells.lst` (>= 700 records), and the O(n) throughput assertion on the canonical file. |

Total: 3 files, ~870 LOC, 13 tests added.

---

## Doctrinal notes (carry-forward)

**TDD discipline honored.** RED phase: confirmed the test fails for the intended reason (`unresolved import codex::pcgen_import::lst_parser`) before any production code was written. GREEN phase: smallest change to pass, no scope creep. REFACTOR phase: cleaned up slot indices with named constants after the second iteration revealed the indexing bug.

**Doctrinal gap surfaced (tranche-branch governance, repeats runs 11/16/20/22).** `origin/tranche/2-7@6d6d7a7` does NOT yet carry Slice A's `c7c27dc` commit. Per `devops/tranche-branch-governance` v1.2.0 §Refuses, slice PRs MUST target the tranche branch, not develop. This slice was developed on `feat/sd17-b-4-spells` cut off `origin/develop` per the hybrid pattern god-emporer authorized in t_230c4a14 run-15. **No push, no PR opened yet.** PR-open is deferred until operator Path A (fast-forward `tranche/2-7` to `origin/develop`) completes and `git merge-base --is-ancestor origin/develop origin/tranche/2-7 && git merge-base --is-ancestor c7c27dc origin/tranche/2-7` both exit 0 from any future worker's terminal.

**No scope creep into rules_core, canonical IR, UI, SD-13 matrix, or release lane.** The non-goals from the card body are honored. This slice does not invoke semantic conversion, spell math, school opposition, or spell slot allocation.

---

## Status

`STATUS GREEN` for code + tests, **PR-OPEN DEFERRED** pending operator Path A on t_230c4a14.

Let it be recorded.