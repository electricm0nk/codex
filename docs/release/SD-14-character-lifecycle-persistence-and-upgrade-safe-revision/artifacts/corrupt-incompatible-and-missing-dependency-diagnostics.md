# Corrupt, Incompatible, and Missing-Dependency Diagnostics

## Objective
Define the diagnostic classes and user-visible outcomes for saved-state failure so later implementation cannot hide persistence failures behind vague error messages or fake success.

## Diagnostic classes

| Class | Typical cause | Minimum visible outcome |
|---|---|---|
| Corrupt save artifact | malformed file, unreadable payload, checksum/structure mismatch, partial write | classify as corrupt; no normal-open claim |
| Incomplete / interrupted save | crash or interrupted write left a partial artifact | classify as incomplete or recovery candidate; do not claim authoritative save success |
| Incompatible schema/app version | current runtime cannot interpret the saved artifact safely | classify as migrate, read-only, or blocked |
| Missing or unsupported content dependency | saved character expects content/rules the current runtime does not support | classify explicitly; preserve saved identity and issue evidence |
| Derived-versus-authoritative drift | recomputed state no longer matches a cached/previous snapshot | surface drift; do not silently rewrite history |

## Required user-visible posture
For each diagnostic class, later implementation must make clear:
- whether normal editability is allowed
- whether read-only inspection is allowed
- whether migration or repair can be attempted
- what saved character and revision the failure applies to
- whether a recovery artifact exists

## Evidence-capture posture
The later runtime should preserve enough structured evidence for SD-11 issue/report flows to cite:
- `character_id`
- `revision_id`
- failure class
- compatibility vectors involved
- whether the artifact was authoritative, autosave, or recovery-only
- whether the app was opening a same-version, upgraded, downgraded, or otherwise changed runtime/content state

## Forbidden behaviors
- generic “could not open file” messages with no classifiable outcome when the contract says classification is possible
- silent fallback to a default/empty character
- silent omission of missing-dependency state from issue or evidence capture
- claiming update failure, save failure, and compatibility failure are the same event when they must stay distinguishable
