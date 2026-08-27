---
canonical: true
owner: god-emporer
bundle_id: SD-34
status: not-started — planning-ready, launch gates unrun
date: 2026-08-26
---

# SD-34 Progress

Live cycle-by-cycle record. Cycles **prepend** their entry (newest first) and update
`kanban.md` in the same commit, via `workflow-instruction.md §5`'s retry protocol.

## Status

`tranche/14` cut at `571307724f`, `0.14.0` stamped, launch checklist items 1-9, 11, 12 run.
Item 10 (widest build scope + inherited test baseline) is a separate lane's obligation and is
not reported here. Epic 1 dispatch underway.

**1 of 27 criteria complete. 1 of 26 kanban rows complete.**

Baseline at authoring, measured against `origin/develop` `ea2b3396f2`
(`content-unit-inventory.md` carries the re-derive command for each):

| Figure | Value |
|---|---|
| Corpus population | 49,438 units across 37 books |
| Ingestion | **complete** — 49,438 of 49,438 units carry a real source_file + source_line |
| DONE | 12,265 of 49,438 |
| Non-DONE | 37,173 of 49,438 |
| Largest bucket: B (record not in its table) | 11,921 of 49,438 |
| Bucket A (no engine table exists) | 8,463 of 49,438, across 9 kinds — 8 built here, `power` costed |
| Core Rulebook (vehicle 1) | 6,701 units, 1,150 DONE, 5,551 non-DONE, every bucket present |
| Ultimate Campaign (vehicle 2) | 265 units, 0 DONE — A=242, U=21, X=2 and nothing else |
| Shape-engine feedstock still unheld by the engine | 13,119 of 26,396 |

## Cycle log

### Cycle 1 — AT-34-E1-001 — every unit carries exactly one named remaining-step

**Status: complete.** New `scripts/completion_atlas.py` partitions the full 49,438-unit
`docs/work-inventory.json` into the ten buckets fixed by `decisions.md §2`
(`DONE A B C D M V U X Z`), keyed on `status` + `evidence` per `technical-design.md §1`'s
implementation table.

`python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0
overlap=0`, exit 0. Bucket counts: `DONE=12265 A=8463 B=11921 C=4388 D=1230 M=2455 V=8330
U=321 X=46 Z=19` (sum = 49,438). `A` and `U` match the epic-breakdown's independently-stated
figures (8,463 across 9 kinds; 321 split 270/51 by evidence, 140/119/62 by kind) exactly on
the first live run. `D` and `U` sub-causes are enumerated in the committed artifact, not
shrugged. Cross-checked against SD-33's inherited, independent `box_ledger.py --check`
partition: `uncovered=0 overlap=0 population=49438` — both partitions agree on the same
population.

18/18 new unit tests green (`scripts.tests.test_completion_atlas`); a live mutation on the
bucket-A marker string was proven RED for the intended reason then reverted to GREEN.
Denominator gate against this package: `files_checked=15 violations=0`. `cargo test --locked
--no-run` exits 0 at the widest workspace scope; `apps/desktop/src-tauri` not touched, not run.

**This cycle does not implement AT-34-E1-002** (the six fail-closed conditions) — a separate
criterion in the same file, picked up next. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-001_cycle_receipt.md`.

## Open blockers

**This section is not a parking lot.** An entry here is a request for an operator ruling and
it **pauses the bundle** (`../../governance/blocker-closure-doctrine.md`). It is never a
disposition, never a closure path, and no later cycle may proceed past a blocked card on its
own authority.

*(empty)*
