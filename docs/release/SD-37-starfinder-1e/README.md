---
title: SD-37 — Starfinder 1e — Release Package
status: planning — authored 2026-10-02 under operator-away defaults; C0.2 (Opus review) pending before launch
bundle_id: SD-37
slug: starfinder-1e
scope: docs/release/SD-37-starfinder-1e
artifact_type: release-index
canonical_branch: tranche/17
kanban_board: local-file ./kanban.md
target_version: 0.17.0
canonical_source: docs/release/SD-37-starfinder-1e (this folder)
date: 2026-10-02
---

# SD-37 — Starfinder 1e — Release Package

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as one long `Workflow`-tool script invoked from a live session. It is NOT
> run with `/loop /batch` and it is NOT a one-shot task.** To create that script, see
> `workflow-instruction.md §2.4`. **UNATTENDED MODE applies:** the operator is away. Do not ask
> questions. Take the safe defaults in `decisions.md §12.1`. Escalate only the card that is
> blocked, and never open the PR while any card is open. `scope-draft.md` is the *what*;
> `workflow-instruction.md` is the *how*.

This folder is the canonical surface for SD-37. A harness needs only this folder and the in-repo
doc tree.

## 0. Preamble

SD-37 makes Campaign Codex generate **Starfinder 1e character sheets**. Sheet totals (EAC, KAC,
Stamina, HP, Resolve, saves, BAB, skills, spells, bulk, credits) are computed from converted
PCGen data by a **generic** chassis, and every other rule is printed. PF1e stays
**byte-identical**. On a parallel, fenced track, SD-37 also moves `src/rules_core/rules_tables`
(180,883 lines; `content-unit-inventory.md` F-10) into a runtime data package.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-37 |
| Slug | `starfinder-1e` |
| Canonical branch | `tranche/17`, cut from `origin/develop` `20bf84a3b2` (PR #394's merge, 2026-09-30); `decisions.md §2`. Local only until card C1 pushes it |
| Kanban board | local file `./kanban.md` |
| Epics / criteria | C, E0–E7, E4a (+ E8 planned deferral, not a card) / **55 cards**, one criterion each (`kanban.md` row check) |
| Target version | `0.17.0`, stamped by **card C1's single commit** across 14 surfaces (CUI F-17). Published triple `0.17.<GitHub run number>`, which **resolves at the first tester publish after C1** |
| Dispatch mechanism | `Workflow` tool, one long run with a resume handle (`decisions.md §12.2`) |
| Cadence | N/A |
| Closure gate | all 55 cards `complete` → retrospective + cite → sweep → release notes → architecture truth-up → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Oracle | PCGen `7f818006e3` (pinned); SF books: 8 in scope, 2 excluded (`content-unit-inventory.md §2`) |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | Intent, Definition of Done, book scope, epic order |
| `decisions.md` | §1–§19, all operator-away defaults with revisit lines |
| `forward-scope-register.md` | Adopted, implied, candidate rows; DEF-1 starship deferral |
| `technical-design.md` | Partition, schema, converter, generic SF chassis, data package, desktop |
| `technical-requirements.md` | Prerequisites P-1…P-9, normative requirements NR-1…NR-12, inherited items |
| `epic-breakdown.md` | §0 parallel/serial map (leads), cross-bundle section, per-card criteria with commands and "does not cover" |
| `risks-and-open-questions.md` | Ranked risks with controls; open questions → defaults |
| `acceptance-and-verification.md` | Bundle gates G-1…G-6 and artifact map |
| `content-unit-inventory.md` | Figures of record F-1…F-20 (two implementations each), book roster, content tuple, seeds |
| `workflow-instruction.md` | The *how*: unattended callout, prefix, fences, per-cycle procedure, receipt schema, corrected closure, rules R1–R7 |
| `progress.md` | Live log; Cycle 0 unattended receipt; safe-default decisions; open blockers |
| `kanban.md` | One row per card |
| `receipts.md` | Closure-script YAML receipts (architecture truth-up, graphify) |
| `artifacts/`, `artifacts/README.md` | Per-epic receipts and evidence |
| `references/`, `references/README.md` | Doctrine, skills, sibling bundles, retrospectives |

`release-notes.md` is written at E7.6. It does not exist yet.

## 3. In-repo cross-references

- Conduct: `../../../AGENTS.md`, `../../../CLAUDE.md`.
- Doctrine: `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/no-stub-mvp-doctrine.md`, `../../governance/wired-integration-stubs-registry.md`,
  `../../governance/license-matrix.md`, `../../governance/ogl-pi-blacklist.md`,
  `../../doctrine-external/identifier-discipline.md`.
- Architecture: `../../architecture/` (truthed-up at E7.7).

## 4. Relationship to other release folders

- **SD-36** (`../SD-36-consolidation/`): predecessor, merged (PR #393). It supplies the crate
  wall, the structural-diff protocol, the seeds and lessons 14–20 (rules R1–R7). Its loose ends
  are card C0.1, not a gate.
- **SD-35** (`../SD-35-corpus-sheet-completion/`): its C2.1 is adopted as E3, and its §11 kept the
  converter for Starfinder.
- **SD-34** (`../SD-34-book-completion/`): its P1s are candidate FSR-C1.

## 5. Build version target

- `major` = 0; `tranche-base` = **17** (a new bare `tranche/17` cut, so the tranche digit moves at
  the cut and never at closure); `build` = the GitHub run number, stamped at publish by
  `publish-tester-release.yml` (`VERSION="0.17.${GITHUB_RUN_NUMBER}"` after C1).
- Repo files read `0.17.0` after C1. The first concrete published value is
  `0.17.<run number>`, recorded in C1's receipt when the first tester publish runs.

## 6. Open rulings (operator-away defaults — review on return)

Each default can be revisited. Its decision says how.

| # | Ruling | Default taken | Where |
|---|---|---|---|
| D1 | Package location | `docs/release/SD-37-starfinder-1e/`; **flagged deviation** from the overlay's `docs/stc/stc-NN` | `decisions.md §1` |
| D2 | Branch / board / version / #395 / SD-36 loose ends | `tranche/17` from `origin/develop`; local `kanban.md`; `0.17.0` via C1's one commit; #395 not a gate; C0.1 housekeeping | `§2` |
| D3 | Epic spine incl. E4a and E8 | E0∥E1→E2→E3→(E4∥E5)→E6→E7; E4a parallel; E8 planned deferral | `§3`, `§17`, `§19` |
| D4 | Book scope | CRB proof, then 7 books; SSRGG and LPJ excluded; licence matrix is E0.2's deliverable | `§4`, `§6` |
| D5 | Paper-sheet rule | Standing; compute totals only | `§5` |
| D6 | Seeds | Soldier 3, Mystic 5, Technomancer 5, Envoy 3 (+ Aldric, Elowen) | `§9` |
| D7 | Predecessor adoptions | C2.1 adopted (E3); FS-2/15/27/28 are candidates | `§10` |
| D8 | Model tiering | Opus default, Sonnet mechanical, Haiku housekeeping; merge checks always Opus; planning was Sonnet → C0.2 Opus review | `§11` |
| D9 | Blockers, unattended protocol, watchdog, quota, crash-resume | Reconciled protocol; one long Workflow run; 85% / 10 M (estimate) stop | `§12` |
| D10 | Closure order | Release notes → arch truth-up → graphify LAST → PR last; template defect flagged (FSR-C9) | `§13` |
| D11 | Per-cycle discipline | TDD, dual audit, four-check audit, receipts, three statuses | `§14` |
| D12 | SD-36 lessons 1–7 | Rules R1–R7 in `workflow-instruction.md §12.1` | `§15` |
| D13 | Falsifiable acceptance | Gates G-1…G-6 | `§16` |
| — | PF data layout | PF unchanged; SF under `data/starfinder-1e/` | `§7` |
| — | Overloaded-field mapping | SF-only mapping table, oracle row per field | `§8` |
| — | Hand-value source | Starfinder Reference Document (no book on the machine) | `§18` |
| — | SF licence sign-off | `operator_sign_off: false` until the operator signs | `§6`, G-5 |

## 7. Initial-package construction

Authored directly in the repo worktree on 2026-10-02 from three read-only research reports
(Starfinder source inventory, engine-reuse map, predecessor lessons). These lived in the authoring
session's scratch space, and their content is folded into this package with re-derived figures.
They are not load-bearing after this commit.

## 8. Cross-reference

- `../../governance/workflow-instruction-template.md`: the template `workflow-instruction.md` is
  authored from. Its §11 closure order is defective (FSR-C9); this bundle's copy is corrected.
- `../template/template.md`: the chassis template. Its §6 closure sub-steps are superseded here by
  `workflow-instruction.md §11`'s corrected order.
