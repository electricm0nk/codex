---
title: SD-40 — Pathfinder Sheet Design — Release Package
status: planning-ready — authored 2026-11-07 from the operator's brief and 7 rulings; the lead re-reads against the repo before launch (decisions.md §21)
bundle_id: SD-40
slug: pf-sheet-design
scope: docs/release/SD-40-pf-sheet-design
artifact_type: release-index
canonical_branch: tranche-20
kanban_board: local-file ./kanban.md
target_version: 0.20.0
canonical_source: docs/release/SD-40-pf-sheet-design (this folder)
date: 2026-11-07
---

# SD-40 — Pathfinder Sheet Design — Release Package

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as one long `Workflow`-tool script invoked from a live session. It is NOT run
> with `/loop /batch` and it is NOT a one-shot task.** `workflow-instruction.md §2.4` says how to write
> the script. **🟡 UNATTENDED MODE** after launch (`decisions.md §13`): no questions mid-run, safe defaults
> logged, a report after each epic. **Two cards are operator gates:** E2.G (Aldric's page 1, "step 1")
> and E5.2 (the full set); the run waits there and nowhere else. `scope-draft.md` is the *what*;
> `workflow-instruction.md` is the *how*.

This folder is the canonical surface for SD-40. A harness needs only this folder and the in-repo doc tree.

## 0. Preamble

SD-39 built a Rust + Typst print pipeline. The operator printed its three sheets on 2026-11-07, signed
them usable, and said they "lack both artistry and density. They look more like a table of contents than
a character sheet." SD-40 gives the Pathfinder sheet a design: a page grid, one Typst look per block,
and print styles that are layouts (ordered block lists), with PCGen's output sheets as the picture to aim
at and never as code. Every design agent looks at its own rasterized output before claiming done. Step 1
(page 1) stops for the operator's printer; the rest runs unattended to a final operator look.
Starfinder 1e and Cyberpunk RED keep their output unchanged; the architecture lets them adopt the looks
later (`decisions.md §3`).

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-40 |
| Slug | `pf-sheet-design` |
| Canonical branch | `tranche-20`, cut 2026-11-07 from `tranche-19` @ `d4489b3557` (`git merge-base HEAD origin/tranche-19`); catches up to `develop` after SD-39 merges (`decisions.md` SD-o) |
| Kanban board | local file `./kanban.md` |
| Epics / cards | C, E0–E5 / **23 cards**, one criterion each (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 23) |
| Target version | `0.20.0`, stamped by C1's single commit on 14 surfaces + `current-release.json` (`decisions.md §2`) |
| Dispatch mechanism | `Workflow` tool, one long run (`decisions.md §13.2`); ends cleanly at each gate |
| Cadence | N/A |
| Closure gate | 23 cards `complete` (E2.G and E5.2 need the operator's signature) → retrospective + cite → sweep → release notes → architecture truth-up + critic → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Source | the operator's brief (`artifacts/cycle_0/operator-brief-2026-11-07.md`); PCGen pinned oracle for the reference picture only (`scripts/pcgen-oracle-pin.env`) |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | The defect in the operator's words, the code's state today, the Definition of Done, epic order |
| `decisions.md` | §1–§21: the seven rulings R1–R7, the three layers in code, vocabulary, presets, density, look rules, unattended protocol with safe defaults SD-a…SD-p, closure order, pre-launch corrections |
| `forward-scope-register.md` | Committed, implied, candidate rows; DEF-1…DEF-4; FSR-C1 is the operator's question |
| `technical-design.md` | Today's module map with line counts; the model, dispatcher and grid; the vocabulary fixture; the proof harness; page 1's composition; presets and the dialog |
| `technical-requirements.md` | Prerequisites P-1…P-9, normative requirements NR-1…NR-13 |
| `epic-breakdown.md` | §0 parallel/serial map (leads), per-card criteria with FILES YOU OWN, acceptance commands, "does not cover" |
| `risks-and-open-questions.md` | Ranked risks with controls; open questions → defaults |
| `acceptance-and-verification.md` | Gates G-1…G-8 and the artifact map |
| `content-unit-inventory.md` | Figures F-1…F-27 with commands: version surfaces, blocks, templates, pages, PCGen sheets, UI ids, targets |
| `workflow-instruction.md` | The *how*: prefix, per-cycle procedure with the look-at-your-output step, receipt schema, closure order, rules R2–R24 |
| `progress.md` | Live log; Cycle 0 receipt; epic reports; open blockers |
| `kanban.md` | One row per card (23) |
| `release-notes.md` | Interim body (sentinel `SD-40 INTERIM NOTES`); replaced at E5.4 |
| `artifacts/`, `artifacts/README.md` | Per-epic receipts, PNGs and evidence; `cycle_0/` holds the brief, the C0 receipt and the workflow prefix |
| `references/`, `references/README.md` | Doctrine, skills, sibling bundles, retrospectives |

`receipts.md` is created by the closure scripts (E5.4, E5.5).

## 3. In-repo cross-references

- Conduct: `../../../AGENTS.md`, `../../../CLAUDE.md`.
- Doctrine: `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/no-stub-mvp-doctrine.md`, `../../governance/wired-integration-stubs-registry.md`,
  `../../doctrine-external/identifier-discipline.md`.
- Architecture: `../../architecture/` (truthed-up at E5.4).

## 4. Relationship to other release folders

- **SD-39** (`../SD-39-ui-print-corrections/`): predecessor, closing on `tranche-19` in `.worktrees/sd39-main`
  (PR not yet open on 2026-11-07). Supplies the print pipeline (E8), rules R1–R20
  (`workflow-instruction.md §12.1`) and the retrospective's five "Changes for SD-40"
  (`../../retro/sd39-retrospective.md`), carried here as R21–R24 and FSR-C1.
- **tranche-17-ui** (`../tranche-17-ui/`): the layout registry `sheetLayout.ts` whose ids the print blocks
  now share.
- **Live peer work:** `.worktrees/sd39-main` (`decisions.md §20`); never touched by SD-40.

## 5. Build version target

`major` 0; `tranche-base` **20** (moves at C1's cut, never at closure); `build` = the GitHub run number,
stamped at publish by `publish-tester-release.yml` (`VERSION="0.20.${GITHUB_RUN_NUMBER}"` after C1).
Repo files read `0.20.0` after C1.

## 6. Author's calls not covered by an operator ruling (review before launch)

| # | Call | Default taken | Where |
|---|---|---|---|
| 1 | Package location | `docs/release/SD-40-…/` (repo tooling and every sibling live here) | `decisions.md §1` |
| 2 | Grid width and the model's shapes | 12 columns; `id` / `look` / `place` on `SheetBlock`, `columns` on `SheetDocument`; `columns == 1` keeps today's loop for SF/CPR | `decisions.md §4` |
| 3 | Vocabulary mechanism | a JSON fixture read by a Rust test and a TS test | `decisions.md §5` |
| 4 | Combat block split along the UI's box ids | yes | `decisions.md §8` |
| 5 | Preset semantics | Spellbook = the spellbook pages alone; Full = everything; SF/CPR derive their old options from the preset | `decisions.md §9` |
| 6 | Density ceilings before E0.1 measures | Aldric Basic ≤ 2, Elowen Basic ≤ 3, …; Letter-first design | `decisions.md §10` |
| 7 | Look rules | black and white, Liberation Sans only, 8 pt floor, write-in list | `decisions.md §11` |
| 8 | Feats print compact everywhere; Full picks a `feats-full` look | one look per preset choice, chosen by the list | `epic-breakdown.md` E3.1, E4.1 |
| 9 | **The 14 stale ui-smoke rows** (SD-39 retro change 5 says "give them an owner in SD-40"; the operator asked for a small print bundle) | **operator question:** recommended answer = not in SD-40; route to the next corrections bundle; if "include", one Sonnet card E5.0 before E5.1 | `forward-scope-register.md` FSR-C1 |
| 10 | Catch-up to `develop` | the lead rebases `tranche-20` between epics, once, after SD-39 merges | `decisions.md` SD-o |

## 7. Initial-package construction

Authored on 2026-11-07 by the planning session (Fable 5.1) in `docs/release/SD-40-pf-sheet-design/` on
`tranche-20` in `.worktrees/sd40-main`, from the lead's brief carrying the operator's rulings
(`artifacts/cycle_0/operator-brief-2026-11-07.md`). Every repo path was checked with `ls`/`grep -n`/`cat` on
`tranche-19` @ `d4489b3557`; the print module, the Typst templates, the golden suite, the dialog, the
registry and the PCGen launcher were read, not assumed. The placeholder gate (`workflow-instruction.md §9`)
was run before commit; its output is in `progress.md` Cycle 0.

## 8. Cross-reference

- `../template/template.md`, `../../governance/workflow-instruction-template.md` — the two templates.
- `../../../.claude/skills/stc-authoring/SKILL.md` — the repo-local authoring skill.
