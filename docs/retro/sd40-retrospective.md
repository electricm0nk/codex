---
canonical: true
owner: sd40-e5.3
purpose: SD-40 (Campaign Codex: Pathfinder sheet design) retrospective, grounded in the retro log and the package's own receipts, written at E5.3.
date: 2026-11-14
bundle: SD-40
branch: tranche-20
---

# SD-40 retrospective

SD-40 redesigned the Pathfinder 1e printed sheet on a three-layer model (data blocks, one Typst look per block,
layouts as lists on a page grid), against PCGen's standard sheet as a picture only, with SF1e and CPR output
held byte-identical. Every number below carries the command that produced it. Numbers from `retro.py summary`
come from

```
python3 scripts/retro.py summary --since 2026-11-07 --json
```

whose output at writing is kept in
`docs/release/SD-40-pf-sheet-design/artifacts/epic_5/E5.3/retro-summary.json` (generated
`2026-11-15T00:39:20Z`; the log keeps growing, so a later re-run reads higher).

**State at writing.** 58 of 61 `kanban.md` rows `complete`; the three that are not are E5.3 (this card) to E5.5,
the closure chain (`cd docs/release/SD-40-pf-sheet-design && awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'`
-> 58 before this card's row moves; total `awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'` -> 61).
The bundle ran 2026-11-07 to 2026-11-14 on `tranche-20`: 167 commits past the cut at `d4489b3557`
(`git log --oneline d4489b3557..origin/tranche-20 | awk 'END{print NR}'` -> 167, 163 without merges; the
summary's `git_join.commits` reads 174 because it counts since the date across every branch the log saw).
The package launched with 23 cards (`git show <first kanban commit>:…/kanban.md | awk -F'|' '$2 ~ /^ (C|E)[0-9]/' | awk 'END{print NR}'` -> 23)
and closed with 61.

## What shipped

Epics C, E0–E5.2 (see `progress.md` Summary and its six epic reports): the PCGen reference render and the 14
stale ui-smoke rows repaired (E0), the page grid and block looks (E1), page 1 through three operator rounds
(E2, E2.G), sections 2–7 plus the Biography and Notes tabs (E3), sections as lists with an eight-checkbox print
dialog replacing presets (E4), and the widest verify, clean `.deb`/AppImage build and the final operator gate
(E5.1, E5.2). The operator signed page 1 twice (`grep -c '^- Aldric .* — usable: yes' artifacts/epic_2/E2.G_cycle_receipt.md`
-> 2) and the full set five times (`grep -c 'usable: yes' artifacts/epic_5/E5.2_cycle_receipt.md` -> 5).
Six cards were discovered inside the run and finished, not flagged
(`awk -F'|' '/discovered by/ && $2 ~ /^ (C|E)[0-9]/ {print $2}' kanban.md` -> E2.15, E2.28, E2.31, E2.32, E3.8, E3.9).
SF1e ×8 and CPR ×8 golden PDFs stayed byte-identical at every merge check (E3.MC, E4.MC epic reports: 16 of 16).

## The log

`events.total` 146 in the window (`jq .events.total retro-summary.json`); 132 of them are SD-40's own
(`jq '.events.by_actor | to_entries | map(select(.key|startswith("sd40-"))) | map(.value) | add'` -> 132;
the rest are the SD-39 closure tail, `sd39-*` 14). SD-40's own: 60 corrections, 51 verification runs,
12 reworks, 6 notes, 3 incidents, 0 deferrals (Python over `docs/retro/events/*.jsonl`, actor prefix `sd40-`,
`ts >= 2026-11-07`; the correction count re-derived with
`awk '/"actor": ?"sd40-/ && /"type": ?"correction"/' docs/retro/events/*.jsonl | awk 'END{print NR}'` -> 60).
Verification fail rate 0.0 (0 of 51, `verification.failed_runs`). 0 invalid lines over 63 shards.

### 1. The package was the most-corrected artifact, for the third bundle running

Of SD-40's 60 corrections, 57 name our own package, rows, acceptance commands or receipts as the wrong party
(predicate: `subject` matches
`\.md|package|dispatch|receipt|row|epic-breakdown|decisions|inventory|kanban|workflow|acceptance|FILES YOU OWN|G-\d|NR-\d`,
case-insensitive, Python over the raw events; the three that do not are tests: E2.3's page-one test, E2.4's
overflow test, E2.30's own AppMenu test). Two shapes dominate:

- **FILES YOU OWN not closed:** 21 corrections whose subject names a FILES YOU OWN row (Python over the raw
  SD-40 events -> 21; `jq` over `corrections.by_subject` for the whole window -> 21; a whole-line `awk` match
  reads 27 because it also hits the word in `claimed`/`actual`, a wider predicate). E2.29's row alone was
  corrected three times (`corrections.repeat_subjects`).
- **Acceptance commands that read the wrong thing:** 18 whose subject names an acceptance, grep or check
  (E2.G's sign-off count matched its own instructions, `decisions.md §24.5`; E3.6's `aria-label=` grep could not
  match a prop-forwarded label; E4.2's removal grep counted lines kept by design; NR-2's Typst grep needed
  widening twice).

SD-39's changes 1 and 2 (R21 RED-before-dispatch, R22 second call sites by `git grep`) were written into the
package but their enforcement surface, `progress.md`'s "Lead's dispatch log", is empty at closure. The rules did
not run as a pre-dispatch step; cards caught the defects themselves, one at a time, which is where every one of
these 39 corrections was logged.

### 2. Merge checks again found what cards could not

`corrections.caught_before` records 4 caught at implementation. Each merge check's "Refuted" line is a real
defect fixed in its own commit: E3.MC's 65-file `.pcg` population probe found 20 lost words on page 1 that every
card's fixtures passed, and that E2.4's and E2.MC3's skills check had never read a words row; E4.MC found page 1's
Campaign cell blank on every print without Bio and ran 858 renders (39 characters × 2 papers × 11 section sets,
Typst warnings 0) to state page counts per section set. Same pattern as SD-39: a card proves its fixture; the
whole-population probe finds the shape the fixture never held.

### 3. Recurrence: one key fired twice, none more

`incidents.recurring` lists `fence-exit-print-file` (2: E3.6 needed `block_vocabulary.json`, E4.1 needed
`frame.typ` and `blocks/core.rs`, both outside FILES YOU OWN). It is the same root as section 1's FILES YOU OWN
corrections, seen from the other side: the row was not closed, so the card hit a fence. The third SD-40 incident,
`ui-smoke-update-tab-stall` (E2.30: the webview stops answering after the Settings Update tab, red on the base
too), fired once. `incidents.silent` 0, `time_lost_minutes` 0 (only incidents that record it count).
SD-39's recurring key `ui-smoke-marker-beats-reload` did not fire in SD-40
(`awk '/"actor": ?"sd40-/ && /ui-smoke-marker-beats-reload/' docs/retro/events/*.jsonl | awk 'END{print NR}'` -> 0).

### 4. Reworks were layout physics

12 reworks (`rework.total`), none repeating a cause (`rework.by_cause`: 12 keys, 1 each). Six are page
geometry (a Letter column with 0.7 cm or 1.63 cm of room, the 35-line skills column, a second weapon pushing
Speed off Letter page 1, grid cells that cannot share a row left of an earlier cell, breakable headings orphaned
at a page foot); two are probes (E4.MC's 858 renders at ~15 s each single-process, re-run as 8 shards; E3.MC's
presence check misreading page-2 continuations); the other four are a compile error, a test pin, a shared render
helper and one fix drafted before its RED test ran (reverted, RED run, re-applied). The split is this author's
reading of the twelve `by_cause` strings, not a computed predicate.

## What worked

- **Looking at the output.** R7 (render, rasterize, read the PNGs, compare to the PCGen picture) is in every look
  card's receipt (`grep -L 'Looked at' artifacts/epic_[1-4]/*_cycle_receipt.md` minus the named engine cards prints
  nothing). E4.MC found the Campaign defect by eye.
- **Operator gates as planned stops, defects as cards.** E2.G took three rounds; rounds 1 and 2 became replans
  (`decisions.md §23`, `§24`) and cards (23 -> 61), not arguments.
- **SF1e and CPR untouched.** 16 of 16 golden PDFs byte-identical at each merge check.
- **R24 (one writer per `spec.json`).** The serial edge E0.2 -> E4.2 held; no `spec.json` rebase incident
  (SD-39 had one).
- **FSR-C1 got an owner** (E0.2, SD-39 change 5); the 14 stale ui-smoke rows were repaired, not deleted.

## What did not work

- **R21/R22 were rules on paper.** Section 1. A rule whose evidence surface (the dispatch log) stays empty is
  a warning, not a control (AGENTS.md rule 8).
- **An early PR merged a third of the bundle.** At E2.G round 3 the operator asked for the PR; PR #406 merged
  `tranche-20` to develop on 2026-11-13 (`gh pr view 406 --json mergedAt` -> `2026-11-13T15:51:23Z`) with E3,
  E4 and E5 still waiting, and its release notes were still the interim body (PR #407, `fix/sd40-release-notes`,
  opened to fix the publish). The cost was a second release-notes path and a closure PR that now carries only
  the remainder.
- **Page-1 Letter geometry was discovered, not designed.** Section 4. The skills column (35 lines) and per-weapon
  growth were known from the seeds; their sum on Letter was found by stress renders mid-epic.

## Changes for SD-41

1. Make R21 a mechanism: the dispatch script runs each card's acceptance command on the base and refuses to
   dispatch a card whose command exits 0 with the expected output, writing the result to the dispatch log
   itself, so the log cannot be empty.
2. FILES YOU OWN closure as a script, not a reading: for each producer a row names, `git grep` its callers and
   the tests that pin its output (goldens, vocabulary JSON, packaged-resource counts) and fail authoring when a
   hit is outside the row. 21 corrections and both `fence-exit-print-file` incidents are this one shape.
3. When the operator asks for a PR before the bundle is done, name the open cards and ask merge-now or finish
   first (memory: early PR); if merge-now, finalize the release notes in the same commit.
4. For any fixed-size page area, state its budget (cm of room, lines) in the card from a stress render
   before the look card is dispatched.

## Not covered by this retrospective

The summary reads events logged under `--since 2026-11-07`; the window holds 14 SD-39 closure events, excluded
from the SD-40 figures above by actor prefix. Event counts are events someone chose to log, not a count of
defects. The 57-of-60, 21 and 18 figures depend on the subject predicates stated above and on how each logger
named its subject. This document is written before E5.4–E5.5 run; release notes, truth-up, graphify and the PR
are not described. The operator's E5.2 sign-off reads "I will look later": the five `usable: yes` lines are the
gate's evidence, not a printed-and-read judgment of every page.
