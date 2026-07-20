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

(Entries 0002-000n reserved for operator-directed exceptions. Any accidental stub found by the per-cycle audit goes into `risks-and-open-questions.md` as a Wired Integration Cleanup candidate, not here — the registry is operator-granted only.)
