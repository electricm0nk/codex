# SD-24 — Acceptance and Verification

> **Operating method:** see `./scope-draft.md`. Per-criterion artifact map below; closure gates at the bottom.

## 1. Per-criterion verification

The closure evaluation runs every criterion through four tests:

1. **`## Status matrix` says `complete`.** The progress doc reflects the criterion as completed.
2. **Cycle artifact exists** at `./artifacts/<epic>/<cycle-id>_cycle_receipt.md`. The artifact is the durable receipt.
3. **Dual-audit gate passes** for the criterion's commit SHA. Both identifier and wired-integration audits must be clean.
4. **`cargo test --locked --tests 2>&1 | tail -40` returns green** for any criterion touching production code.

| Criterion | Cycle artifact path | Verification command |
|---|---|---|
| 1.1 Source-code identifier audit | `./artifacts/epic_1/identifier-audit-cycle_receipt.md` | `git grep -nE '\b(sd(16\|19\|22\|23\|24)_)' apps/desktop/ apps/desktop/src-tauri/ src/ \| wc -l` returns 0 |
| 1.2 Per-cycle tests pass | per-rename cycle | `cargo test --locked --tests 2>&1 \| tail -20` |
| 2.1 `codex-tranche-5` reachable | per Epic 2 | `hermes kanban list-boards` shows `codex-tranche-5` |
| 2.2 `tranche/5-2` pushed | per Epic 2 | `git ls-remote origin tranche/5-2` resolves |
| 2.3 SD-23 closure PR merged | per Epic 2 | `git log origin/develop --oneline \| head -5` shows SD-23 closure as HEAD |
| 2.4 Working tree clean | per Epic 2 | `git status --porcelain` returns empty |
| 2.5 Doctrines loaded | per Epic 2 | `hermes skills --profile god-emporer --list` shows required skills |
| 3.1 Wired-Integration Audit | `./artifacts/epic_3/wired-integration-audit.md` | Run audit script; capture output |
| 3.2–3.4 remediation cycles | per-file cycle | Dual-audit; `cargo test --locked` |
| 4.1–4.3 per-class coverage audit | `./artifacts/epic_4/per-class-coverage-matrix.md` | Per-class `class_features_wired` count |
| 4.4 remediation plan | `./artifacts/epic_4/remediation-plan.md` | n/a (planning artifact) |
| 4.5 APG/ACG multiclass deferral | `./artifacts/epic_4/apg-acg-multiclass-deferred.md` | n/a (deferral artifact) |
| 5.1 F+W multiclass dispatch | per-cycle artifact | `cargo test --locked --test sd24_multiclass_deterministic` |
| 5.2 30 character-advancement cycles | `./artifacts/epic_5/multiclass-fixture.md` | `cargo test --locked --test sd24_multiclass_deterministic` runs 30+ cases |
| 5.3 integration test | `./artifacts/epic_5/integration-test-cycle_receipt.md` | `cargo test --locked --test sd24_multiclass_integration` |
| 5.4 multiclass dispatch four-check audit | per-cycle artifact | Dual-audit gate |
| 5.5 APG/ACG multiclass deferred | `./artifacts/epic_5/apg-acg-multiclass-deferred.md` | n/a (deferral artifact) |
| 6.1 Equipment coverage audit | `./artifacts/epic_6/equipment-coverage-matrix.md` | Audit output |
| 6.2–6.5 content completion | per-cycle artifact in `./artifacts/epic_6/content-completion-log.md` | Audit re-run; coverage = 100% |
| 7.1 `appendToCharacter` | per-cycle artifact | inline test via `#[cfg(test)] mod tests` in `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs` (no standalone `tests/sd24_characterhub_append.rs` file) — (corrected 2026-07-22 per SD-25 criterion 7.P: all Tauri command tests are inline, not standalone test files) |
| 7.2 `recomputeCharacter` | per-cycle artifact | `cargo test --locked --test sd24_characterhub_recompute` |
| 7.3 `reSaveCharacter` | per-cycle artifact | `cargo test --locked --test sd24_characterhub_resave` |
| 7.4 Add Weapon/Armor/Spell onClick | per-cycle artifact | `cargo test --locked --test sd24_picker_wired` |
| 7.5 `character_hub.rs::compose_character_input` loadout | per-cycle artifact | Dual-audit; `cargo test --locked --test sd24_characterhub_loadout` |
| 8.1 Final criterion scan | `./artifacts/epic_8/final-criterion-scan-cycle_receipt.md` | `## Status matrix` row counts |
| 8.2 Architecture closure pipeline | `./receipts.md` (architecture-truth-up + graphify + PR receipts) | Sub-step scripts run; `receipts.md` accumulates |
| 8.3 Release notes | `./release-notes.md` | Section checklist (Summary / User-Visible / Defects / Operational / Verification / Known / Update) |
| 8.4 Build version increment | per-file diff | `Cargo.toml` workspace version increments; SD-24's first value lands as `0.5.<build>` |

## 2. Closure gates (per-bundle)

| # | Closure gate | Verification |
|---|---|---|
| CG-01 | All 35 acceptance criteria `complete` or have a real `## Open blockers` entry | Final scan criterion 8.1 |
| CG-02 | Wired-Integration Audit Cycle (3.1) covers the entire codebase | Artifact at `artifacts/epic_3/wired-integration-audit.md` |
| CG-03 | Per-class coverage matrix (4.1–4.3) covers CRB + APG + ACG | Artifact at `artifacts/epic_4/per-class-coverage-matrix.md` |
| CG-04 | Multiclass F+W to 10 passes deterministic + integration tests | `cargo test --locked --test sd24_multiclass_*` |
| CG-05 | Equipment corpus 100% field coverage | `artifacts/epic_6/equipment-coverage-matrix.md` shows 0 missing fields |
| CG-06 | Tauri command surface for iterative mutation works | `cargo test --locked --test sd24_characterhub_*` |
| CG-07 | Add Weapon/Armor/Spell onClick wires to real corpus | `cargo test --locked --test sd24_picker_wired` |
| CG-08 | Dual-audit gate (identifier + wired-integration) is clean on the closure PR diff | Run both audits on the closure PR's `git diff` |
| CG-09 | Architecture-truth-up sub-step ran with a YAML receipt in `receipts.md` | `./receipts.md` has the entry |
| CG-10 | Graphify-update sub-step ran (success or graceful failure) | `./receipts.md` has the entry |
| CG-11 | `tranche/5-2 → develop` PR is opened and merged | `gh pr view` shows merged |
| CG-12 | Release notes generated per template section requirements | `./release-notes.md` has all required sections |
| CG-13 | Build counter incremented to `0.5.<next_build>` | `Cargo.toml`, `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json` show the new value |
| CG-14 | Workspace-side package deleted at publish (move-not-copy) | the workspace-side planning surface for SD-24 no longer exists after the publish commit — only the repo-side `docs/release/SD-24-.../` directory remains |
| CG-15 | Workspace-side loose SD-19/20/21/22 files deleted; workspace-side mirror trimmed to SD-11..SD-15 + `templates/` + `README.md` + `GE-*` only | the workspace-side listing shows no SD-16-forward packages; only the older governance/program mirror remains |
| CG-16 | SD-24's package does not contain any operator-workspace-rooted or workspace-program-mirror absolute paths in its markdown files | `grep -rE '<workspace-root>/programs' docs/release/SD-24-...` returns 0 (excluding this row + the matching rows in scope-draft.md and decisions.md which intentionally describe the publish-mode doctrine) |
| CG-17 | Hard-stop shape honored at 5am (grace-tail or strict per FLAG-A) | `## Open blockers` reflects "stopped at cycle N" note if applicable |

## 3. Per-criterion artifact map

The cycle picker writes each cycle's artifact to `./artifacts/<epic>/<cycle-id>_cycle_receipt.md` (per the SD-23 / SD-22 pattern). Closure reads the artifact to confirm RED → GREEN → re-audit. The artifact is the durable receipt.

For criteria with non-trivial artifacts (audit outputs, coverage matrices, deferral memos), the cycle artifacts are committed at the paths named in §1. The artifact index lives at `./artifacts/README.md`.

## 4. Cross-reference

- `./scope-draft.md §6` — Hard-stop conditions
- `./decisions.md §3` — Build counter inheritance
- `./decisions.md §5` — Equipment corpus strict 100% field coverage
- `./decisions.md §7` — Kanban is receipt-only
- `./epic-breakdown.md` — Per-cycle stories
- `./risks-and-open-questions.md §3` — Override flags
- `./loop-instruction.md §4` — Self-heal posture
