---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready — every figure carries its command; measured on tranche-19 @ d4489b3557 on 2026-11-07 unless stated
---

# SD-40 Content Unit Inventory

A design bundle counts blocks, looks, layouts, pages and tests rather than book content. Every figure
has its command and its denominator. Figures marked **(E0.1)** are measured by that card and written
back here in its commit.

## §1 Version and package surfaces

| F | Figure | Value | Command |
|---|---|---|---|
| F-1 | Version surfaces C1 bumps to `0.20.` (the files SD-39's C1 bumped, less the pointer) | **14** | `git show --name-only --format= 49d598c13a \| awk 'NF' \| grep -vc current-release.json` |
| F-2 | C1 commit file count (F-1 + `current-release.json`) | **15** | `git show --name-only --format= <sha> \| awk 'NF' \| wc -l` |
| F-3 | Cards in this bundle | **38** (24 + E2.4 by E2.MC + E2.5 at E2.G round 1 + 12 by the round-1 replan, `decisions.md §23`) | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' docs/release/SD-40-pf-sheet-design/kanban.md \| awk 'END{print NR}'` |
| F-4 | Chassis files present | **16** (13 required + `technical-design.md`, `technical-requirements.md`, `kanban.md`) | `find docs/release/SD-40-pf-sheet-design -maxdepth 2 -name '*.md' \| awk 'END{print NR}'` |

## §2 The print module today (the before state)

| F | Figure | Value | Command |
|---|---|---|---|
| F-5 | Block builders (`pub fn`) under `print/blocks/`; block types | **16** builders; **15** types (Spells and Spellbook share one) | `grep -h '^pub fn ' apps/desktop/src-tauri/src/print/blocks/*.rs \| awk 'END{print NR}'`; the type list is `print/blocks/mod.rs`'s header |
| F-6 | Typst template lines (the whole look layer today) | **58** (`frame.typ` 27 + `blocks.typ` 31) | `cat apps/desktop/src-tauri/resources/print/*.typ \| awk 'END{print NR}'` |
| F-7 | Lines under `print/` + `resources/print/` | **4,226** | `find apps/desktop/src-tauri/src/print apps/desktop/src-tauri/resources/print -type f \| xargs wc -l \| tail -1` |
| F-8 | PF layout blocks in print order; option blocks | **13** + **3** | `python3 -c "import json;d=json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'));print(len(d['pf1e-a4-none']['headings']),len(d['pf1e-a4-all']['headings'])-len(d['pf1e-a4-none']['headings']))"` |
| F-9 | Golden cases; `#[test]` attributes under `print/` | **24**; **30** (the 24 goldens come from one macro) | `python3 -c "import json;print(len(json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'))))"`; `grep -rc '#\[test\]' apps/desktop/src-tauri/src/print \| awk -F: '{s+=$2} END{print s}'` |
| F-10 | Page counts today: Aldric full A4; Elowen basic A4; SF-Mystic-5 basic; cpr-netrunner-decked basic | **10**; **5**; **8**; **4** | `pdfinfo docs/release/SD-39-ui-print-corrections/artifacts/epic_9/E9.4/Aldric-Ironhand-PF1e-full-A4.pdf \| awk '/Pages/{print $2}'`; `python3 -c "import json;d=json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'));print(d['pf1e-a4-none']['pages'],d['sf1e-a4-none']['pages'],d['cpr-a4-none']['pages'])"` |
| F-11 | Bundled font faces; Typst version | **4**; **0.15.1** | `ls apps/desktop/src-tauri/resources/fonts/*.ttf \| awk 'END{print NR}'`; `grep -n '^typst' apps/desktop/src-tauri/Cargo.toml` |

## §3 The picture to aim at (PCGen, pinned oracle)

| F | Figure | Value | Command |
|---|---|---|---|
| F-12 | PCGen fantasy PDF output sheets; std / simple / spellbook variants | **36**; **10** / **9** / **5** | `ls "$PCGEN_REPO_DIR/outputsheets/d20/fantasy/pdf" \| awk 'END{print NR}'`; `ls … \| grep -c csheet_fantasy_std`; `… simple`; `… spellbook` |
| F-13 | PCGen std sheet page count for Aldric (the density reference, never a ceiling); simple; Elowen on the spellbook 3x5 sheet (one 3x5 card per page: a blank card, then one per known/prepared spell, cantrips auto-added) | **3**; **3**; **25** (E0.1, 2026-11-07; page 1 = sheet, page 2 = equipment/feats/features, page 3 = bio) | `pdfinfo docs/release/SD-40-pf-sheet-design/artifacts/epic_0/E0.1/aldric-pcgen-std.pdf \| awk '/Pages/{print $2}'`; same for `aldric-pcgen-simple.pdf`; same for `elowen-pcgen-spellbook-3x5.pdf` |
| F-14 | PCGen std sheet: template calls and selects in the `match="character"` body up to and including `protection` (the armor box) | **29** names, of which the page-1 sections are basics, abilities, armor_class, initiative, bab, encumbrance, skills, skillinfo, saving_throws, resistances, attack, weapons, protection (13; the rest are page templates, class-feature boxes and weapon sub-kinds). **E0.1 restated on the rendered picture:** `basics/bab` is inside an XML comment (`<!--> … -->`, body line 50) and renders nothing (BAB is a column of the `attack` rows); the 13 rendered page-1 sections are basics, abilities, hp_table (`select="."`), armor_class, initiative, resistances, encumbrance, skills, skillinfo, saving_throws, attack, weapons, protection (`artifacts/epic_0/E0.1/block-map.md`) | `awk '/template match="character"/{f=1} f' "$PCGEN_REPO_DIR/outputsheets/d20/fantasy/pdf/fantasy_master_common_blocks.xslt" \| grep -oE 'call-template name="[^"]+"\|apply-templates select="[^"]+"' \| sed -E 's/.*="([^"]+)"/\1/' \| awk '{print} /^protection$/{exit}' \| awk 'END{print NR}'` → 29 |
| F-15 | Local `.pcg` fixtures to model `aldric.pcg` on; those naming `CLASS:Fighter` | **13**; **13** | `ls scripts/oracle_harness/eqm-fixtures/*.pcg \| awk 'END{print NR}'`; `grep -l 'CLASS:Fighter' scripts/oracle_harness/eqm-fixtures/*.pcg \| awk 'END{print NR}'` |
| F-16 | Oracle pin | `7f818006e371188e5717fd18d74d18a420747fc6` | `grep PCGEN_ORACLE_SHA= scripts/pcgen-oracle-pin.env`; `bash scripts/fetch-pcgen-oracle.sh --check --quiet` exit 0 |

## §4 The UI vocabulary the print blocks must share

| F | Figure | Value | Command |
|---|---|---|---|
| F-17 | `SheetBoxId` members; `SquareBoxId` members | **13**; **4** | `awk '/export type SheetBoxId/,/;/' apps/desktop/src/characterHub/sheetLayout.ts \| grep -c "^  \| '"`; `grep -c "'hitPoints' \| 'initiative' \| 'encumbrance' \| 'conditions'" apps/desktop/src/characterHub/sheetLayout.ts` (≥ 1; the four names are on one line) |
| F-18 | `SHEET_TABS` entries | **8** (Weapons, Defense, Gear, Spells, Pets, Feats, Actions, Custom) | `grep -n 'export const SHEET_TABS' apps/desktop/src/characterHub/CharacterSheet.tsx` (line 544) |
| F-19 | Print dialog option boxes today (`PRINT_OPTION_BOXES` entries); `Paper` variants | **3**; **2** | `grep -c "^  { key: '" apps/desktop/src/characterHub/PrintDialog.tsx`; `grep -c '^    A4,\|^    Letter,' apps/desktop/src-tauri/src/print/model.rs` |

## §5 Gates and harness

| F | Figure | Value | Command |
|---|---|---|---|
| F-20 | `verify.sh` stages (full) | **53** | `bash scripts/verify.sh --list \| awk 'NR>1 && $2=="yes"' \| awk 'END{print NR}'` |
| F-21 | ui-smoke rows; stale red rows (SD-39 FSR-C2; owned by E0.2) | **173**; **14** | `python3 -c "import json;print(len(json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']))"`; `python3 -c "import json;print('\n'.join(r['id'] for r in json.load(open('docs/release/SD-39-ui-print-corrections/artifacts/epic_2/E2.4/full-suite-results.json')) if r['status']=='red'))" \| awk 'END{print NR}'` (the id list E0.2 repairs; SD-39 E9.5 FSR-C2 row) |
| F-22 | PDF tools on this box | `pdftoppm`, `pdftotext`, `pdfinfo` present; no viewer | `which pdftoppm pdftotext pdfinfo` |
| F-23 | PF starter seeds the harness prints | **2** (Aldric Fighter 3 `…0001`, Elowen Wizard 5 `…0002`) | `grep -n 'ALDRIC_ID\|ELOWEN_ID' apps/desktop/src-tauri/src/print/print_smoke.rs apps/desktop/src-tauri/src/print/print_golden.rs` |

## §6 Recorded figures (no page-count ceiling: `decisions.md §10`, §22)

| F | Figure | Value | Command |
|---|---|---|---|
| F-24 | Aldric Basic pages, A4 and Letter | **(recorded per receipt; no ceiling)** | `pdfinfo <dir>/aldric-<paper>-basic.pdf \| awk '/Pages/{print $2}'` after `PRINT_PROOF_OUT=<dir> cargo test print_proof` |
| F-25 | Elowen Basic pages (the golden PF character) | **(recorded per receipt; no ceiling)** | `python3 -c "import json;d=json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'));print(d['pf1e-a4-basic']['pages'],d['pf1e-letter-basic']['pages'])"` |
| F-26 | Looks at closure | **≥ 23** files (table + 7 E2.1 + skills + weapons + 4 E3.1 + 3 E3.2 + 7 E3.3) | `ls apps/desktop/src-tauri/resources/print/looks/*.typ \| awk 'END{print NR}'` |
| F-27 | Print sections (replace the 4 presets, `decisions.md §23.4`) | **6** (4 shown on SF1e/CPR) | after E4.1: `grep -c 'Section::' apps/desktop/src-tauri/src/print/layout/pf1e.rs` (≥ 6); 0 on `451333ee3e` |
| F-28 | Converted rules with a situational gate, by target (the E2.6 population) | Hp 15, Initiative 10, Fortitude 52, Reflex 58, Will 78, Ac 178 (of 73,363 rules in 49,768 files) | `python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r1/census.py` (output kept beside it, `census.txt`) |
| F-29 | Aspect prose entries E2.6 reads | SaveBonus 407, AllSaveBonus 3, Save Bonus 2, WillSaveBonus 3, RefSaveBonus 2, ModifyAC 6; CombatBonus 314 (not read, FSR-C8) | same command |
