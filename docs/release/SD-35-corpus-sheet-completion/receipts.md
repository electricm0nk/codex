---
canonical: true
owner: god-emporer
bundle_id: SD-35
date: 2026-09-15
---

# SD-35 Closure-Pipeline Receipts

YAML receipts appended by the bundle-closure pipeline (`../template/template.md §6`). **An empty diff still writes a receipt** — the receipt *is* the audit evidence that the gate fired.

Expected blocks, in order, all from Epic 7 (`AT-35-E7-003`):

1. `architecture:truth-up` — `architecture_truth_up.py --integration-target <target> --receipts-md <this-file> --bundle SD-35`
2. `graphify:update` — `update_graphify.py --integration-target <target> --receipts-md <this-file> --bundle SD-35`. **A non-zero graphify exit does not refuse the closure pipeline** — the failure receipt is the audit trail and the operator decides retry-vs-proceed.
3. `merge_conflict:*` — only if the PR reports conflicts.

**Ordering is load-bearing:** the final-acceptance scan (`AT-35-E7-001`), the retrospective (`AT-35-E7-002`) and the full worktree sweep happen **before** the PR opens (`workflow-instruction.md §11`). A retrospective or a stray worktree found after the PR is open is a correction cycle, not a clean closure.

## Receipts

- cycle_id: 2026-09-15T20:41:21Z
  row_or_kind: architecture:truth_up
  bundle: SD-35
  branch: tranche/15
  integration_target: develop
  branch_tip_before: tranche/
  branch_tip_after: tranche/
  diff_path_count: 108104
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: fail
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-09-15T20:44:17Z
  row_or_kind: architecture:truth_up
  bundle: SD-35
  branch: tranche/15
  integration_target: develop
  branch_tip_before: tranche/
  branch_tip_after: tranche/
  diff_path_count: 108104
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: fail
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-09-15T20:45:13Z
  row_or_kind: architecture:truth_up
  bundle: SD-35
  branch: tranche/15
  integration_target: develop
  branch_tip_before: tranche/
  branch_tip_after: tranche/
  diff_path_count: 108104
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-09-15T21:17:57Z
  row_or_kind: graphify:update
  bundle: SD-35
  branch: tranche/15
  integration_target: develop
  branch_tip: tranche/
  graphify_exit_code: 0
  outcome: success
  wall_clock_seconds: 1322.4
  log_path: graphify-out/.truth-up-run-2026-09-15T21:17:57Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: graphify succeeded

- cycle_id: 2026-09-15T21:53:00Z
  row_or_kind: merge_conflict:pr-390
  bundle: SD-35
  branch: tranche/15
  integration_target: develop
  branch_tip: d122d22b33
  pr: 390
  mergeable_state: CONFLICTING
  develop_commits_ahead_of_cut: 47
  conflicted_files: 3
  resolved_by_agent: 2
  needs_operator_ruling: 1
  outcome: escalated
  receipt_note: >
    Trial merge run with `git merge origin/develop --no-commit`, analysed, then
    `git merge --abort`; tranche/15 is byte-identical to the pushed d122d22b33.
    Two conflicts are purely additive and were resolved correctly in the trial
    (character_hub.rs and rule_system_adapter.rs: both sides add fields to
    LoadSavedCharacterResponse and its two constructors -- keep both, develop's
    ability_scores/skill_allocations/equipment_selections then SD-35's
    sheet_lines/sheet_rules_unavailable_reason). The third,
    apps/desktop/src-tauri/src/class_catalog_generic.rs, is a SEMANTIC collision
    an agent must not decide: SD-35 Epic 6 deleted tokens_from() from this
    live-side file because it reads data["raw_tokens"], while develop's v0.8
    class_spell_levels.rs:38 imports it. Take ours and develop's new feature does
    not compile; take theirs and pcgen_residue_gate.py --closure goes above zero
    and the bundle's central deliverable is undone. Per ../template/template.md
    §6 step 5 the operator resolves. Options are named in
    deferral 1789509194439-at-35-e7-003-arch-144b36 and in the PR body.
