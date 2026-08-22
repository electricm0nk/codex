---
canonical: true
owner: god-emporer
status: doctrine, effective 2026-08-22
origin: SD-31 retrospective — the single most expensive procedural failure in the package
---

# Deferral revisit doctrine

## The rule

**A ruling that defers a capability must name the condition under which it is revisited, and that
condition must be checked — not remembered.**

Any decision of the form *"we are not building X yet"* is incomplete unless it states:

1. **the condition** that would make X worth building — specific enough to be evaluated, not "when it
   becomes a problem";
2. **who or what checks it**, and how often. A condition nobody is scheduled to evaluate is a
   condition nobody will evaluate;
3. **the cost being accepted** in the meantime, so the trade is visible when it is re-read.

## Why this exists

On 2026-07-31, SD-27 `decisions.md §24.1` ruled: *"No formula interpreter. Each feature is a
hand-written, corpus-verified pure function."* The reasoning was sound — an interpreter is the
highest-risk option for **silently** wrong answers, and a hand-modelled formula that is wrong is a
failing test while a misinterpreted token is a plausible number nobody checks. The ruling accepted
linear cost deliberately in exchange for that property.

Its own stated concern named the condition that would resolve it: a mechanism that could check
interpreted output. **That mechanism landed in wave 13** — `derived_evaluator_fixture_check`, which
pins a computed value against an expected value transcribed from bytes the evaluator never reads, and
is mutation-proven able to fail.

**Nobody re-read the ruling.** It stood for roughly **eighteen further waves**, during which the
program hand-transcribed arithmetic that was sitting in the corpus in machine-readable form the whole
time. When the operator finally questioned it — *"it feels like you are trying to reinvent the
wheel"* — they also said they did not recall making the original ruling.

That is the failure this doctrine prevents. Not a wrong decision: **a decision that was right when
made, whose expiry condition was satisfied and never checked.**

## What it does not mean

It is **not** a licence to relitigate settled decisions. A ruling with no stated condition is simply
in force. The obligation is on the *author* of a deferral to state the condition, and on the package
that inherits it to check the conditions it inherited.

It also does not weaken the ruling being revisited. When §24.1 was overturned, the safety property it
protected was **kept** — every interpreted value still clears the fixture gate. The overturn changed
the mechanism, not the standard.

## How to apply it

**When writing a deferral:** state condition, checker, and accepted cost. Three lines.

**When inheriting a package:** enumerate the deferrals you inherited and evaluate each stated
condition once, at the start. A deferral whose condition has been met is a decision waiting to be
made, and it is cheaper to find it in scoping than in wave nineteen.

**When a deferral has no stated condition:** that is itself a finding. Record it, and ask the operator
for the condition rather than inventing one.
