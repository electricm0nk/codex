# Status

> Scope: what is real, working product surface today across the whole repo, and what is stubbed, partially wired, or deferred — superseding the root README's "Current state" section.
> Last verified: 2026-07-23 against tranche/5-4 (SD-26 Epic 6 closure)
> Maintenance: pre-PR truth-up cycle per [README.md](./README.md) §Maintenance contract — fires before every PR via the architecture-truth-up skill

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
| Four rule-table catalogs | CRB (full), APG (6/6 classes), ACG (10/10 classes), Bestiary 1 (41 monsters across 8 subsets, plus its own small equipment table) | [rules-data-tables.md](./rules-data-tables.md) |
| Character Hub | Create, load, clone, portrait upload/load/delete, JSON export, recompute — all real engine compute + real persistence | [desktop-app.md](./desktop-app.md) |
| Rule-system adapter seam (hub-of-hubs) | `RuleSystemAdapter` trait is the object-safe seam the Character Hub's mutation commands (`append_to_character`/`recompute_character`/`re_save_character`) dispatch through on a `rule_system_id`: `"pf1"` resolves to the real `Pf1Adapter` (wraps the extracted PF1 free functions); any other id resolves to the governed `StubAdapter`, which reports an honest "not yet implemented" diagnostic — never fabricated data (registered exception 0002 in `docs/governance/wired-integration-stubs-registry.md`) | [desktop-app.md](./desktop-app.md) §"Rule-system adapter seam" |
| Corpus-ingest diagnostic | `corpus_ingest_diagnostic` Tauri command reports the real ingested state (record-kind counts + last-touched git timestamp) of every populated `rules_tables` book, counted from the tables actually compiled into the binary — reachable from the Character Hub landing via the `CorpusIngestDiagnosticPanel`. Sketch-scoped to four fields; SD-26 fans out the full status table | [desktop-app.md](./desktop-app.md) |
| PCGen runner scaffolding | `scripts/pcgen-run-character.sh` drives the real headless PCGen Gradle batch-export; `scripts/pcgen-normalize-output.py` normalizes its XML into the golden-fixture comparison shape. Real, invocable, smoke-tested end-to-end (`tests/pcgen_runner_smoke.rs`), and now wrapped into one Rust call by `oracle_validation::pcgen_runner::run_pcgen_character` (SD-26 Epic 2). The in-crate comparator that consumes its output now exists too — see the oracle-parity comparator row below | [testing.md](./testing.md) |
| Campaign manager (local) | Create/edit/list campaigns and their assets, backed by `CampaignStore` on disk; nonce-based conflict detection with local-wins + preserved-conflict-copy resolution | [persistence.md](./persistence.md) |
| Update eligibility / restore / verify | `is_install_eligible`, `perform_restore_previous`, `verify_relaunch_artifact` — all real, tested Tauri commands | [update-and-feedback.md](./update-and-feedback.md) |
| Feedback composers + browser handoff | Bug/enhancement draft composition, evidence capture/redaction, and the governed GitHub-issue browser handoff | [update-and-feedback.md](./update-and-feedback.md) |
| Release pipeline | Multi-platform publish, dual manifest validation, channel-index push, branch-promotion gates — the machinery is real and has shipped releases; the `test` job's frontend-typecheck step passes cleanly (see [testing.md](./testing.md)) | [release-pipeline.md](./release-pipeline.md) |
| Support-state matrix | 34-row typed truth ledger, read-only bridged to the desktop tester workbench | [support-state-matrix.md](./support-state-matrix.md) |
| IPC bridge liveness | `load_backend_health` returns the real crate version and compile-time git SHA; reaching it at all proves the Tauri bridge is alive | [desktop-app.md](./desktop-app.md) |
| Homebrew authoring workbench | The Guard Stance proof package's validate/persist/preview round trip, read-only bridged to the desktop tester workbench | [homebrew-and-oracle.md](./homebrew-and-oracle.md) |
| Encounter difficulty / party CR compute | `Encounter::new` and `party_challenge_rating` are real, grounded compute — but see the DM Toolkit UI row below | [rules-engine.md](./rules-engine.md) |
| Fighter+Wizard multiclass base-chassis dispatch | `compute_multiclass_base_chassis` grounds BAB/save stacking + per-class named-feature explanations for any Fighter+Wizard split, total level 1-10, deterministically proven at every level and both transition directions (SD-24 Epic 5) — but this grounds the base-chassis layer only, not a full `Computed` receipt end-to-end (see the Class/level compute coverage row below) | [rules-engine.md](./rules-engine.md) §"Multiclass base-chassis dispatch" |
| Repo-resident JSON corpus cache | `data/corpus/<book>/**/*.json` holds a Shape-B JSON dump of the four in-scope books' already-landed `rules_tables` state (SD-26 Epic 3): core_rulebook (3326 records: 2663 equipment + 652 spell + 11 class), advanced_players_guide (641), advanced_class_guide (423), beastiary (45). Written by one-off codegen tools (`src/bin/sd26_gen_core_rulebook_cache.rs`, `src/bin/gen_cache_apg.rs`, `src/bin/gen_cache_acg.rs`, `src/bin/gen_cache_beastiary.rs`) driving `src/rules_core/cache_gen/`; each generator *dumps* the compiled Rust module's runtime state and never re-parses raw LST for values (only for line-number citations). Round-trip-tested by `tests/sd26_cache_core_rulebook.rs` and its `apg`/`acg`/`beastiary` siblings | [rules-data-tables.md](./rules-data-tables.md) |
| CRB/APG/ACG/Bestiary 1 equipment + spell record ingestion | 100% record coverage (equipment and spells) across all four books; `weight`/`description` fields on every book's `EquipmentTableEntry`, populated toward each book's honest ceiling. SD-25 Epic 7 raised those ceilings via cited web second-source passes: CRB `description` 2021/2977 (67.9%, was 61.2%); APG `description` 331/338 (was 0% — the APG corpus itself carries no `DESC:` token, every value identity-matched from `aonprd.com`/`d20pfsrd.com`); APG spell full-text 284/297 (was 261); Bestiary 1 equipment newly ingested at 4/4 records with full cost/weight/description. Remaining gaps are honest, undispatched residue, not silently accepted (per-book counts asserted exactly by `tests/sd24_equipment_coverage_audit.rs` / `tests/sd24_equipment_field_completion.rs`) | [rules-data-tables.md](./rules-data-tables.md) §"Equipment/spell content completeness" |

## Stubbed / partially wired / deferred today

Grouped by the plane each item lives in. Every row was re-verified directly
against the cited source, not carried over from a sibling doc unchecked.

### Desktop app: character sheet and update actions

| Item | Status | Where (re-verified) |
|---|---|---|
| `perform_install` | Always returns `Err("...not wired: downloading the AppImage artifact requires an HTTP client...")`; its TS caller `installAction.ts::performInstall` has zero production call sites — `Ui.tsx`'s `handleInstall` is a documented no-op. Doubly inert. | `apps/desktop/src-tauri/src/update/transaction.rs:763-771`; `apps/desktop/src/sd16/update/Ui.tsx:110-117` |
| `perform_retention_sweep` | Real, tested body (`perform_retention_sweep_impl`), but not in `main.rs`'s `generate_handler!` list — unreachable from the frontend. | `apps/desktop/src-tauri/src/update/transaction.rs:817`; `apps/desktop/src-tauri/src/main.rs:113-140` |
| `drive_list_campaigns` / `drive_load_campaign` / `drive_save_campaign` / `drive_delete_campaign` | Registered in `generate_handler!` and unit-tested, but no frontend file invokes any of them (confirmed: zero grep hits across `apps/desktop/src`). `campaignModel.ts` uses `localStorage` as the real source of truth; only `write_campaign_drive_artifacts` (one-way mirror) is called. | `apps/desktop/src-tauri/src/main.rs:132-135`; `apps/desktop/src/campaign/campaignModel.ts` |
| `append_to_character` / `re_save_character` | Registered in `generate_handler!` and unit-tested (SD-24 Epic 7, criteria 7.1/7.3), but no `boundary/*.ts` wrapper and zero `invoke()` call sites exist anywhere in `apps/desktop/src` — same "registered-but-unreachable" shape as the `drive_*` row above. (Their sibling `recompute_character` was wired to a real UI affordance by SD-25 Epic 3 — see the Real-today Character Hub row and `desktop-app.md`; these two were not, because SD-25 Criterion 3.5's own file-touch grant only wired the recompute call site.) | `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs`, `apps/desktop/src-tauri/src/characterHub/reSaveCharacter.rs`; `apps/desktop/src-tauri/src/main.rs` (registration) |
| Level-up acceptance | `LevelUpDialog`'s `onAccept` in `CharacterSheet.tsx` is an empty closure with a comment: "accepting is a no-op today." Nothing is persisted or recomputed. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`LevelUpDialog` `onAccept`) |
| Skill-allocation acceptance | `SkillAllocationDialog`'s own header comment: "Accepting only updates in-memory state (`onAccept`) — there is no backend [persistence]." Wired to a plain `useState` setter, lost on sheet close. | `apps/desktop/src/characterHub/SkillAllocationDialog.tsx` (header comment) |
| Character-sheet bio fields | Alignment/deity/sex/age/height/weight/hair/eyes are explicitly session-local; no persisted schema slot exists yet. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`DetailsPanel`) |
| Sheet `☰ Menu` | Graduated (SD-25 Epic 3, register A4): `Open` and `Clone` are now wired to real behavior, and `Save` — which had nothing session-local to persist — is replaced by a real `Recompute` action that calls `recompute_character` through the active rule-system adapter. `Print` (`window.print()`) is unchanged. No menu item is a bare no-op today. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`menuItems`, `handleRecompute`) |
| Campaign conflict merge | Conflict detection is real and tested (nonce-based); resolution is local-wins with both copies preserved under `conflicts/<timestamp>/` — there is no merge UI. | [persistence.md](./persistence.md) §"Conflict detection" |
| DM Toolkit UI | The Landing screen's "DM Toolkit" action routes to `StubScreen.tsx`, a generic "not built yet" placeholder — it does not call `encounters.rs`/`party_cr.rs` even though that compute is real (see the Real-today table above). | `apps/desktop/src/characterHub/CharacterHubPage.tsx:93-99`; `apps/desktop/src/characterHub/StubScreen.tsx` |

### Core engine: compute coverage and proof surfaces

| Item | Status | Where (re-verified) |
|---|---|---|
| Class/level compute coverage | Only single-class Fighter levels 1-3 reach `Computed` for any race; Wizard level 1 is closest but still blocked on spellbook/school-power diagnostics. | `apps/desktop/src-tauri/src/character_hub.rs:949-954` (test) |
| Oracle-parity comparator | **Graduated (SD-26 Epic 2): the in-crate harness now exists and is tested.** `oracle_validation::comparator::compare` aligns a normalized PCGen output against Codex's selected dimensions and reports per-dimension matches/mismatches; `normalization` reduces raw PCGen text into the comparator's input shape; `parity_report` renders a real `PASS`/`FAIL` `parity_report_<case-id>.md`; `pcgen_runner::run_pcgen_character` wraps the two real PCGen scripts into one Rust call. What is still deferred is a *passing* parity claim: the pilot end-to-end run (`tests/sd26_pilot_case_verification.rs`) currently produces a real **FAIL** — two genuine `skill.selected_modifier.{climb,swim}` mismatches because `pilot_compute::compute_ability_modifiers` does not yet apply the chosen racial ability bonus (the open CG-03 blocker). `SelectedParityDimensions` still carries only a `Computed` `ClaimTierFloor` (no `OracleChecked` variant), so no fixture can yet assert oracle-checked parity. The harness is real; a green parity verdict is not, pending CG-03. | `src/oracle_validation/comparator.rs`; `src/oracle_validation/normalization.rs`; `src/oracle_validation/parity_report.rs`; `src/oracle_validation/pcgen_runner.rs`; `tests/sd26_pilot_case_verification.rs`; `src/rules_core/pilot_compute.rs` (CG-03) |
| Bestiary 1 monster parser | `monster_stat_block.rs`'s row parser is fully unwired: no `ParsedLstRecord`/`SourceContentPayload` variant exists for it, and its only callers outside its own module are its own test file. Bestiary 1 table content is hand-transcribed, not parsed through the canonical-IR path. | `src/pcgen_import/lst_parser/monster_stat_block.rs`; zero references in `ir_converter.rs`/`source_content_payload.rs`/`source_content.rs` |
| Failure-owner classifier | `pilot_failure.rs`'s `primary_owner` only ever returns `OracleGap` (on `Computed`) or `EngineFlaw` (on `Blocked`); `ModelFlaw`/`ImporterFlaw`/`UiGap` are unreachable from the current receipt surface. | `src/rules_core/pilot_failure.rs:61-66` |
| Per-item corpus equipment stats | `pilot_compute_corpus.rs`'s `DerivedEquipmentStats` is always `default()` — a permanent placeholder there; real per-item stats are computed separately by `equipment_effects.rs`. | `src/rules_core/pilot_compute_corpus.rs:80-147` |
| Homebrew content breadth | Guard Stance (`guard_stance_shell`/`guard_stance_proof`) is the only authored package content the authoring format ships; no second package constructor exists. | `src/homebrew_authoring/mod.rs:106-117` |
| Future-state books (`book_stub`) | 21 out-of-scope Paizo books (`data/stubs/*.json`, e.g. `bestiary_4`, `ultimate_magic`) are registered as honest `book_stub` future-state placeholders (SD-26 Epic 4) — each carries only `book_id`/`book_name`/`planned_resolution_bundle`/`registered_at`, `content_kind_counts: null`, and no rule data. They are declared, not implemented: the registry (`docs/governance/wired-integration-stubs-registry.md`, 21 `book_stub` entries) tracks them so the corpus-diagnostic surface can name a book as "known but unbuilt" rather than silently omit it. Concrete rule-system content lands in SD-27+. | `data/stubs/*.json`; `docs/governance/wired-integration-stubs-registry.md` |

### Release pipeline: CI coverage gaps

| Item | Status | Where (re-verified) |
|---|---|---|
| No concurrency guard on publish | `publish-tester-release.yml` declares no `concurrency:` block; two rapid pushes to `develop` can run two concurrent `finalize` jobs, each pushing to the shared `update-index` branch (mitigated only by each push being a fast-forward-or-fail `git push`, not by the workflow serializing runs itself). | `.github/workflows/publish-tester-release.yml` (no `concurrency` key anywhere in the file — re-confirmed by grep) |
| No tranche/5-scoped CI workflow | `tranche-3-ci.yml` is the only tranche-specific workflow present, and it is scoped to `tranche/3` only (refuses PRs targeting any other branch by design). No `tranche-5-ci.yml` or equivalent exists yet. | `.github/workflows/` (directory listing: only `tranche-3-ci.yml` matches `tranche*`) |

This doc is the first one every SD closure re-checks — a stub graduating to
real, tested behavior is the most common architectural-doc change, and it
must be reflected here before it is reflected anywhere else. See
[README.md](./README.md) §Maintenance contract for the update procedure.
