# SD-43 intake

Opened 2026-11-18 by the operator. Items are listed in the order the operator raised them.
The full SD-43 package is authored from this list when SD-42 closes.

## 0. Before SD-43 starts: post-SD-42 cleanup (operator, 2026-11-18)

Run in this order, after SD-42 is merged:

1. Clean up everything. Collapse all worktrees under `.worktrees/` and remove their `CARGO_TARGET_DIR`s.
2. Update the public site (campaign-codex.org, Cloudflare Pages) with the latest content.
3. MOVE the latest site content into the `electricm0nk/codex-web` repo (private, default branch `main`; it already holds `.github/`, `README.md`, `site/` as of 2026-11-18). Move, not copy: the site leaves this repo.
4. Update graphify (no `--force`, no `graphify update`).

## 1. BUG: campaign folder relocation does not migrate content or create a skeleton

- **Reported:** 2026-11-18.
- **Observed:** the operator changed the campaign folder location. The existing content did not move to the new location, and no skeleton was created there.
- **Expected:** a location change carries the existing content to the new folder and creates the standard skeleton there.
- **Code to start from:** `apps/desktop/src/campaign/campaignFolderModel.ts`, `apps/desktop/src/campaign/campaignMigrationModel.ts`, `apps/desktop/src/campaign/EditCampaignScreen.tsx`.
- **Open questions for the operator:**
  - Move or copy?
  - What if the new folder already holds content (for example, another member's synced copy)?
  - Keep or remove the old folder?

## 2. ENHANCEMENT: rest and recovery rules shown as plain text

- **Reported:** 2026-11-18. Restated by the operator the same day.
- **Ask:** show, per race and class, what the character needs to start fresh after rest, and what is healed per night of sleep.
- **Source:** the game's own rules text. The operator's first examples (1 hour meditation for spells, 8 hours for panache) were illustrations only, not requirements. Use what the rules say.
- **Display:** plain text. No calculations: do not compute hit points healed or any other number.
- **Override:** the player can override this text in the custom rules menu.
- **Placement (operator, 2026-11-18):** the printout, under the bio page.
- **Adjustment (operator, 2026-11-22):** also show a copy of the same rest and recovery text in the app,
  on the character screen's **Biography** tab, under the biography text. Same text, same order as the
  printed sheet. It reads from the same source as the printout, with the same player override. It is
  not a second copy of the rules.
  - Code to start from (read, not run): `CharacterSheet.tsx` ~4531, the `Biography` tab (`FreeTextTab`).
  - Check before building: the Biography text area is one large field that saves on blur. The rest text
    is read-only and must not be saved into the biography text.

## 3. ENHANCEMENT: new House Rules menu, per character and per game system

- **Reported:** 2026-11-19. Replaces the first draft (a slot-limit switch in the custom menu).
- **Menu:** a new top-level menu entry, **House Rules**. It is a new menu, not part of the custom menu.
- **Scope rules:**
  - Tied to one character, not to the install. Setting a rule on Brother Tomas Vane does not turn it on for Aonghus or Aldric.
  - New characters start with every house rule off.
  - Game-system specific: only rules that apply to the character's system show. None of the rules below show for Cyberpunk RED.
- **House rules:**
  1. **Slot limit off.** Turns off the per-slot equip limit (most slots allow 1 item), so a slot can hold more than one.
  2. **Hit Points** at level-up, one of:
     - User Rolled: the player enters the hit points gained.
     - Random: Codex rolls the hit die.
     - Random, 50% floor: Codex rolls, and the result is never below 50% of the die maximum.
     - Average.
     - Average, round up.
     - Maximum.
     - Percentage: the player enters a percentage of the die maximum.
  3. **Ignore feat prerequisites.**
  4. **Ignore weapon proficiency requirements.**
  5. **Ignore armour proficiency requirements.**
  6. **Ambidextrous.** No off-hand penalty.
- **To verify at planning:** where existing custom options (overrides, grants) are stored today. Any found in the install-wide store (`apps/desktop/src/settings/`) breaks the per-character rule.
- **Open questions for the operator:** first-level hit points (the rules give maximum at level 1: does the house rule apply there too); rounding for Average and Percentage; whether a rolled result is kept or re-rolled when the sheet is regenerated.

## 4. ENHANCEMENT: export and import custom items, locally or through the campaign folder

- **Reported:** 2026-11-19.
- **Why:** so other party members can pick up a player's custom items.
- **Export:** the player picks a target: local (a file the player saves) or campaign (written into the campaign folder).
- **Import:** the app asks the source: a local file, or load from the campaign folder.
- **Duplicate name:** when an imported custom item has the same name as an existing one, the import is an edit of that item, not a second copy.
- **Open questions for the operator:** one file per item or one file of many; where in the campaign folder skeleton the items live; whether an import across game systems is refused.

## 5. BUG + REDESIGN: gear screen equipment sets — one list column, and container contents wrong after a move

- **Reported:** 2026-11-21.
- **Redesign ask (operator):** under the equipment set, the gear screen shows two columns. Use ONE column, a list view. The action buttons (Customize, Move to, Drop, Sell, Discard) sit on the SAME LINE as the item name.
- **Observed (operator, test character Brother Tomas Vane; not yet reproduced by us):**
  1. Pouch, Belt and Sack #2 still show inside the Backpack after the operator removed them from it.
  2. Sack #1 and Sack #2 both show inside the Bag of Holding.
  3. The right panel lists one sack (holding a bedroll and a blanket) as "not carried", and "Sack x3" also as "not carried".
- **Expected:** a container shows only the copies actually in it. A move or a remove changes the source and the target at once. The Not Carried list matches what the left side shows.
- **Code to start from:** `apps/desktop/src/characterHub/GearScreen.tsx` (688 lines; Move to / Drop / Sell / Discard, `Customize` at line ~309), `gearScreenModel.ts`, `GearDetail.tsx`.
- **To verify at planning:**
  - Reproduce on Brother Tomas Vane before any fix. Record the exact steps and the stored container data for each symptom.
  - Whether symptoms 1–3 are display bugs (the model reads stale container membership) or data bugs (the move did not write). The fix is different for each.
  - Whether the two columns are what causes the wrong picture, or a separate defect underneath. Do not assume the one-column change fixes the data.
  - Context: SD-41 (gear and equipment sets) and SD-42 (container tables, per-copy rename) both touched this screen. Read their `decisions.md` rulings (SD-42 §27.3) before changing row layout.
- **Row order (operator ruling, 2026-11-21):** the single list has two sections, each sorted alphabetically:
  1. **Non-containers**, alphabetical.
  2. **Containers**, alphabetical, with their nesting kept. A container's contents sit under it, in the same two-part order: its non-container items alphabetical, then its container items alphabetical. This repeats at every depth.
  - Example: top level holds a Belt Pouch and a Backpack. Both list in section 2. Inside the Backpack: its loose items A–Z, then its own containers A–Z.
  - **Indent kept (operator, 2026-11-21):** contents are indented under their container.
  - **Light boxes kept (operator, 2026-11-21):** the light box drawn around each container today stays. Nested containers get their own box inside the parent's box. Re-derive today's styling from `GearScreen.tsx` before changing it.
  - **Sort tie-break, to state at planning:** `Sack #2` before `Sack #10` needs a stated rule (natural number order), since plain A to Z puts `#10` first.
- **Not Carried (operator ruling, 2026-11-21):** its own section, separate from the two-part list. It can hold items stored away, for example in a bank. Items in it do NOT count toward encumbrance. Check at planning that the encumbrance total (`encumbrance.totalCarriedWeightLbs`, the engine's) excludes them, and that the screen's carried weight matches the sheet.
- **Likely cause (operator's hypothesis, 2026-11-21; not yet verified by us):** empty containers of the same name are stacked. The two panels name them differently.
  - Operator's case: one Sack holds 2 items; three empty Sacks sit beside it.
  - Right panel (the holdings): shows "Sack x3" for the three empty sacks.
  - Left panel (the container boxes): labels the sack with items "Sack #1" and the whole stack of three "Sack #2". It treats the stack as ONE container.
  - Renamed copy: the operator split one sack off and renamed it "Custom Sack Name". The left panel still labels it "Sack #2", not the custom name.
- **Naming rule (operator, 2026-11-21):** the same name on both panels. The sack with 2 items is just "Sack", not "Sack #1". A renamed copy shows its custom name on both panels.
- **To verify at planning:**
  - The left panel's label comes from a numbering step ("#N") that ignores the per-copy name (`CopyFacts.name`, SD-42 E4.9, `decisions.md §27.3`). Find where, and why a renamed copy loses its name.
  - Whether a stack of N identical empty containers can be a container box at all. A stack cannot hold items per copy. Putting an item in one sack of a stack must split that sack off the stack first. Test this path.
  - Symptoms 1 and 2 in "Observed" (items shown inside the wrong container) may come from this same stack-as-one-container defect. Check before treating them as separate bugs.
- **Duplicate names (operator ruling, 2026-11-21):** a container that holds items is NEVER part of a stack. Several containers may share one name; that is fine. No "#N" numbering. The player renames a copy if the repeats confuse them. A stack is only identical EMPTY copies, shown as "Name xN".
- **Open questions for the operator:** none left on naming. At planning, confirm what happens when an item goes into an empty stacked container (it splits off the stack first, then holds the item) and when a container becomes empty again (it stays separate; it does not re-merge unless the operator says so).

## 6. ENHANCEMENT: double-click an item to equip it or carry it

- **Reported:** 2026-11-21.
- **Ask (operator):** double-clicking an item that has a slot (armor, for example) sends it to that slot. If it is already in a slot, a double-click sends it to Carried.
- **Code to start from:** `apps/desktop/src/characterHub/GearScreen.tsx` (`onMove` at line ~73; the move write at ~674) and `gearScreenModel.ts` (`WORN_SLOTS`). No double-click handler exists today (`grep -n -i -E "onDoubleClick|dblclick" apps/desktop/src/characterHub/GearScreen.tsx` returned nothing). Reuse the existing Move to path. Do not add a second write path.
- **To verify at planning:**
  - How an item's slot is known (the record's slot field) and what "no slot" looks like. A double-click on a slot-less item must do nothing visible, or say why.
  - Items that fit more than one slot (a ring, a one-hand weapon: main or off hand). Which slot does a double-click pick?
  - A slot that is already full. Replace, swap, or refuse?
  - A stack with quantity above 1: does a double-click move one or all?
  - The double-click must not fire when the player double-clicks a button on the row (Customize, Drop, and the rest).
  - It must work by keyboard too (an accessibility need), for example Enter on a focused row.
- **Rulings (operator, 2026-11-21):**
  1. **Item fits several slots:** a double-click picks the FIRST FREE slot. If none is free, it uses rule 2 on the first slot in `WORN_SLOTS` order that the item fits.
  2. **Slot already full:** SWAP. The old item goes to Carried.
  3. **Stack above 1:** ONE per double-click.
  4. **Keyboard:** Enter on a focused row does the same as a double-click.
- **Open questions for the operator:** none. At planning, confirm "first slot in `WORN_SLOTS` order" is the right tie-break when no slot is free (my wording of the operator's "swap"; the operator did not name the tie-break).

## 7. BUG: dropping an item onto a container works only on the container's name row

- **Reported:** 2026-11-21.
- **Observed (operator):** to drag an item into a container, the drop must land on the container's name. Dropping anywhere else in the container's space does not work.
- **Expected (operator):** the whole container box is a drop target. Dropping on top of an item already inside the container puts the dragged item into that container.
- **Likely cause (read from the code; not yet reproduced):** in `apps/desktop/src/characterHub/GearScreen.tsx` the drop zone is attached to the container's own row only (`dropZone(...)` at lines ~397 and ~414, `row.isContainer ? dropZone(`inside-…`) : {}`). Item rows inside the box carry no drop zone, so a drop on them bubbles up to the section zone (line ~543). That may move the item to the section, not into the container. Re-derive: `grep -n "dropZone" apps/desktop/src/characterHub/GearScreen.tsx`.
- **To verify at planning:**
  - Reproduce first: drag onto the container name (works), onto an item inside it, onto the empty space in the box. Record where each drop lands.
  - Nesting: dropping on an item inside a nested container must target the NEAREST container, not an outer one. `stopPropagation` at line ~372 already stops bubbling; keep that rule.
  - A drop on the container the item already sits in is a no-op, not an error.
  - A container cannot be dropped into itself or into its own contents. Check that `dropTarget` in `gearScreenModel.ts` (line ~363) already refuses it, and that the larger drop area does not widen a hole.
  - Related: item 5 reshapes the same rows into one list with boxes. Land this fix on the new layout, not the old one.
- **Open questions for the operator:** none yet. Dragging onto the header line of the box, the empty space, and any item inside it should all mean "into this container".

## 8. ENHANCEMENT: one standard Market button, top-left on every panel; Print button and pricing control leave the panels

- **Reported:** 2026-11-21. Replaces the first draft (a Market tab). **Operator ruling: do NOT add an entry to the tab list** (`SHEET_TABS` stays as it is).
- **Asks (operator):**
  1. The Market button sits **top and left-justified on every panel** that has one. Today: middle, beside Print, on Weapons; top middle on Defense; under the money and pricing control on Gear.
  2. The button is **twice its current size** and uses the **existing accent color**.
  3. **Remove the Print button** from the panels. Print belongs under the menu.
  4. **Remove the pricing control** (the price-mode choice) from the Gear panel. Pricing shows only in the Market window.
- **Code to start from (`apps/desktop/src/characterHub/CharacterSheet.tsx`; re-derive with `grep -n "onOpenMarket\|PriceModeControl\|props.onPrint" apps/desktop/src/characterHub/CharacterSheet.tsx`):**
  - Weapons panel: `PriceModeControl` ~849, Market ~851, Print ~854.
  - Defense panel: Market ~1396.
  - Gear panel: `PriceModeControl` ~1615, Market ~1617.
  - Shared style `addItemButtonStyle` (~220) is also used by Add Spell and others; the new size and color apply to the Market button only. Prefer one shared Market-button component so the three panels cannot drift apart.
  - `PriceModeControl` is also used in `MarketDialog.tsx:181` and `CreationEquipmentDialog.tsx:111`. Those stay.
  - Starfinder and Cyberpunk sheets have their own Market (`StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx` with `SystemMarketDialog`). Check each for the same placement problem.
- **To verify at planning (these can break something):**
  - **Sell depends on the pricing control.** `GearScreen.tsx` says "Sell returns money under the pricing above" (lines ~31, ~188, ~205). `priceMode` is session-only state held in `CharacterSheet.tsx` (~2890). Removing the control from Gear leaves Sell with a price mode the player can no longer see or change on that screen. Decide: Sell uses the mode last set in the Market window, or Sell has its own fixed rule. Update the Sell wording either way, since "above" would point at nothing.
  - **Weapons also shows a pricing control (~849).** The operator named Gear only. Confirm whether Weapons loses it too.
  - **Where Print goes.** `grep` found a Print tab in Settings (paper choice, `settings/PrintPanel.tsx`) and `PrintDialog.tsx`, but no menu entry that opens the print dialog for a character. Find where "under menu" is, or add the entry there. Do not delete the Print button until another route to print exists (no-stub rule: do not leave a feature with no way in).
  - Print buttons on other panels. Only Weapons showed one (~854); check the rest.
  - The two button sizes: "twice" means twice today's Market button. State the numbers (height, font size) in the cycle receipt.
- **Open questions for the operator:** (a) Sell pricing: RULED 2026-11-21, see item 9 (Sell gets its own small window with its own price choice); (b) the Weapons pricing control; my pick: remove it too, so pricing lives only in the Market; (c) where the character print entry lives in the menu today.

## 9. ENHANCEMENT: Sell opens a small Market sell window (Market Rate / Build / Negotiate)

- **Reported:** 2026-11-21. Follows from item 8 (the pricing control leaves the Gear panel, so Sell needs its own price choice).
- **Today:** Sell drops an inline row under the item: a "Negotiate (gp, blank = the pricing above)" box with Sell and Cancel (`GearScreen.tsx`, `PendingForm`, ~172–215; the caption at ~31 and the strings at ~188 and ~205 say "the pricing above").
- **Ask (operator):** clicking Sell pops up a small **Market sell window**.
  - The player picks **Market Rate**, **Build**, or **Negotiate**.
  - The window shows the **default sale amount** for the chosen option.
  - If **Negotiate** is chosen, the player **types the value**.
  - The player finishes with **Sell** or **Cancel**.
- **Code to start from:** `GearScreen.tsx` `PendingForm` and the Sell branch of `onSell` (~74, ~338); `priceMode.ts` (`PriceMode` = `cashless | standard | characterBuild`, `refundPreviewCopper`, `PRICE_MODE_OPTIONS`); `MarketDialog.tsx` for the window look. `refundPreviewCopper` mirrors `refund_copper` in the backend; the window must use it, not a second formula.
- **Mapping to confirm at planning:** Market Rate = `standard` (returns 50% of the price); Build = `characterBuild` (returns 100%); Negotiate = the typed gp total, which the Sell write already accepts (`onSell(row, quantity, negotiatedGp)`).
- **To verify at planning:**
  - **Cashless has no place in the operator's three options.** In `cashless` mode a sale returns no money. Decide what Sell does when the session is cashless (see open question 1).
  - Stacks: the window keeps the "How many?" box for a stack, and the default amount scales with the count.
  - Items with no catalog price (custom items): the default amount is blank or 0, and Negotiate is the way to set a value. Check `marketPriceGp` for the null case.
  - The window defaults to Market Rate, or to the Market window's last choice. `priceMode` is session-only state in `CharacterSheet.tsx` (~2890); the sell window's choice must not silently change what the Market window uses next.
  - Negotiate input: reject negative and non-number values; the existing check at ~187 does this. Keep it.
  - Keyboard: Escape cancels; Enter sells; focus lands in the window and returns to the Sell button.
  - The wording "the pricing above" is removed everywhere (caption ~31, ~188, ~205).
- **Rulings (operator, 2026-11-21):** (1) **Cashless is left out of the sell window.** It shows only Market Rate, Build and Negotiate. (2) **The window opens on Market Rate.**
- **Consequence to confirm at planning (my reading, not the operator's words):** the window always returns money per the option chosen, even when the session's Market pricing is set to Cashless. The sell window's choice is its own and is not written back to the session `priceMode`.
- **Open questions for the operator:** none.
- **Name ruling (operator, 2026-11-21):** the first option is called **Market Rate**, not Normal. The word "Normal" must not appear in the window. The mode behind it is unchanged (`standard`, sells at 50%).
- **Scope of the naming and Cashless rulings (operator, 2026-11-21):** they apply ONLY to the new sell window. The Market window keeps its existing pricing control and labels unchanged: **Cashless**, **Buy 100% / Sell 50%**, **Character build (100% / 100%)** (`PRICE_MODE_OPTIONS` in `priceMode.ts` is not edited). The same mode can carry two names (Market Rate in the sell window, "Buy 100% / Sell 50%" in the Market window); that is intended.

## 10. ENHANCEMENT: the Market's description box shows the item's stats, not only its prose

- **Reported:** 2026-11-21.
- **Observed (operator):** the Description box in the Market shows only the description text. For a bastard sword it shows the paragraph about its length and the exotic-weapon rule. It does not show the damage, the critical range, or any other stat.
- **Ask (operator):** show the stats in that box. Same for Armor and Gear. Where an item has stats, list them.
- **Cause (read from the code; not yet run):** the Market row carries no stats. `MarketEntryDto` (`apps/desktop/src/boundary/listMarket.ts`) extends `EquipmentCatalogEntryDto`, which holds key, category, name, cost, weight, book, types and the description prose, plus `slot` and `variants`. There is no damage, critical, range, armor bonus, max Dex, check penalty or spell failure field. The box (`MarketDialog.tsx` ~305) prints `selected.description ?? selected.detail`.
- **Where stats already exist:** the Gear screen's detail panel (`apps/desktop/src/characterHub/GearDetail.tsx`, SD-42 `decisions.md §27.4`, DET-1) lists the item's record from the engine's `describe_held_equipment`, including weapon damage (`WeaponDamageDto`) and container stats. It takes a HELD copy, not a catalog row.
- **Design constraint:** the twin rule applies (`docs/release/SD-27-future-state-book-content-ingestion/decisions.md §29.1`). The UI must format the engine's numbers and never derive them. Stats come from a Rust command over the catalog record, shown as the engine states them.
- **To verify at planning:**
  - What the corpus record holds per category. Read whole records, not filtered fields (`corpus-read-whole-record-not-filtered-fields`): weapons (damage, critical, range, type, proficiency, size), armor and shields (armor bonus, max Dex, check penalty, spell failure, speed), gear (what stats exist at all, such as capacity, duration, uses).
  - Whether `describe_held_equipment` can be reused for a catalog row, or whether a new `describe_market_entry` command is needed. Prefer reuse: one renderer for the Gear detail and the Market, so the two cannot disagree.
  - Rows with variants (`+1` to `+5`): the stats shown follow the chosen `+N` (enhanced damage bonus, armor bonus).
  - A stat the record does not state reads `—`, as `GearDetail.tsx` does today. No fabricated zeros.
  - Magic items and Create-your-own items: show what the record has; do not invent stats.
  - The box gets taller. Check it at the dialog's smallest size and make it scroll.
  - Print: this is a screen change; the printed sheet is not touched.
- **Proof width, stated up front:** a test on the bastard sword proves one weapon. It does NOT prove armor, shields, gear, magic items, or the many records with missing fields. The acceptance run must sample every category and every `TYPE` shape in the catalog, and count how many rows show `—` for each stat, so a silent gap shows as a number (`proof-is-only-as-wide-as-its-cases`).
- **Open questions for the operator:** the order of the stats (my pick: stats first, then the prose); whether a missing stat is hidden or shown as `—` (my pick: shown as `—`, same as the Gear panel).

## 11. ENHANCEMENT + INVESTIGATION: Market type filters, no 200-row cap, and the item counts

- **Reported:** 2026-11-21. The operator asked for this to be discussed first ("this one might be hard"); the rulings below came from that talk.
- **Problem:** the Market shows only the first 200 rows of a list (`MARKET_ROW_LIMIT = 200`, `apps/desktop/src/characterHub/itemPickerFilter.ts:362`; the notice is at `MarketDialog.tsx:248`). The rest is reachable only by search.
- **Rulings (operator, 2026-11-21):**
  1. **A filter column**, not chips above the table. Each list gets one or more filter columns beside the table.
  2. **Remove the 200 cap.** Every row is reachable by scrolling. A long list must not freeze the dialog: draw only the rows on screen (windowed list), never all 4,000 at once.
  3. **Weapons filter on kind AND damage type.** A weapon with two damage types (piercing and slashing) appears under both filters. **Ammunition is a filter value.**
- **Filter columns proposed (to confirm at planning; tag counts are ESTIMATES, see below):**
  - **Weapons:** Kind (Melee, Ranged, Thrown, Ammunition); Damage (Bludgeoning, Piercing, Slashing); Skill (Simple, Martial, Exotic); Hands (Light, One-handed, Two-handed).
  - **Armor:** Weight class (Light, Medium, Heavy); Suit and Bracer.
  - **Shields:** Light, Heavy, Tower, Buckler.
  - **Gear:** reuse the app's existing category function (`apps/desktop/src/characterHub/equipmentCategories.ts`: Adventuring Gear, Tools & Kits, Containers, Alchemical Items, Poisons, Mounts & Animals, Services, Trade Goods, Gems, Spell Components). Raw tags are too coarse here (the single tag `Goods` covers about half the list).
  - **Magic items:** Kind (Wondrous, Scroll, Wand, Rod, Staff, Potion, Ring) and Slot (Neck, Belt, Headband, Glove, Cloak, Headgear). The Wondrous list alone is far over 200, so the cap removal is required, not optional.
- **Where the filter data lives:** each row's `types` (PCGen `TYPE:` tags) is already on the Market row (`EquipmentCatalogEntryDto.types`). The filter needs no new backend field. The filter VALUES per list are a table kept in one place (a model file with tests), not scattered in the dialog.
- **THE COUNT GAP (operator, 2026-11-21): the app says 3,680 magic items and 1,624 gear; the discussion estimate said 4,677 and 3,573.** The operator reads this as missing items. **That is NOT established.** The estimate came from a different source and a re-implemented rule:
  - Estimate source: `data/equipment_types.json` (`types`, 10,674 names). The Market's rows come from `build_equipment_catalog()` over converted records (`apps/desktop/src-tauri/src/equipment_catalog.rs`, `built_catalog()` ~834), a different set.
  - Re-derive the estimate: `python3 -c "import json;print(len(json.load(open('data/equipment_types.json'))['types']))"` → 10674. Its list rule skipped `is_magic_marked`, so rows moved between Gear and Magic.
  - Rules that legitimately drop rows from a Market list, all in `equipment_catalog.rs`: dedupe by `key`, first wins (`market_from_catalog`, ~816); `hidden_from_market` (a key ending `.FORGET`, a `(Base)` template with no price, a `Codex-Named Unit`); category `Equipmods` (`market_category_of`, ~709, returns none).
  - **What must happen first (before any filter is built):** produce the REAL per-list counts and account for every row of the source that is not on a list. Method: a Rust test or `cargo` example that walks `built_catalog()` and prints, per Market list, the count shown, and for every dropped row its reason (duplicate key, template, forget directive, Codex-Named Unit, Equipmods, no tags, book not ingested). Acceptance: shown + dropped-with-reason = source total, per list, and the sum matches `built_catalog().entries.len()`. A row dropped with no reason is a defect to fix, not a number to explain.
  - The operator also reports Armor, Weapons and Shields as missing items. The operator has not given the app's counts for those three lists; get them (the Market header or a screenshot) as the baseline.
  - This is the "a number is not wired until it moves on the surface the player reads" lesson: the proof is the number the Market dialog itself shows, per list, before and after.
- **Proof width, stated up front:** the filter lists above come from tag counts over an uneven tag corpus. A passing test on the Weapons filter does NOT prove Armor, Gear or Magic. Acceptance must, per list, count rows that match NO filter value (so an unfiltered row can never be hidden by the filter column) and show that count as `Other`. A row must stay reachable by "All".
- **To verify at planning:** the windowed list keeps keyboard selection, the description box, sort, Only Show Affordable and the `+N` choice working; the header count reads "N of N" with no "first 200" notice; the dialog stays fast on the largest list (state the measured time to first paint, and say it was measured on this machine).
- **Open questions for the operator:** (1) the app's Weapons, Armor and Shields counts (the baseline above); (2) whether a row that matches no filter value is shown under an `Other` value or only under `All` (my pick: `Other`).
- **Baseline from the Market dialog (operator, 2026-11-21; read off the screen, not derived by us):**

  | List | Market shows | Estimate from `equipment_types.json` (not the Market's source) |
  |---|---|---|
  | Magic items | 3,680 | 4,677 |
  | Gear | 1,624 | 3,573 |
  | Weapons | 480 | 1,603 |
  | Armor | under 200 (no "first 200" notice) | 590 |
  | Shields | under 200 (no "first 200" notice) | 231 |

  - Ceiling on the Market's total: 3,680 + 1,624 + 480 + 199 + 199 = **6,182 at most** (Armor and Shields counted at their largest possible value). The source file has 10,674 names. So **at least about 4,500 source names are on no Market list.** Whether that is correct (not ingested books, duplicate keys, templates, Equipmods, tag-only entries with no converted record) or a loss is the open question. Re-derive: the operator's four numbers plus `python3 -c "import json;print(len(json.load(open('data/equipment_types.json'))['types']))"`.
  - Every Market list is below the estimate, Weapons most of all. The shortfall is not one list's rule; look for one cause that hits all five (the estimate file includes names with no converted record, or the catalog build drops them) before looking for five.
  - The counting test in this item must reproduce these five numbers exactly from `built_catalog()`. If it cannot, the test is wrong, not the screen. Armor and Shields need exact values from the test; "under 200" is only a bound.
- **Ruling (operator, 2026-11-21): an `Other` filter value** holds every row that fits no other value in a filter column. A row is always reachable through `All` as well. Acceptance: per list and per filter column, the counts of all values plus `Other` account for every row (a row with two tags can count in two values, so state the rule used and test it). Open questions for item 11: none beyond the count investigation.

## 12. ENHANCEMENT + BUG CHECK: the Market's Inventory panel shows each item's sell value; selling a container moves its contents to Carried

- **Reported:** 2026-11-21.
- **Ask 1 (operator):** on the main Market screen, the Inventory panel shows the **sell value** of each item, based on the pricing option chosen in the Market. It sits in the space **between the description and the Negotiate button**.
- **Ask 2 (operator):** when a **container** is sold, **all the objects in it move to Carried**.
- **Today (read from the code):** `apps/desktop/src/characterHub/MarketDialog.tsx` ~316–336. Each Inventory row is a table row: a `Sell <name>` button, the `×quantity`, then a `Negotiate` button. No value is shown. My reading of "between the description and the Negotiate button" is the cell between the name/quantity and Negotiate; confirm at planning.
- **Reuse, do not re-derive:** `sellNegotiateGp(bundleGp, pieces, baseQuantity, mode)` in `marketModel.ts` ~115 already returns the refund the backend will pay (it mirrors `refund_copper`; `refundPreviewCopper` in `priceMode.ts`). The shown value must come from it, so the number on screen is the number paid. Backend rule: `sell_held_pieces_at_root` in `apps/desktop/src-tauri/src/character_hub.rs` ~5609 (Standard 50%, CharacterBuild 100%, Cashless nothing; a `+N` copy is priced at its Market offer; no catalog price means nothing to refund).
- **Value display rules to settle at planning:**
  - It follows the Market's pricing option live: change the option, the values change.
  - **Cashless:** the value reads `0` or `—` (no money moves); say which.
  - **Item with no catalog price:** `—`, not `0` (nothing fabricated). Sell still removes it, as today.
  - **Stack:** the value is for the whole stack shown on the row, and scales with the pieces; a `+N` copy shows its Market-offer value.
  - Format and unit come from the existing price formatter (`formatMarketPrice`), same as the Market's Price column.
  - Same value formula as the new sell window (item 9). One function, two screens, so the two cannot disagree.
- **Container sale (Ask 2) — UNVERIFIED, needs a failing test first:**
  - I did NOT find, in a quick search of `character_hub.rs`, any code that moves a sold container's contents to Carried. Sell and Discard remove the copy (`sell_held_pieces_at_root`, `discard_equipment_copy_at_root` ~5754). What happens to the contents today is not established. Likely wrong results: contents point at a container that no longer exists, so they vanish from the list or show under a missing parent. This may be the same defect family as item 5's symptoms.
  - Rule (operator): the container's DIRECT contents go to Carried. A nested container's own contents stay inside it (the nested container moves to Carried with its contents).
  - Test first, RED: sell a Backpack holding a Sack that holds a Bedroll, and loose items. Expect: Backpack gone and paid; loose items and the Sack in Carried; Bedroll still inside the Sack; carried weight correct.
  - Applies to Sell on both screens (Market inventory, Gear screen) because both call the same backend. Discard and Drop of a container are different actions and are NOT part of this item; ask the operator (below).
  - Selling a stack of containers: each sold piece is empty by construction (a container with items is never in a stack, item 5), so the stack case has no contents to move. Check that this holds.
  - A container sold while its contents are "Not Carried": they stay Not Carried (they were not in Carried to begin with). My reading; confirm.
- **Proof width, stated up front:** one RED/GREEN case proves one shape. Acceptance must cover: empty container, container with loose items, container with a nested container, a container inside a Bag of Holding, a worn container (a belt pouch on the Belt slot) and its encumbrance after the sale.
- **Open questions for the operator:** (1) Should Drop and Discard of a container also move its contents to Carried? (my pick: yes for Drop, since Drop keeps the item; for Discard, ask: "removes it for nothing" may mean the contents go too); (2) Cashless shows `0` or `—` (my pick: `—`); (3) contents of a sold container that were Not Carried stay Not Carried (my pick: yes).

## 13. ENHANCEMENT: send bug reports and enhancement requests through the codex-web report API

- **Reported:** 2026-11-21 (operator).
- **Ask:** the app sends bug reports and enhancement requests to GitHub through the codex-web API, which
  acts as a proxy. The user never needs a GitHub account or token.
- **Endpoint (operator):** `POST https://campaign-codex.org/api/report`
  - Header `x-codex-key: <key>`. The operator placed the key at `~/workspace/.report_key`, outside the
    repo. The key is never written into this repo or its docs. How it reaches the build (CI secret
    stamped in at build time, like `UPDATE_SIGNING_KEY`) is settled at planning.
  - Header `Content-Type: application/json`.
  - Body: `{"type":"bug","title":"...","body":"...","version":"0.22"}`. `type` is `bug` or
    `enhancement`. `title` and `body` must not be empty. `body` must stay under 8 KB.
    `version` comes from the running build's own version, not a literal.
- **Replies the app must handle without crashing:**
  - `201` `{"ok":true,"id":"..."}`: report saved.
  - `403`: wrong key. `400`: a field is missing or the JSON is bad. `413`: body too large.
  - Also: no network, timeout, any other status. Each one shows a clear message; nothing is lost silently.
- **Wording (operator):** on success the app says the report was **sent for review**. It must NOT
  say that an issue was created.
- **Today (read from the code, not yet traced):** a feedback flow already exists in
  `apps/desktop/src/feedback/` (`submissionState.ts`, `controlledDefectPayload.ts`, browser handoff to
  GitHub). Planning decides whether this API replaces the browser handoff or sits beside it.
- **Check before building:**
  - Enforce the 8 KB limit before sending (count bytes, not characters) and tell the user, rather than
    relying on a `413`.
  - Whether the report should carry diagnostics (OS, install kind, channel) and how that fits in 8 KB.
- **Risk to state:** a key built into a shipped desktop app can be pulled out by anyone who has the
  app. It stops casual misuse, not a determined one. Rate limits on the codex-web side are the real
  protection. Note this in the plan.
- **Open questions for the operator:** (1) replace the browser handoff, or keep both? (2) how the key
  gets into release builds, and what dev builds do without it (my pick: the Send button is disabled
  with a stated reason, not a fake success); (3) attach diagnostics, and which.

## 14. ENHANCEMENT: Update menu layout — channel selector inside the top block, more padding in the text blocks

- **Reported:** 2026-11-22 (operator).
- **Ask 1:** the channel selector is a drop-down that sits above the body of the Update menu. Move it
  into the body of the top block, right next to the `Channel:` label.
- **Ask 2:** the text blocks have a large margin at the top and on both sides. Release notes wrap in odd
  places because of it. Give the text more room (reduce the margin, widen the text area).
- **Code to start from (read from the file list, not yet traced):** `apps/desktop/src/update/ChannelSelector.tsx`,
  `CheckPanel.tsx`, `installedPanel.tsx`, `lastCheckPanel.tsx`, `Ui.tsx`.
- **Check before building:** the release-notes text is the test case. Look at a long note at the app's
  narrowest window and confirm lines wrap at word boundaries.

## 15. ENHANCEMENT: bug report and enhancement screens — direct GitHub stays preferred, an Anonymous selector chooses the API path

- **Reported:** 2026-11-22 (operator). Answers item 13's open question (1): keep both paths.
- **Ask:**
  - Direct submit to GitHub stays the preferred channel.
  - Add a selector on the bug report and enhancement screens: **Anonymous**.
  - Anonymous off: submit directly to GitHub (today's browser handoff).
  - Anonymous on: submit through the codex-web report API from item 13. The user needs no GitHub account.
  - Anonymous on also shows a **Handle** field. Handle is optional, never required. It lets a reporter say who we can thank.
- **Code to start from (not yet traced):** `apps/desktop/src/feedback/` (`submissionState.ts`, `controlledDefectPayload.ts`).
- **Rulings (operator, 2026-11-22):**
  - Screens: the bug report and enhancement screens.
  - Handle goes in the report `body` as the first line. No API change.
  - GitHub unavailable: show a message. The user turns Anonymous on themselves. No silent fallback.

## 16. CLEANUP + DECISION: the Options → Developer screen

- **Reported:** 2026-11-22 (operator).
- **Ask 1:** clear out the **Notes** section. It is old history. Today it shows one line from the backend:
  `"Real GE-08 authoring workbench snapshot from headless substrate."` (`authoring_workbench.rs:480`,
  shown at `apps/desktop/src/App.tsx` ~1033, fed by `loadTesterWorkbenchSurface.ts` `notes:`).
- **Ask 2 (question):** is the screen still worth having?
- **What I found (read, not run):**
  - The tab is `developer` in `SettingsModal.tsx`. Its body is inline in `App.tsx` (~893–1045).
  - It shows: Build, Platform, Backend version + commit, Feature check, Data source, Backend feature
    check, Diagnostics, Blocked calculations, Explanations, Sources, Update & platform support,
    Feedback evidence, Notes.
  - The same `surface` object feeds the **bug** and **enhancement** composers. Those need it. The
    loader must stay even if the screen goes.
  - Most of the rest of the screen is GE-08 era workbench output. A player cannot act on it.
  - Backend version + commit is the one part that is useful in a bug report.
- **Options:**
  1. Remove Notes only. Smallest change.
  2. Slim the screen to build, platform, backend version + commit, update status, and the evidence
     panel. Remove the GE-08 sections. **My pick.** Keeps what a bug report needs.
  3. Remove the Developer tab. Show the version + commit line inside the bug and enhancement screens.
- **Check before building:** grep `surface.notes` and each removed section for other readers; check
  `FeedbackEvidencePanel` does not depend on a section that goes.
- **Ruling (operator, 2026-11-22): option 2.** Slim the screen. Keep: build, platform, backend version +
  commit, update status, and the Feedback evidence panel. Remove: Feature check, Data source,
  Backend feature check, Diagnostics, Blocked calculations, Explanations, Sources, fallback notice,
  Notes. The loader stays because the bug and enhancement composers use it.

## 17. ENHANCEMENT: character screen — the top bar stays fixed while the page scrolls

- **Reported:** 2026-11-22 (operator).
- **Ask:** on the character screen, the top bar holds the menu, the close button, and the character name
  and class, then a line under it. Keep that bar floating at the top. Scrolling moves only what is
  below the line.
- **Addition (operator, 2026-11-22):** the left rail (Character Details) and the right column (abilities
  and skills) also stay fixed. Only the central pane scrolls. The page itself does not scroll.
- **Ruling (operator, 2026-11-22):** same behavior on the Starfinder and Cyberpunk sheets. The top bar
  and the layout stay static on every game system.
- **Code to start from (read, not run):** `apps/desktop/src/characterHub/CharacterSheet.tsx` ~4553. The bar
  is a plain `div` with `borderBottom: 2px solid var(--color-accent)`. The sheet wrapper uses
  `marginTop: -3rem` and `width: 100vw` to escape the page padding. That wrapper may break a plain
  `position: sticky`, and the bar needs an opaque background so content does not show through.
- **Check before building:**
  - The other sheets have their own bars: `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx`.
    Prefer one shared layout piece over three copies.
  - The side columns can be taller than a short window (the skills list is long). Decide whether each
    side column scrolls on its own when it does not fit. My pick: yes, each side scrolls inside its own
    column, so nothing is cut off.
  - The App's own ☰ menu is `fixed` at the top right on screens without a menu (`App.tsx`). Confirm it
    does not overlap the sticky bar.
  - Dialogs and the progression drawer sit above the bar (z-index).

## 18. BUG + CORRECTION: the character sheet's "Open <campaign>" button errors, and the big banner went on the wrong screen

- **Reported:** 2026-11-22 (operator).
- **Ask 1 (correction):** the button to enlarge is the **Open <campaign name>** button on the character
  screen (under Skills). It should fill that space. SD-42 put the large gradient banner on the Campaign
  Manager screen instead (`CampaignManagerScreen.tsx` `OpenCampaignBanner`, commit `cd33e7d82d`).
  Whether the Manager keeps its banner is an open question (below).
- **Ask 2 (bug):** clicking that button shows
  `Failed opening a campaign folder: 234b13c1-d676-4731-ac8c-b93c7a8161fe is not an absolute folder path`.
- **Cause (read from the code, not yet run):**
  - `CharacterSheet.tsx` ~4425 calls `props.onOpenCampaign(membership.campaignId)`. That value is the
    campaign's UUID.
  - The handler in `CharacterHubPage.tsx` ~310 names its argument `campaignDir`, and the backend
    `open_campaign_folder` (`campaign_folder.rs` `require_absolute`) accepts only an absolute path.
  - Since SD-42 a campaign is its folder, so the id-to-path step is missing. The membership record
    probably holds the path (`campaignDir`, seen in `CampaignMembershipDialog.tsx`).
- **Why tests missed it (state in the plan):** check whether a test clicks this button end to end. A test
  that passes a fake path to the handler would not catch an id passed in.
- **Fix shape:** pass the folder path (or resolve id to path in one place), add a failing test first
  that clicks the button with a real membership row, and enlarge the button to fill its space using the
  same `ActionBanner` look.
- **Rulings (operator, 2026-11-22):**
  - The Campaign Manager screen keeps its banner, but smaller. It does not need to be so large.
  - A character in several campaigns gets one full-width button per campaign, stacked.

## 19. CLEANUP: move "Corpus Ingest Diagnostic" off the main screen and into Options → Developer

- **Reported:** 2026-11-22 (operator).
- **Ask:** the main (landing) screen carries a **Corpus Ingest Diagnostic** entry. It is a developer
  tool. Move it to the Developer tab (item 16).
- **Code to start from (read, not run):**
  - Entry point: `LandingScreen.tsx` prop `onCorpusIngestDiagnostic`; wired in `CharacterHubPage.tsx`
    ~136 (`setMode('corpusIngestDiagnostic')`), screen at ~181.
  - Panel: `characterHub/CorpusIngestDiagnosticPanel.tsx` (117 lines). It takes `onClose` and runs as a
    full screen today.
  - Developer tab: inline in `App.tsx` ~893 (`SettingsModal` panel `developer`).
- **Check before building:**
  - The panel is a full screen with its own Close. Inside the Options dialog it should render as a
    section with no Close button. Decide: embed it in the Developer tab, or keep the screen and put a
    button on the tab that opens it. My pick: embed it as a section.
  - Remove the landing-screen entry and the `corpusIngestDiagnostic` mode in `CharacterHubPage.tsx`,
    with no dead code left behind. Grep for other callers and tests first.
  - The panel states its data is the real ingested corpus status (not fixture data). Keep that true.
- **Depends on:** item 16 (the slimmed Developer tab). Build them in the same cycle so the tab is only
  reshaped once.

## 20. BUG: Market names show an unfilled `[NAME]` placeholder — every item needs a proper name

- **Reported:** 2026-11-22 (operator). Example: the Market lists `[NAME] Giant-hide`; only the description
  says it is the Hill Giant-hide.
- **Cause (read from the data):** the PCGen source name carries a `[NAME]` placeholder that PCGen fills
  in per variant. The record's `data.name` keeps the raw text (`OUTPUTNAME` token: `[NAME] Giant-hide`).
  The variant is in `data.key` (`Giant Hide (Hill)`).
- **Scan done (2026-11-22):** `data/corpus/*/equipment/*.json`, field `data.name`, pattern
  `[`, `]`, `%` or the word `NAME`. Re-derive with:
  `python3 - <<'PY'` loop over `glob('data/corpus/*/equipment/*.json')`, load, read `d['data']['name']`,
  match `re.search(r'[\[\]%]|\bNAME\b', name)`.
  - **3,805** equipment records scanned. **111** have a placeholder name. **18** distinct patterns.
  - By book: Advanced Player's Guide 60, Ultimate Psionics 50, Ultimate Combat 1. Same loop, counted by `f.split('/')[2]`.
  - Other patterns: `Skin of the [NAME]`, `Psionatrix of [NAME]`, `Setting Stone of [NAME]`,
    `Companion Stone of [NAME]`, `Greater/Lesser/True Psicrown of [the] [NAME]`, `Psionic Restraint,
    [NAME]`, `Meld Stone of the [NAME]`, `Ioun Stone, [NAME]`.
- **What this scan does NOT cover:** other record kinds (spells, feats, class features), the `label` and
  `OUTPUTNAME` fields, the `_settled` and `rules_tables` copies, the desktop bundle copy
  (`apps/desktop/src-tauri/resources/corpus_bundle/`), and names with other placeholder shapes. A clean
  result on this one pattern does not prove every name is proper.
- **Ask:** scan for these, and give every item a proper name. The Market (and the Inventory panel, item
  12) must show `Hill Giant-hide`, not `[NAME] Giant-hide`.
- **Fix shape (to decide at planning):** derive the display name from the variant (`key`) in one place
  that every screen reads, rather than editing 111 generated records by hand. Corpus files are
  generated, and a regeneration would undo hand edits. Add a test that fails on any equipment display
  name still containing `[`, `]`, or `NAME`, so a new book cannot reintroduce it.
- **Ruling (operator, 2026-11-22), question 1:** I propose names from the book text; the operator
  approves the list before it ships. Read from a one-word reply ("1"): confirm if that was not the intent.
- **Ruling (operator, 2026-11-22):** the "no placeholder in a display name" test covers all item kinds,
  one test.

## 21. BUG: "Show Progression" is cut off at the bottom of the character screen on a shorter display

- **Reported:** 2026-11-22 (operator). On the operator's screen the button is slightly cut off by the
  bottom edge (the OS dock bar). The full-screen window has a few fewer pixel rows than the layout
  assumes.
- **Ask:** the layout must fit the real window height, not an assumed one. The button shows in full.
- **Code to start from (read, not run):** the button is in `apps/desktop/src/characterHub/CharacterSheet.tsx`
  ~4370 (`progressionButtonRef`), inside a panel `div` that sits in normal page flow. The window size is
  set by the Tauri window config (`apps/desktop/src-tauri/tauri.conf.json`, not yet read).
- **Link to item 17:** item 17 fixes the side columns in place. That change must size the columns to the
  real viewport height, with the bottom padding the button needs. Do 17 and 21 in one cycle, so the
  layout is built once. Fixing 21 alone would be undone by 17.
- **Check before building:**
  - Reproduce at a short window height (about 40 px less than the current full-screen). Do not guess the
    dock bar's height. Use the viewport (`100dvh`), not a fixed pixel number.
  - Look for other bottom-edge controls on the same screen with the same fault.
  - Test with the window at the smallest size the app allows.
