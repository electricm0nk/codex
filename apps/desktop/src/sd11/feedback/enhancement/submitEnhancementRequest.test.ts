import {
  submitEnhancementRequest,
  renderCopyableEnhancementPayload,
  type EnhancementRequestTransport,
} from './submitEnhancementRequest';
import { composeEnhancementRequest } from './composeEnhancementRequest';
import { assembleFeedbackEvidence } from '../evidence';
import type { Sd11TesterWorkbenchSurface } from '../../loadSd11TesterWorkbenchSurface';
import { assert, assertEqual } from '../../../testSupport/asserts';
import { makeSurface } from '../../../testSupport/makeSurface';

function completeComposed(overrides: Partial<Sd11TesterWorkbenchSurface> = {}) {
  const surface = makeSurface(overrides);
  return composeEnhancementRequest({
    title: 'Add duplicate-and-edit for Guard Stance packages',
    payload: assembleFeedbackEvidence({
      flow: 'enhancement',
      surface,
      testerInput: {
        testerGoal: 'Author several packages back-to-back.',
        currentFriction: 'The wizard restarts every time.',
        requestedCapability: 'A duplicate-and-edit action.',
        affectedSurface: 'GE08 authoring workbench',
      },
    }),
  });
}

async function main() {
  await blockedWhenIncomplete();
  await draftPreservedWhenNoTransport();
  await draftPreservedWhenTransportThrows();
  await draftPreservedWhenNoIssueHandle();
  await submittedOnlyWithRealHandle();
  copyablePayloadCarriesStructuredRequest();
  await noTransportCopyablePayloadStripsMemoryContext();
}

async function blockedWhenIncomplete() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({
    title: '',
    payload: assembleFeedbackEvidence({
      flow: 'enhancement',
      surface,
      testerInput: { testerGoal: 'x' },
    }),
  });
  const outcome = await submitEnhancementRequest({ composed, transport: null });
  assertEqual(outcome.status, 'blocked-incomplete', 'incomplete request is blocked');
  assertEqual(outcome.claimedSubmitted, false, 'nothing is claimed when blocked');
  assert(outcome.copyablePayload.length > 0, 'a copyable payload is preserved even when blocked');
}

async function draftPreservedWhenNoTransport() {
  const outcome = await submitEnhancementRequest({ composed: completeComposed(), transport: null });
  assertEqual(outcome.status, 'draft-preserved', 'no transport preserves the draft');
  assertEqual(outcome.claimedSubmitted, false, 'no submission is claimed without a transport');
  assertEqual(outcome.resultHandle, null, 'no result handle without a transport');
  assert(outcome.message.includes('manual GitHub issue filing'), 'no transport directs the tester to manual filing');
  assert(outcome.message.includes('none is claimed'), 'no transport message refuses claimed submission');
  assert(outcome.copyablePayload.includes('Add duplicate-and-edit'), 'copyable payload carries the title');
}

async function draftPreservedWhenTransportThrows() {
  const transport: EnhancementRequestTransport = async () => {
    throw new Error('network down');
  };
  const outcome = await submitEnhancementRequest({ composed: completeComposed(), transport });
  assertEqual(outcome.status, 'draft-preserved', 'a throwing transport preserves the draft');
  assertEqual(outcome.claimedSubmitted, false, 'no success is claimed when transport throws');
  assert(outcome.message.includes('network down'), 'the failure reason is surfaced honestly');
}

async function draftPreservedWhenNoIssueHandle() {
  const whitespaceTransport: EnhancementRequestTransport = async () => ({ ok: true, issueUrl: '   ' });
  const whitespaceOutcome = await submitEnhancementRequest({ composed: completeComposed(), transport: whitespaceTransport });
  assertEqual(whitespaceOutcome.status, 'draft-preserved', 'a missing issue handle preserves the draft');
  assertEqual(whitespaceOutcome.claimedSubmitted, false, 'no success without a real issue handle');

  const invalidUrlTransport: EnhancementRequestTransport = async () => ({ ok: true, issueUrl: 'javascript:alert(1)' });
  const invalidOutcome = await submitEnhancementRequest({ composed: completeComposed(), transport: invalidUrlTransport });
  assertEqual(invalidOutcome.status, 'draft-preserved', 'an invalid issue handle preserves the draft');
  assertEqual(invalidOutcome.claimedSubmitted, false, 'no success without a valid issue URL');
}

async function submittedOnlyWithRealHandle() {
  const transport: EnhancementRequestTransport = async () => ({
    ok: true,
    issueUrl: 'https://github.com/electricm0nk/codex/issues/123',
    issueNumber: 123,
  });
  const outcome = await submitEnhancementRequest({ composed: completeComposed(), transport });
  assertEqual(outcome.status, 'submitted', 'a real handle yields submitted');
  assertEqual(outcome.claimedSubmitted, true, 'submission is claimed only with a real handle');
  assert(!!outcome.resultHandle && outcome.resultHandle.issueNumber === 123, 'result handle carries the issue number');
  assert(outcome.message.includes('https://github.com/electricm0nk/codex/issues/123'), 'message carries the filed URL');
}

function copyablePayloadCarriesStructuredRequest() {
  const composed = completeComposed();
  const payload = renderCopyableEnhancementPayload(composed.draft);
  assert(payload.includes('Issue type: enhancement'), 'copyable payload records the enhancement issue type');
  assert(payload.includes('Labels: '), 'copyable payload records labels');
  assert(payload.includes('## Tester goal'), 'copyable payload preserves the structured markdown body');
}

async function noTransportCopyablePayloadStripsMemoryContext() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({
    title: 'Strip internal context from feedback drafts',
    payload: assembleFeedbackEvidence({
      flow: 'enhancement',
      surface,
      testerInput: {
        testerGoal: [
          'Prepare a manual enhancement issue.',
          '<memory-context>',
          '[System note: The following is recalled memory context, NOT new user input.]',
          'Todd private profile observation that must not be copied.',
          '</memory-context>',
          'Keep the surrounding tester context.',
        ].join('\n'),
        currentFriction: 'The draft includes internal recalled-memory context.',
        requestedCapability: 'Strip internal context before copyable manual filing output.',
        affectedSurface: 'feedback draft output',
      },
    }),
  });
  const outcome = await submitEnhancementRequest({ composed, transport: null });

  assertEqual(outcome.status, 'draft-preserved', 'no transport still preserves the draft');
  assert(outcome.copyablePayload.includes('Prepare a manual enhancement issue.'), 'ordinary evidence remains');
  assert(outcome.copyablePayload.includes('Keep the surrounding tester context.'), 'ordinary trailing evidence remains');
  assert(outcome.copyablePayload.includes('internal context block removed'), 'removed context is marked');
  assert(!outcome.copyablePayload.includes('<memory-context>'), 'memory-context marker is stripped');
  assert(!outcome.copyablePayload.includes('recalled memory context'), 'system note is stripped');
  assert(!outcome.copyablePayload.includes('Todd private profile observation'), 'private observation is stripped');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
