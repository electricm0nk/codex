---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
date: 2026-11-01
status: planning-ready — the defects are the operator's words; the rulings are the operator's answers
---

# SD-39 Scope Draft — UI and Print Corrections

## 1. The defects (operator, 2026-11-01, in dictation order)

| # | Defect | Operator's words (condensed, not reinterpreted) | Epic |
|---|---|---|---|
| D1 | Printing unusable | "outputs a jumble of lines to a pdf file. In no way does it represent anything like a character sheet." Third attempt. Wants: A4 default + US Letter; one core template adapted per system (Pathfinder character → Pathfinder sheet, Starfinder → Starfinder sheet); options basic / spellbook / full (math + bio + campaign notes) | E8 |
| D2 | Main view layout | Move Abilities to where Character Details is; Character Details to the left panel below Class; the left-nav history (levels, feats, what next) becomes a **Show Progression** button opening a dismissable flyover; Armor Class / Attack / Speed / Saving Throws join the 2x2 (HP / Initiative / Encumbrance / Conditions) row as a 1x4 | E2 |
| D3 | Add Armor lists non-armor | "ammunition, rods, weapons"; replace Add Weapon / Add Armor / Add Gear with a single **Market** button that buys and sells inventory | E3 |
| D4 | Magic variants missing | "Every armor should have its magical counterparts. Same with most weapons." Leather Armor, +1 Leather Armor, +2 … | E3 |
| D5 | Equipping mechanic missing | Body slots (helmet → head, ring → ring …) as in the PCGen source; Gear section gets an Equipment control for carried vs worn; "Worn armor affects the armor class, carried armor does not. That entire mechanic is not present." | E4 |
| D6 | Feats unguarded | Can add feats with none left; can add the same feat repeatedly | E1 |
| D7 | Feat picker filter | Checkbox to hide feats whose prerequisites are not met | E1 |
| D8 | Left column title | "Progression" → "Character Details" | E2 |
| D9 | Progression content | The flyover shows what was chosen at each level: feats, spells learned, skills, abilities | E5 |
| D10 | Add Custom top row | Click highlights; clicking another leaves the first light with white text instead of reverting | E1 |
| D11 | Custom equipment stats | Real stats (to-hit, armor class, …) and a slot, both optional, instead of text only | E4 |
| D12 | Character import | Must support PCGen character files | E7 |

The verbatim transcript is `artifacts/cycle_0/defects-and-rulings-2026-11-01.md`.

> **Corrections to the Definition of Done below:** `decisions.md §25` and rulings R17–R20 win where they differ.

## 2. Definition of Done

SD-39 is done when every row below holds on `tranche-19` and the operator has signed E9.4.

1. **Print.** Save PDF and Print on the Pathfinder, Starfinder and Cyberpunk sheets produce a PDF from the
   Rust generator (never `window.print()`); A4 by default, US Letter by setting or one-off override; the
   three stacking options (Spellbook, Show the math, Bio and campaign notes) change the output; golden
   text-extraction tests pass for every system × paper × option set named in G-10; the operator has
   looked at one printed sheet per system and recorded "usable" in E9.4's receipt.
2. **Layout.** The Pathfinder sheet matches `decisions.md §7`'s target layout; the left panel is titled
   Character Details; Show Progression opens a drawer that closes by X, click-outside and Escape.
3. **Feats.** Add is refused at zero remaining slots (per pool), refused for a duplicate unless the feat is
   stackable with a new target, and the picker hides unmet feats by default; an over-budget legacy save
   shows the badge and blocks adds, nothing is deleted.
4. **Market.** The three Add buttons are gone on all three systems; Market lists only the category chosen;
   PF armor, shields and weapons offer +1…+5 generated at runtime at Core Rulebook prices; buy and sell
   honour the three price modes; an unaffordable buy is refused.
5. **Equipping.** Every PF equipment record that names a body location (tags or `Slot:` line) resolves to a slot; worn
   items apply and carried items do not; a taken slot refuses a second item with the slot named; the Gear
   tab has an Equipment control; custom items may declare structured bonuses and a slot.
6. **Ledger.** Every new feat / spell / skill rank / ability increase is stamped with the level taken;
   legacy saves are migrated once by slot order and marked; the drawer lists real choices per level.
7. **Overrides.** The Custom tab has the eight toggles; each is honoured by the command it names; every
   item added under an override carries a visible tag in the UI and on the "full" sheet.
8. **Import.** The sidecar converts a `.pcg` to a Codex character; the Import button offers PCGen files;
   unknown items become custom items listed in the report; no live-app code parses `.pcg`.
9. **Scope fences.** Starfinder and Cyberpunk seed outputs are unchanged by every card outside E3.5 and
   E8.3; every PF seed delta is explained field by field (`decisions.md §18`).
10. **Closure.** The corrected order (`decisions.md §17`): all cards complete → retrospective cited → sweep
    → release notes → truth-up → graphify → PR. The operator merges.

## 3. Not in the Definition of Done (planned deferrals, `forward-scope-register.md §4`)

Magic special abilities (Flaming …) and special materials (mithral …); Starfinder armor-upgrade slots and
Cyberpunk install locations; a GM-override path on prerequisites beyond the Overrides toggles; PCGen XML
import; pixel-snapshot print tests; editing a past level's choice from the drawer.

## 4. Epic order (operator ruling R16: UI first, print last)

C → E1 guards → E2 layout → E3 Market → E4 slots → E5 ledger → E6 overrides → E7 import → E8 print → E9 closure.
Cross-epic parallelism is limited by the shared files `CharacterSheet.tsx`, `character_hub.rs`, `main.rs`
and `spec.json` (`epic-breakdown.md §0`).

## 5. Mode

Unattended: the run takes safe defaults
and reports after each epic (`decisions.md §16`). E9.4 is the only card that waits for the operator.
