import type {
  Sd12ReleaseTruthRequest,
  Sd12ReleaseTruthSnapshot,
} from '../boundary/loadSd12ReleaseTruth';
import type { Ge08AuthoringWorkbenchRequest, Ge08AuthoringWorkbenchSnapshot } from '../boundary/loadGe08AuthoringWorkbench';
import type { PilotShellSnapshot } from '../boundary/loadPilotShellSnapshot';
import {
  buildFallbackDiagnostics,
  buildFallbackExplanationRefs,
  buildGe08Diagnostics,
  buildGe08ExplanationRefs,
  buildGe08ProvenanceRefs,
  type Sd11WorkbenchDiagnostic,
  type Sd11WorkbenchReference,
} from './diagnostics/buildSd11WorkbenchEvidence';
import {
  createSd11WorkbenchStatus,
  formatSd11WorkbenchBuildLabel,
  type Sd11WorkbenchStatus,
} from './status/createSd11WorkbenchStatus';

export interface Sd11WorkbenchRuntimeContext {
  buildVersion: string;
  platformLabel: string;
}

export interface Sd11WorkbenchDependencies {
  loadGe08AuthoringWorkbench: (
    request: Ge08AuthoringWorkbenchRequest
  ) => Promise<Ge08AuthoringWorkbenchSnapshot>;
  loadPilotShellSnapshot: () => Promise<PilotShellSnapshot>;
  loadSd12ReleaseTruth: (
    request: Sd12ReleaseTruthRequest
  ) => Promise<Sd12ReleaseTruthSnapshot>;
}

export interface Sd11WorkbenchSummaryRow {
  label: string;
  value: string;
}

export interface Sd11TesterWorkbenchSurface {
  surfaceLabel: string;
  headline: string;
  lead: string;
  buildLabel: string;
  channelLabel: 'alpha';
  platformLabel: string;
  supportTierLabel: string;
  workflowName: string;
  workflowState: string;
  dataTruthLabel: string;
  fallbackNotice: string | null;
  boundedScopeNotice: string;
  feedbackStatusNotice: string;
  updateStatusLabel: string;
  summaryRows: Sd11WorkbenchSummaryRow[];
  diagnostics: Sd11WorkbenchDiagnostic[];
  blockedClaims: string[];
  explanationRefs: Sd11WorkbenchReference[];
  provenanceRefs: Sd11WorkbenchReference[];
  notes: string[];
  status: Sd11WorkbenchStatus;
}

const DEFAULT_REQUEST: Ge08AuthoringWorkbenchRequest = {
  packageRoot: 'tests/fixtures/ge08/guard-stance-package',
};

export async function loadSd11TesterWorkbenchSurface(
  context: Sd11WorkbenchRuntimeContext,
  dependencies: Sd11WorkbenchDependencies
): Promise<Sd11TesterWorkbenchSurface> {
  const releaseTruthPromise = dependencies.loadSd12ReleaseTruth(buildReleaseTruthRequest(context));

  try {
    const [snapshot, releaseTruth] = await Promise.all([
      dependencies.loadGe08AuthoringWorkbench(DEFAULT_REQUEST),
      releaseTruthPromise,
    ]);
    return mapGe08Snapshot(context, snapshot, releaseTruth);
  } catch (cause: unknown) {
    const [fallbackSnapshot, releaseTruth] = await Promise.all([
      dependencies.loadPilotShellSnapshot(),
      releaseTruthPromise,
    ]);
    return mapPilotFallback(context, fallbackSnapshot, formatError(cause), releaseTruth);
  }
}

function buildReleaseTruthRequest(context: Sd11WorkbenchRuntimeContext): Sd12ReleaseTruthRequest {
  return {
    buildVersion: context.buildVersion,
    buildLabel: formatSd11WorkbenchBuildLabel(context.buildVersion),
    platformLabel: context.platformLabel,
    testerChannelLabel: 'alpha',
  };
}

function mapGe08Snapshot(
  context: Sd11WorkbenchRuntimeContext,
  snapshot: Ge08AuthoringWorkbenchSnapshot,
  releaseTruth: Sd12ReleaseTruthSnapshot
): Sd11TesterWorkbenchSurface {
  const status = createSd11WorkbenchStatus(context, releaseTruth);

  return {
    surfaceLabel: 'SD-11 tester workbench',
    headline: 'Bounded tester workbench over a real desktop command surface',
    lead:
      'This frame is the first SD-11 tester workbench slice: it presents one real bounded workflow over the Tauri command boundary, keeps diagnostics visible, and refuses to pretend broader product readiness.',
    buildLabel: status.build.label,
    channelLabel: status.channel.testerFacingLabel,
    platformLabel: status.support.platformLabel,
    supportTierLabel: status.support.tierMatrixLabel,
    workflowName: 'GE08 Guard Stance authoring workbench',
    workflowState: `${snapshot.packageState} / ${snapshot.preview.previewStatus}`,
    dataTruthLabel: 'Real Tauri command snapshot',
    fallbackNotice: null,
    boundedScopeNotice:
      'Bounded scope only: this slice proves the GE08 authoring workflow, not a full character-builder surface, GitHub submission transport, or updater mechanics.',
    feedbackStatusNotice:
      'Feedback intake is intentionally deferred in this slice. The frame keeps diagnostics, workflow identity, and synchronized release-truth evidence visible so later GitHub flows can consume honest evidence.',
    updateStatusLabel: status.update.label,
    summaryRows: [
      {
        label: 'Package',
        value: snapshot.packageManifest.packageId,
      },
      {
        label: 'Preview',
        value: snapshot.preview.previewStatus,
      },
      {
        label: 'Data source',
        value: snapshot.dataSource,
      },
      {
        label: 'Baseline AC',
        value: formatBaselineArmorClass(snapshot.preview.baselineArmorClass),
      },
    ],
    diagnostics: buildGe08Diagnostics(snapshot.preview.diagnostics),
    blockedClaims: snapshot.preview.blockedClaims,
    explanationRefs: buildGe08ExplanationRefs(snapshot.preview.explanationRefs),
    provenanceRefs: buildGe08ProvenanceRefs(snapshot.preview.provenanceRefs),
    notes: [snapshot.note],
    status,
  };
}

function mapPilotFallback(
  context: Sd11WorkbenchRuntimeContext,
  snapshot: PilotShellSnapshot,
  failure: string,
  releaseTruth: Sd12ReleaseTruthSnapshot
): Sd11TesterWorkbenchSurface {
  const status = createSd11WorkbenchStatus(context, releaseTruth);

  return {
    surfaceLabel: 'SD-11 tester workbench',
    headline: 'Bounded tester workbench fallback over the pilot seam',
    lead:
      'The preferred GE08 workbench could not load, so the app drops to an explicitly labeled fallback. The fallback exists to preserve truthful runtime seams and must not masquerade as full product state.',
    buildLabel: status.build.label,
    channelLabel: status.channel.testerFacingLabel,
    platformLabel: status.support.platformLabel,
    supportTierLabel: status.support.tierMatrixLabel,
    workflowName: 'GE07 pilot snapshot seam',
    workflowState: snapshot.receiptStatus,
    dataTruthLabel:
      snapshot.dataSource === 'tauri-command'
        ? 'Explicit fallback over a Tauri pilot seam'
        : 'Explicit fallback placeholder',
    fallbackNotice:
      `GE08 authoring workbench unavailable: ${failure}. This fallback exists because the real bounded snapshot could not load and the UI must not counterfeit product truth.`,
    boundedScopeNotice:
      'Bounded scope only: this fallback preserves the pilot runtime seam and visible failure context. It does not claim the broader tester workbench, GitHub feedback transport, or updater behavior are implemented.',
    feedbackStatusNotice:
      'Feedback intake remains deferred. Use the visible fallback reason, diagnostics, workflow identity, and synchronized release-truth evidence as the current evidence surface until the richer SD-11 flows land.',
    updateStatusLabel: status.update.label,
    summaryRows: [
      {
        label: 'Case',
        value: snapshot.caseId,
      },
      {
        label: 'Receipt status',
        value: snapshot.receiptStatus,
      },
      {
        label: 'Data source',
        value: snapshot.dataSource,
      },
    ],
    diagnostics: buildFallbackDiagnostics(snapshot.diagnostics),
    blockedClaims: [],
    explanationRefs: buildFallbackExplanationRefs(snapshot.explanationRefs),
    provenanceRefs: [],
    notes: [snapshot.note],
    status,
  };
}

function formatBaselineArmorClass(
  baselineArmorClass: Ge08AuthoringWorkbenchSnapshot['preview']['baselineArmorClass']
): string {
  if (baselineArmorClass.kind === 'Computed') {
    return `${baselineArmorClass.value}`;
  }

  return `Blocked: ${baselineArmorClass.reason}`;
}

function formatError(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
}
