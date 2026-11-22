---
canonical: true
owner: god-emporer
status: doctrine, effective 2026-08-22
origin: operator ruling 2026-08-22, after SD-32's first dispatch run closed the bundle and opened a PR over an open card
---

# Blocker closure doctrine

## The rule

**A blocker standing between the bundle and 100% of its Definition of Done gets attacked until it
is cleared, or escalated to the operator. It never gets deferred.**

There are exactly two dispositions for such a blocker:

1. **Clear it.** Do the work. If it is bigger than one cycle, decompose it and run the cycles —
   a large blocker is a sequencing problem, not an exemption.
2. **Raise your hand.** Escalate to the operator, stating what blocks you, what you have already
   tried, and the specific ruling, precondition, or write scope you need. Then **stop and wait** —
   the bundle is paused, not closed.

There is no third disposition. "Filed with a named owner", "forwarded to a successor bundle",
"deferred with reason", and "out of scope for this cycle" are **not** dispositions for a blocker
on the Definition of Done. They are ways of writing down that the bundle is not done.

## What `## Open blockers` actually is

A `## Open blockers` entry is **a request for an operator ruling**. It is the *escalation* half of
disposition 2 — the written form of raising your hand.

It is not a disposition, not a closure path, and not a licence to proceed past the blocked card.
Filing one **pauses the bundle**. Only an operator ruling may move blocked scope out of a card and
into a forward-scope register.

This distinction is the whole doctrine. Before 2026-08-22 the closure criterion in
`workflow-instruction-template.md §11` read *"every criterion `complete` **or** has a filed
`## Open blockers` entry with a named resolution owner"*, and §8 told a blocked cycle to *"write
`## Open blockers`, exit FAIL"* with no statement of what that meant for closure. A cycle could
therefore file a blocker and a later closure cycle could count that filing as satisfying the
criterion. Both halves behaved correctly and the bundle closed over open work.

## What counts as a blocker on the Definition of Done

Any of these, when it stands between the bundle and every acceptance criterion being met:

- an epic or card whose scope is not fully closed;
- a card marked `complete` with a half of its criterion explicitly deferred;
- a measured population that no shape, engine, or gate reaches;
- a defect discovered mid-run that invalidates a criterion already marked met;
- a scope boundary (a file another bundle owns, a missing write permission) that a cycle cannot
  cross on its own authority.

The last one is the clearest case for disposition 2: a cycle that correctly refuses to write
outside its scope has **not** failed. It has hit a blocker only the operator can clear, and the
right move is to prepare the exact change, escalate, and wait.

## Why this exists

On 2026-08-22, SD-32's first dispatch run met all four of its gates and closed twelve of thirteen
cards. Card 11 (`epic-2-cause-closure`) had eight measured blocker shapes — roughly sixteen thousand
units — of which one was closed. The closure cycle filed the remaining seven under `## Open blockers`
with a named owner ("a successor SD-N bundle"), marked card 12 `complete` with half its criterion
deferred, wrote the retrospective, swept the worktrees, and opened the PR.

**Every one of those steps satisfied the criterion as written.** The operator rejected the result:

> *"if card 11 is returned to the backlog, then sd-32 isn't ready for a pr, nor a merge."*

And, on the pattern:

> *"we have had a recurring problem where a blocker was discovered and then instead of clearing it,
> it was just deferred - that is not the right thing to do."*

The failure is not one cycle's judgment. It is that **deferral was the cheapest legal move**, and
the criterion made it legal. A gate that can be satisfied by writing down that you did not do the
work is not a gate.

Note what this costs when it goes unchecked: gates measure that the *method* is sound; cards
measure that the *content* is closed. A bundle closing on gates alone ships a green board over
unfinished work, and the next bundle inherits scope it did not plan for — while the retrospective
records a success.

## Relationship to the deferral-revisit doctrine

`deferral-revisit-doctrine.md` governs a **capability deferral** — a deliberate, up-front scoping
choice of the form *"we are not building X yet"*, made while planning, with a stated revisit
condition. That remains legitimate and is unchanged by this doctrine.

This doctrine governs a **blocker** — something discovered *during execution* that stands between
the bundle and its own stated Definition of Done. The two are easy to conflate, and conflating them
is how a blocker gets laundered into a deferral: it acquires a "named owner" and a
forward-scope-register row and stops looking like unfinished work.

The test: **was this scope in the Definition of Done when the bundle launched?** If yes, it is a
blocker — clear it or escalate. If no, it may be a deferral, and the revisit doctrine applies.

## How to apply it

**Writing a bundle's closure criterion.** The final-acceptance scan requires every epic card at
`complete`. Never write "complete **or** filed" — that phrasing is the defect. If a criterion in a
package you inherit contains it, that is a finding to fix before launch, not prose to work around.

**Running a cycle that hits a blocker.** Attack it first. Decompose it, measure it, take it in
cycles. Escalate only when the blocker is genuinely outside your authority — a ruling, a write
scope, a decision only the operator can make — and when you escalate, name the exact thing you
need. "This is hard" is not an escalation; "I need write scope to `<path>` to fix `<defect>`" is.

**Running a closure cycle.** Read the board, not the filings. Any card short of `complete` stops
the closure — do not write the retrospective, do not sweep, **do not open the PR**. Report what is
short, with the command that shows it, and exit. That is a correct outcome for a closure cycle.

**Orchestrating.** When a run reports work deferred, do not relay it as a closed bundle. Reopen the
closure card, close any premature PR, and dispatch the real work. If the deferral needs an operator
ruling, surface it as a decision with options — not as a completed result with a footnote.
