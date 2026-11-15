---
canonical: true
bundle_id: SD-40
---

# SD-40 Receipts

This file holds the YAML receipts the closure scripts append: the architecture truth-up and its claims
critic (E5.4), and the graphify update (E5.5), each run as `workflow-instruction.md §11` describes. Every
graphify receipt records the indexed SHA, with the empty `git status --porcelain` output taken immediately
before the run.

- cycle_id: 2026-11-15T00:50:22Z
  row_or_kind: architecture:truth_up
  bundle: SD-40
  branch: tranche-20
  integration_target: develop
  branch_tip_before: tranche-
  branch_tip_after: tranche-
  diff_path_count: 2612
  docs_touched: []
  stub_graduations: []
  stub_regressions: []
  obsolete_removals: 0
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: no architecture impact — diff is outside architecture scope

- cycle_id: 2026-11-15T00:50:22Z-hand-mapped
  row_or_kind: architecture:truth_up
  bundle: SD-40
  branch: tranche-20
  integration_target: develop  # local develop = 2b14020bc7, before SD-39 and SD-40; the edits cover SD-40's delta over SD-39's tip cd873a6d0d
  supersedes_note_on: 2026-11-15T00:50:22Z  # the script's own block above; it ran with --branch tranche-20, so its tip fields read "tranche-"
  branch_tip_before: 2fd74a5c15
  branch_tip_after: ce2c610c79  # the edit commits 379ab09b16 + ce2c610c79; the critic's fixes land in the E5.4 receipt commit
  diff_path_count: 2612  # the script's count, develop...HEAD
  docs_touched: [conventions.md, desktop-app.md, glossary.md, persistence.md, release-pipeline.md, status.md, testing.md]
  stub_graduations: []
  stub_regressions: []
  removals: ["hit-point tracking (adjust_character_hp, load_character_durability, hp.json writer, DurabilityPanel) removed from desktop-app.md, status.md, persistence.md", "three stacking print option boxes replaced by the eight section checkboxes"]
  obsolete_removals: 43  # removed lines, not statements: git diff -U0 2fd74a5c15..ce2c610c79 -- docs/architecture | awk '/^-[^-]/' | awk 'END{print NR}' -> 43 (git diff --stat: 126 insertions, 50 deletions, 7 files)
  cited_path_check: pass
  relative_link_check: pass
  evidence_tier_before: doc set last verified at tranche-19 (118 commands, three print option boxes, HP tracking, 0.19)
  evidence_tier_after: verified at tranche-20 for the rows named in each touched doc's new Last verified line; rules-engine.md, corpus-ingest.md, overview.md, update-and-feedback.md not re-read (see the E5.4 receipt's Does not cover)
  receipt_note: "Hand-mapped like SD-39 E9.9: the script's index parser (|| row pattern) reads 0 rows of the single-pipe README table, so it reports 'none' (retro correction logged). Its two one-liners pass, run through the script."

- cycle_id: 2026-11-15T-claims-critic
  row_or_kind: architecture:claims_critic
  bundle: SD-40
  branch: tranche-20
  branch_tip_before: ce2c610c79
  docs_touched: [desktop-app.md]
  cited_path_check: pass  # was MISSING tests/packaged_resources.rs at 379ab09b16; fixed in ce2c610c79
  relative_link_check: pass
  release_notes_changes: none after the critic  # two draft errors were fixed before the notes commit 2fd74a5c15 (probe population 37 -> 39)
  receipt_note: "Opus claims critic over every line the truth-up added and over release-notes.md; each claim checked against a command or the code. Refuted and fixed: the inventory was missing 3 registered commands; a cited path was not repo-rooted; a glossary line claimed print presets had shipped (they never did). Listed in artifacts/epic_5/E5.4_cycle_receipt.md 'Claims critic'."
