# SD-26 — Release Notes (0.5.99)

> **Generated at Epic 6 (Criterion 6.3).** Per template REQUIRED_NOTES_SECTIONS: Summary, User-Visible Changes, Defects Fixed, Operational Notes, Verification Evidence, Known Issues, Update Eligibility.

## Summary

**SD-26 — Ingest Strategy and Rule-System Plumbing** is a four-load delivery bundle that builds the oracle-validation comparator (Epic 2), generates durable JSON caches for four in-scope PF1 books (CRB, APG, ACG, Bestiary 1; Epic 3), registers 21 future-state PF1 books as operator-granted stubs in the Stubs Registry (Epic 4), conducts a doctrine-cost audit that measures per-class compilation time and identifies over-spent gates for future optimization (Epic 5), and closes the cycle with architecture refreshes and governance audits (Epic 6). The bundle is purely backend and data-plumbing work; the oracle-validation harness is read-only for SD-26 (its oracle-checked upgrades are gated on fixing CG-03, a real pilot-compute bug discovered during end-to-end verification).

**Scope:** 38 declarative + dynamic criteria across 6 epics (E1 governance, E2–E5 structural work, E6 closure epilogue). Four parallel-eligible workstreams in E3 (per-book JSON cache) and E4 (21 concurrent book-stub registrations).

**Verification:** `cargo test --workspace --locked` clean (4124 passed, 0 failed, 468 test binaries). All 38 criteria complete with verdicts verified against three independent sources (status matrix, cycle receipts on disk, kanban board done-receipts). 7 live `## DISCOVERED` entries; 1 newly discovered open blocker (CG-03 — ability-modifier bug in `pilot_compute.rs`, blocks oracle-checked claim) documented and forward-tracked. 4 pre-existing kanban/receipt paper-trail gaps found during closure scan (all non-blocking, documented for operator remediation).

**Version:** 0.5.99 (stamped at criterion 6.4).

## User-Visible Changes

SD-26 ships **no user-facing UI surface** — the entire bundle is backend, data, and plumbing work. Changes are strictly operational and internal-system:

### Oracle-Validation Comparator (Epic 2)

- **New `src/oracle_validation/` modules:** `comparator.rs` (dimension-by-dimension parity checking), `normalization.rs` (trim + integer-coercion rule engine), `parity_report.rs` (markdown report generation), `pcgen_runner.rs` (Rust bindings to PCGen shell invocation + output normalization). These are the missing second half of the oracle-validation harness (ED-06's PCGen runner scaffolding provided the first half). The harness is read-only for end users; comparison and reporting happen at developer/operator verification time, not at runtime.
- **Pilot-case end-to-end verification:** The pipeline runs successfully end-to-end against the PF1 CRB Human Fighter level-1 pilot character, producing a real, multi-dimension parity report. **Current claim status:** `not_yet_grounded` (CG-03 blocker, below). 7 of 9 selected parity dimensions now match; 2 dimensions (skill modifiers for Climb and Swim) mismatch due to the ability-modifier bug.

### JSON Cache Build (Epic 3)

- **New `data/corpus/` directory structure:** Four books' worth of durable JSON caches (3,326 CRB records + 641 APG records + 423 ACG records + 45 Bestiary-1 records = 4,435 total). Records carry discriminated-union `source` provenance metadata (LST token, inherited copy, corrected ingest bug, web second-source, same-book fallback). `data/corpus/core_rulebook/`, `/advanced_players_guide/`, `/advanced_class_guide/`, `/beastiary/` are fully populated.
- **Corpus-ingest-diagnostic integration:** The corpus_ingest_diagnostic command (SD-25 Epic 5 prototype) now reads live record counts and book-specific completeness from these JSON caches instead of running slow git-log queries. Performance: ~100ms per book vs. ~2s per book with the prior shell-out approach.

### Book Stub Manifest (Epic 4)

- **21 PF1 books registered in the Stubs Registry** (`docs/governance/wired-integration-stubs-registry.md`). Future-state sourcebooks (Advanced Race Guide through Ultimate Wilderness) are now visible in the system's canonical book-inventory as "stubbed, planned for SD-27+ implementation". Stubs carry `planned_resolution_bundle: "SD-27+ (unscheduled)"` and a citation to the operator's "in-scope books no stubs, future-state books knowingly stub" doctrine. No new UI surface; this is a governance + inventory transparency change.

### Doctrine-Cost Audit (Epic 5)

- **Per-class compilation floor measurement:** Pre-cut doctrine-cost gate times (~40min per SD-22's audit, Alchemist example) reduced to a hard real-world floor of ~6.3 minutes. Identifies which per-class gates are over-provisioned and ripe for future optimization cycles. This is an operational metric, not a user-visible change; reported for infrastructure-planning purposes.

## Defects Fixed

No defects were fixed in SD-26. The bundle is closure-ready for shipping; the only new defect found is **CG-03** (ability-modifier bug), which is documented as an open blocker below.

| Defect | Status | Evidence |
|---|---|---|
| CG-03: `pilot_compute::compute_ability_modifiers` does not apply Human racial ability bonus | **newly discovered, not fixed** | `tests/sd26_pilot_case_verification.rs::full_pipeline_runs_end_to_end_and_finds_two_genuine_skill_mismatches` passes with expected mismatch on `skill.selected_modifier.{climb,swim}` (PCGen 6, Codex 5). Source code audit: `src/rules_core/pilot_compute.rs:4743-4767` confirmed — `compute_ability_modifiers` derives modifiers from raw chosen ability score with no racial-bonus application step. Blocks oracle-checked upgrade until fixed. |

All defects verified via RED → GREEN test-driven approach with dual-audit (identifier discipline + wired-integration four-check) per bundle protocol.

## Operational Notes

### Architecture and Governance

- **Oracle-validation harness is structurally complete.** All components (comparator, normalization, parity-report, PCGen runner) are in place and tested. The harness is verification-only (not shipped to end users); it is a developer/operator tool for assessing rule-system parity against PCGen's reference engine. End-to-end smoke test runs successfully and produces real, informative comparison reports.
- **Durable JSON corpus caches land as `data/corpus/`** with Shape B discriminated-union `source` provenance metadata (per `decisions.md §11`), replacing the one-off in-memory corpus state. Enables reproducible, timestamped, web-sourced content audits and provides a foundation for future multi-rule-system corpus loading.
- **Book stub manifests are now visible as governance entries.** All 21 future-state PF1 sourcebooks carry registry entries in `docs/governance/wired-integration-stubs-registry.md` with transparent, operator-verbatim justifications and deferral dates. This replaces ad-hoc "someday" planning with an explicit, searchable forward-planning registry.
- **Living architecture docs refreshed** (E6.2 architecture-closure pipeline): `docs/architecture/{overview,status,rules-data-tables,homebrew-and-oracle}.md` updated to describe the post-SD-26 system, including the new oracle-validation comparator/normalization/parity-report/PCGen-runner stack (E2), JSON corpus caches (E3), book stub registry (E4), and doctrine-cost audit methodology (E5). Graphify cluster re-run green (12,243 nodes, 29,197 edges, 632 communities, backup 2026-07-23/). All 11 architecture files refreshed, README `## Links` verification clean (0 missing paths, 0 broken links).

### Testing and Verification

- **Regression test suite:** `cargo test --workspace --locked` clean at release time. All test binaries report `test result: ok` with 4124 passed, 0 failed, across 468 test binaries.
- **End-to-end oracle-validation smoke test:** `tests/sd26_pilot_case_verification.rs` (2/2 pass) runs the real PCGen engine, normalizes its output, and compares it against Codex's pilot character computation. Produces real, parity-dimension-aware comparison results. Currently 7/9 dimensions matching; the 2 mismatches (Climb/Swim skill modifiers) trace to CG-03 (ability-modifier bug).
- **Per-book JSON cache tests:** `tests/sd26_cache_{core_rulebook,apg_json_cache,acg_json_cache,beastiary_json_cache}.rs` all pass (6+6+6+6 = 24 tests total), verifying cache structure, record counts, and discriminated-union `source.kind` attribution.
- **Dual-audit clean:** Identifier-tag leak audit (`OK_NO_BUNDLE_TAGS`), wired-integration four-check audit (`OK_NO_TOKENS`).

### Deployment Eligibility

This bundle is **closure-ready** for merge to `develop` after criterion 6.5 completes. No infrastructure or runtime changes; backward-compatible with existing PF1 character data. The oracle-validation harness is read-only and does not affect user-facing behavior. New `data/corpus/` caches are optional/informational (used by `corpus_ingest_diagnostic.rs` for faster book-status queries, not by any shipped production path). Book stub registrations are governance/inventory transparency only, with no runtime impact.

## Verification Evidence

### Criterion Completion and Cross-Verification

All 38 declarative criteria + dynamically-spawned book-stub cycles (4.2–4.22) complete and cross-verified against three independent sources:

1. **Status matrix** (`progress.md` lines 9–47): 38 criteria listed with state, cycle ID, commit SHA, and notes.
2. **Receipts on disk** (`artifacts/epic_*/` subdirectories): One receipt file per criterion, containing acceptance criterion, RED → GREEN evidence, acceptance verdict, and discovery forwards.
3. **Kanban board** (`codex-tranche-5`): Each complete criterion has a corresponding done-receipt card (with noted exceptions: 4 small paper-trail gaps documented in closure-readiness-report §3, all non-blocking).

**Three-way cross-check result:** 38/38 criteria accounted for; 0 criteria missing substance-truth where `progress.md` claims `complete`. Four pre-existing kanban/receipt paper-trail gaps found during closure scan (criteria 1.1, 2.1, 4.6, 5.1 — documented in artifacts/epic_6/closure-readiness-report.md §3, no functional impact).

### Dual-Audit Results (Per-Criterion)

Every cycle ran the identifier-discipline + wired-integration four-check audit per `loop-instruction.md §6`. Final results:
- **Identifier audit:** 0 bundle-tag leaks found across all 38 criteria combined. Result: `OK_NO_BUNDLE_TAGS`.
- **Wired-integration four-check:**
  - `no_zero_tolerance_forbidden_tokens`: 0 hits
  - `no_would_strings`: 0 hits
  - `no_noop_handlers`: 0 hits
  - `no_mock_leaks`: 0 hits
  - Result: `OK_NO_TOKENS`.

### Live Test Run Summary

**`cargo test --workspace --locked` results (verified live at E6.1 closure scan):**
- Total test binaries: 468
- Passed: 4124 tests
- Failed: 0
- Exit code: 0

## Known Issues

### 7 Live `## DISCOVERED` Entries

Per the bundle protocol, the `## DISCOVERED` queue has been triaged to 7 live entries (all genuine, none trivial, all forward-tracked):

1. **Criterion 4.1 — `decisions.md §10`'s `planned_resolution_bundle` default conflicts with cycle-brief instruction.** `decisions.md §10` pins the field to the literal `"SD-27"` (operator-pinned default); this cycle's brief instructed `"SD-27+ (unscheduled)"` instead (citing `risks-and-open-questions.md §5`'s open-ended deferral posture). Followed the brief (more specific/recent). **All 21 landed book_stub entries carry** `"SD-27+ (unscheduled)"` consistently. Operator must resolve: either correct `decisions.md §10` and `risks-and-open-questions.md §4 Q2` to match the landed value, or confirm `"SD-27"` is intended and correct all 21 JSON files the other way.

2. **Criterion 3.3 (informational) — ACG's real per-field completion ceiling, independently measured for the first time.** ACG was already SD-24-complete and was not covered by SD-25's corpus-intake pass. Measured: classes 10/10 with full 20-level chassis, spell description+full_text 144/144 (100%), equipment description 264/269 (98.1%, sourced from `SPROP:` tokens). ACG's higher ceiling (100% spell text) reflects SD-24's original more thorough ingest, not a corpus-complexity difference. Informational only; not blocking.

3. **Criterion 2.5 (resolved, cross-cycle followup) — Real PCGen-native `.pcg` pilot character file now exists and is wired into the pipeline.** A real precursor file was completed to the exact pilot spec (Dodge + Weapon Focus feats, rank-1 Climb/Intimidate/Swim skills, Chain Shirt + Longsword, no shield) and renamed to match the pilot's `case_id`. `tests/sd26_pilot_case_verification.rs` points at this file. Supersedes the prior blocker "no real `.pcg` file exists"; resolved by followup cycle.

4. **Criterion 2.5 (newly discovered, not fixed) — CG-03: `pilot_compute::compute_ability_modifiers` never applies the chosen Human `+2 Strength` racial ability bonus.** Running the oracle-validation harness end-to-end against the now-real pilot `.pcg` revealed a genuine `skill.selected_modifier.{climb,swim}` parity mismatch (PCGen 6, Codex 5). Root cause: `compute_ability_modifiers` derives modifiers from raw chosen ability score (16 → +3) with no racial-bonus application step, so the effective (bonus-applied) score of 18 (→ +4) is never computed. Blocks `oracle_checked` claim until fixed in `src/rules_core/pilot_compute.rs`. Not self-healable inline (wrong module ownership; fix belongs to `rules_core`, not `oracle_validation`). Forward to a `rules_core` fix cycle.

5. **Criterion 3.1 — `equipmods.rs`'s 314/658 duplicate-`key` shell-record defect remains unfixed at source.** Confirmed exactly: 658 raw entries, 344 truly-unique keys, 314 near-empty shells (`name == key`, `cost_gp: None`). This cycle de-duplicates at cache-write time only (fixing `equipmods.rs` would break several SD-24 tests hard-coding `658`/`2977` totals). `data/corpus/core_rulebook/equipment/equipmods/` correctly has 344 records (one per unique key). A future data-hygiene cycle should fix `equipmods.rs` at source and correct dependent totals.

6. **Criterion 3.4 (resolved) — `beastiary1::mod.rs`'s `MonsterId` enum had no public `ALL`/count constant.** `corpus_ingest_diagnostic.rs` had to hand-maintain a duplicate 41-entry list to work around this. This cycle added the real `MonsterId::ALL` constant to `beastiary1/mod.rs` and removed the duplicate list. Also confirmed Bestiary 1's real equipment record count is 4 (not ~7 as a prior estimate assumed). Resolved by this cycle.

7. **Criterion 4.22 (resolved) — Criterion-to-book count mismatch in Epic 4.** After landing `ultimate_magic` (4.21, #0022), 1 book remained (`ultimate_wilderness`) but 2 criterion numbers were open (`4.12`, `4.22`). Per the 4.22 cycle's brief, `ultimate_wilderness` was assigned to criterion 4.22 (#0023), confirming `4.12` is the orphaned no-op criterion from the original 22-book miscount. All 21 future-state books now registered (#0003–#0023); `progress.md` records `4.12` as an explicit no-op row. Resolved by criterion 4.22.

### Open Blocker

**CG-03 — `pilot_compute::compute_ability_modifiers` does not apply racial ability-score bonuses.** The `compute_ability_modifiers()` function in `src/rules_core/pilot_compute.rs:4743-4767` derives each ability modifier directly from the raw chosen score, never folding in the chosen Human `+2 Strength` racial bonus. This causes a real 1-point mismatch on Climb and Swim skill modifiers when compared against PCGen's engine (PCGen: +4 STR modifier → +6 Climb; Codex: +3 STR modifier → +5 Climb). `tests/sd26_pilot_case_verification.rs::full_pipeline_runs_end_to_end_and_finds_two_genuine_skill_mismatches` documents and verifies this mismatch; the test correctly keeps `current_claim_status` at `not_yet_grounded` rather than forcing an `oracle_checked` upgrade. Blocks the oracle-checked claim until a `rules_core` cycle fixes the ability-modifier application and re-runs verification. **Not a regression** — this is the real pilot-compute bug CG-03 was originally created to surface; SD-26's oracle-validation harness simply made it visible for the first time.

## Update Eligibility

- **Backward compatibility:** Full. PF1 character behavior is identical to pre-release (oracle-validation harness is read-only for operators, invisible to users; JSON caches are informational; book stubs are governance-only). No data migration, no API contract changes.
- **Deployment:** Requires restart of `apps/desktop` after `npm install` and `cargo build` (as usual). No special deployment steps or database migrations.
- **Update authorization:** Requires code review + CI pass + manual merge to `develop`. Criterion 6.5 (PR + merge) completes this gate.
- **Operator runbook:** No operator-facing changes to startup, configuration, or monitoring. New oracle-validation harness is a developer/verification tool (not shipped); new JSON caches and book-stub registry are optional/informational.
- **Staged rollout:** Not required. No feature flags, no gradual deployment strategy needed; the whole bundle ships as one atomic release.
