---
title: GE-01 Technical Requirements
stc_id: STC-CODEX-GE-01
artifact_type: technical-requirements
status: active
scope: programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix
source_stc: ./README.md
source_artifacts:
  - ../../plans/spec-domains/GE-01-legacy-corpus-and-conversion-matrix.md
  - ../../research/pcgen-port-findings-2026-06-17.md
  - ../../research/codex-reference-architecture-2026-06-17.md
last_reviewed_at: 2026-06-19
---

# GE-01 Technical Requirements

## Objective
Define the normative requirements for converting the legacy PCGen corpus needed by the first Codex pilot into an explicit migration-control surface: inventory, taxonomy, matrix, ledger, provenance, and oracle discovery.

## Normative language
- **MUST** means required for GE-01 completion.
- **SHOULD** means expected unless a later decision surface records a justified deviation.
- **MUST NOT** means prohibited for this STC.

## TR-01-001 — Legacy substrate posture
Codex MUST treat the PCGen corpus as all of the following:

- legacy corpus
- migration source material
- behavior oracle
- compatibility reference

Codex MUST NOT treat the legacy repo as:

- the new runtime architecture
- the new canonical model schema
- the new UI template
- the Codex implementation surface

## TR-01-002 — Pilot boundary for legacy discovery
GE-01 MUST bound its discovery surface to the Pathfinder 1e Core Rulebook Human Fighter level 1 pilot slice while remaining explicit about adjacent files or token families that are discovered but not pilot-critical.

The source STC MUST distinguish between:

- pilot-critical legacy artifacts
- adjacent but non-critical artifacts
- explicitly deferred artifacts
- unknown artifacts that still block later work

## TR-01-003 — Corpus inventory requirements
The source STC MUST define a pilot legacy corpus inventory model that can represent at minimum:

- PCC entry files
- included LST files
- object classes represented in those files
- include or composition edges between files
- evidence status for each inventory fact
- whether a file is required, optional, adjacent, or unresolved for the pilot

The inventory requirements MUST preserve source paths and MUST allow later enrichment with source spans or parser-derived metadata.

## TR-01-004 — Grounded reference artifacts
The STC MUST preserve the already-grounded reference artifacts that inform this work:

- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`
- `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html`
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/GenericLoader.java`
- `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/CampaignSourceEntry.java`

Later work MAY add more sources, but MUST NOT discard these grounded starting points without explanation.

## TR-01-005 — Token-family taxonomy
The STC MUST define a token-family taxonomy for the pilot slice.

For each token family or semantic class, the taxonomy MUST be able to record:

- source object class or file type
- legacy token or construct family
- human meaning
- pilot criticality (`critical`, `supporting`, `adjacent`, `deferred`)
- downstream owning epic if not handled in GE-01
- notes about ambiguity or semantic risk

The taxonomy MUST NOT collapse distinct high-risk constructs such as prerequisites, bonuses, formulas, or choice semantics into a single undifferentiated bucket.

## TR-01-006 — Conversion-matrix schema
The STC MUST define a conversion-matrix schema as a first-class artifact class.

Each row MUST be able to capture at minimum:

- legacy source path or object class
- legacy token/family
- legacy meaning
- intended Codex target concept
- support disposition
- lossiness class
- provenance requirement
- validation requirement
- downstream owner or epic
- notes / caveats

The matrix MUST be designed to block vague claims such as “Pathfinder imports” when token-level evidence does not exist.

## TR-01-007 — Disposition vocabulary
The STC MUST require a vocabulary capable of expressing all of the following states:

- exact
- partial
- unsupported
- intentionally ignored
- deferred

The STC MUST also require an explicit lossiness signal so that “partial” and “lossy” are not treated as synonyms.

A future implementation may refine the exact enums, but it MUST preserve the ability to distinguish support state from lossiness risk.

## TR-01-008 — Unsupported-token ledger
The STC MUST define an unsupported-token ledger artifact separate from prose notes and TODO comments.

A ledger row MUST be able to record at minimum:

- token family or semantic construct
- source path and source span precision available
- reason for unsupported or deferred status
- blocking severity
- recommended downstream owner
- workaround or mitigation if any
- evidence or reference link
- review status

Unsupported or lossy behavior MUST NOT disappear silently from either the matrix or the ledger.

## TR-01-009 — Provenance requirements
The STC MUST require provenance strong enough to support debugging and parity review.

The required provenance model MUST preserve, when available:

- which PCC included the artifact
- which LST file defined it
- which line or token span generated it
- which conversion rule handled it
- whether anything was dropped, approximated, deferred, or unsupported

If future tooling cannot capture token-span precision immediately, the STC MUST still require an explicit downgrade path rather than pretending the information does not matter.

## TR-01-010 — Oracle-surface discovery
The STC MUST define oracle-surface discovery requirements for the legacy PCGen runtime.

The oracle inventory MUST capture, at minimum:

- command or entry surface
- whether the surface is headless, scriptable, or GUI-bound
- expected evidence type (counts, choices, derived values, export output, diagnostics)
- prerequisites or environment needs
- limitations and trust level

The STC MUST treat unknown oracle capability as a documented question, not as permission to fabricate validation strategy.

## TR-01-011 — Evidence-backed parity prohibition
GE-01 MUST prohibit parity or import-success claims unless they are backed by explicit matrix and verification evidence.

This prohibition applies to claims about:

- package loading
- object-class coverage
- token support
- computed derived values
- exported summaries
- “Pathfinder support” in general

## TR-01-012 — Authority surface boundary
During source-STC generation, writable scope MUST remain limited to the Codex documentation authority surface.

The work MUST NOT:

- modify `/home/ubuntu/workspace/repos/pcgen`
- write implementation code into the future Codex repo
- substitute repo scaffolding for requirement truth

## TR-01-013 — Downstream routing rule
GE-01 MUST route later work into bounded downstream epics rather than smearing implementation intent across the requirements files.

At minimum, the source STC MUST decompose follow-on work into:

- corpus inventory implementation
- token taxonomy implementation
- matrix and ledger implementation
- oracle discovery / oracle runner work
- provenance-preserving import work that belongs downstream

## TR-01-014 — Produced artifacts and destination paths
GE-01 MUST produce a source-STC bundle containing all of the following concrete files:

- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`
- `references/oracle-surface-inventory.md`
- `artifacts/pilot-corpus-inventory.csv`
- `artifacts/pilot-token-taxonomy.csv`
- `artifacts/conversion-matrix.csv`
- `artifacts/unsupported-token-ledger.csv`

The CSV artifacts MUST live under `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/`.

The oracle surface inventory MUST live under `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/`.

The artifact set is not optional supporting material. It is the literal inventory/matrix/ledger deliverable surface for GE-01.

At minimum, those generated artifacts MUST support the following columns or fields:

- `artifacts/pilot-corpus-inventory.csv`
  - `artifact_kind`
  - `source_path`
  - `evidence_status`
  - `pilot_role`
  - `object_classes`
  - `include_edges`
  - `notes`

- `artifacts/pilot-token-taxonomy.csv`
  - `source_kind`
  - `token_family`
  - `meaning`
  - `pilot_criticality`
  - `downstream_owner`
  - `semantic_risk`
  - `notes`

- `artifacts/conversion-matrix.csv`
  - `source_path_or_object_class`
  - `legacy_token_family`
  - `legacy_meaning`
  - `target_codex_concept`
  - `support_disposition`
  - `lossiness_class`
  - `provenance_requirement`
  - `validation_requirement`
  - `downstream_owner`
  - `notes`

- `artifacts/unsupported-token-ledger.csv`
  - `token_family_or_construct`
  - `source_path`
  - `source_span_precision`
  - `reason`
  - `blocking_severity`
  - `recommended_owner`
  - `workaround_or_mitigation`
  - `evidence_link`
  - `review_status`

## Success definition
GE-01 succeeds when Codex has a documentary control plane strong enough to say:

- which legacy source files matter first
- which token families matter first
- how each family will be tracked in the conversion matrix
- how unsupported behavior will be surfaced instead of hidden
- how provenance will be retained
- how legacy PCGen can be consulted as an oracle later

If those questions still require invention, GE-01 is not complete.


## Closure Addendum — 2026-06-19

The GE-01 closure pass accepts the documentary deliverables for the PF1 Core Rulebook Human Fighter level 1 boundary.

Current artifact state:
- `artifacts/pilot-corpus-inventory.csv` separates pilot-critical, supporting, adjacent, and candidate surfaces.
- `artifacts/pilot-token-taxonomy.csv` records pilot-critical token families.
- `artifacts/conversion-matrix.csv` provides an explicit route for every pilot-critical token family.
- `artifacts/unsupported-token-ledger.csv` preserves deferred/lossy semantics for downstream owners.
- `references/oracle-surface-inventory.md` records discovered PCGen oracle surfaces, including the command-line/batch export path.

The unresolved design work that remains belongs downstream: GE-02 owns canonical model decisions, GE-03 owns parser/import implementation, GE-04 owns engine semantics, and GE-05 owns automated oracle validation. Those downstream obligations do not block GE-01 closure.
