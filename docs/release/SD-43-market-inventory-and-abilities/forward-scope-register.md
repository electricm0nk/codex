---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — planned capability deferrals only; nothing in the Definition of Done is here
---

# SD-43 Forward Scope Register

**The rule.** This file holds only **planned capability deferrals**: scope that was **not** in SD-43's Definition
of Done (`scope-draft.md §3`) when the bundle was scoped, each with the three fields
`../../governance/deferral-revisit-doctrine.md` requires — the **condition** that would make it worth building,
**who or what checks it** (and when), and the **cost accepted** meanwhile. The successor is **SD-44, the operator's
feedback round on SD-43** (ruling 22b-1, 2026-11-22: *"SD-44 is a feedback round, not unfinished scope"*).

**What can never be here.** A card in `kanban.md`, or any part of a DoD row, that the run did not finish. That is a
blocker: cleared, or escalated to the operator and paused (`../../governance/blocker-closure-doctrine.md`). No
cycle may add a row to this file; only the operator moves scope into it. The test for any candidate row: *was this
scope in the Definition of Done at launch?* If yes, it is a blocker, not a row.

Every revisit command below is run once at E7.2's final-acceptance scan (output pasted in the E7.2 receipt) and
again by SD-44's scoping session when it inherits this register.

## 1. Planned capability deferrals

| ID | Capability | Why it is not in SD-43's DoD | Condition (when it is worth building) | Checker | Accepted cost meanwhile |
|---|---|---|---|---|---|
| DEF-1 | **Retraining** — swapping an earlier ability pick for a new one at a later level | operator's strawman (item 22, 2026-11-22) leaves it for the next SD; no later ruling brings it in; `scope-draft.md §5`; Q-22.5 | the operator asks for it in SD-44's feedback, or a successor scope names it: `ls docs/release/SD-4[4-9]-*/scope-draft.md 2>/dev/null \| xargs -r grep -li 'retrain' \| awk 'END{print NR}'` ≥ 1 | E7.2 (once); SD-44 scoping | a player who retrains removes the old pick and adds the new one by hand; the engine still enforces the budget and prerequisites on the result |
| DEF-2 | **Replace-a-pick** — an advanced/master trick taking the slot of a lower trick as a replacement mechanic | same strawman list; Q-22.5. **Not deferred:** advanced and master tricks are selectable as pool members with their prerequisites (in the DoD) | the operator asks, or `ls docs/release/SD-4[4-9]-*/scope-draft.md 2>/dev/null \| xargs -r grep -li 'replace-a-pick\|replaces an earlier' \| awk 'END{print NR}'` ≥ 1 | E7.2; SD-44 scoping | the replacing trick occupies its own pick; a GM who wants the swap removes the lower trick by hand |
| DEF-3 | **Abilities tab on the Starfinder and Cyberpunk RED sheets** | intake 22/22b describe PCGen's Pathfinder tab and pools; no ruling names SF or CPR; Q-22.6 | the operator asks, or an SF/CPR abilities census exists: `ls docs/release/SD-4[4-9]-*/scope-draft.md 2>/dev/null \| xargs -r grep -li 'abilities' \| xargs -r grep -li 'starfinder\|cyberpunk' \| awk 'END{print NR}'` ≥ 1 | E7.2; SD-44 scoping | SF and CPR sheets keep their current ability surfaces |
| DEF-4 | **PCGen Abilities-tab extras beyond the seven sub-tabs' core**: tree views (Name/Type/Source/Prerequisite), Multiples/Stacks/Cost columns, `EDITPOOL` / `FRACTIONALPOOL` (intake 22a rows 4, 5, 10) | the operator ruled 22a background, not frame (22b, 2026-11-22); `technical-design.md §3.3` | the operator names one in SD-44's feedback | E7.2; SD-44 scoping | each sub-tab is a flat searchable list with the Qualified filter |
| DEF-5 | **Layout polish after hands-on use** of every SD-43 screen (gear list spacing, sub-tab arrangement, button sizes beyond "twice today's") | the operator: *"I'm sure we will need to do adjustments in SD-44 once I get to put my hands on something"* (22b-1); SD-43 ships every item as a working first version | the operator's SD-44 intake lists layout items: `ls docs/release/SD-44-*/ 2>/dev/null \| awk 'END{print NR}'` ≥ 1 | the operator, on the alpha built after E7.4's merge | screens ship with the layouts the rulings describe and no more |
| DEF-6 | **Drop and Discard of a container moving its contents** | item 12 names Sell only and says Drop and Discard "are NOT part of this item"; Q-12.1 | the operator answers Q-12.1 yes. **If that happens during the run, it becomes a card, not a row here** | E7.2 (reads `progress.md ## Operator replies` for a Q-12.1 line: `grep -c '^- Q-12\.1 ' progress.md`) | Drop and Discard of a container behave as today |
| DEF-7 | **Re-merging an emptied container into a stack** | item 5: an emptied container "stays separate; it does not re-merge unless the operator says so"; Q-5.2 | the operator says so (`grep -c '^- Q-5\.2 ' progress.md` ≥ 1) | E7.2 | an emptied sack stays its own row |

## 2. Considered and not deferred

- **Race-specific favored class alternates** are **in** the DoD (`decisions.md §19`, DoD row 13: "and the race's
  alternate options"; ruling 22-a: core first, race alternates second). E3.8 delivers them. Not a row here.
- **Data gaps** (a sub-tab or pool with zero selectable records in the corpus) are **in** the DoD as on-screen
  sentences naming what is missing (ruling 22b-1). They are not deferrals. E0.1 writes the list to
  `artifacts/epic_0/abilities-census.json`; E3.MC proves the built app shows exactly that list. Filling a data gap
  (ingesting records the corpus lacks) is new content work the operator would scope; nothing about it is
  promised here.
- **Engine gaps** (records exist, no budget, prerequisite check or saved field) are SD-43 cards (E3.2–E3.10),
  never notices at closure and never rows here (`decisions.md §18`).

## 3. Excluded, not deferred (no build is planned under any condition this bundle knows of)

| Item | Ruling |
|---|---|
| A codex-web API change for the Handle or anything else | item 15: "No API change" |
| The signed update manifest, beta and stable channels | merged outside SD-43 (PR #419; the brief's own placement ruling, 2026-11-21); SD-43 only re-checks that the publish still signs (`acceptance-and-verification.md` G-R) |
| A Market tab in `SHEET_TABS` | item 8: "do NOT add an entry to the tab list" |
| Editing `PRICE_MODE_OPTIONS` | item 9: the sell-window rulings apply to the sell window only |
| PCGen code or data in live paths | standing (`decisions.md §28`) |

## 4. No deferrals for SD-43 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is `blocked-escalated` in `kanban.md`
with an `## Open blockers` entry in `progress.md`, which pauses the work; the bundle does not close over it.
