# Status

> Scope: what is real, working product surface today across the whole repo, and what is stubbed, partially wired, or deferred — superseding the root README's "Current state" section.
> Last verified: **2026-09-20 against `tranche/16`, HEAD `b22ea9e113`** (SD-36 consolidation architecture-docs truth-up). **This pass trims the file substantially** (from ~1,400 lines to the shape below): the wave-by-wave corpus-coverage narrative (SD-29 through SD-31 wave 27, plus the SD-33 `unknown`-reaches-zero account) documented the history of an instrument — *src/bin/v06_work_inventory.rs*, *support_state_matrix.rs*, *reach_gate.rs* (italicized because SD-36 Epic B deleted all three; none of these paths exists in this checkout any more) — that SD-36 Epic B **deleted outright** (55,827 lines removed: `git show --stat` on the Epic B commit, or `docs/release/SD-36-consolidation/epic-breakdown.md`'s own per-file line counts). Per this doc set's own rule ("obsolete statements are REMOVED, not annotated as deprecated") and the maintenance contract's instruction that release-bundle narrative belongs under `docs/release/`/`docs/retro/`, not here, that history is removed from this file rather than kept as a growing appendix — it remains readable at `docs/retro/` and in the superseded commits' own diffs for anyone who needs the historical account. The one corpus-completion figure that is still current-state truth is the frozen public status site — see "Corpus coverage" below.
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
and `claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class`,
both still present in the file at this HEAD).

**What has changed since the last full pass**: several desktop-facing actions
this doc used to describe as session-local or inert are now real, persisted
mutations — see "Corrections since the last pass" below. The remaining gap is
narrower than it was, but the core finding is unchanged: this is a compute
and persistence engine with a proven, narrow character-creation ceiling and a
much wider character-*editing* surface (money, HP, bio, equipment purchase,
feats, traits, skills) once a character exists.

## Corrections since the last pass

- **The DM Toolkit is real**, not a `StubScreen.tsx` placeholder — a real
  encounter builder and DM-console export, backed by the real
  `Encounter::new`/`party_challenge_rating` compute this doc already listed
  as real. See [desktop-app.md](./desktop-app.md).
- **Skill-allocation and level-up dialog acceptance are now real, persisted
  mutations** (`set_skill_allocations`, `level_up_character` via
  `preview_level_up`), not in-memory-only `useState`/no-op closures.
- **The character sheet's `☰ Menu` has no bare no-ops left** — `Open`,
  `Recompute`, `Clone`, `Export`, `Print` are all wired to real handlers.
- **`StubScreen.tsx` is unreferenced dead code**, not a live placeholder for
  any current screen.
- **The Tauri command count is 75**, re-derived directly from
  `generate_handler![...]` (see [desktop-app.md](./desktop-app.md)); a saved
  character's on-disk bundle can now hold up to six files, not two (four new
  sidecar files: bio/money/HP/portrait — see [persistence.md](./persistence.md)).
- **The PCGen converter now lives behind a real crate boundary.** SD-36 Epic
  A moved the code formerly at *src/pcgen_import/* and *src/oracle_validation/* (neither path exists
  at that location any more) into
  `crates/codex-ingest`, a `[dev-dependencies]`-only crate from
  `apps/desktop/src-tauri`'s own `Cargo.toml` (verified: `codex-ingest` does
  not appear under that manifest's `[dependencies]` section) — the "PCGen
  wall" is now a real crate/dependency-graph boundary, not only a residue
  gate over string patterns. See [corpus-ingest.md](./corpus-ingest.md).
- **`src/rules_core/pilot_compute/mod.rs` is split into per-class submodule
  files** (SD-36 Epic C1) rather than one 88,000-line file; the whole
  directory is ~100,900 lines across many files today (`find … -name '*.rs' |
  xargs wc -l`), the largest single file ~6,160 lines. Call sites are
  unaffected (`pilot_compute::` paths still resolve the same way).

## Corpus coverage

**The corpus-completion instrument this section used to report against —
`v06_work_inventory`/`docs/work-inventory.json`'s per-unit classifier,
*support_state_matrix.rs*, and *apps/desktop/src-tauri/src/reach_gate.rs* (all three deleted) —
was deleted outright by SD-36 Epic B**, not refactored or superseded by a
successor instrument. `docs/work-inventory.json` itself is now frozen
(unchanged content, recorded once at `docs/work-inventory.FROZEN.md`) and is
no longer regenerated by anything.

**The one live, current-state corpus-completion figure** is the frozen
public status snapshot:

```
$ python3 -c "import json;print(json.load(open('site/status-data.json'))['overall'])"
{'done': 49450, 'partial': 0, 'not_started': 0, 'denominator': 49450, 'pct': 100.0, ...}
```

`scripts/site/check_frozen_status.py --check` gates this file never drifting; per-book detail lives
under `site/status-data/`, and this is the same JSON the public `campaign-codex.org` site (deployed by
[release-pipeline.md](./release-pipeline.md)'s `deploy-site.yml`) renders. **This is a statement about
the corpus reaching a rendered sheet line under the sheet rule** (`docs/release/SD-35-corpus-sheet-completion/decisions.md
§1`: a record is done when it renders a line a player could write — one final number, dice in final
form, or the rule's own words; "the engine cannot model X" is a number to report, not an exemption).
**It is not a claim that every class/level combination reaches a fully `Computed` receipt** — the
Posture section above is the separate, narrower chassis-coverage figure, and the two must not be
conflated.

The detailed wave-by-wave history of how this number was reached (SD-29 through SD-33, dozens of
integration cycles) is retired along with the instrument that produced it; it is not reproduced here.
Anyone who needs it can read the superseded commits under `docs/release/SD-29-*` through
`docs/release/SD-33-*` and `docs/retro/`.

## Real today

| Area | What works | Where |
|---|---|---|
| Corpus-ingest pipeline | `.pcc`/`.lst` parsing through canonical `SourcePackageContent` projection, now behind the `crates/codex-ingest` crate boundary (dev-dependency only from the desktop shell) | [corpus-ingest.md](./corpus-ingest.md) |
| Pilot compute + boundary contract | `compute_pilot_base_chassis` → `compute_pilot_with_corpus` → `to_pilot_receipt` → `printed_sheet_cell_map`, fail-honest throughout; `pilot_compute/` is now split into per-class submodule files | [rules-engine.md](./rules-engine.md) |
| Per-domain engines | Spellbook, skill allocation, feat prerequisites, equipment effects, damage total, level-up | [rules-engine.md](./rules-engine.md) |
| Rule-table catalogs | Dozens of `RuleSetId` variants across CRB, APG, ACG, Bestiary 1, and many further Paizo books — see [rules-data-tables.md](./rules-data-tables.md) for the current enumeration and per-book ceiling detail (not re-derived in this pass; that doc's own maintenance owns this figure) | [rules-data-tables.md](./rules-data-tables.md) |
| Character Hub | Create, load, clone, portrait upload/load/delete, JSON export, recompute, plus (new since the last full pass) equipment purchase/attach, feat/trait selection, skill allocation, bio/money/HP tracking — all real engine compute + real persistence | [desktop-app.md](./desktop-app.md) |
| DM Toolkit | Real encounter builder + DM console export (`rate_encounter`, `export_dm_console`), consuming the real `Encounter`/party-CR compute below | [desktop-app.md](./desktop-app.md) |
| Rule-system adapter seam (hub-of-hubs) | `RuleSystemAdapter` trait is the object-safe seam three commands (`append_to_character`/`recompute_character`/`re_save_character`) dispatch through on a `rule_system_id`: `"pf1"` resolves to the real `Pf1Adapter`; any other id resolves to the governed `StubAdapter` (registered exception 0002, `docs/governance/wired-integration-stubs-registry.md`). All other character-mutation commands call PF1 free functions directly and do not go through this seam | [desktop-app.md](./desktop-app.md) §"Rule-system adapter seam" |
| Corpus-ingest diagnostic | `corpus_ingest_diagnostic` Tauri command reports real ingested-record-kind counts per book, counted from the tables actually compiled into the binary | [desktop-app.md](./desktop-app.md) |
| PCGen runner scaffolding | `scripts/pcgen-run-character.sh` drives the real headless PCGen Gradle batch-export; wrapped by `oracle_validation::pcgen_runner::run_pcgen_character` (now under `crates/codex-ingest`) | [testing.md](./testing.md) |
| Campaign manager (local) | Create/edit/list campaigns and their assets, backed by `CampaignStore` on disk; nonce-based conflict detection with local-wins + preserved-conflict-copy resolution. `localStorage` remains the actual frontend source of truth; `write_campaign_drive_artifacts` is a one-way write-through mirror | [persistence.md](./persistence.md) |
| Update eligibility / restore / verify | `is_install_eligible`, `perform_restore_previous`, `verify_relaunch_artifact` — all real, tested Tauri commands | [update-and-feedback.md](./update-and-feedback.md) |
| Feedback composers + browser handoff | Bug/enhancement draft composition, evidence capture/redaction, the governed GitHub-issue browser handoff, and a controlled-defect SHA-256 mismatch test harness | [update-and-feedback.md](./update-and-feedback.md) |
| Release pipeline | Multi-platform publish, dual manifest validation, channel-index push, branch-promotion gates, plus a separate `deploy-site.yml` lane for the public status site | [release-pipeline.md](./release-pipeline.md) |
| IPC bridge liveness | `load_backend_health` returns the real crate version and compile-time git SHA | [desktop-app.md](./desktop-app.md) |
| Homebrew authoring workbench | The Guard Stance proof package's validate/persist/preview round trip, read-only bridged to the desktop tester workbench | [homebrew-and-oracle.md](./homebrew-and-oracle.md) |
| Encounter difficulty / party CR compute | `Encounter::new` and `party_challenge_rating` are real, grounded compute, now reachable through the real DM Toolkit UI (see above — no longer blocked behind a stub screen) | [rules-engine.md](./rules-engine.md) |
| Fighter+Wizard multiclass base-chassis dispatch | `compute_multiclass_base_chassis` grounds BAB/save stacking + per-class named-feature explanations for any Fighter+Wizard split, total level 1-10 — grounds the base-chassis layer only, not a full `Computed` receipt end-to-end | [rules-engine.md](./rules-engine.md) §"Multiclass base-chassis dispatch" |
| Repo-resident JSON corpus cache | `data/corpus/<book>/**/*.json` — see [rules-data-tables.md](./rules-data-tables.md) for the current book/file-count figures (not re-derived here); a sanitized runtime mirror of the same data ships in the desktop installer as `resources/corpus_bundle/` (64 MiB, 14,029 JSON files) — see [desktop-app.md](./desktop-app.md) |

## Known gaps and stubs, by area

### Desktop app: character sheet and update actions

| Item | Status | Where (re-verified) |
|---|---|---|
| `perform_install` | Always returns `Err("...not wired: downloading the AppImage artifact requires an HTTP client...")`; its TS caller `installAction.ts::performInstall` has zero production call sites in `Ui.tsx`'s composed panels. Doubly inert. | `apps/desktop/src-tauri/src/update/transaction.rs`; [update-and-feedback.md](./update-and-feedback.md) |
| `perform_retention_sweep` | Real, tested body (`perform_retention_sweep_impl`), but still not in `main.rs`'s `generate_handler!` list — unreachable from the frontend. | `apps/desktop/src-tauri/src/update/transaction.rs`; `apps/desktop/src-tauri/src/main.rs` |
| `drive_list_campaigns` / `drive_load_campaign` / `drive_save_campaign` / `drive_delete_campaign` | Registered and unit-tested, but no frontend file invokes any of them — `campaignModel.ts` uses `localStorage` as the real source of truth; only `write_campaign_drive_artifacts` (one-way mirror) is called. | `apps/desktop/src-tauri/src/campaign_drive.rs`; `apps/desktop/src/campaign/campaignModel.ts` |
| `append_to_character` / `re_save_character` | Registered and unit-tested, but no `boundary/*.ts` wrapper and zero `invoke()` call sites exist anywhere in `apps/desktop/src`. Their sibling `recompute_character` **is** wired to a real UI affordance (`CharacterSheet.tsx`'s `☰ Menu` → Recompute). | `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs`, `.../reSaveCharacter.rs` |
| Character-sheet bio fields | Persisted, not session-local — corrected since the last pass. `update_character_bio`/`load_character_bio` round-trip through `bio.json`. | [persistence.md](./persistence.md) |
| DM Toolkit UI | Real, not a stub — corrected since the last pass. See "Corrections" above. | [desktop-app.md](./desktop-app.md) |
| Skill allocation / level-up dialog acceptance | Real, persisted mutations — corrected since the last pass. | [desktop-app.md](./desktop-app.md) |
| Campaign conflict merge | Conflict detection is real and tested (nonce-based); resolution is local-wins with both copies preserved under `conflicts/<timestamp>/` — there is no merge UI. | [persistence.md](./persistence.md) §"Conflict detection" |

### Core engine: compute coverage and proof surfaces

| Item | Status | Where (re-verified) |
|---|---|---|
| Class/level compute coverage | Only single-class Fighter levels 1-3 reach `Computed` for any race. | `apps/desktop/src-tauri/src/character_hub.rs` (tests named in Posture above) |
| Oracle-parity comparator | The in-crate harness (`oracle_validation::comparator::compare`, now under `crates/codex-ingest`) exists and is tested — normalizes PCGen output, reports per-dimension matches/mismatches, renders a real `PASS`/`FAIL` report. A *passing* end-to-end parity claim is a separate, further-out question this doc does not re-verify this pass. | `crates/codex-ingest/src/oracle_validation/` |
| Bestiary 1 monster parser | `monster_stat_block.rs`'s row parser is still unwired — no `ParsedLstRecord`/`SourceContentPayload` variant references it outside its own test file (0 hits, re-confirmed this pass). Bestiary 1 table content is hand-transcribed, not parsed through the canonical-IR path. | grep across `src/rules_core/source_content.rs` / ingest converter |
| Failure-owner classifier | `pilot_failure.rs`'s `primary_owner` still only ever returns `OracleGap` (on `Computed`) or `EngineFlaw` (on `Blocked`) — the other two `PrimaryOwner` variants remain unreachable from the current receipt surface (re-confirmed this pass). | `src/rules_core/pilot_failure.rs` |
| Spellbook magnitude — a disconnected twin | `contract::build_pilot_receipt` wires `spellbook::compute_spellbook_coverage` into `PilotReceipt.spellbook`, but nothing in the shipped desktop app reaches it (`grep -rn build_pilot_receipt apps/desktop/src-tauri/src` still returns 0 hits, re-confirmed this pass). The app instead gates on `pf1_adapter::resolve_unified_pilot_snapshot`. | `src/rules_core/contract.rs`; `apps/desktop/src-tauri/src/pf1_adapter.rs` |
| Per-item corpus equipment stats | `pilot_compute_corpus.rs`'s `DerivedEquipmentStats` is still constructed via `default()` at its call sites — real per-item stats are computed separately by `equipment_effects.rs` (re-confirmed this pass). | `src/rules_core/pilot_compute_corpus.rs` |
| Homebrew content breadth | Guard Stance is still the only authored package content the authoring format ships; no second package constructor exists. | `src/homebrew_authoring/mod.rs` |
| Future-state books (`book_stub`) | **33** out-of-scope Paizo books (`data/stubs/*.json`, up from 21 at the last pass — `find data/stubs -name '*.json' | wc -l`) are registered as honest future-state placeholders — each carries only identity/registration metadata and no rule data. | `data/stubs/*.json`; `docs/governance/wired-integration-stubs-registry.md` |

### Release pipeline: CI coverage gaps

| Item | Status | Where (re-verified) |
|---|---|---|
| No concurrency guard on publish | `publish-tester-release.yml` still declares no `concurrency:` block; two rapid pushes to `develop` can run two concurrent `finalize` jobs, each pushing to the shared `update-index` branch. | `.github/workflows/publish-tester-release.yml` |
| No tranche/16-scoped CI workflow | `tranche-3-ci.yml` remains the only tranche-specific workflow, scoped to `tranche/3` only. No `tranche-16-ci.yml` or equivalent exists. | `.github/workflows/` (only `tranche-3-ci.yml` matches `tranche*`) |
| `check-release-manifest.yml`'s stale path globs | Two of six `paths:` globs (`apps/desktop/src/sd16/**`, `apps/desktop/src/sd17/**`) resolve to nothing in this checkout; the other four were fixed since the last pass. | [release-pipeline.md](./release-pipeline.md) §"Promotion gates chain" item 5 |

This doc is the first one every SD closure re-checks — a stub graduating to
real, tested behavior is the most common architectural-doc change, and it
must be reflected here before it is reflected anywhere else. See
[README.md](./README.md) §Maintenance contract for the update procedure.
