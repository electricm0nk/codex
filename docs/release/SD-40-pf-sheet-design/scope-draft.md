---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready — the rulings are the operator's (artifacts/cycle_0/operator-brief-2026-11-07.md); the author's defaults are marked and name their revisit
---

# SD-40 Scope Draft — Pathfinder Sheet Design

## 1. The defect (operator, 2026-11-07)

> "These three are HUGE steps in the correct direction. They are technically usable, but they lack both
> artistry and density. They look more like a table of contents than a character sheet."

What the code does today (checked on `tranche-19` @ `d4489b3557`, 2026-11-07):

- Every block renders through **one** generic Typst table (`resources/print/blocks.typ`, 31 lines;
  `frame.typ`, 27 lines: `cat apps/desktop/src-tauri/resources/print/*.typ | awk 'END{print NR}'` → 58)
  and blocks stack full width in the order the layout file lists them; there is no page grid and no
  per-block look.
- The data already exists as blocks: 16 block builders
  (`grep -h '^pub fn ' apps/desktop/src-tauri/src/print/blocks/*.rs | awk 'END{print NR}'` → 16; 15 block
  types, Spells and Spellbook sharing one), the model `SheetDocument → SheetBlock{heading, value_columns,
  new_page, rows} → SheetRow{label, values, notes, derivation}` (`print/model.rs`, 100 lines), and three
  layout files (`print/layout/{pf1e,sf1e,cpr}.rs`).
- Aldric (Fighter 3), the "full" PF sheet, is **10 pages** of one-column label/value rows
  (`pdfinfo docs/release/SD-39-ui-print-corrections/artifacts/epic_9/E9.4/Aldric-Ironhand-PF1e-full-A4.pdf | grep Pages` → 10);
  Elowen's basic sheet is 5 (`print_golden.json` `pf1e-a4-none.pages` → 5).
- The golden suite (`print/print_golden.rs`, 24 cases) checks extracted text, heading order and page
  counts, never appearance, so nothing pushed visual quality.

## 2. Definition of Done

SD-40 is done when every row below holds on `tranche-20` and the operator has signed both gates.

1. **Three layers.** Data blocks (Rust, numbers only), block looks (one Typst template per block kind,
   dispatched by a `look` the block names), and layouts (an ordered list of blocks with a place on a page
   grid, held as data in the system's layout file). Adding a print style is adding a list; adding a look
   is adding one `.typ` file and one table line. (`decisions.md §4`, `technical-design.md §1`.)
2. **Shared vocabulary.** Every PF print block carries an id that is a `SheetBoxId` or a `SHEET_TABS`
   entry of `sheetLayout.ts` / `CharacterSheet.tsx`, or one of the print-only ids a shared fixture
   allow-lists; a Rust test and a TS test both read that fixture. (`technical-design.md §2`.)
3. **Step 1, the operator gate (E2.G, round 2; redesigned at round 1, `decisions.md §23`).** Aldric's page 1
   prints **totals only** on the grid with write-in space: portrait + Character Name + Campaign (no title, no
   identity block); one block of Hit Points (max, big current area, Nonlethal), Initiative, Encumbrance
   (limits, carried, remaining, gear value) and Conditions, with the engine's conditional notes for HP and
   initiative; abilities (score + modifier); Armor Class with every part filled from engine rows; attack
   (BAB, melee, CMB/CMD); speed; saves with their conditional notes; weapons (attack, damage as `1d8+4`,
   critical, range); skills in one column (name + total); languages. Every breakdown moves to the
   Show-the-math pages. The operator prints it and writes `usable: yes` per paper.
4. **The rest of the PF blocks have looks, on their own pages:** feats, class features and special
   abilities (actions), daily spells, custom records, pets (section 2); armor, gear (worn | carried | left
   behind), money (section 3, Inventory); the spellbook (section 4); bio details, the Biography (a new
   app field, E3.6), campaign notes and ruled lines (section 5); progression level by level (section 6, `decisions.md §24.3.7`); Math with every page-1 breakdown (section 7). Each section after page 1 starts a page under a prominent title.
5. **Section checkboxes** (replace the presets, operator at E2.G round 1, `decisions.md §23.4`). The print
   dialog offers eight boxes (`decisions.md §24.3.7`, §24.9): Primary (page 1), Abilities and Feats, Inventory, Spellbook, Bio,
   Progression, Math, Notes (off by default); paper stays separate; the ticks are remembered per character, first time 1 + 2 + 3; each section is a
   layout list in `layout/pf1e.rs`; SF1e and CPR show boxes 1, 4, 5, 6 and print through their flow layouts.
6. **Density.** On A4 **and** US Letter, designed Letter-first, with **no page-count ceiling** (operator
   answer "3: No hard limits", `decisions.md §22`): every look card's receipt records each PF case's page
   count before → after, E0.1 records PCGen's counts as the reference figure, and the operator judges
   density by eye at E2.G and E5.2 (`decisions.md §10`).
7. **Looked at.** Every look or layout card rasterized its own output (`pdftoppm -r 60 -png`), read the
   PNGs, compared them to the reference pictures, and kept the PNGs under `artifacts/` with a "Looked at"
   row in its receipt (`workflow-instruction.md §6` step 8).
8. **SF1e and CPR unchanged.** Their golden text and page counts are identical to C1's for the
   four old option sets (none, spellbook, math, all, read through the section mapping); a PDF byte delta exists only through the shared
   frame and is named in the receipt; "Spellbook alone" is the one named addition
   (`decisions.md §23.4`, G-2).
9. **Final look (E5.2).** The operator prints the full set (Aldric × 4 section sets × 2 papers; Elowen
   Spellbook alone and all six) and signs `usable: yes` per set.
10. **Closure.** All cards `complete` → final-acceptance scan → retrospective written and cited → sweep →
    release notes → architecture truth-up + claims critic → **graphify LAST** → **PR last**
    (`decisions.md §15`). The operator merges.
11. **Stale ui-smoke rows (E0.2).** The 14 rows SD-39's E2.4 full run recorded red pass by name under
    `npm run ui-smoke`, each repaired to the current UI or rewritten to match it, none deleted (FSR-C1 →
    COM-9, operator answer "1: B", `decisions.md §22`; G-9).

## 3. Not in the Definition of Done (planned deferrals, `forward-scope-register.md §4`)

SF1e and CPR looks and grids (they adopt the architecture later); a new font family; colour; a portrait
on pages 2+ (page 1's portrait box is in scope since `decisions.md §23`); pixel-snapshot tests; a custom layout editor in the app; PCGen's companion / notes /
condensed sheets as print sections.

## 4. Epic order

C → E0 (reference render + block map, side lane `ref`; the 14 stale ui-smoke rows, side lane `smoke`, landing
before E4.2) → E1 (layout model + grid + harness) → E2 (step-1 looks)
→ **E2.G operator gate** (round 1 → replan `decisions.md §23` → engine lane E2.6–E2.8, E2.14 beside the
new page-1 cards → round 2) → E3 (remaining looks) → E4 (sections + checkbox dialog) → E5 (closure,
with the **E5.2 final look**). The main lane is serial: every look card edits `resources/print/*.typ`,
`print/typst.rs`'s look table and `print/golden/print_golden.json` (`epic-breakdown.md §0`).

## 5. Mode

Unattended after launch except at the two operator gates: E2.G (page 1) and E5.2 (the full set). A taste
question inside scope takes the safe default in `decisions.md §13.1` and is logged; nothing else waits.
