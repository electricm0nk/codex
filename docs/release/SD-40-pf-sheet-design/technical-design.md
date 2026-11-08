---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready — every existing path below was checked with ls/grep -n on tranche-19 @ d4489b3557; paths marked (new) are design choices the owning card confirms
---

# SD-40 Technical Design

> **`decisions.md §21` wins where this file differs.**

## §0 Today (tranche-19 @ `d4489b3557`)

```
apps/desktop/src-tauri/src/print/
  mod.rs       190 lines  PrintRequest { paper, spellbook, math, bio, campaigns }; render_sheet_pdf_at_root;
                          sheet_document_at_root picks layout::{pf1e,sf1e,cpr}::layout by game_system;
                          save_sheet_pdf / print_sheet_pdf commands
  model.rs     100        Paper; SheetDocument { title, paper, blocks }; SheetBlock { heading, value_columns,
                          new_page, rows }; SheetRow { label, values, notes, derivation }; Derivation
  typst.rs     151        SheetWorld: main.typ + frame.typ + blocks.typ (include_str!) + sheet.json; 4 fonts
  blocks/      mod.rs + core.rs (identity, combat, abilities) skills.rs gear.rs (weapons, armor, inventory)
               feats.rs (feats, class_features, actions) spells.rs (spells, spellbook) extras.rs (custom, pets)
               math.rs bio_notes.rs; print_pf_blocks.rs (13 tests)           -- 16 pub fn builders
  layout/      mod.rs 292 (Placement, cells_row, with_options, rules_and_features …) pf1e.rs 53 sf1e.rs 422 cpr.rs 643
  print_smoke.rs 8 tests; print_golden.rs 24 cases (text, headings, pages, diagnostics 0, determinism);
  golden/print_golden.json, golden/class_feature_labels.json
apps/desktop/src-tauri/resources/print/frame.typ 27, blocks.typ 31   -- one generic table for every block
apps/desktop/src/characterHub/PrintDialog.tsx   paper select + three checkboxes (PRINT_OPTION_BOXES) + Save PDF / Print
apps/desktop/src/boundary/saveSheetPdf.ts        PrintOptionsDto { paper, spellbook, math, bio, campaigns }
apps/desktop/src/settings/printPaper.ts          codex.print.paper (a4 | letter)
apps/desktop/src/characterHub/sheetLayout.ts     SheetBoxId (13), SquareBoxId (4), SHEET_LAYOUT registry
apps/desktop/src/characterHub/CharacterSheet.tsx:544  SHEET_TABS = Weapons, Defense, Gear, Spells, Pets, Feats, Actions, Custom
```

Line counts: `find apps/desktop/src-tauri/src/print apps/desktop/src-tauri/resources/print -type f | xargs wc -l` (4,226 total).

## §1 The three layers (E1.1; `decisions.md §4`)

```
LoadSavedCharacterResponse + held copies + custom + bio + money  (unchanged; nothing re-derived, NR-2)
        │
        ▼  data blocks (print/blocks/*.rs, unchanged builders; E2.1 splits Combat along the UI's ids)
SheetBlock { id: &'static str, look: Look, place: Place, heading, value_columns, new_page, rows }
        │
        ▼  layout = an ordered &[Placed] list in layout/pf1e.rs (one list per preset)
SheetDocument { title, paper, columns: u8, blocks: Vec<SheetBlock> }
        │  serde → sheet.json
        ▼
frame.typ:  columns == 1 → today's `for b in doc.blocks { sheet-block(b) }`   (SF1e, CPR: unchanged)
            columns  > 1 → grid(columns: doc.columns, ..doc.blocks.map(b => grid.cell(x: b.place.x,
                           colspan: b.place.span, sheet-block(b))))  with `new_page` → pagebreak
blocks.typ: sheet-block(b) = looks.at(b.look)(b)      -- the dispatcher; `table` is today's template
resources/print/looks/<look>.typ (new, one per look)  -- E2/E3 add them; typst.rs LOOKS table lists them
```

- `Look` (Rust enum, `serde(rename_all = "kebab-case")`): `Table` (today), then per card: `Abilities`,
  `HitPoints`, `Initiative`, `ArmorClass`, `Attack`, `Saves`, `Speed`, `Skills`, `Weapons` (E2); `Feats`,
  `ClassFeatures`, `Actions`, `Armor`, `Gear`, `Money`, `Spells`, `Spellbook`, `BioNotes`, `Math`,
  `Custom`, `Pets`, `CharacterDetails` (E3). A variant's name is its file (`looks/abilities.typ`) and the
  key the dispatcher reads; a test asserts every variant has a file and every file a variant
  (`print_pf_blocks_every_look_has_its_template`).
- `Place { x: u8, span: u8 }` on a 12-column grid (`decisions.md §4`); `SheetBlock::new` defaults to
  `Place { x: 0, span: 12 }` and `Look::Table`, so every existing builder compiles unchanged; the PF
  layout sets them.
- `Placed` (layout data): `{ id, build: fn(&SheetSources) -> SheetBlock, look, place, new_page }`; a preset
  is `const BASIC: &[Placed]` etc. (E4.1). `layout(sources)` = the list → blocks, in order.
- Typst 0.15.1 (`Cargo.toml` `typst = "=0.15.1"`) supports `grid.cell(x:, y:, colspan:, rowspan:)` and
  breakable grids; a page-1 grid that overflows flows to page 2 and the golden page count says so.
- The Typst world (`typst.rs`): `LOOKS: &[(&str, &str)] = &[("/looks/table.typ", include_str!(…)), …]`;
  `source()` and `file()` search `[main, frame, blocks] ++ LOOKS`. Adding a look = one file + one table
  line + one enum variant.

## §2 One vocabulary (E1.1; `decisions.md §5`)

`src/print/golden/block_vocabulary.json` (new):

```json
{ "abilities": {"ui": "sheetLayout.SheetBoxId"}, "hitPoints": {"ui": "sheetLayout.SquareBoxId"},
  "initiative": {"ui": "sheetLayout.SquareBoxId"}, "armorClass": {"ui": "sheetLayout.SheetBoxId"},
  "attack": {"ui": "sheetLayout.SheetBoxId"}, "saves": {"ui": "sheetLayout.SheetBoxId"},
  "speed": {"ui": "sheetLayout.SheetBoxId"}, "skills": {"ui": "sheetLayout.SheetBoxId"},
  "characterDetails": {"ui": "sheetLayout.SheetBoxId"}, "weapons": {"ui": "SHEET_TABS"},
  "defense": {"ui": "SHEET_TABS"}, "gear": {"ui": "SHEET_TABS"}, "spells": {"ui": "SHEET_TABS"},
  "pets": {"ui": "SHEET_TABS"}, "feats": {"ui": "SHEET_TABS"}, "actions": {"ui": "SHEET_TABS"},
  "custom": {"ui": "SHEET_TABS"},
  "classFeatures": {"printOnly": "the Feats tab's Class Features panel"}, "money": {"printOnly": "the Gear tab's money row"},
  "spellbook": {"printOnly": "preset"}, "math": {"printOnly": "preset"}, "bioNotes": {"printOnly": "preset"} }
```

Rust: `print_pf_blocks_every_pf_block_id_is_in_the_shared_vocabulary` walks every PF preset's blocks. TS:
`sheetLayout.test.ts` asserts every `ui`-sourced id is a `SHEET_LAYOUT` / `SQUARE_BLOCK_LAYOUT` id or a
`SHEET_TABS` entry lower-cased. `identity` is renamed `characterDetails` (E1.1); `armor` → `defense`,
`inventory` → `gear` (E3.2); headings keep their printed words.

## §3 The proof harness (E1.2; R7)

- `src/print/print_proof.rs` (new, `#[cfg(test)] mod` of the binary like `print_golden`): with
  `PRINT_PROOF_OUT=<dir>` set it writes Aldric and Elowen (the two PF starter seeds,
  `seed_default_characters_at`) for every preset (until E4.1: the four option sets) on both papers to
  `<dir>/<seed>-<paper>-<preset>.pdf`; without the variable it is a no-op test that passes (so the suite
  never fails on a missing dir). Aldric id `00000000-0000-0000-0000-000000000001`, Elowen `…0002`
  (`print_smoke.rs:16`, `print_golden.rs:46`).
- `scripts/print/rasterize.sh <pdf-dir> <png-dir>` (new): `pdftoppm -r 60 -png` per PDF (`/usr/bin/pdftoppm`
  present; `which pdftoppm`), prints one line per PNG, exits non-zero on zero PDFs.
- The per-cycle step (`workflow-instruction.md §6` step 8): run both, `Read` every page-1 PNG and the
  reference PNGs under `artifacts/epic_0/E0.1/`, write the "Looked at" row, iterate up to three passes,
  commit the final PNGs under `artifacts/epic_<n>/<card>/`.

## §4 Page 1 (E2; R5)

Target composition on the 12-column grid, Letter-first (`decisions.md §10`), from PCGen's std sheet
(`fantasy_master_common_blocks.xslt` `match="character"` order: basics, abilities, armor_class,
initiative, bab, encumbrance, skills, saving_throws, attack, weapons, protection …):

```
row 0  characterDetails (x0 span12): name · race · class/level · size · alignment · player   (one band)
row 1  abilities (x0 span3: boxes score | mod | temp | temp mod)   hitPoints (x3 span3: total; write-in current, nonlethal)
                                                                    initiative (x6 span2)  speed (x8 span2)  armorClass (x10 span2: AC / touch / FF)
row 2  saves (x3 span4: Fort / Ref / Will, base + ability + misc write-in)   attack (x7 span5: BAB, melee, CMB / CMD)
row 3  skills (x0 span7: two narrow columns, class-skill mark, ranks, total, write-in)   weapons (x7 span5: table + ammo write-in)
```

E2.3 may move cells to fit Letter; the receipt shows the PNG. **As built (E2.3):** the composition above does not fit Letter with
the E2.1 looks (measured in the E2.3 receipt), so `Place` gained `under` (a block stacked in the cell
of the block before it, same columns) and page 1 reads: row 0 characterDetails (x0 span3) · hitPoints
(x3 span5) · initiative (x8 span4); row 1 abilities (x0 span6) · armorClass with attack under it
(x6 span6); row 2 skills (x0 span7) · weapons with saves and speed under it (x7 span5). Page 2 starts
with `defense` (`new_page`). Everything after page 1 flows as today
until E3 gives it looks and E4 the presets.

## §5 Presets and the dialog (E4; `decisions.md §9`)

- `PrintPreset { Basic, BasicBio, Full, Spellbook }` on `PrintRequest` (serde `camelCase`, default
  `Basic`); E4.1 keeps the three booleans with `#[serde(default)]` and maps them when `preset` is absent;
  E4.2 removes them from `PrintRequest`, `PrintOptionsDto`, `PRINT_OPTION_BOXES` and the golden
  `request()`.
- `layout/pf1e.rs`: `pub const PRESETS: [(PrintPreset, &[Placed]); 4]`; `layout(sources)` picks the list
  by `sources.request.preset`.
- `layout/mod.rs with_options(sheet, spellbook, sources)` reads `preset.options()` → `(spellbook, math,
  bio)` so `sf1e.rs` and `cpr.rs` do not change; `Spellbook` returns only the spellbook block.
- `PrintDialog.tsx`: a radio group `Print style` (four presets, Basic checked) replaces the `Include`
  fieldset; the paper select and both buttons stay; `data-testid="print-preset-<preset>"`; two ui-smoke
  rows (open the dialog, pick Full, Save PDF writes a file; pick Spellbook on Elowen).
- Density: no page-count ceiling (`decisions.md §10`, §22); `print_golden.json` keeps each case's exact
  `pages` count, re-recorded on purpose only with before → after in the receipt.

## §6 Does not cover

Pixel positions (DEF-1): a look whose words land in the wrong cell passes the goldens; the PNGs and the
two gates are the control. SF1e/CPR looks (DEF-4). Any engine value: SD-40 adds no explanations row and
no DTO field the screen reads (`decisions.md §16`).
