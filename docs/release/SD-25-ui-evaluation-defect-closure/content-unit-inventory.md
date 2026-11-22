# SD-25 — Content Unit Inventory

> **Per-content-unit N-tuple.** For SD-25, content units are the file-touch sets per criterion, mapped to their canonical files on disk.

## 1. Hub-of-Hubs content units (Epic 3)

### 1.1 Per-criterion file-touch map

| Criterion | File(s) on disk | Cycle artifact path | Source canonical |
|---|---|---|---|
| 3.1 trait definition | new `apps/desktop/src-tauri/src/rule_system_adapter.rs` | `./artifacts/epic_3/rule_system_adapter_trait-cycle_receipt.md` | new file; standards: PascalCase per identifier-discipline |
| 3.2 Pf1Adapter extraction | new `apps/desktop/src-tauri/src/pf1_adapter.rs`; existing `apps/desktop/src-tauri/src/character_hub.rs` shrunk | `./artifacts/epic_3/pf1_adapter_extraction-cycle_receipt.md` | existing character_hub.rs logic preserved |
| 3.3 StubAdapter | new `apps/desktop/src-tauri/src/stub_adapter.rs` + Stubs Registry entry | `./artifacts/epic_3/stub_adapter-cycle_receipt.md` | per wired-integration doctrine |
| 3.4 Tauri command routing | `apps/desktop/src-tauri/src/{append_to_character,recompute_character,re_save_character}.rs` | `./artifacts/epic_3/tauri_command_routing-cycle_receipt.md` | SD-24's command-surface extension |
| 3.5 UI panel adapter-aware | `apps/desktop/src/characterHub/{CharacterHubPage.tsx,LoadCharacterScreen.tsx,characterHubRuntime.ts}` | `./artifacts/epic_3/ui_panel_adapter_aware-cycle_receipt.md` | existing UI |

## 2. PCGen Runner content units (Epic 4)

### 2.1 Per-criterion file-touch map

| Criterion | File(s) on disk | Cycle artifact path | Source canonical |
|---|---|---|---|
| 4.1 pcgen-run-character.sh | new `scripts/pcgen-run-character.sh` (per cycle artifact location, but actually: bundle's `scripts/pcgen-run-character.sh`) | `./artifacts/epic_4/pcgen_run_character-script_receipt.md` | PCGen `/home/ubuntu/workspace/repos/pcgen/gradlew` + `code/testsuite/base-xml.ftl` |
| 4.2 pcgen-normalize-output.py | new `scripts/pcgen-normalize-output.py` | `./artifacts/epic_4/pcgen_normalize_output-script_receipt.md` | `src/oracle_validation/selected_parity_dimensions.rs` interface |
| 4.3 pcgen_runner_smoke.rs | new `tests/oracle_validation/pcgen_runner_smoke.rs` | `./artifacts/epic_4/pcgen_runner_smoke-cycle_receipt.md` | `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` |
| 4.4 verification cycle | reads 4.1-4.3 outputs + `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` | `./artifacts/epic_4/pcgen_runner_verification-cycle_receipt.md` | golden fixture case_id=`pf1-crb-human-fighter-level1` |

## 3. Corpus Ingest Diagnostic content unit (Epic 5)

| Criterion | File(s) on disk | Cycle artifact path |
|---|---|---|
| 5.1 corpus_ingest_diagnostic | new `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`; new `apps/desktop/src/characterHub/CorpusIngestDiagnosticPanel.tsx` | `./artifacts/epic_5/corpus_ingest_diagnostic-cycle_receipt.md` |

## 4. UI-Evaluation Discovered Defects (Epic 6)

Defects surface from the operator's UI-eval session on 2026-07-21 and accumulate in `## DISCOVERED`. Each defect = one content unit. Per-defect routing:

| Defect ID | Discovery origin | Cycle artifact |
|---|---|---|
| per-UI-finding-1..N | operator's UI-eval session | `./artifacts/epic_6/<defect-id>_cycle_receipt.md` |

## 5. Per-class residue (Epic 7)

Per-feature routing:

| Feature ID | Origin bundle | Cycle artifact |
|---|---|---|
| per-feature-gap-1..M | SD-22 Epic 4 + SD-24 Epic 4 coverage audits | `./artifacts/epic_7/<feature-id>_cycle_receipt.md` |

## 6. Cross-reference

- `./scope-draft.md §1` — Epic decomposition
- `./epic-breakdown.md §3` — per-cycle stories
- `./loop-instruction.md §4` — file-touch verification
- `../SD-24-beta-readiness-and-multiclass/content-unit-inventory.md` — predecessor bundle content units
- `src/oracle_validation/{golden_fixture,selected_parity_dimensions}.rs` — Oracle-harness schema surface (Epic 4 reads)
