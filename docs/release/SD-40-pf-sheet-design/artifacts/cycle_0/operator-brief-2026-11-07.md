# SD-40 — the operator's brief, 2026-11-07

The planning session received this brief from the lead on 2026-11-07, after the operator printed SD-39's
three sheets (E9.4) and signed them `usable: yes`. Quoted text is the operator's or the lead's, verbatim;
the rest is the lead's condensation of the operator's rulings. `decisions.md §3` turns the rulings into
R-rows; this file is the record.

## Why SD-40 exists

> "These three are HUGE steps in the correct direction. They are technically usable, but they lack both
> artistry and density. They look more like a table of contents than a character sheet."

The PF "full" sheet for Aldric (Fighter 3) is 10 pages of one-column label/value rows
(`../../../SD-39-ui-print-corrections/artifacts/epic_9/E9.4/Aldric-Ironhand-PF1e-full-A4.pdf`; `pdfinfo` → 10).

## Rulings (numbered as the lead gave them)

1. **Architecture = three layers**, the operator's own idea: (a) **data blocks** (numbers only; mostly
   exist), (b) **block looks** (each block its own Typst template: abilities as boxes, skills in two narrow
   columns, weapons as a table, etc.; tweakable one at a time), (c) **layouts** = an ordered list of blocks
   plus their place on a page grid. Print styles are layouts, not code: e.g. Basic, Basic + Bio, Full with
   the math, Spellbook. Adding a print style = adding a list. Block names should match the app's existing
   UI blocks (skills, hit points/initiative, weapons, armor class, saves, attacks …; see
   `apps/desktop/src/characterHub/sheetLayout.ts`) so screen and paper share one vocabulary.
2. **Pathfinder 1e only** for the design work: "getting just the one we use the most working well and
   looking great". The architecture must be system-neutral so SF1e and CPR can adopt it later; in SD-40
   their output must not change (their golden PDFs/text stay byte-identical or the delta is named and
   justified).
3. **Visual model:** lean toward PCGen's output sheets over Paizo's (PCGen already has std / simple /
   spell-list-only / spellbook 3x5 / bio / notes pages). Use them as a PICTURE TO AIM AT ONLY. Standing
   rule: "It's not fine to include 1 line of java in our live code"; no PCGen template, XSL or token
   enters the app. PCGen output templates live under the pinned oracle's `outputsheets/d20/fantasy/pdf/`
   (e.g. `csheet_fantasy_std_*.xslt`, `csheet_fantasy_simple_*`, `csheet_fantasy_spellbook*`), resolved via
   `$PCGEN_REPO_DIR` per AGENTS.md (pin in `scripts/pcgen-oracle-pin.env`; never the literal path in docs).
   Java and the PCGen build (`build/libs/pcgen-6.09.08.RC1.jar`, `build/install/pcgen/`) exist on this box.
4. Operator answered: branch = `tranche-20` from `tranche-19` now (catch up to `develop` after SD-39
   merges); YES to a tool-side PCGen reference render of Aldric on PCGen's standard Pathfinder sheet (and
   the simple + spellbook ones if cheap) as the target picture, stored under the package's `artifacts/`,
   never shipped; operator check = **stop only after step 1**, then run the rest unattended.
5. **Step 1 (the operator gate)** = the page grid + new looks for the first 4–5 PF page-1 blocks:
   abilities, combat (HP, AC/touch/flat-footed, initiative, BAB, CMB/CMD, speed), saves, skills, weapons.
   The run renders Aldric's page 1, stops, and the operator prints it and writes `usable: yes` (or
   defects). Later steps: remaining blocks (feats, special abilities/class features, armor/gear/equipment
   incl. worn vs carried vs left behind, money, spells and spellbook, bio, campaign notes, show-the-math),
   the layout presets, density target, Letter paper, and closure. Do not "swing for the fences": each step
   bounded.
6. **Standing rules that bind:** paper-sheet generator, not a video game; print the rule, compute only
   numbers that land on the sheet, dice stay literal; leave some write-in space where a player writes at
   the table (current HP, ammo, spells used, notes); orchestrator never edits shipping code; every
   `agent()` sets a model (Opus default, Sonnet for well-specified mechanical work, Haiku for
   housekeeping); one writer per tree; never `git stash`; batch big; one-pass verify.
7. **New mechanism the operator approved:** design agents **look at their own output**: rasterize the PDF
   (`pdftoppm -r 60 -png`) and read the PNG images, compare against the PCGen reference picture, and
   iterate before claiming done. Put this in the per-cycle procedure for every look/layout card, and keep
   the PNGs in artifacts. Text goldens stay as the regression net; add a density/page-count check (e.g.
   Aldric Basic fits N pages — pick a target from the reference render and justify it).

## What the lead asked the planning session for

The full chassis file set; C1 = version bump to 0.20.0 on every version surface SD-39's C1 touched;
roughly 15–22 cards; suggested epics E0 reference render + block inventory, E1 layout model + page grid,
E2 step-1 looks + the operator gate, E3 remaining PF looks, E4 layouts/presets + print dialog + Letter,
E5 closure; every card with criterion, FILES YOU OWN closed under the change, an acceptance command that
cannot match 0 tests, and "does not cover"; every figure with its command; an unattended-mode safe-defaults
section so cards never stop on taste questions except at the two operator gates (step 1 and the final look).
