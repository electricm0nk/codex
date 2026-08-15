---
canonical: true
owner: sd31-orchestrator
purpose: Unattended-mode blocker and open-question log for SD-31. The operator reads THIS file on check-in.
started: 2026-08-15
---

# SD-31 — Open Issues, Blockers, and Operator Rulings Needed

**How this file works.** SD-31 runs unattended. A cycle never stops to ask a question. When a cycle
hits a hard block, needs an operator ruling, or takes a default it wants reviewed, it appends a row
here and keeps working. The operator reads this file on check-in and answers in the `Operator ruling`
column.

**Rules for cycles appending here:**
- Append, never rewrite another entry.
- One row per issue. Give the exact command and exit code, not a narration.
- `Severity`: `BLOCKER` (work stopped on this card), `RULING-NEEDED` (proceeded on a default, operator
  should confirm), `NOTE` (recorded, no action wanted).
- A Structural Exclusion Register proposal (`decisions.md §3`) is logged here as `RULING-NEEDED` with
  a pointer to the proposal — a cycle may propose, only the operator grants.

## Open

| # | Opened | Cycle-id | Severity | Issue | What the cycle did instead | Operator ruling |
|---|--------|----------|----------|-------|-----------------------------|-----------------|
| — | — | — | — | *(none yet)* | — | — |

## Resolved

| # | Opened | Closed | Cycle-id | Issue | Resolution |
|---|--------|--------|----------|-------|------------|
| — | — | — | — | *(none yet)* | — |
