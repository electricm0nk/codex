/**
 * Shared SD-11 evidence field catalog.
 *
 * This is the single source of truth for which structured evidence fields the
 * tester feedback flows capture, how each field is captured, and which flows it
 * applies to. Both the GitHub bug-report flow and the GitHub enhancement-request
 * flow consume this catalog so the shared backbone cannot drift between them.
 *
 * Source of record:
 *   programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/
 *     artifacts/tester-feedback-evidence-capture-matrix.md
 */

export type FeedbackFlowKind = 'bug' | 'enhancement';

/** How a field's value originates. */
export type EvidenceCaptureMode = 'auto-captured' | 'tester-entered';

/** Whether and when a field must be populated before a payload is complete. */
export type EvidenceRequirement =
  | 'required'
  | 'optional'
  | 'required-when-present'
  | 'required-when-attachments-present';

/**
 * The resolved per-field state for a concrete captured payload. This is the
 * distinction the substrate must make visible: a field is either auto-captured,
 * tester-entered, left optional/empty, or redacted (had content that was
 * scrubbed or intentionally omitted).
 */
export type EvidenceFieldState = 'auto-captured' | 'tester-entered' | 'optional' | 'redacted';

export type EvidenceFieldKey =
  | 'buildLabel'
  | 'channelSupportLabel'
  | 'platformLabel'
  | 'currentWorkflow'
  | 'dataSourceIdentity'
  | 'observedBehavior'
  | 'expectedBehavior'
  | 'reproductionSteps'
  | 'testerGoal'
  | 'currentFriction'
  | 'requestedCapability'
  | 'affectedSurface'
  | 'diagnostics'
  | 'explanationRefs'
  | 'provenanceRefs'
  | 'attachments'
  | 'redactionDeclaration';

export interface EvidenceFieldDescriptor {
  key: EvidenceFieldKey;
  label: string;
  captureMode: EvidenceCaptureMode;
  requirement: EvidenceRequirement;
  appliesTo: FeedbackFlowKind[];
  /** Whether this field can carry user-sensitive content subject to redaction rules. */
  redactable: boolean;
  note: string;
}

const BOTH: FeedbackFlowKind[] = ['bug', 'enhancement'];

/**
 * The shared evidence field catalog. The auto-captured backbone is identical for
 * both flows; only the tester-entered narrative fields branch by flow.
 */
export const SHARED_EVIDENCE_FIELDS: EvidenceFieldDescriptor[] = [
  {
    key: 'buildLabel',
    label: 'Build label/version',
    captureMode: 'auto-captured',
    requirement: 'required',
    appliesTo: BOTH,
    redactable: false,
    note: 'Comes from the running app/build surface.',
  },
  {
    key: 'channelSupportLabel',
    label: 'Tester-facing channel/support label',
    captureMode: 'auto-captured',
    requirement: 'required',
    appliesTo: BOTH,
    redactable: false,
    note: 'Operator branch lineage stays internal; the tester-facing label is what the tester sees.',
  },
  {
    key: 'platformLabel',
    label: 'Platform / OS',
    captureMode: 'auto-captured',
    requirement: 'required',
    appliesTo: BOTH,
    redactable: false,
    note: 'Include package/install context later when available.',
  },
  {
    key: 'currentWorkflow',
    label: 'Current bounded workflow',
    captureMode: 'auto-captured',
    requirement: 'required',
    appliesTo: BOTH,
    redactable: false,
    note: 'Identifies the active tester flow.',
  },
  {
    key: 'dataSourceIdentity',
    label: 'Current data-source identity',
    captureMode: 'auto-captured',
    requirement: 'required',
    appliesTo: BOTH,
    redactable: false,
    note: 'Distinguishes real command data from placeholder/fallback state.',
  },
  {
    key: 'observedBehavior',
    label: 'Observed behavior',
    captureMode: 'tester-entered',
    requirement: 'required',
    appliesTo: ['bug'],
    redactable: true,
    note: 'What happened.',
  },
  {
    key: 'expectedBehavior',
    label: 'Expected behavior',
    captureMode: 'tester-entered',
    requirement: 'required',
    appliesTo: ['bug'],
    redactable: true,
    note: 'What should have happened.',
  },
  {
    key: 'reproductionSteps',
    label: 'Reproduction steps',
    captureMode: 'tester-entered',
    requirement: 'required',
    appliesTo: ['bug'],
    redactable: true,
    note: 'Explicit steps when reproducible.',
  },
  {
    key: 'testerGoal',
    label: 'Tester goal',
    captureMode: 'tester-entered',
    requirement: 'required',
    appliesTo: ['enhancement'],
    redactable: true,
    note: 'What the tester was trying to accomplish.',
  },
  {
    key: 'currentFriction',
    label: 'Current friction / limitation',
    captureMode: 'tester-entered',
    requirement: 'required',
    appliesTo: ['enhancement'],
    redactable: true,
    note: 'What blocked or slowed the goal.',
  },
  {
    key: 'requestedCapability',
    label: 'Requested capability / improvement',
    captureMode: 'tester-entered',
    requirement: 'required',
    appliesTo: ['enhancement'],
    redactable: true,
    note: 'Desired change.',
  },
  {
    key: 'affectedSurface',
    label: 'Affected surface',
    captureMode: 'tester-entered',
    requirement: 'required',
    appliesTo: ['enhancement'],
    redactable: true,
    note: 'Workbench area, workflow slice, or update/support surface.',
  },
  {
    key: 'diagnostics',
    label: 'Diagnostics / blocked claims',
    captureMode: 'auto-captured',
    requirement: 'required-when-present',
    appliesTo: BOTH,
    redactable: false,
    note: 'Preserve severity/class/reason fields when the UI has them. Required for bug when present.',
  },
  {
    key: 'explanationRefs',
    label: 'Explanation references',
    captureMode: 'auto-captured',
    requirement: 'optional',
    appliesTo: BOTH,
    redactable: false,
    note: 'Useful when available; do not fabricate.',
  },
  {
    key: 'provenanceRefs',
    label: 'Provenance references',
    captureMode: 'auto-captured',
    requirement: 'optional',
    appliesTo: BOTH,
    redactable: false,
    note: 'Useful when available; do not fabricate.',
  },
  {
    key: 'attachments',
    label: 'Screenshot / attachment references',
    captureMode: 'tester-entered',
    requirement: 'optional',
    appliesTo: BOTH,
    redactable: true,
    note: 'Allowed only with explicit attachment handling; never captured silently.',
  },
  {
    key: 'redactionDeclaration',
    label: 'Redaction declaration',
    captureMode: 'tester-entered',
    requirement: 'required-when-attachments-present',
    appliesTo: BOTH,
    redactable: false,
    note: 'Must state that sensitive content was scrubbed or omitted when attachments/logs are included.',
  },
];

const FIELD_INDEX: Map<EvidenceFieldKey, EvidenceFieldDescriptor> = new Map(
  SHARED_EVIDENCE_FIELDS.map((field) => [field.key, field])
);

export function evidenceFieldByKey(key: EvidenceFieldKey): EvidenceFieldDescriptor {
  const field = FIELD_INDEX.get(key);
  if (!field) {
    throw new Error(`Unknown evidence field key: ${key}`);
  }
  return field;
}

export function fieldsForFlow(flow: FeedbackFlowKind): EvidenceFieldDescriptor[] {
  return SHARED_EVIDENCE_FIELDS.filter((field) => field.appliesTo.includes(flow));
}

/** The auto-captured fields shared by every flow — the no-drift backbone. */
export function sharedAutoCapturedFields(): EvidenceFieldDescriptor[] {
  return SHARED_EVIDENCE_FIELDS.filter(
    (field) =>
      field.captureMode === 'auto-captured' &&
      field.appliesTo.includes('bug') &&
      field.appliesTo.includes('enhancement')
  );
}
