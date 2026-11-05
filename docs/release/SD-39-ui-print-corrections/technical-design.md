---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready — every existing path below was checked with ls/grep -n on develop @ 5ef2f17b67; paths marked (new) are design choices the owning card confirms
---

# SD-39 Technical Design

> **`decisions.md §25` wins where this file differs.** It corrects §2 (slots from tags, stow exists), §3 (extend `purchase_equipment`, reuse the eqmods), §5 (ledger file) and §7 (sidecar config). Print opens the viewer (R19); BioNotes is the bio + the campaign description (R17).

## §1 Print pipeline (E8)

**Today.** `CharacterSheet.tsx:2854` calls `window.print()` after two animation frames;
`printLayout.ts` emits `@page { size: letter }` CSS that re-grids the on-screen DOM. There is no PDF
library in either cargo workspace (`grep -n 'typst\|printpdf\|genpdf\|lopdf' Cargo.lock
apps/desktop/src-tauri/Cargo.lock` → nothing) and no font file in the repo.

**Target.**

```
LoadSavedCharacterResponse + explanations rows + CharacterCustomDto (bio, notes, overrides)
        │  (the same data the screen renders; nothing re-derived)
        ▼
apps/desktop/src-tauri/src/print/model.rs     (new)  SheetDocument { frame, blocks[] }  — system-agnostic
apps/desktop/src-tauri/src/print/blocks/*.rs  (new)  one block type per section: Identity, Abilities,
                                                     Combat (AC/attack/speed/saves/HP/init), Skills, Weapons,
                                                     Armor, Inventory (worn | carried), Feats, ClassFeatures,
                                                     Spells (list | spellbook), Custom, Pets, Actions, Math,
                                                     BioNotes
apps/desktop/src-tauri/src/print/layout/{pf1e,sf1e,cpr}.rs (new)  the short per-system layout file:
                                                     which blocks, in which order, with which labels
apps/desktop/src-tauri/src/print/typst.rs     (new)  SheetDocument → Typst source (frame template +
                                                     block templates under resources/print/*.typ) → PDF bytes
apps/desktop/src-tauri/src/print/mod.rs       (new)  #[tauri::command] print_sheet(PrintRequest) ->
                                                     { pdf_path } ; save_sheet_pdf ; open/print sinks
```

- `PrintRequest { character_id, paper: A4 | Letter, spellbook: bool, math: bool, bio: bool, sink: Save | Print }`.
- Paper is a **Settings** value (`apps/desktop/src/settings/`, new key `print.paper`, default `A4`) read
  into the dialog, overridable per print.
- Blocks **flow**: Typst lays out pages; the layout file never names an inch. Page 1 of PF = identity,
  combat, abilities, saves, skills; weapons/armor/inventory next; feats/features; spells; then the
  optional Spellbook (full text per known spell from the spell catalog), Math (one line per explanations
  row the sheet shows, grouped by sheet value), BioNotes (bio + the description of each campaign whose party lists the character; those
  live in `campaignModel.ts`'s localStorage, so the caller sends them in `PrintRequest.campaigns`,
  `decisions.md §25`).
- Typst as a library: `typst` + `typst-pdf` (E8.1 records versions and `Cargo.lock` lines).
  **Recorded by E8.1:** `typst = "=0.15.1"`, `typst-layout = "=0.15.1"`, `typst-pdf = "=0.15.1"` (`apps/desktop/src-tauri/Cargo.toml`;
  `awk '/^name = "(typst|typst-pdf|typst-layout|krilla)"$/{n=$3; getline; print n, $3}' apps/desktop/src-tauri/Cargo.lock`
  prints typst 0.15.1, typst-layout 0.15.1, typst-pdf 0.15.1, krilla 0.8.2); tests read back with `pdf-extract` 0.12.1 and `lopdf` 0.42.0
  (dev only). Fonts: Liberation Sans 2.1.5, SIL OFL 1.1, compiled into the binary (`decisions.md §25`). Fonts: OFL
  families under `apps/desktop/src-tauri/resources/fonts/` (e.g. Liberation Sans/Serif), listed in
  `tauri.conf.json bundle.resources` and asserted in `tests/packaged_resources.rs`; the Typst world loads
  **only** these fonts (no system font search), so output is identical on every machine.
- Sinks: Save = `tauri-plugin-dialog` save-as (default `<name>-<system>.pdf`) then
  `tauri-plugin-opener`; Print = write to the app's temp dir and open with the opener in print mode where
  the platform offers one, else open the PDF (the receipt names the mechanism per platform).
- CPR already has a printed-lines module (`cpr_sheet_print.rs`); its lines become the CPR layout file's
  block inputs. SF uses `sf_sheet_print.rs`'s seed lines the same way.
- **Tests.** `pdf-extract` (or `lopdf` text ops) as a dev-dependency pulls the text back; golden tests
  assert: every sheet value string appears, block headings appear in the layout file's order, page count
  per (system, paper, options). Fixtures = the six seeds (`decisions.md §18`) + the three CPR seeds.

## §2 Body slots and worn vs carried (E1.0, E4)

- **Data.** `data/sheet_rules/<book>/equipment/*.json` gain `"slot": "<BodySlot>"` (E1.0). Source:
  PCGen `SLOTS:` (`lst_parser/equipment.rs:87`, currently dropped at `sheet_rule/convert.rs:1936,2557`)
  and the EQSLOTS location list from `$PCGEN_REPO_DIR` (pinned by `scripts/pcgen-oracle-pin.env`).
  `data/equipment_types.json` (exists) gains the `BodySlot` vocabulary.
- **Engine.** `src/rules_core/character_input.rs:163 ActiveState` already distinguishes
  `EquippedActive` (worn/wielded) from `SelectedInactive` (carried). New: `src/rules_core/body_slots.rs`
  (new) with `BodySlot`, `slot_of(item)`, `equip(selections, item) -> Result<_, SlotConflict { slot,
  held_by }>`; ring allows two; weapons: `WieldMain | WieldOff | WieldTwoHanded`; shields occupy
  `shield` and `WieldOff`. Consumers (`equipment_resolver.rs`, `encumbrance.rs`, `damage_total.rs`,
  `pilot_compute_corpus.rs` AC path) already branch on `ActiveState`; E4.2 proves with a test per
  consumer that `SelectedInactive` contributes weight only.
- **Commands.** `equip_item`, `unequip_item` (new, `character_hub.rs`); `add_equipment_selection`
  defaults new items to `SelectedInactive` (carried) — a behaviour change the receipt states; the Market
  buy flow offers "equip now".
- **UI.** Gear tab: `EquipmentPanel` (new, `apps/desktop/src/characterHub/EquipmentPanel.tsx`) with Worn
  (by slot) and Carried lists, Equip/Stow, the refusal message; E6.2 adds the override tag (`decisions.md §25`).

## §3 Market (E3)

- `list_market(MarketFilter { category: Weapon|Armor|Shield|Gear|MagicItem, name_contains, book })`
  (new command, `equipment_catalog.rs`) derives category from record type tags. Each row of an
  `enhanceable` priced arms or armor record has a `variants` list: `{ bonus, name "+N <Base>", priceGp,
  modifierKey }`, price per `decisions.md §8`. `modifierKey` is the eqmod `Special Ability ~ +N ~ <Kind>`;
  E3.2 attaches it, and the engine applies it to AC (armor/shield) or attack+damage (weapon) only while
  `EquippedActive` (E3.4). `decisions.md §25`.
- `buy_item { item_id, price_mode, equip_now }` / `sell_item { item_id, price_mode }` wrap
  `add/remove_equipment_selection` and move `gold` (`character_hub.rs:4857`). Refusal: `Insufficient
  funds: <price> > <gold>`.
- UI: `MarketDialog.tsx` (new) replaces `buildItemPickerConfig`'s weapon/armor/gear branches
  (`CharacterSheet.tsx:255–300`); the Spell/Feat/Modifier branches stay. SF and CPR sheets get the same
  dialog with their catalogs and currency (E3.5).
  The Sell list is `list_held_equipment`, not `corpusDerived.equippedItems` (`decisions.md §25`).

## §4 Feat guards (E1)

- `src/rules_core/feat_budget.rs` (new): pools from the class catalog and race; rows
  `feats.budget.<pool>.granted|spent|remaining` in explanations.
- `add_feat_selection_enforcing_prerequisites_at_root` (`character_hub.rs` ~3560) gains the budget and
  duplicate checks before the prerequisite check; error strings name the pool or the duplicate.
- Picker: `ItemPickerModal` over `itemPickerFilter.ts` reads the catalog's `eligibility` (`feat_catalog.rs`); checkbox "Hide
  feats I don't qualify for" default on; unticked rows greyed with `unavailableReason`, the unmet list joined by `; `.

## §5 Level ledger (E5)

- `ledger.json` beside the character (`apps/desktop/src-tauri/src/character_ledger.rs`, `decisions.md §25`):
  `{kind, id, target, taken_at_level, assigned_on_upgrade}`, one row a feat copy, a spell learned, a skill
  rank or a point of positive ability grant. The saved character and its schema do not change.
- Every Pathfinder change passes `mutate_saved_character_at_root`, which brings the ledger in step with
  what the character holds; a held thing with no row is stamped at the character's level, a row nothing
  holds goes. `save_character_custom_at_root` does the same for grants, and creation, `clone_character`
  and the import file carry it.
- A character with no file gets one on its first Pathfinder load, or its first change if that comes
  sooner (`rows_of_a_character_with_no_ledger`), every row `assigned_on_upgrade`. Slots are read off the
  character as it stood at each level, classes in saved order: feats through `feat_slot_levels`, ranks
  through each level's budget, spells through what the class casts and its known table or book rows,
  ability increases at every 4th level. What no slot fits has no level. `decisions.md §25` has the rules.
- `list_progression_ledger(character_id) -> {levels, withoutLevel}` (new command, off the main thread)
  assembles the drawer's rows from `ledger.json`, or what the next load would write when there is no file,
  and the character as it stood at each level: its sheet's class feature lines, feat budget, rank budget and
  known tables, and `hit_points.json`. It writes nothing. `characterProgression.ts` holds the words the
  drawer prints; its other logic stays for Create, the level-up preview and print (`decisions.md §25`).

## §6 Overrides (E6)

`CharacterCustomDto` (`character_custom.rs:67`) gains `overrides` (eight booleans, `decisions.md §10`);
each enforcing command receives the flags; admitted records carry `override: Option<String>`; the UI
badge component `OverrideTag.tsx` (new); the print Math/Inventory/Feats blocks print the tag in "full".

## §7 `.pcg` import (E7)

`crates/codex-ingest/src/pcg_import/{mod,parse,map}.rs` (new) + `bin/pcg_import.rs` (new):
`pcg_import <file.pcg> --out <character.json>`; mapping table in `map.rs` documented per key (CLASS,
RACE, STAT, FEAT, SKILL, SPELLLIST/SPELLNAME, EQUIPNAME + EQUIPSET location → `slot`/state, MONEY,
LANGUAGE, ALIGN, DEITY, CHARACTERNAME/BIO fields). The Tauri app ships the binary via
`tauri.conf.json bundle.externalBin` and calls it with `tauri-plugin-shell` (added) from
`import_character_from_pcg` (new command). The live crates keep `pcgen-residue-gate` green.

## §8 Layout registry (E2)

`sheetLayout.ts` today registers only the middle stat block (`SheetBoxId`, `SHEET_COLUMNS`). E2.1 adds
`'leftRail' | 'combatRow' | 'rightColumn'` regions with ordered boxes (`portrait, levelXp, classPanel,
characterDetails, showProgression` / `squareBlock, combatStrip` / `abilities, skills`), and
`CharacterSheet.tsx:4424–4690` renders from the registry. `ProgressionDrawer.tsx` (new) hosts the
former left-rail progression content (E2.3) and, after E5.3, the ledger.

## §9 Does not cover

Pixel layout (no browser on this box: `../tranche-17-ui/brief-2-layout.md:73`); the operator's eyeball
(E9.4) is the control. SF/CPR slot rules (DEF-2). Special abilities/materials (DEF-1).
