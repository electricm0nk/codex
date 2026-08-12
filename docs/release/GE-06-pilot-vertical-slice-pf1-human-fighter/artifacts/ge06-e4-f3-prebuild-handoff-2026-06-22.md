---
title: GE06-E4-F3 Prebuild Handoff Draft — One Exportable Summary Boundary
handoff_id: HANDOFF-CODEX-GE-06-E4-F3-PREBUILD-2026-06-22
stc_id: STC-CODEX-GE-06
handoff_kind: execution-handoff-draft
work_type: implementation-ready
workflow_route: coding
readiness: blocked
status: prebuilt-draft
owner: Todd Hintzmann
scope: program
canonical: false
canonical_path: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/execution-handoff.md
source_stc: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md
readiness_closure: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md
selected_slice: GE06-E4-F3 — One exportable summary boundary
run_in: Claude Code or equivalent frontier coding harness, but only after post-E4-F1 promotion
code_authority: false
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: origin/develop
  recommended_branch: ge06-e4-f3-exportable-summary-boundary
future_live_artifacts:
  - artifacts/ge06-e4-f3-execution-readiness-closure-YYYY-MM-DD.md
  - artifacts/ge06-e4-f3-execution-handoff-YYYY-MM-DD.md
  - artifacts/ge06-e4-f3-merge-receipt-YYYY-MM-DD.md
allowed_write_scope:
  - src/rules_core/mod.rs
  - src/rules_core/pilot_summary.rs
  - tests/ge06_pilot_summary.rs
forbidden_write_scope:
  - /home/ubuntu/workspace/repos/pcgen
  - apps/**
  - src-tauri/**
  - src/oracle_validation/**
  - src/pcgen_import/**
  - src/rules_core/pilot_compute.rs
  - src/rules_core/pilot_failure.rs
  - src/rules_core/pilot_view_model.rs
  - tests/ge06_pilot_view_model.rs
  - Cargo.toml
  - Cargo.lock
  - AGENTS.md
  - CLAUDE.md
  - programs/codex/**
---

# GE06-E4-F3 Prebuild Handoff Draft — One Exportable Summary Boundary

## Status
This is a prebuilt draft only.

Do not hand this to Claude Code yet. It carries `code_authority: false` until a later documentary pass promotes it after GE06-E4-F1 merge evidence exists and the live repo still supports the same narrow implementation lane.

## Objective
Once GE06-E4-F1 is merged, create the smallest additive summary/export boundary that lets the pilot emit one exportable character summary over real GE-06 data without widening into full-sheet parity or export-tool sprawl.

The future surface should preserve:
- the real GE06-E4-F1 pilot snapshot or explicit blocked posture
- the current evidence ceiling and known-gap language
- diagnostics and explanation references relevant to each surfaced summary value
- primary-owner / blocker truth when the path is blocked
- exactly one bounded exportable summary, not a family of export templates

## Required reads before any future promotion
1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
3. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f3-prebuild-readiness-closure-2026-06-22.md`
4. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-merge-receipt-YYYY-MM-DD.md`
5. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md`
6. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md`
7. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`
8. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_view_model.rs`
9. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_compute.rs`
10. `/home/ubuntu/workspace/repos/codex/src/rules_core/pilot_failure.rs`
11. `/home/ubuntu/workspace/repos/codex/src/oracle_validation/selected_parity_dimensions.rs`
12. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md`
13. `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md`
14. `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md`
15. `programs/codex/doctrine/quality-gate-policy.md`

## Post-merge promotion gate
A future documentary run may mint the live GE06-E4-F3 execution handoff only if all are true:

1. GE06-E4-F1 is merged and receipted.
2. The merged repo still exposes a stable view-model / snapshot surface materially compatible with the E4-F1 handoff.
3. The candidate summary/export lane below still remains the smallest truthful implementation and stays disjoint from GE06-E4-F2 inspection UI work.
4. The promoted summary surface can preserve diagnostics, blocker language, and current claim-tier ceilings without inventing broader parity or product-visible success.
5. No new upstream doctrine changed the one-summary boundary or narrowed the evidence ceiling further.

If any item fails, throw this draft away and derive a fresh stage-specific handoff.

## Candidate write scope after promotion
```text
src/rules_core/mod.rs
src/rules_core/pilot_summary.rs
tests/ge06_pilot_summary.rs
```

The goal of that future code slice should be to stay inside the rules-core read-model / export-boundary lane and off both the UI inspection lane and the parity-comparator lane.

## Draft implementation behavior
When promoted, the live handoff should require the coding harness to:
1. consume the merged GE06-E4-F1 pilot snapshot or blocked posture through a read-only boundary
2. emit one bounded exportable summary contract or renderer over already-grounded pilot fields only
3. preserve diagnostics, explanation references, and known-gap / blocker language in the summary output instead of discarding them for cleanliness
4. preserve blocked posture without emitting a faux success stat block when the upstream payload is blocked
5. avoid shell inspection UI work, export-template breadth, parity promotion, or rules recomputation

## Mandatory boundary content
The emitted boundary should preserve these fields or exact equivalents:

```text
case_id
source_package_id
status
claim_tier_floor_or_evidence_ceiling
summary_values
explanation_refs
diagnostics
primary_owner_or_blocker_language
```

Summary output should include only already-grounded pilot identity and values. If a desired field is not present in the merged GE06-E4-F1 contract or another explicitly allowed upstream surface, the summary must preserve that absence honestly rather than inventing or recomputing it.

At minimum, the exported summary should preserve the deterministic pilot identity and the currently grounded selected outputs carried by the accepted GE-06 surfaces:

```text
case_id = pf1-crb-human-fighter-level1
source_package_id = pf1.core_rulebook
combat.baseline_melee_attack_bonus = 5
defense.baseline_armor_class = 17
defense.total_save.fortitude = 4
defense.total_save.reflex = 2
defense.total_save.will = 1
skill.selected_modifier.climb = 5
skill.selected_modifier.intimidate = 3
skill.selected_modifier.swim = 5
```

It may include additional already-grounded summary values from the merged GE06-E4-F1 contract, but it must not widen into full-sheet breadth or imply oracle-checked parity.

For a blocked fixture, the summary must:
- keep `status = Blocked`
- preserve the real primary owner or blocker language available from upstream
- preserve real diagnostics
- refuse to surface a faux computed stat block or clean success summary

## Verification posture to ground later
The future live handoff should not invent an export stack or downstream template system that the repo has not grounded yet.

Instead, the later promotion pass should require exact receipts proving at minimum:
- one bounded summary surface is emitted from real GE06-E4-F1-derived data
- the summary preserves the configured evidence ceiling rather than claiming oracle-checked parity
- diagnostics and blocker language remain visible when present
- no hardcoded pilot numbers are introduced outside the existing grounded evidence surface
- blocked payloads stay blocked in the exported summary

## Non-goals
The future live handoff must not authorize:
- edits to `src/oracle_validation/**`, `src/pcgen_import/**`, or `/home/ubuntu/workspace/repos/pcgen`
- edits to `src/rules_core/pilot_compute.rs`, `src/rules_core/pilot_failure.rs`, or `src/rules_core/pilot_view_model.rs`
- `apps/**`, `src-tauri/**`, or shell inspection UI work
- full character-sheet / export-sheet parity
- template libraries, print/share studio features, or report-generation breadth
- parity comparator or claim-tier promotion work
- frontend-owned semantic formatting logic
- diagnostic suppression for cleanliness

## Why this draft exists
GE06-E4-F3 is a real downstream packet, but today it depends on a merged GE06-E4-F1 contract that does not exist yet.

This draft preserves the bounded objective, future artifact identities, and candidate implementation lane without counterfeiting activation. The real code-authorizing moment remains after GE06-E4-F1 merge evidence, where it belongs.
