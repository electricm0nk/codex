---
title: GE-06 Risks and Open Questions
stc_id: STC-CODEX-GE-06
artifact_type: risks-and-open-questions
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter
source_stc: ./README.md
source_artifacts:
  - ./technical-requirements.md
  - ./artifacts/pilot-character-fixture-requirements.md
  - ./artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
  - ./artifacts/pilot-stack-viability-decision-criteria.md
  - ../GE-05-oracle-validation-and-parity-harness/research-handoff.md
---

# GE-06 Risks and Open Questions

## Objective
Quarantine the unresolved decisions and structural risks that must remain visible while GE-06 stays planning-ready rather than counterfeit execution-ready.

## Open questions requiring later closure

The first deterministic pilot input choices are closed by `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`. GE06-E2-F1a, GE06-E2-F2a, GE06-E2-F2b, GE06-E2-F2c, GE06-E2-F2d, GE06-E2-F3, GE06-E3-F2, and GE06-E3-F1 are now merged. `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md`, `artifacts/ge06-e3-f2-merge-receipt-2026-06-22.md`, and `artifacts/ge06-e3-f1-merge-receipt-2026-06-22.md` record the merged integrated headless, failure-routing, and selected-dimension footholds; `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md` materializes the E3 evidence family; `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md` fixes the posture at `computed-but-not-oracle-checked`; `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md` fixes the next branch at narrow-the-pilot through GE-05 parity ownership; `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` plus `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md` now ground the bounded view-model lane; `artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md` plus `artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md` now preserve the downstream inspection lane as explicit prebuild-only truth; `artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md` plus `artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md` now preserve the downstream export-summary lane as explicit prebuild-only truth; and `execution-handoff.md` now carries the live E4-F1 pair at `awaiting-todd-launch` while `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md` governs the remaining GE-06 packet family. The remaining open questions are parity closure sequence, downstream UI minimum after the bounded view-model lane, the smallest truthful export-summary boundary after that same merge, and what exact GE-05 follow-on surfaces must be grounded before stronger GE-06 claim-tier promotion is honest.

### RQ-06-001 — Exact deterministic feat/choice set
Status: closed for the first bounded pilot case by `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`.

Grounded facts:
- the pilot charter names `power_attack` explicitly
- the grounded GE-01 inputs include `STARTFEATS`, `ABILITYPOOL`, `CHOOSE`, `MULT`, and proficiency-choice surfaces

Closure:
- level-1 character feat: Power Attack
- Human bonus feat: Dodge
- Fighter bonus feat: Weapon Focus (Longsword)

Why it matters:
- any later deviation from this set is scope-bearing and must update the deterministic input contract

### RQ-06-002 — Exact skill allocation
Status: closed for the first bounded pilot case by `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`.

Grounded facts:
- the pilot charter requires skill behavior and names skill-rank outputs
- GE-01 and GE-04 ground `SKILL`, `CSKILL`, `KEYSTAT`, and `STARTSKILLPTS` as pilot-critical

Closure:
- Climb 1
- Intimidate 1
- Swim 1
- favored-class bonus fixed to hit point, not skill rank

### RQ-06-003 — Exact equipment loadout and active-state rules
Status: closed for the first bounded pilot case by `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`.

Grounded facts:
- the pilot charter requires equipment handling, armor class, melee attack bonus, and equipment effects
- GE-01 grounds Chain Shirt and Longsword as representative equipment rows

Closure:
- Chain Shirt worn/active
- Longsword primary/active
- no shield
- no other first-slice inventory
- Power Attack selected but inactive for baseline outputs

### RQ-06-004 — Mandatory parity dimensions for viability
Status: closed for the current review surface by `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md` and consumed by `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`.

Grounded facts:
- the pilot charter and GE-05 both imply selected, not universal, comparison targets
- the charter names one exportable character summary/stat-block boundary

Closure:
- the current mandatory selected pilot dimensions are `character.identity`, `combat.baseline_melee_attack_bonus`, `defense.baseline_armor_class`, `defense.total_save.fortitude`, `defense.total_save.reflex`, `defense.total_save.will`, `skill.selected_modifier.climb`, `skill.selected_modifier.intimidate`, and `skill.selected_modifier.swim`
- those dimensions currently sit at a `Computed` floor and remain blocked from stronger claim tiers by `OracleGap`
- optional or later dimensions may still exist, but they are not required to decide the present viability posture honestly

### RQ-06-005 — Minimum acceptable UI surface
Grounded facts:
- GE-06 requires a minimal UI slice
- GE-07 now has a source STC
- GE06-E4-F1 now has a live bounded readiness/handoff pair, but that lane is restricted to a rules-core view-model contract rather than product-visible shell work
- GE06-E4-F2 and GE06-E4-F3 now have explicit prebuild-only documentary artifacts, but they remain non-authorizing until later post-E4-F1 merge promotion passes

Open question:
- after the bounded rules-core view-model lane is merged, what is the smallest product-visible UI that proves direction without smuggling broad GE-07 work into GE-06, and do the prebuilt E4-F2 and E4-F3 packets still remain the smallest truthful downstream lanes after a live repo re-read?

### RQ-06-006 — Required upstream implementation surfaces after F3
Grounded facts:
- GE-03, GE-04, GE-05, and GE-07 source STCs exist
- GE06-E5-F2 now fixes the next mandatory proof burden on GE-05 parity ownership rather than broad GE-06 or GE-07 expansion
- the GE-05 execution route surface names `GE05-E2-F2 — PF1 Human Fighter level 1 governed fixture instance` as the current truthful next bounded candidate

Open question:
- which exact GE-05 follow-on slices after `GE05-E2-F2` become sufficient to promote the current selected dimensions from `Computed` toward `Oracle-checked`, and at what point would evidence justify reopening GE-03, GE-04, GE-06, or GE-07 requirements surfaces?

### RQ-06-007 — Branch/worktree policy for integrated code work
Status: closed for the early GE-06 coding path by merged receipts `artifacts/ge06-e2-f1a-merge-receipt-2026-06-21.md`, `artifacts/ge06-e2-f2a-merge-receipt-2026-06-21.md`, `artifacts/ge06-e2-f2b-merge-receipt-2026-06-21.md`, `artifacts/ge06-e2-f2c-merge-receipt-2026-06-21.md`, `artifacts/ge06-e2-f2d-merge-receipt-2026-06-21.md`, and `artifacts/ge06-e2-f3-merge-receipt-2026-06-21.md`.

Grounded facts:
- GE06-E2-F1a merged to `develop` at `9f3cb93`
- GE06-E2-F2a merged to `origin/develop` at `760c9b0`
- GE06-E2-F2b merged to `origin/develop` at `75c26ce`
- GE06-E2-F2c merged to `origin/develop` at `1b44c07`
- GE06-E2-F2d merged to `origin/develop` at `2deb11b`
- GE06-E2-F3 merged to `origin/develop` at `6977c86`
- all six slices used a develop-first route with bounded write scope and stage-specific handoffs

Closure:
- the working policy for subsequent GE-06 coding slices is to branch from current `origin/develop` unless an explicit dependency branch is named in a later readiness closure
- if a later slice requires stacked unpublished dependencies, that dependency posture must be declared explicitly in its own handoff rather than assumed globally

## Planning risks that could become execution blockers

### RK-06-001 — Broad integration handoff risk
If GE-06 is handed to a coding harness as “build the pilot,” scope explosion is almost guaranteed.

Required mitigation:
- require a bounded readiness closure before any execution handoff
- select one narrow slice at a time

### RK-06-002 — Mock-UI counterfeit progress risk
GE-06 is especially vulnerable to a pretty shell over unproven behavior.

Required mitigation:
- enforce the headless-first gate
- require the UI to consume real outputs and visible diagnostics

### RK-06-003 — Oracle-route incompleteness risk
GE-05 still treats old-system command discovery as a bounded problem rather than a solved fact.

Consequence:
- some GE-06 outputs may be able to reach `Computed` before they can reach `Oracle-checked`

Required mitigation:
- record those cases explicitly in the viability criteria rather than pretending the gap does not exist

### RK-06-004 — Provenance/explanation leakage risk
If importer or engine outputs lose provenance or explanation fidelity, GE-06 may look functional while still failing the program's actual doctrine.

Required mitigation:
- make explanation/provenance visibility a hard acceptance gate for selected outputs

### RK-06-005 — Hidden scope expansion through fixture “clarification”
Exact fixture closure can tempt the project into adding adjacent feats, gear breadth, or export surfaces that belong to later scope.

Required mitigation:
- apply the charter-alignment artifact and ADR trigger rules before accepting expansions

## Forbidden assumptions
- do not assume the named `power_attack` feat is the entire mandatory selection surface for the pilot
- do not alter the accepted deterministic input contract without treating the change as scope-bearing
- do not assume GE-07 can be skipped just because GE-06 mentions a minimal UI
- do not assume a missing parity dimension is harmless; classify it explicitly
- do not assume current branch state in the repo is the eventual GE-06 integration base
