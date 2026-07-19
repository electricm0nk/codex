---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit + Closure Readiness — Progress
mirrors: /home/ubuntu/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
created: 2026-07-19
snapshot_as_of: 3c9fa6a
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
| E2.3 | 2 — Operator Pre-Launch | prelaunch:board | `codex-tranche-5` kanban board set as SD-22 default | **complete** — `hermes kanban boards switch codex-tranche-5` ran locally 2026-07-19; persistent state file `~/.hermes/kanban/current` = `codex-tranche-5`; loop's per-invocation `hermes kanban --board codex-tranche-5` (per loop-instruction Step 10b) resolves to the same board. NB: session env `HERMES_KANBAN_BOARD=codex-tranche-4` was overriding the on-disk default until unset; not persisted in any shell init file. | n/a |
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

(none — E2.3 resolved by operator-local `hermes kanban boards switch codex-tranche-5` 2026-07-19; see cycle log.)

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

### cycle-2026-07-19T03:50:00Z | Epic 2 follow-up: E2.3 + receipts-doctrine amendment | n/a (operator-local + doctrine) | no card (operator-local action; amendment commits land as `1df00d0` and `3c9fa6a`) | E2.3 → **complete**; no other row touched

Operator ran `hermes kanban boards switch codex-tranche-5` from a local terminal with
`hermes` available — the persistent state file `~/.hermes/kanban/current` now reads
`codex-tranche-5`. The loop's per-invocation `hermes kanban --board codex-tranche-5`
calls (loop-instruction Step 10b) will resolve to the same board. One snag: the
session's `HERMES_KANBAN_BOARD=codex-tranche-4` env var was masking the on-disk default
in `hermes kanban boards current` output; the env var is not in any shell init file,
so it is session-scoped only and will not survive into the next launched loop session.
Loop launch will need either `unset HERMES_KANBAN_BOARD` first, or to rely on the
explicit `--board codex-tranche-5` flag (which is what Step 10b does already, so the
loop is correct as written).

Between cycles, the operator landed a doctrine amendment on top of cloud cycle 1:
- `1df00d0 feat(sd22): repo-resident receipts.md + Step 10a/10b split` — adds
  `docs/release/SD-22/receipts.md` (durability backbone for cloud cycles) and splits
  Step 10 into 10a (always-write the repo-resident receipt) and 10b (best-effort
  kanban card mint). Cycle-receipt schema lives at the top of `receipts.md`.
- The amendment post-dates the cloud cycle that wrote `progress.md`, so the cycle
  log here does not retroactively reference Step 10a. Future cycles will.

No Epic 3/4/5 cycles have started yet (correct per dependency graph: Epic 1 vacuous
done, Epic 2 fully done as of this entry). Loop is ready for the first ingest cycle
on next restart.
