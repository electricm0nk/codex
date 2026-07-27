# SD-27 — Artifacts

> **Per-cycle receipt structure.** Every cycle produces a `<cycle>_cycle_receipt.md` file under the corresponding `epic_<n>/` directory.

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
│   ├── 2.0.9-beastiary1-license-retrofit-cycle_receipt.md             (E2.0.9)
│   ├── 2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md (E2.0.10)
│   ├── advanced_race_guide_cache-cycle_receipt.md  (E2.1)
│   ├── adventurers_guide_cache-cycle_receipt.md    (E2.2)
│   ├── bestiary_2_cache-cycle_receipt.md           (E2.3; was E2.4 pre-2026-07-27)
│   ├── bestiary_3_cache-cycle_receipt.md           (E2.4; was E2.5 pre-2026-07-27)
│   ├── bestiary_4_cache-cycle_receipt.md           (E2.5; was E2.6 pre-2026-07-27)
│   ├── bestiary_5_cache-cycle_receipt.md           (E2.6; was E2.7 pre-2026-07-27)
│   ├── bestiary_6_cache-cycle_receipt.md           (E2.7; was E2.8 pre-2026-07-27)
│   ├── bonus_bestiary_cache-cycle_receipt.md       (E2.8; was E2.9 pre-2026-07-27)
│   ├── horror_adventures_cache-cycle_receipt.md    (E2.9; was E2.11 pre-2026-07-27)
│   ├── monster_codex_cache-cycle_receipt.md        (E2.10; was E2.12 pre-2026-07-27)
│   ├── mythic_adventures_cache-cycle_receipt.md    (E2.11; was E2.13 pre-2026-07-27)
│   ├── occult_adventures_cache-cycle_receipt.md    (E2.12; was E2.14 pre-2026-07-27)
│   ├── pathfinder_unchained_cache-cycle_receipt.md (E2.13; was E2.15 pre-2026-07-27)
│   ├── ultimate_campaign_cache-cycle_receipt.md     (E2.14; was E2.16 pre-2026-07-27)
│   ├── ultimate_combat_cache-cycle_receipt.md       (E2.15; was E2.17 pre-2026-07-27)
│   ├── ultimate_equipment_cache-cycle_receipt.md    (E2.16; was E2.18 pre-2026-07-27)
│   ├── ultimate_intrigue_cache-cycle_receipt.md     (E2.17; was E2.19 pre-2026-07-27)
│   ├── ultimate_magic_cache-cycle_receipt.md        (E2.18; was E2.20 pre-2026-07-27)
│   └── ultimate_wilderness_cache-cycle_receipt.md  (E2.19; was E2.21 pre-2026-07-27)
├── epic_3/
│   ├── .gitkeep
│   ├── advanced_race_guide_parity-cycle_receipt.md  (E3.1)
│   ├── adventurers_guide_parity-cycle_receipt.md    (E3.2)
│   ├── bestiary_2_parity-cycle_receipt.md           (E3.3; was E3.4 pre-2026-07-27)
│   ├── bestiary_3_parity-cycle_receipt.md           (E3.4; was E3.5 pre-2026-07-27)
│   ├── bestiary_4_parity-cycle_receipt.md           (E3.5; was E3.6 pre-2026-07-27)
│   ├── bestiary_5_parity-cycle_receipt.md           (E3.6; was E3.7 pre-2026-07-27)
│   ├── bestiary_6_parity-cycle_receipt.md           (E3.7; was E3.8 pre-2026-07-27)
│   ├── bonus_bestiary_parity-cycle_receipt.md       (E3.8; was E3.9 pre-2026-07-27)
│   ├── horror_adventures_parity-cycle_receipt.md    (E3.9; was E3.11 pre-2026-07-27)
│   ├── monster_codex_parity-cycle_receipt.md        (E3.10; was E3.12 pre-2026-07-27)
│   ├── mythic_adventures_parity-cycle_receipt.md    (E3.11; was E3.13 pre-2026-07-27)
│   ├── occult_adventures_parity-cycle_receipt.md    (E3.12; was E3.14 pre-2026-07-27)
│   ├── pathfinder_unchained_parity-cycle_receipt.md (E3.13; was E3.15 pre-2026-07-27)
│   ├── ultimate_campaign_parity-cycle_receipt.md     (E3.14; was E3.16 pre-2026-07-27)
│   ├── ultimate_combat_parity-cycle_receipt.md       (E3.15; was E3.17 pre-2026-07-27)
│   ├── ultimate_equipment_parity-cycle_receipt.md    (E3.16; was E3.18 pre-2026-07-27)
│   ├── ultimate_intrigue_parity-cycle_receipt.md     (E3.17; was E3.19 pre-2026-07-27)
│   ├── ultimate_magic_parity-cycle_receipt.md        (E3.18; was E3.20 pre-2026-07-27)
│   └── ultimate_wilderness_parity-cycle_receipt.md  (E3.19; was E3.21 pre-2026-07-27)
└── epic_4/
    ├── .gitkeep
    ├── final-criterion-scan-cycle_receipt.md       (E4.1)
    ├── architecture-closure-cycle_receipt.md        (E4.2)
    ├── release-notes-cycle_receipt.md               (E4.3)
    ├── version-bump-cycle_receipt.md                (E4.4)
    ├── pr-merge-cycle_receipt.md                    (E4.5)
    └── closure-readiness-report.md                  (E4.6; aggregate)
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
