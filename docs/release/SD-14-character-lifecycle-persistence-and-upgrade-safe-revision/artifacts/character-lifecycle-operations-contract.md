# Character Lifecycle Operations Contract

## Objective
Define the bounded lifecycle operations for local saved characters so later implementation cannot improvise semantics ad hoc.

## Operations table

| Operation | Preconditions | Success outcome | Blocked / failure posture |
|---|---|---|---|
| Create | supported character input exists | new `character_id` and first authoritative revision are created | if required authoritative input is missing or invalid, no save success is claimed |
| Open / Reopen | saved character artifact exists | character loads in normal editable posture when compatibility is safe | if compatibility is unsafe, classify as migrate, read-only, or blocked rather than pretending normal open |
| Save | current edit session is valid for authoritative save | current session becomes the latest authoritative revision | partial write, invalid authoritative payload, or interrupted write must not report success |
| Save new revision / Save As | current session or source character exists | new revision or duplicated lineage is recorded explicitly | if lineage cannot be recorded honestly, refuse success |
| Duplicate | source saved character exists | new `character_id` is created with explicit lineage break or fork semantics | do not silently alias two saved characters to one identity |
| Archive | source saved character exists | saved character becomes archived but recoverable when the slice supports recovery | archive must not masquerade as delete |
| Delete | source saved character exists | saved character becomes deleted or permanently removed according to the later slice contract | destructive outcome must remain explicit; do not silently hide it as archive |

## First-slice mandatory behaviors
The first executable SD-14 slice must support at minimum:
- create
- reopen
- authoritative save
- compatibility classification on reopen
- explicit failure posture when save/open cannot be trusted

Duplicate, archive, and delete may be split into adjacent slices, but only if the handoff preserves the rest of the lifecycle contract honestly.

## Required visible state
A later implementation of this contract must keep visible:
- current `character_id`
- current `revision_id`
- whether the current session is dirty or clean
- whether the current revision is authoritative, autosave, recovery, archived, or blocked
- the compatibility verdict before editability is claimed

## Forbidden behaviors
- no silent overwrite of a different character identity
- no reporting save success before the authoritative revision is durable
- no silent reopen into a newer or incompatible interpreted meaning without classification
- no use of archive/delete labels interchangeably
