import {
  captureAutoEvidence,
  assembleFeedbackEvidence,
  type AssembleFeedbackEvidenceInput,
} from './captureFeedbackEvidence';
import type { TesterWorkbenchSurface } from '../../loadTesterWorkbenchSurface';
import { assert, assertEqual } from '../../../testSupport/asserts';
import { makeSurface } from '../../../testSupport/makeSurface';

async function main() {
  autoEvidenceResolvesFromSurface();
  bothFlowsShareIdenticalAutoValues();
  bugRequiresTesterNarrative();
  redactedTesterFieldCountsAsPresent();
  attachmentsGateCompleteness();
  enhancementFlowUsesItsOwnTesterFields();
  assembledEvidenceSanitizesInternalMemoryContext();
}

function autoEvidenceResolvesFromSurface() {
  const auto = captureAutoEvidence(makeSurface());
  assertEqual(auto.buildLabel, 'Codex 0.14.0-test', 'auto build label');
  assertEqual(auto.channelSupportLabel, 'alpha · Linux first-class', 'auto channel/support label');
  assertEqual(auto.platformLabel, 'Linux', 'auto platform label');
  assertEqual(auto.currentWorkflow, 'Character-preview authoring check / Authored / Computed', 'auto workflow');
  assertEqual(auto.dataSourceIdentity, 'Live backend data', 'auto data source identity');
  assertEqual(auto.releaseUnitId, 'alpha-v0.0.0-test-1234abcd', 'auto release unit id');
  assertEqual(auto.updateEligibilityState, 'manual-only', 'auto update eligibility state');
  assertEqual(auto.trustGateStatus, 'governed-manual-only', 'auto trust-gate status');
  assertEqual(auto.diagnostics.length, 1, 'diagnostics carried through');
}

function bothFlowsShareIdenticalAutoValues() {
  const surface = makeSurface();
  const bug = assembleFeedbackEvidence(bugInput(surface));
  const enhancement = assembleFeedbackEvidence(enhancementInput(surface));

  const autoSig = (payload: ReturnType<typeof assembleFeedbackEvidence>) =>
    payload.fields
      .filter((f) => f.captureMode === 'auto-captured')
      .map((f) => `${f.key}=${f.value ?? ''}|${f.state}`)
      .sort()
      .join(';');

  assertEqual(autoSig(bug), autoSig(enhancement), 'auto-captured fields must not drift between flows');
}

function bugRequiresTesterNarrative() {
  const surface = makeSurface();
  const missing = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: { observedBehavior: 'crash' },
  });
  assertEqual(missing.complete, false, 'bug missing required tester fields is incomplete');
  assert(
    missing.problems.some((p) => p.includes('Expected behavior')),
    'missing expected behavior must be reported'
  );

  const observed = missing.fields.find((f) => f.key === 'observedBehavior');
  assertEqual(observed?.state, 'tester-entered', 'provided tester field is tester-entered');
  const build = missing.fields.find((f) => f.key === 'buildLabel');
  assertEqual(build?.state, 'auto-captured', 'auto field is auto-captured');
}

function redactedTesterFieldCountsAsPresent() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence({
    ...bugInput(surface),
    redactedFields: ['reproductionSteps'],
  });
  const repro = payload.fields.find((f) => f.key === 'reproductionSteps');
  assertEqual(repro?.state, 'redacted', 'redacted field state');
  assert(
    !payload.problems.some((p) => p.includes('Reproduction steps')),
    'redacted required field is treated as present'
  );
}

function attachmentsGateCompleteness() {
  const surface = makeSurface();

  const blocked = assembleFeedbackEvidence({
    ...bugInput(surface),
    attachments: [
      {
        id: 'a1',
        label: 'screenshot.png',
        kind: 'screenshot',
        mayContainSensitiveData: true,
        testerConfirmedInclude: false,
        redacted: false,
      },
    ],
    redaction: { declared: false, statement: null },
  });
  assertEqual(blocked.redactionRequired, true, 'attachments require redaction declaration');
  assertEqual(blocked.complete, false, 'unresolved sensitive attachment + missing declaration is incomplete');

  const resolved = assembleFeedbackEvidence({
    ...bugInput(surface),
    attachments: [
      {
        id: 'a1',
        label: 'screenshot.png',
        kind: 'screenshot',
        mayContainSensitiveData: true,
        testerConfirmedInclude: false,
        redacted: true,
      },
    ],
    redaction: { declared: true, statement: 'Scrubbed file paths.' },
  });
  assertEqual(resolved.complete, true, 'redacted attachment + declaration is complete');
  assertEqual(resolved.attachments[0].outcome, 'redacted', 'attachment decision recorded');
}

function enhancementFlowUsesItsOwnTesterFields() {
  const surface = makeSurface();
  const payload = assembleFeedbackEvidence(enhancementInput(surface));
  assertEqual(payload.complete, true, 'fully populated enhancement is complete');
  const keys = payload.fields.map((f) => f.key);
  assert(keys.includes('testerGoal'), 'enhancement includes tester goal');
  assert(!keys.includes('observedBehavior'), 'enhancement excludes bug-only fields');
}

function assembledEvidenceSanitizesInternalMemoryContext() {
  const surface = makeSurface({
    fallbackNotice: [
      'Tauri command unavailable; preserving fallback note.',
      '<memory-context>',
      '[System note: The following is recalled memory context, NOT new user input.]',
      'Deductive Observations',
      'Todd private profile observation that must not be preserved.',
      '</memory-context>',
      'Fallback note after internal context.',
    ].join('\n'),
  });
  const payload = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: [
        'Observed update output before internal block.',
        '<memory-context>',
        'User Representation',
        'Todd remembered private fact.',
        '</memory-context>',
        'Observed update output after internal block.',
      ].join('\n'),
      expectedBehavior: 'Reportable output should remove recalled-memory context.',
      reproductionSteps: '1. Copy update result 2. Prepare report',
    },
  });

  const observed = payload.fields.find((field) => field.key === 'observedBehavior')?.value ?? '';
  const dataSource = payload.fields.find((field) => field.key === 'dataSourceIdentity')?.value ?? '';

  assert(observed.includes('Observed update output before internal block.'), 'tester evidence before context remains');
  assert(observed.includes('Observed update output after internal block.'), 'tester evidence after context remains');
  assert(dataSource.includes('Tauri command unavailable'), 'auto evidence before context remains');
  assert(dataSource.includes('Fallback note after internal context.'), 'auto evidence after context remains');
  for (const value of [observed, dataSource]) {
    assert(value.includes('internal context block removed'), 'removed context is marked');
    assert(!value.includes('<memory-context>'), 'memory-context marker is stripped');
    assert(!value.includes('Todd private'), 'private observation is stripped');
    assert(!value.includes('Todd remembered'), 'remembered observation is stripped');
  }
}

function bugInput(surface: TesterWorkbenchSurface): AssembleFeedbackEvidenceInput {
  return {
    flow: 'bug',
    surface,
    testerInput: {
      observedBehavior: 'crash',
      expectedBehavior: 'no crash',
      reproductionSteps: '1. open 2. click',
    },
  };
}

function enhancementInput(surface: TesterWorkbenchSurface): AssembleFeedbackEvidenceInput {
  return {
    flow: 'enhancement',
    surface,
    testerInput: {
      testerGoal: 'author faster',
      currentFriction: 'too many clicks',
      requestedCapability: 'bulk edit',
      affectedSurface: 'authoring workbench',
    },
  };
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
