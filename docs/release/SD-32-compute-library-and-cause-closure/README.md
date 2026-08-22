---
canonical: true
owner: god-emporer
status: draft — opened 2026-08-22 during SD-31 wave 31, before that wave's measurement returned
predecessor: SD-31-corpus-closure-grind
tranche: 12 (to be cut from tranche/11 after SD-31 closes)
---

# SD-32 — Compute Library and Cause Closure

## Why this package exists

SD-31 spent thirty waves converting PCGen's rules into this engine feature by feature. The board
moved 15.15% → 35.07%. The operator's standing objection, in their own words, is the thing SD-32
exists to answer:

> *"I really feel like it shouldn't be taking this long to convert rewritten Java logic into rust.
> measure twice, cut once. let's keep measuring."*

They are right, and SD-31's own measurement waves (28, 30, 31) found why. Three findings shape this
package, and none of them is "the rules are hard":

**1. We solved the same problem thousands of times.** Across `data/corpus`, 33,830 formula-bearing
tokens reduce to 14,752 distinct formulas — and those reduce to **ten semantic families**. Not forty,
not one; ten, and that reduction survived independent re-derivation exactly. A skill bonus and a
damage bonus are different nouns wrapped around the same arithmetic. The operator's framing:

> **Correction, and it is the orchestrator's own.** This README first stated "1,049 normalised
> shapes, top 15 covering 80%". The first two figures (33,830 tokens, 14,752 distinct formulas)
> reproduce exactly; **the shape count does not reproduce under any normalisation two independent
> lanes tried, and is retracted.** It was a crude syntactic normalisation written into this package
> as settled fact *before* the measurement that was supposed to establish it returned — the exact
> mistake SD-32 exists to stop. The real answer, ten semantic families, is better and smaller.

> *"1d6 per level. or +2 damage on a dagger, or you get 3 spells at this level and 4 at that level.
> those types of things repeat a lot. you shouldn't be trying to figure them out from scratch for
> every item — you need a library of common computes you can draw from."*

**2. Most blockers were our own plumbing, not the rules.** The archetype is the Monk case: a
complete chassis table sat unreachable for a month because one line of dispatch was missing, and
adding it closed four claim-blocking diagnostics at all 20 levels. SD-31's history holds 98 uses of
"root cause", 332 of "silently" and 17 of "blind spot" across 36,930 lines — a taxonomy nobody had
mined until wave 31.

**3. The engine can only build eleven classes.** `compute_class_chassis` recognises exactly the
eleven CRB base classes. Every prestige class, every Advanced Class Guide class, every archetype
computes correctly and reaches nobody, because no character of that class can exist. `class` is 28
done of 185, and it gates `class_feature` — 60% of everything remaining.

## What SD-32 is, in one sentence

**Stop hand-deriving what repeats; build the compute library, close the plumbing causes by class
rather than by instance, and unblock the classes that make the rest reachable.**

## What SD-32 inherits from SD-31

Load-bearing, and none of it should be rediscovered:

* **A complete inventory.** `SD-31/artifacts/THE-BOX.md` — all 24,914 not-done units in 46 groups,
  uncovered = 0, verified by `scripts/coverage_ledger.py` rather than hand arithmetic. Every group
  carries a todo entry.
* **A working formula interpreter.** `src/rules_core/pilot_compute/formula_interpreter.rs`, semantics
  derived per claim from PCGen's own Java source, reproducing 22 of 22 hand-modelled functions across
  7,040 comparisons with zero disagreements. Reads 84% of corpus arithmetic; refuses the rest by
  name rather than guessing.
* **A trustworthy grant-fact parser** and the merged grant data it produces.
* **The anti-gaming apparatus**, which is the reason any of these numbers can be believed: four GAMED
  verdicts across waves 18–27, every one correct; integration cycles that re-derive rather than trust
  and have caught a load-bearing defect in every wave since 18.
* **The todo directory** (`SD-31/todo/`) as the scheduling layer, reconciled every wave.

## Standing constraints carried forward

These are not negotiable and were each bought with a failure:

* **Anti-gaming (Decision 1a).** A gate that cannot fail is worse than no gate.
* **Proof width.** A correctness proof is only as wide as the cases it covers. Wave 21's parser
  passed its own mutation proof, reproduced 64 records exactly, and still fabricated 73.4% of its
  output. Every lane states which shapes its proof does **not** cover.
* **Fixtures.** Every interpreted value clears `derived_evaluator_fixture_check`, whose expected
  value is transcribed from bytes the evaluator never reads. An interpreted value with no fixture is
  not done. This is the condition operator ruling §20 rests on.
* **Reclassification is not a gain**, and is reported as its own number.
* **§7's prose bar**: shown to a player, proven on screen with the real driver. A record that merely
  loads is not done.
* **§18**: option pools show only valid choices; exclusive pools may not use the browsable pattern.
* **Race attribution frozen**; the Supersession Register proposed, not applied.

## Epics — draft, to be finalised when SD-31 wave 31's measurement returns

| # | Epic | Rests on |
|---|---|---|
| 1 | **The compute library.** Build the top shape families once, prove each once, and reuse. Harvest what already exists — ~166 hand-modelled functions are already proven byte-exact. | wave 31's family clustering and unit counts |
| 2 | **Cause closure.** Take each blocker shape from the root-cause taxonomy and close it corpus-wide rather than instance by instance. | wave 31's taxonomy |
| 3 | **Class reachability.** The 77 prestige classes need entry-requirement gating that exists nowhere in the codebase; 18 real base classes have no table; 28 sit in books with no compiled rule set. | SD-31 wave 27 census |
| 4 | **Book onboarding.** Four books have no compiled rule set at all — 2,300+ units, recurring across five kinds. Cost is calibrated at roughly 1.5–2h per book, dominated by ~7 count-pinning files. | THE-BOX §3 #3 |
| 5 | **Automation, decided on evidence.** Only candidates whose output can be independently checked. A tool that generates values needs a fixture from bytes it does not read, or it manufactures plausible numbers faster than a human could. | wave 31's automation case |

## Open questions for the operator

Carried from SD-31's `todo/blocked.md`, still live:

* **B1** — `mod_only_rescue`: a 249-unit cross-kind phantom-duplicate population that would shrink
  both the `feat` kind and the denominator. Proposed, never ruled.
* **B2** — per-race branch 1/2/3 classification. Race attribution stays frozen until this is answered.
* **B4** — do the 48 structurally-non-PC-class `class` units belong under the class doneness gate at
  all? Monster hit-dice progressions, Eidolon, psionic power-list menus.
* **B5** — are the 5 `Ex-*` records real classes, or PCGen alignment-violation bookkeeping?

B4 and B5 would shrink the honest denominator without changing a line of code.

## Status

**Draft.** Opened during SD-31 wave 31 so the reasoning is captured while it is fresh. The epic
breakdown is deliberately provisional: wave 31 is measuring the compute-shape families and the
root-cause taxonomy right now, and those numbers decide Epics 1, 2 and 5. Finalise after SD-31
closes and `tranche/12` is cut.
