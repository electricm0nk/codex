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
        ▼  layout = an ordered &[Placed] list in layout/pf1e.rs (one list per print section, `decisions.md §23.4`)
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
  key the dispatcher reads (2026-11-11, `decisions.md §23`: E2.9 folds `HitPoints` and `Initiative` into
  `SquareBlock`; E2.5, E2.13, E3.4, E3.5 add `Portrait`, `NameCampaign`, `Languages`, `Bio`, `Math`, `Breakdown`);
  a test asserts every variant has a file and every file a variant
  (`print_pf_blocks_every_look_has_its_template`).
- `Place { x: u8, span: u8 }` on a 12-column grid (`decisions.md §4`); `SheetBlock::new` defaults to
  `Place { x: 0, span: 12 }` and `Look::Table`, so every existing builder compiles unchanged; the PF
  layout sets them.
- `Placed` (layout data): `{ id, build: fn(&SheetSources) -> SheetBlock, look, place, new_page }`; a section
  is one `&[Placed]` list (E4.1, `decisions.md §23.4`). `layout(sources)` = the list → blocks, in order.
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

Rust: `print_pf_blocks_every_pf_block_id_is_in_the_shared_vocabulary` walks every PF section's blocks. TS:
`sheetLayout.test.ts` asserts every `ui`-sourced id is a `SHEET_LAYOUT` / `SQUARE_BLOCK_LAYOUT` id or a
`SHEET_TABS` entry lower-cased. `identity` is renamed `characterDetails` (E1.1); `armor` → `defense`,
`inventory` → `gear` (E3.2); headings keep their printed words.

## §3 The proof harness (E1.2; R7)

- `src/print/print_proof.rs` (new, `#[cfg(test)] mod` of the binary like `print_golden`): with
  `PRINT_PROOF_OUT=<dir>` set it writes Aldric and Elowen (the two PF starter seeds,
  `seed_default_characters_at`) for every section set (until E4.1: the four option sets) on both papers to
  `<dir>/<seed>-<paper>-<sections>.pdf`; without the variable it is a no-op test that passes (so the suite
  never fails on a missing dir). Aldric id `00000000-0000-0000-0000-000000000001`, Elowen `…0002`
  (`print_smoke.rs:16`, `print_golden.rs:46`).
- `scripts/print/rasterize.sh <pdf-dir> <png-dir>` (new): `pdftoppm -r 60 -png` per PDF (`/usr/bin/pdftoppm`
  present; `which pdftoppm`), prints one line per PNG, exits non-zero on zero PDFs.
- The per-cycle step (`workflow-instruction.md §6` step 8): run both, `Read` every page-1 PNG and the
  reference PNGs under `artifacts/epic_0/E0.1/`, write the "Looked at" row, iterate up to three passes,
  commit the final PNGs under `artifacts/epic_<n>/<card>/`.

## §4 Page 1 (E2; R5; redesigned by the operator at E2.G round 1, `decisions.md §23`)

**Until 2026-11-11 (E2.3 as built):** row 0 characterDetails (x0 span3) · hitPoints (x3 span5) · initiative
(x8 span4); row 1 abilities (x0 span6) · armorClass with attack under it (x6 span6); row 2 skills (x0 span7) ·
weapons with saves and speed under it (x7 span5); page 2 starts with `defense`. `Place` has `under` (a block
stacked in the cell of the block before it).

**Target after round 1 (E2.5, E2.9–E2.13, assembled and made overflow-safe by E2.4; Letter first):** page 1 is
the most-used page and prints **totals only** (the AC parts are the exception). A starting composition the
cards may move to fit Letter (each receipt shows the PNG):

```
row 0  portrait (x0 span2, framed)   nameCampaign (x2 span10: "Character Name" · "Campaign"; no title above)
row 1  squareBlock (x0 span12: Hit Points max + big current area + small Nonlethal + DR | Initiative total |
       Encumbrance L/M/H, carried, remaining, gear value | Conditions box; then HP / initiative notes, blank lines)
row 2  abilities (x0 span3: score + modifier, white space)   armorClass (x3 span5: AC / Touch / FF + seven parts)
       attack (x8 span4: BAB, Melee, CMB, CMD) with speed under it
row 3  skills (x0 span4: one column, name + total; "Not added to a total" under it)
       saves (x4 span8: three totals + notes)  with weapons (name, attack, 1d8+4, crit, range; ammo) and
       languages (comma list + one write-in line) under it
```

Data sources (all engine values; nothing computed in Rust print code or Typst, NR-2):

| Page-1 item | Source | Card |
|---|---|---|
| Portrait | `<character dir>/portrait.png` (`load_character_portrait`) | E2.5 |
| Character Name / Campaign | `summary.display_label`; `PrintRequest.campaigns[].name` | E2.5 |
| HP max, initiative total, AC / Touch / FF, BAB, Melee, CMB, CMD, speed, save totals, ability score + modifier, skill totals | the rows and snapshot fields E2.1–E2.3 already read | E2.9–E2.11 |
| Encumbrance limits, carried, gear value | `corpus_derived.encumbrance` (`EncumbranceDto`) | E2.9 |
| Remaining lb | `EncumbranceDto.remainingLbs` (new; today TS-only, `encumbranceTabModel.ts:75`) | E2.8 → E2.9 |
| Conditions | `custom.conditions` | E2.9 |
| HP / initiative / save / AC notes | `conditionalNotes` (new) | E2.6 → E2.9, E2.10 |
| AC parts | `defense.armor_class.<part>` rows (new; today the parts exist only as terms in `combat.rs`'s sum and its detail words) | E2.7 → E2.12 |
| Weapon attack total | `combat.weapon_attack_bonus.<weapon>` | E2.10 |
| Weapon damage `1d8+4`, range | `combat.weapon_damage_total.<weapon>` and the range field (new; the engine refuses a damage total today, `contract.rs:215-226`) | E2.14 → E2.10 |
| Languages | `languages { known }` (new; today TS-only, `spokenLanguages`); paper prints the list + exactly one blank line (`decisions.md §23.7`) | E2.8 → E2.13 |

Overflow (new SD-b, E2.4): a page-1 list that does not fit its cell shrinks to 8 pt, then continues at the top
of page 2 under "<heading> (continued)"; no page-1 block leaves page 1.

## §5 Sections and the checkbox dialog (E4; `decisions.md §23.4`, superseding §9)

- `PrintRequest.sections: PrintSections { main, feats, inventory, spellbook, bio, math }` (serde camelCase);
  E4.1 keeps the three booleans with `#[serde(default)]` and maps them when `sections` is absent; E4.2 removes
  them from `PrintRequest`, `PrintOptionsDto`, `PRINT_OPTION_BOXES` and the golden `request()`. Nothing ticked:
  `print.no_sections`.
- `layout/pf1e.rs`: `pub const SECTIONS: [(Section, &[Placed]); 6]`; `layout(sources)` concatenates the ticked
  lists in box order, each from a new page: (1) page 1; (2) `feats`, `classFeatures`, `actions`, `spells`,
  `custom`, `pets`; (3) `defense`, `gear`, `money`; (4) `spellbook`; (5) `characterDetails`, Backstory (E3.6), campaign notes,
  ruled lines; (6) breakdown tables + derivations, built from the page-1 blocks even when (1) is unticked.
- `layout/mod.rs with_options(sheet, spellbook, sources)` maps `sections` → SF1e/CPR's `(spellbook, math, bio)`
  and drops their sheet when Main is unticked, so `sf1e.rs` and `cpr.rs` do not change.
- Goldens: the 24 names stay (none = 1+2+3, spellbook = 1+2+3+4, math = 1+2+3+6, all = 1–6) + 6
  `spellbookOnly` = 30.
- `PrintDialog.tsx`: six checkboxes (`data-testid="print-section-<key>"`), four on SF1e/CPR sheets (1, 4, 5, 6);
  the paper select and both buttons stay; ticks remembered per character under
  `codex.print.sections.<characterId>` (`settings/printSections.ts`, the `printPaper.ts` shape); first time
  1+2+3. Four ui-smoke rows.
- Density: no page-count ceiling (`decisions.md §10`, §22); `print_golden.json` keeps each case's exact
  `pages` count, re-recorded on purpose only with before → after in the receipt.

## §6 Does not cover

Pixel positions (DEF-1): a look whose words land in the wrong cell passes the goldens; the PNGs and the
two gates are the control. SF1e/CPR looks (DEF-4). Any change to an existing engine value: SD-40 adds only the named rows and fields of
E2.6–E2.8 and E2.14 (`decisions.md §23.2`; G-1 as rewritten).
