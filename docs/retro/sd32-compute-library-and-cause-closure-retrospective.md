---
canonical: true
owner: closure-epilogue
purpose: SD-32 retrospective, grounded in `scripts/retro.py summary --since 2026-08-22 --json`
  rather than recollection.
date: 2026-08-22
board: four gates (G0/G1/G2/G3) closed to Definition of Done; Epic 1-5 cards 1-10,12 complete,
  card 11 filed under Open blockers with a named owner
---

# SD-32 retrospective

Twelve dispatched cycles plus this closure epilogue. Every number below is re-derivable —
`python3 scripts/retro.py summary --since 2026-08-22 --json` for the retro-log figures, or the
cited command for anything else.

```
EVENTS (whole 2026-08-22 window, includes SD-31's own tail)   60
   21  verification         16  correction
    9  incident              7  deferral
    6  note                  1  rework

Genuine SD-32 dispatched-cycle events (excludes sd31-orchestrator/sd31-transcribe carryover
and codex's own reclaim.sh housekeeping)                       27
verification fail rate                                         5/21 = 23.8% (all preflight-oracle,
                                                                 all self-healed per §8 — fresh
                                                                 worktree, empty oracle slot)
incident recurrence keys firing more than once                 disk-full (3)
```

---

## What the data says, before any interpretation

### 1. Four gates closed clean — every one met on its own written criteria, not on a schedule

`decisions.md §2` reaffirmed the closure trigger was the Definition of Done — all four gates'
AT-32-* criteria met — never a wave count. That held: Gate 0 (`gate-0-census-closure/001,002
_cycle_receipt.md`), Gate 1 (`gate-1-shape-closure/001_cycle_receipt.md`), Gate 2
(`gate-2-engines/001,007,008,008-f1f9_cycle_receipt.md`, four cycles for two engines), Gate 3
(`gate-3-closure-invariant/001_cycle_receipt.md`) each closed on their own AT-32-G*-* text, and
each receipt names what it does **not** claim as clearly as what it does (Gate 2's card 6 receipt:
"AT-32-G2-004 explicitly NOT claimed — card 8's own criterion").

### 2. The dispatch-ordering gap Gate 3 found was real, and the log shows it was judged, not skipped

Card 9 (`gate-3-invariant`) was dispatched while Gate 2's cards 6-8 locally still read `pending`.
Rather than blocking or silently proceeding, that cycle re-read `acceptance-and-verification.md`
Gate 3's own text, confirmed it depends on Gate 1's `shape_ledger.py` output only (not on any Gate
2 engine or fixture-check), and logged the finding as a `scripts/retro.py correction`, not a
blocker — the kind of judgment call the anti-gaming doctrine asks lanes to make explicitly rather
than by default. Gate 2 had in fact already landed on origin by the time Gate 3's own push rebased,
so the finding's practical stakes were moot, but the reasoning stands as written, made before
hindsight was available.

### 3. Two population-figure corrections moved the census's own denominators

Gate 0's own cycle corrected two numbers baked into the bundle's planning prose before it ran a
single line of code: the "158-book" oracle claim (`scope-draft.md`, `acceptance-and-verification.md`
AT-32-G0-001) was **186** books by `scripts/census_independent.py`'s own reproducible definition,
and the "38,372 units" denominator (`acceptance-and-verification.md` AT-32-G0-002) was **38,391**
by `jq '.total_units' docs/work-inventory.json` at Gate 0's cycle time. Neither figure had a
derivation command anywhere in the bundle before this cycle. Both are logged as
`scripts/retro.py correction`s, not silently reconciled.

### 4. The Epic 5 sweep found more live vulnerabilities than its own predecessor's count

Epic 5's own receipt (`epic-5-protective-sweep/cycle-1_cycle_receipt.md`) corrected
`epic-breakdown.md`'s own D9 framing — "17 of 29 generators never checked" was treated as an
unmeasured residual with no known findings; the actual count was **5 of those 17 genuinely
vulnerable** to the same self-erasure shape `scripts/derive_derived_evaluator_fixtures.py` had
(protecting up to ~3,100 enriched records: 413 ACG + 622 APG + 3 Bestiary + 712 spell-lane + 1,368
Ultimate Equipment). This is the sharpest instance of AGENTS.md rule 8 ("a warning is not a
control") working as intended: the sweep was a mechanism, not a caution, and it found what the
caution alone would have missed.

### 5. Card 11 (Epic 2) is the one card this bundle closes honestly incomplete

Every other Epic 1-5 card reached `complete`. Card 11 closed T1 corpus-wide (the "dispatch gap /
Monk shape", across classes/races/monsters, with a new standing test — `reach_gate::tests::
dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms`) and correctly cited T5/T3
rather than re-closing them, but named T2a (8,243 units), T2b (2,472), T9 (2,651), T4 (up to
2,763), T12 (~3,000, overlapping T2a by 1,354-2,124 and un-closeable independently of it), T7 (4,
fix site identified, not implemented) and T8 (12, blocked on an operator ruling over a read-only
SD-30 surface) as **not attempted** — each a multi-thousand-unit population that on Gate 2's own
precedent (three cycles for a *narrower* ten-family scope) needs its own measurement-and-close
cycle. This closure epilogue filed that state formally: `kanban.md` card 11 →
`returned-to-backlog`, `progress.md` "## Open blockers", `forward-scope-register.md` C2.5 (named
owner: a successor SD-N bundle). `acceptance-and-verification.md` AT-32-CLOSE-001 explicitly
allows this path ("complete or filed under Open blockers with a named owner") — this is that path
exercised, not a scope failure hidden as a success.

---

## What worked

**File-disjointness held.** `workflow-instruction.md §3`/§4 named the exact surface each phase
would touch before dispatch; the actual commits confirm it (Gate 2's two engine chains ran in
parallel worktrees on genuinely disjoint files — `formula_interpreter.rs` vs
`bonus_stack_reader.rs` — with zero cross-cycle conflicts across four Gate 2 cycles).

**The mechanical base-pin check (§6 step 1) caught what a prose warning alone could not.** SD-31's
identical warning fired 27 times against six waves. In SD-32, `epic-3-class-reachability`'s own
cycle hit `wrong-base-worktree` once (`worktree-base` correction: claimed cut from `tranche/12`'s
tip, actually cut from a site-publish merge commit missing `docs/`/`data/`/`scripts/`), and the
mechanical `git merge-base --is-ancestor $PIN HEAD` check caught it immediately, before any real
work was attempted on the wrong tree — self-healed via `git reset --hard $PIN`, one incident, not
27.

**"Dispatch first, report second" held for the whole bundle.** Zero `report-feels-like-the-
deliverable` incidents are attributed to any SD-32 dispatched cycle in this window (the one
instance in the log is `sd31-orchestrator`'s own carryover entry from before SD-31 closed, not a
new SD-32 occurrence).

**Fixture discipline (Decision 3 / operator ruling §20) held under real pressure.** Every engine
cycle that could have claimed a plausible number instead built the narrower, real proof: Gate 2's
`formula_interpreter` corpus-wide run (card 8, F1-F9 leg) explicitly refused to fabricate 4,798
`vars` maps to claim a numeric-value proof, and built a population-scoped grammar-reach proof
instead (4,696 of 4,798 recognised, 102 refused, all within the engine's own already-disclosed
proof-width gaps) — the same discipline AGENTS.md rule 7 names by its worst historical instance
(73.4% fabrication behind a passing reference-set proof).

**The self-test-the-gate discipline (Decision 1a) was applied literally, not just cited.** Gate 3's
own cycle found `classify_unit()` structurally cannot organically produce an "uncovered" row (it
always falls through to F0/F8), so it proved the gate's own failure mechanism with a *fabricated*
uncovered row and pile mismatch rather than declaring the gate untestable — "a gate that cannot
fail is worse than no gate" turned into a concrete test, not left as doctrine.

---

## What did not work

**Card 11 could not be closed in one cycle, and the bundle's own dispatch script did not scope for
that in advance.** `workflow-instruction.md §2.4`'s pipeline dispatches card 11 once, in parallel
with cards 10 and 12, with no chained follow-up cycle the way Gate 2's card 8 chains behind cards
6/7. The eight blocker shapes were measured (in `epic-breakdown.md`, at planning time) but the
dispatch script did not carry that measurement into a multi-cycle plan the way it did for Gate 2's
two engines. Consequence: this closure epilogue had to do at dispatch-boundary work (filing card 11
under Open blockers) that a `card(11) -> card(11b) -> card(11c)...` chain, sized to the Gate-2
precedent, would have made routine instead of a closure-time finding.

**Two figures in the bundle's own planning prose were wrong before a single cycle ran** (the
158-book / 38,372-unit claims above). Both were caught by the very first Gate 0 cycle, which is the
system working — but they were written into `acceptance-and-verification.md`, a canonical,
committed file, not a draft, which is exactly the "our own documents are the most frequently wrong
artifact" pattern SD-31's own retrospective named as its #1 finding. It recurred one bundle later,
in the successor bundle's own launch-ready acceptance criteria.

**The oracle preflight failed 5 of 21 times (23.8%)**, entirely from fresh-worktree empty slots
self-healing via `scripts/fetch-pcgen-oracle.sh` — expected and self-healable per §8, but every one
of those five re-fetches (86MB sparse cone) is dead time a persistent worktree pool would avoid.
Not fixed this bundle; named here because five recurrences of the identical self-heal is worth a
mechanism if a successor bundle runs at higher worktree-turnover.

---

## Changes for a successor bundle

**1. Size a card's dispatch chain to its own measured population, at authoring time, not at
closure time.** Gate 2 got a `card(N) -> card(N+1)` chain because its scope was measured (ten
families) before dispatch. Card 11's eight blocker shapes were *also* measured before dispatch
(`epic-breakdown.md`'s own T1-T12 unit counts) but the dispatch script treated it as one
parallel-fan-out card anyway. The fix is mechanical: any card whose own acceptance criterion names
more than one multi-thousand-unit population gets a chain, sized to that population count, in
`workflow-instruction.md §2.4` at authoring time.

**2. AT-32-CLOSE-001's "complete or filed under Open blockers" clause is the right shape and
should be a template-canonical closure criterion, not bundle-specific text.** It let this bundle
close honestly on the four gates it actually built while carrying forward exactly what it did not,
with a named owner, instead of forcing either an artificial "complete" claim or an indefinite
stall. Worth promoting into `../../governance/workflow-instruction-template.md`'s own closure
section for every future bundle.

**3. Run the placeholder-resolution / figure-derivation checklist (§10-style) against
`acceptance-and-verification.md` specifically, before it is marked canonical — not just against
the whole bundle at chassis time.** Both of this bundle's own pre-launch figure corrections
(158-book, 38,372-unit) were in that one file, and both were wrong at the same remediation pass
that resolved every other placeholder in the bundle. A file-scoped, not just glob-scoped, pass
would likely have caught them before Gate 0 had to.

---

## The finding that carries forward unchanged

**A ruling is not in force until it is committed, and this bundle proved the corollary too: a
measured scope is not a schedule until it is chained.** SD-31's retrospective closed on "the
no-formula-interpreter ruling sat unexamined for ~18 waves after its own precondition was met."
SD-32's own version of the same shape is smaller (one card, not eighteen waves) but structurally
identical: the eight blocker shapes' measured sizes existed in `epic-breakdown.md` from the
bundle's own chassis-completion day, and the dispatch script still fanned card 11 out flat. The
data was there. Nobody re-read it as a chain requirement before dispatch. Naming the condition is
necessary but not sufficient — the condition has to be checked against the dispatch shape itself,
not just against whether work can start.
