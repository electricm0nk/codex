# SD-14 Character Lifecycle, Persistence, and Upgrade-Safe Revision Acceptance and Verification

## Acceptance posture
This is a planning-ready documentary gate. The immediate proof burden is that the SD-14 packet defines saved-character continuity honestly and concretely enough for later execution-story minting without inventing implementation truth.

## Documentation gate checks

### Gate A — Source STC shape exists
Acceptance:
- `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md` exist
- `references/upstream-dependency-contract.md` exists
- all named same-epic output artifacts exist under `artifacts/`

Verification:
- verify the file set exists under `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/`

### Gate B — Saved-character authority is concrete
Acceptance:
- the packet explicitly distinguishes authoritative saved choices from recomputable derived state
- the packet explicitly defines lifecycle operations and compatibility vectors
- the packet explicitly defines corrupt/incompatible/missing-dependency outcomes

Verification:
- confirm the README, `technical-requirements.md`, and `artifacts/persisted-character-state-contract.md` all name the authoritative-versus-derived split
- confirm `artifacts/character-lifecycle-operations-contract.md` names create/open/save/reopen/duplicate/archive/delete
- confirm `artifacts/upgrade-migration-and-compatibility-contract.md` and `artifacts/corrupt-incompatible-and-missing-dependency-diagnostics.md` define blocked/read-only/migration/diagnostic posture

### Gate C — Adjacent authority surfaces remain separate
Acceptance:
- SD-14 does not counterfeit SD-11 tester-workbench authority
- SD-14 does not counterfeit SD-12 distribution/update authority
- SD-14 does not claim broader rules/content support than GE-06 and adjacent accepted lanes ground

Verification:
- confirm README `Authority and Scope`, `In Scope`, and `Out of Scope` preserve the GE-06 / SD-11 / SD-12 boundary split
- confirm `references/upstream-dependency-contract.md` names what each upstream surface does and does not authorize

### Gate D — Repo reality is grounded honestly
Acceptance:
- the packet cites the live repo as having adjacent GE-08 authored-package persistence but no accepted character persistence subsystem yet
- the packet does not claim runtime save/load implementation already exists

Verification:
- confirm README `Readiness`, `Closure State`, and `Target Runtime` mention GE-08 package persistence and the absence of character save/load/migration truth
- confirm `technical-design.md` `Executable boundary truth as of 2026-06-30` preserves that distinction

### Gate E — Same-epic documentary outputs are concrete
Acceptance:
- the packet names concrete output artifacts with exact paths and completion rules
- those artifacts are sufficient to seed later execution-story minting without re-inventing persistence scope

Verification:
- confirm README frontmatter `expected_output_artifacts` matches the `Expected Output Artifacts` table
- confirm each named artifact exists and carries a concrete contract rather than placeholder prose

### Gate F — Epic decomposition is ready for successor routing
Acceptance:
- every major requirement family routes into at least one bounded epic
- the epic list preserves dependencies and anti-scope-creep boundaries
- no epic is itself an execution handoff

Verification:
- confirm `epic-breakdown.md` includes bounded epics for state identity/store, lifecycle operations, revision/recovery, compatibility/migration, and diagnostic/evidence coupling
- confirm `epic-breakdown.md` ends with an explicit handoff boundary rule

## Future implementation proof obligations
A later SD-14 execution handoff is acceptable only when it names exact repo paths, exact allowed write scope, and exact commands, and proves at least the relevant subset of these obligations:

### Roundtrip continuity proof
- create a bounded supported character
- save it
- close and reopen it
- verify authoritative choices survive and recomputed derived values are explainable

### Revision and dirty-state proof
- modify a loaded character
- verify dirty/unsaved state becomes visible
- save a new authoritative revision
- verify prior recovery/autosave state does not silently replace the new authoritative revision

### Migration and compatibility proof
- load a save authored against an older or different version/content vector
- verify the system classifies it correctly as safe, migratable, read-only, or blocked
- verify migration does not silently drop authoritative state

### Failure and recovery proof
- exercise corrupt/incomplete/missing-dependency cases
- verify diagnostics remain visible
- verify evidence capture can distinguish the failure class
- verify any read-only or repair posture is explicit

### Upgrade-safety proof
- exercise a governed SD-12-adjacent build/version change
- verify saved-state survival is classified honestly
- verify update success is not mistaken for save compatibility when the vectors disagree

## Anti-counterfeit rules
- existence of local files alone does not prove lifecycle correctness
- successful app update alone does not prove saved-state survival
- caching derived values alone does not prove persistence
- absence of immediate failure does not prove migration safety
- GE-08 authored-package persistence must not be reported as character persistence success
