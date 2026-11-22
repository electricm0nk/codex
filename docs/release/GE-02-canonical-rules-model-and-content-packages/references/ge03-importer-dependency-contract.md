---
title: GE-02 to GE-03 Importer Dependency Contract
stc_id: STC-CODEX-GE-02
artifact_type: reference
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages/references
source_stc: ../README.md
downstream_stc: ../../GE-03-pcgen-import-pipeline-and-provenance/README.md
source_artifacts:
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
  - ../artifacts/canonical-model-specification.md
  - ../artifacts/content-package-layout-specification.md
  - ../artifacts/pilot-object-examples.yaml
  - ../artifacts/provenance-source-map-specification.md
  - ../artifacts/expression-language-decision-criteria.md
  - ../artifacts/compiled-ir-boundary-definition.md
  - ../artifacts/content-validation-and-diagnostics-specification.md
reviewed_at: 2026-06-19
---

# GE-02 to GE-03 Importer Dependency Contract

## Purpose
Record exactly what the accepted GE-02 source STC gives to GE-03 importer planning, and what it does **not** give. This prevents GE-03 from inventing canonical targets, treating GE-02 documentary artifacts as final production schemas, or bypassing GE-01's unsupported-token and provenance obligations.

## Governing premise from GE-03
GE-03 defines the importer as a compatibility bridge from PCGen PCC/LST files into the canonical rules model. Its declared inputs include:

- GE-01 conversion matrix and pilot corpus inventory
- GE-02 canonical model specification
- PCGen source corpus
- GE-00 no-unsupported-token-silence doctrine

Therefore GE-02 must present an importer-facing dependency contract strong enough for GE-03 to target the canonical model without letting the importer become the model owner.

## GE-01 pilot evidence GE-02 passes through

| Upstream surface | Verified content | Importer implication |
|---|---:|---|
| `pilot-corpus-inventory.csv` | 66 rows | GE-03 parser targets must begin from the governed PF1 Core Rulebook Human Fighter pilot files, package root, include edges, and object-class posture rather than a broad PCGen crawl. |
| `pilot-token-taxonomy.csv` | 26 rows | GE-03 token registry planning must preserve pilot-critical token families and downstream owner labels. |
| `conversion-matrix.csv` | 29 rows | GE-03 conversion handlers must target the GE-02 canonical concept homes named by the matrix or emit explicit unresolved diagnostics. |
| `unsupported-token-ledger.csv` | 13 rows | GE-03 unsupported-token reports must keep high-risk prerequisite, formula, choice, trait, proficiency, and source-span debt visible. |
| `oracle-surface-inventory.md` | grounded + candidate surfaces | GE-03 may cite oracle surfaces for source-truth and later comparison pressure, but must not claim runtime parity. |

## GE-02 artifact contract for GE-03

| GE-02 artifact | What GE-03 may rely on | What GE-03 must not assume |
|---|---|---|
| `artifacts/canonical-model-specification.md` | Canonical homes for `SourcePackage`, `StableId`, `Race`, `RaceTrait`, `Class`, `ClassFeature`, `Feat`, `Skill`, `Equipment`, `Proficiency`, `AbilityScore`, `Save`, `Effect`, `Prerequisite`, `Formula`, `ChoiceSet`, `Selector`, `Diagnostic`, `ProvenanceRecord` / `SourceMapEntry`, and `CompiledRuntimeIR`. | Final production schema syntax, final object serialization, or full Pathfinder coverage. |
| `artifacts/content-package-layout-specification.md` | Package identity, manifest, object sections, rule semantics, provenance, diagnostics, and validation-section expectations that importer output must respect. | Repo-local file layout, package registry behavior, or implementation-specific directory names. |
| `artifacts/pilot-object-examples.yaml` | Documentary skeleton examples showing how the PF1 Human Fighter pilot can be projected into canonical model terms. | Accepted production YAML schema, exhaustive fixtures, or engine-evaluable data. |
| `artifacts/provenance-source-map-specification.md` | Required lineage/source-map fields, diagnostic linkage, source-span downgrade policy, and oracle linkage posture. | That token-level source spans are already implemented or that parity can be claimed without GE-05 evidence. |
| `artifacts/expression-language-decision-criteria.md` | Qualities required of later formula/prerequisite expression choices, including determinism, inspectability, diagnostics, and provenance. | A chosen evaluator or permission for GE-03 to invent one inside parser/importer work. |
| `artifacts/compiled-ir-boundary-definition.md` | The authority split between source packages and derived compiled runtime IR/cache. | Runtime engine semantics, cache serialization, or GE-04 implementation authority. |
| `artifacts/content-validation-and-diagnostics-specification.md` | Validation classes and diagnostic classes that GE-03 conversion reports and unsupported-token reports must feed. | That validation CLI/API shape or report serialization is final. |

## Importer-facing requirements imposed by GE-02

GE-03 importer planning must obey these requirements:

1. Parser output must preserve enough source identity for GE-02 provenance fields: package, include chain where known, file path, entry name, line/token span or explicit downgrade, and legacy construct.
2. Conversion handlers must write toward GE-02 model homes rather than raw LST-token bags.
3. Unsupported, lossy, deferred, or unresolved behavior must produce diagnostics linked to source-map records and GE-01 ledger/matrix posture.
4. Formula, prerequisite, and choice-set constructs must remain structured or explicitly deferred; prose-only placeholders are insufficient.
5. Importer reports must be machine-readable and human-auditable enough to expose canonical target, support disposition, lossiness, and diagnostic references.
6. Importer work must not redefine GE-02 canonical model homes locally; materially new homes must propagate back to GE-02 or a superseding decision surface.
7. GE-03 implementation handoffs must stay narrower than the spec domain and must name exact repo paths, branch/worktree policy, write scope, and verification commands before code work begins.

## Remaining boundaries

This contract does not make GE-02 or GE-03 `codex-ready`. It only closes the planning dependency between the accepted GE-02 artifact set and GE-03 importer requirements.

Still unresolved for future handoffs:

- final stable-ID syntax
- final production schema serialization
- expression/evaluator implementation choice
- runtime engine behavior and compiled IR implementation details
- exact importer source-span precision threshold for the first code milestone
- branch/worktree policy, allowed repo write scope, and verification commands for any coding run

## Closure judgment
GE-02 now explicitly supplies GE-03 with an importer-facing dependency contract grounded in GE-01 evidence and GE-02 generated artifacts. GE-03 may consume the contract as planning authority, but it must still derive a bounded implementation handoff before any parser, converter, report, source-map, or diagnostics code is written.
