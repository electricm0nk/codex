# Pilot UX Flow Requirements

## Objective
Define the minimum truthful user journey for the first GE-07 shell.

## Required pilot flow
1. Launch the desktop shell.
2. Open or load the bounded pilot case.
3. View the current pilot character state.
4. Inspect one or more derived values through explanation affordances.
5. Inspect an invalid or unavailable choice reason when relevant.
6. Inspect diagnostics / validation / unsupported-token visibility when present.
7. Traverse from the active character into the relevant rule or source-package context and return without losing orientation.

## Truth rules
- every step must be backed by real domain outputs from the upstream Codex substrate
- no step may rely on hardcoded numbers as proof of product behavior
- explanation and diagnostics must be inspectable, not merely alluded to

## Nice-to-have but not required for the first flow
- advanced customization workflows
- full package installation management
- export studio breadth
- design-system showcase surfaces

## Completion rule
This artifact is satisfied when a future implementation team can identify the smallest real user journey that proves the shell is exposing genuine Codex behavior instead of a mock demo.
