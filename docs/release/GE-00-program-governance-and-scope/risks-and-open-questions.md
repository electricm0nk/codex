---
title: GE-00 Risks and Open Questions
status: draft
scope: programs/codex/requirements/GE-00-program-governance-and-scope
artifact_type: risks-and-open-questions
grand_epic: ../../plans/spec-domains/GE-00-program-governance-and-scope.md
---

# GE-00 Risks and Open Questions

## Primary risks

| ID | Risk | Consequence | Mitigation |
|---|---|---|---|
| R-00-001 | UI-first drift | The visible app improves while the real PCC/LST semantic burden remains unsolved. | Require headless core behavior and source STCs before broad UI work. |
| R-00-002 | PCGen clone gravity | The new system inherits old data/runtime assumptions instead of using PCGen as an oracle. | State doctrine that PCGen is oracle, not architecture; require decision records for deviations. |
| R-00-003 | Unsupported-token silence | Import appears successful while unsupported or lossy semantics disappear. | Mandate conversion matrix, diagnostics, unsupported-token ledger, and backlog linkage. |
| R-00-004 | Pathfinder breadth explosion | Work expands into all of Pathfinder before proving the architecture. | Freeze first proof to PF1 Core Rulebook Human Fighter level 1. |
| R-00-005 | Requirement bypass | Agents or developers implement directly from spec domains or research prose. | Treat source STCs as the gate into implementation epics/features. |
| R-00-006 | Documentation drift | Requirements, plans, and implementation docs diverge without upward propagation. | Require bidirectional links and upstream delta/no-change review. |
| R-00-007 | Over-ceremony | Every small spike is forced through full governance and progress slows. | Distinguish production implementation from bounded spikes; use decision records for intentional exceptions. |
| R-00-008 | Decision folklore | Stack and scope decisions are made in chat and not captured durably. | Use ADR-style decision records for program-affecting choices. |
| R-00-009 | Premature stack finality | Tauri/Rust/TypeScript decisions become dogma before the pilot proves them. | Treat stack as current reference architecture; confirm through pilot-slice evidence. |
| R-00-010 | Explainability delayed | The system computes results but cannot explain them. | Make explanation trails a quality gate, not a later UX enhancement. |

## Open questions

### OQ-00-001 — Decision record location

Where should PCGen decision records live?

Candidate location:

```text
programs/codex/doctrine/decisions/
```

Alternative:

```text
programs/codex/docs/decisions/
```

Recommended initial answer: use `programs/codex/doctrine/decisions/` because these decisions govern program behavior and scope, not merely document findings.

### OQ-00-002 — Spike exception rule

What is the exact rule for allowing bounded spikes before full implementation requirements exist?

Recommended answer:

A spike may proceed before full production requirements only if it has:

- explicit spike objective
- explicit non-production status
- fixed time or scope boundary
- evidence output path
- no claim of production readiness
- upstream delta/no-change review on completion

### OQ-00-003 — First GE-01 token inventory boundary

Which PCC/LST token families are required for the Human Fighter level 1 pilot slice?

Likely early families:

- PCC campaign metadata and includes
- race files
- class files
- feat/ability files
- skill files
- equipment files
- prerequisite tokens
- formula/JEP expressions
- BONUS/effect-like tokens
- source metadata tokens

GE-01 must confirm this from the live PCGen corpus.

### OQ-00-004 — Oracle comparison mechanism

How will the new system compare against legacy PCGen?

Candidate evidence paths:

- legacy datatest/inttest outputs
- scripted PCGen character creation/export
- generated stat block comparison
- loaded object count comparison
- hand-curated golden fixtures

GE-05 must decide the full harness, but GE-00 should preserve the oracle requirement.

### OQ-00-005 — Canonical model authorship format

What authoring format should canonical content use first?

Current reference architecture suggests versioned YAML/TOML/JSON package files compiled into normalized SQLite/JSON IR.

GE-02 must choose the initial authoring subset and schema discipline.

### OQ-00-006 — UI framework finality

Should the UI be React or Svelte inside Tauri?

Current reference architecture leans React + TypeScript + TanStack + Tailwind/shadcn-style components. This should remain a recommendation until the pilot UI pressure test confirms it.

### OQ-00-007 — License and content distribution boundaries

What content can be imported, transformed, shipped, or referenced in a future Codex package?

This is explicitly deferred from GE-00 implementation, but GE-09 must handle packaging and release governance before public distribution claims.

### OQ-00-008 — How broad is “PCGen compatibility”?

Does compatibility mean:

- import of selected PCC/LST source material?
- exact runtime parity for supported token families?
- export compatibility?
- round-trip compatibility?
- preservation of old homebrew behavior?

GE-00 answer: compatibility claims must be scoped and evidence-backed. Full PCGen parity is out of scope for the initial roadmap.

## Questions that are intentionally not answered by GE-00

- Complete schema design for the canonical rules model.
- Full conversion matrix content.
- Exact parser architecture.
- Complete desktop UI design.
- Public release model.
- Plugin API design.
- Broad Pathfinder coverage.

Those belong to later spec domain source STCs.

## Risk review trigger

Reopen this file when any of the following occur:

- a new source STC changes pilot scope
- implementation begins before GE-01/GE-02 requirements are accepted
- an unsupported-token handling path is disputed
- a UI-first plan is proposed
- a broad Pathfinder coverage claim appears
- a stack decision becomes blocking or controversial
