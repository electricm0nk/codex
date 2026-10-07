---
canonical: true
bundle_id: SD-37
---

# SD-37 Receipts

This file holds YAML receipts appended by the closure scripts: architecture truth-up (E7.7) and
graphify update (E7.8), invoked as described in `workflow-instruction.md §11`. Every graphify
receipt records the indexed SHA, together with the empty `git status --porcelain` output that was
taken immediately before the run.

(No receipts yet.)

- cycle_id: 2026-10-07T18:29:55Z
  row_or_kind: architecture:truth_up
  bundle: SD-37
  branch: tranche/17
  integration_target: develop
  branch_tip_before: b99c3d4b02
  branch_tip_after: 4abf95c122
  diff_path_count: 40296
  docs_touched: [README.md, conventions.md, corpus-ingest.md, desktop-app.md, getting-started.md, glossary.md, homebrew-and-oracle.md, overview.md, persistence.md, release-pipeline.md, rules-data-tables.md, rules-engine.md, status.md, testing.md]
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: not counted (rules-data-tables.md was rewritten whole around the data package; `git diff --stat b99c3d4b02..4abf95c122 -- docs/architecture` is the extent)
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: truth-up touched 14 doc(s) in 4abf95c122 (edited by hand before the script ran; Opus claims critic follows). The script's own lines said "docs touched none, no architecture impact": its README index parser needs a leading "||" per row and matches 0 of the 14 linked rows, so it maps no path to any doc (retro correction 1791397812938-sd37-e7-7-7cf9ac in docs/retro/events/sd37-e7-7.jsonl). The script's two verification one-liners ran and passed on 4abf95c122; the README's two one-liners (which include the crates/ and data/ roots the script's cited-path grep omits) were also run and print nothing. Before the edits the README one-liners printed 12 MISSING lines (artifacts/epic_7/E7.7_logs/base_miss.txt). Starfinder figures in the docs are release-notes R-n figures (`python3 docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.6_check.py --plant`).

- cycle_id: 2026-10-07T18:43:34Z
  row_or_kind: architecture:claims_critic
  bundle: SD-37
  branch: tranche/17
  base: 8bc1d7c47f
  model: opus
  scope: the 14 docs in 4abf95c122 (docs/architecture/*.md) and docs/release/SD-37-starfinder-1e/release-notes.md
  findings: 6
  fixed: 6
  blockers_left: 0
  docs_edited: [desktop-app.md, overview.md, rules-data-tables.md, status.md]
  cited_path_check: pass
  relative_link_check: pass
  release_notes_check: "python3 docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.6_check.py --plant -> RESULT PASS 20 figures, 3 plants REJECTED"
  log: docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.7_logs/critic_checks.log
  receipt: docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.7_cycle_receipt.md (section "Second half")

- cycle_id: 2026-10-07T18:45:25Z
  row_or_kind: graphify:update
  bundle: SD-37
  branch: 05f2a3252909fba8d61f05c4ad428280aaffa022
  integration_target: develop
  branch_tip: 05f2a325
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 0.2
  log_path: graphify-out/.truth-up-run-2026-10-07T18:45:25Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: graphify exited 1; operator to decide retry-vs-proceed (see log)
