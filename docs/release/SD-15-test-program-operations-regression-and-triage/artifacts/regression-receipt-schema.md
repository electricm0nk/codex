# SD-15 Regression Receipt Schema

## Purpose
Define the receipt-grade evidence contract for SD-15 so a regression claim, unsupported-path report, install/use failure, persistence failure, or status-drift finding can be reconstructed later from durable fields rather than operator memory.

## Core posture
- every material SD-15 defect state should produce a receipt, even when the path is `unsupported`, `partial`, `blocked`, or `not-yet-verified`
- the receipt records evidence; it does not by itself declare a fix, tranche closure, or repo/workspace status update
- missing provenance must remain visible as an explicit field state such as `unknown`, `not-captured`, or `not-applicable`; do not delete the field because the data is absent
- tester-supplied, auto-captured, and operator-added evidence remain separate so later readers can tell what was observed by the tester, what the product captured, and what the operator concluded afterward

## When a receipt is required
Create or update an SD-15 regression receipt when any of the following is true:
- a tester report is triaged into an SD-15 class and the claim may need later reconstruction
- a bounded regression rerun is attempted and the outcome is `reproduced`, `not-reproduced`, `blocked`, `unsupported`, `partial`, or `status-drift`
- install/use or clean-machine validation produces evidence that later closure or reconciliation work may consume
- a persistence, migration, or saved-state continuity report requires explicit SD-14 context
- an operator needs a durable bridge between intake evidence and a later reconciliation, clean-machine, or external-test artifact

## Evidence partitions
| Evidence partition | Meaning | Typical fields | Mutation rule |
|---|---|---|---|
| tester-supplied | what the tester says happened and what they expected | observed behavior, expected behavior, reproduction steps, tester goal/mission, attachment notes, redaction declaration | preserve verbatim except for explicit redaction handling |
| auto-captured | runtime facts captured by the product or controlled intake flow | build label/version, tester-facing channel/support label, platform/OS, current bounded workflow, current data-source identity, diagnostics/explanation when present | preserve exactly as captured; do not retroactively rewrite these fields to match later guesses |
| operator-added | triage and provenance interpretation added after intake | primary SD-15 class, outcome state, adjacent-authority references, package/install context, operator provenance handle, support-state note, persistence/migration note, sufficiency note | operators may append or refine, but must not blur the source boundary |
| attachment / external evidence | files or references outside the structured payload | screenshots, logs, save files, release metadata, publication handles, commit/build links | retain the handle, redaction posture, and why the evidence matters |

## Requiredness vocabulary
- `required` — must be present for every receipt in this schema
- `required when available` — if the field can be grounded from the observed system or operator records, it must be recorded; otherwise the receipt must retain an explicit absence marker
- `conditional` — required only when the named trigger condition is true

## Canonical receipt sections

### 1. Receipt identity and lineage
| Field | Source | Requiredness | Meaning / rule |
|---|---|---|---|
| `receipt_id` | operator-added | required | durable handle for this exact receipt revision |
| `intake_handle` | operator-added | required when available | issue number, ticket URL, intake payload handle, or equivalent origin reference |
| `created_at` | operator-added | required | timestamp when the receipt record was first created |
| `last_updated_at` | operator-added | required | timestamp of the latest material evidence or verdict update |
| `supersedes_receipt_id` | operator-added | conditional | required when this receipt refreshes or replaces an older receipt |
| `evidence_owner` | operator-added | required when available | operator or system responsible for the current receipt update |

### 2. Build and provenance context
| Field | Source | Requiredness | Meaning / rule |
|---|---|---|---|
| `build_label_or_version` | auto-captured | required | tester-visible build label or version under test |
| `tester_channel_support_label` | auto-captured | required | tester-facing channel and support label visible during the run |
| `operator_provenance_handle` | operator-added | required when available | operator-side branch, release record, artifact URL, publication handle, or equivalent provenance anchor |
| `commit_or_build_identity` | operator-added | required when available | commit SHA, CI build ID, release asset digest, or equivalent immutable identity |
| `publication_or_acquisition_handle` | operator-added | conditional | release page, artifact download handle, or acquisition reference when packaging/distribution context matters |
| `rollback_withdrawal_context` | operator-added | conditional | required when the claim concerns superseded, withdrawn, blocked, or recovery-preferred builds |

### 3. Platform and install context
| Field | Source | Requiredness | Meaning / rule |
|---|---|---|---|
| `platform_os` | auto-captured | required | operating system of the observed run |
| `platform_architecture` | operator-added | required when available | architecture or similar environment discriminator when known |
| `package_install_context` | operator-added | required when available | artifact type, package format, install path, or acquisition method when relevant |
| `environment_kind` | operator-added | required when available | authoring machine, clean-machine, external tester machine, VM, or equivalent bounded environment label |
| `environment_identity_handle` | operator-added | conditional | required when a clean-machine or named environment claim is being made |

### 4. Workflow and adjacent-authority context
| Field | Source | Requiredness | Meaning / rule |
|---|---|---|---|
| `bounded_workflow_under_test` | auto-captured | required | the exact tester-visible mission or workflow active when the claim arose |
| `current_data_source_identity` | auto-captured | required when available | distinguishes real command data from placeholder/fallback state when the intake surface can capture it |
| `primary_sd15_class` | operator-added | required | one primary SD-15 class from `triage-class-dictionary.md` |
| `outcome_state` | operator-added | required | `defect`, `unsupported`, `partial`, `not-yet-verified`, `blocked`, or `status-drift` |
| `adjacent_authority_references` | operator-added | required | the SD-11, SD-12, SD-13, or SD-14 surfaces used to justify the classification |
| `sd13_support_state_context` | operator-added | conditional | required when roster breadth, class/race support, progression, or unsupported semantics are involved |
| `sd14_persistence_migration_context` | operator-added | conditional | required when save/load/reopen/revise/migrate/update continuity is involved |

### 5. Claim statement
| Field | Source | Requiredness | Meaning / rule |
|---|---|---|---|
| `observed_behavior` | tester-supplied | required | what actually happened |
| `expected_behavior` | tester-supplied | required | what should have happened |
| `claim_summary` | operator-added | required when available | concise operator statement of the bounded claim being preserved |
| `attempted_goal_or_mission` | tester-supplied | required when available | what the tester was trying to accomplish when the failure or contradiction appeared |

### 6. Reproduction and diagnostics
| Field | Source | Requiredness | Meaning / rule |
|---|---|---|---|
| `reproduction_status` | operator-added | required | `reproduced`, `not-reproduced`, `not-re-run`, `blocked`, `unsupported`, `partial`, or `status-drift-confirmed` |
| `reproduction_steps_or_impossibility_note` | tester-supplied plus operator-added | required | either the reproduction path used or the explicit reason the claim could not be rerun truthfully |
| `diagnostics_or_status_evidence` | auto-captured plus operator-added | required when available | logs, UI diagnostics, blocked reasons, explanation/provenance references, or status evidence |
| `evidence_sufficiency_note` | operator-added | required | what the current receipt proves, what is still missing, and whether later work should treat it as complete, partial, or insufficient |
| `next_required_surface` | operator-added | required when available | regression rerun, install/use matrix row, clean-machine receipt, external-test artifact, reconciliation checklist, or equivalent next proof step |

### 7. Attachments and redaction posture
| Field | Source | Requiredness | Meaning / rule |
|---|---|---|---|
| `attachment_handles` | tester-supplied plus operator-added | required when available | screenshots, logs, save files, release metadata, or other evidence handles |
| `attachment_types` | operator-added | required when available | classify each attachment as screenshot, log, save file, release metadata, or equivalent |
| `redaction_posture` | tester-supplied plus operator-added | required | whether attachments were omitted, scrubbed, partially redacted, or attached as-is |
| `redaction_reason` | operator-added | conditional | required when evidence was redacted or omitted because of sensitivity |

## Minimal canonical skeleton
```yaml
receipt_id: SD15-RR-YYYYMMDD-001
intake_handle: github://owner/repo/issues/123
created_at: 2026-06-30T17:30:00Z
last_updated_at: 2026-06-30T17:42:00Z
supersedes_receipt_id: null
build_context:
  build_label_or_version: 0.4.0-alpha.3
  tester_channel_support_label: alpha / linux-first
  operator_provenance_handle: origin/develop
  commit_or_build_identity: a0a90dcdf75cd0cc09d4b71e9fb7d3b440aaf293
  publication_or_acquisition_handle: github-release://tester-build-2026-06-30
  rollback_withdrawal_context: not-applicable
platform_context:
  platform_os: ubuntu-24.04
  platform_architecture: x86_64
  package_install_context: appimage
  environment_kind: clean-machine
  environment_identity_handle: vm://tranche2-clean-machine-01
workflow_context:
  bounded_workflow_under_test: tester workbench -> load build -> open bounded mission
  current_data_source_identity: live command data
  primary_sd15_class: install/use defect
  outcome_state: defect
  adjacent_authority_references:
    - SD-12 README
    - SD-11 README
  sd13_support_state_context: not-applicable
  sd14_persistence_migration_context: not-applicable
claim_statement:
  observed_behavior: app fails before bounded mission entry
  expected_behavior: bounded workbench should launch
  claim_summary: linux tester build launches to a fatal startup error before mission entry
  attempted_goal_or_mission: open the bounded tester workbench and reach mission entry
evidence:
  reproduction_status: reproduced
  reproduction_steps_or_impossibility_note: followed tester launch path from published artifact
  diagnostics_or_status_evidence:
    - screenshot://startup-error
    - log://desktop-startup-2026-06-30
  evidence_sufficiency_note: complete for install/use triage; clean-machine receipt still needed for broader closure use
  next_required_surface: tranche-2-clean-machine-validation-report.md
attachments:
  attachment_handles:
    - screenshot://startup-error
    - log://desktop-startup-2026-06-30
  attachment_types:
    - screenshot
    - log
  redaction_posture: logs scrubbed for local paths
  redaction_reason: home-directory path removed
```

## Population rules
- if a field is `required when available` and the operator cannot ground it yet, populate an explicit absence marker such as `unknown`, `not-captured`, or `not-yet-derived` and name the gap in `evidence_sufficiency_note`
- if the same receipt is refreshed later, do not overwrite historical build identity silently; either create a superseding receipt or fill `supersedes_receipt_id`
- keep tester-supplied wording distinct from operator interpretation; summarize separately rather than rewriting the tester claim into operator prose
- when the claim touches SD-13 or SD-14, the contextual field must point at the governing support-state or persistence authority instead of embedding folklore in free text
- when attachments are absent, say whether that is because none existed, because the path could not capture them, or because they were intentionally omitted

## Section-3 requirement crosswalk
| Technical requirement from `technical-requirements.md` section 3 | Schema home |
|---|---|
| tester-visible build label or version | `build_label_or_version` |
| tester-facing channel and support label | `tester_channel_support_label` |
| operator branch/provenance handle when available | `operator_provenance_handle` |
| commit or build identity when available | `commit_or_build_identity` |
| platform and package/install context | `platform_os`, `platform_architecture`, `package_install_context` |
| active bounded workflow or mission under test | `bounded_workflow_under_test` |
| relevant SD-13 support-state context | `sd13_support_state_context` |
| relevant SD-14 persistence/migration context | `sd14_persistence_migration_context` |
| observed behavior | `observed_behavior` |
| expected behavior | `expected_behavior` |
| reproduction steps or reproduction impossibility note | `reproduction_steps_or_impossibility_note` |
| diagnostics, explanation, or status evidence | `diagnostics_or_status_evidence` |
| attachment/redaction posture | `attachment_handles`, `attachment_types`, `redaction_posture`, `redaction_reason` |
| auto-captured vs user-supplied vs operator-added split | evidence partitions plus per-field `Source` column |

## Explicit refusals
- do not treat a GitHub issue number by itself as a regression receipt
- do not collapse `not-reproduced`, `unsupported`, `partial`, or `status-drift` into a generic defect receipt without keeping the outcome state visible
- do not drop provenance fields because they are inconvenient to capture on the first pass
- do not let a later status or closure surface cite a receipt as proof if the receipt hides missing identity, redaction, or adjacent-authority context
