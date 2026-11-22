---
title: GE06-E4-F2 Prebuild Readiness Closure
artifact_type: execution-readiness-prebuild
stc_id: STC-CODEX-GE-06
source_stc: ../README.md
source_epic_breakdown: ../epic-breakdown.md
selected_slice: GE06-E4-F2 — Explanation and diagnostic inspection surface
workflow_route: readiness-closure
readiness: blocked
status: prebuilt-draft
created_handoff:
  - ./ge06-e4-f2-prebuild-handoff-2026-06-22.md
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# GE06-E4-F2 Prebuild Readiness Closure

## Verdict
A bounded GE06-E4-F2 packet can be prebuilt now, but it must remain non-authorizing until GE06-E4-F1 lands on `origin/develop` and the merged view-model/output contract is re-read from the live repo.

This artifact exists to make that launch gate explicit. It does not pretend the gate is already open.

## Core problem
TR-06-009 and TR-06-011 require the pilot slice to keep explanation and diagnostic truth visible: a user must be able to inspect why a derived value exists, why a choice is unavailable, and which diagnostics still apply.

GE06-E4-F1 already defines the smallest upstream bridge that can expose a real pilot snapshot over the merged headless receipt path. GE06-E4-F2 is the next narrow downstream packet: consume that real view-model/output contract and add one bounded inspection surface for explanation, invalid/unavailable-choice reason, and diagnostics visibility without widening into broad UI, rules browsing, or export work.

Because GE06-E4-F1 is still only a live stage-specific handoff at `awaiting-todd-launch`, GE06-E4-F2 cannot yet mint a truthful code-authorizing brief over implementation that does not exist.

## Selected bounded slice

```text
GE06-E4-F2 — Explanation and diagnostic inspection surface
```

Once its gate opens, this slice should do only four things:

1. consume the merged GE06-E4-F1 view-model / snapshot contract as read-only input
2. expose one inspectable path for a surfaced derived value explanation and one inspectable invalid/unavailable-choice reason path
3. keep diagnostics and blocked/computed posture visible instead of flattening them into faux success UI
4. preserve the existing evidence ceiling and known-gap language rather than inventing parity or product-viability claims

It should not broaden into rules-library browsing, source-package browsing, export breadth, shell-polish work, package-management work, or new semantic logic in the UI.

## Grounded source evidence available now
| Gate | Grounded source |
|---|---|
| GE-06 explanation/diagnostic doctrine | `technical-requirements.md` TR-06-009 requires the slice to surface why a derived value has its observed value, why a choice/prerequisite is unavailable, and which diagnostics or known-gap states still apply. |
| GE-06 minimal UI truth doctrine | `technical-requirements.md` TR-06-011 requires the first acceptable UI surface to show explanation affordances, keep diagnostics visible, and make invalid or unavailable choices inspectable without owning rules semantics. |
| GE-06 payload shape | `technical-design.md` already defines computed payload minima (`explanation references`, `failed-prerequisite / unavailable-choice outputs`, `engine diagnostics`) plus UI projection minima (`explanation affordance references`, `diagnostics visible to the UI`, `blocked/unavailable state visibility`). |
| GE06-E4-F1 upstream bridge | `artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md` and `artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md` define the bounded rules-core view-model lane that E4-F2 must consume after merge. |
| GE-07 UI inspection doctrine | `../GE-07-desktop-shell-and-modern-ux/technical-requirements.md` R2, R4, and R5 require explanation affordances, invalid-choice inspection, and visible diagnostics over an explicit UI/core boundary. |
| GE-07 shell path/default boundary | `../GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md` names the smallest additive shell subtree under `apps/desktop/`, and `artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md` recommends a read-only shell boundary over real GE-06 pilot data. |
| Live repo boundary today | `git -C /home/ubuntu/workspace/repos/codex rev-parse --short origin/develop` returns `7bc89e8`; file search shows no `src/rules_core/pilot_view_model.rs`, no `apps/desktop/` subtree, no `src-tauri/`, and no `package.json`, so the upstream view-model bridge and any shell-facing inspection surface are both still absent from merged repo truth. |

## Launch gates that remain closed
This packet must not be promoted into a code-authorizing handoff until all of the following are true:

1. `artifacts/ge06-e4-f1-merge-receipt-YYYY-MM-DD.md` exists for the real merged GE06-E4-F1 slice.
2. The live repo at merged `origin/develop` still exposes a stable view-model / snapshot contract materially compatible with the GE06-E4-F1 handoff.
3. A post-merge documentary pass confirms the smallest truthful E4-F2 implementation lane and exact write scope against the live repo state rather than against the current documentary hypothesis alone.
4. The future inspection surface still remains narrower than broad shell/product work and still preserves explanation/diagnostic visibility without local semantic recomputation.

If any gate fails, re-derive the packet instead of widening silently.

## Candidate implementation posture after gate clear
If the later post-E4-F1 audit still matches the GE07-E1 scaffold posture, the smallest likely implementation surface is:

```text
apps/desktop/src/main.tsx
apps/desktop/src/App.tsx
apps/desktop/src-tauri/src/main.rs
```

Read-only dependencies for that later run should include:

```text
src/rules_core/pilot_view_model.rs
src/rules_core/pilot_compute.rs
src/rules_core/pilot_failure.rs
tests/ge06_pilot_view_model.rs
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-readiness-closure-2026-06-22.md
programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-requirements.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/technical-design.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ui-command-boundary-requirements.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/component-surface-inventory.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/pilot-ux-flow-requirements.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-shell-scaffold-receipt-2026-06-22.md
programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-runtime-boundary-adr-input-2026-06-22.md
programs/codex/doctrine/quality-gate-policy.md
```

That candidate scope is intentionally provisional. Because neither the F1 bridge nor the additive shell subtree exists in merged repo truth yet, a later promotion pass must re-check the exact file identities before any code authority exists.

## Explicit non-goals
Do not let a future GE06-E4-F2 handoff authorize:
- changes to `src/rules_core/**`, `src/oracle_validation/**`, or `src/pcgen_import/**`
- rules-library browsing or source-package browsing surfaces
- export summary or sheet-parity work
- frontend-owned explanation logic, prerequisite logic, or value recomputation
- hiding diagnostics behind mock-clean success UI
- parity promotion above the current evidence ceiling
- product-viable or broad GE-07 claims

## Completion rule
This prebuild readiness closure is complete when it leaves no ambiguity about three facts:
- GE06-E4-F2 is downstream of the real GE06-E4-F1 merged contract, not merely its documentary brief
- the packet can be prebuilt now without counterfeit activation
- the live code-authorizing moment remains in the future, after post-E4-F1 merge evidence and a fresh scope audit
