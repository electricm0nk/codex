# E0.4 — gear defects reproduced (intake items 5, 7, 12)

Run 2026-11-22 on a fresh Brother Tomas Vane seed, in the real app, by the E0.4 agent (lane `gear`).

- **App:** `apps/desktop/.claude/skills/run-desktop/driver.sh launch` from the gear lane tree at base `26bd0b7cd9`
  (`RUN_DESKTOP_AGENT=sd43-e0.4-r2`, Xvfb `:77`, 1920x1200).
- **Isolated app data:** `RUN_DESKTOP_DATA_ROOT=<gear lane tree>/target/e04-appdata` (gitignored `/target`). The app
  seeded its three starters there itself; Tomas is `characters/00000000-0000-0000-0000-000000000003/`. The
  operator's store was never opened.
- **Setup (Market, Cashless):** bought Backpack, Sack ×4 (four separate Buy clicks, quantity 1), Pouch (Belt),
  Bedroll, Blanket, Bag of Holding (Type I). Saved lines after the buys (`grep equipment= authoritative_character_input.txt`):
  `Backpack`, `Sack` ×4, `Pouch (Belt)`, `Bedroll`, `Blanket`, `Bag of Holding (Type I)`, all `selected_inactive`.
- **Stored rows:** after each step the agent copied `equipment_sets.json` to `gear-repro/stored-rows/sets-<step>.json`
  (and the `equipment=` lines to `input-S*.txt` for the sale). The rows below give the active set's (`Default`)
  non-`item:` entries as `<copy> -> <location>`, where a copy key is `itemId|bonus|ordinal`. They were printed by the
  Python one-liner in the receipt over those files. Carried is stored as **no entry** (`decisions.md §5.3`).
- **Screenshots:** `gear-repro/<step>.png`, cropped to the sheet's centre column (x 300–1570) and reduced to 48
  colours. Left panel = list + container tables; right panel = the slot/section tree.
- **Moves:** "Move to" is the row's menu. "Drag" is a real HTML5 drag made with `xdotool` (mousedown, 20 steps,
  mouseup) in the webview; D6 is the positive control that shows the harness drags work.

## Classification key

- **display bug:** the stored rows are right and the screen draws them wrong.
- **data bug:** the stored rows are wrong (a write went to the wrong place, or the wrong copy was written).
- **does not reproduce:** the steps tried are listed, and so is what was not tried.

## Item 5 — gear screen containers

### 5.N — the `Sack #1` / `Sack #2` names (operator's "likely cause")

- **Steps:** E0 baseline: Sack ×4 Carried. E1: Bedroll → Move to → `Into Sack`. E2: Blanket → Move to → `Into Sack #1`.
- **Stored rows (E2):** `Bedroll|0|0 -> inside Sack|0|0; Blanket|0|0 -> inside Sack|0|0`; `copies: {}`, so no copy has a stored name.
- **Seen:** the left panel's container tables are titled `Sack #1 · 6 lb` (the sack holding 2 items) and `Sack #2 · 0 lb`
  (the stack of three, **one table**), and the Move to menu offers `Into Sack #1` / `Into Sack #2`. The right panel
  says `Sack` and `Sack ×3`. The two panels name the same copies differently (`E1-bedroll-into-sack.png`,
  `E2-blanket-into-sack1.png`).
- **Classification: display bug.** The rows are right: the sack holding items is its own copy, `Sack|0|0`, and the
  three empty sacks hold nothing. The `#N` comes from `copyTitle` (`apps/desktop/src/characterHub/gearScreenModel.ts:324–329`),
  which numbers container rows that share a name. A stack of N empty copies is one table and one Move to target,
  bound to its first copy (`:312`, `:342–343`).
- **Put into a stacked container (Q-5.1):** E1 put the Bedroll "Into Sack" while all four sacks were one stack
  (`Sack ×4`, a single table in `E0-baseline.png`). The write went to `Sack|0|0` (the stack's first copy), and the
  next listing split that copy off as `Sack` beside `Sack ×3` (`holds_anything`, `character_hub.rs`). The data
  already splits. No stack holds items in any stored row of this run.

### 5.1 — "Pouch, Belt and Sack #2 still show inside the Backpack after the operator removed them"

- **Steps:** E3: Pouch, Belt → Move to → `Into Backpack`. E4: Sack ×3 → Move to → `Into Backpack`, how many 3.
  E5: Pouch (in the Backpack table) → Move to → `Carried`. E6: Sack ×3 (in the Backpack table) → Move to → `Carried`, 3.
- **Stored rows:**
  - E4: `Pouch (Belt)|0|0 -> inside Backpack|0|0; Sack|0|1 -> inside Backpack|0|0; Sack|0|2 -> inside Backpack|0|0; Sack|0|3 -> inside Backpack|0|0` (+ Bedroll, Blanket in `Sack|0|0`)
  - E5: the Pouch entry is gone (Carried); the three sacks are still `inside Backpack|0|0`
  - E6: only `Bedroll|0|0 -> inside Sack|0|0; Blanket|0|0 -> inside Sack|0|0` remain
- **Seen:** after E5 and E6 the Backpack table reads `Empty`, both panels show Pouch and Sack ×3 under Carried, and
  the Backpack load returns to `0 of 60 lb` (`E4-sacks-into-backpack.png`, `E5-pouch-out-of-backpack.png`, `E6-sacks-out-of-backpack.png`).
- **Classification: does not reproduce** through Move to (whole stack and single copy). **Not tried:** dragging an
  item out of a container, removing through Sell or Discard, an equipment set other than `Default`, and the
  operator's own save (not on this machine). The **sale defect in 12.2** (Sell removes a *different* copy of the same
  item) is one path that leaves a copy where the player did not expect it, but this run did not show it causing 5.1.

### 5.2 — "Sack #1 and Sack #2 both show inside the Bag of Holding"

- **Steps:** E7: the sack holding Bedroll + Blanket (`Sack #1`) → Move to → `Into Bag of Holding (Type I)`.
  E8: Sack ×3 → Move to → `Into Bag of Holding (Type I)`, how many **1**.
- **Stored rows:**
  - E7: `Sack|0|0 -> inside Bag of Holding (Type I)|0|0` (+ its Bedroll, Blanket); Sack 1–3 have no entry (Carried)
  - E8: also `Sack|0|1 -> inside Bag of Holding (Type I)|0|0`
- **Seen:** after E7 only the holding sack is in the Bag of Holding (`E7-sack1-into-boh.png`). After E8 there are two
  sacks in it, and two left as `Sack ×2` Carried. The tables are now `Sack #1` (holding), `Sack #2` (the empty one in
  the Bag) and `Sack #3` (the ×2 stack). The Bag's own table lists both sacks as plain `Sack` (`E8-one-sack-split-into-boh.png`).
- **Classification: does not reproduce** as a wrong picture: after E8 both sacks really are in the Bag. The
  labelling is the 5.N display bug. Three sack tables numbered #1–#3 while the Bag's table says `Sack` twice cannot be
  told apart on screen. **Not tried:** the same moves by drag.

### 5.3 — "the right panel lists one sack (with a bedroll and a blanket) as Not Carried, and Sack ×3 also as Not Carried"

- **Steps:** E11: the holding sack (in the Bag) → `Drop`. D2: drag the Carried empty `Sack` onto the **Bedroll row**
  under that sack in the right panel's Not Carried section (see 7.2).
- **Stored rows:**
  - E11: `Sack|0|0 -> notCarried`; Bedroll and Blanket stay `inside Sack|0|0`
  - D2: adds `Sack|0|3 -> notCarried`, although the drop was aimed inside `Sack|0|0`
- **Seen:** the right panel shows `Not Carried › Sack 6 lb › Bedroll, Blanket` (`E11-drop-holding-sack.png`). After D2
  a second, empty `Sack` sits under Not Carried as its own row (`D2-drop-on-child-row-goes-to-section.png`).
- **Classification: data bug** (D2). A drop meant for a container lands in the enclosing section and writes
  `notCarried`. That gives exactly the operator's picture: a filled sack and loose empty sacks all under Not Carried.
  E11 is the Drop button working as captioned. The screen matched the stored rows at every step. Carried weight left
  out the Not Carried copies (`Gear · 47 lb carried` after the Pouch went into the Not Carried sack, down from 47.5).

### 5.R — "a renamed copy still shows `Sack #2`"

- **Steps:** E9: the empty sack in the Bag → Customize → Name `Custom Sack Name` → Save name. E10: the `Sack ×2` row →
  Customize (the dialog is titled `Customize Sack ×2`) → Name `Stack Renamed` → Save name.
- **Stored rows:** E9 `copies: {"Sack|0|1": {"name": "Custom Sack Name"}}`. E10 adds `"Sack|0|2": {"name": "Stack Renamed"}`.
- **Seen:** E9: `Custom Sack Name` is the table title, the Bag's row and the right panel's row, on **both** panels
  (`E9-rename-split-copy.png`). E10: the stack split into `Stack Renamed` (its own row and table) and a plain `Sack`.
  The remaining same-name tables were renumbered (`Sack #3` became `Sack #2`) (`E10-rename-stack-copy.png`).
- **Classification: does not reproduce.** The custom name shows on both panels. Two related findings:
  (a) **display bug:** the `#N` of the *other* sacks changes whenever one is renamed or moved, so a player who reads
  `Sack #2` before a rename finds a different sack under that title after it. That is the likely source of the
  report, but it was not proven to be. (b) The dialog titled `Customize Sack ×2` renames **one** copy, `copies[0]`, and
  splits it off the stack. The data is consistent with that, but the title does not say only one copy will be renamed.

## Item 7 — drop places

Each drag was made from a left-panel row. "Stored" is the diff of `equipment_sets.json` before and after.

| # | Dragged → dropped on | Expected (intake 7) | Stored | Seen | Classification |
|---|---|---|---|---|---|
| D1 | Pouch, Belt → the right panel's **container name row** `Sack` (Not Carried) | into Sack | `+ Pouch (Belt)\|0\|0 -> inside Sack\|0\|0` | Pouch under Sack on both panels (`D1-drop-on-container-name-row.png`) | works |
| D2 | Sack (Carried, empty) → the right panel's **Bedroll row inside** that Sack | into Sack | `+ Sack\|0\|3 -> notCarried` | a new `Sack` row directly under Not Carried (`D2-…png`) | **data bug**: lands in the enclosing section |
| D3 | Stack Renamed → the **left panel's `Bedroll` row inside the `Sack #1` table** | into Sack #1 | unchanged (`cmp sets-D2.json sets-D3.json` → equal) | nothing happens | **display bug** (no drop zone; nothing written) |
| D4 | Stack Renamed → the **`Empty` cell of the left `Backpack` table** (box space) | into Backpack | unchanged (`cmp` D3 D4 → equal) | nothing happens | **display bug** (no drop zone) |
| D5 | Stack Renamed → the **caption (title line) of the left `Sack #1` table** | into Sack #1 | unchanged (`cmp` D4 D5 → equal) | nothing happens | **display bug** (no drop zone) |
| D6 | Stack Renamed → the right panel's **`Backpack` name row** (positive control) | into Backpack | `+ Sack\|0\|2 -> inside Backpack\|0\|0` | Stack Renamed under Backpack (`D6-positive-control.png`) | works: the harness drags work, so D3–D5 are real no-ops |

- **Mechanism (read from the code; it agrees with every row above):** `GearScreen.tsx` gives a drop zone only to a
  **container's own row** (`:397` tree, `:414` list/table row) and to each **right-panel section** (`:543`). The right
  panel has no box. A container's children are sibling `div`s inside the section, so a drop on a child row bubbles
  to the section zone, and `dropTarget` resolves it to the section (D2). That is a wrong move when the item is not
  already in the section; when it is, the drop is refused and nothing is written. The left panel's container
  `<table>`s (caption, rows, `Empty` cell) have **no zone at all** (D3–D5).
- **Not tried:** dropping onto a child row nested two levels deep (nearest-container rule), dropping a container into
  its own descendant over the wider area, and keyboard moves.

## Item 12 — container sale today

### 12.1 — selling a Backpack that holds a sack (with a Bedroll) and a loose Blanket

- **Steps (Gear screen, Cashless):** D6 put `Stack Renamed` (`Sack|0|2`) in the Backpack, then drags put Blanket in
  the Backpack and Bedroll in `Stack Renamed` (`S0-before-sale.png`, `Gear · 52.5 lb carried`). Backpack → `Sell` → `Sell`.
- **Stored rows:**
  - S0: `Sack|0|2 -> inside Backpack|0|0; Blanket|0|0 -> inside Backpack|0|0; Bedroll|0|0 -> inside Sack|0|2`
  - S1: both `inside Backpack` entries are gone (Carried); `Bedroll|0|0 -> inside Sack|0|2` stays
  - saved lines: `- equipment=Backpack:selected_inactive` (`diff input-S0.txt input-S1.txt`)
- **Seen:** Carried › `Stack Renamed` › `Bedroll`, and `Blanket` Carried; `Gear · 50.5 lb carried` (52.5 − the 2 lb
  Backpack) (`S1-after-backpack-sale.png`).
- **Classification: does not reproduce the intake's feared defect.** The contents do not vanish. The sale removes the
  line, and `forget_copy_in_sets` (`character_hub.rs:6751`) drops every set entry inside the removed copy, so its
  direct contents become Carried and a nested container keeps its own contents. For this shape, today's behaviour
  is item 12's rule. Retro correction `1795366852155-sd43-e0.4-ce9e6c`.

### 12.2 — selling one of several same-item containers sells a different copy (**new defect**)

- **Steps:** with four Sack lines (`Sack|0|0` Not Carried holding the Pouch; `Sack|0|1` "Custom Sack Name" inside the
  Bag; `Sack|0|2` "Stack Renamed" Carried, holding the Bedroll; `Sack|0|3` Not Carried), `Stack Renamed` → `Sell` → `Sell`.
- **Stored rows:**
  - S1: `Sack|0|1 -> inside Bag of Holding (Type I)|0|0`; `Bedroll|0|0 -> inside Sack|0|2`; copies `Sack|0|1` = Custom Sack Name, `Sack|0|2` = Stack Renamed
  - S2: `Sack|0|0 -> notCarried; Pouch (Belt)|0|0 -> inside Sack|0|0; Sack|0|2 -> notCarried; Bedroll|0|0 -> inside Sack|0|1`; copies `{"Sack|0|1": {"name": "Stack Renamed"}}`
  - saved lines: the first `equipment=Sack:selected_inactive` line was removed
- **Seen:** `Stack Renamed` is still there, still holding the Bedroll, and `Custom Sack Name` has vanished from the Bag
  (`S2-sell-sold-the-wrong-sack.png`).
- **Classification: data bug.** The clicked copy was `Sack|0|2`, but `Sack|0|1` was removed (`Sack|0|2` was then
  renumbered to `|1`). The Gear screen's Sell (`CharacterSheet.tsx` `handleGearSell`) sends `itemId`, `bonus` and
  `fromState`, not the copy key. `sell_held_pieces_at_root` (`character_hub.rs:5635`) then takes
  `first_copy_in_hand` (`pf1_adapter.rs:1349`), the first saved line of that item in that state. Drop, Discard and
  Move send `row.held.copies[0]` (`GearScreen.tsx:675–677`) and act on the right copy. When the wrong copy is a
  container, its contents go to Carried (12.1's mechanism) and the clicked container keeps its contents. Item 12's
  container rule cannot hold while Sell picks the copy this way. Filed as discovered card **E1.12**.

## What this run does not cover

- The operator's own save (not on this machine; epic-breakdown E0.4 "Does not cover").
- Buy 100 % / Sell 50 % and Character build pricing: every sale ran under Cashless, so no refund amounts were checked.
- Equipment sets other than `Default`, the SF and CPR sheets, clicking the Market dialog's own Sell button (it was not clicked in the app;
  in the code, `handleMarketSell` sends the same `itemId` + `bonus` + `activeState` shape, so 12.2 is expected there
  too), Discard of a container, a worn container
  (Pouch on the Belt slot: `Pouch, Belt` shows Slot `—` in the Market), and Not Carried contents of a sold container (Q-12.3).
- Drag-based moves for 5.1 and 5.2, nested-two-deep drops, keyboard moves.
- One character and one item family (Backpack, Sack, Pouch, Bag of Holding Type I). Other containers with a stated
  capacity (Backpack 60 lb) and the Bag's "contents weigh nothing" were seen only as labels.
