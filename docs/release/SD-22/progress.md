---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit + Closure Readiness — Progress
mirrors: /home/ubuntu/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
created: 2026-07-19
snapshot_as_of: 233c426
---

# SD-22 — Progress

## SD-22 STATUS: LOOP RUNNING (cycle 1)

Loop launched 2026-07-19 per `decisions.md §5` amendments (corpus generation in-bundle,
`/batch` deferred). Running from a remote execution session — `hermes` CLI is not
available in this environment, so kanban card minting (Step 10) is recorded here as a
markdown note instead of a live board card; the operator should backfill cards on
`codex-tranche-5` from this log when next at a terminal with `hermes` available.

---

SD-22's own progress doc. Loop's claim protocol and per-cycle history live here under
`## SD-22 cycles`.

## Status matrix

| ID | Epic | row_or_kind | Description | Status | Commit |
|---|---|---|---|---|---|
| E1.1 | 1 — Identifier Cleanup | identifier:audit | `sd22_\|SD22_\|Sd22\|SD-22-[A-Z][0-9]` grep across `apps/desktop/`, `apps/desktop/src-tauri/`, `src/rules_core/` | **complete** (0 hits; defensive audit found nothing to clean) | n/a (verification-only) |
| E1.2 | 1 — Identifier Cleanup | identifier:regression_check | Per-rename tests pass | **complete (vacuous)** — no renames needed; baseline `cargo test --locked` green (14 tests, 0 failed) before Epic 3/4/5 work began | n/a |
| E2.3 | 2 — Operator Pre-Launch | prelaunch:board | `codex-tranche-5` kanban board set as SD-22 default | **blocked (environment)** — `hermes` CLI unavailable in this remote session; operator must run `hermes kanban boards switch codex-tranche-5` locally | n/a |
| E2.4 | 2 — Operator Pre-Launch | prelaunch:branch | `tranche/5` pushed to origin | **complete** — `git ls-remote origin tranche/5` = `233c426...` matches local HEAD | 233c426 |
| E2.5 | 2 — Operator Pre-Launch | prelaunch:no_inflight | No other `claude` processes touching `rules_tables/<book>/` | **complete** — `ps -eo pid,etime,stat,cmd \| grep claude` shows only this session's own process | n/a |
| E3.6-9 | 3 — APG ingest | ingest:apg_class | Alchemist (cycle 1 of 8) | see cycle log | pending |
| E4.10-13 | 4 — ACG ingest | ingest:acg_class | Alchemist-ACG (cycle 1 of 10) | see cycle log | pending |
| E5.14-17 | 5 — Bestiary 1 ingest | ingest:beastiary1_subset | Subset 01 (CR 1: Goblin/Kobold/Orc/Skeleton/Zombie) | see cycle log | pending |
| E6.18-21 | 6 — DM Toolkit | dm:encounter, dm:party_cr | Not started (requires ≥1 book ingested) | open | — |
| E7.22-26 | 7 — Closure Epilogue | closure:* | Not started (fires last) | open | — |
| E8.27-30 | 8 — Build Version | version:* | Not started | open | — |
| E9.31 | 9 — Closure Readiness | closure_readiness:* | Not started (fires after Epic 8, before Epic 7) | open | — |

## Open blockers

- **E2.3** — `codex-tranche-5` kanban board switch is an operator-local `hermes` action; not reachable from this remote execution session. Non-blocking for cycle work (Step 10 card-mint substituted with this progress-doc entry per cycle); operator should reconcile the board when next at a local terminal.

## Cycle log

### cycle-2026-07-19T00:00:00Z | Epic 1 + Epic 2 pre-flight | n/a (verification-only) | no card (hermes unavailable; logged here) | open → **complete** (E1.1, E1.2, E2.4, E2.5); E2.3 → **blocked (environment)**

Ran the Epic 1 identifier-audit grep gate scoped to SD-22-specific patterns
(`sd22_|SD22_|Sd22|SD-22-[A-Z][0-9]`) across `apps/desktop/`, `apps/desktop/src-tauri/`,
`src/rules_core/` — zero hits. (The broader `sd[0-9]+_` pattern in the criterion's
verification command also matches pre-existing `sd19_*`/`sd13_*`/`sd16_*` identifiers
from already-shipped, unrelated spec domains — those are out of Epic 1's scope per
`epic-breakdown.md`'s own scope-doctrine note and AGENTS.md's no-scope-expansion rule;
not touched.) Ran baseline `cargo test --locked` — 14 tests passed, 0 failed, confirming
a clean starting tree before Epic 3/4/5 cycles begin. Verified `tranche/5` is pushed to
origin (E2.4) and no other `claude` processes are in-flight (E2.5). E2.3 (kanban board)
requires operator-local `hermes`, unavailable here — recorded as a blocker, non-gating.
