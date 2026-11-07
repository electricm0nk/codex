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
3. **Step 1, the operator gate (E2.G).** Aldric's page 1 holds abilities, hit points / initiative /
   armor class / attack (BAB, melee, CMB/CMD) / speed, saves, skills (two narrow columns) and weapons
   on the grid with their own looks and write-in space; the operator prints it and writes `usable: yes`.
4. **The rest of the PF blocks have looks:** feats, class features and special abilities (actions),
   armor, gear (worn | carried | left behind), money, spells and the spellbook, bio and campaign notes,
   show-the-math, custom records and pets.
5. **Presets.** The print dialog offers **Basic, Basic + Bio, Full with the math, Spellbook**; each is a
   layout list in `layout/pf1e.rs`; the three option checkboxes are gone; SF1e and CPR print the same
   presets through their flow layouts (`decisions.md §9`).
6. **Density.** On A4 **and** US Letter, designed Letter-first, with **no page-count ceiling** (operator
   answer "3: No hard limits", `decisions.md §22`): every look card's receipt records each PF case's page
   count before → after, E0.1 records PCGen's counts as the reference figure, and the operator judges
   density by eye at E2.G and E5.2 (`decisions.md §10`).
7. **Looked at.** Every look or layout card rasterized its own output (`pdftoppm -r 60 -png`), read the
   PNGs, compared them to the reference pictures, and kept the PNGs under `artifacts/` with a "Looked at"
   row in its receipt (`workflow-instruction.md §6` step 8).
8. **SF1e and CPR unchanged.** Their golden text and page counts are identical to C1's for Basic,
   Spellbook and Full (= the old none, spellbook, all); a PDF byte delta exists only through the shared
   frame and is named in the receipt; the new Basic + Bio preset is the one named addition
   (`decisions.md §9`, G-2).
9. **Final look (E5.2).** The operator prints the full set (Aldric × 4 presets × 2 papers; Elowen
   Spellbook and Full) and signs `usable: yes` per preset.
10. **Closure.** All cards `complete` → final-acceptance scan → retrospective written and cited → sweep →
    release notes → architecture truth-up + claims critic → **graphify LAST** → **PR last**
    (`decisions.md §15`). The operator merges.
11. **Stale ui-smoke rows (E0.2).** The 14 rows SD-39's E2.4 full run recorded red pass by name under
    `npm run ui-smoke`, each repaired to the current UI or rewritten to match it, none deleted (FSR-C1 →
    COM-9, operator answer "1: B", `decisions.md §22`; G-9).

## 3. Not in the Definition of Done (planned deferrals, `forward-scope-register.md §4`)

SF1e and CPR looks and grids (they adopt the architecture later); a new font family; colour; portrait
image on the sheet; pixel-snapshot tests; a custom layout editor in the app; PCGen's companion / notes /
condensed sheets as presets.

## 4. Epic order

C → E0 (reference render + block map, side lane `ref`; the 14 stale ui-smoke rows, side lane `smoke`, landing
before E4.2) → E1 (layout model + grid + harness) → E2 (step-1 looks)
→ **E2.G operator gate** → E3 (remaining looks) → E4 (presets + dialog) → E5 (closure,
with the **E5.2 final look**). The main lane is serial: every look card edits `resources/print/*.typ`,
`print/typst.rs`'s look table and `print/golden/print_golden.json` (`epic-breakdown.md §0`).

## 5. Mode

Unattended after launch except at the two operator gates: E2.G (page 1) and E5.2 (the full set). A taste
question inside scope takes the safe default in `decisions.md §13.1` and is logged; nothing else waits.
