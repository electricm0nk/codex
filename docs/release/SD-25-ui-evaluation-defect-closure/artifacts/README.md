# SD-25 — Cycle Artifacts Index

> **Operating method:** see `./scope-draft.md` and `loop-instruction.md`. Per-cycle artifacts land in `./artifacts/<epic>/<cycle-id>_cycle_receipt.md`. This index is appended-to as cycles complete.

The epic subdirectories are pre-created at package construction time. The first cycle of each epic writes its receipt there. Per `loop-instruction.md §7`, each cycle's artifact follows the schema:

```markdown
# Cycle <cycle-id> — <epic-name> / Criterion <n>
- **Card ID:** t_<hex>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violation list>
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS / <violation list>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Status:** complete | returned-to-backlog | DISCOVERED-forked
- **Notes:** <judgment calls, deferred items, audit-exclusion requests>
- **Discovery forwards:** <list of ## DISCOVERED entries added>
- **Next-cycle plan:** <what the next cycle picks up>
```

## Epic subdirectories

| Epic | Subdirectory | Concurrency | Subagent tier |
|---|---|---|---|
| E1 Code-Side Identifier Cleanup | `./epic_1/` | serial | Sonnet |
| E2 Operator Pre-Launch | `./epic_2/` | serial | Sonnet |
| E3 Character Hub as Hub of Hubs | `./epic_3/` | 4 parallel + 1 serial | Sonnet |
| E4 PCGen Runner Scaffolding | `./epic_4/` | 3 parallel + 1 serial | Sonnet |
| E5 Corpus Ingest Diagnostic | `./epic_5/` | serial | Sonnet |
| E6 UI-Eval Discovered Backend Defects | `./epic_6/` | serial | Sonnet |
| E7 Deferred Per-Class Work | `./epic_7/` | serial | Sonnet |
| E8 Closure Epilogue | `./epic_8/` | serial | Haiku (8.3, 8.4); Sonnet (8.1, 8.5); Opus (8.2) |

## Closure-readiness report

At Epic 8's Criterion 8.1 (Final criterion scan), the cycle produces `./artifacts/epic_8/closure-readiness-report.md` summarizing the bundle's evaluation.

## Per-cycle dynamic artifacts

- **E3:** 4 parallel cycle receipts (3.1, 3.2, 3.3, 3.5) + 1 serial (3.4).
- **E4:** 3 parallel cycle receipts (4.1, 4.2, 4.3) + 1 serial (4.4) verification.
- **E6:** per-defect `<defect-id>_cycle_receipt.md` files (dynamic count).
- **E7:** per-feature `<feature-id>_cycle_receipt.md` files (dynamic count).
- **E8:** `final-criterion-scan-cycle_receipt.md`, `pr_merge-cycle_receipt.md`, `release-notes-cycle_receipt.md` (Haiku).
