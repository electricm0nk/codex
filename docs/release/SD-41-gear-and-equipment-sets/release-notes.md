---
canonical: true
bundle_id: SD-41
slug: gear-and-equipment-sets
tranche: 21
date: 2026-11-12
status: interim — replaced at E5.3
---

# Campaign Codex 0.21 — Gear and Equipment Sets

SD-41 INTERIM NOTES. This body is replaced at E5.3 with the final notes; it exists so the release-notes
resolver passes from C1 on (`decisions.md §2`). Nothing publishes from `tranche-21`.

## Planned for 0.21

- A new gear screen in the Gear tab: every item with its weight and location on the left (filter All /
  Equipped / Not equipped), every location on the right (the Pathfinder slots, Equipped, Carried with
  containers nested, Not Carried); move items by drag or by a "Move to" menu.
- Equipment sets: name sets ("Default", "Formalwear"), place the same items differently in each, and
  switch the active one; the sheet, AC, carried weight and the print follow the active set.
- Containers: a backpack or pouch holds items, pouches nest in packs, capacity is shown with a warning;
  a custom-named item based on a container is one.
- Market purchases go to Carried.
- Weapons list in-hand first, then carried; the printed Inventory page prints the active set by location.
- Old saves load into a Default set with AC and weight unchanged.
- The Weapons block always ends with an Unarmed row; ammunition boxes appear only under weapons that use
  ammunition, with the carried count.
- The Market is a sortable table (Item, Price, Weight, Slot) with a description box, Only Show Affordable,
  a Quantity, Negotiate on Buy and Sell, Buy Custom (name, masterwork, enhancement, abilities, material,
  size, priced from the rules) and Create (a free-text item with a weight, an optional price, an optional
  container and slot).
- Missing and negative prices are fixed in the converter; the rest come from a cited price list.
- Drop keeps the item as Not Carried; Sell and Discard are separate; identical items stack with a count.
- A character in a campaign is copied, whole, to the campaign's folder on every save (one folder per
  character); the Campaign field on the sheet adds or removes campaigns; one button per campaign under
  Skills opens it in the Campaign Manager.
- Starfinder and Cyberpunk RED gear and markets are unchanged.

Final figures, commands and the operator's two sign-offs are filled in at E5.3.
