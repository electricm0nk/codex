---
title: GE-01 Epic Breakdown
stc_id: STC-CODEX-GE-01
artifact_type: epic-breakdown
status: active
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix
source_stc: ./README.md
last_reviewed_at: 2026-06-19
---

# GE-01 Epic Breakdown

This file decomposes the Codex GE-01 source STC into bounded downstream epics and feature seeds. These are implementation-facing planning units, not execution prompts.

## Epic GE01-E1 — Pilot Corpus Inventory
**Objective:** Identify and classify the minimum PCC and LST surfaces needed for the PF1 Core Rulebook Human Fighter level 1 pilot.

**Derived from:**
- TR-01-002
- TR-01-003
- TR-01-004
- technical-design.md: Corpus Inventory

### Feature seeds

#### GE01-E1-F1 — PCC entry-surface inventory
**Outcome:** A documentary inventory of the pilot PCC entry file and its include edges.

**Acceptance signals:**
- `core_rulebook.pcc` is represented as a grounded reference surface.
- include/composition edges are captured without claiming more precision than evidence supports.

**Notes:**
- Do not broaden into non-pilot books unless the inventory explicitly marks them adjacent or deferred.

#### GE01-E1-F2 — Pilot LST file inventory
**Outcome:** A list of pilot-relevant LST files for race, class, feats, skills, equipment, formulas, and metadata.

**Acceptance signals:**
- required vs adjacent vs unresolved distinctions exist.
- each file record names evidence status.

#### GE01-E1-F3 — Inventory evidence policy
**Outcome:** A repeatable rule for recording grounded vs inferred inventory facts.

**Acceptance signals:**
- the inventory model distinguishes grounded evidence from unresolved questions.
- unknowns remain visible.

## Epic GE01-E2 — Token Taxonomy and Semantic Risk Map
**Objective:** Classify pilot token families and semantic constructs so later conversion work knows what matters first and what is risky.

**Derived from:**
- TR-01-005
- TR-01-007
- technical-design.md: Token Taxonomy

### Feature seeds

#### GE01-E2-F1 — Pilot-critical token-family list
**Outcome:** A first-pass taxonomy for token families that matter to Human Fighter level 1.

**Acceptance signals:**
- token families carry pilot criticality.
- high-risk semantics such as prerequisites, bonuses, formulas, and choices are not flattened away.

#### GE01-E2-F2 — Semantic risk tagging
**Outcome:** Each taxonomy entry records ambiguity or semantic risk.

**Acceptance signals:**
- risky constructs are visible before implementation.
- downstream owner is named for deferred families.

## Epic GE01-E3 — Conversion Matrix and Unsupported-Token Ledger
**Objective:** Define the control-plane artifacts that make coverage and non-coverage reviewable.

**Derived from:**
- TR-01-006
- TR-01-007
- TR-01-008
- TR-01-011
- technical-design.md: Conversion Matrix
- technical-design.md: Unsupported-Token Ledger

### Feature seeds

#### GE01-E3-F1 — Matrix row schema
**Outcome:** A required row shape for mapping legacy constructs to Codex target concepts.

**Acceptance signals:**
- row fields include disposition, lossiness, provenance, validation, and owner.
- the row shape can block vague “supported” claims.

#### GE01-E3-F2 — Ledger row schema
**Outcome:** A required row shape for unsupported, deferred, or intentionally ignored behavior.

**Acceptance signals:**
- each unresolved construct can be named, explained, and routed.
- silence is structurally disallowed.

#### GE01-E3-F3 — Coverage reporting rule
**Outcome:** A rule for summarizing matrix and ledger state without overstating support.

**Acceptance signals:**
- coverage summaries cannot erase unsupported or lossy rows.
- summary language remains evidence-backed.

## Epic GE01-E4 — Provenance Contract and Import Preconditions
**Objective:** Define the lineage guarantees downstream import work must preserve before translation claims are allowed.

**Derived from:**
- TR-01-009
- technical-design.md: Provenance Contract

### Feature seeds

#### GE01-E4-F1 — Minimum provenance fields
**Outcome:** A required lineage field set from PCC include through conversion rule outcome.

**Acceptance signals:**
- file/include/source details are explicit.
- downgrade behavior is named if token spans are unavailable initially.

#### GE01-E4-F2 — Provenance degradation policy
**Outcome:** A rule that partial provenance must be admitted explicitly rather than silently accepted as complete.

**Acceptance signals:**
- partial provenance is visible to reviewers.
- no fake precision appears in downstream reports.

## Epic GE01-E5 — Oracle Surface Discovery
**Objective:** Determine which legacy PCGen surfaces can later serve as trusted comparison or parity inputs.

**Derived from:**
- TR-01-010
- technical-design.md: Oracle Surface Catalog

### Feature seeds

#### GE01-E5-F1 — Oracle entry-surface catalog
**Outcome:** A catalog of commands, export paths, or runtime surfaces usable for validation.

**Acceptance signals:**
- each entry names automation level, evidence type, and limitations.
- unknown surfaces remain open questions rather than assumptions.

#### GE01-E5-F2 — Oracle trust policy
**Outcome:** A rule for describing trust and limitations of each oracle surface.

**Acceptance signals:**
- GUI-bound and headless surfaces are distinguished.
- downstream validation work knows what cannot yet be automated.

## Recommended sequencing (dependency order, not exclusive scope)
1. GE01-E1 — Pilot Corpus Inventory
2. GE01-E2 — Token Taxonomy and Semantic Risk Map
3. GE01-E3 — Conversion Matrix and Unsupported-Token Ledger
4. GE01-E4 — Provenance Contract and Import Preconditions
5. GE01-E5 — Oracle Surface Discovery

GE-01 is fulfilled by eventually executing all of these downstream epics. This ordering is dependency guidance, not permission to stop after GE01-E1.

## Handoff boundary
No coding harness should receive this file as an execution prompt.

Before a derived code-authorizing implementation handoff is allowed, the following must be true:
- planning-only review bridges may exist earlier only if they remain documentary and explicitly non-authorizing
- the local Codex checkout exists and its workdir/branch/write-scope facts are grounded
- the specific downstream epic or feature seed covered by that handoff is chosen
- the handoff names only the bounded slice being implemented, even though GE-01 as a spec domain is expected to drive all listed downstream epics over time
- unresolved questions remain referenced back to `risks-and-open-questions.md`

## Completion gate
- [ ] every GE-01 requirement is routed to at least one downstream epic
- [ ] every epic has a bounded objective
- [ ] unresolved questions remain in `risks-and-open-questions.md`
- [ ] no epic silently authorizes broad Pathfinder or full-PCGen scope
- [ ] the decomposition remains upstream of execution handoff


## Closure Addendum — 2026-06-19

GE-01's documentary deliverables are accepted for the PF1 Core Rulebook Human Fighter level 1 pilot boundary. Downstream epics may now consume the inventory, taxonomy, matrix, ledger, and oracle surfaces as governed inputs.

The next natural requirements move is GE-02 canonical rules-domain modeling. Coding remains blocked until a later coding-route source STC and execution handoff are produced.
