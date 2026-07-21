# SD-23 Content-Unit Inventory

Per-content-unit N-tuple: (rust_module_path / test_fixture_path / cycle_artifact_path / CommandName-or-ComponentName). Models SD-22's `corpus-source-inventory.md` four-tuple (rust module / test fixture / cycle artifact / RuleSetId), adapted for SD-23's mixed Rust + TypeScript + React workload.

Each unit is one cycle's worth of work. A cycle that touches multiple units either consolidates them into one larger unit or splits into multiple cycles — the inventory's unit count drives `epic-breakdown.md` criterion count.

## Unit types

For SD-23 the canonical unit types are:

- **Tauri command** — one Tauri command, one unit. Rust function in `apps/desktop/src-tauri/src/`, frontend boundary function in `apps/desktop/src/boundary/`, integration test in `tests/`.
- **UI component** — one React component, one unit. TypeScript/TSX file in `apps/desktop/src/`, tests in `__tests__/` or `*.test.tsx`.
- **Data model field** — one field add/remove/rename, one unit. Touches the data model and any consumers.
- **Audit/reconciliation** — one audit cycle, one unit. Per-cycle receipts only.

## Per-content-unit inventory

### Epic 1 — Identifier Cleanup

No content units — read-only audit per criterion 1, optional rename cycle per criterion 2 (driven by the audit findings).

### Epic 2 — Operator Pre-Launch

No content units — pre-launch checklist verification per criterion 5, single progress.md update per criterion 6.

### Epic 3 — Wired Integration Cleanup

| # | Unit | Type | Rust path | Test fixture | Cycle artifact |
| --- | --- | --- | --- | --- | --- |
| E3-1 | Stubs Registry file | Audit | — | — | `artifacts/epic_3/stubs_registry_audit_cycle_receipt.md` |
| E3-2 | Four-check baseline | Audit | — | — | `artifacts/epic_3/four_check_audit_baseline_cycle_receipt.md` |
| E3-3 | Skill cross-reference | Audit | — | — | `artifacts/epic_3/skill_cross_ref_cycle_receipt.md` |
| E3-4 | Accidental-stub remediation | Variable | Variable | Variable | `artifacts/epic_3/accidental_stub_remediation_cycle_receipt.md` |
| E3-5 | Post-Epic-3 audit clean | Audit | — | — | `artifacts/epic_3/epic_3_clean_diff_cycle_receipt.md` |

### Epic 4 — Campaign Manager Simplification

| # | Unit | Type | Rust path | Test fixture | Cycle artifact |
| --- | --- | --- | --- | --- | --- |
| E4-1 | `createCampaign` return-shape change | Data model field | — | `apps/desktop/src/campaign/campaignModel.test.ts` | `artifacts/epic_4/create_campaign_no_drive_summary_cycle_receipt.md` |
| E4-2 | `CampaignMember.invited` field deletion | Data model field | — | `apps/desktop/src/campaign/campaignModel.test.ts` | `artifacts/epic_4/member_invited_field_deleted_cycle_receipt.md` |
| E4-3 | `syncCampaignDriveArtifacts` rename | Rename | — | — | `artifacts/epic_4/local_folder_rename_cycle_receipt.md` |
| E4-4 | Post-Epic-4 audit clean | Audit | — | — | `artifacts/epic_4/epic_4_campaign_audit_cycle_receipt.md` |

### Epic 5 — Character Mutation Surface

| # | Unit | Type | Rust path | Test fixture | Cycle artifact |
| --- | --- | --- | --- | --- | --- |
| E5-1 | `mutate_saved_character` operation table | Tauri command | `apps/desktop/src-tauri/src/character_hub.rs` | `tests/sd23_mutation_table.rs` | `artifacts/epic_5/mutation_table_dispatch_cycle_receipt.md` |
| E5-2 | `level_up_character` Tauri command | Tauri command | `apps/desktop/src-tauri/src/character_hub.rs` | `tests/sd23_level_up_character.rs` | `artifacts/epic_5/level_up_command_cycle_receipt.md` |
| E5-3 | `add_equipment_selection` + `add_spell_selection` | Tauri command | `apps/desktop/src-tauri/src/character_hub.rs` | `tests/sd23_add_selection.rs` | `artifacts/epic_5/add_equipment_and_spell_selection_cycle_receipt.md` |
| E5-4 | `list_spells(filter)` + `list_equipment(filter)` | Tauri command | `apps/desktop/src-tauri/src/character_hub.rs` | `tests/sd23_list_corpus.rs` | `artifacts/epic_5/list_filter_corpus_commands_cycle_receipt.md` |
| E5-5 | `ItemPickerModal.tsx` | UI component | — | `apps/desktop/src/characterHub/__tests__/ItemPickerModal.test.tsx` | `artifacts/epic_5/picker_modal_component_cycle_receipt.md` |
| E5-6 | Character sheet refresh after mutation | UI component | — | `apps/desktop/src/characterHub/__tests__/CharacterSheet.refresh.test.tsx` | `artifacts/epic_5/sheet_refresh_after_mutation_cycle_receipt.md` |

### Epic 6 — Storage Tier Minimal Fix

| # | Unit | Type | Rust path | Test fixture | Cycle artifact |
| --- | --- | --- | --- | --- | --- |
| E6-1 | `delete_character` Tauri command | Tauri command | `apps/desktop/src-tauri/src/character_hub.rs` | `tests/sd23_delete_character.rs` | `artifacts/epic_6/delete_character_command_cycle_receipt.md` |
| E6-2 | `import_character` Tauri command | Tauri command | `apps/desktop/src-tauri/src/character_hub.rs` | `tests/sd23_import_character.rs` | `artifacts/epic_6/import_character_command_cycle_receipt.md` |
| E6-3 | Load Character screen buttons wired | UI component | — | `apps/desktop/src/characterHub/__tests__/LoadCharacterScreen.buttons.test.tsx` | `artifacts/epic_6/load_screen_buttons_wired_cycle_receipt.md` |

### Epic 7 — Closure Epilogue

| # | Unit | Type | Rust path | Test fixture | Cycle artifact |
| --- | --- | --- | --- | --- | --- |
| E7-1 | Pre-promotion verification | Audit | — | — | `artifacts/epic_7/pre_promotion_verification_cycle_receipt.md` |
| E7-2 | Promotion PR | Audit | — | — | `artifacts/epic_7/promotion_pr_cycle_receipt.md` |
| E7-3 | Build counter advance | Audit | — | — | `artifacts/epic_7/build_counter_advance_cycle_receipt.md` |
| E7-4 | Decisions/risks final review | Audit | — | — | `artifacts/epic_7/decisions_risks_final_review_cycle_receipt.md` |
| E7-5 | Progress log complete | Audit | — | — | `artifacts/epic_7/progress_log_complete_cycle_receipt.md` |
| E7-6 | Bundle closed on board | Audit | — | — | `artifacts/epic_7/bundle_closed_on_board_cycle_receipt.md` |

## Recorded

Authored 2026-07-20 per SD-23 scope-drafting session. Mirrors SD-22's `corpus-source-inventory.md` four-tuple structure; expanded to N-tuple because SD-23 has UI components (no rust module path) and Tauri commands (no React component path). Epic 1-2 have no content units — they're audit/pre-launch, not code-bearing.
