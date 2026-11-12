---
title: SD-41 — Gear and Equipment Sets — Release Package
status: planning-ready — authored 2026-11-12 from the operator's request and nine answers; the lead re-reads against the repo before launch (decisions.md §20)
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
> root crate) may run now beside SD-40; everything from E2 on waits for SD-40 to merge
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
| Epics / cards | C, E1–E5 / **23 cards**, one criterion each (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 23) |
| Target version | `0.21.0`, stamped by C1's single commit on the 14 surfaces + `current-release.json` (`decisions.md §2`) |
| Dispatch mechanism | `Workflow` tool, one long run per phase (`decisions.md §12.2`); ends cleanly at E4.G |
| Cadence | N/A |
| Closure gate | 23 cards `complete` (E4.G needs the operator's two `usable: yes` lines) → retrospective + cite → sweep → release notes → architecture truth-up + critic → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Source | the operator's request and answers (`artifacts/cycle_0/operator-brief-2026-11-12.md`); the census (`artifacts/epic_0/census.txt`); PCGen's Equipping tab as a functional reference only (`decisions.md §17`) |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | The request in the operator's words, the code's state today, the Definition of Done, epic order |
| `decisions.md` | §1–§21: the operator's nine answers verbatim (§3), the location model and the sidecar file (§5), projection into today's engine inputs (§6), containers and capacity (§7), the Default-set migration (§8), weapons order (§9), the market default (§10), custom containers (§11), unattended protocol with safe defaults SD-a…SD-n (§12), the SD-40 partition (§4), PCGen rule (§17), the operator questions with defaults (§21) |
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
| 4 | What makes an item a container; where capacity comes from | the `Container` tag (232 sheet-rule records); the `CONTAINS` token's weight in the corpus record (70 of 138; 43 unlimited; 25 weight-reduction kinds show "no stated capacity") | `decisions.md §7`; `§21` Q4 |
| 5 | A carried (not in hand) weapon | prints and shows its attack rows after the in-hand weapons, marked carried; Not Carried weapons are off | `decisions.md §9`; `§21` Q2 |
| 6 | A custom container | a custom item names an optional base item; it is a container and weighs what its base weighs only when a base is named; old custom items are unchanged (weightless, not containers) | `decisions.md §11`; `§21` Q3 |
| 7 | Where the gear screen lives | it replaces the Equipment panel inside the Gear tab; `SHEET_TABS` unchanged | `decisions.md §13`; `§21` Q5 |
| 8 | Drag and drop in ui-smoke | the harness has no drag op (`census.txt §5`); every smoke row moves through the "Move to" menu; drag is proven by a unit test of the drop handler and the operator's eye at E4.G | `decisions.md §14` |
| 9 | Phasing against SD-40 | E1 (root crate + new files) now; E2–E5 after SD-40 merges and `tranche-21` catches up to `develop` | `decisions.md §4` |

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
