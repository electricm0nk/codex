# SD-25 — Acceptance and Verification

> **Operating method:** see `./scope-draft.md`. Per-criterion artifact map below; closure gates at the bottom.

## 1. Per-criterion verification

The closure evaluation runs every criterion through four tests:

1. **`## Status matrix` says `complete`.**
2. **Cycle artifact exists** at `./artifacts/<epic>/<cycle-id>_cycle_receipt.md`.
3. **Dual-audit gate passes** for the criterion's commit SHA (per `loop-instruction.md §6`).
4. **`cargo test --locked --tests` returns green** for any criterion touching production code.

| Criterion | Cycle artifact path | Verification command |
|---|---|---|
| 1.1 identifier audit | `./artifacts/epic_1/identifier-audit-cycle_receipt.md` | `git grep -nE '\b(sd(16\|19\|22\|23\|24)_)' apps/desktop/ apps/desktop/src-tauri/ src/ scripts/ \| wc -l` returns 0 |
| 2.1 board reachable | per Epic 2 | `hermes kanban boards` shows `codex-tranche-5` |
| 2.2 branch pushed | per Epic 2 | `git ls-remote origin tranche/5-3` resolves |
| 2.3 SD-24 closure PR | per Epic 2 | `git log origin/develop --oneline \| head -5` shows SD-24 closure as HEAD |
| 2.4 working tree clean | per Epic 2 | `git status --porcelain` returns empty |
| 2.5 doctrines loaded | per Epic 2 | `loop-instruction.md §6` dual-audit gate runs successfully |
| 3.1 trait definition | `./artifacts/epic_3/rule_system_adapter_trait-cycle_receipt.md` | Dual-audit; `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` green |
| 3.2 Pf1Adapter extraction | `./artifacts/epic_3/pf1_adapter_extraction-cycle_receipt.md` | Existing tests pass |
| 3.3 StubAdapter | `./artifacts/epic_3/stub_adapter-cycle_receipt.md` + Stubs Registry entry | Dual-audit; Stubs Registry entry exists |
| 3.4 Tauri command routing | `./artifacts/epic_3/tauri_command_routing-cycle_receipt.md` | Dual-audit; existing Tauri command tests pass |
| 3.5 UI panel adapter-aware | `./artifacts/epic_3/ui_panel_adapter_aware-cycle_receipt.md` | Dual-audit; UI test fixture passes |
| 4.1 pcgen-run-character.sh | `./artifacts/epic_4/pcgen_run_character-script_receipt.md` | Script runs end-to-end against one pilot case |
| 4.2 pcgen-normalize-output.py | `./artifacts/epic_4/pcgen_normalize_output-script_receipt.md` | Script produces normalized JSON |
| 4.3 pcgen_runner_smoke.rs | `./artifacts/epic_4/pcgen_runner_smoke-cycle_receipt.md` | `cargo test --locked --test pcgen_runner_smoke` |
| 4.4 verification cycle | `./artifacts/epic_4/pcgen_runner_verification-cycle_receipt.md` | Pilot case normalized output matches golden fixture SHA |
| 5.1 corpus_ingest_diagnostic | `./artifacts/epic_5/corpus_ingest_diagnostic-cycle_receipt.md` | Tauri command + UI panel route defined |
| 6.1 cycle shape | per Epic 6 first cycle | Dual-audit |
| 6.2..6.N per-defect | per-defect `<defect-id>_cycle_receipt.md` | Dual-audit |
| 7.1 residue intake | `./artifacts/epic_7/residue-intake-cycle_receipt.md` | Dual-audit |
| 7.2..7.M per-feature | per-feature cycle receipt | Dual-audit |
| 8.1 final criterion scan | `./artifacts/epic_8/final-criterion-scan-cycle_receipt.md` | Status matrix row counts |
| 8.2 architecture closure pipeline | `./receipts.md` (architecture-truth-up + graphify + PR) | Sub-step scripts run; `receipts.md` accumulates (Opus tier) |
| 8.3 release notes | `./release-notes.md` | Section checklist (Haiku tier) |
| 8.4 build version | per-file diff at `apps/desktop/package.json` + `tauri.conf.json` + `Cargo.toml` = `0.5.98` (Haiku tier) |
| 8.5 PR + merge | `./artifacts/epic_8/pr_merge-cycle_receipt.md` | `gh pr view` shows merged |

## 2. Closure gates

| # | Closure gate | Verification |
|---|---|---|
| CG-01 | All criteria `complete` or have a real blocker | Final scan 8.1 |
| CG-02 | Tier-1 launch-gate (SD-24 closure PR) honored | Criterion 2.3 |
| CG-03 | Dual-audit gate clean on closure PR diff | Run both audits on `git diff` |
| CG-04 | Architecture-truth-up sub-step ran (Opus) | `./receipts.md` |
| CG-05 | Graphify-update sub-step ran (graceful failure OK) | `./receipts.md` |
| CG-06 | `tranche/5-3 → develop` PR opened and merged | `gh pr view` shows merged |
| CG-07 | Release notes per template (Haiku) | `./release-notes.md` sections |
| CG-08 | Build counter at `0.5.98` (Haiku) | `apps/desktop/package.json` + `tauri.conf.json` + `Cargo.toml` |
| CG-09 | Workspace package deleted at publish | workspace-side listing shows no SD-25 directory |
| CG-10 | Hard-stop shape honored at operator-set deadline | `## Open blockers` reflects "stopped at cycle N" if applicable |

## 3. Cross-reference

- `./scope-draft.md §5 Hard-stop conditions`
- `./decisions.md §3` — per-epic concurrency
- `./decisions.md §4` — build counter inheritance
- `./epic-breakdown.md` — per-cycle stories
- `./risks-and-open-questions.md §3` — override flags
- `./loop-instruction.md §5` — concurrent-write protocol
- `./loop-instruction.md §8` — self-heal posture
