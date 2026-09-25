---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Epic Breakdown

Acceptance criteria per epic, grouped with acceptance commands. Read this before dispatch.

**Epic order (corrected 2026-09-21, Epic F scoped in):** B → E → A → C → D(docs) → F →
D(closure). Epic D's own D1 (architecture-docs full-set rewrite) landed early, ahead of D2–D6
(retrospective, release notes, graphify, PR, worktree sweep); Epic F — class completion — is
scoped between D1 and the rest of D per the operator's 2026-09-21 ruling to close the class
gaps inside this bundle before PR #393 merges (`decisions.md §11`). Full detail:
`epic-f-class-completion.md`.

---

## Epic B — Freeze PF1e status page, retire producers (one batch, one pass)

| Criterion | Acceptance command / evidence |
|---|---|
| B1. Frozen-status gate exists and fires | `python3 -m pytest scripts/tests/test_check_frozen_status.py -q` (all green); `scripts/site/check_frozen_status.py --check` exits zero on `site/status-data.json` |
| B2. Public status rewritten per D5 | `python3 scripts/site/build_public_status.py --check` exits zero; `site/status-data.json` shows 49,450 / 49,450; old-rule tests in `test_build_public_status.py` rewritten and green |
| B3. Whole files deleted (8 binaries/modules, 9 tests) | `ls -la src/bin/v06_work_inventory.rs` exits nonzero (file gone); same for `support_state_matrix.rs`, `reach_gate.rs`, desktop bridge, 9 test files |
| B4. Desktop UI cleanup | `grep -r 'SupportDebtPanel\|BreadthClaimAuditPanel\|supportStateTone' apps/desktop/src/` exits nonzero (component code gone); git diff shows the UI components deleted, wiring removed |
| B5. Verify.sh stages updated | `bash scripts/verify.sh --list \| grep 'site-dashboard'` (0 matches); `bash scripts/verify.sh --list \| grep 'site-status-frozen'` (1+ matches in both stage sets) |
| B6. Baseline table updated | `scripts/verify-baselines.env` shows `BASELINE_ROOT_TEST_BINARIES=412` (was 419 before deletions); `BASELINE_ROOT_FULL_TESTS` re-measured and recorded |
| B7. Work inventory frozen | `ls -la docs/work-inventory.FROZEN.md` (file exists); content records SHA, date, rule; `docs/work-inventory.json` unchanged |
| B8. Deferral/extraction decision recorded | `scripts/retro.py deferral` entry if `pf1e_dashboard_producer.py` extraction incomplete; commit message references the receipt |
| B9. One pass green | `cargo test --locked --no-run -j 6` at root and desktop succeeds; `bash scripts/verify.sh` all stages green; all test suites pass |

**Deleted files / line counts (for receipt):**
- `src/bin/v06_work_inventory.rs` — 33,091 lines
- `src/rules_core/support_state_matrix.rs` — 7,475 lines
- `apps/desktop/src-tauri/src/support_state_matrix_bridge.rs` — ~1,500 lines (approx)
- `apps/desktop/src-tauri/src/reach_gate.rs` — 8,846 lines
- 9 test files — ~2,500 lines (total)
- 3+ support scripts and docs — ~3,000 lines (total)

Total: 55,827 lines.

---

## Epic A — PCGen wall: crate `crates/codex-ingest` (one batch, one pass)

| Criterion | Acceptance command / evidence |
|---|---|
| A1. Gate tests green | `python3 -m pytest scripts/tests/test_pcgen_residue_gate.py -q` (all green); patterns detect `lst_file` identifier and `codex_ingest` in test vs. live root |
| A2. Residue gate patterns added | `grep 'lst_file' scripts/pcgen_residue_gate.py` (pattern present); `python3 scripts/pcgen_residue_gate.py --check --closure` exits zero; output shows `lst_file files=0 hits=0` and `codex_ingest files=0 hits=0` |
| A3. Crate scaffolded | `ls -la crates/codex-ingest/{Cargo.toml,src/lib.rs}` (both exist); manifest declares workspace membership; lib exports `pub mod pcgen_import; pub mod oracle_validation;` etc. |
| A4. Machinery moved | `ls -la src/pcgen_import` exits nonzero (moved); same for `src/oracle_validation`; `wc -l crates/codex-ingest/src/pcgen_import/` shows the moved code; 47 tool bins present in crate (git diff `--name-status` shows M mode for moves) |
| A5. Path rewrites complete | `git grep -c 'CARGO_MANIFEST_DIR' -- crates` exits 0 (no remaining); `git grep -c 'crate::pcgen_import' -- crates/codex-ingest` shows uses redirected to `codex_ingest::` within the crate |
| A6. Test-module eviction done | `find src/rules_core -name '*.rs' -exec grep -l 'cfg(test)' {} +` shows 0 lines with `#[cfg(test)] mod` + `crate::pcgen_import` (these are gone; they moved to fixtures or were routed to `crates/codex-ingest/tests/`); `git grep -c 'crate::pcgen_import' -- src/rules_core` exits 0 |
| A7. `SourceRef.lst_file` renamed | `git grep -c 'lst_file' -- src` exits 0 (no raw field name); `git grep -c 'source_path' -- src` shows uses; desktop fixtures diff shows only the key rename (no content drift) |
| A8. Desktop import rewritten | `git grep 'codex_ingest::pcgen_import::pcgen_desc::leaked_pcgen_syntax' -- apps/desktop/src-tauri/tests` shows 9+ lines (9 test imports); `git log -1 --name-status` shows edits to those files |
| A9. Verify.sh stages added | `bash scripts/verify.sh --list \| grep 'ingest\|crate-wall'` shows both stages present; `bash scripts/verify.sh --list \| grep 'clippy' \| tail -1` shows third clippy (crate-ingest); CI workflow shows `cargo test --locked --workspace` (not just `--lib`) |
| A10. Baselines updated | `scripts/verify-baselines.env` shows `BASELINE_INGEST_FULL_TESTS`, `BASELINE_INGEST_TEST_BINARIES`, `BASELINE_CLIPPY_WARNINGS_INGEST=0`; root binaries drop by 47; root + ingest totals ≥ Epic B green count |
| A11. One pass green | `cargo test --locked --workspace --no-run -j 6` at root succeeds; `cargo test --locked --no-run -j 6` in desktop succeeds; `bash scripts/verify.sh` all stages green, `crate-wall` stage green; residue gate 0/0; all test suites pass |

---

## Epic C — Bloat cuts (two batches, two passes)

### Pass C1 — Source-side refactor

| Criterion | Acceptance command / evidence |
|---|---|
| C1.1. `pilot_compute/mod.rs` split | `wc -l src/rules_core/pilot_compute/mod.rs` shows ≤ 4,000 lines (was 88,828); `ls -la src/rules_core/pilot_compute/class_*.rs` shows ~12 class files exist; largest file `wc -l src/rules_core/pilot_compute/prestige_class_features.rs` shows ≤ 6,600 |
| C1.2. Submodules wired | `git grep -c 'mod class_' -- src/rules_core/pilot_compute/mod.rs` shows ~20+ mod declarations; `git grep -c 'pilot_compute::' -- src` shows call sites unchanged (paths still work) |
| C1.3. Path helpers consolidated | `ls -la src/support/paths.rs` (exists); `wc -l src/support/paths.rs` shows 100-200 lines (includes the 5 fns + tests); `git grep -c 'fn repo_root' -- src` shows 1 hit (only in paths.rs) |
| C1.4. Clippy lock added | `grep -A2 'fn clippy_one_crate' scripts/verify.sh \| grep -- '-- -D warnings'` (present); `bash scripts/verify.sh --list \| grep clippy \| wc -l` shows same count or +1 |
| C1.5. One pass green | `cargo test --locked -- --list \| head -50` (output same as before, modules normalized); `bash scripts/verify.sh` all stages green; all suites pass |

### Pass C2 — Test-side rewrite

| Criterion | Acceptance command / evidence |
|---|---|
| C2.1. Table-driven tests rewritten | **Corrected 2026-09-20** (`docs/retro/events/epic-c2-test-rewrite.jsonl` id `1789886083389-epic-c2-test-rewrite-6b6500`): `ls tests/sd18_widening/rows.rs` / `ls tests/sd13_progression/rows.rs` must exist; per-row files remain; `cargo test --locked --test sd18_widening -- --list \| grep -c ': test$'` shows 891 (unchanged) and `cargo test --locked --test sd13_progression -- --list \| grep -c ': test$'` shows 1,136 (unchanged) — **not started**, both `rows.rs` files are absent |
| C2.2. Test count proof | `cargo test --locked --test sd18_widening -- --list` and `cargo test --locked --test sd13_progression -- --list` each saved to a diff artifact before/after (per-binary, since `--list` lines are `module::test: test` and never carry the binary name — a combined-output `grep sd18_widening` always returns 0); byte-identical match required — **not started** |
| C2.3. Path helpers re-exported | `git grep -c 'fn repo_root' -- tests/` shows exactly 1 hit (the canonical `tests/support/paths.rs` definition itself, tracked — `git ls-files tests/support/paths.rs` non-empty); 27 files carry `#[path = "support/paths.rs"] mod paths;` (`git grep -l '#\[path = "support/paths.rs"\]' -- tests/ \| wc -l`); local copy count elsewhere in `tests/` is 0 — **done** 2026-09-20 |
| C2.4. Branch promotion test moved | `ls -la tools/ci/test_branch_promotion_guard.sh` (exists); `ls -la tests/sd16-e5-f1/` (directory exists but file gone); 4 live references updated (2 GitHub Actions workflow comments — `allow-only-develop-into-test.yml`, `allow-only-test-into-main.yml` — plus `docs/architecture/release-pipeline.md` and `docs/architecture/testing.md`; `scripts/publish-site-to-main.sh` never referenced the old path — corrected per `docs/retro/events/epic-c2-test-rewrite.jsonl` id `1789886770689-epic-c2-test-rewrite-23a26d`) — **done** |
| C2.5. Oracle tests kept | `git grep -c '^#\[ignore\]\|    #\[ignore\]' -- tests` shows 21 hits (20 files); the same over `crates/codex-ingest/tests` shows 31 (6 files; `pcgen_runner_smoke.rs` mentions `` `#[ignore]` `` only in a doc comment); both sets run once against the real PCGen corpus (`PCGEN_CORPUS_ROOT=$HOME/workspace/repos/pcgen/data`), 52/52 passed, documented in `receipts.md`'s Epic C2 evidence section — **done** 2026-09-20 |
| C2.6. One pass green | `cargo test --locked --test sd18_widening -- --list` / `--test sd13_progression -- --list` diffs IDENTICAL (only meaningful once C2.1/C2.2 land); `bash scripts/verify.sh` all stages green (`verify2-C2D-1.log`, 50/50 PASS, exit 0) with `scripts/verify-baselines.env`'s floors matching the measured run (corrected 2026-09-20 — see that file's SD-36 Epic C2D block) |

---

## Epic D — Closure

| Criterion | Acceptance command / evidence |
|---|---|
| D1. Architecture docs refreshed | `git diff docs/architecture/` shows updated Last-verified headers and refreshed boundary/rules-engine/desktop-app/status/testing sections for topics touched by B/A/C |
| D2. Retrospective written | `ls -la docs/retro/sd36-retrospective.md` (exists); content includes retro events for each retirement (B7, A6 deferral decision, etc.); referenced in `docs/release/SD-36-consolidation/references/README.md` |
| D3. Release notes updated | `ls -la docs/release/SD-36-consolidation/release-notes.md`; baseline table re-derived (each row with its command from plan); before/after src/tests/binaries counts match verified gate output |
| D4. Graphify run | `ls -la docs/release/SD-36-consolidation/receipts.md` (exists); contains `graphify:update` receipt block with exit code |
| D5. PR open and merged | `gh pr list --state merged --base develop \| grep 'tranche/16'` (1+ rows); CI run for merged PR is green (`gh run list --limit 1 --jq '.[] \| select(.name == "Publish tester release") \| .conclusion'` shows "success") |
| D6. Worktree swept | Closure receipt shows `git worktree list` count and `git branch` in main tree; worktrees removed, branches cleaned (tranche/15 deleted) |

---

## Epic F — Class completion (scoped between D1 and D2–D6, operator ruling 2026-09-21)

Measured baseline: **42 of 135** class ids reach `HeadlessReceiptStatus::Computed` at every
swept level, corpus-wide, across every registry the engine's dispatch chain reads
(`docs/release/SD-36-consolidation/artifacts/epic-f/docs-truth/class-census.md`, generated
2026-09-20 against `424e93e93c`). Target: **135 of 135**. Full plan, every command, every
file:line, RED-first tests, flip lists, risk, size and order:
`docs/release/SD-36-consolidation/epic-f-class-completion.md`. Batches F0–F5 are **open** —
see `kanban.md` / `progress.md` — nothing in this section is a completion claim.

### F0 — Permanent census instrument (RED first)

| Criterion | Acceptance command / evidence |
|---|---|
| F0.1 RED truth (review finding 12d) | `cargo run --locked --bin class_census -- --json /tmp/census.json` prints `ids=135 computed=42 blocked=93`; AND `cargo test --locked --lib class_census::tests::census_id_set_matches_the_published_partition` green (merged id set == `status.md`'s partition 31+3+20+7+74, pinned before the instrument may move) |
| F0.2 stage (review finding 10) | `bash scripts/verify.sh --list \| grep -E '^class-census +yes +yes'` prints the row (membership in both stage sets by the columns — `--list` is one row per stage headed `stage full quick`, so `-> 2` cannot pass for a correctly registered stage); `bash scripts/verify.sh --only class-census` green |
| F0.3 generated table | `python3 scripts/gen_class_status_table.py --check` exits 0; `python3 -m pytest scripts/tests/test_gen_class_status_table.py -q` green |
| F0.4 list parity | `cargo test --locked --lib class_census` green (prestige list == fixture's 74 of 74; no id in two families) |

### F1 — Link repair corpus-wide (Option A) + class weapon proficiency

| Criterion | Acceptance command / evidence |
|---|---|
| F1.1 reproducible before | `cargo run --locked --quiet -j 2 -p codex-ingest --bin sheet_rule_convert -- --check` exits 0 at the pre-change commit |
| F1.2 oracle pin for the reader (extended, review finding 15) | `cargo test --locked -p codex-ingest --test class_weapon_proficiency_via_converter` green (reader output == each of the 42 static rows AND every other reader row re-derived from a named oracle row — tier, `Weapon Group <x>`, or expanded `WeaponSet`); `an_unrecognized_proficiency_tag_makes_the_class_unknown` green (e.g. `Auto`, `KoboldTailAttachment` -> `Unknown`, never fabricated) |
| F1.3 links closed (per-edge pin, review finding 12a) | `data/sheet_rules/_defects/unresolved-references.json` length: 11,925 - 4,456 = **7,469** (or the difference explained per row); mechanism A = 0 AND D/E/F unchanged at 3,033/3,565/808; per-edge: each added edge's target rule `provenance.closure_rows` contains the named oracle line (a count match alone does not rule out an edge landing on a colliding slug, e.g. `wizard`) |
| F1.4 no silent drop | `cargo test --locked -p codex-ingest automatic_grants_are_never_dropped_silently` green |
| F1.5 coverage | census: classes blocked on `combat.baseline_weapon_proficiency_unknown` -> 0 of 135 |
| F1.6 structural diff | `unexpected field deltas: 0`, `records 49450 -> 49450`, added edges per kind == mechanism A's per-kind table |
| F1.7 gates | `python3 scripts/pcgen_residue_gate.py --check --closure` 0 of 0; `python3 scripts/site/check_frozen_status.py --check`; `git status --porcelain -- data/corpus site \| wc -l` -> 0 |
| F1.8 gate carried, not dropped | `cargo test --locked -p codex-ingest a_pre_gated_weapon_proficiency_is_not_granted_unconditionally` green; `grep -n 'let _ = when' crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs` -> 0 hits |
| F1.9 package handle | `cargo test --locked --lib rules_core::sheet_rule_package` green (named `Err`, never panic, never silent empty); load-time and memory figures recorded in the F1 receipt |

### F1b — Print-path reconciliation

| Criterion | Acceptance command / evidence |
|---|---|
| F1b.0 sibling-amplified count | offline script run against the real A-target list; table of `edges / (1+siblings) total / split by print:true` committed; 4,456 never reused as a print-surface size |
| F1b.1 instrument | `cargo run --locked --bin class_census -- --sheet-dump /tmp/dump --only wizard` writes 3 files |
| F1b.2 n=1 / n=5 receipts | both artifacts exist; each reports `changed-value=0 removed-unexplained=0 duplicate=0` |
| F1b.3 join | `cargo test --locked --lib rule_for_explanation` green, including `a_facet_id_never_joins_to_the_class_principal_rule` and `the_three_known_good_pairs_still_join` |
| F1b.4 agreement (test-only) | `cargo test --locked --test sd36_sheet_value_agreement` green (0 disagreements out of every joined pair, all census classes, levels 1/10/max) |
| F1b.5 fixtures (script, review finding 12c) | `python3 scripts/tests/check_fixture_rebaseline_receipts.py` exits 0: diffs `git show --name-only HEAD`'s fixture paths against the `fixture-rebaseline-*.md` receipt filenames, non-zero on any mismatch |

### F2 — Gate arm, CLASS_FAMILY_BOOKS, prestige alone

| Criterion | Acceptance command / evidence |
|---|---|
| F2.1 | `cargo test --locked --lib generic_class_chassis` green with the re-measured pin (78 -> measured; ceiling 96) |
| F2.2 | `cargo test --locked --lib prestige_alone` green; census `alone_blocked=74` of 74 |
| F2.3 (falsifiable) | census `computed == 42` of 135 BEFORE F2's gate-arm change AND `computed == 42` of 135 AFTER it |

### F3 — Multiclass for every class with a chassis

| Criterion | Acceptance command / evidence |
|---|---|
| F3.0 unknown, not zero | every census class with `ClassChassis.hit_die`/`.skill_ranks_per_level == None` reports HP/skill-points `Unknown` (named list); `cargo test --locked --lib a_class_missing_hit_die_reports_hp_unknown` and `..._missing_skill_ranks_reports_skill_points_unknown` green |
| F3.1 (re-measured at F3d, `decisions.md` §14.2) | (i) census `mix_computed == BASELINE_CENSUS_MIX_COMPUTED` (185 of 185 under the GE-06 canonical fixture) -- the Computed proof for mixes; (ii) the 187 multiclass negative controls (`artifacts/epic-f/stage-f2-f3/f3d-sites.tsv`) assert STATUS PARITY, not Computed: mix receipt status == class-alone status AND mix claim-blocking set (`multiclass.<class>.` re-scope stripped) == class-alone set; assertion (a) verbatim; vacuity guard `class_levels.len() >= 2`. Reason: measured at F3d, their class-specific fixtures are Blocked class-ALONE on `combat.baseline_unsupported` + `skill.selected_modifier.unsupported` (187 of 187, not the GE-06 posture) and every mix's set equals the alone set modulo re-scope (187 of 187), so a Computed assertion ran 187 of 187 red and would fabricate a success. `cargo test --locked -j 8 --no-fail-fast --test sd18_widening --test sd13_progression` + the 43 top-level bins: 187 of 187 green (`f3d-verify.log`) |
| F3.2 | `--list` diffs for `sd18_widening` (891 of 891) and `sd13_progression` (1,136 of 1,136): IDENTICAL |
| F3.3 (re-measured at F3d, `decisions.md` §14.2) | sabotage = disable the fold's carry-over of each class's own claim-blocking lines into the mix (`multiclass_fold::explain_multiclass_fold`): **14 of 187** negative controls red (the Monk mixes; the other 173 hold on lines the mix raises itself -- pillars 129, sorcerer 25, cleric 19 -- named by mechanism), 0 of 187 red restored; `artifacts/epic-f/stage-f2-f3/f3d-sabotage-log.md` |
| F3.4 | `cargo test --locked --test sd21_multiclass_fighter_wizard_chassis_computes --test sd24_multiclass_integration` green |

### F4 — Desktop

| Criterion | Acceptance command / evidence |
|---|---|
| F4.1 | `cd apps/desktop/src-tauri && cargo test --locked list_class_creation_roster` green; roster length == census computed base count |
| F4.2 | `cd apps/desktop && npm test -- classRoster characterHubModel characterProgression skillsModel` green; `npm run typecheck` green |
| F4.3 (strengthened, review finding 12b) | `git grep -c 'fn canonical_seeds_for' -- src apps` -> 1 (the single definition) AND `git grep -n 'use .*canonical_seeds_for' -- src/bin apps` -> 2 (both call sites import it) |
| F4.4 | ui-smoke rows green: `create-character-samurai`, `-magus`, `-warrior`, `-kineticist`, `-inquisitor-generic` and `level-up-fighter6-into-arcane-archer` |
| F4.5 (new, review finding 13) | `cargo test --locked --lib no_computed_class_is_unoffered_without_a_named_reason` green: `in_desktop_roster == false` always carries `hit_die_absent \| not_computed \| prestige \| ex_state`; the 7 `hit_die_absent` classes (0.4) asserted present by id |

### F5 — Closure deltas

| Criterion | Acceptance command / evidence |
|---|---|
| F5.1 | `docs/architecture/status.md` class table regenerated between markers, `scripts/gen_class_status_table.py --check` exits 0; five head-count sites updated together; grep at `status.md:70` re-run to zero stale hits |
| F5.2 | `decisions.md` §11–§14, `technical-design.md` Epic F section, `workflow-instruction.md` §0/§3 rows, `kanban.md`/`progress.md`/`receipts.md`/`release-notes.md`/`forward-scope-register.md` all updated |
| F5.3 | `scripts/verify-baselines.env` re-derived; PR #393 body updated; graphify run LAST against the final tree |

---

