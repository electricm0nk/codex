---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready
---

# SD-40 Forward Scope Register

Classes: **Committed** (in the DoD; a shortfall is a blocker), **Structurally implied** (falls out of
committed work), **Candidate** (not in the DoD; a revisit condition is *checked* at E5.3). Only the
operator moves scope between classes. The predecessor is SD-39 (`../SD-39-ui-print-corrections/`), whose
E9.4 receipt hands the design and density of the sheets to SD-40 verbatim: "design and density go to SD-40".

## 1. Committed

| ID | Work | Source | SD-40 cards |
|---|---|---|---|
| COM-1 | Three-layer print architecture: data blocks, looks, layouts on a page grid; system-neutral; SF/CPR unchanged | R1, R2 | E1.1, E1.MC |
| COM-2 | Screen and paper share one block vocabulary | R1 | E1.1 (fixture), E2.1 (Combat split) |
| COM-3 | PCGen reference render of Aldric (std, simple; spellbook if cheap) as the picture to aim at, tool-side only | R3, R4 | E0.1 |
| COM-4 | Step-1 looks: abilities, hit points / initiative / armor class / attack / speed, saves, skills, weapons; page 1 assembled; **operator gate**; redesigned at round 1 (totals only, square block, one-column skills, AC parts filled, languages, conditional notes; `decisions.md §23`) | R5, §23 | E2.1–E2.3, E2.MC, E2.4, E2.5, E2.9–E2.13, E2.MC2, E2.G |
| COM-5 | Remaining PF looks, each section on its own pages: feats, class features, actions, daily spells, custom, pets; Inventory (armor, gear, money); spellbook; Bio (details, campaign notes, ruled lines); show the math with every page-1 breakdown | R5, §23 | E3.1–E3.5, E3.MC |
| COM-6 | Print sections as layout lists; the dialog's seven checkboxes (four on SF1e/CPR; seven since `decisions.md §24.3.7`), remembered per character, first time 1 + 2 + 3 (replaces the four presets, `decisions.md §23.4`); page counts recorded on A4 and Letter (no ceiling, `decisions.md §22`) | R1, R5, R7, §23 | E4.1, E4.2, E4.MC |
| COM-7 | Look-at-your-own-output in the per-cycle procedure; PNGs kept; page counts recorded in the goldens and every receipt | R7 | E1.2 (harness), every look card, E4.1 |
| COM-8 | Final operator look at the full set; closure in the standing order | R4, §15 | E5.1–E5.5 |
| COM-10 | Portrait box top-left of page 1 (was FSR-C4) | `decisions.md §23.1` item 1 | E2.5 |
| COM-12 | A Backstory free-text field in the app's Bio UI, stored in `bio.json`, printed on the Bio page | operator answer Q2, `decisions.md §23.7` | E3.6, E3.4 |
| COM-11 | Engine values page 1 needs and the engine lacked: conditional notes (HP, initiative, saves, AC), AC parts, remaining lb, languages, weapon damage total + range; the screen's TS copies removed | `decisions.md §23.1` items 3a, 4, 6; §23.6 Q1, Q5 | E2.6, E2.7, E2.8, E2.14 |
| COM-9 | The 14 stale red ui-smoke rows (SD-39 FSR-C2) pass by name under `npm run ui-smoke`, each repaired to the current UI or rewritten to match it, never deleted (was FSR-C1; moved here by the operator's answer "1: B", 2026-11-07) | `decisions.md §22` | E0.2 |

## 2. Structurally implied

| ID | Work | Falls out of |
|---|---|---|
| IMP-1 | `print_golden.json` keeps the 24 case names through the section mapping and gains 6 `spellbookOnly` cases (`decisions.md §23.4`); PF page counts re-recorded at each look card with before → after (no page-count ceiling, `decisions.md §22`) | COM-6, COM-7 |
| IMP-2 | `typst.rs`'s world serves a table of look files (one `include_str!` per look) instead of two fixed templates | COM-1 (E1.1) |
| IMP-3 | `PrintRequest` carries `sections`; the TS boundary DTO and `PrintDialog.tsx` follow; the three booleans go; `settings/printSections.ts` remembers ticks per character | COM-6 (E4.1, E4.2) |
| IMP-4 | The Combat data block is split along the UI's box ids (`decisions.md §8`) | COM-2 (E2.1) |
| IMP-5 | four ui-smoke rows for the section checkboxes; inventory regenerated (after E0.2's repair: serial edge on `spec.json`) | COM-6 (E4.2) |
| IMP-6 | Architecture docs (`desktop-app.md`, `testing.md`, `glossary.md`) gain looks, layouts, print sections and the proof harness | E5.4 |
| IMP-7 | `tranche-20` catches up to `develop` once SD-39's PR merges (`decisions.md` SD-o) | R4 |

## 3. Candidates (not in the DoD)

| ID | Work | Source | Why not in SD-40's DoD | Revisit condition (checked at E5.3) |
|---|---|---|---|---|
| FSR-C1 | The 14 stale red ui-smoke rows (SD-39 FSR-C2, condition met) get an owner | `docs/retro/sd39-retrospective.md` change 5: "Give the stale ui-smoke rows (FSR-C2) an owner in SD-40's scope" | **No longer a candidate: moved to Committed (COM-9) by the operator's answer "1: B" (2026-11-07, `decisions.md §22`); owned by card E0.2** | none: E5.3 checks E0.2 is `complete` |
| FSR-C2 | SF1e and CPR adopt the grid and looks | R2 | PF only in SD-40 | a scope draft after SD-40's launch names it: `git log --since=2026-11-07 --diff-filter=AM --name-only --format= -- 'docs/release/SD-*/scope-draft.md' \| sort -u \| grep -v SD-40 \| xargs -r grep -lis 'starfinder.*look\|cyberpunk.*look' \| awk 'END{print NR}'` ≥ 1 |
| FSR-C3 | A serif heading face (a second bundled OFL family) | `decisions.md §11` | determinism and bundle size; not needed for the reference picture | the operator asks after E5.2 |
| FSR-C4 | Portrait image on the printed sheet | asked by the operator at E2.G 2026-11-11 | — | **Committed as COM-10, card E2.5** (`decisions.md §23.1` item 1); none: E5.3 checks E2.5 is `complete` |
| FSR-C8 | Aspect `CombatBonus` notes (attack / AC against a named foe, e.g. Defensive Training) and encumbrance notes on page 1 | E2.6's "does not cover" (census: CombatBonus 314 prose entries; no carrying-capacity `BonusTarget`) | the operator named HP, initiative, encumbrance and saves; encumbrance has no converted target | the operator asks at E2.G round 2 or E5.2 |
| FSR-C5 | PCGen's companion, notes and condensed sheets as print sections | R3 (picture list) | the six named sections are the DoD | the operator asks |
| FSR-C6 | Full `npm run ui-smoke` as a gate | SD-39 R17 | unchanged from SD-39; E0.2 repairs only the 14 named rows | the operator asks |

## 4. Planned capability deferrals

The test (`docs/governance/deferral-revisit-doctrine.md`): was this in the DoD when scoped? No, by
operator ruling in the brief.

| ID | Capability | Operator ruling | Revisit condition (a command, checked at E5.3) | Accepted cost |
|---|---|---|---|---|
| DEF-1 | Pixel-snapshot print tests | SD-39 DEF-4, unchanged; R7 replaces them with agents' eyes + the operator's printer | the FSR-C2 grep for 'pixel snapshot' | a look that prints its words in the wrong place passes the goldens; the eyes and the gates catch it |
| DEF-2 | A layout editor in the app (drag blocks on the sheet) | R1: "adding a print style = adding a list"; layouts are data in the layout file, not a UI | the FSR-C2 grep for 'layout editor' | new presets need a code change |
| DEF-3 | Colour variants (PCGen's blue / green / red sheets) | `decisions.md §11` (DEFAULT) | the operator asks | black and white only |
| DEF-4 | SF1e and CPR looks | R2 | FSR-C2 | their sheets stay the table-of-contents look |

If the operator says any of these was meant to be in the DoD, it becomes a blocker.

## 5. No deferrals for SD-40 blockers

No card in `kanban.md` may be moved here by a cycle. A card that cannot close is `blocked-escalated`.

## 6. Excluded from this scope (the operator passed on them)

Paizo's official sheet as the model (R3 chose PCGen's); presets (replaced by checkboxes, `decisions.md §23.4`;
the earlier exclusion of checkboxes is withdrawn); a converter change of any kind; an engine change beyond
E2.6–E2.8 and E2.14's named rows and fields, which add values, never a new rules engine (`decisions.md §17`, §23.2).
