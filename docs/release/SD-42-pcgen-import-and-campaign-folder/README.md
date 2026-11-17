---
title: SD-42 — PCGen Import Accuracy and the Campaign Folder — Release Package
status: planning-ready — authored 2026-11-16 from the operator's intake of 2026-11-15 and rulings Q2–Q25 of 2026-11-16 (decisions.md §3); the import root causes are hypotheses until E1.1 (technical-design.md §1); the lead re-reads against the repo before launch
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
scope: docs/release/SD-42-pcgen-import-and-campaign-folder
artifact_type: release-index
canonical_branch: tranche-22
kanban_board: local-file ./kanban.md
target_version: 0.22.0
canonical_source: docs/release/SD-42-pcgen-import-and-campaign-folder (this folder)
date: 2026-11-16
---

# SD-42 — PCGen Import Accuracy and the Campaign Folder — Release Package

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as one long `Workflow`-tool script invoked from a live session. It is NOT run with
> `/loop /batch` and it is NOT a one-shot task.** `workflow-instruction.md §2.4` says how to write the script.
> **🟡 UNATTENDED MODE** after launch (`decisions.md §12`): no questions mid-run, safe defaults logged, a report
> after each epic. **One card is an operator gate:** E5.G (the imported test character; the Campaign Manager;
> the editor); the run waits there and nowhere else. **Two phases:** three Phase-1 lanes (import converter,
> campaign folder, editor) run now beside SD-41 on files none of its open cards own; Phase 2 (every write to
> `character_hub.rs`) waits for SD-41 to merge (`epic-breakdown.md §0`, `decisions.md §4`). **The operator's
> test `.pcg` never enters the repo** (`decisions.md §10`; the PII check runs on every commit). `scope-draft.md`
> is the *what*; `workflow-instruction.md` is the *how*.

This folder is the canonical surface for SD-42. A harness needs only this folder and the in-repo doc tree.

## 0. Preamble

The operator imported a PCGen character into alpha v0.20.152 and got the wrong gear: weapons that read as the
default character's longsword, armor only partly present, a stack of 84 rations listed 84 times. Reading the
converter (`crates/codex-ingest/src/pcg_import/map.rs`) shows why, as hypotheses E1.1 proves first: the key is
looked up base-item first so a custom name is lost, only `+N` eqmods survive, one row is emitted per copy with a
cap of 99, and the worn item's slot and container are never read. The rulings of 2026-11-16 widen the fix to
the whole character for Pathfinder and Starfinder, proved against PCGen's own engine on the operator's file
and seven PCGen sample files, with nothing dropped. The same rulings turn the Campaign Manager into what the
operator wants it to be: a plain folder of Obsidian-compatible Markdown that any sync tool can share, with
the creator as member #1, a help screen, migration, Party Resources, an Adventure Log, Maps with pins, a Wiki
with backlinks and search, a shared `Characters/` backup guarded against the two-master trap, the DM toolkit
reading the party from it, and a CodeMirror 6 live-preview editor that never rewrites a file.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-42 |
| Slug | `pcgen-import-and-campaign-folder` |
| Canonical branch | `tranche-22`, cut 2026-11-16 from `origin/tranche-21` @ `1046daefc0` (`git merge-base HEAD origin/tranche-21`); catches up to `develop` after SD-41 merges (`decisions.md §4`, SD-m) |
| Kanban board | local file `./kanban.md` |
| Epics / cards | C, E1–E5 / **48 cards** (2 + 10 + 14 + 4 + 13 + 5: 37 at authoring + E1.8 discovered by E1.2 + E1.9 discovered by E1.5 + E2.12, E2.13, E4.6 added by the operator on 2026-11-17, `decisions.md §26` + E4.7–E4.11 added by the operator later that day, `decisions.md §27` + E4.12 for GitHub issue #409, `decisions.md §28`), one criterion each (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` → 48) |
| Target version | `0.22.0`, stamped by C1's single commit on the 14 surfaces + `current-release.json` (`decisions.md §2`) |
| Dispatch mechanism | `Workflow` tool, one long run per phase (`decisions.md §12.2`); Phase 1 = three parallel lanes; ends cleanly at E5.G |
| Cadence | N/A |
| Closure gate | 48 cards `complete` (E5.G needs the operator's three `usable: yes` lines; E4.5's receipt holds the local proof run's counts, Q9) → retrospective + cite → sweep → release notes → architecture truth-up + critic → **graphify LAST** → **PR last**; the operator merges (`workflow-instruction.md §11`) |
| Source | the operator's intake and rulings (`artifacts/cycle_0/operator-brief-2026-11-16.md`); the census (`artifacts/epic_0/census.json`); PCGen's sample files and engine as a test oracle only (`decisions.md §22`) |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | The request in the operator's words, the Definition of Done (A import, B Campaign Manager), epic order, the one-bundle call |
| `decisions.md` | §3 every ruling verbatim with its cards; §4 the SD-41 partition; §5–§10 the import (stacks, properties, nothing dropped, Starfinder, parity, the local test file); §11–§18 the campaign folder (layout, run protocol with safe defaults SD-a…SD-s, members/Drive removal/help, migration, the four tabs, `Characters/` two-master, DM toolkit, the editor and its packages); §19 fences; §20 corrections; §21 the operator questions with defaults; §22–§25 standing rules and the one-bundle decision; §26 the three cards the operator added mid-run (2026-11-17); §27 the five gear cards (bag of holding / haversack weight, slot kinds, Customize = rename + modifiers with the redundant list dropped, detail panel, stacks as one row + split by moving) added later that day; §28 the issue #409 card (Linux glibc floor 2.35, gated) |
| `forward-scope-register.md` | Committed COM-1…COM-20, implied IMP-1…IMP-9, candidates FSR-C1…C8, deferrals DEF-1…DEF-5 |
| `technical-design.md` | Today's module map with line counts; **§1 the root-cause hypotheses H1–H7 with file:line**; the import after SD-42; Starfinder; parity; the folder commands; the editor; the tests the bundle adds |
| `technical-requirements.md` | Prerequisites P-1…P-12, normative requirements NR-1…NR-18, out of scope |
| `epic-breakdown.md` | §0 parallel/serial map (leads) with the lanes and the SD-41 partition; per-card criteria with FILES YOU OWN, acceptance commands, "does not cover" |
| `risks-and-open-questions.md` | Ranked risks R-1…R-23 with controls; the operator questions with recommended answers; **§4 the split not taken** |
| `acceptance-and-verification.md` | Gates G-1…G-9; **the traceability table ruling → card → acceptance row**; the artifact map |
| `content-unit-inventory.md` | Figures F-1…F-33 with their `census.json` keys and commands |
| `workflow-instruction.md` | The *how*: the prefix (with the PII check and the unique `RUN_DESKTOP_AGENT`), per-cycle procedure, receipt schema, closure order, rules R2–R28 |
| `progress.md` | Live log; Cycle 0 receipt; epic reports; open blockers |
| `kanban.md` | One row per card (48) |
| `release-notes.md` | Interim body (sentinel `SD-42 INTERIM NOTES`); replaced at E5.3 |
| `artifacts/`, `artifacts/README.md` | Per-epic receipts and evidence; `cycle_0/` holds the brief and the C0 receipt; `epic_0/` the census script and its output |
| `references/`, `references/README.md` | Doctrine, skills, sibling bundles, retrospectives, the oracle, the third-party packages |

`receipts.md` is created by the closure scripts (E5.3, E5.4).

## 3. In-repo cross-references

- Conduct: `../../../AGENTS.md`, `../../../CLAUDE.md`.
- Doctrine: `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/no-stub-mvp-doctrine.md`, `../../governance/wired-integration-stubs-registry.md`,
  `../../doctrine-external/identifier-discipline.md`.
- Architecture: `../../architecture/` (`pcg-import.md`, `persistence.md`, `desktop-app.md`, `testing.md`; truthed-up at E5.3).

## 4. Relationship to other release folders

- **SD-41** (`../SD-41-gear-and-equipment-sets/`): predecessor, **live** on `tranche-21` in `.worktrees/sd41-main`
  (49 of 56 cards complete on 2026-11-16; open: E3.12, E4.MC, E4.G, E5.1–E5.4). Its open cards own
  `character_hub.rs`, `BuyCustomDialog.tsx`, `scripts/verify-baselines.env`, `docs/architecture/**`; SD-42's
  Phase 1 touches none of them (`decisions.md §4`, G-3). SD-42 builds on SD-41's stacks, `CopyFacts`, Buy Custom
  and the campaign mirror.
- **SD-40** (`../SD-40-pf-sheet-design/`): the Notes tab the import's Notes block lands in.
- **SD-37** (`../SD-37-starfinder-1e/`): the SF engine and the explained-ledger parity pattern.
- **SD-33** (`../SD-33-computed-value-verification/`): the PCGen export harness E1.7 reuses.

## 5. Build version target

`major` 0; `tranche-base` **22** (moves at C1's cut, never at closure); `build` = the GitHub run number, stamped
at publish by `publish-tester-release.yml` (`VERSION="0.22.${GITHUB_RUN_NUMBER}"` after C1). Repo files read
`0.22.0` after C1.

## 6. Author's calls not covered by an operator ruling (review before launch)

| # | Call | Default taken | Where |
|---|---|---|---|
| 1 | Where the stack count and the copy's custom fields live | the wire gains an `equipmentSets` block in `equipment_sets.json`'s own shape; `CopyFacts` gains `damage_override`, `weight_override_lbs`, `cost_override_copper`, `properties`; the engine reads each in one place | `decisions.md §5, §6, §24` |
| 2 | How a custom PCGen item with eqmods is represented | as SD-41's Buy Custom build (base + material + masterwork + abilities), priced and weighed by the same code; an eqmod Codex lacks is printed text + a Notes line | `decisions.md §6`; SD-e |
| 3 | Where "unmapped" is listed | `bio.notes` under `## Imported from PCGen (<file>, <date>)`, written by the sidecar | `decisions.md §7` |
| 4 | How parity is proved | PCGen's own engine at the pin exports totals per proof file; a committed `explained.tsv` ledger with a stale-row failure (the SD-37 pattern) | `decisions.md §9` |
| 5 | The folder layout and `campaign.json` at the root | `decisions.md §11`; `§21` Q-K | |
| 6 | Where known folders are remembered | `<app data>/campaign-folders.json` | `§21` Q-C |
| 7 | Member #1's name | a required "Your name" field, remembered locally | `§21` Q-B |
| 8 | Adventure Log filename; pin coordinates; image link syntax | `<YYYY-MM-DD> <author> - <title>.md`; fractions; `![](Attachments/<name>)` | `§21` Q-D, Q-E, Q-F |
| 9 | The two-master mechanism | `master.json` {device, counter}; the counter check before every backup write; stand-in never writes | `decisions.md §16` |
| 10 | The editor's packages | the eleven `decisions.md §18` names (CodeMirror 6 + Lezer + `marked`), all MIT; no sanitiser, no React wrapper | `decisions.md §18`; SD-g |
| 11 | One gate, not two | E5.G covers import and the Campaign Manager; every MC leaves screenshots | `§21` Q-H |
| 12 | One bundle, not two | the lanes are file-disjoint and run in parallel; the split point is named | `decisions.md §25`; `risks-and-open-questions.md §4` |
| 13 | The drone sample | imports as its own SF character if the engine computes a Drone, else `Blocked` with the reason | `§21` Q-A |

## 7. Initial-package construction

Authored on 2026-11-16 by the planning session (Fable 5.1) in `docs/release/SD-42-pcgen-import-and-campaign-folder/`
on `tranche-22` in `.worktrees/sd42-plan`, from the lead's brief carrying the operator's intake and rulings
(`artifacts/cycle_0/operator-brief-2026-11-16.md`). Every repo path was checked with `ls`/`grep -n`/`sed -n` on
`tranche-22` @ `1046daefc0`; the census (`artifacts/epic_0/census.py`, run with `python3 -I`) produced every
figure; the operator's file was read for counts only. The import defects were diagnosed by reading the code,
not by running the app; the root causes are hypotheses until E1.1. The placeholder gate
(`workflow-instruction.md §9`) was run before commit; its output is in `progress.md` Cycle 0.

## 8. Cross-reference

- `../template/template.md`, `../../governance/workflow-instruction-template.md` — the two templates.
- `../../../.claude/skills/stc-authoring/SKILL.md` — the repo-local authoring skill.
