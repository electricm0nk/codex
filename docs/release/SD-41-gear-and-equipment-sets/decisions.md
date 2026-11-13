---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready
---

# SD-41 Decisions

Each section is marked **OPERATOR** (a ruling on record), **DEFAULT** (the author's call; revisit named) or
**STANDING** (carried from earlier bundles). `§20` (pre-launch corrections) wins over older text here.

## §1 — Package location

**DEFAULT.** `docs/release/SD-41-gear-and-equipment-sets/`, as every sibling (SD-38, SD-39, SD-40 recorded
the same deviation from the overlay's `docs/stc/`). **Enforced by:** C0. **Revisit:** the operator says
"move to docs/stc"; one `git mv` before C1.

## §2 — Branch, board, version

**OPERATOR ("New package after SD-40") + DEFAULT.** Branch `tranche-21`, cut from `origin/tranche-20` @
`f7f85f3d24` on 2026-11-12 (`git merge-base HEAD origin/tranche-20`), while SD-40 is live in
`.worktrees/sd40-main`. Board: `./kanban.md` + `./progress.md`. Version `0.21.0` stamped by C1's **single
commit** on the 14 surfaces SD-40's C1 bumped (`git show --name-only --format= 3ceeb63b66 | awk 'NF' |
grep -vc current-release.json` → 14; F-1) plus `docs/release/current-release.json` → `{"tranche": 21,
"notes_path": "docs/release/SD-41-gear-and-equipment-sets/release-notes.md"}`. `release-notes.md` ships an
interim body with the sentinel `SD-41 INTERIM NOTES`; E5.3 replaces it; E5.4 refuses the PR while the
sentinel is present. **Enforced by:** C1's receipt; `python3 tools/release/resolve_release_notes.py
--repo-root .` exit 0. **Revisit:** the tranche digit moves only at a new bare cut.

## §3 — Operator answers on record (2026-11-12, verbatim as relayed by the lead)

| # | Question | Operator's answer |
|---|---|---|
| A1 | Slots | "Full Pathfinder list" — right panel = the app's existing `WORN_SLOTS` (`apps/desktop/src/characterHub/EquipmentPanel.tsx:7`: Main hand, Off hand, Both hands, Shield, Armor, Head, Headband, Eyes, Shoulders, Neck, Chest, Body, Belt, Wrist, Hands, Ring ×2, Feet), then Equipped (no slot), Carried, Not Carried |
| A2 | Sets | "Where each item is" — all sets share one item list; a set stores only each item's location (slot / equipped / carried / not carried / inside container X); one active set drives sheet, AC, encumbrance and print |
| A3 | Containers | "Weight counts, capacity shown" — contents count toward weight only when the container is carried or equipped; capacity shown (e.g. backpack 60 lb) with a warning when over, never a block; nesting allowed; a custom-named item (e.g. "Food", "Camping") works as a container when its base item is one; hooks into the existing Custom tab / custom item naming |
| A4 | Moving | "Drag plus a 'Move to' menu" (keyboard + testable) |
| A5 | Weapons block (screen + print) | "In hand first, then carried"; Not Carried weapons left off |
| A6 | Old saves | "Make a 'Default' set" — on first load, slotted items keep their slot, everything else Carried; AC and weight unchanged from today |
| A7 | Print inventory page | "Active set, by location" — slots in use, then Carried with each container's contents nested under it, then a short Not Carried list (no weight); totals match encumbrance |
| A8 | Market purchases | default to Carried (the operator's words in the request) |
| A9 | Package placement | "New package after SD-40" |

The request itself is quoted in full in `scope-draft.md §1` and `artifacts/cycle_0/operator-brief-2026-11-12.md`.

## §4 — Phasing against SD-40 (DEFAULT; the partition is derived, not assumed)

SD-40 is live on `tranche-20` (32 of 56 cards complete, `artifacts/epic_0/census.txt §7`). Its open cards
own `character_hub.rs` (E2.28, E2.29, E3.6), `CharacterSheet.tsx` (E2.24–E2.26, E2.29, E3.6), `main.rs`
(E2.29), `spec.json` (E2.24–E2.26, E2.29, E3.6, E4.2, E4.MC), `print/blocks/gear.rs` (E3.2, E3.5),
`print/layout/pf1e.rs`, `print/{model,typst}.rs`, `blocks.typ` (E2.23, E3.x, E4.1), `print_golden.json`
(E3.1, E3.3, E4.1), `packaged_resources.rs` (E3.x, E5.1), `loadSavedCharacterDetail.ts` (E2.24–E2.26).
None of SD-40's open cards owns `equipment_record.rs`, `encumbrance.rs`, `equipment_effects.rs`,
`damage_total.rs`, `body_slots.rs`, `src/saved_character/**`, `character_custom.rs`, `EquipmentPanel.tsx`,
`MarketDialog.tsx`, `CustomDialog.tsx` (the census §7 command, one line per file). The Market cards (E2.5,
E2.6, E3.4) write `MarketDialog.tsx` (free) **and** `CharacterSheet.tsx` (its Market handlers; SD-40-owned),
so they are Phase 2 like every other `apps/` card.

- **Phase 1, now:** C0, C1 and Epic E1 (root crate + new files only: `src/rules_core/{equipment_record,
  containers,damage_total,encumbrance}.rs`, `src/saved_character/equipment_sets.rs` *(new)*, their tests),
  plus the tool-side lane `convert` (E1.8, E1.9: `crates/codex-ingest/**`, `data/corpus/**/equipment/**`,
  `data/price_overrides/**` *(new)*, the regenerated compiled tables — no open SD-40 card owns a `data/`
  path: `awk -F'|' '$2 ~ /^ (E2\.2[3-9]|E2\.MC3|E2\.G|E3|E4|E5)[0-9A-Za-z.]* *$/ && $4 ~ /data\//' ../SD-40-pf-sheet-design/epic-breakdown.md`
  prints nothing). Cargo lanes on the box stay ≤ 3 including SD-40's (`pgrep -x cargo`; R-Q2).
- **Phase 2, after SD-40's PR merges:** the lead rebases `tranche-21` onto `develop` (SD-m, at most once,
  logged in `progress.md`), re-runs E1's tests, then dispatches E2 → E3 → E4 → E5. E2's first card
  (E2.1) is the first write to `character_hub.rs`.
- If SD-40 stalls past E1.MC, the lead does **not** start E2 on `tranche-20`'s moving base: an SD-41 card
  that writes an SD-40-owned file before the merge is a two-writers incident (R19). The only exception the
  operator may grant is a named region rule as SD-40 used between its own lanes (`../SD-40-pf-sheet-design/epic-breakdown.md §0`, E2.17 ∥ E2.27).

**Enforced by:** `epic-breakdown.md §0`'s Phase column; `kanban.md` E2.1 `Depends on` includes `SD-40 merged`
(the lead flips it to a SHA); the prefix's wrong-base control. **Revisit:** the operator rules otherwise.

## §5 — The location model and the sidecar file (DEFAULT on the shapes; OPERATOR on the vocabulary, A1, A2)

### §5.1 Locations

```
Location = Slot(BodySlot)      -- one of the 17 real slots (BodySlot less `None`); Ring holds 2, the rest 1 (data/equipment_types.json `slots[].capacity`)
         | Equipped            -- worn, takes no slot (BodySlot::None today; e.g. a slotless wondrous item)
         | Carried             -- with the character, not worn
         | NotCarried          -- owned, left behind (today's `Absent`)
         | Inside(CopyKey)     -- in a container copy; nesting allowed; the container's own location decides carried-ness
```

The 18 rows of `WORN_SLOTS` are A1's list (`Main hand, Off hand, Both hands, Shield, Armor, Head, Headband,
Eyes, Shoulders, Neck, Chest, Body, Belt, Wrist, Hands, Ring, Feet, No slot`); the screen shows Ring once
with two places. `No slot` worn = `Equipped`.

### §5.2 Copy identity

A location names a copy by `CopyKey { item_id, bonus, ordinal }`: the `ordinal`-th copy (0-based) of
that `item_id` and `bonus` in the saved input's order. Custom items are `custom:<id>`, bonus 0, ordinal 0.
This is the identity `set_equipment_active_state` already approximates with `bonus` + `fromState`
(`setEquipmentActiveState.ts`), made stable. Removing a copy (sell) shifts later ordinals: `remove_equipment_selection`
rewrites every set's entries for that `item_id`/`bonus` (E2.2 owns the second call site).

### §5.3 The file

`<character root>/equipment_sets.json` beside `envelope.txt` and `custom.json` (`local_store.rs:23–24`;
`custom.json` is the precedent for a sidecar the desktop crate owns):

```json
{ "schema": 1, "active": "Default",
  "sets": [ { "name": "Default", "locations": [ { "copy": { "itemId": "item:chain_shirt", "bonus": 0, "ordinal": 0 }, "location": { "kind": "slot", "slot": "armor" } },
                                                { "copy": { "itemId": "item:shield", "bonus": 0, "ordinal": 0 }, "location": { "kind": "notCarried" } },
                                                { "copy": { "itemId": "item:rations", "bonus": 0, "ordinal": 0 }, "location": { "kind": "inside", "container": { "itemId": "custom:food", "bonus": 0, "ordinal": 0 } } } ] } ] }
```

**A copy a set does not name is Carried in that set.** So a purchase writes nothing to the file (§10), a
new set starts as "everything Carried" unless created as a copy of the active one (E3.1 offers both), and
the file holds only what differs from Carried. Set names are unique, trimmed, 1–40 characters; the last set
cannot be deleted; deleting the active set activates `Default` or the first remaining.

The model, the file read/write and the projection (§6) live in the **root crate**,
`src/saved_character/equipment_sets.rs` *(new)*, beside the store that owns the character root — so E1.2
runs now (§4) and the desktop crate (E2.1) only calls it. The saved input's `equipment=` lines and the
`EquipmentSelection` shape are **not** changed (97 producers, F-5).

**Enforced by:** E1.2's tests (round trip, unnamed = Carried, name rules, last-set rule). **Revisit:** a
second game system wants sets (the file is system-neutral by construction).

## §6 — Projection: the active set becomes today's `active_state` (DEFAULT; the twin rule)

The engine keeps computing from `active_state` (AC: `EquippedActive` only, `equipment_effects.rs:207`;
weight: all but `Absent`, `encumbrance.rs:408`; weapons rows: `EquippedActive`, `damage_total.rs:820`).
SD-41 adds **one** function, `project(set, copies) -> Vec<ActiveState>`:

| Location | Projected `active_state` |
|---|---|
| `Slot(_)`, `Equipped` | `EquippedActive` |
| `Carried` (named or unnamed) | `SelectedInactive` |
| `NotCarried` | `Absent` |
| `Inside(c)` | `SelectedInactive` when `c` projects to `EquippedActive` or `SelectedInactive`; `Absent` when `c` projects to `Absent`; a cycle (refused at move time, §7) projects to `SelectedInactive` and is logged, never a panic |

Every write that changes a location (E2.2) projects the active set, stores the result in the saved input's
`active_state`s through the existing recompute-and-save path, and re-saves. Switching the active set is a
projection too. Reading never projects: the saved input is already the active set's picture, which is why
a `.pcg` import, a seed, the print and every existing test keep working. **This is the control for A6's
"AC and weight unchanged":** the Default set (§8) projects to exactly today's `active_state`s, and a test
asserts it over the population (G-1).

**Enforced by:** E1.2 `projection_round_trips_every_saved_state`; E2.1 G-1. **Revisit:** none while the
engine keys on `ActiveState`.

## §7 — Containers and capacity (OPERATOR A3; DEFAULT on the data source)

- **Is a container:** the record's tags hold `Container` (`data/sheet_rules/**/equipment/*.json` `tags`;
  232 files / 265 rows) **and the record is not a weapon: no base damage and no `Weapon` type word** (§20; operator, §31 R2: "Container tag, but not
  weapons"). E1.1 settles `container` and `container_capacity` through `ir_converter.rs` into
  `data/corpus/*/_settled/equipment.json`, the file `CorpusEquipmentRecord` is read from (§31 R1); the live record
  never reads tags or tokens.
- **Capacity:** the `CONTAINS` token's first field in the corpus record (210 of 7,804 records: `<lb>|…` 88,
  `UNLIM|…` 73, `*<n>|…` weight-reduction kinds 49; §31 R3), settled by the converter as
  `ContainerCapacity = Pounds(f64) | Unlimited | Unstated` (the `*` kinds are `Unstated`: contents weigh in full,
  DEF-1; the base Backpack is `UNLIM|Any` → `Unlimited`). `data/sheet_rules` holds no `CONTAINS`
  (`grep -rl CONTAINS data/sheet_rules … | awk` → 1 file, not an equipment record), so capacity comes from the
  corpus record, the same bytes weight comes from, through the same settled bundle.
- **Container loads** (E1.3, `src/rules_core/containers.rs` *(new)*): for every container copy in the
  active set, `ContainerLoad { copy, contents_lbs, capacity, over: bool }`, contents summed recursively
  (a pouch inside a backpack adds to both), weights from the same `weight_and_cost()` read encumbrance uses.
  The load response carries them as `containerLoads` (E2.1); the screen shows "12 of 60 lb" and a warning
  row when `over`; the print shows the same figure. **Never a block:** a move into a full container succeeds.
- **Cycle:** a move whose target container is the moved copy or is inside it (transitively) is refused by
  `move_equipment_copy` with a diagnostic (E2.2); a non-container target is refused the same way.
- Custom containers: §11.

**Enforced by:** E1.1, E1.3 tests with the counts above; E2.2's refusals; E3.1 `gearScreenModel.test.ts`.
**Revisit:** a converter emits capacity into sheet rules (then the read path moves, the figures do not).

## §8 — The Default set for old saves (OPERATOR A6)

On first load of a character root with no `equipment_sets.json`, E2.1 writes one with a single `Default`
set built by `default_from(copies)` (E1.2): `EquippedActive` with a real slot → `Slot(slot)` (the slot the
list derives today, `worn_places`); `EquippedActive` with `BodySlot::None` → `Equipped`; `Absent` →
`NotCarried`; `SelectedInactive` → unnamed (Carried). Custom items follow `worn` / `left_behind`. By §6,
`project(default_from(x), x) == x` for every copy, so **AC and weight are unchanged**, proved by:

- E1.2: the round-trip theorem over the seeds' inputs and every `.pcg` import fixture
  (`crates/codex-ingest/src/pcg_import/fixtures_tests.rs::fixtures()`, the population SD-40's E1.MC
  counted: 37 saved of 65 files; E1.2 measured 63 of 63 converted files round-trip, §20);
- E2.1: G-1's seed render hashes `SAME_EXCEPT_NAMED` (only the new keys) and a desktop test that loads a
  pre-SD-41 root, asserts the file now exists, and asserts `armor_class` and `total_carried_weight_lbs`
  equal the pre-load values.

No schema bump of `envelope.txt` (`CURRENT_SAVED_CHARACTER_SCHEMA_VERSION` stays 2): the sidecar is
additive and absent-means-Default. **Revisit:** none.

## §9 — Weapons: in hand first, then carried (OPERATOR A5, A7)

`damage_total.rs` builds weapon rows for `EquippedActive` weapons only (line 820). E1.4 widens the input
to `SelectedInactive` weapons with `in_hand: false` on the breakdown; `EquippedActive` rows keep their
values byte-for-byte (G-1 over the seeds: no existing row moves; new rows only for carried weapons, which no
seed holds today — E1.4 names the population it adds). E2.4 passes `inHand` through `WeaponDamageDto`
and orders the Weapons tab in-hand first; E4.1 orders the printed Weapons block the same way and marks
carried rows "(carried)". `Absent` weapons stay off both. The attack total of a carried weapon is the
engine's row as if wielded in the main hand (the same primary-hand assumption SD-40's E2.14 named).

The printed Inventory page (A7): the `gear` block lists the active set by location — each slot in use with
its copy; then **Carried**, each container's contents nested (indented) under it with the container's
"x of y lb"; then **Not Carried** as a short list without weight; then the totals and limits exactly as
the encumbrance rows print today (no new sum: `total_carried_weight_lbs` is the engine's).

**Revisit:** the operator wants carried weapons without attack rows (`§21` Q2).

## §10 — Market purchases default to Carried (OPERATOR A8)

`CharacterSheet.tsx:3295` requests `'EquippedActive'`; E2.2 changes it to `'SelectedInactive'`, so
`purchase_equipment` appends a `SelectedInactive` selection and writes no set entry (unnamed = Carried in
every set, §5.3). `bought_copy_placement`'s wear-if-free branch becomes unreachable from the app and is
removed with its test in the same card (no dead shipping code). The smoke row `sheet-market-buy-weapon-worn`
is rewritten as `sheet-market-buy-carried` (E3.3). **Revisit:** none.

## §11 — Custom items as containers (OPERATOR A3; DEFAULT on the mechanism) — **superseded by §25 (2026-11-12)**

> A custom container needs no base item: it is a stock record tagged `Container`, a Buy Custom item on a
> container base (§24), or a Create item with a capacity (§25). `base_item_id` is not added. The text
> below is kept for the record.

`CustomRecordDto` (equipment kind) gains `base_item_id: Option<String>` (an equipment id the catalog
knows). When set: the item's weight is the base's, it is a container iff the base is, and its capacity is
the base's. When unset (every existing custom item): unchanged — weightless, not a container. The Custom
dialog gets a base-item picker (search over the catalog, the same list the Market filters). "Food" and
"Camping" are custom items based on `Pouch, belt` / `Backpack` with contents inside them. **Enforced by:**
E2.3's tests (a based custom item weighs; a rations copy inside "Food" counts toward weight when Food is
carried and not when Not Carried). **Revisit:** `§21` Q3.

## §12 — Run protocol (UNATTENDED after launch)

The standing autonomous rules apply (memory: "operator away — autonomous mode", "run to 100% DoD
without asking"):

1. Clear a blocker first; if only an operator ruling can clear it, mark **only that card**
   `blocked-escalated` with the exact ruling under `progress.md ## Open blockers`; continue every
   independent card.
2. Never open the PR while any card is open, partial or blocked-escalated.
3. A choice inside scope with a safe default (§12.1): take it, log it, continue. No `AskUserQuestion`.
4. **Report after each epic** (the merge-check card writes the Epic report block in `progress.md`).
5. **One planned wait:** E4.G. The run proceeds past it only on the operator's two `usable: yes` lines.
   Defects found there are discovered cards (§13 of SD-40's shape, here §19), not blockers to argue with.
6. **Two phases** (§4): the Phase-1 run ends after E1.MC; the Phase-2 run starts with `START=E2.1` after
   the catch-up rebase.

### §12.1 Safe defaults (taste questions never stop a card)

| # | Decision point | Safe default |
|---|---|---|
| SD-a | A `Container`-typed record that is a weapon (Pistol: `CONTAINS 1|FirearmBullet=1`) | **replaced by the operator's ruling (§31 R2, "Container tag, but not weapons"):** a container is `Container`-typed **and** states no base damage **and** has no `Weapon` type word (§20: Wrist Launcher); Pistol is not one, Powder Horn is; logged once per card when the rule excludes a record
| SD-b | A record's weight is unknown (`weight_and_cost()` → `None`) inside a container | contents weigh what the engine already says (nothing); the container's "x of y" counts only weighed copies; the same "weight could not be verified" posture `encumbrance.rs` has |
| SD-c | The screen's left list order | saved order (the order the engine weighs in); the filter never re-sorts |
| SD-d | A slot that is full when a move asks for it | refuse with the existing `slot_conflict` diagnostic ("head slot is taken by …"); the item stays where it was; the screen shows the refusal word for word (today's posture) |
| SD-e | Ring ×2, Both hands vs Main + Off | `worn_places` as today: two ring places; `wield_two_handed` occupies both hand places (the existing `BodySlot` semantics, `body_slots.rs:329`) |
| SD-f | A set name collision on create/rename | refuse with a diagnostic; never auto-suffix |
| SD-g | The drag library | none; HTML5 `draggable` + `onDragStart`/`onDrop` on rows and location headers, calling the same boundary as "Move to"; no dependency added |
| SD-h | A Typst or TS warning | fix it before claiming |
| SD-i | Usage-limit stop | stop dispatching, write a resume receipt (§12.3) |
| SD-j | No CI on `tranche-21` pushes | CI evidence = the closure PR's `pr-tests`, awaited by E5.4 |
| SD-k | Rebase conflict | `kanban.md`: take upstream, re-apply only your own row, run the duplicate-ID check; `progress.md` append-only sections: keep both; any other file: abort, `blocked-escalated` |
| SD-l | No display on this box for the gear screen | the `run-desktop` skill's screenshot path with an isolated `XDG_DATA_HOME` (R5); the PNGs under `artifacts/epic_4/E4.G/` are what the operator looks at first; the operator's own app is the judge |
| SD-m | SD-40 merges to `develop` between phases | the lead rebases `tranche-21` onto `develop` once, re-runs E1's tests, logs it in `progress.md`; never mid-card |
| SD-n | Screenshot and PDF artefacts grow large | PNG at the app's window size, ≤ 8 images per card, ≤ 2 MB per card directory (`du -sk`) |

### §12.2 Watchdog — one long Workflow run per phase

One run per phase with a recorded run id; recovery starts a **new** run with `START=<card>`; never resume
a run with parallel lanes. No cron. The Phase-2 run ends cleanly at E4.G (resume receipt).

### §12.3 Quota stop rule

Stop dispatching when any `agent()` result matches `/usage limit|rate limit|quota|limit reached/i`, or
the harness shows a weekly reading ≥ 85%. Lanes in flight finish; write a resume receipt. Never
downgrade a model.

### §12.4 Crash-resume recipe

Keep every dirty worktree; read the run id from `progress.md` and `journal.jsonl`; start a new run with
`START=` the first unrun card; the prefix must be byte-identical to `artifacts/cycle_0/sd41-workflow.js`;
log a `retro.py incident`.

## §13 — The gear screen (OPERATOR A1, A4; DEFAULT on placement)

`GearScreen.tsx` *(new)* replaces `<EquipmentPanel …>` at `CharacterSheet.tsx:1666` inside the Gear tab
(`SHEET_TABS` unchanged; the print/UI vocabulary fixture unchanged). Its pure model `gearScreenModel.ts`
*(new)* derives from the load response and the held list: the left rows `{ copy, name, weightLabel,
locationLabel, isContainer, capacityLabel }`, the filter `all | equipped | notEquipped` (Equipped = in a
slot or Equipped), the right tree (18 slot rows with their places; Equipped; Carried with containers as
subtrees, recursively; Not Carried), and the "Move to" targets for a row (every location but the row's own
and anything inside it). No number is computed in TS: weight labels come from `encumbrance.per_item` and
`containerLoads`; AC from the sheet's existing rows. `encumbranceTabModel.ts` keeps its job.

The right panel's AC and weight headline reads the same engine rows the Defense tab and the square block
read (`defense.armor_class`, `encumbrance.total_carried_weight_lbs`), so a move that changes AC is visible
on the gear screen without a tab change. **Revisit:** `§21` Q5.

## §14 — Moving: menu and drag (OPERATOR A4)

Every left row and every right-panel row has a **Move to** button opening a menu of targets (the 17 slots
with free places, Equipped, Carried, Not Carried, every container copy that is not the row or inside it);
keyboard: Tab to the button, Enter/Space opens, arrows choose, Enter moves, Esc closes (a `<select>`-free
menu with `role="menu"`). The menu calls `moveEquipmentCopy` *(new boundary)* → `move_equipment_copy`.
Drag: rows are `draggable`; slot rows, Equipped, Carried, Not Carried and container rows are drop targets;
a drop calls the same boundary with the same arguments; the drop handler is a pure function
(`dropTarget(event, model) -> Location | null`) with a unit test. ui-smoke has no drag op
(`census.txt §5`: ops are click/key/type/select/scroll/wait/nativeFile), so every smoke row uses Move to;
E4.G's eye covers drag. The set picker (a `<select>` of set names + New / Rename / Delete) calls the
`equipmentSets` boundary *(new)*.

## §15 — ui-smoke rows (DEFAULT on names; R24 one writer)

E3.3 adds ≥ 6 rows (`sheet-gear-filter-equipped`, `sheet-gear-move-to-slot-ac`, `sheet-gear-move-to-carried`,
`sheet-gear-not-carried-weight`, `sheet-gear-set-create-activate`, `sheet-gear-container-nest`,
`sheet-gear-container-over-capacity`) and rewrites the six rows the screen replaces
(`sheet-equipment-lists`, `sheet-equipment-conflict`, `sheet-equipment-unequip`, `sheet-equipment-equip`,
`sheet-equipment-left-behind`, `sheet-market-buy-weapon-worn` → `sheet-market-buy-carried`); `npm run
ui-smoke:doc` regenerates the inventory in the same commit. The 31 equipment/market/custom rows today
(`census.txt §5`) are the population; the receipt names which it ran. SD-40's E4.2 is the last SD-40
writer of `spec.json`; SD-41 writes it only after SD-40 merges (§4), so R24 holds by phasing.

## §16 — Engine-owned numbers (STANDING; AGENTS.md twin rule; `../SD-27-future-state-book-content-ingestion/decisions.md §29.1`)

AC, carried weight, load, remaining pounds, container loads and the weapon rows are computed once in Rust
and rendered by the screen and the print. The only new computation is `containers.rs` (contents sums and
the capacity comparison, numbers that land on the screen and the sheet). No TS and no Typst sums a weight.
**Enforced by:** NR-2's greps; E2.MC, E3.MC, E4.MC read the diffs.

## §17 — PCGen is a functional reference only (STANDING, 2026-09-07; OPERATOR in the request)

"It's not fine to include 1 line of java in our live code." The author read PCGen's documentation of its
Equipping and Purchase sub-tabs (`$PCGEN_REPO_DIR/docs/tabpages/inventory/{inventoryequipping,inventorygear}.html`,
resolved per `scripts/pcgen-oracle-pin.env`) and describes the behaviour in our words in
`technical-design.md §1`: a split panel; equipment sets named by the player, printable separately; items
marked Equipped / Carried / Not Carried; containers that hold items; carried weapons printed in the weapons
section. No PCGen source, template or class structure enters `apps/`, `src/`, `crates/` or `data/`
(`pcgen-residue-gate` in `scripts/verify.sh`; the residue check in every MC).

## §18 — Closure order (STANDING, operator-pinned 2026-09-14)

All cards `complete` → retrospective written and cited → full worktree/branch sweep → **release notes**
→ **architecture truth-up + claims critic** → **graphify LAST** over a clean tree at `origin/tranche-21`
→ **PR as the final action**. The operator merges.

## §19 — Discovered cards and fence exits

A defect found mid-run that is in the DoD becomes a card appended to `kanban.md` and `epic-breakdown.md`
by the card that found it (with the row-count constant in G-8 and `kanban.md` re-counted in the same
commit, R18), never a note. E4.G's defects are discovered cards in E2/E3/E4. A lane that needs a file
outside its FILES YOU OWN returns `declined` with `ownedBy:<card>` unless that file's owning epic merge
check is `complete`; then it edits under its own criterion and re-runs that epic's gate, naming both in
the receipt.

## §20 — Pre-launch corrections

The lead re-reads this package against the repo before dispatch and records each corrected claim here
(package said | code shows | effect). Rows the author already corrected while writing:

| Brief said | Code shows | Effect |
|---|---|---|
| "CarriedItemDto ~:290" | `pub struct CarriedItemDto` at `character_hub.rs:298`; `EquipmentEffectsDto` at 262; `EncumbranceDto` at 319 | F-6 quotes the lines |
| "equipment_catalog.rs slots" | slots live in `src/rules_core/body_slots.rs` (`BodySlot`, 18 variants; `slot_table()` reads `data/equipment_types.json`, 26 slot rows, Ring capacity 2); `equipment_catalog.rs` lists and filters the market | E1/E2 name `body_slots.rs`, not the catalog, for slot logic |
| "which corpus items are containers and whether capacity data exists" | two sources: the `Container` tag in sheet rules (232) and the `CONTAINS` token in corpus records (138; 70 with a weight); the app reads no capacity today (0 hits) | §7 |
| "how equip/carry state is stored on disk" | `equipment=item:<id>:<equipped\|selected_inactive\|absent>` lines in `authoritative_character_input.txt`; the slot is never stored | §5.3 chooses a sidecar; the input format is untouched |
| "custom items … custom named pouch" | `CustomRecordDto` has `slot`, `worn`, `left_behind`, no weight, no base item | §11 adds `base_item_id` |
| "138 records with a `CONTAINS` token (70 / 43 / 25)" (§7, this table, E1.1) | 210 of 7,804 equipment files: 88 lb / 73 `UNLIM` / 49 `*` (`grep -rl CONTAINS data/corpus --include=*.json \| grep /equipment/ \| awk 'END{print NR}'` → 210); `census.py:41` globbed `equipment/*.json` and skipped `equipment/<subdir>/` (3,805 files); base Backpack is `general/backpack.json`, `CONTAINS UNLIM\|Any` | E1.1 tests pin the corrected figures (sd41-e1.1, 2026-11-12) |
| "232 container-tagged records" | 232 files, 265 rows; 123 of the rows also tagged `Weapon` | E1.1 names which population it pins |
| SD-a: "ammunition holders (Pistol, Powder Horn) have no `Container` tag" | both carry `Container` (sheet rules `ultimate_equipment/equipment/pistol.json`, `powder_horn.json`) | SD-a's rule needs replacing; E1.1 blocked-escalated, receipt R2 |
| E1.1 FILES YOU OWN "`equipment_record.rs` (+ `sheet_rule.rs`)" | the live record is read from `data/corpus/<book>/_settled/equipment.json` (written by `codex-ingest`'s `gen_settled_corpus`) and carries no tags or tokens; live `raw_tokens` reads are residue-gate hits | E1.1 blocked-escalated, receipt R1 |
| §31 R4 / E1.8 / E1.11: "`build_equipment_catalog()` … reads the settled bundles" | `build_equipment_catalog()` (`desktop/equipment_catalog.rs:468`) chains the compiled per-book tables (`src/rules_core/rules_catalog/*/equipment_tables.rs`) and `equipment_gap_tables`; it reads no `_settled/equipment.json` | E1.9's price fix reaches the Market only through the regenerated compiled tables (its (b) already names them); the settled regen serves the live record, not the Market (sd41-e1.8) |
| E1.8 / E1.9 / E1.11 / G-8 / G-10: `cargo test --locked -j 8 --test market_price_census -- --test-threads=8` → 1 / 2 / 3 passed | the file lives in `apps/desktop/src-tauri/tests/` (the crate has no lib target; the catalog modules are `#[path]`-included), so their own `#[cfg(test)]` suites compile in too: unfiltered the binary holds 134 tests | commands now `cd apps/desktop/src-tauri` and filter `market_price` (1 passed, 133 filtered out) (sd41-e1.8) |
| E1.9 (a) formula kinds: `WT*N`, `BASECOST*N`, `+N` | of the 1,763 unpriced Market rows, 1,278 are priced only by their eqmods; their eqmods' costs are `%SPELLLEVEL`/`%CASTERLEVEL` (scrolls 558, wands 351, potions 108 rows), `%CHOICE` (111) and `%CHARGES` formulas whose inputs are the `EQMOD:` token's own arguments (`SPELLLEVEL[2]CASTERLEVEL[3]`, `\|1721`, `CHARGES[50]`) | d20pfsrd list = 1,477 rows if E1.9 evaluates only its listed kinds, 346 if it also evaluates the in-record `%` kinds (`artifacts/epic_1/E1.8/market-prices.json` `d20pfsrd_bound`); the lead decides E1.9's kinds before E1.11 is sized (sd41-e1.8) |
| §31 R2 / E1.1: "not a weapon" = "no `DAMAGE` token" alone | of 209 corpus records whose `TYPE` holds `Container`, 130 have neither `DAMAGE` nor the `Weapon` type word, 78 have both, **1 has `Weapon` without `DAMAGE`**: Ultimate Intrigue `wrist_launcher.json` (`CONTAINS UNLIM\|Dart=1`); its desktop twin `fixtures_src/equipment/equip_wrist_launcher.json` would make the fixture bundle move, against R1 (Python recursive walk over `data/corpus/**/equipment/**/*.json`, first `TYPE`/`DAMAGE` token; pinned by `container_capacity_population_is_210_88_73_49`) | E1.1 settles `container` = `Container` and no `DAMAGE` **and** no `Weapon` type word — the operator's "not weapons" by either signal; 130 settled containers (32 lb / 50 unlimited / 48 unstated); fixture sha256 unchanged (sd41-e1.1, 2026-11-12) |
| §31 R1 / E1.1: "`cd crates/codex-ingest && cargo run --locked --bin gen_settled_corpus`" | the binary reads the relative `data/corpus`; from `crates/codex-ingest` it prints `gen_settled_corpus: no corpus roots under data/corpus; run from the repo root` and exits 2 | run from the repo root: `cargo run --locked -p codex-ingest --bin gen_settled_corpus` (sd41-e1.1, 2026-11-12) |
| §8 / F-24: the `.pcg` population is "37 saved of 65 files" | 65 files (13 repo eqm fixtures + 52 under the oracle); 2 refused by the converter as older than the format; **63 convert, 63 hold equipment (338 copies), 63 of 63 round-trip** (`cd crates/codex-ingest && cargo test --locked -j 8 --test equipment_sets_pcg_population -- --nocapture`; `find` over both dirs → 65; `grep -L -E '^(PCGVERSION\|CHARACTERNAME):'` → 2). "37 saved" is SD-40's other predicate | E1.2's receipt states its own predicate (converts) (sd41-e1.2) |
| E1.2: the theorem over "every seed input" from the root crate | the three PF starter seeds are composed in the desktop crate (`character_hub.rs::starter_seeds` → `compose_character_input`), out of the root lib's reach under the Phase-1 `apps/` partition; the root population is the 113 `tests/fixtures/**/*.txt` inputs holding an `equipment=` line (`git grep -l '^equipment=' -- 'tests/fixtures/**/*.txt' \| awk 'END{print NR}'` → 113; a Python `rglob` → 113) plus the sd14 saved root: 114 inputs, all three active states | the starter seeds' round trip moves to E2.1 (`equipment_sets_on_load_writes_default_once` asserts it for each PF starter seed) (sd41-e1.2) |
| E1.2 R21 RED: `error[E0433]: … equipment_sets` | on the base the `--lib equipment_sets` filter matches **0 tests** (`0 passed; 3099 filtered out`, a fail by the 0-match rule); with the tests written first the build fails `E0422`/`E0425` (`cannot find … CopyKey`, `cannot find function load`) | E1.2's receipt quotes both (sd41-e1.2) |
| E1.3: "a backpack (60 lb) holding a pouch holding 10 lb of rations → backpack 10, pouch 10" | the CRB `Pouch (Belt)` weighs 0.5 lb (`_settled/equipment.json`) and sits inside the backpack, so the backpack holds 10.5; the CRB Backpack is `Unlimited` until E1.12 (§32), so the 60 lb backpack in the test is a Create item (§25, `(CopyKey, 60.0)`) | `containers_nested_sum_reaches_the_outer` asserts 10.5 / 10 (sd41-e1.3) |
| E1.3 R21 RED: `error[E0433]` | on the base `--lib containers_` matches **0 tests** (`0 passed; 3108 filtered out`, a fail by the 0-match rule); with the tests written first: `E0425` ×7 (`cannot find function \`container_loads\``), `E0422` ×1, `E0433` ×1 | E1.3's receipt quotes both (sd41-e1.3) |
| E1.3: `container_loads(set, selections, corpus)` | `EquipmentSelection` carries no `+N`; the copy's bonus (and so its `CopyKey`) is derived by the caller (E1.2's tests read the enhancement eqmod; the desktop has `copy_bonus`) | the function takes `held: &[(CopyKey, &str)]` (each copy's key and item id) and `created: &[(CopyKey, f64)]`; the caller keys its copies once (sd41-e1.3) |
| E1.4 FILES YOU OWN: `damage_total.rs`, `combat.rs`, "their test modules" (root only, Phase 1) | `resolve_weapon_damage_breakdown` is called **directly by the desktop crate** (`character_hub.rs:2879` load response, `rule_system_adapter.rs:435`; `git grep -n resolve_weapon_damage_breakdown -- apps`), and the desktop test `enhancement_rows_a_stowed_plus_one_adds_nothing` (`character_hub.rs:10720-10725`) asserts a `SelectedInactive` longsword gives `is_empty()` — the exact row E1.4's criterion requires. Root pins of the old contract outside the row: `tests/worn_vs_carried.rs:122-123,155` and `src/rules_core/corpus_loader.rs:634-636` (test). The widened rows also reach the Weapons tab and the PF print (`print/blocks/print_pf_blocks.rs`, `gear.rs`) unmarked until E2.4/E4.1 | E1.4 cannot land green inside the Phase-1 `apps/` partition: `declined`, ownedBy E2.4 (sd41-e1.4). Lead action: either grant E1.4 the one desktop test edit as a named `apps/` exception (like the price test) plus the two root test files, or fold E1.4 into E2.4 |
| E1.4 R21 RED: `error[E0560]: … no field in_hand` | on the base `--lib carried_weapon` matches **0 tests** (`0 passed; 0 failed; … 3112 filtered out`, a fail by the 0-match rule) | quoted in E1.4's receipt (sd41-e1.4) |
| §33 / E1.4 R21 RED: `error[E0425]: cannot find function resolve_weapon_damage_breakdown_with_carried` | on the base `--lib carried_weapon` matches **0 tests** (`0 passed; 0 failed; … 3112 filtered out`, a fail by the 0-match rule); with the tests written first: `E0425` ×3 `cannot find value \`resolve_weapon_damage_breakdown_with_carried\`` (the tests pass it as a `fn` pointer, so rustc says *value*), `E0560` ×1 and `E0609` ×4 (`no field in_hand`) | quoted in E1.4's receipt (sd41-e1.4, 2026-11-13) |
| §22 / E1.5: "the damage total row … prints `1d3+3` for Aldric" | Aldric is Strength 17 + the Human +2 = 19, **+4** (`character_hub.rs::starter_seeds`, `"strength"` as the bonus ability; his longsword prints `1d8+4`, `character_hub.rs:21130`) | `unarmed_row_is_last_and_nonlethal_for_aldric` asserts `1d3+4` and attack `+7` (base attack +3, Strength +4) (sd41-e1.5) |
| E1.5: "`damage_total.rs` appends one `WeaponDamageBreakdown` for every character" + "G-1: `NEW_ROWS` += `combat.weapon_*.unarmedstrike`, every seed" | appending to `resolve_weapon_damage_breakdown` (the desktop's live caller, `character_hub.rs:2879`) turns desktop tests that pin the row set red (`character_hub.rs:10722` `is_empty()`, `12683`/`12791` exact `combat.weapon_attack_bonus.*` lists) — §33's shape | by §33 option (c) the Unarmed row is appended to `resolve_weapon_damage_breakdown_with_carried` only (in hand → carried → Unarmed), and its `combat.weapon_*.unarmedstrike` rows come from a new `pilot_compute::unarmed_strike_combat_rows(base, input)` (the headless per-weapon melee path over the record); the pilot's own rows and every seed render are unchanged in Phase 1; E2.4 calls both when it switches the callers, and the G-1 `NEW_ROWS` move there (sd41-e1.5) |
| E1.5 R21 RED: `error[E0560]: … no field unarmed` | on the base `--lib unarmed_row` matches **0 tests** (`0 passed; 0 failed; … 3116 filtered out`, a fail by the 0-match rule); with the tests written first: `E0609` ×11 (`no field \`unarmed\`` / `no field \`nonlethal\``), `E0425` ×1 (`cannot find value \`UNARMED_STRIKE_ITEM_ID\``), `E0432` ×1 (`unresolved import … unarmed_strike_combat_rows`) — the tests read the fields, they construct no breakdown | quoted in E1.5's receipt (sd41-e1.5) |
| E1.6 acceptance: `cargo test --locked -j 8 --lib uses_ammunition -- --test-threads=8` → 3 passed; R21 RED `error[E0560]: … no field uses_ammunition` | the filter also matches `body_slots::tests::enhanceable_comes_from_the_tags_and_refuses_ammunition_barding_and_magic` (`ref`**`uses_ammunition`**): on the base it prints `1 passed; … 3119 filtered out`, GREEN would be 4; with the tests written first: `E0061` ×1 (the carried entry point takes a `quantities` argument) and `E0609` ×6 (`no field ammunition_carried` ×4, `uses_ammunition` ×2) | the command is `--lib ammunition_tests::uses_ammunition` → 3 passed (base: 0 match, a fail) (sd41-e1.6) |
| §23 / E1.6: "132 sheet-rule records" carry `Projectile` | the live record is the corpus record: **87** of the 7,300 token-bearing corpus equipment files hold `Projectile` in `TYPE` (77 name a `CONTAINS` family); **228** are `Ammunition`-typed (the sheet-rule 83 is another population, and the corpus count includes material/eqmod rows such as Ultimate Combat `Stone`); **30** state `BASEQTY` > 0 (`ammunition_population_is_87_77_228_30`, settled bundles vs `raw_tokens`; a Python glob over `data/corpus/*/equipment/**` agrees). Inherited-copy records (`lst_inherited_copy`, no tokens: CRB `Longbow`, `Crossbow (Light)`, `Sling`, `Arrows (20)`) take the facts from their `base_item` one hop, as damage does | E1.6 pins the corpus figures (sd41-e1.6) |
| E1.6 / E1.7: E1.7 settles `base_quantity`; E1.6 counts "a copy without a quantity" as `base_quantity` | E1.6 runs first and its own test (3 × `Arrow, Flight (20)` → 60) needs the bundle size | E1.6 settles `base_quantity` (`BASEQTY` → `base_quantity_of`) and adds `base_quantity() -> u32` (1 when absent); E1.7 consumes them; its `--lib base_quantity` test is green on its base, so E1.7's R21 is the `extra_weighed` / `piece_quantity` filters (sd41-e1.6) |
| §31 / §2.1 for E1.6: the new fields settle and "the desktop fixture bundle stays byte-identical" | 41 of the fixture's 279 records are `Projectile` weapons; `gen_settled_corpus` gives them `uses_ammunition` + `ammunition_family` (Python diff of the regenerated fixture vs `HEAD`: 41 records, those 2 fields only); Phase 1 may not write `apps/` | E1.6 restored the fixture to its base bytes (`74bb85a4…`) and carded its regeneration as **E2.9** (discovered, Phase 2, before E2.4); until E2.9, `gen_settled_corpus -- --check` reports that one file as drift (sd41-e1.6) |
| §28 / §30: "3 × `Arrows (20)` = 60 arrows" (bundle size from `BASEQTY`) | the 5 CRB ammunition bundles are `lst_inherited_copy` records ingested without their `COPY=` line's tokens (`cr_equip_arms_armor.lst:23` states `BASEQTY:20`), so their settled record has no `base_quantity`: `Arrows (20)`, `Bolts (Crossbow/10)`, `Bolts (Crossbow/Repeating/5)`, `Bullets (Sling/10)`, `Darts (Blowgun/10)` count 1 piece per copy (Python over the settled bundles: 15 ammunition-by-stand-in records, 5 with a bundle name N > 1); APG/UE bundles (`Arrow, Flight (20)`) carry their own `BASEQTY` and count right | discovered card **E1.13** (convert lane) (sd41-e1.6) |

## §21 — Questions only the operator can settle (defaults taken; the run does not wait)

| # | Question | Default taken (one card's criterion) | Alternative |
|---|---|---|---|
| Q1 | Where is a bought copy in the **other** sets? | Carried everywhere (an unnamed copy is Carried, §5.3); no set entry written | Not Carried in every set but the active one (then a purchase writes one entry per set) |
| Q2 | Does a carried (not in hand) weapon get attack rows on the sheet and the print? | yes, after the in-hand rows, marked "(carried)", main-hand assumption (§9) | name + damage dice only, no attack total |
| Q3 | Must a custom container name a base item? | **Answered by the operator the same day (§25, "Optional container + slot"): no** — a Create item may be a container with a capacity in lb and no base | — |
| Q4 | Weight-reduction containers (bag of holding kinds, 25 records) | "no stated capacity"; contents weigh in full (DEF-1) | model `*N` as a fixed weight (contents weigh nothing) in SD-41 |
| Q5 | Where the gear screen lives | replaces the Equipment panel in the Gear tab (§13) | a new "Equipment" tab (changes `SHEET_TABS` and the print/UI vocabulary fixture) |

| Q6 | Buy Custom: an eqmod whose price is a formula the converter cannot evaluate (97 formula kinds among 1,494 eqmods) | **Answered 2026-11-12 (§30): "Offer them, price by Negotiate"** — the default stands | — |
| Q7 | How far the d20pfsrd price list goes | **Answered 2026-11-12 (§30): "Every Market item"** — a closure criterion; `price-unknown.json` empty for Market-visible rows; card E1.11 in batches via the lead's `web-fetch` agent; Negotiate is haggling only | — |
| Q8 | Splitting a bundle (10 of "Arrows (20)" into a quiver) | **Answered 2026-11-12 (§30): "Count single arrows"** — pieces; `quantity` per stack in the sidecar's `copies` map; migration = a bundle copy without a quantity is `base_quantity` pieces; old-save weight unchanged | — |
| Q10 | Which logo file per game system (PF1e, SF1e, CPR) for the portrait box when no portrait is stored (§34; trademarks, public repo) | **none taken — a ruling request** (`progress.md ## Open blockers`): E4.4 ships the three asset paths and is `blocked-escalated` on the files until the operator supplies them | the operator supplies three files, or rules "no logo: print nothing in the box" |
| Q11 | A `speed.climb` engine row for races with a climb speed (Vanara, `class_shared_core.rs:2733`) so the Speed grid prints a Climb value | not in SD-41: Climb prints "—" unless a `speed.climb` row exists (§34 (6); paper-sheet rule) | add the row in E1 for the races that state one (a value that lands on the sheet) |
| Q9 | Where campaign membership is the truth | the character's `campaigns.json` (the app is master, §29); the campaign's `partyCharacterIds` is kept in step by the dialog | the campaign snapshot alone (then every save must read the Drive folder to know where to copy) |

The author's recommendation is the default in every row.

## §22 — Weapons: an Unarmed row, always (OPERATOR, 2026-11-12)

> "Weapons - always include unarmed as an option"

Answers relayed the same day: placement = **"Last row, full numbers"** — the Weapons block (screen and
print) always ends with an Unarmed strike row showing the engine's real attack bonus and damage (1d3
nonlethal Medium / 1d2 Small; monk unarmed damage progression when the character has it; Improved Unarmed
Strike makes it lethal), computed in Rust, rendered by UI and print (twin rule). Timing = **"With SD-41
gear work"** — carded with the weapons-block cards: E1.5 (engine row), E2.4 (tab), E4.1 (print).

**Census** (`artifacts/epic_0/census.txt §8`): the engine has no unarmed attack or damage row today —
`grep -n -i unarmed src/rules_core/damage_total.rs src/rules_core/pilot_compute/combat.rs` → one doc
comment each. The corpus record exists: `data/corpus/core_rulebook/equipment/arms_armor/unarmed_strike.json`
(`DAMAGE 1d3`, `CRITMULT x2`, `COST 0`, `WT 0`, TYPE holds `Unarmed.Monk.Natural.Light`). The monk
progression exists: `src/rules_core/pilot_compute/class_monk.rs:470 ground_unchained_monk_unarmed_strike_damage`
(1d6 / 1d8 / 1d10 / 2d6 / 2d8 / 2d10 bands, line 175) — the card reads whether the CRB monk path shares it
and names the function it reuses. Improved Unarmed Strike: 54 sheet-rule files name it; Monk grants it
(`feat_effects.rs:3218`). **DEFAULT on the shape:** the row is a `WeaponDamageBreakdown` with
`weapon_item_id = "item:unarmed_strike"`, `in_hand: true`, `unarmed: true`, `nonlethal: bool`, built from
the corpus record (never a literal in code), Small size → `1d2` by the record's size step; the attack row is
the melee attack path with the record (a light weapon, finesseable). **Order:** in hand → carried →
Unarmed last. **Enforced by:** E1.5 tests (Aldric Fighter 3 → `1d3+4` nonlethal (`1d3+3` corrected, §20); a Monk 4 → `1d8` lethal;
a Small character → `1d2`); G-1: an added row id `combat.weapon_*.unarmedstrike` for every seed, named in
`NEW_ROWS`.

## §23 — Weapons: ammunition boxes only for weapons that use ammunition (OPERATOR, 2026-11-12)

> "weapons - only show ammunition if the item has ammuntion.  ie:  swords don't have ammo, don't show it.
> crossbows do have ammo, show the ammo indicators"

Answers: timing = **"With SD-41 gear work"**; scope = **"Yes, screen and print"**. Today page 1 prints one
global "Ammunition" row of boxes under the weapons (`resources/print/looks/weapons.typ:88`); the UI shows no
ammunition (`grep -n -i 'ammunition\|ammo' apps/desktop/src/characterHub/CharacterSheet.tsx` → 0). New rule:
each weapon that uses ammunition gets its own tick boxes under its row; weapons without (swords, thrown
daggers, Unarmed) get none; no global row.

**Census:** the corpus marks ammunition users by the `Projectile` tag (sheet rules: 132 records; of them
Bow 26, Crossbow 24, Sling 11, Firearm 63, the counts overlap and include "(weapon)" twins); `Thrown` (141)
is not ammunition; `Ammunition` (83) tags the ammunition itself; 79 corpus weapon records also carry a
`CONTAINS` token naming their ammunition family (`Pistol: 1|Firearm Bullet=1`). **DEFAULT:** `uses_ammunition`
= the `Projectile` tag (E1.6, root crate; decided by the engine from corpus data, never a TS list);
`ammunition_carried` = the count of carried copies whose record carries the `Ammunition` tag **and** whose
name or type matches the weapon's `CONTAINS` family when the weapon has one (bows: `Arrow`; crossbows:
`Bolt`; firearms: `Firearm Bullet`; slings: `Bullet (Sling)`), else the carried `Ammunition` count of any
kind — the figure the stacked rows give (§28). The screen and the print show **20 tick boxes** per
ammunition weapon (write-in, SD-m's ruled blank) with the label "Arrows: 60 carried" when the engine
counts any; the print's global row goes. **Enforced by:** E1.6 tests (longbow yes, longsword no, dagger
no, pistol yes); E2.4 (tab); E4.1 (print: `grep -c '"Ammunition"' looks/weapons.typ` → 0 as a global row).

## §24 — The Market screen: table, sort, description, Only Show Affordable, Buy Custom (OPERATOR, 2026-11-12)

> "on the market screen:  I would prefer the list of armor be more of a table view that shows the item and
> the price.  Include a space at the bottom of the window for description that shows the description of
> the item that is highlighted.  You should be able to sort any list by the name or the price.  Include a
> checkbox called 'Only Show Affordable' that filters out items that the character lacks sufficient funds
> to purchase.  We also need a buy custom button.  Take a look at the pcgen code for how this works.  It
> allows the assignemtn of custom names and abilities to the item being purchased."

Answers: Buy Custom = **"Full rules builder"**: base item + custom name + masterwork + enhancement +1…+5
+ special abilities (flaming, keen …) + special material (mithral, cold iron …) + size; price and weight
computed from the rules (corpus data, in Rust); enhancement reaches AC / attack / damage through the
engine; ability text prints as text, dice literal (paper-sheet rule). PCGen's custom-equipment builder is
read for FUNCTION only (`technical-design.md §1`: right-click → Create custom item; properties with
prerequisites; Name, Sprop, Cost, Weight editable; OK adds to the list, Buy also buys). Columns = **"Item,
Price, Weight"** (+ Slot by §26), every list sortable by any column header. Description box at the bottom
shows the highlighted item's corpus description. 'Only Show Affordable' filters by the character's funds
at the active price mode (three modes exist: `cashless`, `standard`, `characterBuild`,
`apps/desktop/src/characterHub/priceMode.ts`; funds from `load_character_money`'s `total_copper`). Package
= **"SD-41, with the gear work"**; custom-bought items land as Carried with their custom names, which the
gear screen's container logic uses.

**Census:** `MarketDialog.tsx` (402 lines) lists `list_market` entries as buttons with `+1…+5` variants
(`MarketVariantDto { bonus, name, price_gp, modifier_key }`), category tabs (`weapon | armor | shield |
gear | magicItem`), a search, a book filter, and a Sell list of held copies; no sort, no description box,
no weight column; `EquipmentCatalogEntryDto` already carries `cost_gp`, `weight_lbs`, `types`,
`description`. The `+N` price rule is `Enhanceable::price_gp` (`body_slots.rs:412`: masterwork 150/300 +
N² × 1,000/2,000) and the modifier is attached through `applied_modifiers` (`attach_equipment_modifier`).
Equipmods in the corpus: **1,494** records, COST flat 608 / formula 97 / none 789; **140** tagged `Material`
(`Paueliel Wood: WT*15` is a formula kind); `Resizable` on 733 sheet-rule equipment records.
**DEFAULTS:** (a) Buy Custom is `purchase_custom_equipment` (E2.6): the base selection plus
`applied_modifiers` for the enhancement (today's path), each chosen special ability, the material and the
size, in that order, with the price from each eqmod's flat `COST` and the formulas the card can evaluate
(`WT*N`, `BASECOST*N`, the `+N`-equivalent abilities' bonus-equivalent rule: a +2-equivalent ability prices
as the next enhancement step); an eqmod whose formula the card cannot evaluate is **offered without a
price and routes through Negotiate** (§26), never a guessed number. (b) The custom name is stored per copy
in `equipment_sets.json`'s `copyNames` map (`CopyKey → name`), shown everywhere the copy is named.
(c) Sorting and filtering are pure functions in `marketModel.ts` (new) over the DTO; the description box
reads `description`. **Rules gap → `§21` Q6.**

## §25 — The Market's Create option (OPERATOR, 2026-11-12)

> "On the market screen, include a Create option that allows the user to manufactor a custom items.  They
> can free text the name and description, and then enter a weight."

Answers: price = **"Optional price"** — a price field starting at 0; above 0 it is deducted from the
character's funds like a normal purchase. Kind = **"Optional container + slot"** — an optional 'Container'
toggle with a capacity in lb, and an optional body slot (default none); a created item lands as Carried
like every Market item. Distinct from Buy Custom (which starts from a corpus base item): Create makes an
item with **no corpus base**. It persists with the character, round-trips save/load/clone, prints on the
inventory page with its weight, counts toward encumbrance when carried, works as a container in the gear
screen, and is editable/removable afterwards.

**Census:** `character_custom.rs` stores custom records in `<character root>/custom.json`
(`CustomRecordDto { id, name, description, stats, bonuses, slot, worn, left_behind }`, F-12: no weight, no
price, no capacity); `list_held_equipment_at_root` lists a custom item only when it has a slot or a bonus
(`character_hub.rs:5015`); the engine never weighs a custom item. **A no-base item fits that store**
with three new optional fields. **DEFAULTS:** (a) `CustomRecordDto` (equipment kind) gains `weight_lbs:
Option<f64>`, `price_gp: Option<f64>`, `container_capacity_lbs: Option<f64>` (serde default; old
`custom.json` loads unchanged); `create_custom_item` (E2.3) writes the record, deducts the price at the
active price mode, and lists it Carried (unnamed in every set). (b) The engine weighs custom items:
`compute_encumbrance` takes extra weighed copies `(id, weight_lbs, active_state)` (E1.7, root crate), so
the total stays one Rust sum (NR-2) and the projection (§6) covers them. (c) The held list includes every
custom equipment record (the `:5015` filter widens to "equipment kind"). (d) **Edit and remove live in the
Custom tab** (the existing `CustomDialog` edits custom records; it gains the three fields); the gear screen
moves, sells (no price → Negotiate), drops and discards them like any copy. (e) This supersedes `§11`'s
`base_item_id`: a custom container needs no base — it is either a stock record tagged `Container`, a Buy
Custom item on a container base, or a Create item with a capacity. `§21` Q3 is answered.

## §26 — Prices: missing and negative; Negotiate; Market columns (OPERATOR, 2026-11-12)

> "I see a few items with no price:  Adamantine Breastplate - we need to find the actual price for all
> items.  Another example is Astral Armor that has a negative price.  All that needs to get cleaned up.  If
> you cant find the price in the pcgen files, get it from d20pfrsd.com.  ie:
> https://www.d20pfsrd.com/magic-items/magic-armor/specific-magic-armor/adamantine-breastplate/.  Oh, all
> items in marketplace should include 2 more colums: weight and slot"

> "allow the user to specify the price on purchase - which gives me an idea, let's add a negotiage button
> next two buy that allows the user to customize what they pay.  We need a similar option where we can
> negotiate the sale price if selling something."

Answers: (1) Source = **"Fix converter + cited list"**: first fix the tool-side converter so prices derive
as PCGen's data defines them; prices still missing come from d20pfsrd.com into ONE checked-in list with the
page URL per item (a converter input, tool side; the live app reads only converted data); a test fails if
any Market item has a missing or negative price (allowing only items explicitly listed as price-unknown).
(2) A **Negotiate** button next to Buy (the user enters what they pay; default = the listed price at the
active price mode; a price-unknown item's Buy routes through Negotiate) and a Negotiate option on Sell
(default = the app's current sell rule: `refund_copper`, `character_hub.rs:3741`, 50 % standard / 100 %
character build / 0 cashless). (3) Market columns become **Item, Price, Weight, Slot**, all sortable.

**Census** (`census.txt §8`; over `data/corpus/**/equipment/**`, 7,804 records, all `in_scope` — a proxy
for the Market, which serves compiled per-book tables plus corpus gap rows, `equipment_catalog.rs:468`;
E1.8 derives the Market's own count through `build_equipment_catalog()`): price positive 4,854; **none
2,476** (Equipmods 961, magic items 1,034, arms & armor 322, general 158; by book: core 1,591, UE 200, UP
171 …); **zero 470** (core 215, UE 79, UP 71 …; magic items 116, arms & armor 126, equipmods 153, general
75); **negative 4** (Astral Armor −150, Astral Juggernaut −150, Demon Armor Claw Attack −2,300, Demon
Prince Armor Claw Attack −18,300). **Root causes, proved on the two named items:** *Adamantine Breastplate*
(`data/corpus/core_rulebook/equipment/arms_armor/adamantine_breastplate.json`) is converted as
`name: "Breastplate (Base)"`, `cost_gp: None`, **no raw tokens** — the converter resolved the PCGen row
(a `BASEITEM:Breastplate` + `EQMOD:ADAMANTINE` composite) to its base template and dropped the material;
PCGen's own price is base + the material eqmod's formula, which the live catalog cannot read because the
tokens are gone. *Astral Armor* carries PCGen's literal `COST:-150` beside `EQMOD:Special Quality ~
Masterwork ~ Armor` (+150): PCGen sums COST with the eqmods' costs (→ 0 for an astral suit); the converter
copies COST alone. The negatives are all of this shape (an eqmod-offset literal); the 2,476 `none` rows are
dominated by eqmods (not Market rows) and magic items whose price is a formula on their eqmods. **DEFAULTS:**
(a) E1.8 (tool lane `convert`) is a census card: a `codex-ingest` test prints the Market's missing / zero /
negative rows by book and category and names the converter cause per class — that count, not this proxy,
bounds the d20pfsrd list. (b) E1.9 fixes the converter: composite rows keep their base + eqmods and price
as base + Σ eqmod cost (flat and the formulas the converter evaluates), re-converts the equipment corpus
(`docs/governance/book-ingestion-playbook.md`'s path; the diff must touch only `cost_gp`, `weight`,
`raw_tokens`, `name`, never `license` / `pi_*` fields — the "generated artifacts mutated post-hoc" hazard),
regenerates the compiled tables, and reads `data/price_overrides/d20pfsrd-prices.json` *(new)* —
`[{ key, price_gp, url, fetched_at }]`, one row per item with the page URL — for what is still missing; the
gate test allows only keys in `data/price_overrides/price-unknown.json` *(new, with a reason each)*. (c)
**Network:** `curl -sI https://www.d20pfsrd.com/` answered `HTTP/1.1 410 Gone` from this box on 2026-11-12;
the card tries a browser user agent, else the lead's session fetches the pages (a `web-fetch` agent) and
the card consumes the list; the operator is asked only if both fail (P-9). (d) Negotiate on Buy and Sell
is a `negotiated_price_copper: Option<u64>` on the existing purchase and sell requests (E2.5), applied
instead of the catalog price at the active mode; the receipt row says "negotiated". (e) Slot column =
`body_slots::slot_of(record)` on the catalog entry (E2.5). **Revisit:** `§21` Q7 (how far the d20pfsrd
list goes).

## §27 — Drop goes to Not Carried; Sell and Discard are separate (OPERATOR, 2026-11-12)

> "the dropped option in gear is causing the item to disappear, it should go to the \"not carried\" slot
> instead"

Facts: today "Drop" sells the item back (`CharacterSheet.tsx:749` doc; buttons at `:870` and `:1706`; the
caption `:881` "Dropping an item sells it back under the pricing chosen above"). Answer: **"Sell + Discard
buttons"** — Drop moves the item to Not Carried in the active set (no money, item kept); Sell is a separate
button that sells it (with the Negotiate sale price, §26); Discard deletes it for no money after a confirm.
**Census:** no ui-smoke row drives Drop (`python3 -I -c … 'drop' in json.dumps(r).lower()` → only
`sheet-f7c-samurai-skills-only-skills`, a dropdown); the caption and the two buttons are the surfaces.
**DEFAULTS:** Drop = `move_equipment_copy(NotCarried)` (E2.2); Discard = `discard_equipment_copy` (E2.5:
removes the selection and its set entries, no refund, `confirm` in the UI); the caption reads "Drop keeps
the item and marks it Not Carried; Sell returns money under the pricing above; Discard removes it for
nothing." The three buttons live on the gear screen's rows (E3.1) and the Weapons tab keeps Drop only.
Smoke rows: `sheet-gear-drop-not-carried`, `sheet-gear-discard-confirm` (E3.3).

## §28 — Bulk buying and selling; stacks (OPERATOR, 2026-11-12)

> "We also need to allow someone to buy things in bulk.  No one wants to click buy 50 times to buy 50
> arrows.  Same with Sell"

Answers: **"One row with a count"** — Buy and Sell (and Negotiate) take a Quantity; identical items stack
into one gear row "Arrows ×50" with weight and price × quantity; Move asks how many and splits the stack;
Sell / Drop / Discard also take a quantity. **"Bundles, as the book sells them"** — Quantity counts the
corpus item as sold (3 × "Arrows (20)" = 60 arrows at 3× bundle price/weight); the row shows "Arrows (20)
×3 = 60"; prices and weights stay exact to the book.

**Census:** the store keeps **one `equipment=` line per copy** (`local_store.rs:458`); no quantity field
exists on `EquipmentSelection` (F-5: 97 producers, untouched by NR-3); bundle-shaped corpus records: **68**
whose name holds `(N)` or whose tokens hold `BASEQTY`, **30** with `BASEQTY` (`Arrow, Flight (20)`:
`BASEQTY 20`, `COST 2`, `WT 3`). **DEFAULTS:** (a) **Stacks are a view, not a store change:** copies with
the same `item_id`, `bonus`, custom name and location stack into one row with `quantity` = the copy count
(E2.1's held list gains `quantity` and `copies: [CopyKey]`); weight × quantity is what the engine already
sums copy by copy; existing duplicate entries become stacks on first load **with no migration** (nothing on
disk changes; G-1 holds). (b) `purchase_equipment`, the sell and discard commands and `move_equipment_copy`
take `quantity` (default 1) and act on that many copies of the stack atomically (one save; price ×
quantity, or negotiated total); a move of part of a stack splits it by ordinals (`Inside(quiver)` for 1
bundle, `Inside(pack)` for 2). (c) The row label is `"<name> ×<n>"`, and `"<name> ×<n> = <n × BASEQTY>"`
when the record carries `BASEQTY` > 1 (E1.1 adds `base_quantity()` to the record read; the figure is the
engine's, the TS only formats). (d) The ammunition count (§23) is Σ `quantity × base_quantity` over the
carried ammunition stacks matching the weapon. (e) Splitting below one bundle (10 of an "Arrows (20)") is
not possible: a bundle is the unit the book sells (`§21` Q8). **Enforced by:** E2.1 `held_list_stacks_identical_copies`;
E2.5 `purchase_quantity_buys_n_copies_at_n_times_price`, `move_quantity_splits_a_stack`; E3.1 model tests.

## §29 — Campaign folders: write-through of the character's files; the Campaign field and buttons (OPERATOR, 2026-11-12)

> "for SD41:  when addiing a character to a campaign, the characters data needs to be written to the
> campaign folder.  Every update to the character should be written to that folder.  Story character files
> separately from each other so that they do not collide as might happen if all characters are stored in a
> single json file.  In the UI, make the campaign option clickable - from here a user can add one or more
> campaigns, or remove them from a campaigns.  If the User is in a campaign, expose a campaign button under
> skills that will take the to the campaign manager screen."

Answers: master = **"App is master, folder is a copy"** (one-way write-through: every save of the character
writes a fresh copy to each campaign folder it belongs to; nothing is read back). Files = **"Full character
folder"**: `<campaign dir>/characters/<slug>-<characterId>/` holding the same files the app keeps (the saved
character, `bio.json` incl. the Backstory from SD-40 E3.6, the portrait, `equipment_sets.json`, `custom.json`
…) — one folder per character, atomic write (temp + rename), never a shared JSON. Remove = **"Keep it, stop
updating"** (the folder stays; saves stop going there). Button = **"One button per campaign"** under the
Skills block, each opening that campaign in the Campaign Manager. "Campaign option clickable" = the
Campaign field on the sheet's identity row (`CharacterSheet.tsx` `IdentityPanel`, the "Campaign" label at
`:410`; SD-40 E2.18 built the printed Name/Campaign row) opens a dialog to add the character to one or more
campaigns or remove it.

**Census** (`artifacts/epic_0/census.txt §9`): the campaign dir is `<driveFolderPath>/<folderName>/`
(`campaign_drive.rs:195 campaign_dir_for`) with `.config/<sanitized name>.json` written by
`codex::campaign::local_store::CampaignStore::save` (`src/campaign/local_store.rs:53`, atomic write, nonce
sidecar); `drive_folder_path` arrives **per request** from the UI's Google Drive settings
(`apps/desktop/src/settings/googleDrive.ts:16`; the backend keeps no copy); `partyCharacterIds` is written only
by `EditCampaignScreen.tsx:112` through `drive_save_campaign` (E2.10's nonce check); the Campaign Manager is
`CharacterHubPage.tsx:128` `setMode('campaign-list')` → `CampaignManagerScreen` (`onLoad(campaignId)` opens
`CampaignSheet`); the sheet shows the campaign as read-only text (`IdentityPanel`, `:403–412`). Character
writers: `SavedCharacterStore::save` (171 call sites across 16 files, `grep -rn 'SavedCharacterStore::save('
… | awk 'END{print NR}'`) plus the sidecar writers in the desktop crate (`custom.json` 13 write lines,
`hit_points.json` 5, `money.json` 2, `bio.json` 1, portrait 8, and E1.2's `equipment_sets.json`). A missing
Drive folder is already a surfaced error (`campaign_drive.rs:91`). 5 smoke rows cover campaigns today.

**DEFAULTS:** (a) **Membership lives with the character:** `<character root>/campaigns.json` *(new)* =
`[{ campaignId, campaignName, campaignDir, active }]`, written by the sheet's dialog (E2.7); the dialog also
updates the campaign's `partyCharacterIds` through the existing `drive_save_campaign` (with its nonce) so the
Manager agrees; Remove sets `active: false` and drops the id from the party — the folder stays. (b) **One
mirror function, root crate** (E1.10, `src/saved_character/campaign_mirror.rs` *(new)*): `mirror(root,
memberships) -> Vec<MirrorWarning>` copies **every file** of the character root to
`<campaignDir>/characters/<slug>-<characterId>/` for each active membership (`slug` = `sanitize_filename`
of the display label, lower-cased, spaces → `-`; the id makes the folder unique: two characters named alike
never share a path), by writing `<target>.tmp-<pid>` and renaming over the old folder; `campaigns.json`
itself is not copied. A missing or unwritable campaign dir is a **warning**, never an error: the local save
has already succeeded. (c) **Every writer calls the mirror after its write:** `SavedCharacterStore::save`
(root crate; covers the 171 sites) and each desktop sidecar writer (`save_character_custom`,
`save_character_bio_at_root`, `save_character_hit_points_at_root`, the money writer, the portrait writer,
`equipment_sets::save`'s desktop caller) — E2.7 names each; a **source-scanning test** (the shape
`packaged_resources.rs` uses) lists every `fs::write` / `atomic_write` whose path is under a character root
in the desktop crate and fails if its enclosing function does not call `campaign_mirror::after_write`. (d)
Warnings reach the UI as `mirrorWarnings: string[]` on `CreateCharacterOutcome` and are shown on the sheet
("Campaign copy failed: <dir>: <reason>"). (e) **UI** (E3.5): the Campaign label becomes a button opening
`CampaignMembershipDialog` *(new)* listing the Drive folder's campaigns (`drive_list_campaigns`) with a
checkbox each (checked = active); under the Skills block, one button per active campaign, "Open <name>",
which sets the hub's mode to the campaign sheet for that id (`CharacterHubPage` gains a `campaign-sheet`
entry with the id; `CampaignManagerScreen.onLoad` already names it). (f) `clone_character` copies no
membership (a clone is a new character). **Enforced by:** E1.10 tests (two alike names, two ids → two
folders; byte-identical copy of every file; an unwritable dir → warning + local files intact; inactive
membership → no write); E2.7's scanning test and an end-to-end test (save custom → the campaign copy's
`custom.json` changes); E3.5 model tests; smoke rows `sheet-campaign-add-from-identity`,
`sheet-campaign-button-under-skills`, `sheet-campaign-remove-keeps-folder` (E3.3). **Overlap:** the
root-crate mirror (E1.10) touches no SD-40 file → Phase 1; the hooks and the UI touch `character_hub.rs`,
`CharacterSheet.tsx`, `CharacterHubPage.tsx` → Phase 2.

## §30 — Operator answers to §21 Q6–Q8 (2026-11-12, verbatim as relayed by the lead)

- **Q7 = "Every Market item":** SD-41 does not close until every item the Market shows has a real price
  (converter fix, else d20pfsrd with URL); `price-unknown.json` must be **empty for Market-visible items at
  closure** (it may hold non-Market records only); Negotiate is for haggling, not a missing-price stand-in.
  → A **closure criterion** (G-8, G-10): the gate test `market_prices_are_positive_or_listed` becomes
  `market_prices_are_all_positive` at closure (no Market row may be in `price-unknown.json`). The lookup work
  is sized from E1.8's count and runs as card **E1.11** in batches (R-B batch big: one pass per batch of
  ≤ 150 rows; the receipt per batch names the rows and URLs). **How the pages are fetched:** d20pfsrd answered
  `410 Gone` to a bare `curl -sI` on 2026-11-12; the plan is the lead's `web-fetch` agent per batch (E1.11's
  dispatch names it), with a browser-user-agent `curl` as the first try inside the card (P-9); the card
  writes `d20pfsrd-prices.json` from what the agent returns, one URL per item. Until E1.11 completes, a
  Market row without a price is **not buyable** (the Buy button is disabled with "no price yet"); Negotiate
  never substitutes for a missing price (`§26` (d) amended).
- **Q6 = "Offer them, price by Negotiate"** (the default stands): a Buy Custom eqmod whose formula the
  converter cannot evaluate is offered, and the purchase is priced by Negotiate (the quote says which eqmod
  is unpriced).
- **Q8 = "Count single arrows":** bundles become single pieces once bought (the Market still sells by the
  book's bundle: buying 3 × "Arrows (20)" adds 60 arrows); stacks split to any count; ammo boxes count real
  pieces. **This changes the saved shape** — the lead asked for a quantity on the stored copy (or equivalent),
  a migration for existing bundle copies (1 copy of "Arrows (20)" → 20 pieces) with weight and price
  unchanged, and an old-save test. **DEFAULT (the equivalent that keeps NR-3):** the per-copy `quantity`
  (pieces) lives in `equipment_sets.json`'s `copies` map beside the custom name
  (`copies: { "<itemId>|<bonus>|<ordinal>": { name?, quantity? } }`); the saved input keeps one `equipment=`
  line per **stack** (a line is now "a stack of n pieces", n = 1 when absent); `EquipmentSelection` is
  untouched (97 producers). The engine learns pieces through E1.7's extra argument: `compute_encumbrance(…,
  quantities: &[(CopyKey, u32)])` weighs a stack as `record.weight × quantity / base_quantity`; a copy with no
  `quantity` entry weighs **one bundle** (= `base_quantity` pieces) — so an old save's "Arrows (20)" line
  weighs exactly what it weighs today, which is the migration: E2.1 writes `quantity = base_quantity` for
  every bundle copy without one and the old-save test asserts `total_carried_weight_lbs` unchanged. Buying
  3 × "Arrows (20)" appends one line with `quantity 60` (or adds 60 to an existing Carried stack of the same
  item, bonus and name); a move of n pieces lowers one stack and raises or creates another at the target;
  Sell / Discard of n pieces price `n / base_quantity × bundle price` floored to a copper; the row reads
  `"Arrows ×60"` (pieces; the bundle name's `(20)` is dropped from the label). Ammunition boxes count Σ
  `quantity` over matching ammunition stacks (`§23` amended: no `× base_quantity`). `§28` (a), (c), (d),
  (e) and `§21` Q8 are amended by this section; DEF-6 is withdrawn.

### §30.1 Correction (the lead, 2026-11-12): a price-unknown row stays listed and buys through Negotiate

Two operator answers meet here. The Negotiate item (§26): > "allow the user to specify the price on purchase".
Q7 (§30): > "Every Market item" — Negotiate "is for haggling, not a missing-price stand-in". **Ruling:** hiding a
row (`market: false`) contradicts the first; Q7 is the **effort bar**, not a filter. So: a Market row with no
price from any source **stays listed**, shows "no price yet", and **Buy routes through Negotiate** (the user
types the price). E1.11 must try every unpriced Market row (converter fix, then d20pfsrd); `price-unknown.json`
may name a Market key **only with a per-row reason showing every source was tried** (`{ key, reason, tried:
["converter", "d20pfsrd:<url>"] }`); G-8 and G-10 check that; the operator sees the final price-unknown count at
E4.G. The `market: false` filter is dropped (E1.11 touches no desktop file); E2.5's test is
`market_backend_unpriced_item_buys_through_negotiate`. The gate at closure is `market_prices_are_positive_or_listed`
with the listed exceptions each carrying `tried` (the stricter `market_prices_are_all_positive` is not a gate).
§26 (d) as first written (a price-unknown item's Buy routes through Negotiate) stands; §30's "not buyable" and
"leaves the Market listing" sentences are withdrawn.

## §31 — E1.1 blocker cleared (2026-11-12): containers are settled by the converter; weapons are not containers

E1.1 returned `blocked-escalated` (commit `370e3346b4`, `artifacts/epic_1/E1.1_cycle_receipt.md`): the live
`CorpusEquipmentRecord` is read from `data/corpus/<book>/_settled/equipment.json` (30 bundles, written by
`codex-ingest`'s `gen_settled_corpus` through `ir_converter::equipment_record_to_corpus`), and that record carries
**no tags and no tokens**; reading `raw_tokens` in live code is a `pcgen-residue-gate` hit (§17). Four §20
corrections were recorded. The lead's rulings:

- **(R1) LEAD GRANT.** E1.1's FILES YOU OWN widens to `crates/codex-ingest/src/pcgen_import/ir_converter.rs`
  (`equipment_record_to_corpus` + its new `*_of` helpers) and the regenerated `data/corpus/*/_settled/equipment.json`
  (from the repo root: `cargo run --locked -p codex-ingest --bin gen_settled_corpus`; §20). The desktop fixture
  `apps/desktop/src-tauri/resources/corpus_fixtures/_settled/equipment.json` **stays byte-identical**: the new
  fields are omitted at their defaults and settled only for non-weapons (the receipt's proposal (b)); E1.1 checks
  `sha256sum` before and after. **Sequencing:** E1.9 now depends on E1.1 too (both edit `ir_converter.rs`); E1.8
  (census, no converter edit) may still run beside E1.1.
- **(R2) OPERATOR RULING, 2026-11-12.** Question: SD-a's premise is false — Pistol and Powder Horn both carry the
  `Container` tag (sheet rules `ultimate_equipment/equipment/pistol.json`, `powder_horn.json`); what makes a copy a
  container? Answer, verbatim: **"Container tag, but not weapons."** → `is_container` = `TYPE` holds `Container`
  **and** the record is not a weapon. "Not a weapon" is **the record states no base damage** (`DAMAGE` absent in the
  source; `states_base_damage == false` on the settled record): the corpus states `DAMAGE` on every weapon that
  attacks (7,804 files; 78 of the 209 `Container`-typed corpus records carry `DAMAGE`, receipt figures), while the
  `Weapon` TYPE word also sits on non-attacking gear (e.g. `Rock`, thrown improvised) and is absent on some
  attacking exotics in other books — damage is what the engine already keys weapon rows on (`damage_total.rs`), so
  it is the field the corpus states reliably and the one the engine trusts. Pistol (`DAMAGE 1d8`) → not a container;
  Powder Horn (no `DAMAGE`, `CONTAINS UNLIM|BlackPowder=10`) → a container, `Unlimited`. SD-a is replaced (§12.1).
- **(R3) Figures corrected everywhere** (`census.py:41` globbed `equipment/*.json` and skipped `equipment/<subdir>/`,
  3,805 of 7,804 files): `CONTAINS` records **210** of 7,804 — **88 lb / 73 `UNLIM` / 49 `*`**; `Container` tag in
  sheet rules **232 files / 265 rows** (123 rows also `Weapon`); base Backpack (`general/backpack.json`) reads
  `UNLIM|Any` → **`Unlimited`**, not `Pounds(60.0)`; weapon records with a `CONTAINS` family 79 (unchanged).
  `census.py` fixed and re-run; F-14, F-15, §7, E1.1, G-5, the design and the scope carry the new figures.
- **(R4) Package defect fixed.** E1.9's price fix must also regenerate `_settled/equipment.json` (where the live
  app reads `cost_gp`); its diff guard and the §2.1 prefix line allow `_settled/equipment.json`; E1.8's census and
  E1.11's gate read the Market through `build_equipment_catalog()`, which reads the settled bundles, so they see
  the regenerated prices.
- **Settled-fields principle (derived from R1):** every corpus fact a Phase-1 card needs on the live record —
  `container`, `container_capacity` (E1.1), `uses_ammunition` (`Projectile`) and `ammunition_family` (`CONTAINS`
  family; E1.6), `base_quantity` (`BASEQTY`; E1.7), the unarmed record's size step (E1.5) — is **settled by
  `ir_converter.rs` into `_settled/equipment.json`** and read from there; no live code reads tags or `raw_tokens`.
  E1.6 and E1.7 therefore own `ir_converter.rs` and the regenerated bundles for their fields too (serial in the
  main lane after E1.1; the same fixture-byte-identical check), and the sheet-rule `tags` are not a live read path.

E1.1's kanban row is `ready` again; the `## Open blockers` entry is closed with this section.

## §32 — The Backpack holds 60 lb; an override switch (OPERATOR, 2026-11-12)

> "Make the backpack have a 60 lbs limit, but also add an override switch in the custom settings menu as an option."

**The data and the books (checked 2026-11-12 on the pinned oracle `7f818006e371188e5717fd18d74d18a420747fc6`):**
PCGen's CRB row `Backpack` carries `CONTAINS:UNLIM|Any` and `SOURCEPAGE:p.158`
(`$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_general.lst`); Ultimate Equipment's
`Backpack.MOD` (`SOURCEPAGE:p.58`) renames it "Backpack, Common" and changes no capacity; the APG/UE
`Backpack (Masterwork)` is `UNLIM|Any` too. The converted CRB record (`data/corpus/core_rulebook/equipment/general/backpack.json`)
has no description text; no backpack record in `data/corpus` or `data/sheet_rules` mentions "60"
(`grep -rl -i backpack data/corpus --include=*.json | grep /equipment/ | xargs grep -l '60 pounds\|60 lb' → none`;
the only equipment text with a pound limit is the Handy Haversack). **No PF1e book in the pinned data prints a
60 lb limit for the Backpack**; the PCGen data says unlimited. The 60 lb figure is therefore recorded as the
**operator's ruling**, not as a book citation (flagged to the lead in the reply).

**DEFAULTS:** (a) **A cited capacity override list, like the price fix** (`§26`):
`data/capacity_overrides/container-capacities.json` *(new)* = `[{ key, capacity_lbs, source: { kind:
"operator-ruling" | "book", ref, date } }]`; one row: `{ "key": "Backpack", "capacity_lbs": 60, "source": { "kind":
"operator-ruling", "ref": "decisions.md §32; overrides PCGen CRB p.158 CONTAINS:UNLIM|Any", "date": "2026-11-12" } }`.
`ir_converter::container_capacity_of` applies the list after the token (an override row wins); the settled
`container_capacity` for Backpack reads `Pounds(60.0)`; live code reads only the settled field and never names
"backpack" (`grep -rn -i backpack src/ apps/desktop/src-tauri/src/ | grep -v test` → 0). Card **E1.12** (opus,
Phase 1, `convert` scope, after E1.1 and E1.9: all three edit `ir_converter.rs` and regenerate `_settled`).
Backpack (Masterwork) is not overridden (the operator named the backpack; a second row is a later ruling).
(b) **The override switch is a per-character toggle, "Ignore container capacity limits",** in the Custom tab's
existing Overrides group (SD-39: `Overrides` in `character_custom.rs:120–128`; `OVERRIDE_TOGGLES` in
`customModel.ts:90`), not the app-wide ☰ Options; it persists in `custom.json` with the other overrides
(`ignore_container_capacity: bool`, serde default false). When on: the gear screen and the print still show
"x of y lb" but no container is flagged overweight (`containerLoads[].over` stays the engine's figure; the
screen and print suppress the warning when the override is on), and the sheet tags the override visibly as
the other Overrides do (`OverrideTag`, the `admittedUnderOverride` posture). Owned by E2.3 (the field, the
DTO, the Custom tab toggle: it already edits `character_custom.rs`, `CustomDialog/CustomTab`, `customModel.ts`),
E3.1 (the gear screen's suppression + tag) and E4.1 (the print's). Smoke row `sheet-gear-capacity-override`
(E3.3, 19 new). **Enforced by:** E1.12's RED test `container_capacity_backpack_is_60_lb` on the settled bundle;
E2.3 `custom_overrides_ignore_container_capacity_round_trips`; E3.1 model test "over suppressed when the
override is on"; E4.1 `print_pf_blocks_gear_capacity_override_hides_the_flag`.

## §33 — E1.4 re-scoped as an additive engine entry point (LEAD, 2026-11-13)

E1.4 declined with no code written (`862ca3fe86`, `artifacts/epic_1/E1.4_cycle_receipt.md`): widening
`resolve_weapon_damage_breakdown` so a `SelectedInactive` weapon gets a row turns a desktop test red
(`character_hub.rs:10720–10725`, "a stowed sword has no damage row"), and the desktop calls the function directly
(`character_hub.rs:2879`, `rule_system_adapter.rs:435`); Phase 1 may not write under `apps/`. The lead's ruling,
verbatim: **"use option (c). Do not use (a) or (b)."** — *"(c) makes the change additive. E1.4 adds a new engine
entry point for the carried view (for example `resolve_weapon_damage_breakdown_with_carried`, or a parameter on a
new function). The existing `resolve_weapon_damage_breakdown` keeps its exact behaviour. No desktop caller and no
existing test changes. That covers `tests/worn_vs_carried.rs:122-123,155`, `src/rules_core/corpus_loader.rs:634-636`
and the desktop test. E2.4 switches the desktop callers to the new entry point, marks carried weapons, and
updates the desktop test there. E4.1 does the same for the print."*

**Why:** (1) the Phase-1 `apps/` fence stays intact — no exception, no desktop test edit before SD-40 merges;
(2) **no unmarked carried weapon ever appears on `tranche-21`**: the old function keeps returning in-hand rows
only, so the Weapons tab and the PF print are unchanged until E2.4 / E4.1 switch to the new entry point *and* mark
the carried rows in the same card; (3) the new function is consumed in Phase 2 by named callers, so it is not an
unwired path (the wired-integration doctrine's concern in the receipt) — it is the entry point E2.4 and E4.1 are
scoped to call. The `in_hand` field lands on `WeaponDamageBreakdown` in E1.4 (default `true` for every row the
old function returns, so its callers compile and behave unchanged). E1.5's unarmed row and E1.6's ammunition
flags build on the same struct. E1.4 is `ready` again; the `## Open blockers` entry is closed with this section.

## §34 — Print page 1: title boxes, swap, skills table, weapons cleanup, speed 2×2, portrait or logo (OPERATOR, 2026-11-13)

> "On the print. You have the following in place Speed, Hit Points, Initiatives, Ecumberance, Abilitys, Skills,
> Weapons, Attacks, Saving Trhows, Conditional Bonuses, and Resistances all of their titles, when an underline. I
> would prefer to have that be a colored box with white letters - similar to the pcgen sheets. I also want to swap
> locations between Conditional Bonuses and Resistances. I would also like to remove the lines inside of conditional
> bonuses - just leave it an empty box. For Skills, let's make that more of a table that has alternate rows have a
> highlight. Weapons also has blanks line and boxes - we don't need that. Speed also neeeds to include climb - You
> probably have enough room to do it in 2x2 instead of 1x4. Instead of hte word portrait, we need to either print a
> portrait or we should put in a logo. We should grab the logo for the game system involved."

**Lead rulings (2026-11-13):** all are print page-1 changes under `apps/desktop/src-tauri/src/print/` →
**Phase 2**, after SD-40 (PR #406) merges and `tranche-21` is rebased; in E4, serial after E4.1 (the SD-41 print
card that already touches page 1), so two cards never edit one block file at once. (1) **Section titles:** a
filled box with white bold letters, like the PCGen sheets; default colour near-black (`#222` or the sheet's
existing ink token); **one shared title style** in the frame for all 11 sections (Speed, Hit Points, Initiative,
Encumbrance, Abilities, Skills, Weapons, Attack, Saving Throws, Conditional Bonuses, Resistances), not one per
block; checked at A4 and Letter. (2) **Swap** Conditional Bonuses and Resistances on page 1. (3) **Conditional
Bonuses:** an empty bordered box, no ruled lines inside; the printed bonuses it already shows stay; only the
blank ruled lines go. (4) **Skills:** a table with alternate rows shaded light grey that still reads on a
black-and-white printer. (5) **Weapons:** no blank lines and no empty boxes; only real weapon rows (with SD-41's
rule: Unarmed last, ammunition boxes only under ammunition weapons). (6) **Speed:** add Climb; a 2×2 grid
(Land, Fly, Swim, Climb) instead of 1×4; Climb from the engine's climb speed; if the engine has none, print "—",
never a made-up value (paper-sheet rule). (7) **Portrait box:** never print the word "portrait"; print the
character's portrait image when there is one, else the game system's logo. **HOLD the logo source:** the repo
is public and system logos are trademarks; the operator rules which file to use. The card makes the logo a
bundled asset per system (one path each: PF1e, SF1e, CPR); the files are a named pending item the operator
supplies — listed under `progress.md ## Open blockers` as a ruling request. (8) Goldens: each card updates the
PF print goldens and names its test; E4.G gains a gate line for page 1 so the operator checks these in the
round-1 print.

**Census (2026-11-13, `tranche-21` @ `79b08e4760`):** a portrait store **exists** — `save_character_portrait`
(`character_hub.rs:5744`), `character_portrait_path_at_root` (`:5811`), `load_character_portrait_bytes_at_root`
(`:5817`), read by `print/mod.rs` into the `portrait` block; `looks/portrait.typ` (SD-40 E2.5) frames the image
when the block carries one, else prints the word. Looks today: `abilities, armor-class, attack, conditional-bonuses,
languages, name-campaign, portrait, resistances, saves, skills, speed, square-block, table, weapons` (14 files,
`ls apps/desktop/src-tauri/resources/print/looks/`). Speed: `core.rs:269` builds Land / Swim / Fly from the
`speed.land` / `speed.swim` / `speed.fly` rows; **no `speed.climb` row exists** (`grep -rn '"speed.climb"'
src/rules_core` → 0; the Vanara climb speed is a constant, `class_shared_core.rs:2733`, not a row). No logo
file exists in the repo (`find apps/desktop -iname '*logo*'` → none; `resources/` holds `authoring_workbench
corpus_bundle corpus_fixtures fonts print`).

**DEFAULTS:** three cards, E4.2 → E4.3 → E4.4, each serial after E4.1. **E4.2** titles + swap + empty box.
**E4.3** skills table + weapons cleanup + speed 2×2 (Climb reads a `speed.climb` row when one exists, else "—";
adding the row is `§21` Q11, default not in SD-41). **E4.4** portrait or logo: `looks/portrait.typ` prints the
image, else `resources/print/logos/<system>.png` (`pf1e.png`, `sf1e.png`, `cpr.png`; `packaged_resources.rs`
asserts them), never a word; until the operator supplies the files the card ships **placeholder-free**: the
three paths exist as committed files the operator replaces (a plain framed system name typeset in the box is
**not** acceptable to the operator's words — so E4.4 is `blocked-escalated` on the files alone if they have not
arrived by its dispatch; everything else in the card lands). `§21` Q10 records the ruling request. The title
style is a `frame.typ` function every look calls (one place; NR-2 unchanged: strings only). Shading uses a
grey ≥ `luma(230)` (SD-40 SD-d's range) so it prints on a monochrome printer.
