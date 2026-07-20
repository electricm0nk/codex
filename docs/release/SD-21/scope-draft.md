---
title: SD-21 — Campaign Manager + Drive + Multiclass Support + Identifier Cleanup + Update UI Bug + Build Versioning + Closure Epilogue — Scope Draft
status: approved (operator review 2026-07-16; operator directives 2026-07-17: branch flip tranche/5 → tranche/4-1, board flip codex-tranche-5 → codex-tranche-4-1, APG+ACG+advanced guides moved to SD-22 per lifecycle routing, Identifier Cleanup renumbered as Epic 1 with Epics 2-7 in order, 7-epic / 30-criteria final shape; /loop 60m /batch /goal launch form, pre-launch checklist added; bundle marked approved with operator directives 2026-07-16/17)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
mirror_of: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md §1
canonical_branch: tranche/4-1 (operator directive 2026-07-17)
kanban_board: codex-tranche-4-1 (operator directive 2026-07-17)
---

# SD-21 — Scope Draft (Tranche-4-1 dash release)

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
> 
> **This bundle is operated via the `/loop 60m /batch /goal` invocation model — NOT a one-shot task.** After exiting plan mode, the coding harness is **required** to launch SD-21 as:
> 
> ```bash
> /loop 60m /batch /goal ./loop-instruction.md
> ```
> 
> The full per-cycle procedure, file-touch partition, post-mortem card mint, and progress-doc update live in the loop-instruction file body. The scope-draft (this file) is the canonical handoff *what* — the loop-instruction is the *how*. See the loop-instruction's leading `⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE` block for the verbatim launch instruction and pre-launch checklist.

## 0. Preamble

This is the canonical handoff for SD-21. The `/loop 60m /batch /goal ./loop-instruction.md` invocation reads this file plus its sibling doctrine files and runs to closure.

Working in bounded cycles against the integration branch `tranche/4-1` (per operator directive 2026-07-17; SD-21's branch is `tranche/4-1`, NOT `tranche/3` or `tranche/4`; slash-form dash release per the operator's call). Each cycle lands one acceptance criterion.

The progress doc `./progress.md` (created on first cycle by the loop) carries the cycle-log + status matrix (per `governance/spec-domain-lifecycle.md`'s plan A on the SD-status transcription surface).

## 1. SD-21 — 30 criteria across 7 epics on `tranche/4-1`

SD-21 ships seven epics, each with its own capability slice. Identifier Cleanup (Epic 1) fires FIRST on shared files; Closure Epilogue (Epic 4) fires LAST. Per operator directive 2026-07-17, APG + ACG + advanced-guides content-source ingest is **SD-22's surface**, not SD-21's; SD-21 reads from SD-19's `rules_tables/crb/` only.

### 1.1 Epic 1 — Code-Side Identifier Cleanup (governance base requirement; fires FIRST)

Under the identifier-discipline doctrine (`../../doctrine-external/identifier-discipline.md`), source-code identifiers must describe what the artifact does, not which release or spec domain it came from. Epic 1 cleans up the load-bearing identifier leaks already in the codebase: Tauri command names with the `sd16_` / `sd19_` prefix, TypeScript functions and constants with `Sd16` / `SD16` text, `data-testid` attributes with `sd16-` prefixes, inline doc-comments citing `SD-N-Ex...` identifiers, and any `t_<hex>` kanban tokens / `AV-PAY-5` audit IDs embedded in source. The Epic 1 cycles also rename TypeScript function names like `createSd16UpdateControllerDeps` → `CreateUpdateControllerDeps` and constants like `SD16_UI_BUTTON_BASE_STYLE` → `UpdateRestoreButtonBaseStyle`. **Directory tree renames** (`apps/desktop/src/sd16/` → `apps/desktop/src/update/`) are explicitly out of scope for Epic 1; that work belongs to a follow-on epic in a future bundle. **Cycle order: Epic 1 fires before Epic 3.** Both touch `apps/desktop/src/sd16/update/controllerAdapter.ts`. Detailed acceptance criteria 1-4 in `programs/codex/requirements/SD-21-campaign-manager-and-persistence/epic-breakdown.md` §"Epic 1 — Code-Side Identifier Cleanup."

### 1.2 Epic 2 — Campaign Manager + Drive persistence (gates 2, 3, 4, 7, 8)

The campaign manager + Drive persistence epic is the load-bearing epic for SD-21 (after Identifier Cleanup fires). It defines the engine-side `CampaignSnapshot` and `CampaignBackend` types and lands the Drive adapter as the first `CampaignBackend` implementation. Per-cycle reads from SD-19's `rules_tables/crb/`; APG/ACG/advanced-guides content is SD-22's surface.

**Concrete deliverables:**

- **`CampaignSnapshot` types.** NEW module `src/rules_core/campaign.rs`. Types: `CampaignSnapshot`, `CampaignMetadata`, `Party`, `PartyMember`, `CharacterSummary`, `PartyResources`, `AdventureLogEntry`, `MapRef`, `WikiPage`. JSON-serializable. Per `technical-design.md` §1.1.
- **`CampaignBackend` trait.** NEW module `src/rules_core/persistence/mod.rs`. Backends: load_campaign, save_campaign, list_campaigns, create_campaign, delete_campaign, snapshot_known_format. Per `technical-design.md` §1.3.
- **Drive adapter.** NEW module `src/rules_core/persistence/drive.rs`. Implements `CampaignBackend`. OAuth flow per `technical-design.md` §2.1. Markdown file layout per `technical-design.md` §2.2. Conflict detection per `decisions.md` §7.
- **Tauri command surface.** NEW module `apps/desktop/src-tauri/src/campaign_drive.rs`. Commands: `drive_authorize`, `drive_pick_folder`, `drive_list_campaigns`, `drive_load_campaign`, `drive_save_campaign`, `drive_delete_campaign`. The GUI's vibe-coded campaign-manager screens (PR #316) consume these via `apps/desktop/src/boundary/campaignDrive.ts` (NEW).
- **Tests.** NEW test files at `tests/sd21_drive_round_trip.rs`, `tests/sd21_drive_conflict_log.rs`, `tests/sd21_campaign_snapshot_serializes.rs`. The latter is the per-cycle acceptance criterion (the campaign-round-trip test reads a `CampaignSnapshot` from a fixture, writes it through the Drive adapter to a fixture-folder, reads it back, asserts identity for unchanged fields).
- **Capability-slice artifacts.** NEW doc `docs/SD-21/campaign-boundary-contract.md` (engine-side API surface), `docs/SD-21/drive-adapter-spec.md` (OAuth + folder + conflict spec), `docs/SD-21/markdown-file-format.md` (YAML frontmatter schema per file).
- **Google OAuth credentials.** Operator supplies `GOOGLE_OAUTH_CLIENT_ID`, `GOOGLE_OAUTH_CLIENT_SECRET`, `GOOGLE_OAUTH_REDIRECT_URI`; saved in `~/.hermes/profiles/god-emporer/.env` per the credentials-location rule.
- **Identifier-cleanup prerequisite.** Epic 1 lands first per the shared-file rule (`controllerAdapter.ts`), so when Epic 2 lands its identifier names are already descriptive.

### 1.3 Epic 3 — Update UI bug remediation (post-tranche-3 defect remediation; code references only)

Routed to SD-21 under the spec-domain lifecycle doctrine (`../../doctrine-external/spec-domain-lifecycle.md`): the work belongs to the bundle currently shaping the next release package — SD-21 — not to the originating closed bundle. The bug fix is owned by SD-21 because SD-21 is shaping the next release package. Affected code is cited by file:line; no reorganization of `programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/` is implied. Detailed acceptance criteria 12-15 in `programs/codex/requirements/SD-21-campaign-manager-and-persistence/epic-breakdown.md` §"Epic 3 — Update UI bug remediation." Epic 3 cycles land AFTER Epic 1 (both touch `controllerAdapter.ts`).

### 1.4 Epic 4 — Closure Epilogue (final scan + PR + worktree cleanup + release notes + version increment; fires LAST)

The standard part-of-handoff-doctrine for every spec-domain closure going forward (per the operator directive 2026-07-17): Epic 4 fires LAST in the SD-21 cycle-ordering (its slot is Epic 4 in the 7-epic layout; "fires LAST" is its position in the cycle order, not its slot number). Its cycle scans every prior criterion (1-30) for `complete` or `## Open blockers` status; opens the `tranche/4-1 → develop` closure PR via `gh pr create`; cleans up worktrees and stale branches; generates release notes under `programs/codex/requirements/SD-21-campaign-manager-and-persistence/release-notes.md`; runs the closure test suite as the final gate. The *generic* version-increment logic in Epic 4 reads the current `<major>.<tranche>.<build>` triple, increments only the **tranche** position by `1` (and resets build to `0`) on tranche promotion (`0.4.<last_build>` → `0.5.0` for the next dash-release); the *specific* `0.4.<current_build>` value comes from Epic 5's first-cycle bump. Detailed acceptance criteria 19-24 in `epic-breakdown.md` §"Epic 4 — Closure Epilogue."

### 1.5 Epic 5 — Build Version Numbering (`<major>.<tranche-base>.<build>` + build-label format)

The display-build-version bug handoff (operator directive 2026-07-17) lands here. The version scheme is a three-position triple `<major>.<tranche-base>.<build>` (replacing the prior `0.0.X` patch-only scheme that the operator has confirmed was a bad call):

- **`major`** (first number) is `0` until the first publish to `main`; increments by `1` per main-publish.
- **`tranche-base`** (second number) is the **base** of the active working tranche — `tranche/4-1` carries `4`; `tranche/4-2` (a future dash from 4) also carries `4`; `tranche/5` carries `5`. Increments *slowly*, only on tranche promotion.
- **`build`** (third number) is a **monotonic counter across all builds across all branches — never resets**. Increments by `1` on every merge. `0` → `92` → `93` → `100` → `200` → ... → `∞`.

The first concrete value for SD-21's release is **`0.4.<current_build>`** — e.g. **`0.4.93`** if the build counter is at 92 today (operator's prior-session note). The current-build anchor is checkable via git log + the prior-session receipt comment chain; the operator pins a value at SD-21 cycle launch if not retrievable.

Epic 5 cycles touch three files for the version field (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`); `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` for the build-label format change (`Codex 0.4.<build>` instead of `codex@0.0.0`); three test-fixture files; and a new `docs/SD-21/release-closure-checklist.md` establishing the closure-process for future SD-N releases (with per-position increment rules: build per-CI-build, tranche per-tranche-promotion, major per-main-publish). **Epic 5 fires BEFORE Epic 4** so the version commit is in Epic 4's closure PR's history. Detailed acceptance criteria 25-27 in `epic-breakdown.md` §"Epic 5 — Build Version Numbering."

### 1.6 Epic 6 — Single-class coverage completion (Wizard-first; rules-engine core)

Per the multiclass-and-broader-single-class-support bug handoff (operator directive 2026-07-17), `compute_pilot_base_chassis` at `src/rules_core/pilot_compute.rs:4568` currently dispatches only to `compute_fighter_chassis`, leaving every non-Fighter single class and any multiclass combination permanently blocked. Epic 6 refactors the dispatch to per-class routing and brings at least Wizard (the bug handoff's recommended first target, since `supported_wizard_level` exists at `pilot_compute.rs:13967` capped at level 11) to full single-class Computed support. **One sub-feature per cycle** (BAB progression, saves, spell slots, Arcane School, Scribe Scroll, etc. — six or more cycles for the Wizard extension alone). Cleric/Sorcerer/etc. follow as operator-pinned 6b/6c/...epics. Detailed acceptance criteria 25-27 in `epic-breakdown.md` §"Epic 6 — Single-class coverage completion."

### 1.7 Epic 7 — Multiclass stacking (BAB + saves + features)

Once Epic 6 proves a single second class can be computed correctly, Epic 7 extends `compute_pilot_base_chassis` to handle multiclass inputs (length-2+ `class_levels`). Three mechanics: BAB stacking (sum each class's own progression: `good` / `medium` / `poor`); PF1's best-fractional-progression save stacking (NOT a naive sum — the *correct* PF1 rule, which Epic 7 must apply verbatim from `decideEligibility.class_save_bonus`); and per-class feature integration (reconcile two-class feature grants without clobbering each other). Detailed acceptance criteria 28-30 in `epic-breakdown.md` §"Epic 7 — Multiclass stacking."

## 2. Promotion gate

After all seven epics close AND `codex-tranche-4-1` board shows every acceptance criterion `complete` (or `## Open blockers` documented), the loop opens a `tranche/4-1 → develop` promotion PR per the existing cadence. Epic 4's cycle IS the closure PR open — Epic 4 criterion 17 runs `gh pr create`. The PR's description references all 30 acceptance criteria, the cycle-merge receipt SHAs, the release notes preview, and the worktree/branch summary from Epic 4 criterion 18. The PR body includes audit-trail comments per the codex-tranche-2-5 respawn-guard pattern.

## 3. Cross-reference

- `./decisions.md` — 21-item decision record; required reading for understanding the SD-21 shape. §11 documents the branch flip from `tranche/5` to `tranche/4-1`; §15 documents Epic 3's lifecycle routing; §16 documents Epic 1's identifier discipline; §17 documents the closure epilogue; §18 documents the build version numbering; §19 documents the multiclass + Epic 6/7 routing; §20 documents the bundle-size posture; §21 closes the operator-deferred shape decisions.
- `./technical-design.md` — campaign-shape boundary contract shape, Drive adapter, markdown interop format, Epic 6/7 `pilot_compute.rs` hooks; required reading for engineering work.
- `./epic-breakdown.md` — 30 acceptance criteria grouped into 7 epics (Epic 1 Code-Side Identifier Cleanup; Epic 2 Campaign Manager + Drive persistence; Epic 3 Update UI bug remediation; Epic 4 Closure Epilogue; Epic 5 Build Version Numbering; Epic 6 Single-class coverage completion; Epic 7 Multiclass stacking).
- `./acceptance-and-verification.md` — closure gates (gates 1-13).
- `../SD-22/` — sibling bundle (advanced guides + APG + ACG + Bestiary 1 + DM toolkit; scope expanded 2026-07-17 per operator directive).
- `../../doctrine-external/spec-domain-lifecycle.md` — sibling lifecycle doctrine; governs Epic 3's lifecycle routing.
- `../../doctrine-external/identifier-discipline.md` — sibling identifier-discipline doctrine; governs Epic 1's identifier-cleanup criteria.
- `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md` — procedural skill for Epic 1 cycles; loaded by the SD-21 loop when Epic 1 fires.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store (CRB); SD-22 sources the `RuleSetId::Apg` and `RuleSetId::Acg` content from the same pattern, SD-21 reads `RuleSetId::Crb` only.
- `../SD-20/` — sibling bundle (parallel; per-character tabletop-readiness on `tranche/4`).
- `./loop-instruction.md` — loop body, the `/loop` invocation reads this.
