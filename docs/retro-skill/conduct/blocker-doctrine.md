# Blocker closure doctrine

## The rule

**A blocker standing between a work bundle and 100% of its Definition of Done gets attacked until it
is cleared, or escalated to the operator. It never gets deferred.**

There are exactly two dispositions for such a blocker:

1. **Clear it.** Do the work. If it is bigger than one cycle, decompose it and run several cycles —
   a large blocker is a sequencing problem, not an exemption. A fix that actually lives in another
   subsystem, another team's component, or another part of the codebase is still a fix; it does not
   stop being your responsibility to see closed just because the change happens somewhere else.
2. **Raise your hand.** Escalate to the operator, stating what blocks you, what you have already
   tried, and the specific ruling, precondition, or scope you need. Then **stop and wait** — the
   bundle is paused, not closed.

There is no third disposition. "Filed with a named owner," "forwarded to a successor bundle,"
"deferred with reason," and "out of scope for this cycle" are **not** dispositions for a blocker on
the Definition of Done. They are ways of writing down that the bundle is not done.

## What an `## Open blockers` entry actually is

An `## Open blockers` entry is **a request for an operator ruling**. It is the *escalation* half of
disposition 2 — the written form of raising your hand.

It is not a disposition, not a closure path, and not a licence to proceed past the blocked item.
Filing one **pauses the bundle**. Only an operator ruling may move blocked scope out of the bundle
and into some forward-scope register.

## What counts as a blocker on the Definition of Done

Any of these, when it stands between the bundle and every acceptance criterion being met:

- a criterion or cycle whose scope is not fully closed;
- an item marked "complete" with part of its criterion explicitly deferred;
- a measured population that no check, tool, or gate actually reaches;
- a defect discovered mid-run that invalidates a criterion already marked met;
- a scope boundary (a component another team owns, a permission you don't hold) that a cycle cannot
  cross on its own authority.

The last one is the clearest case for disposition 2: a cycle that correctly refuses to touch
something outside its scope has **not** failed. It has hit a blocker only the operator can clear,
and the right move is to prepare the exact change, escalate, and wait.

## Why this exists

A closure criterion once read, in effect: *"every criterion is complete, **or** has a filed blocker
entry with a named owner."* A dispatch run met every other gate, closed almost all of its work, and
for the one item it could not finish, it filed the remainder under `## Open blockers` with a named
owner ("a future bundle"), marked the surrounding item "complete" with half its criterion deferred,
and proceeded to close out and open a request to merge.

**Every one of those steps satisfied the criterion as written.** The operator rejected the result
anyway: work that is filed away instead of finished is not the same as work that is done, no matter
how correctly each individual step followed the letter of the rule.

The failure was not one person's or one cycle's judgment. It was that **deferral was the cheapest
legal move**, and the criterion made it legal. A gate that can be satisfied by writing down that you
did not do the work is not a gate. Grep your own closure criteria for that exact phrasing shape —
"complete, or has a filed blocker/exception with a named owner" — because if it is present, it will
get used exactly this way sooner or later.

Note what this costs when it goes unchecked: gates measure that the *method* is sound; the actual
line items measure that the *content* is closed. A bundle that closes on gates alone ships a green
board over unfinished work, and whatever comes next inherits scope it never planned for — while the
retrospective record shows a success.

## The sibling distinction: needing more cycles is not a blocker

A blocker is a wall you cannot get past without a ruling or a scope you do not have. **Needing more
time or more cycles to finish is not a blocker** — it is a partial result with a named remainder, and
it is a completely normal thing to report.

A cycle that closes part of its assigned population and names every remaining unit by sub-cause, with
the closed count and the remaining count summing exactly to the original population, should report
`partial` — not `blocked` and not `complete` — and the dispatch simply continues into the next cycle.

Only a genuine question the cycle is not authorized to decide on its own — a ruling, a scope
decision, a trade-off only the operator can make — is an escalation.

Watch your status vocabulary here: if the only words on offer are "complete" or "blocked," a cycle
doing perfectly good partial work is forced to pick the word that halts the entire bundle, or — worse
— tempted to round its partial result up to "complete" so it doesn't have to stop anything. Give
partial work its own honest word, and reserve "blocked" for the cases in this doctrine.

## Relationship to the deferral doctrine

This doctrine governs a **blocker** — something discovered *during execution* that stands between
the bundle and its own stated Definition of Done: clear it or escalate it.

It is distinct from a **planned capability deferral** — a deliberate, up-front scoping choice made
while planning, with a stated revisit condition. That remains legitimate. See
`deferral-doctrine.md`. The two are easy to conflate, and conflating them is exactly how a blocker
gets laundered into a deferral: it acquires a "named owner" and a forward-scope-register row and
stops looking like unfinished work.

The test: **was this scope in the Definition of Done when the bundle launched?** If yes, it is a
blocker — clear it or escalate. If no, it may be a legitimate deferral, and the deferral doctrine
applies instead.

## How to apply it

**Writing a bundle's closure criterion.** The final acceptance check requires every criterion at
"complete." Never write "complete **or** filed" — that phrasing is the defect. If a package or
template you inherit contains it, that is a finding to fix before launch, not prose to work around.

**Running a cycle that hits a blocker.** Attack it first. Decompose it, measure it, take it in
cycles. Escalate only when the blocker is genuinely outside your authority — a ruling, a permission,
a decision only the operator can make — and when you escalate, name the exact thing you need. "This
is hard" is not an escalation; "I need write access to `<component>` to fix `<defect>`" is.

**Running a closure cycle.** Read the board, not the filings. Any item short of "complete" stops the
closure — do not write the retrospective, do not clean up, **do not open the request to merge**.
Report what is short, with the check that shows it, and exit. That is a correct outcome for a
closure cycle.

**Orchestrating.** When a run reports work deferred, do not relay it as a closed bundle. Reopen the
closure item, close any premature merge request, and dispatch the real work. If the deferral needs an
operator ruling, surface it as a decision with options — not as a completed result with a footnote.
