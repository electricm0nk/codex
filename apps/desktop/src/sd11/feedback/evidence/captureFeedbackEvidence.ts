/**
 * Shared SD-11 feedback evidence capture and assembly.
 *
 * `captureAutoEvidence` resolves the auto-captured backbone from the live tester
 * workbench surface. `assembleFeedbackEvidence` merges that backbone with
 * tester-entered narrative fields and attachment/redaction decisions into a
 * single validated payload. Both the bug and enhancement flows call this so the
 * shared schema cannot drift between them.
 */

import type { Sd11TesterWorkbenchSurface } from '../../loadSd11TesterWorkbenchSurface';
import type {
  Sd11WorkbenchDiagnostic,
  Sd11WorkbenchReference,
} from '../../diagnostics/buildSd11WorkbenchEvidence';
import {
  evidenceFieldByKey,
  fieldsForFlow,
  type EvidenceCaptureMode,
  type EvidenceFieldKey,
  type EvidenceFieldState,
  type EvidenceRequirement,
  type FeedbackFlowKind,
} from './evidenceFields';
import {
  evaluateAttachment,
  requiresRedactionDeclaration,
  validateRedaction,
  type AttachmentDecision,
  type EvidenceAttachment,
  type RedactionDeclaration,
} from './redaction';
import { sanitizeReportableOutput } from './sanitizeReportableOutput';

export interface AutoCapturedEvidence {
  buildLabel: string;
  channelSupportLabel: string;
  platformLabel: string;
  currentWorkflow: string;
  dataSourceIdentity: string;
  releaseUnitId: string | null;
  sourceRevision: string | null;
  manifestPath: string | null;
  updateEligibilityState: string | null;
  trustGateStatus: string | null;
  replacementReleaseId: string | null;
  officialSurface: string | null;
  localBuildAuthority: string | null;
  diagnostics: Sd11WorkbenchDiagnostic[];
  blockedClaims: string[];
  explanationRefs: Sd11WorkbenchReference[];
  provenanceRefs: Sd11WorkbenchReference[];
}

export interface TesterEnteredInput {
  observedBehavior?: string;
  expectedBehavior?: string;
  reproductionSteps?: string;
  testerGoal?: string;
  currentFriction?: string;
  requestedCapability?: string;
  affectedSurface?: string;
}

export interface AssembleFeedbackEvidenceInput {
  flow: FeedbackFlowKind;
  surface: Sd11TesterWorkbenchSurface;
  testerInput: TesterEnteredInput;
  attachments?: EvidenceAttachment[];
  redaction?: RedactionDeclaration;
  /** Tester fields whose content was scrubbed/omitted before inclusion. */
  redactedFields?: EvidenceFieldKey[];
}

export interface CapturedEvidenceField {
  key: EvidenceFieldKey;
  label: string;
  captureMode: EvidenceCaptureMode;
  requirement: EvidenceRequirement;
  state: EvidenceFieldState;
  value: string | null;
  present: boolean;
}

export interface FeedbackEvidencePayload {
  flow: FeedbackFlowKind;
  auto: AutoCapturedEvidence;
  fields: CapturedEvidenceField[];
  attachments: AttachmentDecision[];
  redaction: RedactionDeclaration;
  redactionRequired: boolean;
  complete: boolean;
  problems: string[];
}

const NO_REDACTION: RedactionDeclaration = { declared: false, statement: null };

/** Resolve the auto-captured evidence backbone from the live workbench surface. */
export function captureAutoEvidence(surface: Sd11TesterWorkbenchSurface): AutoCapturedEvidence {
  const dataSourceIdentity = surface.fallbackNotice
    ? sanitizeReportableOutput(`${surface.dataTruthLabel} (fallback: ${surface.fallbackNotice})`)
    : sanitizeReportableOutput(surface.dataTruthLabel);
  const releaseTruth = surface.status.issueCapture.releaseTruth;

  return {
    buildLabel: sanitizeReportableOutput(surface.buildLabel),
    channelSupportLabel: sanitizeReportableOutput(surface.status.issueCapture.testerFacingChannelSupportLabel),
    platformLabel: sanitizeReportableOutput(surface.platformLabel),
    currentWorkflow: sanitizeReportableOutput(`${surface.workflowName} / ${surface.workflowState}`),
    dataSourceIdentity,
    releaseUnitId: sanitizeNullable(releaseTruth?.releaseUnitId ?? null),
    sourceRevision: sanitizeNullable(releaseTruth?.sourceRevision ?? null),
    manifestPath: sanitizeNullable(releaseTruth?.manifestPath ?? null),
    updateEligibilityState: sanitizeNullable(releaseTruth?.updateEligibilityState ?? null),
    trustGateStatus: sanitizeNullable(releaseTruth?.trustGateStatus ?? null),
    replacementReleaseId: sanitizeNullable(releaseTruth?.replacementReleaseId ?? null),
    officialSurface: sanitizeNullable(releaseTruth?.officialSurface ?? null),
    localBuildAuthority: sanitizeNullable(releaseTruth?.localBuildAuthority ?? null),
    diagnostics: surface.diagnostics.map((diagnostic) => ({
      ...diagnostic,
      classLabel: sanitizeReportableOutput(diagnostic.classLabel),
      severityLabel: sanitizeReportableOutput(diagnostic.severityLabel),
      message: sanitizeReportableOutput(diagnostic.message),
      subjectRef: sanitizeNullable(diagnostic.subjectRef ?? null),
    })),
    blockedClaims: surface.blockedClaims.map(sanitizeReportableOutput),
    explanationRefs: surface.explanationRefs.map(sanitizeReference),
    provenanceRefs: surface.provenanceRefs.map(sanitizeReference),
  };
}

/**
 * Assemble a validated feedback evidence payload for the given flow. Categorizes
 * each applicable field into exactly one of auto-captured / tester-entered /
 * optional / redacted, and reports any completeness or redaction problems.
 */
export function assembleFeedbackEvidence(
  input: AssembleFeedbackEvidenceInput
): FeedbackEvidencePayload {
  const auto = captureAutoEvidence(input.surface);
  const attachments = input.attachments ?? [];
  const redaction = sanitizeRedactionDeclaration(input.redaction ?? NO_REDACTION);
  const redactedFields = new Set(input.redactedFields ?? []);

  const problems: string[] = [];
  const fields: CapturedEvidenceField[] = [];

  for (const descriptor of fieldsForFlow(input.flow)) {
    // Attachment/redaction fields are validated separately below.
    if (descriptor.key === 'attachments' || descriptor.key === 'redactionDeclaration') {
      continue;
    }

    const value = resolveFieldValue(descriptor.key, auto, input.testerInput);
    const isRedacted = redactedFields.has(descriptor.key);
    const present = isRedacted || value !== null;

    const state: EvidenceFieldState = isRedacted
      ? 'redacted'
      : present
        ? descriptor.captureMode
        : 'optional';

    fields.push({
      key: descriptor.key,
      label: descriptor.label,
      captureMode: descriptor.captureMode,
      requirement: descriptor.requirement,
      state,
      value: isRedacted ? null : value,
      present,
    });

    if (descriptor.requirement === 'required' && !present) {
      problems.push(`${descriptor.label} is required for the ${input.flow} flow but is missing.`);
    }
  }

  const attachmentDecisions = attachments.map(evaluateAttachment);
  const redactionRequired = requiresRedactionDeclaration(attachments);
  const redactionValidation = validateRedaction(attachments, redaction);
  problems.push(...redactionValidation.problems);

  return {
    flow: input.flow,
    auto,
    fields,
    attachments: attachmentDecisions,
    redaction,
    redactionRequired,
    complete: problems.length === 0,
    problems,
  };
}

function resolveFieldValue(
  key: EvidenceFieldKey,
  auto: AutoCapturedEvidence,
  testerInput: TesterEnteredInput
): string | null {
  switch (key) {
    case 'buildLabel':
      return auto.buildLabel;
    case 'channelSupportLabel':
      return auto.channelSupportLabel;
    case 'platformLabel':
      return auto.platformLabel;
    case 'currentWorkflow':
      return auto.currentWorkflow;
    case 'dataSourceIdentity':
      return auto.dataSourceIdentity;
    case 'releaseUnitId':
      return auto.releaseUnitId;
    case 'sourceRevision':
      return auto.sourceRevision;
    case 'manifestPath':
      return auto.manifestPath;
    case 'updateEligibilityState':
      return auto.updateEligibilityState;
    case 'trustGateStatus':
      return auto.trustGateStatus;
    case 'replacementReleaseId':
      return auto.replacementReleaseId;
    case 'officialSurface':
      return auto.officialSurface;
    case 'localBuildAuthority':
      return auto.localBuildAuthority;
    case 'diagnostics': {
      const diagnosticCount = auto.diagnostics.length;
      const blockedCount = auto.blockedClaims.length;
      if (diagnosticCount === 0 && blockedCount === 0) {
        return null;
      }
      return `${diagnosticCount} diagnostic(s) · ${blockedCount} blocked claim(s)`;
    }
    case 'explanationRefs':
      return auto.explanationRefs.length ? `${auto.explanationRefs.length} explanation ref(s)` : null;
    case 'provenanceRefs':
      return auto.provenanceRefs.length ? `${auto.provenanceRefs.length} provenance ref(s)` : null;
    case 'observedBehavior':
      return normalise(testerInput.observedBehavior);
    case 'expectedBehavior':
      return normalise(testerInput.expectedBehavior);
    case 'reproductionSteps':
      return normalise(testerInput.reproductionSteps);
    case 'testerGoal':
      return normalise(testerInput.testerGoal);
    case 'currentFriction':
      return normalise(testerInput.currentFriction);
    case 'requestedCapability':
      return normalise(testerInput.requestedCapability);
    case 'affectedSurface':
      return normalise(testerInput.affectedSurface);
    default:
      // attachments / redactionDeclaration are handled outside this resolver.
      evidenceFieldByKey(key);
      return null;
  }
}

function normalise(value: string | undefined): string | null {
  if (value === undefined) {
    return null;
  }
  const trimmed = value.trim();
  return trimmed.length === 0 ? null : sanitizeReportableOutput(trimmed);
}

function sanitizeNullable(value: string | null): string | null {
  return value === null ? null : sanitizeReportableOutput(value);
}

function sanitizeRedactionDeclaration(redaction: RedactionDeclaration): RedactionDeclaration {
  return {
    declared: redaction.declared,
    statement: sanitizeNullable(redaction.statement),
  };
}

function sanitizeReference(reference: Sd11WorkbenchReference): Sd11WorkbenchReference {
  return {
    ...reference,
    label: sanitizeReportableOutput(reference.label),
    detail: sanitizeReportableOutput(reference.detail),
    machineRef: sanitizeReportableOutput(reference.machineRef),
  };
}
