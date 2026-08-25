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

- cycle_id: 2026-08-22T23:51:06Z
  row_or_kind: architecture:truth_up
  bundle: SD-32
  branch: 89a71b2832d50a0d432c6e0fb0fb8ff1f96fa39a
  integration_target: develop
  branch_tip_before: 89a71b28
  branch_tip_after: 89a71b28
  diff_path_count: 25618
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: fail
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-08-22T23:56:09Z
  row_or_kind: architecture:truth_up
  bundle: SD-32
  branch: 8e7b1420561f4b4d63de4c125de3eea5abab427f
  integration_target: develop
  branch_tip_before: 8e7b1420
  branch_tip_after: 8e7b1420
  diff_path_count: 25618
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: fail
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-08-22T23:58:39Z
  row_or_kind: architecture:truth_up
  bundle: SD-32
  branch: 8053a3d8c046103aff31a64ceb05bde1f3b2102a
  integration_target: develop
  branch_tip_before: 8053a3d8
  branch_tip_after: 8053a3d8
  diff_path_count: 25618
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-08-22T23:58:55Z
  row_or_kind: graphify:update
  bundle: SD-32
  branch: 0721aabddfbd569d3546d74f96dacba082f750a5
  integration_target: develop
  branch_tip: 0721aabd
  graphify_exit_code: 0
  outcome: success
  wall_clock_seconds: 1489.5
  log_path: graphify-out/.truth-up-run-2026-08-22T23:58:55Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: graphify succeeded

- cycle_id: 2026-08-23T00:24:38Z
  row_or_kind: merge_conflict:post_pr
  bundle: SD-32
  branch: tranche/12
  integration_target: develop
  branch_tip_before: e2bbbae7
  branch_tip_after: e2bbbae7
  outcome: unknown
  conflict_files: []
  pr_number: 375
  wall_clock_seconds: 0.4
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: 0 conflict(s) found; loop self-heals

- cycle_id: 2026-08-25T01:11:27Z
  row_or_kind: architecture:truth_up
  bundle: SD-32
  branch: 88449a629705a8067728306ca660cf0e00f27243
  integration_target: origin/develop
  branch_tip_before: 88449a62
  branch_tip_after: 88449a62
  diff_path_count: 38225
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-08-25T01:11:47Z
  row_or_kind: graphify:update
  bundle: SD-32
  branch: 88449a629705a8067728306ca660cf0e00f27243
  integration_target: origin/develop
  branch_tip: 88449a62
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 0.2
  log_path: graphify-out/.truth-up-run-2026-08-25T01:11:47Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: graphify exited 1 (no pre-existing graph.json in the scratch worktree used
    for this run -- "run /graphify first"); per operator directive 2026-07-20 a non-zero
    graphify exit does not refuse closure. Pipeline continues.
