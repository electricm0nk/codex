---
canonical: true
bundle_id: SD-41
---

# SD-41 Receipts

This file holds the YAML receipts the closure scripts append: the architecture truth-up and its claims
critic (E5.3), and the graphify update (E5.4), each run as `workflow-instruction.md §11` describes. Every
graphify receipt records the indexed SHA, with the empty `git status --porcelain` output taken immediately
before the run.

- cycle_id: 2026-11-17T04:46:32Z
  row_or_kind: architecture:truth_up
  bundle: SD-41
  branch: be27ecc83dd9a9b1d38e357d59c873d50d0ee10c
  integration_target: develop
  branch_tip_before: be27ecc8
  branch_tip_after: be27ecc8
  diff_path_count: 4416
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-11-17T04:46:32Z-hand-mapped
  row_or_kind: architecture:truth_up
  bundle: SD-41
  branch: tranche-21
  integration_target: develop  # the local `develop` ref is 2b14020bc7 (stale, owned by the lead checkout); SD-41's delta is 83a8a4f414 (merge base with origin/develop) ..HEAD
  supersedes_note_on: 2026-11-17T04:46:32Z  # the script's own block above
  branch_tip_before: be27ecc83d  # includes origin/develop's seven SD-40 E5.4 docs taken byte-for-byte as the base (be27ecc83d), so the PR does not conflict in docs/architecture
  diff_path_count: 4416  # the script's count against the stale local develop; `git diff --name-only origin/develop...HEAD | awk 'END{print NR}'` -> 1942
  docs_touched: [desktop-app.md, glossary.md, persistence.md, release-pipeline.md, rules-engine.md, status.md, testing.md]
  stub_graduations: []
  stub_regressions: []
  removals: ["EquipmentPanel.tsx (deleted by E3.1) replaced by GearScreen.tsx in the command inventory", "'only write_campaign_drive_artifacts is called' (drive_list_campaigns is now called by the Campaign dialog)", "'four sidecar' counts (equipment_sets.json and campaigns.json added)", "0.20 stamp lines -> 0.21"]
  obsolete_removals: 38  # removed lines, not statements: git diff -U0 be27ecc83d -- docs/architecture | awk '/^-[^-]/' | awk 'END{print NR}' -> 38 (149 lines added)
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: the seven docs last verified at tranche-20 (116 commands, 75 boundary files, 0.20, no equipment sets)
  evidence_tier_after: verified at tranche-21 for the rows each touched doc's new Last verified line names; corpus-ingest.md, overview.md, homebrew-and-oracle.md, getting-started.md, pcg-import.md, rules-data-tables.md, conventions.md not re-read (E5.3 receipt, Does not cover)
  receipt_note: "Hand-mapped as in SD-39 E9.9 and SD-40 E5.4: the script's index parser (|| row pattern) reads 0 rows of the single-pipe README table, so it reports 'none'. Its two one-liners pass, run through the script (--dry-run --force on the edited tree)."

- cycle_id: 2026-11-17T-claims-critic
  row_or_kind: architecture:claims_critic
  bundle: SD-41
  branch: tranche-21
  branch_tip_before: 006f314abd
  docs_touched: [desktop-app.md, glossary.md]
  cited_path_check: pass
  relative_link_check: pass
  release_notes_changes: "Defects Fixed: Astral Armor is not priced as base + eqmods (0.0 gp, listed price-unknown); R-5 now carries its two counting commands; R-10 added"
  receipt_note: "Opus claims critic over every line the truth-up added (git diff -U0 be27ecc83d..006f314abd -- docs/architecture | awk '/^\\+[^+]/') and over release-notes.md; each claim checked against the code or a command. Refuted and fixed: (1) release notes said Astral Armor is priced as base plus eqmods; (2)+(3) the stack key in glossary.md and desktop-app.md omitted abilities and the override flag (character_hub.rs list_held_equipment_stacks_at_root `same`); (4) desktop-app.md said Only Show Affordable reads load_character_money (MarketDialog reads props.balance). Listed in artifacts/epic_5/E5.3_cycle_receipt.md 'Claims critic'."

- cycle_id: 2026-11-17T04:56:29Z
  row_or_kind: graphify:update
  bundle: SD-41
  branch: cfef4404df274ba0cc3229a1e58ce7b69498aded
  integration_target: develop
  branch_tip: cfef4404
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 0.2
  log_path: graphify-out/.truth-up-run-2026-11-17T04:56:29Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: "graphify cluster-only exited 1: error: no graph found at <worktree>/graphify-out/graph.json. graphify-out is gitignored and the only graph lives in the main checkout; this is not the node-count guard and no force flag was used. Same shape as the SD-40 E5.5 failure receipt; operator decides the re-index (open item since SD-37)."
