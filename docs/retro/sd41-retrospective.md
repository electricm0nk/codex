---
canonical: true
owner: sd41-e5.2
purpose: SD-41 (Campaign Codex: gear and equipment sets) retrospective, grounded in the retro log and the package's own receipts, written at E5.2.
date: 2026-11-16
bundle: SD-41
branch: tranche-21
---

# SD-41 retrospective

SD-41 gave Pathfinder 1e characters locations for every copy they own (Worn by slot, Carried with nested
containers, Not Carried), named equipment sets with one active set projected into the engine, settled container
capacities, an Unarmed row and per-weapon ammunition, a rebuilt Market (table, sort, Negotiate, Buy Custom, Create,
bulk pieces), campaign folders written through from every saver, and the printed Inventory page and page-1 changes.
Every number below carries the command that produced it. Numbers from `retro.py summary` come from

```
python3 scripts/retro.py summary --since 2026-11-12 --json
```

whose output at writing is kept in
`docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_5/E5.2/retro-summary.json` (`generated_at`
`2026-11-17T04:39:40Z`; the log keeps growing, so a later re-run reads higher). The window also holds SD-40's
closure tail (`sd40-*` actors); the SD-41 figures below are filtered by actor prefix `sd41-` (Python over
`docs/retro/events/*.jsonl`, 943 shards, 0 invalid lines).

**State at writing.** 55 of 58 `kanban.md` rows `complete`; the three that are not are E5.2 (this card) to E5.4,
the closure chain (`cd docs/release/SD-41-gear-and-equipment-sets && awk -F'|' '{ gsub(/\\\|/, "") } $2 ~ /^ (C|E)[0-9]/ && $5 !~ /^ complete *$/ { print $2 }' kanban.md`
-> ` E5.2 `, ` E5.3 `, ` E5.4 ` before this card's row moves; total `awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'` -> 58).
The bundle ran 2026-11-12 to 2026-11-16 on `tranche-21`: 137 non-merge commits past the merge-base with
`develop` (`git log --oneline --no-merges $(git merge-base origin/tranche-21 origin/develop)..origin/tranche-21 | awk 'END{print NR}'`
-> 137 before this card's commit). The package launched with 23 cards
(`git show 2fea61de8f:docs/release/SD-41-gear-and-equipment-sets/kanban.md | awk -F'|' '$2 ~ /^ (C|E)[0-9]/' | awk 'END{print NR}'` -> 23)
and closed with 58.

## What shipped

Epics C, E1–E5.1 (`progress.md` Summary and its four epic reports): the root-crate model (settled containers,
equipment sets and the projection, container loads, the carried-weapon entry point, Unarmed, ammunition, piece
quantities, the campaign mirror, 1,763 unpriced Market rows cut to 356 with every source tried listed) in Phase 1
beside SD-40; the desktop backend (E2), the gear screen, Market and campaign dialog (E3), the print (E4), and the
widest verify plus a clean `.deb` / AppImage (E5.1). The operator signed E4.G's three lines
(`grep -c '^- \(Gear screen\|Inventory page\|Page 1 ([^)]*)\) — usable: yes' docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_4/E4.G_cycle_receipt.md`
-> 3), the gear screen "accepted without screen review" until the release build. 16 cards were discovered inside the
run and finished (`awk -F'|' '/discovered by/ && $2 ~ /^ (C|E)[0-9]/ {print $2}' kanban.md` -> E1.13, E1.15, E2.8–E2.13,
E3.7–E3.10, E3.12, E4.5, E4.6, E5.5); seven more came from operator requests mid-run (E1.12, E1.14, E3.6, E3.11,
E4.2, E4.3, E4.4; the kanban count constant's history names each). SF1e and CPR goldens stayed
byte-identical at E4.MC (`progress.md` E4 report: SF1e ×10 + CPR ×10).

## The log

SD-41's own events: 158 (Python, actor prefix `sd41-`, `ts >= 2026-11-12`): 103 corrections, 47 verification runs,
4 reworks, 4 incidents, 0 deferrals, 0 notes (`awk '/"actor": ?"sd41-/ && /"type": ?"correction"/' docs/retro/events/*.jsonl | awk 'END{print NR}'`
-> 103, agreeing). Verification: 5 of 47 runs failed a stage (clippy 2, `pcgen-residue-gate` 2, `corpus-sweep` 1;
each fixed in the same card). 111 events are agent-written, 47 derived.

### 1. The package was the most-corrected artifact, for the fourth bundle running

Of the 103 corrections, 90 name our own package, rows, acceptance commands, receipts or the lead as the wrong party
(predicate: `subject` matches
`\.md|package|dispatch|receipt|row|epic-breakdown|decisions|inventory|kanban|workflow|acceptance|FILES YOU OWN|G-\d|NR-\d|lead|brief`,
case-insensitive, Python over the raw events). The window's repeat subjects (`corrections.repeat_subjects`) are
`lead` 8, `package` 7, `epic-breakdown.md:E2.5` 5. Two shapes:

- **Acceptance counts that drift:** 39 corrections whose subject, claim or actual mentions
  `passed|filter|R21|acceptance|matches 0`. A filter's "N passed" was written at authoring and later cards added
  tests under the same filter; every merge check found more (E1.MC 3 rows, E2.MC 6, E4.MC 3). E1.MC and E2.MC built
  `acceptance_filter_counts.py` and `desktop_filter_counts.py`, which check every claim against one full log — the
  first SD bundle where this class got a mechanism instead of a caution (AGENTS.md rule 8).
- **Premises the code refuted:** 12 whose text names FILES YOU OWN, a fence, a producer or a missing file. SD-40's
  21 fell to 5 by subject (`FILES YOU OWN` in `subject`), but the class did not vanish; it moved into "the row named
  the wrong file home".

### 2. The dispatch log stayed empty again

`progress.md`'s "Lead's dispatch log" holds its heading and template line only, for the second bundle running. SD-40's
change 1 (R21 as a mechanism) was not built; cards ran RED themselves and logged 11 corrections against `lead`.
A rule whose evidence surface is empty is a warning, not a control.

### 3. A discovered card fell off the end of the run

`incidents.recurring` lists `fence-exit-print-file` (2, both SD-40's). SD-41's four incidents each fired once:
`discovered-card-never-dispatched` (E3.12, found by E3.MC, was never dispatched: the workflow script loops over an
epic's cards fixed at dispatch time, so a card discovered by that epic's own merge check had no slot; E5.2's first
scan stopped on it, the lead dispatched it, it closed in one cycle), `orphaned-smoke-run-after-agent-kill` (15 min),
`tauri-dev-watcher-restart` (20 min), `publisher-fetch-refused` (the CPR logo, cleared by the operator's crop ruling).
`incidents.silent` 0.

### 4. Reworks were process, not design

4 reworks: a purchase join that put two single items on one line (E2.5, caught by the desktop sweep), a
verification re-run after the operator killed a task by accident (E3.9), a wired-integration token audit tripped by
an HTML `placeholder=` attribute (E3.4), and a RED smoke run aborted because the live vite server served the
card's edits (E3.10).

## What worked

- **Phase 1 beside a live bundle.** E1.x ran root-crate and tool side only while SD-40 held the desktop; the
  partition check (G-3) read 0 over all 34 E1 commits, and R25 started Phase 2 on SD-40's merged base.
- **Merge checks looked at the screens.** E3.MC found the Market description box off screen while its smoke row
  was green (the harness reads clipboard text); E4.MC found the gear screen and the print disagreeing on a stack's
  weight. Both fixed RED-first in the merge check.
- **Settled fields, not tags in live code.** Containers, ammunition and eqmod prices were settled by the converter
  into `_settled/equipment.json` (`decisions.md §31`); no live code reads a tag.
- **Discovered cards were finished, not flagged.** 16 of 16 closed.

## What did not work

- **Section 2.** The dispatch log was empty again.
- **Section 3.** The workflow script had no path for a card discovered after its epic was dispatched.
- **The G-8 scan itself was wrong once.** Its status `awk` split on the `\|` inside E3.12's title; fixed in the first
  E5.2 cycle (`gsub` first), and the receipt-row greps found three receipts (E2.9, E3.9, E4.G) without the R15 or
  "Looked at" row, back-filled by E5.2's second cycle with the check each states.

## Changes for SD-42

1. Make R21 a mechanism in the dispatch script: run each card's acceptance command on the base, write the output
   to the dispatch log, and refuse to dispatch on an exit 0 with the expected output. (Carried from SD-40, unbuilt.)
2. The dispatch script re-reads `kanban.md` after every merge check and dispatches any `waiting` card whose
   dependencies are `complete` before moving to the next epic, so a discovered card cannot fall off the end.
3. Ship the two filter-count checkers (`artifacts/epic_1/E1.MC/acceptance_filter_counts.py`,
   `artifacts/epic_2/E2.MC/desktop_filter_counts.py`) as a `scripts/` tool and run it at every card, not only at
   merge checks.
4. The receipt template's rows (closure check, Looked at) are checked by the dispatch script when a card returns,
   not first at closure.

## Not covered by this retrospective

Event counts are events someone chose to log, not a count of defects. The 90-of-103, 39 and 12 figures depend on the
predicates stated above and on how each logger named its subject. This document is written before E5.3–E5.4 run;
release notes, truth-up, graphify and the PR are not described. The gear screen was signed without the operator
seeing it; the release build is the first operator look at it.
