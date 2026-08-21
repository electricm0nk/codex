---
canonical: true
owner: sd31-orchestrator
purpose: The scheduling layer for SD-31. OPEN-ISSUES.md records what was found; this decides what
  happens to it. Reconciled at the end of every wave, without exception.
created: 2026-08-21
---

# SD-31 TODO — how this works

## Why it exists

`OPEN-ISSUES.md` had grown to **379 rows, 246 of them plain NOTEs**. It is append-only: findings go
in and nothing takes them out. The result was measurable waste:

* **The race-trait key matcher.** Found wave 19. Named again in waves 23, 24, 25, 26 and 27 — eleven
  mentions across the package docs, eight in `progress.md` alone. Wave 22 fixed half of it
  (hyphen/space normalisation) and the compound-key half is still open today. Six waves of naming a
  defect nobody was ever scheduled to close.
* **The Monk case.** A code comment written 2026-07-29 recorded that Monk's chassis table had been
  complete all along and only a one-line dispatch mapping was missing — and that adding it closed
  four claim-blocking diagnostics at all 20 levels. Nobody asked whether other classes were in the
  same state until wave 27, nearly a month later. That question is now the highest-yield item in the
  program.

The failure was never recording. It was **reconciliation**. This directory is that step.

## The four files

| File | Holds |
|---|---|
| `sweeps.md` | "This pattern may apply elsewhere — has the whole corpus been checked?" |
| `defects.md` | Known-wrong things, with blast radius |
| `blocked.md` | Waiting on an operator ruling, with the exact question |
| `levers.md` | Structural work that unblocks many units at once |

## The rule that makes it work

**A sweep is not done when one instance is fixed. It is done when the corpus has been checked and
the remaining count is stated.** The Monk fix was real and it was not a sweep; that distinction is
the whole point of `sweeps.md`.

## Reconciliation, required at the end of every wave

The integration cycle must, before writing its receipt:

1. **Close** items the wave actually finished — with the evidence, not the intent.
2. **Add** every new finding, filed into the right one of the four files. A finding that names a
   pattern ("X may be true elsewhere") goes in `sweeps.md`, not `defects.md`.
3. **Re-state** anything a lane re-discovered that was already open — and say why it was not closed.
   Repeated re-discovery is the signal this directory exists to catch.
4. **Never silently carry.** An item that survives a wave untouched must say which wave last looked
   at it. Three waves untouched means it is either not important or not actually actionable, and it
   should be demoted or closed with a reason.

## What does NOT go here

Operator rulings live in `artifacts/OPERATOR-RULINGS-*.md` and `decisions.md`. Wave receipts live in
`progress.md`. Raw findings still land in `OPEN-ISSUES.md`. This directory only ever holds **what
happens next**, and every item points back at its source row.
