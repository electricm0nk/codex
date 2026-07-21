# Desktop App

> Scope: How the Tauri desktop shell is built, how it talks to the Rust backend, and how its frontend surfaces are organized.
> Last verified: 2026-07-20 against ef9012bf5de8
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

## Build shape

The desktop app lives at `apps/desktop/` and is a React 18 + Tauri 2 application. The frontend is Vite-built; the backend is a thin Tauri (Rust) shell over the root headless `codex` crate.

- **Entry chain**: `apps/desktop/index.html` loads `/src/main.tsx` as a module script. `apps/desktop/src/main.tsx` mounts `<App />` (from `apps/desktop/src/App.tsx`) into `#root` inside `<React.StrictMode>`, after importing `./theme.css`.
- **Vite config** (`apps/desktop/vite.config.ts`): uses `@vitejs/plugin-react`; both the dev server and the preview server are pinned to port `1420` with `strictPort: true` (the port Tauri's `devUrl` points at — see below), so a port conflict fails loudly instead of silently picking a different port.
- **Tauri shell config** (`apps/desktop/src-tauri/tauri.conf.json`): `build.beforeDevCommand` is `npm run dev`, `build.beforeBuildCommand` is `npm run build`, `build.frontendDist` is `../dist`, and `build.devUrl` is `http://localhost:1420` — matching the Vite port above. The app window (`app.windows[0]`) is titled "Codex", `1920x1200`, resizable. Bundle targets are `deb`, `appimage`, `msi`, `nsis`, `app`, `dmg`; bundled `resources` include `resources/ge08/guard-stance-package/` and `resources/sd19_corpus_fixtures/`.
- **src-tauri is a thin IPC shell over the root crate**: `apps/desktop/src-tauri/Cargo.toml` declares `codex = { path = "../../.." }` — the Tauri crate (`codex-desktop`) depends on the repo-root `codex` crate by relative path, not a published version. Its other dependencies are `serde`, `serde_json`, `sha2`, `base64`, `tauri`, `tauri-plugin-opener`, and `tauri-plugin-dialog`. There is no HTTP client crate (no `reqwest`/`ureq`/etc.) in the dependency tree — this is the concrete reason `perform_install` cannot download an update artifact today (see [update-and-feedback.md](./update-and-feedback.md)).
- **npm scripts** (`apps/desktop/package.json`): `dev` → `vite`; `typecheck` → `tsc --noEmit`; `test` → `node scripts/run-tests.mjs`; `build` → `vite build`; `tauri:check` → `cargo check --manifest-path src-tauri/Cargo.toml`. Frontend dependencies are `@tauri-apps/api`, `@tauri-apps/plugin-dialog`, `ajv`, `react`, `react-dom`; dev dependencies include `@tauri-apps/cli`, `vite`, `typescript`, `tsx`.
- **`tsconfig.json`** targets ES2020, uses `moduleResolution: "Bundler"`, and enables `resolveJsonModule`. Its `include` list additively pulls in `../../schemas/update/*.json` and `../../tests/fixtures/update/**/*.json` from the repo root — this is how `sd16/update/loadSchemas.ts` imports the canonical JSON Schema documents as typed modules directly from `schemas/update/` (see [update-and-feedback.md](./update-and-feedback.md)).
- **Git-sha embedding**: `apps/desktop/src-tauri/build.rs` runs `git rev-parse --short=12 HEAD` at compile time and sets the `CODEX_GIT_SHA` rustc env var (falling back to `"unknown"` outside a git checkout). It also emits `cargo:rerun-if-changed=../../../.git/HEAD` so a commit anywhere in the repo — not just inside the crate — invalidates the cached build and refreshes the embedded hash. The `load_backend_health` Tauri command (`apps/desktop/src-tauri/src/main.rs`) reads this via `env!("CODEX_GIT_SHA")` alongside `env!("CARGO_PKG_VERSION")`, so the running app can report exactly which commit and crate version it was built from — reaching the command at all is itself proof the IPC bridge is alive.

## The boundary rule

**Rule: components never call `invoke()` inline.** IPC calls are meant to go through a dedicated wrapper under `apps/desktop/src/boundary/`, each of which follows the same shape: check `hasTauriRuntime()`, and either `invoke()` the real Tauri command or — outside a real Tauri runtime (e.g. the Vite browser preview at `localhost:1420`, or a `vitest`/`tsx` test host) — let the caller's runtime layer fall back to browser-preview sample data.

`apps/desktop/src/boundary/runtime.ts` is the shared seam every wrapper (and the direct-invoke exceptions below) depends on:

```ts
export function hasTauriRuntime(): boolean {
  return typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
}
export function formatError(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
}
```

The boundary files (all under `apps/desktop/src/boundary/`), one per Tauri command family:

`characterPortrait.ts`, `cloneCharacter.ts`, `exportCharacterJson.ts`, `loadBackendHealth.ts`, `loadClassCatalog.ts`, `loadCreateCharacter.ts`, `loadEquipmentCatalog.ts`, `loadGe08AuthoringWorkbench.ts`, `loadListSavedCharacters.ts`, `loadPilotShellSnapshot.ts`, `loadRaceCatalog.ts`, `loadSavedCharacterDetail.ts`, `loadSd11UpdateAction.ts`, `loadSd12ReleaseTruth.ts`, `loadSd13SupportStateMatrix.ts`, `loadSpellCatalog.ts`, `writeCampaignDriveArtifacts.ts`, plus the shared `runtime.ts` itself.

**Verified exception to the "boundary/*.ts only" rule**: three files call `invoke()` directly rather than through a dedicated `boundary/` wrapper, though all three still import `hasTauriRuntime`/`formatError` from `boundary/runtime.ts` and each keeps its own testability seam so tests never touch the real `invoke`. The guard discipline holds even where the file-per-command convention doesn't:

- `apps/desktop/src/sd16/update/controllerAdapter.ts` — its `callInvoke<T>` helper wraps `invoke()` for `is_install_eligible`, `verify_relaunch_artifact`, and `perform_restore_previous`; accepts an injectable `invokeImpl`.
- `apps/desktop/src/sd16/update/installAction.ts` — `performInstall()` calls `invoke("perform_install", …)` directly; returns a no-runtime sentinel (`NO_TAURI_RUNTIME_RESPONSE`) and keeps its DOM output pure (`buildRelaunchPromptMarkup`) as its testability seam, rather than an injectable `invokeImpl`.
- `apps/desktop/src/sd16/feedback/browserHandoff.ts` — `runBrowserHandoff()` calls `invokeImpl('handoff_defect_report_to_browser', …)` directly (default `invokeImpl` is the real `invoke`); accepts an injectable `invokeImpl`.

## The complete Tauri command inventory

Derived directly from `#[tauri::command]` attributes in every `apps/desktop/src-tauri/src/*.rs` file and cross-checked against the `tauri::generate_handler![...]` list in `apps/desktop/src-tauri/src/main.rs`. **26 commands are registered** (reachable via `invoke()`); one additional `#[tauri::command]`-attributed function exists in the source but is **not** registered (see note below the table).

| Command | Rust file | What it does | Boundary wrapper | Main consuming surface |
|---|---|---|---|---|
| `load_pilot_shell_snapshot` | `main.rs` (inline) | Returns a fixed placeholder snapshot naming the GE-06/GE-07 scaffold seam; never real product truth. | `boundary/loadPilotShellSnapshot.ts` | legacy GE07 scaffold path, superseded by `characterHub` |
| `load_ge08_authoring_workbench_snapshot` | `main.rs` (inline command) delegating to `ge08_workbench.rs`'s `build_ge08_workbench_snapshot` | Loads and previews a GE08 homebrew authoring package (manifest + authored records + AC preview). | `boundary/loadGe08AuthoringWorkbench.ts` | GE08 authoring workbench (referenced from the SD-11 tester workbench) |
| `load_support_state_matrix` | `main.rs` (inline command) delegating to `sd13_support_state_matrix.rs`'s `build_support_state_matrix_snapshot` | Read-only projection of the seeded SD-13 support-state/evidence-tier matrix, verbatim. | `boundary/loadSd13SupportStateMatrix.ts` | `App.tsx`'s `SupportDebtPanel` / `BreadthClaimAuditPanel` |
| `load_backend_health` | `main.rs` (inline) | Reports the Rust crate version (`CARGO_PKG_VERSION`) and the compile-time git short SHA (`CODEX_GIT_SHA`, from `build.rs`). Reaching it proves the IPC bridge is alive. | `boundary/loadBackendHealth.ts` | `App.tsx` "Backend" status card |
| `handoff_defect_report_to_browser` | `sd16_browser_handoff.rs` | Builds + re-validates a prefilled GitHub "new issue" URL, then opens it via `tauri-plugin-opener`; returns `opened: true` only after the OS-level open succeeds. | direct `invoke()` in `sd16/feedback/browserHandoff.ts` (no dedicated `boundary/*.ts` file) | Bug-report / enhancement-request composers in `App.tsx` |
| `is_install_eligible` | `update/transaction.rs` | Real local-state probe: reads `installed-state.json` and reports install kind/version/hash/managed-path writability; does not itself render an eligible/ineligible verdict. | direct `invoke()` in `sd16/update/controllerAdapter.ts` | `sd16/update` eligibility check (`decideEligibility` consumes the facts) |
| `perform_install` | `update/transaction.rs` | **Stub**: always returns `Err(...)`. See note below. | direct `invoke()` in `sd16/update/installAction.ts`'s `performInstall()` | defined but not called by any live UI path (see [update-and-feedback.md](./update-and-feedback.md)) |
| `perform_restore_previous` | `update/transaction.rs` | Real rollback: restores the most recent backup AppImage over the managed path, per the AV-RB decision table (promoted / auto-restored / rollback-failed / no-backup / no-pending). | direct `invoke()` in `sd16/update/controllerAdapter.ts`'s `restorePreviousVersion()` | `App.tsx`'s `UpdateSection` restore-offer flow |
| `verify_relaunch_artifact` | `update/transaction.rs` | Hashes the running binary and compares it against `pending-update.json`; on match, promotes to a fresh `installed-state.json`. | direct `invoke()` in `sd16/update/controllerAdapter.ts`'s `loadMountTimeState()` | `App.tsx`'s `UpdateSection`, run at every mount |
| `create_character` | `character_hub.rs` | Composes a `CharacterInput` from race/class/level/ability scores plus a fixed feat/skill/equipment loadout, computes it via the real rules-core engine, and — only if `Computed` — persists via `SavedCharacterStore`. | `boundary/loadCreateCharacter.ts` | `apps/desktop/src/characterHub/characterHubRuntime.ts`'s `createCharacterRuntime` (see worked example below) |
| `clone_character` | `character_hub.rs` | Duplicates a saved character's full `CharacterInput` under a new id, recomputes, and saves; never persists an unproven recompute. | `boundary/cloneCharacter.ts` | Character Hub clone action |
| `list_saved_characters` | `character_hub.rs` | Lists every saved character summary under the app data dir's `characters/` root. | `boundary/loadListSavedCharacters.ts` | `apps/desktop/src/characterHub/buildCharacterHubListSurface.ts` |
| `load_saved_character` | `character_hub.rs` | Loads one saved character, recomputes its snapshot/diagnostics/corpus-derived data live from the stored `CharacterInput`. | `boundary/loadSavedCharacterDetail.ts` | `apps/desktop/src/characterHub/CharacterSheet.tsx` |
| `save_character_portrait` | `character_hub.rs` | Writes base64-decoded PNG bytes to `portrait.png` next to an already-saved character (3 MiB defensive cap). | `boundary/characterPortrait.ts` | `apps/desktop/src/characterHub/PortraitUpload.tsx` |
| `load_character_portrait` | `character_hub.rs` | Returns the character's portrait as a `data:image/png;base64,...` URL, or `None`. | `boundary/characterPortrait.ts` | `apps/desktop/src/characterHub/PortraitUpload.tsx` |
| `delete_character_portrait` | `character_hub.rs` | Removes `portrait.png` if present. | `boundary/characterPortrait.ts` | `apps/desktop/src/characterHub/PortraitUpload.tsx` |
| `export_character_json` | `character_hub.rs` | Writes arbitrary JSON text to a caller-supplied path (chosen via the dialog plugin), outside the app data dir. | `boundary/exportCharacterJson.ts` | Character Hub export action |
| `write_campaign_drive_artifacts` | `campaign_drive.rs` | Deserializes a JSON `Campaign` snapshot and delegates to `codex::campaign::local_store::CampaignStore` to write `.config/<name>.json` + markdown asset files under a Drive-folder-like local path. | `boundary/writeCampaignDriveArtifacts.ts` | `apps/desktop/src/campaign/campaignModel.ts`'s `syncCampaignDriveArtifacts` (one-way mirror — see State approach below) |
| `drive_list_campaigns` | `campaign_drive.rs` | Lists campaign summaries under a local folder root, via `CampaignStore::list_all`. | **none** | not called from any frontend code today (see note below) |
| `drive_load_campaign` | `campaign_drive.rs` | Loads one campaign snapshot plus a conflict-detection nonce, via `CampaignStore::load_with_nonce`. | **none** | not called from any frontend code today |
| `drive_save_campaign` | `campaign_drive.rs` | Saves a campaign snapshot with optimistic-concurrency conflict detection (`expected_nonce`), via `CampaignStore::save_under_root_with_conflict_detection`. | **none** | not called from any frontend code today |
| `drive_delete_campaign` | `campaign_drive.rs` | Deletes a campaign folder, via `CampaignStore::delete`. | **none** | not called from any frontend code today |
| `list_equipment_catalog` | `sd19_equipment_catalog.rs` | Full CRB equipment table (~2,977 records across 4 categories), literal, not per-character. | `boundary/loadEquipmentCatalog.ts` | `apps/desktop/src/equipmentCatalog/EquipmentCatalogScreen.tsx` |
| `list_spell_catalog` | `sd19_spell_catalog.rs` | Full CRB spell list (652 records across 9 strict schools), literal, not per-character. | `boundary/loadSpellCatalog.ts` | `apps/desktop/src/spellCatalog/SpellCatalogScreen.tsx` |
| `list_class_catalog` | `sd19_class_catalog.rs` | Full CRB class progression table (207 rows across 11 classes), literal, not per-character. | `boundary/loadClassCatalog.ts` | `apps/desktop/src/classCatalog/ClassCatalogScreen.tsx` |
| `list_race_catalog` | `sd19_race_catalog.rs` | Full CRB race trait table, literal, not per-character. | `boundary/loadRaceCatalog.ts` | `apps/desktop/src/raceCatalog/RaceCatalogScreen.tsx` |

**Setup-time hook**: `main.rs`'s `tauri::Builder::default().setup(...)` calls `character_hub::seed_default_character_if_needed(app.handle())` on every app launch — verified in `main.rs`. It seeds a starter character ("Aldric Ironhand", a single-class Human Fighter 3) into a fresh install, gated on a `.default_character_seeded` marker file in the app data dir (not on whether the characters directory is empty, so deleting the starter character does not resurrect it). It reuses `compose_character_input`/the same `Computed`-only-persists invariant as `create_character`, and only single-class Fighter levels 1–3 reach `Computed` today (see Character flow below).

**Registered-but-unreachable-from-TS gap**: `update/transaction.rs` also defines `perform_retention_sweep` with a real, tested body (`perform_retention_sweep_impl`) that enforces the backup/staging/pending retention policy. It carries `#[tauri::command]`, but `main.rs`'s `use update::transaction::{ is_install_eligible, perform_install, perform_restore_previous, verify_relaunch_artifact };` import list and its `generate_handler![...]` list both omit it — so it is not reachable via `invoke()` from the frontend at all. It exists as tested Rust-only surface.

**Registered-but-unmirrored-in-TS gap**: `drive_list_campaigns`, `drive_load_campaign`, `drive_save_campaign`, and `drive_delete_campaign` are registered, real, and tested (`campaign_drive.rs`'s `#[cfg(test)]` module round-trips save/list/load/delete against a temp dir), but no file under `apps/desktop/src/boundary/` invokes them and no other TS file calls `invoke()` with those command names — confirmed by grepping every `invoke(`/`invoke<` call site in `apps/desktop/src`. `apps/desktop/src/campaign/campaignModel.ts` explicitly documents that campaigns are sourced from `localStorage`, and only `write_campaign_drive_artifacts` is called, as a one-way write-through mirror (see State approach below).

## Frontend directory map

All paths below are under `apps/desktop/src/`.

**`characterHub/`** — the mode state machine that is the app's actual home screen. `CharacterHubPage.tsx` holds a `Mode` union with exactly these values (verified in the file): `'landing' | 'load' | 'create' | 'sheet' | 'equipmentCatalog' | 'spellCatalog' | 'classCatalog' | 'raceCatalog' | 'dm-toolkit' | 'campaign-list' | 'campaign-create' | 'campaign-edit' | 'campaign-sheet'`. It is a plain `useState<Mode>` switchboard with no router library — each mode renders a different top-level screen component and threads callbacks like `onCreate`/`onLoad`/`onBrowseEquipment` back to `setMode`. The directory also holds the character-creation and character-sheet machinery: `composeCreateCharacterRequest.ts` (pure request builder), `characterHubRuntime.ts` (DI runtime layer), `buildCharacterHubListSurface.ts` / `buildCreateCharacterOutcomeSurface.ts` (pure surface builders), `characterHubModel.ts` (race/class catalogues and PF1 math helpers), `characterProgression.ts` and `skillsModel.ts` (level/skill-point math), `previewData.ts` (browser-preview fallback sample data), and the screen/dialog components (`LandingScreen.tsx`, `CreateCharacterForm.tsx`, `LoadCharacterScreen.tsx`, `CharacterSheet.tsx`, `LevelUpDialog.tsx`, `SkillAllocationDialog.tsx`, `PortraitUpload.tsx`, `CharacterListRow.tsx`, `StubScreen.tsx`).

**`campaign/`** — campaign management screens (`CampaignManagerScreen.tsx`, `CreateCampaignScreen.tsx`, `EditCampaignScreen.tsx`, `CampaignSheet.tsx`) plus `campaignModel.ts`, the localStorage-backed data model (see State approach below), and `campaignManagerAccessGate.ts`, a pure gate function deciding whether the Campaign Manager entry point is reachable (it requires Google Drive to be "configured" — see `settings/googleDrive.ts`).

**`classCatalog/`, `raceCatalog/`, `spellCatalog/`, `equipmentCatalog/`** — four near-identical catalog browsers, each following the same `*Screen.tsx` + `*Runtime.ts` pair: the `Screen` component renders a literal, full listing of every real corpus record the engine knows about (not a per-character sample), and the `Runtime` module is the DI seam described below (real boundary loader behind Tauri, hardcoded preview sample array otherwise).

**`sd11/`** — the tester workbench and its feedback composers. `loadSd11TesterWorkbenchSurface.ts` / `loadSd11TesterWorkbenchSurfaceRuntime.ts` assemble the single `Sd11TesterWorkbenchSurface` object that drives most of `App.tsx`'s "Developer" tab (build/channel/platform status, diagnostics, support-debt panel, breadth-claim audit). Subdirectories: `diagnostics/` (evidence-list builder), `status/` (workbench status composer), `update/` (an older `deriveSd11UpdateAction.ts` action-derivation model, superseded in the UI by `sd16/update`), and `feedback/` — the governed bug-report and enhancement-request composers (`bug/composeBugReport.ts` + `bug/submitBugReport.ts`, `enhancement/composeEnhancementRequest.ts` + `enhancement/submitEnhancementRequest.ts`) plus the shared `evidence/` substrate (`captureFeedbackEvidence.ts`, `redaction.ts`, `evidenceFields.ts`, `sanitizeReportableOutput.ts`) — detailed in [update-and-feedback.md](./update-and-feedback.md).

**`sd15/`** — `buildSd15OperatorTriageDraft.ts`, a single pure draft-builder module (plus its test) for operator-facing triage document composition; no UI component in this directory.

**`sd16/`** — the real self-update chain (`update/`) and the browser-handoff half of feedback submission (`feedback/`), both covered in detail in [update-and-feedback.md](./update-and-feedback.md).

**`sd21/`, `sd22/`** — tests-only directories: each contains only `*.test.ts` files, with no corresponding implementation module under `src/`. `sd21/` holds `buildVersionTriple.test.ts` and `releaseClosureChecklistDoc.test.ts`; `sd22/` holds its own re-anchored copies of both (retargeted from tranche/4 to tranche/5) plus `buildLabelFixtureFreshness.test.ts`. These exercise release-governance logic that lives elsewhere in the repo (release-pipeline tooling) rather than shipping their own desktop-app runtime surface — see [release-pipeline.md](./release-pipeline.md) and [testing.md](./testing.md).

**`settings/`** — the settings modal and its panels. `SettingsModal.tsx` defines `SettingsTab = 'appearance' | 'google-drive' | 'update' | 'bug' | 'enhancement' | 'developer'` (verified) and renders whichever panel `App.tsx` supplies per tab. `AppearancePanel.tsx` / `themeMode.ts` / `communityTheme.ts` / `ThemeBrowserModal.tsx` / `obsidianThemeCatalog.ts` handle theme selection and community-theme install, all localStorage-backed. `GoogleDrivePanel.tsx` / `googleDrive.ts` manage a locally-stored Drive-folder-path config — there is no real Google OAuth or Drive API integration; "Drive folder" means a plain local path (typically a synced folder from Drive/Dropbox/Syncthing desktop clients), documented explicitly in `campaign_drive.rs`'s module doc comment. `FriendsSection.tsx` / `friends.ts` is a localStorage-backed friends list.

**`testSupport/`** — shared test fixtures: `makeSurface.ts` (canonical `Sd11TesterWorkbenchSurface` fixture, the single source of truth so drift between test files can't reintroduce silently-broken submit-flow tests), `makeCharacterSummary.ts`, `asserts.ts`.

**`boundary/`** — the Tauri IPC wrapper layer described above.

## The surface/runtime DI pattern

Nearly every screen follows the same dependency-injection shape: a pure `build*Surface` (or `compose*`) function that has no I/O and is fully unit-testable, paired with a `*Runtime.ts` module that supplies the real boundary loader when a Tauri runtime is present and a hardcoded browser-preview fallback otherwise. Screens call only the `*Runtime` function; they never import `invoke()` or a boundary file directly.

**Worked example — character creation, traced through real files:**

1. **Compose the request** — `apps/desktop/src/characterHub/composeCreateCharacterRequest.ts`'s `composeCreateCharacterRequest(fields, deps)` is a pure function; `deps.generateId` / `deps.now` are injected so callers (and tests) control identity and timestamp generation instead of the module reaching for `crypto.randomUUID()` / `Date` directly. It returns a `CreateCharacterRequest` shaped exactly like the boundary's wire type.
2. **Runtime dispatch** — `apps/desktop/src/characterHub/characterHubRuntime.ts`'s `createCharacterRuntime(request)` calls `boundary/loadCreateCharacter.ts`'s `loadCreateCharacter(request)`, then maps the result through the pure `buildCreateCharacterOutcomeSurface` mapper.
3. **Boundary wrapper** — `boundary/loadCreateCharacter.ts`'s `loadCreateCharacter()` guards `hasTauriRuntime()` (throwing if absent — character creation has no browser-preview fallback, unlike the list/detail loaders), then calls `invoke<CreateCharacterOutcome>('create_character', { request })`.
4. **Rust command** — `apps/desktop/src-tauri/src/character_hub.rs`'s `create_character` composes a `CharacterInput` via `compose_character_input` (fixed feat/skill/equipment loadout; only race/class/level/ability scores are the caller's real choices), calls the real engine's `build_pilot_headless_receipt`, and — only if `receipt.status == HeadlessReceiptStatus::Computed` — persists through `SavedCharacterStore::save` and returns `CreateCharacterResponse::Saved`. Any other receipt status returns `CreateCharacterResponse::Blocked { diagnostics }` and nothing is written to disk.
5. **Real core compute** — the receipt comes from `codex::rules_core::pilot_compute::build_pilot_headless_receipt`, the same root-crate compute seam used everywhere else in the app; the desktop shell adds no rules logic of its own.

The catalog screens (`apps/desktop/src/classCatalog/classCatalogRuntime.ts` etc.) follow the identical shape but with a fallback: `loadClassCatalogRuntime()` returns a small hardcoded `buildPreviewCatalog()` array when `!hasTauriRuntime()`, letting the screen render in the Vite browser preview without ever touching Tauri.

## Character flow

**Landing** (`apps/desktop/src/characterHub/LandingScreen.tsx`) offers action banners wired to `CharacterHubPage`'s mode setters: New Character, Load Character, Campaign Manager (gated by `campaignManagerAccessGate.ts`), DM Toolkit (a `StubScreen.tsx` placeholder — "not built yet"), and the four catalog browsers.

**Create** (`apps/desktop/src/characterHub/CreateCharacterForm.tsx`) drives the DI chain above: `composeCreateCharacterRequest` → `createCharacterRuntime` → boundary `loadCreateCharacter` → Rust `create_character` → real core compute. `apps/desktop/src/characterHub/characterHubModel.ts`'s `CLASS_OPTIONS` array records each class's `supportLevel` (`'full' | 'partial-human-only' | 'none'`) — verified against `character_hub.rs`'s own test suite (`compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3`, which iterates every curated race across levels 1–3, and `claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class`, which pins the per-class/race diagnostic sets): **only single-class Fighter at levels 1–3 reaches `Computed` for any race**; every other class/level combination returns real claim-blocking diagnostics from the engine, verbatim (not fabricated by the shell). Wizard level 1 is closer than most (only two named diagnostics remain, both about spellbook/school-power gaps) but still does not reach `Computed` without a populated, in-budget spellbook.

**Sheet** (`apps/desktop/src/characterHub/CharacterSheet.tsx`) renders a Pathbuilder-style three-column layout consuming `LoadSavedCharacterResponse`'s `PilotSnapshotDto` (ability modifiers, BAB, saves, baseline AC, selected skill modifiers — all real engine output) and `CorpusDerivedDto` (spell-school and equipment reachability against a small bundled corpus fixture, `apps/desktop/src-tauri/src/sd19_corpus.rs` — not the full PCGen corpus). `apps/desktop/src/characterHub/characterProgression.ts` supplies level-benefit/next-level entries, HP, caster level, and weapon proficiency from the character's held-class string; `apps/desktop/src/characterHub/skillsModel.ts` supplies the skill list, class-skill lookup, and point-cost math.

**What persists vs. what is session-local/in-memory (verified precisely):**

- Real, persisted: the `CharacterInput` (race, classes, ability scores, feats, skills, equipment, spells) written by `create_character`/`clone_character` via `SavedCharacterStore`; the portrait PNG via `save_character_portrait`.
- **`apps/desktop/src/characterHub/LevelUpDialog.tsx`** — accepting a level-up calls `props.onAccept(classId)`; in `apps/desktop/src/characterHub/CharacterSheet.tsx` the `onAccept` handler body is a comment-documented no-op: *"Backend wiring (persisting the new level onto the saved character) is a separate follow-on — accepting is a no-op today."* Nothing is written to disk and nothing is recomputed.
- **`apps/desktop/src/characterHub/SkillAllocationDialog.tsx`** — its own file-header comment states plainly: *"Accepting only updates in-memory state (`onAccept`) — there is no backend [persistence]."* `apps/desktop/src/characterHub/CharacterSheet.tsx` wires `onAccept={setSkillAllocation}`, a plain React `useState` setter; the allocation is lost on sheet close/reopen.
- **Bio fields** (`DetailsPanel` in `apps/desktop/src/characterHub/CharacterSheet.tsx`: alignment, deity, sex, age, height, weight, hair, eyes) — explicitly documented in-component as session-local: *"edits here only last for this session"*; there is no persisted schema slot for them yet.
- Vision and Size in the same panel are derived read-only from race data (`RACE_OPTIONS` in `apps/desktop/src/characterHub/characterHubModel.ts`), not stored.
- **The sheet's `☰ Menu` dropdown** — `apps/desktop/src/characterHub/CharacterSheet.tsx`'s `menuItems` array wires `Open`, `Save`, and `Clone` to `onSelect: () => {}` (verified verbatim: empty no-op closures); only `Print` does anything real, calling `window.print()`. None of the three named actions currently reach `create_character`/`clone_character`/`load_saved_character` from this menu.

`apps/desktop/src/characterHub/abilityScoreMethods.ts` defines the ability-score generation methods the Create Character form can offer (`manual`, `pool`, `straight`, `pointBuy`, plus dice-roll variants); its own doc comment states `manual` (free-typed numbers) is "today's only behavior, kept as the default" — the other methods are modeled but not yet wired into the form's rendering path. `apps/desktop/src/characterHub/portraitImageProcessing.ts` does client-side portrait prep entirely in the browser (canvas-based center-crop + resize to a fixed `256`px square PNG, animated GIFs flattened to their first frame) before any bytes cross the Tauri boundary to `save_character_portrait` — there is no image-processing crate on the Rust side.

## State approach

There is no state-management library (no Redux/Zustand/etc.) anywhere in `apps/desktop/src/`. Screen-level state is local `useState` mode unions (`CharacterHubPage`'s `Mode`, `App.tsx`'s `settingsTab`/`themeMode`/`surface`).

**localStorage usage** (verified by grepping every `localStorage.` call site in `apps/desktop/src`):

- `apps/desktop/src/campaign/campaignModel.ts` — campaigns themselves (`codex.campaigns` key) and per-campaign markdown assets (`codex.campaign.assets.<id>` key prefix) are the actual source of truth. The module's own doc comment states this plainly: campaign data "works entirely from localStorage." `write_campaign_drive_artifacts` is called by `syncCampaignDriveArtifacts` as a fire-and-forget write-through mirror to a local "Drive folder" path — if that write fails (no folder configured, disk error), the campaign still exists and works entirely from localStorage. The Rust-side `drive_list_campaigns`/`drive_load_campaign`/`drive_save_campaign`/`drive_delete_campaign` commands that would make the Drive folder itself the source of truth are registered and tested but unreferenced by any frontend code (see the command inventory note above).
- `apps/desktop/src/settings/friends.ts` — a friends list, same persistence pattern as `campaignModel.ts` (its own doc comment says so).
- `apps/desktop/src/settings/communityTheme.ts` — installed community theme CSS and the active-theme id.
- `apps/desktop/src/settings/googleDrive.ts` — the locally-configured Drive-folder-path config (no OAuth token; there is no OAuth flow).
- `apps/desktop/src/settings/themeMode.ts` — the light/dark/system theme mode, so it survives reloads.

## Testing hooks

The `makeSurface` pattern (`apps/desktop/src/testSupport/makeSurface.ts`) is the canonical fixture-building convention this app uses for its larger DI surfaces: a single exported factory function returning a complete, valid object with shallow-spreadable `overrides`, so every test consumes the same shape and a schema change only needs updating in one place. The same shape recurs across the surface/runtime pattern described above — `build*Surface` functions are the production analog of `make*` test fixtures. See [testing.md](./testing.md) for the wider test-running convention (`npm test` → `apps/desktop/scripts/run-tests.mjs`) and how these fixtures are used across the suite.

## See also

- [update-and-feedback.md](./update-and-feedback.md) — the self-update chain and the feedback/defect-report submission chain in full detail.
- [release-pipeline.md](./release-pipeline.md) — how the channel index and update manifest this app fetches are published.
- [rules-engine.md](./rules-engine.md) — the root `codex` crate's compute engine this app's `character_hub.rs` calls into.
- [persistence.md](./persistence.md) — `SavedCharacterStore` / `CampaignStore` on-disk formats.
- [testing.md](./testing.md) — the desktop app's test runner and fixture conventions.
- [conventions.md](./conventions.md) — cross-cutting idioms (DI seams, honest-degradation wording, command/pure-fn split).
- [status.md](./status.md) — current capability/stub status across the whole repo.
