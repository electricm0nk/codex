/**
 * Shared SD-11 feedback evidence-capture and redaction substrate.
 *
 * Consumed by both the GitHub bug-report flow and the GitHub enhancement-request
 * flow so the captured evidence schema and redaction rules cannot diverge.
 */

export * from './evidenceFields';
export * from './redaction';
export * from './captureFeedbackEvidence';
