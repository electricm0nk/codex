---
canonical: true
owner: god-emporer
status: planning-ready (2026-08-13)
date: 2026-08-13
companion_to: ./kanban.md
---

# SD-32 Progress

Per-cycle receipt log. One entry per closed card, appended in close order.

## Receipt format

Every entry carries, at minimum:

- card id, cycle id, actor, start/end
- **units moved** and **units examined-and-left-alone**, each with the
  invocation that produced it (`decisions.md §8`, AT-32-011)
- for the left-alone units: the reason class, per `decisions.md §1.3`
- `./scripts/verify.sh` exit code, captured directly, not through a pipe
- the four-check no-stub audit result (`AGENTS.md` §6)
- files written

`held` figures are reported separately and never summed with `done`
(AT-32-010).

---

## Cycle 0 — Scope authoring (2026-08-13)

- **Card:** scope-the-instrument-work (pre-bundle; no card on this board)
- **Actor:** `doneness-scope`
- **Outcome:** COMPLETE. This package authored. No code written — `CLAUDE.md`
  forbids code-writing without a bounded execution brief, and this bundle had
  none until now.
- **Units moved:** 0, by design. A scope card moves no units.
- **Derivation:** `python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py`
  — transcription validated against the live dashboard payload, six of six
  doneness buckets agreeing exactly.
- **Verification:** doc-only card. `./scripts/verify.sh` FULL was **not** run and
  is **not** owed: no file under `src/`, `tests/` or `apps/` was touched. Stating
  that plainly rather than implying a sweep passed.
- **Findings surfaced:**
  1. Only two cells in the producer's verdict table produce `done`; `static` and
     `derived` have none, freezing 7,479 held units regardless of repo work
     (`decisions.md §2`).
  2. `companion`'s `NO_GROUNDING_PROBE` listing is stale — 922 grounded companion
     units exist — but the cap moves 0 companion units (`decisions.md §6`).
  3. The equipment grounding probe's key universe covers 4 of the 11 books that
     have a compiled equipment table, leaving 358 units unexamined
     (`decisions.md §4`, epic E2).
- **Files written:** this package only.

---

## Cycles

_(none yet — the bundle has not been launched)_
