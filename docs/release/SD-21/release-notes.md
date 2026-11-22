# Release Notes: SD-21 Campaign Manager + Drive + Multiclass Support + Identifier Cleanup + Update UI Bug + Build Versioning

## Summary

Release: `tranche/4-1` (dash release off Tranche 4). Build version: **`0.4.94`**.

This tranche ships a Campaign Manager with local-folder persistence, fixes a real update-UI bug where release notes and install eligibility never actually reflected fetched/probed state, completes Wizard's spellbook/school-powers chassis and PF1 multiclass save/BAB stacking, runs a code-side identifier-cleanup pass, and establishes the `<major>.<tranche-base>.<build>` build-version scheme.

Generated as part of Epic 4 (Closure Epilogue), criterion E4.22. Sourced from `~/workspace/SD-21-campaign-manager-and-persistence-progress.md`'s `## Status matrix` and `## Cycle log`. Every commit SHA below is a real, landed commit on `tranche/4-1`.

## User-Visible Changes

**Campaign Manager + local-folder persistence (Epic 2).**

- `CampaignSnapshot` types (id, name, ruleSetId/Label, description, members, party character IDs, four asset lists, `schema_version`) landed in a new `src/campaign/` module, mirroring `campaignModel.ts`'s `Campaign`/`CampaignAssets` shape 1:1 — `f208588`.
- Concrete `CampaignStore` struct (not a trait — no second backend exists yet) providing save/load with per-entry-failure isolation and a missing-root-tolerant `list_all`, mirroring `SavedCharacterStore`'s conventions — `739867c`.
- Local-folder persistence layout (`.config/<name>.json` + per-asset markdown files under `resources/adventure-log/maps/wiki/`), targeting a Google-Drive-for-Desktop-synced folder the user points at directly (Google OAuth/Drive-API descoped from this release per operator directive) — `da7cc5f`.
- Four new Tauri commands — `drive_list_campaigns`, `drive_load_campaign`, `drive_save_campaign`, `drive_delete_campaign` — extending the already-shipped `campaign_drive.rs`/`writeCampaignDriveArtifacts.ts` boundary; `write_campaign_drive_artifacts` became a thin adapter delegating to `CampaignStore` while keeping its request/response contract unchanged — `b46e5cc`.
- Markdown file layout verified round-trip-safe with external (Obsidian-style) edits — `dec3984`.
- Nonce-based conflict detection on load: a stale nonce with genuinely differing content moves the prior on-disk state to `conflicts/<timestamp>/` before writing — `f761af1`.
- Campaign Manager's landing-page authorization copy rewritten to describe the local-folder picker directly, removing leftover OAuth-flavored language — `a115da7`.

**Rules engine: Wizard chassis, spellbook, and school-powers completion (Epic 6 + Epic 6b).**

- `compute_pilot_base_chassis` refactored from Fighter-only to per-class dispatch; `compute_wizard_chassis` created, consuming the existing per-class `BabProgression`/`GoodSaves` tables — `2fe3b9f`.
- `compute_combat_baseline` and `compute_selected_skill_modifiers` widened from a Fighter-only gate to the full dispatch-supported class set, so Wizard's melee attack bonus, armor class, and skill-modifier cells stop rendering `Blocked` — `6ed19bd`.
- Real prepared-spellbook system grounded: spellbook contents, daily preparation, and the prepared-vs-spontaneous distinction, with a verified PF1 Wizard spells-per-day table (levels 1-3) and slot-consumption validation — resolves `class_spell.wizard.prepared_spellbook.unsupported` — `de554ab`.
- Real Arcane School selection grounded: Evocation's Intense Spells / Force Missile school powers, plus the PF1 opposed-school 2-slot preparation-cost penalty — resolves `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported` — `1c7ad89`.

**Multiclass stacking (Epic 7).**

- `compute_pilot_base_chassis` extended to length-2+ `class_levels` via a new `compute_multiclass_base_chassis`, running each class's own chassis computation in isolation and combining BAB — `63e4c2e`.
- PF1's actual best-fractional-progression save-stacking rule implemented (summed fractional save contributions per class, floored once), replacing a naive per-class-round-then-sum approximation — `02ff5ef`.
- Per-class feature integration reconciled: Fighter's own feature grants (e.g. Bravery) no longer silently vanish when another class joins the mix — `17bac41`.

## Defects Fixed

**Update-UI release-notes and eligibility bug (Epic 3).**

- Release-notes body now actually fetched and displayed: `fetchReleaseNotesBody` added to `apps/desktop/src/sd16/update/fetch.ts`, verified against `release_notes_hash`, wired into `controllerAdapter.ts`'s `runCheck` so `deps.releaseNotes` is populated instead of permanently `'unavailable'` — `8b984e8`.
- `is_install_eligible` (previously a registered-but-unwired Tauri stub) now reports real installed-state and managed-path-writability facts — `bde03ca`.
- `computeDecision`'s success path no longer short-circuits to `'unknown'`; it now calls the existing, already-correct `decideEligibility` fed by real fetched/probed data, so the eligibility card reflects genuine install state — `326e139`.
- Added outcome-coverage tests for each fetch/probe/decision path (unwritable managed path, deb-install-kind mapping, same-version/hash-as-installed, etc.) — `5dfd002`.

**Integration regressions.** Two integration-level regressions surfaced only once all Epic 1/2/3/5/6/7 lanes were combined (not by any individual lane's own local tests) and were fixed directly at the merged `HEAD`: `b207853` and `6c7a12b`, both updating `character_hub.rs`'s claim-blocking-diagnostic-shape test to the new, correct post-dispatch-widening diagnostic set.

## Operational Notes

**Code-side identifier cleanup (Epic 1).**

- Renamed `sd*_`-prefixed Tauri commands and invoke strings to descriptive names (`sd16_browser_handoff` → `handoff_defect_report_to_browser`, `load_sd13_support_state_matrix` → `load_support_state_matrix`, plus their supporting types) — `9206ad0`.
- Renamed `Sd13`/`Sd16`/`SD16`-carrying TypeScript functions, types, and constants; rewrote `SD-N`/`AV-PAY-N`/`t_<hex>` inline doc-comments as plain prose; stripped `sd16-` prefixes from `data-testid` attributes — landed across three parallel lanes and closure-gate re-verified via a full-repo grep — `22fa0a9`.
- A subsequent closure-gate scan found three small leftover leaks the parallel lanes' disjoint scopes had missed (`character_hub.rs`'s `sd19_demo_spells_selected`, `browserHandoff.ts`'s stale doc-comment, `update/transaction.rs`'s header + `t_<hex>` tokens); fixed and re-verified via the full literal grep — `25e5050`.

**Build version numbering (Epic 5).**

- Three version files (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`) bumped from `0.1.0` to **`0.4.94`** under the new `<major>.<tranche-base>.<build>` three-position scheme (`major` stays `0` until first main-publish; `tranche-base` is `4` for `tranche/4-1`; `build` is the monotonic per-build counter). `.github/workflows/publish-tester-release.yml`'s publish-time stamp updated to match the new triple so published/tester builds don't silently revert to the old scheme — `6ea6bfd`.
- Build-label format updated: `BUILD_PREFIX` changed `'codex'` → `'Codex'`, template changed from `${BUILD_PREFIX}@${buildVersion}` to `${BUILD_PREFIX} ${buildVersion}` — `5980037`.
- New `docs/release/SD-21/release-closure-checklist.md` documenting the four-step version-bump process for future closure epilogues — `d487416`.
- This closure cycle (Epic 4) confirmed the tranche position does **not** bump this time: `tranche/4-1` is not being promoted to a new tranche number, so `0.4.94` stands as the final version for this release. The tranche-bump logic (`0.4.<last_build>` → `0.5.0`) fires the next time a new tranche actually launches.

## Verification Evidence

- Capstone reproducer verified directly: a single-class Human Wizard 3, Evocation specialization, populated spellbook, and a daily preparation selection reaches `Status: Computed` with zero claim-blocking diagnostics (`tests/sd21_epic6b_full_completion_reproducer.rs`).
- Identifier-cleanup renames closure-gate re-verified via a full-repo literal grep after each lane and again after the closure-gate scan found three leftover leaks (all fixed and re-verified).
- Every commit SHA cited above is a real, landed commit on `tranche/4-1`, sourced from the bundle's own progress doc status matrix and cycle log.

## Known Issues

- The four new `drive_*` Tauri commands are not yet called from `apps/desktop/src/boundary/` — the GUI's campaign persistence flow still goes through the pre-existing write-only `write_campaign_drive_artifacts` path. Wiring the frontend to load/list/delete from disk is real follow-on scope, not a regression.
- Only Fighter and Wizard are dispatch-supported in the rules engine today. Every other single class (Barbarian, Bard, Cleric, Druid, Monk, Paladin, Ranger, Rogue, Sorcerer) still falls through to the generic `class_chassis.unsupported` diagnostic — straightforward per-class follow-on work, not attempted in this release given effort budget.
- `perform_install` remains an honest deferred stub (no HTTP client dependency in this crate for the AppImage download step); this is a documented, non-blocking scope boundary, not a regression.

## Update Eligibility

- This release directly fixes update eligibility reporting (see Defects Fixed above): `is_install_eligible` and `computeDecision` now reflect genuine fetched/probed install state instead of permanently reporting `'unknown'`/stub values.
- Install target and mechanism are otherwise unchanged from SD-16's established Linux AppImage flow (`docs/release/SD-16/release-notes.md`).
