---
canonical: true
owner: sd42-e5.2
purpose: SD-42 (Campaign Codex: PCGen import accuracy and the campaign folder) retrospective, grounded in the retro log and the package's own receipts, written at E5.2.
date: 2026-11-20
bundle: SD-42
branch: tranche-22
---

# SD-42 retrospective

SD-42 made the PCGen `.pcg` import faithful: one stack per place a line is held, the file's locations and nested
containers, custom names and properties on the copy, unmappable items imported as Create items, everything
unmapped listed in Notes, Starfinder files imported as Starfinder characters, and whole-character parity against
the pinned PCGen engine for 7 sample files plus the operator's own file. It also turned a campaign into a plain
folder (the Drive API path removed, migration with a backup, Party Resources, Adventure Log, Maps with pins, a
Wiki, a `Characters/` backup with a master device) with a CodeMirror editor, and fixed issue #409 (the Linux
build's glibc floor, 2.35). Every number below carries the command that produced it. Numbers from
`retro.py summary` come from

```
python3 scripts/retro.py summary --since 2026-11-16 --json
```

whose output at writing is kept in
`docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/epic_5/E5.2/retro-summary.json` (`generated_at`
`2026-11-20T22:30:18Z`; the log keeps growing, so a later re-run reads higher). The window also holds SD-41's
closure tail (`sd41-*` actors); the SD-42 figures below are filtered by actor prefix `sd42-`, case-insensitive
(some cards logged as `sd42-E4.14`), and exclude this card's own events (Python over `docs/retro/events/*.jsonl`,
996 shards, 0 invalid lines).

**State at writing.** 58 of 61 `kanban.md` rows `complete`; the three that are not are E5.2 (this card) to E5.4,
the closure chain (`cd docs/release/SD-42-pcgen-import-and-campaign-folder && awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 !~ /^ complete *$/ { print $2 }' kanban.md`
-> ` E5.2 `, ` E5.3 `, ` E5.4 ` before this card's row moves; total `awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'` -> 61).
The bundle ran 2026-11-16 to 2026-11-20 on `tranche-22`: 131 non-merge commits whose subject names `sd42`
(`git log --no-merges --format=%s 1046daefc0..origin/tranche-22 | awk '/\(sd42/' | awk 'END{print NR}'` -> 131
before this card's commit; `1046daefc0` is the cut point on `tranche-21`). The package launched with 37 cards
(`git show 54535f54f9:docs/release/SD-42-pcgen-import-and-campaign-folder/kanban.md | awk -F'|' '$2 ~ /^ (C|E)[0-9]/' | awk 'END{print NR}'` -> 37)
and closed with 61: 15 discovered inside the run (E1.8, E1.9, E1.10, E2.14, E2.15, E4.13–E4.21, E5.5) and 9 the
operator added mid-run (E2.12, E2.13, E4.6, E4.7–E4.11, E4.12; `decisions.md §26–§28`), as the count line in
`kanban.md` names each.

## What shipped

Phase 1 ran three lanes in parallel beside SD-41 (`import` E1, `campaign` E2, `editor` E3), Phase 2 serial in
`sd42-main` after SD-41 merged (`progress.md` Summary and its four epic reports). The import's parity, per sample,
is in E4.13/E4.17's receipts: desktop parity 6 of 6 builds equal-or-explained, the E4.13 and E4.17 ledger rows
9 -> 0 and 49 -> 0. The operator's file was proved locally only (counts in `artifacts/epic_4/E4.5_cycle_receipt.md`;
the file never entered the repo: G-8's two `-S` greps over `589815b65f..HEAD` -> 0 and 0 at this card). SF1e and
CPR goldens stayed byte-identical at E4.MC (20 of 20 PDFs). E5.1's widest verify passed every stage at
`cfa9776b66` (root-full 6909 passed across 300 suites).

**The operator gate was waived, not signed.** E5.G's three `usable: yes` lines exist
(`grep -c '^- \(Import ([^)]*)\|Campaign Manager ([^)]*)\|Editor\) — usable: yes' docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/epic_5/E5.G_cycle_receipt.md`
-> 3), each marked "WAIVED by the operator, 2026-11-20 … no hands-on check was done", written by the lead. E5.1's
fresh-install check moved, by operator ruling, to the post-merge alpha release. The first human look at the
import, the Campaign Manager and the editor is that alpha.

## The log

SD-42's own events before this card: 141 (Python, actor prefix `sd42-`, case-insensitive, `ts >= 2026-11-16`,
actor `sd42-e5.2` excluded): 66 corrections, 58 verification runs, 14 incidents, 2 deferrals (both the planner's,
both FSR candidates), 1 note, 0 reworks (`awk '/"actor": ?"[sS][dD]42-/ && /"type": ?"correction"/' docs/retro/events/*.jsonl | awk 'END{print NR}'`
-> 66 before this card, agreeing). Verification: 2 of 58 runs failed a stage (`pcgen-residue-gate` once, the lead's
run that surfaced E4.19; `root-full` once, E5.1 r4, which became E5.5); both fixed by a discovered card.

### 1. The package was the most-corrected artifact, for the fifth bundle running

Of the 66 corrections, 57 name our own package, rows, acceptance commands, receipts or the lead as the wrong party
(predicate: `subject` matches
`\.md|package|dispatch|receipt|row|epic-breakdown|decisions|inventory|kanban|workflow|acceptance|FILES YOU OWN|G-\d|NR-\d|lead|brief`,
case-insensitive, SD-41's predicate unchanged). Repeat subjects: `SD-42 epic-breakdown E1.2 row` 3,
`epic-breakdown.md E4.5 row` 2, `SD-42 epic-breakdown.md E1.3 row` 2. Two shapes, as in SD-41:

- **Acceptance commands that were wrong before they drifted:** 16 corrections whose subject, claim or actual mention
  `passed|filter|R21|acceptance|matches 0`. E4.MC's report lists the cards whose RED/acceptance command was
  wrong as written (E4.3, E4.6, E4.9 ×2, E4.12, E4.19 — `cargo test` takes one positional filter).
- **FILES YOU OWN not closed:** 11 corrections naming FILES YOU OWN, a fence, a producer or a second call site.
  E2.MC counted 5 under one key and named the control owed (the lead's R22 grep over component and command names).

### 2. R21 and the dispatch log: a rule with no evidence surface, third bundle running

`progress.md`'s "Lead's dispatch log" holds its heading and template line only, as in SD-40 and SD-41. Every E4
and E5 dispatch said "the lead has not run your RED command"; each card ran RED itself and found the row's command
or premise wrong. SD-41's change 1 (R21 as a mechanism in the script) was not built. The workflow script itself is
not in the repo: `workflow-instruction.md §2.1` says the prefix is written to `artifacts/cycle_0/sd42-workflow.js`,
and R26 is checked by a grep of that file, but `git ls-files docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/cycle_0`
lists only C0's and C1's receipts and the operator brief.

### 3. Discovered cards no longer fell off the end

SD-41 lost E3.12 because the script fixed an epic's cards at dispatch time. In SD-42, 15 of 15 discovered cards were
dispatched and closed, including two found by the last cards of their epics (E4.21 by E4.MC, E5.5 by E5.1), so
SD-41's change 2 held in practice.

### 4. Incidents: one key got a control, one got a warning again

`incidents.by_recurrence_key` (SD-42 rows): `diagnosis-test-rewrites-frozen-artifacts` 4 (E1.4, E1.5, E1.6, E1.7:
the `pcg_import_diagnosis` test rewrote E1.1's committed evidence) — its control is E1.9 (the test writes under
`CARGO_TARGET_TMPDIR`), re-checked at E1.MC (2 runs, 0 files dirtied). `pgrep-wait-loop-matches-itself` 2 (E1.MC,
E4.5; ~10 and ~40 minutes lost), plus `pkill-matches-itself` 1 (E4.4): the memory note names the fix, the prefix does
not, so this is a warning that fired three times. The others fired once each: a 34 G `target/` inside a worktree
when a fresh shell lost `CARGO_TARGET_DIR` (E1.10), a stale-copy doc commit that dropped four kanban rows from
`epic-breakdown.md` (E4.18, restored by E4.19), `pcgen-residue-gate` red unseen from E4.1 to E4.14 because no card
ran it, a foreign `docs/.obsidian/` in a lane tree, a rebase conflict in a fence-exit file, a GTK dialog dropping
typed keys, an npm failure whose name was cut off by `| tail`.

## What worked

- **Diagnose before you fix (R28).** E1.1 proved or refuted H1–H7 with file:line before E1.2–E1.7 were dispatched;
  the corrections it caused re-scoped rows by `§20` before code, not after.
- **Parity ledgers as the closure instrument.** Every PCGen difference is a ledger row with a card; E4.13, E4.14 and
  E4.17 took their own ledger rows to 0 (9, 12 and 49 rows; E4.17's 49 = 43 fixed + 6 re-classified with evidence, per their receipts).
- **Phase 1 beside a live bundle, again.** Three lanes, one main lane after SD-41 merged; G-3's partition held with one named fence exit (E2.10's `character_hub.rs` test lines, after SD-41 merged; E2.MC report).
- **Merge checks looked and fixed.** E4.MC found the alias weapons' missing Strength on damage (118 records) from
  its own screenshots and fixed it RED-first; E3.MC found a stored-XSS path in the Reading view.

## What did not work

- **Section 2.** R21 remained a sentence; the dispatch log stayed empty; the script is not committed.
- **The closure scan was wrong once.** G-9's `SKIPPED` grep counted E4.5's quoted absent-path proofs (2) as a skipped
  local run; the receipt also holds the present-file counts. Corrected by this card (`decisions.md §20`, 2026-11-20),
  and two receipts (E5.1, E5.G) without the R15 row were back-filled with the check each states — the receipt-row
  check SD-41 asked to move to card return (its change 4) still runs first at closure.
- **The gate checked nothing by eye.** The waiver is the operator's call; it means the closure evidence for usability
  is screenshots taken by agents (E2.MC 19, E4.MC 22) and the alpha.

## Changes for the next bundle

1. Commit the workflow script under the package's `artifacts/cycle_0/` at launch and have it write each card's R21
   RED output into `progress.md`'s dispatch log before dispatch; refuse to dispatch on exit 0. (Carried from SD-40
   and SD-41, unbuilt.)
2. Put the pgrep/pkill rule in the dispatch prefix as a command, not a memory note: wait with
   `pgrep -x cargo` (process name) and never `pgrep -f`/`pkill -f` on a pattern the shell's own command line holds.
3. Run the G-9 receipt-row greps (`closure check`, `Looked at`) on each card's receipt when it returns.
4. Any card that changes a desktop `#[cfg(test)]` module runs `verify.sh --only pcgen-residue-gate` (the gate was red
   unseen from E4.1 until E4.14).

## Not covered by this retrospective

Event counts are events someone chose to log, not a count of defects. The 57-of-66, 16 and 11 figures depend on the
predicates stated above and on how each logger named its subject. This document is written before E5.3–E5.4 run;
release notes, truth-up, graphify and the PR are not described. No human has used the shipped import, campaign
folder or editor yet; the alpha release is the first look.
