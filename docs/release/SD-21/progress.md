---
title: SD-21 — Campaign Manager + Drive + Multiclass Support + Identifier Cleanup + Update UI Bug + Build Versioning + Closure Epilogue — Progress
mirrors: /home/ubuntu/workspace/SD-21-campaign-manager-and-persistence-scope-draft.md
created: 2026-07-18
snapshot_as_of: 8494dd1
---

# SD-21 — Progress

## SD-21 STATUS: CLOSURE PR OPENED

**PR #322:** https://github.com/electricm0nk/codex/pull/322 (base=`develop`, head=`tranche/4-1`, state=OPEN, mergeable)

All 30 criteria complete. Closure scan (2026-07-19, Opus-tier gate verification) confirmed READY FOR CLOSURE — all ten load-bearing gates (1,2,4,5,6,8,9,10,11,12) independently re-verified. Closure execution (2026-07-19, 06:00 UTC) delivered: final scan + PR + release notes + version checks + full test suite green (root crate 397/397 lines ok, 0 failed; src-tauri 85/85 green; TS 45/45; clippy clean).

---

SD-21's own progress doc. Separate from SD-18/SD-19/SD-20's progress files. Loop's
claim protocol and per-cycle history live here under `## SD-21 cycles`.

**Criterion identity note:** the source `epic-breakdown.md` numbers criteria 1-30
globally, but a renumbering artifact left Epic 5 and Epic 6 **both** using numbers
25-27 (a duplicate; not fixed here since the source docs are the canonical/approved
handoff — flagged in the loop-instruction's 2026-07-18 verification-findings section).
This progress doc disambiguates using `E<epic>.<criterion>` (e.g. `E5.25`, `E6.25`)
instead of the bare number. **Epic 6b** (added 2026-07-19) is new, unnumbered follow-on
work scoped after Epic 6's own cycle proved its original acceptance reproducer
unreachable as stated — see `E6b.1`-`E6b.3` below and `epic-breakdown.md`'s new
"Epic 6b" section. SD-21 closure (gate 11) is gated on Epic 6b, not just Epic 6.

## Status matrix

| Criterion | Epic | row_or_kind | Brief description | Started | Duration | Status | Receipt SHA |
|---|---|---|---|---|---|---|---|
| E1.1 | 1 — Identifier Cleanup | identifier:rust_tauri | Rename `sd*_`-prefixed Tauri commands + invoke strings. Corrected 2026-07-18: only 2 dirty command names existed in the whole repo (the 4 `sd19_*_catalog.rs` files and `sd19_corpus.rs` already had clean command names) — both now renamed. | 2026-07-18T19:57:33Z | ~2100s total (2 cycles) | **complete** | 9206ad0 |
| E1.2 | 1 — Identifier Cleanup | identifier:ts_function_or_class | Rename `Sd13`/`Sd16`-carrying TS functions, types, and constants. Landed via 3 parallel worktree lanes (SD13, `sd16/feedback/*`, `sd16/update/*`); SD19 already fully clean. Closure-gate re-verified 2026-07-19 by the operator directly (not just self-reported by the lanes): full repo grep for `Sd16\|SD16_\|sd16-\|Sd13\|SD13_\|sd13-` returns zero hits outside (a) out-of-scope import/file paths, (b) meta-commentary in `loadSd13SupportStateMatrix.test.ts` describing the rename itself, (c) trivial `sd16-*` temp-dir scratch-naming in `update/transaction.rs` test fixtures (not an identifier/command/testid/doc-comment position). | 2026-07-18T21:30:00Z | ~5400s (3 parallel lanes) | **complete** | 22fa0a9 |
| E1.3 | 1 — Identifier Cleanup | identifier:doc_comment / identifier:data_testid | Rewrite inline `SD-N`/`AV-PAY-N`/`t_<hex>` doc-comments as plain prose; rename `sd16-*` data-testid attributes. Landed in the same 3 parallel lanes as E1.2 (identifier and doc-comment/testid work was done together per file to avoid two lanes touching the same file). Closure-gate re-verified 2026-07-19, see E1.2's note — same result. | 2026-07-18T21:30:00Z | ~5400s (3 parallel lanes) | **complete** | 22fa0a9 |
| E1.4 | 1 — Identifier Cleanup | identifier:tests_followup | Per-rename follow-up tests exercising each new name; CI green after every rename. Satisfied inline by each E1.2/E1.3 rename cycle's own RED→GREEN test additions (`loadSd13SupportStateMatrix.test.ts`, `docCommentHygiene.test.ts`, the 4 stray snake_case test-fn renames in `parseUpdateManifest.test.ts`) rather than a separate pass. | 2026-07-18T21:30:00Z | — | **complete** | 22fa0a9 |
| E2.5 | 2 — Campaign Manager | campaign:snapshot | Define `CampaignSnapshot` in a new `src/campaign/` module (sibling to `src/saved_character/`, not `src/rules_core/` — campaigns aren't rules-computation, per 2026-07-18 engine-shape research). Fields mirror `campaignModel.ts`'s `Campaign`+`CampaignAssets` 1:1 (id, name, ruleSetId/Label, description, members[{email,invited}], partyCharacterIds: Vec\<String\> keyed on existing `character_id` space, createdAt/updatedAt, 4 asset lists of {title,body}), plus a `schema_version: u16` from day one per the `SavedCharacterStore` precedent. | 2026-07-18T22:45:00Z | — | **complete** | f208588 |
| E2.6 | 2 — Campaign Manager | campaign:persistence | **Operator directive 2026-07-18: build a concrete `CampaignStore` struct, NOT a `CampaignBackend` trait** — no `*Backend` trait pattern exists anywhere in this codebase (`SavedCharacterStore` is a concrete zero-field struct with associated fns); OAuth/Drive-API is already descoped so there's no second backend to justify trait-object indirection. Mirror `SavedCharacterStore`'s conventions: `list_all` tolerant of missing root dir, per-entry failures isolated (not whole-listing failure), flat `CampaignStoreError { message: String }`. | 2026-07-18T23:00:00Z | — | **complete** | 739867c |
| E2.7 | 2 — Campaign Manager | campaign:persistence | `CampaignStore`'s local-folder impl — **OAuth/Drive-API descoped per 2026-07-18 operator directive**; targets a Drive-for-Desktop-synced local folder. Must NOT break the load-bearing boundary contract PR #320 shipped: `campaign_drive.rs`'s `write_campaign_drive_artifacts_impl` becomes a thin adapter that deserializes the already-JSON `campaign_config_json` into the new typed `CampaignSnapshot` and delegates to `CampaignStore`, keeping `WriteCampaignDriveArtifactsRequest`/`Response` unchanged. | 2026-07-18T23:15:00Z | — | **complete** | da7cc5f |
| E2.8 | 2 — Campaign Manager | campaign:persistence | Tauri commands (`drive_list_campaigns`, `drive_load_campaign`, `drive_save_campaign`, `drive_delete_campaign` — local-folder backed, no `drive_authorize`/OAuth) extending the already-landed `campaign_drive.rs` + `writeCampaignDriveArtifacts.ts` boundary. Note: `campaign_drive.rs` currently has zero dependency on the `codex` engine crate (pure Tauri-app-local logic) — this criterion grows the seam, mirroring how `character_hub.rs` wraps `codex::saved_character`. Opportunistically fix the pre-existing clippy error at `campaign_drive.rs:70` while this file is open (see Open blockers). | 2026-07-18T23:30:00Z | — | **complete** | b46e5cc |
| E2.9 | 2 — Campaign Manager | campaign:persistence | Markdown file layout per `technical-design.md` §2.2; Obsidian round-trip | 2026-07-18T23:45:00Z | — | **complete** | dec3984 |
| E2.10 | 2 — Campaign Manager | campaign:conflict_log | Conflict-detection on load (nonce mismatch → `conflicts/<timestamp>/`) | 2026-07-19T00:00:00Z | — | **complete** | f761af1 |
| E2.11 | 2 — Campaign Manager | campaign:persistence | First-run authorization surfaces in GUI's campaign-manager landing page (local-folder picker, not OAuth, per descope) | 2026-07-19T00:15:00Z | — | **complete** | a115da7 |
| E3.12 | 3 — Update UI Bug | update:release_notes_fetch | Release-notes-body fetch path — extends the already-existing `fetch.ts`, assigns `deps.releaseNotes` (confirmed still live 2026-07-18) | 2026-07-19T01:30:00Z | — | **complete** | 8b984e8 |
| E3.13 | 3 — Update UI Bug | update:installed_state_probe | Implement the already-existing, registered-but-not-wired `is_install_eligible`/`perform_install` stubs at `update/transaction.rs:659,680` (confirmed 2026-07-18; no `install_eligibility_probe.rs` file to create) | 2026-07-19T01:30:00Z | — | **complete** | bde03ca |
| E3.14 | 3 — Update UI Bug | update:computeDecision_rewire | Rewire `computeDecision`'s success-path short-circuit to call real `decideEligibility` (confirmed still live 2026-07-18) | 2026-07-19T01:30:00Z | — | **complete** | 326e139 |
| E3.15 | 3 — Update UI Bug | update:tests_followup | Per-fetch/per-probe/per-decision outcome tests | 2026-07-19T01:30:00Z | — | **complete** | 5dfd002 |
| E5.25 | 5 — Build Version Numbering | version:patch_bump | Bump 3 version files to `0.4.<build>` — anchor is `0.4.94` per 2026-07-18 verification (files currently `0.1.0`, not the doc's assumed `0.0.X`); **also update `.github/workflows/publish-tester-release.yml`'s publish-time stamp** (line 62) to the new triple per 2026-07-18 operator directive, or published builds keep showing the old scheme | 2026-07-19T00:30:00Z | — | **complete** | 6ea6bfd |
| E5.26 | 5 — Build Version Numbering | version:build_label_format | `createSd11WorkbenchStatus.ts` `BUILD_PREFIX` → `'Codex'`, template → `${BUILD_PREFIX} ${buildVersion}`; fixture updates | 2026-07-19T00:45:00Z | — | **complete** | 5980037 |
| E5.27 | 5 — Build Version Numbering | version:closure_checklist | New `docs/release/SD-21/release-closure-checklist.md` (relocated 2026-07-20 from a stray `docs/SD-21/` path) | 2026-07-19T01:00:00Z | — | **complete** | d487416 |
| E6.25 | 6 — Single-class coverage | class:compute_dispatch | Refactor `compute_pilot_base_chassis` (`pilot_compute.rs:4568`) from Fighter-only to per-class dispatch (confirmed still live 2026-07-18: SD-20 never touched this file) | 2026-07-19T01:15:00Z | ~2100s | **complete** | 2fe3b9f |
| E6.26 | 6 — Single-class coverage | class:compute_wizard_chassis | **Create** `compute_wizard_chassis` (does not exist yet, confirmed 2026-07-18 — `supported_wizard_level` already gates 1-20 but only for a spell-baseline explanation, not chassis); consume `class_tables.rs`'s `BabProgression`/`GoodSaves`; one sub-feature per cycle | 2026-07-19T01:15:00Z | ~2100s | **complete** | 2fe3b9f |
| E6.27 | 6 — Single-class coverage | class:compute_dispatch | Per-class foundation module shapes (`compute_<class>_chassis` + `explain_<class>_chassis`) | 2026-07-19T01:15:00Z | ~2100s | **complete** | 2fe3b9f |
| E6b.1 | 6b — Wizard full-completion (new 2026-07-19) | class:combat_baseline_dispatch | Extend `compute_combat_baseline` + `compute_selected_skill_modifiers` to the dispatch-supported class set via `has_supported_class_chassis` (mirrors work already done for `compute_total_saves`) | 2026-07-19T04:30:00Z | ~600s | **complete** | 6ed19bd |
| E6b.2 | 6b — Wizard full-completion (new 2026-07-19) | class:wizard_prepared_spellbook | Ground Wizard's prepared spellbook (spellbook contents, daily preparation, prepared-vs-spontaneous) — resolves `class_spell.wizard.prepared_spellbook.unsupported` | 2026-07-19T04:30:00Z | ~1200s | **complete** | de554ab |
| E6b.3 | 6b — Wizard full-completion (new 2026-07-19) | class:wizard_school_powers | Ground Wizard's Arcane School selection (school powers e.g. Evocation's Intense Spells/Force Missile, opposed-school slot-cost penalty) — resolves `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported`. Largest of the three E6b gaps; may split into per-school sub-cycles. | 2026-07-19T04:30:00Z | ~1400s | **complete** | 1c7ad89 |
| E7.28 | 7 — Multiclass stacking | class:multiclass_dispatch | Extend `compute_pilot_base_chassis` to length-2+ `class_levels` via `compute_multiclass_base_chassis` (confirmed still live 2026-07-18: no multiclass logic exists anywhere) | 2026-07-19T02:00:00Z | — | **complete** | 63e4c2e |
| E7.29 | 7 — Multiclass stacking | class:multiclass_save_stacking | PF1 best-fractional-progression save stacking — canonical source is `class_tables.rs`'s `GoodSaves`, **not** `decideEligibility.class_save_bonus` (that function doesn't exist in the Rust engine; corrected 2026-07-18) | 2026-07-19T02:00:00Z | — | **complete** | 02ff5ef |
| E7.30 | 7 — Multiclass stacking | class:multiclass_feature_integration | Reconcile two-class feature grants without clobbering | 2026-07-19T02:00:00Z | — | **complete** | 17bac41 |
| E4.19 | 4 — Closure Epilogue | closure:final_scan | Scan all 30 criteria for `complete`/blocked before opening the closure PR | 2026-07-19T06:00:00Z | ~600s | **complete** | n/a (scan; references the prior adversarial Opus-tier final scan — READY FOR CLOSURE, all 10 load-bearing gates independently re-verified); card t_783a4ada |
| E4.20 | 4 — Closure Epilogue | closure:pr | Open `tranche/4-1 → develop` promotion PR | 2026-07-19T06:00:00Z | ~300s | **complete** | PR #322 — https://github.com/electricm0nk/codex/pull/322; card t_99a9488d |
| E4.21 | 4 — Closure Epilogue | closure:worktree_cleanup | Worktree + stale-branch sweep | 2026-07-19T06:00:00Z | ~120s | **complete** | n/a (verification-only; `git worktree list` showed only the main checkout, `git branch -a` showed only develop/tranche-4-1/main/test/update-index — nothing outside SD-21's lane to remove); card t_ce18f469 |
| E4.22 | 4 — Closure Epilogue | closure:release_notes | Generate `release-notes.md` | 2026-07-19T06:00:00Z | ~600s | **complete** | 9a291f6; card t_5fb8368e |
| E4.23 | 4 — Closure Epilogue | closure:version_tranche_bump | Tranche-position version increment (only fires on next tranche promotion, not this release) | 2026-07-19T06:00:00Z | ~60s | **complete (no-op)** | n/a — verified all 3 version files still read `0.4.94`; correctly left unchanged since `tranche/4-1` is not being promoted to a new tranche this cycle; card t_dbae1b5c |
| E4.24 | 4 — Closure Epilogue | closure:final_tests | Full `cargo test` + `clippy` + acceptance-gate suite as closure gate | 2026-07-19T06:00:00Z | ~900s | **complete** | 8494dd1; card t_6c103d63 |

## SD-21 cycles

### Cycle log

#### cycle-2026-07-18T19:57:33Z | E1.1 (identifier:rust_tauri, sd16_browser_handoff sub-target) | b7a8201 | card t_0a79e528 | pending → running (1/7 sd*_*.rs files renamed) | cargo test 78/78 green (src-tauri) + TS 40/40 files green | clippy dirty (pre-existing, unrelated — see Open blockers) | ~900s

Renamed the Tauri command `sd16_browser_handoff` → `handoff_defect_report_to_browser`
across the Rust command definition (`sd16_browser_handoff.rs`), its `main.rs`
registration, the JS invoke-string caller (`browserHandoff.ts`), and the test
assertion (`browserHandoff.test.ts`). File name stays `sd16_browser_handoff.rs` per
Epic 1's explicit directory/file-rename carve-out (follow-on bundle). RED confirmed
first (updated test assertion failed against unrenamed code), then GREEN after the
rename. A follow-up full-repo sweep (see cycle below) found the remaining scope was
much smaller than this cycle assumed — corrected in the matrix row above.

#### cycle-2026-07-18T21:15:00Z | E1.1 (identifier:rust_tauri, support_state_matrix + closure) | 9206ad0 | card t_add3b188 | running → **complete** | cargo test 78/78 green (src-tauri) + TS 40/40 files green | clippy dirty (7 pre-existing, unrelated, identical locations) | ~1200s

A full-repo sweep (all `#[tauri::command]` blocks + every `invoke()`/template-literal
invoke call cross-referenced) found the prior cycle's assumption — 6 more `sd*_*.rs`
files needing renames — was wrong: the four `sd19_*_catalog.rs` files already exposed
clean command names (`list_spell_catalog` etc.), and `sd19_corpus.rs` exposes no Tauri
commands at all. The **only** remaining dirty command was `load_sd13_support_state_matrix`.
Renamed it → `load_support_state_matrix`, its internal builder
`load_sd13_support_state_matrix_snapshot` → `build_support_state_matrix_snapshot`, and
its Rust presentation types `Sd13SupportStateMatrixSnapshot` → `SupportStateMatrixSnapshot`
/ `Sd13SupportStateRow` → `SupportStateRowPresentation` (renamed rather than merely
de-prefixed, to avoid colliding with the already-imported
`rules_core::support_state_matrix::SupportStateRow` raw type in the same file — a real
naming hazard a blind find-replace would have hit). TS invoke-string updated to match;
TS-side `Sd13*` type/function names intentionally left for E1.2, same split established
for SD16. RED confirmed via the module's own `mod tests` (edited to reference new
names first, compile-failed against unrenamed production code), then GREEN after the
rename. **E1.1 is now fully complete** — closure-gate grep
(`sd16_|sd19_|sd13_` across `apps/desktop/src` + `apps/desktop/src-tauri/src`) shows
zero remaining hits in Tauri-command identifier or invoke-string positions.

#### cycle-2026-07-18T21:30:00Z | E1.2/E1.3 (identifier:ts_function_or_class / identifier:doc_comment, SD13 family) | f5a2184 | card t_76fb5380 | pending → complete | TS 41/41 green | no clippy changes | ~1100s

Renamed all Sd13/sd13-tagged TS identifiers to descriptive names across 4 files
(boundary/loadSd13SupportStateMatrix.ts, sd11/loadSd11TesterWorkbenchSurface.ts,
sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts, sd15/buildSd15OperatorTriageDraft.ts).
Rewrote all SD-13/SD13-Ex-Fxx doc-comments as plain prose in those same 4 files.
Added RED/GREEN test pinning the renamed exports. Verified zero remaining Sd13/SD13_/sd13-
identifiers outside filename references and new test fixture prose.

#### cycle-2026-07-18T21:45:00Z | E1.2/E1.3 (identifier:doc_comment, sd16/feedback subtree) | b73c708 | card t_0d675899 | pending → complete | TS 41/41 green | clippy clean | ~800s

Confirmed zero Sd16/SD16_ identifiers in controlledDefectPayload, submissionState,
submissionUiState and their tests (already clean). Rewrote 14 AV-PAY-N and SD-16 tag
mentions as plain prose in those 6 files plus sd16_browser_handoff.rs (Rust). Added
docCommentHygiene.test.ts regression (RED→GREEN) to prevent doc-comment tag leakage.
Verified roundtrip via RED test fixture scan before rewrite.

#### cycle-2026-07-18T22:00:00Z | E1.2 (identifier:ts_function_or_class, sd16/update) | 6f3768c | pending → complete | TS 41/41 green | no new clippy | ~1300s

Renamed 27 exported SD16 types/interfaces, 12 SD16_* constants, and 3 helper functions
across ~25 files in apps/desktop/src/sd16/update/. Fixed cross-boundary consumers in
App.tsx and boundary/loadSd11UpdateAction.ts. Verified zero naming collisions before
rename. Real fixture strings (STC-CODEX-SD-16, SD-16-feedback-loop spec-tranche paths)
confirmed on-disk and left untouched.

#### cycle-2026-07-18T22:15:00Z | E1.3 (identifier:data_testid, sd16/update) | 8d2be85 | pending → complete | TS 41/41 green | no new clippy | ~500s

Stripped sd16- prefix from every data-testid attribute across 10 files (Ui.tsx,
restoreOffer, lastCheckPanel, InstallControl, pendingRollbackPanel, installedPanel,
ChannelSelector, CheckPanel) and the matching id=\"sd16-pending-rollback-source-note\".
Updated UPDATE_UI_ID literal from 'sd16-update-ui' to 'update-ui'. Verified no testid
collisions repo-wide before stripping.

#### cycle-2026-07-18T22:30:00Z | E1.3 (identifier:doc_comment, sd16/update + App.tsx) | 22fa0a9 | pending → complete | TS 42/42 green | no new clippy | ~1200s

Rewrote 21 SD16-/SD-16-tagged doc-comment mentions across sd16/update/* and App.tsx as
plain prose. Fixed user-visible leaked identifier in pendingRollbackPanel.tsx
('SD-16-E7 staged-transaction module' → 'staged-transaction module'). Fixed
updateModel.ts's doc-comment referencing nonexistent artifact file. Fixed 4 stray
sd16-tagged snake_case test function names in parseUpdateManifest.test.ts. Updated 4
additional test files' shared makeSurface() fixture consumers (composeBugReport,
composeEnhancementRequest, captureFeedbackEvidence, buildSd15OperatorTriageDraft) to
avoid regression from fixture update.

#### cycle-2026-07-18T22:45:00Z | E2.5 (campaign:snapshot) | f208588 | card t_971a2dc2 | pending → complete | 122/122 lib tests green | clippy clean | ~900s

Defined CampaignSnapshot/CampaignMember/CampaignAsset/CampaignAssets types in new
src/campaign/mod.rs (sibling to src/saved_character/). Fields mirror campaignModel.ts
1:1 with schema_version:u16 from day one, serde default for legacy payloads. Added
serde/serde_json deps to root Cargo.toml (first external deps in codex crate).

#### cycle-2026-07-18T23:00:00Z | E2.6 (campaign:persistence, CampaignStore) | 739867c | card t_0ed1c2b2 | pending → complete | 125/125 lib tests green | clippy clean | ~700s

Added src/campaign/local_store.rs defining concrete CampaignStore struct (zero-field,
per operator directive — no CampaignBackend trait). Implemented save/load via
whole-snapshot JSON file round-trip, proven via characterization tests.

#### cycle-2026-07-18T23:15:00Z | E2.7 (campaign:persistence, local-folder layout) | da7cc5f | card t_28d9558c | pending → complete | 128/128 lib tests green | clippy clean | ~800s

Rewrote CampaignStore save/load to real local-folder layout (.config/<name>.json minus
assets + resources/adventure-log/maps/wiki dirs). Added list_all (tolerant of missing
root, per-entry failures isolated) and delete (idempotent). Moved sanitize_filename
helper to src/campaign/local_store.rs as pub(crate).

#### cycle-2026-07-18T23:30:00Z | E2.8 (campaign:persistence, Tauri commands) | b46e5cc | card t_c3b5dd03 | pending → complete | 80/80 bin tests + 130/130 lib tests green | clippy clean (6 known-baseline) | ~1100s

Added drive_list_campaigns/drive_load_campaign/drive_save_campaign/drive_delete_campaign
Tauri commands (local-folder backed, no OAuth) to campaign_drive.rs. Registered in
main.rs. write_campaign_drive_artifacts_impl became thin adapter over CampaignStore;
WriteCampaignDriveArtifactsRequest/Response preserved verbatim. Opportunistically fixed
pre-existing clippy::ptr_arg error at campaign_drive.rs:70.

#### cycle-2026-07-18T23:45:00Z | E2.9 (campaign:persistence, Obsidian round-trip) | dec3984 | card t_0f56d76e | pending → complete | 132/132 lib tests green | clippy clean | ~600s

Added two characterization tests pinning markdown-layout Obsidian round-trip: externally
edited asset .md file's new body honored on reload; brand-new .md file dropped into asset
dir outside app picked up with filename-derived title. No production code change needed —
E2.7's design already generalized support.

#### cycle-2026-07-19T00:00:00Z | E2.10 (campaign:conflict_log, nonce-based detection) | f761af1 | card t_3e783d38 | pending → complete | 136/136 lib tests + 81/81 bin tests green | clippy clean (6 known-baseline) | ~900s

Added nonce-based conflict detection to CampaignStore
(save_with_conflict_detection/load_with_nonce). Nonce stored as .config/nonce sidecar
(not in CampaignSnapshot fields, keeping 1:1 mirror with campaignModel.ts intact).
Stale expected_nonce moves prior on-disk state to conflicts/<timestamp>/ before writing
new snapshot. Wired through drive_save_campaign/drive_load_campaign DTOs.

#### cycle-2026-07-19T00:15:00Z | E2.11 (campaign:persistence, authorization surfaces) | a115da7 | card t_5a2eed9f | pending → complete | 45/45 desktop TS test files green | no new clippy | ~500s

Added computeCampaignManagerAccessGate (pure, tested) replacing hardcoded
OAuth-flavored disabledHint on campaign-manager landing banner with copy naming
local-folder picker directly. Dedicated test asserts hint never implies OAuth/account
flow. Wired into LandingScreen.tsx/CharacterHubPage.tsx. GoogleDrivePanel.tsx
(local-folder picker) needed no changes. Required `npm ci` first (node_modules missing).

#### cycle-2026-07-19T00:30:00Z | E5.25 (version:patch_bump) | 6ea6bfd | card t_d6201605 | pending → complete | TS 41/41 green + cargo check clean | clippy baseline unchanged | ~800s

Bumped apps/desktop/package.json, src-tauri/tauri.conf.json, src-tauri/Cargo.toml from
0.1.0 to 0.4.94 (major=0 until main-publish, tranche=4 for tranche/4-1, build=94
monotonic after alpha v0.0.93). Updated .github/workflows/publish-tester-release.yml
stamp from 0.0.<run_number> to 0.4.<run_number>. Added RED/GREEN test
buildVersionTriple.test.ts asserting agreement across all three files and workflow shape.
Ran cargo check to refresh Cargo.lock.

#### cycle-2026-07-19T00:45:00Z | E5.26 (version:build_label_format) | 5980037 | card t_c2c97b5d | pending → complete | TS 41/41 green | no new clippy | ~600s

Changed BUILD_PREFIX from 'codex' to 'Codex' and formatSd11WorkbenchBuildLabel template
from `${BUILD_PREFIX}@${buildVersion}` to `${BUILD_PREFIX} ${buildVersion}` (dropped @,
added space) in createSd11WorkbenchStatus.ts. Updated fixtures in
createSd11WorkbenchStatus.test.ts, loadSd11TesterWorkbenchSurface.test.ts,
testSupport/makeSurface.ts from 'codex@0.0.0-test' to 'Codex 0.4.94-test'. Updated 4
additional test files consuming shared makeSurface() fixture (composeBugReport,
composeEnhancementRequest, captureFeedbackEvidence, buildSd15OperatorTriageDraft) to
prevent regressions.

#### cycle-2026-07-19T01:00:00Z | E5.27 (version:closure_checklist) | d487416 | card t_bcc376d2 | pending → complete | TS 42/42 green | no new clippy | ~500s

Wrote docs/SD-21/release-closure-checklist.md documenting four-step version-bump
process: (1) bump version in package.json/tauri.conf.json/Cargo.toml + publish-tester-release.yml
stamp together; (2) build-label format check (Codex <version>); (3) cargo check for
Cargo.lock refresh; (4) commit as 'feat(sd21): bump version to <major>.<tranche>.<build>'.
Documented per-position increment rules. Added RED/GREEN test
releaseClosureChecklistDoc.test.ts asserting doc exists and covers all steps/keywords.

#### cycle-2026-07-19T01:15:00Z | E6.25/E6.26/E6.27 (class:compute_dispatch / class:compute_wizard_chassis, combined) | 2fe3b9f | pending → **running** (partial) | cargo test ~394 lines, 0 failures; cargo clippy clean (no new) | 6 known-baseline errors (unrelated files) | ~2100s

Refactored compute_pilot_base_chassis to per-class dispatch (Fighter/Wizard only,
multiclass deferred to Epic 7). Created compute_wizard_chassis consuming
class_tables::crb::class_tables()'s verified Wizard BAB/save rows. Widened
compute_total_saves's gate via new has_supported_class_chassis(input) helper so Wizard
gets ability modifiers too (previously Fighter-only). Followed compute_<class>_chassis
naming with generic explanation-ids. Updated two pre-existing SD13 tests via 'superseded'
idiom. Added tests/sd21_wizard_chassis_computes.rs covering real BAB/saves/combat
for Human Wizard 3, persistence of unrelated SD13 diagnostics, Fighter regression-free,
multiclass-stays-deferred. **Partial** because acceptance reproducer ('Wizard 3
Status:Computed, spellbook-unsupported diagnostic only') not fully reachable without
extending compute_combat_baseline/compute_selected_skill_modifiers dispatch gates and
resolving Wizard's two permanent spell-burden diagnostics (school-powers and
prepared-spellbook, both claim-blocking per SD13 fixtures) — see Open blockers for details.

#### cycle-2026-07-19T01:30:00Z | E3.12 (update:release_notes_fetch) | 8b984e8 | card t_<assigned> | pending → **complete** | TS 42/42 green | no new clippy | ~1200s

Added fetchReleaseNotesBody to apps/desktop/src/sd16/update/fetch.ts, fetching manifest.release_notes_url
and verifying against release_notes_hash via Web Crypto SHA-256, fail-closed on http-error
or hash-mismatch. Wired into controllerAdapter.ts's runCheck so a successful manifest
fetch now populates deps.releaseNotes and lastCheck.releaseNotesStatus='loaded' instead
of the never-assigned 'unavailable' stub. Added tests to fetch.test.ts and
controllerAdapter.test.ts, all green on first run.

#### cycle-2026-07-19T01:45:00Z | E3.13 (update:installed_state_probe) | bde03ca | card t_<assigned> | pending → **complete** | cargo test 78/78 green (src-tauri) | clippy clean (fixed 5 pre-existing baseline) | ~1100s

Implemented the body of is_install_eligible in apps/desktop/src-tauri/src/update/transaction.rs
(was a not-wired stub). Now reads installed-state.json and reports real facts. perform_install
stays an honest deferred error, but now names its exact narrower remaining gap: no HTTP
client dependency in this crate for the AppImage download step (dependency-adding decision
scoped out per file-touch partition). Opportunistically fixed 5 pre-existing baseline
clippy errors in this file (4× io::Error::other, 1× derivable Default) plus my own new
suspicious_open_options hit, per operator directive.

#### cycle-2026-07-19T02:00:00Z | E3.14 (update:computeDecision_rewire) | 326e139 | card t_<assigned> | pending → **complete** | TS 42/42 green | no new clippy | ~1300s

Rewired controllerAdapter.ts's computeDecision success path to call decideEligibility (eligibility.ts,
untouched/already-correct) fed by real data instead of a hardcoded 'unknown' short-circuit.
Broadened is_install_eligible's Rust return type from EligibilityPolicy verdict to
InstallEligibilityProbe{installed: Option<InstalledState>, is_managed_path_writable}
struct, since decideEligibility needs raw version/hash/install-kind fields to compare
against fetched manifest. Every degradation path (probe throws, no local record, probe
not yet run) still resolves honestly to 'unknown' with specific reason. Fixed real tsc
type error in E3.12's test code by adding expectReleaseNotesOk helper instead of
reusing FetchResult<T>-typed expectOk.

#### cycle-2026-07-19T02:15:00Z | E3.15 (update:tests_followup) | 5dfd002 | card t_<assigned> | pending → **complete** | TS 42/42 green | no new clippy | ~800s

Added remaining per-fetch/per-probe/per-decision outcome-coverage tests: Rust probe outcome
for unwritable managed-path parent directory, and TS decision outcomes for writability-false,
deb-install-kind (mapped to 'tarball'), and same-version/hash-as-installed. All 4 new tests
passed on first run, confirming E3.14's rewiring already correctly covers the full
decideEligibility row set. **E3 lane scope (E3.12-E3.15) fully complete** — all commits
on origin/tranche/4-1 verified via git fetch + log, kanban cards minted with merge_receipt_sha,
worktree clean. perform_install remains an honest deferred stub per file-touch scope
(HTTP client dependency-adding scoped out, flagged in code comment as future-slice decision).

#### cycle-2026-07-19T02:30:00Z | E7.28 (class:multiclass_dispatch) | 63e4c2e | card t_6492608f | pending → **complete** | cargo test ~400+ lines, 0 failures | cargo clippy clean | ~1600s

Verified Epic 6 landed compute_class_chassis dispatch + compute_wizard_chassis (commit 2fe3b9f)
before starting. Added compute_multiclass_base_chassis extending compute_class_chassis to
route length-2+ class_levels mixes by running each class's own compute_<class>_chassis
in per-class isolation (synthetic single-class CharacterInput clone) and combining results
(BAB summed; save combination superseded in E7.29). Widened has_supported_class_chassis so
compute_total_saves also becomes real for supported multiclass mixes. Updated pre-existing
Epic-6 negative control (multiclass_inputs_stay_deferred_to_epic_7 → multiclass_inputs_are_now_dispatch_supported_by_epic_7).
Added tests/sd21_multiclass_fighter_wizard_chassis_computes.rs (Fighter4/Wizard4 reproducer,
4 tests). cargo test + cargo clippy both clean.

#### cycle-2026-07-19T02:45:00Z | E7.29 (class:multiclass_save_stacking) | 02ff5ef | card t_bd71f80b | pending → **complete** | cargo test ~400+ lines, 0 failures | cargo clippy clean | ~1400s

Replaced E7.28's naive per-class-round-then-sum save combination with PF1's actual
multiclass rule: sum each class's un-rounded fractional save contribution (good=level/2+2,
poor=level/3, mirroring class_tables.rs's GoodSaves rows for Fighter and Wizard) across
the mix, then floor once. Added fractional_save_value and multiclass_good_saves helpers.
RED/GREEN proven via Fighter3/Wizard2 acceptance test: naive shape gives Fortitude 3,
correct fractional-sum gives 4 (3.5+0.667=4.167 floors to 4). Fixed one clippy::question_mark
lint (let-else replaced with ? operator) before landing. cargo test + clippy both clean.

#### cycle-2026-07-19T03:00:00Z | E7.30 (class:multiclass_feature_integration) | 17bac41 | card t_fcc378c4 | pending → **complete** | cargo test ~400+ lines, 0 failures | cargo clippy clean | ~1200s

Found that explain_fighter_class_features (Fighter's Bravery/bonus-feat grants) gated on
single-class-only supported_fighter_level, so Fighter's own feature explanations silently
vanished when any other class joined the mix — a real 'clobbering by omission' gap given
E7.28/E7.29 already made that same Fighter's BAB/saves genuinely computed. Added
fighter_level_in_mix (resolves Fighter's own sub-level within a supported multiclass mix,
falling back to supported_fighter_level for single-class) and rewired explain_fighter_class_features's
gate onto it. Proved via Fighter4/Wizard4 tests that class_feature.fighter.bravery now fires
(RED before, GREEN after) and combined class_chassis.* explanations stay single-entry, not
duplicated/clobbered by per-class feature reconciliation. **E7 lane scope (E7.28-E7.30)
fully complete** — all commits on origin/tranche/4-1 verified, working tree clean. Known
out-of-scope remainders (compute_combat_baseline, compute_selected_skill_modifiers gate
limits, deliberate multiclass support bounded to Fighter+Wizard pair) noted in commit
messages/kanban cards for future epic.

## Notes for future cycles

- **`character_hub.rs:sd19_demo_spells_selected()`** — an internal (non-Tauri-command)
  Rust helper fn with an `sd19_` prefix, found during E1.1's closure-gate grep. Does
  NOT block E1.1 (that criterion is scoped to Tauri *command* names specifically, and
  this isn't one), but it IS in scope for the full Epic-1 closure gate (acceptance gate
  8's `sd19_` sweep). Left untouched this cycle because `character_hub.rs` is Epic 2's
  file per the loop-instruction's file-touch partition (Epic 1 reads it, doesn't edit
  it). Whoever's E1.2/E1.3 cycle does the final Rust-side identifier sweep should
  either get explicit partition clearance to touch this one function, or Epic 2's own
  first cycle (which touches this file anyway) should fold in the rename.
- `sd19_corpus_fixtures` (a resource-directory name referenced in `tauri.conf.json`,
  `sd19_corpus.rs`, `sd19_class_catalog.rs` doc comments) and the engine-crate fixture
  fn `seeded_sd13_e1_f1_current_truth` (defined in the root `codex` crate's
  `rules_core::support_state_matrix`, outside `apps/desktop/`) are out of Epic 1's
  scope entirely — Epic 1's file-touch partition never includes the root engine crate,
  matching how the chassis/corpus-aware seam files stay untouched by Epics 1-2-3-5.

#### cycle-2026-07-19T04:00:00Z | integration fix (not tied to one criterion) | b207853 | no card (operator-direct fix, not a loop cycle) | n/a | src-tauri 85/85 green (was 84/1 FAILED) | clippy 1 known pre-existing error (character_hub.rs:185, unrelated) | ~15 min

Post-workflow independent verification at the Wave-1+Wave-2 merged HEAD (`5dfd002`)
found one real test failure that no individual lane's own local run had caught:
`character_hub.rs`'s `claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class`
still expected Wizard's pre-Epic-6 diagnostic shape. Fixed by updating the test's
expectation to the new, correct 4-diagnostic set (removing `class_chassis.unsupported`
and `defense.total_save.unsupported`, which Epic 6's dispatch fix genuinely resolved
for Wizard). Also independently confirmed the full closure gate for E1.2/E1.3
(a repo-wide grep the individual lanes' self-reports didn't run) and corrected their
status-matrix rows from `running` to `complete` accordingly — see the matrix above.

#### cycle-2026-07-19T04:30:00Z | E6b.1 (class:combat_baseline_dispatch) | 6ed19bd | card t_5b0e95ee | pending → **complete** | cargo test 397/397 lines ok, 0 failures | cargo clippy clean | ~600s

Widened both `compute_combat_baseline` and `compute_selected_skill_modifiers` functions'
class gate from `supported_fighter_level(input)`-only to `has_supported_class_chassis`
(mirroring `compute_total_saves`'s Epic-6 widening). Conditioned the `choice:fighter_bonus_feat`
requirement on Fighter actually being the dispatch-supported class (a Wizard has no such
class feature). A Wizard 3 with the GE-06 combat/skill posture (Longsword/Chain Shirt/Dodge/Weapon
Focus/no-shield, Climb/Intimidate/Swim rank 1) now clears `combat.baseline_unsupported`
and `skill.selected_modifier.unsupported`. Fighter regression-tested unaffected (melee attack
bonus still 5). Tests added/updated in `tests/sd21_wizard_chassis_computes.rs`.

#### cycle-2026-07-19T04:30:00Z | E6b.2 (class:wizard_prepared_spellbook) | de554ab | card t_619dd84b | pending → **complete** | cargo test 397/397 lines ok, 0 failures | cargo clippy clean | ~1200s

Grounded real prepared-spellbook/daily-preparation state via the existing SD-19
`spells_selected` field (`AcquisitionMode::Known`=spellbook contents, `Prepared`=daily prep)
with no CharacterInput schema change. Added real, 2-source-verified (d20pfsrd.com + Archives of
Nethys) PF1 Wizard base spells-per-day table (levels 1-3) and reused the existing
Paladin/Ranger/Sorcerer/Bard Intelligence-bonus-spells formula. Spell IDs use a documented,
corpus-free convention `<school>.<level>.<name>` reusing the real `Pf1SchoolId` enum. Grounds
real per-level slot-consumption validation: prepared-vs-known structural check, opposed-school
spells cost 2 slots, total consumption vs total budget (base+specialist bonus+Int bonus).
`class_spell.wizard.prepared_spellbook.unsupported` now clears for a populated in-budget spellbook;
stays blocked with a more specific unmet-reason message otherwise. New test file
`tests/sd21_wizard_prepared_spellbook.rs` (5 tests) plus `sd13_wizard_evocation_school_powers.rs`
message-content regression fix.

#### cycle-2026-07-19T04:30:00Z | E6b.3 (class:wizard_school_powers, Evocation reproducer) | 1c7ad89 | card t_db0c1dbf | pending → **complete** | cargo test 397/397 lines ok, 0 failures | cargo clippy clean | ~1400s

Confirmed Intense Spells / Force Missile flat magnitudes remain grounded (landed pre-Epic-6b in
Epic 6's own cycle) and live. The opposed-school 2-slot preparation cost (verified against
d20pfsrd.com: 'a wizard who prepares spells from his opposition schools must use two spell slots
of that level') was grounded as part of E6b.2's slot-consumption check (`wizard_opposed_school_slot_cost`),
proven both fitting-at-budget and over-budget in `tests/sd21_wizard_prepared_spellbook.rs`.
`class_feature.wizard.school_powers_and_opposed_school_cost.unsupported` now clears together with
the spellbook diagnostic. Added capstone acceptance test `tests/sd21_epic6b_full_completion_reproducer.rs`:
single-class Human Wizard 3, Evocation specialization, GE-06 combat/skill posture, populated
Evocation-only spellbook + daily prep → `build_pilot_headless_receipt` reaches `Status::Computed`
with zero claim-blocking diagnostics. Bounded to Evocation only per operator-sanctioned scope;
the other 7 PF1 schools are an explicit follow-on (Epic 6c/6d/...). Worktree clean, full test
suite green (src-tauri 85/85, TS 45/45, root crate 397/397).

#### cycle-2026-07-19T05:15:00Z | integration fix #2 (not tied to one criterion) | 6c7a12b | no card (operator-direct fix, not a loop cycle) | n/a | src-tauri 85/85 green (was 84/1 FAILED) | clippy 1 known pre-existing error (character_hub.rs:185, unrelated) | ~10 min

Same integration test as cycle `2026-07-19T04:00:00Z`'s fix (`b207853`) hit again, for a
different reason: Epic 6b's E6b.1 (`6ed19bd`) widened `compute_combat_baseline`/
`compute_selected_skill_modifiers` to `has_supported_class_chassis`, so
`combat.baseline_unsupported`/`skill.selected_modifier.unsupported` no longer trip for
Wizard at ANY input, not just the Epic 6b reproducer's populated-spellbook one. Fixed
the test's Wizard-at-level-1 expectation to the new, correct 2-diagnostic set (only the
still-genuinely-unsupported school-powers and prepared-spellbook diagnostics remain for
an empty spellbook). Confirms the same lesson from before: an independent full-suite
run at the final merged HEAD is necessary after every workflow wave, even a single-lane
one — the workflow's own local tests correctly tested its own new reproducer, but had
no reason to touch this pre-existing, unrelated, cross-cutting assertion.

#### cycle-2026-07-19T06:00:00Z | Epic 4 — Closure Epilogue (E4.19-E4.24, hygiene reconciliation) | 9a291f6, 8494dd1 | no card yet (minted immediately after this cycle log entry) | pending → **complete** (all 6 Epic 4 criteria) | root crate cargo test 397/397 lines ok, 0 failed; src-tauri cargo test 85/85 green; TS 45/45 test files green | clippy clean (root and src-tauri, 0 known errors remaining) | ~35 min

**E4.19 (final scan):** re-confirmed the prior adversarial Opus-tier final-scan verdict
(READY FOR CLOSURE — all ten load-bearing gates 1/2/4/5/6/8/9/10/11/12 independently
re-verified; gate 3 legitimately descoped per operator directive; gates 7/13 are this
closure action itself). Walked the full status matrix: every criterion E1.1-E7.30 is
`complete`; corrected the stale E6.25/E6.26/E6.27 rows from `running` to `complete`
(the underlying dispatch code landed at `2fe3b9f` and Epic 6b's later work proved the
full acceptance reproducer reachable — the `running` status was a snapshot of a
mid-lane self-report that was never rolled forward once Epic 6b closed the gap).

**E4.20 (PR):** opened the `tranche/4-1 → develop` promotion PR (see PR URL in the
final report). Body includes a one-line summary per epic (1-7 + 6b), the full receipt
SHA list from this matrix, a link to the release-notes preview (`9a291f6`), and an
explicit callout of the two known, accepted, non-blocking follow-on items (Epic 6's
non-Wizard single classes; Epic 2's four `drive_*` commands lacking frontend bindings).

**E4.21 (worktree/branch sweep):** `git worktree list` showed only the main checkout
(already clean, nothing to remove). `git branch -a` showed only
develop/tranche-4-1/main/test/update-index (remotes) plus local tranche/4-1 — no
branches outside SD-21's own lane needed reporting or deletion. Verification-only
cycle, no commit produced by this step.

**E4.22 (release notes):** generated
`programs/codex/requirements/SD-21-campaign-manager-and-persistence/release-notes.md`
with New features / Bug fixes / Rules engine / Maintenance / Versioning sections,
citing the real commit SHAs from this matrix. Committed and pushed as `9a291f6`;
verified present on `origin/tranche/4-1` via independent `git fetch` + `git log`.

**E4.23 (version increment):** verified all three version files
(`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`,
`apps/desktop/src-tauri/Cargo.toml`) still read `0.4.94` (Epic 5's value, unchanged).
Confirmed this is correctly a no-op: `tranche/4-1` is not being promoted to a new
tranche number in this closure action, so the tranche position does not bump. The
tranche-promotion bump (`0.4.<last_build>` → `0.5.0`) is deferred to whichever future
bundle actually launches on `tranche/5`.

**E4.24 (final tests, closure gate):** ran the full suite fresh at the merged HEAD
rather than trusting the matrix's `complete` markers, per the lesson from the two
prior post-workflow integration fixes (`b207853`, `6c7a12b`). Root crate: `cargo test
--locked` 397/397 test-result lines `0 failed`; `cargo clippy --locked --tests -- -D
warnings` clean. `apps/desktop/src-tauri`: `cargo test --locked` 85/85 green;
`cargo clippy --locked --tests -- -D warnings` initially surfaced the one previously-known
pre-existing exception (`character_hub.rs:185`, `clippy::large_enum_variant` on
`CreateCharacterResponse`) — resolved definitively this cycle by boxing the `Saved`
variant's `summary: CharacterSummaryDto` field to `Box<CharacterSummaryDto>` (serde
serializes `Box<T>` identically to `T`, so the TS boundary's wire shape is unchanged;
verified via the single construction site at `character_hub.rs:481` and a re-run of the
full src-tauri suite, still 85/85 green). Committed as `8494dd1`, pushed, and re-verified
clippy-clean afterward. `apps/desktop`: TS test suite (`npm test`) 45/45 test files
green. **Closure gate now genuinely clean across every surface** — zero known clippy
exceptions remain anywhere in the tree.

## Open blockers

- **GATE 11 SATISFIED (2026-07-19): Epic 6b landed — E6's acceptance reproducer now reachable.**
  All three E6b criteria (E6b.1, E6b.2, E6b.3) landed at commits 6ed19bd, de554ab, 1c7ad89.
  Capstone test `tests/sd21_epic6b_full_completion_reproducer.rs` passes: Human Wizard 3,
  Evocation specialization, GE-06 combat/skill posture, populated Evocation-only spellbook +
  daily prep → `build_pilot_headless_receipt` reaches `Status::Computed` with zero claim-blocking
  diagnostics. Epic 4's closure gate (criterion E4.19 gate 11 scan) can now move to met.
  See `epic-breakdown.md`'s "Epic 6b — Wizard full-completion" section and
  this doc's `E6b.1`/`E6b.2`/`E6b.3` status-matrix rows and cycle log above.

- **E6 non-Wizard single classes still unsupported.**
  Only Fighter and Wizard are dispatch-supported in compute_class_chassis. Every other
  single class (Barbarian, Bard, Cleric, Druid, Monk, Paladin, Ranger, Rogue, Sorcerer)
  still falls through the dispatch's `_ => None` arm to the generic class_chassis.unsupported
  fallback, same as before this cycle. Extending to those classes (each has its own
  already-verified class_tables.rs row) is straightforward per-class follow-on work but
  not attempted this lane given effort budget.

- **E2 four new Tauri commands lack frontend boundary bindings.**
  E2.8 added drive_list_campaigns/drive_load_campaign/drive_save_campaign/drive_delete_campaign
  Tauri commands (Rust layer, tested, registered in main.rs) but nothing in
  apps/desktop/src/boundary/ calls them yet. campaignModel.ts's actual persistence flow
  still goes exclusively through the pre-existing write_campaign_drive_artifacts
  (write-only). Wiring the frontend to actually load/list/delete from the local folder
  (not just write-through or localStorage) is real follow-on scope if a future cycle
  wants Campaign Manager to read back from disk. Flagged here rather than silently
  implicit.

- **Pre-existing clippy debt — RESOLVED to 0 of 7 (2026-07-19, Epic 4 closure).** The
  original 7 pre-existing errors (`campaign_drive.rs:70`, `character_hub.rs:185`,
  `update/transaction.rs:1065,1122,1191,1281,1757`) predated any SD-21 cycle (verified
  via `git stash`/`git stash pop` around cycle `2026-07-18T19:57:33Z`, confirmed to come
  in with PR #320/#321). Per the standing instruction to fix opportunistically when a
  cycle already has the file open: E2.8's lane fixed `campaign_drive.rs:70`; E3.13's
  lane fixed all 5 in `update/transaction.rs`. The last one, `character_hub.rs:185`
  (large-size-difference `CreateCharacterResponse` enum variant), was resolved in Epic
  4's closure cycle (commit `8494dd1`) by boxing the `Saved` variant's `summary` field
  (`Box<CharacterSummaryDto>` — serde-transparent, single construction site, no wire-shape
  change). `cargo clippy --locked --tests -- -D warnings` in `apps/desktop/src-tauri` is
  now clean with zero errors. Epic 4's closure gate (criterion E4.24) is satisfied.

- **Integration-level regression caught and fixed post-workflow (2026-07-19), not by
  any individual lane.** After the Epic 1/2/3/5/6/7 parallel workflow run, an
  independent full-suite verification at the merged HEAD (`5dfd002`) found
  `character_hub.rs`'s `claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class`
  test FAILING — not a real regression, but a stale expectation: the test still assumed
  Wizard trips `class_chassis.unsupported` and `defense.total_save.unsupported` (the
  pre-Epic-6 Fighter-only-dispatch shape), which Epic 6's cycle (`2fe3b9f`) genuinely
  fixed. No lane was scoped to touch `character_hub.rs` (Epic 2's lane was explicitly
  read-only on it, Epic 6's lane worked only in the root engine crate), so this
  cross-cutting staleness only surfaced once all lanes' changes were combined — each
  lane's own local test run was green in isolation. Fixed directly in commit `b207853`;
  full suite re-verified green afterward (root crate 395+/415 test-result lines ok,
  src-tauri 85/85, TS 45/45). **Lesson for future waves: an integration-level full-suite
  re-run at the final merged HEAD is necessary after any parallel wave — per-lane green
  does not guarantee combined-state green when lanes touch the same downstream
  consumer's assumptions, even across file-touch-partition boundaries.**
