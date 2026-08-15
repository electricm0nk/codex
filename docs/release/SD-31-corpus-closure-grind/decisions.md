# SD-31 Decisions

## Decision 1 — Package created by split from SD-30; binding rules inherited verbatim (2026-08-14, operator ruling)

**Status:** New. Operator ruling, 2026-08-14, verbatim: "ok, let's split phase 3 and phase 4 into
their own SD's. SD-31 and SD-32. Take the existing SD-31 and rename it to SD-33."

**What moved here, from `SD-30-class-feature-archetype-bundle/`:** Epics 4 (per-class measurement), 5
(archetype mechanism), 6 (per-class chassis sweep), 10 (corpus-wide ingest lanes folded from SD-29),
11 (7-book onboarding), and the grind-lane half of Epic 14 (cloud fan-out protocol) — renumbered
Epics 1-6 in this package's own `epic-breakdown.md`/`kanban.md`. See `epic-breakdown.md`'s renumber
map for the exact `SD30-E# → SD31-E#` mapping. `SD-30-class-feature-archetype-bundle/decisions.md §51`
records the split from the origin side; this decision records it from here.

**What did not move:** SD-30's Epic 0 (instrument-apply), Epic 1 (identifier cleanup), Epic 2
(pre-launch), Epic 3 (PI-screening gate), Epic 7 (version numbering), Epic 8 (bundle code review),
Epic 9 (closure). SD-30 retains ownership of the PI-screening gate; this package's ingest epics
(3, 4, 5) are hard-gated on it as a cross-SD dependency, not a co-owned surface.

**Rules reproduced below, verbatim, load-bearing on every cycle in this package from here forward**
(copied rather than cited-by-reference, because a cross-SD reference that outlives the origin
package's own scope narrowing is a maintenance hazard this program has already been burned by once —
see `SD-30-.../decisions.md §50`'s own reasoning for absorbing SD-32's rules the same way):

### (a) The anti-gaming rule

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §50(a)`
(originally SD-32 `decisions.md` Decision 1):

> **THE ONE RULE THAT OVERRIDES EVERYTHING ELSE: YOU MAY NOT MOVE A NUMBER BY LOWERING THE BAR.**
>
> The operator's directive is "improve our numbers, assuming the measuring systems are accurate." That
> second clause is a constraint, not a licence: the instruments are to be trusted and EXTENDED, never
> tuned to flatter the result. Every one of the following is forbidden, and doing any of them makes
> this work worse than not doing it:
>
> - Reclassifying a unit into an easier wiring_class so it clears a lower bar.
> - Loosening, skipping, #[ignore]-ing or special-casing a check so more units pass.
> - Marking a unit done on evidence weaker than its class actually requires.
> - Counting 'held' as done. SD-29 decisions.md §46.4 deliberately does NOT count it, and the
>   doneness_meaning text says so explicitly: "As done as the current instruments can prove, and
>   deliberately not counted as done."
> - Widening a bucket definition, or editing doneness_meaning, to make a bucket look better.
> - Ingesting fixture data, or hand-authoring rules data, to satisfy a check.
>
> This program has spent three days learning that a green instrument over an empty screen is worse
> than a red one. A number that moved because the bar moved is a lie told to the operator in the one
> artifact they use to judge progress. If a unit cannot legitimately reach its bar, LEAVE IT and say
> why. Reporting "fewer moved than hoped, honestly" is a success. If you ever find yourself editing a
> threshold, a classifier, or a definition to make a count rise, STOP and report it instead.

### (b) The table-sheet doneness doctrine

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §49` (operator
ruling, verbatim):

> I would rather bring SD-32 into the SD-30 scope. I think the gist of what I was
> saying with "done rung for static and derived" was basically that some things do
> not require computation. If a fireball is 1d6 per spellcaster level - you don't
> need to compute 6d6 for a 6th level caster - you need to display that the
> fireball spell is 6d6 because the character in question is 6th level. That's
> just printed in the character sheet. The actual rolling of 6d6 happens on a
> table, with dice, and the additions are added by the player's brain. Our goal
> here is to print a character sheet that the user can use at the table - we are
> not making a video game. So in many cases we just need to expose the end rule -
> once we can do that it's done. If a spell says 1d6 per character level, you just
> need to be able to determine the character level and say the true value when the
> character sheet is created.

**Operative consequence for this package:** SD-31's Epic 1/2/3 (`class_feature`) acceptance is governed
by this bar directly — a feature is accepted once its rule is resolved to a true, character-specific
value and displayed, not once its mechanism is internally simulated. A resolved value must still be
**true**: `static` values pass byte-equality against the corpus literal, `derived` values pass
evaluator-vs-fixture verification, and the value must be **displayed** — reach/on-screen verification
still applies. This does not relax the anti-gaming rule at (a).

### (c) The PI-gate hard-block (cross-SD dependency, not a copy of the gate itself)

This package does not own or re-implement the PI-screening provenance gate — that stays
`SD-30-class-feature-archetype-bundle`'s Epic 3. What is binding here: **no cycle in this package's
Epic 3 (chassis sweep), Epic 4 (ingest lanes), or Epic 5 (book onboarding) may claim a book before that
book's declared-PI screen (SD30-E3-F2, the `NAMEISPI`/`DESCISPI` reader, and SD30-E3-F3, the corpus-wide
backfill sweep) shows `COMPLETE` for that book in SD-30's own `progress.md`.** This mirrors exactly how
SD-30's own Epic 6/Epic 10 were gated before the split (`SD-30-.../decisions.md §39`, `§44` lesson 4) —
the fold from SD-29 widened which kinds' ingest is subject to the gate, it never relaxed it, and moving
those epics to a sibling package does not create a bypass either.

### (d) The concurrency/cloud fan-out protocol

Reproduced from `SD-30-class-feature-archetype-bundle/decisions.md §47`: hardware baseline (8 cores /
45GB RAM / 968GB disk at 19% used, captured 2026-08-14), concurrency cap of three concurrent
build-capable agents (re-derived empirically per the existing budget discipline), and the cloud
fan-out rules — every cloud agent works its own branch, never two writers on one branch; the local
orchestrator owns all merges to `tranche/10`, verified by content not commit count; DoD-8 on-screen
verification and dashboard-producer work stay local. Governs this package's Epic 6.

**Authority:** operator ruling, 2026-08-14, transcribed in the dispatch brief for this split;
`SD-30-class-feature-archetype-bundle/decisions.md §43-§51` (the widened-charter, 100%-mandate, and
split decisions this package's scope descends from).

### (e) The classifier accuracy-not-movement rule (LOAD-BEARING, binds Epic 2 directly)

**Absorbed 2026-08-15 from `SD-32-engine-capability-builds/decisions.md` Decision 1(b) (`decisions.md
§2`).** It is lettered (e) here because (b) in this package's Decision 1 is already the table-sheet
doctrine; any inherited citation reading "SD-32 Decision 1(b)" resolves to this subsection.

Reproduced exactly as it stands in `SD-30-class-feature-archetype-bundle/decisions.md §50(c)`
(originally the deleted SD-32 package's Decision 3):

> **Decision.** The classifier that resolves `ambiguous` (360 units) and re-examines
> `display`+`grounded` (1,416 units) is accepted or rejected on **agreement with a hand-labelled
> sample**, and on nothing else.
>
> 1. **The gate that runs first.** A sample of at least 100 units, stratified across the five
>    wiring classes and across at least four kinds, is hand-labelled from the corpus record — the whole
>    record, not a field-filtered grep — **before** the classifier is written. The labels are
>    committed. The labeller records the token evidence for each label.
> 2. The classifier's acceptance criterion is its **agreement rate against that held-out sample**,
>    reported per class and per kind, plus its full confusion matrix. There is no target count of
>    units moved anywhere in this epic's acceptance.
> 3. **Movement is reported in both directions.** A classifier that reclassifies 180 units into
>    `computed` and 400 units out of `computed` into `static` reports both, and its net effect on
>    `done` may be **negative**. That is a **passing** outcome. A classifier that only ever moves units
>    toward the two `done`-producing cells is presumptively wrong and must be re-examined before its
>    output is accepted.
> 4. If the labelling gate's sample shows the current classifier is substantially correct and any
>    `display`+`grounded` contradiction is real but rare, the reclassification pass is **not
>    dispatched**, this epic closes at the labelling gate, and the affected units are reported as
>    "examined, correctly classified, left alone." That is `COMPLETE`.
>
> **Rationale.** This lever is ranked #2 by ceiling and #1 by gaming risk. Under (a)'s first forbidden
> item — "reclassifying a unit into an easier wiring_class so it clears a lower bar" — a classifier is
> exactly the instrument that could do that at scale while looking principled. The defence is that the
> classifier is judged against ground truth established *before* anyone knows which way it moves the
> count.

## Decision 2 — SD-32 absorbed into SD-31; epics re-sequenced so capability precedes the grind that depends on it (2026-08-15, operator ruling)

**Status:** New. Operator ruling, 2026-08-15, verbatim:

> SD-31 will be next, if there are prereqs in SD-32, then they need to be moved into SD-31. I'm
> inclined to merge SD-31 and SD-32 and have you reshuffle the epic order to ensure everything is
> sequenced correctly

**The defect this ruling fixes.** `SD-30 decisions.md §51` split SD-30 into a "grind" package (SD-31)
and a "capability builds" package (SD-32), numbered and intended to run in that order, each declaring
the other file-disjoint and concurrent. That ordering was **inverted with respect to the actual
dependency**, and both packages said so in their own text without either drawing the conclusion:

- `SD-32/README.md` "Dependency position": *"**Unblocks:** `SD-31-corpus-closure-grind`'s Epic 4-F3/F4
  (race/race_trait ingest ceiling) once Epic 1 lands; `SD-31-corpus-closure-grind`'s Epic 1-F4
  (`unknown`-bucket disposal) once Epic 2 lands."*
- `SD-32/README.md` "Exit statement": *"its contribution is specifically *unlocking* SD-31's ceiling,
  not moving `done` counts directly itself."*
- `SD-31/epic-breakdown.md` "Completion gate", twice: *"or named a successor"* — a unilateral
  per-cycle deferral hatch, available to exactly the lanes SD-32 was going to unblock.

Run in the numbered order, SD-31 would have closed legitimately with its race/`race_trait` lanes and
its `unknown`-bucket disposal deferred to a successor, SD-32 would then have built the capability that
unblocked them, and the deferred work would have had to be reopened — the ordering failure discovered
only after both packages reported closure.

**The evidence, re-derived 2026-08-15** by importing the dashboard producer's own
`doneness_verdict()` and replaying it over `docs/work-inventory.json` (`beginner_box` excluded, as the
live producer excludes it):

```
python3 -c "
import json, importlib.util
spec = importlib.util.spec_from_file_location('m','scripts/observer/pf1e_dashboard_producer.py')
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
S=['deferred-with-reason','fixture-verified','grounded','ingested-magnitude','literal-verified',
   'not-ingested','not-started','text-complete','unknown']
for wc in ('ambiguous','display','static','derived','computed'):
    print(wc, [s for s in S if mod.doneness_verdict(wc,s,'race_trait')=='done'])
"
# ambiguous []   <- no status reaches done
# display   ['text-complete']
# static    ['fixture-verified', 'literal-verified']
# derived   ['fixture-verified', 'literal-verified']
# computed  ['grounded']
```

| population gated on capability, not on grind | units |
|---|---:|
| `wiring_class == ambiguous` — **no path to `done` at any status** | 2,109 |
| `unmeasurable` (all `status == unknown`; `class_feature` 3,622 + `feat` 367) | 3,989 |
| `race` + `race_trait` not yet `done` | 3,284 |
| **distinct union** | **8,524 — 22.1 % of the 38,521-unit board** |
| **board ceiling reachable without the capability builds** | **77.9 %** |

That 77.9 % is an independent re-derivation of the ~81 % "honest ceiling" `SD-30
state-goals-and-lessons.md §2.3` recorded before `§45` superseded it with the 100 % mandate. The
ceiling figure was right; `§45`'s answer was to build the capability rather than descope — and the
split then scheduled that capability *after* the work depending on it.

**A figure correction this derivation produced.** `SD-30 decisions.md §45` item 2 and this package's
absorbed Epic 2 both describe the target as "the ~3,547 unmeasurable units incl. 2,109 `ambiguous`."
Those are **two nearly disjoint populations, not a nested one**: 3,989 `unmeasurable` and 2,109
`ambiguous`, overlapping by **119**. The real Epic 2 target is their union, ~5,979 units. The scope is
larger than the charter stated, not smaller. `retro.py correction` emitted.

**Disposition:**

1. **`docs/release/SD-32-engine-capability-builds/` is absorbed into this package and deleted**,
   following the precedent `SD-30 decisions.md §50` set when it absorbed the previous SD-32. The
   number `SD-32` is free for reuse. SD-32's Epics 1 and 2 become **this package's Epics 1 and 2**;
   its Epic 3 (cloud fan-out) merges with this package's own into a single Epic 8 — the two packages
   carried independent copies of identical rules, and one copy cannot drift from itself.
2. **Directory name unchanged** (`SD-31-corpus-closure-grind`). It is cited from a shipped Rust test
   and a production Python script as well as from SD-29/SD-30/SD-33 docs; a rename buys accuracy of
   slug at the cost of touching code for a cosmetic gain, and this program has already paid for one
   rename dance this month (`§51`'s SD-31→SD-33). The widened scope is stated in `README.md`'s
   opening instead.
3. **Epics re-sequenced** to the ten-epic order in `epic-breakdown.md`: capability first (Epic 1 race
   chassis, Epic 2 verdict paths), then the `class_feature` chain, then the ingest lanes that consume
   the capability, then onboarding, fan-out and closure. Full renumber map in `README.md`.
4. **The two dependencies that were cross-SD are now internal hard gates**, stated on the cards
   themselves: no `race`/`race_trait` ingest cycle claims a book before Epic 1 has landed a chassis
   for the races that book's rows reference; no `unknown`-bucket characterization or disposal cycle
   (Epic 3-F4, Epic 5-F3) claims before Epic 2 has landed. A cycle claiming across an open gate is
   out of protocol, exactly as the PI-gate block already was.
5. **The deferral hatch is closed.** "Or named a successor for the remainder" is struck everywhere it
   appeared. A unit leaves the 100 % bar only through the **Structural Exclusion Register**
   (`acceptance-and-verification.md AT-31-100`) — see Decision 3.
6. **A standing reachability audit becomes Epic 0** — see Decision 4.

**Authority:** operator ruling 2026-08-15 (verbatim above); dependency evidence re-derived the same
day by the commands recorded in this section.

## Decision 3 — The Structural Exclusion Register replaces per-cycle deferral (2026-08-15)

**Status:** New. Follows from Decision 2 item 5.

Under the 100 % mandate a cycle may not retire a unit from the bar by its own judgment. The prior
language — "reached its measured workable-pool ceiling **or named a successor for the remainder**" —
let any cycle convert unfinished work into someone else's problem without an operator ever seeing it,
and it was pointed at precisely the lanes whose ceiling was an artifact of scheduling rather than of
capability.

**The rule.** A unit may be excluded from the 100 % denominator only by an entry in the Structural
Exclusion Register carrying all four of:

1. the exact command, run this cycle, showing that no path to `done` exists for that unit;
2. a statement of *which capability is missing*, and why building it is genuinely impossible or
   out-of-charter — not merely expensive. **Cost is never an exclusion reason**; `SD-30
   loop-instruction.md` "Stop vs. press on" already binds: no scope is too big to just do;
3. the reachability-audit run (Epic 0) that reproduces the finding independently;
4. **operator sign-off**, recorded with its date. A cycle may *propose* an exclusion; it may not
   grant one.

An unsigned proposed exclusion leaves the unit in the denominator, and the epic stays open. This is
the mechanism that makes "SD-31 closed but the mandate silently didn't" a state that cannot be
reached without an operator having said so in writing.

## Decision 4 — Epic 0: the reachability audit is a standing gate, not a report (2026-08-15)

**Status:** New. Follows from Decision 2 item 6.

The `ambiguous` dead-end (2,109 units with no path to `done` at any status) existed in the engine
before either successor package was written, is trivially detectable — the query in Decision 2 is the
whole of it — and was found by neither package's authoring pass. It was found only when the operator
asked whether the sequencing could strand the mandate.

**The rule.** `scripts/reachability_audit.py` answers one question mechanically, for every unit on the
board: *given current engine capability, does a path to `done` exist?* It runs:

- before the first cycle of this package fires;
- at every epic closure;
- before any closure receipt is written.

It publishes the **reachable ceiling** — the share of the board that could reach `done` if every
in-flight lane succeeded perfectly. A reachable ceiling below 100 % is a **capability gap with a
name**, and either an epic owns it or it goes to the Structural Exclusion Register with operator
sign-off. It is never a number a closure receipt may quote and pass over.

The audit must be proven able to fail before it is trusted (`SD-30
state-goals-and-lessons.md §3.1`: this repo has shipped three gates that could not fail). Its own
tests corrupt a wiring-class/status pair and confirm the audit reports the resulting dead-end.
