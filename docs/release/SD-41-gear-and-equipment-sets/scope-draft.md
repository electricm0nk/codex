---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready — the request and the nine answers are the operator's (artifacts/cycle_0/operator-brief-2026-11-12.md); the author's defaults are marked and name their revisit
---

# SD-41 Scope Draft — Gear and Equipment Sets

## 1. The request (operator, 2026-11-12, verbatim)

> "on the gear screen, we need to make some adjustments. We need to borrow from the type of interface that
> pcgen is using - use this as a reference point, but do not copy the code. Functionally, it is as follows:
> It uses equipment (we can call it gear, that is fine) but that screen has a split panel. The left panel
> shows all the gear and can be filtered by equipped, not equipped, and all. The right gear shows a listing
> of all the equipment slots: Head, Hands, Torso, Legs, Shield, Equipped, Carried, and Not Carried. When
> someone buys something from the market it needs to default to carried. When someone equips something, for
> instance a sheild, that will take up the shield spot and will impact the AC. If it is not in the slot, it
> doesn't impact the AC. If something is carried, the encumberance is impacted by the weight. If it is not
> carried, then it doesn't count against the weight. The Equipment set allows for the creation of custom
> sets. One might have a default set, but then maybe you have a 'formalwear' set - one doesn't normally wear
> a suit of armor to a fancy dinner with the lords of the castle. The equipment view on the left side shows
> the item, it's weight, and location. Location is the slot. This might be the Primary Hand, or it might be
> not carried. A user can move items between the different locations by dragging them. Some items are
> containers: I might equip a backpack and put items in the backpack. I might even put items in a pouch and
> then the pouch in the backpack to keep things organized. This all needs to hook in with our ability to
> customize items. I use a custom named pouch called food, and then put food in food pouch. I might have
> another that is called camping and i put a bedroll and such in that thing."

The nine answers of the same day are in `decisions.md §3`.

What the code does today (checked on `tranche-20` @ `f7f85f3d24`, 2026-11-12; every figure in
`artifacts/epic_0/census.txt` with its command):

- **Three states, no locations.** A copy is `EquippedActive`, `SelectedInactive` (carried) or `Absent`
  (left behind) — `ActiveState` in `src/rules_core/character_input.rs`, saved as `equipment=item:<id>:<state>`
  lines in `authoritative_character_input.txt` (`local_store.rs:458`). The slot of a worn copy is derived
  at list time from the record (`body_slot_of_item`, `worn_places`), never stored. 97 struct literals build
  an `EquipmentSelection` (`grep -rnF 'EquipmentSelection {' … | awk 'END{print NR}'` → 97): the record
  shape is not where SD-41 adds a field.
- **The Gear tab's Equipment panel** (`EquipmentPanel.tsx`, 194 lines) shows three flat lists — Worn (by
  slot, 18 `WORN_SLOTS`), Carried, Left behind — with one button per move (Equip / Stow / Leave behind /
  Pick up) through `set_equipment_active_state`. No weight per row, no filter, no drag
  (`grep -c 'draggable\|onDrop' EquipmentPanel.tsx` → 0), no sets.
- **Buying wears the item** when its slot is free: `CharacterSheet.tsx:3295` requests `'EquippedActive'`
  and `bought_copy_placement` falls back to carried only on a conflict.
- **The engine already keys on the three states:** AC and every worn bonus count `EquippedActive` copies
  only (`equipment_effects.rs:207`); weight counts everything but `Absent` (`encumbrance.rs:408`); the
  Weapons rows exist for `EquippedActive` weapons only (`damage_total.rs:820`).
- **Containers do not exist.** No code reads `CONTAINS` or a capacity (`grep … | awk` → 0). The data
  holds them: 232 sheet-rule equipment records carry the `Container` tag; 138 corpus records carry a
  `CONTAINS` token, 70 with a weight capacity, 43 unlimited, 25 weight-reduction kinds.
- **Custom items** (`character_custom.rs`) have `slot`, `worn`, `left_behind` and bonuses; no weight, no
  base item, so a custom "Food" pouch neither weighs nor holds anything.
- **The printed Inventory** (`print/blocks/gear.rs::inventory`) lists every copy with State (Worn (slot) /
  Carried / Left behind) and weight, then the totals and limits; SD-40's E3.2 (waiting) re-lays it as
  Worn / Carried / Left behind.

## 2. Definition of Done

SD-41 is done when every row below holds on `tranche-21` and the operator has signed E4.G.

1. **Locations.** Every held copy has a location in the active set: one of the 18 body slots (`WORN_SLOTS`,
   the app's full Pathfinder list), **Equipped** (worn, no slot), **Carried**, **Not Carried**, or **inside
   a container copy**. The location is what the screen's left list shows beside the item and its weight,
   what the right panel groups by, and what the print lists by. (`decisions.md §5`.)
2. **Engine-owned numbers, unchanged engine.** AC counts a copy only when it is in a slot or Equipped;
   weight counts a copy only when it is Carried, in a slot, Equipped, or inside a container that is one of
   those; a Not Carried container's contents weigh nothing. These are the engine's existing rules over
   `active_state`; SD-41 **projects** the active set into `active_state` at every save (`decisions.md §6`)
   and adds no second computation (AGENTS.md twin rule; `decisions.md §16`).
3. **Containers.** A copy whose record carries the `Container` tag can hold copies; nesting allowed; a
   cycle is refused. The engine reports each container's contents weight and stated capacity on the load
   response; the screen and the print show them with a warning (never a block) when over. (`decisions.md §7`.)
4. **Equipment sets.** All sets share the one item list; a set stores only locations; one set is active
   and drives the sheet, AC, encumbrance, the Weapons rows and the print. Create, rename, delete (never the
   last one), activate. (`decisions.md §5`.)
5. **Old saves.** A character with no `equipment_sets.json` loads into a `Default` set: slotted copies keep
   their slot, `Absent` copies are Not Carried, the rest Carried; AC and weight are unchanged from today,
   proved by a test over the seeds and the `.pcg` import population (`decisions.md §8`, G-1).
6. **Market.** A bought copy is Carried in every set (no set entry is written; an unnamed copy reads
   Carried). (`decisions.md §10`.)
7. **Weapons.** The Weapons tab and the printed Weapons block list in-hand weapons first, then carried
   ones (marked), never Not Carried ones. (`decisions.md §9`.)
8. **Custom containers.** A custom item may name a base item; with a container base it holds copies and
   weighs what its base weighs; the Custom tab and dialog expose the base. (`decisions.md §11`.)
9. **The gear screen.** A split panel in the Gear tab: left, every copy with name, weight and location,
   filter All / Equipped / Not equipped; right, the locations (18 slots, Equipped, Carried with containers
   nested, Not Carried); a "Move to" menu reachable by keyboard on every row; drag and drop to any
   location; a set picker. Every number on it is an engine row. (`decisions.md §13, §14`.)
10. **ui-smoke rows** for the screen (≥ 6 new) and the five existing equipment rows rewritten to it
    (`sheet-equipment-{lists,conflict,unequip,equip,left-behind}`, `sheet-market-buy-weapon-worn`); the
    inventory regenerated. (`decisions.md §15`.)
11. **Print.** The Inventory page prints the active set by location: slots in use, then Carried with each
    container's contents nested under it, then a short Not Carried list without weight; its totals equal
    the encumbrance rows. (`decisions.md §9`, E4.1.)
12. **Operator gate (E4.G).** The operator looks at the gear screen (screenshots under `artifacts/`) and
    prints the inventory page, and writes `usable: yes` for each.
13. **Closure.** All cards `complete` → final-acceptance scan → retrospective written and cited → sweep →
    release notes → architecture truth-up + claims critic → **graphify LAST** → **PR last**
    (`decisions.md §18`). The operator merges.

## 3. Not in the Definition of Done (planned deferrals, `forward-scope-register.md §4`)

Weight-reduction containers (bag of holding kinds, 25 records: contents weigh in full, "no stated
capacity" shown); moving items between characters; quantity stacks (today one copy is one row);
Starfinder and Cyberpunk RED gear screens (their markets stay as they are); a wield style per weapon
(one-handed / two-handed) beyond the three wield slots the app already has; charges.

## 4. Epic order and the SD-40 partition

C → **E1 (engine, root crate; may run now beside SD-40)** → *SD-40 merges; `tranche-21` catches up* → E2
(desktop backend: sets file on the character root, commands, DTOs, migration) → E3 (the gear screen, drag,
smoke rows) → E4 (print; merge check; **E4.G operator gate**) → E5 (closure). The partition is derived
from SD-40's open cards' FILES YOU OWN (`artifacts/epic_0/census.txt §7`; `epic-breakdown.md §0`).

## 5. Mode

Unattended after launch except at E4.G. A taste question inside scope takes the safe default in
`decisions.md §12.1` and is logged; nothing else waits. A question only the operator can answer is one of
`decisions.md §21`'s five, each with its default already taken.
