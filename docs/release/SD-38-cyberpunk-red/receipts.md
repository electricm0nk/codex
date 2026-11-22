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

- cycle_id: 2026-10-15T21:00:00Z
  row_or_kind: architecture:claims_critic
  bundle: SD-38
  model: opus
  branch_tip_before: 4c4ff18d6e
  docs_checked: [README.md, corpus-ingest.md, desktop-app.md, glossary.md, homebrew-and-oracle.md, overview.md, persistence.md, release-pipeline.md, rules-engine.md, status.md, testing.md]
  docs_corrected: [corpus-ingest.md, desktop-app.md, rules-engine.md, status.md, testing.md]
  claims_refuted: 11
  blockers_after_fixes: 0
  identifiers_checked: "145 of 145 backticked identifiers in part 1's added lines found in src/apps/crates/tests/scripts/.github (git grep -w -F -f)"
  cited_path_check: pass
  relative_link_check: pass
  receipt_note: "Opus claims critic (E7.9 part 2). Refuted and fixed: the cpr 'every table value from the corpus' claim (netrunning.rs holds NET Actions, deck slots and Program/Black ICE slot costs as code); status.md command arithmetic (86 + 6 + 4 + 7 = 103); WORK_INVENTORY/VAR_NAMES_DIR live in sheet_rule/mod.rs, not system_books.rs; two stale 103-row ui-smoke figures (127 now) and 'two' PF render-hash seeds (three); ingest bin count 48 -> 49; understated cpr_improve capability; Improve dialog host; reputation/critical refusals; LICENSE.json field path; R-n provenance in status.md header. Full list with refuting commands: artifacts/epic_7/E7.9_cycle_receipt.md part 2."

- cycle_id: 2026-10-15T20:46:07Z
  row_or_kind: graphify:update
  bundle: SD-38
  branch: c0929752c6421d433eddea9ecec615b84b37a73a
  integration_target: develop
  branch_tip: c0929752
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 0.2
  log_path: graphify-out/.truth-up-run-2026-10-15T20:46:07Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: graphify exited 1; operator to decide retry-vs-proceed (see log)

- cycle_id: 2026-10-15T20:47:22Z
  row_or_kind: graphify:update
  bundle: SD-38
  branch: 4d1af843d1051cab5251a16c3e073d859ee1e488
  integration_target: develop
  branch_tip: 4d1af843
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 1399.1
  log_path: graphify-out/.truth-up-run-2026-10-15T20:47:22Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: "node-count guard hit (exit 1): new graph 648327 nodes vs existing graph.json 648328 (net -1), graph.json NOT written, nothing force-applied. Indexed SHA 4d1af843d1051cab5251a16c3e073d859ee1e488 = origin/tranche-18, unfiltered git status --porcelain empty immediately before the run. Attempt 1 of this card (cycle 2026-10-15T20:46:07Z, exit 1, wall 0.2s) failed earlier for a different cause: the E7.10 worktree had no graph.json (graphify-out is gitignored and lives in the lead checkout); attempt 2 seeded the worktree graphify-out from /home/ubuntu/workspace/repos/codex/graphify-out (copy, lead checkout untouched). Operator ruling needed: accept the guard-hit as non-blocking (2026-07-20 policy) or authorize a full semantic rebuild."
