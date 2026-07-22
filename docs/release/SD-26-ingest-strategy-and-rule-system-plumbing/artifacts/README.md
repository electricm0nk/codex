# SD-26 — Cycle Artifacts Index

> **Operating method:** see `./scope-draft.md` and `loop-instruction.md`. Per-cycle artifacts land in `./artifacts/<epic>/<cycle-id>_cycle_receipt.md`. This index is appended-to as cycles complete.

The epic subdirectories are pre-created at package construction time. The first cycle of each epic writes its receipt there. Per `loop-instruction.md §7`, each cycle's artifact follows the standard schema.

## Epic subdirectories

| Epic | Subdirectory | Concurrency | Subagent tier |
|---|---|---|---|
| E1 Code-Side Identifier Cleanup | `./epic_1/` | serial | Sonnet |
| E2 Oracle-Harness Comparator | `./epic_2/` | serial | Sonnet |
| E3 JSON Cache Build | `./epic_3/` | 4 parallel | Sonnet |
| E4 Book Stub Manifest | `./epic_4/` | serial research + 21 parallel | Sonnet |
| E5 Doctrine-Cost Reduction | `./epic_5/` | serial | Sonnet |
| E6 Closure Epilogue | `./epic_6/` | serial; sub-step tiering | Haiku (6.3, 6.4); Sonnet (6.1, 6.5); Opus (6.2) |

## Closure-readiness report

At Epic 6's Criterion 6.1 (Final criterion scan), the cycle produces `./artifacts/epic_6/closure-readiness-report.md`.

## Per-cycle dynamic artifacts

- **E3:** one per-book `<book>_json_cache-cycle_receipt.md` (4 total).
- **E4:** 21 per-book `<book>_stub_manifest-cycle_receipt.md` files plus the research-epic output `<research_book_stub_kind-cycle_receipt.md>`.
- **E5:** `per-class-cycle-floor-measurement.md` (audit output).
- **E6:** `final-criterion-scan-cycle_receipt.md`, `pr_merge-cycle_receipt.md`, `release-notes-cycle_receipt.md`.

## Data artifacts (not part of cycle receipts; repo-resident durable JSON)

- `data/corpus/<book>/<content_kind>/<content_id>.json` — 4 in-scope books' JSON cache.
- `data/stubs/<book_id>.json` — 21 future-state book stub manifests.
- `governance/wired-integration-stubs-registry.md` — `book_stub` kind entries (21 added by E4).

These are repo-resident (committable) per operator directive 2026-07-21 17:39:26 and durable SD-to-SD per 2026-07-21 15:36:12.
