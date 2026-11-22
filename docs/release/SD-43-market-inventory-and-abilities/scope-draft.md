---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
date: 2026-11-22
status: planning — front half; every operator ruling is quoted in decisions.md §3; the author's defaults are marked DEFAULT and listed in decisions.md §30
---

# SD-43 Scope Draft — Market, Inventory and the Abilities Tab

## 1. The request (operator, 2026-11-18 to 2026-11-22; `../SD-43-intake/intake.md`)

The operator's order (restated in the dispatch brief of 2026-11-22): **first** improve the Market and
inventory management (intake items 5–12); **then** the Abilities tab (items 22, 22a, 22b) — all seven PCGen
sub-tabs, Feats, Racial Abilities, Traits, Archetype, Background, Class Features, Misc, each with what can be
selected, what is selected, the picks granted per level, qualification enforcement, the UI and the printout;
**then** the rest. On the Abilities tab the operator ruled (2026-11-22): *"all seven sub-tabs in SD-43"* as a
first working version — *"I'm sure we will need to do adjustments in SD-44 once I get to put my hands on
something."* SD-44 is the feedback round, not unfinished scope. Where the engine does not yet hold a sub-tab's
data, SD-43 still ships the tab **with that gap shown on screen and listed in the plan** — never hidden, never
faked, no stub shipped as done; and engine gaps are SD-43 work, *"not a reason to ship an unchecked picker"*.

## 2. Disposition of every intake item

| Item | Title (short) | Disposition | Epic / cards |
|---|---|---|---|
| 0 | Post-SD-42 cleanup | **Mostly done; one step open.** Worktrees: only the main checkout (`git worktree list` → 1 line). Site moved to `electricm0nk/codex-web` (`ls site` → no such directory; commit `f4c8d4a3cc`); codex-web updated 2026-11-21 (`gh api 'repos/electricm0nk/codex-web/commits?per_page=3'` → `161c245581` 2026-11-21). Open: step 4 graphify update — newest dated snapshot is `graphify-out/2026-11-17` (`ls graphify-out`), not evidenced after SD-42's merge | C2 (graphify, no `--force`); closure re-runs it last (E7.4) |
| 1 | Campaign folder relocation does not migrate or make a skeleton | IN | E6.1 |
| 2 | Rest and recovery text (printout under the bio page; copy on the Biography tab; player override) | IN | E2.4, E2.5 |
| 3 | House Rules menu, per character, per game system | IN | E6.2, E6.3, E6.4 |
| 4 | Export and import custom items, local or campaign folder | IN | E6.5 |
| 5 | Gear screen one-column list; container contents wrong after a move | IN | E0.4 (reproduce), E1.1, E1.2 |
| 6 | Double-click (and Enter) equips or carries | IN | E1.4 |
| 7 | Whole container box is a drop target | IN | E1.3 |
| 8 | One standard Market button top-left; Print and pricing control leave the panels | IN | E1.7 |
| 9 | Sell opens a Market sell window (Market Rate / Build / Negotiate) | IN | E1.6 |
| 10 | Market description box shows the item's stats | IN | E1.9 |
| 11 | Market filter columns, no 200 cap, the item counts | IN | E0.2 (counts first), E1.10, E1.11 |
| 12 | Inventory panel sell value; selling a container moves its contents to Carried | IN | E1.5, E1.8 |
| 13 | Bug/enhancement reports through the codex-web report API | IN | E5.1 |
| 14 | Update menu layout | IN | E5.3 |
| 15 | Anonymous selector + Handle on the bug and enhancement screens | IN (same epic as 13) | E5.2 |
| 16 | Options → Developer screen slimmed (option 2, ruled) | IN (one card with 19) | E5.4 |
| 17 | Fixed top bar, left rail and right column; only the centre scrolls; every game system | IN (one card with 21) | E2.1, E2.2 |
| 18 | "Open campaign" button errors; enlarge it; Manager banner smaller | IN | E2.3 |
| 19 | Corpus Ingest Diagnostic moves into Options → Developer | IN (one card with 16) | E5.4 |
| 20 | `[NAME]` placeholders in item names; every item a proper name; one test over all item kinds | IN, with one operator wait (names list approval) | E0.3, E4.1, E4.G, E4.2 |
| 21 | "Show Progression" cut off on a shorter display | IN (one card with 17) | E2.1 |
| 22 / 22a / 22b | The Abilities tab: seven sub-tabs, grants and prerequisites enforced, UI and printout | IN | E0.1, E3.1–E3.11, E3.MC |
| 23 | Minor version follows the SD number (0.43) | **Done** in `147a3f71eb`, except `current-release.json` | C1 |
| Brief | Signed update manifest, beta, stable default | **Not SD-43** (the brief's own placement ruling, 2026-11-21) and already merged (PR #419) | none; `decisions.md §25` |

## 3. Definition of Done

SD-43 is done when every row below holds on `tranche-43`, every card in `kanban.md` is `complete`, and the
operator has approved the names list (E4.G). A row's card proves it with the command in `epic-breakdown.md`.

### A. Market and inventory (items 5–12)

1. **Gear list.** Under the equipment set the gear screen is ONE column, a list. Action buttons (Customize,
   Move to, Drop, Sell, Discard) sit on the item's line. Order: non-containers A–Z, then containers A–Z with
   their nesting, the same two-part order at every depth, natural number order (`Sack #2` before `Sack #10`).
   Contents indented; each container in a light box, nested boxes inside the parent's. Not Carried is its own
   section and adds nothing to encumbrance; the screen's carried weight equals the sheet's
   `encumbrance.totalCarriedWeightLbs`.
2. **Containers hold what they hold.** A container shows only the copies actually in it; a move or remove
   changes source and target at once; the Not Carried list matches the left side. A container that holds items
   is never part of a stack; only identical EMPTY copies stack, shown `Name xN`; no `#N` numbering; a renamed
   copy shows its custom name on both panels; an item put into a stacked empty container splits that copy off
   first.
3. **Double-click / Enter.** On an item with a slot: to the first free slot it fits; if none free, swap with the
   first slot in `WORN_SLOTS` order it fits, old item to Carried; already in a slot → Carried; one piece of a
   stack per action; never fires from a row button; Enter on a focused row does the same.
4. **Drop anywhere in the box.** The container's header, its empty space and any item inside it are all "into
   this container", the nearest container when nested; a drop on the item's own container is a no-op; a
   container cannot go into itself or its contents.
5. **Market button.** One shared component, top-left on every panel that has one, twice today's size, the
   existing accent colour; no Print button on the panels (Print stays in the ☰ menu, which already has it);
   no pricing control on Gear (or Weapons, DEFAULT).
6. **Sell window.** Sell opens a small window: Market Rate (default, `standard`, 50 %), Build
   (`characterBuild`, 100 %), Negotiate (typed gp); the default amount shown for the choice; How many? for a
   stack; Sell / Cancel; Escape and Enter; "the pricing above" gone everywhere; the window's choice never
   changes the Market's session price mode; no Cashless option and the word "Normal" nowhere in it.
7. **Stats in the Market.** The description box lists the engine's stats for weapons, armor, shields, gear and
   magic items (stats first, then prose, DEFAULT), `—` for a stat the record does not state, following the
   `+N` choice; formatted by the UI, never derived; one renderer shared with the gear detail panel where the
   record shapes allow.
8. **Every Market row reachable.** No 200 cap; a windowed list; filter columns per list with an `Other` value;
   the per-list counts reconciled against `built_catalog()` with every dropped row's reason; header "N of N".
9. **Sell values and container sales.** The Market inventory shows each row's sell value from the same
   function as the sell window (`sellNegotiateGp`), live with the price option; selling a container moves its
   direct contents to Carried (nested containers keep theirs), on both screens.

### B. The Abilities tab (items 22, 22a, 22b)

10. **Seven sub-tabs.** Feats, Racial Abilities, Traits, Archetype, Background, Class Features, Misc, each
    showing: the picks granted at each level and their source (class level, race, archetype, background), what
    can be selected (with search and a Qualified filter), what is selected, the remaining picks per pool, an
    info pane with the rule text.
11. **The engine enforces.** For every sub-tab the engine owns the budget and the prerequisite check, as
    `feats.budget.*` does for feats; the UI renders rows and never re-derives a grant or a prerequisite. A pick
    with no grant left is refused with the pool named; a pick the character does not qualify for is refused
    with the reason. Overrides (the GM's allowance) use the existing `Overrides` mechanism with flags shown.
12. **Saved and printed.** Every selection is saved on the character, survives reload, and an old save loads
    unchanged; the printout lists the chosen abilities and the favored class bonus as text under the same
    headings.
13. **Favored class.** The player picks the favored class and, per level, hit point or skill rank (and the
    race's alternate options); the HP and skill totals follow, computed by the engine.
14. **Gaps shown, never faked.** Two kinds, kept apart (`decisions.md §18`):
    - an **engine gap** (no budget, no prerequisite check, no saved field for a sub-tab whose records exist) is
      SD-43 work — a card, cleared before closure (operator: engine gaps are *"not a reason to ship an
      unchecked picker"*);
    - a **data gap** (the corpus holds no records for a sub-tab or pool, so there is nothing to select) ships as
      an on-screen sentence naming what is missing (operator: *"SD-43 still ships the tab with that gap shown
      on screen, and the gap is listed in the plan"*). E0.1 lists every data gap with its count; E3.MC
      re-counts the notices the built app shows and the two lists must be equal.

### C. The rest

15. **Campaign folder relocation** carries the content and writes the skeleton (item 1).
16. **Rest and recovery** text per race and class, printed under the bio page and shown read-only under the
    Biography text, from one source, with a player override (item 2).
17. **House Rules** menu per character and per game system with the six rules (item 3).
18. **Custom items** export/import, local or campaign folder, a same-name import edits (item 4).
19. **Reports** through the codex-web API when Anonymous is on (optional Handle as the body's first line),
    the GitHub browser handoff when off; every reply handled; "sent for review" on success (items 13, 15).
20. **Update menu** layout (item 14). **Developer tab** slimmed, Corpus Ingest Diagnostic embedded, landing
    entry removed (items 16, 19).
21. **Sheet layout**: top bar, left rail and right column fixed, only the centre scrolls, sized to the real
    viewport; Show Progression fully visible at a short window; on PF, SF and CPR (items 17, 21).
22. **Open campaign** button opens the folder, one full-width button per campaign; Manager banner smaller
    (item 18).
23. **Names**: no display name of any item kind contains `[`, `]` or `NAME`; one test guards it; names
    proposed from the book text and approved by the operator (item 20).
24. **Release**: `current-release.json` names tranche 43 and this package's notes, which resolve (item 23).
25. **Closure**: all cards complete → retrospective written and cited → sweep → release notes → architecture
    truth-up → graphify last → PR last; the lead merges when green and watches the develop publish
    (`decisions.md §26`).

Every code-bearing card passes the four-check audit of `../../governance/no-stub-mvp-doctrine.md` §"Per-cycle
audit", output in its receipt.

## 4. Non-goals (things SD-43 does not set out to do; none is in the Definition of Done)

- **No simulation.** Rule text is printed; only values that feed a sheet total are computed (STANDING,
  `decisions.md §27`). Pool members' effects are shown as their rule text; SD-43 adds no compute for them.
- **No new game-system support.** SD-43 does not add Starfinder or Cyberpunk RED features beyond what the
  rulings name for them (item 17 layout, item 3's per-system filter, item 8's button check).
- **No PCGen code in live paths.** The pinned oracle is read for reference only (STANDING, `decisions.md §28`).
- **No change to the Market window's own price labels** (`PRICE_MODE_OPTIONS` untouched; item 9 scope ruling).
- **No change to the signed-update pipeline** (merged outside SD-43).

## 5. Out of scope (real exclusions; each was never in the Definition of Done)

| Excluded | Why it is not a laundered blocker |
|---|---|
| Retraining (swapping an earlier pick for a new one at a later level) | The operator's strawman (item 22, 2026-11-22) lists it as left for the next SD; no later ruling brings it in; it is a separate mechanic, not a sub-tab. Open to the operator (`decisions.md §30` Q-22.5) |
| Abilities that *replace* an earlier pick (advanced/master tricks taking a slot of a lower trick) as a replacement mechanic | Same strawman list. Advanced and master tricks are still **selectable** as pool members with their prerequisites (they are in the DoD as members); only the replace-a-pick mechanic is excluded. Open to the operator (Q-22.5) |
| Abilities tab on the Starfinder and Cyberpunk RED sheets | Intake 22/22b describe PCGen's Pathfinder tab and Pathfinder pools; no ruling names SF or CPR. DEFAULT, open to the operator (Q-22.6) |
| Drop and Discard of a container moving its contents | Item 12 names Sell only and says Drop and Discard "are NOT part of this item"; asked as Q-12.1 with a default. If the operator answers yes, it becomes a card, not a deferral |
| A codex-web API change | Item 15 ruling: "No API change" |
| Two-way merge or re-merge of emptied containers into a stack | Item 5: an emptied container "stays separate; it does not re-merge unless the operator says so" |

## 6. Epic order

C (package, current-release pointer, graphify) → **E0 census** (answers every "not yet checked" with a
command; nothing downstream starts on a guess) → **E1 Market and inventory** and **E2 sheet layout** and **E5
settings/reports** and **E6 campaign/house rules/custom items** in parallel lanes on disjoint files
(`epic-breakdown.md §0`) → **E3 Abilities** (after E0.1; shares `CharacterSheet.tsx` with E1.7, E2.x, so it is
serialized against them on that file) → **E4 names** (E4.G waits for the operator) → **E7 closure**.

## 7. Mode

Unattended after launch except at E4.G. A question with a default in `decisions.md §30` takes the default and
logs it; nothing else waits.
