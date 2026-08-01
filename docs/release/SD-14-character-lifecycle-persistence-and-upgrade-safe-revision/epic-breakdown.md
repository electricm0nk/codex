# SD-14 Character Lifecycle, Persistence, and Upgrade-Safe Revision Epic Breakdown

## Breakdown rule
This file decomposes the SD-14 source STC into implementation-facing epics and feature seeds without becoming an execution handoff.

## Epic SD14-E1 — Saved-character identity, artifact classes, and local-store boundary
**Objective:** Define and later implement the saved-character envelope, catalog/index posture, and explicit split between authoritative character state and subordinate derived/cache material.

**Derived from:**
- SD-14 README: Objective / In Scope / Acceptance Summary
- `technical-requirements.md`: Persisted artifact classes; Authoritative versus derived state
- `artifacts/persisted-character-state-contract.md`

**Depends on:**
- GE-06 bounded character-input truth
- current repo character-input substrate in `src/rules_core/character_input.rs`

### Feature seed SD14-F1 — Saved-character envelope
**Outcome:** Every saved character has stable identity, revision identity, version/provenance vectors, and explicit authoritative payload boundaries.

**Acceptance signals:**
- identity/version/provenance fields are explicit
- authoritative versus derived state is machine-distinguishable

**Notes:**
- later handoff must choose exact storage shape without weakening the envelope contract

### Feature seed SD14-F2 — Local catalog and summary surface
**Outcome:** Saved characters can be enumerated with bounded local summary state without turning the catalog into campaign-management breadth.

**Acceptance signals:**
- a bounded local character index exists
- archive/deleted/current state remains classifiable

**Notes:**
- the first execution slice may stay narrow, but it must still preserve truthful lifecycle state

## Epic SD14-E2 — Save/load/reopen and lifecycle operations
**Objective:** Implement the lifecycle coordinator for create/open/save/reopen/duplicate/archive/delete behavior over bounded local character artifacts.

**Derived from:**
- `artifacts/character-lifecycle-operations-contract.md`
- `technical-requirements.md`: Lifecycle operation requirements

**Depends on:**
- SD14-E1
- SD-11 tester evidence posture

### Feature seed SD14-F3 — Authoritative save and reopen
**Outcome:** A bounded supported character can be saved, closed, reopened, and recognized as the same saved character/revision lineage.

**Acceptance signals:**
- reopen preserves authoritative choices and identity
- no fake save success occurs without durable authoritative revision state

**Notes:**
- later handoff must name exact fixtures and roundtrip verification commands

### Feature seed SD14-F4 — Duplicate, archive, and delete posture
**Outcome:** Non-destructive lifecycle operations remain explicit and evidence-bearing instead of ad hoc filesystem side effects.

**Acceptance signals:**
- duplicate preserves lineage semantics honestly
- archive/delete states are distinguishable and recoverable only when the contract says so

**Notes:**
- do not broaden this slice into broad roster-management UX

## Epic SD14-E3 — Revision, autosave, and interrupted-write recovery
**Objective:** Implement the revision lineage, dirty-state visibility, autosave/backup posture, and interrupted-write recovery contract.

**Derived from:**
- `artifacts/revision-autosave-and-recovery-policy.md`
- `technical-requirements.md`: Dirty state, revision, autosave, and recovery requirements

**Depends on:**
- SD14-E1
- SD14-E2

### Feature seed SD14-F5 — Dirty-state and authoritative-revision tracking
**Outcome:** The runtime can distinguish unsaved work from the latest authoritative saved revision.

**Acceptance signals:**
- dirty/clean state is visible
- authoritative revision identity updates only on real save success

**Notes:**
- no autosave or crash recovery artifact may silently replace the latest authoritative save

### Feature seed SD14-F6 — Autosave and recovery artifacts
**Outcome:** Interrupted or crashed sessions leave explicit recovery posture instead of silent data loss.

**Acceptance signals:**
- recovery artifacts are distinguishable from authoritative revisions
- restart/reopen can classify recovery availability honestly

**Notes:**
- later handoff must name exact retention depth and cleanup rules

## Epic SD14-E4 — Compatibility, migration, and upgrade-safe survival
**Objective:** Implement the compatibility preflight and migration state machine that classifies saved-character reopen across schema/app/content/version change.

**Derived from:**
- `artifacts/upgrade-migration-and-compatibility-contract.md`
- `technical-requirements.md`: Migration and compatibility requirements

**Depends on:**
- SD14-E1
- SD14-E3
- SD-12 update/rollback truth

### Feature seed SD14-F7 — Compatibility verdict engine
**Outcome:** A saved character is classified explicitly as safe, migratable, read-only, or blocked before editability is claimed.

**Acceptance signals:**
- compatibility vectors are evaluated before normal reopen
- unsupported or withdrawn conditions remain visible

**Notes:**
- later handoff must name exact vector sources and exact blocked/read-only outcomes

### Feature seed SD14-F8 — Migration execution and lineage recording
**Outcome:** When migration is allowed, it records what changed and preserves prior lineage instead of mutating invisibly.

**Acceptance signals:**
- migrated revisions are auditable
- pre-migration and post-migration state remain classifiable

**Notes:**
- no handoff may claim migration safety without explicit rollback/recovery evidence

## Epic SD14-E5 — Corrupt, incompatible, and missing-dependency diagnostics
**Objective:** Implement the diagnostic surface that keeps saved-state failure modes visible to users, operators, and SD-11 evidence capture.

**Derived from:**
- `artifacts/corrupt-incompatible-and-missing-dependency-diagnostics.md`
- `technical-requirements.md`: Failure, corruption, and missing-dependency requirements

**Depends on:**
- SD14-E2
- SD14-E4
- SD-11 evidence-capture posture

### Feature seed SD14-F9 — Diagnostic classification and read-only posture
**Outcome:** Saved-state failures remain classifiable and can surface blocked versus read-only outcomes honestly.

**Acceptance signals:**
- corrupt/incomplete/missing-dependency/incompatible states are distinguishable
- read-only inspection is explicit when offered

**Notes:**
- later handoff must preserve structured evidence, not only human-readable strings

### Feature seed SD14-F10 — Evidence and issue-capture coupling
**Outcome:** Tester issue/report flows can cite saved-state identity, revision, and failure class without inventing them locally.

**Acceptance signals:**
- evidence payloads can reference save identity/revision/failure class
- operator triage can distinguish update failure from saved-state incompatibility

**Notes:**
- this is coupling, not ownership transfer; SD-11 still owns issue-flow UX contracts

## Epic SD14-E6 — Supported progression continuity over time
**Objective:** Preserve meaningful continuation of supported bounded character state as the rules/content surface evolves.

**Derived from:**
- SD-14 README: Objective / In Scope / Minimum persistence truths
- `technical-requirements.md`: Progression continuity requirements

**Depends on:**
- SD14-E1
- SD14-E2
- SD14-E4

### Feature seed SD14-F11 — Supported progression reopen contract
**Outcome:** A character can continue across ordinary bounded edits and supported progression without becoming disposable demo residue.

**Acceptance signals:**
- bounded supported progression changes remain reopenable and attributable
- saved-state continuity remains separate from breadth expansion authority

**Notes:**
- this slice must not smuggle in broader class/race/level expansion that belongs elsewhere

### Feature seed SD14-F12 — Drift visibility across rules/content evolution
**Outcome:** When supported rules/content changes alter recomputed outcomes, the drift becomes visible and classifiable rather than silently absorbed.

**Acceptance signals:**
- recomputation deltas can be surfaced
- prior authoritative intent remains inspectable

**Notes:**
- later handoff must coordinate with the canonical model/content authority rather than inventing hidden compatibility rules locally

## Initial sequencing
1. SD14-E1 — Saved-character identity, artifact classes, and local-store boundary
2. SD14-E2 — Save/load/reopen and lifecycle operations
3. SD14-E3 — Revision, autosave, and interrupted-write recovery
4. SD14-E4 — Compatibility, migration, and upgrade-safe survival
5. SD14-E5 — Corrupt, incompatible, and missing-dependency diagnostics
6. SD14-E6 — Supported progression continuity over time

## Handoff boundary
No coding harness should act directly from this file. Each later execution slice must receive a dedicated handoff that names:
- exact repo paths
- exact allowed write scope
- exact required reads
- exact verification commands
- exact non-goals
- exact persistence/migration/recovery authority boundaries

Any derived handoff file must also receive its own artifact card on the board.

## Completion gate
- [ ] every requirement is routed to at least one epic
- [ ] every epic has a bounded objective
- [ ] no epic silently changes program doctrine
- [ ] unresolved decisions remain in `risks-and-open-questions.md`, not hidden here
