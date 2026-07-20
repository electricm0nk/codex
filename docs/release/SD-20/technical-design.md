---
title: SD-20 — Technical Design
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
---

# SD-20 — Technical Design

This file is the load-bearing engineering-design surface for SD-20. `decisions.md` records *why* SD-20 is shaped the way it is; this file records *what* the shape is at the code level. A future implementer (tech-priest, contracted engineer, or the loop) should be able to work from this document with no further clarification needed.

## 1. Boundary contract (epic 1)

The engine-side boundary contract names every `CharacterInput` shape the engine accepts, every `PilotReceipt` shape it returns, and every printed-sheet cell the GUI renders. The GUI consumes the contract; the engine implements against it; the wire-fixture parity tests prove both sides stay in sync.

### 1.1 Contract shape

The contract lives at `docs/release/SD-20/boundary-contract.md` (created as an artifact of epic 1's capability slice). It has three sections:

**Inputs (what the engine accepts):** A `CharacterInput` for each canonical permutation:
- Brand-new character (`CharacterInput` with minimal fields, all defaults)
- Character mid-build with feats selected, skills allocated, equipment chosen, spells prepared, at any level 1-20
- Multiclass character (any subset of the 11 core classes at any class-level distribution)

**Outputs (what the engine returns):** A `PilotReceipt` (or its SD-20 successor shape) for each input. The receipt is the JSON-serializable surface the GUI consumes:
- Per-derived-stat fields (BAB, saves, HP, AC, attack bonus, ability mods, skill totals, spell DCs, damage expressions, equipment-derived stats)
- Per-source-record fields with provenance (which table cell or formula produced each value, for auditability)
- Diagnostic fields (claim-blocking diagnostics remain `claim_blocking: true`, non-blocking diagnostics carry narrative)

**Cells (what the GUI prints):** A row-by-row map of the printed PF1 character sheet, each cell pointing at exactly one `PilotReceipt` field. The GUI cannot invent a value; it renders what the receipt gives it. If a cell's source field is claim-blocked, the GUI renders "blocked — see diagnostics" rather than a fabricated number (no counterfeit completion).

### 1.2 Parity test format

Wire-fixture parity tests are golden JSON files at `tests/fixtures/wire/sd20/<criterion>.json`. Each file is:

```json
{
  "name": "human_fighter_level_1_basic",
  "input": { /* CharacterInput — see contract */ },
  "expected_output": { /* PilotReceipt — see contract */ },
  "expected_diagnostics": [ /* list of claim_blocking: true diagnostics if any */ ]
}
```

The engine's epic RED tests read these files and assert the engine produces exactly the `expected_output`. The GUI's render tests read the same files and assert each cell renders exactly the corresponding value. Both sides fail on divergence.

### 1.3 Cross-cutting authority surface

The boundary contract is the single authority for what the engine produces and what the GUI renders. No subsystem engine (epics 2–5), no Level Up grant (epic 7), and no integration closure (epic 8) may introduce a new `CharacterInput` field or a new `PilotReceipt` field without first extending the boundary contract and adding the parity test fixture.

## 2. Per-epic seam signatures

Each epic ships one or more Rust functions whose signature is the contract for that epic's compute surface. The signatures below are the load-bearing API each epic implements; epic implementation is "fill in the smallest body that makes the boundary contract + parity tests pass."

### 2.0 Table-store access convention (Decision, added post-cycle `17443b6`, 2026-07-17)

**`RulesTables` as sketched below was never built, and is retired from this document.** The original illustrative signatures in §2.1–§2.6 each took a third `rules_tables: &RulesTables` parameter — a single bundled type wrapping "the tables." No such type was ever defined anywhere in the codebase. Three epics landed real code against these seams before this question was resolved (Epic 2 spellbook — `3147b28` abjuration, `4f53724` conjuration; Epic 5 equipment effects — `fcd8571` arms_armor, `17443b6` general) plus one epic (Epic 4 skill allocation, `6c9b4af`/`c24c5f2`) that reasoned about the same question without yet needing a table read. Epic 3 (feat prereqs) hit this exact question first and logged it as a blocker (`./progress.md`, cycle `cycle-2026-07-17T1920`), flagging that no single epic's file-touch partition could safely invent `RulesTables` unilaterally without risking a shape collision with a concurrent sibling, and asking for "an owner decision so Epic 2/3/4/5 don't independently diverge."

**Finding:** they did not diverge. Every epic that actually touched the table store — and the SD-19 precedent both epics explicitly cited (`spell_resolver.rs`, `equipment_resolver.rs`) — landed on the identical access shape:

- **No parameter threading.** The table store is never passed into a compute function as an argument. It is read via a direct, fully-qualified `use` import of the specific table item the function needs, called or referenced inline inside the function body.
- **The table item is either a `pub const` static slice** (e.g. `rules_tables::crb::spell_list::SPELL_LIST: &[SpellListEntry]`) **or a `pub fn() -> &'static [T]` free function**, usually caching a generated `Vec<T>` behind a `std::sync::OnceLock` for tables built programmatically from the corpus (e.g. `rules_tables::crb::equipment_tables::equipment_tables()`, `rules_tables::crb::class_tables::class_tables()`, `rules_tables::crb::race_tables::race_traits()`). Both forms are read the same way at the call site — a plain `TABLE.iter().find(...)` or equivalent — so this is a data-representation detail (const-array vs. generated-and-cached), not a divergence in how the seam is threaded.
- **No local wrapper, newtype, or alias.** Nothing re-exports or re-wraps the table item under a per-epic name; every consumer imports the real `rules_tables::crb::<module>::<item>` path directly.
- **No ownership/borrowing variation.** Every table read yields `'static` borrowed data (`&'static [T]` or an iterator over it); nothing clones the table or takes ownership of it.
- **Corpus record resolution is a separate, already-existing seam element, not part of this decision.** Whether a given spell/equipment *id* resolves to a real corpus record goes through SD-19's own resolvers (`spell_resolver::spell_id_resolve`, `equipment_resolver::equipment_id_resolve`), which take an explicit `corpus: &SourcePackageContent` parameter. That parameter stays on the seam signatures that need corpus-identity resolution (spellbook, equipment effects); it is orthogonal to table-store access and is not being folded into "RulesTables" or removed.

**Decision:** `RulesTables` is retired as a documented type. The canonical seam contract, effective immediately, is: **a compute function that needs canonical table data imports the specific `rules_tables::crb::<table_module>` item it needs and reads it directly — no `rules_tables` parameter of any kind appears on the function signature.** §2.1–§2.6 below are updated to match. This is the same "adapt illustrative doctrine types to the real codebase shape, don't re-derive a parallel type" precedent `pilot_compute_corpus.rs`'s own doc comment already set for the illustrative `PilotReceipt` (SD-19) and that Epic 1's `contract.rs` followed for the same reason (§1's `PilotReceipt` composes `PilotBaseChassisComputation` + `CorpusDerivedSection` rather than inventing a parallel struct).

**Already-landed code check:** Epic 2 (`spellbook.rs`, `spellbook/abjuration.rs`, `spellbook/conjuration.rs`) and Epic 5 (`equipment_effects.rs`, `equipment_effects/arms_armor.rs`) already match this shape exactly — no retrofit needed. Epic 4 (`skill_allocation.rs`) has not yet landed a table read (no class-skill table exists in `rules_tables::crb` as of this writing) but its signature already anticipated this decision by dropping the illustrative `rules_tables: &RulesTables` parameter. Epic 3 (feat prereqs) remains blocked on the underlying table (`rules_tables::crb::feats` does not exist) independent of this decision — once that table lands, Epic 3 should read it the same direct way, per §2.2 below.

### 2.1 Epic 2 — Spellbook engine seam

```rust
// src/rules_core/spellbook.rs — parent dispatcher (landed)
pub fn compute_spellbook_coverage(
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> SpellbookCoverage;

// src/rules_core/spellbook/<school>.rs — per-school contribution function
// (landed: abjuration, conjuration; remaining schools per scope-draft.md §1.2
// Step 2 order: divination, enchantment, evocation, illusion, necromancy,
// transmutation, universal)
pub fn resolve_<school>_spell_effect(spell_id: &str) -> Option<<School>SpellEffect>;
// Internally: `rules_tables::crb::spell_list::SPELL_LIST.iter().find(|e| e.key
// == spell_id && e.school == Pf1SchoolId::<School>)` — direct import, no
// parameter. Spell *identity* (does this id resolve at all) is a distinct
// concern already handled by the dispatcher via SD-19's
// `spell_resolver::spell_id_resolve(spell_id, RuleSetId::Crb, corpus)`
// before dispatch reaches the per-school function.

pub struct SpellbookCoverage {
    pub spells_prepared: Vec<PreparedSpell>,
    pub spells_known: Vec<KnownSpell>,
    pub slots_total: BTreeMap<u8, u8>,    // spell level -> count
    pub slots_used: BTreeMap<u8, u8>,
    pub spell_save_dc: BTreeMap<ClassId, u8>,
    pub bonus_slots_from_ability: BTreeMap<u8, u8>,
    pub school_specialization: Option<ArcaneSchool>,
    // ...additional fields per spellbook engine epic
}
```

### 2.2 Epic 3 — Feat prerequisite engine seam

**Blocked as of `cycle-2026-07-17T1920`** — `rules_tables::crb::feats` does not exist yet (no feat catalog has been surfaced from `core_rulebook/cr_feats.lst` into the table store); see `./progress.md`'s "Open blockers" section. The signatures below apply the §2.0 decision (no `rules_tables: &RulesTables` parameter) so the next cycle that unblocks this epic doesn't have to re-derive the access pattern, but no code has landed against them yet.

```rust
// src/rules_core/feat_prereqs.rs (NEW module, not yet landed)
pub fn evaluate_feat_prerequisites(
    feat: &FeatKey,
    character_history: &CharacterHistory,  // feats taken, race, class, ability scores, BAB, skills
) -> PrerequisiteEvaluation;
// Internally: reads `rules_tables::crb::feats::<table>` directly (fully-qualified
// import, no parameter) once that table exists, the same way
// `spellbook/<school>.rs` reads `SPELL_LIST` and `equipment_effects.rs` reads
// `equipment_tables()`.

pub struct PrerequisiteEvaluation {
    pub is_eligible: bool,
    pub failing_prerequisites: Vec<FailedPrerequisite>,
    pub warnings: Vec<PrerequisiteWarning>,  // soft fails (e.g. "takes -2 to disable this feat")
}

pub fn compute_feat_effects(
    feat: &FeatKey,
    character: &CharacterInput,
) -> FeatEffects;  // the delta this feat contributes to derived stats
```

### 2.3 Epic 4 — Skill-rank allocation engine seam

**Landed (`6c9b4af`, `c24c5f2`), signature adapted:** the illustrative `allocation: &SkillAllocation` parameter is dropped — it would duplicate the already-landed `CharacterInput.chosen.skill_allocations: Vec<character_input::SkillAllocation>` (`skill_id: String, ranks: u8`), and this codebase keeps a single wire contract rather than a second echo carrier (see `contract.rs`'s `classify_character_input`/`to_pilot_receipt`, which take the composed input type directly). The illustrative `rules_tables: &RulesTables` parameter is dropped per §2.0. No class-skill table exists in `rules_tables::crb` yet, so this epic currently reads a bounded, cited, in-module posture (see the real `skill_allocation.rs` module doc comment) instead of a table read; once a real class-skill table lands in `rules_tables::crb`, it should be read the same direct way §2.0 establishes (fully-qualified import, no parameter) — not via a newly-invented parameter.

```rust
// src/rules_core/skill_allocation.rs (landed)
pub fn allocate_skill_ranks(input: &CharacterInput) -> SkillTotals;
// Skill choices come from the existing `input.chosen.skill_allocations`
// (no separate `allocation` parameter). Class-skill data source: bounded,
// cited posture today (see module doc comment); a future `rules_tables::crb`
// class-skill table, once it exists, would be read directly per §2.0 — no
// `rules_tables` parameter is added to this signature to consume it.

pub struct SkillTotals {
    pub totals: BTreeMap<SkillId, SkillTotal>,  // one per skill
    pub class_skills: Vec<SkillId>,
    pub cross_class_penalty_applied: bool,
    pub untrained_use: BTreeMap<SkillId, i8>,  // untrained-only skills
}

pub struct SkillTotal {
    pub ranks: u8,
    pub ability_modifier: i8,
    pub class_skill_bonus: i8,
    pub misc_modifier: i8,
    pub total_modifier: i8,
}
```

### 2.4 Epic 5 — Equipment-effect engine seam

**Landed (`fcd8571` arms_armor, `17443b6` general), signature adapted per §2.0:** the illustrative `rules_tables: &RulesTables` parameter is dropped; a `corpus: &SourcePackageContent` parameter is added (needed for SD-19's `equipment_resolver::equipment_id_resolve`, a distinct concern from table-store access — see §2.0's last bullet).

```rust
// src/rules_core/equipment_effects.rs — parent dispatcher (landed)
pub fn compute_equipment_effects(
    equipped: &[EquipmentSelection],
    corpus: &SourcePackageContent,
) -> EquipmentEffects;
// Internally: resolves each selection via `equipment_resolver::equipment_id_resolve`,
// then looks up category via `rules_tables::crb::equipment_tables::equipment_tables()`
// — direct import, no parameter — then dispatches to the category's own function.

// src/rules_core/equipment_effects/<category>.rs — per-category contribution
// function (landed: arms_armor, general; remaining per scope-draft.md §1.5
// work-unit order: magic_items, equipmods)
pub fn compute_<category>_effect(record: &EquipmentRecord) -> EquipmentStatEffect;
// Takes the already-resolved corpus record directly (no rules_tables access
// of its own — the CRB armor/shield/skill-bonus stat tokens this reads live
// on `EquipmentRecord` itself, not in a separate `rules_tables::crb` table).

pub struct EquipmentEffects {
    pub per_item: Vec<ResolvedEquipment>,  // existing SD-19 type, extended
    pub armor_class_delta: i8,             // aggregate AC change from equipped items
    pub attack_bonus_delta: i8,           // aggregate attack bonus from magic weapons
    pub max_dex_cap: Option<u8>,           // from armor
    pub spell_failure_chance: BTreeMap<SpellLevel, f32>,  // from armor
    pub derived_stats_per_item: BTreeMap<EquipmentKey, DerivedEquipmentStats>,
}
```

### 2.5 Epic 6 — Damage-total engine seam

**Not yet landed.** Signature updated per §2.0 (no `rules_tables: &RulesTables` parameter — read the specific `rules_tables::crb::<table>` item directly, inline, wherever this engine needs weapon/critical table data).

```rust
// src/rules_core/damage_total.rs (NEW module, not yet landed)
pub fn compute_damage(
    attacker: &CharacterState,
    weapon: &WeaponKey,
    target: Option<&TargetState>,           // None for theoretical; Some for attack flow
    attack_roll: Option<u8>,                // None for theoretical
) -> DamageRoll;

pub struct DamageRoll {
    pub base_dice: DiceExpression,         // e.g. "1d8", "2d6"
    pub damage_modifier: i8,                // STR mod + weapon enhancement + ...
    pub weapon_specialization_bonus: i8,   // greater /keen/ etc.
    pub critical_threat_range: (u8, u8),
    pub critical_multiplier: u8,
    pub expected_damage: f32,               // for GUI display: mean of the distribution
}
```

### 2.6 Epic 7 — Level Up grant model seam

**Not yet landed.** Signature updated per §2.0 (no `rules_tables: &RulesTables` parameter). Per-class files (`src/rules_core/level_up/<class>.rs`) should read the specific `rules_tables::crb::class_tables::class_tables()` rows (or a future per-class table) directly, the same way `spellbook/<school>.rs` reads `SPELL_LIST` — not via a parameter threaded down from `compute_level_up_grants`.

```rust
// src/rules_core/level_up.rs (NEW module, not yet landed)
pub fn compute_level_up_grants(
    character: &CharacterInput,
    from_level: u8,
    to_level: u8,
) -> LevelUpPlan;

pub struct LevelUpPlan {
    pub automatic_features: Vec<Grant>,         // Bravery +2 at F6, BAB rise at any level, etc.
    pub pick_from_lists: Vec<PickList>,         // "1 feat from [list]", "1 spell from class list", etc.
    pub resource_pool_change: ResourcePoolChange,  // skill points to allocate, HP roll
    pub prerequisites_added: Vec<Prerequisite>,   // feat prereqs at this level
    pub capstone_threshold: bool,                 // true if this level crosses a capstone
}

pub struct Grant {
    pub name: String,
    pub source_table: TableCellRef,            // provenance — where this grant comes from
    pub effects: Vec<GrantEffect>,
}

pub struct PickList {
    pub category: PickCategory,                 // feat, spell, school power, etc.
    pub count: u8,                              // how many the user picks
    pub candidates: Vec<PickCandidate>,         // what they can pick from
    pub filter: PickFilter,                     // e.g. "must satisfy BAB +4"
}
```

The `LevelUpPlan` is the user-facing surface: "When you gain a level in Fighter 6, you automatically get Bravery +2, and you pick 1 of these 3 bonus combat feats." The GUI consumes the `LevelUpPlan` and renders the pick list; the engine doesn't track what the user picks (that's `CharacterInput`'s job on the next iteration).

### 2.7 Epic 8 — Tabletop-readiness integration closure

No new seam. This epic is the integration test: every other epic's seam produces values into a `PrintSheetData` (per the revised Q3 pin: print-ready data the UI plugs into cell locations) for the same `CharacterInput`, and the closed-form data matches what the GUI needs to plug into the printed-sheet cell map. The integration test fixture set is the canonical "tabletop scenario" set — one fixture per core class at level 1 (11 fixtures) plus a smaller sample at higher levels and one multiclass fixture (per the broadened acceptance criterion and `acceptance-and-verification.md` gate 10) — exercising every seam end-to-end.

## 3. Per-epic authority surface

Each epic is the authoritative source for a specific class of value. No epic may fabricate or duplicate another epic's outputs.

| Epic | Authoritative for | Forbidden to fabricate |
|---|---|---|
| Boundary contract (epic 1) | The shape of `CharacterInput`, `PilotReceipt`, and printed-sheet cells. The contract is the only place these are defined. | Anything outside the contract. |
| Spellbook engine (epic 2) | Spell effects, prepared-spell mechanics, spell save DCs, bonus slots from high ability. Reads from SD-19's `src/rules_core/rules_tables/crb/spell_list.rs`. | Spell *names* (those live in the corpus, resolved via SD-19's `spell_id_resolve`). |
| Feat prerequisite engine (epic 3) | Feat eligibility, feat effects. Reads from SD-19's feat table data. | Class features that grant feats (those live in the spellbook engine or class-feature tables). |
| Skill-rank allocation engine (epic 4) | Skill rank totals. Reads from SD-19's skill table. | Feat effects that grant skill bonuses (those live in the feat engine). |
| Equipment-effect engine (epic 5) | Per-item derived stats, aggregate AC/attack deltas, max-dex cap, spell failure. Reads from SD-19's `src/rules_core/rules_tables/crb/equipment_tables.rs`. | Class features that grant equipment (those live in class-feature tables). |
| Damage-total engine (epic 6) | Weapon damage rolls including critical. Reads from epic 5's outputs. | Spell damage (spell damage lives in the spellbook engine's spell descriptions). |
| Level Up grant model (epic 7) | What the user gets at each level transition. Reads from SD-19's class tables + epic 3's feat catalog. | Anything outside the published level-up table (e.g. invented free feats not in the CRB). |
| Tabletop-readiness integration closure (epic 8) | End-to-end integration. Reads from every other epic. | Anything; this epic only verifies. |

## 4. File-touch partition (defense for the loop)

The loop's file-touch partition mirrors SD-19's. Per-epic modules live under `src/rules_core/<epic-slug>/` (one per epic that owns a new module), and each epic ships exactly one new file plus a single dispatcher line in `pilot_compute.rs` (which itself stays untouched after SD-18).

**For loop-routed cycle work** (post-capability-slice, per the SD-19 loop pattern):
- Boundary contract (epic 1) — adds types to `src/rules_core/contract.rs` (NEW). Touches the contract signature and the boundary-contract doc only.
- Spellbook (epic 2) — `src/rules_core/spellbook.rs`. Per-school contribution functions in `src/rules_core/spellbook/<school>.rs` (9 files, one per school).
- Feat prereqs (epic 3) — `src/rules_core/feat_prereqs.rs`. Per-feat or per-category functions in `src/rules_core/feat_prereqs/<category>.rs`.
- Skill ranks (epic 4) — `src/rules_core/skill_allocation.rs`.
- Equipment effects (epic 5) — `src/rules_core/equipment_effects.rs`. Per-category files in `src/rules_core/equipment_effects/<category>.rs`.
- Damage total (epic 6) — `src/rules_core/damage_total.rs`.
- Level Up grants (epic 7) — `src/rules_core/level_up.rs`. Per-class files in `src/rules_core/level_up/<class>.rs` (11 files, one per core class).
- Integration closure (epic 8) — `tests/sd20_tabletop_readiness_integration.rs`. Reads from every other epic; writes nothing.

Each epic's capability slice lands atomically (per SD-19 §1's atomic-slice pattern). Each epic's loop cycles add one sub-module file plus one dispatcher line in the parent module. The trunk file (`pilot_compute.rs`) is touched only by integration-closure work and only through additive dispatch lines.

## 5. Cross-reference

- `acceptance-and-verification.md` — closure gates including tabletop-readiness.
- `decisions.md` — the 9-item decision record.
- `epic-breakdown.md` — 15 acceptance criteria grouped into 8 epics.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-requirements.md` — pre-loop prerequisites for SD-20.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store (the layered seam pattern).
- `../SD-21/` — sibling bundle, parallelizable.
