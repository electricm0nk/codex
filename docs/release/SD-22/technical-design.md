---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-15; operator directives 2026-07-17 expanded scope to APG + ACG; operator clarification 2026-07-18: "ACG, APG are the two advanced guides"; branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; bundle marked planning-ready)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
---

# SD-22 — Technical Design

This file is the load-bearing engineering design surface for SD-22. `decisions.md` records *why* SD-22 is shaped the way it is; this file records *what* the shape is at the code level. A future implementer should be able to work from this document with no further clarification needed.

## 1. Content-source ingest architecture (Epics 3 + 4 + 5)

SD-22's three content-source ingest epics share a common pattern (per SD-19 §9 source-book subdirectories):

```
src/rules_core/rules_tables/
├── crb/        (SD-19 ships; SD-21 reads; SD-22 does NOT touch)
├── apg/        (SD-22 Epic 3)
├── acg/        (SD-22 Epic 4)
└── beastiary1/ (SD-22 Epic 5)
```

Each `<book>/` directory has the same shape (mirroring SD-19's `rules_tables/crb/`):

```
<book>/
├── mod.rs           (the book-level module: registration, index, RuleSetId variant)
├── class_<class>.rs (one file per class table; for APG/ACG only)
├── monster_<subset>.rs (one file per monster-block subset; for Bestiary 1 only)
├── spell_list.rs    (book's spells; structured data, KEY: tokens)
└── equipment_tables.rs (book's equipment; structured data, KEY: tokens)
```

### 1.1 `RuleSetId` enum extension

SD-22 extends the `RuleSetId` enum (defined in SD-19):

```rust
// SD-22's contribution to the canonical enum
pub enum RuleSetId {
    Crb,           // SD-19 (sibling; SD-22 doesn't own)
    Apg,           // SD-22 Epic 3
    Acg,           // SD-22 Epic 4
    Bestiary1,     // SD-22 Epic 5
    // No Ultimate-variant entries; Ultimate books are NOT in scope per operator 2026-07-18
}
```

### 1.2 Per-book resolver pattern

The existing `equipment_id_resolve(item_id: &str, rule_set: RuleSetId, corpus: &SourcePackageContent) -> Option<...>` and `spell_id_resolve` resolver signatures (per SD-19 §3, extended by SD-21 §12 for cross-book fallback) are reused by SD-22. Each book's `mod.rs` exports a `RuleSetId::<Book>::resolve_*` function that the resolver chain dispatches to based on the `rule_set` parameter.

The cross-book priority order (per SD-21 §12 doctrine) is **APG → CRB → ACG → Bestiary1** when the resolver falls back. SD-22's per-book resolver functions implement that priority chain.

### 1.3 Per-class cycle shape

One cycle lands one class table for one book. For APG's Alchemist class, the cycle:

1. Writes `src/rules_core/rules_tables/apg/class_alchemist.rs` with structured data: `class_alchemist::BAB_PROGRESSION`, `class_alchemist::SAVE_PROGRESSION`, `class_alchemist::FEATURES_BY_LEVEL: HashMap<u8, Vec<Feature>>`, `class_alchemist::SPELLS_PER_DAY_BY_LEVEL: HashMap<u8, SpellSlots>`, etc.
2. Writes the corresponding per-cycle test `tests/sd22_apg_class_alchemist_resolves.rs`.
3. Commits and mints a kanban post-mortem card on `codex-tranche-5`.

For ACG's Arcanist class, the cycle does the same with `acg/class_arcanist.rs`. For Bestiary 1's first monster-block subset, the cycle writes `beastiary1/monster_<subset>.rs` with structured monster data.

### 1.4 Per-monster-block cycle shape (Bestiary 1)

Bestiary 1's 300+ monsters are split into monster-block subsets (operator-pinned at SD-22 cycle launch; default: alphabetical by monster name within CR band). One cycle lands one subset.

## 2. DM-toolkit architecture (Epic 6)

### 2.1 Encounter-difficulty module

```rust
// src/rules_core/encounters.rs (NEW)
use crate::rules_core::party_cr::party_challenge_rating;
use crate::rules_core::rules_tables::*;  // MonsterRef, CharacterSnapshot

pub enum Difficulty { Easy, Medium, Hard, Deadly }

pub struct EncounterResult {
    pub difficulty: Difficulty,
    pub total_xp: u32,
    pub adjusted_xp: u32,  // accounting for monster-count multipliers
}

pub fn encounter_difficulty(
    party: &[CharacterSnapshot],
    monsters: &[MonsterRef],
) -> EncounterResult {
    // Per PF1 "Encounter Building" rules:
    // 1. Sum monster XP values to get raw XP
    // 2. Apply monster-count multiplier (per PF1's table)
    // 3. Compare adjusted XP against party's CR thresholds (per PF1's table by party size + average level)
    // 4. Return Difficulty
}
```

### 2.2 Party-CR module

```rust
// src/rules_core/party_cr.rs (NEW)
use crate::rules_core::CharacterSnapshot;

pub fn party_challenge_rating(party: &[CharacterSnapshot]) -> f32 {
    // Per PF1 "Determining Party Strength" rules:
    // 1. Sum each character's CR contribution (per character level / class table)
    // 2. Apply class-difficulty modifier (per PF1's table)
    // 3. Average across the party
    // 4. Return the average as f32
}
```

### 2.3 DM-toolkit tests

The DM-toolkit ships with deterministic tests against canonical Paizo examples:
- 4 level-3 PCs vs. 1 CR-2 monster → Easy
- 4 level-3 PCs vs. 4 CR-3 monsters → Hard
- Party of 4 level-3 PCs → CR ~3.5

The happy-path integration test consumes an ingested `PartySnapshot` (mixed-class, with at least one APG class from Epic 3 and one ACG class from Epic 4) and a `MonsterRef` from Epic 5's first ingested monster-block subset, then runs the encounter-difficulty computation and asserts the result matches the canonical Paizo encounter-table.

## 3. Cross-cutting authority surface

| Epic | Authoritative for | Forbidden to fabricate |
|---|---|---|
| Epic 1 — Code-Side Identifier Cleanup | Identifier audits + renames in source per the identifier-discipline doctrine (`../../doctrine-external/identifier-discipline.md`) | Any feature work. Epic 1 only runs audits and renames; doesn't change behavior. |
| Epic 2 — Operator Pre-Launch | Board-exists / branch-pushed / clean-state verification | Any cycle work. Epic 2 is gating only. |
| Epic 3 — APG content-source ingest | APG class tables + spells + equipment + `RuleSetId::Apg` variant | Anything outside APG content. Per-class test fixtures are not "examples" — they reflect APG's published content. |
| Epic 4 — ACG content-source ingest | ACG class tables + spells + equipment + `RuleSetId::Acg` variant | Anything outside ACG content. |
| Epic 5 — Bestiary 1 content-source ingest | Bestiary 1 monster data + `RuleSetId::Bestiary1` variant | Anything outside Bestiary 1 content. DM-toolkit math (Epic 6) is separate. |
| Epic 6 — DM Toolkit | Encounter-difficulty + party-CR math; consumes Epic 3+4+5 output | The PF1 encounter-building rules themselves (PF1 is publisher-canonical); rules-engine correctness violations. |
| Epic 7 — Closure Epilogue | Final criterion scan + closure PR (`tranche/5 → develop`) + worktree cleanup + release-notes generation + tranche-position version increment | The specific `0.5.<current_build>` value (that's Epic 8). Worktree cleanup outside the SD-22 lane. Auto-merging the closure PR (`decisions.md §6 no-branches convention). |
| Epic 8 — Build Version Numbering | Three version fields (package.json, tauri.conf.json, Cargo.toml) set to `0.5.<current_build>`, build-label format (`Codex 0.5.<build>`), `docs/SD-22/release-closure-checklist.md` | Per-CI-build *build*-counter automation. Major-publish automation. Build-label parsing. |

APG / ACG / Bestiary 1 content-source ingest is **SD-22's primary surface** (operator directive 2026-07-17). Ultimate-line books are NOT in scope (per operator clarification 2026-07-18).

## 4. File-touch partition (defense for the loop)

Per-epic module placement:

- **Epic 1 — Identifier Cleanup**: source-code audit + identifier renames across the existing codebase (no new files unless operator-pinned). One cycle per identifier-class.
- **Epic 2 — Operator Pre-Launch**: no code surface; the three criteria are operator-only pre-flight checks.
- **Epic 3 — APG content-source ingest**: `src/rules_core/rules_tables/apg/` (NEW directory; APG structured-data files). Per-cycle: `apg/class_<class>.rs`, `apg/spell_list.rs`, `apg/equipment_tables.rs`. Add `RuleSetId::Apg` if not yet added.
- **Epic 4 — ACG content-source ingest**: `src/rules_core/rules_tables/acg/` (NEW directory; symmetric to APG).
- **Epic 5 — Bestiary 1 content-source ingest**: `src/rules_core/rules_tables/beastiary1/` (NEW directory; per-monster-block subsets).
- **Epic 6 — DM Toolkit**: `src/rules_core/encounters.rs` (NEW), `src/rules_core/party_cr.rs` (NEW).
- **Epic 7 — Closure Epilogue**: `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/release-notes.md` (NEW; generated by the loop's release-notes generator) + the closure PR itself.
- **Epic 8 — Build Version Numbering**: `apps/desktop/package.json` (version field set to `0.5.<current_build>`), `apps/desktop/src-tauri/tauri.conf.json` (version field set to `0.5.<current_build>`), `apps/desktop/src-tauri/Cargo.toml` (version field set to `0.5.<current_build>`), `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts:61` (`BUILD_PREFIX = 'Codex'`), `createSd11WorkbenchStatus.ts:72-74` (template `${BUILD_PREFIX} ${buildVersion}` rendering `<major>.<tranche>.<build>` as space-separated display), three test-fixture files. New `docs/SD-22/release-closure-checklist.md`.

The existing chassis and corpus-aware seam files (`src/rules_core/pilot_compute.rs`, `pilot_compute_corpus.rs`, `support_state_matrix.rs`) stay untouched by SD-22's Epic 1 (defensive cleanup). SD-22's Epic 6 (DM Toolkit) reads from `src/rules_core/rules_tables/<book>/` after Epic 3+4+5 land.

## 5. Rule-cycle table (Epic 3+4+5 vs Epic 6 — the load-bearing dependency)

The rule-cycle table binds each source-book content unit (rules) to its consumer (cycles). Epic 6 (DM Toolkit) consumes Epic 3+4+5 output; without those epics the happy-path integration test cannot land.

| Source (rule) | Cycle-of-record (per `corpus-source-inventory.md`) | Type | RuleSetId | Consumed by |
|---|---|---|---|---|
| CRB class table for Fighter | (SD-19 already ships; index in `src/rules_core/rules_tables/crb/`) | Existing rule | `RuleSetId::Crb` | SD-21's Epic 6 Wizard + Epic 7 Multiclass; SD-22's Epic 6 (read-only consumption in deterministic + happy-path tests) |
| CRB class table for Wizard | (SD-19 ships) | Existing rule | `RuleSetId::Crb` | SD-21's Epic 6 + Epic 7; SD-22's Epic 6 |
| CRB class tables for the 9 remaining core classes | (SD-18 ships; extended by SD-19) | Existing rule | `RuleSetId::Crb` | SD-21 Epic 6 (per-class); SD-22 Epic 6 (read-only) |
| APG class tables (Alchemist, Cavalier, Gunslinger, Inquisitor, Magus, Oracle, Summoner, Witch — 8 classes) | SD-22 Epic 3 (per-class cycles; 8 cycles) | New rule | `RuleSetId::Apg` | SD-22 Epic 6 (deterministic + happy-path) |
| APG spell list | SD-22 Epic 3 (criterion 9 shared spell-list cycle) | New rule | `RuleSetId::Apg` | SD-22 Epic 6 (deterministic spells/DC tests) |
| APG equipment tables | SD-22 Epic 3 (criterion 9 shared equipment cycle) | New rule | `RuleSetId::Apg` | SD-22 Epic 6 (deterministic equipment tests) |
| ACG class tables (Alchemist, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest — 10 classes) | SD-22 Epic 4 (per-class cycles; 10 cycles) | New rule | `RuleSetId::Acg` | SD-22 Epic 6 (read-only consumption) |
| ACG spell list | SD-22 Epic 4 (criterion 13 shared spell-list cycle) | New rule | `RuleSetId::Acg` | SD-22 Epic 6 |
| ACG equipment tables | SD-22 Epic 4 (criterion 13 shared equipment cycle) | New rule | `RuleSetId::Acg` | SD-22 Epic 6 |
| Bestiary 1 monster subsets (default: 8-12 subsets, alphabetical by CR-band-then-name) | SD-22 Epic 5 (per-subset cycles; default 8 cycles) | New rule | `RuleSetId::Bestiary1` | SD-22 Epic 6 (encounter-difficulty consumption) |
| Paizo encounter-math | (PF1 publisher-canonical; not ingested; SD-22 Epic 6 implements deterministic functions against the published rules) | External | (no RuleSetId; rules from publisher) | SD-22 Epic 6 |
| Paizo party-strength math | (PF1 publisher-canonical; same as encounter-math) | External | (no RuleSetId) | SD-22 Epic 6 |
| SD-21's per-character engine (PilotReceipt, CharacterSnapshot, etc.) | (SD-21 ships) | Existing output | (n/a) | SD-22 Epic 6 happy-path (consumes Epic 3+4+5 PartySnapshots + MonsterRefs to produce an `EncounterResult`) |

The dependency order is strict: Epic 6 cannot start its happy-path test until at least one ingested PartySnapshot (from Epic 3 + Epic 4) and one ingested MonsterRef (from Epic 5) exist. SD-22's `loop-instruction.md` Step 1 enforces this by checking Epic 3+4+5's status matrix before Epic 6 cycles pick up — the cycle picker rejects Epic 6 cycles if their inputs aren't clean.

## 6. Cross-reference

- `acceptance-and-verification.md` — 16 closure gates.
- `decisions.md` — the 3-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions).
- `epic-breakdown.md` — 30 acceptance criteria grouped into 8 epics.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags (Flag A through Flag D; Open Q1 through Open Q5).
- `technical-requirements.md` — pre-loop prerequisites.
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md` — canonical handoff; carries the prominent-early `/loop /goal` OPERATING METHOD callout (`/batch` deferred per `decisions.md §5`).
- `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md` — loop body.
- `~/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/` — Tranche-3 corpus-source ingest pattern (source-book sibling-directory convention).
- `~/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/` — per-character rules-engine surface.
- `~/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/` — Tranche-4-1 sibling bundle; SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22 will provide.
