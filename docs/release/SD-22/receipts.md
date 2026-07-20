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

- cycle_id: 2026-07-19T14:00:00Z
  epic: 3
  criterion: apg_alchemist
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (criteria 6, 7, 8)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: blocked
  evidence_tier_after: complete (criteria 6-8; criterion 9's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: e2d7194
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "apg/class_alchemist_cycle_receipt.md"
  red_phase_evidence: "tests/sd22_apg_class_alchemist_resolves.rs asserted class_chassis_resolve(...) against RuleSetId::Apg/Crb; failed to compile (E0432/E0599: rules_tables::apg and RuleSetId::Apg did not exist yet) for the intended reason (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "added rules_tables/apg/{mod.rs,class_alchemist.rs} and RuleSetId::Apg; 4/4 new tests green (1 real-corpus-gated test also run and green under PCGEN_CORPUS_ROOT); full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_apg_class_alchemist_resolves: 4 passed, 1 ignored (real-corpus-gated, separately run and green); full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 3's next-eligible cycle is Cavalier (class 2 of 8), or apg/spell_list.rs + apg/equipment_tables.rs (criterion 9) for Alchemist's bomb/extract data; Epic 4 (ACG) and Epic 5 (Bestiary 1) remain blocked on their own parser-coverage gaps (ACG has no CLASS: allowlist entry yet; Bestiary 1's b1_races.lst uses unprefixed bare rows race_ability.rs cannot parse) — unchanged by this cycle, since this cycle only widened the class-chassis surface for Alchemist specifically"
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:11 (CLASS:Alchemist)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["apg/class_alchemist_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Alchemist record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB, good Fort+Reflex, poor Will, MAXLEVEL:20) -- same scope boundary as rules_tables/crb/class_tables.rs (named per-level features out of scope, formula-derived chassis only). This is the first Epic 3 cycle to land a commit; it builds on the operator-side parser-allowlist widening (commit d1b2f80) that unblocked E3.6-9."

- cycle_id: 2026-07-19T15:00:00Z
  epic: 3
  criterion: apg_cavalier
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (criteria 7, 8; second APG class)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open (class 2 of 8, not yet started)
  evidence_tier_after: complete (criteria 7-8 for Cavalier; criterion 9's spell/equipment resolution out of scope, no APG spell/equipment tables exist yet)
  branch_tip_before: 9c187a7
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "apg/class_cavalier_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_cavalier_record_from_apg_classes_lst added to tests/sd17_b1_martial_class.rs, failed (Cavalier out of MARTIAL_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_apg_class_cavalier_resolves.rs failed to compile (E0599: ApgClassId::Cavalier did not exist) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened MARTIAL_CLASS_NAMES by one name (Cavalier) in src/pcgen_import/lst_parser/class.rs; added rules_tables/apg/class_cavalier.rs and ApgClassId::Cavalier; lifted shared ClassTableRow into apg/mod.rs; 4/4 new acceptance tests green (1 real-corpus-gated test also run and green); widening test green; full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_apg_class_cavalier_resolves: 4 passed, 1 ignored (real-corpus-gated, separately run and green); sd17_b1_martial_class: 16 passed including the new widening test and the real-corpus core-rulebook test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 3's next-eligible cycle is Gunslinger (class 3 of 8), or apg/spell_list.rs + apg/equipment_tables.rs (criterion 9) for Alchemist's bomb/extract data; Epic 4 (ACG) and Epic 5 (Bestiary 1) remain blocked on their own parser-coverage gaps (ACG has no CLASS: allowlist entry yet; Bestiary 1's b1_races.lst uses unprefixed bare rows race_ability.rs cannot parse) — unchanged by this cycle"
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:42 (CLASS:Cavalier)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["apg/class_cavalier_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Cavalier record's BONUS:COMBAT/BONUS:SAVE formula tokens (full BAB, good Fortitude, poor Will+Reflex, MAXLEVEL:20) -- same scope boundary as class_alchemist.rs (named per-level features out of scope, formula-derived chassis only). Cavalier's good/poor save split differs from Alchemist's (Fort+Reflex good for Alchemist vs. only Fortitude good for Cavalier), confirmed against the real record rather than assumed from the class-archetype template."

- cycle_id: 2026-07-19T16:00:00Z
  epic: 3
  criterion: apg_inquisitor
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (criteria 7, 8; fourth class in the operator-pinned ordering)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open (class 3 of 8 in ordering was Gunslinger, found blocked; Inquisitor picked as next-eligible)
  evidence_tier_after: complete (criteria 7-8 for Inquisitor; criterion 9's spell/equipment resolution out of scope)
  branch_tip_before: 675ca65
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "apg/class_inquisitor_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_inquisitor_record_from_apg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Inquisitor out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_apg_class_inquisitor_resolves.rs failed to compile (E0599: ApgClassId::Inquisitor did not exist) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Inquisitor) in src/pcgen_import/lst_parser/spellcasting_class.rs; added rules_tables/apg/class_inquisitor.rs and ApgClassId::Inquisitor; 4/4 new acceptance tests green (1 real-corpus-gated test also run and green); widening test green; full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_apg_class_inquisitor_resolves: 4 passed, 1 ignored (real-corpus-gated, separately run and green); sd17_b_spellcasting_class: 4/4 ignored real-corpus tests green including the new widening test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "New Open Blockers entry this cycle: apg_classes.lst has no CLASS:Gunslinger or CLASS:Magus record anywhere -- both live in ultimate_combat/uc_classes.lst and ultimate_magic/um_classes.lst respectively, which decisions.md §1 explicitly excludes from SD-22 scope. corpus-source-inventory.md §1.1's 8-class APG roster is itself wrong for those 2 rows, not just its Content-shape prose. Operator decision needed: drop Gunslinger/Magus from Epic 3's class count (6 real APG classes total), or explicitly expand SD-22 scope to include Ultimate Combat/Ultimate Magic. Epic 3's next-eligible cycle in the meantime is Oracle (class 5 of 8, next after Magus in ordering), or apg/spell_list.rs + apg/equipment_tables.rs (criterion 9). Epic 4/5 remain blocked on their own separate parser-coverage gaps, unchanged."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:50 (CLASS:Inquisitor)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["apg/class_inquisitor_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Inquisitor record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB, good Fortitude+Will, poor Reflex, MAXLEVEL:20, SPELLSTAT:WIS MEMORIZE:NO) -- same scope boundary as class_alchemist.rs/class_cavalier.rs. Also discovered and logged: Gunslinger and Magus, next in the operator-pinned 8-class ordering, have no real APG .lst record at all -- they belong to Ultimate Combat / Ultimate Magic, books explicitly out of SD-22 scope per decisions.md §1. This is a genuine inventory-doc defect (the routing table itself, not just its illustrative prose), surfaced to the operator via progress.md's Open Blockers and a push notification."

- cycle_id: 2026-07-19T17:00:00Z
  epic: 3
  criterion: apg_oracle
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (criteria 7, 8; fourth class in the corrected 6-class ordering)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open (Gunslinger/Magus blocker resolved by operator commit 6923e54, narrowing the roster to 6 real classes; Oracle next-eligible)
  evidence_tier_after: complete (criteria 7-8 for Oracle; criterion 9's spell/equipment resolution out of scope, no APG spell/equipment tables exist yet)
  branch_tip_before: 6923e54
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "apg/class_oracle_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_oracle_record_from_apg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Oracle out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_apg_class_oracle_resolves.rs failed to compile (E0599: ApgClassId::Oracle did not exist) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Oracle) in src/pcgen_import/lst_parser/spellcasting_class.rs; added rules_tables/apg/class_oracle.rs and ApgClassId::Oracle; 5/5 new acceptance tests green (including the real-corpus-gated test); widening test green; full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_apg_class_oracle_resolves: 5/5 passed (--include-ignored); sd17_b_spellcasting_class: 5/5 ignored real-corpus tests green including the new widening test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 3's next-eligible cycle is Summoner (class 5 of 6), Witch (class 6 of 6), or apg/spell_list.rs + apg/equipment_tables.rs (criterion 9). Epic 4 (ACG) and Epic 5 (Bestiary 1) remain blocked on their own, separate parser-coverage gaps (ACG has no CLASS: allowlist entry yet; Bestiary 1's b1_races.lst uses unprefixed bare rows race_ability.rs cannot parse) — unchanged by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:107 (CLASS:Oracle)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["apg/class_oracle_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Oracle record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB, good Will only, poor Fortitude+Reflex, MAXLEVEL:20, SPELLSTAT:CHA MEMORIZE:NO) -- same scope boundary as class_alchemist.rs/class_cavalier.rs/class_inquisitor.rs. This cycle picked up the operator's own commit 6923e54 (landed since the Inquisitor cycle), which resolved the standing Gunslinger/Magus Open Blocker by narrowing APG's roster to the 6 real classes -- confirming the fix and continuing the per-class cycle sequence with Oracle."

- cycle_id: 2026-07-19T18:00:00Z
  epic: 3
  criterion: apg_summoner
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (criteria 7, 8; fifth class in the corrected 6-class ordering)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open (class 5 of 6, not yet started; Oracle just landed)
  evidence_tier_after: complete (criteria 7-8 for Summoner; criterion 9's spell/equipment resolution out of scope, no APG spell/equipment tables exist yet)
  branch_tip_before: aa9b924
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "apg/class_summoner_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_summoner_record_from_apg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Summoner out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_apg_class_summoner_resolves.rs failed to compile (E0599: ApgClassId::Summoner did not exist) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Summoner) in src/pcgen_import/lst_parser/spellcasting_class.rs; added rules_tables/apg/class_summoner.rs and ApgClassId::Summoner; 5/5 new acceptance tests green (including the real-corpus-gated test); widening test green; full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_apg_class_summoner_resolves: 5/5 passed (--include-ignored); sd17_b_spellcasting_class: 6/6 ignored real-corpus tests green including the new widening test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 3's next-eligible cycle is Witch (class 6 of 6, the last real APG class), or apg/spell_list.rs + apg/equipment_tables.rs (criterion 9). Epic 4 (ACG) and Epic 5 (Bestiary 1) remain blocked on their own, separate parser-coverage gaps (ACG has no CLASS: allowlist entry yet; Bestiary 1's b1_races.lst uses unprefixed bare rows race_ability.rs cannot parse) — unchanged by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:139 (CLASS:Summoner)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["apg/class_summoner_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Summoner record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB, good Will only, poor Fortitude+Reflex, MAXLEVEL:20, SPELLSTAT:CHA MEMORIZE:NO) -- identical good/poor save split to Oracle, same scope boundary as class_alchemist.rs/class_cavalier.rs/class_inquisitor.rs/class_oracle.rs. Summoner is a spontaneous arcane caster (TYPE:Base.PC.SpontaneousArcane.Spontaneous) rather than divine, but that distinction doesn't affect the parser's posture derivation or the chassis formulas."

- cycle_id: 2026-07-19T14:00:00Z
  epic: 3
  criterion: apg_oracle
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (this firing's independent attempt at Oracle, criteria 7-8)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open (from this firing's perspective at start; branch tip f933ecf)
  evidence_tier_after: "no change from this firing — concurrent stream (aa9b924, b160857) already landed Oracle + Summoner before this firing's push"
  branch_tip_before: f933ecf
  branch_tip_after: "n/a — commit discarded, never pushed; local branch reset to origin/tranche/5 (b160857)"
  merge_receipt_sha: "n/a — no commit landed from this firing"
  cycle_artifact_path: "n/a — this firing's Oracle receipt was discarded; see apg/class_oracle_cycle_receipt.md (written by the concurrent stream) for the landed cycle"
  red_phase_evidence: "completed locally (widening + acceptance RED for Oracle), then discarded along with the rest of this firing's commit"
  green_phase_evidence: "completed locally (full cargo test + clippy green), then discarded — see notes"
  cargo_test_summary: "this firing's own local run was green before the push conflict; not the landed state (superseded by the concurrent stream's own green run)"
  clippy_signal: "n/a — discarded work"
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Witch (class 6 of 6) is next-eligible per the concurrent stream's own Summoner receipt. Operator should confirm whether a second SD-22 loop stream is intentionally running concurrently with this hourly-firing routine -- if so, the file-touch-partition's '1 cycle at a time' default is being violated across streams, not just within one, and either the second stream should be stopped or the routine's cadence should be coordinated with it."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:107 (CLASS:Oracle) -- same record the concurrent stream used"
  rule_set_used: Apg
  kanban_card: "no card: no commit landed"
  progress_file_updated: "yes"
  artifacts_written: []
  notes: "git push origin tranche/5 was rejected (non-fast-forward); git fetch showed origin/tranche/5 had moved past this firing's f933ecf base to aa9b924 (Oracle) then b160857 (Summoner), landed by a different, concurrently running stream. This is the loop-instruction's own documented hard stop ('two live claude processes... touch the same per-epic module file'), discovered after the fact via git state rather than avoidable in advance. Discarded this firing's local commit (git reset --hard origin/tranche/5) rather than force-pushing or merging duplicate Oracle content; no shared work was lost since the commit was never pushed. Did not additionally attempt Witch this firing to avoid racing the same concurrent stream a second time within one cycle."

- cycle_id: 2026-07-19T19:00:00Z
  epic: 3
  criterion: apg_witch
  criterion_section: "§1.1 Epic 3 — APG content-source ingest (criteria 7, 8; sixth and last class in the corrected 6-class ordering)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open (class 6 of 6, not yet started; Summoner just landed)
  evidence_tier_after: complete (criteria 7-8 for Witch; criterion 9's spell/equipment resolution out of scope, no APG spell/equipment tables exist yet)
  branch_tip_before: 6f2a13e
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "apg/class_witch_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_witch_record_from_apg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Witch out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_apg_class_witch_resolves.rs failed to compile (E0599: ApgClassId::Witch did not exist) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Witch) in src/pcgen_import/lst_parser/spellcasting_class.rs; added rules_tables/apg/class_witch.rs and ApgClassId::Witch; 5/5 new acceptance tests green (including the real-corpus-gated test); widening test green (20/20 in sd17_b_spellcasting_class); full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_apg_class_witch_resolves: 5/5 passed (--include-ignored); sd17_b_spellcasting_class: 20/20 passed (--include-ignored), including the new witch widening test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "All six real APG classes now have chassis tables (Epic 3 criteria 7-8 complete for the full roster). Next-eligible: apg/spell_list.rs + apg/equipment_tables.rs (criterion 9), or Epic 4 (ACG)/Epic 5 (Bestiary 1) first cycles -- both remain blocked on their own, separate parser-coverage gaps (ACG has no CLASS: allowlist entry yet; Bestiary 1's b1_races.lst uses unprefixed bare rows race_ability.rs cannot parse) -- unchanged by this cycle. Epic 6 (DM Toolkit) can now consider itself unblocked on 'at least one book ingested' since APG chassis data exists, though criterion 9's spell/equipment gap may still limit what Epic 6 can consume."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:172 (CLASS:Witch)"
  rule_set_used: Apg
  kanban_card: "no card: hermes unavailable from cloud sandbox"
  progress_file_updated: "yes"
  artifacts_written: ["apg/class_witch_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Witch record's BONUS:COMBAT/BONUS:SAVE formula tokens (half BAB -- the first poor-BAB class landed in this roster -- good Will only, poor Fortitude+Reflex, MAXLEVEL:20, SPELLSTAT:INT with no MEMORIZE:NO/SPELLBOOK:YES token). Casting posture derives to Prepared via the parser's absent-signals default, the same shape as Cleric/Druid -- same scope boundary as class_alchemist.rs/class_cavalier.rs/class_inquisitor.rs/class_oracle.rs/class_summoner.rs."

- cycle_id: 2026-07-19T19:51:46Z
  epic: 3
  criterion: apg_spell_and_equipment_resolution
  criterion_section: "§1.2 Epic 3 — APG shared spell and equipment tables (criterion 9)"
  row_or_kind: ingest:apg_class
  evidence_tier_before: open (all six class chassis complete, criterion 9 not yet started -- no apg/spell_list.rs or apg/equipment_tables.rs existed)
  evidence_tier_after: complete (criterion 9 -- bootstrap/representative-sample coverage, not exhaustive; see next_required_uplift)
  branch_tip_before: e134bb4
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "apg/spell_list_cycle_receipt.md, apg/equipment_tables_cycle_receipt.md"
  red_phase_evidence: "tests/sd22_apg_spell_list_resolves.rs and tests/sd22_apg_equipment_resolves.rs both failed to compile (E0432: could not find spell_list/equipment_tables in apg) against the unchanged tree (see cycle_artifact_path:Red-phase evidence, both files)"
  green_phase_evidence: "added rules_tables/apg/spell_list.rs (4-entry bootstrap SPELL_LIST + spell_resolve) and rules_tables/apg/equipment_tables.rs (3-entry bootstrap EQUIPMENT_TABLE + equipment_resolve), registered both in apg/mod.rs; 7/7 and 6/6 new tests green (including both real-corpus-gated grounding tests, run with PCGEN_CORPUS_ROOT set); full cargo test suite green, 0 failed anywhere; clippy clean (see cycle_artifact_path:Green-phase evidence, both files)"
  cargo_test_summary: "sd22_apg_spell_list_resolves: 7/7 passed (--include-ignored); sd22_apg_equipment_resolves: 6/6 passed (--include-ignored); full cargo test --locked: 0 failed across every suite (grepped full output for FAILED/N-failed, none found); cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Criterion 9 now has real, non-fabricated bootstrap coverage (4 spells across 4 of 5 APG caster classes; 3 equipment items across the 3 apg_equip_*.lst files), mirroring crb/equipment_tables.rs's own 'one representative item per category, exhaustive coverage is later loop work' precedent -- not exhaustive. Documented gaps for a future cycle: Summoner has zero active (non-#-commented) spell records anywhere in apg_spells.lst's dedicated Summoner block; Alchemist bombs/discoveries and other named per-level features remain out of scope per the established chassis-only scope boundary (would need apg_abilities_class.lst in a dedicated ingest slice). Epic 4 (ACG) and Epic 5 (Bestiary 1) remain blocked on their own, separate parser-coverage gaps -- unaffected by this cycle. With criteria 6-9 all complete, Epic 3 (APG) is now fully closed out; next-eligible per Step 1 is Epic 4 or Epic 5's first cycle (both still blocked), or Epic 9/Epic 6 depending on how the operator resolves those blockers."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_spells.lst (Bomber's Eye L44, Burst Bonds L53, Borrow Fortune L277, Ill Omen L150) and apg_equip_general.lst/apg_equip_arms_armor.lst/apg_equip_magic_items.lst (Iron Spike, Arrow (Blunt), Knucklebone of Fickle Fortune)"
  rule_set_used: Apg
  kanban_card: "t_1d2c1dce (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["apg/spell_list_cycle_receipt.md", "apg/equipment_tables_cycle_receipt.md"]
  notes: "Deliberately did not add a fabricated 'apg:alchemist:bomb:acid' equipment row despite corpus-source-inventory.md §1.3's illustrative example naming it -- that file's own corrective banner marks such prose non-authoritative, and no Bomb/Acid Bomb record exists in any real apg_equip_*.lst file (bombs are a Su class feature computed by formula, not a purchasable item). Kept both new resolver functions (spell_resolve, equipment_resolve) self-contained inside rules_tables/apg/ rather than wiring into the existing global equipment_id_resolve/spell_id_resolve in src/rules_core/ -- those two functions already exist and already accept a RuleSetId parameter (satisfying criterion 6's literal wording) but are hard-wired to the CRB tables regardless of rule_set; widening their dispatch logic to branch on RuleSetId::Apg is a cross-cutting change outside this cycle's file-touch partition (rules_core/equipment_resolver.rs and spell_resolver.rs are not listed as SD-22-cycle-touchable files) and is left as a follow-on if a future cycle needs the two ingest paths unified."

- cycle_id: 2026-07-19T20:18:28Z
  epic: 4
  criterion: acg_arcanist
  criterion_section: "§2.1 Epic 4 — ACG content-source ingest (criteria 10-12; first real ACG class)"
  row_or_kind: ingest:acg_class
  evidence_tier_before: open (Epic 4 not yet started; corpus-source-inventory.md's row 1 named a non-existent Alchemist-ACG class)
  evidence_tier_after: complete (criteria 10-12 for Arcanist; criterion 13's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: 87e7ec3
  branch_tip_after: "<see commit landed this cycle, immediately following this receipt in git log>"
  merge_receipt_sha: "<same as branch_tip_after>"
  cycle_artifact_path: "acg/class_arcanist_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_arcanist_record_from_acg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Arcanist out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_acg_class_arcanist_resolves.rs failed to compile (E0432/E0599: rules_tables::acg module and RuleSetId::Acg did not exist) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Arcanist) in src/pcgen_import/lst_parser/spellcasting_class.rs; added rules_tables/acg/{mod.rs,class_arcanist.rs} and RuleSetId::Acg; 6/6 new acceptance tests green (including the real-corpus-gated grounding test); widening test green (21/21 in sd17_b_spellcasting_class); full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_acg_class_arcanist_resolves: 6/6 passed (--include-ignored); sd17_b_spellcasting_class: 21/21 passed (--include-ignored) including the new widening test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: ["Alchemist-ACG roster defect (corpus-source-inventory.md §2.1 row 1 names a class with no real CLASS:Alchemist record in acg_classes.lst -- same shape as the resolved Gunslinger/Magus blocker) corrected in-cycle: logged to progress.md Open blockers and proceeded directly to Arcanist, the first real ACG class, rather than fabricating an Alchemist-ACG chassis or stalling"]
  next_required_uplift: "Epic 4's next-eligible cycle is Bloodrager (class 2 of the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest), or a dedicated cycle for criterion 13's shared spell/equipment tables once more classes land. corpus-source-inventory.md §2.1, decisions.md, and epic-breakdown.md still need an operator/doc-correction pass to formally replace 'Alchemist (ACG-side)' with Slayer in the row list, mirroring commit 6923e54's APG roster fix -- not blocking further Epic 4 cycles. Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (b1_races.lst's unprefixed bare-row monster records) -- unaffected by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:11 (CLASS:Arcanist)"
  rule_set_used: Acg
  kanban_card: "t_81cf6382 (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["acg/class_arcanist_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Arcanist record's BONUS:COMBAT/BONUS:SAVE formula tokens (poor/half BAB -- same shape as APG's Witch -- good Will only, poor Fortitude+Reflex, MAXLEVEL:20, SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES -- spellbook posture, same shape as APG's Alchemist) -- same scope boundary as every APG class module (named per-level features out of scope, formula-derived chassis only). This is the first Epic 4 (ACG) cycle to land a commit; rules_tables/acg/ and RuleSetId::Acg are new this cycle, mirroring rules_tables/apg/'s established shape exactly."

- cycle_id: 2026-07-19T21:15:57Z
  epic: 4
  criterion: acg_bloodrager
  criterion_section: "§2.1 Epic 4 — ACG content-source ingest (criteria 10-12; second real ACG class)"
  row_or_kind: ingest:acg_class
  evidence_tier_before: open (Arcanist complete; Bloodrager not yet started)
  evidence_tier_after: complete (criteria 10-12 for Bloodrager; criterion 13's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: 3f8df8a
  branch_tip_after: 3413884
  merge_receipt_sha: 3413884
  cycle_artifact_path: "acg/class_bloodrager_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_bloodrager_record_from_acg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Bloodrager out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_acg_class_bloodrager_resolves.rs failed to compile (E0599: AcgClassId::Bloodrager did not exist, 5 call sites) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Bloodrager) in src/pcgen_import/lst_parser/spellcasting_class.rs; added rules_tables/acg/class_bloodrager.rs and AcgClassId::Bloodrager match arm; 7/7 new acceptance tests green (including the real-corpus-gated grounding test and a cross-class regression check that Arcanist still resolves); widening test green (22/22 in sd17_b_spellcasting_class); full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_acg_class_bloodrager_resolves: 7/7 passed (--include-ignored); sd17_b_spellcasting_class: 22/22 passed (--include-ignored) including the new widening test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 4's next-eligible cycle is Brawler (class 3 of the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest), or a dedicated cycle for criterion 13's shared spell/equipment tables once more classes land. corpus-source-inventory.md §2.1, decisions.md, and epic-breakdown.md still need an operator/doc-correction pass to formally replace 'Alchemist (ACG-side)' with Slayer in the row list, mirroring commit 6923e54's APG roster fix -- not blocking further Epic 4 cycles. Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (b1_races.lst's unprefixed bare-row monster records) -- unaffected by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:40 (CLASS:Bloodrager)"
  rule_set_used: Acg
  kanban_card: "t_5cc43e43 (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["acg/class_bloodrager_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Bloodrager record's BONUS:COMBAT/BONUS:SAVE formula tokens (full BAB -- no fractional divisor, unlike Arcanist's poor/half BAB -- good Fortitude, poor Reflex+Will, MAXLEVEL:20, SPELLSTAT:CHA MEMORIZE:NO -- spontaneous posture, same shape as Oracle/Summoner) -- same scope boundary as class_arcanist.rs (named per-level features out of scope, formula-derived chassis only)."

- cycle_id: 2026-07-19T22:17:13Z
  epic: 4
  criterion: acg_brawler
  criterion_section: "§2.1 Epic 4 — ACG content-source ingest (criteria 10-12; third real ACG class, class 3 of 10)"
  row_or_kind: ingest:acg_class
  evidence_tier_before: open (Arcanist + Bloodrager complete; Brawler not yet started)
  evidence_tier_after: complete (criteria 10-12 for Brawler; criterion 13's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: 143dea6
  branch_tip_after: 6ddfdd1
  merge_receipt_sha: 6ddfdd1
  cycle_artifact_path: "acg/class_brawler_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_brawler_record_from_acg_classes_lst added to tests/sd17_b1_martial_class.rs, failed (Brawler out of MARTIAL_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_acg_class_brawler_resolves.rs failed to compile (E0599: AcgClassId::Brawler did not exist, 5 call sites) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened MARTIAL_CLASS_NAMES by one name (Brawler) in src/pcgen_import/lst_parser/class.rs (not spellcasting_class.rs -- Brawler's real record carries no SPELLSTAT token, same non-caster posture as Cavalier); added rules_tables/acg/class_brawler.rs and AcgClassId::Brawler match arm; 7/7 new acceptance tests green (including the real-corpus-gated grounding test and a cross-class regression check that Arcanist+Bloodrager still resolve); widening test green (17/17 in sd17_b1_martial_class); full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_acg_class_brawler_resolves: 7/7 passed (--include-ignored); sd17_b1_martial_class: 17/17 passed (--include-ignored) including the new widening test; full cargo test --locked: 0 failed across every suite (408 test-result:ok blocks); cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 4's next-eligible cycle is Hunter (class 4 of the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest), or a dedicated cycle for criterion 13's shared spell/equipment tables once more classes land. corpus-source-inventory.md §2.1, decisions.md, and epic-breakdown.md still need an operator/doc-correction pass to formally replace 'Alchemist (ACG-side)' with Slayer in the row list, mirroring commit 6923e54's APG roster fix -- not blocking further Epic 4 cycles. Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (b1_races.lst's unprefixed bare-row monster records) -- unaffected by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:84 (CLASS:Brawler)"
  rule_set_used: Acg
  kanban_card: "t_41a3578f (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["acg/class_brawler_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Brawler record's BONUS:COMBAT/BONUS:SAVE formula tokens (full BAB -- no fractional divisor, same posture as Bloodrager -- good Fortitude+Reflex from one combined token, poor Will from a separate CL/3 token, MAXLEVEL:20, no SPELLSTAT token -- non-caster, same posture as Cavalier) -- this is the first ACG class to land in class.rs's MARTIAL_CLASS_NAMES rather than spellcasting_class.rs's SPELLCASTING_CLASS_NAMES (Arcanist and Bloodrager were both spellcasters); same scope boundary as class_arcanist.rs/class_bloodrager.rs (named per-level features out of scope, formula-derived chassis only)."

- cycle_id: 2026-07-19T23:16:42Z
  epic: 4
  criterion: acg_hunter
  criterion_section: "§2.1 Epic 4 — ACG content-source ingest (criteria 10-12; fourth real ACG class, class 4 of 10)"
  row_or_kind: ingest:acg_class
  evidence_tier_before: open (Arcanist + Bloodrager + Brawler complete; Hunter not yet started)
  evidence_tier_after: complete (criteria 10-12 for Hunter; criterion 13's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: 3e10397
  branch_tip_after: 4c8e6a4
  merge_receipt_sha: 4c8e6a4
  cycle_artifact_path: "acg/class_hunter_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_hunter_record_from_acg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Hunter out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_acg_class_hunter_resolves.rs failed to compile (E0599: AcgClassId::Hunter did not exist, 5 call sites) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Hunter) in src/pcgen_import/lst_parser/spellcasting_class.rs (not class.rs -- Hunter's real record carries SPELLSTAT:WIS MEMORIZE:NO, same spontaneous-caster posture as Bloodrager/Oracle/Summoner); added rules_tables/acg/class_hunter.rs and AcgClassId::Hunter match arm; 7/7 new acceptance tests green (including the real-corpus-gated grounding test and a cross-class regression check that Arcanist+Bloodrager+Brawler still resolve); widening test green (23/23 in sd17_b_spellcasting_class); full cargo test suite green, 0 failed; clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_acg_class_hunter_resolves: 7/7 passed (--include-ignored); sd17_b_spellcasting_class: 23/23 passed (--include-ignored) including the new widening test; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 4's next-eligible cycle is Investigator (class 5 of the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest), or a dedicated cycle for criterion 13's shared spell/equipment tables once more classes land. corpus-source-inventory.md §2.1, decisions.md, and epic-breakdown.md still need an operator/doc-correction pass to formally replace 'Alchemist (ACG-side)' with Slayer in the row list, mirroring commit 6923e54's APG roster fix -- not blocking further Epic 4 cycles. Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (b1_races.lst's unprefixed bare-row monster records) -- unaffected by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:108 (CLASS:Hunter)"
  rule_set_used: Acg
  kanban_card: "t_3e37745a (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["acg/class_hunter_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Hunter record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB -- same posture as APG's Alchemist/Inquisitor/Oracle/Summoner -- good Fortitude+Reflex from one combined token, poor Will, MAXLEVEL:20, SPELLSTAT:WIS MEMORIZE:NO -- spontaneous divine posture, same shape as Bloodrager/Oracle/Summoner) -- confirming Hunter belongs in spellcasting_class.rs's SPELLCASTING_CLASS_NAMES rather than class.rs's MARTIAL_CLASS_NAMES (which Brawler widened last cycle); same scope boundary as class_arcanist.rs/class_bloodrager.rs/class_brawler.rs (named per-level features out of scope, formula-derived chassis only)."

- cycle_id: 2026-07-20T00:16:35Z
  epic: 4
  criterion: acg_investigator
  criterion_section: "§2.1 Epic 4 — ACG content-source ingest (criteria 10-12; fifth real ACG class, class 5 of 10)"
  row_or_kind: ingest:acg_class
  evidence_tier_before: open (Arcanist + Bloodrager + Brawler + Hunter complete; Investigator not yet started)
  evidence_tier_after: complete (criteria 10-12 for Investigator; criterion 13's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: d032466
  branch_tip_after: 5f9a5bb
  merge_receipt_sha: 5f9a5bb
  cycle_artifact_path: "acg/class_investigator_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_investigator_record_from_acg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Investigator out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_acg_class_investigator_resolves.rs failed to compile (E0599: AcgClassId::Investigator did not exist, 5 call sites) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Investigator) in src/pcgen_import/lst_parser/spellcasting_class.rs (not class.rs -- Investigator's real record carries SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES, same spellbook-prepared posture as Alchemist/Arcanist); added rules_tables/acg/class_investigator.rs and AcgClassId::Investigator match arm; 7/7 new acceptance tests green (including the real-corpus-gated grounding test and a cross-class regression check that Arcanist+Bloodrager+Brawler+Hunter still resolve); widening test green (24/24 in sd17_b_spellcasting_class); full cargo test suite green, 0 failed (410 test-result:ok blocks); clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_acg_class_investigator_resolves: 7/7 passed (--include-ignored); sd17_b_spellcasting_class: 24/24 passed (--include-ignored) including the new widening test; full cargo test --locked: 0 failed across every suite (410 test-result:ok blocks); cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 4's next-eligible cycle is Shaman (class 6 of the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest), or a dedicated cycle for criterion 13's shared spell/equipment tables once more classes land. corpus-source-inventory.md §2.1, decisions.md, and epic-breakdown.md still need an operator/doc-correction pass to formally replace 'Alchemist (ACG-side)' with Slayer in the row list, mirroring commit 6923e54's APG roster fix -- not blocking further Epic 4 cycles. Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (b1_races.lst's unprefixed bare-row monster records) -- unaffected by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:168 (CLASS:Investigator)"
  rule_set_used: Acg
  kanban_card: "t_a80b480d (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["acg/class_investigator_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Investigator record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB -- same posture as ACG's Hunter and APG's Alchemist/Inquisitor/Oracle/Summoner -- poor Fortitude from its own single-save token, good Will+Reflex from one combined token (BASE.Will,BASE.Reflex -- reverse pairing from Brawler's/Hunter's BASE.Fortitude,BASE.Reflex), MAXLEVEL:20, SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES -- spellbook-prepared posture, same shape as Alchemist/Arcanist) -- confirming Investigator belongs in spellcasting_class.rs's SPELLCASTING_CLASS_NAMES rather than class.rs's MARTIAL_CLASS_NAMES; same scope boundary as class_arcanist.rs/class_bloodrager.rs/class_brawler.rs/class_hunter.rs (named per-level features out of scope, formula-derived chassis only)."

- cycle_id: 2026-07-20T01:17:01Z
  epic: 4
  criterion: acg_shaman
  criterion_section: "§2.1 Epic 4 — ACG content-source ingest (criteria 10-12; sixth real ACG class, class 6 of 10)"
  row_or_kind: ingest:acg_class
  evidence_tier_before: open (Arcanist + Bloodrager + Brawler + Hunter + Investigator complete; Shaman not yet started)
  evidence_tier_after: complete (criteria 10-12 for Shaman; criterion 13's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: 63e93c9
  branch_tip_after: 0d93e05
  merge_receipt_sha: 0d93e05
  cycle_artifact_path: "acg/class_shaman_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_shaman_record_from_acg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Shaman out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_acg_class_shaman_resolves.rs failed to compile (E0599: AcgClassId::Shaman did not exist, 5 call sites) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Shaman) in src/pcgen_import/lst_parser/spellcasting_class.rs (not class.rs -- Shaman's real record carries SPELLSTAT:WIS MEMORIZE:YES with no SPELLBOOK:YES and no MEMORIZE:NO, same standard-prepared posture as APG's Witch); added rules_tables/acg/class_shaman.rs and AcgClassId::Shaman match arm; 7/7 new acceptance tests green (including the real-corpus-gated grounding test and a cross-class regression check that Arcanist+Bloodrager+Brawler+Hunter+Investigator still resolve); widening test green (25/25 in sd17_b_spellcasting_class); full cargo test suite green, 0 failed (411 test-result:ok blocks); clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_acg_class_shaman_resolves: 7/7 passed (--include-ignored); sd17_b_spellcasting_class: 25/25 passed (--include-ignored) including the new widening test; full cargo test --locked: 0 failed across every suite (411 test-result:ok blocks); cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 4's next-eligible cycle is Skald (class 7 of the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest), or a dedicated cycle for criterion 13's shared spell/equipment tables once more classes land. corpus-source-inventory.md §2.1, decisions.md, and epic-breakdown.md still need an operator/doc-correction pass to formally replace 'Alchemist (ACG-side)' with Slayer in the row list, mirroring commit 6923e54's APG roster fix -- not blocking further Epic 4 cycles. Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (b1_races.lst's unprefixed bare-row monster records) -- unaffected by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:221 (CLASS:Shaman)"
  rule_set_used: Acg
  kanban_card: "t_05ab0c9b (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["acg/class_shaman_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Shaman record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB -- same posture as ACG's Hunter/Investigator and APG's Alchemist/Inquisitor/Oracle/Summoner -- good Will from its own single-save token, poor Fortitude+Reflex from one combined token (BASE.Fortitude,BASE.Reflex -- same pairing shape as Brawler's/Hunter's combined token, but poor instead of good), MAXLEVEL:20, SPELLSTAT:WIS MEMORIZE:YES with no SPELLBOOK:YES and no MEMORIZE:NO -- standard-prepared posture, same shape as APG's Witch) -- confirming Shaman belongs in spellcasting_class.rs's SPELLCASTING_CLASS_NAMES rather than class.rs's MARTIAL_CLASS_NAMES; same scope boundary as class_arcanist.rs/class_bloodrager.rs/class_brawler.rs/class_hunter.rs/class_investigator.rs (named per-level features out of scope, formula-derived chassis only)."

- cycle_id: 2026-07-20T02:20:00Z
  epic: 4
  criterion: acg_skald
  criterion_section: "§2.1 Epic 4 — ACG content-source ingest (criteria 10-12; seventh real ACG class, class 7 of 10)"
  row_or_kind: ingest:acg_class
  evidence_tier_before: open (Arcanist + Bloodrager + Brawler + Hunter + Investigator + Shaman complete; Skald not yet started)
  evidence_tier_after: complete (criteria 10-12 for Skald; criterion 13's spell/equipment resolution deferred to a later cycle)
  branch_tip_before: 1c5b590
  branch_tip_after: 694533e
  merge_receipt_sha: 694533e
  cycle_artifact_path: "acg/class_skald_cycle_receipt.md"
  red_phase_evidence: "Widening RED: parses_real_skald_record_from_acg_classes_lst added to tests/sd17_b_spellcasting_class.rs, failed (Skald out of SPELLCASTING_CLASS_NAMES scope, silently skipped). Acceptance RED: tests/sd22_acg_class_skald_resolves.rs failed to compile (E0599: AcgClassId::Skald did not exist, 5 call sites) (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "widened SPELLCASTING_CLASS_NAMES by one name (Skald) in src/pcgen_import/lst_parser/spellcasting_class.rs (not class.rs -- Skald's real record carries SPELLSTAT:CHA MEMORIZE:NO SPELLBOOK:YES; MEMORIZE:NO takes precedence over SPELLBOOK:YES in the parser's posture derivation, so Skald resolves as spontaneous, same posture as Bard, whose spell list Skald's own SPELLLIST:1|Bard token borrows from); added rules_tables/acg/class_skald.rs and AcgClassId::Skald match arm; 7/7 new acceptance tests green (including the real-corpus-gated grounding test and a cross-class regression check that Arcanist+Bloodrager+Brawler+Hunter+Investigator+Shaman still resolve); widening test green (26/26 in sd17_b_spellcasting_class); full cargo test suite green, 0 failed (412 test-result:ok blocks); clippy clean (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "sd22_acg_class_skald_resolves: 7/7 passed (--include-ignored); sd17_b_spellcasting_class: 26/26 passed (--include-ignored) including the new widening test; full cargo test --locked: 0 failed across every suite (412 test-result:ok blocks); cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 4's next-eligible cycle is Slayer (class 8 of the corrected 10-class roster: Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest), or a dedicated cycle for criterion 13's shared spell/equipment tables once more classes land. corpus-source-inventory.md §2.1, decisions.md, and epic-breakdown.md still need an operator/doc-correction pass to formally replace 'Alchemist (ACG-side)' with Slayer in the row list, mirroring commit 6923e54's APG roster fix -- not blocking further Epic 4 cycles. Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (b1_races.lst's unprefixed bare-row monster records) -- unaffected by this cycle."
  corpus_input_path: "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_classes.lst:274 (CLASS:Skald)"
  rule_set_used: Acg
  kanban_card: "t_31d72140 (codex-tranche-5, status=done)"
  progress_file_updated: "yes"
  artifacts_written: ["acg/class_skald_cycle_receipt.md"]
  notes: "BAB/save chassis read directly off the real CLASS:Skald record's BONUS:COMBAT/BONUS:SAVE formula tokens (three-quarter BAB -- same posture as ACG's Hunter/Investigator/Shaman and APG's Alchemist/Inquisitor/Oracle/Summoner -- good Will+Fortitude from one combined token (BASE.Will,BASE.Fortitude -- mirror-image pairing from Shaman's good-Will/poor-Fortitude+Reflex shape), poor Reflex from its own single-save token, MAXLEVEL:20, SPELLSTAT:CHA MEMORIZE:NO SPELLBOOK:YES -- MEMORIZE:NO wins over SPELLBOOK:YES in the parser's derivation order, so posture is spontaneous, same shape as Bard whose spell list Skald borrows via SPELLLIST:1|Bard) -- confirming Skald belongs in spellcasting_class.rs's SPELLCASTING_CLASS_NAMES rather than class.rs's MARTIAL_CLASS_NAMES; same scope boundary as class_arcanist.rs/class_bloodrager.rs/class_brawler.rs/class_hunter.rs/class_investigator.rs/class_shaman.rs (named per-level features out of scope, formula-derived chassis only)."

- cycle_id: 2026-07-20T01:50:52Z
  epic: 6
  criterion: dm_encounter
  criterion_section: "§4 Epic 6 — DM Toolkit (criterion 18, Encounter::new, DM Toolkit's first cycle)"
  row_or_kind: dm:encounter
  evidence_tier_before: open (Epic 6 not started; blocked on ≥1 book ingested -- Epic 3 APG fully complete satisfies this)
  evidence_tier_after: complete (criterion 18 only; criteria 19-21 remain open)
  branch_tip_before: 0244642
  branch_tip_after: "(this cycle's own commit, backfilled below)"
  merge_receipt_sha: "(this cycle's own commit, backfilled below)"
  cycle_artifact_path: "dm_toolkit/encounters_cycle_receipt.md"
  red_phase_evidence: "in-file #[cfg(test)] mod tests inside src/rules_core/encounters.rs itself (tests/sd22_dm_toolkit_deterministic.rs is reserved for criterion 20's own cycle per loop-instruction.md's file-touch partition); Encounter::new temporarily stubbed to a constant wrong value, 4/6 tests failed for the intended reason (see cycle_artifact_path:Red-phase evidence)"
  green_phase_evidence: "implemented CharacterSnapshot/MonsterRef/Difficulty/EncounterResult/Encounter::new grounded in the PF1 Core Rulebook's Table: Encounter Design + Table: CR Equivalencies + Table: Experience Point Awards (CR 1-10), verified against legacy.aonprd.com/corerulebook/gamemastering.html; 6/6 in-file tests green; full cargo test suite green, 0 failed anywhere; clippy clean after fixing one real new_ret_no_self finding with a documented #[allow] (see cycle_artifact_path:Green-phase evidence)"
  cargo_test_summary: "cargo test --locked --lib rules_core::encounters: 6/6 passed; full cargo test --locked: 0 failed across every suite; cargo clippy --locked --tests -- -D warnings clean"
  clippy_signal: clean
  cycle_timing_seconds: 0
  self_heals_applied: []
  next_required_uplift: "Epic 6's next-eligible cycle is criterion 19 (party_cr.rs, party_challenge_rating), per Step 2's ordering (Encounter::new first, party_challenge_rating second, deterministic tests third, happy-path integration fourth). Criterion 20's dedicated cycle should also reconcile the documented discrepancy against corpus-source-inventory.md §4.1 case 2's stated Hard expectation (this cycle's grounded math computes Deadly for that case -- see cycle_artifact_path)."
  corpus_input_path: "n/a -- not a PCGen .lst ingest cycle; grounded instead in legacy.aonprd.com/corerulebook/gamemastering.html (PF1 Core Rulebook, Gamemastering chapter), verified 2026-07-20"
  rule_set_used: n/a
  kanban_card: "(pending mint, backfilled below)"
  progress_file_updated: "yes"
  artifacts_written: ["dm_toolkit/encounters_cycle_receipt.md"]
  notes: "Ran in parallel with a sibling stream on Epic 4 (ACG); file-touch set (encounters.rs, mod.rs's one-line registration) disjoint from acg/ per loop-instruction.md. Found and documented a real discrepancy between corpus-source-inventory.md §4.1 case 2's stated Hard expectation and the verified-rulebook-grounded computation (Deadly) rather than force-fitting the formula to match the unverified fixture table, mirroring this bundle's Gunslinger/Magus and ACG-Alchemist roster-correction precedent."
