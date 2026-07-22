# SD-26 — Acceptance and Verification

## 1. Per-criterion verification

Per the standard 4-test pattern.

| Criterion | Cycle artifact path | Verification command |
|---|---|---|
| 1.1 identifier audit | `./artifacts/epic_1/identifier-audit-cycle_receipt.md` | `git grep -nE '\b(sd(16\|19\|22\|23\|24)_)' apps/desktop/ apps/desktop/src-tauri/ src/ scripts/ data/ \| wc -l` returns 0 |
| 2.1 comparator | `./artifacts/epic_2/comparator-cycle_receipt.md` | `cargo test --locked --test sd26_comparator` |
| 2.2 normalization | `./artifacts/epic_2/normalization-cycle_receipt.md` | `cargo test --locked --test sd26_normalization` |
| 2.3 parity_report | `./artifacts/epic_2/parity_report-cycle_receipt.md` | Manual review + cargo test |
| 2.4 pcgen_runner | `./artifacts/epic_2/pcgen_runner_rust-cycle_receipt.md` | `cargo test --locked --test sd26_pcgen_runner` |
| 2.5 verification cycle | `./artifacts/epic_2/pilot_case_oracle_checked-cycle_receipt.md` | Pilot case fixture's `current_claim_status=oracle_checked` |
| 3.1 core_rulebook cache | `./artifacts/epic_3/core_rulebook_json_cache-cycle_receipt.md` | `cargo test --locked --test sd26_cache_core_rulebook` |
| 3.2 advanced_players_guide cache | `./artifacts/epic_3/apg_json_cache-cycle_receipt.md` | Similar |
| 3.3 advanced_class_guide cache | `./artifacts/epic_3/acg_json_cache-cycle_receipt.md` | Similar |
| 3.4 beastiary cache | `./artifacts/epic_3/beastiary_json_cache-cycle_receipt.md` | Similar |
| 4.1 research epic | `./artifacts/epic_4/research_book_stub_kind-cycle_receipt.md` | Dual-audit; template at `governance/wired-integration-stubs-registry.md` |
| 4.2..4.22 per-book | per-book `<book>_stub_manifest-cycle_receipt.md` | Dual-audit; Stubs Registry entry exists |
| 5.1 doctrine-cost audit | `./artifacts/epic_5/per-class-cycle-floor-measurement.md` | Pre-cut vs post-cut cycle floor measured |
| 6.1 final criterion scan | `./artifacts/epic_6/final-criterion-scan-cycle_receipt.md` | Sonnet |
| 6.2 architecture closure pipeline | `./receipts.md` | Opus (adversarial-verify) |
| 6.3 release notes | `./release-notes.md` | Haiku |
| 6.4 build version (`0.5.99`) | per-file diff | Haiku |
| 6.5 PR + merge | `./artifacts/epic_6/pr_merge-cycle_receipt.md` | Sonnet |

## 2. Closure gates

| # | Closure gate | Verification |
|---|---|---|
| CG-01 | All 17 declarative + 21 dynamic criteria `complete` or have a real blocker | Final scan 6.1 |
| CG-02 | Tier-1 launch-gate (SD-25 closure PR) honored | E2.5 verification |
| CG-03 | Pilot case upgraded to `oracle_checked` | E2.5 |
| CG-04 | All 4 in-scope books have JSON cache coverage | `find data/corpus -type f -name '*.json' \| wc -l` > 0 per book |
| CG-05 | All 21 future-state books have Stubs Registry entries | `grep -c '^book_id:' governance/wired-integration-stubs-registry.md` ≥ 21 |
| CG-06 | Dual-audit gate clean on closure PR diff | Run both audits on `git diff` |
| CG-07 | Architecture-truth-up sub-step ran (Opus) | `./receipts.md` |
| CG-08 | Graphify-update sub-step ran | `./receipts.md` |
| CG-09 | `tranche/5-4 → develop` PR opened and merged | `gh pr view` |
| CG-10 | Release notes per template (Haiku) | Sections |
| CG-11 | Build counter at `0.5.99` (Haiku) | `apps/desktop/package.json` + `tauri.conf.json` + `Cargo.toml` |
| CG-12 | Workspace package deleted at publish | workspace-side listing shows no SD-26 directory |
| CG-13 | Hard-stop shape honored at deadline | `## Open blockers` reflects "stopped at cycle N" if applicable |

## 3. Cross-reference

- `./scope-draft.md §5 Hard-stop conditions`
- `./decisions.md §3` — per-epic concurrency
- `./decisions.md §4` — build counter
- `./epic-breakdown.md` — per-cycle stories
- `./risks-and-open-questions.md §3` — override flags
- `./loop-instruction.md §5` — concurrent-write
