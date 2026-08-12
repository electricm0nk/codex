# SD15-E6 Automation/Helper Boundary Handoff

## Card outcome
- evidence_class: `documentary-handoff-artifact`
- route truth: `SD-15 FLOW` card; documentary only
- expected delivery evidence for this card: this markdown artifact exists at the path named below
- not expected from this card: no repo code, no helper launch, no PR, no branch-ready claim, no merge evidence, and no helper-success claim
- downstream CODE lane warranted: `yes, but only as a narrow headless assisted-triage / receipt-draft composer`
- downstream surfaces not warranted by this card: `status-surface sync`, `tranche-closure automation`, `external-test launch automation`, `clean-machine orchestration`, or `authoritative triage auto-classification`

## Source basis
This handoff is bounded by these accepted SD-15 and live-repo authority surfaces:

1. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/README.md`
   - defines SD-15 as planning-only and documentary-first
   - names SD15-E6 as the optional later automation/helper epic
   - preserves that later handoffs must name exact repo paths, exact write scope, exact reads, exact verification, and exact non-goals

2. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/technical-design.md`
   - fixes the six cooperating SD-15 surfaces
   - preserves status-truth reconciliation as an evidence surface rather than a repo automation surface
   - defers the exact automation boundary as an unresolved design decision

3. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/risks-and-open-questions.md`
   - explicitly isolates automation-boundary uncertainty
   - warns against generic backlog collapse, counterfeit regression confidence, and status drift

4. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/intake-to-triage-mapping.md`
   - preserves the SD-11 intake schema
   - defines operator-added SD-15 classification data separately from tester-supplied and auto-captured evidence

5. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md`
   - defines the receipt-grade SD-15 evidence contract
   - requires the helper to preserve tester-supplied, auto-captured, and operator-added partitions

6. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md`
   - defines the primary SD-15 classes and visible outcome vocabulary
   - forbids collapsing unsupported, partial, blocked, or status-drift states into generic defects

7. `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-project-status-truth-reconciliation-checklist.md`
   - proves that reconciliation is a downstream evidence-and-governance lane, not a repo helper lane
   - names later README/ledger sync as separate downstream slice classes

8. Live repo instruction and implementation surfaces
   - `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
   - `/home/ubuntu/workspace/repos/codex/AGENTS.md`
   - `/home/ubuntu/workspace/repos/codex/README.md`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
   - `/home/ubuntu/workspace/repos/codex/.github/workflows/publish-tester-release.yml`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/update/deriveSd11UpdateAction.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
   - `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.ts`

## Exact target repo and workdir for any downstream CODE lane
- target repo: `/home/ubuntu/workspace/repos/codex`
- repo root grounded by `git rev-parse --show-toplevel`: `/home/ubuntu/workspace/repos/codex`
- desktop workdir for verification: `/home/ubuntu/workspace/repos/codex/apps/desktop`
- governing repo instruction files:
  - `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
  - `/home/ubuntu/workspace/repos/codex/AGENTS.md`

This documentary artifact lives outside the implementation repo at:

`/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e6-automation-helper-boundary-handoff-2026-06-30.md`

## Exact allowed write scope for this FLOW card
This FLOW card may write only:
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e6-automation-helper-boundary-handoff-2026-06-30.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/README.md` for a minimal document-map sync only

It must not write repo code, workflow files, package files, tests, README/ledger status surfaces, or launch any helper path.

## Live repo truth grounded on 2026-07-01
The repo now contains real helper-adjacent seams. This is why a narrow CODE lane is warranted at all.

Grounded live surfaces:
- `.github/workflows/publish-tester-release.yml` exists as the governed tester-release publication surface
- `apps/desktop/src/boundary/loadSd12ReleaseTruth.ts` exists as a dedicated SD-12 read-only bridge
- `apps/desktop/src/sd11/update/deriveSd11UpdateAction.ts` classifies bounded update states without applying installers or fabricating parity
- `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` and `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` already preserve build/channel/platform/update/issue-capture truth suitable for downstream SD-15 evidence packaging
- `apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts` already preserves the tester-supplied versus auto-captured split that SD-15 must not blur
- `apps/desktop/src/sd11/feedback/bug/composeBugReport.ts` already demonstrates a copyable-draft pattern that preserves evidence without claiming submission

Grounded verification results run during this FLOW card:
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts` -> passed
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/feedback/bug/composeBugReport.test.ts` -> passed
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/update/deriveSd11UpdateAction.test.ts` -> passed
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/status/createSd11WorkbenchStatus.test.ts` -> passed
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/loadSd11TesterWorkbenchSurface.test.ts` -> passed
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck` -> passed
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build` -> passed
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check` -> passed

Grounded warning surface:
- `cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/feedback/bug/submitBugReport.test.ts` currently fails because the no-transport case now resolves `blocked-incomplete` instead of the test's expected `draft-preserved`
- that transport-path mismatch is real repo truth today and is explicitly out of scope for the first SD15-E6 CODE lane below

## Decisive judgment
A downstream CODE lane is warranted, but only one.

The warranted lane is not status-sync automation and not an authority-bearing triage robot. The exact warranted lane is a headless helper that composes and validates SD-15 operator triage / regression-receipt drafts from already-captured SD-11 evidence plus explicit operator-added SD-15 fields.

Why this lane is warranted:
1. the documentary artifacts already define exact field partitions, outcome vocabulary, next-proof routing, and receipt sections
2. the live repo already contains real read-only evidence surfaces that expose build, channel, platform, workflow, release-truth, and diagnostic context
3. the repo already uses a copyable-draft pattern (`composeBugReport.ts`) that preserves evidence without claiming transport success
4. a headless draft composer can reduce operator friction while still forcing the operator to supply the authoritative SD-15 judgment fields explicitly

Why broader helper ideas are not warranted here:
- automated status-surface sync would cross from repo code into program/workspace/ledger authority surfaces that SD-15 explicitly keeps separate
- helper-driven tranche-closure verdicts would counterfeit the reconciliation gate
- auto-classifying tester narrative into authoritative SD-15 classes would outrun the accepted operator-added classification boundary
- transport, submission, or external-test orchestration would widen into separate surfaces that this slice did not earn

## Exact downstream CODE lane this handoff authorizes
Recommended governed title:

`SD15-E6 CODE: Headless SD-15 operator triage and regression-receipt draft composer`

Route class:
- `headless-only`
- `Claude Code only`
- not UI-facing in the first slice
- not Tauri-command work in the first slice

Exact bounded objective:
- consume an existing SD-11 `FeedbackEvidencePayload`
- require explicit operator-entered SD-15 fields such as primary class, outcome state, adjacent authority references, evidence sufficiency note, and next required surface
- emit a structured SD-15 draft object plus a copyable receipt/triage draft aligned to `intake-to-triage-mapping.md` and `regression-receipt-schema.md`
- preserve the tester-supplied / auto-captured / operator-added partitions visibly
- report completeness problems when required operator-added SD-15 fields are missing
- refuse to claim submission, reconciliation success, closure, or authoritative triage completion

## Exact allowed write scope for the downstream CODE lane
The first honest code slice should stay inside these exact paths only:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd15/buildSd15OperatorTriageDraft.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd15/buildSd15OperatorTriageDraft.test.ts`

Read-only comparison / grounding surfaces for that CODE lane:
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/update/deriveSd11UpdateAction.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`

If truthful completion requires edits outside the two exact SD-15 files above, stop and block the CODE card instead of widening scope by implication.

## Exact required reads for the downstream CODE lane
Before any write, the CODE lane must read:

### This handoff and SD-15 authority
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e6-automation-helper-boundary-handoff-2026-06-30.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/technical-design.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/intake-to-triage-mapping.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/regression-receipt-schema.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md`
- `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-project-status-truth-reconciliation-checklist.md`

### Repo conduct and desktop runtime truth
- `/home/ubuntu/workspace/repos/codex/CLAUDE.md`
- `/home/ubuntu/workspace/repos/codex/AGENTS.md`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/evidenceFields.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/feedback/bug/submitBugReport.test.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/sd11/update/deriveSd11UpdateAction.ts`
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/boundary/loadSd12ReleaseTruth.ts`

## Exact non-goals for the downstream CODE lane
- no `App.tsx` or other UI-surface edits
- no Tauri command additions or runtime registration changes
- no GitHub issue-transport, submission, or label-routing work
- no repair of `submitBugReport.ts` or `submitBugReport.test.ts`; that is a separate readiness/repair lane if later needed
- no edits to `.github/workflows/`, repo README, program README, or the execution ledger
- no auto-classification from tester narrative alone; operator-selected SD-15 class and outcome state must remain explicit inputs
- no helper-generated closure verdict, reconciliation-state verdict, external-test launch, or clean-machine orchestration
- no silent fallback from missing fields to invented values; missing operator data must surface as problems or explicit absence markers

## Exact verification commands for the downstream CODE lane
The CODE lane must run these exact commands, at minimum:

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd15/buildSd15OperatorTriageDraft.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/feedback/bug/composeBugReport.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/update/deriveSd11UpdateAction.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/status/createSd11WorkbenchStatus.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && tsx src/sd11/loadSd11TesterWorkbenchSurface.test.ts
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run typecheck
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run build
cd /home/ubuntu/workspace/repos/codex/apps/desktop && npm run tauri:check
```

Execution-floor notes:
- `submitBugReport.test.ts` is not part of the verification floor for this slice because it is already red in grounded repo truth and outside the authorized write scope
- if the new SD-15 helper test cannot be made red first and green second without touching out-of-scope files, block the card

## Claude-only execution and receipt requirements for the downstream CODE lane
The later CODE card must say all of the following plainly:
- required harness: `Claude Code only`
- if Claude Code cannot be launched truthfully, block the card instead of editing through Hermes
- leave a durable `claude-execution-receipt` comment before closeout
- completion class: `pr-created` truth only
- required receipt fields:
  - launcher identity
  - handoff path
  - Claude session id if available
  - Claude model if available
  - turn count, cost, and duration if available
  - repo, worktree, branch, base SHA, head SHA
  - exact changed files
  - RED summary and GREEN verification results
  - exact commands run
  - PR URL and verified PR state
- if any receipt field is unavailable, record `unknown`; do not fabricate

## Launch substrate requirement for the downstream CODE lane
The shared checkout is not acceptable for direct execution right now.

Grounded current shared-checkout truth on 2026-07-01:
- current branch: `feat/sd13-e6-f11-support-state-debt-presentation`
- dirty file: `README.md`
- untracked path: `apps/desktop/src-tauri/gen/`
- current accepted remote base reverified during this FLOW card: `origin/develop` at `454a92ed67578124d88232b130a832de6ed571df`

Therefore the CODE lane must launch from a fresh worktree off the accepted `origin/develop` tip reverified at execution time. It must not edit the shared checkout in place.

Recommended launch substrate for the CODE card:
- worktree: `/home/ubuntu/workspace/worktrees/codex-sd15-e6-helper-drafts`
- branch: `feat/sd15-e6-helper-drafts`
- PR target: `develop`

If that worktree/branch naming collides at execution time, the worker may choose a deterministic variant, but it must preserve the same bounded objective and write scope.

## Acceptance evidence for this FLOW card
This card is complete when:
- this file exists at `/home/ubuntu/workspace/programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/sd15-e6-automation-helper-boundary-handoff-2026-06-30.md`
- the file states whether a downstream CODE lane is warranted at all
- the file narrows the warranted lane to an exact helper boundary instead of vague automation prose
- the file names exact repo/workdir, exact downstream write scope, exact required reads, exact verification commands, exact non-goals, and Claude-only receipt requirements
- the file explicitly refuses helper authority over status sync, closure, external-test launch, or automatic triage judgment
- the file claims no repo code or helper success from this documentary FLOW card itself
