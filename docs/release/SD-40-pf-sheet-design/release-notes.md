---
canonical: true
bundle_id: SD-40
slug: pf-sheet-design
tranche: 20
date: 2026-11-14
status: final
---

# Campaign Codex 0.20 — Pathfinder Sheet Design

The printed Pathfinder sheet, designed: a 12-column page grid, one Typst look per block, a page 1 the
operator signed as usable, and eight print sections chosen by checkbox. Starfinder and Cyberpunk RED print
exactly as in 0.19.

PR #406 merged the first part of SD-40 (Epics E0 to E2) to `develop` on 2026-11-13 at the operator's request.
This file covers the whole bundle, E0 to E5. Every figure is listed under "Figures" as `R-n`, with the
command that produced it, run from the repository root on 2026-11-14 at this file's commit.

## Summary

SD-40 splits Pathfinder print into three layers. **Data blocks** hold numbers only. **Looks** are one Typst
template per block. **Layouts** are ordered lists of blocks with a place on the page grid. 30 looks ship
(R-3). Page 1 was redesigned over three operator rounds and signed `usable: yes` on A4 and Letter (R-2).
Every page after page 1 now has a look too: abilities and feats, inventory, spellbook, bio, progression, math
and notes. The print dialog's three option boxes became eight section checkboxes, remembered per character.
The app gained the fields the new sheet prints: Languages, Swim and Fly, Conditional Bonuses, Resistances,
and Biography and Notes tabs. It lost hit-point tracking and the ⚙ button. 59 of the 61 kanban rows were
`complete` when this file was written. The other two are this card (E5.4) and E5.5, the graphify run and
the PR that carries this file (R-1).

## User-Visible Changes

- **Page 1 (E1, E2).** On a 12-column grid: a portrait box two rows tall with the known Languages under it,
  beside Character Name and Campaign; the Armor Class boxes with their breakdown filled from engine rows and
  Speed (Land, Swim, Fly); a left column holding one square block (Hit Points, Initiative, Encumbrance,
  Conditions), Weapons (with a damage total and range per weapon), Attack, Saving Throws, a fixed-size
  Conditional Bonuses box and Resistances; a right column holding Abilities over a one-column Skills list.
  A list too long for page 1 shrinks to the 8 pt floor, then continues at the top of page 2 under
  "<heading> (continued)". No data is dropped (E2.4).
- **Section pages (E2.23, E3).** Each section opens a new page under its title. Abilities and Feats lists
  feats, class features, actions, daily spells, custom entries and pets (E3.1, E3.3). Inventory has defense,
  gear (worn, carried, left behind) and money (E3.2). Spellbook comes next (E3.3). Bio has the details, the
  Biography, campaign notes and ruled lines (E3.4). Progression shows the Show Progression drawer level by
  level (E3.7). Math has every page-1 breakdown and the derivation lines (E3.5). Notes prints the Notes
  tab (E4.1).
- **The print dialog (E4.1, E4.2).** Eight checkboxes in box order: Primary (page 1), Abilities and Feats,
  Inventory, Spellbook, Bio, Progression, Math, Notes. The first-time ticks are Primary, Abilities and
  Feats, and Inventory. The ticks are remembered per character on this machine (`localStorage`
  `codex.print.sections.<characterId>`), not saved with the character. Starfinder and Cyberpunk RED read
  Primary, Spellbook, Bio and Math. The three old option boxes are gone.
- **Biography and Notes tabs (E3.6).** Both are saved in `bio.json`. The Notes text prints as its own
  section, which is off by default.
- **Languages field (E2.27).** It is saved with the bio and feeds the sheet's known languages. With nothing
  stored, the sheet shows Common.
- **Swim and Fly (E2.24), Conditional Bonuses (E2.25), Resistances (E2.26) in the app.** They read the same
  engine rows the print reads.
- **No hit-point tracking (E2.29).** The Hit Points box shows the maximum only. The two tracking commands
  (`adjust_character_hp`, `load_character_durability`) and the DurabilityPanel are removed (R-5). A save
  that still holds the old `hp.json` loads, and the file is ignored.
- **Options in a ☰ menu on every screen (E2.30).** The ⚙ button is removed.
- **Page counts.** Pathfinder's golden (Elowen, Wizard 5) on both papers, from the 0.19 tree to now: Primary
  + Abilities and Feats + Inventory 5 → 5, with Spellbook 6 → 6, with Math 12 → 11, sections 1 to 7 14 → 14;
  Spellbook alone is a new case at 1 page (R-6). Starfinder and Cyberpunk RED page counts are unchanged in
  every old case (R-6).

## Defects Fixed

- **Wrong or missing sheet numbers (E2.7, E2.15, E2.31, E2.32).** AC is 10 plus the parts the engine
  states. A custom item's AC bonus now enters both the AC row and its part. `hit_points.max` now counts
  Toughness and a bonded Toad familiar, and Toughness scales with Hit Dice as `max(3, HD)`.
- **Conditional notes from `print: false` rules (E2.6, E2.28).** Held rules the sheet used to drop, for
  example Vermin Focus on CMD, now print as conditional notes for HP, initiative, saves and AC.
- **Spell stat-block slots (E3.8, E3.9).** Range, Duration and Target read the caster's level. A Paladin's or
  Ranger's spells use the class's caster level, not its class level.
- **Page-1 Skills (E3.MC).** A rank with no panel total now prints its words, where it used to lose them.
  A table line now keeps its words on its own page (actions, gear, spells, custom).
- **Page-1 Campaign cell (E4.MC).** The cell stayed blank unless Bio was ticked. The dialog now sends the
  campaigns on every print.
- **14 stale ui-smoke rows (E0.2).** They were repaired to the current UI. None was deleted.
- **A fourth weapon line broke page 1 on Letter.** E2.MC found it and E2.4's overflow rule fixed it.

## Operational Notes

- **Version.** The repo value is `0.20.0` on the two desktop manifests (R-4).
  `docs/release/current-release.json` names this file, and `python3 tools/release/resolve_release_notes.py
  --repo-root .` prints its path.
- **Two copies of the notes.** PR #407 (`fix/sd40-release-notes`, still open when this was written) carries
  this file's early-merge version. This version is a later edit of the same file on `tranche-20`, so it
  replaces that one whichever PR merges first. The PR-time release-manifest check runs the publish lane's
  two notes commands (`resolve_release_notes.py`, then `validate-tranche-notes.py`), so a PR can no longer
  merge notes the publish would refuse.
- **Print engine.** As in 0.19, Typst is compiled in and there is no external binary at print time. The 30
  looks are compiled into the binary (`include_str!` in `print/typst.rs`). E5.1 found all 30 names in the
  release build.
- **Seeds.** The Pathfinder seed renders are SAME_EXCEPT_NAMED against C1 at each merge check. The named
  prose is E3.8's spell slots (G-1).

## Verification Evidence

- **E5.1, widest verify** (`artifacts/epic_5/E5.1_cycle_receipt.md`). `bash scripts/verify.sh --full
  --show-actuals -j 8` on `cc3cc92fde` gave `RESULT: PASS`, 53 stages, and 0 `test result: FAILED` lines.
  Root full: 6785 tests in 300 suites. Ingest full: 1859 in 177. Desktop: 1317. Frontend: 194 of 194 files.
  Clippy: 0 warnings in each workspace. A clean worktree built `Codex_0.20.0_amd64.deb` and the AppImage.
  The floors in `scripts/verify-baselines.env` were re-recorded at these values (R-7).
- **E5.2, operator gate** (`artifacts/epic_5/E5.2_cycle_receipt.md`). Five `usable: yes (operator,
  2026-11-14)` lines (R-2). They cover Aldric with the first-time sections, all seven sections, Bio alone
  and Math alone, on A4 and Letter, and Elowen's Spellbook alone and all six on A4.
- **E2.G round 3, operator sign-off, 2026-11-13.** Aldric's page 1 with seven languages, A4 and Letter,
  both `usable: yes` (R-2).
- **E5.3, final-acceptance scan** (`artifacts/epic_5/E5.3_cycle_receipt.md`). G-8 is clean, and the
  retrospective is `docs/retro/sd40-retrospective.md`.
- **Starfinder and Cyberpunk RED.** Their 16 golden PDFs from C1 are byte-identical at every merge check
  (E1.MC to E4.MC receipts).

## Known Issues

- **E5.5 had not run when this file was written.** It runs graphify and opens the PR that carries this
  file. Its receipt records the outcome.
- **The operator's E5.2 sign-off reads "I will look later".** The five `usable: yes` lines are the gate's
  evidence. They are not a printed-and-read judgment of every page.
- **Starfinder and Cyberpunk RED keep the 0.19 table-of-contents look** (DEF-4, FSR-C2). The grid and the
  looks are Pathfinder only.
- **Pixel placement is checked by eye, not by a test** (DEF-1). The goldens check words, headings in order,
  page counts and determinism, not where on the page a word lands.

## Does not cover

- A real update install of 0.20.x, Windows and macOS builds (CI publishes them), and launching the built
  `.deb` or AppImage. E5.1 built them and checked their contents.
- Only Aldric and Elowen were printed for the operator. The third seed (Brother Tomas) and the 39 characters
  of E4.MC's `.pcg` probe are covered by renders and tests, not by an operator's print.
- R-1 and R-6 move with later cards. They are pinned to the tree at this file's commit.

## Figures

- **R-1** kanban rows, rows `complete` (when this file was written, before E5.4's own row): **61 59**.
  `awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {n++; if ($5 ~ /^ complete *$/) c++} END{print n, c}' docs/release/SD-40-pf-sheet-design/kanban.md`
  gives `61 60` once E5.4's row is `complete`.
- **R-2** operator `usable: yes` lines: E2.G round 3 **2**, E5.2 **5**.
  `grep -c '^- Aldric .* — usable: yes' docs/release/SD-40-pf-sheet-design/artifacts/epic_2/E2.G_cycle_receipt.md`;
  `grep -c 'usable: yes' docs/release/SD-40-pf-sheet-design/artifacts/epic_5/E5.2_cycle_receipt.md`.
- **R-3** look templates: **30**. `ls apps/desktop/src-tauri/resources/print/looks/*.typ | awk 'END{print NR}'`.
- **R-4** desktop manifest versions: **0.20.0,0.20.0**. `grep -h -m1 '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'"' '{print $4}' | paste -sd,`.
- **R-5** registered Tauri commands: **116**, down from 118 at SD-39's tip `cd873a6d0d` (the two
  hit-point commands). `python3 -c "import re,sys;s=open('apps/desktop/src-tauri/src/main.rs').read();b=re.sub(r'//.*','',re.search(r'generate_handler!\[(.*?)\]',s,re.S).group(1));print(len([x for x in b.split(',') if x.strip()]))"`.
  For SD-39's tip, run the same parse on `git show cd873a6d0d:apps/desktop/src-tauri/src/main.rs`.
- **R-6** golden cases and page counts: **30** cases (3 systems × 2 papers × 5 section sets; 24 at SD-39).
  `python3 -c "import json;print(len(json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'))))"`.
  Per-case pages, before → after:
  `python3 -c "import json,subprocess;f='apps/desktop/src-tauri/src/print/golden/print_golden.json';a=json.loads(subprocess.run(['git','show','cd873a6d0d:'+f],capture_output=True,text=True).stdout);b=json.load(open(f));[print(k,a.get(k,{}).get('pages'),'->',b[k]['pages']) for k in b]"`.
  Only `pf1e-{a4,letter}-math` moves among the 24 old cases (12 → 11).
- **R-7** recorded floors: `BASELINE_ROOT_FULL_TESTS` 6785, `BASELINE_DESKTOP_TESTS` 1317,
  `BASELINE_FRONTEND_TEST_FILES` 194, with the last assignment winning.
  `python3 -c "import re;d={m.group(1):m.group(2) for l in open('scripts/verify-baselines.env') for m in [re.match(r'^([A-Z_]+)=(\S+)',l)] if m};print(d['BASELINE_ROOT_FULL_TESTS'],d['BASELINE_DESKTOP_TESTS'],d['BASELINE_FRONTEND_TEST_FILES'])"`.

## Update Eligibility

Set at publish by the release manifest (`tools/release/check_release_manifest.py`). The repo keeps the
version placeholder until then, and this document does not decide it. This bundle did not exercise the
update path. 0.20.x is the next tester publish after `tranche-20` reaches `develop`.
