---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — front half; every claim names the file it was read from at `e451225461`; parts not read are marked "to verify" with the command
---

# SD-43 Technical Design

> `decisions.md` wins where this file differs. Read, not run, unless a command's output is quoted.

## 1. Today's module map (the files SD-43 changes)

Line counts: `wc -l` over the files in the first column, on `e451225461`.

| File | Lines | What it holds that SD-43 touches |
|---|---|---|
| `apps/desktop/src/characterHub/CharacterSheet.tsx` | 4,762 | `SHEET_TABS` `:579`; `FreeTextTab` `:620`; Weapons/Defense/Gear panels (Market `:851`/`:1396`/`:1617`, `PriceModeControl` `:849`/`:1615`, Print `:854`); `RacialTraitsSection` `:1689`; `POOL_REFERENCE_SECTIONS` `:1798`; `ClassFeaturePoolReferenceSection` `:1833`; `ClassFeatureDescriptionReferenceSection` `:1914`; `FeatsTab` `:2343`; `TraitsSection` `:2516`; `priceMode` state `:2890`; ☰ menu items with Print `:4153`; Show Progression `:4370`; Open campaign `:4425`; wrapper and bar `:4553`/`:4558` |
| `apps/desktop/src/characterHub/GearScreen.tsx` | 688 | caption `:31`; `PendingForm` `:173` (Sell's "pricing above" `:188`, `:205`); `dropZone` `:362`, used `:397`, `:414`, `:543`; `stopPropagation` `:353`, `:366`, `:372` |
| `apps/desktop/src/characterHub/gearScreenModel.ts` | 390 | `WORN_SLOTS` `:11`; `isContainer` `:225`; move targets bound to `copies[0]` `:312`; `copyTitle` with `#N` `:324`; `containerTables` bound to the first copy `:342`; `dropTarget` `:363` |
| `apps/desktop/src/characterHub/MarketDialog.tsx` | 686 | the "first 200" notice `:248`; description box ~`:305`; Inventory rows ~`:316–336`; imports `MARKET_ROW_LIMIT`, `sellNegotiateGp` |
| `apps/desktop/src/characterHub/itemPickerFilter.ts` | 379 | `MARKET_ROW_LIMIT = 200` `:362`; `marketShownRows` `:365` |
| `apps/desktop/src/characterHub/marketModel.ts` | 121 | `sellNegotiateGp(bundleGp, pieces, baseQuantity, mode)` `:115` |
| `apps/desktop/src/characterHub/priceMode.ts` | 37 | `PriceMode` `:5`; `PRICE_MODE_OPTIONS` `:9` (not edited) |
| `apps/desktop/src/characterHub/characterProgression.ts` | 422 | `FeatBudgetView` `:215`; `readFeatBudget` `:228`; `withBudgetOverride` `:252` |
| `apps/desktop/src-tauri/src/character_hub.rs` | 25,106 | `feat_budget_for` `:951`; `feat_budget_rows` `:1019`; `refund_copper` `:4098`; `sell_held_pieces_at_root` `:5635`; `discard_equipment_copy_at_root` `:5754`; `describe_held_equipment_at_root` `:7126` |
| `apps/desktop/src-tauri/src/character_custom.rs` | 949 | `custom.json`; `Overrides` `:131` (nine flags); `CustomRecordDto` `:99` |
| `apps/desktop/src-tauri/src/equipment_catalog.rs` | 2,692 | `build_equipment_catalog` `:527`; `market_category_of` `:709`; `hidden_from_market` `:736`; `market_from_catalog` `:807`; `built_catalog` `:834` |
| `apps/desktop/src-tauri/src/class_feature_pool_picker.rs` | 156 | `list_class_feature_pool_options` (registered `main.rs:412`) |
| `apps/desktop/src-tauri/src/race_trait_picker.rs` | 2,508 | `list_alternate_racial_traits` `:1067`; `resolve_race_alternate_selection` `:1075` |
| `apps/desktop/src-tauri/src/campaign_folder.rs` | 1,217 | `require_absolute` `:152`; `create_campaign_folder_at` `:200`; `open_campaign_folder_at` `:219`; `open_campaign_folder` `:546` |
| `src/rules_core/feat_budget.rs` | 819 | the model for every ability budget (§3.2) |
| `src/rules_core/class_feature_pool_catalog.rs` | 1,478 | the pool catalog; `REGISTERED_POOL_GROUPS` `:198` (documentation only); `is_registered_pool_group` `:209` |
| `src/rules_core/archetype_resolver.rs` | 396 | `ARCHETYPE_CHOICE_ID = "choice:archetype"` `:41`; `archetype_claims_slot` |
| `src/rules_core/character_input.rs` | 1,088 | `ChosenCharacterState` `:17` (`selected_feats`, `selected_traits`, `selected_choices`, …) |
| `apps/desktop/src/App.tsx` | 1,052 | the Developer tab body (~893–1045; Notes `:1033`) |

## 2. Market and inventory (E1)

### 2.1 Containers and the one-column list

**Today (read):** the gear model builds container rows whose `held.copies` may hold several identical copies; a
container row's move target and its table are bound to `copies[0]` (`gearScreenModel.ts:312`, `:342–343`), and
`copyTitle` numbers same-named container rows `#1`, `#2` (`:324–329`). The operator's symptoms (items shown in the
wrong container; a renamed sack shown as `Sack #2`) fit this: a stack of three empty sacks is one row bound to one
copy, and its title comes from the stack's label, not the copy's `CopyFacts.name`. Whether the *stored* data is
also wrong (the move did not write) is what E0.4 establishes; the design below covers both.

**After E1.1:** a container row is built per copy when the copy holds items or carries a custom name; only
identical, empty, un-renamed copies collapse into one `Name xN` row; the title is the copy's own name; no `#N`.
Putting an item into a stack row first splits one copy off (a backend split if the stored model needs it — E0.4
decides), then moves the item into that copy. Emptying a container leaves its row separate.

**After E1.2:** `GearScreen` renders one list per location: a sort key `(isContainer, collator.compare(name))`
with `Intl.Collator(undefined, {numeric: true, sensitivity: 'base'})`, applied at every depth; each container
renders as a box (today's styling re-derived first) with its children indented inside; Not Carried is a separate
section whose items the engine already excludes from `totalCarriedWeightLbs` (to verify in E1.2's test: the
engine's exclusion is the claim, the test proves it with a Not Carried item present).

**E1.3:** every element inside a container box spreads `dropZone(`inside-${copyId(copy)}`)` (the shape `GearScreen.tsx:397` already uses) for the nearest container; child
container boxes override with their own (the nearest wins because `stopPropagation` at `:366`/`:372` stops the
bubble at the first zone hit).

**E1.4:** `doubleClickTarget(row, model) → {location} | {refused: reason}` is pure; the handler calls the same
Move-to write as the menu. The keyboard path binds Enter on the row's focus.

### 2.2 Sell, sell values, container sales

- `SellDialog` takes `(row, quantity, mode)`; mode is local state initialised to `standard`; its amount is
  `sellNegotiateGp(bundleGp, pieces, baseQuantity, mode)`; Negotiate passes the typed gp to the existing
  `onSell(row, quantity, negotiatedGp)`. The Market inventory column calls the same function with the Market's
  `priceMode`. One function, two screens (`decisions.md §10`).
- Container sale (E1.5): inside `sell_held_pieces_at_root`, before removing a container copy, every copy whose
  location is `inside(this copy)` is relocated to Carried unless it is Not Carried; nested contents are not
  touched (they stay inside their own container). The weight follows from the engine's existing encumbrance read.

### 2.3 Market stats (E1.9)

`MarketEntryDto` (`apps/desktop/src/boundary/listMarket.ts`) extends `EquipmentCatalogEntryDto` (no stat fields).
The gear detail panel renders `describe_held_equipment_at_root` (`character_hub.rs:7126`) for a held copy. Design:
extract the record-to-stats part of that function into one function over a catalog record, call it from both the
held path and a catalog path, and render with one component used by `GearDetail.tsx` and the Market box. Whether
the extraction is possible without changing the held path's output is the card's first test (the existing
`GearDetail.test.ts` must stay green). To verify in the card: `grep -n "fn describe_held_equipment_at_root" -A60
apps/desktop/src-tauri/src/character_hub.rs`.

### 2.4 Filters and the windowed list (E0.2, E1.10, E1.11)

Counts first: a test over `built_catalog()` with a reason per dropped row (`decisions.md §14`). Filter values: one
table in `marketFilters.ts` keyed by list → column → value → predicate over `types`; `Other` = no predicate in the
column matches. The dialog renders a filter column per list, then a windowed list (only visible rows mounted;
implementation chosen in the card — a fixed row height with an offset slice needs no dependency; a library is
added only if that fails, with its licence read).

## 3. The Abilities tab (E3)

### 3.1 What the operator asked for, mapped to code

| Need (22b) | Feats today (the model) | Other six sub-tabs after SD-43 |
|---|---|---|
| What is granted at each level, from which source | `FeatPool {id, granted, spent, sources}`; rows `feats.budget.<pool>.granted` with "Feats granted by …" (`feat_budget.rs:139–153`) | `abilities.budget.<subtab>.<pool>.granted` rows with sources per level |
| Refuse a pick not granted | `FeatBudget::refusal` (`feat_budget.rs:159`): "no feat pool has room for it (…)" | `AbilityBudget::refusal` naming the pool |
| Refuse a pick not qualified for, and say why | `src/rules_core/feat_prereqs/` evaluators; the picker's "Hide feats I don't qualify for" (`settings/featPickerPreference.ts`) | an `ability_prereqs` evaluator over pool members' `PRE*` tokens, reusing `feat_prereqs` where shapes match |
| Remaining picks per pool | badge `pool.label: spent of granted` (`CharacterSheet.tsx:2407–2410`) | same line per pool on each sub-tab |
| UI never re-derives | `readFeatBudget` reads rows only (`characterProgression.ts:228`) | `abilitySubTabModel` reads rows only (test) |
| Override | `Overrides.feats_without_prerequisites`, `ignore_feat_budget`; `admitted` flags (`OverrideTag`) | two new flags; same flags display |

### 3.2 Engine shape

```
src/rules_core/ability_budget.rs   (new; same structure as feat_budget.rs)
  pub enum SubTab { Feats, RacialAbilities, Traits, Archetype, Background, ClassFeatures, Misc }
  pub struct AbilityPool { subtab, id, granted, spent, sources: Vec<(level, source)>, admits }
  pub struct AbilityBudget { pools, held, over_by }
  impl AbilityBudget {
      fn explanation_rows(&self) -> Vec<ComputationExplanation>   // abilities.budget.<subtab>.<pool>.{granted,spent,remaining}
      fn refusal(&self, package, subtab, pool, record) -> Option<String>
  }
  pub fn ability_budget(package, input, base) -> AbilityBudget
src/rules_core/ability_prereqs.rs  (new)  — PRE* token evaluation for pool members
apps/desktop/src-tauri/src/abilities.rs (new) — list_ability_options / add_ability_selection / remove_ability_selection
```

- **Feats stay where they are.** The Feats sub-tab keeps `feat_budget.rs` and `feats.budget.*` unchanged; the new
  module covers the other six. No migration of feat data.
- **Storage.** Traits keep `selected_traits`; archetypes keep `selected_choices` with `choice:archetype`; class
  feature picks and favored class choices need a saved home — `ChosenCharacterState` has none today
  (`character_input.rs:17–55` lists the fields). E3.2 adds one `#[serde(default)]` field for ability selections
  (subtab, pool, record key, level taken, target) and the saved-character writer that persists it (to verify:
  `grep -rn "selected_traits" src/saved_character apps/desktop/src-tauri/src --include=*.rs | head`). Alternate
  racial traits: where the creation flow saves them is to verify
  (`grep -n "alternate" apps/desktop/src/characterHub/composeCreateCharacterRequest.ts`); E3.6 reuses that field.
- **Pool grants per level.** How a class record states "a rogue talent at 2nd and every 2 levels" is not yet
  known in our engine; E0.1 names the tokens and E3.4 reads them. The feat module's precedent: pools come from the
  rules the character holds (`Pool(<feat pool>)` targets, offers with counts), never a per-class table
  (`feat_budget.rs:3–9`). The same rule applies here: no hand-kept per-class table.
- **The catalog.** `class_feature_pool_catalog` already serves every `" ~ "`-grouped record that passes its safety
  gates (`:178–211`); 7,328 records are text-only and catalog-servable, 9,020 carry an engine-effect token
  (`python3 scripts/census_class_feature_pool_population.py`). A member that the catalog refuses (for an
  unresolved magnitude) is still a valid pick: its rule text prints, its number is not computed (paper-sheet
  rule). Whether the catalog's refusal gates should hide such members from the picker is a question E3.4
  answers with counts (to verify: how many members of the Rogue Talent, Rage Power and Ninja Trick groups the
  catalog serves versus the group totals 130, 170, 45).

### 3.3 UI shape

```
CharacterSheet  ── tab "Abilities" (was "Feats")
  AbilitiesTab
    sub-tab bar: Feats · Racial Abilities · Traits · Archetype · Background · Class Features · Misc
    Feats            → FeatsTab (unchanged)
    each other one   → AbilitySubTab(rows, options, selections)
                         pool lines: "<pool>: spent of granted"   (rows only)
                         Available: search · Qualified filter · options (unqualified dimmed, reasons on hover)
                         Selected: chosen picks with Choices column; Remove
                         Info pane: rule text of the highlighted record
                         Add disabled + reason when the engine refuses
                         data-gap sentence when E0.1 lists the sub-tab/pool as having no records
```

The sub-tab order and names are the operator's (22b). PCGen's tree views (Name, Type, Source, Prerequisite),
Multiples/Stacks/Cost columns and `EDITPOOL`/`FRACTIONALPOOL` (22a rows 4, 5, 10) are not in the DoD; 22a is
background (operator, 22b). The SD-44 feedback round decides whether any of them is wanted.

### 3.4 Printout

A new block after the feats block lists, per sub-tab heading, each chosen record's name, the level taken, and its
rule text; the favored class line states the class and the per-level choices. Registered in the PF block list
(`apps/desktop/src-tauri/src/print/layout/mod.rs`, near `:312` where `bio_notes` is pushed).

### 3.5 What this design does not cover

- Starfinder and Cyberpunk RED abilities (`scope-draft.md §5`).
- Retraining and replace-a-pick mechanics (excluded; Q-22.5).
- Whether every pool member's prose is correct — the corpus is the source; SD-43 renders it.
- Pool-size token shapes beyond those E0.1 finds — each extra shape becomes a discovered card (E3.4 split rule).

## 4. Sheet layout (E2.1, E2.2)

**Today (read):** the PF sheet root is `<div style={{ marginLeft: 'calc(50% - 50vw)', marginTop: '-3rem', width:
'100vw' }}>` (`CharacterSheet.tsx:4553`) with a bar `div` bordered by `2px solid var(--color-accent)` (`:4558`); the
page scrolls as a whole; Show Progression (`:4370`) sits in a panel in normal flow, so a window shorter than the
layout's natural height pushes it under the dock. The window config is 1920 × 1200 (`tauri.conf.json:17–18`).
The SF and CPR sheets have their own bars with Market and Print buttons (`StarfinderCharacterSheet.tsx:82–85`,
`CyberpunkCharacterSheet.tsx:168–171`).

**After:** one `SheetLayout` component — a CSS grid of `height: 100dvh` (no page scroll): row 1 the bar (opaque
background, the accent line), row 2 three columns: left rail, centre, right column. Only the centre has
`overflow: auto` as the main scroller; each side column has its own `overflow: auto` when taller than the window
(Q-17.1). Show Progression lives in the side column with bottom padding, so it is always reachable. Dialogs and the
progression drawer keep a z-index above the bar. The PF, SF and CPR sheets all render inside it (E2.1, E2.2). To
verify in E2.1: whether the `-3rem` margin escape is still needed once the root is `100dvh`
(`grep -rn "3rem" apps/desktop/src/layout/screenLayout.ts`).

## 5. Reports (E5.1, E5.2)

```
bug / enhancement screen ── Anonymous off ──► browserHandoff (today)
                         └─ Anonymous on  ──► invoke('send_report', {type, title, body, version})
                                                  Rust: key from the build (never in JS); byte check < 8 KB;
                                                  ureq POST https://campaign-codex.org/api/report
                                                  201 → "sent for review"; 400/403/413/timeout/offline/other → a message each
```

Handle, when given, is prepended as `body`'s first line in the UI before the byte check. `version` is the running
build's version (the same source the Update panel reads; to verify: `grep -rn "getVersion\|app_version"
apps/desktop/src/update/*.ts | head -3`).

## 6. What this document did not read

- `GearScreen.tsx`'s container box styling (E1.2 re-derives it first).
- `describe_held_equipment_at_root`'s body (E1.9 reads it).
- The saved-character writer for `ChosenCharacterState` (E3.2).
- The two-weapon attack path in rules_core (E6.4; `grep -rln "off_hand\|offhand\|two_weapon" src/rules_core`
  lists `feat_prereqs.rs`, `pilot_compute/class_shared_core.rs`, `pilot_compute/class_paladin_ranger.rs`).
- PCGen's UI strings for the seven sub-tabs (E0.1).
