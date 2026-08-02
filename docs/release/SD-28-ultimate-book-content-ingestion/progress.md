# SD-28 — Per-cycle Receipts

This file carries the per-cycle receipt for SD-28. Each cycle appends a
new section with the cycle-id and the operator-readable per-cycle facts.

The supervisor reads this file to verify completion before the next cycle
claim (per `decisions.md §15a` local-file dispatch + `loop-instruction.md`
§"Step 6").

## Cycle 0.0 — Chassis Land (planning-ready)

**Date:** 2026-08-01
**Cycle ID:** `SD28-LAND-1`
**Operator:** Todd Hintzmann (directive 2026-08-01)
**Surface:** this directory (`programs/codex/requirements/SD-28-ultimate-book-content-ingestion/`)

### What landed

- 12-file canonical chassis per the spec-domain-bundle-authoring skill.
- Per-doctrine amendments per operator directive 2026-08-01:
  - **Decision §13** — seven books confirmed (six Paizo + Dreamscarred Press).
  - **Decision §14** — `tranche/8` branch, no Hermes board.
  - **Decision §15** — `0.8.<build>` build version.
  - **Decision §15a** — Hermes board retired, local-file dispatch.
  - **Decision §16** — cross-book conflict rule (newer = doctrine).
  - **Decision §17** — Dreamscarred Press license gate.
  - **Decision §17a** — bulk modifications deferred.
  - **Decision §18** — reach gate is the definition of done; engines only when strictly necessary; rules-as-data with pre-computed values (supersedes §12).
  - **Decision §19** — operator ack-chain recorded.

### Pre-launch state

| Check | Status |
|-------|--------|
| `kanban.md` exists | DONE 2026-08-01 — kanban.md present with 12 cards (epics 1–12), dispatch-ordered |
| Branch `tranche/8` pushed to origin | DONE 2026-08-01 — `git branch -r --list 'origin/tranche/8'` → `origin/tranche/8` |
| OAuth credentials valid | PENDING (operator action at cycle launch) |
| Working tree clean | ASSUMED (pre-launch verification) |
| Dreamscarred Press license precheck | PENDING (Epic 9 cycle 0) |

### Next cycle

The next cycle is Epic 2's pre-flight: launch the cycle on `kanban.md`; verify branch + OAuth + tree state. This is
the local-file counterpart to the prior Hermes-board readiness check.

---

**(c) Per-cycle receipts append below this line as cycles fire.**

**2026-08-01 pre-launch readiness pass (operator-side):** branch tip at launch prep: 4d75856c on `tranche/8`. Launch-readiness audit applied fixes to loop-instruction.md (claim step, merged append steps, corpus shape notes, unattended item 4, receipt path), kanban.md (dispatch ordering + Depends-on), scope-draft.md (seven-book reconciliation), and v06_work_inventory (ultimate_psionics roster entry). Eligibility rule 'progress.md corresponds to the operator-pinned branch tip' is satisfied by this entry.

---

## Cycle 0.1 — Workspace cleanup (cross-bundle, applied during SD-28 land)

**Date:** 2026-08-01
**Cycle ID:** `SD28-CLEANUP-1`
**Surface:** `programs/codex/requirements/SD-27-future-state-book-content-ingestion/`

### What landed

The workspace SD-27 directory was deleted on operator directive 2026-08-01
(the move-not-copy doctrine was honored by removing the workspace copy that
had been retained past the prior publish). The canonical SD-27 chassis
remains at `docs/release/SD-27-future-state-book-content-ingestion/`. SD-27
published docs were updated to reflect the workspace removal (Decisions §6
on the publish mechanic + technical-requirements.md line 5 + the
cross-bundle-findings-2026-07-30.md artifact).

Per SD-27's `decisions.md §18`, the underlying conflict (§19.1 "content-only
scope vs. the reach gate") is resolved by SD-28's `decisions.md §18` (reach
gate is the definition of done; engines permitted only when strictly
necessary; no dice-rolling).

## Cycle 0.0+1 — Unattended-mode acknowledgment (operator directive 2026-08-01)

**Date:** 2026-08-01
**Cycle ID:** `SD28-LAND-2` (unattended-mode directive landing)
**Operator:** Todd Hintzmann (out of town per directive)
**Surface:** this directory (`docs/release/SD-28-ultimate-book-content-ingestion/`)

### What landed

The operator is out of town and may not see the harness's output for days. Per
operator directive 2026-08-01, this bundle operates in **unattended mode**.

Cycles MUST NOT pause to ask the operator questions. The operator's verbatim:

> "include instructions to all 3 that indicate they will be running in unnattended
> mode since i will be out of town while this runs. They may not stop to ask
> questions - it might be days before i notice."

The doctrine is mirrored across three files:

- `loop-instruction.md` §"OPERATING METHOD" sub-callout (cycle supervisor reads it first).
- `decisions.md` Decision §21 (load-bearing doctrine entry).
- `progress.md` Cycle 0.0+1 (this entry — per-cycle receipt confirms the operator-on-record).

The receipt chain is the operator's after-return review surface. When the
operator returns, the cycle receipts in this file carry the per-cycle decisions
that the harness made on its behalf.

### Operating protocol summary (mirror of `decisions.md §21`)

1. Default-and-flag, not ask.
2. No `clarify` tool calls.
3. Blockers are recorded, not raised.
4. `decision-blocked` IS allowed.
5. Closure is a goal, not a stop signal.

### Bundle-specific unattended-mode notes

The Dreamscarred Press tier (Epic 9) is the most likely place where the cycle
will want an operator decision. Per the unattended-mode protocol:

- **Dreamscarred Press license audit (Epic 9 cycle 0)** — if the trap-report
  surfaces records not matching open-content tier, record the drops in
  `artifacts/upsi-license-drops.md` and proceed. Do not pause to ask.
- **Epic 9 dispatch decision** — if cycle 0 finds license-conformance gaps
  that would require Dreamscarred Press-specific trap patterns, record
  `decision-blocked` in this file and proceed with the safe default (drop
  non-conforming records; carry the gap into the next cycle-batch).
- **Epic 1 Identifier Cleanup finding** — if the audit surfaces new forbidden
  patterns specific to the third-party tier, record the finding and proceed.

---

(c) Per-cycle receipts append below this line as cycles fire.
