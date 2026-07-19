---
purpose: durable-receipt-ledger
sd: SD-22
branch: tranche/5
status: opened 2026-07-19
owner: Todd Hintzmann
scope: per-cycle post-mortem record (one block per /loop cycle)
durability: repo-resident — survives when kanban DB and ~/workspace/ are both unreachable
audience: operator + future-self auditing a cloud-run bundle
---

# SD-22 Receipts

## What this file is

This file is the repo-resident ledger of cycle-level receipts for SD-22. Every cycle
that runs under `/loop 60m /goal docs/release/SD-22/loop-instruction.md` (per `decisions.md §5`; `/batch` deferred)
appends exactly one block to this file before exiting, mirroring the kanban card
body schema defined in `loop-instruction.md` §"Step 10 — Mint the kanban card".

This is the **durability backbone** for cloud-run cycles. In a cloud environment:

- The kanban DB (`~/.hermes/profiles/<profile>/...`) may be unreachable, so the
  `hermes kanban create` call in Step 10 fails. The cycle continues; this file
  captures the receipt instead.
- The workspace progress file (`~/workspace/SD-22-content-source-ingest-and-dm-toolkit-progress.md`)
  lives outside the repo and is ephemeral across a cloud sandbox. The mirror is
  this file, plus the commit SHA on `tranche/5`.

If this file is missing for a cycle, the cycle **did not run** in a verifiable way —
regardless of whether the kanban card exists.

## What this file is NOT

- Not a substitute for the kanban board when the kanban board is reachable. The kanban
  card is the primary post-mortem surface; this file is the **fallback** written first.
- Not a redesign of `progress.md`. The workspace progress file remains the operator-facing
  status matrix; this file is the durable audit trail.
- Not freeform prose. Each block conforms to the schema below.

## Schema (one block per cycle)

Append the following block at the END of this file after each cycle. Do not edit
or delete prior blocks; if a cycle is rolled back or re-run, write a new block
referencing the prior block's `cycle_id`. Edit in place to keep the diff auditable.

```yaml
- cycle_id: <ISO-8601 timestamp, e.g. 2026-07-20T03:42:00Z>
  epic: <SD-22 epic number, e.g. 3>
  criterion: <criterion key, e.g. apg_alchemist or identifier_cleanup_wave_1>
  criterion_section: <scope-doc section reference, e.g. "§1.3 Epic 3 — APG content-source ingest">
  row_or_kind: <one of: ingest:apg_class | ingest:acg_class | ingest:beastiary1_subset | dm:encounter | dm:party_cr | identifier:rust_tauri | identifier:ts_function_or_class | version:patch_bump | version:build_label_format | version:closure_checklist | closure_readiness:eval | closure_readiness:self_heal | closure_readiness:dispatch>
  evidence_tier_before: <previous matrix row state>
  evidence_tier_after: <new matrix row state after this commit>
  branch_tip_before: <short SHA on tranche/5 before this cycle's push>
  branch_tip_after: <short SHA on tranche/5 after this cycle's push>
  merge_receipt_sha: <commit SHA on tranche/5 (== branch_tip_after when direct commit)>
  cycle_artifact_path: <path under docs/release/SD-22/artifacts/, e.g. cycles/2026-07-20T03-42-00Z-apg-alchemist.md>
  red_phase_evidence: <one-line summary or "see cycle_artifact_path:Red-phase evidence">
  green_phase_evidence: <one-line summary or "see cycle_artifact_path:Green-phase evidence">
  cargo_test_summary: <test summary string, e.g. "1/1 green; full suite green; clippy clean">
  clippy_signal: <clean | dirty>
  cycle_timing_seconds: <N>
  self_heals_applied: <list, empty [] if none>
  next_required_uplift: <recommendation for next iteration, or "none">
  corpus_input_path: <path to operator-supplied structured-data file if applicable, else "n/a">
  rule_set_used: <Apg | Acg | Bestiary1 | n/a>
  kanban_card: <card id, or "no card: <reason>" — e.g. "no card: board unreachable from cloud sandbox">
  progress_file_updated: <yes | no: <reason>>
  artifacts_written: <list of paths under docs/release/SD-22/artifacts/, empty [] if none>
  notes: <freeform one-line note, empty string if nothing>
```

## Cycle log

(One block per cycle appended below this line in YAML-frontmatter-free blockquote form.
Schema above is the canonical shape; cycles do NOT need to repeat the schema fields
that are already self-evident from the cycle_artifact_path.)

- cycle_id: 2026-07-19T04:00:00Z
  epic: 3
  criterion: apg_alchemist
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (Alchemist)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open
  evidence_tier_after: blocked
  branch_tip_before: cd9e88b
  branch_tip_after: cd9e88b
  merge_receipt_sha: "n/a — no commit landed"
  cycle_artifact_path: "n/a — blocked before RED phase; see progress.md ## Open blockers"
  red_phase_evidence: "not started"
  green_phase_evidence: "not started"
  cargo_test_summary: "not re-run; no production code touched (last known-green: 14/14)"
  clippy_signal: "n/a"
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "operator decision needed: supply a real corpus/reference source, narrow Epic 3/4/5 acceptance shape to formula-derivable data only (mirroring crb/class_tables.rs), or explicitly re-affirm memory-recalled content is acceptable outside this bundle's own self-referential docs"
  corpus_input_path: "n/a — not generated (fabrication-risk hard stop)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: []
  notes: "WebFetch to aonprd.com and d20pfsrd.com both 403'd; no verifiable source reachable; see progress.md Open blockers for full reasoning"

- cycle_id: 2026-07-19T05:02:04Z
  epic: 8
  criterion: three_version_fields
  criterion_section: "§1.8 Epic 8 — Build Version Numbering (criterion 27)"
  row_or_kind: version:patch_bump
  evidence_tier_before: open
  evidence_tier_after: complete
  branch_tip_before: 05a9ced
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "epic_8/three_version_fields_cycle_receipt.md"
  red_phase_evidence: "src/sd22/buildVersionTriple.test.ts asserted pkg.startsWith('0.5.'); failed against 0.4.94 for the intended reason (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "package.json/tauri.conf.json/Cargo.toml bumped to 0.5.95; sd22/buildVersionTriple.test.ts green; 46/46 JS test files green; cargo test 136+ tests green; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "npm test 46/46 green; cargo test --locked all suites green, 0 failed; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: ["fixed stale sibling regression in apps/desktop/src/sd21/buildVersionTriple.test.ts (tranche-4 anchor -> tranche-5, caused by this cycle's own version bump)"]
  next_required_uplift: "Epic 3/4/5 remain blocked on the fabrication-risk open blocker (unchanged this cycle); Epic 8 criterion 28 (build-label format) should be explicitly verified/marked complete by a future cycle rather than assumed"
  corpus_input_path: "n/a"
  rule_set_used: n/a
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["epic_8/three_version_fields_cycle_receipt.md"]
  notes: "Version bump is mechanically derivable (last committed build on this line was 94 per SD-21 commit 6ea6bfd; next monotonic build is 95; tranche moves 4->5 per decisions.md §2), not fabricated content -- distinct from the Epic 3/4/5 blocker."

- cycle_id: 2026-07-19T06:15:00Z
  epic: 8
  criterion: build_label_format
  criterion_section: "§1.8 Epic 8 — Build Version Numbering (criterion 28)"
  row_or_kind: version:build_label_format
  evidence_tier_before: open
  evidence_tier_after: complete
  branch_tip_before: 4b79f5c
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "epic_8/build_label_format_cycle_receipt.md"
  red_phase_evidence: "src/sd22/buildLabelFixtureFreshness.test.ts asserted the three partitioned fixture files carry 'Codex <package.json version>-test'; failed against the pre-bump 'Codex 0.4.94-test' literal for the intended reason (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "loadSd11TesterWorkbenchSurface.test.ts, createSd11WorkbenchStatus.test.ts, and makeSurface.ts re-anchored to 'Codex 0.5.95-test'; 4 sibling-regression consumers of makeSurface.ts fixed in the same commit; 47/47 JS test files green; cargo test 136+ tests green; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "npm test 47/47 green; cargo test --locked all suites green, 0 failed; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: ["fixed 4 sibling-regression test files that hard-coded makeSurface.ts's stale build-label literal (composeBugReport.test.ts, composeEnhancementRequest.test.ts, captureFeedbackEvidence.test.ts, buildSd15OperatorTriageDraft.test.ts)", "restored missing node_modules via npm install (absent at cycle start; all 46 JS test files failed for an environment reason, not a code reason)"]
  next_required_uplift: "Epic 3/4/5 remain blocked on the fabrication-risk open blocker (unchanged this cycle, re-verified: no corpus/ dir, no reachable SRD mirror); Epic 8 criterion 29 (release-closure-checklist.md) is next-eligible in Epic 8; Epic 6 remains transitively blocked pending ≥1 book ingested"
  corpus_input_path: "n/a"
  rule_set_used: n/a
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["epic_8/build_label_format_cycle_receipt.md"]
  notes: "Pure fixture re-sync to an already-committed version value (0.5.95 from criterion 27), not fabricated content. An initial draft of the RED test used an overly-broad regex that false-positived on an unrelated arbitrary-input fixture ('Codex 0.0.0-test'); narrowed to the specific known-stale literal before treating RED as valid."

- cycle_id: 2026-07-19T07:00:00Z
  epic: 8
  criterion: release_closure_checklist
  criterion_section: "§1.8 Epic 8 — Build Version Numbering (criterion 29)"
  row_or_kind: version:closure_checklist
  evidence_tier_before: open
  evidence_tier_after: complete
  branch_tip_before: e555f64
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "epic_8/release_closure_checklist_cycle_receipt.md"
  red_phase_evidence: "src/sd22/releaseClosureChecklistDoc.test.ts asserted docs/SD-22/release-closure-checklist.md exists and covers all four steps; failed because the doc didn't exist yet (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "docs/SD-22/release-closure-checklist.md added, mirroring SD-21's E5.27 doc re-anchored to tranche-5 / 0.5.95; 48/48 JS test files green; cargo test all suites green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "npm test 48/48 green; cargo test --locked all suites green, 0 failed; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 3/4/5 remain blocked on the fabrication-risk open blocker (unchanged this cycle); Epic 8 criterion 30 is a standing verification gate (not a one-shot artifact) closed out by Epic 9's eval, not a separate cycle; .github/workflows/publish-tester-release.yml's stamp line is stale at 0.4.-prefix (flagged as a candidate Epic 9 self-heal item, out of Epic 8's file-touch-partition scope this cycle)"
  corpus_input_path: "n/a"
  rule_set_used: n/a
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["epic_8/release_closure_checklist_cycle_receipt.md"]
  notes: "Pure process-documentation mirror of an already-established SD-21 precedent, not fabricated content. All four epic 8 criteria that are gated on file-touch-partition scope (27, 28, 29) are now complete; criterion 30 is a standing gate, not a discrete artifact."
