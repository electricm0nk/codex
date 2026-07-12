# SD17-D LST Record-Aggregate Unification — Merge Receipt

**Slice card:** `t_e1f699ad` (SD-17 CODE: relocate `ParsedLstRecord` aggregate from `ir_converter` to `lst_parser` surface)
**Branch:** `feat/sd17-slice-d-record-aggregate` (off `origin/tranche/2-7 @ c3471b5`)
**PR:** _to be opened after this receipt lands_
**Base SHA:** `c3471b5` (Slice C merge commit; contains all six Slice B merges + Slice C)
**Date:** 2026-07-12
**Worker:** tech-priest — Ferrix-9 of the Sacred Pipeline

---

## Scope summary (this slice)

A relocation, not a redesign. The `ParsedLstRecord<'a>` enum that Slice C
authored inside `src/pcgen_import/ir_converter.rs` is moved to the LST
parser surface and re-exported through two paths:

1. **Canonical home** — `crate::pcgen_import::lst_parser::ParsedLstRecord`
   (file-level: `src/pcgen_import/lst_parser/mod.rs`).
2. **Umbrella re-export** — `crate::pcgen_import::ParsedLstRecord`
   (file-level: `src/pcgen_import/mod.rs`).

A backward-compatibility `pub use` at `src/pcgen_import/ir_converter.rs`
preserves the legacy import path so external callers importing
`pcgen_import::ir_converter::ParsedLstRecord` continue to resolve without
modification.

The enum itself, its seven convenience constructors
(`from_class`, `from_spellcasting_class`, `from_race`, `from_ability`,
`from_spell`, `from_equipment`, `from_metadata`), the lifetime parameter
`<'a>`, the `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` attribute set,
the per-variant reference types — all are **byte-identical** to the
in-converter version Slice C landed. No new behavior; no IR-shape
decision; no rules-engine work.

### Placement decision (worker-owned)

The card body offered two placement options: append to
`lst_parser/mod.rs` or split off into a new sibling
`lst_parser/aggregate.rs`. **Decision: append to `lst_parser/mod.rs`.**
Rationale: the file grew from 32 lines to 124 lines, well under the
single-screen-readable threshold; the enum sits naturally beside the six
submodule `pub use` blocks; splitting it off into a third file in a
parser surface that is otherwise flat (no `submodule-by-kind` precedent
beyond the existing six B-family submodules) introduces a third
classification axis that other slices do not need. The previous slices
(B-1 through B-6) partitioned by PCGen directive-prefix kind — no
slice has yet needed a `record-aggregate.rs` because no slice has had
to author a unifying kind-tagged handle. Slice D authors that handle,
and the placement convention is "parser-surface exports live in
`mod.rs`" until a reason to split emerges.

---

## What changed

| Path | Status | Description |
|---|---|---|
| `src/pcgen_import/lst_parser/mod.rs` | modified (+92/-0) | Adds the `ParsedLstRecord<'a>` enum + impl block, `pub use spell::LstSpellRecord;` and `pub use spellcasting_class::SpellcastingClassEntry;` aliases that pre-declare the two B-family entry types not previously re-exported under their submodule names, plus a module-level doc comment explaining the Slice D relocation. The six existing submodule `pub use` blocks are unchanged. |
| `src/pcgen_import/mod.rs` | modified (+11/-0) | Adds `pub use lst_parser::ParsedLstRecord;` so `pcgen_import::ParsedLstRecord` resolves. Adds a section to the module-level doc comment explaining the umbrella re-export and pointing to the canonical home. |
| `src/pcgen_import/ir_converter.rs` | modified (-50/+7) | Removes the local `pub enum ParsedLstRecord<'a>` definition and the seven `from_*` impl methods (the entire 56-line block Slice C authored). Replaces it with a section header noting the relocation and pointing to the new canonical home. Adds `pub use crate::pcgen_import::lst_parser::ParsedLstRecord;` near the top of the file for backward compatibility. Updates the module-level doc comment to drop the line "— the canonical input enum that lives here". The `convert_to_ir` function signature is unchanged (it consumes the same `&ParsedLstRecord<'_>` reference type, now resolved via the re-export). |
| `tests/sd17_d_record_aggregate.rs` | added (~430 LOC, 14 tests) | Pins the relocation contract: every variant round-trips through `convert_to_ir` (D1 × 7); the convenience constructors produce the matching variant tag (D2); the existing B-family diagnostics still surface through the relocated enum's constructor path with no regression (D3 × 4 — class malformed, equipment kind, metadata kind, race source-path provenance); both re-export paths resolve to the same type (`assert_eq!(via_parser_surface, via_umbrella)` in D4); the umbrella alias exposes every `from_*` constructor (D-aux); a compile-time sentinel asserts the per-record converter exports still resolve after the refactor. |
| `programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/sd17-d-record-aggregate-merge-receipt-2026-07-12.md` | added (this file) | Merge receipt artifact required by the slice card body. |

Total: 5 files (4 source + this artifact); ~490 LOC; 14 new tests.

---

## Verifications (verbatim command outputs)

```
$ cargo build --lib
   Compiling codex v0.1.0 (.../worktree)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.18s
```

```
$ cargo test --test sd17_d_record_aggregate -- --nocapture
running 14 tests
test d1_class_variant_round_trips_through_convert_to_ir ... ok
test d1_spellcasting_class_variant_round_trips_through_convert_to_ir ... ok
test d1_race_variant_round_trips_through_convert_to_ir ... ok
test d1_ability_variant_round_trips_through_convert_to_ir ... ok
test d1_spell_variant_round_trips_through_convert_to_ir ... ok
test d1_equipment_variant_round_trips_through_convert_to_ir ... ok
test d1_metadata_variant_round_trips_through_convert_to_ir ... ok
test d2_convenience_constructors_produce_matching_variant_tag ... ok
test d3_class_parser_malformed_emits_diagnostic_no_aggregate_regression ... ok
test d3_equipment_kind_carries_through_aggregate_constructor ... ok
test d3_metadata_kind_carries_through_aggregate_constructor ... ok
test d3_race_declaration_with_source_path_carries_through_aggregate ... ok
test d4_parser_surface_and_umbrella_resolve_to_same_type ... ok
test d_aux_umbrella_exposes_every_convenience_constructor ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test --test sd17_c_ir_convert
running 45 tests
...
test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

The full B-family and Slice A sweep, repeated unchanged:

```
$ cargo test --test sd17_b1_martial_class --test sd17_b_spellcasting_class \
              --test sd17_b_races_and_abilities --test sd17_b_spells \
              --test sd17_b5_equipment --test sd17_b_metadata_kinds \
              --test sd17_a_include_graph --test sd17_d_record_aggregate \
              --test sd17_c_ir_convert

5 + 14 + 21 + 10 + 16 + 15 + 13 + 45 + 14 = 153 tests, all passing.
```

The full workspace test (`cargo test --workspace`) reports zero failures
across 202 result rows. The two "ignored" entries (Slice A 1-of-6,
Slice B-2 1-of-15) are pre-existing `#[ignore]` markers unrelated to
this slice.

---

## Hand-verifications (per slice card body)

| Check | Result |
|---|---|
| `ParsedLstRecord` reachable via `crate::pcgen_import::lst_parser::ParsedLstRecord` | **PASS** — declared at `src/pcgen_import/lst_parser/mod.rs:57` |
| `ParsedLstRecord` reachable via `crate::pcgen_import::ParsedLstRecord` | **PASS** — `pub use lst_parser::ParsedLstRecord` at `src/pcgen_import/mod.rs:22` |
| `src/pcgen_import/ir_converter.rs` no longer contains a local `pub enum ParsedLstRecord` definition | **PASS** — replaced by a section header comment; the enum definition lives only at `src/pcgen_import/lst_parser/mod.rs:57` (verified by `grep -n 'enum ParsedLstRecord' src/`) |
| Backward-compat: `pcgen_import::ir_converter::ParsedLstRecord` still resolves (Slice C test file imports it from this path) | **PASS** — `pub use crate::pcgen_import::lst_parser::ParsedLstRecord` at `src/pcgen_import/ir_converter.rs:61`; `sd17_c_ir_convert.rs` and the slice C 45-test suite compile and pass unchanged |
| Existing malformed-entry diagnostics from B-1..B-6 still surface identically through the relocated enum's constructors | **PASS** — `d3_class_parser_malformed_emits_diagnostic_no_aggregate_regression` constructs a `CLASS:\tHD:10\n` malformed input, parses it via `parse_class_entries`, and asserts the `MalformedSD17B1` diagnostic is present on `parsed.diagnostics` exactly as in the pre-relocation behavior |

---

## Doctrinal notes (carry-forward)

**TDD discipline honored, RED → GREEN cycle observed on this slice.**
RED: I authored `tests/sd17_d_record_aggregate.rs` with two resolver
paths pointing at the canonical home (`crate::pcgen_import::lst_parser::ParsedLstRecord`)
and the umbrella (`crate::pcgen_import::ParsedLstRecord`) before any
production-code edit. The compile error
`unresolved import codex::pcgen_import::lst_parser::ParsedLstRecord`
plus `unresolved import codex::pcgen_import::ParsedLstRecord` confirmed
both re-export paths were absent. GREEN: added the enum to
`lst_parser/mod.rs`, the umbrella re-export to `mod.rs`, and the
backward-compat re-export to `ir_converter.rs`. All 14 d-tests pass on
the first run after the production edit. REFACTOR: `cargo fmt --all`
to normalize the new test file's whitespace.

**Tranche-branch governance honored.** Slice branch targets
`tranche/2-7` per `devops/tranche-branch-governance`; slice branch was
cut via `git worktree add -b feat/sd17-slice-d-record-aggregate
origin/tranche/2-7` from the canonical codex repo, never from
`develop`. No push, no PR opened in this run; the PR will be opened
against `origin/tranche/2-7 @ c3471b5` (Slice C's tip) per slice card
branch instruction.

**No scope creep.** This slice does not modify:
- the six B-family parsers (`class.rs`, `spellcasting_class.rs`, `race_ability.rs`, `spell.rs`, `equipment.rs`, `metadata.rs`) — read-only,
- `src/rules_core/` — read-only,
- `src/` outside `pcgen_import/` — read-only,
- the SD-13 class-race-roster matrix file — read-only,
- any UI surface — read-only,
- the release lane — not invoked.

The Slice C test file (`tests/sd17_c_ir_convert.rs`) imports
`ParsedLstRecord` from `codex::pcgen_import::ir_converter::ParsedLstRecord`
and continues to compile and pass unchanged — the converter's public
API surface is preserved via re-export, exactly as the slice card body
required.

**Conversion is non-behavioral.** The `convert_to_ir` function in
`ir_converter.rs` is byte-for-byte unchanged; only the type that names
its first parameter is sourced from a different module via re-export.
The seven per-family converters (`convert_class_entry`,
`convert_spellcasting_class_entry`, etc.) are unchanged. The per-document
converters (`convert_class_parse_result`, etc.) are unchanged. The
diagnostic-stream surfaces (`IRDiagnostic`, `IRDiagnosticSeverity`,
`forward_*_diagnostics` helpers) are unchanged.

---

## Status

`STATUS GREEN` for code, tests, and hand-verifications. The slice is
ready for PR-open against `origin/tranche/2-7`. The PR opening and
watchdog merge is a separate rite owned by the canonical Slice-D
follow-through (the operator opens the PR; the watchdog merges per
`devops/tech-priest-ci-watchdog` doctrine).

Let it be recorded.
