# SD-27 — Artifacts

> **Per-cycle receipt structure.** Every cycle produces a `<cycle>-cycle_receipt.md` file under the corresponding `epic_<n>/` directory.
>
> **Rewritten 2026-07-27 to the committed 2-book scope** (ARG + PU). The tree previously listed 19 books under a `<book>_cache-cycle_receipt.md` naming that no other document used; cycle IDs are now `pre_build` / `verify` / `parity` throughout, matching `epic-breakdown.md`, `loop-instruction.md`, `progress.md` and `scripts/sd27-workflow.py`.
>
> `receipts.md` is **not** a cycle receipt — it is the append-only architecture ledger that `scripts/architecture-truth-up.sh` and `scripts/graphify-update.sh` require and append to.

## 1. Directory structure

```
artifacts/
├── epic_1/
│   ├── .gitkeep
│   └── identifier-audit-cycle_receipt.md          (E1.1)
├── epic_2/
│   ├── .gitkeep
│   ├── label-resolution-cycle_receipt.md                              (E2.0)
│   ├── 2.0.5-shape-b-license-stripping-preflight-cycle_receipt.md      (E2.0.5)
│   ├── 2.0.6-crb-license-retrofit-cycle_receipt.md                    (E2.0.6)
│   ├── 2.0.7-apg-license-retrofit-cycle_receipt.md                    (E2.0.7)
│   ├── 2.0.8-acg-license-retrofit-cycle_receipt.md                    (E2.0.8)
│   ├── 2.0.9-beastiary-license-retrofit-cycle_receipt.md              (E2.0.9)
│   ├── 2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md (E2.0.10)
│   ├── advanced_race_guide_pre_build-cycle_receipt.md                 (E2.1)
│   ├── advanced_race_guide_verify-cycle_receipt.md                    (E2.1')
│   ├── pathfinder_unchained_pre_build-cycle_receipt.md                (E2.2)
│   └── pathfinder_unchained_verify-cycle_receipt.md                   (E2.2')
├── epic_3/
│   ├── .gitkeep
│   ├── advanced_race_guide_parity-cycle_receipt.md                    (E3.1)
│   └── pathfinder_unchained_parity-cycle_receipt.md                   (E3.2)
├── receipts.md                                                        (append-only architecture ledger, E4.2)
└── epic_4/
    ├── .gitkeep
    ├── final-criterion-scan-cycle_receipt.md       (E4.1)
    ├── architecture-closure-cycle_receipt.md        (E4.2)
    ├── release-notes-cycle_receipt.md               (E4.3)
    ├── version-bump-cycle_receipt.md                (E4.4)
    ├── pr-merge-cycle_receipt.md                    (E4.5)
    └── closure-readiness-report.md                  (referenced by E4.5's PR body, not its own cycle — see acceptance-and-verification.md §2.9)
```

## 2. Per-cycle receipt shape

Every cycle's receipt follows the canonical six-section shape (per `loop-instruction-template.md §6`):

1. **Cycle header** — `Cycle ID`, `Criterion`, `Owner`, `Status`, `Route class`, `Started at`, `Completed at`.
2. **Inputs** — exact file paths consulted, exact prior cycle outputs.
3. **Outputs** — exact files created/modified, exact lines added, exact commits.
4. **Operations** — RED → GREEN → REFACTOR walkthrough, dual-audit gate result.
5. **Verification** — exact commands run, exact pass/fail counts, exact receipts.
6. **Notes** — judgment calls, deferred items, audit-exclusion requests.

## 3. Per-cycle tier model

The per-cycle receipt's `Route class` field records the tier: `Sonnet` (default) or `free-discounted` (operator-authorized per `decisions.md §11`).

## 4. Cross-reference

- `./README.md` — bundle entry point.
- `./scope-draft.md` — the committed scope.
- `./decisions.md` — decision record.
- `./technical-design.md` — architectural surface.
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements.
- `./epic-breakdown.md` — per-cycle stories.
- `./loop-instruction.md` — per-cycle procedure.
- `./progress.md` — live cycle log.
- `./release-notes.md` — bundle summary at closure.
- `./acceptance-and-verification.md` — per-criterion acceptance + verification commands.

A receipt is also the reporting manifest's proof-of-work: `scripts/sd27-workflow.py complete` refuses a
receipt path that does not exist on disk, so a `complete` item always points at a real file
(`loop-instruction.md §8`).
