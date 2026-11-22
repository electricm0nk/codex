---
title: GE06-E3-F3 Viability Evidence Bundle
artifact_type: evidence-bundle
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_route_surface: ../execution-handoff.md
selected_slice: GE06-E3-F3 — Viability evidence bundle
workflow_route: review
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
source_artifacts:
  - ./ge06-e2-f3-merge-receipt-2026-06-21.md
  - ./ge06-e3-f1-merge-receipt-2026-06-22.md
  - ./ge06-e3-f2-merge-receipt-2026-06-22.md
related_artifacts:
  - ./ge06-post-e3-fan-in-handoff-rack-2026-06-22.md
  - ./ge06-e4-f1-launch-posture-2026-06-22.md
  - ./pilot-stack-viability-decision-criteria.md
  - ../../../doctrine/quality-gate-policy.md
---

# GE06-E3-F3 Viability Evidence Bundle

## Verdict
GE06-E3-F3 now exists as the explicit E3 fan-in evidence surface.

For the accepted deterministic PF1 Human Fighter level-1 pilot, the merged GE06-E3-F1 adapter and merged GE06-E3-F2 classifier together prove all nine selected pilot dimensions are currently grounded at a `Computed` claim-tier floor with machine-checkable new-system evidence.

The current blocker for those dimensions is not engine ambiguity and not narration. It is an explicit `OracleGap`: the deterministic headless receipt computes, the selected-dimension carrier exists, but no selected old-vs-new comparison artifact has yet promoted those dimensions to `Oracle-checked`.

This bundle does not declare the pilot viable. It prepares the explicit evidence surface that the downstream GE-06 viability / domain-confidence decision must consume alongside the already-recorded E4 launch posture.

## Observed repo anchor and verification
Grounded on 2026-06-22 in `/home/ubuntu/workspace/repos/codex`:

```text
origin/develop: b2f2154
recent merges:
  - b2f2154 Merge pull request #15 from electricm0nk/ge06-e3-f1-selected-parity-dimensions
  - 5e1f68f Merge pull request #14 from electricm0nk/ge06-e3-f2-failure-classifier
verification commands:
  - "$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet -> pass (1 passed)
  - "$HOME/.cargo/bin/cargo" test --test ge06_failure_classifier --quiet -> pass (5 passed)
  - "$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet -> pass (2 passed)
  - "$HOME/.cargo/bin/cargo" test --quiet -> pass
```

## Source surfaces consumed
| Surface | What it contributes |
|---|---|
| `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md` | The computed headless receipt floor: deterministic pilot identity, computed outputs, explanations, and blocked-vs-computed receipt posture. |
| `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md` | The selected parity-dimension carrier over exactly nine mandatory pilot dimensions with an explicit `Computed` claim-tier floor. |
| `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md` | The primary-owner classifier that resolves the current computed receipt posture to `OracleGap` and the blocked receipt posture to `EngineFlaw`. |
| `artifacts/ge06-e4-f1-launch-posture-2026-06-22.md` | The already-recorded UI-side posture and remaining non-blocker gates for any future E4 spike or UI-facing handoff. |
| `artifacts/pilot-stack-viability-decision-criteria.md` | The downstream decision rubric for `computed-but-not-oracle-checked`, `oracle-checked-but-not-product-visible`, `pilot-viable`, and `fatal-flaw` outcomes. |
| `../../../doctrine/quality-gate-policy.md` | The compatibility claim-tier ladder: `Observed -> Parsed -> Converted -> Computed -> Oracle-checked -> Product-visible`. |

## Selected-dimension evidence table
The selected-dimension carrier established by GE06-E3-F1 emits exactly these nine mandatory pilot dimensions (`src/oracle_validation/selected_parity_dimensions.rs:42-126`; `tests/ge06_selected_parity_dimensions.rs:31-94`).

| Dimension | Current evidence | Claim tier now | Evidence source | Primary owner if promoted today | Blocking reason / next gate |
|---|---|---|---|---|---|
| `character.identity` | `case_id=pf1-crb-human-fighter-level1`, `source_package_id=pf1.core_rulebook` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:48-56`; `tests/ge06_selected_parity_dimensions.rs:61-64` | `OracleGap` | Identity is present in the selected-dimension carrier, but no selected oracle-comparison artifact exists yet for promotion to `Oracle-checked`. |
| `combat.baseline_melee_attack_bonus` | `5` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:58-64`; `tests/ge06_selected_parity_dimensions.rs:67-68` | `OracleGap` | Computed new-system value exists, but no GE-05 comparison artifact yet states old-vs-new parity. |
| `defense.baseline_armor_class` | `17` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:66-72`; `tests/ge06_selected_parity_dimensions.rs:70-71` | `OracleGap` | Computed new-system value exists, but no selected oracle evidence exists yet. |
| `defense.total_save.fortitude` | `4` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:74-80`; `tests/ge06_selected_parity_dimensions.rs:73-74` | `OracleGap` | Computed new-system value exists, but parity evidence is still absent. |
| `defense.total_save.reflex` | `2` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:82-88`; `tests/ge06_selected_parity_dimensions.rs:76-77` | `OracleGap` | Computed new-system value exists, but parity evidence is still absent. |
| `defense.total_save.will` | `1` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:90-96`; `tests/ge06_selected_parity_dimensions.rs:79-80` | `OracleGap` | Computed new-system value exists, but parity evidence is still absent. |
| `skill.selected_modifier.climb` | `5` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:98-104`; `tests/ge06_selected_parity_dimensions.rs:82-83` | `OracleGap` | Computed new-system value exists, but parity evidence is still absent. |
| `skill.selected_modifier.intimidate` | `3` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:106-112`; `tests/ge06_selected_parity_dimensions.rs:85-86` | `OracleGap` | Computed new-system value exists, but parity evidence is still absent. |
| `skill.selected_modifier.swim` | `5` | `Computed` | `src/oracle_validation/selected_parity_dimensions.rs:114-120`; `tests/ge06_selected_parity_dimensions.rs:88-89` | `OracleGap` | Computed new-system value exists, but parity evidence is still absent. |

## Failure-owner evidence from GE06-E3-F2
The failure classifier established by GE06-E3-F2 proves the current fan-in bundle has an explicit owner vocabulary and does not collapse into a fake `IntegrationIssue` sink (`src/rules_core/pilot_failure.rs:13-66`; `tests/ge06_failure_classifier.rs:29-149`).

| Receipt posture | Evidence | Primary owner | Why it matters for viability review |
|---|---|---|---|
| Supported deterministic pilot receipt | `HeadlessReceiptStatus::Computed` with no claim-blocking diagnostics (`tests/ge06_pilot_headless_receipt.rs:41-107`) | `OracleGap` (`tests/ge06_failure_classifier.rs:61-81`) | The current pilot is not blocked because computation failed. It is blocked because comparison evidence is still missing. |
| Mutated blocked pilot receipt (`class:fighter:1` -> `class:rogue:1`) | `HeadlessReceiptStatus::Blocked` with claim-blocking diagnostics (`tests/ge06_pilot_headless_receipt.rs:109-143`) | `EngineFlaw` (`tests/ge06_failure_classifier.rs:85-123`) | The classifier also proves that if the headless path itself stops computing, the failure owner narrows to the engine rather than dissolving into narration. |

Required vocabulary present but not currently exercised by the accepted deterministic receipt:

```text
ModelFlaw
ImporterFlaw
EngineFlaw
OracleGap
UiGap
```

That vocabulary remains part of the stable review contract even though the current supported pilot path only exercises `OracleGap`, and the mutated blocked example exercises `EngineFlaw`.

## Layer-status snapshot for downstream GE-06 review
| Layer | Current status from this bundle | Grounded evidence | What remains outside this bundle |
|---|---|---|---|
| Documentation gate | pass | GE-06 source STC, route surface, merge receipts, and this E3 bundle all exist as concrete artifacts. | The downstream viability decision still has to publish an explicit verdict artifact. |
| Import fidelity gate | inherited upstream, not re-adjudicated here | The E3 bundle depends on the merged GE06-E2-F3 receipt and its preserved pilot identity/source-package truth. | The downstream viability review still decides whether the inherited import evidence is sufficient for the exact verdict language. |
| Rules correctness gate | pass at `Computed` for the selected dimensions | The deterministic receipt computes the selected outputs; the E3-F1 adapter surfaces them directly; targeted and full tests pass. | The downstream viability review still decides whether any unresolved engine-side gaps are fatal, narrowable, or acceptable. |
| Oracle parity gate | blocked at `OracleGap` | The selected-dimension carrier exists, and the failure classifier explicitly resolves the current computed posture to `OracleGap`. | No GE-05 comparison artifact yet promotes any selected dimension to `Oracle-checked`. |
| UI truth gate | not yet `Product-visible` | `artifacts/ge06-e4-f1-launch-posture-2026-06-22.md` already records the truthful UI-side pre-viability posture and remaining grounding gates. | No live E4 handoff or product-visible UI evidence exists yet. |

## Current outcome class against the GE-06 viability rubric
Against `artifacts/pilot-stack-viability-decision-criteria.md`, the current E3 evidence posture most truthfully supports:

```text
computed-but-not-oracle-checked
```

Why:
- selected outputs required for the first deterministic pilot now have machine-checkable `Computed` evidence
- the primary blocker is explicit and named as `OracleGap`
- no selected dimension in this bundle is yet `Oracle-checked`
- no selected UI surface is yet `Product-visible`

This is stronger than vague "integration incomplete" language and weaker than a viability verdict. That is the correct level of truth for GE06-E3-F3.

## Prepared downstream inputs
The later GE-06 viability / domain-confidence decision should read, at minimum:

1. `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md`
2. `artifacts/ge06-e4-f1-launch-posture-2026-06-22.md`
3. `artifacts/pilot-stack-viability-decision-criteria.md`
4. `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md`
5. `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md`

That downstream review should answer whether the current architecture is:
- still merely `computed-but-not-oracle-checked`
- narrowable but still directionally survivable
- blocked by missing UI proof only
- or already exposing a fatal architectural flaw

This bundle does not decide that question. It removes the excuse to answer it from vibes.

## Completion rule
GE06-E3-F3 is complete only if the fan-in bundle states all of the following without invention:

- the exact selected pilot dimensions and their current new-system evidence
- the current claim-tier floor for each dimension
- the current primary owner for the deterministic supported posture
- the explicit blocking reason that prevents promotion to a stronger claim tier
- the distinct blocked-path owner example proving the classifier does not collapse into narration
- the exact downstream artifacts that must consume this bundle next
