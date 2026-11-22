---
title: GE06-E4-F3 Prebuild Readiness Closure
artifact_type: execution-readiness-prebuild
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E4-F3 — One exportable summary boundary
workflow_route: readiness-closure
readiness: blocked
status: prebuilt-draft
created_handoff:
  - ./ge06-e4-f3-prebuild-handoff-2026-06-22.md
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E4-F3 Prebuild Readiness Closure

## Verdict
A bounded GE06-E4-F3 packet can be prebuilt now, but it must remain non-authorizing until GE06-E4-F1 lands on `origin/develop` and the merged view-model / snapshot contract is re-read from the live repo.

This artifact exists to make that launch gate explicit. It does not pretend the gate is already open.

## Core problem
The pilot charter, GE-00 program doctrine, and the accepted GE06-E1-F1 deterministic input contract all require one exportable character summary boundary. The current repo truth does not yet contain the upstream view-model bridge that later summary work must consume, and it also does not contain any bounded export-summary adapter over real GE-06 pilot outputs.

GE06-E4-F1 is the upstream bridge that can expose a real pilot snapshot or explicit blocked posture without fabricating shell behavior. GE06-E4-F3 is the next downstream packet: consume that merged view-model contract and add one bounded exportable summary surface that carries only already-grounded pilot identity, summary values, diagnostics, explanation references, and known-gap / claim-tier language.

Because GE06-E4-F1 is still only a live stage-specific handoff at `awaiting-todd-launch`, GE06-E4-F3 cannot yet mint a truthful code-authorizing brief over implementation that does not exist.

## Selected bounded slice

```text
GE06-E4-F3 — One exportable summary boundary
```

Once its gate opens, this slice should do only four things:

1. consume the merged GE06-E4-F1 view-model / snapshot contract as read-only input
2. emit one bounded exportable summary surface for the accepted deterministic pilot and explicit blocked posture
3. preserve the current evidence ceiling, diagnostics, primary-owner / blocker language, and known-gap posture instead of flattening them into faux-clean success text
4. stay disjoint from GE06-E4-F2 inspection UI work so both downstream lanes can remain independently bounded

It should not broaden into full character-sheet parity, export-template breadth, frontend-owned formatting studio work, rules browsing, shell inspection work, or stronger oracle / product-visible claims.

## Grounded source evidence available now
| Gate | Grounded source |
|---|---|
| Program-level slice boundary | `programs/codex/doctrine/program-doctrine-and-scope-charter.md`, `programs/codex/requirements/GE-00-program-governance-and-scope/README.md`, `programs/codex/requirements/GE-00-program-governance-and-scope/technical-requirements.md`, and `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` all name the first pilot slice as including exactly one exportable character summary, not full export-sheet breadth. |
| Closed GE-06 export-summary doctrine | `artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md` closes the export-summary boundary as one headless summary receipt and names the required receipt sections: identity, deterministic selections, selected computed outputs or blockers, explanation/provenance references, diagnostics/known gaps, GE-05 reference state, and quality-gate claim-tier language. |
| GE-06 summary/explanation/diagnostic obligations | `technical-requirements.md` TR-06-009 and TR-06-011 require explanation visibility, diagnostics visibility, blocked-state honesty, and minimal UI truth over real domain outputs rather than fabricated export success. |
| GE-06 payload boundary | `technical-design.md` defines the UI projection payload minimums: product-visible character summary fields, explanation-affordance references, diagnostics visible to the UI, and blocked/unavailable state visibility. |
| GE-07 downstream consumer boundary | `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md` already names `summary_values`, `diagnostics[]`, and `explanation_refs[]` as the shape a later shell can consume without becoming the semantic owner. |
| GE06-E4-F1 upstream bridge | `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` and `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md` define the bounded rules-core view-model lane that E4-F3 must consume after merge. |
| Live repo boundary today | `git -C /home/ubuntu/workspace/repos/codex rev-parse --short origin/develop` returns `7bc89e8`; file search shows no `src/rules_core/pilot_view_model.rs`, no `src/rules_core/pilot_summary.rs`, no summary/export module under `src/`, and no `apps/desktop/` subtree, so both the upstream view-model bridge and any summary adapter are still absent from merged repo truth. |
| Current computed evidence floor | `/home/ubuntu/workspace/repos/codex/src/oracle_validation/selected_parity_dimensions.rs` projects only the nine mandatory pilot dimensions at a `Computed` floor, proving that current evidence remains bounded and must not be relabeled as oracle-checked or full-sheet parity. |

## Launch gates that remain closed
This packet must not be promoted into a code-authorizing handoff until all of the following are true:

1. `artifacts/ge06-e4-f1-merge-receipt-YYYY-MM-DD.md` exists for the real merged GE06-E4-F1 slice.
2. The live repo at merged `origin/develop` still exposes a stable view-model / snapshot contract materially compatible with the GE06-E4-F1 handoff.
3. A post-merge documentary pass confirms the smallest truthful E4-F3 implementation lane and exact write scope against the live repo state rather than against the current documentary hypothesis alone.
4. The future summary surface still preserves the current evidence ceiling, diagnostics, and known-gap language without widening into full export-sheet parity or richer UI inspection work.

If any gate fails, re-derive the packet instead of widening silently.

## Candidate implementation posture after gate clear
If the later post-E4-F1 audit still matches the current repo posture, the smallest likely implementation surface is:

```text
src/rules_core/mod.rs
src/rules_core/pilot_summary.rs
tests/ge06_pilot_summary.rs
```

Read-only dependencies for that later run should include:

```text
src/rules_core/pilot_view_model.rs
src/rules_core/pilot_compute.rs
src/rules_core/pilot_failure.rs
src/oracle_validation/selected_parity_dimensions.rs
tests/ge06_pilot_view_model.rs
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f2-narrow-vs-expand-decision-2026-06-22.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md
programs/codex/doctrine/quality-gate-policy.md
```

That candidate scope is intentionally provisional. Because neither the F1 bridge nor the summary adapter exists in merged repo truth yet, a later promotion pass must re-check the exact file identities before any code authority exists.

## Explicit non-goals
Do not let a future GE06-E4-F3 handoff authorize:
- edits to `src/oracle_validation/**`, `src/pcgen_import/**`, or `/home/ubuntu/workspace/repos/pcgen`
- changes to `src/rules_core/pilot_compute.rs`, `src/rules_core/pilot_failure.rs`, or `src/rules_core/pilot_view_model.rs`
- explanation / diagnostic inspection UI work owned by GE06-E4-F2
- `apps/**`, `src-tauri/**`, frontend packaging, or shell-polish work
- full export-sheet parity, template breadth, print/share studio features, or general-purpose report generation
- parity promotion above the current evidence ceiling
- product-viable or broad GE-07 claims

## Completion rule
This prebuild readiness closure is complete when it leaves no ambiguity about three facts:
- GE06-E4-F3 is downstream of the real GE06-E4-F1 merged contract, not merely its documentary brief
- the packet can be prebuilt now without counterfeit activation
- the live code-authorizing moment remains in the future, after post-E4-F1 merge evidence and a fresh scope audit
