# SD-25 — Progress

> **Operating method:** see `./scope-draft.md` and `scripts/workflow-dispatch.sh`. This file is created on cycle 0 of Epic 2 with the deterministic seed. The orchestrator reads `## TODO` + `## DISCOVERED` and dispatches the highest-priority unclaimed item.

This file is the bundle's runtime state. The orchestrator's `progress.md` is the canonical cycle-log + status matrix; the kanban board is the durable receipt; the per-cycle `artifacts/<epic>/<cycle>_cycle_receipt.md` is the per-cycle truth.

## Status matrix (placeholder; populated by cycle 0 of Epic 2)

| Criterion | State | Cycle ID | Commit SHA | Notes |
|---|---|---|---|---|
| 1.1 Source-code identifier audit | not-started | — | — | Epic 1 fires FIRST |
| 2.1 board reachable | not-started | — | — | — |
| 2.2 branch pushed | not-started | — | — | — |
| 2.3 SD-24 closure PR merged | not-started | — | — | Tier-1 launch gate |
| 2.4 working tree clean | not-started | — | — | — |
| 2.5 doctrines loaded | not-started | — | — | — |
| 3.1 RuleSystemAdapter trait | not-started | — | — | parallel: yes |
| 3.2 Pf1Adapter extraction | not-started | — | — | parallel: yes |
| 3.3 StubAdapter | not-started | — | — | parallel: yes; requires Stubs Registry entry |
| 3.4 Tauri command routing | not-started | — | — | parallel: no |
| 3.5 UI panel adapter-aware | not-started | — | — | parallel: yes |
| 4.1 pcgen-run-character.sh | not-started | — | — | parallel: yes |
| 4.2 pcgen-normalize-output.py | not-started | — | — | parallel: yes |
| 4.3 pcgen_runner_smoke.rs | not-started | — | — | parallel: yes |
| 4.4 verification cycle | not-started | — | — | parallel: no |
| 5.1 corpus_ingest_diagnostic | not-started | — | — | serial |
| 6.1 UI-eval defect cycle shape | not-started | — | — | — |
| 6.2..6.N per-defect | not-started | — | — | spawned dynamically |
| 7.1 residue intake | not-started | — | — | — |
| 7.2..7.M per-feature | not-started | — | — | spawned dynamically |
| 8.1 Final criterion scan | not-started | — | — | fires LAST; Sonnet |
| 8.2 Architecture closure pipeline | not-started | — | — | fires LAST; Opus |
| 8.3 Release notes | not-started | — | — | fires LAST; Haiku |
| 8.4 Build version increment (→ 0.5.98) | not-started | — | — | fires LAST; Haiku |
| 8.5 PR + merge | not-started | — | — | fires LAST; Sonnet |

## TODO (deterministic seed)

- 1.1, 2.1–2.5, 3.1–3.5, 4.1–4.4, 5.1, 6.1, 7.1, 8.1–8.5

## DONE

(empty)

## DISCOVERED

(empty — populated by UI-eval findings + per-class residue intake)

## Cycle log

(empty)

## Open blockers

(empty)

---

*Per `loop-instruction.md §6 step 7`: the orchestrator updates this file in place on every cycle via the concurrent-write protocol (`§5`).*
