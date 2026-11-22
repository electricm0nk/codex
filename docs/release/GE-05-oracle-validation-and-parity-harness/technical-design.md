---
title: GE-05 Technical Design
stc_id: STC-CODEX-GE-05
artifact_type: technical-design
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness
source_stc: ./README.md
source_artifacts:
  - ./README.md
  - ./technical-requirements.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
  - ../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - ../GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - ../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
---

# GE-05 Technical Design

## Purpose
Describe the intended architecture of the Codex oracle-validation and parity-harness boundary without converting this planning STC into an implementation handoff.

The design separates four concerns that lesser approaches would collapse:

1. PCGen oracle output capture.
2. Codex new-system output capture.
3. Controlled normalization and comparison.
4. Compatibility claim governance.

## Design posture
GE-05 is a comparison and evidence subsystem. It is not the rules engine, not the importer, not the UI, and not a license to reimplement PCGen internals.

The design target is a headless harness that can eventually run from tests or CLI commands and produce reviewable evidence. GUI driving remains a last-resort oracle capture route, not the default design.

## Conceptual pipeline

```text
Golden case fixture
  ├─ old-system route -> PCGen oracle output capture -> PCGen output normalizer
  ├─ new-system route -> Codex engine/import output capture -> Codex output normalizer
  └─ case metadata / known-gap references

Normalized outputs
  -> comparator
  -> actionable diff records
  -> parity report
  -> claim-tier update or blocked-known-gap state
```

The harness must keep raw and normalized values distinguishable. A normalized pass does not erase the underlying source evidence.

## Boundary model

| Boundary | Owner | GE-05 design obligation |
|---|---|---|
| Legacy corpus and candidate oracle surfaces | GE-01 | Consume the oracle-surface inventory and require further discovery for runtime output. |
| Import/provenance bridge | GE-03 | Require new-system output to preserve provenance, source maps, conversion diagnostics, and unsupported-token status. |
| Rules computation and explanation | GE-04 | Require new-system output to expose derived values, explanations, diagnostics, and golden fixture shape. |
| Old-vs-new comparison | GE-05 | Define fixture binding, normalization, comparator, report, diff, known-gap, and claim-tier behavior. |
| Integrated pilot viability | GE-06 | Consume GE-05 evidence without redefining the oracle standard. |

## Component responsibilities

### Oracle route discovery
The first implementation-facing slice should discover how PCGen can produce usable pilot behavior evidence. Discovery must prefer stable, repeatable, non-GUI routes.

Candidate surfaces include validation tasks, command-line behavior, scripted character generation, export output, test fixtures, or GUI-driven output if unavoidable. Static PCC/LST source files remain source truth but are not sufficient runtime oracle evidence.

### Golden case fixture
The fixture binds old and new systems to one case. It records identity, inputs, output references, normalization rules, compared dimensions, known gaps, and claim-tier target.

The fixture should be versioned and minimal. It must remain narrow enough to finish: the PF1 Core Rulebook Human Fighter level 1 pilot, not broad Pathfinder coverage.

### PCGen output capture
The old-system capture component records raw PCGen output and the route that produced it. It must preserve enough context for audit: command or export route, repo/build identity when known, input case, warnings/errors, and output artifact path.

### Codex output capture
The new-system capture component records Codex output that GE-03 and GE-04 make comparable: imported content summaries, character input echo, derived values, choices/prerequisites, diagnostics, provenance/source-map links, and explanation references.

### Normalizers
Normalizers translate old and new output into comparable field sets. They may handle formatting, labels, ordering, units, and structural differences, but they must not hide semantic disagreement.

Each normalization rule must be explicit and auditable. Raw source evidence must remain linked.

### Comparator
The comparator evaluates normalized dimensions. It produces pass/fail/blocked/known-gap statuses per dimension and emits actionable diffs for failures.

The comparator should classify likely ownership where possible: oracle capture, importer/provenance, canonical model, rules engine, normalization, fixture definition, or known gap.

### Parity report writer
The report writer is the evidence artifact. It records case, sources, dimensions, old/new values or references, normalization, result, diffs, diagnostics, known gaps, and claim tier.

The report must be strict enough for machine checks and legible enough for a human review.

### Known-gap ledger/policy
Known gaps prevent unsupported or non-comparable behavior from disappearing. The policy decides when a comparison is blocked, accepted as intentionally divergent, deferred, or routed to a decision record.

## Claim-control design

GE-05 inherits the compatibility claim tiers from the quality-gate policy:

```text
Observed -> Parsed -> Converted -> Computed -> Oracle-checked -> Product-visible
```

GE-05 controls only the promotion from `Computed` to `Oracle-checked`. Promotion requires a reproducible comparison artifact for the exact behavior under claim.

A comparison report can produce one of these claim states:

- `oracle-checked-pass` — scoped output matched under declared normalization.
- `oracle-checked-fail` — scoped output differed and produced actionable diffs.
- `blocked-known-gap` — scoped output could not be compared and the reason is recorded.
- `intentionally-divergent` — PCGen behavior is known but not preserved, backed by a decision record.
- `insufficient-evidence` — old or new output route is missing or untrusted.

## Initial pilot case design
The first case should inherit the pilot charter target:

```text
pf1-crb-human-fighter-level1
```

The initial case should compare only dimensions for which old and new output can be grounded. It should be ready to record known gaps for outputs that are unavailable, legally constrained, not implemented, or non-comparable.

The first case must not become a broad regression suite. Its purpose is to prove the harness and expose fatal abstraction flaws before GE-06 integrates the full vertical slice.

## Diagnostic posture
Diagnostics are part of the comparison, not secondary log noise.

A parity report must carry or link:

- importer/conversion diagnostics from GE-03-derived outputs
- rules/validation diagnostics from GE-04-derived outputs
- oracle capture warnings or failures from PCGen
- normalization warnings
- known-gap references

A clean numeric match with hidden diagnostics is not a full pass.

## Deferred implementation choices
The following remain unresolved until a bounded discovery or implementation handoff grounds them:

- final PCGen command, validation task, export path, scripting route, or GUI route
- exact output serialization for old and new captures
- exact comparator implementation language or module placement
- exact report file extension and storage path in the implementation repo
- exact verification commands
- exact branch/worktree and write scope
- exact legal retention rule for PCGen-derived fixture outputs

## Design completion rule
The design is complete for source-STC planning when it defines the pipeline, boundaries, component responsibilities, claim-control model, and deferred choices without inventing runtime evidence.
