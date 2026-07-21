# SD-23 Epic Breakdown — Character Mutation and Wired Integration

7 epics / 33 acceptance criteria / 16 closure gates. Per-criterion test contracts and per-cycle stories. Identifier Cleanup fires first (doctrine). Epic Closure fires last; its sub-steps are extensible so future obligations (architecture truth-up, version increment, worktree cleanup, PR creation) can be added without renumbering.

---

## Epic 1 — Code-Side Identifier Cleanup

**Doctrinal anchor:** `../../governance/identifier-discipline.md`. Source-code identifiers describe what the artifact does, not which release or spec domain it came from.

**Depends on:** None. Fires FIRST.

**Audit exclusions:** None — Epic 1 IS the cleanup.

### Criterion 1 — Identifier-discipline audit runs on the cycle's diff

- **Acceptance:** A grep over the cycle's diff against develop returns zero hits for `sd23_|SD23_|Sd23|sd23-` test-IDs, `t_<hex>` kanban tokens, `SD-23-Ex...` audit-IDs in non-test source files under `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`.
- **Test contract:** `grep -rnE 'sd23_|SD23_|Sd23|sd23-' apps/desktop/src apps/desktop/src-tauri/src 2>/dev/null | grep -v __tests__ | grep -v '.test.' || echo OK_NO_SD23_TAGS`. Output captured in the kanban card comments stream.
- **Files touched:** None — read-only audit.
- **Audit-exclusion:** None.

### Criterion 2 — Per-cycle rename cycle completes

- **Acceptance:** Any SD-23-introduced identifier that violates identifier-discipline is renamed before the cycle ends. Renames follow the PascalCase convention.
- **Test contract:** Cycle's commit history shows rename commit; subsequent commit history shows the renamed identifier in use; test fixtures updated in lockstep.
- **Files touched:** Variable based on findings — typical renames target TypeScript function/constant names that bundle-tag.
- **Audit-exclusion:** None.

### Criterion 3 — Identifier-discipline skill loaded into cycle context

- **Acceptance:** The cycle's documentation / decision record shows the `identifier-discipline` skill was loaded.
- **Test contract:** Receipt comment references the skill by name.
- **Files touched:** None.
- **Audit-exclusion:** None.

### Criterion 4 — Epic 1 closure-gate: zero identifier-discipline violations on develop after Epic 1

- **Acceptance:** `git diff origin/develop...HEAD` returns zero identifier-discipline violations.
- **Test contract:** Re-run Criterion 1's grep against the post-Epic-1 branch.
- **Files touched:** None.
- **Audit-exclusion:** None.

---

## Epic 2 — Operator Pre-Launch

**Doctrinal anchor:** Repo `AGENTS.md` §"Required Minimum Handoff" — write authority, target repo, allowed scope, verification commands must be unambiguous before implementation.

**Depends on:** Epic 1.

**Audit exclusions:** None.

### Criterion 5 — Pre-launch checklist passes

- **Acceptance:** All 7 pre-launch checklist items in `loop-instruction.md` are verified true.
- **Test contract:** Cycle logs each checklist item's verification command and output in the kanban card comments stream.
- **Files touched:** None — verification only.
- **Audit-exclusion:** None.

### Criterion 6 — Build counter captured in progress.md

- **Acceptance:** `progress.md` §"Build counter inheritance" is filled with the develop HEAD's `Cargo.toml` workspace version.
- **Test contract:** Cycle reads `Cargo.toml` workspace version, captures build counter, writes to `progress.md`.
- **Files touched:** `programs/codex/requirements/SD-23-character-mutation-and-wired-integration/progress.md`.
- **Audit-exclusion:** None.

---

## Epic 3 — Wired Integration Cleanup

**Doctrinal anchor:** `../../governance/no-stub-mvp-doctrine.md` + `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md`. No stubs in shipping code. Stubs are operator-granted exceptions registered at `../../governance/wired-integration-stubs-registry.md`.

**Depends on:** Epic 1.

**Audit exclusions:** None.

### Criterion 7 — Stubs Registry exists

- **Acceptance:** `../../governance/wired-integration-stubs-registry.md` exists with at least one operator-granted entry.
- **Test contract:** `cat ../../governance/wired-integration-stubs-registry.md` shows entry #0001 (browser-preview fallback).
- **Files touched:** `../../governance/wired-integration-stubs-registry.md`.
- **Audit-exclusion:** None.

### Criterion 8 — Four-check audit runs clean on a known-clean slice

- **Acceptance:** Run the four-check audit on a known-clean slice (the post-Epic-1 branch); all four checks return `OK_NO_*`.
- **Test contract:** Cycle runs the audit, captures output in kanban card comments stream.
- **Files touched:** None.
- **Audit-exclusion:** None.

### Criterion 9 — Stubs Registry is referenced by `wired-integration-discipline` skill

- **Acceptance:** The skill's cross-reference section lists the Stubs Registry.
- **Test contract:** `grep -n "wired-integration-stubs-registry" ~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` returns a hit.
- **Files touched:** Skill file.
- **Audit-exclusion:** None.

### Criterion 10 — Accidental stubs surfaced by the audit are remediated

- **Acceptance:** Any stub surfaced by the four-check audit (not in the Stubs Registry) is remediated before the cycle ends.
- **Test contract:** Cycle logs the stub, the remediation cycle, and the post-remediation audit pass.
- **Files touched:** Variable.
- **Audit-exclusion:** During the cleanup cycle itself, audit may temporarily surface the stub being replaced — this is the explicit "Whitelist for in-flight cleanup" carve-out from the skill. The carve-out is recorded in `decisions.md` §11 if invoked.

### Criterion 11 — Epic 3 closure-gate: zero unremediated stubs in the diff

- **Acceptance:** Every stub-pattern hit in the diff is either remediated or registered in the Stubs Registry with operator approval.
- **Test contract:** Re-run the four-check audit; cross-reference hits against the Stubs Registry; zero unmatched hits.
- **Files touched:** None.
- **Audit-exclusion:** None.

---

## Epic 4 — Campaign Manager Simplification

**Doctrinal anchor:** Operator directive 2026-07-20 ("we are going to use a shared drive and leave it at that for our initial release"). Drop OAuth, drop Drive API, keep local-folder contract.

**Depends on:** Epic 3.

**Audit exclusions:** Epic 4 is the in-flight cleanup of the existing campaign-stub surface — the audit's "Would …" check may fire on `createCampaign`'s `driveActionSummary` return string during the rewrite. This is the explicit "Whitelist for in-flight cleanup" carve-out from the skill.

### Criterion 12 — `createCampaign` no longer returns `driveActionSummary`

- **Acceptance:** `createCampaign` returns `{ campaign: Campaign, syncResult: { ok: boolean, campaignFolderPath?: string, error?: string } }`. The `driveActionSummary` string is removed.
- **Test contract:** TypeScript test asserts the new return shape; the test fixture is updated.
- **Files touched:** `apps/desktop/src/campaign/campaignModel.ts`, `apps/desktop/src/campaign/CreateCampaignScreen.tsx`, `apps/desktop/src/campaign/campaignModel.test.ts` (if exists) or new test file.
- **Audit-exclusion:** Yes — `driveActionSummary` removal is the in-flight cleanup.

### Criterion 13 — `CampaignMember.invited` field deleted

- **Acceptance:** The `invited` field is removed from `CampaignMember`. `createCampaign` no longer maps member emails to `{email, invited: true}`.
- **Test contract:** TypeScript test asserts the new shape; Create Campaign UI input is unchanged (still collects emails); persisted shape doesn't carry `invited`.
- **Files touched:** `apps/desktop/src/campaign/campaignModel.ts`, test fixtures.
- **Audit-exclusion:** None.

### Criterion 14 — `syncCampaignDriveArtifacts` renamed to `writeCampaignLocalFolderArtifacts`

- **Acceptance:** Function renamed; doc comment rewritten to remove the "Drive mirror" framing.
- **Test contract:** TypeScript test references the new name; no source-code references to `syncCampaignDriveArtifacts` remain outside test fixtures.
- **Files touched:** `apps/desktop/src/campaign/campaignModel.ts`, `apps/desktop/src/boundary/writeCampaignDriveArtifacts.ts` (rename to match), all call sites, test fixtures.
- **Audit-exclusion:** None.

### Criterion 15 — Epic 4 closure-gate: campaign CRUD cycle passes four-check audit

- **Acceptance:** Running the four-check audit on the post-Epic-4 branch returns `OK_NO_*` for all four checks.
- **Test contract:** Cycle runs the audit; output captured.
- **Files touched:** None.
- **Audit-exclusion:** None.

---

## Epic 5 — Character Mutation Surface

**Doctrinal anchor:** Operator directive 2026-07-20 ("every cycle passes the four-check audit before marking complete"). New functionality ships fully wired from cycle 1.

**Depends on:** Epic 3.

**Audit exclusions:** None — Epic 5 is net-new functionality, no existing stub to replace.

### Criterion 16 — `mutate_saved_character` operation table documented

- **Acceptance:** A typed operation table in the Rust backend enumerates: `level_up_character`, `add_equipment_selection`, `add_spell_selection`, with shared `load → mutate → recompute → re-save → return envelope` semantics.
- **Test contract:** Rust unit test asserts the operation table's dispatch shape.
- **Files touched:** `apps/desktop/src-tauri/src/character_hub.rs` (or new module).
- **Audit-exclusion:** None.

### Criterion 17 — `level_up_character` Tauri command

- **Acceptance:** Backend command loads envelope via `SavedCharacterStore::load`, increments/adds the class level, recomputes via `compute_pilot_with_corpus`, re-saves via `SavedCharacterStore::save`, returns the updated envelope.
- **Test contract:** Rust integration test asserts the operation round-trips correctly with a real SavedCharacterStore fixture.
- **Files touched:** `apps/desktop/src-tauri/src/character_hub.rs`, `src/saved_character/local_store.rs`, test files.
- **Audit-exclusion:** None.

### Criterion 18 — `add_equipment_selection` and `add_spell_selection` Tauri commands

- **Acceptance:** Backend commands append to `chosen.equipment_selections` or `chosen.spells_selected`, recompute, re-save, return the updated envelope.
- **Test contract:** Rust integration tests assert append + recompute + re-save round-trips.
- **Files touched:** Same as Criterion 17.
- **Audit-exclusion:** None.

### Criterion 19 — `list_spells(filter)` and `list_equipment(filter)` Tauri commands

- **Acceptance:** Backend commands return filtered lists from the corpus (CRB for now; APG/ACG sample-only per SD-22 scope).
- **Test contract:** Rust integration test asserts filter behavior on a known corpus subset.
- **Files touched:** `apps/desktop/src-tauri/src/character_hub.rs`, `apps/desktop/src-tauri/src/sd19_equipment_catalog.rs`, `src/rules_core/spell_resolver.rs`, `src/rules_core/equipment_resolver.rs`.
- **Audit-exclusion:** None.

### Criterion 20 — Picker/modal UI component

- **Acceptance:** A new picker/modal component (search input + filtered list + select) is created. Wired to `Add Weapon` button on WeaponsTab, `Add Armor` button on Gear tab, `Add Spell` button on Spells tab.
- **Test contract:** TypeScript test asserts the picker renders, filters, and selects; UI tests assert the buttons call the picker.
- **Files touched:** New component file `apps/desktop/src/characterHub/ItemPickerModal.tsx`, `CharacterSheet.tsx` (button wiring), test file.
- **Audit-exclusion:** None — must not contain empty handlers.

### Criterion 21 — Character sheet refresh after successful add / level-up

- **Acceptance:** After a successful `add_equipment_selection` / `add_spell_selection` / `level_up_character` call, the character sheet's `detail` prop is refreshed. The Level box, class panel, and Progression rail all reflect the new state without requiring a close-and-reopen.
- **Test contract:** TypeScript integration test asserts the refresh path. UI test asserts the visible state changes after Accept.
- **Files touched:** `apps/desktop/src/characterHub/CharacterHubPage.tsx`, `apps/desktop/src/characterHub/CharacterSheet.tsx`, `apps/desktop/src/boundary/loadSavedCharacterDetail.ts` (or new boundary function).
- **Audit-exclusion:** None.

---

## Epic 6 — Storage Tier Minimal Fix (Delete / Import)

**Doctrinal anchor:** Operator directive 2026-07-20 ("option a, confirmed"). Close the Load Character screen's no-op buttons.

**Depends on:** Epic 3.

**Audit exclusions:** None.

### Criterion 22 — `delete_character` Tauri command

- **Acceptance:** Backend command removes the character's `<app_data_dir>/characters/<id>/` directory. Returns `{ ok: boolean, error?: string }`.
- **Test contract:** Rust integration test asserts directory removal on a real SavedCharacterStore fixture.
- **Files touched:** `apps/desktop/src-tauri/src/character_hub.rs`, `src/saved_character/local_store.rs` (add `delete` method), test files.
- **Audit-exclusion:** None.

### Criterion 23 — `import_character` Tauri command

- **Acceptance:** Backend command accepts a file path + character JSON; validates the JSON against `CharacterInput` schema; saves via `SavedCharacterStore::save` with a fresh id. Returns the new envelope.
- **Test contract:** Rust integration test asserts import round-trip with a known fixture.
- **Files touched:** `apps/desktop/src-tauri/src/character_hub.rs`, `src/saved_character/local_store.rs`, test files.
- **Audit-exclusion:** None.

### Criterion 24 — Load Character screen Delete/Import buttons wired

- **Acceptance:** `LoadCharacterScreen.tsx:268, 279` button `onClick` handlers call the new Tauri commands via a new boundary function. The `setStatus('Delete/Import will be available once database storage lands.')` strings are removed.
- **Test contract:** TypeScript test asserts the buttons invoke the boundary function with correct arguments; UI test asserts the visible outcome (character removed from list, imported character added to list).
- **Files touched:** `apps/desktop/src/characterHub/LoadCharacterScreen.tsx`, new boundary function `apps/desktop/src/boundary/loadDeleteCharacter.ts` / `loadImportCharacter.ts`, test files.
- **Audit-exclusion:** None — empty handlers are exactly what the doctrine forbids.

---

## Epic 7 — Closure Epilogue (Epic Closure pattern)

**Doctrinal anchor:** Standard part-of-handoff for every SD-N bundle. The Epic Closure is a single epic whose sub-steps are extensible — new obligations land inside Epic Closure as new sub-steps rather than as new numbered epics.

**Depends on:** All other epics.

**Sub-steps (sequential; each must complete before the next fires):**

1. **Acceptance-criteria done check.** All 26 code-bearing criteria (Epics 1-6) marked `complete` in `progress.md`. If any are missing or have blockers, the loop self-heals and runs more cycles until done.
2. **Architecture truth-up.** Run `architecture_truth_up.py` per the architecture-truth-up skill. The script edits touched docs in place, removes obsolete statements, refreshes `Last verified:` headers, runs the verification one-liners, and appends a YAML receipt to `docs/release/SD-23-character-mutation-and-wired-integration/receipts.md`. Empty diffs still write a receipt — the receipt IS the audit trail.
3. **Graphify update.** Run `update_graphify.py` per the graphify-update skill. The script invokes graphify against the codex repo, captures stdout/stderr/exit-code, and appends a `graphify:update` receipt to `receipts.md`. **Graphify non-zero exit does NOT refuse the closure pipeline** — the failure receipt is the audit trail; operator decides retry-vs-proceed.
4. **PR open.** Open the `tranche/5-1 → develop` promotion PR. PR creation is a bash-level command in the loop-instruction, not a separate skill.
5. **Merge conflicts resolved.** Run `resolve_merge_conflicts.py` per the merge-conflict-resolution skill (mode `pre-flight` or `post-pr`). On conflicts, the script emits a `merge_conflict:*` receipt and exits non-zero — the loop self-heals, operator resolves manually, loop re-runs until `outcome: clean`.
6. **`tranche/5-1 → develop` promotion PR merge.** CI passes; merge is clean.
7. **Build counter advances on promotion.** Per Decision §3, build counter advances `0.5.<last_build> → 0.6.0` on promotion. Tranche base advances 5 → 6 (the next working tranche starts at 6).
8. **Decisions + risks final review.** Final-entry sections in `decisions.md` and `risks-and-open-questions.md` record the bundle's closure state.
9. **Progress log complete.** Every cycle's post-mortem entry exists with commit SHA + kanban card id + audit result.
10. **Bundle marked closed on the board.** All SD-23 kanban cards in `complete` state on `codex-tranche-5`.

**Audit exclusions:** None.

**Future-proofing:** Sub-steps 11, 12, ... are reserved for new closure obligations as they emerge (e.g. release-notes generation, worktree cleanup, security audit). Adding a sub-step does NOT renumber prior epics — Epic Closure is a bucket, not a number.

### Criterion 25 — Pre-promotion verification

- **Acceptance:** All 16 closure gates from `acceptance-and-verification.md` pass before the promotion PR opens.
- **Test contract:** Cycle logs each gate's verification command and output.
- **Files touched:** None.
- **Audit-exclusion:** None.

### Criterion 26 — Architecture truth-up cycle (Epic Closure sub-step 2)

- **Acceptance:** `architecture_truth_up.py` runs against `develop` with `--integration-target develop --receipts-md docs/release/SD-23-character-mutation-and-wired-integration/receipts.md --bundle SD-23`. The script appends a YAML receipt to `receipts.md` with `row_or_kind: architecture:truth_up`. Both verification one-liners (cited-path existence + relative-link check) pass. Any obsolete statements in touched docs are REMOVED, not annotated as deprecated.
- **Test contract:** Cycle runs the script; the receipt block is in `receipts.md`; the verification one-liners emit zero hits.
- **Files touched:** `docs/release/SD-23-character-mutation-and-wired-integration/receipts.md` (append); any `docs/architecture/*.md` doc whose `Source dirs` overlaps the diff.
- **Audit-exclusion:** None.

### Criterion 27 — Graphify update cycle (Epic Closure sub-step 3)

- **Acceptance:** `update_graphify.py` runs against the codex repo with `--integration-target develop --receipts-md docs/release/SD-23-character-mutation-and-wired-integration/receipts.md --bundle SD-23`. The script appends a YAML receipt to `receipts.md` with `row_or_kind: graphify:update`. **Success OR failure both write a receipt**; graphify non-zero exit does NOT refuse the closure pipeline. The receipt records `outcome: success | failed`, `graphify_exit_code`, `wall_clock_seconds`, and `log_path` (under `graphify-out/`).
- **Test contract:** Cycle runs the script; the receipt block is in `receipts.md`. Operator reviews the receipt's `outcome` and `receipt_note` fields; success proceeds, failure triggers an operator decision (retry, proceed, or roll back).
- **Files touched:** `docs/release/SD-23-character-mutation-and-wired-integration/receipts.md` (append); `graphify-out/.truth-up-run-<cycle_id>.log` (graphify's captured stdout/stderr).
- **Audit-exclusion:** None.

### Criterion 28 — Merge conflict resolution cycle (Epic Closure sub-step 5)

- **Acceptance:** `resolve_merge_conflicts.py` runs in `--mode pre-flight` or `--mode post-pr` per the merge-conflict-resolution skill. The script emits a `merge_conflict:<mode>` receipt to `receipts.md` with `outcome: clean | conflicts_found`. On `clean`, the loop continues to sub-step 6 (PR merge). On `conflicts_found`, the script exits non-zero, the loop self-heals, the operator resolves manually, and the loop re-runs until clean.
- **Test contract:** Cycle runs the script; the receipt is in `receipts.md`; `outcome: clean` is the gate. The conflict-files list (when present) names exactly what the operator needs to fix.
- **Files touched:** `docs/release/SD-23-character-mutation-and-wired-integration/receipts.md` (append); any conflict files the operator resolves.
- **Audit-exclusion:** None.

### Criterion 29 — `tranche/5-1 → develop` promotion PR

- **Acceptance:** Promotion PR opens against develop with all SD-23 cycles' commits; CI passes; merge is clean.
- **Test contract:** GitHub PR URL captured in `progress.md` and the closure receipt.
- **Files touched:** PR metadata.
- **Audit-exclusion:** None.

### Criterion 30 — Build counter advances on promotion

- **Acceptance:** Per Decision §3, build counter advances `0.5.<last_build> → 0.6.0` on promotion. Tranche base advances 5 → 6 (the next working tranche starts at 6).
- **Test contract:** `Cargo.toml` workspace version updated post-merge; captured in `decisions.md` §3 final entry.
- **Files touched:** `Cargo.toml` (workspace version).
- **Audit-exclusion:** None.

### Criterion 31 — `decisions.md` and `risks-and-open-questions.md` final review

- **Acceptance:** Both files have a final-entry section recording the bundle's closure state.
- **Test contract:** Cycle reads both files and asserts the final entries exist.
- **Files touched:** `decisions.md`, `risks-and-open-questions.md`.
- **Audit-exclusion:** None.

### Criterion 32 — `progress.md` cycle log complete

- **Acceptance:** Every cycle's post-mortem entry exists with commit SHA + kanban card id + audit result.
- **Test contract:** Cycle counts entries; expected count covers all code-bearing criteria (Epics 1-6) plus all closure-pipeline cycles (Epic 7).
- **Files touched:** `progress.md`.
- **Audit-exclusion:** None.

### Criterion 33 — Bundle marked closed on the board

- **Acceptance:** All SD-23 kanban cards are in `complete` state on `codex-tranche-5`. The bundle's Epic Closure card has the closure receipt.
- **Test contract:** `hermes kanban list-cards --board codex-tranche-5 --status complete --criteria-only` returns the expected card count.
- **Files touched:** None.
- **Audit-exclusion:** None.

(Sub-step numbering is local to the Epic Closure; criterion numbering remains 1-33 across the bundle. Future closure obligations add criteria 34, 35, ... in Epic Closure without renumbering the prior bundle's epics.)
