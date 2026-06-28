import { invoke } from '@tauri-apps/api/core';

export type PackageState = 'draft' | 'valid' | 'invalid' | 'deferred';
export type PreviewStatus = 'success' | 'blocked' | 'unsupported';
export type BaselineArmorClassKind = 'Computed' | 'Blocked';

export interface Ge08AuthoringWorkbenchRequest {
  packageRoot: string;
  activeRecordRef?: string | null;
}

export interface Ge08PackageManifest {
  packageId: string;
  packageTitle: string;
  packageVersion: string;
  dependsOn: string[];
  supportedObjectKinds: string[];
}

export interface Ge08AuthoredRecord {
  stableId: string;
  owningFeatId?: string | null;
  displayName: string;
  objectKind: string;
  targetFamily?: string | null;
  modifierType?: string | null;
  modifierValue?: number | null;
  predicate?: string | null;
}

export interface Ge08AuthoredRecords {
  feat: Ge08AuthoredRecord | null;
  effect: Ge08AuthoredRecord | null;
  prerequisite: Ge08AuthoredRecord | null;
}

export interface Ge08SelectedSlotResolution {
  slot: string;
  removed: string;
  added: string;
  resolvedFeatId: string;
}

export interface Ge08BaselineArmorClass {
  kind: BaselineArmorClassKind;
  value?: number;
  reason?: string;
}

export interface Ge08Diagnostic {
  class: string;
  severity: 'Error' | 'Warning';
  message: string;
  subjectRef: string;
  claimBlocking: boolean;
}

export interface Ge08ProvenanceRef {
  stableId: string;
  sourcePackageId: string;
  authoredPath: string;
}

export interface Ge08ExplanationRef {
  nodeKind: string;
  refId: string;
  detail: string;
}

export interface Ge08OracleDimensionStatus {
  dimension: string;
  status: string;
}

export interface Ge08LifecycleGateState {
  saveAllowed: boolean;
  previewAllowed: boolean;
  exportAllowed: boolean;
  diffMode: string;
}

export interface Ge08PreviewEnvelope {
  caseId: string;
  previewStatus: PreviewStatus;
  selectedSlotResolution: Ge08SelectedSlotResolution;
  baselineArmorClass: Ge08BaselineArmorClass;
  diagnostics: Ge08Diagnostic[];
  provenanceRefs: Ge08ProvenanceRef[];
  explanationRefs: Ge08ExplanationRef[];
  oracleDimensionStatus: Ge08OracleDimensionStatus[];
  blockedClaims: string[];
}

export interface Ge08AuthoringWorkbenchSnapshot {
  packageRoot: string;
  packageState: PackageState;
  packageManifest: Ge08PackageManifest;
  activeRecordRef: string | null;
  authoredRecords: Ge08AuthoredRecords;
  preview: Ge08PreviewEnvelope;
  lifecycleGateState: Ge08LifecycleGateState;
  dataSource: 'ge08-headless-preview-bridge' | 'tauri-unavailable';
  note: string;
}

function hasTauriRuntime(): boolean {
  return typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window || '__TAURI__' in window);
}

function formatError(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
}

export async function loadGe08AuthoringWorkbench(
  request: Ge08AuthoringWorkbenchRequest
): Promise<Ge08AuthoringWorkbenchSnapshot> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for GE08 authoring workbench');
  }

  try {
    return await invoke<Ge08AuthoringWorkbenchSnapshot>('load_ge08_authoring_workbench_snapshot', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to load GE08 authoring workbench: ${formatError(cause)}`);
  }
}
