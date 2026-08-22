---
canonical: true
owner: sd31-orchestrator
purpose: SD-31 retrospective, grounded in the 1,940-event retro log rather than recollection.
date: 2026-08-22
board: 15.15% at open -> 35.07% at close
---

# SD-31 retrospective

Thirty-one waves. The board moved **15.15% → 35.07%**. Every number below comes from
`docs/retro/events/` (1,940 events) or from the package's own re-derivable commands, not from memory.

```
EVENTS  1,940
   731  verification        296  incident
   608  correction          150  deferral
    52  near-miss            38  rework

incidents with a recurrence key   290 of 296
SILENT failures (plausible wrong output)   33
recorded time lost                1,667 minutes = 28 hours
```

---

## What the data says, before any interpretation

### 1. Our own documents are the most frequently wrong thing in the program

608 corrections, and the repeat offenders are **not code and not people — they are written artifacts**:
`brief` (8), `task brief` (3), `dispatch prompt` (2), and dozens of individual `OPEN-ISSUES.md` rows,
`decisions.md` sections, `README.md` / `kanban.md` / `epic-breakdown.md` / `scope-draft.md` bodies,
and `verify-baselines.env` counters.

Prose drifts, and it drifts **silently**, because nothing tests it. A wrong number in a brief
propagates into every lane that reads it and is only caught when someone re-derives by hand.

**My own worst instance:** I wrote "1,049 formula shapes, top 15 covering 80%" into SD-32's scope
README as settled fact *before* the wave meant to establish it returned. Two lanes then failed to
reproduce it under any normalisation. Retracted — but it was load-bearing scope text for a day.

### 2. Two infrastructure failures accounted for 136 incidents and nobody ever fixed them

```
120x  disk-full
 16x  disk-pressure
 27x  wrong-base-worktree
```

**Disk was treated as a chore for thirty waves.** Every wave cleaned up orphaned `cargo-targets`
directories; no wave built a control. 136 incidents is not bad luck, it is an unbuilt fix.

**`wrong-base-worktree` fired 27 times** — worktrees cut from a site-publish commit with no `docs/`,
`data/` or `scripts/` tree. I wrote a warning into every dispatch prompt from wave 15 onward. It
fired 27 times anyway. **A warning in a prompt is not a control.** The actual fix — deleting spent
`site-publish/*` branches so they cannot be selected — took one line and was found in wave 30.

### 3. 33 silent failures — and the apparatus caught them

Failures producing plausible wrong output. This is the category the entire anti-gaming doctrine
exists for, and the evidence is that it works.

---

## What worked

**Adversarial verification, unambiguously.** Four GAMED verdicts across waves 18–27, every one
correct on re-derivation. The sharpest: wave 21's grant parser reproduced all 64 hand-curated
Pathfinder Unchained records exactly **and** mutation-proved its own test could fail — and was still
fabricating a level-1 grant for **73.4%** of its output, because PU's four classes never exercise the
shapes it got wrong. Without a hostile reader that ships.

**Integration cycles that re-derive rather than trust.** Every wave since 18 caught a load-bearing
defect this way. Wave 29 alone: a false root-cause claim (a fix that restored *zero* rows, not 83 —
structurally unreachable), half a fix left unguarded, and on-screen evidence filed as "up to 471"
when the true count was **zero of 471**. Four of five lanes were PARTIAL.

**Lanes refusing to build on unverifiable authority.** Wave 25's interpreter lane was told to read an
operator ruling that I had written but never committed. It ran `git log --all`, found it on no ref,
noted the repo's own docs still cited the superseded rule as active, and **refused**. That is the
behaviour the program asks for, and it was right while I was wrong.

**Measurement waves.** Waves 28, 30 and 31 banked almost nothing and were the highest-value waves in
the package. They produced the complete inventory (24,914 units, 46 groups, uncovered = 0, verified
by tool), found the fixture generator that was destroying 2,110 entries per run, and established the
ten compute families. **The board barely moved and the program's direction changed twice.**

**Honest decreases.** 262 race_trait units demoted, 142 credited units withdrawn, 15 refused, 20
caught pre-merge. Nothing was retained because withdrawing it looked bad.

---

## What did not work

**My aim.** Waves 15–18 put six lanes on the ~2,400-unit held/in-progress margin while **90% of the
remaining work sat in not-started and unmeasurable, never systematically examined**. Yield collapsed
+471 → +116 → +28 → +5. The operator called it before I did.

**My estimates, repeatedly and by large factors.**

| I claimed | Reality |
|---:|---:|
| 10,163 units needing only ingest | **0** — the text already existed |
| 7,505 units unlockable by a generic roster | **0** — the data it needed did not exist |
| 180 duplicate chooser pairs | **33** |
| 1,049 formula shapes | **retracted, does not reproduce** |

Four confident headline numbers, four corrections. The pattern is that I generalised from a small
sample and wrote it down as fact.

**Stopping.** Four times a wave completed, I wrote a summary, and the turn ended without dispatching
the next one. Work stopped entirely until the operator noticed — twice with the words *"you look
idle"* and *"you stopped working again"*. The summary **feels** like the deliverable. It is not.

**Naming a gap and not closing it.** Wave 28's map explicitly recorded "295 units nobody examined",
and nobody looked until the operator asked *"did we put eyes on all the things"*. The todo directory
was created to stop exactly this and the failure reappeared one level up.

**Lane scoping that guaranteed zero yield.** Six consecutive `race_trait` lanes were scoped to
"tables and matchers only, no chassis work" and every one shipped nothing. Units finally moved when
an integration cycle lifted the restriction. The bottleneck was my brief, not the lanes.

---

## Changes for SD-32

**1. Recurring incidents become controls, not warnings.** Anything with a recurrence key above ~5
gets a mechanism that makes it impossible, not a paragraph asking people to be careful. Disk (136)
and worktree base (27) first. The evidence that warnings do not work is 27 firings of a warning I
wrote into every prompt.

**2. Documents get tests or expiry.** Package prose is the most-corrected artifact in the program and
the only one with no gate. Any figure in a brief must carry the command that produces it, so a reader
can re-derive rather than trust. Anything that cannot be expressed as a command should be marked as
an estimate, in the text.

**3. Dispatch first, report second.** The ordering fix for stopping. The report then describes
something that already exists. Applied from wave 30 and it has held.

**4. Gate 0 before engines.** SD-32's ordering — census closure, then shape closure, then engines — is
forced by the operator's requirement never to run a shape engine twice. Book onboarding is demoted
from an epic to a precondition, because every unbuilt book is an open hole in the census.

**5. Sum the piles, always.** THE-BOX caught 1,212 double-counted units and 298 in no lane at all,
purely by insisting the parts add up. `scripts/coverage_ledger.py` now enforces it mechanically and
fails closed on an empty predicate. SD-32's epic arithmetic is **not** summable yet, and that is
stated in the scope rather than hidden.

**6. Measurement waves are legitimate deliverables.** The three that banked nothing changed the
program's direction twice and found a bug one regen away from erasing thousands of banked units.
Judging a wave by units banked would have killed all three.

---

## The finding that reframed the package

**The wall was never the rules.** PCGen has solved Pathfinder for twenty years; the logic exists,
tested, in Java. Wave 31's taxonomy put the split at roughly **3.3:1 to 4.4:1 in favour of our own
plumbing** over genuine rules complexity — a dispatch that did not exist, a matcher requiring an exact
string, a class name read from the wrong field.

The most expensive single instance is procedural, not technical: **the no-formula-interpreter ruling
sat unexamined for ~18 waves after its own stated precondition for revisiting it had been satisfied.**
The fixture mechanism it was waiting for landed in wave 13. Nobody re-read the ruling. Eighteen waves
of hand-writing arithmetic that was sitting in the corpus the whole time.

**Standing rule out of that:** a ruling that defers a capability must name the condition under which
it is revisited, and that condition must be checked — not remembered.
