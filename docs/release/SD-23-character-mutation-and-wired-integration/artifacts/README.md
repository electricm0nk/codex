---
title: SD-23 — Cycle-Artifacts Index (`artifacts/`)
status: closed 2026-07-21
scope: docs/release/SD-23-character-mutation-and-wired-integration/artifacts
artifact_type: index
date: 2026-07-20
canonical_branch: tranche/5-1
kanban_board: codex-tranche-5
purpose: "Epic 7 (Closure Epilogue) criteria each have a per-cycle receipt artifact under artifacts/epic_7/. Epics 1-6's per-cycle receipts live in progress.md's cycle log, not as separate files — see §1's note below for why this deviates from the original plan documented here."
---

# SD-23 — Cycle-Artifacts Index (`artifacts/`)

This index documents the per-cycle receipt artifacts for SD-23's closure epilogue. Mirrors the SD-22 convention (`artifacts/` + per-cycle receipt files), scoped to Epic 7 in actual execution — see the deviation note below.

## 1. Where artifacts actually landed (corrected 2026-07-21, post-closure)

```
docs/release/SD-23-character-mutation-and-wired-integration/artifacts/
├── README.md                                          (this file)
├── epic_3/ .. epic_6/                                 (empty — see deviation note below)
├── epic_7/                                            (Closure Epilogue — per-criterion receipts)
│   ├── pre_promotion_verification_cycle_receipt.md    (criterion 25: 16 closure gates verified)
│   ├── architecture_truth_up_cycle_receipt.md         (criterion 26: architecture truth-up)
│   ├── graphify_update_cycle_receipt.md               (criterion 27: graphify update, non-blocking failure)
│   ├── merge_conflict_resolution_cycle_receipt.md     (criterion 28: pre-flight merge-conflict check)
│   ├── promotion_pr_cycle_receipt.md                  (criterion 29: tranche/5-1 → develop PR)
│   ├── build_counter_advance_cycle_receipt.md         (criterion 30: 0.5.96 → 0.5.97)
│   ├── decisions_risks_final_review_cycle_receipt.md  (criterion 31: decisions + risks final review)
│   ├── progress_log_complete_cycle_receipt.md         (criterion 32: progress.md cycle log complete)
│   └── bundle_closed_on_board_cycle_receipt.md        (criterion 33: 14 kanban cards complete)
└── closure-readiness-report.md                        (criterion 33's canonical closure report)
```

**Deviation from the original plan (documented here for anyone auditing this bundle):** this file originally planned per-criterion receipt files under `epic_3/` through `epic_6/` subdirectories, mirroring SD-22's structure. Execution didn't follow that — Epics 1-6's per-cycle post-mortems (commit SHAs, files touched, audit results, judgment calls) were recorded directly in `progress.md`'s cycle log instead, which turned out to carry the same information with less duplication. `epic_7/` does have separate artifact files because Epic 7's criteria are individually script-driven (each closure-pipeline script — truth-up, graphify, merge-conflict — needs its own narrative receipt beyond the YAML block it appends to `receipts.md`). Retroactively backfilling `epic_3/`-`epic_6/` with files that would just restate `progress.md`'s existing entries was judged not worth doing at closure — the real audit trail (kanban card comments + `progress.md` + actual commits) already fully exists; fabricating additional files to match a plan that changed during execution would add noise, not evidence. `epic_3/`-`epic_6/` are left as empty directories rather than deleted, so the original plan's structure is still visible for comparison.

Epic 1 (Identifier Cleanup) and Epic 2 (Operator Pre-Launch) do not produce code-bearing cycles — their receipts live in `progress.md` (cycles 1-2).

## 2. Two sibling doctrine docs that Epic 7 reads

- **`../content-unit-inventory.md`** — per-content-unit four-tuple (rust_module_path / test_fixture_path / cycle_artifact_path / RuleSetId-or-CommandName). The wired-integration equivalent of SD-22's `corpus-source-inventory.md`.
- **`../acceptance-and-verification.md` §"Per-criterion artifact map"** — the per-criterion artifact-path table.
- **`../loop-instruction.md` Step 7-8** — the per-cycle procedure, with cross-references to this README.

## 3. What each per-cycle receipt looks like

Per `loop-instruction.md` §"Post-mortem schema," every per-cycle receipt has this shape:

```markdown
# <cycle-name> cycle receipt — <ISO-8601 UTC>

## Red-phase evidence (where applicable)
<command> <output>
<test failure paste showing the test fails for the intended reason>

## Green-phase evidence
<command> <output>
<test success paste showing all green>

## Files touched
- `apps/desktop/...` — added/modified
- `apps/desktop/src-tauri/...` — added/modified
- `src/...` — added/modified
- `tests/...` — added/modified
- `docs/release/SD-23-character-mutation-and-wired-integration/artifacts/epic_7/<cycle>_cycle_receipt.md` — this file

## Audit result
- OK_NO_TOKENS
- OK_NO_NOOP_HANDLERS
- OK_NO_MOCK_LEAKS
- OK_NO_WOULD_STRINGS

## Cycle metadata
- cycle_id: <ISO-8601 timestamp>
- duration: <N> seconds
- bundle_criterion: <criterion-NN>
- four_check_audit: <pass|fail>
- identifier_discipline_audit: <pass|fail>

## kanban
- card: <hermes kanban card id>
- audit_comment: <comment id>
- commit_sha: <sha>
```

Receipts without RED-phase evidence (for non-TDD criteria like Epic 4 criterion 13 field deletion) get a "Reconciliation evidence" section instead — the before/after state of the data model showing the field is gone. Bucket-B / Bucket-C shortfalls are self-heal triggers; Epic 7's evaluator treats them as not-yet-complete.

## 4. Operator / cold-cloud-clone read path

A coding harness operating on a cold cloud clone (no access to `~/workspace/`) reads this tree:

1. **`../content-unit-inventory.md`** first (the four-tuple: rust module / test fixture / cycle artifact / RuleSetId-or-CommandName per content unit).
2. **`../acceptance-and-verification.md` §"Per-criterion closure gate → artifact map"** (the per-criterion artifact-path table).
3. **`../loop-instruction.md`** third (the operator-pinned cycle pipeline: RED → GREEN → cycle-artifact → commit).
4. **Per-cycle receipts** — the load-bearing surfaces of Epic 7's evaluation.
5. **`../risks-and-open-questions.md`** — latent risks (R1: storage-tier referential integrity; R2: stat-field promotion; R3: SD-22 closure gating).

## 5. Recorded

Authored 2026-07-20 per SD-23 scope-drafting session. Mirrors SD-22's artifacts/README.md structure (canonical pattern: cycle-artifacts index + per-Epic subdirectory + per-cycle receipt files + closure-readiness-report). Updated 2026-07-21 at bundle closure to correct stale `programs/` paths, fix Epic 7's criterion numbers (25-33, not 25-30), and document the Epics 1-6 artifact-location deviation (see §1).
