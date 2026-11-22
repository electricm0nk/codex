# SD-14 Character Lifecycle, Persistence, and Upgrade-Safe Revision Technical Design

## Purpose
This design operationalizes the SD-14 source STC by defining how saved-character identity, authoritative payloads, revision/autosave, compatibility checks, and failure diagnostics should cooperate without turning the desktop shell or update transport into persistence authority.

## Design posture
- architecture style: `local-first saved-character control plane`
- authority posture: `authoritative-input-first, derived-state-subordinate`
- migration posture: `explicit and refusal-first`
- diagnostics posture: `strict`

## Context and constraints
- the current live repo already has deterministic character-input and computation proof surfaces, but no accepted character save/load subsystem
- the repo also has adjacent authored-package persistence for GE-08, which proves durable local artifact patterns without proving character-state continuity
- the tester workbench already carries evidence-capture and update-status posture through SD-11 and SD-12, so saved-state failures must plug into those surfaces instead of inventing a parallel folklore channel
- upgrade and rollback mechanics belong to SD-12, but whether a character save survives those mechanics honestly belongs to SD-14
- the first truthful saved-character lane must remain local-first and single-user without inviting campaign-management sprawl

## Executable boundary truth as of 2026-06-30
- `src/rules_core/character_input.rs` defines the bounded authoritative input substrate the future saved-character lane must preserve rather than reinterpret
- `src/homebrew_authoring/package_store.rs` already demonstrates deterministic local persistence, load, diff, and export behavior for GE-08 authored packages, but only for authored package artifacts
- `apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` already exposes lifecycle-gate vocabulary such as `saveAllowed`, `exportAllowed`, and `diffMode`, but only for GE-08 authored-package workflow truth
- `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` already composes a tester-facing workbench model and evidence surfaces, but no dedicated saved-character boundary feeds it today
- therefore the persistence problem is not “invent whether continuity matters”; it is “define the saved-character seam before implementation improvises incompatible local files, migrations, or faux durability claims”

## Proposed system shape
SD-14 defines six cooperating surfaces:
1. a **saved-character catalog surface** that can enumerate bounded local saved characters and their high-level state
2. a **saved-character authoritative envelope** that stores identity, revision, provenance, and the user-authored payload needed to reconstruct supported character truth
3. a **derived snapshot surface** that may cache recomputable summaries for UX/performance but never outranks authoritative saved intent
4. a **lifecycle coordinator surface** that owns create/open/save/duplicate/archive/delete semantics and authoritative-save transitions
5. a **compatibility and migration surface** that classifies reopen as safe, migratable, read-only, or blocked
6. a **diagnostic and evidence surface** that makes corruption, missing dependencies, incompatible versions, and recovery posture visible to users and to tester evidence capture

## Data flow
1. a user creates or opens a bounded local character and the system assigns or retrieves stable character identity plus revision identity
2. the authoritative envelope records user-authored choices, version/provenance vectors, and lifecycle metadata
3. when the UI needs derived summaries, the engine computes them from the authoritative payload and may optionally emit a cache/snapshot surface
4. on save, the lifecycle coordinator promotes one revision to the latest authoritative state and records any autosave/recovery sidecar as subordinate evidence
5. on reopen after app/content/version changes, the compatibility surface evaluates the saved vectors and classifies the outcome before editability is claimed
6. on corruption or incompatibility, the diagnostic surface exposes what failed, whether inspection remains possible, and what evidence/recovery paths exist

## Component boundaries

### Saved-character catalog surface
- responsibilities:
  - list bounded local saved characters
  - expose stable identity, human label, last-authoritative-save, and high-level compatibility state
  - keep archive/deleted posture explicit
- inputs:
  - saved-character envelopes
  - lifecycle metadata
  - compatibility classifications
- outputs:
  - local character index / summary surface
- must not own:
  - derived rules computation
  - silent migration decisions

### Saved-character authoritative envelope
- responsibilities:
  - preserve stable character identity and revision identity
  - persist user-authored choices and the minimum provenance/version vectors required for honest reopen
  - record whether a revision is authoritative, autosaved, archived, or recovery-only
- inputs:
  - character-domain authoritative state
  - lifecycle metadata
  - compatibility vectors
- outputs:
  - durable saved-character artifact(s)
- must not own:
  - UI-only embellishments as sole source of truth
  - uncontrolled duplication of recomputable derived values

### Derived snapshot surface
- responsibilities:
  - cache or materialize recomputable summaries when helpful
  - preserve enough linkage back to the authoritative revision to detect drift
- inputs:
  - authoritative envelope
  - current supported engine/content behavior
- outputs:
  - cached read models or snapshot summaries
- must not own:
  - canonical identity or user-authored choice state
  - migration decisions by itself

### Lifecycle coordinator surface
- responsibilities:
  - orchestrate create/open/save/save-new-revision/duplicate/archive/delete flows
  - track dirty state and authoritative-save transitions
  - manage autosave and interrupted-write recovery posture
- inputs:
  - authoritative envelopes
  - runtime edit session
  - recovery state
- outputs:
  - updated revision lineage and user-visible lifecycle state
- must not own:
  - rules computation semantics
  - release/update transport

### Compatibility and migration surface
- responsibilities:
  - classify reopen as safe, migrate, read-only, or blocked
  - evaluate schema/app/content/provenance vectors before editability is claimed
  - preserve explicit recovery posture when migration is unsafe
- inputs:
  - saved-character envelope metadata
  - current runtime/content compatibility rules
  - SD-12 adjacent rollback/update state when relevant
- outputs:
  - compatibility verdict and migration plan/outcome
- must not own:
  - silent mutation with no recorded decision
  - policy-free heuristics hidden in UI code

### Diagnostic and evidence surface
- responsibilities:
  - render corruption, incompatibility, missing dependency, and recovery outcomes visibly
  - package enough structured evidence for SD-11 issue capture and operator triage
- inputs:
  - lifecycle failures
  - compatibility failures
  - recovery outcomes
- outputs:
  - tester-facing messages
  - machine-readable evidence handles
- must not own:
  - authority to suppress failures for polish
  - fake success when evidence is missing

## Data and schema notes
- minimum logical entities:
  - character identity
  - revision identity
  - authoritative character payload
  - provenance / dependency vector
  - derived snapshot or cache marker
  - compatibility classification
  - recovery artifact or record
- minimum version/compatibility vectors:
  - persisted schema version
  - app/runtime version or build identity
  - content/rules provenance handle(s)
  - optional migration lineage / prior revision reference
- lossiness/disposition model:
  - when a field is recomputable, the system may omit or invalidate stale caches rather than pretending they are authoritative
  - when a dependency is missing or unsupported, the system must preserve the save and surface the gap rather than dropping the unknown portion silently
  - when migration cannot complete safely, reopen may remain blocked or read-only

## External dependencies and references
- `../GE-06-pilot-vertical-slice-pf1-human-fighter/README.md` — bounded character-domain truth
- `../GE-10-demo-proof-and-onboarding/README.md` — current desktop proof posture
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — tester-workbench and evidence authority
- `../SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` — distribution/update/rollback authority
- `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs` — current authoritative input seam
- `/home/ubuntu/workspace/repos/codex/src/homebrew_authoring/package_store.rs` — adjacent deterministic persistence pattern

## Design decisions already fixed
- the lane is local-first and single-user
- authoritative user-authored state outranks recomputable derived state
- persistence and migration are product-truth obligations, not convenience features
- update success does not imply saved-state safety; saved-state survival is a separate proof burden
- corruption, incompatibility, and missing-dependency outcomes must stay visible and evidence-bearing

## Deferred design decisions
- exact physical storage format (directory bundle, single-file envelope, embedded database, or hybrid)
- exact number/depth of autosave or backup revisions required in the first executable slice
- exact UI affordances for archive/library management
- exact migration implementation technology and repair workflow
- exact way future broader coverage changes saved-character compatibility surfaces

## Failure modes and observability
- a save claims success without durable authoritative revision identity
- reopen succeeds but silently drops unsupported or missing-dependent state
- a migration path mutates state with no recorded compatibility verdict
- autosave or crash recovery replaces a newer authoritative revision silently
- a saved character appears current even though its dependency or app/schema vectors are incompatible
- tester evidence cannot distinguish corruption, incompatibility, missing dependency, or operator-withdrawn build posture

Required observable signals:
- current revision identity and authoritative-save status remain visible
- dirty/unsaved state remains visible
- compatibility verdict remains classifiable before editability is claimed
- corrupted/incompatible/missing-dependency saves remain attributable
- recovery artifacts remain distinguishable from authoritative revisions

## Verification implications
`acceptance-and-verification.md` must prove that this packet defines explicit contracts for saved-character identity, lifecycle operations, revision/autosave, compatibility/migration, and diagnostic posture. It must also prove the packet does not counterfeit an already-implemented character persistence subsystem while the live repo still only proves GE-06 character-domain truth and GE-08 authored-package persistence. Later execution handoffs must prove real roundtrip save/load, reopen-after-relaunch, migration classification, and recovery behavior without inventing scope or hiding failure states.

## Change constraints
- do not let later handoffs treat the desktop shell as persistence authority by burying semantics in UI-local state
- do not let later handoffs serialize only derived snapshots and call that saved-character truth
- do not let later handoffs conflate SD-12 update transport with SD-14 saved-state survival
- do not let later handoffs broaden this lane into roster/campaign/cloud scope under the cover of “lifecycle”
