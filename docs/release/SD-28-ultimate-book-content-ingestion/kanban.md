# SD-28 — Local-file Work Queue (replaces Hermes board `codex-tranche-8`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-28's
work queue is a local-file Markdown table. The supervisor reads this file
at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

## Status legend

- `READY` — not yet claimed. Cycle can pick up once every `Depends-on` card is `COMPLETE`.
- `IN-FLIGHT` — claimed by a cycle, in progress. Other cycles must wait.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.

**Dispatch tiebreak:** next card = lowest `Order` among `READY` cards whose
every `Depends-on` card is `COMPLETE`. A card whose `Depends-on` is not
fully `COMPLETE` is not eligible regardless of `Order` or `Status`.

## Cards (one row per per-book epic cycle-batch), in dispatch order

| Order | ID | Status | Book | Cycle-type | Depends-on | Claimed-by | Claimed-at | Cycle-id |
|---|----|--------|------|-----------|------------|------------|------------|----------|
| 1 | `epic-1-identifier` | COMPLETE | Identifier Cleanup | identifier-discipline audit pass | none | sd28-epic1 | 2026-08-02T02:25:56Z | SD28-E1-F1-001 |
| 2 | `epic-2-prelaunch` | COMPLETE | Operator Pre-Launch | local-file dispatch readiness + license precheck | `epic-1-identifier` | sd28-epic2 | 2026-08-01T00:00:00Z | SD28-E2-F1-001 |
| 3 | `epic-3-uc` | READY | Ultimate Combat | per-class / per-chooser | `epic-2-prelaunch` | — | — | — |
| 4 | `epic-4-um` | READY | Ultimate Magic | per-class / per-spell-subsystem | `epic-2-prelaunch` | — | — | — |
| 5 | `epic-5-ue` | READY | Ultimate Equipment | per-equipment-entry | `epic-2-prelaunch` | — | — | — |
| 6 | `epic-6-ui` | READY | Ultimate Intrigue | per-class / per-social-rule | `epic-2-prelaunch` | — | — | — |
| 7 | `epic-7-ucam` | READY | Ultimate Campaign | per-system-subsystem | `epic-2-prelaunch` | — | — | — |
| 8 | `epic-8-uw` | READY | Ultimate Wilderness | per-class / per-Companion-rule | `epic-2-prelaunch` | — | — | — |
| 9 | `epic-9-upsi` | READY | Ultimate Psionics (Dreamscarred Press tier) | per-class / per-power, license-gated | `epic-2-prelaunch` | — | — | — |
| 10 | `epic-11-version` | COMPLETE | Build Version Numbering | first concrete value `0.8.<build>` | `epic-1-identifier` | sd28-epic11 | 2026-08-02T03:00:00Z | SD28-E11-F1-001 |
| 11 | `epic-12-code-review` | COMPLETE | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §26`) | `epic-3-uc`, `epic-4-um`, `epic-5-ue`, `epic-6-ui`, `epic-7-ucam`, `epic-8-uw`, `epic-9-upsi`, `epic-11-version` | sd28-epic12 | 2026-08-02T00:00:00Z | SD28-E12-F1-001 |
| 12 | `epic-13-calibration` | COMPLETE | Ultimate Campaign (23 units) | cost calibration — one small book end-to-end to 100% proven | `epic-2-prelaunch` | epic-13-calibration | 2026-08-03T00:00:00Z | SD28-E13-F1-001 |
| 13 | `epic-14-harness` | COMPLETE | Observation harness (spell + equipment) | consumer-probe widening; +173 grounded (equipment only -- spell probe reverted after review, no wired spell-magnitude consumer exists); 3877 remain ingested-magnitude, named in `artifacts/e14-harness-widening.md` | `epic-13-calibration` | epic-14-harness | 2026-08-06T22:29:15Z | SD28-E14-F1-F2-F3-001 |
| 14 | `epic-15-unknown-sweep` | READY | Classification sweep | dispose all 4172 `unknown` units | `epic-13-calibration` | — | — | — |
| 15 | `epic-16-backfill` | READY | `not-ingested` backfill | 8492 real gaps inside started books | `epic-13-calibration`, `epic-15-unknown-sweep` | — | — | — |
| 16 | `epic-17-punchained` | READY | Pathfinder Unchained | per-book completion — gap 830 | `epic-14-harness`, `epic-15-unknown-sweep`, `epic-16-backfill` | — | — | — |
| 17 | `epic-18-bestiary` | READY | Bestiary | per-book completion — gap 985 | `epic-17-punchained` | — | — | — |
| 18 | `epic-19-arg` | READY | Advanced Race Guide | per-book completion — gap 2063 | `epic-17-punchained` | — | — | — |
| 19 | `epic-20-acg` | READY | Advanced Class Guide | per-book completion — gap 2523 | `epic-17-punchained` | — | — | — |
| 20 | `epic-21-ce` | READY | Core Essentials | per-book completion — gap 2593 | `epic-17-punchained` | — | — | — |
| 21 | `epic-22-apg` | READY | Advanced Player's Guide | per-book completion — gap 2948 | `epic-17-punchained` | — | — | — |
| 22 | `epic-23-crb` | READY | Core Rulebook | per-book completion — gap 4804 (3062 of it hard-gated on `epic-14-harness`) | `epic-17-punchained` | — | — | — |
| 23 | `epic-24-ui-complete` | IN-FLIGHT | Ultimate Intrigue | slices 1-2 landed (104 feats + 101 spells + 98 equipment, no deferrals); classifier fix landed program-wide (1,861-unit `class_feature` `proven`→`not-ingested` correction, `decisions.md §43`); `class_feature`'s real remaining gap blocked on two named engine epics (Vigilante chassis, archetype-swap — see `decisions.md §42`); `race_trait` traced to 0 closable units | `epic-14-harness`, `epic-15-unknown-sweep`, `epic-16-backfill` | epic-24-ultimate-intrigue | 2026-08-08T06:47:00-04:00 | SD28-E24-F3-001 |
| 24 | `epic-25-ue-complete` | IN-FLIGHT | Ultimate Equipment | slice 1 (1,549-record equipment catalog, 55 real cross-book collisions excluded) landed and verified; re-derived real new content is 1,549 of 1,615 declared, not the whole book — see `decisions.md §44` | `epic-14-harness`, `epic-24-ui-complete` | epic-24-ultimate-intrigue | 2026-08-08T09:22:00-04:00 | SD28-E25-F1-001 |
| 25 | `epic-26-uw-complete` | IN-FLIGHT | Ultimate Wilderness | slice 1 (135-record feat catalog, 1 cross-book collision excluded) landed and verified; cost-model prediction confirmed (one unplanned finding, `Ferocious Beast`'s orphaned formula tail — see `decisions.md §45`); desktop clippy ceiling now fully spent (7/7) | `epic-14-harness`, `epic-24-ui-complete` | epic-24-ultimate-intrigue | 2026-08-08T12:21:00-04:00 | SD28-E26-F1-001 |
| 26 | `epic-27-uc-complete` | IN-FLIGHT | Ultimate Combat | slice 1 (261-record feat catalog, 2 textless exclusions, 1 record recovered from an invisible `.MOD` row) landed and verified; cost-model prediction broke 4-for-4 pattern with 3 unplanned findings, not 1 — see `decisions.md §46`/`§47`; nine-book `.MOD`-recovery sweep found a live sibling gap in APG (`Deadly Aim`), recorded as an `OPEN_FINDINGS`-shaped handoff, not fixed this cycle | `epic-14-harness`, `epic-24-ui-complete` | epic-24-ultimate-intrigue | 2026-08-08T14:56:00-04:00 | SD28-E27-F1-001 |
| 27 | `epic-28-um-complete` | IN-FLIGHT | Ultimate Magic | slice 1 (144-record feat catalog, 3 auto-grant exclusions, 4 textless-but-real records kept via a new `effect` field) landed and verified; raw-syntax leak-join bug caught by two guards and fixed pre-commit, never shipped -- see `decisions.md §49`; `.MOD`/text-shape hazard triad now complete across three books | `epic-14-harness`, `epic-24-ui-complete` | epic-24-ultimate-intrigue | 2026-08-08T17:20:00-04:00 | SD28-E28-F1-001 |
| 28 | `epic-29-upsi-complete` | IN-FLIGHT | Ultimate Psionics | slice 1 (221-record feat catalog, 1 source-disabled exclusion, 1 cross-book collision excluded) landed and verified; licence precheck complete -- Dreamscarred Press's ISOGL:YES/OGL.txt declaration checked structurally, no anomaly found; DESC:-is-complete book convention required zero textless-stub exclusions; last Ultimate book -- see `decisions.md §50` | `epic-14-harness`, `epic-24-ui-complete`, `epic-2-prelaunch` | epic-24-ultimate-intrigue | 2026-08-08T20:23:00-04:00 | SD28-E29-F1-001 |
| 29 | `epic-30-integrity` | READY | Completion Integrity Gate | anti-gaming audit + final 32061/32061 count | `epic-12-code-review`, `epic-13-calibration`, `epic-14-harness`, `epic-15-unknown-sweep`, `epic-16-backfill`, `epic-17-punchained`, `epic-18-bestiary`, `epic-19-arg`, `epic-20-acg`, `epic-21-ce`, `epic-22-apg`, `epic-23-crb`, `epic-24-ui-complete`, `epic-25-ue-complete`, `epic-26-uw-complete`, `epic-27-uc-complete`, `epic-28-um-complete`, `epic-29-upsi-complete` | — | — | — |
| 30 | `epic-10-closure` | READY | Closure Epilogue | tranche promotion PR | `epic-1-identifier`, `epic-2-prelaunch`, `epic-3-uc`, `epic-4-um`, `epic-5-ue`, `epic-6-ui`, `epic-7-ucam`, `epic-8-uw`, `epic-9-upsi`, `epic-11-version`, `epic-12-code-review`, `epic-30-integrity` (everything else) | — | — | — |
| 31 | `epic-31-spell-wiring` | COMPLETE | Spell magnitude → player surface | wire `spellbook::compute_spellbook_coverage` into `pf1_adapter::resolve_unified_pilot_snapshot`/`PilotSnapshot`/desktop Spells tab; closes `epic-14-harness`'s "third, disconnected twin" finding | `epic-14-harness` | epic-31-spell-wiring | 2026-08-07T00:00:00Z | SD28-E31-F1-001 |
| 32 | `epic-32-archetype-swap` | IN-FLIGHT | Archetype-swap mechanism | piece 1 (UPsi 15-record proof table, `src/rules_core/rules_tables/ultimate_psionics/archetype_tables.rs`) landed and verified; two populations sized (930 tier-1 selections, 4,550 tier-2 mechanics -- 4.9x tier-1); `pilot_compute.rs` integration blocked on an explicit scope decision (task #67 reversal) -- see `decisions.md §51`, `forward-scope-register.md §C4.8`. Note: this card was informally called "epic-30" during scoping before the `epic-30-integrity` id collision was caught; renumbered here | `epic-14-harness` | epic-24-ultimate-intrigue | 2026-08-08T22:40:00-04:00 | SD28-E30-F1-001 |

## Completion-epic cards (added 2026-08-02, operator 100%-proven directive)

Cards `epic-13-calibration` through `epic-30-integrity` implement
`epic-breakdown.md §"Completion epics (E13–E30)"` and `decisions.md §32`.
Total gap across the 13 books: **29,161 units** (32,061 target − 2,900 proven,
re-derived 2026-08-02T11:50:31Z).

Two constraints govern dispatch of these cards specifically:

- **`epic-13-calibration` runs before every other completion card**, and no
  duration may be asserted for any later card until its receipt
  (`artifacts/e13-cost-calibration.md`) reports a measured cost per unit.
- **`epic-14-harness` and `epic-15-unknown-sweep` both edit
  `src/bin/v06_work_inventory.rs`.** They may be claimed concurrently only in
  separate worktrees with separate `CARGO_TARGET_DIR`s (`decisions.md §29`).
  Two cycles holding uncommitted work in the same tree is a stop condition.

Cards `epic-3-uc` … `epic-9-upsi` are **not retired**. Their definition of done
is raised from "a reach claim exists" to "100% proven for this book," and each
is superseded book-for-book by its completion card (`epic-7-ucam` by
`epic-13-calibration`; the other six by `epic-24`…`epic-29`).

## Cycle claims (cycle-supervisor protocol)

When a cycle claims a card:

1. Edit the card's `Status` to `IN-FLIGHT`.
2. Edit `Claimed-by` to the cycle's harness identifier.
3. Edit `Claimed-at` to the cycle's ISO-8601 timestamp.
4. Edit `Cycle-id` to the cycle's audit ID (e.g., `SD28-E3-F1-001`).
5. Append the cycle's per-cycle facts to `progress.md` (write to
   `progress.md` after writing the kanban claim; the supervisor reads
   progress.md to verify the prior cycle complete before claiming the
   next).
6. On cycle completion, edit `Status` to `COMPLETE` and append the
   completion receipt to `progress.md`.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle
dispatch honors the post-edit state.

## Resolution to operator directives

This file is the load-bearing replacement for the Hermes `codex-tranche-8`
board (operator-confirmed 2026-08-01). When a Hermes board card is
referenced from prior doctrine (`decisions.md`, `scope-draft.md`,
`loop-instruction.md`, etc.), the reference resolves to a `kanban.md`
card id at the time of cycle dispatch.
