---
title: GE-09 Coverage Dashboard Requirements
stc_id: STC-CODEX-GE-09
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts
source_stc: ../README.md
related:
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - ../../GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
  - ../../../doctrine/quality-gate-policy.md
---

# GE-09 Coverage Dashboard Requirements

## Purpose
Define the governed field contract for any future GE-09 dashboard, CSV, or evidence ledger so package, token-family, evidence-tier, compatibility-ceiling, and known-gap posture remain legible without turning reporting into a substitute for proof.

## Evidence basis for this planning pass
The field contract is grounded in the current evidence surfaces, not in hypothetical future automation:
- GE-01 conversion matrix currently provides 29 pilot-bounded rows of token-family and construct coverage truth.
- GE-01 unsupported-token ledger currently provides 13 tracked unresolved or deferred entries with explicit severities and owners.
- GE-05 defines the authoritative known-gap classes and required ledger/reporting behavior.
- The quality-gate policy defines the evidence-gate stack and compatibility claim tiers (`Observed`, `Parsed`, `Converted`, `Computed`, `Oracle-checked`, `Product-visible`).
- GE-06 currently caps the integrated pilot claim ceiling at `computed-but-not-oracle-checked`.
- GE-08 is planning-ready but does not yet authorize stronger authored-package or contribution claims.

## Governing design rules
- The dashboard is an evidence index, not a release decision engine and not a substitute for the underlying artifacts.
- Unknown, blocked, deferred, or downgraded states must be visible as first-class values rather than inferred from missing rows.
- Every row must link to concrete evidence artifacts or doctrine surfaces; status-only rows are invalid.
- The dashboard must support both expansion selection and claim refusal by making ceilings, blockers, and missing proof visible.
- Aggregated summaries may exist, but the canonical data model must remain inspectable at row level.

## Required row scopes
A future implementation may render different views, but the underlying ledger MUST support at least these row scopes:

| Row scope | Purpose | Minimum granularity rule |
|---|---|---|
| `token-family` | Track migration or rules coverage for a distinct construct family. | One row per governed token family or construct cluster that can carry its own evidence ceiling. |
| `package-scope` | Track a source package, book, or bounded content domain. | One row per package or adjacent-domain band that could be named in a compatibility or expansion claim. |
| `candidate-band` | Track ranked expansion or scope-deepening candidate bands. | One row per GE-09 candidate band when the band spans multiple token families or packages. |
| `known-gap-cluster` | Surface grouped unresolved debt that caps claims across multiple rows. | One row per reusable blocker cluster when the same unresolved mechanics or parity debt affects multiple scopes. |

A single UI may collapse these views, but the stored model must preserve which scope class each row represents.

## Required fields
Every dashboard row MUST include the following fields.

### 1. Identity and scope fields

| Field | Required meaning | Notes |
|---|---|---|
| `row_id` | Stable unique identifier for the row. | Must survive sorting or UI changes. |
| `row_scope` | One of `token-family`, `package-scope`, `candidate-band`, `known-gap-cluster`. | Required for downstream interpretation. |
| `scope_key` | Canonical short key for the governed scope. | Example shapes: `fighter-progression-formulas`, `core-rulebook-adjacent-domain-cluster`, `human-race-trait-composition`. |
| `scope_label` | Human-readable scope name. | Must be explicit enough to appear in a report or review packet. |
| `source_package_or_book` | The exact package, book, or domain source being discussed. | Multi-source rows must name the aggregation rule. |
| `token_family_or_domain` | The token family, mechanics cluster, or content domain represented. | Must not collapse unrelated mechanics into a vague bucket. |
| `in_pilot_scope` | Whether the row is inside the current GE-06 pilot boundary. | Boolean or equivalent explicit classification. |
| `scope_classification` | One of `pilot-core`, `pilot-adjacent`, `scope-deepening`, `scope-broadening`, `authoring-linked`, `future-release-only`. | Prevents stabilization work from being mislabeled as expansion. |

### 2. Evidence and claim fields

| Field | Required meaning | Notes |
|---|---|---|
| `evidence_tier_ceiling` | Highest quality-gate claim tier actually proven for this row. | Must reuse doctrine tiers, not invent new ones. |
| `claim_ceiling_phrase` | Human-readable compatibility ceiling for the row. | Example: `computed but not oracle-checked for pilot-bounded Fighter progression`. |
| `gate_posture` | Current posture across documentation, import, rules, oracle, and UI truth gates. | May be structured or summarized, but must preserve failing gates. |
| `latest_verification_artifact` | Most recent artifact proving the current ceiling. | Must be a path or immutable reference, not free text alone. |
| `evidence_date` | Date of the latest verification artifact. | Required for drift review. |
| `verification_basis_class` | One of `matrix-row`, `unsupported-ledger`, `parity-artifact`, `decision-record`, `manual-review`, `multi-source`. | Makes evidence provenance legible. |
| `compatibility_language_ceiling` | Strongest allowed external wording for this row. | Must become narrower when evidence is weaker. |
| `downgrade_required` | Whether the row is currently under downgrade pressure relative to prior claims. | Must not require the reader to infer this from comments. |

### 3. Known-gap and regression fields

| Field | Required meaning | Notes |
|---|---|---|
| `known_gap_count` | Count of known gaps affecting the row. | Zero must be explicit, not implied. |
| `highest_gap_severity` | Highest active severity posture affecting the row. | Must align with GE-01/GE-05 governance surfaces. |
| `blocking_gap_classes` | Gap classes from GE-05 that currently block a stronger claim. | Use exact GE-05 class names where applicable. |
| `accepted_gap_classes` | Gap classes visible but currently tolerated within the named claim boundary. | Cannot silently broaden the claim. |
| `regression_state` | One of `none-known`, `suspected`, `confirmed`, `downgraded`, `blocked`. | Makes regression posture first-class. |
| `regression_artifact` | Evidence reference for the newest regression finding, if any. | Empty only when `regression_state` is `none-known`. |
| `block_condition_summary` | Compact statement of what must remain false before a stronger claim is allowed. | Example: `blocked until oracle parity exists for scoped outputs`. |

### 4. Ownership and action fields

| Field | Required meaning | Notes |
|---|---|---|
| `owning_surface` | Primary owning GE, subsystem, or doctrine surface. | Must identify who can actually change the posture. |
| `downstream_owner` | The next work lane or authority surface expected to act. | Can name a future GE slice rather than a person. |
| `next_honest_move` | The smallest truthful next action for the row. | Example: `maintain hold`, `expand matrix coverage`, `run parity comparison`, `mint narrower authoring policy`. |
| `review_trigger` | Named event that forces re-review. | A row without this is governance theater. |
| `review_status` | One of `current`, `needs-review`, `blocked`, `deferred`, `superseded`. | Must remain explicit even in aggregate views. |
| `last_reviewed_at` | Date the row was last materially reviewed. | Required for drift detection. |

## Required derived classifications
A future implementation MUST support at least the following derived classifications so humans can filter without rewriting doctrine:
- by evidence tier ceiling
- by compatibility-language ceiling
- by scope classification (`pilot-core`, `pilot-adjacent`, `scope-deepening`, `scope-broadening`, `authoring-linked`, `future-release-only`)
- by owning surface (GE-01, GE-05, GE-06, GE-08, GE-09, doctrine)
- by regression state
- by block posture (`unblocked`, `known-gap-limited`, `downgrade-required`, `blocked`)
- by candidate rank or hold status for expansion review

## Review-trigger contract
Each row MUST name at least one event-driven review trigger. A future dashboard implementation must support these trigger classes at minimum:

| Trigger class | Required effect |
|---|---|
| New GE-05 parity artifact changes a gap class, evidence tier, or comparison outcome. | Recompute the row's evidence ceiling and compatibility-language ceiling. |
| GE-06 propagated posture changes. | Re-evaluate whether scope-deepening or scope-broadening remains blocked. |
| GE-01 matrix coverage expands or unsupported-ledger posture changes. | Re-rank candidate bands and refresh package/token-family counts. |
| A regression artifact lands for a previously claimed scope. | Force downgrade-or-block review before the prior claim survives. |
| A doctrine decision records intentional divergence or authority change. | Update claim wording, gap posture, and downstream obligations. |
| GE-08 readiness or contribution posture changes. | Re-evaluate authored-package and contribution-linked rows. |
| Calendar drift threshold passes with no event trigger. | Require stale-row review; this is fallback only, never the primary governance model. |

## Minimum truth rules for aggregated views
If a future UI shows summary cards, charts, or color bands, those summaries MUST preserve these truths:
- the number of rows at each evidence ceiling
- which rows are blocked by known gaps or regressions
- which rows are merely `Computed` and not `Oracle-checked`
- which rows fall outside pilot scope and are therefore not eligible for broad compatibility language
- which rows represent scope-deepening stabilization versus true scope broadening

Any summary that can turn `computed-but-not-oracle-checked` into an implied green state is invalid.

## Implementation guardrails for future builders
- CSV, SQLite, Markdown ledger, and UI dashboard implementations are all acceptable as long as they preserve the required fields and doctrine.
- Missing evidence links, missing review triggers, or implied zero-gap rows must fail validation.
- Automated rollups may summarize but must never delete row-level provenance.
- GE-09 reporting surfaces must consume GE-05 known-gap truth and GE-01 evidence truth; they may not fork either model.

## Completion rule
This requirement artifact is complete for the planning pass when a future builder can implement a governed dashboard or evidence ledger without inventing the row classes, field meanings, compatibility ceilings, or review triggers that control GE-09 truth.