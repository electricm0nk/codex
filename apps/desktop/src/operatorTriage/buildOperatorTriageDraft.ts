/**
 * Headless SD-15 operator triage / regression-receipt draft composer.
 *
 * This module consumes an already-assembled SD-11 `FeedbackEvidencePayload`
 * (tester-supplied + auto-captured evidence) plus explicit operator-entered
 * SD-15 judgment fields, and emits:
 *   - a structured SD-15 triage / regression-receipt draft object, and
 *   - a copyable plain-text draft.
 *
 * It is deliberately narrow and honest. It follows the SD-15 boundary handoff and
 * the SD-15 artifacts:
 *   - artifacts/sd15-e6-automation-helper-boundary-handoff-2026-06-30.md
 *   - artifacts/intake-to-triage-mapping.md
 *   - artifacts/regression-receipt-schema.md
 *   - artifacts/triage-class-dictionary.md
 *
 * What it does NOT do (by construction):
 *   - it does not submit, transmit, or file anything (no transport)
 *   - it does not reconcile status surfaces or claim reconciliation success
 *   - it does not declare tranche closure or an authoritative triage completion
 *   - it does not auto-classify tester narrative into an SD-15 class; the primary
 *     class and outcome state are explicit operator inputs
 *   - it never invents a value for a missing required operator field; it reports a
 *     completeness problem and preserves an explicit absence marker instead
 *
 * The tester-supplied, auto-captured, and operator-added evidence partitions stay
 * visibly separate so a later reader can tell what the tester observed, what the
 * product captured, and what the operator concluded afterward.
 */

import type {
  CapturedEvidenceField,
  FeedbackEvidencePayload,
} from '../testerWorkbench/feedback/evidence';

/** The eight SD-15 primary triage classes from `triage-class-dictionary.md`. */
export interface TriageClassDescriptor {
  id: PrimaryClassId;
  label: string;
}

export type PrimaryClassId =
  | 'ui-or-presentation-defect'
  | 'rules-engine-defect'
  | 'content-or-data-defect'
  | 'unsupported-semantics'
  | 'packaging-or-distribution-defect'
  | 'install-use-defect'
  | 'persistence-migration-continuity-defect'
  | 'status-or-documentation-drift';

export const PRIMARY_CLASSES: readonly TriageClassDescriptor[] = [
  { id: 'ui-or-presentation-defect', label: 'UI or presentation defect' },
  { id: 'rules-engine-defect', label: 'Rules-engine defect' },
  { id: 'content-or-data-defect', label: 'Content or data defect' },
  { id: 'unsupported-semantics', label: 'Unsupported semantics or known unsupported paths' },
  { id: 'packaging-or-distribution-defect', label: 'Packaging or distribution defect' },
  { id: 'install-use-defect', label: 'Install/use defect' },
  { id: 'persistence-migration-continuity-defect', label: 'Persistence, migration, or saved-state continuity defect' },
  { id: 'status-or-documentation-drift', label: 'Status or documentation drift' },
];

/** The SD-15 outcome vocabulary from the dictionary / receipt schema. */
export type OutcomeState =
  | 'defect'
  | 'unsupported'
  | 'partial'
  | 'not-yet-verified'
  | 'blocked'
  | 'status-drift';

export const OUTCOME_STATES: readonly OutcomeState[] = [
  'defect',
  'unsupported',
  'partial',
  'not-yet-verified',
  'blocked',
  'status-drift',
];

/** The SD-15 reproduction-status vocabulary from receipt-schema section 6. */
export type ReproductionStatus =
  | 'reproduced'
  | 'not-reproduced'
  | 'not-re-run'
  | 'blocked'
  | 'unsupported'
  | 'partial'
  | 'status-drift-confirmed';

export const REPRODUCTION_STATUSES: readonly ReproductionStatus[] = [
  'reproduced',
  'not-reproduced',
  'not-re-run',
  'blocked',
  'unsupported',
  'partial',
  'status-drift-confirmed',
];

/**
 * Fixed non-claim markers. Every emitted draft carries these so no downstream
 * reader can mistake a draft for a submission, reconciliation, or closure.
 * (Phrasing intentionally avoids the words "submitted"/"reconciled" so the draft
 * text can never be misread as a positive success claim.)
 */
export const AUTHORITY_DISCLAIMERS: readonly string[] = [
  'This is an operator-entered SD-15 triage / regression-receipt DRAFT only.',
  'This draft performs no submission and files nothing; transport stays out of scope.',
  'This draft performs no status-surface reconciliation and asserts no reconciliation success.',
  'This draft asserts no tranche closure and no authoritative triage completion.',
  'The primary SD-15 class and outcome state are operator-entered judgments, not auto-classified from tester narrative.',
];

/** Explicit operator-entered SD-15 judgment fields. */
export interface OperatorInput {
  // Receipt identity and lineage (schema section 1)
  receiptId: string;
  createdAt: string;
  lastUpdatedAt: string;
  intakeHandle?: string | null;
  supersedesReceiptId?: string | null;
  evidenceOwner?: string | null;

  // Build and provenance context (schema section 2, operator-added parts)
  operatorProvenanceHandle?: string | null;
  commitOrBuildIdentity?: string | null;
  publicationOrAcquisitionHandle?: string | null;
  rollbackWithdrawalContext?: string | null;

  // Platform and install context (schema section 3, operator-added parts)
  platformArchitecture?: string | null;
  packageInstallContext?: string | null;
  environmentKind?: string | null;
  environmentIdentityHandle?: string | null;

  // Workflow and adjacent-authority context (schema section 4)
  primaryClass: string;
  outcomeState: string;
  adjacentAuthorityReferences: string[];
  supportStateContext?: string | null;
  sd14PersistenceMigrationContext?: string | null;

  // Claim statement (schema section 5, operator-added parts)
  claimSummary?: string | null;

  // Reproduction and diagnostics (schema section 6)
  reproductionStatus: string;
  reproductionStepsOrImpossibilityNote: string;
  evidenceSufficiencyNote: string;
  nextRequiredSurface?: string | null;
}

export interface OperatorTriageDraftInput {
  evidence: FeedbackEvidencePayload;
  operator: OperatorInput;
}

/** One visible evidence field inside a partition. */
export type PartitionState =
  | 'tester-entered'
  | 'auto-captured'
  | 'optional'
  | 'redacted'
  | 'operator-added'
  | 'operator-missing';

export interface PartitionEntry {
  key: string;
  label: string;
  value: string | null;
  state: PartitionState;
}

export interface EvidencePartitions {
  testerSupplied: PartitionEntry[];
  autoCaptured: PartitionEntry[];
  operatorAdded: PartitionEntry[];
}

export interface ReceiptIdentity {
  receiptId: string | null;
  intakeHandle: string | null;
  createdAt: string | null;
  lastUpdatedAt: string | null;
  supersedesReceiptId: string | null;
  evidenceOwner: string | null;
}

export interface BuildProvenanceContext {
  buildLabelOrVersion: string;
  testerChannelSupportLabel: string;
  operatorProvenanceHandle: string | null;
  commitOrBuildIdentity: string | null;
  publicationOrAcquisitionHandle: string | null;
  rollbackWithdrawalContext: string | null;
}

export interface PlatformInstallContext {
  platformOs: string;
  platformArchitecture: string | null;
  packageInstallContext: string | null;
  environmentKind: string | null;
  environmentIdentityHandle: string | null;
}

export interface WorkflowAndAuthorityContext {
  boundedWorkflowUnderTest: string;
  currentDataSourceIdentity: string;
  /** The resolved dictionary class id, or null when the operator value is unrecognized. */
  primaryClassId: PrimaryClassId | null;
  primaryClassLabel: string | null;
  /** The raw operator-entered class string, preserved even when unrecognized. */
  primaryClassInput: string | null;
  outcomeState: OutcomeState | null;
  outcomeStateInput: string | null;
  adjacentAuthorityReferences: string[];
  supportStateContext: string | null;
  sd14PersistenceMigrationContext: string | null;
}

export interface ClaimStatement {
  observedBehavior: string | null;
  expectedBehavior: string | null;
  claimSummary: string | null;
}

export interface ReproductionAndDiagnostics {
  reproductionStatus: ReproductionStatus | null;
  reproductionStatusInput: string | null;
  reproductionStepsOrImpossibilityNote: string | null;
  diagnosticsSummary: string;
  evidenceSufficiencyNote: string | null;
  nextRequiredSurface: string | null;
}

export interface AttachmentsAndRedaction {
  redactionPosture: string;
  redactionRequired: boolean;
  attachmentCount: number;
}

export interface OperatorTriageDraft {
  kind: 'operator-triage-receipt-draft';
  receiptIdentity: ReceiptIdentity;
  buildProvenanceContext: BuildProvenanceContext;
  platformInstallContext: PlatformInstallContext;
  workflowAndAuthorityContext: WorkflowAndAuthorityContext;
  claimStatement: ClaimStatement;
  reproductionAndDiagnostics: ReproductionAndDiagnostics;
  attachmentsAndRedaction: AttachmentsAndRedaction;
  partitions: EvidencePartitions;
  authorityDisclaimers: string[];
  classRecognized: boolean;
  outcomeStateRecognized: boolean;
  reproductionStatusRecognized: boolean;
}

export interface OperatorTriageDraftResult {
  draft: OperatorTriageDraft;
  /** Copyable plain-text rendering of the draft. */
  textDraft: string;
  /** True only when operator fields are complete AND the underlying evidence is complete. */
  complete: boolean;
  /** Problems for missing/invalid required operator-added fields. */
  completenessProblems: string[];
  /** Problems carried forward from the underlying SD-11 evidence payload. */
  evidenceProblems: string[];
}

const ABSENT_MARKER = 'not provided';

/**
 * Compose a headless SD-15 operator triage / regression-receipt draft from an
 * existing SD-11 evidence payload plus explicit operator SD-15 fields.
 */
export function buildOperatorTriageDraft(
  input: OperatorTriageDraftInput
): OperatorTriageDraftResult {
  const { evidence, operator } = input;
  const auto = evidence.auto;

  const primaryClassInput = blankToNull(operator.primaryClass);
  const primaryClass = primaryClassInput
    ? PRIMARY_CLASSES.find((candidate) => candidate.id === primaryClassInput) ?? null
    : null;
  const classRecognized = primaryClass !== null;

  const outcomeStateInput = blankToNull(operator.outcomeState);
  const outcomeState = outcomeStateInput && isOutcomeState(outcomeStateInput) ? outcomeStateInput : null;
  const outcomeStateRecognized = outcomeState !== null;

  const reproductionStatusInput = blankToNull(operator.reproductionStatus);
  const reproductionStatus =
    reproductionStatusInput && isReproductionStatus(reproductionStatusInput) ? reproductionStatusInput : null;
  const reproductionStatusRecognized = reproductionStatus !== null;

  const adjacentAuthorityReferences = (operator.adjacentAuthorityReferences ?? [])
    .map((reference) => reference.trim())
    .filter((reference) => reference.length > 0);

  const completenessProblems = collectOperatorProblems({
    operator,
    primaryClassInput,
    classRecognized,
    outcomeStateInput,
    outcomeStateRecognized,
    reproductionStatusInput,
    reproductionStatusRecognized,
    adjacentAuthorityReferences,
  });

  const evidenceProblems = [...evidence.problems];

  const partitions = buildPartitions(evidence, operator, {
    primaryClassInput,
    primaryClass,
    outcomeStateInput,
    reproductionStatusInput,
    adjacentAuthorityReferences,
  });

  const draft: OperatorTriageDraft = {
    kind: 'operator-triage-receipt-draft',
    receiptIdentity: {
      receiptId: blankToNull(operator.receiptId),
      intakeHandle: blankToNull(operator.intakeHandle),
      createdAt: blankToNull(operator.createdAt),
      lastUpdatedAt: blankToNull(operator.lastUpdatedAt),
      supersedesReceiptId: blankToNull(operator.supersedesReceiptId),
      evidenceOwner: blankToNull(operator.evidenceOwner),
    },
    buildProvenanceContext: {
      buildLabelOrVersion: auto.buildLabel,
      testerChannelSupportLabel: auto.channelSupportLabel,
      operatorProvenanceHandle: blankToNull(operator.operatorProvenanceHandle),
      commitOrBuildIdentity: blankToNull(operator.commitOrBuildIdentity),
      publicationOrAcquisitionHandle: blankToNull(operator.publicationOrAcquisitionHandle),
      rollbackWithdrawalContext: blankToNull(operator.rollbackWithdrawalContext),
    },
    platformInstallContext: {
      platformOs: auto.platformLabel,
      platformArchitecture: blankToNull(operator.platformArchitecture),
      packageInstallContext: blankToNull(operator.packageInstallContext),
      environmentKind: blankToNull(operator.environmentKind),
      environmentIdentityHandle: blankToNull(operator.environmentIdentityHandle),
    },
    workflowAndAuthorityContext: {
      boundedWorkflowUnderTest: auto.currentWorkflow,
      currentDataSourceIdentity: auto.dataSourceIdentity,
      primaryClassId: primaryClass?.id ?? null,
      primaryClassLabel: primaryClass?.label ?? null,
      primaryClassInput,
      outcomeState,
      outcomeStateInput,
      adjacentAuthorityReferences,
      supportStateContext: blankToNull(operator.supportStateContext),
      sd14PersistenceMigrationContext: blankToNull(operator.sd14PersistenceMigrationContext),
    },
    claimStatement: {
      observedBehavior: fieldValue(evidence, 'observedBehavior'),
      expectedBehavior: fieldValue(evidence, 'expectedBehavior'),
      claimSummary: blankToNull(operator.claimSummary),
    },
    reproductionAndDiagnostics: {
      reproductionStatus,
      reproductionStatusInput,
      reproductionStepsOrImpossibilityNote: blankToNull(operator.reproductionStepsOrImpossibilityNote),
      diagnosticsSummary: summarizeDiagnostics(evidence),
      evidenceSufficiencyNote: blankToNull(operator.evidenceSufficiencyNote),
      nextRequiredSurface: blankToNull(operator.nextRequiredSurface),
    },
    attachmentsAndRedaction: {
      redactionPosture: describeRedaction(evidence),
      redactionRequired: evidence.redactionRequired,
      attachmentCount: evidence.attachments.length,
    },
    partitions,
    authorityDisclaimers: [...AUTHORITY_DISCLAIMERS],
    classRecognized,
    outcomeStateRecognized,
    reproductionStatusRecognized,
  };

  const complete = completenessProblems.length === 0 && evidenceProblems.length === 0;
  const textDraft = renderTextDraft(draft, { complete, completenessProblems, evidenceProblems });

  return { draft, textDraft, complete, completenessProblems, evidenceProblems };
}

interface OperatorProblemContext {
  operator: OperatorInput;
  primaryClassInput: string | null;
  classRecognized: boolean;
  outcomeStateInput: string | null;
  outcomeStateRecognized: boolean;
  reproductionStatusInput: string | null;
  reproductionStatusRecognized: boolean;
  adjacentAuthorityReferences: string[];
}

function collectOperatorProblems(context: OperatorProblemContext): string[] {
  const { operator } = context;
  const problems: string[] = [];

  if (!blankToNull(operator.receiptId)) {
    problems.push('Receipt id is a required operator-added field but is missing.');
  }
  if (!blankToNull(operator.createdAt)) {
    problems.push('Created-at timestamp is a required operator-added field but is missing.');
  }
  if (!blankToNull(operator.lastUpdatedAt)) {
    problems.push('Last-updated-at timestamp is a required operator-added field but is missing.');
  }

  if (!context.primaryClassInput) {
    problems.push('Primary SD-15 class is a required operator-added field but is missing.');
  } else if (!context.classRecognized) {
    problems.push(
      `Primary SD-15 class "${context.primaryClassInput}" is not one of the recognized SD-15 triage classes.`
    );
  }

  if (!context.outcomeStateInput) {
    problems.push('Outcome state is a required operator-added field but is missing.');
  } else if (!context.outcomeStateRecognized) {
    problems.push(
      `Outcome state "${context.outcomeStateInput}" is not one of the recognized SD-15 outcome states.`
    );
  }

  if (!context.reproductionStatusInput) {
    problems.push('Reproduction status is a required operator-added field but is missing.');
  } else if (!context.reproductionStatusRecognized) {
    problems.push(
      `Reproduction status "${context.reproductionStatusInput}" is not one of the recognized SD-15 reproduction statuses.`
    );
  }

  if (!blankToNull(operator.reproductionStepsOrImpossibilityNote)) {
    problems.push('Reproduction steps or impossibility note is a required operator-added field but is missing.');
  }

  if (context.adjacentAuthorityReferences.length === 0) {
    problems.push('At least one adjacent-authority reference is a required operator-added field but is missing.');
  }

  if (!blankToNull(operator.evidenceSufficiencyNote)) {
    problems.push('Evidence sufficiency note is a required operator-added field but is missing.');
  }

  return problems;
}

interface PartitionOperatorResolved {
  primaryClassInput: string | null;
  primaryClass: TriageClassDescriptor | null;
  outcomeStateInput: string | null;
  reproductionStatusInput: string | null;
  adjacentAuthorityReferences: string[];
}

function buildPartitions(
  evidence: FeedbackEvidencePayload,
  operator: OperatorInput,
  resolved: PartitionOperatorResolved
): EvidencePartitions {
  const testerSupplied: PartitionEntry[] = [];
  const autoCaptured: PartitionEntry[] = [];

  for (const field of evidence.fields) {
    const entry = partitionEntryFromField(field);
    if (field.captureMode === 'tester-entered') {
      testerSupplied.push(entry);
    } else {
      autoCaptured.push(entry);
    }
  }

  // Redaction posture is a tester-declared evidence fact; keep it visible.
  testerSupplied.push({
    key: 'redactionPosture',
    label: 'Redaction posture',
    value: describeRedaction(evidence),
    state: 'tester-entered',
  });

  const operatorAdded: PartitionEntry[] = [
    operatorEntry('receiptId', 'Receipt id', blankToNull(operator.receiptId)),
    operatorEntry('intakeHandle', 'Intake handle', blankToNull(operator.intakeHandle)),
    operatorEntry('createdAt', 'Created at', blankToNull(operator.createdAt)),
    operatorEntry('lastUpdatedAt', 'Last updated at', blankToNull(operator.lastUpdatedAt)),
    operatorEntry('supersedesReceiptId', 'Supersedes receipt id', blankToNull(operator.supersedesReceiptId)),
    operatorEntry('evidenceOwner', 'Evidence owner', blankToNull(operator.evidenceOwner)),
    operatorEntry('primaryClass', 'Primary SD-15 class', resolved.primaryClassInput),
    operatorEntry('outcomeState', 'Outcome state', resolved.outcomeStateInput),
    operatorEntry(
      'adjacentAuthorityReferences',
      'Adjacent-authority references',
      resolved.adjacentAuthorityReferences.length > 0 ? resolved.adjacentAuthorityReferences.join('; ') : null
    ),
    operatorEntry('supportStateContext', 'Support-state context', blankToNull(operator.supportStateContext)),
    operatorEntry(
      'sd14PersistenceMigrationContext',
      'SD-14 persistence/migration context',
      blankToNull(operator.sd14PersistenceMigrationContext)
    ),
    operatorEntry('operatorProvenanceHandle', 'Operator provenance handle', blankToNull(operator.operatorProvenanceHandle)),
    operatorEntry('commitOrBuildIdentity', 'Commit or build identity', blankToNull(operator.commitOrBuildIdentity)),
    operatorEntry(
      'publicationOrAcquisitionHandle',
      'Publication or acquisition handle',
      blankToNull(operator.publicationOrAcquisitionHandle)
    ),
    operatorEntry('rollbackWithdrawalContext', 'Rollback / withdrawal context', blankToNull(operator.rollbackWithdrawalContext)),
    operatorEntry('platformArchitecture', 'Platform architecture', blankToNull(operator.platformArchitecture)),
    operatorEntry('packageInstallContext', 'Package / install context', blankToNull(operator.packageInstallContext)),
    operatorEntry('environmentKind', 'Environment kind', blankToNull(operator.environmentKind)),
    operatorEntry('environmentIdentityHandle', 'Environment identity handle', blankToNull(operator.environmentIdentityHandle)),
    operatorEntry('claimSummary', 'Claim summary', blankToNull(operator.claimSummary)),
    operatorEntry('reproductionStatus', 'Reproduction status', resolved.reproductionStatusInput),
    operatorEntry(
      'reproductionStepsOrImpossibilityNote',
      'Reproduction steps or impossibility note',
      blankToNull(operator.reproductionStepsOrImpossibilityNote)
    ),
    operatorEntry('evidenceSufficiencyNote', 'Evidence sufficiency note', blankToNull(operator.evidenceSufficiencyNote)),
    operatorEntry('nextRequiredSurface', 'Next required surface', blankToNull(operator.nextRequiredSurface)),
  ];

  return { testerSupplied, autoCaptured, operatorAdded };
}

function partitionEntryFromField(field: CapturedEvidenceField): PartitionEntry {
  return {
    key: field.key,
    label: field.label,
    value: field.value,
    state: field.state,
  };
}

function operatorEntry(key: string, label: string, value: string | null): PartitionEntry {
  return {
    key,
    label,
    value,
    state: value === null ? 'operator-missing' : 'operator-added',
  };
}

function fieldValue(evidence: FeedbackEvidencePayload, key: string): string | null {
  const field = evidence.fields.find((candidate) => candidate.key === key);
  if (!field) {
    return null;
  }
  if (field.state === 'redacted') {
    return null;
  }
  return field.value;
}

function summarizeDiagnostics(evidence: FeedbackEvidencePayload): string {
  const diagnosticCount = evidence.auto.diagnostics.length;
  const blockedCount = evidence.auto.blockedClaims.length;
  if (diagnosticCount === 0 && blockedCount === 0) {
    return 'No diagnostics or blocked claims captured in the bounded snapshot.';
  }
  return `${diagnosticCount} diagnostic(s) · ${blockedCount} blocked claim(s)`;
}

function describeRedaction(evidence: FeedbackEvidencePayload): string {
  if (evidence.redaction.declared && evidence.redaction.statement) {
    return `declared: ${evidence.redaction.statement}`;
  }
  if (evidence.redactionRequired) {
    return 'required but not yet declared for included attachments';
  }
  return 'no redaction declaration required (no attachments included)';
}

function renderTextDraft(
  draft: OperatorTriageDraft,
  status: { complete: boolean; completenessProblems: string[]; evidenceProblems: string[] }
): string {
  const lines: string[] = [];

  lines.push('SD-15 OPERATOR TRIAGE / REGRESSION-RECEIPT DRAFT');
  lines.push('(operator-entered judgment; not for submission; no reconciliation; no closure claim)');
  lines.push('');

  lines.push('== Non-claim markers ==');
  for (const disclaimer of draft.authorityDisclaimers) {
    lines.push(`- ${disclaimer}`);
  }
  lines.push('');

  lines.push('== Receipt identity and lineage ==');
  lines.push(`Receipt id: ${render(draft.receiptIdentity.receiptId)}`);
  lines.push(`Intake handle: ${render(draft.receiptIdentity.intakeHandle)}`);
  lines.push(`Created at: ${render(draft.receiptIdentity.createdAt)}`);
  lines.push(`Last updated at: ${render(draft.receiptIdentity.lastUpdatedAt)}`);
  lines.push(`Supersedes receipt id: ${render(draft.receiptIdentity.supersedesReceiptId)}`);
  lines.push(`Evidence owner: ${render(draft.receiptIdentity.evidenceOwner)}`);
  lines.push('');

  lines.push('== Build and provenance context (auto-captured + operator-added) ==');
  lines.push(`Build label / version: ${render(draft.buildProvenanceContext.buildLabelOrVersion)}`);
  lines.push(`Tester channel / support label: ${render(draft.buildProvenanceContext.testerChannelSupportLabel)}`);
  lines.push(`Operator provenance handle: ${render(draft.buildProvenanceContext.operatorProvenanceHandle)}`);
  lines.push(`Commit or build identity: ${render(draft.buildProvenanceContext.commitOrBuildIdentity)}`);
  lines.push(`Publication or acquisition handle: ${render(draft.buildProvenanceContext.publicationOrAcquisitionHandle)}`);
  lines.push(`Rollback / withdrawal context: ${render(draft.buildProvenanceContext.rollbackWithdrawalContext)}`);
  lines.push('');

  lines.push('== Platform and install context ==');
  lines.push(`Platform / OS: ${render(draft.platformInstallContext.platformOs)}`);
  lines.push(`Platform architecture: ${render(draft.platformInstallContext.platformArchitecture)}`);
  lines.push(`Package / install context: ${render(draft.platformInstallContext.packageInstallContext)}`);
  lines.push(`Environment kind: ${render(draft.platformInstallContext.environmentKind)}`);
  lines.push(`Environment identity handle: ${render(draft.platformInstallContext.environmentIdentityHandle)}`);
  lines.push('');

  lines.push('== Workflow and adjacent-authority context ==');
  lines.push(`Bounded workflow under test: ${render(draft.workflowAndAuthorityContext.boundedWorkflowUnderTest)}`);
  lines.push(`Current data-source identity: ${render(draft.workflowAndAuthorityContext.currentDataSourceIdentity)}`);
  lines.push(`Primary SD-15 class (operator-entered): ${renderClass(draft)}`);
  lines.push(`Outcome state (operator-entered): ${renderOutcome(draft)}`);
  lines.push(
    `Adjacent-authority references: ${
      draft.workflowAndAuthorityContext.adjacentAuthorityReferences.length > 0
        ? draft.workflowAndAuthorityContext.adjacentAuthorityReferences.join('; ')
        : ABSENT_MARKER
    }`
  );
  lines.push(`Support-state context: ${render(draft.workflowAndAuthorityContext.supportStateContext)}`);
  lines.push(`SD-14 persistence/migration context: ${render(draft.workflowAndAuthorityContext.sd14PersistenceMigrationContext)}`);
  lines.push('');

  lines.push('== Claim statement ==');
  lines.push(`Observed behavior (tester-supplied): ${render(draft.claimStatement.observedBehavior)}`);
  lines.push(`Expected behavior (tester-supplied): ${render(draft.claimStatement.expectedBehavior)}`);
  lines.push(`Claim summary (operator-added): ${render(draft.claimStatement.claimSummary)}`);
  lines.push('');

  lines.push('== Reproduction and diagnostics ==');
  lines.push(`Reproduction status: ${renderReproduction(draft)}`);
  lines.push(
    `Reproduction steps or impossibility note: ${render(draft.reproductionAndDiagnostics.reproductionStepsOrImpossibilityNote)}`
  );
  lines.push(`Diagnostics / status evidence: ${render(draft.reproductionAndDiagnostics.diagnosticsSummary)}`);
  lines.push(`Evidence sufficiency note: ${render(draft.reproductionAndDiagnostics.evidenceSufficiencyNote)}`);
  lines.push(`Next required surface: ${render(draft.reproductionAndDiagnostics.nextRequiredSurface)}`);
  lines.push('');

  lines.push('== Attachments and redaction posture ==');
  lines.push(`Attachment count: ${draft.attachmentsAndRedaction.attachmentCount}`);
  lines.push(`Redaction posture: ${render(draft.attachmentsAndRedaction.redactionPosture)}`);
  lines.push('');

  lines.push('== Evidence partitions ==');
  lines.push('-- Tester-supplied --');
  appendPartition(lines, draft.partitions.testerSupplied);
  lines.push('-- Auto-captured --');
  appendPartition(lines, draft.partitions.autoCaptured);
  lines.push('-- Operator-added --');
  appendPartition(lines, draft.partitions.operatorAdded);
  lines.push('');

  lines.push('== Draft completeness (not an authoritative triage verdict) ==');
  lines.push(`Overall draft complete: ${status.complete ? 'yes' : 'no'}`);
  if (status.completenessProblems.length > 0) {
    lines.push('Operator-added completeness problems:');
    for (const problem of status.completenessProblems) {
      lines.push(`- ${problem}`);
    }
  } else {
    lines.push('Operator-added completeness problems: none');
  }
  if (status.evidenceProblems.length > 0) {
    lines.push('Underlying SD-11 evidence problems:');
    for (const problem of status.evidenceProblems) {
      lines.push(`- ${problem}`);
    }
  } else {
    lines.push('Underlying SD-11 evidence problems: none');
  }

  return lines.join('\n');
}

function appendPartition(lines: string[], entries: PartitionEntry[]): void {
  for (const entry of entries) {
    const value = entry.state === 'redacted' ? '[redacted by tester before inclusion]' : render(entry.value);
    lines.push(`- ${entry.label} [${entry.state}]: ${value}`);
  }
}

function renderClass(draft: OperatorTriageDraft): string {
  const context = draft.workflowAndAuthorityContext;
  if (draft.classRecognized && context.primaryClassId && context.primaryClassLabel) {
    return `${context.primaryClassLabel} (${context.primaryClassId})`;
  }
  if (context.primaryClassInput) {
    return `${context.primaryClassInput} [unrecognized SD-15 class]`;
  }
  return ABSENT_MARKER;
}

function renderOutcome(draft: OperatorTriageDraft): string {
  const context = draft.workflowAndAuthorityContext;
  if (draft.outcomeStateRecognized && context.outcomeState) {
    return context.outcomeState;
  }
  if (context.outcomeStateInput) {
    return `${context.outcomeStateInput} [unrecognized SD-15 outcome state]`;
  }
  return ABSENT_MARKER;
}

function renderReproduction(draft: OperatorTriageDraft): string {
  const context = draft.reproductionAndDiagnostics;
  if (draft.reproductionStatusRecognized && context.reproductionStatus) {
    return context.reproductionStatus;
  }
  if (context.reproductionStatusInput) {
    return `${context.reproductionStatusInput} [unrecognized SD-15 reproduction status]`;
  }
  return ABSENT_MARKER;
}

function render(value: string | null): string {
  return value === null ? ABSENT_MARKER : value;
}

function blankToNull(value: string | null | undefined): string | null {
  if (value === null || value === undefined) {
    return null;
  }
  const trimmed = value.trim();
  return trimmed.length === 0 ? null : trimmed;
}

function isOutcomeState(value: string): value is OutcomeState {
  return (OUTCOME_STATES as readonly string[]).includes(value);
}

function isReproductionStatus(value: string): value is ReproductionStatus {
  return (REPRODUCTION_STATUSES as readonly string[]).includes(value);
}
