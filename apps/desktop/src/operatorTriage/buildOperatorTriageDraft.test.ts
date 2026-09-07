/**
 * Tests for the headless SD-15 operator triage / regression-receipt draft composer.
 *
 * The composer consumes an already-assembled SD-11 `FeedbackEvidencePayload` plus
 * explicit operator-entered SD-15 judgment fields and emits a structured draft
 * object and a copyable text draft. These tests pin the SD-15 boundary rules:
 *   - tester-supplied / auto-captured / operator-added partitions stay visible
 *   - missing required operator-added fields surface as problems, never invented
 *   - the primary class and outcome state must come from the accepted SD-15
 *     vocabulary, not from tester narrative or improvised labels
 *   - the draft never claims submission, reconciliation, closure, or authoritative
 *     triage completion
 */

import {
  buildOperatorTriageDraft,
  PRIMARY_CLASSES,
  OUTCOME_STATES,
  REPRODUCTION_STATUSES,
  AUTHORITY_DISCLAIMERS,
  type OperatorInput,
} from './buildOperatorTriageDraft';
import { assembleFeedbackEvidence } from '../testerWorkbench/feedback/evidence';
import type { FeedbackEvidencePayload } from '../testerWorkbench/feedback/evidence';
import type { TesterWorkbenchSurface } from '../testerWorkbench/loadTesterWorkbenchSurface';
import { assert, assertEqual } from '../testSupport/asserts';
import { makeSurface } from '../testSupport/makeSurface';

function completeBugPayload(surface: TesterWorkbenchSurface): FeedbackEvidencePayload {
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

function completeOperatorInput(overrides: Partial<OperatorInput> = {}): OperatorInput {
  return {
    receiptId: 'SD15-RR-20260701-001',
    createdAt: '2026-07-01T10:00:00Z',
    lastUpdatedAt: '2026-07-01T10:05:00Z',
    primaryClass: 'rules-engine-defect',
    outcomeState: 'defect',
    reproductionStatus: 'reproduced',
    reproductionStepsOrImpossibilityNote: 'Followed the tester launch path and reproduced on the governed alpha build.',
    adjacentAuthorityReferences: [
      'programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md',
    ],
    evidenceSufficiencyNote: 'Sufficient for rules-engine triage; regression rerun receipt still needed for closure use.',
    nextRequiredSurface: 'regression evidence receipt with SD-13 progression context',
    ...overrides,
  };
}

async function main() {
  exposesOnlyDictionaryVocabulary();
  partitionsPreserveTesterAutoOperatorSplit();
  completeOperatorInputProducesCompleteDraft();
  missingRequiredOperatorFieldsReportedNotInvented();
  invalidPrimaryClassRejectedNotCoerced();
  invalidOutcomeAndReproductionVocabularyEnforced();
  operatorJudgmentDrivesClassNotTesterNarrative();
  authorityDisclaimersPresentAndNoSuccessClaims();
  redactionAndAbsenceMarkersPreservedNotDropped();
  textDraftIsCopyableAndPreservesEvidence();
  evidencePayloadProblemsSurfacedSeparately();
  completeDraftStillCarriesNonClaimMarkers();
}

function exposesOnlyDictionaryVocabulary() {
  // The eight SD-15 triage classes from triage-class-dictionary.md, no invented extras.
  assertEqual(PRIMARY_CLASSES.length, 8, 'exactly the eight SD-15 primary classes are defined');
  const ids = PRIMARY_CLASSES.map((c) => c.id);
  for (const expected of [
    'ui-or-presentation-defect',
    'rules-engine-defect',
    'content-or-data-defect',
    'unsupported-semantics',
    'packaging-or-distribution-defect',
    'install-use-defect',
    'persistence-migration-continuity-defect',
    'status-or-documentation-drift',
  ]) {
    assert(ids.includes(expected as (typeof ids)[number]), `primary class ${expected} is present`);
  }

  // Outcome vocabulary from the dictionary / receipt schema.
  for (const expected of ['defect', 'unsupported', 'partial', 'not-yet-verified', 'blocked', 'status-drift']) {
    assert(
      (OUTCOME_STATES as readonly string[]).includes(expected),
      `outcome state ${expected} is present`
    );
  }
  assertEqual(OUTCOME_STATES.length, 6, 'exactly the six outcome states are defined');

  // Reproduction-status vocabulary from receipt schema section 6.
  for (const expected of [
    'reproduced',
    'not-reproduced',
    'not-re-run',
    'blocked',
    'unsupported',
    'partial',
    'status-drift-confirmed',
  ]) {
    assert(
      (REPRODUCTION_STATUSES as readonly string[]).includes(expected),
      `reproduction status ${expected} is present`
    );
  }
}

function partitionsPreserveTesterAutoOperatorSplit() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput(),
  });

  const { partitions } = result.draft;

  const observed = partitions.testerSupplied.find((e) => e.key === 'observedBehavior');
  assert(!!observed, 'tester-supplied partition includes observed behavior');
  assert(!!observed && observed.value === 'The preview crashed when computing baseline AC.', 'observed value preserved verbatim');

  const build = partitions.autoCaptured.find((e) => e.key === 'buildLabel');
  assert(!!build, 'auto-captured partition includes build label');
  assert(!!build && build.value === 'Codex 0.14.0-test', 'build label preserved from auto-captured evidence');

  const primaryClass = partitions.operatorAdded.find((e) => e.key === 'primaryClass');
  assert(!!primaryClass, 'operator-added partition includes primary class');
  assert(
    !!primaryClass && (primaryClass.value ?? '').includes('rules-engine'),
    'operator primary class preserved in operator-added partition'
  );

  // The three partitions must be genuinely distinct — a tester field must not leak
  // into the auto-captured partition and vice versa.
  assert(
    !partitions.autoCaptured.some((e) => e.key === 'observedBehavior'),
    'tester field does not leak into auto-captured partition'
  );
  assert(
    !partitions.testerSupplied.some((e) => e.key === 'buildLabel'),
    'auto-captured field does not leak into tester-supplied partition'
  );
}

function completeOperatorInputProducesCompleteDraft() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput(),
  });

  assertEqual(result.complete, true, 'complete evidence + complete operator input yields a complete draft');
  assertEqual(result.completenessProblems.length, 0, 'no operator completeness problems on a complete draft');
  assertEqual(result.draft.classRecognized, true, 'known primary class is recognized');
  assertEqual(result.draft.outcomeStateRecognized, true, 'known outcome state is recognized');
  assertEqual(result.draft.kind, 'operator-triage-receipt-draft', 'draft carries its receipt-draft kind marker');
}

function missingRequiredOperatorFieldsReportedNotInvented() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput({
      evidenceSufficiencyNote: '   ',
      adjacentAuthorityReferences: [],
      receiptId: '',
    }),
  });

  assertEqual(result.complete, false, 'missing required operator fields make the draft incomplete');
  assert(
    result.completenessProblems.some((p) => /evidence sufficiency/i.test(p)),
    'missing evidence sufficiency note is reported'
  );
  assert(
    result.completenessProblems.some((p) => /adjacent[- ]authority/i.test(p)),
    'missing adjacent-authority reference is reported'
  );
  assert(
    result.completenessProblems.some((p) => /receipt id/i.test(p)),
    'missing receipt id is reported'
  );

  // The absent fields must not be silently invented into a plausible value.
  assert(
    !result.textDraft.includes('undefined') && !result.textDraft.includes('null'),
    'absent fields are not rendered as raw undefined/null'
  );
  assert(
    /not provided|not-captured|unknown|missing/i.test(result.textDraft),
    'the text draft carries an explicit absence marker for missing operator data'
  );
}

function invalidPrimaryClassRejectedNotCoerced() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput({ primaryClass: 'totally-made-up-class' }),
  });

  assertEqual(result.draft.classRecognized, false, 'an invented primary class is not recognized');
  assertEqual(result.complete, false, 'an invented primary class makes the draft incomplete');
  assert(
    result.completenessProblems.some((p) => /primary.*class/i.test(p) && /totally-made-up-class/.test(p)),
    'the invalid class value is named in a problem'
  );
  // It must not be silently coerced to a real class.
  assertEqual(
    result.draft.workflowAndAuthorityContext.primaryClassId,
    null,
    'an unrecognized class id resolves to null rather than a fabricated dictionary class'
  );
}

function invalidOutcomeAndReproductionVocabularyEnforced() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput({ outcomeState: 'wont-fix', reproductionStatus: 'maybe' }),
  });

  assertEqual(result.draft.outcomeStateRecognized, false, 'an out-of-vocabulary outcome state is not recognized');
  assert(
    result.completenessProblems.some((p) => /outcome state/i.test(p) && /wont-fix/.test(p)),
    'the invalid outcome state is named in a problem'
  );
  assert(
    result.completenessProblems.some((p) => /reproduction status/i.test(p) && /maybe/.test(p)),
    'the invalid reproduction status is named in a problem'
  );
  assertEqual(result.complete, false, 'invalid vocabulary makes the draft incomplete');
}

function operatorJudgmentDrivesClassNotTesterNarrative() {
  const surface = makeSurface();
  // Same tester narrative, two different operator classifications — the class must
  // follow the operator field, proving no auto-classification from narrative.
  const asRules = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput({ primaryClass: 'rules-engine-defect' }),
  });
  const asUnsupported = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput({ primaryClass: 'unsupported-semantics', outcomeState: 'unsupported' }),
  });

  assertEqual(
    asRules.draft.workflowAndAuthorityContext.primaryClassId,
    'rules-engine-defect',
    'operator rules-engine classification is honored'
  );
  assertEqual(
    asUnsupported.draft.workflowAndAuthorityContext.primaryClassId,
    'unsupported-semantics',
    'operator unsupported classification is honored for identical narrative'
  );
}

function authorityDisclaimersPresentAndNoSuccessClaims() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput(),
  });

  assert(AUTHORITY_DISCLAIMERS.length > 0, 'authority disclaimers are defined');
  assert(result.draft.authorityDisclaimers.length > 0, 'the draft carries authority disclaimers');
  const disclaimerBlob = result.draft.authorityDisclaimers.join(' ').toLowerCase();
  assert(disclaimerBlob.includes('draft'), 'disclaimers state this is a draft');
  assert(/not.*submit|no.*submission/.test(disclaimerBlob), 'disclaimers deny submission');
  assert(/not.*reconcil|no.*reconciliation/.test(disclaimerBlob), 'disclaimers deny reconciliation');
  assert(/not.*closure|no.*closure/.test(disclaimerBlob), 'disclaimers deny closure');

  // The text draft must not assert positive submission/closure/reconciliation success.
  const text = result.textDraft.toLowerCase();
  assert(!/\bsubmitted\b/.test(text), 'text draft never claims the report was submitted');
  assert(!/\breconciled\b/.test(text), 'text draft never claims reconciliation succeeded');
  assert(!/closure-ready|tranche closed|closure verdict/.test(text), 'text draft never claims closure');
}

function redactionAndAbsenceMarkersPreservedNotDropped() {
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
  const result = buildOperatorTriageDraft({
    evidence: payload,
    // omit the optional next-required-surface to exercise absence markers
    operator: completeOperatorInput({ nextRequiredSurface: null }),
  });

  const repro = result.draft.partitions.testerSupplied.find((e) => e.key === 'reproductionSteps');
  assert(!!repro && repro.state === 'redacted', 'redacted tester field keeps its redacted state');
  assert(!!repro && repro.value === null, 'redacted content is not leaked as a value');
  assert(!result.textDraft.includes('secret repro path'), 'redacted content is not leaked into the text draft');

  // A missing optional operator field surfaces as an explicit absence marker, not blank.
  assert(
    /next required surface[^\n]*(not provided|not-captured|unknown)/i.test(result.textDraft),
    'a missing optional operator field renders an explicit absence marker'
  );
}

function textDraftIsCopyableAndPreservesEvidence() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput(),
  });

  const text = result.textDraft;
  assertEqual(typeof text, 'string', 'text draft is a string');
  assert(text.length > 0, 'text draft is non-empty');
  assert(text.includes('SD15-RR-20260701-001'), 'text draft carries the receipt id');
  assert(text.includes('Codex 0.14.0-test'), 'text draft carries the build label');
  assert(text.includes('The preview crashed when computing baseline AC.'), 'text draft carries observed behavior');
  assert(text.includes('rules-engine'), 'text draft carries the operator primary class');
  assert(/tester-supplied/i.test(text), 'text draft labels the tester-supplied partition');
  assert(/auto-captured/i.test(text), 'text draft labels the auto-captured partition');
  assert(/operator-added/i.test(text), 'text draft labels the operator-added partition');
}

function evidencePayloadProblemsSurfacedSeparately() {
  const surface = makeSurface();
  // A bug payload missing required narrative is itself incomplete.
  const incompleteEvidence = assembleFeedbackEvidence({
    flow: 'bug',
    surface,
    testerInput: { observedBehavior: 'crash' },
  });
  const result = buildOperatorTriageDraft({
    evidence: incompleteEvidence,
    operator: completeOperatorInput(),
  });

  assert(result.evidenceProblems.length > 0, 'evidence payload problems are surfaced');
  assert(
    result.evidenceProblems.some((p) => /Expected behavior/i.test(p)),
    'the underlying evidence completeness problem is preserved'
  );
  assertEqual(result.complete, false, 'incomplete underlying evidence blocks overall completeness');
  // Operator fields themselves were complete, so operator problems stay empty.
  assertEqual(result.completenessProblems.length, 0, 'operator completeness is reported separately from evidence completeness');
}

function completeDraftStillCarriesNonClaimMarkers() {
  const surface = makeSurface();
  const result = buildOperatorTriageDraft({
    evidence: completeBugPayload(surface),
    operator: completeOperatorInput(),
  });

  // Even a fully complete draft must not read as authoritative triage completion.
  assertEqual(result.complete, true, 'the draft is complete for its inputs');
  assert(
    result.draft.authorityDisclaimers.length > 0,
    'a complete draft still carries non-claim disclaimers'
  );
  assert(
    /draft/i.test(result.textDraft.split('\n')[0] ?? ''),
    'the text draft headline still identifies the artifact as a draft'
  );
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
