---
title: GE-06 Pilot Stack Viability Decision Criteria
stc_id: STC-CODEX-GE-06
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts
source_stc: ../README.md
related:
  - ../../../doctrine/quality-gate-policy.md
  - ../../GE-05-oracle-validation-and-parity-harness/technical-requirements.md
---

# GE-06 Pilot Stack Viability Decision Criteria

## Purpose
Define how GE-06 decides whether the current stack survives the first integrated pilot path or exposes a fatal flaw.

## Decision question
The review question is not “does the demo look promising?”

The review question is:

> Can the current architecture carry one bounded PF1 Human Fighter level 1 path across import, canonical representation, computation, explanation, selected oracle comparison, and product-visible UI without hiding the reasons it succeeds or fails?

## Required evidence by layer
| Layer | Minimum evidence for viability review |
|---|---|
| Documentation gate | GE-06 source STC and required output artifacts exist and are internally linked. |
| Import fidelity gate | Selected pilot surfaces reach at least `Converted` with provenance and diagnostics, or are blocked explicitly. |
| Rules correctness gate | Selected pilot outputs reach at least `Computed` with explanations and diagnostics. |
| Oracle parity gate | Selected comparison dimensions reach `Oracle-checked` or are explicitly recorded as known gaps with stated consequence. |
| UI truth gate | Selected UI surfaces are `Product-visible` over real outputs, explanations, and diagnostics rather than mocks. |

## Acceptable viability posture
The pilot may be called **viable** only when all of the following are true:
- selected import surfaces required by the first case are at least `Converted`
- selected rules outputs required by the first case are at least `Computed`
- selected comparison targets required for viability are `Oracle-checked` or explicitly accepted as known-gap-limited with rationale
- the UI surface required for the pilot is `Product-visible` over real outputs
- remaining gaps are bounded, named, and do not erase the pilot's ability to prove the architecture

## Fatal-flaw triggers
GE-06 should classify the architecture as **fatally flawed for the current pilot shape** when any of the following occurs:
- the pilot requires semantic objects or relationships that GE-02 cannot represent without collapsing back into raw LST syntax
- critical token families cannot be imported, computed, or at least diagnosed honestly for the selected pilot path
- required derived values cannot produce explanation/provenance trails even when the numbers appear plausible
- selected UI truth depends on mock state or hardcoded examples rather than real outputs
- no meaningful selected parity dimensions can be bounded even after the oracle route has been grounded

## Narrowing triggers
GE-06 should recommend **narrow the pilot, do not abandon it** when:
- one optional output category blocks progress but the core import/compute/explain path is otherwise sound
- the export-summary boundary proves broader than required and can be reduced without invalidating the pilot
- one equipment or choice path adds disproportionate complexity while a narrower charter-compliant path still tests the architecture honestly

Any narrowing that changes the pilot claim must be reflected in the charter or review artifact explicitly.

## Upstream-expansion triggers
GE-06 should recommend **expand upstream requirements before resuming** when:
- the pilot depends on missing GE-02 canonical homes
- the pilot depends on GE-03 token families not yet grounded for the selected case
- the pilot depends on GE-04 computation/explanation surfaces that remain under-specified
- the pilot depends on GE-05 comparison doctrine that is too vague for the selected outputs
- the pilot depends on UI architecture decisions that properly belong to a GE-07 source STC

## Failure classification rule
Every blocked or failed pilot dimension MUST identify one primary owner:
- model flaw
- importer flaw
- engine flaw
- oracle gap
- UI gap

A dimension may list contributing owners, but it MUST still name one primary owner so follow-up work narrows.

## Recommended outcome classes
| Outcome | Meaning |
|---|---|
| `blocked-documentary` | The GE-06 package or required upstream documentation is still insufficient. |
| `headless-blocked` | The import/compute/explain path is not yet runnable or not yet grounded. |
| `computed-but-not-oracle-checked` | Headless behavior exists, but required selected parity evidence is still missing. |
| `oracle-checked-but-not-product-visible` | Selected parity evidence exists, but the UI truth gate is still unmet. |
| `pilot-viable` | The selected slice reaches the required evidence tiers and remaining gaps are bounded. |
| `fatal-flaw` | The selected slice exposes a structural failure that invalidates the current pilot shape. |

## Final rule
GE-06 is not trying to create optimism. It is trying to produce a survivable verdict.
