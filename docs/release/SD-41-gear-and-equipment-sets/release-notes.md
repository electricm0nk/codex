---
canonical: true
bundle_id: SD-41
slug: gear-and-equipment-sets
tranche: 21
date: 2026-11-16
status: final
---

# Campaign Codex 0.21 — Gear and Equipment Sets

Pathfinder gear gets its own screen: every item has a place, items can be grouped into named equipment
sets, containers hold things, and the Market became a sortable table with bulk buying, Negotiate, Buy
Custom and Create. The printed sheet follows the active set. Characters in a campaign are copied to the
campaign's folder every time they are saved.

Figures are listed under "Figures" as `R-n`, each with the command that produced it. Run the commands
from the repository root. They were run on 2026-11-16 against `tranche-21`.

## Summary

SD-41 adds equipment locations (worn slots, Carried, inside a container, Not Carried) and named
equipment sets to Pathfinder characters. The engine reads only the active set. AC, carried weight, the
Weapons block and the print all follow it. The Market was rebuilt, prices were fixed in the converter,
and the rest of the prices come from a cited d20pfsrd list. Campaign membership writes through to the
campaign folder. Page 1 of the print got the operator's design changes. The bundle has 58 cards; 57 were
`complete` when this file was written, and the 58th, E5.4, is the PR itself (R-1). The operator signed
the three E4.G gate lines `usable: yes` (R-2). Starfinder and Cyberpunk RED gear and markets are
unchanged.

## User-Visible Changes

- **Gear screen (E3.1, E3.2, E3.11).** It replaces the Equipment panel in the Gear tab. The left side
  lists every held item with its weight. You can filter All / Equipped / Not equipped, and there is one
  table per container copy, showing what is directly inside it. The right side is the location tree:
  the Pathfinder body slots, Carried with containers nested inside it, and Not Carried. Each container
  shows its load as "x of y lb". Items move with a "Move to" menu or by drag and drop. Drop, Sell and
  Discard are separate actions.
- **Equipment sets (E1.2, E2.1, E2.2).** You can create, rename, delete and activate named sets
  ("Default", "Formalwear"). The same items can sit in different places in each set. Only the active
  set feeds the engine. The sets are stored in `equipment_sets.json`, a new file beside the saved
  character. The saved input format did not change.
- **Old saves (E2.1).** When an old save is first loaded, it gets one active `Default` set. AC, carried
  weight and load level stay the same. A bundle copy that has no stored quantity counts as one bundle
  of pieces, which is what it weighed before (`old_save_loads_unchanged`).
- **Containers (E1.1, E1.3, E1.12).** An item is a container if its corpus type is `Container` and it is
  not a weapon (operator ruling, "Container tag, but not weapons"). 130 corpus records are settled as
  containers (R-5). The Backpack holds 60 lb. This is an operator ruling recorded in
  `data/capacity_overrides/container-capacities.json`; the PCGen data says unlimited. Containers nest,
  and the load of a nested container counts toward the outer one. The engine computes container loads
  with the same bundle and piece rule it uses for encumbrance.
- **Ignore container capacity limits (E2.3, E3.1, E4.1).** This is a per-character override in the
  Custom tab. With it on, loads are still shown, no container is flagged as over capacity, and the
  sheet and the print say the override is on.
- **Stacks and quantities (E2.1, E2.5).** Identical copies are shown as one row, for example
  "Arrows ×60". Bundles become single pieces once bought: 3 × "Arrows (20)" = 60 arrows. Buy, Sell,
  Discard and Move each take a quantity, and Move can split a stack.
- **Weapons (E1.4, E1.5, E1.6, E2.4, E4.5).** The list shows weapons in hand first, then carried weapons
  marked "(carried)", then an Unarmed strike row, which is always there. The Unarmed row uses the
  engine's real attack and damage numbers (1d3 nonlethal for Medium, 1d2 for Small, the monk progression
  where it applies). A carried weapon shows its attack total as if it were wielded in the main hand.
  Only weapons that use ammunition get ammunition boxes and a carried count, for example
  "Arrow: 20 carried".
- **Climb speed (E1.14, E2.4, E4.3).** The engine now has a `speed.climb` row, read from race data
  (Vanara 20 ft). The Skinwalker's bestial-form climb is printed as Change Shape text and is not modelled
  (`decisions.md §36`).
- **Market (E2.5, E2.6, E2.12, E3.4, E3.12).** The Market is a table with Item, Price, Weight and Slot
  columns. Every column sorts. A description box shows the selected item. "Only Show Affordable" filters
  by the character's funds in the active price mode. Other additions:
  - a Quantity field;
  - Negotiate on Buy and on Sell;
  - **Buy Custom**: start from a base item and choose a name, masterwork, enhancement +1 to +5, special
    abilities, a material and a size (Small or Large). Price and weight come from the rules;
  - **Create**: a free-text item with a weight, an optional price, an optional container capacity and
    an optional slot.

  Market purchases land in Carried.
- **Prices (E1.8, E1.9, E1.11, E1.15).** The converter now prices composite and eqmod-priced rows the way
  PCGen's data defines them. Market rows with no price went from 1,763 to 397 after that fix (E1.9
  receipt). 41 more prices come from a cited d20pfsrd list, one URL per item (R-6). The 356 rows that
  are still unpriced stay in the Market as "no price yet" and are bought through Negotiate. Each of them
  lists every source that was tried (R-6). No Market price is negative.
- **Campaign folders (E1.10, E2.7, E2.13, E3.5).** Each character keeps a `campaigns.json` file. Every
  save, including the sidecar files, copies the whole character folder to
  `<campaign dir>/characters/<slug>-<characterId>/` for each active campaign. Cyberpunk RED characters
  are copied too. A failed copy shows as a warning on the sheet, and the local save still stands. The
  Campaign field on the sheet opens a dialog for adding or removing campaigns. Removing a campaign keeps
  the folder and stops further updates to it. Under Skills there is one "Open <name>" button per
  campaign, which opens that campaign in the Campaign Manager.
- **Print (E4.1–E4.5).**
  - The Inventory page prints the active set by location: worn items by slot; then Carried, with each
    container's contents nested and its load shown; then Not Carried, as names only. Stacks print with
    their counts.
  - Page 1 changes:
    - the 11 section titles are filled bars with white letters;
    - Resistances sits above an empty Conditional Bonuses box;
    - the Skills table has shaded alternate rows;
    - the Weapons block has no blank lines;
    - Speed is a 2×2 grid with Climb ("—" when the character has no climb speed);
    - the portrait box prints the stored portrait, or else the game system's logo, and never the
      word "portrait". The Pathfinder, Starfinder and Cyberpunk RED logos are in git and cited in
      `logos/SOURCES.md`.
  - Pathfinder golden page counts did not change (R-7).
- **Starfinder and Cyberpunk RED.** Their gear and markets are unchanged. Their golden print text and page
  counts are byte-identical to the base (E4.1 and E4.MC receipts).

## Defects Fixed

- **Drop made the item disappear (`decisions.md §27`).** Drop now moves the item to Not Carried. On the
  Weapons tab, Drop on a carried row used to sell the copy in hand. It now moves that row's own copy
  (E2.MC).
- **Missing and negative Market prices (`decisions.md §26`).** For example, Adamantine Breastplate had no
  price and Astral Armor was −150 gp. Both are now priced as base plus their eqmods (E1.9).
- **Two weight sums disagreed.** A container's load and the character's encumbrance weighed stacks and
  created items differently: in the same case, the container read 3.0 lb and the character 14.0 lb. Both
  sums now share one bundle rule (E1.MC).
- **A moved AppImage blocked every update (`decisions.md §37`).** An installed-state record that names
  another path is now rewritten to the running path (E3.6).
- **Not-items shown as items.** The feat toggle `power_attack` and the no-shield row `item:shield` no
  longer appear as unweighed or unshown items on the Gear tab (E4.6).
- **Buy Custom option names.** They had PCGen's leading `|` and type-tag prerequisite lists. Both are now
  shown in plain words (E3.12).
- **Smaller fixes.** The gear screen labelled a 60-arrow stack `—` (E4.MC). The Market's description box
  was off screen (E3.MC). The corpus literal sweep could not re-derive a settled price; it is now a
  passing `verify.sh` stage (E5.5).

## Operational Notes

- **Version.** The repo value is `0.21.0` on both desktop manifests (R-3).
  `docs/release/current-release.json` points to this file:
  `python3 tools/release/resolve_release_notes.py --repo-root .` prints its path.
- **New files on disk, per character:** `equipment_sets.json` and `campaigns.json`. `custom.json` gains
  `weight_lbs`, `price_gp`, `container_capacity_lbs` and `ignore_container_capacity`. These fields use
  serde defaults, so an old `custom.json` loads unchanged.
- **New data inputs, used on the tool side only:**
  - `data/price_overrides/d20pfsrd-prices.json`;
  - `data/price_overrides/price-unknown.json`;
  - `data/capacity_overrides/container-capacities.json`.

  The live app reads only the converted data: `_settled/equipment.json` and the compiled Market tables.
- **Commands.** 12 Tauri commands are new and none were removed: 116 → 128 (R-4).
- **Seeds.** The three Pathfinder seed renders are `SAME_EXCEPT_NAMED` against the Phase-2 base at each
  merge check (G-1, E2.MC and E4.MC reports in `progress.md`).

## Verification Evidence

- **Widest verify (E5.1, on `34d8e7f593`).** `bash scripts/verify.sh --full --show-actuals -j 8` gave
  `RESULT: PASS`: 53 stages passed, 0 failed, and there were 0 `test result: FAILED` lines (R-8). Test
  counts: root lib 3141, root full 6839, ingest 1880, desktop 1507, frontend 198 of 198 files, and
  clippy 0 warnings and 0 errors in all three workspaces.
- **Clean build and fresh install (E5.1).** A clean worktree built `Codex_0.21.0_amd64.deb` and the
  AppImage (`BUILD_EXIT=0`). Both were installed into empty profiles, and in both Settings → Update →
  Check showed `check ok`, with no `unknown` row after the check (screenshots in
  `artifacts/epic_5/E5.1/`).
- **Final-acceptance scan (E5.2, on `11ade49a54`).** Every G-8 line passed. The price gate
  `market_price_census` passed 3 tests with 0 failed. `price-unknown.json` has 356 rows, and all 356
  carry `tried`.
- **Operator gate (E4.G, 2026-11-16).** The operator signed all three lines `usable: yes`: the gear
  screen, the Inventory page, and page 1 (R-2). The operator accepted the gear screen without looking at
  it ("will review in the release build"). The Inventory page and page 1 were judged from the PDFs in
  `artifacts/epic_4/E4.G/`.
- **ui-smoke.** The spec grew from 182 rows to 203 (R-9). E3.3 and E3.11's 30 rows were 30/30 green at
  E4.MC.
- **After the widest verify.** One code card, E3.12 (`74603b2036`), landed after E5.1's run. Its own
  receipt reports the desktop full suite with 0 failed, frontend 198 of 198, the clippy stage passing,
  and `sd24_wired_integration_audit` 5 passed. `verify.sh --full` was not re-run on the final tree.

## Known Issues

- **356 Market rows have no price from any source (R-6).** They show "no price yet" and are bought by
  typing a price in Negotiate (`decisions.md §30.1`). Each row lists the sources that were tried
  (`converter`, `d20pfsrd:<url>`).
- **Deferred by ruling (`forward-scope-register.md`).**
  - Weight-reduction containers (bags of holding, efficient quivers) have no stated capacity, so their
    contents weigh in full (DEF-1).
  - ui-smoke has no drag step. Drag is covered by a unit test (DEF-2).
  - You cannot print a set without activating it (DEF-3).
  - There is no preview of a set's AC or weight before you activate it (DEF-4).
  - Eqmod formulas the converter does not evaluate are offered in Buy Custom without a price and are
    bought through Negotiate (DEF-5).
- **Gear screen not reviewed by eye.** The operator has not yet looked at the gear screen; that review
  waits for the release build (E4.G).
- **The Backpack's 60 lb is an operator ruling, not a book figure.** No PF1e book in the pinned data
  prints a capacity for the Backpack (`decisions.md §32`).
- **Item size in Buy Custom is Small or Large only**, one step from Medium (`damage_total.rs`
  `weapon_damage_for_size`, a recorded `shortcut:`).

## Does not cover

- macOS and Windows builds. The packaged app was checked on the Update panel only; no real update was
  installed.
- The cited d20pfsrd prices are checked for shape (a URL and a fetch time). Whether each price matches
  its page today is not checked.
- Only the Aldric and Elowen print sets were rendered for the operator gate (4 PDFs). Other seeds are
  covered by the golden and smoke tests only.
- R-1 changes with every later card. It is pinned to the tree at E5.3's closing commit.

## Figures

- **R-1** Kanban rows, and rows `complete`, at E5.3's closing commit: **58 57**.
  `awk -F'|' '{ gsub(/\\\|/, "") } $2 ~ /^ (C|E)[0-9]/ {n++; if ($5 ~ /^ complete *$/) c++} END{print n, c}' docs/release/SD-41-gear-and-equipment-sets/kanban.md`.
  The one row that is not `complete` is E5.4: graphify and the PR.
- **R-2** Operator `usable: yes` lines: **3**.
  `grep -c '^- \(Gear screen\|Inventory page\|Page 1 ([^)]*)\) — usable: yes' docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_4/E4.G_cycle_receipt.md`.
- **R-3** Desktop manifest versions: **0.21.0,0.21.0**.
  `grep -h -m1 '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'"' '{print $4}' | paste -sd,`.
- **R-4** Registered Tauri commands: **128**, up from 116 at the merge base `83a8a4f414`. Count the
  comma-split names of `generate_handler![…]` in `apps/desktop/src-tauri/src/main.rs`, with `//`
  comments stripped (Python `re.search(r'generate_handler!\[(.*?)\]', t, re.S)`). The 12 new commands are
  `activate_equipment_set`, `create_custom_item`, `create_equipment_set`, `delete_equipment_set`,
  `discard_equipment_copy`, `list_custom_equipment_options`, `move_equipment_copy`,
  `purchase_custom_equipment`, `quote_custom_equipment`, `rename_equipment_copy`,
  `rename_equipment_set` and `set_character_campaigns`; none were removed. These come from `comm` over
  the two sorted lists.
- **R-5** Corpus records settled as containers: **130**. The
  `container_capacity_population_is_210_88_73_49` pin and its tally of (130, 33, 49, 48) are quoted in
  `decisions.md §20`, E1.12 row.
- **R-6** Unpriced Market rows listed in `price-unknown.json`, and rows with a `tried` list: **356 356**.
  `python3 -c "import json;u=json.load(open('data/price_overrides/price-unknown.json'));print(len(u),sum(1 for r in u if r.get('tried')))"`.
  A second count: `awk '/"tried"/{n++} END{print n}' data/price_overrides/price-unknown.json` gives
  356. Cited d20pfsrd prices: **41**.
  `python3 -c "import json;print(len(json.load(open('data/price_overrides/d20pfsrd-prices.json'))))"`.
  Keys across both files: `awk '/"key"/{n++} END{print n}' data/price_overrides/price-unknown.json data/price_overrides/d20pfsrd-prices.json`
  gives 397, which is 356 + 41.
- **R-7** Pathfinder golden page counts before → after E4.1: a4/letter all 14 → 14, math 11 → 11,
  none 5 → 5, spellbook 6 → 6, spellbookOnly 1 → 1. Source: the E4.1 receipt, "Golden deltas"
  (`pdfinfo` over `PRINT_GOLDEN_OUT` renders).
- **R-8** `grep -c '    PASS' docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_5/E5.1/verify-final-full.log`
  gives **53**, and `grep -c '    FAIL'` on the same log gives 0.
- **R-9** ui-smoke spec rows: **203**.
  `python3 -c "import json;print(len(json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']))"`.
  The count was 182 before E3.3 (`decisions.md §20`, E3.3 row).

## Update Eligibility

Update eligibility is set at publish time by the release manifest (`tools/release/check_release_manifest.py`).
The repo keeps the version placeholder until then. This document does not decide eligibility. The
installed-state rewrite (E3.6) changes how an AppImage that has been moved is updated: the record is
rewritten to the running path instead of blocking the update. The first tester publish of 0.21.x comes
after the operator merges `tranche-21`.
