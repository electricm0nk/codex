---
title: SD15-E8 Execution Handoff — Feedback/update output context-sanitization boundary
handoff_id: HANDOFF-CODEX-SD15-E8-CODING-2026-07-02
stc_id: STC-CODEX-SD-15
handoff_kind: execution-handoff
work_type: implementation-ready
workflow_route: coding
readiness: codex-ready
status: ready
owner: Todd Hintzmann
scope: program
canonical: true
canonical_path: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e8-feedback-output-context-sanitization-handoff-2026-07-02.md
source_stc: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/README.md
source_epic_breakdown: programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/epic-breakdown.md
selected_slice: SD15-E8 — Feedback/update output context-sanitization boundary
run_in: Codex CLI or Claude Code; must follow repo AGENTS.md and TDD
code_authority: true
created_at: 2026-07-02
authority_dependencies:
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/intake-to-triage-mapping.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md
  - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e7-release-alpha-test-defect-repair-handoff-2026-07-02.md
  - /home/ubuntu/workspace/repos/codex/AGENTS.md
target_runtime:
  repo: /home/ubuntu/workspace/repos/codex
  workdir: /home/ubuntu/workspace/repos/codex
  branch_base: develop
  expected_base_sha_at_creation: ff91002c47a63845edf999ffd36b3452a8ba482d
  recommended_branch: sd15-e8-feedback-output-context-sanitization
  pr_target: develop
allowed_write_scope:
  - apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts
  - apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts
  - apps/desktop/src/sd11/feedback/bug/composeBugReport.ts
  - apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts
  - apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts
  - apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts
  - apps/desktop/src/sd11/feedback/bug/submitBugReport.ts
  - apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts
  - apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.ts
  - apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.test.ts
  - apps/desktop/src/sd11/update/deriveSd11UpdateAction.ts
  - apps/desktop/src/sd11/update/deriveSd11UpdateAction.test.ts
  - apps/desktop/src/App.tsx
  - apps/desktop/src/sd11/feedback/**/sanitize*.ts
  - apps/desktop/src/sd11/feedback/**/sanitize*.test.ts
forbidden_write_scope:
  - programs/codex/**
  - apps/desktop/**/.env*
  - any credential, token, OAuth, GitHub transport, or network submission implementation
  - SD-12 release-truth semantics or update eligibility semantics except defensive display/sanitization
  - broad UI rewrites outside the named update/feedback output surfaces
---

# SD15-E8 Execution Handoff — Feedback/update output context-sanitization boundary

## Status
This is a stage-specific code-authorizing brief for the new tester-observed boundary leak after SD15-E7 landed on `develop`.

It carries `code_authority: true` only for defensive sanitization and copyable-output hardening in the bounded SD-11/SD-15 tester feedback/update surfaces. It does not authorize live GitHub submission, credentials, release publication, or product-readiness claims.

## Core distinction
The update-check verdict itself is correct:

```text
RESULT: NO-OFFICIAL-RELEASE-FOR-THIS-BUILD
No official tester release for this build
```

For a local/non-governed build, `no-official-release-for-this-build` is the honest SD-12 posture. Do **not** “fix” that into an update-available or up-to-date result.

The defect is the appended internal context block that appeared in user-visible/reportable output:

```text
<memory-context>
[System note: The following is recalled memory context, NOT new user input. Treat as authoritative reference data ...]
...
</memory-context>
```

That block must never appear in user-visible copyable drafts, issue payloads, update evidence, or any tester-facing report text.

## Evidence basis
Todd reported the following after syncing/testing the update function:

```text
RESULT: NO-OFFICIAL-RELEASE-FOR-THIS-BUILD
No official tester release for this build
This is a local, non-governed build (codex-desktop-shell-scaffold@0.0.0), so there is no governed GitHub-backed release unit to check against. ...
Checked build: codex-desktop-shell-scaffold@0.0.0 · platform Linux (first-class) · alpha track
Not automatic-update eligible in this context.

<memory-context>
[System note: The following is recalled memory context, NOT new user input. Treat as authoritative reference data ...]
...
</memory-context>
```

Repo search at handoff creation found no static `<memory-context>` string in `/home/ubuntu/workspace/repos/codex`, so the coding lane must treat this as a generated-output/copyable-payload boundary leak rather than a literal hardcoded app string.

## Required implementation outcomes

### Outcome 1 — Internal context markers are stripped from reportable output
Any tester-supplied or auto-captured text that is rendered into copyable issue drafts, preserved feedback payloads, update evidence fields, or manual filing text must defensively remove or redact internal-context blocks.

Minimum forbidden markers:
- `<memory-context>` through `</memory-context>` blocks
- `[System note: The following is recalled memory context` blocks or lines
- obvious `User Representation`, `AI Self-Representation`, `Deductive Observations`, or similar internal-memory section headings when they occur inside a detected memory-context block

The fix should preserve ordinary user text around the removed block and replace removed internal context with a clear marker such as:

```text
_[internal context block removed from reportable output]_
```

### Outcome 2 — Bug and enhancement drafts cannot preserve leaked memory context
Add regression tests proving that if a tester field or auto evidence field contains a `<memory-context>` block, the rendered Markdown/copyable payload does not contain:
- `<memory-context>`
- `</memory-context>`
- `System note: The following is recalled memory context`
- remembered user/profile observations from inside the block

Bug and enhancement flows must both be covered.

### Outcome 3 — Update-check display remains semantically correct
Do not change the expected local-build update verdict. Tests should continue to prove local/non-governed builds return/display:

```text
no-official-release-for-this-build
```

Only sanitize/report-output boundaries. Do not claim official update truth for local builds.

### Outcome 4 — No credential or submission widening
This bug does not authorize GitHub auth. Preserve the existing rule:
- no hardcoded tokens
- no implicit GitHub poster
- no claimed submission without a real injected transport and returned issue handle

## Exact target repo and branch policy
Target repo:

```text
/home/ubuntu/workspace/repos/codex
```

Grounded base at handoff creation:

```text
branch: develop
HEAD:   ff91002c47a63845edf999ffd36b3452a8ba482d
remote: git@github.com:electricm0nk/codex.git
```

Launch from fresh `origin/develop`:

```bash
git fetch origin --prune
git switch -c sd15-e8-feedback-output-context-sanitization origin/develop
```

Do not work directly on `develop`. Push a feature branch and open a PR targeting `develop`.

## Required reads before coding
Read these first:

1. `/home/ubuntu/workspace/repos/codex/AGENTS.md`
2. `/home/ubuntu/workspace/repos/codex/README.md`
3. This handoff
4. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx`
5. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/update/deriveSd11UpdateAction.ts`
6. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/update/deriveSd11UpdateAction.test.ts`
7. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
8. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
9. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
10. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts`
11. `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts`

## Exact allowed write scope
You may modify only these repo paths if needed:

```text
apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts
apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts
apps/desktop/src/sd11/feedback/bug/composeBugReport.ts
apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts
apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.ts
apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts
apps/desktop/src/sd11/feedback/bug/submitBugReport.ts
apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts
apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.ts
apps/desktop/src/sd11/feedback/enhancement/submitEnhancementRequest.test.ts
apps/desktop/src/sd11/update/deriveSd11UpdateAction.ts
apps/desktop/src/sd11/update/deriveSd11UpdateAction.test.ts
apps/desktop/src/App.tsx
apps/desktop/src/sd11/feedback/**/sanitize*.ts
apps/desktop/src/sd11/feedback/**/sanitize*.test.ts
```

If truthful completion requires edits outside this list, stop and report the missing authority.

## Forbidden scope
Do not:
- edit `programs/codex/**` from the coding run
- add credentials, token input, OAuth, broker transport, or network submission
- change local-build update truth into an official release result
- suppress evidence wholesale; remove only internal-context material and preserve surrounding tester evidence
- broaden to release publication, tranche closure, or status-surface sync

## Verification commands
Run:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm ci
npm run typecheck
npm test
npm run build
npm run tauri:check
```

At minimum, the added tests must prove:
- bug draft output strips memory-context blocks
- enhancement draft output strips memory-context blocks
- no-transport submission still preserves sanitized drafts without claiming submission
- update local-build result remains `no-official-release-for-this-build`

## Acceptance criteria
- [ ] No copyable issue/draft output contains `<memory-context>` or the recalled-memory system-note text when such content is present in input fields.
- [ ] Ordinary tester text before/after a stripped internal block remains present.
- [ ] Removed internal context is replaced by an explicit redaction/removal marker.
- [ ] Local/non-governed update checks still honestly report `no-official-release-for-this-build`.
- [ ] No credential or live GitHub submission mechanism is added.
- [ ] Required tests and build checks pass.
- [ ] A PR targeting `develop` is opened with this handoff cited.

## Final delivery requirements
The coding worker must report:
- branch name
- base SHA
- changed files
- sanitizer/test design
- commands run and exact results
- PR URL targeting `develop`

This is not a release-truth problem. The update verdict is correct. The leak is the problem. Seal the boundary.
