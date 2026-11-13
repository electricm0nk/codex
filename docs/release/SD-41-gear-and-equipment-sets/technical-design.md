---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready — every existing path below was checked with ls/grep -n/sed -n on tranche-20 @ f7f85f3d24; paths marked (new) are design choices the owning card confirms
---

# SD-41 Technical Design

> **`decisions.md §20` wins where this file differs.**

## §0 Today (tranche-20 @ `f7f85f3d24`; line counts from `artifacts/epic_0/census.txt §2`)

```
src/rules_core/character_input.rs        EquipmentSelection { item_id, equipped_or_active, active_state: ActiveState, applied_modifiers }
                                         ActiveState = EquippedActive | Absent | SelectedInactive   (97 struct literals build a selection)
src/saved_character/local_store.rs       envelope.txt + authoritative_character_input.txt; `equipment=item:<id>:<state>` lines (:458)
src/rules_core/body_slots.rs      906    BodySlot (18 variants); slot_table() ← data/equipment_types.json (26 slot rows; Ring capacity 2)
                                         worn_slot / worn_places / slot_conflict — the slot is derived from the record, never stored
src/rules_core/equipment_effects.rs 865  AC and every worn bonus: EquippedActive copies only (:207)
src/rules_core/encumbrance.rs     687    weight: every copy but Absent (:408); thresholds; remaining_lbs
src/rules_core/damage_total.rs   1259    weapon rows: EquippedActive weapons only (:820)
src/rules_core/equipment_record.rs 224   CorpusEquipmentRecord { weight_lbs, cost_gp, … }::weight_and_cost()  — no tags, no CONTAINS read
apps/desktop/src-tauri/src/character_hub.rs 21,905
   EquipmentEffectsDto :262  CarriedItemDto :298  EncumbranceDto :319  HeldEquipmentCopyDto :4968 { itemId, name, bonus, activeState, slot, override }
   list_held_equipment :5094 (derives each worn copy's place via worn_places; custom items appended from custom.json)
   purchase_equipment :4221 (bought_copy_placement: wear if the slot is free, else carried)
   set_equipment_active_state :5179 (bonus + fromState name the copy); remove_equipment_selection :5634
apps/desktop/src-tauri/src/character_custom.rs 837   CustomRecordDto { id, name, description, stats, bonuses, slot, worn, left_behind }; custom.json
apps/desktop/src-tauri/src/print/blocks/gear.rs 188  weapons (in-hand rows), armor, inventory (State / Weight per copy + totals + money)
apps/desktop/src/characterHub/EquipmentPanel.tsx 194 WORN_SLOTS (18); Worn / Carried / Left behind lists; one button per move; no drag
apps/desktop/src/characterHub/CharacterSheet.tsx     :562 SHEET_TABS; :1666 <EquipmentPanel …>; :3295 purchase requests 'EquippedActive'
apps/desktop/src/boundary/listHeldEquipment.ts 67; setEquipmentActiveState.ts 39
apps/desktop/scripts/ui-smoke/spec.json              174 rows; 31 equipment/market/custom rows; ops: click key type select scroll wait nativeFile (no drag)
data/sheet_rules/**/equipment/*.json                 232 files / 265 rows tagged Container (123 rows weapons); 0 with CONTAINS
data/corpus/**/equipment/**/*.json                   7,804 records; 210 with a CONTAINS token: 88 `<lb>|…`, 73 `UNLIM|…`, 49 `*…`; live record read from data/corpus/*/_settled/equipment.json (30 bundles, no tags/tokens)
```

## §1 PCGen's Equipping tab, in our words (functional reference only; `decisions.md §17`)

Read from the pinned oracle's documentation (`$PCGEN_REPO_DIR/docs/tabpages/inventory/inventoryequipping.html`
and `inventorygear.html`, resolved per `scripts/pcgen-oracle-pin.env`). What it does, as the operator
described it and as the docs confirm: a Purchase sub-tab buys from a catalog into one owned list; an
Equipping sub-tab shows that owned list on the left and a tree of locations on the right; the player names
**equipment sets** ("Dungeon", "City") and places the same owned items differently in each; an item must be
marked Equipped for its benefit to count and Carried for its weight to count; weapons have wield styles;
containers hold items; only equipped items reach the output sheet's sections, and carried weapons reach the
weapons section. SD-41 keeps the function (split panel; sets as placements of one list; equipped/carried/not
carried; containers; carried weapons printed) and none of the code, data format or class structure.

## §2 The location model (E1.2; `decisions.md §5`)

```
src/saved_character/equipment_sets.rs (new)
  pub enum Location { Slot(BodySlot), Equipped, Carried, NotCarried, Inside(CopyKey) }
  pub struct CopyKey { item_id: String, bonus: u8, ordinal: u16 }           // custom:<id> → bonus 0, ordinal 0
  pub struct EquipmentSet { name: String, locations: Vec<(CopyKey, Location)> }   // unnamed copy = Carried
  pub struct EquipmentSets { schema: u16 (1), active: String, sets: Vec<EquipmentSet> }
  pub fn load(root) -> Result<Option<EquipmentSets>>      // None when the file is absent
  pub fn save(root, &EquipmentSets)                        // atomic, as local_store writes
  pub fn location_of(&EquipmentSet, &CopyKey) -> Location
  pub fn project(&EquipmentSet, copies: &[(CopyKey, is_container: bool)]) -> Vec<ActiveState>   // decisions.md §6
  pub fn default_from(copies: &[(CopyKey, ActiveState, BodySlot)]) -> EquipmentSets              // decisions.md §8
  pub fn rename / create(copy_of: Option<&str>) / delete (last-set rule) / activate
```

The file: `<character root>/equipment_sets.json` (`decisions.md §5.3`). The saved input is untouched.

**Projection is the only bridge to the engine.** `project` is called by every write that changes a
location or the active set (E2.2), and its result goes into `active_state` through the existing
recompute-and-save path (`apply_set_equipment_active_state`'s successor). Reads never project. This is
why `pf_seed_render_hash`, the print goldens, the `.pcg` import and every existing engine test keep their
values: for a Default set, `project(default_from(x), x) == x` (E1.2's theorem test over the seeds and the
fixture population).

## §3 Containers (E1.1, E1.3; `decisions.md §7`)

```
crates/codex-ingest/src/pcgen_import/ir_converter.rs   container_of(tokens) = TYPE has Container && no DAMAGE; container_capacity_of(tokens) from CONTAINS; settled into data/corpus/*/_settled/equipment.json (decisions.md §31)
src/rules_core/equipment_record.rs     CorpusEquipmentRecord { container: bool, container_capacity: ContainerCapacity { Pounds(f64) | Unlimited | Unstated } } (serde default; omitted on weapons → fixture bundle byte-identical)
                                       is_container() / capacity() read the settled fields; no live read of tags or raw_tokens
src/rules_core/containers.rs (new)     pub struct ContainerLoad { copy: CopyKey, contents_lbs: f64, capacity: ContainerCapacity, over: bool }
                                       pub fn container_loads(set, selections, corpus, custom_bases) -> Vec<ContainerLoad>   // recursive; SD-b for unweighed copies
```

Carried-ness of contents is the projection's job (`Inside(c)` follows `c`); `containers.rs` only sums
what each container holds against its capacity. A move into a full container is never refused (A3); a
cycle and a non-container target are refused by the command (E2.2).

## §4 The desktop backend (E2.x)

```
character_hub.rs
  load_saved_character_at_root: equipment_sets::load(root) → None ⇒ save(default_from(copies)) once; response += equipmentSets { active, names }, containerLoads[]
  HeldEquipmentCopyDto += location: LocationDto, ordinal: u16            (LocationDto on the wire: { kind, slot?, container? })
  #[tauri::command] move_equipment_copy { characterId, copy, location, savedAt } -> CreateCharacterOutcome
                     create_equipment_set { characterId, name, copyActive } / rename_equipment_set / delete_equipment_set / activate_equipment_set
  set_equipment_active_state  = a move (EquippedActive → derived slot or Equipped; SelectedInactive → Carried; Absent → NotCarried)
  remove_equipment_selection  = also drops the copy from every set and re-ordinals later copies of the same item/bonus
  purchase_equipment          = unchanged; the app requests SelectedInactive (CharacterSheet.tsx:3295); bought_copy_placement removed
character_custom.rs
  CustomRecordDto += weight_lbs, price_gp, container_capacity_lbs: Option<..> (serde default; decisions.md §25); create_custom_item deducts the price; custom items weigh via compute_encumbrance's extra copies
  WeaponDamageDto += in_hand (E2.4; from WeaponDamageBreakdown.in_hand, E1.4)
main.rs: the five new commands registered
boundary/moveEquipmentCopy.ts, boundary/equipmentSets.ts (new); listHeldEquipment.ts, loadCreateCharacter.ts, loadSavedCharacterDetail.ts, loadCharacterCustom.ts (types)
```

Old-save proof (E2.1): `tests/old_save_loads_unchanged.rs` copies a seed root without `equipment_sets.json`
into a temp app-data dir, loads it, asserts the file now exists with one `Default` set, and asserts
`armor_class`, `total_carried_weight_lbs`, `level` and every `defense.armor_class.*` row equal literals
recorded from the base (bytes the engine's read path does not touch).

## §5 The gear screen (E3.x; `decisions.md §13, §14`)

```
ui/characterHub/gearScreenModel.ts (new, pure)
  rows(held, encumbrance, containerLoads) -> GearRow[] { copy, name, weightLabel, locationLabel, isContainer, capacityLabel, over }
  filter(rows, 'all' | 'equipped' | 'notEquipped')                 // Equipped = Slot(_) | Equipped
  tree(rows, slotTable) -> LocationNode[]                            // 17 slots (places shown), Equipped, Carried{ containers nested }, NotCarried
  moveTargets(row, rows, slotTable) -> Location[]                   // excludes self, descendants, full slots
  dropTarget(dataTransfer, targetId, model) -> Location | null      // E3.2
ui/characterHub/GearScreen.tsx (new)
  <section aria-label="Gear"> left: filter + rows (draggable; Move to role="menu"); right: tree (drop targets); set picker; AC + carried weight headline from the sheet's engine rows
CharacterSheet.tsx:1666  <EquipmentPanel …> → <GearScreen …>; EquipmentPanel.tsx deleted
```

No TS computes a weight or an AC: every label is a formatted engine value (`encumbrance.per_item`,
`containerLoads`, the AC row). `data-testid`s: `gear-row-<itemId>-<bonus>-<ordinal>`, `gear-move-to`,
`gear-location-<kind>[-<slot>]`, `gear-set-select`, `gear-filter`.

## §6 The print (E4.1; `decisions.md §9`)

`gear.rs::inventory` (or the name SD-40's E3.2 left it) reads `held[].location` and `containerLoads`:
rows grouped Slot in use → Carried (containers' contents nested; a `depth` string the look indents by) →
Not Carried (names only) → the totals and limits as today. `gear.rs::weapons` orders `in_hand` rows first,
then carried rows with "(carried)"; Not Carried weapons are absent from `weapon_damage` by E1.4.
`looks/gear.typ` indents by the row's depth; it computes nothing (NR-2).

## §7a The weapons rows after SD-41 (E1.4–E1.6, E2.4, E4.1; `decisions.md §9, §22, §23`)

```
WeaponDamageBreakdown += in_hand: bool, unarmed: bool, nonlethal: bool, uses_ammunition: bool, ammunition_carried: u32
  resolve_weapon_damage_breakdown: unchanged (EquippedActive only, in_hand true)   -- decisions.md §33
  resolve_weapon_damage_breakdown_with_carried (new, E1.4): in-hand rows, then SelectedInactive weapons in_hand false (Absent off) + one appended unarmed row from unarmed_strike.json (E1.5)
  desktop callers (character_hub.rs:2879, rule_system_adapter.rs:435) switch to the carried view in E2.4, marking carried rows in the same card
  unarmed: die from the record (1d3; Small → size step), monk progression via class_monk.rs:470's table, nonlethal unless Improved Unarmed Strike
  uses_ammunition: the record's `Projectile` tag; ammunition_carried: Σ quantity × base_quantity over carried `Ammunition` copies matching the weapon's CONTAINS family
WeaponDamageDto += inHand, unarmed, nonlethal, usesAmmunition, ammunitionCarried
order everywhere: in hand → carried "(carried)" → Unarmed last; 20 tick boxes + "<kind>: <n> carried" under uses_ammunition rows only; no global Ammunition row
```

## §8 The Market after SD-41 (E2.5, E2.6, E3.4; `decisions.md §24, §26, §28`)

```
equipment_catalog.rs   EquipmentCatalogEntryDto += slot: BodySlot (slot_of(record)); every producer sets it
character_hub.rs       purchase_equipment  += quantity, negotiated_price_copper      (n copies, one save; price × n or the negotiated total)
                       sell (existing path :4910) += quantity, negotiated_price_copper (default refund_copper × n)
                       discard_equipment_copy { copy, quantity }                      (no money; set entries dropped)
                       rename_equipment_copy { copy, name }                           (copyNames in equipment_sets.json)
                       purchase_custom_equipment { baseItemId, name, masterwork, enhancement, specialAbilities, material, size, quantity, negotiatedPriceCopper }
                       quote_custom_equipment (dry run: price, weight, unpriced eqmods)  list_custom_equipment_options { baseItemId }
                       create_custom_item { name, description, weightLbs, priceGp, containerCapacityLbs, slot }   (custom.json; price deducted; Carried)
ui/characterHub/marketModel.ts (pure)   sortEntries(entries, column, dir); affordable(entry, mode, totalCopper); quantityLabel; needsNegotiate(entry)
ui/characterHub/MarketDialog.tsx        table Item | Price | Weight | Slot (sortable headers); description box; Only Show Affordable; Quantity; Negotiate; Buy Custom; Create; Sell table over stacks
ui/characterHub/BuyCustomDialog.tsx, CreateItemDialog.tsx (new)
```

Pricing lives in Rust: `Enhanceable::price_gp` (masterwork + N² rate) + each eqmod's flat `COST` or an
evaluated formula (`WT*N`, `BASECOST*N`, `+N`-equivalent steps); an unpriced eqmod makes the quote
"negotiate". No TS multiplies a price: the quantity total and the custom quote come from the backend.

## §9 Prices: the `convert` lane (E1.8, E1.9; `decisions.md §26`)

```
crates/codex-ingest/src/pcgen_import/corpus_equipment_json.rs, cache_gen/*.rs   composite rows (BASEITEM + EQMOD) keep name + tokens; cost = base + Σ eqmod cost; literal COST summed with eqmods (never negative)
data/corpus/**/equipment/**            re-converted; only cost_gp / weight / name / raw_tokens may change (field-diff check)
src/rules_core/rules_catalog/**        regenerated compiled tables (the Market's per-book maps)
data/price_overrides/d20pfsrd-prices.json   [{ key, price_gp, url, fetched_at }]   converter input; one URL per item
data/price_overrides/price-unknown.json     [{ key, reason }]                        the gate's only allowed exceptions
tests: market_price_census (E1.8: the Market's none / zero / negative by book, category, cause) ; market_prices_are_positive_or_listed (E1.9 gate)
```

## §10 Stacks and created items (E1.7, E2.1, E2.3; `decisions.md §25, §28`)

```
equipment_sets.json copies map: "<itemId>|<bonus>|<ordinal>" → { name?, quantity? }   -- pieces per stack (decisions.md §30); absent = one bundle (base_quantity pieces)
held list: rows = group(copies by item_id, bonus, name, location) → { quantity: Σ pieces, copies: [CopyKey], baseQuantity }; one equipment= line per stack
migration (E2.1, first load): a bundle copy without a quantity gets quantity = base_quantity; weight unchanged (old-save test)
labels: "<name> ×<n>" in pieces (the bundle's "(20)" dropped)   (TS formats the engine's figures)
commands: purchase by bundle (adds quantity × base_quantity pieces), sell / discard n pieces (price n / base_quantity × bundle price, floored), move n pieces (lowers one stack, raises or creates one at the target), drop (= move NotCarried)
compute_encumbrance(selections, corpus, extra: &[WeighedCopy], quantities: &[(CopyKey, u32)])   -- a stack weighs weight × quantity / base_quantity; created items weigh via extra; same Absent rule
CustomRecordDto += weight_lbs, price_gp, container_capacity_lbs (Option; serde default)   -- custom.json; edit/remove in the Custom tab
```

## §11 Campaign write-through (E1.10, E2.7, E3.5; `decisions.md §29`)

```
<character root>/campaigns.json      [{ campaignId, campaignName, campaignDir, active }]     -- the app is master; written by set_character_campaigns
src/saved_character/campaign_mirror.rs (new)
  mirror(root, label, character_id, memberships) -> Vec<MirrorWarning>   -- every file of root (less campaigns.json) → <campaignDir>/characters/<slug>-<id>.tmp-<pid>/ → rename over the target; one warning per bad dir; nothing read back
  SavedCharacterStore::save_with_warnings(...)   -- calls mirror after both files land; save(...) keeps the old signature for the 171 call sites
desktop: campaign_mirror::after_write(root)      -- called by every sidecar writer (custom, bio, hit points, money, portrait, equipment_sets); tests/every_character_root_writer_mirrors.rs scans the sources for a writer without it
UI: IdentityPanel's Campaign label → CampaignMembershipDialog (drive_list_campaigns; checkboxes; Save → set_character_campaigns + drive_save_campaign with nonce for partyCharacterIds)
    Skills block footer: one "Open <name>" button per active campaign → CharacterHubPage mode campaign-sheet(id)
    mirrorWarnings on CreateCharacterOutcome / the load response → a dismissible warning line on the sheet
```

## §7 Tests the bundle adds (names are the acceptance filters)

| Card | Where | Names |
|---|---|---|
| E1.1 | root `--lib`; codex-ingest | `container_tag_population_is_232_files_265_rows`, `container_capacity_population_is_210_88_73_49`, `container_capacity_reads_backpack_unlimited`, `container_weapon_with_contains_is_not_a_container`; `container_of` (2) |
| E1.2 | root `--lib`; `crates/codex-ingest/tests/equipment_sets_pcg_population.rs` | `equipment_sets_*` (8); the population test |
| E1.3 | root `--lib` | `containers_*` (4) |
| E1.4 | root `--lib` | `carried_weapon_*` (3) |
| E1.5 | root `--lib` | `unarmed_row_*` (4) |
| E1.6 | root `--lib` | `uses_ammunition_*` (3) |
| E1.7 | root `--lib` | `extra_weighed_*` (2), `base_quantity` (1) |
| E1.8, E1.9 | `tests/market_price_census.rs` (codex-ingest or desktop) | `market_price_census`, `market_prices_are_positive_or_listed` |
| E1.10 | root `--lib` | `campaign_mirror_*` (5) |
| E2.7 | desktop `tests/`; desktop | `every_character_root_writer_mirrors`; `campaign_write_through_*` (3) |
| E3.5 | `campaignMembershipModel.test.ts` | ≥ 4 cases |
| E2.1 | desktop | `equipment_sets_on_load_*` (2), `held_list_stacks_*` (2); `tests/old_save_loads_unchanged.rs` |
| E2.2 | desktop | `move_equipment_*` (6), `move_quantity_splits_a_stack` |
| E2.3 | desktop; `customModel.test.ts` | `create_custom_item_*` (4) |
| E2.4 | desktop; `weaponsTabModel.test.ts` | `weapons_in_hand_first_*` (3) |
| E2.5 | desktop | `market_backend_*` (5), `catalog_slot_is_set_on_every_entry` |
| E2.6 | desktop | `buy_custom_*` (4) |
| E3.1, E3.2 | `gearScreenModel.test.ts` | ≥ 8 model cases + ≥ 4 drop cases |
| E3.4 | `marketModel.test.ts`, `MarketDialog.test.ts` | ≥ 6 model cases |
| E3.3 | ui-smoke | 18 new + 6 rewritten rows |
| E4.1 | desktop `print_pf_blocks` | `print_pf_blocks_gear_*` (5) |
