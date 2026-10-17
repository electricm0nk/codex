---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-15
status: final notes written at E7.8; closure cards E7.9 to E7.11 still to run
---

# SD-38 Release Notes

Cyberpunk RED (corebook v1.22). Every figure in this file is listed under "Figures" as `R-n`, each with
the command that produced it, a second independent implementation, and the value both gave when card
E7.8 ran. Where prose below quotes a number from a receipt instead of an `R-n`, it says "quoted" and
names the receipt.

**Status at time of writing (2026-10-15, `tranche-18`, base `cc9d7d9a3c`).** Cards through E7.7 are
`complete` (R-3). Still to run after this file, in order: the architecture truth-up and claims critic
(E7.9), graphify against the final tree (E7.10), and the PR `tranche-18` to `develop` (E7.11), which the
operator merges. The repo's build value is the one in R-1; the published triple `0.18.<run>` resolves at
the first tester publish after the operator merges, and is not decided here.

Re-derive every figure in one command (exit 0 only if each figure matches the notes and the three
planted corruptions are rejected):

```
python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.8_check.py --plant
```

## Summary

SD-38 adds Cyberpunk RED as a third game system beside Pathfinder 1e and Starfinder 1e. The game system
seam was widened to three systems (E0). The corebook text was transcribed by hand from the source PDF
into a corpus of record kinds (R-4), with a 10% sample checked against page images (E1). A Rust engine
computes every sheet number from those records (E2), and the desktop app creates (E3), shows and prints
(E4), improves (E5) and browses (E6) Cyberpunk RED characters. Three starter characters are restorable
(E7.1). Pathfinder and Starfinder are unchanged: the Pathfinder render hashes and the Starfinder seed-line
files are byte-identical to the base (R-12, R-13).

## User-Visible Changes

Under the Cyberpunk Red chip on the landing page of the desktop app:

- **Create.** New Character opens a Cyberpunk creation form with the three methods of the corebook:
  Streetrat, Edgerunner and Complete Package. A seedable roller serves the creation-time table rolls (the
  Streetrat template row, the Edgerunner STAT rolls, the Lifepath), a Lifepath panel, a skills allocator,
  Role Ability at Rank 4 with its sub-choices, a starting kit with cash, and a cyberware pack with a
  Humanity preview (E3).
- **Sheet.** A saved Cyberpunk character opens a Cyberpunk sheet (STATs, HP, Seriously Wounded, Death
  Save, Humanity, skills, weapons with range DVs, armour locations, cyberware, Role Ability, a Cyberdeck
  block, Reputation, IP, Lifepath and a critical-injury checklist) and not the Pathfinder layout. Every
  computed number is an engine explanations row with an id starting `cpr.`; no screen re-derives one (E4).
  Text rules are printed, not simulated, and dice stay literal on the sheet.
- **Improve.** Improve opens dialogs for spending IP on skills and Role Rank, switching Role, installing
  cyberware with its Humanity Loss, therapy, and Reputation changes (E5).
- **Catalogs and tools.** Cyberpunk catalog screens, a Netrunning tools screen with a Cyberdeck
  calculator, and print-only GM reference pages (E6).
- **Starters.** Three starter characters, Jax Moreno (Solo, Streetrat), Nyx Calder (Netrunner, Complete
  Package) and Imani Okafor (Medtech, Edgerunner), are seeded on a fresh install and returned by Restore
  starter characters (E7.1). Quoted from `artifacts/epic_7/cpr-seed-builds.md`: every number on each
  loaded sheet (231, 213 and 223 rows) equals a re-derivation from the book's formulas and the corpus
  with no engine code.
- **Update panel.** After a Check on a fresh install, Pending / Rollback prints `idle` and `none` instead
  of `unknown`, and the footnote that called the values placeholders is gone (E7.3a).

Pathfinder and Starfinder users see no change in the Pathfinder render hashes or the Starfinder seed
lines (R-12, R-13).

## Defects Fixed

Found by this bundle's own cards and fixed in it (each is a retro event or a progress.md finding; none is
a defect in a shipped release):

- On a fresh install the Update page printed Pending update state and Rollback state as `unknown`, and
  Last check printed an "Install disabled reason" beside Eligibility `eligible`. Found by the fresh-install
  acceptance in E7.3 and fixed in E7.3a (`updateModel.ts`, `controllerAdapter.ts`).
- The live Cyberpunk sheet showed raw ids (`cpr.cash`, `cpr.ammo.<i>`) as labels under Other totals for
  every created character, and the critical-injury checkboxes had no accessible name. Found in E4.7,
  fixed in E4.8.
- The seed-line writer tests failed with `NotFound` when the output directory was absent (three
  incidents). E4.8 made both the Cyberpunk and the Starfinder writer tests create their output directory
  (`create_dir_all`, quoted from `artifacts/epic_4/E4.8_cycle_receipt.md`).
- The closure-scan row count read 69 after E7.3a was discovered; raised to 70 at all seven places
  (E7.5 blocker, cleared by the lead). Recorded as retro corrections (R-14).

## Operational Notes

- **Licence.** Cyberpunk RED is not OGL. By operator ruling of 2026-10-13 the whole book text ships in a
  private single-user build; `data/cyberpunk-red/corpus/LICENSE.json` says `open_game_content` false and
  `signed_off` true (R-6), and the licence matrix records a DEVIATION row (`decisions.md §4`). No Product
  Identity screening is applied to these records. A later bundle can reword `text` fields page by page
  (FSR-C1). Nothing here is legal clearance.
- **Source.** The corpus is a hand transcription of the PDF; no PCGen data or oracle governs any
  Cyberpunk figure (`decisions.md §23`). The PDF, page text and page images are never committed.
- **Corpus size.** Tracked files under the Cyberpunk data root are in R-5.
- **ui-smoke isolation.** The ui-smoke harness runs against an isolated app-data root; the Cyberpunk rows
  are counted in R-7.
- **Packaging.** Quoted from `artifacts/epic_7/E7.3_cycle_receipt.md`: a `.deb` and an AppImage built from
  this tree (`Codex_0.18.0`) were installed on a clean profile on Linux, and a Cyberpunk seed opened.
- **Version.** The repo value is `0.18.0` on 14 surfaces and `docs/release/current-release.json` names this
  file (`python3 tools/release/resolve_release_notes.py --repo-root .` prints its path).

## Verification Evidence

The last full run is `bash scripts/verify.sh --full --show-actuals -j 8` in E7.4, on `efe5e27a0a`: every
stage passed (R-8), with test counts in R-9, no `test result: FAILED` line in any of the four logs (R-10)
and the frontend suite in R-11. Commits after it touched only documents (`git diff --name-only
efe5e27a0a..cc9d7d9a3c -- . ':!docs'` prints `scripts/verify-baselines.env` only). Logs: `artifacts/epic_7/E7.4/logs/`. Per-card receipts are listed in `kanban.md`. CI evidence
(`pr-tests`) belongs to E7.11 and does not exist yet. The retro log summary is R-14.

## Known Issues

- **Full ui-smoke is red on Pathfinder rows.** Quoted from `progress.md` (E4.7 findings, 2026-10-15):
  9 of the first 40 rows were red (Pathfinder create and equipment-catalog rows), because tranche-17-ui
  changed the PF create form and the catalog chips before the cut; about 50 PF rows stall on it. The
  full `npm run ui-smoke` was not run at E7.4 (the card's gate does not list it). No card owns the row
  rewrite.
- **Starfinder seed-line test naming.** `sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue`
  passes but writes no files; the test that writes the seed-line files is
  `sf_seed_printed_features_equal_the_features_the_converted_records_grant` (quoted from the E7.4
  receipt). `docs/retro/sd38-retrospective.md` section 2 says the out-dir control was not built; E4.8's
  receipt says `create_dir_all` was added to both writers. The two disagree and neither is re-run here.
- **Planned deferrals** (not in the Definition of Done): a NET Architecture generator, an in-play dice
  engine, NET-run simulation, vehicle-combat simulation and an NPC/Mook generator ship as print-only
  tables (`decisions.md §22`, DEF-1 to DEF-5).
- **Other game systems.** Solarus Arcanum, Traveller and World of Darkness remain placeholder chips (FSR-C4).

## Does not cover

- Windows and macOS packaged builds, and CI (`pr-tests`, E7.11). The fresh-install acceptance ran the Linux
  `.deb` and AppImage only.
- A full `npm run ui-smoke` run on this tree (see Known Issues).
- Pathfinder and Starfinder beyond the seeds: R-12 covers three Pathfinder render hashes and R-13 the four
  Starfinder seed-line files. Other characters are covered by the test suites in R-9, not by hash.
- Transcription accuracy beyond the sample: E1.V compared a 10% sample of records to page images; records
  outside the sample are covered by the manifest counts (R-4), not read against the page.
- Re-runs: R-8 to R-11 are re-derived from E7.4's recorded logs, not re-run, because E7.7 deleted the
  build directory. Their identity with the current tree rests on the docs-only diff statement above.
- R-2 and R-3 are pinned to `cc9d7d9a3c`; they move with every later commit, and this card's own row is
  not in R-3.
- R-14 is the recorded summary written at E7.6 (2026-10-15T20:17:56Z); the retro log keeps growing and a
  re-run of `scripts/retro.py summary` reads higher.
- Judgments in the prose (what a capability "is") are checked against receipts, not by a command; only
  `R-n` figures carry commands.

## Figures

Each line: value in bold, implementation A (shell, run from the repository root), implementation B
(Python, independent). "Gives" is the output of that implementation when E7.8 ran, 2026-10-15.

- **R-1** build version in the two desktop manifests: **0.18.0,0.18.0**. A: `grep -h -m1 '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'"' '{print $4}' | paste -sd,` gives `0.18.0,0.18.0`. B (Python, independent): `json.load` of both files, key `version` gives `0.18.0,0.18.0`.
- **R-2** commits on tranche-18 past the cut bb6107aee0, to cc9d7d9a3c: **166**. A: `git log --oneline bb6107aee0..cc9d7d9a3c | awk 'END{print NR}'` gives `166`. B (Python, independent): `git rev-list --count` over the same range gives `166`.
- **R-3** kanban rows, then rows `complete`, at cc9d7d9a3c (before this card's own row moved): **70 66**. A: `git show cc9d7d9a3c:docs/release/SD-38-cyberpunk-red/kanban.md | awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++; if ($5 ~ /^ complete *$/) c++ } END { print n, c }'` gives `70 66`. B (Python, independent): split on `|`, id in column 1 and status in column 4 of the same file gives `70 66`.
- **R-4** Cyberpunk RED record kinds in MANIFEST.json, records they expect, kinds shipped print-only: **53 930 8**. A: `jq -r '[(.kinds|length), ([.kinds[].expected]|add), ([.kinds[]|select(.print_only)]|length)]|map(tostring)|join(" ")' data/cyberpunk-red/corpus/MANIFEST.json` gives `53 930 8`. B (Python, independent): `json.load`, `len`, `sum` and a filter over `kinds` gives `53 930 8`.
- **R-5** tracked files under `data/cyberpunk-red`: **55**. A: `git ls-files data/cyberpunk-red | awk 'END{print NR}'` gives `55`. B (Python, independent): `git ls-files` read in Python, non-empty lines gives `55`.
- **R-6** licence file: `open_game_content`, `operator_sign_off.signed_off`: **false true**. A: `jq -r '[.license_declaration.open_game_content, .operator_sign_off.signed_off]|map(tostring)|join(" ")' data/cyberpunk-red/corpus/LICENSE.json` gives `false true`. B (Python, independent): `json.load` of the same file gives `false true`.
- **R-7** ui-smoke rows in `spec.json`, of them Cyberpunk rows: **127 23**. A: `jq -r '[(.rows|length), ([.rows[]|select(.id|test("cyberpunk"))]|length)]|map(tostring)|join(" ")' apps/desktop/scripts/ui-smoke/spec.json` gives `127 23`. B (Python, independent): `json.load`, `len(rows)` and a substring test on `id` gives `127 23`.
- **R-8** `verify.sh` stages passed, stages failed, in the E7.4 full run: **53 0**. A: `awk '/PASS  /{p++} /FAIL  /{f++} END{print p, f+0}' docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.4/logs/verify-sh.out` gives `53 0`. B (Python, independent): regex `^    PASS  ` and `^    FAIL  ` over the same log gives `53 0`.
- **R-9** tests passed: root lib, root full, ingest full, desktop: **3019 6707 1841 892**. A: `for n in root-lib root-full ingest-full desktop; do awk '/^test result:/{for(i=1;i<=NF;i++) if($i=="passed;") s+=$(i-1)} END{printf "%s ", s+0}' docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.4/logs/stage-$n.log; done | awk '{$1=$1; print}'` gives `3019 6707 1841 892`. B (Python, independent): regex `test result: \w+\. (\d+) passed;` summed per log gives `3019 6707 1841 892`.
- **R-10** `test result: FAILED` lines in the same four logs: **0 0 0 0**. A: `for n in root-lib root-full ingest-full desktop; do awk '/^test result: FAILED/{c++} END{printf "%s ", c+0}' docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.4/logs/stage-$n.log; done | awk '{$1=$1; print}'` gives `0 0 0 0`. B (Python, independent): `str.startswith('test result: FAILED')` per log gives `0 0 0 0`.
- **R-11** frontend test files passed: **173/173**. A: `awk '/PASS +frontend-test/{gsub(/[()]/,"",$3); print $3}' docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.4/logs/verify-sh.out` gives `173/173`. B (Python, independent): regex `PASS +frontend-test +\((\d+/\d+) files\)` over the same log gives `173/173`.
- **R-12** Pathfinder seed render hashes (first 8 hex), E7.4 head: **aldric 7b2f2f26 elowen f0230bf0 tomas 2320c050**. A: `awk '{n=$2; sub(/\.json/,"",n); printf "%s%s %s", (k++?" ":""), n, substr($1,1,8)} END{print ""}' docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.4/pf-render/head/sha256.txt` gives `aldric 7b2f2f26 elowen f0230bf0 tomas 2320c050`. B (Python, independent): `str.split` per line of the same file gives `aldric 7b2f2f26 elowen f0230bf0 tomas 2320c050`.
- **R-13** Pathfinder render hash pairs equal between E0.6 base and E7.4 head (diff exit status), then Starfinder seed-line hash files equal (diff exit status): **0 0**. A: `diff -q docs/release/SD-38-cyberpunk-red/artifacts/epic_0/E0.6/pf-render/base/sha256.txt docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.4/pf-render/head/sha256.txt >/dev/null; a=$?; diff -q docs/release/SD-38-cyberpunk-red/artifacts/epic_0/E0.6/sf-lines/base/sha256.txt docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.4/sf-lines/head/sha256.txt >/dev/null; echo $a $?` gives `0 0`. B (Python, independent): byte comparison of the same file pairs in Python gives `0 0`.
- **R-14** retro events since 2026-10-13 (recorded summary), corrections, incidents: **122 22 10**. A: `jq -r '[.events.total, .events.by_type.correction, .events.by_type.incident]|map(tostring)|join(" ")' docs/release/SD-38-cyberpunk-red/artifacts/epic_7/E7.6_logs/retro-summary.json` gives `122 22 10`. B (Python, independent): `json.load`, `sum(by_type.values())` gives `122 22 10`.

## Update Eligibility

Set at publish by the release manifest (`tools/release/check_release_manifest.py`; the repo keeps the
version placeholder until then). Not decided in this document.
