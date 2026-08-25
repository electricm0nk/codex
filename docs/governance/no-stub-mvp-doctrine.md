---
title: Wired Integration Doctrine (No Stubs, No Mock Data, No MVP Hand-Waves)
stc_id: GOV-WIRED-INTEGRATION-DISCIPLINE
canonical: true
owner: Todd Hintzmann
scope: universal
status: active
review_state: accepted
last_reviewed_at: 2026-07-20
canonical_source: ~/workspace/repos/codex/docs/governance/no-stub-mvp-doctrine.md (this file)
workspace_citation: ~/workspace/governance/doctrine/no-stub-mvp-doctrine.md
supersedes: (none — first issuance)
upstream_targets:
  - ~/workspace/governance/agents/CLAUDE.md
  - ~/workspace/governance/agents/AGENTS.md
  - ~/workspace/repos/codex/AGENTS.md
  - ~/workspace/repos/codex/CLAUDE.md
  - programs/codex/requirements/SD-N-.../epic-breakdown.md (every existing and future bundle)
  - programs/codex/requirements/SD-N-.../workflow-instruction files at ~/workspace/
related_artifacts:
  - ./wired-integration-stubs-registry.md (operator-granted exceptions; sibling, in-repo)
  - ~/workspace/governance/identifier-discipline.md (sibling doctrine — names must not bundle-tag)
  - ~/workspace/governance/spec-domain-lifecycle.md (sibling doctrine — closed bundles stay closed)
  - ~/workspace/governance/agents/CLAUDE.md
  - ~/workspace/governance/agents/AGENTS.md
date: 2026-07-20
---

# Wired Integration Doctrine — No Stubs, No Mock Data, No MVP Hand-Waves

## Operator's Law (verbatim, 2026-07-20)

> "No more stub work. No more mock data. I expect everything from this point forward to be fully wired."

This is the operator's recorded directive at the launch of SD-23. It is a base-level governance requirement that applies to every SD-N bundle launching on or after 2026-07-20. Earlier bundles (SD-21, SD-22) are not retroactively invalidated; their stubs are remediated in the SD-23 scope.

## Stubs are the exception, not the rule

Stubs in shipped code are forbidden by default. The presence of a stub requires explicit operator approval recorded in the doctrine's Stubs Registry (`docs/governance/wired-integration-stubs-registry.md` per the bundle that introduces the exception). Each registry entry carries:

- The file path and approximate line number.
- The stub pattern (e.g., empty `onClick`, "Would …" return string, fixture-only data in production path).
- The operator's verbatim justification for the exception.
- The bundle or cycle where the stub will be remediated, or the explicit "permanent exception" rationale (e.g., "OS API does not exist yet on the target platform; the call returns a structured error and the UI surfaces it").
- A remediation-tracker entry on the active bundle's `risks-and-open-questions.md` if the exception has a future cycle.

Stubs the operator did not design or approve are treated as accidental debt and remediated by the next Wired Integration Cleanup epic (in SD-23, Epic 3 — the bundle's primary wired-integration deliverable; in future bundles whose scope is not primarily about wired-integration, by a defensive cleanup pass at the bundle's identifier-cleanup or wired-integration-cleanup epic as the bundle's epic structure dictates). The doctrine does not pin a single epic number — the cleanup epic is whatever bundle-internal epic handles stubs, and the Wired Integration Cleanup epic's criteria call it out by name. The operator's count of designed stubs as of 2026-07-20: 2-3. Anything beyond that count is accidental debt.

The companion skill `wired-integration-discipline` carries the Stubs Registry authoring template; the doctrine-of-record for any given stub lives in the registry, not in source-code comments. Source-code comments naming a stub are themselves a forbidden pattern — the comment leaks governance into shipping code, which is the same anti-pattern as identifier-discipline's bundle-tag leak.

## Core doctrine

A code path that ships in a release package must actually do what it claims to do. If a function exists, it has a real implementation. If a UI affordance exists, it has a real handler. If a persistence claim exists, it has a real write to a real backend. The presence of the affordance is itself the commitment to the behavior; an "Add Weapon" button with no `onClick` is a lie the codebase tells the user, and lies compound faster than code.

The companion to this doctrine is `~/workspace/governance/identifier-discipline.md`: identifier-discipline forbids the *names* of code from being bundle-tagged (no `sd21_`, no `SD16_UI_*`, no `t_<hex>` in source). Wired-integration-discipline forbids the *behavior* of code from being incomplete when it ships (no empty handlers, no "would-have-done" returned-as-string, no fixture-only data in production paths).

## Forbidden patterns in shipping code

A code path that lands in a release package must not contain any of the following, in any non-test file under `apps/desktop/`, `apps/desktop/src-tauri/`, `src/`, or any other implementation tree:

1. **No empty event handlers on user-facing affordances.** Buttons, links, menu items, and keybindings that are rendered to the user must have a real handler that does real work. Forbidden:
   - `onClick={() => {}}`
   - `onClick={undefined}` (a deliberate no-op signal)
   - `<button>` with no `onClick` and no `<form>` submission context
   - `void functionThatReturnsPromise` where the promise is intentionally dropped
2. **No "would have" / "will simulate" return strings.** Function return values that describe what the real implementation *would* do, instead of doing it. The actual `createCampaign` function in `apps/desktop/src/campaign/campaignModel.ts:111-113` returns `"Would create Drive folder …"` — that string is the canonical example of a stub masquerading as a value. The real function must create the folder, or it must fail with a real error.
3. **No `STUB`, `MOCK`, `placeholder`, `not yet implemented`, `todo`, `fixme`, `hack`, `temporary` in identifiers, comments, or returned strings of shipping code.** (Test fixtures and audit trail text in doctrine docs are exempt — the rule is about shipped behavior, not documentation history.)
4. **No "Drive folder is really just a local path" half-truths in shipping code.** Either the integration talks to Drive, or the function names and doc-comments say "local folder." A function called `syncCampaignDriveArtifacts` that writes to a local path is named wrong; rename or replace.
5. **No fixture-only data crossing into production code paths.** `__mocks__` directories, `vi.mock(...)` calls, `mockResolvedValue`, and `.mockReturnValue(` calls may exist in test files but must not be imported by shipping code. A `getSpells()` function used by a picker UI must return real corpus data, not a hardcoded array of three spells.
6. **No `success: true` returns from operations that did not actually do the work.** Either the operation completed and the return value reflects that, or it failed and the return value is an error. A `success: true` that lies about what happened is worse than a thrown exception.

## The honest failure mode (and why it's better than the stub)

A function that fails honestly is better than a function that pretends to succeed. The rule for shipping code:

- If a real operation cannot complete (no folder configured, network down, permission denied, target deleted), the function returns a structured failure object with a `reason` field and the calling UI surfaces that reason to the user.
- The UI never displays "Done" when the underlying operation returned a failure object.
- The local state never reflects the success side of an operation that failed — the campaign record is not "saved" if the persistence call returned an error.

This is the test that distinguishes a wired integration from a stub: a stub returns success-shaped data on the happy path and nothing on the sad path; a wired integration returns success-shaped data only when the real backend actually succeeded, and a structured error otherwise.

## What "fully wired" looks like in practice

A user-facing affordance is "fully wired" if all four of the following are true:

1. The handler executes a real call to a real backend (Tauri command, OS API, or local persistence layer).
2. The handler's `await` chain returns a result object that reflects the actual outcome of the call — success, failure with a reason, or a typed domain result.
3. The UI updates to reflect the actual outcome. A successful add changes the visible list. A failed add shows a visible error message and leaves the visible list unchanged.
4. The state layer re-fetches or re-derives from the source of truth after the operation, so the next render is consistent with the new state.

A picker that opens, accepts a selection, calls a Tauri command, and then does not refresh the character detail is *not* fully wired — even if the Tauri command succeeded. The "Add Weapon" button that silently no-ops is the canonical failure case that produced this doctrine.

## The campaign/Drive simplification (operator directive 2026-07-20)

The original SD-21 Epic 2 scope was "Google OAuth + Drive API + Drive folder." The operator's 2026-07-20 simplification:

- No Google OAuth. The app does not authenticate to Google.
- No Drive API. The app does not call Drive REST endpoints.
- The "Drive folder" is a local folder the user picks via the OS folder picker. If the user happens to point that folder at a Google Drive desktop sync client, the user's machine (not this app) handles the sync.

This collapses three planned stubs into a single local-folder contract:

1. `settings/googleDrive.ts` exposes a folder-path config (already a localStorage key).
2. The picker is a real OS folder-picker via Tauri (not a text input).
3. `syncCampaignDriveArtifacts` is renamed to reflect that it writes to a local folder, and its doc-comment is updated to say so.

The member-invite stub (`invited: true` hardcoded, no actual invite flow) is **deleted entirely**. The `CampaignMember.invited` field is removed from the data model; if member invites are added later, they will be a different feature with a different shape. Shipping a `Would invite: a@b.com` string is exactly the kind of lie this doctrine forbids.

## Per-cycle audit (load-bearing control)

The auditable unit is the diff between `develop` (or the slice's base branch) and the slice's branch at cycle commit time. The companion skill `wired-integration-discipline` defines the exact audit commands; the canonical checks are:

1. `git diff --unified=0 develop...HEAD -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.rs' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS`
2. `git diff --unified=0 develop...HEAD -- 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined' || echo OK_NO_NOOP_HANDLERS`
3. `git diff --unified=0 develop...HEAD -- 'apps/desktop/**/*.{ts,tsx,jsx,rs}' ':!**/__tests__/**' ':!**/*.test.*' | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__' || echo OK_NO_MOCK_LEAKS`
4. `git diff --unified=0 develop...HEAD -- 'apps/desktop/**/*.{ts,tsx}' 'src/**/*.rs' | grep -nE '"Would [^"]*"' || echo OK_NO_WOULD_STRINGS`

Each cycle commits these four commands' output as part of the cycle receipt (in the kanban card's comments stream, per the `kanban-claude-code-execution-receipt` skill's pattern). A cycle that ships code which fails any of the four checks cannot be marked `complete`; the cycle returns to the cycle-backlog with the audit output as the failure reason.

## Acceptance criterion shape for future bundles

Every SD-N epic that adds or modifies shipping code carries at least one acceptance criterion in the form:

> **Wired-Integration Audit:** the slice's diff against the base branch passes the four-check audit defined in `wired-integration-discipline/SKILL.md` §"Per-cycle audit." Audit output is captured in the cycle receipt.

This criterion is not optional and is not waivable except by an explicit operator override recorded in `risks-and-open-questions.md`. Bundles whose scope is purely documentary, research, or governance are exempt — the audit applies to code-bearing cycles only.

## Worked example (SD-23 Epic 3 — Wired Integration Cleanup, target state)

The Epic 3 acceptance criteria cover:

- **`campaignModel.ts` rewrite.** `createCampaign` no longer returns a `driveActionSummary` string. It writes the campaign record to localStorage, calls the real OS-folder-creation Tauri command, writes the `.config/<name>.json` and markdown asset files at the configured folder, and returns a `Campaign` object with a `syncResult: { ok: boolean, campaignFolderPath?: string, error?: string }` field. UI surfaces the real outcome.
- **`campaignManagerAccessGate` rename + simplification.** The boolean-only gate collapses to a single `isLocalFolderConfigured()` call. The doc comment is rewritten to remove the "no real Google OAuth / Drive API integration" sentence — that sentence is the doctrinally forbidden kind of inline note.
- **`syncCampaignDriveArtifacts` rename.** Function renamed to `writeCampaignLocalFolderArtifacts`. The doc comment is rewritten to drop the "write-through mirror of the localStorage data" framing and say plainly what the function does: writes the campaign record and assets to the configured local folder.
- **`CampaignMember.invited` deletion.** Field removed from the `Campaign` type. `createCampaign` no longer maps member emails to `{email, invited: true}`; the data model becomes `{email}` only or the members array becomes optional.
- **Test-surface reflection.** The campaign test fixtures are updated to assert the new return shape. The `campaignManagerAccessGate.test.ts` is updated to call `isLocalFolderConfigured()` not `computeCampaignManagerAccessGate(localFolderConfigured: boolean)`.
- **Picker / character-mutate audit.** The `addEquipmentSelection` / `addSpellSelection` Tauri commands and the picker UI components pass the four-check audit on first commit. No empty `onClick`, no "would-add" strings, no fixture data in production paths.

## Why this is load-bearing

- **The user-trust problem.** A user who clicks "Add Weapon" and sees nothing happen forms the conclusion that the app is broken, and they form it faster than they form any other conclusion. The cost of that conclusion is the user's continued use of the app.
- **The cost-of-purification compounds.** Cleaning up one stub at a time is mechanical work the loop does well. Cleaning up a thousand stubs spread across the codebase once they have accreted into a release package is a dedicated refactor epic. We are at the early-curation phase; the cost is bounded. In six months it is not.
- **The MVP pattern as a debt vehicle.** The MVP pattern ("we'll wire it up properly later") is a known debt vehicle. Every stub that ships is a future cycle's tax. This doctrine closes that door: stubs do not ship.
- **The operator onboarding pressure.** The operator plans to onboard additional contributors after the next release. A second contributor reading the codebase forms a mental model of "this is how we ship things" — and the mental model formed from a stub-heavy codebase is "we ship things that don't work." This doctrine prevents that.

## Operator-recorded open calls (deferred from first issuance)

- **What is the audit-failure recovery path when a cycle returns from the field with stub-shaped code?** The standing memory is: the cycle returns to the cycle-backlog with the audit output as the failure reason; the operator may override and accept the stub with explicit recorded justification. No automatic retry loop is configured for audit failures; the audit is a hard gate, not a retry-trigger.
- **Should the audit be a separate CI check in addition to the per-cycle grep?** Standing memory: per-cycle grep at commit time is sufficient for the early-curation phase; CI integration is a follow-on hardening cycle. Operator may promote earlier.
- **Test-only mock libraries (`vi.mock`, `jest.mock`) inside shipping modules.** Standing memory: forbidden — the four-check audit excludes `__tests__` and `*.test.*` files but does not exclude inline mocks living in the same module as the shipping code. The rule is: mocks live in dedicated test files; shipping modules import real implementations.

## Cross-reference

- `./wired-integration-stubs-registry.md` — operator-granted stub exceptions registry (in-repo, sibling).
- `~/workspace/governance/docs/wired-integration-stubs-registry.md` — workspace citation copy of the Stubs Registry.
- `~/workspace/governance/doctrine/identifier-discipline.md` — sibling doctrine, "names must not bundle-tag."
- `~/workspace/governance/spec-domain-lifecycle.md` — sibling doctrine, "closed bundles stay closed; bundles don't own code."
- `~/workspace/governance/agents/CLAUDE.md`, `~/workspace/governance/agents/AGENTS.md` — durable conduct surface; this doctrine is appended as Non-Negotiable Rule #6 (workspace-root numbering).
- `~/workspace/governance/doctrine/no-stub-mvp-doctrine.md` — workspace citation copy. Same content; lives outside the repo for workspace-root loads.
- `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/SKILL.md` — procedural skill; loaded by the SD-23 loop and every subsequent bundle's code-bearing cycles.
- `programs/codex/requirements/SD-23-character-mutation-and-wired-integration/epic-breakdown.md` §"Epic 3 — Wired Integration Cleanup" — first worked example under this doctrine.
- `programs/codex/requirements/SD-21-campaign-manager-and-persistence/epic-breakdown.md` §"Epic 2 — Campaign Manager + Drive Persistence" — the SD-21 Epic 2 stub sources this doctrine was authored against.
