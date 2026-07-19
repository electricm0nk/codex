# SD-22 — Cycle-Artifacts Index (`artifacts/`)

Per **operator directive 2026-07-19**: every criterion 1-31 has a per-cycle receipt artifact under `artifacts/`. The receipt is the load-bearing surface for Epic 9's Closure Readiness evaluation — Epic 9 cannot conclude a criterion `complete` without the corresponding artifact's RED→GREEN transition having been persisted.

The **canonical source-of-truth for which artifact path belongs to which criterion** is [`corpus-source-inventory.md`](../corpus-source-inventory.md) §6 ("Cycle-artifact reader's contract"). The same table is mirrored in [`acceptance-and-verification.md`](../acceptance-and-verification.md) §"Per-criterion closure gate → artifact map."

## 1. Where artifacts land

The repo-local artifacts directory is `docs/release/SD-22/artifacts/` (mirror of this upstream). On disk at session-end, it should contain:

```
docs/release/SD-22/artifacts/
├── epic_1/
│   ├── identifier_audit_red.log
│   ├── identifier_audit_green.log
│   └── per_rename_<old-id>_to_<new-id>_cycle_receipt.md
│       (one MD per rename cycle; e.g. per_rename_sd22_X_to_create_X_cycle_receipt.md)
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
│   ├── class_<class>_cycle_receipt.md (×10)
│   ├── cross_book_acg_invariants_cycle_receipt.md
│   ├── spell_list_cycle_receipt.md
│   └── equipment_tables_cycle_receipt.md
├── beastiary1/
│   ├── mod_rs_cycle_receipt.md
│   ├── subset_<NN>_cycle_receipt.md (one per monster-block subset)
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
└── closure-readiness-report.md
```

(Total receipt files: ~50. The numbers above match `corpus-source-inventory.md` per-class cycle counts: 8 APG classes + 10 ACG classes + default 8 Bestiary 1 subsets + per-criterion receipts for epics 1, 2, 5, 7, 8, plus criterion-31's closure-readiness-report.md.)

## 2. What each receipt looks like

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

## Cycle metadata
- cycle_id: <ISO-8601 timestamp>
- duration: <N> seconds
- bundle_criterion: <criterion-NN>
- upstream reference: <path to the cycle-generated corpus file, e.g. corpus/apg_alchemist.json (generated in-cycle per decisions.md §5)>
- RuleSetId: <Apg | Acg | Bestiary1>

## kanban
- card: <hermes kanban card id>
- audit_comment: <comment id>
```

Receipts without RED-phase evidence are Bucket-B / Bucket-C shortfalls; Epic 9's evaluator (criterion-31) treats them as self-heal triggers.

## 3. Operator / cold-cloud-clone read path

A coding harness operating on a cold cloud clone (no access to `~/workspace/`) reads this tree:

1. `corpus-source-inventory.md` first (the load-bearing four-tuple: rust module / test fixture / cycle artifact / RuleSetId per content unit).
2. `acceptance-and-verification.md` §"Per-criterion closure gate → artifact map" (the per-criterion artifact path table).
3. This `artifacts/README.md` (the directory layout + receipt shape).
4. The per-cycle receipts themselves (the load-bearing surfaces of Epic 9's evaluation).

## 4. Recorded

Authored 2026-07-19 per operator directive ("requirements documents should have more artifacts and references; coding harness needs more info to go by; call out the red-green TDD mandate"). The downstream `docs/release/SD-22/artifacts/README.md` is the cloud-clone-accessible mirror of this file.
