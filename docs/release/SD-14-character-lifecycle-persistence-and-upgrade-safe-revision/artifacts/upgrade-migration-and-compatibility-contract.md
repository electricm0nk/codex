# Upgrade, Migration, and Compatibility Contract

## Objective
Define the compatibility vectors and outcome states that make saved-character reopen honest across schema, app, content, and governed update changes.

## Compatibility vectors
Every saved character must be classifiable against at least these vectors:

| Vector | Why it matters |
|---|---|
| Persisted schema version | determines whether the save format itself can be read or migrated |
| App/runtime version | determines whether the current build understands the save contract and its recovery posture |
| Content/rules provenance | determines whether the choices and derived state can be interpreted under current supported content |
| Revision lineage | determines whether migration/recovery history is auditable |

## Outcome states
A later executable slice must classify reopen into one of these high-level states:
- `safe` — open/edit normally with no migration required
- `migrate` — migration is required before normal editability can be claimed
- `read-only` — inspection is allowed but editable continuation is not yet safe
- `blocked` — neither normal editability nor safe read-only continuation can be claimed without further repair or support

Additional sub-states may exist, but these four outcomes may not disappear.

## Migration principles
- migration is an explicit transition, not a hidden side effect
- pre-migration and post-migration lineage must remain attributable
- migration may update authoritative state only when the contract says the change is safe and recorded
- when migration safety cannot be proven, read-only or blocked posture is preferred over optimistic mutation

## SD-12 interaction rule
- a successful app update does not prove saved-character compatibility automatically
- a withdrawn, superseded, or downgraded build may change which compatibility paths are available, but it does not erase the need for explicit saved-state classification
- later SD-14 execution work must preserve compatibility evidence across SD-12-adjacent upgrade or rollback flows

## First-slice minimum proof burden
The first executable compatibility slice must prove at least:
- same-version reopen works honestly
- version or provenance mismatch is classified before normal editability is claimed
- blocked/read-only posture remains visible when safe automatic migration is unavailable

## Forbidden behaviors
- silent field dropping during migration
- silent reset to a default character when compatibility fails
- claiming upgrade-safe durability without explicit compatibility vectors
