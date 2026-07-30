# SD-27 — Progress

> **Live cycle log.** Per `spec-domain-bundle-authoring` — the canonical 10-file chassis includes `progress.md` for tracking cycle state. The orchestrator's `Workflow` writes `## TODO`, `## DONE`, `## DISCOVERED`, `## Status matrix`, `## Open blockers` sections as the bundle runs.

## Status matrix

| Criterion | Cycle | Epic | Status | Started | Completed | Notes |
|-----------|-------|------|--------|---------|-----------|-------|
| 1.1 | `identifier-audit` | E1 | pending | — | — | Defensive scope |
| 2.0 | `label-resolution` | E2 | pending | — | — | Per-cycle blocking decision |
| 2.0.5 | `shape-b-license-stripping-preflight` | E2 | pending | — | — | Schema bump + per-book `LICENSE.json` + PI-blacklist + redaction policy. Gates 2.0.6+ |
| 2.0.6 | `crb-license-retrofit` | E2 | pending | — | — | Retro-fit CRB records with license field + PI redaction |
| 2.0.7 | `apg-license-retrofit` | E2 | pending | — | — | Retro-fit APG records |
| 2.0.8 | `acg-license-retrofit` | E2 | pending | — | — | Retro-fit ACG records |
| 2.0.9 | `beastiary1-license-retrofit` | E2 | pending | — | — | Retro-fit Bestiary 1 records |
| 2.0.10 | `all-23-books-license-conformance-verify` | E2 | pending | — | — | Dual-audit gate across all 23 books (4 in-scope + 2 pre-built + 17 deferred stubs). Gates 2.1+ |
| 2.1 | `advanced_race_guide_pre_build` | E2 | pending | — | — | Tier-1; pre-build shape B v1 cache from LST corpus. Operator-gated: failure → operator picks next action |
| 2.1' | `advanced_race_guide_verify` | E2 | pending | — | — | Tier-1; verify pre-built cache against dual-audit gate |
| 2.1'' | `advanced_race_guide_parity` | E3 | pending | — | — | Tier-1; PCGen parity baseline against pre-built cache |
| 2.2 | `adventurers_guide_pre_build` | E2 | pending | — | — | Tier-1; pre-build shape B v1 cache from LST corpus. Operator-gated |
| 2.2' | `adventurers_guide_verify` | E2 | pending | — | — | Tier-1; verify pre-built cache against dual-audit gate |
| 2.2'' | `adventurers_guide_parity` | E3 | pending | — | — | Tier-1; PCGen parity baseline against pre-built cache |
| 3.1-3.19 | (deferred) | E3 | — | — | — | **17 deferred future-state books** (B2-B6, Bonus Bestiary, Horror Adventures, Monster Codex, Mythic Adventures, Occult Adventures, Pathfinder Unchained, the 6 Tier-2 Ultimate books) are operator-gated on SD-27 closing cleanly. Beginner Box and Core Essentials removed from scope per operator directive 2026-07-27 (redundant to other tomes; will not be brought in). Deferred to SD-28+. The pre-build cycle pattern from E2.1-2.2 is templated and reusable. |
| 4.1-4.5 | (closure) | E4 | pending | — | — | Standard closure epilogue |

## TODO

- (none yet)

## DONE

- (none yet)

## DISCOVERED

- (none yet)

## Open blockers

- **Tier-1 launch-gate dependency:** SD-26's `tranche/5-4 → develop` PR (#338) has not yet landed. SD-27's Epic 2.1+ blocks on this. Per `decisions.md §7`.
- **Bundle label discrepancy:** `SD-27` vs. `SD-27+ (unscheduled)` — operator's lever pull at cycle 2.0. Per `forward-scope-register.md §"Class 1.2"` + `decisions.md §2`.
- **CG-03 inherited baseline:** SD-27's per-book parity baseline asserts "match rate at cycle close," not 9-of-9. Per `forward-scope-register.md §"Class 0.3"` + `decisions.md §10`.
- **v0.6 in-progress overlap:** v0.6 is actively working class/race breadth (Fighter/Wizard/Rogue + 8 remaining CRB classes). SD-27's partition restricts SD-27 cycles to the per-book content paths. Per `scope-draft.md §4`.

## Cycle receipts

| Cycle | Receipt path |
|-------|--------------|
| (none yet) | — |
