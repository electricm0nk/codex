# SD-30 — Per-cycle Receipts

This file carries the per-cycle receipt for SD-30. Each cycle appends a
new section with the cycle-id and the operator-readable per-cycle facts.

The supervisor reads this file to verify completion before the next
cycle claim (per `decisions.md §14a` local-file dispatch + `loop-instruction.md`
§"Step 6").

## Cycle 0.0 — Chassis Land (planning-ready)

**Date:** 2026-08-01
**Cycle ID:** `SD30-LAND-1`
**Operator:** Todd Hintzmann (directive 2026-08-01)
**Surface:** this directory (`programs/codex/requirements/SD-30-occult-and-companion-content-ingestion/`)

### What landed

- 13+ file canonical chassis per the spec-domain-bundle-authoring skill (matching SD-22 through SD-29's published shape).
- Per-doctrine amendments per operator directive 2026-08-01:
  - **Decision §13** — `tranche/10` branch, no Hermes board (parallel to SD-28's `tranche/8` and SD-29's `tranche/9`).
  - **Decision §14a** — Hermes board retired, local-file dispatch.
  - **Decision §15** — `0.10.<build>` build version.
  - **Decision §16** — cross-book conflict rule with "recently published takes precident" precedence; the class-grant overlap (Occultist family) is the only exception.
  - **Decision §17** — bulk modifications deferred.
  - **Decision §18** — reach gate is the prime rule (the definition of done; engines only when strictly necessary; rules-as-data with pre-computed values; supersedes §12).
  - **Decision §19** — operator ack-chain recorded.

### Book list (sixteen in scope; four deferred)

Confirmed 2026-08-01:

- **In scope (16)**: Occult Adventures, Horror Adventures, Mythic Adventures, Monster Codex, Book of the Damned ×2 volumes, Inner Sea World Guide, Inner Sea Combat, Inner Sea Faiths, Inner Sea Gods, Inner Sea Magic, Inner Sea Races, Inner Sea Temples, Inner Sea Taverns, Inner Sea Bestiary, Inner Sea Intrigue.
- **Deferred (4)**: NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes Handbook — recorded in `forward-scope-register.md C2.x` as future-acquisition candidates.

### Pre-launch state

| Check | Status |
|-------|--------|
| `kanban.md` exists | PENDING (lands this cycle) |
| Branch `tranche/10` pushed to origin | PENDING (operator action at cycle launch) |
| OAuth credentials valid | PENDING (operator action at cycle launch) |
| Working tree clean | ASSUMED (pre-launch verification) |
| Cycle-0 trap-report + work-inventory for 16 books | PENDING (Epic 2 cycle) |

### Next cycle

The next cycle is Epic 2's pre-flight: create `kanban.md` with the
sixteen per-book epics as ready cards; verify branch + OAuth + tree
state; run cycle-0 trap-report + work-inventory for all sixteen books.
Epic 3-N+ per-book cycles dispatch after.

## Cycle 0.0+1 — Unattended-mode acknowledgment (operator directive 2026-08-01)

**Date:** 2026-08-01
**Cycle ID:** `SD30-LAND-2` (unattended-mode directive landing)
**Operator:** Todd Hintzmann (out of town per directive)
**Surface:** this directory (`docs/release/SD-30-occult-and-companion-content-ingestion/`)

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

The largest content-source surface to date (sixteen books across occult + mythic +
Monster Codex + Inner Sea series + Book of the Damned ×2) is the highest-stakes
unattended-mode target. Most likely operator-decision points:

- **Epic 5 (Mythic Adventures) consumer surface** — record `decision-blocked`
  and route to `forward-scope-register.md C3.1` retrofit per the unattended-mode
  protocol.
- **Epic 3 (Occult Adventures) psychic-discipline consumer surface** — same
  retrofit routing via `C3.2`.
- **Epic 7+ (Inner Sea series) campaign-tool surface** — same retrofit routing
  via `C3.3`.

The deferred books (NPC Codex, Planar Adventures, Occult Origins, Haunted Heroes
Handbook) remain deferred — they are out of scope per the 2026-08-01 absent-book
rule and the unattended-mode protocol does not change that.

---

(c) Per-cycle receipts append below this line as cycles fire.