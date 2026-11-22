---
title: SD-19 — Technical Design
status: reviewed (operator, 2026-07-16)
date: 2026-07-14
companion_to: /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
---

**2026-07-16 operator review note:** §1.4/§1.5/§6.1 describe per-school and
per-category coverage functions (`school_coverage/<school>.rs`,
`equipment_coverage/<category>.rs`) as landed one-per-loop-cycle. This
conflicts with `scope-draft.md`'s seam-shapes-correctness test (item e),
which requires the capability slice's own test to observe a non-empty
`corpus_derived` section. Operator-confirmed resolution: the capability
slice ships a **generic** resolution path in `compute_pilot_with_corpus`
itself (school/category read directly off the resolved corpus record,
covering all 9 schools / 4 categories generically) rather than an empty
dispatcher skeleton — satisfying the non-empty test now. Loop cycles then
focus on grounding/evidence-tier promotion for their criterion rather than
authoring new dispatch code from scratch.

# SD-19 — Technical Design

This file specifies the seam shape, the resolver signatures, the `CharacterInput` extensions, the `MatrixSubjectType` extension, the cycle surface, and the branch lifecycle for SD-19. The capability-slice PR (pre-loop gate) implements §1–§4 below; the loop's 13 cycles then exercise the seam against each acceptance criterion.

## 1. Corpus-aware compute seam

### 1.1 Function signature

```rust
// src/rules_core/pilot_compute_corpus.rs — NEW module added by the
// capability slice. Imports compute_pilot_base_chassis from
// src/rules_core/pilot_compute.rs (which itself stays untouched).

/// Corpus-augmented compute result. Wraps the existing PilotReceipt and
/// adds a corpus-derived section carrying the spell-coverage and
/// equipped-items contributions produced by the seam.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorpusPilotReceipt {
    /// The unchanged chassis receipt from compute_pilot_base_chassis.
    pub base: PilotReceipt,
    /// The corpus-derived contributions grounded by this call: spell
    /// school coverage, equipped items, and any per-cycle deltas added
    /// by the seam. Non-empty only when at least one corpus record was
    /// resolved; empty when the input carried no spells_selected or
    /// equipment_selections entries that resolved.
    pub corpus_derived: CorpusDerivedSection,
}

/// Per-domain corpus-derived contributions. Each variant is populated
/// when the corresponding CharacterInput field carries at least one
/// id that resolved via the corresponding resolver.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CorpusDerivedSection {
    pub school_coverage: BTreeMap<Pf1SchoolId, SchoolCoverage>,
    pub equipped_items: Vec<ResolvedEquipment>,
}

/// A canonical Paizo-table-cell reference. Each SchoolCoverage and each
/// ResolvedEquipment carries one of these; non-None proves that the
/// corpus record the seam resolved lives at a specific cell of the
/// rule-set's source-book table, not just "a corpus record exists."
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableCellRef {
    /// Which rule set's source book the cell lives in (today: only Crb).
    pub rule_set: RuleSetId,
    /// The table name within the source book (e.g. "spell_list",
    /// "fighter_class_table", "equipment_arms_armor").
    pub table: String,
    /// The row identifier within the table (e.g. spell KEY: token,
    /// item KEY: token, class-level integer for class tables).
    pub row_key: String,
    /// Optional column identifier (e.g. "spells_per_day_l1" for the
    /// spells-per-day table at level 1). Empty string when the row
    /// is the entire cell (e.g. for spell descriptions, feat text).
    pub column_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchoolCoverage {
    pub school: Pf1SchoolId,
    pub spells: Vec<String>,   // corpus KEY: tokens, sorted
    /// Canonical Paizo-table-cell reference for the school's row of
    /// the spell list table. Non-None after the foundation slice lands.
    pub table_cell: Option<TableCellRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedEquipment {
    pub item_id: String,                 // the CharacterInput.item_id verbatim
    pub equipment_record_name: String,   // the resolved corpus record's name
    pub equipment_record_key: String,    // the resolved corpus record's KEY: token
    pub derived_stats: DerivedEquipmentStats,
    /// Canonical Paizo-table-cell reference for the item's row of the
    /// relevant equipment table. Non-None after the foundation slice lands.
    pub table_cell: Option<TableCellRef>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DerivedEquipmentStats {
    pub armor_bonus: Option<i16>,
    pub attack_bonus: Option<i16>,
    pub max_dex: Option<i16>,
    pub spell_failure: Option<f32>,
}

pub fn compute_pilot_with_corpus(
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> CorpusPilotReceipt {
    // 1. Compute the existing chassis (unchanged call to compute_pilot_base_chassis)
    // 2. Resolve every spells_selected entry via spell_id_resolve
    // 3. Resolve every equipment_selections entry via equipment_id_resolve
    // 4. Compute corpus-derived deltas (school coverage, equipped items with
    //    DerivedEquipmentStats) and wrap them in CorpusDerivedSection
    // 5. Return CorpusPilotReceipt { base, corpus_derived }
}
```

### 1.2 Design rationale

Per `decisions.md` §4 and the operator-confirmed option 2 on the seam-return-type question (2026-07-14): the seam is additive at two levels. First, `compute_pilot_base_chassis` stays untouched — every existing SD-18 §3.1/§3.2/§3.3 cycle's tests and fixtures continue to call it with the unchanged signature. Second, the seam returns a *new wrapper type* (`CorpusPilotReceipt`) rather than mutating `PilotReceipt`. Every existing `PilotReceipt` construction site stays untouched; new SD-19 cycle tests destructure `CorpusPilotReceipt.base` to read the unchanged chassis and `CorpusPilotReceipt.corpus_derived` to read the new contributions. This is the same pattern SD-18's §1.1 `ComposedCharacterInput` used (a wrapper type that carries both halves without modifying the inner type) applied at the function-return level.

The wrapper approach also matches your standing preference for narrow, scope-based routing: the seam's "what" (a concrete function over a concrete corpus type) is exactly what §3.4/§3.5 need, and a `CorpusProvider` trait abstraction that would generalize over corpus sources is deferred until a second corpus source actually appears (deferred per your "longer-term flexibility isn't worth it at this point" call).

`DerivedEquipmentStats` carries four fields (`armor_bonus`, `attack_bonus`, `max_dex`, `spell_failure`) at slice-ship per operator directive 2026-07-14. The seam-shapes-correctness test asserts all four fields exist on every `ResolvedEquipment` returned, even when most are `None` (per-cycle representative samples populate the subset the cycle actually grounds). This matches your standing preference for fixture-once, populate-as-needed: the type is stable from ship, cycles don't have to revisit the struct definition when they expand coverage.

### 1.3 What the seam does NOT do

- **It does not compute slot math.** Slot counts (spells prepared per day, bonus slots from high ability scores, DCs) remain deliberately out of scope. The §3.4 blocker entry's full reasoning stands; this bundle closes the *reachability* gap, not the *spellbook engine* gap. Slot math lands in a future SD-N once the seam is exercised and the operator decides what slot math is needed.
- **It does not fabricate spell effects.** `pilot_compute.rs`'s 7 currently-live claim-blocking diagnostics (`class_spell.<class>.<burden>.unsupported`) remain `claim_blocking: true` until a future tranche-level decision authorizes them otherwise. The seam grounds the corpus link; it does not unblock spell-effect computation. This is the same posture SD-18's §3.2 widenings maintained: the loop widens what it can prove, never what it cannot.
- **It does not invent ID semantics.** Both resolvers translate *between* two pre-existing identity namespaces (`CharacterInput.item_id` / `CharacterInput.spell_id` strings, and the corpus's real records); neither resolver invents a new identity namespace. The translation rule is documented in `decisions.md` §2 and §3 respectively.

### 1.4 Corpus-derived contribution surface (per-cycle landing pattern)

Once the seam exists, each loop cycle's smallest-change-to-green lands one of these:

| Domain | Corpus-derived contribution grounded | Acceptance criterion |
|---|---|---|
| Spell school X | A `CorpusDerivedSection.school_coverage[X]` entry containing every spell in school X's slice from the corpus (resolved via `spell_id_resolve`), classified by acquisition_mode, AND a `TableCellRef` pointing at the school-X row of the CRB spell list table | §2.4 school X card |
| Equipment category Y | A `CorpusDerivedSection.equipped_items` entry for each `equipment_selections` entry that resolved, with `DerivedEquipmentStats` populated for the sample the cycle grounded, AND a `TableCellRef` pointing at the item's row of the relevant CRB equipment table | §2.5 category Y card |

A cycle's RED test asserts (a) the corpus-derived contribution is present in the receipt's `corpus_derived` section, (b) at least one stat value (`school_coverage` count, or `derived_stats.armor_bonus` / `derived_stats.attack_bonus` / `derived_stats.max_dex` / `derived_stats.spell_failure` on at least one equipped item) differs from the same input run through `compute_pilot_base_chassis` alone, AND (c) the `TableCellRef` for each grounded entry is non-None and resolvable through the foundation slice's table store. The (c) assertion is the proof that the corpus record the cycle resolved lives at a specific Paizo table cell — the canonical-authority claim, not just a corpus-name-claim.

### 1.5 Per-school and per-category sub-module convention

Each cycle's contribution shape lives in its own file under `src/rules_core/school_coverage/` (one file per school, e.g. `abjuration.rs`, `conjuration.rs`, ..., `universal.rs`) or `src/rules_core/equipment_coverage/` (one file per category: `arms_armor.rs`, `general.rs`, `magic_items.rs`, `equipmods.rs`). Each file exports exactly one function:

```rust
// src/rules_core/school_coverage/abjuration.rs — added by SD-19 cycle
// for §2.4 Abjuration
pub fn populate_school_abjuration_coverage(
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> SchoolCoverage {
    // Cycle's smallest-change-to-green implementation.
    // Reads input.spells_selected entries that resolve via
    // spell_id_resolve to a SpellRecord whose strict-school partition
    // is Abjuration, and returns a SchoolCoverage populated with
    // those spells' corpus KEY: tokens, sorted.
}

// src/rules_core/equipment_coverage/arms_armor.rs — added by SD-19
// cycle for §2.5 arms_armor
pub fn populate_arms_armor_coverage(
    input: &CharacterInput,
    corpus: &SourcePackageContent,
) -> Vec<ResolvedEquipment> {
    // Cycle's smallest-change-to-green implementation.
    // Reads input.equipment_selections entries that resolve via
    // equipment_id_resolve to an EquipmentRecord in the arms_armor
    // category, and returns a Vec<ResolvedEquipment> with
    // DerivedEquipmentStats populated for the sample the cycle grounded.
}
```

`compute_pilot_with_corpus` in `pilot_compute_corpus.rs` is a thin dispatcher: it imports each cycle's function as the cycle lands, and the seam body grows by exactly one new dispatch per cycle (e.g. `school_coverage.insert(Pf1SchoolId::Abjuration, school_coverage::abjuration::populate_school_abjuration_coverage(input, corpus));`). The dispatcher itself never grows beyond the nine school dispatches and four category dispatches. Each cycle's diff is purely additive: one new file in `school_coverage/` or `equipment_coverage/` plus one new line in the dispatcher. No cycle edits an existing function.

**Rationale:** the same additive-only discipline that produced the `pilot_compute_corpus.rs` new-file choice (§7) extends to per-cycle work. The cleanest diff per cycle is "one new file + one new dispatcher line," which makes bisect-and-review trivial and means a cycle that fails cannot break any other cycle's contribution.

## 2. Resolver signatures

### 2.1 Equipment-id resolver

```rust
// src/rules_core/equipment_resolver.rs — added by the capability slice
use crate::pcgen_import::corpus::{EquipmentRecord, SourcePackageContent};
use crate::rules_core::rules_tables::RuleSetId;

/// Resolve a CharacterInput.equipment_selections[].item_id to a corpus record
/// AND its canonical Paizo-table-cell location.
///
/// The lookup rule:
/// 1. Build a HashMap<String, &EquipmentRecord> from corpus.equipment.records,
///    keyed on the normalized form of record.name:
///      - lowercase
///      - spaces -> underscores
///      - strip parenthesized qualifiers (e.g. "(Base)", "(Masterwork)")
/// 2. Build a secondary HashMap<String, &EquipmentRecord> keyed on the
///    record's verbatim KEY: token (for cross-source collision resolution).
/// 3. For a given item_id:
///      - exact match against the secondary KEY: index first (handles cases
///        where the fixture passes the literal corpus KEY: token)
///      - exact match against the normalized primary index second
///      - otherwise: None (the cycle's RED test asserts the failure mode).
/// 4. If the corpus record resolves and the foundation slice's table store
///    has a matching row in `rules_tables::crb::equipment_<rule_set>(category)`,
///    the function returns `Some(EquipmentRecord, TableCellRef)`. Otherwise
///    it returns `Some(record, None)` and the cycle asserts the missing
///    table-cell reference at the cycle level.
///
/// The existing fixture namespace ("item:longsword", "item:chain_shirt",
/// "item:shield") resolves to corpus records whose normalized names match
/// the substring after the "item:" prefix.
pub fn equipment_id_resolve<'a>(
    item_id: &str,
    rule_set: RuleSetId,
    corpus: &'a SourcePackageContent,
) -> Option<(&'a EquipmentRecord, Option<TableCellRef>)> {
    // implementation in the capability slice
}
```

### 2.2 Spell-id resolver

```rust
// src/rules_core/spell_resolver.rs — added by the capability slice
use crate::pcgen_import::corpus::{SpellRecord, SourcePackageContent};
use crate::rules_core::rules_tables::RuleSetId;

/// Resolve a CharacterInput.spells_selected[].spell_id to a corpus record
/// AND its canonical Paizo-table-cell location.
///
/// The lookup rule mirrors equipment_id_resolve:
/// 1. Build a HashMap<String, &SpellRecord> keyed on the verbatim KEY: token.
/// 2. For a given spell_id, exact match against the KEY: index.
/// 3. If the corpus record resolves and the foundation slice's table store
///    has a matching row in `rules_tables::crb::spell_list_<rule_set>(school)`,
///    the function returns `Some(SpellRecord, TableCellRef)`. Otherwise it
///    returns `Some(record, None)` and the cycle asserts the missing
///    table-cell reference at the cycle level.
///
/// Spell identity is unambiguous in the corpus (KEY: tokens are unique per
/// record) so no normalization is needed; the simpler lookup matches the
/// corpus's own semantics.
pub fn spell_id_resolve<'a>(
    spell_id: &str,
    rule_set: RuleSetId,
    corpus: &'a SourcePackageContent,
) -> Option<(&'a SpellRecord, Option<TableCellRef>)> {
    // implementation in the capability slice
}
```

### 2.3 Resolver design rationale

- Both resolvers are pure functions over `(identity string, &SourcePackageContent)`. No state, no global registries, no plugin systems. Each is testable in isolation.
- The equipment resolver's normalization is necessary because the existing fixture namespace (`"item:longsword"`) is the public surface and the corpus's `KEY:` tokens are not always snake_case (`"Padded Armor (Base)"`). The KEY: secondary index handles cross-source collisions.
- The spell resolver's simpler KEY: lookup is sufficient because the corpus's spell records use unique KEY: tokens across the entire PF1 corpus — the cycle's investigation found zero collision risk in the strict-school partition.

## 3. CharacterInput extensions

### 3.1 New field on ChosenCharacterState

```rust
// src/rules_core/character_input.rs — modified by the capability slice

/// One spell a class knows, has prepared, or has granted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpellSelection {
    /// The corpus KEY: token identifying this spell.
    pub spell_id: String,
    /// The class that provides this spell (used for per-class spell-list
    /// validation in future cycles; not consumed by the seam itself).
    pub source_class_id: ClassId,
    /// How this spell was acquired. Determines future slot-math behavior;
    /// the seam does not yet consume this field, but it is present at the
    /// type level so the seam can carry it forward without a refactor.
    pub acquisition_mode: AcquisitionMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquisitionMode {
    /// Spontaneous caster knows the spell; no preparation needed.
    Known,
    /// Prepared caster has prepared this specific spell in a slot today.
    Prepared,
    /// Granted by a class feature, domain, or other non-standard source.
    Granted,
}

// Existing ChosenCharacterState gains one field:
pub struct ChosenCharacterState {
    pub race_id: RaceId,
    pub class_levels: BTreeMap<ClassId, u8>,
    pub ability_scores: AbilityScores,
    pub selected_feats: Vec<FeatId>,
    pub skill_allocations: BTreeMap<SkillId, u8>,
    pub equipment_selections: Vec<EquipmentSelection>,  // existing — unchanged
    pub selected_choices: BTreeMap<String, String>,
    pub spells_selected: Vec<SpellSelection>,           // NEW (SD-19)
}
```

### 3.2 Design rationale

Per `decisions.md` §3: mirrors `equipment_selections` exactly. `acquisition_mode` is the difference between a clean first cut and a future refactor. The seam does not yet consume it (slot math is future work) but the type carries it forward so the seam's output can tag each corpus-derived spell with its acquisition mode without a second `CharacterInput` change later.

### 3.3 Backward compatibility

Every existing test fixture and every landed SD-18 cycle's `CharacterInput` construction site will continue to compile and pass without modification: the new field defaults to `Vec::new()` via a `#[derive(Default)]` impl on `ChosenCharacterState`, which `CharacterInput` already requires. No fixture churn. The seam's RED test for the capability slice is the first construction site to populate the field; subsequent cycle tests populate it for their own grounding work.

## 4. MatrixSubjectType extensions

### 4.1 New variants

```rust
// src/rules_core/support_state_matrix.rs — modified by the capability slice

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatrixSubjectType {
    Race(RaceId),                  // existing
    Class(ClassId),                // existing
    Interaction(InteractionId),    // existing
    School(Pf1SchoolId),           // NEW (SD-19)
    Equipment(EquipmentCategory),  // NEW (SD-19)
}

/// Canonical PF1 strict-school identifier. Mirrors the 9-card list in
/// SD-19's scope doc §2.4. Universal is a strict school in PF1 canon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pf1SchoolId {
    Abjuration, Conjuration, Divination, Enchantment, Evocation,
    Illusion, Necromancy, Transmutation, Universal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentCategory {
    ArmsArmor, General, MagicItems, Equipmods,
}
```

### 4.2 Row shape

The existing `Race` and `Class` row shapes use a `MatrixSubjectType` discriminator + the row's `support_state`, `evidence_tier`, `grounding_ref`, `blocker_or_lossiness_note`, and `next_required_uplift` fields. `School` and `Equipment` rows use the same shape, with the `grounding_ref` naming the corresponding `tests/sd19_<criterion>.rs` test file and the `blocker_or_lossiness_note` carrying the per-row remaining-work notes (e.g. for Abjuration: "spellbook engine, slot math, spell save DCs — out of scope per decisions.md §1.3; this row is grounded on reachability, not on effect execution").

### 4.3 Why this matters

Per `decisions.md` §5: the §3.5 cycle's investigation explicitly noted that neither spell schools nor equipment categories are modeled as matrix rows today. Closure (`supported/Product-visible`) requires a matrix row transition per the SD-13 vocabulary; without the variant, the §3.4/§3.5 cards can never reach the same closure posture as §3.1/§3.2. The variant unblocks closure, no more, no less.

## 5. Capability slices (pre-loop gates)

SD-19 ships in two atomic commits per `decisions.md` §8: the foundation slice ships first, then the main capability slice. No PRs, no ephemeral branches, no merge target separate from `tranche/3` (per `decisions.md` §6).

### 5.1 Foundation slice (atomic commit 1)

The foundation slice ships as a tiny stand-alone atomic direct commit to `tranche/3`. The commit contains only data and one type — no seam, no resolvers, no wrapper types:

```
Title: feat(sd19): canonical Paizo-table store (foundation slice)

Changes:
- src/rules_core/rules_tables/mod.rs: NEW (module shell with RuleSetId enum)
- src/rules_core/rules_tables/crb/role_set_id.rs: NEW (RuleSetId::Crb variant)
- src/rules_core/rules_tables/crb/class_tables.rs: NEW (PF1 CRB class tables
  as structured data — one entry per class per level, with named features
  and exact spell/per-day cells from the CRB)
- src/rules_core/rules_tables/crb/spell_list.rs: NEW (PF1 CRB strict-school
  spell list as structured data — one entry per spell with KEY: token,
  school partition, level, and description text)
- src/rules_core/rules_tables/crb/equipment_tables.rs: NEW (PF1 CRB
  equipment tables as structured data — one entry per item with KEY: token,
  category, name, slot, derived stats where applicable)
- tests/sd19_table_store_foundation.rs: NEW (asserts the CRB directory is
  parseable, RuleSetId::Crb resolves, every class table has entries for
  levels 1-20, every spell list has at least one spell per school, every
  equipment table is non-empty)
```

The foundation slice is the structural prerequisite for the main capability slice. No §2 acceptance criterion in the scope doc is verifiable until both slices land.

### 5.2 Main capability slice (atomic commit 2)

The main capability slice ships as an atomic direct commit to `tranche/3`, with all eight deliverables atomically together — splitting them across multiple commits was explicitly considered and rejected because the slice's review value is in seeing the entire corpus-aware capability ship as one coherent unit:

```
Title: feat(sd19): corpus-aware compute seam (atomic slice)

Changes:
- src/rules_core/pilot_compute_corpus.rs: NEW (compute_pilot_with_corpus
  + the wrapper types CorpusPilotReceipt, CorpusDerivedSection,
  TableCellRef, SchoolCoverage, ResolvedEquipment, DerivedEquipmentStats).
  The new module imports compute_pilot_base_chassis from pilot_compute.rs.
  pilot_compute.rs itself stays untouched.
- src/rules_core/equipment_resolver.rs: NEW (equipment_id_resolve with
  RuleSetId parameter)
- src/rules_core/spell_resolver.rs: NEW (spell_id_resolve with RuleSetId
  parameter)
- src/rules_core/character_input.rs: add SpellSelection, AcquisitionMode,
  spells_selected field
- src/rules_core/support_state_matrix.rs: add School/Equipment variants
  + row shapes
- tests/sd19_seam_shapes_correctness.rs: NEW (proof before loop runs)
- tests/fixtures/rules_core/sd19_seam_crb_*.txt: NEW (13 hand-typed
  fixtures with the sd19_seam_crb_ prefix; one per school for spells,
  one per category for equipment)
```

Per-fixture authoring procedure for `sd19_seam_crb_*.txt` (operator directive 2026-07-14):
- The slice executor reads each corpus file directly:
  `pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst` for
  spells, and the four `cr_equip_*.lst` files for equipment.
- For each school: pick one representative spell record (any spell in that
  school's strict-school partition) and copy the relevant lines verbatim
  into `sd19_seam_crb_spell_<school>.txt`. Prepend a one-line comment naming
  the source: e.g. `# Fixture for §2.4 Abjuration, source: cr_spells.lst,
  KEY: Shield of Faith`.
- For each category: pick one representative equipment record and copy
  verbatim into `sd19_seam_crb_equip_<category>.txt`. Prepend the analogous
  one-line source comment.
- The fixture content is the verbatim corpus record (plus the source
  comment), not a synthesized approximation. A future operator can verify
  the fixture is current by re-running the slice executor's grep against
  the corpus and confirming the source line still matches the fixture
  content.
- Fixtures live in `tests/fixtures/rules_core/` alongside SD-18's
  per-cycle fixtures; the `sd19_seam_crb_` prefix distinguishes them from
  per-cycle fixtures and from any future-book fixtures.

The seam-shapes-correctness test reads each of the 13 `sd19_seam_crb_*.txt` files directly via the SD17-B-4 spell parser (`src/pcgen_import/lst_parser/spell.rs:488`) and the SD17-B-5 equipment parser (`src/pcgen_import/lst_parser/equipment.rs:781`) — the test exercises the seam end-to-end exactly as the parser would see the corpus. No CORPUS_ROOT dependency at slice time. The 13 fixture files are real test input, not inert documentation; a corpus-side drift that breaks parser fixtures breaks the seam test too. The per-cycle CORPUS_ROOT real-corpus assertion called out in `decisions.md` §6.6 is a separate concern that applies to per-cycle fixtures at loop time, not to this slice-ship test.

The main capability slice is the structural prerequisite for the SD-19 loop. No §2 acceptance criterion in the scope doc is verifiable until this commit lands on `tranche/3`.

Acceptance (run from `tranche/3` HEAD after both slices land):

- With `CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data` set: `cargo test --locked --test sd19_seam_shapes_correctness` is green and exercises the end-to-end real-corpus assertion (the chassis-equality proof).
- Without `CORPUS_ROOT` set: the test passes the type-shape and serde-roundtrip assertions, skips the real-corpus end-to-end assertion with the documented `eprintln!` message (matching SD-17-B-4/B-5's skip semantics).
- `cargo test --locked` is green under both conditions (no SD-18 regressions; existing 2464+ tests still pass).
- `cargo clippy --locked --tests -- -D warnings` is clean.
- The commit messages reference this decisions file in their bodies and name the slices in their subject lines.
## 6. Cycle surface (per-cycle mechanics)

### 6.1 File-touch partition

The SD-19 cycle surface is:

| File | Purpose | Cycles that may touch it |
|---|---|---|
| `src/rules_core/pilot_compute.rs` | The chassis seam; SD-18 cycles touch this. **Untouched by SD-19's slice; untouched by SD-19 cycles that only extend the corpus-aware lane.** | One cycle at a time, but only when a cycle's chassis-side work demands it (rare; should not happen for §2.4/§2.5 reachability work). |
| `src/rules_core/pilot_compute_corpus.rs` | The corpus-aware seam dispatcher; the SD-19 main capability slice lives here and SD-19 cycles append one dispatcher line per cycle. | **One cycle at a time** (the dispatcher itself grows additively — each cycle adds exactly one line, never edits an existing one). |
| `src/rules_core/rules_tables/` | The canonical Paizo-table store; the SD-19 foundation slice populates CRB here, and SD-19 cycles' per-school/per-category contribution functions read from it. **Untouched by cycles** once the foundation slice lands; new tables or new book directories land in their own bundle. | One cycle at a time only when a cycle discovers a missing table cell and the cycle's RED test asserts the gap (rare; typically routes to Open Blockers). |
| `src/rules_core/school_coverage/<school>.rs` | Per-school contribution function; one file per school, added by the cycle that grounds that school. | **One cycle per file** (the file's owning cycle). |
| `src/rules_core/equipment_coverage/<category>.rs` | Per-category contribution function; one file per category, added by the cycle that grounds that category. | **One cycle per file** (the file's owning cycle). |
| `src/rules_core/support_state_matrix.rs` | The matrix carrier; every cycle updates a School or Equipment row | **One cycle at a time** |
| `src/rules_core/equipment_resolver.rs` | Equipment-id resolver; touched only if a cycle discovers a normalization edge case | One cycle at a time, only when needed |
| `src/rules_core/spell_resolver.rs` | Spell-id resolver; touched only if a cycle discovers a corpus-side key edge case | One cycle at a time, only when needed |
| `tests/sd19_<criterion>.rs` | Per-cycle test file | One cycle per file (its owning criterion) |
| `tests/fixtures/rules_core/sd19_<criterion>_*.txt` | Per-cycle fixture | One cycle per fixture |

The corpus-aware lane (`pilot_compute_corpus.rs` + `support_state_matrix.rs`) is the choke point for SD-19 cycles. **At most one SD-19 cycle may be active across these two files simultaneously** (the per-cycle dispatch line append is sequential and never overlaps with another cycle's append because the file is single-write). `pilot_compute.rs` is a separate lane that SD-19 cycles do not normally touch (the SD-18 loop may be using it for a different criterion, but the two loops don't run concurrently per `decisions.md` §1). The per-school and per-category sub-module files are owned exclusively by their owning cycle; no other cycle touches them. The `rules_tables/` directory is owned by the foundation slice and is read-only from cycles' perspective.

### 6.2 Per-cycle spawn budget

Default: **1 cycle at a time.** Reason: identical to SD-18's. The file-touch partition collapses any parallel attempt.

### 6.3 No ephemeral feature branches (per `decisions.md` §6)

Each cycle commits directly to `tranche/3` — no ephemeral branch, no auto-merge, no PR. The cycle's commit SHA on `tranche/3` is the durable receipt. Concurrency control is the file-touch partition, not branch isolation; this was already the real control in SD-18's posture, with the branch lifecycle being an artifact of the as-written SD-13 prompt's parallel-cycle assumption.

### 6.4 Per-cycle procedure

Each cycle follows the SD-18 procedure from `~/workspace/SD-18-core-rules-breadth-loop-instruction.md` §Per-cycle procedure, with these SD-19-specific differences:

- **No Step 3 feature-branch creation.** Skip directly from Step 1-2 reading to Step 4-7 TDD work, all on the `tranche/3` working tree.
- **No Step 6 push to a feature branch.** Commit lands directly on `tranche/3`.
- **No Step 7 PR creation.** Per `decisions.md` §6, SD-19 has no PRs.
- **No Step 8 auto-merge.** The commit is already on `tranche/3` by construction.
- **No Step 9 ephemeral-branch cleanup.** There is no branch to clean up.
- Step 10 (kanban card) and Step 11 (progress doc update) are unchanged, with the `merge_receipt_sha` field now holding the direct-commit SHA on `tranche/3` and the `feature_branch` field dropped from the card body schema.
- Card schema `row_or_kind` values: `school:abjuration` | `school:conjuration` | ... | `category:arms_armor` | `category:general` | `category:magic_items` | `category:equipmods`.
- Kanban board: `codex-tranche-3` (same board SD-18 uses).

### 6.5 Self-healing posture

Inherits SD-18's posture (self-healing as a structural requirement; non-self-healable failures write to `## Open blockers` and exit `FAIL`). SD-19 adds one SD-19-specific non-self-healable condition:

| Condition | Detection | Why not self-heal |
|---|---|---|
| Corpus-derived contribution cannot be grounded for a school or category (e.g. the corpus's spell records for that school all have malformed KEY: tokens, or the corpus's equipment records for that category all lack the derived-stat fields the cycle expects) | RED test cannot be made green by extending the seam; corpus-side defect surfaces | The fix is corpus-side repair, which is out of scope for SD-19 (corpus-side work is SD-17's lane) |

### 6.6 Hard stops

Inherits SD-18's Hard Stops (forbidden write scopes, sibling-regression rule, etc.). SD-19 removes two SD-18 stops that no longer apply:

- *"The chosen burden needs a new subsystem"* — SD-19's whole point is that the new subsystem *exists*. Cycles that discover a new-subsystem burden inside a cycle are correct to write `## Open blockers` and exit; they are no longer hard stops because the operator has explicitly authorized the subsystem work via this bundle.
- *"Conflict requires a domain decision on class-feature semantics"* — out of scope (no class-feature semantics changes in SD-19; the seam only adds corpus-derived contributions, not new class behavior).

SD-19 adds one new hard stop:

- *"A cycle's RED test depends on a corpus record that does not exist in the real PCGen corpus"* (e.g. the cycle asserts "spell 'Foo' is in the Conjuration strict-school slice" but `cr_spells.lst` contains no such record). Cycles must verify corpus existence against `CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data` before writing the RED test; a missing corpus record is a hard stop, not a self-heal.

### 6.7 No branch-control protocol

SD-18's `## Branch control protocol` (ephemeral-branch-as-claim) is **not applicable** to SD-19. Since there are no feature branches, the only concurrency signal is the file-touch partition in §6.1 and the `ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep` in-flight check in the loop brief's required-reading section. If two `claude` processes were to start cycles touching the same choke-point file, the file-touch partition's "one cycle at a time" rule is the enforcement mechanism — whichever cycle starts second will see the choke-point file dirty and exit `CLAIM-EXISTS`.

## 7. Capability slice landing targets

Per `decisions.md` §6, §8, and operator directive 2026-07-14, SD-19 ships in two atomic direct commits to `tranche/3`. No PR, no merge target separate from `tranche/3`, no sub-commits beyond the foundation/main pair.

### 7.1 Foundation slice commit message

```
feat(sd19): canonical Paizo-table store (foundation slice)

Implements the §1.0 foundation slice per
programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md
and technical-design.md.

Deliverables (foundation only — no seam, no resolvers, no wrapper types):
- src/rules_core/rules_tables/ module shell with RuleSetId enum
- src/rules_core/rules_tables/crb/class_tables.rs (PF1 CRB class tables)
- src/rules_core/rules_tables/crb/spell_list.rs (PF1 CRB spell list)
- src/rules_core/rules_tables/crb/equipment_tables.rs (PF1 CRB equipment tables)
- tests/sd19_table_store_foundation.rs (parse + resolve + populate assertions)

The main capability slice (compute_pilot_with_corpus, resolvers, wrapper
types, MatrixSubjectType extensions, CharacterInput.spells_selected, fixtures,
seam-shapes-correctness test) follows in a separate atomic commit.

Closes the bundle-1 prerequisite for tranche-4 per the operator's
2026-07-14 directive: "i'm thinking we should put bundle 1 as a deliverable
for SD-19. We haven't started it yet, so let's add it in there."
```

### 7.2 Main capability slice commit message

```
feat(sd19): corpus-aware compute seam (atomic slice)

Implements the §1.1 main capability slice per
programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md
and technical-design.md.

Atomic deliverable list (all in one commit per operator directive 2026-07-14):
- compute_pilot_with_corpus function (src/rules_core/pilot_compute_corpus.rs)
- CorpusPilotReceipt / CorpusDerivedSection / TableCellRef / SchoolCoverage /
  ResolvedEquipment / DerivedEquipmentStats wrapper types
- equipment_id_resolve function (NEW, with RuleSetId parameter)
- spell_id_resolve function (NEW, with RuleSetId parameter)
- CharacterInput.spells_selected field (Vec<SpellSelection>)
- MatrixSubjectType::School and ::Equipment variants
- support_state_matrix.rs row shapes for both
- tests/sd19_seam_shapes_correctness.rs (proof before loop runs)
- tests/fixtures/rules_core/sd19_seam_crb_*.txt (13 real-corpus fixtures,
  sourced from cr_spells.lst and the four cr_equip_*.lst files)

Depends on the foundation slice at the immediately previous commit.

Closes the structural gap documented in
~/workspace/SD-18-core-rules-breadth-progress.md
under the headings "## cycle-2026-07-15T0300 | §3.4 spell-school
reachability-chain investigation" and
"## cycle-2026-07-15T0400 | §3.5 equipment-category
reachability-chain investigation".

Anchored headings are used (not line numbers) because the progress doc
grows as SD-19 cycles land; section headers are stable, line numbers are
not. The grep-the-doc workflow is anchor-based: a future auditor searching
for either investigation cycle by its dated header will find the
underlying evidence regardless of where it sits in the file at audit time.
```

No separate SD-19 tranche branch, no `develop` promotion path during the slice window. Both slices and the loop all land on `tranche/3`, same as SD-18's loop. SD-19 cycles continue appending to the same `tranche/3` history that SD-18 has been building.

**Override flag:** none on the two-slice-vs-single-slice decision (operator-confirmed 2026-07-14). The only other override window is `decisions.md` §6's "if claude-cli's PR-merge friction is later resolved" clause, which would require reverting to a branch-lifecycle posture and re-patching this section.

## 8. Cross-reference

- `decisions.md` — the 8-item decision record.
- `epic-breakdown.md` — 15 acceptance criteria mapped to lanes.
- `technical-requirements.md` — pre-loop prerequisites.
- `acceptance-and-verification.md` — closure gates.
- `risks-and-open-questions.md` — per-criterion risks; the two open override flags.
- `~/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md` — canonical handoff doc.
- `~/workspace/SD-19-core-rules-spell-equipment-reachability-loop-instruction.md` — loop body.
- `~/workspace/SD-18-core-rules-breadth-progress.md` the dated cycle-2026-07-15T0300 (§3.4) and cycle-2026-07-15T0400 (§3.5) headers — the blocker entries this bundle closes.