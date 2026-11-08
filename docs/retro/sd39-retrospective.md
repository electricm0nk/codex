---
canonical: true
owner: sd39-e9.6
purpose: SD-39 (Campaign Codex: UI and print corrections) retrospective, grounded in the retro log and the package's own receipts, written at E9.6.
date: 2026-11-07
bundle: SD-39
branch: tranche-19
---

# SD-39 retrospective

SD-39 corrected twelve operator-dictated UI and print defects and added the pieces they needed: feat guards,
a one-screen sheet layout, a Market, body slots with worn versus carried, a level ledger, flagged overrides, a
`.pcg` importer, and real PDF output through a bundled Typst. Every number below carries the command that
produced it. Numbers from `retro.py summary` come from

```
python3 scripts/retro.py summary --since 2026-11-01 --json
```

whose output at writing is kept in
`docs/release/SD-39-ui-print-corrections/artifacts/epic_9/E9.6_logs/retro-summary.json` (generated
`2026-11-07T18:19:02Z`; the log keeps growing, so a later re-run reads higher).

**State at writing.** 60 of 66 `kanban.md` rows `complete`; the six that are not are E9.6 (this card) to E9.11,
the closure chain
(`cd docs/release/SD-39-ui-print-corrections && awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' kanban.md | awk 'END{print NR}'`
-> 60, total rows `... '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'` -> 66). The bundle ran
2026-11-01 to 2026-11-07 on `tranche-19`: 136 commits past the cut from develop
(`git log --oneline $(git merge-base origin/develop origin/tranche-19)..origin/tranche-19 | awk 'END{print NR}'`
-> 136; the summary's `git_join.commits` reads 137 because it counts since the date, not the merge base).

## What shipped

Epics C, E1-E9.5 (see `progress.md` Summary and its eight epic reports): guards (E1), the one-screen layout
(E2), the Market (E3), body slots and worn items (E4), the level ledger (E5), overrides (E6), `.pcg` import
(E7), print (E8) and closure through the final-acceptance scan (E9.1-E9.5). Nine cards were discovered inside the
run and finished, not flagged (`grep -c 'discovered by' kanban.md` -> 9). Widest verify at E9.3:
`scripts/verify.sh --full --show-actuals -j 8` 53 PASS, 0 FAIL. Fresh-install acceptance (E9.2) ran the clean-checkout
0.19.0 `.deb` and AppImage on empty profiles; Update > Check showed `check ok`, which is the automatic-upgrade
path the operator named as the thing to test. The operator signed three printed sheets usable on 2026-11-07
(E9.4); design and density of the sheets go to SD-40.

## The log

`events.total` 206 (`jq .events.total retro-summary.json`): 128 corrections, 12 incidents, 10 reworks,
0 deferrals, 55 verification runs, 1 note, 0 invalid lines, 57 shards. Verification fail rate 0.0182 (1 of 55
runs), the one failure at stage `tauri-resources-tracked` (`verification.by_failing_stage`). 151 events are
agent-origin and 55 derived (`events.by_origin`). The window also holds events from peer sessions outside this
bundle (`fix-perform-install` 1; `sd39-main` 17 is the lead lane), so the totals are slightly wider than the
cards alone.

### 1. The package was the most-corrected artifact, again

Of 128 corrections, 104 name our own planning documents, briefs, dispatch prompts or receipts as the wrong
party (predicate: a `by_subject` key matching
`\.md|package|dispatch|receipt|row|technical-design|epic-breakdown|decisions|inventory|kanban|workflow`,
case-insensitive; Python `re` and `jq test()` over `retro-summary.json` both -> 104). This is an upper-leaning
predicate (a key like "E3.3 list_held_equipment" is not counted, "E4.5 row, Ring of Protection" is), so read it
as "about four in five", not an exact class size. The shapes recur from SD-38: `epic-breakdown.md` FILES YOU OWN
rows missing a second call site (E2.1, E4.3, E5.1, E5.2, E7.4), acceptance commands that could not fail (E2.1's
`grep` matched a label; E8.MC's NR-2 `grep -n` on a directory printed 0), and counts copied rather than
re-derived (card-count constants, "53 PASS", the SLOTS census). The package's first defence was the lead's
pre-launch re-read: `decisions.md §25` carried nine corrections before dispatch
(`progress.md` Cycle 0), and `sd39-lead` still logged 4 more (`corrections.by_corrector`). Pre-launch reading
catches the loud ones; the quiet ones (a command that cannot fail) still surfaced one card at a time.

### 2. Catching the claim before it shipped

`near_misses.total` is 0, but `corrections.caught_before` records 6 caught at implementation, 1 at merge and 1
at release (`corrections.caught_before`). Merge-check cards earned their Opus tier: each epic report has a
"Refuted" line, and every one is a real defect fixed in the merge check's own commit: E3.MC found Export then
Import dropped every paid `+N` and a `codex-ingest` test red since E3.4 in a crate no card had run; E4.MC found a
worn deflection bonus left CMD alone and a clone lost `custom.json`; E5.MC found a clone lost `hit_points.json`;
E6.MC found the Market's `Longsword (Base)` naming meant the weapon flag refused one weapon and a worn Market
weapon had no attack row; E8.MC found the basic PF sheet omitted 28/25/29 of 35 skill totals (E8.6-E8.8 followed).
The pattern: a card proved its own criterion on its own fixtures, and the next-level check that read the whole
population (5,880 Market rows, 137 classes x 4 levels, 9,040 corpus records) found the shape the fixture never
held. This is AGENTS.md rule 7 working as designed, not a rule being broken.

### 3. Recurrence: one key fired twice, none more

`incidents.recurring` lists `ui-smoke-marker-beats-reload` (2); the other 10 incident keys fired once
(`incidents.by_recurrence_key`). No key fired more than twice, so R-W requires no new non-zero-exit check, and
none was built. `incidents.time_lost_minutes` is 3, but only incidents that recorded it count and
`incidents.silent` is 1; the largest real loss, about 50 minutes rebuilding a rebase after E3.4 and E3.5 both
appended `spec.json` rows, is in the incident text and not in that figure. That is the closest thing to a missing
mechanism: two lanes appending the same JSON array end. It fired once.

SD-38's carried-forward mechanism, `sf-seed-lines-out-dir-missing`, did not fire in SD-39:
`awk '/"actor": "sd39-/ && /"recurrence_key": "sf-seed-lines-out-dir-missing"/' docs/retro/events/*.jsonl | awk 'END{print NR}'`
-> 0 (the same awk with `sd38-` -> 3). E8.3 logged a correction that the package's premise was already false:
both writers `create_dir_all` their out dir (`sf_sheet_print.rs:439`, `cpr_sheet_print.rs:356`).

### 4. Reworks were small and specific

10 reworks (`rework.total`), none repeating a cause (`rework.by_cause`: 10 keys, count 1 each). The expensive
ones were environmental, not logical: counting converted files on disk panicked the app at launch and three
runs read as a harness stall (E3.4); a GTK save dialog doubled `.pdf.pdf` (E8.x); editing a test file while a
background verify built the same tree (stopped in time).

## What worked

- **Merge checks on Opus that read the whole population.** Section 2.
- **Discovered cards instead of carve-outs.** Nine rows added and closed (E3.6, E4.5, E4.6, E4.7, E5.4, E7.7, E8.6,
  E8.7, E8.8: `awk -F'|' '/discovered by/{print $2}' kanban.md`); the final scan found 66 rows, 59 complete plus 7 exempt, with both `awk`s printing nothing.
- **Seeds stayed honest.** Every epic report states PF/SF/CPR deltas by diff against a base built in its own
  tree: PF additive rows only, SF x4 and CPR x5 empty diffs (E9.3: PF 46 additive rows each).
- **The operator wait was one planned stop.** E9.4 returned `partial` until the sign-off line existed, as
  designed (`workflow-instruction.md §2.4` item 11).
- **Clippy before the first commit** (SD-38 change 3) held: the one failed verify run was a resource-tracking
  stage, not clippy (`verification.by_failing_stage`).

## What did not work

- **Full `npm run ui-smoke` is still not a gate.** R17 made this explicit; E2.4 ran it once and recorded
  17 red of 133 in two sessions, 14 stale rows at E9.5. FSR-C2's condition is met and the rewrite is the
  operator's call; it is Candidate work for SD-40.
- **Acceptance commands that cannot fail.** Section 1. Nothing in the chassis runs a candidate acceptance
  command against a known-bad tree before dispatch. E9.6's own command (`grep -c sd39-retrospective
  references/README.md`) already returned 1 before this card wrote anything, because the README line was
  authored in advance; it is strengthened in this commit to also require the file.
- **Receipts claim what they did not run.** E3.4's receipt said a crate was not re-run, and it was the red one.
  Several corrections are of this shape: the claim in the receipt, not the code.

## Changes for SD-40

1. Before a card is dispatched, run its acceptance command against the pre-change tree and require it to
   fail (or exit non-zero); a command that passes on the base is a defect in the brief, caught at authoring.
2. FILES YOU OWN: derive the second-call-site list with `git grep` over the producer, as R15 already says;
   add the check to the lead's pre-dispatch re-read, since the card cannot be the only reader.
3. Two lanes appending to `spec.json` conflict; give side lanes disjoint insertion points or have the
   merge-check lane own the append (one incident, so a note here, not a control).
4. A merge-check card's whole-population probe is the best defence this bundle found; make "name the
   population and its denominator" a required line in every non-MC card's does-not-cover row.
5. Give the stale ui-smoke rows (FSR-C2) an owner in SD-40's scope instead of re-listing them.

## Not covered by this retrospective

The summary reads events logged under `--since 2026-11-01`; the SD-38 tail and the SD-39 authoring session are
partly outside or inside the window depending on timestamps. Event counts are events someone chose to log, not a
count of defects: `incidents.silent` is 1 and no instrument measures defects nobody logged. The 104-of-128
figure depends on the subject-key predicate stated above and on how each logger chose to name its subject. This
document is written before E9.7-E9.11 run; their outcomes (sweep, release notes, truth-up, graphify, PR) are not
described. macOS and Windows installers were not built or run here (E9.1, E9.2). The sheets' design and density
were not judged beyond the operator's three `usable: yes`.
