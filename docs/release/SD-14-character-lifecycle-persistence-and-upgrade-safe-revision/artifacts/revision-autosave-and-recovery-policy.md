# Revision, Autosave, and Recovery Policy

## Objective
Define what counts as the latest authoritative save, how unsaved work remains visible, and how interrupted writes or crashes are handled without counterfeit durability claims.

## Authoritative revision rules
- one revision must be classifiable as the latest authoritative revision for a saved character
- authoritative revision promotion happens only on real save success
- autosave or recovery artifacts may assist recovery but do not silently outrank the latest authoritative revision
- revision lineage must remain inspectable enough to explain what the user is looking at

## Dirty-state posture
- once user-authored state differs from the latest authoritative revision, the session must be dirty
- dirty state must remain visible until either save succeeds or the session is explicitly discarded/reverted
- dirty state must not be cleared merely because a derived snapshot was recomputed

## Autosave posture
The first bounded executable slice may choose a narrow autosave depth, but it must define:
- when autosave triggers
- whether autosave is per character, per session, or per revision family
- how autosave artifacts are labeled
- when autosave artifacts are pruned or retained

## Interrupted-write and crash recovery posture
- interrupted writes must not be reported as authoritative save success
- recovery artifacts must be distinguishable from authoritative revisions
- on restart, the user must be able to tell whether recovery data exists and whether it is authoritative, recoverable, or merely diagnostic
- if recovery cannot be trusted, blocked or read-only posture is preferred over optimistic restoration

## Minimum recovery evidence
Later implementation must preserve enough evidence to answer:
- what authoritative revision existed before the interruption
- whether a recovery artifact is newer than the authoritative revision
- whether the recovery artifact is complete enough to offer restore or only diagnostic inspection

## Forbidden behaviors
- clearing dirty state without durable authoritative save
- silently promoting autosave over a newer authoritative revision
- silently discarding recovery material with no user-visible trace when the slice claims recovery support
