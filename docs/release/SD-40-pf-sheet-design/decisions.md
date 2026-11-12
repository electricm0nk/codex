---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready — R1–R7 are the operator's rulings of 2026-11-07; "DEFAULT" rows are the author's and name how to revisit
---

# SD-40 Decisions

Every entry says who decided (OPERATOR ruling, verbatim where it matters, or author DEFAULT), what
enforces it, and how to revisit it. "R<n>" here is an operator ruling; `workflow-instruction.md §12`
uses "R<n>" for standing rules, told apart by file.

## §1 — Package location

**DEFAULT.** `docs/release/SD-40-pf-sheet-design/`. The overlay names `docs/stc/stc-<##>-<description>/`;
this repo has no `docs/stc/` (`ls docs/stc` → no such directory, 2026-11-07), every sibling lives under
`docs/release/`, and `scripts/architecture_truth_up.py` treats `^docs/release/` as architecture-relevant.
SD-38 and SD-39 recorded the same deviation. **Enforced by:** C0. **Revisit:** the operator says "move to
docs/stc"; one `git mv` before C1.

## §2 — Branch, board, version

**OPERATOR (R4) + DEFAULT.** Branch `tranche-20`, cut from `tranche-19` @ `d4489b3557` on 2026-11-07
(`git merge-base HEAD origin/tranche-19` → `d4489b3557…`), while SD-39 is still closing in
`.worktrees/sd39-main`; it catches up to `develop` after SD-39 merges (a rebase or merge by the lead,
logged in `progress.md`, before E5.1). Board: `./kanban.md` + `./progress.md`. Version `0.20.0` stamped by
C1's **single commit** on the 14 surfaces SD-39's C1 bumped
(`git show --name-only --format= 49d598c13a | awk 'NF' | grep -vc current-release.json` → 14; F-1 in
`content-unit-inventory.md`) plus `docs/release/current-release.json` → `{"tranche": 20, "notes_path":
"docs/release/SD-40-pf-sheet-design/release-notes.md"}`. The lockfile check reads only the `codex-desktop`
entry (SD-39 `decisions.md §25`); `apps/desktop/package-lock.json`'s top `version` is `0.14.0` and no
prior C1 touched it, so neither does this one (noted, not fixed: it is not a version surface the release
reads). `release-notes.md` ships an interim body with the sentinel `SD-40 INTERIM NOTES`; E5.4 replaces
it; E5.5 refuses the PR while the sentinel is present. **Enforced by:** C1's receipt; `python3
tools/release/resolve_release_notes.py --repo-root .` exit 0. **Revisit:** the tranche digit moves only at
a new bare cut.

## §3 — Operator rulings on record (2026-11-07)

The brief is `artifacts/cycle_0/operator-brief-2026-11-07.md`. The rulings:

| R | Question | Operator's choice |
|---|---|---|
| R1 | Print architecture | **Three layers:** data blocks (numbers only; mostly exist) → block looks (each block its own Typst template, tweakable one at a time) → layouts (an ordered list of blocks + their place on a page grid). Print styles are layouts, not code; adding a style = adding a list. Block names match the app's UI blocks (`sheetLayout.ts`) so screen and paper share one vocabulary |
| R2 | System scope | **Pathfinder 1e only** for the design work ("getting just the one we use the most working well and looking great"). The architecture is system-neutral; **SF1e and CPR output must not change** (golden text byte-identical, or the delta named and justified) |
| R3 | Visual model | **Lean toward PCGen's output sheets** (std / simple / spell-list-only / spellbook 3x5 / bio / notes) over Paizo's, as a **picture to aim at only**. "It's not fine to include 1 line of java in our live code": no PCGen template, XSL or token enters the app |
| R4 | Branch and reference | `tranche-20` from `tranche-19` now, catch up to `develop` after SD-39 merges. **Yes** to a tool-side PCGen reference render of Aldric on PCGen's standard sheet (simple and spellbook if cheap), under this package's `artifacts/`, never shipped. **Stop only after step 1**, then unattended |
| R5 | Step 1 = the operator gate | The page grid + new looks for the first 4–5 PF page-1 blocks: abilities, combat (HP, AC/touch/flat-footed, initiative, BAB, CMB/CMD, speed), saves, skills, weapons. Render Aldric's page 1, stop, the operator prints it and writes `usable: yes` (or defects). Later steps bounded: remaining blocks, presets, density, Letter, closure. No swinging for the fences |
| R6 | Standing rules | Paper-sheet generator, not a video game; print the rule, compute only numbers that land on the sheet, dice literal; **write-in space** where a player writes at the table (current HP, ammo, spells used, notes); orchestrator never edits shipping code; every `agent()` sets a model; one writer per tree; never `git stash`; batch big; one-pass verify |
| R7 | Look at your own output | Design agents rasterize the PDF (`pdftoppm -r 60 -png`), read the PNGs, compare against the PCGen reference picture, iterate before claiming done; in the per-cycle procedure for every look/layout card; PNGs kept in artifacts. Text goldens stay the regression net; add a density/page-count check with a target from the reference render, justified |

## §4 — The three layers in code (R1; DEFAULT on the shapes)

> **Amended by §23.2 (2026-11-11):** a print *section* (not a preset) is one `&[Placed]` list.

**Data blocks** stay what they are: `print/blocks/*.rs` builders over `SheetSources`, text only, no number
computed (NR-2). **Looks:** `SheetBlock` gains `id: &'static str` (the shared vocabulary, §5) and `look:
Look`, an enum whose variants name a Typst template file `resources/print/looks/<look>.typ`; `typst.rs`
holds one table `LOOKS: &[(&str, &str)]` of `include_str!` entries, and `blocks.typ` becomes the
dispatcher `sheet-block(b)` → `looks.at(b.look)(b)`. The variant `Table` is today's generic table, so a
block with no look of its own prints exactly as it does now. **Layouts:** `SheetDocument` gains
`columns: u8` (1 = today's full-width flow; the PF layout uses 12) and each block a `place: Place { x: u8,
span: u8 }` on that grid; `new_page` stays the page break. The frame renders a Typst `grid` with
`grid.cell(x:, colspan:)` when `columns > 1` and the old `for b in doc.blocks` loop when `columns == 1`,
so SF1e and CPR go through the loop they go through today (R2). A **layout** is then a `&[Placed]` list
in the system's layout file (`layout/pf1e.rs`): block id → builder, place, page break; a preset (§9) is
one such list. **Enforced by:** E1.1 (model + dispatcher + grid), E1.MC. **Revisit:** a look that needs
more than a block's rows (a two-block composite) is a discovered card, never a Typst-side join of blocks.

## §5 — One vocabulary for screen and paper (R1)

> **Amended by §23.2 (2026-11-11):** page 1 adds `portrait`, `nameCampaign`, `squareBlock`, `encumbrance`, `conditions`, `languages`; `characterDetails` moves to the Bio page.

**DEFAULT on the mechanism.** `src/print/golden/block_vocabulary.json` (new, E1.1) lists every PF print
block id with its UI source: a `SheetBoxId` (`sheetLayout.ts`: `portrait, levelXp, classPanel,
characterDetails, showProgression, squareBlock, combatStrip, armorClass, attack, speed, saves, abilities,
skills`; 13, `awk '/export type SheetBoxId/,/;/' … | grep -c "^  | '"`), a `SquareBoxId` (`hitPoints,
initiative, encumbrance, conditions`), a `SHEET_TABS` entry lower-cased (`weapons, defense, gear, spells,
pets, feats, actions, custom`; 8) or a print-only id the fixture allow-lists with a reason (`identity`
→ renamed `characterDetails`; `classFeatures`, `money`, `spellbook`, `math`, `bioNotes`). A Rust test
(`print_pf_blocks`) asserts every PF block id is in the fixture, and `sheetLayout.test.ts` asserts every
fixture id with a UI source exists in the registry or the tab list. The same shape as SD-39's
`class_feature_labels.json`. **Enforced by:** E1.1, E1.MC. **Revisit:** a new UI box adds a fixture line
in the card that adds the box.

## §6 — The PCGen reference render (R3, R4)

**OPERATOR.** E0.1 runs PCGen's BatchExporter headlessly the way `scripts/oracle_harness/
charbuild_remainder_run_one.sh` already does (`java … -cp "lib/*" pcgen.system.Main -c <pcg> -E <sheet>
-o <out> -s <settings>` from `$PCGEN_REPO_DIR/build/install/pcgen`; the bare jar has no classpath:
`java -jar build/libs/pcgen-6.09.08.RC1.jar --help` → `NoClassDefFoundError`), over a hand-authored
`artifacts/epic_0/E0.1/aldric.pcg` (Human Fighter 3 with Aldric's scores, feats, skills and kit, modelled
on `scripts/oracle_harness/eqm-fixtures/*.pcg`), with `-E $PCGEN_REPO_DIR/outputsheets/d20/fantasy/pdf/
csheet_fantasy_std_blackandwhite.xslt` (std), `csheet_fantasy_simple_blackandwhite.xslt` (simple) and,
for Elowen if a `.pcg` is cheap, `csheet_fantasy_spellbook_3x5.xsl`. Outputs (PDF + 60 dpi PNG per page)
live under `artifacts/epic_0/E0.1/`, never under `apps/`, `src/`, `crates/`, `data/` or `resources/`
(the residue check in E0.1's acceptance). **No PCGen artefact is read by any Rust or TS test.**
**Enforced by:** E0.1, G-7, `pcgen-residue-gate`. **Revisit:** SD-h in §13.1 when the headless run fails.

## §7 — Step 1, the operator gate (R5)

> **Amended by §23 (2026-11-11):** round 1 is recorded; E2.G round 2 judges the redesigned page 1 after E2.MC2 (`kanban.md`).

**OPERATOR.** E2.1–E2.3 build the page-1 looks and E2.3 assembles page 1 on the grid; E2.MC reviews;
**E2.G** renders Aldric's basic sheet on A4 and Letter, rasterizes it, commits the PDFs and PNGs under
`artifacts/epic_2/E2.G/`, returns `partial`, and the run ends with a resume receipt. The operator prints
page 1 and writes `usable: yes` in the receipt, or defects; defects become discovered E2 cards
(§14) and E2.G re-runs. The run resumes with `START=E2.G` (or the first discovered card). **Enforced
by:** G-4; `workflow-instruction.md §2.4` item 11. **Revisit:** none; this is the one wait before closure.

## §8 — The Combat block is split (R1, R5; DEFAULT)

Today one Combat block holds HP, AC/touch/flat-footed, BAB, melee, CMB, CMD, initiative, the three saves,
damage reduction and speed (`blocks/core.rs` `combat`, `layout/pf1e.rs` `combat`). The UI shows them as
`hitPoints`, `initiative`, `armorClass`, `attack`, `saves`, `speed` boxes. E2.1 splits the data block
along the UI's ids so each has its own look; the values and their derivation rows are unchanged
(`print_smoke` and `print_pf_blocks` re-pin them under the new headings; `print_golden.json` PF headings
re-recorded, listed in the receipt). SF1e and CPR build their own blocks and are untouched. **Enforced
by:** E2.1. **Revisit:** none.

## §9 — Presets are layouts; the dialog offers presets (R1, R5; DEFAULT on semantics)

> **SUPERSEDED by §23.4 (operator, 2026-11-11):** six section checkboxes replace the four presets; `PrintPreset` is never built. The text below is the 2026-11-07 record.

**OPERATOR on the names:** Basic, Basic + Bio, Full with the math, Spellbook. **OPERATOR on semantics**
(answer "2: A", 2026-11-07, §22, confirming the author's design below):
`PrintPreset { Basic, BasicBio, Full, Spellbook }` on `PrintRequest`, replacing the three booleans
(`spellbook`, `math`, `bio`). Basic = the sheet; Basic + Bio = the sheet + bio and campaign notes; Full =
sheet + spellbook + show the math + bio; Spellbook = the spellbook pages alone (a booklet; PCGen's
spellbook outputs are spells-only too). For PF each preset is a `&[Placed]` list in `layout/pf1e.rs`. For
SF1e and CPR, `layout/mod.rs with_options` derives (spellbook, math, bio) from the preset so their layout
files do not change: Basic → none, Basic + Bio → bio, Full → all three; Spellbook → their spellbook block
alone. Consequence for the golden set: the old `math`-only and `spellbook`-with-sheet cases have no
preset; the 24 cases become `{pf1e,sf1e,cpr} × {a4,letter} × {basic, basicBio, full, spellbook}` = 24,
with SF/CPR `basic` and `full` text byte-identical to the old `none` and `all` (G-2), `basicBio` new, and
`spellbook` now spells-only (named delta). E4.1 lands presets with the booleans still accepted and mapped
(absent preset: none → Basic, bio only → BasicBio, spellbook only → Spellbook, otherwise Full) so the app
keeps printing between E4.1 and E4.2; E4.2 removes the booleans from both DTOs. **Enforced by:** E4.1,
E4.2, E4.MC, G-2, G-5. **Revisit:** resolved by the operator (§22: Spellbook prints spell pages only; Full prints everything);
a later change of mind is one list edit.
**Not taken:** keeping the checkboxes beside the presets (two ways to say one thing).

## §10 — Density and Letter (R5, R7; OPERATOR answer "3: No hard limits", 2026-11-07, §22)

**No page-count ceiling.** No golden case, test or gate asserts a maximum page count; there is no
`pages_max`. Page counts are recorded per case in each receipt (before → after, with the reason) and the
operator judges density by eye at E2.G and E5.2. E0.1 records the PCGen std and simple page counts for
Aldric (`pdfinfo <pdf> | awk '/Pages/{print $2}'`) here and in `content-unit-inventory.md` F-13 as the
reference figure a look card compares its own output against, never as a limit. **E0.1 figures (2026-11-07):** Aldric on PCGen std = **3** pages, on simple = **3** pages
(`pdfinfo artifacts/epic_0/E0.1/aldric-pcgen-std.pdf | awk '/Pages/{print $2}'`, same for
`aldric-pcgen-simple.pdf`); PCGen page 1 holds the whole combat sheet incl. weapons and armor, page 2
equipment, feats and features, page 3 the bio. Reference, not a ceiling (§22). The exact `pages` count
in `print_golden.json` stays a regression pin that re-records on purpose only (SD-f). **Letter first:**
page 1 is designed to fit US Letter's printable height (11 in − margins); A4 is taller and gets the
slack, so one design serves both papers (`Paper::Letter` already exists: `model.rs`). **Enforced by:**
E0.1 (figures), E2.3 (page 1 on Letter), every look card's receipt (page counts before → after), the
operator's eye at E2.G and E5.2, G-6. **Revisit:** the operator names a limit; no card invents one.

## §11 — Look rules (R6; DEFAULT)

> **Amended by §23.2 (2026-11-11):** page 1 prints totals only; no temp-ability or misc write-in cells; the HP current area is big; notes lists end with two blank lines.

Black and white only, fills no darker than `luma(230)` (PCGen's `_blackandwhite` variants are the
picture; printers); the bundled Liberation Sans faces only (no new font in SD-40: determinism, bundle
size; `resources/fonts/*.ttf` → 4); body text never below 8 pt; every number the sheet prints is a cell
the engine filled; **write-in space** is a ruled blank, never a computed value: current HP and nonlethal
damage (hitPoints), temp score / temp modifier (abilities), ammunition (weapons), spells used per level
(spells), notes lines under feats and gear, conditions (conditions). An empty page-1 block prints its
look with blank cells (a paper sheet has the box whether or not the character fills it); a block after
page 1 with nothing to print prints its heading and two ruled lines. Dice stay literal (`1d8`). No TSX
and no Typst template re-derives a number (NR-2). **Enforced by:** every look card's receipt; E2.MC,
E3.MC. **Revisit:** a serif face for headings is FSR-C3.

## §12 — Model tiering

Per `~/.claude/CLAUDE.md` (2026-09-29): looks, layouts, the model, every merge check → **Opus 5.5**
(`model: 'opus'`; design judgement); well-specified mechanical work, every long wait and the two gate
cards → **Sonnet 5.5**; housekeeping → **Haiku 4.5**. Every `agent()` sets `model`
(`awk '/agent\(/ && !/model:/' <script>` prints nothing). Planning disclosure: this package was written
by Fable 5.1.

## §13 — Run protocol (UNATTENDED after launch)

The operator said: stop only after step 1. The standing autonomous rules apply (memory: "operator away —
autonomous mode", "run to 100% DoD without asking"):

1. Clear a blocker first; if only an operator ruling can clear it, mark **only that card**
   `blocked-escalated` with the exact ruling under `progress.md ## Open blockers`; continue every
   independent card.
2. Never open the PR while any card is open, partial or blocked-escalated.
3. A choice inside scope with a safe default (§13.1): take it, log it, continue. No `AskUserQuestion`.
4. **Report after each epic** (the merge-check card writes the Epic report block in `progress.md`).
5. **Two planned waits:** E2.G (page 1) and E5.2 (the full set). The run proceeds past each only on the
   operator's `usable: yes` lines. Defects found there are discovered cards (§14), not blockers to argue with.

### §13.1 Safe defaults (taste questions never stop a card)

| # | Decision point | Safe default |
|---|---|---|
| SD-a | A look's proportions, rule weights, box sizes | Copy the PCGen reference picture's proportions (`artifacts/epic_0/E0.1/*.png`); log the choice in the receipt's "Looked at" row; never ask |
| SD-b | A block overflows its grid cell or a page-1 list is too long for page 1 | **Rewritten by §23.2 (2026-11-11):** no page-1 block leaves page 1; the list shrinks to the 8 pt floor, then prints what fits and continues at the top of page 2 under "<heading> (continued)"; never drop data (E2.4) |
| SD-c | A block has nothing to print | Page 1: the look with blank cells; later pages: the heading and two ruled lines (§11) |
| SD-d | Colour | None; black, greys ≥ `luma(230)` for fills |
| SD-e | Typst raises a warning | Fix it before claiming; a warning is NR-4's failure |
| SD-f | A PF golden page count moves | Re-record `print_golden.json` with before → after in the receipt (no ceiling, §10) |
| SD-g | SF1e or CPR golden text or page count differs from C1's | A defect; fix it; never re-baseline (G-2) |
| SD-h | The PCGen headless render fails on the hand-authored `.pcg` | Use `$PCGEN_REPO_DIR/code/testsuite/PCGfiles/35e_L10(fighter-human).pcg` or `pf_Paladin.pcg` for the picture, say so in the receipt; if PCGen will not run at all, E0.1 is `blocked-escalated` naming the exception |
| SD-i | Usage-limit stop | Stop dispatching, write a resume receipt (§13.3) |
| SD-j | No CI on `tranche-20` pushes | CI evidence = the closure PR's `pr-tests`, awaited by E5.5 |
| SD-k | Rebase conflict | `kanban.md`: take upstream, re-apply only your own row, run the duplicate-ID check; `progress.md` append-only sections: keep both; any other file: abort, `blocked-escalated` |
| SD-l | No PDF viewer on this box | The PNGs are the look; `pdftotext -layout` is the text; the operator's printer is the final judge at E2.G and E5.2 |
| SD-m | A write-in area's size | One ruled line of 0.5 cm per entry; current-HP area at least 3 × 3 cm on Letter (§23.2; was 2 × 2 cm), Nonlethal box small; copy the reference where it has one |
| SD-n | A data block the operator named is missing a value (e.g. ammunition count) | Print the write-in blank; never add a derivation; a missing engine row is a `retro.py note`, not a card, unless it lands on the sheet as a number today in the UI |
| SD-o | `tranche-19` merges to `develop` mid-run | The lead rebases `tranche-20` onto `develop` between epics (never mid-card), re-runs G-2, logs it in `progress.md`; SD-o fires at most once |
| SD-p | PNG artefacts grow large | 60 dpi only, PNG, at most 12 pages per card and 2 MB per card directory (`du -sk`); more goes to a note with the command to regenerate |

### §13.2 Watchdog — one long Workflow run

One long run with a recorded run id; recovery starts a **new** run with `START=<card>`; never resume a
run with parallel lanes. No cron. The run ends cleanly at E2.G and at E5.2 (resume receipts).

### §13.3 Quota stop rule

Stop dispatching when any `agent()` result matches `/usage limit|rate limit|quota|limit reached/i`, or
the harness shows a weekly reading ≥ 85%. Lanes in flight finish; write a resume receipt. Never
downgrade a model.

### §13.4 Crash-resume recipe

Keep every dirty worktree; read the run id from `progress.md` and `journal.jsonl`; start a new run with
`START=` the first unrun card; the prefix must be byte-identical to `artifacts/cycle_0/sd40-workflow.js`;
log a `retro.py incident`.

### §13.5 Environment guards

Carried verbatim into `workflow-instruction.md §2.1` and `§6`.

## §14 — Discovered cards

A defect found mid-run that is in the DoD becomes a card appended to `kanban.md` and `epic-breakdown.md`
by the card that found it (with the row-count constant in G-8 and `kanban.md` re-counted in the same
commit, R18), never a note. The operator's gate defects are discovered cards in the gate's epic.

## §15 — Closure order (STANDING, operator-pinned 2026-09-14)

All cards `complete` → retrospective written and cited → full worktree/branch sweep → **release notes**
→ **architecture truth-up + claims critic** → **graphify LAST** over a clean tree at `origin/tranche-20`
→ **PR as the final action**. The operator merges.

## §16 — Seed and golden deltas in SD-40 (DEFAULT)

> **Amended by §23.2 (2026-11-11):** E2.6–E2.8 add named fields and rows to the load response; no existing field or row may change value (G-1 as rewritten).

The PF render-hash seeds (`pf_seed_render_hash.rs`) are **unchanged** by every card: SD-40 changes no
DTO the screen reads and no engine value (`diff <base>/sha256.txt <head>/sha256.txt` empty, G-1). SF and
CPR seed lines are unchanged everywhere. The **print goldens** are where SD-40 moves: PF cases move at
E1.1 (grid), E2.1 (Combat split), E2.3 (page 1), E3.x, E4.1 (presets), each receipt listing page counts
before → after and the heading list; SF and CPR cases move only at E4.1 (preset renaming, G-2) and only
in the ways §9 names. **Enforced by:** G-1, G-2; R7 (per-system status) in every receipt.

## §17 — Paper-sheet rule (STANDING, 2026-09-07)

Compute only values that land on the sheet as a number; print everything else; dice stay literal. "Show
the math" prints the engine's existing explanations rows, never a new derivation engine. A look may
format (sign, join, column) but never compute.

## §18 — No PCGen in live code (STANDING, 2026-09-07; R3)

`src/`, `apps/desktop/` and `apps/desktop/src-tauri/` hold no PCGen template, XSL, token or Java. The
reference render lives under this package's `artifacts/` and is read by humans and by design agents'
eyes only. `pcgen-residue-gate` in `scripts/verify.sh` and E0.1's residue check are the controls.

## §19 — Fence exits

A lane that needs a file outside its FILES YOU OWN returns `declined` with `ownedBy:<card>` unless that
file's owning epic merge check is `complete`; then it edits under its own criterion and re-runs that
epic's gate, naming both in the receipt.

## §20 — Live peer work

`.worktrees/sd39-main` (branch `tranche-19`) is SD-39's closure tree, owned by another session; SD-40
never touches it (SD-f of SD-39 carried: a worktree another session may own is not touched, only logged).
`git worktree list` on 2026-11-07: the main checkout on `develop` @ `2b14020bc7`, `sd39-main` and
`sd40-main` both at `d4489b3557`.

## §21 — Pre-launch corrections

The lead re-reads this package against the repo before dispatch and records each corrected claim here
in the SD-39 §25 shape (package said | code shows | effect). Rows the author already corrected while
writing:

| Brief said | Code shows | Effect |
|---|---|---|
| "Letter paper" is a later step | `Paper::Letter` already exists (`model.rs`), the dialog offers it and 12 of 24 goldens are Letter | Letter is a design constraint (§10 Letter-first), not a feature card |
| "15 block types" (SD-39 F-23) | 16 block builders (`grep -h '^pub fn ' apps/desktop/src-tauri/src/print/blocks/*.rs \| awk 'END{print NR}'`), Spells and Spellbook sharing a type | F-5 states both figures with their predicates |
| The bare PCGen jar renders sheets | `java -jar build/libs/pcgen-6.09.08.RC1.jar --help` → `NoClassDefFoundError`; the installed distribution (`build/install/pcgen`, 44 jars, FOP 2.11 among them) runs: `./pcgen --help` prints the `-c/-E/-o/-s` options | §6 names the installed launcher and the harness script that already uses it |
| A version grep on `0\.19\.` names the surfaces | `apps/desktop/package-lock.json` matches on `"node": "^20.19.0"` engine lines (3) and its own `version` is `0.14.0` | C1's set is SD-39's 14 files by `git show`, not a grep; the grep excludes lockfiles |
| §4 / E1.1: "`SheetDocument` gains `columns: u8`" with SF1e and CPR layouts untouched (E1.1 found) | `sf1e.rs:180` and `cpr.rs:415` build `SheetDocument` by struct literal (`git grep -n 'SheetDocument {' apps/desktop/src-tauri/src/print/layout/`); a new field edits both, which G-2 forbids | `columns` is `SheetDocument::columns()`: 1 when every block's `place` is the full width (today's loop; SF1e and CPR), else 12; `typst.rs` sends it to the frame as `doc.columns`. `typeset` refuses a place past the grid or out of row order (`check_places`) |
| E1.1 FILES YOU OWN is closed (E1.1 found) | `tests/packaged_resources.rs` asserts `typst.rs` compiles in exactly the `.typ` files of `resources/print/` and counts 6 | E1.1 owns it too: it also scans `resources/print/looks/` and counts 6 + the looks |
| E0.2 / §2.1 FRONTEND: `npm run ui-smoke` runs as written in a fresh side-lane worktree (E0.2 found) | `apps/desktop/src-tauri/resources/corpus_bundle/` is gitignored and generated; in a fresh `git worktree add` tree it is empty, the dev build copies that empty dir to `$CARGO_TARGET_DIR/debug/data/corpus`, and every Create row fails `No race could be read from the corpus ... book directory not found` (E0.2's first RED run) | Before the first `npm run ui-smoke` in a fresh tree, run `node scripts/gen-corpus-bundle.mjs` from the tree root (`files_copied=14029` on 2026-11-07); applies to E0.2, E4.2 and E4.MC |

## §22 — Operator answers 2026-11-07

The operator answered the package's three open questions (`README.md §6` rows 9, 5, 6) on 2026-11-07:

| # | Question | Answer | Effect in this package |
|---|---|---|---|
| 1 | The 14 stale red ui-smoke rows (FSR-C1; SD-39 FSR-C2; retro change 5) | **"1: B"**: add one small card in SD-40 that owns them | Card **E0.2** "Repair the 14 stale ui-smoke rows" (`opus`), side lane `smoke` parallel to E1; it writes `apps/desktop/scripts/ui-smoke/spec.json`, the regenerated `docs/testing/ui-smoke-inventory.md` and whatever the rows need (its FILES YOU OWN); **serial edge E0.2 → E4.2**, which also writes `spec.json` (`kanban.md` E4.2 `Depends on`; R24). Each row is repaired to the current UI or rewritten to match it, never deleted to make the count pass. FSR-C1 → Committed COM-9; DoD 11; G-9; NR-14. Cards 23 → 24 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 24) |
| 2 | What the Spellbook preset prints | **"2: A"**: Spellbook prints spell pages only; Full prints everything | §9's design stands as written; no card changes |
| 3 | Density ceilings (`pages_max`) | **"3: No hard limits"** | §10 rewritten: no page-count ceiling as a test or gate; page counts are recorded per case in each receipt (before → after) and the operator judges density by eye at E2.G and E5.2. Letter-first design kept. NR-10, G-6, E0.1, E4.1, E4.MC, SD-b, SD-f, R-5, R-6, DoD 6 rewritten to match |

**Enforced by:** `kanban.md` (E0.2 row; E4.2 depends on E0.2); E0.2's acceptance and G-9; G-6 and NR-10
as rewritten. **Revisit:** the operator.

## §23 — Operator at E2.G, round 1 (2026-11-11): page 1 redesigned; checkboxes replace presets

**OPERATOR.** The operator looked at Aldric's page 1 (`artifacts/epic_2/E2.G/aldric-{a4,letter}-none.pdf`)
and gave the list below (recorded by the lead in its scratchpad as `e2g-round1-list.md`, "collect, do not
start"; quoted here verbatim so the package holds it). The first remark of the day was **"We need to include
a spot for the portrait on the top left corner."** (item 1). The round-1 verdict in
`artifacts/epic_2/E2.G_cycle_receipt.md` stays as written; the round-2 sign-off lines stay blank.

### §23.1 The operator's words, item by item

1. **Portrait.** "Portrait box, top-left of page 1. (Already carded E2.5: prints saved portrait.png, else
   empty framed box with "Portrait" caption.)"
2. **Name and campaign.** "Right of the portrait: "Character Name" and "Campaign" labeled fields, like the
   app's IdentityPanel (CharacterSheet.tsx:402). REPLACES the big name title at the top of page 1 (name
   prints once). No campaign -> blank write-in line under the "Campaign" label (not a dash)."
3. **Page 1 = the most-used, densest page.**
   - a. "ONE block: Hit Points + Initiative + Encumbrance + Conditions (like the app's 'square' group)."
     "HP: max HP printed, BIG empty current-HP area for pencil, small Nonlethal box." "Initiative: total
     modifier only (no Dex + misc breakdown)." "Encumbrance: Light/Medium/Heavy limits, total weight
     carried, remaining lb, gear value (all the app's lines)." "Conditions: write-in box; active conditions
     from the app printed in it." "SPECIAL NOTES for these values: print every conditional bonus the rules
     data states for HP / initiative / encumbrance (feat, class feature, item; condition in words, e.g.
     gunslinger +2 init while grit remains, dueling weapon), the way skills' "Not added to a total" lines
     work today (print/blocks/skills.rs, feat_skill_bonuses.situational) — then a few blank lines for the
     player. (Bigger: engine must gather these for init/HP/encumbrance.)"
   - b. "Abilities: score + modifier only. No temp score/temp modifier boxes; leave white space beside for
     pencil."
   - c. "Skills: ONE column, all skills, one line each, NO wraps. Each line = skill name + total only (no
     class-skill box, no * marks, no ability/ranks/misc, no temp boxes). Shorten only Knowledge names that
     would wrap ("Know. (Dungeoneering)")."
4. **"GENERAL RULE: page 1 shows TOTALS ONLY.** Every breakdown ("the math") moves to supplementary pages
   printed only with the 'show the math' print option. For normal play the math is a distraction and must
   not take page-1 space."
   - "Saves: Fort/Ref/Will totals only (no Base/Ability/misc boxes). PLUS conditional save bonuses printed
     as notes (e.g. Dwarf +2 vs poison, vs spells/SLAs; condition in words), then a blank line or two."
   - "Weapons: attack total, damage (e.g. 1d8+4), crit, range; no Str/Enh/Atk-part columns."
   - "Initiative, skills, abilities: as item 3 (totals only)."
   - "EXCEPTION — Armor Class: keep the full breakdown on page 1 AND fill it in completely (Armor, Shield,
     Dex, Size, Natural, Deflection, Misc all printed with real values; today those cells print blank,
     E2.G receipt "no engine rows, SD-n"). Reason: AC changes a lot at the table (no armor at dinner,
     dropped shield) so the player needs every part."
5. **Sections on their own pages + print dialog = CHECKBOXES** "(replaces decisions.md §9 presets
   Basic/BasicBio/Full/Spellbook; reverses §9's "Not taken: checkboxes")."
   - "Checkboxes: (1) Main sheet (page 1), (2) Feats, class features & actions, (3) Inventory, (4)
     Spellbook, (5) Bio, (6) Show the math. Paper size (A4/Letter) stays a separate choice."
   - "The app REMEMBERS the last ticks per character; first time: 1, 2, 3 ticked. Reason (operator): bio
     rarely reprinted, inventory every session — "let people choose"."
   - "Inventory: its own page(s), starts on a new page."
   - "Spellbook: its own thing (own page(s)), spells only (as decided 2026-11-07 "2: A")."
   - "Bio: its own page(s). Top = all the app's left-column details: Alignment, Deity, Sex, Age, Height,
     Weight, Hair, Eyes, Size, Vision (+ race/class). Then the bio/backstory text from the app, then blank
     ruled note lines to the end of the page."
   - "Page 1 has NO identity block (no race/class/size/vision); only portrait + Character Name + Campaign
     at the top (item 2)."
6. **Languages.** "Languages on page 1: the character's known languages as one comma list (wraps only if
   long), plus one blank line to write a new one in." Sub-answer: **"commas are fine"**.

### §23.2 What this supersedes

| Superseded | By | Effect |
|---|---|---|
| §9 (presets Basic / Basic + Bio / Full / Spellbook; `PrintPreset`) and its **"Not taken: checkboxes"** | item 5 | Six section checkboxes (§23.4). `PrintPreset` is never built. `forward-scope-register.md §6`'s "checkboxes beside presets" exclusion is withdrawn |
| `scope-draft.md §3` non-goal "portrait image on the sheet" | item 1 | Portrait box on page 1 (E2.5). Pages 2+ still print no portrait |
| `scope-draft.md §2` DoD 3 (page 1 blocks), DoD 5 (presets), DoD 9 (final look per preset) | items 3–6 | Rewritten in `scope-draft.md §2` |
| §4 "a preset (§9) is one such list" | item 5 | A **section** is one such `&[Placed]` list; the request picks sections |
| §5 vocabulary: `characterDetails` is a page-1 block | item 5 | `characterDetails` prints on the Bio page; page 1 adds `portrait` (SheetBoxId), `nameCampaign` (print-only: the IdentityPanel, `CharacterSheet.tsx:402`), `squareBlock` (SheetBoxId; holds `hitPoints`, `initiative`, `encumbrance`, `conditions`) and `languages` (print-only: the Skills panel's Languages line, `CharacterSheet.tsx:526`) |
| §7 E2.G = Aldric's step-1 page 1 as E2.3 built it | items 1–6 | E2.G **round 2** judges the new page 1 (Aldric, A4 + Letter) after E2.MC2 |
| §11 write-in list: temp score / temp modifier (abilities); misc on initiative, saves, skills; HP box 2 × 2 cm (SD-m) | items 3a, 3b, 4 | Abilities: white space beside score + modifier, no boxes. Initiative, saves, skills: no misc cells. HP: a **big** current-HP area (at least 3 × 3 cm on Letter, SD-m rewritten) and a small Nonlethal box. Conditions: a write-in box holding the app's active conditions. Notes: two blank ruled lines after each notes list |
| §13.1 SD-b ("move the lowest-priority page-1 block (weapons) to page 2") | items 3, 4 | **New SD-b:** no page-1 block leaves page 1. A list too long for its page-1 cell (weapon lines, a notes list, the skills' "Not added to a total" lines) shrinks to the 8 pt floor, then prints what fits and continues at the top of page 2 under "<heading> (continued)". Never drop a line. Owned by E2.4 (§23.5) |
| §13.1 SD-n ("a missing engine row is a `retro.py note`, not a card") — for the AC parts, the HP/initiative/save notes, remaining lb and languages | item 4 (AC), 3a, 6 | These four are carded engine work (E2.6–E2.8); SD-n still holds for every other missing value |
| §16 "SD-40 changes no DTO the screen reads and no engine value" | items 3a, 4, 6 | The load response **gains** named fields and explanations rows (E2.6–E2.8, E2.14). **No existing field or row changes value.** G-1 is rewritten to check exactly that (`acceptance-and-verification.md` G-1) |
| `technical-design.md §4` page-1 composition; `§5` presets | items 1–6 | Rewritten (`technical-design.md §4`, `§5`) |

§3 (R1–R7), §6, §8 (Combat split), §10 (no page-count ceiling; Letter first), §12, §13 (protocol),
§15, §17 (paper-sheet rule), §18 and §22 stand.

### §23.3 Page 1 after round 1 (items 1–4, 6)

Top band: `portrait` (top-left, framed) · `nameCampaign` ("Character Name", "Campaign"; no title above
it). Then: `squareBlock` (Hit Points: max, big current area, small Nonlethal, Damage Reduction when the
screen shows one; Initiative: total; Encumbrance: Light / Medium / Heavy limits, total carried, remaining,
gear value; Conditions: box with the active conditions; then the notes for HP, initiative and encumbrance
and two blank lines), `abilities` (score + modifier, white space), `armorClass` (AC / Touch / Flat-Footed
and the seven parts, every one a real value), `attack` (BAB, Melee, CMB, CMD totals), `speed`, `saves`
(three totals, the save notes, two blank lines), `weapons` (name, attack total, damage as `1d8+4` from
E2.14's engine row, critical, range; blank lines; ammunition write-in), `skills` (one column, name + total, every Skills
panel row, then "Not added to a total" under the column) and `languages` (the known list as a comma list + exactly one blank line).
Every page-1 number is a **total** the engine states; the AC parts are the exception the operator made.

### §23.4 The print dialog and the request (item 5)

- `PrintRequest` carries `sections: PrintSections { main, feats, inventory, spellbook, bio, math }`
  (serde camelCase; all default `false` on the wire except as the dialog sends them). Paper stays its own
  field. E4.1 keeps the three booleans accepted and mapped while `sections` is absent (none → 1+2+3;
  each boolean adds its section) so the app prints between E4.1 and E4.2; E4.2 removes them.
- **PF1e:** each section is a `&[Placed]` list in `layout/pf1e.rs`, printed in box order 1 → 6, each
  starting on a new page: (1) Main = page 1 (§23.3); (2) Feats, class features & actions = `feats`,
  `classFeatures`, `actions`, `spells` (the per-day list with used boxes), `custom`, `pets`; (3) Inventory
  = `defense`, `gear`, `money`; (4) Spellbook = `spellbook` alone (spells only, §22 "2: A"); (5) Bio =
  `characterDetails`, then the Backstory text, then campaign notes, then ruled lines (§23.7 Q2); (6) Show the math = every breakdown removed
  from page 1, then the derivation lines (`blocks/math.rs`), built from the page-1 blocks whether or not
  (1) is ticked. The placement of `spells`, `custom`, `pets` and `defense` was the author's DEFAULT
  (§23.6 Q3), accepted by the operator (§23.7).
- **SF1e and CPR** keep their flow layouts (R2). Their dialog shows four boxes: Main sheet, Spellbook,
  Bio, Show the math (their sheet has no separate feats or inventory section; boxes 2 and 3 are not
  shown). `layout/mod.rs with_options` maps `sections` → today's `(spellbook, math, bio)` and prints
  their sheet only when Main is ticked; so the four old option sets are byte-identical under
  {Main} = none, {Main, Spellbook} = spellbook, {Main, Math} = math, {all} = all (G-2), and "Spellbook
  alone" (Main unticked) prints the spellbook block alone (§9's Spellbook behaviour, a named addition).
  `sf1e.rs` and `cpr.rs` do not change.
- **Golden cases:** the 24 keep their names, read through the mapping (PF: none = sections 1+2+3,
  spellbook = 1+2+3+4, math = 1+2+3+6, all = 1–6); E4.1 adds `{pf1e,sf1e,cpr} × {a4,letter} ×
  spellbookOnly` = 6 cases → 30
  (`python3 -c "import json;print(len(json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'))))"`
  → 24 today).
- **Remembered per character, first time 1+2+3:** the dialog reads and writes
  `codex.print.sections.<characterId>` in `localStorage`, the way `settings/printPaper.ts` keeps
  `codex.print.paper` (author's DEFAULT, accepted by the operator §23.7: per machine, not saved with the character; a clone
  starts at the default). A stored value that does not parse is the default. Nothing ticked disables Save PDF and Print.

### §23.5 E2.4's fate: rewritten as page 1's overflow rule, rebased on the new page 1

E2.4's parked commit `8f75f60b3e` (branch `sd40/e2.4-parked`, local and origin; `git log --oneline
origin/tranche-20..origin/sd40/e2.4-parked` → that one commit; its full test run never finished) makes
`frame.typ` measure page 1's grid run against the room left and, when it is too tall, steps the weapons
body to 8 pt and then moves the **whole Weapons block** to page 2 (`git show --stat 8f75f60b3e` →
`frame.typ` 65, `looks/weapons.typ` 26, `print_pf_blocks.rs` 54 changed lines). The new page 1 still needs an overflow
rule (a fighter with six weapons; a dwarf's save notes; a long "Not added to a total" list), but the old
**outcome** breaks item 3 (weapons are page-1 data) and the old test names blocks that no longer exist
(`characterDetails`, the ten-heading anchor list). **Decision:** card **E2.4 is rewritten**, not dropped:
"Page 1's overflow rule (many weapons, long notes)", run **after** the new page 1 is assembled
(E2.13), owning the new SD-b (§23.2). It may cherry-pick `8f75f60b3e`'s measuring code as its starting
point (named in its receipt) and re-does RED → GREEN from scratch; nothing from the parked commit lands
unverified. The lead deletes `sd40/e2.4-parked` (local and origin) once E2.4 is `complete`; E5.3's sweep
checks it is gone.

### §23.6 Questions only the operator could settle (answered the same day: §23.7)

| # | Question | Facts | DEFAULT taken (and why) |
|---|---|---|---|
| Q1 | Languages (item 6): what is the "known languages" list? | The engine holds no languages a character knows: no language field on the load response or the bio (`git grep -n 'pub [a-z_]*language' apps/desktop/src-tauri/src/character_hub.rs \| awk 'END{print NR}'` → 0); the screen shows `Common` plus "+N bonus language slot(s) (not yet selectable)", made in TypeScript from the Intelligence modifier (`spokenLanguages`, `CharacterSheet.tsx:418`) | Print the list the screen shows, from one backend source: E2.8 puts the known languages (`Common` today) and the bonus-slot count on the load response, computed in Rust from the Intelligence modifier the screen's budget reads (`ability_modifier.intelligence.saved` when an item raised Intelligence, else `ability_modifier.intelligence`; `savedIntelligenceModifier`, `CharacterSheet.tsx:449`), and the screen reads them too (the TypeScript rule goes). Paper: the comma list, then one blank line per bonus slot, at least one |
| Q2 | Bio text (item 5): the app has no bio/backstory text field | `CharacterBioDto` holds Player, Alignment, Deity, Sex, Age, Height, Weight, Hair, Eyes and nothing longer (`git grep -n 'pub struct CharacterBioDto' -A30 apps/desktop/src-tauri/src/character_hub.rs`); the only long text the print has is each campaign's description (today's Bio and Campaign Notes block) | Bio page = details, then each campaign's notes (the text the app holds), then ruled lines to the end of the page. A Backstory field in the app is new UI + storage: FSR-C7 (candidate), not built |
| Q3 | Where the daily spells list, custom records, pets and the armor table print | The six boxes name no home for them | Daily spells (used boxes), custom, pets → box 2 (table-use pages, ticked by default); armor/shield table → box 3 Inventory; box 4 Spellbook = spell entries only |
| Q4 | The page footer prints the name ("Aldric · page 1 of 4") | Item 2 says the name prints once; the footer is on every page and identifies loose pages | Keep the footer as it is |
| Q5 | Weapon damage as one total ("1d8+4", item 4) | The engine refuses a damage total on purpose: `src/rules_core/contract.rs:215-226` ("No summed \"damage roll total\" formula … exists anywhere in this codebase … A future, separate cycle owns turning this structured data into a summed display number, if that is ever wanted"); `combat.weapon_damage_bonus.<weapon>` is the feat part alone (`feat_pillar_and_pool_aggregation.rs:3056`); the Strength part assumes the primary hand (`WeaponDamageDto.str_damage_modifier`); no weapon range reaches the load response | E2.14 adds `combat.weapon_damage_total.<weapon>` in Rust (dice literal + Strength + enhancement + feat parts, the primary-hand / `WIELD` assumption named in its detail) and the record's range increment; page 1 prints `1d8+4`. If the operator says no: page 1 prints the dice and each bonus as the engine states them, unsummed (`1d8 +4 Str +1 Enh`), and E2.14 is dropped |

**Enforced by:** E2.4–E2.14, E2.MC2, E2.G round 2, E3.2–E3.5, E4.1, E4.2, E4.MC, E5.2 (`kanban.md`,
`epic-breakdown.md`). Cards 26 → 38 at the replan, 39 after §23.7 (`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`
→ 39). **Revisit:** the operator, at E2.G round 2 (page 1) and E5.2 (the rest).

### §23.7 Operator answers to §23.6 (2026-11-11, relayed by the lead)

| # | Operator's answer | Effect |
|---|---|---|
| Q5 | **"One total"** | E2.14 stays as proposed: the engine emits `combat.weapon_damage_total.<weapon>`; page 1 prints `1d8+4` |
| Q1 | **"Known + one blank line"** | Paper prints the known languages as a comma list, then **exactly one** blank line, always (not one per bonus slot); the bonus-slot count is not printed on page 1. E2.8 still moves the known list into Rust (`languages.known`), so the print never re-derives it; E2.8 no longer adds a slot count. E2.8 and E2.13 rewritten |
| Q2 | **"Add a Backstory field now"** | New card **E3.6** (main lane, opus, before E3.4): a Backstory free-text field in the sheet's Bio UI beside the existing bio fields, stored in `bio.json` with them (`CharacterBioDto`, `update_character_bio` / `load_character_bio`, `boundary/characterBio.ts`), round-tripping save, load and clone, with a ui-smoke row. E3.4's Bio page prints details, then Backstory, then campaign notes (if any), then ruled lines. No FSR-C7 (the candidate row is withdrawn) |
| Q3 | **as proposed** | Daily spells, custom, pets in box 2; armor table in box 3; Spellbook = spell entries |
| Q4 | **keep** | The footer keeps the name |
| smaller defaults | **accepted** | localStorage per machine; SF1e/CPR show 4 boxes; "Not added to a total" under the skills column |

**Enforced by:** E2.8, E2.13, E2.14, E3.6, E3.4 (`epic-breakdown.md`). Cards 38 → 39.

## §24 — Operator at E2.G, round 2 (2026-11-12): page-1 grid, conditional-bonuses box, resistances, section pages

**OPERATOR.** The operator looked at round 2's page 1 (`artifacts/epic_2/E2.G/aldric-{a4,letter}-none.pdf`,
rendered on `7bb1b2c874` after E2.MC2) and wrote `usable: no` for both sheets
(`artifacts/epic_2/E2.G_cycle_receipt.md`, "Round 2 verdict"). Census of what the repo holds:
`artifacts/epic_2/replan-r2/census.py`, output `census.txt`
(`python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r2/census.py`, from the repo root).

### §24.1 The operator's words (verbatim) and answers

> "a few things:  Skills should be on the right underneight Abilities.  Let's keep those two in a column of a uniform width.  Languages - remove the blank line.  For our test character, give him 7 languages to make sure we have given enough space for them all to show.  Campaign should be where Armor Class is - Character Name and Campaign are a row with a consistent height.  The armor class block title is not necessary, remove it.  For speed you have land, but not swim, nor fly - add a blocks for those two.  Remove hitpoint notes and initiative notes and encumberance notes (and the blank lines that accompany them).  We need a block that lists all the conditional bonuses - put that block in an outline with a fixed size that has a health blank space for people to add things in that we haven't worked out how to print yet.  This would be things like elves haveing a bonus to will save against enchantments.  Another example would be conditional bonsues to combat maneuvers or to initiatives - kind of a catch all for that type of thing.   A player also asked if we could add in energy resistances - some races have those automatically like ifrit and undine.  Let's also make seperate pages for the big sections - i mean they should have a page break when they are done so that they can be printed individually if desired.  Primary (page 1), Abilities and Feats, Inventory, Spellbook, Bio, Progression, Math.  Give each of those sections (except Primary) a prominant title."

Answers the same day (relayed by the lead):

- **Columns:** left column, top to bottom = HP / Initiative / Encumbrance / Conditions, then Weapons, Attack,
  Saving Throws, Conditional bonuses box, Resistances. Right column (even width) = Abilities, then Skills (one column).
- **Grid:**

  ```text
  +---------+------------+-----------+
  |Portrait | Name       | Campaign  |
  +---------+------------+-----------+
  | AC  Touch  FF | parts   | Speed L/S/F
  +---------------------+-------------+
  | HP Init Enc Cond    | Abilities   |
  | Weapons             |             |
  | Attack              | Skills      |
  | Saving Throws       |  (one col)  |
  | Conditional bonuses |             |
  | Resistances         |             |
  +---------------------+-------------+
  ```

- **AC + Speed:** a second row beside the portrait: AC / Touch / Flat-Footed boxes plus the filled parts row
  (no "Armor Class" block title), then Speed with Land, Swim, Fly boxes. Languages stays under the portrait. A
  speed the character lacks prints an empty box to write in.
- **Conditional box:** move ALL conditional notes into it (saves, CMB/CMD, initiative, AC, skills); nothing
  conditional prints anywhere else on page 1.
- **Resistances:** one block with energy resistances, damage reduction, spell resistance and immunities; blank
  boxes when the character has none.

### §24.2 What the repo holds (census, 2026-11-12, base `d5cd0a3ea6`)

Every figure: `python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r2/census.py` (section
letter given), output committed as `census.txt`.

| # | Question | Finding |
|---|---|---|
| a | Swim / fly speed | The engine states **land speed only**: one `speed.<mode>` row, `speed.land` (`character_hub.rs:3132`), from the race's walk speed (`race_creation.rs:244`) plus worn items. `speed.swim` / `speed.fly` rows: **0**. Two feat rows carry a swim or fly increase (`feat.arg_standalone.aquatic_ancestry_swim_speed`, `…stretched_wings_fly_speed`, `feat_pillar_and_pool_aggregation.rs:2057`, `:2143`); maneuverability is words in comments only. Corpus `MOVE` tokens: race_trait Swim **5**, Fly **2**, Climb **4** records (e.g. `undine_speed` `MOVE:Walk,30,Swim,30`, `strix_flight` `MOVE:Fly,60`); race records Walk only (39). The print Speed block prints Land only (`print/blocks/core.rs:207-209`) |
| b | Energy resistance, DR, SR, immunities | **No resistances or immunities field on the load response** (0 matches). Held today: `damage_reduction` on the snapshot (barbarian only, `pilot_view_model.rs:337`; printed under Hit Points, `core.rs:107`), `spell_resistance_total` from worn items (`character_hub.rs:287`), energy-resistance rows for ARG feats and two class features (`feat_pillar_and_pool_aggregation.rs:2004`, `class_inquisitor_warpriest.rs:411`, `class_sorcerer_wizard.rs:2802`). Ifrit and Undine: the converted rule keeps the words only (`bestiary_2:race_trait:ifrit_energy_resistance` target `None`, value `Text`, "Ifrits have fire resistance 5."); the number lives in the corpus record's `raw_bonus_chains` (`VAR FireResistanceBonus 5`). Corpus records granting each (predicate: an energy `BONUS:VAR <Energy>ResistanceBonus` chain; a `DR` token; an `SR` token; a record key naming `Immun`): race_trait energy **15** (awk cross-check: Fire alone 5 files), DR 1, SR 3, Immun 5; class_feature DR **54**, SR 8, Immun 16; ability DR 30, SR 6, Immun 13 |
| c | Conditional bonuses | Round-1 figures re-derived unchanged: situational Hp 15, Initiative 10, Fort 52, Ref 58, Will 78, Ac 178, Cmb **0**, Cmd **0**; Aspect SaveBonus 407, CombatBonus 314. Also situational Skill 216, SkillGroup 41, SkillSituation 19; of the 314 CombatBonus aspects **22** name CMB, CMD or "combat maneuver". The engine's `ConditionalNotes` holds **six** lists (hp, initiative, fortitude, reflex, will, armor_class; `conditional_notes.rs:55-60`) and gathers no CombatBonus aspect (`:111-114`): **no CMB/CMD list**. Skills' conditionals are `feat_skill_bonuses.situational`, printed under the Skills column as "Not added to a total:" (`print/blocks/skills.rs:67`). Page 1 prints notes on Max HP and Initiative (`core.rs:103`, `:110`) and under each save (`core.rs:186`); the **AC notes print nowhere** (0 matches). Elf "+2 vs enchantment": present — `core_rulebook:race_trait:elf_elven_immunities` carries an Aspect SaveBonus, which feeds all three saves (`conditional_notes.rs:133`) |
| d | Print sections | `layout/pf1e.rs`: page 1 = 11 `Placed::at/under` rows (`:20-34`); page 2 opens at `characterDetails` (`:50`); then `defense`, `gear`, `feats`, `classFeatures`, `spells`, `custom`, `pets`, `actions` flow on (`:36-44`); Spellbook, Show the Math, Bio and Campaign Notes are option blocks that each start a new page (`spells.rs:107`, `math.rs:27`, `bio_notes.rs:35`), in that order (`pf1e.rs:67-74`). **No section titles** (0). **No progression block** in `print/` (0); the Show Progression drawer reads `list_progression_ledger_at_root` (`character_ledger.rs:705`, `ProgressionLedgerDto` `:613`) |
| f | The desktop UI today (added after the operator's UI addition, §24.3.8) | Speed: the `speed` box shows **Land only** (`panels/CombatPanels.tsx:55-59`, called at `CharacterSheet.tsx:4506`), from the engine row through `sheetLandSpeed` (`customModel.ts:192`); swim / fly anywhere in the UI: **0**. AC and saves: `ArmorClassPanel` / `SavingThrowsPanel` (`CombatPanels.tsx:20`, `:32`; `CharacterSheet.tsx:4502`, `:4508`) and the Defense tab's `SavesPanel` (`CharacterSheet.tsx:1422`, `:1432`) show totals only. **No UI component reads `conditionalNotes`** (its 9 matches are SF/CPR test fixtures; the type is `loadSavedCharacterDetail.ts:160`, `:224`); the only conditional list shown is the Skills panel's "Not added to a total" (`CharacterSheet.tsx:523`). Damage Reduction: one line in the Defense tab (`CharacterSheet.tsx:1419`); Spell Resistance: an equipment-effects tile (`:1368`); energy resistance and immunities: **shown nowhere** (the one word match, `:1885`, is a catalog note). ui-smoke: 173 rows, 0 expecting Swim, Fly, Resistance, Immunity, Bravery or Conditional |
| e | Aldric's languages | Seed `character_hub.rs:2473` (Human Fighter 3, Int 14; ranks `ALDRIC_SEED_SKILL_RANKS` `:2372`, no Linguistics). **Every PF character knows exactly `Common`**: `known_languages()` is a constant (`character_hub.rs:743`); no save field holds languages. Pins: `character_hub.rs:20853` (`{"known": ["Common"]}` for the seeds), `print_pf_blocks.rs:1513` (contains Common; values `[known, ""]`), `print_golden.rs:229` (`languages` page-1 anchor); `print_golden.json` pins headings and pages only. Giving Aldric 7 real languages needs a language store the app does not have, or Linguistics ranks the seed does not have (moves skill totals: G-1 `VALUE_MOVED`). Cleanest: a **print-proof fixture** through a seam in `print/mod.rs` (`sheet_document_at_root` loads and lays out in one function, `:85-86`) |

### §24.3 Rulings (item 8 added the same day for the operator's UI addition)

1. **Page-1 grid** (E2.18). Row 1: portrait (columns 0–2, spans rows 1–2) · Character Name (3–7) · Campaign
   (8–11), the two fields one row of one height. Row 2 beside the portrait: AC / Touch / Flat-Footed boxes and the
   seven-part row, **no "Armor Class" heading** (columns 3–8), then Speed with **Land, Swim, Fly** boxes (9–11).
   Languages under the portrait. Body: left column (0–6) = square block (HP, Initiative, Encumbrance, Conditions),
   Weapons, Attack, Saving Throws, Conditional Bonuses, Resistances; right column (7–11, one width for both) =
   Abilities then Skills (one column, as E2.11). The column split 7 / 5 is the author's DEFAULT (E2.11 found one
   skills column needs 5 of 12); the card may move it if a block clips, and names the move.
2. **Speed** (E2.17, E2.18). The engine gains `speed.swim` and `speed.fly` explanations rows (feet), from the
   race traits the character holds (their `MOVE` tokens, alternate-trait replacement honoured as the land speed's
   is) plus the two feat increases; the fly row's detail names the maneuverability when the record states one
   ("average", "poor"), and the load response gains `flyManeuverability` (the record's word, or absent) so the
   print and the screen read one field. No row when the character has none; the box then prints empty for the pencil (never a
   dash, never 0). Climb and burrow: not asked; not printed (they stay in the rules text).
3. **Languages** (E2.16). Comma list, **no write-in line**; the cell holds seven names on Letter and A4. The test
   character's seven languages are a **print-proof fixture**, not seed data: `print/mod.rs` splits
   `sheet_document_at_root` into load + `sheet_document_from_loaded(root, &loaded, request)`, and `print_proof`
   renders Aldric a second time with `languages.known` = `Common, Dwarven, Elven, Giant, Gnome, Goblin, Orc`
   (`aldric-languages7-{a4,letter}-none.pdf`; 16 → 18 PDFs). The seed, the engine and every value stay as they
   are (G-1). E2.G round 3 judges the seven-language sheets.
4. **Notes leave the blocks** (E2.20). The HP, Initiative and Encumbrance notes, their headings ("Hit point
   notes", …) and their blank lines go; the save notes no longer print under Saving Throws; "Not added to a total:"
   no longer prints under Skills (its lines move, "Ranks with no Skills panel total:" stays: it is not a
   conditional bonus).
5. **Conditional Bonuses box** (E2.19, E2.20). An outlined box of **fixed height**, heading "Conditional
   Bonuses", listing every conditional the engine states for the character: HP, initiative, Fortitude, Reflex,
   Will (a note given to several saves prints once, as today), AC, CMB, CMD (new engine lists, E2.19: the Aspect
   CombatBonus entries whose words name CMB, CMD or combat maneuvers, and any Situational rule targeting `Cmb` /
   `Cmd`), and skills (`feat_skill_bonuses.situational`); each line `<what>: <label>: <value> <condition>` in the
   engine's words; then ruled blank write-in lines to the box's foot. DEFAULT size: 14 body lines on Letter and
   A4, of which **at least 4 are always blank** (§24.6 Q1). A list too long for the box follows the SD-b rule
   (§23.2): shrink to the 8 pt floor, then continue at the top of page 2 under "Conditional Bonuses (continued)";
   the 4 blank lines stay on page 1.
6. **Resistances block** (E2.21, E2.22). Heading "Resistances": five energy boxes (Acid, Cold, Electricity, Fire,
   Sonic), Damage Reduction (each `amount/bypass`), Spell Resistance, Immunities (a line of names). Filled from a
   new load-response field `resistances` (E2.21) built from the sources the character holds: race traits' energy
   `BONUS:VAR` chains (Ifrit fire 5, Undine cold 5), `DR` and `SR` tokens of held race traits, class features and
   abilities, the existing barbarian DR, worn-item SR and energy-resistance rows; immunities = held records whose
   name says Immunity, by name (words, never a number). A DR or SR the engine cannot evaluate to a number prints
   the record's words in that box (paper-sheet rule, §17: print the rule, do not simulate). Blank boxes when
   none. The barbarian's "Damage Reduction" line leaves the Hit Points block for this block.
7. **Sections** (E2.23, E3.7). Seven sections in this order: **Primary** (page 1, no title), **Abilities and
   Feats** (`feats`, `classFeatures`, `actions`, `spells`, `custom`, `pets`), **Inventory** (`defense`, `gear`,
   then `money` from E3.2), **Spellbook**, **Bio** (`characterDetails` + the bio/campaign notes; E3.4 reworks),
   **Progression** (new, E3.7: the Show Progression drawer's content level by level), **Math**. Each section
   after Primary starts on a new page under a **prominent title** (a section-title look, full width, larger and
   bolder than any block heading) and ends with a page break, so it prints alone. The dialog's checkbox labels are
   these seven names (E4.2); first-time ticks stay Primary + Abilities and Feats + Inventory; Progression is
   Pathfinder only (SF1e/CPR show Primary, Spellbook, Bio, Math).
8. **The desktop UI shows the same new fields** (operator, 2026-11-12, verbatim: "let's make adjustments to the ui to
   include these fields as well"; E2.24–E2.26). Each UI card renders the **same engine rows and fields the print
   reads** (no TypeScript re-derivation of a rules number: AGENTS.md "a magnitude is not wired until it moves on the
   twin the player reads", SD-27 `decisions.md §29`), adds its ui-smoke row to `apps/desktop/scripts/ui-smoke/spec.json`,
   regenerates `docs/testing/ui-smoke-inventory.md`, and depends on the engine card that feeds it. Placements
   **chosen by the author** (closest existing neighbour; not questions):
   - **Swim and Fly** (E2.24): two tiles beside Land in the sheet's `speed` box (`SpeedPanel`), `<n> ft.` from
     `speed.swim` / `speed.fly`, the fly value followed by `(<maneuverability>)` from `flyManeuverability`; a mode the
     character lacks shows `—` (the screen's no-value mark; paper keeps its empty box).
   - **Conditional bonuses** (E2.25): a "Conditional Bonuses" list directly under the three totals in the sheet's
     `saves` box (`SavingThrowsPanel`): the same entries and order as the printed box (HP, Initiative, Fortitude,
     Reflex, Will once per rule, AC, CMB, CMD, the Skills' situational lines); none → "None stated.". The Skills
     panel's "Not added to a total" line stays where it is (the screen has room; not asked to move).
   - **Resistances** (E2.26): a "Resistances" panel in the Defense tab where the Damage Reduction line is today
     (`CharacterSheet.tsx:1417-1421`, above `SavesPanel`): Acid, Cold, Electricity, Fire, Sonic tiles, Damage
     Reduction, Spell Resistance and Immunities from `resistances`; the old one-line DR moves into it; the
     equipment-effects Spell Resistance tile (`:1368`) stays (it is the items' own total). None → `—` tiles.

### §24.4 What this supersedes

| Superseded | By | Effect |
|---|---|---|
| §23.3 page-1 composition (Speed under the name, AC in the top band with its heading, square block beside Abilities, Skills left beside Weapons) | §24.3.1 | The grid above (E2.18) |
| §23.1 item 3a "SPECIAL NOTES … then a few blank lines"; §23.3 "then the notes for HP, initiative and encumbrance and two blank lines"; §23.2's "two blank ruled lines after each notes list" | §24.3.4–5 | Notes print only in the Conditional Bonuses box; its write-in space replaces the per-list blank lines |
| §23.1 item 4 "Saves … PLUS conditional save bonuses printed as notes … then a blank line or two" | §24.3.5 | Saving Throws print the three totals only |
| §23.7 Q1 "exactly one blank line" (E2.13's write-in line) | §24.3.3 | No write-in line under Languages |
| §23.7 smaller default "'Not added to a total' under the skills column" | §24.3.5 | Those lines print in the box |
| §23.3 "Damage Reduction when the screen shows one" in the Hit Points block | §24.3.6 | Damage Reduction prints in Resistances |
| §23.1 item 4 AC exception: heading "Armor Class" | §24.3.1 | The parts stay; the heading goes |
| §23.1 item 5 / §23.4: six sections, labels "Main sheet … Show the math", order 5 Bio → 6 Show the math | §24.3.7 | Seven sections, operator's names and order (Progression new, Math last), section titles; `PrintSections` gains `progression`; golden mapping all = 1–7, math = 1+2+3+7; dialog boxes 7 (4 on SF1e/CPR) |
| §23.6 Q1 / E2.8 "every character knows Common" as the printed list | — | **Stands** for real characters (§24.6 Q2); the seven languages are a print-proof fixture |

SD-b (§23.2, the overflow rule E2.4 built) **stands**; its "a notes list" case is now the Conditional Bonuses box.
§23.1 items 1, 2, 3a (one square block), 3b, 3c, the AC parts, §23.4's per-character memory, §23.5, §23.7 Q2–Q5
stand.

### §24.5 Correction found while recording the verdict

The G-4 check `grep -c 'usable: yes' …/E2.G_cycle_receipt.md` → 2 counted the receipt's own instructions: on
`d5cd0a3ea6`, with no sign-off written, it printed **3**
(`git show d5cd0a3ea6:docs/release/SD-40-pf-sheet-design/artifacts/epic_2/E2.G_cycle_receipt.md | grep -c 'usable: yes'`
→ 3). A presence gate that can pass, or miscount, without the operator. Replaced by the anchored
`grep -c '^- Aldric .* — usable: yes'` (→ 0 now, 2 when round 3 is signed) in the receipt, G-4, G-8 and the E2.G
card. E5.2's `grep -c 'usable: yes'` has the same shape; E5.2's card must write its sign-off lines so an anchored
count works (G-8 note).

### §24.6 Questions only the operator can settle (DEFAULT taken; work proceeds on it)

| # | Question | Facts | DEFAULT |
|---|---|---|---|
| Q1 | How big is the Conditional Bonuses box? | Aldric holds one conditional (Bravery, +1 Will vs. fear); the left column also carries 6 other blocks on Letter | 14 body lines, at least 4 always blank; overflow per SD-b |
| Q2 | Should real characters record their languages? | `known_languages()` is the constant `["Common"]` (`character_hub.rs:743`); the app has no language store or picker; 7 languages exist only in the print proof | No store in SD-40; the seven-language Aldric is a print-proof fixture. A languages field (free text, saved with the bio, printed on page 1) would be one new main-lane card before E2.MC3 if the operator wants it |
| Q3 | Fly maneuverability on paper | The fly record states it in words ("average") | Printed beside the Fly box value: `60 ft. (average)` |

**Enforced by:** E2.16–E2.26, E2.MC3, E2.G round 3, E3.7, E3.MC, E4.1, E4.2, E4.MC (`kanban.md`,
`epic-breakdown.md`). Cards 40 → 53: 50 at the replan, + E2.24–E2.26 for the operator's UI addition, §24.3.8
(`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'` → 53). ui-smoke rows 173 → 176 (E2.24–E2.26) → 177 (E3.6)
→ 181 (E4.2), G-9.
**Revisit:** the operator, at E2.G round 3 (page 1) and E5.2 (the sections).
