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

# SD-41 Forward Scope Register

Classes: **Committed** (in the DoD; a shortfall is a blocker), **Structurally implied** (falls out of
committed work), **Candidate** (not in the DoD; a revisit condition is *checked* at E5.2). Only the
operator moves scope between classes. The predecessor is SD-40 (`../SD-40-pf-sheet-design/`), live at
authoring; its Inventory section (E3.2) is what E4.1 re-lays by location.

## 1. Committed

| ID | Work | Source | SD-41 cards |
|---|---|---|---|
| COM-1 | Locations: 17 slots + Equipped + Carried + Not Carried + inside a container; stored per set in a sidecar; projected into the engine's `active_state` | request; A1, A2 | E1.2, E2.1, E2.2 |
| COM-2 | AC from slots only; weight from carried only; a Not Carried container's contents weigh nothing | request | E1.2 (projection), E2.2 tests |
| COM-3 | Containers: tag + capacity read; recursive loads; warning never block; nesting; cycle refused | request; A3 | E1.1, E1.3, E2.1, E2.2, E3.1 |
| COM-4 | Equipment sets: one item list, per-set locations, one active set; create / rename / delete / activate | request; A2 | E1.2, E2.2, E3.1 |
| COM-5 | Old saves → `Default` set; AC and weight unchanged (tested over the population) | A6 | E1.2, E2.1, E2.MC |
| COM-6 | Market purchases default to Carried | request; A8 | E2.2, E3.3 |
| COM-7 | Weapons: in hand first, then carried; Not Carried off (tab + print) | A5 | E1.4, E2.4, E4.1 |
| COM-8 | Custom items as containers through a base item | request; A3 | E2.3 |
| COM-9 | The gear screen: split panel, filter, locations tree, Move to menu, drag, set picker, capacity shown | request; A1, A4 | E3.1, E3.2 |
| COM-10 | ui-smoke rows for the screen; the replaced rows rewritten | R17, R24 | E3.3 |
| COM-11 | Print: Inventory by location with nested containers; totals = encumbrance | A7 | E4.1 |
| COM-12 | Operator gate on the screen and the printed page; closure in the standing order | §12, §18 | E4.G, E5.1–E5.4 |
| COM-13 | The Unarmed row, always last, with real numbers (monk progression, IUS lethal) | `decisions.md §22` | E1.5, E2.4, E4.1 |
| COM-14 | Ammunition boxes only under ammunition weapons, with the carried count; no global row | `decisions.md §23` | E1.6, E2.4, E4.1 |
| COM-15 | The Market table (Item / Price / Weight / Slot, sortable), description box, Only Show Affordable, Quantity, Negotiate on Buy and Sell, Buy Custom builder, Create dialog | `decisions.md §24–§26, §28` | E2.5, E2.6, E3.4 |
| COM-16 | Created items: weight, optional price, optional container + slot; persisted, printed, weighed, editable in the Custom tab | `decisions.md §25` | E1.7, E2.3 |
| COM-17 | Prices: converter fix, cited d20pfsrd list, price-unknown list, the gate test | `decisions.md §26` | E1.8, E1.9 |
| COM-18 | Drop → Not Carried; Sell and Discard separate; stacks with quantities on every action (was FSR-C1) | `decisions.md §27, §28` | E2.1, E2.2, E2.5, E3.1 |

## 2. Structurally implied

| ID | Work | Falls out of |
|---|---|---|
| IMP-1 | `equipment_sets.json` is a new per-character file the app writes; `docs/architecture/persistence.md` names it (E5.3) | COM-1 |
| IMP-2 | `HeldEquipmentCopyDto` gains `location` + `ordinal`; `EquipmentPanel.tsx` is deleted; `set_equipment_active_state` becomes a move | COM-1, COM-9 |
| IMP-3 | `bought_copy_placement`'s wear branch is removed with its tests | COM-6 |
| IMP-4 | `WeaponDamageBreakdown.in_hand` / `WeaponDamageDto.inHand`; G-1 `NEW_KEYS` grows by five | COM-7 |
| IMP-5 | `CustomRecordDto.base_item_id` (serde default) | COM-8 |
| IMP-6 | `tranche-21` catches up to `develop` once SD-40's PR merges (SD-m) | `decisions.md §4` |
| IMP-7 | Architecture docs (`desktop-app.md`, `persistence.md`, `rules-engine.md`, `testing.md`, `glossary.md`) gain locations, sets, containers | E5.3 |

## 3. Candidates (not in the DoD)

| ID | Work | Source | Why not in SD-41's DoD | Revisit condition (checked at E5.2) |
|---|---|---|---|---|
| FSR-C1 | Quantity stacks (one row "Arrows ×20") | PCGen's Purchase tab buys n at a time | **No longer a candidate: committed as COM-18 by the operator's bulk-buying item (2026-11-12, `decisions.md §28`)** | none: E5.2 checks E2.5 is `complete` |
| FSR-C2 | Wield styles per weapon (one-handed / two-handed / off-hand) beyond the three wield slots | PCGen equips weapons by style | the app's `wield_main` / `wield_off` / `wield_two_handed` slots carry the choice today | the operator asks |
| FSR-C3 | Moving an item to another character | PCGen's "Move To / Copy To" | not requested | the operator asks |
| FSR-C4 | Starfinder and Cyberpunk RED gear screens | A1 names the Pathfinder list | their markets are separate shells | a scope draft after SD-41's launch names them: `git log --since=2026-11-12 --diff-filter=AM --name-only --format= -- 'docs/release/SD-*/scope-draft.md' \| sort -u \| grep -v SD-41 \| xargs -r grep -lis 'starfinder.*gear\|cyberpunk.*gear' \| awk 'END{print NR}'` ≥ 1 |
| FSR-C5 | Charges on wands and the like | PCGen's Modify Charges | not requested | the operator asks |

## 4. Planned capability deferrals

The test (`docs/governance/deferral-revisit-doctrine.md`): was this in the DoD when scoped? No, by the
author's default or the operator's silence, named here.

| ID | Capability | Ruling | Revisit condition (a command, checked at E5.2) | Accepted cost |
|---|---|---|---|---|
| DEF-1 | Weight-reduction containers (`*N` capacity kinds, 25 records: bags of holding, efficient quivers) | `decisions.md §7`, Q4 default | `python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_0/census.py \| grep 'reduction_or_other'` → the count; the operator asks | their contents weigh in full; the screen says "no stated capacity" |
| DEF-2 | Drag in ui-smoke | the harness has no drag op (`census.txt §5`) | `python3 -c "import json;print('drag' in {s['op'] for r in json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'] for s in r['steps']})"` → `True` once a drag op exists | drag is proven by a unit test and the operator's eye |
| DEF-3 | Per-set print (print the Formalwear set without activating it) | A2: one active set drives the print | the operator asks | switch the set, then print |
| DEF-4 | A set's own AC / weight preview before activating | A2 | the operator asks | activate to see |
| DEF-5 | Eqmod price formulas the converter does not evaluate | `decisions.md §24`, Q6 | E1.9's receipt lists the count; the operator asks | those abilities sell through Negotiate |
| DEF-6 | Splitting a bundle below the book's unit | `decisions.md §28`, Q8 | the operator asks | a stack splits by bundles |

If the operator says any of these was meant to be in the DoD, it becomes a blocker.

## 5. No deferrals for SD-41 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is `blocked-escalated`.

## 6. Excluded from this scope (the operator passed on them or the request did not name them)

PCGen's code or data model (A-request: "do not copy the code"); a converter change beyond E1.9's price
derivation; an engine change beyond E1.1–E1.7's named functions and fields (`decisions.md §16`); a new app
tab (Q5 default).
