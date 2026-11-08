---
canonical: true
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
date: 2026-11-07
status: planning-ready
---

# SD-40 Progress

## Summary

| Field | Value |
|---|---|
| Cards complete | 2 of 24 (`awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md \| awk 'END{print NR}'`) |
| Run id | — (written at launch, `workflow-instruction.md §2.4` item 9) |
| Pinned `tranche-20` SHA | `3ceeb63b6622061d9dde1e0068df4b3d4065d626` (C1 version commit, pushed 2026-11-07; was `d4489b3557a1fa6f59be53ffc16561990c7e6715` at the cut) |

## Run handle

(the lead writes the run id and script path here the moment the run starts; after each gate the
resume `START=` card)

| Run | Started | Start card | Script | Outcome |
|---|---|---|---|---|
| wf_00f61ec5-013 | 2026-11-07 | C1 | `artifacts/cycle_0/sd40-workflow.js` (session copy in the lead's scratchpad) | running; stops at E2.G |

## Cycle 0 — package authored (2026-11-07, planning session, Fable 5.1)

- Source: the lead's brief carrying the operator's seven rulings (`artifacts/cycle_0/operator-brief-2026-11-07.md`).
- Repo facts checked on `tranche-19` @ `d4489b3557`: 16 block builders, 58 Typst lines in two templates,
  one generic table per block, no grid, 24 text goldens, `Paper::Letter` already present, the dialog's
  three checkboxes, `SheetBoxId` 13 / `SHEET_TABS` 8, the PCGen installed launcher runs headlessly
  (`./pcgen --help`), the oracle on pin, poppler present, 14 version surfaces by `git show 49d598c13a`.
- Author calls without an operator ruling: `README.md §6` rows 1–10; rows 5, 6 and 9
  were the open questions, answered by the operator on 2026-11-07 (below).
- Brief corrections: `decisions.md §21` (Letter already exists; 16 builders not 15; the bare jar does not
  run; the version grep's lockfile false positives).
- **UNATTENDED MODE after launch:** `decisions.md §13`; E2.G and E5.2 are the two planned waits.
- Peer tree `.worktrees/sd39-main` present and untouched (`decisions.md §20`).

### Placeholder gate output (author, 2026-11-07)

Command: `grep -rhoE '<[a-z_-]+>' docs/release/SD-40-pf-sheet-design/*.md | sort | uniq -c | sort -rn`
run before commit; the list is pasted in `artifacts/cycle_0/C0_cycle_receipt.md`. Every token is in the
`workflow-instruction.md §9` allowed list; zero bugs. Deferred values: `(E0.1)` figures in
`content-unit-inventory.md` F-13, F-14; `0.20.<run>`; the run id.

## Operator answers (2026-11-07)

`decisions.md §22`: "1: B" (card E0.2 owns the 14 stale ui-smoke rows; E4.2 waits on it), "2: A"
(Spellbook prints spell pages only; Full prints everything), "3: No hard limits" (no page-count
ceiling; page counts recorded per receipt; the operator's eye at E2.G and E5.2). Cards 23 → 24
(`awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`).

## Cycle log

| Cycle | Card | Status | SHA | Receipt |
|---|---|---|---|---|
| C0 | C0 | complete | this commit | artifacts/cycle_0/C0_cycle_receipt.md |
| C1 | C1 | complete | 3ceeb63b66 | artifacts/cycle_1/C1_cycle_receipt.md |
| E0.1 | E0.1 | complete | this commit | artifacts/epic_0/E0.1_cycle_receipt.md |
| E1.1 | E1.1 | complete | ad412a4a88 + receipt commit | artifacts/epic_1/E1.1_cycle_receipt.md |
| E1.2 | E1.2 | complete | (this commit) | artifacts/epic_1/E1.2_cycle_receipt.md |

## Epic reports

(the merge-check card of each epic appends its block here: cards, refuted claims and fixes, golden
deltas, what was looked at, retro summary, worktrees swept)

## Lead's dispatch log

(one line per dispatch: card, model, `git status --porcelain` / `git worktree list` readings (R19), the
R21 RED output path, the R22 grep)

## Open blockers

(none)
