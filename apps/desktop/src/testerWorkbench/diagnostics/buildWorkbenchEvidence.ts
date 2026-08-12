import type {
  Diagnostic,
  ExplanationRef,
  ProvenanceRef,
} from '../../boundary/loadAuthoringWorkbench';

export interface WorkbenchDiagnostic {
  classLabel: string;
  severity: 'info' | 'warning' | 'error';
  severityLabel: string;
  message: string;
  subjectRef: string | null;
  claimBlocking: boolean;
}

export interface WorkbenchReference {
  label: string;
  detail: string;
  machineRef: string;
}

export function buildDiagnostics(
  diagnostics: Diagnostic[]
): WorkbenchDiagnostic[] {
  return diagnostics.map((diagnostic) => ({
    classLabel: diagnostic.class,
    severity: normaliseSeverity(diagnostic.severity),
    severityLabel: diagnostic.severity,
    message: diagnostic.message,
    subjectRef: diagnostic.subjectRef,
    claimBlocking: diagnostic.claimBlocking,
  }));
}

export function buildFallbackDiagnostics(
  messages: string[]
): WorkbenchDiagnostic[] {
  return messages.map((message) => ({
    classLabel: 'Fallback',
    severity: 'warning',
    severityLabel: 'Warning',
    message,
    subjectRef: null,
    claimBlocking: false,
  }));
}

export function buildExplanationRefs(
  references: ExplanationRef[]
): WorkbenchReference[] {
  return references.map((reference) => ({
    label: `${reference.nodeKind}:${reference.refId}`,
    detail: reference.detail,
    machineRef: `${reference.nodeKind}:${reference.refId}`,
  }));
}

export function buildFallbackExplanationRefs(
  references: string[]
): WorkbenchReference[] {
  return references.map((reference) => ({
    label: reference,
    detail: 'Fallback explanation reference preserved for later evidence capture.',
    machineRef: reference,
  }));
}

export function buildProvenanceRefs(
  references: ProvenanceRef[]
): WorkbenchReference[] {
  return references.map((reference) => ({
    label: reference.stableId,
    detail: `${reference.sourcePackageId} · ${reference.authoredPath}`,
    machineRef: `${reference.sourcePackageId}:${reference.authoredPath}`,
  }));
}

function normaliseSeverity(severity: string): 'info' | 'warning' | 'error' {
  if (severity === 'Warning') {
    return 'warning';
  }

  if (severity === 'Error') {
    return 'error';
  }

  return 'info';
}
