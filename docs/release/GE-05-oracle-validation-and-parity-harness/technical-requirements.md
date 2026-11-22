---
title: GE-05 Technical Requirements
stc_id: STC-CODEX-GE-05
artifact_type: technical-requirements
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness
source_stc: ./README.md
source_artifacts:
  - ../../plans/spec-domains/GE-05-oracle-validation-and-parity-harness.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/README.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
  - ../GE-03-pcgen-import-pipeline-and-provenance/README.md
  - ../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - ../GE-04-rules-engine-and-explainability-core/README.md
  - ../GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - ../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - ../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../doctrine/quality-gate-policy.md
---

# GE-05 Technical Requirements

## Objective
Define the normative requirements for the Codex oracle validation and parity harness: reproducible PCGen oracle invocation, golden-case fixtures, old/new output capture, normalization, comparison dimensions, actionable diffs, parity reports, known-gap handling, and evidence boundaries for compatibility claims.

## Normative language
- **MUST** means required for GE-05 completion.
- **SHOULD** means expected unless a later decision surface records a justified deviation.
- **MUST NOT** means prohibited for this STC.

## TR-05-001 — Oracle-harness posture
Codex MUST treat GE-05 as the evidence boundary between computed behavior and oracle-checked compatibility.

The GE-05 oracle harness MUST be all of the following:

- a reproducible comparison standard for bounded old-vs-new cases
- a report producer that records exact claim scope, evidence, diffs, diagnostics, and known gaps
- a downstream consumer of GE-03 provenance/import outputs and GE-04 computation/explanation outputs
- a guardrail that prevents “looks plausible” behavior from becoming a parity claim

The GE-05 oracle harness MUST NOT be treated as:

- a clone of PCGen internals
- a substitute for GE-03 import fidelity
- a substitute for GE-04 deterministic rules correctness
- proof of full Pathfinder or full PCGen compatibility
- authority to preserve undesirable legacy behavior without a decision record

## TR-05-002 — Upstream dependency truth
The GE-05 source STC MUST state these dependencies explicitly:

- GE-01 owns legacy corpus discovery and candidate oracle-surface inventory
- GE-03 owns importer, provenance, conversion-report, source-map, and unsupported-token diagnostic requirements
- GE-04 owns deterministic new-system computation, explanations, diagnostics, and golden computation fixture requirements
- GE-06 owns the integrated PF1 Human Fighter level 1 vertical-slice viability contract
- GE-00 and the quality-gate policy define that oracle parity claims require comparison evidence, not intuition

GE-05 MUST NOT treat any upstream planning artifact as proof that implementation outputs already exist.

## TR-05-003 — Oracle surface discovery and trust tiers
GE-05 MUST require a bounded discovery step that identifies the lowest-friction PCGen surface capable of producing usable pilot oracle output.

At minimum, oracle discovery MUST classify candidate surfaces into trust tiers such as:

| Tier | Meaning | Example posture |
|---|---|---|
| Static source truth | PCGen files or docs establish structure or declared semantics, not runtime behavior. | PCC/LST files and listfile docs from GE-01. |
| Runtime behavior evidence | PCGen execution, validation, export, or script output establishes old-system behavior for a case. | Future command/export output for the Human Fighter case. |
| GUI-derived evidence | Output obtained only through GUI driving. | Allowed only if no lower-friction route exists and recorded as a risk. |
| Unknown / ungrounded | Candidate surface has not been proven usable. | Current PCGen runtime character-generation path as of GE-01. |

A static source surface MAY inform expected output hypotheses, but MUST NOT by itself satisfy an oracle parity gate.

## TR-05-004 — Reproducible PCGen invocation requirements
GE-05 MUST require every old-system oracle run to record enough information to be repeated.

At minimum, the eventual oracle-runner requirements MUST capture:

- PCGen repository or build identity when available
- command, task, script, export route, or GUI route used
- working directory and relevant environment assumptions
- source package or campaign loaded
- character fixture input or save file identity
- generated output file paths or captured stdout/stderr references
- exit status and diagnostic/failure information
- limitations that affect comparability

If the old-system output requires GUI automation, the harness MUST record why headless, validation, export, or scripting routes were rejected or unavailable.

## TR-05-005 — Golden-case fixture format
GE-05 MUST define a golden-case fixture format that can bind a new-system run to a legacy PCGen oracle run.

The fixture format MUST include at minimum:

- stable case ID
- game system/source-package identity
- pilot character input dimensions
- old-system oracle output reference or production rule
- new-system output reference or production rule
- compared dimensions
- normalization declarations
- expected diagnostics and known-gap references
- claim-tier target
- provenance or source-map requirements inherited from GE-03 and GE-04

The fixture format MUST allow expected values to be supplied by captured oracle/new-system outputs later; it MUST NOT fabricate expected values during planning.

## TR-05-006 — New-system output capture contract
GE-05 MUST define what new-system output must expose before it can be compared.

At minimum, comparable new-system output SHOULD include:

- loaded source package and object-count summaries when available
- character input echo and resolved choice state
- derived value outputs for selected pilot categories
- choice availability or prerequisite results for selected pilot paths
- diagnostics from importer, validation, rules execution, and known gaps
- explanation or provenance references for values being compared

A new-system output that lacks diagnostics and provenance MUST be treated as lower-trust evidence even if its numbers match PCGen.

## TR-05-007 — PCGen output capture contract
GE-05 MUST define how legacy PCGen output is captured for comparison.

At minimum, PCGen output capture MUST preserve:

- raw output artifact path or captured command output
- source route used to produce it
- case identity and input assumptions
- fields or sections extracted for comparison
- errors, warnings, or unsupported route notes
- whether output is runtime behavior evidence or merely static/source evidence

The captured PCGen output MUST remain auditable enough that a later reviewer can distinguish oracle behavior from hand-transcribed expectation.

## TR-05-008 — Normalization boundary requirements
GE-05 MUST define normalization as a controlled comparison boundary, not a place to hide disagreement.

Normalization requirements MUST state:

- which fields are normalized before comparison
- why each normalization is allowed
- whether normalization is lossy
- how original raw values remain traceable
- which differences MUST remain visible even after normalization
- when normalization uncertainty creates a known gap instead of a pass

Normalization MUST NOT turn materially different behavior into a pass merely because the old and new systems use different labels, formatting, ordering, or omitted fields.

## TR-05-009 — Comparison dimensions
GE-05 MUST define comparison dimensions for the pilot and allow later expansion only through governed updates.

The pilot comparison dimensions SHOULD include, where practical:

- loaded content count or coverage summaries
- selected race/class/level identity
- available choice or prerequisite outcomes for at least one pilot-relevant feat/proficiency path
- derived values from the GE-04 pilot fixture categories
- diagnostics and known-gap sets
- one exportable character summary/stat-block boundary if a usable PCGen route exists

Each compared dimension MUST identify its old-system source, new-system source, normalization rule, pass/fail rule, and known-gap fallback.

## TR-05-010 — Parity report schema
GE-05 MUST define a parity report schema that can support both human review and machine checks.

At minimum, each report MUST record:

- report ID and timestamp or run identifier
- case ID and fixture version/reference
- old-system output source and new-system output source
- compared dimensions and result status
- raw and normalized old/new values or references
- actionable diff records for failures
- diagnostics and known-gap references
- compatibility claim tier achieved or blocked
- links to commands, test output, or receipts when available

A report that says only “pass” or “fail” without evidence is insufficient.

## TR-05-011 — Actionable diff requirements
Parity failures MUST produce actionable diffs.

At minimum, failure records MUST identify:

- compared dimension
- old-system value or reference
- new-system value or reference
- normalized values when normalization was applied
- delta classification
- suspected owner when known, such as importer, canonical model, rules engine, oracle capture, or normalization
- diagnostics or known gaps that influenced the result
- next investigation target or blocker

Diff output MUST be precise enough that a later implementation handoff can become narrower, not broader.

## TR-05-012 — Known-gap and non-comparable output policy
GE-05 MUST define a known-gap policy for behavior that cannot be compared or should not be preserved.

The policy MUST distinguish at least:

- oracle route unavailable
- PCGen behavior known but undesirable
- non-comparable output format or missing output surface
- unsupported or lossy import semantics
- new-system computation not implemented
- normalization ambiguity
- legal/licensing fixture limitation
- out-of-pilot-scope behavior

Non-comparable output MUST be reported as a known gap or blocked comparison, never silently omitted.

## TR-05-013 — Compatibility claim-tier promotion
GE-05 MUST inherit the quality-gate policy's claim-tier model.

A claim may move to `Oracle-checked` only when there is a reproducible comparison artifact tying the new-system output to legacy PCGen evidence for the exact scoped behavior.

GE-05 MUST NOT promote any behavior from `Computed` to `Oracle-checked` based only on GE-04 rules-engine tests, static PCGen files, or visual plausibility.

## TR-05-014 — Initial Human Fighter expected-output source requirements
GE-05 MUST produce source requirements for the initial PF1 Core Rulebook Human Fighter level 1 expected-output fixture.

Those requirements MUST identify:

- inherited pilot character dimensions from the pilot charter
- which output categories require old-vs-new comparison
- which outputs may initially be known gaps
- which GE-03 provenance and GE-04 explanation/diagnostic fields must accompany compared outputs
- what evidence upgrades the fixture from hypothesized to oracle-checked

The artifact MUST NOT fabricate final expected values before PCGen oracle output and new-system output are grounded.

## TR-05-015 — Legal and fixture-retention constraints
GE-05 MUST require fixture and report retention choices to respect legal, licensing, and provenance boundaries.

At minimum, later handoffs MUST decide whether PCGen-derived output can be stored directly, stored as reduced facts, stored as hashes/references plus extraction commands, or generated on demand.

If fixture legality or redistribution status is unclear, the uncertainty MUST appear as a known gap or blocker rather than being ignored.

## TR-05-016 — Headless and test-command documentation
GE-05 MUST prefer headless, test, CLI, validation, export, or scripting routes over GUI-driven routes.

A later implementation handoff MUST document exact commands or tests for:

- producing old-system oracle output or recording why it cannot be produced headlessly
- producing new-system output
- normalizing outputs
- comparing outputs
- writing parity reports
- failing when expected diffs, diagnostics, or known-gap records are missing

This source STC does not invent those commands before a bounded discovery/implementation pass grounds them.

## TR-05-017 — Produced artifacts
GE-05 MUST produce a source-STC bundle containing:

- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`

GE-05 MUST also produce same-epic documentary artifacts containing:

- `artifacts/oracle-strategy-specification-requirements.md`
- `artifacts/golden-case-fixture-format.md`
- `artifacts/parity-report-format.md`
- `artifacts/initial-human-fighter-l1-expected-output-source-requirements.md`
- `artifacts/known-gap-policy.md`

This package MUST live under `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/`.

## TR-05-018 — Downstream routing rule
GE-05 MUST route later implementation work into bounded downstream epics rather than treating the entire parity harness as one handoff.

At minimum, downstream decomposition MUST include:

- PCGen command/oracle discovery
- golden-case fixture schema and fixture source requirements
- new-system output contract and normalizer
- PCGen output capture and normalizer
- comparator and actionable diff reporter
- parity report writer
- known-gap ledger and decision-routing policy
- headless test/CLI integration

## Success definition
GE-05 succeeds when Codex has an oracle-validation planning surface strong enough to say:

- which evidence is required before a behavior may be called oracle-checked
- what the first Human Fighter old-vs-new comparison case must contain
- how old and new outputs are captured, normalized, compared, and reported
- how parity failures become actionable diffs
- how non-comparable or intentionally divergent behavior is preserved as known gaps or decision records
- which downstream implementation epics exist and which runtime facts remain unresolved

If those answers still require invention, GE-05 is not complete.
