---
title: GE-00 Acceptance Tests
status: draft
scope: programs/codex/requirements/GE-00-program-governance-and-scope
artifact_type: acceptance-and-verification
grand_epic: ../../plans/spec-domains/GE-00-program-governance-and-scope.md
---

# GE-00 Acceptance Tests

These acceptance and verification checks prove that program governance and scope are defined well enough to unblock GE-01 and GE-02 while blocking premature implementation.

## AT-00-001 — Program identity is explicit

**Given** a reader opens this source STC  
**When** they read the README and technical requirements  
**Then** they can state that Codex is a rules platform with a modern character-builder UI, not a PCGen UI port.

Evidence:

- README contains a program doctrine statement.
- `technical-requirements.md` contains TR-00-001 and TR-00-002.

## AT-00-002 — PCGen oracle boundary is defined

**Given** the program uses legacy PCGen during migration  
**When** a later source STC describes validation or parity  
**Then** it treats PCGen as the oracle for behavior, not as the architecture template.

Evidence:

- Technical requirements state that PCGen may define expected behavior but not the target internal model.
- Later source STCs must link to this rule or explicitly supersede it by decision record.

## AT-00-003 — Scope boundaries are complete enough to reject drift

**Given** a proposed feature or plan  
**When** it is compared to the GE-00 scope boundary  
**Then** it can be classified as in scope, out of scope, deferred, or requiring a decision record.

Evidence:

- `technical-requirements.md` includes in-scope, out-of-scope, and deferred sections.
- `risks-and-open-questions.md` names known drift risks.

## AT-00-004 — Pilot slice is named and bounded

**Given** a contributor asks what the first proof target is  
**When** they read this source STC  
**Then** they find one named pilot slice and its included domains.

Required pilot slice:

```text
Pathfinder 1e Core Rulebook Human Fighter level 1
```

Minimum included domains:

- race
- class
- ability scores
- skills
- feats
- equipment
- basic combat stats
- saving throws
- source lineage
- import diagnostics
- oracle comparison
- one exportable character summary

## AT-00-005 — Unsupported-token silence is prohibited

**Given** a PCGen token, syntax form, or behavior cannot be converted exactly  
**When** importer or conversion work encounters it  
**Then** the work must record it explicitly rather than dropping it silently.

Acceptable evidence includes at least one of:

- conversion matrix row
- diagnostic output
- unsupported-token ledger row
- backlog item tied to token family and validation evidence
- decision record explaining intentional non-support

Failure condition:

- any claim of successful import while unsupported or lossy behavior is omitted from diagnostics and governance artifacts.

## AT-00-006 — Source STC template exists

**Given** a future spec domain must be converted into requirements  
**When** a source STC is created  
**Then** it follows the minimum file shape defined by GE-00.

Required shape:

```text
requirements/GE-XX-<slug>/
  README.md
  technical-requirements.md
  acceptance-and-verification.md
  risks-and-open-questions.md
  epic-breakdown.md
```

## AT-00-007 — Implementation work is blocked until requirements exist

**Given** someone proposes scaffolding the app or writing production code  
**When** the relevant source STC does not exist  
**Then** the work is blocked.

Initial block:

- App scaffolding is blocked until GE-00, GE-01, and GE-02 requirements exist or a decision record explicitly narrows the experiment into a spike.

## AT-00-008 — Documentation path is explicit

**Given** a reader wants to trace authority  
**When** they inspect the source STC frontmatter and links  
**Then** they can trace from source STC to spec domain, roadmap, and research sources.

Required upward links:

- GE-00 spec domain
- PCGen spec-domain roadmap
- PCGen port findings
- PCGen next reference architecture

## AT-00-009 — Future epics are derivable

**Given** GE-00 has future implementation work  
**When** a planner reads `epic-breakdown.md`  
**Then** they find implementation epics and feature seeds for:

- program doctrine and scope charter
- documentation control-plane setup
- decision record scaffold
- pilot-slice charter
- quality gate policy

## AT-00-010 — Exit gates pass

GE-00 is accepted only when all of the following are true:

- The team can explain why this is not a PCGen UI port.
- The pilot slice is named and bounded.
- The documentation path from spec domain to technical requirements is explicit.
- The program has a written rule against unsupported-token silence.
- Implementation work is blocked until source STCs exist.

## Manual verification checklist

Use this checklist before marking the source STC accepted:

- [ ] README has source STC purpose, contents, authority surface, doctrine, non-negotiables, and pilot slice.
- [ ] Technical requirements include the required GE-00 outputs.
- [ ] Acceptance and verification checks map to GE-00 exit gates.
- [ ] Risks and open questions identify drift and missing decisions.
- [ ] Epic breakdown contains concrete implementation epics and feature seeds.
- [ ] Parent requirements index links to this source STC.
