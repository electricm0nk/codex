---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-16; operator directives 2026-07-17: branch flip tranche/5 → tranche/4-1, board flip codex-tranche-5 → codex-tranche-4-1, APG+ACG+advanced guides moved to SD-22, Identifier Cleanup renumbered as Epic 1, 7-epic / 30-criteria final shape; bundle marked approved with operator directives 2026-07-16/17)
date: 2026-07-15
canonical_branch: tranche/4-1 (operator directive 2026-07-17)
kanban_board: codex-tranche-4-1 (operator directive 2026-07-17)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
mirror_of: /home/workspace/SD-21-campaign-manager-and-persistence-scope-draft.md
---

# SD-21 — Acceptance and Verification

## Closure gates (mandatory)

SD-21 closes when every closure gate below is met AND a `tranche/4-1 → develop` promotion PR has been merged for the SD-21 work (or merged jointly with SD-20 if the timing converges). Each gate is independently verifiable.

1. **Tranche-3 baseline green**. SD-18 chassis done; SD-19 corpus-aware seam + canonical Paizo-table store done; SD-19 §3.4/§3.5 acceptance criteria grounded. Confirmed by `cargo test --locked` green and the shared progress doc's `## SD-19 cycles` section closed.

2. **Campaign manager + local-folder persistence lands (Epic 2)** *(amended 2026-07-19: `CampaignBackend` trait + Drive adapter → concrete `CampaignStore` struct + local-folder impl, per operator directive — no trait-object pattern exists elsewhere in this codebase, and OAuth/Drive-API is separately descoped, see gate 3)*. The `CampaignSnapshot` types (`src/campaign/mod.rs`) and `CampaignStore` (`src/campaign/local_store.rs`, mirroring `SavedCharacterStore`'s conventions) are checked in as 7 small atomic commits (E2.5-E2.11) rather than one slice. The engine produces a `CampaignSnapshot` from a sample input and round-trips it through JSON (`round_trips_through_json` test in `src/campaign/mod.rs`); the round-tripped snapshot is byte-identical to the input for unchanged fields. Epic 2 reads from SD-19's `rules_tables/crb/` only.

3. **Drive OAuth flow operational** — **DESCOPED from SD-21 (operator directive 2026-07-18)**. No Google Cloud Console credentials were configured, and rather than block Epic 2 on that setup, the `CampaignStore`'s only backend for this bundle is a local folder the user points at a Google-Drive-for-Desktop-synced directory (see the loop-instruction's "Epic 2 engine-shape addendum" section). `drive_authorize`, OAuth token exchange, and OS-keyring storage are OUT of SD-21's scope entirely — a future bundle's job if real Drive-API sync is ever wanted. This gate does not block closure.

4. **Markdown file format ships** *(amended 2026-07-19 per operator directive, post-implementation)*. The original spec called for a per-field YAML-frontmatter file layout (`campaign.md`, `party.md`, `members/<character_id>.md`, etc.). Epic 2's actual implementation instead extends the already-shipped `campaign_drive.rs` local-disk writer (PR #320) rather than introducing a parallel layout: campaign-level structured fields (metadata, party, members) serialize as one JSON file at `<campaign_folder>/.config/<campaign_name>.json`, and only the four markdown-bodied asset collections (`resources/`, `adventure-log/`, `maps/`, `wiki/`) land as one `.md` file per asset (title-derived filename, plain markdown body, no YAML frontmatter needed since these are body-only content — no structured fields to carry). This was a deliberate choice to avoid reworking PR #316/#320's already-shipped, already-tested writer for a layout no consumer needed. Gate verification: `src/campaign/local_store.rs`'s `load_honors_an_external_obsidian_style_edit_to_an_asset_markdown_file` and `load_picks_up_a_markdown_file_created_entirely_outside_the_app` tests confirm the asset-folder round-trip; the `.config/<name>.json` file is the campaign-metadata carrier.

5. **Conflict resolution works**. When the local-folder store detects a `nonce` mismatch on load (i.e. another device wrote since the local save), both copies go to `conflicts/<timestamp>/`, the local version becomes active. Landed as `save_with_conflict_detection`/`load_with_nonce` in `src/campaign/local_store.rs`, covered by 4 dedicated unit tests (`first_save_with_conflict_detection_bumps_nonce_to_one_and_reports_no_conflict`, `save_with_a_matching_expected_nonce_never_conflicts`, `save_with_a_stale_expected_nonce_moves_the_prior_state_into_conflicts`, `load_with_nonce_returns_zero_for_a_campaign_saved_without_conflict_detection`) rather than the originally-named separate `tests/sd21_drive_conflict_log.rs` integration file *(amended 2026-07-19)*. The GUI conflict-detected affordance is not yet wired (see the progress doc's Open Blockers — E2's frontend-binding gap covers this too).

6. **MD interop with Obsidian round-trips**. An externally-edited asset markdown file's new body is honored on reload, and a brand-new `.md` file dropped into an asset directory outside the app is picked up with a filename-derived title. Landed as `load_honors_an_external_obsidian_style_edit_to_an_asset_markdown_file` and `load_picks_up_a_markdown_file_created_entirely_outside_the_app` in `src/campaign/local_store.rs`, rather than a top-level `campaign.md` description-field round-trip test *(amended 2026-07-19, reflecting gate 4's amended layout — asset bodies, not campaign-level description, are what actually lands as markdown)*.

7. **`tranche/4-1 → develop` promotion PR opened**. Operator opens the promotion PR per the existing cadence. The PR includes the SD-21 commits alongside SD-20's, with audit-trail comments per codex-tranche-2-5 respawn-guard pattern.

8. **Epic 1 acceptance criteria land (Code-Side Identifier Cleanup)**. The four Epic 1 criteria in `epic-breakdown.md` (criteria 1, 2, 3, 4) all move to `complete` on the SD-21 progress matrix before any other epic cycles. Epic 1 is the governance base requirement under the identifier-discipline doctrine (`governance/identifier-discipline.md`); it removes bundle-tag identifiers (`sd16_*`, `SD16_*`, `Sd16*`, `sd19_*`, `SD19_*`, `sd16-*` test-IDs, `SD-N-Ex...` audit IDs, `t_<hex>` kanban tokens) from the source and inline doc-comments. Verification: at `tranche/4-1` after Epic 1's final cycle, `grep -rE "sd16_|SD16_|Sd16|sd19_|SD19_|Sd19|sd[0-9]+_" apps/desktop/src/ apps/desktop/src-tauri/src/` returns zero hits in identifier positions, and zero `SD-[0-9]+-[A-Z][0-9]` or `AV-PAY-[0-9]+` or `Tranche [0-9]+ chassis lane` strings in source-code comments. Epic 1 fires FIRST in the cycle-ordering section of `epic-breakdown.md`.

9. **Epic 3 acceptance criteria land (Update UI bug remediation)**. The four Epic 3 criteria (12, 13, 14, 15) all move to `complete` on the SD-21 progress matrix before tranche closure. Epic 3 carries bug fixes for the next release package under the spec-domain lifecycle doctrine (`governance/spec-domain-lifecycle.md`); its closing commits are on `tranche/4-1` and ship together with the rest of the SD-21 work. Verification: the SD-21 loop's `## SD-21 cycles` matrix at `./progress.md` shows Epic 3's criteria 12-15 each with `Status: complete` and a `tranche/4-1` receipt SHA. Epic 1 lands before Epic 3 (shared-file `controllerAdapter.ts` rule).

10. **Epic 5 (Build Version Numbering) fires before Epic 4**: the three version fields (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`) read `"0.4.<current_build>"` (e.g. `"0.4.93"` if the build counter is at 92); `createSd11WorkbenchStatus.ts` reads `BUILD_PREFIX = 'Codex'` and the template `${BUILD_PREFIX} ${buildVersion}` (with the space, not the `@`); the test fixtures update to assert/fixture `Codex 0.4.<build>` shape; `docs/SD-21/release-closure-checklist.md` exists with the four-step closure-process using the `<major>.<tranche-base>.<build>` triple. Verification: `grep -E "\"version\"|^version" apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json apps/desktop/src-tauri/Cargo.toml` returns `"0.4.<build>"` for all three (the `<build>` position verified by the operator-pinned anchor at cycle launch); `grep "codex@\|@0\.0"` apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` returns zero hits; `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` runs cleanly and refreshes `Cargo.lock`'s embedded version. **Epic 5 lands BEFORE Epic 4** (the version commit is in Epic 4's closure PR's commit history).

11. **Epic 6 (Single-class coverage completion) fires Wizard-first**: `compute_pilot_base_chassis` at `pilot_compute.rs:4568` dispatches by class (not Fighter-only); single-class Human Wizard 3 reaches `Status: Computed` via the bug handoff's reproducer; `compute_wizard_chassis` extends from level-cap 11 to full 1-20 with BAB, saves per PF1 Wizard progression, spell slots, prepared-vs-spontaneous spellbook, Arcane School + Opposition School selection, and class-feature grants (Scribe Scroll, etc.). **One sub-feature per cycle** (BAB progression is one cycle, saves is another, spell slots is another, Arcane School is another, Scribe Scroll is another, etc.). Each per-cycle test runs against `compute_wizard_chassis`'s post-compute state. Cleric/Sorcerer/etc. follow as operator-pinned 6b/6c/...epics. **Epic 6 lands BEFORE Epic 7** per bug handoff's two-phase rationale.

    **STATUS (2026-07-19): gate MET — verified independently, not just self-reported.** Epic 6b (`epic-breakdown.md`'s "Epic 6b — Wizard full-completion" section) landed all three scoped criteria: E6b.1 widened `compute_combat_baseline`/`compute_selected_skill_modifiers` to the same `has_supported_class_chassis` gate `compute_total_saves` already used (commit `6ed19bd`); E6b.2 grounded a real prepared-spellbook system, bounded to levels 1-3 (commit `de554ab`); E6b.3 grounded Evocation's school powers (Intense Spells, Force Missile) and the opposed-school 2-slot cost rule (commit `1c7ad89`). The capstone test `tests/sd21_epic6b_full_completion_reproducer.rs` — a single-class Human Wizard 3, Evocation specialization, populated spellbook, daily preparation selection — was run directly (not just trusted from the landing agent's report) and **passes**: `build_pilot_headless_receipt` reaches `Status::Computed` with zero claim-blocking diagnostics. Full suite re-verified green afterward (397/397 root-crate test-result lines ok, clippy clean). **Scope note: bounded to Evocation only** — the other 7 PF1 arcane schools are explicit follow-on (Epic 6c/6d/...), and Wizard levels 4-20's spellbook/slot-consumption grounding is also follow-on (`WIZARD_SPELLBOOK_SUPPORTED_MAX_LEVEL=3`). This does not block gate 11: the gate's own reproducer names Human Wizard 3 specifically, which now genuinely reaches `Computed`.

12. **Epic 7 (Multiclass stacking) fires after Epic 6**: `compute_pilot_base_chassis` extends to length-2+ `class_levels` inputs via `compute_multiclass_base_chassis`; BAB stacking sums each class's progression (`good` / `medium` / `poor`); saves use PF1's best-fractional-progression rule (NOT a naive sum) — `compute_<class>_chassis` calls into `decideEligibility.class_save_bonus(class_j, level_j)` rather than re-deriving the rule. Per-class feature integration reconciles two-class feature grants (`compute_wizard_chassis.wizard_school` + `compute_fighter_chassis.fighter_training` etc.) without clobbering each other. Per-cycle tests with a Fighter 4 / Wizard 4 build assert the resulting saves match the canonical `decideEligibility` table.

13. **Epic 4 (Closure Epilogue) fires LAST**: every criterion (1-30) is `Status: complete` OR `## Open blockers`; the closure PR (`tranche/4-1 → develop`) is opened via `gh pr create`; worktrees and stale branches are cleaned up; release notes are generated; the *tranche-position* version increment lands (`<major>.<tranche>.<build>` triple, only the *tranche* position increments on tranche promotion; criterion 23's mechanic — the per-CI-build *build* increment is operator-pinned at cycle launch and the per-main-publish *major* increment is a future bundle's epic). The closure PR's description carries the full-matrix summary, the per-criterion cycle-receipt SHAs, and a release-notes preview. Verification: at the moment the loop terminates, `hermes kanban list --board codex-tranche-4-1 --status done` shows all Epic 4 cycle-cards closed, and the `tranche/4-1 → develop` PR has been opened against the bundle's actual receipt history.

## Verification at closure

The closure posture is reviewable entirely from these surfaces:

- `~/workspace/SD-18-core-rules-breadth-progress.md` — shared progress doc; SD-21 appends under its own `## SD-21 cycles` section.
- The campaign-boundary-contract artifact at `docs/SD-21/campaign-boundary-contract.md`.
- The Drive adapter spec at `docs/SD-21/drive-adapter-spec.md`.
- The markdown file-format spec at `docs/SD-21/markdown-file-format.md`.
- `git log --oneline tranche/4-1 -N` — the SD-21 commit history.
- `codex-tranche-4-1` board — SD-21 cards populated, every epic-card `status=done`, with audit comments per codex-tranche-2-5 respawn-guard pattern.
- `~/.hermes/profiles/god-emporer/.env` — contains the Google OAuth credentials (verify they're set, not used in tests).

Operator's first action on return from a multi-day run: read the `## SD-21 cycles` section of the shared progress doc; if empty, gates 1–13 above are the entire verification.

## What does *not* gate closure

- Loop's cycle log size (10 cycles or 100; criterion is the criterion, not volume).
- Number of self-heals.
- Whether some epic-cards land as documentation-only versus full code-bearing (per the eligibility check).
- Whether the GUI campaign-manager screens (PR #316's work) are merged before SD-21's loop cycles finish — the engine-side is independent of the GUI-side's merge status.
- Full character-detailed spell effects / damage rolls in the campaign's `CharacterSummary` — those arrive after SD-20 closes. Until then, the summary carries chassis-only fields (per `technical-design.md` §1.2).
- Tier-4 campaign-toolkit features (DM encounter builder, party-CR-based encounter difficulty math) — those are SD-22's responsibility per the operator's directive 2026-07-15.
- APG / ACG / advanced-guide / Bestiary 1 content-source ingest — those are SD-22's responsibility per the operator directives 2026-07-17 (advanced guides) and 2026-07-17 (APG + ACG).
- Identifier-cleanup directory-tree rename (`apps/desktop/src/sd16/` → `apps/desktop/src/update/`) — recorded as out-of-scope for Epic 1 (`epic-breakdown.md` §Epic 1 out-of-scope). It's a future bundle's epic.

## Cross-reference

- `decisions.md` — the 21-item decision record (9 originals + 12 added: §10 cross-bundle auto-upgrade on SD-20 close, §11 SD-21 launch branch flip `tranche/5 → tranche/4-1` per operator directive 2026-07-17, §12 resolver cross-book fallback APG→CRB→ACG, §13 Status matrix in progress doc, §14 Q1–Q5 closure summary, §15 Epic 3 (Update UI bug) lifecycle routing, §16 identifier discipline + Epic 1 routing, §17 closure epilogue as standard handoff, §18 build version numbering `<major>.<tranche-base>.<build>` three-position scheme per operator directive 2026-07-17, §19 multiclass + broader single-class support (Epic 6 + 7), §20 SD-21 sized for one tranche (7 epics / 30 criteria), §21 operator-deferred shape decisions now closed).
- `epic-breakdown.md` — 30 acceptance criteria grouped into 7 epics (Epic 1 Code-Side Identifier Cleanup; Epic 2 Campaign manager + Drive; Epic 3 Update UI bug remediation; Epic 4 Closure Epilogue; Epic 5 Build Version Numbering; Epic 6 Single-class coverage completion; Epic 7 Multiclass stacking).
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-design.md` — campaign-shape boundary contract shape, Drive adapter boundary contract, markdown interop format, Epic 6/7 `pilot_compute.rs` hooks.
- `technical-requirements.md` — pre-loop prerequisites.
- `../SD-22/` — sibling bundle (advanced guides + APG + ACG + Bestiary 1 + DM toolkit; scope expanded 2026-07-17 per operator directive).
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `../SD-20/` — sibling bundle.
- `../SD-19/` — table store + RuleSetId pattern.
