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
  prints nothing). Cargo lanes on the box stay ≤ 3 including SD-40's (`pgrep -f '[c]argo'`; R-Q2).
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
  232 records). E1.1 adds `is_container()` to the record the engine already reads for weight
  (`CorpusEquipmentRecord`, `equipment_record.rs`, 224 lines) or, if that read path has no tags, to the
  sheet-rule lookup the catalog uses; E1.1 names the path it chose in its receipt.
- **Capacity:** the `CONTAINS` token's first field in the corpus record's `raw_tokens` (138 records:
  `<lb>|…` 70, `UNLIM|…` 43, `*<n>|…` weight-reduction kinds 25). `ContainerCapacity = Pounds(f64) |
  Unlimited | Unstated` (the `*` kinds are `Unstated`: contents weigh in full, DEF-1). `data/sheet_rules`
  holds no `CONTAINS` (`grep -rl CONTAINS data/sheet_rules … | awk` → 1 file, not an equipment record), so
  capacity is read from the corpus record, the same bytes weight comes from.
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
  counted: 37 saved of 65 files);
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
| SD-a | A copy whose record has no `Container` tag but a `CONTAINS` token (ammunition holders: Pistol, Powder Horn) | not a container on the screen (the tag rules); the token is ignored; logged once per card |
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
Unarmed last. **Enforced by:** E1.5 tests (Aldric Fighter 3 → `1d3+3` nonlethal; a Monk 4 → `1d8` lethal;
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
