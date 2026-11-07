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
reference figure a look card compares its own output against, never as a limit. The exact `pages` count
in `print_golden.json` stays a regression pin that re-records on purpose only (SD-f). **Letter first:**
page 1 is designed to fit US Letter's printable height (11 in − margins); A4 is taller and gets the
slack, so one design serves both papers (`Paper::Letter` already exists: `model.rs`). **Enforced by:**
E0.1 (figures), E2.3 (page 1 on Letter), every look card's receipt (page counts before → after), the
operator's eye at E2.G and E5.2, G-6. **Revisit:** the operator names a limit; no card invents one.

## §11 — Look rules (R6; DEFAULT)

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
| SD-b | A block overflows its grid cell or a step-1 block spills off page 1 | Typst flows the overflow; for a page-1 spill, shrink that block's body to the 8 pt floor, then move the lowest-priority page-1 block (weapons) to page 2; never drop data |
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
| SD-m | A write-in area's size | One ruled line of 0.5 cm per entry; HP box 2 × 2 cm; copy the reference where it has one |
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

## §22 — Operator answers 2026-11-07

The operator answered the package's three open questions (`README.md §6` rows 9, 5, 6) on 2026-11-07:

| # | Question | Answer | Effect in this package |
|---|---|---|---|
| 1 | The 14 stale red ui-smoke rows (FSR-C1; SD-39 FSR-C2; retro change 5) | **"1: B"**: add one small card in SD-40 that owns them | Card **E0.2** "Repair the 14 stale ui-smoke rows" (`opus`), side lane `smoke` parallel to E1; it writes `apps/desktop/scripts/ui-smoke/spec.json`, the regenerated `docs/testing/ui-smoke-inventory.md` and whatever the rows need (its FILES YOU OWN); **serial edge E0.2 → E4.2**, which also writes `spec.json` (`kanban.md` E4.2 `Depends on`; R24). Each row is repaired to the current UI or rewritten to match it, never deleted to make the count pass. FSR-C1 → Committed COM-9; DoD 11; G-9; NR-14. Cards 23 → 24 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 24) |
| 2 | What the Spellbook preset prints | **"2: A"**: Spellbook prints spell pages only; Full prints everything | §9's design stands as written; no card changes |
| 3 | Density ceilings (`pages_max`) | **"3: No hard limits"** | §10 rewritten: no page-count ceiling as a test or gate; page counts are recorded per case in each receipt (before → after) and the operator judges density by eye at E2.G and E5.2. Letter-first design kept. NR-10, G-6, E0.1, E4.1, E4.MC, SD-b, SD-f, R-5, R-6, DoD 6 rewritten to match |

**Enforced by:** `kanban.md` (E0.2 row; E4.2 depends on E0.2); E0.2's acceptance and G-9; G-6 and NR-10
as rewritten. **Revisit:** the operator.
