# SD-27 — Progress

> **Live cycle log.** Per `spec-domain-bundle-authoring` — the canonical 10-file chassis includes `progress.md` for tracking cycle state. The orchestrator's `Workflow` writes `## TODO`, `## DONE`, `## DISCOVERED`, `## Status matrix`, `## Open blockers` sections as the bundle runs.

## Status matrix

| Criterion | Cycle | Epic | Status | Started | Completed | Notes |
|-----------|-------|------|--------|---------|-----------|-------|
| 1.1 | `identifier-audit` | E1 | complete | 2026-07-27T19:59Z | 2026-07-27T20:04Z | Full-tree scan (not diff-scoped — see receipt); 0 genuine bundle-tagged code identifiers. Matches SD-26's own E1.1 precedent |
| 2.0 | `label-resolution` | E2 | complete | 2026-07-27T20:10Z | 2026-07-27T20:16Z | Operator chose `"SD-27"`. All 21 stubs + registry + v0.6 risks doc resolved |
| 2.0.5 | `shape-b-license-stripping-preflight` | E2 | complete | 2026-07-27T20:20Z | 2026-07-27T20:28Z | src/rules_core/shape_b_v1.rs + docs/governance/ogl-pi-blacklist.md (draft) + dual-audit test, all independently re-verified. Gates 2.0.6+ |
| 2.0.6 | `crb-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 3,326 records classified (3,326 OGL, 0 redacted). Registry update N/A (no CRB entry exists) |
| 2.0.7 | `apg-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 641 records classified (641 OGL, 0 redacted) |
| 2.0.8 | `acg-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 423 records classified (422 OGL, 1 PI-REDACTED — flagged for operator review) |
| 2.0.9 | `beastiary-license-retrofit` | E2 | complete | 2026-07-27T20:36Z | 2026-07-27T20:44Z | 45 records classified (45 OGL, 0 redacted) |
| 2.0.10 | `all-23-books-license-conformance-verify` | E2 | complete | 2026-07-27T21:00Z | 2026-07-27T21:05Z | 23/23 books accounted for, 0 defects (4 corpus-conformant + 19 honest stub gaps). Gates 2.1+ cleared |
| 2.1 | `advanced_race_guide_pre_build` | E2 | pending | — | — | Tier-1; pre-build shape B v1 cache from LST corpus (23 `.lst` files). Operator-gated: failure → operator picks next action |
| 2.1' | `advanced_race_guide_verify` | E2 | pending | — | — | Tier-1; verify pre-built cache against dual-audit gate |
| 3.1 | `advanced_race_guide_parity` | E3 | pending | — | — | Tier-1; PCGen parity baseline against pre-built cache |
| 2.2 | `pathfinder_unchained_pre_build` | E2 | pending | — | — | Tier-1; pre-build shape B v1 cache from LST corpus (11 `.lst` files). Operator-gated |
| 2.2' | `pathfinder_unchained_verify` | E2 | pending | — | — | Tier-1; verify pre-built cache against dual-audit gate |
| 3.2 | `pathfinder_unchained_parity` | E3 | pending | — | — | Tier-1; PCGen parity baseline against pre-built cache |
| (deferred) | — | E3 | — | — | — | **17 deferred future-state books** (Adventurer's Guide, B2-B6, Bonus Bestiary, Horror Adventures, Monster Codex, Mythic Adventures, Occult Adventures, the 6 Tier-2 Ultimate books) are operator-gated on SD-27 closing cleanly. Beginner Box and Core Essentials removed from scope per operator directive 2026-07-27 (redundant to other tomes; will not be brought in). Deferred to SD-28+. The pre-build cycle pattern from E2.1-2.2 is templated and reusable. |
| 4.1-4.5 | (closure) | E4 | pending | — | — | Standard closure epilogue |

## TODO

- (none yet)

## DONE

- **1.1 identifier-audit** (2026-07-27T20:04Z) — full-tree code-identifier scan clean, 0 hits. Receipt:
  `artifacts/epic_1/identifier-audit-cycle_receipt.md`.
- **2.0 label-resolution** (2026-07-27T20:16Z) — operator chose `"SD-27"`; all 21 stubs + registry + the
  v0.6 risks doc reconciled. Receipt: `artifacts/epic_2/label-resolution-cycle_receipt.md`.
- **2.0.5 shape-b-license-stripping-preflight** (2026-07-27T20:28Z) — Shape B v1 schema (additive over
  v0, proven against the real on-disk corpus) + PI-blacklist draft + dual-audit test, 10/10 new tests
  passing (independently re-verified by the orchestrator). Receipt:
  `artifacts/epic_2/2.0.5-shape-b-license-stripping-preflight-cycle_receipt.md`.

## DISCOVERED

- (none yet)

## Open blockers

- ~~**Tier-1 launch-gate dependency:** SD-26's closure PR has not yet landed.~~ **CLEARED 2026-07-27** (corrected same day — an earlier note here said PR #339; that was backwards). SD-26 merged via **PR #338** — `62e7b617` is a confirmed ancestor of `origin/develop`, and the SD-26 package + `src/bin/sd26_gen_core_rulebook_cache.rs` are both present there. PR #339 is a separate, later CG-03 bugfix, unrelated to SD-26 closure. Caveat: SD-26's own `progress.md` on develop still shows its terminal `6.5` row as "awaiting operator merge" — stale paper-trail, not evidence the merge didn't happen. Per `decisions.md §7` + `loop-instruction.md §2` item 1.
- ~~**Bundle label discrepancy:** `SD-27` vs. `SD-27+ (unscheduled)` — operator's lever pull at cycle 2.0.~~ **CLEARED 2026-07-27.** Operator chose `"SD-27"`; resolved across all 21 stubs + registry + SD-26's `decisions.md:102` (already correct) + v0.6's risks doc. Per `decisions.md §2` + `artifacts/epic_2/label-resolution-cycle_receipt.md`.
- **CG-03 inherited baseline:** SD-27's per-book parity baseline asserts "match rate at cycle close," not 9-of-9. Per `forward-scope-register.md §"Class 0.3"` + `decisions.md §10`.
- **v0.6 in-progress overlap:** v0.6 is actively working class/race breadth (Fighter/Wizard/Rogue + 8 remaining CRB classes). SD-27's partition restricts SD-27 cycles to the per-book content paths. Per `scope-draft.md §4`.

- **2.0.6-2.0.9 in-scope book license retrofit** (2026-07-27T20:44Z) — all 4 in-scope books
  retro-fitted in parallel (4,435 total records: 3,326 CRB + 641 APG + 423 ACG + 45 Bestiary; 1 real
  redaction, in ACG). Fixed a shared-test-file staleness (Audit 1's pre-retrofit `license==None`
  assumption) and a real regression in SD-26's own `tests/sd26_cache_core_rulebook.rs` (broke on the
  new `LICENSE.json`), both applied once by the orchestrator after collecting all 4 parallel agents'
  results. `cargo test --workspace --locked`: 4,802 passed / 3 pre-existing environment-dependent
  failures, zero regressions. Receipts: `artifacts/epic_2/2.0.{6,7,8,9}-*-license-retrofit-cycle_receipt.md`.

- **2.0.10 all-23-books-license-conformance-verify** (2026-07-27T21:05Z) — 23/23 books accounted
  for, 0 defects. Gates E2.1+ cleared. Receipt:
  `artifacts/epic_2/2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md`.

## Reporting manifest

The live cycle state is the reporting manifest `sd27_book_pre_build`, not this table — see
`loop-instruction.md §8`. Read it with:

```bash
python3 scripts/sd27-workflow.py status
```

6 items (2 books × pre_build/verify/parity — the license-prep stage is folded into pre_build, per
scope-draft.md §1.2.1; no cycle ever claims a standalone "license" item). This file and the manifest must agree; the
manifest is authoritative for status, this table for narrative.

## Cycle receipts

| Cycle | Receipt path |
|-------|--------------|
| 1.1 identifier-audit | `artifacts/epic_1/identifier-audit-cycle_receipt.md` |
| 2.0 label-resolution | `artifacts/epic_2/label-resolution-cycle_receipt.md` |
| 2.0.5 shape-b-license-stripping-preflight | `artifacts/epic_2/2.0.5-shape-b-license-stripping-preflight-cycle_receipt.md` |
| 2.0.6 crb-license-retrofit | `artifacts/epic_2/2.0.6-crb-license-retrofit-cycle_receipt.md` |
| 2.0.7 apg-license-retrofit | `artifacts/epic_2/2.0.7-apg-license-retrofit-cycle_receipt.md` |
| 2.0.8 acg-license-retrofit | `artifacts/epic_2/2.0.8-acg-license-retrofit-cycle_receipt.md` |
| 2.0.9 beastiary-license-retrofit | `artifacts/epic_2/2.0.9-beastiary-license-retrofit-cycle_receipt.md` |
| 2.0.10 all-23-books-license-conformance-verify | `artifacts/epic_2/2.0.10-all-23-books-license-conformance-verify-cycle_receipt.md` |
