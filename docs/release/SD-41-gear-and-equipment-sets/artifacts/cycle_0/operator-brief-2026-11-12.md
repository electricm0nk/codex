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

## Seven follow-up items (2026-11-12, verbatim as relayed by the lead; the answers follow each)

1. > "Weapons - always include unarmed as an option"
   Answers: placement = "Last row, full numbers" (the engine's real attack bonus and damage; 1d3 nonlethal Medium / 1d2 Small; monk unarmed damage progression; Improved Unarmed Strike makes it lethal; computed in Rust, rendered by UI and print); timing = "With SD-41 gear work".
2. > "weapons - only show ammunition if the item has ammuntion.  ie:  swords don't have ammo, don't show it.  crossbows do have ammo, show the ammo indicators"
   Answers: timing = "With SD-41 gear work"; scope = "Yes, screen and print"; each ammunition-using weapon (decided by the engine from corpus data, not a TS list) gets its own ammo tick boxes; no global ammo row.
3. > "on the market screen:  I would prefer the list of armor be more of a table view that shows the item and the price.  Include a space at the bottom of the window for description that shows the description of the item that is highlighted.  You should be able to sort any list by the name or the price.  Include a checkbox called 'Only Show Affordable' that filters out items that the character lacks sufficient funds to purchase.  We also need a buy custom button.  Take a look at the pcgen code for how this works.  It allows the assignemtn of custom names and abilities to the item being purchased."
   Answers: Buy Custom = "Full rules builder" (base item + custom name + masterwork + enhancement +1..+5 + special abilities + special material + size; price and weight from the rules in Rust; enhancement reaches AC / attack / damage through the engine; ability text prints as text, dice literal); PCGen read for function only; columns = "Item, Price, Weight", every list sortable by any header; description box at the bottom; 'Only Show Affordable' at the active price mode (three modes honoured); package = "SD-41, with the gear work".
4. > "On the market screen, include a Create option that allows the user to manufactor a custom items.  They can free text the name and description, and then enter a weight."
   Answers: price = "Optional price" (0 by default; above 0 deducted like a purchase); kind = "Optional container + slot" (a Container toggle with a capacity in lb; an optional body slot, default none); lands Carried; distinct from Buy Custom (no corpus base); persists, round-trips, prints with its weight, weighs when carried, works as a container, editable/removable afterwards.
5. > "I see a few items with no price:  Adamantine Breastplate - we need to find the actual price for all items.  Another example is Astral Armor that has a negative price.  All that needs to get cleaned up.  If you cant find the price in the pcgen files, get it from d20pfrsd.com.  ie: https://www.d20pfsrd.com/magic-items/magic-armor/specific-magic-armor/adamantine-breastplate/.  Oh, all items in marketplace should include 2 more colums: weight and slot"
   > "allow the user to specify the price on purchase - which gives me an idea, let's add a negotiage button next two buy that allows the user to customize what they pay.  We need a similar option where we can negotiate the sale price if selling something."
   Answers: source = "Fix converter + cited list" (converter first; still-missing prices from d20pfsrd.com into ONE checked-in list with the page URL per item, a converter input; a test fails on any missing or negative Market price not listed as price-unknown); a Negotiate button next to Buy (default = the listed price at the active mode; price-unknown Buy routes through Negotiate) and on Sell (default = the current sell rule); Market columns Item, Price, Weight, Slot, all sortable; the converter lane is tool side with its own CARGO_TARGET_DIR; the d20pfsrd lookups are capped by the census count.
6. > "the dropped option in gear is causing the item to disappear, it should go to the \"not carried\" slot instead"
   Facts: today Drop sells the item back (`CharacterSheet.tsx:749`, `:870`, `:881`, `:1706`). Answer: "Sell + Discard buttons" — Drop moves to Not Carried (no money, item kept); Sell is separate (with the Negotiate sale price); Discard deletes for no money after a confirm; update the smoke rows and help text.
7. > "We also need to allow someone to buy things in bulk.  No one wants to click buy 50 times to buy 50 arrows.  Same with Sell"
   Answers: "One row with a count" (Buy, Sell and Negotiate take a Quantity; identical items stack into one row with weight and price × quantity; Move asks how many and splits; Sell / Drop / Discard take a quantity); "Bundles, as the book sells them" (3 × "Arrows (20)" = 60 arrows at 3× bundle price/weight; the row shows "Arrows (20) ×3 = 60"); ammo tick boxes read the stacked count.

The lead also ruled: Q3 of `decisions.md §21` is answered by item 4 (a container need not name a base item); Q1, Q2, Q4, Q5 defaults stand unless the operator says otherwise.
