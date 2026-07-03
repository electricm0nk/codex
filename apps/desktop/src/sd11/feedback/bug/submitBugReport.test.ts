import { submitBugReport, renderCopyableBugPayload } from './submitBugReport';
import { composeBugReport, type ComposedBugReport } from './composeBugReport';
import { assembleFeedbackEvidence } from '../evidence';
import { assert, assertEqual } from '../../../testSupport/asserts';
import { makeSurface } from '../../../testSupport/makeSurface';

function completeComposed(): ComposedBugReport {
  const surface = makeSurface();
  return composeBugReport({
    title: 'Preview crashes on baseline AC',
    payload: assembleFeedbackEvidence({
      flow: 'bug',
      surface,
      testerInput: {
        observedBehavior: 'crash',
        expectedBehavior: 'no crash',
        reproductionSteps: '1. open 2. compute',
      },
    }),
  });
}

function incompleteComposed(): ComposedBugReport {
  const surface = makeSurface();
  return composeBugReport({
    title: 'Preview crashes',
    payload: assembleFeedbackEvidence({
      flow: 'bug',
      surface,
      testerInput: { observedBehavior: 'crash' },
    }),
  });
}

async function main() {
  await incompleteReportIsBlockedNotSubmitted();
  await noTransportPreservesDraftWithoutClaimingSuccess();
  await transportReceivesStructuredDraftPayload();
  await transportThrowPreservesDraft();
  await transportFailurePreservesDraft();
  await okWithoutHandleNeverCountsAsSuccess();
  await nonHttpIssueHandleNeverCountsAsSuccess();
  await realHandleIsTheOnlySuccessPath();
  copyablePayloadCarriesTheStructuredReport();
  await failedSubmissionCopyablePayloadMatchesTheStructuredDraft();
}

async function incompleteReportIsBlockedNotSubmitted() {
  let transportCalled = false;
  const outcome = await submitBugReport({
    composed: incompleteComposed(),
    transport: async () => {
      transportCalled = true;
      return { ok: true, issueUrl: 'https://example/issues/1', issueNumber: 1 };
    },
  });
  assertEqual(outcome.status, 'blocked-incomplete', 'incomplete report is blocked, not submitted');
  assertEqual(outcome.claimedSubmitted, false, 'incomplete report does not claim success');
  assertEqual(outcome.resultHandle, null, 'no result handle for a blocked report');
  assertEqual(transportCalled, false, 'transport is not called for an incomplete report');
  assert(outcome.copyablePayload.length > 0, 'copyable payload preserved even when blocked');
}

async function noTransportPreservesDraftWithoutClaimingSuccess() {
  const outcome = await submitBugReport({ composed: completeComposed(), transport: null });
  assertEqual(outcome.status, 'draft-preserved', 'no transport preserves the draft');
  assertEqual(outcome.claimedSubmitted, false, 'no transport never claims success');
  assertEqual(outcome.resultHandle, null, 'no transport yields no result handle');
  assert(outcome.message.includes('manual GitHub issue filing'), 'no transport directs the tester to manual filing');
  assert(outcome.message.includes('none is claimed'), 'no transport message refuses claimed submission');
  assert(outcome.copyablePayload.includes('Preview crashes on baseline AC'), 'copyable payload carries the title');
}

async function transportReceivesStructuredDraftPayload() {
  const composed = completeComposed();
  let received: ComposedBugReport['draft'] | null = null;
  const outcome = await submitBugReport({
    composed,
    transport: async (draft) => {
      received = draft;
      return { ok: true, issueUrl: 'https://github.com/x/codex/issues/7', issueNumber: 7 };
    },
  });

  assertEqual(outcome.status, 'submitted', 'transport with handle submits');
  assertEqual(received, composed.draft, 'transport receives the structured draft object');
  assertEqual(received!.issueType, 'bug', 'transport draft carries issue type');
  assert(received!.sections.some((section) => section.heading === 'Observed behavior'), 'transport draft carries structured sections');
  assert(received!.labels.includes('bug'), 'transport draft carries labels');
}

async function transportThrowPreservesDraft() {
  const outcome = await submitBugReport({
    composed: completeComposed(),
    transport: async () => {
      throw new Error('network down');
    },
  });
  assertEqual(outcome.status, 'draft-preserved', 'transport throw preserves the draft');
  assertEqual(outcome.claimedSubmitted, false, 'transport throw never claims success');
  assert(outcome.message.includes('network down'), 'transport error surfaced in the message');
}

async function transportFailurePreservesDraft() {
  const outcome = await submitBugReport({
    composed: completeComposed(),
    transport: async () => ({ ok: false, error: 'unauthorized' }),
  });
  assertEqual(outcome.status, 'draft-preserved', 'transport failure preserves the draft');
  assertEqual(outcome.claimedSubmitted, false, 'transport failure never claims success');
  assert(outcome.message.includes('unauthorized'), 'transport failure detail surfaced');
}

async function okWithoutHandleNeverCountsAsSuccess() {
  const outcome = await submitBugReport({
    composed: completeComposed(),
    // Counterfeit guard: ok=true but no real issue handle.
    transport: async () => ({ ok: true }),
  });
  assertEqual(outcome.status, 'draft-preserved', 'ok-without-handle is not a real submission');
  assertEqual(outcome.claimedSubmitted, false, 'ok-without-handle does not claim success');
  assertEqual(outcome.resultHandle, null, 'ok-without-handle yields no result handle');
}

async function nonHttpIssueHandleNeverCountsAsSuccess() {
  // Counterfeit guard, mirroring the enhancement flow: a transport that returns
  // ok=true with a non-http(s) handle must never be claimed as filed, because the
  // handle is rendered into an <a href> in the UI. Protects against javascript:/
  // file:/garbage URLs being presented as a real issue link.
  const scriptUrlOutcome = await submitBugReport({
    composed: completeComposed(),
    transport: async () => ({ ok: true, issueUrl: 'javascript:alert(1)' }),
  });
  assertEqual(scriptUrlOutcome.status, 'draft-preserved', 'a javascript: handle is not a real submission');
  assertEqual(scriptUrlOutcome.claimedSubmitted, false, 'a javascript: handle never claims success');
  assertEqual(scriptUrlOutcome.resultHandle, null, 'a javascript: handle yields no result handle');

  const garbageUrlOutcome = await submitBugReport({
    composed: completeComposed(),
    transport: async () => ({ ok: true, issueUrl: 'not a url' }),
  });
  assertEqual(garbageUrlOutcome.status, 'draft-preserved', 'an unparseable handle is not a real submission');
  assertEqual(garbageUrlOutcome.claimedSubmitted, false, 'an unparseable handle never claims success');
}

async function realHandleIsTheOnlySuccessPath() {
  const outcome = await submitBugReport({
    composed: completeComposed(),
    transport: async () => ({ ok: true, issueUrl: 'https://github.com/x/codex/issues/42', issueNumber: 42 }),
  });
  assertEqual(outcome.status, 'submitted', 'a confirmed handle is the only success path');
  assertEqual(outcome.claimedSubmitted, true, 'confirmed handle claims success');
  assert(!!outcome.resultHandle, 'result handle is set on success');
  assertEqual(outcome.resultHandle!.issueUrl, 'https://github.com/x/codex/issues/42', 'result handle url');
  assertEqual(outcome.resultHandle!.issueNumber, 42, 'result handle number');
}

function copyablePayloadCarriesTheStructuredReport() {
  const composed = completeComposed();
  const rendered = renderCopyableBugPayload(composed.draft);
  assert(rendered.includes('Preview crashes on baseline AC'), 'copyable payload carries the title');
  assert(rendered.includes('## Observed behavior'), 'copyable payload preserves structured sections');
  assert(rendered.includes('Labels: bug'), 'copyable payload records the bug labels');
}

async function failedSubmissionCopyablePayloadMatchesTheStructuredDraft() {
  const composed = completeComposed();
  const outcome = await submitBugReport({
    composed,
    transport: async () => ({ ok: false, error: 'unauthorized' }),
  });

  assertEqual(outcome.status, 'draft-preserved', 'failed submission preserves the draft');
  assertEqual(
    outcome.copyablePayload,
    renderCopyableBugPayload(composed.draft),
    'failed-submission copyable payload matches the structured draft'
  );
  assert(outcome.copyablePayload.includes(composed.draft.markdownBody), 'copyable payload embeds the markdown draft');
  assert(outcome.copyablePayload.includes(`Labels: ${composed.draft.labels.join(', ')}`), 'copyable payload embeds labels');
  assert(outcome.copyablePayload.includes('## Expected behavior'), 'copyable payload keeps expected behavior separate');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
