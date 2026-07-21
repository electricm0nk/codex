---
title: SD-23 — Technical Design
status: active
owner: Todd Hintzmann
scope: bundle
canonical: true
date: 2026-07-20
last_reviewed_at: 2026-07-20
---

# SD-23 — Technical Design

Architectural surface for the SD-23 bundle. Engine/API shapes; cross-book resolution patterns; integration contracts.

## 1. Character mutation surface

### 1.1 Backend (Rust) — Tauri commands

SD-23 adds a typed operation table for mutating saved characters. The shared substrate is `mutate_saved_character(character_id, op) -> SavedCharacterEnvelope` with `op` being one of:

- `LevelUp { class_id: String }` — increments the target class's level (or adds it at level 1 if new). Triggers recompute via `compute_pilot_with_corpus`.
- `AddEquipment { item_id: String, equipped: bool }` — appends to `chosen.equipment_selections`.
- `AddSpell { spell_id: String, source_class_id: String, acquisition_mode: AcquisitionMode }` — appends to `chosen.spells_selected`.

Each operation follows the same load → mutate → recompute → re-save → return envelope pattern. The operation table is a single Rust enum; each variant is implemented as a method on a `mutate_saved_character` dispatch function.

### 1.2 Frontend (TypeScript) — boundary functions

- `levelUpCharacter(characterId, classId)` — invokes `'level_up_character'`. Returns updated envelope.
- `addEquipmentSelection(characterId, itemId, equipped)` — invokes `'add_equipment_selection'`. Returns updated envelope.
- `addSpellSelection(characterId, spellId, sourceClassId, mode)` — invokes `'add_spell_selection'`. Returns updated envelope.

The boundary functions live at `apps/desktop/src/boundary/` and are imported by the picker / Level Up dialog.

### 1.3 State refresh contract

After any successful mutation, the calling UI MUST refresh the character sheet's `detail` prop. `CharacterHubPage` holds the loaded character detail via `useState`/`loadSavedCharacterDetail`. On mutation success, the page re-invokes the loader (or uses the command's returned envelope directly) so the next render reflects the new state. A successful mutation that does NOT trigger a refresh is doctrine-noncompliant (per `wired-integration-discipline/SKILL.md` §"Fully wired checklist" item 4).

## 2. Campaign Manager simplification

### 2.1 OAuth + Drive API: dropped

Per operator directive 2026-07-20, the Campaign Manager does not authenticate to Google and does not call Drive REST endpoints. The "Drive folder" is a local folder the user configures via OS folder picker (Tauri command). If the user points that folder at a Google Drive desktop sync client, the user's machine (not this app) handles the sync.

### 2.2 Local folder contract

- `settings/googleDrive.ts` exposes `driveFolderPath: string` (already a localStorage key).
- The folder picker is a real OS folder-picker via Tauri. Refuses on missing folder; surfaces the error in the UI.
- `writeCampaignLocalFolderArtifacts` writes the campaign record and assets to the configured local folder.

### 2.3 Member invites: deleted

`CampaignMember.invited` field is removed from the data model. `createCampaign` no longer hardcodes `invited: true`. The Drive mirror writes the campaign record without `invited`.

If member invites are added later, they will be a different feature with a different shape. The current data model has no `invited` field; the no-stub doctrine forbids a "Would invite: ..." return string.

## 3. Storage tier minimal fix (Option A)

Per operator directive 2026-07-20 ("option a, confirmed"), SD-23 ships `delete_character` and `import_character` Tauri commands on the existing file-based `SavedCharacterStore`. No database. No migration.

- `delete_character(character_id)` — removes the character's `<app_data_dir>/characters/<id>/` directory. Returns `{ ok: boolean, error?: string }`.
- `import_character(file_path)` — reads the file, validates against `CharacterInput` schema, saves via `SavedCharacterStore::save` with a fresh id. Returns the new envelope.

The latent referential-integrity risk between file-store characters and localStorage campaigns is captured in `risks-and-open-questions.md` §R1 and tracked for a future bundle.

## 4. Wired Integration doctrine

The Wired Integration doctrine (`../../governance/no-stub-mvp-doctrine.md`) applies to every code-bearing cycle in SD-23:

- No empty event handlers on user-facing affordances.
- No "would have done" return strings.
- No `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack`/`temporary` in identifiers, comments, or returned strings of shipping code.
- No fixture-only data in production paths.
- No `success: true` returns from operations that did not actually do the work.

The per-cycle four-check audit (`wired-integration-discipline/SKILL.md` §"Per-cycle audit") runs before marking `complete`. Audit output captured in the kanban card comments stream per `kanban-claude-code-execution-receipt/SKILL.md`.

The Stubs Registry (`../../governance/wired-integration-stubs-registry.md`) is the doctrine-of-record for any operator-granted stub exception. Entry #0001 is the browser-preview fallback in `apps/desktop/src/characterHub/characterHubRuntime.ts:17-18`.

## 5. Epic Closure pipeline (5 sub-steps)

Per operator directive 2026-07-20 ("docs update → update graphify → PR open → fix any merge conflicts → stop"), the Epic Closure is a 5-step sequential gate. Each sub-step fires regardless of diff content; empty diffs still write a receipt.

1. **All acceptance criteria done?** If not, self-heal.
2. **Architecture docs updated?** If not, run `architecture-truth-up`.
3. **Graphify run?** If not, run `graphify-update`.
4. **PR open?** If not, open it.
5. **Merge conflicts resolved?** If any, fix via `merge-conflict-resolution`.
6. **Stop the loop.**

Skills: `architecture-truth-up` (sub-step 2), `graphify-update` (sub-step 3), `merge-conflict-resolution` (sub-step 5). Receipts to `docs/release/SD-23-.../receipts.md` (append-only YAML).

## 6. Identifier discipline

Per `../../governance/identifier-discipline.md`:

- Functions/methods/constants/properties/Tauri commands: PascalCase.
- Variables: lowercase camelCase.
- No `sd23_*`, `SD23_*`, `Sd23*` in source.
- No `t_<hex>` kanban tokens in source.
- No `SD-23-Ex...` audit-IDs in source.
- No `~/workspace/...` references in source (the release-folder copy is canonical post-promotion).

## 7. Cross-references

- `scope-draft.md` — bundle scope and operator rulings
- `epic-breakdown.md` — 7 epics / 33 acceptance criteria / per-cycle story
- `decisions.md` — decision log
- `risks-and-open-questions.md` — latent risks and deferred questions
- `acceptance-and-verification.md` — closure-gate list
- `../../governance/no-stub-mvp-doctrine.md` — Wired Integration doctrine
- `../../governance/identifier-discipline.md` — Identifier discipline
- `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` — per-cycle audit
- `~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/SKILL.md` — Epic Closure sub-step 2
- `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/SKILL.md` — Epic Closure sub-step 3
- `~/.hermes/profiles/god-emporer/skills/devops/merge-conflict-resolution/SKILL.md` — Epic Closure sub-step 5
