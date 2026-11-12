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
> logged, a report after each epic. **Two cards are operator gates:** E2.G (Aldric's page 1, "step 1"; round 1 answered 2026-11-11, round 2 next)
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
| Epics / cards | C, E0–E5 / **54 cards** (24 + E2.4 by E2.MC + E2.5 at E2.G round 1 + 12 by the round-1 replan + E3.6 from the operator's answers, `decisions.md §23` + E2.15 discovered by E2.7 + 10 by the round-2 replan + 3 UI cards + E2.27, `decisions.md §24`), one criterion each (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 54) |
| Target version | `0.20.0`, stamped by C1's single commit on 14 surfaces + `current-release.json` (`decisions.md §2`) |
| Dispatch mechanism | `Workflow` tool, one long run (`decisions.md §13.2`); ends cleanly at each gate |
| Cadence | N/A |
| Closure gate | 54 cards `complete` (E2.G and E5.2 need the operator's signature) → retrospective + cite → sweep → release notes → architecture truth-up + critic → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Source | the operator's brief (`artifacts/cycle_0/operator-brief-2026-11-07.md`); PCGen pinned oracle for the reference picture only (`scripts/pcgen-oracle-pin.env`) |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | The defect in the operator's words, the code's state today, the Definition of Done, epic order |
| `decisions.md` | §1–§23: the seven rulings R1–R7, the three layers in code, vocabulary, presets, density, look rules, unattended protocol with safe defaults SD-a…SD-p, closure order, pre-launch corrections, the operator's answers of 2026-11-07 (§22), the operator's E2.G round-1 redesign of page 1 and the checkbox dialog with the five questions it raised and the operator's same-day answers (§23.6, §23.7) |
| `forward-scope-register.md` | Committed, implied, candidate rows; DEF-1…DEF-4; FSR-C1 is Committed (COM-9), owned by card E0.2 |
| `technical-design.md` | Today's module map with line counts; the model, dispatcher and grid; the vocabulary fixture; the proof harness; page 1's composition and data sources (redesigned, `decisions.md §23`); sections and the checkbox dialog |
| `technical-requirements.md` | Prerequisites P-1…P-9, normative requirements NR-1…NR-16 |
| `epic-breakdown.md` | §0 parallel/serial map (leads), per-card criteria with FILES YOU OWN, acceptance commands, "does not cover" |
| `risks-and-open-questions.md` | Ranked risks with controls; open questions → defaults |
| `acceptance-and-verification.md` | Gates G-1…G-9 and the artifact map |
| `content-unit-inventory.md` | Figures F-1…F-29 with commands: version surfaces, blocks, templates, pages, PCGen sheets, UI ids, targets |
| `workflow-instruction.md` | The *how*: prefix, per-cycle procedure with the look-at-your-output step, receipt schema, closure order, rules R2–R24 |
| `progress.md` | Live log; Cycle 0 receipt; epic reports; open blockers |
| `kanban.md` | One row per card (54) |
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

Rows 5, 6 and 9 were the package's three open questions; the operator answered them on 2026-11-07
(`decisions.md §22`: "1: B", "2: A", "3: No hard limits").

| # | Call | Default taken | Where |
|---|---|---|---|
| 1 | Package location | `docs/release/SD-40-…/` (repo tooling and every sibling live here) | `decisions.md §1` |
| 2 | Grid width and the model's shapes | 12 columns; `id` / `look` / `place` on `SheetBlock`, `columns` on `SheetDocument`; `columns == 1` keeps today's loop for SF/CPR | `decisions.md §4` |
| 3 | Vocabulary mechanism | a JSON fixture read by a Rust test and a TS test | `decisions.md §5` |
| 4 | Combat block split along the UI's box ids | yes | `decisions.md §8` |
| 5 | Preset semantics | **Resolved, operator answer "2: A" (2026-11-07):** Spellbook = the spell pages alone; Full = everything; SF/CPR derive their old options from the preset. **Superseded 2026-11-11** by the checkbox ruling (`decisions.md §23.4`); Spellbook stays spells-only | `decisions.md §9`, §22, §23 |
| 6 | Density | **Resolved, operator answer "3: No hard limits" (2026-11-07):** no page-count ceiling; page counts recorded per case in every receipt (before → after); the operator judges density by eye at E2.G and E5.2; Letter-first design | `decisions.md §10`, §22 |
| 7 | Look rules | black and white, Liberation Sans only, 8 pt floor, write-in list | `decisions.md §11` |
| 8 | Feats print compact everywhere; Full picks a `feats-full` look | **Superseded 2026-11-11:** no Full preset; section 2 prints feats with their whole text, one look | `epic-breakdown.md` E3.1 |
| 9 | **The 14 stale ui-smoke rows** (SD-39 retro change 5 says "give them an owner in SD-40") | **Resolved, operator answer "1: B" (2026-11-07):** card E0.2 (opus, side lane `smoke` parallel to E1) repairs them; it lands before E4.2 (both write `spec.json`) | `forward-scope-register.md` FSR-C1 / COM-9; `epic-breakdown.md` E0.2; `decisions.md §22` |
| 10 | Catch-up to `develop` | the lead rebases `tranche-20` between epics, once, after SD-39 merges | `decisions.md` SD-o |
| 11 | Presets → checkboxes; page 1 totals only; E2.4's fate | **Resolved by the operator at E2.G round 1 (2026-11-11):** six section checkboxes; page 1 redesigned; E2.4 rewritten as page 1's overflow rule; five questions answered the same day (§23.7: one damage total; known languages + one blank line; add a Backstory field, card E3.6; placement and footer as proposed) | `decisions.md §23` |

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
