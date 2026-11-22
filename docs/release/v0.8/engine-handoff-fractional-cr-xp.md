---
title: Engine handoff — every fractional-CR monster is misrated
status: open
severity: wrong-but-safe rating on ~30% of the monster catalog; 81 monsters materially misrated
found_by: backend teammate, v0.8 encounter-generator strawman, ticket E-1
found_on: 2026-09-02
owner: the concurrent session that owns repo-root src/
scope_note: repo-root src/ is out of scope for this sprint (agent-team-ui.md §4.2). Nothing was
  changed in the engine; the bridge discloses the limit per-encounter instead.
---

# Fractional-CR monsters are floored to CR 1 and rated as 400 XP

## The defect

`src/rules_core/encounters.rs`, `xp_for_cr` (~line 180):

    m.challenge_rating.round() as i32   // then .max(1)

A CR 1/8, 1/4 or 1/3 creature rounds to **0** and is floored to **CR 1 = 400 XP**. A CR 1/2
creature rounds to 1 — also 400 XP. The rulebook's sub-CR-1 XP values are not grounded at all, and
the engine's own doc says so: *"CR below 1 is floored to CR 1 … extending to it is deferred to
whichever future cycle first needs it."* This cycle needed it.

**Concrete consequence**, pinned by `a_fractional_cr_is_flagged_and_its_engine_rating_is_disclosed`:
four CR 1/4 creatures against four level-1 PCs rate **Deadly** (4 × 400 = 1,600 XP → EL 5 vs APL 1).
By the rulebook that is roughly a **Medium** fight. The error overstates threat, so it is at least
in the safe direction — a DM under-challenges rather than kills the party — but it is precisely the
"quietly wrong tier at the table" failure the strawman was built to avoid.

## How much of the catalog this touches

Measured by a temporary probe over `list_monster_catalog()` (deleted after measuring):

| Figure | Count |
|---|---|
| Monsters in the catalog | 1,243 |
| **Below CR 1 — materially misrated** | **81** (dire rat 0.25, badger 0.5, sootwing bat 0.5 …) |
| Above CR 10 — extrapolated, not re-verified | 289 (Tarrasque at CR 25) |
| Fractional CR | 80 |

So roughly **30% of the catalog sits outside the verified CR 1–10 table**, and every low-level
vermin/animal encounter — the bread and butter of levels 1–3 — is misrated.

## The recommended fix is small

**Ground the sub-CR-1 XP sub-table. It is six rows** (CR 1/8, 1/6, 1/4, 1/3, 1/2 and the CR 0 case)
and it alone fixes all 81. That is a far better return than any other engine work this sprint
surfaced.

## Second, weaker finding: the CR 11+ extrapolation

Above CR 10 the engine doubles every 2 CR (CR 11 = 12,800 … CR 25 = 1,638,400). `backend` believes
this matches the CRB table as published but **could not verify it against any in-repo source**, and
the engine's own doc says it is "not independently re-verified." It is therefore **flagged, not
asserted correct** — the bridge discloses it as extrapolated and the UI shows that per monster.
Worth verifying against the book when someone has it open.

## Third: no party-size APL adjustment

The engine applies no adjustment for parties of fewer than 4 or more than 5 characters, which the
CRB does specify. `backend` flagged this as *a rule it knows, not one the engine states*, and
deliberately did not author it in the bridge. Whether it belongs in the engine is a question for
the engine session.

## What the bridge does meanwhile (no engine change)

`rate_encounter` discloses rather than corrects. Per monster it returns `crAsRated`,
`outsideVerifiedRange` and a `reason` worded from the engine's own doc statements, plus a
top-level `anyOutsideVerifiedRange`, `verifiedCrRange: [1,10]`, and a `difficultyScale` describing
the four-tier collapse. **`crAsRated` is read back from the engine** — each monster is rated alone,
and a lone monster's Encounter Level *is* the CR the engine rated it at — so no copy of the
rounding/flooring rule lives in the bridge. The UI shows the caveat on the encounter itself.

Nothing here is corrected in the bridge, deliberately: a bridge that silently repaired the engine's
arithmetic would hide the defect and drift from it.
