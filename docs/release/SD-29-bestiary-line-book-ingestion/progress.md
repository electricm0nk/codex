# SD-29 — Per-cycle Receipts

This file carries the per-cycle receipt for SD-29. Each cycle appends a
new section with the cycle-id and the operator-readable per-cycle facts.

The supervisor reads this file to verify completion before the next cycle
claim (per `decisions.md §14a` local-file dispatch + `loop-instruction.md`
§"Step 6").

## Cycle 0.0 — Chassis Land (planning-ready)

**Date:** 2026-08-01
**Cycle ID:** `SD29-LAND-1`
**Operator:** Todd Hintzmann (directive 2026-08-01)
**Surface:** this directory (`docs/release/SD-29-bestiary-line-book-ingestion/`)

### What landed

- 15-file canonical chassis per the spec-domain-bundle-authoring skill (matching SD-22 through SD-28's published shape).
- Per-doctrine amendments per operator directive 2026-08-01:
  - **Decision §13** — `tranche/9` branch, no Hermes board (parallel to SD-28's `tranche/8`).
  - **Decision §14** — `0.9.<build>` build version.
  - **Decision §14a** — Hermes board retired, local-file dispatch.
  - **Decision §15** — four-bestiary list confirmed (B2-B5) with cycle-0 shape gating.
  - **Decision §16** — cross-book conflict rule (newer = doctrine).
  - **Decision §17** — bulk modifications deferred.
  - **Decision §18** — Bestiary 5 shape-resolution (player-options cycles, not monster-block).
  - **Decision §19** — reach gate is the definition of done; engines only when strictly necessary; rules-as-data with pre-computed values (supersedes §12).
  - **Decision §20** — operator ack-chain recorded.

### Pre-launch state

| Check | Status |
|-------|--------|
| `kanban.md` exists | DONE 2026-08-02 — kanban.md present, 13 dispatch-ordered cards (epics 1–13) |
| Branch `tranche/9` pushed to origin | PENDING (operator action at cycle launch) |
| OAuth credentials valid | PENDING (operator action at cycle launch) |
| Working tree clean | ASSUMED (pre-launch verification) |
| Cycle-0 trap-report + work-inventory for B2-B5 | PENDING (Epic 2 cycle) |

### Next cycle

The next cycle is Epic 2's pre-flight: claim from the existing 13-card `kanban.md` board (epics 1-13 as dispatch-ordered cards); verify branch + OAuth + tree state; run cycle-0 trap-report + work-inventory for all four bestiaries (B5's shape finding determines Epic 6's cycle shape).

## Cycle 0.0+1 — Unattended-mode acknowledgment (operator directive 2026-08-01)

**Date:** 2026-08-01
**Cycle ID:** `SD29-LAND-2` (unattended-mode directive landing)
**Operator:** Todd Hintzmann (out of town per directive)
**Surface:** this directory (`docs/release/SD-29-bestiary-line-book-ingestion/`)

### What landed

The operator is out of town and may not see the harness's output for days. Per
operator directive 2026-08-01, this bundle operates in **unattended mode**.

Cycles MUST NOT pause to ask the operator questions. The operator's verbatim:

> "include instructions to all 3 that indicate they will be running in unnattended
> mode since i will be out of town while this runs. They may not stop to ask
> questions - it might be days before i notice."

The doctrine is mirrored across three files:

- `loop-instruction.md` §"OPERATING METHOD" sub-callout (cycle supervisor reads it first).
- `decisions.md` Decision §22 (load-bearing doctrine entry).
- `progress.md` Cycle 0.0+1 (this entry — per-cycle receipt confirms the operator-on-record).

The receipt chain is the operator's after-return review surface. When the
operator returns, the cycle receipts in this file carry the per-cycle decisions
that the harness made on its behalf.

### Operating protocol summary (mirror of `decisions.md §22`)

1. Default-and-flag, not ask.
2. No `clarify` tool calls.
3. Blockers are recorded, not raised.
4. `decision-blocked` IS allowed.
5. Closure is a goal, not a stop signal.

### Bundle-specific unattended-mode notes

Epic 7 (DM Toolkit extension, consumer of Bestiary 2-5 records) is the most
likely place where the cycle will want an operator decision. The unattended-mode
protocol routes Epic 7's resolution through `successor-forward-scope-register.md C3.1`
retrofit — record `decision-blocked` in this file with the recorded reason and
proceed on the safe default (retrofit; Epic 7 lands inside SD-29's epic
structure only if Epic 3-6 per-book cycles complete with reach-gate claims and
the toolkit-extension scope is operator-pinned in scope before Epic 7 fires).

## Cycle 0.0+2 — Kind-lane re-cut (operator directive 2026-08-10)

**Date:** 2026-08-10
**Cycle ID:** `SD29-LAND-3`
**Operator:** Todd Hintzmann (directive: "The SDs are our bodies of work. If our plan for SD-29
needs to be completely rescoped to address something else, then that is where we need to make
those updates. After the PR is merged, we will start SD-29 in tranche/9. That needs to be defined
and recorded.")
**Surface:** this directory (`docs/release/SD-29-bestiary-line-book-ingestion/`) and
`docs/release/corpus-work-channels.md`.

### What landed

`decisions.md §36` (2026-08-10, same day) had already ruled SD-29 partitions by kind, not by book,
following the `file_kind()` correction that moved Bestiary 1 from 620 `race_trait` to 21
`race_trait` + 523 `monster_ability`. The ruling had not been executed — `epic-breakdown.md` still
carried Epics 3-6/11-13 as seven per-book epics. `decisions.md §37` (this cycle) executes it:

- `epic-breakdown.md` rewritten in full: 11 epics — Epic 3 (new, Provenance Gate: PI-screening)
  plus Epics 4-7 (kind lanes: Monster/Monster-Ability chassis [2,159 units, pilot-then-extend on
  Bonus Bestiary], Race-Trait [1,124 units, defect-fix-alongside], Companion [275 units], Residual
  proven-path content [203 units, excludes `class_feature`'s 90 units]) replace the retired
  per-book Epics 3-6/11-13. Epic 7 (DM Toolkit) renumbered Epic 8; Epic 8 (Closure) renumbered
  Epic 11; Epic 9 (Build Version) and Epic 10 (Code Review) keep their numbers.
- `kanban.md` cards rewritten to match: lane cards (with the monster lane split into pilot +
  extend), renumbered dependency chain.
- `acceptance-and-verification.md`: new AT-29-003a (provenance gate); AT-29-008 repurposed
  (retired the Bestiary-5-specific fallback-cycle-type question, which the lane structure no
  longer needs); every Epic cross-reference renumbered.
- `scope-draft.md`, `technical-requirements.md`, `technical-design.md`, `loop-instruction.md`,
  `risks-and-open-questions.md`, `successor-forward-scope-register.md`, `release-notes.md`,
  `README.md`: every per-book epic cross-reference updated to the lane structure; the
  "SUPERSEDED IN PART" banner on `epic-breakdown.md` (which had pointed at this exact gap since
  2026-08-01) is removed, since the re-cut it called for has landed.
- Provenance (`../corpus-work-channels.md §6`, "Blocking before the first channel runs"):
  resolved for OGL/attribution — `docs/governance/license-matrix.md` (commit `314a7ad9`) already
  establishes it for all seven SD-29 books. **Not resolved** for PI-screening — the matrix found
  zero PI-screening anywhere in `rules_tables/*.rs` (the pipeline every SD-29 lane writes into)
  and three real leaks in other bundles' tables of the same pipeline. Epic 3 gates every lane on a
  per-lane PI-blacklist sweep before that lane's first content commit.
- `../corpus-work-channels.md` committed (was untracked) so the channel analysis this re-cut
  executes is not lost.
- Correction recorded: `corpus-work-channels.md §4`'s own "SD-29's 7 books" kind totals
  (`monster_ability` 1,869 / `monster` 1,143 / `race_trait` 1,145 / `companion` 334) are the
  **eight**-book sum including Bestiary 1, not SD-29's actual seven-book sum (1,346 / 813 / 1,124
  / 275 respectively). See `decisions.md §37.0.1`.

### Verification

- Every figure in `decisions.md §37` re-derived via `python3` against `docs/work-inventory.json`;
  commands included inline.
- `git status --porcelain` checked before staging; only the explicit paths this cycle touched were
  staged (docs-only, no `.rs`/`.tsx`/scripts).

### Next cycle

Epic 1 (Identifier Cleanup) is still the first cycle to fire once SD-29 launches on `tranche/9`
(cut from the post-SD-28 tip, per `decisions.md §34`). This re-cut is a docs-only change; no
tranche/9 branch exists yet and no lane cycle has run.

## Pre-launch readiness audit (2026-08-02)

**2026-08-02 pre-launch readiness pass (operator-side):** branch tip at audit: b63cda4e on tranche/8 (tranche/9 cut deferred to SD-29 launch per decisions.md §34). Scope operator-pinned to the 7-book cut (adds bestiary_6, bonus_bestiary, monster_codex — Epics 11–13). Launch-readiness fixes applied across the package; decisions §21–§34 landed. Sequential launch after SD-28 closure.

---

(c) Per-cycle receipts append below this line as cycles fire.
