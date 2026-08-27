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

**3 of 27 criteria complete. 3 of 26 kanban rows complete.**

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

### Cycle 3 — AT-34-E1-003 — the missing engine tables are enumerated and their book coverage mapped

**Status: complete.** New `scripts/missing_engine_tables.py` re-derives bucket A (`status ==
not-ingested`, evidence contains `has_no_engine_table`) directly from `docs/work-inventory.json`
and reports, per kind: unit count, per-book breakdown, the exact `not_ingested(...)`
engine-surface citation in `v06_work_inventory.rs` a real table would replace, and which books'
entire bucket-A population zeroes out once that kind's table exists.

`python3 scripts/missing_engine_tables.py --check` → `population=8463 kinds=9
citation_failures=0`, exit 0. Per-kind: `ability=4337 template=2248 trait=487 deity=459
power=421 domain=183 skill=149 language=136 companion=43` (sum = 8,463, matching
`completion_atlas.py`'s committed `buckets.A.count` exactly). Core Rulebook's slice
(`ability=471 template=262 skill=110 domain=34 language=22 deity=21 companion=14`, summing to
934 of `core_rulebook`'s 6,701 units) matches `technical-design.md §4` exactly and cross-checks
against `completion_atlas.py --by-book`'s independently-computed `core_rulebook A=934`.
`ultimate_campaign`'s slice (`ability=88 trait=154`, summing to 242 of 265 units, 91.3%)
confirms the epic-breakdown's "almost-single-bucket book" claim, cross-checked the same way.

`zero_bucket_a_books` (books a single kind's table alone would fully clear of bucket A):
`ability` → `inner_sea_faiths`; `language` → `inner_sea_temples`; `template` →
`inner_sea_intrigue`, `ultimate_intrigue`; the other 6 kinds → none (every book they touch also
carries a second bucket-A kind, so both tables are needed).

**Notable finding along the way:** a 10th `Kind::MonsterAbility` match arm in
`v06_work_inventory.rs` emits the same `has_no_engine_table` marker shape but contributes zero
live bucket-A units — its 3,806 units are already `text-complete`/`grounded`/verified, with only
13 `not-ingested` units, all landing in bucket B. Confirmed by reading the corpus data, not the
code path alone, before concluding the population is genuinely 9 kinds not 10 — the same
field-name-vs-field-meaning trap `decisions.md §12` L1 names.

12/12 new unit tests green (`scripts.tests.test_missing_engine_tables`), covering per-kind
counts, non-bucket-A exclusion, `zero_bucket_a_books` derivation, the engine-surface citation
(including a live re-check against the committed source), and a fail-closed
`UnknownKindError` for any future kind reaching bucket A with no citation entry. Denominator
gate against this package: `files_checked=15 violations=0`. `cargo test --locked --no-run`
exits 0 at `2ec0462736` (Python-only change; no Rust source touched); `apps/desktop/src-tauri`
not touched, not run. `docs/work-inventory.json` untouched — zero movement across all four
buckets; this cycle is a reclassification (finer view of the already-fixed bucket-A partition),
not new closure work. Receipt: `artifacts/epic-1-atlas/AT-34-E1-003_cycle_receipt.md`.

### Cycle 2 — AT-34-E1-002 — the atlas fails closed on six conditions

**Status: complete.** `scripts/completion_atlas.py` extended in place with the five remaining
fail-closed conditions on top of AT-34-E1-001's `unclassified`/`overlap` gate: (3) a `DONE`
unit whose evidence does not support it, (4) a bucket with no named clearing mechanism, (5) a
`derived_at` SHA that is not an ancestor of `HEAD` (staleness gate), (6) a bucket whose
definition does not cite the `file:line` that emits the evidence string it keys on, or whose
citation no longer resolves/matches at `HEAD`.

Live, unmutated: `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10
unclassified=0 overlap=0 done_evidence_violations=0 missing_clearing_mechanisms=0
stale_derived_at=False citation_failures=0`, exit 0. All ten buckets carry a real, verified
`file:line` citation into `src/bin/v06_work_inventory.rs`.

**Six RED→GREEN mutation proofs, one per condition, in
`artifacts/epic-1-atlas/fail-closed-proofs.md`.** Notable finding along the way: the naive
condition-3 design (reuse the A/B/C bucket markers verbatim as "must never appear in DONE
evidence") would have flagged 245 real, legitimate `DONE` units carrying `explanation_id` —
confirmed against the live corpus and excluded, with the exclusion documented in code and in
the proofs file (the same "field name vs. field meaning" trap condition 6 itself targets,
caught here before it shipped).

38/38 unit tests green (20 new + 18 inherited). Denominator gate against this package:
`files_checked=15 violations=0`. `cargo test --locked --no-run` exits 0 at the widest
workspace scope (run at `ceac19da29`); `apps/desktop/src-tauri` not touched, not run.
`docs/work-inventory.json` untouched — zero movement across all four buckets. Receipt:
`artifacts/epic-1-atlas/AT-34-E1-002_cycle_receipt.md`.

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
