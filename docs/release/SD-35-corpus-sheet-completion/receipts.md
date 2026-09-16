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
  branch_tip: d122d22b33
  indexed_sha: d122d22b33
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

- cycle_id: 2026-09-16T00:47:00Z
  row_or_kind: merge_conflict:pr-390-resolved
  bundle: SD-35
  branch: tranche/15
  integration_target: develop
  branch_tip_before: d122d22b33
  branch_tip_after: 0f3fdb75eb
  pr: 390
  conflicted_files: 3
  resolved_by: operator-ruling-option-1
  outcome: resolved
  root_workspace_test: "test result: ok. 3390 passed; 0 failed; 16 ignored; 0 measured; 0 filtered out"
  desktop_crate_test: "test result: ok. 633 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
  pcgen_residue_gate: "live_files=0 live_hits=0 verdict=PASS"
  completion_atlas: "population=49450 buckets=10 unclassified=0 overlap=0; DONE: 49450"
  receipt_note: >
    Operator ruling 2026-09-15 (option 1): ported class_spell_levels.rs onto
    data/sheet_rules/ so it needs no PCGen token read, rather than registering
    a gate exception. `git merge origin/develop` for real this time (the prior
    receipt's trial was aborted before a build). class_catalog_generic.rs
    resolved by taking tranche/15's side wholesale (`git checkout --ours`) --
    SD-35 Epic 6's data/sheet_rules/-reading rewrite supersedes develop's
    raw-token version outright, nothing on develop's side needed to survive.
    class_spell_levels.rs was NOT itself a git conflict (only develop had
    touched it since the fork point, so it merged in clean with develop's
    v0.8 B-9 SpellcastingStatus feature intact), but its corpus_class_facts()
    imported tokens_from/walk_json_files from class_catalog_generic.rs, which
    no longer exports them post-rewrite. Ported corpus_class_facts() to read
    data/sheet_rules/<book>/class/*.json (SheetRule, Effect::FactDeclare{name:
    "SpellType", ..}) for the same two facts (spell_type, base_selection_of)
    the raw-token reader derived, same "<Base> Class Selection" hop.
    pathfinder_unchained's `class` kind carries no sheet-rule conversion at
    all (no such directory under data/sheet_rules/), so its four Unchained
    shells now report ClassNotInCorpus rather than the NonCaster/
    CasterListNotIngested answer a PCGen-token read of the un-ingested record
    could still give -- report absence, not a value read around the gap
    (no-stub doctrine); replaced
    unchained_classes_inherit_their_base_records_caster_status with
    unchained_classes_report_class_not_in_corpus_until_their_book_converts to
    assert the honest behavior. The two mechanical conflicts
    (character_hub.rs, rule_system_adapter.rs) resolved exactly as the prior
    receipt pre-analysed: both sides add fields to LoadSavedCharacterResponse
    and its two constructors, kept both, develop's ability_scores/
    skill_allocations/equipment_selections first then SD-35's sheet_lines/
    sheet_rules_unavailable_reason. Two further breaks surfaced only on a
    real build, neither a git conflict -- API drift across develop's 47
    commits that the prior receipt's trial-merge-and-abort could not have
    caught: equipment_catalog.rs's develop-added
    catalog_weight_agrees_with_the_encumbrance_corpus_read_for_every_
    resolvable_crb_row test read `record.tokens` off equipment_id_resolve's
    result, but tranche/15's Epic 6 already changed that function to return
    the converted CorpusEquipmentRecord (no .tokens field) -- fixed to read
    the record's own settled weight_lbs directly; reach_gate.rs's
    sheet_rule_lines_cross_the_ipc_carrying_label_and_value builds a
    CreateCharacterRequest by hand and develop added a new
    additional_choices field to that struct -- added
    additional_choices: Vec::new() to the literal. Verified at the widest
    build scope: cargo build/test --workspace at the repo root and
    cargo build/test in apps/desktop/src-tauri (a separate workspace), both
    green, both gates green. `git status --porcelain` empty after folding
    the completion-atlas.json derived_at stamp and a live reclaim.sh
    retro-log append picked up mid-run.
