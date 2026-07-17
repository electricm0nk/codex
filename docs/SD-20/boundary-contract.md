---
title: SD-20 — Boundary Contract (Epic 1)
status: closed (cycle 4 of Epic 1 landed 2026-07-17; CharacterInput permutations, PilotReceipt types, the printed-sheet cell map, and the boundary-contract parity fixture all landed — Epic 1 fully closed)
mirrors: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/technical-design.md §1
---

# SD-20 boundary contract

This is the boundary-contract artifact named by
`SD-20-rules-engine-completeness-scope-draft.md` §1.1 and
`technical-design.md` §1.1. It is the engine-side API surface every other
SD-20 epic produces into: the `CharacterInput` shapes the engine accepts,
the `PilotReceipt` shape it returns, and the printed-sheet cell map the
GUI renders from.

Per `technical-design.md` §1.3: no subsystem engine (epics 2–5), no Level
Up grant (epic 7), and no integration closure (epic 8) may introduce a new
`CharacterInput` field or a new `PilotReceipt` field without first
extending this contract and adding the parity test fixture at
`tests/fixtures/wire/sd20/<criterion>.json`.

This artifact landed progressively across Epic 1's four cycles (per the
loop instruction's Step 2: `CharacterInput` types first, then
`PilotReceipt` types, then the printed-sheet cell map, then the first
parity fixture for the boundary contract itself). All four sections below
are now landed; Epic 1 is closed as of cycle 4.

## 1. Inputs — what the engine accepts

Landed (cycle 1, commit pending at time of writing — see
`~/workspace/SD-20-rules-engine-completeness-progress.md` for the
commit SHA).

The engine's existing, SD-19-shaped `CharacterInput` type
(`src/rules_core/character_input.rs`) is the boundary contract's
`CharacterInput` type. This cycle does not introduce a new, parallel
struct — `class_levels: Vec<CharacterClassLevel>` already carries
brand-new, mid-build, and multiclass character state without a shape
change; what the contract adds is a *classification* over that existing
shape, since "which of the three canonical permutations is this input"
was previously only prose.

`src/rules_core/contract.rs` (NEW module) adds:

- `CharacterInputPermutation` — an enum with three variants:
  - `BrandNew` — zero or one class level at level ≤ 1, and no player
    choices recorded yet (no feats, skill ranks, equipment selections,
    spell selections, or other selected choices).
  - `MidBuild` — a single class, but with at least one player choice
    recorded, or a class level above 1.
  - `Multiclass` — more than one entry in `class_levels` (any subset of
    the 11 core classes at any class-level distribution). Multiclass
    takes precedence over mid-build: a multiclass character with feats
    selected still classifies as `Multiclass`.
- `classify_character_input(input: &CharacterInput) -> CharacterInputPermutation`
  — the classification function.

RED/GREEN test: `tests/sd20_contract_character_input.rs` (7 cases, one
per permutation-boundary condition described above).

## 2. Outputs — what the engine returns (`PilotReceipt`)

Landed (cycle 2). `src/rules_core/contract.rs` adds:

- `PilotReceipt` — a struct with three fields, per
  `technical-design.md` §1.1 "Outputs":
  - `chassis: PilotBaseChassisComputation` — per-derived-stat fields
    (BAB, saves, HP, AC, attack bonus, ability mods, selected skill
    modifiers). The unchanged chassis computation from
    `src/rules_core/pilot_compute.rs`; no re-derivation.
  - `corpus_derived: CorpusDerivedSection` — per-source-record fields
    with `TableCellRef` provenance (spell-school coverage, resolved
    equipment). The unchanged corpus-derived section from
    `src/rules_core/pilot_compute_corpus.rs`'s `CorpusPilotReceipt`.
  - `diagnostics: Vec<ComputationDiagnostic>` — diagnostic fields,
    hoisted from the chassis computation's own `diagnostics` field to
    the receipt's top level unchanged. `claim_blocking: true`
    diagnostics (e.g. `class_chassis.unsupported`) remain
    `claim_blocking: true`.
- `to_pilot_receipt(receipt: &CorpusPilotReceipt) -> PilotReceipt` — the
  builder function. Wraps the existing corpus-aware compute seam's
  output (`compute_pilot_with_corpus` in `pilot_compute_corpus.rs`);
  this cycle does not introduce a new, parallel receipt-computation
  path. As with the "Inputs" section's `CharacterInputPermutation`, the
  precedent shapes already in this repo —
  `PilotBaseChassisComputation` (`src/rules_core/pilot_compute.rs`) and
  `CorpusPilotReceipt` (`src/rules_core/pilot_compute_corpus.rs`) — are
  what `PilotReceipt` composes with, not shapes it duplicates from
  scratch.

RED/GREEN test: `tests/sd20_contract_pilot_receipt.rs` (3 cases: chassis
section matches `compute_pilot_base_chassis` called directly,
corpus-derived section matches the seam's own section unmodified,
diagnostics preserve `claim_blocking: true` for an unsupported chassis
posture).

## 3. Cells — what the GUI prints (printed-sheet cell map)

Landed (cycle 3). `src/rules_core/contract.rs` adds:

- `PrintedSheetCell` — a struct with `cell_id` (stable id, e.g.
  `sheet.base_attack_bonus`), `source_field` (the exact `PilotReceipt`
  field path this cell renders, e.g. `chassis.base_attack_bonus`, for
  auditability), and `value: PrintedSheetCellValue`.
- `PrintedSheetCellValue` — an enum: `Number(i16)` for a real computed
  value, or `Blocked` — the "blocked — see diagnostics" rendering per
  `technical-design.md` §1.1 — for a cell whose source field is
  claim-blocked. The GUI cannot invent a value; it renders exactly what
  this map gives it.
- `printed_sheet_cell_map(receipt: &PilotReceipt) -> Vec<PrintedSheetCell>`
  — builds the row-by-row map from a `PilotReceipt`. Fifteen cells land
  in this cycle: base attack bonus, the three total saves, the
  deterministic baseline armor class and melee attack bonus, the three
  selected skill modifiers (Climb, Intimidate, Swim), and the six ability
  modifiers. The nine chassis-dependent cells (BAB, saves, baseline AC,
  baseline melee attack bonus, selected skill modifiers) render `Blocked`
  when the chassis computation's `class_chassis.unsupported` diagnostic
  is `claim_blocking: true` — those `PilotBaseChassisComputation` fields
  are zeroed (not real data) in that case, so showing the zero as a
  number would be a fabricated value. The six ability-modifier cells are
  computed directly from ability scores independent of chassis support,
  so they are never blocked by `class_chassis.unsupported` alone —
  blanket-blocking every cell whenever any one diagnostic fires would
  itself violate the "no fabricated/imprecise output" requirement in the
  opposite direction (under-reporting real, independently-computed data).

This cycle does not widen the cell set beyond the fifteen chassis-derived
cells above — corpus-derived cells (spell coverage, equipment) are a
future Epic-1 or per-epic cycle's concern once those epics' own outputs
are named in this contract.

RED/GREEN test: `tests/sd20_contract_cell_map.rs` (2 cases: a genuinely
supported single-class `class:fighter` level-1 posture renders `Number`
for every cell with the value matching the receipt's own field; a
wizard-only posture — `class_chassis.unsupported` claim-blocking — renders
`Blocked` for all nine chassis-dependent cells while the six
ability-modifier cells still render their real `Number` values).

## 4. Parity fixture — the boundary contract's own wire-fixture

Landed (cycle 4). The first wire-fixture parity JSON per
`technical-design.md` §1.2's format
(`{ "name", "input", "expected_output", "expected_diagnostics" }`) lands
at `tests/fixtures/wire/sd20/boundary_contract_parity.json`. It names a
brand-new, no-selections `human`/`class:fighter` level-1 character and
exercises the *whole* boundary-contract round trip in one fixture:
`classify_character_input` (expects `BrandNew`) into the existing
corpus-aware compute seam (`compute_pilot_with_corpus`, empty corpus)
into `to_pilot_receipt` into `printed_sheet_cell_map` — all fifteen cells
from §3 above, plus the chassis's `ability_modifiers` / `base_attack_bonus`
/ `base_saves` / `baseline_melee_attack_bonus` / `baseline_armor_class` /
`total_saves` / `selected_skill_modifiers`, the (empty, for this input)
`corpus_derived` section, and the two `claim_blocking: true` diagnostics
this exact input produces (`combat.baseline_unsupported`,
`skill.selected_modifier.unsupported` — the deterministic Longsword/Chain
Shirt/Dodge/selected-skill posture this input does not opt into; see
`pilot_compute.rs`'s own diagnostics for why those two, and not
`class_chassis.unsupported`, fire for a *supported* Fighter chassis).

`tests/sd20_contract_boundary_parity.rs` reads the fixture from disk (this
crate has no `serde`/`serde_json` dependency — `Cargo.toml`'s
`[dependencies]` table is empty, and adding one is out of Epic 1's
file-touch partition — so the test carries a small self-contained,
`std`-only JSON reader scoped to this one test file), builds the engine's
real `CharacterInput` from the fixture's `input` section, computes the
real `PilotReceipt` and cell map, and asserts exact parity against the
fixture's `expected_permutation` / `expected_output` /
`expected_diagnostics`. This is the pattern the GUI's own render tests
would follow against the same on-disk file per `technical-design.md`
§1.2 ("The GUI's render tests read the same files and assert each cell
renders exactly the corresponding value").

This closes Epic 1: all four work-units (`CharacterInput` types,
`PilotReceipt` types, printed-sheet cell map, and this parity fixture)
are landed. Per the loop instruction's dependency graph, Epics 2
(spellbook), 3 (feat prereqs), 4 (skill ranks), and 5 (equipment effects)
are eligible as parallel streams starting the next cycle.

## Cross-reference

- `~/workspace/SD-20-rules-engine-completeness-scope-draft.md` §1.1 — Epic 1 acceptance criterion.
- `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/technical-design.md` §1 — contract shape, parity test format, cross-cutting authority surface.
- `~/workspace/SD-20-rules-engine-completeness-progress.md` — cycle log (commit SHAs, evidence-tier transitions).
