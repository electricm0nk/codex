---
canonical: true
bundle_id: SD-38
---

# SD-38 Receipts

This file holds YAML receipts appended by the closure scripts: architecture truth-up and claims critic (E7.9) and graphify update (E7.10), invoked as described in `workflow-instruction.md §11`. Every graphify receipt records the indexed SHA, together with the empty `git status --porcelain` output taken immediately before the run.

- cycle_id: 2026-10-15T20:31:56Z
  row_or_kind: architecture:truth_up
  bundle: SD-38
  branch: 296e80e7bbdd96a8c5333d6a1563b77747b4ed81
  integration_target: develop
  branch_tip_before: 9d23977679
  branch_tip_after: 296e80e7bb
  diff_path_count: 965
  docs_touched: [README.md, corpus-ingest.md, desktop-app.md, glossary.md, homebrew-and-oracle.md, overview.md, persistence.md, release-pipeline.md, rules-engine.md, status.md, testing.md]
  stub_graduations: ["rule-system id cyberpunk-red: StubAdapter placeholder retired for CyberpunkAdapter (registry exception 0002 narrowed, docs/governance/wired-integration-stubs-registry.md)"]
  stub_regressions: []
  obsolete_removals: not counted (`git diff --stat 9d23977679..296e80e7bb -- docs/architecture` is the extent: 11 files, 267 insertions, 75 deletions)
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: "truth-up touched 11 doc(s) in 296e80e7bb (edited by hand before the script ran; Opus claims critic follows as part 2 of E7.9). The script's own lines said 'docs touched none, no architecture impact': its README index parser needs a leading '||' per row and matches 0 of the 14 linked rows, so it maps no path to any doc (the same defect SD-37 E7.7 recorded, retro correction 1791397812938-sd37-e7-7-7cf9ac). The script's two verification one-liners ran and passed; the README's two one-liners (which include the crates/ and data/ roots the script's cited-path grep omits) were also run and print nothing. Figures: command count 103 by the python parse of generate_handler (HEAD 103, origin/develop 96, tranche/17 b99c3d4b02 86); Cyberpunk figures are release-notes R-n figures, `python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.8_check.py --plant` -> RELEASE_NOTES_CHECK_OK 14 figures, 3 plants rejected."
