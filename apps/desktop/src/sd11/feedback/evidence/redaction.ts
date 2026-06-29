/**
 * Attachment and redaction rules for SD-11 tester feedback.
 *
 * These rules are intentionally explicit so neither flow can capture attachments
 * silently or ship sensitive content without a recorded decision. Mirrors the
 * redaction rules in the tester-feedback-evidence-capture-matrix.
 */

export type AttachmentKind = 'screenshot' | 'log' | 'save-file' | 'other';

export interface EvidenceAttachment {
  id: string;
  label: string;
  kind: AttachmentKind;
  /** Whether this attachment could contain user-sensitive data. */
  mayContainSensitiveData: boolean;
  /** Explicit tester confirmation to include the attachment as-is. */
  testerConfirmedInclude: boolean;
  /** Whether the attachment content has been scrubbed/redacted. */
  redacted: boolean;
}

export type AttachmentOutcome = 'included' | 'redacted' | 'requires-confirmation';

export interface AttachmentDecision {
  attachment: EvidenceAttachment;
  outcome: AttachmentOutcome;
  reason: string;
}

export interface RedactionDeclaration {
  declared: boolean;
  statement: string | null;
}

export interface RedactionValidation {
  ok: boolean;
  problems: string[];
}

export const REDACTION_POLICY_NOTICE =
  'Attachments are never captured silently. Anything that may contain user-sensitive data must be ' +
  'redacted or explicitly confirmed by the tester, and when any attachment is included the payload ' +
  'must declare that redaction occurred so triage understands intentionally missing context.';

/**
 * Decide how a single attachment may be handled. The core rule: nothing is
 * silently included, and sensitive content must be redacted or explicitly
 * confirmed before it can ship.
 */
export function evaluateAttachment(attachment: EvidenceAttachment): AttachmentDecision {
  if (attachment.redacted) {
    return {
      attachment,
      outcome: 'redacted',
      reason: 'Content was redacted/scrubbed before inclusion.',
    };
  }

  if (attachment.mayContainSensitiveData && !attachment.testerConfirmedInclude) {
    return {
      attachment,
      outcome: 'requires-confirmation',
      reason: 'May contain sensitive data; must be redacted or explicitly confirmed before inclusion.',
    };
  }

  if (!attachment.testerConfirmedInclude) {
    return {
      attachment,
      outcome: 'requires-confirmation',
      reason: 'No attachment is included without explicit tester confirmation.',
    };
  }

  return {
    attachment,
    outcome: 'included',
    reason: 'Tester explicitly confirmed inclusion.',
  };
}

/** A redaction declaration is required whenever any attachment is part of the payload. */
export function requiresRedactionDeclaration(attachments: EvidenceAttachment[]): boolean {
  return attachments.length > 0;
}

/**
 * Validate the attachment set against its redaction declaration. The payload is
 * only valid when every attachment is resolved (included or redacted) and, if any
 * attachments exist, a redaction declaration with a statement is present.
 */
export function validateRedaction(
  attachments: EvidenceAttachment[],
  declaration: RedactionDeclaration
): RedactionValidation {
  const problems: string[] = [];

  for (const decision of attachments.map(evaluateAttachment)) {
    if (decision.outcome === 'requires-confirmation') {
      problems.push(`Attachment "${decision.attachment.label}" is unresolved: ${decision.reason}`);
    }
  }

  if (requiresRedactionDeclaration(attachments)) {
    if (!declaration.declared || !declaration.statement || declaration.statement.trim().length === 0) {
      problems.push(
        'A redaction declaration with a statement is required when attachments are included.'
      );
    }
  }

  return { ok: problems.length === 0, problems };
}
