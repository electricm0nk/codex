---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-16; operator directives 2026-07-17: branch flip tranche/5 → tranche/4-1, board flip codex-tranche-5 → codex-tranche-4-1, scope trim: APG + ACG + advanced guides moved to SD-22, renumber Epic 1 to Identifier Cleanup, Epic 2-7 renumbered; branch/board updates per operator directive; bundle marked approved with operator directives 2026-07-16/17)
date: 2026-07-15
canonical_branch: tranche/4-1 (operator directive 2026-07-17)
kanban_board: codex-tranche-4-1 (operator directive 2026-07-17)
---

# SD-21 — Epic Breakdown

Maps the **30 acceptance criteria** for SD-21 (Code-Side Identifier Cleanup + Campaign Manager + Drive persistence + Update UI bug remediation + Closure Epilogue + Build Version Numbering + Single-class Coverage Completion + Multiclass Stacking) into **7 epics** inside the SD-21 bundle. Each epic has its own acceptance criteria; each epic lands via the same loop-routed-cycle pattern SD-19 used.

The 4 APG/ACG criteria and the 4 advanced-guide-criteria that previously sat in SD-21's scope have moved to `../SD-22/` (per operator directive 2026-07-17; per the spec-domain lifecycle routing rule). SD-21 is now scoped to: campaign manager + Drive persistence (CRB-only data; APG/ACG/advanced-guide content is SD-22's lane), the identifier-discipline base requirement, the Update UI bug fix, the closure epilogue (standard part-of-handoff for any SD-N), the build version numbering patch, and the rules-engine multiclass support.

## Execution lane split

- Epic 1: Code-Side Identifier Cleanup        (governance base requirement; fires FIRST)
- Epic 2: Campaign manager + Drive persistence    (gates 2, 3, 4, 8, 9)
- Epic 3: Update UI bug remediation            (post-tranche-3 defect remediation; code references only)
- Epic 4: Closure Epilogue                    (final scan, PR, worktree cleanup, release notes, version increment — fires LAST)
- Epic 5: Build Version Numbering             (`<major>.<tranche-base>.<build>` three-position scheme + build-label format — fires before Epic 4)
- Epic 6: Single-class coverage completion    (Wizard-first, multiclass prerequisite; rules-engine core)
- Epic 7: Multiclass stacking                 (BAB/save/feature stacking; depends on Epic 6)

Total: **30 acceptance criteria grouped into 7 epics + 2 promotion gates** (gate 1 is prereq; gate 10 is promotion). Epic 1 (Code-Side Identifier Cleanup) is listed first in the cycle-priority order below because Epic 3's `apps/desktop/src/sd16/update/controllerAdapter.ts` work depends on Epic 1 having removed the `sd16_*` identifiers that Epic 1's criteria 1-4 rename. Epic 1 does not interfere with Epic 2's files. Epic 4 (Closure Epilogue) is listed LAST in the cycle-priority order (it scans every prior criterion before opening the final PR). Epic 5 (Build Version Numbering) fires after Epics 1-3 land and before Epic 4's closure worktree-cleanup cycle, so the version bump lands on the same `tranche/4-1` commit-history Epic 4 is sweeping. Epic 6 (Single-class coverage completion) and Epic 7 (Multiclass stacking) are interleaved with Epics 2-3 — Epic 6 must run before Epic 7 (bug handoff's two-phase rationale), and Epic 7 has no dependency on Epic 2's output.

## Linear dependency (per decisions.md)

```
Epic 1 (Code-Side Identifier Cleanup) — must fire first on shared files
└── Epic 2 (Campaign Manager + Drive persistence) — independent of Epic 1, can launch any time after Epic 1 lands
└── Epic 3 (Update UI bug remediation) — independent of Epic 2, can launch any time after Epic 1 lands
└── Epic 4 (Closure Epilogue) — depends on Epics 1, 2, 3, 5, 6, 7 all complete
└── Epic 5 (Build Version Numbering) — depends on Epic 1-3 lands, before Epic 4's closure sweep
└── Epic 6 (Single-class coverage completion, Wizard-first) — independent of Epics 1-3, can launch any time after Epic 1 lands
└── Epic 7 (Multiclass stacking) — depends on Epic 6's single-second-class proof
```

Epic 1 (Code-Side Identifier Cleanup) lands **first**. Epic 2 (Campaign Manager + Drive) can run in parallel with Epic 3 (Update UI bug remediation) IF the operator hosts them concurrently; if not, Epic 2 lands first by default. Neither depends on Epic 6's output. Epic 3's work touches code surfaces originally carved out by closed spec domains and is owned by SD-21 because SD-21 is shaping the next release package. Epic 1's touches to `controllerAdapter.ts` remove the `sd16_*` rename class Epic 3 then doesn't have to fight against. Epic 4 fires LAST; it scans every prior criterion's status before opening the final PR, runs release-note generation, and orchestrates version-increment + worktree cleanup. Epic 5 lands after Epics 1-3 — it bumps the patch version `0.0.X` per tranche and rewires the build label format, both of which need to land before Epic 4's closure epilogue can sweep the versioned commit history. Epic 6 (Wizard-first per bug handoff) lands independent of Epics 2-3 but must run before Epic 7; Epic 7 (Multiclass stacking, BAB/save/feature) follows Epic 6's proof-out of single-second-class correctness.

## Acceptance criteria (30, across 7 epics)

### Epic 1 — Code-Side Identifier Cleanup (governance base requirement)

**Scope doctrine (operational rule):** under the identifier-discipline doctrine (`governance/identifier-discipline.md`), source-code identifiers must describe what the artifact does, not which release or spec domain it came from. Epic 1 is the SD-21 cycle that fires to clean up the load-bearing identifier leaks already in the codebase: Tauri command names with the `sd16_` prefix, TypeScript functions and constants with `Sd16` / `SD16` text, `data-testid` attributes with `sd16-` prefixes, inline doc-comments citing `SD-N-Ex...` identifiers, and any `t_<hex>` kanban tokens / `AV-PAY-5` audit-IDs embedded in source. **Out of scope** for Epic 1 (recorded explicitly to prevent scope creep): directory tree renames (`apps/desktop/src/sd16/` → `apps/desktop/src/update/`, `apps/desktop/src-tauri/src/sd19_*.rs` → descriptive file names) — those are Epic 4 follow-on work because directory rename churns every relative import, every release-channel JSON, and every electron-vite config.

**Cycle order: Epic 1 fires before Epic 3.** Both touch `apps/desktop/src/sd16/update/controllerAdapter.ts`. Landing Epic 1 first means Epic 3's bug-fix work builds on already-renamed identifiers and never has to fight the `sd16_*` style during its own cycles. Epic 1 has no impact on Epic 2's files.

1. **Rust Tauri command names renamed**: every Tauri command currently prefixed with `sd16_` or `sd19_` (e.g. `sd16_browser_handoff`, `sd19_spell_catalog`, `sd19_race_catalog`, `sd19_equipment_catalog`, `sd19_class_catalog`) is renamed to a descriptive PascalCase / snake_case name. The rename covers three surfaces that must move together: the Rust function definition in `apps/desktop/src-tauri/src/sd*_*.rs`, the JS invoke-string in callers (e.g. `apps/desktop/src/sd16/feedback/browserHandoff.ts:114` calls `'sd16_browser_handoff'` — that string literal changes), and the test-assertion strings in `*.test.ts` files. The Rust file itself stays named `sd*_*.rs` until a follow-on epic (the rename churns every relative import, every release-channel JSON, and every electron-vite config — release-package scale). At the conclusion of this criterion, `grep -rE "sd16_|SD16_|sd19_|SD19_" apps/desktop/` returns zero hits in identifier-or-string-literal positions; matches only in `// Tranche-3-chassis`-style prose comments are addressed by criterion 3.

2. **TypeScript function and constant names renamed**: every TypeScript function and constant currently carrying an `Sd16` / `SD16_` / `sd16-` identifier (e.g. `createSd16UpdateControllerDeps`, `loadSd16MountTimeState`, `SD16_UI_BUTTON_BASE_STYLE`, `SD16_UPDATE_UI_ID`) is renamed to descriptive PascalCase. The rename covers the function definition, every call site, and every test assertion. Constants follow PascalCase per the operator's base convention (`UpdateRestoreButtonBaseStyle`, `UpdateUiContainerId`). At the conclusion of this criterion, `grep -rE "Sd16[A-Z]|SD16_" apps/desktop/src/` returns zero hits.

3. **Inline doc-comments and `data-testid` attributes cleaned**: every inline source-code comment that says `// SD-N-Ex...`, `// See SD-N for...`, `// Tranche N chassis lane`, `// Audit ID AV-PAY-N`, or similar (e.g. `controllerAdapter.ts:1-18` module-header mentions of `is_install_eligible` and `perform_install` as "deferred slices"; `Ui.tsx:44` reference to "SD-16-E6-F3c page-level entry"; `Ui.tsx:112` "SD-16-E7" comment) is rewritten as plain-prose: a description of what the artifact does, with optional doctrinal cross-reference *to a file path*, but with no SD-N or audit identifier in the comment text itself. `data-testid="sd16-..."` attributes become `data-testid="update-..."` and similar. At the conclusion of this criterion, `grep -rnE "SD-[0-9]+-[A-Z][0-9]|Tranche [0-9]+ chassis lane|AV-PAY-[0-9]+" apps/desktop/src/ apps/desktop/src-tauri/src/` returns zero hits.

4. **Per-cycle tests pass after every rename**: each renamed identifier gets a follow-up test cycle that exercises the new name (function call, invoke call, data-testid query) and asserts the new behavior. Tests that previously asserted `cmd === 'sd16_browser_handoff'` now assert `cmd === 'defect_submission_browser_handoff'`. Tests that previously queried `data-testid="sd16-restore-previous-button"` now query `data-testid="update-restore-previous-button"`. CI runs green on `tranche/4-1` after each rename; no regression on existing functionality.

**Out of scope for Epic 1 (recorded explicitly to prevent scope creep):**

- **Directory tree renames** (`apps/desktop/src/sd16/` → `apps/desktop/src/update/`, `apps/desktop/src-tauri/src/sd{16,19}_*.rs` → descriptive file names). Follow-on epic; the rename churns every relative import, release-channel JSON, and electron-vite config — release-package scale.
- **Audit-trail rename mapping** (e.g. a `docs/SD-21/identifier-renames.md` recording old → new). Git log carries that audit trail; no separate audit file unless governance author wants one.
- **Auto-rename tooling.** The renames are hand-driven per cycle so the operator can audit each rename. No codemod unless Epic 1's scope creeps to a point where hand-driven becomes unworkable.

### Epic 2 — Campaign Manager + Drive persistence

5. **`CampaignSnapshot` type is defined** in `src/rules_core/campaign.rs` per `technical-design.md` §1.1. Includes `CampaignMetadata`, `Party`, `PartyMember`, `CharacterSummary`, `PartyResources`, `AdventureLogEntry`, `MapRef`, `WikiPage`. JSON-serializable.

6. **`CampaignBackend` trait is defined** in `src/rules_core/persistence/mod.rs` with the four ops: `load_campaign`, `save_campaign`, `list_campaigns`, `create_campaign`, plus `delete_campaign` and `snapshot_known_format`. Engine calls into the trait object; never imports backend-specific types.

7. **The Drive adapter implements `CampaignBackend`** in `src/rules_core/persistence/drive.rs`. OAuth flow per `technical-design.md` §2.1 (Google OAuth 2.0 authorization-code, file-scoped, OS-keyring-backed token storage). Drive v3 API client (or v3 equivalent) communicates with `campaign_root_folder_id`. The adapter is the only thing in the codebase that imports Drive types.

8. **Tauri commands expose the campaign manager** as `drive_authorize`, `drive_pick_folder`, `drive_list_campaigns`, `drive_load_campaign`, `drive_save_campaign`, `drive_delete_campaign`. The GUI's vibe-coded campaign-manager screens (PR #316) consume these via `apps/desktop/src/boundary/campaignDrive.ts`.

9. **Markdown file layout per `technical-design.md` §2.2**. Each `CampaignSnapshot` field lands in a named markdown file under the campaign's Drive folder. YAML frontmatter carries structured fields; markdown body carries human-authored content. The layout round-trips through Obsidian (per `acceptance-and-verification.md` gate 9).

10. **Conflict-detection on load**: when `CampaignSnapshot.nonce` on disk differs from the local nonce, the adapter saves both copies to `conflicts/<timestamp>/`, surfaces a conflict-detected affordance. The DM resolves manually (per `decisions.md` §7).

11. **First-run Drive authorization surfaces in the GUI's campaign-manager landing page**. Per PR #316's vibe-coded campaigns screen.

**Out of scope for Epic 2 (recorded explicitly to prevent scope creep):**

- *APG/ACG/advanced-guide content ingestion.* Those now live in SD-22 (per operator directive 2026-07-17). Epic 2 reads from SD-19's `rules_tables/crb/` only.
- *Character sheet shape beyond `CharacterSummary`.* The campaign manager cares about character *names*, not character *spell effects* or *damage rolls* (those arrive after SD-20 closes via `risks-and-open-questions.md` Q4 auto-upgrade).
- *DM-toolkit GUI screens.* That's SD-22's lane (encounter builder, party-CR math surfaces).

### Epic 3 — Update UI bug remediation

**Scope doctrine (operational rule):** a defect against code originally carved out by a closed spec domain is owned by the bundle currently shaping the next release package — see `governance/spec-domain-lifecycle.md`. Epic 3 is SD-21's bundle of bug fixes for the next release package; it cites affected code by file:line rather than re-opening the originating bundle's doctrine.

12. **Release-notes fetch path lands**: a fetch path is added to `apps/desktop/src/sd16/update/fetch.ts` (the GitHub Releases API per version, or an extension of the existing channel-index / update-manifest fetches) that retrieves per-version release-notes body content. The fetched value is assigned to `deps.releaseNotes` by `runCheck` (the load-bearing change; today the success path leaves `deps.releaseNotes` as `null` per `apps/desktop/src/sd16/update/controllerAdapter.ts:142` and sets `releaseNotesStatus = 'unavailable'` per `:240`). The existing `Sd16ReleaseNotes` type (`apps/desktop/src/sd16/update/updateModel.ts:108-111`) is reused without schema work. After a successful Check, `Sd16CheckPanel.renderReleaseNotes` (`apps/desktop/src/sd16/update/CheckPanel.tsx:100-118`) renders the body rather than the empty-state placeholder. On fetch failure (network, schema mismatch, missing field) the placeholder remains rendered — no regression on the existing empty-state branch.

13. **Local installed-state probe lands**: a new Rust backend Tauri command `is_install_eligible(…)` (plus its JS bridge binding) is added under `apps/desktop/src-tauri/src/` to probe local installed state (install kind, writability, current version). Its companion `perform_install(plan) → InstallResult` (named in `controllerAdapter.ts:1-18` module header as the deferred pair) is also added so the install side has the corresponding backend surface. `controllerAdapter.ts:loadSd16MountTimeState` (`:280-332`) populates `Sd16InstalledState.{updateEligible, ineligibleReason}` from the real probe rather than from stubbed values.

14. **`computeDecision` success path is rewired**: `controllerAdapter.ts:computeDecision` (`:150-178`) replaces the success-path short-circuit (`:157-163`, returning `{ result: 'unknown', reason: LOCAL_STATE_UNAVAILABLE_REASON }` at `:162-163`) with a real call into the already-correct `decideEligibility` (`eligibility.ts:81-127`). The eligibility-card surface then renders the actual decision rather than the "Unknown" placeholder. The existing fetch-failure branch (`:164-166`) is preserved — no regression on that branch.

15. **Per-cycle tests cover each outcome**: per-fetch unit tests at `fetch.test.ts` + `controllerAdapter.test.ts` cover success-body-renders, fetch-failure-no-regression, schema-mismatch-tolerated; per-probe tests at `controllerAdapter.test.ts` cover the Tauri-binding invocation and each eligibility reason (eligible, not-in-install-kind, not-writable, and the other named reasons in `eligibility.ts:81-127`); `eligibility.test.ts` continues to cover the decision-table shape unchanged. Run-check method bounds for the load-bearing change: `runCheck` lives at `controllerAdapter.ts:196-244` (a method inside `deps.controller` returned by `createSd16UpdateControllerDeps` at `:133-259`); the `releaseNotesStatus = 'unavailable'` and any new `releaseNotes` assignment both go inside that method body.

**Out of scope for Epic 3 (recorded explicitly to prevent scope creep):**

- Any change to the spec-domain lifecycle doctrine itself.
- Re-opening any closed spec domain (`SD-16-…` and earlier are closed; the bug fix belongs to SD-21 by lifecycle routing, not by tag-inheritance).
- Drive persistence or campaign-manager bug fixes (those would be a different epic if any surface; the current Epic 3 is scoped to the Update-tab release-notes-and-eligibility bug alone).
- Identifier-cleanup follow-on (Epic 1 has the rename work).

### Epic 4 — Closure Epilogue (final scan + PR + worktree cleanup + release notes + version increment)

**Scope doctrine (operational rule):** per the spec-domain lifecycle doctrine (`governance/spec-domain-lifecycle.md`), every closed bundle gets a final-cycle epilogue that scans all acceptance criteria, opens the develop-merge PR, cleans up worktrees and stale branches, generates release notes, and increments the version number. This is the standard part-of-handoff-doctrine for SD-N closures going forward; SD-21's Epic 4 is the first worked example.

19. **Final criterion scan**: at the start of Epic 4's cycle, the loop walks every row of the SD-21 progress matrix (`./progress.md` §"SD-21 cycles") and asserts every criterion (1-30) is `Status: complete` OR has an `## Open blockers` entry documenting a real blocker. If any criterion is `pending` or `running` without a blocker, Epic 4's cycle routes around it: the loop surfaces the criterion to the operator via a kanban card on `codex-tranche-4-1` and pauses for operator decision (per the loop's hard-stop semantics; not a counterfeit-completion).

20. **Open the closure PR**: once the scan passes, the loop opens the `tranche/4-1 → develop` promotion PR via `gh pr create`. The PR's description references all 30 acceptance criteria, the cycle-merge receipt SHAs, and a one-line summary of which epics landed which criteria. The PR's body also includes a release-notes preview (see criterion 19). The closure-flow PR is opened alongside the doc-control-plane PR that flips closure-state frontmatter (per `governance/spec-domain-lifecycle.md` Plan A).

21. **Worktree cleanup and stale-branch sweep**: the loop calls `git worktree list` and identifies any worktrees whose `branch` is not `tranche/4-1`, `develop`, or `main`; those worktrees get removed (`git worktree remove --force`). Stale branches (branches fully merged into `tranche/4-1` or `develop` that have not been active in the last 30 days) get deleted (`git branch -d`). A summary of removed worktrees and branches lands in the closure PR's description.

22. **Generate release notes**: the loop runs a small generator that produces `programs/codex/requirements/SD-21-campaign-manager-and-persistence/release-notes.md` with sections: "New features" (from Epic 2 — campaign manager + Drive persistence), "Bug fixes" (from Epic 3 — Update UI remediation), "Maintenance" (from Epic 1 — identifier cleanup), "Versioning" (from Epic 5 — `<major>.<tranche-base>.<build>` bump). The release notes are a generated artifact (not prose-by-hand) and committed alongside the closure PR.

23. **Increment the version** *(tranche promotion only)*: SD-21's release is on `tranche/4-1` (a dash release off the Tranche-4 base). **Epic 4's job is the *tranche* position of the `<major>.<tranche-base>.<build>` triple** (i.e. `4` in `0.4.<build>`) plus, on tranche promotion, resetting the build counter to 0 (e.g. `0.4.<last_build>` → `0.5.0`). Reads the current triple from the three version files (Epic 5's locks the values), increments the **tranche** position by 1, resets the build position to `0`, and writes back to `apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json` + `apps/desktop/src-tauri/Cargo.toml`. Cargo.lock's embedded version updates on the next `cargo check`. **Net for this SD release: Epic 5 lands first (sets the version to `0.4.<current_build>` — e.g. `0.4.93` — the build counter increments on every merge, the tranche stays at `4` because we're still in Tranche 4 work, the major stays at `0` until first main-publish); Epic 4 then runs the closure PR with the version already committed.** *The concrete bump to `0.4.<current_build>` is Epic 5 (Build Version Numbering); Epic 4 (Closure Epilogue) owns the per-tranche-promotion bump shape (the next time SD-22 launches on `tranche/5`, Epic 4's equivalent bumps to `0.5.0`).*

24. **Per-cycle tests pass at closure**: full `cargo test --locked` (zero regressions), `cargo clippy --locked --tests -- -D warnings` (clean), and the SD-21 acceptance gate suite (all 30 criteria at `complete`). The loop runs the entire test suite as the closure gate; any failure routes back into Epic 4's "Open blockers" entry with the test name + commit SHA.

**Out of scope for Epic 4 (recorded explicitly to prevent scope creep):**

- The actual *value* of the version bump. Epic 4 owns the increment logic (e.g. `0.4.<current_build>` → `0.5.0` on tranche promotion); the *content* (`0.4.<current_build>` per Epic 5's first-cycle bump) is Epic 5.
- Worktree and branch cleanup for branches outside the SD-21 lane (Tranche-3 chassis lane, sibling SD-20 work). Those branches are governed by their own bundles.
- Auto-merging the closure PR. Operator-driven only (per `decisions.md` §6 no-branches convention).

### Epic 5 — Build Version Numbering (`<major>.<tranche-base>.<build>` + build-label format)

**Scope doctrine (operational rule):** the displayed build version is wired to three files (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`) and a single build-label format (`${BUILD_PREFIX} ${buildVersion}`) at `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts:72-74`. The version scheme is a three-position triple **`major.tranche-base.build`** with the following rules (per operator directive 2026-07-17; replacing the prior `0.0.X` patch-only scheme):

- **`major`** (first number; `0` until first main-publish). Increments by `1` per merge to `main` (the publish surface). Stays at `0` until the first publish to main; a future repo that ships to main for the first time might jump to `1.x.y`.
- **`tranche-base`** (second number; the base tranche digit of the active working branch). Increments *slowly*, only when the active branch is promoted off the prior base tranche. Concretely: `tranche/4-1` carries `4`; `tranche/5` carries `5`; `tranche/4-2` (a hypothetical future dash from 4) also carries `4`. The number is the **base** of the active tranche, not an increment counter.
- **`build`** (third number; per-build counter). **Monotonic across all builds across all branches — never resets.** Increments by `1` on every merge to `tranche/<N>` (or any working branch). The build counter accumulates toward infinity over the lifetime of the project: `0` → `92` → `93` → `100` → `200` → `500` → ... → `1000` → ... → `∞`.

The patch-version scheme (prior `0.0.X`) was deemed a bad call by the operator; this three-position scheme replaces it because it (a) makes the displayed build version carry operational meaning — viewers can tell at a glance which tranche built the artifact and how many builds have happened since — and (b) gives the build a clear ordinal position relative to previous builds.

The first concrete SD-21 release value (operator-pinned) is **`0.4.<current_build>`** — e.g. **`0.4.93`** if the build counter is currently at `92`. After Epic 4's closure PR lands, the version is `0.4.<last_build>`. When `tranche/5` launches (the next spec domain), Epic 4's closure bump (à la criterion 23's tranche-promotion logic) advances the version to `0.5.0` (the build counter starts at `0` again for the new tranche). Future SD-22 builds continue from `0.5.<N+>`. Any future publish to `main` increments `major` to `1` and resets `tranche` and `build` to `0`.

Epic 5 lands **before Epic 4** so the version commit is in the closure PR's history. Epic 5's three criteria cover the *concrete* version bump (criterion 25), the *build-label format* (criterion 26), and the *bump process* (criterion 27).

25. **Version fields set to `0.4.<current_build>`**: `apps/desktop/package.json`'s `"version"` field, `apps/desktop/src-tauri/tauri.conf.json`'s `"version"` field, and `apps/desktop/src-tauri/Cargo.toml`'s `version =` line are all set to `"0.4.<current_build>"`, where `<current_build>` is the next build number after the last committed build on `tranche/4-1` (per the operator's prior session note, the last build was `0.0.92`; under the new scheme the last build is *checkable via git log + the prior-session receipt comment chain* — if not retrievable, the operator pins a value at SD-21 cycle launch). Cargo.lock's embedded copy of the version updates automatically on the next `cargo check`; no manual edit needed. **The tranche position stays `4`** because `tranche/4-1` is a dash release off the Tranche-4 base; the operator does NOT increment `tranche` until `tranche/5` actually launches. The `major` position stays `0` until the first publish to `main`.

26. **Build-label format updated**: `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts:61` changes `BUILD_PREFIX = 'codex'` to `BUILD_PREFIX = 'Codex'`; `createSd11WorkbenchStatus.ts:72-74` changes the template from `${BUILD_PREFIX}@${buildVersion}` to `${BUILD_PREFIX} ${buildVersion}` (drop the `@`, add a space). `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`, `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`, and `apps/desktop/src/testSupport/makeSurface.ts` are updated to assert/fixture the new `Codex 0.4.<build>` shape rather than the old `codex@0.0.0-test` shape. The change is presentation-only: every consumer of `buildLabel` (verified at `apps/desktop/src/sd11/`, `apps/desktop/src/boundary/loadSd11UpdateAction.ts`, `apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`, and `apps/desktop/src/sd15/`) treats the value as an opaque display string; no parsing, no `split on @`, no pattern-matching against release tags. Verified low-risk by the operator's bug handoff.

27. **Bump process established (checklist or CI gate)**: a new `docs/release/SD-21/release-closure-checklist.md` (relocated 2026-07-20 from a stray `docs/SD-21/` path) records the four-step process that future closure epilogues (Epic-4-equivalents in any future SD) follow: (1) bump version in all three files using the `<major>.<tranche-base>.<build>` triple (with the relevant position incremented per the current operation — every-CI-build increments build; every-tranche-promotion increments tranche and resets build; every-main-publish increments major and resets both), (2) run build-label format check (optional: a CI step that diffs the format string against an expected template), (3) run cargo check to refresh Cargo.lock, (4) commit as `feat(sd21): bump version to <major>.<tranche>.<build>` per the operator's PR-message convention. The checklist is part of the standard handoff going forward (per the operator directive 2026-07-17).

**Out of scope for Epic 5 (recorded explicitly to prevent scope creep):**

- Major-version or `major != 0` first-publish logic. The first publish to `main` is a separate epic; this SD ships with `major = 0`.
- Automated build-counter increment as part of every CI commit. Epic 5 establishes the *value* and the *format*; the CI integration is a separate concern (the bump-counter scripting is a future bundle's epic — not in SD-21).
- Build-label parsing anywhere in the codebase. The format is presentation-only.
- Tranche-promotion automation. Epic 4's generic version-increment owns the per-tranche-promotion case (bumping `tranche` and resetting `build` to `0`); Epic 5 owns the format. Per-CI-build increment is operator-pinned at SD-21 cycle launch, not in Epic 5's scope.

### Epic 6 — Single-class coverage completion (Wizard-first; rules-engine core)

**Scope doctrine (operational rule):** the bug handoff identifies a structural gate in the rules-engine core: only single-class Fighter characters can reach a Computed result. Every other single class and any multiclass combination is permanently blocked because `compute_pilot_base_chassis` (per the bug handoff at `src/rules_core/pilot_compute.rs:4568`) calls exactly one function for base attack bonus + saving throws (`compute_fighter_chassis`), which only matches class_levels vectors of length 1 with class == Fighter. Per `governance/spec-domain-lifecycle.md`, the bug's originating release is Tranche-3 chassis substrate by SD-18 (closed); the bug routes to the bundle currently shaping the next release — SD-21. **Epic 6 lands Wizard-first** (per bug handoff's recommendation, given `supported_wizard_level` already exists at `pilot_compute.rs:13967` capped at level 11). Cleric/Sorcerer/etc. follow as operator-pinned 6b/6c/...epics.

25. **`compute_pilot_base_chassis` dispatches by class, not by Fighter-only.** `pilot_compute.rs:4568` is refactored from a direct `compute_fighter_chassis(input, ...)` call into a `compute_X_chassis(input, ...) for X in [Fighter, Wizard, ...]` dispatch — a match on `class_levels[0].class_id` (length-1 single-class input) that produces the same base_attack_bonus + base_saves as today's `compute_fighter_chassis` for Fighter, and calls `compute_wizard_chassis` (with its existing partial implementation) for Wizard class. The dispatch returns `None` for any multiclass mix (deferred to Epic 7) and surfaces a `class_chassis.unsupported` diagnostic. Per-cycle: a single-class Human Wizard 3 (the bug handoff's reproducer) reaches `Status: Blocked` → `Status: Computed`, with only the partial-grounding class_spell.wizard.prepared_spellbook.unsupported diagnostic remaining.

26. **`compute_wizard_chassis` brought to full Computed support.** The existing partial implementation at `pilot_compute.rs:13967` is extended from level-cap 11 to full level-cap 20, including the Wizard-specific base chassis (BAB, saves per PF1 Wizard progression), spell slots per level, spellbook setup with prepared-vs-spontaneous distinction, familiar handling (where not already stubbed), Arcane School + Opposition School selection, and class-feature grants (Scribe Scroll, etc.). Each sub-feature lands as a per-cycle acceptance test that exercises the resolved CharacterInput's post-compute state. **One sub-feature per cycle** (BAB progression is one cycle, saves progression is another, spell slots is another, Arcane School is another, Scribe Scroll is another, etc. — six or more cycles for the Wizard extension alone; per-cycle TDD discipline). Operator-pinned: which sub-features land in Epic 6a vs. Epic 6b vs. Epic 6c as the Wizard scope expands.

27. **Per-class foundation module shapes for Wizard (and any subsequent class).** Each class lands as a `compute_<class>_chassis` function in `src/rules_core/pilot_compute.rs` (or in per-class submodules under `src/rules_core/` per the loop-instruction's file-touch partition expansion) plus an `explain_<class>_chassis` decomposition function. Naming follows the identifier-discipline doctrine (`governance/identifier-discipline.md`): descriptive class name, no bundle tag, PascalCase per the operator's `MyPreferredMethodIsPascalCase` convention.

**Out of scope for Epic 6 (recorded explicitly to prevent scope creep):**

- *Multiclass dispatch from `compute_pilot_base_chassis`.* Epic 7's job. Epic 6's dispatch returns `None` for length-1 non-Fighter-and-non-Wizard classes (deferred to Epic 6b/6c/.../epics), and `None` for any length-2+ input. The diagnostic emitted in those paths stays `claim_blocking: true`.
- *Charter module rework.* If Epic 6 reveals PF1-correctness bugs in the per-class functions themselves (e.g. Wizard's spell DC formula), those get filed as separate bugs, not absorbed into Epic 6.
- *PCGen re-ingestion.* The Tranche-3 corpus-source ingestion (`src/pcgen_import/`) is independent. Wizard's spell content is already on disk; Epic 6 just lights up the existing parser output.
- *GUI changes.* The Campaign Manager GUI is outside the bundle per `decisions.md` §6; Epic 6 is engine-only.

### Epic 6b — Wizard full-completion (spellbook + school powers + remaining chassis gates)

**Scope doctrine (operational rule; added 2026-07-19 per operator directive following a fresh closure-gate scan).** Epic 6's landed cycle (`2fe3b9f`) gave Wizard real BAB/saves via `compute_class_chassis`'s per-class dispatch, but a single-class Human Wizard 3 still cannot reach `Status: Computed` — proven by the bundle's own acceptance test (`tests/sd21_wizard_chassis_computes.rs`'s `wizard_level3_still_stays_blocked_on_its_own_spell_burdens`). Three concrete, independently-landable gaps remain, discovered and documented by Epic 6's own cycle rather than invented after the fact:

6b.1. **Extend `compute_combat_baseline` and `compute_selected_skill_modifiers` to the dispatch-supported class set.** Both functions in `pilot_compute.rs` independently gate on `supported_fighter_level(input)` only — a separate, older gate from `compute_pilot_base_chassis`'s own dispatch (already widened for `compute_total_saves` in Epic 6's landed cycle via `has_supported_class_chassis`). Extend both to the same `has_supported_class_chassis` check so Wizard's melee attack bonus, armor class, and skill-modifier cells stop rendering `Blocked`. This is the smallest, most mechanical of the three gaps (mirrors work already done once for `compute_total_saves`) and should land first.

6b.2. **Ground Wizard's prepared spellbook system**, resolving `class_spell.wizard.prepared_spellbook.unsupported`. Real spellbook contents (which spells a Wizard has recorded, sourced from the same corpus data `supported_wizard_level`'s spell-baseline explanation already reads), daily preparation (choosing which known spells are prepared for the day, consuming spell slots by level), and the prepared-vs-spontaneous distinction the original Epic 6 criterion 26 named but didn't ground. Acceptance: a Human Wizard 3 with a chosen spellbook and a daily preparation selection reaches a real (non-placeholder) prepared-spells state; the diagnostic clears.

6b.3. **Ground Wizard's Arcane School selection**, resolving `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported`. Real school-power grants per specialized school (e.g. Evocation's Intense Spells + Force Missile at level 1) and the opposed-school mechanic (choosing 2 opposed schools the Wizard cannot prepare from, with the correct PF1 slot-cost penalty for any opposed-school spell prepared via other means). This is the largest of the three gaps — a real spell-engine feature, not a wiring fix — and may itself split into per-school sub-cycles (Evocation first, as the bug handoff's own reproducer implies, per the existing pattern of shipping one school before generalizing).

**Acceptance reproducer (supersedes Epic 6's original, now-provably-incomplete one):** a single-class Human Wizard 3 with a chosen Evocation specialization, a populated spellbook, and a daily preparation selection reaches `Status: Computed` with zero claim-blocking diagnostics remaining. This is the gate 11 fix: `acceptance-and-verification.md` gate 11 does not move to met until this reproducer passes for real, not until the old (now-known-unreachable) reproducer is retargeted.

**Out of scope for Epic 6b:**

- *Cleric/Sorcerer/etc.* Still Epic 6c/6d/... per operator-pinned future scope; Epic 6b is Wizard-only.
- *Familiar handling.* Named in Epic 6's original criterion 26 but not load-bearing for the `Computed` reproducer (no diagnostic blocks on it); remains a follow-on if the operator wants it modeled.
- *Multiclass interaction with the newly-grounded spellbook/school-power features.* Epic 7 already landed Fighter/Wizard multiclass BAB/save/feature stacking against Epic 6's dispatch; if 6b's spellbook/school-power work changes Wizard's per-class feature surface, reconciling that with Epic 7's multiclass feature-integration is a follow-on, not 6b's job.

### Epic 7 — Multiclass stacking (BAB + saves + features)

**Scope doctrine (operational rule):** with Epic 6's single-class coverage proven correct, Epic 7 extends `compute_pilot_base_chassis` to handle multiclass inputs (class_levels vectors of length 2+). The three mechanics are: (a) **BAB stacking** — sum each class's own BAB progression (PF1's `good: +1/full every level`, `medium: +1/2 per level rounded up`, `poor: +1/3 per level rounded up`). (b) **PF1 best-fractional-progression save stacking** — for each of Fortitude/Reflex/Will, the character's save is the highest class save plus 0.5 (PF1's actual rule for multiclass: `class_save(class_i) + floor(SUM(class_save_bonus(class_j for j in [0..n])) / (number_of_classes))`, with rounding per the player's character level). A naive sum overshoots PF1 and is a rules-engine correctness violation; Epic 7 must apply the best-fractional formula correctly. (c) **Per-class feature integration** — when two+ classes contribute class features (skill points, spellcasting, class features keyed off total vs. class level), Epic 7 must reconcile them without either class clobbering the other.

28. **`compute_pilot_base_chassis` dispatches across all supported classes** (length-1 dispatch from Epic 6 + length-2+ dispatch from Epic 7). Single-class inputs continue to route per Epic 6's match. Multiclass inputs (length-2+ class_levels) route to a new `compute_multiclass_base_chassis` that runs the per-class `compute_<class>_chassis` for each entry, sums their BABs per the per-class progression (full/3-4/half per class), applies PF1's best-fractional-progression save rule for each save, and emits a `multiclass.feature.<class>.<feature>` diagnostic for each feature the resolver is honoring or omitting.

29. **PF1 best-fractional-progression save stacking is correct.** A real test (e.g. Fighter 4 / Wizard 4) exercises the formula and asserts the resulting saves match the PF1 rule's expected values: Fortitude is `(fighter_fort(4) + 0.5) + wizard_fort(4) = 3 + 0 + 0 = 3` or `2 + 1 + 0 = 3`, depending on PF1-specific semantics; **the exact formula is pinned to `src/rules_core/pilot_compute.rs`'s `decideEligibility` table as the canonical source of truth** — the per-class save progressions in `compute_<class>_chassis` call into `decideEligibility`'s `class_save_bonus(class_j, level_j)` function rather than re-deriving the rule. The save-stacking test runs against the canonical `decideEligibility` table and asserts the multiclass result equals the PF1 rule verbatim.

30. **Per-class feature integration reconciles two-class feature grants.** A Fighter/Wizard build with the Scribe Scroll feat (Wizard-side) and Martial Flexibility (Fighter-side hypothetical) gets both features' effects applied to the resolved `PilotBaseChassisComputation`. Single-class feature lists (e.g. `compute_wizard_chassis`'s `wizard_school`, `compute_fighter_chassis`'s `fighter_training`) are merged non-destructively. No regression on Epic 6's single-class feature outputs.

**Out of scope for Epic 7 (recorded explicitly to prevent scope creep):**

- *Multiclass spell-stacking edge cases (Sorcerer/Wizard prepared casting across both classes).* Edge case requiring investigation; deferred to a future Epic 7b or Epic 7c.
- *Triple-class (length-3+).* Triple-class and higher multiclass inputs are technically possible but rare in PF1; deferred to a future Epic 7d if operator needs it.
- *Multiclass skill-point allocation.* PF1 skill points per multiclass are 4×4 + INT, then distributed; this is the sum of both classes' skill-point budgets but with class-skill restrictions applying per class. Deferred to a follow-on Epic 7 if it surfaces as a bug.

**Cycle ordering (operator-prioritized)**

The operator can prioritize per the dependency graph. Default ordering:

1. Epic 1 — Code-Side Identifier Cleanup (must land before Epic 3 on shared files `controllerAdapter.ts`)
2. Epic 2 — Campaign Manager + Drive persistence
3. Epic 3 — Update UI bug remediation (lands after Epic 1; both touch `controllerAdapter.ts`)
4. Epic 6 — Single-class coverage completion (Wizard-first; rules-engine core)
5. Epic 5 — Build Version Numbering (after Epics 1-3 land; before Epic 4's closure sweep)
6. Epic 7 — Multiclass stacking (after Epic 6's single-second-class proof)
7. Epic 4 — Closure Epilogue (fires LAST; scans all 30 criteria before opening the final `tranche/4-1 → develop` PR)

## Cycle unit definition

A single loop cycle within an epic lands one acceptance criterion (or one representative sample for that criterion). Each cycle:

1. Picks one acceptance criterion from the epic's open list.
2. Verifies the working tree is on `tranche/4-1` (no feature branches; per the no-branches convention; per operator directive 2026-07-17: SD-21 launches on `tranche/4-1`, not `tranche/3`).
3. Reads the cycle's parity test fixture (for ingestion cycles) or the boundary-contract test (for campaign-manager cycles).
4. Implements the smallest change that satisfies the criterion.
5. Runs `cargo test --locked` (zero regressions) and `cargo clippy --locked --tests -- -D warnings` (clean).
6. Commits directly to `tranche/4-1` with a `feat(sd21): <criterion> (<row transition>)` message.
7. Mints a kanban card on `codex-tranche-4-1` as a post-mortem record (`status=done`, with merge receipt, audit-trail comment per codex-tranche-2-5 respawn-guard pattern).
8. Updates the shared progress doc's `## SD-21 cycles` section.
9. Exits.

A cycle is a *unit of post-mortem*, not a unit of delivered scope. One cycle, one criterion, one card, one commit.

## What the breakdown does not specify

- Per-cycle implementation approach — the loop picks the smallest change that satisfies the criterion.
- Per-cycle timing — depends on content-volume, parser friction, behavioral complexity; the loop's self-healing handles friction.
- Whether the campaign manager's GUI screens are tracked separately from the engine-side cycle work — the GUI is outside the bundle per `decisions.md` §2, so the engine-side cycles don't depend on the GUI's merge status.
- APG / ACG / advanced-guides content-source ingest — those live in SD-22 (`../SD-22/`). SD-21's Epic 2 reads from SD-19's `rules_tables/crb/` only; APG/ACG content is SD-22's job.
- Whether Epic 6's second-class extension (Cleric, Sorcerer, etc.) ships in this SD's release — operator-pinned at SD-21 launch. Epic 6a is Wizard; Epic 6b-... follow only if operator confirms.

## Cross-reference

- `decisions.md` — the 21-item decision record (epic split, ordering, dependency reasoning).
- `acceptance-and-verification.md` — closure gates including campaign-manager integration.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags (Flag A through Flag D; Open Q1 through Open Q5).
- `technical-design.md` — campaign-shape boundary contract shape, Drive adapter, markdown interop format.
- `technical-requirements.md` — pre-loop prerequisites.
- `../SD-22/` — sibling bundle (advanced guides + APG + ACG + Bestiary 1 + DM toolkit; scope expanded 2026-07-17 per operator directive).
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
