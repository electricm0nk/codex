import {
  submitEnhancementRequest,
  renderCopyableEnhancementPayload,
  type EnhancementRequestTransport,
} from './submitEnhancementRequest';
import { composeEnhancementRequest } from './composeEnhancementRequest';
import { assembleFeedbackEvidence } from '../evidence';
import type { Sd11TesterWorkbenchSurface } from '../../loadSd11TesterWorkbenchSurface';

function assertEqual<T>(actual: T, expected: T, message: string) {
  if (actual !== expected) {
    throw new Error(`${message}: expected ${String(expected)}, got ${String(actual)}`);
  }
}

function assert(condition: boolean, message: string) {
  if (!condition) {
    throw new Error(message);
  }
}

function makeSurface(overrides: Partial<Sd11TesterWorkbenchSurface> = {}): Sd11TesterWorkbenchSurface {
  return {
    surfaceLabel: 'SD-11 tester workbench',
    headline: 'Bounded tester workbench',
    lead: 'lead',
    buildLabel: 'codex-desktop-shell-scaffold@0.0.0-test',
    channelLabel: 'alpha',
    platformLabel: 'Linux',
    supportTierLabel: 'Linux first-class · macOS second-class · Windows third-class',
    workflowName: 'GE08 Guard Stance authoring workbench',
    workflowState: 'Authored / Computed',
    dataTruthLabel: 'Real Tauri command snapshot',
    fallbackNotice: null,
    boundedScopeNotice: 'bounded',
    feedbackStatusNotice: 'feedback',
    updateStatusLabel: 'alpha tester track on Linux first-class',
    summaryRows: [],
    diagnostics: [],
    blockedClaims: [],
    explanationRefs: [],
    provenanceRefs: [],
    notes: ['note'],
    status: {
      build: { label: 'codex-desktop-shell-scaffold@0.0.0-test', version: '0.0.0-test' },
      channel: {
        testerFacingLabel: 'alpha',
        operatorBranch: 'develop',
        operatorPromotionPath: 'develop -> uat -> main',
        audience: 'audience',
        detail: 'detail',
      },
      support: {
        platformLabel: 'Linux',
        platformTier: 'first-class',
        currentPlatformSupportLabel: 'Linux first-class',
        tierMatrixLabel: 'Linux first-class · macOS second-class · Windows third-class',
        platformSupportDetail: 'detail',
      },
      update: { state: 'not-yet-supported', label: 'Update checks not yet wired in this slice', detail: 'detail' },
      issueCapture: {
        testerFacingChannelSupportLabel: 'alpha · Linux first-class',
        operatorBranch: 'develop',
        operatorPromotionPath: 'develop -> uat -> main',
        platformLabel: 'Linux',
        platformTier: 'first-class',
      },
    },
    ...overrides,
  };
}

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

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
