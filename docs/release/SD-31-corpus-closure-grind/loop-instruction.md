---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
---

# SD-31 Loop Instruction

**`SD-30-class-feature-archetype-bundle/loop-instruction.md` governs this package's cycles, with the
overrides below.** Do not fork a second copy of that file's cycle procedure (branch-state check,
disk-budget step, verify.sh discipline, stop-vs-press-on rules, retrospective-event emission,
unattended-mode authorization) — read it directly each cycle. This file states only what differs.

## Overrides

1. **Card source:** this package's own `kanban.md`, not SD-30's — `epic-1-measurement` through
   `epic-6-cloud-fanout`.
2. **Cross-SD PI-gate check is a cycle-0 precondition, every cycle in Epic 3/4/5:** before claiming a
   card that touches a specific book, read `SD-30-class-feature-archetype-bundle/kanban.md` and confirm
   `epic-3-pi-gate`'s state for that book (cite the SD-30 `progress.md` receipt showing SD30-E3-F2/F3
   `COMPLETE` for it). A cycle that skips this check is out of protocol (`acceptance-and-verification.md
   AT-31-003`).
3. **Progress receipts land in this package's own `progress.md`**, not SD-30's — even though the cycle
   consumes an SD-30 gate, the receipt for work done under this package's epics is recorded here.
4. **Concurrency/hardware numbers:** use `SD-30-class-feature-archetype-bundle/decisions.md §47`'s
   2026-08-14 capture (8 cores / 45GB / 968GB at 19% used, 3-agent cap) — same box, same session,
   nothing to re-derive at split time; re-derive per the standing rule if a cycle finds the numbers
   stale.
5. **Race/race_trait cross-SD dependency (Epic 4-F3/F4):** a cycle working these cards checks whether
   `SD-32-engine-capability-builds/`'s race-chassis epic has landed before assuming the ~2,894-unit
   chassis-blocked remainder is still out of reach; if it has landed, re-derive the workable pool before
   planning further rounds.

## What is not overridden

Everything else in SD-30's `loop-instruction.md` — the per-cycle procedure shape, the disk reclamation
step, the "generated, never hand-maintained" figure discipline, Decision §22's Workflow-tool dispatch
mode, Decision §24's stop-vs-press-on rules, Decision §48's Opus-high orchestrator mandate — applies to
this package's cycles unchanged.
