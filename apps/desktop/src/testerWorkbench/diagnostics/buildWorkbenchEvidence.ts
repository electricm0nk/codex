import type {
  Ge08Diagnostic,
  Ge08ExplanationRef,
  Ge08ProvenanceRef,
} from '../../boundary/loadGe08AuthoringWorkbench';

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

export function buildGe08Diagnostics(
  diagnostics: Ge08Diagnostic[]
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

export function buildGe08ExplanationRefs(
  references: Ge08ExplanationRef[]
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

export function buildGe08ProvenanceRefs(
  references: Ge08ProvenanceRef[]
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
