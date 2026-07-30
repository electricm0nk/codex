# SD-27 — Epic Breakdown

> **Note (2026-07-27):** Beginner Box (was E2.3 / E3.3) and Core Essentials (was E2.10 / E3.10) were removed from scope per operator directive (redundant to other tomes; will not be brought in). All E2.x and E3.x cycle-IDs below the deletions have been cascade-renumbered to fill the gaps. The Tier-2 range is unaffected.

## 1. Epic 1 — Code-Side Identifier Cleanup (governance base; fires FIRST)

Single cycle + post-SD-26 cleanup. Defensive scope, runner audits the existing identifier shape, no renames expected.

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 1.1 | `identifier-audit` | Backend | Sonnet | Standard audit; produces `artifacts/epic_1/identifier-audit-cycle_receipt.md` |

## 2. Epic 2 — Book Stub Resolution (19 future-state books)

20 cycles total: 1 label-resolution cycle + 19 per-book cycles (down from 22 = 1 + 21 before the Beginner Box + Core Essentials removal).

### 2.1 Tier-1 (13 books, parallel)

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 2.0 | `label-resolution` | Backend | Sonnet | Per-cycle blocking decision; resolves the `SD-27` vs. `SD-27+ (unscheduled)` discrepancy across 20 surfaces (19 stubs + 1 decisions.md) |
| 2.1 | `advanced_race_guide_cache` | Backend | Sonnet (or free/discounted) | Tier-1; in-scope for SD-27 |
| 2.2 | `adventurers_guide_cache` | Backend | Sonnet (or free/discounted) | Tier-1; in-scope for SD-27 |
| 2.3 | `bestiary_2_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.4 pre-2026-07-27 |
| 2.4 | `bestiary_3_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.5 pre-2026-07-27 |
| 2.5 | `bestiary_4_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.6 pre-2026-07-27 |
| 2.6 | `bestiary_5_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.7 pre-2026-07-27 |
| 2.7 | `bestiary_6_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.8 pre-2026-07-27 |
| 2.8 | `bonus_bestiary_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.9 pre-2026-07-27 |
| 2.9 | `horror_adventures_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.11 pre-2026-07-27 |
| 2.10 | `monster_codex_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.12 pre-2026-07-27 |
| 2.11 | `mythic_adventures_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.13 pre-2026-07-27 |
| 2.12 | `occult_adventures_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.14 pre-2026-07-27 |
| 2.13 | `pathfinder_unchained_cache` | Backend | Sonnet (or free/discounted) | Tier-1; was E2.15 pre-2026-07-27 |

### 2.2 Tier-2 (6 books, parallel after Tier-1 reaches parity phase)

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 2.14 | `ultimate_campaign_cache` | Backend | Sonnet (or free/discounted) | Tier-2; was E2.16 pre-2026-07-27 |
| 2.15 | `ultimate_combat_cache` | Backend | Sonnet (or free/discounted) | Tier-2; was E2.17 pre-2026-07-27 |
| 2.16 | `ultimate_equipment_cache` | Backend | Sonnet (or free/discounted) | Tier-2; was E2.18 pre-2026-07-27 |
| 2.17 | `ultimate_intrigue_cache` | Backend | Sonnet (or free/discounted) | Tier-2; was E2.19 pre-2026-07-27 |
| 2.18 | `ultimate_magic_cache` | Backend | Sonnet (or free/discounted) | Tier-2; was E2.20 pre-2026-07-27 |
| 2.19 | `ultimate_wilderness_cache` | Backend | Sonnet (or free/discounted) | Tier-2; was E2.21 pre-2026-07-27 |

### 2.3 Per-book cycle story (per `loop-instruction.md`)

```
Per-book cycle (E2.x):
1. Read source LST corpus at ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/
2. Inventory content kinds (classes, spells, equipment, feats, bestiary, etc.)
3. Generate src/rules_core/rules_tables/<book>/ Rust module (Shape B enumeration)
4. Run sd27_gen_book_cache codegen tool against the rules_tables module
5. Write data/corpus/<book>/{content_kind}/{content_id}.json per Shape B
6. Write tests/sd27_<book>_cache_shape.rs (Shape B key-set + key-order conformance)
7. Update docs/governance/wired-integration-stubs-registry.md: book_stub entry's
   Status → "Resolved" + resolved_at + resolved_by + bundle_of_record + cycle_receipt
8. Update data/stubs/<book>.json: content_kind_counts → real number map
9. Run dual-audit gate (identifier-discipline + wired-integration four-check)
10. Commit + push + receipt
```

## 3. Epic 3 — PCGen Parity Baseline (19 fixtures)

19 cycles, one per book (down from 21 before the Beginner Box + Core Essentials removal). Parallel-after-E2.0.

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 3.1 | `advanced_race_guide_parity` | Backend | Sonnet (or free/discounted) | Tier-1; in-scope for SD-27 |
| 3.2 | `adventurers_guide_parity` | Backend | Sonnet (or free/discounted) | Tier-1; in-scope for SD-27 |
| 3.3 | `bestiary_2_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.4 pre-2026-07-27 |
| 3.4 | `bestiary_3_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.5 pre-2026-07-27 |
| 3.5 | `bestiary_4_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.6 pre-2026-07-27 |
| 3.6 | `bestiary_5_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.7 pre-2026-07-27 |
| 3.7 | `bestiary_6_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.8 pre-2026-07-27 |
| 3.8 | `bonus_bestiary_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.9 pre-2026-07-27 |
| 3.9 | `horror_adventures_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.11 pre-2026-07-27 |
| 3.10 | `monster_codex_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.12 pre-2026-07-27 |
| 3.11 | `mythic_adventures_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.13 pre-2026-07-27 |
| 3.12 | `occult_adventures_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.14 pre-2026-07-27 |
| 3.13 | `pathfinder_unchained_parity` | Backend | Sonnet (or free/discounted) | Tier-1; was E3.15 pre-2026-07-27 |
| 3.14 | `ultimate_campaign_parity` | Backend | Sonnet (or free/discounted) | Tier-2; was E3.16 pre-2026-07-27 |
| 3.15 | `ultimate_combat_parity` | Backend | Sonnet (or free/discounted) | Tier-2; was E3.17 pre-2026-07-27 |
| 3.16 | `ultimate_equipment_parity` | Backend | Sonnet (or free/discounted) | Tier-2; was E3.18 pre-2026-07-27 |
| 3.17 | `ultimate_intrigue_parity` | Backend | Sonnet (or free/discounted) | Tier-2; was E3.19 pre-2026-07-27 |
| 3.18 | `ultimate_magic_parity` | Backend | Sonnet (or free/discounted) | Tier-2; was E3.20 pre-2026-07-27 |
| 3.19 | `ultimate_wilderness_parity` | Backend | Sonnet (or free/discounted) | Tier-2; was E3.21 pre-2026-07-27 |

### 3.1 Per-book parity cycle story

```
Per-book cycle (E3.x):
1. Read `data/corpus/<book>/` cache (the bundle's own E2.x output)
2. Author `pf_<book>_human_<class>_level1_golden.pcg` fixture
   (mirrors SD-26 pilot Fighter pattern at
   programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/)
3. Run the existing PCGen Gradle pipeline against the fixture
   (scripts/pcgen-run-character.sh, scripts/pcgen-normalize-output.py)
4. Sanitize output, write `data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json`
5. Run `src/oracle_validation/comparator::compare` against the per-book receipt
6. Record the per-cycle parity comparison in `artifacts/epic_3/<id>_parity-cycle_receipt.md`
7. Document the inherited CG-03 baseline in the receipt (7-of-9 ceiling)
8. Commit + push + receipt
```

## 4. Epic 4 — Closure Epilogue (fires LAST)

5 criteria + per-criterion tiering.

| Criterion | Cycle | Owner | Tier | Notes |
|-----------|-------|-------|------|-------|
| 4.1 | `final-criterion-scan` | Backend | Sonnet | Per-criterion terminal-state table cross-checked against 3 independent sources |
| 4.2 | `architecture-closure` | Backend | Opus | Truth-up + graphify + PR + merge; template §2's adversarial-verify |
| 4.3 | `release-notes` | Backend | Haiku | Bundle summary + per-book resolution table + caveats |
| 4.4 | `version-bump` | Backend | Haiku | 0.6.0 → 0.6.1 per major.tranche-base.build scheme |
| 4.5 | `pr-merge` | Backend | Sonnet | Operator merges per standing convention |

## 5. Total cycle count

| Epic | Cycles |
|------|--------|
| E1 | 1 |
| E2 | 20 (1 label + 19 per-book) |
| E3 | 19 |
| E4 | 5 |
| Total | 45 cycles |

(Was 49 before the 2026-07-27 Beginner Box + Core Essentials removal: -2 E2 cycles, -2 E3 cycles.)

## 6. Cross-reference

- `./scope-draft.md` — the committed scope.
- `./decisions.md` — decision record.
- `./technical-design.md` — architectural surface.
- `./technical-requirements.md` — pre-loop prerequisites + normative requirements.
- `./loop-instruction.md` — per-cycle procedure.
- `./progress.md` — live cycle log.
- `./artifacts/README.md` — per-cycle receipt structure.