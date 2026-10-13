---
title: SD-38 — Cyberpunk RED — Release Package
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
bundle_id: SD-38
slug: cyberpunk-red
scope: docs/release/SD-38-cyberpunk-red
artifact_type: release-index
canonical_branch: tranche-18
kanban_board: local-file ./kanban.md
target_version: 0.18.0
canonical_source: docs/release/SD-38-cyberpunk-red (this folder)
date: 2026-10-13
---

# SD-38 — Cyberpunk RED — Release Package

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as one long `Workflow`-tool script invoked from a live session. It is NOT run
> with `/loop /batch` and it is NOT a one-shot task.** To create the script, see
> `workflow-instruction.md §2.4`. **🟡 UNATTENDED MODE applies:** the operator is travelling for a few
> weeks. Do not ask questions. Take the safe defaults in `decisions.md §16.1`. Escalate only the card
> that is blocked, report after each epic, and never open the PR while any card is open.
> `scope-draft.md` is the *what*; `workflow-instruction.md` is the *how*.

This folder is the canonical surface for SD-38. A harness needs only this folder, the in-repo doc
tree, and the source at `/home/ubuntu/workspace/inbox/cyberpunk-red/` (never copied into git).

## 0. Preamble

SD-38 adds **Cyberpunk RED** (corebook v1.22) as Campaign Codex's third game system: creation by the
book's three methods, a sheet whose every number is an engine row, improvement after play, and every
player and GM table browsable. The data is transcribed from the PDF by Sonnet lanes and checked against
page images by an Opus lane; there is no PCGen data and no converter (`decisions.md §23`). Pathfinder 1e
and Starfinder 1e stay **byte-identical**.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-38 |
| Slug | `cyberpunk-red` |
| Canonical branch | `tranche-18`, cut by C1 from `origin/develop` after PR #398 merges (`decisions.md §2`) |
| Kanban board | local file `./kanban.md` |
| Epics / cards | C, E0–E7 / **68 cards**, one criterion each (`kanban.md` row check) |
| Target version | `0.18.0`, stamped by C1's single commit on 14 surfaces + `current-release.json`. Published triple `0.18.<run>` resolves at the first tester publish after the operator merges |
| Dispatch mechanism | `Workflow` tool, one long run (`decisions.md §16.2`) |
| Cadence | N/A |
| Closure gate | 68 cards `complete` → retrospective + cite → sweep → release notes → architecture truth-up → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Source | PDF text, 459 page files (CUI F-19); no PCGen oracle |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | Operator intent, Definition of Done, source, epic order |
| `decisions.md` | §1–§23: operator rulings, DEFAULTs, D-1…D-12, A-1…A-11, deferrals |
| `forward-scope-register.md` | Committed, implied, candidate rows; DEF-1…DEF-5 |
| `technical-design.md` | Seam, data layout, engine, adapter, desktop, packaging, with repo line cites |
| `technical-requirements.md` | Prerequisites P-1…P-10, normative requirements NR-1…NR-12 |
| `epic-breakdown.md` | §0 parallel/serial map (leads), cross-bundle section, per-card criteria with FILES YOU OWN, commands, "does not cover" |
| `risks-and-open-questions.md` | Ranked risks with controls; open questions → defaults |
| `acceptance-and-verification.md` | Gates G-1…G-11 and the artifact map |
| `content-unit-inventory.md` | Figures F-1…F-88 with commands, the MANIFEST kind table, garbled pages, UNCLEAR list, seeds |
| `workflow-instruction.md` | The *how*: unattended callout, prefix, per-cycle procedure, receipt schema, corrected closure, rules R1–R13 |
| `progress.md` | Live log; Cycle 0 unattended receipt; epic reports; open blockers |
| `kanban.md` | One row per card |
| `release-notes.md` | Interim body (sentinel `SD-38 INTERIM NOTES`, resolves in CI from C1 on); replaced at closure (E7.8) |
| `artifacts/`, `artifacts/README.md` | Per-epic receipts and evidence; `cycle_0/inventory/` holds the seven inventories |
| `references/`, `references/README.md` | Doctrine, skills, sibling bundles, retrospectives |

`receipts.md` is created by the closure scripts (E7.9, E7.10).

## 3. In-repo cross-references

- Conduct: `../../../AGENTS.md`, `../../../CLAUDE.md`.
- Doctrine: `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/no-stub-mvp-doctrine.md`, `../../governance/wired-integration-stubs-registry.md`,
  `../../governance/license-matrix.md`, `../../doctrine-external/identifier-discipline.md`.
- Architecture: `../../architecture/` (truthed-up at E7.9).

## 4. Relationship to other release folders

- **SD-37** (`../SD-37-starfinder-1e/`): predecessor, merged (PR #397). Supplies the `GameSystem` seam,
  the adapter resolver, the `sf_*` module pattern, the PF render-hash harness and SF seed-lines output
  (G-1), and the retrospective changes that become rules R8–R13.
- **tranche-17-ui** (PR #398, open): its layout registry and restore-starters mechanism are reused; C1
  waits for its merge.
- No other bundle is live (`epic-breakdown.md` cross-bundle section).

## 5. Build version target

`major` 0; `tranche-base` **18** (a new bare `tranche-18` cut moves the tranche digit at C1, never at
closure); `build` = the GitHub run number, stamped at publish by `publish-tester-release.yml`
(`VERSION="0.18.${GITHUB_RUN_NUMBER}"` after C1). Repo files read `0.18.0` after C1.

## 6. Open rulings for the operator (review on return)

Operator rulings (licence, scope, mode, no automation) are on record (`decisions.md §3`). Everything
below was decided without the operator and can be revisited; each decision says how.

| # | Ruling | Default taken | Where |
|---|---|---|---|
| 1 | Package location | `docs/release/SD-38-cyberpunk-red/`; flagged deviation from the overlay's `docs/stc/` | `decisions.md §1` |
| 2 | Branch / version / pointer | `tranche-18` after #398; `0.18.0` on 14 surfaces (Cargo.lock in, `test_resolve_release_notes.py` out) + `current-release.json` | `§2` |
| 3 | Licence deviation | whole text ships; LICENSE.json signed off; DEVIATION row; no PI screen | `§4` |
| 4 | GM tables | print-only; Mooks/Encounters/Beat Charts inventoried by E1.8 first | `§5` |
| 5 | Epic spine | E6 moved into the E2 window; shared files serialised by dependency | `§7` |
| 6 | No sheet-rule package for CPR | `sheet_rules_relative() -> Option`, `None` | `§8` |
| 7 | Data layout | one JSON array per kind + MANIFEST + LICENSE | `§11` |
| 8 | Seeds | Streetrat Solo, Complete Package Netrunner, Edgerunner Medtech | `§14` |
| 9 | Model tiering | Opus engine/UI/verify; Sonnet transcription/long waits; Haiku housekeeping | `§15` |
| 10 | Unattended protocol, epic reports, quota, crash-resume | per `§16` | `§16` |
| 11 | D-1…D-12 (range-DV mapping, brain damage, Air Pistol, DV29, therapy cap, "(−2 EMP)", Maker/Medicine points, Backup 16/15/14, Corp Kind 1d10, price oddities, slot enforcement, no encumbrance) | the brief's defaults | `§20` |
| 12 | **A-1…A-11 — author defaults not in the brief** (multiclass 60 IP; Moto/Exec/Cryo picks; post-creation HL entry or roll; EMP clamp at 0; extra eye/arm slots; attachment 1 slot; deck slots both apply + over-capacity refused; Linear Frame Sigma plugs; computed starting cash; `cpr/mod.rs` lane anchors; envelope optional field) | flagged for review | `§21` |
| 13 | Deferrals DEF-1…DEF-5 | not in DoD | `§22` |

## 7. Initial-package construction

Authored on 2026-10-13 in a staging directory from the planner's brief (Fable 5.1) and seven read-only
inventories of the PDF text, by an Opus 5.5 sub-agent; the lead copies it into
`docs/release/SD-38-cyberpunk-red/`. The inventories ship in `artifacts/cycle_0/inventory/` so every
command in `content-unit-inventory.md` stays runnable.

## 8. Cross-reference

- `../../governance/workflow-instruction-template.md`: the template `workflow-instruction.md` is
  authored from. Its §11 closure order is defective; this bundle's copy uses the corrected order.
- `../template/template.md`: the chassis template.
