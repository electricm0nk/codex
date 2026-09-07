import { composeEnhancementRequest, ENHANCEMENT_ISSUE_LABEL } from './composeEnhancementRequest';
import { assembleFeedbackEvidence, type EvidenceAttachment } from '../evidence';
import type { TesterWorkbenchSurface } from '../../loadTesterWorkbenchSurface';
import { assert, assertEqual } from '../../../testSupport/asserts';
import { makeSurface } from '../../../testSupport/makeSurface';

function completeEnhancementPayload(surface: TesterWorkbenchSurface) {
  return assembleFeedbackEvidence({
    flow: 'enhancement',
    surface,
    testerInput: {
      testerGoal: 'Author several Guard Stance packages back-to-back without re-walking each wizard step.',
      currentFriction: 'Every package forces the full multi-click wizard even when only one field differs.',
      requestedCapability: 'A duplicate-and-edit action that clones the last authored package as a starting point.',
      affectedSurface: 'authoring workbench',
    },
  });
}

async function main() {
  draftCarriesRequiredStructuredIssueFields();
  draftHasEnhancementTypeAndRequiredLabels();
  bodyHasSevenSectionsInContractOrder();
  fallbackDataSourceIsPreservedInBuildContext();
  goalFrictionCapabilityAndSurfaceStayDistinct();
  submittableOnlyWhenCompleteAndTitled();
  missingEnhancementFieldBlocksSubmission();
  emptyTitleBlocksSubmissionButStillDrafts();
  redactedTesterFieldIsMarkedNotDropped();
  supportingEvidenceReflectsAutoContext();
  attachmentsAndRedactionDeclarationAreIntegrated();
  unresolvedAttachmentBlocksSubmissionButStillDrafts();
  reportableDraftStripsMemoryContextFromTesterInput();
  bugPayloadIsRejected();
}

function draftCarriesRequiredStructuredIssueFields() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({
    title: 'Add duplicate-and-edit for Guard Stance packages',
    payload: completeEnhancementPayload(surface),
  });

  assertEqual(composed.draft.title, 'Add duplicate-and-edit for Guard Stance packages', 'draft title is preserved');
  assertEqual(composed.draft.issueType, 'enhancement', 'draft issue type is enhancement');
  assert(Array.isArray(composed.draft.labels), 'draft carries structured labels');
  assertEqual(typeof composed.draft.markdownBody, 'string', 'draft carries rendered markdown body');
  assertEqual(composed.draft.sections.length, 7, 'draft carries structured sections');
}

function draftHasEnhancementTypeAndRequiredLabels() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({
    title: 'Add duplicate-and-edit for Guard Stance packages',
    payload: completeEnhancementPayload(surface),
  });

  assertEqual(composed.draft.issueType, 'enhancement', 'issue type is enhancement');
  assert(composed.draft.labels.includes(ENHANCEMENT_ISSUE_LABEL), 'labels include the enhancement label');
  assert(composed.draft.labels.includes('channel:alpha'), 'labels include channel label when available');
  assert(composed.draft.labels.includes('platform:linux'), 'labels include platform label when available');
  assert(
    composed.draft.labels.some((label) => label.startsWith('surface:')),
    'labels include an affected-surface label when determinable'
  );
  // The affected-surface label is derived from the tester-entered affected surface.
  assert(
    composed.draft.labels.includes('surface:authoring-workbench'),
    'affected-surface label comes from the tester-entered affected surface'
  );
  assertEqual(new Set(composed.draft.labels).size, composed.draft.labels.length, 'labels are unique');
}

function bodyHasSevenSectionsInContractOrder() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({
    title: 'Add duplicate-and-edit for Guard Stance packages',
    payload: completeEnhancementPayload(surface),
  });

  const headings = composed.draft.sections.map((section) => section.heading);
  assertEqual(headings[0], 'Summary', 'section 1 is Summary');
  assertEqual(headings[1], 'Current build / channel / platform / workflow', 'section 2 is build/channel/platform/workflow');
  assertEqual(headings[2], 'Tester goal', 'section 3 is Tester goal');
  assertEqual(headings[3], 'Current friction or limitation', 'section 4 is Current friction or limitation');
  assertEqual(headings[4], 'Requested capability or improvement', 'section 5 is Requested capability or improvement');
  assertEqual(headings[5], 'Affected surface', 'section 6 is Affected surface');
  assertEqual(headings[6], 'Supporting evidence / examples', 'section 7 is Supporting evidence / examples');
  assertEqual(headings.length, 7, 'exactly the seven contract sections are present');

  for (const heading of headings) {
    assert(composed.draft.markdownBody.includes(`## ${heading}`), `markdown body keeps the ${heading} section`);
  }
  // Build/channel/platform/workflow metadata must never be omitted when available.
  assert(composed.draft.markdownBody.includes('Codex 0.14.0-test'), 'build label present in body');
  assert(composed.draft.markdownBody.includes('alpha · Linux first-class'), 'channel/support label present in body');
  assert(composed.draft.markdownBody.includes('Linux'), 'platform present in body');
  assert(composed.draft.markdownBody.includes('Character-preview authoring check'), 'workflow present in body');
  assert(
    composed.draft.markdownBody.includes('Governed release unit id: alpha-v0.0.0-test-1234abcd'),
    'release unit id present in body'
  );
  assert(
    composed.draft.markdownBody.includes('Official release-truth surface: GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadUpdateAction.ts over the F3a fetch pipeline'),
    'official release-truth surface present in body'
  );
}

function fallbackDataSourceIsPreservedInBuildContext() {
  const surface = makeSurface({ fallbackNotice: 'Tauri command unavailable; using sample fixture' });
  const composed = composeEnhancementRequest({
    title: 'Add duplicate-and-edit',
    payload: completeEnhancementPayload(surface),
  });
  const metadata = composed.draft.sections.find((s) => s.heading === 'Current build / channel / platform / workflow');

  assert(
    !!metadata &&
      metadata.body.includes('Live backend data (fallback: Tauri command unavailable; using sample fixture)'),
    'fallback data-source identity is preserved in the build/channel/platform/workflow section'
  );
}

function goalFrictionCapabilityAndSurfaceStayDistinct() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({
    title: 'Add duplicate-and-edit for Guard Stance packages',
    payload: completeEnhancementPayload(surface),
  });

  const goal = composed.draft.sections.find((s) => s.heading === 'Tester goal');
  const friction = composed.draft.sections.find((s) => s.heading === 'Current friction or limitation');
  const capability = composed.draft.sections.find((s) => s.heading === 'Requested capability or improvement');
  const affected = composed.draft.sections.find((s) => s.heading === 'Affected surface');

  assert(!!goal && goal.body.includes('back-to-back'), 'tester goal carries goal text');
  assert(!!friction && friction.body.includes('multi-click wizard'), 'friction carries friction text');
  assert(!!capability && capability.body.includes('duplicate-and-edit'), 'capability carries capability text');
  assert(!!affected && affected.body.includes('authoring workbench'), 'affected surface carries surface text');

  const bodies = new Set([goal!.body, friction!.body, capability!.body, affected!.body]);
  assertEqual(bodies.size, 4, 'goal, friction, capability and affected surface are four distinct fields');
}

function submittableOnlyWhenCompleteAndTitled() {
  const surface = makeSurface();
  const complete = composeEnhancementRequest({
    title: 'Add duplicate-and-edit',
    payload: completeEnhancementPayload(surface),
  });
  assertEqual(complete.submittable, true, 'complete + titled request is submittable');
  assertEqual(complete.problems.length, 0, 'no problems on a complete request');
}

function missingEnhancementFieldBlocksSubmission() {
  const surface = makeSurface();
  const missing = composeEnhancementRequest({
    title: 'Add duplicate-and-edit',
    payload: assembleFeedbackEvidence({
      flow: 'enhancement',
      surface,
      testerInput: {
        testerGoal: 'author faster',
        currentFriction: 'too many clicks',
        // requestedCapability intentionally omitted
        affectedSurface: 'authoring workbench',
      },
    }),
  });
  assertEqual(missing.submittable, false, 'incomplete enhancement evidence is not submittable');
  assert(
    missing.problems.some((p) => p.includes('Requested capability')),
    'incomplete request surfaces the missing required capability field'
  );
}

function emptyTitleBlocksSubmissionButStillDrafts() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({ title: '   ', payload: completeEnhancementPayload(surface) });
  assertEqual(composed.submittable, false, 'empty title blocks submission');
  assert(
    composed.problems.some((p) => p.toLowerCase().includes('title')),
    'empty title is reported as a problem'
  );
  assertEqual(composed.draft.sections.length, 7, 'a draft is still composed even when not submittable');
}

function redactedTesterFieldIsMarkedNotDropped() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence({
    flow: 'enhancement',
    surface,
    testerInput: {
      testerGoal: 'author faster',
      currentFriction: 'secret internal workflow detail',
      requestedCapability: 'bulk edit',
      affectedSurface: 'authoring workbench',
    },
    redactedFields: ['currentFriction'],
  });
  const composed = composeEnhancementRequest({ title: 'Add bulk edit', payload });
  const friction = composed.draft.sections.find((s) => s.heading === 'Current friction or limitation');
  assert(!!friction && /redact/i.test(friction.body), 'redacted field is explicitly marked as redacted');
  assert(!friction!.body.includes('secret internal workflow detail'), 'redacted content is not leaked into the body');
  assertEqual(composed.submittable, true, 'redacted required field still counts as present');
}

function supportingEvidenceReflectsAutoContext() {
  const surface = makeSurface();
  const composed = composeEnhancementRequest({
    title: 'Add duplicate-and-edit',
    payload: completeEnhancementPayload(surface),
  });
  const supporting = composed.draft.sections.find((s) => s.heading === 'Supporting evidence / examples');
  assert(!!supporting, 'supporting evidence section exists');
  // Diagnostics are optional for enhancement but preserved when present (do not fabricate).
  assert(supporting!.body.includes('A claim is blocked.'), 'diagnostic context is preserved when present');
  assert(supporting!.body.includes('Baseline AC blocked'), 'blocked claim context is preserved when present');
  assert(/no attachments/i.test(supporting!.body), 'no-attachment state is stated honestly');
}

function attachmentsAndRedactionDeclarationAreIntegrated() {
  const surface = makeSurface();
  const screenshot: EvidenceAttachment = {
    id: 'shot-1',
    label: 'wizard-flow-redacted.png',
    kind: 'screenshot',
    mayContainSensitiveData: true,
    testerConfirmedInclude: false,
    redacted: true,
  };
  const sample: EvidenceAttachment = {
    id: 'sample-1',
    label: 'sample-packages.json',
    kind: 'other',
    mayContainSensitiveData: false,
    testerConfirmedInclude: true,
    redacted: false,
  };
  const payload = assembleFeedbackEvidence({
    flow: 'enhancement',
    surface,
    testerInput: {
      testerGoal: 'author faster',
      currentFriction: 'too many clicks',
      requestedCapability: 'bulk edit',
      affectedSurface: 'authoring workbench',
    },
    attachments: [screenshot, sample],
    redaction: { declared: true, statement: 'Screenshot scrubbed; sample data anonymized.' },
  });
  const composed = composeEnhancementRequest({ title: 'Add bulk edit', payload });
  const supporting = composed.draft.sections.find((s) => s.heading === 'Supporting evidence / examples');

  assertEqual(composed.submittable, true, 'resolved attachments with declaration remain submittable');
  assert(!!supporting && supporting.body.includes('wizard-flow-redacted.png (screenshot) — redacted'), 'redacted attachment is listed');
  assert(!!supporting && supporting.body.includes('sample-packages.json (other) — included'), 'included attachment is listed');
  assert(
    !!supporting && supporting.body.includes('Redaction declaration: Screenshot scrubbed; sample data anonymized.'),
    'redaction declaration is rendered with attachments'
  );
}

function unresolvedAttachmentBlocksSubmissionButStillDrafts() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence({
    flow: 'enhancement',
    surface,
    testerInput: {
      testerGoal: 'author faster',
      currentFriction: 'too many clicks',
      requestedCapability: 'bulk edit',
      affectedSurface: 'authoring workbench',
    },
    attachments: [
      {
        id: 'shot-1',
        label: 'wizard-sensitive.png',
        kind: 'screenshot',
        mayContainSensitiveData: true,
        testerConfirmedInclude: false,
        redacted: false,
      },
    ],
    redaction: { declared: false, statement: null },
  });
  const composed = composeEnhancementRequest({ title: 'Add bulk edit', payload });
  const supporting = composed.draft.sections.find((s) => s.heading === 'Supporting evidence / examples');

  assertEqual(composed.submittable, false, 'unresolved attachment blocks submission');
  assert(composed.problems.some((p) => p.includes('wizard-sensitive.png')), 'unresolved attachment problem is surfaced');
  assert(
    composed.problems.some((p) => p.includes('redaction declaration')),
    'missing redaction declaration problem is surfaced'
  );
  assert(!!supporting && supporting.body.includes('wizard-sensitive.png'), 'draft still preserves attachment context');
}

function reportableDraftStripsMemoryContextFromTesterInput() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence({
    flow: 'enhancement',
    surface,
    testerInput: {
      testerGoal: [
        'Author packages without losing work.',
        '<memory-context>',
        '[System note: The following is recalled memory context, NOT new user input.]',
        'AI Self-Representation',
        'Todd private profile observation that must not be reportable.',
        '</memory-context>',
        'Continue the ordinary tester goal after the internal block.',
      ].join('\n'),
      currentFriction: 'The current report copy included internal recalled context.',
      requestedCapability: 'Copyable enhancement drafts should strip internal context blocks.',
      affectedSurface: 'feedback draft output',
    },
  });
  const composed = composeEnhancementRequest({ title: 'Strip internal context from enhancement drafts', payload });

  assert(composed.draft.markdownBody.includes('Author packages without losing work'), 'ordinary evidence before the leak remains');
  assert(composed.draft.markdownBody.includes('ordinary tester goal after the internal block'), 'ordinary evidence after the leak remains');
  assert(composed.draft.markdownBody.includes('internal context block removed'), 'removed context is explicitly marked');
  assert(!composed.draft.markdownBody.includes('<memory-context>'), 'opening memory-context marker is stripped');
  assert(!composed.draft.markdownBody.includes('</memory-context>'), 'closing memory-context marker is stripped');
  assert(!composed.draft.markdownBody.includes('recalled memory context'), 'recalled-memory system note is stripped');
  assert(!composed.draft.markdownBody.includes('Todd private profile observation'), 'remembered observations are stripped');
}

function bugPayloadIsRejected() {
  const surface = makeSurface();
  const bugPayload = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: 'crash',
      expectedBehavior: 'no crash',
      reproductionSteps: '1. open 2. compute',
    },
  });

  let threw = false;
  try {
    composeEnhancementRequest({ title: 'x', payload: bugPayload });
  } catch {
    threw = true;
  }
  assertEqual(threw, true, 'enhancement composer refuses a bug payload');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
