---
title: SD-23 — Technical Requirements
status: active
owner: Todd Hintzmann
scope: bundle
canonical: true
date: 2026-07-20
last_reviewed_at: 2026-07-20
---

# SD-23 — Technical Requirements

Pre-loop prerequisites and normative requirements for the SD-23 bundle.

## 1. Pre-loop environment

- **Tranche:** `tranche/5-1` (dash release from `tranche/5`)
- **Board:** `codex-tranche-5` (reused after SD-22 closure PR lands)
- **Working tree:** clean at bundle launch (per `loop-instruction.md` pre-launch checklist)
- **Doctrines loaded:** `identifier-discipline` and `wired-integration-discipline` skills active
- **Build counter:** captured at SD-22 closure PR merge (see `decisions.md` §3)

## 2. Normative requirements

### 2.1 Backend (Rust)

- **R1.** Tauri commands added in this bundle MUST load, mutate, recompute via `compute_pilot_with_corpus`, and re-save via `SavedCharacterStore::save`. Each command returns the updated envelope so the frontend can refresh without a second round-trip.
- **R2.** Tauri command names follow PascalCase per the identifier-discipline doctrine; invoke-string is the snake_case form (`level_up_character` → invoke `'level_up_character'`).
- **R3.** SavedCharacterStore gains `delete` and `import` methods. `delete` removes the character's `<app_data_dir>/characters/<id>/` directory. `import` accepts a file path + character JSON; validates the JSON against `CharacterInput` schema; saves via `SavedCharacterStore::save` with a fresh id.
- **R4.** No stub returns `success: true` from operations that did not actually do the work. Per `wired-integration-discipline/SKILL.md` §"Forbidden patterns in shipping code."

### 2.2 Frontend (TypeScript / TSX)

- **F1.** Picker/modal component (`ItemPickerModal.tsx`) renders a search input + filtered list + select. Wired to `Add Weapon` / `Add Armor` / `Add Spell` affordances on the character sheet. Real calls to real Tauri commands on selection. Real refresh of the character sheet's `detail` prop on success.
- **F2.** Level Up dialog (`LevelUpDialog.tsx`) calls `level_up_character` on Accept and refreshes the sheet. The empty-handler stub that pre-existed on `tranche/5` was a designed exception per Stubs Registry §0001 — and the user-confirmed current bundle removes that exception at this cycle (no LevelUpDialog.tsx on `tranche/5`; the bundle lands the dialog fully wired from cycle 1).
- **F3.** Load Character screen buttons (Delete/Import) wire to `delete_character` and `import_character` Tauri commands via new boundary functions. The `setStatus('Delete/Import will be available once database storage lands.')` strings are removed.

### 2.3 Data model

- **D1.** `CampaignMember.invited` field removed from the data model. `createCampaign` no longer hardcodes `invited: true`. The Drive mirror writes the campaign record without `invited`.
- **D2.** `createCampaign` returns `{ campaign: Campaign, syncResult: { ok, campaignFolderPath?, error? } }` — the `driveActionSummary` string is removed.
- **D3.** `syncCampaignDriveArtifacts` renamed to `writeCampaignLocalFolderArtifacts` per the operator's 2026-07-20 OAuth-drop directive. Doc comment rewritten to remove the "Drive mirror" framing.
- **D4.** Mutation surface operates against `CharacterInput.chosen.ChosenCharacterState.spells_selected` and `equipment_selections`. Class levels live in the classSummary string format the frontend already parses.

### 2.4 Test surface

- **T1.** TDD mandatory per repo `AGENTS.md` §"Non-Negotiable Rules" — write failing test before production code, confirm test fails for the intended reason, implement smallest change to pass, run relevant tests, refactor only after green.
- **T2.** Every code-bearing cycle runs the four-check audit (per `wired-integration-discipline/SKILL.md` §"Per-cycle audit") before marking `complete`. Audit output captured in the cycle receipt.
- **T3.** Tier-2 tests (Rust unit/integration) pass under `cargo test --workspace`. Tier-3 tests (TypeScript/TSX) pass under the bundle's runner (confirm at SD-23 launch).

## 3. Closure-pipeline requirements (per `wired-integration-discipline` doctrine)

- **C1.** Epic Closure pipeline fires sub-steps 1-6 in sequence: criteria done → architecture-truth-up → graphify-update → PR open → merge-conflict-resolution → stop. Empty diffs at any sub-step still write a receipt to `receipts.md` — the receipt IS the audit trail.
- **C2.** Architecture-truth-up runs `architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-23-.../receipts.md --bundle SD-23`. Empty diff still writes a receipt.
- **C3.** Graphify-update runs `update_graphify.py` with the same flags. Graphify non-zero exit does NOT refuse the closure pipeline — the failure receipt is the audit trail; operator decides retry-vs-proceed.
- **C4.** Merge-conflict-resolution runs `resolve_merge_conflicts.py` in `--mode pre-flight` or `--mode post-pr`. On conflicts, the script exits non-zero, the loop self-heals, the operator resolves manually, the loop re-runs until `outcome: clean`.

## 4. Non-goals (deferred to future bundles)

- Database / storage-tiers convergence (deferred to a future bundle; see `programs/codex/research/storage-tiers-convergence-2026-07-20.md`).
- Stat-field promotion for added equipment/spells (corpus → generated-table refactor; deferred).
- Auto-granting spells/feats at level-up (level-up persistence only takes the level; specific known-spell/bonus-feat selection is a future UI/backend surface).
