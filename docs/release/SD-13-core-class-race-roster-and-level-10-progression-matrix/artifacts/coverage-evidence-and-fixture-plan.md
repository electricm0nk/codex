# Coverage Evidence and Fixture Plan

## Objective
Define the minimum fixture families and evidence classes required before SD-13 support-state changes are honest.

## Evidence-class posture
Support-state promotions should generally respect this order:
1. structural representability or observed scope grounding
2. deterministic computed proof
3. known-gap capture for anything still missing, lossy, or blocked
4. oracle-checked proof where the claim tier needs it
5. tester-visible support language aligned to the resulting state

A row may remain product-visible only if its state and evidence class still match reality.

## Minimum fixture families

### F1 — Input-shape fixtures
Purpose:
- prove the targeted race/class/level selections can be represented and loaded structurally

### F2 — Progression fixtures
Purpose:
- prove the targeted class through the relevant level checkpoints, including level 10 for any row claiming bounded level-10 support

### F3 — Feature-pressure fixtures
Purpose:
- prove class-specific pressure such as rage, talents, combat styles, spell contexts, or other mandatory feature families

### F4 — Race-semantics fixtures
Purpose:
- prove the targeted race-linked semantics that alter the honesty of the row

### F5 — Gap/diagnostic fixtures
Purpose:
- prove that partial, lossy, blocked, or unverified outcomes surface the right reasons instead of fake success

### F6 — Evidence-refresh or parity fixtures
Purpose:
- when a row is being promoted beyond merely computed internal support, prove the required parity or refresh posture for that state change

## State-promotion rules
- `supported` requires all mandatory fixture families for the targeted row and claim tier.
- `partial` requires explicit proof of what works plus explicit ledger capture of what does not.
- `lossy` requires explicit proof of the simplified or approximated path plus explicit recording of what semantics were discarded.
- `blocked` requires explicit identification of the blocker and a durable evidence trail.
- `unverified` must remain visible whenever proof is missing or stale.

## Refresh posture
- support-state changes must update the row state, supporting evidence references, and any affected tester-facing language together
- stale evidence should bias toward demotion or `unverified` rather than toward silent optimism
- a build shipping broader UI affordances without refreshed breadth evidence should not promote row states automatically
