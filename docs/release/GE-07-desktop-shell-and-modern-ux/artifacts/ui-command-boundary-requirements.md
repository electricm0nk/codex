# UI Command Boundary Requirements

## Objective
Define what the shell must be able to ask of the core substrate, and what it is forbidden to do itself.

## Required boundary capabilities
A future GE-07 implementation must be able to request:
1. pilot character snapshot / current state
2. explanation for a selected value or invalid choice
3. validation / problems payload
4. import diagnostics / unsupported-token payload
5. rules-library browse/search payload for the bounded pilot scope
6. source-package / provenance inspection payload

## Boundary rules
- payloads must be structured rather than forcing the UI to scrape text blobs where avoidable
- explanation payloads must preserve enough source/modifier/provenance detail for a truthful UI rendering
- diagnostics payloads must preserve warning severity and category rather than flattening everything into “info”
- the boundary must let the UI correlate a visible value or warning with the relevant detail surface

## Explicit UI prohibitions
The UI is forbidden to:
- recompute authoritative rules values
- run prerequisite logic as a second semantic engine
- fabricate provenance/source references
- suppress upstream failures by omitting them from the returned view model

## Still unresolved here
- exact transport mechanism
- exact payload schema names
- whether the first boundary is synchronous command calls, an evented channel, or another explicit mechanism

## Completion rule
This artifact is satisfied when a future readiness closure can turn these duties into a bounded adapter contract without guessing what the shell must ask for or what it must never own.
