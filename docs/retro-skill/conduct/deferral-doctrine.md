# Deferral revisit doctrine

## The rule

**A ruling that defers a capability must name the condition under which it is revisited, and that
condition must be checked — not remembered.**

Any decision of the form *"we are not building X yet"* is incomplete unless it states:

1. **The condition** that would make X worth building — specific enough to be evaluated, not "when
   it becomes a problem."
2. **Who or what checks it, and how often.** A condition nobody is scheduled to evaluate is a
   condition nobody will evaluate.
3. **The cost being accepted** in the meantime, so the trade-off is visible when the decision is
   re-read later, by someone who was not there when it was made.

The one-line test that separates a deferral from a blocker: **was this scope in the Definition of
Done when the bundle launched?** If yes, it is a blocker — see `blocker-doctrine.md`: clear it or
escalate, never defer it. If no — it was never in scope, and this is a genuine up-front choice not
to build something yet — this doctrine applies.

## Why this exists

Consider a planning ruling like: *"No generic rule-interpreter for discount calculations. Each
promotion type is a hand-written, individually tested function."* The reasoning is sound: an
interpreter is the highest-risk option for **silently** wrong output — a misread token still
produces a plausible-looking price that nobody double-checks — while a hand-written function that is
wrong simply fails its test. The ruling accepted the extra cost of hand-writing each one, deliberately,
in exchange for that safety property.

Its own stated concern already named the condition that would resolve it: build a mechanism that can
check interpreted output against a known-correct answer. Suppose that mechanism lands, much later,
as part of unrelated work — a fixture-based checker that pins a computed value against an expected
value taken from an independent source, and is proven able to catch a wrong answer.

If nobody re-reads the original ruling, it can stand for a long time after its own condition was
satisfied — during which the team keeps hand-transcribing arithmetic that a checked interpreter could
now safely compute, because the ruling that forbade the interpreter was never revisited even after
the very thing it was waiting for showed up.

That is the failure this doctrine prevents. Not a wrong decision: **a decision that was right when
made, whose expiry condition was met and never checked.**

## What this doctrine does NOT cover — blockers

This doctrine governs a **capability deferral**: a deliberate, up-front scoping choice of the form
*"we are not building X yet,"* made while planning, with a stated revisit condition.

It does **not** cover a **blocker** — something discovered *during execution* that stands between
the bundle and its own stated Definition of Done. Those are governed by `blocker-doctrine.md`: clear
it, or escalate to the operator and wait. Never defer it.

The two are easy to conflate, and conflating them is exactly how a blocker gets laundered into a
deferral — it acquires a "named owner" and a forward-scope-register row and stops looking like
unfinished work.

## What it does not mean

It is **not** a licence to relitigate settled decisions. A ruling with no stated condition is simply
in force. The obligation is on the *author* of a deferral to state the condition, and on whoever
later inherits the ruling to check the conditions it came with.

It also does not weaken the ruling being revisited. If a deferral is overturned because its condition
was met, the safety property or standard it was protecting should be **kept** — only the mechanism
changes, not the bar.

## Worked example

A storefront selling lawn ornaments defers dynamic per-region tax calculation at launch: *"We ship
with three hard-coded flat rates. No general tax-rules engine yet."* Written properly, that deferral
reads:

- **Condition:** the store starts selling into a fourth region, or any of the three flat rates
  changes more than once in a quarter.
- **Checker:** the monthly regional-sales report, reviewed by whoever owns pricing, flags any order
  from an unlisted region or any rate correction.
- **Accepted cost:** for now, out-of-region customers see no tax at checkout and get invoiced
  separately by hand — slower, and it does not scale past a handful of regions.

Six months later, the monthly report shows orders from a fourth region. That is the condition firing.
The deferral is now a decision waiting to be made, found in a routine check rather than three quarters
later when someone finally asks why regional tax handling still doesn't exist.

## How to apply it

**When writing a deferral:** state condition, checker, and accepted cost. Three lines.

**When inheriting a body of work:** enumerate the deferrals you inherited and evaluate each stated
condition once, at the start. A deferral whose condition has already been met is a decision waiting
to be made, and it is far cheaper to find that in scoping than deep into execution.

**When a deferral has no stated condition:** that is itself a finding. Record it, and ask whoever
owns the decision for the condition rather than inventing one yourself.
