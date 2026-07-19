---
purpose: durable-receipt-ledger
sd: SD-22
branch: tranche/5
status: opened 2026-07-19
owner: Todd Hintzmann
scope: per-cycle post-mortem record (one block per /loop cycle)
durability: repo-resident — survives when kanban DB and ~/workspace/ are both unreachable
audience: operator + future-self auditing a cloud-run bundle
---

# SD-22 Receipts

## What this file is

This file is the repo-resident ledger of cycle-level receipts for SD-22. Every cycle
that runs under `/loop 60m /goal docs/release/SD-22/loop-instruction.md` (per `decisions.md §5`; `/batch` deferred)
appends exactly one block to this file before exiting, mirroring the kanban card
body schema defined in `loop-instruction.md` §"Step 10 — Mint the kanban card".

This is the **durability backbone** for cloud-run cycles. In a cloud environment:

- The kanban DB (`~/.hermes/profiles/<profile>/...`) may be unreachable, so the
  `hermes kanban create` call in Step 10 fails. The cycle continues; this file
  captures the receipt instead.
- The workspace progress file (`~/workspace/SD-22-content-source-ingest-and-dm-toolkit-progress.md`)
  lives outside the repo and is ephemeral across a cloud sandbox. The mirror is
  this file, plus the commit SHA on `tranche/5`.

If this file is missing for a cycle, the cycle **did not run** in a verifiable way —
regardless of whether the kanban card exists.

## What this file is NOT

- Not a substitute for the kanban board when the kanban board is reachable. The kanban
  card is the primary post-mortem surface; this file is the **fallback** written first.
- Not a redesign of `progress.md`. The workspace progress file remains the operator-facing
  status matrix; this file is the durable audit trail.
- Not freeform prose. Each block conforms to the schema below.

## Schema (one block per cycle)

Append the following block at the END of this file after each cycle. Do not edit
or delete prior blocks; if a cycle is rolled back or re-run, write a new block
referencing the prior block's `cycle_id`. Edit in place to keep the diff auditable.

```yaml
- cycle_id: <ISO-8601 timestamp, e.g. 2026-07-20T03:42:00Z>
  epic: <SD-22 epic number, e.g. 3>
  criterion: <criterion key, e.g. apg_alchemist or identifier_cleanup_wave_1>
  criterion_section: <scope-doc section reference, e.g. "§1.3 Epic 3 — APG content-source ingest">
  row_or_kind: <one of: ingest:apg_class | ingest:acg_class | ingest:beastiary1_subset | dm:encounter | dm:party_cr | identifier:rust_tauri | identifier:ts_function_or_class | version:patch_bump | version:build_label_format | version:closure_checklist | closure_readiness:eval | closure_readiness:self_heal | closure_readiness:dispatch>
  evidence_tier_before: <previous matrix row state>
  evidence_tier_after: <new matrix row state after this commit>
  branch_tip_before: <short SHA on tranche/5 before this cycle's push>
  branch_tip_after: <short SHA on tranche/5 after this cycle's push>
  merge_receipt_sha: <commit SHA on tranche/5 (== branch_tip_after when direct commit)>
  cycle_artifact_path: <path under docs/release/SD-22/artifacts/, e.g. cycles/2026-07-20T03-42-00Z-apg-alchemist.md>
  red_phase_evidence: <one-line summary or "see cycle_artifact_path:Red-phase evidence">
  green_phase_evidence: <one-line summary or "see cycle_artifact_path:Green-phase evidence">
  cargo_test_summary: <test summary string, e.g. "1/1 green; full suite green; clippy clean">
  clippy_signal: <clean | dirty>
  cycle_timing_seconds: <N>
  self_heals_applied: <list, empty [] if none>
  next_required_uplift: <recommendation for next iteration, or "none">
  corpus_input_path: <path to operator-supplied structured-data file if applicable, else "n/a">
  rule_set_used: <Apg | Acg | Bestiary1 | n/a>
  kanban_card: <card id, or "no card: <reason>" — e.g. "no card: board unreachable from cloud sandbox">
  progress_file_updated: <yes | no: <reason>>
  artifacts_written: <list of paths under docs/release/SD-22/artifacts/, empty [] if none>
  notes: <freeform one-line note, empty string if nothing>
```

## Cycle log

(One block per cycle appended below this line in YAML-frontmatter-free blockquote form.
Schema above is the canonical shape; cycles do NOT need to repeat the schema fields
that are already self-evident from the cycle_artifact_path.)

- cycle_id: 2026-07-19T04:00:00Z
  epic: 3
  criterion: apg_alchemist
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (Alchemist)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open
  evidence_tier_after: blocked
  branch_tip_before: cd9e88b
  branch_tip_after: cd9e88b
  merge_receipt_sha: "n/a — no commit landed"
  cycle_artifact_path: "n/a — blocked before RED phase; see progress.md ## Open blockers"
  red_phase_evidence: "not started"
  green_phase_evidence: "not started"
  cargo_test_summary: "not re-run; no production code touched (last known-green: 14/14)"
  clippy_signal: "n/a"
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "operator decision needed: supply a real corpus/reference source, narrow Epic 3/4/5 acceptance shape to formula-derivable data only (mirroring crb/class_tables.rs), or explicitly re-affirm memory-recalled content is acceptable outside this bundle's own self-referential docs"
  corpus_input_path: "n/a — not generated (fabrication-risk hard stop)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: []
  notes: "WebFetch to aonprd.com and d20pfsrd.com both 403'd; no verifiable source reachable; see progress.md Open blockers for full reasoning"
