# SD-14 Character Lifecycle, Persistence, and Upgrade-Safe Revision Technical Requirements

## Purpose
This document defines the normative requirements for saved-character continuity in Codex: what must be persisted, what may be recomputed, how character lifecycle operations behave, how upgrades and migrations are classified, and how failure remains visible.

## Current-state grounding
- GE-06 already proves a bounded character-input and computation substrate, but it is still a deterministic proof slice rather than a durable saved-character system.
- GE-10 already proves a buildable developer-facing desktop shell and honest onboarding path, but not persistent local character continuity.
- SD-11 already defines tester-facing workbench and evidence-capture truth, including save-file attachment vocabulary, but not save/load implementation or migration behavior.
- SD-12 already defines distribution/update/rollback truth, but not whether local saved state survives those events.
- The live repo has an adjacent persistence substrate for GE-08 authored packages in `src/homebrew_authoring/package_store.rs`; it does not yet expose an accepted character save/load or migration subsystem.

## Requirement families

### 1. Boundary and scope requirements
- SD-14 MUST define saved-character continuity as a product-truth obligation for bounded supported character state.
- SD-14 MUST remain local-first and single-user.
- SD-14 MUST NOT imply cloud sync, account identity, multiplayer, roster sharing, or campaign/world-state persistence.
- SD-14 MUST NOT claim broader character/content coverage than the adjacent accepted rules/model lanes support.
- SD-14 MUST preserve the distinction between:
  - GE-06 and later character-domain truth
  - SD-11 tester-facing workbench/evidence truth
  - SD-12 distribution/update/rollback truth
  - SD-14 saved-character continuity truth

### 2. Persisted artifact classes
The persisted-character contract MUST define at least these artifact classes:
1. a saved-character identity envelope
2. a saved-character authoritative payload
3. a revision and recovery record surface
4. a compatibility/migration preflight surface
5. a diagnostic surface for corrupt, incompatible, or missing-dependency state

The exact physical storage technology may remain open, but the logical classes above may not.

The same-epic artifact `artifacts/persisted-character-state-contract.md` is authoritative for the external contract of these classes.

### 3. Authoritative versus derived state
- The packet MUST define which saved data is authoritative user-authored truth and which data is recomputable derived state.
- Authoritative state MUST include enough user-authored intent to reconstruct the supported character honestly after reopen.
- Recomputable derived state MUST NOT be treated as the only source of truth for character identity, selected options, or persisted intent.
- Later implementation MAY cache derived snapshots for UX/performance, but those caches MUST remain invalidatable and subordinate to authoritative state.
- When recomputation differs after rules/content/app changes, the system MUST surface the delta explicitly rather than overwriting history silently.

### 4. Identity, version, and provenance requirements
Every persisted character MUST carry enough information to answer whether reopen is:
- safe with no migration
- requires migration
- allowed only in read-only / diagnostic posture
- blocked pending missing dependency or incompatible state resolution

At minimum, the packet MUST require explicit treatment of:
- stable character identity
- revision identity
- save timestamp and latest-authoritative-save status
- schema version for persisted character format
- app/build or runtime version that authored the save
- content/rules provenance sufficient to explain what the save expected
- any bounded source-package or dependency handles needed for honest reopen

The same-epic artifact `artifacts/upgrade-migration-and-compatibility-contract.md` is authoritative for the compatibility-vector surface.

### 5. Lifecycle operation requirements
The lifecycle contract MUST define observable behavior for:
- create new character save
- open/reopen saved character
- save current authoritative revision
- save a new revision / save-as equivalent when required
- duplicate a saved character
- archive and restore archived character state when supported by the slice
- delete a saved character

For each operation, the contract MUST define:
- preconditions
- success outcome
- blocked/error outcome
- what identity/revision fields change
- what evidence or user-visible state confirms the result

The same-epic artifact `artifacts/character-lifecycle-operations-contract.md` is authoritative for this surface.

### 6. Dirty state, revision, autosave, and recovery requirements
- The packet MUST define what counts as the latest authoritative save.
- The packet MUST define how unsaved changes remain visible.
- The packet MUST define the minimum autosave or backup posture required before tester data-loss claims become unacceptable.
- The packet MUST define interrupted-write and crash-recovery behavior.
- Recovery posture MUST preserve evidence of what was recovered, what is authoritative, and what may require user review.
- A stale autosave MUST NOT silently replace a newer authoritative save.

The same-epic artifact `artifacts/revision-autosave-and-recovery-policy.md` is authoritative for this surface.

### 7. Migration and compatibility requirements
- The packet MUST define compatibility decisions over at least schema version, app/runtime version, and content/rules provenance.
- Migration MUST be classified as an explicit state machine, not an invisible implementation side effect.
- When migration cannot be completed safely, the system MUST surface a blocked or read-only posture rather than mutating state optimistically.
- Rollback-adjacent behavior MUST preserve saved-state truth even when SD-12 changes which build is current, preferred, withdrawn, or blocked.
- The packet MUST define how missing or changed content dependencies are diagnosed and how that diagnosis affects reopen/editability.
- The packet MUST explicitly forbid silent field dropping, silent coercion, and silent reset in the name of compatibility.

The same-epic artifact `artifacts/upgrade-migration-and-compatibility-contract.md` is authoritative for this surface.

### 8. Failure, corruption, and missing-dependency requirements
The packet MUST define explicit diagnostic behavior for at least:
- unreadable or corrupt save artifact
- incomplete or partially written save
- schema version too old/new for safe automatic migration
- content dependency missing or unsupported
- authoritative-versus-derived mismatch after recomputation
- withdrawn or unsupported build attempting to open a save with incompatible expectations

For each class, the packet MUST define:
- severity
- whether reopen is allowed
- whether read-only inspection is allowed
- whether migration or repair can be offered
- what evidence should be available for tester issue capture

The same-epic artifact `artifacts/corrupt-incompatible-and-missing-dependency-diagnostics.md` is authoritative for this surface.

### 9. SD-11 and SD-12 coupling requirements
- SD-14 MUST feed tester-facing status/evidence surfaces without allowing SD-11 to become persistence authority.
- Save/load, corruption, and migration outcomes MUST be representable in tester evidence capture.
- SD-14 MUST respect SD-12 update/rollback truth while separately proving whether local saved state survives those changes.
- If a build is withdrawn or downgraded under SD-12, SD-14 MUST define how saved characters remain classifiable and recoverable.

### 10. Progression continuity requirements
- Persistence MUST preserve meaningful continuation of bounded supported character state over time.
- Reopen after close/relaunch MUST preserve authoritative character identity and saved intent.
- Changes in supported rules/content MAY force recomputation or compatibility review, but MUST NOT erase or counterfeit prior user-authored state.
- This lane MUST remain continuity authority for supported state over time even as broader scope grows later.

### 11. Verification and proof obligations for later execution lanes
A future execution handoff derived from this packet MUST name exact commands and fixtures, but it may not weaken these proof classes:
- roundtrip save/load proof for supported characters
- reopen-after-relaunch proof
- dirty-state and authoritative-revision proof
- interrupted-write / autosave / recovery proof
- compatibility preflight proof across version or content changes
- blocked/read-only/corrupt/missing-dependency diagnostic proof
- issue-evidence or operator-evidence proof where failure must be reported

### 12. Non-goals
This packet does not authorize:
- backend sync services
- public sharing or collaboration workflows
- campaign or journal state
- unbounded roster management breadth
- changing GE-06 computation truth or SD-12 release/update doctrine by implication
