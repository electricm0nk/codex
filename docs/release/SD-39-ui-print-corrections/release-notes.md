---
canonical: true
bundle_id: SD-39
slug: ui-print-corrections
tranche: 19
date: 2026-11-07
status: final notes written at E9.8; closure cards E9.9 to E9.11 still to run
---

# Campaign Codex 0.19 — UI and Print Corrections

Twelve operator-dictated defects in the desktop app (printing, the Pathfinder sheet layout, feats, the
Market, equipping, the level ledger, overrides, `.pcg` import), corrected on `tranche-19`. Every figure
below is listed under "Figures" as `R-n` with two independent commands and the value both gave when card
E9.8 ran (2026-11-07). Where prose quotes a number from a receipt instead, it says "quoted" and names the
receipt.

**Status at time of writing.** 62 of 66 kanban rows are `complete` (R-3); this card is one of the four
that are not. Still to run after this file, in order: architecture truth-up and claims critic (E9.9),
graphify against the final tree (E9.10), and the PR `tranche-19` to `develop` (E9.11), which the operator
merges. Nothing publishes from `tranche-19`; the published triple `0.19.<run>` resolves at the first
tester publish after the operator merges and is not decided here.

## Summary

SD-39 answers the operator's twelve defects D1 to D12 (`scope-draft.md §1`). Printing is rebuilt as a PDF
generated from per-system Typst templates (Pathfinder, Starfinder, Cyberpunk RED), A4 or US Letter. The
Pathfinder sheet is rearranged to the requested layout. Feats cannot be over-added or duplicated. A Market
replaces the three Add buttons. Armor, shields and weapons have +1 to +5 variants. Items can be worn or
carried, in body slots, and worn items change the sheet's numbers. A per-level ledger shows what each level
chose. Overrides are explicit toggles and every admitted record is tagged. `.pcg` characters import through
a sidecar. Nine cards were discovered while the run was in progress and finished inside it (R-5).

## User-Visible Changes

- **Printing (D1).** Settings has a Print tab (paper: A4 default, US Letter). One Print dialog on the
  Pathfinder, Starfinder and Cyberpunk RED sheets offers Spellbook, Show the math, and Bio and campaign
  notes, each printed on pages of its own after the sheet, plus Save PDF and Print. The PDF is built by a
  bundled Typst with Liberation Sans 2.1.5 (OFL) compiled in. Hit Points, Speed and the 35 skill totals
  printed come from the same backend value the screen shows. Quoted from `artifacts/epic_8/E8.5_cycle_receipt.md`:
  24 golden PDFs (3 characters, A4 and Letter, 4 option sets), zero Typst diagnostics, two renders
  byte-equal. Quoted from `artifacts/epic_9/E9.2_cycle_receipt.md`: Save PDF from an installed `.deb` and an
  AppImage wrote the same 4-page A4 PDF (identical SHA-256).
- **Pathfinder layout (D2, D8, D9).** Abilities sit where Character Details was; Character Details is in the
  left panel below Class; Armor Class, Attack, Speed and Saving Throws join the combat row as a 1x4 beside
  the 2x2; the old left-nav history is a Show Progression button opening a drawer with the per-level ledger
  (feats, spells, skills, abilities, and a hit die per level).
- **Feats (D6, D7, D10).** Adding a feat with none left, or the same non-repeatable feat twice, is refused
  with a reason. A checkbox hides feats whose prerequisites are unmet. The Add Custom chip no longer sticks
  highlighted.
- **Market (D3, D4).** One Market button buys and sells, in the price modes, with +1 to +5 armor, shields
  and weapons. Starfinder and Cyberpunk RED have a Market shell. Attaching a modifier charges the +N it adds.
- **Equipping (D5, D11).** Body slots, worn versus carried, an Equipment panel, and "Left behind" for owned
  but not carried. Worn items change AC, attack, damage, initiative, speed, skills, CMB and CMD by what they
  state. Custom items take real bonuses and a slot. Nothing is required to be worn for a character to compute.
- **Overrides.** Toggles on the Custom tab; every item an override admitted is tagged on the sheet and print.
- **Import (D12).** Import PCGen `.pcg` characters, with a report of what mapped and what did not.
- **Update page.** On a fresh install after Check, Update shows `check ok` and no `unknown` in the Installed,
  Last check or Pending / Rollback rows (quoted from the E9.2 receipt, `.deb` and AppImage).

## Defects Fixed

The twelve defects are the bundle. Defects found by the bundle's own cards and fixed in it:

- **Install button.** Update > Install sent flat arguments to `perform_install`, which takes the manifest and
  index URL directly (PR #402, `99315fbe45`, merged to `develop`). That commit is in this tree and in the
  build E9.2 installed (R-7).
- Export then Import dropped every paid `+N` (E3.MC); a worn deflection bonus left CMD alone and a clone
  lost `custom.json` (E4.MC); a clone lost `hit_points.json` and a cut-short `ledger.json` refused every
  later change (E5.MC); a Market weapon was read by no lookup and a race's or deity's weapon was refused
  (E6.MC); the basic Pathfinder print left out 28, 25 and 29 of 35 skill totals (E8.MC, fixed by E8.6 to E8.8).
  Each is quoted from the named merge-check receipt.
- A CPR therapy test failed on a high Extreme roll from the clock (E5.4: pinned to a seed, 40 of 40 exact runs).

## Operational Notes

- **Version.** The repo value is `0.19.0` on 14 surfaces (R-1). `docs/release/current-release.json` names
  this file (`python3 tools/release/resolve_release_notes.py --repo-root .` prints its path).
- **Print engine.** Typst 0.15.1 and typst-pdf are compiled in; no external binary is called at print time.
- **`.pcg` sidecar.** `pcg_import` ships as a sidecar binary beside the app; the `.deb` carries it at
  `/usr/bin/pcg_import`. No `.lst` or `.pcg` is parsed in `src/` or `apps/desktop/` (crate wall PASS in R-4).
- **Packaging.** Quoted from the E9.2 receipt: a clean-checkout build produced `Codex_0.19.0_amd64.deb`
  and `.AppImage`, each run on an empty profile under an isolated app-data root.
- **Seeds.** The Pathfinder, Starfinder and Cyberpunk RED seed renders showed NO_DIFF on every code card
  except where a card named an intended value change (quoted from each receipt's Seed deltas line).

## Verification Evidence

Widest verify once, at E9.3: `bash scripts/verify.sh --full --show-actuals -j 8`, RESULT PASS, 53 stages
PASS and 0 FAIL, no `test result: FAILED` line (R-4; summary `artifacts/epic_9/E9.3_verify_summary.txt`).
E9.4: the operator signed three printed A4 sheets usable (Aldric Pathfinder full, Soldier Starfinder basic,
Solo Cyberpunk RED basic), 3 of 3 (R-8). E9.5: the final-acceptance scan and every DEF/FSR revisit command
passed. Commits after E9.3 touched documents and receipts only. Per-card receipts are listed in
`kanban.md`; the retrospective is `docs/retro/sd39-retrospective.md`. CI evidence (`pr-tests`) belongs to
E9.11 and does not exist yet.

## Known Issues

- **Full `npm run ui-smoke` is not green.** Quoted from the E2.4 receipt: of 133 rows, 64 green, 14 red,
  52 blocked, 3 manual; none of the 14 red is an SD-39 row. This is FSR-C2, whose revisit condition is met;
  it is a Candidate outside the Definition of Done and the operator's to schedule. E9.3 ran
  `ui-smoke:doc`, not the full run.
- **Print design and density.** The operator accepted the three sheets as usable and sent design and
  density to SD-40.
- **Hit-point tracker.** The tracker's own Max still follows `load_character_durability`, not the sheet's
  `hit_points.max` (E8.6 "does not cover").
- **Planned deferrals** (not in the DoD, `forward-scope-register.md §4`): magic weapon and armor special
  abilities and special materials; Starfinder armor-upgrade slots and Cyberpunk install locations; PCGen XML
  import; pixel-snapshot print tests; editing a past level from the drawer.

## Does not cover

- **A real update install.** E9.2 and E9.5 checked the update path only. 0.19.0 is above the published
  0.18.147, so nothing was eligible, and the `perform_install` download, verify and replace path (fixed in
  #402) was not exercised against a live newer release. The first publish of 0.19.x and an install of it from
  0.19.0 is the test that closes this.
- Windows and macOS packaged builds, and CI.
- Print: only the three signed sheets were printed by hand. The golden suite covers 3 characters on A4 and
  Letter; other characters are covered by the test suites in R-4, not by PDF.
- A PDF opening in a viewer: this box has no default PDF handler. The PDFs were read with `pdfinfo`,
  `pdftotext` and `pdftoppm`.
- R-2 and R-3 move with every later commit and are pinned to the tree at E9.8; this card's own row is not in R-3.
- R-6 is the recorded summary written at E9.6 (2026-11-07T18:19:02Z); the retro log keeps growing.
- Judgments in the prose are checked against receipts, not by a command; only `R-n` figures carry commands.

## Figures

Each line: value in bold, implementation A (shell, run from the repository root of this worktree),
implementation B (Python, independent). "Gives" is the output when E9.8 ran, 2026-11-07.

- **R-1** build version in the two desktop manifests: **0.19.0,0.19.0**. A: `grep -h -m1 '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'"' '{print $4}' | paste -sd,` gives `0.19.0,0.19.0`. B: `json.load` of both files, key `version`, gives `0.19.0,0.19.0`.
- **R-2** commits on tranche-19 past the merge base with `origin/develop` (`2b14020bc7`), to the tree before this card's commit: **138**. A: `git log --oneline $(git merge-base origin/develop HEAD)..HEAD | awk 'END{print NR}'` gives `138`. B: `git rev-list --count $(git merge-base origin/develop HEAD)..HEAD` gives `138`.
- **R-3** kanban rows, then rows `complete`, before this card's row moved: **66 62**. A: `awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {n++; if ($5 ~ /^ complete *$/) c++} END{print n, c}' docs/release/SD-39-ui-print-corrections/kanban.md` gives `66 62`. B: split on `|`, id in column 1 and status in column 4, gives `66 62`.
- **R-4** `verify.sh` stages passed, stages failed in the E9.3 full run: **53 0**. A: `awk '/^ +PASS /{p++} /^ +FAIL /{f++} END{print p, f+0}' docs/release/SD-39-ui-print-corrections/artifacts/epic_9/E9.3_verify_summary.txt` gives `53 0`. B: regexes `^\s+PASS ` and `^\s+FAIL ` over the same file give `53 0`. Of the counts in that file, quoted: root-full 6771 passed across 300 suites, ingest-full 1858, desktop 1240, frontend 191/191 files, clippy 0 warnings and 0 errors in each of three workspaces.
- **R-5** cards discovered inside the run: **9**. A: `grep -c 'discovered by' docs/release/SD-39-ui-print-corrections/kanban.md` gives `9`. B: Python count of lines containing `discovered by` gives `9`.
- **R-6** retro events since 2026-11-01 (recorded summary), corrections, incidents: **206 128 12**. A: `jq -r '[.events.total,.events.by_type.correction,.events.by_type.incident]|map(tostring)|join(" ")' docs/release/SD-39-ui-print-corrections/artifacts/epic_9/E9.6_logs/retro-summary.json` gives `206 128 12`. B: `json.load` and the same keys give `206 128 12`.
- **R-7** the `perform_install` argument fix (`99315fbe45`) is an ancestor of this tree, then of the commit E9.2 built (`257790a6e2`), exit statuses: **0 0**. A: `git merge-base --is-ancestor 99315fbe45 HEAD; echo $?` then the same with `257790a6e2` gives `0` and `0`. B: `git rev-list HEAD | grep -c '^99315fbe45'` and the same on `257790a6e2` each gives `1`.
- **R-8** operator sign-offs `usable: yes` in the E9.4 receipt, then ui-smoke rows in `spec.json`: **3 173**. A: `grep -c 'usable: yes' docs/release/SD-39-ui-print-corrections/artifacts/epic_9/E9.4_cycle_receipt.md` gives `3`; `jq -r '.rows|length' apps/desktop/scripts/ui-smoke/spec.json` gives `173`. B: Python `str.count` and `len(json.load(...)['rows'])` give `3` and `173`.

## Update Eligibility

Set at publish by the release manifest (`tools/release/check_release_manifest.py`; the repo keeps the
version placeholder until then). Not decided in this document. What this bundle established about the
automatic upgrade path: a fresh 0.19.0 install, `.deb` and AppImage, reads the published index, shows
`check ok`, reports its install kind correctly (deb / appimage) and reports `ineligible` only because it is
newer than the published 0.18.147; the Install button's argument fix is in the build (R-7). What it did not
establish is a real install of a newer release (see "Does not cover").
