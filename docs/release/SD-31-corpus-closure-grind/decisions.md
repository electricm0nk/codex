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

**Correction, 2026-08-15 (launch-readiness remediation Step 5, drift D2).** The epic numbers above
("Epic 3 (chassis sweep), Epic 4 (ingest lanes), or Epic 5 (book onboarding)") are this package's
*original* split-time numbering, superseded by `decisions.md §2`'s 2026-08-15 re-sequencing. Under
the live renumber map (`README.md` "Epic renumber map"), chassis sweep is **Epic 5**, corpus-wide
ingest lanes are **Epic 6**, and book onboarding is **Epic 7**. The gate itself is unchanged — it
binds whichever epics currently carry those three functions, by function, not by number — and the
original sentence is left visible above per this program's doc convention; read "Epic 3/4/5" there as
"Epic 5/6/7" today.

### (d) The concurrency/cloud fan-out protocol

Reproduced from `SD-30-class-feature-archetype-bundle/decisions.md §47`: hardware baseline (8 cores /
45GB RAM / 968GB disk at 19% used, captured 2026-08-14), concurrency cap of three concurrent
build-capable agents (re-derived empirically per the existing budget discipline), and the cloud
fan-out rules — every cloud agent works its own branch, never two writers on one branch; the local
orchestrator owns all merges to `tranche/11` (updated from `tranche/10` by `decisions.md §6`, the
tranche/11 branch cut), verified by content not commit count; DoD-8 on-screen verification and
dashboard-producer work stay local. Governs this package's Epic 6.

**Superseded 2026-08-15 (launch-readiness remediation Step 5, drift D2), on two counts, per this
program's "original text stays visible, correction points forward, dated" convention:**

1. **The hardware baseline above is stale twice over.** `SD-30-class-feature-archetype-bundle/
   loop-instruction.md`'s own live block (`SD30-PRELAUNCH-002`, re-measured after `§47` itself) found
   the box had moved again the same day, past `§47`'s 8-core/45GB capture, to **24 cores, 167 GiB RAM,
   968 GB disk at 19% used**, with a re-derived **concurrent full-gate-agent cap of 8** (disk-bound,
   693 G headroom ÷ ~82 G per full-gate `CARGO_TARGET_DIR`), not the 3-agent cap named above. Re-derive
   both figures (`nproc`, `free -h`, `df -B1G /`, the `du -sh` sizing) at time of use, per that file's
   own standing instruction — the numbers above are a snapshot, not a constant, and were already
   snapshot-stale the day `§47` was written.
2. **"Governs this package's Epic 6" is this package's pre-merge numbering.** Under the current
   renumber map (`README.md`), the cloud fan-out protocol is **Epic 8**, not Epic 6 (Epic 6 is now
   Corpus-Wide Ingest Lanes). Read "Epic 6" above as "Epic 8."

**Authority:** operator ruling, 2026-08-14, transcribed in the dispatch brief for this split;
`SD-30-class-feature-archetype-bundle/decisions.md §43-§51` (the widened-charter, 100%-mandate, and
split decisions this package's scope descends from).

### (e) The classifier accuracy-not-movement rule (LOAD-BEARING, binds Epic 2 directly)

**Absorbed 2026-08-15 from `SD-32-engine-capability-builds/decisions.md` Decision 1(b) (`decisions.md
§2`).** It is lettered (e) here because (b) in this package's Decision 1 is already the table-sheet
doctrine; any inherited citation reading "SD-32 Decision 1(b)" resolves to this subsection.

Reproduced with edits noted (not exactly — see the correction below) from
`SD-30-class-feature-archetype-bundle/decisions.md §50(c)` (originally the deleted SD-32 package's
Decision 3):

> **Decision.** The classifier that resolves `ambiguous` (2,109 units) and re-examines
> `display`+`grounded` (1,243 units) is accepted or rejected on **agreement with a hand-labelled
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

**Correction, 2026-08-15 (launch-readiness remediation Step 5, drift D9).** SD-30's `§50(c)` — and,
by inheritance, this subsection before this correction — carried the historic split-time figure
"`ambiguous` (360 units)". Re-derived this cycle by importing the dashboard producer's own
`doneness_verdict()` and counting `wiring_class == 'ambiguous'` over `docs/work-inventory.json`
(`beginner_box` excluded, as the live producer excludes it):

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))['units']
c = collections.Counter(u.get('wiring_class') for u in d if u.get('book') != 'beginner_box')
print(c)
"
# Counter({'display': 14366, 'computed': 8477, 'static': 7394, 'derived': 6175, 'ambiguous': 2109})
```

The true count is **2,109**, already the figure this package's own README/Decision 2 use elsewhere
(the 360 was a stale pre-widening snapshot that survived only inside this one reproduced quote). The
quote's other figure, `display`+`grounded` (1,416), is likewise stale — re-derived the same way
(`wiring_class == 'display' and status == 'grounded'`) at **1,243**, matching `AT-31-010`'s binding
figure elsewhere in this package. Both are corrected in place above; the quote is therefore no longer
an exact reproduction of SD-30 §50(c), hence "reproduced with edits noted," not "reproduced exactly."
The rule itself — the acceptance criterion, the four numbered clauses, the rationale — is unchanged;
only the two population counts moved.

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

## Decision 5 — The mandate denominator (operator ruling 2026-08-15)

**Status:** New. Landed by the SD-31 launch-readiness remediation pass, blocker B1/B3
(`~/.claude/plans/conduct-a-launch-readines-zesty-ripple.md`).

**The defect this ruling fixes.** Before this decision, the package's own completion gate
(`epic-breakdown.md` "Completion gate", `SD31-E9-F1`, `AT-31-102`) required only that
`scripts/reachability_audit.py`'s **reachable ceiling** — the share of the board a unit *could* reach
`done` for, if every in-flight lane succeeded — hit 100 %, or be signed off unit-by-unit. No acceptance
test anywhere required `done / denominator == 100 %`. The readiness review that surfaced this
(`~/.claude/plans/conduct-a-launch-readines-zesty-ripple.md`, blocker B1) found the reachable ceiling —
before `scripts/reachability_audit.py` exists (it is Epic 0's own not-yet-built deliverable, so this
figure is not independently re-derived in this decision; it is carried here by source) — at
**36,412 / 38,521 = 94.53 %** while the board sits at **5,837 done**: the gate as written is
satisfiable by giving the 2,109 `ambiguous` units one `done`-producing status, with the board still at
15 % actually done. `AT-31-005`'s per-kind floors compound this: they are stated as
`done+held` floors (e.g. race_trait `513 / 3,447, 14.9%, done+held`), and Decision 1(a)'s anti-gaming
rule explicitly forbids counting `held` as `done` — a floor a cycle can satisfy without any unit
reaching `done` is not a closure criterion, whatever it is labelled.

**Operator ruling, 2026-08-15, as recorded in the launch-readiness plan's Context section** (this
review's four rulings, taken together; the denominator ruling is the first):

> Denominator = everything, strictest. All 37 non-`beginner_box` books incl. the 7 `future_state` ones;
> `unmeasurable` and `deferred` stay IN until `done` or operator-signed exclusion. The dashboard
> headline must show this number.
>
> Oracle = pin + bootstrap script, not vendored (public repo; `.gitignore` policy).
>
> Open cards for the six unowned kinds and add a hard done-% bar to Epic 9.
>
> Commit the other session's three dirty files as-is on `tranche/10` in their own commit.

This decision records the first of the four (the denominator and the doneness bar); the oracle pin, the
six kind cards, and the housekeeping commit are recorded by the plan's other remediation steps.

**The rule.** The mandate denominator is **every unit in `docs/work-inventory.json` except the books in
`pf1e_dashboard_producer.EXCLUDED_BOOKS`** (today `{"beginner_box"}`, 1 of 38 books). It includes:

- the 7 `future_state` books (`scope == "future_state"` on the book row) — already present in the
  inventory (4,094 units), not yet onboarded by Epic 7;
- `unmeasurable` units (`status == "unknown"`, no instrument exists yet) — 3,989;
- `deferred` (`deferred-with-reason`) units — 36.

None of these leave the denominator except through an operator-signed Structural Exclusion Register
entry (`decisions.md §3`, `AT-31-100`).

**Re-derived this cycle** (command below, run against `docs/work-inventory.json` at this cycle's HEAD):

```
python3 -c "import json,sys,collections; sys.path.insert(0,'scripts/observer'); import
pf1e_dashboard_producer as P; U=[u for u in json.load(open('docs/work-inventory.json'))['units'] if
u.get('book') not in P.EXCLUDED_BOOKS]; c=collections.Counter(P.doneness_verdict(u.get('wiring_class'),
u.get('status'), u.get('kind')) for u in U); print(c, len(U))"
```

```
Counter({'not-started': 20895, 'held': 6916, 'done': 5837, 'unmeasurable': 3989, 'in-progress': 848,
'deferred': 36}) 38521
```

**Mandate headline: 5,837 / 38,521 = 15.15 %.**

**The pre-ruling headline, kept only as a labelled secondary.** The dashboard's live `usableDenom()`
(`~/swarm-observer/PF1e-dashboard.html renderCompletion()`) computes over `inScopeUnits()` — books
with `scope == "in_scope"` only (30 of 38 books; the 7 `future_state` books and `beginner_box`
excluded) — minus `unmeasurable` and `deferred` from that narrower population. Re-derived this cycle
by replaying the same book-scope filter over `docs/work-inventory.json`:

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer')
import pf1e_dashboard_producer as P
d = json.load(open('docs/work-inventory.json'))
in_scope_ids = {b['id'] for b in d['books'] if b.get('scope') == 'in_scope'}
U = [u for u in d['units'] if u.get('book') in in_scope_ids]
c = collections.Counter(P.doneness_verdict(u.get('wiring_class'), u.get('status'), u.get('kind')) for u in U)
denom = len(U) - c['unmeasurable'] - c['deferred']
print(c, 'denom', denom, 'pct', round(c['done']/denom*100,2))
"
```

Reproduces exactly: `done=5,837`, `denom=30,402`, **19.20 %** — matching the figure `B3` in the
readiness review cited from the live dashboard byte-for-byte. **This figure is superseded by the
15.15 % strict mandate headline above and is kept below it, labelled "in-scope, measurable-only
(secondary)", never as the package's own headline.**

**The invariance property.** Under the strict rule, the denominator is fixed at 38,521 by the set of
non-`EXCLUDED_BOOKS` units already in `docs/work-inventory.json` today — it does not grow or shrink as
work lands:

- **Epic 2** (resolving `unmeasurable`/`ambiguous` units into a `done`-reachable path) does not touch
  the denominator — those units are already counted, today, as `unmeasurable`/`held`/`not-started`.
  Epic 2's effect is entirely on the *numerator* (or on the reachable-ceiling audit), never on the
  denominator.
- **Epic 7** (onboarding the 7 `future_state` books) does not touch the denominator either — those
  4,094 units are already inside the 38,521 (re-derived above: 7 `future_state` books, 4,094 units,
  confirmed present in `docs/work-inventory.json` today). Epic 7 moves those units toward `done`; it
  does not add them to the board.

**Only a unit reaching `done` moves the mandate headline.** This is the property the pre-ruling
headline lacked: because `usableDenom()` scopes to `in_scope` books, an Epic 7 book flipping from
`future_state` to `in_scope` would have *widened* that denominator the same cycle it started
contributing `done` units, so the old headline could read as a regression (denominator growth
outpacing numerator growth) for work that is unambiguously forward progress. The strict headline
cannot do that: the 38,521 is already the full board.

**What this changes in the package**, landed together with this decision:

- `epic-breakdown.md` `SD31-E9-F1` and "Completion gate" gain the **doneness bar**
  (`done / denominator == 100 %`, or every shortfall unit signed off) **alongside, not instead of**,
  the existing reachability-ceiling bar. A closure that satisfies only the reachable-ceiling bar no
  longer passes.
- `acceptance-and-verification.md` gains **AT-31-103 — Doneness bar**, with the replay command above.
- `AT-31-005`'s per-kind `done+held` figures are relabelled **progress floors**, not closure
  criteria — useful as a per-cycle "is this kind moving" signal, never sufficient on their own to
  close an epic or the package.
- `README.md`'s Purpose/Exit statement names the doneness bar explicitly, not only the reachable
  ceiling.

**Retro.** A correction is emitted for the denominator ambiguity (38,521 strict vs. 30,402
measurable-secondary) — the package's own text used the 38,521/15.15 % figures in `README.md` and
Decision 2 already, but no acceptance criterion bound the package to them, and `AT-31-005`'s floors and
the pre-ruling exit gate together left the 100 % mandate satisfiable at 15 % `done`. No new
`retro.py decision` event is separately warranted — this file's own Decision 5 entry, dated and
attributed, already is that record.

**Authority:** operator ruling 2026-08-15 (verbatim above, as recorded in the launch-readiness plan);
figures re-derived the same day by the commands in this section.

## Decision 6 — Release 0.11.<build> on tranche/11 (operator ruling 2026-08-15)

**Status:** New. Operator ruling, 2026-08-15, verbatim: "it goes without saying - SD-31 will be
release 0.11.xxx. And will be operating against branch tranche/11."

**The cut.** `tranche/11` was cut from `tranche/10`'s tip, commit `1980d6b95` — not from `develop` —
because `PR #363` (`tranche/10 -> develop`) is still open (the operator holds sole merge authority and
has not merged it). This mirrors the precedent set at the `tranche/9 -> tranche/10` boundary: `PR #360`
(`tranche/9 -> develop`) was a true merge commit, and `tranche/10` was cut from `develop` at that
point, cleanly. Here, with `#363` still open, cutting from `develop` would have missed all of SD-30's
work; cutting from `tranche/10`'s own tip instead means `tranche/11` carries SD-30's full history. A
later `git merge origin/develop` into `tranche/11` is expected to be clean once `#363` lands (SD-30's
work will already be present via the shared history), and the eventual `tranche/11 -> develop` PR will
show only SD-31's own diff against that merged base.

**This program's rule, restated so Epic 9 does not re-derive it:** the tranche digit bumps only on a
NEW `tranche/N` branch cut for the next bundle — never automatically at a bundle's own closure while
still on the same tranche branch (`SD-22 decisions.md §2`, `SD-21 decisions.md §18`). `tranche/11`
being newly cut (this ruling) is what authorizes the advance from `0.10` to `0.11`; nothing else does.

**First concrete value.** `0.11.<build>`, where `<build>` is the current build-counter state
(`GITHUB_RUN_NUMBER` on `publish-tester-release.yml`) at the time of cycle close. The last completed
run of that workflow before this cycle was run `#123` (success, 2026-08-14, `PR #360` merge to
`develop`); the next publish from this lineage would stamp `0.11.124`. Landed as commit `147f1c2b7`
on `tranche/11` this cycle (`SD31-E10-F1-001`).

**Closing-PR increment rule** (restated from `SD-30 decisions.md §15` / the 2026-07-17 build-version
amendment, so this package does not re-derive it): major stays `0` until first main-publish;
tranche-base is the numeral in the branch name (`11`); build is a monotonic counter that never resets.
Tranche-promotion increments only on the `tranche/11 -> develop` PR. The closure Epic's (Epic 9's)
recorded value is `0.11.<last_build>` at the time that PR is opened.

**What this changes in the package**, landed together with this decision: `README.md`,
`scope-draft.md`, `epic-breakdown.md`, and `kanban.md` frontmatter move `canonical_branch` to
`tranche/11` and `build_version_target` to `0.11.<build>`; `loop-instruction.md` and
`technical-requirements.md` update their checkout instructions; `epic-breakdown.md` gains **Epic 10**
recording the bump itself; `risks-and-open-questions.md` risk 5 is restated for the new shared-history
shape; historical receipts in `progress.md` that say `tranche/10` are left as written — they were true
when recorded — with a dated note added at the top instead.

**Authority:** operator ruling, 2026-08-15, verbatim above.

## Decision 7 — The prose done-bar: zero-magnitude units finish, they do not leave the denominator (operator ruling 2026-08-16)

**Asked:** sign a Structural Exclusion Register entry (`§3`) removing the 404-unit
`ambiguous:prose_scaling_phrase` / `ambiguous:prose_ability_scaling` population from the 100 %
denominator. Logged as `artifacts/OPEN-ISSUES.md` row 36, proposed by `SD31-E2-F3-001`.

**Ruled, verbatim:**

> "if they are prose only, nothing to compute, and the prose is available to print in the
> description on the character sheet, then it counts as done"

**The exclusion is DECLINED and a done-bar granted instead.** The 404 units stay in the denominator
and gain a real path to `done`. Row 36's proposal is answered and closed; nothing leaves the
mandate. `§3`'s register remains unsigned and empty.

### The bar — three conjunctive conditions

A unit counts `done` under this decision when ALL THREE hold:

1. **Prose only** — the record's content is text, not a mechanic carrying a magnitude.
2. **Nothing to compute** — no value the engine would have to derive, scale, or apply.
3. **The prose is available to print in the description on the character sheet.**

**Condition 3 is load-bearing and is not satisfied by a record merely existing.** It is DoD-8
shaped: the description must be populated from the real corpus row AND the sheet must render it,
proven on-screen, not inferred from a green code gate. Three compute twins in this program's
history each passed a code gate while showing nothing on the sheet (`SD-30 decisions.md §28`).
A unit whose description is empty, placeholder, or unrendered is **not** done under this decision,
and marking one so is the anti-gaming violation Decision 1(a) forbids.

### Continuity

This restates, for the corpus board, the standing v0.6 ruling of 2026-07-28 — *"you just include
the feat description. there is no need to calculate anything… all of our feats need to have
descriptions on the feats tab and on the printed page. that's all you need to do."* That ruling
unstuck the v0.6 class matrix, which had sat frozen for a week because zero-magnitude features were
being counted as unbuilt work. Same category error, same correction, wider scope.

### Size — re-derived 2026-08-16 at `89846f5c9`, not transcribed

```
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book') not in P.EXCLUDED_BOOKS]
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
zero=[u for u in U if u.get('magnitude_token_count')==0]
nd=[u for u in zero if v(u)!='done']
print(len(zero), len(nd))
print(collections.Counter(u.get('kind') for u in nd).most_common())
print(collections.Counter(u.get('wiring_class') for u in nd).most_common())
"
```

- **16,812 of 38,521 units (43.6 %) carry `magnitude_token_count == 0`**; 2,226 are already `done`.
- **14,586 are not done — 37.9 % of the whole board.**
- By kind: `class_feature` 8,637 · `monster_ability` 1,958 · `race_trait` 1,769 · `feat` 928 ·
  `companion` 520 · `spell` 406 · `equipment` 200 · `equipment_modifier` 134.
- By wiring class: `display` 12,762 · `derived` 1,339 · `ambiguous` 305 · `static` 161 ·
  `computed` 19.

The 404 that prompted the ruling are a small corner of what its own wording reaches.

### PROXY WARNING — binding, and it must be discharged before any unit is banked

`magnitude_token_count == 0` is **the engine's own token count**, used above as a proxy for
conditions 1 and 2. It is not the ruling. Per the standing rule to validate a proxy where it makes
its confident claim (`SD-30`'s instrument-failure record; 8 in one session, all name-shaped), a
cycle must first hand-verify a stratified sample of zero-magnitude units against their **whole**
corpus rows and confirm the row genuinely carries no computable magnitude — held to the same
whole-record standard as Epic 2's ground-truth sample, and for the same reason: this proxy would
silently decide ~14,586 units and nothing downstream would catch it being wrong. Where the proxy is
wrong, the ruling still applies — to the units that genuinely meet conditions 1 and 2.

**Authority:** operator ruling, 2026-08-16, verbatim above.

## Decision 8 — A `done` credit resting on a non-shipping code path is wired, not retracted (operator ruling 2026-08-16)

**Asked:** `SD31-E6-F11-002` (wave 3) moved 7 `monster` units to `done` through a new evaluator
seam, `spell_like_ability_caster_level()`. The Opus adversarial verifier accepted the evidence as
clean — every fixture value reproduced byte-for-byte from the pinned `.lst` rows — but found the
function had **zero production callers anywhere in the tree**, so the credit rested on a code path
no player could reach (`artifacts/OPEN-ISSUES.md` row 44). Should the 7 be retracted, or wired?

**Ruled, verbatim:**

> "you were right to say wire up the 7 for real."

**Wire it. Do not retract.** The correct response to a `done` credit resting on a non-shipping code
path is to make the path ship.

**Discharged the same day** by `SD31-E6-F1-002`: `spell_like_ability_caster_level()` is now called
from `apps/desktop/src-tauri/src/monster_catalog.rs`'s `map_chassis_monster`, serves
`MonsterCatalogEntryDto.spellLikeAbilityCasterLevel` over the real `list_monster_catalog` Tauri
command, and renders on `MonsterCatalogScreen.tsx` — proven by two new Rust tests **and a DoD-8
on-screen screenshot of Demon (Balor) showing "Spell-like abilities CL 20"**. A
`has_spell_like_abilities` presence gate was added so the function never serves a caster level to a
monster that has none. Row 44 is closed.

**What does NOT change:** the adversarial finding was correct and the bar it asserted stands — an
evaluator nothing ships is not a path to `done`. The credit became legitimate when the wiring
became real and was proven on-screen, not when the ruling was quoted.

### The precedent both of today's rulings set, stated generally

> When a cycle proposes to **remove** something from the mandate — exclude units from the
> denominator, retract credit, defer a remainder, declare something out of scope — the answer is
> **"no, finish it properly."** The operator supplies the finish line, not the exit.

Decision 7 declined a 404-unit exclusion and granted a done-bar. Decision 8 declined a 7-unit
retraction and required the wiring. Neither accepted a reduction in scope. This is consistent with
what already stands: deferral was struck from this package (`§2` item 5), cost is forbidden as an
exclusion reason (`§3`), and no scope is too big to just do. **Cycles should stop drafting exclusion
proposals as a primary deliverable** — build the path, and propose exclusion only for something
genuinely impossible, never merely expensive.

**Authority:** operator ruling, 2026-08-16, verbatim above.

### Decision 7 — CORRECTION to this section's own sizing (2026-08-16, `SD31-D7-PROSE-001`)

**The `magnitude_token_count == 0` figures above are wrong, and this section said they might be.**
Decision 7's PROXY WARNING required a cycle to hand-validate the proxy before any unit rode on it.
`SD31-D7-PROSE-001` did exactly that and the proxy failed badly:

```
python3 scripts/sample_ground_truth_units.py --inventory docs/work-inventory.json \
  --zero-magnitude-only --target-per-cell 4 --seed 31 \
  --out artifacts/SD31-D7-PROSE-001-proxy-sample-draw.json      # 121 units, 36 cells, 5 wiring classes, 10 kinds
python3 -c "import json,collections; r=json.load(open('docs/release/SD-31-corpus-closure-grind/artifacts/SD31-D7-PROSE-001-proxy-sample-evidence.json')); print(collections.Counter(x['hand_genuinely_zero_magnitude'] for x in r))"
```

- **Raw proxy: 57 of 121 hand-checked units (47 %) genuinely DO carry a magnitude** despite
  `magnitude_token_count == 0`. The "14,586 not-done zero-magnitude units" figure above is therefore
  **an upper bound that roughly doubles the real population**, not an estimate of it.
- **Gated additionally on `wiring_class == display`, the miss rate falls to 9 of 121 (7 %)**, and
  those 9 are a single named failure mode — flat, non-scaling numerics in `SPROP:`/`BENEFIT:` fields
  that the wiring-class classifier does not read (`OPEN-ISSUES.md` row 69).

**The ruling is unchanged; only its scope estimate is.** Decision 7's three conditions are the bar;
`magnitude_token_count` was never the ruling, and this correction is the proxy warning working as
written. Future cycles size this population as **`wiring_class == display` AND
`magnitude_token_count == 0`**, and re-derive rather than quoting either figure.

**The orchestrating session stated the 14,586 figure to the operator when reporting Decision 7.
That statement is corrected here rather than quietly dropped.** `retro.py correction` emitted.

### Decision 7 — the structural blocker the ruling now runs into (2026-08-16, `SD31-E4-F1-001`)

`SD31-E4-F1-001` wired Slayer's three remaining archetype-supersession slots for real (clearance
table 4/7 → 7/7, 6 new tests green, a real `if let`/`else` branch plus three previously-absent
archetype records) and **the board did not move by one unit.** Root cause, fully reproduced: the
dashboard producer's `doneness_verdict()` maps `display` + `grounded` to **`held`**, not `done`.

Under Decision 7 a `display` unit that is prose-only, has nothing to compute, and whose description
renders on the sheet **is** done. That cell is therefore the single point where the operator's
ruling is currently blocked from paying out, for `class_feature` and for every other kind.

`SD31-D7-PROSE-001` built the correct mechanism for exactly this — a rung that requires the
description to be present, to byte-match the corpus row, and to render — and applied it to
`race_trait` (+146 units, DoD-8 proven on-screen). **Extending that same rung to the
`display`+`grounded` population generally is the next cycle's headline, and it must be the rung, not
a table edit**: widening the cell without the description-and-render requirement is precisely the
Decision 1(a) violation this package forbids, and it is how the 1,060 wrongly-`done` units below
happened in the first place.

### Decision 7 — what condition 3 caught on its first real application

Building the bar's condition 3 immediately exposed **1,060 units that had been counted `done` with
an empty or null corpus description** — 836 `equipment_modifier`, 212 `equipment`, 11 `feat`,
1 `spell` — i.e. marked complete with nothing for a player to see. Condition 3 had never been
checked before it existed. They are demoted, and the board's headline fell **7,603 → 7,340
(19.74 % → 19.05 %)** across the wave as a result.

**That decrease is the ruling working, not a regression.** The prior figure was inflated. Recorded
here so no later cycle "fixes" the drop by reverting the check.

A known, quantified under-claim rides with it: `closure_has_real_description()` reads only the raw
`.lst` closure and not the already-ingested corpus JSON's `data.description`, so **247 of the 1,060**
demoted units genuinely do have a real description that the check cannot see (`OPEN-ISSUES.md`
row 70). The conservative direction was shipped deliberately; recovering those 247 is owed work, not
a defect to argue about.

## Decision 9 — `core_essentials` is not a book; it leaves scope, but only after its content is re-attributed (operator ruling 2026-08-16)

**Ruled, verbatim:**

> "I did some digging, there is no such book in pathfinder 1e as 'core essentials'   it should be
> removed from scope and ignored.   If you see one listed in the pcgen corpus, it might be a
> community construct that just groups together 2 or 3 common books everyone considers essential.
> we can safely ignore that if we ingest all the real books."

**The operator is right, and the corpus proves it.**
`$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/core_essentials/_core_essentials.pcc` declares
`CAMPAIGN:Core Essentials` — a PCGen *packaging bundle* — and its own `#SOURCELONG:Core Essentials`
and `#SOURCESHORT:CE` are **commented out**, so it asserts no book identity at all. Its
`#SOURCEWEB` points at Paizo's free-downloads page and `#SOURCEDATE:2009-08` is the Core Rulebook's
release month. Meanwhile every `.lst` file inside declares its own real source:

```
grep -rhoP 'SOURCELONG:[^\t]*' "$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/core_essentials/" \
  | sort | uniq -c | sort -rn
#   53 Core Rulebook     6 Ultimate Magic    5 Ironfang Invasion   2 Bestiary 6/5/4 (each)
#   19 Bestiary          6 Bestiary 2        3 Bestiary 3          1 Universal Rules
#                                            3 Advanced Player's Guide
#                                            1 Pathfinder Player Companion: Blood of the Moon
```

**~11 real books aggregated under a name that is not one of them.**

### THE CONDITION BINDS: re-attribute first, drop the label second

The ruling's own clause — *"we can safely ignore that if we ingest all the real books"* — is the
whole safety property. **Adding `core_essentials` to `EXCLUDED_BOOKS` before re-attribution would
silently delete real content from the denominator**, the shape Decision 1(a) forbids and the
operator has twice declined. The label must end at **zero units because every unit moved**, never
because the bucket was excluded. Enforce it with the attribution contract gate `SD31-ATTRIB-001`
built: a unit stamped `core_essentials` is a gate failure, not a silent label.

### The re-attribution is provable — 92 % resolves mechanically

Re-derived 2026-08-16 at `5d0cd1595` by joining each still-labelled unit's `source_file` to that
file's own `SOURCELONG` header in the pinned oracle:

| | units |
|---|---:|
| still labelled `core_essentials` | **644** (`monster_ability` 378, `race_trait` 258, `race` 8) |
| **resolved by the file's own `SOURCELONG`** | **591 (92 %)** — `Bestiary` 545, `Core Rulebook` 46 |
| unresolved (no uncommented `SOURCELONG`) | **53**, across 12 files |

The 53 are four races and their ability files: `aquaticelf` 13+1, `ghoran` 12+1, `android` 11+1,
`gathlain` 9+1. Next signal for those: each race's own `core_essentials/races/<race>/_race.pcc`,
which `SD31-ATTRIB-001` already used successfully for its 44-race hand-derived table. **A race that
still cannot be resolved is left unattributed and said so** — a confidently wrong label is what
created this defect.

**Watch for books outside the roster.** `Ironfang Invasion`, `Pathfinder Player Companion: Blood of
the Moon` and `Universal Rules` appear in that census and are not obviously on the 37-book mandate
roster. A unit resolving to a book we do not carry is **a scope question to log, not a unit to
delete** — the ruling's condition is "if we ingest all the real books."

**Also noted:** ids still carry a stale `core_essentials:` prefix even where `book` has been
repaired (e.g. `id=core_essentials:race:catfolk` with `book=bestiary_3`). The id namespace needs the
same repair, or the fake book survives in every receipt that quotes an id.

**Authority:** operator ruling, 2026-08-16, verbatim above.

## Decision 10 — the Supersession Register: duplicates do not inflate the denominator; newest printing wins (operator ruling 2026-08-16)

**Ruled, verbatim:**

> "we need to track, in writing, all the superseded objects and the sourcebooks. i dont want
> duplicates falsely adding to the denominator. if a duplicate is found, the most recent publishing
> takes precedence and the older one is flagged as supersceded/out of scope."

and, clarifying with a worked example:

> "if catfolk exists as a race in beastiary and advanced race guide - thats a duplicate. most recent
> publish wins"

### This is a SECOND register, and it must not be conflated with §3

| | Structural Exclusion Register (§3) | **Supersession Register (this decision)** |
|---|---|---|
| removes a unit because | finishing it is genuinely **impossible** | it is a **duplicate** of a newer printing |
| authorization | per-entry **operator signature** | this ruling is a **standing rule** a cycle may apply |
| bar | impossible, never merely expensive | **proven to be the same object**; newest printing wins |
| state today | **empty** — twice declined | to be built |

Tracked **in writing**, covering both superseded **objects** and superseded **sourcebooks**.
Publication order is provable from the PCGen `.pcc` headers' `SOURCEDATE:` — use it; never date a
book from memory.

### The Catfolk clarification widens this from deletion to ATTRIBUTION

The worked example resolves an ambiguity the first statement left open. Today the corpus ships
**one** Catfolk race record (`catfolk_races.lst`, whose row carries only `SOURCEPAGE:p.xx`), so
there is no duplicate *record* to delete — but Catfolk is printed in **Bestiary 3 (2011)** and in
the **Advanced Race Guide (2012)**, and ARG is the newer printing. **The unit therefore belongs to
ARG.** So the rule governs attribution wherever multiple books print an object, not only cases where
we hold two rows.

**This resolves the operator's earlier `advanced_race_guide` observation** ("ARG reports as nearly
untouched"): ARG shows **1** race unit because its reprinted races are filed to the Bestiary line.
Under this decision they file to ARG.

**The provable signal, re-derived 2026-08-16:** a book prints a race if that book's own `.lst` files
carry rows for it — ARG's `arg_abilities_race.lst` carries `Catfolk ~ Cat's Claws`, `Catfolk ~
Clever Cat`, `Catfolk ~ Climber`, `Catfolk ~ Curiosity`, `Catfolk ~ Nimble Faller`, `Catfolk ~
Scent`, plus `arg_feats.lst`'s `Catfolk Exemplar` line and `arg_equip_arms_armor.lst`'s
`Claw Blades (Catfolk)`. Scanning that signal across all books:

> **50 of the 103 `race` units are currently attributed to a book older than another book that also
> prints them.** Examples: Aasimar (now `bestiary`, also ARG) · Dhampir (now `bestiary_2`, also
> ARG) · Changeling (now `bestiary_4`, also ARG) · Fetchling, Grippli (now `bestiary_2`, also ARG)
> · Drow, Duergar, Goblin, Hobgoblin (now `bestiary`, also ARG).

### THE TWO GUARDS — non-optional, both derived the hard way

**1. A shared NAME is not a duplicate.** Matching `(kind, name)` implicates **8,738 units, 22.7 % of
the board** — and sampling shows most are unrelated: `class_feature` "Flight" is a Witch Hex *and*
an Aegis power *and* a Psychic power; "Misfortune" is an Elf Shaman Hex *and* a Guecubu monster
ability; "Outsider (Earth)" is four different favored-enemy contexts. Matching `(kind, corpus_key)`,
which carries the owning context, gives the defensible figure:

| measure | value |
|---|---:|
| objects sharing `(kind, corpus_key)` across books | **748** |
| units involved | **1,553** (4.0 %) |
| **redundant excess if only the newest survives** | **805** (2.1 %) |

Confirmed-looking samples: `equipment:bullet_firearm_pitted` (Ultimate Combat → Ultimate
Equipment) · `equipment:pathfinder_chronicle` (Inner Sea World Guide → Adventurer's Guide) ·
`race_trait:half_orc_bestial` (APG → ARG).

**2. A later VARIANT is not a reprint.** `core_rulebook` ↔ `mythic_adventures` share **95** objects,
including `feat:weapon_focus` and `feat:improved_channel`. Mythic Adventures publishes a *mythic
version* — a different object. Blind "newest wins" there deletes base Core Rulebook feats that are
unambiguously still in the game. The same hazard applies to `pathfinder_unchained` (Unchained Rage
Power vs Rage Power) and to the race scan above, where Mythic Adventures over-fires because it adds
mythic racial traits without printing the race. **Every register entry states the field-level
evidence that the two records are the SAME object, not merely the same key.**

### Sequencing

**Decision 9's re-attribution runs BEFORE the register is built.** `core_essentials` produces
phantom collisions — e.g. `monster_ability:kyton_unnerving_gaze` colliding between `bestiary` and
`core_essentials` — that dissolve once its units move to their real books. (Separately noted: the
misspelled `beastiary` directory now carries **0** units, so that potential systematic duplicate is
already gone.)

### Register shape

`artifacts/SUPERSESSION-REGISTER.md` plus a machine-readable JSON the inventory consumes:
superseded **sourcebooks**; superseded **objects**, one row each carrying `kind`, `corpus_key`, the
surviving unit id + book + `SOURCEDATE`, the superseded unit id + book + `SOURCEDATE`, the
same-object evidence, and the command that produced the pairing; and a **gate, proven able to
fail**, that refuses an entry whose two records differ materially and that recomputes the
denominator so the reduction is visible and attributable. The denominator change is reported as its
own number — `§5` defines the mandate denominator and a change to it is never incidental.

**`core_essentials` does NOT belong in this register.** It is not a superseded book; it is not a
book (Decision 9).

**Authority:** operator rulings, 2026-08-16, both verbatim above.

### Decision 10 — AMENDMENT: variant lines are new content, never supersession (operator ruling 2026-08-16)

**Ruled, verbatim:**

> "good catch with mythic and unchained.  that goes along with the previous ruling that rogue and
> unchained rogue are two completely different classes - one does not replace the other."

**This promotes the "later variant is not a reprint" guard from a cycle's caution to standing
doctrine, and it is not new — it restates a ruling this program already holds.**

**The prior ruling, located rather than paraphrased:**
`SD-28-ultimate-book-content-ingestion/decisions.md:1855-1858` records *"the operator's separate
ruling that **Unchained variants are distinct classes, not replacements**, at the data layer"* —
and records that SD-28's own measurement *independently validated* it: Summoner's
shield-ally / aspect / life-link ground **only** under `pu.unchained_summoner`, never under the base
APG class the archetype tables target. The engine already models them as two things.

### The rule, stated generally

**A book that publishes a VARIANT of an object publishes a NEW object. It never supersedes the
original, and the original is never flagged out of scope because the variant exists.** Named
instances, all in scope for this package:

| line | example | status |
|---|---|---|
| **Pathfinder Unchained** | Rogue vs **Unchained** Rogue; Summoner vs Unchained Summoner; Rage Power vs **Unchained** Rage Power | **two objects**, both in the denominator |
| **Mythic Adventures** | `feat:weapon_focus` vs its mythic version; `feat:improved_channel` | **two objects**, both in the denominator |

A reprint carries the same object forward under a newer book; a variant creates a second object that
coexists. **Decision 10's "most recent publishing wins" applies ONLY to the first.** Where the two
are confused, the failure is silent and permanent: the base Rogue, or base Weapon Focus, would be
struck from the mandate while remaining unambiguously in the game.

### What the corpus already shows, re-derived 2026-08-16

The engine does **not** currently conflate them, and a supersession cycle's risk is *creating* the
conflation rather than inheriting it:

```
python3 -c "
import json, collections
d=json.load(open('docs/work-inventory.json'))
u=[x for x in d['units'] if x.get('book')=='pathfinder_unchained']
print(len(u), collections.Counter(x.get('kind') for x in u).most_common())
"
# -> 826 units: class_feature 577, race_trait 127, monster_ability 72, equipment_modifier 42, feat 8
#    and ZERO `class` units — Unchained content is keyed `Unchained Rogue ~ ...`, distinct by key.
```

Scanning every rogue-named object for cross-book `corpus_key` collisions returns **exactly one**, and
it is not a Rogue/Unchained pair at all: `class_feature: Rogue Talent ~ Nimble Climber`
(`advanced_players_guide` ↔ `advanced_race_guide`) — a genuine reprint candidate of the kind
Decision 10 *does* govern.

### Binding on the register-building cycle

1. **Blanket-exclude the variant lines from supersession pairing.** No `pathfinder_unchained` or
   `mythic_adventures` record may be entered as either side of a supersession pair without
   record-level proof that it is a reprint and not a variant — and the default answer for those two
   books is **variant**.
2. The 95 `core_rulebook` ↔ `mythic_adventures` and the `pathfinder_unchained` ↔ `ultimate_combat`
   collisions counted in Decision 10 are therefore **presumed NOT supersessions**, and the 805
   redundant-excess figure recorded there is an **upper bound** that will fall once they are removed.
   Re-derive it after the variant lines are excluded; do not quote 805 as the outcome.
3. The same caution applies to the race scan: Mythic Adventures over-fires there because it publishes
   mythic racial traits without printing the race. A book "covers" a race for attribution purposes
   only if it prints the race, not merely content keyed to it.

**Authority:** operator ruling 2026-08-16, verbatim above, restating and generalizing the operator's
earlier Unchained-variants ruling recorded at `SD-28 decisions.md:1855-1858`.

## Decision 7 — REFINED: the real axis is universal vs conditional, not flat vs scaling (operator ruling 2026-08-16, `SD31-D7-PROSE-004`)

**Asked:** `OPEN-ISSUES.md` rows 69/87/95/107 all raised the same open interpretive question about
Decision 7's condition 2 ("nothing to compute") — does it mean (a) no numeric value appears anywhere
in the prose at all, or (b) no *character-specific SCALING* formula (the bar `wiring_class.rs`'s
`prose_scaling_phrases` detector already enforces)? At least 856 units rode on the answer across
`monster_ability`, `equipment`/`equipment_modifier`, `race_trait`, `class_feature` and `feat`.

**Ruled, verbatim:**

> "+1 size bonus to AC means you need to give a +1 on the AC - that's computed. … Now if that +4
> bonus was ONLY against certain creature types, like with dwarf racial hatred - that's not a
> universal bump and would just be listed in a description block for the player to add in when
> appropriate."

and, on damage:

> "if it says acid damage - that's a condition, many things shrug off acid."

**Neither reading (a) nor reading (b) was correct.** The operator's own framing is a THIRD axis,
orthogonal to "does a number appear" and to "does the number scale": **UNIVERSAL vs CONDITIONAL.**

- **Universal** — a modifier to a value the character sheet computes, that applies UNCONDITIONALLY
  whenever that value is read (a flat `+1` to AC is exactly as universal as a scaling one). **Must be
  COMPUTED.** Text alone does not satisfy Decision 7's condition 2 for a universal modifier.
- **Conditional / situational** — the modifier applies only against a named subset of targets,
  effects, actions, or circumstances (a creature subtype, a damage type, a specific maneuver, a
  specific stance, an environmental state, a narrative duration/resource cost). **DESCRIBE it; text
  is complete.** Flatness is irrelevant — a flat conditional bonus is exactly as done-as-text as a
  scaling one.

Condition families named by the ruling and confirmed against the corpus this cycle: **damage type**
(`special_ability_corrosive_weapon`'s `SPROP:+1d6 acid damage`) · **target subtype**
(`dwarf_hatred`'s "against humanoid creatures of the orc and goblinoid subtypes") · **manoeuvre type
and stance** (`duergar_stability`'s "against bull rush or trip attempts while standing on the
ground") · **effect type** (`half_orc_stoic`'s "against emotion and fear effects") · **environmental
state** (`fetchling_shadow_blending`'s "in dim light"; `devilfish_water_dependency`'s "out of the
water") · **zero magnitude outright** · **narrative duration** (`monk_empty_body`'s "for 1 minute").

### Sizing and outcome — `SD31-D7-PROSE-004`

Built the real discriminator, `closure_states_universal_sheet_modifier`
(`src/bin/v06_work_inventory.rs`), over the same raw `.lst` closure text every other rung in that
file already reads (`DESC:`/`SPROP:`/`BENEFIT:`), and retired the four hand-picked
`*_FLAT_MAGNITUDE_PENDING_RULING` name-lists (`monster_ability`/`companion`/`class_feature`/`feat`)
that rows 69/95/107 and wave 7 had built as conservative placeholders pending exactly this ruling.

**Validated on ACCURACY before it moved any count**, per Decision 1(e): a 32-case hand-labelled test
(30 real corpus records quoted verbatim from the pinned oracle, plus 2 edge cases; 0 misses) proved
the discriminator against the SAME units rows 69/87/95/107 and the retired lists already named.
Running the retired-list replacement against the FULL corpus (not only the hand-labelled sample)
then surfaced **5 false positives** the first draft's broader positive-cue list produced
(`advanced_race_guide:feat:guardian_of_the_wild`, `core_rulebook:feat:critical_focus`,
`advanced_race_guide:feat:orc_weapon_expertise_killer`, `ultimate_intrigue:feat:timely_coordination`,
`advanced_players_guide:feat:greater_blind_fight`) — each genuinely conditional (action-specific,
terrain-specific, or a phrase appearing inside an outright negation a substring match cannot see).
Narrowed the universal cue list to `"size bonus"` alone — the one phrase every hand-verified true
positive shares and no false positive contains — and the corpus-wide re-run came back exactly the
expected 27-unit diff, zero unexpected movement in either direction.

**Movement, both directions, every unit individually re-derived:**

| direction | count | kinds |
|---|---:|---|
| promoted (`held`/`grounded` → `done`) | **21** | `class_feature` 13, `feat` 7, `monster_ability` 1 — every unit the four retired lists had parked, all confirmed conditional |
| demoted (`done` → `held`/`grounded`) | **6** | `race_trait` 6 — `gnome_size`, `grippli_size`, `halfling_size`, `kobold_size`, `svirfneblin_size`, `goblin_size` |

Board: **9,780 → 9,795 done (25.3887% → 25.4277%)**, net **+15**. Zero denominator change.

**The 6 demotions are a correct, expected outcome of the ruling, not a regression** — the Small-race
size traits state exactly the "+1 size bonus to AC... +1 size bonus on attack rolls... penalty to
Combat Maneuver Bonus/Defense... +4 size bonus on Stealth" shape the ruling's own worked example
names as the paradigm universal case. They remain in the denominator (`race_trait`, not excluded)
and are a real, un-shipped compute gap for a future cycle's engine-wiring lane, not this cycle's to
build — Decision 1(a)'s anti-gaming bar forbids counting them `done` on text alone once identified as
universal, and Decision 8's precedent ("wire it, don't retract the finding") is what governs their
eventual close, not this decision.

### Correction to this cycle's own dispatch brief

The brief that carried this ruling to the executing cycle mis-classified `goblin_size` as one of "7
stay text" units, annotated "(grants nothing)". **That annotation is wrong.** The real shipped
`bestiary:race_trait:goblin_size` row (`data/corpus/beastiary/race_trait/goblin/goblin_size.json`,
re-attributed from `core_essentials` per Decision 9) states the IDENTICAL universal size-bonus text
as `gnome_size`/`grippli_size`/`halfling_size`/`kobold_size`/`svirfneblin_size` — verified by direct
file read before acting, per this program's own standing "verify each against its ACTUAL shipped
text" rule (the same rule this exact ruling's own history had already violated once). `goblin_size`
is therefore the 6th demotion, not a 7th text-complete unit; `retro.py correction` emitted.

**Authority:** operator ruling, 2026-08-16, verbatim above.

## Decision 12 — Public-feed PI redaction: withhold the name, keep the row (operator ruling 2026-08-17)

**Asked:** `artifacts/OPEN-ISSUES.md` rows 141 and 149. The versioned public status feed under
`site/dashboard/` publishes names their own corpus rows declare as Product Identity —
**261 unit names** in `site/dashboard/units/*.json`, and **56 class / prestige-class / feat /
spellbook identities** in the top-level `PF1e-dashboard.json`'s `manifests` roadmap content. Row 149's
exposure is pre-existing and predates this package's dashboard feature. Three options were put:
(A) withhold the name and keep the row, (B) drop the record entirely, (C) publish totals only.

**Ruled: A.**

**The rule.** A public artifact may publish that a record exists, and every derived figure about it —
counts, percentages, kind, book, status — but **never a name its own corpus row declares as Product
Identity**. The row survives with the name withheld; the counts stay honest and continue to
reconcile against the internal board.

**Why A rather than B or C.** This is already the treatment `data/corpus/` gives declared-PI records
(`license: "PI-REDACTED"`, `pi_field`), so A is the existing rule applied to a new surface rather
than a new special case. B would make the public totals disagree with the internal denominator for a
reason no reader could see; C would discard the roadmap the feed exists to show.

### Binding implementation requirements

1. **The redaction happens in the PRODUCER, not in a hand-edit.** `site/dashboard/PF1e-dashboard.json`
   is generated; a trimmed file is silently undone by the next `scripts/publish-site-dashboard.sh`
   run. `scripts/observer/pf1e_dashboard_producer.py` — including `build_unit_shards` — must apply
   the rule at generation time.
2. **The oracle is the authority, not a blacklist.** Row 141's own finding is that
   `build_unit_shards` has **no oracle cross-reference at all**. Redaction is decided by the record's
   own declared-PI state (`NAMEISPI:` / `DESCISPI:`, via `SD-30 decisions.md §53.5`'s reader), not by
   substring matching against a term list. **An exact-substring blacklist is not sufficient evidence
   of safety** — this program has already shipped deity-name typo variants straight through one
   (wave 10), and near-miss text has been the live failure mode in four of the last nine waves.
3. **A gate, proven able to fail.** A `verify.sh` stage must fail when the committed feed or any
   shard carries a declared-PI name. Mutation-prove it by seeding one, in both the top-level feed and
   a shard.
4. **`site/dashboard/units/` stays uncommitted until 1-3 are in place.** It is currently absent from
   the tree, which is why row 141 reads as fixed-for-now; committing it before the producer redacts
   would reintroduce all 261.
5. **Nothing merges to `main` until this lands.** `deploy-site.yml` publishes `site/` to Cloudflare
   Pages on every push to `main`, and `main` currently has no `site/dashboard/` directory — so the
   exposure is real but **not live**. That is the whole margin, and it closes on merge.

**Authority:** operator ruling, 2026-08-17, answering rows 141 and 149. Row 197 (Elysian Shield
cross-book declared-PI propagation) is a related but SEPARATE question and is **not** answered by
this decision.

## Decision 13 — Supersession direction CORRECTED: identical printings are owned by the FIRST print (operator ruling 2026-08-17)

**Ruled, verbatim:**

> "if they are identical - first print owns it."

**This corrects Decision 10's stated direction.** `§10` recorded the operator's earlier framing as
*"the most recent publishing takes precedence and the older one is flagged as superseded/out of
scope"*, and the orchestrating session applied that to attribution — which moved all 7 Core Rulebook
races to the Advanced Race Guide and left `core_rulebook` reporting **zero** races, the very symptom
the operator originally raised. The corrected rule resolves that without a carve-out.

### The rule, as a two-branch test

Given the same object printed in two books, **compare the two printings**:

1. **IDENTICAL → it is a duplicate → the FIRST printing owns it.** The later printing is superseded
   and out of scope. (`§10`'s denominator protection is unchanged; only the direction of the survivor
   flips — the survivor is now the OLDER book.)
2. **DIFFERENT → it is not a duplicate at all → BOTH stay**, each owned by its own book. The later
   book published something new, not a reprint.

Branch 2 is the same principle as `§10`'s AMENDMENT (*"rogue and unchained rogue are two completely
different classes - one does not replace the other"*), generalised: **the amendment was a special
case of this test all along.** Pathfinder Unchained and Mythic Adventures are simply the loudest
instances of "different, therefore both stay".

### What this settles

* **The 7 Core Rulebook races stay with the Core Rulebook.** ARG's core-race chapters add alternate
  racial traits — they are *not identical* — so branch 2 applies: the CRB owns the base race, ARG
  owns the material it added. `core_rulebook`'s race count is restored by the rule itself, not by an
  exception carved out for it.
* **Bestiary 4's 9 races**: decided per race by the same comparison, not by book. Re-derive.
* **Catfolk** (the operator's own worked example, `§10`): now decided by comparing Bestiary 3's and
  ARG's printings rather than by date. If ARG merely reprints the base traits, **Bestiary 3 owns it**
  — the opposite of `§10`'s recorded outcome. If ARG adds material, both stay. **Compare, do not
  assume.**

### THE COMPARISON IS THE WHOLE RULE, AND IT IS NOT A DATE CHECK

"Identical" is a claim about the two records' CONTENT, established field by field against the corpus,
exactly as `§10` already requires same-object evidence. A date ordering alone decides nothing under
this decision; it only breaks the tie *after* identity is proven. Publication order still comes from
the `.pcc` headers' `SOURCEDATE:` — never from memory.

### Binding consequences for the Supersession Register

* **Every existing entry's direction must be re-derived.** The register (116 objects, still
  **PROPOSED, NOT APPLIED**) was built under "newest wins". Under this decision the survivor of an
  identical pair is the OLDER printing, so each entry's surviving/superseded sides swap — and any
  pair whose two records are NOT identical **leaves the register entirely**, because it was never a
  duplicate.
* Expect the register to SHRINK. `§10`'s already-reduced excess figure is an upper bound again.
* The register stays PROPOSED until it is re-derived under this rule. **Nothing has left the
  denominator and nothing may until then.**
* Race attribution stays FROZEN until the re-derivation lands, then unfreezes under this rule.

**Authority:** operator ruling, 2026-08-17, verbatim above. Supersedes `§10`'s direction; `§10`'s
guards (a shared NAME is not a duplicate; match on `(kind, corpus_key)`; a later VARIANT is not a
reprint) all stand unchanged and are reinforced by branch 2.

### Decision 13 — AMENDMENT: "not identical" splits into two cases (operator ruling 2026-08-17)

**Ruled, verbatim:**

> "with race - if they are not identical - there are one of two options - a sub/alt race or a rules
> update where the newest wins. devil is in the details. without examples of what you found, it's
> hard for me to give a ruling - but just to put it out there, the core rules might have a dwarf.
> then the arg maybe has something like a grey dwarf. those are two different things and deserve
> their own records. if it's just a dwarf and it says they can see 60 feet in the dark, and the later
> book says 90 feet - go with 90 feet. savvy?"

**Decision 13's branch 2 was too coarse.** "Different, therefore both stay" is right for one of the
two shapes and wrong for the other. The complete test is **three branches**:

| | the two printings are… | outcome |
|---|---|---|
| **1** | **IDENTICAL** | duplicate — **the FIRST printing owns it**, the later is superseded |
| **2** | **A DIFFERENT THING** — a sub-race, alt-race, or otherwise distinct entity (Dwarf vs **Grey Dwarf**) | not a duplicate — **BOTH stay, each with its own record** |
| **3** | **THE SAME THING WITH CHANGED VALUES** — a rules update or errata (Dwarf darkvision **60 ft → 90 ft**) | one thing — **the NEWEST value wins** |

Branch 3 is where `§10`'s original "newest wins" was genuinely right, and it is narrower than that
framing implied: it applies to **the value**, on the same object, not to ownership of a distinct
record.

**Distinguishing 2 from 3 is a content judgement, not a name check.** A different NAME strongly
suggests branch 2 (Grey Dwarf is not Dwarf), but the reverse does not hold — the same name with
changed numbers is branch 3, and the same name with genuinely new sub-entries may be branch 2. The
`§10` guard stands: **a shared name never implies a shared thing, and a different name never by
itself proves a different thing.** Compare the records.

**Pathfinder Unchained and Mythic Adventures remain branch 2** — Unchained Rogue is a different class,
not a re-statement of Rogue's values, per `§10`'s amendment and the operator's own earlier ruling.

### THE OPERATOR HAS NOT RULED ON THE RACE CASES, AND IS WAITING ON EVIDENCE

> *"devil is in the details. without examples of what you found, it's hard for me to give a ruling"*

**This is owed work, and it is a dispatchable card.** No race attribution changes until it lands.
The deliverable is a per-race **worked-example** table — for each race printed in more than one book,
the actual differing fields side by side (which book, which value, which line), so the operator can
classify each as branch 1, 2 or 3 by reading real data rather than a summary. The orchestrating
session has already handed the operator two wrong race figures; this table is the correction for
that pattern, not another summary.

Race attribution stays **FROZEN**, and the Supersession Register stays **PROPOSED, NOT APPLIED**,
until the evidence table exists and the operator rules from it.

**Authority:** operator ruling, 2026-08-17, verbatim above.

## Decision 14 (PROPOSED — awaiting operator) — Provenance status: one fixed classification per (object, book)

**Operator direction, verbatim:**

> "i think each object per book needs a fixed set of status. Origin, Superceded, Duplicate,
> Descoped-Licensing, perhaps a few more. We should get a well defined set of rules to classify
> everything"

**This is a NEW AXIS, and keeping it separate from the existing one is the whole safety property.**
The board already has a *doneness* axis (`wiring_class` × `status` → `not-started` / `held` / `done`,
`§7`). Provenance answers a different question: **does this (object, book) pair belong in the mandate
denominator at all, and which book owns it?** Conflating the two would let a provenance edit silently
move the `done` percentage, which is precisely the Decision 1(a) violation this package forbids.
**Provenance decides denominator MEMBERSHIP. Doneness measures progress WITHIN it. Neither may be
derived from the other.**

Recorded as **PROPOSED**. It supersedes nothing until the operator confirms the set.

### The proposed status set

Every `(object, book)` pair gets exactly one:

| status | meaning | in denominator? |
|---|---|---|
| **`origin`** | this book is where the object is defined; the owning printing | **YES** |
| **`duplicate`** | an identical reprint in a later book (`§13` branch 1) | no |
| **`superseded`** | the same object's values were changed by a later printing (`§13` branch 3) — this pair holds the outdated values | no |
| **`errata-source`** | the later printing whose changed values win under `§13` branch 3 | *see OPEN QUESTION* |
| **`variant`** | a distinct derived object sharing a lineage — sub-race, Unchained, Mythic (`§13` branch 2, `§10` amendment). **Its own record; it is an `origin` in its own right and this status records only the lineage.** | **YES** |
| **`descoped-licensing`** | cannot be shipped for licensing / declared-PI reasons | no |
| **`descoped-structural`** | the `§3` Structural Exclusion Register — finishing is genuinely impossible, operator-signed | no |
| **`packaging-artifact`** | the pair names a PCGen packaging directory, not a real book (`§9`, `core_essentials`) — **must be re-attributed, never left** | no (and must reach zero) |
| **`out-of-roster`** | resolves to a real book outside the 37-book mandate roster (`Ironfang Invasion`, `Blood of the Moon`, `Universal Rules` appeared in `§9`'s census) — **a scope question, not a deletion** | no, pending operator |

### Invariants a gate must enforce, each proven able to fail

1. **Totality.** Every `(object, book)` pair carries exactly one status. No default, no absent value.
2. **Exactly one `origin` per object.** Zero origins means the object is unowned; two means the
   supersession test was never applied. Both are gate failures.
3. **`denominator = origin + variant`**, and nothing else. The published denominator is derived from
   provenance, never hand-maintained.
4. **`packaging-artifact` must trend to zero** and is a hard failure once `§9`'s re-attribution
   completes. It is a transitional state, not a resting place.
5. **`descoped-structural` requires an operator signature** per `§3`; a cycle may only propose.
   `descoped-licensing` does not require a signature but requires the declared-PI evidence.
6. **A provenance change must NEVER change a unit's doneness fields**, and vice versa. Assert it: a
   provenance-only commit shows zero `doneness_verdict` movement for units that remain in the
   denominator.
7. **Any denominator change is reported as its own number** with the count per status — `§5` defines
   the denominator and a change to it is never incidental.

### OPEN QUESTION — the one thing the operator must settle

**Under `§13` branch 3 (same object, values updated by a later book — darkvision 60 ft → 90 ft),
which book is `origin`?**

* **(a) `origin` stays with the FIRST printing**, and the later book is `errata-source` supplying the
  winning values. Keeps "origin = where it was defined", and a book never loses its content to a later
  errata. The object's live values then come from two pairs, so the value-resolution order must be
  explicit.
* **(b) `origin` MOVES to the book holding the current authoritative values**, and the earlier pair
  becomes `superseded`. One pair is the single source of truth for the object, which is simpler to
  compute — but the Core Rulebook would stop being the origin of anything later errata touched, which
  is the shape that produced the `core_rulebook` = 0 races complaint in the first place.

**Recommendation: (a).** It matches the operator's own framing — *"first print owns it"* was stated
about ownership, while *"go with 90 feet"* was stated about the value. Separating ownership from
value-resolution honours both sentences without a carve-out, and it keeps `errata-source` from
hollowing out early books.

### Not yet decided by this proposal

The per-race branch-1/2/3 classification itself. `§13`'s amendment records that the operator is
waiting on a worked-example evidence table before ruling on the race cases; **this schema is the
vocabulary for that ruling, not a substitute for it.**

**Status:** PROPOSED 2026-08-17 from operator direction. Race attribution stays FROZEN and the
Supersession Register stays PROPOSED, NOT APPLIED, until this set is confirmed and the race evidence
table is ruled on.

### Decision 14 — CONFIRMED, with the origin-flip mechanics (operator ruling 2026-08-17)

**Ruled, verbatim:**

> "in that case, origin becomes superceded and the later book becomes errata-source. Origin is the
> original publish, all identical values become duplicate. If a later publication comes out with
> corrected values, origin flips to superseded."

**The status set in Decision 14 is CONFIRMED.** The open question is answered — **neither (a) nor
(b) as I framed them**, but a cleaner third mechanic:

### The lifecycle of a pair

1. **`origin`** — the original publish. Assigned on first printing, always.
2. A later book reprinting **identical values** → that later pair is **`duplicate`**. The origin pair
   is untouched and stays `origin`.
3. A later book publishing **corrected values** → that later pair becomes **`errata-source`**, **and
   the origin pair FLIPS to `superseded`.**

So `origin` is not permanent. It is the status of the original publish *until corrected*, at which
point ownership of the live record moves to the errata.

**With successive errata (A → B → C), only the LATEST is `errata-source`;** every earlier pair,
including the original publish, is `superseded`. Exactly one `errata-source` per object, or none.

### FORCED CORRECTION to Decision 14's invariants 2 and 3

The proposal said *"exactly one `origin` per object"* and *"denominator = origin + variant"*. **Both
break under this ruling**: an object with errata has NO `origin` pair at all — it flipped — so it
would fall out of the denominator entirely and silently vanish from the 100 % mandate. That is
plainly not the intent (*"i dont want duplicates falsely adding to the denominator"* is about
accuracy, not disappearance), and it is exactly the class of silent-shrinkage defect this package
exists to prevent. The invariants are therefore restated:

* **Invariant 2 (restated): exactly one AUTHORITATIVE pair per object**, being its `origin` if no
  errata exists, or its single `errata-source` if one does. Zero authoritative pairs means the object
  is unowned; two means the comparison was never applied. Both are gate failures.
* **Invariant 3 (restated): `denominator = authoritative + variant`**, i.e.
  `origin + errata-source + variant`. Still derived, never hand-maintained.

The remaining invariants (totality; `packaging-artifact` trending to zero; `descoped-structural`
needing an operator signature; a provenance change moving zero doneness fields; any denominator
change reported as its own number) stand unchanged.

### OWED DETAIL — flagged, not assumed

**Does an `errata-source` replace the WHOLE record, or only the fields it corrects?** Real errata
often restates a single value (darkvision 60 → 90) while the rest of the record stays as first
printed. Under a whole-record reading, everything the errata omits would be lost; under a
field-level reading, the live record is the original overlaid with the errata's changed fields.
**Field-level overlay is the only reading that does not destroy data**, so cycles will implement
that unless the operator says otherwise — recorded here rather than decided silently, and it should
be re-confirmed against the first real worked example the race evidence table produces.

**Authority:** operator ruling, 2026-08-17, verbatim above. Decision 14 moves from PROPOSED to
CONFIRMED with these mechanics. Race attribution stays FROZEN and the Supersession Register stays
PROPOSED, NOT APPLIED, until the race evidence table exists and is ruled on.

## Decision 15 — The 13 `.COPY=` spell variants: exclusion WITHDRAWN, a real path exists (2026-08-17)

**Operator direction:** *"As for the spells, they stay - i'll look them up for you. i need a list"*,
then, after the list: *"i understand now why you had problems with those 13 spells. See if you can
find the path back on those to the parent spell that they copy"*.

**The path exists, it is short, and the exclusion proposal (`OPEN-ISSUES` row 55) is WITHDRAWN.**
No signature is needed and no unit leaves the denominator.

### The parent is named in the row itself

PCGen's `.COPY=` syntax is `<parent>.COPY=<new name>`, so the parent is the text **before** `.COPY=`:

```
Speak with Animals.COPY=Speak with Animals (rodents only)   CLASSES:.CLEARALL
^^^^^^^^^^^^^^^^^^ the parent
```

All 13 parents were located in the same file as their copy, each carrying real class levels
(re-derived 2026-08-17 against the pinned oracle):

| copy | parent | parent's levels |
|---|---|---|
| Animate Objects (Small or Smaller) | Animate Objects | Bard, Cleric = 6 |
| Charm Animal (aquatic animals only) | Charm Animal | Druid, Ranger = 1 |
| Disguise Self (humanoid only) | Disguise Self | Bard, Sorcerer, Wizard = 1 |
| Nondetection (self only) | Nondetection | Ranger, Sorcerer, Wizard = 3 |
| Plane Shift (×2 variants) | Plane Shift | Cleric = 5 \| Sorcerer, Wizard = 7 |
| Speak with Animals (×4 variants) | Speak with Animals | Druid, Ranger = 1 \| Bard = 3 |
| Summon Monster III (lantern archon only) | Summon Monster III | Bard, Cleric, Sorcerer, Wizard = 3 |
| Summon Nature's Ally I (dolphins only) | Summon Nature's Ally I | Druid, Ranger = 1 |
| Fins to Feet (self only) | Fins to Feet | Druid, Sorcerer, Witch, Wizard = 3 |

### `CLASSES:.CLEARALL` is a correct statement, not missing data

Row 55 read the cleared class list as "PCGen declaring this variant has no class, therefore no level
exists, therefore ingest is impossible". **That reading was wrong.** Every one of the 13 is
referenced from a `*_abilities_race.lst` file: these are **racial spell-like abilities**, and the
class list is cleared precisely because *no class grants them* — a race does. Verbatim from the
corpus:

```
KEY:Wererat-Kin ~ Spell-Like Ability
DESC:A nightskulk skinwalker with a Wisdom score of 11 or higher can use speak with animals
     (rodents only) three times per day as a spell-like ability, using his Wisdom modifier to
     determine his concentration checks. The caster level for this ability is equal to the
     skinwalker's character level.
```

Others confirmed the same way: Fetchling grants *Disguise Self (humanoid only)* and *Plane Shift (to
Shadow or Material Plane)*; Inner Sea Races grants *Fins to Feet (self only)* and *Summon Nature's
Ally I (dolphins only)*; Skinwalker/Bestiary 4 races grant the *Speak with Animals* variants.

### The ingest shape

* **Spell properties** (school, range, duration, description) — inherit from the parent record.
* **Spell level** — the parent's. It is real and present.
* **Caster level** — from the granting racial ability, which states it in plain text ("equal to the
  skinwalker's character level").
* **The consumer** — the race ability that grants it, a genuine player-visible surface, so DoD-8 is
  achievable rather than blocked.

This is ordinary ingest work under `epic-6-ingest-lanes`, not an exclusion. Any cycle claiming it
must resolve the parent from the row rather than hardcoding a mapping, and must NOT invent a caster
level where the granting ability does not state one.

### The lesson, which is this package's most-repeated one

Row 55 did everything the process asks — it re-derived its figure, read the record one level deep,
and wrote all four `§3` items honestly. It still reached the wrong conclusion, because it read the
copy and never followed `.COPY=` back to the parent. **The data was complete; the reader stopped one
hop short.** Same shape as `core_essentials` (`§9`), the corpus-row path join (wave 2), and the pool
name matcher (wave 11). `retro.py correction` emitted against row 55's "impossible" claim.

**Authority:** operator direction 2026-08-17; finding derived and verified by the orchestrating
session against the pinned oracle `7f818006e371188e5717fd18d74d18a420747fc6`.
