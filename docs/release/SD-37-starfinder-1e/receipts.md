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
