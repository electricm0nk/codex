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
