# SD-24 — Technical Design

> **Operating method:** see `./scope-draft.md`. Architectural surface for the bundle.

## 1. Architectural posture

SD-24's architectural posture is **loose-ends reconciliation + new-feature stabilization**, with three load-bearing surfaces:

1. **Audit-driven remediation** (Epic 3 + 4 + 6). Read-only sweep across the codebase → coverage/coverage-matrix → remediation cycles. No new product features; pure reconciliation.
2. **Multiclass stacking real-and-full** (Epic 5) — Fighter + Wizard only. New behavior on the existing per-character rules-engine (`src/rules_core/pilot_compute.rs`); depends on Epic 4's coverage report.
3. **Tauri command-surface extension** (Epic 7). New commands (`appendToCharacter`, `recomputeCharacter`, `reSaveCharacter`) on top of the existing `create_character` / `list_saved_characters` / `load_saved_character` surface (per duracon 2026-07-18 18:20:41 sentinel).

## 2. Multiclass dispatch design (Epic 5)

### 2.1 Phase ordering

The SD-21 Epic 9 multiclass work shipped Fighter-only compute (per duracon 2026-07-18 20:17:52; `src/rules_core/pilot_compute.rs:4568`). SD-24 Epic 5 extends that:

1. **Phase A — Coerce Wizard compute path.** Extend the `pilot_compute.rs` `decideEligibility` switch to handle Wizard alongside Fighter. Test surface: Wizard level 1 → 10 deterministic.
2. **Phase B — Multiclass dispatch.** Add a per-character class-list field; let a character carry 1-N classes. The multiclass dispatch walks the class list and:
   - BAB: best-progression (PF1 rules per class table)
   - Saves: best-fractional-progression (PF1 rules)
   - Caster level: stack per class's caster-level-advance rate (Wizard: full progression; Fighter: 0)
3. **Phase C — Split-class progression.** At level 5, the test fixture splits the character into Fighter (lv 5) + Wizard (lv 5) by adding the Wizard class on level 5. Levels 6-10 advance both. The dispatch handles fractional per-class BAB / saves.

### 2.2 Test surface

Per Criterion 5.2: 30 character-advancement cycles cover:

- 10 cycles for Fighter alone (lv 1 → 10)
- 10 cycles for Wizard alone (lv 1 → 10)
- 10 cycles for Fighter + Wizard split at lv 5, advance to lv 10 each side (20 character-side advancements)

These are deterministic against canonical PF1 expected values from `archetype_test_data/per-character/Fighter-wizard-muticlass.json` (per SD-22's test-fixture pattern).

### 2.3 Out-of-scope

- APG/ACG-class multiclass (per Epic 4 deferral).
- Prestige classes (none in scope; SD-24 multiclass is "classic multiclass" not "multiclass + PRC").
- Class-feature combinations (e.g. fighter's Weapon Training interacting with wizard's spell school). Deferred to follow-on bundle.

## 3. Tauri command-surface extension design (Epic 7)

### 3.1 The gap

Per duracon 2026-07-18 18:20:41 + 2026-07-20 09:24:59:
- `create_character` / `list_saved_characters` / `load_saved_character` only compose-and-save once.
- `SavedCharacterStore::save` always writes from scratch with both current write paths (`create_character`, `seed_default_character_if_needed`) hardcoding `revision_id: "{id}.rev.1"`.
- No command exists to mutate / re-save an existing character.

### 3.2 The repair surface

Three new commands land in `apps/desktop/src-tauri/src/characterHub/`:

```
appendToCharacter   { characterId, itemsToAppend: Vec<ItemRef> }
                    -> { success: bool, character: CharacterSnapshot, error?: string }

recomputeCharacter  { characterId }
                    -> { success: bool, character: CharacterSnapshot, error?: string }

reSaveCharacter     { characterId, character: CharacterSnapshot }
                    -> { success: bool, revisionId: string, error?: string }
```

The `compose_character_input` function (at `apps/desktop/src/characterHub/characterHubRuntime.ts:compose_character_input`) hardcodes the loadout to defaults. SD-24 Epic 7 fixes this: the function reads from the saved character's equipment list, not hardcoded defaults.

### 3.3 Failure-mode handling

- `appendToCharacter` returns `{ success: false, error: "equipment_not_found" }` if `itemsToAppend` references equipment not in the corpus. No silent failure.
- `recomputeCharacter` returns `{ success: false, error: "character_not_found" }` if `characterId` is invalid.
- `reSaveCharacter` returns `{ success: false, error: "revision_conflict" }` if the local revision_id doesn't match the canonical revision_id. Resolution path: re-load + re-apply edits.

### 3.4 Stubs Registry

The "browser preview fallback" entry in the Stubs Registry (`apps/desktop/src/characterHub/characterHubRuntime.ts:17-18`) persists until operator-granted removal. SD-24 Epic 7 may incidentally clean it up if the underlying preview-fallback code path is no longer needed; otherwise deferred.

## 4. Equipment-corpus delivery design (Epic 6)

### 4.1 Coverage matrix (criterion 6.1)

The first cycle of Epic 6 produces `./artifacts/epic_6/equipment-coverage-matrix.md` with:

```
| Item | Has cost | Has weight | Has full description | Audit status |
```

The audit reads from `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/equipment.lst` (plus per-book equipment LST files) and from `src/rules_core/rules_tables/equipment/*.rs`. Per-row discrepancies become Epic 6's remediation backlog.

### 4.2 Content-completion log (criteria 6.2–6.5)

Per remediation cycle, append to `./artifacts/epic_6/content-completion-log.md`:

```
| ISO-8601 | Item | Missing field | Source line | Cycle SHA | Wired-integration audit |
```

### 4.3 Field-coverage threshold

Per operator directive 2026-07-21: **strict 100%**. Closure requires every row to have all three required fields populated (or 0 for items where cost / weight is genuinely not applicable — e.g. spells, where weight is 0; and items where there is genuinely no cost, marked with reason).

## 5. Cycle dispatch model design (decisions.md §2)

### 5.1 Deterministic seed

The 35-criterion list from `./epic-breakdown.md` is the seed. The first cycle of Epic 2 creates `progress.md` with `## TODO` listing all 35 criteria in epic+number order.

### 5.2 Dynamic queue

`## DISCOVERED` accumulates entries from cycles that find work outside the deterministic list. Each entry carries:

```
<ISO-8601> | <epic-of-origin> | <criterion-of-origin> | <priority-bump-tag> | <description> | <suggested-epic-and-criterion>
```

### 5.3 Picker priority

- Default: epic order (1 → 8), criterion-number within epic.
- Priority-bump: `## DISCOVERED` items go first if their `priority-bump-tag` is "HIGH".
- Tied: cycles with the smaller epic-failure-count first; cycles whose artifacts already exist; cycles with shorter expected runtime.

### 5.4 Discovery threshold

`## DISCOVERED` queue > 10 entries: pause dispatch, write `## Open blockers`, operator override required.

## 6. Cross-reference

- `./scope-draft.md` — bundle intent, epics, criteria, cycle dispatch model
- `./decisions.md §4` — multiclass scope (Fighter+Wizard only)
- `./decisions.md §7` — kanban is receipt-only
- `./epic-breakdown.md` — per-cycle stories
- `./content-unit-inventory.md §5` — multiclass routing
- `./content-unit-inventory.md §4` — Tauri command routing
- `../SD-23-character-mutation-and-wired-integration/` — predecessor bundle (handed-off the `compose_character_input` loadout hardcoding)
