---
title: GE-00 Epic Breakdown
status: draft
scope: programs/codex/requirements/GE-00-program-governance-and-scope
artifact_type: epic-breakdown
grand_epic: ../../plans/spec-domains/GE-00-program-governance-and-scope.md
---

# GE-00 Epic Breakdown

This file decomposes GE-00 governance requirements into implementation epics and feature seeds. These are not production application features. They are the control-plane work needed before product implementation begins.

## Epic GE00-E1 — Program Doctrine and Scope Charter

**Objective:** Create the durable doctrine and scope boundary that prevents UI-first drift and unbounded migration claims.

**Derived from:** TR-00-001, TR-00-002, TR-00-003, TR-00-010

### Feature seeds

#### GE00-E1-F1 — Program mission statement

Write a concise mission statement that defines Codex as a rules platform with a modern character-builder UI, not a PCGen port.

Acceptance:

- Mission names the rules platform objective.
- Mission states PCGen is oracle, not architecture.
- Mission rejects UI-only framing.

#### GE00-E1-F2 — Scope boundary charter

Publish in-scope, out-of-scope, and deferred categories for the initial roadmap.

Acceptance:

- Scope can reject full Pathfinder and full PCGen parity claims.
- Scope allows the pilot slice and migration-control work.
- Scope requires a decision record for expansion.

#### GE00-E1-F3 — Non-negotiables section

Capture the non-negotiables that later source STCs must inherit.

Acceptance:

- Headless core first.
- Conversion matrix as control plane.
- No unsupported-token silence.
- Vertical slice before breadth.
- Explainability as product behavior.

## Epic GE00-E2 — Documentation Control-Plane Setup

**Objective:** Establish the source STC lifecycle from research through implementation features.

**Derived from:** TR-00-004, TR-00-005, TR-00-006, TR-00-011

### Feature seeds

#### GE00-E2-F1 — Source STC template

Define the minimum source STC structure used by all spec domains.

Acceptance:

- Template includes README, technical requirements, acceptance and verification checks, risks/open questions, and epic breakdown.
- Template includes required questions every source STC must answer.

#### GE00-E2-F2 — Parent requirements index

Maintain a parent requirements README that links to each derived source STC.

Acceptance:

- `programs/codex/requirements/README.md` links to GE-00.
- Future source STC additions can be appended without changing the taxonomy.

#### GE00-E2-F3 — Bidirectional authority links

Ensure derived source STCs link upward to sources and higher-order docs link downward where appropriate.

Acceptance:

- GE-00 source STC links to spec domain, roadmap, and research.
- Parent requirements index links down to source STC.

#### GE00-E2-F4 — Implementation block marker

Define the rule that implementation work is blocked until source STCs exist.

Acceptance:

- Technical requirements state the block.
- Acceptance and verification checks include a block check.
- Future GE-01/GE-02 source STCs can inherit the block rule.

## Epic GE00-E3 — Decision Record Scaffold

**Objective:** Create the mechanism for durable program-affecting decisions.

**Derived from:** TR-00-009

### Feature seeds

#### GE00-E3-F1 — Decision record schema

Define minimum decision record fields.

Acceptance:

- Fields include title, status, date, scope, related spec domain, owners, context, decision, consequences, supersedes, superseded_by, and links.

#### GE00-E3-F2 — Decision record placement decision

Resolve the location for PCGen decision records.

Recommended path:

```text
programs/codex/doctrine/decisions/
```

Acceptance:

- Location is chosen or explicitly deferred.
- If chosen, add README or template in that directory.

#### GE00-E3-F3 — Decision capture triggers

List decisions that require ADR-style capture.

Acceptance:

- Stack commitments.
- Pilot boundary changes.
- Canonical model strategy.
- Import/conversion policy.
- Oracle-validation policy.
- Unsupported-token treatment.
- Expansion beyond pilot.

## Epic GE00-E4 — Pilot Slice Charter

**Objective:** Name and bound the first product proof target before technical design expands.

**Derived from:** TR-00-007

### Feature seeds

#### GE00-E4-F1 — Pilot-slice statement

Publish the pilot slice as PF1 Core Rulebook Human Fighter level 1.

Acceptance:

- Statement includes source domain and character path.
- Statement rejects broad Pathfinder support as the first proof.

#### GE00-E4-F2 — Pilot coverage checklist

List the architecture surfaces touched by the pilot slice.

Acceptance:

- PCC loading.
- LST object parsing.
- race/class/feat/skill/equipment handling.
- formulas and prerequisites.
- derived stats and saving throws.
- source lineage and diagnostics.
- oracle comparison.
- minimal UI workflow.

#### GE00-E4-F3 — Pilot non-expansion rule

Define how pilot-scope changes are approved.

Acceptance:

- Expansion requires decision record.
- Expansion must identify which downstream spec domain owns the added work.

## Epic GE00-E5 — Quality Gate Policy

**Objective:** Seed the quality gates that later source STCs must turn into concrete tests.

**Derived from:** TR-00-008, TR-00-010, TR-00-012

### Feature seeds

#### GE00-E5-F1 — Unsupported-token policy

Define explicit treatment for unsupported, lossy, approximated, or ignored PCGen semantics.

Acceptance:

- Policy requires conversion matrix or ledger entry.
- Policy treats silence as failure.

#### GE00-E5-F2 — Evidence-backed compatibility claims

Define how compatibility claims must be proven.

Acceptance:

- Claims name supported scope.
- Claims link to validation evidence.
- Claims avoid full-PCGen parity unless evidence exists.

#### GE00-E5-F3 — Explainability gate

Require explanation trails for derived values and invalid choices.

Acceptance:

- Later rules-engine requirements include explanation output.
- UI requirements treat explanations as primary product behavior.

#### GE00-E5-F4 — Headless-before-UI gate

Require CLI/test-backed domain behavior before broad UI implementation.

Acceptance:

- Later implementation plans cannot use UI screenshots as proof of domain correctness.
- Rules/import/oracle behavior must have testable headless evidence.

## Initial sequencing

Recommended order:

1. Accept this GE-00 source STC.
2. Create GE-01 conversion matrix source STC.
3. Create GE-02 canonical rules model source STC.
4. Only then allow bounded implementation spikes or repo scaffolding.

## Completion gate

GE-00 epic decomposition is complete when each future work item can be routed into one of:

- doctrine/scope charter
- documentation control plane
- decision record scaffold
- pilot-slice charter
- quality gate policy
- later spec domain source STC

Anything that cannot be routed is either out of scope or requires a decision record.
