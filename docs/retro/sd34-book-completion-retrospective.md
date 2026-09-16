---
canonical: true
owner: AT-35-E1-006 (SD-35 Epic 1, written on SD-34's behalf — SD-34 merged without running its epilogue)
purpose: SD-34 retrospective, grounded in the 387-event retro window 2026-08-27 → 2026-09-08 rather than recollection.
date: 2026-09-08
board: DONE 12,265 of 49,438 at open (24.8%) -> 26,123 of 49,438 at the tranche/15 cut (52.8%)
window_command: python3 scripts/retro.py summary --since 2026-08-27 --json
---

# SD-34 retrospective

Fifty-one waves, 2026-08-26 → 2026-09-07. The board moved **DONE 12,265 of 49,438 → 26,123 of
49,438** (24.8% → 52.8% of the same 49,438-unit corpus). SD-34 was merged to `develop` by operator
ruling on 2026-09-07 (PR #383, `fe5ae6cd4a`) **without running its own closure epilogue** — no
final-acceptance scan, no retrospective, 17 of its 37 `kanban.md` rows not `complete`. This
document is that epilogue, written by SD-35's AT-35-E1-006 under `SD-35 decisions.md §12`.

Every number below comes from `docs/retro/events/` via
`python3 scripts/retro.py summary --since 2026-08-27 --json` (the window that starts at SD-34's
first dispatched cycle), or from the command written next to it. Nothing is from memory.

```
EVENTS  387   (window 2026-08-27 → 2026-09-08T01:33Z, 50 shards)
   173  verification         52  incident
    93  correction           34  deferral
    20  note                  9  resolution
     3  near_miss             3  rework

incidents with a recurrence key        52 of 52 (14 distinct keys)
SILENT failures (plausible wrong output)    4
recorded time lost                     45 minutes
corrections that had already propagated when caught   39 of 93
verification runs with a failing stage                 31 of 173 (17.9%)
git in the same window                                522 commits, 1 author, 0.74 events/commit
deferrals                                              34 recorded, 5 resolved, 29 open at the cut
```

Board figures: `python3 scripts/completion_atlas.py --check | grep '  DONE:'` at the cut
(`4c6c57eb9f`) prints `DONE: 26123`; SD-34's opening figure is the first row of
`docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-001_cycle_receipt.md`
(`DONE 12,265 of 49,438`).

---

## What the data says, before any interpretation

### 1. Our own dispatch briefs were the most frequently wrong artifact — again

93 corrections. The five repeat subjects (`corrections.repeat_subjects`) are **all prose we wrote
to ourselves**: the wave-24 dispatch brief (3), the AT-34-E3-001 dispatch brief (2), the
AT-34-E4-002 dispatch brief (2), the atlas/engine-table citation constants (2), and `?` (3 —
corrections filed without naming who was wrong). Of the 93, **39 had already propagated** into a
downstream artifact before anyone caught them; only 4 were caught at the brief stage
(`corrections.caught_before`: brief 4, implementation 13, commit 9, merge 8, closure 2,
release 1).

This is SD-31's finding one more time (`sd31-retrospective.md §1`). The three dispatch-brief
corrections that cost the most: wave 39 lane A was told the Shape 2 remainder was 54 units when
it was 154 at the real rebased HEAD — the 100-unit gap was a lane's commit chain that had never
been pushed (`2026-09-04T03:23Z`); wave 39's brief listed 9 classes / 27 units when the corpus
held 14 / 28 and silently omitted Summoner (`2026-09-04T03:36Z`); wave 37 lane B's brief
described a 634-unit population as one uniform shape when its top two classes alone were two
different unbuilt subsystems (`2026-09-03T19:25Z`).

### 2. Disk exhaustion fired 33 times and was finally made a mechanism — after the bundle

```
33x  disk-full
 6x  wrong-base-worktree
 2x  shared-target-dir
```

`disk-full` is 33 of 52 incidents. SD-31 recorded 120 of them and wrote "an unbuilt fix"
(`sd31-retrospective.md §2`); SD-34 recorded 33 more. The control — `scripts/reclaim.sh --apply`
on a timer, reclaiming stale `cargo-target` directories, verify logs, worktrees and branches — is
visible in the log only from 2026-09-06 (ten `reclaim.sh` incident rows, one reclaiming 49.2 GB).
It exists now; it did not exist for the 33 firings. `wrong-base-worktree` fired 6 more times
despite SD-31's one-line fix, because the hazard moved: worktrees now start on a **stale**
`origin/tranche/N` fetch rather than on a branch with no `docs/` tree (`kanban.md` rows 33 and
36 each record a `git reset --hard origin/tranche/14` before any work).

### 3. The wave-end gate was red 31 of 173 runs, and 24 of those were the same stage

`verification.by_failing_stage`: `site-dashboard-check` 24, `denominator-gate` 7, `root-full` 7,
`corpus-trap-audit` 6, `clippy` 5, everything else ≤ 3. The dashboard producer may only run from
the wave-end gate, never from a lane (deferral `1788290815996-sd34-at-34-e6-001-50a7ee` records
the brief's own "doubled hazard note"), so every wave that closed units arrived at its gate with a stale dashboard
and paid one extra producer run. **A gate that is red by construction on every productive wave is
a stage in the wrong place**, not a finding.

### 4. Four silent failures, all caught by re-derivation

Four incidents produced plausible wrong output. The sharpest: `citation_failures=0` hid a stale
citation whose target substring happened to still occur on a doc-comment line at the old line
number (`2026-09-03T03:20Z`) — the gate was green and the citation was wrong. Two of three
citation instruments were stale at HEAD before wave 51 for the same reason (waves 49 and 50
edited the engine source and re-derived only the atlas's pins). That is SD-35 AT-35-E1-002's
whole justification: content anchors, not line numbers, and every `--check` as a `verify.sh`
stage.

### 5. Batch size decayed to two units per build, then recovered by ruling

From `docs/release/SD-34-book-completion/progress.md` (`grep -n '^### Cycle — Wave'`): waves 42–48
closed 2, 12, 4, 32, 20, 43 and 16 units per build (wave 44's four were classifier collisions
beside a census-script fix); waves 49, 50 and 51 closed 129, 343 and 217 with one build each. The recovery was an operator ruling (`SD-35 decisions.md §2`), not a lane
noticing. The memory note `batch-big-not-small-per-dispatch` existed before wave 43 and did not
stop the decay — the same "warning is not a control" shape as disk-full.

---

## What worked

**The Completion Atlas as a fail-closed partition.** `completion_atlas.py --check` held
`population=49438 unclassified=0 overlap=0` through 51 waves and 522 commits, and its condition 6
(citation drift) fired exactly when it should — 10 stale citations at wave 32, all re-derived. Every
wave's unit count was a bucket diff against it, never a lane's own number. The three near-misses
in the window were all caught the same way: a before/after diff of `docs/work-inventory.json`
against a saved snapshot.

**Generic mechanisms over per-object lanes, once the bundle got there.** The 2026-08-30 salvage's
one corpus-wide `closure_has_real_aspect_description` widening closed 123 units (`kanban.md`
row 28); wave 50 closed 343 across buckets B/C/D/M of two books, 99 of them a cross-book bonus
from the same generic fixes; wave 51's 217 came from two generic mechanisms. Per-class lanes in
waves 42–48 produced 2–43 each. The SD-32 lesson (`generic-pass-not-per-object-lanes`) held when applied and was expensive
whenever it was not.

**Lanes refusing to fabricate.** Wave 40 lane A's Druid ~ Nature Bond alias was declined because
the only candidate id is a permanent `+0` recognition record and aliasing it "would credit a
fabricated non-zero-looking DONE"; wave 37 lane C declined a zero-bucket-movement owner-override
that would have produced a green diff and no closure. Both are recorded as deferrals with the
reasoning intact, which is what let this cycle disposition them in minutes.

**Escalation discipline held, once.** AT-34-E3-001's `## Open blockers` filing asked permission to
run more cycles; it was cleared as a sequencing decision, not a ruling, in the same session
(`progress.md` "Open blockers", `decisions.md §14`) and the nine mechanisms it named were
dispatched cheapest-first. The filing's own arithmetic ("ten mechanisms") was corrected on
re-derivation to nine summing to 1,006 of 1,006.

**Honest partials.** 29 deferrals with a `revisit` condition each. Every one names a population
and a reason; none says "later".

---

## What did not work

**The finish line was simulation-shaped.** SD-34's `grounded` bar required a magnitude *observed
reaching a consumer through a real pipeline run*. Under it, wave 51 built a whole `racial_sla`
module to compute a save DC and then refused 48 Monk unarmed-damage units because no engine
character can be size Large — a dice literal that a player would simply write down. Most of the
20 open deferrals this cycle mapped (rather than resolved) are the same shape: a computable value with no modelled consumer (Domain
Base's DC, `2026-08-27T20:49Z`), a choice the engine does not track (%CHOICE equipment
modifiers, `2026-08-28T13:13Z`), a cross-book attribution question about a dice column
(`2026-08-30T18:29Z`). The operator's 2026-09-07 ruling (`SD-35 decisions.md §1`) replaces the
bar: a final number, a dice expression, or the rule's words.

**The closure epilogue never ran.** SD-34's own `workflow-instruction.md §11` names a
final-acceptance scan, this retrospective, and a worktree sweep before the PR. The bundle was
merged by ruling on 2026-09-07 with 17 rows open and no epilogue, and the successor package's
launch-readiness audit found the gap (`SD-35 decisions.md §12`). A closure step that depends on
someone remembering to run it is the same shape as a warning in a prompt.

**One lane's work existed only as a dangling commit chain.** Wave 38 lane C's dot-segment matcher
fix (`b80ccbffa4`) was never pushed; wave 39's brief was written as if it had merged, and the
lane that received the brief found a 154-unit remainder where it had been told 54. `SD-35
workflow-instruction.md §2.5` ("commit and push before ending the turn, always") is the direct
response. A rework event records the same failure in a different lane: a ledger data file built,
`complete` written to `kanban.md`, and the file never `git add`ed.

**The inventory regenerator could not finish inside a lane's turn.** Twice (wave 40 lane A,
`blocked-escalated`; AT-34-E3-003's `2026-08-31T21:15Z` deferral) a full `v06_work_inventory`
regen was killed after ~13 minutes with the classify pass still running, on a shared
`CARGO_TARGET_DIR`. The fix was procedural (regen once per wave from the gate, guarded) and the
measurement lesson from SD-33 (`measure-before-population-run`) had to be relearned.

**Two P1 defects had no owner for the whole bundle.** The fable review's R11-01 (`character_id`
joined into a filesystem path with no validation, including `delete_character →
fs::remove_dir_all`) and R14-02 (non-atomic saved-character writes) were confirmed on 2026-08-31,
assigned in wave-28 briefs, and are unfixed at the cut (`git log --grep 'R11-01\|R14-02'` finds
only the review and its fold). Every gate lane was scoped to a `verify.sh` stage; a defect that
belongs to no stage belonged to nobody. They are now `SD-35 forward-scope-register.md` C2.5 —
tracked, still not fixed.

---

## Changes for SD-35 — and where each already landed

Each row names the mechanism, not the intention. Rows already carried by
`SD-35 decisions.md §9` are marked; the rest were added to `§9` by AT-35-E1-006 in the same cycle.

| # | Change | Where it is enforced |
|---|---|---|
| 1 | The finish line is a rendered sheet line, not an observed pipeline delta | `decisions.md §1`; carried as `§9` L1 |
| 2 | 500-unit floor per cycle, as a script with a nonzero exit | `scripts/cycle_scope_gate.py --min 500`; carried as `§9` L2 |
| 3 | Citation pins are content anchors; every `--check` is a `verify.sh` stage | AT-35-E1-002; carried as `§9` L3, L4 |
| 4 | Commit and push before the turn ends — a lane's unpushed chain is a wrong brief for the next lane | `workflow-instruction.md §2.5`; **added as `§9` L14** |
| 5 | The dashboard producer runs at the epic wrap-up, and its `--check` is the gate there — never a per-wave red stage | `decisions.md §3`, `workflow-instruction.md §10` step 0; **added as `§9` L15** |
| 6 | A predecessor's unrun closure is folded into the successor's Epic 1, not forgotten | AT-35-E1-006, `workflow-instruction.md §12` row 37; **added as `§9` L16** |
| 7 | Disk reclamation is a timer, not a per-wave chore | `scripts/reclaim.sh --apply` on the 4-hourly cron (visible in the log from 2026-09-06); per-lane `CARGO_TARGET_DIR` in `§2.1` |
| 8 | A defect that belongs to no gate stage still needs a named owner | `forward-scope-register.md` C2.5 names the eight P1s and their successor |

---

## What was left, by mechanism — the 29 open deferrals

`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-1-tax-cut/sd34-deferral-dispositions.json`
carries every id. In summary: **8 resolved** with a resolving SHA (the shape-engine-boundary
test drift, the inventory regen that would not finish, the dashboard refresh, the 40/40 gate,
the root-full recount, the atlas regeneration, the 5-unit proficiency rung, the Stunning Fist
synonym entry); **1 superseded** by a register row (the two P1s, C2.5); **20 mapped** to the
SD-35 criterion whose bucket owns their units at the cut. The 17 open kanban rows map the same way in
`sd34-open-row-map.json`: Core Rulebook's 1,529 non-DONE units and Ultimate Campaign's 61 (from
`completion_atlas.py --book core_rulebook --check` and `--book ultimate_campaign --check`, 1,590
of 1,590) land in AT-35-E3-001, E3-003, E4-001, E4-002, E5-002, E5-003 and E5-004; the two closure
rows land in AT-35-E7-001..003.

---

## The finding that reframed the package

**The harness around the number was the cost, not the number.** The fable review's TOKEN-MODEL
found the slow hours were transcription — reading tokens by eye, writing bespoke Rust per
feature, hand-writing per-feature probes — and that 28 token types cover 90% of 181,274 token
instances. SD-34 wave 51 computed a save DC correctly and then spent its remaining budget proving
the DC reached a consumer. The operator's ruling is that a character sheet is paper: print the
number, print the dice, print the words. SD-35 is built on that sentence.

**Standing rule out of that:** a Definition of Done that requires observing a value in flight
builds machinery to observe values in flight. State the DoD as what the reader sees.
