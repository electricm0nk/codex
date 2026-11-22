---
title: SD-19 — Spell/Equipment Reachability — Scope Draft (Canonical Handoff)
status: reviewed (operator, 2026-07-16)
date: 2026-07-14
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/
mirror_of: /home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md §1
---

# SD-19 — Spell/Equipment Reachability + Canonical Paizo Table Store — Scope Draft

This is the canonical handoff document for SD-19. The loop reads this file directly. The doctrine record lives at `/home/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/`. Two audiences, two locations — see SD-18's `decisions.md` §10 for the pattern.

## 1. Pre-loop capability slices

SD-19 ships two atomic commits to `tranche/3` per `decisions.md` §8: a foundation slice first (the canonical Paizo-table store + `RuleSetId`), then the main capability slice (the corpus-aware compute seam + its resolvers + the wrapper types). Both ship linear, both into `tranche/3`.

### 1.0 Foundation slice — canonical Paizo-table store (atomic commit 1)

Per operator directive 2026-07-14: "i'm thinking we should put bundle 1 as a deliverable for SD-19." The canonical Paizo-table store is the structural prerequisite everything in tranche-4 depends on; folding it into SD-19's scope preserves the dependency-ordering that prevents future subsystems from synthesizing fake data or hard-coding the table twice.

**Concrete deliverables:**

- **Table store module shell.** Add `src/rules_core/rules_tables/mod.rs` (NEW) exporting a `pub enum RuleSetId { Crb, /* future: Um, Apg, ... */ }`. CRB is the only variant populated today; future rule books (UM, APG, etc.) get sibling directories and their own enum variants in their own STC sub-bundle.
- **CRB class tables.** Add `src/rules_core/rules_tables/crb/class_tables.rs` (NEW) — one entry per class per level, with named features and exact spell/per-day cells from the CRB. Paizo's table format, not derived formulas; the source of truth for what a character looks like at level N.
- **CRB spell list.** Add `src/rules_core/rules_tables/crb/spell_list.rs` (NEW) — one entry per spell with `KEY:` token, strict-school partition, level, and description text.
- **CRB equipment tables.** Add `src/rules_core/rules_tables/crb/equipment_tables.rs` (NEW) — one entry per item with `KEY:` token, category, name, slot, derived stats where applicable.
- **Foundation test.** Add `tests/sd19_table_store_foundation.rs` (NEW) asserting the CRB directory is parseable, `RuleSetId::Crb` resolves, every class table has entries for levels 1-20, every spell list has at least one spell per school, every equipment table is non-empty.

The SD-18 loop's own investigation cycles (cycle-2026-07-15T0300 for §3.4, cycle-2026-07-15T0400 for §3.5) proved at first-hand code depth that both spell schools and equipment categories are blocked on the same structural gap: `pilot_compute.rs` has no corpus-aware compute path, `CharacterInput` has no spell-content or equipment-id selection mechanism that links to corpus identity, and the SD18-PRELOOP `ComposedCharacterInput` is built and immediately discarded at every call site. The main capability slice (§1.1) closes that gap; the foundation slice (§1.0) supplies the canonical Paizo tables the seam consumes.

**Concrete deliverables:**

- **Seam function and wrapper type.** Add to `src/rules_core/pilot_compute_corpus.rs` (NEW module):
  - `pub struct CorpusPilotReceipt { pub base: PilotReceipt, pub corpus_derived: CorpusDerivedSection }` — the wrapper return type.
  - `pub struct CorpusDerivedSection { pub school_coverage: BTreeMap<Pf1SchoolId, SchoolCoverage>, pub equipped_items: Vec<ResolvedEquipment> }` — the corpus-derived contributions.
  - `pub struct TableCellRef { pub rule_set: RuleSetId, pub table: String, pub row_key: String, pub column_key: String }` — the canonical Paizo-table-cell reference; each `SchoolCoverage` and each `ResolvedEquipment` carries one, asserting the corpus record the seam resolved lives at a specific CRB table cell.
  - `pub struct SchoolCoverage { pub school: Pf1SchoolId, pub spells: Vec<String>, pub table_cell: Option<TableCellRef> }` — one entry per school that resolved.
  - `pub struct ResolvedEquipment { pub item_id: String, pub equipment_record_name: String, pub equipment_record_key: String, pub derived_stats: DerivedEquipmentStats, pub table_cell: Option<TableCellRef> }` — one entry per equipped item that resolved.
  - `pub struct DerivedEquipmentStats { pub armor_bonus: Option<i16>, pub attack_bonus: Option<i16>, pub max_dex: Option<i16>, pub spell_failure: Option<f32> }` — the bounded baseline of equipment-derived stats.
  - `pub fn compute_pilot_with_corpus(input: &CharacterInput, corpus: &SourcePackageContent) -> CorpusPilotReceipt` — the seam itself. Calls `compute_pilot_base_chassis` (imported from `pilot_compute.rs`) internally; wraps the result in `CorpusPilotReceipt { base, corpus_derived }`. `pilot_compute.rs` itself stays untouched — every landed SD-18 cycle continues to work unchanged, and the slice's diff to the choke-point file is zero.

- **Equipment resolver.** Add `src/rules_core/equipment_resolver.rs` with `pub fn equipment_id_resolve<'a>(item_id: &str, rule_set: RuleSetId, corpus: &'a SourcePackageContent) -> Option<(&'a EquipmentRecord, Option<TableCellRef>)>`. Lookup rule: primary index on normalized `EquipmentRecord.name` (lowercase, spaces → underscores, strip parenthesized qualifiers), secondary index on verbatim `KEY:` token. If the corpus record resolves and the foundation slice's table store has a matching row, returns `Some(record, Some(ref))`; otherwise returns `Some(record, None)`. Preserves the existing fixture namespace (`"item:longsword"`, `"item:chain_shirt"`).

- **Spell resolver.** Add `src/rules_core/spell_resolver.rs` with `pub fn spell_id_resolve<'a>(spell_id: &str, rule_set: RuleSetId, corpus: &'a SourcePackageContent) -> Option<(&'a SpellRecord, Option<TableCellRef>)>`. Lookup rule: exact match on verbatim `KEY:` token. If the corpus record resolves and the foundation slice's table store has a matching row, returns `Some(record, Some(ref))`; otherwise returns `Some(record, None)`. Spell identity is unambiguous in the corpus (KEY: tokens are unique per record) so no normalization is needed.

- **`CharacterInput` extension.** Add `SpellSelection { spell_id: String, source_class_id: ClassId, acquisition_mode: AcquisitionMode }` and `AcquisitionMode { Known | Prepared | Granted }` to `src/rules_core/character_input.rs`. Add `pub spells_selected: Vec<SpellSelection>` to `ChosenCharacterState`. The existing `equipment_selections: Vec<EquipmentSelection>` is unchanged.

- **Matrix carrier extension.** Add `School(Pf1SchoolId)` and `Equipment(EquipmentCategory)` variants to `MatrixSubjectType` in `src/rules_core/support_state_matrix.rs`, with full row-shape support mirroring the existing `Race` and `Class` row shapes. `Pf1SchoolId` enumerates the 9 PF1 strict schools; `EquipmentCategory` enumerates the 4 core-rulebook categories.

- **Seam correctness proof.** Add `tests/sd19_seam_shapes_correctness.rs` that proves: (a) the seam function and wrapper types exist with the documented signatures; (b) the resolvers return `Some` for the documented fixture set and `None` for the documented unknown set; (c) `CharacterInput.spells_selected` round-trips through serde; (d) the new `MatrixSubjectType` variants round-trips through serde; (e) a sample end-to-end call reads each of the 13 `tests/fixtures/rules_core/sd19_seam_*.txt` files via the SD17-B-4 spell parser (`src/pcgen_import/lst_parser/spell.rs:488`) and the SD17-B-5 equipment parser (`src/pcgen_import/lst_parser/equipment.rs:781`), and produces a `CorpusPilotReceipt` whose `corpus_derived` section is non-empty AND whose `base` field equals the same input run through `compute_pilot_base_chassis` directly. The fixtures are real test input, not inert documentation; a corpus-side drift that breaks parser fixtures breaks the seam test too. The test has no `CORPUS_ROOT` dependency — the per-cycle `CORPUS_ROOT` real-corpus pattern from `decisions.md` §6.6 applies separately to per-cycle fixtures at loop time.

- **Real-corpus fixtures.** Add `tests/fixtures/rules_core/sd19_seam_crb_*.txt` covering 13 fixtures — one spell per school (9 files: `sd19_seam_crb_spell_<school>.txt`) and one item per category (4 files: `sd19_seam_crb_equip_<category>.txt`). Authoring per operator directive 2026-07-14: hand-typed at slice-ship, copied verbatim from the real PCGen corpus records (read from `cr_spells.lst` for spells and the four `cr_equip_*.lst` files for equipment). Each fixture file is prepended with a one-line comment naming its source: e.g. `# Fixture for §2.4 Abjuration, source: cr_spells.lst, KEY: Shield of Faith`. Fixtures are checked into the repo; a future operator verifies currency by re-running the slice executor's grep against the corpus and confirming the source line still matches the fixture content. No helper script, no live-only references.

**What this slice does NOT do** (per `decisions.md` §1.3 and `technical-design.md` §1.3):

- It does not compute slot math (spells prepared per day, bonus slots, DCs).
- It does not unblock the 7 currently-live claim-blocking diagnostics (`class_spell.<class>.<burden>.unsupported`).
- It does not compute equipment effects beyond what the per-cycle grounding pattern asserts (bounded baseline: AC, attack bonus, max dex, spell failure).

These remain out of scope by deliberate architectural decision. They are the deliverable of future SD-N, not SD-19.

**Acceptance criterion (per this card):**

- `cargo test --locked --test sd19_seam_shapes_correctness` is green.
- `cargo test --locked` is green with zero SD-18 regressions (all 2464+ existing tests still pass).
- `cargo clippy --locked --tests -- -D warnings` is clean.
- The capability-slice PR is open against `develop` with the full deliverable list above.

## 2. Loop-routed coverage

### 2.4 Spell school cards (9 cards: Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, Universal)

For each PF1 strict school, prove the school's spells are reachable from a chosen `CharacterInput.spells_selected` and consumable by the rules engine through the corpus-aware compute seam.

**Concrete corpus and code surface:**

- Spell corpus: `core_rulebook/cr_spells.lst`. ~652 spell records with strict `SCHOOL:` tags. Per-school distribution (already verified live by the SD-18 §3.4 investigation cycle): Transmutation 152, Conjuration 116, Evocation 87, Abjuration 73, Necromancy 62, Enchantment 60, Divination 50, Illusion 47, Universal 5.
- Spell parser: `src/pcgen_import/lst_parser/spell.rs:488`. Corpus conversion: `src/pcgen_import/ir_converter.rs::convert_spell_record`/`convert_spell_file` (already proven generic, no per-school special-casing anywhere).
- Resolver: `src/rules_core/spell_resolver.rs::spell_id_resolve` (lands in the capability slice).
- Seam: `src/rules_core/pilot_compute.rs::compute_pilot_with_corpus` (lands in the capability slice).
- Matrix row: `MatrixSubjectType::School(Pf1SchoolId)` (lands in the capability slice).

**Acceptance criterion (per school card):** every spell in the school's slice is (a) resolvable via `spell_id_resolve(spell_id, corpus) -> Some(&SpellRecord)` when the chosen `CharacterInput.spells_selected` carries the spell's `KEY:` token; (b) present in the receipt's `corpus_derived.school_coverage[<school>].spells` list after a call to `compute_pilot_with_corpus`; (c) tagged with the correct `acquisition_mode`; (d) reflected in the corresponding `MatrixSubjectType::School(Pf1SchoolId)` row with `support_state=Supported` and `evidence_tier=Product-visible` after the per-cycle grounding work.

End-user-visible proof: a L10 casting-class character whose `CharacterInput.spells_selected` carries a representative sample of the school's spells demonstrates the school reaches the rules engine. The proof is the corpus-derived `school_coverage[<school>]` entry in `CorpusPilotReceipt.corpus_derived`, not a spell-effect execution trace.

### 2.5 Equipment category cards (4 cards: arms_armor, general, magic_items, equipmods)

For each of the four `core_rulebook/cr_equip_*.lst` files, prove the equipment in that category is reachable from a chosen `CharacterInput.equipment_selections`, resolved via `equipment_id_resolve`, and produces corpus-derived stat contributions in the receipt.

**Concrete corpus and code surface:**

| Category | File | Content shape | Parser |
|---|---|---|---|
| arms_armor | `cr_equip_arms_armor.lst` | weapons and armor (PCGen object-kind tags) | `parse_equipment_file` in `equipment.rs:781` |
| general | `cr_equip_general.lst` | adventuring gear (poisons, mounts, vehicles) | same |
| magic_items | `cr_equip_magic_items.lst` | scrolls (~634), wands (~351), potions (~87), rings (~59), belts (~22), etc. | same |
| equipmods | `cr_equipmods.lst` | equipment modifiers (BONUS: chains) | same |

- Resolver: `src/rules_core/equipment_resolver.rs::equipment_id_resolve` (lands in the capability slice).
- Seam: `src/rules_core/pilot_compute.rs::compute_pilot_with_corpus` (lands in the capability slice).
- Matrix row: `MatrixSubjectType::Equipment(EquipmentCategory)` (lands in the capability slice).

**Acceptance criterion (per category card):** every item in the category is (a) resolvable via `equipment_id_resolve(item_id, corpus) -> Some(&EquipmentRecord)` when the chosen `CharacterInput.equipment_selections` carries the item's id (in the existing `"item:<name>"` fixture namespace or in the corpus `KEY:` token); (b) present in the receipt's `corpus_derived.equipped_items` list after a call to `compute_pilot_with_corpus`, with the item's `equipment_record_name`, `equipment_record_key`, and `derived_stats` (armor_bonus, attack_bonus, max_dex, spell_failure) populated; (c) reflected in the corresponding `MatrixSubjectType::Equipment(EquipmentCategory)` row with `support_state=Supported` and `evidence_tier=Product-visible` after the per-cycle grounding work. **Pre-condition:** the SD-17 parser-merge defect at `src/pcgen_import/lst_parser/equipment.rs:544-553` (and the `.COPY=` strip at lines 383-389) must be fixed before this criterion can be met; otherwise full coverage is structurally blocked. Per category, the count of items that must ground = the line count of the relevant `cr_equip_*.lst` file (verified by `wc -l` against the live corpus). Per operator directive 2026-07-16 ("we brought in ALL spells, ALL armor, ALL weapons, ALL equipment, not just samples"), §2.5 expands from the original "representative sample" criterion to "every item," symmetric with §2.4's "every spell."

End-user-visible proof: a character whose `CharacterInput.equipment_selections` carries **every item** in the category demonstrates the category reaches the rules engine at full coverage. The proof is the `equipped_items` list in `CorpusPilotReceipt.corpus_derived`, with `derived_stats` populated for items whose CRB table cell defines them.

## 3. Closure gates

See `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/acceptance-and-verification.md` for the 7-gate closure posture. The gate that matters most here: every acceptance criterion in §2 of this doc is `done` in the progress doc with row id, branch, merge SHA, and card id; every corresponding matrix row reaches `Supported/Product-visible`.

## 4. Loop execution posture

### 4.1 Per-iteration commit lifecycle

Each iteration commits directly to `tranche/3` — no ephemeral feature
branch, no auto-merge, no PR. The commit SHA on `tranche/3` is the
durable receipt. SD-19 shares `tranche/3` with SD-18 per operator
directive 2026-07-14; SD-19 will not begin until SD-18's loop completes.

**Concrete command sequence per iteration** (full detail in `technical-design.md` §6.4):

```bash
# Step 1-2: read state (scope doc + SD-18/SD-19 progress doc + SD-18 §3.4/§3.5 blockers + live git state + in-flight detection)
cat /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
cat /home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md
grep -A 9999 "## cycle-2026-07-15T0300 | §3.4 spell-school reachability-chain investigation" /home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md
# and
grep -A 9999 "## cycle-2026-07-15T0400 | §3.5 equipment-category reachability-chain investigation" /home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
git log origin/tranche/3 --oneline -5
ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep   # in-flight check

# Step 3: verify working tree on tranche/3
git checkout tranche/3
git pull origin tranche/3
git status --porcelain | wc -l   # expect 0

# Step 4-7: TDD
cargo test --locked --test sd19_<criterion> 2>&1 | tail -40   # RED
# <implement>
cargo test --locked 2>&1 | tail -20                          # GREEN
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20 # CLEAN

# Step 8: commit, push directly to tranche/3
git add src/rules_core/pilot_compute.rs \
        src/rules_core/support_state_matrix.rs \
        tests/sd19_<criterion>.rs \
        tests/fixtures/rules_core/sd19_<criterion>_*.txt
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd19): <criterion> (<row transition>)"
git push origin tranche/3

# Step 9: mint kanban card (post-mortem record)
hermes kanban --board codex-tranche-3 create \
  "SD19 <criterion> (<criterion-section>) [cycle <cycle-id>]" \
  --assignee operator \
  --workspace scratch \
  --initial-status done \
  --created-by operator \
  --priority 3 \
  --body "<card body per schema — see loop brief §Step 10>"

# Step 10: update progress doc (append to SD-19 section of the shared SD-18 progress doc)
# Step 11: exit
```

### 4.2 Concurrency rules (read first, obey always)

Inherit SD-18's file-touch partition (see SD-18 loop brief §Concurrency rules). SD-19-specific touches: `src/rules_core/equipment_resolver.rs` and `src/rules_core/spell_resolver.rs` are added to the partition as "touched only if a cycle discovers a normalization/key edge case" — one cycle at a time when needed, zero cycles otherwise.

### 4.3 Eligibility check (read first, obey always)

Inherit SD-18's eligibility check from the SD-18 loop brief. SD-19 removes two conditions the seam has now satisfied:

- *"It is an arithmetic extension or a single-seam recognition record, NOT a new subsystem"* — SD-19's whole point is that the new subsystem (the seam) exists. A cycle that exercises the seam to ground a school's reachability is a normal cycle-level landing, not a forbidden new-subsystem landing.
- *"The chosen burden needs a new subsystem"* — removed; same reason.

SD-19 adds one new condition:

- *The cycle's RED test must verify that the corpus record it asserts exists in the real PCGen corpus, against `CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data`, before writing the test.* A cycle whose asserted corpus record does not exist is a hard stop, not a self-heal.

### 4.4 Progress doc structure

SD-19 cycles append to the shared `~/workspace/SD-18-core-rules-breadth-progress.md` (because SD-19 shares `tranche/3` with SD-18). The shared progress doc gains a new `## SD-19 cycles` section after the existing SD-18 content, with this structure:

```yaml
---
title: Tranche-3 SD-18 Core Rules Breadth — Loop Progress  (existing; unchanged)
mirrors: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
       + /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md  (added)
created: <first-cycle-timestamp>
snapshot_as_of: "origin/tranche/3 @ <short-sha>"  (shared line, both SD-18 and SD-19 cycles update)
---

# SD-18 progress doc  (existing; unchanged)

## Status summary
## §3.1 Race rows  (existing)
## §3.2 Class rows  (existing)
## §3.3 Interaction rows  (existing)
## §3.4 Spell schools (existing; 9 cards, 0/9 landed at slice start)
## §3.5 Equipment categories (existing; 4 cards, 0/4 landed at slice start)
## Open blockers  (shared; SD-18 and SD-19 cycles both append)
## Cycle log  (existing; SD-18 cycles append)

## SD-19 cycles  (NEW; SD-19 cycles append here)

### §2.4 Spell schools (9)
- Abjuration @ supported/Product-visible | commit ... | card t_xxxxx
- ...

### §2.5 Equipment categories (4)
- arms_armor @ supported/Product-visible | commit ... | card t_xxxxx
- ...

### SD-19 cycle log
<one entry per SD-19 cycle, dated>
```

The existing SD-18 §3.4/§3.5 sections remain in place as the pre-slice state
record. SD-19 cycles update the corresponding rows there (moves `done` status
into the row's last column) AND add a `## SD-19 cycles` cross-reference so a
reader can find the SD-19 cycle log entries without scrolling through SD-18
content. The `## Open blockers` block is shared.

## 5. Cross-reference

- `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md` — the 8-item decision record; required reading for understanding the SD-19 shape.
- `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/technical-design.md` — seam/resolver signatures and cycle surface.
- `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/technical-requirements.md` — pre-loop prerequisites.
- `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/acceptance-and-verification.md` — closure gates.
- `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/risks-and-open-questions.md` — the two open override flags.
- `~/workspace/SD-18-core-rules-breadth-progress.md` — under the headings "## cycle-2026-07-15T0300 | §3.4 spell-school reachability-chain investigation" and "## cycle-2026-07-15T0400 | §3.5 equipment-category reachability-chain investigation" (anchored headings, not line numbers, because the progress doc grows as SD-19 cycles land).
- `/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-loop-instruction.md` — the loop body the `/loop` invocation reads.