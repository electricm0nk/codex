---
title: GE06-E5-F1 Viability / Domain-Confidence Decision
artifact_type: viability-review
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_route_surface: ../execution-handoff.md
selected_slice: GE06-E5-F1 — Pilot viability / domain-confidence decision
workflow_route: review
status: active
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
source_artifacts:
  - ./ge06-e3-f3-viability-evidence-bundle-2026-06-22.md
  - ./ge06-e4-f1-launch-posture-2026-06-22.md
  - ./ge06-e2-f3-merge-receipt-2026-06-21.md
  - ./ge06-e3-f1-merge-receipt-2026-06-22.md
  - ./ge06-e3-f2-merge-receipt-2026-06-22.md
  - ./pilot-stack-viability-decision-criteria.md
related_artifacts:
  - ./ge06-post-e5-f1-decision-rack-2026-06-22.md
  - ../execution-handoff.md
  - ../../../doctrine/quality-gate-policy.md
---

# GE06-E5-F1 Viability / Domain-Confidence Decision

## Verdict
GE-06 is not yet pilot-viable.

The strongest truthful outcome class remains:

```text
computed-but-not-oracle-checked
```

The domain-confidence decision is narrower than a rejection and weaker than a viability claim:
- the current deterministic headless pilot path survives with real `Computed` evidence on the supported route
- no fatal model, importer, or engine collapse is currently exposed on that supported route
- the present limiting fact is explicit `OracleGap`, not narration
- the UI truth gate is still unmet, and any E4 move remains a bounded non-production spike posture rather than a product-visible proof

Therefore downstream epics may depend on this posture:

```text
GE-06 survives as a bounded headless proof path, but it does not yet justify pilot-viable or product-visible claims.
```

## Observed repo anchor and live verification
Grounded on 2026-06-22 in `/home/ubuntu/workspace/repos/codex`:

```text
origin/develop: b2f2154
recent merges:
  - b2f2154 Merge pull request #15 from electricm0nk/ge06-e3-f1-selected-parity-dimensions
  - 5e1f68f Merge pull request #14 from electricm0nk/ge06-e3-f2-failure-classifier
  - 6977c86 Merge pull request #13 from electricm0nk/ge06-e2-f3-headless-receipt-path
live verification run:
  - "$HOME/.cargo/bin/cargo" test --test ge06_selected_parity_dimensions --quiet -> pass (1 passed)
  - "$HOME/.cargo/bin/cargo" test --test ge06_failure_classifier --quiet -> pass (5 passed)
  - "$HOME/.cargo/bin/cargo" test --test ge06_pilot_headless_receipt --quiet -> pass (2 passed)
  - "$HOME/.cargo/bin/cargo" test --quiet -> pass
```

## Evidence consumed
| Surface | What it proves for this decision |
|---|---|
| `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md` | The nine mandatory selected pilot dimensions are grounded at a `Computed` floor and the supported-path blocker is explicitly `OracleGap`. |
| `artifacts/ge06-e4-f1-launch-posture-2026-06-22.md` | The stale GE-07 planning blocker is gone, but no live UI handoff or `Product-visible` proof exists; E4-F1 remains only a bounded pre-viability spike posture. |
| `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md` | The deterministic headless receipt path computes on the supported route and preserves explicit blocked-state diagnostics on the mutated route. |
| `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md` | The selected-dimension carrier preserves the nine mandatory review dimensions at a `Computed` floor without counterfeiting parity. |
| `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md` | The failure classifier names the supported-route blocker as `OracleGap` and the blocked-route example as `EngineFlaw`, proving the review can narrow failure ownership honestly. |
| `artifacts/pilot-stack-viability-decision-criteria.md` | Supplies the rubric for `computed-but-not-oracle-checked`, `oracle-checked-but-not-product-visible`, `pilot-viable`, and `fatal-flaw`. |

## Layer-by-layer decision
| Layer | Current posture | Grounded evidence | Effect on final verdict |
|---|---|---|---|
| Documentation gate | pass | The GE-06 source STC, merge receipts, E3 bundle, E4 posture review, and this decision artifact all exist as linked authority surfaces. | Not blocking. |
| Import fidelity gate | inherited supporting pass for the accepted pilot route; not the gating failure in this review | The deterministic fixture is grounded by the final input contract and preserved through the receipt and selected-dimension carrier via `case_id` and `source_package_id`; no supported-route `ImporterFlaw` evidence surfaced in the reviewed bundle. | Not the reason viability is withheld. |
| Rules correctness gate | pass at `Computed` for the supported deterministic route | The headless receipt computes; the selected-dimension carrier emits the nine mandatory dimensions; targeted and full tests pass. | Establishes that the architecture survives the current headless proof path. |
| Oracle parity gate | blocked at `OracleGap` | The supported route classifies to `OracleGap`, and no selected old-vs-new comparison artifact promotes any mandatory dimension to `Oracle-checked`. | This is the primary blocker to stronger viability language. |
| UI truth gate | not `Product-visible` | E4 posture records no live handoff, no grounded UI write scope, and no product-visible proof over real outputs. | Independently prevents a `pilot-viable` verdict. |

## Selected output tier summary
The mandatory selected pilot dimensions currently share the same ceiling in the evidence family:

| Output set | Converted | Computed | Oracle-checked | Product-visible | Current owner if promoted today |
|---|---|---|---|---|---|
| `character.identity` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `combat.baseline_melee_attack_bonus` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `defense.baseline_armor_class` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `defense.total_save.fortitude` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `defense.total_save.reflex` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `defense.total_save.will` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `skill.selected_modifier.climb` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `skill.selected_modifier.intimidate` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |
| `skill.selected_modifier.swim` | inherited from the grounded deterministic pilot route; not re-adjudicated separately here | yes | no | no | `OracleGap` |

This is sufficient to reject counterfeit optimism.
It is not sufficient to grant viability.

## Fatal-flaw audit
The current reviewed evidence does not support `fatal-flaw`.

Why:
- the supported deterministic route computes instead of collapsing in representation or rules execution
- the failure vocabulary narrows to `OracleGap` on the supported route rather than exposing a model/importer/engine impossibility
- the blocked-path example proves the classifier can still surface `EngineFlaw` when computation actually breaks, so the current supported verdict is not hiding an engine collapse
- the missing UI truth is a missing evidence tier, not proof that the architecture cannot carry a bounded UI surface

## Domain-confidence posture for downstream work
Downstream epics should treat GE-06 as follows:

1. `pilot-viable` is not authorized.
2. `product-visible` is not authorized.
3. The architecture has a real, bounded headless proof foothold worth continuing.
4. The next mandatory evidence gap is parity, not a vague rewrite of the entire pilot.
5. Any E4 move before a stronger viability posture exists is a manually authorized non-production spike only.

## Decisive next move
Do not spend the next unit of authority on broad UI work.

The decisive action is to close the selected-dimension oracle gap: derive the narrow follow-on work that produces explicit old-vs-new comparison evidence for the nine mandatory pilot dimensions so the current `Computed` foothold can be promoted or honestly rejected at the parity gate.

Implications:
- treat GE06-E4-F1 as optional and spike-only unless Todd explicitly wants UI-side evidence before parity closure
- route the next mandatory proof burden toward the GE-05 comparison surface rather than pretending the missing gate is cosmetic
- keep downstream product or confidence claims bounded to the current headless-survives / parity-missing posture until that evidence exists

## Explicit non-authorizations
This decision does not authorize:
- calling the pilot viable
- calling any selected dimension `Oracle-checked`
- calling any UI surface `Product-visible`
- broad GE-07 product implementation
- a vague "keep going" instruction that ignores the named parity blocker

## Completion rule
This decision artifact is complete only if it leaves no ambiguity about five facts:

1. the current outcome class is `computed-but-not-oracle-checked`
2. the supported deterministic route survives with real `Computed` evidence
3. the primary blocker is `OracleGap`, not narration
4. the UI gate is still unmet and E4 remains spike-only unless Todd explicitly authorizes it
5. downstream epics must not treat GE-06 as pilot-viable until parity and UI truth evidence actually exist
