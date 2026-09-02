---
title: Engine handoff — level-up pick lists are never emitted, and slugged ids stick without computing
status: open
severity: feature gap — every rogue talent, rage power and hex is silently skipped at level-up
found_by: backend teammate, v0.8 iteration 2, ticket G-1
found_on: 2026-09-02
owner: the concurrent session that owns repo-root src/
scope_note: repo-root src/ is out of scope (§4.2). Nothing was changed; five permanent tests pin
  the current behaviour so any engine change shows up immediately.
---

# Two findings, and the second is the dangerous one

## Finding 1 — the engine emits no pick lists at all

Scout's audit item 27 said `LevelUpDialog` "ignores the `candidates` array". **It isn't ignoring
anything — the engine never produces any.** `PickList` / `PickCandidate` are constructed *nowhere*
in the engine (`grep -rn "PickList {" src/ --include=*.rs` finds only the type definition; the
per-class `level_up/*.rs` files never touch `pick_from_lists`).

Driven through the real `preview_level_up_at_root` on freshly created characters — Rogue 1→2,
Barbarian 1→2, Witch 1→2, Fighter 1→2, Wizard 1→2, Unchained Rogue 1→2, Rogue 3→4 — **all return
`pickFromLists: []`**. `level_up.rs`'s own doc says why: *"Empty when the underlying candidate
catalog is not yet grounded anywhere in rules_tables::crb."* `PickCategory` has only
Feat / Spell / RagePower — no talent or hex variant exists even as a type.

So a picker built on `enginePlan.pickFromLists` would render nothing. **This corrects the audit.**

## Finding 2 — the channel works, which is what makes it hazardous

There is **no second B13**: the choice channel round-trips cleanly for the engine's own
two-segment ids, verified through the real `level_up_character_at_root` plus reload.

| class | choiceSetId | selectionId | persists | grounds a magnitude |
|---|---|---|---|---|
| Rogue 1→2 | `choice:rogue_talent` | `talent:resiliency` | yes | **yes** |
| Rogue 1→2 | `choice:rogue_talent` | `talent:ledge_walker` | yes | **no** — +0 "recognized" record only |
| Barbarian 1→2 | `choice:barbarian_rage_power` | `rage_power:superstition` | yes | **yes** |
| Barbarian 1→2 | same | `rage_power:powerful_blow` | yes | **no** |
| Witch 1→2 | `choice:witch_hex` | `hex:ward` | yes | **yes** |
| Witch 1→2 | same | `hex:slumber` | yes | **no** (silent) |
| Rogue 1→2 | `choice:rogue_talent` | `Rogue Talent ~ Ledge Walker` (raw corpus key) | **Err**, level-up refused | — |

**The split is per-ID, not per-list: every slugged id persists; only the hand-modelled ones
compute.** Hand-modelled today: **1 of ~74** rogue talents, **1 of ~105** rage powers, **3 of 89**
hexes.

This is the sprint's recurring pattern — *"resolves" is not "computes"* — one step later than
usual. A picker over the pool catalog would let a player choose Ledge Walker, save it successfully,
see it on the sheet, and get **nothing**. That is a worse failure than today's silent skip, because
it looks like it worked. Raw corpus keys at least fail loudly at the store.

## The ask

1. **Emit real `PickList` candidates** for talent / rage-power / hex slots from the pool catalog,
   with slugged ids. The engine already has both the catalog and the slug function.
2. **Make the slug function (or an inversion) public** so the bridge can map corpus keys → engine
   ids without copying the rule — the same ask B-7 §0 raised for the class-choice pools.
3. Ideally, ground more than one talent / one rage power / three hexes, or make an ungrounded
   selection *say* it is ungrounded rather than recording a +0 "recognized" row.

## Why no UI was built

The safe-to-offer set today is a 1-option Rogue picker, a 1-option Barbarian picker and a 3-option
Witch picker. The orchestrator cancelled the ticket on that basis: shipping a picker that computes
for 1 of 74 choices would be the stub-shape this project has refused throughout, and it would be
harder to detect than the empty state it replaced.

## Id grammar, recorded for whoever builds the UI later

`choiceSetId`: `choice:rogue_talent`, then `choice:rogue_talent_2` (level 4), `_3` (6) … `_8` (16);
`choice:barbarian_rage_power`, `_2` (4), `_3` (6) …; `choice:witch_hex` for every hex (one set,
multiple entries). `selectionId`: `talent:<slug>` / `rage_power:<slug>` / `hex:<slug>`, where slug
is the engine's `class_feature_id_slug` of the corpus member name (lowercase, spaces → `_`).
