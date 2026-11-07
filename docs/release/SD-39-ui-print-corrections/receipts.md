---
canonical: true
bundle_id: SD-39
---

# SD-39 Receipts

This file holds YAML receipts appended by the closure scripts: architecture truth-up (E9.9) and graphify update (E9.10), invoked as described in `workflow-instruction.md §11`. Every graphify receipt records the indexed SHA, together with the empty `git status --porcelain` output taken immediately before the run.

- cycle_id: 2026-11-07T18:29:38Z
  row_or_kind: architecture:truth_up
  bundle: SD-39
  branch: tranche-19
  integration_target: origin/develop
  branch_tip_before: e5c7415d7f
  branch_tip_after: 5e962091a6
  diff_path_count: 2104
  docs_touched: [README.md, desktop-app.md, overview.md, persistence.md, release-pipeline.md, rules-engine.md, status.md, update-and-feedback.md]
  stub_graduations: ["window.print() Print path retired for the Rust/Typst PDF sinks (save_sheet_pdf, print_sheet_pdf)", "update-and-feedback: AppImage perform_install described as not wired; it dispatches to appimage_install"]
  stub_regressions: []
  obsolete_removals: 25  # removed lines, not statements: git diff -U0 e5c7415d7f..HEAD -- docs/architecture | awk '/^-[^-]/' | awk 'END{print NR}' -> 25 (git diff --stat: 47 insertions, 36 deletions, 8 files)
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: doc set last verified at tranche-18 (command count 103, Print = window.print())
  evidence_tier_after: doc set verified at tranche-19 for the rows above; testing.md, conventions.md, corpus-ingest.md not re-read line by line (see receipt Does not cover)
  receipt_note: "architecture-truth-up done by hand-mapping: the script's index parser (|| row pattern) reads 0 rows of a single-pipe README table, so its own run reports 'none'. The one-liners were run through the script with --dry-run --force and both pass. The Opus claims critic (E9.9 second half) follows and is not in this block."

- cycle_id: 2026-11-07T-claims-critic
  row_or_kind: architecture:claims_critic
  bundle: SD-39
  branch: tranche-19
  branch_tip_before: d4489b3557
  docs_touched: [corpus-ingest.md, desktop-app.md, glossary.md, persistence.md, release-pipeline.md, rules-engine.md, status.md]
  cited_path_check: pass  # was MISSING src/pcg_import/ at d4489b3557; fixed
  relative_link_check: pass
  release_notes_changes: none  # no false claim found
  receipt_note: "Opus claims critic over the truth-up's added lines and release-notes.md; seven corrections, listed in artifacts/epic_9/E9.9_cycle_receipt.md 'Claims critic'."

- cycle_id: 2026-11-07T18:42:16Z
  row_or_kind: graphify:update
  bundle: SD-39
  branch: cd873a6d0df9a92c762d19e38083794f62ce8460
  integration_target: develop
  branch_tip: cd873a6d
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 1388.6
  log_path: graphify-out/.truth-up-run-2026-11-07T18:42:16Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: "node-count guard hit (exit 1): new graph 648327 nodes vs existing graph.json 648328 (net -1), graph.json NOT written, nothing force-applied, no force flag used. Indexed SHA cd873a6d0df9a92c762d19e38083794f62ce8460 = origin/tranche-19, unfiltered git status --porcelain empty immediately before the run. The worktree had no graph.json (graphify-out is gitignored); seeded by copy from the lead checkout graphify-out (lead checkout untouched). Same -1 as SD-38 E7.10. Operator ruling needed: accept the guard-hit as non-blocking (2026-07-20 policy) or authorize a full semantic rebuild."
