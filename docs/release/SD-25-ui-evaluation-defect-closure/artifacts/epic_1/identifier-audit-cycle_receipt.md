# Cycle 1.1 — Epic 1 Code-Side Identifier Cleanup / Criterion 1.1

- **Card ID:** (see kanban section of this receipt below / reported alongside)
- **Commit SHA:** `62c4785d098c3c288093f5076130ec44efdce23f` (pushed to `tranche/5-3`)
- **Files touched:** 50 files — see `git show --stat 62c4785`. Summary:
  - `apps/desktop/src/sd11/**` → `apps/desktop/src/testerWorkbench/**` (dir rename; `Sd11*`/`sd11_*` identifiers stripped: `Sd11TesterWorkbenchSurface` → `TesterWorkbenchSurface`, etc.)
  - `apps/desktop/src/boundary/loadSd13SupportStateMatrix.ts(+.test.ts)` → `loadSupportStateMatrix.ts(+.test.ts)` (filename only; exported symbols were already clean)
  - `apps/desktop/src/boundary/loadSd11UpdateAction.ts(+.test.ts)` → `loadUpdateAction.ts(+.test.ts)`; `loadSd11UpdateAction` → `loadUpdateAction`; `formatSd11WorkbenchBuildLabel` → `formatWorkbenchBuildLabel`
  - `apps/desktop/src/boundary/loadSd12ReleaseTruth.ts` → `loadReleaseTruth.ts`; `loadSd12ReleaseTruth` → `loadReleaseTruth`; `Sd12ReleaseTruth(Request|Snapshot|IssueCapture)` → `ReleaseTruth(Request|Snapshot|IssueCapture)`
  - `apps/desktop/src/sd15/` → `apps/desktop/src/operatorTriage/`; `buildSd15OperatorTriageDraft.ts(+.test.ts)` → `buildOperatorTriageDraft.ts(+.test.ts)`; all `Sd15*` types stripped; **also found and fixed** live `SCREAMING_SNAKE_CASE` consts missed by the prior partial pass: `SD15_PRIMARY_CLASSES` → `PRIMARY_CLASSES`, `SD15_OUTCOME_STATES` → `OUTCOME_STATES`, `SD15_REPRODUCTION_STATUSES` → `REPRODUCTION_STATUSES`, `SD15_AUTHORITY_DISCLAIMERS` → `AUTHORITY_DISCLAIMERS` (no external consumers; verified by grep)
  - `apps/desktop/src/sd22/` → `apps/desktop/src/releaseChecks/` (dir rename only; no `Sd22`/`sd22_` identifiers existed inside)
  - `apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` → `support_state_matrix_bridge.rs`; `main.rs`'s `mod`/`use` fixed accordingly; Tauri command name was already `load_support_state_matrix` (verified, unchanged)
  - `src/rules_core/support_state_matrix.rs`: stripped `SD13_`/`SD18_` prefixes from ~53 `const` declarations (e.g. `SD13_ROSTER_MATRIX_DOC` → `ROSTER_MATRIX_DOC`, `SD18_GNOME_HATRED_TEST` → `GNOME_HATRED_TEST`); **string values left untouched** (still cite real `tests/sd13_*.rs`/`tests/sd18_*.rs` files verbatim, per this criterion's explicit carve-out for test-traceability data)
  - `scripts/release/validate_manifest.py`, `scripts/release/write_release_manifest.py`: removed the raw `SD16-E4-F3b — owned by \`t_b7833349\`.` bundle+kanban attribution line from both module docstrings; surrounding doctrine-reminder prose kept
  - `docs/doctrine-external/identifier-discipline.md` + `~/workspace/governance/identifier-discipline.md`: added a "Documented exclusion class" section recording that real `tests/...`-file citations in comments/string-literals are not identifier violations, so a future audit pass doesn't re-litigate

- **Identifier audit result:** **NOT literally `OK_NO_BUNDLE_TAGS`** — full-repo RED scan (`git grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' apps/desktop/ apps/desktop/src-tauri/ src/ scripts/`) returns **404 residual hits**, all inside the documented exclusion class below (comment/string-literal citations to real, out-of-scope `tests/` files — `tests/` is deliberately excluded from the RED command's own scan paths). Zero hits are live identifiers (function/const/type/module/command/file names). Per-file breakdown:

  | File | Hits | Nature |
  |---|---:|---|
  | `src/rules_core/support_state_matrix.rs` | 351 | `const NAME: &str = "tests/sd13_*.rs + tests/sd18_*.rs + ..."` string-literal test-traceability citations (170× `sd13_`, 223× `sd18_`, 13× `sd19_` lowercase tokens inside string values only — no uppercase `SD13_`/`SD18_` identifier tokens remain) |
  | `src/rules_core/pilot_compute.rs` | 15 | `// SD18 widening (cycle-...), tests/sd18_rogue_level*.rs):` doc-comments |
  | `src/rules_core/rules_tables/beastiary1/mod.rs` | 8 | `//! ... tests/sd22_beastiary1_subset_0N_resolves.rs's header):` doc-comments |
  | `apps/desktop/src-tauri/src/corpus_fixtures.rs` | 2 | doc-comments citing `tests/fixtures/rules_core/sd19_seam_crb_*.txt` / `tests/sd19_seam_shapes_correctness.rs` |
  | `src/rules_core/contract.rs` | 2 | doc-comments citing `tests/sd20_contract_level_up_preview.rs` / `tests/sd20_tabletop_readiness_integration.rs` |
  | `src/rules_core/level_up/druid.rs` | 2 | doc-comments citing `tests/sd13_druid_base_attack_and_saves.rs` / `tests/sd18_druid_level15_widening.rs` |
  | `src/rules_core/rules_tables/{acg,apg}/*.rs` (18 files) | 1 each (18) | doc-comments citing `tests/sd22_{acg,apg}_class_*_resolves.rs` / `tests/sd24_*_class_coverage_audit.rs` |
  | `src/rules_core/rules_tables/crb/{equipment_tables,spell_list}.rs` | 1 each (2) | doc-comments citing `tests/sd24_equipment_coverage_audit.rs` |
  | `src/pcgen_import/lst_parser/{equipment,monster_stat_block,race_ability}.rs` | 1 each (3) | doc-comments citing `tests/sd17_b*.rs` |
  | **Total** | **404** | — |

  **Documented exclusion (per this criterion's explicit instruction, generalizing SD-24 carry-forward register item A9's "exclude known-legitimate content by name" precedent):** a doc-comment, `///`/`//!` line, or `const X: &str = "..."` string value in a non-`tests/` source file that names a real file under `tests/` by its exact path is test-traceability grounding data, not an identifier encoding spec domain. Rewriting these would sever real engineering value (which test proves which support-state-matrix row / rules-table entry) for zero compliance gain, since the cited `tests/` files are themselves explicitly out of the RED command's scan scope and out of this criterion's rename scope. Recorded in `~/workspace/governance/identifier-discipline.md` (new section) and `docs/doctrine-external/identifier-discipline.md` (TODO pointer) so a future audit treats these 28 files as already resolved.

  **Diff-scoped dual-audit gate (per `loop-instruction.md §6`, `BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `7f07d85`):** re-run against the full diff (committed + working tree) from `BASE_BRANCH`, filtering to non-test files: 459 raw pattern matches, of which 332 are removed (`-`) lines from the old, now-deleted bundle-tagged identifiers being cleaned up, 15 are added (`+`) lines and are all new `const NAME: &str = "tests/sd13_*..."` declarations in the documented-exclusion class (clean identifier, cited string value only), and the remainder are diff-metadata noise (`diff --git`/`rename from`/`@@` hunk-header context showing old signatures). **Zero net-new identifier violations introduced.**

- **Wired-integration audit result:** `OK_NO_TOKENS` — diff-scoped four-check grep (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` over the same diff, non-test files) returns zero hits.

- **Acceptance criterion:** Criterion 1.1 — Source-code identifier audit (`epic-breakdown.md` Epic 1): sweep `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, `scripts/` for bundle-tagged identifiers (`sd[0-9]+_`, `SD[0-9]+_`, `Sd[0-9]+`, raw kanban `t_<hex>` ids) and rename/strip them; `tests/` is out of scope.

- **Status:** complete

- **Notes / judgment calls:**
  1. Test-file citation strings and comments under 28 files (404 hits) are a documented, named exclusion — not a literal 0, but 0 net identifier violations. See table above.
  2. `scripts/release/validate_manifest.py` / `write_release_manifest.py` module docstrings carried a genuine violation (`SD16-E4-F3b — owned by \`t_b7833349\`.`) distinct from the citation class — fixed, not excluded.
  3. Found one additional live violation not called out in the original handoff: `SD15_PRIMARY_CLASSES`/`SD15_OUTCOME_STATES`/`SD15_REPRODUCTION_STATUSES`/`SD15_AUTHORITY_DISCLAIMERS` exported consts in `operatorTriage/buildOperatorTriageDraft.ts(+.test.ts)` — renamed, no external consumers.
  4. `apps/desktop/src/operatorTriage/buildOperatorTriageDraft.ts:12`'s comment citing `artifacts/sd15-e6-automation-helper-boundary-handoff-2026-06-30.md` left as-is: hyphenated real artifact path, doesn't match the RED pattern (no `sd15_` with trailing underscore), same citation class as the `tests/` exclusion.
  5. Pre-existing, unrelated failing test discovered and left alone (out of scope for this criterion): `apps/desktop/src/sd21/buildVersionTriple.test.ts` fails with `Cargo.toml version must match package.json version: expected 0.5.98, got 0.5.97` — confirmed via `git stash` that this fails identically on `tranche/5-3` HEAD before any of this criterion's changes (root cause: commit `e841156` bumped `package.json`/`tauri.conf.json` to `0.5.98` for SD-24 8.4 but not `Cargo.toml`). Not touched.
  6. Two unrelated uncommitted edits found in the working tree at commit time (`docs/release/SD-25-ui-evaluation-defect-closure/decisions.md` §13, `governance/loop-instruction-template.md §2.1` — a process-lesson about a prior session executing inline instead of dispatching) belong to a different, concurrently-running process; left unstaged/uncommitted by this cycle.

- **Test results:**
  - `apps/desktop`: `npm run typecheck` — clean (no errors). `npm test -- --run` — 57/60 files pass; 1 pre-existing unrelated failure (see Note 5); 2 skipped/other not applicable.
  - `apps/desktop/src-tauri`: `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` — clean (3 pre-existing dead-code warnings in `character_hub.rs`, unrelated).
  - Root crate: `cargo check --package codex` — clean. `cargo build --tests` — all 444 test binaries compile clean. `cargo test` — **4018 passed, 0 failed** across all `src/rules_core` and `src/pcgen_import` suites (full run, not scoped, since binaries were already built and execution was fast).

- **Discovery forwards:** none (the `SD15_*` const fix and the two scripts/release fixes were absorbed inside this same criterion's scope, not forwarded as new `## DISCOVERED` entries).

- **Next-cycle plan:** Epic 1 has no further criteria; the orchestrator proceeds to Epic 2 (2.1–2.5, operator pre-launch gates) per `progress.md`'s deterministic `## TODO` seed.
