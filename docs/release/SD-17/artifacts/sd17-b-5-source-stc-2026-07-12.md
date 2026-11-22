# SD-17-B-5 Source STC

epic: SD-17 (PCGen corpus include-graph resolution)
slice: SD-17-B-5
slice_role: LST parser — equipment (EQUIP:, EQUIPMOD:)
assignee: tech-priest
parent_gate: t_dd3dacbd
parent_tranche: tranche-2-7
date: 2026-07-12
base_sha: 94c1bb7 (origin/tranche/2-7 tip, post Slice A + B-1/B-3/B-4/B-6 merges)
branch: feat/sd17-slice-b-5-equipment (off origin/tranche/2-7)
pr_target: tranche/2-7

## Scope summary

Parses every PCGen LST file of the equipment kind, recognizing equipment
records (`EQUIP:` directive-prefix form, plus the corpus-typical
name-leading form used in `*_equip*.lst` files) and equipment-modifier
records (`EQUIPMOD:` directive-prefix form, plus the corpus-typical
name-leading form used in `*_equipmods.lst` files). Equipment-modifier
records carry deeply nested `BONUS:` chains; the parser flattens them
into a list of bonus tokens (no recursion, no stack overflow).

Covers the entire PCGen equipment corpus — every `*_equip*.lst` and
`*_equipmods.lst` file under `pcgen/data/`, regardless of which system
(Pathfinder, D&D 3.5e, Starfinder, Pathfinder 2e, homebrew, 5e, etc.).
The parser does not artificially narrow scope per size-of-work grounds
(operator directive 2026-07-12).

## Specific action

1. Create `src/pcgen_import/lst_parser/equipment.rs` — the equipment
   parser module, partitioned into its own submodule per the
   parallel-slice-module-partition doctrine (SD-17 4-way collision
   worked example, 2026-07-12). Defines:
   - `pub enum EquipmentRecordKind { Equip, EquipMod }`
   - `pub struct EquipmentRecord` — one equipment or equipmod record
     with `kind`, `name`, `header_line_number`, `header_raw_line`,
     `tokens: Vec<EquipmentToken>`, `bonus_chains: Vec<BonusToken>`,
     `is_record_start: bool`, and `diagnostics: Vec<EquipmentDiagnostic>`.
   - `pub struct EquipmentToken` — one tab-delimited `KEY:VAL` token
     lifted from the record's primary line.
   - `pub struct BonusToken` — one `BONUS:` clause lifted from the
     record (non-recursive; nesting handled by repeated bonus tokens
     on the same record).
   - `pub enum EquipmentDiagnosticKind { MalformedSD17B5,
     MalformedBonusChain, UnleveledFeatureLine, ReadFailed }`
   - `pub struct EquipmentDiagnostic`
   - `pub struct EquipmentParseResult`
   - `pub fn parse_equipment_entries(source_path, input_text) -> EquipmentParseResult`
   - `pub fn parse_equipment_file(path: &Path) -> Result<EquipmentParseResult, EquipmentDiagnostic>`
2. Wire the new submodule into `src/pcgen_import/lst_parser/mod.rs`
   via `pub mod equipment;` and `pub use equipment::{...};` for the
   public types.
3. Extend the PCC parser (`src/pcgen_import/pcc.rs`) to recognize
   `EQUIP:` and `EQUIPMOD:` PCC directives. These directives declare
   equipment / equipment-modifier LST files in PCC files (e.g.
   `EQUIPMOD:cr_equipmods.lst`). Add the directive variants to the
   PCC directive enum and a corresponding resolver hookup in
   `include_resolver.rs` so the equipment LST files are emitted in
   the resolver's flat LST reference list.
4. Create `tests/sd17_b5_equipment.rs` — acceptance tests covering
   every verification criterion in the card body.
5. Create `tests/fixtures/lst/equipment_minimal.lst` — a hand-built
   synthetic fixture with deterministic line numbers.

## Allowed write scope

- `src/pcgen_import/lst_parser/equipment.rs` (new)
- `src/pcgen_import/lst_parser/mod.rs` (re-export line added)
- `src/pcgen_import/pcc.rs` (extend directive vocabulary)
- `src/pcgen_import/include_resolver.rs` (route EQUIP:/EQUIPMOD: edges)
- `tests/sd17_b5_equipment.rs` (new)
- `tests/fixtures/lst/equipment_minimal.lst` (new)
- `programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/sd17-b-5-tech-priest-merge-receipt-2026-07-12.md` (new, post-merge)
- `programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/artifacts/.github/branch-protection-rulesets/tranche-2-7.json` (if not already present)

Read-only: existing GE-03 PCC parser surface (non-EQUIP directives),
Slice A resolver, prior Slice B submodules (class, race_ability, spell,
metadata), `src/rules_core/`.

## Verification

- `cargo test --test sd17_b5_equipment -- --nocapture` passes every
  test in the new test file.
- `cargo test --locked` (whole suite) passes — no regressions in
  any of the merged B-* slices.
- A malformed-equipment-entry test produces a `MalformedSD17B5`
  diagnostic carrying the source line number and the raw line text.
- A representative corpus file (`cr_equip_arms_armor.lst` under
  `PCGEN_CORPUS_ROOT`) parses deterministically when the corpus is
  available; the test is skipped (not failed) when the corpus root
  is not present.
- Every parsed record carries its one-based source line number and
  its raw line text. A round-trip test asserts line numbers match
  the source file exactly.
- A 5000-record synthetic file parses in under 2 seconds (O(n)
  binding).
- A `BONUS:` chain with 100+ nested `|PREVAREQ:` clauses flattens
  into a list of bonus tokens without stack overflow.

## Non-goals

- Do not modify `src/rules_core/`.
- Do not redesign the canonical IR shape — records stay attached to
  their LST source rows.
- Do not touch UI.
- Do not modify the SD-13 matrix file.
- Do not invoke the release lane.
- Do not artificially narrow scope per size-of-work grounds.

## Doctrinal notes

- The card body's `branch: develop` and `base_sha: c78287c + Slice A
  merge tip` are stale. Per `devops/tranche-branch-governance`, slice
  branches target `tranche/2-7`. The base SHA is `94c1bb7` (current
  `origin/tranche/2-7` tip, which already includes Slice A and
  B-1/B-3/B-4/B-6 parser surfaces).
- Equipment files use `###Block:` structural markers. The parser
  handles those without colliding with the class parser's block-marker
  logic — each slice owns its own diagnostic enum under its own
  submodule.
- Equipment-modifier `BONUS:` chains in real PCGen files can be 10+
  pipes deep with nested `|PREVAREQ:` and `|PRETYPE:` qualifiers. The
  parser treats each `BONUS:` token as an atomic clause (no recursion).
- PCC `EQUIP:` / `EQUIPMOD:` directives are file-references (like
  `SPELL:`), not record declarations. They must be captured by the
  PCC parser / resolver so the equipment LST files get discovered.