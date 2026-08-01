# Persisted Character State Contract

## Objective
Define the logical saved-character envelope for SD-14 so later implementation can choose a storage technology without re-inventing what counts as authoritative state.

## Logical artifact classes

| Class | Required purpose | Must be authoritative? |
|---|---|---|
| Saved-character envelope | Carries stable character identity, revision identity, compatibility vectors, and lifecycle metadata | Yes |
| Saved-character authoritative payload | Carries the user-authored character choices needed to reconstruct supported character truth honestly | Yes |
| Derived snapshot/cache material | Carries recomputable summaries, convenience projections, or performance aids | No |
| Recovery/autosave artifact | Carries subordinate recovery material when unsaved work must be recoverable | No, unless explicitly promoted |
| Catalog/index summary | Carries bounded local listing state for saved characters | Mixed: summary only; it must point back to authoritative revisions |

## Mandatory envelope fields
Every authoritative saved-character envelope must carry at least:
- `character_id` — stable identity that survives rename, reopen, and later revision creation
- `revision_id` — identity of the exact saved revision
- `revision_kind` — authoritative, autosave, recovery, archived, or other governed state
- `saved_at` — save timestamp for the revision
- `schema_version` — persisted-artifact schema version
- `app_or_runtime_version` — version/build identity that produced the revision
- `content_or_rules_provenance` — enough package/content/rules lineage to classify compatibility honestly
- `latest_authoritative_revision_ref` or equivalent lineage signal
- `display_label` or equivalent human-facing summary handle

## Authoritative payload families
The authoritative payload must preserve user-authored intent such as:
- character identity/display metadata required by the supported slice
- chosen race/class/level or later supported equivalent selections
- user-authored ability scores, feat/choice selections, and other supported bounded choices
- equipment or other supported selection state where the supported slice treats it as authored truth
- any supported local notes/flags that are explicitly chosen to be authoritative in later lanes

The exact field names may evolve. The contract is that later implementation must preserve user-authored intent strongly enough to reconstruct the supported character honestly.

## Non-authoritative / recomputable families
These may be stored, cached, or omitted, but may not outrank authoritative state:
- derived stat summaries
- explanation snapshots
- convenience indexes or denormalized search material
- UI-local presentation preferences
- any computed value that can be regenerated from the authoritative payload plus current supported rules/content

If such data is cached and later disagrees with recomputation, the system must surface drift explicitly rather than treating the cache as canonical.

## Completion boundary
A later executable slice satisfies this contract only when it can prove:
- authoritative state survives save and reopen
- derived state can be recomputed or invalidated without losing saved intent
- revision identity and compatibility vectors remain visible
- recovery/autosave artifacts remain distinguishable from authoritative saved revisions
