---
purpose: durable-receipt-ledger
sd: SD-32
branch: tranche/12
status: opened 2026-08-22 (closure epilogue, card 13)
owner: Todd Hintzmann
scope: append-only receipt ledger for the closure epilogue's pipeline scripts (architecture truth-up, graphify update, merge-conflict resolution) — `workflow-instruction.md §13` step 4 / `docs/release/template/template.md §6`
durability: repo-resident — survives when kanban DB is unreachable
audience: operator + future-self auditing the closure pipeline
---

# SD-32 Receipts

## What this file is

Append-only ledger of closure-pipeline receipts for SD-32
(`compute-library-and-cause-closure`), per `docs/release/template/template.md §6`'s three
pipeline scripts: architecture truth-up (sub-step 2), graphify update (sub-step 3), merge-conflict
resolution (sub-step 5). Each script run appends exactly one YAML block below, regardless of
outcome — an empty diff or a failed run still gets a receipt, since the receipt is the audit trail
that the gate fired.

Per-cycle cycle receipts for SD-32's 13 cards live in `progress.md` / `artifacts/<gate>/`, not
here — this file is scoped to the closure epilogue's own three pipeline scripts only. SD-32 uses
the local-file `kanban.md`/`progress.md` pattern (Hermes retired 2026-08-01), so this file is the
one Hermes-era artifact this bundle still produces — required by `template.md §6`'s script
contract, not by `workflow-instruction.md`'s own local-file convention.

## Receipts

(Append new YAML blocks below this line, most recent at the bottom.)
