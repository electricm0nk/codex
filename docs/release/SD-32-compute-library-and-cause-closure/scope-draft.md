---
canonical: true
owner: god-emporer
status: scoped 2026-08-22 — definition of done agreed with the operator, not wave-budgeted
predecessor: SD-31-corpus-closure-grind
---

# SD-32 scope and definition of done

**SD-32 is scoped to reach a definition of done, not to a wave budget.** The operator was explicit:
measurement waves are acceptable, and the board may barely move for several of them. That cost is
accepted deliberately in exchange for never running the same shape engine twice.

---

## The principle this package is built on

The operator's own framing, and it is the constraint everything below serves:

> *"What I do not want to do, is run a shape against an engine, and then find new objects requiring
> us to run that same shape engine again."*

That is **program-level idempotence**, and it is the same discipline SD-31 applied to generators one
level up. It creates a hard ordering, not a preference:

```
census closure   →   shape closure   →   engines
```

An engine's correctness claim is *"this shape is now handled."* That claim is false if the shape's
population can still grow. So engines cannot run until the population is closed.

**This demotes book onboarding from an epic to a precondition.** Every unbuilt book is an open hole
in the census, and running engines with holes open guarantees the rerun the operator does not want.

---

## The gap that makes Gate 0 necessary

SD-31 wave 30 proved **every not-done unit is classified** — 24,914 units, 46 groups, uncovered = 0,
verified by `scripts/coverage_ledger.py` rather than hand arithmetic.

**That is not the same claim as "every object in every book is a unit."** THE-BOX proves the second
half of the census; nothing proves the first.

The denominator, 38,372, is derived from what the walker enumerated. Freezing it was a governance
decision, not evidence of completeness — and there is precedent for it being wrong. **SD-31 wave 1
found a single-level directory join that missed every nested `.lst` file, stranding ~1,707 real
units in in-scope books**, invisible to the census until someone tripped over them.

The inventory holds **37 books**. The pinned oracle holds **158 book directories**. Most of that gap
is deliberate roster scope — but nobody has audited which part is scope and which part is oversight.

---

## Definition of done — four gates

### Gate 0 — Census closure

**Every object in every in-scope book is enumerated as a unit.**

Proven by a tool that walks the pinned oracle **independently of the existing walker** and diffs. An
independent walk is the point: a census that reuses the enumerator it is auditing cannot detect the
enumerator's blind spots, which is exactly how wave 1's 1,707 units hid.

Deliverables:
* the census tool (see *Tooling* below), with the 37-book roster audited against the oracle's 158
  directories — every excluded directory named and justified as scope, not oversight;
* a diff of oracle objects against inventory units, per book, per kind, reaching zero-unexplained;
* **an honest definition of "object" per kind.** This is not a formality. For PCGen `.lst` data an
  object is a row, but `.MOD` continuation lines, `.COPY=` derivations and template rows each need a
  stated rule. Gate 0 cannot be declared without that definition written down.

**Gate 0 may not be fully achievable, and that must be discovered rather than assumed.** Some books
may hold objects PCGen itself does not model uniformly. If a category genuinely cannot be enumerated,
the gate is met by *naming and counting* it, not by pretending it does not exist.

### Gate 1 — Shape closure

**Every unit maps to a known shape; the unclassified count is zero.**

The same standard `coverage_ledger.py` already enforces for groups, raised to shapes. SD-31 wave 31
established the starting vocabulary: **ten semantic families**, a reduction that survived independent
re-derivation exactly. Gate 1 is met when every unit in the closed census maps to one of them, or the
vocabulary is honestly extended.

### Gate 2 — Engines

**One engine per shape. Run once. Every emitted value fixture-checked.**

Each engine declares its shape handled — a declaration that is only meaningful behind Gates 0 and 1.
Ruling §20's condition is unchanged and non-negotiable: every interpreted value clears
`derived_evaluator_fixture_check`, whose expected value is transcribed from bytes the evaluator never
reads. **An interpreted value with no fixture is not done.**

### Gate 3 — The closure invariant

**A standing test that goes red if any object appears that no shape covers.**

This is what makes *"we will not run it twice"* enforceable rather than hoped for. It must be able to
fail — the same construction as `coverage_ledger.py`, which fails closed on an empty predicate so a
placeholder group cannot manufacture false coverage.

---

## Tooling — and what is deliberately NOT built

Future systems are planned: Starfinder, Traveller, Cyberpunk Red, World of Darkness, Solarus Arcanum.
The operator asked whether census and shape analysis deserve reusable tools. They do, with limits
that are worth stating precisely, because the reuse case is not uniform.

**Starfinder is already in the PCGen checkout** (`data/starfinder`), same `.lst` format, same `.pcc`
includes, same token grammar. For that system the census tool is not *reusable* — it is the **same
tool** pointed at a different directory. That alone pays for building it properly.

**Traveller, Cyberpunk Red and World of Darkness are not in PCGen.** Their rules come from PDFs the
operator will need read and built from. There is no structured source.

### What gets built

* **Census tool** — framework plus a pluggable reader. The LST reader exists in substance already.
  Structure it as reader / analyser / reporter so the seam is present.
* **Shape analyser** — built as a **method, never a taxonomy**. The procedure is portable: extract
  value-bearing expressions, normalise away the nouns, cluster, count the units behind each cluster,
  report coverage. **The ten families are NOT portable and must not be shipped as vocabulary.** They
  are d20 families that assume levels exist and modifiers are numbers. Traveller has no levels at all
  — it is career and skill based, where "per level" is a category error. World of Darkness uses dice
  pools: the shape is not "+2 damage" but "add 2 dice." A shape analyser carrying PF1e's families
  would actively mislead on system two.

### What is NOT built, and why

**No PDF reader in SD-32.** The orchestrator argued for one on the premise that PF1e PDFs existed and
could calibrate the extractor against PCGen ground truth. **The operator corrected that: there are no
PF1e PDFs. The nearest source is scraping d20pfsrd.com.** The argument does not survive the
correction — HTML scraping and PDF extraction have different failure modes (columns, ligatures and
reconstructed tables versus templating and navigation cruft), so calibrating one against the other
measures the wrong thing.

**No speculative generalisation of the reader seam.** We have one worked example. An abstraction drawn
from a single instance reliably comes out shaped like that instance. The seam is built; generalising
it waits for a second real reader to test it against.

### What is genuinely unresolved, and belongs in its own scoping

**How to ingest a system whose rules exist only as prose.** This is a research question, not an
engineering one, and assuming it into SD-32's tool design would be the same error as the PDF reader.

Two things are known and worth carrying forward:

* The pipeline splits in two. **Getting text out of a source is source-specific and does not
  transfer.** Turning prose into structured objects and shapes is shared regardless of source — and it
  is the harder, riskier half, so it is where calibration effort belongs if any is available.
* **The whole anti-gaming apparatus rests on a checkable source.** Every "re-derive it yourself"
  instruction, every mutation proof, every GAMED verdict bottoms out in *the corpus says X, verifiably
  against a pinned SHA*. A prose source removes that foundation: the extraction **is** the corpus, and
  the extraction is the thing most likely to be wrong. The discipline does not port unchanged, and a
  replacement for pinned ground truth must be designed before a prose-sourced system is attempted.

**On d20pfsrd specifically**, if it is ever used: it is Open Game Content, which is legally cleaner
than a commercial PDF, but it is a **subset of the books by construction** — Product Identity is
excluded by design, so deity names and setting content are simply absent. Gaps found against it are
real in the source, not extraction errors. And for PF1e it is largely redundant: PCGen is better
structured, pinned, and already the ground truth every gate depends on. Adding a second PF1e source
creates a conflict-resolution problem that does not currently exist. If tested at all, test it on one
of the four uncompiled books first.

---

## Sequencing

1. **Finish the generator idempotency sweep.** 17 of 29 Rust binaries never checked; 3 of the 12
   checked are vulnerable, one live-reproduced wiping 93 spell and 15 equipment records. Protective,
   closes zero units, still first — scaling engines over a generator that silently empties its own
   fixtures is how thousands of banked units disappear with the suite green.
2. **Gate 0.** Census closure, including the book onboarding that is now a precondition.
3. **Gate 1.** Shape closure against the closed census.
4. **Gate 2.** Engines, one per shape.
5. **Gate 3.** The closure invariant, standing.

Unit-closing work (the compute library, cause closure, class reachability) is sequenced *behind* the
gates by construction, not by preference. Its measured ceilings are in `epic-breakdown.md`.

---

## What SD-32 does NOT promise

**A percentage.** `epic-breakdown.md` arithmetic suggests 72–74% if every epic fully lands, but that
figure sums populations whose overlaps are mostly unmeasured — and SD-31's own THE-BOX had to correct
1,212 double-counted units across just two lanes that each counted honestly. Five epics will collide
worse than two lanes did.

This program's unit estimates have been wrong in both directions and by large factors: 10,163
ingestable units turned out to be 0; 7,505 roster-unlockable turned out to be 0; 180 duplicate chooser
pairs turned out to be 33. **SD-32 commits to the four gates, and reports the board honestly as it
moves.**
