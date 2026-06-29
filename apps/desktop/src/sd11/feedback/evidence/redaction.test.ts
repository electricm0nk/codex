import {
  evaluateAttachment,
  requiresRedactionDeclaration,
  validateRedaction,
  REDACTION_POLICY_NOTICE,
  type EvidenceAttachment,
} from './redaction';

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

const baseAttachment: EvidenceAttachment = {
  id: 'a1',
  label: 'workbench-screenshot.png',
  kind: 'screenshot',
  mayContainSensitiveData: false,
  testerConfirmedInclude: false,
  redacted: false,
};

async function main() {
  nonSensitiveStillNeedsExplicitConfirmation();
  sensitiveUnconfirmedUnredactedIsBlocked();
  sensitiveRedactedIsIncludedAsRedacted();
  sensitiveConfirmedIsIncluded();
  declarationRequiredOnlyWhenAttachmentsExist();
  declarationMustBePresentWhenAttachmentsIncluded();
  policyNoticeIsNonEmpty();
}

function nonSensitiveStillNeedsExplicitConfirmation() {
  const decision = evaluateAttachment(baseAttachment);
  assertEqual(decision.outcome, 'requires-confirmation', 'unconfirmed attachment must not be silently included');
}

function sensitiveUnconfirmedUnredactedIsBlocked() {
  const decision = evaluateAttachment({ ...baseAttachment, mayContainSensitiveData: true });
  assertEqual(decision.outcome, 'requires-confirmation', 'sensitive attachment must be redacted or confirmed');
}

function sensitiveRedactedIsIncludedAsRedacted() {
  const decision = evaluateAttachment({ ...baseAttachment, mayContainSensitiveData: true, redacted: true });
  assertEqual(decision.outcome, 'redacted', 'sensitive but redacted attachment is included as redacted');
}

function sensitiveConfirmedIsIncluded() {
  const decision = evaluateAttachment({
    ...baseAttachment,
    mayContainSensitiveData: true,
    testerConfirmedInclude: true,
  });
  assertEqual(decision.outcome, 'included', 'sensitive attachment with explicit confirmation is included');
}

function declarationRequiredOnlyWhenAttachmentsExist() {
  assertEqual(requiresRedactionDeclaration([]), false, 'no attachments -> no declaration required');
  assertEqual(
    requiresRedactionDeclaration([{ ...baseAttachment, testerConfirmedInclude: true }]),
    true,
    'attachments present -> declaration required'
  );
}

function declarationMustBePresentWhenAttachmentsIncluded() {
  const attachments: EvidenceAttachment[] = [
    { ...baseAttachment, mayContainSensitiveData: true, redacted: true },
  ];

  const missing = validateRedaction(attachments, { declared: false, statement: null });
  assertEqual(missing.ok, false, 'missing declaration with attachments is invalid');
  assert(missing.problems.length > 0, 'missing declaration must report a problem');

  const present = validateRedaction(attachments, {
    declared: true,
    statement: 'Sensitive paths scrubbed from the attached screenshot before inclusion.',
  });
  assertEqual(present.ok, true, 'declared redaction with statement is valid');
  assertEqual(present.problems.length, 0, 'valid declaration has no problems');

  // A blocked attachment (requires confirmation) makes the set invalid regardless of declaration.
  const blocked = validateRedaction(
    [{ ...baseAttachment, mayContainSensitiveData: true }],
    { declared: true, statement: 'scrubbed' }
  );
  assertEqual(blocked.ok, false, 'an unresolved sensitive attachment blocks the payload');
}

function policyNoticeIsNonEmpty() {
  assert(REDACTION_POLICY_NOTICE.length > 0, 'redaction policy notice must be present for UI surfacing');
}

main().catch((error: unknown) => {
  console.error(error);
  throw error;
});
