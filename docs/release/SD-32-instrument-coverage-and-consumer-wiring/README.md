---
canonical: true
owner: god-emporer
status: planning-ready (scope authored 2026-08-13; not yet operator-launched)
date: 2026-08-13
canonical_branch: tranche/9 (scope authored here; launch branch set at launch)
kanban_board: local-file — see kanban.md
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
build_version_target: 0.<tranche>.<build> (set at launch, per SD-22 tranche rule)
---

# SD-32 — Instrument Coverage and Consumer Wiring

## Purpose

The operator has reworked the PF1e dashboard from engineering-focused to
product-focused and asked for **the product numbers to improve, assuming the
measuring systems are accurate.** This bundle is the bounded execution brief
for that request.

It exists because the raw remainder on the board is **not** the workload. The
dashboard shows 9,475 `held` + 734 `in-progress` = **10,209 movable units**.
Re-derivation (`artifacts/derive-movable-mass.py`, validated cell-for-cell
against the live payload) shows that of those 10,209:

| bucket | units | share | what it needs |
|---|---:|---:|---|
| **A** — reachable with an instrument that EXISTS | **734** | 7.2% | consumer wiring / probe coverage |
| **B** — reachable once a NAMED missing instrument is built | **8,194** | 80.3% | wiring-class classifier (1,776) + static sweep and derived check (6,418) |
| ~~**C** — structurally unreachable~~ | ~~**1,281**~~ | ~~12.5%~~ | ~~every one is a `spell`: no consumer reads a spell magnitude, at all~~ |

> **Bucket C is retracted (2026-08-13).** Its premise — "no consumer reads a
> spell magnitude, at all" — was already false when this table was written:
> `epic-31-spell-wiring` (2026-08-07) put a spell's own level on the save-DC
> cell `CharacterSheet.tsx` renders. Cards `spell-consumer-delta-probe` and
> `ground-spell-units` built the probe, proved it, and applied it: **623 spell
> units are legitimately `grounded`, 46 of them reaching `done`**, with **0**
> units moved into a worse bucket. See the `[SUPERSEDED]` banner on
> `decisions.md §5` and the `ground-spell-units` receipt in `progress.md`.
> The genuinely unreachable remainder is **113** units, not 1,281, and its
> three blockers are named in `forward-scope-register.md F2`.

and that **6,418 of bucket B additionally require a change to the measurement
pipeline that nobody has sanctioned** — the producer's `doneness_verdict()`
table has no cell that maps `static` or `derived` to `done` for any status.
Building the byte-equality sweep and the evaluator-vs-fixture check, in full,
correctly, would move **zero** units on the board as the board is wired today.

That finding is the reason this package exists as a scope document before any
code is written, per `CLAUDE.md`'s activation rules.

## The load-bearing constraint

`decisions.md §1` carries the anti-gaming rule verbatim. It is not advisory
and it is not a preamble: **you may not move a number by lowering the bar.**
Every acceptance criterion in this package is phrased as *units legitimately
reach their existing bar*, never *the count rises*. A cycle that finds itself
editing a threshold, a classifier definition, or a bucket definition to make a
count rise stops and reports it.

## Source STC contents

- `scope-draft.md` — the re-derived movable mass, the reachability split, the ranking, the epics.
- `decisions.md` — numbered decisions; `§1` is the anti-gaming rule, `§2` the measurement gate.
- `epic-breakdown.md` — dependency-ordered epics with feature seeds and acceptance.
- `kanban.md` — local-file work queue (one row per epic).
- `technical-design.md` — the instrument surfaces this bundle touches and the ones it must not.
- `acceptance-and-verification.md` — Given/When/Then per criterion.
- `risks-and-open-questions.md` — primary risks, headed by the gaming risk.
- `progress.md` — per-cycle receipt log.
- `release-notes.md` — populated at closure.
- `forward-scope-register.md` — successor work this bundle deliberately does not take.
- `artifacts/derive-movable-mass.py` — the one command behind every figure in this package.

## Authority surface

- Read-only, absolutely: `/home/ubuntu/swarm-observer/` (the dashboard JSON, the
  HTML viewer) and `~/.hermes/profiles/god-emporer/skills/release-swarm-observer/`
  (the producer). This bundle moves the reality the dashboard measures; it does
  not touch the measurement. `decisions.md §2` is the single exception path and
  it is a decision request to the operator, not a code change by a cycle.
- Writable: `src/bin/v06_work_inventory.rs`, `src/rules_core/**` equipment-effect
  surfaces, `tests/**`, and this package.

## The one command

Every number in this package comes from:

```
python3 docs/release/SD-32-instrument-coverage-and-consumer-wiring/artifacts/derive-movable-mass.py
```

It re-reads `docs/work-inventory.json`, re-applies the producer's verdict
table, and **asserts its own transcription against the live dashboard payload
before printing anything.** If the producer's table changes, it exits non-zero
rather than reporting a stale split.
