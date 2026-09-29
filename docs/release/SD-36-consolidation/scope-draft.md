---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Scope

## Operator ruling — Goal (2026-09-15)

**CI green today; then a tight, clean codebase before UI work and before Starfinder; nothing of PCGen in live code, enforced by the build not by a script; disk reclaimed.**

## Seven operator decisions (verbatim-binding)

| # | decision |
|---|---|
| D1 | PCGen tool side (converter + oracle) is KEPT for Starfinder but walled off in its own crate so the desktop can never link it. Close the two gate blind spots. |
| D2 | Hotfix PR to `develop` now. Then cut `tranche/16` (0.16.0) as SD-36 "consolidation". Operator merges both. |
| D3 | Freeze a static snapshot of the PF1e status page; retire `v06_work_inventory`, `support_state_matrix` + desktop bridge, `reach_gate`, and everything that exists only to feed them. |
| D4 | Disk: delete the 234G SD-33 scratch dir and the 38G `~/cargo-targets/w31-integrate`. Keep `target/`. |
| D5 | Public page freezes at **100%**: change `scripts/site/build_public_status.py` to count every unit with the inventory's own DONE vocabulary (overrides the SD-31 public-denominator rule). |
| D6 | Residue gate: add the `lst_file` identifier pattern now and rename the field; fix the one player-visible `.lst` message with a test. Do NOT add a `\.lst\b` pattern; the 8,592 table-citation strings are provenance and burn down when rules tables become a data package (Starfinder bundle row). |
| D7 | Remove the three dead v0.6 dashboard cron lines (`dashboard-watchdog.sh`, `dashboard-renderer.sh`, `swarm-status-heartbeat.py`). Keep `reclaim.sh`. |

## Epic order: B → A → C → D

**Why this order:**

- **B first** deletes 55k+ lines that A would otherwise move, reducing A's scope.
- **A before C** because C's `pilot_compute` split and test rewrite would collide with A's test-module eviction.
- **D last** is closure: architecture-docs refresh, retrospective, PR.

## Epic breakdown

### Epic B — Freeze the PF1e status page, retire the producers (one batch, one pass)

**55,827 lines deleted.** Freeze the PF1e status page at 100% complete (49,450 of 49,450 units). Retire `v06_work_inventory`, `support_state_matrix`, `reach_gate`, and all supporting machinery (tests, desktop bridge, cron jobs, docs).

**Acceptance criteria:**

1. New gate `scripts/site/check_frozen_status.py` + `scripts/tests/test_check_frozen_status.py` asserting frozen state (100.0%, 49,450 units, 0 partial, 0 not_started).
2. `scripts/site/build_public_status.py` rewritten per D5; `site/status-data.json` regenerated and frozen.
3. Whole files deleted (8 binaries, 2 support modules, 1 desktop bridge, 1 desktop tool, 8 test files, 4 desktop UI components, 1 script, 1 doc, 1 fixture bin).
4. Desktop UI: `SupportDebtPanel`, `BreadthClaimAuditPanel` removed; `supportStateTone` and wiring deleted.
5. `scripts/verify.sh`: 6 stages dropped (site-dashboard-*), add `site-status-frozen-check` to BOTH stage sets. Baselines updated.
6. `docs/work-inventory.json` stays; add `docs/work-inventory.FROZEN.md` sidecar.
7. `scripts/observer/pf1e_dashboard_producer.py` extraction (doneness.py) or deferral decision recorded.
8. Docs updated: `site/dashboard/README.md`, `.claude/skills/publish-site/SKILL.md`, governance and architecture files touched.
9. ONE pass: `cargo test --locked --no-run -j 6` at root and desktop; `bash scripts/verify.sh`. Commit, push.

---

### Epic A — PCGen wall: crate `crates/codex-ingest` (one batch, one pass)

**Root workspace gains `crates/codex-ingest`, desktop lists it dev-dep-only. Enforce via build.** Move PCGen machinery (oracle + converter) into dedicated crate so the desktop can never link it. Close two gate blind spots: `lst_file` identifier and `codex_ingest` runtime check.

**Acceptance criteria:**

1. Residue gate tests green (`lst_file` pattern hits, `codex_ingest` in test cfg is OK, in live root is a hit).
2. `scripts/pcgen_residue_gate.py` adds `lst_file` to patterns; no `\.lst\b` pattern.
3. Scaffold `crates/codex-ingest/{Cargo.toml,src/lib.rs}` with public modules and `repo_root()`.
4. Move 47 tool bins and ~82 tool tests into the crate via `git mv`.
5. Path rewrites: `crate::` → `codex::`, `codex::pcgen_import` → `codex_ingest::pcgen_import`, `CARGO_MANIFEST_DIR` → `repo_root()` (zero remaining).
6. Move `run_bar_check` bin into crate.
7. Evict 36 `src/rules_core` `#[cfg(test)]` modules (161 lines): route to either moved tests or fixture-ized JSON.
8. Rename `SourceRef.lst_file` → `source_path` (57 test files affected); regenerate desktop fixtures; fix one player-visible `.lst` message.
9. Desktop: codex-ingest as dev-dep; rewrite 9 test imports.
10. `scripts/verify.sh`: add `-p codex-ingest` stages, new stage `ingest-full`, new stage `crate-wall` (both stage sets). Update baselines.
11. CI: `.github/workflows/` and `tranche-3-ci.yml` switch to `--workspace` testing.
12. Refresh `Cargo.lock` files.
13. ONE pass: `cargo test --locked --workspace --no-run -j 6` at root, desktop; `bash scripts/verify.sh`. Wall stage green; residue gate 0/0. Commit, push.

---

### Epic C — Bloat cuts (two batches, two passes)

**~163k lines touched.** Two passes: C1 (source refactor) + C2 (test rewrite).

**Pass C1 — source side:**

1. Split `pilot_compute/mod.rs` (88,828 lines) into ~36 submodules by classifier (class files, race_seams, combat, feat_pillars, skills, spellcasting, pools, companion, untabled features, prestige class features, dispatch, unchained). Moves only; no dedup. Largest file ≈ 6,600 lines after split.
2. Consolidate path helpers: new `src/support/paths.rs` with `repo_root`, `corpus_root`, `corpus_root_if_set`, `pcgen_corpus_root`, `find_json_files`; delete local copies (8 sites).
3. Clippy lock: `clippy_one_crate()` adds `-- -D warnings`.
4. Proof: `cargo test --locked -- --list` before/after IDENTICAL; `bash scripts/verify.sh`. Commit, push.

**Pass C2 — tests side:**

1. Table-driven rewrite of `tests/sd18_widening/` (sd13_progression/: keep rosters + per-row files; add `rows.rs` with `const ROWS: &[Row]` and macro emitting per-row test; bespoke tests (~770) stay. Entry count byte-identical. Expected ~75k → ~26k lines.
2. Delete 59 local path-helper copies in `tests/`.
3. Move `tests/sd16-e5-f1/test_branch_promotion_guard.sh` to `tools/ci/`, update references.
4. `#[ignore]` audit: keep 65 oracle-gated tests; delete 2 evidence dumps; fix one reason string.
5. Proof: `--list` diff IDENTICAL; `bash scripts/verify.sh`. Commit, push.

**Note:** `rules_tables/**` (181,797 lines) stays as Rust data; becomes data package in Starfinder bundle row.

---

### Epic D — Closure

1. Refresh `docs/architecture/` for every touched topic.
2. `docs/retro/`: events for each retirement; SD-36 retrospective from `scripts/retro.py summary`.
3. `docs/release/SD-36-consolidation/release-notes.md`: re-derive baseline table before/after.
4. Graphify against the FINAL tree, then `gh pr create --base develop` for `tranche/16`. Operator merges.

## Verification (end to end)

1. Hotfix: merged; CI run green.
2. Per epic: compile gate, `--list` diff artifact, ONE `bash scripts/verify.sh`, every FAILED attributed.
3. Desktop: `cargo test --locked` in src-tauri, `npm run typecheck`, `npm test`.
4. Wall: `crate-wall` stage green; `codex` builds with `codex-ingest/src` renamed; residue gate 0/0.
5. Snapshot: `site-status-frozen-check` green; 49,450 of 49,450.
6. Counts: path helpers (only in `src/support/paths.rs`); deleted files gone.
7. Box: `df -h /` ≥ 850G free; `crontab -l` = `reclaim.sh` only.

