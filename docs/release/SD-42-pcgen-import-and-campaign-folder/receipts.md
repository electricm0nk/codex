---
canonical: true
bundle_id: SD-42
---

# SD-42 Receipts

This file holds the YAML receipts the closure scripts append: the architecture truth-up and its claims
critic (E5.3), and the graphify update (E5.4), each run as `workflow-instruction.md §11` describes. Every
graphify receipt records the indexed SHA, with the empty `git status --porcelain` output taken immediately
before the run.


- cycle_id: 2026-11-20T22:44:06Z
  row_or_kind: architecture:truth_up
  bundle: SD-42
  branch: fc465cddd16211cddadd0cf8b264e6e8c2926cf2
  integration_target: develop
  branch_tip_before: fc465cdd
  branch_tip_after: fc465cdd
  diff_path_count: 3163
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: fail
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-11-20T22:44:06Z-hand-mapped
  row_or_kind: architecture:truth_up
  bundle: SD-42
  branch: tranche-22
  integration_target: develop  # local `develop` = d6890a6c73 = merge-base with origin/develop (origin/develop is 50d7291041, site/ only beyond it)
  supersedes_note_on: 2026-11-20T22:44:06Z  # the script's own block above: it mapped 0 docs and its cited-path check failed on campaign_drive.rs and campaignModel.ts, both deleted by SD-42
  branch_tip_before: fc465cddd1
  diff_path_count: 3163  # git diff --name-only origin/develop...HEAD | awk 'END{print NR}' (before this commit)
  docs_touched: [README.md, conventions.md, desktop-app.md, glossary.md, overview.md, persistence.md, release-pipeline.md, rules-engine.md, status.md, testing.md, update-and-feedback.md]
  stub_graduations: []
  stub_regressions: []
  removals: ["campaign_drive.rs / drive_* commands / write_campaign_drive_artifacts (deleted by E2.3) replaced by the campaign_folder.rs command groups", "campaignModel.ts localStorage source of truth and campaignManagerAccessGate.ts (deleted) replaced by the campaign folder", "GoogleDrivePanel/googleDrive.ts and the 'google-drive' Settings tab (deleted)", "status.md drive_* gap row (commands removed)", "CampaignStore .config layout as the live campaign layout (now migration-only)", "128 commands -> 158; 0.21 stamp lines -> 0.22; SD-41 floors -> SD-42 E5.1 floors; ui-smoke 203 -> 223; boundary 108 -> 120 (with tests)"]
  obsolete_removals: 172  # removed lines, not statements: git diff -U0 -- docs/architecture | awk '/^-[^-]/' | awk 'END{print NR}' before commit (207 lines added)
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: the touched docs last verified at tranche-21 (128 commands, a Drive-folder mirror, localStorage campaigns, 0.21, no glibc gate)
  evidence_tier_after: verified at tranche-22 for the rows each touched doc's new Last verified line names; corpus-ingest.md, getting-started.md, homebrew-and-oracle.md, pcg-import.md (updated by E4 cards), rules-data-tables.md not re-read (E5.3 receipt, Does not cover)
  receipt_note: "Hand-mapped as in SD-39 E9.9, SD-40 E5.4 and SD-41 E5.3: the script's index parser reads 0 rows of the single-pipe README table. Edits mapped from git diff --name-status origin/develop...HEAD over apps/desktop/src, apps/desktop/src-tauri, src, crates/codex-ingest/src, .github, tools, scripts and from the code. Its two one-liners pass on the edited tree (--dry-run --force, rc 0)."

- cycle_id: 2026-11-20T-claims-critic
  row_or_kind: architecture:claims_critic
  bundle: SD-42
  branch: tranche-22
  branch_tip_before: cc6980eefc
  docs_touched: [desktop-app.md, persistence.md]
  cited_path_check: pass
  relative_link_check: pass
  release_notes_changes: "Known-sync-tool examples (Syncthing, cloud, git) removed: nothing in the code names them"
  receipt_note: "Opus claims critic over every line the truth-up added (git diff -U0 fc465cddd1..cc6980eefc -- docs/architecture | awk '/^\\+[^+]/', 207 lines) and over release-notes.md; each claim checked against the code or a command. 15 refuted and fixed, listed in artifacts/epic_5/E5.3_cycle_receipt.md 'Claims critic'."

- cycle_id: 2026-11-20T22:54:49Z
  row_or_kind: graphify:update
  bundle: SD-42
  branch: 022faed18e9754f24d2e696062ed9fecf5b40268
  integration_target: develop
  branch_tip: 022faed1
  graphify_exit_code: 1
  outcome: failed
  wall_clock_seconds: 0.2
  log_path: graphify-out/.truth-up-run-2026-11-20T22:54:49Z.log
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: graphify exited 1; operator to decide retry-vs-proceed (see log)
