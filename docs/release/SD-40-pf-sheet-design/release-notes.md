---
canonical: true
bundle_id: SD-40
slug: pf-sheet-design
tranche: 20
date: 2026-11-13
status: final
---

# Campaign Codex 0.20 — Pathfinder Sheet Design

The first printed Pathfinder page, designed: a page grid, one look per block, and a page 1 the operator
signed as usable. This is the **early-merge** form of SD-40. The operator merged `tranche-20` to `develop`
on 2026-11-13 ("SD-40 is good enough for now, let's go ahead and issue the PR."), with the E3, E4 and E5
cards still waiting. Only E0 to E2 shipped. Every card that did not ship is listed under "Known Issues".

Figures are listed under "Figures" as `R-n`, each with the command that produced it (run from the
repository root, 2026-11-13).

## Summary

SD-40 gives the Pathfinder print a page-grid layout model, per-block looks, and a redesigned page 1
(Aldric, seven languages, on A4 and US Letter), backed by new engine rows (AC parts, swim and fly speeds,
resistances, conditional notes, per-weapon damage, remaining weight, known languages). 43 of 59 kanban
rows are `complete` (R-1); the 16 that are not are named in "Known Issues". The operator signed page 1
usable on both papers at E2.G round 3 (R-2). Starfinder and Cyberpunk RED print as before.

## User-Visible Changes

- **Page-1 design (E1, E2).** The printed Pathfinder sheet is laid out on a 12-column page grid. Page 1 holds
  a portrait with Character Name and Campaign, abilities, saves, attack, speed (Land, Swim, Fly), Armor Class
  with its breakdown filled from engine rows, Hit Points, Initiative, Encumbrance and Conditions in one
  square block, a one-column skills list, weapons with a damage total and range, known Languages as a
  comma list, a fixed-size Conditional Bonuses box, and a Resistances block. 15 look templates exist (R-3).
- **Sections.** Abilities and Feats, Inventory, Spellbook, Bio and Math each open on their own page under a
  prominent title (E2.23).
- **Languages field (E2.27).** A Languages field in the app, saved with the bio, feeds the sheet's known
  languages; with nothing stored the sheet shows Common.
- **Swim and Fly in the app (E2.24).** Swim and Fly tiles beside Land, from the same engine rows as the print.
- **Conditional Bonuses in the app (E2.25).** A Conditional Bonuses list under the Saving Throws totals.
- **Resistances in the app (E2.26).** A Resistances panel in the Defense tab.
- **No hit-point tracking (E2.29).** The Hit Points box shows the maximum only; the DurabilityPanel and the
  hit-point tracking paths are removed. The sheet's `hit_points.max` now counts Toughness and a bonded Toad
  familiar (E2.31), and Toughness scales with Hit Dice as `max(3, HD)` (E2.32).
- **Options menu (E2.30).** Options live in a ☰ menu on every screen; the ⚙ button is removed.
- **Page counts moved.** Elowen's golden print, C1 to E2 round 3 (E2.MC3's recorded measurement): PF none
  5 to 6, spellbook 6 to 7, math 12 to 13, all 14 to 14 on A4 and on Letter. The rise is the section page
  breaks (E2.23). Starfinder and Cyberpunk RED page counts are unchanged.

## Defects Fixed

- **The 14 stale ui-smoke rows** were repaired (E0.2).
- **Wrong or missing sheet numbers (E2.7, E2.15, E2.31, E2.32).** AC is now 10 plus engine-stated parts; a
  custom item's AC bonus enters the AC row and its part; `hit_points.max` counted neither Toughness nor a
  Toad familiar; Toughness did not scale with Hit Dice.
- **Conditional notes from `print: false` rules (E2.28).** Held rules the sheet used to drop (for example
  Vermin Focus on CMD, 13 entries) now appear as conditional notes.
- **A fourth weapon line broke page 1 on Letter (found by E2.MC, fixed by E2.4's overflow rule).**
- **Each merge check's refuted claims** are recorded in the "Epic report" blocks of `progress.md`.

## Operational Notes

- **Version.** The repo value is `0.20.0` on the two desktop manifests (R-4);
  `docs/release/current-release.json` names this file (`python3 tools/release/resolve_release_notes.py
  --repo-root .` prints its path).
- **Notes were interim at merge.** PR #406 merged with the interim body of this file because closure card
  E5.4 (final notes) had not run. The publish run 43456800040 failed at "Resolve and validate tranche
  release notes" for that reason. This file replaces the interim body, and the PR-time release-manifest
  check now runs the same validator.
- **Print engine.** As in 0.19: Typst compiled in, no external binary at print time.
- **Seeds.** The three Pathfinder seed renders are SAME_EXCEPT_NAMED against C1 at each merge check
  (quoted from the E2.MC3 report, "G-1").

## Verification Evidence

- **PR #406 CI** (`gh pr checks 406 -R electricm0nk/codex`, 2026-11-13): Release tooling tests pass (41s),
  Test (PR into develop) pass (52m42s), Validate bound release manifest(s) pass (35s). Those checks passed
  with the interim notes, which is why the publish step was the first to refuse them.
- **E2.G round 3, operator sign-off, 2026-11-13.** Aldric page 1 with seven languages, A4 and Letter, both
  `usable: yes` (R-2). Round 2 (2026-11-12) was `usable: no` on both sheets and produced the round-3 cards
  E2.16 to E2.32.
- **Epic reports** (E1, E2, E2 round 2, E2 round 3) in `progress.md` carry the re-derived test counts and
  their commands. Quoted from the E2.MC3 report at `122e065f87`: `cargo test --locked -j 8 print_` 89 + 1
  passed; desktop full 1274 + 2 passed; root `--lib` 3085 passed; `npm run typecheck && npm test` 193/193;
  0 `FAILED` lines in either log.
- **What was not done:** the widest verify at the tree as merged (E5.1), the five-sheet operator print
  (E5.2), the final-acceptance scan (E5.3), the architecture truth-up and claims critic (E5.4), and graphify
  (E5.5). No packaged build of 0.20 was checked by a card.

## Known Issues

The operator merged early, so these 16 cards are **not in 0.20** (R-1). They are not deferred by ruling;
they are unfinished work in the Definition of Done, and they wait on a successor decision.

- **E3.1** Feats, class features, actions looks (section 2)
- **E3.2** Inventory pages: defense, gear (worn / carried / left behind), money (section 3)
- **E3.3** Daily spells, spellbook, custom, pets looks (sections 2, 4)
- **E3.4** Bio pages: details, Biography, campaign notes, ruled lines (section 5)
- **E3.5** Show-the-math pages: every page-1 breakdown + derivations (section 7, Math)
- **E3.6** Biography and Notes tabs in the app (`bio.json`), two ui-smoke rows
- **E3.7** Progression pages: the Show Progression drawer level by level (section 6)
- **E3.MC** E3 merge check + Epic report
- **E4.1** Sections as lists; `PrintSections`; golden 24 + 6
- **E4.2** Print dialog: eight checkboxes remembered per character; booleans removed; ui-smoke rows
- **E4.MC** E4 merge check + Epic report
- **E5.1** Widest verify + clean build + packaged resources
- **E5.2** OPERATOR GATE: the full set printed, `usable: yes` x5
- **E5.3** Final-acceptance scan, retrospective, sweep
- **E5.4** Release notes, architecture truth-up, claims critic (this file is the release-notes part only)
- **E5.5** Graphify LAST, PR `tranche-20` to `develop`

Consequences a player will see: only page 1 has the new design; pages after page 1 and the Starfinder and
Cyberpunk RED sheets keep the previous looks; the print dialog keeps its three option boxes (E4.2 not
done); there is no Biography or Notes tab (E3.6). Page counts after page 1 are as in the Epic E2 report, not
final.

## Does not cover

- Only Aldric (seven languages) was signed by the operator. Elowen and the other seeds are covered by the
  golden and smoke tests in the Epic reports, not by an operator's print.
- A real update install of 0.20.x, Windows and macOS builds.
- R-1 moves with every later card; it is pinned to the tree at this file's commit.

## Figures

- **R-1** kanban rows, rows `complete`: **59 43**. `awk -F'|' '$2 ~ /^ (C|E)[0-9]/ {n++; if ($5 ~ /^ complete *$/) c++} END{print n, c}' docs/release/SD-40-pf-sheet-design/kanban.md`. Not-complete rows: `awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 !~ /^ complete *$/' docs/release/SD-40-pf-sheet-design/kanban.md | awk 'END{print NR}'` gives 16.
- **R-2** operator `usable: yes` lines for round 3: **2**. `grep -c '^- Aldric .* — usable: yes' docs/release/SD-40-pf-sheet-design/artifacts/epic_2/E2.G_cycle_receipt.md`.
- **R-3** look templates: **15**. `ls apps/desktop/src-tauri/resources/print/looks/*.typ | awk 'END{print NR}'`.
- **R-4** desktop manifest versions: **0.20.0,0.20.0**. `grep -h -m1 '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | awk -F'"' '{print $4}' | paste -sd,`.

## Update Eligibility

Set at publish by the release manifest (`tools/release/check_release_manifest.py`; the repo keeps the
version placeholder until then). Not decided in this document. This bundle did not exercise the update
path; 0.20.x is the next tester publish after this notes fix reaches `develop`.
