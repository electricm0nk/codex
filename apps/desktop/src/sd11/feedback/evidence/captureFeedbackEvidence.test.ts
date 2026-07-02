import {
  captureAutoEvidence,
  assembleFeedbackEvidence,
  type AssembleFeedbackEvidenceInput,
} from './captureFeedbackEvidence';
import type { Sd11TesterWorkbenchSurface } from '../../loadSd11TesterWorkbenchSurface';
import { assert, assertEqual } from '../../../testSupport/asserts';
import { makeSurface } from '../../../testSupport/makeSurface';

async function main() {
  autoEvidenceResolvesFromSurface();
  bothFlowsShareIdenticalAutoValues();
  bugRequiresTesterNarrative();
  redactedTesterFieldCountsAsPresent();
  attachmentsGateCompleteness();
  enhancementFlowUsesItsOwnTesterFields();
}

function autoEvidenceResolvesFromSurface() {
  const auto = captureAutoEvidence(makeSurface());
  assertEqual(auto.buildLabel, 'codex-desktop-shell-scaffold@0.0.0-test', 'auto build label');
  assertEqual(auto.channelSupportLabel, 'alpha · Linux first-class', 'auto channel/support label');
  assertEqual(auto.platformLabel, 'Linux', 'auto platform label');
  assertEqual(auto.currentWorkflow, 'GE08 Guard Stance authoring workbench / Authored / Computed', 'auto workflow');
  assertEqual(auto.dataSourceIdentity, 'Real Tauri command snapshot', 'auto data source identity');
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

function bugInput(surface: Sd11TesterWorkbenchSurface): AssembleFeedbackEvidenceInput {
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

function enhancementInput(surface: Sd11TesterWorkbenchSurface): AssembleFeedbackEvidenceInput {
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
