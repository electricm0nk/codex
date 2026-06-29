import {
  SHARED_EVIDENCE_FIELDS,
  fieldsForFlow,
  sharedAutoCapturedFields,
  evidenceFieldByKey,
  type EvidenceFieldDescriptor,
} from './evidenceFields';

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

async function main() {
  catalogHasNoDuplicateKeys();
  bugAndEnhancementShareTheSameAutoCaptureSchema();
  bugOnlyAndEnhancementOnlyFieldsStaySeparated();
  redactionDeclarationIsAttachmentGated();
}

function catalogHasNoDuplicateKeys() {
  const keys = SHARED_EVIDENCE_FIELDS.map((field) => field.key);
  const unique = new Set(keys);
  assertEqual(unique.size, keys.length, 'evidence field keys must be unique');

  for (const field of SHARED_EVIDENCE_FIELDS) {
    assert(field.appliesTo.length > 0, `field ${field.key} must apply to at least one flow`);
  }
}

function bugAndEnhancementShareTheSameAutoCaptureSchema() {
  const bugAuto = autoKeys(fieldsForFlow('bug'));
  const enhancementAuto = autoKeys(fieldsForFlow('enhancement'));
  const sharedAuto = autoKeys(sharedAutoCapturedFields());

  // No divergent schema drift: every shared auto field is present in both flows.
  for (const key of sharedAuto) {
    assert(bugAuto.includes(key), `bug flow missing shared auto field ${key}`);
    assert(enhancementAuto.includes(key), `enhancement flow missing shared auto field ${key}`);
  }

  // The required always-on auto fields are exactly the shared backbone.
  const backbone = sharedAutoCapturedFields()
    .filter((field) => field.requirement === 'required')
    .map((field) => field.key)
    .sort();
  assertEqual(
    backbone.join(','),
    ['buildLabel', 'channelSupportLabel', 'currentWorkflow', 'dataSourceIdentity', 'platformLabel'].sort().join(','),
    'shared required auto backbone'
  );
}

function bugOnlyAndEnhancementOnlyFieldsStaySeparated() {
  const observed = evidenceFieldByKey('observedBehavior');
  assertEqual(observed.captureMode, 'tester-entered', 'observed behavior capture mode');
  assertEqual(observed.appliesTo.includes('bug'), true, 'observed behavior applies to bug');
  assertEqual(observed.appliesTo.includes('enhancement'), false, 'observed behavior excludes enhancement');

  const goal = evidenceFieldByKey('testerGoal');
  assertEqual(goal.appliesTo.includes('enhancement'), true, 'tester goal applies to enhancement');
  assertEqual(goal.appliesTo.includes('bug'), false, 'tester goal excludes bug');
}

function redactionDeclarationIsAttachmentGated() {
  const declaration = evidenceFieldByKey('redactionDeclaration');
  assertEqual(
    declaration.requirement,
    'required-when-attachments-present',
    'redaction declaration requirement'
  );
  const attachments = evidenceFieldByKey('attachments');
  assertEqual(attachments.requirement, 'optional', 'attachments requirement');
  assertEqual(attachments.redactable, true, 'attachments are redactable');
}

function autoKeys(fields: EvidenceFieldDescriptor[]): string[] {
  return fields.filter((field) => field.captureMode === 'auto-captured').map((field) => field.key);
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
