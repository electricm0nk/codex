# SD-24 — Progress

> **Operating method:** see `./scope-draft.md` and `./loop-instruction.md`. This file is created on cycle 0 of Epic 2 with the deterministic seed of 35 criteria. Cycle picker reads `## TODO` + `## DISCOVERED` per the deterministic-seeded-then-dynamic model.

This file is the bundle's runtime state. The loop's `progress.md` is the canonical cycle-log + status matrix; the kanban board is the durable receipt; the per-cycle `artifacts/<epic>/<cycle>_cycle_receipt.md` is the per-cycle truth.

`## TODO` — the deterministic seed + dynamic entries, in dispatch-priority order. The picker reads this first.

`## DONE` — completed criteria, with commit SHA and cycle-id. Append-only.

`## DISCOVERED` — cycle-found items outside the deterministic list. Each entry has `<ISO-8601> | <epic-of-origin> | <criterion-of-origin> | <priority-bump-tag> | <description> | <suggested-epic-and-criterion>`. Priority-bump items go to the front of `## TODO` on operator call.

`## Status matrix` — per-criterion state in epic+criterion-number order (1.1, 1.2, 2.1, ..., 8.4). Update on every cycle's Step 9.

`## Cycle log` — append-only cycle entries (per `./loop-instruction.md §3` schema).

`## Open blockers` — non-self-healable items requiring operator intervention.

## Status matrix (placeholder — populated by cycle 0 of Epic 2)

| Criterion | State | Cycle ID | Commit SHA | Notes |
|---|---|---|---|---|
| 1.1 Source-code identifier audit | not-started | — | — | Epic 1 fires FIRST |
| 1.2 Per-cycle tests pass | not-started | — | — | — |
| 2.1 board reachable | not-started | — | — | — |
| 2.2 branch pushed | not-started | — | — | — |
| 2.3 SD-23 closure PR merged | not-started | — | — | Tier-1 launch gate |
| 2.4 working tree clean | not-started | — | — | — |
| 2.5 doctrines loaded | not-started | — | — | — |
| 3.1 Wired-Integration Audit | not-started | — | — | — |
| 3.2 forbidden tokens remediation | not-started | — | — | — |
| 3.3 noop handlers remediation | not-started | — | — | — |
| 3.4 mock leaks + "Would …" remediation | not-started | — | — | — |
| 4.1 per-class audit CRB | not-started | — | — | — |
| 4.2 per-class audit APG | not-started | — | — | — |
| 4.3 per-class audit ACG | not-started | — | — | — |
| 4.4 remediation plan | not-started | — | — | — |
| 4.5 APG/ACG multiclass deferral | not-started | — | — | — |
| 5.1 F+W multiclass dispatch | not-started | — | — | gated on 4.x |
| 5.2 30 character-advancement cycles | not-started | — | — | gated on 5.1 |
| 5.3 integration test | not-started | — | — | gated on 5.2 |
| 5.4 multiclass four-check audit | not-started | — | — | gated on 5.3 |
| 5.5 APG/ACG multiclass deferred | not-started | — | — | — |
| 6.1 Equipment coverage audit | not-started | — | — | — |
| 6.2 Equipment content completion: cost | not-started | — | — | gated on 6.1 |
| 6.3 Equipment content completion: weight | not-started | — | — | gated on 6.1 |
| 6.4 Equipment content completion: description | not-started | — | — | gated on 6.1 |
| 6.5 Spell content completion: full text | not-started | — | — | gated on 6.1 |
| 7.1 appendToCharacter Tauri command | not-started | — | — | gated on 5/6 |
| 7.2 recomputeCharacter Tauri command | not-started | — | — | — |
| 7.3 reSaveCharacter Tauri command | not-started | — | — | — |
| 7.4 Add Weapon/Armor/Spell onClick | not-started | — | — | — |
| 7.5 loadout hardcoding removed | not-started | — | — | — |
| 8.1 Final criterion scan | not-started | — | — | fires LAST |
| 8.2 Architecture closure pipeline | not-started | — | — | fires LAST |
| 8.3 Release notes | not-started | — | — | fires LAST |
| 8.4 Build version increment | not-started | — | — | fires LAST |

## TODO (deterministic seed; populated by cycle 0 of Epic 2)

- 1.1 (Epic 1, Identifier audit; only eligible at loop launch)
- 1.2 (Epic 1, Per-cycle tests pass)
- 2.1, 2.2, 2.3, 2.4, 2.5 (Epic 2, gating epic)
- 3.1, 3.2, 3.3, 3.4 (Epic 3, audit + remediation)
- 4.1, 4.2, 4.3, 4.4, 4.5 (Epic 4, per-class audit)
- 5.1, 5.2, 5.3, 5.4, 5.5 (Epic 5, multiclass F+W)
- 6.1, 6.2, 6.3, 6.4, 6.5 (Epic 6, equipment 100%)
- 7.1, 7.2, 7.3, 7.4, 7.5 (Epic 7, unwired workflows + Tauri surface)
- 8.1, 8.2, 8.3, 8.4 (Epic 8, closure epilogue)

## DONE

(empty — populated by completed cycles)

## DISCOVERED

(empty — populated by cycles that find work outside the deterministic list)

## Cycle log

(empty — populated by completed cycles)

## Open blockers

(empty — populated by non-self-healable failures)

---

*Per `./loop-instruction.md §2.3 step 9`: the cycle picker updates this file in place on every cycle. Do not rewrite from scratch; each cycle appends to its own section.*
