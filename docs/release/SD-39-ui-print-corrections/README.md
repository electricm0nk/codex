---
title: SD-39 — UI and Print Corrections — Release Package
status: planning-ready — authored 2026-11-01 from the operator's 12 defects and 16 grill rulings; operator reviews before launch
bundle_id: SD-39
slug: ui-print-corrections
scope: docs/release/SD-39-ui-print-corrections
artifact_type: release-index
canonical_branch: tranche-19
kanban_board: local-file ./kanban.md
target_version: 0.19.0
canonical_source: docs/release/SD-39-ui-print-corrections (this folder)
date: 2026-11-01
---

# SD-39 — UI and Print Corrections — Release Package

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as one long `Workflow`-tool script invoked from a live session. It is NOT run
> with `/loop /batch` and it is NOT a one-shot task.** `workflow-instruction.md §2.4` says how to write
> the script. **🟡 UNATTENDED MODE** (`decisions.md §16`, §24; corrections §25): no questions mid-run, safe defaults
> logged, a report after each epic. One card, E9.4, is an **operator gate** (the printed-sheet eyeball);
> the run waits there and nowhere else. `scope-draft.md` is the *what*; `workflow-instruction.md` is the *how*.

This folder is the canonical surface for SD-39. A harness needs only this folder and the in-repo doc tree.

## 0. Preamble

SD-39 is a corrections release. The operator used the app on 2026-11-01 and dictated twelve defects
(`scope-draft.md §1`), then answered sixteen design questions (`decisions.md §3`). The two largest items
are (1) **printing**, which produces "a jumble of lines" and is replaced by a Rust PDF generator with
per-system templates, and (2) the **equipping mechanic** (body slots, worn vs carried), which the rule-set
import never carried. The rest are UI layout moves, feat guards, a Market that replaces three Add
buttons, a level ledger behind a Show Progression drawer, a per-character Overrides group, and a
PCGen `.pcg` character import on the tool side. Pathfinder 1e gets everything; Starfinder 1e and
Cyberpunk RED get the print engine, the layout moves and the Market shell (`decisions.md §15`).

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-39 |
| Slug | `ui-print-corrections` |
| Canonical branch | `tranche-19`, cut by C1 from `origin/develop` @ `2b14020bc7` (PR #402 merge; `git rev-parse --short origin/develop` on 2026-11-01) |
| Kanban board | local file `./kanban.md` |
| Epics / cards | C, E1–E9 / **65 cards**, one criterion each (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 65) |
| Target version | `0.19.0`, stamped by C1's single commit on 14 surfaces + `current-release.json` (`decisions.md §2`) |
| Dispatch mechanism | `Workflow` tool, one long run (`decisions.md §16.2`) |
| Cadence | N/A |
| Closure gate | 65 cards `complete` (E9.4 needs the operator's signature) → retrospective + cite → sweep → release notes → architecture truth-up → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Source | the operator's defect list and grill rulings (`decisions.md §3`); PCGen pinned oracle for `.pcg` fixtures and SLOTS data (`scripts/pcgen-oracle-pin.env`) |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | The twelve defects verbatim, Definition of Done, epic order (operator: UI first, print last) |
| `decisions.md` | §1–§24: the sixteen rulings R1–R16, defaults, autonomous protocol, closure order |
| `forward-scope-register.md` | Committed, implied, candidate rows; DEF-1…DEF-6 |
| `technical-design.md` | Print pipeline, slot model, ledger schema, Market, overrides, `.pcg` sidecar, with repo line cites |
| `technical-requirements.md` | Prerequisites P-1…P-9, normative requirements NR-1…NR-14 |
| `epic-breakdown.md` | §0 parallel/serial map (leads), per-card criteria with FILES YOU OWN, commands, "does not cover" |
| `risks-and-open-questions.md` | Ranked risks with controls; open questions → defaults |
| `acceptance-and-verification.md` | Gates G-1…G-12 and the artifact map |
| `content-unit-inventory.md` | Figures F-1…F-20 with commands: version surfaces, pickers, slots, seeds, print blocks |
| `workflow-instruction.md` | The *how*: prefix, per-cycle procedure, receipt schema, closure order, rules R1–R19 |
| `progress.md` | Live log; Cycle 0 receipt; epic reports; open blockers |
| `kanban.md` | One row per card (65) |
| `release-notes.md` | Interim body (sentinel `SD-39 INTERIM NOTES`); replaced at E9.8 |
| `artifacts/`, `artifacts/README.md` | Per-epic receipts and evidence; `cycle_0/` holds the authoring notes and the workflow prefix |
| `references/`, `references/README.md` | Doctrine, skills, sibling bundles, retrospectives |

`receipts.md` is created by the closure scripts (E9.9, E9.10).

## 3. In-repo cross-references

- Conduct: `../../../AGENTS.md`, `../../../CLAUDE.md`.
- Doctrine: `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/no-stub-mvp-doctrine.md`, `../../governance/wired-integration-stubs-registry.md`,
  `../../doctrine-external/identifier-discipline.md`.
- Architecture: `../../architecture/` (truthed-up at E9.9).

## 4. Relationship to other release folders

- **SD-38** (`../SD-38-cyberpunk-red/`): predecessor, merged (PR #399, `5804a32e58`). Supplies rules R1–R13
  and the retrospective's six "Changes for SD-39" (`../../retro/sd38-retrospective.md`), carried here as
  R14–R19 (`workflow-instruction.md §12.3`).
- **tranche-17-ui** (`../tranche-17-ui/`): the layout registry `sheetLayout.ts` SD-39 extends; its open
  findings (29 lb draft kit, level-up gating, no-browser eyeball) are candidates FSR-C3…C5.
- **Live peer work:** none (`decisions.md §24`).

## 5. Build version target

`major` 0; `tranche-base` **19** (moves at C1's cut, never at closure); `build` = the GitHub run number,
stamped at publish by `publish-tester-release.yml` (`VERSION="0.19.${GITHUB_RUN_NUMBER}"` after C1).
Repo files read `0.19.0` after C1.

## 6. Author's calls not covered by an operator ruling (review before launch)

| # | Call | Default taken | Where |
|---|---|---|---|
| 1 | Package location | `docs/release/SD-39-…/` (repo tooling and every sibling live here); the overlay's `docs/stc/` is noted as the chassis name | `decisions.md §1` |
| 2 | Narrow window | the 1x4 combat row wraps under the 2x2 below a breakpoint | `decisions.md §7` |
| 3 | "Next level grants" text | moves into the Show Progression drawer | `decisions.md §7` |
| 4 | Override extras | ignore feat budget, ignore alignment restrictions, ignore encumbrance added to the operator's five | `decisions.md §10` |
| 5 | Overridden items are tagged | badge in the UI, marked on the "full" printed sheet | `decisions.md §10` |
| 6 | Unknown `.pcg` items | imported as custom items and listed in an import report | `decisions.md §13` |
| 7 | No converter re-run: tags and `repeatable` already hold the data | card E1.0 proves it | `decisions.md §25` |
| 8 | PDF engine crate | Typst as a library, OFL fonts bundled; the card records the exact versions | `decisions.md §4`, `technical-design.md §1` |
| 9 | Seed renders change in this bundle | additive fields are allowed; every delta is explained field by field in the receipt; silent change is a defect | `decisions.md §18` |

## 7. Initial-package construction

Authored on 2026-11-01 by the planning session (Fable 5.1) in `docs/release/SD-39-ui-print-corrections/`
on `develop`, from the operator's dictated defects and the grill transcript
(`artifacts/cycle_0/defects-and-rulings-2026-11-01.md`). Every repo path below was checked with `ls`/`grep -n`
on `develop` @ `5ef2f17b67` (local) / `2b14020bc7` (origin). The placeholder gate
(`workflow-instruction.md §9`) was run before commit; its output is in `progress.md` Cycle 0.

## 8. Cross-reference

- `../template/template.md`, `../../governance/workflow-instruction-template.md` — the two templates.
- `../../../.claude/skills/stc-authoring/SKILL.md` — the repo-local authoring skill.
