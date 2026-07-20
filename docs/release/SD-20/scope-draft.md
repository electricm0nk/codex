---
title: SD-20 — Rules Engine Completeness (Per-Character Tabletop-Readiness, Any Class Any Level 1-20) — Scope Draft
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
mirror_of: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md §1
canonical_branch: tranche/4 (operator directive 2026-07-16; slash form per prior naming convention)
---

# SD-20 — Rules Engine Completeness (Per-Character Tabletop-Readiness) — Scope Draft

This is the canonical handoff document for SD-20. The loop reads this file directly. The doctrine record lives at `./`. Two audiences, two locations — see SD-19's `decisions.md` §10 (workspace root holds operator-facing working docs; `programs/codex/requirements/` holds doctrine).

## 1. Pre-loop capability slices

SD-20 ships eight per-character epics, each with its own capability slice per the SD-19 §1 atomic-slice pattern. Epic 1 (boundary contract + wire-fixture parity tests) lands first; it defines the engine-side API surface every other epic produces into. Epics 2, 3, 4, and 5 all depend on SD-19's table store only and can land as concurrent slices. Epic 6 (damage total) is sequential after epic 5 (equipment). Epic 7 (Level Up grants) integrates after epics 2–6 close. Epic 8 (tabletop-readiness integration closure) is the integration milestone.

### 1.1 Epic 1 — Boundary contract + wire-fixture parity tests (gates 2, 3)

The boundary contract + wire-fixture parity tests epic is the load-bearing epic for SD-20. It defines the engine-side `CharacterInput`, `PilotReceipt`, and printed-sheet cell map; it lands the golden JSON fixtures both the engine and the GUI consume.

**Concrete deliverables:**

- **`CharacterInput` / `PilotReceipt` types.** NEW module `src/rules_core/contract.rs`. Types: `CharacterInput` (with `CharacterInput` per permutation: brand-new, mid-build, multiclass), `PilotReceipt` (per-derived-stat fields, per-source-record fields with `TableCellRef` provenance, diagnostic fields with `claim_blocking: true` preserved), and the printed-sheet cell map. Per `./technical-design.md` §1.1.
- **Boundary contract artifact.** NEW doc `docs/SD-20/boundary-contract.md` (engine-side API surface, three sections: CharacterInput shapes, PilotReceipt fields, printed-sheet cell map).
- **Wire-fixture parity tests.** NEW directory `tests/fixtures/wire/sd20/` with at least eight golden JSON files: one for the boundary contract itself, one each for spellbook, feat prereqs, skill ranks, equipment effects, damage total, Level Up grants, and integration closure. Each fixture: `{"name": ..., "input": {...CharacterInput...}, "expected_output": {...PilotReceipt...}, "expected_diagnostics": [...]}`. Per `technical-design.md` §1.2.
- **Engine-side RED test reading fixtures.** NEW test files at `tests/sd20_<criterion>.rs` (one per epic). The engine reads the wire-fixture JSON and asserts the receipt matches `expected_output`. The GUI's render tests (vibe-coded outside the bundle per `decisions.md` §6) consume the same fixtures; both sides fail on divergence.
- **Per-epic contract extension rule.** Per `technical-design.md` §1.3, no subsystem engine (epics 2–5), no Level Up grant (epic 7), and no integration closure (epic 8) may introduce a new `CharacterInput` field or a new `PilotReceipt` field without first extending the boundary contract and adding the parity test fixture.

### 1.2 Epic 2 — Spellbook engine (gate 4)

The spellbook engine epic computes spell effects, prepared-spell mechanics, spell save DCs, and bonus slots from high ability.

**Concrete deliverables (per cycle, per school):**

- **Spellbook engine module.** NEW module `src/rules_core/spellbook.rs` with `compute_spellbook_coverage` signature per `technical-design.md` §2.1.
- **Per-school contribution functions.** NEW files `src/rules_core/spellbook/<school>.rs` (9 files, one per strict school: abjuration, conjuration, divination, enchantment, evocation, illusion, necromancy, transmutation, universal).
- **CRB spell list reader.** Reads from SD-19's `src/rules_core/rules_tables/crb/spell_list.rs`. Spell *names* live in the corpus, resolved via SD-19's `spell_id_resolve`; spell *effects* (text, target, duration, save throw, spell resistance) come from the CRB table cell with `TableCellRef` provenance.
- **Per-cycle tests.** NEW `tests/sd20_spellbook_<school>.rs`. Asserts a `CharacterInput` with at least one spell of that school produces a non-empty `SpellbookCoverage` in the `PilotReceipt`; spell save DC is computed (not hardcoded); bonus slots from high ability are computed per the engine's `bonus_slots_from_ability` field.
- **Diagnostic elimination.** After this epic lands, `class_spell.<class>.<burden>.unsupported` no longer fires claim-blocking for any of the 9 strict schools in CRB scope.

### 1.3 Epic 3 — Feat prerequisite engine (gate 5)

The feat prerequisite engine epic computes feat eligibility checks and feat effects applied to derived stats.

**Concrete deliverables:**

- **Feat prerequisite engine module.** NEW module `src/rules_core/feat_prereqs.rs` with `evaluate_feat_prerequisites` and `compute_feat_effects` signatures per `technical-design.md` §2.2.
- **Per-feat or per-category functions.** NEW files `src/rules_core/feat_prereqs/<category>.rs`.
- **CRB feat catalog.** Reads from SD-19's feat table data. Per `epic-breakdown.md` criterion 7, every feat in CRB's feat tables must be in the engine's feat catalog (`src/rules_core/rules_tables/crb/feats/...` or equivalent).
- **Per-cycle tests.** NEW `tests/sd20_feat_<category>.rs`. Asserts a user-selected feat that satisfies all prerequisites produces a non-empty `FeatEffects`; a feat that fails a prerequisite produces a non-empty `PrerequisiteEvaluation.failing_prerequisites`.
- **Cross-epic consumption.** Epic 3 reads from epic 5's equipment outputs (for weapon focus / specialization entries) and from epic 2's spellbook outputs (for spell-related feats); the dependency is one-way (epic 3 consumes), no cycle.

### 1.4 Epic 4 — Skill-rank allocation engine (gate 6)

The skill-rank allocation engine epic computes per-level skill rank totals, class-skill bonuses, untrained vs. trained split, and max-rank caps.

**Concrete deliverables:**

- **Skill allocation engine module.** NEW module `src/rules_core/skill_allocation.rs` with `allocate_skill_ranks` signature per `technical-design.md` §2.3.
- **CRB skill table reader.** Reads from SD-19's skill table. Per `epic-breakdown.md` criterion 9, the engine enforces PF1's max-rank cap: class skills max at character level + 3, cross-class skills max at (character level + 1) / 2 rounded up. Cap violations produce diagnostics, not fabricated totals.
- **Per-cycle tests.** NEW `tests/sd20_skill_allocation_<criterion>.rs`. Asserts a user-allocated skill distribution produces `SkillTotals` whose per-skill totals match what the chassis + user-allocated + cross-class-penalty rules would yield; cap violations surface as diagnostics, not as silently capped or unbounded totals.
- **Class skill bonuses.** Flow from SD-19's class-feature tables (Ranger's +2 to specific skills, Bard's kit bonuses, etc.) into the receipt's `SkillTotal.class_skill_bonus` field.

### 1.5 Epic 5 — Equipment-effect engine (gate 7)

The equipment-effect engine epic extends SD-19's bounded baseline (AC, attack bonus, max dex, spell failure) to every field on every Paizo equipment entry.

**Concrete deliverables:**

- **Equipment-effects engine module.** NEW module `src/rules_core/equipment_effects.rs` with `compute_equipment_effects` signature per `technical-design.md` §2.4.
- **Per-category functions.** NEW files `src/rules_core/equipment_effects/<category>.rs` (4 categories: `arms_armor`, `general`, `magic_items`, `equipmods`).
- **CRB equipment table reader.** Reads from `src/rules_core/rules_tables/crb/equipment_tables.rs`. Every CRB equipment category reaches the engine end-to-end; per-item stats come from the foundation slice; per-item fields beyond the bounded baseline extend to every field the CRB table cell defines.
- **Aggregate equipment effects.** Computes `armor_class_delta`, `attack_bonus_delta`, `max_dex_cap`, `spell_failure_chance` from per-item stats. Magic weapons with enhancement bonuses contribute to `attack_bonus_delta`; armor contributes to `armor_class_delta` and `max_dex_cap` and `spell_failure_chance`.
- **Per-cycle tests.** NEW `tests/sd20_equipment_<category>.rs`. Asserts a `CharacterInput` with a full equipment loadout produces a populated `EquipmentEffects` whose per-item stats are non-default where the table defines them.

### 1.6 Epic 6 — Damage-total engine (gate 8)

The damage-total engine epic computes weapon damage rolls including critical hits, sequential after epic 5 because damage modifier reads from equipment stat breadth (STR mod + weapon enhancement + relevant feat effects).

**Concrete deliverables:**

- **Damage-total engine module.** NEW module `src/rules_core/damage_total.rs` with `compute_damage` signature per `technical-design.md` §2.5.
- **CRB damage entry reader.** Reads from the foundation slice's `equipment_tables.rs`. The receipt's `DamageRoll.base_dice` cites the weapon's KEY via `TableCellRef` provenance.
- **Critical hit rules.** ×2 default, ×3 for keen or other sources, ×4 for some specific weapons per CRB. Critical damage is the base-dice rolled again plus the modifier (no added modifier on the additional dice).
- **Per-cycle tests.** NEW `tests/sd20_damage_<criterion>.rs`. Asserts a weapon attack produces a `DamageRoll` whose base dice come from the weapon's CRB damage entry; damage modifier sums STR + weapon enhancement + relevant feat effects (read from epic 3's outputs); critical threat range and critical multiplier come from the weapon entry; PF1's critical rules are enforced.

### 1.7 Epic 7 — Level Up grant model (gate 9)

The Level Up grant model epic computes what the user gets at each level transition (free features, pick-from lists, ASI eligibility, feat picks, spell picks, skill rank pool).

**Concrete deliverables:**

- **Level Up engine module.** NEW module `src/rules_core/level_up.rs` with `compute_level_up_grants` signature per `technical-design.md` §2.6.
- **Per-class functions.** NEW files `src/rules_core/level_up/<class>.rs` (11 files, one per core class: barbarian, bard, cleric, druid, fighter, monk, paladin, ranger, rogue, sorcerer, wizard).
- **CRB class tables reader.** Reads from SD-19's class tables. The plan cites each grant's source via `TableCellRef`.
- **Multiclass handling.** Per `risks-and-open-questions.md` Open Q2 (default: no special shape; the foundation slice's class tables encode per-class-level grants and the engine layers them at the total character level).
- **Per-cycle tests.** NEW `tests/sd20_levelup_<class>.rs`. Asserts advancing a `CharacterInput` from level N to level N+1 produces a `LevelUpPlan` whose `automatic_features` and `pick_from_lists` match the published CRB table at level N+1 for the character's class(es). When the user picks selections (free feats, spells known, ASI allocation, etc.) and provides them in the next `CharacterInput`, the engine produces the updated receipt without re-fabricating any feature already auto-granted.

### 1.8 Epic 8 — Tabletop-readiness integration closure (gate 10 — load-bearing)

The tabletop-readiness integration closure epic is the end-to-end integration test. No new module; this epic is the closure criterion that every other epic's output composes correctly. Per the broadened acceptance criterion (operator directive 2026-07-16: "any class, any level"), this epic ships a fixture **set** rather than a single canonical character.

**Concrete deliverables:**

- **Integration test fixture set.** NEW directory `tests/fixtures/wire/sd20/tabletop/`. The fixture set covers:
  - **One canonical character per core class at level 1** (11 fixtures: `human_barbarian_level_1.json`, `human_bard_level_1.json`, ..., `human_wizard_level_1.json`). Each fixture is the class-appropriate first-build state: feats = the class-appropriate first-feat pick from epic 3, skill ranks = class skill allocation from epic 4, equipped = class-appropriate starting equipment from epic 5, prepared/known = class-appropriate starting spells from epic 2 (none for non-spellcasters; cantrips + 1st-level for casters).
  - **Higher-level sample fixtures** to ground multi-level mechanics: `human_fighter_level_4.json` (feat pick at L4), `human_fighter_level_8.json`, `human_fighter_level_12.json`, `human_fighter_level_16.json`, `human_fighter_level_20.json` (feat-pick-at-level cadence + ASI eligibility + capstone threshold at L20); `human_wizard_level_5.json`, `human_wizard_level_10.json` (spell-pick cadence); `human_cleric_level_20.json` (capstone); `fighter2_wizard1_total3.json` (multiclass per-class-level grant layering).
  - Total: 11 + 5 + 2 + 1 + 1 = **20 canonical tabletop fixtures** covering all 11 core classes at level 1 plus the multi-level and multiclass sample.
- **Integration test.** NEW file `tests/sd20_tabletop_readiness_integration.rs`. Reads each canonical fixture in `tests/fixtures/wire/sd20/tabletop/`, calls the engine end-to-end (`pilot_compute` + the boundary-contract dispatcher), asserts every `PrintSheetData` cell matches the table cells referenced by `TableCellRef`s.
- **Pathbuilder 2e parity check.** Each fixture's `expected_output` matches the values printed by Pathbuilder 2e for the same character (operator-driven verification; the test asserts engine output, not Pathbuilder output — Pathbuilder is the cross-reference for the operator's visual review).
- **GUI cycle.** The GUI consumes the same fixtures via the parity-test surface (`tests/fixtures/wire/sd20/tabletop/`); the user's "Print Sheet" click plugs values from `PrintSheetData` into the corresponding cell locations on the printed page.

## 2. Promotion gate

After all eight epics close (gate 10 met) AND SD-21 has landed its campaign manager + Drive persistence + APG + ACG ingestion epics AND a `tranche/4 → develop` promotion PR has been merged, SD-20 is closed and tranche-4 is done. The PR includes the SD-20 commits alongside SD-21's (with audit-trail comments per the codex-tranche-2-5 respawn-guard pattern). Per operator directive 2026-07-16: SD-20 launches on `tranche/4`; the `tranche/3 → develop` promotion PR is the chassis-lane promotion, not the per-character-rules-engine-lane promotion.

## 3. What does NOT gate SD-20 closure

- Loop's cycle log size (10 cycles or 100; criterion is the criterion, not volume).
- Number of self-heals applied during the run (zero or many; self-heals are the normal operating mode).
- Whether some epic-cards land as documentation-only versus full code-bearing (per the eligibility check — a school or category may legitimately land as a doc-only entry if the engine proves sufficient to ground the corpus-derived contribution in a recognition-only form).
- Spell *effects* beyond the engine's spellbook output — i.e. spell descriptions rendered as prose paragraphs rather than computed dice expressions. (Spell effect *text* is the canonical Paizo table cell; the engine renders it; computed dice within the prose is documented in epic 2's seam signature.)
- Equipment *effects* beyond the bounded baseline that SD-19 closed on (SD-19 closed at AC / attack / max dex / spell failure; epic 5 extends to every field on every Paizo equipment entry, but those extensions are part of SD-20's tabletop-readiness, not a separate gate from the bounded-baseline contract).
- GUI implementation (character sheet rendering, CreateCharacter form). The GUI stays outside the bundle per `decisions.md` §6 (operator's directive 2026-07-14: "let's leave it outside for now. if need be we can add a tranche-4-1"). The boundary contract is the only GUI-facing artifact in the bundle; the operator vibe-codes the GUI against the parity-test fixtures.
- Campaign manager + Drive persistence + APG + ACG ingestion (sibling bundle SD-21; per-character work and campaign/persistence work are parallelizable but separate lifecycles).
