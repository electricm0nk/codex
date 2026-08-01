---
title: GE-09 Known-Gap and Regression Governance
stc_id: STC-CODEX-GE-09
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts
source_stc: ../README.md
related:
  - ../../GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
  - ../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv
  - ../../../doctrine/quality-gate-policy.md
  - ./coverage-dashboard-requirements.md
  - ./release-milestone-model.md
  - ./package-compatibility-and-migration-policy.md
---

# GE-09 Known-Gap and Regression Governance

## Purpose
Define how GE-09 expansion, compatibility, release, and dashboard surfaces consume GE-05 known-gap truth and regression evidence so missing behavior, downgraded claims, and accepted divergence remain visible instead of being misreported as progress.

## Authority stack
GE-09 does not own a separate gap taxonomy.
The authoritative stack is:
1. GE-05 known-gap policy defines the gap classes, required ledger/report behavior, and intentional-divergence rule.
2. The quality-gate policy defines claim tiers and the expansion/oracle/UI truth gates that gaps can block.
3. GE-01 evidence surfaces provide token-family and unresolved-debt grounding for pilot-bounded and adjacent-domain scopes.
4. GE-06 defines the current claim ceiling that GE-09 must not overstate.
5. GE-09 consumes those truths to control package, dashboard, and release posture.

Any GE-09 artifact that invents a second gap model is invalid.

## Required GE-05 gap-class reuse
When GE-09 names blocking or accepted gap posture, it MUST reuse GE-05 class names exactly where applicable:
- `oracle-route-unavailable`
- `codex-output-unavailable`
- `unsupported-imported-semantics`
- `rules-engine-debt`
- `normalization-ambiguous`
- `non-comparable-output`
- `intentionally-divergent`
- `legal-retention-limited`
- `out-of-pilot-scope`

GE-09 may group multiple gaps into a row-level summary, but it may not rename or blur the underlying class semantics.

## Core governance rules
- Every expansion, compatibility, or release claim must declare whether blocking or accepted known gaps exist for the exact named scope.
- A gap that affects a claim ceiling must be visible on the dashboard and in any scoped release or compatibility review; omission is failure.
- Regression findings are first-class inputs to expansion and release posture, not cleanup trivia.
- Accepted known gaps must remain narrow and scoped; they may justify a limited claim, never a broader one.
- Intentional divergence may cap a claim only when the row or report links to an explicit decision record. GE-09 may not treat silent divergence as acceptable debt.
- If a stronger past claim is weakened by a new gap or regression, GE-09 must downgrade or block the claim immediately rather than wait for a later planning pass.

## Surface-consumption contract
The following GE-09 surfaces MUST consume known-gap and regression truth in specific ways.

| Surface | Required consumption behavior |
|---|---|
| `artifacts/coverage-dashboard-requirements.md` | Every row must show gap counts, highest severity, blocking classes, regression state, and review triggers. |
| `artifacts/release-milestone-model.md` | Milestone promotion must refuse stronger language when blocking gaps or regressions invalidate the named evidence tier. |
| `artifacts/package-compatibility-and-migration-policy.md` | Compatibility wording must narrow to the affected package, token family, and evidence tier whenever gap posture weakens. |
| `artifacts/expansion-scope-selection-policy.md` | Candidate ranking must treat unresolved high-leverage gap clusters and parity debt as scope-broadening brakes, not footnotes. |
| future GE-09 release notes, dashboards, and evidence ledgers | They must expose known-gap and regression posture as part of the claim itself, not as optional appendix material. |

## Block and downgrade matrix
GE-09 surfaces MUST apply the following minimum behavior.

| Condition | Required GE-09 response |
|---|---|
| A blocking GE-05 gap class affects the exact scoped package or token family. | Block any stronger compatibility or release claim for that scope. |
| A new regression affects a scope previously presented at a stronger claim tier. | Downgrade the claim immediately and force review before restoration. |
| The only available evidence is below the claimed tier. | Lower the claim ceiling to the proven tier; do not retain aspirational language. |
| A gap is accepted only because it sits outside the exact bounded claim. | Keep the gap visible and prevent language bleed into broader scopes. |
| A row depends on GE-08 authoring or contribution posture that is not yet grounded. | Mark the authored-package or contribution claim provisional or blocked rather than compatible. |
| A scope is outside the pilot boundary or depends on adjacent non-pilot mechanics. | Treat it as non-authorizing for pilot-wide compatibility; route it through GE-09 ranking and later evidence collection. |

## Review triggers
GE-09 must reopen gap and regression posture when any of the following occurs:
- a GE-05 parity artifact changes the classification, severity, or comparison result for a scoped behavior
- a new unsupported-token-ledger entry appears for a candidate package or token family
- an existing unsupported-token-ledger entry changes severity, owner, or review status
- a regression artifact lands for any previously claimed scope
- a decision record marks a behavior intentionally divergent or changes authority posture
- GE-06 propagated posture changes the current pilot claim ceiling
- GE-08 readiness changes the authoring or contribution boundary for package lifecycle claims
- a package upgrade, migration surface change, or release review reopens the compatibility ceiling

## Required reporting behavior
Any future GE-09 dashboard, release review, expansion decision packet, or package-compatibility surface MUST, at minimum:
- name the exact scope affected
- state the highest proven evidence tier for that scope
- list or summarize the active blocking gap classes
- state whether active regressions exist
- link the latest verification or regression artifact
- state whether the current posture is `unblocked`, `known-gap-limited`, `downgrade-required`, or `blocked`
- name the next review trigger and owning surface

A report that omits non-comparable, downgraded, or blocked behavior to look green is invalid.

## Handling accepted known gaps
Accepted known gaps are allowed only when all are true:
- the gap is visible in the relevant GE-09 surface
- the gap sits outside the exact bounded claim being made
- the gap does not invalidate the claimed milestone or compatibility tier
- the gap has an owner or owning GE surface
- the gap has a review trigger
- the existence of the gap does not silently broaden the public claim language

If any of those conditions fail, the gap is not accepted; it is blocking.

## Handling intentional divergence
When GE-09 encounters behavior that Codex intentionally will not preserve:
- the scoped surface must link to a doctrine decision record
- the row or report must classify the posture as `intentionally-divergent`
- the claim language must explain the bounded effect of the divergence
- the divergence must not be counted as accidental parity success

No future GE-09 builder may hide intentional divergence in migration notes or release prose.

## Expansion-specific rule
For expansion ranking, unresolved mechanics clusters with broad downstream impact must be treated as candidate-brake evidence. In the current posture this includes, at minimum, formula-bearing progression semantics, choice/pool semantics, predicate-gated grants, and Human race-trait composition debt surfaced by GE-01 and the unsupported-token ledger.

The effect is decisive:
- scope-deepening work that reduces those blockers may outrank exciting scope broadening
- adjacent non-pilot domains may remain inventoried but held below stabilization work until stronger parity and model truth exist

## Completion rule
This policy is complete for the planning pass when future GE-09 release, dashboard, and expansion-review surfaces can neither ignore known gaps nor treat regressions as optional narrative, and when downgrade/block behavior is explicit enough that a stronger claim cannot survive on inertia alone.