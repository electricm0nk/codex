import type {
  ReleaseTruthRequest,
  ReleaseTruthSnapshot,
} from '../boundary/loadReleaseTruth';
import type { AuthoringWorkbenchRequest, AuthoringWorkbenchSnapshot } from '../boundary/loadAuthoringWorkbench';
import type { PilotShellSnapshot } from '../boundary/loadPilotShellSnapshot';
import type { BackendHealthSnapshot } from '../boundary/loadBackendHealth';
import {
  buildFallbackDiagnostics,
  buildFallbackExplanationRefs,
  buildDiagnostics,
  buildExplanationRefs,
  buildProvenanceRefs,
  type WorkbenchDiagnostic,
  type WorkbenchReference,
} from './diagnostics/buildWorkbenchEvidence';
import {
  createWorkbenchStatus,
  formatWorkbenchBuildLabel,
  type SupportTier,
  type WorkbenchStatus,
} from './status/createWorkbenchStatus';
import { formatError } from '../boundary/runtime';

export interface WorkbenchRuntimeContext {
  buildVersion: string;
  platformLabel: string;
}

export interface WorkbenchDependencies {
  loadAuthoringWorkbench: (
    request: AuthoringWorkbenchRequest
  ) => Promise<AuthoringWorkbenchSnapshot>;
  loadPilotShellSnapshot: () => Promise<PilotShellSnapshot>;
  loadReleaseTruth: (
    request: ReleaseTruthRequest
  ) => Promise<ReleaseTruthSnapshot>;
  /**
   * Rust backend version/commit check. Optional so SD-11 callers that predate
   * this slice remain valid; when absent or it fails, the backend health card
   * reports unreachable rather than fabricating a version.
   */
  loadBackendHealth?: () => Promise<BackendHealthSnapshot>;
}

export interface BackendHealthPresentation {
  reachable: boolean;
  version: string | null;
  gitCommit: string | null;
  unavailableNotice: string | null;
}

export interface WorkbenchSummaryRow {
  label: string;
  value: string;
}

export interface TesterWorkbenchSurface {
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
  summaryRows: WorkbenchSummaryRow[];
  diagnostics: WorkbenchDiagnostic[];
  blockedClaims: string[];
  explanationRefs: WorkbenchReference[];
  provenanceRefs: WorkbenchReference[];
  /**
   * Rust backend version/commit check. Optional in the type so SD-11 surface
   * literals that predate this slice stay valid; the live loader always
   * populates it.
   */
  backendHealth?: BackendHealthPresentation;
  notes: string[];
  status: WorkbenchStatus;
}

/** Loads and shapes the backend-health check; never rejects — a failure becomes an unreachable presentation. */
async function loadBackendHealthPresentation(
  dependencies: WorkbenchDependencies
): Promise<BackendHealthPresentation> {
  if (!dependencies.loadBackendHealth) {
    return {
      reachable: false,
      version: null,
      gitCommit: null,
      unavailableNotice: 'Backend health check unavailable: no backend health bridge was provided to the workbench.',
    };
  }

  try {
    const health = await dependencies.loadBackendHealth();
    return {
      reachable: true,
      version: health.version,
      gitCommit: health.gitCommit,
      unavailableNotice: null,
    };
  } catch (cause: unknown) {
    return {
      reachable: false,
      version: null,
      gitCommit: null,
      unavailableNotice: `Backend health check unavailable: ${formatError(cause)}.`,
    };
  }
}

const DEFAULT_REQUEST: AuthoringWorkbenchRequest = {
  packageRoot: 'resources/authoring_workbench/guard-stance-package',
};

export async function loadTesterWorkbenchSurface(
  context: WorkbenchRuntimeContext,
  dependencies: WorkbenchDependencies
): Promise<TesterWorkbenchSurface> {
  const releaseTruthRequest = buildReleaseTruthRequest(context);
  const releaseTruthPromise = dependencies.loadReleaseTruth(releaseTruthRequest).catch((cause: unknown) => {
    const reason = `Release-truth bridge failed: ${formatError(cause)}`;
    const normalised = releaseTruthRequest.platformLabel.trim().toLowerCase();
    const platformTier: SupportTier =
      normalised === 'linux'
        ? 'first-class'
        : normalised === 'macos'
          ? 'second-class'
          : normalised === 'windows'
            ? 'third-class'
            : 'unknown';

    return {
      truth: {
        kind: 'check-failed' as const,
        reason,
        buildLabel: releaseTruthRequest.buildLabel,
        version: releaseTruthRequest.buildVersion,
      },
      updateAction: {
        state: 'check-failed' as const,
        headline: 'Update check failed',
        detail: reason,
        platformLabel: releaseTruthRequest.platformLabel,
        platformTier,
        testerChannelLabel: releaseTruthRequest.testerChannelLabel,
        automaticEligible: false,
        manualReason: null,
        replacementTarget: null,
        recoveryDirection: null,
        checkedBuildLabel: releaseTruthRequest.buildLabel,
        checkedVersion: releaseTruthRequest.buildVersion,
        operatorPromotionPathReference: null,
        evidenceNotes: [],
      },
      issueCapture: {
        releaseUnitId: null,
        sourceRevision: null,
        manifestPath: null,
        updateEligibilityState: 'check-failed',
        trustGateStatus: 'unverified-runtime-check-failed',
        replacementReleaseId: null,
        officialSurface:
          'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadUpdateAction.ts over the F3a fetch pipeline',
        localBuildAuthority: reason,
      },
    };
  });
  const backendHealthPromise = loadBackendHealthPresentation(dependencies);

  try {
    const [snapshot, releaseTruth] = await Promise.all([
      dependencies.loadAuthoringWorkbench(DEFAULT_REQUEST),
      releaseTruthPromise,
    ]);
    return mapSnapshot(context, snapshot, releaseTruth, await backendHealthPromise);
  } catch (cause: unknown) {
    const [fallbackSnapshot, releaseTruth] = await Promise.all([
      dependencies.loadPilotShellSnapshot(),
      releaseTruthPromise,
    ]);
    return mapPilotFallback(
      context,
      fallbackSnapshot,
      formatError(cause),
      releaseTruth,
      await backendHealthPromise
    );
  }
}

function buildReleaseTruthRequest(context: WorkbenchRuntimeContext): ReleaseTruthRequest {
  return {
    buildVersion: context.buildVersion,
    buildLabel: formatWorkbenchBuildLabel(context.buildVersion),
    platformLabel: context.platformLabel,
    testerChannelLabel: 'alpha',
  };
}

function mapSnapshot(
  context: WorkbenchRuntimeContext,
  snapshot: AuthoringWorkbenchSnapshot,
  releaseTruth: ReleaseTruthSnapshot,
  backendHealth: BackendHealthPresentation
): TesterWorkbenchSurface {
  const status = createWorkbenchStatus(context, releaseTruth);

  return {
    surfaceLabel: 'Developer diagnostics',
    headline: 'Connected to the app backend',
    lead:
      'This tab shows live diagnostics from one specific backend check (character-preview authoring) so you can see whether the app is actually talking to its backend and what it found, rather than a guess.',
    buildLabel: status.build.label,
    channelLabel: status.channel.testerFacingLabel,
    platformLabel: status.support.platformLabel,
    supportTierLabel: status.support.tierMatrixLabel,
    workflowName: 'Character-preview authoring check',
    workflowState: `${snapshot.packageState} / ${snapshot.preview.previewStatus}`,
    dataTruthLabel: 'Live backend data',
    fallbackNotice: null,
    boundedScopeNotice:
      'This check only covers character-preview authoring — it is not a full health check of every feature in the app.',
    feedbackStatusNotice:
      'Automatic feedback capture from this tab is not built yet. Use the diagnostics and any error details below when filing a bug report.',
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
    diagnostics: buildDiagnostics(snapshot.preview.diagnostics),
    blockedClaims: snapshot.preview.blockedClaims,
    explanationRefs: buildExplanationRefs(snapshot.preview.explanationRefs),
    provenanceRefs: buildProvenanceRefs(snapshot.preview.provenanceRefs),
    backendHealth,
    notes: [snapshot.note],
    status,
  };
}

const NO_TAURI_RUNTIME_FAILURE = 'Tauri runtime not available for GE08 authoring workbench';

// `loadPilotShellSnapshot` has no real backend behind it today — every field
// it returns for this marker is a hardcoded placeholder (see its own file),
// so surfacing it as if it were diagnostic data would just be noise. Real
// content instead comes from `failure`, the actual error from the primary
// backend check. If a future slice gives PilotShellSnapshot a real data
// source, this marker check is what to remove.
const PLACEHOLDER_DATA_SOURCE = 'scaffold-placeholder';

// The two lines `loadPilotShellSnapshot`'s placeholder always emits — drop
// them so only a genuine appended failure line (if any) survives.
const PLACEHOLDER_DIAGNOSTIC_TEXT = new Set([
  'This scaffold is additive only.',
  'Real GE-06 pilot data is not wired in this slice.',
]);

function describePilotFallbackFailure(failure: string): string {
  if (failure.includes(NO_TAURI_RUNTIME_FAILURE)) {
    return (
      'This check needs the compiled Codex desktop app — it can\'t reach a backend from a plain browser tab. ' +
      'If you opened this at localhost:1420 in a browser, open the actual Codex app window instead.'
    );
  }

  return `Backend check failed: ${failure}. Showing fallback data instead of live data.`;
}

function mapPilotFallback(
  context: WorkbenchRuntimeContext,
  snapshot: PilotShellSnapshot,
  failure: string,
  releaseTruth: ReleaseTruthSnapshot,
  backendHealth: BackendHealthPresentation
): TesterWorkbenchSurface {
  const status = createWorkbenchStatus(context, releaseTruth);
  const isPlaceholderData = snapshot.dataSource === PLACEHOLDER_DATA_SOURCE;

  return {
    surfaceLabel: 'Developer diagnostics',
    headline: 'Backend check failed — showing fallback data',
    lead:
      'The live backend check could not complete, so this tab is showing fallback data instead of live data. The reason for the failure is shown below rather than being hidden.',
    buildLabel: status.build.label,
    channelLabel: status.channel.testerFacingLabel,
    platformLabel: status.support.platformLabel,
    supportTierLabel: status.support.tierMatrixLabel,
    workflowName: 'Fallback data source',
    workflowState: snapshot.receiptStatus,
    dataTruthLabel:
      snapshot.dataSource === 'tauri-command'
        ? 'Fallback data (backend reachable, but the check itself failed)'
        : 'Fallback data (backend unavailable)',
    fallbackNotice: describePilotFallbackFailure(failure),
    boundedScopeNotice:
      'This fallback only preserves the failure context for this one check — it does not mean the rest of the app, feedback submission, or updates are broken.',
    feedbackStatusNotice:
      'Automatic feedback capture from this tab is not built yet. Use the fallback reason and diagnostics below when filing a bug report.',
    updateStatusLabel: status.update.label,
    summaryRows: isPlaceholderData
      ? []
      : [
          { label: 'Case', value: snapshot.caseId },
          { label: 'Receipt status', value: snapshot.receiptStatus },
          { label: 'Data source', value: snapshot.dataSource },
        ],
    diagnostics: buildFallbackDiagnostics(snapshot.diagnostics.filter((line) => !PLACEHOLDER_DIAGNOSTIC_TEXT.has(line))),
    blockedClaims: [],
    explanationRefs: isPlaceholderData ? [] : buildFallbackExplanationRefs(snapshot.explanationRefs),
    provenanceRefs: [],
    backendHealth,
    notes: isPlaceholderData ? [] : [snapshot.note],
    status,
  };
}

function formatBaselineArmorClass(
  baselineArmorClass: AuthoringWorkbenchSnapshot['preview']['baselineArmorClass']
): string {
  if (baselineArmorClass.kind === 'Computed') {
    return `${baselineArmorClass.value}`;
  }

  return `Blocked: ${baselineArmorClass.reason}`;
}

