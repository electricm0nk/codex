---
title: SD-41 — Gear and Equipment Sets — Release Package
status: planning-ready — authored 2026-11-12 from the operator's request, nine answers and eight follow-up items of the same day (decisions.md §3, §22–§29); the lead re-reads against the repo before launch (decisions.md §20)
bundle_id: SD-41
slug: gear-and-equipment-sets
scope: docs/release/SD-41-gear-and-equipment-sets
artifact_type: release-index
canonical_branch: tranche-21
kanban_board: local-file ./kanban.md
target_version: 0.21.0
canonical_source: docs/release/SD-41-gear-and-equipment-sets (this folder)
date: 2026-11-12
---

# SD-41 — Gear and Equipment Sets — Release Package

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as one long `Workflow`-tool script invoked from a live session. It is NOT run
> with `/loop /batch` and it is NOT a one-shot task.** `workflow-instruction.md §2.4` says how to write
> the script. **🟡 UNATTENDED MODE** after launch (`decisions.md §12`): no questions mid-run, safe defaults
> logged, a report after each epic. **One card is an operator gate:** E4.G (the gear screen on screen and
> the printed inventory page); the run waits there and nowhere else. **Two phases:** Epic E1 (the engine,
> root crate, and the tool-side `convert` lane) may run now beside SD-40; everything from E2 on waits for SD-40 to merge
> (`epic-breakdown.md §0`, `decisions.md §4`). `scope-draft.md` is the *what*; `workflow-instruction.md`
> is the *how*.

This folder is the canonical surface for SD-41. A harness needs only this folder and the in-repo doc tree.

## 0. Preamble

Today the Gear tab shows three flat lists (Worn by slot, Carried, Left behind) with one button per move;
buying tries to wear the item; containers do not exist; one loadout is the only loadout. The operator
asked (2026-11-12) for the gear screen PCGen's Equipping tab has, in function and never in code: a split
panel (every item with its weight and location on the left, filterable; every location on the right, from
the Pathfinder slots down to Carried and Not Carried), moves by drag and by a menu, containers that hold
items and nest, custom-named containers, and named **equipment sets** ("formalwear") that each place the
same items differently. AC comes only from what sits in a slot; weight only from what is carried; the
active set drives the sheet, the screen and the print. Old saves load into a `Default` set with AC and
weight unchanged.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-41 |
| Slug | `gear-and-equipment-sets` |
| Canonical branch | `tranche-21`, cut 2026-11-12 from `origin/tranche-20` @ `f7f85f3d24` (`git merge-base HEAD origin/tranche-20`); catches up to `tranche-20`'s tip and then to `develop` after SD-40 merges (`decisions.md §4`) |
| Kanban board | local file `./kanban.md` |
| Epics / cards | C, E1–E5 / **46 cards** (E2.11 discovered by E1.11; E1.15 and E2.10 discovered by E1.9; E1.14 from the Q11 answer, `decisions.md §35`; E4.2–E4.4 from the page-1 print request, `decisions.md §34`; E1.13 and E2.9 discovered by E1.6; E1.12 from the Backpack ruling, `decisions.md §32`; 23 + 8 from the seven follow-ups + 3 from the campaign item + E1.11 from the Q7 answer, `decisions.md §22–§30`, + E2.8 discovered by E1.8), one criterion each (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 36) |
| Target version | `0.21.0`, stamped by C1's single commit on the 14 surfaces + `current-release.json` (`decisions.md §2`) |
| Dispatch mechanism | `Workflow` tool, one long run per phase (`decisions.md §12.2`); ends cleanly at E4.G |
| Cadence | N/A |
| Closure gate | 46 cards `complete` (E4.G needs the operator's three `usable: yes` lines; every unpriced Market row tried, `decisions.md §30.1`) (E4.G needs the operator's two `usable: yes` lines) → retrospective + cite → sweep → release notes → architecture truth-up + critic → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Source | the operator's request, answers and eight follow-ups (`artifacts/cycle_0/operator-brief-2026-11-12.md`); the census (`artifacts/epic_0/census.txt`); PCGen's Equipping, Purchase and custom-item tabs as a functional reference only (`decisions.md §17`) |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | The request in the operator's words, the code's state today, the Definition of Done, epic order |
| `decisions.md` | §1–§28: the operator's nine answers verbatim (§3), the seven follow-ups verbatim with their census and defaults (§22 Unarmed row, §23 ammunition per weapon, §24 Market table / sort / description / Only Show Affordable / Buy Custom, §25 Create item, §26 prices + Negotiate + columns, §27 Drop / Sell / Discard, §28 bulk quantity and stacks, §29 campaign folders: write-through, membership dialog, per-campaign buttons), the location model and the sidecar file (§5), projection into today's engine inputs (§6), containers and capacity (§7), the Default-set migration (§8), weapons order (§9), the market default (§10), custom containers (§11), unattended protocol with safe defaults SD-a…SD-n (§12), the SD-40 partition (§4), PCGen rule (§17), the operator questions with defaults (§21) |
| `forward-scope-register.md` | Committed, implied, candidate rows; DEF-1…DEF-4 |
| `technical-design.md` | Today's module map with line counts; the set model, the file, the projection, the container loads, the gear screen, the print page |
| `technical-requirements.md` | Prerequisites P-1…P-8, normative requirements NR-1…NR-14 |
| `epic-breakdown.md` | §0 parallel/serial map (leads) with the SD-40 partition, per-card criteria with FILES YOU OWN, acceptance commands, "does not cover" |
| `risks-and-open-questions.md` | Ranked risks with controls; open questions → defaults |
| `acceptance-and-verification.md` | Gates G-1…G-8 and the artifact map |
| `content-unit-inventory.md` | Figures F-1…F-24 with commands: slots, states, producers, containers, smoke rows, SD-40 overlap |
| `workflow-instruction.md` | The *how*: prefix, per-cycle procedure, receipt schema, closure order, rules R2–R25 |
| `progress.md` | Live log; Cycle 0 receipt; epic reports; open blockers |
| `kanban.md` | One row per card (23) |
| `release-notes.md` | Interim body (sentinel `SD-41 INTERIM NOTES`); replaced at E5.3 |
| `artifacts/`, `artifacts/README.md` | Per-epic receipts and evidence; `cycle_0/` holds the brief, the C0 receipt and the workflow prefix; `epic_0/` holds the census script and its output |
| `references/`, `references/README.md` | Doctrine, skills, sibling bundles, retrospectives, the PCGen functional reference |

`receipts.md` is created by the closure scripts (E5.3, E5.4).

## 3. In-repo cross-references

- Conduct: `../../../AGENTS.md`, `../../../CLAUDE.md`.
- Doctrine: `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/no-stub-mvp-doctrine.md`, `../../governance/wired-integration-stubs-registry.md`,
  `../../doctrine-external/identifier-discipline.md`.
- Architecture: `../../architecture/` (`desktop-app.md`, `persistence.md`, `rules-engine.md`, `testing.md`; truthed-up at E5.3).

## 4. Relationship to other release folders

- **SD-40** (`../SD-40-pf-sheet-design/`): predecessor, **live** on `tranche-20` in `.worktrees/sd40-main`
  (32 of 56 cards complete on 2026-11-12, `artifacts/epic_0/census.txt §7`). SD-40's open cards own
  `character_hub.rs`, `CharacterSheet.tsx`, `main.rs`, `spec.json`, `print/blocks/gear.rs`,
  `print/layout/pf1e.rs`, `print/{model,typst}.rs`, `blocks.typ`, `print_golden.json`,
  `packaged_resources.rs`, `loadSavedCharacterDetail.ts`. SD-41's E1 touches none of them; E2–E5 wait
  (`epic-breakdown.md §0`, `decisions.md §4`).
- **SD-39** (`../SD-39-ui-print-corrections/`): the print pipeline and rules R1–R20.
- **tranche-17-ui** (`../tranche-17-ui/`): `sheetLayout.ts`, the Gear tab's registry ids.

## 5. Build version target

`major` 0; `tranche-base` **21** (moves at C1's cut, never at closure); `build` = the GitHub run number,
stamped at publish by `publish-tester-release.yml` (`VERSION="0.21.${GITHUB_RUN_NUMBER}"` after C1).
Repo files read `0.21.0` after C1.

## 6. Author's calls not covered by an operator ruling (review before launch)

| # | Call | Default taken | Where |
|---|---|---|---|
| 1 | Where a set is stored | a sidecar `equipment_sets.json` beside `envelope.txt`; the saved input's `equipment=` lines are untouched; the active set is **projected** into `active_state` at every save, so the engine and its 97 `EquipmentSelection` producers change nothing | `decisions.md §5, §6` |
| 2 | An item a set does not name | is **Carried** in that set (so a purchase writes no set entry and is Carried everywhere) | `decisions.md §5.3, §10`; `§21` Q1 |
| 3 | Copy identity for a location | `{itemId, bonus, ordinal}` — the n-th copy of that item and bonus in saved order; custom items `custom:<id>` | `decisions.md §5.2` |
| 4 | What makes an item a container; where capacity comes from | **operator (§31 R2): "Container tag, but not weapons"** — Container-typed, no base damage and no `Weapon` type word (`decisions.md §20`); capacity from the `CONTAINS` token (210 records: 88 lb / 73 unlimited / 49 weight-reduction kinds → "no stated capacity"), both settled by the converter into `_settled/equipment.json` (lead grant R1) | `decisions.md §7, §31` |
| 5 | A carried (not in hand) weapon | prints and shows its attack rows after the in-hand weapons, marked carried; Not Carried weapons are off | `decisions.md §9`; `§21` Q2 |
| 6 | A custom container | **answered by the operator (§25):** a Create item may be a container with a capacity and no base; `base_item_id` is not added | `decisions.md §25`; `§21` Q3 |
| 7 | Where the gear screen lives | it replaces the Equipment panel inside the Gear tab; `SHEET_TABS` unchanged | `decisions.md §13`; `§21` Q5 |
| 8 | Drag and drop in ui-smoke | the harness has no drag op (`census.txt §5`); every smoke row moves through the "Move to" menu; drag is proven by a unit test of the drop handler and the operator's eye at E4.G | `decisions.md §14` |
| 9 | Phasing against SD-40 | E1 (root crate + new files + the `convert` lane) now; E2–E5 after SD-40 merges and `tranche-21` catches up to `develop` | `decisions.md §4` |
| 10 | Where a stack's piece count lives | **operator Q8 (§30): pieces** — `quantity` per stack in `equipment_sets.json`'s `copies` map (the saved input keeps one line per stack; `EquipmentSelection` untouched); a bundle copy without one is `base_quantity` pieces (the migration); the engine weighs `weight × quantity / base_quantity` | `decisions.md §28, §30` |
| 11 | Buy Custom pricing | **operator Q6 (§30): the default stands** — flat `COST` + evaluable formulas; an unpriced eqmod prices by Negotiate | `decisions.md §24, §30` |
| 12 | The d20pfsrd list's reach | **operator Q7 (§30) + the lead's reconciliation (§30.1): every Market item tried** — E1.11 in batches via the lead's `web-fetch` agent; a row still unpriced stays listed, says "no price yet", and buys through Negotiate; each such key in `price-unknown.json` names every source tried | `decisions.md §26, §30, §30.1` |
| 13 | A Create item's edit and removal | the Custom tab's existing dialog (it gains weight, price, capacity, slot) | `decisions.md §25` |
| 14 | Campaign membership truth and the write-through hook | the character's `campaigns.json` (the app is master); one root-crate `mirror` called by `SavedCharacterStore::save` and every sidecar writer, closed by a source-scanning test; a bad folder is a warning, never a failed save | `decisions.md §29`; `§21` Q9 |

## 7. Initial-package construction

Authored on 2026-11-12 by the planning session (Fable 5.1) in `docs/release/SD-41-gear-and-equipment-sets/`
on `tranche-21` in `.worktrees/sd41-main`, from the lead's brief carrying the operator's request and nine
answers (`artifacts/cycle_0/operator-brief-2026-11-12.md`). Every repo path was checked with `ls`/`grep -n`/
`sed -n` on `tranche-20` @ `f7f85f3d24`; the census (`artifacts/epic_0/census.py`, run with `python3 -I`)
produced every figure. PCGen's Equipping and Purchase tab documentation was read for function only
(`decisions.md §17`). The placeholder gate (`workflow-instruction.md §9`) was run before commit; its output
is in `progress.md` Cycle 0.

## 8. Cross-reference

- `../template/template.md`, `../../governance/workflow-instruction-template.md` — the two templates.
- `../../../.claude/skills/stc-authoring/SKILL.md` — the repo-local authoring skill.
