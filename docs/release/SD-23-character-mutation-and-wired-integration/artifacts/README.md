---
title: SD-23 — Cycle-Artifacts Index (`artifacts/`)
status: scaffolded (operator review pending)
scope: programs/codex/requirements/SD-23-character-mutation-and-wired-integration/artifacts
artifact_type: index
date: 2026-07-20
canonical_branch: tranche/5-1
kanban_board: codex-tranche-5
purpose: "Every criterion 1-30 has a per-cycle receipt artifact under `artifacts/`. The artifacts are load-bearing for Epic 7's Closure Epilogue evaluation — Epic 7 cannot conclude a criterion `complete` without the corresponding receipt's RED→GREEN transition having been persisted."
---

# SD-23 — Cycle-Artifacts Index (`artifacts/`)

This index documents the per-cycle receipt artifacts for every code-bearing cycle in SD-23. Mirrors the SD-22 convention (`artifacts/` + per-cycle receipt files + per-Epic subdirectories).

## 1. Where artifacts land

```
programs/codex/requirements/SD-23-character-mutation-and-wired-integration/artifacts/
├── README.md                                          (this file)
├── epic_3/                                            (Wired Integration Cleanup — per-criterion receipts)
│   ├── stubs_registry_audit_cycle_receipt.md          (criterion 7: Stubs Registry exists)
│   ├── four_check_audit_baseline_cycle_receipt.md     (criterion 8: audit baseline run)
│   ├── skill_cross_ref_cycle_receipt.md               (criterion 9: Stubs Registry referenced in skill)
│   ├── accidental_stub_remediation_cycle_receipt.md   (criterion 10: accidental stubs remediated)
│   └── epic_3_clean_diff_cycle_receipt.md             (criterion 11: post-Epic-3 audit clean)
├── epic_4/                                            (Campaign Manager Simplification — per-criterion receipts)
│   ├── create_campaign_no_drive_summary_cycle_receipt.md  (criterion 12: driveActionSummary removed)
│   ├── member_invited_field_deleted_cycle_receipt.md  (criterion 13: CampaignMember.invited removed)
│   ├── local_folder_rename_cycle_receipt.md           (criterion 14: syncCampaignDriveArtifacts → writeCampaignLocalFolderArtifacts)
│   └── epic_4_campaign_audit_cycle_receipt.md         (criterion 15: post-Epic-4 four-check audit clean)
├── epic_5/                                            (Character Mutation Surface — per-criterion receipts)
│   ├── mutation_table_dispatch_cycle_receipt.md       (criterion 16: typed operation table)
│   ├── level_up_command_cycle_receipt.md              (criterion 17: level_up_character Tauri command)
│   ├── add_equipment_and_spell_selection_cycle_receipt.md  (criterion 18: add_* Tauri commands)
│   ├── list_filter_corpus_commands_cycle_receipt.md   (criterion 19: list_spells/equipment Tauri commands)
│   ├── picker_modal_component_cycle_receipt.md        (criterion 20: ItemPickerModal.tsx)
│   └── sheet_refresh_after_mutation_cycle_receipt.md  (criterion 21: CharacterSheet detail refresh)
├── epic_6/                                            (Storage Tier Minimal Fix — per-criterion receipts)
│   ├── delete_character_command_cycle_receipt.md      (criterion 22: delete_character Tauri command)
│   ├── import_character_command_cycle_receipt.md      (criterion 23: import_character Tauri command)
│   └── load_screen_buttons_wired_cycle_receipt.md     (criterion 24: LoadCharacterScreen buttons wired)
├── epic_7/                                            (Closure Epilogue — per-criterion receipts)
│   ├── pre_promotion_verification_cycle_receipt.md    (criterion 25: 16 closure gates verified)
│   ├── promotion_pr_cycle_receipt.md                  (criterion 26: tranche/5-1 → develop PR)
│   ├── build_counter_advance_cycle_receipt.md         (criterion 27: 0.5.<build> → 0.6.0)
│   ├── decisions_risks_final_review_cycle_receipt.md  (criterion 28: decisions + risks final review)
│   ├── progress_log_complete_cycle_receipt.md         (criterion 29: progress.md cycle log complete)
│   └── bundle_closed_on_board_cycle_receipt.md        (criterion 30: 30 kanban cards complete)
└── closure-readiness-report.md                        (criterion 30's canonical closure report)
```

Epic 1 (Identifier Cleanup) and Epic 2 (Operator Pre-Launch) do not produce code-bearing cycles — their receipts live in `progress.md` (Epic 1) and `progress.md` (Epic 2 pre-launch checklist verification). The artifacts directory starts at Epic 3 because that is the bundle's first code-bearing epic.

## 2. Two sibling doctrine docs that Epic 7 reads

- **`../content-unit-inventory.md`** — per-content-unit four-tuple (rust_module_path / test_fixture_path / cycle_artifact_path / RuleSetId-or-CommandName). The wired-integration equivalent of SD-22's `corpus-source-inventory.md`.
- **`../acceptance-and-verification.md` §"Per-criterion artifact map"** — the per-criterion artifact-path table.
- **`../loop-instruction.md` Step 7-8** — the per-cycle procedure, with cross-references to this README.

## 3. What each per-cycle receipt looks like

Per `loop-instruction.md` §"Post-mortem schema," every per-cycle receipt has this shape:

```markdown
# <cycle-name> cycle receipt — <ISO-8601 UTC>

## Red-phase evidence (where applicable)
<command> <output>
<test failure paste showing the test fails for the intended reason>

## Green-phase evidence
<command> <output>
<test success paste showing all green>

## Files touched
- `apps/desktop/...` — added/modified
- `apps/desktop/src-tauri/...` — added/modified
- `src/...` — added/modified
- `tests/...` — added/modified
- `programs/codex/requirements/SD-23-character-mutation-and-wired-integration/artifacts/<epic>/<cycle>_cycle_receipt.md` — this file

## Audit result
- OK_NO_TOKENS
- OK_NO_NOOP_HANDLERS
- OK_NO_MOCK_LEAKS
- OK_NO_WOULD_STRINGS

## Cycle metadata
- cycle_id: <ISO-8601 timestamp>
- duration: <N> seconds
- bundle_criterion: <criterion-NN>
- four_check_audit: <pass|fail>
- identifier_discipline_audit: <pass|fail>

## kanban
- card: <hermes kanban card id>
- audit_comment: <comment id>
- commit_sha: <sha>
```

Receipts without RED-phase evidence (for non-TDD criteria like Epic 4 criterion 13 field deletion) get a "Reconciliation evidence" section instead — the before/after state of the data model showing the field is gone. Bucket-B / Bucket-C shortfalls are self-heal triggers; Epic 7's evaluator treats them as not-yet-complete.

## 4. Operator / cold-cloud-clone read path

A coding harness operating on a cold cloud clone (no access to `~/workspace/`) reads this tree:

1. **`../content-unit-inventory.md`** first (the four-tuple: rust module / test fixture / cycle artifact / RuleSetId-or-CommandName per content unit).
2. **`../acceptance-and-verification.md` §"Per-criterion closure gate → artifact map"** (the per-criterion artifact-path table).
3. **`../loop-instruction.md`** third (the operator-pinned cycle pipeline: RED → GREEN → cycle-artifact → commit).
4. **Per-cycle receipts** — the load-bearing surfaces of Epic 7's evaluation.
5. **`../risks-and-open-questions.md`** — latent risks (R1: storage-tier referential integrity; R2: stat-field promotion; R3: SD-22 closure gating).

## 5. Recorded

Authored 2026-07-20 per SD-23 scope-drafting session. Mirrors SD-22's artifacts/README.md structure (canonical pattern: cycle-artifacts index + per-Epic subdirectory + per-cycle receipt files + closure-readiness-report).
