import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

export type PackageState = 'draft' | 'valid' | 'invalid' | 'deferred';
export type PreviewStatus = 'success' | 'blocked' | 'unsupported';
export type BaselineArmorClassKind = 'Computed' | 'Blocked';

export interface AuthoringWorkbenchRequest {
  packageRoot: string;
  activeRecordRef?: string | null;
}

export interface PackageManifest {
  packageId: string;
  packageTitle: string;
  packageVersion: string;
  dependsOn: string[];
  supportedObjectKinds: string[];
}

export interface AuthoredRecord {
  stableId: string;
  owningFeatId?: string | null;
  displayName: string;
  objectKind: string;
  targetFamily?: string | null;
  modifierType?: string | null;
  modifierValue?: number | null;
  predicate?: string | null;
}

export interface AuthoredRecords {
  feat: AuthoredRecord | null;
  effect: AuthoredRecord | null;
  prerequisite: AuthoredRecord | null;
}

export interface SelectedSlotResolution {
  slot: string;
  removed: string;
  added: string;
  resolvedFeatId: string;
}

export interface BaselineArmorClass {
  kind: BaselineArmorClassKind;
  value?: number;
  reason?: string;
}

export interface Diagnostic {
  class: string;
  severity: 'Error' | 'Warning';
  message: string;
  subjectRef: string;
  claimBlocking: boolean;
}

export interface ProvenanceRef {
  stableId: string;
  sourcePackageId: string;
  authoredPath: string;
}

export interface ExplanationRef {
  nodeKind: string;
  refId: string;
  detail: string;
}

export interface OracleDimensionStatus {
  dimension: string;
  status: string;
}

export interface LifecycleGateState {
  saveAllowed: boolean;
  previewAllowed: boolean;
  exportAllowed: boolean;
  diffMode: string;
}

export interface PreviewEnvelope {
  caseId: string;
  previewStatus: PreviewStatus;
  selectedSlotResolution: SelectedSlotResolution;
  baselineArmorClass: BaselineArmorClass;
  diagnostics: Diagnostic[];
  provenanceRefs: ProvenanceRef[];
  explanationRefs: ExplanationRef[];
  oracleDimensionStatus: OracleDimensionStatus[];
  blockedClaims: string[];
}

export interface AuthoringWorkbenchSnapshot {
  packageRoot: string;
  packageState: PackageState;
  packageManifest: PackageManifest;
  activeRecordRef: string | null;
  authoredRecords: AuthoredRecords;
  preview: PreviewEnvelope;
  lifecycleGateState: LifecycleGateState;
  dataSource: 'headless-preview-bridge' | 'tauri-unavailable';
  note: string;
}

export async function loadAuthoringWorkbench(
  request: AuthoringWorkbenchRequest
): Promise<AuthoringWorkbenchSnapshot> {
  if (!hasTauriRuntime()) {
    throw new Error('Tauri runtime not available for GE08 authoring workbench');
  }

  try {
    return await invoke<AuthoringWorkbenchSnapshot>('load_authoring_workbench_snapshot', { request });
  } catch (cause: unknown) {
    throw new Error(`Failed to load GE08 authoring workbench: ${formatError(cause)}`);
  }
}
