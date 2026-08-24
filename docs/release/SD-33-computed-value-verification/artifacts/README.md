---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Cycle Artifacts

One subdirectory per epic. Every cycle writes its receipt to `<epic-dir>/<cycle-id>_cycle_receipt.md` per `../workflow-instruction.md §7`.

| Directory | Epic | Holds |
|---|---|---|
| `epic-1-instruments/` | 1 | box mutation proofs, probe-surface census, denominator-gate proof |
| `epic-2-oracle-harness/` | 2 | build transcript, character round-trip, harness fixtures, Path ruling |
| `epic-3-engine-coverage/` | 3 | coverage-gap root-cause, per-family tables, regenerated corpus-wide run |
| `epic-4-unknown-classification/` | 4 | `unknown` root-cause, reclassification evidence |
| `epic-5-reverification/` | 5 | per-unit `(ours, oracle, verdict)` rows, disagreement dossiers |
| `epic-6-closure/` | 6 | final-acceptance scan, sweep counts, closure receipts |

## Receipt discipline

Beyond the schema in `../workflow-instruction.md §7`:

- **Every figure carries the command that produces it and its denominator** (`../decisions.md §2`). A bare percentage fails `scripts/verify.sh --only denominator-gate`.
- **Movement is reported in four buckets** — closure / reclassification / reachability / instrument-correction. A count that dropped because the measurement changed is **not** closure.
- **Coordinates only, never names.** Never write a Product Identity term or blacklist item into a receipt, test name, test constant, kanban row, or commit message (`../technical-requirements.md` R5).
- **A figure derived from the pinned corpus names `PCGEN_ORACLE_SHA`.**

## Fixture discipline

A fixture's expected value is transcribed from bytes the engine's or harness's own read path does **not** touch. **A fixture built from the same file the code reads is a mirror, not a check** — it will happily validate a fabricated value.

## What does not belong here

- Narrative status writing — that goes in `../progress.md`.
- Anything that duplicates `../kanban.md`'s Notes column, which is a pointer only.
