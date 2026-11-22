---
stc_id: STC-CODEX-GE-01
stc_kind: source-requirements
template_version: 2
work_type: data-collection
workflow_route: collection
readiness: collection-ready
status: active
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md
target_runtime:
  repo: /home/ubuntu/workspace
  workdir: /home/ubuntu/workspace
  branch: n/a for documentary collection route; repo-specific branch/worktree decisions remain downstream-handoff-specific
  write_scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/**
review_state: accepted
last_reviewed_at: 2026-06-19
parent_scopes:
  - programs/codex
source_artifacts:
  - programs/codex/plans/spec-domains/GE-01-legacy-corpus-and-conversion-matrix.md
  - programs/codex/requirements/GE-00-program-governance-and-scope/README.md
  - programs/codex/requirements/GE-00-program-governance-and-scope/technical-requirements.md
  - programs/codex/research/pcgen-port-findings-2026-06-17.md
  - programs/codex/research/codex-reference-architecture-2026-06-17.md
related_artifacts:
  - programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
upstream_targets:
  - programs/codex/README.md
  - programs/codex/requirements/README.md
supersedes: []
superseded_by: []
tags:
  - codex
  - legacy-corpus
  - conversion-matrix
  - provenance
  - oracle
  - pf1
---

# GE-01 — Legacy Corpus and Conversion Matrix

## Objective
Turn the legacy PCGen corpus required for the PF1 Core Rulebook Human Fighter level 1 pilot into an explicit migration map for Codex: corpus inventory, token-family taxonomy, conversion-matrix requirements, unsupported-token handling, provenance expectations, and oracle-surface discovery.

## Deliverable Type
`data-collection`

## Workflow Route
`collection`

## Readiness
`collection-ready`

Why this readiness is accurate:
- the owning Codex authority surface now exists under `programs/codex/`
- the source artifact, research basis, pilot slice, and reference PCGen corpus are all identified explicitly
- the STC names the exact GE-01 documentary artifacts and their destination paths
- the GE-01 artifact set has been created, populated from live PCGen sources, and closure-checked against the Human Fighter level 1 pilot boundary
- every pilot-critical token family in `artifacts/pilot-token-taxonomy.csv` is now explicitly routable through `artifacts/conversion-matrix.csv`; deferred/lossy semantics are captured in `artifacts/unsupported-token-ledger.csv`
- code-authorizing handoff facts remain a later concern for later code-owning work, not the current GE-01 route

## Closure State
GE-01's pilot documentary deliverables are accepted for the PF1 Core Rulebook Human Fighter level 1 slice as of the 2026-06-19 closure pass. This means GE-01 no longer blocks downstream GE-02 modeling or later GE-03 parser planning on missing inventory/taxonomy/matrix facts for the pilot slice. It does **not** authorize Codex implementation code by itself.

## Authority and Scope
- owner: `Todd Hintzmann`
- scope: `program`
- canonical: `true`
- canonical path: `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md`
- parent scopes:
  - `programs/codex`

This STC governs Codex program requirements. It is derived from legacy PCGen planning material, but the Codex STC itself is the canonical planning surface for this work going forward.

## Target Runtime
- repo: `/home/ubuntu/workspace`
- workdir: `/home/ubuntu/workspace`
- branch/worktree: `n/a for documentary collection route; repo-specific branch/worktree remains downstream-handoff-specific`
- allowed write scope: `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/**`

The STC is truthful about the current state: GE-01 is currently authorizing documentary generation inside the requirements bundle itself. Future repo-code work must still ground its own branch/worktree/write-scope facts in a later route-appropriate handoff.

## Document Map
- `technical-requirements.md` — detailed normative requirements for corpus inventory, token taxonomy, conversion matrix, unsupported-token ledger, provenance, and oracle discovery
- `technical-design.md` — architecture/design response for how Codex should structure the documentary control plane and future subsystem boundaries for this migration problem
- `acceptance-and-verification.md` — observable checks proving this STC is complete enough for downstream planning and later handoff derivation
- `risks-and-open-questions.md` — unresolved decisions, blocked facts, and anti-hallucination boundaries
- `epic-breakdown.md` — implementation-facing epics and feature seeds derived from the source STC while remaining upstream of execution handoff
- `collection-handoff.md` — bounded same-epic collection brief for continuing GE-01 documentary artifact generation
- `execution-handoff.md` — superseded misrouted bridge artifact retained only as an audit trail; not an active instruction surface
- `references/oracle-surface-inventory.md` — concrete GE-01 documentary deliverable for discovered and candidate PCGen oracle surfaces
- `artifacts/pilot-corpus-inventory.csv` — concrete GE-01 inventory deliverable for pilot-relevant source files and include edges
- `artifacts/pilot-token-taxonomy.csv` — concrete GE-01 taxonomy deliverable for pilot token families and semantic risk
- `artifacts/conversion-matrix.csv` — concrete GE-01 conversion-matrix deliverable
- `artifacts/unsupported-token-ledger.csv` — concrete GE-01 unsupported/deferred/lossy behavior deliverable

## Required Reads
- `../../plans/spec-domains/GE-01-legacy-corpus-and-conversion-matrix.md` — the strategic source artifact being normalized into a Codex source STC
- `../../requirements/GE-00-program-governance-and-scope/technical-requirements.md` — inherited governance rules, pilot slice, and implementation block doctrine
- `../../research/pcgen-port-findings-2026-06-17.md` — decisive migration posture: PCGen as oracle, not architecture
- `../../research/codex-reference-architecture-2026-06-17.md` — reference architecture for provenance, conversion matrix, and unsupported-token handling

## Conditional Reads
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc` — only if verifying pilot PCC include semantics or book-entry boundaries
- `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html` — only if token-family interpretation is ambiguous or undocumented in the source STC
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/GenericLoader.java` — only if loader behavior claims need grounding
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/CampaignSourceEntry.java` — only if campaign include semantics or source-entry behavior must be justified

## In Scope
- Codex GE-01 source-STC documents under `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/`
- legacy pilot corpus inventory requirements for PF1 Core Rulebook Human Fighter level 1
- expanding and correcting the GE-01 documentary artifact set itself
- PCC and LST source-discovery requirements relevant to the pilot slice
- token-family taxonomy requirements for pilot-critical content
- conversion-matrix schema requirements and disposition vocabulary requirements
- unsupported-token ledger requirements
- source provenance and oracle-surface discovery requirements
- epic decomposition for downstream Codex implementation planning

## Out of Scope
- writing Codex implementation code in the target repo
- modifying the legacy PCGen repo
- claiming parity, import coverage, or successful conversion before evidence exists
- final canonical rules-model design beyond what GE-01 must name for conversion targets
- UI design or product shell implementation
- full Pathfinder or full PCGen system coverage

## Acceptance Summary
The acceptance criteria in `acceptance-and-verification.md` are satisfied for the PF1 Core Rulebook Human Fighter level 1 pilot closure boundary.

Compact summary:
- the pilot legacy corpus is bounded and mapped to documentary requirements rather than vague import intent
- the GE-01 artifact set exists as real governed outputs and has been populated from live PCGen sources
- every pilot-critical token family is routable through a conversion-matrix row; unsupported or deferred behavior is visible in the ledger where relevant
- provenance and oracle-discovery requirements are explicit enough to block counterfeit import claims
- downstream work can be decomposed into bounded implementation epics without inventing missing GE-01 facts

## Allowed Assumptions
- the PF1 Core Rulebook Human Fighter level 1 pilot remains the first proof target unless a higher-order decision record changes it
- the PCGen repo at `/home/ubuntu/workspace/repos/pcgen` remains read-only reference material for Codex planning
- the future local Codex checkout is intended to live at `/home/ubuntu/workspace/repos/codex`, but execution work must re-ground that fact before coding begins

## Blockers / Forbidden Assumptions
- stop if pilot scope changes without an explicit upstream decision surface
- stop if a future execution handoff tries to invent branch, worktree, or write scope facts not grounded at handoff time
- do not assume that legacy token presence implies pilot importance
- do not assume that parser design, canonical schema, or validation commands are already settled merely because the source repo contains related code
- do not claim import readiness, token support, or parity without explicit matrix and verification evidence

## Next Stage Rule
- GE-01's own named documentary outputs are now materially complete for the PF1 Core Rulebook Human Fighter level 1 pilot closure boundary.
- Further same-epic expansion may still use `collection-handoff.md`, but it is no longer required before downstream source-STC work begins.
- The next truthful downstream move is GE-02 rules-domain/canonical-model STC work, or a review pass if Todd requests one.
- GE-01 as a spec domain is expected to drive execution across all downstream epics defined in `epic-breakdown.md`, not merely GE01-E1.
- Do not derive any code-authorizing implementation handoff until a later coding-route source STC has been reviewed, the local Codex checkout is grounded, and each proposed handoff names a bounded downstream epic or feature seed with explicit scope, non-goals, and verification.
