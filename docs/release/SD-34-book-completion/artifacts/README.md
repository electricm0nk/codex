---
canonical: true
owner: god-emporer
bundle_id: SD-34
date: 2026-08-26
---

# SD-34 Cycle Artifacts

One subdirectory per epic, each seeded with a `.gitkeep` so it survives the commit. Every cycle
writes its receipt to
`artifacts/<epic-dir>/<criterion-id>_cycle_receipt.md`, using the schema in
`../workflow-instruction.md §7`.

| Directory | Epic | Expected contents |
|---|---|---|
| `epic-1-atlas/` | 1 — Completion Atlas | `completion-atlas.json`, `missing-engine-tables.json`, `shape-engine-boundary.md`, `fail-closed-proofs.md` |
| `epic-2-tables/` | 2 — Build 8 of 9 tables | per-table build transcripts, `fail-closed-proofs.md`, `table-build-rate.json` |
| `epic-3-core-rulebook/` | 3 — Core Rulebook to zero | `step-cost-ledger.json`, `core-rulebook-completion-manifest.json`, **`atlas-defects.md`** |
| `epic-4-ultimate-campaign/` | 4 — Ultimate Campaign to zero | `step-cost-ledger.json`, `ultimate-campaign-completion-manifest.json` |
| `epic-5-forward-plan/` | 5 — Price 35 books | `forward-plan.json`, `capability-register.json` |
| `epic-6-closure/` | 6 — Closure epilogue | final-acceptance scan receipt, closure-readiness report |

## The three artifacts that ARE the bundle

**`epic-1-atlas/completion-atlas.json`** — every one of 49,438 units in exactly one named
bucket, `unclassified=0`. If this is wrong, nothing downstream is trustworthy.

**`epic-3-core-rulebook/atlas-defects.md`** — every remaining step encountered that the atlas
did not predict. **An empty file is a valid and excellent result. An absent file is a
failure.** This is the mechanism behind the operator's requirement that "everything I think
we are done, you surface 3 more things" stops here.

**`epic-5-forward-plan/forward-plan.json`** — per book, per bucket: units, clearing mechanism,
projected cost, and **the measured rate and sample size each projection rests on**. A confident
number from a thin sample is a failure this program has hit repeatedly; the sample size is
therefore a required field, not a footnote.

Its rates come from three ledgers: `epic-2-tables/table-build-rate.json` (what a table costs,
with the spread across eight), plus a step-cost ledger from each vehicle book — a **deep** one
(Core Rulebook, every bucket) and a **shallow** one (Ultimate Campaign, effectively one).
Two shapes give a range instead of one blended number.

## Receipt requirements

Every receipt carries, per `../workflow-instruction.md §7`:

- the **figures** row — every number with its re-derive command **and its denominator**
- the **row-count command output** row — the literal output of the count on that cycle's own
  artifact (`../decisions.md §4`)
- the **build scope verified** row — `--no-run` exit, workspace result, desktop crate result
- the **four buckets** row — closure / reclassification / reachability / instrument-correction
- the **build scope verified** row names the **SHA it ran at** (`../decisions.md §12` L7)
- the **sweep population** row — `corpus_literal_sweep` examined-count before → after any corpus
  change, delta equal to records added (`../decisions.md §12` L8)
- **A figure derived from the pinned corpus names `PCGEN_ORACLE_SHA`** (from
  `scripts/pcgen-oracle-pin.env`; `7f818006e371188e5717fd18d74d18a420747fc6` at authoring). A
  figure without a stated oracle commit is a number that may have drifted.

`- **Status:** <x>` is a **bullet**, not a `## Status` heading. Prior bundles drifted on this
and it breaks naive parsers.

## Completion manifests

`core-rulebook-completion-manifest.json` carries **one row per unit**: its final state and the
evidence pointer establishing it. AT-34-E6-001's scan independently re-derives a random
sample. A unit whose evidence pointer does not resolve is a blocking shortfall.
