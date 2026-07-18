---
title: SD-20 — Boundary Contract (Epic 1)
status: closed (Epic 1 itself closed cycle 4, 2026-07-17; the Epic 2-7 wiring project — `adaptive-squishing-mccarthy.md` — closed on top of it as of Cycle 7, `integration:epic_wiring_closure`: `to_pilot_receipt` now also composes Epics 2/3/4/5/6's real engines directly into `PilotReceipt`, plus a standalone Epic 7 `compute_level_up_preview` pass-through; see §5 below)
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

Landed (cycle 2; widened by the Epic 2-7 wiring project, cycles 0-6 — see
§5). `src/rules_core/contract.rs` adds:

- `PilotReceipt` — per `technical-design.md` §1.1 "Outputs". Cycle 2
  landed three fields (`chassis`, `corpus_derived`, `diagnostics`); the
  wiring project's cycles 1-4 and 5b added six more (`skills`,
  `spellbook`, `feats`, `equipment_effects`, `weapon_damage`, plus the
  standalone `compute_level_up_preview` function below, which is
  deliberately NOT a `PilotReceipt` field — see §5):
  - `chassis: PilotBaseChassisComputation` — per-derived-stat fields
    (BAB, saves, HP, AC, attack bonus, ability mods, selected skill
    modifiers). The unchanged chassis computation from
    `src/rules_core/pilot_compute.rs`; no re-derivation.
  - `corpus_derived: CorpusDerivedSection` — per-source-record fields
    with `TableCellRef` provenance (spell-school coverage, resolved
    equipment). The unchanged corpus-derived section from
    `src/rules_core/pilot_compute_corpus.rs`'s `CorpusPilotReceipt`.
    **Still unchanged as of the wiring project**: its
    `equipped_items[].derived_stats` stub stays `::default()` forever —
    it lives in `pilot_compute_corpus.rs`, a trunk file SD-20 does not
    touch (see §5's fact 2). Epic 5's real per-item stats are reachable
    instead via the new `equipment_effects` field below.
  - `diagnostics: Vec<ComputationDiagnostic>` — diagnostic fields,
    hoisted from the chassis computation's own `diagnostics` field to
    the receipt's top level unchanged. `claim_blocking: true`
    diagnostics (e.g. `class_chassis.unsupported`) remain
    `claim_blocking: true`.
  - `skills: SkillTotals`, `spellbook: SpellbookCoverage`, `feats:
    Vec<ResolvedFeat>`, `equipment_effects: EquipmentEffects`,
    `weapon_damage: Vec<WeaponDamageBreakdown>` — see §5 for what each
    composes and its cell-map policy.
- `to_pilot_receipt(receipt: &CorpusPilotReceipt, input: &CharacterInput,
  corpus: &SourcePackageContent) -> PilotReceipt` — the builder function.
  Wraps the existing corpus-aware compute seam's output
  (`compute_pilot_with_corpus` in `pilot_compute_corpus.rs`) for
  `chassis`/`corpus_derived`/`diagnostics`; this cycle does not introduce
  a new, parallel receipt-computation path for those three. As with the
  "Inputs" section's `CharacterInputPermutation`, the precedent shapes
  already in this repo — `PilotBaseChassisComputation`
  (`src/rules_core/pilot_compute.rs`) and `CorpusPilotReceipt`
  (`src/rules_core/pilot_compute_corpus.rs`) — are what `PilotReceipt`
  composes with, not shapes it duplicates from scratch. **Signature
  widened** by the wiring project's Cycle 0
  (`contract:receipt_signature_threading`, commit `52ed2ea`) from
  `to_pilot_receipt(receipt: &CorpusPilotReceipt) -> PilotReceipt` to the
  three-argument form above — `input`/`corpus` are what let cycles 1-4
  and 5b actually call Epics 2/3/4/5/6's engines, none of which are
  reachable from `CorpusPilotReceipt` alone.

RED/GREEN test: `tests/sd20_contract_pilot_receipt.rs` (3 cases: chassis
section matches `compute_pilot_base_chassis` called directly,
corpus-derived section matches the seam's own section unmodified,
diagnostics preserve `claim_blocking: true` for an unsupported chassis
posture) — fixed forward to the 3-argument signature by Cycle 0, still
green. §5's own per-field wiring cycles each add their own dedicated
`tests/sd20_contract_<field>_wiring.rs` parity test.

## 3. Cells — what the GUI prints (printed-sheet cell map)

Landed (cycle 3; widened by the wiring project's cycles 1, 2, and 4 — see
§5). `src/rules_core/contract.rs` adds:

- `PrintedSheetCell` — a struct with `cell_id` (stable id, e.g.
  `sheet.base_attack_bonus`), `source_field` (the exact `PilotReceipt`
  field path this cell renders, e.g. `chassis.base_attack_bonus`, for
  auditability), and `value: PrintedSheetCellValue`.
- `PrintedSheetCellValue` — an enum: `Number(i16)` for a real computed
  value, or `Blocked` — the "blocked — see diagnostics" rendering per
  `technical-design.md` §1.1 — for a cell whose source field is
  claim-blocked (or, for the skill cells below, genuinely absent data).
  The GUI cannot invent a value; it renders exactly what this map gives
  it.
- `printed_sheet_cell_map(receipt: &PilotReceipt) -> Vec<PrintedSheetCell>`
  — builds the row-by-row map from a `PilotReceipt`. Cycle 3 landed
  fifteen cells: base attack bonus, the three total saves, the
  deterministic baseline armor class and melee attack bonus, three
  selected-skill-modifier cells (Climb, Intimidate, Swim, originally
  sourced from the chassis's narrow single-posture check), and the six
  ability modifiers. The nine chassis-dependent cells (BAB, saves,
  baseline AC, baseline melee attack bonus, the original three skill
  cells) render `Blocked` when a relevant claim-blocking diagnostic fires
  (`class_chassis.unsupported` universally, plus
  `combat.baseline_unsupported` / `defense.total_save.unsupported` for
  their own specific cells — see `contract.rs`'s diagnostic-id constants)
  — those `PilotBaseChassisComputation` fields are zeroed (not real data)
  in that case, so showing the zero as a number would be a fabricated
  value. The six ability-modifier cells are computed directly from
  ability scores independent of chassis support, so they are never
  blocked by any chassis diagnostic.

  As of the wiring project (§5), three more cell families exist:
  - Cycle 1 (`contract:skill_wiring`) **replaced** (not added to) the
    three original skill cells' source: `sheet.skill.climb` /
    `.intimidate` / `.swim` now source from `receipt.skills.totals`
    (Epic 4's real `allocate_skill_ranks`) instead of the chassis's old
    narrow check, plus two brand-new cells, `sheet.skill.diplomacy` and
    `sheet.skill.disable_device` (five `sheet.skill.*` cells total).
    None of the five is ever blocked by a chassis diagnostic any more
    (`allocate_skill_ranks`'s own diagnostics are all `claim_blocking:
    false`); `Blocked` here means "this character never allocated ranks
    to this skill at all" — honest absence of data, not a diagnostic
    gate.
  - Cycle 2 (`contract:spellbook_wiring`) added a *dynamic* cell family:
    one `sheet.spellbook.slots_total.<level>` /
    `sheet.spellbook.slots_used.<level>` / `sheet.spellbook.spell_save_dc.<class_id>`
    cell per present key in `receipt.spellbook`'s corresponding
    `BTreeMap`. A non-caster (or any character with an empty map)
    produces zero cells of that kind — never a fabricated placeholder.
    These cells are never `Blocked` (`compute_spellbook_coverage` pushes
    no diagnostics at all); absence is expressed purely by the cell not
    existing in the returned `Vec`.
  - Cycle 4 (`contract:equipment_wiring`) added
    `sheet.equipment.armor_class_delta` (always present — a plain `i16`,
    `0` is an honest "no bonus" value, not fabricated) and
    `sheet.equipment.max_dex_cap` (present only when
    `equipment_effects.max_dex_cap` is `Some`; omitted entirely, never
    `Blocked` or a fabricated `Number(0)`, when no cap exists).

  For this repo's canonical tabletop fixture (a fully-supported level-1
  Human Fighter, `tests/fixtures/wire/sd20/human_fighter_level_1_tabletop.json`)
  the map currently produces 19 cells: the 17 fixed cells above (6
  chassis + 5 skill + 6 ability-modifier) plus 0 dynamic spellbook cells
  (this Fighter has no spells) plus 2 equipment cells (this Fighter has a
  Chain Shirt, so both `armor_class_delta` and `max_dex_cap` are
  present).

### Not every epic output becomes a printed-sheet cell (policy, established across cycles 2-6)

`PrintedSheetCellValue` is `Number(i16) | Blocked` only — it cannot
represent a spell list, a feat's prose description, or a fractional
spell-failure percentage without a real type extension, which stayed out
of every wiring cycle's scope (mirroring Epic 1's own original precedent
of deferring corpus-derived cells, cited below). Concretely, deliberately
**NOT** flattened into any cell, and reachable only via the named
`PilotReceipt` field directly:

- `spellbook.spells_prepared`, `spellbook.spells_known`,
  `spellbook.school_specialization` (Cycle 2) — spell lists and a school
  choice, not single numbers.
- `feats[].prerequisites`, `feats[].effects` (Cycle 3) — prose failure
  reasons/descriptions and structured provenance; `PilotReceipt.feats`
  itself adds no cell at all (numeric feat-derived combat bonuses flow
  through the separate `weapon_damage` field's `feat_effects`, not
  through a `sheet.feat.*` cell family).
- `equipment_effects.spell_failure_chance: Option<f32>` (Cycle 4) — a
  fractional percentage does not fit `Number(i16)` cleanly.
- `weapon_damage: Vec<WeaponDamageBreakdown>` in its entirety (Cycle 5b)
  — no summed "damage roll total" formula (base dice + STR + weapon
  enhancement + feat bonuses, combined into one number) exists anywhere
  in this codebase; inventing one to populate a cell would be
  fabrication. The structured per-weapon breakdown stays reachable via
  `receipt.weapon_damage` directly. A future, separate cycle owns turning
  this into a summed display number, if that is ever wanted.
- Epic 7's `LevelUpPlan` in its entirety (Cycle 6) — it is not even a
  `PilotReceipt` field (see §5): it models a level *transition*, not
  snapshot state, so it has no cell-map presence of any kind.

RED/GREEN tests: `tests/sd20_contract_cell_map.rs` (Cycle 3's original 2
cases: a genuinely supported single-class `class:fighter` level-1 posture
renders `Number` for every cell with the value matching the receipt's own
field; a wizard-only posture — `class_chassis.unsupported`
claim-blocking — renders `Blocked` for all chassis-dependent cells while
the six ability-modifier cells still render their real `Number` values);
each wiring cycle's own `tests/sd20_contract_<field>_wiring.rs`; and
`tests/sd20_tabletop_readiness_integration.rs`'s primary test, which
round-trips the full 19-cell map for the canonical fixture and — as of
Cycle 7 — additionally proves every one of the six new `PilotReceipt`
fields agrees byte-for-byte with that field's own epic engine called
directly (see §5).

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

## 5. Epic 2-7 wiring — the six new `PilotReceipt` fields (wiring project, closed)

Epic 1's boundary contract (§1-§4 above) defined `PilotReceipt` and
`printed_sheet_cell_map`, but for a long stretch of SD-20's history
nothing in Epics 2-7 (spellbook, feat prereqs, skill ranks, equipment
effects, damage total, Level Up) actually called into `contract.rs` —
each engine was real and independently tested, but unreachable from the
one surface meant to expose it to a GUI
(`tests/sd20_tabletop_readiness_integration.rs`'s "Finding 1", first
recorded at Epic 8's original closure). The follow-on wiring project
(`/home/ubuntu/.claude/plans/adaptive-squishing-mccarthy.md`, 8 cycles,
all landed on `tranche/4`) closed that gap:

| Cycle | Lands | Commit |
|---|---|---|
| 0 `contract:receipt_signature_threading` | Widens `to_pilot_receipt` to take `input: &CharacterInput, corpus: &SourcePackageContent` | `52ed2ea` |
| 1 `contract:skill_wiring` | `PilotReceipt.skills: SkillTotals` (Epic 4's `allocate_skill_ranks`) | `4859b77` |
| 2 `contract:spellbook_wiring` | `PilotReceipt.spellbook: SpellbookCoverage` (Epic 2's `compute_spellbook_coverage`) | `2dbe0c8` |
| 3 `contract:feat_wiring` | `PilotReceipt.feats: Vec<ResolvedFeat>` (Epic 3's `evaluate_feat_prerequisites` + `compute_feat_effects`) | `0066599` |
| 4 `contract:equipment_wiring` | `PilotReceipt.equipment_effects: EquipmentEffects` (Epic 5's `compute_equipment_effects`) | `2942875` |
| 5a `damage:aggregate_weapons` | New `resolve_weapon_damage_breakdown` aggregator in `damage_total.rs` itself (Epic 6) — the one cycle that never touches `contract.rs` | `89fba8c` |
| 5b `contract:damage_wiring` | `PilotReceipt.weapon_damage: Vec<WeaponDamageBreakdown>`, reusing Cycle 4's `equipment_effects` local | `8510151` |
| 6 `contract:level_up_preview` | Standalone `compute_level_up_preview(character, from_level, to_level) -> LevelUpPlan` (Epic 7's `compute_level_up_grants`) | `62f7783` |
| 7 `integration:epic_wiring_closure` | Closing proof: the tabletop fixture's wired-path output matches each epic's direct-call output for every one of the six fields | (this cycle) |

### Field-by-field summary

- **`skills: SkillTotals`** (Epic 4). Real per-skill totals
  (rank + ability modifier + class-skill/cross-class handling +
  untrained-use), replacing (not supplementing) the old chassis-level
  narrow Climb/Intimidate/Swim-rank-1-only check. Every
  `allocate_skill_ranks` diagnostic is `claim_blocking: false` — it never
  fabricates a total, it either computes a real one or omits the skill
  from `totals`/`untrained_use` entirely. Cells: five `sheet.skill.*`
  (climb, intimidate, swim, diplomacy, disable_device) — see §3.
- **`spellbook: SpellbookCoverage`** (Epic 2). Prepared/known spells,
  slot totals/used, spell-save DCs, school specialization. Cells:
  dynamic `sheet.spellbook.slots_total.*` / `slots_used.*` /
  `spell_save_dc.*` — one per present `BTreeMap` key, zero for a
  non-caster — see §3. `spells_prepared`/`spells_known`/
  `school_specialization` are NOT cells (see §3's policy subsection).
- **`feats: Vec<ResolvedFeat>`** (Epic 3). One `ResolvedFeat { feat_id,
  prerequisites: PrerequisiteEvaluation, effects: FeatEffects }` per
  entry in `input.chosen.selected_feats` that resolves against
  `rules_tables::crb::feats::feat_tables()` (`entry.key == feat_id ||
  entry.name == feat_id`). `selected_feats` carries no category field of
  its own, so this match-by-key-or-name is how the wiring recovers the
  category the engine needs. An unmatched id is honestly skipped, not
  fabricated. No cells (see §3's policy subsection).
- **`equipment_effects: EquipmentEffects`** (Epic 5). Computed over
  `input.chosen.equipment_selections` filtered to `active_state ==
  ActiveState::EquippedActive` only. Cells: `sheet.equipment.armor_class_delta`
  (always present) and `sheet.equipment.max_dex_cap` (present only when
  `Some`) — see §3. `spell_failure_chance` is NOT a cell (see §3's policy
  subsection).
- **`weapon_damage: Vec<WeaponDamageBreakdown>`** (Epic 6). One entry per
  `EquippedActive` item `resolve_base_damage_dice` identifies as a weapon
  (carries a `DAMAGE:` corpus token); reuses the exact same
  `equipment_effects` value already computed for the field above, and the
  chassis's already-computed STR modifier — neither is recomputed. No
  cells; no summed damage-roll total is fabricated (see §3's policy
  subsection).
- **`compute_level_up_preview(character, from_level, to_level) ->
  LevelUpPlan`** (Epic 7). A standalone function, deliberately **not** a
  `PilotReceipt` field: Level-Up models a level *transition*
  (`from_level`/`to_level`), not current-state snapshot data like every
  other field on `PilotReceipt`. Folding it in would force fabricating
  those two params for every snapshot-only consumer, or contaminating the
  whole contract with transition-only fields. A thin pass-through to
  `level_up::compute_level_up_grants` — adds no logic and no cell of its
  own.

### Two facts that shaped every cycle (verified against real code before the project started)

1. `to_pilot_receipt(receipt: &CorpusPilotReceipt) -> PilotReceipt`
   could not reach any epic's data as originally signed — it never
   received the raw `CharacterInput`/`SourcePackageContent` every epic
   function needs. Cycle 0 widened it; all 4 real call sites already had
   `input`/`corpus` in scope as locals, so the widening was mechanically
   safe.
2. `CorpusDerivedSection.equipped_items[].derived_stats` (the pre-existing
   stub in `pilot_compute_corpus.rs` that looks like it was meant for
   Epic 5's data) cannot be filled in — `pilot_compute_corpus.rs` is a
   trunk file SD-20 does not touch. The new `equipment_effects` field
   carries Epic 5's real data instead; the stub stays `::default()`
   forever, correctly out of scope. (`tests/sd20_tabletop_readiness_integration.rs`'s
   `epic_probe_equipment_effects_are_real_but_not_reflected_in_corpus_derived`
   pins this exact contrast, side by side, for the same physical item.)

### Closing verification (Cycle 7)

`tests/sd20_tabletop_readiness_integration.rs`'s primary test now asserts,
for each of the six fields above, that `receipt.<field>` (from the real,
wired `to_pilot_receipt` call) is byte-identical (`PartialEq`) to that
epic's own function called directly on the same fixture
`CharacterInput`/corpus — not merely that both independently exist, but
that they genuinely **agree**. All six agreed with no discrepancy found
(no `## Open blockers` entry was needed for this cycle); see
`~/workspace/SD-20-rules-engine-completeness-progress.md`'s Cycle 7 log
entry for the full verification record. This closes Finding 1.

## Cross-reference

- `~/workspace/SD-20-rules-engine-completeness-scope-draft.md` §1.1 — Epic 1 acceptance criterion.
- `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/technical-design.md` §1 — contract shape, parity test format, cross-cutting authority surface.
- `~/workspace/SD-20-rules-engine-completeness-progress.md` — cycle log (commit SHAs, evidence-tier transitions).
- `/home/ubuntu/.claude/plans/adaptive-squishing-mccarthy.md` — the Epic 2-7 wiring project's plan (§5 above), 8 cycles, all landed on `tranche/4`.
- `tests/sd20_tabletop_readiness_integration.rs` — Epic 8's integration test; its module doc comment records Finding 1's full closure history and the feat-id fixture quirk's resolution.
