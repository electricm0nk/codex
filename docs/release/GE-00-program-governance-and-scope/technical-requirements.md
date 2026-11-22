---
title: GE-00 Technical Requirements
status: draft
scope: programs/codex/requirements/GE-00-program-governance-and-scope
artifact_type: technical-requirements
grand_epic: ../../plans/spec-domains/GE-00-program-governance-and-scope.md
---

# GE-00 Technical Requirements

## Objective

Establish the operating doctrine, scope boundaries, artifact flow, decision rules, pilot-slice charter, and governance scaffolds that must exist before technical implementation begins.

## Normative language

- **MUST** means required for GE-00 completion.
- **SHOULD** means expected unless a decision record explicitly explains the deviation.
- **MUST NOT** means prohibited unless GE-00 itself is revised.

## TR-00-001 — Program mission

The program MUST define Codex as:

> a rules platform with a modern character-builder UI that can ingest PCGen PCC/LST content as legacy source material while using existing PCGen behavior as an oracle during migration.

The mission MUST NOT describe the work as merely:

- a Java-to-new-stack port
- a JavaFX replacement
- a Pathfinder-only app
- a generic rules-builder product
- a prettier PCGen clone

## TR-00-002 — Product doctrine

The program MUST publish and preserve the following doctrine:

1. The old PCGen runtime is the migration oracle, not the target architecture.
2. PCC/LST semantics are the migration substrate that must be understood, converted, and validated.
3. The new target model is a canonical rules model with provenance, diagnostics, versioning, and explainability.
4. UI work is subordinate to proven headless domain behavior.
5. Unsupported or lossy conversion must produce explicit diagnostics and backlog items.
6. Broad system coverage is blocked until the pilot slice proves the architecture.

## TR-00-003 — Scope boundary document

The source STC MUST define scope boundaries for the initial roadmap.

### In scope

- Linux, Windows, and macOS desktop support.
- Local-first character building and rules browsing.
- Rust core engine and import tooling.
- Tauri desktop shell with TypeScript UI.
- Pathfinder 1e as the first proof domain.
- PCGen PCC/LST import for migration and compatibility.
- Conversion matrix and unsupported-token ledger.
- Oracle comparison against legacy PCGen where practical.
- Canonical rules model that users can eventually author directly.
- Explainable rules computation.
- Safe homebrew and package customization.

### Out of scope for the initial roadmap

- Full PCGen parity across every supported game system.
- Full Pathfinder coverage before the pilot slice succeeds.
- Cloud-first or account-required architecture.
- Multiplayer campaign management.
- VTT integration.
- Marketplace, billing, public package registry, or social sharing platform.
- Mobile apps.
- A general-purpose scripting free-for-all for rules.
- Reusing PCGen internals as the new runtime architecture.
- UI breadth beyond what is needed to validate the pilot slice.

### Explicitly deferred

- Spells beyond what the pilot requires.
- Complex class archetypes and variants beyond the pilot.
- Broad export-sheet parity.
- Deep plugin API for third-party developers.
- Package signing and trust network.
- Web app deployment.

## TR-00-004 — Documentation lifecycle

The program MUST use the following artifact flow:

```text
research -> roadmap -> spec domain -> source STC -> implementation epic -> feature/story -> repo-local implementation docs -> upstream delta/no-change review
```

The artifact flow MUST preserve links in both directions:

- higher-order artifacts link downward to derived source STCs
- derived source STCs link upward to their source research, roadmap, and spec domain

Spec domains MUST NOT be used directly as implementation prompts.

## TR-00-005 — Source STC template

Every spec domain source STC MUST use this minimum shape:

```text
requirements/GE-XX-<slug>/
  README.md
  technical-requirements.md
  acceptance-and-verification.md
  risks-and-open-questions.md
  epic-breakdown.md
```

Every source STC MUST answer:

1. What is the exact objective?
2. What is explicitly in scope?
3. What is explicitly out of scope?
4. What artifacts are produced?
5. What does success look like?
6. What test or evidence proves success?
7. What prior spec domain does it depend on?
8. What future spec domain does it unblock?

## TR-00-006 — Implementation block rule

Production implementation work MUST be blocked until the relevant source STC exists.

For GE-00, this means the program MUST NOT begin app scaffolding as the first action. The next actionable source STCs after GE-00 SHOULD be:

```text
requirements/GE-01-legacy-corpus-and-conversion-matrix/
requirements/GE-02-canonical-rules-model-and-content-packages/
```

## TR-00-007 — Pilot-slice charter

The program MUST name and bound the first pilot slice as:

> Pathfinder 1e Core Rulebook Human Fighter level 1, including race, class, ability scores, skills, feats, equipment, basic combat stats, saving throws, source lineage, import diagnostics, oracle comparison, and one exportable character summary.

The pilot slice MUST test enough of the architecture to expose real substrate risk:

- PCC loading
- LST object parsing
- race modeling
- class modeling
- feat modeling
- prerequisite evaluation
- formula evaluation
- skill handling
- equipment handling
- derived combat stats
- source lineage
- explainability
- oracle comparison
- minimal modern UI workflow

The pilot slice MUST NOT expand into broad Pathfinder support until its success criteria are met.

## TR-00-008 — Unsupported-token silence prohibition

The program MUST maintain a written rule that no unsupported PCGen token, syntax form, conversion loss, approximation, or intentionally ignored behavior may disappear silently.

Every unsupported or lossy conversion MUST produce at least one of:

- conversion matrix entry
- diagnostic output
- unsupported-token ledger row
- backlog item tied to token family and validation evidence
- decision record explaining intentional non-support

The absence of explicit handling MUST be treated as a failure, not as success.

## TR-00-009 — Decision log scaffold

The program MUST create and use a decision record pattern for program-affecting choices.

Minimum decision record fields:

```yaml
title: ADR-XXXX-short-title
status: proposed | accepted | superseded | rejected
date: YYYY-MM-DD
scope: programs/codex
related_grand_epic: GE-XX
owners: []
context: ''
decision: ''
consequences: ''
supersedes: []
superseded_by: null
links: []
```

Decision records SHOULD be created for:

- stack commitments
- pilot-slice boundary changes
- canonical model strategy
- importer/conversion policy
- oracle-validation policy
- unsupported-token treatment
- expansion beyond the pilot slice
- release/packaging governance

## TR-00-010 — Roadmap acceptance gates

The program MUST define stage and spec-domain gates before implementation work proceeds.

GE-00 gates are:

1. Team can explain why the project is not a PCGen UI port.
2. Pilot slice is named and bounded.
3. Documentation path from spec domain to technical requirements is explicit.
4. Program has a written rule against unsupported-token silence.
5. Implementation work is blocked until source STCs exist.

## TR-00-011 — Authority surface rule

Codex program artifacts MUST remain under `programs/codex/` unless they define global workspace doctrine or repo-local implementation truth.

- Program research belongs under `programs/codex/research/`.
- Program plans belong under `programs/codex/plans/`.
- Program requirements belong under `programs/codex/requirements/`.
- Repo-local implementation details belong in the future implementation repo.
- Workspace-wide filing doctrine belongs under `governance/`.

## TR-00-012 — Quality gate policy seed

GE-00 MUST establish the quality-gate stance for later source STCs:

- headless behavior before UI breadth
- fixture-backed import behavior
- oracle comparison where parity is claimed
- source provenance retained through conversion
- explicit diagnostics for unsupported behavior
- explanations for derived values and invalid choices
- expansion by evidence, not enthusiasm

## Produced artifacts

GE-00 produces:

- program doctrine statement
- scope boundary document
- source STC template
- initial pilot-slice charter
- decision log scaffold
- implementation block rule
- unsupported-token silence prohibition
- seed quality gate policy

## Success definition

GE-00 succeeds when the Codex program has enough written control to prevent false starts and to derive GE-01 and GE-02 without ambiguity.
