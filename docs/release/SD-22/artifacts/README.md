---
title: SD-22 — Cycle-Artifacts Index (`artifacts/`)
status: representative-content (operator directive 2026-07-19)
scope: docs/release/SD-22/artifacts
artifact_type: index
date: 2026-07-19
canonical_branch: tranche/5
kanban_board: codex-tranche-5
purpose: "Every criterion 1-31 has either (a) a per-cycle receipt artifact under `artifacts/` (Epic 1-8 cycles), or (b) the on-disk source-shape artifact under `artifacts/corpus/` (Epic 3/4/5/6 ingest cycles). Both surfaces are load-bearing for Epic 9's Closure Readiness evaluation — Epic 9 cannot conclude a criterion `complete` without the corresponding artifact's RED→GREEN transition having been persisted, OR the corpus-source artifact shape existing (verified by RED-phase parser against the stub)."
mirror_of: /home/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/artifacts/README.md
---

# SD-22 — Cycle-Artifacts Index (`artifacts/`)

This index documents two sibling artifact surfaces under `artifacts/`:

1. **`corpus/`** (operator directive 2026-07-19) — on-disk file-shape stubs that coding harnesses read at RED-phase to know what the input source looks like. Stub files model the schemas operator-supplied files must satisfy at cycle-launch.
2. **Per-cycle receipts (`epic_*/` + `apg/` + `acg/` + `beastiary1/` + `dm_toolkit/`)** — the RED→GREEN transition evidence every Epic 1-9 cycle writes so Epic 9's evaluator can conclude criteria `complete`.

Plus the top-level `closure-readiness-report.md` for criterion-31.

## 1. Where artifacts land

```
docs/release/SD-22/artifacts/
├── corpus/                                          (per operator directive 2026-07-19: on-disk corpus-shape surface)
│   ├── README.md                                    (the schema-of-record for the corpus directory)
│   ├── apg/                                         (6 APG class stubs + the operator-supplied slot README)
│   ├── acg/                                         (10 ACG class stubs + the operator-supplied slot README)
│   ├── beastiary1/                                  (3 default subset samples + Tarrasque edge case + operator-supplied slot README)
│   ├── spell-list/                                  (APG + ACG shared spell lists)
│   ├── equipment-table/                             (APG + ACG shared equipment tables)
│   └── operator-supplied/                           (licensed Paizo/PcGen files dropped here at cycle-launch; gitignored)
├── epic_1/
│   ├── identifier_audit_red.log
│   ├── identifier_audit_green.log
│   └── per_rename_<old-id>_to_<new-id>_cycle_receipt.md
├── epic_2/
│   ├── codex_tranche_5_pin_cycle_receipt.md
│   ├── tranche_5_push_cycle_receipt.md
│   └── no_claude_in_flight_cycle_receipt.md
├── apg/
│   ├── mod_rs_cycle_receipt.md
│   ├── class_alchemist_cycle_receipt.md
│   ├── class_cavalier_cycle_receipt.md
│   ├── class_gunslinger_cycle_receipt.md
│   ├── class_inquisitor_cycle_receipt.md
│   ├── class_magus_cycle_receipt.md
│   ├── class_oracle_cycle_receipt.md
│   ├── class_summoner_cycle_receipt.md
│   ├── class_witch_cycle_receipt.md
│   ├── cross_book_apg_crb_invariants_cycle_receipt.md
│   ├── spell_list_cycle_receipt.md
│   └── equipment_tables_cycle_receipt.md
├── acg/
│   ├── mod_rs_cycle_receipt.md
│   ├── class_alchemist_cycle_receipt.md
│   ├── class_arcanist_cycle_receipt.md
│   ├── class_bloodrager_cycle_receipt.md
│   ├── class_brawler_cycle_receipt.md
│   ├── class_hunter_cycle_receipt.md
│   ├── class_investigator_cycle_receipt.md
│   ├── class_shaman_cycle_receipt.md
│   ├── class_skald_cycle_receipt.md
│   ├── class_swashbuckler_cycle_receipt.md
│   ├── class_warpriest_cycle_receipt.md
│   ├── cross_book_acg_invariants_cycle_receipt.md
│   ├── spell_list_cycle_receipt.md
│   └── equipment_tables_cycle_receipt.md
├── beastiary1/
│   ├── mod_rs_cycle_receipt.md
│   ├── subset_<NN>_cycle_receipt.md (one per monster-block subset; default 8)
│   ├── tarrasque_edge_case_cycle_receipt.md (criterion-17's coverage)
│   ├── cross_book_invariants_cycle_receipt.md
│   └── dm_toolkit_consumption_cycle_receipt.md
├── dm_toolkit/
│   ├── encounters_cycle_receipt.md
│   ├── party_cr_cycle_receipt.md
│   ├── deterministic_tests_cycle_receipt.md
│   ├── happy_path_integration_cycle_receipt.md
│   └── json_roundtrip_cycle_receipt.md
├── epic_7/
│   ├── final_scan_cycle_receipt.md
│   ├── closure_pr_cycle_receipt.md
│   ├── worktree_branch_cleanup_cycle_receipt.md
│   ├── release_notes.md
│   └── tranche_version_increment_cycle_receipt.md
├── epic_8/
│   ├── three_version_fields_cycle_receipt.md
│   ├── build_label_format_cycle_receipt.md
│   └── per_cycle_tests_cycle_receipt.md
└── closure-readiness-report.md                      (criterion-31's Epic 9 artifact)
```

(Total receipt files: ~48. The numbers above match `corpus-source-inventory.md` per-class cycle counts: 6 APG classes (corrected 2026-07-19) + 10 ACG classes + default 8 Bestiary 1 subsets + per-criterion receipts for epics 1, 2, 5, 7, 8, plus criterion-31's closure-readiness-report.md.)

## 2. Two sibling doctrine docs that Epic 9 reads

- **`../corpus-source-inventory.md`** — per-content-unit four-tuple (rust_module_path / test_fixture_path / cycle_artifact_path / RuleSetId).
- **`../ingest.md`** — operator-pinned canonical process doctrine for RED → GREEN → cycle-artifact → commit pipeline (per operator directive 2026-07-19).
- **`../loop-instruction.md` Step 4-5** — the per-cycle procedure, with cross-references to `ingest.md` for Epic 3/4/5/6 cycles.

## 3. What each per-cycle receipt looks like

Per `corpus-source-inventory.md` §6, every per-cycle receipt has this shape:

```markdown
# <class/monster/module name> cycle receipt — <ISO-8601 UTC>

## Red-phase evidence
<command> <output>
<paste from `cargo test --test sd22_<X>_<Y>_resolves 2>&1 | tail -40` showing the test fails for the intended reason>

## Green-phase evidence
<command> <output>
<paste from `cargo test --locked 2>&1 | tail -20` and `cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20` showing all green>

## Files touched
- `src/...` — added/modified
- `tests/...` — added/modified
- `docs/release/SD-22/artifacts/corpus/operator-supplied/<book>/<file>.lst` — operator-supplied licensed file that the cycle consumed; the bundled stub `<book>.lst.md` was renamed to `<book>.lst.md.superseded` at the time of the swap. Path is `<corpus_input_path>` from `corpus-source-inventory.md` §1-3.

## Cycle metadata
- cycle_id: <ISO-8601 timestamp>
- duration: <N> seconds
- bundle_criterion: <criterion-NN>
- corpus_input_path: `<artifacts/corpus/<book>/<file>.lst>` (the canonical stub path; the operator-supplied swap at cycle-launch is at `<artifacts/corpus/operator-supplied/<book>/<file>.lst>` with the same schema)
- RuleSetId: <Apg | Acg | Bestiary1>
- ingest_pipeline_version: 1 (per `ingest.md` §6; bump if the column-count schema changes)

## kanban
- card: <hermes kanban card id>
- audit_comment: <comment id>
```

Receipts without RED-phase evidence are Bucket-B / Bucket-C shortfalls; Epic 9's evaluator (criterion-31) treats them as self-heal triggers.

## 4. Operator / cold-cloud-clone read path

A coding harness operating on a cold cloud clone (no access to `~/workspace/`) reads this tree:

1. **`../corpus-source-inventory.md`** first (the load-bearing four-tuple: rust module / test fixture / cycle artifact / RuleSetId per content unit).
2. **`./corpus/`** second (the actual on-disk file shapes the cycle reads as input).
3. **`../ingest.md`** third (the operator-pinned ingest pipeline: RED → GREEN → cycle-artifact → commit + the operator-supplied swap procedure).
4. **`../acceptance-and-verification.md` §"Per-criterion closure gate → artifact map"** (the per-criterion artifact path table).
5. **Per-cycle receipts** — the load-bearing surfaces of Epic 9's evaluation.

## 5. Recorded

Authored 2026-07-19 per operator directive ("coding harness ran into some snags ... need to provide information how that is done, and source that content in an artifacts folder local to the repo. Any lst of pcc files that we needed have to be in that folder. references to those files need to be made in the handover"). 26 corpus files + per-receipt layouts + this README. Total +28 files (added in a single repo commit); mirror at the operator-workspace `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/artifacts/` + new `ingest.md` at the bundle root.
