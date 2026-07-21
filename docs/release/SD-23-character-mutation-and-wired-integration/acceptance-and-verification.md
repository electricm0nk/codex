# SD-23 Acceptance and Verification — Character Mutation and Wired Integration

Test-surface contract and per-bundle closure-gate list.

## Per-cycle audit (load-bearing)

Every cycle passes the four-check audit defined in `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` §"Per-cycle audit" before marking `complete`. Audit output is captured in the kanban card's comments stream per `kanban-claude-code-execution-receipt/SKILL.md`.

## Closure gates (16 gates)

A closure gate is a verification step that must pass before SD-23 can be marked closed (Epic 7 promotion PR merged to develop). The 16 gates mirror SD-22's gate list structure.

| Gate | Verification |
| --- | --- |
| 1 | SD-22 closure PR merged to develop (pre-launch gate, not post-cycle). |
| 2 | `tranche/5-1` branch pushed to origin and rebased on develop HEAD. |
| 3 | `codex-tranche-5` board has zero `ready` cards remaining after all cycles complete. |
| 4 | 33 acceptance criteria across 7 epics all marked `complete` with commit SHA + receipt comment. |
| 5 | Four-check audit output for the final cycle's diff is captured in the kanban card comments stream and is clean. |
| 6 | Stubs Registry has entries for every operator-granted stub; no stub in the diff is absent from the registry. |
| 7 | `progress.md` cycle log is complete with post-mortem schema entries for every cycle. |
| 8 | `Cargo.toml` workspace version reflects `0.5.<final_build>` at cycle end (last cycle's build counter value before promotion). |
| 9 | Identifier-discipline audit: zero `sd23_*`, `SD23_*`, `Sd23*`, `sd23-*` test-IDs, `t_<hex>` kanban tokens, `SD-23-Ex...` audit-IDs in source. |
| 10 | TDD compliance: every acceptance criterion has at least one test added or modified; test failure confirmed before implementation; implementation cycle ends green. |
| 11 | Tier-2 tests (Rust unit/integration) pass under `cargo test --workspace`. |
| 12 | Tier-3 tests (TypeScript/TSX) pass under `pnpm test` (or the repo's actual test runner — confirm at SD-23 launch). |
| 13 | `tranche/5-1` rebases cleanly on develop HEAD before promotion PR opens. |
| 14 | Promotion PR opens against develop with all 30 cycles' commits included; CI passes; merge is clean. |
| 15 | `decisions.md` final entry records the build counter at promotion (`0.5.<last_build> → 0.6.0` per Epic 7). |
| 16 | `risks-and-open-questions.md` final review: R1-R5 mitigation status recorded; OQ1-OQ2 resolution status recorded; D1-D4 deferred status confirmed. |

## Per-criterion artifact map

Every code-bearing criterion has a corresponding receipt artifact under `artifacts/`. The receipt is the load-bearing proof Epic 7's evaluator reads to confirm a criterion is `complete`.

| Criterion | Epic | Artifact path | Notes |
| --- | --- | --- | --- |
| 1 | 1 | `progress.md` (audit cycle entry) | Identifier-discipline audit. Read-only; no receipt file. |
| 2 | 1 | `progress.md` (rename cycle entries) | Optional rename cycles, driven by audit findings. |
| 3 | 1 | `progress.md` (cycle entry) | Skill-load verification. |
| 4 | 1 | `progress.md` (closure cycle entry) | Post-Epic-1 audit. |
| 5 | 2 | `progress.md` (pre-launch cycle entry) | Pre-launch checklist verification. |
| 6 | 2 | `progress.md` (build-counter cycle entry) | Build counter capture. |
| 7 | 3 | `artifacts/epic_3/stubs_registry_audit_cycle_receipt.md` | |
| 8 | 3 | `artifacts/epic_3/four_check_audit_baseline_cycle_receipt.md` | |
| 9 | 3 | `artifacts/epic_3/skill_cross_ref_cycle_receipt.md` | |
| 10 | 3 | `artifacts/epic_3/accidental_stub_remediation_cycle_receipt.md` | Variable per findings. |
| 11 | 3 | `artifacts/epic_3/epic_3_clean_diff_cycle_receipt.md` | |
| 12 | 4 | `artifacts/epic_4/create_campaign_no_drive_summary_cycle_receipt.md` | |
| 13 | 4 | `artifacts/epic_4/member_invited_field_deleted_cycle_receipt.md` | |
| 14 | 4 | `artifacts/epic_4/local_folder_rename_cycle_receipt.md` | |
| 15 | 4 | `artifacts/epic_4/epic_4_campaign_audit_cycle_receipt.md` | |
| 16 | 5 | `artifacts/epic_5/mutation_table_dispatch_cycle_receipt.md` | |
| 17 | 5 | `artifacts/epic_5/level_up_command_cycle_receipt.md` | |
| 18 | 5 | `artifacts/epic_5/add_equipment_and_spell_selection_cycle_receipt.md` | |
| 19 | 5 | `artifacts/epic_5/list_filter_corpus_commands_cycle_receipt.md` | |
| 20 | 5 | `artifacts/epic_5/picker_modal_component_cycle_receipt.md` | |
| 21 | 5 | `artifacts/epic_5/sheet_refresh_after_mutation_cycle_receipt.md` | |
| 22 | 6 | `artifacts/epic_6/delete_character_command_cycle_receipt.md` | |
| 23 | 6 | `artifacts/epic_6/import_character_command_cycle_receipt.md` | |
| 24 | 6 | `artifacts/epic_6/load_screen_buttons_wired_cycle_receipt.md` | |
| 25 | 7 | `artifacts/epic_7/pre_promotion_verification_cycle_receipt.md` | |
| 26 | 7 | `artifacts/epic_7/architecture_truth_up_cycle_receipt.md` + `receipts.md` (append-only YAML block with `row_or_kind: architecture:truth_up`) | |
| 27 | 7 | `artifacts/epic_7/graphify_update_cycle_receipt.md` + `receipts.md` (append-only YAML block with `row_or_kind: graphify:update`) | |
| 28 | 7 | `artifacts/epic_7/merge_conflict_resolution_cycle_receipt.md` + `receipts.md` (append-only YAML block with `row_or_kind: merge_conflict:<mode>`) | |
| 29 | 7 | `artifacts/epic_7/promotion_pr_cycle_receipt.md` | |
| 30 | 7 | `artifacts/epic_7/build_counter_advance_cycle_receipt.md` | |
| 31 | 7 | `artifacts/epic_7/decisions_risks_final_review_cycle_receipt.md` | |
| 32 | 7 | `artifacts/epic_7/progress_log_complete_cycle_receipt.md` | |
| 33 | 7 | `artifacts/epic_7/bundle_closed_on_board_cycle_receipt.md` | Plus `artifacts/closure-readiness-report.md` as the canonical closure report. |

## Test-surface contract

Each acceptance criterion in `epic-breakdown.md` carries:

- The verbatim acceptance text (the "what").
- The test contract (the "how we know it's true").
- The expected files touched.
- The audit-exclusion list (if any — for in-flight cleanup epics).
- The artifact path from the table above.

## Per-bundle verification commands

Run these at closure-gate evaluation time:

```bash
# Tier-2 tests (Rust)
cargo test --workspace

# Tier-3 tests (TypeScript / TSX)
pnpm test  # confirm package name and runner at SD-23 launch

# Identifier-discipline audit
grep -rnE 'sd23_|SD23_|Sd23|sd23-' apps/desktop/src apps/desktop/src-tauri apps/desktop/src-tauri/src 2>/dev/null | grep -v __tests__ | grep -v '.test.' || echo OK_NO_SD23_TAGS

# Wired-integration four-check audit
bash programs/codex/requirements/SD-23-character-mutation-and-wired-integration/audit-script.sh  # if extracted; otherwise run inline per loop-instruction
```

## Cross-references

- `loop-instruction.md` — operational cycle mechanics
- `epic-breakdown.md` — per-cycle test contract
- `decisions.md` — decision log
- `risks-and-open-questions.md` — latent risks and deferred questions
- `progress.md` — cycle log
- `../../governance/no-stub-mvp-doctrine.md` — parent doctrine
