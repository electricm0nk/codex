# SD-41 — the operator's brief, 2026-11-12

The planning session received this brief from the lead on 2026-11-12, while SD-40 was live on
`tranche-20`. Quoted text is the operator's, verbatim as relayed by the lead; the rest is the lead's
condensation. `decisions.md §3` turns the answers into A-rows; `scope-draft.md §1` quotes the request;
this file is the record.

## The request (verbatim)

> "on the gear screen, we need to make some adjustments.  We need to borrow from the type of interface that pcgen is using - use this as a reference point, but do not copy the code.  Functionally, it is as follows:  It uses equipment (we can call it gear, that is fine) but that screen has a split panel.  The left panel shows all the gear and can be filtered by equipped, not equipped, and all.  The right gear shows a listing of all the equipment slots:  Head, Hands, Torso, Legs, Shield, Equipped, Carried, and Not Carried.  When someone buys something from the market it needs to default to carried.  When someone equips something, for instance a sheild, that will take up the shield spot and will impact the AC.  If it is not in the slot, it doesn't impact the AC.  If something is carried, the encumberance is impacted by the weight.  If it is not carried, then it doesn't count against the weight.  The Equipment set allows for the creation of custom sets.  One might have a default set, but then maybe you have a 'formalwear' set - one doesn't normally wear a suit of armor to a fancy dinner with the lords of the castle.  The equipment view on the left side shows the item, it's weight, and location.  Location is the slot.  This might be the Primary Hand, or it might be not carried.  A user can move items between the different locations by dragging them.  Some items are containers:  I might equip a backpack and put items in the backpack.  I might even put items in a pouch and then the pouch in the backpack to keep things organized.  This all needs to hook in with our ability to customize items.  I use a custom named pouch called food, and then put food in food pouch.  I might have another that is called camping and i put a bedroll and such in that thing."

## The answers (2026-11-12, verbatim as relayed)

- Slots: "Full Pathfinder list" — right panel = the app's existing WORN_SLOTS (`apps/desktop/src/characterHub/EquipmentPanel.tsx:7`: Main hand, Off hand, Both hands, Shield, Armor, Head, Headband, Eyes, Shoulders, Neck, Chest, Body, Belt, Wrist, Hands, Ring ×2, Feet), then Equipped (no slot), Carried, Not Carried.
- Sets: "Where each item is" — all sets share one item list; a set stores only each item's location (slot / equipped / carried / not carried / inside container X); one active set drives sheet, AC, encumbrance and print.
- Containers: "Weight counts, capacity shown" — contents count toward weight only when the container is carried or equipped; capacity shown (e.g. backpack 60 lb) with a warning when over, never a block; nesting allowed; a custom-named item (e.g. "Food", "Camping") works as a container when its base item is one; hooks into the existing Custom tab / custom item naming.
- Moving: "Drag plus a 'Move to' menu" (keyboard + testable).
- Weapons block (screen + print): "In hand first, then carried"; Not Carried weapons left off.
- Old saves: "Make a 'Default' set" — on first load, slotted items keep their slot, everything else Carried; AC and weight unchanged from today.
- Print inventory page: "Active set, by location" — slots in use, then Carried with each container's contents nested under it, then a short Not Carried list (no weight); totals match encumbrance.
- Market purchases default to Carried (operator's words).
- Package placement: "New package after SD-40".

## The PCGen rule (the lead, carrying the standing ruling)

PCGen is a converter and test oracle only; "It's not fine to include 1 line of java in our live code."
The planning session may read PCGen's equipment UI under `$PCGEN_REPO_DIR` (resolved via
`scripts/pcgen-oracle-pin.env` / `scripts/fetch-pcgen-oracle.sh`; never the literal path) for functional
reference only and describe behaviour in our words. No code or class structure copied.

## The lead's instructions to the planning session

Mirror SD-40's package shape; census first (`artifacts/epic_0/census.py`, `python3 -I`, every figure with
its command, awk not `grep -o`); name which SD-40 open cards touch the same files and partition what can
run now from what waits; TDD per card; FILES YOU OWN closed; engine-owned numbers in Rust rendered by UI
and print; ui-smoke rows; a save-format migration with a test that an old save loads with AC and weight
unchanged; an operator eyeball gate on the screen and the printed page; merge checks per epic; a closure
epic; the paper-sheet rule.
