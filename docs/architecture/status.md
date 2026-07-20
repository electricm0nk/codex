# Status

> Scope: what is real, working product surface today across the whole repo, and what is stubbed, partially wired, or deferred — superseding the root README's "Current state" section.
> Last verified: 2026-07-20 against ef9012bf5de8
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

## Posture

Codex today is a developer proof-harness and a buildable desktop workbench,
not a finished character-management product. The corpus-ingest pipeline, the
deterministic compute chassis, the boundary contract, and every persistence
store are real, tested, and exercised end to end by `cargo test --locked`
and `npm test`. But character coverage is narrow: **single-class Fighter at
levels 1-3, for any race, is the only path that reaches a fully `Computed`
receipt today** — every other class/level combination returns real
claim-blocking diagnostics from the engine (two `apps/desktop/src-tauri/src/character_hub.rs`
tests prove this jointly:
`compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3`
covers the Fighter levels-1-3 `Computed` half, and
`claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class`
asserts the per-class claim-blocking diagnostic sets for the other classes;
verified directly against both test bodies).
Several desktop-facing actions that look interactive are session-local or
inert by design, named individually below — this is the fail-honest
convention (see [conventions.md](./conventions.md)) applied at the product
level: a stub says so rather than pretending to work.

## Real today

| Area | What works | Where |
|---|---|---|
| Corpus-ingest pipeline | `.pcc`/`.lst` parsing through canonical `SourcePackageContent` projection, six of seven record kinds fully wired | [corpus-ingest.md](./corpus-ingest.md) |
| Pilot compute + boundary contract | `compute_pilot_base_chassis` → `compute_pilot_with_corpus` → `to_pilot_receipt` → `printed_sheet_cell_map`, fail-honest throughout | [rules-engine.md](./rules-engine.md) |
| Per-domain engines | Spellbook (9/9 schools), skill allocation, feat prerequisites (4/4 categories), equipment effects (4/4 categories), damage total, level-up (11/11 classes) | [rules-engine.md](./rules-engine.md) |
| Four rule-table catalogs | CRB (full), APG (6/6 classes), ACG (10/10 classes), Bestiary 1 (41 monsters across 8 subsets) | [rules-data-tables.md](./rules-data-tables.md) |
| Character Hub | Create, load, clone, portrait upload/load/delete, JSON export — all real engine compute + real persistence | [desktop-app.md](./desktop-app.md) |
| Campaign manager (local) | Create/edit/list campaigns and their assets, backed by `CampaignStore` on disk; nonce-based conflict detection with local-wins + preserved-conflict-copy resolution | [persistence.md](./persistence.md) |
| Update eligibility / restore / verify | `is_install_eligible`, `perform_restore_previous`, `verify_relaunch_artifact` — all real, tested Tauri commands | [update-and-feedback.md](./update-and-feedback.md) |
| Feedback composers + browser handoff | Bug/enhancement draft composition, evidence capture/redaction, and the governed GitHub-issue browser handoff | [update-and-feedback.md](./update-and-feedback.md) |
| Release pipeline | Multi-platform publish, dual manifest validation, channel-index push, branch-promotion gates — the machinery is real and has shipped releases, but the three most recent publish runs from `develop` (2026-07-19/20) failed at the `test` job's frontend-typecheck step, so nothing is currently shipping (see [testing.md](./testing.md)) | [release-pipeline.md](./release-pipeline.md) |
| Support-state matrix | 34-row typed truth ledger, read-only bridged to the desktop tester workbench | [support-state-matrix.md](./support-state-matrix.md) |
| IPC bridge liveness | `load_backend_health` returns the real crate version and compile-time git SHA; reaching it at all proves the Tauri bridge is alive | [desktop-app.md](./desktop-app.md) |
| Homebrew authoring workbench | The Guard Stance proof package's validate/persist/preview round trip, read-only bridged to the desktop tester workbench | [homebrew-and-oracle.md](./homebrew-and-oracle.md) |
| Encounter difficulty / party CR compute | `Encounter::new` and `party_challenge_rating` are real, grounded compute — but see the DM Toolkit UI row below | [rules-engine.md](./rules-engine.md) |

## Stubbed / partially wired / deferred today

Grouped by the plane each item lives in. Every row was re-verified directly
against the cited source, not carried over from a sibling doc unchecked.

### Desktop app: character sheet and update actions

| Item | Status | Where (re-verified) |
|---|---|---|
| `perform_install` | Always returns `Err("...not wired: downloading the AppImage artifact requires an HTTP client...")`; its TS caller `installAction.ts::performInstall` has zero production call sites — `Ui.tsx`'s `handleInstall` is a documented no-op. Doubly inert. | `apps/desktop/src-tauri/src/update/transaction.rs:763-771`; `apps/desktop/src/sd16/update/Ui.tsx:110-117` |
| `perform_retention_sweep` | Real, tested body (`perform_retention_sweep_impl`), but not in `main.rs`'s `generate_handler!` list — unreachable from the frontend. | `apps/desktop/src-tauri/src/update/transaction.rs:817`; `apps/desktop/src-tauri/src/main.rs:113-140` |
| `drive_list_campaigns` / `drive_load_campaign` / `drive_save_campaign` / `drive_delete_campaign` | Registered in `generate_handler!` and unit-tested, but no frontend file invokes any of them (confirmed: zero grep hits across `apps/desktop/src`). `campaignModel.ts` uses `localStorage` as the real source of truth; only `write_campaign_drive_artifacts` (one-way mirror) is called. | `apps/desktop/src-tauri/src/main.rs:132-135`; `apps/desktop/src/campaign/campaignModel.ts` |
| Level-up acceptance | `LevelUpDialog`'s `onAccept` in `CharacterSheet.tsx` is an empty closure with a comment: "accepting is a no-op today." Nothing is persisted or recomputed. | `apps/desktop/src/characterHub/CharacterSheet.tsx:803-806` |
| Skill-allocation acceptance | `SkillAllocationDialog`'s own header comment: "Accepting only updates in-memory state (`onAccept`) — there is no backend [persistence]." Wired to a plain `useState` setter, lost on sheet close. | `apps/desktop/src/characterHub/SkillAllocationDialog.tsx` (header comment) |
| Character-sheet bio fields | Alignment/deity/sex/age/height/weight/hair/eyes are explicitly session-local; no persisted schema slot exists yet. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`DetailsPanel`) |
| Sheet `☰ Menu` | `Open`/`Save`/`Clone` menu items are `onSelect: () => {}` no-ops; only `Print` (`window.print()`) does anything real. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`menuItems`) |
| Campaign conflict merge | Conflict detection is real and tested (nonce-based); resolution is local-wins with both copies preserved under `conflicts/<timestamp>/` — there is no merge UI. | [persistence.md](./persistence.md) §"Conflict detection" |
| DM Toolkit UI | The Landing screen's "DM Toolkit" action routes to `StubScreen.tsx`, a generic "not built yet" placeholder — it does not call `encounters.rs`/`party_cr.rs` even though that compute is real (see the Real-today table above). | `apps/desktop/src/characterHub/CharacterHubPage.tsx:93-99`; `apps/desktop/src/characterHub/StubScreen.tsx` |

### Core engine: compute coverage and proof surfaces

| Item | Status | Where (re-verified) |
|---|---|---|
| Class/level compute coverage | Only single-class Fighter levels 1-3 reach `Computed` for any race; Wizard level 1 is closest but still blocked on spellbook/school-power diagnostics. | `apps/desktop/src-tauri/src/character_hub.rs:949-954` (test) |
| Oracle-parity comparator | `oracle_validation` is structurally incapable of claiming parity today: `DimensionStatus` has no `Passed` variant (only `Candidate`/`Blocked`/`NotYetGrounded`), `ClaimTierFloor` has no `OracleChecked` variant (only `Computed`); comparator, normalizer, report-writer, and PCGen-runner are all absent. | `src/oracle_validation/golden_fixture.rs:117-121`; `src/oracle_validation/selected_parity_dimensions.rs:12-16` |
| Bestiary 1 monster parser | `monster_stat_block.rs`'s row parser is fully unwired: no `ParsedLstRecord`/`SourceContentPayload` variant exists for it, and its only callers outside its own module are its own test file. Bestiary 1 table content is hand-transcribed, not parsed through the canonical-IR path. | `src/pcgen_import/lst_parser/monster_stat_block.rs`; zero references in `ir_converter.rs`/`source_content_payload.rs`/`source_content.rs` |
| Failure-owner classifier | `pilot_failure.rs`'s `primary_owner` only ever returns `OracleGap` (on `Computed`) or `EngineFlaw` (on `Blocked`); `ModelFlaw`/`ImporterFlaw`/`UiGap` are unreachable from the current receipt surface. | `src/rules_core/pilot_failure.rs:61-66` |
| Per-item corpus equipment stats | `pilot_compute_corpus.rs`'s `DerivedEquipmentStats` is always `default()` — a permanent placeholder there; real per-item stats are computed separately by `equipment_effects.rs`. | `src/rules_core/pilot_compute_corpus.rs:80-147` |
| Homebrew content breadth | Guard Stance (`guard_stance_shell`/`guard_stance_proof`) is the only authored package content the authoring format ships; no second package constructor exists. | `src/homebrew_authoring/mod.rs:106-117` |

### Release pipeline: CI coverage gaps

| Item | Status | Where (re-verified) |
|---|---|---|
| No concurrency guard on publish | `publish-tester-release.yml` declares no `concurrency:` block; two rapid pushes to `develop` can run two concurrent `finalize` jobs, each pushing to the shared `update-index` branch (mitigated only by each push being a fast-forward-or-fail `git push`, not by the workflow serializing runs itself). | `.github/workflows/publish-tester-release.yml` (no `concurrency` key anywhere in the file — re-confirmed by grep) |
| No tranche/5-scoped CI workflow | `tranche-3-ci.yml` is the only tranche-specific workflow present, and it is scoped to `tranche/3` only (refuses PRs targeting any other branch by design). No `tranche-5-ci.yml` or equivalent exists yet. | `.github/workflows/` (directory listing: only `tranche-3-ci.yml` matches `tranche*`) |

This doc is the first one every SD closure re-checks — a stub graduating to
real, tested behavior is the most common architectural-doc change, and it
must be reflected here before it is reflected anywhere else. See
[README.md](./README.md) §Maintenance contract for the update procedure.
