---
title: GE-03 Epic Breakdown
stc_id: STC-CODEX-GE-03
artifact_type: epic-breakdown
status: draft
scope: programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance
source_stc: ./README.md
---

# GE-03 Epic Breakdown

This file decomposes the Codex GE-03 source STC into bounded downstream epics and feature seeds. These are implementation-facing planning units, not execution prompts.

## Epic GE03-E1 — PCC Parser Boundary
**Objective:** Define and later implement the bounded parser slice for pilot PCC entry files and include semantics.

**Derived from:**
- TR-03-003
- TR-03-004
- technical-design.md: PCC Parser Boundary

### Feature seeds

#### GE03-E1-F1 — PCC entry-file parse shape
**Outcome:** A bounded definition of how pilot PCC entry files become structured parse records.

**Acceptance signals:**
- entry-file identity is preserved
- include relationships are represented
- parse failures are diagnosable rather than silent

**Execution-readiness status:** selected as first implementation-slice candidate and ready for `execution-handoff.md` derivation under the branch policy, allowed write scope, runnable Rust verification substrate, verification commands, and first-slice provenance threshold recorded in `artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md`.

#### GE03-E1-F2 — PCC provenance capture rule
**Outcome:** A rule for how PCC source identity flows into later source-map records.

**Acceptance signals:**
- later handlers can point back to source PCC context
- downgrade behavior is explicit if only partial precision is available

## Epic GE03-E2 — LST Parser Boundary
**Objective:** Define and later implement the bounded parser slice for pilot LST files and their structured records.

**Derived from:**
- TR-03-003
- TR-03-004
- TR-03-005
- technical-design.md: LST Parser Boundary
- technical-design.md: Structured Parse Representation

### Feature seeds

#### GE03-E2-F1 — Pilot LST record parse shape
**Outcome:** A bounded definition of how pilot-relevant LST records become structured parse results.

**Acceptance signals:**
- source file identity is preserved
- ordering/span posture is explicit
- parse and semantic stages remain separate

#### GE03-E2-F2 — Parse error and unsupported syntax policy
**Outcome:** A rule for how parse-stage unsupported syntax becomes explicit diagnostics.

**Acceptance signals:**
- parse failures do not disappear
- unsupported syntax can be escalated into later work items

## Epic GE03-E3 — Token Registry and Semantic Routing
**Objective:** Define and later implement the routing layer between structured legacy inputs and semantic conversion ownership.

**Derived from:**
- TR-03-006
- technical-design.md: Token Registry

### Feature seeds

#### GE03-E3-F1 — Registry entry shape
**Outcome:** A first-class record shape for legacy families, handler ownership, risk, and validation obligations.

**Acceptance signals:**
- registry entries distinguish meaning from handler implementation
- high-risk constructs remain visible

#### GE03-E3-F2 — Routing policy for deferred semantics
**Outcome:** A rule for how registry entries express unsupported or deferred posture before handlers exist.

**Acceptance signals:**
- no token family disappears into generic fallback language
- unresolved semantics stay reviewable

## Epic GE03-E4 — Conversion Handlers and Canonical Projection
**Objective:** Define and later implement bounded conversion-handler classes that translate structured legacy inputs into canonical target intents or explicit diagnostics.

**Derived from:**
- TR-03-007
- TR-03-013
- technical-design.md: Conversion Handlers

### Feature seeds

#### GE03-E4-F1 — Pilot token-handler classes
**Outcome:** A bounded set of initial handler classes for pilot-critical semantics.

**Acceptance signals:**
- handler classes name intended canonical targets from accepted GE-02 artifacts
- remaining GE-02-adjacent schema/runtime uncertainties remain explicit

#### GE03-E4-F2 — Lossiness and unsupported posture
**Outcome:** A rule for how exact, partial, lossy, unsupported, and intentionally ignored outcomes are represented at handler level.

**Acceptance signals:**
- coverage posture is explicit
- handler output cannot overstate support

## Epic GE03-E5 — Provenance and Source-Map Writer
**Objective:** Define and later implement the lineage layer that preserves source-to-target explanation through the importer.

**Derived from:**
- TR-03-008
- technical-design.md: Provenance / Source-Map Contract

### Feature seeds

#### GE03-E5-F1 — Minimum source-map fields
**Outcome:** A required field set from source file and span through handler identity and canonical target.

**Acceptance signals:**
- source lineage is explicit
- diagnostic outcomes are representable

#### GE03-E5-F2 — Provenance degradation policy
**Outcome:** A rule that partial provenance is admitted explicitly rather than silently treated as complete.

**Acceptance signals:**
- partial precision is visible
- no fake source accuracy appears in downstream reports

## Epic GE03-E6 — Diagnostics and Conversion Report CLI
**Objective:** Define and later implement auditable diagnostics and coverage reporting for importer outcomes.

**Derived from:**
- TR-03-009
- TR-03-010
- technical-design.md: Diagnostics / Reporting Surface

### Feature seeds

#### GE03-E6-F1 — Unsupported-token diagnostic shape
**Outcome:** A first-class diagnostic record for unsupported, deferred, or lossy behavior.

**Acceptance signals:**
- unresolved constructs can be named, sourced, and routed
- no silent drops are structurally allowed

#### GE03-E6-F2 — Conversion-report summary rule
**Outcome:** A rule for how importer coverage summaries are expressed without exaggeration.

**Acceptance signals:**
- exact/partial/unsupported/ignored posture is visible
- caveats and validation evidence remain attached to claims

## Epic GE03-E7 — Fixture and Parity Harness Preparation
**Objective:** Define and later implement the bounded verification surface for parser/conversion fixtures and eventual oracle-backed parity comparisons.

**Derived from:**
- TR-03-011
- technical-design.md: Diagnostics / Reporting Surface

### Feature seeds

#### GE03-E7-F1 — Fixture inventory rule
**Outcome:** A required shape for pilot fixtures and expected outcomes.

**Acceptance signals:**
- parse expectations are explicit
- diagnostic expectations are explicit
- provenance/report expectations are explicit

#### GE03-E7-F2 — Oracle comparison readiness rule
**Outcome:** A rule for how future PCGen comparisons are linked without pretending current automation already exists.

**Acceptance signals:**
- oracle-backed validation remains a named downstream concern
- unknown automation capability stays visible

## Recommended sequencing (dependency order, not exclusive scope)
1. GE03-E1 — PCC Parser Boundary
2. GE03-E2 — LST Parser Boundary
3. GE03-E3 — Token Registry and Semantic Routing
4. GE03-E4 — Conversion Handlers and Canonical Projection
5. GE03-E5 — Provenance and Source-Map Writer
6. GE03-E6 — Diagnostics and Conversion Report CLI
7. GE03-E7 — Fixture and Parity Harness Preparation

GE-03 is fulfilled by eventually executing all of these downstream epics. This ordering is dependency guidance, not permission to stop after GE03-E1.

## Handoff boundary
No coding harness should receive this file as an execution prompt.

Before a derived code-authorizing GE-03 implementation handoff is allowed, the following must be true:
- the GE-03 source STC has been reviewed beyond initial drafting
- the specific downstream epic or feature seed covered by that handoff is chosen
- the local Codex checkout exists and its workdir/branch/write-scope facts are grounded for that exact slice
- the execution substrate is grounded; if the handoff requires Rust, `cargo` and `rustc` must be available in the target execution runtime or the handoff must explicitly name a different prepared environment
- the handoff names only the bounded slice being implemented, even though GE-03 as a spec domain is expected to drive all listed downstream epics over time
- remaining GE-02-adjacent schema/runtime questions remain referenced back to `risks-and-open-questions.md`

## Completion gate
- [ ] every GE-03 requirement is routed to at least one downstream epic
- [ ] every epic has a bounded objective
- [ ] accepted GE-02 artifact dependencies and remaining GE-02-adjacent gaps remain visible rather than hidden
- [ ] unresolved questions remain in `risks-and-open-questions.md`
- [ ] the decomposition remains upstream of execution handoff
