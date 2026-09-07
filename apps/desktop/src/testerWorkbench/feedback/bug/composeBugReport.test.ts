import { composeBugReport, BUG_ISSUE_LABEL } from './composeBugReport';
import { assembleFeedbackEvidence, type EvidenceAttachment } from '../evidence';
import type { TesterWorkbenchSurface } from '../../loadTesterWorkbenchSurface';
import { assert, assertEqual } from '../../../testSupport/asserts';
import { makeSurface } from '../../../testSupport/makeSurface';

function completeBugPayload(surface: TesterWorkbenchSurface) {
  return assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: 'The preview crashed when computing baseline AC.',
      expectedBehavior: 'The preview should render the computed baseline AC.',
      reproductionSteps: '1. Open workbench 2. Load package 3. Open preview',
    },
  });
}

async function main() {
  draftCarriesRequiredStructuredIssueFields();
  draftHasBugTypeAndRequiredLabels();
  bodyHasSevenSectionsInContractOrder();
  fallbackDataSourceIsPreservedInBuildContext();
  observedAndExpectedStayDistinct();
  submittableOnlyWhenCompleteAndTitled();
  emptyTitleBlocksSubmissionButStillDrafts();
  redactedTesterFieldIsMarkedNotDropped();
  diagnosticsAndAttachmentSectionsReflectEvidence();
  attachmentsAndRedactionDeclarationAreIntegrated();
  unresolvedAttachmentBlocksSubmissionButStillDrafts();
  reportableDraftStripsMemoryContextFromTesterInput();
  enhancementPayloadIsRejected();
}

function draftCarriesRequiredStructuredIssueFields() {
  const surface = makeSurface();
  const composed = composeBugReport({ title: 'Preview crashes on baseline AC', payload: completeBugPayload(surface) });

  assertEqual(composed.draft.title, 'Preview crashes on baseline AC', 'draft title is preserved');
  assertEqual(composed.draft.issueType, 'bug', 'draft issue type is bug');
  assert(Array.isArray(composed.draft.labels), 'draft carries structured labels');
  assertEqual(typeof composed.draft.markdownBody, 'string', 'draft carries rendered markdown body');
  assertEqual(composed.draft.sections.length, 7, 'draft carries structured sections');
}

function draftHasBugTypeAndRequiredLabels() {
  const surface = makeSurface();
  const composed = composeBugReport({ title: 'Preview crashes on baseline AC', payload: completeBugPayload(surface) });

  assertEqual(composed.draft.issueType, 'bug', 'issue type is bug');
  assert(composed.draft.labels.includes(BUG_ISSUE_LABEL), 'labels include the bug label');
  assert(composed.draft.labels.includes('channel:alpha'), 'labels include channel label when available');
  assert(composed.draft.labels.includes('platform:linux'), 'labels include platform label when available');
  assert(
    composed.draft.labels.some((label) => label.startsWith('surface:')),
    'labels include an affected-surface label when determinable'
  );
  // Labels must be unique.
  assertEqual(new Set(composed.draft.labels).size, composed.draft.labels.length, 'labels are unique');
}

function bodyHasSevenSectionsInContractOrder() {
  const surface = makeSurface();
  const composed = composeBugReport({ title: 'Preview crashes on baseline AC', payload: completeBugPayload(surface) });

  const headings = composed.draft.sections.map((section) => section.heading);
  assertEqual(headings[0], 'Summary', 'section 1 is Summary');
  assertEqual(headings[1], 'Current build / channel / platform / workflow', 'section 2 is build/channel/platform/workflow');
  assertEqual(headings[2], 'Observed behavior', 'section 3 is Observed behavior');
  assertEqual(headings[3], 'Expected behavior', 'section 4 is Expected behavior');
  assertEqual(headings[4], 'Reproduction steps', 'section 5 is Reproduction steps');
  assertEqual(headings[5], 'Diagnostics / explanation context', 'section 6 is Diagnostics / explanation context');
  assertEqual(headings[6], 'Attachments / redactions', 'section 7 is Attachments / redactions');
  assertEqual(headings.length, 7, 'exactly the seven contract sections are present');

  // The markdown body must preserve the structured headings as H2 sections.
  for (const heading of headings) {
    assert(composed.draft.markdownBody.includes(`## ${heading}`), `markdown body keeps the ${heading} section`);
  }
  // Build metadata must never be omitted when available.
  assert(composed.draft.markdownBody.includes('Codex 0.14.0-test'), 'build label present in body');
  assert(composed.draft.markdownBody.includes('alpha · Linux first-class'), 'channel/support label present in body');
  assert(composed.draft.markdownBody.includes('Linux'), 'platform present in body');
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
  const composed = composeBugReport({ title: 'Preview uses fallback data', payload: completeBugPayload(surface) });
  const metadata = composed.draft.sections.find((s) => s.heading === 'Current build / channel / platform / workflow');

  assert(
    !!metadata && metadata.body.includes('Live backend data (fallback: Tauri command unavailable; using sample fixture)'),
    'fallback data-source identity is preserved in the build/channel/platform/workflow section'
  );
}

function observedAndExpectedStayDistinct() {
  const surface = makeSurface();
  const composed = composeBugReport({ title: 'Preview crashes on baseline AC', payload: completeBugPayload(surface) });

  const observed = composed.draft.sections.find((s) => s.heading === 'Observed behavior');
  const expected = composed.draft.sections.find((s) => s.heading === 'Expected behavior');
  assert(!!observed && observed.body.includes('crashed'), 'observed behavior carries observed text');
  assert(!!expected && expected.body.includes('should render'), 'expected behavior carries expected text');
  assert(observed!.body !== expected!.body, 'observed and expected sections are distinct');
}

function submittableOnlyWhenCompleteAndTitled() {
  const surface = makeSurface();
  const complete = composeBugReport({ title: 'Preview crashes', payload: completeBugPayload(surface) });
  assertEqual(complete.submittable, true, 'complete + titled report is submittable');
  assertEqual(complete.problems.length, 0, 'no problems on a complete report');

  const missingNarrative = composeBugReport({
    title: 'Preview crashes',
    payload: assembleFeedbackEvidence({
      flow: 'bug',
      surface,
      testerInput: { observedBehavior: 'crash' },
    }),
  });
  assertEqual(missingNarrative.submittable, false, 'incomplete evidence is not submittable');
  assert(
    missingNarrative.problems.some((p) => p.includes('Expected behavior')),
    'incomplete report surfaces the missing required field'
  );
}

function emptyTitleBlocksSubmissionButStillDrafts() {
  const surface = makeSurface();
  const composed = composeBugReport({ title: '   ', payload: completeBugPayload(surface) });
  assertEqual(composed.submittable, false, 'empty title blocks submission');
  assert(
    composed.problems.some((p) => p.toLowerCase().includes('title')),
    'empty title is reported as a problem'
  );
  // A draft is still produced so evidence is never lost.
  assertEqual(composed.draft.sections.length, 7, 'a draft is still composed even when not submittable');
}

function redactedTesterFieldIsMarkedNotDropped() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: 'crash',
      expectedBehavior: 'no crash',
      reproductionSteps: 'secret repro path',
    },
    redactedFields: ['reproductionSteps'],
  });
  const composed = composeBugReport({ title: 'Preview crashes', payload });
  const repro = composed.draft.sections.find((s) => s.heading === 'Reproduction steps');
  assert(!!repro && /redact/i.test(repro.body), 'redacted field is explicitly marked as redacted');
  assert(!repro!.body.includes('secret repro path'), 'redacted content is not leaked into the body');
  // Redacted-but-present still satisfies completeness.
  assertEqual(composed.submittable, true, 'redacted required field still counts as present');
}

function diagnosticsAndAttachmentSectionsReflectEvidence() {
  const surface = makeSurface();
  const composed = composeBugReport({ title: 'Preview crashes', payload: completeBugPayload(surface) });

  const diagnostics = composed.draft.sections.find((s) => s.heading === 'Diagnostics / explanation context');
  assert(!!diagnostics && diagnostics.body.includes('A claim is blocked.'), 'diagnostic message preserved');
  assert(!!diagnostics && diagnostics.body.includes('Baseline AC blocked'), 'blocked claim preserved');

  const attachments = composed.draft.sections.find((s) => s.heading === 'Attachments / redactions');
  assert(!!attachments && /no attachments/i.test(attachments.body), 'no-attachment state stated honestly');
}

function attachmentsAndRedactionDeclarationAreIntegrated() {
  const surface = makeSurface();
  const screenshot: EvidenceAttachment = {
    id: 'shot-1',
    label: 'preview-redacted.png',
    kind: 'screenshot',
    mayContainSensitiveData: true,
    testerConfirmedInclude: false,
    redacted: true,
  };
  const log: EvidenceAttachment = {
    id: 'log-1',
    label: 'renderer.log',
    kind: 'log',
    mayContainSensitiveData: false,
    testerConfirmedInclude: true,
    redacted: false,
  };
  const payload = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: 'crash',
      expectedBehavior: 'no crash',
      reproductionSteps: '1. open 2. compute',
    },
    attachments: [screenshot, log],
    redaction: { declared: true, statement: 'Screenshot scrubbed; log reviewed for secrets.' },
  });
  const composed = composeBugReport({ title: 'Preview crashes', payload });
  const attachments = composed.draft.sections.find((s) => s.heading === 'Attachments / redactions');

  assertEqual(composed.submittable, true, 'resolved attachments with declaration remain submittable');
  assert(!!attachments && attachments.body.includes('preview-redacted.png (screenshot) — redacted'), 'redacted attachment is listed');
  assert(!!attachments && attachments.body.includes('renderer.log (log) — included'), 'included attachment is listed');
  assert(
    !!attachments && attachments.body.includes('Redaction declaration: Screenshot scrubbed; log reviewed for secrets.'),
    'redaction declaration is rendered with attachments'
  );
}

function unresolvedAttachmentBlocksSubmissionButStillDrafts() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: 'crash',
      expectedBehavior: 'no crash',
      reproductionSteps: '1. open 2. compute',
    },
    attachments: [
      {
        id: 'shot-1',
        label: 'preview-sensitive.png',
        kind: 'screenshot',
        mayContainSensitiveData: true,
        testerConfirmedInclude: false,
        redacted: false,
      },
    ],
    redaction: { declared: false, statement: null },
  });
  const composed = composeBugReport({ title: 'Preview crashes', payload });
  const attachments = composed.draft.sections.find((s) => s.heading === 'Attachments / redactions');

  assertEqual(composed.submittable, false, 'unresolved attachment blocks submission');
  assert(composed.problems.some((p) => p.includes('preview-sensitive.png')), 'unresolved attachment problem is surfaced');
  assert(
    composed.problems.some((p) => p.includes('redaction declaration')),
    'missing redaction declaration problem is surfaced'
  );
  assert(!!attachments && attachments.body.includes('preview-sensitive.png'), 'draft still preserves attachment context');
}

function reportableDraftStripsMemoryContextFromTesterInput() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: [
        'The update check honestly returned no official release.',
        '<memory-context>',
        '[System note: The following is recalled memory context, NOT new user input.]',
        'User Representation',
        'Todd private profile observation that must not be reportable.',
        '</memory-context>',
        'The tester evidence after the internal context still matters.',
      ].join('\n'),
      expectedBehavior: 'No internal context should appear in copyable report output.',
      reproductionSteps: '1. Run update check 2. Preserve report text',
    },
  });
  const composed = composeBugReport({ title: 'Update report leaked internal context', payload });

  assert(composed.draft.markdownBody.includes('honestly returned no official release'), 'ordinary evidence before the leak remains');
  assert(composed.draft.markdownBody.includes('tester evidence after the internal context'), 'ordinary evidence after the leak remains');
  assert(composed.draft.markdownBody.includes('internal context block removed'), 'removed context is explicitly marked');
  assert(!composed.draft.markdownBody.includes('<memory-context>'), 'opening memory-context marker is stripped');
  assert(!composed.draft.markdownBody.includes('</memory-context>'), 'closing memory-context marker is stripped');
  assert(!composed.draft.markdownBody.includes('recalled memory context'), 'recalled-memory system note is stripped');
  assert(!composed.draft.markdownBody.includes('Todd private profile observation'), 'remembered observations are stripped');
}

function enhancementPayloadIsRejected() {
  const surface = makeSurface();
  const enhancementPayload = assembleFeedbackEvidence({
    flow: 'enhancement',
    surface,
    testerInput: {
      testerGoal: 'author faster',
      currentFriction: 'too many clicks',
      requestedCapability: 'bulk edit',
      affectedSurface: 'authoring workbench',
    },
  });

  let threw = false;
  try {
    composeBugReport({ title: 'x', payload: enhancementPayload });
  } catch {
    threw = true;
  }
  assertEqual(threw, true, 'bug composer refuses an enhancement payload');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
