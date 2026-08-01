---
title: GE-04 Pilot Golden Computation Fixture Requirements
stc_id: STC-CODEX-GE-04
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts
source_stc: ../README.md
related:
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../../plans/spec-domains/GE-06-pilot-vertical-slice-pf1-human-fighter.md
---

# GE-04 Pilot Golden Computation Fixture Requirements

## Purpose
Define the deterministic fixture requirements GE-04 needs before a future implementation can prove computed pilot rules behavior.

This artifact does not fabricate final expected values. It defines the fixture shape and proof obligations that later GE-04/GE-06/GE-05 work must ground.

## Fixture identity
The first GE-04 golden computation fixture SHOULD target:

```text
Pathfinder 1e Core Rulebook Human Fighter level 1
```

This inherits the program pilot boundary and must remain narrow unless a higher-order decision surface changes the pilot.

## Required fixture inputs
A future fixture MUST define source package identity/version, imported/canonical content snapshot or fixture package reference, race/class/level if retained by GE-06, ability score inputs, feat selections or feat candidates, skill allocations/state, equipment selections and active state, choice-set selections, provenance/source-map references, and expected diagnostics or known gaps.

## Required output categories
The fixture MUST name expected output categories before implementation handoff.

Candidate pilot categories include:

- ability modifiers
- class/level-dependent values
- base attack bonus or attack-related value under a selected equipment path
- armor/equipment-influenced value such as armor class when equipment scope is selected
- saving throws
- skill-related values needed by the pilot
- feat/proficiency prerequisite outcomes
- at least one available or unavailable choice explanation
- diagnostic and known-gap set

Exact final values must be supplied by fixture/oracle work, not invented by this STC.

## Required explanation assertions
For each tested derived value, the fixture SHOULD assert that the explanation graph contains character input nodes, canonical object/effect nodes, formula/prerequisite nodes where applicable, contribution edges, provenance/source-map links when imported content contributed, and diagnostic nodes when behavior is unsupported, unresolved, invalid, or known-gap.

For each tested failed prerequisite or unavailable choice, the fixture SHOULD assert the checked condition, expected value/state, actual observed value/state or diagnostic, and reason the option is unavailable or prerequisite failed.

## Required diagnostic assertions
The fixture MUST be able to assert diagnostics for invalid fixture input, unresolved canonical references, unsupported imported semantics affecting tested behavior, invalid or unsupported expressions, circular/unstable dependency if encountered, and provenance gaps affecting explanation or parity claim level.

## Headless verification posture
A later implementation handoff must identify a command or test path that loads the fixture, computes outputs without desktop UI, emits derived values, emits explanation graph output, emits diagnostics, and fails if expected values, explanation edges, or required diagnostics are absent.

## GE-05 boundary
This fixture may become comparison-ready for GE-05, but GE-04 cannot claim PCGen parity. GE-05 must supply the legacy PCGen output path, normalization rules, comparison dimensions, and parity report evidence.

## GE-06 boundary
GE-06 must finalize the integrated pilot character path. GE-04 may define fixture requirements, but exact selections and full end-to-end viability are GE-06 concerns.
