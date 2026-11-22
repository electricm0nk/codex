---
title: GE-06 Epic Breakdown
stc_id: STC-CODEX-GE-06
artifact_type: epic-breakdown
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter
source_stc: ./README.md
source_artifacts:
  - ./technical-requirements.md
  - ./technical-design.md
  - ./artifacts/pilot-character-fixture-requirements.md
  - ./artifacts/pilot-stack-viability-decision-criteria.md
---

# GE-06 Epic Breakdown

## Purpose
Decompose GE-06 into implementation-facing epics and feature seeds that can later become bounded readiness closures or handoffs.

This file is not a code-authorizing brief. It names the terrain so later execution can stay narrow.

## Epic GE06-E1 — Pilot character fixture closure and governed-input alignment
**Objective:** Finalize the exact integrated pilot case and reconcile charter facts, token-family gates, and canonical-object obligations into one bounded fixture contract.

**Derived from:** TR-06-004, TR-06-005, TR-06-006, `artifacts/pilot-charter-alignment.md`, `artifacts/pilot-character-fixture-requirements.md`.

### Feature seeds
#### GE06-E1-F1 — Grounded character selection ledger
Acceptance:
- the exact race/class/level/ability-score/feat/skill/equipment selection set is explicit
- unresolved selections are either closed or promoted into explicit blockers
- no selection silently expands the charter boundary

#### GE06-E1-F2 — Token-family hard-gate contract
Acceptance:
- the hard-gate versus supporting token families for the first integrated case are explicit
- each missing token family is routed back to an owning upstream epic rather than hidden in GE-06 prose

#### GE06-E1-F3 — Charter delta and ADR trigger review
Acceptance:
- any pilot-scope expansion is classified as no-change, charter patch, or ADR trigger
- GE-06 does not quietly broaden the pilot under the label of fixture cleanup

## Epic GE06-E2 — Headless integrated import/compute/proof path
**Objective:** Prove the pilot can flow through a real headless path before any UI-visible claim is allowed.

**Derived from:** TR-06-007, TR-06-008, TR-06-009.

### Feature seeds
#### GE06-E2-F1 — Integrated source-package load contract
Acceptance:
- the slice can identify the exact package/load path or explicit blocker
- importer outputs preserve provenance and diagnostics needed downstream

#### GE06-E2-F2 — Character compute and explanation contract
Acceptance:
- the slice can compute selected outputs or block explicitly
- explanation and diagnostic payloads are available for those outputs

Current bounded history:
- `GE06-E2-F2a` is complete and merged as the first compute foothold: base ability modifiers plus Fighter level-1 class chassis (BAB and base saves only)
- `GE06-E2-F2b` is complete and merged as the first deterministic combat-totals foothold: baseline melee attack bonus `+5` and baseline armor class `17` under the accepted Longsword / Chain Shirt / Dodge / no-shield posture
- `GE06-E2-F2c` is complete and merged as the first deterministic total-saves foothold: total Fortitude `4`, Reflex `2`, and Will `1`, plus truthful `pilot_compute.rs` prose for the post-F2b/post-F2c support surface
- `GE06-E2-F2d` is complete and merged as the first deterministic selected-skill foothold: Climb `5`, Intimidate `3`, and Swim `5`, with truthful explanation payloads and claim-blocking refusal of unsupported or widened posture
- `GE06-E2-F3` is complete and merged as the first integrated headless receipt foothold: one bounded library-first receipt preserves deterministic pilot identity, grounded computation payloads, representative explanation ids, and blocked-vs-computed status without inventing parity or UI claims

Current queue posture after the E5-F3 review:
- the root route surface is now retired to `no-active-handoff` and preserves GE06-E4-F1 as merged historical authority through `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md`
- the remaining packet family is governed by `artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md` as historical branch context rather than a live route gate
- GE06-E3-F2 and GE06-E3-F1 are complete and merged as the first bounded E3 evidence pair
- GE06-E3-F3 is complete as `artifacts/ge06-e3-f3-viability-evidence-bundle-2026-06-22.md`, which records the selected dimensions at a `Computed` floor and names `OracleGap` as the current supported-path blocker
- GE06-E5-F1 is complete as `artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`, which fixes the current posture at `computed-but-not-oracle-checked` and refuses counterfeit `pilot-viable` language
- GE06-E5-F2 is complete as `artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md`, which chooses narrow-the-pilot and routes the next mandatory proof burden to GE-05 parity ownership
- GE06-E4-F1 is complete and merged as `artifacts/ge06-e4-f1-merge-receipt-2026-06-26.md`, while GE06-E5-F3 is complete as `artifacts/ge06-e5-f3-upstream-delta-review-2026-06-22.md`; the downstream E4-F2 / E4-F3 packets remain prebuilt, non-authorizing follow-ons pending a fresh post-merge promotion pass

#### GE06-E2-F3 — End-to-end command and receipt path
Acceptance:
- one bounded command or test path can produce integrated headless evidence or a clear blocker receipt
- emitted evidence is sufficient for later parity/UI consumers

## Epic GE06-E3 — Oracle comparison and failure-routing integration
**Objective:** Connect selected old-vs-new comparison dimensions and integrated failure classification without broadening GE-05.

**Derived from:** TR-06-010, TR-06-012, TR-06-013.

### Feature seeds
#### GE06-E3-F1 — Selected parity-dimension adapter
Acceptance:
- mandatory comparison dimensions for viability are explicit
- optional or known-gap dimensions are classified without pretending they passed

#### GE06-E3-F2 — Failure classifier and owner mapping
Acceptance:
- integrated failures resolve to model flaw, importer flaw, engine flaw, oracle gap, or UI gap
- cross-layer failures still identify a primary owner

#### GE06-E3-F3 — Viability evidence bundle
Acceptance:
- selected outputs record claim tier, evidence source, and blocking reason when not yet viable
- the pilot can be judged by evidence rather than narration

## Epic GE06-E4 — Minimal UI truth slice
**Objective:** Surface the integrated pilot path in the smallest product-visible UI that still uses real outputs and keeps diagnostics/explanations visible.

**Derived from:** TR-06-011.

### Feature seeds
#### GE06-E4-F1 — Pilot view-model contract from real outputs
Acceptance:
- UI inputs come from real headless outputs or explicit blockers
- no hardcoded character data substitutes for domain truth
- blocked receipts do not silently degrade into faux success snapshots

#### GE06-E4-F2 — Explanation and diagnostic inspection surface
Acceptance:
- the user can inspect why a value exists and why a choice is unavailable
- diagnostics remain visible instead of being hidden as implementation noise

Current documentary posture:
- `artifacts/ge06-e4-f2-prebuild-readiness-closure-2026-06-22.md` and `artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md` may exist as non-authorizing downstream truth before launch
- `artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md` and `artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md` may exist as non-authorizing downstream truth before launch
- the live implementation lanes must still wait for a post-E4-F1 merge promotion pass over the real contract

#### GE06-E4-F3 — One exportable summary boundary
Acceptance:
- the slice defines one bounded export/summary surface or records why it remains blocked
- export scope does not silently expand into broad sheet parity

## Epic GE06-E5 — Viability review and upstream delta handling
**Objective:** Convert integrated evidence into a decision about whether the architecture survives the first pilot and what must happen next.

**Derived from:** TR-06-013 and the documentation-control-plane propagation rule.

### Feature seeds
#### GE06-E5-F1 — Pilot viability review artifact
Acceptance:
- the review records which selected outputs reached `Converted`, `Computed`, `Oracle-checked`, and `Product-visible`
- fatal flaws, narrowable gaps, and acceptable known gaps are separated explicitly

#### GE06-E5-F2 — Narrow-vs-expand decision trigger
Acceptance:
- the review states whether the next move is narrow the pilot, expand upstream requirements, or stop due to architectural failure
- the recommendation points at owning STCs or decisions rather than vague follow-up

#### GE06-E5-F3 — Upstream delta/no-change review
Acceptance:
- any new cross-epic learning is propagated to the owning STC, charter, or decision surface
- the absence of upstream change is declared explicitly when true

## Recommended sequencing
1. GE06-E1 — close the exact fixture and governed-input boundary first.
2. GE06-E2 — prove the headless integrated path.
3. GE06-E3 — add selected oracle comparison and failure routing.
4. GE06-E4 — surface the same real outputs in the minimum UI slice.
5. GE06-E5 — write the viability decision and upstream delta review.

## Completion gate
GE-06 is implementation-ready only when a bounded slice has:
- exact objective
- exact repo/workdir
- branch/worktree policy
- allowed write scope
- required upstream evidence surfaces
- selected claim-tier target
- verification commands or receipt requirements
- non-goals preventing broad integration drift

Until then, this epic breakdown remains planning authority, not code authority.
