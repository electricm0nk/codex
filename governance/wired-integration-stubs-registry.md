---
title: Wired Integration Stubs Registry
stc_id: GOV-WIRED-INTEGRATION-STUBS-REGISTRY
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-20
canonical_source: ~/workspace/repos/codex/governance/wired-integration-stubs-registry.md (this file)
workspace_citation: ~/workspace/governance/docs/wired-integration-stubs-registry.md
supersedes: (none — first issuance)
upstream_targets:
  - ./no-stub-mvp-doctrine.md (parent doctrine; in-repo)
  - ~/workspace/governance/doctrine/no-stub-mvp-doctrine.md (parent doctrine; workspace citation)
  - ~/workspace/governance/agents/CLAUDE.md
  - ~/workspace/governance/agents/AGENTS.md
related_artifacts:
  - ./no-stub-mvp-doctrine.md (parent doctrine; in-repo)
date: 2026-07-20
---

# Wired Integration Stubs Registry

The doctrine of record for any given stub. Per `no-stub-mvp-doctrine.md` §"Stubs are the exception, not the rule," no stub may ship without an entry here.

## How to use this registry

When a stub is proposed (by an operator directive, a planned cycle, or a defensive audit find), the cycle authoring it must:

1. Add a numbered entry below with all required fields filled.
2. Reference the entry from the bundle's `risks-and-open-questions.md` if the stub has a remediation cycle, or document the permanent-exception rationale inline.
3. If the bundle is closed without remediation, the entry remains here indefinitely as a permanent exception, and the audit grep remains scoped to exclude it via `epic-breakdown.md` §"Audit exclusions."

The operator's verbatim directive is required for every entry — exceptions are operator-granted, not self-asserted.

## Registry entries

### 0001 — Browser-preview fallback in character hub runtime

- **File / line:** `apps/desktop/src/characterHub/characterHubRuntime.ts:17-18`
- **Stub pattern:** Returns `buildPreviewListSurface()` (sample character) when `!hasTauriRuntime()`.
- **Justification (operator verbatim, 2026-07-20):** Browser preview needs a sample character so the Load → sheet flow stays walkable without the desktop backend. The fallback path is the test surface for mappers without a Tauri runtime; the production desktop path uses real `loadListSavedCharacters()`. Permanent exception.
- **Audit-grep impact:** The browser-preview branch's `return buildPreviewListSurface();` is permitted to remain in the diff indefinitely. No defensive cleanup cycle needed.
- **Bundle-of-record:** SD-23 (registry created with the bundle).
- **Remediation cycle:** None — permanent exception.
- **Status:** Accepted 2026-07-20.

### 0002 — `StubAdapter` future-rule-system placeholder

- **File / line:** `apps/desktop/src-tauri/src/stub_adapter.rs` (whole file — doc comment, the `would_render_message` builder, and every trait-method arm that surfaces it via a diagnostic/`error` field/`Err`).
- **Stub pattern:** Every `RuleSystemAdapter` method on `StubAdapter` reports `"Would render for system {system_id}; not yet implemented"` (the wired-integration doctrine's forbidden "Would ..." pattern, matched by the dual-audit grep's `not yet implemented` bucket) instead of computing a real result, for any `rule_system_id` this codebase has not built a real adapter for yet.
- **Justification (operator-pinned, per `docs/release/SD-25-ui-evaluation-defect-closure/epic-breakdown.md` §Criterion 3.3 and `cycles/3_3.md`):** "returns 'Would render for system X; not yet implemented' results. Wired-integration doctrine forbids 'Would …' strings in *shipping code* — this stub gets an entry in `governance/wired-integration-stubs-registry.md` with the operator-granted justification (the future-system rollout is operator-pinned)." Criterion 3.4's Tauri command surface must have a `dyn RuleSystemAdapter` to hand back for a not-yet-built rule system's id rather than refuse to route at all; `StubAdapter` is that seam's honest placeholder until a real adapter for that system lands, at which point that system's real adapter replaces this dispatch entry — it never silently swaps in fabricated data.
- **Audit-grep impact:** any `not yet implemented` / `Would` hit inside `stub_adapter.rs` is permitted to remain in the diff. No defensive cleanup cycle needed for this file; the exclusion is file-scoped, not project-wide.
- **Bundle-of-record:** SD-25, Epic 3 "Character Hub as Hub of Hubs," criterion 3.3.
- **Remediation cycle:** None per rule system that never gets a real adapter; superseded per-system the moment that system's real `RuleSystemAdapter` implementation lands (mirrors `Pf1Adapter`'s criterion 3.2 precedent) and criterion 3.4's registry routes that `rule_system_id` to the real implementation instead.
- **Status:** Accepted 2026-07-21.

(Entries 0003-000n reserved for operator-directed exceptions. Any accidental stub found by the per-cycle audit goes into `risks-and-open-questions.md` as a Wired Integration Cleanup candidate, not here — the registry is operator-granted only.)
